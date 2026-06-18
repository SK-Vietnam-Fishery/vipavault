# VipaVault — Project Spec
**Phiên bản:** 1.0  
**Ngày:** 05/2026  
**Tech Stack:** Tauri 2.x · Rust · React + TypeScript · SQLCipher

---

## 1. Mục đích

App quản lý tập trung tài sản kỹ thuật số của công ty: shared hosting, email account, domain, SSL certificate, và tương lai là subscription service (M365, Google Workspace).

**Hai loại user trong cùng một app, phân quyền theo role:**

| Role | Mục đích chính | Quyền |
|---|---|---|
| `admin` (IT) | Làm việc — tạo email, reset pass, sync, quản lý credential | Full access |
| `viewer` (Sếp/CEO) | Nhìn — dashboard chi phí, cảnh báo hết hạn, trạng thái tổng thể | Read-only |

Phân quyền qua `app_settings.json` per-machine, không phải user account system.

---

## 2. Kiến trúc tổng thể

```
┌─────────────────────────────────────────────────────┐
│              VIPAVAULT (Tauri App)                   │
│                                                      │
│  ┌──────────────────┐    ┌─────────────────────┐    │
│  │  React Frontend  │    │   Rust Backend       │    │
│  │                  │    │                      │    │
│  │ • Dashboard      │    │ • Vault Engine       │    │
│  │   (2 role view)  │◄──►│   (SQLCipher)        │    │
│  │ • Hosting list   │    │ • Argon2id + AES-GCM │    │
│  │ • Email manager  │    │ • Provider clients   │    │
│  │ • Notify composer│    │ • Auto-lock timer    │    │
│  │ • Confuse engine │    │ • zeroize() on lock  │    │
│  └──────────────────┘    └─────────┬────────────┘    │
│                                    │                  │
│                         ┌──────────▼──────────┐      │
│                         │  .hvault (SQLCipher) │      │
│                         │  AES-256-GCM         │      │
│                         └─────────────────────┘      │
└─────────────────────────────────────────────────────┘
                                    │ HTTPS (manual sync)
                         ┌──────────▼──────────┐
                         │   Provider API       │
                         │ cPanel UAPI / future │
                         └─────────────────────┘
```

### Cấu trúc file

```
~/.vipavault/
  ├── profiles.json          ← Metadata profiles (không mã hóa)
  ├── company_a.hvault       ← SQLCipher DB, mã hóa toàn bộ
  └── backups/
      └── company_a_20260513.hvault

app_settings.json            ← Per-machine, NGOÀI vault
  sync_enabled: false        ← false = máy sếp (viewer)
  machine_role: viewer       ← 'admin' | 'viewer'
```

**Phân quyền máy sếp:** copy file `.hvault` sang máy sếp. `app_settings.json` set `machine_role: viewer`. Tất cả nút write bị disabled, badge "Chế độ xem" hiển thị.

---

## 3. Bảo mật

### Mã hóa

```
Master Password (user nhập)
        │
        ▼
  Argon2id (KDF)
  salt = random 32 bytes
  memory = 64MB, iterations = 3
        │
        ▼
  Derived Key 256-bit
        │
        ▼
  SQLCipher mở file .hvault
  (AES-256-GCM toàn bộ file)
```

**Lý do SQLCipher thay vì field-level encryption:** Schema phức tạp (1 hosting → nhiều email, audit log, sync cache). File-level encryption đơn giản hơn, consistent hơn, và cho phép copy `.hvault` sang máy sếp dễ dàng.

### Key lifecycle

```
Unlock → Argon2id derive → SQLCipher open → plain data in RAM
                                                    │
                        Idle timeout / Lock          │
                                │                   ▼
                                └──── zeroize() key khỏi RAM
                                      (Rust crate 'zeroize')
```

### Rủi ro & Giải pháp

| Rủi ro | Giải pháp |
|---|---|
| cPanel block IP do spam | Manual sync, rate limit 1 lần/10 phút/service |
| Rule confuse bị lộ (NV nghỉ) | Đổi rule khi NV nghỉ, max 6 tháng/lần |
| Master pass bị quên | Không có recovery — document kỹ khi setup |
| NV chụp màn hình pass | Watermark tên IT vào màn hình reveal pass |
| SQLCipher corruption | WAL mode + backup tự động |

---

## 4. Database Schema

> Toàn bộ data là plain text bên trong SQLCipher — file-level encryption lo phần còn lại.

### V1 Schema (hiện tại)

```sql
-- Provider abstraction: chuẩn bị cho Phase 2+
-- provider_type: 'cpanel' | 'directadmin' | 'm365' | 'google_workspace'
-- auth_scheme:   'api_token' | 'oauth2_client_credentials' | 'oauth2_service_account'
-- V1 chỉ xử lý cpanel/directadmin + api_token, ignore giá trị khác

CREATE TABLE services (
    id              TEXT PRIMARY KEY,
    service_type    TEXT NOT NULL,      -- 'web_hosting'|'email_hosting'|'domain'|'vps'
    display_name    TEXT NOT NULL,
    provider        TEXT NOT NULL,      -- "Namecheap" | "Viettel IDC" | "Mắt Bão"
    provider_type   TEXT NOT NULL DEFAULT 'cpanel',   -- ← abstraction hook
    auth_scheme     TEXT NOT NULL DEFAULT 'api_token', -- ← abstraction hook
    portal_url      TEXT,
    server_ip       TEXT,
    nameservers     TEXT,               -- JSON array
    package_name    TEXT,
    domain_primary  TEXT,
    expires_at      INTEGER,            -- Unix timestamp
    auto_renew      INTEGER DEFAULT 0,
    monthly_cost    REAL,               -- V1: nhập tay. Phase 3: computed từ licenses
    status          TEXT DEFAULT 'active',
    notes           TEXT,
    created_at      INTEGER DEFAULT (unixepoch()),
    updated_at      INTEGER DEFAULT (unixepoch())
);

-- Credential: cPanel token, FTP, SSH
-- CHỈ dùng cho auth_scheme = 'api_token'
-- OAuth provider KHÔNG dùng bảng này
CREATE TABLE service_credentials (
    id              TEXT PRIMARY KEY,
    service_id      TEXT NOT NULL REFERENCES services(id),
    label           TEXT NOT NULL,      -- "cPanel Admin" | "FTP backup"
    username        TEXT NOT NULL,
    password        TEXT NOT NULL,
    credential_type TEXT DEFAULT 'panel', -- 'panel'|'ftp'|'ssh'|'api_key'
    notes           TEXT,
    last_used_at    INTEGER,
    created_at      INTEGER DEFAULT (unixepoch())
);

CREATE TABLE email_accounts (
    id              TEXT PRIMARY KEY,
    service_id      TEXT NOT NULL REFERENCES services(id),
    email_address   TEXT NOT NULL,
    display_name    TEXT,               -- "Nguyễn Văn A - Kế toán"
    department      TEXT,
    password        TEXT NOT NULL,
    quota_mb        INTEGER DEFAULT 500,
    is_active       INTEGER DEFAULT 1,
    must_change_password INTEGER DEFAULT 1,
    last_reset_at   INTEGER,
    created_at      INTEGER DEFAULT (unixepoch()),
    updated_at      INTEGER DEFAULT (unixepoch()),
    UNIQUE(service_id, email_address)
);

CREATE TABLE domains (
    id              TEXT PRIMARY KEY,
    service_id      TEXT REFERENCES services(id),
    domain_name     TEXT NOT NULL UNIQUE,
    registrar       TEXT,
    registrar_url   TEXT,
    expires_at      INTEGER,
    auto_renew      INTEGER DEFAULT 0,
    notes           TEXT,
    created_at      INTEGER DEFAULT (unixepoch())
);

CREATE TABLE ssl_certs (
    id              TEXT PRIMARY KEY,
    domain_id       TEXT REFERENCES domains(id),
    issuer          TEXT,
    expires_at      INTEGER,
    notes           TEXT,
    created_at      INTEGER DEFAULT (unixepoch())
);

CREATE TABLE activity_log (
    id              TEXT PRIMARY KEY,
    entity_type     TEXT NOT NULL,
    entity_id       TEXT NOT NULL,
    action          TEXT NOT NULL,      -- 'password_reset'|'created'|'viewed_password'
    actor_note      TEXT,               -- "IT - Tuấn"
    old_value_hint  TEXT,               -- "***abc" (3 ký tự cuối)
    new_value_hint  TEXT,
    confuse_used    TEXT,               -- prefix/suffix đã dùng khi gửi
    created_at      INTEGER DEFAULT (unixepoch())
);

CREATE TABLE cpanel_sync_cache (
    id              TEXT PRIMARY KEY,
    service_id      TEXT NOT NULL REFERENCES services(id),
    last_synced_at  INTEGER,
    email_count     INTEGER,
    disk_used_mb    INTEGER,
    disk_quota_mb   INTEGER,
    sync_status     TEXT DEFAULT 'never',
    error_msg       TEXT
);

CREATE TABLE app_settings (
    key             TEXT PRIMARY KEY,
    value           TEXT NOT NULL
);
```

---

## 5. Provider Abstraction — Lộ trình Schema

### Nguyên tắc cốt lõi

`provider_type` + `auth_scheme` là abstraction boundary. Code nhìn vào 2 cột này để route đến đúng client. Thêm provider mới = thêm giá trị enum + implement client, không động vào schema cũ.

### Phase 2 — OAuth Providers (khi mua M365 hoặc Google Workspace)

```sql
-- Thêm bảng này, KHÔNG sửa service_credentials
CREATE TABLE oauth_credentials (
    id                      TEXT PRIMARY KEY,
    service_id              TEXT NOT NULL REFERENCES services(id),
    auth_scheme             TEXT NOT NULL,

    -- Client Credentials (M365)
    tenant_id               TEXT,
    client_id               TEXT,
    client_secret           TEXT,

    -- Service Account (Google Workspace)
    service_account_json    TEXT,       -- JSON key file, encrypted trong SQLCipher

    -- Runtime cache — không phải source of truth
    access_token            TEXT,
    token_expires_at        INTEGER,

    created_at              INTEGER DEFAULT (unixepoch()),
    updated_at              INTEGER DEFAULT (unixepoch())
);
```

**Mapping:**
- `m365` → `oauth2_client_credentials` → đọc `oauth_credentials`
- `google_workspace` → `oauth2_service_account` → đọc `oauth_credentials`
- `cpanel` / `directadmin` → `api_token` → đọc `service_credentials`

### Phase 3 — License Tracking (khi cần báo cáo chi phí per-user)

```sql
CREATE TABLE subscription_licenses (
    id              TEXT PRIMARY KEY,
    service_id      TEXT NOT NULL REFERENCES services(id),
    license_type    TEXT NOT NULL,      -- 'microsoft_365_business_basic' | 'google_workspace_starter'
    total_seats     INTEGER NOT NULL,
    used_seats      INTEGER DEFAULT 0,
    cost_per_seat   REAL,
    billing_cycle   TEXT DEFAULT 'monthly',
    next_billing_at INTEGER,
    created_at      INTEGER DEFAULT (unixepoch())
);
-- monthly_cost trong services trở thành computed: cost_per_seat * used_seats
```

### Invariants — KHÔNG được vi phạm

```
1. KHÔNG dùng service_credentials cho OAuth provider.
   OAuth credentials luôn vào oauth_credentials.

2. provider_type và auth_scheme phải consistent:
   cpanel / directadmin  →  api_token
   m365                  →  oauth2_client_credentials
   google_workspace      →  oauth2_service_account
   Thêm provider mới: define cả 2 trước khi code.

3. access_token trong oauth_credentials là cache, không phải source of truth.
   Hết hạn → re-fetch tự động, không alert user.

4. subscription_licenses chỉ cho subscription provider.
   cPanel / DirectAdmin không có record trong bảng này.

5. V1 code PHẢI ignore provider_type khác 'cpanel'/'directadmin'.
   Không throw error — log warning và skip.
```

---

## 6. Tính năng

### Dashboard (cả 2 role)

**Sếp thấy:** tổng chi phí tháng, số dịch vụ, cảnh báo hết hạn (đỏ/vàng/xanh), provider breakdown.

**IT thấy:** toàn bộ trên + disk usage, email count, sync status, nút action.

Alert threshold:
- 🔴 Đỏ: hết hạn < 7 ngày
- 🟡 Vàng: hết hạn 7–30 ngày
- 🟢 Xanh: còn lại

### Quản lý Email (IT only)

- Tạo email mới + sinh password CSPRNG
- Reset password qua cPanel UAPI
- Flag `must_change_password`
- Ghi audit log mỗi thao tác

### Sync — Manual, có rate limit

```
User bấm "Refresh từ cPanel"
  → Rate limit: tối đa 1 lần / 10 phút / service
  → Hiện timestamp "Cập nhật lúc 14:32"
  → Tạo email: lưu local (status = pending_sync) → user review → Apply lên cPanel
```

**Không auto-sync** — tránh block IP cPanel.

### Confuse Engine

```
Pass thật:     Kd#9mNpQ
Rule công ty:  prefix = "hello", suffix = "2025abc"

Gửi Zalo:      helloKd#9mNpQ2025abc  +  "(bỏ 5 đầu 7 cuối)"
Gửi Email:     địa chỉ email + webmail + hướng dẫn cài đặt  (KHÔNG CÓ PASS)
```

Vault lưu pass thật. Confuse string chỉ sinh khi gửi, ghi vào audit log.

### cPanel UAPI Reference

```
Tạo email:    POST /execute/Email/add_pop
Đổi pass:     POST /execute/Email/passwd_pop
Danh sách:    GET  /execute/Email/list_pops
Xóa email:    POST /execute/Email/delete_pop

Auth:  Authorization: cpanel username:api_token
       (API Token, không dùng main password)
```

---

## 7. Roadmap

### Phase 1 — MVP

| Task | Priority |
|---|---|
| Vault engine: SQLCipher + Argon2id + lock/unlock + zeroize | P0 |
| Multi-profile UI + switcher | P0 |
| Dashboard hosting/domain với alert | P0 |
| Credential viewer (admin only) | P0 |
| Email account list per hosting | P0 |
| Password generator CSPRNG | P0 |
| Viewer mode (sync disabled, badge) | P0 |
| Confuse engine + tin nhắn tự động | P1 |
| Audit log viewer | P1 |
| cPanel UAPI: tạo/reset/xóa email | P1 |
| Rate limit sync | P1 |

### Phase 2 — OAuth Providers

| Task | Priority |
|---|---|
| oauth_credentials table | P0 |
| M365 Graph API client (Rust) | P0 |
| Google Workspace Admin SDK client | P1 |
| Token refresh tự động | P0 |

### Phase 3 — License & Reporting

| Task | Priority |
|---|---|
| subscription_licenses table | P0 |
| Chi phí computed từ seats | P0 |
| Export PDF báo cáo tháng | P1 |
| Alert gia hạn license | P1 |
| Backup tự động định kỳ | P2 |

# AGENTS.md — Hosting Vault

> Đọc file này TRƯỚC KHI làm bất kỳ task nào.
> Đây là source of truth cho mọi quyết định kiến trúc.

---

## 1. Project Overview

Tauri 2.x desktop app quản lý hosting/email/domain credential.
- **Backend:** Rust
- **Frontend:** React + TypeScript
- **Storage:** SQLCipher (`.hvault` file, AES-256-GCM)
- **Spec đầy đủ:** `docs/hosting-vault-spec.md`

---

## 2. Quy trình bắt buộc mỗi task

```
1. Đọc docs/hosting-vault-spec.md — xác định module liên quan
2. Đọc .context/GLOBAL.md — xem module index
3. Load context module: context-gen load <module> . --include-manual
4. Nếu [manual] còn template placeholder → DỪNG, hỏi lại human
5. TDD: viết test FAIL trước → implement → test PASS
6. context-gen build . --quiet && git add .context/
```

**Verification gate trước khi commit:**
```bash
context-gen build . --quiet && cargo test 2>&1 | tail -10
```

---

## 3. Kiến trúc — Những gì KHÔNG được làm

### Storage
- **KHÔNG** dùng `service_credentials` cho OAuth provider (M365, Google Workspace)
- **KHÔNG** lưu `access_token` dài hạn — đây là runtime cache, expire là re-fetch
- **KHÔNG** dùng SQLite thường — phải là SQLCipher

### Security
- **KHÔNG** dùng `drop()` để clear key — phải dùng `zeroize()` crate
- **KHÔNG** log credential dù là debug log
- **KHÔNG** lưu master password dưới bất kỳ hình thức nào
- **KHÔNG** gọi cPanel API bằng main password — chỉ API Token

### Provider Abstraction
- **KHÔNG** hardcode `"cpanel"` trong business logic — đọc `provider_type` từ DB
- **KHÔNG** thêm provider mới mà không define cả `provider_type` lẫn `auth_scheme`
- **KHÔNG** đọc `oauth_credentials` cho `auth_scheme = 'api_token'` và ngược lại

### Sync
- **KHÔNG** auto-sync — chỉ sync khi user bấm "Refresh"
- **KHÔNG** sync quá 1 lần / 10 phút / service — enforce rate limit cứng

### Viewer mode
- **KHÔNG** enable nút write khi `machine_role = viewer`
- **KHÔNG** gọi Provider API khi `sync_enabled = false`

---

## 4. Provider Routing — Rule cứng

```
provider_type       auth_scheme                    credential table
─────────────────────────────────────────────────────────────────
cpanel              api_token                      service_credentials
directadmin         api_token                      service_credentials
m365                oauth2_client_credentials      oauth_credentials      [Phase 2]
google_workspace    oauth2_service_account         oauth_credentials      [Phase 2]
```

Nếu gặp `provider_type` không có trong bảng trên → log warning, KHÔNG throw panic, KHÔNG crash app.

---

## 5. Phase hiện tại: V1

V1 chỉ implement `cpanel` và `directadmin`. Code phải:

```rust
match service.provider_type.as_str() {
    "cpanel" | "directadmin" => { /* implement */ }
    other => {
        warn!("Provider '{}' not implemented in V1, skipping", other);
        return Ok(());
    }
}
```

**KHÔNG** implement OAuth flow trong V1 dù schema đã có chỗ.

---

## 6. Schema — Thứ tự ưu tiên khi conflict

1. `docs/hosting-vault-spec.md` — source of truth
2. Migration file mới nhất trong `migrations/`
3. Code hiện tại

Nếu code conflict với spec → sửa code, không sửa spec. Nếu spec cần sửa → tạo tension HIGH.

---

## 7. Tension Register

Khi detect conflict giữa task và constraint trong spec/[manual]:

```markdown
## [timestamp] | [module]
Tension:    mô tả conflict
Context:    đang làm task gì
Proposal:   muốn làm gì
Constraint: rule nào conflict (trích dẫn từ spec hoặc [manual])
Severity:   low | high
Decision:   [human fill in]
```

Ghi vào `.context/TENSIONS.md`.

**Routing:**
- `low` → tiếp tục conservative, human review sau
- `high` → tạo `HANDSHAKE_<timestamp>.md`, dừng task, chờ `.resolved`

**Ví dụ tension hợp lệ:**
- Task yêu cầu auto-sync → conflict với "KHÔNG auto-sync" → HIGH
- Task thêm provider mới không define auth_scheme → HIGH
- Task log credential để debug → HIGH (security)

---

## 8. Confuse Engine — Constraint

- Vault lưu **pass thật**, không bao giờ lưu confuse string
- Confuse string chỉ sinh tại thời điểm gửi thông báo
- `confuse_used` trong `activity_log` là audit trail sau khi gửi
- Rule confuse (prefix/suffix) đọc từ `app_settings`, không hardcode

---

## 9. cPanel UAPI — Reference

```
POST /execute/Email/add_pop       ← Tạo email
POST /execute/Email/passwd_pop    ← Đổi password
GET  /execute/Email/list_pops     ← Danh sách
POST /execute/Email/delete_pop    ← Xóa email

Header: Authorization: cpanel {username}:{api_token}
```

Tạo email flow: lưu local `status = pending_sync` → user confirm → Apply → update `status = synced` → ghi `activity_log`.

---

## 10. Test Strategy

```
src-tauri/src/
  vault/          → unit test: encrypt/decrypt, zeroize
  providers/      → mock API, test routing logic
  confuse/        → unit test: sinh string, decode
  sync/           → test rate limit enforcement

src/ (frontend)
  → test: viewer mode disables write buttons
  → test: alert threshold đúng ngưỡng 7/30 ngày
```

Cargo test PHẢI pass trước khi commit. Frontend test chạy qua `npm test`.

---

## 11. File Structure

```
hosting-vault/
  docs/
    hosting-vault-spec.md     ← Spec chính
  src-tauri/
    src/
      vault/                  ← SQLCipher engine, key lifecycle
      providers/              ← cPanel client, router
      confuse/                ← Confuse engine
      sync/                   ← Sync worker + rate limit
      commands/               ← Tauri IPC commands
    migrations/               ← SQL migration files
  src/
    components/
      Dashboard/              ← 2 view: admin + viewer
      EmailManager/
      HostingList/
    hooks/
  .context/
    GLOBAL.md
    TENSIONS.md
```

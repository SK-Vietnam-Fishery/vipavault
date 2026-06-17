# Decision Record: Vault Storage Architecture

**Status:** APPROVED  
**Date:** 2026-06-17  
**Approved by:** Human — "Giữ SQLCipher"  
**Milestone:** V1 / M0–M1 boundary  
**Related tensions:** `storage`, `autofill_mechanism`, `vault_tier_model` (new)  
**Severity:** HIGH

---

## 1. Câu hỏi cần quyết định

VipaVault nên dùng gì làm lớp lưu trữ mã hóa chính?

| Phương án | Mô tả |
|---|---|
| **A — SQLCipher (hiện tại)** | 1 file `.hvault`, SQLite quan hệ + AES-256-GCM toàn file, Argon2id KDF |
| **B — KeePass hybrid** | KDBX chỉ giữ secrets theo cây phân cấp; metadata (services, sync, audit) ở store thứ 2 |
| **C — KDBX-only** | Toàn bộ data map sang Group/Entry KeePass |

Brainstorm bổ sung: mô hình **3–4 tầng** trong vault:

- **Tầng 1 — Công ty:** profile/vault per company (manager theo công ty)
- **Tầng 2 — Nhóm dịch vụ:** hosting, email server, domain, license keys
- **Tầng 3 — Credential con:** MariaDB, PostgreSQL, cPanel, SSH trên cùng 1 VPS
- **(Tầng 4 — tùy chọn):** sub-credential hoặc attachment

Đề xuất brainstorm: KeePass đóng vai trò quản lý pass; Rust wrapper ánh xạ phân tầng.

---

## 2. KeePass/KDBX — sự thật kỹ thuật (có dẫn chứng)

> KeePass **không có sẵn trong repo VipaVault**. Đây là format/phần mềm **bên ngoài** có thể tích hợp.

### 2.1 KDBX là gì?

Theo [KDBX File Format Specification](https://keepass.info/help/kb/kdbx.html):

- KDBX là format database KeePass 2.x, lưu username, password, URL, v.v.
- Hỗ trợ **encryption, data authentication, compression, extensibility**
- Spec mới nhất: **KDBX 4.1** (format version `0x00040001`)

### 2.2 Crypto trong KDBX 4.x

Theo [KDBX spec — Header & Keys](https://keepass.info/help/kb/kdbx.html):

| Thành phần | Giá trị được hỗ trợ |
|---|---|
| File encryption | AES-256 (CBC, PKCS#7) hoặc ChaCha20 |
| KDF | AES-KDF, **Argon2d**, **Argon2id** |
| Header auth | HMAC-SHA-256 |
| Data auth | **Encrypt-then-MAC** (HMAC-SHA-256 trên ciphertext) |

Theo [KDBX 4 changes](https://keepass.info/help/kb/kdbx_4.html):

- Argon2 được thêm từ KDBX 4 — memory-hard, chống GPU/ASIC tốt hơn AES-KDF
- Encrypt-then-MAC thay MAC-then-Encrypt của KDBX 3.1

Theo [KeePass Security — Database Encryption](https://keepass.info/help/base/security.html):

- Toàn bộ database được mã hóa (không chỉ password)
- Master key → SHA-256 compress → KDF với random salt → derived key

**Claim:** KDBX cung cấp crypto battle-tested tương đương mục tiêu VipaVault (AES-256 + Argon2id).  
**Dẫn chứng:** Đúng theo spec — nhưng VipaVault spec đã chọn SQLCipher đạt mục tiêu tương tự (`docs/vipavault-spec.md` §3).

### 2.3 Cấu trúc dữ liệu — Group/Entry tree

Theo [KeePassXC specs — KeePass2 XML RFC](https://github.com/keepassxreboot/keepassxc-specs/blob/master/kdbx-xml/rfc.txt) §2.1:

> Database comprises meta data section and data section as **tree structure of groups and entries**. A group can contain sub groups and entries. Database MUST have exactly one root group.

Theo [KDBX 4 — Extensible Entries and Groups](https://keepass.info/help/kb/kdbx_4.html):

- KDBX 4+ có `CustomData` dictionary trên Entry và Group — plugin/app có thể lưu metadata tùy chỉnh

**Claim:** Mô hình 3 tầng (Company → Service → Credential) map tự nhiên sang nested Groups.  
**Dẫn chứng:** Đúng cho **secrets và hierarchy**. Không đúng cho relational query (dashboard, sync cache).

### 2.4 Inner encryption & process memory

Theo [KDBX spec — Inner Encryption](https://keepass.info/help/kb/kdbx.html):

> Inner encryption has **no effect on the cryptographic security** of the KDBX file format. Purpose is **process memory protection** only.

Theo [KeePass Security — Process Memory Protection](https://keepass.info/help/base/security.html):

- Sensitive data encrypted in RAM; master key erased when not needed
- Usernames, notes **không** được memory-protect mặc định — chỉ master key và entry passwords

**Claim:** KDBX inner encryption ≠ thay thế app-level security (confuse, audit, viewer gate).

---

## 3. KeePass vs KeePassXC — chọn cái nào?

### 3.1 Khác nhau thực chất

| | **KeePass 2.x** | **KeePassXC** |
|---|---|---|
| Tác giả / maintainer | Dominik Reichl | KeePassXC Team (community fork) |
| Platform chính | Windows (.NET) | **Linux, macOS, Windows** (Qt/C++) |
| Spec KDBX | **Authoritative** — [keepass.info/help/kb/](https://keepass.info/help/kb/kdbx.html) | **Implementer** — follow spec; docs bổ sung WIP tại [keepassxc-specs](https://github.com/keepassxreboot/keepassxc-specs) |
| Mở `.kdbx` | Native | Native — [User Guide: Opening Database](https://keepassxc.org/docs/KeePassXC_UserGuide/#_opening_an_existing_database) |
| CLI automation | Hạn chế | **keepassxc-cli** — [User Guide § Command Line Tool](https://keepassxc.org/docs/KeePassXC_UserGuide/#_command_line_tool) |
| License | GPL-2 | GPL-3 |
| Phù hợp WSL/Linux dev | Kém | **Tốt** |

KeePassXC RFC §1 nói rõ:

> KDBX is used by **KeePass 2.x, KeePassXC and various other** password managers.

→ Cả hai dùng **cùng container format**; không phải 2 format khác nhau.

### 3.2 Khuyến nghị cho VipaVault (nếu có interop KDBX sau này)

**VipaVault là Tauri app (Rust) — không embed KeePass hay KeePassXC làm runtime.**

| Vai trò | Chọn gì | Lý do |
|---|---|---|
| **Spec / implementation target** | **KeePass KDBX 4.1 spec** ([keepass.info](https://keepass.info/help/kb/kdbx.html)) | Normative, đầy đủ, versioned |
| **Rust library** | **`keepass-db` crate** ([docs.rs](https://docs.rs/keepass-db/latest/keepass_db/)) | Read/write KDBX 4.x, Argon2, Group/Entry API |
| **Interop test (IT mở file)** | **KeePassXC** | Cross-platform, CLI, active maintenance, phù hợp Linux/WSL |
| **Không chọn** | KeePass 2.x làm dependency | Windows-centric; VipaVault target desktop đa nền |

**Kết luận §3:** Nếu có interop KDBX → follow **KeePass spec**, test bằng **KeePassXC**. Không "chọn KeePass thay KeePassXC" ở tầng format — chọn **spec author** + **interop client** khác nhau cho đúng vai trò.

---

## 4. Phân tích fit với VipaVault

### 4.1 Điểm mạnh phương án KeePass (Tier 3)

Mô hình brainstorm:

```
CompanyA/                    ← Tier 1
  Hosting/
    VPS-Prod-01/             ← Tier 2
      cPanel Admin           ← Tier 3 (entry)
      MariaDB root           ← Tier 3 (entry)
      PostgreSQL admin       ← Tier 3 (entry)
```

Map trực tiếp sang KDBX Groups/Entries — **đúng mental model IT**.

### 4.2 Phản biện — vì sao KeePass không đủ làm "backend vault"

#### (a) VipaVault cần relational data, không chỉ secrets

`docs/vipavault-spec.md` §4 định nghĩa 8+ bảng: `services`, `email_accounts`, `cpanel_sync_cache`, `activity_log`, ...

| Data | Ví dụ | Fit KDBX? |
|---|---|---|
| Secret | password, API token | Entry password field — **tốt** |
| Metadata | expires_at, monthly_cost, provider_type | CustomData hoặc SQL — **kém** |
| Workflow state | pending_sync, sync_status | High-churn — **rất kém** |
| Audit | confuse_used, old_value_hint | Structured log — **không fit** |
| Dashboard query | SUM(monthly_cost), alert < 7 days | Cần SQL aggregation — **không có trong KDBX** |

→ Hybrid (KDBX + SQL) là **bắt buộc**, không phải tùy chọn.

#### (b) Tier 1 (công ty) — spec đã có

```
~/.vipavault/company_a.hvault  ≈  company_a.kdbx
```

Multi-profile trong spec §2 — **đổi format không thêm giá trị** ở Tier 1.

#### (c) Tier 3 — schema đã cover

`service_credentials` với FK `service_id` + `credential_type` — thêm `mariadb`, `postgres` là đủ. Vấn đề là **UI tree**, không phải storage backend.

#### (d) Dual-store sync risk

Tạo VPS → INSERT SQL + CREATE KDBX groups. Xóa service → orphan entries. Wrapper phải là **distributed transaction coordinator** — phức tạp hơn 1 SQLCipher file với FK.

#### (e) Security regression

Spec §6 Confuse Engine: vault lưu pass thật; confuse chỉ lúc gửi.

Nếu IT mở `.kdbx` bằng KeePassXC → copy pass thật trực tiếp → **bypass confuse, watermark, activity_log**.

KeePassXC có screenshot protection ([User Guide § Screenshot Security](https://keepassxc.org/docs/KeePassXC_UserGuide/#_screenshot_security)) — nhưng không thay audit trail VipaVault.

#### (f) Quyết định cũ vẫn hợp lệ

`.context/TENSIONS_ACTIVE.md` entry `storage` (SQLCipher) và `autofill_mechanism` (drop KeePass) — rationale vẫn đúng sau brainstorm.

---

## 5. Bảng so sánh tổng hợp

| Tiêu chí | SQLCipher `.hvault` | KeePass KDBX hybrid |
|---|---|---|
| Tier 1 company isolation | 1 file/profile | 1 `.kdbx`/profile — tương đương |
| Tier 2–3 hierarchy | FK + UI tree | Native groups — **UX tốt hơn** |
| Dashboard / alerts | SQL query | Cần SQL anyway |
| Sync cache + audit | Native tables | SQL anyway |
| 1 source of truth | **Có** | **Không** (2 stores) |
| Copy sang viewer | 1 file | 1–2 files |
| Crypto surface | SQLCipher + Argon2id | keepass-db + (SQL?) |
| Interop KeePassXC | Không | Có — **nhưng bypass app controls** |
| M1 implementation | Đã planned | Split vault + mapping layer |
| Align spec hiện tại | **Có** | Cần rewrite §2–4 |

---

## 6. Quyết định (APPROVED)

### 6.1 Primary vault storage

**GIỮ SQLCipher + `.hvault`** theo spec và tension `storage`.

**KHÔNG** dùng KeePass/KDBX làm backend mã hóa chính cho VipaVault V1.

### 6.2 Phân tầng 3–4 lớp

**Implement ở tầng data model + UI**, không đổi crypto backend:

- Tier 1: multi-profile `.hvault` (đã có trong spec)
- Tier 2: `services` + `service_type` grouping
- Tier 3: `service_credentials` — mở rộng `credential_type`: `mariadb`, `postgres`, `redis`, ...
- UI: nested tree view từ FK quan hệ

### 6.3 KeePass/KeePassXC — vai trò (nếu cần sau này)

| Vai trò | Cho phép? |
|---|---|
| Embed KeePass/KeePassXC runtime | **Không** |
| Target KDBX 4.1 spec cho export/import | **Có — Phase 2+**, optional |
| Spec reference | [keepass.info KDBX 4.1](https://keepass.info/help/kb/kdbx.html) |
| Interop test client | **KeePassXC** + `keepassxc-cli` |
| Rust crate nếu implement | `keepass-db` |

### 6.4 Cập nhật tensions

- `storage`: giữ nguyên — không reopen
- `autofill_mechanism`: KeePass dropped cho autofill; KDBX export là scope khác (xem entry `vault_tier_model`)
- `vault_tier_model`: hierarchy qua SQL FK + UI, không qua KDBX

---

## 7. Điều kiện reopen

Chỉ xem xét KeePass backend khi có **requirement cứng**:

1. IT **bắt buộc** chỉnh sửa vault bằng KeePassXC desktop hàng ngày, hoặc
2. Compliance yêu cầu file vault ở format KDBX chuẩn, hoặc
3. PoC chứng minh hybrid sync đáp ứng dashboard + audit + confuse invariant

Khi đó: spike `keepass-db` + document dual-store transaction design trước M1.

---

## 8. Tài liệu tham chiếu

| Nguồn | URL | Dùng cho |
|---|---|---|
| KDBX 4.1 File Format Specification | https://keepass.info/help/kb/kdbx.html | Crypto, structure, CustomData |
| KDBX 4 Changes | https://keepass.info/help/kb/kdbx_4.html | Argon2, Encrypt-then-MAC |
| KeePass Security | https://keepass.info/help/base/security.html | Encryption, KDF, memory protection |
| KeePassXC User Guide | https://keepassxc.org/docs/KeePassXC_UserGuide/ | Interop, CLI, `.kdbx` workflow |
| KeePassXC KDBX XML RFC (WIP) | https://github.com/keepassxreboot/keepassxc-specs/blob/master/kdbx-xml/rfc.txt | Group/Entry tree structure |
| keepass-db Rust crate | https://docs.rs/keepass-db/latest/keepass_db/ | Rust integration feasibility |
| VipaVault Spec §2–4 | docs/vipavault-spec.md | Schema, SQLCipher, multi-profile |
| TENSIONS_ACTIVE | .context/TENSIONS_ACTIVE.md | storage, autofill_mechanism, vault_tier_model |

---

## 9. Human sign-off

- [x] Đồng ý §6 — Giữ SQLCipher, hierarchy qua UI/FK
- [ ] Không đồng ý — reopen HIGH tension, spike KeePass hybrid
- Ghi chú: Human xác nhận "Giữ SQLCipher" @ 2026-06-17
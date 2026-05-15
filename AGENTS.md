# AGENTS.md — VipaVault

> Đọc file này TRƯỚC KHI làm bất kỳ task nào.
> Đây là source of truth cho mọi quyết định kiến trúc.

---

## 1. Project Overview

Tauri 2.x desktop app quản lý hosting/email/domain credential.
- **Backend:** Rust
- **Frontend:** React + TypeScript
- **Storage:** SQLCipher (`.hvault` file, AES-256-GCM)
- **Spec đầy đủ:** `docs/vipavault-spec.md`

---

## 2. Context Map — Hiểu trước khi làm

Project này dùng **context-gen** để quản lý context cho agent. Đây không phải tool tùy chọn — đây là cách duy nhất để mày biết codebase đang ở trạng thái nào và constraints nào đang áp dụng.

### Tại sao có tool này

Source code chỉ nói **cái gì đang tồn tại** (factual). Nó không nói:
- Tại sao code làm thế này
- Cái gì không được làm
- Cái gì chưa implement nhưng đã có quyết định

`context-gen` sinh file `.context/<module>.md` cho mỗi module bằng cách parse AST. Mỗi file có 2 vùng tách biệt:

```
<!-- AUTO_START -->
[auto] — do tool sinh từ AST: public functions, structs, Tauri commands
         Tool sẽ overwrite vùng này mỗi lần build. KHÔNG sửa tay.
<!-- AUTO_END -->

<!-- MANUAL_START -->
[manual] — do human viết: design decisions, invariants, constraints, behavior chưa implement
           Tool KHÔNG BAO GIỜ động vào vùng này.
<!-- MANUAL_END -->
```

### Rule đọc context

Trước khi làm bất kỳ task nào liên quan đến module X:

```bash
context-gen load src-tauri/src/vault . --include-manual
```

Lệnh này in ra context của module — cả [auto] lẫn [manual]. Đọc **[manual] trước**. [manual] chứa constraints mà code không thể hiện được.

### Nếu [manual] còn placeholder

File mới sinh ra có template như:

```
[manual] Design Decisions
<!-- Viết tại đây -->

[manual] Invariants & Constraints
<!-- Viết tại đây -->
```

Nếu thấy `<!-- Viết tại đây -->` → **DỪNG task**. Hỏi human điền trước. Làm khi chưa có [manual] = làm khi chưa biết constraints = có thể vi phạm invariant mà không biết.

### Sau khi thay đổi code

```bash
context-gen build . --quiet
```

Lệnh này rebuild [auto] section cho tất cả module. Chạy sau mỗi lần thay đổi để [auto] không bị stale. Commit `.context/` cùng với code.

### Module index

`.context/GLOBAL.md` chứa danh sách tất cả module và mô tả ngắn. Đọc file này đầu tiên nếu chưa biết task liên quan đến module nào.

---

## 3. Bootstrap — Chạy một lần khi setup project

```bash
# Cài context-gen
pip install context-gen

# Sinh context lần đầu từ toàn bộ source
context-gen build . --quiet

# Kiểm tra output
ls .context/
# Phải thấy: GLOBAL.md, TENSIONS.md, và các file per-module

# Khởi tạo TENSIONS.md nếu chưa có
touch .context/TENSIONS.md
```

**Sau bước này**, mở từng file `.context/<module>.md` và điền `[manual]` section cho:
- `src-tauri_src_vault` → invariant zeroize, Argon2id params, lý do SQLCipher
- `src-tauri_src_providers` → routing table, lý do KHÔNG auto-sync

---

## 4. Quy trình bắt buộc mỗi task

```
START
  │
  ▼
Đọc docs/vipavault-spec.md
Xác định module liên quan
  │
  ▼
context-gen load <module> . --include-manual
Đọc [manual] section
  │
  ├── [manual] còn placeholder? → DỪNG. Hỏi human điền trước.
  │
  ▼
Detect tension với constraint trong spec / [manual]?
  │
  ├── Có → ghi vào .context/TENSIONS.md (xem Section 8)
  │         LOW  → tiếp tục conservative
  │         HIGH → tạo HANDSHAKE_<timestamp>.md, DỪNG, chờ .resolved
  │
  ▼
Viết test FAIL trước (TDD)
  │
  ▼
Implement
  │
  ▼
cargo test PASS + npm test PASS
  │
  ▼
context-gen build . --quiet
  │
  ▼
git add .context/ && git commit
  │
DONE
```

**Verification gate — bắt buộc trước commit:**
```bash
context-gen build . --quiet && cargo test 2>&1 | tail -10
```

---

## 5. Kiến trúc — Những gì KHÔNG được làm

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

## 6. Provider Routing — Rule cứng

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

## 7. Phase hiện tại: V1

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

## 8. Schema — Thứ tự ưu tiên khi conflict

1. `docs/vipavault-spec.md` — source of truth
2. Migration file mới nhất trong `migrations/`
3. Code hiện tại

Nếu code conflict với spec → sửa code, không sửa spec. Nếu spec cần sửa → tạo tension HIGH.

---

## 9. Tension Register & Decision Log

`TENSIONS.md` có **2 loại entry**. Đọc cả hai trước khi làm task — đặc biệt là Decision Log, vì nó chứa những thứ đã bị reject mà mày không được propose lại.

### Loại 1 — Tension chưa resolve (open)

Khi detect conflict giữa task và constraint trong spec/[manual]:

```markdown
## [timestamp] | [module] | OPEN
Tension:    mô tả conflict
Context:    đang làm task gì
Proposal:   muốn làm gì
Constraint: rule nào conflict (trích dẫn từ spec hoặc [manual])
Severity:   low | high
Decision:   [human fill in]
```

**Routing:**
- `low` → tiếp tục conservative, human review sau
- `high` → tạo `HANDSHAKE_<timestamp>.md`, dừng task, chờ `.resolved`

**Ví dụ tension hợp lệ:**
- Task yêu cầu auto-sync → conflict với "KHÔNG auto-sync" → HIGH
- Task thêm provider mới không define auth_scheme → HIGH
- Task log credential để debug → HIGH (security)

### Loại 2 — Decision Log (resolved, vĩnh viễn)

Ghi lại những quyết định đã được cân nhắc và chốt. Mục đích: agent mới đọc vào biết đây là **quyết định có chủ ý**, không phải lỗ hổng cần fix.

```markdown
## [timestamp] | [module] | RESOLVED
Tension:     tên vấn đề đã được cân nhắc
Options:     A → ưu/nhược | B → ưu/nhược
Decision:    lựa chọn cuối cùng
Rationale:   lý do chọn, bằng chứng hoặc risk cụ thể
Constraint:  KHÔNG reopen trừ khi [điều kiện cụ thể]
Phase:       V1 | Phase2 | all
```

**Rule:** Entry RESOLVED không bao giờ bị xóa. Chỉ thêm entry mới nếu context thay đổi và cần reopen.

**Ví dụ thực tế của project này:**

```markdown
## 2026-05-15 | sync | RESOLVED
Tension:     Auto-sync 15 phút vs manual sync
Options:     Auto → tiện, real-time | nhưng risk block IP cPanel (không có SLA công khai)
             Manual → user friction | nhưng safe, controllable
Decision:    Manual sync, rate limit cứng 1 lần/10 phút/service
Rationale:   cPanel không public rate limit policy. Block IP = mất access toàn bộ
             hosting. Conservative là đúng khi downside không đối xứng.
Constraint:  KHÔNG reopen trừ khi có rate limit policy chính thức từ provider
Phase:       V1

## 2026-05-15 | storage | RESOLVED
Tension:     SQLCipher (file-level) vs field-level AES-GCM trong SQLite thường
Options:     Field-level → granular control | nhưng phức tạp, dễ miss field mới
             SQLCipher → toàn bộ file mã hóa | copy .hvault sang máy sếp là xong
Decision:    SQLCipher
Rationale:   Use case copy file sang máy viewer (sếp) làm SQLCipher rõ ràng hơn.
             Field-level tạo maintenance burden mỗi khi thêm column nhạy cảm.
Constraint:  KHÔNG reopen trừ khi có use case cần partial decryption
Phase:       all

## 2026-05-15 | provider | RESOLVED
Tension:     V1 support multi-provider (DirectAdmin, Plesk) hay chỉ cPanel
Options:     Multi → flexible | nhưng UAPI khác nhau, tăng scope V1
             cPanel only → đơn giản | nhưng DirectAdmin UAPI gần giống cPanel
Decision:    cPanel + DirectAdmin cùng client (UAPI tương đồng), abstract qua provider_type
Rationale:   Login flow giống nhau. Tách client chỉ khi có divergence thực tế.
Constraint:  KHÔNG thêm Plesk vào V1 — Plesk dùng XML-RPC khác hoàn toàn
Phase:       V1
```

### Convention trong [manual] — Phase tag & Deprecated marker

Khi viết [manual] section, tag phase để agent sau biết constraint nào sẽ thay đổi:

```markdown
[manual] Invariants — Phase: V1
Constraint X chỉ áp dụng Phase 1.
Phase 2 (OAuth): constraint này sẽ thay đổi — xem TENSIONS.md entry oauth_credentials.
```

Khi constraint cũ hết hiệu lực, KHÔNG xóa — mark deprecated:

```markdown
[manual] ~~Constraint Y~~ — DEPRECATED @ commit abc1234
Lý do: Phase 2 migrate sang oauth_credentials.
Thay bằng: xem [manual] oauth_credentials section.
```

Git blame trên dòng deprecated cho ra full history. Không cần tool thêm.

---

## 10. Confuse Engine — Constraint

- Vault lưu **pass thật**, không bao giờ lưu confuse string
- Confuse string chỉ sinh tại thời điểm gửi thông báo
- `confuse_used` trong `activity_log` là audit trail sau khi gửi
- Rule confuse (prefix/suffix) đọc từ `app_settings`, không hardcode

---

## 11. cPanel UAPI — Reference

```
POST /execute/Email/add_pop       ← Tạo email
POST /execute/Email/passwd_pop    ← Đổi password
GET  /execute/Email/list_pops     ← Danh sách
POST /execute/Email/delete_pop    ← Xóa email

Header: Authorization: cpanel {username}:{api_token}
```

Tạo email flow: lưu local `status = pending_sync` → user confirm → Apply → update `status = synced` → ghi `activity_log`.

---

## 12. Test Strategy

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

## 13. File Structure

```
vipavault/
  docs/
    vipavault-spec.md         ← Spec chính
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

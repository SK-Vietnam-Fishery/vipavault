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

Layout `.context/` theo mô hình phân tầng (tham khảo `../skvn-marine`). Chi tiết folder: **`.context/README.md`**.

Project dùng **context-gen** cho AST facts + **`.context/modules/`** cho decisions/invariants human-readable.
Máy local có thể có hướng dẫn riêng trong `LOCAL_CONTEXT_GEN.md` (gitignored).

### Đọc trước — Bắt buộc mỗi task

```
1. Đọc .context/GLOBAL.md          → stack, module index
2. Đọc .context/PROJECT.md         → quyết định project-wide
3. Đọc .context/MILESTONES.md      → milestone / execution boundary
4. Đọc .context/TENSIONS_OPEN.md   → toàn bộ OPEN tensions
5. Đọc .context/TENSIONS_ACTIVE.md → RESOLVED_ACTIVE (tag filter)
6. Đọc .context/modules/<module>.md → module sắp sửa
```

Nếu `.context/modules/<module>.md` chưa tồn tại → đọc `GLOBAL.md` + hỏi human trước khi tạo.

### Context load rules

| Folder | Load mặc định? | Mục đích |
|---|---|---|
| Root (`GLOBAL`, `PROJECT`, `MILESTONES`, `TENSIONS_*`) | Có | Governance |
| `modules/` | Có — theo task | Design decisions, invariants |
| `generated/` | Qua `context-gen load` | AST: functions, structs, commands |
| `decisions/` | Khi liên quan | Decision records đã approve |
| `planning/` | Không | Roadmap chi tiết (`MILESTONES_REFERENCE.md`) |
| `proposals/` | Không | Brainstorm chưa approve |

`TENSIONS_HISTORY.md` và `planning/` — chỉ đọc khi audit hoặc human yêu cầu.

### context-gen — AST layer

`context-gen` sinh `.context/generated/<path>.md` từ AST:

```
<!-- AUTO_START -->  … regenerate mỗi build, KHÔNG sửa tay … <!-- AUTO_END -->
<!-- MANUAL_START --> … tool giữ nguyên nếu có trong generated file … <!-- MANUAL_END -->
```

**Source of truth cho [manual] protocol:** `.context/modules/<MODULE>.md` (human-friendly).
**AST refresh:**

```bash
context-gen build . --quiet
context-gen load src-tauri/src/vault . --include-manual
```

### Nếu [manual] còn placeholder

Trong `.context/modules/<module>.md`, nếu thấy `_Chưa có ghi chú._` hoặc `<!-- Viết tại đây -->` → **DỪNG task**, hỏi human điền trước.

Module index (domain → file):

| Module | File |
|---|---|
| Vault / SQLCipher | `modules/VAULT.md` |
| Providers | `modules/PROVIDERS.md` |
| Sync | `modules/SYNC.md` |
| Confuse | `modules/CONFUSE.md` |
| Tauri commands | `modules/COMMANDS.md` |
| React UI | `modules/FRONTEND.md` |
| App shell | `modules/APP.md` |

---

## 3. Bootstrap — Chạy một lần khi setup project

```bash
# Cài context-gen
pip install context-gen

# Sinh context lần đầu từ toàn bộ source
context-gen build . --quiet

# Kiểm tra output
ls .context/
# Phải thấy: README.md, GLOBAL.md, PROJECT.md, MILESTONES.md, TENSIONS_*.md
#            modules/, generated/, decisions/, planning/

# Khởi tạo tension V3 files nếu chưa có
touch .context/TENSIONS_OPEN.md .context/TENSIONS_ACTIVE.md .context/TENSIONS_HISTORY.md .context/MILESTONES.md
```

**Sau bước này**, điền/duy trì `.context/modules/*.md` — đặc biệt `VAULT.md` và `PROVIDERS.md` trước khi implement M1+.

---

## 4. Quy trình bắt buộc mỗi task

```
START
  │
  ▼
Đọc .context/GLOBAL.md → PROJECT.md → MILESTONES.md → TENSIONS_*
Đọc docs/vipavault-spec.md
Xác định module → đọc .context/modules/<MODULE>.md
  │
  ▼
context-gen load <source_path> . --include-manual  (AST từ generated/)
  │
  ├── modules/[manual] còn placeholder? → DỪNG. Hỏi human điền trước.
  │
  ▼
Detect tension với constraint trong spec / [manual]?
  │
  ├── Có → ghi vào .context/TENSIONS_OPEN.md (xem Section 9)
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

Tension V3 dùng 3 file:
- `.context/TENSIONS_OPEN.md` — chỉ `Status: OPEN`, luôn đọc full trước task.
- `.context/TENSIONS_ACTIVE.md` — chỉ `Status: RESOLVED_ACTIVE` của milestone hiện tại, đọc theo tag filter.
- `.context/TENSIONS_HISTORY.md` — chỉ `Status: ARCHIVED`, không đọc mặc định trừ khi human yêu cầu audit.

`.context/MILESTONES.md` chứa `Current:` để xác định milestone hiện tại.

### Loại 1 — Tension chưa resolve (open)

Khi detect conflict giữa task và constraint trong spec/[manual]:

```markdown
## [timestamp] | [module] | OPEN
Status:     OPEN
Tension:    mô tả conflict
Context:    đang làm task gì
Proposal:   muốn làm gì
Constraint: rule nào conflict (trích dẫn từ spec hoặc [manual])
Severity:   low | high
Tags:       tag1, tag2
Milestone:  current milestone từ .context/MILESTONES.md
Decision:   [human fill in]
```

**Routing:**
- `low` → tiếp tục conservative, human review sau
- `high` → tạo `HANDSHAKE_<timestamp>.md`, dừng task, chờ `.resolved`

**Ví dụ tension hợp lệ:**
- Task yêu cầu auto-sync → conflict với "KHÔNG auto-sync" → HIGH
- Task thêm provider mới không define auth_scheme → HIGH
- Task log credential để debug → HIGH (security)

### Loại 2 — Decision Log active

Ghi lại những quyết định đã được cân nhắc và chốt trong `.context/TENSIONS_ACTIVE.md`. Mục đích: agent mới đọc vào biết đây là **quyết định có chủ ý**, không phải lỗ hổng cần fix.

```markdown
## [timestamp] | [module]
Status:      RESOLVED_ACTIVE
Tension:     tên vấn đề đã được cân nhắc
Options:     A → ưu/nhược | B → ưu/nhược
Decision:    lựa chọn cuối cùng
Rationale:   lý do chọn, bằng chứng hoặc risk cụ thể
Constraint:  KHÔNG reopen trừ khi [điều kiện cụ thể]
Severity:    low | high
Tags:        tag1, tag2
Milestone:   V1
Phase:       V1 | Phase 2 | all
```

**Rule:** Entry RESOLVED_ACTIVE không bao giờ bị xóa. Khi qua milestone mới, chỉ move sang `.context/TENSIONS_HISTORY.md` và đổi `Status: ARCHIVED` nếu human approve.

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
Phase 2 (OAuth): constraint này sẽ thay đổi — xem `.context/TENSIONS_ACTIVE.md` entry oauth_credentials.
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
    MILESTONES.md
    TENSIONS_OPEN.md
    TENSIONS_ACTIVE.md
    TENSIONS_HISTORY.md
```

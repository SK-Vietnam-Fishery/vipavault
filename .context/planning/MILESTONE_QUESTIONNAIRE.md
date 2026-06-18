# VipaVault — Milestone Planning Questionnaire

> **Mục đích:** Dev/human trả lời **trước khi implement** từng milestone.  
> **Không phải** checklist làm việc — khai thác kiến trúc, design, trade-off, và gap trong tài liệu.  
> **Tổng:** 132 câu hỏi, theo thứ tự milestone `0.1.0` → `0.11.0`.  
> **Trạng thái:** Chờ trả lời từng phần — điền vào block `### Trả lời` dưới mỗi section.

---

## Cách dùng

1. Trả lời theo thứ tự section — câu sau có thể phụ thuộc câu trước.
2. Ghi rõ: **Quyết định** / **Chưa rõ — cần spike** / **Giữ nguyên spec**.
3. Nếu đổi spec → tạo entry `.context/TENSIONS_OPEN.md` (severity `high` nếu ảnh hưởng security).
4. Tag: `[ARCH]` kiến trúc · `[SEC]` bảo mật · `[UX]` giao diện · `[DATA]` schema · `[OPS]` vận hành · `[TEST]` kiểm thử · `[OPEN]` chưa có trong tài liệu

### Tài liệu nền

| Tài liệu | Nội dung |
|----------|----------|
| `docs/vipavault-spec.md` | Spec chính — schema, security, features, roadmap |
| `.context/MILESTONES.md` | Milestone hiện tại + exit criteria |
| `.context/planning/MILESTONES_REFERENCE.md` | Roadmap đầy đủ 12 milestone |
| `.context/PROJECT.md` | Quyết định project-wide + open decisions |
| `.context/TENSIONS_ACTIVE.md` | Quyết định đã chốt (sync, storage, role, provider…) |
| `.context/decisions/DECISION_VAULT_STORAGE.md` | SQLCipher vs KeePass — phân tầng 3–4 lớp |
| `.context/modules/*.md` | Invariants từng module |
| `.local/ENVIRONMENT.md` | Toolchain + lệnh chạy local (machine-specific) |

### Gap ưu tiên (trả lời trước 0.2.0)

1. `email_accounts` thiếu `sync_status` — spec §6 nói `pending_sync` nhưng SQL §4 chưa có column
2. `app_settings` trong vault vs `app_settings.json` ngoài vault — phân chia key chưa rõ
3. `credential_type` mở rộng — open decision trong `PROJECT.md`
4. Migration boundary 0.1.1 vs 0.2.0 — vault create có chạy migration không
5. README mô tả sai stack (Next.js/Prisma)

---

## Thống kê

| Section | Số câu | Trạng thái |
|---------|--------|------------|
| P — Program-wide | 12 | ⬜ Chưa trả lời |
| 0.1.0 — Foundation | 10 | ⬜ Chưa trả lời |
| 0.1.1 — Vault Core | 14 | ⬜ Chưa trả lời |
| 0.2.0 — Data Model | 14 | ⬜ Chưa trả lời |
| 0.3.0 — App Shell | 12 | ⬜ Chưa trả lời |
| 0.4.0 — Dashboard | 10 | ⬜ Chưa trả lời |
| 0.5.0 — Credentials | 12 | ⬜ Chưa trả lời |
| 0.6.0 — Email Local | 10 | ⬜ Chưa trả lời |
| 0.7.0 — Provider Routing | 10 | ⬜ Chưa trả lời |
| 0.8.0 — Manual Sync | 10 | ⬜ Chưa trả lời |
| 0.9.0 — Email Apply | 10 | ⬜ Chưa trả lời |
| 0.10.0 — Confuse | 8 | ⬜ Chưa trả lời |
| 0.11.0 — MVP Release | 12 | ⬜ Chưa trả lời |
| X — Cross-milestone | 10 | ⬜ Chưa trả lời |
| M — Meta process | 6 | ⬜ Chưa trả lời |
| **Tổng** | **132** | |

---

# P — Program-wide

**Đọc trước:** `docs/vipavault-spec.md` §1–2, `.context/PROJECT.md`, `.context/TENSIONS_ACTIVE.md`

### P-001 [ARCH]
V1 target user thực tế là ai — chỉ IT nội bộ, hay IT + CEO viewer ngay từ ngày đầu? Điều này ảnh hưởng thứ tự ưu tiên 0.3.0 vs 0.4.0.

### P-002 [ARCH]
Một công ty quản lý bao nhiêu profile `.hvault` trong V1 — 1, vài công ty con, hay unlimited? Spec §2 có `profiles.json` multi-profile nhưng chưa giới hạn.

### P-003 [SEC]
Ai được giữ master password — một người IT, hay nhiều người? Spec §3 nói không recovery — quy trình offboarding IT khi nghỉ việc là gì?

### P-004 [OPS]
File `.hvault` copy sang máy viewer (CEO): ai copy, qua kênh nào (USB, SharePoint, email)? Có policy xóa bản copy sau khi dùng không?

### P-005 [ARCH]
`app_settings.json` nằm ngoài vault — path chính xác trên Windows/WSL/macOS? Có sync qua cloud (OneDrive) không — risk gì nếu `machine_role` bị sửa tay?

### P-006 [ARCH]
Tauri app có cần hỗ trợ macOS trong V1 không, hay chỉ Windows + Linux (WSL dev)? Ảnh hưởng packaging 0.11.0.

### P-007 [DATA]
`monthly_cost` nhập tay (spec §4) — đơn vị VND hay USD? Có cần multi-currency dashboard không?

### P-008 [UX]
Ngôn ngữ UI V1: tiếng Việt only, song ngữ, hay English? Ảnh hưởng copy confuse/notification.

### P-009 [TEST]
Môi trường test provider thật: có staging cPanel/DirectAdmin sandbox không, hay mock-only đến 0.9.0?

### P-010 [OPEN]
`.context/PROJECT.md` ghi open decision: `credential_type` mở rộng (mariadb, postgres…) — V1 có cần hay defer hết sang sau 0.5.0?

### P-011 [ARCH]
README hiện tại mô tả Next.js/Prisma — intentional hay cần rewrite? Ai là audience của README V1?

### P-012 [OPS]
Chiến lược backup `.hvault`: spec §3 đề cập `backups/` — ai trigger, tần suất, retention bao lâu?

### Trả lời (P)

```
(điền tại đây)
```

---

# 0.1.0 — Project Foundation

**Đọc trước:** `.context/MILESTONES.md` §0.1.0, `.context/modules/APP.md`, `.local/ENVIRONMENT.md`, `AGENTS.md` §2–4

### Q-0.1.0-001 [OPS]
Dev chính chạy trên WSL Debian (theo `.local/ENVIRONMENT.md`) hay native Windows? Quyết định này cố định CI/local workflow.

### Q-0.1.0-002 [ARCH]
Node version target: nvm 24 hiện tại có chấp nhận là chính thức không? Tauri 2 + Vite 6 có pin version trong `package.json` engines không?

### Q-0.1.0-003 [ARCH]
Vite dev port `1420` (`.local/ENVIRONMENT.md`) — có conflict với service khác trên máy dev không?

### Q-0.1.0-004 [TEST]
`cargo test` hiện fail vì thiếu `dist/` — chấp nhận workflow `npm build` trước `cargo test`, hay tách `lib.rs` để test không cần `generate_context!()`?

### Q-0.1.0-005 [ARCH]
Có cần CI (GitHub Actions) trong 0.1.0 exit criteria không, hay chỉ local pass đủ?

### Q-0.1.0-006 [ARCH]
`context-gen` — ai chịu trách nhiệm commit `.context/generated/` sau mỗi PR? Quy trình review `[manual]` khi có staleness warning (`.context/TENSIONS_OPEN.md`)?

### Q-0.1.0-007 [ARCH]
Cấu trúc monorepo hiện tại (root npm + `src-tauri/`) — có đủ cho V1 hay cần workspace split (shared types crate)?

### Q-0.1.0-008 [UX]
Boot shell tối thiểu khi `tauri dev` — chỉ hiện tên app, hay cần health panel (version, IPC smoke)?

### Q-0.1.0-009 [SEC]
`.gitignore` ignore `Cargo.lock` — có chủ ý cho app binary không? Team có đồng ý lockfile reproducible build không?

### Q-0.1.0-010 [OPS]
`tauri.conf.json` `icon: []` — icon brand có sẵn chưa, hay placeholder đến 0.11.0?

### Trả lời (0.1.0)

```
(điền tại đây)
```

---

# 0.1.1 — Vault Core

**Đọc trước:** `docs/vipavault-spec.md` §3, `.context/modules/VAULT.md`, `.context/decisions/DECISION_VAULT_STORAGE.md` §6, `.context/TENSIONS_ACTIVE.md` entry `storage`

### Q-0.1.1-001 [ARCH]
Rust crate SQLCipher: `rusqlite` + bundled SQLCipher, hay crate khác (`sqlcipher-sys`)? Tiêu chí chọn — license, maintenance, Tauri compatibility?

### Q-0.1.1-002 [SEC]
Argon2id params spec §3 (memory 64MB, iterations 3) — có phù hợp máy CEO cũ (viewer) khi unlock không? Có cần profile "low memory" không?

### Q-0.1.1-003 [SEC]
Salt 32 bytes — lưu ở đâu trong file `.hvault` header? Có document format file (.hvault magic bytes + version) chưa?

### Q-0.1.1-004 [SEC]
Master password policy: min length, complexity, passphrase vs password — có enforce ở create vault không?

### Q-0.1.1-005 [SEC]
Wrong password UX: message generic "Invalid password" — có cần rate-limit brute force ở local level không?

### Q-0.1.1-006 [ARCH]
`profiles.json` schema: fields nào bắt buộc (id, display_name, hvault_path, created_at)? Có avatar/logo công ty không?

### Q-0.1.1-007 [ARCH]
Multi-profile V1: switch profile = lock vault A → unlock vault B, hay giữ nhiều connection? Memory implication?

### Q-0.1.1-008 [SEC]
`zeroize()` scope: chỉ derived key, hay cả master password buffer trong RAM sau derive?

### Q-0.1.1-009 [SEC]
Auto-lock idle timeout — implement ở 0.1.1 hay defer 0.11.0? Nếu defer, 0.1.1 lock chỉ manual?

### Q-0.1.1-010 [TEST]
Test wrong password "không leak timing" — ngưỡng chấp nhận timing delta bao nhiêu ms?

### Q-0.1.1-011 [UX]
Unlock flow UI: modal mỗi lần mở app, hay system keychain substitute (macOS/Windows)? Tension với "không lưu master password".

### Q-0.1.1-012 [OPS]
Tạo vault mới: default path `~/.vipavault/{profile_id}.hvault` — user có chọn custom path không?

### Q-0.1.1-013 [ARCH]
Vault file extension `.hvault` only — có magic header để nhận diện file không phải SQLite thường?

### Q-0.1.1-014 [OPEN]
Migration chạy lúc nào — first open sau create, hay embed trong create? Liên quan boundary 0.1.1 vs 0.2.0.

### Trả lời (0.1.1)

```
(điền tại đây)
```

---

# 0.2.0 — Data Model & Migrations

**Đọc trước:** `docs/vipavault-spec.md` §4–5, `.context/modules/VAULT.md`, `.context/TENSIONS_ACTIVE.md` entries `m365_google_schema`, `vault_tier_model`

### Q-0.2.0-001 [DATA]
ID strategy: `TEXT PRIMARY KEY` — UUID v4, ULID, hay human-readable slug (`svc_namecheap_01`)?

### Q-0.2.0-002 [DATA]
`services.provider` vs `services.provider_type` — khác nhau thế nào? "Namecheap" là provider, `cpanel` là provider_type — UI hiển thị cái nào?

### Q-0.2.0-003 [DATA]
`nameservers` JSON array — schema validate ở app layer hay SQLite CHECK constraint?

### Q-0.2.0-004 [DATA]
`email_accounts` thiếu `status` (`pending_sync`/`synced`) trong spec §4 SQL — thêm column migration hay dùng bảng khác?

### Q-0.2.0-005 [DATA]
`app_settings` table trong vault vs `app_settings.json` ngoài vault — key nào ở đâu? Trùng lặp `machine_role` xử lý thế nào?

### Q-0.2.0-006 [DATA]
`credential_type` enum V1: chỉ `panel|ftp|ssh|api_key` hay thêm `mariadb|postgres|redis` ngay? Ref: `.context/PROJECT.md` open decision.

### Q-0.2.0-007 [ARCH]
Repository pattern: raw SQL trong vault module, hay layer `storage/` riêng?

### Q-0.2.0-008 [DATA]
Soft delete (`deleted_at`) cho services/credentials hay hard delete V1? Audit log có đủ không?

### Q-0.2.0-009 [DATA]
`activity_log.old_value_hint` — luôn 3 ký tự cuối hay configurable?

### Q-0.2.0-010 [ARCH]
Migration versioning: single `schema_version` table, hay numbered files `001_init.sql`? Rollback strategy?

### Q-0.2.0-011 [DATA]
Phase 2 `oauth_credentials` — V1 migration có tạo bảng rỗng sẵn (forward-compatible) hay không tạo gì?

### Q-0.2.0-012 [TEST]
Seed data dev: có fixture SQL sample (2 hosting, 5 email) cho dashboard test không?

### Q-0.2.0-013 [DATA]
`ssl_certs` link `domain_id` nullable — cert wildcard `*.example.com` map thế nào?

### Q-0.2.0-014 [ARCH]
WAL mode spec §3 — enable ở migration 001 hay default SQLCipher? Backup strategy tương thích WAL?

### Trả lời (0.2.0)

```
(điền tại đây)
```

---

# 0.3.0 — App Shell & Roles

**Đọc trước:** `docs/vipavault-spec.md` §1–2, `.context/TENSIONS_ACTIVE.md` entry `role_model`, `.context/modules/FRONTEND.md`, `.context/modules/COMMANDS.md`, `.context/modules/APP.md`

### Q-0.3.0-001 [UX]
Layout shell: sidebar navigation, top nav, hay single-page tabs? Có design mockup/wireframe không?

### Q-0.3.0-002 [SEC]
`machine_role` đọc từ `app_settings.json` — ai được phép sửa file này? Có checksum/sign không?

### Q-0.3.0-003 [SEC]
Viewer bypass: user sửa `app_settings.json` thành admin — backend guard đủ chưa? Có cần admin PIN local không?

### Q-0.3.0-004 [ARCH]
`sync_enabled: false` — tách biệt `machine_role` hay luôn đi cùng viewer? Matrix quyền 2×2?

### Q-0.3.0-005 [UX]
Badge "Chế độ xem" — vị trí, màu, có dismiss được không?

### Q-0.3.0-006 [ARCH]
Role guard ở Rust command layer — macro `require_admin!` hay middleware pattern? Danh sách command read vs write ban đầu?

### Q-0.3.0-007 [UX]
Profile switcher UI — dropdown header, hay màn hình riêng? Switch có cần re-enter master password không?

### Q-0.3.0-008 [ARCH]
React state management V1: Context API, Zustand, TanStack Query — chọn gì cho vault lock state + role?

### Q-0.3.0-009 [UX]
Locked vault state: full-screen unlock, hay shell mờ + modal?

### Q-0.3.0-010 [TEST]
Test viewer mode: assert button `disabled` hay button không render? Cái nào là source of truth?

### Q-0.3.0-011 [OPEN]
`app_settings.json` schema formal — có JSON Schema file trong repo không?

### Q-0.3.0-012 [UX]
Keyboard shortcut global (Ctrl+L lock) — V1 có cần không?

### Trả lời (0.3.0)

```
(điền tại đây)
```

---

# 0.4.0 — Dashboard Slice

**Đọc trước:** `docs/vipavault-spec.md` §6 Dashboard, `.context/modules/FRONTEND.md` alert thresholds

### Q-0.4.0-001 [UX]
Dashboard admin vs viewer — cùng layout khác action, hay layout khác hẳn?

### Q-0.4.0-002 [DATA]
"Tổng chi phí tháng" — sum `services.monthly_cost` where `status=active` — có include domain riêng không?

### Q-0.4.0-003 [UX]
Alert đỏ/vàng/xanh — áp dụng cho `expires_at` của service, domain, SSL, hay tất cả?

### Q-0.4.0-004 [DATA]
`expires_at` null — hiển thị xanh, xám "unknown", hay ẩn?

### Q-0.4.0-005 [UX]
Provider breakdown chart — pie, bar, table? CEO cần drill-down không?

### Q-0.4.0-006 [DATA]
Disk usage / email count trên dashboard — data từ `cpanel_sync_cache` (chưa sync = 0) — hiển thị "N/A" hay "Chưa sync"?

### Q-0.4.0-007 [ARCH]
Dashboard query: một IPC command aggregate, hay nhiều command parallel?

### Q-0.4.0-008 [UX]
Timezone `expires_at` — UTC storage, hiển thị Asia/Ho_Chi_Minh?

### Q-0.4.0-009 [TEST]
Alert threshold boundary: exactly 7 days và 30 days — đỏ hay vàng?

### Q-0.4.0-010 [UX]
Empty state vault mới tạo — onboarding CTA "Thêm hosting đầu tiên" hay blank dashboard?

### Trả lời (0.4.0)

```
(điền tại đây)
```

---

# 0.5.0 — Credential Management

**Đọc trước:** `docs/vipavault-spec.md` §4 `service_credentials`, `.context/decisions/DECISION_VAULT_STORAGE.md` §6.2, `.context/TENSIONS_ACTIVE.md` `vault_tier_model`, `autofill_mechanism`

### Q-0.5.0-001 [UX]
Nested tree UI (company → service → credential) — tree view, master-detail, hay flat list + filter?

### Q-0.5.0-002 [SEC]
Reveal password: click "Hiện" — timeout auto-hide sau bao lâu?

### Q-0.5.0-003 [SEC]
Watermark spec §3 — tên IT overlay: text gì, opacity, chấp nhận risk screenshot OS-level?

### Q-0.5.0-004 [SEC]
Copy clipboard password — cho phép không? Clear clipboard sau N giây?

### Q-0.5.0-005 [UX]
Mở `portal_url` — external browser hay in-app webview? Tension KeePass autofill đã drop.

### Q-0.5.0-006 [DATA]
Một service nhiều credentials — UI default sort order?

### Q-0.5.0-007 [SEC]
`activity_log` action `viewed_password` — log mỗi lần reveal hay chỉ lần đầu/session?

### Q-0.5.0-008 [DATA]
API token cPanel lưu trong `service_credentials.password` — label convention bắt buộc?

### Q-0.5.0-009 [UX]
Form add credential — token-only panel credential cho phép username placeholder?

### Q-0.5.0-010 [OPEN]
Sub-credential tier 4 (attachment/file) — V1 có scope không? Ref: `DECISION_VAULT_STORAGE.md` §1.

### Q-0.5.0-011 [TEST]
Test "viewer cannot reveal" — IPC trả 403 hay empty string?

### Q-0.5.0-012 [UX]
Import bulk credentials (CSV) — V1 có cần hay manual only?

### Trả lời (0.5.0)

```
(điền tại đây)
```

---

# 0.6.0 — Email Accounts Local

**Đọc trước:** `docs/vipavault-spec.md` §6 Email + Sync flow, `.context/modules/SYNC.md`

### Q-0.6.0-001 [SEC]
CSPRNG password generator — length default, charset (ambiguous 0/O excluded?), entropy minimum?

### Q-0.6.0-002 [DATA]
`pending_sync` status — column mới `email_accounts.sync_status` hay reuse field khác?

### Q-0.6.0-003 [UX]
Tạo email local: bắt buộc chọn `service_id` + domain suffix từ `domain_primary`?

### Q-0.6.0-004 [DATA]
`must_change_password` default 1 — có cho phép tạo email không bắt đổi pass khi apply lên cPanel?

### Q-0.6.0-005 [UX]
`display_name`, `department` — bắt buộc hay optional? Dùng cho notification sau này?

### Q-0.6.0-006 [DATA]
`quota_mb` default 500 — match cPanel default package hay configurable per service?

### Q-0.6.0-007 [ARCH]
Email list per service — pagination nếu >100 accounts?

### Q-0.6.0-008 [TEST]
Audit log `created` — có lưu `new_value_hint` cho password không?

### Q-0.6.0-009 [UX]
Xóa email local chưa sync — cho phép hard delete không?

### Q-0.6.0-010 [OPEN]
Trùng `email_address` trên provider đã tồn tại — detect lúc sync 0.8.0 hay validate lúc create local?

### Trả lời (0.6.0)

```
(điền tại đây)
```

---

# 0.7.0 — Provider Routing V1

**Đọc trước:** `docs/vipavault-spec.md` §5–6 cPanel UAPI, `.context/modules/PROVIDERS.md`, `.context/TENSIONS_ACTIVE.md` `provider_scope_v1`

### Q-0.7.0-001 [ARCH]
cPanel + DirectAdmin shared client — divergence đầu tiên cần branch ở đâu (base URL, auth header, endpoint path)?

### Q-0.7.0-002 [SEC]
HTTPS only — có cho phép self-signed cert internal hosting không?

### Q-0.7.0-003 [ARCH]
HTTP client: `reqwest` async trong Tauri command — timeout, retry policy?

### Q-0.7.0-004 [SEC]
API token trong `Authorization: cpanel user:token` — token rotation workflow khi expire?

### Q-0.7.0-005 [ARCH]
`provider_type` unknown — `warn!` + skip: log level, có surface UI notification không?

### Q-0.7.0-006 [TEST]
Mock provider: wiremock, httptest server, hay trait injection?

### Q-0.7.0-007 [OPEN]
DirectAdmin API docs version nào là reference chính thức — team có tài liệu nội bộ không?

### Q-0.7.0-008 [ARCH]
Base URL pattern: `https://{server}:2083` hardcode hay user nhập `portal_url`?

### Q-0.7.0-009 [SEC]
Credential pick: nhiều `service_credentials` — chọn `credential_type=panel` auto hay user chọn?

### Q-0.7.0-010 [OPS]
Egress firewall: máy IT có outbound 2083/2087 — proxy corporate có block không?

### Trả lời (0.7.0)

```
(điền tại đây)
```

---

# 0.8.0 — Manual Sync & Rate Limit

**Đọc trước:** `docs/vipavault-spec.md` §6 Sync, `.context/modules/SYNC.md`, `.context/TENSIONS_ACTIVE.md` entry `sync`

### Q-0.8.0-001 [ARCH]
Rate limit storage: in-memory HashMap, SQLite table, hay file — survive app restart?

### Q-0.8.0-002 [UX]
User bấm Refresh khi còn cooldown — hiện countdown timer, hay modal "Thử lại sau X phút"?

### Q-0.8.0-003 [DATA]
`cpanel_sync_cache.sync_status` values: `never|ok|error|rate_limited` — enum đủ chưa?

### Q-0.8.0-004 [ARCH]
Sync pull những gì V1: email list only, hay disk quota + email count (spec §4 cache columns)?

### Q-0.8.0-005 [DATA]
Sync conflict: email có trên cPanel nhưng không có local — auto-import local hay prompt user?

### Q-0.8.0-006 [SEC]
`sync_enabled=false` guard — block ở command entry hay provider client wrapper? Defense in depth?

### Q-0.8.0-007 [OPS]
cPanel trả 429/403 — map error message tiếng Việt cho user?

### Q-0.8.0-008 [TEST]
Rate limit per-service isolation — test với 2 service IDs mock.

### Q-0.8.0-009 [UX]
"Cập nhật lúc 14:32" — relative time ("5 phút trước") hay absolute?

### Q-0.8.0-010 [OPEN]
Sync đồng thời 2 service parallel — cho phép hay queue tuần tự để giảm IP risk?

### Trả lời (0.8.0)

```
(điền tại đây)
```

---

# 0.9.0 — Provider Email Apply

**Đọc trước:** `docs/vipavault-spec.md` §6 Email + cPanel UAPI Reference, `.context/modules/SYNC.md`, `.context/modules/PROVIDERS.md`

### Q-0.9.0-001 [UX]
Flow pending → review → Apply: một màn hình queue, hay inline per email row?

### Q-0.9.0-002 [ARCH]
Apply atomicity: add_pop fail giữa batch — rollback local status thế nào?

### Q-0.9.0-003 [SEC]
Apply gửi real password lên cPanel HTTPS — có log request body không (must not)?

### Q-0.9.0-004 [UX]
Confirm dialog trước Apply — hiển thị email address, không hiện password — đủ chưa?

### Q-0.9.0-005 [DATA]
Sau Apply success: `sync_status=synced` + update `last_reset_at`?

### Q-0.9.0-006 [ARCH]
Delete email: soft delete local + delete_pop provider — thứ tự nào trước?

### Q-0.9.0-007 [OPS]
cPanel `add_pop` fail vì quota full — UX message + link docs?

### Q-0.9.0-008 [TEST]
Mock tests cover add/reset/delete — có cần integration test against real cPanel staging?

### Q-0.9.0-009 [UX]
Reset password đã synced — tạo pending mới hay apply immediate?

### Q-0.9.0-010 [SEC]
Viewer cố gọi Apply IPC — error code convention across app?

### Trả lời (0.9.0)

```
(điền tại đây)
```

---

# 0.10.0 — Confuse & Notification

**Đọc trước:** `docs/vipavault-spec.md` §3 Rủi ro confuse + §6 Confuse Engine, `.context/modules/CONFUSE.md`

### Q-0.10.0-001 [SEC]
Rule confuse prefix/suffix — lưu `app_settings` vault table hay `app_settings.json` ngoài vault?

### Q-0.10.0-002 [SEC]
Spec §3: đổi rule khi NV nghỉ, max 6 tháng — ai approve đổi rule, audit trail?

### Q-0.10.0-003 [UX]
Template Zalo vs Email khác nhau (spec §6) — composer một form hai output?

### Q-0.10.0-004 [UX]
Decode hint "bỏ 5 đầu 7 cuối" — số N/M fixed hay theo rule dynamic?

### Q-0.10.0-005 [SEC]
Confuse string không persist — clipboard copy confuse có clear timer không?

### Q-0.10.0-006 [ARCH]
Integration gửi Zalo: manual copy-paste only V1, hay API Zalo OA?

### Q-0.10.0-007 [DATA]
`activity_log.confuse_used` — lưu full rule hay chỉ prefix/suffix values?

### Q-0.10.0-008 [TEST]
Test confuse không bao giờ ghi pass thật vào log/clipboard history file?

### Trả lời (0.10.0)

```
(điền tại đây)
```

---

# 0.11.0 — MVP Hardening & Release

**Đọc trước:** `docs/vipavault-spec.md` §3 Key lifecycle + Rủi ro, `.context/planning/MILESTONES_REFERENCE.md` §0.11.0

### Q-0.11.0-001 [SEC]
Auto-lock idle timeout — default bao nhiêu phút? User configurable?

### Q-0.11.0-002 [OPS]
Backup tự động `~/.vipavault/backups/` — trigger: daily on lock, on close, manual only?

### Q-0.11.0-003 [OPS]
WAL corruption recovery — user-facing wizard restore từ backup?

### Q-0.11.0-004 [OPS]
Packaging target V1: Windows .msi only, hay + Linux .deb + macOS .dmg?

### Q-0.11.0-005 [SEC]
Code signing Windows — có certificate không? Không thì SmartScreen warning chấp nhận?

### Q-0.11.0-006 [TEST]
"Final test sweep" — minimum coverage % hay checklist manual QA?

### Q-0.11.0-007 [OPS]
Auto-update Tauri V1 — có hay manual reinstall?

### Q-0.11.0-008 [UX]
Error handling pass — global toast, error boundary, crash report?

### Q-0.11.0-009 [SEC]
Penetration test nội bộ trước release — scope nào (IPC, file encryption, viewer bypass)?

### Q-0.11.0-010 [OPS]
Version ship: app version `1.0.0` khi milestone ID là `0.11.0` — mapping chính thức?

### Q-0.11.0-011 [OPS]
Release notes audience — IT only hay cả CEO viewer?

### Q-0.11.0-012 [ARCH]
Telemetry/crash reporting — có gửi data ra ngoài không (privacy)?

### Trả lời (0.11.0)

```
(điền tại đây)
```

---

# X — Cross-milestone & Phase 2+

**Đọc trước:** `docs/vipavault-spec.md` §5 Phase 2–3, `.context/TENSIONS_ACTIVE.md` `m365_google_schema`

### X-001 [ARCH]
Shared TypeScript types cho IPC payloads — generate từ Rust (`tauri-specta`, `typeshare`) hay hand-written?

### X-002 [SEC]
Threat model document — có cần formal STRIDE trước 0.1.1 không?

### X-003 [DATA]
Phase 2 `oauth_credentials` — token refresh background task conflict với manual sync invariant?

### X-004 [ARCH]
Phase 3 `subscription_licenses` — computed `monthly_cost` migration path từ nhập tay?

### X-005 [OPEN]
KDBX export/import (`.context/PROJECT.md`) — user story cụ thể nào trigger Phase 2+?

### X-006 [UX]
Audit log viewer P1 spec §7 — defer post-0.11.0 hay squeeze vào MVP?

### X-007 [OPS]
Multi-IT concurrent edit cùng `.hvault` file — SQLCipher single-writer — workflow chia sẻ file?

### X-008 [SEC]
Compliance: có yêu cầu lưu audit log immutable/export không?

### X-009 [ARCH]
Plugin/extension architecture — V1 monolith đủ hay cần hook points sớm?

### X-010 [TEST]
E2E test framework: Playwright Tauri driver, hay chỉ unit+integration?

### Trả lời (X)

```
(điền tại đây)
```

---

# M — Meta process

**Đọc trước:** `AGENTS.md`, `.context/TENSIONS_OPEN.md` staleness entries

### M-001 [OPS]
Ai approve milestone activation (`Active Execution:` trong `MILESTONES.md`)? Chỉ human hay lead dev?

### M-002 [OPS]
Câu trả lời bộ questionnaire này lưu ở đâu — `.context/decisions/`, wiki, hay issue tracker?

### M-003 [OPS]
OPEN tension staleness `src-tauri_src` — ai review `[manual]` trước 0.1.1?

### M-004 [TEST]
Definition of Done mỗi milestone: bắt buộc `npm test` + `cargo test` + manual QA checklist?

### M-005 [ARCH]
PR size limit / review policy — có không cho dự án solo+agent?

### M-006 [OPS]
`.local/ENVIRONMENT.md` — ai maintain khi đổi máy dev mới?

### Trả lời (M)

```
(điền tại đây)
```

---

## Changelog

| Ngày | Ghi chú |
|------|---------|
| 2026-06-17 | Tạo file — 132 câu hỏi, chờ trả lời từng section |
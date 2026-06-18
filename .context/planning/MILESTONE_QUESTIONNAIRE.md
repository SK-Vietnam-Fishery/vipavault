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
5. Answer sẽ viết tắt là "A:"

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

1. `email_accounts` thiếu `sync_status`  
   **A:** Thêm cột `sync_status` ở 0.2.0 (`local_only` | `pending_sync` | `synced` | `sync_error`).

2. `app_settings` trong vault vs `app_settings.json` ngoài vault  
   **A:** Tách rõ — xem bảng **Storage split** trong section Auth bên dưới.

3. `credential_type` mở rộng  
   **A:** *(chưa chốt — 4 loại spec hay thêm mariadb/postgres/redis)*

4. Migration boundary 0.1.1 vs 0.2.0  
   **A:** *(đề xuất: 0.1.1 tạo vault trống encrypted, 0.2.0 chạy migration lần đầu mở — chưa chốt)*

5. README mô tả sai stack  
   **A:** README cũ do LLM MS — rewrite cho đúng Tauri/Rust/React/SQLCipher khi làm 0.1.0 cleanup.

---

## Quyết định đã chốt — Auth & Sharing (SIMPLE V1 + FULL extension)

> Ghi nhận từ planning trao đổi (2026-06). **Human approved.**  
> **SIMPLE V1** implement trước; **FULL** là mở rộng sau V1 MVP. Cập nhật `docs/vipavault-spec.md` khi implement.

### SIMPLE V1 — Login & roles (đã chốt)

**Một yếu tố knowledge — không user account + password_hash:**

1. **Master password** — mở SQLCipher `.hvault`.

- **Không** email allowlist / `workspace_members` trong V1.
- **Không** Settings → Share Workspace trong V1.
- **Không** bảng `users` + `password_hash`.
- **Roles:** `machine_role` + `sync_enabled` trong `app_settings.json` per-machine (IT = admin, CEO = viewer).
- **Recovery:** không forgot-password trong app; backup `.hvault` + escrow master pass **ngoài app** — không reversible.
- **PIN quick unlock:** defer sau V1 (0.11.0+ nếu làm).

### Flow đăng nhập V1 (backend)

```
master_password
  → SQLCipher open .hvault
  → session unlocked, apply machine_role từ app_settings.json
```

### Màn login V1 — UI vs backend (đã chốt / cập nhật 2026-06-18)

**Backend** (yếu tố knowledge): chỉ **master password** (+ chọn profile nếu ≥2 vault).

**UI** — thứ tự **luôn** giống nhau:

1. **Tên vault** — `profiles.json` → `display_name`
2. **Tên người dùng** — read-only; `app_settings.json` → `operator_display_name`, fallback OS username
3. **Mật khẩu** — input master password

| Số vault | Row ① Tên vault | Row ② Người dùng | Row ③ Backend input |
|----------|-----------------|------------------|---------------------|
| **1 vault** | Label read-only | Label read-only | Chỉ mật khẩu |
| **≥ 2 vault** | Dropdown chọn vault | Label read-only | Vault đã chọn + mật khẩu |

```
1 vault (UI — không chỉ một ô pass):
┌──────────────────────────────────┐
│ Vault:      Công ty A            │
│ Người dùng: Tuấn (IT)            │
│ Mật khẩu:   [ •••••••••••••••• ] │
│         [ Mở khóa ]              │
└──────────────────────────────────┘

≥ 2 vault:
┌──────────────────────────────────┐
│ Vault:      [ Công ty A      ▼ ] │
│ Người dùng: Tuấn (IT)            │
│ Mật khẩu:   [ •••••••••••••••• ] │
│         [ Mở khóa ]              │
└──────────────────────────────────┘
```

**Milestone V1:** unlock **0.1.1** (vault core) + **0.3.0** (UI shell).

### Storage split (gap #2 — đã chốt)

| Lưu ở đâu | Keys / tables | Mã hóa? | Đi theo copy .hvault? |
|-----------|---------------|---------|----------------------|
| **`app_settings.json`** (per-machine) | `machine_role`, `sync_enabled` | Không | **Không** — mỗi máy riêng |
| **`profiles.json`** | `id`, `display_name`, `hvault_path` | Không | Tuỳ cách copy/setup |
| **Trong `.hvault`** — `app_settings` table | `confuse_prefix`, `confuse_suffix`, … | SQLCipher | **Có** |

**Không trùng key** giữa JSON ngoài vault và table trong vault.

### Chia sẻ V1 — copy full vault

- Copy **toàn bộ** `.hvault` sang máy viewer + set `machine_role: viewer` trên máy đích.
- Viewer thấy **hết** nội dung trong file (read-only) — **không** partial copy trong V1.
- Khuyến nghị ops: 1 vault = 1 phạm vi chia sẻ nếu cần tách dữ liệu sớm (multi-profile).

### Audit V1

- `activity_log.actor_note` — OS username hoặc free-text (vd. `"IT - Tuấn"`). **Không** bắt buộc email gate.

### Chưa chốt (V1)

- [ ] `credential_type` mở rộng (gap #3)
- [ ] Migration boundary 0.1.1 vs 0.2.0 (gap #4)
- [ ] Offboarding wizard: rotate master pass — milestone nào (0.3.0?)

---

## FULL extension — Share Package (subset export) `[Phase 1.5+]`

> **Defer** sau V1 MVP. Thay thế ý tưởng email allowlist / Share Workspace cho use case “chỉ chia một phần vault”.

### Mô hình (đã chốt hướng)

**Share ≠ copy cả vault.** Admin **chọn** service / credential / email đưa vào gói → app tạo **file `.hvault` mới** (subset + FK liên quan).

Ví dụ `whyscool.com`:

```
☑ Service Web Hosting + credential cPanel Admin
☑ Email accounts (@whyscool.com)
☐ Credential FTP backup
☐ Service Domain / Registrar + credential Namecheap
```

Người nhận **không có** dữ liệu không được tick — không phải “có nhưng ẩn”.

### Flow UX (đã chốt hướng) — người nhận tự đặt pass lần đầu

**Có.** Admin **không** đặt master password lâu dài cho gói share. Thay vào đó:

```
[Admin] Settings → Tạo gói chia sẻ
  → checklist tree (service → credential → email)
  → app sinh mã kích hoạt một lần (one-time activation code)
  → xuất whyscool_viewer.hvault + hiển thị mã cho admin (copy / QR — TBD)
  → admin gửi FILE và MÃ qua hai kênh (hoặc cùng kênh nếu chấp nhận risk)

[Người nhận] Mở app lần đầu với file gói
  → detect vault chưa được “claim” (first_setup_pending)
  → nhập mã kích hoạt từ IT
  → màn “Đặt mật khẩu của bạn” (2 lần, policy min length)
  → SQLCipher PRAGMA rekey → master password = của người nhận
  → mã kích hoạt vô hiệu; lần sau chỉ dùng pass người nhận đã đặt
```

**Lợi ích:** IT không phải nhớ N mật khẩu cho N gói; CEO tự sở hữu pass trên máy mình; tooltip “admin giữ pass” không còn áp dụng cho admin — chuyển sang **mã kích hoạt một lần**.

**Kỹ thuật (draft):** Export mã hóa file bằng key từ activation code (hoặc random setup key + hash lưu trong vault metadata `share_packages.setup_state`). First open: verify code → `rekey` sang pass người nhận → flag `claimed_at` / xóa setup state.

- Gói share là **snapshot** — không auto-sync ngược vault admin (trừ khi thiết kế riêng sau).
- `activity_log` / `cpanel_sync_cache` — mặc định **không** copy sang gói viewer (hoặc chỉ metadata dashboard).

### Mật khẩu gói share — tooltip bắt buộc `[UX]` `[SEC]`

**Phía admin** (wizard tạo gói — không có field master password):

> **Bạn chỉ cần gửi mã kích hoạt cho người nhận.**  
> Người nhận sẽ **tự đặt mật khẩu** khi mở gói lần đầu.  
> Mã kích hoạt dùng **một lần** — không lưu lại sau khi đóng màn hình này.

**Phía người nhận** (màn đặt pass lần đầu — icon ⓘ):

> **Mật khẩu do bạn đặt — bạn phải tự lưu giữ.**  
> Nếu quên, **không thể khôi phục** trong app (không có “Quên mật khẩu”).  
> Liên hệ IT để nhận **gói chia sẻ mới** nếu cần.

**Invariant implement:**

- Admin wizard: hiển thị activation code **một lần**; checkbox *“Tôi đã copy mã / đã gửi cho người nhận.”* trước [Đóng].
- Recipient first-setup: tooltip + checkbox *“Tôi hiểu mật khẩu không thể khôi phục nếu mất.”* trước [Hoàn tất].
- Không lưu master password hay activation code trong app / keychain / `app_settings.json` sau khi flow kết thúc.
- Mất **mã kích hoạt** trước khi claim → admin tạo gói mới. Mất **pass sau claim** → không recovery (giống vault thường).

### KeePass/KeePassXC — có hỗ trợ không? → **Không đủ — đổ nợ kỹ thuật VipaVault**

> Tham chiếu: `.context/decisions/DECISION_VAULT_STORAGE.md` — SQLCipher primary; KDBX chỉ optional export Phase 2+.

| Tính năng Share Package | KeePass / KeePassXC | VipaVault phải tự làm |
|-------------------------|---------------------|------------------------|
| Export **subset** (chọn group/entry) | **Có — thủ công** (Export → file `.kdbx` mới, chọn entries) | Wizard checklist + FK integrity (`services` ↔ `credentials` ↔ `emails`) |
| Chọn **từng credential**, bỏ domain registrar | **Không relational** — chỉ cây entry; metadata dashboard/sync không đi theo | SQL `SELECT` subset + strip `activity_log` / sync cache |
| Người nhận **tự đặt pass lần đầu** (activation code → rekey) | **Không** — creator đặt master pass khi tạo/export DB; không có flow “claim” một lần | `first_setup_pending` + one-time code + SQLCipher `PRAGMA rekey` |
| `machine_role` viewer / disable write | **Không** — mở file = full quyền sửa (trừ read-only file OS) | `app_settings.json` + UI/IPC gate |
| Tooltip / không recovery | Cảnh báo chung; **không** flow share package | Wizard admin + first-setup recipient |
| Gói share = **snapshot**, không sync ngược | Không — hai file `.kdbx` độc lập, sync thủ công | Explicit invariant + (sau) optional re-export |

**Kết luận:** KeePass **không thay** milestone Share Package. Export KDBX (nếu làm Phase 2+) chỉ là **interop thô** cho IT quen KeePassXC — không có activation code, không viewer gate, không relational subset.

### Technical debt — ghi khi tới milestone Share Package `[ARCH]` `[OPEN]`

**Milestone đề xuất:** post-0.11.0 / Phase 1.5 (tên tạm: `0.12.0` hoặc `1.1.0` — chốt khi plan roadmap).

| # | Nợ kỹ thuật | Ước lượng | Ghi chú |
|---|-------------|-----------|---------|
| TD-SHARE-01 | Subset export engine (SQL → new `.hvault`) | M | Copy rows theo selection; regenerate UUIDs hoặc giữ id — spike |
| TD-SHARE-02 | Checklist UI tree (service → credential → email) | M | FRONTEND + IPC |
| TD-SHARE-03 | One-time activation code + `first_setup_pending` metadata | M | SEC review; entropy / expiry |
| TD-SHARE-04 | First-open flow: verify code → recipient pass → `rekey` | M | VAULT module; test zeroize |
| TD-SHARE-05 | Admin one-time code display + “đã copy” gate | S | UX |
| TD-SHARE-06 | Recipient tooltips + checkbox không recovery | S | UX copy đã chốt |
| TD-SHARE-07 | FK orphan rules khi partial export | M | VD: share email không share parent service metadata? |
| TD-SHARE-08 | (Optional) KDBX subset export — interop only | L | `keepass-db`; **không** thay TD-SHARE-01–06 |

**Không borrow từ KeePass:** TD-SHARE-03, 04, 07 — không có reference implementation trong KeePass ecosystem.

### Giải pháp đề xuất — research 2026-06 (Grok Build / web + SQLCipher docs)

> **Kết luận:** Có lộ trình **không cần thư viện mới** — dùng SQLCipher + SQLite patterns đã có; UI/IPC là phần làm chính. KeePass/Bitwarden/1Password **không** copy được offline-first nhưng cho **mental model**.

#### Kiến trúc tổng — 2 module Rust

```
share/export.rs   SharePackageExporter
share/claim.rs    SharePackageClaimer
share/manifest.rs SelectionManifest + FkClosureRules
```

#### Map nợ → giải pháp

| Nợ | Giải pháp | Nguồn / pattern |
|----|-----------|-----------------|
| **TD-SHARE-01** | **ATTACH staging DB** → `INSERT INTO share.* SELECT … WHERE id IN (?)` theo manifest → `DETACH` → file `.hvault` mới. Schema: chạy migration V1 trên DB trống trước khi insert. Không cần `sqlcipher_export()` (copy full DB). | [SQLite copy between DBs](https://stackoverflow.com/questions/2359205/copying-data-from-one-sqlite-database-to-another); [SQLCipher ATTACH + KEY](https://www.zetetic.net/sqlcipher/sqlcipher-api/) |
| **TD-SHARE-02** | Tree checkbox React — group theo `services.display_name` / `domain_primary`; leaf = `service_credentials`, `email_accounts`. IPC gửi `SelectionManifest` JSON. Pattern giống Bitwarden **Collections** (chọn tập con) nhưng offline. | [Bitwarden Collections](https://bitwarden.com/help/about-collections/) — concept only, cloud |
| **TD-SHARE-03** | Sinh `activation_code` = 128-bit random → hiển thị dạng **nhóm từ** (hoặc Crockford base32). File gói mã hóa bằng **chính activation code** qua Argon2id (cùng params vault). Trong vault: `app_settings` keys `share_package=1`, `share_claimed=0`, `share_export_id=uuid`. | Tương tự “temporary password đổi lần đầu login” enterprise; **không** lưu code sau wizard |
| **TD-SHARE-04** | First open: detect `share_claimed=0` → unlock bằng activation code → `PRAGMA rekey = recipient_pass` → set `share_claimed=1` → `zeroize()` buffers. Rust: `rusqlite` + feature SQLCipher, `connection.pragma_update(None, "rekey", …)`. | [SQLCipher PRAGMA rekey](https://www.zetetic.net/sqlcipher/sqlcipher-api/#rekey); [rusqlite pragma_update](https://docs.rs/rusqlite/latest/rusqlite/struct.Connection.html#method.pragma_update) |
| **TD-SHARE-05–06** | Copy UX đã chốt — không thêm tech. | — |
| **TD-SHARE-07** | **`FkClosureRules`** cố định trong manifest validator: (1) tick credential → auto include parent `service` row **metadata only**; (2) tick email → include parent service; (3) **không** auto-include sibling credentials; (4) `domains`/`ssl_certs` chỉ khi tick service domain hoặc tick explicit; (5) **never** copy `activity_log`, `cpanel_sync_cache` sang gói viewer (hoặc chỉ `sync_status` aggregate nếu cần dashboard). | App logic — không có off-shelf |
| **TD-SHARE-08** | Cùng `SelectionManifest` → adapter `keepass-db` map sang Group/Entry. **Song song**, không thay pipeline SQLCipher. | `DECISION_VAULT_STORAGE.md` §6.3 |

#### Flow kỹ thuật end-to-end (đề xuất implement)

```
[Export]
1. Admin chọn manifest M
2. Validate M với FkClosureRules
3. CREATE empty share.hvault (migration 001)
4. ATTACH share AS share KEY activation_code
5. INSERT SELECT subset từ vault admin (main)
6. INSERT app_settings: share_package, share_claimed=0
7. DETACH → đóng file; admin thấy activation_code một lần

[Claim]
1. User add share.hvault vào profiles.json
2. App thấy share_claimed=0 (cần unlock tạm bằng activation để đọc — hoặc plaintext header marker TBD spike)
3. Nhập activation_code → unlock
4. Nhập pass mới ×2 → PRAGMA rekey
5. share_claimed=1; activation_code zeroized
```

**Spike cần làm trước code:** đọc `share_claimed` **trước** unlock — options: (A) sidecar `.hvault.meta` không nhạy cảm; (B) `PRAGMA cipher_plaintext_header_size` + magic bytes `VVSH` (tradeoff SEC — review); (C) luôn prompt “Đây là gói chia sẻ?” + activation field trên login.

#### Tham chiếu SaaS (không dùng trực tiếp — offline khác)

| Sản phẩm | Bài học cho VipaVault |
|----------|----------------------|
| **Bitwarden Collections** | Partial share + read-only permission — map sang manifest + `machine_role` |
| **1Password Shared Vaults** | Tách vault theo scope — map sang multi-profile / share package file riêng |
| **KeePass Export** | Subset thủ công — TD-SHARE-08 chỉ interop |
| **age** | Optional: mã hóa thêm file `.hvault` khi truyền (USB/email) — **không** thay SQLCipher trong app |

#### Ước lượng lại sau research

| # | Trước | Sau | Lý do |
|---|-------|-----|-------|
| TD-SHARE-01 | M | **S–M** | ATTACH + INSERT SELECT — pattern chuẩn SQLite |
| TD-SHARE-03 | M | **S** | Activation code = initial PRAGMA key, không crypto riêng |
| TD-SHARE-04 | M | **S–M** | `PRAGMA rekey` documented, đã dùng trong vault core |
| TD-SHARE-07 | M | **M** | Vẫn cần spec rules + tests — không rút gọn nhiều |

**Tổng milestone Share Package:** ~2–3 vertical slices (export / claim / UI) — **khả thi post-0.11.0**, không blocker kiến trúc.

### FULL — defer (không V1)

- [ ] `workspace_members` + email gate login
- [ ] Settings → Share Workspace (allowlist email)
- [ ] `activity_log.actor_email`
- [ ] PIN quick unlock
- [ ] Share Package milestone + technical debt TD-SHARE-01–08 (post-0.11.0 / Phase 1.5)

---

## Schema draft — 0.2.0 (planning, chưa migrate)

> Bổ sung spec §4. Human-approved planning; implement tại milestone **0.2.0**.  
> **`workspace_members` — defer FULL**, không migration V1.

### Bổ sung: `email_accounts.sync_status`

```sql
-- ALTER / migration 002 — column on existing table
sync_status TEXT NOT NULL DEFAULT 'local_only'
-- 'local_only' | 'pending_sync' | 'synced' | 'sync_error'
```

### `activity_log` V1

Giữ `actor_note` (OS username hoặc free-text). `actor_email` — defer FULL (email gate).

### `app_settings.json` schema (per-machine, ngoài vault)

```json
{
  "machine_role": "admin",
  "sync_enabled": true
}
```

### Migration note (gap #4 — đề xuất, chưa chốt)

- **0.1.1:** tạo file `.hvault` encrypted (có thể trống hoặc chỉ `schema_version`).
- **0.2.0:** migration `001` tạo toàn bộ tables V1 (spec §4); chạy on first open / migrate path.

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
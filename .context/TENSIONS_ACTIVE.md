# Tensions — Active

> Chỉ chứa Status: RESOLVED_ACTIVE entries của milestone hiện tại.
> Agent đọc file này với tag filter.
> Move sang TENSIONS_HISTORY.md chỉ khi human approve milestone transition.

---

## 2026-05-15 | sync
Status:     RESOLVED_ACTIVE
Tension:    Auto-sync định kỳ vs manual sync
Options:    Auto (15 phút) → real-time, tiện | nhưng risk block IP cPanel vì không có SLA rate limit công khai
            Manual → user phải bấm Refresh | nhưng safe, controllable, không bị block
Decision:   Manual sync. Rate limit cứng: tối đa 1 lần / 10 phút / service.
Rationale:  cPanel không publish rate limit policy. Block IP = mất access toàn bộ hosting trên server đó. Downside không đối xứng — conservative là đúng.
Constraint: KHÔNG reopen trừ khi có rate limit policy chính thức bằng văn bản từ provider.
Severity:   high
Tags:       sync, provider, rate-limit, security
Milestone:  V1
Phase:      all

---

## 2026-05-15 | storage
Status:     RESOLVED_ACTIVE
Tension:    SQLCipher file-level encryption vs field-level AES-GCM trong SQLite thường
Options:    Field-level → granular, chỉ mã hóa field nhạy cảm | nhưng phức tạp, dễ miss field mới, maintenance burden mỗi khi thêm column
            SQLCipher → toàn bộ file mã hóa, đơn giản | copy .hvault sang máy viewer là xong, không cần setup thêm
Decision:   SQLCipher — AES-256-GCM toàn file, key từ Argon2id.
Rationale:  Use case copy file sang máy viewer làm SQLCipher rõ ràng hơn field-level. Không có nhu cầu partial decryption.
Constraint: KHÔNG reopen trừ khi xuất hiện use case cần partial decryption hoặc multi-user concurrent access vào cùng file.
Severity:   high
Tags:       storage, encryption, vault, security
Milestone:  V1
Phase:      all

---

## 2026-05-15 | provider_scope_v1
Status:     RESOLVED_ACTIVE
Tension:    V1 support cPanel only vs cPanel + DirectAdmin vs multi-provider đầy đủ
Options:    cPanel only → đơn giản nhất | nhưng DirectAdmin UAPI gần giống, bỏ phí
            cPanel + DirectAdmin → UAPI tương đồng, abstract qua provider_type
            Multi đầy đủ (Plesk, M365...) → flexible | nhưng Plesk dùng XML-RPC khác hoàn toàn
Decision:   cPanel + DirectAdmin dùng chung client, phân biệt qua provider_type. Plesk và OAuth provider để Phase 2+.
Rationale:  Login flow và UAPI của cPanel/DirectAdmin đủ giống để abstract đơn giản. Tách client khi có divergence thực tế, không tách sớm.
Constraint: KHÔNG thêm Plesk vào V1. KHÔNG thêm M365/Google vào V1.
Severity:   high
Tags:       provider, scope, v1, oauth
Milestone:  V1
Phase:      V1

---

## 2026-05-15 | autofill_mechanism
Status:     RESOLVED_ACTIVE
Tension:    KeePass/KeeForm để auto-fill portal vs vault riêng tích hợp
Options:    KeePass/KeeForm → không cần build | nhưng thêm dependency ngoài, UX fragmented
            Vault tích hợp → credential trong .hvault, mở portal bằng link trực tiếp
Decision:   Vault riêng tích hợp. KeePass bị drop.
Rationale:  App đã có vault mã hóa. Thêm KeePass = thêm dependency không cần thiết. Credential lưu trong .hvault, portal mở qua URL trong services table.
Constraint: KHÔNG reintroduce KeePass dependency. KDBX export/import (nếu có) là scope khác — xem `vault_tier_model` và `.context/decisions/DECISION_VAULT_STORAGE.md`.
Severity:   low
Tags:       ux, credential, dependency
Milestone:  V1
Phase:      all

---

## 2026-06-17 | vault_tier_model
Status:     RESOLVED_ACTIVE
Tension:    KeePass/KDBX hierarchy (Company → Service → Sub-credential) vs SQLCipher schema + UI tree
Options:    KeePass hybrid → native group tree cho secrets | nhưng bắt buộc dual-store (KDBX + SQL), sync risk, bypass confuse/audit nếu mở KeePassXC
            SQLCipher + FK + UI tree → 1 file .hvault, dashboard/sync/audit native | hierarchy chỉ kém trực quan hơn KDBX groups
Decision:   Giữ SQLCipher. Phân tầng 3–4 lớp qua multi-profile + services + service_credentials (mở rộng credential_type) + nested UI.
Rationale:  Pain point brainstorm là UX hierarchy và domain model, không phải crypto. Schema spec đã cover Tier 1–3; KeePass không giảm complexity tổng thể vì vẫn cần SQL cho metadata/workflow. Brainstorm + phản biện: `.context/decisions/DECISION_VAULT_STORAGE.md`.
Constraint: KHÔNG dùng KDBX làm primary vault backend V1. KHÔNG reopen trừ khi requirement cứng (compliance KDBX, IT bắt buộc KeePassXC daily, hoặc PoC hybrid pass).
Severity:   high
Tags:       storage, vault, ux, hierarchy, keepass
Milestone:  V1
Phase:      V1
Ref:        .context/decisions/DECISION_VAULT_STORAGE.md

---

## 2026-05-15 | role_model
Status:     RESOLVED_ACTIVE
Tension:    Multi user account system vs per-machine role config
Options:    User accounts → flexible | nhưng over-engineer cho use case 2 người (IT + CEO)
            Per-machine role → đơn giản | machine_role trong app_settings.json
Decision:   Per-machine role. app_settings.json: machine_role = admin | viewer.
Rationale:  Chỉ có 2 loại user thực tế. User account system tạo complexity không cần thiết. Viewer machine nhận file .hvault copy, sync disabled.
Constraint: KHÔNG build user authentication system trong app. Nếu cần thêm role → mở tension mới, không tự thêm.
Severity:   high
Tags:       role, viewer, security, scope
Milestone:  V1
Phase:      all

---

## 2026-05-15 | m365_google_schema
Status:     RESOLVED_ACTIVE
Tension:    Nhét M365/Google credential vào service_credentials vs bảng riêng
Options:    Chung bảng → đơn giản | nhưng OAuth credential không fit schema username/password
            Bảng riêng oauth_credentials → clean separation, không break V1 data
Decision:   Bảng oauth_credentials riêng, thêm ở Phase 2. V1 không động vào.
Rationale:  OAuth credential có structure khác hoàn toàn. Dùng chung bảng = type confusion. Thêm bảng mới ở Phase 2 không break migration V1.
Constraint: KHÔNG lưu OAuth credential vào service_credentials dù chỉ là temporary.
Severity:   high
Tags:       oauth, schema, provider, storage
Milestone:  V1
Phase:      Phase 2

---

## 2026-06-18 | staleness | src-tauri_src
Status:      RESOLVED_ACTIVE
Tension:     `[auto]` thay đổi (`run()` → `build_app()`) nhưng `[manual]` chưa review
Options:     Giữ placeholder generated | cập nhật `APP.md` + rebuild context-gen
Decision:    Cập nhật `.context/modules/APP.md` (build_app, capabilities, TDD B); `context-gen build` refresh AST.
Rationale:   Foundation 0.1.0 exit; manual source of truth là modules/, không sửa generated tay.
Constraint:  KHÔNG reopen trừ khi lib.rs bootstrap pattern đổi lại.
Severity:    low
Tags:        staleness, src-tauri_src, foundation
Milestone:   V1
Phase:       V1

---

## 2026-06-18 | auth_ui | operator_email
Status:      RESOLVED_ACTIVE
Tension:     Nhãn người thao tác trên login — free-text, OS user, hay email UI?
Options:     operator_display_name free-text | OS whoami only | email ô nhập + lưu text backend
Decision:    `operator_email` plaintext trong `app_settings.json`; backend Rust đọc/ghi; lần đầu UI email + regex format; fallback whoami; không tham gia unlock.
Rationale:   Cùng lớp “lưu text hiển thị”; UI sẵn cho V2 shell; per-machine tránh lệch khi copy .hvault sang viewer.
Constraint:  KHÔNG dùng operator_email làm credential V1; KHÔNG allowlist/DNS verify; reopen chỉ nếu đổi sang auth email thật (V2).
Severity:    low
Tags:        auth, ui, foundation, simple-v1
Milestone:   V1
Phase:       V1
Ref:         docs/technical-decisions.md §3.1.2

---

## 2026-06-18 | staleness | src-tauri_src_commands
Status:      RESOLVED_ACTIVE
Tension:     `[auto]` hash mismatch sau thêm `app_status` tests
Options:     Bỏ qua | confirm `COMMANDS.md` + rebuild
Decision:    `COMMANDS.md` đã mô tả `app_status` foundation; rebuild clears staleness.
Rationale:   Low severity; module manual đã đầy đủ trước khi resolve.
Constraint:  KHÔNG reopen trừ khi IPC surface thay đổi mà không cập nhật COMMANDS.md.
Severity:    low
Tags:        staleness, src-tauri_src_commands, foundation
Milestone:   V1
Phase:       V1


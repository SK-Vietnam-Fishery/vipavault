# TENSIONS.md — VipaVault
> File này có 2 loại entry: OPEN (chưa resolve) và RESOLVED (decision log vĩnh viễn).
> Agent mới: đọc toàn bộ RESOLVED trước khi làm task bất kỳ.
> KHÔNG xóa entry RESOLVED. KHÔNG reopen trừ khi điều kiện trong Constraint được thỏa.

---

## DECISION LOG — Những quyết định đã chốt

---

## 2026-05-15 | sync | RESOLVED
Tension:     Auto-sync định kỳ vs manual sync
Options:     Auto (15 phút) → real-time, tiện | nhưng risk block IP cPanel vì không có SLA rate limit công khai
             Manual → user phải bấm Refresh | nhưng safe, controllable, không bị block
Decision:    Manual sync. Rate limit cứng: tối đa 1 lần / 10 phút / service.
Rationale:   cPanel không publish rate limit policy. Block IP = mất access toàn bộ
             hosting trên server đó. Downside không đối xứng — conservative là đúng.
Constraint:  KHÔNG reopen trừ khi có rate limit policy chính thức bằng văn bản từ provider.
Phase:       all

---

## 2026-05-15 | storage | RESOLVED
Tension:     SQLCipher (file-level encryption) vs field-level AES-GCM trong SQLite thường
Options:     Field-level → granular, chỉ mã hóa field nhạy cảm
                         | nhưng phức tạp, dễ miss field mới, maintenance burden mỗi khi thêm column
             SQLCipher  → toàn bộ file mã hóa, đơn giản
                         | copy .hvault sang máy sếp (viewer) là xong, không cần setup thêm
Decision:    SQLCipher — AES-256-GCM toàn file, key từ Argon2id.
Rationale:   Use case copy file sang máy viewer làm SQLCipher rõ ràng hơn field-level.
             Không có nhu cầu partial decryption. Đơn giản hơn = ít bug hơn.
Constraint:  KHÔNG reopen trừ khi xuất hiện use case cần partial decryption hoặc
             multi-user concurrent access vào cùng file.
Phase:       all

---

## 2026-05-15 | provider_scope_v1 | RESOLVED
Tension:     V1 support cPanel only vs cPanel + DirectAdmin vs multi-provider đầy đủ
Options:     cPanel only → đơn giản nhất | nhưng DirectAdmin UAPI gần giống, bỏ phí
             cPanel + DirectAdmin → UAPI tương đồng, abstract qua provider_type
             Multi đầy đủ (Plesk, M365...) → flexible | nhưng Plesk dùng XML-RPC khác hoàn toàn
Decision:    cPanel + DirectAdmin dùng chung client, phân biệt qua provider_type.
             Plesk và OAuth provider (M365, Google) để Phase 2+.
Rationale:   Login flow và UAPI của cPanel/DirectAdmin đủ giống để abstract đơn giản.
             Tách client khi có divergence thực tế, không tách sớm.
Constraint:  KHÔNG thêm Plesk vào V1 — XML-RPC khác hoàn toàn, cần client riêng.
             KHÔNG thêm M365/Google vào V1 — OAuth flow cần bảng oauth_credentials riêng.
Phase:       V1

---

## 2026-05-15 | autofill_mechanism | RESOLVED
Tension:     KeePass/KeeForm để auto-fill portal vs vault riêng tích hợp
Options:     KeePass/KeeForm → không cần build | nhưng thêm dependency ngoài, UX fragmented
             Vault tích hợp → credential trong .hvault, mở portal bằng link trực tiếp
Decision:    Vault riêng tích hợp. KeePass bị drop.
Rationale:   App đã có vault mã hóa. Thêm KeePass = thêm dependency không cần thiết.
             Credential lưu trong .hvault, portal mở qua URL trong services table.
Constraint:  KHÔNG reintroduce KeePass dependency.
Phase:       all

---

## 2026-05-15 | role_model | RESOLVED
Tension:     Multi user account system vs per-machine role config
Options:     User accounts → flexible | nhưng over-engineer cho use case 2 người (IT + CEO)
             Per-machine role → đơn giản | machine_role trong app_settings.json
Decision:    Per-machine role. app_settings.json: machine_role = admin | viewer.
Rationale:   Chỉ có 2 loại user thực tế. User account system tạo complexity không cần thiết.
             Viewer machine (máy sếp) nhận file .hvault copy, sync disabled.
Constraint:  KHÔNG build user authentication system trong app.
             Nếu cần thêm role → mở tension mới, không tự thêm.
Phase:       all

---

## 2026-05-15 | m365_google_schema | RESOLVED
Tension:     Nhét M365/Google credential vào service_credentials vs bảng riêng
Options:     Chung bảng → đơn giản | nhưng OAuth credential (tenant_id, client_secret, JSON key)
             không fit schema username/password → schema vẹo
             Bảng riêng oauth_credentials → clean separation, không break V1 data
Decision:    Bảng oauth_credentials riêng, thêm ở Phase 2. V1 không động vào.
Rationale:   OAuth credential có structure khác hoàn toàn. Dùng chung bảng = type confusion.
             Thêm bảng mới ở Phase 2 không break migration V1.
Constraint:  KHÔNG lưu OAuth credential vào service_credentials dù chỉ là temporary.
Phase:       Phase 2

---

# OPEN TENSIONS

> Chưa có. Thêm vào đây khi detect tension trong quá trình làm task.

<!-- Template:
## [YYYY-MM-DD HH:MM] | [module] | OPEN
Tension:    mô tả conflict
Context:    đang làm task gì
Proposal:   muốn làm gì
Constraint: rule nào conflict (trích dẫn từ spec hoặc [manual])
Severity:   low | high
Decision:   [human fill in]
-->

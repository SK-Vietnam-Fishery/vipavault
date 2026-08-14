# Module 4 — System Design cho Desktop App

> **Mục tiêu:** Thiết kế được (và bảo vệ được) kiến trúc VipaVault: trust boundary, IPC, data modeling, schema evolution, provider abstraction, xử lý lỗi và concurrency. Đây là module biến "biết code" thành "biết thiết kế".
>
> **Thời lượng:** 2 tuần · **Điều kiện vào:** Module 1; nên đọc song song Module 2.

---

## 4.1 Kiến trúc tổng và trust boundary

Nắm vững sơ đồ trong `technical-decisions.md` §2: WebView (hạn chế tin cậy) ↔ Rust core (tin cậy đầy đủ) ↔ file/API bên ngoài.

**Nguyên tắc rút ra:**
- Mọi enforcement (role check, rate limit, validate) phải nằm ở **Rust**, UI chỉ phản chiếu — UI disable nút là UX, backend từ chối mới là bảo vệ.
- Dữ liệu qua IPC là **hàng nhập khẩu**: validate ở biên, đừng tin type của TypeScript (WebView có thể bị thao túng).

**Tài liệu:**
- Tauri Security — https://v2.tauri.app/security/
- Tauri Process Model — https://v2.tauri.app/concept/process-model/
- OWASP Input Validation Cheat Sheet — https://cheatsheetseries.owasp.org/cheatsheets/Input_Validation_Cheat_Sheet.html

**Bài tập:** vẽ C4 diagram mức Container cho VipaVault (tay hoặc mermaid), đánh dấu trust boundary bằng nét đứt. So với sơ đồ trong technical-decisions — bạn vẽ thiếu/thừa gì? C4 model: https://c4model.com/

## 4.2 Thiết kế bề mặt IPC (API design nội bộ)

Bề mặt `#[tauri::command]` chính là API của app. Nguyên tắc:

| Nguyên tắc | Ví dụ VipaVault |
|-----------|-----------------|
| Command theo **use case**, không theo bảng | `unlock_vault(profile_id, password)` chứ không `execute_sql(...)` |
| Trả lỗi **phân loại được** | Enum lỗi: `WrongPassword` vs `FileCorrupt` vs `FileNotFound` — UI hiển thị khác nhau |
| Secret không bao giờ đi "nhờ" response | Trả `Entry` không kèm password; reveal là command riêng + ghi audit |
| Idempotency khi có thể | Bấm "Tạo email" 2 lần không tạo 2 email (UNIQUE constraint + xử lý conflict) |

**Tài liệu:**
- Tauri — Calling Rust from the Frontend: https://v2.tauri.app/develop/calling-rust/
- "A Philosophy of Software Design" (Ousterhout) ch. về deep modules — API mỏng che phức tạp dày: https://web.stanford.edu/~ouster/cgi-bin/book.php

**Bài tập:** liệt kê toàn bộ command cần cho milestone 0.1.1 + 0.3.0 (tên, tham số, kiểu trả về, kiểu lỗi) **trước khi code** — đây là interface-first design. Đối chiếu spec §3.1 và IPC ghi chú trong technical-decisions §3.1.1.

## 4.3 Data modeling & schema evolution

| Học | Nội dung | Tài liệu |
|-----|----------|----------|
| Mô hình quan hệ, normalization vừa đủ | Schema V1 spec §4 — hiểu tại sao `email_accounts` tách khỏi `services` | DDIA ch.2 (Kleppmann) — https://dataintensive.net/ |
| Migrations: forward-only, đánh version | Mỗi thay đổi schema = 1 file SQL đánh số; `user_version` PRAGMA | https://www.sqlite.org/pragma.html#pragma_user_version · refinery crate: https://docs.rs/refinery |
| Bẫy SQLite `ALTER TABLE` hạn chế | Không DROP COLUMN dễ dàng (bản cũ), pattern "new table + copy + rename" | https://www.sqlite.org/lang_altertable.html §"making other kinds of changes" |
| Downgrade protection | App cũ mở DB schema mới → phải từ chối đọc, không silent corrupt | Ghi `user_version` + check khi mở |
| Sparse fields: JSON column vs EAV | Case thật của repo: `PROPOSAL_CUSTOM_FIELDS.md` + phần Fable phản biện | https://www.sqlite.org/json1.html |

**Bài tập:** viết chuỗi 3 migration cho DB chơi: v1 tạo bảng, v2 thêm cột, v3 đổi kiểu cột tiền REAL→INTEGER (pattern new-table-copy-rename, kèm chuyển đổi giá trị). Viết test: DB v1 chạy qua 3 migration ra đúng v3; DB v3 mở bằng "app v1 giả" phải bị từ chối.

## 4.4 Provider abstraction — ports & adapters

`provider_type` + `auth_scheme` là **port**; cPanel/DirectAdmin client là **adapter**. Invariants spec §5 ("V1 ignore provider lạ — warn + skip, không panic") là hợp đồng của port.

**Tài liệu:**
- Hexagonal Architecture (Alistair Cockburn, bản gốc): https://alistair.cockburn.us/hexagonal-architecture/
- cPanel UAPI reference (adapter thật sẽ gọi): https://api.docs.cpanel.net/
- DirectAdmin API: https://docs.directadmin.com/developer/api/

**Bài tập:** định nghĩa trait `EmailProvider` (list/create/reset/delete) + 2 implement: `MockProvider` (cho test) và skeleton `CpanelProvider` (chưa gọi mạng, chỉ dựng request đúng format UAPI). Viết test routing: service `provider_type='plesk'` → warn + skip, không lỗi.

## 4.5 Rate limiting, retry, và lỗi từ thế giới ngoài

- Rate limit 1 lần/10 phút/service là **quyết định risk-asymmetry** (bị block IP = mất toàn bộ access) — đọc lý do trong technical-decisions §5.2 và bảo vệ được nó.
- Phân loại lỗi mạng: timeout / 4xx / 5xx / parse error — cái nào retry được, cái nào không (4xx thường là không).
- Trạng thái `pending_sync` → apply: mô hình **outbox** thu nhỏ — thay đổi ghi local trước, đẩy đi sau, có review.

**Tài liệu:**
- Google SRE Book ch.22 — Addressing Cascading Failures (retry amplification): https://sre.google/sre-book/addressing-cascading-failures/
- AWS Architecture — Exponential Backoff and Jitter: https://aws.amazon.com/builders-library/timeouts-retries-and-backoff-with-jitter/

## 4.6 Concurrency trong backend

- Một vault mở = một connection + state trong `Arc<Mutex<...>>` — hiểu deadlock cơ bản và tại sao giữ lock qua `.await`/gọi chậm là bug.
- Auto-lock timer chạy nền: cần cancel-safe khi user hoạt động lại.
- SQLite một writer tại một thời điểm — với desktop app điều này **đơn giản hóa** mọi thứ; đừng thêm connection pool khi chưa cần.

**Tài liệu:** The Book ch.16 (Fearless Concurrency) — https://doc.rust-lang.org/book/ch16-00-concurrency.html · rusqlite + threading: https://docs.rs/rusqlite/latest/rusqlite/#threading

## 4.7 Câu hỏi system design phải trả lời được

Viết câu trả lời vào journal — vài câu chưa trả lời được cho đến khi làm milestone tương ứng, quay lại điền:

1. App crash **giữa** một transaction ghi email mới — mở lại vault thấy gì? Cơ chế nào của SQLite bảo đảm điều đó? (atomicity, WAL)
2. Mất điện **giữa** `PRAGMA rekey` — file ở trạng thái nào? Cần quy trình gì trước khi rekey? (backup trước, verify sau)
3. User mở **2 cửa sổ app** cùng một `.hvault` — chuyện gì xảy ra? Có cần lock file không?
4. `.hvault` nằm trong thư mục OneDrive đang sync — liệt kê 2 kịch bản hỏng. (howtocorrupt.html)
5. Viewer machine có file + password — thiết kế nào ngăn viewer *sửa* dữ liệu? Trả lời trung thực: không có — chỉ có UX + audit (pm-review §3). Vậy audit log đọc được từ máy nào?
6. Đồng hồ máy CEO sai 3 tháng — dashboard cảnh báo hết hạn hiển thị gì? Chống thế nào?
7. Provider API đổi format response — code fail ở đâu, lỗi hiển thị cho user thế nào, và test nào bắt được trước khi user thấy?
8. 5.000 entries + search full-text custom fields — nghẽn ở đâu trước: SQL, IPC serialize, hay React render? Đo bằng gì?
9. Thêm provider thứ 3 (Plesk) — liệt kê **mọi file phải đụng**. Nếu >4 chỗ, abstraction đang rò.
10. Migration v5 lỗi giữa chừng trên máy user — rollback thế nào? (transaction quanh migration + backup file trước khi migrate)
11. Hai máy admin cùng sửa 2 bản copy `.hvault` — merge được không? Vì sao không? (đây là cửa vào Module 6 — sync)
12. Nếu ngày mai phải đổi từ SQLCipher sang backend khác — lớp nào của code phải viết lại, lớp nào không? Câu trả lời đo mức độ coupling của vault engine.

## Checkpoint ra khỏi Module 4

- [ ] C4 container diagram có trust boundary
- [ ] Danh sách command 0.1.1 + 0.3.0 (interface-first) hoàn chỉnh
- [ ] Bộ 3 migration + test up/downgrade-protection chạy xanh
- [ ] Trait `EmailProvider` + mock + test routing skip
- [ ] 12 câu hỏi có câu trả lời (hoặc đánh dấu "trả lời tại milestone X")

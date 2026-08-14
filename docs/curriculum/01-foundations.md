# Module 1 — Foundations: Rust · TypeScript/React · Tauri 2 · SQLite

> **Mục tiêu:** Đủ nền để đọc hiểu và tự viết mọi phần của VipaVault. Không học "toàn bộ" từng công nghệ — học đúng tập con mà codebase này dùng, có chỉ dẫn phần nào bỏ qua được.
>
> **Thời lượng:** 4–6 tuần · **Điều kiện vào:** biết một ngôn ngữ lập trình bất kỳ, dùng được git cơ bản.

---

## 1.1 Rust (2–3 tuần) — phần quan trọng nhất

Backend VipaVault là Rust vì logic nhạy cảm (key, credential) cần memory safety. Đây là ngôn ngữ khó nhất trong stack — đầu tư ở đây trả lãi cả giáo trình.

### Học gì, theo thứ tự

| # | Chủ đề | Vì sao VipaVault cần | Tài liệu |
|---|--------|----------------------|----------|
| 1 | Ownership, borrowing, lifetimes | Hiểu tại sao key material "move" được và "zeroize" được | The Book ch.4, ch.10.3 — https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html |
| 2 | `Result`, `Option`, `?`, error handling | Mọi Tauri command trả `Result`; vault mở fail phải báo đúng lỗi | The Book ch.9; bài "Error Handling in Rust" — https://blog.burntsushi.net/rust-error-handling/ |
| 3 | Structs, enums, pattern matching | `provider_type` routing là một `match` lớn | The Book ch.5–6 |
| 4 | Traits, generics | Provider abstraction (cPanel/DirectAdmin cùng interface) | The Book ch.10 |
| 5 | Modules, crates, Cargo | Tổ chức `src-tauri/src/{vault,providers,commands,sync}` | The Book ch.7, ch.14 |
| 6 | Testing (`#[test]`, integration tests) | Exit criteria mọi milestone là `cargo test` xanh | The Book ch.11 — https://doc.rust-lang.org/book/ch11-00-testing.html |
| 7 | `serde` serialize/deserialize | JSON qua IPC, `profiles.json`, custom_fields | https://serde.rs/ |
| 8 | Smart pointers, `Arc<Mutex<T>>` cơ bản | State vault đang mở chia sẻ giữa các command | The Book ch.15–16 |

**Bỏ qua được (chưa cần):** async/await sâu (chỉ cần mức gọi HTTP client), macro tự viết, unsafe, FFI.

### Bài tập bắt buộc

1. **Rustlings** — làm hết phần ownership, error handling, traits: https://github.com/rust-lang/rustlings
2. Viết CLI nhỏ (ngoài repo): đọc file JSON danh sách credential giả → filter theo field → in bảng. Ép dùng `serde`, `Result`, module tách file. Đây chính là hình dạng thu nhỏ của vault engine.
3. Viết unit test cho CLI trên, gồm 1 test case lỗi (file không tồn tại, JSON hỏng).

### Câu hỏi phải trả lời được

- Tại sao Rust không cho hai mutable reference cùng lúc, và điều đó liên quan gì đến việc lock/unlock vault an toàn?
- `String` vs `&str` — khi nào dùng cái nào trong signature của Tauri command?
- Khi một `struct` chứa key material bị `drop`, memory có bị xóa không? (Gợi ý: không — đó là lý do tồn tại crate `zeroize`, học ở Module 2.)

---

## 1.2 TypeScript + React (1–1.5 tuần)

Frontend chỉ là lớp hiển thị — **WebView là vùng hạn chế tin cậy** (technical-decisions §2), logic nhạy cảm không nằm ở đây. Học vừa đủ.

| Chủ đề | Vì sao cần | Tài liệu |
|--------|-----------|----------|
| React components, props, state, hooks (`useState`, `useEffect`) | Toàn bộ UI: login, dashboard, list | https://react.dev/learn |
| TypeScript types, interfaces, generics cơ bản | Type cho dữ liệu IPC (Entry, Service, Filter) | https://www.typescriptlang.org/docs/handbook/intro.html |
| Vite dev/build | `npm run dev` / `npm run build` của repo | https://vite.dev/guide/ |
| Vitest + Testing Library | `npm test` của repo; test hành vi không test implementation | https://vitest.dev/guide/ · https://testing-library.com/docs/react-testing-library/intro/ |

**Bỏ qua được:** Redux/state manager ngoài (app này đủ nhỏ cho hooks + context), SSR/Next.js (đã loại trong technical-decisions), CSS framework phức tạp.

### Bài tập

1. Tutorial chính thức React (tic-tac-toe) rồi **làm lại không nhìn**: https://react.dev/learn/tutorial-tic-tac-toe
2. Dựng form login 3 dòng đúng thứ tự spec (tên vault → email → password) với validate rỗng, bằng dữ liệu giả — chưa cần Tauri. So với mock trong `technical-decisions.md` §3.1.2.

---

## 1.3 Tauri 2 (1 tuần)

| Chủ đề | Vì sao cần | Tài liệu |
|--------|-----------|----------|
| Process model: WebView vs Core | Trust boundary của toàn kiến trúc | https://v2.tauri.app/concept/process-model/ |
| Commands + `invoke()` | Toàn bộ giao tiếp FE↔BE của app | https://v2.tauri.app/develop/calling-rust/ |
| State management (`tauri::State`) | Giữ connection vault đang mở | https://v2.tauri.app/develop/state-management/ |
| Capabilities / Permissions (ACL) | Chặn WebView gọi API ngoài danh sách cho phép | https://v2.tauri.app/security/capabilities/ |
| Security model tổng quan | Hiểu Tauri bảo vệ gì và KHÔNG bảo vệ gì | https://v2.tauri.app/security/ |

### Bài tập

1. Tạo app Tauri "hello" ngoài repo: 1 command Rust nhận string, trả về string đảo ngược; gọi từ React. Sau đó thêm 1 command trả `Result::Err` và xử lý lỗi phía UI.
2. Đọc file capabilities của repo VipaVault, giải thích từng dòng bằng comment.

### Câu hỏi phải trả lời được

- Nếu WebView bị XSS (một chuỗi hiển thị chứa script), attacker gọi được những gì? Danh sách đó do file nào quyết định?
- Tại sao KHÔNG bao giờ đưa master password vào state của React lâu hơn thời điểm bấm "Mở khóa"?

---

## 1.4 SQLite + SQL (1 tuần)

Vault engine = SQLCipher = SQLite + lớp mã hóa. Kỹ năng SQLite dùng thẳng cho SQLCipher.

| Chủ đề | Vì sao cần | Tài liệu |
|--------|-----------|----------|
| SQL: JOIN, index, transaction | Schema 8 bảng có FK; dashboard là các câu SELECT tổng hợp | https://www.sqlite.org/lang.html · sách bài tập: https://sqlbolt.com/ |
| Datatypes & type affinity | Hiểu tại sao tiền phải INTEGER không REAL (pm-review §8) | https://www.sqlite.org/datatype3.html |
| WAL mode | Spec chọn WAL chống corruption | https://www.sqlite.org/wal.html |
| How to corrupt SQLite | Biết điều KHÔNG được làm (sync folder, nhiều process) | https://www.sqlite.org/howtocorrupt.html |
| JSON1 (`json_extract`, `json_each`) | Custom fields proposal | https://www.sqlite.org/json1.html |
| rusqlite (Rust binding) | Cách repo nói chuyện với DB | https://docs.rs/rusqlite/latest/rusqlite/ |

### Bài tập

1. Tạo DB SQLite tay (CLI `sqlite3`) với schema `services` + `email_accounts` từ spec §4; INSERT 10 dòng; viết query "các service hết hạn trong 30 ngày" và "đếm email theo service".
2. Viết chương trình Rust dùng `rusqlite`: mở DB trên, chạy 2 query đó, in kết quả. Bọc INSERT trong transaction và chứng minh rollback hoạt động (test).

---

## Checkpoint ra khỏi Module 1

- [ ] Rustlings xong các phần liệt kê; CLI credential giả chạy + có test
- [ ] Giải thích được ownership/borrowing bằng lời của mình trong journal
- [ ] App Tauri hello với command lỗi được xử lý đúng ở UI
- [ ] DB SQLite tay + chương trình rusqlite có transaction test
- [ ] Trả lời hết các "câu hỏi phải trả lời được" — viết vào journal

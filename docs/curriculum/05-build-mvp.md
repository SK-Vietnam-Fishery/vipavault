# Module 5 — Xây V1 theo Milestone (0.1.0 → 0.10.0)

> **Mục tiêu:** Biến kiến thức Module 1–4 thành sản phẩm chạy được, theo đúng roadmap đã reorder (dogfood tại 0.6.0, release tại 0.10.0). **MVP chỉ là một chương của giáo trình** — ship thật nằm ở Module 7.
>
> **Nguồn chuẩn:** goal / deliverables / exit criteria của từng milestone lấy từ [`MILESTONES_REFERENCE.md`](../../.context/planning/MILESTONES_REFERENCE.md) — file này không lặp lại, chỉ bổ sung *kỹ năng cần, thứ tự làm, và bẫy*.
>
> **Thời lượng:** 8–14 tuần.

---

## Cách làm việc mỗi milestone

1. Đọc section milestone trong MILESTONES_REFERENCE — chép exit criteria vào journal làm checklist.
2. Viết **danh sách command IPC + thay đổi schema** trước (interface-first, Module 4.2).
3. Viết test cho exit criteria **trước hoặc song song** code (không cần TDD giáo điều — nhưng exit criteria phải có test tự động tương ứng).
4. Xong → chạy `npm run verify` → viết journal 10 dòng → mới sang milestone kế.

---

## 0.1.0 — Foundation *(đã xong trong repo — dùng làm bài đọc-hiểu)*

**Bài tập thay thế:** đọc toàn bộ scaffold hiện có và viết vào journal: Tauri config nằm đâu, test runner nối thế nào, `npm run verify` chạy những gì. Xóa một dòng bất kỳ trong config để nó hỏng, đọc lỗi, sửa lại — học đọc lỗi build là kỹ năng ngầm quan trọng nhất của Tauri.

## 0.1.1 — Vault Core *(kỹ năng: Module 2 toàn bộ)*

- **Thứ tự làm:** KDF thuần (hàm + test) → create/open/lock trên file tạm (integration test, chưa UI) → state `Arc<Mutex>` + command IPC → UI login → auto-lock timer.
- **Cộng thêm từ pm-review §4:** Emergency Kit in được khi tạo vault.
- **Bẫy đã biết:**
  - Test Argon2id 64MB chạy chậm trên CI — tách tham số test nhỏ qua `#[cfg(test)]`, nhưng phải có **1 test integration dùng đúng tham số thật**.
  - Lỗi "wrong password" và "file corrupt" của SQLCipher trông giống nhau (`file is not a database`) — phân biệt bằng thử mở + kiểm tra schema version, đừng đoán.
  - Đường dẫn Windows vs WSL (`~/.vipavault/`) — quyết định sớm dùng `dirs` crate: https://docs.rs/dirs

## 0.2.0 — Data Model & Migrations *(kỹ năng: Module 4.3)*

- Schema từ spec §4 — **kèm 2 sửa đã chốt:** tiền INTEGER (pm-review §8); cân nhắc cột `custom_fields` nếu proposal được approve sau sửa F1–F4.
- Mỗi bảng: migration file riêng đánh số + test insert/select round-trip.
- **Bẫy:** SQLite không enforce FK mặc định — bật `PRAGMA foreign_keys = ON` **mỗi connection** và viết test chứng minh FK thật sự chặn: https://www.sqlite.org/foreignkeys.html

## 0.3.0 — App Shell & Roles *(kỹ năng: Module 4.1–4.2)*

- **Điều kiện vào (pm-review §1):** đã làm phỏng vấn Mom Test với CEO (Module 3) — nếu kết quả là "PDF là đủ", scope milestone này co lại đáng kể; đừng bỏ qua bước này.
- Role check ở **backend** từng command (viewer → từ chối write), UI disable chỉ là phản chiếu. Test: gọi command write với `machine_role: viewer` qua IPC → phải bị từ chối dù UI không cho bấm.

## 0.4.0 — Dashboard *(kỹ năng: SQL tổng hợp Module 1.4)*

- Ngưỡng cảnh báo đỏ/vàng/xanh từ spec §6 — viết thành hàm thuần có test (input: expires_at, now → output: mức) trước khi đụng UI.
- **Cộng thêm từ pm-review §2:** banner "Dữ liệu tại ngày X" cho viewer.
- **Bẫy:** timezone — `expires_at` là unix timestamp, hiển thị theo local TZ; test với mốc nửa đêm.

## 0.5.0 — Credential Management *(kỹ năng: Module 2.6)*

- Reveal password = command riêng + ghi `activity_log` + auto-hide sau N giây + copy clipboard có tự xóa.
- **Từ đây bạn có thể bắt đầu nhập dữ liệu thật của công ty** — backup trước mỗi phiên làm việc trên vault thật.

## 0.6.0 — Email Accounts Local — **🏁 MỐC DOGFOOD**

- CSPRNG password generator (Module 2.6) + CRUD email local + audit.
- **Nghi thức dogfood:** từ tuần này, dùng app cho việc thật hằng ngày. Đặt lịch 4 tuần; ghi journal mỗi lần app làm bạn **quay lại Excel/cPanel** — mỗi lần như vậy là một bug report thật cho chính mình.
- **Cổng quyết định:** 0.7.0–0.9.0 **chỉ làm** nếu sau ≥1 tháng dogfood, thao tác cPanel tay vẫn đau (theo quyết định 2026-07-13). Đau ít → nhảy thẳng 0.10.0, ship sớm hơn 3 milestone.

## 0.7.0 — Provider Routing *(kỹ năng: Module 4.4)*

- Trait + mock đã viết ở Module 4 — giờ nối vào app thật. Invariant: provider lạ → warn + skip.

## 0.8.0 — Manual Sync & Rate Limit *(kỹ năng: Module 4.5)*

- Rate limit state persist qua restart (lưu `last_synced_at` trong DB, không chỉ RAM) — test: sync, đóng app, mở lại, sync ngay → phải bị chặn.
- **Bẫy:** gọi HTTP thật trong test — dùng mock server (crate `wiremock`): https://docs.rs/wiremock

## 0.9.0 — Provider Email Apply *(kỹ năng: Module 4.5 outbox)*

- `pending_sync` → review → apply. Test kịch bản: apply fail giữa chừng 3 email (email 2 lỗi mạng) — trạng thái từng email phải đúng, retry không tạo trùng (idempotency).
- Đây là milestone **rủi ro thật cao nhất** (ghi lên hệ thống production của công ty) — chạy lần đầu với 1 email test trên hosting ít quan trọng nhất.

## 0.10.0 — MVP Hardening & Release *(chuyển tiếp sang Module 7)*

- Auto-lock, WAL/backup checks, error handling pass, packaging, final test sweep — theo exit criteria trong MILESTONES_REFERENCE.
- **Cộng thêm từ pm-review:** nút "Đổi master password" (rekey — §6), nhắc backup định kỳ (§5).
- Toàn bộ kỹ thuật đóng gói/ký số/phân phối học ở **Module 7** — milestone này là nơi áp dụng.

---

## Kỹ thuật học khi bị kẹt (không có AI agent)

| Kẹt kiểu | Làm gì |
|----------|--------|
| Lỗi compiler Rust khó hiểu | Đọc **từ dòng đầu** + `rustc --explain EXXXX`; 80% lỗi borrow có gợi ý sửa ngay trong message |
| Không biết API crate | docs.rs của crate đó, mục Examples trước; sau đó đọc test của chính crate trên GitHub |
| Bug hành vi | Viết test tái hiện **trước khi** sửa — bug không tái hiện được là bug chưa hiểu |
| Kẹt > 1 ngày | Viết câu hỏi chuẩn Stack Overflow (minimal reproducible example) — 50% trường hợp tự thấy đáp án khi thu nhỏ ví dụ: https://stackoverflow.com/help/minimal-reproducible-example |
| Cần người thật | Tauri Discord (https://discord.com/invite/tauri), users.rust-lang.org, r/rust |

## Checkpoint ra khỏi Module 5

- [ ] `npm run verify` xanh tại mọi milestone đã qua
- [ ] Dogfood ≥4 tuần từ 0.6.0, journal có danh sách "lần quay lại Excel/cPanel"
- [ ] Quyết định cổng 0.7.0–0.9.0 được ghi rõ ràng (làm/bỏ + lý do)
- [ ] Vault thật của công ty đang chạy trên app, có backup

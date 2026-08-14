# Module 8 — Điểm mù: những thứ bạn (có lẽ) chưa tính tới

> **Mục tiêu:** Danh mục các vấn đề thường chỉ lộ ra khi đã muộn — mỗi mục: vì sao nó tồn tại, mức phải lo, làm gì. Đọc lướt một lần đầu giáo trình, quay lại từng mục khi đến milestone liên quan. Không mục nào ở đây là lý thuyết suông — tất cả đều từng đánh gục sản phẩm thật.

---

## A. Bảo mật ngoài phạm vi mã hóa file

### A1. Bộ nhớ không sạch như bạn tưởng
`zeroize` xóa buffer bạn kiểm soát — nhưng key/password còn có thể nằm ở: **swap/pagefile**, file **hibernation**, **crash dump**, bản copy tạm khi allocator move, buffer của WebView (password đi qua input HTML trước khi tới Rust!). Bài học thật: KeePass CVE-2023-32784 — dựng lại master password từ memory dump dù app có che chắn.
- **Làm gì:** chấp nhận có văn bản (ghi vào threat model — Module 2.5); giảm thiểu: chuyển password từ UI sang Rust sớm nhất có thể, không giữ trong React state; cân nhắc tắt crash dump chứa heap.
- **Tham khảo:** https://nvd.nist.gov/vuln/detail/CVE-2023-32784

### A2. Clipboard là kênh rò số 1
Copy password → mọi process đọc được clipboard; **Windows Clipboard History (Win+V)** lưu lại; **Cloud Clipboard** còn sync lên tài khoản Microsoft sang máy khác!
- **Làm gì:** auto-clear sau N giây; dùng API đánh dấu loại trừ history (`ExcludeClipboardContentFromMonitorProcessing`); cảnh báo user trong docs.
- **Tham khảo:** https://learn.microsoft.com/en-us/windows/win32/dataxchg/clipboard-formats#cloud-clipboard-and-clipboard-history-formats

### A3. Màn hình bị nhìn trộm bởi phần mềm
Spec chống chụp màn hình bằng watermark — nhưng **phần mềm** họp/ghi màn hình (Zoom share, TeamViewer, recorder) thấy password reveal mà watermark vô nghĩa.
- **Làm gì:** reveal mặc định che, giữ-để-xem, tự ẩn nhanh; cân nhắc `SetWindowDisplayAffinity(WDA_EXCLUDEFROMCAPTURE)` trên Windows cho cửa sổ reveal.
- **Tham khảo:** https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowdisplayaffinity

### A4. Kênh phân phối app là bề mặt tấn công
Update feed, dependency (cargo/npm), GitHub token của bạn — chiếm được một trong ba là chiếm được mọi máy user. Đã cover ở Module 7.3–7.4; nhắc lại vì hay bị coi là "chuyện của dự án to".
- **Tham khảo:** SLSA framework — https://slsa.dev/ · RustSec — https://rustsec.org/

### A5. Secret rò qua chính công cụ debug của bạn
Ba đường rò không cần attacker giỏi: (1) `#[derive(Debug)]` trên struct chứa password → một dòng `println!("{:?}")`/log lỗi in secret ra console/file; (2) **WebView devtools bật trong bản release** → xem được payload mọi `invoke()` gồm cả master password; (3) chuỗi lỗi SQL kèm giá trị bind (password nằm trong message lỗi).
- **Làm gì:** struct chứa secret implement `Debug` tay in `"[REDACTED]"` (hoặc dùng crate `secrecy`); kiểm tra devtools chỉ bật qua feature flag dev; quy ước lỗi trả về UI không bao giờ chứa giá trị bind. Viết 1 test grep output log không chứa chuỗi password test.
- **Tham khảo:** crate secrecy — https://docs.rs/secrecy · Tauri debug/devtools — https://v2.tauri.app/develop/debug/

### A6. "Đã xóa" trong SQLite không có nghĩa là biến mất
DELETE một credential chỉ đánh dấu page tự do — **nội dung cũ còn nguyên trong file** cho đến khi bị ghi đè hoặc VACUUM. Trong file mã hóa thì kẻ ngoài không đọc được, nhưng: ai mở được vault (đúng threat model "NV nghỉ từng biết pass cũ") vẫn moi được password đã xóa/đổi từ free pages. Ngoài ra SQLite có thể tạo **file tạm trên đĩa** (sort lớn, vacuum) — với SQLCipher cần ép vào RAM.
- **Làm gì:** bật `PRAGMA secure_delete = ON` (ghi đè page khi xóa) + `PRAGMA temp_store = MEMORY` ngay khi mở connection — 2 dòng, làm từ 0.1.1; VACUUM định kỳ khi đóng vault.
- **Tham khảo:** https://www.sqlite.org/pragma.html#pragma_secure_delete · file tạm SQLite: https://www.sqlite.org/tempfiles.html

## B. Dữ liệu & thời gian

### B1. Backup chưa restore thử = chưa có backup
Ai cũng viết code backup; rất ít người **diễn tập restore**. File backup có thể hỏng âm thầm (đĩa lỗi, copy dở).
- **Làm gì:** quy trình "restore drill" mỗi quý ghi vào runbook; backup xong verify mở được (SQLCipher key check + `PRAGMA integrity_check`).

### B2. Đồng hồ máy là input không đáng tin
Tính năng đinh của app là **cảnh báo hết hạn** — hoàn toàn phụ thuộc đồng hồ máy. Máy CEO lệch giờ/timezone sai → cảnh báo sai theo hướng nguy hiểm (đỏ thành xanh).
- **Làm gì:** so `now` với `last_synced_at`/`updated_at` — nếu "now" sớm hơn dữ liệu, hiện cảnh báo đồng hồ; luôn hiển thị ngày hết hạn tuyệt đối cạnh nhãn đỏ/vàng/xanh.

### B3. OneDrive/Documents redirection trên Windows
Windows hay redirect `Documents`/`Desktop` vào OneDrive — nếu `~/.vipavault/` rơi vào vùng đó, bạn vô tình có "file DB sống trên sync folder" (rủi ro corruption — Module 6.1) mà không hề chọn.
- **Làm gì:** chọn thư mục qua `dirs::data_local_dir()` (LocalAppData — không roam, không sync) một cách **có chủ đích**, ghi thành ADR.
- **Tham khảo:** https://docs.rs/dirs · https://www.sqlite.org/howtocorrupt.html

### B4. Chữ và số không "chỉ là text"
Đã gặp trong review: LIKE không case-insensitive với tiếng Việt; thêm: chuẩn hóa Unicode (NFC/NFD — "ế" gõ 2 cách khác byte nhau → search miss), so sánh email case, độ dài password nhiều byte.
- **Làm gì:** normalize NFC mọi text trước khi lưu; test search với dữ liệu tiếng Việt có dấu ngay từ 0.2.0.
- **Tham khảo:** https://unicode.org/reports/tr15/

### B5. Copy file `.hvault` khi vault đang mở = snapshot có thể hỏng
WAL mode nghĩa là dữ liệu nằm ở **2 file** (`.hvault` + `.hvault-wal`) khi đang mở. Copy mỗi file chính giữa phiên làm việc → snapshot thiếu transaction chưa checkpoint, mở ra có thể hỏng hoặc mất dữ liệu mới nhất. Đây là bug backup âm thầm — chỉ lộ đúng hôm cần restore.
- **Làm gì:** backup bằng cơ chế của SQLite, không phải file copy: `VACUUM INTO 'backup.hvault'` (một lệnh, snapshot nhất quán, còn tiện remove free pages) hoặc Online Backup API; nếu bắt buộc copy file thì checkpoint + đóng connection trước.
- **Tham khảo:** VACUUM INTO — https://www.sqlite.org/lang_vacuum.html#vacuuminto · Backup API — https://www.sqlite.org/backup.html

### B6. KDF agility — tham số Argon2id hôm nay là nợ của 5 năm sau
Nếu 64MB/3-iter bị **hard-code**, bạn vĩnh viễn không nâng được độ khó KDF (phần cứng attacker mạnh lên mỗi năm) mà không phá vault cũ. Tương tự: SQLCipher đổi major version là đổi format file (vụ SQLCipher 3→4 là nỗi đau kinh điển của cả hệ sinh thái).
- **Làm gì:** lưu **tham số KDF + version format** vào metadata plaintext đi kèm vault (header riêng hoặc `profiles.json`) ngay từ 0.1.1 — mở vault đọc tham số từ metadata, không từ hằng số; giữ **bộ fixture vault** tạo bởi mỗi version đã release làm test tương thích chạy mãi mãi; biết trước tồn tại `PRAGMA cipher_migrate` cho ngày nâng SQLCipher.
- **Tham khảo:** cipher_migrate — https://www.zetetic.net/sqlcipher/sqlcipher-api/#cipher_migrate · bài học 3→4: https://www.zetetic.net/blog/2018/11/30/sqlcipher-400-release/

### B7. Antivirus/EDR là "process khác" luôn chạm vào file của bạn
AV quét/khóa `.hvault` giữa lúc ghi → lỗi IO lạ, chậm bất thường, thậm chí góp phần corruption; app Rust không ký số còn hay bị AV cách ly thẳng file exe (liên quan Module 7.2). Máy SME VN thường cài AV mạnh tay.
- **Làm gì:** mọi thao tác ghi có retry ngắn cho lỗi sharing-violation trên Windows; docs hướng dẫn thêm exclusion thư mục `~/.vipavault/`; test trên máy có AV thật trước khi phát cho user thứ 2.

## C. Pháp lý & con người

### C1. Nghị định 13/2023/NĐ-CP — bạn đang lưu dữ liệu cá nhân nhân viên
`email_accounts` có `display_name`, `department` — là **dữ liệu cá nhân** theo pháp luật VN. Dùng nội bộ 1 công ty thì nhẹ; nhưng nếu bán cho công ty khác, "công cụ xử lý dữ liệu cá nhân" kéo theo nghĩa vụ mới (và là câu khách hàng doanh nghiệp sẽ hỏi).
- **Làm gì:** V1 nội bộ — ghi nhận rủi ro là đủ; trước khi bán: đọc nghị định + thêm mục privacy trong docs sản phẩm.
- **Tham khảo:** Nghị định 13/2023/NĐ-CP về bảo vệ dữ liệu cá nhân — https://vanban.chinhphu.vn/ (tra "13/2023/NĐ-CP"); tổng quan tiếng Anh: https://iapp.org/news/a/a-look-at-vietnams-new-personal-data-protection-decree/

### C2. Bus factor = 1, cho chính két sắt của công ty
Bạn là người duy nhất hiểu app **và** giữ master password của toàn bộ credential công ty. Bạn ốm 2 tuần / rời công ty → ai mở vault, ai sửa app hỏng?
- **Làm gì:** Emergency Kit cất két công ty (không phải két của riêng bạn); runbook "vận hành không có tôi" 1 trang; docs đủ để một dev khác build được từ repo (chính giáo trình này là một phần câu trả lời).

### C3. Hỗ trợ là chi phí thật khi có user thứ 2
Mỗi user ngoài bạn = câu hỏi cài đặt, máy lạ, "app không mở được". Với app credential, một câu trả lời hỗ trợ sai (vd hướng dẫn xóa nhầm file) là thảm họa.
- **Làm gì:** trước khi trao cho user thứ 2: FAQ, quy trình chẩn đoán "không mở được vault" (phân biệt sai pass / file hỏng / file thiếu), và chính sách hỗ trợ (giờ nào, kênh nào) — xem BUSINESS_MODEL §4.

## D. Trải nghiệm bị bỏ quên

### D1. Onboarding máy mới / chuyển máy
Vòng đời thật có ngày "đổi laptop". Không có flow chuyển máy → user tự copy sai file, thiếu `app_settings.json`, mất Emergency Kit.
- **Làm gì:** mục "Chuyển máy" trong docs (checklist file nào đi cùng); về sau: export/import có kiểm tra.

### D2. Uninstall và data retention
Gỡ app: `.hvault` nên ở lại (dữ liệu quý) — nhưng phải **nói cho user biết** nó ở lại và ở đâu, kẻo họ tưởng đã xóa sạch credential khi thanh lý máy.
- **Làm gì:** trang docs "Gỡ cài đặt & xóa dữ liệu hoàn toàn" (gồm secure-delete lưu ý: xóa file thường không xóa nội dung khỏi đĩa — với SSD, xóa file mã hóa là đủ an toàn thực dụng vì không có key).

### D3. Accessibility & bàn phím
App dùng hằng ngày bởi IT → tốc độ bàn phím quan trọng: unlock bằng Enter, quick-search toàn cục, không bắt dùng chuột. Accessibility (focus ring, contrast, screen reader labels) là bonus rẻ khi làm sớm, đắt khi vá sau.
- **Tham khảo:** WAI keyboard patterns — https://www.w3.org/WAI/ARIA/apg/patterns/

### D4. Hiệu năng trên máy tệ nhất, không phải máy bạn
Argon2id 64MB + WebView + React trên máy văn phòng 4GB RAM cũ — unlock có thể mất nhiều giây. Nếu unlock chậm gây bực → user để vault mở cả ngày → **phá luôn mô hình auto-lock**. Hiệu năng ở đây là *tính năng bảo mật*.
- **Làm gì:** đo unlock-time trên máy yếu nhất công ty ngay ở 0.1.1; spinner + thông điệp trung thực ("đang giải mã — vài giây").

### D5. Migration dữ liệu VÀO app (ngày 1) — và RA khỏi app (ngày cuối)
Dữ liệu hiện ở Excel/ghi chú — không có import thì "nhập tay 100 dòng" chính là lý do bỏ app từ tuần đầu. Ngược lại, **export toàn bộ ra CSV/JSON** là lời hứa chống lock-in (và là đường thoát nếu dự án dừng).
- **Làm gì:** import CSV tối thiểu trước mốc dogfood 0.6.0; export đầy đủ trước khi có user thứ 2. (Export chứa plaintext secret → chỉ admin, có cảnh báo, ghi audit.)

### D6. UniKey/Telex biến password thành thứ khác
User VN gõ master password khi **bộ gõ tiếng Việt đang bật**: Telex biến `aa` → `â`, `dd` → `đ` — password nhập ra khác password đã đặt, và lỗi "sai mật khẩu" này gần như không thể tự chẩn đoán. Đây là bug hỗ trợ số 1 chờ sẵn cho mọi app có password ở thị trường VN.
- **Làm gì:** không sửa được bộ gõ ngoài app — phòng bằng UX: nút hiện/ẩn password khi gõ (ít nhất ở màn *tạo* vault, nơi sai là thảm họa), nhập 2 lần khi tạo, và dòng nhắc "kiểm tra bộ gõ tiếng Việt" trong thông báo sai mật khẩu sau ≥2 lần thất bại.

### D7. Auto-lock phải nghe hệ điều hành, không chỉ đếm giờ
Idle timer bỏ sót các sự kiện quan trọng hơn: **gập laptop / Win+L / sleep** — máy vào túi xách với vault đang mở trong RAM nếu app không nghe session-lock/suspend của OS. Ngoài ra 2 instance app cùng mở một vault là nguồn lỗi khó hiểu.
- **Làm gì:** lock vault khi nhận sự kiện session lock/suspend của OS (Windows: `WTSRegisterSessionNotification`); dùng plugin single-instance của Tauri chặn instance thứ 2 từ 0.1.1.
- **Tham khảo:** https://learn.microsoft.com/en-us/windows/win32/api/wtsapi32/nf-wtsapi32-wtsregistersessionnotification · https://v2.tauri.app/plugin/single-instance/

---

## Cách dùng module này

| Thời điểm | Đọc lại mục |
|-----------|-------------|
| Trước 0.1.1 | A1, A5, A6, B3, B6, D4, D6, D7 |
| Trước 0.2.0 | B4, D5 |
| Trước 0.5.0 (credential UI) | A2, A3 |
| Trước dogfood 0.6.0 | B1, B5, D5 (import), D3 |
| Trước 0.10.0 release | A4, B2, B7, C2, C3, D1, D2 + toàn bộ Module 7 |
| Trước khi bán cho công ty khác | B7, C1, C3, D2 |

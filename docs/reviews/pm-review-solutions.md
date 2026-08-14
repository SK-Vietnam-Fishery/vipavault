# Phản biện PM & Giải pháp — VipaVault

> **Ngày:** 2026-07-13 · **Trạng thái:** Đề xuất — chưa phải decision record
>
> Tài liệu này ghi lại các phản biện sản phẩm/kỹ thuật (review góc PM) và **nhiều phương án giải quyết kèm trade-off** cho từng vấn đề. Quyết định chính thức vẫn theo quy trình decision record (`.context/decisions/README.md`). Mục nào đã được human chốt sẽ ghi rõ **✅ ĐÃ CHỐT**.
>
> Nguồn phản biện: session review 2026-07-13. Source of truth sản phẩm: [`../vipavault-spec.md`](../vipavault-spec.md) · [`../technical-decisions.md`](../technical-decisions.md).

---

## Tóm tắt các vấn đề

| # | Vấn đề | Mức độ | Trạng thái |
|---|--------|--------|------------|
| 1 | Viewer CEO persona chưa được kiểm chứng | 🔴 Chiến lược | Đề xuất |
| 2 | Dữ liệu viewer bị đóng băng (copy-file) | 🔴 Chiến lược | Đề xuất |
| 3 | `machine_role` không phải ranh giới bảo mật | 🟡 Threat model | Đề xuất |
| 4 | Không có recovery master password | 🔴 Data loss | Đề xuất |
| 5 | Backup cùng ổ đĩa với file gốc | 🟡 Data loss | Đề xuất |
| 6 | Không có quy trình xoay master password | 🟡 Threat model | Đề xuất |
| 7 | Confuse engine là security theater | 🟡 Scope | **✅ ĐÃ CHỐT: cắt khỏi V1** |
| 8 | Cột tiền dùng `REAL` | 🟡 Schema bug | Đề xuất |
| 9 | Mâu thuẫn định vị: vault offline vs ITAM reporting | 🔴 Chiến lược | Đề xuất |
| 10 | Roadmap 12 milestone trước khi dogfood được | 🟡 Scope | **✅ ĐÃ CHỐT: reorder dogfood-first** |

---

## 1. Viewer CEO persona chưa được kiểm chứng

**Vấn đề:** Trục kiến trúc `machine_role: viewer` + copy `.hvault` + role gates (0.3.0–0.4.0) xây trên giả định *CEO sẽ mở desktop app để xem dashboard*. Kinh nghiệm SME: CEO thường không cài tool — họ hỏi IT hoặc muốn báo cáo qua email.

### Phương án

| PA | Mô tả | Ưu | Nhược |
|----|-------|-----|-------|
| **(a) PDF/email digest thay viewer app** | Kéo "Export PDF báo cáo tháng" (Phase 3) lên sớm; IT gửi cho CEO hàng tháng | Cắt được phần lớn scope 0.3.0 role gates + toàn bộ quy trình copy file; khớp hành vi CEO thật | Mất tính "tự phục vụ"; CEO không xem được real-time |
| **(b) Test giả định trước khi build** | Đưa CEO một PDF mock dashboard, hỏi: "muốn nhận qua email tháng hay tự mở app?" — làm **trước 0.3.0** | Chi phí ~1 giờ, quyết định 2–3 milestone | Cần CEO thật tham gia |
| **(c) Hybrid** | Giữ viewer app + thêm nút Export PDF cho IT gửi | Phủ cả hai hành vi | Tốn cả hai chi phí; nguy cơ build viewer không ai dùng |

**Khuyến nghị:** Làm **(b)** ngay (chi phí gần 0), kết quả quyết định giữa (a) và (c). Không build role gates 0.3.0 trước khi có câu trả lời.

**Tham khảo:**
- Jobs to be Done framework — https://jtbd.info/ (phỏng vấn theo "job" thay vì feature)
- The Mom Test (Rob Fitzpatrick) — https://www.momtestbook.com/ (cách hỏi user không bị trả lời xã giao)
- 1Password Insights/Reports — https://support.1password.com/insights/ (ví dụ dashboard-as-report cho người không phải IT)

---

## 2. Dữ liệu viewer bị đóng băng khi copy file

**Vấn đề:** Giá trị chính của viewer là **cảnh báo hết hạn** (đỏ/vàng/xanh). File `.hvault` copy sang máy CEO đóng băng tại thời điểm copy — cảnh báo trên dữ liệu cũ 2 tháng **tệ hơn không có cảnh báo** (an toàn giả). Spec chưa định nghĩa: ai copy, tần suất, quy trình.

### Phương án

| PA | Mô tả | Ưu | Nhược |
|----|-------|-----|-------|
| **(a) Banner tuổi dữ liệu** *(tối thiểu bắt buộc)* | Viewer UI hiển thị nổi bật "Dữ liệu tại ngày DD/MM" + cảnh báo đỏ khi file cũ hơn N ngày (đề xuất N=14, so với ngưỡng alert 7 ngày) | Rẻ (~1 ngày dev); loại bỏ an toàn giả | Không giải quyết gốc — dữ liệu vẫn cũ |
| **(b) Sync folder làm kênh copy** | Đặt `.hvault` trong thư mục Syncthing / Google Drive / OneDrive; file mã hóa toàn phần nên cloud không đọc được nội dung | Dữ liệu tươi gần real-time; không cần code | SQLite/SQLCipher + file sync có rủi ro conflict/corruption nếu 2 máy mở cùng lúc — cần quy ước "viewer chỉ mở bản copy, không mở file sync trực tiếp" hoặc WAL off khi sync |
| **(c) Hoãn viewer đến FULL Share Package** | V1 không có viewer; CEO nhận PDF (PA 1a) | Đơn giản nhất | Phụ thuộc quyết định vấn đề #1 |

**Khuyến nghị:** **(a)** là bắt buộc bất kể chọn gì — đưa vào scope 0.4.0 (Dashboard). Nếu viewer sống sót qua test #1(b), hướng dẫn **(b)** trong docs với cảnh báo rõ về concurrent access.

**Lưu ý kỹ thuật cho (b):** SQLite khuyến cáo không đặt DB đang mở trên file-sync/network folder (rủi ro corruption). Pattern an toàn: máy admin export bản snapshot vào sync folder (file copy đóng), máy viewer mở bản copy read-only.

**Tham khảo:**
- SQLite: How To Corrupt An SQLite Database File §"filesystem sync" — https://www.sqlite.org/howtocorrupt.html
- Syncthing docs — https://docs.syncthing.net/
- Spec §2 cấu trúc file — [`../vipavault-spec.md`](../vipavault-spec.md)

---

## 3. `machine_role` là tiện ích UX, không phải ranh giới bảo mật

**Vấn đề:** Máy viewer có **toàn bộ** `.hvault` và CEO biết **master password** để mở. Sửa `app_settings.json` (plaintext) từ `viewer` → `admin` là có full quyền. Nếu docs/dev/agent tưởng nhầm đây là security boundary, sẽ đưa ra quyết định sai (vd: "viewer không thấy được password nên không cần lo").

### Phương án

| PA | Mô tả | Ưu | Nhược |
|----|-------|-----|-------|
| **(a) Ghi thẳng vào threat model** *(khuyến nghị)* | Thêm vào spec §3 và technical-decisions: *"Trust boundary duy nhất = master password. `machine_role` là UX convenience — người có file + password là có toàn bộ dữ liệu, bất kể role."* | Chi phí 0; ngăn quyết định sai về sau | Không thay đổi thực tế bảo mật |
| **(b) FULL Share Package là ranh giới thật** | Subset export với key riêng (đã planned post-0.10.0) — viewer chỉ nhận dữ liệu được chọn | Boundary thật sự | Đúng như kế hoạch FULL; không phải việc V1 |
| **(c) Signed settings / khóa role** | Ký `app_settings.json`, chống sửa tay | — | **Không khuyến nghị**: security theater tầng 2 — kẻ có file + password vẫn đọc được mọi thứ bằng tool SQLCipher bất kỳ; thêm phức tạp vô ích cho SME |

**Khuyến nghị:** **(a)** ngay (1 đoạn văn trong spec), **(b)** giữ nguyên lịch FULL, loại **(c)**.

**Tham khảo:**
- OWASP Threat Modeling — https://owasp.org/www-community/Threat_Modeling
- "Trust boundary" concept — https://en.wikipedia.org/wiki/Trust_boundary
- `.context/planning/MILESTONE_QUESTIONNAIRE.md` §Auth, §FULL

---

## 4. Không có recovery master password

**Vấn đề:** Spec chấp nhận "quên master pass = mất hết, document kỹ khi setup". Đúng về crypto, nhưng với SME đây là **sự kiện mất dữ liệu số 1** và là ticket hỗ trợ đắt nhất nếu sau này bán sản phẩm.

### Phương án

| PA | Mô tả | Ưu | Nhược |
|----|-------|-----|-------|
| **(a) Emergency Kit in được** *(khuyến nghị)* | Khi tạo vault, sinh PDF/trang in: tên vault, đường dẫn file, ngày tạo, **ô trống viết tay** master password, hướng dẫn cất két/tủ khóa. Mô hình 1Password đã chứng minh | ~1 ngày dev; giảm mạnh xác suất mất vĩnh viễn; không đụng crypto | Phụ thuộc kỷ luật người dùng (in ra và cất) |
| **(b) Recovery key thứ hai** | Sinh key ngẫu nhiên 2nd, mã hóa DB key bằng cả 2 (envelope encryption); recovery key in ra cất két | Recovery thật sự, không cần nhớ pass | Tăng phức tạp vault engine 0.1.1 đáng kể; thêm bề mặt tấn công; nên để Phase 2 nếu cần |
| **(c) Giữ nguyên (chỉ document)** | Như spec hiện tại | 0 effort | Rủi ro mất toàn bộ credential công ty vì 1 lần quên |

**Khuyến nghị:** **(a)** vào scope **0.1.1** (đang làm vault create — đúng chỗ). **(b)** ghi backlog Phase 2 nếu có khách trả tiền yêu cầu.

**Tham khảo:**
- 1Password Emergency Kit — https://support.1password.com/emergency-kit/
- Envelope encryption (cho PA b) — https://cloud.google.com/kms/docs/envelope-encryption

---

## 5. Backup nằm cùng ổ đĩa với file gốc

**Vấn đề:** `~/.vipavault/backups/` cùng ổ với file gốc — máy chết/ransomware = mất cả gốc lẫn backup.

### Phương án

| PA | Mô tả | Ưu | Nhược |
|----|-------|-----|-------|
| **(a) Thư mục backup thứ 2 tùy chọn** | Setting cho phép trỏ backup đến ổ khác/USB/NAS; app copy sau mỗi lần đóng vault có thay đổi | Đơn giản; `.hvault` đã mã hóa nên đặt đâu cũng an toàn về confidentiality | User phải có ổ thứ 2 |
| **(b) Nhắc backup định kỳ trong UI** | Banner "Backup gần nhất cách đây X ngày" + nút export | Rẻ nhất | Thủ công, dễ bị lờ |
| **(c) Quy tắc 3-2-1 trong docs** | Document: 3 bản, 2 loại media, 1 off-site (cloud folder OK vì file đã mã hóa) | 0 dev | Chỉ là hướng dẫn |

**Khuyến nghị:** **(b) + (c)** trước 0.10.0 release; **(a)** khi có thời gian (nhỏ, có thể gộp vào 0.4.0 hoặc hardening).

**Tham khảo:**
- 3-2-1 backup rule (CISA) — https://www.cisa.gov/sites/default/files/publications/data_backup_options.pdf
- Backblaze 3-2-1 — https://www.backblaze.com/blog/the-3-2-1-backup-strategy/

---

## 6. Không có quy trình xoay master password (IT/CEO nghỉ việc)

**Vấn đề:** IT và CEO dùng **chung một master password** — không có revocation. Bảng rủi ro spec có "NV nghỉ → đổi rule confuse" nhưng thiếu kịch bản nghiêm trọng hơn: *người biết master password rời công ty*.

### Phương án

| PA | Mô tả | Ưu | Nhược |
|----|-------|-----|-------|
| **(a) Runbook rotation bằng `PRAGMA rekey`** *(khuyến nghị)* | SQLCipher hỗ trợ rekey tại chỗ. Quy trình: đổi master pass → rekey file → **copy lại file mới cho mọi máy** → hủy các bản backup cũ theo lịch. Ghi thành mục trong spec §3 Rủi ro | Dùng tính năng có sẵn của SQLCipher; chỉ là docs + 1 command trong app | Bản copy/backup cũ vẫn mở được bằng pass cũ — phải nêu rõ trong runbook |
| **(b) Nút "Đổi master password" trong app** | UI wrapper cho (a): nhập pass cũ + mới → Argon2id mới salt mới → rekey | Không cần thao tác kỹ thuật | Thêm scope 0.1.1 hoặc hardening |
| **(c) Không làm gì** | — | — | Lỗ hổng threat model lớn nhất còn lại |

**Khuyến nghị:** **(a)** viết ngay vào spec (docs-only); **(b)** đưa vào 0.10.0 hardening — đây là tính năng bắt buộc trước khi cho công ty khác dùng.

**Tham khảo:**
- SQLCipher API `PRAGMA rekey` — https://www.zetetic.net/sqlcipher/sqlcipher-api/#rekey
- NIST SP 800-63B §5.1.1 (memorized secrets, rotation khi compromise) — https://pages.nist.gov/800-63-3/sp800-63b.html

---

## 7. Confuse engine — ✅ ĐÃ CHỐT: cắt khỏi V1 (2026-07-13)

**Vấn đề:** Confuse (prefix/suffix rule công ty khi gửi password qua Zalo) là security theater với chi phí thật (milestone 0.10.0 riêng, cột audit riêng, quy trình đổi rule):

- Rule là **bí mật tĩnh dùng chung toàn công ty** — sau người nhận thứ 2, không còn là bí mật.
- Threat model không rõ: người đọc trộm Zalo cũng đọc được dòng hướng dẫn "(bỏ 5 đầu 7 cuối)" gửi kèm.
- Lớp bảo vệ thật đã tồn tại: `must_change_password = 1` — password gửi đi chỉ sống đến lần login đầu.

**Quyết định (human, 2026-07-13):** **Cắt hẳn khỏi V1.** Milestone "Confuse & Notification" chuyển vào backlog Phase 2+ trong `MILESTONES_REFERENCE.md`, kèm điều kiện: *threat model phải được làm rõ trước khi implement*.

**Phương án thay thế khi Phase 2+ mở lại nhu cầu "gửi password an toàn":**

| PA | Mô tả | Tham khảo |
|----|-------|-----------|
| One-time secret link (self-host) | Password nằm sau link tự hủy sau 1 lần xem/TTL — không đi qua Zalo dạng text | https://github.com/onetimesecret/onetimesecret |
| Chỉ dựa vào `must_change_password` | Gửi pass tạm + ép đổi lần đầu — đã có trong schema V1 | Spec §6 |

**Ghi chú schema:** cột `activity_log.confuse_used` giữ nguyên trong schema V1 (không gây hại, tránh churn migration) nhưng V1 không ghi giá trị vào.

---

## 8. Cột tiền dùng `REAL`

**Vấn đề:** `services.monthly_cost REAL` và `subscription_licenses.cost_per_seat REAL` — float cho tiền là bug kinh điển (0.1 + 0.2 ≠ 0.3; cộng dồn báo cáo tháng sẽ lệch). Phải sửa **trước khi 0.2.0 đóng băng migration** — sau đó chi phí đổi tăng gấp nhiều lần.

### Phương án

| PA | Mô tả | Ưu | Nhược |
|----|-------|-----|-------|
| **(a) INTEGER đơn vị nhỏ nhất** *(khuyến nghị)* | `monthly_cost_vnd INTEGER` (VND không có xu — lưu nguyên đồng); nếu đa tiền tệ sau này: lưu minor units + cột `currency` | Chuẩn ngành; SQLite INTEGER 64-bit dư sức | Frontend phải format |
| **(b) TEXT decimal** | Lưu "1200000.00" dạng chuỗi, parse bằng decimal lib | Chính xác tùy ý | Không SUM được trực tiếp trong SQL |
| **(c) Giữ REAL** | — | — | Sai số tích lũy trong báo cáo chi phí — đúng tính năng bán cho CEO |

**Khuyến nghị:** **(a)** — sửa spec §4 schema ngay (docs-only, migration chưa viết), kèm quy ước: mọi cột tiền mới đều INTEGER minor units.

**Tham khảo:**
- Martin Fowler — Money pattern — https://martinfowler.com/eaaCatalog/money.html
- SQLite datatypes (REAL là IEEE 754 float) — https://www.sqlite.org/datatype3.html
- Falsehoods programmers believe about prices — https://gist.github.com/rgs/6509585

---

## 9. Mâu thuẫn định vị: vault offline vs ITAM reporting

**Vấn đề:** Mục tiêu phát biểu là "quản lý danh mục phần mềm cho SME" — bài toán ITAM/SaaS-spend (luôn tươi, nhiều người xem, báo cáo chi phí). Nhưng kiến trúc là **vault mã hóa offline, 1 file, 1 password** — chủ động chống lại nhu cầu reporting đa người xem. Phase 3 (license, chi phí, PDF) sẽ đòi lớp chia sẻ mà copy-tay không gánh nổi.

### Phương án

| PA | Mô tả | Hệ quả kiến trúc |
|----|-------|------------------|
| **(a) Chốt: vault + ops cho chính công ty** *(khuyến nghị cho hiện tại)* | V1–V2 phục vụ đúng use case gốc (IT 1 người + CEO); ITAM là byproduct, không phải promise | Giữ nguyên mọi thứ; Phase 3 = báo cáo PDF từ dữ liệu local, không cần server |
| **(b) Pivot ITAM/SaaS-spend thật** | Cạnh tranh mini-Torii/Zluri cho SME VN | Cần server/sync layer — mâu thuẫn USP offline; gần như sản phẩm khác |
| **(c) Hai tầng: vault local + report xuất bản** | Vault giữ offline; báo cáo (không chứa secret) được export/publish riêng cho nhiều người xem | Trung đạo khả thi — nhưng phải thiết kế ranh giới "dữ liệu nào rời vault" từ Phase 3 |

**Khuyến nghị:** Chốt **(a)** thành một mục trong `technical-decisions.md` (qua quy trình decision record) để Phase 3 không phải trả nợ kiến trúc. Đánh giá lại (b)/(c) chỉ khi `.local/BUSINESS_MODEL.md` §8 được điền và chọn hướng B/C.

**Tham khảo:**
- Torii (SaaS management) — https://www.toriihq.com/
- Zluri — https://www.zluri.com/
- `.local/BUSINESS_MODEL.md` §2.2, §5 (phân khúc thị trường, positioning)

---

## 10. Roadmap — ✅ ĐÃ CHỐT: reorder dogfood-first (2026-07-13)

**Vấn đề:** 12 milestone trước MVP cho deployment 1 công ty là quá dài. Câu hỏi PM: *phiên bản sớm nhất mà chính bạn dùng hàng ngày là gì?* — đó là vault + nhập tay credential + dashboard cảnh báo, không phải provider sync.

**Quyết định (human, 2026-07-13):**
- Reorder `MILESTONES_REFERENCE.md` theo nguyên tắc **dogfood sớm nhất có thể** — mốc dogfood tại **0.6.0 (Email Accounts Local)**: từ đây app dùng được hàng ngày với dữ liệu nhập tay.
- Provider integration (Routing → Sync → Apply) dồn về sau mốc dogfood — chỉ build tiếp nếu sau ≥1 tháng dùng thật vẫn thấy đau ở khâu thao tác cPanel tay.
- Confuse cắt khỏi V1 (xem §7) → chuỗi V1 còn 11 milestone, release tại **0.10.0**.

Chi tiết thứ tự mới: [`.context/planning/MILESTONES_REFERENCE.md`](../../.context/planning/MILESTONES_REFERENCE.md).

---

## Việc cần làm ngay (tổng hợp)

| Việc | Deadline logic | Gắn vào | Chi phí |
|------|----------------|---------|---------|
| Test viewer persona bằng PDF mock (§1b) | Trước khi build 0.3.0 role gates | Việc human, không phải code | ~1 giờ |
| Đổi cột tiền sang INTEGER trong spec §4 (§8a) | Trước khi 0.2.0 viết migration | Sửa `vipavault-spec.md` | ~30 phút |
| Emergency Kit (§4a) | Gộp vào 0.1.1 (đang làm vault create) | Scope 0.1.1 | ~1 ngày |
| Ghi "trust boundary = master password" (§3a) | Sớm — trước khi agent code role logic | Sửa spec §3 + technical-decisions | ~30 phút |
| Runbook rotation `PRAGMA rekey` (§6a) | Trước 0.10.0 release | Sửa spec §3 Rủi ro | ~1 giờ |
| Banner tuổi dữ liệu viewer (§2a) | Nếu viewer sống sót qua test §1b | Scope Dashboard milestone | ~1 ngày |
| Chốt định vị (a) vault+ops (§9) | Trước Phase 3 | Decision record mới | Quy trình decision |

---

*Tài liệu này là đề xuất từ review PM — không thay thế spec. Khi một mục được chốt, chuyển thành decision record theo `.context/decisions/README.md` và cập nhật trạng thái tại bảng đầu file.*

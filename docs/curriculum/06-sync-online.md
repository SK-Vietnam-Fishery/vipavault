# Module 6 — Sync Online: 6 hướng, pros & cons

> **Mục tiêu:** Khi VipaVault cần vượt qua "copy file bằng USB" — nhiều máy, nhiều người, dữ liệu tươi — có **6 hướng kiến trúc**. Module này phân tích từng hướng để bạn chọn có căn cứ, vì đây là **quyết định một chiều đắt nhất** sau lựa chọn SQLCipher: nó thay đổi threat model, mô hình key, và cả mô hình kinh doanh (xem `.local/BUSINESS_MODEL.md`).
>
> **Thời lượng:** 1–2 tuần đọc + 1 design doc. **Không code trong module này** — thiết kế trước Phase 2.

---

## 6.0 Đặt vấn đề đúng trước

Ba nhu cầu khác nhau thường bị gộp làm một — tách ra trước khi chọn:

| Nhu cầu | Ai cần | Độ khó |
|---------|--------|--------|
| N1. **Backup off-site** | Mọi user, ngay từ V1 | Thấp — file mã hóa, đẩy đi đâu cũng được |
| N2. **Multi-device, 1 người dùng** (admin có 2 máy) | Sớm | Trung bình — conflict hiếm nhưng có |
| N3. **Multi-user đồng thời** (2 IT cùng sửa, CEO xem tươi) | Muộn / có thể không bao giờ | Cao — conflict là chuyện thường ngày |

Nhiều hướng dưới đây giải N1+N2 rẻ; chỉ vài hướng giải nổi N3. **Đừng trả giá kiến trúc N3 khi nhu cầu thật là N1.**

---

## 6.1 Hướng 1 — File snapshot qua sync folder (Syncthing / Drive / OneDrive)

Máy admin export snapshot `.hvault` (bản copy đóng, không phải file đang mở) vào thư mục sync; máy khác mở read-only.

| | |
|---|---|
| **Pros** | Không viết server, không đổi threat model (file luôn mã hóa); Syncthing còn là P2P E2EE sẵn; giải N1 + N2-đọc gần miễn phí |
| **Cons** | Single-writer — chỉ 1 máy được sửa; SQLite trên folder đang sync có rủi ro corruption nếu làm sai (phải sync snapshot đóng, không sync file sống); dữ liệu tươi mức "phút", không real-time; không giải N3 |
| **Đổi threat model?** | Gần như không — thêm bên giữ ciphertext (Google/MS thấy file mã hóa) |
| **Hợp khi** | Ngay sau V1 — chính là mitigation pm-review §2b |

**Tài liệu:** SQLite How To Corrupt §sync — https://www.sqlite.org/howtocorrupt.html · Syncthing — https://docs.syncthing.net/ · Syncthing BEP protocol (đọc thêm P2P): https://docs.syncthing.net/specs/bep-v1.html

## 6.2 Hướng 2 — Central server truyền thống (client–server API)

Server (self-host/cloud) giữ DB, app trở thành client gọi API; server **thấy plaintext** (bảo vệ bằng TLS + auth + disk encryption phía server). Mô hình của Hudu/IT Glue.

| | |
|---|---|
| **Pros** | Giải trọn N1–N3; query/report phía server dễ; nhiều user + phân quyền thật (role enforce ở server — sửa được điểm yếu machine_role!); dễ làm web viewer cho CEO |
| **Cons** | **Phá USP offline-first + zero-knowledge** — server thành két vàng, bị hack là mất tất cả của mọi khách; bạn thành người vận hành hạ tầng 24/7 (on-call, patch, backup server); chi phí dev lớn nhất trong 6 hướng; AGPL §13 kích hoạt nếu modify + network service |
| **Đổi threat model?** | Đảo ngược hoàn toàn — từ "tin file mã hóa" sang "tin server + người vận hành server" |
| **Hợp khi** | Chỉ khi pivot hẳn sang SaaS ITAM (BUSINESS_MODEL hướng C mạnh) — với định vị hiện tại, **gần như là anti-pattern** |

**Tài liệu:** So sánh mô hình: Hudu — https://www.hudu.com/ · AGPL §13 — https://www.gnu.org/licenses/agpl-3.0.html#section13

## 6.3 Hướng 3 — E2EE blob sync server (mô hình Bitwarden)

Server chỉ giữ **ciphertext**; mọi mã hóa/giải mã ở client; server không bao giờ có key. Bitwarden/Vaultwarden là mẫu đã kiểm chứng, có whitepaper mô tả đầy đủ.

| | |
|---|---|
| **Pros** | Giữ zero-knowledge (server bị hack chỉ lộ ciphertext); giải N1–N2 tốt, N3 ở mức per-record; có mẫu tham chiếu trưởng thành (Bitwarden whitepaper + Vaultwarden source Rust để đọc!); nếu định bán sản phẩm, đây là câu chuyện bảo mật kể được |
| **Cons** | Phải chẻ vault thành đơn vị sync (per-record ciphertext) — **rời bỏ mô hình "1 file SQLCipher"**, viết lại tầng storage; key hierarchy phức tạp (master key → key wrapping → per-record); conflict resolution tự xử; server dù "chỉ blob" vẫn phải vận hành + auth |
| **Đổi threat model?** | Ít về confidentiality; thêm bề mặt: server auth, phía metadata (server thấy *số lượng, thời điểm sửa* dù không thấy nội dung) |
| **Hợp khi** | Phase 2+ nếu multi-user thành yêu cầu thật và muốn giữ USP bảo mật |

**Tài liệu:** Bitwarden Security Whitepaper — https://bitwarden.com/help/bitwarden-security-white-paper/ · Vaultwarden (server Rust, AGPL — đọc source học kiến trúc): https://github.com/dani-garcia/vaultwarden

## 6.4 Hướng 4 — SQLite replication layer (Litestream / LiteFS / Turso)

Tận dụng hệ sinh thái replicate SQLite: Litestream stream WAL lên S3 (backup liên tục); LiteFS replicate qua FUSE; Turso embedded replicas.

| | |
|---|---|
| **Pros** | Giữ nguyên SQLite; Litestream giải **N1 xuất sắc** (point-in-time restore) với effort thấp; không đổi code app |
| **Cons** | Đây là công cụ **server-side single-writer** — không thiết kế cho desktop nhiều máy ngang hàng; LiteFS cần FUSE (không hợp desktop Windows); SQLCipher + WAL streaming cần kiểm chứng (page đã mã hóa — stream được nhưng restore phải nguyên vẹn cả chuỗi); vẫn không giải N3 |
| **Đổi threat model?** | Không nếu stream ciphertext lên storage mình kiểm soát |
| **Hợp khi** | Nghiêm túc hóa **backup** (N1) cho máy admin — một dạng nâng cấp của pm-review §5, không phải giải pháp multi-user |

**Tài liệu:** Litestream — https://litestream.io/how-it-works/ · LiteFS — https://fly.io/docs/litefs/ · Turso embedded replicas — https://docs.turso.tech/features/embedded-replicas/introduction

## 6.5 Hướng 5 — CRDT / local-first (cr-sqlite, Automerge)

Mỗi máy ghi tự do offline; thay đổi merge tự động không cần server trung tâm (server nếu có chỉ là relay). CRDT bảo đảm mọi máy hội tụ về cùng trạng thái.

| | |
|---|---|
| **Pros** | Giải N2 + N3 **đúng bản chất** mà vẫn offline-first — triết lý khớp VipaVault nhất về lý thuyết; cr-sqlite làm ngay trên SQLite (extension biến bảng thành CRR); không bắt buộc server tin cậy |
| **Cons** | Công nghệ trẻ nhất trong 6 hướng — cr-sqlite chưa ở độ chín của SQLCipher; **tương thích cr-sqlite × SQLCipher phải tự kiểm chứng** (cả hai đều là extension/fork của SQLite); ngữ nghĩa merge phải thiết kế từng bảng (2 máy cùng đổi password 1 email → last-writer-wins theo đồng hồ nào? đồng hồ máy sai thì sao?); merge "tự động" của dữ liệu credential có thể nguy hiểm hơn conflict báo lỗi to |
| **Đổi threat model?** | Thêm: lịch sử thay đổi (oplog) cũng là dữ liệu nhạy cảm phải mã hóa; relay thấy metadata |
| **Hợp khi** | Đọc để hiểu tương lai; chỉ chọn khi N3 là thật **và** hướng 3 đã thử thấy quá nặng — hoặc chờ hệ sinh thái chín thêm |

**Tài liệu:** "Local-first software" (Ink & Switch — bài nền tảng, đọc bắt buộc): https://www.inkandswitch.com/local-first/ · cr-sqlite: https://github.com/vlcn-io/cr-sqlite · Automerge: https://automerge.org/ · CRDT tổng quan: https://crdt.tech/

## 6.6 Hướng 6 — P2P thuần (device pairing, không server)

Các máy pair trực tiếp (mã QR/passphrase), đồng bộ qua LAN hoặc relay công cộng. Syncthing là hiện thân ở mức file; ở mức record là P2P + CRDT (hướng 5 không server).

| | |
|---|---|
| **Pros** | Không hạ tầng để vận hành/bị hack; khớp triệt để triết lý offline |
| **Cons** | Hai máy phải cùng online mới sync (hoặc cần relay ≈ server tối giản); NAT traversal là nỗi đau kinh điển; discovery + pairing UX khó làm đúng; debug sự cố sync ở nhà khách hàng cực khổ |
| **Hợp khi** | N2 (2 máy 1 admin) trong cùng văn phòng — nhưng hướng 1 (Syncthing làm hộ P2P) đạt 90% giá trị với 5% công sức |

**Tài liệu:** Syncthing BEP — https://docs.syncthing.net/specs/bep-v1.html · libp2p (nếu muốn hiểu sâu): https://docs.libp2p.io/

---

## 6.7 Bảng tổng so sánh

| Hướng | N1 backup | N2 multi-device | N3 multi-user | Giữ zero-knowledge | Giữ "1 file SQLCipher" | Effort | Vận hành server |
|-------|-----------|------------------|---------------|--------------------|------------------------|--------|-----------------|
| 1. File snapshot sync | ✓ | ✓ (đọc) | ✗ | ✓ | ✓ | ★ | Không |
| 2. Central server | ✓ | ✓ | ✓✓ | ✗ | ✗ | ★★★★ | Nặng |
| 3. E2EE blob server | ✓ | ✓✓ | ✓ | ✓ | ✗ | ★★★★ | Vừa |
| 4. SQLite replication | ✓✓ | ~ | ✗ | ✓ | ✓ | ★★ | Nhẹ (S3) |
| 5. CRDT local-first | ~ | ✓✓ | ✓✓ | ✓ (tự xử) | ~ (CRR) | ★★★★★ | Nhẹ (relay) |
| 6. P2P thuần | ✗ | ✓ | ~ | ✓ | ✓ | ★★★★ | Không |

**Đường đi khuyến nghị:** V1 → **Hướng 1** (snapshot + Syncthing, gần như miễn phí, giải luôn pm-review §2/§5) → nếu cần backup mạnh: **Hướng 4** (Litestream cho máy admin) → nếu multi-user thành nhu cầu thật có bằng chứng dogfood: **Hướng 3** (theo mẫu Bitwarden), đọc hướng 5 để biết khi nào đáng nhảy. **Tránh hướng 2** trừ khi pivot business model có chủ đích.

## 6.8 Chủ đề xuyên suốt: key management đa thiết bị/đa người

Mọi hướng từ 3 trở đi buộc trả lời các câu hỏi mà mô hình "1 password mở tất cả" hiện tại né được:

- **Key hierarchy:** master password → wrap **data key**; đổi password chỉ re-wrap, không re-encrypt toàn bộ (đây cũng là cách làm nút rekey nhanh). Đọc: envelope encryption — https://cloud.google.com/kms/docs/envelope-encryption
- **Per-user key:** mỗi người một password riêng, data key được wrap N lần → thu hồi được 1 người mà không đổi password mọi người (giải đúng lỗ hổng rotation pm-review §6!)
- **Device enrollment/revocation:** máy mới vào bằng gì (QR? mã 1 lần?), máy mất thu hồi thế nào.
- **Mẫu tham chiếu:** Bitwarden whitepaper (mục Key Derivation & Encryption) — cùng link ở 6.3; 1Password Security Design white paper — https://1passwordstatic.com/files/security/1password-white-paper.pdf (mô hình 2 secret: password + secret key — đáng đọc).

## 6.9 Deliverable của module

Viết **1 design doc** (`.context/proposals/PROPOSAL_SYNC_DIRECTION.md`, theo format proposal có sẵn trong repo):

1. Nhu cầu thật của bạn hôm nay là N1/N2/N3 nào — kèm bằng chứng từ dogfood.
2. Hướng chọn cho 12 tháng tới + 2 hướng bị loại và vì sao.
3. Threat model thay đổi những gì (delta so với threat-model.md ở Module 2).
4. Điều kiện mở lại quyết định (trigger nào khiến bạn xét lại).

## Câu hỏi phải trả lời được

1. Vì sao "sync file `.hvault` sống qua Dropbox" khác về bản chất với "sync snapshot đóng"?
2. Hướng 3: server thấy được metadata gì dù không giải mã được nội dung? Metadata đó lộ điều gì về công ty?
3. Ở hướng 5, hai máy offline cùng đổi password một email rồi merge — mô tả 2 chiến lược xử lý và rủi ro của mỗi chiến lược với dữ liệu credential.
4. Mô hình per-user key (6.8) sửa được điểm yếu nào trong pm-review §3 và §6? Không sửa được điểm nào?

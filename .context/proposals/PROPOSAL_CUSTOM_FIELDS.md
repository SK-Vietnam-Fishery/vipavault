# Proposal: Custom Fields (trường tự quy định trên credential)

**Status:** DRAFT — chờ human sign-off
**Date:** 2026-07-13
**Author:** brainstorm Human + Claude
**Milestone dự kiến:** ≥ 0.1.1 (sau khi vault core + schema entries ổn định)
**Related:** `DECISION_VAULT_STORAGE.md` (SQLCipher), tension `vault_tier_model`
**Severity:** MEDIUM

---

## 1. Vấn đề

IT cần gắn **các trường tùy biến do người dùng tự đặt** vào từng credential, kiểu như thêm cột trong NocoDB — nhưng **thưa (sparse)**: không phải entry nào cũng dùng mọi trường.

Ví dụ thực tế:

- Nhà cung cấp có nhiều mail server rải rác: `cp73.congty.vn`, `cp193.congty.vn` → muốn ghi rõ `node`, `hosting_ip` cho từng cái.
- MS Office license → key lưu như password, còn cần ghi `version`, ghi chú tự do.
- Email riêng → không cần trường phụ nào.

Yêu cầu: **search được, filter được** theo các trường tùy biến này.

---

## 2. Quyết định thiết kế (đã chốt trong brainstorm)

| # | Quyết định | Lý do |
|---|---|---|
| D1 | **Mọi custom field là text** — không typed (không Number/Date/Select) | Đơn giản, đủ dùng theo kinh nghiệm vận hành |
| D2 | Không có "record type / template" cứng | Field tự do gắn tùy ý vào bất kỳ entry nào |
| D3 | Lưu bằng **1 cột JSON** trên bảng entry, **không** bảng EAV riêng | Schema tự do + không cần validation ⇒ EAV thừa |
| D4 | **Secret KHÔNG bỏ vào JSON** | Password/key vẫn đi đường `secret_enc`; JSON chỉ chứa metadata non-secret, search được |
| D5 | Search/filter bằng **SQLite JSON1** chạy thẳng dưới SQLCipher | Không đổi mô hình mã hóa; DB đã mã hóa toàn file |

Ví dụ giá trị `custom_fields`:

```jsonc
// shared-host / mail server
{ "node": "cp79", "hosting_ip": "1.1.1.1" }

// MS Office license  (key -> secret_enc, KHÔNG nằm ở đây)
{ "version": "Professional 2016", "note": "tự điền" }
```

---

## 3. Ranh giới bảo mật (quan trọng)

- Toàn bộ file `.hvault` đã được **SQLCipher mã hóa** (AES-256 + Argon2id). Khi vault khóa, JSON không đọc được.
- Vì vậy để `custom_fields` ở dạng JSON plaintext **bên trong** DB là chấp nhận được cho **metadata non-secret** (node, ip, version…).
- **Không được** đặt secret (password, license key, API token) vào JSON này — chúng phải đi qua `secret_enc` như hiện tại, để giữ nguyên bất biến của Confuse Engine / activity_log (xem `DECISION_VAULT_STORAGE.md` §4.2e).
- Hệ quả: field trong JSON = **searchable / non-secret**; secret = **giấu, route riêng**.

---

## 4. Schema

```sql
-- cột thêm vào bảng entries (migration)
ALTER TABLE entries ADD COLUMN custom_fields TEXT NOT NULL DEFAULT '{}';

-- index cho các key CỐ ĐỊNH hay lọc (tùy chọn, thêm khi cần)
CREATE INDEX idx_entries_node
  ON entries(json_extract(custom_fields, '$.node'));
```

> `json_extract` / `json_each` là hàm **JSON1** của SQLite — có sẵn dưới SQLCipher vì SQLCipher = SQLite + lớp mã hóa.

---

## 5. Search / Filter

**Filter theo một field cố định:**

```sql
SELECT * FROM entries
WHERE json_extract(custom_fields, '$.node') = 'cp79';
```

**Full-text "gõ congty ra hết" (không cần biết trước tên field):**

```sql
SELECT DISTINCT e.*
FROM entries e, json_each(e.custom_fields) j
WHERE j.value LIKE '%congty%';
```

**Khớp pattern host `cp*.congty.vn`:**

```sql
SELECT DISTINCT e.*
FROM entries e, json_each(e.custom_fields) j
WHERE j.value LIKE 'cp%.congty.vn';
```

### Chiến lược index theo quy mô

| Loại truy vấn | Cách làm | Ghi chú |
|---|---|---|
| Field key cố định, hay lọc (`node`, `hosting_ip`) | Expression index trên `json_extract(...)` | Nhanh |
| Key tự do / full-text | `json_each` + `LIKE`, quét thẳng | Đủ nhanh với ≤ vài nghìn entry — **không tối ưu sớm** |
| Nếu vault rất lớn sau này | FTS5 shadow table | Chỉ thêm khi có bằng chứng chậm |

---

## 6. Phác thảo Rust (mẫu, chưa phải code cuối)

```rust
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};

/// Custom fields = túi key/value text, thưa, do người dùng tự đặt.
/// BTreeMap để key có thứ tự ổn định khi serialize (diff/sync sạch hơn).
pub type CustomFields = BTreeMap<String, String>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    pub id: i64,
    pub vault_id: i64,
    pub title: String,
    pub username: Option<String>,
    // secret (password/key) mã hóa — KHÔNG nằm trong custom_fields
    #[serde(skip_serializing)]
    pub secret_enc: Vec<u8>,
    /// serialize/deserialize sang cột TEXT `custom_fields`
    pub custom_fields: CustomFields,
    pub created_at: i64,
    pub updated_at: i64,
}

/// Bộ lọc cho search_entries.
#[derive(Debug, Default, Deserialize)]
pub struct EntryFilter {
    /// full-text trên mọi value của custom_fields (dùng json_each)
    pub query: Option<String>,
    /// lọc chính xác theo cặp field=value (dùng json_extract)
    pub fields: BTreeMap<String, String>,
}
```

**Tauri command (mẫu):**

```rust
#[tauri::command]
pub fn search_entries(
    vault_id: i64,
    filter: EntryFilter,
    // state: DbConn ...
) -> Result<Vec<Entry>, String> {
    // 1. base: SELECT ... FROM entries e WHERE e.vault_id = ?
    // 2. mỗi (k,v) trong filter.fields:
    //      AND json_extract(e.custom_fields, '$.'||?) = ?
    // 3. nếu filter.query có:
    //      AND EXISTS (SELECT 1 FROM json_each(e.custom_fields) j
    //                  WHERE j.value LIKE '%'||?||'%')
    // 4. deserialize custom_fields (serde_json) -> CustomFields
    //    KHÔNG trả secret_enc ra frontend
    todo!()
}

/// Autocomplete tên field đã dùng — giữ dữ liệu search sạch (xem §8).
#[tauri::command]
pub fn list_field_names(vault_id: i64) -> Result<Vec<String>, String> {
    // SELECT DISTINCT j.key
    // FROM entries e, json_each(e.custom_fields) j
    // WHERE e.vault_id = ? ORDER BY j.key
    todo!()
}
```

---

## 7. Việc cần làm khi implement

1. Migration: thêm cột `custom_fields TEXT NOT NULL DEFAULT '{}'`.
2. Struct `Entry` + `CustomFields` (serde_json cho cột TEXT).
3. Command `search_entries(vault_id, filter)` dựng SQL `json_extract` / `json_each`.
4. Command `list_field_names(vault_id)` cho autocomplete.
5. UI kiểu NocoDB: hàng "+ thêm field" tự do; secret field route riêng qua `secret_enc`.
6. Test: sparse entries, full-text, pattern host, không rò secret ra JSON.

---

## 8. Rủi ro & giảm thiểu

| Rủi ro | Giảm thiểu |
|---|---|
| Key tự do gây loạn tên (`node` vs `Node` vs `máy chủ`) → data khó search | UI **autocomplete tên field đã dùng** (`list_field_names`) để tái dùng tên cũ |
| Ai đó lỡ bỏ secret vào custom_fields | UI tách rõ ô "secret" (route `secret_enc`) vs ô "custom field"; review/lint khi lưu |
| Vault lớn → full-text chậm | Đo trước; chỉ thêm FTS5 khi có bằng chứng |
| Sync diff bẩn do thứ tự key | Dùng `BTreeMap` (key sorted) khi serialize |

---

## 9. Câu hỏi mở / điều kiện nâng cấp

- Có cần **typed field** (Number/Date/Select) trong tương lai? Nếu có → cân nhắc chuyển sang bảng field-definition riêng; hiện **loại bỏ** (D1).
- Có cần **template theo loại entry** không? Hiện **loại bỏ** (D2) — mở lại nếu người dùng phàn nàn phải gõ lại field lặp đi lặp lại.

---

## Fable phản biện

> **Ngày:** 2026-07-13 · **Người review:** Claude (Fable 5) — góc PM + kỹ thuật, đối chiếu spec §4 và `DECISION_VAULT_STORAGE.md`.
> **Kết luận:** Hướng D1–D3, D5 **đúng** — nhưng **chưa sign-off được** vì proposal viết trên schema không tồn tại. Cần sửa 4 điểm dưới đây trước khi approve.

### F1. 🔴 Schema ảo — bảng `entries` và cột `secret_enc` không tồn tại (chặn sign-off)

**Phát hiện:** Grep toàn repo — `CREATE TABLE entries` và `secret_enc` chỉ xuất hiện trong chính file này. Schema V1 thật (spec §4) là `services`, `service_credentials`, `email_accounts`, `domains`, `ssl_certs`, và spec ghi rõ: *"Toàn bộ data là plain text bên trong SQLCipher — file-level encryption lo phần còn lại"*. Không có đường mã hóa field-level nào cho secret.

**Vì sao nghiêm trọng:**
- **D4 dựa trên cơ chế không tồn tại.** Nếu proposal thật sự muốn thêm `secret_enc` (field-level encryption), đó là quyết định kiến trúc mới **đảo ngược** `DECISION_VAULT_STORAGE.md` — nơi phương án "SQLite thường + field AES" đã bị loại vì chi phí maintenance mỗi column. Việc đó phải mở tension riêng, không được đi kèm lặng lẽ trong proposal custom fields.
- **Chưa trả lời cột gắn vào bảng nào.** Chính 2 ví dụ ở §1 trải trên 2 bảng khác nhau: `node`/`hosting_ip` là thuộc tính *service* (mà `services` đã có sẵn `server_ip`, `nameservers`), còn `version` của license là thuộc tính *credential*. Nhiều khả năng cần cột trên **cả `services` lẫn `service_credentials`** (cân nhắc thêm `email_accounts`) — phải liệt kê rõ trong §4.

**Cách sửa D4 cho đúng thực tế:** ranh giới không phải "JSON vs `secret_enc`" mà là "**cột JSON vs cột `password` của `service_credentials`**". Secret vẫn là plain text trong SQLCipher như mọi dữ liệu khác; điểm khác là nó nằm ở cột được **UI che + watermark + ghi audit khi reveal**, còn custom_fields là dữ liệu hiển thị/search tự do. D4 là **ranh giới UI + audit**, không phải ranh giới mã hóa.

### F2. 🔴 Bug SQL mẫu — nối chuỗi JSON path (§6)

```sql
-- SAI: key người dùng ghép thẳng vào path
AND json_extract(e.custom_fields, '$.'||?) = ?
```

**Vì sao:** key chứa `.`, `"`, `[`, khoảng trắng (rất thực tế với key tiếng Việt như `"máy chủ"`) làm path sai hoặc error — lỗi injection ngữ nghĩa kinh điển. Cách đúng, tham số hóa sạch bằng chính `json_each` đã dùng:

```sql
AND EXISTS (SELECT 1 FROM json_each(e.custom_fields) j
            WHERE j.key = ? AND j.value = ?)
```

**Hệ quả nhất quán:** cách này không dùng được expression index trên `json_extract` — nhưng §5 đã tự tuyên bố "đủ nhanh với ≤ vài nghìn entry, không tối ưu sớm", vậy **bỏ expression index khỏi scope V1 luôn**, giữ trong bảng chiến lược làm nâng cấp khi có bằng chứng chậm.

### F3. 🟡 Tham chiếu lỗi thời + timing mâu thuẫn quyết định dogfood-first

- **Confuse Engine đã bị cắt khỏi V1** (human chốt 2026-07-13 — `docs/pm-review-solutions.md` §7). Mọi lý luận ở §3 dựa trên "bất biến của Confuse Engine" phải gỡ.
- **Trích dẫn sai:** `DECISION_VAULT_STORAGE.md` §4.2(e) thực tế là mục "Security regression" trong so sánh KDBX — không nói về route secret riêng.
- **Timing:** "Milestone dự kiến ≥ 0.1.1" quá mơ hồ. Câu hỏi PM phải hỏi trước: `notes TEXT` (đã có trên mọi bảng) + `server_ip` (đã có) **chưa đủ đau đến mức nào?** — ví dụ `node: cp79` nhét vào notes vẫn LIKE ra được. Theo nguyên tắc dogfood-first vừa chốt, bằng chứng nhu cầu nên đến từ dùng thật (0.6.0). Tuy nhiên có bất đối xứng chi phí: thêm cột `TEXT DEFAULT '{}'` vào migration **0.2.0 bây giờ rẻ**, thêm sau lại tốn migration mới. → **Đề xuất: chốt cột JSON trong schema 0.2.0; hoãn toàn bộ UI + search command đến sau mốc dogfood 0.6.0.** Nếu dogfood cho thấy notes đủ dùng, cột JSON nằm im không tốn gì.

### F4. 🟡 Bổ sung kỹ thuật nhỏ (sửa khi viết lại)

| Điểm | Lý do |
|------|-------|
| Thêm `CHECK (json_valid(custom_fields))` vào cột | Một dòng, chặn vĩnh viễn JSON hỏng lọt vào DB từ mọi code path |
| Ghi known-limitation: `LIKE` SQLite chỉ case-insensitive với ASCII | "may chu" không khớp "Máy Chủ", không bỏ dấu tiếng Việt — UX search thật cho user VN; nếu cần thì normalize (lowercase + bỏ dấu) cột shadow sau |
| Chặn/gợi ý trùng cột chuẩn (`note` vs `notes`, `ip` vs `server_ip`) | Ví dụ §2 tự tạo field `note` song song cột `notes` có sẵn → dữ liệu tách đôi; autocomplete phải ưu tiên gợi ý cột chuẩn |
| Sửa custom_fields phải ghi `activity_log` (action `updated`, không cần value hint) | Giữ nhất quán audit với mọi thao tác write khác |
| BTreeMap → UI hiển thị field theo alphabet, không theo thứ tự thêm | Chấp nhận được, nhưng ghi rõ là chủ ý để khỏi bị coi là bug |
| Không dựa mỗi `#[serde(skip_serializing)]` để giấu secret | Cần chặn ở tầng command trả về frontend (comment §6 đã nói đúng — nâng thành yêu cầu test) |

### Những gì proposal làm đúng — giữ nguyên

- **JSON column > EAV** cho sparse text fields ở quy mô SME — EAV là over-engineering kinh điển, loại là đúng (D3).
- **All-text, không typed, không template** (D1, D2) kèm điều kiện mở lại rõ ở §9 — đúng tinh thần "mở tension mới mới được phá ràng buộc".
- **Autocomplete `list_field_names`** cho rủi ro loạn tên key — rủi ro thật số 1 của free-form fields, mitigation rẻ và đúng chỗ.
- "Đo trước, FTS5 chỉ khi có bằng chứng chậm" — kỷ luật tốt.

### Điều kiện approve (tóm tắt)

1. Viết lại §3–§6 trên schema V1 thật: chỉ rõ bảng nhận cột; định nghĩa lại D4 thành ranh giới UI/audit.
2. Sửa SQL filter sang `json_each(j.key = ? AND j.value = ?)`; thêm `CHECK(json_valid(...))`; bỏ expression index khỏi V1.
3. Gỡ tham chiếu Confuse + §4.2e; retiming: schema 0.2.0, UI/search sau dogfood 0.6.0.
4. Bổ sung audit log + known-limitation LIKE tiếng Việt.

---

## 10. Human sign-off

- [ ] Đồng ý §2 (D1–D5) — implement 1 cột JSON + JSON1 search
- [ ] Không đồng ý — cần bàn lại (typed field / template / EAV)
- [ ] Đồng ý hướng D1–D3, D5 **với điều kiện** sửa theo §Fable phản biện (F1–F4) trước khi implement
- Ghi chú:

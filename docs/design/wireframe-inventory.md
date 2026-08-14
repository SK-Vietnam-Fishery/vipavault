# Wireframe inventory — thông tin cần hiện trên UI

> **Mục đích:** Catalog field/màn để viết wireframe. **Không** thay spec.
> **Source of truth:** [`../vipavault-spec.md`](../vipavault-spec.md), [`../technical-decisions.md`](../technical-decisions.md) §3–4.
> **Trạng thái:** working draft — phần §13 chưa chốt, ghi vào wireframe khi quyết.
> **Phạm vi:** SIMPLE V1. Custom fields, Share Package, OAuth: không nằm trong catalog này.
> **Ngày gom:** 2026-08-14

Chưa có wireframe hình / HTML. Chỉ có ASCII login trong `technical-decisions.md` §3.1. Code React hiện là boot shell 0.1.0.

---

## 1. Hai vai trò — cùng app, khác bề mặt

| | Admin (IT) | Viewer (CEO) |
|---|---|---|
| Mục đích | Làm việc: credential, email, sync | Nhìn: chi phí, hết hạn, tổng thể |
| Quyền | Full | Chỉ đọc — mọi nút write **disabled** |
| Badge | không | **"Chế độ xem"** (luôn thấy) |
| Sync API | Refresh nếu `sync_enabled` | `sync_enabled: false` → không gọi provider |

`machine_role` là UX, không phải ranh giới bảo mật: ai có file + master password là đọc được hết.

---

## 2. Vỏ app (sau unlock) — luôn có

| Hiện | Nguồn | Ghi chú |
|---|---|---|
| Tên vault đang mở | `profiles.json` → `display_name` | |
| Role / badge | `app_settings.json` → `machine_role` | Viewer: "Chế độ xem" |
| Email operator | `app_settings.json` → `operator_email` | Label, không phải account |
| Nút khóa vault | — | Idle timeout → khóa, zeroize key |
| Nav | — | Dashboard · Hosting · Email · (Notify P1) · (Audit P1) |

Profile switcher: **P0** (spec). ≥2 vault thì chọn profile; 1 vault thì chỉ label.

---

## 3. Màn login / tạo vault

**Thứ tự luôn:** ① tên vault → ② email operator → ③ mật khẩu.

### Lần đầu (chưa có `operator_email`)

| Hiện | Loại |
|---|---|
| Ô "Email của bạn" | Input + regex format (`@`, local/domain tối thiểu) — **backend** validate |
| Nút "Tiếp tục" | Ghi `operator_email`, rồi thành label |

### Lần sau / unlock

| Hiện | 1 vault | ≥2 vault |
|---|---|---|
| Tên vault | Label | Dropdown `display_name` |
| Email operator | Label | Label (không đổi theo vault) |
| Ô mật khẩu | Input, **không lưu** | Input |
| Nút "Mở khóa" | | |

Lỗi cần phân biệt trên UI: sai mật khẩu / file hỏng / không tìm thấy file — **không** hiện nội dung secret.

**Không hiện:** OTP, forgot-password, allowlist, PIN, Share Package.

Tạo vault lần đầu (0.1.1): cần tên vault + master password. Recovery in giấy = khuyến nghị PM, **chưa chốt spec**.

ASCII đã có: [`../technical-decisions.md`](../technical-decisions.md) §3.1.2.

---

## 4. Dashboard — cả 2 role

### Cả admin và viewer

| Hiện | Nguồn / công thức | Ghi chú |
|---|---|---|
| Tổng chi phí tháng | `SUM(services.monthly_cost)` — V1 nhập tay | Chưa chốt: có cộng domain riêng không |
| Số dịch vụ | `services` (thường `status = active`) | |
| Cảnh báo hết hạn | `expires_at` → đỏ / vàng / xanh | Xem rule dưới |
| Provider breakdown | nhóm theo `services.provider` (Namecheap, Viettel…) | Không nhầm với `provider_type` (`cpanel`) |

**Alert (đã chốt ngưỡng, chưa chốt áp lên bảng nào):**

- Đỏ: còn **< 7 ngày**
- Vàng: **7–30 ngày**
- Xanh: còn lại

Ngày hết hạn có trên: `services.expires_at`, `domains.expires_at`, `ssl_certs.expires_at`.

### Chỉ admin thêm

| Hiện | Nguồn |
|---|---|
| Disk usage | `cpanel_sync_cache.disk_used_mb` / `disk_quota_mb` |
| Số email | `cpanel_sync_cache.email_count` |
| Sync status | `sync_status` + `last_synced_at` → **"Cập nhật lúc HH:MM"** |
| Lỗi sync | `error_msg` |
| Nút action | Refresh, đi tới hosting/email… |

Viewer: **không** nút write / Refresh.

**Khuyến nghị PM (chưa vào spec):** banner viewer **"Dữ liệu tại ngày DD/MM"** + đỏ nếu file cũ hơn ~14 ngày (copy `.hvault` đóng băng). Xem [`../reviews/pm-review-solutions.md`](../reviews/pm-review-solutions.md) §2.

---

## 5. Cây dữ liệu (3 tầng) — Hosting / service

UI tree: **profile → service → credential / email / domain**. Chưa chốt tree vs master-detail vs list.

### Thẻ / hàng service

| Hiện | Cột |
|---|---|
| Tên hiển thị | `display_name` |
| Loại | `service_type`: `web_hosting` \| `email_hosting` \| `domain` \| `vps` |
| Nhà cung cấp (thương hiệu) | `provider` — **cái user nhìn** |
| Kiểu panel | `provider_type`: `cpanel` \| `directadmin` (V1) |
| Gói | `package_name` |
| Domain chính | `domain_primary` |
| IP server | `server_ip` |
| Nameserver | `nameservers` (JSON array) |
| Portal | `portal_url` — mở browser (chưa chốt in-app) |
| Hết hạn + màu | `expires_at` + `auto_renew` |
| Chi phí tháng | `monthly_cost` |
| Trạng thái | `status` (mặc định `active`) |
| Ghi chú | `notes` |

V1: `provider_type` lạ → bỏ qua, **không crash**. Có thể toast "chưa hỗ trợ" — chưa chốt.

---

## 6. Credential (admin; viewer không reveal)

Bảng `service_credentials` — chỉ `auth_scheme = api_token`.

| Hiện | Cột | Reveal? |
|---|---|---|
| Nhãn | `label` — vd "cPanel Admin", "FTP backup" | luôn |
| Username | `username` | luôn |
| Password / token | `password` | **ẩn mặc định**; hiện có watermark tên IT |
| Loại | `credential_type`: `panel` \| `ftp` \| `ssh` \| `api_key` | luôn |
| Ghi chú | `notes` | luôn |
| Dùng lần cuối | `last_used_at` | luôn |

Khi reveal: watermark tên IT (spec §3). Audit `viewed_password` + `actor_note`. Copy clipboard / auto-hide: chưa chốt.

Viewer: không nút Hiện / Copy / Sửa.

---

## 7. Email (admin; viewer list — chưa chốt)

Spec: **"Quản lý Email (IT only)"** — viewer không tạo/reset; trên dashboard chỉ `email_count`.

| Hiện | Cột |
|---|---|
| Địa chỉ | `email_address` |
| Tên người | `display_name` — "Nguyễn Văn A - Kế toán" |
| Phòng ban | `department` |
| Password | ẩn; sinh CSPRNG khi tạo/reset |
| Quota | `quota_mb` (mặc định 500) |
| Active | `is_active` |
| Đổi pass lần đầu | `must_change_password` |
| Reset lần cuối | `last_reset_at` |
| Sync | `sync_status` (0.2.0): `local_only` \| `pending_sync` \| `synced` \| `sync_error` |

**Hành động admin:** Tạo (local `pending_sync`) → review → **Apply** lên provider. Reset / xóa tương tự.

Dialog Apply: **hiện địa chỉ email, không hiện password**.

---

## 8. Domain + SSL (gắn service)

**Domain**

| Hiện | Cột |
|---|---|
| Tên miền | `domain_name` |
| Registrar | `registrar` + `registrar_url` |
| Hết hạn + màu | `expires_at` |
| Auto renew | `auto_renew` |
| Ghi chú | `notes` |

**SSL**

| Hiện | Cột |
|---|---|
| Issuer | `issuer` |
| Hết hạn + màu | `expires_at` |
| Ghi chú | `notes` |

---

## 9. Sync (nút, không phải màn riêng)

| Hiện | Nguồn |
|---|---|
| Nút "Refresh từ cPanel" (hoặc DirectAdmin) | chỉ admin + `sync_enabled` |
| Cooldown còn lại | tối đa **1 lần / 10 phút / service** |
| "Cập nhật lúc …" | `last_synced_at` |
| Trạng thái | `never` / ok / lỗi + `error_msg` |
| Queue Apply | email `pending_sync` chờ xác nhận |

Không auto-sync. Không spinner "đang đồng bộ nền".

---

## 10. Notify / Confuse — P1 (0.10.0)

Vault **không** hiện confuse string trên màn credential. Chỉ sinh lúc gửi.

| Kênh | Hiện | Không hiện |
|---|---|---|
| Zalo | `prefix + pass thật + suffix` + hint "(bỏ N đầu M cuối)" | — |
| Email | địa chỉ + webmail + hướng dẫn cài | **password** |

Rule `prefix` / `suffix` đọc từ settings (trong vault), không hardcode. Sau gửi: ghi `activity_log.confuse_used`.

---

## 11. Audit (P1)

| Hiện | Cột |
|---|---|
| Thời điểm | `created_at` |
| Đối tượng | `entity_type` + `entity_id` |
| Hành động | `created` \| `password_reset` \| `viewed_password` … |
| Người thao tác | `actor_note` — "IT - Tuấn" (không `actor_email`) |
| Gợi ý giá trị cũ/mới | `old_value_hint` / `new_value_hint` — kiểu `***abc` (3 ký tự cuối) |
| Confuse đã dùng | `confuse_used` |

Không hiện password đầy đủ trong log.

---

## 12. Cái không được hiện

- Master password (không lưu, không echo lại sau unlock)
- Access token OAuth (Phase 2, cache)
- Confuse string trên màn xem credential
- Password trong dialog Apply / mail hướng dẫn
- Credential trong log/debug
- Wizard Share Package, OTP, user account

---

## 13. Việc cần chốt khi viết wireframe

| # | Câu hỏi | Ảnh hưởng layout |
|---|---|---|
| 1 | Dashboard admin/viewer: **cùng khung khác nút**, hay 2 layout? | Toàn dashboard |
| 2 | Alert áp `services` + `domains` + `ssl`, hay chỉ service? | Số chấm đỏ |
| 3 | `expires_at` null: xám "chưa có ngày" / ẩn / xanh? | Hàng thiếu ngày |
| 4 | Disk/email chưa sync: **"Chưa sync"** hay N/A? | Card admin |
| 5 | Tree vs master-detail vs list + filter | Hosting/credential |
| 6 | Viewer có thấy **danh sách** hosting/email (ẩn secret) không, hay chỉ dashboard? | Nav viewer |
| 7 | Timezone hiện: `Asia/Ho_Chi_Minh`? | Mọi ngày |
| 8 | Empty vault: CTA "Thêm hosting đầu tiên"? | Dashboard trống |
| 9 | Reveal: timeout ẩn? copy clipboard? | Overlay pass |
| 10 | Ngôn ngữ UI: Việt only? | Copy trên wireframe |
| 11 | Banner tuổi dữ liệu viewer? | Header dashboard |
| 12 | Settings trong app hay sửa file tay (`machine_role`, confuse rule)? | Có/không màn Settings |

Câu hỏi gốc: `.context/planning/MILESTONE_QUESTIONNAIRE.md` (0.3.0–0.5.0).

---

## 14. Gợi ý thứ tự vẽ

1. **Unlock** (đã có ASCII) — `technical-decisions.md` §3.1.2
2. **Dashboard viewer** — chi phí, số service, 3 mức hết hạn, breakdown, badge, (banner ngày dữ liệu nếu nhận §13.11)
3. **Service detail admin** — cây service → credential ẩn + email + nút Refresh/Apply

Màn phụ: login lần đầu, reveal + watermark, compose Zalo, audit.

---

## 15. Wireframes (viết tiếp tại đây)

_Chưa có. Dán ASCII / ghi chú layout bên dưới._

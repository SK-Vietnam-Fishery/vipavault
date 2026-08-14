# VipaVault

App desktop quản lý hosting, email, domain và SSL của công ty — IT làm việc, sếp nhìn dashboard, không cần vào cPanel.

Một file két mã hóa (`.hvault`) trên máy. Không phải SaaS, không tài khoản user.

**Mục lục toàn bộ tài liệu:** [INDEX.md](INDEX.md)

---

## Ai dùng

| Role | Máy | Việc |
|---|---|---|
| `admin` (IT) | `machine_role: admin` | Tạo/reset email, xem credential, bấm Refresh |
| `viewer` (sếp) | copy `.hvault` + `machine_role: viewer` | Chi phí, hết hạn, tổng quan — chỉ đọc |

Role nằm trong `app_settings.json` **ngoài** vault, theo từng máy. Ai có file + master password là đọc được hết; role chỉ là UX.

---

## Cách hoạt động (V1)

1. Mở khóa bằng **master password** (Argon2id → SQLCipher). Email trên màn login chỉ là nhãn, không phải tài khoản.
2. Dữ liệu quan hệ nằm trong `.hvault`: service → credential / email / domain / SSL.
3. Đồng bộ provider **thủ công**, tối đa 1 lần / 10 phút / service. V1: cPanel và DirectAdmin.
4. Tạo email: lưu local (`pending_sync`) → xác nhận → Apply lên panel.

**Không làm trong V1:** OAuth (M365, Google), Share Package, auto-sync, khôi phục master password, tài khoản nhiều user.

---

## Stack

Tauri 2 · Rust · React + TypeScript · Vite · SQLCipher (AES-256-GCM cả file)

License: AGPL-3.0.

---

## Hiện trạng

| | |
|---|---|
| Phase | V1 |
| Xong | 0.1.0 Foundation (app boot, test, context) |
| Tiếp | 0.1.1 Vault Core — tạo / mở / khóa `.hvault` |
| UI | Shell boot; chưa có dashboard / login sản phẩm |
| Wireframe | Catalog field: [design/wireframe-inventory.md](design/wireframe-inventory.md) |

---

## Đọc tiếp

| Cần | Mở |
|---|---|
| Tính năng, schema, ràng buộc | [vipavault-spec.md](vipavault-spec.md) |
| Vì sao chọn kỹ thuật này | [technical-decisions.md](technical-decisions.md) |
| Mọi file trong `docs/` | [INDEX.md](INDEX.md) |
| Chạy máy dev | [README gốc](../README.md) |

# PROJECT — VipaVault

## [manual] Decisions

- Desktop app Tauri 2.x + Rust + React + TypeScript quản lý hosting/email/domain credentials.
- **Storage:** SQLCipher file `.hvault` (AES-256-GCM toàn file), key từ Argon2id — **không** dùng KeePass/KDBX làm backend V1. Xem `.context/decisions/DECISION_VAULT_STORAGE.md`.
- **Phân tầng vault (3–4 lớp):** Company profile → service group → sub-credentials — implement qua SQL FK + UI tree, không qua KDBX groups.
- **Roles:** `admin` | `viewer` per-machine qua `app_settings.json` — không user account system.
- **Providers V1:** `cpanel` + `directadmin` only; OAuth Phase 2.
- **Sync:** Manual only, rate limit 1 lần / 10 phút / service.
- Spec source of truth: `docs/vipavault-spec.md`.
- Milestone hiện tại: V1 / 0.1.0 Project Foundation.

## [manual] Invariants

- Không dùng SQLite thường — phải SQLCipher.
- Không log credential dù debug.
- Không lưu master password.
- Không auto-sync provider API.
- Viewer mode: không enable write, không gọi provider API khi `sync_enabled = false`.
- Agent ghi tension thay vì silently break rules.

## [manual] Open Decisions

- Chi tiết `credential_type` mở rộng (mariadb, postgres, …) — khi implement 0.5.0/0.6.0.
- KDBX export/import optional — Phase 2+ nếu cần.
# Decision Record: Operator Email (display label)

**Status:** APPROVED  
**Date:** 2026-06-18  
**Approved by:** Human — login UX clarification session  
**Milestone:** V1 / 0.3.0 (UI), storage from first-run  
**Severity:** LOW

---

## 1. Vấn đề

Màn login cần dòng “người thao tác” trước ô master password. Cần chốt: lưu gì, ở đâu, có phải auth không.

## 2. Quyết định

| | |
|---|---|
| **Field** | `operator_email` — string plaintext |
| **Lưu trữ** | `app_settings.json`, per-machine |
| **Ai ghi** | **Backend Rust** (Tauri commands) — không frontend trực tiếp |
| **Lần đầu** | UI ô email → regex format đơn giản → lưu → thành label → ô pass |
| **Lần sau** | Label email từ backend |
| **Fallback** | `whoami` khi chưa có giá trị |
| **Unlock** | Chỉ master password — `operator_email` không tham gia |

## 3. Không làm (V1)

- Email allowlist / `workspace_members`
- Verify DNS / OTP / forgot password
- Lưu trong `.hvault`
- Sai email → từ chối login

## 4. Tài liệu

- `docs/technical-decisions.md` §3.1.2
- `docs/vipavault-spec.md` — `app_settings.json` example
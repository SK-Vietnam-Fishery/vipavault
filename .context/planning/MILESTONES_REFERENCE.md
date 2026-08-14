# Milestones Reference — VipaVault

> Full V1 roadmap reference.
> Do not load by default. Pull details into `.context/MILESTONES.md` when a milestone approaches or human asks.
> **Reordered 2026-07-13** theo `docs/pm-review-solutions.md` — dogfood tại 0.6.0; Confuse & Notification cắt khỏi V1 (chuyển Phase 2+ backlog); release tại 0.10.0.

**Version ID convention:** bootstrap `0.1.0`–`0.1.1`; mọi milestone sau đó `0.N.0` tăng dần (`0.2.0` … `0.10.0`).

---

## Summary

| # | ID | Milestone | Primary Outcome |
|---|---|---|---|
| 1 | 0.1.0 | Project Foundation | Tauri/Rust/React/test/context baseline works |
| 2 | 0.1.1 | Vault Core | Create/open/lock `.hvault` safely |
| 3 | 0.2.0 | Data Model & Migrations | V1 schema and migrations match spec |
| 4 | 0.3.0 | App Shell & Roles | Admin/viewer shell with enforced role gates |
| 5 | 0.4.0 | Dashboard Slice | Dashboard reads real local data and alert thresholds |
| 6 | 0.5.0 | Credential Management | Admin credential viewer/editor with no secret leakage |
| 7 | 0.6.0 | Email Accounts Local | Local email lifecycle with generated passwords and audit — **DOGFOOD milestone** |
| 8 | 0.7.0 | Provider Routing V1 | cPanel/DirectAdmin routing and unknown-provider skip |
| 9 | 0.8.0 | Manual Sync & Rate Limit | Refresh sync with hard rate limit |
| 10 | 0.9.0 | Provider Email Apply | Pending email changes apply to provider APIs |
| 11 | 0.10.0 | MVP Hardening & Release | Packaged desktop MVP with tests passing |

---

## V1 vs FULL (Phase 1.5+)

| Capability | V1 (0.1.0–0.10.0) SIMPLE | FULL (post-0.10.0) |
|------------|--------------------------|---------------------|
| Login | Master password only | Optional email gate (TBD) |
| Share | Full `.hvault` copy + `machine_role: viewer` | Share Package subset + activation claim |
| Schema | No `workspace_members` | `workspace_members` if email gate approved |
| Audit actor | `actor_note` | Optional `actor_email` |
| PIN quick unlock | Defer | 0.10.0+ if implemented |

Reference: `.context/planning/MILESTONE_QUESTIONNAIRE.md` §Auth, §FULL, TD-SHARE-*.

---

## 0.1.0 — Project Foundation

Goal: Establish runnable Tauri 2.x + React + TypeScript + Rust baseline.

Deliverables:
- App scaffold.
- Backend/frontend test runners.
- Context-gen indexes real source files.
- `.context/GLOBAL.md` generated.

Exit Criteria:
- `cargo test`, `npm test`, `context-gen build .`, and `context-gen check-consistency .` pass.

---

## 0.1.1 — Vault Core

Goal: Implement safe `.hvault` lifecycle (SIMPLE V1 unlock).

Deliverables:
- SQLCipher vault create/open/lock.
- Argon2id key derivation.
- zeroize key material.
- No master password persistence.
- Master-password unlock only (no email gate).
- `profiles.json` when multiple vaults.

Exit Criteria:
- Vault lifecycle tests pass.
- Multi-profile selection opens correct `.hvault`.
- No `workspace_members` or email login.

---

## 0.2.0 — Data Model & Migrations

Goal: Implement V1 schema as migrations (SIMPLE V1 exclusions).

Deliverables:
- `services`
- `service_credentials`
- `email_accounts` (+ `sync_status` column)
- `domains`
- `ssl_certs`
- `activity_log` (`actor_note`; no `actor_email` in V1)
- `cpanel_sync_cache`
- `app_settings` (in-vault table)
- **Not in V1:** `workspace_members`

Exit Criteria:
- Migrations match spec with SIMPLE V1 auth exclusions.
- No `workspace_members` table.

---

## 0.3.0 — App Shell & Roles

Goal: Establish admin/viewer role boundary (SIMPLE V1 login UX).

Deliverables:
- Main shell.
- Login: master password; vault dropdown if ≥2 profiles.
- `app_settings.json`: `machine_role`, `sync_enabled` only.
- Viewer badge.
- Write action gating.
- Ops note: viewer = full vault copy + per-machine role (no Share wizard).

Not in V1:
- Email login, Share Workspace, `last_login_email`.

Exit Criteria:
- Viewer cannot write through UI or backend commands.
- Login UI has no email field.

Open: rotate master password wizard — milestone TBD.

---

## 0.4.0 — Dashboard Slice

Goal: First end-to-end read-only product workflow.

Deliverables:
- Dashboard query command.
- Cost/service/domain summary.
- Expiration alerts.
- Provider breakdown.

Exit Criteria:
- Dashboard shows meaningful local vault data.

---

## 0.5.0 — Credential Management

Goal: Manage local credentials without provider API dependency.

Deliverables:
- Hosting/service list.
- Admin-only credential viewer.
- Add/edit credential forms.
- Audit entry for sensitive views.

Exit Criteria:
- Admin can manage credentials; viewer cannot reveal or edit secrets.

---

## 0.6.0 — Email Accounts Local

Goal: Model email account workflows locally before provider apply.

> **DOGFOOD:** Từ milestone này app dùng được hàng ngày với dữ liệu nhập tay. 0.7.0–0.9.0 chỉ triển khai nếu sau ≥1 tháng dogfood vẫn thấy đau ở thao tác cPanel thủ công.

Deliverables:
- Email account list per service.
- Create local email with CSPRNG password.
- `pending_sync` status.
- Activity log entries.

Exit Criteria:
- Local email lifecycle is test-covered and provider-independent.

---

## 0.7.0 — Provider Routing V1

Goal: Implement provider abstraction without OAuth scope creep.

Deliverables:
- Provider router by `provider_type` + `auth_scheme`.
- cPanel/DirectAdmin client boundary.
- Unknown provider warning and skip.

Exit Criteria:
- Routing tests prove unsupported providers do not panic.

---

## 0.8.0 — Manual Sync & Rate Limit

Goal: Sync provider state only by explicit user action.

Deliverables:
- Refresh command.
- 10-minute per-service rate limit.
- Sync cache update.
- `sync_enabled = false` guard.

Exit Criteria:
- Repeated refresh is blocked by rate limit.

---

## 0.9.0 — Provider Email Apply

Goal: Apply reviewed local email changes to cPanel/DirectAdmin.

Deliverables:
- Add email API call.
- Reset password API call.
- Delete email API call.
- Pending → synced status transition.
- Activity log for provider writes.

Exit Criteria:
- Mock provider tests cover add/reset/delete paths.

---

## 0.10.0 — MVP Hardening & Release

Goal: Prepare MVP for real desktop use (SIMPLE V1 complete).

Deliverables:
- Auto-lock behavior.
- WAL/backup checks.
- Error handling pass.
- Packaging.
- Final test sweep.

Exit Criteria:
- Desktop build succeeds and all required tests pass.
- MVP scope excludes FULL extension (Share Package, email gate, PIN).
- SIMPLE V1 auth/sharing invariants documented in release notes.

---

## Phase 1.5+ — FULL extension (not V1 execution)

Deferred until human activates execution past 0.10.0:

- **Share Package** — subset export, activation-code claim, recipient `PRAGMA rekey` (TD-SHARE-*).
- **Workspace / email gate** — only if reopened (`workspace_members`, Share Workspace).
- **PIN quick unlock** — admin machine only, if approved.

Detail: `.context/planning/MILESTONE_QUESTIONNAIRE.md` §FULL. No milestone ID assigned until roadmap review.

---

## Phase 2+ Backlog

### Confuse & Notification (cắt khỏi V1 — 2026-07-13)

Goal: Generate notification-safe password messages.

Deliverables:
- Confuse rule from `app_settings`.
- Generate confuse string at send time only.
- `confuse_used` audit field.
- Notification composer.

Exit Criteria:
- Vault stores real password only; confuse string is not source of truth.

**Điều kiện mở lại:** threat model phải được làm rõ trước khi implement — rule prefix/suffix tĩnh dùng chung toàn công ty không còn là bí mật sau người nhận thứ 2; V1 dựa vào `must_change_password` làm lớp bảo vệ thật. Phương án thay thế cân nhắc: one-time secret link self-host. Chi tiết: `docs/pm-review-solutions.md` §7.
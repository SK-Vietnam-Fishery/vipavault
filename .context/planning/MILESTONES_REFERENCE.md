# Milestones Reference — VipaVault

> Full V1 roadmap reference.
> Do not load by default. Pull details into `.context/MILESTONES.md` when a milestone approaches or human asks.

**Version ID convention:** bootstrap `0.1.0`–`0.1.1`; mọi milestone sau đó `0.N.0` tăng dần (`0.2.0` … `0.11.0`).

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
| 7 | 0.6.0 | Email Accounts Local | Local email lifecycle with generated passwords and audit |
| 8 | 0.7.0 | Provider Routing V1 | cPanel/DirectAdmin routing and unknown-provider skip |
| 9 | 0.8.0 | Manual Sync & Rate Limit | Refresh sync with hard rate limit |
| 10 | 0.9.0 | Provider Email Apply | Pending email changes apply to provider APIs |
| 11 | 0.10.0 | Confuse & Notification | Confuse message generated only at send time |
| 12 | 0.11.0 | MVP Hardening & Release | Packaged desktop MVP with tests passing |

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

Goal: Implement safe `.hvault` lifecycle.

Deliverables:
- SQLCipher vault create/open/lock.
- Argon2id key derivation.
- zeroize key material.
- No master password persistence.

Exit Criteria:
- Vault lifecycle tests pass.

---

## 0.2.0 — Data Model & Migrations

Goal: Implement V1 schema as migrations.

Deliverables:
- `services`
- `service_credentials`
- `email_accounts`
- `domains`
- `ssl_certs`
- `activity_log`
- `cpanel_sync_cache`
- `app_settings`

Exit Criteria:
- Migrations match spec and tests pass.

---

## 0.3.0 — App Shell & Roles

Goal: Establish admin/viewer role boundary.

Deliverables:
- Main shell.
- App settings read path.
- Viewer badge.
- Write action gating.

Exit Criteria:
- Viewer cannot write through UI or backend commands.

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

## 0.10.0 — Confuse & Notification

Goal: Generate notification-safe password messages.

Deliverables:
- Confuse rule from `app_settings`.
- Generate confuse string at send time only.
- `confuse_used` audit field.
- Notification composer.

Exit Criteria:
- Vault stores real password only; confuse string is not source of truth.

---

## 0.11.0 — MVP Hardening & Release

Goal: Prepare MVP for real desktop use.

Deliverables:
- Auto-lock behavior.
- WAL/backup checks.
- Error handling pass.
- Packaging.
- Final test sweep.

Exit Criteria:
- Desktop build succeeds and all required tests pass.
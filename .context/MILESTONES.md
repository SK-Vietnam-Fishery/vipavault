# Milestones — VipaVault

Current: V1
Active Execution: 0.1.0 — Project Foundation

**Version ID convention:** bootstrap `0.1.0`–`0.1.1`; mọi milestone sau đó `0.N.0` tăng dần (`0.2.0` … `0.11.0`). Không dùng `0.1.4` kiểu patch lẫn minor.

---

## Milestone Operating Model

VipaVault uses a domain/security-led vertical-slice plan.

Meaning:
- Domain, security, schema, provider routing, and sync rules define the hard boundary.
- UX validates real workflows for admin/viewer users.
- Work is delivered in vertical slices whenever possible: UI + IPC + backend + DB + tests.
- The full roadmap lives in `.context/planning/MILESTONES_REFERENCE.md`; this file keeps only the current phase, summary table, and the next 3-4 execution milestones.

Agents must load this file before planning work. Pull detail from `.context/planning/MILESTONES_REFERENCE.md` only when the active execution milestone is about to change or the human asks for broader planning.

---

## Milestone Activation Protocol

Milestone activation is a human-controlled project state change.

### For Human

To activate a new execution milestone:

1. Set `Active Execution:` to the new milestone id and name.
2. Mark the previous execution milestone as `Status: done`.
3. Mark the new execution milestone as `Status: current`.
4. Add `Started: YYYY-MM-DD` to the new milestone.
5. Pull the next milestone details from `.context/MILESTONES_REFERENCE.md` into this file if useful.
6. Run `context-gen check-consistency .`.
7. Commit `.context/MILESTONES.md` separately when practical.

To activate a new phase:

1. Set `Current:` to the new phase id.
2. Update affected entries in `.context/TENSIONS_ACTIVE.md`.
3. Move no-longer-active decisions to `.context/TENSIONS_HISTORY.md` with `Status: ARCHIVED` only when explicitly approved.

### For Agent

At the start of every task:

1. Read `.context/MILESTONES.md`.
2. Treat `Current:` as the phase boundary and `Active Execution:` as the work boundary.
3. Load context only for modules relevant to the current task and active execution milestone.
4. If the requested task is outside active execution, ask for confirmation unless the human explicitly expands scope.
5. If the requested task conflicts with phase constraints, create an OPEN tension before proceeding.
6. Never activate, complete, or archive a milestone unless the human explicitly asks.
7. After code or context changes, run `context-gen build .` and `context-gen check-consistency .`.

---

## V1 Phase Boundary

Status: current

Scope: MVP desktop app for local encrypted hosting/email/domain credential management.

Rules:
- Implement `cpanel` and `directadmin` only.
- OAuth providers remain Phase 2.
- Sync remains manual with hard rate limit.
- Viewer mode remains read-only and must not call provider APIs when sync is disabled.
- SQLCipher is mandatory; do not replace with plain SQLite.
- Master password is never stored.

### SIMPLE V1 — Auth & sharing (human-approved 2026-06)

Login:
- Master password only — opens SQLCipher `.hvault`.
- 1 vault: password field only. ≥2 vaults: profile dropdown + password.
- No email allowlist, no `workspace_members`, no user accounts.

Roles & sharing:
- `machine_role` + `sync_enabled` in per-machine `app_settings.json` (outside vault).
- CEO/viewer: copy full `.hvault` to viewer machine; set `machine_role: viewer`.
- No partial vault export, no Share Package wizard in V1.

Audit:
- `activity_log.actor_note` only (OS username or free-text). No `actor_email` in V1.

Deferred past 0.11.0 (Phase 1.5+ / FULL):
- Share Package subset export, activation-code claim, `workspace_members`, PIN quick unlock.
- Detail: `.context/planning/MILESTONE_QUESTIONNAIRE.md` §FULL.

Auth UX source of truth: questionnaire §Auth — update `docs/vipavault-spec.md` when implementing 0.3.0.

---

## V1 Execution Overview

| # | ID | Milestone | Method | Primary Outcome | Status |
|---|---|---|---|---|---|
| 1 | 0.1.0 | Project Foundation | Foundation slice | Tauri/Rust/React/test/context baseline works | current |
| 2 | 0.1.1 | Vault Core | Security-led backend slice | Create/open/lock `.hvault` safely | planned |
| 3 | 0.2.0 | Data Model & Migrations | Schema-led slice | V1 schema and migrations match spec | planned |
| 4 | 0.3.0 | App Shell & Roles | UX workflow slice | Admin/viewer shell with enforced role gates | planned |
| 5 | 0.4.0 | Dashboard Slice | Vertical slice | Dashboard reads real local data and alert thresholds | planned |
| 6 | 0.5.0 | Credential Management | Vertical slice | Admin credential viewer/editor with no secret leakage | planned |
| 7 | 0.6.0 | Email Accounts Local | Vertical slice | Local email lifecycle with generated passwords and audit | planned |
| 8 | 0.7.0 | Provider Routing V1 | Domain/provider slice | cPanel/DirectAdmin routing and unknown-provider skip | planned |
| 9 | 0.8.0 | Manual Sync & Rate Limit | Security/provider slice | Refresh sync with hard rate limit | planned |
| 10 | 0.9.0 | Provider Email Apply | Vertical slice | Pending email changes apply to provider APIs | planned |
| 11 | 0.10.0 | Confuse & Notification | Workflow/security slice | Confuse message generated only at send time | planned |
| 12 | 0.11.0 | MVP Hardening & Release | Release slice | Packaged desktop MVP with tests passing | planned |

**SIMPLE V1 gates:** 0.1.1 — master-password unlock only. 0.2.0 — no `workspace_members`; add `email_accounts.sync_status`. 0.3.0 — login UI + roles; no email gate / Share Workspace. 0.11.0 — MVP completes SIMPLE V1; FULL extension out of scope.

---

## 0.1.0 — Project Foundation

Status: current
Method: foundation slice
Goal: Establish a runnable project baseline without implementing product behavior prematurely.

Scope:
- Initialize Tauri 2.x + React + TypeScript + Rust workspace.
- Add backend/frontend test runners.
- Keep context-gen V3 files consistent.
- Add minimal app shell only if needed to verify the stack boots.

Backend:
- Create Rust crate structure under `src-tauri`.
- Add baseline `cargo test`.
- No vault crypto yet.

Frontend:
- Create React + TypeScript app structure.
- Add baseline `npm test`.
- No dashboard/product UI beyond boot validation.

Tests:
- `cargo test` passes.
- `npm test` passes.
- `context-gen build .` and `context-gen check-consistency .` pass.

Exit Criteria:
- Repo has real source files for context-gen to index.
- `.context/GLOBAL.md` is generated.
- No placeholder `[manual]` sections are left unreviewed for touched modules.

Constraints:
- Do not implement OAuth.
- Do not choose plain SQLite.
- Do not add production credential logic in this milestone.

---

## 0.1.1 — Vault Core

Status: planned
Method: security-led backend slice
Goal: Implement the minimum safe vault lifecycle before any credential UI depends on it.

Scope:
- Create/open/lock `.hvault`.
- Argon2id key derivation.
- SQLCipher integration.
- zeroize key material on lock/drop path.
- Profile metadata outside vault (`profiles.json`).
- Unlock with master password only (backend + minimal UI).
- Multi-profile path when ≥2 vaults exist.

Backend:
- `src-tauri/src/vault`
- Vault commands for create/open/lock/status.
- Error types that do not leak secrets.

Frontend:
- Minimal unlock/create flow: master password (+ vault selector if ≥2 profiles).
- No credential browsing yet.

Tests:
- Encrypt/decrypt round trip.
- Wrong password fails safely without timing hints.
- Lock zeroizes key material.
- Master password is never persisted.

Exit Criteria:
- A new vault can be created, opened, locked, and reopened.
- Multi-profile selection opens the correct `.hvault`.
- Tests cover key lifecycle invariants.

Constraints:
- No `drop()` as key clearing substitute; use `zeroize()`.
- No credential logging.
- Do not implement `workspace_members` or email login.

---

## 0.2.0 — Data Model & Migrations

Status: planned
Method: schema-led slice
Goal: Make V1 schema real and migration-backed before UI workflows rely on data.

Scope:
- Add migrations for V1 tables from spec **except** `workspace_members` (FULL defer).
- Add `email_accounts.sync_status` (`local_only` | `pending_sync` | `synced` | `sync_error`).
- `activity_log`: use `actor_note`; do not add `actor_email` in V1.
- Add data access boundaries for services, credentials, email accounts, domains, SSL certs, activity log, sync cache, app settings.
- Keep OAuth schema out of V1 runtime logic.

Backend:
- `src-tauri/migrations`
- Storage/repository functions needed by 0.3.0/0.4.0.

Frontend:
- None beyond smoke validation if needed.

Tests:
- Migration creates all required V1 tables.
- Schema has no `workspace_members`.
- `sync_status` column exists on `email_accounts`.
- Provider/auth scheme consistency is representable.
- OAuth providers are not routed through `service_credentials`.

Exit Criteria:
- Empty vault migrates to V1 schema.
- Schema matches `docs/vipavault-spec.md` with SIMPLE V1 auth exclusions above.

Constraints:
- Spec wins over code when conflicts appear.
- OAuth tables may be documented but OAuth flow remains Phase 2.
- Do not migrate or seed email allowlist tables in V1.

---

## 0.3.0 — App Shell & Roles

Status: planned
Method: UX workflow slice
Goal: Establish the admin/viewer experience boundary before write workflows exist.

Scope:
- Main app layout.
- SIMPLE V1 login screen: master password; vault dropdown if ≥2 profiles.
- Profile switcher placeholder or initial implementation.
- Per-machine `app_settings.json`: `machine_role`, `sync_enabled` only.
- `machine_role = admin | viewer`.
- Viewer badge and write-action gating.
- Document ops path: full `.hvault` copy for viewer (no in-app Share wizard).

Not in V1 scope:
- Email + master login.
- Settings → Share Workspace.
- Remember email / `last_login_email`.

Backend:
- Commands to read local app settings.
- Role-aware command guard helpers.

Frontend:
- Admin/viewer shell.
- Disabled write controls in viewer mode.
- No provider API calls when `sync_enabled = false`.

Tests:
- Login UI has no email field.
- Viewer mode disables write buttons.
- Viewer mode prevents write IPC paths.
- `sync_enabled = false` blocks provider API paths.

Exit Criteria:
- Same app can run as admin or viewer.
- Viewer cannot trigger writes through UI or backend command path.
- Copied vault on viewer machine works with `machine_role: viewer` without allowlist.

Constraints:
- Do not build user account system.
- Per-machine role config only.
- Sharing in V1 is file copy + manual `app_settings.json`, not subset export.

Open:
- Rotate master password wizard — milestone TBD (questionnaire).

---

## 0.4.0 — Dashboard Slice

Status: planned
Method: vertical slice
Goal: First user-visible end-to-end product workflow using local vault data.

Scope:
- Service/domain summary.
- Monthly cost total.
- Expiration alert thresholds.
- Provider breakdown.
- Viewer-safe dashboard.

Backend:
- Read-only query commands for dashboard data.
- Alert classification from timestamps.

Frontend:
- Dashboard for admin and viewer.
- Red/yellow/green alert states.

Tests:
- Alert thresholds: red `< 7 days`, yellow `7-30 days`, green otherwise.
- Viewer dashboard has no write action.

Exit Criteria:
- User can unlock vault and view a meaningful dashboard from local data.

Constraints:
- No auto-sync.
- Dashboard reads local data only.


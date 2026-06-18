# VipaVault

Desktop app for managing hosting, email, and domain credentials — built for operators and executives who need clarity without cPanel complexity.

**Stack:** Tauri 2.x · Rust · React · TypeScript · Vite · SQLCipher (`.hvault`)

**License:** AGPL-3.0 — see [LICENSE](LICENSE).

---

## What it does (V1)

- Encrypted local vault (`.hvault`) with master-password unlock
- cPanel and DirectAdmin provider sync (manual refresh, rate-limited)
- Admin vs viewer machine roles via per-machine `app_settings.json`
- CEO-friendly dashboard for services, domains, and email accounts

OAuth providers (M365, Google Workspace) and Share Package export are **Phase 2+**.

Full product spec: [`docs/vipavault-spec.md`](docs/vipavault-spec.md).

---

## Prerequisites

Development uses **WSL Debian** (see `.local/ENVIRONMENT.md` on your machine — not committed).

| Tool | Version (reference) |
|------|---------------------|
| Node.js | 24.x (via nvm) |
| Rust | 1.95+ |
| context-gen | pip install from [context-mapping](https://github.com/WhySchools/context-mapping) |

---

## Quick start (WSL)

```bash
# Clone
git clone https://github.com/yourorg/vipavault.git
cd vipavault

# Frontend deps
npm install

# Production frontend build (required for Tauri)
npm run build

# Run tests
npm run verify          # npm test + cargo test

# Dev desktop app (Vite on :1420)
npm run tauri dev
```

App data and `.hvault` files live outside the repo (e.g. `~/.vipavault/`), not in git.

---

## Project layout

```
vipavault/
  docs/vipavault-spec.md     # Product spec (source of truth)
  src/                       # React + TypeScript (Vite)
  src-tauri/                 # Rust backend + Tauri shell
    src/vault/               # SQLCipher engine (0.1.1+)
    src/providers/           # cPanel / DirectAdmin clients
    src/commands/            # Tauri IPC commands
  .context/                  # Architecture context (context-gen)
```

---

## Verification (foundation milestone)

```bash
npm run verify
context-gen build . --quiet
context-gen check-consistency .
```

Agent workflow and slice DAG: `.context/planning/AGENT_AUTOMATION_PLAN.md`.

---

## Security notes

- Master password is **never** stored
- Vault file uses **SQLCipher** (AES-256-GCM at file level)
- API tokens only — no cPanel main-password auth
- Viewer mode: read-only; no provider API when `sync_enabled = false`

See `AGENTS.md` for full architecture constraints.
# APP — Project shell & Tauri workspace

## [manual] Role

Tauri app bootstrap, workspace config, foundation milestone.

**Source:** `src-tauri/`, root config (`package.json`, `vite.config.ts`, …)  
**Generated AST:** `.context/generated/root.md`, `.context/generated/src-tauri.md`, `.context/generated/src-tauri_src.md`

## [manual] Design Decisions

- Tauri 2.x desktop target — không web-only deployment V1.
- `build_app()` trong `lib.rs` — testable without `dist/`; `generate_context!()` chỉ `main.rs` (`[[bin]] test = false`).
- Capabilities least-privilege: `src-tauri/capabilities/default.json` — `allow-app-status` + core defaults; window label `main`.
- `app_settings.json` per-machine ngoài vault: `machine_role`, `sync_enabled`.
- Data dir: `~/.vipavault/` (profiles + `.hvault` files).
- Login V1: backend master password only; UI vault name → user name (read-only) → password (≥2 vault → dropdown row ①). FULL: Share Package — questionnaire §Auth.

## [manual] Invariants — Phase: 0.1.0

- Không implement product behavior sớm trong 0.1.0 — chỉ baseline tests + context.
- `cargo test` không phụ thuộc `npm run build` (TDD phương án B).

## [manual] Test Strategy — lib.rs / shell

- `app_info()` — unit test package name + version.
- `build_app()` — compile test registers `app_status` handler.
- Vitest `app-version` — frontend boot panel.

## [manual] Milestone

- Current execution: **0.1.0 Project Foundation**
- Exit: `cargo test` + `npm test` + `context-gen build` + `check-consistency` pass.
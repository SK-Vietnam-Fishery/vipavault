# APP — Project shell & Tauri workspace

## [manual] Role

Tauri app bootstrap, workspace config, foundation milestone.

**Source:** `src-tauri/`, root config (`package.json`, `vite.config.ts`, …)  
**Generated AST:** `.context/generated/root.md`, `.context/generated/src-tauri.md`, `.context/generated/src-tauri_src.md`

## [manual] Design Decisions

- Tauri 2.x desktop target — không web-only deployment V1.
- `app_settings.json` per-machine ngoài vault.
- Data dir: `~/.vipavault/` (profiles + `.hvault` files).

## [manual] Invariants

- Không implement product behavior sớm trong 0.1.0 — chỉ baseline tests + context.

## [manual] Milestone

- Current execution: **0.1.0 Project Foundation**
- Exit: `cargo test` + `npm test` + `context-gen build` pass.
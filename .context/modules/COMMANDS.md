# COMMANDS — Tauri IPC

## [manual] Role

Tauri command surface giữa React frontend và Rust backend.

**Source:** `src-tauri/src/commands`  
**Generated AST:** `.context/generated/src-tauri_src_commands.md`  
**Load:** `context-gen load src-tauri/src/commands . --include-manual`

## [manual] Design Decisions

- Commands là boundary IPC — validate role (`admin`/`viewer`) trước write paths.
- Error response không chứa secret values.
- 0.1.0: chỉ `app_status` foundation command.
- 0.3.0: `get_app_settings`, `set_operator_email` (plaintext + regex format; không secret).

## [manual] Invariants & Constraints

- Viewer role → reject write commands at backend (không chỉ UI disable).
- Không log payload có password/token.

## [manual] Behavior chưa implement

- Vault create/open/lock commands — 0.1.1.
- App settings read/write (`operator_email`, `machine_role`) — 0.3.0.
- Role guard helpers — 0.3.0.
- Dashboard read-only queries — 0.4.0.
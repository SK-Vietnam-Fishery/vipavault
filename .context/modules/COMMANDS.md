# COMMANDS — Tauri IPC

## [manual] Role

Tauri command surface giữa React frontend và Rust backend.

**Source:** `src-tauri/src/commands`  
**Generated AST:** `.context/generated/src-tauri_src_commands.md`  
**Load:** `context-gen load src-tauri/src/commands . --include-manual`

## [manual] Design Decisions

- Commands là boundary IPC — validate role (`admin`/`viewer`) trước write paths.
- Error response không chứa secret values.
- M0: chỉ `app_status` foundation command.

## [manual] Invariants & Constraints

- Viewer role → reject write commands at backend (không chỉ UI disable).
- Không log payload có password/token.

## [manual] Behavior chưa implement

- Vault create/open/lock commands — M1.
- Role guard helpers — M3.
- Dashboard read-only queries — M4.
<!-- AUTO_START -->
# Global Context

> **[auto-generated — không sửa tay phần này]**

## [auto] Tech Stack

- Tauri v2
- Rust (backend)
- React + TypeScript (frontend)

## [auto] Module Index

Load file context của module cụ thể khi làm việc với nó:

- [`.`](.context/generated/root.md)
- [`src`](.context/generated/src.md)
- [`src-tauri`](.context/generated/src-tauri.md)
- [`src-tauri/src`](.context/generated/src-tauri_src.md)
- [`src-tauri/src/commands`](.context/generated/src-tauri_src_commands.md)
- [`src-tauri/src/confuse`](.context/generated/src-tauri_src_confuse.md)
- [`src-tauri/src/providers`](.context/generated/src-tauri_src_providers.md)
- [`src-tauri/src/sync`](.context/generated/src-tauri_src_sync.md)
- [`src-tauri/src/vault`](.context/generated/src-tauri_src_vault.md)
- [`src/test`](.context/generated/src_test.md)

## [auto] Rust Dependencies (Cargo.toml)

```
serde
serde_json
tauri
```

<!-- AUTO_END -->

---

<!-- MANUAL_START -->
## [manual] Context Map Layout

Bắt đầu từ `.context/README.md` nếu folder trông rối.

### Module context (đọc khi làm task)

| Module | File | Source |
|---|---|---|
| Vault | `.context/modules/VAULT.md` | `src-tauri/src/vault` |
| Providers | `.context/modules/PROVIDERS.md` | `src-tauri/src/providers` |
| Sync | `.context/modules/SYNC.md` | `src-tauri/src/sync` |
| Confuse | `.context/modules/CONFUSE.md` | `src-tauri/src/confuse` |
| Commands | `.context/modules/COMMANDS.md` | `src-tauri/src/commands` |
| Frontend | `.context/modules/FRONTEND.md` | `src/` |
| App shell | `.context/modules/APP.md` | Tauri workspace + root config |

### AST generated (context-gen)

- Path: `.context/generated/<path_with_underscores>.md`
- Refresh: `context-gen build . --quiet`

### Decisions & planning

- `.context/decisions/` — approved decision records
- `.context/planning/MILESTONES_REFERENCE.md` — full roadmap (không load mặc định)
- `.context/proposals/` — chưa approve (không load mặc định)

### Startup protocol

Xem `AGENTS.md` §2 — load order: GLOBAL → PROJECT → MILESTONES → TENSIONS → modules/
<!-- MANUAL_END -->


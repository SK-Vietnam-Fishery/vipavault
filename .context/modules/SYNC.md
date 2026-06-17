# SYNC — Manual provider sync

## [manual] Role

Refresh data từ provider API theo yêu cầu user; enforce rate limit.

**Source:** `src-tauri/src/sync`  
**Generated AST:** `.context/generated/src-tauri_src_sync.md`  
**Load:** `context-gen load src-tauri/src/sync . --include-manual`

## [manual] Design Decisions — Phase: V1

- **Manual sync only** — không auto-sync định kỳ. Tension `sync`.
- Rate limit cứng: **tối đa 1 lần / 10 phút / service**.
- Hiển thị `last_synced_at` cho user.
- Email tạo local: `pending_sync` → review → Apply → `synced`.

## [manual] Invariants & Constraints

- **KHÔNG** auto-sync background timer.
- **KHÔNG** bypass rate limit dù admin role.
- Viewer machine (`sync_enabled = false`) → block mọi provider API path.

## [manual] Test Strategy

- Rate limit: second call within 10 minutes rejected.
- Per-service isolation: service A sync không block service B.
- Viewer/sync_disabled guard ở command layer.

## [manual] Behavior chưa implement — M8+

- Sync worker + `cpanel_sync_cache` update.
- Pending email apply flow.
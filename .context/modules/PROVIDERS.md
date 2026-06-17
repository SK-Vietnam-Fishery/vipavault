# PROVIDERS — Provider routing & API clients

## [manual] Role

Route `provider_type` + `auth_scheme` tới đúng client và credential table.

**Source:** `src-tauri/src/providers`  
**Generated AST:** `.context/generated/src-tauri_src_providers.md`  
**Load:** `context-gen load src-tauri/src/providers . --include-manual`

## [manual] Design Decisions — Phase: V1

| provider_type | auth_scheme | credential table |
|---|---|---|
| cpanel | api_token | service_credentials |
| directadmin | api_token | service_credentials |
| m365 | oauth2_client_credentials | oauth_credentials (Phase 2) |
| google_workspace | oauth2_service_account | oauth_credentials (Phase 2) |

- cPanel + DirectAdmin dùng chung UAPI-style client, phân biệt qua `provider_type`.
- **Không** hardcode `"cpanel"` trong business logic — đọc từ DB.
- Unknown provider → `warn!` + skip, **không** panic.

## [manual] Invariants & Constraints — Phase: V1

- **KHÔNG** đọc `oauth_credentials` cho `auth_scheme = api_token` và ngược lại.
- **KHÔNG** gọi cPanel API bằng main password — chỉ API Token.
- **KHÔNG** auto-sync — chỉ khi user bấm Refresh (module `SYNC.md`).
- Viewer + `sync_enabled = false` → không gọi provider API.

## [manual] Test Strategy

- Mock HTTP: routing table chọn đúng client theo `provider_type`.
- Unknown provider logs warning, returns `Ok(())` skip.
- V1 ignore `m365` / `google_workspace` without error.

## [manual] Behavior chưa implement

- cPanel UAPI client (Email add/reset/list/delete) — M7+.
- DirectAdmin client — M7.
- OAuth clients — Phase 2.
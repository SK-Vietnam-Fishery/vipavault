# FRONTEND — React UI

## [manual] Role

React + TypeScript + Vite UI; admin vs viewer presentation.

**Source:** `src/`  
**Generated AST:** `.context/generated/src.md`, `.context/generated/src_test.md`  
**Load:** `context-gen load src . --include-manual`

## [manual] Design Decisions

- 0.1.0: boot shell only — validate stack wiring.
- Dashboard, Email Manager, Hosting List — vertical slices 0.3.0+.
- Viewer: badge "Chế độ xem", disable write controls.
- Login V1 UI (0.3.0): vault (label/▼) → **email label** (từ IPC `get_app_settings`) → master password. Lần đầu: ô email → backend `set_operator_email` + regex → label. Không auth email. FULL Share Package wizard: tooltip + checkbox “mật khẩu gói không khôi phục được”.

## [manual] Invariants & Constraints

- **KHÔNG** enable write buttons khi `machine_role = viewer`.
- Alert thresholds: đỏ < 7 ngày, vàng 7–30, xanh còn lại — test ở 0.4.0.

## [manual] Test Strategy

- Vitest + Testing Library.
- Viewer mode test: write actions disabled/absent.

## [manual] Behavior chưa implement

- Profile switcher, unlock flow — 0.1.1/0.3.0.
- Nested credential tree UI — 0.5.0+ (tier model).
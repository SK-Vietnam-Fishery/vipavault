# FRONTEND — React UI

## [manual] Role

React + TypeScript + Vite UI; admin vs viewer presentation.

**Source:** `src/`  
**Generated AST:** `.context/generated/src.md`, `.context/generated/src_test.md`  
**Load:** `context-gen load src . --include-manual`

## [manual] Design Decisions

- M0: boot shell only — validate stack wiring.
- Dashboard, Email Manager, Hosting List — vertical slices M3+.
- Viewer: badge "Chế độ xem", disable write controls.

## [manual] Invariants & Constraints

- **KHÔNG** enable write buttons khi `machine_role = viewer`.
- Alert thresholds: đỏ < 7 ngày, vàng 7–30, xanh còn lại — test ở M4.

## [manual] Test Strategy

- Vitest + Testing Library.
- Viewer mode test: write actions disabled/absent.

## [manual] Behavior chưa implement

- Profile switcher, unlock flow — M1/M3.
- Nested credential tree UI — M5+ (tier model).
<!-- AUTO_START | hash: d876eeca | built: 2026-08-14T14:26 -->
# Context: `src-tauri/src`

> **[auto-generated — không sửa tay phần này]**  
> Language: `rust`  
> Source files: 2

## [auto] Public Functions

### `app_info` (line 15)
```rust
pub fn app_info() -> AppStatus
```

### `build_app` (line 23) — Tauri builder for the desktop shell — safe to compile under `cargo test` (no `dist/`).
```rust
pub fn build_app() -> tauri::Builder<tauri::Wry>
```

## [auto] Structs

### `AppStatus`
_derives: Debug, Clone, PartialEq, Eq, Serialize_

| Field | Type |
|-------|------|
| `app_name` | `&'static str` |
| `version` | `&'static str` |

## [auto] Key Imports

```
use serde::Serialize;
use super::*;
```

<!-- AUTO_END -->

<!-- MANUAL_START -->
## [manual] Design Decisions
> Tại sao module này được thiết kế như vậy? Trade-off gì đã được chọn?

_Chưa có ghi chú._

## [manual] Invariants & Constraints
> Các quy tắc KHÔNG BAO GIỜ được vi phạm khi sửa code ở đây.

_Chưa có ghi chú._

## [manual] Test Strategy
> Cách test module này: unit/integration, mock gì, test case quan trọng nhất là gì?

_Chưa có ghi chú._

## [manual] Behavior chưa implement (TODO)
> Các behavior đã thiết kế nhưng chưa code. LLM đọc để không "sáng tác" sai hướng.

_Chưa có ghi chú._
<!-- MANUAL_END -->
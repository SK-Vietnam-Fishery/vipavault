<!-- AUTO_START | hash: e7bc6a3b | built: 2026-05-24T11:47 -->
# Context: `src-tauri/src`

> **[auto-generated — không sửa tay phần này]**  
> Language: `rust`  
> Source files: 2

## [auto] Public Functions

### `foundation_status` (line 15)
```rust
pub fn foundation_status() -> AppStatus
```

### `run` (line 22)
```rust
pub fn run()
```

## [auto] Structs

### `AppStatus`
_derives: Debug, Clone, PartialEq, Eq, Serialize_

| Field | Type |
|-------|------|
| `app_name` | `&'static str` |
| `milestone` | `&'static str` |

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
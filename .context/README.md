# Context Map Layout — VipaVault

> Bắt đầu từ đây khi `.context/` trông rối.
> Folder structure giúp human đọc; **load order** trong `AGENTS.md` mới là source of truth cho agent.

---

## Root Files (flat — load thường xuyên)

| File | Mục đích |
|---|---|
| `GLOBAL.md` | Stack, module index, invariants toàn project |
| `PROJECT.md` | Quyết định project-wide, không gắn 1 module |
| `MILESTONES.md` | Milestone hiện tại + execution scope |
| `TENSIONS_OPEN.md` | Tensions chưa resolve |
| `TENSIONS_ACTIVE.md` | Quyết định đã chốt, còn hiệu lực |
| `TENSIONS_HISTORY.md` | Archive — không load mặc định |

Không move các file governance này trừ khi cập nhật protocol trong `AGENTS.md`.

---

## Modules

Context theo **domain / implementation layer** (human-friendly):

```text
.context/modules/
```

| File | Source path | Khi nào load |
|---|---|---|
| `VAULT.md` | `src-tauri/src/vault` | Crypto, `.hvault`, lock/unlock |
| `PROVIDERS.md` | `src-tauri/src/providers` | cPanel/DirectAdmin routing |
| `SYNC.md` | `src-tauri/src/sync` | Manual sync, rate limit |
| `CONFUSE.md` | `src-tauri/src/confuse` | Confuse engine, notification |
| `COMMANDS.md` | `src-tauri/src/commands` | Tauri IPC |
| `FRONTEND.md` | `src/` | React UI, viewer gates |
| `APP.md` | project root / Tauri shell | Bootstrap, config |

Mỗi file module có `[manual]` — design decisions, invariants, test strategy.
**Đọc `[manual]` trước** khi sửa code.

AST facts (functions, structs, commands) nằm ở `generated/` — xem bên dưới.

---

## Generated (context-gen — không sửa tay)

```text
.context/generated/
```

- Tool `context-gen build` sinh/cập nhật vùng `<!-- AUTO_START -->` … `<!-- AUTO_END -->`
- Map path: `src-tauri/src/vault` → `generated/src-tauri_src_vault.md`
- Refresh: `context-gen build . --quiet`

```bash
context-gen load src-tauri/src/vault . --include-manual
```

---

## Decisions

Decision records đã approve (brainstorm → chốt):

```text
.context/decisions/
```

- `DECISION_VAULT_STORAGE.md` — SQLCipher vs KeePass/KDBX (APPROVED)

Khác với `TENSIONS_ACTIVE.md`: decision file có phân tích đầy đủ + dẫn chứng; tension entry là summary + constraint.

---

## Planning

Roadmap chi tiết, không load mặc định:

```text
.context/planning/
```

- `MILESTONES_REFERENCE.md` — full roadmap 0.1.0–0.11.0
- `MILESTONE_QUESTIONNAIRE.md` — 132 câu hỏi planning theo milestone (dev trả lời trước implement)
- `AGENT_AUTOMATION_PLAN.md` — DAG slice + gate verify + STOP rules (Mode: **AUTO** @ 0.1.0)
- `FOUNDATION_WORKFLOW.md` — tiêu chuẩn Tauri/Rust/React + quy trình commit + AUTO→MANUAL

Chỉ đọc khi đổi execution milestone hoặc human yêu cầu planning rộng / chạy agent tự động.

---

## Proposals

Ý tưởng chưa approve — **không phải active protocol**:

```text
.context/proposals/
```

Không load mặc định. Sau khi human approve → cập nhật `decisions/`, `TENSIONS_ACTIVE.md`, hoặc `planning/`.

- `PROPOSAL_CUSTOM_FIELDS.md` — trường tự quy định trên credential (1 cột JSON + JSON1 search) — **DRAFT**
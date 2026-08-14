# INDEX — tài liệu VipaVault

> Mục lục + quy tắc đặt file trong `docs/`.
> Tóm tắt sản phẩm: [README.md](README.md).
> Governance (milestone, tension, module): `.context/` — không nhân vào đây.

---

## Bắt đầu

| Đọc khi | File |
|---|---|
| Dự án là gì | [README.md](README.md) |
| Scope / schema / tính năng | [vipavault-spec.md](vipavault-spec.md) |
| Vì sao chọn X | [technical-decisions.md](technical-decisions.md) |
| Chạy / test / layout repo | [../README.md](../README.md) |
| Agent protocol | [../AGENTS.md](../AGENTS.md) |

Spec thắng mọi file khác trong `docs/`.

---

## Cây `docs/`

```
docs/
  README.md                 ← tóm tắt dự án
  INDEX.md                  ← file này
  vipavault-spec.md         ← SoT sản phẩm — KHÔNG đổi path
  technical-decisions.md    ← quyết định kỹ thuật đã chốt — KHÔNG đổi path
  design/                   ← UX / brand, chưa phải spec
  reviews/                  ← phản biện, đề xuất
  guides/                   ← how-to
  curriculum/               ← giáo trình 01–08
```

Gốc chỉ gồm: `README.md`, `INDEX.md`, `vipavault-spec.md`, `technical-decisions.md`.

---

## Product & kỹ thuật (gốc)

| File | Nội dung |
|---|---|
| [README.md](README.md) | Tóm tắt: ai dùng, V1 làm gì, trạng thái |
| [INDEX.md](INDEX.md) | Mục lục + quy tắc sắp xếp |
| [vipavault-spec.md](vipavault-spec.md) | Spec: kiến trúc, schema, tính năng, roadmap |
| [technical-decisions.md](technical-decisions.md) | Quyết định đã chốt + sơ đồ |

---

## `design/` — UX / brand (chưa SoT)

| File | Nội dung |
|---|---|
| [design/wireframe-inventory.md](design/wireframe-inventory.md) | Field / màn cần hiện; §13 chưa chốt |
| [design/logo-prompts.md](design/logo-prompts.md) | Brief + prompt gen logo |
| [design/vipavault-palette.css](design/vipavault-palette.css) | Palette của ứng dụng |


---

## `reviews/` — phản biện, chưa approve

| File | Nội dung |
|---|---|
| [reviews/pm-review-solutions.md](reviews/pm-review-solutions.md) | 10 vấn đề PM + phương án |

Sau khi human chốt: ghi vào spec, `technical-decisions.md`, hoặc `.context/decisions/`. Không để bản đã chốt chỉ sống ở đây.

---

## `guides/`

| File | Nội dung |
|---|---|
| [guides/context-map-guide.md](guides/context-map-guide.md) | Khi nào cập nhật `.context/` |

---

## `curriculum/`

| File | Nội dung |
|---|---|
| [curriculum/README.md](curriculum/README.md) | Mục lục giáo trình |
| [curriculum/01-foundations.md](curriculum/01-foundations.md) | Rust, TS/React, Tauri, SQLite |
| [curriculum/02-security-crypto.md](curriculum/02-security-crypto.md) | Argon2id, SQLCipher, threat model |
| [curriculum/03-product-design-thinking.md](curriculum/03-product-design-thinking.md) | JTBD, ADR, scope |
| [curriculum/04-system-design.md](curriculum/04-system-design.md) | IPC, model, provider, migration |
| [curriculum/05-build-mvp.md](curriculum/05-build-mvp.md) | Xây V1 theo milestone |
| [curriculum/06-sync-online.md](curriculum/06-sync-online.md) | Hướng sync đa máy (Phase 2+) |
| [curriculum/07-ship-beyond-mvp.md](curriculum/07-ship-beyond-mvp.md) | Package, signing, update |
| [curriculum/08-blind-spots.md](curriculum/08-blind-spots.md) | Điểm mù bảo mật / pháp lý / vận hành |

---

## Ngoài `docs/` (không copy vào đây)

| Chỗ | Việc |
|---|---|
| `.context/MILESTONES.md` | Phase + execution hiện tại |
| `.context/modules/` | Invariant theo module |
| `.context/decisions/` | ADR đã approve |
| `.context/proposals/` | Brainstorm kỹ thuật chưa approve |
| `.context/planning/` | Roadmap đầy đủ, questionnaire |

---

## Quy tắc đặt file

| Thư mục | Cho vào | Không cho vào |
|---|---|---|
| Gốc `docs/` | README, INDEX, spec, technical-decisions | Draft, review, prompt, bài học |
| `design/` | Wireframe, inventory UI, logo | Schema, ADR, milestone |
| `reviews/` | Phản biện / nhiều phương án | Decision đã approve |
| `guides/` | How-to trong repo | Giáo trình ngôn ngữ (→ `curriculum/`) |
| `curriculum/` | Bài đánh số 01–08 | Spec, ADR |

**Tên file:** `kebab-case.md`. Một chủ đề một file. Không `notes.md` / `misc.md`. Không gắn ngày vào tên. Chỉ `curriculum/` được đánh số.

**File mới:**

```
Đã chốt, ràng buộc sản phẩm?     → sửa vipavault-spec.md
Đã chốt, vì sao chọn kỹ thuật?   → technical-decisions.md hoặc .context/decisions/
Phản biện / nhiều phương án?     → docs/reviews/
Wireframe, logo, catalog UI?     → docs/design/
How-to maintain repo?            → docs/guides/
Bài học theo milestone?          → docs/curriculum/
Brainstorm schema/protocol?      → .context/proposals/
```

Không tạo file mới ở gốc `docs/` trừ khi human yêu cầu (như README / INDEX).
Không đổi path `vipavault-spec.md` hay `technical-decisions.md` nếu chưa cập nhật hết link.

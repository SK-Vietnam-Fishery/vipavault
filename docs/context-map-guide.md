# Context Map — Hướng dẫn Maintain

> File này không giải thích context-gen từ đầu.
> File này giải thích **khi nào cập nhật cái gì** và **tại sao**.
> Đọc cùng với AGENTS.md và TENSIONS.md.

---

## Tổng quan: layout phân tầng (tham khảo skvn-marine)

Xem **`.context/README.md`** — bản đồ đầy đủ.

```
.context/
  README.md              ← bắt đầu từ đây
  GLOBAL.md              ← context-gen [auto] + layout [manual]
  PROJECT.md             ← quyết định project-wide [manual]
  MILESTONES.md          ← milestone hiện tại
  TENSIONS_*.md          ← governance tensions V3
  modules/<MODULE>.md    ← [manual] protocol — source of truth cho agent
  generated/<path>.md    ← context-gen [auto] từ AST
  decisions/*.md         ← decision records đã approve
  planning/*.md          ← roadmap chi tiết (không load mặc định)
  proposals/             ← brainstorm chưa approve (không load mặc định)
```

**Rule cốt lõi:** `[auto]` trong `generated/` — tool lo. `[manual]` trong `modules/` — human lo. Không đảo ngược.

---

## Khi nào cập nhật gì

### 1. Sau khi thêm function / struct mới → KHÔNG cần làm gì

```bash
context-gen build . --quiet
```

Tool tự rebuild `[auto]`. Xong.

### 2. Sau khi đưa ra quyết định kiến trúc → Update `modules/<MODULE>.md` hoặc `decisions/`

Dấu hiệu cần update [manual]:
- Vừa quyết định "sẽ không làm X vì Y"
- Vừa chọn approach A thay vì approach B
- Vừa xác định một invariant mới ("function này không bao giờ được return null")
- Vừa quyết định behavior của Phase sau

Cách update:

```markdown
<!-- MANUAL_START -->
[manual] Design Decisions — Phase: V1
Dùng SQLCipher thay vì field-level encryption.
Lý do: copy .hvault sang máy viewer đơn giản hơn. Xem TENSIONS.md entry: storage.

[manual] Invariants & Constraints — Phase: all
- zeroize() key sau khi lock, không dùng drop()
- KHÔNG log credential dù debug
- rate limit sync: 1 lần / 10 phút / service

[manual] Behavior chưa implement — Phase: Phase2
- OAuth flow cho M365, Google Workspace
- Xem oauth_credentials table trong spec
<!-- MANUAL_END -->
```

**Không cần viết lại toàn bộ.** Chỉ thêm entry mới hoặc sửa entry liên quan đến quyết định vừa thảo luận.

### 3. Sau khi resolve tension → Update `TENSIONS_OPEN.md` / `TENSIONS_ACTIVE.md`

Khi một OPEN tension được human quyết định:

1. Move entry từ `TENSIONS_OPEN.md` → `TENSIONS_ACTIVE.md`
2. Đổi `Status: OPEN` → `Status: RESOLVED_ACTIVE`
3. Điền `Decision`, `Rationale`, `Constraint`, `Phase`

Brainstorm lớn → thêm file trong `.context/decisions/` trước, rồi summary vào `TENSIONS_ACTIVE.md`.

Nếu tension dẫn đến thay đổi protocol → update `modules/<MODULE>.md` trong cùng commit.

### 4. Khi constraint cũ hết hiệu lực → DEPRECATED marker, không xóa

```markdown
[manual] ~~Constraint: KHÔNG dùng oauth_credentials trong V1~~ — DEPRECATED @ abc1234
Lý do: Phase 2 bắt đầu, oauth_credentials table đã có.
Thay bằng: xem [manual] providers — Phase 2 section.
```

Git blame trên dòng này = full history tự động. Không cần tool thêm.

### 5. Khi đổi architecture lớn (đổi phase, thêm provider) → Update GLOBAL.md [manual]

`GLOBAL.md` [manual] giữ module index và dependency map. Cập nhật khi:
- Thêm module mới
- Đổi dependency giữa các module
- Bắt đầu phase mới

---

## Những gì KHÔNG cần update thủ công

| Thứ | Lý do |
|---|---|
| `[auto]` section trong bất kỳ file nào | context-gen build tự handle |
| Function signature, struct fields | AST parser tự detect |
| Import list | AST parser tự detect |
| Tauri command list | AST parser tự detect |

Nếu thấy mình đang sửa tay vào vùng `<!-- AUTO_START --> ... <!-- AUTO_END -->` → dừng, chạy `context-gen build` thay vào đó.

---

## Workflow thực tế sau một buổi thảo luận với AI

Đây là pattern thường xảy ra: thảo luận trong chat → ra quyết định → cần capture vào context.

**Bước 1 — Identify những quyết định đã đưa ra trong chat**

Câu hỏi để tự check:
- Vừa chọn A thay vì B vì lý do gì?
- Vừa xác định "cái này không được làm" không?
- Vừa push scope gì sang phase sau không?
- Vừa resolve mâu thuẫn giữa 2 approach không?

**Bước 2 — Mỗi quyết định → một trong 3 nơi**

```
Quyết định về approach / tại sao
        → TENSIONS_ACTIVE.md entry RESOLVED_ACTIVE
        → `.context/decisions/` nếu cần phân tích dài
        → `modules/<MODULE>.md` Design Decisions

Quyết định về constraint / invariant
        → `modules/<MODULE>.md` Invariants
        → `PROJECT.md` hoặc AGENTS.md §5 nếu project-wide

Quyết định về scope / phase
        → `modules/<MODULE>.md` Behavior chưa implement (Phase tag)
        → `planning/` nếu là roadmap milestone
```

**Bước 3 — Commit cùng với code hoặc commit riêng**

```bash
git add .context/
git commit -m "context: capture decisions từ session [date]"
```

Commit message ghi rõ là từ session nào để có thể trace lại nếu cần.

---

## Dấu hiệu context map đang stale

Nếu thấy những dấu hiệu sau → context map cần update:

- Agent propose lại thứ đã bị reject → TENSIONS.md thiếu RESOLVED entry
- Agent không biết tại sao code làm thế này → [manual] Design Decisions chưa có
- Agent implement sai invariant → [manual] Invariants chưa ghi hoặc stale
- [auto] section có function không còn tồn tại → chạy `context-gen build` là xong
- [manual] có `<!-- Viết tại đây -->` → placeholder chưa được điền

---

## Tần suất update thực tế

| Trigger | Cần update | Thời gian |
|---|---|---|
| Sau mỗi buổi thảo luận kiến trúc | TENSIONS.md + [manual] liên quan | 10–15 phút |
| Sau khi merge PR lớn | context-gen build + kiểm tra [manual] còn đúng không | 5 phút |
| Bắt đầu phase mới | GLOBAL.md [manual] + deprecated marker cho constraints cũ | 20–30 phút |
| Thêm function/struct thường | Không cần gì | 0 phút |

Context map không phải documentation — không cần perfect. Cần đủ để agent mới không đi sai hướng.

# Agent Automation Plan — VipaVault

> **Mục đích:** Kịch bản làm việc **tự động** cho agent (Cursor / Grok / CLI) — từng slice, gate kiểm tra, điểm dừng cần human.  
> **Không thay** `.context/MILESTONES.md` (execution boundary) hay `AGENTS.md` (protocol).  
> **Load khi:** human yêu cầu chạy agent tự động, hoặc bắt đầu / hoàn thành milestone.  
> **Tiêu chuẩn + AUTO/MANUAL:** `.context/planning/FOUNDATION_WORKFLOW.md`

**Status:** ACTIVE  
**Mode:** **AUTO** (foundation only — chuyển **MANUAL** khi activate 0.2.0+; xem `FOUNDATION_WORKFLOW.md` §1)  
**Active execution:** `0.1.0 — Project Foundation`  
**Cập nhật:** 2026-06-18  
**SIMPLE V1:** Auth/sharing theo `.context/MILESTONES.md` §SIMPLE V1 — agent không implement FULL / Share Package trong V1.

---

## 1. Mô hình tự động hóa

### 1.1 Vòng lặp agent (mỗi slice)

```text
START slice
  │
  ▼
Đọc AGENTS.md load order:
  GLOBAL → PROJECT → MILESTONES → TENSIONS_OPEN → TENSIONS_ACTIVE (filter)
  → modules/<relevant> → docs/vipavault-spec.md (nếu security/schema)
  → .local/ENVIRONMENT.md (lệnh toolchain)
  │
  ▼
Đọc section slice trong file này (PR id + Acceptance)
  │
  ▼
context-gen load <path> . --include-manual
  │
  ├── [manual] placeholder? → STOP, hỏi human
  │
  ▼
Tension mới vs spec/[manual]? → TENSIONS_OPEN.md (HIGH → HANDSHAKE, STOP)
  │
  ▼
TDD: test FAIL → implement → test PASS
  │
  ▼
Verification gate (§4)
  │
  ├── FAIL → fix, max 3 vòng; vẫn FAIL → STOP báo human
  │
  ▼
context-gen build . --quiet
  │
  ▼
git commit (code + .context/ nếu đổi context)
  │
  ▼
Đánh dấu slice Done trong §3; chuyển slice kế tiếp hoặc STOP nếu §5
END
```

### 1.2 Human vs agent

| Việc | Agent | Human |
|------|-------|-------|
| Implement slice trong active milestone | ✓ | |
| Activate / complete milestone | | ✓ |
| Resolve OPEN tension (Decision field) | | ✓ |
| Điền questionnaire khi blocker | | ✓ |
| Commit | ✓ (theo slice) | Review optional |
| Merge PR / push origin | | ✓ (mặc định) |
| Đổi spec / AGPL / business model | | ✓ |

### 1.3 Nguyên tắc slice

- **Một slice = một PR logic** (có thể nhiều file, một mục tiêu test được).
- Slice **không** vượt `Active Execution` trong `MILESTONES.md`.
- Ưu tiên **vertical slice** từ 0.3.0 trở đi; 0.1.0 là foundation horizontal.

### 1.4 TDD — bắt buộc mỗi slice

```text
RED   → viết / chạy test mô tả acceptance; phải FAIL (hoặc chưa compile) trước code
GREEN → implement tối thiểu để PASS
REFACTOR → dọn code; test vẫn PASS
```

| Layer | Tool | Ghi chú |
|-------|------|---------|
| Rust logic / commands | `cargo test` | `run()` + `generate_context!` → `#[cfg(not(test))]` để test không cần `dist/` |
| React UI | `npm test` (Vitest) | Test hành vi + `data-testid`; không snapshot mù |
| IPC / E2E sau 0.3.0 | mock Tauri + integration | 0.1.0 chỉ unit |

**Không** merge slice nếu chưa có test mới hoặc test acceptance cập nhật cho hành vi slice đó.

### 1.5 Commit — theo turn / slice

**Turn** = một lần agent hoàn thành **một slice** (S1, S2, …) và gate §4 PASS.

| Quy tắc | Chi tiết |
|---------|----------|
| **1 slice → 1 commit** | Không gộp S1+S3+S6 trong một commit trừ human yêu cầu |
| **Cuối turn** | Agent commit ngay sau `npm run verify` + `context-gen build` (nếu đổi source) |
| **Tách context** | Đổi `.context/` đáng kể → có thể commit riêng *sau* commit code cùng slice, hoặc gộp nếu cùng slice |
| **Không commit** | `.local/`, secrets, `.hvault`, `node_modules`, `dist/` (trừ CI artifact policy) |
| **Human push** | Agent `git commit` local; **push** do human (mặc định) |
| **Agent footer** | **Bắt buộc** dòng cuối mọi commit do agent — xem dưới |

**Format message (Conventional Commits + slice id):**

```text
<type>(<scope>): <mô tả ngắn> [0.1.0-Sn]

<body tùy chọn — RED→GREEN, test count>

Grok AI Agent working using https://github.com/WhySchools/context-mapping
```

**Attribution:** VipaVault dùng `context-gen` từ [WhySchools/context-mapping](https://github.com/WhySchools/context-mapping) (`.local/LOCAL_CONTEXT_GEN.md`). Footer ghi rõ agent + repo context — human commit thuần có thể bỏ footer.

| type | Khi nào |
|------|---------|
| `test` | Chỉ test RED (hiếm — thường kèm `feat`/`fix`) |
| `feat` | Hành vi mới / acceptance slice |
| `fix` | Sửa gate FAIL |
| `chore` | deps, tooling (`npm install`, scripts) |
| `docs` | README, planning không đổi code |
| `context` | Chỉ `.context/` (staleness, milestone note) |

**Ví dụ turn vừa rồi (tách 3 commit nếu làm đúng retro):**

```bash
# Turn 1 — S1
git add package.json package-lock.json
git commit -m "chore(deps): npm install for vitest baseline [0.1.0-S1]

Grok AI Agent working using https://github.com/WhySchools/context-mapping"

# Turn 2 — S3
git add src-tauri/Cargo.toml src-tauri/src/lib.rs src-tauri/src/main.rs package.json
git commit -m "fix(tauri): isolate generate_context to bin for cargo test TDD [0.1.0-S3]

build_app() in lib; [[bin]] test=false. npm run verify passes 3 rust tests.

Grok AI Agent working using https://github.com/WhySchools/context-mapping"

# Turn 3 — S6 + planning
git add src/App.tsx src/App.test.tsx .context/planning/AGENT_AUTOMATION_PLAN.md
git commit -m "feat(ui): boot health panel app-version with vitest [0.1.0-S6]

Grok AI Agent working using https://github.com/WhySchools/context-mapping"

# Turn 4 — milestone docs (cùng session, slice riêng hoặc gộp S6)
git add .context/MILESTONES.md .context/README.md .context/planning/AGENT_AUTOMATION_PLAN.md
git commit -m "context(planning): SIMPLE V1 milestones + agent automation plan

Grok AI Agent working using https://github.com/WhySchools/context-mapping"
```

**Một turn human chat dài:** agent vẫn **nhiều commit** theo slice đã chạy — không một commit khổng lồ cuối session.

**Trước mỗi commit:**

```bash
npm run verify
context-gen build . --quiet    # khi chạm src/
```

---

## 2. Trạng thái hiện tại (snapshot 2026-06-18)

### 2.1 Repo

| Kiểm tra | Trạng thái |
|----------|------------|
| Rust modules scaffold (`vault`, `providers`, …) | Có — stub |
| `cargo test` | **PASS** — `build_app()` trong lib; `generate_context!` chỉ `main.rs`; `[[bin]] test = false` |
| `npm test` | **PASS** — 2 tests (foundation shell + `app-version`) |
| `npm run verify` | **PASS** — `npm test` + `test:rust` |
| `npm build` / `dist/` | Cần cho `tauri dev` / release (S2) |
| SIMPLE V1 trong milestones | Đã cập nhật |
| OPEN tensions | 2× staleness `src-tauri_src*` (low) |

### 2.2 Lệnh chuẩn

Đọc **`.local/ENVIRONMENT.md`** — không hardcode npm/cargo nếu khác máy.

Thứ tự verify đề xuất sau khi có `node_modules`:

```bash
# 1. Frontend deps + build (Tauri cần dist/)
npm install
npm run build

# 2. Backend tests
cd src-tauri && cargo test

# 3. Frontend tests
npm test

# 4. Context
context-gen build . --quiet
context-gen check-consistency .
```

---

## 3. DAG — Milestone 0.1.0 (agent có thể chạy tuần tự)

Chạy **theo thứ tự**. Slice sau chỉ start khi slice trước **Acceptance** pass.

| ID | Slice | Mục tiêu | Acceptance (gate) | Phụ thuộc |
|----|-------|----------|-------------------|-----------|
| **0.1.0-S1** | Bootstrap deps | `npm install`, lock vitest | `npm test` chạy được (pass hoặc fail test logic, không "command not found") | — |
| **0.1.0-S2** | Frontend build | `npm run build` → `dist/` | Thư mục `dist/` tồn tại | S1 |
| **0.1.0-S3** | Cargo test green (TDD) | `#[cfg(not(test))]` trên `run()` — tests `app_info` / `app_status` đã RED→GREEN | `cargo test` exit 0 **không** cần `dist/` | S1 |
| **0.1.0-S4** | README + stack docs | Rewrite README đúng Tauri/Rust/React/SQLCipher | Human-readable; không Next.js/Prisma | S1 |
| **0.1.0-S5** | Context hygiene | Review staleness OPEN; update `modules/` nếu cần; `context-gen build` | `check-consistency` pass; staleness resolved hoặc ghi Decision | S3 |
| **0.1.0-S6** | Boot shell (TDD) | Test Vitest `app-version` → UI `data-testid="app-version"` | `npm test` pass; optional `tauri dev` smoke | S1 |
| **0.1.0-S7** | CI (optional) | GitHub Actions: npm + cargo + context-gen | Workflow green trên push — **chỉ nếu human approve Q-0.1.0-005** | S3, S5 |
| **0.1.0-S8** | Exit audit | Checklist §3.1 | Tất cả required ✓ → **hỏi human** activate 0.1.1 | S3–S6 |

### 3.1 Exit checklist 0.1.0 (required)

- [ ] `cargo test` pass
- [ ] `npm test` pass
- [ ] `context-gen build . --quiet` pass
- [ ] `context-gen check-consistency .` pass
- [ ] `.context/GLOBAL.md` tồn tại / generated
- [ ] Không placeholder `[manual]` trên module đã chạm
- [ ] README phản ánh stack thật

**Optional (human):** CI workflow; icon brand; `Cargo.lock` policy.

### 3.2 Ghi chú kỹ thuật S3 — **đã chốt: phương án B (TDD)**

| Phương án | Mô tả | Tradeoff |
|-----------|--------|----------|
| **A** | `npm run build` trước `cargo test` | Đơn giản; cargo phụ thuộc frontend build |
| **B** ✓ | `#[cfg(not(test))]` trên `run()` / `generate_context!` | **Cargo test độc lập `dist/`** — phù hợp TDD |

`npm run build` vẫn cần cho `tauri dev` / release (S2), không cho unit test Rust hàng ngày.

---

## 4. Verification gate (mọi slice)

```bash
# TDD gate mặc định (0.1.0+)
npm run verify

# Trước commit — thêm context khi đổi source
context-gen build . --quiet && context-gen check-consistency .
```

| Slice type | Thêm |
|------------|------|
| Frontend | `npm test` |
| Full stack | `npm run build` rồi `cargo test` |
| Context only | `context-gen check-consistency .` |

**Không commit** nếu: credential log, plain SQLite, OAuth V1, `workspace_members`, auto-sync.

---

## 5. Điểm dừng bắt buộc (STOP — hỏi human)

1. Hoàn thành **0.1.0-S8** → xin activate **0.1.1**.
2. OPEN tension **severity: high** hoặc security/schema conflict.
3. `[manual]` placeholder trong module đang sửa.
4. Câu questionnaire **chưa trả lời** và ảnh hưởng implement (xem §6).
5. 3 lần liên tiếp cùng một gate FAIL.
6. Task ngoài `Active Execution` (trừ human mở rộng scope).

---

## 6. Câu hỏi blocker — 0.1.0 (ưu tiên trả lời để agent chạy hết DAG)

Điền vào `MILESTONE_QUESTIONNAIRE.md` §0.1.0 hoặc block dưới đây.

| ID | Câu hỏi | Đề xuất agent (có thể override) | Blocker slice |
|----|---------|----------------------------------|---------------|
| Q-0.1.0-001 | WSL vs Windows native | **WSL Debian** per `.local/ENVIRONMENT.md` | — |
| Q-0.1.0-004 | cargo test vs `dist/` | **Phương án A** trước | S3 |
| Q-0.1.0-005 | CI trong 0.1.0? | **Defer** — S7 optional | S7 |
| Q-0.1.0-008 | Boot shell | **Version + app name** tối thiểu | S6 |
| Q-0.1.0-009 | `Cargo.lock` | **Commit lockfile** — reproducible desktop app | S3 |

```text
### Trả lời nhanh (human) — copy vào questionnaire khi rảnh:

Q-0.1.0-001: WSL Debian
Q-0.1.0-004: A — npm build trước cargo test
Q-0.1.0-005: Defer CI
Q-0.1.0-008: Minimal — app name + version
Q-0.1.0-009: Commit Cargo.lock
```

---

## 7. Hàng đợi sau 0.1.0 (preview — chưa activate)

Agent **không** implement cho đến khi human đổi `Active Execution`.

| Milestone | Slice gợi ý đầu tiên | Module context |
|-----------|----------------------|----------------|
| **0.1.1** | SQLCipher create/open/lock + master pass unlock | `VAULT.md`, spec §3 |
| **0.2.0** | Migration 001 — SIMPLE V1 (no `workspace_members`, +`sync_status`) | `VAULT.md`, questionnaire schema |
| **0.3.0** | Login UI SIMPLE + `app_settings.json` | `APP.md`, `FRONTEND.md` |

Chi tiết: `MILESTONES_REFERENCE.md`, questionnaire từng section.

---

## 8. Prompt mẫu cho human (copy vào agent)

### Chạy một slice

```text
Đọc .context/planning/AGENT_AUTOMATION_PLAN.md.
Thực hiện slice 0.1.0-S1 (Bootstrap deps).
Tuân AGENTS.md + SIMPLE V1. Dừng nếu §5.
```

### Chạy đến hết 0.1.0

```text
Đọc AGENT_AUTOMATION_PLAN.md.
Chạy tuần tự 0.1.0-S1 → S8; sau mỗi slice báo acceptance.
STOP tại S8 để tôi activate 0.1.1.
```

### Chạy verify only

```text
Đọc .local/ENVIRONMENT.md.
Chạy full verification gate §4 cho milestone 0.1.0.
Báo pass/fail từng lệnh.
```

---

## 9. Tài liệu tham khảo

| Tài liệu | Path |
|----------|------|
| Protocol agent | `AGENTS.md` |
| Execution boundary | `.context/MILESTONES.md` |
| Full roadmap | `.context/planning/MILESTONES_REFERENCE.md` |
| Câu hỏi planning | `.context/planning/MILESTONE_QUESTIONNAIRE.md` |
| Toolchain | `.local/ENVIRONMENT.md` |
| Business (local) | `.local/BUSINESS_MODEL.md` |
| Spec | `docs/vipavault-spec.md` |

---

## 10. Nhật ký chạy agent (điền khi thực thi)

| Ngày | Slice | Agent / human | Kết quả | Commit |
|------|-------|---------------|---------|--------|
| 2026-06-18 | 0.1.0-S1 | agent | PASS — `npm install` | |
| 2026-06-18 | 0.1.0-S3 | agent | PASS — TDD B: `build_app` + bin `test=false` | |
| 2026-06-18 | 0.1.0-S6 | agent | PASS — Vitest `app-version` RED→GREEN | |
| | 0.1.0-S2 | | pending — `npm run build` / `dist/` | |
| | 0.1.0-S4 | | pending — README | |
| | 0.1.0-S5 | | pending — context staleness | |
| | 0.1.0-S7 | | | |
| | 0.1.0-S8 | | | |

---

## 11. Cập nhật file này khi

- Human activate milestone mới → đổi §2, thêm DAG section (0.1.1, …).
- Blocker mới phát hiện → §2.1 + §6.
- Quy trình verify đổi → §4.
- Slice hoàn thành → §10.
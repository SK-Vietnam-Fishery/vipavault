# Foundation Workflow — Tauri / Rust / React

> **Phạm vi:** Chỉ **nền móng** desktop Tauri 2.x (milestone **0.1.0**, mở rộng tùy human tới **0.1.1**).  
> **Không** thay `AGENTS.md` (governance toàn project) hay spec sản phẩm.  
> **Chế độ:** **AUTO** (agent + slice + commit) **hiện tại** → **MANUAL** khi bắt đầu implement sản phẩm (từ **0.2.0** trở đi, trừ khi human nói sớm hơn).

**Cập nhật:** 2026-06-18  
**Liên quan:** `AGENT_AUTOMATION_PLAN.md` (DAG slice), `AGENTS.md` §4, `MILESTONES.md` §SIMPLE V1

### Toolchain agent (xác nhận)

| Thành phần | Nguồn |
|------------|--------|
| Agent IDE | **Grok AI Agent** (Cursor / Grok Build) |
| Context map / AST | **[context-mapping](https://github.com/WhySchools/context-mapping)** — CLI `context-gen` (local: `.local/LOCAL_CONTEXT_GEN.md`) |
| Governance | `AGENTS.md`, `.context/` |

Mọi commit do agent tạo trong **AUTO** (và commit agent trong **MANUAL**) phải ghi attribution ở footer — §5.2.

---

## 1. Hai chế độ làm việc

| | **AUTO** (hiện tại) | **MANUAL** (sau nền móng) |
|--|---------------------|---------------------------|
| **Khi nào** | `Active Execution: 0.1.0` (± 0.1.1 nếu human kéo dài foundation) | Human activate **0.2.0+** hoặc tuyên bố *"bắt đầu làm sản phẩm"* |
| **Ai dẫn** | Agent chạy slice DAG §`AGENT_AUTOMATION_PLAN` | Human chỉ task; agent không tự chạy S1→Sn |
| **Commit** | Agent: **1 slice = 1 commit** cuối turn | Human quyết định khi commit; agent đề xuất message |
| **TDD** | Bắt buộc RED→GREEN mỗi slice | Bắt buộc nhưng human review trước merge |
| **Push / PR** | Human | Human |
| **Dừng** | §STOP `AGENT_AUTOMATION_PLAN` | Tension HIGH, spec conflict, human pause |

### 1.1 Chuyển AUTO → MANUAL (checklist human)

Khi **một** điều kiện đúng → ghi ngày vào bảng §10, đổi `Mode:` trong `AGENT_AUTOMATION_PLAN.md`:

- [ ] Exit **0.1.0** đạt (§3.1 automation plan)
- [ ] (Tuỳ chọn) Exit **0.1.1** vault core
- [ ] Human activate **0.2.0 — Data Model & Migrations**
- [ ] Human message: *foundation xong, làm manual từ đây*

Sau chuyển: agent **không** tự chạy DAG; vẫn tuân `AGENTS.md`, TDD, verify gate khi được giao việc.

---

## 2. Tiêu chuẩn tham chiếu (research)

Áp dụng **có chọn lọc** cho foundation — không ôm hết enterprise day one.

### 2.1 Bảo mật

| Nguồn | Áp dụng foundation | Ghi chú VipaVault |
|-------|-------------------|-------------------|
| [Tauri 2 Security](https://v2.tauri.app/security/) — trust boundaries, IPC | **Bắt buộc** | Rust core = full trust; WebView chỉ qua IPC/commands |
| [Tauri Capabilities / Permissions](https://v2.tauri.app/security/capabilities/) | **0.1.0+** cấu hình tối thiểu | Principle of least privilege; không `allow-all` |
| [Tauri CSP](https://v2.tauri.app/security/csp/) | **0.3.0** shell | Restrict script/connect khi có UI thật |
| [Tauri Isolation Pattern](https://v2.tauri.app/concept/inter-process-communication/isolation/) | Defer post-MVP | Cân nhắc khi IPC surface lớn |
| `AGENTS.md` §5 | **Luôn** | SQLCipher, zeroize, không log credential, viewer gate |
| OWASP Desktop / local storage | **0.1.1+** | Master pass không lưu; file `.hvault` ngoài repo |

**Foundation invariant:** Mọi command Tauri mới phải qua `commands/` module, có test; không `eval`, không shell tùy tiện từ frontend.

### 2.2 Đa nền tảng (scalable trên PC)

| Nguồn | Áp dụng |
|-------|---------|
| [Tauri 2 — cross-platform](https://v2.tauri.app/) | Windows + Linux (WSL dev) + macOS khi packaging 0.11.0 |
| Path & config | `app_settings.json`, `~/.vipavault/` — không hardcode `C:\` hay chỉ POSIX |
| CI (defer 0.1.0 optional) | Matrix `ubuntu-latest`, `windows-latest` khi bật S7 |
| WebView OS-native | Không bundle WebView — chấp nhận theo Tauri security model |

**Không** nhầm “scalable” với server horizontal scale — desktop = **multi-platform + maintainable modules**.

### 2.3 Rust

| Nguồn | Foundation |
|-------|------------|
| [Rust API Guidelines — checklist](https://rust-lang.github.io/api-guidelines/checklist.html) | Naming, error types, `pub` có chủ đích |
| [Cargo project layout](https://doc.rust-lang.org/cargo/guide/project-layout.html) | `src-tauri/` package + `[[bin]]`; lib testable |
| `clippy` | `cargo clippy -- -D warnings` — **từ 0.1.0 exit** (warn → deny dần) |
| `rustfmt` | `cargo fmt --check` trước commit foundation |
| Edition 2021 | Giữ `Cargo.toml` |
| Secrets | `zeroize` crate tại 0.1.1 — không `drop()` thay thế |

**Layout foundation (hiện tại → mở rộng):**

```text
src-tauri/src/
  lib.rs          # build_app(), app_info — testable
  main.rs         # generate_context! only here
  commands/       # IPC surface
  vault/          # 0.1.1+
  providers/      # stub tới 0.7.0
```

### 2.4 React + TypeScript (Vite — không Next.js)

| Nguồn | Áp dụng | Khác Next.js |
|-------|---------|--------------|
| [Bulletproof React — structure](https://github.com/alan2207/bulletproof-react/blob/master/docs/project-structure.md) | Feature folders từ **0.3.0** | Không `app/` router, không RSC |
| TypeScript `strict` | **Bắt buộc** `tsconfig` | Giống Next khuyến nghị |
| Testing Library + Vitest | **0.1.0** | Thay Jest; chạy Vite-native |
| `@tauri-apps/api` | IPC typed wrappers từ 0.3.0 | Không `fetch` server actions |

**Tư vấn Next.js:**

| Next.js pattern | VipaVault (Tauri + Vite) |
|-----------------|---------------------------|
| `app/` routes | **Không dùng** — single-window SPA; route bằng React state hoặc `react-router` sau 0.3.0 nếu cần |
| Server Components | **Không** — mọi logic nhạy cảm ở **Rust** |
| `next/image`, SSR | **Không** — static Vite build → `dist/` |
| ESLint `next/core-web-vitals` | Dùng **ESLint flat + typescript-eslint + react-hooks** (thêm ở cuối 0.1.0) |
| Env `NEXT_PUBLIC_*` | Vite `import.meta.env` — **không** embed secret |

**Kết luận:** Lấy **discipline** từ Next ecosystem (strict TS, lint, component test), **không** copy App Router / SSR.

### 2.5 Chất lượng & Git

| Tiêu chuẩn | Foundation |
|------------|------------|
| [Conventional Commits](https://www.conventionalcommits.org/) | `type(scope): msg [0.1.0-Sn]` |
| `npm run verify` | Gate TDD trước commit |
| `context-gen build` | Sau đổi `src/` / `src-tauri/` |
| AGPL | Giữ `LICENSE`; commit không đổi license |

---

## 3. Quy trình AUTO (foundation — đang dùng)

```text
Human: "Chạy slice 0.1.0-Sn" (hoặc S1→S8)
        │
        ▼
Agent đọc: AGENTS.md → MILESTONES.md → FOUNDATION_WORKFLOW (file này)
           → AGENT_AUTOMATION_PLAN §3 slice Sn
           → .local/ENVIRONMENT.md
        │
        ▼
TDD RED → GREEN → REFACTOR
        │
        ▼
Quality gate (§4)
        │
        ▼
git commit (§5) — 1 slice
        │
        ▼
Cập nhật §10 nhật ký + AGENT_AUTOMATION_PLAN §10
        │
        ├── Sn < S8 → slice tiếp (nếu human yêu cầu tiếp tục)
        └── S8 → STOP, human activate 0.1.1 hoặc chuyển MANUAL
```

---

## 4. Quality gate (foundation)

Thứ tự chạy trước **mỗi commit** trong AUTO:

```bash
# 1. TDD
npm run verify

# 2. Rust style (khi đã thêm rustfmt/clippy script — S8 hoặc sớm hơn)
cd src-tauri && cargo fmt --check && cargo clippy -- -D warnings 2>/dev/null || true

# 3. Context (khi chạm source)
context-gen build . --quiet
context-gen check-consistency .
```

| Gate | 0.1.0 bắt buộc | 0.1.1 thêm |
|------|----------------|------------|
| `npm test` | ✓ | ✓ |
| `cargo test` (không cần `dist/`) | ✓ | ✓ |
| `cargo clippy` | Khuyến nghị → bắt buộc exit | ✓ |
| `cargo fmt --check` | Khuyến nghị | ✓ |
| `npm run build` | ✓ (S2 — `tauri dev`) | ✓ |

---

## 5. Quy trình commit (foundation AUTO)

### 5.1 Quy tắc

1. **Một slice hoàn thành → một commit** (xem `AGENT_AUTOMATION_PLAN.md` §1.5).
2. **Không** commit half-red test (trừ WIP branch — foundation dùng `main` local, tránh WIP).
3. **Tách** `context` commit nếu slice chỉ docs `.context/`; có thể gộp nếu cùng slice.
4. **Body commit** ghi: tests pass count, slice id.
5. **Footer bắt buộc** (agent commits) — dòng cuối message, không bỏ:

```text
Grok AI Agent working using https://github.com/WhySchools/context-mapping
```

### 5.2 Template

```text
<type>(<scope>): <english short title> [0.1.0-Sn]

- TDD: <what was tested>
- Verify: npm run verify (N tests)

Grok AI Agent working using https://github.com/WhySchools/context-mapping
```

**Ví dụ đầy đủ:**

```bash
git commit -m "fix(tauri): isolate generate_context to bin for cargo test [0.1.0-S3]

- TDD: cargo test without dist/
- Verify: npm run verify (5 tests)

Grok AI Agent working using https://github.com/WhySchools/context-mapping"
```

| scope ví dụ | Dùng cho |
|-------------|----------|
| `tauri` | Rust / IPC / `src-tauri/` |
| `ui` | `src/` React |
| `deps` | package.json / lock |
| `context` | `.context/` only |
| `tooling` | scripts, ci, vite config |

### 5.3 Không commit

`.local/`, `node_modules/`, `dist/`, `target/`, `*.hvault`, env secrets.

---

## 6. Quy trình MANUAL (từ 0.2.0 / product)

```text
Human mô tả task (có thể 1 milestone, 1 feature)
        │
        ▼
Agent đọc context theo AGENTS.md — KHÔNG tự kích hoạt slice DAG
        │
        ▼
Human/Agent thống nhất: test nào RED trước
        │
        ▼
Implement → human review diff
        │
        ▼
Human: "commit" → agent soạn message, human approve hoặc tự commit
        │
        ▼
Human activate milestone / merge khi sẵn sàng
```

**Giữ nguyên:** TDD, security invariants, SIMPLE V1, tension protocol.

**Khác AUTO:** Không auto-chạy S1→S8; không auto-commit cuối turn; có thể nhiều task nhỏ trong một PR human-crafted.

---

## 7. Roadmap tiêu chuẩn theo milestone foundation

| Milestone | Tiêu chuẩn bổ sung | Mode |
|-----------|---------------------|------|
| **0.1.0** | Tauri boot, Vitest, cargo test độc lập `dist/`, capabilities skeleton, ESLint (optional) | AUTO |
| **0.1.1** | SQLCipher, Argon2id, zeroize, vault tests, clippy/fmt bắt buộc | AUTO* hoặc MANUAL nếu human chuyển sớm |
| **0.2.0+** | Schema, migrations, product IPC | **MANUAL** |

\* Human quyết định giữ 0.1.1 trong AUTO hay không khi activate.

---

## 8. Việc cần làm (foundation backlog — standards)

| ID | Hạng mục | Slice gợi ý | Tiêu chuẩn |
|----|----------|-------------|------------|
| F-01 | `tauri.conf.json` capabilities least-privilege | 0.1.0-S9 mới | Tauri Security |
| F-02 | ESLint + typescript-eslint + react-hooks | 0.1.0-S9 | Bulletproof / Next discipline |
| F-03 | `cargo fmt` + `clippy` in `npm run verify` | 0.1.0-S8 | Rust API guidelines |
| F-04 | `rust-toolchain.toml` pin Rust | 0.1.1 | Reproducible builds |
| F-05 | CSP draft in tauri config | 0.3.0 | Tauri CSP |
| F-06 | CI matrix win + linux | 0.1.0-S7 optional | Multi-platform |

---

## 9. Tài liệu tham khảo (link)

| Chủ đề | URL |
|--------|-----|
| Tauri 2 Security | https://v2.tauri.app/security/ |
| Tauri Capabilities | https://v2.tauri.app/security/capabilities/ |
| Tauri CSP | https://v2.tauri.app/security/csp/ |
| Rust API Guidelines | https://rust-lang.github.io/api-guidelines/checklist.html |
| Cargo layout | https://doc.rust-lang.org/cargo/guide/project-layout.html |
| Bulletproof React | https://github.com/alan2207/bulletproof-react |
| Conventional Commits | https://www.conventionalcommits.org/ |
| VipaVault spec security | `docs/vipavault-spec.md` §3 |
| Agent automation DAG | `.context/planning/AGENT_AUTOMATION_PLAN.md` |
| context-gen / context-mapping | https://github.com/WhySchools/context-mapping |

---

## 10. Nhật ký chế độ

| Ngày | Mode | Ghi chú |
|------|------|---------|
| 2026-06-18 | **AUTO** | Foundation workflow published; active 0.1.0 |
| | MANUAL | (điền khi activate 0.2.0+) |

---

## 11. Prompt mẫu

**AUTO — tiếp foundation:**

```text
Mode: AUTO. Đọc FOUNDATION_WORKFLOW.md + AGENT_AUTOMATION_PLAN.md.
Chạy slice 0.1.0-S2. TDD + commit §5 khi PASS.
```

**Chuyển MANUAL:**

```text
Foundation xong. Chuyển MANUAL từ 0.2.0.
Cập nhật §10 FOUNDATION_WORKFLOW và AGENT_AUTOMATION_PLAN Mode.
```

**MANUAL task:**

```text
Mode: MANUAL. Task: <mô tả>. TDD trước. Không auto-commit.
```
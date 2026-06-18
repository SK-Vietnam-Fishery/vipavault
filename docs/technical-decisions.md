# Quyết định kỹ thuật — VipaVault

> **Đối tượng:** Developer (ưu tiên). Agent có thể đọc file này để nắm bối cảnh, nhưng **source of truth chi tiết** vẫn là `docs/vipavault-spec.md` và `.context/`.
>
> **Cập nhật:** 2026-06-18 · Foundation **0.1.0 done** · Chờ activate **0.1.1**

---

## 1. Mục đích tài liệu

Tài liệu này ghi lại **các quyết định đã chốt** — không phải brainstorm. Mỗi mục gồm:

- **Vấn đề** cần giải quyết
- **Phương án đã chọn** và phương án bị loại
- **Lý do** (trade-off thực tế)
- **Ràng buộc** — không được phá trừ khi mở tension mới

Chi tiết vault storage (SQLCipher vs KeePass): [`.context/decisions/DECISION_VAULT_STORAGE.md`](../.context/decisions/DECISION_VAULT_STORAGE.md).

---

## 2. Tổng quan kiến trúc

VipaVault là **desktop app** (không phải web SaaS): Tauri 2 bọc React SPA, logic nhạy cảm chạy trong Rust.

```mermaid
flowchart TB
  subgraph UI["WebView — hạn chế tin cậy"]
    React["React + TypeScript\n(Vite → dist/)"]
  end

  subgraph Core["Rust — tin cậy đầy đủ"]
    IPC["commands/ — Tauri IPC"]
    Vault["vault/ — SQLCipher .hvault"]
    Prov["providers/ — cPanel, DirectAdmin"]
    Sync["sync/ — manual refresh + rate limit"]
    Conf["confuse/ — obfuscate khi gửi thông báo"]
  end

  subgraph External["Bên ngoài app"]
    Hvault["~/.vipavault/*.hvault"]
    Settings["app_settings.json\n(per-machine)"]
    APIs["Provider APIs\ncPanel / DirectAdmin UAPI"]
  end

  React -->|"invoke() qua ACL"| IPC
  IPC --> Vault
  IPC --> Prov
  IPC --> Sync
  Vault --> Hvault
  IPC --> Settings
  Prov --> APIs
  Sync --> APIs
```

### Stack đã chốt

| Lớp | Công nghệ | Ghi chú |
|-----|-----------|---------|
| Shell | Tauri 2.x | Cross-platform desktop |
| Backend | Rust 2021 | `src-tauri/` |
| Frontend | React 18 + TypeScript + Vite | **Không** Next.js, không SSR |
| Storage | SQLCipher (`.hvault`) | Toàn file AES-256-GCM |
| KDF | Argon2id | Milestone 0.1.1 |
| License | AGPL-3.0 | Xem `LICENSE` |

---

## 3. SIMPLE V1 — Auth & chia sẻ

Human approve 2026-06. Đây là ranh giới sản phẩm cho toàn Phase V1.

### 3.1 Đăng nhập

**Tách backend và UI** — hay nhầm lẫn ở đây:

| Lớp | Quy tắc |
|-----|---------|
| **Backend** | Chỉ **master password** là yếu tố knowledge để mở SQLCipher. 1 vault → không cần chọn profile; ≥2 vault → cần `profile_id` / path. |
| **UI (màn login)** | Luôn hiển thị theo thứ tự: **tên vault** → **tên người dùng** → **ô mật khẩu**. Hai dòng đầu là ngữ cảnh (read-only), không thay thế xác thực. |

```mermaid
flowchart TB
  subgraph UI["Màn login — luôn cùng thứ tự"]
    V["① Tên vault\nprofiles.json display_name"]
    U["② Tên người dùng\nread-only — xem §3.1.1"]
    P["③ Ô mật khẩu\nmaster password — input duy nhất khi 1 vault"]
    V --> U --> P
  end

  subgraph Backend["Backend unlock"]
    P --> Derive[Argon2id → SQLCipher]
    Multi{≥ 2 vault?}
    Multi -->|Có| Pick[profile_id từ bước ①]
    Multi -->|Không| Single[profile mặc định]
    Pick --> Derive
    Single --> Derive
    Derive --> App([App shell])
  end
```

#### 3.1.1 Nguồn hiển thị trên UI

| Field UI | 1 vault | ≥ 2 vault | Nguồn dữ liệu |
|----------|---------|-----------|----------------|
| Tên vault | Label read-only | Dropdown / select | `profiles.json` → `display_name` |
| Tên người dùng | Label read-only | Label read-only | `app_settings.json` → `operator_display_name`; fallback tên OS user |
| Mật khẩu | Input | Input | User nhập — **không lưu** |

```
1 vault (UI):
┌──────────────────────────────────┐
│ Vault:      Công ty A            │  ← read-only
│ Người dùng: Tuấn (IT)            │  ← read-only
│ Mật khẩu:   [ •••••••••••••••• ] │  ← input duy nhất (backend)
│         [ Mở khóa ]              │
└──────────────────────────────────┘

≥ 2 vault (UI):
┌──────────────────────────────────┐
│ Vault:      [ Công ty A      ▼ ] │
│ Người dùng: Tuấn (IT)            │
│ Mật khẩu:   [ •••••••••••••••• ] │
│         [ Mở khóa ]              │
└──────────────────────────────────┘
```

| Chốt | Không làm trong V1 |
|------|-------------------|
| Backend: chỉ master password mở `.hvault` | Email login, user account + `password_hash` |
| UI: luôn có tên vault + tên người dùng **trước** ô pass | Ẩn tên vault khi 1 profile |
| ≥2 vault: dropdown **chỉ** ở dòng tên vault | PIN quick unlock |
| | `workspace_members`, allowlist email |
| | Remember email / `last_login_email` |

### 3.2 Vai trò & chia sẻ file

```mermaid
flowchart TB
  AdminPC["Máy IT — admin\napp_settings.json"]
  ViewerPC["Máy CEO — viewer\napp_settings.json"]
  HvaultFile["file.hvault\n(copy nguyên file)"]

  AdminPC -->|"machine_role: admin"| HvaultFile
  HvaultFile -->|"Copy USB / sync folder"| ViewerPC
  ViewerPC -->|"machine_role: viewer\nsync_enabled: false"| ReadOnly["Chỉ đọc — không gọi provider API"]

  style ReadOnly fill:#fff3cd
```

| Chốt | Không làm trong V1 |
|------|-------------------|
| `machine_role` + `sync_enabled` trong `app_settings.json` **ngoài vault** | Share Package wizard |
| Chia sẻ = copy **toàn bộ** `.hvault` | Partial vault export |
| Viewer: UI + backend chặn write | Activation-code claim |
| Audit: `activity_log.actor_note` | `actor_email` |

Chi tiết FULL extension: `.context/planning/MILESTONE_QUESTIONNAIRE.md` §Auth.

---

## 4. Lưu trữ & mã hóa

### 4.1 SQLCipher — không KeePass backend

```mermaid
flowchart LR
  subgraph Chosen["✓ Đã chọn"]
    SQL["SQLCipher .hvault\n1 file, quan hệ + mã hóa"]
  end
  subgraph Rejected["✗ Loại"]
    KDBX["KDBX / KeePass hybrid"]
    Field["SQLite thường + field AES"]
  end

  SQL --> Copy["Copy file → máy viewer"]
  KDBX -.->|"dual-store, sync risk"| X1[Loại]
  Field -.->|"maintenance mỗi column mới"| X2[Loại]
```

**Lý do chính:** Use case “copy `.hvault` sang máy sếp” đơn giản với file-level encryption; dashboard, sync, audit, confuse đều cần SQL quan hệ native.

### 4.2 Phân tầng dữ liệu (3 lớp)

```mermaid
flowchart TB
  T1["Tầng 1 — Company profile\n(multi-profile / nhiều .hvault)"]
  T2["Tầng 2 — Service group\nhosting, email, domain, license"]
  T3["Tầng 3 — Sub-credential\nMariaDB, SSH, cPanel token…"]

  T1 --> T2 --> T3
```

Implement qua **FK trong SQL + UI tree** — không map sang KDBX groups.

### 4.3 Invariants bảo mật (luôn áp dụng)

- Master password **không** lưu ở đâu trong app
- Key material clear bằng **`zeroize`** — không dùng `drop()` thay thế (từ 0.1.1)
- **Không** log credential kể cả debug
- cPanel/DirectAdmin: chỉ **API Token** — không main password

---

## 5. Provider & đồng bộ

### 5.1 Routing V1

```mermaid
flowchart TD
  Svc["service.provider_type"] --> Match{match}
  Match -->|cpanel / directadmin| Impl["UAPI client chung\nphân nhánh theo type"]
  Match -->|m365 / google / plesk| Skip["warn + skip\nkhông panic"]
  Impl --> Cred["service_credentials\nauth_scheme: api_token"]
  Skip -.-> OAuth["oauth_credentials\nPhase 2 only"]
```

| `provider_type` | `auth_scheme` | Bảng credential | Phase |
|-----------------|---------------|-----------------|-------|
| `cpanel` | `api_token` | `service_credentials` | V1 |
| `directadmin` | `api_token` | `service_credentials` | V1 |
| `m365` | `oauth2_client_credentials` | `oauth_credentials` | 2 |
| `google_workspace` | `oauth2_service_account` | `oauth_credentials` | 2 |

### 5.2 Sync — manual only

```mermaid
sequenceDiagram
  participant User
  participant UI
  participant Sync as sync/
  participant API as Provider API

  User->>UI: Bấm Refresh
  UI->>Sync: sync_service(id)
  alt < 10 phút kể từ lần trước
    Sync-->>UI: Rate limit — từ chối
  else OK + sync_enabled + admin
    Sync->>API: UAPI call
    API-->>Sync: Response
    Sync-->>UI: Cập nhật local
  end
```

**Lý do:** cPanel không công bố rate limit policy; block IP = mất toàn bộ access hosting. Auto-sync 15 phút → risk không đối xứng.

**Ràng buộc:** Tối đa **1 lần / 10 phút / service**; không auto-sync nền.

---

## 6. Foundation 0.1.0 — Quyết định đã implement

Milestone **0.1.0 done** (2026-06-18). Các quyết định dưới đây ảnh hưởng trực tiếp cách dev làm việc hàng ngày.

### 6.1 Tách `lib.rs` / `main.rs` — cargo test độc lập `dist/`

**Vấn đề:** `tauri::generate_context!()` cần `dist/` → `cargo test` fail nếu chưa `npm run build`.

```mermaid
flowchart LR
  subgraph Before["✗ Phương án A — loại"]
    B1["npm run build"] --> B2["cargo test"]
  end
  subgraph After["✓ Phương án B — đã chọn"]
    Lib["lib.rs: build_app()"]
    Main["main.rs: generate_context!()"]
    Test["cargo test — chỉ lib"]
    Lib --> Test
    Main --> Run["tauri dev / release"]
  end
```

| File | Trách nhiệm |
|------|-------------|
| `src-tauri/src/lib.rs` | `build_app()`, `app_info()`, unit tests |
| `src-tauri/src/main.rs` | Chỉ `generate_context!()` + `run()` |
| `Cargo.toml` | `[[bin]] test = false` — bin không chạy trong test harness |

**Hệ quả:** `npm run verify` = Vitest + `cargo test` **không** cần build frontend trước. `npm run build` / `build:check` vẫn bắt buộc cho `tauri dev` và release.

### 6.2 Tauri Capabilities — least privilege

```mermaid
flowchart TB
  Win["Window label: main"] --> Cap["capabilities/default.json"]
  Cap --> Core["core:* defaults"]
  Cap --> Cmd["allow-app-status"]
  Build["build.rs AppManifest"] --> Perm["permissions/autogenerated/\napp_status.toml"]
  Perm --> Cmd
```

- Mọi IPC command mới → khai báo trong `build.rs` `AppManifest::commands(&[...])`
- Permission tương ứng trong `capabilities/` — **không** `allow-all`
- CSP: `null` ở 0.1.0; siết ở **0.3.0** khi có UI thật

### 6.3 Chiến lược test

```mermaid
flowchart LR
  subgraph Gate["npm run verify"]
    V["Vitest\nUI behavior"]
    R["cargo test\nRust logic"]
  end
  V --> Pass{PASS?}
  R --> Pass
  Pass -->|Yes| Commit[Commit slice]
  Pass -->|No| Fix[Fix — max 3 vòng]
```

| Lớp | Tool | Foundation coverage |
|-----|------|---------------------|
| Rust | `cargo test` | `app_info`, `build_app`, `app_status` |
| React | Vitest + Testing Library | Boot panel `data-testid="app-version"` |
| IPC integration | Mock Tauri | Từ 0.3.0 |
| E2E | — | Post-MVP |

**TDD bắt buộc:** RED → GREEN → REFACTOR mỗi slice.

### 6.4 Cấu trúc module Rust (hiện tại)

```mermaid
flowchart TB
  lib["lib.rs"]
  lib --> commands
  lib --> vault["vault/ (stub → 0.1.1)"]
  lib --> providers["providers/ (stub)"]
  lib --> sync["sync/ (stub)"]
  lib --> confuse["confuse/ (stub)"]
  commands --> app_status["app_status()"]
```

Stub có chủ đích — product logic theo milestone, không implement sớm trong 0.1.0.

### 6.5 Dev environment

| Quyết định | Chi tiết |
|------------|----------|
| Dev OS | **WSL Debian** (không cài toolchain vào repo) |
| Node | 24.x via nvm |
| Vite dev port | **1420** (`tauri.conf.json` `devUrl`) |
| App data | `~/.vipavault/` — ngoài git |
| `docs/` | Tracked trong git (spec, tài liệu này) |
| `dist/` | Gitignored — sinh bởi `npm run build` |

Lệnh chuẩn trên máy dev: `.local/ENVIRONMENT.md` (gitignored, per-machine).

### 6.6 Context map (dev + agent)

```mermaid
flowchart TB
  Spec["docs/vipavault-spec.md\n(product)"]
  TD["docs/technical-decisions.md\n(this file)"]
  Modules[".context/modules/*.md\n[manual] decisions"]
  Gen[".context/generated/*.md\n[auto] AST"]
  Agent["AGENTS.md\n(agent protocol)"]

  Spec --> Modules
  TD --> Modules
  Code["src/ + src-tauri/"] -->|"context-gen build"| Gen
  Modules --> Agent
  Gen --> Agent
```

- **Dev đọc trước:** spec → file này → module liên quan
- **Agent đọc thêm:** `AGENTS.md`, `MILESTONES.md`, tensions
- **Không sửa tay** phần `<!-- AUTO_START -->` trong generated

---

## 7. Vai trò per-machine (không user account)

```mermaid
stateDiagram-v2
  [*] --> Admin: machine_role = admin
  [*] --> Viewer: machine_role = viewer

  Admin: Write UI enabled
  Admin: Provider API khi sync_enabled
  Viewer: Write UI disabled
  Viewer: Backend reject write IPC
  Viewer: Không gọi provider khi sync_enabled = false
```

**Lý do:** Chỉ 2 persona thực tế (IT + CEO). User account system = over-engineering.

---

## 8. Confuse engine (preview — 0.10.0)

| Chốt | Chi tiết |
|------|----------|
| Vault lưu password **thật** | Không lưu confuse string |
| Confuse chỉ sinh lúc gửi thông báo | Prefix/suffix từ `app_settings` |
| `confuse_used` trong `activity_log` | Audit sau khi gửi |

---

## 9. Việc cố ý hoãn (deferred)

```mermaid
timeline
  title Lộ trình kỹ thuật (rút gọn)
  section Done
    0.1.0 : Tauri boot, test, context baseline
  section Next
    0.1.1 : SQLCipher create/open/lock
    0.2.0 : Schema + migrations SIMPLE V1
    0.3.0 : Login UI + role gates + CSP
  section V1
    0.7.0 : Provider routing
    0.8.0 : Manual sync + rate limit
    0.11.0 : MVP package
  section Later
    Phase 2 : OAuth M365/Google
    FULL : Share Package, workspace_members
```

| Hạng mục | Milestone / Phase | Ghi chú |
|----------|-------------------|---------|
| Vault crypto | 0.1.1 | Argon2id, SQLCipher, zeroize |
| CI GitHub Actions | Optional S7 | Q-0.1.0-005 defer |
| ESLint flat config | F-02 | Cuối foundation |
| `clippy` trong verify | F-03 | Khuyến nghị → bắt buộc 0.1.1 exit |
| CSP strict | 0.3.0 | F-05 |
| OAuth flow | Phase 2 | Schema có chỗ, runtime chưa |
| Share Package | Post 0.11.0 | `TD-SHARE-*` |
| KDBX import/export | Phase 2+ optional | Không thay Share Package |

---

## 10. Bảng tra cứu nhanh

| Câu hỏi | Trả lời ngắn |
|---------|--------------|
| Tại sao không Next.js? | Desktop Tauri + Vite SPA — không cần SSR/RSC |
| `cargo test` cần `dist/` không? | **Không** (phương án B) |
| `tauri dev` cần gì? | `npm run build` hoặc `npm run dev` (Vite :1420) |
| Credential OAuth lưu đâu? | `oauth_credentials` — Phase 2, không `service_credentials` |
| Viewer có sync được không? | Chỉ khi `sync_enabled: true` **và** admin policy cho phép — mặc định viewer copy file + `sync_enabled: false` |
| Login UI 1 vault chỉ 1 ô pass? | **Backend** đúng; **UI** vẫn hiện tên vault + tên người dùng trước ô pass |
| Spec conflict với code? | **Sửa code**; sửa spec → tension HIGH |
| Agent có tự activate milestone không? | **Không** — human activate |

---

## 11. Tài liệu liên quan

| Tài liệu | Đối tượng | Nội dung |
|----------|-----------|----------|
| [`vipavault-spec.md`](vipavault-spec.md) | Dev + PM | Product spec đầy đủ |
| File này | **Dev** (agent tham khảo) | Quyết định kỹ thuật đã chốt |
| [`README.md`](../README.md) | Dev mới | Quick start WSL |
| [`.context/decisions/`](../.context/decisions/) | Dev + agent | Decision records chi tiết |
| [`.context/MILESTONES.md`](../.context/MILESTONES.md) | Agent + lead | Execution boundary |
| [`AGENTS.md`](../AGENTS.md) | Agent | Protocol bắt buộc cho agent |
| [`.context/planning/FOUNDATION_WORKFLOW.md`](../.context/planning/FOUNDATION_WORKFLOW.md) | Dev + agent | Tiêu chuẩn foundation, AUTO→MANUAL |

---

## 12. Lịch sử cập nhật

| Ngày | Thay đổi |
|------|----------|
| 2026-06-18 | Khởi tạo — tổng hợp quyết định 0.1.0 + SIMPLE V1 + tensions active |
| 2026-06-18 | §3.1 — sửa login UI: vault name + user name trước ô pass; backend 1 vault vẫn chỉ cần password |
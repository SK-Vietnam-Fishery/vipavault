# CONFUSE — Confuse engine

## [manual] Role

Sinh confuse string **chỉ lúc gửi** thông báo; vault luôn lưu pass thật.

**Source:** `src-tauri/src/confuse`  
**Generated AST:** `.context/generated/src-tauri_src_confuse.md`  
**Load:** `context-gen load src-tauri/src/confuse . --include-manual`

## [manual] Design Decisions — Phase: V1

- Vault lưu pass thật — **không** lưu confuse string.
- Rule prefix/suffix đọc từ `app_settings` / DB — không hardcode.
- `confuse_used` ghi vào `activity_log` sau khi gửi.

## [manual] Invariants & Constraints

- Confuse string chỉ tồn tại runtime khi compose message.
- Không persist confuse output vào credential tables.

## [manual] Behavior chưa implement — M10

- Compose Zalo/email templates.
- Decode hint cho recipient (bỏ N đầu M cuối).
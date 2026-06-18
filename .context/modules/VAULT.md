# VAULT — SQLCipher engine

## [manual] Role

Mã hóa và lifecycle file `.hvault`: create, open, lock, multi-profile metadata.

**Source:** `src-tauri/src/vault`  
**Generated AST:** `.context/generated/src-tauri_src_vault.md`  
**Load:** `context-gen load src-tauri/src/vault . --include-manual`

## [manual] Design Decisions — Phase: V1

- **Vault engine = SQLCipher** (SQLite encrypted, file `.hvault`) — **không** MariaDB/MySQL/Postgres server. Tên “MariaDB” trong doc/product chỉ là *loại sub-credential lưu trong vault* (pass DB trên hosting), không phải stack backend app.
- **SQLCipher** file-level encryption thay field-level — copy `.hvault` sang máy viewer đơn giản. Tension `storage`.
- **Không** KeePass/KDBX backend — hierarchy 3 tầng qua schema + UI. `.context/decisions/DECISION_VAULT_STORAGE.md`.
- Argon2id: salt 32 bytes, memory 64MB, iterations 3 (spec §3).
- Profile metadata (`profiles.json`) nằm **ngoài** vault, không mã hóa.
- Master password **không** persist — không recovery path.

## [manual] Invariants & Constraints — Phase: all

- Clear key material bằng `zeroize()` — **không** dùng `drop()` thay thế.
- Không log master password, derived key, hay plaintext credential.
- Lock/idle timeout → zeroize key trước khi đóng SQLCipher connection.
- 0.1.1 scope: vault crypto trước khi UI credential phụ thuộc.

## [manual] Test Strategy

- Round-trip encrypt/decrypt empty vault + schema migrate.
- Wrong password fails without leaking timing hints về partial correctness.
- Lock path zeroizes — test với mock/key inspection pattern.
- Master password never written to disk or app settings.

## [manual] Behavior chưa implement — Milestone 0.1.1

- Create/open/lock `.hvault` commands.
- SQLCipher + rusqlite (or equivalent) integration.
- Argon2id KDF wiring.
- Error types không leak secrets.
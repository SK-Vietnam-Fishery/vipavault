# Module 2 — Bảo mật & Mật mã ứng dụng cho Vault

> **Mục tiêu:** Hiểu **vì sao** từng lớp bảo mật của VipaVault tồn tại, đủ để tự implement 0.1.1 (Vault Core) và tự bảo vệ quyết định trước review. Nguyên tắc số 1: **không tự chế mật mã** — chỉ ghép đúng các primitive đã kiểm chứng.
>
> **Thời lượng:** 2–3 tuần · **Điều kiện vào:** Module 1 (Rust).

---

## 2.1 Bức tranh: chuỗi từ password đến dữ liệu

```
Master Password ──Argon2id (KDF)──▶ Derived Key 256-bit ──▶ SQLCipher mở .hvault ──▶ plain data trong RAM
      ▲ không lưu                          ▲ zeroize khi lock            ▲ AES mã hóa từng page file
```

Ba câu hỏi định hình module: (1) tại sao cần KDF thay vì hash thường, (2) file được mã hóa thế nào, (3) key sống và chết ra sao trong RAM.

## 2.2 Key Derivation — Argon2id

| Học | Nội dung | Tài liệu |
|-----|----------|----------|
| Vì sao không dùng SHA-256(password) | GPU thử hàng tỷ hash/giây; KDF phải **đắt có chủ đích** (memory-hard) | OWASP Password Storage Cheat Sheet — https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html |
| Argon2id là gì, 3 tham số (memory, iterations, parallelism) | Spec chốt 64MB / 3 iter / salt 32 bytes — phải giải thích được từng con số | RFC 9106 (đặc tả chính thức) — https://www.rfc-editor.org/rfc/rfc9106 |
| Salt: vì sao random, vì sao lưu công khai được | Salt chống rainbow table, không phải bí mật | RFC 9106 §3.1; OWASP cheat sheet ở trên |
| Crate Rust | `argon2` (RustCrypto) | https://docs.rs/argon2/latest/argon2/ |

**Bài tập:** viết chương trình Rust: nhập password → derive key với đúng tham số spec → đo thời gian trên máy bạn. Sau đó thử 8MB/1 iter và giải thích trong journal tại sao nhanh hơn lại là **tệ hơn**. Thử trên máy yếu nhất bạn có (liên quan blind spot hiệu năng — Module 8).

## 2.3 Mã hóa file — SQLCipher

| Học | Nội dung | Tài liệu |
|-----|----------|----------|
| Thiết kế SQLCipher: mã hóa per-page, HMAC per-page, KDF nội bộ | Hiểu "file-level encryption" thực sự nghĩa là gì | SQLCipher Design — https://www.zetetic.net/sqlcipher/design/ |
| API: `PRAGMA key`, `PRAGMA rekey` | Mở vault + xoay master password (runbook pm-review §6) | https://www.zetetic.net/sqlcipher/sqlcipher-api/ |
| Vì sao raw key (`PRAGMA key = "x'...'"`) thay vì passphrase | App tự chạy Argon2id rồi đưa raw key — kiểm soát tham số KDF thay vì dùng PBKDF2 mặc định của SQLCipher | SQLCipher API §key — cùng link trên |
| So sánh với thiết kế vault khác | Đối chiếu để hiểu trade-off | Bitwarden Security Whitepaper — https://bitwarden.com/help/bitwarden-security-white-paper/ · KeePass security — https://keepass.info/help/base/security.html |

**Bài tập:** dùng `sqlcipher` CLI tạo DB mã hóa, mở bằng key đúng/sai, quan sát lỗi. Chạy `PRAGMA rekey` và xác nhận key cũ không mở được nữa — nhưng **bản copy trước rekey vẫn mở được bằng key cũ** (ghi nhận xét này vào journal: đây là lý do runbook rotation phải thu hồi backup cũ).

## 2.4 Key lifecycle trong RAM — zeroize và giới hạn của nó

| Học | Nội dung | Tài liệu |
|-----|----------|----------|
| Vì sao `drop()` không đủ | Compiler có thể tối ưu bỏ việc ghi đè memory sắp free ("dead store elimination") | zeroize docs — https://docs.rs/zeroize/latest/zeroize/ |
| `Zeroize` + `ZeroizeOnDrop` derive | Áp cho struct chứa key/password | Cùng link trên |
| **Giới hạn thật** (quan trọng): swap file, hibernation, crash dump, copy tạm của allocator | zeroize giảm cửa sổ phơi nhiễm, KHÔNG phải bảo đảm tuyệt đối | Đọc case thực: KeePass CVE-2023-32784 (master password trong memory dump) — https://nvd.nist.gov/vuln/detail/CVE-2023-32784 |

**Bài tập:** viết struct `DerivedKey([u8; 32])` với `ZeroizeOnDrop`, viết test unlock→lock→unlock lại. Trong journal: liệt kê 3 con đường key vẫn có thể rò dù đã zeroize, và với mỗi con đường ghi "chấp nhận / giảm thiểu thế nào" (đây chính là bài tập threat modeling thu nhỏ).

## 2.5 Threat Modeling — làm một lần cho VipaVault

| Học | Nội dung | Tài liệu |
|-----|----------|----------|
| Threat modeling 4 câu hỏi: đang làm gì? có thể hỏng thế nào? làm gì với nó? làm đủ tốt chưa? | Khung tư duy trước mọi tính năng bảo mật | OWASP Threat Modeling — https://owasp.org/www-community/Threat_Modeling · Threat Modeling Manifesto — https://www.threatmodelingmanifesto.org/ |
| STRIDE (Spoofing, Tampering, Repudiation, Info disclosure, DoS, Elevation) | Checklist quét từng thành phần | Microsoft STRIDE — https://learn.microsoft.com/en-us/azure/security/develop/threat-modeling-tool-threats |
| Trust boundary của VipaVault | **Boundary duy nhất = master password.** `machine_role` là UX, không phải security (pm-review §3) | `docs/reviews/pm-review-solutions.md` §3 |

**Bài tập lớn (deliverable của module):** viết `threat-model.md` cá nhân ~2 trang: tài sản (credential, master key), tác nhân đe dọa (malware trên máy, người nhặt được laptop, NV nghỉ việc, chính IT), từng cặp tài sản×tác nhân → có phòng thủ gì / chấp nhận gì. So kết quả với bảng Rủi ro trong spec §3 — chỗ nào bạn tìm ra mà spec thiếu?

## 2.6 Xử lý secret trong ứng dụng — quy tắc vận hành

| Quy tắc | Lý do | Tài liệu |
|---------|-------|----------|
| Không log credential, kể cả debug | Log file sống lâu hơn RAM, không mã hóa | OWASP Logging Cheat Sheet — https://cheatsheetseries.owasp.org/cheatsheets/Logging_Cheat_Sheet.html |
| Sinh password bằng CSPRNG, không `rand` thường | Password đoán được = vô nghĩa | crate `rand` với `OsRng` — https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html · NIST SP 800-63B §5.1.1 — https://pages.nist.gov/800-63-3/sp800-63b.html |
| Clipboard: tự xóa sau N giây | Clipboard là plaintext toàn cục (chi tiết Module 8) | Tham khảo hành vi KeePassXC — https://keepassxc.org/docs/ |
| Audit mọi lần reveal password | `activity_log` — phát hiện lạm dụng sau sự cố | Spec §4 `activity_log` |

## Câu hỏi phải trả lời được (viết vào journal)

1. Nếu hai vault dùng chung salt, attacker lợi gì?
2. Tại sao Argon2id (không phải Argon2i hay Argon2d)? — RFC 9106 §9 có câu trả lời.
3. `.hvault` bị copy trộm khi vault **đang khóa**: attacker cần gì để đọc? Chi phí brute-force với tham số spec ước tính ra sao?
4. `.hvault` bị copy khi vault **đang mở**: khác gì tình huống trên? (Gợi ý: không khác — file trên đĩa luôn mã hóa; khác biệt nằm ở RAM.)
5. Vì sao "không có recovery" là hệ quả toán học chứ không phải lựa chọn UX, và Emergency Kit (pm-review §4) lách điều đó bằng cách nào mà không phá crypto?

## Checkpoint ra khỏi Module 2

- [ ] Chương trình Argon2id chạy + bảng đo thời gian 2 bộ tham số
- [ ] Thực hành `PRAGMA key`/`rekey` bằng CLI, ghi chú về backup cũ
- [ ] Struct key có `ZeroizeOnDrop` + test
- [ ] `threat-model.md` cá nhân hoàn thành, có so với spec §3
- [ ] Trả lời 5 câu hỏi trên trong journal

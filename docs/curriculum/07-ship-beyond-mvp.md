# Module 7 — Ship: từ MVP đến sản phẩm thật (và Phase 2/3)

> **Mục tiêu:** "Chạy trên máy dev" → "người khác cài được, tin được, cập nhật được". Đây là phần đa số tutorial bỏ qua và là lý do MVP không đồng nghĩa với sản phẩm. Áp dụng vào milestone 0.10.0 và mở tiếp Phase 2/3.
>
> **Thời lượng:** 3–4 tuần (chưa tính chờ chứng chỉ ký số).

---

## 7.1 Đóng gói & phân phối

| Học | Nội dung | Tài liệu |
|-----|----------|----------|
| Tauri bundler: MSI/NSIS (Windows), DMG (macOS), AppImage/deb (Linux) | Chọn format cho user SME VN (Windows là chính → NSIS/MSI) | https://v2.tauri.app/distribute/ |
| Icon, metadata, version trong `tauri.conf.json` | Bản cài trông chuyên nghiệp | https://v2.tauri.app/reference/config/ |
| Cài đặt per-user vs per-machine, thư mục dữ liệu | `.hvault` phải sống sót khi gỡ app (quyết định có chủ đích!) | Xem Module 8 §uninstall |

**Bài tập:** build bản cài Windows, cài lên **một máy sạch** (VM), chạy không có Rust/Node — danh sách mọi thứ hỏng chính là backlog hardening của bạn.

## 7.2 Code signing — bắt buộc, không phải tùy chọn

Binary không ký trên Windows bị SmartScreen chặn màn hình xanh "Windows protected your PC" — với app **quản lý credential**, một cảnh báo như vậy giết niềm tin ngay lập tức.

| Học | Nội dung | Tài liệu |
|-----|----------|----------|
| Windows Authenticode: OV vs EV certificate, SmartScreen reputation | EV có reputation ngay; OV phải "nuôi" reputation theo số lượt cài | Microsoft SmartScreen — https://learn.microsoft.com/en-us/windows/security/operating-system-security/virus-and-threat-protection/microsoft-defender-smartscreen/ |
| Lựa chọn thực dụng 2026: Azure Trusted Signing (rẻ hơn EV truyền thống) | ~$9.99/tháng, tích hợp CI được | https://learn.microsoft.com/en-us/azure/trusted-signing/ |
| Tauri signing integration | Ký trong pipeline build | https://v2.tauri.app/distribute/sign/windows/ |
| macOS: Developer ID + notarization (nếu ship macOS) | Gatekeeper chặn app không notarize | https://developer.apple.com/documentation/security/notarizing-macos-software-before-distribution |

**Chi phí phải tính trước:** chứng chỉ $100–400/năm hoặc Trusted Signing ~$120/năm — đã ước trong `.local/BUSINESS_MODEL.md` §4.3.

## 7.3 Auto-update — và bảo mật của chính nó

Update chính là **kênh tấn công supply-chain vào user của bạn**: ai chiếm được update feed sẽ đẩy binary độc cho mọi máy.

| Học | Nội dung | Tài liệu |
|-----|----------|----------|
| Tauri updater plugin: update manifest + chữ ký riêng (minisign) | Update bị từ chối nếu sai chữ ký — private key này quý ngang master password của bạn | https://v2.tauri.app/plugin/updater/ |
| Hosting update: GitHub Releases là đủ cho V1 | Không cần server riêng | Cùng link trên |
| Bảo vệ private key ký update | Offline, backup, KHÔNG commit — mất key = mất kênh update vĩnh viễn | The Update Framework (đọc để hiểu mối đe dọa, không cần implement): https://theupdateframework.io/ |

**Bài tập:** dựng flow update hoàn chỉnh giữa 2 version app test, gồm 1 lần cố tình sai chữ ký → xác nhận app từ chối.

## 7.4 Release engineering

| Học | Nội dung | Tài liệu |
|-----|----------|----------|
| SemVer kỷ luật | Repo đã dùng 0.x — hiểu quy tắc breaking change khi lên 1.0 | https://semver.org/ |
| Changelog cho người đọc | Mỗi release: Added/Changed/Fixed/Security | https://keepachangelog.com/ |
| CI: GitHub Actions build + test + bundle đa nền tảng | Không release từ máy dev — build phải lặp lại được | https://docs.github.com/en/actions · Tauri GitHub Action: https://github.com/tauri-apps/tauri-action |
| Supply chain: audit dependency | `cargo audit` (RustSec), `npm audit`, lockfile commit | https://rustsec.org/ · cargo-deny: https://github.com/EmbarkStudios/cargo-deny |
| Release checklist viết sẵn | Test xanh → audit sạch → build ký → smoke test máy sạch → tag → changelog → publish | Tự viết ở bài tập |

**Bài tập:** viết `RELEASE_CHECKLIST.md` cho repo + dựng workflow CI chạy `npm run verify` mỗi push. Tag thử `v0.1.1-rc1` đi hết checklist một vòng.

## 7.5 Chất lượng trước khi trao cho người khác

- **Beta có cấu trúc:** 1–3 người dùng thật ngoài bạn (nhân viên/agency quen — xem BUSINESS_MODEL §5.2), kịch bản nhiệm vụ cụ thể ("tạo email cho NV mới"), quan sát không trợ giúp. 5 user tìm ra ~85% vấn đề usability: https://www.nngroup.com/articles/why-you-only-need-to-test-with-5-users/
- **Crash handling:** với app credential, **không** gửi telemetry tự động (dump có thể chứa secret!) — thay bằng: error dialog tử tế + nút "copy chi tiết lỗi" (đã lọc secret) để user tự gửi bạn.
- **Bảng hỗ trợ tối thiểu:** README hướng dẫn cài, FAQ 10 câu, email hỗ trợ, và **runbook cho chính bạn**: restore từ backup, rekey, chuyển máy.

## 7.6 Trách nhiệm của sản phẩm bảo mật

| Việc | Nội dung | Tài liệu |
|------|----------|----------|
| `SECURITY.md` + security.txt | Kênh báo lỗ hổng riêng (không phải GitHub issue công khai) | https://securitytxt.org/ · GitHub security policy: https://docs.github.com/en/code-security/getting-started/adding-a-security-policy-to-your-repository |
| Disclaimer trách nhiệm | AGPL đã có no-warranty; cân nhắc thêm dòng rõ ràng trong README | LICENSE §15–16 |
| AGPL compliance khi phát hành binary | Kèm offer source; giữ LICENSE trong installer | https://www.gnu.org/licenses/gpl-faq.html#DoesTheGPLAllowDownloadFee |

## 7.7 Phase 2 — OAuth providers (M365, Google Workspace)

Kỹ năng mới so với V1: OAuth 2.0 server-to-server (không có user browser flow).

| Học | Nội dung | Tài liệu |
|-----|----------|----------|
| OAuth2 client credentials grant | M365: app registration, tenant, secret → token | RFC 6749 §4.4 — https://www.rfc-editor.org/rfc/rfc6749#section-4.4 |
| Microsoft Graph API: users, licenses | Thay thế thao tác admin M365 tay | https://learn.microsoft.com/en-us/graph/overview · auth: https://learn.microsoft.com/en-us/graph/auth-v2-service |
| Google Workspace: service account + domain-wide delegation | Admin SDK Directory API | https://developers.google.com/workspace/admin/directory/v1/guides · service account: https://developers.google.com/identity/protocols/oauth2/service-account |
| Token cache + tự refresh | Invariant spec §5: access_token là cache, không phải source of truth | Spec §5 |

Schema đã chuẩn bị sẵn (`oauth_credentials`, spec §5 Phase 2) — đây là phần thưởng của abstraction `provider_type`/`auth_scheme` làm đúng từ V1.

## 7.8 Phase 3 — License tracking & báo cáo

- `subscription_licenses` (spec §5 Phase 3); chi phí computed từ seats — nhớ INTEGER tiền.
- **Xuất PDF báo cáo tháng** — có thể chính là "viewer" thật (pm-review §1): crate `printpdf` https://docs.rs/printpdf hoặc render HTML → in qua WebView.
- Trước khi build Phase 3, quay lại câu hỏi định vị pm-review §9 — Phase 3 là nơi hai định vị (vault vs ITAM) va nhau.

## Checkpoint ra khỏi Module 7

- [ ] Bản cài ký số chạy trên máy sạch không cảnh báo SmartScreen (hoặc kế hoạch nuôi reputation ghi rõ)
- [ ] Auto-update hoạt động + test từ chối chữ ký sai
- [ ] CI xanh, `cargo audit`/`npm audit` sạch, RELEASE_CHECKLIST.md tồn tại và đã đi 1 vòng
- [ ] ≥1 người ngoài bạn cài và dùng thành công theo kịch bản, phản hồi được ghi
- [ ] SECURITY.md + email hỗ trợ + runbook restore/rekey hoàn thành

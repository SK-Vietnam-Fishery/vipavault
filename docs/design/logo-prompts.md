# Logo prompts — VipaVault

> Dùng để gen **cảm hứng**, không phải brand guide đã chốt.
> Mark (biểu tượng) gen bằng AI. Wordmark chữ **VipaVault** nên ghép sau bằng font — model hay viết sai chữ.
> Ngày: 2026-08-14

---

## 0. Brief bắt buộc — dán đầu mọi prompt

Sao chép khối này rồi nối thêm 1 hướng ở §3.

```
Product: VipaVault — a calm desktop app for a small Vietnamese company.
It stores hosting, email, domain, and SSL credentials in one local encrypted
file (a vault), so an IT operator can work and a CEO can see costs and expiry
alerts without opening cPanel.

Audience: two people, same app — an IT admin and a non-technical executive.
Feeling: quiet competence, local file you can copy, clarity over fortress.
Not a bank, not crypto, not a hacker tool, not a password-manager chrome badge.

Name: VipaVault (one word, V and V capitalized). Do not invent a mascot for
“Vipa”. The mark may use a V, a vault/file, layered tiers, or a keyhole-as-window.

Technical: square app icon and simple mark. Must read at 16–32 px. Flat or
subtle 2.5D. Generous padding. Isolated on a single flat background (off-white
#F6F8FB or deep ink #17202A). One symbol, centered. No scene, no desk, no
people, no screenshots, no passwords, no padlock-plus-USB cliché, no Bitcoin,
no skull. Pure symbol — no letters, no numbers, no watermark.
```

### Màu đã có trên UI (neo palette)

| Vai trò | Hex | Dùng |
|---|---|---|
| Ink | `#17202A` | nền tối / nét chính |
| Body | `#344054` | secondary |
| Slate | `#526173` | eyebrow, phụ |
| Line | `#D8DEE8` | viền |
| Paper | `#F6F8FB` | nền sáng |
| White | `#FFFFFF` | mặt mark |

Accent gợi ý (chưa chốt): teal `#0F766E` (tin cậy), amber `#D97706` (cảnh báo hết hạn), indigo `#3730A3`.

---

## 1. Constraint cho mọi lần gen

| Phải | Tránh |
|---|---|
| 1 biểu tượng, căn giữa, nền phẳng 1 màu | Cảnh 3D phòng server, người, laptop |
| Đọc được khi thu nhỏ 32px | Chi tiết mỏng, texture nhiễu |
| 1:1, padding ~12–15% | Chữ “VipaVault” / “VIP” / “VAULT” trong mark |
| Vector-like, mép sạch | Chrome bóng, neon cyber, glassmorphism dày |
| Ẩn dụ: file mã hóa, tầng dữ liệu, cửa sổ rõ ràng | Ổ khóa + khiên generic, Bitcoin, mắt thần |

Hai asset tách nhau:

1. **Mark only** — gen AI (prompt dưới).
2. **Wordmark** — font + kerning thủ công. Nếu vẫn gen chữ: chỉ 1 dòng `VipaVault`, rồi đọc lại từng chữ; sai là bỏ.

---

## 2. Brief ngắn (tiếng Việt) — nếu tool hiểu Việt tốt

VipaVault là app desktop nội bộ: một file két mã hóa chứa hosting, email, domain. Người dùng là IT và sếp — cần cảm giác **rõ, tin được, không hoành tráng**. Logo là biểu tượng đơn, nền phẳng, không chữ, đọc được lúc 32px. Lấy ý từ file/két, các tầng (công ty → dịch vụ → mật khẩu), hoặc lỗ khóa như cửa sổ nhìn dashboard. Không ngân hàng, không crypto, không hacker.

---

## 3. Hướng khám phá — prompt tiếng Anh (dán sau brief §0)

Mỗi hướng = một lần gen, `aspect_ratio: 1:1`.

### A — Monogram V trong két hình học

A quiet app-icon mark: a rounded square tile in deep ink #17202A. Inside, a single geometric “V” cut from a vault-door motif — two thick chevron planes meeting, a small circular keyhole sitting in the negative space like a window, not a cartoon lock. Flat vector, 2–3 tones only (ink, cool slate, one teal #0F766E accent on the keyhole). Centered, large padding, off-white #F6F8FB canvas. No lettering.

### B — File `.hvault` gấp lớp

A simple symbol of a standing document with one folded corner, the page made of three nested layers like a thin stack of cards (company, service, secret). A single fine seam suggests encryption without showing a lock. Matte paper white on ink #17202A, one slate line. Flat, icon-ready, centered on #F6F8FB. No text, no file-extension letters.

### C — Cửa sổ lỗ khóa (rõ ràng, không pháo đài)

A circular monochrome mark: a keyhole whose opening is a calm rounded rectangle — like a small clear window — with two or three soft horizontal bands inside suggesting a dashboard of status, not a prison. Deep ink shape, interior a warm off-white, one thin amber tick as a “due soon” signal. Flat, highly simplified, 32px-legible, centered on pale gray-blue. No typography.

### D — Ba tầng đồng tâm

An abstract emblem of three nested rounded rectangles, slightly offset like a careful stack, implying profile → service → credential. Outer shape ink, middle slate, inner a small teal block. No lock, no key. Strict geometry, even stroke, lots of padding, white or #F6F8FB ground. App-icon clarity.

### E — Dấu triện mực (SME Việt, không “cyber”)

A square seal-like mark in wet ink blue-black, as if stamped once on rice paper. Motif: a simplified house-roof chevron over a small enclosed rectangle (a local vault, a company house). Soft paper tooth, one impression, not ornate dragon/phoenix, not national flag. Centered, generous margin, no characters.

### F — Khiên cắt V (nếu cần “bảo mật” rõ)

A shield reduced to five strokes, the inner cutout a sharp V that also reads as an open folder. Two colors: ink and teal. Completely flat, no gradient, no chrome. Isolated on #F6F8FB. No letters.

### G — Wordmark (chỉ khi chấp nhận rủi ro chữ)

Single-line wordmark “VipaVault” in a modern grotesque, medium weight, tight tracking, ink #17202A on #F6F8FB. Capital V, lowercase remainder of each camel hump: VipaVault. No icon beside it, no tagline, no extra words. Centered, lots of space. (Verify spelling letter-by-letter; discard if any letter is wrong.)

---

## 4. Biến thể kỹ thuật (cùng 1 hướng đã thích)

Sau khi chọn 1 hướng, gen thêm:

| Mục đích | Thêm vào cuối prompt |
|---|---|
| Icon app sáng | `Same mark, flat #F6F8FB background, high contrast.` |
| Icon app tối | `Same mark inverted onto #17202A; keep the teal accent.` |
| Favicon 16px | `Even simpler: drop the thinnest lines; 3 shapes maximum.` |
| Tray / Windows | `Rounded-square squircle container, mark inset 18%.` |
| Đen trắng in | `Pure black mark on white; no gray, no teal.` |

Không gen lại từ đầu nếu đã có mark ưng — chỉnh bằng image-to-image / edit, giữ cùng hình.

---

## 5. Prompt hệ thống (nếu tool có ô System / Style)

```
You are designing a software product mark, not a poster and not a 3D render.
Output one centered symbol, flat or barely dimensional, production-icon
quality. Prefer fewer shapes. Never add slogans, URLs, or the product name
unless the user prompt is the wordmark variant. Never depict passwords,
terminals, or people.
```

---

## 6. Checklist sau khi gen

- [ ] Đọc được khi squint / thu 32px
- [ ] Không có chữ (trừ biến thể G)
- [ ] G: đúng từng chữ `V-i-p-a-V-a-u-l-t`
- [ ] Không giống Bitcoin / 1Password / KeePass / Chrome lock
- [ ] Cắt được nền (1 màu)
- [ ] Không mật khẩu, QR, screenshot

Hướng chọn xong → vẽ lại SVG thủ công (AI chỉ lấy ý).

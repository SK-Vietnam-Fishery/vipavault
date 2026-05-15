# VipaVault

> **Vibe + Admin Vault + Vipafood = VipaVault for Hosting & Email** — bảng điều khiển dành cho CEO, không phải cho sysadmin.

Quản lý website, domain, và email doanh nghiệp bằng ngôn ngữ bình thường. Không cPanel, không SSH, không ticket.

---

## ✨ Vì sao là VipaVault?

Hosting panel truyền thống được làm cho kỹ thuật. **VipaVault** được làm cho founder, CEO, và team vận hành cần sự rõ ràng, kiểm soát và chi phí minh bạch.

**Vipa = VIP + Admin**

## 🚀 Tính năng chính

### Executive-First
- **CEO Dashboard** — uptime, chi phí, dung lượng, sức khỏe email trong 1 màn hình
- **Plain-English Insights** — "Email gửi đi giảm 12%" thay vì lỗi SPF
- **Cost Control** — chi phí hosting/email theo dự án, team, khách hàng
- **One-Click Approvals** — duyệt gia hạn, nâng cấp từ Slack/email

### Hosting Made Simple
- Deploy WordPress, Next.js, Laravel trong 60s
- SSL tự động, backup hàng ngày, CDN toàn cầu
- Staging → production kéo-thả, rollback 1 click

### Email Admin cho lãnh đạo
- Kết nối Google Workspace, Microsoft 365, Postal
- Theo dõi domain health (SPF, DKIM, DMARC)
- Onboarding/offboarding nhân sự tự động

## 🛠️ Tech Stack
- **Frontend:** Next.js 14, TypeScript, Tailwind, shadcn/ui
- **Backend:** tRPC, Prisma, PostgreSQL
- **Infra:** Docker, Traefik, Cloudflare, Hetzner/AWS

## 📦 Cài đặt

```bash
# 1. Clone
git clone https://github.com/yourorg/vipavault.git
cd vipavault

# 2. Install
pnpm install

# 3. Config
cp .env.example .env
# chỉnh DATABASE_URL, AUTH_SECRET, CLOUDFLARE_API_KEY...

# 4. DB
pnpm prisma migrate dev

# 5. Run
pnpm dev
```

## 💻 Sử dụng
1. Tạo Organization
2. Kết nối Providers (Cloudflare, Hetzner, Google Workspace)
3. Mời team — gán role "CEO View" hoặc "Ops"
4. Deploy — chọn template → gắn domain → launch

## 📄 License
MIT — Built for CEOs who don't want to be sysadmins.

---
> *"Nếu cần hơn 3 click thì nó quá phức tạp."* — VipaVault Design Manifesto

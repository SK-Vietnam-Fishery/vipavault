# Module 3 — Product & Design Thinking

> **Mục tiêu:** Tránh chế độ thất bại đắt nhất của solo developer: **xây đúng thứ không ai cần**. Module này chạy **song song** với các module kỹ thuật — mỗi giả định sản phẩm được kiểm chứng trước khi milestone tương ứng bắt đầu.
>
> **Thời lượng:** 1–2 tuần đọc + bài tập rải suốt dự án.

---

## 3.1 Design Thinking — dùng như vòng lặp, không phải nghi lễ

5 bước Empathize → Define → Ideate → Prototype → Test (Stanford d.school). Với solo dev + 1 công ty, phiên bản thực dụng:

| Bước | Với VipaVault nghĩa là |
|------|------------------------|
| Empathize | Quan sát chính bạn (IT) và CEO **làm việc thật**: hiện đang tra credential ở đâu, mất bao lâu, sai gì |
| Define | Viết problem statement 1 câu, không nhắc giải pháp. VD: "IT mất X phút/lần tìm credential rải rác và không ai biết dịch vụ nào sắp hết hạn" |
| Ideate | ≥3 giải pháp kể cả "không code" (spreadsheet + reminder?) — nếu app vẫn thắng thì lý do phải nói được thành lời |
| Prototype | Mock rẻ nhất có thể: PDF dashboard giả, form vẽ giấy — **trước khi** viết React |
| Test | Đưa cho người dùng thật, quan sát, KHÔNG giải thích hộ |

**Tài liệu:**
- Stanford d.school — Design Thinking Bootleg (bộ method cards miễn phí): https://dschool.stanford.edu/resources/design-thinking-bootleg
- IDEO Design Kit (methods): https://www.designkit.org/methods.html
- Nielsen Norman Group — Design Thinking 101: https://www.nngroup.com/articles/design-thinking/

## 3.2 Jobs to be Done + The Mom Test — kiểm chứng không tự lừa

- **JTBD:** người dùng "thuê" sản phẩm để làm một job. Job của CEO không phải "dùng app vault" — có thể là *"biết trong 30 giây rằng không có gì sắp cháy và tốn bao nhiêu tiền/tháng"*. Job đó một email tháng cũng làm được → đó chính là phản biện viewer persona (pm-review §1).
- **The Mom Test:** quy tắc hỏi để không nhận câu trả lời xã giao — hỏi về **hành vi quá khứ cụ thể**, không hỏi "anh có dùng không?"

**Tài liệu:**
- JTBD — Clayton Christensen, "Know Your Customers' Jobs to Be Done" (HBR 2016): https://hbr.org/2016/09/know-your-customers-jobs-to-be-done
- Intercom on Jobs-to-be-Done: https://www.intercom.com/resources/books/intercom-jobs-to-be-done
- The Mom Test: https://www.momtestbook.com/

**Bài tập bắt buộc (trước milestone 0.3.0):** phỏng vấn CEO 15 phút theo Mom Test — chỉ hỏi hành vi quá khứ: "Lần gần nhất anh cần biết chi phí hosting là khi nào? Anh đã làm gì? Mất bao lâu? Kết quả có đủ không?". Đưa PDF mock dashboard, hỏi "anh muốn nhận cái này thế nào?". Ghi nguyên văn câu trả lời vào journal → quyết định số phận viewer app.

## 3.3 Spec là giả thuyết — ADR là bộ nhớ quyết định

- Spec (`vipavault-spec.md`) không phải kinh thánh — nó là **tập giả thuyết được đánh số**. Mỗi milestone ship xong là một lần kiểm chứng vài giả thuyết.
- **ADR (Architecture Decision Record):** mỗi quyết định khó đảo (chọn SQLCipher, cắt Confuse, INTEGER cho tiền) ghi 1 file: bối cảnh → phương án → quyết định → hệ quả. Repo đã có mẫu sẵn: `.context/decisions/DECISION_VAULT_STORAGE.md`.

**Tài liệu:**
- ADR homepage + templates: https://adr.github.io/
- Michael Nygard — gốc của format ADR: https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions
- Shape Up (Basecamp, miễn phí) — appetite & scope hammering, rất hợp solo dev: https://basecamp.com/shapeup

**Bài tập:** viết 1 ADR thật cho quyết định "cột tiền INTEGER thay REAL" theo format Nygard, đặt vào `.context/decisions/`. So với 2 ADR có sẵn trong repo về độ chặt của phần "Consequences".

## 3.4 Kỷ luật scope — kỹ năng sống còn của solo dev

Ba câu hỏi trước khi thêm bất kỳ tính năng nào:

1. **Ai yêu cầu?** (bằng chứng hành vi, không phải "chắc sẽ cần")
2. **Chuyện gì xảy ra nếu không làm?** (nếu câu trả lời là "vẫn ổn" → backlog)
3. **Có đường lùi không?** (quyết định 1 chiều — như đổi license, đổi format file — cần cân nhắc gấp 10 lần quyết định 2 chiều)

Case study nội bộ: Confuse engine bị cắt khỏi V1 (pm-review §7) chính vì fail câu 1 và 2 — đọc lại như một bài học scope.

**Tài liệu:**
- YAGNI (Fowler): https://martinfowler.com/bliki/Yagni.html
- "Choose Boring Technology" (McKinley): https://mcfunley.com/choose-boring-technology

## 3.5 Câu hỏi design thinking phải tự trả lời (checklist xuyên dự án)

Trả lời bằng văn bản, xem lại mỗi khi qua milestone:

1. Job thật của **IT (chính bạn)** là gì — phát biểu không nhắc đến "app" hay "vault"?
2. Job thật của **CEO** là gì? Bằng chứng nào ngoài suy đoán của bạn?
3. Giải pháp hiện tại (trước app) là gì và nó **thất bại ở đâu, bao nhiêu lần/tháng**? Nếu không đo được tần suất đau → nghi ngờ độ đau.
4. Phiên bản **nhỏ nhất** thay thế được giải pháp hiện tại là gì? (đây là định nghĩa dogfood 0.6.0 — không phải danh sách feature)
5. Nếu chỉ được giữ **3 màn hình**, đó là những màn nào? Mọi màn khác là nghi vấn.
6. Điều gì khiến bạn **bỏ dùng chính app của mình** sau 2 tuần? (ma sát unlock? nhập liệu chậm hơn Excel?) — thiết kế chống lại điều đó trước.
7. Với người dùng thứ 2 (công ty khác): cái gì trong thiết kế hiện tại là "chỉ hợp công ty mình"? (hard-code tiếng Việt? quy ước đặt tên? confuse rule?)
8. Sự kiện tệ nhất có thể xảy ra với người dùng là gì? (mất vault không recovery — đã có Emergency Kit chưa?) Sản phẩm tin cậy được thiết kế từ failure mode, không phải happy path.
9. Ai là người **nói không** với sản phẩm này và vì sao? (VD: agency đã quen Bitwarden — xem `.local/BUSINESS_MODEL.md` §2.1) Câu trả lời định hình positioning.
10. Bạn đo "thành công" của mỗi milestone bằng hành vi gì — không phải bằng "code xong"? (VD 0.4.0: "tôi mở dashboard thay vì mở Excel, tự nguyện, 5 ngày liên tiếp")

## Checkpoint ra khỏi Module 3

- [ ] Phỏng vấn Mom Test với CEO hoàn thành, nguyên văn trong journal, quyết định viewer đã chốt
- [ ] 1 ADR thật đã viết và đặt đúng chỗ
- [ ] 10 câu hỏi trên có câu trả lời văn bản (chấp nhận "chưa biết — sẽ kiểm chứng ở milestone X")
- [ ] Định nghĩa dogfood cá nhân cho 0.6.0 viết thành 1 câu đo được

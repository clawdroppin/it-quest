OUT = "content/csharp/d2_01_invoices.md"
BASE = "tools/appdev/cs_invoices"
FILES = ["Domain.cs", "InvoiceService.cs", "Storage.cs", "Program.cs", "Tests.cs", "InvoiceManager.csproj"]

PARTS = [
    ("text", r'''
@@lesson c9-invoices Invoice Manager (a .NET business application)
@@text
Invoicing is the heart of every business system, and Japan has precise rules for it. Since October 2023 the **Qualified Invoice System (インボイス制度)** says how consumption tax must appear on an invoice: subtotals and tax **per tax rate**, with the tax rounded **once per rate**, not per line. You'll build an invoice manager that gets this right, with the same layered design real .NET line-of-business applications use.

```text
dotnet run -- demo                   create two sample invoices
dotnet run -- list
dotnet run -- show 1
dotnet run -- new "Kaede Dental" office@kaede.example
dotnet run -- add 2 "Monthly IT support" 30000 1
dotnet run -- add 2 "Bottled tea" 150 24 reduced
dotnet run -- issue 2
dotnet run -- aging
dotnet run -- --test
```

## The design

| File | Layer | What's in it |
|---|---|---|
| `Domain.cs` | domain | `Invoice`, `LineItem`, `Totals`; validation; the tax calculation |
| `InvoiceService.cs` | application | the use cases, and the interfaces they need (`IInvoiceRepository`, `IClock`) |
| `Storage.cs` | infrastructure | implementations: a JSON file repository, an in-memory one, clocks |
| `Program.cs` | presentation | the command line, and wiring the real classes together |
| `Tests.cs` | tests | a tiny test runner with fakes: `MemoryRepository` and `FixedClock` |

This is "Clean Architecture" as Microsoft's .NET guides describe it: the domain and application layers don't reference files, consoles or databases. An ASP.NET Core web API or a WPF window could replace `Program.cs` without touching them.

## The rules

| | 10% (standard) | 8% (reduced: food, drinks, newspapers) |
|---|---|---|
| subtotal | sum of the 10% lines | sum of the 8% lines |
| tax | floor(subtotal × 0.10) | floor(subtotal × 0.08) |

- Invoices start as **drafts**. Only drafts can be edited. **Issuing** gives them a number (`INV-2026-0001`, counting from 1 again each year), an issue date and a due date 30 days later.
- Issued invoices can be **paid** or **voided**. Paid invoices can't be voided (that needs a credit note).
- The **aging report** tells finance how much money is late, in four buckets: current, 1-30, 31-60, 61+ days overdue.

## Your tasks

1. `Invoice.ComputeTotals` in `Domain.cs`: LINQ (`Where`, `Sum`) and `Math.Floor`.
2. `InvoiceService.Issue`: checks, numbering, dates.
3. `InvoiceService.AgingReport`: grouping into buckets. A `switch` expression with relational patterns (`<= 30 => "1-30"`) fits well.

Run `dotnet run -- --test` after each. Tests print `TODO` for the parts you haven't written yet.
'''),
    ("project", "invoices", "Invoice Manager",
     "Implement the three methods that throw `NotImplementedException` until `dotnet run -- --test` passes, then try `dotnet run -- demo` and `dotnet run -- show 1`.",
     BASE + "/starter", FILES),
    ("text", r'''
@@text
## Stretch goals

- **The registration number**: qualified invoices must show the issuer's number, `T` + 13 digits. Add the issuer's details to every invoice and validate the number.
- **A PDF**: generate a printable invoice with QuestPDF (a NuGet package). Only the presentation layer changes.
- **A web API**: an ASP.NET Core minimal API project that references this code: `app.MapPost("/invoices/{id}/issue", (int id, InvoiceService s) => s.Issue(id));`. Register `InvoiceService`, the repository and the clock with `builder.Services`.
- **A real database**: an EF Core `DbContext` and an `EfInvoiceRepository : IInvoiceRepository`. Your tests keep using `MemoryRepository`.
- **Credit notes**: instead of voiding a paid invoice, issue a credit note (`CN-2026-0001`) with negative lines that references it.
- **Concurrency**: two people issue invoices at the same moment and get the same number. How would a database sequence or a unique index solve it?
'''),
    ("project", "invoices_finished", "Invoice Manager: the finished version",
     "A complete solution to compare with yours.",
     BASE + "/finished", FILES),
    ("text", r'''
@@text
🎉 Line-of-business applications like this one (invoicing, inventory, HR, booking) are what most .NET developers in Japan build, often for 業務システム projects. You now know the shape of them: a domain with rules, use cases behind interfaces, swappable storage, and tests that run in milliseconds.
'''),
]

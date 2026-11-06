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
@@project invoices Invoice Manager
Implement the three methods that throw `NotImplementedException` until `dotnet run -- --test` passes, then try `dotnet run -- demo` and `dotnet run -- show 1`.
@@file Domain.cs
namespace InvoiceManager;

// The domain: invoices and the rules of Japan's Qualified Invoice System (インボイス制度).
// No files, no console, no clock: everything here is easy to test.

public enum TaxRate { Standard, Reduced }          // 10%, and 8% for food and newspapers
public enum InvoiceStatus { Draft, Issued, Paid, Void }

public class DomainException(string message) : Exception(message);

public record Customer(string Name, string Email);

public record LineItem(string Description, decimal UnitPrice, int Quantity, TaxRate Rate)
{
    public decimal Amount => UnitPrice * Quantity;

    public static LineItem Create(string description, decimal unitPrice, int quantity, TaxRate rate)
    {
        if (string.IsNullOrWhiteSpace(description)) throw new DomainException("description is required");
        if (unitPrice < 0 || unitPrice != decimal.Truncate(unitPrice)) throw new DomainException("unit price must be whole yen, 0 or more");
        if (quantity < 1) throw new DomainException("quantity must be at least 1");
        return new LineItem(description.Trim(), unitPrice, quantity, rate);
    }
}

/// <summary>Amounts per tax rate, as a qualified invoice must show them.</summary>
public record Totals(decimal Subtotal10, decimal Tax10, decimal Subtotal8, decimal Tax8)
{
    public decimal Subtotal => Subtotal10 + Subtotal8;
    public decimal Tax => Tax10 + Tax8;
    public decimal Total => Subtotal + Tax;
}

public class Invoice
{
    public required int Id { get; init; }
    public required Customer Customer { get; init; }
    public List<LineItem> Lines { get; init; } = new();
    public InvoiceStatus Status { get; set; } = InvoiceStatus.Draft;
    public string? Number { get; set; }            // "INV-2026-0001", given when issued
    public DateOnly? IssueDate { get; set; }
    public DateOnly? DueDate { get; set; }
    public DateOnly? PaidDate { get; set; }

    public static decimal RateOf(TaxRate rate) => rate == TaxRate.Standard ? 0.10m : 0.08m;

    /// <summary>
    /// Add up the lines per tax rate, then compute the tax ONCE per rate (not per line) and round
    /// it down to whole yen. That's the qualified-invoice rule: rounding per line is not allowed.
    /// Example: two 10% lines of ¥1,005 and ¥1,005: subtotal ¥2,010, tax floor(201.0) = ¥201.
    /// </summary>
    public Totals ComputeTotals()
    {
        throw new NotImplementedException();
    }

    public int DaysOverdue(DateOnly today) =>
        Status == InvoiceStatus.Issued && DueDate is { } due && today > due ? today.DayNumber - due.DayNumber : 0;
}
@@file InvoiceService.cs
namespace InvoiceManager;

// The use cases. The service depends on interfaces, which Program.cs (or a test) fills in:
// a JSON file or a fake repository, the real clock or a fixed date.

public interface IInvoiceRepository
{
    IReadOnlyList<Invoice> All();
    Invoice? Find(int id);
    void Save(Invoice invoice);
    int NextId();
}

public interface IClock
{
    DateOnly Today { get; }
}

public record AgingBucket(string Name, int Count, decimal Amount);

public class InvoiceService(IInvoiceRepository repo, IClock clock)
{
    public const int PaymentTermDays = 30;

    public Invoice CreateDraft(Customer customer)
    {
        if (string.IsNullOrWhiteSpace(customer.Name)) throw new DomainException("customer name is required");
        var invoice = new Invoice { Id = repo.NextId(), Customer = customer with { Name = customer.Name.Trim() } };
        repo.Save(invoice);
        return invoice;
    }

    public Invoice AddLine(int id, string description, decimal unitPrice, int quantity, TaxRate rate)
    {
        var invoice = Get(id);
        if (invoice.Status != InvoiceStatus.Draft) throw new DomainException("only drafts can be changed");
        invoice.Lines.Add(LineItem.Create(description, unitPrice, quantity, rate));
        repo.Save(invoice);
        return invoice;
    }

    /// <summary>
    /// Issue a draft: it needs at least one line ("an invoice needs at least one line") and must be
    /// a draft ("only drafts can be issued"). It gets the next number for this year, counting the
    /// invoices already issued with this year's prefix: "INV-2026-0001", "INV-2026-0002"... (a new
    /// year starts again at 0001). IssueDate is today; DueDate is today + PaymentTermDays.
    /// </summary>
    public Invoice Issue(int id)
    {
        throw new NotImplementedException();
    }

    public Invoice MarkPaid(int id, DateOnly paidOn)
    {
        var invoice = Get(id);
        if (invoice.Status != InvoiceStatus.Issued) throw new DomainException("only issued invoices can be paid");
        if (paidOn < invoice.IssueDate) throw new DomainException("payment date is before the issue date");
        invoice.Status = InvoiceStatus.Paid;
        invoice.PaidDate = paidOn;
        repo.Save(invoice);
        return invoice;
    }

    public Invoice Void(int id)
    {
        var invoice = Get(id);
        if (invoice.Status is InvoiceStatus.Paid or InvoiceStatus.Void) throw new DomainException($"a {invoice.Status.ToString().ToLower()} invoice can't be voided");
        invoice.Status = InvoiceStatus.Void;
        repo.Save(invoice);
        return invoice;
    }

    /// <summary>
    /// Unpaid issued invoices grouped by how late they are, for the finance team:
    /// "current" (not overdue yet), "1-30", "31-60" and "61+" days overdue. Always these four
    /// buckets in this order, with the number of invoices and the sum of their totals.
    /// </summary>
    public List<AgingBucket> AgingReport()
    {
        throw new NotImplementedException();
    }

    public IEnumerable<Invoice> Overdue() =>
        repo.All().Where(i => i.DaysOverdue(clock.Today) > 0).OrderByDescending(i => i.DaysOverdue(clock.Today));

    public Invoice Get(int id) => repo.Find(id) ?? throw new DomainException($"no invoice #{id}");
}
@@file Storage.cs
using System.Text.Json;
using System.Text.Json.Serialization;

namespace InvoiceManager;

// Adapters: the outside-world implementations of the service's interfaces.

public class SystemClock : IClock
{
    public DateOnly Today => DateOnly.FromDateTime(DateTime.Now);
}

public class FixedClock(DateOnly today) : IClock
{
    public DateOnly Today { get; set; } = today;
}

public class MemoryRepository : IInvoiceRepository
{
    protected readonly Dictionary<int, Invoice> Invoices = new();

    public IReadOnlyList<Invoice> All() => Invoices.Values.OrderBy(i => i.Id).ToList();
    public Invoice? Find(int id) => Invoices.GetValueOrDefault(id);
    public virtual void Save(Invoice invoice) => Invoices[invoice.Id] = invoice;
    public int NextId() => Invoices.Count == 0 ? 1 : Invoices.Keys.Max() + 1;
}

/// <summary>Keeps every invoice in one JSON file, rewritten safely (temp file + replace) on each save.</summary>
public class JsonFileRepository : MemoryRepository
{
    static readonly JsonSerializerOptions Options = new()
    {
        WriteIndented = true,
        Converters = { new JsonStringEnumConverter() },
    };

    readonly string path;

    public JsonFileRepository(string path)
    {
        this.path = path;
        if (!File.Exists(path)) return;
        var list = JsonSerializer.Deserialize<List<Invoice>>(File.ReadAllText(path), Options) ?? new();
        foreach (var invoice in list) Invoices[invoice.Id] = invoice;
    }

    public override void Save(Invoice invoice)
    {
        base.Save(invoice);
        string tmp = path + ".tmp";
        File.WriteAllText(tmp, JsonSerializer.Serialize(All(), Options));
        File.Move(tmp, path, overwrite: true);
    }
}
@@file Program.cs
using System.Globalization;
using InvoiceManager;

// dotnet run -- demo                     fill invoices.json with sample data
// dotnet run -- list | show 1 | aging | overdue
// dotnet run -- new "Sakura Cafe" billing@sakura.example
// dotnet run -- add 1 "Laptop setup" 15000 2 [reduced]
// dotnet run -- issue 1 | paid 1 2026-11-20 | void 1
// dotnet run -- --test
Console.OutputEncoding = System.Text.Encoding.UTF8;   // so ¥ prints correctly in Windows consoles
if (args.Contains("--test"))
    return Tests.RunAll();

var service = new InvoiceService(new JsonFileRepository("invoices.json"), new SystemClock());
string Yen(decimal amount) => "¥" + amount.ToString("N0", CultureInfo.InvariantCulture);
int Id(int index) => int.TryParse(args.ElementAtOrDefault(index), out int id) ? id : throw new DomainException("give an invoice number, like 1");

try
{
    switch (args.ElementAtOrDefault(0))
    {
        case "new":
            var draft = service.CreateDraft(new Customer(args.ElementAtOrDefault(1) ?? "", args.ElementAtOrDefault(2) ?? ""));
            Console.WriteLine($"draft #{draft.Id} for {draft.Customer.Name}");
            break;
        case "add":
            var rate = args.ElementAtOrDefault(5) == "reduced" ? TaxRate.Reduced : TaxRate.Standard;
            var changed = service.AddLine(Id(1), args.ElementAtOrDefault(2) ?? "",
                decimal.Parse(args.ElementAtOrDefault(3) ?? "x", CultureInfo.InvariantCulture), int.Parse(args.ElementAtOrDefault(4) ?? "1"), rate);
            Console.WriteLine($"#{changed.Id} now totals {Yen(changed.ComputeTotals().Total)}");
            break;
        case "issue":
            var issued = service.Issue(Id(1));
            Console.WriteLine($"issued {issued.Number}, due {issued.DueDate:yyyy-MM-dd}");
            break;
        case "paid":
            service.MarkPaid(Id(1), DateOnly.Parse(args.ElementAtOrDefault(2) ?? DateTime.Today.ToString("yyyy-MM-dd"), CultureInfo.InvariantCulture));
            Console.WriteLine("marked as paid");
            break;
        case "void":
            service.Void(Id(1));
            Console.WriteLine("voided");
            break;
        case "show":
            Print(service.Get(Id(1)));
            break;
        case "aging":
            foreach (var b in service.AgingReport())
                Console.WriteLine($"{b.Name,-8} {b.Count,3} invoice(s) {Yen(b.Amount),12}");
            break;
        case "overdue":
            foreach (var i in service.Overdue())
                Console.WriteLine($"{i.Number} {i.Customer.Name,-20} {i.DaysOverdue(DateOnly.FromDateTime(DateTime.Today)),4} days {Yen(i.ComputeTotals().Total),10}");
            break;
        case "demo":
            var a = service.CreateDraft(new Customer("Sakura Cafe", "billing@sakura.example"));
            service.AddLine(a.Id, "PC setup and Wi-Fi survey", 45000, 1, TaxRate.Standard);
            service.AddLine(a.Id, "Coffee beans for the office", 1800, 3, TaxRate.Reduced);
            service.Issue(a.Id);
            var b2 = service.CreateDraft(new Customer("Kaede Dental", "office@kaede.example"));
            service.AddLine(b2.Id, "Monthly IT support", 30000, 1, TaxRate.Standard);
            Console.WriteLine("created two invoices: try `dotnet run -- list`");
            break;
        default:
            foreach (var i in new JsonFileRepository("invoices.json").All())
                Console.WriteLine($"#{i.Id,-3} {i.Number ?? "(draft)",-14} {i.Status,-7} {i.Customer.Name,-20} {Yen(i.ComputeTotals().Total),10}");
            break;
    }
    return 0;
}
catch (Exception e) when (e is DomainException or FormatException)
{
    Console.Error.WriteLine($"error: {e.Message}");
    return 1;
}

void Print(Invoice i)
{
    var t = i.ComputeTotals();
    Console.WriteLine($"{i.Number ?? "DRAFT"}   {i.Status}");
    Console.WriteLine($"To: {i.Customer.Name} <{i.Customer.Email}>");
    if (i.IssueDate is { } issued) Console.WriteLine($"Issued {issued:yyyy-MM-dd}, due {i.DueDate:yyyy-MM-dd}");
    foreach (var l in i.Lines)
        Console.WriteLine($"  {l.Description,-32} {Yen(l.UnitPrice),9} × {l.Quantity,-3} {Yen(l.Amount),10} {(l.Rate == TaxRate.Reduced ? "8%*" : "10%")}");
    Console.WriteLine($"  10% subtotal {Yen(t.Subtotal10),10}   tax {Yen(t.Tax10),8}");
    Console.WriteLine($"   8% subtotal {Yen(t.Subtotal8),10}   tax {Yen(t.Tax8),8}   (* reduced rate)");
    Console.WriteLine($"  TOTAL {Yen(t.Total)}");
}
@@file Tests.cs
namespace InvoiceManager;

/// <summary>A tiny test runner, so the project needs no test framework. Run: dotnet run -- --test</summary>
public static class Tests
{
    static int passed, failed;

    static void Check(string name, Func<bool> test)
    {
        try
        {
            if (test()) { passed++; Console.WriteLine($"  ok    {name}"); return; }
            Console.WriteLine($"  FAIL  {name}");
        }
        catch (NotImplementedException)
        {
            Console.WriteLine($"  TODO  {name} (not written yet)");
        }
        catch (Exception e)
        {
            Console.WriteLine($"  FAIL  {name}: {e.GetType().Name}: {e.Message}");
        }
        failed++;
    }

    static string Error(Action a)
    {
        try { a(); return "no error"; }
        catch (DomainException e) { return e.Message; }
    }

    static (InvoiceService, FixedClock, MemoryRepository) Setup()
    {
        var clock = new FixedClock(new DateOnly(2026, 11, 2));
        var repo = new MemoryRepository();
        return (new InvoiceService(repo, clock), clock, repo);
    }

    static Invoice Ready(InvoiceService s, decimal price = 10000)
    {
        var i = s.CreateDraft(new Customer("Sakura Cafe", "billing@sakura.example"));
        s.AddLine(i.Id, "Support", price, 1, TaxRate.Standard);
        return i;
    }

    public static int RunAll()
    {
        Console.WriteLine("Totals");
        Check("tax is per rate, rounded down once", () =>
        {
            var i = new Invoice { Id = 1, Customer = new("A", "a@x") };
            i.Lines.Add(LineItem.Create("Cable", 1005, 1, TaxRate.Standard));
            i.Lines.Add(LineItem.Create("Adapter", 1005, 1, TaxRate.Standard));
            i.Lines.Add(LineItem.Create("Coffee", 999, 3, TaxRate.Reduced));
            return i.ComputeTotals() == new Totals(2010, 201, 2997, 239) && i.ComputeTotals().Total == 5447;
        });
        Check("an empty invoice is all zeros", () => new Invoice { Id = 1, Customer = new("A", "a@x") }.ComputeTotals().Total == 0);
        Check("line validation", () =>
            Error(() => LineItem.Create(" ", 1, 1, TaxRate.Standard)) == "description is required"
            && Error(() => LineItem.Create("x", 10.5m, 1, TaxRate.Standard)) == "unit price must be whole yen, 0 or more"
            && Error(() => LineItem.Create("x", 10, 0, TaxRate.Standard)) == "quantity must be at least 1");

        Console.WriteLine("Issuing");
        Check("numbers count up per year", () =>
        {
            var (s, clock, _) = Setup();
            var a = s.Issue(Ready(s).Id);
            var b = s.Issue(Ready(s).Id);
            clock.Today = new DateOnly(2027, 1, 5);
            var c = s.Issue(Ready(s).Id);
            return a.Number == "INV-2026-0001" && b.Number == "INV-2026-0002" && c.Number == "INV-2027-0001";
        });
        Check("drafts don't use up numbers", () =>
        {
            var (s, _, _) = Setup();
            Ready(s);
            return s.Issue(Ready(s).Id).Number == "INV-2026-0001";
        });
        Check("dates and status", () =>
        {
            var (s, _, _) = Setup();
            var i = s.Issue(Ready(s).Id);
            return i.Status == InvoiceStatus.Issued && i.IssueDate == new DateOnly(2026, 11, 2) && i.DueDate == new DateOnly(2026, 12, 2);
        });
        Check("rules", () =>
        {
            var (s, _, _) = Setup();
            var empty = s.CreateDraft(new Customer("Kaede", "k@x"));
            var i = s.Issue(Ready(s).Id);
            return Error(() => s.Issue(empty.Id)) == "an invoice needs at least one line"
                && Error(() => s.Issue(i.Id)) == "only drafts can be issued"
                && Error(() => s.AddLine(i.Id, "More", 1, 1, TaxRate.Standard)) == "only drafts can be changed"
                && Error(() => s.Issue(99)) == "no invoice #99";
        });

        Console.WriteLine("Payments and voiding");
        Check("pay and void", () =>
        {
            var (s, _, _) = Setup();
            var i = s.Issue(Ready(s).Id);
            var draft = Ready(s);
            return Error(() => s.MarkPaid(draft.Id, new DateOnly(2026, 11, 3))) == "only issued invoices can be paid"
                && Error(() => s.MarkPaid(i.Id, new DateOnly(2026, 11, 1))) == "payment date is before the issue date"
                && s.MarkPaid(i.Id, new DateOnly(2026, 11, 20)).Status == InvoiceStatus.Paid
                && Error(() => s.Void(i.Id)) == "a paid invoice can't be voided"
                && s.Void(draft.Id).Status == InvoiceStatus.Void;
        });

        Console.WriteLine("Aging report");
        Check("buckets by days overdue", () =>
        {
            var (s, clock, _) = Setup();
            var old = s.Issue(Ready(s, 1000).Id);              // due 2026-12-02
            clock.Today = new DateOnly(2026, 11, 20);
            var mid = s.Issue(Ready(s, 2000).Id);              // due 2026-12-20
            clock.Today = new DateOnly(2027, 1, 10);
            var fresh = s.Issue(Ready(s, 3000).Id);            // due 2027-02-09
            var paid = s.Issue(Ready(s, 5000).Id);
            s.MarkPaid(paid.Id, clock.Today);
            clock.Today = new DateOnly(2027, 2, 3);            // old: 63 days late, mid: 45, fresh: not due
            var report = s.AgingReport();
            return report.Select(b => b.Name).SequenceEqual(new[] { "current", "1-30", "31-60", "61+" })
                && report[0] == new AgingBucket("current", 1, 3300)
                && report[1] == new AgingBucket("1-30", 0, 0)
                && report[2] == new AgingBucket("31-60", 1, 2200)
                && report[3] == new AgingBucket("61+", 1, 1100);
        });
        Check("the due date itself isn't overdue", () =>
        {
            var (s, clock, _) = Setup();
            s.Issue(Ready(s).Id);
            clock.Today = new DateOnly(2026, 12, 2);
            return s.AgingReport()[0].Count == 1 && !s.Overdue().Any();
        });

        Console.WriteLine("Storage");
        Check("JSON file round trip", () =>
        {
            string path = Path.Combine(Path.GetTempPath(), $"invoices-test-{Guid.NewGuid():N}.json");
            try
            {
                var s = new InvoiceService(new JsonFileRepository(path), new FixedClock(new DateOnly(2026, 11, 2)));
                var i = s.CreateDraft(new Customer("Sakura Cafe", "billing@sakura.example"));
                s.AddLine(i.Id, "Coffee", 999, 3, TaxRate.Reduced);
                s.Issue(i.Id);
                var loaded = new JsonFileRepository(path).Find(i.Id)!;
                return loaded.Number == "INV-2026-0001" && loaded.Status == InvoiceStatus.Issued
                    && loaded.Lines.Single().Rate == TaxRate.Reduced && loaded.ComputeTotals().Total == 3236;
            }
            finally { File.Delete(path); }
        });

        Console.WriteLine($"\n{passed} passed, {failed} failed");
        return failed == 0 ? 0 : 1;
    }
}
@@file InvoiceManager.csproj
<Project Sdk="Microsoft.NET.Sdk">

  <PropertyGroup>
    <OutputType>Exe</OutputType>
    <TargetFramework>net8.0</TargetFramework>
    <ImplicitUsings>enable</ImplicitUsings>
    <Nullable>enable</Nullable>
    <!-- Same number and date formatting on every computer. -->
    <InvariantGlobalization>true</InvariantGlobalization>
    <RollForward>Major</RollForward>
  </PropertyGroup>

</Project>
@@text
## Stretch goals

- **The registration number**: qualified invoices must show the issuer's number, `T` + 13 digits. Add the issuer's details to every invoice and validate the number.
- **A PDF**: generate a printable invoice with QuestPDF (a NuGet package). Only the presentation layer changes.
- **A web API**: an ASP.NET Core minimal API project that references this code: `app.MapPost("/invoices/{id}/issue", (int id, InvoiceService s) => s.Issue(id));`. Register `InvoiceService`, the repository and the clock with `builder.Services`.
- **A real database**: an EF Core `DbContext` and an `EfInvoiceRepository : IInvoiceRepository`. Your tests keep using `MemoryRepository`.
- **Credit notes**: instead of voiding a paid invoice, issue a credit note (`CN-2026-0001`) with negative lines that references it.
- **Concurrency**: two people issue invoices at the same moment and get the same number. How would a database sequence or a unique index solve it?
@@project invoices_finished Invoice Manager: the finished version
A complete solution to compare with yours.
@@file Domain.cs
namespace InvoiceManager;

// The domain: invoices and the rules of Japan's Qualified Invoice System (インボイス制度).
// No files, no console, no clock: everything here is easy to test.

public enum TaxRate { Standard, Reduced }          // 10%, and 8% for food and newspapers
public enum InvoiceStatus { Draft, Issued, Paid, Void }

public class DomainException(string message) : Exception(message);

public record Customer(string Name, string Email);

public record LineItem(string Description, decimal UnitPrice, int Quantity, TaxRate Rate)
{
    public decimal Amount => UnitPrice * Quantity;

    public static LineItem Create(string description, decimal unitPrice, int quantity, TaxRate rate)
    {
        if (string.IsNullOrWhiteSpace(description)) throw new DomainException("description is required");
        if (unitPrice < 0 || unitPrice != decimal.Truncate(unitPrice)) throw new DomainException("unit price must be whole yen, 0 or more");
        if (quantity < 1) throw new DomainException("quantity must be at least 1");
        return new LineItem(description.Trim(), unitPrice, quantity, rate);
    }
}

/// <summary>Amounts per tax rate, as a qualified invoice must show them.</summary>
public record Totals(decimal Subtotal10, decimal Tax10, decimal Subtotal8, decimal Tax8)
{
    public decimal Subtotal => Subtotal10 + Subtotal8;
    public decimal Tax => Tax10 + Tax8;
    public decimal Total => Subtotal + Tax;
}

public class Invoice
{
    public required int Id { get; init; }
    public required Customer Customer { get; init; }
    public List<LineItem> Lines { get; init; } = new();
    public InvoiceStatus Status { get; set; } = InvoiceStatus.Draft;
    public string? Number { get; set; }            // "INV-2026-0001", given when issued
    public DateOnly? IssueDate { get; set; }
    public DateOnly? DueDate { get; set; }
    public DateOnly? PaidDate { get; set; }

    public static decimal RateOf(TaxRate rate) => rate == TaxRate.Standard ? 0.10m : 0.08m;

    /// <summary>
    /// Add up the lines per tax rate, then compute the tax ONCE per rate (not per line) and round
    /// it down to whole yen. That's the qualified-invoice rule: rounding per line is not allowed.
    /// Example: two 10% lines of ¥1,005 and ¥1,005: subtotal ¥2,010, tax floor(201.0) = ¥201.
    /// </summary>
    public Totals ComputeTotals()
    {
        decimal sub10 = Lines.Where(l => l.Rate == TaxRate.Standard).Sum(l => l.Amount);
        decimal sub8 = Lines.Where(l => l.Rate == TaxRate.Reduced).Sum(l => l.Amount);
        return new Totals(sub10, Math.Floor(sub10 * RateOf(TaxRate.Standard)), sub8, Math.Floor(sub8 * RateOf(TaxRate.Reduced)));
    }

    public int DaysOverdue(DateOnly today) =>
        Status == InvoiceStatus.Issued && DueDate is { } due && today > due ? today.DayNumber - due.DayNumber : 0;
}
@@file InvoiceService.cs
namespace InvoiceManager;

// The use cases. The service depends on interfaces, which Program.cs (or a test) fills in:
// a JSON file or a fake repository, the real clock or a fixed date.

public interface IInvoiceRepository
{
    IReadOnlyList<Invoice> All();
    Invoice? Find(int id);
    void Save(Invoice invoice);
    int NextId();
}

public interface IClock
{
    DateOnly Today { get; }
}

public record AgingBucket(string Name, int Count, decimal Amount);

public class InvoiceService(IInvoiceRepository repo, IClock clock)
{
    public const int PaymentTermDays = 30;

    public Invoice CreateDraft(Customer customer)
    {
        if (string.IsNullOrWhiteSpace(customer.Name)) throw new DomainException("customer name is required");
        var invoice = new Invoice { Id = repo.NextId(), Customer = customer with { Name = customer.Name.Trim() } };
        repo.Save(invoice);
        return invoice;
    }

    public Invoice AddLine(int id, string description, decimal unitPrice, int quantity, TaxRate rate)
    {
        var invoice = Get(id);
        if (invoice.Status != InvoiceStatus.Draft) throw new DomainException("only drafts can be changed");
        invoice.Lines.Add(LineItem.Create(description, unitPrice, quantity, rate));
        repo.Save(invoice);
        return invoice;
    }

    /// <summary>
    /// Issue a draft: it needs at least one line ("an invoice needs at least one line") and must be
    /// a draft ("only drafts can be issued"). It gets the next number for this year, counting the
    /// invoices already issued with this year's prefix: "INV-2026-0001", "INV-2026-0002"... (a new
    /// year starts again at 0001). IssueDate is today; DueDate is today + PaymentTermDays.
    /// </summary>
    public Invoice Issue(int id)
    {
        var invoice = Get(id);
        if (invoice.Status != InvoiceStatus.Draft) throw new DomainException("only drafts can be issued");
        if (invoice.Lines.Count == 0) throw new DomainException("an invoice needs at least one line");
        var today = clock.Today;
        string prefix = $"INV-{today.Year}-";
        int count = repo.All().Count(i => i.Number?.StartsWith(prefix) == true);
        invoice.Number = $"{prefix}{count + 1:D4}";
        invoice.Status = InvoiceStatus.Issued;
        invoice.IssueDate = today;
        invoice.DueDate = today.AddDays(PaymentTermDays);
        repo.Save(invoice);
        return invoice;
    }

    public Invoice MarkPaid(int id, DateOnly paidOn)
    {
        var invoice = Get(id);
        if (invoice.Status != InvoiceStatus.Issued) throw new DomainException("only issued invoices can be paid");
        if (paidOn < invoice.IssueDate) throw new DomainException("payment date is before the issue date");
        invoice.Status = InvoiceStatus.Paid;
        invoice.PaidDate = paidOn;
        repo.Save(invoice);
        return invoice;
    }

    public Invoice Void(int id)
    {
        var invoice = Get(id);
        if (invoice.Status is InvoiceStatus.Paid or InvoiceStatus.Void) throw new DomainException($"a {invoice.Status.ToString().ToLower()} invoice can't be voided");
        invoice.Status = InvoiceStatus.Void;
        repo.Save(invoice);
        return invoice;
    }

    /// <summary>
    /// Unpaid issued invoices grouped by how late they are, for the finance team:
    /// "current" (not overdue yet), "1-30", "31-60" and "61+" days overdue. Always these four
    /// buckets in this order, with the number of invoices and the sum of their totals.
    /// </summary>
    public List<AgingBucket> AgingReport()
    {
        var today = clock.Today;
        string BucketOf(int days) => days switch { 0 => "current", <= 30 => "1-30", <= 60 => "31-60", _ => "61+" };
        var unpaid = repo.All().Where(i => i.Status == InvoiceStatus.Issued).ToList();
        return new[] { "current", "1-30", "31-60", "61+" }
            .Select(name =>
            {
                var inBucket = unpaid.Where(i => BucketOf(i.DaysOverdue(today)) == name).ToList();
                return new AgingBucket(name, inBucket.Count, inBucket.Sum(i => i.ComputeTotals().Total));
            })
            .ToList();
    }

    public IEnumerable<Invoice> Overdue() =>
        repo.All().Where(i => i.DaysOverdue(clock.Today) > 0).OrderByDescending(i => i.DaysOverdue(clock.Today));

    public Invoice Get(int id) => repo.Find(id) ?? throw new DomainException($"no invoice #{id}");
}
@@file Storage.cs
using System.Text.Json;
using System.Text.Json.Serialization;

namespace InvoiceManager;

// Adapters: the outside-world implementations of the service's interfaces.

public class SystemClock : IClock
{
    public DateOnly Today => DateOnly.FromDateTime(DateTime.Now);
}

public class FixedClock(DateOnly today) : IClock
{
    public DateOnly Today { get; set; } = today;
}

public class MemoryRepository : IInvoiceRepository
{
    protected readonly Dictionary<int, Invoice> Invoices = new();

    public IReadOnlyList<Invoice> All() => Invoices.Values.OrderBy(i => i.Id).ToList();
    public Invoice? Find(int id) => Invoices.GetValueOrDefault(id);
    public virtual void Save(Invoice invoice) => Invoices[invoice.Id] = invoice;
    public int NextId() => Invoices.Count == 0 ? 1 : Invoices.Keys.Max() + 1;
}

/// <summary>Keeps every invoice in one JSON file, rewritten safely (temp file + replace) on each save.</summary>
public class JsonFileRepository : MemoryRepository
{
    static readonly JsonSerializerOptions Options = new()
    {
        WriteIndented = true,
        Converters = { new JsonStringEnumConverter() },
    };

    readonly string path;

    public JsonFileRepository(string path)
    {
        this.path = path;
        if (!File.Exists(path)) return;
        var list = JsonSerializer.Deserialize<List<Invoice>>(File.ReadAllText(path), Options) ?? new();
        foreach (var invoice in list) Invoices[invoice.Id] = invoice;
    }

    public override void Save(Invoice invoice)
    {
        base.Save(invoice);
        string tmp = path + ".tmp";
        File.WriteAllText(tmp, JsonSerializer.Serialize(All(), Options));
        File.Move(tmp, path, overwrite: true);
    }
}
@@file Program.cs
using System.Globalization;
using InvoiceManager;

// dotnet run -- demo                     fill invoices.json with sample data
// dotnet run -- list | show 1 | aging | overdue
// dotnet run -- new "Sakura Cafe" billing@sakura.example
// dotnet run -- add 1 "Laptop setup" 15000 2 [reduced]
// dotnet run -- issue 1 | paid 1 2026-11-20 | void 1
// dotnet run -- --test
Console.OutputEncoding = System.Text.Encoding.UTF8;   // so ¥ prints correctly in Windows consoles
if (args.Contains("--test"))
    return Tests.RunAll();

var service = new InvoiceService(new JsonFileRepository("invoices.json"), new SystemClock());
string Yen(decimal amount) => "¥" + amount.ToString("N0", CultureInfo.InvariantCulture);
int Id(int index) => int.TryParse(args.ElementAtOrDefault(index), out int id) ? id : throw new DomainException("give an invoice number, like 1");

try
{
    switch (args.ElementAtOrDefault(0))
    {
        case "new":
            var draft = service.CreateDraft(new Customer(args.ElementAtOrDefault(1) ?? "", args.ElementAtOrDefault(2) ?? ""));
            Console.WriteLine($"draft #{draft.Id} for {draft.Customer.Name}");
            break;
        case "add":
            var rate = args.ElementAtOrDefault(5) == "reduced" ? TaxRate.Reduced : TaxRate.Standard;
            var changed = service.AddLine(Id(1), args.ElementAtOrDefault(2) ?? "",
                decimal.Parse(args.ElementAtOrDefault(3) ?? "x", CultureInfo.InvariantCulture), int.Parse(args.ElementAtOrDefault(4) ?? "1"), rate);
            Console.WriteLine($"#{changed.Id} now totals {Yen(changed.ComputeTotals().Total)}");
            break;
        case "issue":
            var issued = service.Issue(Id(1));
            Console.WriteLine($"issued {issued.Number}, due {issued.DueDate:yyyy-MM-dd}");
            break;
        case "paid":
            service.MarkPaid(Id(1), DateOnly.Parse(args.ElementAtOrDefault(2) ?? DateTime.Today.ToString("yyyy-MM-dd"), CultureInfo.InvariantCulture));
            Console.WriteLine("marked as paid");
            break;
        case "void":
            service.Void(Id(1));
            Console.WriteLine("voided");
            break;
        case "show":
            Print(service.Get(Id(1)));
            break;
        case "aging":
            foreach (var b in service.AgingReport())
                Console.WriteLine($"{b.Name,-8} {b.Count,3} invoice(s) {Yen(b.Amount),12}");
            break;
        case "overdue":
            foreach (var i in service.Overdue())
                Console.WriteLine($"{i.Number} {i.Customer.Name,-20} {i.DaysOverdue(DateOnly.FromDateTime(DateTime.Today)),4} days {Yen(i.ComputeTotals().Total),10}");
            break;
        case "demo":
            var a = service.CreateDraft(new Customer("Sakura Cafe", "billing@sakura.example"));
            service.AddLine(a.Id, "PC setup and Wi-Fi survey", 45000, 1, TaxRate.Standard);
            service.AddLine(a.Id, "Coffee beans for the office", 1800, 3, TaxRate.Reduced);
            service.Issue(a.Id);
            var b2 = service.CreateDraft(new Customer("Kaede Dental", "office@kaede.example"));
            service.AddLine(b2.Id, "Monthly IT support", 30000, 1, TaxRate.Standard);
            Console.WriteLine("created two invoices: try `dotnet run -- list`");
            break;
        default:
            foreach (var i in new JsonFileRepository("invoices.json").All())
                Console.WriteLine($"#{i.Id,-3} {i.Number ?? "(draft)",-14} {i.Status,-7} {i.Customer.Name,-20} {Yen(i.ComputeTotals().Total),10}");
            break;
    }
    return 0;
}
catch (Exception e) when (e is DomainException or FormatException)
{
    Console.Error.WriteLine($"error: {e.Message}");
    return 1;
}

void Print(Invoice i)
{
    var t = i.ComputeTotals();
    Console.WriteLine($"{i.Number ?? "DRAFT"}   {i.Status}");
    Console.WriteLine($"To: {i.Customer.Name} <{i.Customer.Email}>");
    if (i.IssueDate is { } issued) Console.WriteLine($"Issued {issued:yyyy-MM-dd}, due {i.DueDate:yyyy-MM-dd}");
    foreach (var l in i.Lines)
        Console.WriteLine($"  {l.Description,-32} {Yen(l.UnitPrice),9} × {l.Quantity,-3} {Yen(l.Amount),10} {(l.Rate == TaxRate.Reduced ? "8%*" : "10%")}");
    Console.WriteLine($"  10% subtotal {Yen(t.Subtotal10),10}   tax {Yen(t.Tax10),8}");
    Console.WriteLine($"   8% subtotal {Yen(t.Subtotal8),10}   tax {Yen(t.Tax8),8}   (* reduced rate)");
    Console.WriteLine($"  TOTAL {Yen(t.Total)}");
}
@@file Tests.cs
namespace InvoiceManager;

/// <summary>A tiny test runner, so the project needs no test framework. Run: dotnet run -- --test</summary>
public static class Tests
{
    static int passed, failed;

    static void Check(string name, Func<bool> test)
    {
        try
        {
            if (test()) { passed++; Console.WriteLine($"  ok    {name}"); return; }
            Console.WriteLine($"  FAIL  {name}");
        }
        catch (NotImplementedException)
        {
            Console.WriteLine($"  TODO  {name} (not written yet)");
        }
        catch (Exception e)
        {
            Console.WriteLine($"  FAIL  {name}: {e.GetType().Name}: {e.Message}");
        }
        failed++;
    }

    static string Error(Action a)
    {
        try { a(); return "no error"; }
        catch (DomainException e) { return e.Message; }
    }

    static (InvoiceService, FixedClock, MemoryRepository) Setup()
    {
        var clock = new FixedClock(new DateOnly(2026, 11, 2));
        var repo = new MemoryRepository();
        return (new InvoiceService(repo, clock), clock, repo);
    }

    static Invoice Ready(InvoiceService s, decimal price = 10000)
    {
        var i = s.CreateDraft(new Customer("Sakura Cafe", "billing@sakura.example"));
        s.AddLine(i.Id, "Support", price, 1, TaxRate.Standard);
        return i;
    }

    public static int RunAll()
    {
        Console.WriteLine("Totals");
        Check("tax is per rate, rounded down once", () =>
        {
            var i = new Invoice { Id = 1, Customer = new("A", "a@x") };
            i.Lines.Add(LineItem.Create("Cable", 1005, 1, TaxRate.Standard));
            i.Lines.Add(LineItem.Create("Adapter", 1005, 1, TaxRate.Standard));
            i.Lines.Add(LineItem.Create("Coffee", 999, 3, TaxRate.Reduced));
            return i.ComputeTotals() == new Totals(2010, 201, 2997, 239) && i.ComputeTotals().Total == 5447;
        });
        Check("an empty invoice is all zeros", () => new Invoice { Id = 1, Customer = new("A", "a@x") }.ComputeTotals().Total == 0);
        Check("line validation", () =>
            Error(() => LineItem.Create(" ", 1, 1, TaxRate.Standard)) == "description is required"
            && Error(() => LineItem.Create("x", 10.5m, 1, TaxRate.Standard)) == "unit price must be whole yen, 0 or more"
            && Error(() => LineItem.Create("x", 10, 0, TaxRate.Standard)) == "quantity must be at least 1");

        Console.WriteLine("Issuing");
        Check("numbers count up per year", () =>
        {
            var (s, clock, _) = Setup();
            var a = s.Issue(Ready(s).Id);
            var b = s.Issue(Ready(s).Id);
            clock.Today = new DateOnly(2027, 1, 5);
            var c = s.Issue(Ready(s).Id);
            return a.Number == "INV-2026-0001" && b.Number == "INV-2026-0002" && c.Number == "INV-2027-0001";
        });
        Check("drafts don't use up numbers", () =>
        {
            var (s, _, _) = Setup();
            Ready(s);
            return s.Issue(Ready(s).Id).Number == "INV-2026-0001";
        });
        Check("dates and status", () =>
        {
            var (s, _, _) = Setup();
            var i = s.Issue(Ready(s).Id);
            return i.Status == InvoiceStatus.Issued && i.IssueDate == new DateOnly(2026, 11, 2) && i.DueDate == new DateOnly(2026, 12, 2);
        });
        Check("rules", () =>
        {
            var (s, _, _) = Setup();
            var empty = s.CreateDraft(new Customer("Kaede", "k@x"));
            var i = s.Issue(Ready(s).Id);
            return Error(() => s.Issue(empty.Id)) == "an invoice needs at least one line"
                && Error(() => s.Issue(i.Id)) == "only drafts can be issued"
                && Error(() => s.AddLine(i.Id, "More", 1, 1, TaxRate.Standard)) == "only drafts can be changed"
                && Error(() => s.Issue(99)) == "no invoice #99";
        });

        Console.WriteLine("Payments and voiding");
        Check("pay and void", () =>
        {
            var (s, _, _) = Setup();
            var i = s.Issue(Ready(s).Id);
            var draft = Ready(s);
            return Error(() => s.MarkPaid(draft.Id, new DateOnly(2026, 11, 3))) == "only issued invoices can be paid"
                && Error(() => s.MarkPaid(i.Id, new DateOnly(2026, 11, 1))) == "payment date is before the issue date"
                && s.MarkPaid(i.Id, new DateOnly(2026, 11, 20)).Status == InvoiceStatus.Paid
                && Error(() => s.Void(i.Id)) == "a paid invoice can't be voided"
                && s.Void(draft.Id).Status == InvoiceStatus.Void;
        });

        Console.WriteLine("Aging report");
        Check("buckets by days overdue", () =>
        {
            var (s, clock, _) = Setup();
            var old = s.Issue(Ready(s, 1000).Id);              // due 2026-12-02
            clock.Today = new DateOnly(2026, 11, 20);
            var mid = s.Issue(Ready(s, 2000).Id);              // due 2026-12-20
            clock.Today = new DateOnly(2027, 1, 10);
            var fresh = s.Issue(Ready(s, 3000).Id);            // due 2027-02-09
            var paid = s.Issue(Ready(s, 5000).Id);
            s.MarkPaid(paid.Id, clock.Today);
            clock.Today = new DateOnly(2027, 2, 3);            // old: 63 days late, mid: 45, fresh: not due
            var report = s.AgingReport();
            return report.Select(b => b.Name).SequenceEqual(new[] { "current", "1-30", "31-60", "61+" })
                && report[0] == new AgingBucket("current", 1, 3300)
                && report[1] == new AgingBucket("1-30", 0, 0)
                && report[2] == new AgingBucket("31-60", 1, 2200)
                && report[3] == new AgingBucket("61+", 1, 1100);
        });
        Check("the due date itself isn't overdue", () =>
        {
            var (s, clock, _) = Setup();
            s.Issue(Ready(s).Id);
            clock.Today = new DateOnly(2026, 12, 2);
            return s.AgingReport()[0].Count == 1 && !s.Overdue().Any();
        });

        Console.WriteLine("Storage");
        Check("JSON file round trip", () =>
        {
            string path = Path.Combine(Path.GetTempPath(), $"invoices-test-{Guid.NewGuid():N}.json");
            try
            {
                var s = new InvoiceService(new JsonFileRepository(path), new FixedClock(new DateOnly(2026, 11, 2)));
                var i = s.CreateDraft(new Customer("Sakura Cafe", "billing@sakura.example"));
                s.AddLine(i.Id, "Coffee", 999, 3, TaxRate.Reduced);
                s.Issue(i.Id);
                var loaded = new JsonFileRepository(path).Find(i.Id)!;
                return loaded.Number == "INV-2026-0001" && loaded.Status == InvoiceStatus.Issued
                    && loaded.Lines.Single().Rate == TaxRate.Reduced && loaded.ComputeTotals().Total == 3236;
            }
            finally { File.Delete(path); }
        });

        Console.WriteLine($"\n{passed} passed, {failed} failed");
        return failed == 0 ? 0 : 1;
    }
}
@@file InvoiceManager.csproj
<Project Sdk="Microsoft.NET.Sdk">

  <PropertyGroup>
    <OutputType>Exe</OutputType>
    <TargetFramework>net8.0</TargetFramework>
    <ImplicitUsings>enable</ImplicitUsings>
    <Nullable>enable</Nullable>
    <!-- Same number and date formatting on every computer. -->
    <InvariantGlobalization>true</InvariantGlobalization>
    <RollForward>Major</RollForward>
  </PropertyGroup>

</Project>

@@drill
Which C# type should you use for money?
= decimal
? double can't store 0.1 exactly.

@@text
🎉 Line-of-business applications like this one (invoicing, inventory, HR, booking) are what most .NET developers in Japan build, often for 業務システム projects. You now know the shape of them: a domain with rules, use cases behind interfaces, swappable storage, and tests that run in milliseconds.

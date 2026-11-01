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

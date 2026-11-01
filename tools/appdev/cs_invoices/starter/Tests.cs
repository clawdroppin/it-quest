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

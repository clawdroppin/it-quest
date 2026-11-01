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

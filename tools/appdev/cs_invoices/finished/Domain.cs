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

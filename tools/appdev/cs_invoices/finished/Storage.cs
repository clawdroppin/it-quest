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

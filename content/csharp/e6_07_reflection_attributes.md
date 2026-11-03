@@lesson e6-reflection Reflection, Attributes and Source Generators
@@concepts objects, modules
@@text
ASP.NET Core finds your controllers, xUnit finds your `[Fact]` methods, System.Text.Json reads `[JsonPropertyName]`, and EF Core maps classes to tables. They all inspect your code's **metadata** at run time (reflection) or at compile time (source generators).

## Reflection basics

```csharp
using System.Reflection;

var t = typeof(Server);
Console.WriteLine($"{t.Name} in {t.Namespace ?? "(global)"}, {t.GetProperties().Length} properties");
foreach (var p in t.GetProperties())
    Console.WriteLine($"  {p.Name}: {p.PropertyType.Name}");

var s = new Server { Name = "web01", Cores = 8 };
var prop = t.GetProperty("Cores")!;
prop.SetValue(s, 16);                                   // set by name
Console.WriteLine(prop.GetValue(s));
var created = (Server)Activator.CreateInstance(typeof(Server))!;  // create by type
Console.WriteLine(created.Name == "");
Console.WriteLine(typeof(List<int>).GetGenericTypeDefinition().Name); // List`1

class Server
{
    public string Name { get; set; } = "";
    public int Cores { get; set; }
}
```

- `typeof(T)` at compile time, `obj.GetType()` at run time.
- `GetProperties`, `GetMethods`, `GetFields` with `BindingFlags` for non-public or static members.
- `MethodInfo.Invoke`, `PropertyInfo.GetValue/SetValue`, `Activator.CreateInstance`.
- `Assembly.GetExecutingAssembly().GetTypes()` to discover types: how plugin systems and test runners work.

## Attributes

Attributes attach metadata to code. You define one by deriving from `Attribute`:

```csharp
using System.Reflection;

foreach (var m in typeof(Jobs).GetMethods().Where(m => m.GetCustomAttribute<ScheduleAttribute>() is not null))
{
    var a = m.GetCustomAttribute<ScheduleAttribute>()!;
    Console.WriteLine($"{m.Name} runs {a.Cron}");
}

[AttributeUsage(AttributeTargets.Method)]
class ScheduleAttribute(string cron) : Attribute
{
    public string Cron { get; } = cron;
}

class Jobs
{
    [Schedule("0 2 * * *")] public void Backup() { }
    [Schedule("*/5 * * * *")] public void HealthCheck() { }
    public void Helper() { }
}
```

Common built-in ones: `[Obsolete]`, `[Flags]`, `[Serializable]`, `[JsonPropertyName("x")]`, `[Required]`/`[Range]` (DataAnnotations), `[CallerMemberName]` (the compiler fills in the caller's name: used by `INotifyPropertyChanged`).

## nameof

`nameof(Server.Cores)` gives `"Cores"` at compile time, so renaming the property updates the string too. Prefer it over string literals for property names, argument names and log messages.

## The cost of reflection

Reflection is slower than direct calls and invisible to the compiler: renaming a property can break code that looks it up by name. It also fights **trimming** and **Native AOT** (publishing a small, fast-starting self-contained binary), because the trimmer can't see what you'll look up at run time.

## Source generators

A source generator runs **inside the compiler**, reads your code and adds more C# code. The result is as fast as hand-written code and AOT-friendly:

```csharp,norun
[JsonSerializable(typeof(Server))]
partial class AppJsonContext : JsonSerializerContext { }          // System.Text.Json generates the serializer

[GeneratedRegex(@"^\d{3}-\d{4}$")]
private static partial Regex PostalCode();                         // regex compiled at build time

[LoggerMessage(Level = LogLevel.Warning, Message = "Disk {Drive} at {Percent}%")]
static partial void DiskWarning(ILogger logger, string drive, int percent); // allocation-free logging

[ObservableProperty] private string hostName = "";               // CommunityToolkit.Mvvm writes the property + PropertyChanged
```

Modern .NET libraries increasingly replace reflection with generators. Knowing reflection still matters: it's how you understand (and debug) what the generators and frameworks do.

@@quiz
How does a test runner find methods marked `[Fact]`?
* It uses reflection to scan types and methods for the attribute
- The compiler sends it a list
- By method name conventions only
- It runs every method
? Attributes are metadata; reflection reads them.

@@quiz
Why does heavy reflection conflict with Native AOT and trimming?
- AOT doesn't support attributes
* The trimmer removes code it can't see being used, and lookups by name at run time are invisible to it
- Reflection needs the internet
- AOT is only for Linux
? Source generators make the uses visible at compile time.

@@quiz
What does `nameof(Server.Cores)` produce?
- The property's value
* The string "Cores", checked at compile time
- The type name
- A PropertyInfo
? Refactoring-safe names.

@@flashcards
`typeof(T)` :: Type at compile time
`obj.GetType()` :: runtime type
`GetProperties()` :: PropertyInfo list
`Activator.CreateInstance` :: create an object from a Type
`Attribute` :: base class for attributes
`[AttributeUsage]` :: where an attribute may go
`GetCustomAttribute<T>()` :: read an attribute
`nameof(x)` :: compile-time name string
`[CallerMemberName]` :: compiler fills in caller's name
Source generator :: compiler plugin that writes code

@@exercise Validation attributes
Write a tiny validation library, similar to DataAnnotations:

- attributes `[NotEmpty]`, `[Between(min, max)]` (for ints) and `[Pattern(regex, message)]` (for strings), all deriving from an abstract `RuleAttribute` with `string? Check(object? value)` returning an error or null
- `Validate(object obj)` loops over the object's public properties with reflection, runs every `RuleAttribute` on each property and returns `Property: error` lines in declaration order
- `NotEmpty` → `is required`; `Between` → `must be between min and max`; `Pattern` → its message
@@starter
using System.Reflection;
using System.Text.RegularExpressions;

var good = new NewServer { Name = "web01", Cores = 8, Ip = "10.0.0.5" };
var bad = new NewServer { Name = "", Cores = 300, Ip = "10.0.0" };
Console.WriteLine("TODO");

class NewServer
{
    public string Name { get; set; } = "";
    public int Cores { get; set; }
    public string Ip { get; set; } = "";
}
@@expected
good: valid
bad:
  Name: is required
  Cores: must be between 1 and 256
  Ip: must be an IPv4 address
@@requires
GetCustomAttributes
GetProperties
@@hint
`abstract class RuleAttribute : Attribute { public abstract string? Check(object? value); }` with `[AttributeUsage(AttributeTargets.Property, AllowMultiple = true)]`. In `Validate`: `foreach (var p in obj.GetType().GetProperties()) foreach (var rule in p.GetCustomAttributes<RuleAttribute>()) if (rule.Check(p.GetValue(obj)) is string err) errors.Add($"{p.Name}: {err}");`. Decorate: Name `[NotEmpty]`, Cores `[Between(1, 256)]`, Ip `[NotEmpty, Pattern(@"^\d{1,3}(\.\d{1,3}){3}$", "must be an IPv4 address")]`.
@@solution
using System.Reflection;
using System.Text.RegularExpressions;

var good = new NewServer { Name = "web01", Cores = 8, Ip = "10.0.0.5" };
var bad = new NewServer { Name = "", Cores = 300, Ip = "10.0.0" };
foreach (var (label, obj) in new[] { ("good", good), ("bad", bad) })
{
    var errors = Validate(obj);
    if (errors.Count == 0)
    {
        Console.WriteLine($"{label}: valid");
        continue;
    }
    Console.WriteLine($"{label}:");
    foreach (var e in errors) Console.WriteLine($"  {e}");
}

List<string> Validate(object obj)
{
    var errors = new List<string>();
    foreach (var p in obj.GetType().GetProperties())
        foreach (var rule in p.GetCustomAttributes<RuleAttribute>())
            if (rule.Check(p.GetValue(obj)) is string err)
                errors.Add($"{p.Name}: {err}");
    return errors;
}

[AttributeUsage(AttributeTargets.Property, AllowMultiple = true)]
abstract class RuleAttribute : Attribute
{
    public abstract string? Check(object? value);
}

class NotEmptyAttribute : RuleAttribute
{
    public override string? Check(object? value) => value is string s && s.Trim() != "" ? null : "is required";
}

class BetweenAttribute(int min, int max) : RuleAttribute
{
    public override string? Check(object? value) =>
        value is int n && n >= min && n <= max ? null : $"must be between {min} and {max}";
}

class PatternAttribute(string pattern, string message) : RuleAttribute
{
    public override string? Check(object? value) =>
        value is string s && (s == "" || Regex.IsMatch(s, pattern)) ? null : message;
}

class NewServer
{
    [NotEmpty] public string Name { get; set; } = "";
    [Between(1, 256)] public int Cores { get; set; }
    [NotEmpty, Pattern(@"^\d{1,3}(\.\d{1,3}){3}$", "must be an IPv4 address")] public string Ip { get; set; } = "";
}

@@exercise A CSV mapper with column attributes
Write `Read<T>(string csv) where T : new()` that maps CSV rows (first line is a header) onto objects using reflection:

- a property is filled from the column named by its `[Column("name")]` attribute, or by its own name (case-insensitive) without one
- values are converted with `Convert.ChangeType(text, property.PropertyType, CultureInfo.InvariantCulture)`, except `bool` columns which accept `yes`/`no`
- properties with `[Ignore]` are skipped; columns with no matching property are ignored
- a conversion failure throws `FormatException` with `row N, column 'col': cannot convert 'text' to Int32`
@@starter
using System.Globalization;
using System.Reflection;

string csv = """
    hostname,cpu_cores,ram_gb,Os,virtual,notes
    web01,8,16.5,Ubuntu 24.04,yes,front
    db01,32,256,Windows Server 2025,no,primary
    """;
Console.WriteLine("TODO");

class Machine
{
    public string Hostname { get; set; } = "";
    public int Cores { get; set; }
    public double RamGb { get; set; }
    public string Os { get; set; } = "";
    public bool Virtual { get; set; }
    public string Location { get; set; } = "unknown";
}
@@expected
web01 8 cores 16.5 GB Ubuntu 24.04 virtual=True location=unknown
db01 32 cores 256 GB Windows Server 2025 virtual=False location=unknown
error: row 2, column 'cpu_cores': cannot convert 'many' to Int32
@@requires
GetCustomAttribute
Convert.ChangeType
@@hint
Define `ColumnAttribute(string name)` and `IgnoreAttribute`. Mark `Cores` `[Column("cpu_cores")]`, `RamGb` `[Column("ram_gb")]` and `Location` `[Ignore]`. Build a map from header column index to `PropertyInfo` once, then for each data line create `new T()` and set values. Rows are numbered from 1 after the header. For the error case, read `csv.Replace(",32,", ",many,")`.
@@solution
using System.Globalization;
using System.Reflection;

string csv = """
    hostname,cpu_cores,ram_gb,Os,virtual,notes
    web01,8,16.5,Ubuntu 24.04,yes,front
    db01,32,256,Windows Server 2025,no,primary
    """;
foreach (var m in Read<Machine>(csv))
    Console.WriteLine($"{m.Hostname} {m.Cores} cores {m.RamGb} GB {m.Os} virtual={m.Virtual} location={m.Location}");
try
{
    Read<Machine>(csv.Replace(",32,", ",many,"));
}
catch (FormatException e)
{
    Console.WriteLine($"error: {e.Message}");
}

List<T> Read<T>(string text) where T : new()
{
    var lines = text.Split('\n', StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries);
    var header = lines[0].Split(',');
    var props = typeof(T).GetProperties().Where(p => p.GetCustomAttribute<IgnoreAttribute>() is null).ToList();
    var map = new Dictionary<int, PropertyInfo>();
    for (int i = 0; i < header.Length; i++)
    {
        var match = props.FirstOrDefault(p =>
            string.Equals(p.GetCustomAttribute<ColumnAttribute>()?.Name ?? p.Name, header[i], StringComparison.OrdinalIgnoreCase));
        if (match is not null) map[i] = match;
    }
    var result = new List<T>();
    for (int row = 1; row < lines.Length; row++)
    {
        var cells = lines[row].Split(',');
        var obj = new T();
        foreach (var (col, prop) in map)
        {
            string raw = cells[col];
            try
            {
                object value = prop.PropertyType == typeof(bool)
                    ? raw switch { "yes" => true, "no" => false, _ => throw new FormatException() }
                    : Convert.ChangeType(raw, prop.PropertyType, CultureInfo.InvariantCulture);
                prop.SetValue(obj, value);
            }
            catch (FormatException)
            {
                throw new FormatException($"row {row}, column '{header[col]}': cannot convert '{raw}' to {prop.PropertyType.Name}");
            }
        }
        result.Add(obj);
    }
    return result;
}

[AttributeUsage(AttributeTargets.Property)]
class ColumnAttribute(string name) : Attribute { public string Name { get; } = name; }

[AttributeUsage(AttributeTargets.Property)]
class IgnoreAttribute : Attribute { }

class Machine
{
    public string Hostname { get; set; } = "";
    [Column("cpu_cores")] public int Cores { get; set; }
    [Column("ram_gb")] public double RamGb { get; set; }
    public string Os { get; set; } = "";
    public bool Virtual { get; set; }
    [Ignore] public string Location { get; set; } = "unknown";
}

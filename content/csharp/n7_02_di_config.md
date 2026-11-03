@@lesson n7-di-config Dependency Injection, Configuration and Logging in Depth
@@concepts modules, json
@@text
Every modern .NET app (ASP.NET Core, worker services, MAUI) starts from the **generic host**, which wires up three things: a DI container, layered configuration and logging.

```csharp,norun
var builder = Host.CreateApplicationBuilder(args);   // or WebApplication.CreateBuilder(args)

builder.Services.AddSingleton<IClock, SystemClock>();           // one for the whole app
builder.Services.AddScoped<IInventoryRepository, SqlInventory>(); // one per request/scope
builder.Services.AddTransient<IReportBuilder, CsvReportBuilder>(); // new every time
builder.Services.AddHttpClient<IpamClient>(c => c.BaseAddress = new Uri("https://ipam.local"));
builder.Services.AddOptions<SmtpOptions>()
    .Bind(builder.Configuration.GetSection("Smtp"))
    .ValidateDataAnnotations()
    .ValidateOnStart();                                          // fail at startup, not at 3 a.m.
builder.Services.AddHostedService<NightlyReportWorker>();

builder.Build().Run();
```

## Lifetimes and the captive dependency bug

| Lifetime | Instance | Typical use |
|---|---|---|
| Singleton | one per app | caches, clients, configuration |
| Scoped | one per scope (an HTTP request) | `DbContext`, unit of work |
| Transient | new each resolve | lightweight stateless services |

A singleton that takes a scoped service in its constructor **captures** one instance forever (one `DbContext` shared by every request: a concurrency bug). The container's scope validation catches this in Development.

## Constructor injection

The container reads constructor parameters and supplies registered services, recursively. Classes declare what they need and never `new` up their dependencies, so tests pass fakes. Avoid the **service locator** anti-pattern (asking the container for things inside methods).

## Configuration layers

Sources are layered; **later sources win**:

1. `appsettings.json`
2. `appsettings.{Environment}.json` (e.g. `Production`)
3. user secrets (Development only, kept out of git)
4. environment variables (`Smtp__Port=2525`: double underscore = section separator)
5. command-line arguments (`--Smtp:Port 2525`)

Keys are flattened paths, case-insensitive: `Smtp:Host`, `Smtp:Port`, `Servers:0:Name` for arrays. The **options pattern** binds a section to a class (`IOptions<SmtpOptions>`, or `IOptionsMonitor<T>` to pick up file changes). Secrets belong in a vault (Azure Key Vault, AWS Secrets Manager) or environment variables, never in `appsettings.json` in git.

## Logging

```csharp,norun
public class NightlyReportWorker(ILogger<NightlyReportWorker> log, IReportBuilder reports) : BackgroundService
{
    protected override async Task ExecuteAsync(CancellationToken ct)
    {
        log.LogInformation("Report for {Date} with {Count} hosts", DateOnly.FromDateTime(DateTime.Today), 42);
        // message templates: {Date} and {Count} become structured fields, not just text
    }
}
```

Use message templates, not string interpolation, so log stores (Seq, Elastic, Application Insights, Loki) can query by field. Levels: Trace, Debug, Information, Warning, Error, Critical, set per category in configuration (`Logging:LogLevel:Microsoft.AspNetCore = Warning`). Serilog and OpenTelemetry plug into the same `ILogger` abstraction.

@@quiz
What's the "captive dependency" problem?
* A singleton holding a scoped service, which then lives (and is shared) forever
- A service with too many dependencies
- A circular dependency
- A transient that is never disposed
? Classic with DbContext.

@@quiz
With `appsettings.json` and an environment variable `Smtp__Port=2525`, which port wins?
- The JSON value
* 2525: environment variables are a later source
- Neither, it's an error
- Whichever is larger
? Later sources override earlier ones.

@@quiz
Why use `log.LogInformation("Disk {Drive} at {Pct}%", d, p)` instead of `$"Disk {d} at {p}%"`?
- It's shorter
* The values are stored as structured fields that log tools can query
- Interpolation doesn't compile
- It's encrypted
? Message templates keep the structure.

@@flashcards
Singleton :: one instance per app
Scoped :: one per request/scope
Transient :: new each time
Constructor injection :: dependencies as constructor parameters
Captive dependency :: singleton holding a scoped service
`Section__Key` :: environment variable for Section:Key
Options pattern :: bind config sections to classes
`ValidateOnStart` :: fail fast on bad config
`ILogger<T>` :: logger with category T
Message template :: structured log message

@@exercise A mini DI container
Write `Container` with constructor injection via reflection:

- `AddSingleton<TService, TImpl>()`, `AddTransient<TService, TImpl>()`, `AddScoped<TService, TImpl>()`
- `CreateScope()` returns a `Scope` with `Resolve<T>()`: singletons are shared by every scope, scoped instances per scope, transients always new
- to create an implementation, pick its constructor with the **most** parameters and resolve each parameter recursively
- an unregistered service throws `InvalidOperationException("no registration for <Name>")`; a cycle throws `cycle: A -> B -> A` (track the types being created)

`main` (provided via the hint) shows instance ids to prove the lifetimes.
@@starter
using System.Reflection;

Console.WriteLine("TODO");

interface IClock { }
interface IRepo { int Id { get; } }
interface IReport { string Describe(); }
interface IA { }
interface IB { }

class Clock : IClock { }

class Repo(IClock clock) : IRepo
{
    private static int next;
    public int Id { get; } = ++next;
}

class Report(IRepo repo, IClock clock) : IReport
{
    private static int next;
    private readonly int id = ++next;
    public string Describe() => $"report#{id} repo#{repo.Id}";
}

class A(IB b) : IA { }
class B(IA a) : IB { }
@@expected
scope 1: report#1 repo#1, report#2 repo#1
scope 2: report#3 repo#2
same clock: True
error: no registration for IDisposable
error: cycle: A -> B -> A
@@hint
Store registrations as `Dictionary<Type, (Type Impl, string Lifetime)>` and singletons in the container's dictionary; each `Scope` has its own scoped dictionary. Create with `impl.GetConstructors().OrderByDescending(c => c.GetParameters().Length).First()` and `ctor.Invoke(args)`. Track a `Stack<Type>` (or list) of implementations in progress; if the impl is already in it, throw with the names joined by ` -> `. Registrations: `IClock`→`Clock` singleton, `IRepo`→`Repo` scoped, `IReport`→`Report` transient, `IA`→`A`, `IB`→`B` transient. In `main`, resolve `IReport` twice in scope 1 and once in scope 2, compare `IClock` across scopes with `ReferenceEquals`, then resolve `IDisposable` and `IA` inside try/catch.
@@solution
using System.Reflection;

var c = new Container();
c.AddSingleton<IClock, Clock>();
c.AddScoped<IRepo, Repo>();
c.AddTransient<IReport, Report>();
c.AddTransient<IA, A>();
c.AddTransient<IB, B>();

var s1 = c.CreateScope();
Console.WriteLine($"scope 1: {s1.Resolve<IReport>().Describe()}, {s1.Resolve<IReport>().Describe()}");
var s2 = c.CreateScope();
Console.WriteLine($"scope 2: {s2.Resolve<IReport>().Describe()}");
Console.WriteLine($"same clock: {ReferenceEquals(s1.Resolve<IClock>(), s2.Resolve<IClock>())}");
foreach (var attempt in new Action[] { () => s1.Resolve<IDisposable>(), () => s1.Resolve<IA>() })
{
    try { attempt(); }
    catch (InvalidOperationException e) { Console.WriteLine($"error: {e.Message}"); }
}

class Container
{
    internal readonly Dictionary<Type, (Type Impl, string Lifetime)> Registrations = new();
    internal readonly Dictionary<Type, object> Singletons = new();

    public void AddSingleton<TService, TImpl>() where TImpl : TService => Registrations[typeof(TService)] = (typeof(TImpl), "singleton");
    public void AddScoped<TService, TImpl>() where TImpl : TService => Registrations[typeof(TService)] = (typeof(TImpl), "scoped");
    public void AddTransient<TService, TImpl>() where TImpl : TService => Registrations[typeof(TService)] = (typeof(TImpl), "transient");
    public Scope CreateScope() => new(this);
}

class Scope(Container root)
{
    private readonly Dictionary<Type, object> scoped = new();
    private readonly List<Type> building = new();

    public T Resolve<T>() => (T)Resolve(typeof(T));

    private object Resolve(Type service)
    {
        if (!root.Registrations.TryGetValue(service, out var reg))
            throw new InvalidOperationException($"no registration for {service.Name}");
        var cache = reg.Lifetime switch { "singleton" => root.Singletons, "scoped" => scoped, _ => null };
        if (cache is not null && cache.TryGetValue(service, out var existing)) return existing;
        var instance = Create(reg.Impl);
        cache?.Add(service, instance);
        return instance;
    }

    private object Create(Type impl)
    {
        if (building.Contains(impl))
            throw new InvalidOperationException($"cycle: {string.Join(" -> ", building.Append(impl).Select(t => t.Name))}");
        building.Add(impl);
        try
        {
            var ctor = impl.GetConstructors().OrderByDescending(c => c.GetParameters().Length).First();
            var args = ctor.GetParameters().Select(p => Resolve(p.ParameterType)).ToArray();
            return ctor.Invoke(args);
        }
        finally
        {
            building.Remove(impl);
        }
    }
}

interface IClock { }
interface IRepo { int Id { get; } }
interface IReport { string Describe(); }
interface IA { }
interface IB { }

class Clock : IClock { }

class Repo(IClock clock) : IRepo
{
    private static int next;
    public int Id { get; } = ++next;
}

class Report(IRepo repo, IClock clock) : IReport
{
    private static int next;
    private readonly int id = ++next;
    public string Describe() => $"report#{id} repo#{repo.Id}";
}

class A(IB b) : IA { }
class B(IA a) : IB { }

@@exercise Layered configuration
Implement the configuration layering of .NET with `System.Text.Json`:

- `Flatten(string json)`: turn a JSON document into keys like `Smtp:Host`, `Servers:0:Name` (arrays by index), using a case-insensitive dictionary
- `Build(params IDictionary<string, string>[] layers)`: later layers win
- `FromEnvironment(IDictionary<string, string> env, string prefix)`: keep variables starting with the prefix (e.g. `APP_`), strip it, and turn `__` into `:`
- `Bind`: create `SmtpOptions { Host, Port, UseTls }` from the `Smtp` section (case-insensitive), then `Validate` it: host required, port 1–65535

Print the final keys sorted and the bound options.
@@starter
using System.Text.Json;

string baseJson = """
    {
      "Smtp": { "Host": "localhost", "Port": 25, "UseTls": false },
      "Servers": [ { "Name": "web01" }, { "Name": "db01" } ]
    }
    """;
string prodJson = """
    { "Smtp": { "Host": "smtp.example.jp", "UseTls": true } }
    """;
var env = new Dictionary<string, string> { ["APP_SMTP__PORT"] = "587", ["APP_SERVERS__1__NAME"] = "db02", ["PATH"] = "/usr/bin" };
Console.WriteLine("TODO");

class SmtpOptions
{
    public string Host { get; set; } = "";
    public int Port { get; set; }
    public bool UseTls { get; set; }
}
@@expected
Servers:0:Name = web01
SERVERS:1:NAME = db02
Smtp:Host = smtp.example.jp
SMTP:PORT = 587
Smtp:UseTls = true
smtp: smtp.example.jp:587 tls=True valid=True
broken: Port must be 1-65535
@@requires
JsonDocument
@@hint
Recursive `Walk(JsonElement e, string prefix, Dictionary<string, string> into)`: objects → recurse with `prefix + ":" + name`, arrays → index, values → `e.ToString()` for strings and `GetRawText()` otherwise (`true`, `25`). Note on spelling: assigning `dict[key] = value` in a case-insensitive dictionary keeps the key's **old** spelling. The expected output shows the overriding layer's spelling (`SMTP:PORT`), so `Build` removes the key before adding it again: `merged.Remove(k); merged[k] = v;`. `Validate` returns a list of errors; the broken case binds a copy with `Smtp:Port` set to `0`.
@@solution
using System.Text.Json;

string baseJson = """
    {
      "Smtp": { "Host": "localhost", "Port": 25, "UseTls": false },
      "Servers": [ { "Name": "web01" }, { "Name": "db01" } ]
    }
    """;
string prodJson = """
    { "Smtp": { "Host": "smtp.example.jp", "UseTls": true } }
    """;
var env = new Dictionary<string, string> { ["APP_SMTP__PORT"] = "587", ["APP_SERVERS__1__NAME"] = "db02", ["PATH"] = "/usr/bin" };

var config = Build(Flatten(baseJson), Flatten(prodJson), FromEnvironment(env, "APP_"));
foreach (var (k, v) in config.OrderBy(kv => kv.Key, StringComparer.OrdinalIgnoreCase))
    Console.WriteLine($"{k} = {v}");

var smtp = Bind(config);
Console.WriteLine($"smtp: {smtp.Host}:{smtp.Port} tls={smtp.UseTls} valid={Validate(smtp).Count == 0}");

var broken = Build(config, new Dictionary<string, string> { ["Smtp:Port"] = "0" });
Console.WriteLine($"broken: {string.Join("; ", Validate(Bind(broken)))}");

Dictionary<string, string> Flatten(string json)
{
    var into = new Dictionary<string, string>(StringComparer.OrdinalIgnoreCase);
    using var doc = JsonDocument.Parse(json);
    Walk(doc.RootElement, "", into);
    return into;
}

void Walk(JsonElement e, string prefix, Dictionary<string, string> into)
{
    string Join(string key) => prefix == "" ? key : $"{prefix}:{key}";
    switch (e.ValueKind)
    {
        case JsonValueKind.Object:
            foreach (var p in e.EnumerateObject()) Walk(p.Value, Join(p.Name), into);
            break;
        case JsonValueKind.Array:
            int i = 0;
            foreach (var item in e.EnumerateArray()) Walk(item, Join((i++).ToString()), into);
            break;
        case JsonValueKind.String:
            into[prefix] = e.GetString()!;
            break;
        default:
            into[prefix] = e.GetRawText();
            break;
    }
}

Dictionary<string, string> Build(params IDictionary<string, string>[] layers)
{
    var merged = new Dictionary<string, string>(StringComparer.OrdinalIgnoreCase);
    foreach (var layer in layers)
        foreach (var (k, v) in layer)
        {
            merged.Remove(k);
            merged[k] = v;
        }
    return merged;
}

Dictionary<string, string> FromEnvironment(IDictionary<string, string> vars, string prefix) =>
    vars.Where(kv => kv.Key.StartsWith(prefix, StringComparison.OrdinalIgnoreCase))
        .ToDictionary(kv => kv.Key[prefix.Length..].Replace("__", ":"), kv => kv.Value, StringComparer.OrdinalIgnoreCase);

SmtpOptions Bind(IDictionary<string, string> cfg) => new()
{
    Host = cfg.TryGetValue("Smtp:Host", out var h) ? h : "",
    Port = cfg.TryGetValue("Smtp:Port", out var p) && int.TryParse(p, out var n) ? n : 0,
    UseTls = cfg.TryGetValue("Smtp:UseTls", out var t) && bool.TryParse(t, out var b) && b,
};

List<string> Validate(SmtpOptions o)
{
    var errors = new List<string>();
    if (string.IsNullOrWhiteSpace(o.Host)) errors.Add("Host is required");
    if (o.Port is < 1 or > 65535) errors.Add("Port must be 1-65535");
    return errors;
}

class SmtpOptions
{
    public string Host { get; set; } = "";
    public int Port { get; set; }
    public bool UseTls { get; set; }
}

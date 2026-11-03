@@lesson e6-oop Object-Oriented C# in Depth: Inheritance, Abstraction and Composition
@@concepts objects
@@text
## Inheritance, virtual and override

```csharp
Check[] checks = [new DiskCheck(85), new PingCheck("db01")];
foreach (var c in checks)
    Console.WriteLine(c.Describe());

abstract class Check(string name)
{
    public string Name { get; } = name;
    public abstract bool Passes();                       // subclasses MUST implement
    public virtual string Describe() =>                   // subclasses MAY override
        $"{Name}: {(Passes() ? "ok" : "FAIL")}";
}

class DiskCheck(int usedPercent) : Check("disk")
{
    public override bool Passes() => usedPercent < 90;
}

sealed class PingCheck(string host) : Check($"ping {host}")
{
    public override bool Passes() => host != "db01";
    public override string Describe() => base.Describe() + " (3 attempts)"; // extend the base version
}
```

- `abstract` class: can't be instantiated; may contain `abstract` members without a body.
- `virtual`: overridable; `override`: replaces it; `base.X()`: call the parent's version.
- `sealed`: no further inheritance (also lets the JIT optimize calls).
- A method without `virtual` is **not** overridable; `new` hides it instead, which is almost always a mistake.

## Encapsulation: control how state changes

```csharp
var t = new Ticket { Title = "VPN down" };
t.Assign("ana");
Console.WriteLine($"{t.Title} -> {t.Assignee} ({t.Status})");

class Ticket
{
    public required string Title { get; init; }     // must be set at creation, then read-only
    public string? Assignee { get; private set; }   // only the class changes it
    public string Status { get; private set; } = "New";

    public void Assign(string who)
    {
        if (Status == "Closed") throw new InvalidOperationException("ticket is closed");
        Assignee = who;
        Status = "Assigned";
    }
}
```

Expose **behaviour** (`Assign`, `Close`) rather than settable properties, and the rules can't be bypassed.

## Interfaces: default methods and static members

```csharp,norun
interface INotifier
{
    Task SendAsync(string message);
    Task SendManyAsync(IEnumerable<string> ms) => Task.WhenAll(ms.Select(SendAsync)); // default implementation
}

interface IParsable<TSelf> where TSelf : IParsable<TSelf>
{
    static abstract TSelf Parse(string s);   // static abstract (C# 11): T.Parse(...) in generic code
}
```

A class has **one** base class but any number of interfaces. Interfaces describe *capabilities* (`IComparable`, `IDisposable`, `INotifier`); base classes share *implementation*.

## Composition over inheritance

Deep hierarchies (`Device → NetworkDevice → Router → CiscoRouter → ...`) get rigid. Prefer small interfaces and objects that **wrap** other objects:

- **Decorator**: same interface, adds behaviour (caching, logging, retries) around an inner object.
- **Strategy**: pass the varying behaviour in as an object or delegate.
- **Liskov substitution**: a subclass must work anywhere its base type is expected. If `ReadOnlyFile : File` throws on `Write`, the hierarchy is wrong.

## Records and inheritance

Records support inheritance and non-destructive mutation with `with`:

```csharp
Device d = new Server("web01", "10.0.0.5", Cores: 8);
var bigger = (Server)d with { Cores = 16 };
Console.WriteLine(d);
Console.WriteLine(bigger);

record Device(string Name, string Ip);
record Server(string Name, string Ip, int Cores) : Device(Name, Ip);
```

@@quiz
What does `abstract` on a method mean?
* It has no body here; every non-abstract subclass must override it
- It can't be called
- It's private
- It's static
? The class must then be abstract too.

@@quiz
Why expose `Assign(who)` instead of a public `Assignee` setter?
- It's shorter
* The method can enforce rules (like "closed tickets can't be assigned") that a setter would let callers bypass
- Setters are slow
- Properties can't be strings
? Encapsulation protects invariants.

@@quiz
What's the decorator pattern?
- A class attribute
* An object that implements an interface by wrapping another implementation and adding behaviour
- A base class with many virtual methods
- A UI styling technique
? `new CachingStorage(new LoggingStorage(new FileStorage()))`.

@@flashcards
`abstract` :: must be overridden; class can't be instantiated
`virtual` / `override` :: overridable / overriding member
`base.Method()` :: call the parent implementation
`sealed` :: no further inheritance
`required` :: must be set in the object initializer
`init` :: settable only during creation
`private set` :: only the class can change it
Default interface method :: interface member with a body
Decorator :: wrapper adding behaviour
`with` :: copy a record with changes

@@exercise A template method for health checks
Build a small check framework:

- `abstract class HealthCheck` with `Name`, an abstract `protected CheckResult Run()`, and a **non-virtual** `Evaluate()` that times nothing but wraps `Run()`: it catches any exception and returns `Error: <message>` instead of crashing
- `record CheckResult(string Status, string Detail)` with `ToString()` → `Status (Detail)`, or just `Status` when there's no detail
- `ThresholdCheck(name, value, warn, crit)`: `OK`, `WARN` (≥ warn) or `CRIT` (≥ crit) with detail `value/crit`
- `sealed class ServiceCheck(name, IReadOnlyDictionary<string, string> states)`: `OK` if the service is `running`, `CRIT` with detail = the state, and a `KeyNotFoundException` if the service is unknown (which `Evaluate` turns into an error)
- `Summary(IEnumerable<HealthCheck>)` prints each `name: result`, then the worst status (`CRIT` > `WARN` > `OK`; errors count as `CRIT`)
@@starter
var states = new Dictionary<string, string> { ["nginx"] = "running", ["postgres"] = "stopped" };
Console.WriteLine("TODO");

record CheckResult(string Status, string Detail);
@@expected
cpu: OK (45/95)
disk /: WARN (88/95)
memory: CRIT (97/95)
nginx: OK
postgres: CRIT (stopped)
redis: Error (The given key 'redis' was not present in the dictionary.)
overall: CRIT
@@hint
`public CheckResult Evaluate() { try { return Run(); } catch (Exception e) { return new CheckResult("Error", e.Message); } }`. Override `ToString` in the record: `public override string ToString() => Detail == "" ? Status : $"{Status} ({Detail})";`. Checks: `cpu` 45, `disk /` 88, `memory` 97, all with warn 80 and crit 95; services nginx, postgres, redis. For the overall status, map each status to a rank.
@@solution
var states = new Dictionary<string, string> { ["nginx"] = "running", ["postgres"] = "stopped" };
HealthCheck[] checks =
[
    new ThresholdCheck("cpu", 45, 80, 95),
    new ThresholdCheck("disk /", 88, 80, 95),
    new ThresholdCheck("memory", 97, 80, 95),
    new ServiceCheck("nginx", states),
    new ServiceCheck("postgres", states),
    new ServiceCheck("redis", states),
];
Summary(checks);

void Summary(IEnumerable<HealthCheck> all)
{
    int worst = 0;
    string[] names = ["OK", "WARN", "CRIT"];
    foreach (var c in all)
    {
        var r = c.Evaluate();
        Console.WriteLine($"{c.Name}: {r}");
        int rank = r.Status switch { "OK" => 0, "WARN" => 1, _ => 2 };
        worst = Math.Max(worst, rank);
    }
    Console.WriteLine($"overall: {names[worst]}");
}

record CheckResult(string Status, string Detail)
{
    public override string ToString() => Detail == "" ? Status : $"{Status} ({Detail})";
}

abstract class HealthCheck(string name)
{
    public string Name { get; } = name;

    protected abstract CheckResult Run();

    public CheckResult Evaluate()
    {
        try
        {
            return Run();
        }
        catch (Exception e)
        {
            return new CheckResult("Error", e.Message);
        }
    }
}

class ThresholdCheck(string name, int value, int warn, int crit) : HealthCheck(name)
{
    protected override CheckResult Run()
    {
        string status = value >= crit ? "CRIT" : value >= warn ? "WARN" : "OK";
        return new CheckResult(status, $"{value}/{crit}");
    }
}

sealed class ServiceCheck(string name, IReadOnlyDictionary<string, string> states) : HealthCheck(name)
{
    protected override CheckResult Run()
    {
        string state = states[Name];
        return state == "running" ? new CheckResult("OK", "") : new CheckResult("CRIT", state);
    }
}

@@exercise Decorators around storage
Implement `IStore` (`string? Get(string key)` and `void Set(string key, string value)`) three times:

- `MemoryStore`: a dictionary that counts its `Get` calls in `Reads` (to simulate a slow database)
- `CachingStore(IStore inner)`: serves repeated `Get`s from its own cache; `Set` writes through to `inner` **and** updates the cache
- `LoggingStore(IStore inner, List<string> log)`: records `GET key -> value` (`null` when missing) and `SET key` lines, then delegates

Compose them as `new LoggingStore(new CachingStore(memory), log)` and show that the inner store is read only once per key, while every call is logged.
@@starter
interface IStore
{
    string? Get(string key);
    void Set(string key, string value);
}

Console.WriteLine("TODO");
@@expected
GET web01 -> null
SET web01
GET web01 -> 10.0.0.5
GET web01 -> 10.0.0.5
GET db01 -> 10.0.0.9
GET db01 -> 10.0.0.9
inner reads: 2
@@hint
Top-level statements must come before type declarations, so move the interface below `main`'s code. Seed the memory store with `db01` directly (`memory.Set("db01", "10.0.0.9")`) before wrapping. Calls: Get web01, Set web01, Get web01 twice, Get db01 twice. The caching store must not cache a `null` for web01 (or `Set` must update it): either way the first web01 read happens before Set and the next ones come from the cache. Count: db01 one read; web01's first miss one read: 2.
@@solution
var memory = new MemoryStore();
memory.Set("db01", "10.0.0.9");
var log = new List<string>();
IStore store = new LoggingStore(new CachingStore(memory), log);

store.Get("web01");
store.Set("web01", "10.0.0.5");
store.Get("web01");
store.Get("web01");
store.Get("db01");
store.Get("db01");

foreach (var line in log) Console.WriteLine(line);
Console.WriteLine($"inner reads: {memory.Reads}");

interface IStore
{
    string? Get(string key);
    void Set(string key, string value);
}

class MemoryStore : IStore
{
    private readonly Dictionary<string, string> data = new();
    public int Reads { get; private set; }

    public string? Get(string key)
    {
        Reads++;
        return data.GetValueOrDefault(key);
    }

    public void Set(string key, string value) => data[key] = value;
}

class CachingStore(IStore inner) : IStore
{
    private readonly Dictionary<string, string> cache = new();

    public string? Get(string key)
    {
        if (cache.TryGetValue(key, out var hit)) return hit;
        var value = inner.Get(key);
        if (value is not null) cache[key] = value;
        return value;
    }

    public void Set(string key, string value)
    {
        inner.Set(key, value);
        cache[key] = value;
    }
}

class LoggingStore(IStore inner, List<string> log) : IStore
{
    public string? Get(string key)
    {
        var value = inner.Get(key);
        log.Add($"GET {key} -> {value ?? "null"}");
        return value;
    }

    public void Set(string key, string value)
    {
        log.Add($"SET {key}");
        inner.Set(key, value);
    }
}

@@lesson c2-classes Classes, Records and Interfaces
@@concepts objects
@@text
## Classes

A class bundles data (**fields** and **properties**) with behaviour (**methods**). Objects are made from it with `new`.

```csharp
var srv = new Server("web01", 16);
srv.Cpu = 42;
Console.WriteLine(srv);
Console.WriteLine(srv.IsHealthy());

class Server
{
    // Properties: { get; set; } can be read and changed; { get; } only read.
    public string Name { get; }
    public int RamGb { get; }
    public int Cpu { get; set; }

    // The constructor runs when you write `new Server(...)`.
    public Server(string name, int ramGb)
    {
        Name = name;
        RamGb = ramGb;
    }

    public bool IsHealthy() => Cpu < 90;

    public override string ToString() => $"{Name} ({RamGb} GB, cpu {Cpu}%)";
}
```

`public` means code outside the class can use it; `private` (the default) hides it. Keep fields private and expose what's needed: that's **encapsulation**.

**Primary constructors** (C# 12) shorten this: `class Server(string name, int ramGb) { public string Name => name; }`.

## Validation in properties

A property can run code when it's set, so an object can never hold nonsense:

```csharp
var t = new Ticket { Title = "VPN down" };
try { t.Priority = 9; }
catch (ArgumentOutOfRangeException e) { Console.WriteLine(e.Message); }
Console.WriteLine($"{t.Title} P{t.Priority}");

class Ticket
{
    private int priority = 3;
    public required string Title { get; init; }   // must be set at creation, then fixed
    public int Priority
    {
        get => priority;
        set => priority = value is >= 1 and <= 4 ? value : throw new ArgumentOutOfRangeException(nameof(Priority), "must be 1-4");
    }
}
```

`init` means "settable only while the object is created". `required` makes the compiler insist you set it.

## Records: data with value equality

A **record** is a class meant for holding data. One line gives you properties, a constructor, `ToString`, and equality by value:

```csharp
var a = new Endpoint("db01", 5432);
var b = new Endpoint("db01", 5432);
Console.WriteLine(a);              // Endpoint { Host = db01, Port = 5432 }
Console.WriteLine(a == b);         // True: same values
var c = a with { Port = 6432 };    // copy with one change
Console.WriteLine(c);

record Endpoint(string Host, int Port);
```

Two ordinary class objects are only `==` if they are the very same object. Use records for DTOs, config, messages, results: anything that's "just data".

## Interfaces and polymorphism

An **interface** is a promise: "anything that implements me has these members". Code can then work with any of them.

```csharp
ICheck[] checks = { new PingCheck("10.0.0.1"), new DiskCheck("C:", 91) };
foreach (var check in checks)
    Console.WriteLine($"{check.Name}: {(check.Run() ? "PASS" : "FAIL")}");

interface ICheck
{
    string Name { get; }
    bool Run();
}

class PingCheck(string host) : ICheck
{
    public string Name => $"ping {host}";
    public bool Run() => true;                // pretend it answered
}

class DiskCheck(string drive, int usedPercent) : ICheck
{
    public string Name => $"disk {drive}";
    public bool Run() => usedPercent < 90;
}
```

Adding a new kind of check doesn't touch the loop. That's the point.

**Inheritance** (`class Admin : User`) lets a class extend another, with `virtual`/`override` methods. Prefer interfaces and small classes; deep inheritance trees get hard to change.

## Enums

```csharp
var s = Status.Degraded;
Console.WriteLine(s);              // Degraded
Console.WriteLine((int)s);         // 1
enum Status { Up, Degraded, Down }
```

@@quiz
Two `record Endpoint(string Host, int Port)` objects with the same values. Is `a == b`?
* `true`: records compare by value
- `false`: they are different objects
- It doesn't compile
- Only if you override `Equals`
? Records get value equality automatically. Ordinary classes compare by reference.

@@quiz
What does `{ get; init; }` mean for a property?
* It can be set only while the object is being created
- It can never be set
- It can be set any time
- It's set automatically to a default
? `init` setters work in constructors and object initializers (`new T { X = 1 }`), not later.

@@quiz
Why use an interface like `ICheck`?
* Code can work with any type that implements it, without knowing which one
- It makes the code faster
- It's required for every class
- It stops a class from having methods
? Interfaces let you add new implementations without changing the code that uses them.

@@drill
Which keyword declares a type with value-based equality and concise syntax?
= record
? record Point(int X, int Y);

@@flashcards
class :: reference type with identity
record :: data type with value equality
property { get; set; } :: a field with accessors
init :: can only be set while creating the object
interface :: a contract of members a class implements
enum :: a fixed set of named constants
constructor :: sets up a new object

@@exercise Inventory record
Complete the `Asset` record and `Report` so the program prints the assets older than 5 years in 2026, oldest first, then the count:

```text
PC-002 (2017) Kenji
PC-004 (2019) Aiko
2 to replace
```
@@starter
var assets = new List<Asset>
{
    new("PC-001", 2023, "Yui"),
    new("PC-002", 2017, "Kenji"),
    new("PC-003", 2022, "Mei"),
    new("PC-004", 2019, "Aiko"),
};
Report(assets, 2026);

void Report(List<Asset> all, int year)
{
    // print each old asset with ToLine(), then the count
}

record Asset(string Tag, int Year, string Owner)
{
    // add AgeIn(int year) and ToLine()
}
@@expected
PC-002 (2017) Kenji
PC-004 (2019) Aiko
2 to replace
@@hint
Inside the record: `public int AgeIn(int year) => year - Year;` and `public string ToLine() => $"{Tag} ({Year}) {Owner}";`. Then `all.Where(a => a.AgeIn(year) > 5).OrderBy(a => a.Year).ToList()`.
@@solution
var assets = new List<Asset>
{
    new("PC-001", 2023, "Yui"),
    new("PC-002", 2017, "Kenji"),
    new("PC-003", 2022, "Mei"),
    new("PC-004", 2019, "Aiko"),
};
Report(assets, 2026);

void Report(List<Asset> all, int year)
{
    var old = all.Where(a => a.AgeIn(year) > 5).OrderBy(a => a.Year).ToList();
    foreach (var a in old) Console.WriteLine(a.ToLine());
    Console.WriteLine($"{old.Count} to replace");
}

record Asset(string Tag, int Year, string Owner)
{
    public int AgeIn(int year) => year - Year;
    public string ToLine() => $"{Tag} ({Year}) {Owner}";
}

@@exercise Notifier interface
Write an `INotifier` interface with `string Send(string message)`, and two classes: `EmailNotifier(string to)` returning `email to <to>: <message>` and `SlackNotifier(string channel)` returning `slack #<channel>: <message>`. The given loop must then print:

```text
email to oncall@example.jp: disk full on db01
slack #ops: disk full on db01
```
@@starter
INotifier[] notifiers = { new EmailNotifier("oncall@example.jp"), new SlackNotifier("ops") };
foreach (var n in notifiers)
    Console.WriteLine(n.Send("disk full on db01"));

// declare INotifier, EmailNotifier and SlackNotifier here
@@expected
email to oncall@example.jp: disk full on db01
slack #ops: disk full on db01
@@requires
interface INotifier
@@hint
`class EmailNotifier(string to) : INotifier { public string Send(string message) => $"email to {to}: {message}"; }`
@@solution
INotifier[] notifiers = { new EmailNotifier("oncall@example.jp"), new SlackNotifier("ops") };
foreach (var n in notifiers)
    Console.WriteLine(n.Send("disk full on db01"));

interface INotifier
{
    string Send(string message);
}

class EmailNotifier(string to) : INotifier
{
    public string Send(string message) => $"email to {to}: {message}";
}

class SlackNotifier(string channel) : INotifier
{
    public string Send(string message) => $"slack #{channel}: {message}";
}

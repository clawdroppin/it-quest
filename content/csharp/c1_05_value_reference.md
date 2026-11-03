@@lesson c1-values Value Types, Reference Types, Equality and the Garbage Collector
@@concepts variables, objects
@@text
Every C# type is either a **value type** or a **reference type**, and the difference explains a whole family of bugs.

| | Value types | Reference types |
|---|---|---|
| Examples | `int`, `double`, `bool`, `DateTime`, `struct`, `enum`, `record struct` | `class`, `record`, `string`, arrays, `List<T>`, delegates |
| A variable holds | the data itself | a reference (pointer) to an object |
| Assignment `b = a` | copies the data | copies the reference: both see the same object |
| Can be `null` | only as `int?` (`Nullable<T>`) | yes |

```csharp
var p1 = new PointS(1, 2);
var p2 = p1;          // copy
p2.X = 99;
Console.WriteLine($"{p1.X} {p2.X}");   // 1 99

var c1 = new PointC { X = 1 };
var c2 = c1;          // same object
c2.X = 99;
Console.WriteLine($"{c1.X} {c2.X}");   // 99 99

var list = new List<int> { 1, 2 };
AddThree(list);
Console.WriteLine(list.Count);         // 3: the method changed the caller's list

void AddThree(List<int> l) => l.Add(3);

struct PointS { public int X, Y; public PointS(int x, int y) { X = x; Y = y; } }
class PointC { public int X; }
```

## A struct inside a list

`list[0]` on a `List<SomeStruct>` returns a **copy**, so `points[0].X = 5;` is a compile error (CS1612): you'd be changing a temporary. Read it, change it, write it back, or make the struct immutable (`readonly struct`, `record struct`), which is the recommended design.

## ref, out and in

- `ref` passes the variable itself, so the method can replace it.
- `out` is a `ref` that the method must assign (the `TryParse` pattern).
- `in` passes a large struct by reference, read-only, to avoid copying.

## Equality

```csharp
var a = new HostC("web01");
var b = new HostC("web01");
Console.WriteLine(a == b);                       // False: different objects
Console.WriteLine(new HostR("web01") == new HostR("web01")); // True: records compare values
Console.WriteLine(ReferenceEquals(a, a));        // True
Console.WriteLine("ab" + "c" == "abc");          // True: string == compares text

class HostC(string name) { public string Name { get; } = name; }
record HostR(string Name);
```

Classes compare **references** by default; records, structs and strings compare **values**. If a class must act as a value (a dictionary key, for example), override `Equals` and `GetHashCode` together, or make it a record.

## Boxing

Putting a value type into an `object` (or a non-generic interface) **boxes** it: allocates a heap copy. `ArrayList` and `object` parameters box; generic collections (`List<int>`) don't, which is one reason generics replaced them.

## The garbage collector

Objects live on the managed heap until nothing references them; then the GC reclaims them. It's **generational**: new objects (gen 0) are collected often and cheaply, survivors are promoted to gen 1 and gen 2. You rarely think about it, except:

- **Unmanaged resources** (files, sockets, database connections, OS handles) aren't memory, so the GC doesn't close them promptly. Types holding them implement `IDisposable`; release them with `using`.
- Event subscriptions and static collections keep objects alive: the usual source of .NET "memory leaks".

```csharp
using (var w = new StringWriter())
{
    w.Write("disposed at the closing brace");
    Console.WriteLine(w);
}
using var reader = new StringReader("disposed at the end of the scope");  // C# 8 using declaration
Console.WriteLine(reader.ReadLine());
```

@@quiz
After `var b = a;` where `a` is a `List<int>`, what happens when you call `b.Add(1)`?
* `a` sees the new item too: both variables reference the same list
- Only `b` changes
- It's a compile error
- `a` becomes null
? Lists are reference types.

@@quiz
Why does `points[0].X = 5;` fail to compile for a `List<PointStruct>`?
- Structs can't have fields
* The indexer returns a copy, so the assignment would change a temporary
- Lists are read-only
- X is private
? Arrays allow it (`array[0].X = 5`) because they give direct access.

@@quiz
Do two separate `class` instances with the same field values compare equal with `==`?
- Yes, always
* No, classes compare references unless equality is overridden; records compare values
- Only for strings
- Only in Release builds
? Use a record for data-like types.

@@flashcards
Value type :: variable holds the data (struct, int, enum)
Reference type :: variable holds a reference (class, array, string)
`readonly struct` :: immutable value type
`ref` :: pass the variable itself
`out` :: method must assign it
`in` :: read-only by reference
Boxing :: value type copied into an object on the heap
`ReferenceEquals` :: same object?
Generational GC :: gen 0, 1, 2
`IDisposable` / `using` :: release unmanaged resources promptly

@@exercise Aliasing bugs
The starter has three bugs caused by value/reference semantics. Fix them so the output matches:

1. `Normalize` should return a **new** sorted, lower-cased list without changing the caller's list
2. `Promote` must update a `Server` struct stored in a `List<Server>` (read, modify, write back)
3. `Defaults.Tags` is shared by every new config; `NewConfig` must give each config its **own** copy of the defaults
@@starter
var original = new List<string> { "Web02", "db01", "WEB01" };
var normalized = Normalize(original);
Console.WriteLine(string.Join(",", original));
Console.WriteLine(string.Join(",", normalized));

var servers = new List<Server> { new("web01", "staging"), new("db01", "staging") };
Promote(servers, 1);
foreach (var s in servers) Console.WriteLine($"{s.Name} {s.Env}");

var a = NewConfig();
var b = NewConfig();
a.Tags.Add("pci");
Console.WriteLine($"a: {string.Join(",", a.Tags)}");
Console.WriteLine($"b: {string.Join(",", b.Tags)}");
Console.WriteLine($"defaults: {string.Join(",", Defaults.Tags)}");

List<string> Normalize(List<string> names)
{
    for (int i = 0; i < names.Count; i++) names[i] = names[i].ToLowerInvariant();
    names.Sort(StringComparer.Ordinal);
    return names;
}

void Promote(List<Server> list, int index)
{
    var s = list[index];
    s.Env = "prod";
}

Config NewConfig() => new Config { Tags = Defaults.Tags };

struct Server(string name, string env)
{
    public string Name = name;
    public string Env = env;
}

class Config { public List<string> Tags = new(); }

static class Defaults { public static readonly List<string> Tags = ["managed", "backup"]; }
@@expected
Web02,db01,WEB01
db01,web01,web02
web01 staging
db01 prod
a: managed,backup,pci
b: managed,backup
defaults: managed,backup
@@hint
1. Start with `var copy = names.Select(n => n.ToLowerInvariant()).ToList();` and sort the copy. 2. Add `list[index] = s;` after changing `s`. 3. `new Config { Tags = new List<string>(Defaults.Tags) }` (or `[.. Defaults.Tags]`).
@@solution
var original = new List<string> { "Web02", "db01", "WEB01" };
var normalized = Normalize(original);
Console.WriteLine(string.Join(",", original));
Console.WriteLine(string.Join(",", normalized));

var servers = new List<Server> { new("web01", "staging"), new("db01", "staging") };
Promote(servers, 1);
foreach (var s in servers) Console.WriteLine($"{s.Name} {s.Env}");

var a = NewConfig();
var b = NewConfig();
a.Tags.Add("pci");
Console.WriteLine($"a: {string.Join(",", a.Tags)}");
Console.WriteLine($"b: {string.Join(",", b.Tags)}");
Console.WriteLine($"defaults: {string.Join(",", Defaults.Tags)}");

List<string> Normalize(List<string> names)
{
    var copy = names.Select(n => n.ToLowerInvariant()).ToList();
    copy.Sort(StringComparer.Ordinal);
    return copy;
}

void Promote(List<Server> list, int index)
{
    var s = list[index];
    s.Env = "prod";
    list[index] = s;
}

Config NewConfig() => new Config { Tags = new List<string>(Defaults.Tags) };

struct Server(string name, string env)
{
    public string Name = name;
    public string Env = env;
}

class Config { public List<string> Tags = new(); }

static class Defaults { public static readonly List<string> Tags = ["managed", "backup"]; }

@@exercise A disposable connection pool
Write a tiny pool of "connections" that must always be returned:

- `Pool(int size)` with `Rent()` returning a `Lease` (throws `InvalidOperationException("pool exhausted")` when all are rented) and `InUse` (count)
- `Lease : IDisposable` with an `Id` (1, 2, ... reused after return: always hand out the **lowest** free id); `Dispose()` returns it to the pool and must be **idempotent** (a second call does nothing)
- `main` (provided) uses `using` blocks and an exception to prove the lease is returned even when code throws
@@starter
var pool = new Pool(2);
using (var a = pool.Rent())
{
    Console.WriteLine($"rented {a.Id}, in use {pool.InUse}");
}
Console.WriteLine($"after using: {pool.InUse}");

class Pool(int size)
{
    public int InUse => 0;
    public Lease Rent() => new Lease(this, 1);
}

class Lease(Pool pool, int id) : IDisposable
{
    public int Id { get; } = id;
    public void Dispose() { }
}
@@expected
rented 1, in use 1
after using: 0
rented 1 and 2
error: pool exhausted
after failure: 0
rented 1 again
disposed twice, in use 0
@@hint
Keep a `SortedSet<int>` of free ids filled with 1..size; `Rent` takes `free.Min` and removes it; an internal `Return(int id)` adds it back. In `Lease`, a `bool disposed` flag guards `Dispose`. Extend `main`: rent two leases inside `try { using var x = ...; using var y = ...; Console.WriteLine(...); pool.Rent(); } catch (InvalidOperationException e) { ... }`, then show `after failure`, rent again, and dispose one lease twice.
@@solution
var pool = new Pool(2);
using (var a = pool.Rent())
{
    Console.WriteLine($"rented {a.Id}, in use {pool.InUse}");
}
Console.WriteLine($"after using: {pool.InUse}");

try
{
    using var x = pool.Rent();
    using var y = pool.Rent();
    Console.WriteLine($"rented {x.Id} and {y.Id}");
    pool.Rent();
}
catch (InvalidOperationException e)
{
    Console.WriteLine($"error: {e.Message}");
}
Console.WriteLine($"after failure: {pool.InUse}");

var z = pool.Rent();
Console.WriteLine($"rented {z.Id} again");
z.Dispose();
z.Dispose();
Console.WriteLine($"disposed twice, in use {pool.InUse}");

class Pool
{
    private readonly SortedSet<int> free = new();
    private readonly int size;

    public Pool(int size)
    {
        this.size = size;
        for (int i = 1; i <= size; i++) free.Add(i);
    }

    public int InUse => size - free.Count;

    public Lease Rent()
    {
        if (free.Count == 0) throw new InvalidOperationException("pool exhausted");
        int id = free.Min;
        free.Remove(id);
        return new Lease(this, id);
    }

    internal void Return(int id) => free.Add(id);
}

class Lease(Pool pool, int id) : IDisposable
{
    private bool disposed;
    public int Id { get; } = id;

    public void Dispose()
    {
        if (disposed) return;
        disposed = true;
        pool.Return(Id);
    }
}

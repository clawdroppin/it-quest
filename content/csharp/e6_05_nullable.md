@@lesson e6-nullable Nullable Reference Types, Guard Clauses and Result Types
@@concepts errors, conditions
@@text
`NullReferenceException` is the most common .NET crash. **Nullable reference types** (on by default in new projects, and in this course) make the compiler track which variables may be null.

## The annotations

```csharp
string name = "web01";      // never null (the compiler warns if you assign null)
string? owner = null;       // may be null
Console.WriteLine(name.Length);
// Console.WriteLine(owner.Length);   // warning CS8602: possible null dereference

if (owner is not null)
    Console.WriteLine(owner.Length);   // flow analysis: safe inside the check

Console.WriteLine(owner?.Length);      // null-conditional: null instead of a crash
Console.WriteLine(owner ?? "unassigned"); // null-coalescing: a fallback
owner ??= "ana";                       // assign only if null
Console.WriteLine(owner);
```

The checks are **compile-time warnings only**. At run time nothing changes, so data from JSON, databases or old libraries can still contain nulls where your types say there are none: validate at the boundaries.

## The null-forgiving operator

`value!` tells the compiler "trust me, this isn't null". It removes the warning but adds no check. Use it rarely, with a reason (e.g. a field set by a framework after construction).

## Guard clauses

Fail fast with a clear message at the start of public methods:

```csharp
Console.WriteLine(Ping("db01", 3));
try { Ping(null!, 3); } catch (ArgumentNullException e) { Console.WriteLine(e.Message); }
try { Ping("db01", 0); } catch (ArgumentOutOfRangeException e) { Console.WriteLine(e.ParamName); }

string Ping(string host, int count)
{
    ArgumentNullException.ThrowIfNull(host);
    ArgumentOutOfRangeException.ThrowIfNegativeOrZero(count);
    return $"pinging {host} x{count}";
}
```

## Teaching the compiler your null contracts

Attributes from `System.Diagnostics.CodeAnalysis` describe what your methods guarantee:

```csharp,norun
bool TryGetOwner(string host, [NotNullWhen(true)] out string? owner);  // owner is non-null when it returns true
[return: NotNullIfNotNull(nameof(input))] string? Normalize(string? input);
[MemberNotNull(nameof(connection))] void EnsureConnected();
```

With `[NotNullWhen(true)]`, callers can write `if (TryGetOwner(h, out var o)) Console.WriteLine(o.Length);` without warnings: the same pattern as `int.TryParse` and `Dictionary.TryGetValue`.

## Generics and null

`T?` on an unconstrained generic means "maybe default" (null for reference types, 0 for ints). `where T : notnull` forbids nullable type arguments, as `Dictionary<TKey, ...>` keys do.

## Errors as values: Result types

Exceptions are for exceptional situations. For **expected** failures (invalid user input, a missing record) many teams return a result instead:

```csharp,norun
public readonly record struct Result<T>(T? Value, string? Error)
{
    public bool IsOk => Error is null;
    public static Result<T> Ok(T value) => new(value, null);
    public static Result<T> Fail(string error) => new(default, error);
}
```

The caller can't forget the failure case, and chains of steps (parse → validate → save) compose with `Bind`, as you'll build in the exercise.

@@quiz
Does `string?` change what happens at run time?
- Yes, it adds null checks
* No, it's compile-time annotation and analysis only
- It makes strings immutable
- It converts null to ""
? Validate data coming from outside your code.

@@quiz
What does `owner ??= "ana"` do?
* Assigns "ana" only if owner is null
- Always assigns "ana"
- Compares owner with "ana"
- Throws if owner is null
? Null-coalescing assignment.

@@quiz
What does `[NotNullWhen(true)] out string? owner` tell the compiler?
- owner is always null
* When the method returns true, owner isn't null
- The method never returns true
- owner is optional
? Same contract as Dictionary.TryGetValue.

@@drill
Which operator tells the compiler a value isn't null, without checking?
= !
= null-forgiving operator
? name!.Length. Use sparingly.

@@flashcards
`string?` :: may be null
`?.` :: call only if not null
`??` :: fallback for null
`??=` :: assign if null
`!` :: null-forgiving (no check!)
`ArgumentNullException.ThrowIfNull` :: guard clause
`[NotNullWhen(true)]` :: non-null when the method returns true
`where T : notnull` :: no nullable type arguments
Result type :: error as a return value

@@exercise Null-safe lookups
An inventory API returns partial data. Write, with no compiler warnings:

- `TryGetOwner(string host, [NotNullWhen(true)] out string? owner)`: true when the host exists **and** has a non-empty owner
- `Contact(string host)`: `owner <email>` using the owner's email from a second dictionary; the email may be missing (`owner <no email>`); unknown host or missing owner → `nobody (unassigned)`
- `CityOf(Site? site)`: `site?.Address?.City` with fallback `unknown`
- `Normalize(string? s)` annotated `[return: NotNullIfNotNull(nameof(s))]`: trimmed, lower-cased, or null for null
@@starter
using System.Diagnostics.CodeAnalysis;

var owners = new Dictionary<string, string?> { ["web01"] = "ana", ["db01"] = null, ["mail01"] = "" , ["vpn01"] = "kenji" };
var emails = new Dictionary<string, string> { ["ana"] = "ana@example.jp" };
Console.WriteLine("TODO");

record Address(string? City);
record Site(string Name, Address? Address);
@@expected
web01: ana <ana@example.jp>
db01: nobody (unassigned)
mail01: nobody (unassigned)
vpn01: kenji <no email>
nas01: nobody (unassigned)
Tokyo
unknown
unknown
unknown
[web01] [null]
@@requires
NotNullWhen(true)
NotNullIfNotNull
@@hint
`TryGetOwner`: `owner = owners.GetValueOrDefault(host); return !string.IsNullOrEmpty(owner);`. `string.IsNullOrEmpty` is itself annotated with `[NotNullWhen(false)]`, so the compiler accepts your `[NotNullWhen(true)]` promise. Sites: `new Site("hq", new Address("Tokyo"))`, `new Site("dc", new Address(null))`, `new Site("remote", null)`, and `null`. Print `Normalize("  WEB01 ")` and `Normalize(null)` as `[{a}] [{b ?? "null"}]`.
@@solution
using System.Diagnostics.CodeAnalysis;

var owners = new Dictionary<string, string?> { ["web01"] = "ana", ["db01"] = null, ["mail01"] = "" , ["vpn01"] = "kenji" };
var emails = new Dictionary<string, string> { ["ana"] = "ana@example.jp" };

foreach (var h in new[] { "web01", "db01", "mail01", "vpn01", "nas01" })
    Console.WriteLine($"{h}: {Contact(h)}");

Site?[] sites = [new Site("hq", new Address("Tokyo")), new Site("dc", new Address(null)), new Site("remote", null), null];
foreach (var s in sites) Console.WriteLine(CityOf(s));

string a = Normalize("  WEB01 ");
string? b = Normalize(null);
Console.WriteLine($"[{a}] [{b ?? "null"}]");

bool TryGetOwner(string host, [NotNullWhen(true)] out string? owner)
{
    owner = owners.GetValueOrDefault(host);
    return !string.IsNullOrEmpty(owner);
}

string Contact(string host)
{
    if (!TryGetOwner(host, out var owner)) return "nobody (unassigned)";
    var email = emails.GetValueOrDefault(owner);
    return $"{owner} <{email ?? "no email"}>";
}

string CityOf(Site? site) => site?.Address?.City ?? "unknown";

[return: NotNullIfNotNull(nameof(s))]
string? Normalize(string? s) => s?.Trim().ToLowerInvariant();

record Address(string? City);
record Site(string Name, Address? Address);

@@exercise A Result type with Bind
Build `Result<T>` as a `readonly record struct` with `Value`, `Error`, `IsOk`, `Ok(value)`, `Fail(error)` and:

- `Map<U>(Func<T, U> f)`: transform the value if OK, otherwise pass the error along
- `Bind<U>(Func<T, Result<U>> f)`: chain a step that can itself fail
- `Match<U>(Func<T, U> ok, Func<string, U> fail)`

Then write a pipeline for "add a user" requests: `ParseAge(string)` → `Validate(int)` (18–70) → `Save(int)` (fails when the store already holds 3 users), composed with `Bind` and `Map`, and report each request with `Match`.
@@starter
var store = new List<int>();
string[] requests = ["34", "abc", "17", "45", "29", "52"];
Console.WriteLine("TODO");
@@expected
34 -> saved #1 (age 34)
abc -> error: 'abc' is not a number
17 -> error: age 17 is outside 18-70
45 -> saved #2 (age 45)
29 -> saved #3 (age 29)
52 -> error: store is full
@@requires
Bind(
Match(
@@hint
`public Result<U> Bind<U>(Func<T, Result<U>> f) => IsOk ? f(Value!) : Result<U>.Fail(Error!);`. `Save` returns `Result<int>` with the new user number. Pipeline: `ParseAge(r).Bind(Validate).Bind(Save).Map(n => (n, age))`... simpler: `.Bind(age => Save(age).Map(n => $"saved #{n} (age {age})"))`.
@@solution
var store = new List<int>();
string[] requests = ["34", "abc", "17", "45", "29", "52"];
foreach (var r in requests)
{
    var outcome = ParseAge(r)
        .Bind(Validate)
        .Bind(age => Save(age).Map(n => $"saved #{n} (age {age})"));
    Console.WriteLine($"{r} -> {outcome.Match(ok => ok, err => $"error: {err}")}");
}

Result<int> ParseAge(string s) =>
    int.TryParse(s, out var n) ? Result<int>.Ok(n) : Result<int>.Fail($"'{s}' is not a number");

Result<int> Validate(int age) =>
    age is >= 18 and <= 70 ? Result<int>.Ok(age) : Result<int>.Fail($"age {age} is outside 18-70");

Result<int> Save(int age)
{
    if (store.Count >= 3) return Result<int>.Fail("store is full");
    store.Add(age);
    return Result<int>.Ok(store.Count);
}

readonly record struct Result<T>(T? Value, string? Error)
{
    public bool IsOk => Error is null;
    public static Result<T> Ok(T value) => new(value, null);
    public static Result<T> Fail(string error) => new(default, error);

    public Result<U> Map<U>(Func<T, U> f) => IsOk ? Result<U>.Ok(f(Value!)) : Result<U>.Fail(Error!);
    public Result<U> Bind<U>(Func<T, Result<U>> f) => IsOk ? f(Value!) : Result<U>.Fail(Error!);
    public U Match<U>(Func<T, U> ok, Func<string, U> fail) => IsOk ? ok(Value!) : fail(Error!);
}

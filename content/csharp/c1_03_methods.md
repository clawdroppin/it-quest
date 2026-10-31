@@lesson c1-methods Methods
@@concepts functions
@@text
## Methods (functions)

In C#, functions are called **methods**. With top-level statements you can declare them right in the file:

```csharp
double ToGb(long bytes)
{
    return bytes / 1024.0 / 1024 / 1024;
}

// One-expression methods can use =>
string Bar(double ratio) => new string('#', (int)(ratio * 10)).PadRight(10, '.');

Console.WriteLine($"{ToGb(8_589_934_592):F1} GB");
Console.WriteLine($"[{Bar(0.7)}]");
```

The type before the name is what it **returns**; `void` means nothing. Each parameter has a type too.

## Optional and named arguments

```csharp
string Ping(string host, int count = 4, int timeoutMs = 1000) =>
    $"ping {host} x{count} (timeout {timeoutMs}ms)";

Console.WriteLine(Ping("10.0.0.1"));
Console.WriteLine(Ping("10.0.0.1", 2));
Console.WriteLine(Ping("10.0.0.1", timeoutMs: 250));   // named: skip `count`
```

Named arguments make calls readable: `CreateUser("aiko", admin: true)` says much more than `CreateUser("aiko", true)`.

## Returning several values: tuples

```csharp
(int ok, int failed) Tally(string[] results)
{
    int ok = 0, failed = 0;
    foreach (var r in results)
        if (r == "ok") ok++; else failed++;
    return (ok, failed);
}

var (good, bad) = Tally(new[] { "ok", "fail", "ok", "ok" });
Console.WriteLine($"{good} ok, {bad} failed");
```

## out and ref

`out` parameters are how `int.TryParse` hands back a second result. You can write your own, but a tuple is usually clearer. `ref` lets a method change the caller's variable. Use both sparingly.

```csharp
bool TryParsePort(string text, out int port) =>
    int.TryParse(text, out port) && port is >= 1 and <= 65535;

Console.WriteLine(TryParsePort("443", out int p) ? $"port {p}" : "bad");
Console.WriteLine(TryParsePort("0", out _) ? "ok" : "bad");   // out _ : don't care
```

## Overloading

Several methods can share a name if their parameters differ. The compiler picks the one that fits:

```csharp
Console.WriteLine(Show.Describe(5));
Console.WriteLine(Show.Describe("5"));

static class Show
{
    public static string Describe(int n) => $"int {n}";
    public static string Describe(string s) => $"string \"{s}\"";
}
```

(Overloads must be methods of a class: local functions in top-level code can't share a name.)

## Recursion

A method may call itself. Each call needs to get closer to a case that stops:

```csharp
long Factorial(int n) => n <= 1 ? 1 : n * Factorial(n - 1);
Console.WriteLine(Factorial(20));
```

## Static methods in classes

In bigger programs, methods live inside classes. A `static` method belongs to the class itself (like `Math.Max` or `int.Parse`), not to an object:

```csharp
Console.WriteLine(Units.ToMb(5_242_880));

static class Units
{
    public static double ToMb(long bytes) => bytes / 1048576.0;
}
```

Type declarations like `static class Units` must come *after* the top-level statements.

@@quiz
What does a method declared `void Log(string msg)` return?
* Nothing
- `null`
- An empty string
- `msg`
? `void` means "no return value". `return;` with no value can still end it early.

@@quiz
`string Ping(string host, int count = 4)`: which call is **not** valid?
* `Ping(count: 2)`
- `Ping("a")`
- `Ping("a", 2)`
- `Ping(host: "a", count: 2)`
? `host` has no default, so it must be given.

@@quiz
What does `var (a, b) = (1, 2);` do?
* Declares `a = 1` and `b = 2` by deconstructing a tuple
- Creates one variable holding a tuple
- Is a syntax error
- Declares `a` as a function
? Tuples can be taken apart into separate variables like this.

@@exercise Subnet host count
Write `HostsInSubnet(int prefix)` that returns the number of usable hosts in an IPv4 subnet with that prefix length: 2^(32 − prefix) − 2. Special cases: `/31` → 2 (point-to-point links) and `/32` → 1. Use `long`, because `/0` is huge.
@@starter
long HostsInSubnet(int prefix)
{
    throw new NotImplementedException("TODO: not done yet");
}

foreach (int p in new[] { 24, 26, 30, 31, 32, 8 })
    Console.WriteLine($"/{p}: {HostsInSubnet(p)}");
@@expected
/24: 254
/26: 62
/30: 2
/31: 2
/32: 1
/8: 16777214
@@hint
`1L << (32 - prefix)` is 2 to the power of (32 − prefix), as a `long`. Handle 31 and 32 first.
@@solution
long HostsInSubnet(int prefix)
{
    if (prefix == 32) return 1;
    if (prefix == 31) return 2;
    return (1L << (32 - prefix)) - 2;
}

foreach (int p in new[] { 24, 26, 30, 31, 32, 8 })
    Console.WriteLine($"/{p}: {HostsInSubnet(p)}");

@@exercise Uptime text
Write `FormatUptime(long seconds)` that returns a string like `3d 4h 5m`, leaving out leading zero parts (`4h 5m`, `5m`), and `0m` for under a minute. Return it as a tuple-free single string.
@@starter
string FormatUptime(long seconds)
{
    throw new NotImplementedException("TODO: not done yet");
}

Console.WriteLine(FormatUptime(273_900));
Console.WriteLine(FormatUptime(14_700));
Console.WriteLine(FormatUptime(300));
Console.WriteLine(FormatUptime(42));
@@expected
3d 4h 5m
4h 5m
5m
0m
@@hint
`long d = seconds / 86400, h = seconds % 86400 / 3600, m = seconds % 3600 / 60;` Then build the text: add the days part only if `d > 0`, the hours part if `d > 0 || h > 0`, and always the minutes.
@@solution
string FormatUptime(long seconds)
{
    long d = seconds / 86400, h = seconds % 86400 / 3600, m = seconds % 3600 / 60;
    string text = "";
    if (d > 0) text += $"{d}d ";
    if (d > 0 || h > 0) text += $"{h}h ";
    return text + $"{m}m";
}

Console.WriteLine(FormatUptime(273_900));
Console.WriteLine(FormatUptime(14_700));
Console.WriteLine(FormatUptime(300));
Console.WriteLine(FormatUptime(42));

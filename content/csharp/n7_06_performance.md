@@lesson n7-performance High-Performance C#: Span, Allocations, Pools and Benchmarks
@@concepts strings, testing
@@text
.NET is fast, and modern C# gives you tools to write allocation-free hot paths when profiling shows you need them: parsing millions of log lines, high-traffic APIs, game loops.

## Measure first

```csharp,norun
[MemoryDiagnoser]
public class ParseBenchmarks
{
    private readonly string line = "2026-11-03T09:12:44Z INFO auth 200 35ms";

    [Benchmark(Baseline = true)] public int WithSplit() => int.Parse(line.Split(' ')[3]);
    [Benchmark] public int WithSpan() => ParseStatus(line);
}
// dotnet run -c Release  -> BenchmarkDotNet prints mean time and allocated bytes per call
```

BenchmarkDotNet handles warm-up, statistics and JIT effects. In production, use `dotnet-counters` (GC and allocation rates live), `dotnet-trace` and the Visual Studio / Rider profilers.

## Span<T>: a view, not a copy

`Span<T>` and `ReadOnlySpan<T>` are **windows** over existing memory (an array, a string, stack memory) with no allocation. Slicing a span is free; `Substring` and `Split` allocate new strings.

```csharp
ReadOnlySpan<char> line = "2026-11-03T09:12:44Z INFO auth 200 35ms";
var rest = line;
for (int i = 0; i < 3; i++) rest = rest[(rest.IndexOf(' ') + 1)..];   // skip three fields
var statusText = rest[..rest.IndexOf(' ')];
int status = int.Parse(statusText);                                     // parses the span directly
Console.WriteLine(status + 1);
Console.WriteLine(line[21..25].SequenceEqual("INFO"));                 // compare without a new string
```

Rules: spans live only on the stack (no fields in classes, no `await` across them). Use `Memory<T>` when you need to store or pass them to async code.

## stackalloc and pools

```csharp
using System.Buffers;

Span<byte> small = stackalloc byte[64];          // stack memory: no GC, for small sizes only
small[0] = 42;
Console.WriteLine(small[0]);

byte[] big = ArrayPool<byte>.Shared.Rent(1_000_000);   // reuse large buffers
try
{
    Console.WriteLine(big.Length >= 1_000_000);   // may be larger than asked!
}
finally
{
    ArrayPool<byte>.Shared.Return(big);
}
```

## Other allocation traps

- LINQ in a hot loop allocates enumerators and delegates: fine in most code, but replace it in measured hot paths.
- Closures capturing variables allocate; `static` lambdas (`static x => x * 2`) can't capture by accident.
- Boxing: value types passed as `object` or non-generic interfaces.
- String concatenation in loops; use `StringBuilder` or `string.Create`.
- `params` arrays and `ToString()` on everything in logs (use log message templates).

## Struct design and the GC

Small, immutable `readonly struct`s avoid heap allocations; large structs are expensive to copy (pass with `in`). Objects of 85,000 bytes or more go to the **Large Object Heap**, which is collected less often, so pool big buffers. Server GC (the default for ASP.NET Core) uses one heap per core; `DOTNET_gcServer`, `GCHeapHardLimit` and `DOTNET_GCConserveMemory` tune it for containers.

## Native AOT and ReadyToRun

`<PublishAot>true</PublishAot>` compiles to native code: start-up in milliseconds and lower memory, useful for CLIs, functions and containers. It requires trimming-friendly code (source generators instead of reflection).

@@quiz
What does slicing a `ReadOnlySpan<char>` over a string do?
* Creates a view of part of the existing string, without copying or allocating
- Copies the characters to a new string
- Locks the string
- Converts it to UTF-8
? That's why span parsing is allocation-free.

@@quiz
Why can't a `Span<T>` be a field of a class?
- It's too large
* It may point to stack memory, so it must stay on the stack (it's a ref struct)
- Classes can't hold structs
- It's immutable
? Use Memory<T> for storage.

@@quiz
Why return rented arrays to `ArrayPool`?
* So the next caller can reuse them instead of allocating a new large array
- To free memory immediately
- It's required for correctness
- To zero the data
? Forgetting to return just means more allocations later.

@@drill
Which type is a view over contiguous memory without copying?
= Span<T>
= Span
? It can only live on the stack.

@@flashcards
`Span<T>` :: stack-only view over memory
`ReadOnlySpan<char>` :: allocation-free string slicing
`Memory<T>` :: storable, async-friendly memory view
`stackalloc` :: stack buffer for small sizes
`ArrayPool<T>.Shared` :: reusable arrays
`BenchmarkDotNet` :: reliable micro-benchmarks
`[MemoryDiagnoser]` :: report allocations per op
LOH :: Large Object Heap (85,000+ bytes)
`static` lambda :: can't capture variables
Native AOT :: ahead-of-time native compilation

@@exercise An allocation-free log parser
Write `bool TryParse(ReadOnlySpan<char> line, out Entry e)` for lines like `2026-11-03T09:12:44Z INFO auth 200 35ms`:

- `Entry` is a `readonly record struct (Level Level, int Status, int Ms)` with `enum Level { Info, Warn, Error }`
- use only span operations (`IndexOf`, slicing, `SequenceEqual`/`Equals`, `int.TryParse(ReadOnlySpan<char>)`): no `Split`, `Substring` or `ToString`
- return false for malformed lines

Then measure: run it 10,000 times (after a warm-up) between two calls to `GC.GetAllocatedBytesForCurrentThread()` and print the bytes allocated (must be 0), and compare with a `Split`-based version.
@@starter
string[] lines =
[
    "2026-11-03T09:12:44Z INFO auth 200 35ms",
    "2026-11-03T09:12:45Z WARN web 429 120ms",
    "2026-11-03T09:12:46Z ERROR db 500 2300ms",
    "garbage line",
    "2026-11-03T09:12:47Z DEBUG web 200 3ms",
];
Console.WriteLine("TODO");

enum Level { Info, Warn, Error }
readonly record struct Entry(Level Level, int Status, int Ms);
@@expected
Entry { Level = Info, Status = 200, Ms = 35 }
Entry { Level = Warn, Status = 429, Ms = 120 }
Entry { Level = Error, Status = 500, Ms = 2300 }
invalid: garbage line
invalid: 2026-11-03T09:12:47Z DEBUG web 200 3ms
span version allocated: 0 bytes
split version allocates: True
@@requires
ReadOnlySpan<char>
GC.GetAllocatedBytesForCurrentThread
@@hint
A helper `static bool Next(ref ReadOnlySpan<char> rest, out ReadOnlySpan<char> field)` that returns the text up to the next space and advances `rest` (the last field has no space after it). Level: `field.SequenceEqual("INFO")` etc. Ms: the field must end with `"ms"` (`EndsWith`), then parse `field[..^2]`. For the measurement, sum the statuses in the loop into a local so the work isn't optimized away. The split version: `var p = line.Split(' '); int.Parse(p[3])`.
@@solution
string[] lines =
[
    "2026-11-03T09:12:44Z INFO auth 200 35ms",
    "2026-11-03T09:12:45Z WARN web 429 120ms",
    "2026-11-03T09:12:46Z ERROR db 500 2300ms",
    "garbage line",
    "2026-11-03T09:12:47Z DEBUG web 200 3ms",
];
foreach (var l in lines)
    Console.WriteLine(TryParse(l, out var e) ? e.ToString() : $"invalid: {l}");

long sum = 0;
for (int i = 0; i < 1000; i++) sum += Measure();
long before = GC.GetAllocatedBytesForCurrentThread();
for (int i = 0; i < 10_000; i++) sum += Measure();
long spanBytes = GC.GetAllocatedBytesForCurrentThread() - before;
Console.WriteLine($"span version allocated: {spanBytes} bytes");

before = GC.GetAllocatedBytesForCurrentThread();
for (int i = 0; i < 10_000; i++) sum += int.Parse(lines[0].Split(' ')[3]);
Console.WriteLine($"split version allocates: {GC.GetAllocatedBytesForCurrentThread() - before > 0}");

int Measure() => TryParse(lines[0], out var e) ? e.Status : 0;

static bool Next(ref ReadOnlySpan<char> rest, out ReadOnlySpan<char> field)
{
    if (rest.IsEmpty)
    {
        field = default;
        return false;
    }
    int space = rest.IndexOf(' ');
    if (space < 0)
    {
        field = rest;
        rest = default;
    }
    else
    {
        field = rest[..space];
        rest = rest[(space + 1)..];
    }
    return true;
}

static bool TryParse(ReadOnlySpan<char> line, out Entry e)
{
    e = default;
    var rest = line;
    if (!Next(ref rest, out _) || !Next(ref rest, out var lvl) || !Next(ref rest, out _)
        || !Next(ref rest, out var st) || !Next(ref rest, out var ms) || !rest.IsEmpty)
        return false;
    Level level;
    if (lvl.SequenceEqual("INFO")) level = Level.Info;
    else if (lvl.SequenceEqual("WARN")) level = Level.Warn;
    else if (lvl.SequenceEqual("ERROR")) level = Level.Error;
    else return false;
    if (!int.TryParse(st, out int status) || !ms.EndsWith("ms") || !int.TryParse(ms[..^2], out int millis))
        return false;
    e = new Entry(level, status, millis);
    return true;
}

enum Level { Info, Warn, Error }
readonly record struct Entry(Level Level, int Status, int Ms);

@@exercise Pooled buffers and stack formatting
Two classic hot-path techniques:

1. `string FormatIp(uint ip)`: format an IPv4 address using a `stackalloc char[15]` buffer and `TryFormat` on each byte, creating exactly one string at the end (`new string(span[..len])`)
2. `long Checksum(Stream s)`: read a stream in chunks into a buffer **rented** from a pool and sum all bytes; always return the buffer (use `try/finally`)

To prove the pool usage is correct, wrap `ArrayPool<byte>.Shared` in a `CountingPool : ArrayPool<byte>` that counts rents and returns, and pass it to `Checksum`.
@@starter
using System.Buffers;

uint[] ips = [0x0A000005, 0xC0A80101, 0xFFFFFFFF, 0];
Console.WriteLine("TODO");
@@expected
10.0.0.5
192.168.1.1
255.255.255.255
0.0.0.0
checksum of 100,000 bytes: 12742320
rents: 1, returns: 1
checksum of empty stream: 0
rents: 2, returns: 2
@@requires
stackalloc
ArrayPool<byte>
@@hint
`FormatIp`: `Span<char> buf = stackalloc char[15]; int len = 0; for (int shift = 24; shift >= 0; shift -= 8) { ((byte)(ip >> shift)).TryFormat(buf[len..], out int w); len += w; if (shift > 0) buf[len++] = '.'; }`. `CountingPool` overrides `Rent(int)` and `Return(byte[], bool)` and delegates to `ArrayPool<byte>.Shared`. Test data: a `MemoryStream` over `Enumerable.Range(0, 100_000).Select(i => (byte)(i % 256)).ToArray()`, read with a 4096-byte buffer.
@@solution
using System.Buffers;

uint[] ips = [0x0A000005, 0xC0A80101, 0xFFFFFFFF, 0];
foreach (var ip in ips) Console.WriteLine(FormatIp(ip));

var pool = new CountingPool();
var data = Enumerable.Range(0, 100_000).Select(i => (byte)(i % 256)).ToArray();
Console.WriteLine($"checksum of 100,000 bytes: {Checksum(new MemoryStream(data), pool)}");
Console.WriteLine($"rents: {pool.Rents}, returns: {pool.Returns}");
Console.WriteLine($"checksum of empty stream: {Checksum(new MemoryStream(), pool)}");
Console.WriteLine($"rents: {pool.Rents}, returns: {pool.Returns}");

static string FormatIp(uint ip)
{
    Span<char> buf = stackalloc char[15];
    int len = 0;
    for (int shift = 24; shift >= 0; shift -= 8)
    {
        ((byte)(ip >> shift)).TryFormat(buf[len..], out int written);
        len += written;
        if (shift > 0) buf[len++] = '.';
    }
    return new string(buf[..len]);
}

static long Checksum(Stream s, ArrayPool<byte> pool)
{
    byte[] buffer = pool.Rent(4096);
    try
    {
        long sum = 0;
        int read;
        while ((read = s.Read(buffer, 0, 4096)) > 0)
            foreach (var b in buffer.AsSpan(0, read)) sum += b;
        return sum;
    }
    finally
    {
        pool.Return(buffer);
    }
}

class CountingPool : ArrayPool<byte>
{
    public int Rents { get; private set; }
    public int Returns { get; private set; }

    public override byte[] Rent(int minimumLength)
    {
        Rents++;
        return Shared.Rent(minimumLength);
    }

    public override void Return(byte[] array, bool clearArray = false)
    {
        Returns++;
        Shared.Return(array, clearArray);
    }
}

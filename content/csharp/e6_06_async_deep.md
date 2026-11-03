@@lesson e6-async async in Depth: Cancellation, Throttling, Channels and Async Streams
@@concepts concurrency
@@text
You've used `async`/`await` for HTTP calls. Production code also needs cancellation, limits on concurrency, producer/consumer pipelines, and an understanding of the traps.

## What await really does

`async` methods are compiled into state machines. `await task` checks whether the task finished; if not, the method returns to its caller and **resumes later** when the task completes, without blocking a thread. That's why one web server thread can serve many requests that are waiting on databases.

## The traps

- **Never block on async code**: `task.Result` / `task.Wait()` can deadlock in UI apps and old ASP.NET, and waste threads everywhere. Go "async all the way".
- **`async void`** only for event handlers: exceptions from it crash the process and it can't be awaited. Return `Task`.
- `await Task.WhenAll(...)` throws the **first** exception; the others are in `task.Exception.InnerExceptions`.
- In libraries, `await x.ConfigureAwait(false)` avoids capturing the UI context; app code (and ASP.NET Core) usually doesn't need it.
- `ValueTask<T>` avoids an allocation when results are often available synchronously (caches); await it exactly once.

## Cancellation

```csharp
using var cts = new CancellationTokenSource(TimeSpan.FromMilliseconds(100));
try
{
    await SlowAsync(cts.Token);
}
catch (OperationCanceledException)
{
    Console.WriteLine("cancelled after 100 ms");
}

// a per-call timeout layered on a caller's token
using var outer = new CancellationTokenSource();
using var linked = CancellationTokenSource.CreateLinkedTokenSource(outer.Token);
linked.CancelAfter(50);
Console.WriteLine(await Task.Delay(500, linked.Token).ContinueWith(t => t.IsCanceled));

async Task SlowAsync(CancellationToken ct)
{
    for (int i = 0; i < 10; i++)
    {
        ct.ThrowIfCancellationRequested();
        await Task.Delay(50, ct);
    }
}
```

Accept a `CancellationToken` as the **last** parameter of every async method that does I/O, and pass it on to everything you call. `task.WaitAsync(timeout)` adds a timeout to a task you don't control.

## Limiting concurrency

```csharp,norun
var gate = new SemaphoreSlim(4);              // at most 4 at once
var tasks = hosts.Select(async h =>
{
    await gate.WaitAsync(ct);
    try { return await ProbeAsync(h, ct); }
    finally { gate.Release(); }
});
var results = await Task.WhenAll(tasks);     // results in input order

// or, built in (.NET 6+):
await Parallel.ForEachAsync(hosts, new ParallelOptions { MaxDegreeOfParallelism = 4, CancellationToken = ct },
    async (h, token) => await ProbeAsync(h, token));
```

`Task.WhenAll` returns results in the **order of the input**, whatever order they finish in.

## Async streams

```csharp
await foreach (var line in ReadLinesAsync())
    Console.WriteLine(line);

async IAsyncEnumerable<string> ReadLinesAsync()
{
    foreach (var l in new[] { "INFO start", "WARN disk 85%", "INFO done" })
    {
        await Task.Delay(10);       // e.g. reading from a network stream page by page
        yield return l;
    }
}
```

## Channels: producer/consumer

`System.Threading.Channels.Channel<T>` is an async queue. A **bounded** channel makes fast producers wait when consumers fall behind (backpressure):

```csharp,norun
var ch = Channel.CreateBounded<string>(100);
var producer = Task.Run(async () =>
{
    foreach (var line in File.ReadLines("app.log")) await ch.Writer.WriteAsync(line);
    ch.Writer.Complete();                           // no more items
});
var consumers = Enumerable.Range(0, 3).Select(_ => Task.Run(async () =>
{
    await foreach (var line in ch.Reader.ReadAllAsync()) Process(line);
}));
await Task.WhenAll(consumers.Append(producer));
```

## Thread-safe state

Code after `await` may run on a different thread. Shared state touched by concurrent tasks needs `Interlocked` (counters), `lock` (short critical sections, never around an `await`), `ConcurrentDictionary`, or a design where only one consumer owns the state.

@@quiz
Why is `var html = GetPageAsync().Result;` dangerous?
* It blocks a thread and can deadlock in UI or legacy ASP.NET apps
- It returns null
- It's a compile error
- It runs the method twice
? Use await all the way up.

@@quiz
In what order does `await Task.WhenAll(tasks)` return results?
- Completion order
* The order of the input tasks
- Random
- Sorted
? Even if later tasks finish first.

@@quiz
What does a bounded channel do when it's full?
- Drops items
- Throws
* Makes writers wait (WriteAsync doesn't complete) until consumers catch up
- Grows without limit
? Backpressure protects memory.

@@flashcards
`CancellationToken` :: cooperative cancellation signal
`CancelAfter` :: timeout on a token source
`CreateLinkedTokenSource` :: cancel when any parent cancels
`OperationCanceledException` :: thrown on cancellation
`SemaphoreSlim` :: async-friendly concurrency limit
`Parallel.ForEachAsync` :: bounded parallel async loop
`IAsyncEnumerable<T>` :: async stream
`await foreach` :: consume an async stream
`Channel<T>` :: async producer/consumer queue
`Interlocked.Increment` :: atomic counter
`async void` :: only for event handlers

@@exercise Throttled host checks with timeouts
Write `CheckAllAsync(IReadOnlyList<string> hosts, int maxConcurrency, TimeSpan timeout, Func<string, CancellationToken, Task<int>> probe)` returning `string[]` in **input order**:

- at most `maxConcurrency` probes at once (`SemaphoreSlim`)
- each probe gets a token that cancels after `timeout` (`CancellationTokenSource.CancelAfter`); a cancelled probe gives `<host>: timeout`
- other exceptions give `<host>: error <message>`; success gives `<host>: <ms> ms`
- track the current and peak number of running probes with `Interlocked` and print whether the peak stayed within the limit

The fake probe (provided) waits a host-specific time with `Task.Delay(ms, token)`.
@@starter
var delays = new Dictionary<string, int> { ["web01"] = 30, ["web02"] = 20, ["db01"] = 400, ["mail01"] = 10, ["vpn01"] = 25, ["nas01"] = 15 };
int running = 0, peak = 0;

async Task<int> Probe(string host, CancellationToken ct)
{
    int now = Interlocked.Increment(ref running);
    int p;
    while (now > (p = Volatile.Read(ref peak)) && Interlocked.CompareExchange(ref peak, now, p) != p) { }
    try
    {
        if (host == "mail01") throw new InvalidOperationException("connection refused");
        await Task.Delay(delays[host], ct);
        return delays[host];
    }
    finally
    {
        Interlocked.Decrement(ref running);
    }
}

Console.WriteLine("TODO");
@@expected
web01: 30 ms
web02: 20 ms
db01: timeout
mail01: error connection refused
vpn01: 25 ms
nas01: 15 ms
peak within limit: True
peak above 1: True
@@requires
SemaphoreSlim
CancelAfter
@@hint
`var gate = new SemaphoreSlim(maxConcurrency);` and `hosts.Select(async h => { await gate.WaitAsync(); try { using var cts = new CancellationTokenSource(); cts.CancelAfter(timeout); ... } finally { gate.Release(); } })`. Catch `OperationCanceledException` before the general `Exception`. Call with a limit of 3 and a 100 ms timeout, then print `peak <= 3` and `peak > 1`.
@@solution
var delays = new Dictionary<string, int> { ["web01"] = 30, ["web02"] = 20, ["db01"] = 400, ["mail01"] = 10, ["vpn01"] = 25, ["nas01"] = 15 };
int running = 0, peak = 0;

async Task<int> Probe(string host, CancellationToken ct)
{
    int now = Interlocked.Increment(ref running);
    int p;
    while (now > (p = Volatile.Read(ref peak)) && Interlocked.CompareExchange(ref peak, now, p) != p) { }
    try
    {
        if (host == "mail01") throw new InvalidOperationException("connection refused");
        await Task.Delay(delays[host], ct);
        return delays[host];
    }
    finally
    {
        Interlocked.Decrement(ref running);
    }
}

var results = await CheckAllAsync(delays.Keys.ToList(), 3, TimeSpan.FromMilliseconds(100), Probe);
foreach (var r in results) Console.WriteLine(r);
Console.WriteLine($"peak within limit: {peak <= 3}");
Console.WriteLine($"peak above 1: {peak > 1}");

async Task<string[]> CheckAllAsync(IReadOnlyList<string> hosts, int maxConcurrency, TimeSpan timeout,
    Func<string, CancellationToken, Task<int>> probe)
{
    using var gate = new SemaphoreSlim(maxConcurrency);
    var tasks = hosts.Select(async h =>
    {
        await gate.WaitAsync();
        try
        {
            using var cts = new CancellationTokenSource();
            cts.CancelAfter(timeout);
            int ms = await probe(h, cts.Token);
            return $"{h}: {ms} ms";
        }
        catch (OperationCanceledException)
        {
            return $"{h}: timeout";
        }
        catch (Exception e)
        {
            return $"{h}: error {e.Message}";
        }
        finally
        {
            gate.Release();
        }
    });
    return await Task.WhenAll(tasks);
}

@@exercise A log pipeline with a channel
Build a producer/consumer pipeline:

- the producer is an `IAsyncEnumerable<string>` of log lines (provided); copy it into a **bounded** channel of capacity 4 and complete the writer at the end
- 3 consumers read with `ReadAllAsync`, parse `LEVEL service message` lines and count per `service/LEVEL` in a `ConcurrentDictionary<string, int>` (`AddOrUpdate`); malformed lines count under `invalid`
- after all tasks finish, print the counts sorted by key and the total
@@starter
using System.Collections.Concurrent;
using System.Threading.Channels;

async IAsyncEnumerable<string> Lines()
{
    string[] all =
    [
        "INFO auth login ok", "WARN auth slow login", "ERROR db timeout", "INFO web GET /",
        "garbage", "INFO web GET /about", "ERROR web 500 on /cart", "INFO auth logout",
        "WARN db replication lag", "INFO web GET /", "", "ERROR db deadlock",
    ];
    foreach (var l in all)
    {
        await Task.Yield();
        yield return l;
    }
}

Console.WriteLine("TODO");
@@expected
auth/INFO 2
auth/WARN 1
db/ERROR 2
db/WARN 1
invalid 2
web/ERROR 1
web/INFO 3
total 12
@@requires
Channel.CreateBounded
ReadAllAsync
@@hint
`var ch = Channel.CreateBounded<string>(4);` Producer: `await foreach (var l in Lines()) await ch.Writer.WriteAsync(l); ch.Writer.Complete();`. Consumer: `await foreach (var line in ch.Reader.ReadAllAsync()) { var parts = line.Split(' ', 3); string key = parts.Length == 3 && parts[0] is "INFO" or "WARN" or "ERROR" ? ... : "invalid"; counts.AddOrUpdate(key, 1, (_, n) => n + 1); }`. Careful with operator precedence: put the `is` pattern in parentheses. Print with `counts.OrderBy(kv => kv.Key, StringComparer.Ordinal)`.
@@solution
using System.Collections.Concurrent;
using System.Threading.Channels;

async IAsyncEnumerable<string> Lines()
{
    string[] all =
    [
        "INFO auth login ok", "WARN auth slow login", "ERROR db timeout", "INFO web GET /",
        "garbage", "INFO web GET /about", "ERROR web 500 on /cart", "INFO auth logout",
        "WARN db replication lag", "INFO web GET /", "", "ERROR db deadlock",
    ];
    foreach (var l in all)
    {
        await Task.Yield();
        yield return l;
    }
}

var ch = Channel.CreateBounded<string>(4);
var counts = new ConcurrentDictionary<string, int>();

var producer = Task.Run(async () =>
{
    await foreach (var l in Lines()) await ch.Writer.WriteAsync(l);
    ch.Writer.Complete();
});

var consumers = Enumerable.Range(0, 3).Select(_ => Task.Run(async () =>
{
    await foreach (var line in ch.Reader.ReadAllAsync())
    {
        var parts = line.Split(' ', 3);
        string key = parts.Length == 3 && (parts[0] is "INFO" or "WARN" or "ERROR")
            ? $"{parts[1]}/{parts[0]}"
            : "invalid";
        counts.AddOrUpdate(key, 1, (_, n) => n + 1);
    }
}));

await Task.WhenAll(consumers.Append(producer));
foreach (var (key, n) in counts.OrderBy(kv => kv.Key, StringComparer.Ordinal))
    Console.WriteLine($"{key} {n}");
Console.WriteLine($"total {counts.Values.Sum()}");

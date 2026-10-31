@@lesson c3-async async, await and HTTP
@@concepts concurrency, networking
@@text
## Why async?

Most IT code waits: for the network, a disk, a database. While one request waits 200 ms, the computer could start fifty more. `async`/`await` lets you write waiting code that reads top to bottom, while .NET does other work during the waits.

```csharp
async Task<string> CheckAsync(string host, int delayMs)
{
    await Task.Delay(delayMs);        // stands in for a network call
    return $"{host} ok after {delayMs}ms";
}

Console.WriteLine(await CheckAsync("web01", 100));
```

- An `async` method returns a `Task` (no result) or `Task<T>` (a result of type T).
- `await` waits for the task *without blocking the thread*.
- By convention async methods end in `Async`.

## Doing things at the same time

Start all the tasks first, then wait for all of them with `Task.WhenAll`:

```csharp
using System.Diagnostics;

async Task<string> CheckAsync(string host, int delayMs)
{
    await Task.Delay(delayMs);
    return host;
}

var sw = Stopwatch.StartNew();
var tasks = new[] { CheckAsync("a", 300), CheckAsync("b", 300), CheckAsync("c", 300) };
string[] done = await Task.WhenAll(tasks);
Console.WriteLine($"{string.Join(",", done)} in ~{sw.ElapsedMilliseconds / 100 * 100}ms, not 900");
```

Three 300 ms waits take about 300 ms in total, because they overlap.

`Task.WhenAny` returns as soon as the first finishes: useful for timeouts. To limit how many run at once (don't open 10,000 connections), use `SemaphoreSlim` or `Parallel.ForEachAsync` with `MaxDegreeOfParallelism`.

## Timeouts and cancellation

```csharp
using var cts = new CancellationTokenSource(TimeSpan.FromMilliseconds(150));
try
{
    await Task.Delay(1000, cts.Token);
}
catch (OperationCanceledException)
{
    Console.WriteLine("gave up after 150ms");
}
```

Pass a `CancellationToken` through your async methods so a slow operation can be stopped.

## HTTP with HttpClient

```csharp,norun
using var http = new HttpClient { Timeout = TimeSpan.FromSeconds(5) };
var resp = await http.GetAsync("https://example.com/health");
Console.WriteLine((int)resp.StatusCode);        // 200
string body = await resp.Content.ReadAsStringAsync();

// JSON straight into your types (System.Net.Http.Json):
// var user = await http.GetFromJsonAsync<User>("https://api.example.jp/users/1");
```

Create **one** `HttpClient` and reuse it. Creating one per request can run out of sockets on a busy server.

## Mistakes to avoid

- **`.Result` or `.Wait()`** on a task blocks the thread, and in UI or ASP.NET apps it can deadlock. Use `await` all the way up.
- **`async void`**: exceptions in it can crash the process. Return `Task` instead (only event handlers use `async void`).
- **Forgetting `await`**: the method carries on before the work finishes, and errors disappear. The compiler warns (CS4014), so read your warnings.

## Threads and locks, briefly

`await` is for *waiting*. For CPU-heavy work on many cores, use `Parallel.For` or `Task.Run`. When several threads change the same data, protect it with `lock`, or use thread-safe types like `ConcurrentDictionary` and `Interlocked.Increment`.

```csharp
int counter = 0;
Parallel.For(0, 10_000, _ => Interlocked.Increment(ref counter));
Console.WriteLine(counter);   // always 10000. With counter++ it could be less!
```

@@quiz
What does `await Task.WhenAll(t1, t2, t3)` do?
* Waits until all three tasks have finished, letting them run at the same time
- Runs t1, then t2, then t3, one after another
- Waits for the first one only
- Cancels the slowest
? The tasks were already started; `WhenAll` waits for all of them. Total time ≈ the slowest one.

@@quiz
Why is calling `.Result` on a task in an ASP.NET or desktop app a problem?
* It blocks the thread and can deadlock
- It returns the wrong value
- It doesn't compile
- It runs the task twice
? Use `await` instead of blocking.

@@quiz
Which return type should an async method with no result have (outside event handlers)?
* `Task`
- `void`
- `Task<void>`
- `object`
? `async void` can't be awaited, and its exceptions can crash the program.

@@exercise Check all hosts
`ProbeAsync` pretends to check a host (it waits and returns whether it's up). Check all hosts **at the same time** with `Task.WhenAll`, then print `<host>: up/down` in the original order and `2/4 up`.
@@starter
async Task<bool> ProbeAsync(string host)
{
    await Task.Delay(50);
    return !host.StartsWith("old");
}

string[] hosts = { "web01", "old-ftp", "db01", "old-mail" };
// start all probes, await them together, print the results

@@expected
web01: up
old-ftp: down
db01: up
old-mail: down
2/4 up
@@requires
Task.WhenAll
@@hint
`bool[] results = await Task.WhenAll(hosts.Select(ProbeAsync));` Then loop with an index: `for (int i = 0; i < hosts.Length; i++)`.
@@solution
async Task<bool> ProbeAsync(string host)
{
    await Task.Delay(50);
    return !host.StartsWith("old");
}

string[] hosts = { "web01", "old-ftp", "db01", "old-mail" };
bool[] results = await Task.WhenAll(hosts.Select(ProbeAsync));
for (int i = 0; i < hosts.Length; i++)
    Console.WriteLine($"{hosts[i]}: {(results[i] ? "up" : "down")}");
Console.WriteLine($"{results.Count(r => r)}/{hosts.Length} up");

@@exercise With a timeout
`SlowAsync(ms)` takes `ms` milliseconds. Write `WithTimeoutAsync(int ms, int limitMs)` that returns `"done"` if it finishes in time, or `"timeout"` otherwise, using `Task.WhenAny` and `Task.Delay(limitMs)`.
@@starter
async Task<string> SlowAsync(int ms)
{
    await Task.Delay(ms);
    return "done";
}

async Task<string> WithTimeoutAsync(int ms, int limitMs)
{
    throw new NotImplementedException("TODO: not done yet");
}

Console.WriteLine(await WithTimeoutAsync(20, 500));
Console.WriteLine(await WithTimeoutAsync(800, 100));
@@expected
done
timeout
@@requires
Task.WhenAny
@@hint
`var work = SlowAsync(ms); var winner = await Task.WhenAny(work, Task.Delay(limitMs));` If `winner == work`, return `await work`.
@@solution
async Task<string> SlowAsync(int ms)
{
    await Task.Delay(ms);
    return "done";
}

async Task<string> WithTimeoutAsync(int ms, int limitMs)
{
    var work = SlowAsync(ms);
    var winner = await Task.WhenAny(work, Task.Delay(limitMs));
    return winner == work ? await work : "timeout";
}

Console.WriteLine(await WithTimeoutAsync(20, 500));
Console.WriteLine(await WithTimeoutAsync(800, 100));

@@lesson e6-project Project: A Plugin-Based Job Scheduler
@@concepts concurrency, objects
@@text
This project combines the module: interfaces and attributes for plugins, reflection for discovery, events for notifications, async with cancellation and bounded concurrency, and results as values.

## The design

```text
[Job("backup", Priority = 1)] class BackupJob : IJob ─┐
[Job("cleanup", Priority = 3)] class CleanupJob : IJob ┼─► discovery (reflection) ─► JobRunner ─► events ─► log / alerts
[Job("report", Priority = 2)] class ReportJob : IJob ─┘                              │
                                                                                     ├ max N in parallel (SemaphoreSlim)
                                                                                     ├ retries with backoff (injected delay)
                                                                                     └ CancellationToken everywhere
```

Real systems with this shape: Hangfire and Quartz.NET (job schedulers), Windows services built on `BackgroundService`, and CI agents.

## Discovery

```csharp,norun
var jobs = Assembly.GetExecutingAssembly().GetTypes()
    .Where(t => typeof(IJob).IsAssignableFrom(t) && !t.IsAbstract)
    .Select(t => (Type: t, Meta: t.GetCustomAttribute<JobAttribute>()))
    .Where(x => x.Meta is not null)
    .OrderBy(x => x.Meta!.Priority);
```

In production you'd register jobs in the dependency injection container instead, or let a source generator build the list. Reflection discovery is how many plugin systems still work, though.

## Testable time

The runner takes a `Func<TimeSpan, CancellationToken, Task>` for its backoff delays. Tests pass a fake that records the delays and returns immediately; production passes `Task.Delay`. .NET 8's `TimeProvider` abstraction does the same for clocks and timers.

@@quiz
Why inject the delay function into the retry logic?
* Tests can run instantly and assert the exact backoff schedule
- Task.Delay doesn't support cancellation
- It's required by SemaphoreSlim
- To make retries random
? Same idea as TimeProvider.

@@quiz
What does `typeof(IJob).IsAssignableFrom(t)` check?
- Whether t is an attribute
* Whether a t instance can be used as an IJob (t implements or derives from it)
- Whether t is abstract
- Whether IJob has a constructor
? The core of plugin discovery.

@@flashcards
Plugin discovery :: scan types for an interface or attribute
`IsAssignableFrom` :: "can this type be used as that type?"
`BackgroundService` :: base class for hosted background workers
Exponential backoff :: delays 1×, 2×, 4×...
`TimeProvider` :: abstract clock (.NET 8)
Hangfire / Quartz.NET :: .NET job schedulers

@@exercise Discovering jobs
Write `Discover()` returning `(string Name, int Priority, IJob Job)` for every non-abstract class in the running assembly that implements `IJob` **and** has a `[Job]` attribute, sorted by priority then name, creating each with `Activator.CreateInstance`.

Then run them one by one (in order) with `await job.RunAsync(ct)`, printing `name (pN): result`. `DisabledJob` has no attribute and must be skipped; `JobBase` is abstract.
@@starter
using System.Reflection;

Console.WriteLine("TODO");

interface IJob { Task<string> RunAsync(CancellationToken ct); }

[AttributeUsage(AttributeTargets.Class)]
class JobAttribute(string name) : Attribute
{
    public string Name { get; } = name;
    public int Priority { get; init; } = 5;
}

abstract class JobBase : IJob
{
    public abstract Task<string> RunAsync(CancellationToken ct);
}

[Job("cleanup", Priority = 3)]
class CleanupJob : JobBase
{
    public override Task<string> RunAsync(CancellationToken ct) => Task.FromResult("deleted 42 temp files");
}

[Job("backup", Priority = 1)]
class BackupJob : JobBase
{
    public override async Task<string> RunAsync(CancellationToken ct)
    {
        await Task.Delay(10, ct);
        return "copied 1,204 files";
    }
}

[Job("report", Priority = 3)]
class ReportJob : IJob
{
    public Task<string> RunAsync(CancellationToken ct) => Task.FromResult("sent weekly report");
}

class DisabledJob : IJob
{
    public Task<string> RunAsync(CancellationToken ct) => Task.FromResult("should not run");
}
@@expected
found 3 jobs
backup (p1): copied 1,204 files
cleanup (p3): deleted 42 temp files
report (p3): sent weekly report
@@requires
IsAssignableFrom
Activator.CreateInstance
@@hint
`Assembly.GetExecutingAssembly().GetTypes().Where(t => !t.IsAbstract && typeof(IJob).IsAssignableFrom(t))`, read `t.GetCustomAttribute<JobAttribute>()`, skip nulls, and `OrderBy(x => x.Priority).ThenBy(x => x.Name, StringComparer.Ordinal)`. Top-level code needs a `CancellationToken`: `CancellationToken.None` is fine here.
@@solution
using System.Reflection;

var jobs = Discover();
Console.WriteLine($"found {jobs.Count} jobs");
foreach (var (name, priority, job) in jobs)
    Console.WriteLine($"{name} (p{priority}): {await job.RunAsync(CancellationToken.None)}");

List<(string Name, int Priority, IJob Job)> Discover() =>
    Assembly.GetExecutingAssembly().GetTypes()
        .Where(t => !t.IsAbstract && t.IsClass && typeof(IJob).IsAssignableFrom(t))
        .Select(t => (Type: t, Meta: t.GetCustomAttribute<JobAttribute>()))
        .Where(x => x.Meta is not null)
        .Select(x => (x.Meta!.Name, x.Meta.Priority, (IJob)Activator.CreateInstance(x.Type)!))
        .OrderBy(x => x.Priority)
        .ThenBy(x => x.Name, StringComparer.Ordinal)
        .ToList();

interface IJob { Task<string> RunAsync(CancellationToken ct); }

[AttributeUsage(AttributeTargets.Class)]
class JobAttribute(string name) : Attribute
{
    public string Name { get; } = name;
    public int Priority { get; init; } = 5;
}

abstract class JobBase : IJob
{
    public abstract Task<string> RunAsync(CancellationToken ct);
}

[Job("cleanup", Priority = 3)]
class CleanupJob : JobBase
{
    public override Task<string> RunAsync(CancellationToken ct) => Task.FromResult("deleted 42 temp files");
}

[Job("backup", Priority = 1)]
class BackupJob : JobBase
{
    public override async Task<string> RunAsync(CancellationToken ct)
    {
        await Task.Delay(10, ct);
        return "copied 1,204 files";
    }
}

[Job("report", Priority = 3)]
class ReportJob : IJob
{
    public Task<string> RunAsync(CancellationToken ct) => Task.FromResult("sent weekly report");
}

class DisabledJob : IJob
{
    public Task<string> RunAsync(CancellationToken ct) => Task.FromResult("should not run");
}

@@exercise A runner with retries, limits and events
Write `JobRunner`:

- constructor `(int maxParallel, int maxAttempts, Func<TimeSpan, CancellationToken, Task> delay)`
- `event EventHandler<JobFinishedEventArgs>? Finished` raised once per job with `Name`, `Attempts` and `Outcome` (`ok: <result>`, `failed: <last error>` or `cancelled`)
- `RunAllAsync(IEnumerable<(string Name, Func<int, CancellationToken, Task<string>> Work)> jobs, CancellationToken ct)`: runs at most `maxParallel` jobs at once; each job's `Work` receives the attempt number (1, 2, ...); an exception triggers a retry after `delay(100ms × 2^(attempt−1))`, up to `maxAttempts`; `OperationCanceledException` stops the job as `cancelled` without retrying

Events may arrive in any order, so `main` collects them and prints them sorted by name, then the recorded delays (sorted).
@@starter
using System.Collections.Concurrent;

var delays = new ConcurrentBag<TimeSpan>();
Func<TimeSpan, CancellationToken, Task> fakeDelay = (d, ct) => { delays.Add(d); return Task.CompletedTask; };
Console.WriteLine("TODO");

class JobFinishedEventArgs(string name, int attempts, string outcome) : EventArgs
{
    public string Name { get; } = name;
    public int Attempts { get; } = attempts;
    public string Outcome { get; } = outcome;
}
@@expected
backup: ok: done after 3 tries (3 attempts)
cleanup: ok: cleaned (1 attempts)
export: cancelled (1 attempts)
report: failed: SMTP 451 (4 attempts)
delays: 100ms 100ms 200ms 200ms 400ms
@@requires
SemaphoreSlim
event EventHandler<JobFinishedEventArgs>
@@hint
Inside the runner, each job: `await gate.WaitAsync(ct)`, then `for (int attempt = 1; ; attempt++) { try { var r = await work(attempt, ct); outcome = $"ok: {r}"; break; } catch (OperationCanceledException) { outcome = "cancelled"; break; } catch (Exception) when (attempt < maxAttempts) { await delay(TimeSpan.FromMilliseconds(100 * (1 << (attempt - 1))), ct); } catch (Exception e) { outcome = $"failed: {e.Message}"; break; } }`. Jobs: backup fails attempts 1–2 (`throw new IOException("busy")`); cleanup succeeds; report always throws `new InvalidOperationException("SMTP 451")`; export throws `OperationCanceledException`. Use 2 in parallel and 4 attempts. Print delays as `{d.TotalMilliseconds}ms`.
@@solution
using System.Collections.Concurrent;

var delays = new ConcurrentBag<TimeSpan>();
Func<TimeSpan, CancellationToken, Task> fakeDelay = (d, ct) => { delays.Add(d); return Task.CompletedTask; };

var runner = new JobRunner(maxParallel: 2, maxAttempts: 4, fakeDelay);
var finished = new ConcurrentBag<JobFinishedEventArgs>();
runner.Finished += (_, e) => finished.Add(e);

(string, Func<int, CancellationToken, Task<string>>)[] jobs =
[
    ("backup", async (attempt, ct) =>
    {
        await Task.Yield();
        if (attempt < 3) throw new IOException("busy");
        return $"done after {attempt} tries";
    }),
    ("cleanup", (attempt, ct) => Task.FromResult("cleaned")),
    ("report", (attempt, ct) => throw new InvalidOperationException("SMTP 451")),
    ("export", (attempt, ct) => throw new OperationCanceledException()),
];
await runner.RunAllAsync(jobs, CancellationToken.None);

foreach (var e in finished.OrderBy(e => e.Name, StringComparer.Ordinal))
    Console.WriteLine($"{e.Name}: {e.Outcome} ({e.Attempts} attempts)");
Console.WriteLine($"delays: {string.Join(" ", delays.Order().Select(d => $"{d.TotalMilliseconds}ms"))}");

class JobFinishedEventArgs(string name, int attempts, string outcome) : EventArgs
{
    public string Name { get; } = name;
    public int Attempts { get; } = attempts;
    public string Outcome { get; } = outcome;
}

class JobRunner(int maxParallel, int maxAttempts, Func<TimeSpan, CancellationToken, Task> delay)
{
    public event EventHandler<JobFinishedEventArgs>? Finished;

    public async Task RunAllAsync(IEnumerable<(string Name, Func<int, CancellationToken, Task<string>> Work)> jobs, CancellationToken ct)
    {
        using var gate = new SemaphoreSlim(maxParallel);
        await Task.WhenAll(jobs.Select(async job =>
        {
            await gate.WaitAsync(ct);
            try
            {
                var (attempts, outcome) = await RunOneAsync(job.Work, ct);
                Finished?.Invoke(this, new JobFinishedEventArgs(job.Name, attempts, outcome));
            }
            finally
            {
                gate.Release();
            }
        }));
    }

    private async Task<(int, string)> RunOneAsync(Func<int, CancellationToken, Task<string>> work, CancellationToken ct)
    {
        for (int attempt = 1; ; attempt++)
        {
            try
            {
                return (attempt, $"ok: {await work(attempt, ct)}");
            }
            catch (OperationCanceledException)
            {
                return (attempt, "cancelled");
            }
            catch (Exception) when (attempt < maxAttempts)
            {
                await delay(TimeSpan.FromMilliseconds(100 * (1 << (attempt - 1))), ct);
            }
            catch (Exception e)
            {
                return (attempt, $"failed: {e.Message}");
            }
        }
    }
}

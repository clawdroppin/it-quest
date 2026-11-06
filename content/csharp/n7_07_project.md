@@lesson n7-project Project: A Worker Service with Graceful Shutdown and Health Checks
@@concepts concurrency, json
@@text
Many .NET jobs in IT are **background services**: queue processors, sync jobs between HR and Active Directory, monitoring agents, report generators. They run as Windows services, systemd units or containers on the generic host.

```csharp,norun
var builder = Host.CreateApplicationBuilder(args);
builder.Services.AddWindowsService();          // or AddSystemd(): integrate with the OS service manager
builder.Services.AddSingleton(Channel.CreateBounded<Job>(1000));
builder.Services.AddHostedService<QueueWorker>();
builder.Services.Configure<HostOptions>(o => o.ShutdownTimeout = TimeSpan.FromSeconds(20));
builder.Build().Run();

public class QueueWorker(Channel<Job> queue, ILogger<QueueWorker> log) : BackgroundService
{
    protected override async Task ExecuteAsync(CancellationToken stoppingToken)
    {
        await foreach (var job in queue.Reader.ReadAllAsync(stoppingToken))
        {
            using var scope = log.BeginScope("job {JobId}", job.Id);
            await ProcessAsync(job, stoppingToken);
        }
    }
}
```

## The host lifecycle

1. `StartAsync` on every `IHostedService`, in registration order. `BackgroundService` starts `ExecuteAsync` without waiting for it.
2. The app runs until it's told to stop (Ctrl+C, SIGTERM from Docker/Kubernetes/systemd, the Windows service manager).
3. `StopAsync` on every service in **reverse** order. `BackgroundService.StopAsync` cancels the `stoppingToken` and waits for `ExecuteAsync`, up to `ShutdownTimeout`.

A service that ignores its token gets abandoned when the timeout expires; in-flight work is lost. Containers get about 30 seconds between SIGTERM and SIGKILL by default.

## Health checks

```csharp,norun
builder.Services.AddHealthChecks()
    .AddCheck("queue", () => queue.Reader.Count < 900 ? HealthCheckResult.Healthy() : HealthCheckResult.Degraded("backlog"))
    .AddNpgSql(connectionString, name: "db");
app.MapHealthChecks("/healthz");      // 200 Healthy/Degraded, 503 Unhealthy
```

Load balancers and orchestrators call these endpoints. Keep checks fast and give each a timeout, because a hanging check makes the whole endpoint hang. Cache results briefly so frequent probes don't hammer the database.

@@quiz
In which order are hosted services stopped?
- Registration order
* Reverse registration order
- Random
- All at once with no order
? Things started last depend on things started first.

@@quiz
What happens to a BackgroundService that ignores its stopping token?
- The host waits forever
* The host stops waiting after ShutdownTimeout and the work is abandoned
- It's restarted
- It throws at startup
? Always pass the token on.

@@quiz
What is a readiness health check for?
* Telling a load balancer or orchestrator whether the service can take work right now
- Measuring CPU temperature
- Restarting the process
- Logging every request
? Liveness asks 'is it alive?'; readiness asks 'can it serve?'.

@@drill
Which base class does a .NET worker service usually inherit from?
= BackgroundService
? Override ExecuteAsync(CancellationToken).

@@flashcards
`IHostedService` :: StartAsync / StopAsync
`BackgroundService` :: ExecuteAsync(stoppingToken)
`ShutdownTimeout` :: how long StopAsync may take
`AddWindowsService` / `AddSystemd` :: OS service integration
`MapHealthChecks` :: health endpoint
Healthy / Degraded / Unhealthy :: health statuses
SIGTERM :: polite stop signal from orchestrators

@@exercise A mini generic host
Write the host lifecycle yourself:

- `interface IHostedService { Task StartAsync(); Task StopAsync(TimeSpan timeout); }`
- `abstract class Worker : IHostedService`: `StartAsync` logs `<name> started` and starts `ExecuteAsync(token)` **without awaiting it**; `StopAsync` cancels the token and waits for `ExecuteAsync` at most `timeout`, logging `<name> did not stop within <ms>ms` if it doesn't finish
- `MiniHost`: `Add(service)`, `StartAsync()` (registration order), `StopAsync(timeout)` (reverse order), plus `OnStopping` callbacks run first when stopping
- `QueueWorker` reads a channel until cancelled, then **drains** what's left with `TryRead` before logging `queue done`; `StuckWorker` ignores cancellation (`await Task.Delay(2000)` without the token); `Metrics` is a plain service logging start/stop

The log is a thread-safe list (lock).
@@starter
using System.Threading.Channels;

var log = new List<string>();
void Log(string s) { lock (log) log.Add(s); }
Console.WriteLine("TODO");

interface IHostedService
{
    Task StartAsync();
    Task StopAsync(TimeSpan timeout);
}
@@expected
metrics started
queue started
stuck started
stuck did not stop within 100ms
processed job1
processed job2
processed job3
processed job4
queue done
metrics stopped
@@hint
In `QueueWorker.ExecuteAsync`: `try { await foreach (var j in reader.ReadAllAsync(ct)) Log($"processed {j}"); } catch (OperationCanceledException) { } while (reader.TryRead(out var j)) Log($"processed {j}"); Log("queue done");`. In `main`: start the host, write four jobs, register `OnStopping(() => writer.Complete())`, then `await host.StopAsync(TimeSpan.FromMilliseconds(100))`. Jobs can be processed while other services are still starting or stopping, so `processed` lines interleave unpredictably. For stable output, print the lifecycle lines first (everything except the queue lines and `metrics stopped`), then the queue lines, then `metrics stopped`.
@@solution
using System.Threading.Channels;

var log = new List<string>();
void Log(string s) { lock (log) log.Add(s); }

var channel = Channel.CreateUnbounded<string>();
var host = new MiniHost();
host.Add(new Metrics(Log));
host.Add(new QueueWorker("queue", channel.Reader, Log));
host.Add(new StuckWorker("stuck", Log));
host.OnStopping(() => channel.Writer.Complete());

await host.StartAsync();
foreach (var j in new[] { "job1", "job2", "job3", "job4" }) await channel.Writer.WriteAsync(j);
await host.StopAsync(TimeSpan.FromMilliseconds(100));

List<string> lines;
lock (log) lines = log.ToList();
var queueLines = lines.Where(l => l.StartsWith("processed") || l == "queue done").ToList();
foreach (var l in lines.Except(queueLines).Where(l => l != "metrics stopped")) Console.WriteLine(l);
foreach (var l in queueLines) Console.WriteLine(l);
if (lines.Contains("metrics stopped")) Console.WriteLine("metrics stopped");

interface IHostedService
{
    Task StartAsync();
    Task StopAsync(TimeSpan timeout);
}

class MiniHost
{
    private readonly List<IHostedService> services = new();
    private readonly List<Action> stopping = new();

    public void Add(IHostedService s) => services.Add(s);
    public void OnStopping(Action a) => stopping.Add(a);

    public async Task StartAsync()
    {
        foreach (var s in services) await s.StartAsync();
    }

    public async Task StopAsync(TimeSpan timeout)
    {
        foreach (var a in stopping) a();
        for (int i = services.Count - 1; i >= 0; i--) await services[i].StopAsync(timeout);
    }
}

abstract class Worker(string name, Action<string> log) : IHostedService
{
    private readonly CancellationTokenSource cts = new();
    private Task? running;
    protected Action<string> Log { get; } = log;

    protected abstract Task ExecuteAsync(CancellationToken ct);

    public Task StartAsync()
    {
        Log($"{name} started");
        running = Task.Run(() => ExecuteAsync(cts.Token));
        return Task.CompletedTask;
    }

    public async Task StopAsync(TimeSpan timeout)
    {
        cts.Cancel();
        if (running is null) return;
        var finished = await Task.WhenAny(running, Task.Delay(timeout));
        if (finished != running) Log($"{name} did not stop within {timeout.TotalMilliseconds}ms");
    }
}

class QueueWorker(string name, ChannelReader<string> reader, Action<string> log) : Worker(name, log)
{
    protected override async Task ExecuteAsync(CancellationToken ct)
    {
        try
        {
            await foreach (var j in reader.ReadAllAsync(ct)) Log($"processed {j}");
        }
        catch (OperationCanceledException)
        {
        }
        while (reader.TryRead(out var j)) Log($"processed {j}");
        Log("queue done");
    }
}

class StuckWorker(string name, Action<string> log) : Worker(name, log)
{
    protected override async Task ExecuteAsync(CancellationToken ct) => await Task.Delay(2000);
}

class Metrics(Action<string> log) : IHostedService
{
    public Task StartAsync()
    {
        log("metrics started");
        return Task.CompletedTask;
    }

    public Task StopAsync(TimeSpan timeout)
    {
        log("metrics stopped");
        return Task.CompletedTask;
    }
}

@@exercise A health endpoint with timeouts and caching
Write `HealthService` for a `/healthz` endpoint:

- constructor `(TimeProvider clock, TimeSpan cacheFor, TimeSpan checkTimeout)`; `Add(name, Func<CancellationToken, Task<Health>> check)`
- `record Health(string Status, string Description)` with statuses `Healthy`, `Degraded`, `Unhealthy`
- `CheckAsync()` runs all checks **concurrently**, each with its own timeout (a timed-out check is `Unhealthy` / `timed out`, an exception is `Unhealthy` / its message); overall status is the worst one
- returns `(int Code, string Json)`: 200 for Healthy/Degraded, 503 for Unhealthy; JSON `{"status":...,"checks":{"name":{"status":...,"description":...}}}` with checks in registration order
- results are cached for `cacheFor` (by the clock): calls within that window return the cached response without running checks
@@starter
using System.Text.Json;

Console.WriteLine("TODO");

record Health(string Status, string Description);

class ManualClock(DateTimeOffset start) : TimeProvider
{
    private DateTimeOffset now = start;
    public override DateTimeOffset GetUtcNow() => now;
    public void Advance(TimeSpan by) => now += by;
}
@@expected
200 {"status":"Degraded","checks":{"db":{"status":"Healthy","description":"12 ms"},"queue":{"status":"Degraded","description":"backlog 950"},"ldap":{"status":"Healthy","description":"bind ok"}}}
db checks run: 1
cached: 200, db checks run: 1
503 {"status":"Unhealthy","checks":{"db":{"status":"Healthy","description":"12 ms"},"queue":{"status":"Degraded","description":"backlog 950"},"ldap":{"status":"Unhealthy","description":"timed out"}}}
db checks run: 2
@@requires
TimeProvider
Task.WhenAll
@@hint
Build the JSON with a `Dictionary<string, object>` (insertion order is kept) and `JsonSerializer.Serialize` using `JsonSerializerDefaults.Web` (camelCase for the record properties). A timeout per check: `using var cts = new CancellationTokenSource(checkTimeout); try { return await check(cts.Token); } catch (OperationCanceledException) { ... }`. In `main`: the db check counts runs and returns `Healthy`/`12 ms`; queue returns `Degraded`/`backlog 950`; ldap first returns `bind ok`, but after you set a flag it does `await Task.Delay(5000, ct)`. Call once, call again after 2 s (cached), set the flag, advance 10 s, call again. Use a 5 s cache and a 100 ms timeout.
@@solution
using System.Text.Json;

int dbRuns = 0;
bool ldapHangs = false;
var clock = new ManualClock(new DateTimeOffset(2026, 11, 3, 9, 0, 0, TimeSpan.Zero));
var health = new HealthService(clock, TimeSpan.FromSeconds(5), TimeSpan.FromMilliseconds(100));
health.Add("db", async ct => { dbRuns++; await Task.Delay(5, ct); return new Health("Healthy", "12 ms"); });
health.Add("queue", ct => Task.FromResult(new Health("Degraded", "backlog 950")));
health.Add("ldap", async ct =>
{
    if (ldapHangs) await Task.Delay(5000, ct);
    return new Health("Healthy", "bind ok");
});

var (code, json) = await health.CheckAsync();
Console.WriteLine($"{code} {json}");
Console.WriteLine($"db checks run: {dbRuns}");
clock.Advance(TimeSpan.FromSeconds(2));
(code, _) = await health.CheckAsync();
Console.WriteLine($"cached: {code}, db checks run: {dbRuns}");
ldapHangs = true;
clock.Advance(TimeSpan.FromSeconds(10));
(code, json) = await health.CheckAsync();
Console.WriteLine($"{code} {json}");
Console.WriteLine($"db checks run: {dbRuns}");

record Health(string Status, string Description);

class ManualClock(DateTimeOffset start) : TimeProvider
{
    private DateTimeOffset now = start;
    public override DateTimeOffset GetUtcNow() => now;
    public void Advance(TimeSpan by) => now += by;
}

class HealthService(TimeProvider clock, TimeSpan cacheFor, TimeSpan checkTimeout)
{
    private static readonly JsonSerializerOptions Json = new(JsonSerializerDefaults.Web);
    private static readonly string[] Order = ["Healthy", "Degraded", "Unhealthy"];
    private readonly List<(string Name, Func<CancellationToken, Task<Health>> Check)> checks = new();
    private (DateTimeOffset At, int Code, string Json)? cached;

    public void Add(string name, Func<CancellationToken, Task<Health>> check) => checks.Add((name, check));

    public async Task<(int Code, string Json)> CheckAsync()
    {
        var now = clock.GetUtcNow();
        if (cached is { } c && now - c.At < cacheFor) return (c.Code, c.Json);

        var results = await Task.WhenAll(checks.Select(async x => (x.Name, Result: await RunOne(x.Check))));
        string overall = Order[results.Max(r => Array.IndexOf(Order, r.Result.Status))];
        var body = new Dictionary<string, object>
        {
            ["status"] = overall,
            ["checks"] = results.ToDictionary(r => r.Name, r => (object)r.Result),
        };
        int code = overall == "Unhealthy" ? 503 : 200;
        string json = JsonSerializer.Serialize(body, Json);
        cached = (now, code, json);
        return (code, json);
    }

    private async Task<Health> RunOne(Func<CancellationToken, Task<Health>> check)
    {
        using var cts = new CancellationTokenSource(checkTimeout);
        try
        {
            return await check(cts.Token);
        }
        catch (OperationCanceledException)
        {
            return new Health("Unhealthy", "timed out");
        }
        catch (Exception e)
        {
            return new Health("Unhealthy", e.Message);
        }
    }
}

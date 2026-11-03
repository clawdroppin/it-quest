@@lesson e6-delegates Delegates, Lambdas, Closures and Events
@@concepts functions
@@text
A **delegate** is a type for "a method with this signature". It lets you pass behaviour around like data, and it's what LINQ, events, ASP.NET Core middleware and `Task.Run` are built on.

## Func, Action and lambdas

```csharp
Func<int, int, int> add = (a, b) => a + b;          // takes two ints, returns int
Action<string> log = msg => Console.WriteLine($"[log] {msg}");
Predicate<string> isPrivate = ip => ip.StartsWith("10.") || ip.StartsWith("192.168.");
Func<string, string> shout = s => s.ToUpper();

log($"2 + 3 = {add(2, 3)}");
Console.WriteLine(isPrivate("10.1.2.3"));
Func<string, string> intern = string.Intern;         // a method group can also be a delegate value
Console.WriteLine(Apply("web01", shout, s => s + "!"));

string Apply(string input, params Func<string, string>[] steps)
{
    foreach (var step in steps) input = step(input);
    return input;
}
```

`Func<..., TResult>` returns a value, `Action<...>` returns nothing. You can still declare your own named delegate types (`delegate bool Validator(string s);`), which is useful for documentation.

## Closures

A lambda can use variables from the surrounding scope. It captures the **variable**, not its current value:

```csharp
int counter = 0;
Action increment = () => counter++;
increment();
increment();
Console.WriteLine(counter);   // 2: the lambda changed the outer variable

var actions = new List<Func<int>>();
for (int i = 0; i < 3; i++)
{
    int copy = i;             // a for loop shares one i; copy it per iteration
    actions.Add(() => copy);
}
Console.WriteLine(string.Join(",", actions.Select(a => a())));   // 0,1,2
```

`foreach` gives each iteration its own variable, but a `for` loop doesn't, which is a classic bug when lambdas run later (for example in a `Task`).

## Multicast and events

Delegates can hold several methods (`+=`). An **event** is a delegate field that outside code can only subscribe to (`+=`) and unsubscribe from (`-=`). Only the owning class can raise it:

```csharp
var backup = new BackupJob();
backup.Completed += (sender, e) => Console.WriteLine($"mail: backup done, {e.Files} files");
backup.Completed += (sender, e) => Console.WriteLine($"metrics: files={e.Files}");
backup.Run(1234);

class BackupJob
{
    public event EventHandler<BackupEventArgs>? Completed;   // standard pattern: (sender, args)

    public void Run(int files)
    {
        // ... copy files ...
        Completed?.Invoke(this, new BackupEventArgs(files)); // ?. : nobody may be subscribed
    }
}

class BackupEventArgs(int files) : EventArgs { public int Files { get; } = files; }
```

## Event pitfalls

- A subscriber stays alive as long as the publisher does. A long-lived service holding events from short-lived windows is the classic .NET memory leak: **unsubscribe** (`-=`) when you're done.
- Exceptions in one handler stop the rest from running.
- Events run synchronously on the raising thread.

UI frameworks (WPF `Click`, `PropertyChanged`), `FileSystemWatcher.Changed` and `Process.Exited` are all events.

@@quiz
What's the difference between `Func<string, bool>` and `Action<string>`?
* Func returns a value (bool); Action returns nothing
- Action is asynchronous
- Func can't take parameters
- None
? The last type argument of Func is the return type.

@@quiz
What does a lambda capture from the surrounding scope?
- A copy of the value at creation time
* The variable itself, so later changes are visible
- Nothing
- Only constants
? That's why `for` loop variables cause surprises.

@@quiz
Why use `event` instead of a public delegate field?
- Events are faster
* Outside code can only subscribe and unsubscribe; it can't raise the event or replace all subscribers
- Delegates can't have several targets
- Events are asynchronous
? An event protects the invocation list.

@@flashcards
Delegate :: type-safe reference to a method
`Func<T, TResult>` :: delegate that returns a value
`Action<T>` :: delegate returning void
`Predicate<T>` :: `Func<T, bool>`
Lambda :: `x => x * 2`
Closure :: lambda capturing outer variables
Multicast :: several methods in one delegate (`+=`)
`event` :: delegate only the owner can raise
`EventHandler<TArgs>` :: standard (sender, args) signature
`?.Invoke` :: raise only if subscribed

@@exercise A middleware pipeline
ASP.NET Core's request pipeline is built from delegates. Build a mini version:

- `delegate string Handler(Request req)`; `record Request(string Path, string? User)`
- a **middleware** is `Func<Handler, Handler>`: it receives the next handler and returns a new one
- `Build(Handler endpoint, params Func<Handler, Handler>[] middleware)`: the **first** middleware in the list is the outermost (runs first)
- write `Logging(List<string> log)` (logs `-> path` before and `<- path` after), `Auth` (returns `401` when `User` is null, without calling next) and `Timing(Func<int> clock)` (appends ` [Nms]` to the response, using a fake clock)
@@starter
var log = new List<string>();
int now = 0;
Func<int> clock = () => now += 5;
Console.WriteLine("TODO");

delegate string Handler(Request req);
record Request(string Path, string? User);
@@expected
200 hello ana [5ms]
401
-> /home
<- /home
-> /admin
<- /admin
@@hint
`Handler Build(Handler endpoint, params Func<Handler, Handler>[] mw)`: loop **backwards** over `mw`, wrapping: `h = mw[i](h)`. Middleware shape: `next => req => { ...; var res = next(req); ...; return res; }`. The endpoint returns `$"200 hello {req.User}"`. Order: `Build(endpoint, Logging(log), Auth, Timing(clock))`, so a 401 never reaches Timing. The fake clock advances 5 each call (two calls per timed request).
@@solution
var log = new List<string>();
int now = 0;
Func<int> clock = () => now += 5;

Handler endpoint = req => $"200 hello {req.User}";
var app = Build(endpoint, Logging(log), Auth, Timing(clock));
Console.WriteLine(app(new Request("/home", "ana")));
Console.WriteLine(app(new Request("/admin", null)));
foreach (var line in log) Console.WriteLine(line);

Handler Build(Handler endpoint, params Func<Handler, Handler>[] middleware)
{
    var h = endpoint;
    for (int i = middleware.Length - 1; i >= 0; i--) h = middleware[i](h);
    return h;
}

Func<Handler, Handler> Logging(List<string> log) => next => req =>
{
    log.Add($"-> {req.Path}");
    var res = next(req);
    log.Add($"<- {req.Path}");
    return res;
};

Handler Auth(Handler next) => req => req.User is null ? "401" : next(req);

Func<Handler, Handler> Timing(Func<int> clock) => next => req =>
{
    int start = clock();
    var res = next(req);
    return $"{res} [{clock() - start}ms]";
};

delegate string Handler(Request req);
record Request(string Path, string? User);

@@exercise Threshold events
Write a `Sensor` that raises events when readings **cross** a threshold (not on every high reading):

- `event EventHandler<ThresholdEventArgs>? Exceeded` and `event EventHandler<ThresholdEventArgs>? Recovered`; `ThresholdEventArgs` has `Value` and `Threshold`
- `Report(int value)`: raise `Exceeded` when going from ≤ threshold to above it, `Recovered` when going back to ≤ threshold
- in `main`: a logger subscribes to both events for the whole run; a "pager" subscribes to `Exceeded` and **unsubscribes itself** after the first page (it needs a reference to its own handler)
@@starter
var sensor = new Sensor("cpu", 90);
Console.WriteLine("TODO");

class Sensor(string name, int threshold)
{
    public string Name { get; } = name;
    public int Threshold { get; } = threshold;
}
@@expected
log: cpu exceeded 95 > 90
PAGE: cpu at 95
log: cpu recovered 70 <= 90
log: cpu exceeded 99 > 90
log: cpu recovered 90 <= 90
pager subscribed: False
@@hint
Keep `bool above` in the sensor. The pager: `EventHandler<ThresholdEventArgs>? pager = null; pager = (s, e) => { Console.WriteLine(...); sensor.Exceeded -= pager; };` then `sensor.Exceeded += pager;`. For the last line, expose the number of `Exceeded` subscribers (`Exceeded?.GetInvocationList().Length ?? 0`): once the pager has left, only the logger remains. Readings: 50, 95, 97, 70, 99, 90.
@@solution
var sensor = new Sensor("cpu", 90);
sensor.Exceeded += (s, e) => Console.WriteLine($"log: {sensor.Name} exceeded {e.Value} > {e.Threshold}");
sensor.Recovered += (s, e) => Console.WriteLine($"log: {sensor.Name} recovered {e.Value} <= {e.Threshold}");

EventHandler<ThresholdEventArgs>? pager = null;
pager = (s, e) =>
{
    Console.WriteLine($"PAGE: {sensor.Name} at {e.Value}");
    sensor.Exceeded -= pager;
};
sensor.Exceeded += pager;

foreach (var v in new[] { 50, 95, 97, 70, 99, 90 }) sensor.Report(v);
Console.WriteLine($"pager subscribed: {sensor.SubscriberCount > 1}");

class ThresholdEventArgs(int value, int threshold) : EventArgs
{
    public int Value { get; } = value;
    public int Threshold { get; } = threshold;
}

class Sensor(string name, int threshold)
{
    private bool above;
    public string Name { get; } = name;
    public int Threshold { get; } = threshold;

    public event EventHandler<ThresholdEventArgs>? Exceeded;
    public event EventHandler<ThresholdEventArgs>? Recovered;

    public int SubscriberCount => Exceeded?.GetInvocationList().Length ?? 0;

    public void Report(int value)
    {
        bool nowAbove = value > Threshold;
        if (nowAbove && !above) Exceeded?.Invoke(this, new ThresholdEventArgs(value, Threshold));
        if (!nowAbove && above) Recovered?.Invoke(this, new ThresholdEventArgs(value, Threshold));
        above = nowAbove;
    }
}

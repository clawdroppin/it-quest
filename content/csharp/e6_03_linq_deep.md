@@lesson e6-linq LINQ in Depth: Deferred Execution, Joins, Grouping and Custom Operators
@@concepts lists, functions
@@text
## Deferred execution

Most LINQ operators don't run when you call them. They build a **query** that runs when you enumerate it (`foreach`, `ToList`, `Count`, `First`...):

```csharp
var hosts = new List<string> { "web01", "db01" };
var webs = hosts.Where(h => { Console.WriteLine($"  checking {h}"); return h.StartsWith("web"); });
hosts.Add("web02");                       // added AFTER the query was defined...
Console.WriteLine("first run:");
Console.WriteLine(string.Join(",", webs)); // ...and still included
Console.WriteLine("second run:");
Console.WriteLine(webs.Count());           // the whole query runs again!
```

Consequences:

- The query sees the data **as it is when enumerated**.
- Enumerating twice does the work twice: an expensive query, or one reading a file or database, should be materialized once with `ToList()`.
- Exceptions inside a lambda appear at enumeration time, not where the query was written.

## Grouping, joining and flattening

```csharp
var users = new[] { new User(1, "ana", "IT"), new User(2, "kenji", "Sales"), new User(3, "mei", "IT") };
var logins = new[] { new Login(1, true), new Login(1, false), new Login(3, false), new Login(9, true) };

foreach (var g in users.GroupBy(u => u.Dept))
    Console.WriteLine($"{g.Key}: {string.Join(",", g.Select(u => u.Name))}");

var joined = users.Join(logins, u => u.Id, l => l.UserId, (u, l) => $"{u.Name}:{(l.Ok ? "ok" : "fail")}");
Console.WriteLine(string.Join(" ", joined));                 // inner join: user 9 and kenji vanish

var perUser = users.GroupJoin(logins, u => u.Id, l => l.UserId,
    (u, ls) => $"{u.Name}={ls.Count()}");                      // left join: kenji=0 stays
Console.WriteLine(string.Join(" ", perUser));

var tags = new[] { "web,prod", "db,prod,pci" }.SelectMany(s => s.Split(','));
Console.WriteLine(string.Join(" ", tags.Distinct()));

record User(int Id, string Name, string Dept);
record Login(int UserId, bool Ok);
```

## The operator toolbox

| Goal | Operators |
|---|---|
| filter / transform | `Where`, `Select`, `SelectMany`, `OfType<T>` |
| order | `OrderBy`, `ThenByDescending`, `Order()` (.NET 7) |
| sets | `Distinct`, `DistinctBy`, `Union`, `Intersect`, `Except`, `ExceptBy` |
| group / lookup | `GroupBy`, `ToLookup`, `CountBy` and `AggregateBy` (.NET 9) |
| aggregate | `Count`, `Sum`, `Average`, `Min`/`Max`, `MinBy`/`MaxBy`, `Aggregate` |
| slicing | `Take`, `Skip`, `TakeWhile`, `SkipLast`, `Chunk(n)` |
| pairing | `Zip`, `Index()` (.NET 9) |
| materialize | `ToList`, `ToArray`, `ToDictionary`, `ToHashSet` |

## Writing your own operators

An extension method plus `yield return` gives you a lazy operator that composes with the rest:

```csharp
foreach (var batch in Enumerable.Range(1, 7).Batches(3))
    Console.WriteLine(string.Join(",", batch));

static class MyLinq
{
    public static IEnumerable<List<T>> Batches<T>(this IEnumerable<T> source, int size)
    {
        var batch = new List<T>(size);
        foreach (var item in source)
        {
            batch.Add(item);
            if (batch.Count == size)
            {
                yield return batch;
                batch = new List<T>(size);
            }
        }
        if (batch.Count > 0) yield return batch;
    }
}
```

## IEnumerable vs IQueryable

`IEnumerable<T>` queries run in your process with compiled lambdas. `IQueryable<T>` (Entity Framework) holds **expression trees** that the provider translates into SQL. Same syntax, but a method the database can't translate fails at run time, and accidentally switching to `IEnumerable` (e.g. calling `.AsEnumerable()` too early) downloads the whole table and filters it in memory.

@@quiz
When does `var q = list.Where(x => x > 5);` run the filter?
- Immediately
* When q is enumerated (foreach, ToList, Count...), and again on every enumeration
- Never
- Only once, the first time
? Deferred execution.

@@quiz
Which operator keeps users who have no logins (a left join)?
- Join
* GroupJoin
- Zip
- Intersect
? Join is an inner join.

@@quiz
Why might `db.Orders.AsEnumerable().Where(o => o.Total > 1000)` be slow?
* AsEnumerable switches to in-memory LINQ, so every order is loaded from the database before filtering
- Where is always slow
- AsEnumerable sorts the data
- It isn't slow
? Keep queries IQueryable until the filtering is done.

@@flashcards
Deferred execution :: query runs when enumerated
`ToList()` :: materialize once
`SelectMany` :: flatten nested sequences
`GroupBy` :: groups with a Key
`Join` :: inner join
`GroupJoin` :: left-join shape
`ToLookup` :: immediate, reusable grouping
`MaxBy(x => x.Score)` :: element with the largest key
`Chunk(n)` :: split into arrays of n
`yield return` :: lazy custom iterator
`IQueryable` :: expression trees for providers like EF

@@exercise A login report
Answer questions about logins with LINQ (one query per question, no loops except for printing):

1. failed logins per department, **including** departments with zero failures, sorted by failures descending then department name
2. users with 2 or more failed logins in a row (consecutive in time order for that user), as `name (n)` with their longest run
3. the busiest hour (most logins of any kind), as `HH:00 (n logins)`; on ties, the earliest hour
4. user ids in the log that don't exist in the user list (`ExceptBy` or `Except`), sorted
@@starter
var users = new List<User>
{
    new(1, "ana", "IT"), new(2, "kenji", "Sales"), new(3, "mei", "IT"), new(4, "yui", "HR"),
};
var logins = new List<Login>
{
    new(1, new DateTime(2026, 11, 3, 8, 55, 0), true),
    new(3, new DateTime(2026, 11, 3, 9, 1, 0), false),
    new(3, new DateTime(2026, 11, 3, 9, 2, 0), false),
    new(3, new DateTime(2026, 11, 3, 9, 3, 0), false),
    new(2, new DateTime(2026, 11, 3, 9, 15, 0), false),
    new(2, new DateTime(2026, 11, 3, 9, 20, 0), true),
    new(2, new DateTime(2026, 11, 3, 10, 5, 0), false),
    new(7, new DateTime(2026, 11, 3, 10, 6, 0), false),
    new(1, new DateTime(2026, 11, 3, 10, 30, 0), false),
    new(5, new DateTime(2026, 11, 3, 11, 0, 0), true),
};
Console.WriteLine("TODO");

record User(int Id, string Name, string Dept);
record Login(int UserId, DateTime At, bool Ok);
@@expected
failures by department:
  IT 4
  Sales 2
  HR 0
repeated failures:
  mei (3)
busiest hour: 09:00 (5 logins)
unknown user ids: 5, 7
@@hint
1: `users.GroupBy(u => u.Dept).Select(g => (Dept: g.Key, Fails: logins.Count(l => !l.Ok && g.Any(u => u.Id == l.UserId))))`, then `OrderByDescending(x => x.Fails).ThenBy(x => x.Dept)`. 2: for each user, the longest run: `ordered.Aggregate((best: 0, cur: 0), (acc, l) => l.Ok ? (acc.best, 0) : (Math.Max(acc.best, acc.cur + 1), acc.cur + 1)).best`. Kenji's failures aren't consecutive. 3: `GroupBy(l => l.At.Hour)`, order by count desc then hour. 4: `logins.Select(l => l.UserId).Distinct().Except(users.Select(u => u.Id)).Order()`.
@@solution
var users = new List<User>
{
    new(1, "ana", "IT"), new(2, "kenji", "Sales"), new(3, "mei", "IT"), new(4, "yui", "HR"),
};
var logins = new List<Login>
{
    new(1, new DateTime(2026, 11, 3, 8, 55, 0), true),
    new(3, new DateTime(2026, 11, 3, 9, 1, 0), false),
    new(3, new DateTime(2026, 11, 3, 9, 2, 0), false),
    new(3, new DateTime(2026, 11, 3, 9, 3, 0), false),
    new(2, new DateTime(2026, 11, 3, 9, 15, 0), false),
    new(2, new DateTime(2026, 11, 3, 9, 20, 0), true),
    new(2, new DateTime(2026, 11, 3, 10, 5, 0), false),
    new(7, new DateTime(2026, 11, 3, 10, 6, 0), false),
    new(1, new DateTime(2026, 11, 3, 10, 30, 0), false),
    new(5, new DateTime(2026, 11, 3, 11, 0, 0), true),
};

var byDept = users
    .GroupBy(u => u.Dept)
    .Select(g => (Dept: g.Key, Fails: logins.Count(l => !l.Ok && g.Any(u => u.Id == l.UserId))))
    .OrderByDescending(x => x.Fails)
    .ThenBy(x => x.Dept, StringComparer.Ordinal);
Console.WriteLine("failures by department:");
foreach (var (dept, fails) in byDept) Console.WriteLine($"  {dept} {fails}");

var repeated = users
    .Select(u => (u.Name, Run: logins
        .Where(l => l.UserId == u.Id)
        .OrderBy(l => l.At)
        .Aggregate((best: 0, cur: 0), (acc, l) => l.Ok ? (acc.best, 0) : (Math.Max(acc.best, acc.cur + 1), acc.cur + 1))
        .best))
    .Where(x => x.Run >= 2);
Console.WriteLine("repeated failures:");
foreach (var (name, run) in repeated) Console.WriteLine($"  {name} ({run})");

var busiest = logins
    .GroupBy(l => l.At.Hour)
    .OrderByDescending(g => g.Count())
    .ThenBy(g => g.Key)
    .First();
Console.WriteLine($"busiest hour: {busiest.Key:00}:00 ({busiest.Count()} logins)");

var unknown = logins.Select(l => l.UserId).Distinct().Except(users.Select(u => u.Id)).Order();
Console.WriteLine($"unknown user ids: {string.Join(", ", unknown)}");

record User(int Id, string Name, string Dept);
record Login(int UserId, DateTime At, bool Ok);

@@exercise Lazy custom operators
Write three lazy LINQ operators as extension methods using `yield return`:

- `Window<T>(int size)`: sliding windows (`[1,2,3]`, `[2,3,4]`, ...), each a new array
- `RunLengths<T>()`: consecutive equal items as `(T Item, int Count)` tuples
- `TakeUntil<T>(Func<T, bool> stop)`: items up to **and including** the first one matching `stop`

The source in `main` counts how many items were pulled from it, which proves the operators are lazy: taking the first 2 windows must read only 4 items.
@@starter
int pulled = 0;
IEnumerable<int> Source(params int[] xs)
{
    foreach (var x in xs)
    {
        pulled++;
        yield return x;
    }
}
Console.WriteLine("TODO");

static class MyLinq
{
}
@@expected
windows: [1,2,3] [2,3,4]
pulled: 4
runs: up x3, down x2, up x1
until 500: 200 200 404 500
pulled: 4
moving average > 300: [200,404,500]
@@hint
`Window`: keep a `Queue<T>`; after enqueueing, dequeue if it's larger than `size`, and `yield return q.ToArray()` once it's full. `RunLengths`: track the current item and count with `EqualityComparer<T>.Default.Equals`; yield the final run after the loop. `TakeUntil`: `yield return item; if (stop(item)) yield break;`. In `main`: `Source(1,2,3,4,5,6).Window(3).Take(2)`; reset `pulled`; `TakeUntil(c => c >= 500)` on `200, 200, 404, 500, 200, 503`; the last line uses `Window(3)` over `200, 200, 404, 500` and keeps windows whose average is above 300.
@@solution
int pulled = 0;
IEnumerable<int> Source(params int[] xs)
{
    foreach (var x in xs)
    {
        pulled++;
        yield return x;
    }
}

string Show(int[] w) => "[" + string.Join(",", w) + "]";

var windows = Source(1, 2, 3, 4, 5, 6).Window(3).Take(2).ToList();
Console.WriteLine($"windows: {string.Join(" ", windows.Select(Show))}");
Console.WriteLine($"pulled: {pulled}");

var states = new[] { "up", "up", "up", "down", "down", "up" };
Console.WriteLine($"runs: {string.Join(", ", states.RunLengths().Select(r => $"{r.Item} x{r.Count}"))}");

pulled = 0;
var codes = Source(200, 200, 404, 500, 200, 503).TakeUntil(c => c >= 500).ToList();
Console.WriteLine($"until 500: {string.Join(" ", codes)}");
Console.WriteLine($"pulled: {pulled}");

var hot = codes.Window(3).Where(w => w.Average() > 300);
Console.WriteLine($"moving average > 300: {string.Join(" ", hot.Select(Show))}");

static class MyLinq
{
    public static IEnumerable<T[]> Window<T>(this IEnumerable<T> source, int size)
    {
        var q = new Queue<T>(size);
        foreach (var item in source)
        {
            q.Enqueue(item);
            if (q.Count > size) q.Dequeue();
            if (q.Count == size) yield return q.ToArray();
        }
    }

    public static IEnumerable<(T Item, int Count)> RunLengths<T>(this IEnumerable<T> source)
    {
        var cmp = EqualityComparer<T>.Default;
        bool any = false;
        T current = default!;
        int count = 0;
        foreach (var item in source)
        {
            if (any && cmp.Equals(item, current))
            {
                count++;
                continue;
            }
            if (any) yield return (current, count);
            current = item;
            count = 1;
            any = true;
        }
        if (any) yield return (current, count);
    }

    public static IEnumerable<T> TakeUntil<T>(this IEnumerable<T> source, Func<T, bool> stop)
    {
        foreach (var item in source)
        {
            yield return item;
            if (stop(item)) yield break;
        }
    }
}

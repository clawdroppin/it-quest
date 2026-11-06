@@lesson c2-generics Generics and Choosing the Right Collection
@@concepts lists, maps
@@text
## Generic types and methods

Generics let one piece of code work with many types while keeping full type safety (and no boxing):

```csharp
var ids = new Box<int>(42);
var name = new Box<string>("web01");
Console.WriteLine($"{ids.Value + 1} {name.Value.ToUpper()}");
Console.WriteLine(Largest(new[] { 3, 9, 4 }));
Console.WriteLine(Largest(new[] { "b", "c", "a" }));

T Largest<T>(IEnumerable<T> items) where T : IComparable<T>
{
    T best = items.First();
    foreach (var x in items)
        if (x.CompareTo(best) > 0) best = x;
    return best;
}

class Box<T>(T value) { public T Value { get; } = value; }
```

## Constraints

| Constraint | Means |
|---|---|
| `where T : class` / `struct` | reference / value type |
| `where T : new()` | has a parameterless constructor |
| `where T : IComparable<T>` | implements an interface |
| `where T : Entity` | derives from a base class |
| `where T : notnull` | non-nullable type |
| `where T : INumber<T>` | any number type (generic math, .NET 7+) |

## Generic math

```csharp
using System.Numerics;

Console.WriteLine(Sum(new[] { 1, 2, 3 }));
Console.WriteLine(Sum(new[] { 1.5m, 2.25m }));

T Sum<T>(IEnumerable<T> xs) where T : INumber<T>
{
    T total = T.Zero;
    foreach (var x in xs) total += x;
    return total;
}
```

## Variance in one paragraph

`IEnumerable<out T>` is **covariant**: an `IEnumerable<string>` can be used where an `IEnumerable<object>` is expected, because you only read from it. `List<T>` is invariant (you could add a wrong type). `Action<in T>` is **contravariant**. You'll mostly notice variance when an assignment between generic types surprisingly works or fails.

## The collection toolbox

| Need | Collection | Lookup / insert |
|---|---|---|
| ordered list, index access | `List<T>` | O(1) index, O(n) search |
| key → value | `Dictionary<K,V>` | O(1) |
| unique items | `HashSet<T>` | O(1) |
| sorted keys / items | `SortedDictionary<K,V>`, `SortedSet<T>` | O(log n) |
| first in, first out | `Queue<T>` | O(1) |
| last in, first out | `Stack<T>` | O(1) |
| always take the smallest priority | `PriorityQueue<T,P>` | O(log n) |
| cheap insert/remove in the middle | `LinkedList<T>` | O(1) with a node |
| many threads | `ConcurrentDictionary`, `ConcurrentQueue`, `Channel<T>` | |
| build once, read a lot | `FrozenDictionary`, `FrozenSet` (.NET 8) | fastest reads |
| never changes | `ImmutableList`, `ImmutableDictionary` | safe sharing |

```csharp
var queue = new PriorityQueue<string, int>();
queue.Enqueue("disk 85%", 3);
queue.Enqueue("server down", 1);
queue.Enqueue("cert expires in 20 days", 5);
while (queue.TryDequeue(out var alert, out var prio))
    Console.WriteLine($"P{prio} {alert}");

var seen = new HashSet<string>();
foreach (var ip in new[] { "10.0.0.1", "10.0.0.2", "10.0.0.1" })
    if (!seen.Add(ip)) Console.WriteLine($"duplicate {ip}");
```

Expose the **least capable interface** that callers need: return `IReadOnlyList<T>` or `IEnumerable<T>` instead of `List<T>` so callers can't modify your internal state.

@@quiz
Which collection always gives you the most urgent item next?
- Queue
- Stack
* PriorityQueue
- SortedList
? Lower priority values come out first.

@@quiz
What does `where T : IComparable<T>` allow inside the method?
* Calling `CompareTo` on T values
- Creating new T objects
- Assigning null to T
- Nothing
? Constraints unlock members you can use.

@@quiz
Why return `IReadOnlyList<T>` from a class instead of its internal `List<T>`?
- It's faster
* Callers can read but not modify your internal state
- It's required by LINQ
- It sorts the list
? Encapsulation through interfaces.

@@drill
Which collection gives fast lookups of unique values?
= HashSet
= HashSet<T>
? Contains is O(1) on average.

@@flashcards
Generic type :: `class Box<T>`
Generic method :: `T Largest<T>(...)`
`where T : new()` :: has a parameterless constructor
`INumber<T>` :: generic math constraint
Covariance :: `IEnumerable<string>` as `IEnumerable<object>`
`HashSet<T>.Add` :: returns false for duplicates
`PriorityQueue<T,P>` :: smallest priority first
`FrozenDictionary` :: read-optimized dictionary (.NET 8)
`IReadOnlyList<T>` :: read-only view

@@exercise A generic LRU cache
Implement `LruCache<TKey, TValue>` where `TKey : notnull`, with a capacity:

- `TryGet(key, out value)`: a hit marks the entry as most recently used
- `Put(key, value)`: insert or update (also marks it most recent); when over capacity, evict the **least** recently used entry and return its key (otherwise `default`)
- `Keys`: from most to least recently used

Use a `Dictionary<TKey, LinkedListNode<(TKey Key, TValue Value)>>` plus a `LinkedList` so every operation is O(1).
@@starter
var cache = new LruCache<string, string>(3);
Console.WriteLine("TODO");

class LruCache<TKey, TValue>(int capacity) where TKey : notnull
{
}
@@expected
put web01, web02, web03 -> web03,web02,web01
get web01: 10.0.0.1 -> web01,web03,web02
put db01 evicts web02 -> db01,web01,web03
get web02: miss
put web03 (update) -> web03,db01,web01
put cache01 evicts web01 -> cache01,web03,db01
web03 = 10.0.0.33
@@hint
On a hit: `list.Remove(node); list.AddFirst(node);`. On `Put` of an existing key, replace `node.Value` and move it to the front. When `map.Count > capacity`, take `list.Last`, remove it from both. `Keys => list.Select(n => n.Key)`. Values: `web0N` → `10.0.0.N`, `db01` → `10.0.0.10`, `cache01` → `10.0.0.20`; the update sets web03 to `10.0.0.33`.
@@solution
var cache = new LruCache<string, string>(3);
cache.Put("web01", "10.0.0.1");
cache.Put("web02", "10.0.0.2");
cache.Put("web03", "10.0.0.3");
Console.WriteLine($"put web01, web02, web03 -> {Keys()}");
cache.TryGet("web01", out var ip);
Console.WriteLine($"get web01: {ip} -> {Keys()}");
var evicted = cache.Put("db01", "10.0.0.10");
Console.WriteLine($"put db01 evicts {evicted} -> {Keys()}");
Console.WriteLine($"get web02: {(cache.TryGet("web02", out _) ? "hit" : "miss")}");
cache.Put("web03", "10.0.0.33");
Console.WriteLine($"put web03 (update) -> {Keys()}");
evicted = cache.Put("cache01", "10.0.0.20");
Console.WriteLine($"put cache01 evicts {evicted} -> {Keys()}");
cache.TryGet("web03", out ip);
Console.WriteLine($"web03 = {ip}");

string Keys() => string.Join(",", cache.Keys);

class LruCache<TKey, TValue>(int capacity) where TKey : notnull
{
    private readonly Dictionary<TKey, LinkedListNode<(TKey Key, TValue Value)>> map = new();
    private readonly LinkedList<(TKey Key, TValue Value)> list = new();

    public IEnumerable<TKey> Keys => list.Select(n => n.Key);

    public bool TryGet(TKey key, out TValue value)
    {
        if (map.TryGetValue(key, out var node))
        {
            list.Remove(node);
            list.AddFirst(node);
            value = node.Value.Value;
            return true;
        }
        value = default!;
        return false;
    }

    public TKey? Put(TKey key, TValue value)
    {
        if (map.TryGetValue(key, out var existing))
        {
            existing.Value = (key, value);
            list.Remove(existing);
            list.AddFirst(existing);
            return default;
        }
        map[key] = list.AddFirst((key, value));
        if (map.Count <= capacity) return default;
        var last = list.Last!;
        list.RemoveLast();
        map.Remove(last.Value.Key);
        return last.Value.Key;
    }
}

@@exercise Generic statistics
Write generic helpers that work for `int`, `long`, `double` and `decimal` alike, using `System.Numerics.INumber<T>`:

- `Sum<T>`, `Min<T>`, `Max<T>`
- `Mean<T>`: the sum divided by the count, as T (`T.CreateChecked(count)` converts the count), so integer means are truncated
- `Median<T>`: of the sorted values; for an even count, the mean of the middle two (in T)
- `Summary<T>(IReadOnlyList<T> xs)` returning `n=.. sum=.. min=.. max=.. mean=.. median=..`

`main` summarizes response times (int ms), invoice amounts (decimal) and an empty list (which must throw `InvalidOperationException("no values")`).
@@starter
using System.Numerics;

int[] latencies = [120, 85, 300, 95, 110, 90];
decimal[] invoices = [1980.50m, 15000m, 499.99m];
Console.WriteLine("TODO");
@@expected
latency: n=6 sum=800 min=85 max=300 mean=133 median=102
invoices: n=3 sum=17480.49 min=499.99 max=15000 mean=5826.83 median=1980.50
empty: no values
@@requires
INumber<T>
T.CreateChecked
@@hint
`T Sum<T>(IReadOnlyList<T> xs) where T : INumber<T>` starting from `T.Zero`. `Median`: `var s = xs.Order().ToList();` and for even counts `(s[n/2 - 1] + s[n/2]) / T.CreateChecked(2)`. Throw from `Summary` when `xs.Count == 0`. The decimal mean 17480.49 / 3 has many decimals, so format the mean with `{mean:0.##}` (works for any number type).
@@solution
using System.Numerics;

int[] latencies = [120, 85, 300, 95, 110, 90];
decimal[] invoices = [1980.50m, 15000m, 499.99m];
Console.WriteLine($"latency: {Summary(latencies)}");
Console.WriteLine($"invoices: {Summary(invoices)}");
try
{
    Summary(Array.Empty<double>());
}
catch (InvalidOperationException e)
{
    Console.WriteLine($"empty: {e.Message}");
}

T Sum<T>(IReadOnlyList<T> xs) where T : INumber<T>
{
    T total = T.Zero;
    foreach (var x in xs) total += x;
    return total;
}

T Min<T>(IReadOnlyList<T> xs) where T : INumber<T> => xs.Aggregate(T.Min);
T Max<T>(IReadOnlyList<T> xs) where T : INumber<T> => xs.Aggregate(T.Max);
T Mean<T>(IReadOnlyList<T> xs) where T : INumber<T> => Sum(xs) / T.CreateChecked(xs.Count);

T Median<T>(IReadOnlyList<T> xs) where T : INumber<T>
{
    var s = xs.Order().ToList();
    int n = s.Count;
    return n % 2 == 1 ? s[n / 2] : (s[n / 2 - 1] + s[n / 2]) / T.CreateChecked(2);
}

string Summary<T>(IReadOnlyList<T> xs) where T : INumber<T>
{
    if (xs.Count == 0) throw new InvalidOperationException("no values");
    return $"n={xs.Count} sum={Sum(xs)} min={Min(xs)} max={Max(xs)} mean={Mean(xs):0.##} median={Median(xs)}";
}

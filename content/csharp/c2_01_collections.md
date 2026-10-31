@@lesson c2-collections Lists, Dictionaries and LINQ
@@concepts lists, maps
@@text
## Arrays and List<T>

An array has a fixed length. A `List<T>` grows; `T` is the type of the items (this is called a *generic* type).

```csharp
int[] ports = { 22, 80, 443 };
Console.WriteLine(ports.Length);

var hosts = new List<string> { "web01", "web02" };
hosts.Add("db01");
hosts.Insert(0, "lb01");
hosts.Remove("web02");
Console.WriteLine(string.Join(", ", hosts));   // lb01, web01, db01
Console.WriteLine(hosts.Count);
Console.WriteLine(hosts.Contains("db01"));
Console.WriteLine(hosts[^1]);                   // ^1 = the last item
hosts.Sort();
Console.WriteLine(string.Join(", ", hosts));
```

`List<string>` can only hold strings, and the compiler enforces it. Collection expressions (C# 12) give a shorter way to write lists: `List<int> nums = [1, 2, 3];`.

## Dictionary<TKey, TValue>

```csharp
var owners = new Dictionary<string, string>
{
    ["web01"] = "aiko",
    ["db01"] = "kenji",
};
owners["mail01"] = "yui";             // add or replace
Console.WriteLine(owners["db01"]);

if (owners.TryGetValue("dns01", out var who))
    Console.WriteLine(who);
else
    Console.WriteLine("dns01 has no owner");

foreach (var (host, owner) in owners)
    Console.WriteLine($"{host} -> {owner}");
```

Reading `owners["dns01"]` when it doesn't exist throws `KeyNotFoundException`. Use `TryGetValue` or `ContainsKey` when you're not sure.

Counting things is the classic dictionary job:

```csharp
var counts = new Dictionary<string, int>();
foreach (var level in new[] { "INFO", "ERROR", "INFO", "WARN", "INFO" })
    counts[level] = counts.GetValueOrDefault(level) + 1;
foreach (var (k, v) in counts) Console.WriteLine($"{k}: {v}");
```

A `HashSet<T>` holds unique values: `new HashSet<string>(ips).Count` counts distinct IPs.

## LINQ: querying collections

LINQ (Language Integrated Query) gives every collection SQL-like methods. They take **lambdas**: small inline functions written `x => expression`.

```csharp
var servers = new[]
{
    (Name: "web01", Cpu: 35, Env: "prod"),
    (Name: "web02", Cpu: 92, Env: "prod"),
    (Name: "test01", Cpu: 88, Env: "test"),
    (Name: "db01", Cpu: 71, Env: "prod"),
};

var hot = servers.Where(s => s.Cpu > 80).Select(s => s.Name);
Console.WriteLine(string.Join(", ", hot));            // web02, test01

Console.WriteLine(servers.Average(s => s.Cpu));
Console.WriteLine(servers.Max(s => s.Cpu));
Console.WriteLine(servers.Any(s => s.Cpu > 90));
Console.WriteLine(servers.Count(s => s.Env == "prod"));

var busiest = servers.OrderByDescending(s => s.Cpu).First();
Console.WriteLine(busiest.Name);

foreach (var g in servers.GroupBy(s => s.Env))
    Console.WriteLine($"{g.Key}: {g.Count()} servers, avg {g.Average(s => s.Cpu):F0}%");
```

Useful LINQ methods: `Where`, `Select`, `OrderBy`/`OrderByDescending`/`ThenBy`, `First`/`FirstOrDefault`, `Any`/`All`, `Count`, `Sum`/`Average`/`Min`/`Max`, `GroupBy`, `Distinct`, `Take`/`Skip`, `ToList`/`ToDictionary`.

LINQ is **lazy**: `Where(...)` doesn't do anything until you loop over it or call `ToList()`, `Count()`, and so on.

@@quiz
What does `hosts[^1]` return?
* The last item
- The first item
- An error: indexes can't be negative
- The second-to-last item
? `^1` means "1 from the end". `^2` is the second to last.

@@quiz
`dict["missing"]` when the key isn't there...
* throws `KeyNotFoundException`
- returns `null`
- returns the default value (0, "")
- adds the key
? Use `TryGetValue`, `ContainsKey` or `GetValueOrDefault` when the key may be missing.

@@quiz
What does `nums.Where(n => n > 2).Select(n => n * 10)` give for `[1, 2, 3, 4]`?
* `30, 40`
- `10, 20, 30, 40`
- `3, 4`
- `30`
? `Where` keeps 3 and 4, then `Select` multiplies each by 10.

@@flashcards
`List<T>` :: a growable list of items of type T
`Dictionary<K,V>` :: key → value lookup
`HashSet<T>` :: unique values, fast "contains"
`x => x * 2` :: a lambda: an inline function
`.Where(predicate)` :: keep matching items (SQL WHERE)
`.Select(f)` :: transform each item (SQL SELECT)
`.GroupBy(key)` :: groups with `.Key` (SQL GROUP BY)

@@exercise Log level counts
Count how many lines have each level, and print them sorted by count, highest first (ties: alphabetical), like `ERROR: 3`.
@@starter
string[] log =
{
    "INFO boot", "ERROR disk", "WARN temp", "INFO login", "ERROR disk",
    "INFO logout", "ERROR fan", "WARN temp", "DEBUG x",
};
// count the first word of each line

@@expected
ERROR: 3
INFO: 3
WARN: 2
DEBUG: 1
@@hint
`log.GroupBy(l => l.Split(' ')[0])`, then `.OrderByDescending(g => g.Count()).ThenBy(g => g.Key)`.
@@solution
string[] log =
{
    "INFO boot", "ERROR disk", "WARN temp", "INFO login", "ERROR disk",
    "INFO logout", "ERROR fan", "WARN temp", "DEBUG x",
};
var groups = log.GroupBy(l => l.Split(' ')[0])
    .OrderByDescending(g => g.Count())
    .ThenBy(g => g.Key);
foreach (var g in groups)
    Console.WriteLine($"{g.Key}: {g.Count()}");

@@exercise Licence report
Each user has a list of assigned licences. Print each licence with the number of users and their names in alphabetical order, licences in alphabetical order:

```text
E3: 2 (aiko, kenji)
Visio: 1 (kenji)
...
```
@@starter
var users = new Dictionary<string, List<string>>
{
    ["kenji"] = ["E3", "Visio", "Project"],
    ["aiko"] = ["E3"],
    ["yui"] = ["F1", "Project"],
};
// build licence -> users, then print

@@expected
E3: 2 (aiko, kenji)
F1: 1 (yui)
Project: 2 (kenji, yui)
Visio: 1 (kenji)
@@hint
Flatten into (licence, user) pairs with `SelectMany`: `users.SelectMany(u => u.Value.Select(l => (Licence: l, User: u.Key)))`, then group by licence. Or use two `foreach` loops and a `Dictionary<string, List<string>>`. `string.CompareOrdinal`-style sorting: `OrderBy(x => x, StringComparer.Ordinal)` keeps upper case before lower.
@@solution
var users = new Dictionary<string, List<string>>
{
    ["kenji"] = ["E3", "Visio", "Project"],
    ["aiko"] = ["E3"],
    ["yui"] = ["F1", "Project"],
};
var byLicence = users
    .SelectMany(u => u.Value.Select(l => (Licence: l, User: u.Key)))
    .GroupBy(x => x.Licence)
    .OrderBy(g => g.Key, StringComparer.Ordinal);
foreach (var g in byLicence)
{
    var names = g.Select(x => x.User).OrderBy(n => n);
    Console.WriteLine($"{g.Key}: {g.Count()} ({string.Join(", ", names)})");
}

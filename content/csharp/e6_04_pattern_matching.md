@@lesson e6-patterns Pattern Matching and Switch Expressions
@@concepts conditions
@@text
Modern C# lets you test the **shape** of data (its type, properties, ranges and list contents) in one readable expression.

## Type, property and relational patterns

```csharp
object[] inputs = [42, "web01", 3.5, null!, new Disk("C:", 97), new Disk("D:", 40)];
foreach (var x in inputs)
    Console.WriteLine(Describe(x));

string Describe(object? o) => o switch
{
    null => "nothing",
    int n when n < 0 => "negative int",
    int n => $"int {n}",
    string { Length: 0 } => "empty string",
    string s => $"string '{s}'",
    Disk { UsedPercent: >= 90 } d => $"{d.Drive} almost full",
    Disk(var drive, var used) => $"{drive} at {used}%",   // positional pattern (deconstruction)
    _ => $"other: {o.GetType().Name}",
};

record Disk(string Drive, int UsedPercent);
```

The compiler checks the arms in order and warns when an arm can never match, or when a `switch` expression doesn't cover every case.

## Combining patterns: and, or, not

```csharp
foreach (var code in new[] { 200, 301, 404, 503, 999 })
    Console.WriteLine($"{code} {Classify(code)}");

foreach (var c in "a7_ ")
    Console.WriteLine($"'{c}' {(c is (>= 'a' and <= 'z') or (>= '0' and <= '9') or '_' ? "ok" : "invalid")}");

string Classify(int code) => code switch
{
    >= 200 and < 300 => "success",
    >= 300 and < 400 => "redirect",
    404 or 410 => "gone",
    >= 500 and < 600 => "server error",
    _ => "unknown",
};
```

`is not null` is the idiomatic null check (it can't be fooled by an overloaded `==`).

## Tuple patterns

Switch on several values at once:

```csharp
Console.WriteLine(Decide("prod", true));
Console.WriteLine(Decide("dev", false));

string Decide(string env, bool hasTicket) => (env, hasTicket) switch
{
    ("prod", false) => "blocked: production changes need a ticket",
    ("prod", true) => "allowed with approval",
    (_, _) => "allowed",
};
```

## List patterns (C# 11)

```csharp
string[][] commands = [["restart", "nginx"], ["scale", "web", "5"], ["logs"], ["ping", "a", "b", "c"], []];
foreach (var cmd in commands)
    Console.WriteLine(cmd switch
    {
        ["restart", var svc] => $"restarting {svc}",
        ["scale", var svc, var n] when int.TryParse(n, out var count) => $"scaling {svc} to {count}",
        ["logs"] => "showing logs",
        ["ping", .. var hosts] => $"pinging {hosts.Length} hosts",   // slice pattern
        [] => "usage: <command> [args]",
        _ => "unknown command",
    });
```

`[first, ..]` matches anything starting with `first`; `[.., last]` checks the end; `..` alone skips any number of items.

## Deconstruction

Records and tuples deconstruct into variables, and any type can opt in with a `Deconstruct` method:

```csharp
var (host, port) = Split("db01:5432");
Console.WriteLine($"{host} / {port}");

(string, int) Split(string s)
{
    var parts = s.Split(':');
    return (parts[0], int.Parse(parts[1]));
}
```

@@quiz
In a switch expression, what does `_` match?
* Anything not matched by earlier arms (the discard pattern)
- Only null
- Only underscores
- Nothing
? Put it last.

@@quiz
What does `["ping", .. var hosts]` match?
- Exactly two elements
* Any array starting with "ping", capturing the rest in hosts
- Arrays ending in "ping"
- Only empty arrays
? `..` is a slice pattern.

@@quiz
Which is the idiomatic null check in modern C#?
- `x == null`
* `x is null` / `x is not null`
- `x.Equals(null)`
- `object.Equals(x)`
? Pattern checks can't be changed by operator overloading.

@@drill
Which pattern keyword matches when a value is not null? Write the pattern.
= is not null
= not null
? if (x is not null) { ... }.

@@flashcards
`x switch { ... }` :: switch expression
`int n when n > 0` :: pattern with a guard
`{ Length: 0 }` :: property pattern
`>= 200 and < 300` :: relational + logical patterns
`404 or 410` :: or pattern
`Disk(var d, var u)` :: positional pattern
`[a, b]` :: list pattern
`[first, ..]` :: starts-with list pattern
`.. var rest` :: slice capture
`var (a, b) = t` :: deconstruction

@@exercise A firewall rule engine
Write `Decide(Packet p)` returning a decision string with a **single switch expression** over `p` (property, relational and logical patterns):

- `record Packet(string Proto, string Src, int Port, int Size)`
- `size > 1500` → `drop: oversized` (checked first)
- TCP to 22 from `10.` addresses → `allow: ssh from LAN`; TCP to 22 from elsewhere → `drop: ssh from internet`
- TCP to 80 or 443 → `allow: web`
- UDP to 53 → `allow: dns`
- UDP to 67 or 68 → `allow: dhcp`
- ICMP → `allow: ping` (any port)
- anything else → `drop: default deny (<proto>/<port>)`
@@starter
var packets = new[]
{
    new Packet("TCP", "10.0.0.7", 22, 60),
    new Packet("TCP", "203.0.113.9", 22, 60),
    new Packet("TCP", "198.51.100.4", 443, 1200),
    new Packet("UDP", "10.0.0.1", 53, 80),
    new Packet("UDP", "0.0.0.0", 68, 300),
    new Packet("ICMP", "10.0.0.9", 0, 64),
    new Packet("TCP", "10.0.0.7", 3389, 60),
    new Packet("UDP", "10.0.0.7", 53, 9000),
};
foreach (var p in packets) Console.WriteLine($"{p.Proto,-4} {p.Src,-13} {p.Port,5} -> {Decide(p)}");

string Decide(Packet p) => "TODO";

record Packet(string Proto, string Src, int Port, int Size);
@@expected
TCP  10.0.0.7         22 -> allow: ssh from LAN
TCP  203.0.113.9      22 -> drop: ssh from internet
TCP  198.51.100.4    443 -> allow: web
UDP  10.0.0.1         53 -> allow: dns
UDP  0.0.0.0          68 -> allow: dhcp
ICMP 10.0.0.9          0 -> allow: ping
TCP  10.0.0.7       3389 -> drop: default deny (TCP/3389)
UDP  10.0.0.7         53 -> drop: oversized
@@requires
switch
@@hint
`{ Size: > 1500 } => "drop: oversized"`, `{ Proto: "TCP", Port: 22 } when p.Src.StartsWith("10.") => ...`, `{ Proto: "TCP", Port: 80 or 443 } => ...`, `{ Proto: "ICMP" } => ...`, and `_ => $"drop: default deny ({p.Proto}/{p.Port})"`.
@@solution
var packets = new[]
{
    new Packet("TCP", "10.0.0.7", 22, 60),
    new Packet("TCP", "203.0.113.9", 22, 60),
    new Packet("TCP", "198.51.100.4", 443, 1200),
    new Packet("UDP", "10.0.0.1", 53, 80),
    new Packet("UDP", "0.0.0.0", 68, 300),
    new Packet("ICMP", "10.0.0.9", 0, 64),
    new Packet("TCP", "10.0.0.7", 3389, 60),
    new Packet("UDP", "10.0.0.7", 53, 9000),
};
foreach (var p in packets) Console.WriteLine($"{p.Proto,-4} {p.Src,-13} {p.Port,5} -> {Decide(p)}");

string Decide(Packet p) => p switch
{
    { Size: > 1500 } => "drop: oversized",
    { Proto: "TCP", Port: 22 } when p.Src.StartsWith("10.") => "allow: ssh from LAN",
    { Proto: "TCP", Port: 22 } => "drop: ssh from internet",
    { Proto: "TCP", Port: 80 or 443 } => "allow: web",
    { Proto: "UDP", Port: 53 } => "allow: dns",
    { Proto: "UDP", Port: 67 or 68 } => "allow: dhcp",
    { Proto: "ICMP" } => "allow: ping",
    _ => $"drop: default deny ({p.Proto}/{p.Port})",
};

record Packet(string Proto, string Src, int Port, int Size);

@@exercise A chat-ops command parser
Write `Handle(string line, Dictionary<string, List<string>> groups)` for a chat bot that manages host groups. Split the line on spaces (removing empty entries) and use **list patterns**:

- `add <group> <host> [more hosts...]`: add hosts (no duplicates); reply `added N to <group>` (N = hosts actually added)
- `remove <group> <host>`: reply `removed <host>` or `<host> not in <group>`
- `list <group>`: `<group>: h1, h2` (sorted) or `<group>: (empty)`
- `list`: all groups as `group(count)`, sorted, space-separated
- `help` or nothing: `commands: add, remove, list`
- anything else: `unknown command '<first word>'`
@@starter
var groups = new Dictionary<string, List<string>>();
string[] lines =
[
    "add web web01 web02",
    "add web web02 web03",
    "add db db01",
    "list web",
    "remove web web09",
    "remove web web01",
    "list",
    "list cache",
    "",
    "reboot all",
];
foreach (var l in lines) Console.WriteLine($"> {l}\n{Handle(l, groups)}");

string Handle(string line, Dictionary<string, List<string>> groups) => "TODO";
@@expected
> add web web01 web02
added 2 to web
> add web web02 web03
added 1 to web
> add db db01
added 1 to db
> list web
web: web01, web02, web03
> remove web web09
web09 not in web
> remove web web01
removed web01
> list
db(1) web(2)
> list cache
cache: (empty)
> 
commands: add, remove, list
> reboot all
unknown command 'reboot'
@@requires
.. var
@@hint
`var words = line.Split(' ', StringSplitOptions.RemoveEmptyEntries); return words switch { ["add", var g, .. var hosts] when hosts.Length > 0 => Add(g, hosts), ["remove", var g, var h] => ..., ["list", var g] => ..., ["list"] => ..., [] or ["help"] => ..., [var cmd, ..] => $"unknown command '{cmd}'" };`. Look groups up with `groups.TryGetValue(g, out var list)`; a local function `Add` keeps the add arm short.
@@solution
var groups = new Dictionary<string, List<string>>();
string[] lines =
[
    "add web web01 web02",
    "add web web02 web03",
    "add db db01",
    "list web",
    "remove web web09",
    "remove web web01",
    "list",
    "list cache",
    "",
    "reboot all",
];
foreach (var l in lines) Console.WriteLine($"> {l}\n{Handle(l, groups)}");

string Handle(string line, Dictionary<string, List<string>> groups)
{
    var words = line.Split(' ', StringSplitOptions.RemoveEmptyEntries);
    return words switch
    {
        ["add", var g, .. var hosts] when hosts.Length > 0 => Add(g, hosts),
        ["remove", var g, var h] => groups.TryGetValue(g, out var list) && list.Remove(h) ? $"removed {h}" : $"{h} not in {g}",
        ["list", var g] => groups.TryGetValue(g, out var list) && list.Count > 0
            ? $"{g}: {string.Join(", ", list.Order(StringComparer.Ordinal))}"
            : $"{g}: (empty)",
        ["list"] => string.Join(" ", groups.OrderBy(kv => kv.Key, StringComparer.Ordinal).Select(kv => $"{kv.Key}({kv.Value.Count})")),
        [] or ["help"] => "commands: add, remove, list",
        [var cmd, ..] => $"unknown command '{cmd}'",
    };

    string Add(string g, string[] hosts)
    {
        if (!groups.TryGetValue(g, out var list)) groups[g] = list = new List<string>();
        int added = 0;
        foreach (var h in hosts)
        {
            if (list.Contains(h)) continue;
            list.Add(h);
            added++;
        }
        return $"added {added} to {g}";
    }
}

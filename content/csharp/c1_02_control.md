@@lesson c1-control Decisions and Loops
@@concepts conditions, loops, input
@@text
## if / else

```csharp
int cpu = 91;
if (cpu >= 90)
{
    Console.WriteLine("CRITICAL");
}
else if (cpu >= 75)
{
    Console.WriteLine("WARNING");
}
else
{
    Console.WriteLine("OK");
}
```

Comparisons: `==`, `!=`, `<`, `<=`, `>`, `>=`. Combine them with `&&` (and), `||` (or), `!` (not). Unlike JavaScript, `==` on two strings compares their text, and there's no type juggling: `5 == "5"` doesn't even compile.

The **ternary** operator picks one of two values: `string state = online ? "up" : "down";`

## switch expressions

When one value decides between many results, a `switch` expression is neater than a chain of `if`s:

```csharp
int port = 22;
string service = port switch
{
    22 => "SSH",
    80 or 8080 => "HTTP",
    443 => "HTTPS",
    >= 49152 => "dynamic",
    _ => "unknown",      // _ means "anything else"
};
Console.WriteLine(service);
```

The patterns can be values, `or`/`and`, comparisons like `>= 49152`, and more. The compiler warns you if a case could never be reached.

## Loops

```csharp
for (int i = 1; i <= 3; i++)
{
    Console.WriteLine($"ping attempt {i}");
}

string[] hosts = { "web01", "web02", "db01" };
foreach (string h in hosts)
{
    if (h.StartsWith("db")) continue;   // skip to the next one
    Console.WriteLine($"checking {h}");
}

int retries = 0;
while (retries < 5)
{
    retries++;
    if (retries == 3) break;            // leave the loop
}
Console.WriteLine($"stopped after {retries}");
```

`do { ... } while (condition);` runs the body at least once: good for "ask until the answer is valid" input loops.

## null

A `string?` (with `?`) is allowed to be `null`, meaning "no value". Plain `string` should never be null, and the compiler warns you if it might be. The null operators keep code short:

```csharp
string? manager = null;
Console.WriteLine(manager ?? "(none)");        // ?? : use this if null
Console.WriteLine(manager?.Length ?? 0);       // ?. : only call if not null
manager ??= "unassigned";                      // assign only if null
Console.WriteLine(manager);
```

`Console.ReadLine()` returns `string?`, because there may be no more input.

@@quiz
What does this print? `int x = 7; Console.WriteLine(x % 2 == 0 ? "even" : "odd");`
* `odd`
- `even`
- `1`
- `True`
? `7 % 2` is 1, so the condition is false, and the ternary picks the second value.

@@quiz
In a `switch` expression, what does `_ => "unknown"` do?
* Matches any value not matched by an earlier case
- Matches only `null`
- Matches an empty string
- Is a syntax error
? `_` is the "discard" pattern: it matches everything, so it goes last.

@@quiz
`string? name = null; int n = name?.Length ?? -1;` What is `n`?
* `-1`
- `0`
- it throws a NullReferenceException
- it doesn't compile
? `name?.Length` gives `null` because `name` is null, and `?? -1` replaces that null with -1.

@@drill
Which operator returns a fallback when the left side is null?
= ??
? name ?? "unknown".

@@flashcards
if / else if / else :: choose a branch
ternary :: cond ? a : b
switch expression :: x switch { 1 => "one", _ => "other" }
for / foreach / while :: loops
string? :: a string that may be null
?. :: null-conditional: stop if null
?? :: use a fallback when null

@@exercise HTTP status classes
Write the body of `Classify(int code)` with a `switch` expression: 200–299 → `"success"`, 300–399 → `"redirect"`, 400–499 → `"client error"`, 500–599 → `"server error"`, anything else → `"invalid"`.
@@starter
string Classify(int code)
{
    throw new NotImplementedException("TODO: not done yet");
}

foreach (int c in new[] { 200, 301, 404, 503, 42 })
    Console.WriteLine($"{c}: {Classify(c)}");
@@expected
200: success
301: redirect
404: client error
503: server error
42: invalid
@@requires
switch
@@hint
Range patterns combine with `and`: `>= 200 and <= 299 => "success",`. Finish with `_ => "invalid"`.
@@solution
string Classify(int code)
{
    return code switch
    {
        >= 200 and <= 299 => "success",
        >= 300 and <= 399 => "redirect",
        >= 400 and <= 499 => "client error",
        >= 500 and <= 599 => "server error",
        _ => "invalid",
    };
}

foreach (int c in new[] { 200, 301, 404, 503, 42 })
    Console.WriteLine($"{c}: {Classify(c)}");

@@exercise Retry with backoff
A service is called with retries. Starting with a 1 second wait, the wait doubles after each failed attempt, up to a cap of 8 seconds. Print the wait for attempts 1 to 6 like this:

```text
attempt 1: wait 1s
attempt 2: wait 2s
attempt 3: wait 4s
attempt 4: wait 8s
attempt 5: wait 8s
attempt 6: wait 8s
```
@@starter
int wait = 1;
// loop over attempts 1..6

@@expected
attempt 1: wait 1s
attempt 2: wait 2s
attempt 3: wait 4s
attempt 4: wait 8s
attempt 5: wait 8s
attempt 6: wait 8s
@@hint
Inside a `for` loop, print first, then `wait = Math.Min(wait * 2, 8);`.
@@solution
int wait = 1;
for (int attempt = 1; attempt <= 6; attempt++)
{
    Console.WriteLine($"attempt {attempt}: wait {wait}s");
    wait = Math.Min(wait * 2, 8);
}

@@exercise Read until valid
Read lines from input until one is a whole number between 1 and 65535, printing `invalid: <text>` for each bad line. Then print `port = <n>`. Stop if the input runs out (ReadLine returns null).
@@stdin
abc
70000
8443
22
@@starter
string? line;
// read lines with Console.ReadLine()

@@expected
invalid: abc
invalid: 70000
port = 8443
@@hint
`while ((line = Console.ReadLine()) != null) { if (int.TryParse(line, out int p) && p >= 1 && p <= 65535) { ...; break; } ... }`
@@solution
string? line;
while ((line = Console.ReadLine()) != null)
{
    if (int.TryParse(line, out int port) && port >= 1 && port <= 65535)
    {
        Console.WriteLine($"port = {port}");
        break;
    }
    Console.WriteLine($"invalid: {line}");
}

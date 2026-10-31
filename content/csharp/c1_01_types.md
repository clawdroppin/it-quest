@@lesson c1-types Numbers, Text and Conversions
@@concepts numbers, strings
@@text
## Number types

| Type | Holds | Example |
|---|---|---|
| `int` | whole numbers, about ±2.1 billion | `int port = 8080;` |
| `long` | huge whole numbers | `long bytes = 5_000_000_000;` |
| `double` | decimals (fast, slightly inexact) | `double load = 0.75;` |
| `decimal` | exact decimals: **use it for money** | `decimal price = 1980.50m;` |

```csharp
Console.WriteLine(7 / 2);      // 3: int divided by int stays int
Console.WriteLine(7 / 2.0);    // 3.5
Console.WriteLine(7 % 2);      // 1 (remainder)
Console.WriteLine(0.1 + 0.2);  // 0.30000000000000004
Console.WriteLine(0.1m + 0.2m);// 0.3: decimal is exact
Console.WriteLine($"{Math.Round(2.567, 1)} {Math.Max(3, 9)}");
```

Whole numbers have limits, and going past them wraps around silently:

```csharp
int big = int.MaxValue;
Console.WriteLine(big + 1);          // wraps around to -2147483648!
Console.WriteLine((long)big + 1);    // 2147483648
```

## Formatting numbers

Inside `{ }`, a colon adds a format:

```csharp
double ratio = 0.8734;
long size = 1234567890;
Console.WriteLine($"{ratio:P1}");      // 87.3%
Console.WriteLine($"{ratio:F2}");      // 0.87
Console.WriteLine($"{size:N0}");       // 1,234,567,890
Console.WriteLine($"{42:D5}");         // 00042
Console.WriteLine($"[{"web01",-8}|{"ok",5}]"); // [web01   |   ok]
```

`,-8` pads to 8 characters, left-aligned; `,5` right-aligns. That's how you print neat tables.

## Strings

```csharp
string fqdn = "  Web01.Contoso.Local ";
string clean = fqdn.Trim().ToLower();
Console.WriteLine(clean);
Console.WriteLine(clean.Split('.')[0]);
Console.WriteLine(clean.Contains("contoso"));
Console.WriteLine(clean.StartsWith("web"));
Console.WriteLine(clean.Replace(".local", ".jp"));
Console.WriteLine(clean.Length);
Console.WriteLine(clean.ToUpper()[..5]);     // WEB01: a range, from the start to index 5
Console.WriteLine(string.Join(", ", new[] { "dns", "dhcp", "ntp" }));
```

Strings never change ("immutable"): `Trim()` returns a *new* string. `fqdn.Trim();` on its own line does nothing useful.

Verbatim strings `@"..."` don't treat `\` as special, which is handy for Windows paths. Raw strings `"""..."""` can hold quotes and new lines:

```csharp
string path = @"C:\Logs\app.log";
string json = """{"host": "web01", "port": 443}""";
Console.WriteLine(path);
Console.WriteLine(json);
```

## Converting text to numbers

Input from users, files and the network always arrives as text.

```csharp
int a = int.Parse("443");                 // throws an exception on bad input
Console.WriteLine(a + 1);
if (int.TryParse("44x", out int b))       // safe: returns true or false
    Console.WriteLine($"got {b}");
else
    Console.WriteLine("not a number");
Console.WriteLine(Convert.ToString(255, 2));       // 11111111: binary
Console.WriteLine(Convert.ToInt32("ff", 16));      // 255: from hex
```

Use `TryParse` whenever the text might be wrong, which is most of the time.

## char

A `char` is one character, in single quotes: `'A'`. `char.IsDigit(c)`, `char.IsLetter(c)` and `char.ToUpper(c)` are useful.

```csharp
int digits = 0;
foreach (char c in "Pa55w0rd")
    if (char.IsDigit(c)) digits++;
Console.WriteLine(digits);   // 3
```

@@quiz
What is `7 / 2` in C#?
* `3`
- `3.5`
- `4`
- a compile error
? Both sides are `int`, so the result is `int`, and the decimal part is cut off. Write `7 / 2.0` or `(double)7 / 2` to get 3.5.

@@quiz
Which type should hold a price in yen or a bank balance?
* `decimal`
- `double`
- `float`
- `int` always
? `decimal` is exact in base 10. `double` gives tiny rounding errors like 0.30000000000000004.

@@quiz
What's the safe way to turn user input into an `int`?
* `int.TryParse(text, out int n)`
- `int.Parse(text)`
- `(int)text`
- `text.ToInt()`
? `Parse` throws an exception on bad input. `TryParse` returns `false` instead, so you can handle the problem.

@@drill
Write the format specifier that prints `0.5` as `50.0%` inside `$"{ratio:...}"`.
= P1
? `P` is percent; the number is how many decimals.

@@flashcards
`decimal` :: exact decimal type for money (suffix `m`)
`int.TryParse(s, out int n)` :: convert text to a number without throwing
`$"{x:N0}"` :: thousands separators, no decimals
`$"{x,-10}"` :: pad to 10 characters, left-aligned
`@"C:\path"` :: verbatim string: backslashes are literal
`s[..3]` :: the first three characters (a range)

@@exercise Disk usage line
Given `usedGb = 412.6` and `totalGb = 512`, print exactly:

```text
C: 412.6 / 512 GB (80.6%)
```

Calculate the percentage; don't type it in.
@@starter
double usedGb = 412.6;
double totalGb = 512;
// print the line

@@expected
C: 412.6 / 512 GB (80.6%)
@@requires
usedGb / totalGb
@@hint
`usedGb / totalGb` is about 0.8059. The `P1` format multiplies by 100 and adds `%`... but it may also add a space depending on the culture. Safer: `{usedGb / totalGb * 100:F1}%`.
@@solution
double usedGb = 412.6;
double totalGb = 512;
Console.WriteLine($"C: {usedGb} / {totalGb} GB ({usedGb / totalGb * 100:F1}%)");

@@exercise Password check
Count the digits, upper-case letters and lower-case letters in `password` and print them like this (for `Tokyo2026!`):

```text
digits=4 upper=1 lower=4
```
@@starter
string password = "Tokyo2026!";
int digits = 0, upper = 0, lower = 0;
// loop over the characters

Console.WriteLine($"digits={digits} upper={upper} lower={lower}");
@@expected
digits=4 upper=1 lower=4
@@hint
`foreach (char c in password)` then `if (char.IsDigit(c)) digits++;` and the same with `char.IsUpper` and `char.IsLower`.
@@solution
string password = "Tokyo2026!";
int digits = 0, upper = 0, lower = 0;
foreach (char c in password)
{
    if (char.IsDigit(c)) digits++;
    else if (char.IsUpper(c)) upper++;
    else if (char.IsLower(c)) lower++;
}
Console.WriteLine($"digits={digits} upper={upper} lower={lower}");

@@lesson c1-strings Strings in Depth: StringBuilder, Comparisons, Raw Literals and Unicode
@@concepts strings, text-processing
@@text
## Strings are immutable

Every "change" to a string creates a new string. That's fine for a few operations, but building a big report with `+=` in a loop copies the text again and again. Use `StringBuilder`:

```csharp
using System.Text;

var sb = new StringBuilder();
sb.AppendLine("host     status");
foreach (var (host, ok) in new[] { ("web01", true), ("db01", false) })
    sb.Append($"{host,-8} ").AppendLine(ok ? "up" : "DOWN");
Console.Write(sb.ToString());
```

## Comparing strings correctly

```csharp
string a = "Admin", b = "admin";
Console.WriteLine(a == b);                                              // False: ordinal, case-sensitive
Console.WriteLine(string.Equals(a, b, StringComparison.OrdinalIgnoreCase)); // True
Console.WriteLine(a.Equals(b, StringComparison.OrdinalIgnoreCase));
Console.WriteLine("web10".CompareTo("web9"));  // -1: text order, not number order
var users = new HashSet<string>(StringComparer.OrdinalIgnoreCase) { "Admin" };
Console.WriteLine(users.Contains("ADMIN"));    // True
```

For identifiers (usernames, hostnames, file extensions, keys), use **Ordinal** or **OrdinalIgnoreCase**: fast and predictable. Culture-aware comparison is for sorting text shown to people. A classic bug: in Turkish, `"file".ToUpper()` isn't `"FILE"` (dotted İ), so case-insensitive checks with the current culture fail.

## Literals: verbatim and raw

```csharp
string path = @"C:\Logs\app.log";             // verbatim: backslashes are literal
string json = """
    {"host": "web01", "path": "C:\\temp"}
    """;                                          // raw literal (C# 11): no escaping at all
var host = "db01";
string query = $$"""
    {"filter": {"host": "{{host}}"} }
    """;                                          // $$: {{ }} interpolates, single braces are literal
Console.WriteLine(path);
Console.WriteLine(json);
Console.WriteLine(query);
```

Raw literals remove the indentation of the closing `"""` line, so they can be indented with your code. With `$$`, two braces in a row always mean interpolation, which is why the JSON above needs a space in `} }`; use `$$$` when the text itself has double braces.

## Splitting and joining

```csharp
var line = "web01, db01,,  cache01 ";
var parts = line.Split(',', StringSplitOptions.TrimEntries | StringSplitOptions.RemoveEmptyEntries);
Console.WriteLine(string.Join("|", parts));       // web01|db01|cache01
Console.WriteLine(string.Concat(Enumerable.Repeat("=-", 5)));
```

## Unicode: char is not a character

A .NET `string` is UTF-16. A `char` is one 16-bit code unit, so emoji and rare kanji take **two** chars (a surrogate pair), and some visible characters are several code points (a family emoji, a letter with a combining accent):

```csharp
using System.Globalization;
using System.Text;

string s = "東京🗼";
Console.WriteLine(s.Length);                              // 4 chars: the emoji is 2
Console.WriteLine(s.EnumerateRunes().Count());            // 3 Unicode code points
Console.WriteLine(new StringInfo(s).LengthInTextElements); // 3 user-perceived characters
Console.WriteLine(Encoding.UTF8.GetByteCount(s));         // 10 bytes in UTF-8
```

`s[..3]` could cut an emoji in half and produce broken text; cut by **text elements** (`StringInfo`) when the result is shown to people.

## Regular expressions

```csharp
using System.Text.RegularExpressions;

var m = Regex.Match("Failed password for admin from 10.0.0.7 port 22", @"for (\w+) from ([\d.]+)");
if (m.Success) Console.WriteLine($"{m.Groups[1].Value} @ {m.Groups[2].Value}");
```

In real apps, the `[GeneratedRegex]` attribute compiles patterns at build time for speed.

@@quiz
Why use `StringBuilder` in a loop that builds a large report?
* Strings are immutable, so `+=` copies the whole text every time
- StringBuilder is required for AppendLine
- It's thread-safe
- It compresses the text
? Hundreds of `+=` on a growing string is quadratic work.

@@quiz
Which comparison should check whether a username equals "admin" regardless of case?
- `name.ToLower() == "admin"`
* `string.Equals(name, "admin", StringComparison.OrdinalIgnoreCase)`
- `name.CompareTo("admin") == 0`
- `name.Contains("admin")`
? Ordinal comparisons don't depend on the user's culture.

@@quiz
What is `"🗼".Length`?
- 1
* 2
- 4
- 3
? Emoji outside the Basic Multilingual Plane need a surrogate pair in UTF-16.

@@drill
Which class builds a long string efficiently in a loop?
= StringBuilder
= System.Text.StringBuilder
? Strings are immutable; += copies every time.

@@flashcards
`StringBuilder` :: efficient text building
`StringComparison.Ordinal` :: byte-wise, culture-free comparison
`StringComparer.OrdinalIgnoreCase` :: case-insensitive keys for collections
`@"..."` :: verbatim string
`"""..."""` :: raw string literal
`$$"""{{x}}"""` :: raw interpolated with literal braces
`StringSplitOptions.TrimEntries` :: trim each part
`char` :: one UTF-16 code unit
`Rune` :: one Unicode code point
`StringInfo` :: user-perceived characters (text elements)

@@exercise A fixed-width report
Write `Render(List<Disk> disks)` that returns a report as one string, built with a `StringBuilder`:

- header `Host       Mount        Used     Size   Use%`, then a line of `-` as long as the header
- columns: host left-aligned in 10, mount left-aligned in 12 (longer mounts are cut to 11 characters plus `…`), used and size in GB right-aligned in 7 with one decimal (`F1`), use% right-aligned in 6 as a whole number plus `%`
- rows with use ≥ 90% end with ` !`
- a final line `3 disks, 1 nearly full`

`Disk` is a record with `Host`, `Mount`, `UsedGb` and `SizeGb` (doubles).
@@starter
using System.Text;

var disks = new List<Disk>
{
    new("web01", "/", 18.4, 40),
    new("db01", "/var/lib/postgresql", 455.2, 500),
    new("files01", "D:", 1210.75, 2000),
};
Console.Write(Render(disks));

string Render(List<Disk> disks)
{
    var sb = new StringBuilder();
    return sb.ToString();
}

record Disk(string Host, string Mount, double UsedGb, double SizeGb);
@@expected
Host       Mount        Used     Size   Use%
--------------------------------------------
web01      /              18.4   40.0    46%
db01       /var/lib/po…  455.2  500.0    91% !
files01    D:           1210.8 2000.0    61%
3 disks, 1 nearly full
@@requires
StringBuilder
@@hint
Format a row with `$"{d.Host,-10} {mount,-12}{d.UsedGb,7:F1}{d.SizeGb,7:F1}{pct,6:F0}%"`. Note the header's exact spacing: count the characters, or build the header with the same alignments. Percent: `d.UsedGb / d.SizeGb * 100`, formatted with `F0` (rounds).
@@solution
using System.Text;

var disks = new List<Disk>
{
    new("web01", "/", 18.4, 40),
    new("db01", "/var/lib/postgresql", 455.2, 500),
    new("files01", "D:", 1210.75, 2000),
};
Console.Write(Render(disks));

string Render(List<Disk> disks)
{
    var sb = new StringBuilder();
    string header = "Host       Mount        Used     Size   Use%";
    sb.AppendLine(header);
    sb.AppendLine(new string('-', header.Length));
    int full = 0;
    foreach (var d in disks)
    {
        string mount = d.Mount.Length > 12 ? d.Mount[..11] + "…" : d.Mount;
        double pct = d.UsedGb / d.SizeGb * 100;
        sb.Append($"{d.Host,-10} {mount,-12}{d.UsedGb,7:F1}{d.SizeGb,7:F1}{pct,6:F0}%");
        if (pct >= 90)
        {
            sb.Append(" !");
            full++;
        }
        sb.AppendLine();
    }
    sb.AppendLine($"{disks.Count} disks, {full} nearly full");
    return sb.ToString();
}

record Disk(string Host, string Mount, double UsedGb, double SizeGb);

@@exercise Unicode-safe truncation
Display names in a Japanese helpdesk app contain kanji and emoji. Write:

- `Truncate(string s, int max)`: at most `max` **text elements**; if cut, the last element becomes `…` (so the result still has at most `max` elements)
- `Initials(string name)`: the first text element of each space-separated word, upper-cased (`ToUpperInvariant`)
- `Describe(string s)`: `chars=<Length> runes=<rune count> elements=<text elements> utf8=<bytes>`

Never split a surrogate pair or an emoji sequence. Use `StringInfo` (`System.Globalization`).
@@starter
using System.Globalization;
using System.Text;

string[] names = ["山田 太郎", "ana 🚀 ops", "👨‍👩‍👧 family team", "Kenji"];
foreach (var n in names)
    Console.WriteLine(n);

string Truncate(string s, int max) => s;
@@expected
山田 太郎 | 山田 … | 山太 | chars=5 runes=5 elements=5 utf8=13
ana 🚀 ops | ana… | A🚀O | chars=10 runes=9 elements=9 utf8=12
👨‍👩‍👧 family team | 👨‍👩‍👧 f… | 👨‍👩‍👧FT | chars=20 runes=17 elements=13 utf8=30
Kenji | Ken… | K | chars=5 runes=5 elements=5 utf8=5
@@requires
StringInfo
@@hint
`var si = new StringInfo(s); if (si.LengthInTextElements <= max) return s; return si.SubstringByTextElements(0, max - 1) + "…";`. For initials, split on `' '` with `RemoveEmptyEntries` and take `new StringInfo(w).SubstringByTextElements(0, 1)`. Print each name as `$"{n} | {Truncate(n, 4)} | {Initials(n)} | {Describe(n)}"`.
@@solution
using System.Globalization;
using System.Text;

string[] names = ["山田 太郎", "ana 🚀 ops", "👨‍👩‍👧 family team", "Kenji"];
foreach (var n in names)
    Console.WriteLine($"{n} | {Truncate(n, 4)} | {Initials(n)} | {Describe(n)}");

string Truncate(string s, int max)
{
    var si = new StringInfo(s);
    if (si.LengthInTextElements <= max) return s;
    return si.SubstringByTextElements(0, max - 1) + "…";
}

string Initials(string name) =>
    string.Concat(name.Split(' ', StringSplitOptions.RemoveEmptyEntries)
        .Select(w => new StringInfo(w).SubstringByTextElements(0, 1).ToUpperInvariant()));

string Describe(string s) =>
    $"chars={s.Length} runes={s.EnumerateRunes().Count()} elements={new StringInfo(s).LengthInTextElements} utf8={Encoding.UTF8.GetByteCount(s)}";

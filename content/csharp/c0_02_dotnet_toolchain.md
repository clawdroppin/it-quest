@@lesson c0-toolchain The .NET Toolchain, Projects, Arguments and Enums
@@concepts modules, variables
@@text
## SDK, runtime and language

Three things share version numbers, which confuses everyone at first:

| Piece | What it is | Example |
|---|---|---|
| **.NET SDK** | the tools to build: compiler, `dotnet` CLI, templates | SDK 8.0.4xx |
| **.NET runtime** | what runs compiled apps (and the base class library) | .NET 8, .NET 10 |
| **C# language** | the syntax, chosen by the SDK | C# 12 (with .NET 8), C# 14 (with .NET 10) |

Even-numbered .NET versions are **LTS** (3 years of support); companies usually standardize on them.

```text
dotnet --list-sdks                  # which SDKs are installed
dotnet new console -o PingTool      # create a project
cd PingTool
dotnet run -- web01 --count 3       # build and run; args after --
dotnet build -c Release
dotnet publish -c Release -r win-x64 --self-contained   # a folder you can copy to any Windows PC
```

## The project file

```xml
<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <OutputType>Exe</OutputType>
    <TargetFramework>net8.0</TargetFramework>
    <Nullable>enable</Nullable>          <!-- warn about possible nulls -->
    <ImplicitUsings>enable</ImplicitUsings> <!-- System, System.Linq, ... imported for you -->
    <TreatWarningsAsErrors>true</TreatWarningsAsErrors>
  </PropertyGroup>
  <ItemGroup>
    <PackageReference Include="Serilog" Version="4.0.0" />  <!-- a NuGet package -->
  </ItemGroup>
</Project>
```

## Top-level statements and Main

The programs in this course use **top-level statements**: code at the top of `Program.cs` runs first. The compiler wraps it in a hidden `Main` method; `args` holds the command-line arguments, and `return 2;` sets the exit code. The classic form is still common in older code:

```csharp,norun
namespace PingTool;

internal static class Program
{
    private static int Main(string[] args)
    {
        Console.WriteLine($"{args.Length} arguments");
        return 0;
    }
}
```

Methods declared in top-level code are local functions; **types** (classes, records, enums) go after the statements.

## Warnings vs errors

Errors (`CS0103: The name 'x' does not exist`) stop the build. Warnings (`CS8602: Dereference of a possibly null reference`) don't, but they usually point at real bugs. Teams turn on `TreatWarningsAsErrors` so they can't pile up.

## Constants and enums

```csharp
const int MaxRetries = 3;               // compile-time constant
Console.WriteLine(Level.Warning);       // Warning
Console.WriteLine((int)Level.Error);    // 3
Console.WriteLine(Enum.Parse<Level>("debug", ignoreCase: true));
Console.WriteLine(Enum.TryParse<Level>("loud", out _));

var p = Perm.Read | Perm.Write;          // combine flags with |
Console.WriteLine(p);                    // Read, Write
Console.WriteLine(p.HasFlag(Perm.Write));
Console.WriteLine((p & Perm.Execute) != 0);
Console.WriteLine(MaxRetries);

enum Level { Debug = 0, Info = 1, Warning = 2, Error = 3 }

[Flags]
enum Perm { None = 0, Read = 1, Write = 2, Execute = 4 }
```

`[Flags]` enums use powers of two so several values fit in one number, like Unix permission bits or Windows ACL rights.

@@quiz
Which piece decides which C# language features you can use?
- The runtime installed on the user's PC
* The SDK (compiler) version and the project's LangVersion
- The operating system
- NuGet
? The runtime runs compiled code; the compiler decides the syntax.

@@quiz
What does `return 2;` at the end of top-level statements do?
* Sets the process exit code to 2
- Prints 2
- Is a compile error
- Restarts the program twice
? Scripts and CI check exit codes.

@@quiz
Why do `[Flags]` enum values use 1, 2, 4, 8...?
- Tradition only
* Each value is one bit, so several can be combined in one number with |
- To save memory
- Enums require powers of two
? `Read | Write` = 3.

@@drill
Which command builds and runs a .NET project?
= dotnet run
? dotnet build only compiles.

@@flashcards
.NET SDK :: compiler and tools
.NET runtime :: runs compiled apps
LTS :: even .NET versions, 3 years of support
`dotnet new console` :: create a project
`dotnet run -- args` :: build and run with arguments
`.csproj` :: project file
`<Nullable>enable` :: nullable reference warnings
Top-level statements :: code that becomes Main
`[Flags]` :: enum values combinable with |
`Enum.TryParse` :: text to enum, safely

@@exercise A tiny argument parser
Write `Parse(string[] args)` returning an `Options` record for a ping tool, and `Run(string[] args)` that returns an exit code:

- usage: `ping <host> [--count N] [--verbose]`; `--count` defaults to 4 and must be 1–100
- `Run` prints `pinging <host> N times` (plus ` (verbose)` when set) and returns 0
- errors print `error: <message>` and return 2: `missing host`, `--count needs a number from 1 to 100`, `unknown option <opt>`

Throw an `ArgumentException` from `Parse` and catch it in `Run`. `main` (provided) prints the exit code of each call.
@@starter
string[][] cases =
[
    ["web01"],
    ["web01", "--count", "3", "--verbose"],
    [],
    ["web01", "--count", "500"],
    ["web01", "--count"],
    ["web01", "--fast"],
];
foreach (var c in cases)
{
    Console.WriteLine($"$ ping {string.Join(' ', c)}");
    Console.WriteLine($"exit {Run(c)}");
}

int Run(string[] args)
{
    return 0;
}

record Options(string Host, int Count, bool Verbose);
@@expected
$ ping web01
pinging web01 4 times
exit 0
$ ping web01 --count 3 --verbose
pinging web01 3 times (verbose)
exit 0
$ ping 
error: missing host
exit 2
$ ping web01 --count 500
error: --count needs a number from 1 to 100
exit 2
$ ping web01 --count
error: --count needs a number from 1 to 100
exit 2
$ ping web01 --fast
error: unknown option --fast
exit 2
@@requires
ArgumentException
@@hint
Loop with an index: `for (int i = 0; i < args.Length; i++)`. On `--count`, check `i + 1 < args.Length && int.TryParse(args[i + 1], out var n) && n is >= 1 and <= 100`, then `i++`. Anything else starting with `--` is unknown; the first other value is the host.
@@solution
string[][] cases =
[
    ["web01"],
    ["web01", "--count", "3", "--verbose"],
    [],
    ["web01", "--count", "500"],
    ["web01", "--count"],
    ["web01", "--fast"],
];
foreach (var c in cases)
{
    Console.WriteLine($"$ ping {string.Join(' ', c)}");
    Console.WriteLine($"exit {Run(c)}");
}

Options Parse(string[] args)
{
    string? host = null;
    int count = 4;
    bool verbose = false;
    for (int i = 0; i < args.Length; i++)
    {
        switch (args[i])
        {
            case "--count":
                if (i + 1 < args.Length && int.TryParse(args[i + 1], out var n) && n is >= 1 and <= 100)
                {
                    count = n;
                    i++;
                }
                else
                {
                    throw new ArgumentException("--count needs a number from 1 to 100");
                }
                break;
            case "--verbose":
                verbose = true;
                break;
            case var opt when opt.StartsWith("--"):
                throw new ArgumentException($"unknown option {opt}");
            default:
                host ??= args[i];
                break;
        }
    }
    if (host is null) throw new ArgumentException("missing host");
    return new Options(host, count, verbose);
}

int Run(string[] args)
{
    try
    {
        var o = Parse(args);
        Console.WriteLine($"pinging {o.Host} {o.Count} times{(o.Verbose ? " (verbose)" : "")}");
        return 0;
    }
    catch (ArgumentException e)
    {
        Console.WriteLine($"error: {e.Message}");
        return 2;
    }
}

record Options(string Host, int Count, bool Verbose);

@@exercise Permission flags
Model file permissions with a `[Flags]` enum `Perm { None = 0, Read = 4, Write = 2, Execute = 1 }` (the Unix values):

- `ToText(Perm p)`: the `rwx` form, with `-` for missing bits (`r-x`)
- `FromText(string s)`: the reverse (notice that a flags enum prints its names in ascending value order, so `Read | Write` shows as `Write, Read` here); throws `FormatException` with `bad permission "<s>"` if it isn't 3 characters of `r`/`-`, `w`/`-`, `x`/`-`
- `Octal(Perm owner, Perm group, Perm other)`: like `chmod`, e.g. `750`

`main` (provided) prints several conversions.
@@starter
Console.WriteLine(ToText(Perm.Read | Perm.Execute));

string ToText(Perm p) => "";

[Flags]
enum Perm { None = 0, Read = 4, Write = 2, Execute = 1 }
@@expected
r-x
rw- = Write, Read = 6
--- = None
750
rwxr-x---
error: bad permission "rwz"
error: bad permission "rw"
@@hint
`ToText`: `(p.HasFlag(Perm.Read) ? "r" : "-") + ...`. `FromText`: check `s.Length == 3`, then each position against its letter or `-`. Octal: `$"{(int)owner}{(int)group}{(int)other}"`. `main`: print `ToText(Perm.Read | Perm.Execute)`; then `var p = FromText("rw-")` printed as `$"rw- = {p} = {(int)p}"`; the same for `---`; `Octal(Perm.Read | Perm.Write | Perm.Execute, Perm.Read | Perm.Execute, Perm.None)`; the three `ToText` values of that concatenated; then `FromText("rwz")` and `FromText("rw")` in a try/catch printing `error: <message>`.
@@solution
Console.WriteLine(ToText(Perm.Read | Perm.Execute));
var p = FromText("rw-");
Console.WriteLine($"rw- = {p} = {(int)p}");
var none = FromText("---");
Console.WriteLine($"--- = {none}");
var owner = Perm.Read | Perm.Write | Perm.Execute;
var group = Perm.Read | Perm.Execute;
Console.WriteLine(Octal(owner, group, Perm.None));
Console.WriteLine(ToText(owner) + ToText(group) + ToText(Perm.None));
foreach (var bad in new[] { "rwz", "rw" })
{
    try
    {
        FromText(bad);
    }
    catch (FormatException e)
    {
        Console.WriteLine($"error: {e.Message}");
    }
}

string ToText(Perm p) =>
    (p.HasFlag(Perm.Read) ? "r" : "-") + (p.HasFlag(Perm.Write) ? "w" : "-") + (p.HasFlag(Perm.Execute) ? "x" : "-");

Perm FromText(string s)
{
    if (s.Length != 3) throw new FormatException($"bad permission \"{s}\"");
    var result = Perm.None;
    (char letter, Perm flag)[] slots = [('r', Perm.Read), ('w', Perm.Write), ('x', Perm.Execute)];
    for (int i = 0; i < 3; i++)
    {
        if (s[i] == slots[i].letter) result |= slots[i].flag;
        else if (s[i] != '-') throw new FormatException($"bad permission \"{s}\"");
    }
    return result;
}

string Octal(Perm owner, Perm group, Perm other) => $"{(int)owner}{(int)group}{(int)other}";

[Flags]
enum Perm { None = 0, Read = 4, Write = 2, Execute = 1 }

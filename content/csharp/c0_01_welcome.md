@@lesson c0-welcome Welcome to C#
@@concepts hello, variables
@@text
## Why C#?

C# (say "C sharp") is Microsoft's main language. It runs on **.NET**, which runs on Windows, Linux and macOS. You'll meet it all over IT:

- **Business software.** In Japan especially, a lot of in-house systems (業務システム) at banks, manufacturers and SIers are written in C#/.NET.
- **Web back ends** with ASP.NET Core, and **cloud** services on Azure.
- **Windows tools and automation.** PowerShell itself is built on .NET, so anything you learn here about .NET types works in PowerShell too.
- **Games**: Unity uses C#.

C# is a *compiled*, *statically typed* language. Every variable has a type that the compiler checks before the program runs, so many mistakes are caught before your users ever see them.

## Your first program

```csharp
Console.WriteLine("Hello, world!");
Console.WriteLine("こんにちは、IT Quest!");
```

That's a whole program. Modern C# lets you write statements at the top of the file ("top-level statements"); the compiler wraps them in a `Main` method for you. Older code you'll see at work looks like this, and means the same:

```csharp
using System;

namespace HelloApp
{
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("Hello, world!");
        }
    }
}
```

The app adds the usual `using` lines for you (`System`, `System.Linq`, `System.Collections.Generic`, `System.IO` and a few more), just as a new `dotnet new console` project does.

## Statements and semicolons

Every statement ends with `;`. Blocks use `{ }`. C# doesn't care about indentation, but people do: use four spaces.

```csharp
Console.Write("No new line... ");
Console.WriteLine("then a new line.");
Console.WriteLine();          // an empty line
Console.WriteLine(6 * 7);
```

## Variables

```csharp
string host = "web01";
int port = 443;
double cpu = 73.5;
bool online = true;

Console.WriteLine($"{host}:{port} cpu={cpu}% online={online}");
```

The `$"..."` string is an **interpolated string**: anything in `{ }` is worked out and put into the text. You'll use it constantly.

`var` lets the compiler work out the type from the value. The variable still has a fixed type: you just don't write it.

```csharp
var server = "db01";    // string
var cores = 8;          // int
// cores = "eight";     // error CS0029: cannot convert string to int
cores = cores * 2;
Console.WriteLine($"{server} has {cores} logical cores");
```

`const` is for values that never change:

```csharp
const int MaxRetries = 3;
Console.WriteLine($"Retrying up to {MaxRetries} times");
```

## Reading compiler errors

C# errors have a code like `CS1002` and a line number. Search the code online if the message isn't clear: Microsoft documents every one.

```csharp,norun
Console.WriteLine("missing semicolon")
```

```text
main.cs(1,39): error CS1002: ; expected
```

`(1,39)` means line 1, column 39. Fix the first error first: later ones are often caused by it.

@@quiz
What does `var count = 5;` give `count`?
* The type `int`, worked out by the compiler, and it stays `int`
- No type: it can hold anything later
- The type `object`
- The type `string`
? `var` is not "any type". The compiler infers the type once, from the value, and checks it from then on.

@@quiz
What does `Console.WriteLine($"{2 + 3} servers");` print?
* `5 servers`
- `{2 + 3} servers`
- `2 + 3 servers`
- an error
? `$` makes it an interpolated string: the expression inside `{ }` is evaluated.

@@quiz
The compiler reports `main.cs(4,12): error CS0103: The name 'hostname' does not exist`. Where should you look?
* Line 4, column 12: probably a typo, or a variable used before it was declared
- Line 12, column 4
- The last line of the file
- It's a runtime error, so in the output
? C# errors are `(line,column)`. CS0103 usually means a misspelling or a missing declaration.

@@flashcards
`Console.WriteLine(x)` :: print x and a new line
`Console.Write(x)` :: print x without a new line
`$"Port {port}"` :: interpolated string
`var` :: let the compiler infer the (fixed) type
`const` :: a value that can never change
`.NET` :: the runtime and libraries C# runs on (Windows, Linux, macOS)

@@exercise Server card
Print exactly these three lines, using variables `host`, `ip` and `ram` (in GB) and interpolated strings:

```text
Host: app01
IP:   10.0.0.15
RAM:  16 GB
```
@@starter
string host = "app01";
// add ip and ram, then print the three lines

@@expected
Host: app01
IP:   10.0.0.15
RAM:  16 GB
@@requires
$"
@@hint
`int ram = 16;` and then `Console.WriteLine($"RAM:  {ram} GB");`. Watch the spaces after `IP:` and `RAM:`.
@@solution
string host = "app01";
string ip = "10.0.0.15";
int ram = 16;
Console.WriteLine($"Host: {host}");
Console.WriteLine($"IP:   {ip}");
Console.WriteLine($"RAM:  {ram} GB");

@@exercise Fix the build
This program doesn't compile. Fix both errors so it prints `Backups kept: 7`.
@@starter
int days = 7
string label = "Backups kept: ";
Console.WriteLine(label + Days);
@@expected
Backups kept: 7
@@hint
One statement is missing its `;`. C# is case-sensitive: `Days` and `days` are different names.
@@solution
int days = 7;
string label = "Backups kept: ";
Console.WriteLine(label + days);

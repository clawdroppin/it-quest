@@lesson c4-professional Professional .NET
@@concepts modules, testing
@@text
## The dotnet CLI

Real C# lives in **projects** (`.csproj` files) grouped in **solutions** (`.sln`). The `dotnet` command does everything:

```bash,norun
dotnet new console -n AssetTool     # a new console app
dotnet new webapi -n TicketApi      # a REST API (ASP.NET Core)
dotnet new xunit -n AssetTool.Tests # a test project
dotnet add package Serilog          # add a NuGet package
dotnet build                        # compile
dotnet run -- --verbose             # run, passing arguments after --
dotnet test                         # run the tests
dotnet publish -c Release -r linux-x64 --self-contained   # one folder to copy to a server
```

A minimal `.csproj`:

```xml
<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <OutputType>Exe</OutputType>
    <TargetFramework>net8.0</TargetFramework>
    <Nullable>enable</Nullable>
    <ImplicitUsings>enable</ImplicitUsings>
  </PropertyGroup>
  <ItemGroup>
    <PackageReference Include="Serilog" Version="4.0.0" />
  </ItemGroup>
</Project>
```

**NuGet** (nuget.org) is .NET's package registry, like npm or PyPI. Check a package's downloads, maintainer and licence before adding it: every dependency is code you trust.

**LTS versions**: .NET 8 and .NET 10 are long-term support (3 years). Companies, especially in Japan, prefer LTS and upgrade slowly; you'll also meet old **.NET Framework 4.x** apps that only run on Windows.

## Namespaces and files

Each file usually holds one public type, in a namespace matching the folder:

```csharp,norun
namespace AssetTool.Models;       // file-scoped namespace: applies to the whole file

public record Asset(string Tag, int Year, string Owner);
```

`internal` (the default for top-level types) means "visible inside this project only": a good default for things other projects shouldn't depend on.

## Testing with xUnit

```csharp,norun
public class UptimeTests
{
    [Fact]
    public void FormatsDaysHoursMinutes() =>
        Assert.Equal("3d 4h 5m", Uptime.Format(273_900));

    [Theory]
    [InlineData(42, "0m")]
    [InlineData(300, "5m")]
    public void ShortDurations(long seconds, string expected) =>
        Assert.Equal(expected, Uptime.Format(seconds));
}
```

`[Fact]` is one test; `[Theory]` runs once per `[InlineData]`. `dotnet test` finds and runs them. NUnit and MSTest are the alternatives you'll see in older code.

## Dependency injection

ASP.NET Core and most modern .NET apps build objects through a DI container: you register interfaces and implementations once, and constructors just ask for what they need.

```csharp,norun
builder.Services.AddSingleton<INotifier, SlackNotifier>();
builder.Services.AddScoped<ITicketRepository, SqlTicketRepository>();

public class TicketService(ITicketRepository repo, INotifier notifier) { ... }
```

Tests can then pass fakes (`new FakeNotifier()`), and swapping Slack for Teams is one line.

## A tiny Web API

```csharp,norun
var builder = WebApplication.CreateBuilder(args);
var app = builder.Build();

var tickets = new List<Ticket>();
app.MapGet("/health", () => Results.Ok(new { status = "ok" }));
app.MapGet("/tickets", () => tickets);
app.MapPost("/tickets", (Ticket t) => { tickets.Add(t); return Results.Created($"/tickets/{t.Id}", t); });
app.Run();

record Ticket(int Id, string Title, int Priority);
```

That's a working REST API (`dotnet new web`, then `dotnet run`). Entity Framework Core adds a database with LINQ queries.

## Habits of a professional

- Turn on `<Nullable>enable</Nullable>` and `<TreatWarningsAsErrors>true</TreatWarningsAsErrors>` in new projects.
- Log with `ILogger` (or Serilog), with levels, never with `Console.WriteLine` in services.
- Keep secrets out of code and `appsettings.json`: use environment variables, `dotnet user-secrets` in development, and Key Vault in Azure.
- Prevent SQL injection with parameters (EF Core and Dapper do this for you), never string concatenation.
- `dotnet format` keeps style consistent; analyzers catch bugs at build time.

@@quiz
Which command runs every xUnit test in a solution?
* `dotnet test`
- `dotnet run --test`
- `dotnet xunit`
- `dotnet build --tests`
? `dotnet test` builds the test projects and runs them.

@@quiz
What does `[Theory]` with several `[InlineData(...)]` do?
* Runs the same test once per data row
- Marks a test as unfinished
- Runs the test in parallel threads
- Skips the test
? Theories are parameterized tests. `[Fact]` is a test with no parameters.

@@quiz
A config file contains a database password. Where should it go instead?
* An environment variable or a secret store (user-secrets, Key Vault)
- A `const` in the code
- A comment in `Program.cs`
- The README
? Anything in code or committed config ends up in Git history.

@@drill
Which attribute marks an xUnit test method?
= [Fact]
= Fact
? [Theory] runs one test with many inputs.

@@flashcards
`dotnet new console` :: create a console app project
`dotnet add package X` :: add a NuGet package
`dotnet publish -r linux-x64` :: build for deployment on Linux
`.csproj` :: a project file (target, packages, settings)
`[Fact]` :: an xUnit test
`AddScoped<IRepo, SqlRepo>()` :: register a service for dependency injection
`ILogger<T>` :: .NET's logging interface
`LTS` :: long-term support release (.NET 8, 10)

@@exercise Fake for a test
Code that sends alerts takes an `INotifier`. Write a `FakeNotifier` that **records** messages in a public `List<string> Sent` instead of sending them, so the "test" below passes and prints `PASS: 2 alerts`.
@@starter
var fake = new FakeNotifier();
var monitor = new DiskMonitor(fake);
monitor.Check("C:", 95);
monitor.Check("D:", 40);
monitor.Check("E:", 91);

Console.WriteLine(fake.Sent.Count == 2 && fake.Sent[1] == "E: is 91% full"
    ? $"PASS: {fake.Sent.Count} alerts"
    : $"FAIL: {string.Join(" | ", fake.Sent)}");

interface INotifier { void Send(string message); }

class DiskMonitor(INotifier notifier)
{
    public void Check(string drive, int percent)
    {
        if (percent >= 90) notifier.Send($"{drive} is {percent}% full");
    }
}

// write FakeNotifier here
class FakeNotifier
{
    public List<string> Sent { get; } = new();
}
@@expected
PASS: 2 alerts
@@hint
`FakeNotifier` must implement the interface: `class FakeNotifier : INotifier`, with `public void Send(string message) => Sent.Add(message);`.
@@solution
var fake = new FakeNotifier();
var monitor = new DiskMonitor(fake);
monitor.Check("C:", 95);
monitor.Check("D:", 40);
monitor.Check("E:", 91);

Console.WriteLine(fake.Sent.Count == 2 && fake.Sent[1] == "E: is 91% full"
    ? $"PASS: {fake.Sent.Count} alerts"
    : $"FAIL: {string.Join(" | ", fake.Sent)}");

interface INotifier { void Send(string message); }

class DiskMonitor(INotifier notifier)
{
    public void Check(string drive, int percent)
    {
        if (percent >= 90) notifier.Send($"{drive} is {percent}% full");
    }
}

class FakeNotifier : INotifier
{
    public List<string> Sent { get; } = new();
    public void Send(string message) => Sent.Add(message);
}

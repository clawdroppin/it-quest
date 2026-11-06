@@lesson c5-architecture Application Architecture in .NET
@@text
C# is the language of **business applications**: banking, insurance, logistics, HR, the 業務システム that Japanese SIers build for their clients. These systems live for ten or twenty years, so the code is organized to survive change. This lesson is that organization.

## A solution, not a project

A real application is a **solution** (`.sln`) of several projects, each a layer. The arrows are project references, and they only point inward:

```text
MyApp.sln
├── src/
│   ├── MyApp.Domain/           entities, value objects, rules       (references nothing)
│   ├── MyApp.Application/      use cases, interfaces it needs        → Domain
│   ├── MyApp.Infrastructure/   EF Core, files, email, other APIs     → Application
│   └── MyApp.Web/              ASP.NET Core API or Blazor UI         → Application, Infrastructure
└── tests/
    ├── MyApp.Domain.Tests/
    └── MyApp.Application.Tests/
```

```text
dotnet new sln -n MyApp
dotnet new classlib -o src/MyApp.Domain
dotnet new classlib -o src/MyApp.Application
dotnet add src/MyApp.Application reference src/MyApp.Domain
dotnet new webapi -o src/MyApp.Web
dotnet new xunit -o tests/MyApp.Application.Tests
dotnet sln add src/**/*.csproj tests/**/*.csproj
```

Because `Domain` references nothing, the compiler itself stops anyone from using the database inside an entity.

## Entities and value objects

- An **entity** has an identity that stays the same while its data changes: invoice #42 is still #42 after it's paid.
- A **value object** is defined only by its values, and is immutable: ¥1,000 is ¥1,000. C# **records** are made for this.

```csharp
var price = new Money(1200, "JPY");
var total = price * 3 + new Money(400, "JPY");
Console.WriteLine(total);
Console.WriteLine(total == new Money(4000, "JPY"));   // records compare by value
try
{
    var bad = price + new Money(10, "USD");
}
catch (InvalidOperationException e)
{
    Console.WriteLine(e.Message);
}

public record Money(decimal Amount, string Currency)
{
    public static Money operator +(Money a, Money b) =>
        a.Currency == b.Currency ? a with { Amount = a.Amount + b.Amount } : throw new InvalidOperationException($"can't add {a.Currency} and {b.Currency}");
    public static Money operator *(Money a, int n) => a with { Amount = a.Amount * n };
    public override string ToString() => Currency == "JPY" ? $"¥{Amount:N0}" : $"{Amount:N2} {Currency}";
}
```

Use `decimal` for money, never `double`: `0.1 + 0.2` is not `0.3` in binary floating point.

## Rich entities: rules inside

Don't make entities bags of public setters that any code can put into a wrong state. Put the rules in methods:

```csharp
var leave = new LeaveRequest("yuki", 3);
leave.Approve("manager-ken");
Console.WriteLine($"{leave.Status} by {leave.ApprovedBy}");
try
{
    leave.Approve("someone-else");
}
catch (InvalidOperationException e)
{
    Console.WriteLine(e.Message);
}

public class LeaveRequest(string employee, int days)
{
    public string Employee { get; } = employee;
    public int Days { get; } = days > 0 ? days : throw new ArgumentOutOfRangeException(nameof(days));
    public string Status { get; private set; } = "Pending";      // private set: only methods change it
    public string? ApprovedBy { get; private set; }

    public void Approve(string manager)
    {
        if (Status != "Pending") throw new InvalidOperationException($"can't approve a request that is {Status}");
        if (manager == Employee) throw new InvalidOperationException("you can't approve your own leave");
        Status = "Approved";
        ApprovedBy = manager;
    }
}
```

## Interfaces and dependency injection

The application layer declares the interfaces it needs; infrastructure implements them; the **DI container** connects them at startup. In ASP.NET Core it's built in:

```csharp,norun
// Program.cs of the web project: the composition root
var builder = WebApplication.CreateBuilder(args);
builder.Services.AddDbContext<AppDbContext>(o => o.UseSqlServer(builder.Configuration.GetConnectionString("Main")));
builder.Services.AddScoped<IInvoiceRepository, EfInvoiceRepository>();   // one per HTTP request
builder.Services.AddSingleton<IClock, SystemClock>();                    // one for the whole app
builder.Services.AddScoped<InvoiceService>();                            // gets its dependencies injected
var app = builder.Build();
```

| Lifetime | One instance per... | Typical use |
|---|---|---|
| `AddSingleton` | application | clocks, caches, configuration |
| `AddScoped` | request | `DbContext`, repositories, services using them |
| `AddTransient` | every injection | small, stateless helpers |

The classic bug: a singleton that holds a scoped service (the `DbContext` then lives forever and is shared between requests). ASP.NET Core detects this in development.

## Configuration and logging

```csharp,norun
// appsettings.json, appsettings.Production.json, environment variables, user secrets: merged in that order
public class SmtpOptions
{
    public required string Host { get; init; }
    public int Port { get; init; } = 587;
}

builder.Services.AddOptions<SmtpOptions>()
    .Bind(builder.Configuration.GetSection("Smtp"))
    .Validate(o => o.Port is > 0 and < 65536, "Smtp:Port is invalid")
    .ValidateOnStart();                                   // fail at startup, not at the first email

public class Mailer(IOptions<SmtpOptions> options, ILogger<Mailer> log)
{
    public void Send(string to)
    {
        log.LogInformation("Sending mail to {Recipient} via {Host}", to, options.Value.Host);  // structured logging
    }
}
```

Secrets never go in `appsettings.json` (it's in git): use `dotnet user-secrets` in development and environment variables or Azure Key Vault in production.

## Testing

```csharp,norun
public class LeaveRequestTests
{
    [Fact]
    public void Cannot_approve_own_leave()
    {
        var leave = new LeaveRequest("yuki", 3);
        var ex = Assert.Throws<InvalidOperationException>(() => leave.Approve("yuki"));
        Assert.Equal("you can't approve your own leave", ex.Message);
    }

    [Theory]
    [InlineData(0)]
    [InlineData(-1)]
    public void Days_must_be_positive(int days) =>
        Assert.Throws<ArgumentOutOfRangeException>(() => new LeaveRequest("yuki", days));
}
```

`dotnet test` runs them. **xUnit** is the most common framework (NUnit and MSTest are also used); **Moq** or **NSubstitute** create fakes, though hand-written fakes are often clearer.

@@quiz
In a Clean Architecture solution, which project may reference Entity Framework Core?
- Domain
- Application
* Infrastructure
- All of them
? Only infrastructure knows the database. Domain and Application stay independent, so they're easy to test and to change.

@@quiz
Why is a C# `record` a good fit for a value object like `Money`?
* It's immutable by default and compares by value
- It's faster than a class
- It can't have methods
- It's stored on the stack
? Two `Money(1000, "JPY")` values are equal. `with` creates changed copies.

@@quiz
A service registered with `AddSingleton` takes a `DbContext` (registered as scoped) in its constructor. What's wrong?
- Nothing
* The DbContext would live as long as the app and be shared by all requests
- Singletons can't have constructors
- The app will use more threads
? This "captive dependency" causes data leaking between requests and threading errors. Make the service scoped too.

@@quiz
Why use `decimal` rather than `double` for prices?
- decimal is faster
* decimal stores base-10 values exactly, so ¥0.1 + ¥0.2 is exactly 0.3
- double can't hold large numbers
- decimal uses less memory
? Floating-point binary fractions are approximations. Money must be exact.

@@drill
What's the name of an object defined by its values rather than an identity, such as Money?
= value object
= a value object
? Two value objects with the same values are equal.

@@flashcards
Clean Architecture :: Domain ← Application ← Infrastructure / Web; dependencies point inward
entity :: has an identity that persists while its data changes
value object :: defined by its values, immutable: a good use for `record`
`private set` :: only the class's own methods can change the property
composition root :: `Program.cs` registering services with `builder.Services`
`AddSingleton` / `AddScoped` / `AddTransient` :: one per app / per request / per injection
captive dependency :: a singleton holding a scoped service: a bug
`IOptions<T>` + `ValidateOnStart` :: typed, validated configuration
`ILogger<T>` with `{Placeholders}` :: structured logging
xUnit `[Fact]` / `[Theory]` + `[InlineData]` :: a test / a test with several inputs

@@exercise A value object
Write a record `EmailAddress` with a static method `Create(string text)` that trims and lower-cases the text, and throws `ArgumentException("invalid email: <text>")` (with the original text) unless it has exactly one `@`, something before it, and a `.` after it that isn't the last character. `ToString()` returns the address. Equal addresses must compare equal.
@@starter
Console.WriteLine(EmailAddress.Create("  Yuki@Example.JP "));
Console.WriteLine(EmailAddress.Create("yuki@example.jp") == EmailAddress.Create("YUKI@example.jp"));
foreach (var bad in new[] { "yuki", "@example.jp", "a@b@c.jp", "yuki@example." })
{
    try { EmailAddress.Create(bad); Console.WriteLine("accepted?!"); }
    catch (ArgumentException e) { Console.WriteLine(e.Message); }
}

public record EmailAddress(string Value)
{
    // TODO: Create and ToString
}
@@expected
yuki@example.jp
True
invalid email: yuki
invalid email: @example.jp
invalid email: a@b@c.jp
invalid email: yuki@example.
@@hint
`var v = text.Trim().ToLowerInvariant(); var parts = v.Split('@');` Valid when `parts.Length == 2 && parts[0].Length > 0 && parts[1].Contains('.') && !parts[1].EndsWith('.')`. Return `new EmailAddress(v)`. Override `public override string ToString() => Value;`.
@@solution
Console.WriteLine(EmailAddress.Create("  Yuki@Example.JP "));
Console.WriteLine(EmailAddress.Create("yuki@example.jp") == EmailAddress.Create("YUKI@example.jp"));
foreach (var bad in new[] { "yuki", "@example.jp", "a@b@c.jp", "yuki@example." })
{
    try { EmailAddress.Create(bad); Console.WriteLine("accepted?!"); }
    catch (ArgumentException e) { Console.WriteLine(e.Message); }
}

public record EmailAddress(string Value)
{
    public static EmailAddress Create(string text)
    {
        var v = text.Trim().ToLowerInvariant();
        var parts = v.Split('@');
        bool ok = parts.Length == 2 && parts[0].Length > 0 && parts[1].Contains('.') && !parts[1].EndsWith('.');
        return ok ? new EmailAddress(v) : throw new ArgumentException($"invalid email: {text}");
    }

    public override string ToString() => Value;
}

@@exercise A rich entity
Write the `Ticket` entity's methods so the program prints the expected output. Rules: `Assign(person)` works only when the status is `Open` or `Assigned`; `Resolve(note)` only when `Assigned`, and the note must not be blank; `Reopen()` only when `Resolved`, and sets the status back to `Assigned`. Any other call throws `InvalidOperationException("can't <action> a <Status> ticket")` with the action in lower case.
@@starter
var t = new Ticket(4410, "VPN drops");
t.Assign("yuki");
t.Assign("ken");
Console.WriteLine($"{t.Status} {t.Assignee}");
TryIt(() => t.Reopen());
t.Resolve("replaced router firmware");
Console.WriteLine($"{t.Status}: {t.Resolution}");
TryIt(() => t.Assign("aiko"));
t.Reopen();
Console.WriteLine(t.Status);
TryIt(() => t.Resolve("  "));

void TryIt(Action a)
{
    try { a(); }
    catch (Exception e) when (e is InvalidOperationException or ArgumentException) { Console.WriteLine(e.Message); }
}

public class Ticket(int id, string title)
{
    public int Id { get; } = id;
    public string Title { get; } = title;
    public string Status { get; private set; } = "Open";
    public string? Assignee { get; private set; }
    public string? Resolution { get; private set; }

    // TODO: Assign, Resolve, Reopen
}
@@expected
Assigned ken
can't reopen a Assigned ticket
Resolved: replaced router firmware
can't assign a Resolved ticket
Assigned
a resolution note is required
@@hint
`public void Assign(string person) { if (Status is not ("Open" or "Assigned")) throw new InvalidOperationException($"can't assign a {Status} ticket"); Assignee = person; Status = "Assigned"; }`. In `Resolve`, check the status first, then `if (string.IsNullOrWhiteSpace(note)) throw new ArgumentException("a resolution note is required");`.
@@solution
var t = new Ticket(4410, "VPN drops");
t.Assign("yuki");
t.Assign("ken");
Console.WriteLine($"{t.Status} {t.Assignee}");
TryIt(() => t.Reopen());
t.Resolve("replaced router firmware");
Console.WriteLine($"{t.Status}: {t.Resolution}");
TryIt(() => t.Assign("aiko"));
t.Reopen();
Console.WriteLine(t.Status);
TryIt(() => t.Resolve("  "));

void TryIt(Action a)
{
    try { a(); }
    catch (Exception e) when (e is InvalidOperationException or ArgumentException) { Console.WriteLine(e.Message); }
}

public class Ticket(int id, string title)
{
    public int Id { get; } = id;
    public string Title { get; } = title;
    public string Status { get; private set; } = "Open";
    public string? Assignee { get; private set; }
    public string? Resolution { get; private set; }

    public void Assign(string person)
    {
        if (Status is not ("Open" or "Assigned")) throw new InvalidOperationException($"can't assign a {Status} ticket");
        Assignee = person;
        Status = "Assigned";
    }

    public void Resolve(string note)
    {
        if (Status != "Assigned") throw new InvalidOperationException($"can't resolve a {Status} ticket");
        if (string.IsNullOrWhiteSpace(note)) throw new ArgumentException("a resolution note is required");
        Resolution = note.Trim();
        Status = "Resolved";
    }

    public void Reopen()
    {
        if (Status != "Resolved") throw new InvalidOperationException($"can't reopen a {Status} ticket");
        Status = "Assigned";
    }
}

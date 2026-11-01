@@lesson c5-data-web Data and Web APIs: EF Core and ASP.NET Core
@@text
Most C# applications are a **web API** in front of a **database**. In .NET that means **ASP.NET Core** and **Entity Framework Core**. Both need packages and a running server, so the big examples here are for your own computer; the exercises practise the same ideas with in-memory data.

## LINQ: the query language you already have

EF Core turns LINQ into SQL, so good LINQ is good database code. A reminder of the operators you'll use most, on in-memory data:

```csharp
var orders = new List<Order>
{
    new(1, "Sakura Cafe", "tokyo", 45000, new(2026, 10, 3)),
    new(2, "Kaede Dental", "osaka", 30000, new(2026, 10, 9)),
    new(3, "Sakura Cafe", "tokyo", 12000, new(2026, 10, 21)),
    new(4, "Momiji Hotel", "kyoto", 98000, new(2026, 11, 1)),
};

var byCustomer = orders
    .Where(o => o.Date.Month == 10)
    .GroupBy(o => o.Customer)
    .Select(g => new { Customer = g.Key, Count = g.Count(), Total = g.Sum(o => o.Amount) })
    .OrderByDescending(x => x.Total);

foreach (var row in byCustomer)
    Console.WriteLine($"{row.Customer,-14} {row.Count} order(s) ¥{row.Total:N0}");

var page2 = orders.OrderBy(o => o.Id).Skip(2).Take(2).Select(o => o.Id);   // paging
Console.WriteLine(string.Join(",", page2));

record Order(int Id, string Customer, string Region, decimal Amount, DateOnly Date);
```

## Entity Framework Core

EF Core maps classes to tables and LINQ to SQL:

```csharp,norun
public class AppDbContext(DbContextOptions<AppDbContext> options) : DbContext(options)
{
    public DbSet<Customer> Customers => Set<Customer>();
    public DbSet<Invoice> Invoices => Set<Invoice>();

    protected override void OnModelCreating(ModelBuilder model)
    {
        model.Entity<Invoice>().Property(i => i.Number).HasMaxLength(20);
        model.Entity<Invoice>().HasIndex(i => i.Number).IsUnique();
        model.Entity<Invoice>().HasOne(i => i.Customer).WithMany(c => c.Invoices);
    }
}

// A query: EF Core writes a parameterized SQL SELECT with a JOIN.
var overdue = await db.Invoices
    .Where(i => i.Status == InvoiceStatus.Issued && i.DueDate < today)
    .Include(i => i.Customer)                // load the customer in the same query (avoids N+1)
    .OrderBy(i => i.DueDate)
    .AsNoTracking()                          // read-only: faster
    .ToListAsync();

// A change: load, modify, save. SaveChanges runs one transaction.
var invoice = await db.Invoices.FindAsync(id) ?? throw new NotFoundException();
invoice.MarkPaid(today);
await db.SaveChangesAsync();
```

**Migrations** keep the database schema in step with your classes:

```text
dotnet tool install --global dotnet-ef
dotnet ef migrations add AddDueDate        # generates a C# migration from your model changes
dotnet ef database update                  # applies pending migrations
dotnet ef migrations script --idempotent   # a SQL script for the DBA / production
```

Common traps: forgetting `Include` (N+1 queries or null navigation properties), calling `.ToList()` too early (filtering then happens in memory, after loading the whole table), and long-lived `DbContext`s (keep one per request).

**Dapper** is the popular alternative when you'd rather write the SQL yourself: `await conn.QueryAsync<Invoice>("SELECT * FROM Invoices WHERE Status = @status", new { status })`.

## ASP.NET Core minimal APIs

```csharp,norun
var builder = WebApplication.CreateBuilder(args);
builder.Services.AddDbContext<AppDbContext>(o => o.UseSqlite("Data Source=app.db"));
builder.Services.AddScoped<InvoiceService>();
builder.Services.AddProblemDetails();          // standard JSON error bodies (RFC 9457)
var app = builder.Build();

var invoices = app.MapGroup("/api/invoices");

invoices.MapGet("/", async (AppDbContext db, string? status) =>
    await db.Invoices.Where(i => status == null || i.Status.ToString() == status).ToListAsync());

invoices.MapGet("/{id:int}", async (int id, AppDbContext db) =>
    await db.Invoices.FindAsync(id) is { } invoice ? Results.Ok(invoice) : Results.NotFound());

invoices.MapPost("/", async (CreateInvoice body, InvoiceService service) =>
{
    var errors = body.Validate();
    if (errors.Count > 0) return Results.ValidationProblem(errors);
    var invoice = await service.CreateAsync(body);
    return Results.Created($"/api/invoices/{invoice.Id}", invoice);
});

invoices.MapPost("/{id:int}/issue", async (int id, InvoiceService service) => Results.Ok(await service.IssueAsync(id)))
    .RequireAuthorization("Accounting");

app.Run();
```

The parameters of each handler are filled in automatically: route values (`id`), query strings (`status`), the JSON body (`body`), and services from DI (`db`, `service`). For bigger APIs, **controllers** (`[ApiController]` classes) organize the same thing. `/openapi/v1.json` and Swagger UI or Scalar document the API.

## Middleware and authentication

```csharp,norun
app.UseExceptionHandler();             // unhandled exceptions → 500 ProblemDetails, no stack traces
app.UseHttpsRedirection();
app.UseAuthentication();                // who are you? (cookies, JWT bearer, Entra ID / OpenID Connect)
app.UseAuthorization();                 // may you? ([Authorize], policies, roles)
```

Companies running Microsoft 365 usually sign users in with **Microsoft Entra ID** (formerly Azure AD): `builder.Services.AddAuthentication().AddMicrosoftIdentityWebApi(...)`. Then `RequireAuthorization("Accounting")` checks a group or role claim.

## async all the way

Database and HTTP calls are I/O: use the `Async` methods and `await` them, all the way up to the endpoint. Never `.Result` or `.Wait()` on a task in a web app: it blocks a thread and can deadlock. Pass the `CancellationToken` through, so work stops when the client disconnects.

@@quiz
What does `.Include(i => i.Customer)` do in an EF Core query?
- Adds a customer to the database
* Loads each invoice's customer in the same SQL query
- Filters invoices that have a customer
- Makes the query read-only
? Without it you either get null customers or one extra query per invoice (N+1).

@@quiz
Where does the filtering happen in `db.Invoices.ToList().Where(i => i.Total > 100000)`?
- In the database, with a WHERE clause
* In memory, after loading every invoice from the database
- It doesn't compile
- In a background thread
? `ToList()` runs the query. Put `Where` before it, so EF Core can translate it to SQL.

@@quiz
In a minimal API handler `(int id, CreateInvoice body, InvoiceService service) => ...`, where does `service` come from?
- The query string
- The JSON body
* The dependency injection container
- A static field
? ASP.NET Core binds route values, query strings and bodies, and resolves registered services from DI.

@@quiz
Why avoid `.Result` on tasks in ASP.NET Core?
* It blocks a thread while waiting, which wastes the server's capacity and can deadlock
- It returns null
- It's slower to type
- It disables logging
? Use `await`. Async I/O lets a few threads serve thousands of requests.

@@flashcards
`GroupBy(...).Select(g => new { g.Key, Total = g.Sum(...) })` :: group and aggregate with LINQ
`Skip(n).Take(m)` :: paging
`DbContext` / `DbSet<T>` :: EF Core's unit of work / a table
`Include` :: load related data in the same query
`AsNoTracking()` :: read-only query, faster
`dotnet ef migrations add Name` :: generate a migration from model changes
Dapper :: write SQL yourself, map rows to objects
`app.MapGet/MapPost` :: minimal API endpoints
`Results.Created / NotFound / ValidationProblem` :: typed HTTP results
ProblemDetails :: standard JSON error format (RFC 9457)
Microsoft Entra ID :: Microsoft's identity platform, formerly Azure AD
`CancellationToken` :: lets work stop when the request is aborted

@@exercise A report with LINQ
Print the total sales per region for October 2026, highest first, formatted `<region>: ¥<total with thousands separators> (<count> orders)`, then the best customer overall (by total across all dates).
@@starter
var orders = new List<Order>
{
    new(1, "Sakura Cafe", "tokyo", 45000, new(2026, 10, 3)),
    new(2, "Kaede Dental", "osaka", 30000, new(2026, 10, 9)),
    new(3, "Sakura Cafe", "tokyo", 12000, new(2026, 10, 21)),
    new(4, "Momiji Hotel", "kyoto", 98000, new(2026, 11, 1)),
    new(5, "Kaede Dental", "osaka", 41000, new(2026, 10, 28)),
    new(6, "Fuji Logistics", "tokyo", 5000, new(2026, 9, 30)),
};

// TODO

record Order(int Id, string Customer, string Region, decimal Amount, DateOnly Date);
@@expected
osaka: ¥71,000 (2 orders)
tokyo: ¥57,000 (2 orders)
best customer: Momiji Hotel
@@requires
GroupBy
@@hint
`orders.Where(o => o.Date.Year == 2026 && o.Date.Month == 10).GroupBy(o => o.Region).Select(g => new { g.Key, Total = g.Sum(o => o.Amount), Count = g.Count() }).OrderByDescending(x => x.Total)`. Format with `{x.Total:N0}`. For the best customer: group by customer, order by the sum descending, `.First().Key`.
@@solution
var orders = new List<Order>
{
    new(1, "Sakura Cafe", "tokyo", 45000, new(2026, 10, 3)),
    new(2, "Kaede Dental", "osaka", 30000, new(2026, 10, 9)),
    new(3, "Sakura Cafe", "tokyo", 12000, new(2026, 10, 21)),
    new(4, "Momiji Hotel", "kyoto", 98000, new(2026, 11, 1)),
    new(5, "Kaede Dental", "osaka", 41000, new(2026, 10, 28)),
    new(6, "Fuji Logistics", "tokyo", 5000, new(2026, 9, 30)),
};

var october = orders
    .Where(o => o.Date.Year == 2026 && o.Date.Month == 10)
    .GroupBy(o => o.Region)
    .Select(g => new { Region = g.Key, Total = g.Sum(o => o.Amount), Count = g.Count() })
    .OrderByDescending(x => x.Total);

foreach (var x in october)
    Console.WriteLine($"{x.Region}: ¥{x.Total:N0} ({x.Count} orders)");

var best = orders.GroupBy(o => o.Customer).OrderByDescending(g => g.Sum(o => o.Amount)).First().Key;
Console.WriteLine($"best customer: {best}");

record Order(int Id, string Customer, string Region, decimal Amount, DateOnly Date);

@@exercise Endpoint logic without a server
Endpoints should be thin: validate, call the service, map the result to a status. Write `Handle(CreateUser body, Dictionary<string, string> users)` that returns `(int Status, string Body)`:

- validation errors (all of them, joined with `"; "`) → `400`: name required (non-blank) → `"name is required"`; email must contain `@` → `"email is invalid"`,
- an email already in `users` (keys are emails, compare lower-case) → `409`, `"email already registered"`,
- otherwise add `email.ToLower() → name.Trim()` and return `201`, `"/api/users/<lower-case email>"`.
@@starter
var users = new Dictionary<string, string> { ["ken@example.jp"] = "Ken" };
Console.WriteLine(Handle(new CreateUser(" Yuki ", "Yuki@Example.JP"), users));
Console.WriteLine(Handle(new CreateUser("Yuki2", "yuki@example.jp"), users));
Console.WriteLine(Handle(new CreateUser(" ", "nope"), users));
Console.WriteLine(users["yuki@example.jp"]);

(int Status, string Body) Handle(CreateUser body, Dictionary<string, string> users)
{
    throw new NotImplementedException("TODO: not done yet");
}

record CreateUser(string Name, string Email);
@@expected
(201, /api/users/yuki@example.jp)
(409, email already registered)
(400, name is required; email is invalid)
Yuki
@@hint
Build a `List<string> errors`, add to it with two `if`s, and `if (errors.Count > 0) return (400, string.Join("; ", errors));`. Then `var email = body.Email.Trim().ToLowerInvariant();` and check `users.ContainsKey(email)`.
@@solution
var users = new Dictionary<string, string> { ["ken@example.jp"] = "Ken" };
Console.WriteLine(Handle(new CreateUser(" Yuki ", "Yuki@Example.JP"), users));
Console.WriteLine(Handle(new CreateUser("Yuki2", "yuki@example.jp"), users));
Console.WriteLine(Handle(new CreateUser(" ", "nope"), users));
Console.WriteLine(users["yuki@example.jp"]);

(int Status, string Body) Handle(CreateUser body, Dictionary<string, string> users)
{
    var errors = new List<string>();
    if (string.IsNullOrWhiteSpace(body.Name)) errors.Add("name is required");
    if (!body.Email.Contains('@')) errors.Add("email is invalid");
    if (errors.Count > 0) return (400, string.Join("; ", errors));
    var email = body.Email.Trim().ToLowerInvariant();
    if (users.ContainsKey(email)) return (409, "email already registered");
    users[email] = body.Name.Trim();
    return (201, $"/api/users/{email}");
}

record CreateUser(string Name, string Email);

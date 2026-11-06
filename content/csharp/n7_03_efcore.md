@@lesson n7-efcore Entity Framework Core in Depth: Change Tracking, Queries and Migrations
@@concepts objects, modules
@@text
EF Core maps C# classes to database tables. You query with LINQ, change objects, and call `SaveChanges`; EF works out the SQL. Knowing **how** it does that is what separates working apps from slow, buggy ones.

```csharp,norun
public class InventoryDb(DbContextOptions<InventoryDb> options) : DbContext(options)
{
    public DbSet<Host> Hosts => Set<Host>();
    public DbSet<Disk> Disks => Set<Disk>();

    protected override void OnModelCreating(ModelBuilder b)
    {
        b.Entity<Host>().HasIndex(h => h.Name).IsUnique();
        b.Entity<Host>().Property(h => h.Name).HasMaxLength(63);
        b.Entity<Disk>().HasOne(d => d.Host).WithMany(h => h.Disks).HasForeignKey(d => d.HostId);
    }
}

builder.Services.AddDbContext<InventoryDb>(o => o.UseNpgsql(builder.Configuration.GetConnectionString("Inventory")));
```

## Change tracking

A `DbContext` remembers every entity it loaded and a **snapshot** of its original values. `SaveChanges` compares current values with the snapshots and sends only what changed, in one transaction:

```csharp,norun
var host = await db.Hosts.SingleAsync(h => h.Name == "web01");
host.Env = "prod";                 // just a property change
db.Hosts.Add(new Host { Name = "web03" });
db.Hosts.Remove(oldHost);
await db.SaveChangesAsync();       // INSERT web03, UPDATE hosts SET env = @p0 WHERE id = @p1, DELETE ...
```

- Read-only queries should use `AsNoTracking()` (faster, less memory).
- `ExecuteUpdateAsync` / `ExecuteDeleteAsync` (EF 7+) run set-based SQL without loading entities.
- A `DbContext` is a short-lived **unit of work**: scoped per request, never a singleton, never shared across threads.

## Queries: what runs where

```csharp,norun
var report = await db.Hosts
    .Where(h => h.Env == "prod" && h.Disks.Any(d => d.UsedPercent > 90))  // translated to SQL
    .OrderBy(h => h.Name)
    .Select(h => new { h.Name, Worst = h.Disks.Max(d => d.UsedPercent) }) // only the needed columns
    .Skip(page * size).Take(size)                                          // paging in SQL
    .ToListAsync(ct);
```

Queries are `IQueryable<T>`: the lambdas are **expression trees** that EF translates to SQL. A method it can't translate throws at run time. `query.ToQueryString()` shows the SQL.

## The N+1 problem

```csharp,norun
foreach (var h in await db.Hosts.ToListAsync())
    Console.WriteLine(h.Disks.Count);           // with lazy loading: one extra query per host!

var hosts = await db.Hosts.Include(h => h.Disks).ToListAsync(); // one query (or AsSplitQuery for two)
```

Use `Include` or a projection with `Select`; watch the SQL log in Development.

## Migrations

```text
dotnet ef migrations add AddDiskTable     # C# migration generated from model changes
dotnet ef database update                 # apply to the database
dotnet ef migrations script --idempotent  # SQL script for production change review
```

Review generated migrations: renaming a property can turn into "drop column, add column", which loses data.

## Concurrency

Add a concurrency token (`[Timestamp] public byte[] Version` on SQL Server, `xmin` on PostgreSQL). If two users edit the same row, the second `SaveChanges` throws `DbUpdateConcurrencyException` instead of silently overwriting: **optimistic concurrency**.

Dapper (hand-written SQL mapped to objects) is the popular lightweight alternative when you want full control of the SQL.

@@quiz
How does EF Core know which columns to UPDATE?
* It compares tracked entities with snapshots of their original values
- It updates every column
- You list them manually
- It asks the database
? That's change tracking.

@@quiz
What fixes the N+1 query problem when listing hosts with their disks?
- Calling SaveChanges
* `Include(h => h.Disks)` or a `Select` projection, so the data comes in one query
- AsNoTracking
- More indexes
? N+1: one query for the list plus one per item.

@@quiz
Why shouldn't a DbContext be registered as a singleton?
- It's too big
* It's a unit of work that isn't thread-safe and would accumulate tracked entities forever
- EF forbids it at compile time
- Singletons can't be async
? Scoped per request is the norm.

@@drill
Which EF Core method loads related entities in the same query?
= Include
= Include()
= .Include
? Avoids the N+1 problem.

@@flashcards
`DbContext` :: unit of work + change tracker
`DbSet<T>` :: queryable table
`SaveChangesAsync` :: send tracked changes in a transaction
`AsNoTracking` :: read-only, faster queries
`Include` :: load related data in the same query
N+1 :: one query per item in a loop
`ToQueryString()` :: show generated SQL
`dotnet ef migrations add` :: create a schema migration
Optimistic concurrency :: detect conflicting edits with a version token
`ExecuteUpdateAsync` :: set-based update without loading

@@exercise A change tracker
Write a minimal change tracker for entities with an `int Id`:

- `Attach(T e)` snapshots the entity's public property values (reflection); `Add(T e)` and `Remove(T e)` mark entities as added/deleted
- `SaveChanges()` returns SQL statements in this order: INSERTs (all properties except `Id`), UPDATEs (**only the changed columns**), DELETEs; parameters are numbered `@p0, @p1...` across the whole batch, and every statement is printed with its parameter values
- after saving, snapshots are refreshed, added entities become tracked, deleted ones are forgotten; a second `SaveChanges` with no edits returns nothing

Table name: the class name plus `s`, lower-cased (`hosts`).
@@starter
using System.Reflection;

var web01 = new Host { Id = 1, Name = "web01", Env = "staging", Cores = 4 };
var db01 = new Host { Id = 2, Name = "db01", Env = "prod", Cores = 16 };
var old = new Host { Id = 3, Name = "old01", Env = "prod", Cores = 2 };
Console.WriteLine("TODO");

class Host
{
    public int Id { get; set; }
    public string Name { get; set; } = "";
    public string Env { get; set; } = "";
    public int Cores { get; set; }
}
@@expected
INSERT INTO hosts (Name, Env, Cores) VALUES (@p0, @p1, @p2) [web03, prod, 8]
UPDATE hosts SET Env = @p3, Cores = @p4 WHERE Id = @p5 [prod, 8, 1]
DELETE FROM hosts WHERE Id = @p6 [3]
second save: 0 statements
after rename: UPDATE hosts SET Name = @p0 WHERE Id = @p1 [web03-new, 4]
@@requires
GetProperties
@@hint
`class Tracker<T> where T : class`. Keep `Dictionary<T, object?[]> snapshots` (by reference: `ReferenceEqualityComparer.Instance`), plus `List<T> added` and `List<T> removed`. Columns: `typeof(T).GetProperties()` without `Id`. Number parameters with one counter per `SaveChanges` call, but collect each statement's values in its own list so every printed statement shows only its values. The new host gets `Id = 4` after insert (pretend the database assigned it).
@@solution
using System.Reflection;

var web01 = new Host { Id = 1, Name = "web01", Env = "staging", Cores = 4 };
var db01 = new Host { Id = 2, Name = "db01", Env = "prod", Cores = 16 };
var old = new Host { Id = 3, Name = "old01", Env = "prod", Cores = 2 };

var tracker = new Tracker<Host>();
tracker.Attach(web01);
tracker.Attach(db01);
tracker.Attach(old);

var web03 = new Host { Name = "web03", Env = "prod", Cores = 8 };
tracker.Add(web03);
web01.Env = "prod";
web01.Cores = 8;
tracker.Remove(old);
foreach (var sql in tracker.SaveChanges()) Console.WriteLine(sql);
Console.WriteLine($"second save: {tracker.SaveChanges().Count} statements");
web03.Name = "web03-new";
Console.WriteLine($"after rename: {string.Join(" | ", tracker.SaveChanges())}");

class Tracker<T> where T : class
{
    private readonly PropertyInfo[] props = typeof(T).GetProperties();
    private readonly PropertyInfo idProp = typeof(T).GetProperty("Id")!;
    private readonly Dictionary<T, object?[]> snapshots = new(ReferenceEqualityComparer.Instance);
    private readonly List<T> added = new();
    private readonly List<T> removed = new();
    private readonly string table = typeof(T).Name.ToLowerInvariant() + "s";
    private int nextId = 4;

    public void Attach(T e) => snapshots[e] = Snapshot(e);
    public void Add(T e) => added.Add(e);
    public void Remove(T e) => removed.Add(e);

    private object?[] Snapshot(T e) => props.Select(p => p.GetValue(e)).ToArray();

    public List<string> SaveChanges()
    {
        var sql = new List<string>();
        int n = 0;
        string Statement(string text, List<object?> values) => $"{text} [{string.Join(", ", values)}]";
        string P(object? v, List<object?> values)
        {
            values.Add(v);
            return $"@p{n++}";
        }

        var columns = props.Where(p => p != idProp).ToList();
        foreach (var e in added)
        {
            var values = new List<object?>();
            var ps = columns.Select(c => P(c.GetValue(e), values)).ToList();
            sql.Add(Statement($"INSERT INTO {table} ({string.Join(", ", columns.Select(c => c.Name))}) VALUES ({string.Join(", ", ps)})", values));
            idProp.SetValue(e, nextId++);
        }
        foreach (var (e, snap) in snapshots)
        {
            if (removed.Contains(e)) continue;
            var changed = props.Select((p, i) => (p, i)).Where(x => x.p != idProp && !Equals(snap[x.i], x.p.GetValue(e))).ToList();
            if (changed.Count == 0) continue;
            var values = new List<object?>();
            var sets = changed.Select(x => $"{x.p.Name} = {P(x.p.GetValue(e), values)}").ToList();
            sql.Add(Statement($"UPDATE {table} SET {string.Join(", ", sets)} WHERE Id = {P(idProp.GetValue(e), values)}", values));
        }
        foreach (var e in removed)
        {
            var values = new List<object?>();
            sql.Add(Statement($"DELETE FROM {table} WHERE Id = {P(idProp.GetValue(e), values)}", values));
        }

        foreach (var e in removed) snapshots.Remove(e);
        foreach (var e in added) snapshots[e] = Snapshot(e);
        foreach (var e in snapshots.Keys.ToList()) snapshots[e] = Snapshot(e);
        added.Clear();
        removed.Clear();
        return sql;
    }
}

class Host
{
    public int Id { get; set; }
    public string Name { get; set; } = "";
    public string Env { get; set; } = "";
    public int Cores { get; set; }
}

@@exercise Composable query specifications
EF Core can only translate **expression trees**, so reusable filters must be `Expression<Func<T, bool>>`, not plain delegates. Write:

- `And(a, b)` and `Or(a, b)` that combine two expressions into one with a **single** parameter (use an `ExpressionVisitor` that replaces `b`'s parameter with `a`'s)
- `Page(IQueryable<Host> q, int page, int size)`: order by `Name`, skip and take
- run specs against `hosts.AsQueryable()` and print each expression (its `ToString()`), then the matching names

Without the parameter replacement, the combined lambda would reference a parameter that isn't in scope, which is exactly the error EF would give.
@@starter
using System.Linq.Expressions;

var hosts = new List<Host>
{
    new("web01", "prod", 8), new("web02", "prod", 4), new("db01", "prod", 32),
    new("dev01", "dev", 2), new("db02", "staging", 16), new("web03", "staging", 8),
};
Expression<Func<Host, bool>> isProd = h => h.Env == "prod";
Expression<Func<Host, bool>> isBig = h => h.Cores >= 8;
Console.WriteLine("TODO");

record Host(string Name, string Env, int Cores);
@@expected
h => (h.Env == "prod")
  db01, web01, web02
h => ((h.Env == "prod") AndAlso (h.Cores >= 8))
  db01, web01
h => ((h.Env == "dev") OrElse ((h.Env == "prod") AndAlso (h.Cores >= 8)))
  db01, dev01, web01
page 2 of size 2: dev01, web01
@@requires
ExpressionVisitor
Expression.AndAlso
@@hint
`class Replace(ParameterExpression from, ParameterExpression to) : ExpressionVisitor { protected override Expression VisitParameter(ParameterExpression node) => node == from ? to : node; }`. `And`: `var body = new Replace(b.Parameters[0], a.Parameters[0]).Visit(b.Body); return Expression.Lambda<Func<T, bool>>(Expression.AndAlso(a.Body, body), a.Parameters);`. Print names with `.Select(h => h.Name).OrderBy(n => n)`. The `Or` example: `Or(h => h.Env == "dev", And(isProd, isBig))`. Page 2 (1-based) of all hosts ordered by name.
@@solution
using System.Linq.Expressions;

var hosts = new List<Host>
{
    new("web01", "prod", 8), new("web02", "prod", 4), new("db01", "prod", 32),
    new("dev01", "dev", 2), new("db02", "staging", 16), new("web03", "staging", 8),
};
Expression<Func<Host, bool>> isProd = h => h.Env == "prod";
Expression<Func<Host, bool>> isBig = h => h.Cores >= 8;
Expression<Func<Host, bool>> isDev = h => h.Env == "dev";

var q = hosts.AsQueryable();
foreach (var spec in new[] { isProd, And(isProd, isBig), Or(isDev, And(isProd, isBig)) })
{
    Console.WriteLine(spec);
    Console.WriteLine($"  {string.Join(", ", q.Where(spec).Select(h => h.Name).OrderBy(n => n))}");
}
Console.WriteLine($"page 2 of size 2: {string.Join(", ", Page(q, 2, 2).Select(h => h.Name))}");

Expression<Func<T, bool>> And<T>(Expression<Func<T, bool>> a, Expression<Func<T, bool>> b) =>
    Combine(a, b, Expression.AndAlso);

Expression<Func<T, bool>> Or<T>(Expression<Func<T, bool>> a, Expression<Func<T, bool>> b) =>
    Combine(a, b, Expression.OrElse);

Expression<Func<T, bool>> Combine<T>(Expression<Func<T, bool>> a, Expression<Func<T, bool>> b,
    Func<Expression, Expression, BinaryExpression> op)
{
    var body = new Replace(b.Parameters[0], a.Parameters[0]).Visit(b.Body);
    return Expression.Lambda<Func<T, bool>>(op(a.Body, body), a.Parameters);
}

IQueryable<Host> Page(IQueryable<Host> query, int page, int size) =>
    query.OrderBy(h => h.Name).Skip((page - 1) * size).Take(size);

class Replace(ParameterExpression from, ParameterExpression to) : ExpressionVisitor
{
    protected override Expression VisitParameter(ParameterExpression node) => node == from ? to : node;
}

record Host(string Name, string Env, int Cores);

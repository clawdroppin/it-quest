@@lesson n7-webapi ASP.NET Core in Depth: Routing, Middleware, Validation and Problem Details
@@concepts networking, json
@@text
ASP.NET Core is one of the fastest web frameworks in any language. **Minimal APIs** define endpoints as lambdas; controllers (`[ApiController]`) are the class-based alternative with the same pipeline underneath.

```csharp,norun
var builder = WebApplication.CreateBuilder(args);
builder.Services.AddDbContext<InventoryDb>(...);
builder.Services.AddProblemDetails();                  // RFC 9457 error bodies
builder.Services.AddOpenApi();                         // OpenAPI document (.NET 9+)
builder.Services.AddAuthentication().AddJwtBearer();
builder.Services.AddAuthorization();

var app = builder.Build();
app.UseExceptionHandler();                             // unhandled exceptions -> 500 problem details
app.UseAuthentication();
app.UseAuthorization();

var hosts = app.MapGroup("/api/hosts").RequireAuthorization();
hosts.MapGet("/", async (InventoryDb db, int page = 1) =>
    await db.Hosts.AsNoTracking().OrderBy(h => h.Name).Skip((page - 1) * 50).Take(50).ToListAsync());
hosts.MapGet("/{id:int}", async Task<Results<Ok<Host>, NotFound>> (int id, InventoryDb db) =>
    await db.Hosts.FindAsync(id) is { } h ? TypedResults.Ok(h) : TypedResults.NotFound());
hosts.MapPost("/", async (CreateHost dto, InventoryDb db) =>
{
    var host = new Host { Name = dto.Name, Ip = dto.Ip };
    db.Hosts.Add(host);
    await db.SaveChangesAsync();
    return TypedResults.Created($"/api/hosts/{host.Id}", host);
});
app.Run();
```

## Routing

Route templates match paths and extract values: `/hosts/{id:int}` (constraint), `/files/{*path}` (catch-all), `{name?}` (optional). Parameters bind automatically from the route, query string, headers (`[FromHeader]`), the JSON body, or DI services. A path that matches but with the wrong method returns **405**, no match at all **404**.

## The middleware pipeline

Each middleware can run code before and after the next one (`app.Use(async (ctx, next) => { ...; await next(ctx); ... })`). **Order matters**: exception handling first, then HTTPS redirection, static files, routing, CORS, authentication, authorization, then endpoints. Endpoint **filters** (`AddEndpointFilter`) are middleware for a single endpoint or group: a good place for validation.

## Errors: Problem Details

Return consistent, machine-readable errors (`application/problem+json`):

```json
{
  "type": "https://tools.ietf.org/html/rfc9110#section-15.5.1",
  "title": "One or more validation errors occurred.",
  "status": 400,
  "errors": { "Name": ["Name is required."] }
}
```

Never leak stack traces or exception messages from unexpected errors to clients; log them with a trace id and return a generic 500.

## Other essentials

- **Auth**: JWT bearer tokens for APIs, cookies for server-rendered apps, Entra ID / OpenID Connect for company logins; policies with `RequireAuthorization("AdminOnly")`.
- **Rate limiting** (`AddRateLimiter`), **output caching**, **health checks** (`MapHealthChecks("/healthz")`), **CORS** for browser clients.
- `IHttpClientFactory` for outgoing calls (connection pooling, Polly/`Microsoft.Extensions.Http.Resilience` retries).
- **Native AOT** publishing for tiny, fast-starting containers (with source-generated JSON).

@@quiz
A route exists for `GET /hosts/{id}` but a client sends `DELETE /hosts/5`. What's the right status?
- 404
* 405 Method Not Allowed
- 400
- 500
? The path matches, the method doesn't.

@@quiz
Why must `UseAuthentication()` come before `UseAuthorization()`?
* Authorization needs to know who the user is, which authentication establishes
- Alphabetical order
- It doesn't matter
- Authorization is slower
? Middleware runs in registration order.

@@quiz
What should a 500 response contain for an unexpected exception?
- The full stack trace
* A generic problem details body (with a trace id), while details go to the logs
- The exception message
- Nothing at all
? Stack traces help attackers.

@@flashcards
`MapGet` / `MapPost` :: minimal API endpoints
`MapGroup` :: shared prefix and settings
`{id:int}` :: route constraint
`{*path}` :: catch-all route segment
`TypedResults.Ok/NotFound/Created` :: typed responses
Middleware order :: exceptions, routing, auth, endpoints
Endpoint filter :: per-endpoint middleware
Problem Details :: RFC 9457 error format
`AddRateLimiter` :: built-in rate limiting
`IHttpClientFactory` :: managed outgoing HttpClients

@@exercise A route matcher
Implement the core of routing:

- `Route(string method, string template, string name)`; templates have literal segments, `{param}`, `{param:int}` and a final `{*rest}` catch-all
- `Router.Match(method, path)` returns `200 <name> {key=value, ...}` (values in template order), `405` when some route matches the path but not the method, otherwise `404`
- a literal segment is preferred over a parameter: with `/hosts/new` and `/hosts/{id}`, the path `/hosts/new` matches the literal route (try routes with more literal segments first)
- matching is case-insensitive for literals; trailing slashes are ignored
@@starter
var router = new Router(
[
    new("GET", "/hosts", "ListHosts"),
    new("POST", "/hosts", "CreateHost"),
    new("GET", "/hosts/{id:int}", "GetHost"),
    new("GET", "/hosts/new", "NewHostForm"),
    new("GET", "/hosts/{id:int}/disks/{name}", "GetDisk"),
    new("GET", "/files/{*path}", "GetFile"),
]);
string[] requests =
[
    "GET /hosts", "POST /hosts/", "GET /hosts/42", "GET /HOSTS/new", "GET /hosts/abc",
    "GET /hosts/42/disks/C", "DELETE /hosts/42", "GET /files/logs/2026/app.log", "GET /files", "GET /nope",
];
foreach (var r in requests)
{
    var parts = r.Split(' ');
    Console.WriteLine($"{r,-30} -> {router.Match(parts[0], parts[1])}");
}

record Route(string Method, string Template, string Name);

class Router(List<Route> routes)
{
    public string Match(string method, string path) => "404";
}
@@expected
GET /hosts                     -> 200 ListHosts {}
POST /hosts/                   -> 200 CreateHost {}
GET /hosts/42                  -> 200 GetHost {id=42}
GET /HOSTS/new                 -> 200 NewHostForm {}
GET /hosts/abc                 -> 404
GET /hosts/42/disks/C          -> 200 GetDisk {id=42, name=C}
DELETE /hosts/42               -> 405
GET /files/logs/2026/app.log   -> 200 GetFile {path=logs/2026/app.log}
GET /files                     -> 404
GET /nope                      -> 404
@@hint
`TryMatch(string template, string[] segs, out List<(string, string)> values)`: split both on `/` with `RemoveEmptyEntries`. For each template segment: `{*x}` takes the rest (at least one segment) joined with `/`; `{x:int}` requires `int.TryParse`; `{x}` takes anything; literals compare with `OrdinalIgnoreCase`. Lengths must match unless there's a catch-all. Sort routes by number of literal segments, descending (stable: `OrderByDescending` keeps order on ties). Track whether any route matched the path at all, for 405.
@@solution
var router = new Router(
[
    new("GET", "/hosts", "ListHosts"),
    new("POST", "/hosts", "CreateHost"),
    new("GET", "/hosts/{id:int}", "GetHost"),
    new("GET", "/hosts/new", "NewHostForm"),
    new("GET", "/hosts/{id:int}/disks/{name}", "GetDisk"),
    new("GET", "/files/{*path}", "GetFile"),
]);
string[] requests =
[
    "GET /hosts", "POST /hosts/", "GET /hosts/42", "GET /HOSTS/new", "GET /hosts/abc",
    "GET /hosts/42/disks/C", "DELETE /hosts/42", "GET /files/logs/2026/app.log", "GET /files", "GET /nope",
];
foreach (var r in requests)
{
    var parts = r.Split(' ');
    Console.WriteLine($"{r,-30} -> {router.Match(parts[0], parts[1])}");
}

record Route(string Method, string Template, string Name);

class Router(List<Route> routes)
{
    private readonly List<Route> ordered = routes
        .OrderByDescending(r => Segments(r.Template).Count(s => !s.StartsWith('{')))
        .ToList();

    private static string[] Segments(string p) => p.Split('/', StringSplitOptions.RemoveEmptyEntries);

    public string Match(string method, string path)
    {
        var segs = Segments(path);
        bool pathMatched = false;
        foreach (var r in ordered)
        {
            if (!TryMatch(Segments(r.Template), segs, out var values)) continue;
            pathMatched = true;
            if (!r.Method.Equals(method, StringComparison.OrdinalIgnoreCase)) continue;
            return $"200 {r.Name} {{{string.Join(", ", values.Select(v => $"{v.Key}={v.Value}"))}}}";
        }
        return pathMatched ? "405" : "404";
    }

    private static bool TryMatch(string[] template, string[] segs, out List<KeyValuePair<string, string>> values)
    {
        values = new();
        for (int i = 0; i < template.Length; i++)
        {
            string t = template[i];
            if (t.StartsWith("{*"))
            {
                if (i >= segs.Length) return false;
                values.Add(new(t[2..^1], string.Join('/', segs[i..])));
                return true;
            }
            if (i >= segs.Length) return false;
            if (t.StartsWith('{'))
            {
                var spec = t[1..^1].Split(':');
                if (spec.Length == 2 && spec[1] == "int" && !int.TryParse(segs[i], out _)) return false;
                values.Add(new(spec[0], segs[i]));
            }
            else if (!t.Equals(segs[i], StringComparison.OrdinalIgnoreCase))
            {
                return false;
            }
        }
        return template.Length == segs.Length;
    }
}

@@exercise Exceptions to Problem Details
Write the error-handling middleware of an API as a function `Handle(Func<object> endpoint)` returning `(int Status, string Json)`:

- success → `200` and the result serialized as JSON (web defaults: camelCase)
- `NotFoundException(resource, id)` → `404` with problem details `title` `Not Found` and `detail` `host 7 was not found`
- `ValidationException(Dictionary<string, string[]> errors)` → `400`, title `One or more validation errors occurred.` and an `errors` object
- anything else → `500` with title `An unexpected error occurred.` and **no** detail (never leak the message); instead log `ERROR <TypeName>: <message>` to a list, plus a trace id `trace-1`, `trace-2`... that is included in the response as `traceId`
- null properties must be left out of the JSON
@@starter
using System.Text.Json;
using System.Text.Json.Serialization;

var log = new List<string>();
Console.WriteLine("TODO");

class NotFoundException(string resource, object id) : Exception($"{resource} {id} was not found");
class ValidationException(Dictionary<string, string[]> errors) : Exception("validation failed")
{
    public Dictionary<string, string[]> Errors { get; } = errors;
}

record Problem(int Status, string Title, string? Detail = null, Dictionary<string, string[]>? Errors = null, string? TraceId = null);
@@expected
200 {"name":"web01","cores":8}
404 {"status":404,"title":"Not Found","detail":"host 7 was not found"}
400 {"status":400,"title":"One or more validation errors occurred.","errors":{"Name":["Name is required."],"Cores":["Cores must be 1-256."]}}
500 {"status":500,"title":"An unexpected error occurred.","traceId":"trace-1"}
log: ERROR NpgsqlException: password authentication failed for user "app"
@@requires
JsonSerializerDefaults.Web
WhenWritingNull
@@hint
`var json = new JsonSerializerOptions(JsonSerializerDefaults.Web) { DefaultIgnoreCondition = JsonIgnoreCondition.WhenWritingNull };`. Note that dictionary keys (`Name`, `Cores`) aren't camel-cased by the web defaults. Simulate the database failure with a local class `class NpgsqlException(string m) : Exception(m);`. Endpoints: `() => new { Name = "web01", Cores = 8 }`, `() => throw new NotFoundException("host", 7)`, a validation failure, and the database failure.
@@solution
using System.Text.Json;
using System.Text.Json.Serialization;

var log = new List<string>();
int traces = 0;
var json = new JsonSerializerOptions(JsonSerializerDefaults.Web) { DefaultIgnoreCondition = JsonIgnoreCondition.WhenWritingNull };

Func<object>[] endpoints =
[
    () => new { Name = "web01", Cores = 8 },
    () => throw new NotFoundException("host", 7),
    () => throw new ValidationException(new()
    {
        ["Name"] = ["Name is required."],
        ["Cores"] = ["Cores must be 1-256."],
    }),
    () => throw new NpgsqlException("password authentication failed for user \"app\""),
];
foreach (var e in endpoints)
{
    var (status, body) = Handle(e);
    Console.WriteLine($"{status} {body}");
}
foreach (var l in log) Console.WriteLine($"log: {l}");

(int Status, string Json) Handle(Func<object> endpoint)
{
    try
    {
        return (200, JsonSerializer.Serialize(endpoint(), json));
    }
    catch (NotFoundException e)
    {
        return Send(new Problem(404, "Not Found", e.Message));
    }
    catch (ValidationException e)
    {
        return Send(new Problem(400, "One or more validation errors occurred.", Errors: e.Errors));
    }
    catch (Exception e)
    {
        string traceId = $"trace-{++traces}";
        log.Add($"ERROR {e.GetType().Name}: {e.Message}");
        return Send(new Problem(500, "An unexpected error occurred.", TraceId: traceId));
    }

    (int, string) Send(Problem p) => (p.Status, JsonSerializer.Serialize(p, json));
}

class NotFoundException(string resource, object id) : Exception($"{resource} {id} was not found");
class ValidationException(Dictionary<string, string[]> errors) : Exception("validation failed")
{
    public Dictionary<string, string[]> Errors { get; } = errors;
}
class NpgsqlException(string message) : Exception(message);

record Problem(int Status, string Title, string? Detail = null, Dictionary<string, string[]>? Errors = null, string? TraceId = null);

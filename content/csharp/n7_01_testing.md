@@lesson n7-testing Testing .NET Code: xUnit Theories, Test Doubles and Fake Time
@@concepts testing
@@text
## xUnit in practice

```csharp,norun
public class SubnetTests
{
    [Fact]
    public void Slash24_has_254_hosts() => Assert.Equal(254, Subnet.HostCount(24));

    [Theory]
    [InlineData(30, 2)]
    [InlineData(31, 2)]     // RFC 3021 point-to-point links
    [InlineData(32, 1)]
    public void Small_prefixes(int prefix, int expected) => Assert.Equal(expected, Subnet.HostCount(prefix));

    [Fact]
    public void Rejects_bad_prefix()
    {
        var ex = Assert.Throws<ArgumentOutOfRangeException>(() => Subnet.HostCount(40));
        Assert.Equal("prefix", ex.ParamName);
    }
}
```

```text
dotnet new xunit -o Inventory.Tests
dotnet add Inventory.Tests reference Inventory
dotnet test                                   # run everything
dotnet test --filter "FullyQualifiedName~Subnet"
dotnet test --collect:"XPlat Code Coverage"
```

- `[Fact]`: a test; `[Theory]` + `[InlineData]`/`[MemberData]`: the same test with many inputs.
- xUnit creates a **new instance of the test class per test**, so fields never leak between tests; shared expensive setup goes in a fixture (`IClassFixture<T>`).
- Assertion libraries: plain `Assert`, Shouldly (`x.ShouldBe(3)`), or AwesomeAssertions/FluentAssertions. MSTest and NUnit are the other big frameworks; the ideas are identical.

## Test doubles

| Double | Does | Example |
|---|---|---|
| **Stub** | returns canned answers | `IExchangeRates` always returns 150 |
| **Fake** | a working light implementation | in-memory repository |
| **Spy** | records how it was called | `IMailer` that stores sent messages |
| **Mock** | verifies expected calls | NSubstitute's `Received(1).Send(...)` |

```csharp,norun
var mailer = Substitute.For<IMailer>();          // NSubstitute
var service = new PasswordResetService(mailer, users);
service.Reset("ana");
mailer.Received(1).Send("ana@example.jp", Arg.Is<string>(s => s.Contains("reset")));
```

Prefer fakes and spies you write yourself for your own interfaces; they're simple and survive refactoring. Mock libraries shine for large third-party interfaces.

## Fake time with TimeProvider

Code that calls `DateTime.Now` can't be tested reliably. .NET 8's `TimeProvider` is an abstract clock: production uses `TimeProvider.System`, and tests pass a fake (`Microsoft.Extensions.TimeProvider.Testing.FakeTimeProvider`, or your own subclass):

```csharp
var clock = new ManualClock(new DateTimeOffset(2026, 11, 3, 9, 0, 0, TimeSpan.Zero));
Console.WriteLine(clock.GetUtcNow());
clock.Advance(TimeSpan.FromMinutes(90));
Console.WriteLine(clock.GetUtcNow().ToString("HH:mm"));

class ManualClock(DateTimeOffset start) : TimeProvider
{
    private DateTimeOffset now = start;
    public override DateTimeOffset GetUtcNow() => now;
    public void Advance(TimeSpan by) => now += by;
}
```

## Integration tests

- `WebApplicationFactory<Program>` starts your ASP.NET Core app in memory and gives you an `HttpClient` to call it, so you can test routing, model binding and middleware without a real server.
- **Testcontainers** starts a real PostgreSQL or SQL Server in Docker for repository tests.
- **Snapshot tests** (the Verify library) store the expected output in a file and show a diff when it changes.

A healthy suite is mostly fast unit tests, some integration tests and a few end-to-end tests.

@@quiz
What's the difference between `[Fact]` and `[Theory]`?
* A Theory runs the same test with several data rows; a Fact has no parameters
- Theories are slower
- Facts are skipped in CI
- None
? `[InlineData]` supplies the rows.

@@quiz
Why use `TimeProvider` instead of `DateTime.UtcNow` in business logic?
- It's more accurate
* Tests can substitute a fake clock and control time exactly
- DateTime is obsolete
- It handles time zones automatically
? Expiry, retries and schedules become testable.

@@quiz
Which test double records how it was called so the test can check it afterwards?
- Stub
* Spy
- Dummy
- Fixture
? E.g. a mailer that stores every message.

@@drill
Which xUnit attribute supplies inline data to a [Theory]?
= [InlineData]
= InlineData
? [InlineData(1, 2, 3)].

@@flashcards
`[Fact]` :: a test without parameters
`[Theory]` + `[InlineData]` :: data-driven test
`Assert.Throws<T>` :: expect an exception
`IClassFixture<T>` :: shared setup across a class's tests
Stub :: canned answers
Fake :: working lightweight implementation
Spy :: records calls
Mock :: verifies expected interactions
`TimeProvider` :: abstract clock (.NET 8)
`WebApplicationFactory` :: in-memory ASP.NET Core for tests

@@exercise Build a mini test runner
Use reflection to write a tiny xUnit:

- attributes `[Fact]`, `[Theory]` and `[InlineData(params object[] values)]` (`AllowMultiple = true`)
- `Run(Type testClass)`: for each public instance method with `[Fact]`, create a **new instance** of the class and invoke it; for `[Theory]`, invoke once per `[InlineData]` row with those arguments
- a test passes if it doesn't throw; print `PASS name` or `FAIL name: message` (unwrap `TargetInvocationException` to the inner exception), with theory names like `Name(30, 2)`
- finally `N passed, M failed`

Methods are run in **name order** (reflection's order isn't guaranteed), and theory rows in attribute order.
@@starter
using System.Reflection;

Console.WriteLine("TODO");

static class Subnet
{
    public static int HostCount(int prefix)
    {
        if (prefix is < 0 or > 32) throw new ArgumentOutOfRangeException(nameof(prefix));
        return prefix >= 31 ? 2 : (1 << (32 - prefix)) - 2;  // bug: /32 is a single host
    }
}

static class Check
{
    public static void Equal<T>(T expected, T actual)
    {
        if (!EqualityComparer<T>.Default.Equals(expected, actual))
            throw new Exception($"expected {expected}, got {actual}");
    }
}

class SubnetTests
{
    private int calls;

    [Fact]
    public void Slash24() => Check.Equal(254, Subnet.HostCount(24));

    [Theory]
    [InlineData(30, 2)]
    [InlineData(31, 2)]
    [InlineData(32, 1)]
    public void Small(int prefix, int expected) => Check.Equal(expected, Subnet.HostCount(prefix));

    [Fact]
    public void FreshInstance()
    {
        calls++;
        Check.Equal(1, calls);
    }

    [Fact]
    public void RejectsBadPrefix()
    {
        try { Subnet.HostCount(40); }
        catch (ArgumentOutOfRangeException) { return; }
        throw new Exception("no exception thrown");
    }

    public void NotATest() => throw new Exception("should not run");
}
@@expected
PASS FreshInstance
PASS RejectsBadPrefix
PASS Slash24
PASS Small(30, 2)
PASS Small(31, 2)
FAIL Small(32, 1): expected 1, got 2
5 passed, 1 failed
@@requires
GetCustomAttributes
TargetInvocationException
@@hint
Declare the attributes after the test code (`class FactAttribute : Attribute { }`, `class InlineDataAttribute(params object[] values) : Attribute { public object[] Values { get; } = values; }`). Loop over `type.GetMethods().OrderBy(m => m.Name, StringComparer.Ordinal)`. For each invocation: `var obj = Activator.CreateInstance(type); try { m.Invoke(obj, args); ... } catch (TargetInvocationException e) { ... e.InnerException!.Message ... }`. Note the deliberate bug in `HostCount`: the runner must report it, not fix it.
@@solution
using System.Reflection;

Run(typeof(SubnetTests));

void Run(Type type)
{
    int passed = 0, failed = 0;
    foreach (var m in type.GetMethods(BindingFlags.Public | BindingFlags.Instance).OrderBy(m => m.Name, StringComparer.Ordinal))
    {
        var cases = new List<(string Name, object?[]? Args)>();
        if (m.GetCustomAttribute<FactAttribute>() is not null) cases.Add((m.Name, null));
        if (m.GetCustomAttribute<TheoryAttribute>() is not null)
            foreach (var row in m.GetCustomAttributes<InlineDataAttribute>())
                cases.Add(($"{m.Name}({string.Join(", ", row.Values)})", row.Values));
        foreach (var (name, args) in cases)
        {
            var obj = Activator.CreateInstance(type);
            try
            {
                m.Invoke(obj, args);
                Console.WriteLine($"PASS {name}");
                passed++;
            }
            catch (TargetInvocationException e)
            {
                Console.WriteLine($"FAIL {name}: {e.InnerException?.Message}");
                failed++;
            }
        }
    }
    Console.WriteLine($"{passed} passed, {failed} failed");
}

[AttributeUsage(AttributeTargets.Method)]
class FactAttribute : Attribute { }

[AttributeUsage(AttributeTargets.Method)]
class TheoryAttribute : Attribute { }

[AttributeUsage(AttributeTargets.Method, AllowMultiple = true)]
class InlineDataAttribute(params object[] values) : Attribute
{
    public object[] Values { get; } = values;
}

static class Subnet
{
    public static int HostCount(int prefix)
    {
        if (prefix is < 0 or > 32) throw new ArgumentOutOfRangeException(nameof(prefix));
        return prefix >= 31 ? 2 : (1 << (32 - prefix)) - 2;  // bug: /32 is a single host
    }
}

static class Check
{
    public static void Equal<T>(T expected, T actual)
    {
        if (!EqualityComparer<T>.Default.Equals(expected, actual))
            throw new Exception($"expected {expected}, got {actual}");
    }
}

class SubnetTests
{
    private int calls;

    [Fact]
    public void Slash24() => Check.Equal(254, Subnet.HostCount(24));

    [Theory]
    [InlineData(30, 2)]
    [InlineData(31, 2)]
    [InlineData(32, 1)]
    public void Small(int prefix, int expected) => Check.Equal(expected, Subnet.HostCount(prefix));

    [Fact]
    public void FreshInstance()
    {
        calls++;
        Check.Equal(1, calls);
    }

    [Fact]
    public void RejectsBadPrefix()
    {
        try { Subnet.HostCount(40); }
        catch (ArgumentOutOfRangeException) { return; }
        throw new Exception("no exception thrown");
    }

    public void NotATest() => throw new Exception("should not run");
}

@@exercise Testing token expiry with a fake clock and a spy
A `SessionService` issues login tokens that expire after 30 minutes, and emails a security notice on every login from a new country.

- it takes a `TimeProvider` and an `IMailer`; write `ManualClock : TimeProvider` (override `GetUtcNow`, add `Advance`) and `SpyMailer : IMailer` (stores `to|subject` lines)
- `Login(user, country)` returns a token `tok-1`, `tok-2`...; it sends `New login from <country>` the first time a user logs in from a country
- `IsValid(token)`: true until **30 minutes** after issue (expired at exactly 30:00)
- write the "tests" as top-level checks that print `ok <name>` or `FAIL <name>`
@@starter
Console.WriteLine("TODO");

interface IMailer { void Send(string to, string subject); }

class SessionService(TimeProvider clock, IMailer mailer)
{
}
@@expected
ok token valid right after login
ok valid at 29:59
ok expired at 30:00
ok first login from JP sends a notice
ok second login from JP sends nothing
ok login from US sends a notice
ok mails: ana|New login from JP; ana|New login from US
@@requires
TimeProvider
GetUtcNow
@@hint
Store `Dictionary<string, DateTimeOffset> issued` and `HashSet<(string User, string Country)> seen`. `IsValid`: `issued.TryGetValue(t, out var at) && clock.GetUtcNow() - at < TimeSpan.FromMinutes(30)`. A helper `void Expect(bool condition, string name) => Console.WriteLine(condition ? $"ok {name}" : $"FAIL {name}");`.
@@solution
var clock = new ManualClock(new DateTimeOffset(2026, 11, 3, 9, 0, 0, TimeSpan.Zero));
var mailer = new SpyMailer();
var sessions = new SessionService(clock, mailer);

var t1 = sessions.Login("ana", "JP");
Expect(sessions.IsValid(t1), "token valid right after login");
clock.Advance(TimeSpan.FromMinutes(29) + TimeSpan.FromSeconds(59));
Expect(sessions.IsValid(t1), "valid at 29:59");
clock.Advance(TimeSpan.FromSeconds(1));
Expect(!sessions.IsValid(t1), "expired at 30:00");
Expect(mailer.Sent.Count == 1, "first login from JP sends a notice");
sessions.Login("ana", "JP");
Expect(mailer.Sent.Count == 1, "second login from JP sends nothing");
sessions.Login("ana", "US");
Expect(mailer.Sent.Count == 2, "login from US sends a notice");
Expect(true, $"mails: {string.Join("; ", mailer.Sent)}");

void Expect(bool condition, string name) => Console.WriteLine(condition ? $"ok {name}" : $"FAIL {name}");

interface IMailer { void Send(string to, string subject); }

class ManualClock(DateTimeOffset start) : TimeProvider
{
    private DateTimeOffset now = start;
    public override DateTimeOffset GetUtcNow() => now;
    public void Advance(TimeSpan by) => now += by;
}

class SpyMailer : IMailer
{
    public List<string> Sent { get; } = new();
    public void Send(string to, string subject) => Sent.Add($"{to}|{subject}");
}

class SessionService(TimeProvider clock, IMailer mailer)
{
    private readonly Dictionary<string, DateTimeOffset> issued = new();
    private readonly HashSet<(string User, string Country)> seen = new();

    public string Login(string user, string country)
    {
        if (seen.Add((user, country))) mailer.Send(user, $"New login from {country}");
        string token = $"tok-{issued.Count + 1}";
        issued[token] = clock.GetUtcNow();
        return token;
    }

    public bool IsValid(string token) =>
        issued.TryGetValue(token, out var at) && clock.GetUtcNow() - at < TimeSpan.FromMinutes(30);
}

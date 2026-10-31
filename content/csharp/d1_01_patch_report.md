@@lesson c9-patch-report Patch Compliance Reporter
@@text
Every IT department gets asked the same question by security and by auditors: **"Are all our machines patched, encrypted and protected?"** In this build you write the tool that answers it.

It reads an inventory export (the kind Intune, SCCM or an RMM tool gives you) and a **baseline**, the company's security rules, and reports every machine that breaks them:

```text
Patch compliance report: 8 machines, 10 findings
[CRITICAL] osk-pc-005: disk not encrypted
[CRITICAL] tky-pc-002: build 26100.3775 is below 26100.4652
[CRITICAL] tky-pc-003: disk not encrypted
[CRITICAL] tky-srv-02: build 6.8.0-40 is below 6.8.0-60
[HIGH] osk-pc-004: antivirus not running
[HIGH] osk-pc-005: last patched 2482 days ago
...
[WARNING] osk-pc-005: unsupported OS Windows 7
Compliant: 3/8 (37.5%)
```

The program exits with code **2** when anything is critical. Run it as a scheduled task or in a pipeline, and it can trigger an alert.

What you practise: records, `List`/`Dictionary`/`HashSet`, LINQ sorting and grouping, parsing with `TryParseExact`, exceptions with useful messages, and JSON. The project has a small built-in test runner (`Tests.cs`), so there's nothing to install apart from the .NET SDK.

Open the folder in VS Code (install the **C# Dev Kit** extension), then:

1. `dotnet run -- --test`: every test shows TODO.
2. Write the methods in `Compliance.cs`, in order. Their doc comments say exactly what to do.
3. When all 18 tests pass: `dotnet run -- inventory.csv baseline.json 2026-10-31`.
@@project patch_report Patch Compliance Reporter
Write the five methods in `Compliance.cs` until `dotnet run -- --test` passes, then run the report on the sample inventory. The date argument makes the results match the lesson; leave it out to use today.
@@file Compliance.cs
using System.Globalization;

namespace PatchReport;

public record Machine(string Name, string Os, string Build, DateOnly LastPatch, bool Encrypted, bool AvRunning);

public record Baseline(Dictionary<string, string> MinBuilds, int MaxPatchAgeDays, bool RequireEncryption);

public record Finding(string Machine, string Severity, string Message);

public static class Compliance
{
    /// <summary>Severities from worst to least bad, for sorting.</summary>
    public static readonly string[] Severities = { "critical", "high", "warning" };

    /// <summary>
    /// Compare version strings like "26100.4652" or "6.8.0-60" part by part, as numbers.
    /// Returns a negative number if a is older, 0 if equal, positive if a is newer.
    /// Missing parts count as 0, so "10.0" equals "10.0.0".
    /// </summary>
    public static int CompareVersions(string a, string b)
    {
        throw new NotImplementedException("TODO: not done yet");
    }

    /// <summary>
    /// Read inventory CSV lines: a header, then name,os,build,last_patch(yyyy-MM-dd),encrypted,av
    /// where the last two are "yes" or "no". Blank lines are skipped. A bad line throws
    /// FormatException("line N: ...") with N counting from 1, header included.
    /// </summary>
    public static List<Machine> ParseInventory(IEnumerable<string> lines)
    {
        throw new NotImplementedException("TODO: not done yet");
    }

    static bool YesNo(string s, int line) => s.ToLowerInvariant() switch
    {
        "yes" => true,
        "no" => false,
        _ => throw new FormatException($"line {line}: expected yes or no, got '{s}'"),
    };

    /// <summary>
    /// Every way the machine breaks the baseline, in this order:
    /// - OS not in MinBuilds: warning "unsupported OS <os>" (and skip the build check)
    /// - build older than the minimum: critical "build <build> is below <minimum>"
    /// - patched more than MaxPatchAgeDays ago: high "last patched <n> days ago"
    /// - not encrypted while RequireEncryption: critical "disk not encrypted"
    /// - antivirus not running: high "antivirus not running"
    /// </summary>
    public static List<Finding> Evaluate(Machine m, Baseline b, DateOnly today)
    {
        throw new NotImplementedException("TODO: not done yet");
    }

    /// <summary>Percentage of machines with no findings at all, rounded to 1 decimal. 100 for an empty fleet.</summary>
    public static double ComplianceRate(List<Machine> machines, List<Finding> findings)
    {
        throw new NotImplementedException("TODO: not done yet");
    }

    /// <summary>
    /// The text report: a title line, one line per finding sorted by severity (critical first)
    /// then machine name, formatted "[CRITICAL] pc-01: disk not encrypted", then
    /// "Compliant: 3/5 (60.0%)".
    /// </summary>
    public static string FormatReport(List<Machine> machines, List<Finding> findings)
    {
        throw new NotImplementedException("TODO: not done yet");
    }
}
@@file PatchReport.csproj
<Project Sdk="Microsoft.NET.Sdk">

  <PropertyGroup>
    <OutputType>Exe</OutputType>
    <TargetFramework>net8.0</TargetFramework>
    <ImplicitUsings>enable</ImplicitUsings>
    <Nullable>enable</Nullable>
    <!-- Same number and date formatting on every computer. -->
    <InvariantGlobalization>true</InvariantGlobalization>
    <RollForward>Major</RollForward>
  </PropertyGroup>

</Project>
@@file Program.cs
using System.Text.Json;
using PatchReport;

if (args.Contains("--test"))
    return Tests.RunAll();

// dotnet run -- [inventory.csv] [baseline.json] [yyyy-MM-dd]
string inventoryPath = args.ElementAtOrDefault(0) ?? "inventory.csv";
string baselinePath = args.ElementAtOrDefault(1) ?? "baseline.json";
var today = args.Length > 2 ? DateOnly.Parse(args[2]) : DateOnly.FromDateTime(DateTime.Today);

try
{
    var machines = Compliance.ParseInventory(File.ReadLines(inventoryPath));
    var options = new JsonSerializerOptions { PropertyNameCaseInsensitive = true };
    var baseline = JsonSerializer.Deserialize<Baseline>(File.ReadAllText(baselinePath), options)
        ?? throw new FormatException("baseline.json is empty");
    var findings = machines.SelectMany(m => Compliance.Evaluate(m, baseline, today)).ToList();

    string report = Compliance.FormatReport(machines, findings);
    Console.WriteLine(report);
    File.WriteAllText("report.txt", report + "\n");

    // Exit code 2 if anything is critical, so a scheduled task or pipeline can alert on it.
    return findings.Any(f => f.Severity == "critical") ? 2 : 0;
}
catch (Exception e) when (e is IOException or FormatException or JsonException or NotImplementedException)
{
    Console.Error.WriteLine($"error: {e.Message}");
    return 1;
}
@@file Tests.cs
namespace PatchReport;

/// <summary>A tiny test runner, so the project needs no test framework. Run: dotnet run -- --test</summary>
public static class Tests
{
    static int passed, failed;

    static void Check(string name, Func<bool> test)
    {
        try
        {
            if (test()) { passed++; Console.WriteLine($"  ok    {name}"); return; }
            Console.WriteLine($"  FAIL  {name}");
        }
        catch (NotImplementedException)
        {
            Console.WriteLine($"  TODO  {name} (not written yet)");
        }
        catch (Exception e)
        {
            Console.WriteLine($"  FAIL  {name}: {e.GetType().Name}: {e.Message}");
        }
        failed++;
    }

    static bool Throws<T>(Action a) where T : Exception
    {
        try { a(); return false; }
        catch (T) { return true; }
    }

    static readonly Baseline Base = new(
        new Dictionary<string, string> { ["Windows 11"] = "26100.4652", ["Ubuntu 24.04"] = "6.8.0-60" },
        30,
        true);

    static readonly DateOnly Today = new(2026, 10, 31);

    static Machine Pc(string build = "26100.5000", int daysAgo = 3, bool enc = true, bool av = true, string os = "Windows 11") =>
        new("pc-01", os, build, Today.AddDays(-daysAgo), enc, av);

    public static int RunAll()
    {
        Console.WriteLine("CompareVersions");
        Check("older build is negative", () => Compliance.CompareVersions("26100.4000", "26100.4652") < 0);
        Check("compares numbers, not text (10 > 9)", () => Compliance.CompareVersions("6.10.0", "6.9.0") > 0);
        Check("missing parts count as zero", () => Compliance.CompareVersions("10.0", "10.0.0") == 0);
        Check("dash separates parts too", () => Compliance.CompareVersions("6.8.0-60", "6.8.0-7") > 0);

        Console.WriteLine("ParseInventory");
        string[] csv = { "name,os,build,last_patch,encrypted,av", "pc-01, Windows 11, 26100.4652, 2026-10-01, yes, no", "", "srv-01,Ubuntu 24.04,6.8.0-60,2026-09-15,NO,yes" };
        Check("reads rows and skips blanks", () => Compliance.ParseInventory(csv).Count == 2);
        Check("trims fields and parses values", () =>
            Compliance.ParseInventory(csv)[0] == new Machine("pc-01", "Windows 11", "26100.4652", new DateOnly(2026, 10, 1), true, false));
        Check("yes/no ignores case", () => !Compliance.ParseInventory(csv)[1].Encrypted);
        Check("bad date names the line", () =>
        {
            try { Compliance.ParseInventory(new[] { "h", "x,Windows 11,1,2026-13-01,yes,yes" }); return false; }
            catch (FormatException e) { return e.Message.StartsWith("line 2:"); }
        });
        Check("wrong field count throws", () => Throws<FormatException>(() => Compliance.ParseInventory(new[] { "h", "a,b,c" })));

        Console.WriteLine("Evaluate");
        Check("a healthy machine has no findings", () => Compliance.Evaluate(Pc(), Base, Today).Count == 0);
        Check("old build is critical", () =>
            Compliance.Evaluate(Pc(build: "26100.100"), Base, Today).Single() == new Finding("pc-01", "critical", "build 26100.100 is below 26100.4652"));
        Check("30 days is fine, 31 is not", () =>
            Compliance.Evaluate(Pc(daysAgo: 30), Base, Today).Count == 0
            && Compliance.Evaluate(Pc(daysAgo: 31), Base, Today).Single().Message == "last patched 31 days ago");
        Check("unknown OS is a warning and skips the build check", () =>
            Compliance.Evaluate(Pc(os: "Windows 7", build: "1"), Base, Today).Single() == new Finding("pc-01", "warning", "unsupported OS Windows 7"));
        Check("all problems, in order", () =>
            string.Join("|", Compliance.Evaluate(Pc(build: "1", daysAgo: 90, enc: false, av: false), Base, Today).Select(f => f.Severity))
            == "critical|high|critical|high");
        Check("encryption only when required", () =>
            Compliance.Evaluate(Pc(enc: false), Base with { RequireEncryption = false }, Today).Count == 0);

        Console.WriteLine("ComplianceRate and FormatReport");
        var fleet = new List<Machine> { Pc() with { Name = "a" }, Pc() with { Name = "b" }, Pc() with { Name = "c" } };
        var findings = new List<Finding> { new("b", "high", "x"), new("b", "critical", "y") };
        Check("rate counts machines, not findings", () => Compliance.ComplianceRate(fleet, findings) == 66.7);
        Check("empty fleet is 100%", () => Compliance.ComplianceRate(new(), new()) == 100);
        Check("report sorts by severity and ends with the rate", () =>
        {
            var lines = Compliance.FormatReport(fleet, findings).Split('\n');
            return lines.Length == 4 && lines[1] == "[CRITICAL] b: y" && lines[2] == "[HIGH] b: x" && lines[3] == "Compliant: 2/3 (66.7%)";
        });

        Console.WriteLine($"\n{passed} passed, {failed} failed");
        return failed == 0 ? 0 : 1;
    }
}
@@file inventory.csv
name,os,build,last_patch,encrypted,av
tky-pc-001,Windows 11,26100.4946,2026-10-20,yes,yes
tky-pc-002,Windows 11,26100.3775,2026-08-02,yes,yes
tky-pc-003,Windows 10,19045.6036,2026-10-14,no,yes
osk-pc-004,Windows 11,26100.4946,2026-10-28,yes,no
osk-pc-005,Windows 7,7601.24544,2020-01-14,no,no
tky-srv-01,Ubuntu 24.04,6.8.0-85,2026-10-25,yes,yes
tky-srv-02,Ubuntu 24.04,6.8.0-40,2026-07-30,yes,yes
osk-srv-03,Windows Server 2022,20348.4171,2026-10-09,yes,yes
@@file baseline.json
{
  "minBuilds": {
    "Windows 11": "26100.4652",
    "Windows 10": "19045.6036",
    "Windows Server 2022": "20348.3932",
    "Ubuntu 24.04": "6.8.0-60"
  },
  "maxPatchAgeDays": 30,
  "requireEncryption": true
}
@@text
> Tips: `CompareVersions`: split on `'.'` and `'-'`, turn the parts into numbers, then compare position by position. `DateOnly.DayNumber` turns a date into a day count, so subtracting two of them gives the age in days. For the rate, the set of machine names that have findings is `findings.Select(f => f.Machine).ToHashSet()`. Sort the report with `Array.IndexOf(Severities, f.Severity)`.
@@project patch_report_finished Patch Compliance Reporter: the finished version
The complete tool. Ideas to extend it: write the report as CSV or HTML as well, add a rule for local admin accounts or firewall status, read the inventory straight from an API with `HttpClient`, or turn it into a `dotnet new webapi` with a `/compliance` endpoint.
@@file Compliance.cs
using System.Globalization;

namespace PatchReport;

public record Machine(string Name, string Os, string Build, DateOnly LastPatch, bool Encrypted, bool AvRunning);

public record Baseline(Dictionary<string, string> MinBuilds, int MaxPatchAgeDays, bool RequireEncryption);

public record Finding(string Machine, string Severity, string Message);

public static class Compliance
{
    /// <summary>Severities from worst to least bad, for sorting.</summary>
    public static readonly string[] Severities = { "critical", "high", "warning" };

    /// <summary>
    /// Compare version strings like "26100.4652" or "6.8.0-60" part by part, as numbers.
    /// Returns a negative number if a is older, 0 if equal, positive if a is newer.
    /// Missing parts count as 0, so "10.0" equals "10.0.0".
    /// </summary>
    public static int CompareVersions(string a, string b)
    {
        int[] Parts(string v) => v.Split('.', '-').Select(p => int.TryParse(p, out int n) ? n : 0).ToArray();
        int[] x = Parts(a), y = Parts(b);
        for (int i = 0; i < Math.Max(x.Length, y.Length); i++)
        {
            int xi = i < x.Length ? x[i] : 0, yi = i < y.Length ? y[i] : 0;
            if (xi != yi) return xi.CompareTo(yi);
        }
        return 0;
    }

    /// <summary>
    /// Read inventory CSV lines: a header, then name,os,build,last_patch(yyyy-MM-dd),encrypted,av
    /// where the last two are "yes" or "no". Blank lines are skipped. A bad line throws
    /// FormatException("line N: ...") with N counting from 1, header included.
    /// </summary>
    public static List<Machine> ParseInventory(IEnumerable<string> lines)
    {
        var machines = new List<Machine>();
        int number = 0;
        foreach (string raw in lines)
        {
            number++;
            if (number == 1 || string.IsNullOrWhiteSpace(raw)) continue;
            string[] f = raw.Split(',').Select(s => s.Trim()).ToArray();
            if (f.Length != 6)
                throw new FormatException($"line {number}: expected 6 fields, got {f.Length}");
            if (!DateOnly.TryParseExact(f[3], "yyyy-MM-dd", CultureInfo.InvariantCulture, DateTimeStyles.None, out var date))
                throw new FormatException($"line {number}: bad date '{f[3]}'");
            machines.Add(new Machine(f[0], f[1], f[2], date, YesNo(f[4], number), YesNo(f[5], number)));
        }
        return machines;
    }

    static bool YesNo(string s, int line) => s.ToLowerInvariant() switch
    {
        "yes" => true,
        "no" => false,
        _ => throw new FormatException($"line {line}: expected yes or no, got '{s}'"),
    };

    /// <summary>
    /// Every way the machine breaks the baseline, in this order:
    /// - OS not in MinBuilds: warning "unsupported OS <os>" (and skip the build check)
    /// - build older than the minimum: critical "build <build> is below <minimum>"
    /// - patched more than MaxPatchAgeDays ago: high "last patched <n> days ago"
    /// - not encrypted while RequireEncryption: critical "disk not encrypted"
    /// - antivirus not running: high "antivirus not running"
    /// </summary>
    public static List<Finding> Evaluate(Machine m, Baseline b, DateOnly today)
    {
        var found = new List<Finding>();
        void Add(string severity, string message) => found.Add(new Finding(m.Name, severity, message));

        if (!b.MinBuilds.TryGetValue(m.Os, out string? minimum))
            Add("warning", $"unsupported OS {m.Os}");
        else if (CompareVersions(m.Build, minimum) < 0)
            Add("critical", $"build {m.Build} is below {minimum}");

        int age = today.DayNumber - m.LastPatch.DayNumber;
        if (age > b.MaxPatchAgeDays) Add("high", $"last patched {age} days ago");
        if (b.RequireEncryption && !m.Encrypted) Add("critical", "disk not encrypted");
        if (!m.AvRunning) Add("high", "antivirus not running");
        return found;
    }

    /// <summary>Percentage of machines with no findings at all, rounded to 1 decimal. 100 for an empty fleet.</summary>
    public static double ComplianceRate(List<Machine> machines, List<Finding> findings)
    {
        if (machines.Count == 0) return 100;
        var bad = findings.Select(f => f.Machine).ToHashSet();
        int good = machines.Count(m => !bad.Contains(m.Name));
        return Math.Round(100.0 * good / machines.Count, 1);
    }

    /// <summary>
    /// The text report: a title line, one line per finding sorted by severity (critical first)
    /// then machine name, formatted "[CRITICAL] pc-01: disk not encrypted", then
    /// "Compliant: 3/5 (60.0%)".
    /// </summary>
    public static string FormatReport(List<Machine> machines, List<Finding> findings)
    {
        var lines = new List<string> { $"Patch compliance report: {machines.Count} machines, {findings.Count} findings" };
        foreach (var f in findings.OrderBy(f => Array.IndexOf(Severities, f.Severity)).ThenBy(f => f.Machine, StringComparer.Ordinal))
            lines.Add($"[{f.Severity.ToUpperInvariant()}] {f.Machine}: {f.Message}");
        int good = machines.Count - findings.Select(f => f.Machine).Distinct().Count();
        lines.Add($"Compliant: {good}/{machines.Count} ({ComplianceRate(machines, findings):F1}%)");
        return string.Join("\n", lines);
    }
}
@@file PatchReport.csproj
<Project Sdk="Microsoft.NET.Sdk">

  <PropertyGroup>
    <OutputType>Exe</OutputType>
    <TargetFramework>net8.0</TargetFramework>
    <ImplicitUsings>enable</ImplicitUsings>
    <Nullable>enable</Nullable>
    <!-- Same number and date formatting on every computer. -->
    <InvariantGlobalization>true</InvariantGlobalization>
    <RollForward>Major</RollForward>
  </PropertyGroup>

</Project>
@@file Program.cs
using System.Text.Json;
using PatchReport;

if (args.Contains("--test"))
    return Tests.RunAll();

// dotnet run -- [inventory.csv] [baseline.json] [yyyy-MM-dd]
string inventoryPath = args.ElementAtOrDefault(0) ?? "inventory.csv";
string baselinePath = args.ElementAtOrDefault(1) ?? "baseline.json";
var today = args.Length > 2 ? DateOnly.Parse(args[2]) : DateOnly.FromDateTime(DateTime.Today);

try
{
    var machines = Compliance.ParseInventory(File.ReadLines(inventoryPath));
    var options = new JsonSerializerOptions { PropertyNameCaseInsensitive = true };
    var baseline = JsonSerializer.Deserialize<Baseline>(File.ReadAllText(baselinePath), options)
        ?? throw new FormatException("baseline.json is empty");
    var findings = machines.SelectMany(m => Compliance.Evaluate(m, baseline, today)).ToList();

    string report = Compliance.FormatReport(machines, findings);
    Console.WriteLine(report);
    File.WriteAllText("report.txt", report + "\n");

    // Exit code 2 if anything is critical, so a scheduled task or pipeline can alert on it.
    return findings.Any(f => f.Severity == "critical") ? 2 : 0;
}
catch (Exception e) when (e is IOException or FormatException or JsonException or NotImplementedException)
{
    Console.Error.WriteLine($"error: {e.Message}");
    return 1;
}
@@file Tests.cs
namespace PatchReport;

/// <summary>A tiny test runner, so the project needs no test framework. Run: dotnet run -- --test</summary>
public static class Tests
{
    static int passed, failed;

    static void Check(string name, Func<bool> test)
    {
        try
        {
            if (test()) { passed++; Console.WriteLine($"  ok    {name}"); return; }
            Console.WriteLine($"  FAIL  {name}");
        }
        catch (NotImplementedException)
        {
            Console.WriteLine($"  TODO  {name} (not written yet)");
        }
        catch (Exception e)
        {
            Console.WriteLine($"  FAIL  {name}: {e.GetType().Name}: {e.Message}");
        }
        failed++;
    }

    static bool Throws<T>(Action a) where T : Exception
    {
        try { a(); return false; }
        catch (T) { return true; }
    }

    static readonly Baseline Base = new(
        new Dictionary<string, string> { ["Windows 11"] = "26100.4652", ["Ubuntu 24.04"] = "6.8.0-60" },
        30,
        true);

    static readonly DateOnly Today = new(2026, 10, 31);

    static Machine Pc(string build = "26100.5000", int daysAgo = 3, bool enc = true, bool av = true, string os = "Windows 11") =>
        new("pc-01", os, build, Today.AddDays(-daysAgo), enc, av);

    public static int RunAll()
    {
        Console.WriteLine("CompareVersions");
        Check("older build is negative", () => Compliance.CompareVersions("26100.4000", "26100.4652") < 0);
        Check("compares numbers, not text (10 > 9)", () => Compliance.CompareVersions("6.10.0", "6.9.0") > 0);
        Check("missing parts count as zero", () => Compliance.CompareVersions("10.0", "10.0.0") == 0);
        Check("dash separates parts too", () => Compliance.CompareVersions("6.8.0-60", "6.8.0-7") > 0);

        Console.WriteLine("ParseInventory");
        string[] csv = { "name,os,build,last_patch,encrypted,av", "pc-01, Windows 11, 26100.4652, 2026-10-01, yes, no", "", "srv-01,Ubuntu 24.04,6.8.0-60,2026-09-15,NO,yes" };
        Check("reads rows and skips blanks", () => Compliance.ParseInventory(csv).Count == 2);
        Check("trims fields and parses values", () =>
            Compliance.ParseInventory(csv)[0] == new Machine("pc-01", "Windows 11", "26100.4652", new DateOnly(2026, 10, 1), true, false));
        Check("yes/no ignores case", () => !Compliance.ParseInventory(csv)[1].Encrypted);
        Check("bad date names the line", () =>
        {
            try { Compliance.ParseInventory(new[] { "h", "x,Windows 11,1,2026-13-01,yes,yes" }); return false; }
            catch (FormatException e) { return e.Message.StartsWith("line 2:"); }
        });
        Check("wrong field count throws", () => Throws<FormatException>(() => Compliance.ParseInventory(new[] { "h", "a,b,c" })));

        Console.WriteLine("Evaluate");
        Check("a healthy machine has no findings", () => Compliance.Evaluate(Pc(), Base, Today).Count == 0);
        Check("old build is critical", () =>
            Compliance.Evaluate(Pc(build: "26100.100"), Base, Today).Single() == new Finding("pc-01", "critical", "build 26100.100 is below 26100.4652"));
        Check("30 days is fine, 31 is not", () =>
            Compliance.Evaluate(Pc(daysAgo: 30), Base, Today).Count == 0
            && Compliance.Evaluate(Pc(daysAgo: 31), Base, Today).Single().Message == "last patched 31 days ago");
        Check("unknown OS is a warning and skips the build check", () =>
            Compliance.Evaluate(Pc(os: "Windows 7", build: "1"), Base, Today).Single() == new Finding("pc-01", "warning", "unsupported OS Windows 7"));
        Check("all problems, in order", () =>
            string.Join("|", Compliance.Evaluate(Pc(build: "1", daysAgo: 90, enc: false, av: false), Base, Today).Select(f => f.Severity))
            == "critical|high|critical|high");
        Check("encryption only when required", () =>
            Compliance.Evaluate(Pc(enc: false), Base with { RequireEncryption = false }, Today).Count == 0);

        Console.WriteLine("ComplianceRate and FormatReport");
        var fleet = new List<Machine> { Pc() with { Name = "a" }, Pc() with { Name = "b" }, Pc() with { Name = "c" } };
        var findings = new List<Finding> { new("b", "high", "x"), new("b", "critical", "y") };
        Check("rate counts machines, not findings", () => Compliance.ComplianceRate(fleet, findings) == 66.7);
        Check("empty fleet is 100%", () => Compliance.ComplianceRate(new(), new()) == 100);
        Check("report sorts by severity and ends with the rate", () =>
        {
            var lines = Compliance.FormatReport(fleet, findings).Split('\n');
            return lines.Length == 4 && lines[1] == "[CRITICAL] b: y" && lines[2] == "[HIGH] b: x" && lines[3] == "Compliant: 2/3 (66.7%)";
        });

        Console.WriteLine($"\n{passed} passed, {failed} failed");
        return failed == 0 ? 0 : 1;
    }
}
@@file inventory.csv
name,os,build,last_patch,encrypted,av
tky-pc-001,Windows 11,26100.4946,2026-10-20,yes,yes
tky-pc-002,Windows 11,26100.3775,2026-08-02,yes,yes
tky-pc-003,Windows 10,19045.6036,2026-10-14,no,yes
osk-pc-004,Windows 11,26100.4946,2026-10-28,yes,no
osk-pc-005,Windows 7,7601.24544,2020-01-14,no,no
tky-srv-01,Ubuntu 24.04,6.8.0-85,2026-10-25,yes,yes
tky-srv-02,Ubuntu 24.04,6.8.0-40,2026-07-30,yes,yes
osk-srv-03,Windows Server 2022,20348.4171,2026-10-09,yes,yes
@@file baseline.json
{
  "minBuilds": {
    "Windows 11": "26100.4652",
    "Windows 10": "19045.6036",
    "Windows Server 2022": "20348.3932",
    "Ubuntu 24.04": "6.8.0-60"
  },
  "maxPatchAgeDays": 30,
  "requireEncryption": true
}
@@text
🎉 This is the kind of tool that gets noticed in an IT team: it turns "I think we're fine" into a number, plus a list of what to fix. Put it on GitHub with a README and a screenshot of the report.

@@lesson xp-event_log_parser Log Analyzer with LINQ
@@text
Load a large application log into records, then answer ops questions with LINQ: errors per hour, the top 10 slowest requests, which users hit the most errors, and the first error after each deployment. Then make it fast on a 1-million-line file.

**You'll practise:** records · LINQ (GroupBy, OrderBy, Take) · DateTime parsing · streaming with File.ReadLines · Stopwatch

## Milestones

1. **Model**: a `record LogEntry(DateTime Time, string Level, string User, int Ms, string Message)`.
2. **Parse**: TryParse each line, count bad lines instead of crashing.
3. **Queries**: the four questions, each as one LINQ expression.
4. **Scale**: generate 1,000,000 lines and measure; switch to File.ReadLines and compare memory.
5. **Output**: a short report and a CSV of errors per hour.

## You're done when

- All four answers are correct on the sample and the big file is processed in a few seconds.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"Show me how GroupBy works with a time bucket like the hour."*
- *"Why does File.ReadLines use less memory than File.ReadAllLines?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project event_log_parser Log Analyzer with LINQ
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Log Analyzer with LINQ

Load a large application log into records, then answer ops questions with LINQ: errors per hour, the top 10 slowest requests, which users hit the most errors, and the first error after each deployment. Then make it fast on a 1-million-line file.

## Milestones

- [ ] 1. Model: a `record LogEntry(DateTime Time, string Level, string User, int Ms, string Message)`.
- [ ] 2. Parse: TryParse each line, count bad lines instead of crashing.
- [ ] 3. Queries: the four questions, each as one LINQ expression.
- [ ] 4. Scale: generate 1,000,000 lines and measure; switch to File.ReadLines and compare memory.
- [ ] 5. Output: a short report and a CSV of errors per hour.

## Done when

- [ ] All four answers are correct on the sample and the big file is processed in a few seconds.
@@file LogAnalyzer.csproj
<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <OutputType>Exe</OutputType>
    <TargetFramework>net8.0</TargetFramework>
    <ImplicitUsings>enable</ImplicitUsings>
    <Nullable>enable</Nullable>
    <InvariantGlobalization>true</InvariantGlobalization>
    <RollForward>Major</RollForward>
  </PropertyGroup>
</Project>
@@file Program.cs
using System.Globalization;

var lines = File.ReadLines(args.Length > 0 ? args[0] : "app.log");
var entries = new List<LogEntry>();
var bad = 0;
foreach (var line in lines)
{
    var parts = line.Split(' ', 5);
    if (parts.Length == 5 && DateTime.TryParse(parts[0], CultureInfo.InvariantCulture, DateTimeStyles.None, out var time)
        && int.TryParse(parts[3].TrimEnd('m', 's'), out var ms))
        entries.Add(new LogEntry(time, parts[1], parts[2], ms, parts[4]));
    else
        bad++;
}
Console.WriteLine($"{entries.Count} entries, {bad} bad lines");
// TODO: errors per hour, slowest requests, users with most errors, first error after each deploy

record LogEntry(DateTime Time, string Level, string User, int Ms, string Message);
@@file app.log
2026-10-30T09:00:01 INFO ann 120ms GET /orders
2026-10-30T09:00:03 ERROR ken 1430ms POST /orders timeout from db
2026-10-30T09:15:10 INFO deploy 0ms deployed version 2.4.1
2026-10-30T09:15:42 ERROR ann 310ms GET /cart null reference
2026-10-30T10:02:00 WARN yui 980ms GET /search slow query

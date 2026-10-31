@@lesson c3-errors-files Exceptions, Files and JSON
@@concepts errors, files, json
@@text
## Exceptions

When something goes wrong at run time, .NET **throws** an exception. If nothing catches it, the program stops with a stack trace.

```csharp
try
{
    int[] ports = { 22, 80 };
    Console.WriteLine(ports[5]);
}
catch (IndexOutOfRangeException e)
{
    Console.WriteLine($"Oops: {e.Message}");
}
finally
{
    Console.WriteLine("finally always runs (close files, release locks)");
}
```

Catch the **specific** exceptions you can handle (`FileNotFoundException`, `FormatException`, `HttpRequestException`...). A bare `catch (Exception)` that hides every error makes problems impossible to find.

Throw your own when a caller gives you something invalid:

```csharp
void SetVlan(int id)
{
    if (id is < 1 or > 4094)
        throw new ArgumentOutOfRangeException(nameof(id), id, "VLAN IDs are 1-4094");
    Console.WriteLine($"vlan {id} set");
}
SetVlan(10);
try { SetVlan(5000); } catch (ArgumentException e) { Console.WriteLine(e.Message); }
```

`ArgumentOutOfRangeException` *is an* `ArgumentException`, so the `catch` matches it. Exceptions form a hierarchy, all ending at `Exception`.

## Files

`System.IO.File` has one-line helpers for small files:

```csharp
string path = Path.Combine(Path.GetTempPath(), "itquest_demo.txt");
File.WriteAllText(path, "line one\n");
File.AppendAllText(path, "line two\n");
foreach (string line in File.ReadAllLines(path))
    Console.WriteLine($"> {line}");
Console.WriteLine(File.Exists(path));
Console.WriteLine(new FileInfo(path).Length + " bytes");
File.Delete(path);
```

`Path.Combine` builds paths with the right separator for the OS (`\` on Windows, `/` on Linux). Never glue paths together with `+`.

For big files, read line by line instead of loading everything into memory:

```csharp,norun
foreach (string line in File.ReadLines(@"C:\Logs\huge.log"))   // lazy, one line at a time
    if (line.Contains("ERROR")) Console.WriteLine(line);
```

`using` makes sure a file (or any `IDisposable`) is closed, even if an exception happens:

```csharp
string p = Path.Combine(Path.GetTempPath(), "itquest_demo.csv");
using (var w = new StreamWriter(p))
{
    w.WriteLine("host,cpu");
    w.WriteLine("web01,35");
}   // closed here
Console.WriteLine(File.ReadAllText(p).Trim());
File.Delete(p);
```

## JSON with System.Text.Json

```csharp
using System.Text.Json;

var cfg = new Config("api.example.jp", 443, ["/health", "/status"]);
var options = new JsonSerializerOptions { WriteIndented = true, PropertyNamingPolicy = JsonNamingPolicy.CamelCase };
string json = JsonSerializer.Serialize(cfg, options);
Console.WriteLine(json);

var back = JsonSerializer.Deserialize<Config>("""{"host":"db01","port":5432,"paths":[]}""", options);
Console.WriteLine(back);

record Config(string Host, int Port, List<string> Paths);
```

`Deserialize<T>` turns JSON into your type, matching property names. For JSON whose shape you don't know, `JsonDocument.Parse(text).RootElement` lets you walk it: `.GetProperty("host").GetString()`.

@@quiz
Which `catch` is usually the **worst** choice?
* `catch (Exception) { }` with an empty body
- `catch (FileNotFoundException e) { log(e); }`
- `catch (FormatException) { askAgain(); }`
- no catch at all for a bug you can't handle
? An empty catch-all hides every error, including bugs. Catch what you can actually handle.

@@quiz
Why use `Path.Combine("logs", "app.log")` instead of `"logs" + "\\" + "app.log"`?
* It uses the right separator on every OS and handles extra slashes
- It's the only way to make a path
- It checks the file exists
- It's faster
? Code that builds paths with `\` breaks on Linux. `Path.Combine` is portable.

@@quiz
What does `using (var w = new StreamWriter(p)) { ... }` guarantee?
* The file is closed when the block ends, even if an exception is thrown
- The file is deleted afterwards
- Nothing else can write to the file ever
- The file is created only if it doesn't exist
? `using` calls `Dispose()` at the end of the block, which closes the file.

@@exercise Parse a config file safely
`ParseLine` gets lines like `timeout = 30`. Return the key and the number as a tuple. Throw `FormatException` with the message `bad line: <line>` if there's no `=` or the value isn't a whole number. The loop prints good lines and catches bad ones.
@@starter
(string Key, int Value) ParseLine(string line)
{
    throw new NotImplementedException("TODO: not done yet");
}

string[] lines = { "timeout = 30", "retries=3", "verbose", "port = eighty" };
foreach (var l in lines)
{
    try
    {
        var (k, v) = ParseLine(l);
        Console.WriteLine($"{k} -> {v}");
    }
    catch (FormatException e)
    {
        Console.WriteLine(e.Message);
    }
}
@@expected
timeout -> 30
retries -> 3
bad line: verbose
bad line: port = eighty
@@requires
throw new FormatException
@@hint
`var parts = line.Split('=', 2);` If `parts.Length != 2` or `!int.TryParse(parts[1].Trim(), out int v)`, throw. Otherwise return `(parts[0].Trim(), v)`.
@@solution
(string Key, int Value) ParseLine(string line)
{
    var parts = line.Split('=', 2);
    if (parts.Length != 2 || !int.TryParse(parts[1].Trim(), out int value))
        throw new FormatException($"bad line: {line}");
    return (parts[0].Trim(), value);
}

string[] lines = { "timeout = 30", "retries=3", "verbose", "port = eighty" };
foreach (var l in lines)
{
    try
    {
        var (k, v) = ParseLine(l);
        Console.WriteLine($"{k} -> {v}");
    }
    catch (FormatException e)
    {
        Console.WriteLine(e.Message);
    }
}

@@exercise Users from JSON
Deserialize the JSON into a list of `User` records, then print the disabled users' names in alphabetical order, one per line, then `enabled: <count>`.
@@starter
using System.Text.Json;

string json = """
[
  {"name": "tanaka", "dept": "sales", "enabled": true},
  {"name": "suzuki", "dept": "it", "enabled": false},
  {"name": "abe", "dept": "it", "enabled": false},
  {"name": "mori", "dept": "hr", "enabled": true}
]
""";
var options = new JsonSerializerOptions { PropertyNameCaseInsensitive = true };
// deserialize and print

record User(string Name, string Dept, bool Enabled);
@@expected
abe
suzuki
enabled: 2
@@requires
JsonSerializer.Deserialize
@@hint
`var users = JsonSerializer.Deserialize<List<User>>(json, options)!;` The `!` tells the compiler "I know it isn't null".
@@solution
using System.Text.Json;

string json = """
[
  {"name": "tanaka", "dept": "sales", "enabled": true},
  {"name": "suzuki", "dept": "it", "enabled": false},
  {"name": "abe", "dept": "it", "enabled": false},
  {"name": "mori", "dept": "hr", "enabled": true}
]
""";
var options = new JsonSerializerOptions { PropertyNameCaseInsensitive = true };
var users = JsonSerializer.Deserialize<List<User>>(json, options)!;
foreach (var u in users.Where(u => !u.Enabled).OrderBy(u => u.Name))
    Console.WriteLine(u.Name);
Console.WriteLine($"enabled: {users.Count(u => u.Enabled)}");

record User(string Name, string Dept, bool Enabled);

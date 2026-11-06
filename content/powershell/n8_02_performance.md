@@lesson n8-performance Performance: Measure, Then Fix the Big Things
@@concepts lists, text-processing
@@text
PowerShell is fast enough for nearly all admin work, but a few patterns turn a one-second script into a one-hour one. Learn to **measure** first, then fix the patterns that matter.

## Measuring

```powershell,norun
Measure-Command { .\Build-Report.ps1 } | Select-Object TotalSeconds
$sw = [System.Diagnostics.Stopwatch]::StartNew(); ...; $sw.Elapsed.TotalMilliseconds
```

Measure several runs, ignore the first (JIT compilation, caches), and measure with realistic data sizes: most performance problems only appear at 10,000+ items.

## The big ones

| Slow pattern | Why | Fast alternative |
|---|---|---|
| `$arr += $x` in a loop | copies the whole array each time | `$list = foreach (...) { $x }` or `List[T]` |
| `$s += "line"` building big strings | strings are immutable: copies each time | `[System.Text.StringBuilder]` or `-join` |
| `Where-Object` inside a loop to look things up | O(n²) | build a hashtable index once: O(1) lookups |
| `Get-Content big.log \| ForEach-Object` | one pipeline object per line | `[System.IO.File]::ReadLines()` or `Get-Content -ReadCount 1000` / `-Raw` |
| `\| Out-Null` | pipeline overhead | `$null = ...` or `[void]` |
| calling a remote/AD/API command per item | network round trips | one bulk query, then filter locally |
| `ForEach-Object` in hot loops | script block invocation per item | `foreach ($x in $items)` statement |

## Lookups: the most common fix

```powershell
$users = 1..3000 | ForEach-Object { [pscustomobject]@{ Id = $_; Name = "user$_" } }
$index = @{}
foreach ($u in $users) { $index[$u.Id] = $u }      # build once
$index[2999].Name                                    # instant, instead of $users | Where-Object Id -eq 2999
```

`Group-Object -AsHashTable -AsString` builds an index too, but a `foreach` loop is faster for big inputs.

## Filtering left

Filter as early as possible, ideally at the **source**:

```powershell,norun
Get-ADUser -Filter "Enabled -eq 'True'" -Properties LastLogonDate     # the DC filters
Get-ADUser -Filter * -Properties * | Where-Object Enabled              # downloads everything first
Get-ChildItem C:\logs -Filter *.log                                    # file system filters (fast)
Get-ChildItem C:\logs | Where-Object Extension -eq '.log'              # PowerShell filters (slow)
Get-WinEvent -FilterHashtable @{ LogName = 'Security'; Id = 4625; StartTime = (Get-Date).AddDays(-1) }
```

## The pipeline is not free, but it streams

`foreach ($x in $items)` is faster than `$items | ForEach-Object`, but needs everything in memory. For a 20 GB log, the streaming pipeline (or `ReadLines`) is what keeps memory flat. Choose speed or streaming consciously.

## .NET when it counts

`[regex]::Matches()`, `[System.IO.File]::ReadLines()`, `[string]::Join()`, `HashSet[T]` and `StringBuilder` are often 10–100× faster than their cmdlet equivalents in tight loops. Don't rewrite everything in .NET style: only the loop that the stopwatch says is slow.

@@quiz
A script looks up each of 20,000 log lines' user in a list of 5,000 users with `Where-Object`. What's the fix?
* Build a hashtable keyed by user once, then look up each line in O(1)
- Use ForEach-Object -Parallel
- Sort the users first
- Use Select-Object -First 1
? 20,000 × 5,000 comparisons vs 25,000 operations.

@@quiz
What does "filter left" mean?
- Put Where-Object at the start of the line
* Filter at the source (the -Filter parameter, the server, the query) before data reaches the pipeline
- Sort before filtering
- Filter in a left join
? Less data transferred, less work.

@@quiz
Which command tells you how long a script block takes?
* Measure-Command { ... }
- Get-Time { ... }
- Start-Stopwatch
- Trace-Command -Duration
? Measure first, so you fix the part that's actually slow.

@@flashcards
`Measure-Command` :: time a script block
`Stopwatch` :: precise timing
`+=` on arrays :: copies every time
`StringBuilder` :: efficient string building
hashtable index :: O(1) lookups
filter left :: filter at the source
`-Filter` :: provider/server-side filtering
`[IO.File]::ReadLines` :: fast streaming file read
`Get-Content -Raw` :: whole file as one string
`$null =` :: cheapest way to discard output

@@exercise Same answer, less work
`Get-SlowReport` joins log lines to users with `Where-Object` inside a loop and builds the output with `+=`. Write `Get-FastReport` that returns the **identical** string using a hashtable index, a `foreach` statement and a `StringBuilder` (or `-join`).

Print both results' line counts, whether they're identical, and whether the fast version took less time (only print the boolean, since timings vary).
@@starter
$users = 1..200 | ForEach-Object { [pscustomobject]@{ Id = $_; Name = "user$_"; Dept = @('IT', 'Sales', 'HR')[$_ % 3] } }
$log = 1..300 | ForEach-Object { [pscustomobject]@{ UserId = ($_ * 7) % 200 + 1; Action = @('login', 'logout', 'print')[$_ % 3] } }
function Get-SlowReport {
    $out = ''
    foreach ($entry in $log) {
        $u = $users | Where-Object Id -eq $entry.UserId
        $out += "$($u.Name) ($($u.Dept)) $($entry.Action)`n"
    }
    $out
}
'TODO'
@@expected
lines: 300 and 300
identical: True
fast is faster: True
@@requires
StringBuilder
foreach (
$index[
@@hint
Index: `$index = @{}; foreach ($u in $users) { $index[$u.Id] = $u }`. Build: `$sb = [System.Text.StringBuilder]::new(); foreach ($e in $log) { $u = $index[$e.UserId]; [void]$sb.Append("$($u.Name) ($($u.Dept)) $($e.Action)`n") }; $sb.ToString()`. Time each with `Measure-Command` and count lines with `($r -split "`n").Count - 1` (the trailing newline adds an empty item). The data is small so the slow version finishes quickly here; try 20,000 log lines yourself to feel the difference.
@@solution
$users = 1..200 | ForEach-Object { [pscustomobject]@{ Id = $_; Name = "user$_"; Dept = @('IT', 'Sales', 'HR')[$_ % 3] } }
$log = 1..300 | ForEach-Object { [pscustomobject]@{ UserId = ($_ * 7) % 200 + 1; Action = @('login', 'logout', 'print')[$_ % 3] } }
function Get-SlowReport {
    $out = ''
    foreach ($entry in $log) {
        $u = $users | Where-Object Id -eq $entry.UserId
        $out += "$($u.Name) ($($u.Dept)) $($entry.Action)`n"
    }
    $out
}

function Get-FastReport {
    $index = @{}
    foreach ($u in $users) { $index[$u.Id] = $u }
    $sb = [System.Text.StringBuilder]::new()
    foreach ($e in $log) {
        $u = $index[$e.UserId]
        [void]$sb.Append("$($u.Name) ($($u.Dept)) $($e.Action)`n")
    }
    $sb.ToString()
}

$slowTime = Measure-Command { $script:slow = Get-SlowReport }
$fastTime = Measure-Command { $script:fast = Get-FastReport }
"lines: $(($slow -split "`n").Count - 1) and $(($fast -split "`n").Count - 1)"
"identical: $($slow -ceq $fast)"
"fast is faster: $($fastTime -lt $slowTime)"

@@exercise Read a big log efficiently
Generate a 30,000-line log file in a temp folder with a `StreamWriter` (fast), then count `ERROR` lines per hour three ways and check they agree:

1. `Get-Content` piped to `ForEach-Object` with `-match`
2. `[System.IO.File]::ReadLines()` in a `foreach` with `.StartsWith` / `.Substring`
3. `Select-String` with `-SimpleMatch`

Print the counts per hour (sorted) from method 2, and whether all three totals are equal. Don't print timings (they vary), but look at them yourself with `Measure-Command`.
@@starter
$path = Join-Path ([System.IO.Path]::GetTempPath()) "big-$PID.log"
'TODO'
Remove-Item $path -ErrorAction SilentlyContinue
@@expected
hour 08: 1000 errors
hour 09: 1000 errors
hour 10: 1000 errors
total 3000, all methods agree: True
@@requires
StreamWriter
ReadLines
Select-String
@@hint
Write line `i` (0-based) as `"2026-11-03 {0:D2}:{1:D2}:00 {2} message {3}" -f (8 + $i % 3), ($i % 60), $(if ($i % 10 -eq 0) { 'ERROR' } else { 'INFO' }), $i`, using `$w = [System.IO.StreamWriter]::new($path)` and `$w.WriteLine(...)`, then `$w.Close()`. In method 2, the level starts at character 20: `$line.Substring(20, 5) -eq 'ERROR'`, and the hour is `$line.Substring(11, 2)`.
@@solution
$path = Join-Path ([System.IO.Path]::GetTempPath()) "big-$PID.log"

$w = [System.IO.StreamWriter]::new($path)
for ($i = 0; $i -lt 30000; $i++) {
    if ($i % 10 -eq 0) { $level = 'ERROR' } else { $level = 'INFO ' }
    $w.WriteLine(('2026-11-03 {0:D2}:{1:D2}:00 {2} message {3}' -f (8 + $i % 3), ($i % 60), $level, $i))
}
$w.Close()

$count1 = 0
Get-Content $path | ForEach-Object { if ($_ -match ' ERROR ') { $count1++ } }

$byHour = @{}
foreach ($line in [System.IO.File]::ReadLines($path)) {
    if ($line.Substring(20, 5) -eq 'ERROR') {
        $h = $line.Substring(11, 2)
        $byHour[$h] = $byHour[$h] + 1
    }
}
$count2 = ($byHour.Values | Measure-Object -Sum).Sum

$count3 = @(Select-String -Path $path -Pattern ' ERROR ' -SimpleMatch).Count

foreach ($h in $byHour.Keys | Sort-Object) { "hour ${h}: $($byHour[$h]) errors" }
"total $count2, all methods agree: $($count1 -eq $count2 -and $count2 -eq $count3)"
Remove-Item $path -ErrorAction SilentlyContinue

@@lesson n8-logging Logging and Observability for Scripts
@@concepts files, json, automation
@@text
A scheduled script that fails at 03:00 is only fixable if it left a good trail. Professional scripts write **structured** logs, rotate them, report to the Windows event log or a central system, and end with an exit code a scheduler understands.

## Levels and structure

Write one event per line with a timestamp (ISO 8601, UTC), a level, a message and **fields**:

```text
2026-11-03T02:00:01Z INFO  backup.start  job=nightly target=\\nas01\backup
2026-11-03T02:14:37Z ERROR backup.copy   job=nightly file=db.bak error="disk full"
```

Or as JSON lines (one JSON object per line), which log platforms (Elastic, Splunk, Loki, Azure Monitor) ingest directly and which you can query with `ConvertFrom-Json`:

```json
{"ts":"2026-11-03T02:14:37Z","level":"ERROR","event":"backup.copy","job":"nightly","file":"db.bak","error":"disk full"}
```

Levels: `DEBUG` (developer detail), `INFO` (normal milestones), `WARN` (unexpected but handled), `ERROR` (a step failed), `FATAL` (the run stopped). Filter by level at write time with a configurable threshold.

## A small logging function

```powershell,norun
function Write-Log {
    param([ValidateSet('DEBUG','INFO','WARN','ERROR')][string]$Level, [string]$Event, [hashtable]$Data = @{})
    $entry = [ordered]@{ ts = (Get-Date).ToUniversalTime().ToString('o'); level = $Level; event = $Event }
    foreach ($k in $Data.Keys) { $entry[$k] = $Data[$k] }
    Add-Content -Path $script:LogPath -Value ($entry | ConvertTo-Json -Compress) -Encoding UTF8
}
```

## Transcripts

`Start-Transcript -Path C:\Logs\job.txt -Append` records everything shown on the console (including errors) until `Stop-Transcript`. It's a quick safety net for old scripts, not a replacement for structured logs.

## The Windows event log

Monitoring tools already watch the event log, so writing important events there integrates with existing alerting:

```powershell,norun
New-EventLog -LogName Application -Source 'BackupScript'      # once, as admin
Write-EventLog -LogName Application -Source 'BackupScript' -EventId 1001 -EntryType Error -Message 'Nightly backup failed: disk full'
```

Use stable event IDs (1000 = started, 1001 = failed, 1002 = finished) so alerts can match on them.

## Rotation

Logs grow forever unless you rotate them: one file per day (`job-2026-11-03.log`) and delete files older than N days, or roll over at a size limit.

## Exit codes and summaries

End every unattended script with:

- a **summary** line (counts, duration, result)
- an **exit code**: 0 success, non-zero failure (and document what each means); Task Scheduler, SCCM/Intune and CI all read it

```powershell,norun
exit $(if ($failed -gt 0) { 2 } else { 0 })
```

## Correlation

When one run touches many systems, generate a **run id** (`[guid]::NewGuid()`) and put it on every log line and every API call you make. Then one search shows the whole story of one run across all logs.

@@quiz
Why are JSON-lines logs useful?
* Each line is a complete structured event that tools (and ConvertFrom-Json) can parse and query by field
- They're smaller than text
- Windows requires them
- They can't be edited
? Search "event=backup.copy AND level=ERROR" instead of grepping text.

@@quiz
What should an unattended script do at the end?
- Print "done"
* Write a summary and exit with a code that tells the scheduler whether it succeeded
- Clear its log
- Restart the computer
? Exit codes drive alerts and retries.

@@quiz
Why put a correlation ID in every log line of one run?
* So all lines from that run can be found together, even when several runs or machines write to the same log
- It encrypts the log
- It makes lines shorter
- Windows requires it
? Search for one ID to see a run's whole story.

@@flashcards
log levels :: DEBUG, INFO, WARN, ERROR, FATAL
JSON lines :: one JSON object per line
ISO 8601 UTC :: `ToUniversalTime().ToString('o')`
`Start-Transcript` :: record the console session
`Write-EventLog` :: write to the Windows event log
event id :: stable number for alerting
rotation :: per-day files + retention
run id :: correlate all events of one run
exit code :: 0 success, non-zero failure

@@exercise A structured logger
Write a logger with a level threshold:

- `New-Logger -Path -MinLevel -RunId` returns a hashtable with the settings
- `Write-Log -Logger -Level -Event -Data` appends a JSON line (keys in order: `ts`, `level`, `run`, `event`, then the data keys sorted) when the level is at or above the threshold
- use a fixed clock (`$script:Clock`, advanced by 1 second per call) so the output is stable
- simulate a backup job, then read the log back with `ConvertFrom-Json` and print: the number of lines, the errors with their file, and the number of events per level
@@starter
$script:Clock = [datetime]::SpecifyKind([datetime]'2026-11-03 02:00:00', 'Utc')
$path = Join-Path ([System.IO.Path]::GetTempPath()) "log-$PID.jsonl"
$files = @(@{ Name = 'users.db'; Ok = $true }, @{ Name = 'mail.pst'; Ok = $false }, @{ Name = 'wiki.tar'; Ok = $true }, @{ Name = 'db.bak'; Ok = $false })
'TODO'
Remove-Item $path -ErrorAction SilentlyContinue
@@expected
{"ts":"2026-11-03T02:00:00Z","level":"INFO","run":"run-42","event":"backup.start","files":4}
lines: 7
error at 2026-11-03T02:00:02Z: mail.pst (disk full)
error at 2026-11-03T02:00:04Z: db.bak (disk full)
ERROR: 2
INFO: 4
WARN: 1
@@requires
ConvertTo-Json -Compress
ConvertFrom-Json
[ordered]
@@hint
Levels as numbers: `$rank = @{ DEBUG = 0; INFO = 1; WARN = 2; ERROR = 3 }`; skip when `$rank[$Level] -lt $rank[$Logger.MinLevel]`. Timestamp: `$script:Clock.ToString("yyyy-MM-dd'T'HH:mm:ss'Z'")`, then `$script:Clock = $script:Clock.AddSeconds(1)` (only for written lines). Log per file: DEBUG `backup.file` (filtered out with MinLevel INFO), then INFO `backup.copied` or ERROR `backup.failed` with `file` and `error`. Finish with WARN `backup.partial` and INFO `backup.end`. Print the first line of the file raw. `ConvertFrom-Json` in 5.1 leaves the ISO timestamps as strings, so you can print `ts` directly.
@@solution
$script:Clock = [datetime]::SpecifyKind([datetime]'2026-11-03 02:00:00', 'Utc')
$path = Join-Path ([System.IO.Path]::GetTempPath()) "log-$PID.jsonl"
$files = @(@{ Name = 'users.db'; Ok = $true }, @{ Name = 'mail.pst'; Ok = $false }, @{ Name = 'wiki.tar'; Ok = $true }, @{ Name = 'db.bak'; Ok = $false })

$rank = @{ DEBUG = 0; INFO = 1; WARN = 2; ERROR = 3 }

function New-Logger([string]$Path, [string]$MinLevel = 'INFO', [string]$RunId) {
    @{ Path = $Path; MinLevel = $MinLevel; RunId = $RunId }
}

function Write-Log([hashtable]$Logger, [string]$Level, [string]$Event, [hashtable]$Data = @{}) {
    if ($rank[$Level] -lt $rank[$Logger.MinLevel]) { return }
    $entry = [ordered]@{
        ts    = $script:Clock.ToString("yyyy-MM-dd'T'HH:mm:ss'Z'")
        level = $Level
        run   = $Logger.RunId
        event = $Event
    }
    foreach ($k in $Data.Keys | Sort-Object) { $entry[$k] = $Data[$k] }
    $script:Clock = $script:Clock.AddSeconds(1)
    Add-Content -Path $Logger.Path -Value ($entry | ConvertTo-Json -Compress) -Encoding UTF8
}

$log = New-Logger -Path $path -MinLevel INFO -RunId 'run-42'
Write-Log $log INFO 'backup.start' @{ files = $files.Count }
$failed = 0
foreach ($f in $files) {
    Write-Log $log DEBUG 'backup.file' @{ file = $f.Name }
    if ($f.Ok) {
        Write-Log $log INFO 'backup.copied' @{ file = $f.Name }
    } else {
        Write-Log $log ERROR 'backup.failed' @{ file = $f.Name; error = 'disk full' }
        $failed++
    }
}
if ($failed) { Write-Log $log WARN 'backup.partial' @{ failed = $failed } }
Write-Log $log INFO 'backup.end' @{ failed = $failed }

$lines = Get-Content $path -Encoding UTF8
$lines[0]
"lines: $($lines.Count)"
$events = $lines | ForEach-Object { $_ | ConvertFrom-Json }
$events | Where-Object level -eq 'ERROR' | ForEach-Object { "error at $($_.ts): $($_.file) ($($_.error))" }
$events | Group-Object level | Sort-Object Name | ForEach-Object { "$($_.Name): $($_.Count)" }
Remove-Item $path -ErrorAction SilentlyContinue

@@exercise Rotation and an exit code
Write `Invoke-LogRotation -Folder -Keep` that keeps the newest N daily log files named `job-yyyy-MM-dd.log` (judge age by the **date in the name**, not the file time) and deletes the rest, returning the deleted names. Then write `Get-RunResult` that turns counts into a summary line and an exit code: 0 all good, 1 warnings only, 2 any failure.

Create 10 daily files in a temp folder, rotate keeping 7, and show three run results.
@@starter
$dir = Join-Path ([System.IO.Path]::GetTempPath()) "rot-$PID"
New-Item -ItemType Directory -Path $dir -Force | Out-Null
$start = [datetime]'2026-10-25'
0..9 | ForEach-Object { Set-Content -Path (Join-Path $dir ('job-{0}.log' -f $start.AddDays($_).ToString('yyyy-MM-dd'))) -Value 'x' }
Set-Content -Path (Join-Path $dir 'notes.txt') -Value 'keep me'
'TODO'
Remove-Item -Recurse -Force $dir
@@expected
deleted: job-2026-10-25.log, job-2026-10-26.log, job-2026-10-27.log
remaining logs: 7, oldest kept: job-2026-10-28.log, notes.txt kept: True
ok=12 warn=0 fail=0 -> SUCCESS (exit 0)
ok=10 warn=2 fail=0 -> WARNINGS (exit 1)
ok=9 warn=1 fail=2 -> FAILED (exit 2)
@@requires
ParseExact
Remove-Item
@@hint
List files with `Get-ChildItem $Folder -Filter 'job-*.log'`, parse the date with `[datetime]::ParseExact($_.BaseName.Substring(4), 'yyyy-MM-dd', $null)`, sort by it descending, and `Select-Object -Skip $Keep` gives the ones to delete. `Get-RunResult` returns `[pscustomobject]@{ Text = ...; ExitCode = ... }`; a real script would end with `exit $result.ExitCode`.
@@solution
$dir = Join-Path ([System.IO.Path]::GetTempPath()) "rot-$PID"
New-Item -ItemType Directory -Path $dir -Force | Out-Null
$start = [datetime]'2026-10-25'
0..9 | ForEach-Object { Set-Content -Path (Join-Path $dir ('job-{0}.log' -f $start.AddDays($_).ToString('yyyy-MM-dd'))) -Value 'x' }
Set-Content -Path (Join-Path $dir 'notes.txt') -Value 'keep me'

function Invoke-LogRotation([string]$Folder, [int]$Keep) {
    $logs = Get-ChildItem $Folder -Filter 'job-*.log' | ForEach-Object {
        [pscustomobject]@{ File = $_; Date = [datetime]::ParseExact($_.BaseName.Substring(4), 'yyyy-MM-dd', $null) }
    }
    $old = $logs | Sort-Object Date -Descending | Select-Object -Skip $Keep | Sort-Object Date
    foreach ($o in $old) {
        Remove-Item $o.File.FullName
        $o.File.Name
    }
}

function Get-RunResult([int]$Ok, [int]$Warn, [int]$Fail) {
    if ($Fail -gt 0) { $state = 'FAILED'; $code = 2 }
    elseif ($Warn -gt 0) { $state = 'WARNINGS'; $code = 1 }
    else { $state = 'SUCCESS'; $code = 0 }
    [pscustomobject]@{ Text = "ok=$Ok warn=$Warn fail=$Fail -> $state (exit $code)"; ExitCode = $code }
}

$deleted = Invoke-LogRotation -Folder $dir -Keep 7
"deleted: $($deleted -join ', ')"
$left = Get-ChildItem $dir -Filter 'job-*.log' | Sort-Object Name
"remaining logs: $($left.Count), oldest kept: $($left[0].Name), notes.txt kept: $(Test-Path (Join-Path $dir 'notes.txt'))"
(Get-RunResult 12 0 0).Text
(Get-RunResult 10 2 0).Text
(Get-RunResult 9 1 2).Text
Remove-Item -Recurse -Force $dir

@@lesson p3-scheduled-automation Running Scripts Unattended: Scheduled Tasks, Exit Codes and Idempotent Changes
@@concepts automation, processes, conditions, loops, maps
@@text
A script you run by hand can ask questions, print colourful output and stop when something looks odd, because you're watching. A script that runs at 3 a.m. from Task Scheduler has nobody watching. It must find its own files, log what it did, report failure through its exit code, avoid prompting, and be safe to run twice. This lesson covers scheduling with PowerShell and the habits that make unattended scripts reliable.

## Creating a scheduled task

The ScheduledTasks module builds a task from four parts: an **action** (what to run), a **trigger** (when), a **principal** (as whom) and **settings** (how).

```powershell,norun
$action = New-ScheduledTaskAction -Execute 'powershell.exe' `
    -Argument '-NoProfile -NonInteractive -ExecutionPolicy Bypass -File "C:\Scripts\Cleanup.ps1"' `
    -WorkingDirectory 'C:\Scripts'

$trigger = New-ScheduledTaskTrigger -Daily -At '03:15'

$principal = New-ScheduledTaskPrincipal -UserId 'SYSTEM' -LogonType ServiceAccount -RunLevel Highest

$settings = New-ScheduledTaskSettingsSet -StartWhenAvailable -ExecutionTimeLimit (New-TimeSpan -Hours 1) `
    -RestartCount 3 -RestartInterval (New-TimeSpan -Minutes 10) -MultipleInstances IgnoreNew

Register-ScheduledTask -TaskName 'Nightly cleanup' -TaskPath '\Ops\' -Action $action `
    -Trigger $trigger -Principal $principal -Settings $settings -Description 'Removes temp files older than 14 days'
```

What each choice means:

- **`-NoProfile`** keeps the run independent of whatever is in a profile script. **`-NonInteractive`** makes any prompt an error instead of a hang. Use **`pwsh.exe`** instead of `powershell.exe` for PowerShell 7.
- **`-WorkingDirectory`** matters because tasks start in `C:\Windows\System32` by default. Better still, build paths from **`$PSScriptRoot`** inside the script, so it never depends on the working directory.
- **Triggers**: `-Daily -At`, `-Weekly -DaysOfWeek Monday,Thursday -At`, `-AtStartup`, `-AtLogOn`, `-Once -At ... -RepetitionInterval (New-TimeSpan -Minutes 15) -RepetitionDuration (New-TimeSpan -Hours 8)` for "every 15 minutes during working hours". A task can have several triggers.
- **Principal**: `SYSTEM` has full local rights but no network identity beyond the computer account. For network shares or remote APIs, use a dedicated service account, ideally a **group managed service account** (gMSA) whose password Windows rotates for you. Avoid running tasks as a person's account: they change jobs and the task breaks.
- **Settings**: `-StartWhenAvailable` runs a missed schedule after the machine was off; `-ExecutionTimeLimit` kills runaway runs; `-RestartCount` retries failures; `-MultipleInstances IgnoreNew` stops overlapping runs.

Manage tasks with `Get-ScheduledTask -TaskPath '\Ops\'`, `Start-ScheduledTask`, `Disable-ScheduledTask`, `Unregister-ScheduledTask -Confirm:$false`, and check results with `Get-ScheduledTaskInfo` (its `LastTaskResult` is the script's exit code, 0 meaning success, and `NextRunTime` shows when it runs next). `Export-ScheduledTask` produces XML you can keep in source control and import elsewhere.

## Exit codes: how the scheduler knows

Task Scheduler, CI systems and monitoring tools all judge a script by its **exit code**. PowerShell's rules:

- `exit 0` means success; `exit 1` (or any non-zero number) means failure. Pick a few meaningful codes and document them (1 = failed, 2 = bad arguments, 3 = partial success).
- An unhandled **terminating error** in a script run with `-File` gives exit code 1.
- **Non-terminating errors don't change the exit code.** A script whose `Remove-Item` fails on half the files still exits 0 unless you act on it. Set `$ErrorActionPreference = 'Stop'` at the top so errors become terminating, and catch the ones you expect.
- With `-Command`, the exit code reflects only whether the **last** command succeeded, unless you call `exit` yourself. Prefer `-File` for scheduled scripts.
- The exit code of the last native program is in `$LASTEXITCODE`; check it after calling `robocopy`, `git` or `msiexec`. (Robocopy is a classic trap: codes 0 to 7 all mean some kind of success, and 8 or higher means failure.)

A reliable skeleton:

```powershell,norun
[CmdletBinding()]
param([int]$Days = 14)
$ErrorActionPreference = 'Stop'
$log = Join-Path $PSScriptRoot ("logs\cleanup-{0:yyyy-MM-dd}.log" -f (Get-Date))
Start-Transcript -Path $log -Append | Out-Null
try {
    # ... the actual work ...
    Write-Output "Done"
    exit 0
}
catch {
    Write-Error "Cleanup failed: $_"
    exit 1
}
finally {
    Stop-Transcript | Out-Null
}
```

## Logging so you can find out what happened

- **`Start-Transcript`** records everything the script writes to the console (output, warnings, errors, verbose messages if enabled) in a text file. It's the quickest way to get a log from an existing script.
- Write **structured lines** with a timestamp and level: `"{0:o} INFO removed {1} files" -f (Get-Date), $count`. Log to a file per day and delete old ones, or the log folder becomes the next disk-space problem.
- For events that monitoring should see, write to the **Windows event log**: `New-EventLog -LogName Application -Source 'OpsScripts'` once (as administrator), then `Write-EventLog -LogName Application -Source 'OpsScripts' -EventId 1001 -EntryType Error -Message ...`. (In PowerShell 7, `Write-EventLog` is gone; use `[System.Diagnostics.EventLog]::WriteEntry` or a logging module.)
- Never log secrets. Watch out for transcripts capturing passwords that a script prints or receives as plain parameters.

## Secrets without prompts

Unattended scripts can't type a password, and passwords in script files end up in backups and repositories. Options, from best to worst:

1. **Use the run-as identity itself**: a gMSA or computer account that has the rights it needs, so there's no secret at all.
2. **SecretManagement** with a vault (`Get-Secret -Name 'ApiToken' -AsPlainText` from the Microsoft.PowerShell.SecretManagement module and a vault extension).
3. **DPAPI**: `Get-Credential | Export-Clixml cred.xml` encrypts the password so only **the same user on the same machine** can read it back with `Import-Clixml`. Create it while logged in as the account the task runs as.

## Idempotence: safe to run twice

A scheduled script will sometimes run twice: a retry after a timeout, an overlap, someone pressing "Run" to test. An **idempotent** script gives the same result however many times it runs. The pattern is **test, then set**:

```powershell,norun
$path = 'C:\Data\Exports'
if (-not (Test-Path $path)) {
    New-Item -ItemType Directory -Path $path | Out-Null
    Write-Output "created $path"
}
$svc = Get-Service -Name 'Spooler'
if ($svc.StartType -ne 'Disabled') {
    Set-Service -Name 'Spooler' -StartupType Disabled
    Write-Output 'disabled Spooler'
}
```

Compare this with "append a line to the hosts file" or "add 10% to every price": run twice, and the result is wrong. Make such steps idempotent by checking first (is the line already there?) or by recording what's done (a marker file, a processed-ID list).

Idempotent scripts can also offer a **`-WhatIf`** mode for free: add `[CmdletBinding(SupportsShouldProcess)]` and wrap each change in `if ($PSCmdlet.ShouldProcess($target, $action)) { ... }`. Then `.\Configure.ps1 -WhatIf` prints what *would* change without changing anything. That's exactly the idea behind **Desired State Configuration** (DSC) and tools like Ansible: describe the desired state, test each item, and set only what differs.

## Checklist for an unattended script

1. `-NoProfile -NonInteractive -File`, and paths from `$PSScriptRoot`.
2. `$ErrorActionPreference = 'Stop'`, `try`/`catch`, explicit `exit` codes.
3. A log per run (transcript or structured lines) with old logs cleaned up.
4. No prompts and no plain-text secrets.
5. Idempotent steps, with `-WhatIf` support for changes.
6. A run identity with only the rights it needs.
7. A way to notice failure: `LastTaskResult` monitored, an event log entry, or an alert.

@@quiz
A scheduled script fails to delete some files but still reports success. What's the most likely reason?
* The errors were non-terminating, so the script exited 0; set $ErrorActionPreference = 'Stop' and exit with a non-zero code on failure
- Task Scheduler ignores exit codes
- Remove-Item never reports errors
- The task ran as SYSTEM
? Non-terminating errors don't change the exit code.

@@quiz
Why add -NonInteractive to the task's PowerShell arguments?
* So any prompt becomes an error instead of hanging the task forever
- To hide the console window
- To run with administrator rights
- To skip the execution policy
? Nobody is there to answer a prompt at 3 a.m.

@@quiz
A task's script uses relative paths and works when run by hand but not from Task Scheduler. Why?
* Tasks start in C:\Windows\System32 unless a working directory is set; build paths from $PSScriptRoot
- Task Scheduler doesn't support relative paths in PowerShell 7
- The execution policy blocks relative paths
- Scheduled tasks can't read files
? $PSScriptRoot is the folder the script lives in, wherever it's started from.

@@quiz
What does it mean for a script to be idempotent?
* Running it again gives the same end state as running it once
- It never produces output
- It only runs once per day
- It can't fail
? Test first, then set only what differs.

@@quiz
Which identity is best for a scheduled task that needs to read a network share?
* A group managed service account (gMSA) with only the rights it needs
- The administrator's own account
- SYSTEM, which has full rights everywhere on the network
- Any user who is logged on
? Windows rotates a gMSA's password automatically, and no person's account is involved.

@@quiz
Robocopy finished with exit code 3. Did it fail?
* No: robocopy codes 0-7 mean success of some kind; 8 or higher means failure
- Yes: any non-zero code is a failure
- Only if $ErrorActionPreference is Stop
- It depends on the task's principal
? Check $LASTEXITCODE -ge 8 for robocopy.

@@flashcards
New-ScheduledTaskAction :: what to run (program, arguments, working directory)
New-ScheduledTaskTrigger :: when: -Daily, -Weekly, -AtStartup, -Once with repetition
New-ScheduledTaskPrincipal :: as whom, and with which run level
New-ScheduledTaskSettingsSet :: time limit, retries, missed runs, overlaps
Get-ScheduledTaskInfo :: LastTaskResult (exit code) and NextRunTime
-NoProfile -NonInteractive -File :: arguments for unattended scripts
$PSScriptRoot :: the folder of the running script
exit 0 / exit 1 :: success / failure for the scheduler
$LASTEXITCODE :: exit code of the last native program
Start-Transcript :: record console output to a log file
Export-Clixml credential :: DPAPI: same user, same machine only
gMSA :: service account whose password Windows rotates
Idempotent :: same result however many times it runs
SupportsShouldProcess :: adds -WhatIf and -Confirm to your script

@@drill
Which automatic variable holds the folder the running script is in?
= $PSScriptRoot
= PSScriptRoot

@@drill
Which cmdlet shows a scheduled task's last result and next run time?
= Get-ScheduledTaskInfo

@@exercise Next run times for task triggers
Given a fixed "now" of Thursday 2026-11-05 14:20, compute the next run time of each trigger the way Task Scheduler would: **Daily** at a time (today if the time is still ahead, otherwise tomorrow), **Weekly** on a set of weekdays at a time (the first matching day from today on whose time is still ahead), and **Repeat**: every N minutes between a start and an end time each day (the next slot after now, or tomorrow's start if today's window is over). Print each trigger with its next run as `yyyy-MM-dd HH:mm ddd` in the invariant culture.
@@starter
$now = [datetime]::ParseExact('2026-11-05 14:20', 'yyyy-MM-dd HH:mm', [cultureinfo]::InvariantCulture)
$triggers = @(
    [pscustomobject]@{ Name = 'Nightly cleanup'; Kind = 'Daily'; At = '03:15' }
    [pscustomobject]@{ Name = 'Report';          Kind = 'Daily'; At = '17:00' }
    [pscustomobject]@{ Name = 'Patch window';    Kind = 'Weekly'; At = '02:00'; Days = @('Saturday', 'Sunday') }
    [pscustomobject]@{ Name = 'Mon/Thu backup';  Kind = 'Weekly'; At = '13:00'; Days = @('Monday', 'Thursday') }
    [pscustomobject]@{ Name = 'Queue check';     Kind = 'Repeat'; At = '08:00'; Until = '18:00'; Every = 15 }
    [pscustomobject]@{ Name = 'Early sync';      Kind = 'Repeat'; At = '06:00'; Until = '09:00'; Every = 30 }
)

function Get-NextRun($trigger, [datetime]$now) {
    throw 'TODO'
}

$inv = [cultureinfo]::InvariantCulture
foreach ($t in $triggers) {
    '{0,-16} {1}' -f $t.Name, (Get-NextRun $t $now).ToString('yyyy-MM-dd HH:mm ddd', $inv)
}
@@expected
Nightly cleanup  2026-11-06 03:15 Fri
Report           2026-11-05 17:00 Thu
Patch window     2026-11-07 02:00 Sat
Mon/Thu backup   2026-11-09 13:00 Mon
Queue check      2026-11-05 14:30 Thu
Early sync       2026-11-06 06:00 Fri
@@requires
AddDays(
DayOfWeek
AddMinutes(
@@hint
Turn `'03:15'` into a time with `[timespan]::Parse($t.At)` and add it to `$now.Date`. Daily: if that's not after `$now`, add a day. Weekly: loop `$i` from 0 to 7 over `$now.Date.AddDays($i)` and return the first one whose `DayOfWeek` is in `$t.Days` and whose time is after `$now`. Repeat: start at today's start time and keep `AddMinutes($t.Every)` while the slot isn't after now; if it passes the end time, return tomorrow's start.
@@solution
$now = [datetime]::ParseExact('2026-11-05 14:20', 'yyyy-MM-dd HH:mm', [cultureinfo]::InvariantCulture)
$triggers = @(
    [pscustomobject]@{ Name = 'Nightly cleanup'; Kind = 'Daily'; At = '03:15' }
    [pscustomobject]@{ Name = 'Report';          Kind = 'Daily'; At = '17:00' }
    [pscustomobject]@{ Name = 'Patch window';    Kind = 'Weekly'; At = '02:00'; Days = @('Saturday', 'Sunday') }
    [pscustomobject]@{ Name = 'Mon/Thu backup';  Kind = 'Weekly'; At = '13:00'; Days = @('Monday', 'Thursday') }
    [pscustomobject]@{ Name = 'Queue check';     Kind = 'Repeat'; At = '08:00'; Until = '18:00'; Every = 15 }
    [pscustomobject]@{ Name = 'Early sync';      Kind = 'Repeat'; At = '06:00'; Until = '09:00'; Every = 30 }
)

function Get-NextRun($trigger, [datetime]$now) {
    $at = [timespan]::Parse($trigger.At)
    switch ($trigger.Kind) {
        'Daily' {
            $next = $now.Date + $at
            if ($next -le $now) { $next = $next.AddDays(1) }
            return $next
        }
        'Weekly' {
            for ($i = 0; $i -le 7; $i++) {
                $candidate = $now.Date.AddDays($i) + $at
                if ($trigger.Days -contains $candidate.DayOfWeek.ToString() -and $candidate -gt $now) {
                    return $candidate
                }
            }
        }
        'Repeat' {
            $end = $now.Date + [timespan]::Parse($trigger.Until)
            $slot = $now.Date + $at
            while ($slot -le $now) { $slot = $slot.AddMinutes($trigger.Every) }
            if ($slot -gt $end) { $slot = $now.Date.AddDays(1) + $at }
            return $slot
        }
    }
}

$inv = [cultureinfo]::InvariantCulture
foreach ($t in $triggers) {
    '{0,-16} {1}' -f $t.Name, (Get-NextRun $t $now).ToString('yyyy-MM-dd HH:mm ddd', $inv)
}

@@exercise Test, then set
Write `Invoke-DesiredState` that compares a desired configuration with the current one and makes only the changes needed. Settings are a hashtable of name to value: for each desired setting print `ok` if it already matches, or `set` with the old and new value and update the current hashtable; for current settings marked in `$Remove`, print `remove` and delete them. With `-WhatIf`-style behaviour (a `$DryRun` switch), print the same plan prefixed with `would` but change nothing. Run it as a dry run, for real, and then again to show the second run changes nothing.
@@starter
$current = @{ 'Spooler.StartType' = 'Automatic'; 'RDP.Enabled' = 'True'; 'TLS1.0' = 'Enabled'; 'NTP.Server' = 'time.windows.com' }
$desired = [ordered]@{ 'Spooler.StartType' = 'Disabled'; 'RDP.Enabled' = 'True'; 'TLS1.0' = 'Disabled'; 'NTP.Server' = 'ntp.nict.jp'; 'SMB1.Installed' = 'False' }
$Remove = @('LegacyAgent.Path')
$current['LegacyAgent.Path'] = 'C:\Agent'

function Invoke-DesiredState([hashtable]$Current, $Desired, [string[]]$Remove, [switch]$DryRun) {
    throw 'TODO'
}

'--- dry run'
Invoke-DesiredState $current $desired $Remove -DryRun
'--- apply'
Invoke-DesiredState $current $desired $Remove
'--- apply again'
Invoke-DesiredState $current $desired $Remove
@@expected
--- dry run
would set Spooler.StartType : Automatic -> Disabled
ok      RDP.Enabled = True
would set TLS1.0 : Enabled -> Disabled
would set NTP.Server : time.windows.com -> ntp.nict.jp
would set SMB1.Installed : (missing) -> False
would remove LegacyAgent.Path
--- apply
set Spooler.StartType : Automatic -> Disabled
ok      RDP.Enabled = True
set TLS1.0 : Enabled -> Disabled
set NTP.Server : time.windows.com -> ntp.nict.jp
set SMB1.Installed : (missing) -> False
remove LegacyAgent.Path
--- apply again
ok      Spooler.StartType = Disabled
ok      RDP.Enabled = True
ok      TLS1.0 = Disabled
ok      NTP.Server = ntp.nict.jp
ok      SMB1.Installed = False
no changes needed
@@requires
ContainsKey(
-not $DryRun
.Remove(
@@hint
Loop over `$Desired.Keys`. Get the old value with `if ($Current.ContainsKey($k)) { $Current[$k] } else { '(missing)' }`. Use a `$prefix = if ($DryRun) { 'would ' } else { '' }`. Only assign `$Current[$k] = $Desired[$k]` and call `$Current.Remove($k)` when `-not $DryRun`. Count the changes and print `no changes needed` when there were none.
@@solution
$current = @{ 'Spooler.StartType' = 'Automatic'; 'RDP.Enabled' = 'True'; 'TLS1.0' = 'Enabled'; 'NTP.Server' = 'time.windows.com' }
$desired = [ordered]@{ 'Spooler.StartType' = 'Disabled'; 'RDP.Enabled' = 'True'; 'TLS1.0' = 'Disabled'; 'NTP.Server' = 'ntp.nict.jp'; 'SMB1.Installed' = 'False' }
$Remove = @('LegacyAgent.Path')
$current['LegacyAgent.Path'] = 'C:\Agent'

function Invoke-DesiredState([hashtable]$Current, $Desired, [string[]]$Remove, [switch]$DryRun) {
    $prefix = if ($DryRun) { 'would ' } else { '' }
    $changes = 0
    foreach ($k in $Desired.Keys) {
        $old = if ($Current.ContainsKey($k)) { $Current[$k] } else { '(missing)' }
        if ($old -eq $Desired[$k]) {
            "ok      $k = $old"
            continue
        }
        "${prefix}set $k : $old -> $($Desired[$k])"
        $changes++
        if (-not $DryRun) { $Current[$k] = $Desired[$k] }
    }
    foreach ($k in $Remove) {
        if ($Current.ContainsKey($k)) {
            "${prefix}remove $k"
            $changes++
            if (-not $DryRun) { $Current.Remove($k) }
        }
    }
    if ($changes -eq 0) { 'no changes needed' }
}

'--- dry run'
Invoke-DesiredState $current $desired $Remove -DryRun
'--- apply'
Invoke-DesiredState $current $desired $Remove
'--- apply again'
Invoke-DesiredState $current $desired $Remove

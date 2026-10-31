@@lesson q1-health-monitor System Health Monitor
@@text
Every IT team wants the same morning email: *is anything broken or about to break?* In this build you'll write **Get-HealthReport.ps1**, a script that checks a Windows computer and produces a colour-coded **HTML report**:

- **disks** below a warning or critical free-space threshold,
- **services** set to start automatically that aren't running,
- **uptime** too long (Windows patches need reboots, and a server up for 120 days is missing updates),
- and an **exit code** monitoring tools understand: `0` all good, `1` warnings, `2` critical.

The design is the important lesson here. The checks are **pure functions** that get data as parameters and return *finding* objects. Only the main script talks to the real system (`Get-CimInstance`, `Get-Service`). That's why the checks can be tested with fake data, the same trick as the Python builds.

```text
HealthChecks.ps1     the check functions (you write these)
tests.ps1            tests with fake data: run it until everything passes
Get-HealthReport.ps1 collects real data, runs the checks, writes report.html
```

A **finding** is a `[pscustomobject]` with four properties:

| Property | Example |
|---|---|
| `Check` | `Disk`, `Service` or `Uptime` |
| `Target` | `C:`, `Spooler`, the computer name |
| `Level` | `OK`, `Warning` or `Critical` |
| `Detail` | `8.2% free (39.1 GB)` |

@@project health_monitor System Health Monitor
Write the four functions in `HealthChecks.ps1` until **▶ tests.ps1** prints `All tests passed`. Then run **▶ Get-HealthReport.ps1** for a report on your own PC.
@@file HealthChecks.ps1
# Health checks: pure functions that turn data into findings. No system calls in here,
# so tests.ps1 can feed them fake data.

function New-Finding([string]$Check, [string]$Target, [string]$Level, [string]$Detail) {
    [pscustomobject]@{ Check = $Check; Target = $Target; Level = $Level; Detail = $Detail }
}

function Get-DiskFinding {
    # One finding per disk. $Disks are objects with DeviceID, Size and FreeSpace (bytes),
    # like Get-CimInstance Win32_LogicalDisk returns.
    # Level: Critical if free% < $CriticalPercent, Warning if free% < $WarnPercent, else OK.
    # Detail: "<free% with 1 decimal>% free (<free GB with 1 decimal> GB)", e.g. "8.2% free (39.1 GB)".
    param($Disks, [double]$WarnPercent = 15, [double]$CriticalPercent = 5)
    throw "TODO: not done yet"
}

function Get-ServiceFinding {
    # A Critical finding for each service whose StartType is Automatic but Status isn't Running.
    # Detail: "Automatic service is <Status>". Healthy services produce no finding.
    param($Services)
    throw "TODO: not done yet"
}

function Get-UptimeFinding {
    # One finding for the computer: Warning if it has been up more than $MaxDays days, else OK.
    # Detail: "Up <whole days> days".
    param([string]$ComputerName, [datetime]$LastBoot, [datetime]$Now, [int]$MaxDays = 30)
    throw "TODO: not done yet"
}

function Get-ExitCode {
    # 2 if any finding is Critical, 1 if any is Warning, 0 otherwise.
    param($Findings)
    throw "TODO: not done yet"
}

function ConvertTo-HealthHtml {
    # A complete HTML page: a table of findings, worst first, each row coloured by level.
    param($Findings, [string]$ComputerName, [datetime]$Time)
    $order = @{ Critical = 0; Warning = 1; OK = 2 }
    $colour = @{ Critical = '#f8d7da'; Warning = '#fff3cd'; OK = '#d1e7dd' }
    $rows = foreach ($f in ($Findings | Sort-Object { $order[$_.Level] }, Check, Target)) {
        $detail = [System.Net.WebUtility]::HtmlEncode($f.Detail)
        "<tr style='background:$($colour[$f.Level])'><td>$($f.Level)</td><td>$($f.Check)</td><td>$($f.Target)</td><td>$detail</td></tr>"
    }
    @"
<!DOCTYPE html>
<html><head><meta charset="utf-8"><title>Health: $ComputerName</title>
<style>body{font-family:Segoe UI,sans-serif;margin:2em}table{border-collapse:collapse}td,th{padding:6px 12px;border:1px solid #ccc;text-align:left}</style>
</head><body>
<h1>Health report: $ComputerName</h1>
<p>Generated $($Time.ToString('yyyy-MM-dd HH:mm'))</p>
<table><tr><th>Level</th><th>Check</th><th>Target</th><th>Detail</th></tr>
$($rows -join "`n")
</table></body></html>
"@
}
@@file tests.ps1
# Run: powershell -ExecutionPolicy Bypass -File tests.ps1
. "$PSScriptRoot\HealthChecks.ps1"
$script:failed = 0

function Test-That([string]$Name, [scriptblock]$Body) {
    try {
        $result = & $Body
        if ($result -eq $true) { Write-Host "  ok    $Name" -ForegroundColor Green }
        else { Write-Host "  FAIL  $Name" -ForegroundColor Red; $script:failed++ }
    }
    catch {
        Write-Host "  FAIL  $Name : $($_.Exception.Message)" -ForegroundColor Red
        $script:failed++
    }
}

$disks = @(
    [pscustomobject]@{ DeviceID = 'C:'; Size = 476GB; FreeSpace = 39.1GB }
    [pscustomobject]@{ DeviceID = 'D:'; Size = 100GB; FreeSpace = 4GB }
    [pscustomobject]@{ DeviceID = 'E:'; Size = 200GB; FreeSpace = 150GB }
)
Write-Host 'Disks'
$d = @(Get-DiskFinding -Disks $disks)
Test-That 'one finding per disk' { $d.Count -eq 3 }
Test-That 'C: is a Warning' { $d[0].Level -eq 'Warning' -and $d[0].Target -eq 'C:' -and $d[0].Check -eq 'Disk' }
Test-That 'C: detail' { $d[0].Detail -eq '8.2% free (39.1 GB)' }
Test-That 'D: is Critical' { $d[1].Level -eq 'Critical' }
Test-That 'E: is OK' { $d[2].Level -eq 'OK' }
Test-That 'custom thresholds' { (Get-DiskFinding -Disks $disks -WarnPercent 80 -CriticalPercent 9)[0].Level -eq 'Critical' }

$services = @(
    [pscustomobject]@{ Name = 'Spooler'; Status = 'Stopped'; StartType = 'Automatic' }
    [pscustomobject]@{ Name = 'W32Time'; Status = 'Running'; StartType = 'Automatic' }
    [pscustomobject]@{ Name = 'Fax'; Status = 'Stopped'; StartType = 'Manual' }
)
Write-Host 'Services'
$s = @(Get-ServiceFinding -Services $services)
Test-That 'only the stopped automatic service' { $s.Count -eq 1 -and $s[0].Target -eq 'Spooler' }
Test-That 'it is Critical with a detail' { $s[0].Level -eq 'Critical' -and $s[0].Detail -eq 'Automatic service is Stopped' }

Write-Host 'Uptime'
$now = [datetime]'2026-10-31 09:00'
Test-That 'up 45 days is a Warning' {
    $u = Get-UptimeFinding -ComputerName 'PC1' -LastBoot $now.AddDays(-45.5) -Now $now
    $u.Level -eq 'Warning' -and $u.Detail -eq 'Up 45 days' -and $u.Target -eq 'PC1'
}
Test-That 'up 3 days is OK' { (Get-UptimeFinding -ComputerName 'PC1' -LastBoot $now.AddDays(-3) -Now $now).Level -eq 'OK' }

Write-Host 'Exit code'
Test-That 'critical gives 2' { (Get-ExitCode -Findings ($d + $s)) -eq 2 }
Test-That 'warning gives 1' { (Get-ExitCode -Findings @($d[0], $d[2])) -eq 1 }
Test-That 'all OK gives 0' { (Get-ExitCode -Findings @($d[2])) -eq 0 }

Write-Host 'Report'
Test-That 'HTML lists the worst first' {
    $html = ConvertTo-HealthHtml -Findings ($d + $s) -ComputerName 'PC1' -Time $now
    $html.IndexOf('Critical') -lt $html.IndexOf('Warning') -and $html -match 'Spooler'
}

if ($script:failed -gt 0) { Write-Host "$script:failed test(s) failed" -ForegroundColor Red; exit 1 }
Write-Host 'All tests passed' -ForegroundColor Green
@@file Get-HealthReport.ps1
<#
.SYNOPSIS
Checks disks, services and uptime, and writes a colour-coded HTML report.
.EXAMPLE
.\Get-HealthReport.ps1 -WarnPercent 20 -NoOpen
#>
[CmdletBinding()]
param(
    [double]$WarnPercent = 15,
    [double]$CriticalPercent = 5,
    [int]$MaxUptimeDays = 30,
    [string]$OutFile = (Join-Path $PSScriptRoot 'report.html'),
    [switch]$NoOpen
)
$ErrorActionPreference = 'Stop'
. "$PSScriptRoot\HealthChecks.ps1"

Write-Verbose 'Collecting data'
$os = Get-CimInstance Win32_OperatingSystem
$disks = Get-CimInstance Win32_LogicalDisk -Filter 'DriveType=3'
$services = Get-Service | Select-Object Name, @{ n = 'Status'; e = { "$($_.Status)" } }, @{ n = 'StartType'; e = { "$($_.StartType)" } }

$findings = @(
    Get-DiskFinding -Disks $disks -WarnPercent $WarnPercent -CriticalPercent $CriticalPercent
    Get-ServiceFinding -Services $services
    Get-UptimeFinding -ComputerName $env:COMPUTERNAME -LastBoot $os.LastBootUpTime -Now (Get-Date) -MaxDays $MaxUptimeDays
)

ConvertTo-HealthHtml -Findings $findings -ComputerName $env:COMPUTERNAME -Time (Get-Date) | Set-Content -Path $OutFile -Encoding UTF8
$findings | Where-Object Level -ne 'OK' | Format-Table Level, Check, Target, Detail -AutoSize
Write-Host "Report written to $OutFile"
if (-not $NoOpen) { Invoke-Item $OutFile }
exit (Get-ExitCode -Findings $findings)
@@text
> Some Windows services are set to Automatic but stop by design once their work is done (like `sppsvc` or `gupdate`), so your first report may show a few. Real monitoring tools keep an **exclusion list**: add a `-ExcludeService` parameter as an extension.

@@project health_monitor_finished System Health Monitor: the finished version
The complete project. Make it yours: schedule it every morning with `Register-ScheduledTask` (lesson *Scripts, Modules and Scheduled Tasks*), email the report with `Send-MailMessage` or Microsoft Graph, check a list of servers with `-ComputerName` and `Get-CimInstance -ComputerName`, or add checks for pending reboots (`Test-Path 'HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\WindowsUpdate\Auto Update\RebootRequired'`).
@@file HealthChecks.ps1
# Health checks: pure functions that turn data into findings. No system calls in here,
# so tests.ps1 can feed them fake data.

function New-Finding([string]$Check, [string]$Target, [string]$Level, [string]$Detail) {
    [pscustomobject]@{ Check = $Check; Target = $Target; Level = $Level; Detail = $Detail }
}

function Get-DiskFinding {
    # One finding per disk. $Disks are objects with DeviceID, Size and FreeSpace (bytes),
    # like Get-CimInstance Win32_LogicalDisk returns.
    # Level: Critical if free% < $CriticalPercent, Warning if free% < $WarnPercent, else OK.
    # Detail: "<free% with 1 decimal>% free (<free GB with 1 decimal> GB)", e.g. "8.2% free (39.1 GB)".
    param($Disks, [double]$WarnPercent = 15, [double]$CriticalPercent = 5)
    foreach ($disk in $Disks) {
        $pct = $disk.FreeSpace / $disk.Size * 100
        $level = if ($pct -lt $CriticalPercent) { 'Critical' } elseif ($pct -lt $WarnPercent) { 'Warning' } else { 'OK' }
        $detail = '{0:N1}% free ({1:N1} GB)' -f $pct, ($disk.FreeSpace / 1GB)
        New-Finding 'Disk' $disk.DeviceID $level $detail
    }
}

function Get-ServiceFinding {
    # A Critical finding for each service whose StartType is Automatic but Status isn't Running.
    # Detail: "Automatic service is <Status>". Healthy services produce no finding.
    param($Services)
    foreach ($svc in $Services) {
        if ($svc.StartType -eq 'Automatic' -and $svc.Status -ne 'Running') {
            New-Finding 'Service' $svc.Name 'Critical' "Automatic service is $($svc.Status)"
        }
    }
}

function Get-UptimeFinding {
    # One finding for the computer: Warning if it has been up more than $MaxDays days, else OK.
    # Detail: "Up <whole days> days".
    param([string]$ComputerName, [datetime]$LastBoot, [datetime]$Now, [int]$MaxDays = 30)
    $days = ($Now - $LastBoot).Days
    $level = if ($days -gt $MaxDays) { 'Warning' } else { 'OK' }
    New-Finding 'Uptime' $ComputerName $level "Up $days days"
}

function Get-ExitCode {
    # 2 if any finding is Critical, 1 if any is Warning, 0 otherwise.
    param($Findings)
    $levels = @($Findings | ForEach-Object { $_.Level })
    if ($levels -contains 'Critical') { return 2 }
    if ($levels -contains 'Warning') { return 1 }
    return 0
}

function ConvertTo-HealthHtml {
    # A complete HTML page: a table of findings, worst first, each row coloured by level.
    param($Findings, [string]$ComputerName, [datetime]$Time)
    $order = @{ Critical = 0; Warning = 1; OK = 2 }
    $colour = @{ Critical = '#f8d7da'; Warning = '#fff3cd'; OK = '#d1e7dd' }
    $rows = foreach ($f in ($Findings | Sort-Object { $order[$_.Level] }, Check, Target)) {
        $detail = [System.Net.WebUtility]::HtmlEncode($f.Detail)
        "<tr style='background:$($colour[$f.Level])'><td>$($f.Level)</td><td>$($f.Check)</td><td>$($f.Target)</td><td>$detail</td></tr>"
    }
    @"
<!DOCTYPE html>
<html><head><meta charset="utf-8"><title>Health: $ComputerName</title>
<style>body{font-family:Segoe UI,sans-serif;margin:2em}table{border-collapse:collapse}td,th{padding:6px 12px;border:1px solid #ccc;text-align:left}</style>
</head><body>
<h1>Health report: $ComputerName</h1>
<p>Generated $($Time.ToString('yyyy-MM-dd HH:mm'))</p>
<table><tr><th>Level</th><th>Check</th><th>Target</th><th>Detail</th></tr>
$($rows -join "`n")
</table></body></html>
"@
}
@@file tests.ps1
# Run: powershell -ExecutionPolicy Bypass -File tests.ps1
. "$PSScriptRoot\HealthChecks.ps1"
$script:failed = 0

function Test-That([string]$Name, [scriptblock]$Body) {
    try {
        $result = & $Body
        if ($result -eq $true) { Write-Host "  ok    $Name" -ForegroundColor Green }
        else { Write-Host "  FAIL  $Name" -ForegroundColor Red; $script:failed++ }
    }
    catch {
        Write-Host "  FAIL  $Name : $($_.Exception.Message)" -ForegroundColor Red
        $script:failed++
    }
}

$disks = @(
    [pscustomobject]@{ DeviceID = 'C:'; Size = 476GB; FreeSpace = 39.1GB }
    [pscustomobject]@{ DeviceID = 'D:'; Size = 100GB; FreeSpace = 4GB }
    [pscustomobject]@{ DeviceID = 'E:'; Size = 200GB; FreeSpace = 150GB }
)
Write-Host 'Disks'
$d = @(Get-DiskFinding -Disks $disks)
Test-That 'one finding per disk' { $d.Count -eq 3 }
Test-That 'C: is a Warning' { $d[0].Level -eq 'Warning' -and $d[0].Target -eq 'C:' -and $d[0].Check -eq 'Disk' }
Test-That 'C: detail' { $d[0].Detail -eq '8.2% free (39.1 GB)' }
Test-That 'D: is Critical' { $d[1].Level -eq 'Critical' }
Test-That 'E: is OK' { $d[2].Level -eq 'OK' }
Test-That 'custom thresholds' { (Get-DiskFinding -Disks $disks -WarnPercent 80 -CriticalPercent 9)[0].Level -eq 'Critical' }

$services = @(
    [pscustomobject]@{ Name = 'Spooler'; Status = 'Stopped'; StartType = 'Automatic' }
    [pscustomobject]@{ Name = 'W32Time'; Status = 'Running'; StartType = 'Automatic' }
    [pscustomobject]@{ Name = 'Fax'; Status = 'Stopped'; StartType = 'Manual' }
)
Write-Host 'Services'
$s = @(Get-ServiceFinding -Services $services)
Test-That 'only the stopped automatic service' { $s.Count -eq 1 -and $s[0].Target -eq 'Spooler' }
Test-That 'it is Critical with a detail' { $s[0].Level -eq 'Critical' -and $s[0].Detail -eq 'Automatic service is Stopped' }

Write-Host 'Uptime'
$now = [datetime]'2026-10-31 09:00'
Test-That 'up 45 days is a Warning' {
    $u = Get-UptimeFinding -ComputerName 'PC1' -LastBoot $now.AddDays(-45.5) -Now $now
    $u.Level -eq 'Warning' -and $u.Detail -eq 'Up 45 days' -and $u.Target -eq 'PC1'
}
Test-That 'up 3 days is OK' { (Get-UptimeFinding -ComputerName 'PC1' -LastBoot $now.AddDays(-3) -Now $now).Level -eq 'OK' }

Write-Host 'Exit code'
Test-That 'critical gives 2' { (Get-ExitCode -Findings ($d + $s)) -eq 2 }
Test-That 'warning gives 1' { (Get-ExitCode -Findings @($d[0], $d[2])) -eq 1 }
Test-That 'all OK gives 0' { (Get-ExitCode -Findings @($d[2])) -eq 0 }

Write-Host 'Report'
Test-That 'HTML lists the worst first' {
    $html = ConvertTo-HealthHtml -Findings ($d + $s) -ComputerName 'PC1' -Time $now
    $html.IndexOf('Critical') -lt $html.IndexOf('Warning') -and $html -match 'Spooler'
}

if ($script:failed -gt 0) { Write-Host "$script:failed test(s) failed" -ForegroundColor Red; exit 1 }
Write-Host 'All tests passed' -ForegroundColor Green
@@file Get-HealthReport.ps1
<#
.SYNOPSIS
Checks disks, services and uptime, and writes a colour-coded HTML report.
.EXAMPLE
.\Get-HealthReport.ps1 -WarnPercent 20 -NoOpen
#>
[CmdletBinding()]
param(
    [double]$WarnPercent = 15,
    [double]$CriticalPercent = 5,
    [int]$MaxUptimeDays = 30,
    [string]$OutFile = (Join-Path $PSScriptRoot 'report.html'),
    [switch]$NoOpen
)
$ErrorActionPreference = 'Stop'
. "$PSScriptRoot\HealthChecks.ps1"

Write-Verbose 'Collecting data'
$os = Get-CimInstance Win32_OperatingSystem
$disks = Get-CimInstance Win32_LogicalDisk -Filter 'DriveType=3'
$services = Get-Service | Select-Object Name, @{ n = 'Status'; e = { "$($_.Status)" } }, @{ n = 'StartType'; e = { "$($_.StartType)" } }

$findings = @(
    Get-DiskFinding -Disks $disks -WarnPercent $WarnPercent -CriticalPercent $CriticalPercent
    Get-ServiceFinding -Services $services
    Get-UptimeFinding -ComputerName $env:COMPUTERNAME -LastBoot $os.LastBootUpTime -Now (Get-Date) -MaxDays $MaxUptimeDays
)

ConvertTo-HealthHtml -Findings $findings -ComputerName $env:COMPUTERNAME -Time (Get-Date) | Set-Content -Path $OutFile -Encoding UTF8
$findings | Where-Object Level -ne 'OK' | Format-Table Level, Check, Target, Detail -AutoSize
Write-Host "Report written to $OutFile"
if (-not $NoOpen) { Invoke-Item $OutFile }
exit (Get-ExitCode -Findings $findings)
@@text
🎉 This is a real portfolio piece: a tested, scheduled monitoring script with an HTML report. In an interview, explain *why* the checks are separate from the data collection. That one design decision is what makes it testable.

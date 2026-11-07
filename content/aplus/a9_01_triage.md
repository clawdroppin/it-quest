@@lesson a9-triage Help Desk Triage Tool
@@text
When a call comes in, a good technician does three things fast: **prioritize** it, think of the **likely causes**, and write a **clear ticket note**. You'll build a PowerShell tool that does all three, backed by a knowledge base of A+ troubleshooting know-how you can keep extending:

```powershell,norun
.\Start-Triage.ps1 -Symptom "printer prints ghost images and smudges" -Impact Department -Urgency Medium
```

```text
Priority: P3  (impact Department, urgency Medium)

Likely causes
  [3] Laser printer: ghosting or smearing
      Causes: Worn imaging drum; Failing fuser (toner smears off); Wrong paper type setting
      Try:    Print a test page; replace the drum/toner cartridge; replace the fuser if toner smears
...
Quick checks on this PC
  IPv4 192.168.1.25, gateway 192.168.1.1: gateway reachable
  DNS lookup of www.microsoft.com: OK
Ticket note saved to tickets\20261031-1415.txt
```

## The pieces

| Function | Does |
|---|---|
| `Get-TicketPriority` | impact × urgency → **P1**–**P4** |
| `Find-LikelyCause` | scores knowledge-base entries by how many of their keywords appear in the symptom |
| `Format-TicketNote` | the standard note: Priority, Issue, Checked, Action, Result |

The priority matrix: give impact (Single = 1, Department = 2, Company = 3) and urgency (Low = 1, Medium = 2, High = 3) points and add them: **6 → P1, 5 → P2, 4 → P3, less → P4**.

The knowledge base, `kb.json`, is **data, not code**: anyone on the team can add an entry without touching the script. That separation is a habit worth keeping.

@@project triage Help Desk Triage
1. Write the three functions in `HelpDesk.ps1` until `tests.ps1` passes: `powershell -ExecutionPolicy Bypass -File tests.ps1`
2. Try `Start-Triage.ps1` with your own symptoms.
3. Add knowledge-base entries for problems you meet in your home lab or at work.
@@file HelpDesk.ps1
# Help desk triage functions. Dot-source this file:  . .\HelpDesk.ps1

function Get-TicketPriority {
    <#
    .SYNOPSIS
    P1-P4 from impact and urgency. Points: Single 1, Department 2, Company 3; Low 1, Medium 2, High 3.
    Total 6 = P1, 5 = P2, 4 = P3, otherwise P4.
    #>
    param(
        [Parameter(Mandatory)][ValidateSet('Single', 'Department', 'Company')][string]$Impact,
        [Parameter(Mandatory)][ValidateSet('Low', 'Medium', 'High')][string]$Urgency
    )
    throw 'TODO: not done yet'
}

function Find-LikelyCause {
    <#
    .SYNOPSIS
    Knowledge-base entries matching a symptom, best first.
    .DESCRIPTION
    An entry's Score is how many of its Keywords appear in the symptom (ignoring case).
    Return entries with Score > 0 as objects with Problem, Score, Causes and Steps,
    sorted by Score (highest first), ties in knowledge-base order, at most -Top of them.
    #>
    param(
        [Parameter(Mandatory)][string]$Symptom,
        [Parameter(Mandatory)][object[]]$KnowledgeBase,
        [int]$Top = 3
    )
    throw 'TODO: not done yet'
}

function Format-TicketNote {
    <#
    .SYNOPSIS
    The standard ticket note, one field per line:
    Priority: P2
    Issue: ...
    Checked: ...
    Action: ...
    Result: ...
    Empty fields are written as "-". Lines are joined with "`n".
    #>
    param(
        [string]$Priority,
        [string]$Issue,
        [string]$Checked,
        [string]$Action,
        [string]$Result
    )
    throw 'TODO: not done yet'
}
@@file kb.json
[
  {
    "Problem": "No power at all",
    "Keywords": ["no power", "won't turn on", "wont turn on", "dead", "no lights"],
    "Causes": ["Outlet or power strip off", "Loose or faulty power cable", "Failed power supply (PSU)", "Front power button cable"],
    "Steps": ["Try a known-good outlet and cable", "Check the PSU switch on the back", "Test the PSU with a tester"]
  },
  {
    "Problem": "Beeps or no display at startup",
    "Keywords": ["beep", "no display", "black screen", "no signal"],
    "Causes": ["RAM not seated or failed", "Graphics card or cable", "Monitor on the wrong input"],
    "Steps": ["Look up the beep code", "Reseat the RAM one stick at a time", "Check the monitor input and cable"]
  },
  {
    "Problem": "Slow or freezing PC",
    "Keywords": ["slow", "freez", "lag", "hang", "takes forever"],
    "Causes": ["RAM full and swapping", "Disk at 100% or nearly full", "Too many startup apps", "Malware"],
    "Steps": ["Check Task Manager for CPU/RAM/disk hogs", "Disable startup apps", "Free disk space", "Run a malware scan"]
  },
  {
    "Problem": "Blue screen / random crashes",
    "Keywords": ["blue screen", "bsod", "stop code", "crash", "reboots"],
    "Causes": ["Bad driver or update", "Faulty RAM", "Overheating", "Corrupt system files"],
    "Steps": ["Note the stop code", "Roll back recent drivers/updates", "Run Windows Memory Diagnostic", "Run DISM then sfc /scannow"]
  },
  {
    "Problem": "No network or 169.254 address",
    "Keywords": ["no internet", "169.254", "limited connectivity", "no network", "can't connect", "cannot connect"],
    "Causes": ["Cable unplugged or Wi-Fi off", "DHCP server unreachable", "Wrong VLAN or port disabled"],
    "Steps": ["Check link lights / Wi-Fi", "ipconfig /release then /renew", "Ping the gateway", "Check the switch port"]
  },
  {
    "Problem": "DNS: sites won't load by name",
    "Keywords": ["website", "websites", "dns", "can ping", "server not found"],
    "Causes": ["DNS server unreachable or wrong", "Stale DNS cache", "Hosts file changed"],
    "Steps": ["nslookup the name", "ipconfig /flushdns", "Check DNS servers in ipconfig /all"]
  },
  {
    "Problem": "Laser printer: ghosting or smearing",
    "Keywords": ["ghost", "smudge", "smear", "faded", "toner"],
    "Causes": ["Worn imaging drum", "Failing fuser (toner smears off)", "Wrong paper type setting"],
    "Steps": ["Print a test page", "Replace the drum/toner cartridge", "Replace the fuser if toner smears"]
  },
  {
    "Problem": "Paper jams",
    "Keywords": ["jam", "jams", "creased", "multiple sheets"],
    "Causes": ["Worn pickup rollers", "Damp or wrong paper", "Torn paper inside"],
    "Steps": ["Remove all paper and scraps", "Clean or replace the pickup rollers", "Use fresh, dry paper"]
  },
  {
    "Problem": "Print jobs stuck in the queue",
    "Keywords": ["print queue", "stuck", "spooler", "won't print", "wont print"],
    "Causes": ["Print Spooler service hung", "Corrupt print job", "Printer offline or IP changed"],
    "Steps": ["Clear the queue and restart the Print Spooler service", "Check the printer's IP and port", "Reinstall the driver"]
  },
  {
    "Problem": "Overheating",
    "Keywords": ["hot", "overheat", "fan loud", "loud fan", "shuts down", "shuts off"],
    "Causes": ["Dust in vents and heatsink", "Failed fan", "Dried thermal paste", "Blocked airflow"],
    "Steps": ["Clean with compressed air", "Check that all fans spin", "Replace thermal paste"]
  },
  {
    "Problem": "Laptop battery or charging",
    "Keywords": ["battery", "won't charge", "wont charge", "not charging", "swollen", "swelling"],
    "Causes": ["Wrong or failed charger", "Worn battery", "Swollen battery (fire risk!)"],
    "Steps": ["Check the charger wattage and port", "Check battery health (powercfg /batteryreport)", "Stop using a swollen battery and replace it"]
  },
  {
    "Problem": "Account locked or can't log in",
    "Keywords": ["locked", "password", "can't log", "cannot log", "login", "log in", "sign in"],
    "Causes": ["Too many failed attempts", "Expired password", "Caps Lock / keyboard layout", "Clock out of sync with the domain"],
    "Steps": ["Verify the caller's identity first", "Unlock / reset in Active Directory", "Check the PC's time"]
  },
  {
    "Problem": "Failing hard drive",
    "Keywords": ["clicking", "grinding", "smart", "bad sector", "disk error"],
    "Causes": ["Mechanical HDD failure", "Bad sectors"],
    "Steps": ["Back up the data immediately", "Check S.M.A.R.T. status", "Replace the drive"]
  },
  {
    "Problem": "Malware symptoms",
    "Keywords": ["pop-up", "popup", "redirect", "toolbar", "ransom", "encrypted files", "virus"],
    "Causes": ["Adware or browser hijacker", "Trojan", "Ransomware"],
    "Steps": ["Quarantine: disconnect from the network", "Follow the malware removal process", "Report ransomware to security immediately"]
  }
]
@@file tests.ps1
# Run: powershell -ExecutionPolicy Bypass -File tests.ps1
. "$PSScriptRoot\HelpDesk.ps1"
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

Write-Host 'Priority'
Test-That 'company + high = P1' { (Get-TicketPriority -Impact Company -Urgency High) -eq 'P1' }
Test-That 'company + medium = P2' { (Get-TicketPriority -Impact Company -Urgency Medium) -eq 'P2' }
Test-That 'department + high = P2' { (Get-TicketPriority -Impact Department -Urgency High) -eq 'P2' }
Test-That 'single + high = P3' { (Get-TicketPriority -Impact Single -Urgency High) -eq 'P3' }
Test-That 'department + low = P4' { (Get-TicketPriority -Impact Department -Urgency Low) -eq 'P4' }
Test-That 'single + low = P4' { (Get-TicketPriority -Impact Single -Urgency Low) -eq 'P4' }

Write-Host 'Likely causes'
$kb = Get-Content -Raw "$PSScriptRoot\kb.json" | ConvertFrom-Json
$r = @(Find-LikelyCause -Symptom 'Printer prints GHOST images and the toner smudges' -KnowledgeBase $kb)
Test-That 'best match first' { $r[0].Problem -eq 'Laser printer: ghosting or smearing' }
Test-That 'score counts keywords' { $r[0].Score -eq 3 }
Test-That 'result has causes and steps' { $r[0].Causes.Count -eq 3 -and $r[0].Steps.Count -eq 3 }
$slow = @(Find-LikelyCause -Symptom 'my laptop is so slow and keeps freezing, fan loud' -KnowledgeBase $kb)
Test-That 'slow beats overheating' { $slow[0].Problem -eq 'Slow or freezing PC' -and $slow[0].Score -eq 2 }
Test-That 'overheating second' { $slow[1].Problem -eq 'Overheating' -and $slow[1].Score -eq 1 }
$tie = @(Find-LikelyCause -Symptom 'dead and hot' -KnowledgeBase $kb)
Test-That 'ties keep knowledge-base order' { $tie[0].Problem -eq 'No power at all' -and $tie[1].Problem -eq 'Overheating' }
Test-That 'top limits results' { @(Find-LikelyCause -Symptom 'slow crash hot battery jam' -KnowledgeBase $kb -Top 2).Count -eq 2 }
Test-That 'no match, no results' { @(Find-LikelyCause -Symptom 'the coffee machine is sad' -KnowledgeBase $kb).Count -eq 0 }

Write-Host 'Ticket note'
$note = Format-TicketNote -Priority P3 -Issue 'Cannot print' -Checked 'Queue stuck' -Action 'Restarted spooler' -Result ''
$lines = $note -split "`n"
Test-That 'five lines' { $lines.Count -eq 5 }
Test-That 'priority line' { $lines[0] -eq 'Priority: P3' }
Test-That 'action line' { $lines[3] -eq 'Action: Restarted spooler' }
Test-That 'empty field is a dash' { $lines[4] -eq 'Result: -' }

if ($script:failed) { Write-Host "$script:failed test(s) failed" -ForegroundColor Red; exit 1 }
Write-Host 'All tests passed' -ForegroundColor Green
@@file Start-Triage.ps1
<#
.SYNOPSIS
Triage a help desk call: priority, likely causes, quick checks, and a saved ticket note.
.EXAMPLE
.\Start-Triage.ps1 -Symptom "outlook can't connect, websites won't load" -Impact Single -Urgency High
#>
param(
    [string]$Symptom,
    [ValidateSet('Single', 'Department', 'Company')][string]$Impact,
    [ValidateSet('Low', 'Medium', 'High')][string]$Urgency,
    [switch]$NoChecks
)

. "$PSScriptRoot\HelpDesk.ps1"

if (-not $Symptom) { $Symptom = Read-Host 'Describe the problem' }
if (-not $Impact) { $Impact = Read-Host 'Impact (Single, Department, Company)' }
if (-not $Urgency) { $Urgency = Read-Host 'Urgency (Low, Medium, High)' }

$priority = Get-TicketPriority -Impact $Impact -Urgency $Urgency
Write-Host ""
Write-Host "Priority: $priority  (impact $Impact, urgency $Urgency)" -ForegroundColor Cyan

$kb = Get-Content -Raw "$PSScriptRoot\kb.json" | ConvertFrom-Json
$causes = @(Find-LikelyCause -Symptom $Symptom -KnowledgeBase $kb)
Write-Host ""
Write-Host 'Likely causes'
if (-not $causes) { Write-Host '  Nothing in the knowledge base matches. Ask more questions, and add an entry when you solve it!' }
foreach ($c in $causes) {
    Write-Host ("  [{0}] {1}" -f $c.Score, $c.Problem) -ForegroundColor Yellow
    Write-Host ("      Causes: {0}" -f ($c.Causes -join '; '))
    Write-Host ("      Try:    {0}" -f ($c.Steps -join '; '))
}

$checked = @()
if (-not $NoChecks) {
    Write-Host ""
    Write-Host 'Quick checks on this PC'
    $ip = Get-NetIPConfiguration -ErrorAction SilentlyContinue |
        Where-Object { $_.IPv4DefaultGateway } | Select-Object -First 1
    if ($ip) {
        $gw = $ip.IPv4DefaultGateway.NextHop
        $ok = Test-Connection -ComputerName $gw -Count 1 -Quiet
        $line = "IPv4 $($ip.IPv4Address.IPAddress), gateway ${gw}: " + $(if ($ok) { 'gateway reachable' } else { 'gateway NOT reachable' })
    }
    else { $line = 'No network with a default gateway' }
    Write-Host "  $line"
    $checked += $line
    try {
        Resolve-DnsName www.microsoft.com -ErrorAction Stop | Out-Null
        $line = 'DNS lookup of www.microsoft.com: OK'
    }
    catch { $line = 'DNS lookup of www.microsoft.com: FAILED' }
    Write-Host "  $line"
    $checked += $line
}

$note = Format-TicketNote -Priority $priority -Issue $Symptom -Checked ($checked -join '; ') -Action '' -Result ''
$dir = Join-Path $PSScriptRoot 'tickets'
New-Item -ItemType Directory -Force $dir | Out-Null
$file = Join-Path $dir ((Get-Date -Format 'yyyyMMdd-HHmm') + '.txt')
Set-Content -Path $file -Value $note -Encoding UTF8
Write-Host ""
Write-Host "Ticket note saved to $file (fill in Action and Result when you're done)"
@@text
> Tips: a hashtable maps names to points: `$points = @{ Single = 1; Department = 2; Company = 3 }`. In `Find-LikelyCause`, loop with an index (`for ($i = 0; ...)`), count keywords with `$Symptom.ToLower().Contains($k.ToLower())`, and sort with `Sort-Object @{Expression = 'Score'; Descending = $true}, Index`, because PowerShell 5.1's sort isn't guaranteed to keep ties in order.

@@project triage_finished Help Desk Triage: the finished version
The complete tool. Next steps: make `Start-Triage.ps1` create a ticket in your ITSM tool through its REST API (`Invoke-RestMethod`), or add a `-Remote` switch that runs the checks on the user's PC with `Invoke-Command`.
@@file HelpDesk.ps1
# Help desk triage functions. Dot-source this file:  . .\HelpDesk.ps1

function Get-TicketPriority {
    <#
    .SYNOPSIS
    P1-P4 from impact and urgency. Points: Single 1, Department 2, Company 3; Low 1, Medium 2, High 3.
    Total 6 = P1, 5 = P2, 4 = P3, otherwise P4.
    #>
    param(
        [Parameter(Mandatory)][ValidateSet('Single', 'Department', 'Company')][string]$Impact,
        [Parameter(Mandatory)][ValidateSet('Low', 'Medium', 'High')][string]$Urgency
    )
    $impactPoints = @{ Single = 1; Department = 2; Company = 3 }
    $urgencyPoints = @{ Low = 1; Medium = 2; High = 3 }
    $total = $impactPoints[$Impact] + $urgencyPoints[$Urgency]
    switch ($total) {
        6 { 'P1' }
        5 { 'P2' }
        4 { 'P3' }
        default { 'P4' }
    }
}

function Find-LikelyCause {
    <#
    .SYNOPSIS
    Knowledge-base entries matching a symptom, best first.
    .DESCRIPTION
    An entry's Score is how many of its Keywords appear in the symptom (ignoring case).
    Return entries with Score > 0 as objects with Problem, Score, Causes and Steps,
    sorted by Score (highest first), ties in knowledge-base order, at most -Top of them.
    #>
    param(
        [Parameter(Mandatory)][string]$Symptom,
        [Parameter(Mandatory)][object[]]$KnowledgeBase,
        [int]$Top = 3
    )
    $text = $Symptom.ToLower()
    $found = for ($i = 0; $i -lt $KnowledgeBase.Count; $i++) {
        $entry = $KnowledgeBase[$i]
        $score = @($entry.Keywords | Where-Object { $text.Contains($_.ToLower()) }).Count
        if ($score -gt 0) {
            [pscustomobject]@{
                Problem = $entry.Problem
                Score   = $score
                Causes  = @($entry.Causes)
                Steps   = @($entry.Steps)
                Index   = $i
            }
        }
    }
    $found |
        Sort-Object @{ Expression = 'Score'; Descending = $true }, Index |
        Select-Object -First $Top Problem, Score, Causes, Steps
}

function Format-TicketNote {
    <#
    .SYNOPSIS
    The standard ticket note, one field per line:
    Priority: P2
    Issue: ...
    Checked: ...
    Action: ...
    Result: ...
    Empty fields are written as "-". Lines are joined with "`n".
    #>
    param(
        [string]$Priority,
        [string]$Issue,
        [string]$Checked,
        [string]$Action,
        [string]$Result
    )
    $fields = [ordered]@{ Priority = $Priority; Issue = $Issue; Checked = $Checked; Action = $Action; Result = $Result }
    $lines = foreach ($name in $fields.Keys) {
        $value = if ([string]::IsNullOrWhiteSpace($fields[$name])) { '-' } else { $fields[$name] }
        "${name}: $value"
    }
    $lines -join "`n"
}
@@file kb.json
[
  {
    "Problem": "No power at all",
    "Keywords": ["no power", "won't turn on", "wont turn on", "dead", "no lights"],
    "Causes": ["Outlet or power strip off", "Loose or faulty power cable", "Failed power supply (PSU)", "Front power button cable"],
    "Steps": ["Try a known-good outlet and cable", "Check the PSU switch on the back", "Test the PSU with a tester"]
  },
  {
    "Problem": "Beeps or no display at startup",
    "Keywords": ["beep", "no display", "black screen", "no signal"],
    "Causes": ["RAM not seated or failed", "Graphics card or cable", "Monitor on the wrong input"],
    "Steps": ["Look up the beep code", "Reseat the RAM one stick at a time", "Check the monitor input and cable"]
  },
  {
    "Problem": "Slow or freezing PC",
    "Keywords": ["slow", "freez", "lag", "hang", "takes forever"],
    "Causes": ["RAM full and swapping", "Disk at 100% or nearly full", "Too many startup apps", "Malware"],
    "Steps": ["Check Task Manager for CPU/RAM/disk hogs", "Disable startup apps", "Free disk space", "Run a malware scan"]
  },
  {
    "Problem": "Blue screen / random crashes",
    "Keywords": ["blue screen", "bsod", "stop code", "crash", "reboots"],
    "Causes": ["Bad driver or update", "Faulty RAM", "Overheating", "Corrupt system files"],
    "Steps": ["Note the stop code", "Roll back recent drivers/updates", "Run Windows Memory Diagnostic", "Run DISM then sfc /scannow"]
  },
  {
    "Problem": "No network or 169.254 address",
    "Keywords": ["no internet", "169.254", "limited connectivity", "no network", "can't connect", "cannot connect"],
    "Causes": ["Cable unplugged or Wi-Fi off", "DHCP server unreachable", "Wrong VLAN or port disabled"],
    "Steps": ["Check link lights / Wi-Fi", "ipconfig /release then /renew", "Ping the gateway", "Check the switch port"]
  },
  {
    "Problem": "DNS: sites won't load by name",
    "Keywords": ["website", "websites", "dns", "can ping", "server not found"],
    "Causes": ["DNS server unreachable or wrong", "Stale DNS cache", "Hosts file changed"],
    "Steps": ["nslookup the name", "ipconfig /flushdns", "Check DNS servers in ipconfig /all"]
  },
  {
    "Problem": "Laser printer: ghosting or smearing",
    "Keywords": ["ghost", "smudge", "smear", "faded", "toner"],
    "Causes": ["Worn imaging drum", "Failing fuser (toner smears off)", "Wrong paper type setting"],
    "Steps": ["Print a test page", "Replace the drum/toner cartridge", "Replace the fuser if toner smears"]
  },
  {
    "Problem": "Paper jams",
    "Keywords": ["jam", "jams", "creased", "multiple sheets"],
    "Causes": ["Worn pickup rollers", "Damp or wrong paper", "Torn paper inside"],
    "Steps": ["Remove all paper and scraps", "Clean or replace the pickup rollers", "Use fresh, dry paper"]
  },
  {
    "Problem": "Print jobs stuck in the queue",
    "Keywords": ["print queue", "stuck", "spooler", "won't print", "wont print"],
    "Causes": ["Print Spooler service hung", "Corrupt print job", "Printer offline or IP changed"],
    "Steps": ["Clear the queue and restart the Print Spooler service", "Check the printer's IP and port", "Reinstall the driver"]
  },
  {
    "Problem": "Overheating",
    "Keywords": ["hot", "overheat", "fan loud", "loud fan", "shuts down", "shuts off"],
    "Causes": ["Dust in vents and heatsink", "Failed fan", "Dried thermal paste", "Blocked airflow"],
    "Steps": ["Clean with compressed air", "Check that all fans spin", "Replace thermal paste"]
  },
  {
    "Problem": "Laptop battery or charging",
    "Keywords": ["battery", "won't charge", "wont charge", "not charging", "swollen", "swelling"],
    "Causes": ["Wrong or failed charger", "Worn battery", "Swollen battery (fire risk!)"],
    "Steps": ["Check the charger wattage and port", "Check battery health (powercfg /batteryreport)", "Stop using a swollen battery and replace it"]
  },
  {
    "Problem": "Account locked or can't log in",
    "Keywords": ["locked", "password", "can't log", "cannot log", "login", "log in", "sign in"],
    "Causes": ["Too many failed attempts", "Expired password", "Caps Lock / keyboard layout", "Clock out of sync with the domain"],
    "Steps": ["Verify the caller's identity first", "Unlock / reset in Active Directory", "Check the PC's time"]
  },
  {
    "Problem": "Failing hard drive",
    "Keywords": ["clicking", "grinding", "smart", "bad sector", "disk error"],
    "Causes": ["Mechanical HDD failure", "Bad sectors"],
    "Steps": ["Back up the data immediately", "Check S.M.A.R.T. status", "Replace the drive"]
  },
  {
    "Problem": "Malware symptoms",
    "Keywords": ["pop-up", "popup", "redirect", "toolbar", "ransom", "encrypted files", "virus"],
    "Causes": ["Adware or browser hijacker", "Trojan", "Ransomware"],
    "Steps": ["Quarantine: disconnect from the network", "Follow the malware removal process", "Report ransomware to security immediately"]
  }
]
@@file tests.ps1
# Run: powershell -ExecutionPolicy Bypass -File tests.ps1
. "$PSScriptRoot\HelpDesk.ps1"
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

Write-Host 'Priority'
Test-That 'company + high = P1' { (Get-TicketPriority -Impact Company -Urgency High) -eq 'P1' }
Test-That 'company + medium = P2' { (Get-TicketPriority -Impact Company -Urgency Medium) -eq 'P2' }
Test-That 'department + high = P2' { (Get-TicketPriority -Impact Department -Urgency High) -eq 'P2' }
Test-That 'single + high = P3' { (Get-TicketPriority -Impact Single -Urgency High) -eq 'P3' }
Test-That 'department + low = P4' { (Get-TicketPriority -Impact Department -Urgency Low) -eq 'P4' }
Test-That 'single + low = P4' { (Get-TicketPriority -Impact Single -Urgency Low) -eq 'P4' }

Write-Host 'Likely causes'
$kb = Get-Content -Raw "$PSScriptRoot\kb.json" | ConvertFrom-Json
$r = @(Find-LikelyCause -Symptom 'Printer prints GHOST images and the toner smudges' -KnowledgeBase $kb)
Test-That 'best match first' { $r[0].Problem -eq 'Laser printer: ghosting or smearing' }
Test-That 'score counts keywords' { $r[0].Score -eq 3 }
Test-That 'result has causes and steps' { $r[0].Causes.Count -eq 3 -and $r[0].Steps.Count -eq 3 }
$slow = @(Find-LikelyCause -Symptom 'my laptop is so slow and keeps freezing, fan loud' -KnowledgeBase $kb)
Test-That 'slow beats overheating' { $slow[0].Problem -eq 'Slow or freezing PC' -and $slow[0].Score -eq 2 }
Test-That 'overheating second' { $slow[1].Problem -eq 'Overheating' -and $slow[1].Score -eq 1 }
$tie = @(Find-LikelyCause -Symptom 'dead and hot' -KnowledgeBase $kb)
Test-That 'ties keep knowledge-base order' { $tie[0].Problem -eq 'No power at all' -and $tie[1].Problem -eq 'Overheating' }
Test-That 'top limits results' { @(Find-LikelyCause -Symptom 'slow crash hot battery jam' -KnowledgeBase $kb -Top 2).Count -eq 2 }
Test-That 'no match, no results' { @(Find-LikelyCause -Symptom 'the coffee machine is sad' -KnowledgeBase $kb).Count -eq 0 }

Write-Host 'Ticket note'
$note = Format-TicketNote -Priority P3 -Issue 'Cannot print' -Checked 'Queue stuck' -Action 'Restarted spooler' -Result ''
$lines = $note -split "`n"
Test-That 'five lines' { $lines.Count -eq 5 }
Test-That 'priority line' { $lines[0] -eq 'Priority: P3' }
Test-That 'action line' { $lines[3] -eq 'Action: Restarted spooler' }
Test-That 'empty field is a dash' { $lines[4] -eq 'Result: -' }

if ($script:failed) { Write-Host "$script:failed test(s) failed" -ForegroundColor Red; exit 1 }
Write-Host 'All tests passed' -ForegroundColor Green
@@file Start-Triage.ps1
<#
.SYNOPSIS
Triage a help desk call: priority, likely causes, quick checks, and a saved ticket note.
.EXAMPLE
.\Start-Triage.ps1 -Symptom "outlook can't connect, websites won't load" -Impact Single -Urgency High
#>
param(
    [string]$Symptom,
    [ValidateSet('Single', 'Department', 'Company')][string]$Impact,
    [ValidateSet('Low', 'Medium', 'High')][string]$Urgency,
    [switch]$NoChecks
)

. "$PSScriptRoot\HelpDesk.ps1"

if (-not $Symptom) { $Symptom = Read-Host 'Describe the problem' }
if (-not $Impact) { $Impact = Read-Host 'Impact (Single, Department, Company)' }
if (-not $Urgency) { $Urgency = Read-Host 'Urgency (Low, Medium, High)' }

$priority = Get-TicketPriority -Impact $Impact -Urgency $Urgency
Write-Host ""
Write-Host "Priority: $priority  (impact $Impact, urgency $Urgency)" -ForegroundColor Cyan

$kb = Get-Content -Raw "$PSScriptRoot\kb.json" | ConvertFrom-Json
$causes = @(Find-LikelyCause -Symptom $Symptom -KnowledgeBase $kb)
Write-Host ""
Write-Host 'Likely causes'
if (-not $causes) { Write-Host '  Nothing in the knowledge base matches. Ask more questions, and add an entry when you solve it!' }
foreach ($c in $causes) {
    Write-Host ("  [{0}] {1}" -f $c.Score, $c.Problem) -ForegroundColor Yellow
    Write-Host ("      Causes: {0}" -f ($c.Causes -join '; '))
    Write-Host ("      Try:    {0}" -f ($c.Steps -join '; '))
}

$checked = @()
if (-not $NoChecks) {
    Write-Host ""
    Write-Host 'Quick checks on this PC'
    $ip = Get-NetIPConfiguration -ErrorAction SilentlyContinue |
        Where-Object { $_.IPv4DefaultGateway } | Select-Object -First 1
    if ($ip) {
        $gw = $ip.IPv4DefaultGateway.NextHop
        $ok = Test-Connection -ComputerName $gw -Count 1 -Quiet
        $line = "IPv4 $($ip.IPv4Address.IPAddress), gateway ${gw}: " + $(if ($ok) { 'gateway reachable' } else { 'gateway NOT reachable' })
    }
    else { $line = 'No network with a default gateway' }
    Write-Host "  $line"
    $checked += $line
    try {
        Resolve-DnsName www.microsoft.com -ErrorAction Stop | Out-Null
        $line = 'DNS lookup of www.microsoft.com: OK'
    }
    catch { $line = 'DNS lookup of www.microsoft.com: FAILED' }
    Write-Host "  $line"
    $checked += $line
}

$note = Format-TicketNote -Priority $priority -Issue $Symptom -Checked ($checked -join '; ') -Action '' -Result ''
$dir = Join-Path $PSScriptRoot 'tickets'
New-Item -ItemType Directory -Force $dir | Out-Null
$file = Join-Path $dir ((Get-Date -Format 'yyyyMMdd-HHmm') + '.txt')
Set-Content -Path $file -Value $note -Encoding UTF8
Write-Host ""
Write-Host "Ticket note saved to $file (fill in Action and Result when you're done)"

@@drill
What is step 1 of the CompTIA troubleshooting methodology?
= identify the problem
? Gather information and question the user first.

@@text
🎉 Prioritizing, diagnosing and documenting are the core of help desk work, and now you have a tool that does all three and a knowledge base that grows with you. In an interview, show the kb.json and explain why it's data and not code.

@@lesson p3-windows Managing Windows: Services, Processes, Events, CIM
@@concepts processes, automation
@@text
This is the PowerShell help desk and sysadmin people actually get hired for. Most of these examples are marked to not run from the lesson, because they change things or need administrator rights. Try them in a real PowerShell window, ideally in a **virtual machine** (Hyper-V and VirtualBox are free).

## Services

```powershell
Get-Service | Where-Object Status -eq 'Running' | Select-Object -First 5 Name, DisplayName
```

```powershell,norun
Get-Service -Name Spooler                     # the print spooler, the classic "printer is stuck" fix:
Restart-Service -Name Spooler
Get-Service | Where-Object { $_.StartType -eq 'Automatic' -and $_.Status -ne 'Running' }   # should run, but doesn't
Set-Service -Name 'wuauserv' -StartupType Manual
```

## Processes

```powershell
Get-Process | Sort-Object CPU -Descending | Select-Object -First 5 Name, Id, @{ n = 'MemMB'; e = { [math]::Round($_.WorkingSet64 / 1MB) } }
```

```powershell,norun
Stop-Process -Name notepad
Start-Process notepad.exe
Start-Process powershell -Verb RunAs     # start as administrator
```

## The event log

When something broke at 3 AM, the event log knows why.

```powershell,norun
# Errors in the System log in the last 24 hours:
Get-WinEvent -FilterHashtable @{ LogName = 'System'; Level = 2; StartTime = (Get-Date).AddDays(-1) } |
    Select-Object TimeCreated, ProviderName, Id, Message -First 20

# Failed logons (Security log, event 4625; needs admin):
Get-WinEvent -FilterHashtable @{ LogName = 'Security'; Id = 4625 } -MaxEvents 50
```

Event IDs worth memorizing: **4624** logon success, **4625** logon failure, **4740** account locked out, **6005/6006** event log started/stopped (boot/shutdown), **41** unexpected shutdown (Kernel-Power), **7036** service changed state.

## CIM: ask Windows about hardware and the OS

CIM (the modern form of WMI) can answer almost any question about a computer, locally or remotely:

```powershell
Get-CimInstance Win32_OperatingSystem | Select-Object Caption, Version, LastBootUpTime
Get-CimInstance Win32_LogicalDisk -Filter "DriveType=3" | Select-Object DeviceID, @{ n = 'FreeGB'; e = { [math]::Round($_.FreeSpace / 1GB, 1) } }
```

```powershell,norun
Get-CimInstance Win32_ComputerSystem | Select-Object Manufacturer, Model, TotalPhysicalMemory
Get-CimInstance Win32_BIOS | Select-Object SerialNumber        # asset tags for the inventory
Get-CimInstance Win32_OperatingSystem -ComputerName SRV01      # a remote computer
```

## Networking

```powershell,norun
Get-NetIPAddress -AddressFamily IPv4
Get-NetAdapter
Test-Connection 8.8.8.8 -Count 2
Test-NetConnection -ComputerName google.com -Port 443          # "can I reach this port?"
Resolve-DnsName contoso.com
Get-NetTCPConnection -State Listen | Sort-Object LocalPort
Clear-DnsClientCache
```

## The registry

The registry is a drive in PowerShell: `HKLM:` and `HKCU:`.

```powershell
Get-ItemProperty 'HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion' | Select-Object ProductName, CurrentBuild
```

```powershell,norun
# Installed programs (64-bit):
Get-ItemProperty 'HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\*' | Select-Object DisplayName, DisplayVersion | Sort-Object DisplayName
New-ItemProperty -Path 'HKCU:\Software\ITQuest' -Name 'Demo' -Value 1 -PropertyType DWord
```

@@quiz
A user says printing is stuck for everybody on the PC. Which command is the classic first fix?
* `Restart-Service -Name Spooler`
- `Stop-Process -Name printer`
- `Restart-Computer -Force`
- `Clear-DnsClientCache`
? The Print Spooler service holds the queue. Restarting it clears most stuck jobs.

@@quiz
Which Security event ID means a **failed** logon?
- 4624
* 4625
- 4740
- 7036
? 4624 is success, 4625 failure, 4740 a lockout.

@@quiz
You need to check whether a server's port 3389 (RDP) is reachable from your PC. Which cmdlet?
- `Test-Connection`
* `Test-NetConnection -Port 3389`
- `Get-NetTCPConnection`
- `Resolve-DnsName`
? `Test-Connection` is ping only (ICMP). `Test-NetConnection -Port` tries a TCP connection.

@@drill
Which cmdlet queries hardware and OS information through CIM?
= Get-CimInstance
? Get-CimInstance Win32_OperatingSystem.

@@flashcards
Get-Service / Restart-Service :: List and restart Windows services
Get-WinEvent -FilterHashtable :: Fast, filtered event log queries
Event 4624 / 4625 :: Logon success / logon failure
Event 4740 :: A user account was locked out
Event 41 (Kernel-Power) :: The computer shut down unexpectedly
Get-CimInstance Win32_OperatingSystem :: OS name, version, last boot time
Get-CimInstance Win32_LogicalDisk :: Drives with size and free space
Test-NetConnection -Port :: Can I reach a TCP port?
HKLM: / HKCU: :: The registry drives: machine-wide and current user

@@exercise Stopped automatic services
`$services` imitates `Get-Service` output. Print the `Name` of each service whose `StartType` is `Automatic` but `Status` isn't `Running`. These are the ones a monitoring tool should alert on.
@@starter
$services = @(
    [pscustomobject]@{ Name = 'Spooler';  Status = 'Stopped'; StartType = 'Automatic' }
    [pscustomobject]@{ Name = 'wuauserv'; Status = 'Stopped'; StartType = 'Manual' }
    [pscustomobject]@{ Name = 'W32Time';  Status = 'Running'; StartType = 'Automatic' }
    [pscustomobject]@{ Name = 'MSSQLSERVER'; Status = 'Stopped'; StartType = 'Automatic' }
    [pscustomobject]@{ Name = 'Fax';      Status = 'Stopped'; StartType = 'Disabled' }
)

@@expected
Spooler
MSSQLSERVER
@@hint
`Where-Object { $_.StartType -eq 'Automatic' -and $_.Status -ne 'Running' }`
@@solution
$services = @(
    [pscustomobject]@{ Name = 'Spooler';  Status = 'Stopped'; StartType = 'Automatic' }
    [pscustomobject]@{ Name = 'wuauserv'; Status = 'Stopped'; StartType = 'Manual' }
    [pscustomobject]@{ Name = 'W32Time';  Status = 'Running'; StartType = 'Automatic' }
    [pscustomobject]@{ Name = 'MSSQLSERVER'; Status = 'Stopped'; StartType = 'Automatic' }
    [pscustomobject]@{ Name = 'Fax';      Status = 'Stopped'; StartType = 'Disabled' }
)
$services | Where-Object { $_.StartType -eq 'Automatic' -and $_.Status -ne 'Running' } | ForEach-Object { $_.Name }

@@exercise Lockout report
`$events` imitates security events. Count the **4740** (lockout) events per `User` and print `user: N lockouts`, most first. Ignore other event IDs.
@@starter
$events = @(
    [pscustomobject]@{ Id = 4625; User = 'ana' }
    [pscustomobject]@{ Id = 4740; User = 'ben' }
    [pscustomobject]@{ Id = 4740; User = 'ana' }
    [pscustomobject]@{ Id = 4624; User = 'ben' }
    [pscustomobject]@{ Id = 4740; User = 'ben' }
    [pscustomobject]@{ Id = 4740; User = 'ben' }
)

@@expected
ben: 3 lockouts
ana: 1 lockouts
@@hint
`Where-Object Id -eq 4740 | Group-Object User | Sort-Object Count -Descending`
@@solution
$events = @(
    [pscustomobject]@{ Id = 4625; User = 'ana' }
    [pscustomobject]@{ Id = 4740; User = 'ben' }
    [pscustomobject]@{ Id = 4740; User = 'ana' }
    [pscustomobject]@{ Id = 4624; User = 'ben' }
    [pscustomobject]@{ Id = 4740; User = 'ben' }
    [pscustomobject]@{ Id = 4740; User = 'ben' }
)
$events | Where-Object Id -eq 4740 | Group-Object User | Sort-Object Count -Descending | ForEach-Object { "$($_.Name): $($_.Count) lockouts" }

@@exercise Uptime from CIM
Write `Get-UptimeDays` that takes a `LastBootUpTime` (a `[datetime]`) and a "now" time, and returns the uptime in **whole days** (rounded down). On a real PC you'd call it as `Get-UptimeDays (Get-CimInstance Win32_OperatingSystem).LastBootUpTime (Get-Date)`.
@@starter
function Get-UptimeDays([datetime]$Boot, [datetime]$Now) {
    throw "TODO: not done yet"
}

@@tests
check (Get-UptimeDays ([datetime]'2026-10-01 08:00') ([datetime]'2026-10-31 07:59')) 29 "booted Oct 1 08:00, now Oct 31 07:59"
check (Get-UptimeDays ([datetime]'2026-10-31 01:00') ([datetime]'2026-10-31 23:00')) 0 "same day"
check (Get-UptimeDays ([datetime]'2025-10-31') ([datetime]'2026-10-31')) 365 "a year"
@@hint
Subtracting two dates gives a `[timespan]`: `($Now - $Boot).Days` is whole days.
@@solution
function Get-UptimeDays([datetime]$Boot, [datetime]$Now) {
    return ($Now - $Boot).Days
}

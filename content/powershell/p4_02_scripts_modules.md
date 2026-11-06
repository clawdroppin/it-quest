@@lesson p4-scripts Scripts, Modules and Scheduled Tasks
@@concepts modules, automation
@@text
## Scripts and the execution policy

A script is a `.ps1` file. Run it with its path: `.\Get-Report.ps1` (the `.\` is required, so a file in the current folder can't pretend to be a command).

The first time, Windows may refuse: *"running scripts is disabled on this system"*. That's the **execution policy**:

```powershell,norun
Get-ExecutionPolicy -List
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser    # your own scripts run; downloaded ones must be signed
Unblock-File .\downloaded-script.ps1                   # trust one downloaded file after reading it
```

The execution policy is a **safety belt, not a security boundary**: it stops accidents, not attackers. Companies often enforce it with Group Policy.

## Script parameters

A script can have a `param()` block at the top, exactly like a function:

```powershell,norun
# Get-Report.ps1
[CmdletBinding()]
param(
    [Parameter(Mandatory)][string]$ComputerName,
    [string]$OutFile = "$PSScriptRoot\report.html"
)
Write-Verbose "Collecting from $ComputerName"
```

`.\Get-Report.ps1 -ComputerName SRV01 -Verbose`. `$PSScriptRoot` is the folder the script lives in, so it can find files next to itself no matter where it's run from.

## Modules: sharing your functions

A **module** is a `.psm1` file of functions (plus an optional `.psd1` manifest), in a folder of the same name under one of the `$env:PSModulePath` folders:

```text
Documents\WindowsPowerShell\Modules\
  ITTools\
    ITTools.psm1     ← functions
    ITTools.psd1     ← version, author, which functions to export
```

```powershell,norun
Import-Module ITTools           # (also loads automatically when you use one of its commands)
Get-Command -Module ITTools
Export-ModuleMember -Function Get-DiskStatus, New-ServerName   # in the .psm1: what's public
```

Thousands of ready-made modules are on the **PowerShell Gallery**:

```powershell,norun
Find-Module -Name *Teams*
Install-Module -Name PSScriptAnalyzer -Scope CurrentUser
Install-Module -Name Microsoft.Graph -Scope CurrentUser   # Microsoft 365 / Entra ID
Install-Module -Name Az -Scope CurrentUser                # Azure
```

## Profiles

`$PROFILE` is a script that runs each time PowerShell starts: a good place for your own aliases and functions. `notepad $PROFILE` (create it with `New-Item $PROFILE -Force` first).

## Scheduled tasks

Automation is only automation if it runs without you:

```powershell,norun
$action  = New-ScheduledTaskAction -Execute 'powershell.exe' -Argument '-NoProfile -ExecutionPolicy Bypass -File C:\Scripts\Get-Report.ps1'
$trigger = New-ScheduledTaskTrigger -Daily -At 7am
Register-ScheduledTask -TaskName 'Daily health report' -Action $action -Trigger $trigger -User 'SYSTEM' -RunLevel Highest
Get-ScheduledTask -TaskName 'Daily health report' | Get-ScheduledTaskInfo   # last run time and result
```

## Logging what a script did

```powershell,norun
Start-Transcript -Path "C:\Logs\report-$(Get-Date -Format yyyyMMdd).log"
# ... the script ...
Stop-Transcript
```

@@quiz
"Running scripts is disabled on this system." What's the usual fix on your own PC?
* `Set-ExecutionPolicy RemoteSigned -Scope CurrentUser`
- Run the script from cmd.exe
- Rename it to .bat
- `Set-ExecutionPolicy Unrestricted -Scope LocalMachine -Force` on every PC
? RemoteSigned for your user lets local scripts run while still blocking unsigned downloaded ones.

@@quiz
What is `$PSScriptRoot`?
- The PowerShell installation folder
* The folder containing the running script
- The user's home folder
- The current working directory
? It lets a script find files next to it, wherever it's run from.

@@quiz
Which file type is a PowerShell module's code?
- `.ps1`
* `.psm1`
- `.psd1`
- `.dll` only
? `.psm1` holds the code; `.psd1` is the manifest (metadata).

@@drill
Which file extension does a script module use?
= .psm1
= psm1
? A .psd1 manifest describes it.

@@flashcards
Execution policy :: Controls whether scripts may run (Restricted, RemoteSigned, AllSigned, Bypass); not a security boundary
RemoteSigned :: Local scripts run; downloaded scripts must be signed
$PSScriptRoot :: Folder of the running script
.psm1 / .psd1 :: Module code / module manifest
Install-Module :: Install a module from the PowerShell Gallery
$PROFILE :: Script that runs when PowerShell starts
Register-ScheduledTask :: Make Windows run a command on a schedule
Start-Transcript :: Record everything a session prints into a log file

@@exercise Build a script command line
Write `Get-TaskCommand` that takes a script path and returns the **argument string** a scheduled task should pass to `powershell.exe`: `-NoProfile -ExecutionPolicy Bypass -File "<path>"` (with the path in double quotes, so spaces work).
@@starter
function Get-TaskCommand([string]$ScriptPath) {
    throw "TODO: not done yet"
}

@@tests
check (Get-TaskCommand 'C:\Scripts\report.ps1') '-NoProfile -ExecutionPolicy Bypass -File "C:\Scripts\report.ps1"' "Get-TaskCommand 'C:\Scripts\report.ps1'"
check (Get-TaskCommand 'C:\My Scripts\a.ps1') '-NoProfile -ExecutionPolicy Bypass -File "C:\My Scripts\a.ps1"' "a path with a space"
@@hint
Inside a double-quoted string, write a literal `"` as a doubled `""` or as `` `" ``. Or use single quotes with `-f`: `'-NoProfile -ExecutionPolicy Bypass -File "{0}"' -f $ScriptPath`.
@@solution
function Get-TaskCommand([string]$ScriptPath) {
    return '-NoProfile -ExecutionPolicy Bypass -File "{0}"' -f $ScriptPath
}

@@exercise Log file name
Write `Get-LogName` that takes a script name and a date, and returns a log file name like `Backup-2026-10-31.log`. Strip a `.ps1` extension from the script name if it has one.
@@starter
function Get-LogName([string]$Script, [datetime]$Date) {
    throw "TODO: not done yet"
}

@@tests
check (Get-LogName 'Backup.ps1' ([datetime]'2026-10-31')) 'Backup-2026-10-31.log' "Get-LogName 'Backup.ps1'"
check (Get-LogName 'Cleanup' ([datetime]'2026-01-05')) 'Cleanup-2026-01-05.log' "Get-LogName 'Cleanup'"
@@hint
`$name = $Script -replace '\.ps1$', ''` and `$Date.ToString('yyyy-MM-dd')`. (Careful: `MM` is month, `mm` is minutes!)
@@solution
function Get-LogName([string]$Script, [datetime]$Date) {
    $name = $Script -replace '\.ps1$', ''
    return "$name-$($Date.ToString('yyyy-MM-dd')).log"
}

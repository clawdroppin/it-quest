@@lesson p1-functions Functions and Parameters
@@concepts functions
@@text
```powershell
function Get-Greeting {
    param(
        [string]$Name,
        [int]$Times = 1
    )
    for ($i = 0; $i -lt $Times; $i++) {
        "Hello, $Name!"
    }
}

Get-Greeting -Name 'Ana'
Get-Greeting -Name 'Ben' -Times 2
Get-Greeting 'Kenji'         # positional: the first parameter
```

Name your functions **Verb-Noun** with an approved verb (`Get-Verb` lists them), so they feel like built-in cmdlets.

## The big gotcha: calling functions

Functions are called **like commands**, with spaces, never with parentheses and commas:

```powershell
function Add-Numbers($a, $b) { $a + $b }
Add-Numbers 2 3        # 5  ✅
Add-Numbers(2, 3)      # passes ONE array @(2, 3) as $a!  ❌
```

Methods on objects (`.ToUpper()`, `[math]::Round(1.5, 0)`) *do* use parentheses and commas. Functions and cmdlets don't.

## Everything is output

A function's result is **everything it outputs**, not just what follows `return`:

```powershell
function Get-Size {
    "calculating..."   # oops: this is output too
    return 42
}
$x = Get-Size
$x.Count        # 2: an array of the text and 42
```

Use `Write-Host` (shows on screen only) or `Write-Verbose` for messages that aren't the result. `return` just stops the function early.

## Switches and mandatory parameters

```powershell
function Remove-OldLogs {
    param(
        [Parameter(Mandatory)]
        [string]$Path,
        [int]$Days = 30,
        [switch]$WhatIfOnly
    )
    if ($WhatIfOnly) { "Would remove logs older than $Days days in $Path" }
    else { "Removing logs older than $Days days in $Path" }
}
Remove-OldLogs -Path 'C:\Logs' -WhatIfOnly
```

A `[switch]` is on when present: `-WhatIfOnly`. A `Mandatory` parameter makes PowerShell ask for it if it's missing.

## Scope

Variables made inside a function disappear when it ends, and assigning to an outside variable creates a new local one. Pass values in as parameters and output results instead of changing outside variables.

@@quiz
`function Mul($a, $b) { $a * $b }`. Which call gives 12?
* `Mul 3 4`
- `Mul(3, 4)`
- `Mul(3 4)`
- `Mul -3 -4`
? `Mul(3, 4)` passes one array. Functions are called like commands: `Mul 3 4`.

@@quiz
What does a function return?
- only the value after `return`
* everything it outputs, plus the value after `return`
- nothing unless you use `Write-Output`
- the last variable assigned
? Every uncaptured value is output. `Write-Host` messages are not.

@@quiz
Which parameter declaration makes `-Name` required?
* [Parameter(Mandatory)][string]$Name
- [string]$Name = $null
- [required]$Name
- $Name!
? PowerShell prompts for a mandatory parameter if it's missing (or fails when non-interactive).

@@flashcards
function Verb-Noun { param(...) } :: define a function
Call: Mul 3 4 :: arguments are separated by spaces, not commas
param() :: declare parameters
[switch]$Force :: an on/off parameter
Mandatory :: the parameter must be supplied
everything is output :: any uncaptured value is returned
$script: / $global: :: scope modifiers

@@exercise Uptime text
Write `Format-Uptime` with a parameter `-Minutes` (an int) that returns text like `2d 3h 15m`. Leave out parts that are zero at the front: `95` minutes is `1h 35m`, `5` is `5m`.
@@starter
function Format-Uptime {
    param([int]$Minutes)
    throw "TODO: not done yet"
}

@@tests
check (Format-Uptime -Minutes 3075) "2d 3h 15m" "Format-Uptime -Minutes 3075"
check (Format-Uptime -Minutes 95) "1h 35m" "Format-Uptime -Minutes 95"
check (Format-Uptime -Minutes 5) "5m" "Format-Uptime -Minutes 5"
check (Format-Uptime -Minutes 1440) "1d 0h 0m" "Format-Uptime -Minutes 1440"
@@hint
Days: `[math]::Floor($Minutes / 1440)`. Hours: `[math]::Floor(($Minutes % 1440) / 60)`. Minutes: `$Minutes % 60`. Then `if ($d -gt 0) { return "${d}d ${h}h ${m}m" }` and so on. (`${d}d` keeps the variable name from swallowing the `d`.)
@@solution
function Format-Uptime {
    param([int]$Minutes)
    $d = [math]::Floor($Minutes / 1440)
    $h = [math]::Floor(($Minutes % 1440) / 60)
    $m = $Minutes % 60
    if ($d -gt 0) { return "${d}d ${h}h ${m}m" }
    if ($h -gt 0) { return "${h}h ${m}m" }
    return "${m}m"
}

@@exercise Port classifier
Write `Get-PortType` with a `-Port` parameter: return `well-known` for 0–1023, `registered` for 1024–49151, `dynamic` for 49152–65535, and `invalid` for anything else.
@@starter
function Get-PortType {
    param([int]$Port)
    throw "TODO: not done yet"
}

@@tests
check (Get-PortType -Port 443) "well-known" "Get-PortType 443"
check (Get-PortType -Port 3389) "registered" "Get-PortType 3389"
check (Get-PortType -Port 50000) "dynamic" "Get-PortType 50000"
check (Get-PortType -Port 70000) "invalid" "Get-PortType 70000"
check (Get-PortType -Port -1) "invalid" "Get-PortType -1"
@@hint
Test invalid first: `if ($Port -lt 0 -or $Port -gt 65535) { return 'invalid' }`.
@@solution
function Get-PortType {
    param([int]$Port)
    if ($Port -lt 0 -or $Port -gt 65535) { return 'invalid' }
    if ($Port -le 1023) { return 'well-known' }
    if ($Port -le 49151) { return 'registered' }
    return 'dynamic'
}

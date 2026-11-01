@@lesson p6-tool-design Designing PowerShell Tools Other People Use
@@concepts modules, functions, errors
@@text
A script you run yourself can be rough. A **tool** is used by colleagues who don't know how it works, in pipelines you never imagined, on servers you've never seen. Good PowerShell tools feel like Microsoft's own cmdlets: they take objects, return objects, support `-WhatIf`, fail clearly, and come with help. This lesson is about that design.

## The tool-making rules

1. **One job per command**, named with an approved verb: `Get-Verb` lists them. `Get-`, `Set-`, `New-`, `Remove-`, `Add-`, `Test-`, `Invoke-`, `Export-`, `Import-`, `ConvertTo-`.
2. **Output objects, never formatted text.** `Format-Table` and `Write-Host` inside a tool break everything downstream. Let the caller decide how to show it.
3. **Accept pipeline input** where it makes sense, and process items one at a time in `process {}`.
4. **Validate with attributes**, not `if` statements.
5. **Anything that changes something supports `-WhatIf` and `-Confirm`.**
6. **Errors are errors**: `Write-Error` for one bad item (carry on with the rest), `throw` when nothing can continue.

## Objects with a type name

```powershell
function Get-PortInfo {
    [CmdletBinding()]
    [OutputType('ItQuest.PortInfo')]
    param([Parameter(Mandatory, ValueFromPipeline)][ValidateRange(1, 65535)][int]$Port)
    begin {
        $known = @{ 22 = 'ssh'; 53 = 'dns'; 80 = 'http'; 443 = 'https'; 3389 = 'rdp' }
    }
    process {
        [pscustomobject]@{
            PSTypeName = 'ItQuest.PortInfo'      # a type name: format files and other tools can recognize it
            Port       = $Port
            Service    = if ($known.ContainsKey($Port)) { $known[$Port] } else { 'unknown' }
            WellKnown  = $Port -lt 1024
        }
    }
}

22, 443, 8080 | Get-PortInfo | Format-Table
(Get-PortInfo 3389).PSObject.TypeNames[0]
```

`begin` runs once before the first item, `process` once per item, `end` once at the end. Setting up the lookup table in `begin` means it isn't rebuilt for every port.

## Parameter sets: one command, several ways to call it

```powershell,norun
function Get-Employee {
    [CmdletBinding(DefaultParameterSetName = 'ByName')]
    param(
        [Parameter(Mandatory, ParameterSetName = 'ByName', Position = 0)][string]$Name,
        [Parameter(Mandatory, ParameterSetName = 'ById')][int]$Id,
        [Parameter(Mandatory, ParameterSetName = 'ByDepartment')][string]$Department
    )
    switch ($PSCmdlet.ParameterSetName) {
        'ByName'       { ... }
        'ById'         { ... }
        'ByDepartment' { ... }
    }
}
```

PowerShell then refuses `Get-Employee -Name ana -Id 7` by itself, and `Get-Help` shows the three ways to call it.

## Tab completion for your parameters

```powershell,norun
function Connect-Office {
    param(
        [ArgumentCompleter({
            param($cmd, $param, $word)
            'Tokyo', 'Osaka', 'London', 'Berlin' | Where-Object { $_ -like "$word*" }
        })]
        [string]$Office
    )
}
```

Users type `Connect-Office -Office T` and press Tab. (`[ValidateSet(...)]` also gives tab completion, when the list is fixed.)

## Errors that callers can handle

```powershell,norun
function Restart-AppPool {
    [CmdletBinding(SupportsShouldProcess)]
    param([Parameter(Mandatory, ValueFromPipeline)][string[]]$Name)
    process {
        foreach ($n in $Name) {
            if (-not (Test-AppPool $n)) {
                Write-Error -Message "App pool '$n' not found" -Category ObjectNotFound -TargetObject $n
                continue                                    # one bad name doesn't stop the others
            }
            if ($PSCmdlet.ShouldProcess($n, 'Restart')) { ... }
        }
    }
}

# The caller chooses: stop on the first error, or collect them.
Restart-AppPool web1, nope, web2 -ErrorAction Stop
Restart-AppPool web1, nope, web2 -ErrorVariable problems
```

## Module layout

A module that grows past one file usually looks like this:

```text
NetTools/
├── NetTools.psd1          # the manifest: version, author, requirements, FunctionsToExport
├── NetTools.psm1          # dot-sources every file in Public and Private, exports Public
├── Public/                # one file per exported command
│   ├── Get-PortInfo.ps1
│   └── Test-Endpoint.ps1
├── Private/               # helpers the user never sees
│   └── ConvertTo-Cidr.ps1
├── en-US/about_NetTools.help.txt
└── tests/NetTools.Tests.ps1
```

```powershell,norun
# NetTools.psm1
$public  = @(Get-ChildItem "$PSScriptRoot\Public\*.ps1")
$private = @(Get-ChildItem "$PSScriptRoot\Private\*.ps1")
foreach ($file in $public + $private) { . $file.FullName }
Export-ModuleMember -Function $public.BaseName
```

Create the manifest with `New-ModuleManifest -Path NetTools.psd1 -RootModule NetTools.psm1 -FunctionsToExport ...`. **List the functions explicitly** in `FunctionsToExport` (not `*`): PowerShell can then find your commands without loading the module, so auto-loading is fast.

## Comment-based help

```powershell,norun
function Get-PortInfo {
    <#
    .SYNOPSIS
    Describes TCP ports.
    .EXAMPLE
    22, 443 | Get-PortInfo
    .EXAMPLE
    Get-PortInfo 3389 | Where-Object WellKnown
    #>
    ...
}
```

Now `Get-Help Get-PortInfo -Examples` works like it does for built-in commands.

@@quiz
Your tool ends with `| Format-Table`. What goes wrong?
- Nothing, it's just prettier
* The output becomes formatting objects, so the caller can't sort, filter or export it
- Format-Table is slow
- It only works in PowerShell 7
? Format-* is the last thing in a pipeline, always typed by the user, never inside a tool.

@@quiz
Why build a lookup table in `begin {}` rather than in `process {}`?
* `begin` runs once; `process` runs once per piped item, so the table would be rebuilt every time
- Variables in `process` can't be hashtables
- `begin` runs in parallel
- `process` can't see parameters
? begin = setup, process = per item, end = summary/cleanup.

@@quiz
`Restart-AppPool web1, nope, web2` and `nope` doesn't exist. What should a well-designed tool do by default?
- Throw and restart nothing
- Silently skip it
* Write a non-terminating error for `nope` and still restart web1 and web2
- Ask the user what to do
? Like built-in cmdlets: report each bad item, carry on, and let the caller use `-ErrorAction Stop` to change it.

@@quiz
Why list functions explicitly in a manifest's `FunctionsToExport` instead of `'*'`?
- `*` exports private functions too
* PowerShell can see the commands without loading the module, so auto-loading and tab completion are fast
- Wildcards aren't allowed in manifests
- It's needed for code signing
? With `*`, PowerShell has to import the module just to discover what's in it.

@@flashcards
`Get-Verb` :: lists the approved verbs for command names
`[OutputType()]` :: documents what a command returns (used by tab completion)
`PSTypeName` in a `[pscustomobject]` :: gives the object a custom type name
`begin` / `process` / `end` :: once before / per pipeline item / once after
parameter sets :: several mutually exclusive ways to call one command
`$PSCmdlet.ParameterSetName` :: which set the caller used
`[ArgumentCompleter()]` :: custom tab completion for a parameter
`Write-Error` vs `throw` :: one item failed (carry on) vs nothing can continue
`New-ModuleManifest` :: creates the .psd1
Public/ and Private/ :: exported commands vs internal helpers
comment-based help :: `.SYNOPSIS`, `.EXAMPLE` inside `<# #>`

@@exercise Objects with a type name
Write `ConvertTo-DeviceRecord`, which takes lines like `"tokyo-sw01,10.0.0.2,switch"` **from the pipeline** and outputs one object per line with `Hostname`, `IP` and `Role`, and the type name `ItQuest.Device`. Skip blank lines. A line without exactly 3 fields is an error for that line only: `Write-Error "Bad line: <line>"`, then carry on.
@@starter
function ConvertTo-DeviceRecord {
    [CmdletBinding()]
    param([Parameter(ValueFromPipeline)][string]$Line)
    process {
        throw "TODO: not done yet"
    }
}

@@tests
$lines = 'tokyo-sw01,10.0.0.2,switch', '', 'broken line', 'osaka-fw01,10.1.0.1,firewall'
$devices = @($lines | ConvertTo-DeviceRecord -ErrorVariable errs -ErrorAction SilentlyContinue)
check $devices.Count 2 "two good lines"
check $devices[0].PSObject.TypeNames[0] 'ItQuest.Device' "type name"
check ($devices | ForEach-Object { "$($_.Hostname)|$($_.IP)|$($_.Role)" }) @('tokyo-sw01|10.0.0.2|switch', 'osaka-fw01|10.1.0.1|firewall') "fields"
check @($errs).Count 1 "one error"
check "$($errs[0])" 'Bad line: broken line' "error message"
@@hint
In `process`: `if (-not $Line.Trim()) { return }`, `$f = $Line.Split(',')`, `if ($f.Count -ne 3) { Write-Error "Bad line: $Line"; return }`, then output `[pscustomobject]@{ PSTypeName = 'ItQuest.Device'; Hostname = $f[0]; IP = $f[1]; Role = $f[2] }`.
@@solution
function ConvertTo-DeviceRecord {
    [CmdletBinding()]
    param([Parameter(ValueFromPipeline)][string]$Line)
    process {
        if (-not $Line.Trim()) { return }
        $f = $Line.Split(',')
        if ($f.Count -ne 3) { Write-Error "Bad line: $Line"; return }
        [pscustomobject]@{ PSTypeName = 'ItQuest.Device'; Hostname = $f[0].Trim(); IP = $f[1].Trim(); Role = $f[2].Trim() }
    }
}

@@exercise begin, process, end
Write `Measure-Transfer`, which takes file sizes in bytes (`[long]`) from the pipeline and, **once at the end**, outputs one object with `Count`, `TotalMB` (rounded to 1 decimal place) and `LargestMB` (also rounded). With no input it outputs `Count` 0 and both sizes 0.
@@starter
function Measure-Transfer {
    [CmdletBinding()]
    param([Parameter(ValueFromPipeline)][long]$Bytes)
    begin { }
    process { }
    end { throw "TODO: not done yet" }
}

@@tests
$r = 1048576, 5242880, 2621440 | Measure-Transfer
check "$($r.Count) $($r.TotalMB) $($r.LargestMB)" '3 8.5 5' "three files"
$e = @() | Measure-Transfer
check "$($e.Count) $($e.TotalMB) $($e.LargestMB)" '0 0 0' "no files"
@@hint
`begin { $count = 0; $total = 0; $largest = 0 }`, in `process` add to them, and in `end` output `[pscustomobject]@{ Count = $count; TotalMB = [math]::Round($total / 1MB, 1); LargestMB = [math]::Round($largest / 1MB, 1) }`. `1MB` is a built-in constant (1048576).
@@solution
function Measure-Transfer {
    [CmdletBinding()]
    param([Parameter(ValueFromPipeline)][long]$Bytes)
    begin { $count = 0; $total = 0; $largest = 0 }
    process {
        $count++
        $total += $Bytes
        if ($Bytes -gt $largest) { $largest = $Bytes }
    }
    end {
        [pscustomobject]@{
            Count     = $count
            TotalMB   = [math]::Round($total / 1MB, 1)
            LargestMB = [math]::Round($largest / 1MB, 1)
        }
    }
}

@@lesson p4-advanced-functions Advanced Functions: Write Your Own Cmdlets
@@concepts functions, testing
@@text
Add `[CmdletBinding()]` and your function gets the powers of a real cmdlet: `-Verbose`, `-ErrorAction`, `-WhatIf`, pipeline input and validated parameters.

```powershell
function Get-DiskStatus {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory, ValueFromPipeline)]
        [pscustomobject]$Disk,

        [ValidateRange(1, 99)]
        [int]$WarnPercent = 15
    )
    begin   { Write-Verbose "Checking with a $WarnPercent% threshold" }
    process {
        $pct = [math]::Round($Disk.FreeGB / $Disk.SizeGB * 100)
        [pscustomobject]@{
            Drive   = $Disk.Drive
            FreePct = $pct
            Status  = if ($pct -lt $WarnPercent) { 'LOW' } else { 'OK' }
        }
    }
    end     { Write-Verbose 'Done' }
}

$disks = @(
    [pscustomobject]@{ Drive = 'C:'; SizeGB = 476; FreeGB = 38 }
    [pscustomobject]@{ Drive = 'D:'; SizeGB = 931; FreeGB = 610 }
)
$disks | Get-DiskStatus -Verbose
```

## begin, process, end

With pipeline input, `process { }` runs **once per object**, `begin` once before the first, `end` once after the last. Without these blocks, the function would only see the *last* object from the pipeline.

## Validation attributes

Let PowerShell check the input, so your code doesn't have to:

| Attribute | Example |
|---|---|
| `[Parameter(Mandatory)]` | must be given |
| `[ValidateSet('Dev','Test','Prod')]` | one of these (with Tab completion!) |
| `[ValidateRange(1, 65535)]` | number range |
| `[ValidatePattern('^SRV-\d{3}$')]` | must match a regex |
| `[ValidateNotNullOrEmpty()]` | not empty |
| `[ValidateScript({ Test-Path $_ })]` | any custom test |

```powershell
function Connect-Env {
    param([ValidateSet('Dev', 'Test', 'Prod')][string]$Environment)
    "Connecting to $Environment"
}
Connect-Env -Environment Test
try { Connect-Env -Environment Staging } catch { "Rejected: $($_.Exception.Message.Split('.')[0])" }
```

## -WhatIf and -Confirm for free

`SupportsShouldProcess` adds `-WhatIf` and `-Confirm`. Wrap the dangerous part in `$PSCmdlet.ShouldProcess(...)`:

```powershell
function Remove-TempFile {
    [CmdletBinding(SupportsShouldProcess)]
    param([string[]]$Name)
    foreach ($n in $Name) {
        if ($PSCmdlet.ShouldProcess($n, 'Delete')) {
            "deleted $n"
        }
    }
}
Remove-TempFile -Name a.tmp, b.tmp -WhatIf
```

With `-WhatIf`, PowerShell prints `What if: Performing the operation "Delete" on target "a.tmp"` and skips the body. Any function that deletes or changes things should support this.

## Comment-based help

Put a help block at the top of the function and `Get-Help Get-DiskStatus -Examples` works like for built-in cmdlets:

```powershell,norun
function Get-DiskStatus {
    <#
    .SYNOPSIS
    Shows which disks are low on space.
    .EXAMPLE
    Get-CimInstance Win32_LogicalDisk | Get-DiskStatus -WarnPercent 10
    #>
    [CmdletBinding()]
    param(...)
}
```

@@quiz
A function has `process { ... }`. You pipe 5 objects into it. How many times does `process` run?
- once
* 5 times
- 6 times (once more for `end`)
- it depends on `-Verbose`
? `process` runs once per pipeline object.

@@quiz
What does `[CmdletBinding(SupportsShouldProcess)]` give your function?
* `-WhatIf` and `-Confirm` parameters
- pipeline input
- faster execution
- automatic logging to a file
? Then `if ($PSCmdlet.ShouldProcess(...))` guards the part that makes changes.

@@quiz
What does `[ValidateSet('Low', 'High')]` on a parameter do?
* Rejects any value other than Low or High, and enables tab completion of those values
- Converts the value to upper case
- Sets a default value
- Makes the parameter mandatory
? Validation attributes check input before the function body runs.

@@drill
Which attribute turns a function into an advanced function with -Verbose and -WhatIf support?
= [CmdletBinding()]
= CmdletBinding
= [CmdletBinding(SupportsShouldProcess)]
? Add SupportsShouldProcess for -WhatIf.

@@flashcards
[CmdletBinding()] :: Makes a function an advanced function, with -Verbose, -ErrorAction and more
ValueFromPipeline :: A parameter that receives objects from the pipeline
begin / process / end :: Run once before / once per pipeline object / once after
[ValidateSet()] :: Parameter must be one of a fixed list (with Tab completion)
SupportsShouldProcess :: Adds -WhatIf and -Confirm
Write-Verbose :: A message shown only with -Verbose

@@exercise Pipeline-aware function
Write `Get-LargeFile`, an advanced function that takes file objects **from the pipeline** (parameter `-File`) and outputs only the **names** of files bigger than `-MinMB` (default **100**) megabytes.
@@starter
function Get-LargeFile {
    [CmdletBinding()]
    param(
        # TODO: a -File parameter from the pipeline, and -MinMB with a default of 100
    )
    throw "TODO: not done yet"
}

@@tests
$files = @(
    [pscustomobject]@{ Name = 'video.mp4'; Length = 700MB }
    [pscustomobject]@{ Name = 'notes.txt'; Length = 2KB }
    [pscustomobject]@{ Name = 'backup.zip'; Length = 150MB }
)
check ($files | Get-LargeFile) @('video.mp4', 'backup.zip') "files | Get-LargeFile"
check ($files | Get-LargeFile -MinMB 500) 'video.mp4' "files | Get-LargeFile -MinMB 500"
@@hint
`param([Parameter(ValueFromPipeline)]$File, [int]$MinMB = 100)`, then `process { if ($File.Length -gt $MinMB * 1MB) { $File.Name } }`. Without `process`, only the last file is seen.
@@solution
function Get-LargeFile {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory, ValueFromPipeline)]
        $File,
        [int]$MinMB = 100
    )
    process {
        if ($File.Length -gt $MinMB * 1MB) {
            $File.Name
        }
    }
}

@@exercise Validated parameters
Write `New-ServerName` with two parameters: `-Site` must be one of `TKY`, `OSA` or `LON` (use `ValidateSet`), and `-Number` must be 1 to 999 (use `ValidateRange`). It returns names like `TKY-SRV-007` (the number padded to 3 digits). Invalid input must be **rejected by the attributes**.
@@starter
function New-ServerName {
    param(
        [string]$Site,
        [int]$Number
    )
    throw "TODO: not done yet"
}

@@tests
check (New-ServerName -Site TKY -Number 7) 'TKY-SRV-007' "New-ServerName -Site TKY -Number 7"
check (New-ServerName -Site LON -Number 120) 'LON-SRV-120' "New-ServerName -Site LON -Number 120"
foreach ($bad in @(@{ Site = 'NYC'; Number = 1 }, @{ Site = 'OSA'; Number = 0 }, @{ Site = 'OSA'; Number = 1000 })) {
    $rejected = $false
    try { New-ServerName @bad | Out-Null } catch { $rejected = $true }
    check $rejected $true "New-ServerName -Site $($bad.Site) -Number $($bad.Number) should be rejected"
}
@@requires
ValidateSet
ValidateRange
@@hint
`[ValidateSet('TKY', 'OSA', 'LON')][string]$Site`, and the name with `'{0}-SRV-{1:D3}' -f $Site, $Number`. `D3` pads a whole number to 3 digits.
@@solution
function New-ServerName {
    param(
        [ValidateSet('TKY', 'OSA', 'LON')]
        [string]$Site,
        [ValidateRange(1, 999)]
        [int]$Number
    )
    return '{0}-SRV-{1:D3}' -f $Site, $Number
}

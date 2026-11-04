@@lesson e7-errors Error Handling in Depth
@@concepts errors, functions
@@text
Module 3 covered `try`/`catch` and `-ErrorAction`. This lesson explains the model underneath, so you can predict what happens in any situation and write functions whose errors behave like a built-in cmdlet's.

## Two kinds of errors

- **Non-terminating**: the cmdlet reports a problem with one item and **continues** (`Get-Item a, missing, b`). By default, `try`/`catch` does **not** see them.
- **Terminating**: execution of the statement stops: `throw`, .NET exceptions, syntax errors, and any non-terminating error once `-ErrorAction Stop` or `$ErrorActionPreference = 'Stop'` is in effect.

```powershell
try { Get-Item 'C:\nope-1', 'C:\nope-2' -ErrorAction SilentlyContinue; 'try finished' } catch { 'caught' }
try { Get-Item 'C:\nope-1' -ErrorAction Stop; 'not reached' } catch { "caught: $($_.CategoryInfo.Category)" }
```

## The ErrorRecord

Inside `catch`, `$_` is an **ErrorRecord**:

| Property | Contains |
|---|---|
| `.Exception` | the .NET exception (`.Message`, `.InnerException`, its type) |
| `.CategoryInfo` | category (ObjectNotFound, PermissionDenied...), target |
| `.FullyQualifiedErrorId` | a stable ID, good for matching: `PathNotFound,Microsoft.PowerShell.Commands.GetItemCommand` |
| `.TargetObject` | what the command was working on |
| `.InvocationInfo` | script name, line number, the line itself (`.PositionMessage`) |
| `.ScriptStackTrace` | the call stack |

## Catching by type

```powershell
try { [int]::Parse('abc') }
catch [System.FormatException] { 'not a number' }
catch { "something else: $($_.Exception.GetType().FullName)" }
```

Catch clauses are checked in order; derived types match their base class (`catch [System.IO.IOException]` also catches `FileNotFoundException`). PowerShell often **wraps** exceptions (for example in `MethodInvocationException`): check `$_.Exception.InnerException` when a typed catch doesn't fire.

## finally, and trap

`finally` always runs, after success, after a caught error and even after Ctrl+C: close files, disconnect sessions, release locks there. `trap { ... }` is an older, script-wide handler; you'll see it in old scripts, but prefer `try`.

## -ErrorVariable and $Error

```powershell
Get-Item 'C:\nope-a', 'C:\nope-b' -ErrorAction SilentlyContinue -ErrorVariable problems
"collected $($problems.Count) errors"
```

`-ErrorVariable +name` appends instead of replacing. `$Error` holds every error of the session (newest first), including caught ones, so don't use it to decide whether *your* step failed.

## Writing errors like a cmdlet

In an advanced function:

- For a **per-item** problem, use `Write-Error` (or `$PSCmdlet.WriteError($record)`) and continue with the next item.
- For "this whole call can't work" (bad parameters, missing dependency), use `$PSCmdlet.ThrowTerminatingError($record)` or `throw`.

Build a proper ErrorRecord so callers can catch it by ID and category:

```powershell,norun
$ex = [System.IO.FileNotFoundException]::new("Config not found: $Path")
$record = [System.Management.Automation.ErrorRecord]::new($ex, 'ConfigMissing', 'ObjectNotFound', $Path)
$PSCmdlet.ThrowTerminatingError($record)
```

## Custom exception classes

```powershell
class QuotaExceededException : System.Exception {
    [string]$User
    QuotaExceededException([string]$user, [string]$message) : base($message) { $this.User = $user }
}
try { throw [QuotaExceededException]::new('asato', 'mailbox over 50 GB') }
catch [QuotaExceededException] { "quota problem for $($_.Exception.User): $($_.Exception.Message)" }
```

## Strict mode

`Set-StrictMode -Version Latest` turns silent mistakes into errors: reading undefined variables, non-existent properties and calling functions like methods (`f(1, 2)`). Turn it on in scripts you maintain.

@@quiz
Why doesn't `try { Get-Item missing.txt } catch { 'x' }` print `x`?
* Get-Item's "not found" is non-terminating by default; add -ErrorAction Stop to make catch see it
- try only catches .NET exceptions from methods
- catch needs a type
- Get-Item doesn't produce errors
? Non-terminating errors pass by try/catch.

@@quiz
When should an advanced function use Write-Error instead of throw?
- Never
* When one item of many fails and processing should continue with the others
- When the error is serious
- When there are no parameters
? Mirror cmdlets: per-item problems are non-terminating.

@@quiz
What does `finally` guarantee?
- That no error is shown
* The block runs whether the try succeeded, failed or was interrupted
- That the error is logged
- That the script continues
? Use it for cleanup.

@@flashcards
non-terminating error :: reported, execution continues
terminating error :: statement stops
`-ErrorAction Stop` :: make errors terminating
`$ErrorActionPreference` :: default error action
`$_.Exception` :: the .NET exception
`FullyQualifiedErrorId` :: stable error ID
`InvocationInfo.ScriptLineNumber` :: where it happened
`catch [Type]` :: catch by exception type
`InnerException` :: the wrapped exception
`-ErrorVariable` :: collect errors in a variable
`ThrowTerminatingError` :: cmdlet-style fatal error
`Set-StrictMode -Version Latest` :: catch silent mistakes

@@exercise A cmdlet-quality error contract
Write `Get-UserQuota` (an advanced function taking user names from the pipeline):

- unknown users: a **non-terminating** error with `Write-Error` using ID `UserNotFound` and category `ObjectNotFound`, then continue
- a user over quota: return the object with `Status = 'OVER'`
- if `-Limit` is 0 or less: a **terminating** error (`ThrowTerminatingError`) with ID `InvalidLimit`

Call it so that: errors are collected with `-ErrorVariable` while results flow; a `try/catch` with `-ErrorAction Stop` stops at the first unknown user; and a bad limit is caught by ID.
@@starter
$usage = @{ asato = 12; bkowalski = 61; cnguyen = 48 }
'TODO'
@@expected
asato 12 GB OK
bkowalski 61 GB OVER
cnguyen 48 GB OK
2 error(s): ghost (UserNotFound), nobody (UserNotFound)
stopped at: user 'ghost' not found
caught InvalidLimit: limit must be positive, got 0
@@requires
Write-Error
ThrowTerminatingError
-ErrorVariable
FullyQualifiedErrorId
@@hint
`Write-Error -Message "user '$name' not found" -ErrorId UserNotFound -Category ObjectNotFound -TargetObject $name`. The ID in the record is `UserNotFound,Get-UserQuota`, so compare with `-like 'UserNotFound*'` or split on the comma. For the limit check, put it in `begin { }` so it runs once.
@@solution
$usage = @{ asato = 12; bkowalski = 61; cnguyen = 48 }

function Get-UserQuota {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory, ValueFromPipeline)][string]$Name,
        [int]$Limit = 50
    )
    begin {
        if ($Limit -le 0) {
            $ex = [System.ArgumentException]::new("limit must be positive, got $Limit")
            $record = [System.Management.Automation.ErrorRecord]::new($ex, 'InvalidLimit', 'InvalidArgument', $Limit)
            $PSCmdlet.ThrowTerminatingError($record)
        }
    }
    process {
        if (-not $usage.ContainsKey($Name)) {
            Write-Error -Message "user '$Name' not found" -ErrorId UserNotFound -Category ObjectNotFound -TargetObject $Name
            return
        }
        $gb = $usage[$Name]
        [pscustomobject]@{ Name = $Name; GB = $gb; Status = $(if ($gb -gt $Limit) { 'OVER' } else { 'OK' }) }
    }
}

$names = 'asato', 'ghost', 'bkowalski', 'nobody', 'cnguyen'
$names | Get-UserQuota -ErrorAction SilentlyContinue -ErrorVariable problems |
    ForEach-Object { "$($_.Name) $($_.GB) GB $($_.Status)" }
$described = foreach ($p in $problems) { "$($p.TargetObject) ($(($p.FullyQualifiedErrorId -split ',')[0]))" }
"$($problems.Count) error(s): $($described -join ', ')"

try {
    $names | Get-UserQuota -ErrorAction Stop | Out-Null
} catch {
    "stopped at: $($_.Exception.Message)"
}

try {
    'asato' | Get-UserQuota -Limit 0
} catch {
    if ($_.FullyQualifiedErrorId -like 'InvalidLimit*') { "caught InvalidLimit: $($_.Exception.Message)" }
}

@@exercise Typed catches, wrapped exceptions and finally
Write `Read-Setting -Path -Key` that reads a `key=value` file and returns an integer setting. Handle each failure differently:

- file missing: catch `[System.IO.FileNotFoundException]` (thrown by `[System.IO.File]::ReadAllLines`); PowerShell wraps it in a `MethodInvocationException`, so unwrap with `.InnerException` and rethrow the inner one in a helper, or catch the wrapper and inspect it
- value not a number: `[int]::Parse` throws `FormatException` (also wrapped)
- key missing: throw a custom `SettingMissingException` class with a `Key` property
- always print `closed <file name>` in `finally`

Print one line per case (exception type names rather than messages, which are translated on non-English Windows), with a strict mode check at the end showing that reading an undefined variable fails.
@@starter
$dir = Join-Path ([System.IO.Path]::GetTempPath()) "settings-$PID"
New-Item -ItemType Directory -Path $dir -Force | Out-Null
Set-Content -Path (Join-Path $dir 'app.conf') -Value 'port=8080', 'workers=four'
'TODO'
Remove-Item -Recurse -Force $dir
@@expected
port = 8080
closed app.conf
missing file: app2.conf
closed app2.conf
bad number for workers: FormatException
closed app.conf
missing key: timeout
closed app.conf
strict mode: VariableIsUndefined
@@requires
class SettingMissingException
catch [System.IO.FileNotFoundException]
InnerException
finally
Set-StrictMode
@@hint
Inside `Read-Setting`, call .NET in a `try` and rethrow the inner exception: `try { $lines = [System.IO.File]::ReadAllLines($Path) } catch { throw $_.Exception.InnerException }`. Then the caller's `catch [System.IO.FileNotFoundException]` matches. Use `[System.IO.Path]::GetFileName($path)` in `finally`. For strict mode, run the check in a script block: `try { & { Set-StrictMode -Version Latest; $undefinedThing } } catch { "strict mode: $($_.FullyQualifiedErrorId)" }`. Print exception type names and error IDs rather than messages: messages are translated on non-English Windows.
@@solution
$dir = Join-Path ([System.IO.Path]::GetTempPath()) "settings-$PID"
New-Item -ItemType Directory -Path $dir -Force | Out-Null
Set-Content -Path (Join-Path $dir 'app.conf') -Value 'port=8080', 'workers=four'

class SettingMissingException : System.Exception {
    [string]$Key
    SettingMissingException([string]$key) : base("setting '$key' not found") { $this.Key = $key }
}

function Read-Setting {
    param([string]$Path, [string]$Key)
    try { $lines = [System.IO.File]::ReadAllLines($Path) } catch { throw $_.Exception.InnerException }
    foreach ($line in $lines) {
        $k, $v = $line -split '=', 2
        if ($k -eq $Key) {
            try { return [int]::Parse($v) } catch { throw $_.Exception.InnerException }
        }
    }
    throw [SettingMissingException]::new($Key)
}

$cases = @(('app.conf', 'port'), ('app2.conf', 'port'), ('app.conf', 'workers'), ('app.conf', 'timeout'))
foreach ($case in $cases) {
    $path = Join-Path $dir $case[0]
    $key = $case[1]
    try {
        "$key = $(Read-Setting -Path $path -Key $key)"
    } catch [System.IO.FileNotFoundException] {
        "missing file: $([System.IO.Path]::GetFileName($path))"
    } catch [System.FormatException] {
        "bad number for ${key}: $($_.Exception.GetType().Name)"
    } catch [SettingMissingException] {
        "missing key: $($_.Exception.Key)"
    } finally {
        "closed $([System.IO.Path]::GetFileName($path))"
    }
}

try { & { Set-StrictMode -Version Latest; $undefinedThing } } catch { "strict mode: $($_.FullyQualifiedErrorId)" }
Remove-Item -Recurse -Force $dir

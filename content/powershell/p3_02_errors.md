@@lesson p3-errors Errors: try, catch and -ErrorAction
@@concepts errors
@@text
PowerShell has two kinds of errors, and confusing them is the #1 source of "my catch block never runs".

- **Terminating** errors stop the command: `throw`, a bad .NET call, a missing mandatory parameter.
- **Non-terminating** errors are reported, and the command **continues** with the next item. `Get-ChildItem` on 100 folders where 1 is missing writes one red error and lists the other 99.

`try/catch` only catches **terminating** errors. To catch a cmdlet's non-terminating error, add `-ErrorAction Stop`:

```powershell
try {
    Get-Item -Path 'C:\does\not\exist' -ErrorAction Stop
    'found it'
}
catch {
    "Problem: $($_.Exception.Message)"
}
finally {
    'this always runs (close files, connections...)'
}
```

Inside `catch`, `$_` is the error record; `$_.Exception.Message` is the readable text.

## -ErrorAction values

| Value | Effect |
|---|---|
| `Continue` | show the error, keep going (default) |
| `SilentlyContinue` | hide the error, keep going |
| `Stop` | make it terminating, so `try/catch` sees it |
| `Ignore` | don't even record it in `$Error` |

`$ErrorActionPreference = 'Stop'` at the top of a script makes every error terminating, a good default for automation scripts that shouldn't keep going after something fails.

## Throwing your own errors

```powershell
function Set-Quota {
    param([int]$GB)
    if ($GB -le 0) { throw "Quota must be positive, got $GB" }
    "Quota set to $GB GB"
}
try { Set-Quota -GB 0 } catch { "Refused: $($_.Exception.Message)" }
```

## Catching specific errors

```powershell
try {
    [int]'abc'
}
catch [System.Management.Automation.RuntimeException] {
    'not a number'
}
catch {
    'something else went wrong'
}
```

## Exit codes for schedulers

Task Scheduler and monitoring tools look at a script's **exit code**: `exit 0` means success, anything else means failure. Native programs set `$LASTEXITCODE`, which you should check after running `robocopy`, `git` or `msiexec`.

@@quiz
`try { Get-ChildItem C:\nope } catch { 'caught' }` prints a red error but not "caught". Why?
* `Get-ChildItem` raised a non-terminating error; add `-ErrorAction Stop`
- `catch` needs a type
- The path must be quoted
- try/catch doesn't work in PowerShell 5.1
? Only terminating errors reach `catch`.

@@quiz
Inside a `catch` block, how do you get the error message?
- `$Error.Text`
* `$_.Exception.Message`
- `$LASTEXITCODE`
- `$catch.Message`
? `$_` is the ErrorRecord; its `.Exception.Message` is the text.

@@flashcards
terminating error :: Stops the command; caught by try/catch
non-terminating error :: Reported, but the command carries on with the next item
-ErrorAction Stop :: Turns a non-terminating error into a terminating one for try/catch
$ErrorActionPreference :: The default -ErrorAction for the whole script
$LASTEXITCODE :: Exit code of the last native program (robocopy, git, ...)
finally :: A block that always runs, error or not

@@exercise Safe conversion
Write `ConvertTo-Port` that converts text to a port number. Return the number if it's a whole number from 1 to 65535. Otherwise **throw** an error whose message starts with `Invalid port`. The tests catch it for you.
@@starter
function ConvertTo-Port($text) {
    throw "TODO: not done yet"
}

@@tests
check (ConvertTo-Port '443') 443 "ConvertTo-Port '443'"
foreach ($bad in 'abc', '0', '70000', '') {
    try { $r = ConvertTo-Port $bad; check $r 'an error' "ConvertTo-Port '$bad' should throw" }
    catch { check ($_.Exception.Message -like 'Invalid port*') $true "message for '$bad' should start with 'Invalid port' (was: $($_.Exception.Message))" }
}
@@hint
`$n = 0; if (-not [int]::TryParse($text, [ref]$n) -or $n -lt 1 -or $n -gt 65535) { throw "Invalid port: '$text'" }`, then `return $n`.
@@solution
function ConvertTo-Port($text) {
    $n = 0
    if (-not [int]::TryParse($text, [ref]$n) -or $n -lt 1 -or $n -gt 65535) {
        throw "Invalid port: '$text'"
    }
    return $n
}

@@exercise Keep going
`$paths` holds paths, some of which don't exist. For each one print `OK <path>` if `Get-Item` finds it, or `MISSING <path>` if it fails. Use `try/catch` with `-ErrorAction Stop`; the script must not stop at the first missing path.
@@starter
$paths = @('C:\Windows', 'C:\no\such\folder', 'C:\Windows\System32', 'Z:\nope')

@@expected
OK C:\Windows
MISSING C:\no\such\folder
OK C:\Windows\System32
MISSING Z:\nope
@@requires
-ErrorAction Stop
@@hint
`foreach ($p in $paths) { try { Get-Item $p -ErrorAction Stop | Out-Null; Write-Output "OK $p" } catch { Write-Output "MISSING $p" } }`
@@solution
$paths = @('C:\Windows', 'C:\no\such\folder', 'C:\Windows\System32', 'Z:\nope')
foreach ($p in $paths) {
    try {
        Get-Item -Path $p -ErrorAction Stop | Out-Null
        Write-Output "OK $p"
    }
    catch {
        Write-Output "MISSING $p"
    }
}

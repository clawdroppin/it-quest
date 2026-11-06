@@lesson n8-ps7 PowerShell 7 and Cross-Platform Scripts
@@concepts strings, files
@@text
There are two PowerShells:

| | Windows PowerShell 5.1 | PowerShell 7.x |
|---|---|---|
| Executable | `powershell.exe` | `pwsh` |
| Runs on | Windows only | Windows, Linux, macOS |
| .NET | .NET Framework 4.x | modern .NET |
| Status | built into Windows, maintenance only | actively developed, installed separately |
| Default file encoding | mixed (often UTF-16 or ANSI) | UTF-8 without BOM |

Both can be installed side by side. New scripts should run on 7; many admin modules (older AD, Exchange, some RSAT) still need 5.1, and 7 can load them through a compatibility session.

## What PowerShell 7 adds

```powershell,norun
$name = $env:USERNAME ?? 'unknown'                # null-coalescing
$label = $disk.Free -lt 10GB ? 'LOW' : 'OK'       # ternary
Get-Item config.json && Write-Host 'found'        # pipeline chain operators && ||
$servers | ForEach-Object -Parallel { Test-Connection $_ -Count 1 } -ThrottleLimit 20
$user?.Manager?.Name                              # null-conditional member access
Get-Content log.txt | Select-String error -Raw
ConvertFrom-Json -AsHashtable; Invoke-RestMethod -Retry...; Get-Error
```

This app runs Windows PowerShell 5.1, so the exercises stay with 5.1 syntax. To support both, check the version:

```powershell
if ($PSVersionTable.PSVersion.Major -ge 7) { 'modern' } else { "Windows PowerShell $($PSVersionTable.PSVersion.Major).$($PSVersionTable.PSVersion.Minor)" }
"edition: $($PSVersionTable.PSEdition)"
```

`#Requires -Version 7.2` at the top of a script refuses to run it on older versions with a clear message.

## Writing scripts that run everywhere

- **Paths**: use `Join-Path` and `[System.IO.Path]` methods, never hard-coded `\`. `$IsWindows`, `$IsLinux`, `$IsMacOS` exist in 7 (they're `$null` in 5.1, so test `$PSVersionTable.PSEdition -eq 'Desktop'` first).
- **Case**: Linux file systems are case-sensitive: `./Config.json` and `./config.json` are different files.
- **Aliases**: on Linux, `ls`, `cat`, `sort`, `curl` run the native programs, not PowerShell cmdlets. Use full cmdlet names in scripts.
- **Line endings and encoding**: write files with an explicit encoding. In 5.1, `-Encoding UTF8` writes a **BOM**; for BOM-less UTF-8 use .NET: `[System.IO.File]::WriteAllText($path, $text, [System.Text.UTF8Encoding]::new($false))`.
- **Environment**: `$env:HOME` vs `$env:USERPROFILE`, `$env:TEMP` vs `/tmp` (use `[System.IO.Path]::GetTempPath()`).
- **Windows-only features**: the registry, CIM/WMI, services cmdlets, WinForms/WPF, event logs.

## Encoding pitfalls in 5.1

| Command | Default output encoding in 5.1 |
|---|---|
| `Out-File`, `>` | UTF-16 LE (!) |
| `Set-Content`, `Add-Content` | system ANSI code page |
| `Export-Csv` | ASCII (non-ASCII becomes `?`) |

Always pass `-Encoding UTF8` (or use .NET) when other programs, Git or Linux machines will read the file. In PowerShell 7, everything defaults to UTF-8 without BOM.

@@quiz
Which statement about PowerShell 5.1 and 7 is true?
- 7 replaces 5.1 when installed
* They install side by side: powershell.exe is 5.1, pwsh is 7
- 5.1 runs on Linux
- 7 can't run on Windows
? Windows still ships 5.1.

@@quiz
In Windows PowerShell 5.1, what encoding does `"text" > file.txt` produce?
- UTF-8
* UTF-16 LE
- ASCII
- It depends on the file name
? Use Set-Content/Out-File -Encoding UTF8 or .NET.

@@quiz
Why avoid `ls` and `curl` in cross-platform scripts?
* On Linux and macOS they run the native programs, not the PowerShell cmdlets
- They're deprecated
- They're slower
- They need admin rights
? Use Get-ChildItem and Invoke-WebRequest.

@@drill
Which automatic variable is `$true` when PowerShell 7 runs on Linux?
= $IsLinux
? $IsWindows and $IsMacOS exist too.

@@flashcards
`powershell.exe` :: Windows PowerShell 5.1
`pwsh` :: PowerShell 7
`??` / `? :` :: PS 7 null-coalescing / ternary
`&&` / `||` :: PS 7 pipeline chain operators
`-Parallel` :: PS 7 ForEach-Object
`$PSVersionTable.PSEdition` :: Desktop (5.1) or Core (6+)
`#Requires -Version 7.2` :: minimum version
`Join-Path` :: portable paths
`$IsLinux` :: platform check (PS 7)
`>` in 5.1 :: writes UTF-16
`UTF8Encoding($false)` :: UTF-8 without BOM

@@exercise Encodings, byte by byte
In a temp folder, write the text `Zażółć Łódź 日本` four ways and show the first bytes and size of each file:

1. `Out-File` (5.1 default)
2. `Set-Content -Encoding UTF8`
3. `.NET WriteAllText` with `UTF8Encoding($false)`
4. `Set-Content -Encoding Default` (ANSI): characters outside the code page become `?`

Then read each file back with `[System.IO.File]::ReadAllText` (which detects BOMs) and print whether the text survived.
@@starter
$text = 'Zażółć Łódź 日本'
$dir = Join-Path ([System.IO.Path]::GetTempPath()) "enc-$PID"
New-Item -ItemType Directory -Path $dir -Force | Out-Null
'TODO'
Remove-Item -Recurse -Force $dir
@@expected
out-file: FF FE 5A 00 ... (UTF-16 LE with BOM), survived: True
utf8-bom: EF BB BF 5A ... (UTF-8 with BOM), survived: True
utf8-nobom: 5A 61 C5 BC ... (UTF-8 without BOM), survived: True
ansi: survived: False
@@requires
Out-File
UTF8Encoding
ReadAllBytes
@@hint
`$bytes = [System.IO.File]::ReadAllBytes($path)`; show the first four as hex: `($bytes[0..3] | ForEach-Object { $_.ToString('X2') }) -join ' '`. Identify the encoding from the start: `FF FE` → UTF-16 LE, `EF BB BF` → UTF-8 BOM, otherwise none. `Out-File` and `Set-Content` add a trailing newline, so compare with `.TrimEnd()`. The ANSI result depends on the system code page, so only print whether it survived (it can't hold both Polish and Japanese characters in any single-byte code page).
@@solution
$text = 'Zażółć Łódź 日本'
$dir = Join-Path ([System.IO.Path]::GetTempPath()) "enc-$PID"
New-Item -ItemType Directory -Path $dir -Force | Out-Null

$files = [ordered]@{
    'out-file'   = Join-Path $dir 'a.txt'
    'utf8-bom'   = Join-Path $dir 'b.txt'
    'utf8-nobom' = Join-Path $dir 'c.txt'
    'ansi'       = Join-Path $dir 'd.txt'
}
$text | Out-File -FilePath $files['out-file']
Set-Content -Path $files['utf8-bom'] -Value $text -Encoding UTF8
[System.IO.File]::WriteAllText($files['utf8-nobom'], $text, [System.Text.UTF8Encoding]::new($false))
Set-Content -Path $files['ansi'] -Value $text -Encoding Default

foreach ($name in $files.Keys) {
    $path = $files[$name]
    $survived = [System.IO.File]::ReadAllText($path).TrimEnd() -ceq $text
    if ($name -eq 'ansi') { "${name}: survived: $survived"; continue }
    $bytes = [System.IO.File]::ReadAllBytes($path)
    $hex = ($bytes[0..3] | ForEach-Object { $_.ToString('X2') }) -join ' '
    if ($bytes[0] -eq 0xFF -and $bytes[1] -eq 0xFE) { $kind = 'UTF-16 LE with BOM' }
    elseif ($bytes[0] -eq 0xEF -and $bytes[1] -eq 0xBB) { $kind = 'UTF-8 with BOM' }
    else { $kind = 'UTF-8 without BOM' }
    "${name}: $hex ... ($kind), survived: $survived"
}
Remove-Item -Recurse -Force $dir

@@exercise A portable helper module
Write functions that behave the same on 5.1 and 7, on Windows and Linux:

- `Get-Platform`: `Windows` / `Linux` / `macOS`, using `$IsLinux`/`$IsMacOS` only when `PSEdition` is `Core`
- `Get-ConfigPath -App NAME`: `%APPDATA%\NAME\config.json` on Windows, `~/.config/NAME/config.json` elsewhere (build with `Join-Path`, which uses the right separator)
- `Write-Utf8File -Path -Text`: UTF-8 without BOM, `\n` line endings, on any version
- `Get-Coalesce`: a 5.1-compatible replacement for `??` that returns the first non-null argument

Show results (for the config path, only show the tail `NAME\config.json` or `NAME/config.json` portion, since home folders differ).
@@starter
'TODO'
@@expected
platform: Windows
edition: Desktop
config tail: itquest\config.json
file bytes: 8, starts with BOM: False, CRLF present: False
coalesce: fallback
coalesce with value: 0
@@requires
PSEdition
Join-Path
UTF8Encoding
@@hint
In 5.1, `$IsWindows` doesn't exist (it's `$null`), so: `if ($PSVersionTable.PSEdition -eq 'Desktop' -or $IsWindows) { 'Windows' } elseif ($IsMacOS) { 'macOS' } else { 'Linux' }`. Normalise line endings with `$Text -replace "`r`n", "`n"`. `Get-Coalesce` loops over `$args` and returns the first `$null -ne $a` (note: `0` is not null, so it's returned).
@@solution
function Get-Platform {
    if ($PSVersionTable.PSEdition -eq 'Desktop' -or $IsWindows) { return 'Windows' }
    if ($IsMacOS) { return 'macOS' }
    return 'Linux'
}

function Get-ConfigPath([string]$App) {
    if ((Get-Platform) -eq 'Windows') {
        $base = $env:APPDATA
    } else {
        $base = Join-Path $HOME '.config'
    }
    Join-Path (Join-Path $base $App) 'config.json'
}

function Write-Utf8File([string]$Path, [string]$Text) {
    $normalised = $Text -replace "`r`n", "`n"
    [System.IO.File]::WriteAllText($Path, $normalised, [System.Text.UTF8Encoding]::new($false))
}

function Get-Coalesce {
    foreach ($a in $args) { if ($null -ne $a) { return $a } }
}

"platform: $(Get-Platform)"
"edition: $($PSVersionTable.PSEdition)"
$path = Get-ConfigPath -App 'itquest'
$tail = $path.Substring($path.Length - 'itquest\config.json'.Length)
"config tail: $tail"

$file = Join-Path ([System.IO.Path]::GetTempPath()) "portable-$PID.txt"
Write-Utf8File -Path $file -Text "one`r`ntwo`r`n"
$bytes = [System.IO.File]::ReadAllBytes($file)
$hasBom = $bytes.Length -ge 3 -and $bytes[0] -eq 0xEF
$hasCrlf = $bytes -contains 13
"file bytes: $($bytes.Length), starts with BOM: $hasBom, CRLF present: $hasCrlf"
Remove-Item $file

$missing = $null
"coalesce: $(Get-Coalesce $missing $null 'fallback')"
"coalesce with value: $(Get-Coalesce $null 0 'fallback')"

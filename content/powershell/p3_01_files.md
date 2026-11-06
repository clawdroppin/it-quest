@@lesson p3-files Files, CSV and JSON
@@concepts files, json
@@text
## Paths and items

```powershell
$dir = Join-Path $env:TEMP 'itquest-demo'          # builds paths safely
New-Item -ItemType Directory -Path $dir -Force | Out-Null
Set-Content -Path (Join-Path $dir 'servers.txt') -Value 'web01', 'web02', 'db01'
Add-Content -Path (Join-Path $dir 'servers.txt') -Value 'mail01'
Get-Content -Path (Join-Path $dir 'servers.txt')
Test-Path (Join-Path $dir 'servers.txt')
Get-ChildItem -Path $dir | Select-Object Name, Length
Remove-Item -Path $dir -Recurse -Force
```

| Cmdlet | Does | Alias |
|---|---|---|
| `Get-ChildItem` | list a folder (`-Recurse`, `-Filter *.log`, `-File`) | `ls`, `dir` |
| `Get-Content` / `Set-Content` / `Add-Content` | read, overwrite, append text | `cat` |
| `New-Item`, `Copy-Item`, `Move-Item`, `Remove-Item`, `Rename-Item` | manage files and folders | `cp`, `mv`, `rm` |
| `Test-Path` | does it exist? | |
| `Join-Path`, `Split-Path` | build and take apart paths | |

`Out-Null` throws away output you don't need (like the object `New-Item` returns).

## Finding big and old files

```powershell,norun
# The 10 biggest files under a folder:
Get-ChildItem C:\Users -Recurse -File -ErrorAction SilentlyContinue |
    Sort-Object Length -Descending | Select-Object -First 10 FullName, @{ n = 'MB'; e = { [math]::Round($_.Length / 1MB) } }

# Logs older than 30 days, and delete them (try -WhatIf first!):
Get-ChildItem C:\Logs -Filter *.log | Where-Object LastWriteTime -lt (Get-Date).AddDays(-30) | Remove-Item -WhatIf
```

`-WhatIf` shows what *would* happen without doing it. Use it every time before deleting in bulk.

## CSV: the admin's favourite format

HR sends a spreadsheet of new hires; you export a list of computers for a manager. CSV is the bridge.

```powershell
$csv = @'
Name,Dept,Laptop
Ana Silva,Sales,LT-0042
Kenji Tanaka,IT,LT-0107
'@
$people = $csv | ConvertFrom-Csv
$people | ForEach-Object { "$($_.Name) gets $($_.Laptop)" }
```

With files: `Import-Csv users.csv` and `$objects | Export-Csv out.csv -NoTypeInformation -Encoding UTF8`. Every column becomes a property, **as a string** (cast numbers with `[int]`).

## JSON: APIs and config files

```powershell
$json = '{ "server": "web01", "ports": [80, 443], "tls": true }'
$cfg = $json | ConvertFrom-Json
$cfg.server
$cfg.ports -join ','
$cfg.tls = $false
$cfg | ConvertTo-Json -Compress
```

`ConvertTo-Json` only goes **2 levels deep** by default in 5.1. For nested data, add `-Depth 5`.

@@quiz
You want to delete old logs, but check first what would be deleted. Which parameter?
* `-WhatIf`
- `-Confirm:$false`
- `-Test`
- `-DryRun`
? Cmdlets that change things support `-WhatIf` (show only) and `-Confirm` (ask each time).

@@quiz
After `Import-Csv`, what type is each column's value?
- whatever type the data looks like
* a string
- an int if it's a number
- a hashtable
? CSV has no types. Cast with `[int]$_.Age` when you need numbers.

@@quiz
Which command turns JSON text into objects?
* ConvertFrom-Json
- Import-Csv
- ConvertTo-Json
- Get-Content -AsObject
? ConvertTo-Json goes the other way; use -Depth for nested objects.

@@drill
Which cmdlet reads a CSV file into objects?
= Import-Csv
? Each column becomes a property.

@@flashcards
Join-Path :: Safely combine folder and file names into a path
Test-Path :: True if a file, folder or registry key exists
Import-Csv / Export-Csv :: Read CSV rows as objects / write objects as CSV (use -NoTypeInformation in 5.1)
ConvertFrom-Json :: Turn JSON text into objects
ConvertTo-Json :: Turn objects into JSON; use -Depth for nested data
-WhatIf :: Show what a command would do, without doing it
Out-Null :: Discard output

@@exercise New hire laptops
Parse the CSV in `$csv` and print, for **IT** department people only, `Name -> Laptop`.
@@starter
$csv = @'
Name,Dept,Laptop
Ana Silva,Sales,LT-0042
Kenji Tanaka,IT,LT-0107
Mia Lee,IT,LT-0108
Ben Jones,Finance,LT-0090
'@

@@expected
Kenji Tanaka -> LT-0107
Mia Lee -> LT-0108
@@hint
`$csv | ConvertFrom-Csv | Where-Object Dept -eq 'IT' | ForEach-Object { ... }`
@@solution
$csv = @'
Name,Dept,Laptop
Ana Silva,Sales,LT-0042
Kenji Tanaka,IT,LT-0107
Mia Lee,IT,LT-0108
Ben Jones,Finance,LT-0090
'@
$csv | ConvertFrom-Csv | Where-Object Dept -eq 'IT' | ForEach-Object { "$($_.Name) -> $($_.Laptop)" }

@@exercise Config check
`$json` is a server config. Print one line per **site** whose `https` is `false`: `WARNING: <name> has no HTTPS`. Then print the number of sites as `Sites: N`.
@@starter
$json = @'
{
  "server": "web01",
  "sites": [
    { "name": "intranet", "https": true },
    { "name": "old-wiki", "https": false },
    { "name": "shop", "https": true },
    { "name": "test", "https": false }
  ]
}
'@

@@expected
WARNING: old-wiki has no HTTPS
WARNING: test has no HTTPS
Sites: 4
@@hint
`$cfg = $json | ConvertFrom-Json`, then `$cfg.sites | Where-Object { -not $_.https }`.
@@solution
$json = @'
{
  "server": "web01",
  "sites": [
    { "name": "intranet", "https": true },
    { "name": "old-wiki", "https": false },
    { "name": "shop", "https": true },
    { "name": "test", "https": false }
  ]
}
'@
$cfg = $json | ConvertFrom-Json
$cfg.sites | Where-Object { -not $_.https } | ForEach-Object { "WARNING: $($_.name) has no HTTPS" }
"Sites: $(@($cfg.sites).Count)"

@@exercise A real folder
Write `Get-FolderReport` that takes a `-Path` and returns `"<count> files, <total> bytes"` for the files directly in that folder (not subfolders). The test creates a temporary folder for you.
@@starter
function Get-FolderReport {
    param([string]$Path)
    throw "TODO: not done yet"
}

@@tests
$t = Join-Path $env:TEMP "itq-folder-$PID"
New-Item -ItemType Directory -Path (Join-Path $t 'sub') -Force | Out-Null
Set-Content -Path (Join-Path $t 'a.txt') -Value 'hello' -NoNewline
Set-Content -Path (Join-Path $t 'b.txt') -Value 'abc' -NoNewline
Set-Content -Path (Join-Path $t 'sub\c.txt') -Value 'not counted' -NoNewline
check (Get-FolderReport -Path $t) '2 files, 8 bytes' "Get-FolderReport (2 files of 5 and 3 bytes)"
Remove-Item $t -Recurse -Force
@@hint
`$files = @(Get-ChildItem -Path $Path -File)`, then `($files | Measure-Object Length -Sum).Sum`. The `@()` keeps `.Count` right.
@@solution
function Get-FolderReport {
    param([string]$Path)
    $files = @(Get-ChildItem -Path $Path -File)
    $total = ($files | Measure-Object Length -Sum).Sum
    return "$($files.Count) files, $total bytes"
}

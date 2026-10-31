@@lesson p1-variables Variables, Types and Math
@@concepts variables, numbers
@@text
A variable name starts with `$`. You don't declare a type; PowerShell figures it out:

```powershell
$server = "WEB01"          # string
$cpuCount = 8              # int
$loadAverage = 0.73        # double
$isOnline = $true          # bool: $true / $false
$nothing = $null           # no value
Write-Output "$server has $cpuCount CPUs"
$cpuCount.GetType().Name
```

Variable names are **not case-sensitive**: `$Server` and `$server` are the same variable.

## Math, and handy size units

```powershell
10 + 3      # 13
10 / 4      # 2.5   (division gives a decimal when needed)
10 % 3      # 1     (remainder)
[math]::Round(2.567, 1)
[int](7 / 2)   # 4: casting to int rounds (to even on .5), it doesn't cut
[math]::Floor(7 / 2)
1GB / 1MB   # 1024: PowerShell knows KB, MB, GB, TB, PB
```

Size suffixes are an admin's best friend: `(Get-Item file.iso).Length / 1GB` gives the size in gigabytes.

## Casting types

Text that looks like a number is still text until you convert it:

```powershell
$a = "40"
$a + 2          # "402": string + makes a longer string
[int]$a + 2     # 42
2 + $a          # 42: the LEFT side decides the type!
```

You can also fix a variable's type: `[int]$port = "443"` stores the number 443, and assigning `"abc"` later would be an error.

## Automatic variables worth knowing

| Variable | Holds |
|---|---|
| `$true`, `$false`, `$null` | the booleans and "nothing" |
| `$env:COMPUTERNAME`, `$env:USERNAME` | environment variables |
| `$PSVersionTable` | PowerShell version info |
| `$HOME` | your user profile folder |
| `$?` | did the last command succeed? |

```powershell
"I am $env:USERNAME on $env:COMPUTERNAME"
```

@@quiz
What does `"5" + 5` give in PowerShell?
* `55` (a string)
- `10`
- an error
- `5`
? The left side is a string, so `+` joins strings. `5 + "5"` would give 10.

@@quiz
What is `1GB / 1MB`?
- 1000
* 1024
- 1
- an error, GB isn't a number
? PowerShell's size suffixes are binary: 1 KB = 1024 bytes, 1 MB = 1024 KB, 1 GB = 1024 MB.

@@drill
Which variable holds the current computer's name? (type the full variable)
= $env:COMPUTERNAME
= $env:computername
? Environment variables live on the `env:` drive, so `$env:COMPUTERNAME`.

@@exercise Disk usage percent
`$sizeBytes` and `$freeBytes` describe a disk. Print the **used** space (size minus free) as a percentage of the size, rounded to **one decimal place**.

For the given numbers, print exactly `Used: 91.9%`.
@@starter
$sizeBytes = 512GB
$freeBytes = 41.5GB

@@expected
Used: 91.9%
@@hint
`$used = ($sizeBytes - $freeBytes) / $sizeBytes * 100`, then `[math]::Round($used, 1)` inside the string with `$( )`.
@@solution
$sizeBytes = 512GB
$freeBytes = 41.5GB
$used = ($sizeBytes - $freeBytes) / $sizeBytes * 100
Write-Output "Used: $([math]::Round($used, 1))%"

@@exercise Convert units
Write a function `ConvertTo-GB` that takes a number of **bytes** and returns it in **gigabytes**, rounded to 2 decimals.
@@starter
function ConvertTo-GB($bytes) {
    throw "TODO: not done yet"
}

@@tests
check (ConvertTo-GB 1073741824) 1 "ConvertTo-GB 1073741824"
check (ConvertTo-GB 5368709120) 5 "ConvertTo-GB 5368709120"
check (ConvertTo-GB 1500000000) 1.4 "ConvertTo-GB 1500000000"
check (ConvertTo-GB 0) 0 "ConvertTo-GB 0"
@@hint
`[math]::Round($bytes / 1GB, 2)`. A function's result is whatever it outputs; `return` works too.
@@solution
function ConvertTo-GB($bytes) {
    return [math]::Round($bytes / 1GB, 2)
}

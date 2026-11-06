@@lesson p2-pipeline-deep The Pipeline in Depth: Binding, Select, Group, Compare
@@concepts objects, lists
@@text
You know the pipeline passes objects. This lesson covers **how** a command decides what to do with each incoming object, and the reshaping cmdlets that turn raw objects into answers.

## Parameter binding: ByValue and ByPropertyName

When an object arrives at a command, PowerShell looks for a parameter to bind it to:

1. **ByValue**: a parameter that accepts the whole object's type (`Stop-Process -InputObject <Process>`).
2. **ByPropertyName**: a parameter whose **name matches a property** of the object (`Get-Service -Name` takes any object with a `Name` property).

That's why this works without a loop:

```powershell,norun
Import-Csv servers.csv | Test-Connection -Count 1     # CSV has a ComputerName column
[pscustomobject]@{ Name = 'Spooler' } | Get-Service
```

`Get-Help Get-Service -Parameter Name` shows `Accept pipeline input? True (ByPropertyName, ByValue)`. Your own functions get the same power with `[Parameter(ValueFromPipelineByPropertyName)]`. If a column has the wrong name, rename it on the fly with a calculated property: `Select-Object @{ n = 'ComputerName'; e = { $_.Host } }`.

## Select-Object: shape, not just choose

```powershell
$procs = @(
    [pscustomobject]@{ Name = 'chrome'; WS = 812MB; Id = 4120 }
    [pscustomobject]@{ Name = 'code';   WS = 640MB; Id = 2210 }
    [pscustomobject]@{ Name = 'svchost'; WS = 40MB; Id = 980 }
)
$procs | Select-Object Name, @{ Name = 'MB'; Expression = { [int]($_.WS / 1MB) } }
$procs | Select-Object -First 2 -ExpandProperty Name     # plain strings, not objects
$procs | Select-Object -Property * -ExcludeProperty Id
```

`-ExpandProperty` is the one people miss: `Select-Object Name` gives objects **with** a Name property; `-ExpandProperty Name` gives the **strings** themselves (like `.Name` on the collection).

## Group-Object and Measure-Object

```powershell
$events = 'error', 'warn', 'error', 'info', 'error'
$events | Group-Object | Sort-Object Count -Descending | Select-Object Count, Name
$groups = $events | Group-Object -AsHashTable -AsString
$groups['error'].Count
1, 5, 9 | Measure-Object -Sum -Average -Maximum | Select-Object Sum, Average, Maximum
```

`Group-Object` returns objects with `Name`, `Count` and `Group` (the items). Group by a **script block** for computed keys: `Group-Object { $_.Length -gt 100MB }`.

## Compare-Object: what changed?

```powershell
$before = 'web01', 'web02', 'db01'
$after  = 'web01', 'db01', 'db02'
Compare-Object $before $after
#  <=  only in the first (reference) set,  =>  only in the second (difference) set
```

Use `-Property` to compare objects by specific properties, and `-IncludeEqual` to see the unchanged ones too.

## ForEach-Object -Begin / -Process / -End, and Tee-Object

```powershell
1..4 | ForEach-Object -Begin { $sum = 0 } -Process { $sum += $_ } -End { "total $sum" }
1..3 | Tee-Object -Variable seen | Measure-Object | Select-Object -ExpandProperty Count
"seen: $($seen -join ',')"
```

## Streaming vs collecting

The pipeline is **streaming**: the first object reaches the next command before the last one is produced, so `Get-ChildItem -Recurse | Select-Object -First 5` stops early. `Sort-Object` and `Group-Object` must collect everything first. Wrapping in parentheses `(Get-ChildItem)` also collects everything before continuing.

@@quiz
`Get-Service` has a `-Name` parameter that accepts pipeline input ByPropertyName. What can you pipe into it?
* Any object with a `Name` property, like rows from a CSV with a Name column
- Only ServiceController objects
- Only strings
- Nothing; you must use a loop
? The property name must match the parameter name (or an alias).

@@quiz
What's the difference between `Select-Object Name` and `Select-Object -ExpandProperty Name`?
- None
* The first gives objects with a Name property; the second gives the name values themselves
- The second sorts by name
- The first is faster
? Use -ExpandProperty when the next step needs plain values.

@@quiz
In `Compare-Object $a $b`, what does `=>` mean?
- The item is in both
- The item is only in $a
* The item is only in $b (the difference object)
- The item changed
? `<=` points at the reference set.

@@drill
Which cmdlet shows the differences between two sets of objects?
= Compare-Object
= compare
= diff
? <= and => show which side each item is on.

@@flashcards
ByValue :: binds the whole object by type
ByPropertyName :: binds a property with the parameter's name
calculated property :: `@{ n = 'X'; e = { ... } }`
`-ExpandProperty` :: unwrap to values
`Group-Object` :: Name, Count, Group
`-AsHashTable` :: groups as a lookup table
`Measure-Object -Sum -Average` :: statistics
`Compare-Object` :: `<=` only left, `=>` only right
`-Begin/-Process/-End` :: ForEach-Object phases
`Tee-Object -Variable` :: save and pass along
`Sort-Object` :: blocks streaming

@@exercise Reshape an inventory
From the inventory objects:

- print a table-free report of each server with a calculated `RAM` property in GB (`MemMB / 1024`, rounded to a whole number) using `Select-Object` and calculated properties, as `name: RAM GB, os`
- group the servers by OS, sorted by count (highest first, ties by name), printing `count x os: names`
- print the total, average and maximum memory in GB with `Measure-Object` (average as a whole number)
- get just the names of Linux servers as strings with `-ExpandProperty`
@@starter
$inventory = @(
    [pscustomobject]@{ Name = 'web01'; Os = 'Linux';   MemMB = 8192 }
    [pscustomobject]@{ Name = 'web02'; Os = 'Linux';   MemMB = 8192 }
    [pscustomobject]@{ Name = 'dc01';  Os = 'Windows'; MemMB = 16384 }
    [pscustomobject]@{ Name = 'db01';  Os = 'Linux';   MemMB = 65536 }
    [pscustomobject]@{ Name = 'rds01'; Os = 'Windows'; MemMB = 32768 }
    [pscustomobject]@{ Name = 'fw01';  Os = 'BSD';     MemMB = 4096 }
)
'TODO'
@@expected
web01: 8 GB, Linux
web02: 8 GB, Linux
dc01: 16 GB, Windows
db01: 64 GB, Linux
rds01: 32 GB, Windows
fw01: 4 GB, BSD
3 x Linux: web01, web02, db01
2 x Windows: dc01, rds01
1 x BSD: fw01
total 132 GB, average 22 GB, max 64 GB
linux names: web01 web02 db01
@@requires
Select-Object
Group-Object
Measure-Object
-ExpandProperty
@@hint
`$withRam = $inventory | Select-Object Name, Os, @{ n = 'RAM'; e = { [int]($_.MemMB / 1024) } }`. Group: `$withRam | Group-Object Os | Sort-Object @{ e = 'Count'; Descending = $true }, Name`. Each group's items are in `.Group`, so the names are `$g.Group.Name -join ', '`. Measure the RAM property: `$stats = $withRam | Measure-Object RAM -Sum -Average -Maximum`.
@@solution
$inventory = @(
    [pscustomobject]@{ Name = 'web01'; Os = 'Linux';   MemMB = 8192 }
    [pscustomobject]@{ Name = 'web02'; Os = 'Linux';   MemMB = 8192 }
    [pscustomobject]@{ Name = 'dc01';  Os = 'Windows'; MemMB = 16384 }
    [pscustomobject]@{ Name = 'db01';  Os = 'Linux';   MemMB = 65536 }
    [pscustomobject]@{ Name = 'rds01'; Os = 'Windows'; MemMB = 32768 }
    [pscustomobject]@{ Name = 'fw01';  Os = 'BSD';     MemMB = 4096 }
)
$withRam = $inventory | Select-Object Name, Os, @{ Name = 'RAM'; Expression = { [int]($_.MemMB / 1024) } }
foreach ($s in $withRam) { "$($s.Name): $($s.RAM) GB, $($s.Os)" }

$withRam | Group-Object Os | Sort-Object @{ Expression = 'Count'; Descending = $true }, Name | ForEach-Object {
    "$($_.Count) x $($_.Name): $($_.Group.Name -join ', ')"
}

$stats = $withRam | Measure-Object RAM -Sum -Average -Maximum
"total $($stats.Sum) GB, average $([int]$stats.Average) GB, max $($stats.Maximum) GB"
$linux = $withRam | Where-Object Os -eq 'Linux' | Select-Object -ExpandProperty Name
"linux names: $linux"

@@exercise Binding by property name, and Compare-Object
1. Write `Get-Port` with a `ComputerName` parameter bound `ValueFromPipelineByPropertyName` (alias `Host`) and a `Port` parameter, returning `name:port`. Pipe in objects that use `ComputerName` and others that use `Host`.
2. Compare yesterday's and today's local administrators with `Compare-Object` and print `added: ...` and `removed: ...`
3. Compare two lists of objects by `Name` and `Version` to find software whose version changed (both `<=` and `=>` rows exist for the same name)
@@starter
$targets = @(
    [pscustomobject]@{ ComputerName = 'web01'; Port = 443 }
    [pscustomobject]@{ Host = 'db01'; Port = 5432 }
)
$yesterday = 'Administrator', 'it-admin', 'jsmith'
$today = 'Administrator', 'it-admin', 'temp-contractor'
$old = @(
    [pscustomobject]@{ Name = '7-Zip'; Version = '24.08' }
    [pscustomobject]@{ Name = 'Git';   Version = '2.47.0' }
    [pscustomobject]@{ Name = 'VLC';   Version = '3.0.21' }
)
$new = @(
    [pscustomobject]@{ Name = '7-Zip'; Version = '24.09' }
    [pscustomobject]@{ Name = 'Git';   Version = '2.47.0' }
    [pscustomobject]@{ Name = 'Zoom';  Version = '6.2.5' }
)
'TODO'
@@expected
web01:443
db01:5432
added: temp-contractor
removed: jsmith
updated: 7-Zip 24.08 -> 24.09
uninstalled: VLC 3.0.21
installed: Zoom 6.2.5
@@requires
ValueFromPipelineByPropertyName
Compare-Object
Alias(
@@hint
`function Get-Port { param([Parameter(ValueFromPipelineByPropertyName)][Alias('Host')][string]$ComputerName, [Parameter(ValueFromPipelineByPropertyName)][int]$Port) process { "${ComputerName}:$Port" } }`. For the software, `Compare-Object $old $new -Property Name, Version`, then group the results by Name: a group with 2 rows is an update; one `=>` row is an install; one `<=` row is an uninstall. Sort the groups by name.
@@solution
$targets = @(
    [pscustomobject]@{ ComputerName = 'web01'; Port = 443 }
    [pscustomobject]@{ Host = 'db01'; Port = 5432 }
)
$yesterday = 'Administrator', 'it-admin', 'jsmith'
$today = 'Administrator', 'it-admin', 'temp-contractor'
$old = @(
    [pscustomobject]@{ Name = '7-Zip'; Version = '24.08' }
    [pscustomobject]@{ Name = 'Git';   Version = '2.47.0' }
    [pscustomobject]@{ Name = 'VLC';   Version = '3.0.21' }
)
$new = @(
    [pscustomobject]@{ Name = '7-Zip'; Version = '24.09' }
    [pscustomobject]@{ Name = 'Git';   Version = '2.47.0' }
    [pscustomobject]@{ Name = 'Zoom';  Version = '6.2.5' }
)

function Get-Port {
    param(
        [Parameter(ValueFromPipelineByPropertyName)][Alias('Host')][string]$ComputerName,
        [Parameter(ValueFromPipelineByPropertyName)][int]$Port
    )
    process { "${ComputerName}:$Port" }
}
$targets | Get-Port

$diff = Compare-Object $yesterday $today
"added: $(($diff | Where-Object SideIndicator -eq '=>').InputObject -join ', ')"
"removed: $(($diff | Where-Object SideIndicator -eq '<=').InputObject -join ', ')"

Compare-Object $old $new -Property Name, Version | Group-Object Name | Sort-Object Name | ForEach-Object {
    $before = $_.Group | Where-Object SideIndicator -eq '<='
    $after = $_.Group | Where-Object SideIndicator -eq '=>'
    if ($before -and $after) { "updated: $($_.Name) $($before.Version) -> $($after.Version)" }
    elseif ($after) { "installed: $($_.Name) $($after.Version)" }
    else { "uninstalled: $($_.Name) $($before.Version)" }
}

@@lesson p0-pipeline The Pipeline: Objects, Not Text
@@text
The `|` symbol (the **pipe**) sends the output of one command into the next. In Linux shells that's text. In PowerShell it's **objects**, and that changes everything.

```powershell
Get-Process | Sort-Object WorkingSet -Descending | Select-Object -First 5 Name, Id, WorkingSet
```

Read it left to right: *get all processes*, *sort them by memory use, biggest first*, *keep the first 5 and only these three properties*. No text cutting: `WorkingSet` is a real number, so sorting is correct.

## The core pipeline cmdlets

| Cmdlet | Does | Example |
|---|---|---|
| `Where-Object` | keep objects that match | `Where-Object Status -eq Running` |
| `Sort-Object` | sort | `Sort-Object Name` |
| `Select-Object` | pick properties, or the first/last N | `Select-Object -First 3` |
| `Measure-Object` | count, sum, average | `Measure-Object Length -Sum` |
| `Group-Object` | group and count | `Group-Object Extension` |
| `ForEach-Object` | do something with each | `ForEach-Object { $_.Name }` |

Inside a script block `{ }`, **`$_`** is "the current object" (also written `$PSItem`).

```powershell
Get-Service | Where-Object Status -eq 'Running' | Measure-Object | Select-Object -ExpandProperty Count
```

## Making your own objects to practice with

`[pscustomobject]@{ ... }` builds an object from names and values. We'll use these a lot, because they behave exactly like the real objects from `Get-Process` or `Get-ADUser`, but give the same result on every computer.

```powershell
$disks = @(
    [pscustomobject]@{ Drive = 'C:'; SizeGB = 476; FreeGB = 38 }
    [pscustomobject]@{ Drive = 'D:'; SizeGB = 931; FreeGB = 610 }
    [pscustomobject]@{ Drive = 'E:'; SizeGB = 119; FreeGB = 4 }
)
$disks | Where-Object FreeGB -lt 50 | ForEach-Object { "$($_.Drive) is low on space: $($_.FreeGB) GB free" }
```

## Output and formatting

At the end of the pipeline, PowerShell formats objects as a table or list for you. `Format-Table` and `Format-List` let you choose, but they produce *display* objects: **always format last**, never pipe `Format-*` into something else. To get plain values, use `Select-Object -ExpandProperty Name` or `ForEach-Object { $_.Name }`.

@@quiz
Inside `ForEach-Object { ... }`, what is `$_`?
- the previous command's text output
* the current object coming through the pipeline
- the last error
- the script's first argument
? `$_` (or `$PSItem`) is the object currently being processed.

@@quiz
Why does `Get-ChildItem | Sort-Object Length` sort file sizes correctly, while sorting text output of `dir` in some shells doesn't?
* Because `Length` is a number on a real object, not text
- Because PowerShell sorts everything alphabetically
- Because Sort-Object reads the file contents
- It doesn't sort correctly
? Objects keep their types. Text sorting puts "10" before "9"; number sorting doesn't.

@@quiz
Which pipeline is a mistake?
- `Get-Service | Where-Object Status -eq Running | Format-Table`
* `Get-Service | Format-Table | Where-Object Status -eq Running`
- `Get-Service | Sort-Object Name | Select-Object -First 5`
- `Get-Process | Measure-Object`
? `Format-*` turns objects into display data. Anything after it no longer sees `Status`. Format last.

@@drill
Which cmdlet keeps only the objects that match a condition?
= Where-Object
= where
= ?
? Get-Process | Where-Object CPU -gt 100.

@@flashcards
Where-Object :: Filter: keep objects that match a condition
Select-Object :: Pick properties, or `-First`/`-Last` N objects; `-ExpandProperty` gives plain values
Sort-Object :: Sort by a property; `-Descending` for biggest first
Measure-Object :: Count, `-Sum`, `-Average`, `-Maximum`, `-Minimum`
Group-Object :: Group objects by a property and count each group
$_ :: The current pipeline object inside a script block
Format-Table :: Display as a table. Always the last command in a pipeline

@@exercise Low disk space
Using the `$disks` objects given, print the **drive** of every disk with **less than 50 GB free**, one per line, in the original order.
@@starter
$disks = @(
    [pscustomobject]@{ Drive = 'C:'; SizeGB = 476; FreeGB = 38 }
    [pscustomobject]@{ Drive = 'D:'; SizeGB = 931; FreeGB = 610 }
    [pscustomobject]@{ Drive = 'E:'; SizeGB = 119; FreeGB = 4 }
    [pscustomobject]@{ Drive = 'F:'; SizeGB = 238; FreeGB = 49 }
)

@@expected
C:
E:
F:
@@hint
`$disks | Where-Object FreeGB -lt 50 | ForEach-Object { $_.Drive }`
@@solution
$disks = @(
    [pscustomobject]@{ Drive = 'C:'; SizeGB = 476; FreeGB = 38 }
    [pscustomobject]@{ Drive = 'D:'; SizeGB = 931; FreeGB = 610 }
    [pscustomobject]@{ Drive = 'E:'; SizeGB = 119; FreeGB = 4 }
    [pscustomobject]@{ Drive = 'F:'; SizeGB = 238; FreeGB = 49 }
)
$disks | Where-Object FreeGB -lt 50 | ForEach-Object { $_.Drive }

@@exercise Top memory users
Print the **names** of the 3 processes using the most memory (`MemMB`), biggest first.
@@starter
$procs = @(
    [pscustomobject]@{ Name = 'chrome';  MemMB = 812 }
    [pscustomobject]@{ Name = 'svchost'; MemMB = 96 }
    [pscustomobject]@{ Name = 'Teams';   MemMB = 640 }
    [pscustomobject]@{ Name = 'explorer'; MemMB = 150 }
    [pscustomobject]@{ Name = 'sqlservr'; MemMB = 1900 }
)

@@expected
sqlservr
chrome
Teams
@@hint
Sort with `Sort-Object MemMB -Descending`, keep `Select-Object -First 3`, then `-ExpandProperty Name` (or `ForEach-Object { $_.Name }`).
@@solution
$procs = @(
    [pscustomobject]@{ Name = 'chrome';  MemMB = 812 }
    [pscustomobject]@{ Name = 'svchost'; MemMB = 96 }
    [pscustomobject]@{ Name = 'Teams';   MemMB = 640 }
    [pscustomobject]@{ Name = 'explorer'; MemMB = 150 }
    [pscustomobject]@{ Name = 'sqlservr'; MemMB = 1900 }
)
$procs | Sort-Object MemMB -Descending | Select-Object -First 3 -ExpandProperty Name
@@text
> On a real computer, try `Get-Process | Sort-Object WorkingSet -Descending | Select-Object -First 3 -ExpandProperty Name`. Same pipeline, live data.

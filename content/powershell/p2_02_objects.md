@@lesson p2-objects Objects: Filter, Group, Measure, Report
@@concepts objects
@@text
Real admin work is: get objects, filter them, shape them, report. Let's do it with a list of users like you'd get from `Get-ADUser`.

```powershell
$users = @(
    [pscustomobject]@{ Name = 'asilva';  Dept = 'Sales';   Enabled = $true;  LastLogonDays = 2 }
    [pscustomobject]@{ Name = 'ktanaka'; Dept = 'IT';      Enabled = $true;  LastLogonDays = 0 }
    [pscustomobject]@{ Name = 'bjones';  Dept = 'Finance'; Enabled = $false; LastLogonDays = 140 }
    [pscustomobject]@{ Name = 'mlee';    Dept = 'Sales';   Enabled = $true;  LastLogonDays = 95 }
)

# Filter: enabled accounts nobody has used for 90+ days (a classic security audit)
$users | Where-Object { $_.Enabled -and $_.LastLogonDays -ge 90 } | Select-Object Name, LastLogonDays

# Group: how many users per department
$users | Group-Object Dept | Select-Object Name, Count

# Measure: average days since logon
($users | Measure-Object LastLogonDays -Average).Average
```

## Where-Object: two syntaxes

```powershell,norun
Get-Service | Where-Object Status -eq 'Running'                          # simple: one condition
Get-Service | Where-Object { $_.Status -eq 'Running' -and $_.Name -like 'Win*' }   # script block: anything
```

## Calculated properties

`Select-Object` can make **new** properties with `@{ Name = ...; Expression = { ... } }`:

```powershell
$disks = @(
    [pscustomobject]@{ Drive = 'C:'; SizeGB = 476; FreeGB = 38 }
    [pscustomobject]@{ Drive = 'D:'; SizeGB = 931; FreeGB = 610 }
)
$disks | Select-Object Drive, @{ Name = 'UsedPct'; Expression = { [math]::Round(100 - $_.FreeGB / $_.SizeGB * 100) } }
```

## Adding properties and methods to objects

```powershell
$server = [pscustomobject]@{ Name = 'web01'; Role = 'IIS' }
$server | Add-Member -NotePropertyName Owner -NotePropertyValue 'ana'
$server
```

## Exporting the result

The best part: a filtered list of objects becomes a spreadsheet or web page in one command.

```powershell,norun
$users | Where-Object { -not $_.Enabled } | Export-Csv -Path disabled.csv -NoTypeInformation
$users | ConvertTo-Html -Title 'Users' | Set-Content users.html
$users | ConvertTo-Json | Set-Content users.json
```

@@quiz
Which command finds out **how many** users each department has?
- `$users | Measure-Object Dept`
* `$users | Group-Object Dept`
- `$users | Sort-Object Dept -Count`
- `$users | Select-Object Dept -Unique`
? `Group-Object` makes one group per value, with a `Count` and the group's objects.

@@quiz
What does `@{ Name = 'GB'; Expression = { $_.Length / 1GB } }` do inside `Select-Object`?
* Adds a calculated property `GB` to each output object
- Renames the Length property
- Filters files bigger than 1 GB
- Creates a hashtable
? A calculated property: a name plus a script block that computes the value for each object.

@@quiz
Which command adds up a property across objects, like the total size of files?
* Measure-Object -Property Length -Sum
- Group-Object -Sum
- Sort-Object Length
- Select-Object -Sum
? Measure-Object also gives -Average, -Minimum and -Maximum.

@@flashcards
Where-Object :: keep objects that match a condition
Select-Object :: choose properties, or first/last N objects
Sort-Object :: order objects by a property
Group-Object :: group objects by a property, with counts
Measure-Object :: count, sum, average, min, max
calculated property :: @{ Name = 'GB'; Expression = { ... } }
[pscustomobject]@{ } :: build your own object
Export-Csv -NoTypeInformation :: save objects as CSV

@@exercise Stale accounts
Print the names of users who are **enabled** and haven't logged on for **90 days or more**, sorted alphabetically.
@@starter
$users = @(
    [pscustomobject]@{ Name = 'asilva';  Enabled = $true;  LastLogonDays = 2 }
    [pscustomobject]@{ Name = 'tkato';   Enabled = $true;  LastLogonDays = 200 }
    [pscustomobject]@{ Name = 'bjones';  Enabled = $false; LastLogonDays = 140 }
    [pscustomobject]@{ Name = 'mlee';    Enabled = $true;  LastLogonDays = 95 }
    [pscustomobject]@{ Name = 'rchen';   Enabled = $true;  LastLogonDays = 90 }
)

@@expected
mlee
rchen
tkato
@@hint
`Where-Object { $_.Enabled -and $_.LastLogonDays -ge 90 }`, then `Sort-Object Name`, then output the names.
@@solution
$users = @(
    [pscustomobject]@{ Name = 'asilva';  Enabled = $true;  LastLogonDays = 2 }
    [pscustomobject]@{ Name = 'tkato';   Enabled = $true;  LastLogonDays = 200 }
    [pscustomobject]@{ Name = 'bjones';  Enabled = $false; LastLogonDays = 140 }
    [pscustomobject]@{ Name = 'mlee';    Enabled = $true;  LastLogonDays = 95 }
    [pscustomobject]@{ Name = 'rchen';   Enabled = $true;  LastLogonDays = 90 }
)
$users | Where-Object { $_.Enabled -and $_.LastLogonDays -ge 90 } | Sort-Object Name | ForEach-Object { $_.Name }

@@exercise Size per extension
Group the files by `Extension` and print, for each extension (alphabetically), `ext: count files, total KB` where the total is the sum of `Length` in KB, rounded to whole numbers.
@@starter
$files = @(
    [pscustomobject]@{ Name = 'a.log'; Extension = '.log'; Length = 20480 }
    [pscustomobject]@{ Name = 'b.log'; Extension = '.log'; Length = 10240 }
    [pscustomobject]@{ Name = 'c.txt'; Extension = '.txt'; Length = 1024 }
    [pscustomobject]@{ Name = 'd.zip'; Extension = '.zip'; Length = 512000 }
    [pscustomobject]@{ Name = 'e.log'; Extension = '.log'; Length = 3000 }
)

@@expected
.log: 3 files, 33 KB
.txt: 1 files, 1 KB
.zip: 1 files, 500 KB
@@hint
`$files | Group-Object Extension | Sort-Object Name | ForEach-Object { ... }`. Inside, `$_.Group` holds the files of that group: `($_.Group | Measure-Object Length -Sum).Sum`.
@@solution
$files = @(
    [pscustomobject]@{ Name = 'a.log'; Extension = '.log'; Length = 20480 }
    [pscustomobject]@{ Name = 'b.log'; Extension = '.log'; Length = 10240 }
    [pscustomobject]@{ Name = 'c.txt'; Extension = '.txt'; Length = 1024 }
    [pscustomobject]@{ Name = 'd.zip'; Extension = '.zip'; Length = 512000 }
    [pscustomobject]@{ Name = 'e.log'; Extension = '.log'; Length = 3000 }
)
$files | Group-Object Extension | Sort-Object Name | ForEach-Object {
    $kb = [math]::Round(($_.Group | Measure-Object Length -Sum).Sum / 1KB)
    "$($_.Name): $($_.Count) files, $kb KB"
}

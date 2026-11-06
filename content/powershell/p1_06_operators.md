@@lesson p1-operators Operators in Depth: Comparison, Types and Surprises
@@concepts conditions, strings, numbers
@@text
PowerShell's operators look like words (`-eq`, `-like`, `-in`) because `<` and `>` are taken by redirection. They have a few rules that surprise people coming from other languages. Learn them once and a whole class of bugs disappears.

## Case: insensitive by default

```powershell
'Admin' -eq 'admin'      # True  (like Windows file names)
'Admin' -ceq 'admin'     # False: c = case-sensitive
'Admin' -ieq 'admin'     # True:  i = explicitly insensitive
```

Every comparison operator has `c` and `i` versions: `-clike`, `-cmatch`, `-creplace`, `-ccontains`...

## Which comparison operator?

| Operator | Question | Example |
|---|---|---|
| `-eq` `-ne` `-gt` `-ge` `-lt` `-le` | equal / ordered | `$disk.Free -lt 10GB` |
| `-like` `-notlike` | wildcard (`*`, `?`, `[a-c]`) | `$name -like 'srv-*'` |
| `-match` `-notmatch` | regular expression | `$line -match '^ERROR'` |
| `-contains` `-notcontains` | does a **collection** contain a value? | `$admins -contains $user` |
| `-in` `-notin` | is a value **in** a collection? | `$user -in $admins` |
| `-is` `-isnot` | type test | `$x -is [int]` |
| `-as` | convert, or `$null` on failure | `'42' -as [int]` |

## The left side decides the type

When types differ, PowerShell converts the **right** operand to the type of the **left** one:

```powershell
10 -eq '10'      # True: '10' becomes 10
'10' -eq 10      # True: 10 becomes '10'
'10' -lt '9'     # True! string comparison: '1' sorts before '9'
10 -lt '9'       # False: numeric comparison
[int]'10' -lt 9  # False: cast first when the data is text (CSV, Read-Host)
```

Data from CSV files and `Read-Host` is always **text**. Cast it before comparing numbers.

## Operators on collections filter

With an array on the left, `-eq`, `-like`, `-match` and friends return the **matching items**, not True/False:

```powershell
$ports = 22, 80, 443, 8080
$ports -gt 100                  # 443, 8080
'web01', 'db01', 'web02' -like 'web*'
```

That's why `$null` belongs on the **left**: `$list -eq $null` filters the list (and returns an empty, falsy result if nothing is null), while `$null -eq $list` asks the real question.

## Logical and bitwise

`-and`, `-or`, `-not` (or `!`), `-xor`. Bitwise: `-band`, `-bor`, `-bxor`, `-bnot`, `-shl`, `-shr`, used for flags like file attributes and userAccountControl in Active Directory:

```powershell
$uac = 514                         # 512 normal account + 2 disabled
[bool]($uac -band 2)               # True: the account is disabled
```

## Truthiness

False values: `$false`, `$null`, `0`, `''`, an **empty** array, and an array holding a single falsy value. Everything else is true, including the string `'False'` and `'0'`. Use explicit checks (`-eq 0`, `[string]::IsNullOrEmpty($s)`) when it matters.

## Ranges, membership and a few extras

```powershell
1..5                       # 1 2 3 4 5
-join [char[]](97..101)    # abcde ('a'..'e' only works in PowerShell 7)
(1..10) -contains 7
"$(5 % 3) $([math]::Floor(7 / 2)) $(7 / 2)"   # % is the remainder; / gives decimals
```

`7 / 2` is `3.5`, not 3: PowerShell switches to decimals. Use `[math]::Floor()` or `[int][math]::Truncate()` for integer division. (Casting with `[int]` rounds half to even: `[int]2.5` is 2, `[int]3.5` is 4.)

@@quiz
What does `'10' -lt '9'` return?
* True, because both are strings and '1' sorts before '9'
- False
- An error
- $null
? Cast text to numbers before comparing.

@@quiz
`$servers = 'web01', 'db01'`. Which expression returns True/False to "is db01 in the list"?
- `'db01' -contains $servers`
* `'db01' -in $servers` (or `$servers -contains 'db01'`)
- `$servers -eq 'db01'`
- `$servers -like 'db01'`
? `-eq` on an array filters and returns the matching items.

@@quiz
Why write `$null -eq $x` instead of `$x -eq $null`?
- It's faster
* If `$x` is a collection, `-eq` filters it instead of answering the question
- PowerShell requires it
- It works with strings only
? PSScriptAnalyzer warns about `$x -eq $null`.

@@drill
Which operator compares strings case-sensitively for equality?
= -ceq
? -eq ignores case by default.

@@flashcards
`-ceq` :: case-sensitive equals
`-like` :: wildcard match
`-match` :: regex match (fills $Matches)
`-contains` :: collection contains value
`-in` :: value in collection
`-is` / `-as` :: type test / safe conversion
left operand :: decides the comparison type
`$null -eq $x` :: correct null check
`-band` :: bitwise AND (flags)
`7 / 2` :: 3.5 (not integer division)
`[int]2.5` :: 2 (banker's rounding)

@@exercise Compare the right way
The values below came from a CSV export, so every field is text. Fix the comparisons:

- list the servers with **more than 9** CPU cores (numeric, not text comparison)
- list the servers whose name matches `web*` **case-sensitively** (only lowercase `web`)
- check whether `db01` is among the names, with `-in`
- count the disabled accounts from the `Uac` flags (bit value 2 means disabled)
- show what `'10' -lt '9'` and `[int]'10' -lt 9` give
@@starter
$rows = @(
    [pscustomobject]@{ Name = 'web01'; Cores = '16'; Uac = '512' }
    [pscustomobject]@{ Name = 'WEB02'; Cores = '8';  Uac = '514' }
    [pscustomobject]@{ Name = 'db01';  Cores = '32'; Uac = '546' }
    [pscustomobject]@{ Name = 'web03'; Cores = '4';  Uac = '66050' }
)
'TODO'
@@expected
more than 9 cores: web01, db01
lowercase web*: web01, web03
db01 present: True
disabled accounts: 3
'10' -lt '9': True
[int]'10' -lt 9: False
@@requires
-clike
-in
-band
[int]
@@hint
`($rows | Where-Object { [int]$_.Cores -gt 9 }).Name -join ', '`. Disabled: `@($rows | Where-Object { [int]$_.Uac -band 2 }).Count`. Print the last two with `"'10' -lt '9': $('10' -lt '9')"`.
@@solution
$rows = @(
    [pscustomobject]@{ Name = 'web01'; Cores = '16'; Uac = '512' }
    [pscustomobject]@{ Name = 'WEB02'; Cores = '8';  Uac = '514' }
    [pscustomobject]@{ Name = 'db01';  Cores = '32'; Uac = '546' }
    [pscustomobject]@{ Name = 'web03'; Cores = '4';  Uac = '66050' }
)
$big = $rows | Where-Object { [int]$_.Cores -gt 9 }
"more than 9 cores: $($big.Name -join ', ')"
$web = $rows | Where-Object { $_.Name -clike 'web*' }
"lowercase web*: $($web.Name -join ', ')"
"db01 present: $('db01' -in $rows.Name)"
$disabled = @($rows | Where-Object { [int]$_.Uac -band 2 })
"disabled accounts: $($disabled.Count)"
"'10' -lt '9': $('10' -lt '9')"
"[int]'10' -lt 9: $([int]'10' -lt 9)"

@@exercise Operators on collections, and truthiness
Explore how operators behave with arrays and "falsy" values:

- filter `$ports` with `-gt 1024` and `-notin $allowed` (use `Where-Object` for the second)
- show `$list -eq $null` vs `$null -eq $list` for a list containing `$null`, by counting results and printing the boolean
- print whether each value in `$values` is truthy, in the form `value -> True/False` (describe the value with its type)
- integer division: print `17 / 5`, `[math]::Floor(17 / 5)`, `17 % 5`, `[int]2.5` and `[int]3.5`
@@starter
$ports = 22, 80, 443, 3389, 5985, 8080
$allowed = 22, 443, 8080
$list = 'a', $null, 'b'
$values = @(0, 1, '', 'False', '0', $null, @(), @(0))
'TODO'
@@expected
above 1024: 3389, 5985, 8080
not allowed: 80, 3389, 5985
$list -eq $null: 1 item(s)
$null -eq $list: False
Int32 0 -> False
Int32 1 -> True
String '' -> False
String 'False' -> True
String '0' -> True
null -> False
Object[] (0 items) -> False
Object[] (1 items) -> False
17 / 5 = 3.4
floor = 3, remainder = 2
[int]2.5 = 2, [int]3.5 = 4
@@requires
-notin
$null -eq
[math]::Floor
@@hint
Loop over the values with `for ($i = 0; $i -lt $values.Count; $i++)`, because `foreach` would unroll the nested arrays. Describe a value: `$null` → `null`; an array (`$v -is [array]`) → `"$($v.GetType().Name) ($($v.Count) items)"`; a string → `"String '$v'"`; otherwise `"$($v.GetType().Name) $v"`. Truthiness: `[bool]$v`. Print `17 / 5` with `.ToString([cultureinfo]::InvariantCulture)` so the decimal point is a dot everywhere.
@@solution
$ports = 22, 80, 443, 3389, 5985, 8080
$allowed = 22, 443, 8080
$list = 'a', $null, 'b'
$values = @(0, 1, '', 'False', '0', $null, @(), @(0))

"above 1024: $(($ports -gt 1024) -join ', ')"
$bad = $ports | Where-Object { $_ -notin $allowed }
"not allowed: $($bad -join ', ')"
"`$list -eq `$null: $(@($list -eq $null).Count) item(s)"
"`$null -eq `$list: $($null -eq $list)"

for ($i = 0; $i -lt $values.Count; $i++) {
    $v = $values[$i]
    if ($null -eq $v) { $desc = 'null' }
    elseif ($v -is [array]) { $desc = "$($v.GetType().Name) ($($v.Count) items)" }
    elseif ($v -is [string]) { $desc = "String '$v'" }
    else { $desc = "$($v.GetType().Name) $v" }
    "$desc -> $([bool]$v)"
}

"17 / 5 = $((17 / 5).ToString([cultureinfo]::InvariantCulture))"
"floor = $([math]::Floor(17 / 5)), remainder = $(17 % 5)"
"[int]2.5 = $([int]2.5), [int]3.5 = $([int]3.5)"

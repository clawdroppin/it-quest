@@lesson p1-strings Working with Text
@@concepts strings
@@text
## Single vs double quotes

```powershell
$name = "ana"
"Hello $name"      # Hello ana       double quotes expand variables
'Hello $name'      # Hello $name     single quotes are literal
"Total: $(2 + 3)"  # $( ) runs any expression
"Path: $env:TEMP\logs"
```

Use **single quotes** when you don't want expansion (passwords, regex, paths with `$`). The escape character is the **backtick** `` ` ``: `` "`n" `` is a new line, `` "`t" `` a tab, `` "`$" `` a literal dollar sign.

## Methods and operators

Strings are .NET objects, so they have methods:

```powershell
$host1 = "  Web01.Contoso.local  "
$host1.Trim()
$host1.Trim().ToUpper()
$host1.Trim().Length
"web01.contoso.local".Split('.')[0]
"web01".StartsWith("web")
"error: disk full".Replace("error", "ERROR")
"web01.contoso.local".Contains("contoso")
```

PowerShell also has its own operators, which are **case-insensitive** by default (Windows usually is):

```powershell
"WEB01" -eq "web01"           # True
"WEB01" -ceq "web01"          # False: the c means case-sensitive
"web01.contoso.local" -like "*.contoso.local"   # wildcard match
"a,b,c" -split ","            # split into an array
@("a", "b", "c") -join ";"     # join an array into text
```

## The format operator -f

For aligned reports, `-f` fills `{0}`, `{1}`, ... with values. After a colon comes a format: `N2` is a number with 2 decimals, `P0` a percentage, `,-10` left-aligns in 10 characters.

```powershell
"{0} has {1:N1} GB free" -f "C:", 38.456
"{0,-8}|{1,6}" -f "DISK", "FREE"
"{0:P0}" -f 0.917
```

## Here-strings

A text block over several lines, for emails or report templates:

```powershell
$user = "ana"
$body = @"
Hello $user,
Your account is ready.
"@
$body
```

The closing `"@` must be at the very start of its line.

@@quiz
What does `'Cost: $5'` print?
* `Cost: $5`
- `Cost: 5`
- `Cost: ` (empty variable)
- an error
? Single quotes never expand variables.

@@quiz
`"SERVER" -eq "server"` is...
* `$true`, comparison operators ignore case by default
- `$false`
- an error
- `$null`
? Use `-ceq` for a case-sensitive comparison.

@@drill
Which operator joins the array `$names` into one string separated by commas? Write the full expression.
= $names -join ","
= $names -join ','
= $names -join ", "
? `-join` glues an array together; `-split` does the reverse.

@@exercise Short host name
`$fqdn` holds a fully qualified domain name. Print just the **short name** (the part before the first dot) in **uppercase**.
@@starter
$fqdn = "web01.contoso.local"

@@expected
WEB01
@@hint
`$fqdn.Split('.')[0].ToUpper()`
@@solution
$fqdn = "web01.contoso.local"
Write-Output $fqdn.Split('.')[0].ToUpper()

@@exercise Username from a full name
Write a function `Get-Username` that turns a full name into a company username: **first letter of the first name + the last name**, all lowercase, spaces trimmed. `"Ana Silva"` becomes `asilva`.
@@starter
function Get-Username($fullName) {
    throw "TODO: not done yet"
}

@@tests
check (Get-Username "Ana Silva") "asilva" "Get-Username 'Ana Silva'"
check (Get-Username "  Kenji   Tanaka ") "ktanaka" "Get-Username '  Kenji   Tanaka '"
check (Get-Username "BEN JONES") "bjones" "Get-Username 'BEN JONES'"
@@hint
Split on whitespace and drop empty parts: `$parts = $fullName.Trim() -split '\s+'`. Then `$parts[0][0]` is the first letter and `$parts[-1]` the last name.
@@solution
function Get-Username($fullName) {
    $parts = $fullName.Trim() -split '\s+'
    return ("$($parts[0][0])$($parts[-1])").ToLower()
}

@@exercise Aligned report
Print each disk in `$disks` as one line: the drive left-aligned in **6** characters, then the free space with **1 decimal** and ` GB`. Use `-f`.
@@starter
$disks = @(
    [pscustomobject]@{ Drive = 'C:'; FreeGB = 38.456 }
    [pscustomobject]@{ Drive = 'D:'; FreeGB = 610 }
)

@@expected
C:    38.5 GB
D:    610.0 GB
@@hint
`"{0,-6}{1:N1} GB" -f $_.Drive, $_.FreeGB` inside `ForEach-Object`.
@@solution
$disks = @(
    [pscustomobject]@{ Drive = 'C:'; FreeGB = 38.456 }
    [pscustomobject]@{ Drive = 'D:'; FreeGB = 610 }
)
$disks | ForEach-Object { "{0,-6}{1:N1} GB" -f $_.Drive, $_.FreeGB }

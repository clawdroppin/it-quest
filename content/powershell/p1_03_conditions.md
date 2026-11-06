@@lesson p1-conditions Decisions: if and switch
@@concepts conditions
@@text
PowerShell's comparison operators are words, because `>` and `<` already mean **redirect output to a file**:

| Operator | Means | Case-sensitive version |
|---|---|---|
| `-eq`, `-ne` | equal, not equal | `-ceq`, `-cne` |
| `-gt`, `-ge` | greater (or equal) | |
| `-lt`, `-le` | less (or equal) | |
| `-like`, `-notlike` | wildcard `*` `?` | `-clike` |
| `-match`, `-notmatch` | regular expression | `-cmatch` |
| `-in`, `-contains` | is in a list / list contains | |

Combine with `-and`, `-or`, `-not` (or `!`).

```powershell
$cpu = 91
if ($cpu -ge 95) {
    "CRITICAL"
} elseif ($cpu -ge 80) {
    "WARNING"
} else {
    "OK"
}
```

> ⚠ A classic mistake: `if ($cpu > 80)` doesn't compare. It **creates a file named 80** and writes `$cpu` into it.

## Lists and membership

```powershell
$allowed = 'ana', 'ben', 'kenji'
'ben' -in $allowed           # True
$allowed -contains 'mia'     # False
'web01' -like 'web*'         # True
```

## switch

When one value can be many things, `switch` is clearer than a long `elseif` chain:

```powershell
$code = 404
switch ($code) {
    200 { "OK" }
    301 { "Moved permanently" }
    404 { "Not found" }
    { $_ -ge 500 } { "Server error" }
    default { "Something else" }
}
```

A condition can be a script block using `$_`. `switch -Wildcard` and `switch -Regex` match patterns, and `switch` even works on arrays, running once per item.

## $null checks

Put `$null` on the **left**: `if ($null -eq $result)`. If `$result` were an array, `$result -eq $null` would filter the array instead of answering yes or no. PSScriptAnalyzer warns about this too.
## Comparison operators are words

Because `>` and `<` are **redirection** in a shell (they write output to files), PowerShell uses words for comparisons:

| Operator | Meaning |
|---|---|
| `-eq`, `-ne` | equal, not equal |
| `-gt`, `-ge`, `-lt`, `-le` | greater than, greater or equal, less than, less or equal |
| `-like`, `-notlike` | wildcard match: `'web01' -like 'web*'` |
| `-match`, `-notmatch` | regular expression match |
| `-in`, `-contains` | membership: `'ssh' -in $allowed`, `$allowed -contains 'ssh'` |
| `-and`, `-or`, `-not` (or `!`) | combine conditions |

Text comparisons are **case-insensitive** by default: `'SERVER' -eq 'server'` is true. Add a `c` for case-sensitive versions: `-ceq`, `-clike`, `-cmatch`.

```powershell
$disk = 8
$role = 'Web'
if ($disk -lt 10 -and $role -eq 'web') { 'web server low on disk' }
'db01' -like 'db*'
'WEB01' -ceq 'web01'
```

Writing `if ($free > 10)` is a classic mistake: it creates a file called `10` and the condition is always true.

@@quiz
What does `if ($free > 10) { ... }` do?
- compares $free with 10
* writes the value of $free into a file called `10`
- throws a syntax error
- compares the lengths
? `>` is redirection in PowerShell. Use `-gt`.

@@quiz
Which condition is true?
- `'Server01' -clike 'server*'`
* `'Server01' -like 'server*'`
- `'Server01' -eq 'Server1'`
- `5 -in 1, 2, 3`
? `-like` ignores case; `-clike` doesn't.

@@quiz
What is `'web01' -like 'web*'`?
* True
- False
- An error, because -like needs a regex
- 'web*'
? -like uses wildcards: * for any characters, ? for one.

@@flashcards
-eq / -ne :: equal / not equal (case-insensitive for text)
-gt -ge -lt -le :: numeric and text ordering
-like :: wildcard match with * and ?
-match :: regular expression match
-in / -contains :: is the value in the list?
-ceq :: case-sensitive equality
switch :: compare one value against many cases

@@exercise Service health
`$status` is a Windows service status. Print `healthy` if it's `Running`, `starting` if it's `StartPending`, and `ALERT: <status>` for anything else.
@@starter
$status = 'Stopped'

@@expected
ALERT: Stopped
@@hint
A `switch` with two values and a `default { "ALERT: $status" }`.
@@solution
$status = 'Stopped'
switch ($status) {
    'Running' { 'healthy' }
    'StartPending' { 'starting' }
    default { "ALERT: $status" }
}

@@exercise Password check
Write a function `Test-Password` that returns `$true` if a password is **at least 12** characters, contains **a digit**, and is **not** in the list of banned words. Otherwise `$false`.
@@starter
$banned = 'password123456', 'welcome12345', 'qwerty123456'

function Test-Password($pw) {
    throw "TODO: not done yet"
}

@@tests
check (Test-Password 'CorrectHorse42') $true "Test-Password 'CorrectHorse42'"
check (Test-Password 'short1') $false "Test-Password 'short1' (too short)"
check (Test-Password 'NoDigitsAtAllHere') $false "Test-Password 'NoDigitsAtAllHere'"
check (Test-Password 'Welcome12345') $false "Test-Password 'Welcome12345' (banned, any case)"
@@hint
`$pw.Length -ge 12 -and $pw -match '\d' -and $pw -notin $banned`. Remember `-notin` ignores case, which is what we want.
@@solution
$banned = 'password123456', 'welcome12345', 'qwerty123456'

function Test-Password($pw) {
    return ($pw.Length -ge 12) -and ($pw -match '\d') -and ($pw -notin $banned)
}

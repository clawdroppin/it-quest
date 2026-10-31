@@lesson p2-collections Arrays and Hashtables
@@concepts lists, maps
@@text
## Arrays

```powershell
$servers = @('web01', 'web02', 'db01')   # @() is the array syntax; commas alone work too
$servers[0]        # web01
$servers[-1]       # db01: negative counts from the end
$servers[0..1]     # web01, web02
$servers.Count     # 3
$servers += 'db02' # adds (by making a new array)
$servers -contains 'db01'
```

`@()` around a command's output **guarantees an array**, even if it returns one item or none. That matters: with one result, `.Count` and indexing behave differently otherwise.

```powershell
$found = @(Get-ChildItem -Path $env:TEMP -Filter '*.nothing-matches-this')
$found.Count   # 0, not an error
```

For a list you'll add to a lot, use a .NET list:

```powershell
$list = [System.Collections.Generic.List[string]]::new()
$list.Add('web01')
$list.Add('web02')
$list.Count
```

## Hashtables: key → value

```powershell
$owners = @{
    'web01' = 'ana'
    'db01'  = 'kenji'
}
$owners['web01']          # ana
$owners.db01              # kenji: dot syntax works too
$owners['mail01'] = 'ben' # add or change
$owners.ContainsKey('db01')
$owners.Remove('web01')
foreach ($key in $owners.Keys) { "$key is owned by $($owners[$key])" }
```

Hashtables have **no fixed order**. Use `[ordered]@{ ... }` when the order matters (reports, CSV columns).

## Counting with a hashtable

The single most useful pattern in log analysis:

```powershell
$events = 'login', 'logout', 'login', 'failed', 'login', 'failed'
$counts = @{}
foreach ($e in $events) {
    $counts[$e] = $counts[$e] + 1   # a missing key gives $null, and $null + 1 is 1
}
$counts.GetEnumerator() | Sort-Object Value -Descending | ForEach-Object { "$($_.Key): $($_.Value)" }
```

## Splatting

A hashtable can hold a command's parameters, handy when there are many:

```powershell,norun
$params = @{
    Path        = 'C:\Logs'
    Filter      = '*.log'
    Recurse     = $true
    ErrorAction = 'SilentlyContinue'
}
Get-ChildItem @params     # @params, not $params
```

@@quiz
`$a = @('x', 'y', 'z')`. What is `$a[-1]`?
- `x`
* `z`
- an error
- `$null`
? Negative indexes count from the end.

@@quiz
Why wrap command output in `@( )`?
* So the result is always an array, even with zero or one item
- To make it faster
- To sort it
- To turn it into a hashtable
? Without `@()`, one result is a single object, and zero results is `$null`.

@@flashcards
@() :: Array syntax; also forces command output into an array
@{} :: Hashtable syntax: key = value pairs
[ordered]@{} :: A hashtable that keeps the order its keys were added
splatting :: Passing a hashtable of parameters to a command with `@name`
.ContainsKey() :: Check whether a hashtable has a key

@@exercise Count failed logins per user
Count how many times each user appears in `$failed`, and print `user: count` lines **sorted by count, highest first** (ties: by name).
@@starter
$failed = 'root', 'admin', 'root', 'ben', 'root', 'admin', 'test', 'ben', 'root'

@@expected
root: 4
admin: 2
ben: 2
test: 1
@@hint
Count into a hashtable, then `$counts.GetEnumerator() | Sort-Object @{e='Value';Descending=$true}, Name`. (A sort property can be a hashtable with its own direction.)
@@solution
$failed = 'root', 'admin', 'root', 'ben', 'root', 'admin', 'test', 'ben', 'root'
$counts = @{}
foreach ($u in $failed) {
    $counts[$u] = $counts[$u] + 1
}
$counts.GetEnumerator() |
    Sort-Object @{ Expression = 'Value'; Descending = $true }, Name |
    ForEach-Object { "$($_.Key): $($_.Value)" }

@@exercise Department lookup
Write `Get-Department` that takes a username and returns its department from the `$staff` hashtable, or `unknown` if the user isn't there.
@@starter
$staff = @{
    'asilva'  = 'Sales'
    'ktanaka' = 'IT'
    'bjones'  = 'Finance'
}

function Get-Department($user) {
    throw "TODO: not done yet"
}

@@tests
check (Get-Department 'ktanaka') 'IT' "Get-Department 'ktanaka'"
check (Get-Department 'BJONES') 'Finance' "Get-Department 'BJONES' (keys ignore case)"
check (Get-Department 'nobody') 'unknown' "Get-Department 'nobody'"
@@hint
`if ($staff.ContainsKey($user)) { return $staff[$user] }`, then `return 'unknown'`. PowerShell hashtables ignore case in keys, just like Windows.
@@solution
$staff = @{
    'asilva'  = 'Sales'
    'ktanaka' = 'IT'
    'bjones'  = 'Finance'
}

function Get-Department($user) {
    if ($staff.ContainsKey($user)) { return $staff[$user] }
    return 'unknown'
}

@@lesson e7-dotnet .NET Collections and Types from PowerShell
@@concepts lists, maps, objects
@@text
PowerShell runs on .NET, so every .NET type is available. For small scripts, `@()` and `@{}` are fine. For thousands of items, or when you need a set, a queue or sorting by key, reach for the **generic collections**.

## Why not `+=` on arrays?

PowerShell arrays have a **fixed size**. `$a += $x` creates a new array and copies everything: harmless for 100 items, painfully slow for 100,000 (quadratic time). Use a `List[T]`:

```powershell
$list = [System.Collections.Generic.List[string]]::new()
$list.Add('web01')               # returns nothing (unlike ArrayList)
$list.AddRange([string[]]@('web02', 'db01'))
$list.Remove('web02') | Out-Null
"$($list.Count): $($list -join ', '), index of db01: $($list.IndexOf('db01'))"
```

Or let PowerShell collect loop output for you, which is fast and simple: `$names = foreach ($s in $servers) { $s.Name }`.

## The useful collection types

| Type | Use it for |
|---|---|
| `List[T]` | growing lists |
| `Dictionary[K,V]` | typed hashtable; `TryGetValue` |
| `HashSet[T]` | unique values; fast `Contains`; set operations |
| `Queue[T]` / `Stack[T]` | first-in-first-out / last-in-first-out work lists |
| `SortedDictionary[K,V]` | a dictionary that keeps keys sorted |
| `[ordered]@{}` | an insertion-ordered hashtable (OrderedDictionary) |

```powershell
$seen = [System.Collections.Generic.HashSet[string]]::new([StringComparer]::OrdinalIgnoreCase)
[void]$seen.Add('WEB01'); $added = $seen.Add('web01')
"added again: $added, count: $($seen.Count)"

$q = [System.Collections.Generic.Queue[string]]::new()
$q.Enqueue('first'); $q.Enqueue('second')
$q.Dequeue()
```

Note: .NET string comparisons are **case-sensitive** by default, unlike PowerShell's operators and hashtables. Pass `[StringComparer]::OrdinalIgnoreCase` when keys are user names or host names.

## Calling .NET

```powershell
[math]::Max(3, 7)                                   # static method
[System.IO.Path]::GetExtension('report.final.csv')  # .csv
[guid]::NewGuid().Guid.Length                       # 36
[string]::Join('-', @('a', 'b'))
[System.Net.IPAddress]::Parse('10.0.0.5').GetAddressBytes() -join '.'
[uri]'https://api.example.jp:8443/v1/users?page=2' | Select-Object Host, Port, AbsolutePath, Query
```

- `[Type]::Member` is static; `$obj.Member` is an instance member.
- `New-Object TypeName -ArgumentList ...` and `[TypeName]::new(...)` (5.0+) both construct objects.
- `Get-Member` shows what an object has; `[Type] | Get-Member -Static` shows static members.
- **Type accelerators** are short names: `[int]`, `[regex]`, `[xml]`, `[ipaddress]`, `[mailaddress]`, `[datetime]`, `[pscustomobject]`, `[hashtable]`.

## Generic method gotcha

Generic type names use square brackets inside: `[System.Collections.Generic.Dictionary[string,int]]`. Arrays of a type: `[string[]]`, `[int[]]`. A strongly typed variable converts on assignment: `[int[]]$ports = '22', '80'` gives integers.

@@quiz
Why is `$a += $item` slow in a loop over 100,000 items?
* PowerShell arrays have a fixed size, so each `+=` copies the whole array into a new one
- `+=` checks types
- Arrays are stored on disk
- It's not slow
? Use List[T] or collect the loop's output.

@@quiz
`$set = [System.Collections.Generic.HashSet[string]]::new()`. Is `$set.Contains('WEB01')` true after `$set.Add('web01')`?
- Yes, PowerShell ignores case
* No: .NET comparisons are case-sensitive unless you pass a comparer like OrdinalIgnoreCase
- It throws an error
- Only on Windows
? Pass [StringComparer]::OrdinalIgnoreCase.

@@quiz
Which collection gives fast lookups of a user by name?
* A Dictionary[string, object] (or a hashtable) keyed by name
- An array searched with Where-Object
- A List[string]
- A string joined with commas
? Dictionary lookups take the same time however many items there are.

@@drill
Which .NET generic type is the fast replacement for adding to an array with `+=`?
= List[T]
= System.Collections.Generic.List[T]
= List
= [System.Collections.Generic.List[object]]
? Add() appends without copying the whole array.

@@flashcards
`List[T]` :: growable typed list
`HashSet[T]` :: unique values, fast lookup
`Queue[T]` :: FIFO (Enqueue/Dequeue)
`Stack[T]` :: LIFO (Push/Pop)
`Dictionary[K,V]` :: typed map
`TryGetValue` :: lookup without exceptions
`OrdinalIgnoreCase` :: case-insensitive comparer
`[Type]::new()` :: construct
`[Type]::Member` :: static member
`[uri]`, `[ipaddress]`, `[xml]` :: handy accelerators
`[int[]]$x` :: typed array (converts values)

@@exercise Sets for access reviews
Compare who **should** have access (from HR) with who **does** (from a group export), ignoring case:

- load both into `HashSet[string]` with `OrdinalIgnoreCase`
- compute: users with access who aren't in HR (remove), HR users without access (add), and the overlap, using `ExceptWith` / `IntersectWith` on **copies**
- print each group sorted, and the counts
@@starter
$hr = 'asato', 'bkowalski', 'cnguyen', 'dmüller', 'esmith'
$group = 'ASATO', 'cnguyen', 'DMÜLLER', 'old-contractor', 'testuser'
'TODO'
@@expected
remove access: old-contractor, testuser
grant access: bkowalski, esmith
correct: asato, cnguyen, dmüller
hr 5, group 5, overlap 3
@@requires
HashSet[string]
OrdinalIgnoreCase
ExceptWith
IntersectWith
@@hint
`$cmp = [StringComparer]::OrdinalIgnoreCase; $hrSet = [System.Collections.Generic.HashSet[string]]::new([string[]]$hr, $cmp)`. A copy: `[System.Collections.Generic.HashSet[string]]::new($groupSet, $cmp)`. For "correct", intersect a copy of `$hrSet` so the names come out in HR's spelling. Sort with `Sort-Object` and lower-case the "remove" names with `.ToLower()` for the report.
@@solution
$hr = 'asato', 'bkowalski', 'cnguyen', 'dmüller', 'esmith'
$group = 'ASATO', 'cnguyen', 'DMÜLLER', 'old-contractor', 'testuser'

$cmp = [StringComparer]::OrdinalIgnoreCase
$hrSet = [System.Collections.Generic.HashSet[string]]::new([string[]]$hr, $cmp)
$groupSet = [System.Collections.Generic.HashSet[string]]::new([string[]]$group, $cmp)

$remove = [System.Collections.Generic.HashSet[string]]::new($groupSet, $cmp)
$remove.ExceptWith($hrSet)
$grant = [System.Collections.Generic.HashSet[string]]::new($hrSet, $cmp)
$grant.ExceptWith($groupSet)
$correct = [System.Collections.Generic.HashSet[string]]::new($hrSet, $cmp)
$correct.IntersectWith($groupSet)

"remove access: $(($remove | ForEach-Object { $_.ToLower() } | Sort-Object) -join ', ')"
"grant access: $(($grant | Sort-Object) -join ', ')"
"correct: $(($correct | Sort-Object) -join ', ')"
"hr $($hrSet.Count), group $($groupSet.Count), overlap $($correct.Count)"

@@exercise A work queue with a dictionary cache
Simulate a crawler of internal wiki pages:

- start a `Queue[string]` with `home`; while the queue isn't empty, dequeue a page, skip it if already visited (`HashSet`), and enqueue its links from `$links`
- record each page's depth in a `Dictionary[string,int]` (the depth of the page that linked it + 1, first time only)
- print the visit order with depths, then pages grouped by depth using a `SortedDictionary[int, List[string]]`
- finally parse the wiki's base URL with `[uri]` and print its host, port and path
@@starter
$links = @{
    home     = @('install', 'faq')
    install  = @('windows', 'linux', 'faq')
    faq      = @('contact')
    windows  = @('drivers')
    linux    = @()
    contact  = @('home')
    drivers  = @()
}
$base = 'https://wiki.example.jp:8443/it/docs/'
'TODO'
@@expected
visit order: home(0) install(1) faq(1) windows(2) linux(2) contact(2) drivers(3)
depth 0: home
depth 1: install, faq
depth 2: windows, linux, contact
depth 3: drivers
host wiki.example.jp, port 8443, path /it/docs/
@@requires
Queue[string]
Dictionary[string,int]
SortedDictionary
[uri]
@@hint
Set the depth when you **enqueue** a page for the first time: `if (-not $depth.ContainsKey($next)) { $depth[$next] = $depth[$page] + 1; $queue.Enqueue($next) }`; that way each page is queued once. For grouping: `if (-not $byDepth.ContainsKey($d)) { $byDepth[$d] = [System.Collections.Generic.List[string]]::new() }; $byDepth[$d].Add($page)`.
@@solution
$links = @{
    home     = @('install', 'faq')
    install  = @('windows', 'linux', 'faq')
    faq      = @('contact')
    windows  = @('drivers')
    linux    = @()
    contact  = @('home')
    drivers  = @()
}
$base = 'https://wiki.example.jp:8443/it/docs/'

$queue = [System.Collections.Generic.Queue[string]]::new()
$visited = [System.Collections.Generic.HashSet[string]]::new()
$depth = [System.Collections.Generic.Dictionary[string,int]]::new()
$order = [System.Collections.Generic.List[string]]::new()

$queue.Enqueue('home')
$depth['home'] = 0
while ($queue.Count -gt 0) {
    $page = $queue.Dequeue()
    if (-not $visited.Add($page)) { continue }
    $order.Add("$page($($depth[$page]))")
    foreach ($next in $links[$page]) {
        if (-not $depth.ContainsKey($next)) {
            $depth[$next] = $depth[$page] + 1
            $queue.Enqueue($next)
        }
    }
}
"visit order: $($order -join ' ')"

$byDepth = [System.Collections.Generic.SortedDictionary[int, System.Collections.Generic.List[string]]]::new()
foreach ($entry in $order) {
    $page = $entry -replace '\(\d+\)$', ''
    $d = $depth[$page]
    if (-not $byDepth.ContainsKey($d)) { $byDepth[$d] = [System.Collections.Generic.List[string]]::new() }
    $byDepth[$d].Add($page)
}
foreach ($kv in $byDepth.GetEnumerator()) { "depth $($kv.Key): $($kv.Value -join ', ')" }

$u = [uri]$base
"host $($u.Host), port $($u.Port), path $($u.AbsolutePath)"

@@lesson p1-loops Loops
@@concepts loops
@@text
## foreach: the one you'll use most

```powershell
$servers = 'web01', 'web02', 'db01'
foreach ($s in $servers) {
    "Checking $s..."
}
```

## ForEach-Object in a pipeline

```powershell
1..5 | ForEach-Object { $_ * $_ }
```

`1..5` is a **range**: the numbers 1 to 5. `foreach (...)` and `ForEach-Object` do the same job: `foreach` is a bit faster on data you already have in a variable, `ForEach-Object` handles objects one at a time as they stream out of a command.

## for, while, do

```powershell
for ($i = 1; $i -le 3; $i++) { "Attempt $i" }

$tries = 0
while ($tries -lt 3) {
    $tries++
    "Ping attempt $tries"
}

do {
    $n = Get-Random -Minimum 1 -Maximum 7
    "Rolled $n"
} until ($n -eq 6)
```

`break` leaves a loop, `continue` skips to the next round.

## Collecting results

Anything a loop outputs can be caught in a variable, which is the cleanest way to build a list:

```powershell
$squares = foreach ($n in 1..5) { $n * $n }
$squares -join ', '
```

This is better than `$list += $item` inside a loop, which copies the whole array on every addition (slow with thousands of items).

## A real retry loop

Admins write this all the time: try something a few times, wait between tries.

```powershell,norun
$ok = $false
for ($i = 1; $i -le 5 -and -not $ok; $i++) {
    $ok = Test-Connection -ComputerName 'db01' -Count 1 -Quiet
    if (-not $ok) { Start-Sleep -Seconds 2 }
}
if ($ok) { "db01 is up" } else { "db01 is still down after 5 tries" }
```

@@quiz
What does `1..4 | ForEach-Object { $_ * 10 }` output?
* 10, 20, 30, 40 (one per line)
- 1, 2, 3, 4
- 100
- 10
? Each number from the range is multiplied by 10.

@@quiz
Why is `$results = foreach (...) { ... }` better than `$results += ...` inside the loop?
- It isn't, they're the same
* `+=` creates a new copy of the array each time, which gets slow with many items
- `+=` doesn't work on arrays
- `foreach` sorts the results
? Arrays have a fixed size; `+=` builds a new one every time.

@@exercise Server list
Print `Pinging <name>` for each server in `$servers`, then a final line `Done: 3 servers` (use the count, don't type 3).
@@starter
$servers = 'web01', 'web02', 'db01'

@@expected
Pinging web01
Pinging web02
Pinging db01
Done: 3 servers
@@hint
A `foreach` loop, and after it `"Done: $($servers.Count) servers"`.
@@solution
$servers = 'web01', 'web02', 'db01'
foreach ($s in $servers) {
    "Pinging $s"
}
"Done: $($servers.Count) servers"

@@exercise Sum of log sizes
Write a function `Get-TotalMB` that takes an array of file sizes in **bytes** and returns their total in **MB**, rounded to 1 decimal. Use a loop (no `Measure-Object` this time).
@@starter
function Get-TotalMB($sizes) {
    throw "TODO: not done yet"
}

@@tests
check (Get-TotalMB @(1048576, 1048576, 524288)) 2.5 "Get-TotalMB 1MB, 1MB, 0.5MB"
check (Get-TotalMB @(1000000, 2500000)) 3.3 "Get-TotalMB 1000000, 2500000"
check (Get-TotalMB @()) 0 "Get-TotalMB (empty)"
@@hint
Start with `$total = 0`, add each size in a `foreach`, then `[math]::Round($total / 1MB, 1)`.
@@solution
function Get-TotalMB($sizes) {
    $total = 0
    foreach ($s in $sizes) {
        $total += $s
    }
    return [math]::Round($total / 1MB, 1)
}

@@exercise FizzBuzz, admin edition
For the numbers 1 to 15: print `ALERT` for multiples of both 3 and 5, `disk` for multiples of 3, `cpu` for multiples of 5, otherwise the number.
@@starter
# 1..15

@@expected
1
2
disk
4
cpu
disk
7
8
disk
cpu
11
disk
13
14
ALERT
@@hint
Check the "both" case (`$n % 15 -eq 0`) first.
@@solution
foreach ($n in 1..15) {
    if ($n % 15 -eq 0) { 'ALERT' }
    elseif ($n % 3 -eq 0) { 'disk' }
    elseif ($n % 5 -eq 0) { 'cpu' }
    else { $n }
}

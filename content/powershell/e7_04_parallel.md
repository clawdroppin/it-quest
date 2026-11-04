@@lesson e7-parallel Doing Things in Parallel: Jobs, Runspaces and -Parallel
@@concepts concurrency, automation
@@text
Checking 500 servers one by one at 2 seconds each takes 17 minutes. Checking 50 at a time takes 20 seconds. PowerShell has several ways to run work concurrently, with different costs.

| Method | Version | Runs in | Start cost | Notes |
|---|---|---|---|---|
| `Start-Job` | 2.0+ | a new **process** | ~0.5–1 s each | isolated, survives errors, slow |
| `Start-ThreadJob` | 7 (module for 5.1) | a thread | ms | job cmdlets, lightweight |
| `ForEach-Object -Parallel` | 7.0+ | runspaces | ms | `-ThrottleLimit`, `$using:` |
| `Invoke-Command -ComputerName a,b,c` | 2.0+ | remote machines | network | runs on targets in parallel (32 by default) |
| Runspace pool | 2.0+ (.NET API) | threads | ms | most control, works in 5.1 |

## Jobs

```powershell,norun
$jobs = foreach ($s in $servers) {
    Start-Job -Name $s -ScriptBlock { param($name) Test-Connection $name -Count 1 -Quiet } -ArgumentList $s
}
$jobs | Wait-Job -Timeout 60 | Receive-Job
$jobs | Remove-Job -Force
```

A job doesn't see your variables: pass data with `-ArgumentList` or `$using:var`. Results come back **serialised** (deserialised objects have properties but no methods).

## ForEach-Object -Parallel (PowerShell 7)

```powershell,norun
$servers | ForEach-Object -Parallel {
    [pscustomobject]@{ Server = $_; Up = Test-Connection $_ -Count 1 -Quiet -TimeoutSeconds 2 }
} -ThrottleLimit 20
```

## Runspace pools: parallelism in Windows PowerShell 5.1

A **runspace** is an isolated PowerShell engine in the same process. A pool runs several on threads:

```powershell
$pool = [runspacefactory]::CreateRunspacePool(1, 4)    # min 1, max 4 at a time
$pool.Open()
$tasks = foreach ($n in 1..6) {
    $ps = [powershell]::Create().AddScript({ param($x) Start-Sleep -Milliseconds 100; $x * $x }).AddArgument($n)
    $ps.RunspacePool = $pool
    [pscustomobject]@{ Input = $n; Shell = $ps; Handle = $ps.BeginInvoke() }
}
$results = foreach ($t in $tasks) {
    "$($t.Input)^2 = $($t.Shell.EndInvoke($t.Handle))"
    $t.Shell.Dispose()
}
$pool.Close()
$results -join '; '
```

`BeginInvoke()` starts the work and returns immediately; `EndInvoke()` waits for that task and returns its output. Errors inside a runspace don't appear on your screen: read `$ps.Streams.Error` or `$ps.HadErrors`.

## Sharing state safely

Runspaces don't share variables. To collect results from several threads into one place, use a **thread-safe** collection:

```powershell,norun
$bag = [System.Collections.Concurrent.ConcurrentBag[object]]::new()
$sync = [hashtable]::Synchronized(@{})
$ps.AddScript({ param($bag) $bag.Add('done') }).AddArgument($bag)
```

A normal `List[T]` or hashtable updated from two threads at once can lose items or throw. Better still: have each task **return** its result and combine them afterwards, as the example above does.

## How many at once?

- CPU-bound work: about the number of cores.
- Waiting on the network (pings, REST calls, WinRM): 20–100 is common.
- Respect the other side: an API's rate limit or a domain controller's load.

## Order

Parallel results finish in any order. Keep the input with each result and **sort** afterwards (or collect results in input order with `EndInvoke` as above).

@@quiz
Why is `Start-Job` slow for 500 tiny tasks?
* Each job starts a new PowerShell process, which costs about half a second or more
- Jobs run one at a time
- It waits for the network
- Jobs are limited to 5
? Use runspaces or thread jobs for many small tasks.

@@quiz
How does code in a runspace get a value from your script?
- It sees all your variables automatically
* You pass it in, e.g. `AddArgument()` (or `$using:` with jobs and -Parallel)
- Through $global:
- Through environment variables only
? Runspaces are isolated engines.

@@quiz
What's the safest way to collect results from parallel tasks?
- Append to a shared List[T] from every thread
* Have each task return its result and combine them after EndInvoke (or use a concurrent collection)
- Write to the same file from every task
- Use $global:results
? Shared non-thread-safe collections can lose data.

@@flashcards
`Start-Job` :: background job in a new process
`Receive-Job` :: get a job's output
`$using:var` :: pass a value into a job/-Parallel
`ForEach-Object -Parallel` :: PS 7 parallel loop
`-ThrottleLimit` :: max concurrent tasks
runspace :: isolated engine, same process
`CreateRunspacePool(1, N)` :: thread pool for runspaces
`BeginInvoke` / `EndInvoke` :: start / wait for result
`$ps.Streams.Error` :: errors from a runspace
`ConcurrentBag` :: thread-safe collection
`[hashtable]::Synchronized()` :: thread-safe hashtable

@@exercise A parallel port scanner (simulated)
Check 12 hosts with a runspace pool of size 4. The "check" sleeps for the host's latency and reports `up` or `down` from a table passed in as an argument:

- create one `[powershell]` per host with `AddScript` and `AddArgument` (the host name and the status table)
- start all with `BeginInvoke`, collect with `EndInvoke`, dispose each instance and close the pool
- print the results sorted by host name, the number up, and whether the total time was well below the sequential time (sum of latencies) (print only `faster than sequential: True`, since exact times vary)
@@starter
$hosts = 1..12 | ForEach-Object { 'srv{0:D2}' -f $_ }
$status = @{ srv03 = 'down'; srv07 = 'down'; srv11 = 'down' }
$latencyMs = 150
'TODO'
@@expected
srv01 up
srv02 up
srv03 down
srv04 up
srv05 up
srv06 up
srv07 down
srv08 up
srv09 up
srv10 up
srv11 down
srv12 up
9 of 12 up
faster than sequential: True
@@requires
CreateRunspacePool
BeginInvoke
EndInvoke
Dispose
@@hint
Script: `{ param($name, $table, $ms) Start-Sleep -Milliseconds $ms; $s = 'up'; if ($table.ContainsKey($name)) { $s = $table[$name] }; [pscustomobject]@{ Host = $name; Status = $s } }`. Time the whole run with `[System.Diagnostics.Stopwatch]::StartNew()`. Sequential time = `$hosts.Count * $latencyMs`; with 4 at a time it should take about a third.
@@solution
$hosts = 1..12 | ForEach-Object { 'srv{0:D2}' -f $_ }
$status = @{ srv03 = 'down'; srv07 = 'down'; srv11 = 'down' }
$latencyMs = 150

$check = {
    param($name, $table, $ms)
    Start-Sleep -Milliseconds $ms
    $s = 'up'
    if ($table.ContainsKey($name)) { $s = $table[$name] }
    [pscustomobject]@{ Host = $name; Status = $s }
}

$watch = [System.Diagnostics.Stopwatch]::StartNew()
$pool = [runspacefactory]::CreateRunspacePool(1, 4)
$pool.Open()
$tasks = foreach ($h in $hosts) {
    $ps = [powershell]::Create().AddScript($check).AddArgument($h).AddArgument($status).AddArgument($latencyMs)
    $ps.RunspacePool = $pool
    [pscustomobject]@{ Shell = $ps; Handle = $ps.BeginInvoke() }
}
$results = foreach ($t in $tasks) {
    $t.Shell.EndInvoke($t.Handle)
    $t.Shell.Dispose()
}
$pool.Close()
$watch.Stop()

$results | Sort-Object Host | ForEach-Object { "$($_.Host) $($_.Status)" }
"$(@($results | Where-Object Status -eq 'up').Count) of $($hosts.Count) up"
"faster than sequential: $($watch.ElapsedMilliseconds -lt ($hosts.Count * $latencyMs * 0.75))"

@@exercise Errors and shared state across runspaces
Run 8 tasks in a pool of 3. Each task adds its id to a `ConcurrentBag[int]`, and tasks 3 and 6 fail with `throw`:

- after `EndInvoke`, check each instance's `HadErrors` and read the message from `Streams.Error` (failed tasks still finish)
- print `task N failed: MESSAGE` for failures, sorted by id
- print the sorted contents of the bag (all tasks add their id **before** failing)
- show that a variable changed inside a runspace isn't changed in your script
@@starter
$counter = 0
'TODO'
@@expected
task 3 failed: task 3 broke
task 6 failed: task 6 broke
bag: 1 2 3 4 5 6 7 8
succeeded: 6
counter in main script: 0
@@requires
ConcurrentBag
HadErrors
Streams.Error
@@hint
Script: `{ param($id, $bag) $bag.Add($id); $counter = 99; if ($id -in 3, 6) { throw "task $id broke" }; "ok $id" }`. A `throw` inside a runspace ends that task; `EndInvoke` may then throw too, so wrap it in `try { } catch { }` and look at `HadErrors` afterwards. The error message is `$ps.Streams.Error[0].Exception.Message`, or if EndInvoke threw, use the caught exception's `InnerException`.
@@solution
$counter = 0
$bag = [System.Collections.Concurrent.ConcurrentBag[int]]::new()
$work = {
    param($id, $bag)
    $bag.Add($id)
    $counter = 99
    if ($id -in 3, 6) { throw "task $id broke" }
    "ok $id"
}

$pool = [runspacefactory]::CreateRunspacePool(1, 3)
$pool.Open()
$tasks = foreach ($id in 1..8) {
    $ps = [powershell]::Create().AddScript($work).AddArgument($id).AddArgument($bag)
    $ps.RunspacePool = $pool
    [pscustomobject]@{ Id = $id; Shell = $ps; Handle = $ps.BeginInvoke() }
}

$ok = 0
$failures = foreach ($t in $tasks) {
    $message = $null
    try {
        $out = $t.Shell.EndInvoke($t.Handle)
    } catch {
        $inner = $_.Exception.InnerException
        if ($inner) { $message = $inner.Message } else { $message = $_.Exception.Message }
    }
    if (-not $message -and $t.Shell.HadErrors -and $t.Shell.Streams.Error.Count -gt 0) {
        $message = $t.Shell.Streams.Error[0].Exception.Message
    }
    if ($message) { [pscustomobject]@{ Id = $t.Id; Message = $message } } else { $ok++ }
    $t.Shell.Dispose()
}
$pool.Close()

$failures | Sort-Object Id | ForEach-Object { "task $($_.Id) failed: $($_.Message)" }
"bag: $(($bag.ToArray() | Sort-Object) -join ' ')"
"succeeded: $ok"
"counter in main script: $counter"

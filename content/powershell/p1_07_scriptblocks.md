@@lesson p1-scriptblocks Script Blocks: Code as a Value
@@concepts functions, variables
@@text
A **script block** is code in braces: `{ ... }`. You've already used them with `Where-Object { }`, `ForEach-Object { }` and `if`. They're also values you can store in variables, pass to functions and call later, PowerShell's version of lambdas or anonymous functions.

## Creating and calling

```powershell
$greet = { param($name) "Hello, $name" }
& $greet 'Aiko'               # call operator
$greet.Invoke('Ben')          # .NET-style call (returns a collection)
$square = { $args[0] * $args[0] }
& $square 7
```

`&` runs the block in a **child scope**: variables it creates disappear afterwards. The dot-source operator `.` runs it in the **current** scope instead, so its variables stay:

```powershell
$x = 1
& { $x = 2 }; "after &: $x"
. { $x = 3 }; "after .: $x"
```

## $_ and $PSItem

Inside pipeline script blocks, `$_` (or `$PSItem`) is the current object. In your own script blocks you can provide it with `ForEach-Object`, or accept parameters with `param()`.

## Passing behaviour to functions

```powershell
function Invoke-WithRetry {
    param([scriptblock]$Action, [int]$Times = 3)
    for ($i = 1; $i -le $Times; $i++) {
        try { return & $Action }
        catch { "attempt $i failed: $($_.Exception.Message)" | Write-Host }
    }
    throw "gave up after $Times attempts"
}
```

Callers decide **what** to do; the function decides **how** (retry, time, log). The same pattern gives you `Measure-Command { }`, `Invoke-Command -ScriptBlock { }` (remoting) and `Start-Job { }`.

## Closures

A script block normally looks up variables **when it runs**. `GetNewClosure()` captures their values **now**:

```powershell
$blocks = foreach ($n in 1..3) { { "late: $n" } }
$closures = foreach ($n in 1..3) { { "captured: $n" }.GetNewClosure() }
& $blocks[0]       # late: 3  ($n is 3 when it finally runs)
& $closures[0]     # captured: 1
```

## Script blocks in hashtables: a dispatch table

Instead of a long `switch`, map names to actions:

```powershell
$actions = @{
    start  = { param($svc) "starting $svc" }
    stop   = { param($svc) "stopping $svc" }
}
& $actions['start'] 'Spooler'
```

## Script blocks as parameters to cmdlets

Several cmdlets accept a script block where you might expect a value, like calculated properties and `Sort-Object { $_.Name.Length }`. Some parameters even accept a block that's evaluated **per pipeline object** (delay-bind script blocks): `Get-ChildItem *.log | Rename-Item -NewName { $_.Name -replace '\.log$', '.old' }`.

@@quiz
What's the difference between `& $block` and `. $block`?
* `&` runs in a child scope (variables don't leak); `.` runs in the current scope
- `.` is faster
- `&` only works with files
- There's no difference
? Dot-sourcing is also how scripts load functions into your session.

@@quiz
Why does `{ "n is $n" }` print the last value of `$n` when called after a loop?
- Script blocks copy variables
* The variable is looked up when the block runs, not when it's created; use GetNewClosure() to capture it
- Loops reset variables
- It's a bug in PowerShell
? Late binding vs closures.

@@flashcards
`{ ... }` :: a script block
`& $sb` :: call in a child scope
`. $sb` :: call in the current scope
`param()` in a block :: named arguments
`$args` :: unnamed arguments
`.GetNewClosure()` :: capture current variable values
`[scriptblock]` :: parameter type for code
dispatch table :: hashtable of script blocks
delay-bind script block :: `-NewName { ... }` per object

@@exercise A retry helper and a timer
Write two higher-order functions:

- `Invoke-WithRetry -Action <scriptblock> -Times <int>`: runs the action until it succeeds, printing `attempt N failed: MESSAGE` with `Write-Output` for each failure; after the last failure it returns `gave up`
- `Measure-Calls -Action <scriptblock> -Inputs <array>`: calls the action for each input with `& $Action $item`, collects the results and returns `N calls: r1, r2, ...`

A flaky action fails twice before succeeding: use a counter in a hashtable (`$state.Count++`), since a plain variable changed inside `&` would be lost.
@@starter
$state = @{ Count = 0 }
$flaky = {
    $state.Count++
    if ($state.Count -lt 3) { throw "timeout #$($state.Count)" }
    "connected on try $($state.Count)"
}
'TODO'
@@expected
attempt 1 failed: timeout #1
attempt 2 failed: timeout #2
connected on try 3
attempt 1 failed: always broken
attempt 2 failed: always broken
gave up
3 calls: 4, 9, 16
@@requires
[scriptblock]
& $Action
@@hint
`for ($i = 1; $i -le $Times; $i++) { try { & $Action; return } catch { "attempt $i failed: $($_.Exception.Message)" } }` then `'gave up'`. Using `& $Action; return` emits the action's output and then leaves. A hashtable is a reference, so `$state.Count++` inside the block updates the same object.
@@solution
$state = @{ Count = 0 }
$flaky = {
    $state.Count++
    if ($state.Count -lt 3) { throw "timeout #$($state.Count)" }
    "connected on try $($state.Count)"
}

function Invoke-WithRetry {
    param([scriptblock]$Action, [int]$Times = 3)
    for ($i = 1; $i -le $Times; $i++) {
        try {
            & $Action
            return
        } catch {
            "attempt $i failed: $($_.Exception.Message)"
        }
    }
    'gave up'
}

function Measure-Calls {
    param([scriptblock]$Action, [object[]]$Inputs)
    $results = foreach ($item in $Inputs) { & $Action $item }
    "$($Inputs.Count) calls: $($results -join ', ')"
}

Invoke-WithRetry -Action $flaky -Times 5
Invoke-WithRetry -Action { throw 'always broken' } -Times 2
Measure-Calls -Action { param($n) $n * $n } -Inputs 2, 3, 4

@@exercise A command dispatcher with closures
Build a tiny command interpreter for a help-desk bot:

- a dispatch table `$commands` mapping `ping`, `disk`, `whoami` and `help` to script blocks that take `$arg`
- `help` lists the command names, sorted
- unknown commands print `unknown command: X (try help)`
- create three "greeter" script blocks in a loop over `'Aiko','Ben','Chen'`, once **without** and once **with** `GetNewClosure()`, and call the first of each to show the difference
@@starter
$disks = @{ 'C' = 41; 'D' = 87 }
$inputLines = 'ping web01', 'disk D', 'whoami', 'reboot now', 'help'
'TODO'
@@expected
> ping web01
pong from web01
> disk D
drive D: 87% used
> whoami
helpdesk-bot
> reboot now
unknown command: reboot (try help)
> help
commands: disk, help, ping, whoami
late binding: hello Chen
closure: hello Aiko
@@requires
GetNewClosure
ContainsKey
& $
@@hint
Split each line with `$name, $arg = $line -split ' ', 2`. Inside `help`, `$commands.Keys | Sort-Object` works because the block reads `$commands` when it runs. For the greeters: `$late = foreach ($n in 'Aiko','Ben','Chen') { { "hello $n" } }`, then call `& $late[0]` after the loop.
@@solution
$disks = @{ 'C' = 41; 'D' = 87 }
$inputLines = 'ping web01', 'disk D', 'whoami', 'reboot now', 'help'

$commands = @{
    ping   = { param($arg) "pong from $arg" }
    disk   = { param($arg) "drive ${arg}: $($disks[$arg])% used" }
    whoami = { param($arg) 'helpdesk-bot' }
    help   = { param($arg) "commands: $(($commands.Keys | Sort-Object) -join ', ')" }
}

foreach ($line in $inputLines) {
    "> $line"
    $name, $arg = $line -split ' ', 2
    if ($commands.ContainsKey($name)) {
        & $commands[$name] $arg
    } else {
        "unknown command: $name (try help)"
    }
}

$late = foreach ($n in 'Aiko', 'Ben', 'Chen') { { "hello $n" } }
$captured = foreach ($n in 'Aiko', 'Ben', 'Chen') { { "hello $n" }.GetNewClosure() }
"late binding: $(& $late[0])"
"closure: $(& $captured[0])"

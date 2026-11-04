@@lesson e7-streams Output, Streams and Unrolling
@@concepts functions, lists
@@text
PowerShell functions don't "return" like functions in other languages. **Everything** a function produces and doesn't capture goes to the output stream. Combined with automatic **unrolling** of collections, this explains most "why does my function return an array?" questions.

## Everything is output

```powershell
function Get-Total {
    $list = [System.Collections.ArrayList]::new()
    $list.Add(10)        # .Add returns the new index: 0 goes to output!
    $list.Add(20)        # 1 goes to output too
    return 30
}
(Get-Total) -join ','     # 0,1,30
```

Silence unwanted output with `[void]$list.Add(10)`, `$null = ...` or `... | Out-Null` (slowest). `return 30` is just "output 30 and leave".

## The six streams

| # | Stream | Write with | Shown by default? |
|---|---|---|---|
| 1 | Success (output) | `Write-Output` or just a value | yes, and captured by `$x = ...` |
| 2 | Error | `Write-Error`, failing cmdlets | yes (red) |
| 3 | Warning | `Write-Warning` | yes |
| 4 | Verbose | `Write-Verbose` | only with `-Verbose` |
| 5 | Debug | `Write-Debug` | only with `-Debug` |
| 6 | Information | `Write-Information`, `Write-Host` | Write-Host yes |

Only stream 1 is **data**. Use the others for messages so that callers can capture clean results: `$users = Get-StaleUser` shouldn't contain progress text.

`Write-Host` writes to the information stream (since 5.0): it shows on screen, isn't captured by `$x = ...`, and can be redirected with `6>`.

## Redirecting streams

```powershell,norun
.\deploy.ps1 2> errors.txt          # errors to a file
.\deploy.ps1 3>&1 | Out-File all.txt  # merge warnings into output
.\deploy.ps1 *> everything.log       # all streams
$result = Get-Thing 4>&1              # capture verbose messages as objects too
```

Captured non-output records keep their type: `$x | Where-Object { $_ -is [System.Management.Automation.WarningRecord] }`.

## Unrolling

When a function outputs a collection, PowerShell sends its **items** one by one down the pipeline. The caller collects them again into a new array, or a single object if there was one, or `$null` if there were none:

```powershell
function Get-Many { @('a', 'b') }
function Get-One { @('a') }
function Get-None { @() }
"many: $((Get-Many).GetType().Name), one: $((Get-One).GetType().Name), none is null: $($null -eq (Get-None))"
```

Two fixes:

- **Caller side**: wrap with `@( )`: `$items = @(Get-One)` is always an array. (Best practice.)
- **Function side**: `return ,$list` (the unary comma wraps it in a one-item array, which is unrolled to the original) or `Write-Output -NoEnumerate $list`.

## $PSCmdlet and the pipeline

In advanced functions, `$PSCmdlet.WriteObject($x, $true)` writes and enumerates, and `$PSCmdlet.WriteVerbose()` etc. mirror the cmdlets. You'll rarely need them, but they explain the model: cmdlets write records to streams; the host decides what to show.

@@quiz
Why does `$list.Add('x')` inside a function add numbers to the function's result?
* ArrayList.Add returns the new index, and every uncaptured value becomes output
- It's a bug in ArrayList
- return adds them
- Because of strict mode
? Use [void], $null = or a generic List[T] (whose Add returns nothing).

@@quiz
A function returns a one-item array. What does the caller receive with `$r = Get-One`?
- A one-item array
* The single item itself (the array was unrolled)
- $null
- An error
? Use `$r = @(Get-One)` to always get an array.

@@quiz
Which stream should progress messages go to, so `$data = Get-Report` captures only data?
- Output (1)
* Verbose or Information (4 or 6), e.g. Write-Verbose / Write-Information
- Error (2)
- They can't be separated
? Only the success stream is captured by assignment.

@@flashcards
stream 1 :: success / output (data)
stream 2 :: error
stream 3 :: warning
stream 4 :: verbose
stream 5 :: debug
stream 6 :: information (Write-Host)
`[void]` / `$null =` :: discard output
`*>` :: redirect all streams
`3>&1` :: merge warnings into output
unrolling :: collections are output item by item
`@(...)` :: always an array
`return ,$x` :: return a collection intact

@@exercise Fix the leaky function
`Get-Inventory` is supposed to return exactly the list of server names, but its output is polluted. Rewrite it so that:

- the list operations no longer leak index numbers
- progress text goes to the verbose stream (`Write-Verbose`), and summary text to the information stream (`Write-Information`)
- the caller gets an **array** even when only one server matches (`@( )`)

Then show: the number of items and their names for two filters, that the verbose messages appear only with `-Verbose` merged into output (`4>&1`), and that the information message can be captured with `6>&1` (count the InformationRecord objects).
@@starter
function Get-Inventory {
    param([string]$Filter = '*')
    $all = [System.Collections.ArrayList]::new()
    $all.Add('web01'); $all.Add('web02'); $all.Add('db01')
    "scanning for $Filter"
    $found = $all | Where-Object { $_ -like $Filter }
    "found $(@($found).Count)"
    $found
}
'TODO'
@@expected
all: 3 item(s): web01, web02, db01
db only: 1 item(s), type Object[]
verbose messages: VERBOSE: scanning for web*
information records: 1
@@requires
[void]
Write-Verbose
Write-Information
[CmdletBinding()]
@@hint
Add `[CmdletBinding()]` so the function gets `-Verbose`. Use `[void]$all.Add(...)`. Capture verbose output with `$v = Get-Inventory -Filter 'web*' -Verbose 4>&1 | Where-Object { $_ -is [System.Management.Automation.VerboseRecord] }` and print `"VERBOSE: $($v.Message)"`. For information records, `6>&1` and test for `[System.Management.Automation.InformationRecord]`.
@@solution
function Get-Inventory {
    [CmdletBinding()]
    param([string]$Filter = '*')
    $all = [System.Collections.ArrayList]::new()
    [void]$all.Add('web01'); [void]$all.Add('web02'); [void]$all.Add('db01')
    Write-Verbose "scanning for $Filter"
    $found = @($all | Where-Object { $_ -like $Filter })
    Write-Information "found $($found.Count)"
    $found
}

$all = @(Get-Inventory)
"all: $($all.Count) item(s): $($all -join ', ')"
$db = @(Get-Inventory -Filter 'db*')
"db only: $($db.Count) item(s), type $($db.GetType().Name)"
$v = Get-Inventory -Filter 'web*' -Verbose 4>&1 | Where-Object { $_ -is [System.Management.Automation.VerboseRecord] }
"verbose messages: VERBOSE: $($v.Message)"
$info = Get-Inventory 6>&1 | Where-Object { $_ -is [System.Management.Automation.InformationRecord] }
"information records: $(@($info).Count)"

@@exercise Unrolling on purpose
Show how collections travel through output:

- `Get-Pair` returns a 2-item array, `Get-Single` a 1-item array, `Get-Empty` an empty array: print the caller's type for each (or `null`)
- `Get-Wrapped` returns its 2-item list with the unary comma (`,$list`), so a single pipeline object arrives: count pipeline items for `Get-Pair` and `Get-Wrapped` with `Measure-Object`
- `Get-Table` returns a list of lists (rows); make sure the caller gets **3 rows of 2 cells**, not 6 loose cells
@@starter
'TODO'
@@expected
pair: Object[]
single: String
empty: null
pipeline items from Get-Pair: 2
pipeline items from Get-Wrapped: 1
rows: 3, first row: id,name, cells in row 2: 2
@@requires
,$
Measure-Object
@@hint
`function Get-Wrapped { $list = 'a', 'b'; ,$list }`. For the table, build each row as `,@('1', 'web01')` inside the array literal: `@( ,@('id','name'), ,@('1','web01'), ,@('2','db01') )` or add rows to a `[System.Collections.Generic.List[object]]` and return it with a leading comma. Describe a value: `if ($null -eq $v) { 'null' } else { $v.GetType().Name }`.
@@solution
function Get-Pair { @('a', 'b') }
function Get-Single { @('a') }
function Get-Empty { @() }
function Get-Wrapped { $list = 'a', 'b'; ,$list }
function Get-Table {
    $rows = [System.Collections.Generic.List[object]]::new()
    $rows.Add(@('id', 'name'))
    $rows.Add(@('1', 'web01'))
    $rows.Add(@('2', 'db01'))
    ,$rows
}
function Describe($v) { if ($null -eq $v) { 'null' } else { $v.GetType().Name } }

"pair: $(Describe (Get-Pair))"
"single: $(Describe (Get-Single))"
"empty: $(Describe (Get-Empty))"
"pipeline items from Get-Pair: $((Get-Pair | Measure-Object).Count)"
"pipeline items from Get-Wrapped: $((Get-Wrapped | Measure-Object).Count)"
$table = Get-Table
"rows: $($table.Count), first row: $($table[0] -join ','), cells in row 2: $($table[1].Count)"

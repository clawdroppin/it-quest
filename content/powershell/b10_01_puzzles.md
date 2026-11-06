@@lesson b10-puzzles 🎓 Beyond IT: Puzzle Solvers and Backtracking
@@concepts lists, functions, loops
@@text
> **Beyond IT scope.** Puzzle solving is classic computer science, not an admin skill. It's here because recursion and backtracking are a great workout, and the same search ideas appear in schedulers and configuration solvers.

## Backtracking

Many puzzles have the same shape: fill slots one at a time, each with a choice; if a choice leads to a dead end, **undo it and try the next**. In code:

```text
solve(state):
    if no empty slot: return success
    slot = pick an empty slot
    for each candidate value allowed in slot:
        place it
        if solve(state): return success
        remove it               ← backtrack
    return failure
```

The search tree can be huge, so two tricks make it fast:

- **Constraint checking early**: reject a value as soon as it breaks a rule, not after filling the whole board.
- **Most constrained first**: pick the empty slot with the **fewest** candidates. A slot with one candidate is forced; a slot with zero means backtrack now.

## Sudoku

A 9×9 grid; each row, column and 3×3 box must contain 1–9 once. Store the grid as an `int[]` of 81 cells (index = row × 9 + column); the box of a cell starts at row `3 * [math]::Floor(r / 3)`. Candidates for a cell = 1..9 minus the values already in its row, column and box.

## N-Queens

Place N queens on an N×N board so none attack each other. Place one queen per row; for each row try each column that isn't attacked (same column, or same diagonal: `|r1 - r2| == |c1 - c2|`). There are 92 solutions for 8 queens.

## Recursion in PowerShell

Recursion works, but each call costs more than in compiled languages, and very deep recursion can exhaust the stack (around a few thousand levels). Backtracking depth equals the number of slots (81 for Sudoku), which is fine. For performance, use typed arrays (`[int[]]`) and avoid the pipeline inside the search.

@@quiz
What does "backtrack" mean in a search?
* Undo the last choice and try the next alternative after reaching a dead end
- Restart from the beginning
- Go back to the previous puzzle
- Sort the candidates
? Depth-first search with undo.

@@quiz
Why choose the empty cell with the fewest candidates first?
- It's always the top-left cell
* It forces moves early and detects dead ends sooner, shrinking the search tree dramatically
- It uses less memory
- It's required for correctness
? Also called the "minimum remaining values" heuristic.

@@quiz
Why does N-Queens backtracking only need to check columns and diagonals, not rows?
* The search places exactly one queen per row, so two queens can never share a row
- Rows are checked automatically by PowerShell
- Queens can share rows
- Rows are checked at the very end
? Building the solution row by row removes one whole class of conflicts.

@@flashcards
backtracking :: try, recurse, undo on failure
constraint check :: reject invalid choices early
MRV heuristic :: most constrained slot first
`r * 9 + c` :: cell index in a flat Sudoku grid
box start :: `3 * floor(r / 3)`
N-Queens :: no shared row, column or diagonal
92 :: solutions for 8 queens

@@exercise Solve a Sudoku
Write `Solve-Sudoku` with backtracking and the most-constrained-cell heuristic:

- parse the puzzle (81 characters, `.` for empty) into an `[int[]]`
- `Get-Candidates $grid $index` returns the values allowed in a cell
- the solver picks the empty cell with the fewest candidates, tries each, and undoes on failure; count the guesses (placements)
- print the solved grid as 9 lines with spaces between 3×3 boxes, whether it's valid, and the number of placements
@@starter
$puzzle = '53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79'
'TODO'
@@expected
534 678 912
672 195 348
198 342 567
859 761 423
426 853 791
713 924 856
961 537 284
287 419 635
345 286 179
valid: True
placements: 51
@@requires
function Get-Candidates
[int[]]
return $false
@@hint
`function Get-Candidates([int[]]$g, [int]$i) { $r = [math]::Floor($i / 9); $c = $i % 9; $used = @{}; for ($k = 0; $k -lt 9; $k++) { $used[$g[$r * 9 + $k]] = 1; $used[$g[$k * 9 + $c]] = 1 }; $br = 3 * [math]::Floor($r / 3); $bc = 3 * [math]::Floor($c / 3); ... ; 1..9 | Where-Object { -not $used.ContainsKey($_) } }`. In the solver, scan all cells to find the empty one with the fewest candidates; if none is empty, return `$true`; if the best has 0 candidates, return `$false`. Increment a `$script:placements` counter for each value you place.
@@solution
$puzzle = '53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79'

function Get-Candidates([int[]]$g, [int]$i) {
    $r = [math]::Floor($i / 9)
    $c = $i % 9
    $used = @{}
    for ($k = 0; $k -lt 9; $k++) {
        $used[$g[$r * 9 + $k]] = 1
        $used[$g[$k * 9 + $c]] = 1
    }
    $br = 3 * [math]::Floor($r / 3)
    $bc = 3 * [math]::Floor($c / 3)
    for ($dr = 0; $dr -lt 3; $dr++) {
        for ($dc = 0; $dc -lt 3; $dc++) { $used[$g[($br + $dr) * 9 + $bc + $dc]] = 1 }
    }
    foreach ($v in 1..9) { if (-not $used.ContainsKey($v)) { $v } }
}

function Solve-Sudoku([int[]]$g) {
    $best = -1
    $bestCands = $null
    for ($i = 0; $i -lt 81; $i++) {
        if ($g[$i] -ne 0) { continue }
        $cands = @(Get-Candidates $g $i)
        if ($best -eq -1 -or $cands.Count -lt $bestCands.Count) {
            $best = $i
            $bestCands = $cands
            if ($cands.Count -le 1) { break }
        }
    }
    if ($best -eq -1) { return $true }
    foreach ($v in $bestCands) {
        $g[$best] = $v
        $script:placements++
        if (Solve-Sudoku $g) { return $true }
    }
    $g[$best] = 0
    return $false
}

function Test-Sudoku([int[]]$g) {
    for ($i = 0; $i -lt 81; $i++) {
        $v = $g[$i]
        $g[$i] = 0
        $ok = $v -in @(Get-Candidates $g $i)
        $g[$i] = $v
        if (-not $ok) { return $false }
    }
    return $true
}

[int[]]$grid = $puzzle.ToCharArray() | ForEach-Object { if ($_ -eq '.') { 0 } else { [int][string]$_ } }
$script:placements = 0
$null = Solve-Sudoku $grid
for ($r = 0; $r -lt 9; $r++) {
    $row = -join $grid[($r * 9)..($r * 9 + 8)]
    "$($row.Substring(0, 3)) $($row.Substring(3, 3)) $($row.Substring(6, 3))"
}
"valid: $(Test-Sudoku $grid)"
"placements: $script:placements"

@@exercise Eight queens
Count all solutions to the N-Queens puzzle for N = 4 to 8 with backtracking (one queen per row), and print the first solution for N = 8 as a board (`Q` and `.`).

Speed it up with three boolean arrays: used columns, used "/" diagonals (`r + c`) and used "\" diagonals (`r - c + N - 1`), so checking a square is O(1).
@@starter
'TODO'
@@expected
N=4: 2 solutions
N=5: 10 solutions
N=6: 4 solutions
N=7: 40 solutions
N=8: 92 solutions
first solution for N=8:
Q.......
....Q...
.......Q
.....Q..
..Q.....
......Q.
.Q......
...Q....
@@requires
[bool[]]
function
@@hint
Keep state in a hashtable so the recursive function can update it: `$s = @{ N = $n; Cols = [bool[]]::new($n); D1 = [bool[]]::new(2 * $n); D2 = [bool[]]::new(2 * $n); Pos = [int[]]::new($n); Count = 0; First = $null }`. In `Place($s, $r)`: if `$r -eq $s.N`, count it (and copy `Pos` the first time); otherwise loop over columns, skip attacked squares, mark, recurse with `$r + 1`, unmark.
@@solution
function Place([hashtable]$s, [int]$r) {
    $n = $s.N
    if ($r -eq $n) {
        $s.Count++
        if ($null -eq $s.First) { $s.First = $s.Pos.Clone() }
        return
    }
    for ($c = 0; $c -lt $n; $c++) {
        $d1 = $r + $c
        $d2 = $r - $c + $n - 1
        if ($s.Cols[$c] -or $s.D1[$d1] -or $s.D2[$d2]) { continue }
        $s.Cols[$c] = $true; $s.D1[$d1] = $true; $s.D2[$d2] = $true
        $s.Pos[$r] = $c
        Place $s ($r + 1)
        $s.Cols[$c] = $false; $s.D1[$d1] = $false; $s.D2[$d2] = $false
    }
}

$eight = $null
foreach ($n in 4..8) {
    $s = @{ N = $n; Cols = [bool[]]::new($n); D1 = [bool[]]::new(2 * $n); D2 = [bool[]]::new(2 * $n); Pos = [int[]]::new($n); Count = 0; First = $null }
    Place $s 0
    "N=${n}: $($s.Count) solutions"
    if ($n -eq 8) { $eight = $s.First }
}
'first solution for N=8:'
foreach ($c in $eight) { ('.' * $c) + 'Q' + ('.' * (7 - $c)) }

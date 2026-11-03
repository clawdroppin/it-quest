@@lesson b10-games 🎓 Beyond IT: Terminal Games in Bash
@@concepts lists, loops, conditions
@@text
> **Beyond IT scope.** Writing games in Bash is a hobby, not a job skill. But it's a great way to master arrays, arithmetic and the terminal, and the same patterns appear in the TUI module.

Bash has everything a simple game needs: integer arithmetic, arrays for the board, `read -rsn1 -t` for a game loop with a tick, and escape codes for drawing. People have written Tetris, Snake, 2048 and even ray-casters in pure Bash.

## Boards as one-dimensional arrays

Bash has no 2D arrays, so store a `W × H` grid in a flat array and compute the index:

```bash
W=4 H=3
board=()
for (( i = 0; i < W * H; i++ )); do board[i]=.; done
set_cell() { board[$2 * W + $1]=$3; }       # x y value
set_cell 1 2 '#'
for (( y = 0; y < H; y++ )); do
    row=""
    for (( x = 0; x < W; x++ )); do row+=${board[y * W + x]}; done
    echo "$row"
done
```

Neighbours with wrap-around (a "torus" world) use modulo: `nx = (x + dx + W) % W`.

## The game loop

```bash,norun
while (( running )); do
    draw
    if read -rsn1 -t 0.15 key; then handle "$key"; fi   # tick every 150 ms
    step                                              # advance the world
done
```

## Randomness

`$RANDOM` gives 0–32767; `$(( RANDOM % 6 + 1 ))` is a die roll. Assigning `RANDOM=42` **seeds** it, making a run repeatable, which is useful for tests and for sharing "level seeds". (The sequence for a given seed can differ between Bash versions, so the exercises here avoid it.)

## Conway's Game of Life

A grid of cells, each alive or dead. Every generation, simultaneously:

- a live cell with 2 or 3 live neighbours survives
- a dead cell with exactly 3 live neighbours becomes alive
- every other cell dies or stays dead

From these rules come gliders that walk across the grid, oscillators and even working computers. The key implementation detail: compute the next generation into a **new** array, never in place.

## 2048

Slide all tiles in one direction; two equal tiles that collide merge into their sum, and each tile merges at most once per move. Every direction reduces to one function, "slide a row to the left", applied to rows or columns, reversed or not.

@@quiz
How do you store a W×H grid in Bash?
* In a flat array, at index `y * W + x`
- In a 2D array `grid[x][y]`
- In W separate variables
- Bash can't store grids
? Associative arrays with keys like "x,y" also work, but slower.

@@quiz
Why must the Game of Life compute the next generation into a new array?
- Arrays are read-only
* All cells change simultaneously; updating in place would let already-updated cells affect their neighbours' counts
- To save memory
- It's faster
? Double buffering.

@@flashcards
`board[y * W + x]` :: 2D grid in a flat array
`(x + dx + W) % W` :: wrap-around coordinate
`read -rsn1 -t 0.15` :: game tick with input
`RANDOM` :: 0–32767 pseudo-random
`RANDOM=42` :: seed for repeatable runs
double buffering :: compute the next state into a new array
Game of Life :: 2-3 survive, 3 born

@@exercise Conway's Game of Life
Simulate a glider on an 8×6 torus (wrapping edges):

- `step` computes the next generation into a new array using the rules, counting the 8 neighbours with wrap-around
- `show` prints the grid with `#` for alive and `.` for dead
- print generations 0 and 4, and the number of live cells; after 4 generations a glider has moved one cell right and one cell down
@@starter
W=8 H=6
glider='.#......
..#.....
###.....
........
........
........'
echo TODO
@@expected
generation 0 (5 alive)
.#......
..#.....
###.....
........
........
........
generation 4 (5 alive)
........
..#.....
...#....
.###....
........
........
@@requires
% W
step
@@hint
Load the grid: loop over the lines and characters, `cells[y * W + x]=$(( c == '#' ))` (compare with `[[ $c == '#' ]]`). In `step`, for each cell sum `cells[ny * W + nx]` over dx, dy in -1..1 except (0, 0), then `next[i]=$(( n == 3 || (cells[i] && n == 2) ))`. Finally `cells=("${next[@]}")`.
@@solution
W=8 H=6
glider='.#......
..#.....
###.....
........
........
........'

cells=()
y=0
while IFS= read -r row; do
    for (( x = 0; x < W; x++ )); do
        if [[ ${row:x:1} == '#' ]]; then cells[y * W + x]=1; else cells[y * W + x]=0; fi
    done
    ((y++))
done <<< "$glider"

step() {
    local x y dx dy n nx ny i
    local -a next=()
    for (( y = 0; y < H; y++ )); do
        for (( x = 0; x < W; x++ )); do
            n=0
            for dy in -1 0 1; do
                for dx in -1 0 1; do
                    (( dx == 0 && dy == 0 )) && continue
                    nx=$(( (x + dx + W) % W ))
                    ny=$(( (y + dy + H) % H ))
                    (( n += cells[ny * W + nx] ))
                done
            done
            i=$(( y * W + x ))
            next[i]=$(( n == 3 || (cells[i] == 1 && n == 2) ))
        done
    done
    cells=("${next[@]}")
}

show() {
    local x y row alive=0
    for (( i = 0; i < W * H; i++ )); do (( alive += cells[i] )); done
    echo "generation $1 ($alive alive)"
    for (( y = 0; y < H; y++ )); do
        row=""
        for (( x = 0; x < W; x++ )); do
            if (( cells[y * W + x] )); then row+='#'; else row+='.'; fi
        done
        echo "$row"
    done
}

show 0
for g in 1 2 3 4; do step; done
show 4

@@exercise 2048 moves
Implement the 2048 logic on a 4×4 board (stored as a flat array, 0 = empty):

- `slide_row a b c d` prints the row slid left: remove zeros, merge equal neighbours once (left to right), pad with zeros; also add merged values to `score` (use a global, so don't call it inside `$( )`; write the result to the global array `row_out` instead)
- `move DIR` applies it to every row or column for `left`, `right`, `up` and `down` (reverse for right/down), and reports whether anything moved
- apply the moves `left up right down` and print the board and score after each
@@starter
board=(2 2 4 0
       0 4 4 4
       2 0 2 8
       0 0 0 2)
score=0
echo TODO
@@expected
left (moved, score 16)
   4    4    .    .
   8    4    .    .
   4    8    .    .
   2    .    .    .
up (moved, score 24)
   4    8    .    .
   8    8    .    .
   4    .    .    .
   2    .    .    .
right (moved, score 40)
   .    .    4    8
   .    .    .   16
   .    .    .    4
   .    .    .    2
down (moved, score 40)
   .    .    .    8
   .    .    .   16
   .    .    .    4
   .    .    4    2
@@requires
row_out
slide_row
@@hint
`slide_row() { local -a v=(); local x; for x in "$@"; do (( x )) && v+=("$x"); done; row_out=(); local i=0; while (( i < ${#v[@]} )); do if (( i + 1 < ${#v[@]} && v[i] == v[i+1] )); then row_out+=($(( v[i] * 2 ))); (( score += v[i] * 2 )); (( i += 2 )); else row_out+=("${v[i]}"); ((i++)); fi; done; while (( ${#row_out[@]} < 4 )); do row_out+=(0); done; }`. For each line in `move`, build the list of 4 board indices in the order to slide (e.g. for `right` on row r: `r*4+3 r*4+2 r*4+1 r*4`), read the values, slide, write back to the same indices.
@@solution
board=(2 2 4 0
       0 4 4 4
       2 0 2 8
       0 0 0 2)
score=0

slide_row() {
    local -a v=()
    local x i=0
    for x in "$@"; do (( x )) && v+=("$x"); done
    row_out=()
    while (( i < ${#v[@]} )); do
        if (( i + 1 < ${#v[@]} && v[i] == v[i + 1] )); then
            row_out+=($(( v[i] * 2 )))
            (( score += v[i] * 2 ))
            (( i += 2 ))
        else
            row_out+=("${v[i]}")
            ((i++))
        fi
    done
    while (( ${#row_out[@]} < 4 )); do row_out+=(0); done
}

move() {
    local dir=$1 line k moved=0
    local -a idx vals
    for (( line = 0; line < 4; line++ )); do
        case $dir in
            left)  idx=($(( line * 4 )) $(( line * 4 + 1 )) $(( line * 4 + 2 )) $(( line * 4 + 3 ))) ;;
            right) idx=($(( line * 4 + 3 )) $(( line * 4 + 2 )) $(( line * 4 + 1 )) $(( line * 4 ))) ;;
            up)    idx=($line $(( line + 4 )) $(( line + 8 )) $(( line + 12 ))) ;;
            down)  idx=($(( line + 12 )) $(( line + 8 )) $(( line + 4 )) $line) ;;
        esac
        vals=()
        for k in "${idx[@]}"; do vals+=("${board[k]}"); done
        slide_row "${vals[@]}"
        for k in 0 1 2 3; do
            (( board[idx[k]] != row_out[k] )) && moved=1
            board[idx[k]]=${row_out[k]}
        done
    done
    if (( moved )); then echo "$dir (moved, score $score)"; else echo "$dir (no change, score $score)"; fi
}

show() {
    local r c v line
    for (( r = 0; r < 4; r++ )); do
        line=""
        for (( c = 0; c < 4; c++ )); do
            v=${board[r * 4 + c]}
            (( v )) || v=.
            printf -v line '%s%4s ' "$line" "$v"
        done
        echo "${line% }"
    done
}

for dir in left up right down; do
    move "$dir"
    show
done

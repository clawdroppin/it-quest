@@lesson b10-generative 🎓 Beyond IT: Generative ASCII Art with Integer Maths
@@concepts numbers, loops
@@text
> **Beyond IT scope.** Fractals, mazes and procedural patterns belong to creative coding and the demoscene, not to IT jobs. They are, however, a superb workout for loops and arithmetic.

## Fixed-point arithmetic

Bash only has integers. To work with fractions, scale everything by a constant, for example **1,000** (three decimal places), or a power of two like 1024 for speed:

```bash
S=1000
a=1500          # 1.5
b=250           # 0.25
echo $(( a * b / S ))                          # 375  → 0.375
printf '%d.%03d\n' $(( a * b / S / S )) $(( a * b / S % S ))
```

Multiply, then divide by the scale once. Watch for overflow: Bash integers are 64-bit, so with S = 1,000 you're safe up to about 9 × 10¹².

## The Mandelbrot set

For each point `c = (cx, cy)` on the plane, iterate `z = z² + c` starting at `z = 0`. If `|z|` stays within 2, the point belongs to the set. Written out with real numbers:

```text
x' = x² - y² + cx
y' = 2xy + cy
escape when x² + y² > 4
```

The number of iterations before escaping picks a character from a "density ramp" like ` .:-=+*#%@`. A 60×24 render needs ~30,000 iterations: a few seconds in Bash, so keep the size modest.

## Mazes: the recursive backtracker

1. Start in a cell; mark it visited and push it on a stack.
2. Look at unvisited neighbours (two cells away in the wall grid). If there are any, pick one, knock down the wall between, mark it, push it.
3. Otherwise pop the stack (backtrack).
4. Stop when the stack is empty: every cell is connected by exactly one path.

The randomness comes from a **linear congruential generator** (LCG), the simplest reproducible PRNG: `seed = (seed * 1103515245 + 12345) % 2^31`. The same seed always gives the same maze, on any machine.

## Other ideas to try

- Sierpiński triangle: print `*` where `(x & y) == 0`.
- Plasma and moiré patterns from sums of `(x*x + y*y) % N`.
- Elementary cellular automata (rule 30, rule 110), one line per generation.
- Spirographs and Lissajous curves (pre-compute a sine table, since Bash has no `sin`).

@@quiz
How do you represent 2.5 in fixed-point arithmetic with a scale of 1000?
- 2.5
* 2500
- 25
- 250
? Multiply real values by the scale.

@@quiz
Why does `a * b` need to be divided by the scale once in fixed-point?
* Both factors carry the scale, so the product has it twice
- To round the result
- Bash requires it
- To avoid negative numbers
? (a·S)(b·S) = ab·S².

@@quiz
Why does Bash need fixed-point tricks for the Mandelbrot set?
* Bash arithmetic only handles integers
- Bash has no loops
- Bash can't print characters
- Floats are too fast
? Scale every number by, say, 1000 and keep all maths in whole numbers.

@@drill
Bash arithmetic has no decimals. What's the name of the technique of scaling numbers by a fixed factor, such as 1000?
= fixed-point arithmetic
= fixed point
= fixed-point
? Store 1.5 as 1500 and divide at the end.

@@flashcards
fixed-point :: integers scaled by a constant
`a * b / S` :: fixed-point multiply
Mandelbrot :: z = z² + c, escape at |z| > 2
density ramp :: characters from light to dark
recursive backtracker :: DFS maze generation
LCG :: seed = (seed·a + c) mod m
`(x & y) == 0` :: Sierpiński pattern

@@exercise ASCII Mandelbrot
Render the Mandelbrot set in a 40×12 grid with fixed-point arithmetic (scale 1000):

- x from -2.1 to 0.7 and y from -1.2 to 1.2 (step = range / (columns or rows - 1); compute in fixed point)
- up to 30 iterations; escape when `x² + y² > 4`
- map the iteration count to the ramp ` .:-=+*#%@` (index = iterations × 9 / 30); points that never escape get `@`
- print each row between `|` borders, then the count of `@` cells
@@starter
S=1000 W=40 H=12 MAX=30
ramp=' .:-=+*#%@'
echo TODO
@@expected
|                                        |
|                    .......:--....      |
|                ........::@@@#:.....    |
|            ........:-@@@@@@@@@@@@+:..  |
|     ......:+-:#-::-@@@@@@@@@@@@@@@#... |
|  ......::--@@@@@@@@@@@@@@@@@@@@@@@:... |
|  ......::==@@@@@@@@@@@@@@@@@@@@@@@:... |
|     ......:@-:@-::-@@@@@@@@@@@@@@@#... |
|            ........:-@@@@@@@@@@@@+:..  |
|                ........::@@@#:.....    |
|                    .......:--....      |
|                                        |
inside: 108
@@requires
$((
ramp
@@hint
`cx=$(( -2100 + col * 2800 / (W - 1) ))`, `cy=$(( -1200 + row * 2400 / (H - 1) ))`. Iterate: `x2=$(( x * x / S )); y2=$(( y * y / S )); (( x2 + y2 > 4 * S )) && break; y=$(( 2 * x * y / S + cy )); x=$(( x2 - y2 + cx ))`.
@@solution
S=1000 W=40 H=12 MAX=30
ramp=' .:-=+*#%@'

inside=0
for (( row = 0; row < H; row++ )); do
    cy=$(( -1200 + row * 2400 / (H - 1) ))
    line=""
    for (( col = 0; col < W; col++ )); do
        cx=$(( -2100 + col * 2800 / (W - 1) ))
        x=0 y=0 i=0
        while (( i < MAX )); do
            x2=$(( x * x / S ))
            y2=$(( y * y / S ))
            (( x2 + y2 > 4 * S )) && break
            y=$(( 2 * x * y / S + cy ))
            x=$(( x2 - y2 + cx ))
            ((i++))
        done
        if (( i == MAX )); then
            line+='@'
            ((inside++))
        else
            line+=${ramp:i * 9 / MAX:1}
        fi
    done
    echo "|$line|"
done
echo "inside: $inside"

@@exercise A reproducible maze
Generate a 6×4-cell maze with the recursive backtracker and an LCG:

- the wall grid is `(2*CW+1) × (2*CH+1)` characters, all `#` at first; cell (cx, cy) is at grid position (2cx+1, 2cy+1)
- `rand N` advances `seed = (seed * 1103515245 + 12345) % 2147483648` and returns `(seed / 65536) % N` in the global `R` (no subshell, or the seed would not advance)
- neighbours are checked in the order up, right, down, left; pick one of the unvisited ones with `rand`
- open the entrance at the top-left and exit at the bottom-right, then print the maze rows between `|` borders and the number of open cells
@@starter
CW=6 CH=4 seed=2026
echo TODO
@@expected
|# ###########|
|#     #     #|
|##### # ### #|
|# #   # # # #|
|# # ### # # #|
|#   #   # # #|
|# ### ### # #|
|#     #     #|
|########### #|
open cells: 49
@@requires
1103515245
stack
@@hint
Keep the stack as two arrays `sx` and `sy` (or one array of `x,y` strings). Each step: collect candidates `(nx, ny)` that are in bounds and unvisited; if none, `unset 'stack[-1]'`; else `rand ${#cand[@]}`, open the wall cell at the midpoint `grid[(cy + ny + 1) * GW + cx + nx + 1]` (in grid coordinates: `(2cx+1 + 2nx+1) / 2`), open the new cell, mark it, push it.
@@solution
CW=6 CH=4 seed=2026
GW=$(( 2 * CW + 1 )) GH=$(( 2 * CH + 1 ))

rand() {
    seed=$(( (seed * 1103515245 + 12345) % 2147483648 ))
    R=$(( (seed / 65536) % $1 ))
}

grid=()
for (( i = 0; i < GW * GH; i++ )); do grid[i]='#'; done
declare -A visited=()
open() { grid[$2 * GW + $1]=' '; }

open 1 1
visited[0,0]=1
stack=("0,0")
dxs=(0 1 0 -1) dys=(-1 0 1 0)
while (( ${#stack[@]} )); do
    top=${stack[-1]}
    cx=${top%,*} cy=${top#*,}
    cand=()
    for d in 0 1 2 3; do
        nx=$(( cx + dxs[d] )) ny=$(( cy + dys[d] ))
        (( nx < 0 || ny < 0 || nx >= CW || ny >= CH )) && continue
        [[ -n ${visited[$nx,$ny]:-} ]] && continue
        cand+=("$nx,$ny")
    done
    if (( ${#cand[@]} == 0 )); then
        unset 'stack[-1]'
        continue
    fi
    rand ${#cand[@]}
    next=${cand[R]}
    nx=${next%,*} ny=${next#*,}
    open $(( cx + nx + 1 )) $(( cy + ny + 1 ))
    open $(( 2 * nx + 1 )) $(( 2 * ny + 1 ))
    visited[$nx,$ny]=1
    stack+=("$next")
done

open 1 0
open $(( GW - 2 )) $(( GH - 1 ))
spaces=0
for (( y = 0; y < GH; y++ )); do
    line=""
    for (( x = 0; x < GW; x++ )); do
        line+=${grid[y * GW + x]}
        [[ ${grid[y * GW + x]} == ' ' ]] && ((spaces++))
    done
    echo "|$line|"
done
echo "open cells: $spaces"

@@lesson b9-generative 🎓 Beyond IT: Procedural Generation and Cellular Automata
@@concepts loops, lists
@@text
> **Beyond IT scope.** Procedural generation creates content (levels, mazes, terrain, art) from rules and randomness. It's used in games and generative art rather than IT, but it's a great workout for algorithms and data structures.

## Seeds make randomness reproducible

```go
package main

import (
	"fmt"
	"math/rand/v2"
)

func main() {
	for range 2 {
		rng := rand.New(rand.NewPCG(2026, 11)) // same seed, same sequence
		fmt.Println(rng.IntN(100), rng.IntN(100), rng.IntN(100))
	}
}
```

A game can share a short seed (like Minecraft's world seeds) instead of a whole level. Use the global `rand.IntN` when you want different results each run, and a seeded `*rand.Rand` when you want reproducibility (tests, replays, shareable worlds).

## Classic techniques

- **Mazes**: a randomized depth-first search (the "recursive backtracker") carves a *perfect* maze: exactly one path between any two cells.
- **Cellular automata**: a grid where each cell's next state depends on its neighbours. Conway's Game of Life is the famous one; similar rules smooth random noise into natural-looking caves.
- **Noise**: Perlin/simplex noise gives smooth random terrain heights and textures.
- **L-systems**: string rewriting rules that grow plants and fractals.
- **Wave function collapse**: fill a grid from tiles whose edges must match.

## Images from Go

The standard library can write images, so generative art needs no dependencies:

```go,norun
img := image.NewRGBA(image.Rect(0, 0, 512, 512))
for y := range 512 {
	for x := range 512 {
		img.Set(x, y, color.RGBA{uint8(x ^ y), uint8(x), uint8(y), 255})
	}
}
f, _ := os.Create("art.png")
defer f.Close()
png.Encode(f, img)
```

@@quiz
Why seed the random generator in a procedural game?
* The same seed reproduces the same world, for sharing, testing and replays
- Seeds make it more random
- Go requires it
- To make it faster
? `rand.New(rand.NewPCG(seed1, seed2))`.

@@quiz
What makes a maze "perfect"?
- It has no dead ends
* There is exactly one path between any two cells
- It's square
- It has one entrance
? A spanning tree of the grid.

@@quiz
In cellular-automaton cave generation, what does each step do?
* Turns each cell into wall or floor depending on how many neighbouring walls it has
- Picks a random cell to flip
- Deletes the outer walls
- Draws straight corridors
? A few smoothing steps turn noise into natural-looking caves.

@@flashcards
Seed :: starting value that fixes a random sequence
`rand.NewPCG` :: seedable generator (math/rand/v2)
Recursive backtracker :: randomized DFS maze
Perfect maze :: exactly one path between cells
Cellular automaton :: cells update from neighbours
Game of Life :: birth on 3, survive on 2 or 3
Glider :: Life pattern that moves diagonally
`image/png` :: write PNG files from the stdlib

@@exercise A seeded maze generator
Generate a perfect maze with the iterative recursive backtracker. To make the output reproducible, follow these rules exactly:

- the maze is `w × h` cells; each cell has walls N, E, S, W; start at cell (0, 0), mark it visited and push it on a stack
- while the stack isn't empty: look at the top cell; collect its **unvisited** neighbours in the order **N, E, S, W**; if there are none, pop; otherwise pick `nbrs[rng.IntN(len(nbrs))]`, remove the walls between the two cells, mark the neighbour visited and push it
- `rng := rand.New(rand.NewPCG(42, 7))`

Render it with `+`, `---`, `|` and spaces (each cell is 3 characters wide), and check that every cell is reachable from (0, 0) and exactly `w*h-1` walls were removed.
@@starter
package main

import (
	"fmt"
	"math/rand/v2"
	"strings"
)

const (
	N = 1 << iota
	E
	S
	W
)

func main() {
	w, h := 6, 4
	rng := rand.New(rand.NewPCG(42, 7))
	_ = rng
	_ = strings.Builder{}
	fmt.Println(w, h)
}
@@expected
+---+---+---+---+---+---+
|   |       |           |
+   +   +---+   +   +---+
|   |           |       |
+   +---+   +---+---+   +
|       |   |           |
+---+   +---+   +---+   +
|               |       |
+---+---+---+---+---+---+
all cells reachable: true
passages carved: 23
@@hint
Store walls as `walls[y][x]` bit sets starting at `N|E|S|W`. Neighbour offsets with their wall and the opposite wall: N `(0,-1)` S, E `(1,0)` W, S `(0,1)` N, W `(-1,0)` E. Rendering: for each row, a line of `+` followed by `---+` or `   +` (north walls), then a line starting with `|` and, per cell, `   ` followed by `|` or a space (east walls); finish with the bottom line.
@@solution
package main

import (
	"fmt"
	"math/rand/v2"
	"strings"
)

const (
	N = 1 << iota
	E
	S
	W
)

type dir struct {
	dx, dy     int
	wall, back int
}

var dirs = []dir{{0, -1, N, S}, {1, 0, E, W}, {0, 1, S, N}, {-1, 0, W, E}}

func Generate(w, h int, rng *rand.Rand) ([][]int, int) {
	walls := make([][]int, h)
	visited := make([][]bool, h)
	for y := range h {
		walls[y] = make([]int, w)
		visited[y] = make([]bool, w)
		for x := range w {
			walls[y][x] = N | E | S | W
		}
	}
	type cell struct{ x, y int }
	stack := []cell{{0, 0}}
	visited[0][0] = true
	carved := 0
	for len(stack) > 0 {
		c := stack[len(stack)-1]
		var nbrs []dir
		for _, d := range dirs {
			nx, ny := c.x+d.dx, c.y+d.dy
			if nx >= 0 && nx < w && ny >= 0 && ny < h && !visited[ny][nx] {
				nbrs = append(nbrs, d)
			}
		}
		if len(nbrs) == 0 {
			stack = stack[:len(stack)-1]
			continue
		}
		d := nbrs[rng.IntN(len(nbrs))]
		nx, ny := c.x+d.dx, c.y+d.dy
		walls[c.y][c.x] &^= d.wall
		walls[ny][nx] &^= d.back
		visited[ny][nx] = true
		carved++
		stack = append(stack, cell{nx, ny})
	}
	return walls, carved
}

func Render(walls [][]int) string {
	var b strings.Builder
	for _, row := range walls {
		b.WriteString("+")
		for _, c := range row {
			if c&N != 0 {
				b.WriteString("---+")
			} else {
				b.WriteString("   +")
			}
		}
		b.WriteString("\n|")
		for _, c := range row {
			if c&E != 0 {
				b.WriteString("   |")
			} else {
				b.WriteString("    ")
			}
		}
		b.WriteString("\n")
	}
	b.WriteString("+" + strings.Repeat("---+", len(walls[0])) + "\n")
	return b.String()
}

func reachable(walls [][]int) int {
	h, w := len(walls), len(walls[0])
	seen := map[[2]int]bool{{0, 0}: true}
	queue := [][2]int{{0, 0}}
	for len(queue) > 0 {
		c := queue[0]
		queue = queue[1:]
		for _, d := range dirs {
			if walls[c[1]][c[0]]&d.wall != 0 {
				continue
			}
			n := [2]int{c[0] + d.dx, c[1] + d.dy}
			if n[0] >= 0 && n[0] < w && n[1] >= 0 && n[1] < h && !seen[n] {
				seen[n] = true
				queue = append(queue, n)
			}
		}
	}
	return len(seen)
}

func main() {
	w, h := 6, 4
	rng := rand.New(rand.NewPCG(42, 7))
	walls, carved := Generate(w, h, rng)
	fmt.Print(Render(walls))
	fmt.Println("all cells reachable:", reachable(walls) == w*h)
	fmt.Println("passages carved:", carved)
}

@@exercise The Game of Life
Implement Conway's Game of Life on a **wrapping** (toroidal) grid:

- `Parse(lines []string) [][]bool` (`#` alive, `.` dead) and `String(g)` back
- `Step(g) [][]bool`: a live cell survives with 2 or 3 live neighbours; a dead cell becomes alive with exactly 3; the 8 neighbours wrap around the edges
- `Equal(a, b)`, and `Shift(g, dx, dy)` that moves every cell by (dx, dy) with wrapping

`main` checks two famous patterns: a **glider** returns to its shape moved by (1, 1) after 4 generations, and a **blinker** has period 2. It prints the glider after 4 steps.
@@starter
package main

import (
	"fmt"
	"strings"
)

func main() {
	glider := []string{
		".#......",
		"..#.....",
		"###.....",
		"........",
		"........",
		"........",
	}
	_ = glider
	_ = strings.Join
	fmt.Println("TODO")
}
@@expected
........
..#.....
...#....
.###....
........
........
glider moved by (1,1): true
blinker period 2: true
blinker changes after 1 step: true
@@hint
Neighbours with wrapping: `g[(y+dy+h)%h][(x+dx+w)%w]` for dx, dy in -1..1 except (0, 0). `Step` must build a new grid: never update in place. The blinker: a 5×5 grid with `###` in the middle row.
@@solution
package main

import (
	"fmt"
	"strings"
)

func Parse(lines []string) [][]bool {
	g := make([][]bool, len(lines))
	for y, l := range lines {
		g[y] = make([]bool, len(l))
		for x, c := range l {
			g[y][x] = c == '#'
		}
	}
	return g
}

func String(g [][]bool) string {
	var b strings.Builder
	for _, row := range g {
		for _, c := range row {
			if c {
				b.WriteByte('#')
			} else {
				b.WriteByte('.')
			}
		}
		b.WriteByte('\n')
	}
	return b.String()
}

func Step(g [][]bool) [][]bool {
	h, w := len(g), len(g[0])
	next := make([][]bool, h)
	for y := range h {
		next[y] = make([]bool, w)
		for x := range w {
			n := 0
			for dy := -1; dy <= 1; dy++ {
				for dx := -1; dx <= 1; dx++ {
					if (dx != 0 || dy != 0) && g[(y+dy+h)%h][(x+dx+w)%w] {
						n++
					}
				}
			}
			next[y][x] = n == 3 || (g[y][x] && n == 2)
		}
	}
	return next
}

func Equal(a, b [][]bool) bool { return String(a) == String(b) }

func Shift(g [][]bool, dx, dy int) [][]bool {
	h, w := len(g), len(g[0])
	out := make([][]bool, h)
	for y := range h {
		out[y] = make([]bool, w)
	}
	for y := range h {
		for x := range w {
			out[(y+dy+h)%h][(x+dx+w)%w] = g[y][x]
		}
	}
	return out
}

func main() {
	glider := Parse([]string{
		".#......",
		"..#.....",
		"###.....",
		"........",
		"........",
		"........",
	})
	g := glider
	for range 4 {
		g = Step(g)
	}
	fmt.Print(String(g))
	fmt.Println("glider moved by (1,1):", Equal(g, Shift(glider, 1, 1)))
	blinker := Parse([]string{".....", ".....", ".###.", ".....", "....."})
	fmt.Println("blinker period 2:", Equal(Step(Step(blinker)), blinker))
	fmt.Println("blinker changes after 1 step:", !Equal(Step(blinker), blinker))
}

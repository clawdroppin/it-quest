@@lesson b9-games 🎓 Beyond IT: 2D Games with Ebitengine
@@concepts loops, objects
@@text
> **Beyond IT scope.** Game development is outside IT, but it's a superb way to practise loops, state machines, geometry and performance thinking.

**Ebitengine** (`github.com/hajimehoshi/ebiten/v2`) is a simple 2D game engine for Go. Games run on Windows, macOS, Linux, the browser (WebAssembly), Android, iOS and even Nintendo Switch. Its API is tiny: implement three methods.

```go,norun
type Game struct {
	player Player
	level  *Level
}

// Update runs at a fixed 60 ticks per second: input, physics, game logic.
func (g *Game) Update() error {
	g.player.HandleInput(
		ebiten.IsKeyPressed(ebiten.KeyArrowLeft),
		ebiten.IsKeyPressed(ebiten.KeyArrowRight),
		inpututil.IsKeyJustPressed(ebiten.KeySpace),
	)
	g.player.Step(g.level)
	return nil
}

// Draw renders the current state; it may run at a different rate than Update.
func (g *Game) Draw(screen *ebiten.Image) {
	g.level.Draw(screen)
	op := &ebiten.DrawImageOptions{}
	op.GeoM.Translate(float64(g.player.X()), float64(g.player.Y()))
	screen.DrawImage(playerSprite, op)
}

// Layout returns the logical screen size; Ebitengine scales it to the window.
func (g *Game) Layout(w, h int) (int, int) { return 320, 240 }

func main() {
	ebiten.SetWindowSize(960, 720)
	ebiten.SetWindowTitle("Cave Runner")
	if err := ebiten.RunGame(&Game{}); err != nil {
		log.Fatal(err)
	}
}
```

## Core game techniques

- **Fixed timestep**: `Update` runs exactly 60 times per second, so physics is deterministic and replays work. Never scale movement by frame time in a fixed-step engine.
- **Sub-pixel positions in integers**: store positions in 1/16 pixel units. Movement stays smooth and identical on every machine (no floating-point drift).
- **Tilemaps**: the level is a grid of tiles (often designed in the Tiled editor); collision checks only the tiles under the player's box.
- **AABB collision** (axis-aligned bounding boxes): move on X, resolve overlaps, then move on Y and resolve. Separating the axes prevents getting stuck on corners.
- **State machines** for players and enemies: idle, running, jumping, falling, hurt.
- **Pathfinding**: enemies find routes with BFS or A* over the tile grid.
- **Game feel**: coyote time (jump shortly after leaving a ledge), jump buffering, screen shake. Small rules that make controls feel good.

Other Go options: `raylib-go` (bindings to raylib), `pixel`, and `g3n` for 3D.

@@quiz
Why store positions in sub-pixel integer units?
* Smooth movement with deterministic, drift-free maths on every machine
- Ebitengine forbids floats
- To save memory
- Pixels can't be fractional on screens
? Many classic and modern platformers do exactly this.

@@quiz
Why resolve collisions one axis at a time?
- It's faster to type
* You know which axis caused the overlap, so you can push out correctly and avoid sticking on corners
- Ebitengine requires it
- Diagonal movement is impossible
? Move X, fix X; move Y, fix Y.

@@quiz
How often does Ebitengine call a game's Update method by default?
* 60 times per second
- Once per second
- As fast as possible
- Only when a key is pressed
? Draw may run at the display's rate; Update runs at a fixed tick rate (TPS).

@@drill
Which Ebitengine method do you implement to draw a frame?
= Draw
? Update changes state; Draw renders it.

@@flashcards
`Update()` :: fixed 60 TPS game logic
`Draw(screen)` :: render the state
`Layout` :: logical screen size
`inpututil.IsKeyJustPressed` :: key went down this tick
AABB :: axis-aligned bounding box
Tilemap :: level as a grid of tiles
Sub-pixel units :: integer positions × 16
A* :: shortest path with a heuristic
Coyote time :: jump grace period after a ledge

@@exercise Platformer physics on a tilemap
Implement a player stepping through a tilemap with integer sub-pixel physics (16 units = 1 pixel, tiles are 16×16 pixels):

- player box 12×14 pixels; position `X, Y` (top-left) and velocity `VX, VY` in sub-pixel units
- each tick: `VX` = −24, 0 or +24 from input; `VY += 6` (gravity), capped at 64; if `jump` is pressed while on the ground, `VY = -96`
- move on X by `VX`, then push the box out of any overlapping solid tile (`#`) on that axis; then the same for Y. Landing on a tile sets `OnGround` and `VY = 0`; hitting a ceiling sets `VY = 0`
- the map's edges are solid

`main` (provided) holds right for 40 ticks and presses jump at ticks 5 and 30, printing the state at a few ticks. The jump at tick 5 carries the player onto the two-tile block; the press at tick 30 happens in mid-air, so it must be ignored.
@@starter
package main

import "fmt"

const sub = 16  // sub-pixel units per pixel
const tile = 16 // tile size in pixels

var level = []string{
	"..........",
	"..........",
	".....##...",
	"..........",
	"..##......",
	"##########",
}

type Player struct {
	X, Y, VX, VY int // sub-pixel units
	OnGround     bool
}

const pw, ph = 12, 14 // player size in pixels

func solid(tx, ty int) bool {
	if ty < 0 || ty >= len(level) || tx < 0 || tx >= len(level[0]) {
		return true
	}
	return level[ty][tx] == '#'
}

func (p *Player) Step(left, right, jump bool) {}

func main() {
	p := &Player{X: 2 * sub, Y: (5*tile - ph) * sub}
	for tick := range 60 {
		p.Step(false, tick < 40, tick == 5 || tick == 30)
		if tick%10 == 9 || tick == 5 {
			fmt.Printf("tick %2d: x=%d y=%d ground=%v\n", tick, p.X/sub, p.Y/sub, p.OnGround)
		}
	}
}
@@expected
tick  5: x=11 y=60 ground=false
tick  9: x=17 y=41 ground=false
tick 19: x=32 y=21 ground=false
tick 29: x=47 y=37 ground=false
tick 39: x=62 y=50 ground=true
tick 49: x=62 y=50 ground=true
tick 59: x=62 y=50 ground=true
@@hint
Write `hit(x, y int) bool` that checks every tile under the box **in sub-pixel units** (a tile is `tile*sub` units; the box spans `x` to `x+pw*sub-1`). Checking whole pixels instead is a classic bug: a fall of 6 sub-units never reaches the floor's pixel, so the player is only \"on the ground\" every other tick and jumps get eaten. To resolve X when moving right, put the box's right edge at the left edge of the column you hit: `X = col*tile*sub - pw*sub`; moving left: `X = (col+1)*tile*sub`. Do the same on Y with rows. Clear `OnGround` before moving on Y. Use a floor division helper so negative coordinates work.
@@solution
package main

import "fmt"

const sub = 16  // sub-pixel units per pixel
const tile = 16 // tile size in pixels

var level = []string{
	"..........",
	"..........",
	".....##...",
	"..........",
	"..##......",
	"##########",
}

type Player struct {
	X, Y, VX, VY int // sub-pixel units
	OnGround     bool
}

const pw, ph = 12, 14 // player size in pixels

func solid(tx, ty int) bool {
	if ty < 0 || ty >= len(level) || tx < 0 || tx >= len(level[0]) {
		return true
	}
	return level[ty][tx] == '#'
}

// floorDiv rounds towards negative infinity so positions left of or above the map work.
func floorDiv(a, b int) int {
	q := a / b
	if a%b != 0 && a < 0 {
		q--
	}
	return q
}

// hit reports whether the box with its top-left corner at (x, y), in sub-pixel units,
// overlaps a solid tile. Working in sub-pixels means a fall of a fraction of a pixel still
// touches the floor.
func hit(x, y int) bool {
	const ts = tile * sub
	for ty := floorDiv(y, ts); ty <= floorDiv(y+ph*sub-1, ts); ty++ {
		for tx := floorDiv(x, ts); tx <= floorDiv(x+pw*sub-1, ts); tx++ {
			if solid(tx, ty) {
				return true
			}
		}
	}
	return false
}

func (p *Player) Step(left, right, jump bool) {
	const ts = tile * sub
	p.VX = 0
	if left {
		p.VX -= 24
	}
	if right {
		p.VX += 24
	}
	if jump && p.OnGround {
		p.VY = -96
	}
	p.VY = min(p.VY+6, 64)

	p.X += p.VX
	if hit(p.X, p.Y) {
		if p.VX > 0 {
			col := floorDiv(p.X+pw*sub-1, ts)
			p.X = col*ts - pw*sub
		} else if p.VX < 0 {
			col := floorDiv(p.X, ts)
			p.X = (col + 1) * ts
		}
		p.VX = 0
	}

	p.OnGround = false
	p.Y += p.VY
	if hit(p.X, p.Y) {
		if p.VY > 0 {
			row := floorDiv(p.Y+ph*sub-1, ts)
			p.Y = row*ts - ph*sub
			p.OnGround = true
		} else if p.VY < 0 {
			row := floorDiv(p.Y, ts)
			p.Y = (row + 1) * ts
		}
		p.VY = 0
	}
}

func main() {
	p := &Player{X: 2 * sub, Y: (5*tile - ph) * sub}
	for tick := range 60 {
		p.Step(false, tick < 40, tick == 5 || tick == 30)
		if tick%10 == 9 || tick == 5 {
			fmt.Printf("tick %2d: x=%d y=%d ground=%v\n", tick, p.X/sub, p.Y/sub, p.OnGround)
		}
	}
}

@@exercise Enemy pathfinding
Enemies need to find their way to the player. Write `ShortestPath(grid []string, from, to Point) []Point` using breadth-first search (or A* with a Manhattan heuristic: both find shortest paths on a grid with equal step costs):

- moves are up, down, left and right; `#` is a wall
- return the path including both ends, or nil if the goal is unreachable (or a wall, or outside the map)
- `Valid(grid, path)` checks a path: every step moves exactly one tile orthogonally onto a non-wall tile

Several shortest paths may exist, so `main` prints lengths and validity rather than the exact route, plus a rendering of the route your code chose for you to look at (not compared).
@@starter
package main

import "fmt"

type Point struct{ X, Y int }

var grid = []string{
	"S....#....",
	".##..#.##.",
	"..#....#..",
	"#.####.#.#",
	"....#..#.G",
	"..#.#.##..",
}

func ShortestPath(grid []string, from, to Point) []Point {
	return nil
}

func main() {
	start := Point{0, 0}
	goals := []Point{{9, 4}, {0, 5}, {5, 0}, {6, 4}}
	for _, g := range goals {
		path := ShortestPath(grid, start, g)
		_ = path
		fmt.Println("TODO", g)
	}
}
@@expected
to {9 4}: 19 steps, valid=true
to {0 5}: 7 steps, valid=true
to {5 0}: unreachable
to {6 4}: 10 steps, valid=true
@@hint
BFS with a `prev map[Point]Point` (or a 2D slice) and a queue slice; when you pop the goal, walk `prev` back to the start and reverse. Steps = `len(path) - 1`. In `main`, print `to %v: %d steps, valid=%v` or `to %v: unreachable`; you may print the rendered map to stderr or skip it.
@@solution
package main

import (
	"fmt"
	"os"
	"slices"
)

type Point struct{ X, Y int }

var grid = []string{
	"S....#....",
	".##..#.##.",
	"..#....#..",
	"#.####.#.#",
	"....#..#.G",
	"..#.#.##..",
}

func open(grid []string, p Point) bool {
	return p.Y >= 0 && p.Y < len(grid) && p.X >= 0 && p.X < len(grid[p.Y]) && grid[p.Y][p.X] != '#'
}

var dirs = []Point{{0, -1}, {1, 0}, {0, 1}, {-1, 0}}

func ShortestPath(grid []string, from, to Point) []Point {
	if !open(grid, from) || !open(grid, to) {
		return nil
	}
	prev := map[Point]Point{from: from}
	queue := []Point{from}
	for len(queue) > 0 {
		cur := queue[0]
		queue = queue[1:]
		if cur == to {
			path := []Point{cur}
			for cur != from {
				cur = prev[cur]
				path = append(path, cur)
			}
			slices.Reverse(path)
			return path
		}
		for _, d := range dirs {
			n := Point{cur.X + d.X, cur.Y + d.Y}
			if _, seen := prev[n]; !seen && open(grid, n) {
				prev[n] = cur
				queue = append(queue, n)
			}
		}
	}
	return nil
}

func Valid(grid []string, path []Point) bool {
	for i, p := range path {
		if !open(grid, p) {
			return false
		}
		if i > 0 {
			dx, dy := p.X-path[i-1].X, p.Y-path[i-1].Y
			if dx*dx+dy*dy != 1 {
				return false
			}
		}
	}
	return len(path) > 0
}

func render(grid []string, path []Point) {
	rows := make([][]byte, len(grid))
	for y := range grid {
		rows[y] = []byte(grid[y])
	}
	for _, p := range path[1 : len(path)-1] {
		rows[p.Y][p.X] = '*'
	}
	for _, r := range rows {
		fmt.Fprintln(os.Stderr, string(r))
	}
}

func main() {
	start := Point{0, 0}
	goals := []Point{{9, 4}, {0, 5}, {5, 0}, {6, 4}}
	for _, g := range goals {
		path := ShortestPath(grid, start, g)
		if path == nil {
			fmt.Printf("to %v: unreachable\n", g)
			continue
		}
		fmt.Printf("to %v: %d steps, valid=%v\n", g, len(path)-1, Valid(grid, path))
		if g == goals[0] {
			render(grid, path)
		}
	}
}

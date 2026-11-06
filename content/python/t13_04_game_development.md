@@lesson t13-games 🎓 Beyond IT: Game Development with Pygame
@@concepts objects, loops, lists
@@text
> **Beyond IT scope.** Game development is a creative field of its own. It's here because games are a fun way to practise everything in this course (objects, loops, state, maths), and many programmers started this way.

**Pygame** (`pip install pygame-ce`, the actively maintained community edition) gives you a window, drawing, images, sound and input. Engines like **Godot** (with its Python-like GDScript) are better for bigger projects, but Pygame teaches the fundamentals that every engine hides.

## The game loop

Every game, from Pong to an open-world game, runs the same loop many times a second:

```python,norun
import pygame

pygame.init()
screen = pygame.display.set_mode((640, 360))
clock = pygame.time.Clock()
player = pygame.Rect(300, 300, 40, 20)
ball = pygame.Rect(320, 100, 12, 12)
velocity = pygame.Vector2(180, 220)                      # pixels per second

running = True
while running:
    dt = clock.tick(60) / 1000                           # seconds since last frame, capped at 60 FPS
    for event in pygame.event.get():                     # 1. input
        if event.type == pygame.QUIT:
            running = False
    keys = pygame.key.get_pressed()
    player.x += (keys[pygame.K_RIGHT] - keys[pygame.K_LEFT]) * 300 * dt

    ball.x += velocity.x * dt                            # 2. update
    ball.y += velocity.y * dt
    if ball.left < 0 or ball.right > 640:
        velocity.x *= -1
    if ball.colliderect(player):
        velocity.y = -abs(velocity.y)

    screen.fill("black")                                 # 3. draw
    pygame.draw.rect(screen, "white", player)
    pygame.draw.ellipse(screen, "orange", ball)
    pygame.display.flip()
pygame.quit()
```

1. **Input**: read events and key states.
2. **Update**: move things, apply physics, check collisions, change game state.
3. **Draw**: clear the screen, draw everything, show the frame.

## Delta time

Multiply every movement by `dt`, the time since the last frame. Then speed is in **pixels per second**, and the game runs at the same speed on a fast PC (144 FPS) and a slow laptop (30 FPS). Physics engines often use a **fixed timestep** instead (update exactly 60 times a second, accumulating leftover time), which keeps physics stable.

## Collisions

- **AABB** (axis-aligned bounding boxes): two rectangles overlap when they overlap on both axes. `Rect.colliderect` does this.
- **Circles**: overlap when the distance between centres is less than the sum of the radii.
- Respond by **separating** the objects (push back by the overlap) and changing velocity (bounce, stop).

## Game state and structure

- **Scenes / states**: menu, playing, paused, game over, each with its own update and draw. A small state machine switches between them.
- **Sprites**: `pygame.sprite.Sprite` and `Group` organise many objects.
- **Tile maps**: levels as a grid of characters or numbers (from a text file or the Tiled editor).
- **Pathfinding**: enemies find routes through the grid with **breadth-first search** (equal step costs) or **A\*** (with a distance heuristic). Exercise 2 implements BFS.

## Sharing your game

`pyinstaller --windowed` (see the Tkinter capstone) packages Pygame games too. **pygbag** compiles Pygame games to WebAssembly so they run in a browser, for itch.io.

@@quiz
Why multiply movement by `dt`?
* So speed is per second and the game runs at the same speed regardless of frame rate
- To make the game faster
- To save memory
- Pygame requires it
? Without it, faster computers would run the game faster.

@@quiz
What are the three phases of each game loop iteration?
- Load, save, quit
* Handle input, update the game state, draw the frame
- Compile, link, run
- Draw, draw, draw
? Every engine follows this pattern.

@@quiz
Which algorithm finds the shortest path on a grid where every step costs the same?
- Depth-first search
* Breadth-first search
- Bubble sort
- Binary search
? A* adds a heuristic to search fewer cells.

@@drill
What's the name of the time since the last frame, used to make movement frame-rate independent?
= delta time
= dt
? position += speed * dt.

@@flashcards
Game loop :: input, update, draw, repeated every frame
Delta time :: seconds since the last frame; scales movement
Fixed timestep :: physics updated at a constant rate
AABB :: axis-aligned bounding box collision
`colliderect` :: Pygame rectangle overlap test
Sprite :: a game object with an image and a position
Tile map :: a level stored as a grid
BFS :: shortest path when every step costs the same
A* :: pathfinding guided by a distance heuristic
pygame-ce :: the maintained community edition of Pygame

@@exercise Movement and collisions
Write a `Body` class with `x`, `y`, `w`, `h` (the top-left corner and size) and velocity `vx`, `vy` (pixels per second):

- `update(dt, width, height)`: move by velocity × dt; if the body leaves the area `0..width` × `0..height`, put it back inside at the edge and reverse that velocity component (bounce)
- `overlaps(other)`: AABB test; touching edges (`a.x + a.w == b.x`) don't count as overlap
- `__repr__`: `Body(x, y)` with both rounded to 1 decimal
@@starter
class Body:
    pass


ball = Body(90, 50, 10, 10, vx=200, vy=-150)
for _ in range(3):
    ball.update(0.1, width=120, height=80)
    print(ball, ball.vx, ball.vy)
paddle = Body(60, 10, 40, 8)
print(ball.overlaps(paddle), Body(0, 0, 10, 10).overlaps(Body(10, 0, 5, 5)))
@@expected
Body(110, 35) 200 -150
Body(110, 20) -200 -150
Body(90, 5) -200 -150
True False
@@tests
b = Body(5, 75, 10, 10, vx=-100, vy=100)
b.update(0.1, 100, 80)
check((b.x, b.y, b.vx, b.vy), (0, 70, 100, -100), "bounce off left and bottom")
check(Body(0, 0, 10, 10).overlaps(Body(9, 9, 5, 5)), True, "corner overlap")
check(Body(0, 0, 10, 10).overlaps(Body(0, 10, 10, 10)), False, "touching vertically")
check(repr(Body(1.234, 5.678, 1, 1)), "Body(1.2, 5.7)", "repr")
@@hint
After moving: `if self.x < 0: self.x = 0; self.vx = -self.vx` and `elif self.x + self.w > width: self.x = width - self.w; self.vx = -self.vx` (same for y). `overlaps`: `self.x < o.x + o.w and o.x < self.x + self.w and` the same on y. For the repr, format with `round(v, 1)` and `:g` to drop a trailing `.0`.
@@solution
class Body:
    def __init__(self, x, y, w, h, vx=0, vy=0):
        self.x, self.y, self.w, self.h = x, y, w, h
        self.vx, self.vy = vx, vy

    def update(self, dt, width, height):
        self.x += self.vx * dt
        self.y += self.vy * dt
        if self.x < 0:
            self.x, self.vx = 0, -self.vx
        elif self.x + self.w > width:
            self.x, self.vx = width - self.w, -self.vx
        if self.y < 0:
            self.y, self.vy = 0, -self.vy
        elif self.y + self.h > height:
            self.y, self.vy = height - self.h, -self.vy

    def overlaps(self, o):
        return (self.x < o.x + o.w and o.x < self.x + self.w
                and self.y < o.y + o.h and o.y < self.y + self.h)

    def __repr__(self):
        return f"Body({round(self.x, 1):g}, {round(self.y, 1):g})"


ball = Body(90, 50, 10, 10, vx=200, vy=-150)
for _ in range(3):
    ball.update(0.1, width=120, height=80)
    print(ball, ball.vx, ball.vy)
paddle = Body(60, 10, 40, 8)
print(ball.overlaps(paddle), Body(0, 0, 10, 10).overlaps(Body(10, 0, 5, 5)))

@@exercise Enemy pathfinding with BFS
A level is a list of strings: `#` walls, `.` floor, `E` the enemy, `P` the player. Write `find_path(level)` returning the shortest list of `(row, col)` steps from `E` to `P` (both included), moving up, down, left or right. Explore neighbours in the order up, down, left, right so the result is deterministic. Return `None` if the player can't be reached.

Then `render(level, path)`: the level with `*` on the path cells between `E` and `P`, as one string with lines joined by `\n`.
@@starter
from collections import deque


def find_path(level):
    pass


def render(level, path):
    pass


level = [
    "#########",
    "#E..#...#",
    "#.#.#.#.#",
    "#.#...#P#",
    "#########",
]
path = find_path(level)
print(len(path) - 1, "steps")
print(render(level, path))
@@expected
12 steps
#########
#E**#***#
#.#*#*#*#
#.#***#P#
#########
@@tests
check(find_path(["#####", "#E#P#", "#####"]), None, "walled off")
check(find_path(["EP"]), [(0, 0), (0, 1)], "adjacent")
check(len(find_path(["E...", "....", "...P"])), 6, "open room")
@@hint
Find E and P by scanning. BFS: `queue = deque([start])`, `came_from = {start: None}`; pop from the left, try the four neighbours that are in bounds, not `#` and not in `came_from`. When you reach P, walk `came_from` back to the start and reverse.
@@solution
from collections import deque


def _find(level, ch):
    for r, row in enumerate(level):
        c = row.find(ch)
        if c != -1:
            return r, c


def find_path(level):
    start, goal = _find(level, "E"), _find(level, "P")
    came_from = {start: None}
    queue = deque([start])
    while queue:
        cell = queue.popleft()
        if cell == goal:
            path = []
            while cell is not None:
                path.append(cell)
                cell = came_from[cell]
            return path[::-1]
        r, c = cell
        for nr, nc in ((r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)):
            if (0 <= nr < len(level) and 0 <= nc < len(level[nr])
                    and level[nr][nc] != "#" and (nr, nc) not in came_from):
                came_from[(nr, nc)] = cell
                queue.append((nr, nc))
    return None


def render(level, path):
    grid = [list(row) for row in level]
    for r, c in path[1:-1]:
        grid[r][c] = "*"
    return "\n".join("".join(row) for row in grid)


level = [
    "#########",
    "#E..#...#",
    "#.#.#.#.#",
    "#.#...#P#",
    "#########",
]
path = find_path(level)
print(len(path) - 1, "steps")
print(render(level, path))

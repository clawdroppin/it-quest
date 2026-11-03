@@lesson b9-procgen 🎓 Beyond IT: Godot, MonoGame and Procedural Generation
@@concepts lists, loops
@@text
> **Beyond IT scope.** Game engines and procedural content. A rewarding hobby and career path of its own, but not needed for IT jobs.

## Other C# game engines

- **Godot** (free, open source) supports C# alongside GDScript. Nodes form a scene tree; scripts derive from `Node2D`/`CharacterBody3D` and override `_Ready()` and `_Process(double delta)`. Signals are its built-in events.
- **MonoGame** (and FNA) is a framework rather than an editor: you write the game loop (`Update(GameTime)`, `Draw(GameTime)`) yourself. Celeste and Stardew Valley were built this way.
- **Stride** is an open-source 3D engine written in C#.

```csharp,norun
// Godot 4
public partial class Player : CharacterBody2D
{
    [Export] public float Speed = 200f;

    public override void _PhysicsProcess(double delta)
    {
        Velocity = Input.GetVector("left", "right", "up", "down") * Speed;
        MoveAndSlide();
    }
}
```

## Procedural generation

Roguelikes, survival games and many indie titles generate levels from code. The essential trick is a **seed**: `new Random(seed)` produces the same sequence every time, so a seed fully describes a level. That makes levels shareable ("seed 1234 is great"), testable and reproducible for bug reports.

Common techniques:

- **room placement**: try random rectangles, reject overlaps, connect them with corridors
- **BSP trees**: split the map recursively, put a room in each leaf
- **cellular automata**: random noise smoothed into caves
- **random walks / drunkard's walk**: carve organic tunnels
- **noise functions** (Perlin, Simplex): terrain heights and biomes
- **wave function collapse**: assemble tiles that fit their neighbours

After generating, **validate**: flood fill to check every floor tile is reachable, and regenerate if not.

## Pathfinding with A*

Enemies need to find their way around the map. **A*** explores the most promising cells first, ordering them by `cost so far + estimated cost to goal` (the heuristic). With an admissible heuristic (never overestimating, like Manhattan distance on a 4-way grid) it finds a shortest path. .NET's `PriorityQueue<TElement, TPriority>` makes the open set easy.

@@quiz
Why generate levels from a seed?
* The same seed reproduces the same level, so levels can be shared, tested and debugged
- It makes generation faster
- Seeds are required by Random
- It prevents duplicates
? `new Random(1234)` gives the same sequence every run.

@@quiz
What makes A* find a shortest path on a grid?
- Exploring cells in random order
* An admissible heuristic that never overestimates the remaining cost
- Visiting every cell
- Using diagonal moves
? Manhattan distance is admissible for 4-way movement.

@@flashcards
Seed :: makes randomness reproducible
Room placement :: random rectangles + corridors
BSP :: recursive space partitioning
Cellular automata :: noise smoothed into caves
Flood fill :: check connectivity
A* :: best-first search with a heuristic
Manhattan distance :: |dx| + |dy|
`PriorityQueue<T, P>` :: open set for A*
Godot `_Process(delta)` :: per-frame callback
MonoGame :: code-first game framework

@@exercise A seeded dungeon generator
Generate a dungeon on a 40×14 grid of walls (`#`):

- with `new Random(seed)`, make up to 30 attempts to place up to 6 rooms: width `Next(4, 9)`, height `Next(3, 6)`, x `Next(1, 40 - w - 1)`, y `Next(1, 14 - h - 1)` (in that order); reject rooms that overlap an existing one including a 1-tile margin
- carve rooms as floor (`.`); connect each new room's centre (`X + W / 2`, `Y + H / 2`) to the previous room's centre with an L-shaped corridor: horizontal along the previous room's row, then vertical along the new room's column
- print the map for seed 42, then the room and floor counts, whether a flood fill from the first room reaches every floor tile, and whether the same seed reproduces the map and seed 7 gives a different one
@@starter
const int W = 40, H = 14;
Console.WriteLine("TODO");

record Room(int X, int Y, int W, int H);
@@expected
########################################
###########################......#######
###################..............#######
#############.......#######......#######
#############..................#########
####.......##.......#####....###########
####.........................###########
####.......########.#####....###########
#################.....##################
#################.....##################
#################.....##################
#################.....##################
########################################
########################################
rooms: 5, floor tiles: 126, all connected: True
same seed, same map: True
seed 7 differs: True
@@requires
new Random(seed)
@@hint
`Overlaps(Room o, int m) => X - m < o.X + o.W && o.X < X + W + m && Y - m < o.Y + o.H && o.Y < Y + H + m`. Corridor: `for (int x = Math.Min(x1, x2); x <= Math.Max(x1, x2); x++) map[y1, x] = '.';` then the same for y along `x2`. Flood fill with a `Queue<(int, int)>` and a visited set, then compare its count to the floor count. Render each map to a string for comparisons.
@@solution
const int W = 40, H = 14;

var (map, rooms) = Generate(42);
string text = Render(map);
Console.WriteLine(text);
int floor = text.Count(c => c == '.');
Console.WriteLine($"rooms: {rooms.Count}, floor tiles: {floor}, all connected: {FloodFill(map, rooms[0]) == floor}");
Console.WriteLine($"same seed, same map: {Render(Generate(42).Map) == text}");
Console.WriteLine($"seed 7 differs: {Render(Generate(7).Map) != text}");

static (char[,] Map, List<Room> Rooms) Generate(int seed)
{
    var rng = new Random(seed);
    var map = new char[H, W];
    for (int y = 0; y < H; y++)
        for (int x = 0; x < W; x++)
            map[y, x] = '#';
    var rooms = new List<Room>();
    for (int attempt = 0; attempt < 30 && rooms.Count < 6; attempt++)
    {
        int w = rng.Next(4, 9), h = rng.Next(3, 6);
        int rx = rng.Next(1, W - w - 1), ry = rng.Next(1, H - h - 1);
        var room = new Room(rx, ry, w, h);
        if (rooms.Any(r => r.Overlaps(room, 1))) continue;
        for (int y = ry; y < ry + h; y++)
            for (int x = rx; x < rx + w; x++)
                map[y, x] = '.';
        if (rooms.Count > 0)
        {
            var prev = rooms[^1];
            int x1 = prev.CenterX, y1 = prev.CenterY, x2 = room.CenterX, y2 = room.CenterY;
            for (int x = Math.Min(x1, x2); x <= Math.Max(x1, x2); x++) map[y1, x] = '.';
            for (int y = Math.Min(y1, y2); y <= Math.Max(y1, y2); y++) map[y, x2] = '.';
        }
        rooms.Add(room);
    }
    return (map, rooms);
}

static string Render(char[,] map)
{
    var sb = new System.Text.StringBuilder();
    for (int y = 0; y < H; y++)
    {
        for (int x = 0; x < W; x++) sb.Append(map[y, x]);
        if (y < H - 1) sb.Append('\n');
    }
    return sb.ToString();
}

static int FloodFill(char[,] map, Room start)
{
    var seen = new HashSet<(int X, int Y)> { (start.CenterX, start.CenterY) };
    var queue = new Queue<(int X, int Y)>(seen);
    while (queue.Count > 0)
    {
        var (x, y) = queue.Dequeue();
        foreach (var (nx, ny) in new[] { (x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1) })
            if (map[ny, nx] == '.' && seen.Add((nx, ny))) queue.Enqueue((nx, ny));
    }
    return seen.Count;
}

record Room(int X, int Y, int W, int H)
{
    public int CenterX => X + W / 2;
    public int CenterY => Y + H / 2;

    public bool Overlaps(Room o, int m) =>
        X - m < o.X + o.W && o.X < X + W + m && Y - m < o.Y + o.H && o.Y < Y + H + m;
}

@@exercise A* pathfinding
Write `List<(int X, int Y)>? FindPath(string[] map, (int X, int Y) start, (int X, int Y) goal)`:

- 4-way movement, `#` is a wall, each step costs 1
- use `PriorityQueue<(int X, int Y), int>` ordered by `g + h` with the Manhattan heuristic, a `cameFrom` dictionary and the best known `g` per cell
- return the path from start to goal (both included), or null

Print the map with the path drawn as `*` (keeping `S` and `G`) and its length in steps, then block cell (8, 3) and show that there's no path.
@@starter
string[] map =
[
    "############",
    "#S.....#...#",
    "#.####.#.#.#",
    "#.#....#.#.#",
    "#.#.####.#.#",
    "#........#G#",
    "############",
];
Console.WriteLine("TODO");
@@expected
############
#S.....#***#
#*####.#*#*#
#*#....#*#*#
#*#.####*#*#
#********#G#
############
path length: 21 steps
with (8, 3) blocked: no path
@@requires
PriorityQueue<
@@hint
Find `S` and `G` with `IndexOf` on each row. When you pop a cell equal to the goal, walk `cameFrom` back to the start and reverse. For each neighbour that isn't a wall, `int ng = g[cur] + 1; if (!g.TryGetValue(n, out var old) || ng < old) { g[n] = ng; cameFrom[n] = cur; open.Enqueue(n, ng + H(n)); }`. To draw, turn each row into a `char[]`, mark path cells other than S and G, and join back.
@@solution
string[] map =
[
    "############",
    "#S.....#...#",
    "#.####.#.#.#",
    "#.#....#.#.#",
    "#.#.####.#.#",
    "#........#G#",
    "############",
];
var start = Find(map, 'S');
var goal = Find(map, 'G');
var path = FindPath(map, start, goal)!;
var grid = map.Select(r => r.ToCharArray()).ToArray();
foreach (var (x, y) in path)
    if (grid[y][x] == '.') grid[y][x] = '*';
foreach (var row in grid) Console.WriteLine(new string(row));
Console.WriteLine($"path length: {path.Count - 1} steps");

var blocked = map.ToArray();
blocked[3] = blocked[3][..8] + "#" + blocked[3][9..];
Console.WriteLine($"with (8, 3) blocked: {(FindPath(blocked, start, goal) is null ? "no path" : "found")}");

static (int X, int Y) Find(string[] map, char c)
{
    for (int y = 0; y < map.Length; y++)
    {
        int x = map[y].IndexOf(c);
        if (x >= 0) return (x, y);
    }
    throw new ArgumentException($"no {c} on the map");
}

static List<(int X, int Y)>? FindPath(string[] map, (int X, int Y) start, (int X, int Y) goal)
{
    int H((int X, int Y) p) => Math.Abs(p.X - goal.X) + Math.Abs(p.Y - goal.Y);
    var open = new PriorityQueue<(int X, int Y), int>();
    var g = new Dictionary<(int X, int Y), int> { [start] = 0 };
    var cameFrom = new Dictionary<(int X, int Y), (int X, int Y)>();
    open.Enqueue(start, H(start));
    while (open.TryDequeue(out var cur, out _))
    {
        if (cur == goal)
        {
            var path = new List<(int X, int Y)> { cur };
            while (cameFrom.TryGetValue(cur, out var prev)) path.Add(cur = prev);
            path.Reverse();
            return path;
        }
        foreach (var n in new[] { (cur.X + 1, cur.Y), (cur.X - 1, cur.Y), (cur.X, cur.Y + 1), (cur.X, cur.Y - 1) })
        {
            if (map[n.Item2][n.Item1] == '#') continue;
            int ng = g[cur] + 1;
            if (!g.TryGetValue(n, out var old) || ng < old)
            {
                g[n] = ng;
                cameFrom[n] = cur;
                open.Enqueue(n, ng + H(n));
            }
        }
    }
    return null;
}

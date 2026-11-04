@@lesson b10-fractals 🎓 Beyond IT: Fractals and Pictures with Recursive SQL
@@concepts loops
@@text
> **Beyond IT scope.** Drawing fractals in SQL is a programming puzzle, not something you'll do at work. It's here because it shows how far recursive CTEs go: SQL is a complete programming language, and pushing it to its limits makes the everyday features feel easy.

Recursive CTEs can generate any sequence of rows, and aggregates can glue rows into strings. Together they can draw.

## A coordinate grid

Two recursive CTEs (or one for x and one for y) and a cross join give every pixel:

```sql
WITH RECURSIVE
  xs(x) AS (SELECT 0 UNION ALL SELECT x + 1 FROM xs WHERE x < 9),
  ys(y) AS (SELECT 0 UNION ALL SELECT y + 1 FROM ys WHERE y < 4)
SELECT y, group_concat(CASE WHEN (x + y) % 2 = 0 THEN '#' ELSE '.' END, '') AS row
FROM (SELECT * FROM ys, xs ORDER BY y, x)
GROUP BY y;
```

`group_concat(..., '')` joins each row's characters in the order they arrive, so sort in a subquery first.

## The Mandelbrot set

For each point c = (cx, cy) of the plane, iterate z ← z² + c starting from z = 0. Points where z stays small forever belong to the set; points where |z| grows past 2 escape. The number of iterations before escaping gives the classic shaded picture.

With complex numbers written out, z² + c is:

```text
new_x = x·x − y·y + cx
new_y = 2·x·y + cy
```

In SQL, the recursive CTE carries one row per **pixel per iteration**: `(px, py, cx, cy, x, y, i)`. The step computes the next z for every pixel that hasn't escaped (`x*x + y*y <= 4`) and hasn't hit the iteration limit. Afterwards, `MAX(i)` per pixel is its escape time, mapped to a character from a "palette" string like `' .:-=+*#%@'`.

The SQLite documentation has a famous Mandelbrot query; you'll write your own in the exercise.

## The Sierpinski triangle

Some fractals have a surprising arithmetic shortcut: in a square grid, filling the cell (x, y) exactly when `(x & y) = 0` (bitwise AND) draws a Sierpinski triangle. It's Pascal's triangle modulo 2 in disguise. SQLite has bitwise operators `&`, `|`, `~`, `<<` and `>>`.

## Cellular automata

**Rule 30** and **Rule 90** are one-dimensional automata: each generation, a cell's new state depends on itself and its two neighbours, looked up in an 8-entry rule table (the rule number's binary digits). A recursive CTE that carries the row as a string can compute generation after generation with `substr`. Rule 30 produces chaotic patterns used as a random number generator in Mathematica; Rule 110 is famously Turing-complete.

## Performance

These queries generate thousands of rows per picture. Keep grids small (60×20 characters is plenty) and iteration limits low (20–30). SQLite does this in milliseconds; a production database would let you do it too, but your DBA would ask questions.

@@quiz
Why sort in a subquery before `group_concat(..., '')`?
* group_concat joins values in the order rows arrive, so the subquery decides the character order
- group_concat sorts automatically
- To remove duplicates
- It's required syntax
? SQLite 3.44+ also allows ORDER BY inside the aggregate.

@@quiz
In the Mandelbrot iteration, when is a point considered to have escaped?
- When x is negative
* When x² + y² > 4, i.e. |z| > 2
- After exactly 10 iterations
- When y = 0
? Once |z| > 2 it's guaranteed to grow forever.

@@flashcards
Mandelbrot step :: x' = x² − y² + cx, y' = 2xy + cy
escape condition :: x² + y² > 4
escape time :: iterations before escaping: picks the shade
Sierpinski grid :: filled when (x & y) = 0
`&`, `|`, `<<`, `>>` :: SQLite bitwise operators
Rule 30 :: chaotic 1-D cellular automaton
`group_concat(c, '')` :: join characters into a row

@@exercise The Sierpinski triangle
Draw a 16×16 Sierpinski triangle: rows y = 0..15, columns x = 0..15, and a cell is `*` when `(x & y) = 0` (bitwise AND), otherwise a space. Output one column `row` per y, with trailing spaces trimmed (`rtrim`). Use recursive CTEs for the coordinates.
@@starter
SELECT 'TODO' AS row;
@@expected
row
----------------
****************
* * * * * * * *
**  **  **  **
*   *   *   *
****    ****
* *     * *
**      **
*       *
********
* * * *
**  **
*   *
****
* *
**
*
@@requires
WITH RECURSIVE
group_concat
&
@@hint
`xs(x)` and `ys(y)` from 0 to 15, then `SELECT y, rtrim(group_concat(CASE WHEN (x & y) = 0 THEN '*' ELSE ' ' END, '')) AS row FROM (SELECT * FROM ys, xs ORDER BY y, x) GROUP BY y`, and select only `row` (ordered by y) in an outer query.
@@solution
WITH RECURSIVE
  xs(x) AS (SELECT 0 UNION ALL SELECT x + 1 FROM xs WHERE x < 15),
  ys(y) AS (SELECT 0 UNION ALL SELECT y + 1 FROM ys WHERE y < 15)
SELECT row FROM (
    SELECT y, rtrim(group_concat(CASE WHEN (x & y) = 0 THEN '*' ELSE ' ' END, '')) AS row
    FROM (SELECT * FROM ys, xs ORDER BY y, x)
    GROUP BY y
)
ORDER BY y;

@@exercise The Mandelbrot set in SQL
Draw the Mandelbrot set on a 56×21 character grid: column px = 0..55 maps to cx = −2.2 + px × 0.05, row py = 0..20 maps to cy = −1.0 + py × 0.1. Iterate at most 24 times. The character is taken from the palette `' .:-=+*#%@'` at position `1 + MAX(i) * 9 / 24` (so points that never escape get `@`). Wrap each row in `|` characters so leading spaces are visible.
@@starter
SELECT 'TODO' AS row;
@@expected
row
----------------------------------------------------------
|               ........................::=-=@.........  |
|            ..........................:--@@=-:..........|
|          .........................:::-+@@@@*-:.........|
|         ......................::------+@@@@=---:::*....|
|        .....................:::=@@*@@@@@@@@@@@#=++*+...|
|       ....................:::-+=@@@@@@@@@@@@@@@@@@#::..|
|      ............:=:::::::::-=#@@@@@@@@@@@@@@@@@@@*-#..|
|     ............::-*#==@==--=@@@@@@@@@@@@@@@@@@@@@@+-:.|
|     ...........::-=*@@@@@@@+*@@@@@@@@@@@@@@@@@@@@@@@:..|
|     .....:::::-==+%@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@-:..|
|    @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@=-::..|
|     .....:::::-==+%@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@-:..|
|     ...........::-=*@@@@@@@+*@@@@@@@@@@@@@@@@@@@@@@@:..|
|     ............::-*#==@==--=@@@@@@@@@@@@@@@@@@@@@@+-:.|
|      ............:=:::::::::-=#@@@@@@@@@@@@@@@@@@@*-#..|
|       ....................:::-+=@@@@@@@@@@@@@@@@@@#::..|
|        .....................:::=@@*@@@@@@@@@@@#=++*+...|
|         ......................::------+@@@@=---:::*....|
|          .........................:::-+@@@@*-:.........|
|            ..........................:--@@=-:..........|
|               ........................::=-=@.........  |
@@requires
WITH RECURSIVE
x * x - y * y
MAX(i)
@@hint
`m(px, py, cx, cy, x, y, i)`: anchor `SELECT px, py, -2.2 + px * 0.05, -1.0 + py * 0.1, 0.0, 0.0, 0 FROM xs, ys`; step `SELECT px, py, cx, cy, x * x - y * y + cx, 2.0 * x * y + cy, i + 1 FROM m WHERE x * x + y * y <= 4.0 AND i < 24`. Then `GROUP BY px, py` for `MAX(i)`, and build rows with group_concat over px.
@@solution
WITH RECURSIVE
  xs(px) AS (SELECT 0 UNION ALL SELECT px + 1 FROM xs WHERE px < 55),
  ys(py) AS (SELECT 0 UNION ALL SELECT py + 1 FROM ys WHERE py < 20),
  m(px, py, cx, cy, x, y, i) AS (
    SELECT px, py, -2.2 + px * 0.05, -1.0 + py * 0.1, 0.0, 0.0, 0 FROM xs, ys
    UNION ALL
    SELECT px, py, cx, cy, x * x - y * y + cx, 2.0 * x * y + cy, i + 1
    FROM m WHERE x * x + y * y <= 4.0 AND i < 24
  ),
  esc(px, py, iter) AS (SELECT px, py, MAX(i) FROM m GROUP BY px, py)
SELECT '|' || group_concat(substr(' .:-=+*#%@', 1 + iter * 9 / 24, 1), '') || '|' AS row
FROM (SELECT * FROM esc ORDER BY py, px)
GROUP BY py
ORDER BY py;

@@exercise Rule 90
Run the **Rule 90** cellular automaton for 16 generations on a row of 31 cells, starting with a single live cell in the middle (position 16). A cell's next state is `left XOR right` (its own state doesn't matter); cells beyond the edges count as dead. Print each generation with `#` for live and `.` for dead. (It draws a Sierpinski triangle again: fractals have many roots.)
@@starter
SELECT '...............#...............' AS row;
@@expected
row
-------------------------------
...............#...............
..............#.#..............
.............#...#.............
............#.#.#.#............
...........#.......#...........
..........#.#.....#.#..........
.........#...#...#...#.........
........#.#.#.#.#.#.#.#........
.......#...............#.......
......#.#.............#.#......
.....#...#...........#...#.....
....#.#.#.#.........#.#.#.#....
...#.......#.......#.......#...
..#.#.....#.#.....#.#.....#.#..
.#...#...#...#...#...#...#...#.
#.#.#.#.#.#.#.#.#.#.#.#.#.#.#.#
@@requires
WITH RECURSIVE
substr(
@@hint
Carry the row as a string padded with a dead cell on each side: `gen(n, cells)` with anchor `SELECT 0, '...............#...............'`. To compute the next row you need every position; generate positions 1..31 in another CTE and use a correlated `group_concat` over them: `(SELECT group_concat(CASE WHEN substr('.' || g.cells || '.', p, 1) <> substr('.' || g.cells || '.', p + 2, 1) THEN '#' ELSE '.' END, '') FROM (SELECT p FROM pos ORDER BY p))`.
@@solution
WITH RECURSIVE
  pos(p) AS (SELECT 1 UNION ALL SELECT p + 1 FROM pos WHERE p < 31),
  gen(n, cells) AS (
    SELECT 0, '...............#...............'
    UNION ALL
    SELECT n + 1,
           (SELECT group_concat(CASE WHEN substr('.' || g.cells || '.', p, 1) <> substr('.' || g.cells || '.', p + 2, 1)
                                     THEN '#' ELSE '.' END, '')
            FROM (SELECT p FROM pos ORDER BY p))
    FROM gen g WHERE n < 15
  )
SELECT cells AS row FROM gen ORDER BY n;

@@lesson b10-graphs 🎓 Beyond IT: Graph Algorithms and the Game of Life in SQL
@@concepts loops, maps
@@text
> **Beyond IT scope.** Shortest paths and cellular simulations are computer science algorithms. Network engineers do ask "what's the shortest path between these routers?", but they use routing protocols and graph tools, not SQL. This lesson is a workout for set-based thinking.

## Graphs as tables

A graph is a set of **nodes** and **edges**. In SQL, an edge table is all you need:

```text
edges(src, dst, cost)
```

An undirected graph stores each edge both ways (or the query unions `src→dst` and `dst→src`).

## Shortest paths with a recursive CTE

Breadth-first search falls out of a recursive CTE naturally: start at the source with cost 0, extend each path along every edge to a node not yet on the path, and keep the cheapest total per destination:

```text
paths(node, cost, route):
    anchor:  ('A', 0, 'A')
    step:    (e.dst, p.cost + e.cost, p.route || '>' || e.dst)
             from paths p join edges e on e.src = p.node
             where the route doesn't already contain e.dst
then:    MIN(cost) per node
```

This explores **every** simple path, which grows exponentially with graph size, so it's fine for tens of nodes and hopeless for thousands. Real shortest-path algorithms (Dijkstra) expand only the cheapest frontier node at a time, which needs a priority queue: easy in a programming language, awkward in SQL. PostgreSQL's `pgRouting` extension and graph databases (Neo4j's Cypher) provide them built in.

## Pruning

Two cheap improvements make the recursive version usable for small networks:

- Stop extending a path once its cost exceeds a budget (`WHERE p.cost + e.cost <= 100`).
- Limit the depth (`WHERE length(route) - length(replace(route, '>', '')) < 6`).

## Conway's Game of Life

A grid of cells, each alive or dead. Every generation:

- a live cell with 2 or 3 live neighbours survives;
- a dead cell with exactly 3 live neighbours becomes alive;
- every other cell dies or stays dead.

In SQL, store only the **live** cells as `(x, y)` rows. The next generation is pure set logic:

1. Count, for every cell next to a live cell, how many live neighbours it has: join live cells to the 8 offsets `(-1..1, -1..1)` except `(0, 0)`, group by the resulting coordinates.
2. Keep cells with count 3, or count 2 if the cell is currently alive.

No loops over the grid, no array indexing: one join and one GROUP BY per generation. That's set-based thinking.

@@quiz
Why is exploring every path with a recursive CTE fine for 10 routers but not for 10,000?
* The number of simple paths grows exponentially with the size of the graph
- SQL can't store 10,000 rows
- Recursive CTEs have a 1,000-row limit
- Costs overflow
? Dijkstra expands only the cheapest frontier, which SQL expresses poorly.

@@quiz
In the SQL Game of Life, what does the table store?
- Every cell with a 0/1 flag
* Only the coordinates of live cells
- The neighbours of each cell
- The generation number only
? Sparse storage makes the neighbour count a simple join.

@@quiz
How is a network of routers stored as a graph in SQL?
* A table of edges: one row per link with the two endpoints (and a cost)
- One column per router
- A single text field
- A view with no rows
? Recursive CTEs walk the edges table.

@@flashcards
edge table :: (src, dst, cost)
undirected edge :: store both directions
BFS in SQL :: recursive CTE extending paths
cycle guard :: route NOT LIKE '%node%'
Dijkstra :: cheapest-frontier shortest path
pgRouting :: PostgreSQL graph routing extension
Game of Life survive :: 2 or 3 neighbours
Game of Life birth :: exactly 3 neighbours

@@exercise Shortest paths between offices
The company WAN links offices with costs (latency in ms). Find the cheapest path from `Tokyo` to every other office: show `office`, `cost` and the `route` (offices joined with `>`); when several routes tie, pick the alphabetically first route. Links work in both directions. Sort by cost, then office.
@@setup
CREATE TABLE links (a TEXT, b TEXT, ms INTEGER);
INSERT INTO links VALUES
  ('Tokyo', 'Osaka', 8), ('Tokyo', 'Sendai', 6), ('Osaka', 'Fukuoka', 9), ('Tokyo', 'Fukuoka', 22),
  ('Osaka', 'Hiroshima', 5), ('Hiroshima', 'Fukuoka', 4), ('Sendai', 'Sapporo', 7), ('Tokyo', 'Sapporo', 15),
  ('Osaka', 'Nagoya', 3), ('Tokyo', 'Nagoya', 5);
@@starter
SELECT b AS office, ms AS cost FROM links WHERE a = 'Tokyo' ORDER BY cost;
@@expected
office    | cost | route
----------+------+-----------------------------
Nagoya    | 5    | Tokyo>Nagoya
Sendai    | 6    | Tokyo>Sendai
Osaka     | 8    | Tokyo>Nagoya>Osaka
Hiroshima | 13   | Tokyo>Nagoya>Osaka>Hiroshima
Sapporo   | 13   | Tokyo>Sendai>Sapporo
Fukuoka   | 17   | Tokyo>Nagoya>Osaka>Fukuoka
@@requires
WITH RECURSIVE
NOT LIKE
ROW_NUMBER()
@@hint
`edges(src, dst, ms)` = links UNION ALL the reversed links. `paths(node, cost, route)`: anchor `('Tokyo', 0, 'Tokyo')`; step joins edges on `src = node` with `route NOT LIKE '%' || dst || '%'`. Then `ROW_NUMBER() OVER (PARTITION BY node ORDER BY cost, route)` and keep rn = 1, excluding Tokyo.
@@solution
WITH RECURSIVE
  edges(src, dst, ms) AS (SELECT a, b, ms FROM links UNION ALL SELECT b, a, ms FROM links),
  paths(node, cost, route) AS (
    SELECT 'Tokyo', 0, 'Tokyo'
    UNION ALL
    SELECT e.dst, p.cost + e.ms, p.route || '>' || e.dst
    FROM paths p JOIN edges e ON e.src = p.node
    WHERE p.route NOT LIKE '%' || e.dst || '%'
  ),
  best AS (
    SELECT node, cost, route, ROW_NUMBER() OVER (PARTITION BY node ORDER BY cost, route) AS rn
    FROM paths WHERE node <> 'Tokyo'
  )
SELECT node AS office, cost, route FROM best WHERE rn = 1 ORDER BY cost, office;

@@exercise The Game of Life
Start with a **glider** plus a **blinker** (the starter's live cells). Compute 4 generations into a `gens(gen, x, y)` table, one `INSERT ... SELECT` per generation that joins the previous generation's live cells to the 8 neighbour offsets and applies the rules. (A recursive CTE can't do this in SQLite, because the recursive part may not use GROUP BY or aggregates.) Then draw generation 0 and generation 4 on the area x = 0..9, y = 0..7 as `#`/`.` rows, labelled `gen`. (After 4 generations the glider has moved one cell diagonally and the blinker is back to its start.)
@@setup
CREATE TABLE live (x INTEGER, y INTEGER);
INSERT INTO live VALUES (1, 0), (2, 1), (0, 2), (1, 2), (2, 2), (7, 4), (7, 5), (7, 6);
@@starter
SELECT x, y FROM live ORDER BY y, x;
@@expected
gen | row
----+-----------
0   | .#........
0   | ..#.......
0   | ###.......
0   | ..........
0   | .......#..
0   | .......#..
0   | .......#..
0   | ..........
4   | ..........
4   | ..#.......
4   | ...#......
4   | .###......
4   | .......#..
4   | .......#..
4   | .......#..
4   | ..........
@@requires
WITH RECURSIVE
GROUP BY
COUNT(*)
@@hint
Put the 8 offsets in a table `offs(dx, dy)`. Generation n + 1: `INSERT INTO gens SELECT n + 1, c.x, c.y FROM (SELECT l.x + o.dx AS x, l.y + o.dy AS y, COUNT(*) AS n FROM gens l, offs o WHERE l.gen = n GROUP BY 1, 2) c WHERE c.n = 3 OR (c.n = 2 AND EXISTS (SELECT 1 FROM gens g WHERE g.gen = n AND g.x = c.x AND g.y = c.y))`. Draw with recursive CTEs for the coordinates, ordered in a subquery before `group_concat`.
@@solution
CREATE TABLE offs (dx INTEGER, dy INTEGER);
INSERT INTO offs SELECT a.v, b.v FROM (SELECT -1 AS v UNION ALL SELECT 0 UNION ALL SELECT 1) a,
                                      (SELECT -1 AS v UNION ALL SELECT 0 UNION ALL SELECT 1) b
WHERE NOT (a.v = 0 AND b.v = 0);
CREATE TABLE gens (gen INTEGER, x INTEGER, y INTEGER);
INSERT INTO gens SELECT 0, x, y FROM live;

INSERT INTO gens
SELECT 1, c.x, c.y FROM (SELECT l.x + o.dx AS x, l.y + o.dy AS y, COUNT(*) AS n FROM gens l, offs o WHERE l.gen = 0 GROUP BY 1, 2) c
WHERE c.n = 3 OR (c.n = 2 AND EXISTS (SELECT 1 FROM gens g WHERE g.gen = 0 AND g.x = c.x AND g.y = c.y));
INSERT INTO gens
SELECT 2, c.x, c.y FROM (SELECT l.x + o.dx AS x, l.y + o.dy AS y, COUNT(*) AS n FROM gens l, offs o WHERE l.gen = 1 GROUP BY 1, 2) c
WHERE c.n = 3 OR (c.n = 2 AND EXISTS (SELECT 1 FROM gens g WHERE g.gen = 1 AND g.x = c.x AND g.y = c.y));
INSERT INTO gens
SELECT 3, c.x, c.y FROM (SELECT l.x + o.dx AS x, l.y + o.dy AS y, COUNT(*) AS n FROM gens l, offs o WHERE l.gen = 2 GROUP BY 1, 2) c
WHERE c.n = 3 OR (c.n = 2 AND EXISTS (SELECT 1 FROM gens g WHERE g.gen = 2 AND g.x = c.x AND g.y = c.y));
INSERT INTO gens
SELECT 4, c.x, c.y FROM (SELECT l.x + o.dx AS x, l.y + o.dy AS y, COUNT(*) AS n FROM gens l, offs o WHERE l.gen = 3 GROUP BY 1, 2) c
WHERE c.n = 3 OR (c.n = 2 AND EXISTS (SELECT 1 FROM gens g WHERE g.gen = 3 AND g.x = c.x AND g.y = c.y));

WITH RECURSIVE
  xs(x) AS (SELECT 0 UNION ALL SELECT x + 1 FROM xs WHERE x < 9),
  ys(y) AS (SELECT 0 UNION ALL SELECT y + 1 FROM ys WHERE y < 7),
  shown(gen) AS (SELECT 0 UNION ALL SELECT 4)
SELECT gen, row FROM (
    SELECT t.gen, t.y,
           group_concat(CASE WHEN EXISTS (SELECT 1 FROM gens g WHERE g.gen = t.gen AND g.x = t.x AND g.y = t.y)
                             THEN '#' ELSE '.' END, '') AS row
    FROM (SELECT shown.gen, ys.y, xs.x FROM shown, ys, xs ORDER BY shown.gen, ys.y, xs.x) t
    GROUP BY t.gen, t.y
)
ORDER BY gen, y;

@@exercise Which routers can be reached?
Some WAN links are down. Using only links with `up = 1` (both directions), list every office **reachable** from Tokyo and the minimum number of `hops`, plus offices that are `unreachable` (hops NULL). Sort by hops (NULLs last), then office.
@@setup
CREATE TABLE links (a TEXT, b TEXT, up INTEGER);
INSERT INTO links VALUES
  ('Tokyo', 'Osaka', 1), ('Tokyo', 'Sendai', 0), ('Osaka', 'Fukuoka', 1), ('Sendai', 'Sapporo', 1),
  ('Osaka', 'Hiroshima', 0), ('Fukuoka', 'Hiroshima', 1), ('Tokyo', 'Nagoya', 1), ('Okinawa', 'Fukuoka', 0);
@@starter
SELECT b FROM links WHERE a = 'Tokyo' AND up = 1;
@@expected
office    | hops
----------+-----
Tokyo     | 0
Nagoya    | 1
Osaka     | 1
Fukuoka   | 2
Hiroshima | 3
Okinawa   | NULL
Sapporo   | NULL
Sendai    | NULL
@@requires
WITH RECURSIVE
UNION
@@hint
Carry `(node, hops)` in a recursive CTE joined to the working links in both directions, with a hop limit (`WHERE r.hops < 10`) so cycles end, and take `MIN(hops)` per node. `UNION` instead of `UNION ALL` drops repeated `(node, hops)` rows, which keeps the work small. List all offices with `SELECT a FROM links UNION SELECT b FROM links`, LEFT JOIN the reachable ones, and sort NULLs last with `ORDER BY hops IS NULL, hops, office`.
@@solution
WITH RECURSIVE
  edges(src, dst) AS (SELECT a, b FROM links WHERE up = 1 UNION ALL SELECT b, a FROM links WHERE up = 1),
  reach(node, hops) AS (
    SELECT 'Tokyo', 0
    UNION
    SELECT e.dst, r.hops + 1 FROM reach r JOIN edges e ON e.src = r.node WHERE r.hops < 10
  ),
  best AS (SELECT node, MIN(hops) AS hops FROM reach GROUP BY node),
  offices(office) AS (SELECT a FROM links UNION SELECT b FROM links)
SELECT o.office, b.hops
FROM offices o LEFT JOIN best b ON b.node = o.office
ORDER BY b.hops IS NULL, b.hops, o.office;

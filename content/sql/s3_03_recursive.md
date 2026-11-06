@@lesson s3-recursive Recursive CTEs: Hierarchies, Series and Graphs
@@concepts loops, functions
@@text
A normal CTE names a query. A **recursive** CTE refers to *itself*, so it can walk a structure of unknown depth: an org chart, folders inside folders, network links, or simply the numbers 1 to 100.

## The shape

```sql
WITH RECURSIVE counter(n) AS (
    SELECT 1                     -- anchor: the starting row(s)
    UNION ALL
    SELECT n + 1 FROM counter    -- recursive step: built from the previous rows
    WHERE n < 5                  -- stop condition
)
SELECT n FROM counter;
```

How it runs:

1. Run the **anchor** query. Its rows are the first "working set".
2. Run the **recursive** part with `counter` meaning *only the rows from the previous step*. Add the new rows to the result and make them the new working set.
3. Repeat until a step produces no rows.

Forget the stop condition and it runs forever (or until the database's recursion limit). While experimenting, add a safety `LIMIT` to the final SELECT.

`RECURSIVE` is required by PostgreSQL and MySQL; SQLite accepts it and doesn't require it. SQL Server leaves the word out.

## Walking a hierarchy

Who's in Kenji's reporting line, and at what depth? Start at the top and join employees whose manager is in the previous level:

```sql
WITH RECURSIVE chain(id, name, depth) AS (
    SELECT id, name, 0 FROM employees WHERE manager_id IS NULL AND dept_id = 1
    UNION ALL
    SELECT e.id, e.name, c.depth + 1
    FROM employees e JOIN chain c ON e.manager_id = c.id
)
SELECT depth, name FROM chain ORDER BY depth, name;
```

Carry extra columns along: a `path` string (`'Kenji > Ana'`) shows the route and also sorts the output into tree order; a `depth` lets you indent with `substr('        ', 1, depth * 2) || name`.

Going **up** instead (an employee's managers to the top) is the same query with the join reversed: `JOIN chain c ON e.id = c.manager_id`.

## Generating series

Reports often need a row for every day, even days with no data. Generate the days, then LEFT JOIN the data:

```sql
WITH RECURSIVE days(day) AS (
    SELECT '2026-10-27'
    UNION ALL
    SELECT date(day, '+1 day') FROM days WHERE day < '2026-10-31'
)
SELECT day, (SELECT COUNT(*) FROM tickets t WHERE date(t.opened) = day) AS opened
FROM days;
```

PostgreSQL has `generate_series()` for this; SQLite has a `generate_series` table-valued function only when the extension is compiled in, so the recursive CTE is the portable way.

## Graphs and cycles

Networks (routers linked to routers), dependencies (package A needs B) and permissions (group inside group) are **graphs**, and graphs can have **cycles**. A recursive query on a cycle never stops. Guard against it by carrying the visited path and refusing to revisit:

```sql,norun
WITH RECURSIVE reach(node, path) AS (
    SELECT 'A', ',A,'
    UNION ALL
    SELECT l.dst, r.path || l.dst || ','
    FROM reach r JOIN links l ON l.src = r.node
    WHERE r.path NOT LIKE '%,' || l.dst || ',%'
)
SELECT * FROM reach;
```

PostgreSQL 14+ has a built-in `CYCLE` clause for this; the path trick works everywhere.

## UNION vs UNION ALL in recursion

With `UNION ALL`, duplicates are kept and the search continues from them. With `UNION`, a row that's already in the result is discarded, so it isn't expanded again; that alone stops simple cycles when the rows are identical (but not when you carry a growing path or depth).

## Limits

Each database caps recursion to stop runaway queries: SQL Server stops at 100 levels by default (`OPTION (MAXRECURSION n)`), PostgreSQL and SQLite have no fixed depth limit but will happily use all your memory. Recursive CTEs are great for hierarchies of hundreds or thousands of rows. For huge graphs, consider a graph database or storing extra structure (a closure table or a materialised path) that makes the query non-recursive.

@@quiz
What does the recursive part of a recursive CTE see when it refers to the CTE's name?
- The whole result so far
* Only the rows produced by the previous step
- The anchor rows only
- The base table
? Each step builds on the previous step's rows.

@@quiz
Your recursive query over a network of links never finishes. What's the most likely cause?
- Missing ORDER BY
* The graph has a cycle and nothing stops the query from following it forever
- UNION ALL is too slow
- Recursive CTEs can't use joins
? Carry the visited path and refuse to revisit nodes.

@@quiz
Why generate a series of days and LEFT JOIN the data to it?
- It's faster than GROUP BY
* So that days with no data still appear (with 0) instead of being missing
- Because dates can't be grouped
- To sort the dates
? GROUP BY can only output days that have rows.

@@drill
Which keywords start a recursive CTE?
= WITH RECURSIVE
? An anchor query, UNION ALL, then the recursive step.

@@flashcards
`WITH RECURSIVE` :: CTE that refers to itself
anchor member :: the starting rows
recursive member :: builds new rows from the previous step
stop condition :: WHERE in the recursive part
`depth + 1` :: track the level in a hierarchy
path string :: tree order and cycle detection
date series :: `date(day, '+1 day')` until the end date
`MAXRECURSION` :: SQL Server's recursion limit (default 100)

@@exercise The org chart
Print the whole organisation as an indented tree. Start with every employee who has no manager (top level), sorted by name, and put each person's reports under them, sorted by name. Show one column `tree` with two spaces of indentation per level and the title in brackets, like `  Ana Silva (Systems Engineer)`.
@@starter
SELECT name FROM employees WHERE manager_id IS NULL ORDER BY name;
@@expected
tree
-------------------------------
Hiro Sato (Sales Manager)
  Ben Jones (Account Executive)
  Yuki Kato (Account Executive)
Kenji Tanaka (IT Manager)
  Ana Silva (Systems Engineer)
  Mia Lee (Help Desk Analyst)
Omar Haddad (Contractor)
Rina Chen (Finance Manager)
  Taro Yamada (Accountant)
Sara Ito (Support Lead)
  Emi Suzuki (Support Agent)
  Leo Martin (Support Agent)
@@requires
WITH RECURSIVE
UNION ALL
@@hint
Carry `depth` and a `path` made of names: anchor `SELECT id, name, title, 0, name FROM employees WHERE manager_id IS NULL`, step `SELECT e.id, e.name, e.title, c.depth + 1, c.path || '/' || e.name FROM employees e JOIN tree c ON e.manager_id = c.id`. Order by `path`. Indent with `substr('          ', 1, depth * 2)`.
@@solution
WITH RECURSIVE tree(id, name, title, depth, path) AS (
    SELECT id, name, title, 0, name FROM employees WHERE manager_id IS NULL
    UNION ALL
    SELECT e.id, e.name, e.title, c.depth + 1, c.path || '/' || e.name
    FROM employees e JOIN tree c ON e.manager_id = c.id
)
SELECT substr('          ', 1, depth * 2) || name || ' (' || title || ')' AS tree
FROM tree
ORDER BY path;

@@exercise Tickets per day, no gaps
Show every day from `2026-10-24` to `2026-10-31` with the number of tickets `opened` that day and the `running` total since the 24th. Days with no tickets must show 0.
@@starter
SELECT date(opened) AS day, COUNT(*) AS opened FROM tickets
WHERE opened >= '2026-10-24' GROUP BY day ORDER BY day;
@@expected
day        | opened | running
-----------+--------+--------
2026-10-24 | 1      | 1
2026-10-25 | 0      | 1
2026-10-26 | 0      | 1
2026-10-27 | 1      | 2
2026-10-28 | 0      | 2
2026-10-29 | 1      | 3
2026-10-30 | 2      | 5
2026-10-31 | 2      | 7
@@requires
WITH RECURSIVE
LEFT JOIN
@@hint
Generate `days(day)` with a recursive CTE, LEFT JOIN `tickets t ON date(t.opened) = d.day`, group by day and count `t.id`. Running total: `SUM(COUNT(t.id)) OVER (ORDER BY d.day)`.
@@solution
WITH RECURSIVE days(day) AS (
    SELECT '2026-10-24'
    UNION ALL
    SELECT date(day, '+1 day') FROM days WHERE day < '2026-10-31'
)
SELECT d.day, COUNT(t.id) AS opened, SUM(COUNT(t.id)) OVER (ORDER BY d.day) AS running
FROM days d
LEFT JOIN tickets t ON date(t.opened) = d.day
GROUP BY d.day
ORDER BY d.day;

@@exercise Which servers does a failure take down?
`depends(service, needs)` says which service needs which. If `storage` fails, which services are affected, directly or indirectly, and how many hops away? Show `service` and the smallest `hops`, sorted by hops then name. The data has a cycle (`auth` ↔ `ldap`), so guard against it.
@@setup
CREATE TABLE depends (service TEXT, needs TEXT);
INSERT INTO depends VALUES
  ('database', 'storage'), ('fileshare', 'storage'), ('backup', 'storage'),
  ('webapp', 'database'), ('webapp', 'auth'), ('auth', 'ldap'), ('ldap', 'auth'),
  ('ldap', 'database'), ('reports', 'webapp'), ('email', 'auth'), ('printing', 'fileshare'), ('wiki', 'cdn');
@@starter
SELECT service FROM depends WHERE needs = 'storage';
@@expected
service   | hops
----------+-----
backup    | 1
database  | 1
fileshare | 1
ldap      | 2
printing  | 2
webapp    | 2
auth      | 3
reports   | 3
email     | 4
@@requires
WITH RECURSIVE
NOT LIKE
MIN(
@@hint
Walk from the failed service to everything that needs it: anchor `SELECT 'storage', 0, ',storage,'`, step `SELECT d.service, i.hops + 1, i.path || d.service || ',' FROM impact i JOIN depends d ON d.needs = i.service WHERE i.path NOT LIKE '%,' || d.service || ',%'`. Then `SELECT service, MIN(hops) ... WHERE hops > 0 GROUP BY service`.
@@solution
WITH RECURSIVE impact(service, hops, path) AS (
    SELECT 'storage', 0, ',storage,'
    UNION ALL
    SELECT d.service, i.hops + 1, i.path || d.service || ','
    FROM impact i JOIN depends d ON d.needs = i.service
    WHERE i.path NOT LIKE '%,' || d.service || ',%'
)
SELECT service, MIN(hops) AS hops
FROM impact
WHERE hops > 0
GROUP BY service
ORDER BY hops, service;

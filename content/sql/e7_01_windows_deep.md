@@lesson e7-windows Window Functions in Depth: Frames, LAG/LEAD and Gaps and Islands
@@concepts lists, loops
@@text
Module 4 introduced `ROW_NUMBER`, `RANK` and running totals. This lesson covers the rest of the toolbox: what exactly a window **frame** is, comparing a row with its neighbours, and a few patterns that show up in every analytics job.

## The full syntax

```text
function(...) OVER (
    PARTITION BY ...          -- separate groups (optional)
    ORDER BY ...              -- order inside each group
    ROWS | RANGE | GROUPS BETWEEN <start> AND <end>   -- the frame
)
```

The **frame** is the set of rows, relative to the current row, that the function looks at. Bounds are `UNBOUNDED PRECEDING`, `n PRECEDING`, `CURRENT ROW`, `n FOLLOWING` and `UNBOUNDED FOLLOWING`.

The default frame catches people out. With an `ORDER BY`, the default is `RANGE BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW`: from the start of the partition to the current row **and all its ties**. Without `ORDER BY`, the frame is the whole partition.

```sql
SELECT id, priority,
       COUNT(*) OVER (ORDER BY priority) AS range_count,
       COUNT(*) OVER (ORDER BY priority ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW) AS rows_count
FROM tickets ORDER BY priority, id LIMIT 6;
```

`RANGE` treats rows with the same ORDER BY value as one "peer group", so every priority-1 ticket gets the same count; `ROWS` counts physical rows. For running totals over data with ties, say `ROWS` explicitly.

## Moving averages and sliding windows

```sql
WITH daily(day, n) AS (
    SELECT date(opened), COUNT(*) FROM tickets GROUP BY date(opened)
)
SELECT day, n,
       ROUND(AVG(n) OVER (ORDER BY day ROWS BETWEEN 2 PRECEDING AND CURRENT ROW), 2) AS avg_last3
FROM daily ORDER BY day;
```

`ROWS BETWEEN 2 PRECEDING AND CURRENT ROW` is "this row and the two before it". Note it counts **rows**, not days: if days are missing, the window spans more calendar time. Generate a day series first (recursive CTE) when calendar accuracy matters, or use `RANGE` with a numeric order key such as `julianday(day)`: `RANGE BETWEEN 2 PRECEDING AND CURRENT ROW` then means "within 2 days".

## Neighbours: LAG and LEAD

`LAG(expr, n, default)` reads a value from n rows **before** the current one in the window order; `LEAD` reads **after**:

```sql
SELECT id, username, time,
       LAG(time) OVER (PARTITION BY ip ORDER BY time) AS previous_from_ip
FROM logins WHERE success = 0 ORDER BY id;
```

Typical uses: time between events, change since last month, "did the status change?".

## First, last and nth values

`FIRST_VALUE(x)`, `LAST_VALUE(x)` and `NTH_VALUE(x, n)` return a value from the frame. Beware: with the default frame, `LAST_VALUE` returns the **current row** (the frame ends there). Use `ROWS BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING`.

## Ranking and distribution

| Function | What it returns |
|---|---|
| `ROW_NUMBER()` | 1, 2, 3, ... (ties broken arbitrarily) |
| `RANK()` | 1, 2, 2, 4 (gaps after ties) |
| `DENSE_RANK()` | 1, 2, 2, 3 |
| `NTILE(n)` | bucket number 1..n of roughly equal size |
| `PERCENT_RANK()` | (rank − 1) / (rows − 1), from 0 to 1 |
| `CUME_DIST()` | share of rows ≤ the current one |

## Named windows

When several functions share a window, name it once:

```sql
SELECT name, salary,
       RANK() OVER w AS salary_rank,
       salary - LAG(salary) OVER w AS gap_to_next
FROM employees
WINDOW w AS (ORDER BY salary DESC)
ORDER BY salary DESC LIMIT 4;
```

## Filtering on a window function

Window functions run **after** WHERE, GROUP BY and HAVING, so you can't use them in WHERE. Wrap the query in a CTE and filter outside. (Some databases add `QUALIFY` for this: Snowflake, BigQuery, DuckDB.)

## Gaps and islands

A classic interview and analytics problem: find runs of consecutive rows. "Streaks of failed logins", "consecutive days a server was down", "ranges of free IP addresses".

The trick: for consecutive values, `value − ROW_NUMBER()` is **constant** within a run and changes when there's a gap. Group by that difference:

```sql
WITH seq(n) AS (VALUES (1), (2), (3), (7), (8), (10), (11), (12))
SELECT MIN(n) AS run_start, MAX(n) AS run_end, COUNT(*) AS length
FROM (SELECT n, n - ROW_NUMBER() OVER (ORDER BY n) AS grp FROM seq)
GROUP BY grp ORDER BY run_start;
```

For runs of the same **status** (not consecutive numbers), use two row numbers: `ROW_NUMBER() OVER (ORDER BY t) − ROW_NUMBER() OVER (PARTITION BY status ORDER BY t)` is constant within each run of one status.

Another approach, often easier to reason about: mark where a new run **starts** (`CASE WHEN LAG(x) ... <> x THEN 1 ELSE 0 END`), then a running `SUM` of those marks numbers the runs.

@@quiz
With `ORDER BY` and no frame clause, what does `SUM(x) OVER (ORDER BY d)` include?
- Only the current row
- The whole partition
* All rows from the start up to the current row, including rows tied with it on `d`
- The previous row only
? The default frame is RANGE ... CURRENT ROW, which includes peers.

@@quiz
Why does `LAST_VALUE(x) OVER (ORDER BY d)` usually return the current row's x?
* Because the default frame ends at the current row
- Because LAST_VALUE is broken
- Because of ties
- Because it ignores ORDER BY
? Use ROWS BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING.

@@quiz
How can you keep only rows where `ROW_NUMBER() OVER (...) = 1`?
- `WHERE ROW_NUMBER() OVER (...) = 1`
- `HAVING ROW_NUMBER() ... = 1`
* Compute it in a CTE or subquery and filter in the outer query
- It's impossible
? Window functions are evaluated after WHERE and HAVING.

@@quiz
In the gaps-and-islands trick, what is constant within a run of consecutive numbers?
- ROW_NUMBER()
* n − ROW_NUMBER()
- n + ROW_NUMBER()
- RANK()
? Both increase by 1 per row inside a run.

@@drill
Which window function reads the previous row's value?
= LAG
= LAG()
? LEAD reads the next row.

@@flashcards
frame :: rows around the current row a window function sees
default frame with ORDER BY :: RANGE UNBOUNDED PRECEDING to CURRENT ROW
`ROWS` vs `RANGE` :: physical rows vs peer groups / value ranges
`LAG(x, n, default)` :: value n rows before
`LEAD(x)` :: value after
`NTILE(4)` :: quartile buckets
`WINDOW w AS (...)` :: name a window
`QUALIFY` :: filter on window results (Snowflake, DuckDB)
gaps and islands :: group by `n − ROW_NUMBER()`

@@exercise Time between failed logins
For each **failed** login, show `id`, `ip`, `time`, and `secs_since_prev`: seconds since the previous failed login **from the same IP** (NULL for the first). Sort by id.
@@starter
SELECT id, ip, time FROM logins WHERE success = 0 ORDER BY id;
@@expected
id | ip            | time                | secs_since_prev
---+---------------+---------------------+----------------
3  | 203.0.113.7   | 2026-10-29 22:31:00 | NULL
4  | 203.0.113.7   | 2026-10-29 22:31:04 | 4
5  | 203.0.113.7   | 2026-10-29 22:31:09 | 5
6  | 203.0.113.7   | 2026-10-29 22:31:15 | 6
7  | 203.0.113.7   | 2026-10-29 22:31:22 | 7
10 | 10.0.1.15     | 2026-10-30 09:15:00 | NULL
12 | 198.51.100.23 | 2026-10-30 03:12:00 | NULL
13 | 198.51.100.23 | 2026-10-30 03:12:03 | 3
15 | 198.51.100.23 | 2026-10-31 03:00:00 | 85677
@@requires
LAG(
PARTITION BY ip
@@hint
`CAST(ROUND((julianday(time) - julianday(LAG(time) OVER (PARTITION BY ip ORDER BY time))) * 86400) AS INTEGER)`.
@@solution
SELECT id, ip, time,
       CAST(ROUND((julianday(time) - julianday(LAG(time) OVER (PARTITION BY ip ORDER BY time))) * 86400) AS INTEGER) AS secs_since_prev
FROM logins
WHERE success = 0
ORDER BY id;

@@exercise Salary bands
Split employees into 4 salary bands with `NTILE` (band 1 = highest). Show `band`, how many `people`, and the `lowest` and `highest` salary in each band.
@@starter
SELECT name, salary FROM employees ORDER BY salary DESC;
@@expected
band | people | lowest  | highest
-----+--------+---------+--------
1    | 3      | 8800000 | 9200000
2    | 3      | 5600000 | 6400000
3    | 3      | 4500000 | 5500000
4    | 3      | 3900000 | 4100000
@@requires
NTILE(4)
@@hint
Compute `NTILE(4) OVER (ORDER BY salary DESC) AS band` in a CTE, then `GROUP BY band`.
@@solution
WITH banded AS (
    SELECT salary, NTILE(4) OVER (ORDER BY salary DESC) AS band FROM employees
)
SELECT band, COUNT(*) AS people, MIN(salary) AS lowest, MAX(salary) AS highest
FROM banded
GROUP BY band
ORDER BY band;

@@exercise Server uptime streaks
`checks(day, server, up)` has one health check per server per day. Find each run of consecutive **down** days: show `server`, `from_day`, `to_day` and `days`. Sort by server and from_day.
@@setup
CREATE TABLE checks (day TEXT, server TEXT, up INTEGER);
WITH RECURSIVE d(day) AS (SELECT '2026-10-01' UNION ALL SELECT date(day, '+1 day') FROM d WHERE day < '2026-10-14')
INSERT INTO checks
SELECT day, s, CASE
    WHEN s = 'SV-0001' AND day IN ('2026-10-03', '2026-10-04', '2026-10-05', '2026-10-11') THEN 0
    WHEN s = 'SV-0002' AND day IN ('2026-10-08', '2026-10-09', '2026-10-13', '2026-10-14') THEN 0
    ELSE 1 END
FROM d, (SELECT 'SV-0001' AS s UNION ALL SELECT 'SV-0002');
@@starter
SELECT server, day FROM checks WHERE up = 0 ORDER BY server, day;
@@expected
server  | from_day   | to_day     | days
--------+------------+------------+-----
SV-0001 | 2026-10-03 | 2026-10-05 | 3
SV-0001 | 2026-10-11 | 2026-10-11 | 1
SV-0002 | 2026-10-08 | 2026-10-09 | 2
SV-0002 | 2026-10-13 | 2026-10-14 | 2
@@requires
ROW_NUMBER()
GROUP BY
@@hint
For down days only, `julianday(day) - ROW_NUMBER() OVER (PARTITION BY server ORDER BY day)` is the same for every day in a run. Group by server and that value; `MIN(day)`, `MAX(day)`, `COUNT(*)`.
@@solution
WITH down AS (
    SELECT server, day,
           julianday(day) - ROW_NUMBER() OVER (PARTITION BY server ORDER BY day) AS grp
    FROM checks WHERE up = 0
)
SELECT server, MIN(day) AS from_day, MAX(day) AS to_day, COUNT(*) AS days
FROM down
GROUP BY server, grp
ORDER BY server, from_day;

@@exercise Department salary context
For every employee in a department, show `name`, `department` name, `salary`, the department's `top` salary, `pct_of_top` (salary as a whole-number percentage of the top) and `next_lower`: the next lower salary in the same department (NULL for the lowest). Use a named window. Sort by department, then salary descending.
@@starter
SELECT e.name, d.name AS department, e.salary
FROM employees e JOIN departments d ON d.id = e.dept_id
ORDER BY department, salary DESC;
@@expected
name         | department | salary  | top     | pct_of_top | next_lower
-------------+------------+---------+---------+------------+-----------
Rina Chen    | Finance    | 9000000 | 9000000 | 100        | 5500000
Taro Yamada  | Finance    | 5500000 | 9000000 | 61         | NULL
Kenji Tanaka | IT         | 9200000 | 9200000 | 100        | 6400000
Ana Silva    | IT         | 6400000 | 9200000 | 69         | 4100000
Mia Lee      | IT         | 4100000 | 9200000 | 44         | NULL
Hiro Sato    | Sales      | 8800000 | 8800000 | 100        | 5600000
Ben Jones    | Sales      | 5600000 | 8800000 | 63         | 5200000
Yuki Kato    | Sales      | 5200000 | 8800000 | 59         | NULL
Sara Ito     | Support    | 6100000 | 6100000 | 100        | 4000000
Emi Suzuki   | Support    | 4000000 | 6100000 | 65         | 3900000
Leo Martin   | Support    | 3900000 | 6100000 | 63         | NULL
@@requires
WINDOW w AS
LEAD(
@@hint
`WINDOW w AS (PARTITION BY d.id ORDER BY e.salary DESC)`; `MAX(e.salary) OVER (PARTITION BY d.id)` for the top (or `FIRST_VALUE(e.salary) OVER w`); `LEAD(e.salary) OVER w` for the next lower.
@@solution
SELECT e.name, d.name AS department, e.salary,
       FIRST_VALUE(e.salary) OVER w AS top,
       100 * e.salary / FIRST_VALUE(e.salary) OVER w AS pct_of_top,
       LEAD(e.salary) OVER w AS next_lower
FROM employees e JOIN departments d ON d.id = e.dept_id
WINDOW w AS (PARTITION BY d.id ORDER BY e.salary DESC)
ORDER BY department, e.salary DESC;

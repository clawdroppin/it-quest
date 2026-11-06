@@lesson s4-window Window Functions
@@text
Window functions calculate across a set of rows **without collapsing them** into one row like GROUP BY does. They're how analysts write rankings, running totals and "compared with the previous one" reports, and a favourite topic in data job interviews.

```sql
SELECT name, dept_id, salary,
       RANK() OVER (PARTITION BY dept_id ORDER BY salary DESC) AS rank_in_dept,
       AVG(salary) OVER (PARTITION BY dept_id) AS dept_avg
FROM employees
WHERE dept_id IS NOT NULL
ORDER BY dept_id, rank_in_dept;
```

`OVER (...)` defines the window: `PARTITION BY` splits rows into groups (like GROUP BY, but each row stays), `ORDER BY` orders within each group.

| Function | Gives |
|---|---|
| `ROW_NUMBER()` | 1, 2, 3... (no ties) |
| `RANK()` / `DENSE_RANK()` | rankings with ties (1, 1, 3 / 1, 1, 2) |
| `SUM(x) OVER (ORDER BY ...)` | a running total |
| `LAG(x)` / `LEAD(x)` | the previous / next row's value |
| `COUNT(*) OVER (...)` | the group's size on every row |

## Running totals and gaps

```sql
SELECT purchased, asset_tag, price,
       SUM(price) OVER (ORDER BY purchased, id) AS spent_so_far
FROM devices
WHERE type = 'laptop';
```

```sql
SELECT username, time,
       LAG(time) OVER (PARTITION BY username ORDER BY time) AS previous_attempt
FROM logins
WHERE username = 'asilva';
```

Seconds between login attempts is a classic way to tell a human typing from a bot: humans don't retry every 2 seconds at 22:31.

## Top N per group

"The newest device of each type" needs a window and a CTE:

```sql
WITH ranked AS (
    SELECT type, asset_tag, purchased,
           ROW_NUMBER() OVER (PARTITION BY type ORDER BY purchased DESC, id DESC) AS n
    FROM devices
)
SELECT type, asset_tag, purchased FROM ranked WHERE n = 1 ORDER BY type;
```

@@quiz
What's the key difference between `GROUP BY dept_id` and `OVER (PARTITION BY dept_id)`?
* GROUP BY returns one row per group; a window keeps every row and adds the group's value to each
- They're identical
- PARTITION BY can only count
- GROUP BY is slower
? Window functions add columns without removing rows.

@@quiz
Scores 90, 90, 80. What does `DENSE_RANK()` give?
* 1, 1, 2
- 1, 2, 3
- 1, 1, 3
- 2, 2, 1
? DENSE_RANK leaves no gaps after ties; RANK would give 1, 1, 3.

@@quiz
What does `LAG(total) OVER (ORDER BY day)` return for each row?
* The total from the previous row in day order
- The total of all previous rows
- The next row's total
- The smallest total
? LAG and LEAD look backwards and forwards; useful for day-over-day changes.

@@drill
Which window function numbers rows 1, 2, 3 within each partition?
= ROW_NUMBER
= ROW_NUMBER()
? Use it for top N per group.

@@flashcards
OVER () :: Turns an aggregate into a window function
PARTITION BY :: Groups for a window function, keeping every row
ROW_NUMBER() :: 1, 2, 3 within each partition
RANK() / DENSE_RANK() :: Rankings with ties, with / without gaps
LAG() / LEAD() :: Value from the previous / next row
running total :: SUM(x) OVER (ORDER BY ...)
top N per group :: ROW_NUMBER() in a CTE, then WHERE n <= N

@@exercise Highest paid per department
Show the **highest paid** employee of each department: department `name` (as `department`), employee `name` (as `employee`) and `salary`. Use `ROW_NUMBER()` in a CTE. Order by department name.
@@starter
-- WITH ranked AS (...) SELECT ...

@@expected
department | employee     | salary
-----------+--------------+--------
Finance    | Rina Chen    | 9000000
IT         | Kenji Tanaka | 9200000
Sales      | Hiro Sato    | 8800000
Support    | Sara Ito     | 6100000
@@requires
OVER
@@hint
`ROW_NUMBER() OVER (PARTITION BY dept_id ORDER BY salary DESC) AS n` in the CTE (only employees with a department), then join departments and keep `n = 1`.
@@solution
WITH ranked AS (
    SELECT name, dept_id, salary,
           ROW_NUMBER() OVER (PARTITION BY dept_id ORDER BY salary DESC) AS n
    FROM employees
    WHERE dept_id IS NOT NULL
)
SELECT d.name AS department, r.name AS employee, r.salary
FROM ranked AS r
JOIN departments AS d ON d.id = r.dept_id
WHERE r.n = 1
ORDER BY d.name;

@@exercise Tickets per day, running total
For October 2026, show each `day` (the date part of `opened`) with the number of tickets opened that day as `opened`, and a running total as `total_so_far`. Order by day.
@@starter
-- date(opened) gives the day

@@expected
day        | opened | total_so_far
-----------+--------+-------------
2026-10-01 | 1      | 1
2026-10-03 | 1      | 2
2026-10-06 | 1      | 3
2026-10-08 | 1      | 4
2026-10-10 | 1      | 5
2026-10-12 | 1      | 6
2026-10-20 | 1      | 7
2026-10-21 | 1      | 8
2026-10-24 | 1      | 9
2026-10-27 | 1      | 10
2026-10-29 | 1      | 11
2026-10-30 | 2      | 13
2026-10-31 | 2      | 15
@@requires
OVER
@@hint
Group first in a CTE (`SELECT date(opened) AS day, COUNT(*) AS opened FROM tickets GROUP BY day`), then `SUM(opened) OVER (ORDER BY day)`.
@@solution
WITH daily AS (
    SELECT date(opened) AS day, COUNT(*) AS opened
    FROM tickets
    WHERE opened LIKE '2026-10-%'
    GROUP BY day
)
SELECT day, opened, SUM(opened) OVER (ORDER BY day) AS total_so_far
FROM daily
ORDER BY day;

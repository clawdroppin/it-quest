@@lesson s1-null NULL and Three-Valued Logic
@@concepts conditions
@@text
`NULL` is the most misunderstood thing in SQL. It means **"unknown"** or **"not applicable"**: a device with no owner, a ticket that isn't closed yet, a contractor with no department. It is *not* zero, *not* an empty string and *not* false.

## Three-valued logic

Because NULL means unknown, a comparison with NULL is also unknown. SQL conditions have **three** possible results: `TRUE`, `FALSE` and `UNKNOWN` (shown as NULL).

| Expression | Result | Why |
|---|---|---|
| `NULL = NULL` | NULL | two unknowns might or might not be equal |
| `NULL <> 5` | NULL | we can't say |
| `NULL + 100` | NULL | unknown plus anything is unknown |
| `'IT' || NULL` | NULL | concatenation too |
| `NULL AND FALSE` | FALSE | false whatever the unknown is |
| `NULL AND TRUE` | NULL | depends on the unknown |
| `NULL OR TRUE` | TRUE | true whatever the unknown is |
| `NOT NULL` | NULL | the opposite of unknown is unknown |

`WHERE` keeps only rows where the condition is **TRUE**. Rows where it is FALSE *or* UNKNOWN are dropped. That's why this finds no contractor:

```sql
SELECT name FROM employees WHERE dept_id <> 1;
```

Omar (dept_id NULL) is in no department, so you'd expect him in "not IT", but `NULL <> 1` is unknown, so he's dropped. To include him:

```sql
SELECT name FROM employees WHERE dept_id <> 1 OR dept_id IS NULL;
```

SQLite (and PostgreSQL) also have a NULL-safe comparison: `a IS b` is true when both are NULL or both are equal, and `a IS NOT b` is its opposite. (Standard SQL spells it `IS DISTINCT FROM`; MySQL uses `<=>`.)

```sql
SELECT name FROM employees WHERE dept_id IS NOT 1;
```

## Functions for NULL

| Function | Meaning | Example |
|---|---|---|
| `COALESCE(a, b, ...)` | the first non-NULL argument | `COALESCE(os, 'none')` |
| `IFNULL(a, b)` | two-argument COALESCE (SQLite, MySQL) | `IFNULL(closed, 'still open')` |
| `NULLIF(a, b)` | NULL if a = b, else a | `NULLIF(price, 0)` avoids division by zero |

```sql
SELECT asset_tag, COALESCE(os, '(no OS)') AS os FROM devices WHERE type IN ('printer', 'server');
```

`NULLIF` is the classic guard against dividing by zero: `x / NULLIF(y, 0)` gives NULL instead of an error (or, in SQLite, instead of a silent NULL anyway, but in PostgreSQL and SQL Server division by zero is an error).

## NULL in aggregates

Aggregate functions **skip NULLs**, with one exception:

- `COUNT(*)` counts rows.
- `COUNT(column)` counts the non-NULL values in that column.
- `SUM`, `AVG`, `MIN`, `MAX` ignore NULLs. `AVG` divides by the number of non-NULL values, not the number of rows.
- `SUM` of no values (or only NULLs) is NULL, not 0. Use `COALESCE(SUM(x), 0)` (SQLite also has `TOTAL(x)`, which returns 0.0).

```sql
SELECT COUNT(*) AS tickets, COUNT(closed) AS closed, COUNT(assignee_id) AS assigned FROM tickets;
```

## NULL in GROUP BY, ORDER BY and DISTINCT

Here NULLs are treated as **equal to each other**: all NULL dept_ids form one group, and `DISTINCT` returns one NULL. When sorting, SQLite and MySQL put NULLs **first** in ascending order; PostgreSQL and Oracle put them **last**. Be explicit when it matters: `ORDER BY closed NULLS LAST` (SQLite 3.30+, PostgreSQL) or `ORDER BY closed IS NULL, closed` (works everywhere).

## The NOT IN trap

This looks like "employees who have never opened a ticket":

```sql
SELECT name FROM employees
WHERE id NOT IN (SELECT assignee_id FROM tickets);
```

It returns **no rows at all**, because `assignee_id` contains NULLs. `x NOT IN (1, 2, NULL)` means `x <> 1 AND x <> 2 AND x <> NULL`, and the last part is always unknown. Fixes: filter the NULLs out of the subquery (`WHERE assignee_id IS NOT NULL`), or use `NOT EXISTS`, which doesn't have this problem. Prefer `NOT EXISTS`.

## Designing with NULL

Use `NOT NULL` on every column that must always have a value, and allow NULL only where "unknown" or "not applicable" is a real state. Avoid "magic" values like `'1900-01-01'` or `-1` for missing data: they sneak into averages and comparisons in a way NULL never does.

@@quiz
What does `WHERE dept_id <> 1` do with rows where `dept_id` is NULL?
- Keeps them, because NULL isn't 1
* Drops them, because `NULL <> 1` is unknown, and WHERE keeps only TRUE
- Raises an error
- Keeps them only in SQLite
? WHERE keeps rows where the condition is TRUE; UNKNOWN is dropped like FALSE.

@@quiz
`SELECT COUNT(*), COUNT(closed) FROM tickets` returns 15 and 8. Why are they different?
* `COUNT(column)` skips NULLs, and 7 tickets have no closed time
- `COUNT(*)` counts duplicates twice
- `COUNT(closed)` counts distinct values
- One of them is wrong
? COUNT(*) counts rows; COUNT(x) counts non-NULL x.

@@quiz
Why can `WHERE id NOT IN (SELECT assignee_id FROM tickets)` return no rows at all?
- NOT IN doesn't work with subqueries
- The subquery returns too many rows
* The subquery contains a NULL, which makes every `id <> NULL` comparison unknown
- IDs must be text
? Use NOT EXISTS, or filter NULLs out of the subquery.

@@quiz
What does `NULL OR TRUE` evaluate to?
- NULL
- FALSE
* TRUE
- An error
? Whatever the unknown is, the OR is true.

@@flashcards
`NULL` :: unknown / not applicable, not 0 or ''
`x = NULL` :: always unknown: use `IS NULL`
`a IS b` :: NULL-safe equality (SQLite, PostgreSQL)
`COALESCE(a, b, c)` :: first non-NULL
`NULLIF(a, b)` :: NULL when a = b
`COUNT(*)` vs `COUNT(col)` :: rows vs non-NULL values
`SUM` of nothing :: NULL: wrap in COALESCE
NOT IN with NULLs :: returns nothing: use NOT EXISTS
`NULLS LAST` :: control where NULLs sort

@@drill
Write an expression that shows a device's `os`, or `unknown` when it is NULL. Use COALESCE.
= COALESCE(os, 'unknown')
= coalesce(os, 'unknown')
? COALESCE returns the first non-NULL argument.

@@exercise Everyone outside IT
List the `name` and `dept_id` of every employee who is **not** in department 1, including people with no department. Sort by name.
@@starter
SELECT name, dept_id FROM employees WHERE dept_id <> 1 ORDER BY name;
@@expected
name        | dept_id
------------+--------
Ben Jones   | 2
Emi Suzuki  | 4
Hiro Sato   | 2
Leo Martin  | 4
Omar Haddad | NULL
Rina Chen   | 3
Sara Ito    | 4
Taro Yamada | 3
Yuki Kato   | 2
@@requires
dept_id IS
@@hint
Either `dept_id <> 1 OR dept_id IS NULL`, or the NULL-safe `dept_id IS NOT 1`.
@@solution
SELECT name, dept_id FROM employees WHERE dept_id IS NOT 1 ORDER BY name;

@@exercise Ticket counts that tell the truth
In one row, show: `total` tickets, how many are `closed_at_set` (have a `closed` time), how many are `unassigned` (no assignee), and `open_share` = the percentage of tickets without a closed time, as an integer.
@@starter
-- your query here
@@expected
total | closed_at_set | unassigned | open_share
------+---------------+------------+-----------
15    | 8             | 3          | 46
@@requires
COUNT(closed)
COUNT(*)
@@hint
Unassigned = `COUNT(*) - COUNT(assignee_id)`. Percentage: `100 * (COUNT(*) - COUNT(closed)) / COUNT(*)` (integer division in SQLite).
@@solution
SELECT COUNT(*) AS total,
       COUNT(closed) AS closed_at_set,
       COUNT(*) - COUNT(assignee_id) AS unassigned,
       100 * (COUNT(*) - COUNT(closed)) / COUNT(*) AS open_share
FROM tickets;

@@exercise Employees who never handled a ticket
List the names of employees who have never been the **assignee** of a ticket, sorted by name. The `NOT IN` version in the starter returns nothing because of NULLs: fix it with `NOT EXISTS`.
@@starter
SELECT name FROM employees
WHERE id NOT IN (SELECT assignee_id FROM tickets)
ORDER BY name;
@@expected
name
-----------
Ben Jones
Emi Suzuki
Hiro Sato
Leo Martin
Omar Haddad
Rina Chen
Sara Ito
Taro Yamada
Yuki Kato
@@requires
NOT EXISTS
@@hint
`WHERE NOT EXISTS (SELECT 1 FROM tickets t WHERE t.assignee_id = e.id)` with `FROM employees e`.
@@solution
SELECT name FROM employees e
WHERE NOT EXISTS (SELECT 1 FROM tickets t WHERE t.assignee_id = e.id)
ORDER BY name;

@@exercise Devices and spend per person
For each employee, show `name`, how many `devices` they have and the `spend` (total price), with 0 for people with no devices (not NULL). Use a LEFT JOIN. Sort by spend (highest first), then name.
@@starter
SELECT e.name, COUNT(*) AS devices, SUM(d.price) AS spend
FROM employees e LEFT JOIN devices d ON d.employee_id = e.id
GROUP BY e.id
ORDER BY spend DESC, e.name;
@@expected
name         | devices | spend
-------------+---------+-------
Ben Jones    | 2       | 322800
Kenji Tanaka | 1       | 285000
Ana Silva    | 1       | 210000
Mia Lee      | 1       | 210000
Taro Yamada  | 1       | 180000
Omar Haddad  | 1       | 175000
Emi Suzuki   | 1       | 140000
Leo Martin   | 1       | 140000
Hiro Sato    | 1       | 124800
Rina Chen    | 0       | 0
Sara Ito     | 0       | 0
Yuki Kato    | 0       | 0
@@requires
COALESCE
COUNT(d.id)
@@hint
`COUNT(*)` counts the NULL row that LEFT JOIN creates for people with no devices; count `d.id` instead. Wrap the sum: `COALESCE(SUM(d.price), 0)`.
@@solution
SELECT e.name, COUNT(d.id) AS devices, COALESCE(SUM(d.price), 0) AS spend
FROM employees e LEFT JOIN devices d ON d.employee_id = e.id
GROUP BY e.id
ORDER BY spend DESC, e.name;

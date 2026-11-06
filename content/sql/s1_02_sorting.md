@@lesson s1-sorting Sorting, Limits, Aliases and Calculations
@@text
## ORDER BY

Without `ORDER BY`, a database may return rows in any order. Ask for one when it matters:

```sql
SELECT name, hired FROM employees ORDER BY hired;
```

```sql
SELECT id, title, priority, opened FROM tickets
ORDER BY priority, opened DESC;
```

`ASC` (the default) is smallest first, `DESC` largest first. Several columns: sort by the first, and use the next to break ties.

## LIMIT and DISTINCT

```sql
SELECT name, salary FROM employees ORDER BY salary DESC LIMIT 3;
SELECT DISTINCT type FROM devices ORDER BY type;
```

`LIMIT n` keeps the first n rows (`OFFSET m` skips some first, for paging). `DISTINCT` removes duplicate rows. (SQL Server writes `SELECT TOP 3 ...` instead of LIMIT.)

## Calculations and aliases

Columns can be expressions, and `AS` gives them a readable name:

```sql
SELECT name,
       salary / 12 AS monthly_yen,
       salary * 1.05 AS after_raise
FROM employees
WHERE dept_id = 1;
```

```sql
SELECT asset_tag, model || ' (' || type || ')' AS description FROM devices LIMIT 3;
```

`||` joins text in SQLite and PostgreSQL (`CONCAT(a, b)` in MySQL and SQL Server).

> Integer division: `7 / 2` is `3` when both are integers. Write `7 / 2.0` or `7 * 1.0 / 2` to get `3.5`.
## Putting it together

The clauses always appear in the same order, even though the database processes them differently:

```sql,norun
SELECT name, ram_gb * 1024 AS ram_mb
FROM devices
WHERE os = 'Windows 11'
ORDER BY ram_mb DESC, name
LIMIT 5;
```

The logical order of processing is `FROM` → `WHERE` → `SELECT` → `ORDER BY` → `LIMIT`. That explains two common surprises:

- You **can** use a column alias like `ram_mb` in `ORDER BY`, because sorting happens after `SELECT` has named the columns.
- You **can't** use it in `WHERE` (in most databases), because filtering happens before the alias exists. Repeat the expression instead: `WHERE ram_gb * 1024 > 8000`.

Without `ORDER BY`, the database may return rows in any order, and that order can change from one run to the next. Whenever the order matters (reports, "top 5", pagination), always say how to sort, and add a unique column such as an ID as a final tie-breaker so the result is stable.

@@quiz
`ORDER BY priority, opened DESC` sorts by...
* priority ascending, then newest opened first among equal priorities
- both descending
- opened only
- priority descending, then opened descending
? `DESC` applies only to the column right before it.

@@quiz
What does `SELECT DISTINCT os FROM devices;` return?
* each different os value once
- the number of operating systems
- all rows sorted by os
- only devices with an os
? DISTINCT removes duplicate rows from the result. (NULL is listed once too.)

@@quiz
Why can you use an alias in ORDER BY but usually not in WHERE?
* WHERE runs before SELECT names the columns; ORDER BY runs after
- Aliases only work with numbers
- WHERE doesn't accept expressions
- ORDER BY ignores aliases
? Logical order: FROM, WHERE, SELECT, ORDER BY, LIMIT.

@@drill
Which keyword sorts results from highest to lowest?
= DESC
? ORDER BY cpu DESC.

@@flashcards
ORDER BY col :: sort ascending (add DESC for descending)
ORDER BY a, b :: sort by a, then by b for ties
LIMIT 10 :: only the first ten rows
DISTINCT :: remove duplicate rows
AS :: give a column or expression a name
logical order :: FROM, WHERE, SELECT, ORDER BY, LIMIT

@@exercise Newest hires
Show the `name` and `hired` date of the **3 most recently hired** employees, newest first.
@@starter
-- your query here

@@expected
name        | hired
------------+-----------
Omar Haddad | 2026-09-01
Leo Martin  | 2024-04-01
Emi Suzuki  | 2023-10-01
@@hint
`ORDER BY hired DESC LIMIT 3`
@@solution
SELECT name, hired FROM employees ORDER BY hired DESC LIMIT 3;

@@exercise Monthly salary
For the **Support** department (`dept_id = 4`), show each `name` and their monthly salary as `monthly` (salary divided by 12, as a whole number), highest first.
@@starter
-- your query here

@@expected
name       | monthly
-----------+--------
Sara Ito   | 508333
Emi Suzuki | 333333
Leo Martin | 325000
@@hint
`SELECT name, salary / 12 AS monthly FROM employees WHERE dept_id = 4 ORDER BY monthly DESC;` (salary is an integer, so `/ 12` gives a whole number.)
@@solution
SELECT name, salary / 12 AS monthly FROM employees WHERE dept_id = 4 ORDER BY monthly DESC;

@@exercise Operating systems in use
List each different `os` in use on devices, without NULLs, alphabetically.
@@starter
-- your query here

@@expected
os
-------------------
Ubuntu 24.04
Windows 10
Windows 11
Windows Server 2022
iOS 18
macOS 15
@@hint
`SELECT DISTINCT os FROM devices WHERE os IS NOT NULL ORDER BY os;`
@@solution
SELECT DISTINCT os FROM devices WHERE os IS NOT NULL ORDER BY os;

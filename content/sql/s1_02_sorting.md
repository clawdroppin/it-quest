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

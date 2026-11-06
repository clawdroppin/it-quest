@@lesson s3-subqueries Subqueries and CTEs
@@text
## A query inside a query

```sql
SELECT name, salary
FROM employees
WHERE salary > (SELECT AVG(salary) FROM employees)
ORDER BY salary DESC;
```

The inner query runs first and gives one value. Subqueries can also give a **list**, for `IN`:

```sql
SELECT name FROM employees
WHERE id IN (SELECT requester_id FROM tickets WHERE category = 'security');
```

`EXISTS` asks "is there at least one matching row?", and is often clearer:

```sql
SELECT d.name FROM departments AS d
WHERE NOT EXISTS (SELECT 1 FROM employees AS e WHERE e.dept_id = d.id);
```

## CTEs: WITH

A **common table expression** names a subquery, so a complicated report becomes readable steps:

```sql
WITH failures AS (
    SELECT ip, COUNT(*) AS n
    FROM logins
    WHERE success = 0
    GROUP BY ip
),
compromised AS (
    SELECT DISTINCT l.ip, l.username
    FROM logins AS l
    JOIN failures AS f ON f.ip = l.ip
    WHERE l.success = 1 AND f.n >= 3
)
SELECT * FROM compromised;
```

*IPs with 3+ failures that then logged in successfully*: the same "probably guessed a password" rule as the Log Sentinel build, now in SQL. Security tools (SIEMs like Splunk and Microsoft Sentinel) use query languages built on these exact ideas.
## Correlated subqueries and EXISTS

A **correlated** subquery refers to the outer query's current row, so it runs once per row:

```sql,norun
SELECT e.name
FROM employees e
WHERE EXISTS (
    SELECT 1 FROM devices d
    WHERE d.employee_id = e.id AND d.os = 'Windows 10'
);
```

This lists employees who still have at least one Windows 10 device. `EXISTS` is true as soon as one matching row is found, so the database can stop looking. `NOT EXISTS` is the safe way to ask "has none": unlike `NOT IN`, it isn't confused by NULLs (if the subquery of a `NOT IN` returns a single NULL, the whole condition becomes unknown and no rows come back).

A subquery can also appear in the `SELECT` list, for example `(SELECT COUNT(*) FROM devices d WHERE d.employee_id = e.id) AS device_count`. It reads clearly, but on large tables a `LEFT JOIN` with `GROUP BY` is often faster. Choose the version that's easiest to read first, and change it only if it's measurably slow.

@@quiz
`WHERE salary > (SELECT AVG(salary) FROM employees)`: what does the subquery return?
* one value: the average salary
- every salary
- one row per employee
- nothing until the outer query runs
? A scalar subquery returns a single value that the outer query compares against.

@@quiz
Why use a CTE (`WITH ...`) instead of nesting subqueries?
* It names each step, so long queries are easier to read and reuse within the query
- It's required for GROUP BY
- It makes changes to the table
- It's always faster
? CTEs are about readability; performance is usually the same.

@@quiz
Why is NOT EXISTS usually safer than NOT IN for 'employees with no devices'?
* NOT IN returns no rows at all if the subquery contains a NULL
- NOT EXISTS is the only one SQLite supports
- NOT IN can't use subqueries
- NOT EXISTS sorts the results
? Comparisons with NULL are unknown, which poisons NOT IN.

@@flashcards
subquery :: a query inside another query
scalar subquery :: returns one value, e.g. (SELECT AVG(x) ...)
IN (subquery) :: matches any value the subquery returns
EXISTS :: true if the subquery returns any row
correlated subquery :: refers to the outer row
CTE :: WITH name AS (...) SELECT ... : a named step

@@exercise Above-average spenders
Show device `type`s whose **average price** is above the average price of **all** devices, with the type's average rounded to a whole number as `avg_price`. Most expensive first.
@@starter
-- your query here

@@expected
type    | avg_price
--------+----------
server  | 1485000
printer | 380000
@@hint
`GROUP BY type HAVING AVG(price) > (SELECT AVG(price) FROM devices)`, and `ROUND(AVG(price)) AS avg_price`. (ROUND gives a decimal like `1485000.0`; wrap it in `CAST(... AS INTEGER)` for a whole number.)
@@solution
SELECT type, CAST(ROUND(AVG(price)) AS INTEGER) AS avg_price
FROM devices
GROUP BY type
HAVING AVG(price) > (SELECT AVG(price) FROM devices)
ORDER BY avg_price DESC;

@@exercise Busiest support staff
Using a **CTE** named `workload` that counts tickets per `assignee_id` (ignore unassigned), show the assignee's `name` and `tickets` for everyone with **more tickets than the average** workload. Most tickets first.
@@starter
-- WITH workload AS (...) SELECT ...

@@expected
name    | tickets
--------+--------
Mia Lee | 7
@@requires
WITH
@@hint
`WITH workload AS (SELECT assignee_id, COUNT(*) AS tickets FROM tickets WHERE assignee_id IS NOT NULL GROUP BY assignee_id)`, then join it to employees and compare with `(SELECT AVG(tickets) FROM workload)`.
@@solution
WITH workload AS (
    SELECT assignee_id, COUNT(*) AS tickets
    FROM tickets
    WHERE assignee_id IS NOT NULL
    GROUP BY assignee_id
)
SELECT e.name, w.tickets
FROM workload AS w
JOIN employees AS e ON e.id = w.assignee_id
WHERE w.tickets > (SELECT AVG(tickets) FROM workload)
ORDER BY w.tickets DESC;

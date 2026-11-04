@@lesson s2-joins-deep Joins in Depth: Self, Outer, Semi and Anti Joins
@@concepts lists, maps
@@text
You know `JOIN ... ON` and `LEFT JOIN`. Real questions need a few more shapes.

## Self joins

A table can be joined to **itself**, with two aliases. Our `employees` table has `manager_id` pointing at another employee:

```sql
SELECT e.name AS employee, m.name AS manager
FROM employees e
LEFT JOIN employees m ON m.id = e.manager_id
ORDER BY e.id;
```

Self joins also compare rows with each other: "pairs of tickets from the same requester", "logins less than a minute apart from the same IP". Use `a.id < b.id` to avoid pairing a row with itself and getting each pair twice.

## ON vs WHERE in an outer join

This is a classic bug. Both queries look like "every employee with their open tickets":

```sql
-- A: the filter is in ON: every employee stays; only open tickets are joined
SELECT e.name, t.id FROM employees e
LEFT JOIN tickets t ON t.requester_id = e.id AND t.status = 'open'
ORDER BY e.id;

-- B: the filter is in WHERE: rows with no ticket (t.status NULL) are thrown away
SELECT e.name, t.id FROM employees e
LEFT JOIN tickets t ON t.requester_id = e.id
WHERE t.status = 'open'
ORDER BY e.id;
```

Query B silently became an inner join. Rule: in a LEFT JOIN, conditions on the **right** table go in `ON`; conditions on the **left** table go in `WHERE`.

## RIGHT and FULL OUTER JOIN

- `RIGHT JOIN` keeps all rows of the right table. It's just a LEFT JOIN written backwards; most people swap the tables and use LEFT.
- `FULL OUTER JOIN` keeps unmatched rows from **both** sides, with NULLs on the missing side. Good for comparing two lists ("in the HR export but not in Active Directory, and vice versa").

SQLite supports RIGHT and FULL joins from version 3.39. On older versions emulate FULL with `LEFT JOIN ... UNION ALL` the unmatched rows of the other side.

## Semi joins: EXISTS

"Employees who have opened at least one ticket": you want employees, not employee-ticket pairs. A JOIN would return Leo three times. `EXISTS` stops at the first match and returns each employee once:

```sql
SELECT name FROM employees e
WHERE EXISTS (SELECT 1 FROM tickets t WHERE t.requester_id = e.id)
ORDER BY name;
```

`IN (subquery)` does the same thing and is fine when the subquery's column has no NULLs.

## Anti joins: NOT EXISTS and LEFT JOIN ... IS NULL

"Rows in A with **no** match in B" has two common forms:

```sql
SELECT d.asset_tag FROM devices d
WHERE NOT EXISTS (SELECT 1 FROM employees e WHERE e.id = d.employee_id);

SELECT d.asset_tag FROM devices d
LEFT JOIN employees e ON e.id = d.employee_id
WHERE e.id IS NULL;
```

Both are correct (unlike `NOT IN` with NULLs). Modern optimisers usually run them the same way.

## CROSS JOIN

`CROSS JOIN` pairs every row of A with every row of B (a Cartesian product): 5 departments × 4 device types = 20 rows. Useful for building a "grid" you then fill in with a LEFT JOIN, so that empty combinations show up as 0 instead of disappearing:

```sql
SELECT dep.name, ty.type, COUNT(d.id) AS devices
FROM departments dep
CROSS JOIN (SELECT DISTINCT type FROM devices) ty
LEFT JOIN employees e ON e.dept_id = dep.id
LEFT JOIN devices d ON d.employee_id = e.id AND d.type = ty.type
GROUP BY dep.name, ty.type
ORDER BY dep.name, ty.type;
```

A *missing* join condition creates an accidental cross join, and row counts explode. If a report suddenly has thousands of rows, check your ON clauses.

## Joining on ranges

Joins don't have to use `=`. "Which ticket was open when this login happened" or "which price band does this device fall in" join on a range:

```sql,norun
SELECT d.asset_tag, b.band
FROM devices d JOIN price_bands b ON d.price >= b.low AND d.price < b.high;
```

## Join order and many-to-many

When you join three or more tables, each JOIN adds to the result of the previous ones. Watch for **fan-out**: joining employees to devices *and* to tickets multiplies rows (2 devices × 3 tickets = 6 rows per person), and SUM/COUNT then over-count. Aggregate each child table in a subquery or CTE first, then join the totals.

@@quiz
In `employees e LEFT JOIN tickets t ON t.requester_id = e.id WHERE t.status = 'open'`, what happens to employees with no tickets?
- They appear with NULLs
* They disappear: `t.status` is NULL for them, so the WHERE condition is never true
- They appear once with status 'open'
- The query fails
? Move the right-table condition into ON to keep them.

@@quiz
Which query shape returns each employee **once**, even if they opened many tickets?
- `JOIN tickets`
- `CROSS JOIN tickets`
* `WHERE EXISTS (SELECT 1 FROM tickets ...)`
- `LEFT JOIN tickets`
? A semi join tests for existence instead of producing pairs.

@@quiz
A report joins employees to their devices and to their tickets, then sums device prices. The sums are too high. Why?
* Fan-out: each device row is repeated once per ticket, so the sum counts it several times
- SUM ignores NULLs
- LEFT JOIN doubles rows
- Prices are stored as text
? Aggregate each child table separately, then join.

@@flashcards
self join :: a table joined to itself with two aliases
`a.id < b.id` :: each pair once, no self-pairs
right-table filter in a LEFT JOIN :: belongs in ON
`FULL OUTER JOIN` :: unmatched rows from both sides (SQLite 3.39+)
semi join :: EXISTS / IN: rows with a match, once
anti join :: NOT EXISTS / LEFT JOIN ... IS NULL
`CROSS JOIN` :: every combination
fan-out :: joining two child tables multiplies rows

@@exercise Who reports to whom
Show every employee's `employee` name, their `manager` name (or `-` when they have none), and how many `reports` (direct reports) they have themselves. Sort by employee id.
@@starter
SELECT e.name AS employee FROM employees e ORDER BY e.id;
@@expected
employee     | manager      | reports
-------------+--------------+--------
Kenji Tanaka | -            | 2
Ana Silva    | Kenji Tanaka | 0
Mia Lee      | Kenji Tanaka | 0
Hiro Sato    | -            | 2
Ben Jones    | Hiro Sato    | 0
Yuki Kato    | Hiro Sato    | 0
Rina Chen    | -            | 1
Taro Yamada  | Rina Chen    | 0
Sara Ito     | -            | 2
Leo Martin   | Sara Ito     | 0
Emi Suzuki   | Sara Ito     | 0
Omar Haddad  | -            | 0
@@requires
LEFT JOIN employees m
COALESCE
@@hint
Join the manager with `LEFT JOIN employees m ON m.id = e.manager_id`. Count reports with a correlated subquery: `(SELECT COUNT(*) FROM employees r WHERE r.manager_id = e.id)`.
@@solution
SELECT e.name AS employee,
       COALESCE(m.name, '-') AS manager,
       (SELECT COUNT(*) FROM employees r WHERE r.manager_id = e.id) AS reports
FROM employees e
LEFT JOIN employees m ON m.id = e.manager_id
ORDER BY e.id;

@@exercise Open tickets per person, everyone listed
Show every employee's `name` and the number of **open** tickets they requested (`open_tickets`), including people with 0. Sort by open_tickets descending, then name.
@@starter
SELECT e.name, COUNT(t.id) AS open_tickets
FROM employees e
LEFT JOIN tickets t ON t.requester_id = e.id
WHERE t.status = 'open'
GROUP BY e.id
ORDER BY open_tickets DESC, e.name;
@@expected
name         | open_tickets
-------------+-------------
Ana Silva    | 1
Hiro Sato    | 1
Leo Martin   | 1
Omar Haddad  | 1
Taro Yamada  | 1
Ben Jones    | 0
Emi Suzuki   | 0
Kenji Tanaka | 0
Mia Lee      | 0
Rina Chen    | 0
Sara Ito     | 0
Yuki Kato    | 0
@@requires
AND t.status = 'open'
@@hint
Move `t.status = 'open'` from WHERE into the ON clause.
@@solution
SELECT e.name, COUNT(t.id) AS open_tickets
FROM employees e
LEFT JOIN tickets t ON t.requester_id = e.id AND t.status = 'open'
GROUP BY e.id
ORDER BY open_tickets DESC, e.name;

@@exercise Brute-force bursts
Find pairs of **failed** logins from the **same IP** less than 10 seconds apart. Show `ip`, the `first` and `second` login ids, and the `gap` in seconds. Sort by first id.
@@starter
SELECT a.ip FROM logins a ORDER BY a.id;
@@expected
ip            | first | second | gap
--------------+-------+--------+----
203.0.113.7   | 3     | 4      | 4
203.0.113.7   | 3     | 5      | 9
203.0.113.7   | 4     | 5      | 5
203.0.113.7   | 5     | 6      | 6
203.0.113.7   | 6     | 7      | 7
198.51.100.23 | 12    | 13     | 3
@@requires
JOIN logins b
a.id < b.id
@@hint
Self join on `b.ip = a.ip AND a.id < b.id`, both with `success = 0`. Gap: `CAST((julianday(b.time) - julianday(a.time)) * 86400 AS INTEGER)`; keep pairs where the gap is between 0 and 9. To pair only *neighbours* you'd need more work: here all pairs within 10 seconds count.
@@solution
SELECT a.ip, a.id AS first, b.id AS second,
       CAST(ROUND((julianday(b.time) - julianday(a.time)) * 86400) AS INTEGER) AS gap
FROM logins a
JOIN logins b ON b.ip = a.ip AND a.id < b.id
WHERE a.success = 0 AND b.success = 0
  AND (julianday(b.time) - julianday(a.time)) * 86400 BETWEEN 0 AND 9.5
ORDER BY a.id, b.id;

@@exercise The full grid
For every department and every device type, show `department`, `type` and the number of `devices` assigned to people in that department (0 when there are none). Only include the types `laptop`, `desktop` and `phone`. Sort by department and type.
@@starter
SELECT dep.name AS department, d.type, COUNT(*) AS devices
FROM departments dep
JOIN employees e ON e.dept_id = dep.id
JOIN devices d ON d.employee_id = e.id
GROUP BY dep.name, d.type
ORDER BY 1, 2;
@@expected
department | type    | devices
-----------+---------+--------
Finance    | desktop | 0
Finance    | laptop  | 1
Finance    | phone   | 0
IT         | desktop | 0
IT         | laptop  | 3
IT         | phone   | 0
Marketing  | desktop | 0
Marketing  | laptop  | 0
Marketing  | phone   | 0
Sales      | desktop | 0
Sales      | laptop  | 1
Sales      | phone   | 2
Support    | desktop | 2
Support    | laptop  | 0
Support    | phone   | 0
@@requires
CROSS JOIN
LEFT JOIN
@@hint
Build the grid with `departments dep CROSS JOIN (SELECT 'laptop' AS type UNION ALL SELECT 'desktop' UNION ALL SELECT 'phone') ty`, then `LEFT JOIN employees e ON e.dept_id = dep.id LEFT JOIN devices d ON d.employee_id = e.id AND d.type = ty.type` and count `d.id`.
@@solution
SELECT dep.name AS department, ty.type, COUNT(d.id) AS devices
FROM departments dep
CROSS JOIN (SELECT 'laptop' AS type UNION ALL SELECT 'desktop' UNION ALL SELECT 'phone') ty
LEFT JOIN employees e ON e.dept_id = dep.id
LEFT JOIN devices d ON d.employee_id = e.id AND d.type = ty.type
GROUP BY dep.name, ty.type
ORDER BY dep.name, ty.type;

@@lesson s3-functions Text, Dates and CASE
@@concepts strings
@@text
## Text functions

```sql
SELECT UPPER(username) AS up, LENGTH(name) AS len,
       SUBSTR(asset_tag, 1, 2) AS prefix, REPLACE(model, ' ', '-') AS slug
FROM employees JOIN devices ON devices.employee_id = employees.id
LIMIT 3;
```

`TRIM`, `LOWER`, `INSTR(text, find)` (position), `||` to join. Names differ a little between databases (`LEN` in SQL Server, `SUBSTRING` elsewhere), but every one has these.

## Dates

SQLite stores dates as text and has functions to work with them:

```sql
SELECT date('2026-10-31', '+30 days') AS in_30_days,
       strftime('%Y', '2026-10-31') AS year,
       julianday('2026-10-31') - julianday('2026-10-01') AS days_between;
```

```sql
SELECT asset_tag, warranty_until,
       CAST(julianday(warranty_until) - julianday('2026-10-31') AS INTEGER) AS days_left
FROM devices
WHERE warranty_until IS NOT NULL
ORDER BY days_left
LIMIT 5;
```

(In other databases: `DATEADD` and `DATEDIFF` in SQL Server, `interval '30 days'` and `age()` in PostgreSQL, `DATE_ADD` in MySQL. The ideas are the same.) This course uses a fixed "today" of **2026-10-31** so results never change; in real queries you'd write `date('now')` or `CURRENT_DATE`.

## CASE: if/else inside a query

```sql
SELECT id, title,
       CASE priority
           WHEN 1 THEN 'Critical'
           WHEN 2 THEN 'High'
           WHEN 3 THEN 'Normal'
           ELSE 'Low'
       END AS priority_name
FROM tickets
WHERE status = 'open';
```

`CASE WHEN condition THEN ... END` works with any condition, and inside aggregates it makes **pivot** reports:

```sql
SELECT category,
       SUM(CASE WHEN status IN ('open', 'in-progress') THEN 1 ELSE 0 END) AS active,
       SUM(CASE WHEN status IN ('resolved', 'closed') THEN 1 ELSE 0 END) AS done
FROM tickets
GROUP BY category;
```

`COALESCE(a, b, ...)` returns the first value that isn't NULL: `COALESCE(assignee_id, 'unassigned')`.

@@quiz
`COALESCE(os, 'none')` returns...
* the os, or 'none' when os is NULL
- 'none' always
- NULL when os is 'none'
- the number of non-null os values
? COALESCE picks the first non-NULL argument.

@@quiz
Which builds a report column that says 'Expired' when the warranty date is before today?
* `CASE WHEN warranty_until < '2026-10-31' THEN 'Expired' ELSE 'OK' END`
- `IF(warranty_until < today, 'Expired')`
- `WHERE warranty_until = 'Expired'`
- `COALESCE(warranty_until, 'Expired')`
? CASE is SQL's if/else for values.

@@exercise Warranty report
For every device with a warranty date, show `asset_tag`, `warranty_until` and `warranty`: `expired` if the date is before `2026-10-31`, `expiring` if it's within the next **180 days** (up to and including `2027-04-29`), otherwise `ok`. Order by warranty date.
@@starter
-- today is '2026-10-31'

@@expected
asset_tag | warranty_until | warranty
----------+----------------+---------
LT-0052   | 2023-03-31     | expired
PR-0003   | 2024-02-28     | expired
PH-0101   | 2025-01-09     | expired
PH-0102   | 2025-01-09     | expired
DT-0007   | 2025-09-30     | expired
DT-0008   | 2025-09-30     | expired
LT-0042   | 2026-03-31     | expired
LT-0043   | 2026-03-31     | expired
LT-0050   | 2027-05-19     | ok
LT-0051   | 2028-01-14     | ok
SV-0001   | 2028-05-31     | ok
SV-0002   | 2028-05-31     | ok
LT-0060   | 2029-08-31     | ok
LT-0061   | 2029-08-31     | ok
@@hint
`CASE WHEN warranty_until < '2026-10-31' THEN 'expired' WHEN warranty_until <= date('2026-10-31', '+180 days') THEN 'expiring' ELSE 'ok' END AS warranty`
@@solution
SELECT asset_tag, warranty_until,
       CASE
           WHEN warranty_until < '2026-10-31' THEN 'expired'
           WHEN warranty_until <= date('2026-10-31', '+180 days') THEN 'expiring'
           ELSE 'ok'
       END AS warranty
FROM devices
WHERE warranty_until IS NOT NULL
ORDER BY warranty_until;

@@exercise Time to resolve
For **closed or resolved** tickets, show the `category` and the average time to close in **hours** as `avg_hours`, rounded to 1 decimal, slowest first.
@@starter
-- julianday gives days as a decimal: multiply by 24 for hours

@@expected
category | avg_hours
---------+----------
hardware | 40.4
software | 27.3
network  | 20.5
security | 2.5
account  | 0.2
@@hint
`ROUND(AVG((julianday(closed) - julianday(opened)) * 24), 1) AS avg_hours`, `WHERE closed IS NOT NULL`, `GROUP BY category`.
@@solution
SELECT category, ROUND(AVG((julianday(closed) - julianday(opened)) * 24), 1) AS avg_hours
FROM tickets
WHERE closed IS NOT NULL
GROUP BY category
ORDER BY avg_hours DESC;

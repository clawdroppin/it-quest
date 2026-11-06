@@lesson s1-aggregates Counting and Grouping
@@concepts maps
@@text
Reports are mostly **aggregates**: how many, how much, on average.

```sql
SELECT COUNT(*) AS employees, SUM(salary) AS payroll, AVG(salary) AS average,
       MIN(hired) AS first_hire, MAX(hired) AS last_hire
FROM employees;
```

`COUNT(*)` counts rows; `COUNT(column)` counts rows where that column **isn't NULL**; `COUNT(DISTINCT column)` counts different values.

## GROUP BY

`GROUP BY` makes one result row per group:

```sql
SELECT status, COUNT(*) AS tickets
FROM tickets
GROUP BY status
ORDER BY tickets DESC;
```

```sql
SELECT category, COUNT(*) AS total, MIN(priority) AS most_urgent
FROM tickets
GROUP BY category;
```

Rule: every column in the SELECT is either in the `GROUP BY` or inside an aggregate function. (SQLite is lenient here; PostgreSQL and SQL Server will give an error, so follow the rule.)

## HAVING: filtering groups

`WHERE` filters **rows** before grouping; `HAVING` filters **groups** after:

```sql
SELECT ip, COUNT(*) AS failures
FROM logins
WHERE success = 0
GROUP BY ip
HAVING COUNT(*) >= 3
ORDER BY failures DESC;
```

That's a brute-force detector in six lines. The order of the clauses is always: `SELECT … FROM … WHERE … GROUP BY … HAVING … ORDER BY … LIMIT`.
## Grouping by more than one column

`GROUP BY` can take several columns, giving one row per **combination**:

```sql,norun
SELECT department, os, COUNT(*) AS devices
FROM devices
GROUP BY department, os
ORDER BY department, devices DESC;
```

Every column in the `SELECT` must either be in the `GROUP BY` or be inside an aggregate function (`COUNT`, `SUM`, `AVG`, `MIN`, `MAX`). Otherwise the database can't know which of the group's values to show. SQLite and older MySQL quietly pick one at random, which hides bugs; PostgreSQL and SQL Server refuse the query.

Two more aggregate tricks are worth knowing: `COUNT(DISTINCT os)` counts the different values, and a conditional count, `SUM(CASE WHEN os = 'Linux' THEN 1 ELSE 0 END)`, counts only the matching rows within each group, so one query can produce a whole table of figures.

@@quiz
What's the difference between `WHERE` and `HAVING`?
* WHERE filters rows before grouping; HAVING filters groups after
- They're the same
- HAVING is for text, WHERE for numbers
- WHERE only works with GROUP BY
? You can't use an aggregate like COUNT(*) in WHERE; that's HAVING's job.

@@quiz
`devices` has 14 rows, 4 of them with `employee_id` NULL. What is `SELECT COUNT(employee_id) FROM devices;`?
- 14
* 10
- 4
- NULL
? COUNT(column) skips NULLs. COUNT(*) would be 14.

@@quiz
Which query counts the different operating systems in use?
* SELECT COUNT(DISTINCT os) FROM devices;
- SELECT DISTINCT COUNT(os) FROM devices;
- SELECT COUNT(os) FROM devices GROUP BY os;
- SELECT SUM(os) FROM devices;
? COUNT(DISTINCT col) counts unique non-NULL values.

@@drill
Which clause filters groups after GROUP BY?
= HAVING
? WHERE filters rows before grouping.

@@flashcards
COUNT(*) :: Number of rows
COUNT(col) :: Number of non-NULL values in col
SUM / AVG / MIN / MAX :: Total / average / smallest / largest
GROUP BY :: One result row per distinct value (or combination)
HAVING :: Filter groups, using aggregates
Clause order :: SELECT, FROM, WHERE, GROUP BY, HAVING, ORDER BY, LIMIT

@@exercise Devices per type
Show each device `type` and how many devices of that type there are (as `count`), most first; ties alphabetically by type.
@@starter
-- your query here

@@expected
type    | count
--------+------
laptop  | 7
desktop | 2
phone   | 2
server  | 2
printer | 1
@@hint
`GROUP BY type ORDER BY count DESC, type`
@@solution
SELECT type, COUNT(*) AS count FROM devices GROUP BY type ORDER BY count DESC, type;

@@exercise Hardware spending
Show the total `price` of devices per `type` as `total_yen`, but only types where the total is **over 500000** yen, biggest first.
@@starter
-- your query here

@@expected
type   | total_yen
-------+----------
server | 2970000
laptop | 1433000
@@hint
`SUM(price) AS total_yen ... GROUP BY type HAVING SUM(price) > 500000 ORDER BY total_yen DESC`
@@solution
SELECT type, SUM(price) AS total_yen FROM devices GROUP BY type HAVING SUM(price) > 500000 ORDER BY total_yen DESC;

@@exercise Suspicious accounts
Show each `username` with at least **2 failed** logins, with the number of failures as `failures` and the number of **different IPs** they came from as `ips`. Most failures first, ties by username.
@@starter
-- your query here

@@expected
username | failures | ips
---------+----------+----
admin    | 2        | 1
asilva   | 2        | 1
test     | 2        | 1
@@hint
`WHERE success = 0 GROUP BY username HAVING COUNT(*) >= 2`, and `COUNT(DISTINCT ip) AS ips`.
@@solution
SELECT username, COUNT(*) AS failures, COUNT(DISTINCT ip) AS ips
FROM logins
WHERE success = 0
GROUP BY username
HAVING COUNT(*) >= 2
ORDER BY failures DESC, username;

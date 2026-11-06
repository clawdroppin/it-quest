@@lesson n8-tuning Query Tuning: Sargable Queries, Composite Indexes and Index Hygiene
@@concepts lists
@@text
Module 6 introduced indexes and `EXPLAIN QUERY PLAN`. This lesson is about the habits that keep a production database fast as it grows from thousands to hundreds of millions of rows.

## How to tune, in order

1. **Find the slow queries**: PostgreSQL's `pg_stat_statements`, SQL Server's Query Store, MySQL's slow query log. Tune the query with the largest *total* time (calls × average), not the scariest-looking one.
2. **Read the plan**: `EXPLAIN` (estimate) and `EXPLAIN ANALYZE` (PostgreSQL: actually runs it and shows real times and row counts). Look for full scans of big tables, nested loops over many rows, sorts that spill to disk, and estimates far from actual counts.
3. **Fix the query or the index**, measure again.
4. Only then think about hardware, caching or schema changes.

## Sargable predicates

A condition is **sargable** (Search ARGument ABLE) when the database can use an index to find matching rows. Wrapping the **column** in a function or calculation usually breaks that:

| Not sargable | Sargable rewrite |
|---|---|
| `WHERE date(opened) = '2026-10-30'` | `WHERE opened >= '2026-10-30' AND opened < '2026-10-31'` |
| `WHERE salary * 12 > 60000000` | `WHERE salary > 5000000` |
| `WHERE substr(asset_tag, 1, 2) = 'LT'` | `WHERE asset_tag LIKE 'LT%'` (with a suitable index/collation) or `>= 'LT' AND < 'LU'` |
| `WHERE lower(username) = 'asilva'` | an index on `lower(username)`, or a NOCASE column |
| `WHERE CAST(id AS TEXT) = '42'` | `WHERE id = 42` |

Rule of thumb: keep the column bare on one side; do the arithmetic on the constant side.

## Composite indexes

An index on `(status, priority, opened)` is sorted by status, then priority within each status, then opened. It helps:

- `WHERE status = ?`
- `WHERE status = ? AND priority = ?`
- `WHERE status = ? AND priority = ? AND opened > ?`
- `WHERE status = ? ORDER BY priority` (no separate sort needed)

It does **not** help `WHERE priority = ?` alone: the leading column isn't constrained (like a phone book sorted by surname can't find everyone named "Mia"). Order columns: **equality** conditions first, then the **range** or ORDER BY column. After a range condition, later columns can't be used for seeking.

## Covering indexes

If an index contains every column a query needs, the database answers from the index alone and never touches the table (an "index-only scan", "COVERING INDEX" in SQLite's plan). PostgreSQL and SQL Server let you add non-key columns with `INCLUDE (...)`.

## Index hygiene

Indexes aren't free: every INSERT, UPDATE and DELETE must update each one, and they take disk and memory. Regularly look for:

- **Redundant indexes**: an index on `(a)` is redundant if there's one on `(a, b)`, because the longer one serves every query the shorter does.
- **Unused indexes**: PostgreSQL's `pg_stat_user_indexes.idx_scan = 0` after weeks of traffic.
- **Missing foreign key indexes**: deleting a parent row must search the child table; without an index on the FK column, that's a full scan, every time.

In SQLite, `sqlite_master` lists indexes, and the table-valued functions `pragma_index_list('t')` and `pragma_index_info('idx')` describe them:

```sql
CREATE INDEX tickets_status ON tickets(status);
CREATE INDEX tickets_status_priority ON tickets(status, priority);
SELECT il.name AS index_name, ii.seqno, ii.name AS column_name
FROM pragma_index_list('tickets') il, pragma_index_info(il.name) ii
ORDER BY il.name, ii.seqno;
```

## Statistics

The optimiser chooses plans from **statistics**: how many rows, how many distinct values. Stale statistics give bad plans. PostgreSQL's autovacuum runs `ANALYZE` automatically; after a big load, run it yourself. SQLite: `ANALYZE` (and `PRAGMA optimize` before closing long-lived connections).

## Other common wins

- **N+1 queries**: an app that runs 1 query for a list, then 1 per row. Replace with one JOIN or an `IN (...)` query.
- **SELECT \***: fetches columns you don't need and prevents covering indexes.
- **OFFSET pagination** on deep pages: use keyset pagination (module 6).
- **OR across columns**: `WHERE a = ? OR b = ?` often can't use one index; `UNION` of two indexed queries can.
- **Implicit conversions**: comparing a TEXT column with a number (or the reverse in SQL Server with NVARCHAR vs VARCHAR) may disable the index.
- **Batching**: one INSERT with 1,000 rows (or 1,000 inserts in one transaction) instead of 1,000 transactions.

@@quiz
Why is `WHERE date(opened) = '2026-10-30'` slow on a big table even with an index on `opened`?
- date() is a slow function
* Wrapping the column in a function prevents the index from being used to seek; rewrite it as a range on the bare column
- Dates can't be indexed
- It needs ORDER BY
? Keep the column bare: `opened >= '2026-10-30' AND opened < '2026-10-31'`.

@@quiz
An index exists on `(status, priority)`. Which query can't use it to seek?
- `WHERE status = 'open'`
- `WHERE status = 'open' AND priority = 1`
* `WHERE priority = 1`
- `WHERE status = 'open' ORDER BY priority`
? The leading column must be constrained.

@@quiz
Which index is redundant?
* `(a)` when `(a, b)` exists
- `(a, b)` when `(a)` exists
- `(b)` when `(a, b)` exists
- None; indexes are never redundant
? The longer index serves every query the shorter one does (on the leading column).

@@quiz
What makes an index "covering" for a query?
- It's unique
- It's on the primary key
* It contains every column the query reads, so the table itself isn't touched
- It covers all rows
? PostgreSQL and SQL Server can add extra columns with INCLUDE.

@@drill
Which SQLite command shows how a query will use indexes?
= EXPLAIN QUERY PLAN
? SCAN means a full table scan; SEARCH uses an index.

@@flashcards
sargable :: a condition an index can seek on
bare column rule :: put calculations on the constant side
composite index order :: equality columns first, then range / ORDER BY
leftmost prefix :: a composite index helps only when its first column is used
covering index :: answers a query without touching the table
`INCLUDE (...)` :: extra non-key columns (PostgreSQL, SQL Server)
redundant index :: (a) when (a, b) exists
FK index :: index child foreign key columns
`ANALYZE` :: refresh optimiser statistics
N+1 :: one query per row: replace with a join
`EXPLAIN ANALYZE` :: real timings (PostgreSQL)

@@exercise Make it sargable
Rewrite the starter's three conditions so an index on the bare column could be used, with the **same results**: tickets opened on 30 October (by UTC date), employees whose monthly salary (salary / 12) is above 500,000, and devices whose asset tag starts with `LT`. Keep the output the same: one result per query.
@@starter
SELECT id, title FROM tickets WHERE date(opened) = '2026-10-30' ORDER BY id;
SELECT name FROM employees WHERE salary / 12 > 500000 ORDER BY name;
SELECT asset_tag FROM devices WHERE substr(asset_tag, 1, 2) = 'LT' ORDER BY asset_tag;
@@expected
id | title
---+----------------------------
12 | Install accounting software
13 | Monitor flickering

name
------------
Ana Silva
Hiro Sato
Kenji Tanaka
Rina Chen
Sara Ito

asset_tag
---------
LT-0042
LT-0043
LT-0050
LT-0051
LT-0052
LT-0060
LT-0061
@@requires
opened >= '2026-10-30'
WHERE salary >
asset_tag >= 'LT'
@@hint
`opened >= '2026-10-30' AND opened < '2026-10-31'`. For the salary, move the arithmetic to the constant side: `salary / 12 > 500000` becomes `salary > 6000000`. (Strictly, integer division makes the exact rewrite `salary >= 6000012`; both give the same rows here.) Tags: `asset_tag >= 'LT' AND asset_tag < 'LU'`.
@@solution
SELECT id, title FROM tickets WHERE opened >= '2026-10-30' AND opened < '2026-10-31' ORDER BY id;
SELECT name FROM employees WHERE salary > 6000000 ORDER BY name;
SELECT asset_tag FROM devices WHERE asset_tag >= 'LT' AND asset_tag < 'LU' ORDER BY asset_tag;

@@exercise Find redundant indexes
Someone added indexes in a hurry. List every index whose column list is a **leading prefix** of another index on the same table (so it's redundant). Show `table_name`, `redundant` and `covered_by`. Use `sqlite_master`, `pragma_index_list` and `pragma_index_info`; build each index's column list with `group_concat` (columns in seqno order). Sort by redundant.
@@setup
CREATE TABLE tickets (id INTEGER PRIMARY KEY, status TEXT, priority INTEGER, opened TEXT, assignee_id INTEGER);
CREATE INDEX t_status ON tickets(status);
CREATE INDEX t_status_priority ON tickets(status, priority);
CREATE INDEX t_status_priority_opened ON tickets(status, priority, opened);
CREATE INDEX t_priority ON tickets(priority);
CREATE INDEX t_assignee ON tickets(assignee_id);
CREATE TABLE logins (id INTEGER PRIMARY KEY, username TEXT, time TEXT);
CREATE INDEX l_user_time ON logins(username, time);
CREATE INDEX l_user ON logins(username);
@@starter
SELECT name, tbl_name FROM sqlite_master WHERE type = 'index' ORDER BY name;
@@expected
table_name | redundant         | covered_by
-----------+-------------------+-------------------------
logins     | l_user            | l_user_time
tickets    | t_status          | t_status_priority_opened
tickets    | t_status_priority | t_status_priority_opened
@@requires
pragma_index_info
group_concat
@@hint
CTE `cols`: `SELECT m.tbl_name, m.name, (SELECT group_concat(name, ',') FROM (SELECT name FROM pragma_index_info(m.name) ORDER BY seqno)) AS cols FROM sqlite_master m WHERE m.type = 'index'`. Self join it: `a.tbl_name = b.tbl_name AND a.name <> b.name AND (b.cols || ',') LIKE (a.cols || ',%')` (the trailing comma stops `status` from matching `status_code`). When several indexes cover one, report the longest: number them with `ROW_NUMBER() OVER (PARTITION BY a.name ORDER BY length(b.cols) DESC)` and keep 1.
@@solution
WITH cols AS (
    SELECT m.tbl_name, m.name,
           (SELECT group_concat(name, ',') FROM (SELECT name FROM pragma_index_info(m.name) ORDER BY seqno)) AS cols
    FROM sqlite_master m WHERE m.type = 'index'
),
pairs AS (
    SELECT a.tbl_name, a.name AS redundant, b.name AS covered_by,
           ROW_NUMBER() OVER (PARTITION BY a.name ORDER BY length(b.cols) DESC) AS rn
    FROM cols a JOIN cols b
      ON a.tbl_name = b.tbl_name AND a.name <> b.name AND (b.cols || ',') LIKE (a.cols || ',%')
)
SELECT tbl_name AS table_name, redundant, covered_by
FROM pairs WHERE rn = 1
ORDER BY redundant;

@@exercise Fix an N+1 report
A dashboard ran one query to list departments, then one query **per department** for its headcount, average salary and newest hire (the starter shows two of those per-department queries). Replace them all with **one** query returning `department`, `headcount`, `avg_salary` (whole yen) and `newest_hire` (name of the most recently hired person) for every department, including empty ones (headcount 0, NULLs). Sort by department.
@@starter
SELECT name FROM departments ORDER BY name;
SELECT COUNT(*), AVG(salary) FROM employees WHERE dept_id = 1;
SELECT COUNT(*), AVG(salary) FROM employees WHERE dept_id = 2;
@@expected
department | headcount | avg_salary | newest_hire
-----------+-----------+------------+------------
Finance    | 2         | 7250000    | Taro Yamada
IT         | 3         | 6566666    | Mia Lee
Marketing  | 0         | NULL       | NULL
Sales      | 3         | 6533333    | Yuki Kato
Support    | 3         | 4666666    | Leo Martin
@@requires
LEFT JOIN
GROUP BY
@@hint
`departments d LEFT JOIN employees e ON e.dept_id = d.id GROUP BY d.id`. `CAST(AVG(e.salary) AS INTEGER)`. Newest hire: a correlated subquery `(SELECT name FROM employees x WHERE x.dept_id = d.id ORDER BY hired DESC LIMIT 1)`, or a ROW_NUMBER CTE.
@@solution
SELECT d.name AS department,
       COUNT(e.id) AS headcount,
       CAST(AVG(e.salary) AS INTEGER) AS avg_salary,
       (SELECT x.name FROM employees x WHERE x.dept_id = d.id ORDER BY x.hired DESC LIMIT 1) AS newest_hire
FROM departments d
LEFT JOIN employees e ON e.dept_id = d.id
GROUP BY d.id
ORDER BY d.name;

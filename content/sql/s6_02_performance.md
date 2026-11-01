@@lesson s6-performance Fast Queries: Indexes, Query Plans and Pagination
@@concepts lists, maps
@@text
A query that takes 2 ms with 1,000 test rows can take 20 seconds with 10 million real ones. The difference between the two is almost always an **index**. This lesson shows how databases find rows, how to see what they're doing, and the common patterns that make application queries slow.

## What an index is

Without an index, `WHERE email = 'aiko@example.jp'` means reading **every row** (a *full table scan*). An index is a sorted copy of one or more columns with pointers to the rows (a B-tree), so the database can jump straight to the match, like the index at the back of a book.

```sql
CREATE TABLE login_events (id INTEGER PRIMARY KEY, username TEXT, ip TEXT, ok INTEGER, at TEXT);
CREATE INDEX idx_login_events_username_at ON login_events (username, at);

-- Ask the database how it would run a query:
EXPLAIN QUERY PLAN
SELECT at, ip FROM login_events WHERE username = 'aiko' AND at >= '2026-10-01' ORDER BY at;
```

The plan says **SEARCH ... USING INDEX** (good: it jumps to the rows) or **SCAN** (it reads the whole table). Every database has this: `EXPLAIN` in PostgreSQL and MySQL (and `EXPLAIN ANALYZE` to actually run it and time each step), and "Display Estimated Execution Plan" in SQL Server Management Studio.

## Which columns to index

- **Primary keys** are indexed automatically. **UNIQUE** constraints create an index too.
- **Foreign key columns**: `orders.customer_id` is used in every join and every "orders of this customer" query, and (unlike in MySQL) SQLite and PostgreSQL don't index it for you.
- Columns in frequent `WHERE`, `JOIN` and `ORDER BY` clauses.
- **Composite indexes** work left to right, like a phone book sorted by last name then first name: an index on `(username, at)` helps `WHERE username = ?` and `WHERE username = ? AND at > ?`, but not `WHERE at > ?` alone. Put the column you test with `=` first, and the range or sort column after it.
- A **covering index** contains every column the query needs, so the table isn't read at all.
- A **partial index** covers only some rows: `CREATE INDEX idx_open ON tickets (priority) WHERE status = 'open'` stays small when 99% of tickets are closed.

Indexes aren't free: each one makes every `INSERT`, `UPDATE` and `DELETE` slower and uses disk space. Index for the queries you actually run.

## Patterns that defeat indexes

| Slow | Why | Fast |
|---|---|---|
| `WHERE strftime('%Y', at) = '2026'` | the function hides the column from the index | `WHERE at >= '2026-01-01' AND at < '2027-01-01'` |
| `WHERE lower(email) = ?` | same | store emails lower-case, use `COLLATE NOCASE`, or an index on the expression `lower(email)` |
| `WHERE name LIKE '%printer%'` | a leading `%` can't use a B-tree | full-text search (SQLite FTS5, PostgreSQL `tsvector`) |
| `WHERE a = ? OR b = ?` | often a scan | two queries with `UNION`, or an index on each |
| `SELECT *` | reads columns you don't use; prevents covering indexes | list the columns |

## The N+1 query problem

The most common performance bug in applications is not in SQL at all:

```python,norun
# 1 query for the orders + 1 query per order = 101 round trips for 100 orders
for order in db.execute("SELECT id, customer_id FROM orders LIMIT 100"):
    customer = db.execute("SELECT name FROM customers WHERE id = ?", (order.customer_id,)).fetchone()
```

Each query is fast, but 101 round trips to a database server add up to seconds. One `JOIN` does the same work in one trip. ORMs (Entity Framework, SQLAlchemy, Hibernate) make this mistake easy, through "lazy loading"; they all have a way to load related rows up front (`Include`, `joinedload`, `JOIN FETCH`).

## Pagination: OFFSET gets slower, keysets don't

```sql,norun
-- Page 5,000: the database still reads and throws away 99,980 rows first.
SELECT id, title FROM tickets ORDER BY id LIMIT 20 OFFSET 99980;

-- Keyset ("seek") pagination: remember where the last page ended, and continue from there.
SELECT id, title FROM tickets WHERE id > :last_seen_id ORDER BY id LIMIT 20;
```

Keyset pagination uses the index to jump straight to the page, so page 5,000 is as fast as page 1, and rows inserted meanwhile don't shift everything by one. To sort by something that isn't unique (like a date), add the id as a tie-breaker: `WHERE (created_at, id) < (:last_created, :last_id) ORDER BY created_at DESC, id DESC`.

## Keep the statistics fresh

Databases choose plans using statistics about the data. After big imports run `ANALYZE` (SQLite, PostgreSQL) or update statistics (SQL Server), and let autovacuum (PostgreSQL) or `VACUUM` reclaim space.

@@quiz
A table has an index on `(customer_id, created_at)`. Which query can't use it efficiently?
- `WHERE customer_id = 7`
- `WHERE customer_id = 7 AND created_at >= '2026-10-01'`
* `WHERE created_at >= '2026-10-01'`
- `WHERE customer_id = 7 ORDER BY created_at`
? A composite index is sorted by its first column first. Without that column, it's like searching a phone book by first name.

@@quiz
Why is `WHERE strftime('%Y', created_at) = '2026'` slow on a big indexed table?
* Wrapping the column in a function means the index on `created_at` can't be used, so every row is checked
- strftime is a slow function
- Dates can't be indexed
- It returns text instead of a number
? Rewrite it as a range on the bare column: `created_at >= '2026-01-01' AND created_at < '2027-01-01'`.

@@quiz
A page lists 50 orders with their customer's name, and runs 51 queries. What is this called, and the fix?
- A deadlock; add a transaction
* The N+1 problem; load the orders with a JOIN (or one query for all the customers)
- A full table scan; add an index
- Pagination; use OFFSET
? Round trips, not query complexity, are the cost. Fetch related data in one go.

@@quiz
Why is keyset pagination (`WHERE id > ?`) faster than `OFFSET` for deep pages?
- OFFSET isn't supported by indexes at all
* OFFSET still reads all the skipped rows; a keyset jumps straight to the start of the page with the index
- Keysets cache the results
- LIMIT is ignored with OFFSET
? Page 10,000 with OFFSET reads 200,000 rows to show 20.

@@flashcards
index :: a sorted structure (B-tree) for finding rows without reading the whole table
full table scan :: reading every row (SCAN in the query plan)
`EXPLAIN QUERY PLAN` / `EXPLAIN ANALYZE` :: show how a query will run / run and time it
composite index :: on several columns; used left to right
covering index :: holds every column a query needs
partial index :: `CREATE INDEX ... WHERE condition`: only some rows
sargable :: a condition that can use an index (no function around the column)
N+1 problem :: one query per row instead of one JOIN
keyset pagination :: `WHERE id > :last ORDER BY id LIMIT n`
`ANALYZE` :: refresh the statistics the planner uses
indexes cost :: slower writes and more disk space

@@exercise Keyset pagination
The app shows the event log **newest first**, 4 per page. The last row on the page the user is looking at had `at = '2026-10-31 09:00'` and `id = 13`. Several events share the same timestamp, so `id` (also descending) breaks ties. Write the query for the **next page**, using a keyset, not OFFSET: `id`, `at`, `message`.
@@setup
CREATE TABLE events (id INTEGER PRIMARY KEY, at TEXT NOT NULL, message TEXT NOT NULL);
CREATE INDEX idx_events_at_id ON events (at, id);
INSERT INTO events (id, at, message) VALUES
 (1, '2026-10-30 08:00', 'backup started'), (2, '2026-10-30 08:00', 'vpn up'),
 (3, '2026-10-30 09:15', 'backup finished'), (4, '2026-10-30 12:00', 'disk 81%'),
 (5, '2026-10-30 23:59', 'cert renewed'), (6, '2026-10-31 07:30', 'login failed: ken'),
 (7, '2026-10-31 07:30', 'login ok: ken'), (8, '2026-10-31 08:45', 'printer offline'),
 (9, '2026-10-31 09:00', 'ticket 5120 opened'), (10, '2026-10-31 09:00', 'ticket 5121 opened'),
 (11, '2026-10-31 09:00', 'ticket 5122 opened'), (12, '2026-10-31 09:00', 'ticket 5123 opened'),
 (13, '2026-10-31 09:00', 'ticket 5124 opened'), (14, '2026-10-31 10:10', 'printer online'),
 (15, '2026-10-31 11:00', 'disk 92%'), (16, '2026-11-01 06:00', 'patch night done');
@@starter
-- the next page after (at = '2026-10-31 09:00', id = 13), newest first, 4 rows

@@expected
id | at               | message
---+------------------+-------------------
12 | 2026-10-31 09:00 | ticket 5123 opened
11 | 2026-10-31 09:00 | ticket 5122 opened
10 | 2026-10-31 09:00 | ticket 5121 opened
9  | 2026-10-31 09:00 | ticket 5120 opened
@@requires
LIMIT 4
@@hint
`WHERE at < '2026-10-31 09:00' OR (at = '2026-10-31 09:00' AND id < 13)` (or the row-value form `WHERE (at, id) < ('2026-10-31 09:00', 13)`), `ORDER BY at DESC, id DESC LIMIT 4`.
@@solution
SELECT id, at, message
FROM events
WHERE (at, id) < ('2026-10-31 09:00', 13)
ORDER BY at DESC, id DESC
LIMIT 4;

@@exercise Make it index-friendly
This report is slow on the real table because it wraps `at` in a function. Rewrite the `WHERE` so it's a plain **range on `at`** (October 2026) that the index can use. Keep the same result: the number of failed logins per user in October 2026, most failures first, then by username.
@@setup
CREATE TABLE logins (id INTEGER PRIMARY KEY, username TEXT NOT NULL, ok INTEGER NOT NULL, at TEXT NOT NULL);
CREATE INDEX idx_logins_at ON logins (at);
INSERT INTO logins (username, ok, at) VALUES
 ('aiko', 0, '2026-09-30 23:59:59'), ('aiko', 0, '2026-10-01 00:00:00'), ('ken', 0, '2026-10-03 08:12:00'),
 ('ken', 0, '2026-10-03 08:12:30'), ('ken', 1, '2026-10-03 08:13:00'), ('yuki', 0, '2026-10-15 14:00:00'),
 ('ken', 0, '2026-10-31 23:59:59'), ('aiko', 1, '2026-10-20 09:00:00'), ('yuki', 0, '2026-11-01 00:00:00'),
 ('mei', 0, '2026-10-09 10:00:00'), ('mei', 0, '2026-10-09 10:00:05');
@@starter
SELECT username, COUNT(*) AS failures
FROM logins
WHERE ok = 0 AND strftime('%Y-%m', at) = '2026-10'
GROUP BY username
ORDER BY failures DESC, username;
@@expected
username | failures
---------+---------
ken      | 3
mei      | 2
aiko     | 1
yuki     | 1
@@requires
at >= '2026-10-01'
at < '2026-11-01'
@@hint
`WHERE ok = 0 AND at >= '2026-10-01' AND at < '2026-11-01'`. ISO dates compare correctly as text, and `< '2026-11-01'` includes every time on 31 October.
@@solution
SELECT username, COUNT(*) AS failures
FROM logins
WHERE ok = 0 AND at >= '2026-10-01' AND at < '2026-11-01'
GROUP BY username
ORDER BY failures DESC, username;

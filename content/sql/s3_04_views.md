@@lesson s3-views Views, Temporary Tables and Generated Columns
@@concepts functions, modules
@@text
As your queries grow, you'll want to **name** useful results and reuse them. SQL has several ways, each with a different lifetime.

| Tool | What it stores | Lives for |
|---|---|---|
| CTE (`WITH`) | nothing; part of one query | one statement |
| View | the **query**, not the data | until dropped |
| Temporary table | the **data**, a snapshot | the connection / session |
| Materialised view (PostgreSQL) | the data, refreshed on demand | until dropped |
| Generated column | a value computed from other columns | as long as the table |

## Views

A view is a saved SELECT you can query like a table:

```sql
CREATE VIEW open_tickets AS
SELECT t.id, t.title, t.priority, e.name AS requester
FROM tickets t JOIN employees e ON e.id = t.requester_id
WHERE t.status IN ('open', 'in-progress');

SELECT * FROM open_tickets WHERE priority <= 2 ORDER BY id;
```

Every time you query the view, the database runs its query again, so the view always shows **current** data. The optimiser usually merges your outer WHERE into the view's query, so a view costs nothing extra.

Why use views:

- **Reuse**: the "active employees" or "open tickets" definition lives in one place. Change the rule once, every report follows.
- **Simplicity**: give report writers and BI tools a clean, joined, renamed shape instead of the raw tables.
- **Security**: grant users `SELECT` on a view that hides salary columns or other departments' rows, and not on the base table (PostgreSQL, SQL Server; SQLite has no users).
- **Stability**: when you refactor tables, keep the old view so old reports keep working.

Change a view with `DROP VIEW IF EXISTS v; CREATE VIEW v AS ...` (SQLite) or `CREATE OR REPLACE VIEW` (PostgreSQL, MySQL).

### Updating through a view

Simple views (one table, no aggregates) are **updatable** in PostgreSQL and MySQL: `UPDATE open_tickets SET priority = 1 WHERE id = 10` changes the base table. SQLite views are read-only, unless you add an `INSTEAD OF` trigger that says what to do (you'll see triggers in module 7). PostgreSQL adds `WITH CHECK OPTION` to refuse updates that would make a row fall out of the view.

## Temporary tables

A temp table is a real table that disappears when your connection closes, and other connections can't see it. Use one to **snapshot** a result you'll use several times in a script, or to break a monster query into steps you can check:

```sql
CREATE TEMP TABLE overdue AS
SELECT id, title FROM tickets WHERE status = 'open' AND opened < '2026-10-30';

SELECT COUNT(*) AS overdue FROM overdue;
```

Unlike a view, a temp table doesn't change when the base tables change.

## Materialised views

A materialised view (PostgreSQL, Oracle; "indexed views" in SQL Server) stores the query's **result** like a table, so expensive reports read instantly. You refresh it when you choose:

```sql,norun
CREATE MATERIALIZED VIEW monthly_sales AS SELECT ...;
REFRESH MATERIALIZED VIEW CONCURRENTLY monthly_sales;
```

The trade-off is staleness: between refreshes, the data is old. SQLite doesn't have them; a table rebuilt by a scheduled job does the same thing.

## Generated columns

A generated column's value is computed from other columns of the same row (SQLite 3.31+, PostgreSQL 12+, MySQL 5.7+):

```sql
CREATE TABLE purchases (
    id INTEGER PRIMARY KEY,
    qty INTEGER NOT NULL,
    unit_price INTEGER NOT NULL,
    total INTEGER GENERATED ALWAYS AS (qty * unit_price) STORED
);
INSERT INTO purchases (qty, unit_price) VALUES (3, 1200), (10, 450);
SELECT * FROM purchases;
```

`STORED` columns are computed on write and saved; `VIRTUAL` ones are computed on read (SQLite's default). Either way, they can't get out of sync, and you can index them, which makes searches on an expression (like `lower(email)`) fast.

## Choosing

- Logic you want everywhere and always current: a **view**.
- Steps of one script, or a snapshot: a **temp table** (or a CTE inside one query).
- A slow report many people read: a **materialised view** or summary table.
- A value that's always a function of the row: a **generated column**.

@@quiz
What does a (normal) view store?
* Only its query; the data is read from the base tables every time
- A copy of the data at creation time
- An index
- Nothing; views are deleted after use
? That's why views always show current data.

@@quiz
Which is a good reason to give report users a view instead of the base table?
- Views are always faster
* The view can hide sensitive columns and rows while still giving a clean, joined shape
- Views can't be queried with WHERE
- Base tables can't be joined
? Grant SELECT on the view, not on the table.

@@quiz
When do temporary tables disappear?
- After each statement
* When the connection (session) that created them closes
- Never
- At midnight
? They're also invisible to other connections.

@@drill
Which statement saves a query under a name you can select from?
= CREATE VIEW
? CREATE VIEW open_tickets AS SELECT ...

@@flashcards
view :: a saved query you can SELECT from
`CREATE OR REPLACE VIEW` :: change a view (PostgreSQL, MySQL)
updatable view :: simple views allow UPDATE (not SQLite)
`INSTEAD OF` trigger :: make a SQLite view writable
temporary table :: per-connection snapshot
materialised view :: stored result, refreshed on demand
`GENERATED ALWAYS AS (...) STORED` :: computed column
`WITH CHECK OPTION` :: refuse updates that leave the view

@@exercise A safe staff directory
HR wants a directory everyone can read, without salaries. Create a view `directory` with `name`, `username`, `department` (the department name, or `(contractor)` when there is none) and `title`. Then select the directory rows for Osaka-based departments, sorted by department and name.
@@starter
-- CREATE VIEW directory AS ...

SELECT * FROM directory ORDER BY department, name;
@@expected
name        | username | department | title
------------+----------+------------+----------------
Rina Chen   | rchen    | Finance    | Finance Manager
Taro Yamada | tyamada  | Finance    | Accountant
Emi Suzuki  | esuzuki  | Support    | Support Agent
Leo Martin  | lmartin  | Support    | Support Agent
Sara Ito    | sito     | Support    | Support Lead
@@requires
CREATE VIEW directory
COALESCE(
@@hint
`CREATE VIEW directory AS SELECT e.name, e.username, COALESCE(d.name, '(contractor)') AS department, e.title FROM employees e LEFT JOIN departments d ON d.id = e.dept_id;`. For Osaka, filter with `WHERE department IN (SELECT name FROM departments WHERE location = 'Osaka')`.
@@solution
CREATE VIEW directory AS
SELECT e.name, e.username, COALESCE(d.name, '(contractor)') AS department, e.title
FROM employees e LEFT JOIN departments d ON d.id = e.dept_id;

SELECT * FROM directory
WHERE department IN (SELECT name FROM departments WHERE location = 'Osaka')
ORDER BY department, name;

@@exercise Views see changes; snapshots don't
Create a view `open_count_v` returning the number of open tickets as `n`, and a temp table `open_count_t` with the same query. Then close ticket 12 (set status `closed` and closed `2026-11-01 09:00`) and select both, as `source` and `n`, with UNION ALL.
@@starter
-- create the view and the temp table, update ticket 12, then compare
@@expected
source     | n
-----------+--
view       | 4
temp table | 5
@@requires
CREATE VIEW
CREATE TEMP TABLE
UPDATE tickets
@@hint
`CREATE VIEW open_count_v AS SELECT COUNT(*) AS n FROM tickets WHERE status = 'open';` and `CREATE TEMP TABLE open_count_t AS SELECT COUNT(*) AS n FROM tickets WHERE status = 'open';`. Then `SELECT 'view' AS source, n FROM open_count_v UNION ALL SELECT 'temp table', n FROM open_count_t;`.
@@solution
CREATE VIEW open_count_v AS SELECT COUNT(*) AS n FROM tickets WHERE status = 'open';
CREATE TEMP TABLE open_count_t AS SELECT COUNT(*) AS n FROM tickets WHERE status = 'open';

UPDATE tickets SET status = 'closed', closed = '2026-11-01 09:00' WHERE id = 12;

SELECT 'view' AS source, n FROM open_count_v
UNION ALL
SELECT 'temp table', n FROM open_count_t;

@@exercise A generated column for case-insensitive search
Create a table `contacts(id, email, email_key)` where `email_key` is a generated, stored column holding `lower(trim(email))`, and a **unique** index on it, so `Ana@Example.com ` and `ana@example.com` can't both be stored. Insert the three starter emails one by one with `INSERT OR IGNORE` (the duplicate is skipped), then show `id` and `email_key`.
@@starter
-- CREATE TABLE contacts (...);
-- CREATE UNIQUE INDEX ...;
INSERT OR IGNORE INTO contacts (email) VALUES ('Ana@Example.com ');
INSERT OR IGNORE INTO contacts (email) VALUES ('mia@example.com');
INSERT OR IGNORE INTO contacts (email) VALUES ('ana@example.com');
SELECT id, email_key FROM contacts ORDER BY id;
@@expected
id | email_key
---+----------------
1  | ana@example.com
2  | mia@example.com
@@requires
GENERATED ALWAYS AS
CREATE UNIQUE INDEX
@@hint
`CREATE TABLE contacts (id INTEGER PRIMARY KEY, email TEXT NOT NULL, email_key TEXT GENERATED ALWAYS AS (lower(trim(email))) STORED);` then `CREATE UNIQUE INDEX contacts_email_key ON contacts(email_key);`.
@@solution
CREATE TABLE contacts (
    id INTEGER PRIMARY KEY,
    email TEXT NOT NULL,
    email_key TEXT GENERATED ALWAYS AS (lower(trim(email))) STORED
);
CREATE UNIQUE INDEX contacts_email_key ON contacts(email_key);
INSERT OR IGNORE INTO contacts (email) VALUES ('Ana@Example.com ');
INSERT OR IGNORE INTO contacts (email) VALUES ('mia@example.com');
INSERT OR IGNORE INTO contacts (email) VALUES ('ana@example.com');
SELECT id, email_key FROM contacts ORDER BY id;

@@lesson s6-app-patterns SQL Patterns Every Application Needs
@@concepts errors, concurrency
@@text
Applications ask the database to do more than look things up: take an order without overselling, record who changed what, handle two people editing the same record, and upgrade the schema without losing data. Each of these has a standard SQL pattern.

## Transactions: all or nothing

```sql
CREATE TABLE stock (sku TEXT PRIMARY KEY, qty INTEGER NOT NULL CHECK (qty >= 0));
CREATE TABLE order_lines (order_id INTEGER, sku TEXT, qty INTEGER);
INSERT INTO stock VALUES ('DOCK-C', 5), ('CABLE-HDMI', 40);

BEGIN;
UPDATE stock SET qty = qty - 2 WHERE sku = 'DOCK-C';
INSERT INTO order_lines VALUES (1001, 'DOCK-C', 2);
COMMIT;                       -- both changes happen, or (on ROLLBACK or a crash) neither does

SELECT * FROM stock;
```

In application code, the pattern is always the same: begin, do the work, commit; on any error, roll back. Most libraries do it for you (`with conn:` in Python, `using var tx = ...` in C#, `@Transactional` in Java).

## Don't oversell: let the UPDATE check

Two customers buy the last dock at the same moment. If the app reads the stock, checks it in code, then writes, both reads can see "1". Put the check **in the UPDATE itself**, which the database runs atomically:

```sql,norun
UPDATE stock SET qty = qty - :wanted
WHERE sku = :sku AND qty >= :wanted;
-- then check how many rows changed: 0 means "not enough stock", so roll back
```

## UPSERT: insert or update in one statement

```sql
CREATE TABLE page_views (page TEXT PRIMARY KEY, views INTEGER NOT NULL, last_at TEXT);

INSERT INTO page_views (page, views, last_at) VALUES ('/vpn-guide', 1, '2026-11-01 09:00')
ON CONFLICT (page) DO UPDATE SET views = views + 1, last_at = excluded.last_at;

INSERT INTO page_views (page, views, last_at) VALUES ('/vpn-guide', 1, '2026-11-01 09:05')
ON CONFLICT (page) DO UPDATE SET views = views + 1, last_at = excluded.last_at;

SELECT * FROM page_views;
```

`excluded` is the row you tried to insert. PostgreSQL uses the same syntax; MySQL has `ON DUPLICATE KEY UPDATE`, SQL Server `MERGE`.

## RETURNING: get back what you wrote

```sql
CREATE TABLE requests (id INTEGER PRIMARY KEY, title TEXT, created_at TEXT DEFAULT (datetime('now')));
INSERT INTO requests (title) VALUES ('Printer jam') RETURNING id, title;
```

No second query to find the new id (PostgreSQL and SQLite: `RETURNING`; SQL Server: `OUTPUT inserted.id`).

## Optimistic locking: two people edit the same record

Aiko and Ken both open ticket 7. Ken saves first; then Aiko saves and silently overwrites Ken's change. The fix is a `version` column:

```sql,norun
-- When loading, remember the version (say 3). When saving:
UPDATE tickets SET title = :title, version = version + 1
WHERE id = :id AND version = :version_when_loaded;
-- 1 row changed: saved. 0 rows: someone else saved first, so show "this ticket was changed, reload?"
```

It's called *optimistic* because nothing is locked while people edit: conflicts are detected when they save. Web APIs expose the same idea with `ETag` and `If-Match` headers.

## Triggers: rules that run automatically

```sql
CREATE TABLE laptops (id INTEGER PRIMARY KEY, hostname TEXT, owner TEXT, updated_at TEXT);
CREATE TABLE laptop_history (laptop_id INTEGER, old_owner TEXT, new_owner TEXT, changed_at TEXT);

CREATE TRIGGER trg_laptop_owner AFTER UPDATE OF owner ON laptops
WHEN OLD.owner IS NOT NEW.owner
BEGIN
    INSERT INTO laptop_history VALUES (NEW.id, OLD.owner, NEW.owner, '2026-11-01');
END;

INSERT INTO laptops (id, hostname, owner) VALUES (1, 'tk-lt-001', 'aiko');
UPDATE laptops SET owner = 'ken' WHERE id = 1;
SELECT * FROM laptop_history;
```

Triggers are good for audit trails and keeping derived data in sync, because they can't be forgotten by any code path. Use them sparingly: logic hidden in triggers surprises the next developer.

## Views: a saved query with a name

```sql,norun
CREATE VIEW open_ticket_summary AS
SELECT t.id, t.title, u.name AS assignee, t.priority
FROM tickets t LEFT JOIN users u ON u.id = t.assignee_id
WHERE t.status = 'open';

SELECT * FROM open_ticket_summary WHERE priority = 'high';
```

Views give reports and other teams a stable interface, even when the tables underneath change. They can also hide sensitive columns (grant access to the view, not the table).

## Migrations: changing the schema safely

Never edit the production schema by hand. Keep numbered **migration** files in Git, applied in order by a tool (Flyway, Liquibase, Alembic, EF Core migrations, Rails, or a few lines of your own code), which records which ones have run:

```text
migrations/
  001_create_tickets.sql
  002_add_priority.sql
  003_backfill_priority.sql
```

Rules: never change a migration that has already run anywhere, only add new ones; make big changes in safe steps (add the new column, backfill it, switch the code, *then* drop the old column), and take a backup first.

## From application code: always parameters

```python,norun
cur.execute("SELECT id, title FROM tickets WHERE assignee = ? AND status = ?", (user, "open"))
```

Never build SQL by concatenating input (that's SQL injection, from the previous module). Every language's database library has placeholders: `?`, `:name`, `$1` or `@name`.

@@quiz
Two requests try to buy the last item at the same moment. Which approach is safe?
- Read the stock, check it in code, then UPDATE
* `UPDATE stock SET qty = qty - 1 WHERE sku = ? AND qty >= 1`, then check how many rows changed
- Read the stock twice to make sure
- Use SELECT ... ORDER BY qty
? The check and the change happen in one atomic statement, so only one of them can succeed.

@@quiz
In `ON CONFLICT (page) DO UPDATE SET last_at = excluded.last_at`, what is `excluded`?
- The row that was already in the table
* The row the INSERT tried to add
- A temporary table of deleted rows
- The previous version of the table
? `excluded` holds the values that conflicted; the existing row's columns are used by name.

@@quiz
An `UPDATE ... WHERE id = 7 AND version = 3` changes 0 rows. What does it mean with optimistic locking?
- The database is locked
* Someone else saved the record since it was loaded, so this save must not overwrite it
- Ticket 7 doesn't exist, always
- The transaction rolled back
? The version moved on. Tell the user, and let them reload or merge.

@@quiz
Why keep migrations as numbered files in Git instead of changing the database by hand?
* Every environment (dev, test, production) gets the exact same changes, in order, with a history
- Hand changes are slower to type
- Databases refuse ALTER TABLE without them
- Git compresses the database
? Reproducible, reviewed, versioned schema changes. Same idea as infrastructure as code.

@@drill
Which clause makes INSERT update the existing row when a key already exists (SQLite and PostgreSQL)?
= ON CONFLICT
= ON CONFLICT DO UPDATE
? INSERT ... ON CONFLICT(id) DO UPDATE SET ...

@@flashcards
transaction :: `BEGIN` ... `COMMIT`: all changes or none
`ROLLBACK` :: undo everything since BEGIN
conditional UPDATE :: `... WHERE qty >= :wanted`, then check rows changed
UPSERT :: `INSERT ... ON CONFLICT (key) DO UPDATE SET ...`
`excluded.col` :: the value the INSERT tried to write
`RETURNING` :: get columns of the inserted/updated rows back
optimistic locking :: `WHERE id = ? AND version = ?`; 0 rows = conflict
trigger :: SQL that runs automatically on INSERT/UPDATE/DELETE
`OLD` / `NEW` :: the row before / after the change, in a trigger
view :: a named, saved query
migration :: a numbered, versioned schema change file
parameters :: `?`, `:name`, `$1`: never concatenate input into SQL

@@exercise Restock with an UPSERT
A delivery arrives. Add each line of `delivery` to `stock` with **one** `INSERT ... SELECT ... ON CONFLICT` statement: known SKUs get their `qty` increased and `updated` set to the delivery's date; new SKUs are inserted. Then show the stock.
@@setup
CREATE TABLE stock (sku TEXT PRIMARY KEY, qty INTEGER NOT NULL, updated TEXT NOT NULL);
INSERT INTO stock VALUES ('DOCK-C', 2, '2026-10-01'), ('CABLE-HDMI', 40, '2026-10-01'), ('MOUSE-W', 0, '2026-10-12');
CREATE TABLE delivery (sku TEXT, qty INTEGER, arrived TEXT);
INSERT INTO delivery VALUES ('DOCK-C', 10, '2026-11-01'), ('MOUSE-W', 25, '2026-11-01'), ('HEADSET-BT', 8, '2026-11-01');
@@starter
-- INSERT INTO stock (sku, qty, updated) SELECT ... FROM delivery WHERE true ON CONFLICT ...

SELECT sku, qty, updated FROM stock ORDER BY sku;
@@expected
sku        | qty | updated
-----------+-----+-----------
CABLE-HDMI | 40  | 2026-10-01
DOCK-C     | 12  | 2026-11-01
HEADSET-BT | 8   | 2026-11-01
MOUSE-W    | 25  | 2026-11-01
@@requires
ON CONFLICT
excluded.qty
@@hint
`INSERT INTO stock (sku, qty, updated) SELECT sku, qty, arrived FROM delivery WHERE true ON CONFLICT (sku) DO UPDATE SET qty = stock.qty + excluded.qty, updated = excluded.updated;` (SQLite needs the `WHERE true` so it can tell where the SELECT ends and ON CONFLICT starts).
@@solution
INSERT INTO stock (sku, qty, updated)
SELECT sku, qty, arrived FROM delivery WHERE true
ON CONFLICT (sku) DO UPDATE SET qty = stock.qty + excluded.qty, updated = excluded.updated;

SELECT sku, qty, updated FROM stock ORDER BY sku;

@@exercise An audit trigger
Create a trigger `trg_ticket_status` that, **after the `status` of a ticket changes** (and only when it really changes), inserts a row into `ticket_history` with the ticket id, the old status, the new status and the ticket's `updated_by`. The statements below then make some changes.
@@setup
CREATE TABLE tickets (id INTEGER PRIMARY KEY, title TEXT, status TEXT NOT NULL, updated_by TEXT);
CREATE TABLE ticket_history (id INTEGER PRIMARY KEY, ticket_id INTEGER, old_status TEXT, new_status TEXT, changed_by TEXT);
INSERT INTO tickets VALUES (1, 'VPN drops', 'open', 'aiko'), (2, 'Printer jam', 'open', 'aiko');
@@starter
-- TODO: CREATE TRIGGER trg_ticket_status ...

UPDATE tickets SET status = 'in progress', updated_by = 'ken' WHERE id = 1;
UPDATE tickets SET title = 'Printer jam on 3F', updated_by = 'yuki' WHERE id = 2;
UPDATE tickets SET status = 'in progress', updated_by = 'ken' WHERE id = 1;
UPDATE tickets SET status = 'closed', updated_by = 'mei' WHERE id IN (1, 2);

SELECT ticket_id, old_status, new_status, changed_by FROM ticket_history ORDER BY id;
@@expected
ticket_id | old_status  | new_status  | changed_by
----------+-------------+-------------+-----------
1         | open        | in progress | ken
1         | in progress | closed      | mei
2         | open        | closed      | mei
@@requires
CREATE TRIGGER trg_ticket_status
AFTER UPDATE OF status ON tickets
@@hint
`CREATE TRIGGER trg_ticket_status AFTER UPDATE OF status ON tickets WHEN OLD.status IS NOT NEW.status BEGIN INSERT INTO ticket_history (ticket_id, old_status, new_status, changed_by) VALUES (NEW.id, OLD.status, NEW.status, NEW.updated_by); END;`
@@solution
CREATE TRIGGER trg_ticket_status AFTER UPDATE OF status ON tickets
WHEN OLD.status IS NOT NEW.status
BEGIN
    INSERT INTO ticket_history (ticket_id, old_status, new_status, changed_by)
    VALUES (NEW.id, OLD.status, NEW.status, NEW.updated_by);
END;

UPDATE tickets SET status = 'in progress', updated_by = 'ken' WHERE id = 1;
UPDATE tickets SET title = 'Printer jam on 3F', updated_by = 'yuki' WHERE id = 2;
UPDATE tickets SET status = 'in progress', updated_by = 'ken' WHERE id = 1;
UPDATE tickets SET status = 'closed', updated_by = 'mei' WHERE id IN (1, 2);

SELECT ticket_id, old_status, new_status, changed_by FROM ticket_history ORDER BY id;

@@exercise Optimistic locking
Aiko and Ken both loaded ticket 7 at `version` 3. Write their two saves as optimistic-locking UPDATEs (Ken's first: title `VPN drops in Osaka`; then Aiko's: title `VPN drops hourly`), each followed by `SELECT changes() AS saved;` to show whether it worked. Finally show the ticket.
@@setup
CREATE TABLE tickets (id INTEGER PRIMARY KEY, title TEXT NOT NULL, version INTEGER NOT NULL);
INSERT INTO tickets VALUES (7, 'VPN drops', 3);
@@starter
-- Ken's save

-- Aiko's save

SELECT id, title, version FROM tickets;
@@expected
saved
-----
1

saved
-----
0

id | title              | version
---+--------------------+--------
7  | VPN drops in Osaka | 4
@@requires
version = version + 1
AND version = 3
SELECT changes()
@@hint
`UPDATE tickets SET title = 'VPN drops in Osaka', version = version + 1 WHERE id = 7 AND version = 3; SELECT changes() AS saved;` and the same for Aiko: she also still has version 3, so her update changes nothing.
@@solution
-- Ken's save
UPDATE tickets SET title = 'VPN drops in Osaka', version = version + 1 WHERE id = 7 AND version = 3;
SELECT changes() AS saved;
-- Aiko's save
UPDATE tickets SET title = 'VPN drops hourly', version = version + 1 WHERE id = 7 AND version = 3;
SELECT changes() AS saved;

SELECT id, title, version FROM tickets;

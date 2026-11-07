@@lesson e7-triggers Constraints, Foreign Key Actions and Triggers in Depth
@@concepts errors, automation
@@text
Application code has bugs, admins run ad-hoc UPDATEs, and imports skip validation. The database is the last line of defence, and **constraints** are how it defends the data. Triggers go further, running your own logic whenever rows change.

## The constraint toolbox

| Constraint | Guarantees |
|---|---|
| `NOT NULL` | a value is always present |
| `UNIQUE` | no duplicates (NULLs allowed, usually several) |
| `PRIMARY KEY` | unique + not null: the row's identity |
| `CHECK (expr)` | expr is true or NULL for every row |
| `FOREIGN KEY ... REFERENCES` | the value exists in the parent table |
| `EXCLUDE` (PostgreSQL) | no two rows "overlap" (e.g. room bookings) |

Name your constraints (`CONSTRAINT price_positive CHECK (price > 0)`): error messages then tell you which rule failed.

A CHECK can compare columns of the same row: `CHECK (closed IS NULL OR closed >= opened)`, `CHECK (status <> 'closed' OR closed IS NOT NULL)`. It can't look at other rows or tables; for that you need a foreign key, a unique index or a trigger.

## Foreign keys in SQLite

SQLite enforces foreign keys only when you turn them on, **per connection**:

```sql
PRAGMA foreign_keys = ON;
```

Every application that uses SQLite should run that right after connecting. PostgreSQL, SQL Server and MySQL (InnoDB) always enforce them.

## What happens to children: ON DELETE / ON UPDATE

When a parent row is deleted (or its key changes), the foreign key's action decides what happens to the rows that reference it:

| Action | Effect |
|---|---|
| `NO ACTION` / `RESTRICT` (default) | refuse the delete while children exist |
| `CASCADE` | delete (or update) the children too |
| `SET NULL` | set the child's foreign key to NULL |
| `SET DEFAULT` | set it to the column default |

Choose per relationship: an order's **lines** should CASCADE (they mean nothing without the order); a ticket's **assignee** should SET NULL (the ticket stays when someone leaves); an invoice's **customer** should RESTRICT (never lose financial records by accident).

## Partial and expression uniqueness

"Only one **active** assignment per laptop", "usernames unique ignoring case": unique **indexes** can do what a UNIQUE constraint can't:

```sql
CREATE TABLE loans (device_id INTEGER, employee_id INTEGER, returned TEXT);
CREATE UNIQUE INDEX one_active_loan ON loans (device_id) WHERE returned IS NULL;
INSERT INTO loans VALUES (1, 2, '2026-01-31'), (1, 3, NULL);
SELECT COUNT(*) AS loans FROM loans;
```

## Triggers

A trigger runs SQL automatically `BEFORE`, `AFTER` or `INSTEAD OF` an `INSERT`, `UPDATE` (optionally `UPDATE OF column`) or `DELETE`. Inside, `NEW` is the row after the change and `OLD` the row before.

```sql,norun
CREATE TRIGGER tickets_closed_time
AFTER UPDATE OF status ON tickets
WHEN NEW.status = 'closed' AND NEW.closed IS NULL
BEGIN
    UPDATE tickets SET closed = datetime('now') WHERE id = NEW.id;
END;
```

### Rejecting changes

In SQLite, `SELECT RAISE(ABORT, 'message')` inside a trigger cancels the statement with an error, and `SELECT RAISE(IGNORE)` in a BEFORE trigger silently skips the current row (PostgreSQL: `RAISE EXCEPTION` in a trigger function). Use it for rules a CHECK can't express because they involve other rows: "a closed ticket can't be reopened", "no more than 3 devices per person".

### INSTEAD OF triggers on views

Views in SQLite are read-only, but an `INSTEAD OF INSERT/UPDATE/DELETE` trigger on a view says what to do instead, which makes the view writable. This is how you keep an old interface working after splitting a table.

## When not to use triggers

Triggers are invisible: someone reading the application code won't know an UPDATE also writes three other tables. Chains of triggers firing triggers are hard to debug, and triggers slow down bulk loads. Good uses: **audit logs**, **integrity rules** across rows, keeping **derived data** (counters, search indexes) in sync. Business workflows (sending emails, calling APIs) belong in the application.

## Deferred constraints

Sometimes a transaction must break a rule temporarily, for example swapping two rows' unique positions, or inserting two rows that reference each other. PostgreSQL and SQLite allow `DEFERRABLE INITIALLY DEFERRED` foreign keys, which are checked at COMMIT instead of after each statement. SQLite also has `PRAGMA defer_foreign_keys = ON` for one transaction.

@@quiz
Which ON DELETE action fits a ticket's `assignee_id` when the employee is deleted?
- CASCADE
* SET NULL
- RESTRICT
- NO ACTION
? The ticket must stay; it just loses its assignee.

@@quiz
Why does a SQLite application need `PRAGMA foreign_keys = ON`?
* SQLite doesn't enforce foreign keys unless it's enabled on each connection
- It makes queries faster
- It creates the foreign keys
- It's only needed for CASCADE
? It's off by default for backwards compatibility.

@@quiz
How do you allow many returned loans of a device but only one active loan?
- A CHECK constraint
- A UNIQUE constraint on device_id
* A partial unique index on device_id WHERE returned IS NULL
- A foreign key
? Partial indexes only include rows matching their WHERE.

@@quiz
What does `RAISE(ABORT, 'msg')` do in a SQLite trigger?
- Logs a warning
* Cancels the statement that fired the trigger with an error
- Skips the trigger
- Rolls back all transactions
? Use it to enforce rules a CHECK can't express.

@@drill
Which foreign key action deletes the child rows when the parent is deleted?
= ON DELETE CASCADE
= CASCADE
? ON DELETE RESTRICT blocks the delete instead.

@@flashcards
`CHECK (expr)` :: row-level rule
`PRAGMA foreign_keys = ON` :: enable FK enforcement in SQLite
`ON DELETE CASCADE` :: delete children with the parent
`ON DELETE SET NULL` :: keep children, clear the link
`ON DELETE RESTRICT` :: refuse while children exist
partial unique index :: unique only among rows matching WHERE
`NEW` / `OLD` :: row after / before, in a trigger
`RAISE(ABORT, 'msg')` :: reject a change from a trigger
`INSTEAD OF` trigger :: make a view writable
`DEFERRABLE INITIALLY DEFERRED` :: check the constraint at COMMIT

@@exercise Choosing foreign key actions
Create `orders(id)` and `order_lines(id, order_id, item)` where lines are deleted with their order, and `notes(id, order_id, text)` where notes survive with `order_id` set to NULL. Turn foreign keys on, insert the starter rows, delete order 1, and show the remaining lines and notes as `kind`, `id`, `order_id`.
@@starter
PRAGMA foreign_keys = ON;
CREATE TABLE orders (id INTEGER PRIMARY KEY);
-- CREATE TABLE order_lines (...);
-- CREATE TABLE notes (...);
INSERT INTO orders VALUES (1), (2);
INSERT INTO order_lines VALUES (1, 1, 'Dock'), (2, 1, 'Cable'), (3, 2, 'Mouse');
INSERT INTO notes VALUES (1, 1, 'Deliver to 3F'), (2, 2, 'Urgent');
DELETE FROM orders WHERE id = 1;
SELECT 'line' AS kind, id, order_id FROM order_lines
UNION ALL
SELECT 'note', id, order_id FROM notes
ORDER BY kind, id;
@@expected
kind | id | order_id
-----+----+---------
line | 3  | 2
note | 1  | NULL
note | 2  | 2
@@requires
ON DELETE CASCADE
ON DELETE SET NULL
@@hint
`CREATE TABLE order_lines (id INTEGER PRIMARY KEY, order_id INTEGER NOT NULL REFERENCES orders(id) ON DELETE CASCADE, item TEXT);` and `notes` with `REFERENCES orders(id) ON DELETE SET NULL`.
@@solution
PRAGMA foreign_keys = ON;
CREATE TABLE orders (id INTEGER PRIMARY KEY);
CREATE TABLE order_lines (id INTEGER PRIMARY KEY, order_id INTEGER NOT NULL REFERENCES orders(id) ON DELETE CASCADE, item TEXT);
CREATE TABLE notes (id INTEGER PRIMARY KEY, order_id INTEGER REFERENCES orders(id) ON DELETE SET NULL, text TEXT);
INSERT INTO orders VALUES (1), (2);
INSERT INTO order_lines VALUES (1, 1, 'Dock'), (2, 1, 'Cable'), (3, 2, 'Mouse');
INSERT INTO notes VALUES (1, 1, 'Deliver to 3F'), (2, 2, 'Urgent');
DELETE FROM orders WHERE id = 1;
SELECT 'line' AS kind, id, order_id FROM order_lines
UNION ALL
SELECT 'note', id, order_id FROM notes
ORDER BY kind, id;

@@exercise No reopening closed tickets
Add a trigger `no_reopen` that stops any UPDATE from changing a ticket's status **from** `closed` to something else. Here the rule should skip such rows **silently** with `RAISE(IGNORE)` (a real system would more often use `RAISE(ABORT, 'closed tickets cannot be reopened')`, which fails the whole statement). Then run the starter's bulk update, which tries to set every ticket to `in-progress`, and show the tickets.
@@setup
CREATE TABLE tickets (id INTEGER PRIMARY KEY, status TEXT NOT NULL);
INSERT INTO tickets VALUES (1, 'closed'), (2, 'open'), (3, 'resolved'), (4, 'closed');
@@starter
-- CREATE TRIGGER no_reopen ...

UPDATE tickets SET status = 'in-progress';
SELECT id, status FROM tickets ORDER BY id;
@@expected
id | status
---+------------
1  | closed
2  | in-progress
3  | in-progress
4  | closed
@@requires
CREATE TRIGGER no_reopen
RAISE(IGNORE)
OLD.status = 'closed'
@@hint
`CREATE TRIGGER no_reopen BEFORE UPDATE OF status ON tickets WHEN OLD.status = 'closed' AND NEW.status <> 'closed' BEGIN SELECT RAISE(IGNORE); END;`. RAISE(IGNORE) in a BEFORE trigger skips the change for that row and carries on with the others.
@@solution
CREATE TRIGGER no_reopen BEFORE UPDATE OF status ON tickets
WHEN OLD.status = 'closed' AND NEW.status <> 'closed'
BEGIN
    SELECT RAISE(IGNORE);
END;

UPDATE tickets SET status = 'in-progress';
SELECT id, status FROM tickets ORDER BY id;

@@exercise A writable view
The old `staff` view combined first and last name into `full_name`. The table now stores them separately. Create the view `staff(id, full_name)` over `people(id, first, last)` and an `INSTEAD OF INSERT` trigger that splits `full_name` at the first space, so old code that inserts into `staff` keeps working. Insert `Mia Lee` and `Kenji Tanaka` through the view, then show the `people` table.
@@setup
CREATE TABLE people (id INTEGER PRIMARY KEY, first TEXT NOT NULL, last TEXT NOT NULL);
INSERT INTO people (first, last) VALUES ('Ana', 'Silva');
@@starter
-- CREATE VIEW staff AS ...;
-- CREATE TRIGGER staff_insert INSTEAD OF INSERT ON staff ...;
INSERT INTO staff (full_name) VALUES ('Mia Lee');
INSERT INTO staff (full_name) VALUES ('Kenji Tanaka');
SELECT id, first, last FROM people ORDER BY id;
@@expected
id | first | last
---+-------+-------
1  | Ana   | Silva
2  | Mia   | Lee
3  | Kenji | Tanaka
@@requires
INSTEAD OF INSERT
CREATE VIEW staff
substr(
@@hint
`CREATE VIEW staff AS SELECT id, first || ' ' || last AS full_name FROM people;` Trigger body: `INSERT INTO people (first, last) VALUES (substr(NEW.full_name, 1, instr(NEW.full_name, ' ') - 1), substr(NEW.full_name, instr(NEW.full_name, ' ') + 1));`
@@solution
CREATE VIEW staff AS SELECT id, first || ' ' || last AS full_name FROM people;
CREATE TRIGGER staff_insert INSTEAD OF INSERT ON staff
BEGIN
    INSERT INTO people (first, last)
    VALUES (substr(NEW.full_name, 1, instr(NEW.full_name, ' ') - 1),
            substr(NEW.full_name, instr(NEW.full_name, ' ') + 1));
END;
INSERT INTO staff (full_name) VALUES ('Mia Lee');
INSERT INTO staff (full_name) VALUES ('Kenji Tanaka');
SELECT id, first, last FROM people ORDER BY id;

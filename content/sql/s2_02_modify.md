@@lesson s2-modify Changing Data: INSERT, UPDATE, DELETE, Transactions
@@text
## INSERT

```sql
INSERT INTO departments (name, location) VALUES ('Security', 'Tokyo');
SELECT * FROM departments WHERE name = 'Security';
```

Name the columns: then the statement still works if someone adds a column to the table later. `id` is filled in automatically (an `INTEGER PRIMARY KEY` in SQLite; `AUTO_INCREMENT` in MySQL, `IDENTITY` in SQL Server, `SERIAL` in PostgreSQL).

## UPDATE

```sql
UPDATE devices SET employee_id = 3 WHERE asset_tag = 'LT-0060';
SELECT asset_tag, employee_id FROM devices WHERE asset_tag = 'LT-0060';
```

> ⚠ **An UPDATE or DELETE without WHERE changes every row.** `UPDATE employees SET salary = 0;` is a famous way to ruin a day. Habit: write the `WHERE` first, and run it as a `SELECT` to see which rows it hits.

## DELETE

```sql
DELETE FROM logins WHERE time < '2026-10-30';
SELECT COUNT(*) AS remaining FROM logins;
```

## Transactions

A **transaction** groups changes so they all happen, or none do:

```sql
BEGIN;
UPDATE devices SET employee_id = NULL WHERE employee_id = 8;      -- Taro returns his laptop
UPDATE devices SET employee_id = 8 WHERE asset_tag = 'LT-0060';   -- and gets the new one
COMMIT;
SELECT asset_tag, employee_id FROM devices WHERE asset_tag IN ('LT-0052', 'LT-0060');
```

`ROLLBACK` instead of `COMMIT` undoes everything since `BEGIN`. Databases guarantee **ACID**: changes are **A**tomic (all or nothing), keep the data **C**onsistent with its rules, transactions are **I**solated from each other, and committed data is **D**urable (survives a crash).

```sql
BEGIN;
DELETE FROM tickets;
SELECT COUNT(*) AS during FROM tickets;
ROLLBACK;
SELECT COUNT(*) AS after_rollback FROM tickets;
```

@@quiz
What does `UPDATE tickets SET status = 'closed';` do?
* Closes every ticket in the table
- Closes the last ticket
- Nothing, WHERE is required
- Asks for confirmation
? No WHERE means every row. Most tools won't warn you.

@@quiz
What does the A in ACID stand for?
* Atomic: a transaction happens completely or not at all
- Automatic
- Available
- Audited
? Atomicity is why a money transfer can't leave the debit done but the credit missing.

@@flashcards
INSERT INTO t (cols) VALUES (...) :: Add a row
UPDATE t SET col = value WHERE ... :: Change rows (WHERE!)
DELETE FROM t WHERE ... :: Remove rows (WHERE!)
BEGIN / COMMIT / ROLLBACK :: Start / save / undo a transaction
ACID :: Atomic, Consistent, Isolated, Durable

@@exercise Onboard a new hire
Add employee **Nao Fujita**, username `nfujita`, in IT (`dept_id` 1), title `Help Desk Analyst`, hired `2026-11-01`, salary `4200000`, manager `1`. Then show the `name`, `username` and `title` of all IT employees ordered by name.
@@starter
-- INSERT, then SELECT

@@expected
name         | username | title
-------------+----------+------------------
Ana Silva    | asilva   | Systems Engineer
Kenji Tanaka | ktanaka  | IT Manager
Mia Lee      | mlee     | Help Desk Analyst
Nao Fujita   | nfujita  | Help Desk Analyst
@@hint
`INSERT INTO employees (name, username, dept_id, title, hired, salary, manager_id) VALUES (...);`
@@solution
INSERT INTO employees (name, username, dept_id, title, hired, salary, manager_id)
VALUES ('Nao Fujita', 'nfujita', 1, 'Help Desk Analyst', '2026-11-01', 4200000, 1);
SELECT name, username, title FROM employees WHERE dept_id = 1 ORDER BY name;

@@exercise Close resolved tickets
Our process: tickets `resolved` for a while become `closed`. Set the status of all `resolved` tickets to `closed`, then show the number of tickets per status (`status`, `tickets`), ordered by status.
@@starter
-- UPDATE, then a GROUP BY query

@@expected
status      | tickets
------------+--------
closed      | 8
in-progress | 2
open        | 5
@@hint
`UPDATE tickets SET status = 'closed' WHERE status = 'resolved';`
@@solution
UPDATE tickets SET status = 'closed' WHERE status = 'resolved';
SELECT status, COUNT(*) AS tickets FROM tickets GROUP BY status ORDER BY status;

@@exercise Offboarding
Ben Jones (`id` 5) is leaving. In **one transaction**: unassign all his devices (set `employee_id` to NULL) and reassign his open tickets' **requester** to his manager (`id` 4). Then show the unassigned devices' `asset_tag`s ordered by tag.
@@starter
-- BEGIN; ... COMMIT; then SELECT

@@expected
asset_tag
---------
LT-0050
LT-0060
PH-0102
PR-0003
SV-0001
SV-0002
@@requires
BEGIN
COMMIT
@@hint
Two UPDATEs between `BEGIN;` and `COMMIT;`. Unassigned devices: `WHERE employee_id IS NULL`.
@@solution
BEGIN;
UPDATE devices SET employee_id = NULL WHERE employee_id = 5;
UPDATE tickets SET requester_id = 4 WHERE requester_id = 5 AND status IN ('open', 'in-progress');
COMMIT;
SELECT asset_tag FROM devices WHERE employee_id IS NULL ORDER BY asset_tag;

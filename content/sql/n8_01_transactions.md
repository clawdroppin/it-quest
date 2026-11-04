@@lesson n8-transactions Transactions, Isolation and Locking
@@concepts errors, concurrency
@@text
Module 2 showed `BEGIN` / `COMMIT` / `ROLLBACK`. In production, many users and programs hit the database **at the same time**, and that's where transactions get interesting.

## ACID

| Letter | Promise |
|---|---|
| **Atomicity** | all statements of a transaction happen, or none do |
| **Consistency** | constraints hold before and after |
| **Isolation** | concurrent transactions don't see each other's half-done work (to a degree you choose) |
| **Durability** | once COMMIT returns, the data survives a crash (it's in the write-ahead log on disk) |

## What can go wrong without isolation

| Anomaly | Example |
|---|---|
| **Dirty read** | you read a row another transaction changed but hasn't committed (and may roll back) |
| **Non-repeatable read** | you read a row twice in one transaction and get different values |
| **Phantom read** | you run the same query twice and new rows appear |
| **Lost update** | two transactions read qty = 5, both write 4: one sale is lost |
| **Write skew** | two doctors each check "at least one other doctor is on call", both go off call |

## Isolation levels

The SQL standard defines four levels, each preventing more anomalies (and allowing less concurrency):

| Level | Dirty | Non-repeatable | Phantom |
|---|---|---|---|
| Read Uncommitted | possible | possible | possible |
| Read Committed | – | possible | possible |
| Repeatable Read | – | – | possible (not in PostgreSQL) |
| Serializable | – | – | – |

Defaults: **PostgreSQL, SQL Server and Oracle use Read Committed**; MySQL InnoDB uses Repeatable Read; SQLite is effectively **Serializable** (only one writer at a time).

```sql,norun
BEGIN TRANSACTION ISOLATION LEVEL SERIALIZABLE;   -- PostgreSQL
SET TRANSACTION ISOLATION LEVEL SNAPSHOT;          -- SQL Server
```

At Serializable, the database may **abort** a transaction that conflicts with another ("could not serialize access"). Your application must then **retry** the whole transaction. Every serious app that uses Serializable has a retry loop.

## How databases isolate: locks and MVCC

- **Locking**: a writer locks the rows it changes; others wait. Simple, but readers and writers block each other.
- **MVCC** (multi-version concurrency control, used by PostgreSQL, Oracle, MySQL InnoDB, SQL Server's snapshot modes and SQLite in WAL mode): a write creates a **new version** of the row. Each transaction reads from a **snapshot**, so readers never block writers and writers never block readers. Old versions are cleaned up later (PostgreSQL's VACUUM).

**SQLite** locks the whole database for writing: many readers, one writer. `BEGIN IMMEDIATE` takes the write lock at the start (instead of at the first write), which avoids "database is locked" errors when two transactions both read first and then try to write. Enable `PRAGMA journal_mode = WAL` so readers don't block the writer.

## Preventing lost updates

The read-modify-write pattern in application code is the classic bug:

```text
qty = SELECT qty FROM stock WHERE sku = 'DOCK'     -- both sessions read 5
UPDATE stock SET qty = 4 WHERE sku = 'DOCK'        -- both write 4
```

Fixes, best first:

1. **Do it in one statement**: `UPDATE stock SET qty = qty - 1 WHERE sku = 'DOCK' AND qty >= 1;` then check how many rows changed (0 = out of stock). The database serialises the row change.
2. **Lock the row** while you think: `SELECT ... FOR UPDATE` (PostgreSQL, MySQL, Oracle; SQL Server uses `WITH (UPDLOCK)`).
3. **Optimistic locking**: a `version` column; `UPDATE ... SET ..., version = version + 1 WHERE id = ? AND version = ?` and retry if 0 rows changed.
4. **Serializable** isolation plus retries.

## Deadlocks

Transaction A locks row 1 then wants row 2; B locks row 2 then wants row 1. Neither can continue. The database detects the cycle and kills one of them (the "deadlock victim"), which must retry. Avoid deadlocks by **always locking things in the same order** (for example by ascending id) and keeping transactions short.

## Savepoints

A savepoint is a bookmark inside a transaction. `ROLLBACK TO name` undoes everything after it, while keeping the earlier work and the transaction open:

```sql
BEGIN;
INSERT INTO departments VALUES (6, 'Legal', 'Tokyo');
SAVEPOINT before_risky;
UPDATE employees SET dept_id = 6;               -- oops, no WHERE!
ROLLBACK TO before_risky;
COMMIT;
SELECT COUNT(*) AS in_legal FROM employees WHERE dept_id = 6;
```

Use them for "try this part, and if it fails, skip it but keep the rest" (batch imports), and for nested operations in application frameworks.

## Job queues: SKIP LOCKED

Many workers taking jobs from one table must not take the same job. PostgreSQL and MySQL 8 have the perfect tool:

```sql,norun
UPDATE jobs SET worker = 'w1', started = now()
WHERE id = (SELECT id FROM jobs WHERE worker IS NULL ORDER BY id
            FOR UPDATE SKIP LOCKED LIMIT 1)
RETURNING id;
```

`SKIP LOCKED` makes each worker skip rows another worker has locked, instead of waiting. In SQLite (one writer at a time) a single `UPDATE ... WHERE id = (SELECT ... LIMIT 1) AND worker IS NULL` is already safe.

## Keep transactions short

A transaction holds locks (or old row versions) until it ends. Never wait for user input, a network call or a slow file inside a transaction. Long transactions cause lock waits, deadlocks, bloat (PostgreSQL can't clean up old versions) and replication lag.

@@quiz
Two sessions both read `qty = 5` and write `qty = 4`. What's this anomaly called?
- Dirty read
- Phantom read
* Lost update
- Deadlock
? Fix it with a single `UPDATE ... SET qty = qty - 1`, a row lock or optimistic locking.

@@quiz
What's the default isolation level in PostgreSQL?
- Read Uncommitted
* Read Committed
- Repeatable Read
- Serializable
? Same as SQL Server and Oracle; MySQL defaults to Repeatable Read.

@@quiz
What must an application do when a Serializable transaction is aborted with a serialization failure?
- Ignore it
- Lower the isolation level permanently
* Retry the whole transaction
- Restart the database
? Serializable trades occasional aborts for correctness.

@@quiz
What does `ROLLBACK TO sp` do?
- Ends the transaction
* Undoes the changes made after savepoint sp, keeping the transaction open
- Rolls back the whole transaction
- Deletes the savepoint only
? Earlier work is kept and can still be committed.

@@flashcards
ACID :: Atomicity, Consistency, Isolation, Durability
dirty read :: reading uncommitted data
non-repeatable read :: same row, different value within a transaction
phantom :: new rows appear in a repeated query
lost update :: concurrent read-modify-write overwrites
MVCC :: readers see a snapshot; writers make new versions
`BEGIN IMMEDIATE` :: SQLite: take the write lock now
`SELECT ... FOR UPDATE` :: lock rows you'll change
deadlock :: two transactions wait for each other
`SAVEPOINT` / `ROLLBACK TO` :: partial undo
`FOR UPDATE SKIP LOCKED` :: job queue pattern

@@exercise A lost update, then the fix
The starter simulates two help desk agents taking a dock from stock at the same time by reading `qty` into a `session` table first and writing back later. Run it as given to see the lost update, then add a second part that does the same two withdrawals correctly with **atomic** updates (`qty = qty - 1`) on a fresh row `DOCK-B`. Show both stock rows.
@@setup
CREATE TABLE stock (sku TEXT PRIMARY KEY, qty INTEGER NOT NULL CHECK (qty >= 0));
INSERT INTO stock VALUES ('DOCK-A', 5), ('DOCK-B', 5);
CREATE TABLE session (agent TEXT, seen INTEGER);
@@starter
-- Both agents read the quantity...
INSERT INTO session SELECT 'mia', qty FROM stock WHERE sku = 'DOCK-A';
INSERT INTO session SELECT 'ana', qty FROM stock WHERE sku = 'DOCK-A';
-- ...and each writes back what they saw minus one.
UPDATE stock SET qty = (SELECT seen - 1 FROM session WHERE agent = 'mia') WHERE sku = 'DOCK-A';
UPDATE stock SET qty = (SELECT seen - 1 FROM session WHERE agent = 'ana') WHERE sku = 'DOCK-A';

-- TODO: two correct withdrawals from DOCK-B

SELECT sku, qty FROM stock ORDER BY sku;
@@expected
sku    | qty
-------+----
DOCK-A | 4
DOCK-B | 3
@@requires
qty = qty - 1
@@hint
`UPDATE stock SET qty = qty - 1 WHERE sku = 'DOCK-B' AND qty >= 1;` twice. The `qty >= 1` guard means a withdrawal from an empty stock changes 0 rows instead of breaking the CHECK.
@@solution
-- Both agents read the quantity...
INSERT INTO session SELECT 'mia', qty FROM stock WHERE sku = 'DOCK-A';
INSERT INTO session SELECT 'ana', qty FROM stock WHERE sku = 'DOCK-A';
-- ...and each writes back what they saw minus one.
UPDATE stock SET qty = (SELECT seen - 1 FROM session WHERE agent = 'mia') WHERE sku = 'DOCK-A';
UPDATE stock SET qty = (SELECT seen - 1 FROM session WHERE agent = 'ana') WHERE sku = 'DOCK-A';

UPDATE stock SET qty = qty - 1 WHERE sku = 'DOCK-B' AND qty >= 1;
UPDATE stock SET qty = qty - 1 WHERE sku = 'DOCK-B' AND qty >= 1;

SELECT sku, qty FROM stock ORDER BY sku;

@@exercise A batch import with savepoints
Import the `incoming` rows into `assets` inside **one** transaction. Each row gets its own savepoint: rows whose tag already exists must be skipped by rolling back to their savepoint, while the good rows are kept. The starter checks the duplicate for you with a `log` table; you write the transaction around it. Show the assets and the log at the end.
@@setup
CREATE TABLE assets (tag TEXT PRIMARY KEY, model TEXT NOT NULL);
INSERT INTO assets VALUES ('LT-0042', 'ThinkPad T14');
CREATE TABLE log (tag TEXT, result TEXT);
@@starter
-- Row 1: LT-0070
INSERT INTO assets VALUES ('LT-0070', 'ThinkPad X1');
INSERT INTO log VALUES ('LT-0070', 'imported');
-- Row 2: LT-0042 (a duplicate: undo whatever this row did)
INSERT OR IGNORE INTO assets VALUES ('LT-0042', 'Dell Latitude');
INSERT INTO log VALUES ('LT-0042', 'imported');
-- Row 3: PH-0200
INSERT INTO assets VALUES ('PH-0200', 'iPhone 16');
INSERT INTO log VALUES ('PH-0200', 'imported');

SELECT tag, model FROM assets ORDER BY tag;
SELECT tag, result FROM log ORDER BY rowid;
@@expected
tag     | model
--------+-------------
LT-0042 | ThinkPad T14
LT-0070 | ThinkPad X1
PH-0200 | iPhone 16

tag     | result
--------+-------------------
LT-0070 | imported
LT-0042 | skipped: duplicate
PH-0200 | imported
@@requires
BEGIN
SAVEPOINT
ROLLBACK TO
COMMIT
@@hint
`BEGIN;` then before each row `SAVEPOINT row1;` (row2, row3). After row 2's statements, `ROLLBACK TO row2;` and record the skip with `INSERT INTO log VALUES ('LT-0042', 'skipped: duplicate');`. Finish with `COMMIT;`. Use a plain INSERT for row 2's asset only if you keep the OR IGNORE: a real error would stop this script.
@@solution
BEGIN;
SAVEPOINT row1;
INSERT INTO assets VALUES ('LT-0070', 'ThinkPad X1');
INSERT INTO log VALUES ('LT-0070', 'imported');
RELEASE row1;

SAVEPOINT row2;
INSERT OR IGNORE INTO assets VALUES ('LT-0042', 'Dell Latitude');
INSERT INTO log VALUES ('LT-0042', 'imported');
ROLLBACK TO row2;
RELEASE row2;
INSERT INTO log VALUES ('LT-0042', 'skipped: duplicate');

SAVEPOINT row3;
INSERT INTO assets VALUES ('PH-0200', 'iPhone 16');
INSERT INTO log VALUES ('PH-0200', 'imported');
RELEASE row3;
COMMIT;

SELECT tag, model FROM assets ORDER BY tag;
SELECT tag, result FROM log ORDER BY rowid;

@@exercise Claiming jobs from a queue
Three workers take jobs from `jobs` in id order. Write the claim as **one** statement that sets `worker` and `claimed` on the lowest-id unclaimed job, and use it for `w1`, `w2`, `w1` (in that order, with claimed times `10:00`, `10:01`, `10:02`). A claim must never take a job that's already claimed. Show all jobs.
@@setup
CREATE TABLE jobs (id INTEGER PRIMARY KEY, task TEXT, worker TEXT, claimed TEXT);
INSERT INTO jobs (task) VALUES ('reimage LT-0052'), ('rotate backups'), ('patch SV-0002'), ('renew certs');
UPDATE jobs SET worker = 'w0', claimed = '09:55' WHERE id = 2;
@@starter
UPDATE jobs SET worker = 'w1', claimed = '10:00' WHERE id = 1;
SELECT id, task, worker, claimed FROM jobs ORDER BY id;
@@expected
id | task            | worker | claimed
---+-----------------+--------+--------
1  | reimage LT-0052 | w1     | 10:00
2  | rotate backups  | w0     | 09:55
3  | patch SV-0002   | w2     | 10:01
4  | renew certs     | w1     | 10:02
@@requires
worker IS NULL
LIMIT 1
@@hint
`UPDATE jobs SET worker = 'w1', claimed = '10:00' WHERE id = (SELECT id FROM jobs WHERE worker IS NULL ORDER BY id LIMIT 1) AND worker IS NULL;` The second `worker IS NULL` protects you if another writer slipped in between (in PostgreSQL you'd add FOR UPDATE SKIP LOCKED).
@@solution
UPDATE jobs SET worker = 'w1', claimed = '10:00'
WHERE id = (SELECT id FROM jobs WHERE worker IS NULL ORDER BY id LIMIT 1) AND worker IS NULL;
UPDATE jobs SET worker = 'w2', claimed = '10:01'
WHERE id = (SELECT id FROM jobs WHERE worker IS NULL ORDER BY id LIMIT 1) AND worker IS NULL;
UPDATE jobs SET worker = 'w1', claimed = '10:02'
WHERE id = (SELECT id FROM jobs WHERE worker IS NULL ORDER BY id LIMIT 1) AND worker IS NULL;
SELECT id, task, worker, claimed FROM jobs ORDER BY id;

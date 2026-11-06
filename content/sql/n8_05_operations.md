@@lesson n8-operations Running Databases: Backups, Replication, Migrations and Health Checks
@@concepts automation, files
@@text
A database admin's job is mostly about one promise: **the data will still be there, correct, tomorrow**. This lesson covers the operational side: backups, high availability, schema changes without downtime, and routine health checks.

## Backups

| Kind | How | Restore |
|---|---|---|
| **Logical** | `pg_dump`, `mysqldump`, `sqlite3 db .dump`: SQL or a portable archive | slow for big databases, but works across versions |
| **Physical** | copy the data files (`pg_basebackup`, storage snapshots) | fast, same version only |
| **Continuous / PITR** | base backup + every WAL/transaction log file | restore to **any moment**, e.g. one second before the bad `DELETE` |

For SQLite: `VACUUM INTO 'backup.db'` or the online backup API (`.backup` in the CLI) make a consistent copy while the database is in use. Copying the file while it's being written can give you a corrupt backup.

The rules that matter more than the tool:

- **3-2-1**: 3 copies, on 2 kinds of media, 1 off-site (and ideally one immutable/offline, against ransomware).
- **Test restores** regularly. A backup you haven't restored is a hope, not a backup.
- Know your **RPO** (how much data you can lose: "15 minutes") and **RTO** (how long you can be down: "1 hour"). They decide the backup design.
- Encrypt backups and protect the keys; they contain everything.

## Replication and high availability

A **replica** receives a stream of changes from the **primary**:

- **Streaming / physical replication** (PostgreSQL, SQL Server Always On, MySQL): an exact copy, read-only. Use it for failover and for read-heavy reports.
- **Synchronous vs asynchronous**: synchronous means COMMIT waits until a replica has the change (no data loss on failover, slower); asynchronous is faster but the replica can lag behind, so a failover can lose the last seconds.
- **Failover** promotes a replica to primary when the primary dies; tools like Patroni or cloud services (RDS, Azure SQL) automate it.
- **Logical replication** sends row changes (not disk blocks), so it can replicate some tables, across versions, or to other systems: the basis of upgrades with near-zero downtime and of change data capture (CDC).

Replication is **not** a backup: a `DROP TABLE` replicates in milliseconds.

## Schema migrations

Application schemas change with every release. Treat schema changes like code:

- Each change is a numbered **migration file** in version control (`0007_add_ticket_sla.sql`), applied in order by a tool (Flyway, Liquibase, Alembic, EF Core migrations, golang-migrate, sqlx).
- The database records which migrations ran (a `schema_migrations` table; SQLite also has `PRAGMA user_version`).
- Migrations are **idempotent where possible** (`CREATE TABLE IF NOT EXISTS`) and run inside a transaction when the database supports transactional DDL (PostgreSQL and SQLite do; MySQL doesn't).
- Never edit a migration that has run in production; write a new one.

## Zero-downtime changes: expand and contract

Renaming a column while old app servers still use the old name breaks them. Do it in steps:

1. **Expand**: add the new column (nullable, or with a default). Deploy code that **writes both** columns.
2. **Backfill**: copy old values to the new column in batches (avoid one giant UPDATE that locks the table).
3. **Switch reads** to the new column. Deploy.
4. **Contract**: stop writing the old column, then drop it in a later release.

Other risky operations: adding a NOT NULL column without a default to a big table (rewrites it in some databases), creating an index without `CONCURRENTLY` (PostgreSQL locks writes), and long-running migrations holding locks. Set a `lock_timeout` so a migration fails fast instead of blocking the whole app.

## Health checks and maintenance

| Check | PostgreSQL | SQLite |
|---|---|---|
| corruption | `amcheck`, checksums | `PRAGMA integrity_check`, `PRAGMA quick_check` |
| orphaned rows | FK constraints prevent them | `PRAGMA foreign_key_check` |
| bloat / free space | `VACUUM`, autovacuum, `pg_stat_user_tables` | `VACUUM`, `PRAGMA freelist_count` |
| statistics | `ANALYZE` | `ANALYZE`, `PRAGMA optimize` |
| slow queries | `pg_stat_statements` | your app's logs |
| connections | `pg_stat_activity` | – |
| replication lag | `pg_stat_replication` | – |
| disk space | monitoring | monitoring |

`PRAGMA foreign_key_check` is a table-valued function too, so it can be queried:

```sql
SELECT * FROM pragma_foreign_key_check;
```

(In our company database it returns nothing: every reference is valid.)

Monitor the basics with alerts: disk usage, connection count, replication lag, long-running transactions, failed backups and certificate expiry. Most outages are a full disk.

@@quiz
A developer ran `DELETE FROM orders` without WHERE at 14:03. Which backup type lets you restore to 14:02:59?
- A weekly logical dump
- A replica
* Point-in-time recovery from a base backup plus transaction logs
- A storage snapshot from midnight
? PITR replays the logs up to the moment you choose.

@@quiz
Why isn't a replica a backup?
* Mistakes like DROP TABLE are replicated to it almost immediately
- Replicas are read-only
- Replicas are slower
- Replicas don't store indexes
? Replication protects against hardware failure, not against bad changes.

@@quiz
What's the safe way to rename a column on a busy system?
- ALTER TABLE ... RENAME during lunch
* Expand and contract: add the new column, write both, backfill, switch reads, then drop the old one later
- Drop and recreate the table
- Rename it in the application only
? Each step is compatible with the code running before and after it.

@@quiz
What do RPO and RTO measure?
- Read and write performance
* How much data you can afford to lose, and how long you can afford to be down
- Replica count and table count
- Rows per operation and time per row
? They drive the backup and HA design.

@@drill
What's the name of the zero-downtime schema change pattern of adding the new form, migrating, then removing the old?
= expand and contract
= expand/contract
= expand-contract
? Old and new code both work during the change.

@@flashcards
logical backup :: SQL dump: portable, slow to restore
physical backup :: copy of data files
PITR :: base backup + logs: restore to any moment
3-2-1 rule :: 3 copies, 2 media, 1 off-site
RPO :: maximum acceptable data loss
RTO :: maximum acceptable downtime
`VACUUM INTO 'file'` :: consistent SQLite backup
synchronous replication :: commit waits for the replica
migration :: numbered, versioned schema change
`PRAGMA user_version` :: SQLite schema version number
expand / contract :: zero-downtime schema change
`PRAGMA integrity_check` :: SQLite corruption check
`pragma_foreign_key_check` :: find orphaned rows

@@exercise An idempotent migration runner
Write migrations 1–3 so that running the **whole script twice** gives the same result, recording each one in `schema_migrations(version, name)` with `INSERT OR IGNORE`:

1. `create assets`: `assets(id INTEGER PRIMARY KEY, tag TEXT NOT NULL UNIQUE)`
2. `add asset locations`: a new table `asset_locations(tag TEXT PRIMARY KEY REFERENCES assets(tag), location TEXT)`. (Adding a column with ALTER TABLE isn't idempotent in plain SQLite, because there's no `ADD COLUMN IF NOT EXISTS`; migration tools solve that by checking `schema_migrations` in code. A new table is an additive change that *can* be written idempotently.)
3. `seed locations`: asset `LT-0042` located in `Tokyo`, inserted only if missing

Then run all three a second time, and show: the migrations table, the columns of both tables (`tbl`, `name` from `pragma_table_info`), and the seeded asset with its location and the total `asset_rows` (which must still be 1).
@@setup
CREATE TABLE schema_migrations (version INTEGER PRIMARY KEY, name TEXT NOT NULL);
@@starter
-- Migration 1
CREATE TABLE IF NOT EXISTS assets (id INTEGER PRIMARY KEY, tag TEXT NOT NULL UNIQUE);
INSERT OR IGNORE INTO schema_migrations VALUES (1, 'create assets');
-- TODO: migrations 2 and 3, then run all three again

SELECT version, name FROM schema_migrations ORDER BY version;
SELECT name FROM pragma_table_info('assets') ORDER BY cid;
@@expected
version | name
--------+--------------------
1       | create assets
2       | add asset locations
3       | seed locations

tbl             | name
----------------+---------
assets          | id
assets          | tag
asset_locations | tag
asset_locations | location

tag     | location | asset_rows
--------+----------+-----------
LT-0042 | Tokyo    | 1
@@requires
schema_migrations
INSERT OR IGNORE
pragma_table_info
@@hint
Each migration is idempotent on its own: `CREATE TABLE IF NOT EXISTS ...`, `INSERT OR IGNORE INTO assets (tag) VALUES ('LT-0042')`, `INSERT OR IGNORE INTO asset_locations VALUES ('LT-0042', 'Tokyo')`, followed by `INSERT OR IGNORE INTO schema_migrations VALUES (n, 'name')`. Copy the whole block a second time for the re-run. Columns: `SELECT 'assets' AS tbl, name FROM pragma_table_info('assets') UNION ALL ...`.
@@solution
-- Run 1
CREATE TABLE IF NOT EXISTS assets (id INTEGER PRIMARY KEY, tag TEXT NOT NULL UNIQUE);
INSERT OR IGNORE INTO schema_migrations VALUES (1, 'create assets');
CREATE TABLE IF NOT EXISTS asset_locations (tag TEXT PRIMARY KEY REFERENCES assets(tag), location TEXT);
INSERT OR IGNORE INTO schema_migrations VALUES (2, 'add asset locations');
INSERT OR IGNORE INTO assets (tag) VALUES ('LT-0042');
INSERT OR IGNORE INTO asset_locations VALUES ('LT-0042', 'Tokyo');
INSERT OR IGNORE INTO schema_migrations VALUES (3, 'seed locations');
-- Run 2 (nothing changes)
CREATE TABLE IF NOT EXISTS assets (id INTEGER PRIMARY KEY, tag TEXT NOT NULL UNIQUE);
INSERT OR IGNORE INTO schema_migrations VALUES (1, 'create assets');
CREATE TABLE IF NOT EXISTS asset_locations (tag TEXT PRIMARY KEY REFERENCES assets(tag), location TEXT);
INSERT OR IGNORE INTO schema_migrations VALUES (2, 'add asset locations');
INSERT OR IGNORE INTO assets (tag) VALUES ('LT-0042');
INSERT OR IGNORE INTO asset_locations VALUES ('LT-0042', 'Tokyo');
INSERT OR IGNORE INTO schema_migrations VALUES (3, 'seed locations');

SELECT version, name FROM schema_migrations ORDER BY version;
SELECT 'assets' AS tbl, name FROM pragma_table_info('assets')
UNION ALL
SELECT 'asset_locations', name FROM pragma_table_info('asset_locations');
SELECT a.tag, l.location, (SELECT COUNT(*) FROM assets) AS asset_rows
FROM assets a JOIN asset_locations l ON l.tag = a.tag;

@@exercise Expand and contract a column rename
`devices.owner` (a free-text name) must become `owner_username`. Do the **expand** steps: add `owner_username`, add a trigger so new writes to `owner` also fill `owner_username` (looked up from `people`), and backfill existing rows. Then simulate old code inserting a device with only `owner`, and show all devices.
@@setup
CREATE TABLE people (username TEXT PRIMARY KEY, name TEXT NOT NULL);
INSERT INTO people VALUES ('asilva', 'Ana Silva'), ('mlee', 'Mia Lee'), ('bjones', 'Ben Jones');
CREATE TABLE devices (id INTEGER PRIMARY KEY, tag TEXT NOT NULL, owner TEXT);
INSERT INTO devices (tag, owner) VALUES ('LT-0042', 'Ana Silva'), ('LT-0043', 'Mia Lee'), ('PR-0003', NULL);
@@starter
-- 1. expand: ALTER TABLE ... ADD COLUMN
-- 2. keep both in sync for old code: CREATE TRIGGER ...
-- 3. backfill: UPDATE ...
INSERT INTO devices (tag, owner) VALUES ('PH-0102', 'Ben Jones');   -- old code
SELECT id, tag, owner, owner_username FROM devices ORDER BY id;
@@expected
id | tag     | owner     | owner_username
---+---------+-----------+---------------
1  | LT-0042 | Ana Silva | asilva
2  | LT-0043 | Mia Lee   | mlee
3  | PR-0003 | NULL      | NULL
4  | PH-0102 | Ben Jones | bjones
@@requires
ADD COLUMN owner_username
CREATE TRIGGER
UPDATE devices
@@hint
`ALTER TABLE devices ADD COLUMN owner_username TEXT REFERENCES people(username);` Trigger: `AFTER INSERT ON devices WHEN NEW.owner_username IS NULL AND NEW.owner IS NOT NULL BEGIN UPDATE devices SET owner_username = (SELECT username FROM people WHERE name = NEW.owner) WHERE id = NEW.id; END;` (add a matching AFTER UPDATE OF owner trigger in real life). Backfill: `UPDATE devices SET owner_username = (SELECT username FROM people WHERE name = devices.owner) WHERE owner_username IS NULL;`
@@solution
ALTER TABLE devices ADD COLUMN owner_username TEXT REFERENCES people(username);
CREATE TRIGGER devices_owner_sync AFTER INSERT ON devices
WHEN NEW.owner_username IS NULL AND NEW.owner IS NOT NULL
BEGIN
    UPDATE devices SET owner_username = (SELECT username FROM people WHERE name = NEW.owner) WHERE id = NEW.id;
END;
UPDATE devices SET owner_username = (SELECT username FROM people WHERE name = devices.owner)
WHERE owner_username IS NULL;
INSERT INTO devices (tag, owner) VALUES ('PH-0102', 'Ben Jones');   -- old code
SELECT id, tag, owner, owner_username FROM devices ORDER BY id;

@@exercise A database health check
Someone loaded data with foreign keys switched off. Run a health report on the starter database: `integrity` (the result of `PRAGMA integrity_check`, via `pragma_integrity_check`), the number of `orphans` found by `pragma_foreign_key_check`, and then list each orphan as `table_name`, `rowid` and `missing_parent` (the parent table). Sort the list by table and rowid.
@@setup
CREATE TABLE departments (id INTEGER PRIMARY KEY, name TEXT);
CREATE TABLE employees (id INTEGER PRIMARY KEY, name TEXT, dept_id INTEGER REFERENCES departments(id));
CREATE TABLE devices (id INTEGER PRIMARY KEY, tag TEXT, employee_id INTEGER REFERENCES employees(id));
INSERT INTO departments VALUES (1, 'IT'), (2, 'Sales');
INSERT INTO employees VALUES (1, 'Ana', 1), (2, 'Ben', 2), (3, 'Cho', 9);
INSERT INTO devices VALUES (1, 'LT-1', 1), (2, 'LT-2', 7), (3, 'LT-3', NULL), (4, 'LT-4', 8);
@@starter
SELECT COUNT(*) FROM employees;
@@expected
integrity | orphans
----------+--------
ok        | 3

table_name | rowid | missing_parent
-----------+-------+---------------
devices    | 2     | employees
devices    | 4     | employees
employees  | 3     | departments
@@requires
pragma_integrity_check
pragma_foreign_key_check
@@hint
`SELECT (SELECT integrity_check FROM pragma_integrity_check) AS integrity, (SELECT COUNT(*) FROM pragma_foreign_key_check) AS orphans;` then `SELECT "table" AS table_name, rowid, parent AS missing_parent FROM pragma_foreign_key_check ORDER BY 1, 2;` (`table` is a keyword, so quote it).
@@solution
SELECT (SELECT integrity_check FROM pragma_integrity_check) AS integrity,
       (SELECT COUNT(*) FROM pragma_foreign_key_check) AS orphans;
SELECT "table" AS table_name, rowid, parent AS missing_parent
FROM pragma_foreign_key_check
ORDER BY table_name, rowid;

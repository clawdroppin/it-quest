@@lesson n8-postgres PostgreSQL and Other Dialects
@@concepts modules
@@text
This course runs on SQLite because it needs no server. At work you'll most likely meet **PostgreSQL**, **SQL Server**, **MySQL/MariaDB** or **Oracle**. The core (SELECT, JOIN, GROUP BY, window functions, CTEs) is the same everywhere; the edges differ. This lesson is a map of those edges, with PostgreSQL in the most detail because it's the most popular open-source database for new applications.

## Same idea, different spelling

| Task | SQLite | PostgreSQL | SQL Server | MySQL |
|---|---|---|---|---|
| auto id | `INTEGER PRIMARY KEY` | `GENERATED ALWAYS AS IDENTITY` | `IDENTITY(1,1)` | `AUTO_INCREMENT` |
| first n rows | `LIMIT n` | `LIMIT n` / `FETCH FIRST n ROWS ONLY` | `TOP (n)` / `OFFSET 0 ROWS FETCH NEXT n ROWS ONLY` | `LIMIT n` |
| concat | `a \|\| b` | `a \|\| b`, `concat()` | `a + b`, `CONCAT()` | `CONCAT()` |
| now | `datetime('now')` | `now()` | `SYSUTCDATETIME()` | `NOW()` |
| if-null | `IFNULL`, `COALESCE` | `COALESCE` | `ISNULL`, `COALESCE` | `IFNULL`, `COALESCE` |
| string list | `group_concat(x, ',')` | `string_agg(x, ',' ORDER BY x)` | `STRING_AGG(x, ',') WITHIN GROUP (ORDER BY x)` | `GROUP_CONCAT(x ORDER BY x)` |
| upsert | `ON CONFLICT DO UPDATE` | `ON CONFLICT DO UPDATE`, `MERGE` | `MERGE` | `ON DUPLICATE KEY UPDATE` |
| quoting names | `"name"` | `"name"` | `[name]` | `` `name` `` |
| boolean | 0/1 | `boolean` | `bit` | `TINYINT(1)` |

`COALESCE`, `CASE`, `CAST`, standard joins, CTEs and window functions work almost everywhere. When portability matters, prefer them.

## PostgreSQL highlights

- **Types**: `uuid`, `jsonb`, `inet`/`cidr` (IP addresses with operators like `<<` "is contained in subnet"), arrays (`text[]`), ranges (`tstzrange`), `interval`, enums, and real `boolean` and `timestamptz`.
- **`RETURNING`**: get back the rows you inserted/updated/deleted, including generated ids, in the same statement (SQLite 3.35+ has it too).
- **`DISTINCT ON (cols)`**: keep the first row per group according to ORDER BY. "Latest login per user" in one line.
- **`FILTER (WHERE ...)`** on aggregates: `COUNT(*) FILTER (WHERE status = 'open')` (SQLite 3.30+ supports it).
- **`generate_series`**: rows from numbers or timestamps.
- **`LATERAL` joins**: a subquery in FROM that can refer to columns of earlier tables, like a per-row function call. "Top 3 tickets per employee" without window functions.
- **Partial, expression, GIN, GiST and BRIN indexes**, `CREATE INDEX CONCURRENTLY` (no write lock).
- **Extensions**: `pg_trgm` (fuzzy search), `pgcrypto`, `PostGIS` (maps), `pg_stat_statements`, `TimescaleDB`.
- **Schemas**: namespaces inside a database (`hr.employees`, `support.tickets`), with their own permissions.
- **`psql`** commands: `\l` databases, `\dt` tables, `\d table` describe, `\x` expanded output, `\timing`, `\copy` to import/export CSV from the client.

```sql,norun
-- Latest login per user
SELECT DISTINCT ON (username) username, ip, time
FROM logins ORDER BY username, time DESC;

-- Insert and get the id back
INSERT INTO tickets (title, priority) VALUES ('VPN down', 1) RETURNING id;

-- Every IP in a subnet
SELECT * FROM logins WHERE ip::inet << '10.0.0.0/16';
```

## SQL Server highlights

T-SQL adds procedural code (`DECLARE @x INT; IF ... BEGIN ... END`), `TOP`, `OUTPUT` (like RETURNING), `CROSS APPLY` (like LATERAL), temporal tables, `[brackets]` for names, and the `GO` batch separator in tools like SSMS. Strings are `NVARCHAR` for Unicode (prefix literals with `N'...'`, which matters for Japanese text).

## MySQL highlights

Watch the defaults: older versions silently truncated bad data unless `sql_mode` includes `STRICT_TRANS_TABLES`; use `utf8mb4`, **not** `utf8` (which can't store emoji or some kanji); InnoDB is the only engine you want. `GROUP BY` used to allow non-aggregated columns (ONLY_FULL_GROUP_BY fixes that).

## Writing portable SQL

1. Prefer standard features (`COALESCE`, `CASE`, CTEs, window functions, `FETCH FIRST` where supported).
2. Keep vendor-specific SQL in one place (a repository layer or migration files per dialect).
3. Test against the real database you'll run in production: SQLite's flexible typing hides bugs PostgreSQL would catch. Containers make that easy (`docker run postgres`).
4. Use your ORM or query builder's dialect support, but read the SQL it generates.

The exercises use SQLite features that match PostgreSQL's spelling: `FILTER`, window-based "DISTINCT ON", and ordered string aggregation.

@@quiz
Which PostgreSQL feature returns the first row per group in one clause?
- `GROUP BY FIRST`
* `DISTINCT ON (cols)` with a matching ORDER BY
- `LIMIT 1 PER GROUP`
- `TOP 1 WITH TIES`
? Elsewhere, use ROW_NUMBER() = 1 in a CTE.

@@quiz
What should MySQL tables storing Japanese text and emoji use?
- `utf8`
* `utf8mb4`
- `latin1`
- `ascii`
? MySQL's old `utf8` is limited to 3 bytes per character.

@@quiz
What does `RETURNING id` on an INSERT do?
* Returns the inserted rows' ids (or other columns) from the same statement
- Rolls back the insert
- Returns the number of rows
- Only works in SQLite
? SQL Server's equivalent is OUTPUT inserted.id.

@@quiz
Why test against PostgreSQL even if SQLite passes locally?
- SQLite is slower
* SQLite's flexible typing and different defaults can hide bugs the production database would reject
- SQLite can't run SELECT
- PostgreSQL has different JOIN syntax
? Run the real engine in a container in CI.

@@drill
Which PostgreSQL column type auto-generates IDs in modern versions? Write the clause.
= GENERATED ALWAYS AS IDENTITY
= generated by default as identity
= identity
? Preferred over the older SERIAL.

@@flashcards
`DISTINCT ON` :: first row per group (PostgreSQL)
`RETURNING` :: get changed rows back (PostgreSQL, SQLite 3.35+)
`OUTPUT` :: SQL Server's RETURNING
`FILTER (WHERE ...)` :: conditional aggregate
`string_agg(x, ',' ORDER BY x)` :: PostgreSQL string list
`LATERAL` / `CROSS APPLY` :: per-row subquery in FROM
`inet <<` :: IP in subnet (PostgreSQL)
`TOP (n)` :: SQL Server's LIMIT
`N'...'` :: Unicode literal in SQL Server
`utf8mb4` :: full Unicode in MySQL
`\d table` :: describe a table in psql

@@exercise A pivot with FILTER
Show one row per ticket `category` with columns `p1`, `p2`, `p3` and `p4`: the number of tickets of each priority, and `total`. Use the standard `COUNT(*) FILTER (WHERE ...)` syntax (it works in PostgreSQL too). Sort by category.
@@starter
SELECT category, priority, COUNT(*) FROM tickets GROUP BY category, priority;
@@expected
category | p1 | p2 | p3 | p4 | total
---------+----+----+----+----+------
account  | 0  | 1  | 2  | 1  | 4
hardware | 1  | 0  | 2  | 1  | 4
network  | 0  | 2  | 0  | 0  | 2
security | 2  | 0  | 0  | 0  | 2
software | 0  | 0  | 2  | 1  | 3
@@requires
FILTER (WHERE
@@hint
`COUNT(*) FILTER (WHERE priority = 1) AS p1`, and so on.
@@solution
SELECT category,
       COUNT(*) FILTER (WHERE priority = 1) AS p1,
       COUNT(*) FILTER (WHERE priority = 2) AS p2,
       COUNT(*) FILTER (WHERE priority = 3) AS p3,
       COUNT(*) FILTER (WHERE priority = 4) AS p4,
       COUNT(*) AS total
FROM tickets
GROUP BY category
ORDER BY category;

@@exercise DISTINCT ON without DISTINCT ON
In PostgreSQL, the latest login per user is `SELECT DISTINCT ON (username) username, ip, time FROM logins ORDER BY username, time DESC`. Write the portable version with `ROW_NUMBER()`: the latest login (successful or not) for each username, showing `username`, `ip`, `time` and `success`. Sort by username.
@@starter
SELECT username, MAX(time) FROM logins GROUP BY username ORDER BY username;
@@expected
username | ip            | time                | success
---------+---------------+---------------------+--------
admin    | 203.0.113.7   | 2026-10-29 22:31:04 | 0
asilva   | 203.0.113.7   | 2026-10-29 22:31:30 | 1
bjones   | 10.0.0.40     | 2026-10-30 09:02:44 | 1
ktanaka  | 10.0.0.10     | 2026-10-31 07:45:00 | 1
lmartin  | 10.0.1.15     | 2026-10-30 09:15:20 | 1
mlee     | 10.0.0.34     | 2026-10-31 08:50:00 | 1
oracle   | 198.51.100.23 | 2026-10-31 03:00:00 | 0
root     | 203.0.113.7   | 2026-10-29 22:31:09 | 0
test     | 198.51.100.23 | 2026-10-30 03:12:03 | 0
@@requires
ROW_NUMBER() OVER (PARTITION BY username
@@hint
`WITH ranked AS (SELECT *, ROW_NUMBER() OVER (PARTITION BY username ORDER BY time DESC) AS rn FROM logins) SELECT ... WHERE rn = 1`.
@@solution
WITH ranked AS (
    SELECT username, ip, time, success,
           ROW_NUMBER() OVER (PARTITION BY username ORDER BY time DESC) AS rn
    FROM logins
)
SELECT username, ip, time, success
FROM ranked WHERE rn = 1
ORDER BY username;

@@exercise Ordered string aggregation
For each department with employees, show `department` and `team`: the usernames joined with `, ` in **alphabetical** order (PostgreSQL: `string_agg(username, ', ' ORDER BY username)`). SQLite only added ORDER BY inside aggregates in 3.44, so use the portable trick: aggregate from a subquery that's already sorted. Sort by department.
@@starter
SELECT d.name AS department, group_concat(e.username, ', ') AS team
FROM employees e JOIN departments d ON d.id = e.dept_id
GROUP BY d.id ORDER BY department;
@@expected
department | team
-----------+-----------------------
Finance    | rchen, tyamada
IT         | asilva, ktanaka, mlee
Sales      | bjones, hsato, ykato
Support    | esuzuki, lmartin, sito
@@requires
group_concat
ORDER BY username
@@hint
`FROM (SELECT * FROM employees ORDER BY username) e JOIN departments d ...`. (Strictly, SQL doesn't promise the subquery's order survives the GROUP BY, but SQLite keeps it; in PostgreSQL use the ORDER BY inside string_agg.)
@@solution
SELECT d.name AS department, group_concat(e.username, ', ') AS team
FROM (SELECT * FROM employees ORDER BY username) e
JOIN departments d ON d.id = e.dept_id
GROUP BY d.id
ORDER BY department;

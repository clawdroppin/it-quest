@@lesson g9-etl ETL and ELT: Staging, Cleaning, Deduplication and History
@@concepts text-processing, automation
@@text
Data doesn't arrive in a warehouse clean. It comes from CSV exports, APIs and other databases, with typos, duplicates, missing values and changes over time. **ETL** (extract, transform, load) or, more common today, **ELT** (load raw data first, then transform it with SQL inside the warehouse, as tools like dbt do) is the plumbing that turns it into trustworthy tables.

## The layers

```text
raw / staging  →  cleaned / conformed  →  marts (facts and dimensions)
```

- **Staging** holds the data exactly as received, all columns as TEXT, plus load metadata (`loaded_at`, `source_file`). Never edit it: if a transform has a bug, you can rerun it from staging.
- **Cleaned** tables fix types, trim and standardise values, deduplicate and validate.
- **Marts** are the star schemas reports read.

Each layer is a set of SQL queries (views or tables) you can rerun. That's what makes ELT pipelines testable and repeatable.

## Cleaning patterns

| Problem | SQL fix |
|---|---|
| stray spaces, case | `trim(lower(email))` |
| empty strings that mean "missing" | `NULLIF(trim(x), '')` |
| inconsistent codes (`TKY`, `Tokyo`, `tokyo `) | a mapping table joined on the cleaned value |
| numbers as text | `CAST`, after checking with `GLOB '[0-9]*'` or `typeof` |
| several date formats | `CASE` on the pattern, rebuild as ISO `YYYY-MM-DD` |
| invalid rows | route them to a **rejects** table with a reason, don't silently drop them |

## Deduplication

Duplicates come from re-sent files, retries and merged systems. Decide the **business key** (what makes two rows "the same thing": employee number, asset tag, email) and which copy wins (the latest `loaded_at`, the most complete row). Then keep one row per key with `ROW_NUMBER()`:

```sql,norun
WITH ranked AS (
    SELECT *, ROW_NUMBER() OVER (PARTITION BY asset_tag ORDER BY loaded_at DESC) AS rn
    FROM stg_assets
)
SELECT * FROM ranked WHERE rn = 1;
```

## Incremental loads

Reloading everything every night stops scaling. Incremental loads process only what's new or changed, using a **watermark** (the highest `updated_at` already loaded) and an UPSERT (`INSERT ... ON CONFLICT DO UPDATE`, or `MERGE` in PostgreSQL 15+/SQL Server/Snowflake). Make loads **idempotent**: running the same batch twice must not create duplicates.

## Slowly changing dimensions (SCD)

Ana moves from IT to Sales in 2027. Should her 2026 tickets now show up under Sales? Usually not. Dimension changes are handled with one of the classic types:

| Type | What happens | History |
|---|---|---|
| **SCD 1** | overwrite the old value | lost; reports always use the current value |
| **SCD 2** | close the old row (`valid_to`), insert a new row with a new surrogate key | kept: old facts point at the old version |
| SCD 3 | keep a "previous value" column | one step only |

SCD type 2 rows carry `valid_from`, `valid_to` (`9999-12-31` for the current row) and often `is_current`. Facts are linked to the version valid **when the fact happened**, so history reports stay correct.

## Data quality tests

Pipelines should fail loudly instead of publishing wrong numbers. Typical tests (dbt calls them `unique`, `not_null`, `accepted_values`, `relationships`) are just queries that must return **zero rows**:

```sql
SELECT 'duplicate username' AS problem, username FROM employees GROUP BY username HAVING COUNT(*) > 1
UNION ALL
SELECT 'bad priority', CAST(id AS TEXT) FROM tickets WHERE priority NOT BETWEEN 1 AND 4
UNION ALL
SELECT 'orphan device owner', asset_tag FROM devices d
WHERE employee_id IS NOT NULL AND NOT EXISTS (SELECT 1 FROM employees e WHERE e.id = d.employee_id);
```

Also watch **volume** (did today's load have 90% fewer rows than usual?) and **freshness** (when was the latest row loaded?).

@@quiz
Why load raw data into a staging table unchanged before transforming it?
* You can rerun or fix transformations from the original data without asking the source again
- It's faster to query
- Staging tables have better indexes
- It's required by SQL
? Staging is the replayable source of truth for the pipeline.

@@quiz
With SCD type 2, what happens when an employee moves department?
- The department is overwritten
* The current row is closed with a valid_to date and a new row with a new key is inserted
- The employee is deleted
- A previous_department column is filled
? Old facts keep pointing at the old version.

@@quiz
What makes a load idempotent?
- It runs once a day
* Running the same batch twice leaves the same result, with no duplicates
- It uses indexes
- It deletes everything first
? UPSERTs on a business key are the usual tool.

@@flashcards
ETL :: transform before loading
ELT :: load raw, transform in the warehouse with SQL
staging :: raw data as received, plus load metadata
business key :: what identifies "the same thing"
`ROW_NUMBER() ... rn = 1` :: deduplicate
watermark :: highest timestamp already loaded
`MERGE` :: standard upsert statement
SCD 1 :: overwrite
SCD 2 :: new row per version, valid_from / valid_to
rejects table :: invalid rows with a reason
data test :: a query that must return zero rows

@@exercise Clean an HR export
`stg_people` is a raw CSV load. Produce the cleaned rows: `emp_no` (trimmed), `email` (trimmed, lowercase), `office` mapped through `office_map` (unknown codes become `Unknown`), and `hired` converted to ISO `YYYY-MM-DD` from either `YYYY-MM-DD` or `DD/MM/YYYY`. Keep only the **latest** load per `emp_no` (by `loaded_at`). Sort by emp_no.
@@setup
CREATE TABLE stg_people (emp_no TEXT, email TEXT, office TEXT, hired TEXT, loaded_at TEXT);
INSERT INTO stg_people VALUES
  (' E001', 'Ana.Silva@Sakura.example ', 'TKY', '2019-10-01', '2026-10-01 02:00'),
  ('E002', 'mia.lee@sakura.example', 'tokyo ', '01/04/2023', '2026-10-01 02:00'),
  ('E003', ' SARA.ITO@sakura.example', 'OSA', '01/09/2018', '2026-10-01 02:00'),
  ('E002', 'mia.lee@sakura.example', 'Osaka', '01/04/2023', '2026-10-15 02:00'),
  ('E004', 'omar.h@contractor.example', 'REMOTE', '2026-09-01', '2026-10-15 02:00');
CREATE TABLE office_map (code TEXT PRIMARY KEY, office TEXT);
INSERT INTO office_map VALUES ('tky', 'Tokyo'), ('tokyo', 'Tokyo'), ('osa', 'Osaka'), ('osaka', 'Osaka'), ('fuk', 'Fukuoka');
@@starter
SELECT emp_no, email, office, hired FROM stg_people ORDER BY emp_no;
@@expected
emp_no | email                     | office  | hired
-------+---------------------------+---------+-----------
E001   | ana.silva@sakura.example  | Tokyo   | 2019-10-01
E002   | mia.lee@sakura.example    | Osaka   | 2023-04-01
E003   | sara.ito@sakura.example   | Osaka   | 2018-09-01
E004   | omar.h@contractor.example | Unknown | 2026-09-01
@@requires
ROW_NUMBER()
LEFT JOIN office_map
CASE
@@hint
CTE `ranked` with `trim(emp_no) AS emp_no` and `ROW_NUMBER() OVER (PARTITION BY trim(emp_no) ORDER BY loaded_at DESC) AS rn`. Office: `LEFT JOIN office_map m ON m.code = lower(trim(office))`, `COALESCE(m.office, 'Unknown')`. Date: `CASE WHEN hired LIKE '__/__/____' THEN substr(hired, 7, 4) || '-' || substr(hired, 4, 2) || '-' || substr(hired, 1, 2) ELSE hired END`.
@@solution
WITH ranked AS (
    SELECT trim(emp_no) AS emp_no, email, office, hired,
           ROW_NUMBER() OVER (PARTITION BY trim(emp_no) ORDER BY loaded_at DESC) AS rn
    FROM stg_people
)
SELECT r.emp_no,
       lower(trim(r.email)) AS email,
       COALESCE(m.office, 'Unknown') AS office,
       CASE WHEN r.hired LIKE '__/__/____'
            THEN substr(r.hired, 7, 4) || '-' || substr(r.hired, 4, 2) || '-' || substr(r.hired, 1, 2)
            ELSE r.hired END AS hired
FROM ranked r
LEFT JOIN office_map m ON m.code = lower(trim(r.office))
WHERE r.rn = 1
ORDER BY r.emp_no;

@@exercise SCD type 2
Ana Silva moves from IT to Sales on `2027-01-01`. `dim_employee` is an SCD 2 dimension. Apply the change: close her current row (`valid_to = '2026-12-31'`, `is_current = 0`) and insert a new current row with a new surrogate key (the next key), department `Sales`, `valid_from = '2027-01-01'`, `valid_to = '9999-12-31'`. Then attribute each fact in `fact_ticket` to the department **valid on the ticket date** and count tickets per department. Show the dimension, then the counts.
@@setup
CREATE TABLE dim_employee (employee_key INTEGER PRIMARY KEY, emp_no TEXT, name TEXT, department TEXT, valid_from TEXT, valid_to TEXT, is_current INTEGER);
INSERT INTO dim_employee VALUES
  (1, 'E001', 'Ana Silva', 'IT', '2019-10-01', '9999-12-31', 1),
  (2, 'E002', 'Mia Lee', 'IT', '2023-04-01', '9999-12-31', 1);
CREATE TABLE fact_ticket (ticket_id INTEGER, emp_no TEXT, opened TEXT);
INSERT INTO fact_ticket VALUES (1, 'E001', '2026-10-03'), (2, 'E001', '2026-11-20'), (3, 'E002', '2026-12-01'), (4, 'E001', '2027-01-15'), (5, 'E001', '2027-02-02');
@@starter
UPDATE dim_employee SET department = 'Sales' WHERE emp_no = 'E001';   -- SCD 1: history lost!
SELECT * FROM dim_employee ORDER BY employee_key;
@@expected
employee_key | emp_no | name      | department | valid_from | valid_to   | is_current
-------------+--------+-----------+------------+------------+------------+-----------
1            | E001   | Ana Silva | IT         | 2019-10-01 | 2026-12-31 | 0
2            | E002   | Mia Lee   | IT         | 2023-04-01 | 9999-12-31 | 1
3            | E001   | Ana Silva | Sales      | 2027-01-01 | 9999-12-31 | 1

department | tickets
-----------+--------
IT         | 3
Sales      | 2
@@requires
valid_to = '2026-12-31'
BETWEEN
@@hint
`UPDATE dim_employee SET valid_to = '2026-12-31', is_current = 0 WHERE emp_no = 'E001' AND is_current = 1;` then `INSERT INTO dim_employee SELECT (SELECT MAX(employee_key) + 1 FROM dim_employee), 'E001', 'Ana Silva', 'Sales', '2027-01-01', '9999-12-31', 1;`. Report: `JOIN dim_employee d ON d.emp_no = f.emp_no AND f.opened BETWEEN d.valid_from AND d.valid_to`.
@@solution
UPDATE dim_employee SET valid_to = '2026-12-31', is_current = 0 WHERE emp_no = 'E001' AND is_current = 1;
INSERT INTO dim_employee
SELECT (SELECT MAX(employee_key) + 1 FROM dim_employee), 'E001', 'Ana Silva', 'Sales', '2027-01-01', '9999-12-31', 1;

SELECT * FROM dim_employee ORDER BY employee_key;

SELECT d.department, COUNT(*) AS tickets
FROM fact_ticket f
JOIN dim_employee d ON d.emp_no = f.emp_no AND f.opened BETWEEN d.valid_from AND d.valid_to
GROUP BY d.department
ORDER BY d.department;

@@exercise Data quality gate
Write the pipeline's quality gate for the company database as **one** query returning `test` and `failures` (a count, 0 = pass) for these tests, in this order: `unique usernames`, `ticket priority 1-4`, `closed tickets have a closed time` (status `closed` or `resolved` must have `closed`), `device owners exist`, `no future hires` (hired after `2026-11-01`), and `open tickets older than 30 days` (opened before `2026-10-02` and still open or in progress). Then a final row `gate` with `PASS` or `FAIL` (FAIL when any test fails).
@@starter
SELECT 'unique usernames' AS test, COUNT(*) - COUNT(DISTINCT username) AS failures FROM employees;
@@expected
test                              | failures
----------------------------------+---------
unique usernames                  | 0
ticket priority 1-4               | 0
closed tickets have a closed time | 0
device owners exist               | 0
no future hires                   | 0
open tickets older than 30 days   | 0
gate                              | PASS
@@requires
UNION ALL
COUNT(DISTINCT
@@hint
Put the tests in a CTE `t(n, test, failures)` built with UNION ALL, one SELECT per test, with a number for ordering. Then `SELECT test, failures FROM t UNION ALL SELECT 'gate', CASE WHEN SUM(failures) > 0 THEN 'FAIL' ELSE 'PASS' END FROM t` (failures becomes text in that last row; that's fine in SQLite). Order by the number with the gate last.
@@solution
WITH t(n, test, failures) AS (
    SELECT 1, 'unique usernames', COUNT(*) - COUNT(DISTINCT username) FROM employees
    UNION ALL
    SELECT 2, 'ticket priority 1-4', COUNT(*) FROM tickets WHERE priority NOT BETWEEN 1 AND 4
    UNION ALL
    SELECT 3, 'closed tickets have a closed time', COUNT(*) FROM tickets WHERE status IN ('closed', 'resolved') AND closed IS NULL
    UNION ALL
    SELECT 4, 'device owners exist', COUNT(*) FROM devices d
    WHERE d.employee_id IS NOT NULL AND NOT EXISTS (SELECT 1 FROM employees e WHERE e.id = d.employee_id)
    UNION ALL
    SELECT 5, 'no future hires', COUNT(*) FROM employees WHERE hired > '2026-11-01'
    UNION ALL
    SELECT 6, 'open tickets older than 30 days', COUNT(*) FROM tickets
    WHERE status IN ('open', 'in-progress') AND opened < '2026-10-02'
)
SELECT test, failures FROM (
    SELECT n, test, failures FROM t
    UNION ALL
    SELECT 99, 'gate', CASE WHEN SUM(failures) > 0 THEN 'FAIL' ELSE 'PASS' END FROM t
)
ORDER BY n;

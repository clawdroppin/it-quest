@@lesson g9-star Data Warehousing: Star Schemas, Facts and Dimensions
@@concepts maps
@@text
The help desk application's database is designed for **transactions** (OLTP): many small reads and writes, normalised tables, no duplicated data. Reporting across years of history wants something else (**OLAP**): few, huge, read-only queries that scan and aggregate. Running those on the production database slows the application down and fights its design. So companies copy the data into a **data warehouse** (Snowflake, BigQuery, Redshift, Azure Synapse/Fabric, or plain PostgreSQL / DuckDB) shaped for analysis.

## The star schema

The classic analytical model (Ralph Kimball's dimensional modelling) puts one **fact table** in the middle, surrounded by **dimension tables**:

```text
               dim_date
                   │
dim_employee ── fact_ticket ── dim_category
                   │
              dim_priority
```

- A **fact** is a measurable event: one ticket, one sale, one login. Fact tables are long (millions of rows) and narrow: foreign keys to dimensions plus numeric **measures** (minutes to resolve, amount, quantity).
- A **dimension** describes the context: who, what, where, when. Dimensions are short and wide, full of descriptive text attributes you filter and group by (department, location, category name, month name, fiscal quarter).

Every report becomes the same shape: join the fact to the dimensions you need, filter on dimension attributes, group by dimension attributes, aggregate measures.

## The grain

The most important design decision is the fact table's **grain**: what does one row mean? "One row per ticket", "one row per ticket per status change", "one row per device per day". Decide it first, write it down, and never mix grains in one table: summing a table where some rows are tickets and some are days gives nonsense.

## The date dimension

Every warehouse has a `dim_date` with one row per day and dozens of pre-computed columns: year, quarter, month name, week, weekday, is_weekend, is_holiday, fiscal year, fiscal quarter. Reports then never calculate dates; they just group by `d.fiscal_quarter` or filter on `d.is_business_day`. A date key like `20261031` (integer) is the usual primary key.

## Surrogate keys

Dimensions get their own integer key (`employee_key`) instead of reusing the source system's id. That allows several versions of the same employee (see slowly changing dimensions in the next lesson), combining several source systems, and a special "Unknown" row (key 0 or -1) so facts with a missing value still join.

## Star vs snowflake

A **snowflake** schema normalises dimensions further (employee → department → location as separate tables). It saves a little space but needs more joins; for analytics, the flat star is usually preferred. Storage is cheap; analyst time isn't.

## Columnar storage

Warehouses store data **by column**, not by row. A query that sums one measure over 100 million rows reads only that column, compressed. That's why `SELECT *` is expensive in a warehouse and why wide fact tables are fine.

## Additive measures

- **Additive** measures can be summed across every dimension: amount, quantity, minutes.
- **Semi-additive**: can be summed across some dimensions but not time: account balance, stock level, headcount (sum per department is fine; summing daily headcounts over a month is not, use the average or the last value).
- **Non-additive**: ratios and percentages. Store the numerator and denominator; compute the ratio after aggregating.

@@quiz
What's the difference between a fact and a dimension table?
* Facts record measurable events with numeric measures; dimensions describe the context you filter and group by
- Facts are small; dimensions are large
- Facts contain text; dimensions contain numbers
- There is no difference
? Facts are long and narrow, dimensions short and wide.

@@quiz
Why decide a fact table's grain first?
- To choose the database
* Every row must mean the same thing, or aggregates mix incompatible rows
- Because indexes depend on it
- To name the table
? "One row per ticket" is a grain.

@@quiz
Why shouldn't you sum a daily "open tickets" snapshot over a month?
- It's too slow
* It's semi-additive: summing over time counts the same tickets many times; use an average or the last value
- SQLite can't
- Snapshots have no numbers
? Stock levels and balances behave the same way.

@@flashcards
OLTP :: transactional workload: small reads/writes, normalised
OLAP :: analytical workload: big scans and aggregates
fact table :: events with measures and dimension keys
dimension table :: descriptive context (who, what, when)
grain :: what one fact row means
star schema :: fact in the middle, flat dimensions around it
snowflake schema :: normalised dimensions
date dimension :: one row per day with calendar attributes
surrogate key :: warehouse-owned integer key
semi-additive :: can't be summed over time
columnar storage :: stores data by column

@@exercise Build a date dimension
Create `dim_date` for **October 2026** with a recursive CTE: `date_key` (integer `YYYYMMDD`), `day`, `weekday` (`Mon`..`Sun`), `is_weekend` (0/1), `week` (`strftime('%W')` as an integer) and `fiscal_quarter` (`FY2026-Q3` for October). Then show the number of days and weekend days per week, sorted by week.
@@starter
SELECT date('2026-10-01');
@@expected
week | days | weekend_days | fiscal_quarter
-----+------+--------------+---------------
39   | 4    | 2            | FY2026-Q3
40   | 7    | 2            | FY2026-Q3
41   | 7    | 2            | FY2026-Q3
42   | 7    | 2            | FY2026-Q3
43   | 6    | 1            | FY2026-Q3
@@requires
CREATE TABLE dim_date
WITH RECURSIVE
@@hint
`CREATE TABLE dim_date AS WITH RECURSIVE d(day) AS (...) SELECT CAST(strftime('%Y%m%d', day) AS INTEGER) AS date_key, day, substr('SunMonTueWedThuFriSat', 1 + 3 * CAST(strftime('%w', day) AS INTEGER), 3) AS weekday, ...`. Then `SELECT week, COUNT(*) AS days, SUM(is_weekend) AS weekend_days FROM dim_date GROUP BY week`.
@@solution
CREATE TABLE dim_date AS
WITH RECURSIVE d(day) AS (
    SELECT '2026-10-01'
    UNION ALL
    SELECT date(day, '+1 day') FROM d WHERE day < '2026-10-31'
)
SELECT CAST(strftime('%Y%m%d', day) AS INTEGER) AS date_key,
       day,
       substr('SunMonTueWedThuFriSat', 1 + 3 * CAST(strftime('%w', day) AS INTEGER), 3) AS weekday,
       strftime('%w', day) IN ('0', '6') AS is_weekend,
       CAST(strftime('%W', day) AS INTEGER) AS week,
       'FY' || (CAST(strftime('%Y', day) AS INTEGER) - (CAST(strftime('%m', day) AS INTEGER) < 4))
           || '-Q' || ((CAST(strftime('%m', day) AS INTEGER) + 8) % 12 / 3 + 1) AS fiscal_quarter
FROM d;

SELECT week, COUNT(*) AS days, SUM(is_weekend) AS weekend_days, MIN(fiscal_quarter) AS fiscal_quarter
FROM dim_date GROUP BY week ORDER BY week;

@@exercise A ticket star schema
Build a small star from the company database: `dim_employee(employee_key, name, department, location)` with a key-0 row `('Unknown', 'Unknown', 'Unknown')`, and `fact_ticket(ticket_id, requester_key, opened_date_key, priority, category, minutes_to_close)` at the grain **one row per ticket** (requester_key 0 when the requester has no department; minutes NULL for open tickets). Then answer from the star only: tickets and average minutes to close per **location**, sorted by location.
@@starter
SELECT COUNT(*) FROM tickets;
@@expected
location | tickets | avg_minutes
---------+---------+------------
Osaka    | 8       | 346
Tokyo    | 6       | 2059
Unknown  | 1       | NULL
@@requires
CREATE TABLE dim_employee
CREATE TABLE fact_ticket
JOIN dim_employee
@@hint
`CREATE TABLE dim_employee AS SELECT 0 AS employee_key, 'Unknown' AS name, 'Unknown' AS department, 'Unknown' AS location UNION ALL SELECT e.id, e.name, d.name, d.location FROM employees e JOIN departments d ON d.id = e.dept_id;`. Fact: `COALESCE((SELECT employee_key FROM dim_employee WHERE employee_key = t.requester_id), 0)`. Report: `JOIN dim_employee de ON de.employee_key = f.requester_key GROUP BY de.location`, `CAST(AVG(minutes_to_close) AS INTEGER)`.
@@solution
CREATE TABLE dim_employee AS
SELECT 0 AS employee_key, 'Unknown' AS name, 'Unknown' AS department, 'Unknown' AS location
UNION ALL
SELECT e.id, e.name, d.name, d.location FROM employees e JOIN departments d ON d.id = e.dept_id;

CREATE TABLE fact_ticket AS
SELECT t.id AS ticket_id,
       COALESCE((SELECT employee_key FROM dim_employee WHERE employee_key = t.requester_id), 0) AS requester_key,
       CAST(strftime('%Y%m%d', t.opened) AS INTEGER) AS opened_date_key,
       t.priority, t.category,
       CAST(ROUND((julianday(t.closed) - julianday(t.opened)) * 1440) AS INTEGER) AS minutes_to_close
FROM tickets t;

SELECT de.location, COUNT(*) AS tickets, CAST(AVG(f.minutes_to_close) AS INTEGER) AS avg_minutes
FROM fact_ticket f
JOIN dim_employee de ON de.employee_key = f.requester_key
GROUP BY de.location
ORDER BY de.location;

@@exercise Semi-additive snapshots
`fact_open_daily(day, team, open_tickets)` is a daily snapshot. For each team, show the `avg_open` (2 decimals), the `last_open` (value on the latest day), and the wrong `naive_sum` (the sum over days) to see why snapshots aren't additive over time. Sort by team.
@@setup
CREATE TABLE fact_open_daily (day TEXT, team TEXT, open_tickets INTEGER);
INSERT INTO fact_open_daily VALUES
  ('2026-10-27', 'Help Desk', 4), ('2026-10-28', 'Help Desk', 5), ('2026-10-29', 'Help Desk', 5), ('2026-10-30', 'Help Desk', 7), ('2026-10-31', 'Help Desk', 6),
  ('2026-10-27', 'Infra', 2), ('2026-10-28', 'Infra', 2), ('2026-10-29', 'Infra', 3), ('2026-10-30', 'Infra', 3), ('2026-10-31', 'Infra', 1);
@@starter
SELECT team, SUM(open_tickets) AS naive_sum FROM fact_open_daily GROUP BY team ORDER BY team;
@@expected
team      | avg_open | last_open | naive_sum
----------+----------+-----------+----------
Help Desk | 5.4      | 6         | 27
Infra     | 2.2      | 1         | 11
@@requires
AVG(
MAX(day)
@@hint
`ROUND(AVG(open_tickets), 2)`; last value: a correlated subquery `(SELECT open_tickets FROM fact_open_daily x WHERE x.team = f.team AND x.day = (SELECT MAX(day) FROM fact_open_daily y WHERE y.team = f.team))`.
@@solution
SELECT f.team,
       ROUND(AVG(f.open_tickets), 2) AS avg_open,
       (SELECT x.open_tickets FROM fact_open_daily x
        WHERE x.team = f.team AND x.day = (SELECT MAX(day) FROM fact_open_daily y WHERE y.team = f.team)) AS last_open,
       SUM(f.open_tickets) AS naive_sum
FROM fact_open_daily f
GROUP BY f.team
ORDER BY f.team;

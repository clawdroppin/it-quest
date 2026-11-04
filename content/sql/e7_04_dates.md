@@lesson e7-dates Dates, Times and Time Zones in Depth
@@concepts numbers, strings
@@text
Date bugs are expensive: SLA reports that are off by an hour twice a year, invoices dated in the wrong month, logs that can't be correlated across servers. This lesson covers how databases store time and how to calculate with it correctly.

## Storing time

| Database | Types |
|---|---|
| PostgreSQL | `date`, `time`, `timestamp` (no zone), **`timestamptz`** (an instant), `interval` |
| SQL Server | `date`, `time`, `datetime2`, **`datetimeoffset`** |
| MySQL | `DATE`, `DATETIME`, `TIMESTAMP` (stored as UTC) |
| SQLite | no date type: TEXT `'YYYY-MM-DD HH:MM:SS'`, REAL Julian day, or INTEGER Unix seconds |

Two different things are both called "time":

- An **instant**: a moment that happened once, the same everywhere (a login, a payment). Store it in **UTC** (or `timestamptz`), and convert to local time only when displaying.
- A **wall-clock** or **civil** time: "the office opens at 09:00", "the report is due on 1 November". Store it as local date/time plus the zone it belongs to (`Asia/Tokyo`).

In SQLite, ISO-8601 text in UTC is the most practical choice: it sorts correctly, is readable and every date function understands it.

## SQLite's date functions

All take a time value followed by any number of **modifiers**:

```sql
SELECT date('2026-10-31', '+1 month') AS plus_month,
       date('2026-10-31', 'start of month', '+1 month', '-1 day') AS end_of_next_month,
       datetime('2026-10-29 22:40', '+9 hours') AS jst,
       strftime('%Y-W%W', '2026-10-29') AS week,
       strftime('%w', '2026-10-31') AS weekday;
```

Useful modifiers: `'+N days|hours|minutes|months|years'`, `'start of month|year|day'`, `'weekday N'` (move forward to the next weekday N, 0 = Sunday), `'unixepoch'` (interpret a number as Unix seconds), `'localtime'` and `'utc'`.

`strftime` codes: `%Y` year, `%m` month, `%d` day, `%H %M %S`, `%w` weekday (0 = Sunday), `%j` day of year, `%W` week of year, `%s` Unix seconds.

Watch month arithmetic: `date('2026-01-31', '+1 month')` is `2026-03-03` in SQLite (31 February overflows). PostgreSQL clamps to the last day of the month (`2026-02-28`). If you mean "end of next month", say so with `start of month` modifiers.

## Differences between times

SQLite: `julianday(b) - julianday(a)` is the difference in **days** as a real number; multiply by 24 for hours or 86400 for seconds. `unixepoch(b) - unixepoch(a)` (3.38+) gives seconds directly. PostgreSQL: `b - a` gives an `interval`; `EXTRACT(EPOCH FROM b - a)` gives seconds. SQL Server: `DATEDIFF(minute, a, b)` (which counts **boundaries crossed**, not elapsed time: `DATEDIFF(year, '2025-12-31', '2026-01-01')` is 1).

```sql
SELECT id, opened, closed,
       ROUND((julianday(closed) - julianday(opened)) * 24, 1) AS hours
FROM tickets WHERE closed IS NOT NULL ORDER BY hours DESC LIMIT 3;
```

## Time zones

Japan is UTC+9 with no daylight saving time, which makes life easy. Most of the world isn't so lucky: in Europe and North America, local time jumps an hour twice a year, so "02:30 on the last Sunday of March" doesn't exist in Berlin, and "02:30 on the last Sunday of October" happens twice.

Rules:

1. Store instants in UTC. Log servers in UTC.
2. Convert at the edges (display, input) using **named zones** (`Europe/Berlin`), never fixed offsets: offsets change with DST and with politics.
3. Group "per day" in the **user's** zone: a ticket at 23:30 UTC on 30 October is on 31 October in Tokyo.

PostgreSQL: `opened AT TIME ZONE 'Asia/Tokyo'`. SQL Server: `AT TIME ZONE 'Tokyo Standard Time'`. SQLite only knows UTC and the machine's local time, so add fixed offsets yourself (fine for Japan) or convert in the application.

## Business hours and calendars

"Response within 4 business hours" excludes nights, weekends and holidays. Real systems use a **calendar table**: one row per day (or per hour) with flags like `is_business_day` and `holiday_name`. Then business time is a join and a sum, and new holidays are just new rows. Japan has ~16 national holidays, plus company days like the New Year break (年末年始).

## Week numbers and fiscal years

Week numbering differs: ISO weeks start on Monday and week 1 contains the first Thursday; US-style weeks start on Sunday. Japanese companies usually use an April–March fiscal year (FY2026 = April 2026 to March 2027). Compute it as: `CASE WHEN month >= 4 THEN year ELSE year - 1 END`.

@@quiz
How should you store the moment a user logged in?
* As an instant in UTC (or timestamptz), converting to local time only for display
- As local time without a zone
- As the user's local time with a fixed offset
- As text in the user's language
? Instants are the same everywhere; store them in UTC.

@@quiz
What's `date('2026-01-31', '+1 month')` in SQLite?
- 2026-02-28
* 2026-03-03, because 31 February overflows into March
- 2026-02-31
- NULL
? Use 'start of month' modifiers when you mean the end of the month.

@@quiz
Why is a calendar table better than calculating business days with formulas?
- It's faster to type
* Holidays and company closures are data, not rules, so they become rows you can add and join
- Databases can't calculate weekdays
- It uses less space
? One row per day with flags handles every exception.

@@flashcards
instant :: a moment in time: store in UTC
civil time :: wall-clock time in a named zone
`timestamptz` :: PostgreSQL instant type
`julianday(b) - julianday(a)` :: difference in days (SQLite)
`'start of month'` :: SQLite modifier
`strftime('%w', d)` :: weekday, 0 = Sunday
`AT TIME ZONE` :: convert zones (PostgreSQL, SQL Server)
DST :: some local times repeat or don't exist
calendar table :: one row per day with business flags
Japanese fiscal year :: April to March

@@exercise Resolution times
For every ticket with a `closed` time, show `id`, `category`, and the time to resolve as `minutes` (whole number) and as a friendly `duration` like `1d 2h 20m` (days, hours, minutes). Sort by minutes descending.
@@starter
SELECT id, category, opened, closed FROM tickets WHERE closed IS NOT NULL;
@@expected
id | category | minutes | duration
---+----------+---------+-----------
6  | hardware | 4800    | 3d 8h 0m
4  | software | 1640    | 1d 3h 20m
2  | network  | 1230    | 0d 20h 30m
5  | security | 148     | 0d 2h 28m
1  | hardware | 53      | 0d 0h 53m
14 | account  | 15      | 0d 0h 15m
3  | account  | 13      | 0d 0h 13m
8  | account  | 11      | 0d 0h 11m
@@requires
julianday
@@hint
Minutes: `CAST(ROUND((julianday(closed) - julianday(opened)) * 1440) AS INTEGER)`. Compute it in a CTE, then `(m / 1440) || 'd ' || ((m % 1440) / 60) || 'h ' || (m % 60) || 'm'`. The parentheses matter: in SQLite `||` binds **tighter** than `/` and `%`, so `m / 1440 || 'd '` means `m / (1440 || 'd ')`.
@@solution
WITH r AS (
    SELECT id, category,
           CAST(ROUND((julianday(closed) - julianday(opened)) * 1440) AS INTEGER) AS minutes
    FROM tickets WHERE closed IS NOT NULL
)
SELECT id, category, minutes,
       (minutes / 1440) || 'd ' || ((minutes % 1440) / 60) || 'h ' || (minutes % 60) || 'm' AS duration
FROM r
ORDER BY minutes DESC;

@@exercise Tokyo days, not UTC days
The `tickets.opened` times are stored in **UTC**. Count tickets per **Tokyo** calendar day (UTC+9) for days from 2026-10-27 (Tokyo time) on. Show `tokyo_day` and `tickets`, sorted by day. Notice that a ticket opened late in the UTC evening moves to the next day.
@@starter
SELECT date(opened) AS tokyo_day, COUNT(*) AS tickets
FROM tickets WHERE date(opened) >= '2026-10-27'
GROUP BY tokyo_day ORDER BY tokyo_day;
@@expected
tokyo_day  | tickets
-----------+--------
2026-10-28 | 1
2026-10-30 | 2
2026-10-31 | 3
@@requires
+9 hours
@@hint
`date(opened, '+9 hours')`, in both the SELECT and the WHERE (or use a CTE).
@@solution
SELECT date(opened, '+9 hours') AS tokyo_day, COUNT(*) AS tickets
FROM tickets
WHERE date(opened, '+9 hours') >= '2026-10-27'
GROUP BY tokyo_day
ORDER BY tokyo_day;

@@exercise A business-day calendar
Build a calendar for **October 2026** with a recursive CTE, mark weekends (`strftime('%w')` 0 or 6) and the holidays in the `holidays` table the starter creates, then show: `business_days` in October, the `last_business_day`, and how many business days each ticket opened in October waited until its closed date: as one summary row for all closed tickets, `avg_wait` rounded to 2 decimals, counting business days **after** the opening day up to and including the closing day.
@@starter
CREATE TABLE holidays (day TEXT PRIMARY KEY, name TEXT);
INSERT INTO holidays VALUES ('2026-10-12', 'Sports Day'), ('2026-11-03', 'Culture Day');
SELECT COUNT(*) FROM holidays;
@@expected
business_days | last_business_day | avg_wait
--------------+-------------------+---------
21            | 2026-10-30        | 0.5
@@requires
WITH RECURSIVE
strftime('%w'
@@hint
`cal(day)` from 2026-10-01 to 2026-10-31; a second CTE `bd` keeps days where `strftime('%w', day) NOT IN ('0', '6') AND day NOT IN (SELECT day FROM holidays)`. The wait per ticket: `(SELECT COUNT(*) FROM bd WHERE bd.day > date(t.opened) AND bd.day <= date(t.closed))`. Combine three scalar subqueries into one SELECT.
@@solution
CREATE TABLE holidays (day TEXT PRIMARY KEY, name TEXT);
INSERT INTO holidays VALUES ('2026-10-12', 'Sports Day'), ('2026-11-03', 'Culture Day');

WITH RECURSIVE cal(day) AS (
    SELECT '2026-10-01'
    UNION ALL
    SELECT date(day, '+1 day') FROM cal WHERE day < '2026-10-31'
),
bd AS (
    SELECT day FROM cal
    WHERE strftime('%w', day) NOT IN ('0', '6') AND day NOT IN (SELECT day FROM holidays)
),
waits AS (
    SELECT (SELECT COUNT(*) FROM bd WHERE bd.day > date(t.opened) AND bd.day <= date(t.closed)) AS wait
    FROM tickets t WHERE t.closed IS NOT NULL
)
SELECT (SELECT COUNT(*) FROM bd) AS business_days,
       (SELECT MAX(day) FROM bd) AS last_business_day,
       (SELECT ROUND(AVG(wait), 2) FROM waits) AS avg_wait;

@@exercise Fiscal quarters
Japanese companies use an April–March fiscal year. For every employee, show `name`, `hired`, the fiscal year `fy` (e.g. `FY2023` for anything from 2023-04-01 to 2024-03-31) and the fiscal quarter `fq` (`Q1` = Apr–Jun). Sort by hired date.
@@starter
SELECT name, hired FROM employees ORDER BY hired;
@@expected
name         | hired      | fy     | fq
-------------+------------+--------+---
Rina Chen    | 2014-02-01 | FY2013 | Q4
Kenji Tanaka | 2015-04-01 | FY2015 | Q1
Hiro Sato    | 2016-07-15 | FY2016 | Q2
Sara Ito     | 2018-09-01 | FY2018 | Q2
Ana Silva    | 2019-10-01 | FY2019 | Q3
Taro Yamada  | 2020-04-01 | FY2020 | Q1
Ben Jones    | 2021-01-10 | FY2020 | Q4
Yuki Kato    | 2022-06-01 | FY2022 | Q1
Mia Lee      | 2023-04-01 | FY2023 | Q1
Emi Suzuki   | 2023-10-01 | FY2023 | Q3
Leo Martin   | 2024-04-01 | FY2024 | Q1
Omar Haddad  | 2026-09-01 | FY2026 | Q2
@@requires
CASE
strftime('%m'
@@hint
Month: `CAST(strftime('%m', hired) AS INTEGER)`. FY year: year minus 1 when month < 4. Quarter: `((month + 8) % 12) / 3 + 1` maps April→1, July→2, October→3, January→4.
@@solution
SELECT name, hired,
       'FY' || (CAST(strftime('%Y', hired) AS INTEGER) - CASE WHEN CAST(strftime('%m', hired) AS INTEGER) < 4 THEN 1 ELSE 0 END) AS fy,
       'Q' || ((CAST(strftime('%m', hired) AS INTEGER) + 8) % 12 / 3 + 1) AS fq
FROM employees
ORDER BY hired;

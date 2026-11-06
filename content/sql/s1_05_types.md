@@lesson s1-types Data Types, CAST and Arithmetic
@@concepts numbers, variables
@@text
Every value in a database has a type, and types decide how values compare, sort and calculate. Getting them right prevents some of the most common bugs: `'10' < '9'`, money that's off by a yen, and averages that are mysteriously whole numbers.

## The common SQL types

| Kind | Standard / PostgreSQL | SQL Server | SQLite storage |
|---|---|---|---|
| whole numbers | `SMALLINT`, `INTEGER`, `BIGINT` | `INT`, `BIGINT` | INTEGER |
| exact decimals | `NUMERIC(10, 2)` / `DECIMAL` | `DECIMAL(10, 2)` | (no exact decimal) |
| approximate | `REAL`, `DOUBLE PRECISION` | `FLOAT` | REAL |
| text | `VARCHAR(n)`, `TEXT` | `NVARCHAR(n)` | TEXT |
| true / false | `BOOLEAN` | `BIT` | INTEGER 0/1 |
| date and time | `DATE`, `TIMESTAMP`, `TIMESTAMPTZ` | `DATE`, `DATETIME2` | TEXT, REAL or INTEGER |
| bytes | `BYTEA` | `VARBINARY` | BLOB |
| identifiers | `UUID` | `UNIQUEIDENTIFIER` | TEXT or BLOB |

**Money**: never store it in `REAL`/`FLOAT`. Binary floating point can't represent 0.1 exactly, so sums drift. Use `NUMERIC(12, 2)`, or store whole minor units in an INTEGER (yen have no minor unit; for dollars, store cents). That's why `salary` and `price` in our database are integers.

## SQLite is dynamically typed

Most databases reject `'hello'` in an INTEGER column. SQLite (by default) stores it anyway: a column has a **type affinity**, a preference, and each *value* has its own **storage class**: NULL, INTEGER, REAL, TEXT or BLOB. `typeof()` shows it:

```sql
SELECT typeof(42), typeof(4.2), typeof('42'), typeof(NULL), typeof(x'00');
```

Since SQLite 3.37 you can opt in to strict checking with `CREATE TABLE t (...) STRICT;`, which allows only `INTEGER`, `REAL`, `TEXT`, `BLOB` and `ANY` and rejects values of the wrong type. Use STRICT for new SQLite tables.

## Integer division

When both sides are integers, `/` throws away the fraction:

```sql
SELECT 7 / 2 AS int_div, 7 / 2.0 AS real_div, 7 % 2 AS remainder, CAST(7 AS REAL) / 2 AS cast_div;
```

This is the #1 reason for "my percentage is always 0": `closed_count / total` with 8 and 15 is `0`. Multiply first (`100 * a / b`) or make one side REAL (`a * 1.0 / b`). PostgreSQL and SQL Server behave the same; MySQL's `/` always returns a decimal (use `DIV` for integer division).

## CAST and rounding

`CAST(expr AS type)` converts explicitly:

```sql
SELECT CAST('2026' AS INTEGER) + 1 AS next_year,
       CAST(9200000 AS TEXT) || ' yen' AS label,
       ROUND(2 / 3.0, 2) AS rounded,
       CAST(2.9 AS INTEGER) AS truncated;
```

`ROUND(x, n)` rounds to n decimals; `CAST(x AS INTEGER)` truncates toward zero. To show a number with a fixed number of decimals, use `printf('%.2f', x)` in SQLite (`format()` in SQLite 3.38+, `TO_CHAR` in PostgreSQL, `FORMAT` in SQL Server).

## Comparing different types

Text compares **character by character**, so `'10' < '9'` is true (because `'1'` < `'9'`). That's why numbers stored as text sort wrong, and why dates must be stored as `YYYY-MM-DD` if they're text. In SQLite, when you compare an INTEGER column to text, the column's affinity converts the text first, but comparing two expressions with no affinity compares the types: any INTEGER or REAL is less than any TEXT. Moral: keep each column one type, and CAST explicitly when the data you received isn't clean.

## Booleans

SQLite has no separate boolean type: comparisons return 1 or 0, and `TRUE`/`FALSE` are aliases for 1 and 0. That makes some neat tricks possible: `SUM(status = 'open')` counts open tickets, because each comparison is 1 or 0. In PostgreSQL write `COUNT(*) FILTER (WHERE status = 'open')` (also supported by SQLite 3.30+) or `SUM(CASE WHEN status = 'open' THEN 1 ELSE 0 END)`, which works everywhere.

@@quiz
`SELECT 8 / 15` returns 0 in SQLite. Why?
* Both operands are integers, so the division is an integer division
- SQLite can't divide small numbers
- The result is rounded down because it's below 1
- It returns NULL
? Make one side REAL (8 * 1.0 / 15) or multiply first.

@@quiz
Which type should you use for prices in PostgreSQL?
- REAL
- DOUBLE PRECISION
* NUMERIC(12, 2), or an integer number of minor units
- TEXT
? Floating point can't represent most decimal fractions exactly.

@@quiz
Why does `ORDER BY version` put '10' before '9' when `version` is TEXT?
- Because of a bug
* Text compares character by character, and '1' sorts before '9'
- Because ORDER BY is descending by default
- Because 10 is even
? Store numbers as numbers, or CAST when sorting.

@@drill
Which function converts a value to another type?
= CAST
= CAST()
? CAST(price AS REAL).

@@flashcards
`typeof(x)` :: SQLite storage class of a value
type affinity :: a SQLite column's preferred type
`STRICT` table :: SQLite 3.37+: enforce column types
`7 / 2` :: 3 (integer division)
`7 / 2.0` :: 3.5
`CAST(x AS INTEGER)` :: truncates toward zero
`ROUND(x, 2)` :: round to 2 decimals
money :: NUMERIC or integer minor units, never FLOAT
`SUM(cond)` :: counts rows where cond is true (SQLite)
`'10' < '9'` :: true for text

@@exercise Closing rate per category
For each ticket `category`, show the number of `tickets`, how many have a `closed` time (`done`), and `rate`: the share that's done as a decimal with 2 places (for example `0.67`). Sort by category.
@@starter
SELECT category, COUNT(*) AS tickets, COUNT(closed) AS done,
       COUNT(closed) / COUNT(*) AS rate
FROM tickets GROUP BY category ORDER BY category;
@@expected
category | tickets | done | rate
---------+---------+------+-----
account  | 4       | 3    | 0.75
hardware | 4       | 2    | 0.5
network  | 2       | 1    | 0.5
security | 2       | 1    | 0.5
software | 3       | 1    | 0.33
@@requires
ROUND
1.0
@@hint
`ROUND(COUNT(closed) * 1.0 / COUNT(*), 2)`.
@@solution
SELECT category, COUNT(*) AS tickets, COUNT(closed) AS done,
       ROUND(COUNT(closed) * 1.0 / COUNT(*), 2) AS rate
FROM tickets GROUP BY category ORDER BY category;

@@exercise Cleaning imported numbers
A CSV import stored everything as text. Show each `item`, its `qty` converted to an integer, the `unit_price` as a real number, the line `total` rounded to 2 decimals, and `typeof` of the converted qty. Sort by the numeric quantity, highest first (as text, '9' would beat '12').
@@setup
CREATE TABLE import (item TEXT, qty TEXT, unit_price TEXT);
INSERT INTO import VALUES ('USB-C cable', '12', '9.99'), ('Dock', '9', '149.50'), ('Mouse', '100', '4.25'), ('Headset', '3', '59.00');
@@starter
SELECT item, qty, unit_price FROM import ORDER BY qty DESC;
@@expected
item        | qty | unit_price | total  | qty_type
------------+-----+------------+--------+---------
Mouse       | 100 | 4.25       | 425.0  | integer
USB-C cable | 12  | 9.99       | 119.88 | integer
Dock        | 9   | 149.5      | 1345.5 | integer
Headset     | 3   | 59.0       | 177.0  | integer
@@requires
CAST(
typeof(
@@hint
`CAST(qty AS INTEGER)`, `CAST(unit_price AS REAL)`, `ROUND(CAST(qty AS INTEGER) * CAST(unit_price AS REAL), 2)`. Order by the cast expression (or its alias).
@@solution
SELECT item,
       CAST(qty AS INTEGER) AS qty,
       CAST(unit_price AS REAL) AS unit_price,
       ROUND(CAST(qty AS INTEGER) * CAST(unit_price AS REAL), 2) AS total,
       typeof(CAST(qty AS INTEGER)) AS qty_type
FROM import
ORDER BY CAST(qty AS INTEGER) DESC;

@@exercise Status counts in one row
Show one row with the number of tickets in each status: `open`, `in_progress`, `resolved` and `closed`, using the "sum of a comparison" trick (or FILTER).
@@starter
SELECT status, COUNT(*) FROM tickets GROUP BY status;
@@expected
open | in_progress | resolved | closed
-----+-------------+----------+-------
5    | 2           | 3        | 5
@@requires
SUM(
@@hint
`SUM(status = 'open') AS open`, and so on for each status.
@@solution
SELECT SUM(status = 'open') AS open,
       SUM(status = 'in-progress') AS in_progress,
       SUM(status = 'resolved') AS resolved,
       SUM(status = 'closed') AS closed
FROM tickets;

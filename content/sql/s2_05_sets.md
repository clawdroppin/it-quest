@@lesson s2-sets Set Operations: UNION, INTERSECT, EXCEPT and VALUES
@@concepts lists
@@text
Joins put tables **side by side**. Set operations stack query results **on top of each other** and compare them as sets of rows.

| Operator | Result |
|---|---|
| `A UNION B` | rows in A or B, duplicates removed |
| `A UNION ALL B` | all rows of A, then all rows of B (duplicates kept) |
| `A INTERSECT B` | rows in both |
| `A EXCEPT B` | rows in A that aren't in B (Oracle calls it `MINUS`) |

Rules:

- Both queries must return the **same number of columns**, with compatible types. Columns are matched **by position**, not by name; the result takes the column names of the first query.
- `ORDER BY` and `LIMIT` go once, at the very end, and apply to the whole result.
- Set operations treat NULLs as equal (unlike `=`), so `EXCEPT` is a NULL-safe way to compare.

```sql
SELECT username FROM employees
INTERSECT
SELECT username FROM logins
ORDER BY username;
```

## UNION vs UNION ALL

`UNION` must find and remove duplicates, which means sorting or hashing the whole result. `UNION ALL` just appends. Use `UNION ALL` unless you actually need de-duplication: it's faster, and it doesn't hide rows you meant to count.

A common use is combining similar data from several places into one list, with a label column:

```sql
SELECT 'device' AS kind, asset_tag AS name, purchased AS date FROM devices WHERE purchased >= '2026-01-01'
UNION ALL
SELECT 'hire', name, hired FROM employees WHERE hired >= '2026-01-01'
ORDER BY date, name;
```

## EXCEPT for reconciliation

IT work is full of "compare two lists" jobs: users in HR vs users in Active Directory, licences bought vs installed, firewall rules expected vs actual. `EXCEPT` in both directions answers "what's missing on each side":

```sql
SELECT username FROM logins
EXCEPT
SELECT username FROM employees
ORDER BY 1;
```

Those are usernames that tried to log in but aren't employees: in our data, the attackers' guesses.

## VALUES: a table you type

`VALUES` creates rows on the spot, which is great for small lookup lists and test data:

```sql,norun
SELECT * FROM (VALUES ('open', 1), ('in-progress', 2), ('resolved', 3), ('closed', 4)) AS s(status, step);
```

That works in PostgreSQL and SQL Server. SQLite doesn't allow naming the columns like `s(status, step)`; it calls them `column1`, `column2`, ... Portable trick: put the VALUES in a CTE with column names:

```sql
WITH s(status, step) AS (VALUES ('open', 1), ('in-progress', 2), ('resolved', 3), ('closed', 4))
SELECT t.id, t.status, s.step FROM tickets t JOIN s ON s.status = t.status
ORDER BY s.step, t.id
LIMIT 5;
```

## Comparing two tables exactly

Are two tables identical? Both `A EXCEPT B` and `B EXCEPT A` must be empty. That's how you test a migration ("the new table has exactly the same rows as the old one") or a refactored query ("the new report gives the same rows as the old one").

@@quiz
What's the difference between UNION and UNION ALL?
* UNION removes duplicate rows; UNION ALL keeps them and is faster
- UNION ALL removes duplicates
- UNION only works on two tables
- There is no difference
? Prefer UNION ALL unless you need de-duplication.

@@quiz
How are columns matched in a UNION?
- By name
* By position; the result uses the first query's column names
- Alphabetically
- By type
? Keep the columns in the same order in every branch.

@@quiz
How do you check that tables A and B contain exactly the same rows?
- `A UNION B` is empty
- `A INTERSECT B` is empty
* `A EXCEPT B` and `B EXCEPT A` are both empty
- `COUNT(A) = COUNT(B)`
? Equal counts can still hide different rows.

@@drill
Which set operator returns rows from the first query that aren't in the second?
= EXCEPT
? Oracle calls it MINUS.

@@flashcards
`UNION` :: combine, remove duplicates
`UNION ALL` :: combine, keep everything (fast)
`INTERSECT` :: rows in both
`EXCEPT` :: rows in the first but not the second
column matching :: by position
`ORDER BY` with UNION :: once, at the end
`VALUES (...), (...)` :: an inline table
`WITH t(a, b) AS (VALUES ...)` :: name VALUES columns

@@exercise Accounts that shouldn't exist
Security wants two lists in one result. Show `problem` and `username`:

- `unknown login`: usernames that appear in `logins` but aren't employees' usernames
- `never logged in`: employee usernames with no row in `logins`

Sort by problem, then username.
@@starter
SELECT username FROM logins;
@@expected
problem         | username
----------------+---------
never logged in | esuzuki
never logged in | hsato
never logged in | ohaddad
never logged in | rchen
never logged in | sito
never logged in | tyamada
never logged in | ykato
unknown login   | admin
unknown login   | oracle
unknown login   | root
unknown login   | test
@@requires
EXCEPT
UNION ALL
@@hint
Each branch is an EXCEPT wrapped in a subquery that adds the label: `SELECT 'unknown login' AS problem, username FROM (SELECT username FROM logins EXCEPT SELECT username FROM employees)`. Join the two branches with UNION ALL.
@@solution
SELECT 'unknown login' AS problem, username
FROM (SELECT username FROM logins EXCEPT SELECT username FROM employees)
UNION ALL
SELECT 'never logged in', username
FROM (SELECT username FROM employees EXCEPT SELECT username FROM logins)
ORDER BY problem, username;

@@exercise A company timeline
Build one timeline of events from **27 October 2026** on: tickets opened (`opened`, kind `ticket`, with the title) and failed logins (`failed login`, with `username from ip`). Show `date` (just the day, `YYYY-MM-DD`), `kind` and `what`, sorted by the full timestamp and only the first 8 rows.
@@starter
SELECT opened AS date, 'ticket' AS kind, title AS what FROM tickets;
@@expected
date       | kind         | what
-----------+--------------+-----------------------------
2026-10-27 | ticket       | Teams camera not working
2026-10-29 | failed login | admin from 203.0.113.7
2026-10-29 | failed login | admin from 203.0.113.7
2026-10-29 | failed login | root from 203.0.113.7
2026-10-29 | failed login | asilva from 203.0.113.7
2026-10-29 | failed login | asilva from 203.0.113.7
2026-10-29 | ticket       | Suspicious login from abroad
2026-10-30 | failed login | test from 198.51.100.23
@@requires
UNION ALL
LIMIT 8
@@hint
Select the full timestamp as a fourth column `ts` in both branches (tickets: `opened`; logins: `time`), and `ORDER BY ts LIMIT 8` at the end. `date(opened)` gives the day. Filter each branch with `>= '2026-10-27'`. To hide `ts`, wrap everything in an outer SELECT.
@@solution
SELECT date, kind, what FROM (
    SELECT date(opened) AS date, 'ticket' AS kind, title AS what, opened AS ts
    FROM tickets WHERE opened >= '2026-10-27'
    UNION ALL
    SELECT date(time), 'failed login', username || ' from ' || ip, time
    FROM logins WHERE success = 0 AND time >= '2026-10-27'
)
ORDER BY ts
LIMIT 8;

@@exercise Did the migration lose anything?
A table was migrated from `old_users` to `new_users`. Show every row that differs: `side` (`only in old` or `only in new`), `username` and `email`. Sort by username, then side. NULL emails count as equal to each other.
@@setup
CREATE TABLE old_users (username TEXT, email TEXT);
INSERT INTO old_users VALUES ('asilva', 'ana@sakura.example'), ('mlee', 'mia@sakura.example'), ('ohaddad', NULL), ('rchen', 'rina@sakura.example');
CREATE TABLE new_users (username TEXT, email TEXT);
INSERT INTO new_users VALUES ('asilva', 'ana@sakura.example'), ('mlee', 'mia.lee@sakura.example'), ('ohaddad', NULL), ('sito', 'sara@sakura.example');
@@starter
SELECT * FROM old_users;
@@expected
side        | username | email
------------+----------+-----------------------
only in new | mlee     | mia.lee@sakura.example
only in old | mlee     | mia@sakura.example
only in old | rchen    | rina@sakura.example
only in new | sito     | sara@sakura.example
@@requires
EXCEPT
@@hint
`SELECT 'only in old' AS side, * FROM (SELECT username, email FROM old_users EXCEPT SELECT username, email FROM new_users) UNION ALL ...`. EXCEPT treats NULLs as equal, so ohaddad won't show up.
@@solution
SELECT 'only in old' AS side, username, email
FROM (SELECT username, email FROM old_users EXCEPT SELECT username, email FROM new_users)
UNION ALL
SELECT 'only in new', username, email
FROM (SELECT username, email FROM new_users EXCEPT SELECT username, email FROM old_users)
ORDER BY username, side;

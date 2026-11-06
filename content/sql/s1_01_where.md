@@lesson s1-where Filtering with WHERE
@@concepts conditions
@@text
`WHERE` keeps only the rows that match a condition:

```sql
SELECT name, title FROM employees WHERE dept_id = 1;
```

| Operator | Example |
|---|---|
| `=`, `<>` (or `!=`) | `status = 'open'` |
| `<`, `<=`, `>`, `>=` | `salary >= 6000000` |
| `AND`, `OR`, `NOT` | `priority = 1 AND status <> 'closed'` |
| `IN (...)` | `category IN ('network', 'security')` |
| `BETWEEN a AND b` | `hired BETWEEN '2020-01-01' AND '2022-12-31'` (inclusive) |
| `LIKE` | `model LIKE 'ThinkPad%'` (`%` any text, `_` one character) |
| `IS NULL`, `IS NOT NULL` | `employee_id IS NULL` |

Text goes in **single quotes**. Dates stored as `'YYYY-MM-DD'` text compare correctly, because that format sorts like the calendar.

```sql
SELECT asset_tag, model, os
FROM devices
WHERE type = 'laptop' AND os LIKE 'Windows%';
```

## NULL is not a value

`NULL` means "unknown / nothing". It isn't equal to anything, not even another NULL, so `WHERE employee_id = NULL` finds **nothing**. Always use `IS NULL`:

```sql
SELECT asset_tag, type FROM devices WHERE employee_id IS NULL;
```

## Parentheses with OR

`AND` binds tighter than `OR`, just like × before + in math. Use parentheses when you mix them:

```sql
SELECT id, title, priority FROM tickets
WHERE (priority = 1 OR category = 'security') AND status <> 'closed';
```

@@quiz
Which finds devices that are **not assigned** to anyone?
* `WHERE employee_id IS NULL`
- `WHERE employee_id = NULL`
- `WHERE employee_id = ''`
- `WHERE employee_id = 0`
? Comparisons with NULL are never true. Use `IS NULL`.

@@quiz
`WHERE model LIKE 'Dell%'` matches...
* models that start with "Dell"
- models that are exactly "Dell%"
- models that end with "Dell"
- nothing, LIKE needs `*`
? In SQL, `%` is the wildcard for any number of characters (`_` for exactly one).

@@quiz
Which condition matches status 'open' or 'pending'?
* WHERE status IN ('open', 'pending')
- WHERE status = 'open', 'pending'
- WHERE status = 'open' AND status = 'pending'
- WHERE status LIKE ('open', 'pending')
? IN is shorter than chaining ORs.

@@flashcards
WHERE :: keep only rows that match a condition
= <> < > <= >= :: comparison operators
AND / OR / NOT :: combine conditions (use brackets with OR)
IN (...) :: matches any value in the list
BETWEEN a AND b :: inclusive range
LIKE 'Dell%' :: pattern: % any text, _ one character
IS NULL :: test for a missing value (never = NULL)

@@drill
Write the WHERE condition (without the word WHERE) for tickets whose priority is 1 or 2. Use IN.
= priority IN (1, 2)
= priority IN (1,2)
? `IN` checks a list of values.

@@exercise Open tickets
Show `id`, `title` and `priority` of tickets whose status is `open`.
@@starter
-- your query here

@@expected
id | title                        | priority
---+------------------------------+---------
10 | Teams camera not working     | 3
11 | Suspicious login from abroad | 1
12 | Install accounting software  | 4
13 | Monitor flickering           | 4
15 | Shared drive access request  | 3
@@hint
`WHERE status = 'open'` (text in single quotes).
@@solution
SELECT id, title, priority FROM tickets WHERE status = 'open';

@@exercise Old laptops
Our policy is to replace laptops bought **before 2023**. Show the `asset_tag`, `model` and `purchased` date of laptops purchased before `2023-01-01`.
@@starter
-- your query here

@@expected
asset_tag | model        | purchased
----------+--------------+-----------
LT-0052   | ThinkPad T14 | 2020-04-01
@@hint
Two conditions joined with AND: the type, and `purchased < '2023-01-01'`.
@@solution
SELECT asset_tag, model, purchased FROM devices WHERE type = 'laptop' AND purchased < '2023-01-01';

@@exercise Failed logins from outside
Show `username`, `ip` and `time` of **failed** logins (`success = 0`) from IPs that **don't** start with `10.` (our internal network).
@@starter
-- your query here

@@expected
username | ip            | time
---------+---------------+--------------------
admin    | 203.0.113.7   | 2026-10-29 22:31:00
admin    | 203.0.113.7   | 2026-10-29 22:31:04
root     | 203.0.113.7   | 2026-10-29 22:31:09
asilva   | 203.0.113.7   | 2026-10-29 22:31:15
asilva   | 203.0.113.7   | 2026-10-29 22:31:22
test     | 198.51.100.23 | 2026-10-30 03:12:00
test     | 198.51.100.23 | 2026-10-30 03:12:03
oracle   | 198.51.100.23 | 2026-10-31 03:00:00
@@hint
`WHERE success = 0 AND ip NOT LIKE '10.%'`
@@solution
SELECT username, ip, time FROM logins WHERE success = 0 AND ip NOT LIKE '10.%';

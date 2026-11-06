@@lesson s2-joins Joining Tables
@@text
Data is split across tables to avoid repeating it (the department's location is stored once, not on every employee). A **JOIN** puts it back together:

```sql
SELECT e.name, d.name AS department, d.location
FROM employees AS e
JOIN departments AS d ON d.id = e.dept_id;
```

`e` and `d` are **table aliases**, short names you'll use constantly. `ON` says how rows match: the employee's `dept_id` equals the department's `id`.

## INNER vs LEFT JOIN

- `JOIN` (= `INNER JOIN`) keeps only rows that have a match on **both** sides. Omar the contractor has no department, so he disappeared above.
- `LEFT JOIN` keeps **every** row from the left table, filling in NULL where there's no match:

```sql
SELECT e.name, d.name AS department
FROM employees AS e
LEFT JOIN departments AS d ON d.id = e.dept_id
WHERE e.id >= 10;
```

A LEFT JOIN plus `WHERE right_side.id IS NULL` finds rows with **no** match, an extremely useful pattern:

```sql
SELECT e.name
FROM employees AS e
LEFT JOIN devices AS dv ON dv.employee_id = e.id
WHERE dv.id IS NULL;
```

*Employees without any device.*

## Several joins, and joining a table to itself

```sql
SELECT t.id, t.title, r.name AS requester, a.name AS assignee
FROM tickets AS t
JOIN employees AS r ON r.id = t.requester_id
LEFT JOIN employees AS a ON a.id = t.assignee_id
WHERE t.status = 'open';
```

The same table (`employees`) appears twice with different aliases. That's also how you get each employee's manager:

```sql
SELECT e.name, m.name AS manager
FROM employees AS e
LEFT JOIN employees AS m ON m.id = e.manager_id
WHERE e.dept_id = 1;
```

@@quiz
`employees` JOIN `departments`: why is Omar (dept_id NULL) missing from the result?
* An inner join only keeps rows with a match on both sides
- NULL rows are always hidden
- He's on the right side
- JOIN sorts him out
? Use LEFT JOIN to keep every employee.

@@quiz
Which pattern finds departments that have **no** employees?
* `departments LEFT JOIN employees ... WHERE employees.id IS NULL`
- `departments JOIN employees ... WHERE employees.id IS NULL`
- `departments JOIN employees WHERE COUNT(*) = 0`
- `SELECT * FROM departments WHERE employees IS NULL`
? LEFT JOIN keeps unmatched rows with NULLs; then filter for those NULLs.

@@quiz
Why give tables aliases like `FROM employees e JOIN departments d`?
* To write shorter column references such as e.name and d.name, and to tell same-named columns apart
- Joins require aliases
- It makes the query faster
- It renames the tables permanently
? Aliases are also required when joining a table to itself.

@@drill
Which join keeps every row from the left table, even without a match?
= LEFT JOIN
= LEFT OUTER JOIN
? Missing columns from the right are NULL.

@@flashcards
JOIN ... ON :: Combine rows from two tables where the ON condition matches
INNER JOIN :: Only rows with a match on both sides
LEFT JOIN :: All rows from the left table, NULLs where nothing matches
LEFT JOIN + IS NULL :: Find rows without a match
table alias :: FROM employees AS e, then e.name
self join :: Joining a table to itself, e.g. employees and their managers

@@exercise Who has which laptop
Show each laptop's `asset_tag`, `model` and the **name** of the employee it's assigned to (as `assigned_to`). Only assigned laptops. Order by asset tag.
@@starter
-- your query here

@@expected
asset_tag | model         | assigned_to
----------+---------------+-------------
LT-0042   | ThinkPad T14  | Ana Silva
LT-0043   | ThinkPad T14  | Mia Lee
LT-0050   | MacBook Air   | Ben Jones
LT-0051   | ThinkPad X1   | Kenji Tanaka
LT-0052   | ThinkPad T14  | Taro Yamada
LT-0061   | Dell Latitude | Omar Haddad
@@hint
`FROM devices AS dv JOIN employees AS e ON e.id = dv.employee_id WHERE dv.type = 'laptop' ORDER BY dv.asset_tag`
@@solution
SELECT dv.asset_tag, dv.model, e.name AS assigned_to
FROM devices AS dv
JOIN employees AS e ON e.id = dv.employee_id
WHERE dv.type = 'laptop'
ORDER BY dv.asset_tag;

@@exercise Tickets per department
Count tickets per **requester's department name** (as `department` and `tickets`). Requesters without a department should be grouped under NULL. Most tickets first, ties by department name.
@@starter
-- your query here

@@expected
department | tickets
-----------+--------
Support    | 5
Finance    | 3
IT         | 3
Sales      | 3
NULL       | 1
@@hint
Join tickets → employees (requester) with JOIN, then employees → departments with **LEFT** JOIN, then `GROUP BY d.name`.
@@solution
SELECT d.name AS department, COUNT(*) AS tickets
FROM tickets AS t
JOIN employees AS e ON e.id = t.requester_id
LEFT JOIN departments AS d ON d.id = e.dept_id
GROUP BY d.name
ORDER BY tickets DESC, department;

@@exercise Unassigned open work
Show `id`, `title` and the requester's `name` (as `requester`) for tickets that are **not closed or resolved** and have **no assignee**. Order by id.
@@starter
-- your query here

@@expected
id | title                       | requester
---+-----------------------------+------------
10 | Teams camera not working    | Hiro Sato
12 | Install accounting software | Taro Yamada
15 | Shared drive access request | Omar Haddad
@@hint
`WHERE t.status IN ('open', 'in-progress') AND t.assignee_id IS NULL`
@@solution
SELECT t.id, t.title, e.name AS requester
FROM tickets AS t
JOIN employees AS e ON e.id = t.requester_id
WHERE t.status IN ('open', 'in-progress') AND t.assignee_id IS NULL
ORDER BY t.id;

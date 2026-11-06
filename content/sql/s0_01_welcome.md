@@lesson s0-welcome Databases and Your First Query
@@concepts hello
@@text
Almost every system in a company keeps its data in a **database**: users, tickets, orders, devices, logs. **SQL** (Structured Query Language) is how you ask a database questions, and it's been the same core language for 50 years. Help desk staff use it for reports, sysadmins to check application databases, security analysts to search logs, and it shows up in the **IPA FE exam** in Japan, AWS and Azure certifications, and countless job postings.

## Tables, rows, columns

A **relational database** stores data in **tables**. Each **row** is one thing (one employee); each **column** is one fact about it (name, department). Tables are linked by **keys**: an employee row stores `dept_id = 1`, which points at the row with `id = 1` in `departments`.

Throughout this course you'll work with the database of **Sakura Systems K.K.**, a small Japanese company:

| Table | Holds |
|---|---|
| `departments` | id, name, location |
| `employees` | id, name, username, dept_id, title, hired, salary (yen per year), manager_id |
| `devices` | asset_tag, type, model, os, purchased, price, warranty_until, employee_id |
| `tickets` | title, category, priority 1–4, status, opened, closed, requester_id, assignee_id |
| `logins` | username, ip, time, success (1 or 0) |

Every example and exercise starts with a fresh copy, so experiment freely: you can't break anything.

## SELECT

```sql
SELECT * FROM departments;
```

`SELECT` picks columns, `FROM` says which table, `*` means all columns, and every statement ends with `;`. List the columns you want, in the order you want:

```sql
SELECT name, title, hired FROM employees;
```

SQL keywords aren't case-sensitive (`select` works), but writing them in CAPITALS is the convention, so queries are easy to read. This course runs **SQLite**; the SQL here also works, nearly unchanged, in PostgreSQL, MySQL, SQL Server and Oracle.

@@quiz
In a table, what is a row?
* one record, e.g. one employee
- one fact, e.g. every employee's name
- the table's name
- a query
? Rows are records; columns are the fields every record has.

@@quiz
Which query shows all columns of the `devices` table?
* `SELECT * FROM devices;`
- `SELECT devices;`
- `GET * FROM devices;`
- `SHOW devices.*;`
? `*` means every column.

@@quiz
Which query returns only the name and os columns of devices?
* SELECT name, os FROM devices;
- SELECT * FROM devices WHERE name, os;
- GET name, os FROM devices;
- SELECT devices(name, os);
? List the columns you want after SELECT, separated by commas.

@@flashcards
database :: Organized, stored data a program can query
table :: Rows (records) and columns (fields)
primary key :: A column that uniquely identifies each row, e.g. id
foreign key :: A column that points at another table's primary key, e.g. dept_id
SELECT ... FROM ... :: Read columns from a table
SQLite :: A small database engine that lives in a single file

@@exercise Department list
Show the `name` and `location` of every department.
@@starter
-- your query here

@@expected
name      | location
----------+---------
IT        | Tokyo
Sales     | Tokyo
Finance   | Osaka
Support   | Osaka
Marketing | Fukuoka
@@hint
`SELECT name, location FROM departments;`
@@solution
SELECT name, location FROM departments;

@@exercise Device inventory
Show `asset_tag`, `type` and `model` of all devices.
@@starter
-- your query here

@@expected
asset_tag | type    | model
----------+---------+------------------
LT-0042   | laptop  | ThinkPad T14
LT-0043   | laptop  | ThinkPad T14
LT-0050   | laptop  | MacBook Air
LT-0051   | laptop  | ThinkPad X1
LT-0052   | laptop  | ThinkPad T14
LT-0060   | laptop  | Dell Latitude
DT-0007   | desktop | OptiPlex 7010
DT-0008   | desktop | OptiPlex 7010
PH-0101   | phone   | iPhone 15
PH-0102   | phone   | iPhone 15
SV-0001   | server  | PowerEdge R650
SV-0002   | server  | PowerEdge R650
PR-0003   | printer | imageRUNNER C3326
LT-0061   | laptop  | Dell Latitude
@@hint
List the three columns after SELECT, separated by commas.
@@solution
SELECT asset_tag, type, model FROM devices;

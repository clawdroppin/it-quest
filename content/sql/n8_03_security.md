@@lesson n8-security Database Security: Privileges, Row-Level Security, Masking and Auditing
@@concepts errors
@@text
Module 5 covered SQL injection and the basics of permissions. In production, the database holds the crown jewels (customer data, salaries, credentials), so it needs defence in depth.

## Least privilege, in practice

Create separate roles for separate jobs, and give each only what it needs:

```sql,norun
-- PostgreSQL
CREATE ROLE app_read NOLOGIN;
GRANT CONNECT ON DATABASE hr TO app_read;
GRANT USAGE ON SCHEMA public TO app_read;
GRANT SELECT ON employees_public, departments TO app_read;

CREATE ROLE app_write NOLOGIN;
GRANT SELECT, INSERT, UPDATE ON tickets TO app_write;   -- no DELETE

CREATE ROLE helpdesk_api LOGIN PASSWORD '...' IN ROLE app_read, app_write;
REVOKE ALL ON employees FROM PUBLIC;
```

- The application never connects as the owner/superuser. Migrations run as the owner; the app runs as a limited role, so a SQL injection can't `DROP TABLE`.
- Humans get personal accounts (auditable), never a shared "admin".
- Analysts read from a **replica** or a reporting schema of views.
- Review grants regularly: `\dp` in psql, `information_schema.role_table_grants`.

## Row-level security (RLS)

Views can hide *columns*; RLS hides *rows*, enforced by the database for every query, whatever the application does:

```sql,norun
ALTER TABLE tickets ENABLE ROW LEVEL SECURITY;
CREATE POLICY own_tickets ON tickets
    USING (requester = current_user OR pg_has_role('helpdesk', 'member'));
```

Multi-tenant SaaS apps use RLS with a tenant id (`USING (tenant_id = current_setting('app.tenant')::int)`) so one bug in a WHERE clause can't leak another customer's data. SQL Server has Security Policies; SQLite has neither users nor RLS, so you emulate it with views filtered by a "current session" value.

## Protecting sensitive data

| Technique | What it does | Example |
|---|---|---|
| **Don't store it** | the best protection | don't keep full card numbers; your payment provider does |
| **Hashing** | one-way; compare, never read back | passwords with bcrypt/Argon2, done in the app |
| **Encryption at rest** | disk/backup theft doesn't expose data | TDE, encrypted volumes, encrypted backups |
| **Column encryption** | only the app (with the key) can read | `pgcrypto`, SQL Server Always Encrypted |
| **Encryption in transit** | no sniffing on the network | require TLS: `sslmode=verify-full` |
| **Masking** | show only part of a value | `****@sakura.example`, `***-****-1234` |
| **Pseudonymisation** | replace identities with tokens | analytics without names |

Passwords are never stored in a recoverable form, and never hashed with plain MD5/SHA-256 (too fast to brute force). Use a slow, salted algorithm in the application.

## Auditing

"Who changed this salary, and when?" needs an audit trail:

- **Trigger-based audit tables**: capture old and new values, who and when. Simple and portable.
- **Database audit features**: PostgreSQL's `pgaudit`, SQL Server Audit, MySQL Enterprise Audit: log statements by user, including SELECTs on sensitive tables.
- **Temporal tables** (SQL Server, MariaDB): the database keeps every past version of each row automatically, queryable `FOR SYSTEM_TIME AS OF '2026-10-01'`.

Audit logs must be **append-only** for the people being audited: write them to a table the application role can INSERT into but not UPDATE or DELETE, or ship them off the server.

## Privacy law: the right to erasure

Under GDPR (EU) and Japan's APPI, people can ask you to delete their personal data. Deleting a customer row is often impossible (invoices must be kept for years for tax law), so systems **anonymise**: replace names, emails and phone numbers with placeholders, keep the non-personal facts. Do it in one transaction across every table that holds personal data, and don't forget backups, logs and analytics copies (document your retention periods).

## Securing the server

- Don't expose the database port to the internet; allow connections only from app servers (firewall / security groups).
- Patch it; databases have CVEs too.
- Use strong authentication (SCRAM-SHA-256 in PostgreSQL, not md5/trust); disable default accounts.
- Encrypt and test **backups**; a stolen backup is a stolen database.

@@quiz
Why shouldn't the application connect to the database as its owner or a superuser?
* A SQL injection or bug could then do anything, including dropping tables; a limited role contains the damage
- It's slower
- Superusers can't run SELECT
- The owner can't use TLS
? Least privilege limits the blast radius.

@@quiz
What does row-level security add on top of views?
- Faster queries
* The database itself filters rows for every query by policy, whatever SQL the application sends
- Column encryption
- Automatic backups
? A missing WHERE in app code can't leak other tenants' rows.

@@quiz
A customer asks to be forgotten, but invoices must be kept for 7 years. What's the usual approach?
- Delete everything including invoices
- Refuse the request
* Anonymise the personal fields everywhere, keeping the non-personal financial records
- Move the data to a backup
? Pseudonymise or anonymise in one transaction, and handle copies.

@@flashcards
least privilege :: each role gets only what it needs
`GRANT SELECT ON v TO role` :: allow reading a view
row-level security :: per-row policies enforced by the DB
`CREATE POLICY` :: PostgreSQL RLS rule
masking :: show only part of a value
hashing passwords :: slow salted algorithm in the app (bcrypt, Argon2)
TDE :: transparent data encryption at rest
`sslmode=verify-full` :: require TLS and verify the server
temporal table :: keeps every row version
pgaudit :: PostgreSQL statement auditing
right to erasure :: GDPR / APPI: anonymise personal data

@@exercise A masked directory
Create a view `employees_masked` that support staff can use: `name`, `username`, department name (`department`, NULL allowed), the department's `location`, the salary replaced by a `salary_band` (`< 5M`, `5–8M`, `>= 8M`; use these exact labels with an en dash) and an `email` built as `username@sakura.example` but masked to the first letter: `a*****@sakura.example` (one star per remaining character of the username). Then select `name`, `username`, `department`, `salary_band` and `email` for everyone in Tokyo departments, sorted by name.
@@starter
SELECT name, username, salary FROM employees ORDER BY name;
@@expected
name         | username | department | salary_band | email
-------------+----------+------------+-------------+-----------------------
Ana Silva    | asilva   | IT         | 5–8M        | a*****@sakura.example
Ben Jones    | bjones   | Sales      | 5–8M        | b*****@sakura.example
Hiro Sato    | hsato    | Sales      | >= 8M       | h****@sakura.example
Kenji Tanaka | ktanaka  | IT         | >= 8M       | k******@sakura.example
Mia Lee      | mlee     | IT         | < 5M        | m***@sakura.example
Yuki Kato    | ykato    | Sales      | 5–8M        | y****@sakura.example
@@requires
CREATE VIEW employees_masked
CASE
@@hint
Stars: `substr(username, 1, 1) || substr('**********', 1, length(username) - 1) || '@sakura.example'`. Band: `CASE WHEN salary < 5000000 THEN '< 5M' WHEN salary < 8000000 THEN '5–8M' ELSE '>= 8M' END`.
@@solution
CREATE VIEW employees_masked AS
SELECT e.name, e.username, d.name AS department, d.location,
       CASE WHEN e.salary < 5000000 THEN '< 5M' WHEN e.salary < 8000000 THEN '5–8M' ELSE '>= 8M' END AS salary_band,
       substr(e.username, 1, 1) || substr('**********', 1, length(e.username) - 1) || '@sakura.example' AS email
FROM employees e LEFT JOIN departments d ON d.id = e.dept_id;

SELECT name, username, department, salary_band, email
FROM employees_masked WHERE location = 'Tokyo'
ORDER BY name;

@@exercise Row-level security, SQLite style
SQLite has no users, so emulate RLS: a one-row table `session(username)` holds the current user. Create a view `my_tickets` that shows a ticket only if the current user requested it, is assigned to it, or is a **manager** (someone who has direct reports). Show `id` and `title` from `my_tickets` for `mlee`, then switch the session to `ykato`, then to `hsato` (a manager), and count rows for each. Output: one table of mlee's tickets, then one row per user with `username` and `visible`.
@@starter
CREATE TABLE session (username TEXT);
INSERT INTO session VALUES ('mlee');
-- CREATE VIEW my_tickets AS ...
@@expected
id | title
---+--------------------------
1  | Printer jam on 3F
3  | Locked out of account
4  | Excel crashes on open
6  | New laptop for contractor
8  | Password reset
13 | Monitor flickering
14 | Locked out again

username | visible
---------+--------
mlee     | 7
ykato    | 1
hsato    | 15
@@requires
CREATE VIEW my_tickets
FROM session
@@hint
In the view, join to `employees me ON me.username = (SELECT username FROM session)` and keep tickets `WHERE t.requester_id = me.id OR t.assignee_id = me.id OR EXISTS (SELECT 1 FROM employees r WHERE r.manager_id = me.id)`. To collect counts as you switch users, insert into a `results` table after each `UPDATE session SET username = ...`.
@@solution
CREATE TABLE session (username TEXT);
INSERT INTO session VALUES ('mlee');
CREATE VIEW my_tickets AS
SELECT t.*
FROM tickets t
JOIN employees me ON me.username = (SELECT username FROM session)
WHERE t.requester_id = me.id OR t.assignee_id = me.id
   OR EXISTS (SELECT 1 FROM employees r WHERE r.manager_id = me.id);

SELECT id, title FROM my_tickets ORDER BY id;

CREATE TABLE results (username TEXT, visible INTEGER);
INSERT INTO results SELECT 'mlee', COUNT(*) FROM my_tickets;
UPDATE session SET username = 'ykato';
INSERT INTO results SELECT 'ykato', COUNT(*) FROM my_tickets;
UPDATE session SET username = 'hsato';
INSERT INTO results SELECT 'hsato', COUNT(*) FROM my_tickets;
SELECT username, visible FROM results;

@@exercise Anonymise on request
Employee `ohaddad` (the contractor) left and asked to be forgotten. In **one transaction**: replace their name with `Former contractor #12` (use their id), their username with `deleted-12`, set their salary to 0, and replace their username in `logins` too (keep the login rows for security statistics). Keep their tickets. Then show the employee row and the number of tickets still linked to id 12.
@@setup
CREATE TABLE employees (id INTEGER PRIMARY KEY, name TEXT, username TEXT UNIQUE, salary INTEGER);
INSERT INTO employees VALUES (11, 'Emi Suzuki', 'esuzuki', 4000000), (12, 'Omar Haddad', 'ohaddad', 4500000);
CREATE TABLE tickets (id INTEGER PRIMARY KEY, title TEXT, requester_id INTEGER);
INSERT INTO tickets VALUES (15, 'Shared drive access request', 12), (16, 'Laptop return', 12);
CREATE TABLE logins (id INTEGER PRIMARY KEY, username TEXT, ip TEXT);
INSERT INTO logins VALUES (1, 'ohaddad', '10.0.2.50'), (2, 'ohaddad', '10.0.2.50'), (3, 'esuzuki', '10.0.1.20');
@@starter
SELECT * FROM employees;
@@expected
id | name                  | username   | salary
---+-----------------------+------------+--------
11 | Emi Suzuki            | esuzuki    | 4000000
12 | Former contractor #12 | deleted-12 | 0

tickets_kept
------------
2

username   | logins
-----------+-------
deleted-12 | 2
esuzuki    | 1
@@requires
BEGIN
COMMIT
UPDATE logins
@@hint
Update `logins` **before** you change the username in `employees` (or use the literal old username). `UPDATE employees SET name = 'Former contractor #' || id, username = 'deleted-' || id, salary = 0 WHERE username = 'ohaddad';`
@@solution
BEGIN;
UPDATE logins SET username = 'deleted-12' WHERE username = 'ohaddad';
UPDATE employees SET name = 'Former contractor #' || id, username = 'deleted-' || id, salary = 0
WHERE username = 'ohaddad';
COMMIT;

SELECT id, name, username, salary FROM employees ORDER BY id;
SELECT COUNT(*) AS tickets_kept FROM tickets WHERE requester_id = 12;
SELECT username, COUNT(*) AS logins FROM logins GROUP BY username ORDER BY username;

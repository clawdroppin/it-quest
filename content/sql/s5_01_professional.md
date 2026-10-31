@@lesson s5-professional Security, Design and Database Administration
@@text
## SQL injection: the classic attack

If a program builds SQL by gluing user input into a string, the input can **change the query**:

```python
import sqlite3
con = sqlite3.connect(":memory:")
con.execute("CREATE TABLE users (name TEXT, password TEXT)")
con.execute("INSERT INTO users VALUES ('admin', 's3cret')")

typed = "' OR '1'='1"     # what an attacker types into the login form
unsafe = f"SELECT * FROM users WHERE name = 'admin' AND password = '{typed}'"
print(unsafe)
print("unsafe login:", con.execute(unsafe).fetchall())          # logged in without the password!

safe = "SELECT * FROM users WHERE name = ? AND password = ?"
print("safe login:  ", con.execute(safe, ("admin", typed)).fetchall())   # [] : the input is just data
```

The fix is always the same: **parameterized queries** (placeholders like `?`, `%s`, `:name` or `@name`), never string formatting. Every language and database library supports them. SQL injection has been in the OWASP Top 10 for over 20 years, and it's on Security+.

## Least privilege

Applications shouldn't connect as the database administrator. Give each one an account with only the rights it needs:

```sql,norun
CREATE USER helpdesk_app WITH PASSWORD '...';               -- PostgreSQL syntax
GRANT SELECT, INSERT, UPDATE ON tickets TO helpdesk_app;     -- no DELETE, no DROP
GRANT SELECT ON employees TO helpdesk_app;
REVOKE ALL ON salaries FROM PUBLIC;
```

## Normalization in one paragraph

Store each fact **once**. If every ticket row repeated the requester's department name, renaming "Support" would mean updating hundreds of rows, and missing one leaves the data contradicting itself. So: one table per kind of thing, linked by keys (**1NF**: one value per cell; **2NF/3NF**: every column depends on the key, the whole key, and nothing but the key). Reporting databases sometimes *de*normalize on purpose, for speed.

## Database administration basics

- **Backups**: full backups plus transaction log / WAL backups for point-in-time recovery. Test restores regularly. `sqlite3 db ".backup copy.db"`, `pg_dump`, `mysqldump`, SQL Server's `BACKUP DATABASE`.
- **Indexes and query plans**: when a query is slow, look at its plan (`EXPLAIN` / `EXPLAIN ANALYZE`): a full table scan on a big table usually means a missing index.
- **Monitoring**: connections, slow queries, disk space, replication lag.
- **High availability**: replicas, failover; in the cloud, managed services (Amazon RDS, Azure SQL Database) handle backups, patching and failover for you.

## SQL vs NoSQL

Relational databases (PostgreSQL, MySQL, SQL Server, Oracle) are the default for structured business data. **NoSQL** databases trade some of SQL's guarantees for other strengths: document stores (MongoDB, Cosmos DB) for flexible JSON, key-value stores (Redis, DynamoDB) for speed and scale, time-series databases for metrics. Knowing which fits which job is a cloud certification topic.

@@quiz
How do you prevent SQL injection?
* Parameterized queries (placeholders), never building SQL from user input with string formatting
- Escaping quotes by hand
- Using a NoSQL database
- Hiding error messages
? Placeholders send the query and the data separately, so input can never become SQL.

@@quiz
Your app only reads and adds tickets. Which database rights should its account have?
* SELECT and INSERT on the tables it uses
- Database owner, to be safe
- ALL PRIVILEGES
- the same account as the DBA
? Least privilege: if the app is hacked, the attacker gets only what the app had.

@@quiz
Why is storing the department **name** on every ticket row a problem?
* The same fact is repeated, so it can get out of sync when it changes
- Text takes too much space
- SQL can't search text
- It's not a problem
? Normalization stores each fact once and references it by key.

@@flashcards
SQL injection :: User input changing a query's meaning; fixed with parameterized queries
parameterized query :: SQL with placeholders (?), data sent separately
least privilege :: Accounts get only the rights they need
GRANT / REVOKE :: Give / take away database permissions
normalization :: Store each fact once; link tables with keys
EXPLAIN :: Show how the database will run a query
point-in-time recovery :: Restoring to any moment using full backups plus logs
NoSQL :: Non-relational databases: document, key-value, graph, time-series

@@exercise Find the injection
The `audit` table holds queries an application ran. Find the ones that look like **SQL injection attempts**: `query` contains `' OR '`, `--`, `UNION SELECT` or `; DROP` (any case). Show their `id` and `source_ip`, ordered by id.
@@setup
CREATE TABLE audit (id INTEGER PRIMARY KEY, source_ip TEXT, query TEXT);
INSERT INTO audit VALUES
 (1, '10.0.0.21', 'SELECT * FROM products WHERE id = 42'),
 (2, '203.0.113.7', 'SELECT * FROM users WHERE name = ''admin'' AND pw = '''' OR ''1''=''1'''),
 (3, '10.0.0.34', 'SELECT name FROM products WHERE name LIKE ''%cable%'''),
 (4, '203.0.113.7', 'SELECT * FROM products WHERE id = 1 union select username, password from users'),
 (5, '198.51.100.23', 'SELECT * FROM users WHERE name = ''x''; DROP TABLE users; --'''),
 (6, '10.0.0.21', 'UPDATE carts SET qty = 2 WHERE id = 9');
@@starter
-- LOWER(query) LIKE '%...%'

@@expected
id | source_ip
---+--------------
2  | 203.0.113.7
4  | 203.0.113.7
5  | 198.51.100.23
@@hint
`WHERE LOWER(query) LIKE '%'' or ''%' OR query LIKE '%--%' OR LOWER(query) LIKE '%union select%' OR LOWER(query) LIKE '%; drop%'`. Inside SQL text, a single quote is written twice: `''`.
@@solution
SELECT id, source_ip
FROM audit
WHERE LOWER(query) LIKE '%'' or ''%'
   OR query LIKE '%--%'
   OR LOWER(query) LIKE '%union select%'
   OR LOWER(query) LIKE '%; drop%'
ORDER BY id;

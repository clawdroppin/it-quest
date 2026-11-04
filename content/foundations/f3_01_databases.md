@@lesson f3-databases Data and Databases
@@text
Almost every business application is a front end on top of a database: the ticketing system, payroll, the online shop, the HR system. IT staff who aren't developers still meet databases constantly: a slow application whose real problem is the database server, a CSV export that must be imported somewhere else, a report someone needs by Friday, a backup that must include the database correctly. This lesson is the map; the SQL track is the deep dive.

## Spreadsheets vs databases

A spreadsheet is great for one person analysing data. It breaks down when:

- several people edit at once,
- the same fact is typed in many places (a customer's address in 40 rows) and drifts out of sync,
- data must be validated (no order without a customer, no negative quantity),
- there are millions of rows,
- access must be controlled per user and audited.

A **database management system (DBMS)** handles all of these.

## Relational databases

The most common kind stores data in **tables** with **rows** (records) and **columns** (fields), and links tables by keys:

```text
customers                         orders
id | name        | city           id  | customer_id | total  | placed
---+-------------+--------         ----+-------------+--------+-----------
1  | Sato Kenji  | Osaka          501 | 1           | 12000  | 2026-03-01
2  | Ana Silva   | Lisbon         502 | 2           |  4500  | 2026-03-02
                                  503 | 1           |  8000  | 2026-03-04
```

- A **primary key** (`id`) uniquely identifies each row.
- A **foreign key** (`orders.customer_id`) points to a primary key in another table. The database can enforce that it always points to an existing customer (**referential integrity**).
- Storing each fact once and linking by keys is **normalisation**. Sato's city is stored once; all his orders refer to him.

Popular relational databases: **PostgreSQL**, **MySQL/MariaDB**, **Microsoft SQL Server**, **Oracle**, **SQLite** (a database in a single file, used inside apps and phones). Cloud versions: Amazon RDS/Aurora, Azure SQL, Google Cloud SQL.

## SQL in one minute

**SQL** (Structured Query Language) is how you ask relational databases questions:

```sql,norun
SELECT c.name, COUNT(*) AS orders, SUM(o.total) AS spent
FROM customers c
JOIN orders o ON o.customer_id = c.id
WHERE o.placed >= '2026-03-01'
GROUP BY c.name
ORDER BY spent DESC;
```

`SELECT` picks columns, `FROM`/`JOIN` combines tables, `WHERE` filters rows, `GROUP BY` summarises, `ORDER BY` sorts. `INSERT`, `UPDATE` and `DELETE` change data, and an `UPDATE` without a `WHERE` changes **every row**: the classic disaster.

## Transactions and ACID

A bank transfer must subtract from one account **and** add to the other, or neither. A **transaction** groups changes so they succeed or fail together. Relational databases guarantee **ACID**:

- **Atomicity**: all or nothing.
- **Consistency**: rules (constraints) always hold.
- **Isolation**: concurrent transactions don't see each other's half-finished work.
- **Durability**: once committed, it survives a crash.

## Other kinds of database (NoSQL)

| Type | Stores | Examples | Used for |
|---|---|---|---|
| Document | JSON-like documents | MongoDB, Cosmos DB, Firestore | flexible app data |
| Key-value | a value per key | Redis, DynamoDB | caching, sessions |
| Wide-column | huge sparse tables | Cassandra, Bigtable | massive write volumes |
| Graph | nodes and relationships | Neo4j | networks, recommendations |
| Time series | values over time | InfluxDB, Prometheus | metrics, IoT |
| Search | indexed text | Elasticsearch, OpenSearch | search, log analysis |

Most organisations use several: PostgreSQL for orders, Redis for caching, Elasticsearch for logs.

## Data formats you'll handle

- **CSV**: rows of comma-separated values. Simple and universal, but watch for commas inside values (they need quotes), encodings (module 0), and Excel mangling leading zeros (`00123` becomes `123`) and long numbers.
- **JSON**: nested objects and lists; the language of web APIs.
- **XML**: older, verbose, still everywhere in enterprise systems and config files.
- **YAML**: human-friendly config (Kubernetes, Ansible, GitHub Actions); indentation matters.
- **Parquet**: compressed column format for analytics.

## Data in IT operations

- **Backups**: databases need consistent backups (the DB's own tools or snapshot integration), and transaction **log** backups for tight RPOs.
- **Performance**: a slow app is often a slow query; **indexes** (like a book's index) make lookups fast. DBAs read **query plans**.
- **Security**: least-privilege database accounts for apps, no shared admin logins, encrypted connections, no customer data in test environments without masking.
- **Privacy laws**: personal data is regulated: Japan's APPI (個人情報保護法), the EU's GDPR, and others. Know where personal data lives, who can see it, and how long it's kept.

@@quiz
What does a foreign key do?
* Links a row to a row in another table and lets the database enforce that the referenced row exists
- Encrypts a column
- Makes a column unique
- Speeds up backups
? `orders.customer_id` → `customers.id`.

@@quiz
Someone runs `UPDATE employees SET salary = 0;`. What happened?
- Nothing; it's missing a WHERE so it fails
* Every row was updated, because there was no WHERE clause
- Only the first row changed
- The table was deleted
? Always write the WHERE first, and use transactions.

@@quiz
Which ACID property means a transfer either fully happens or not at all?
* Atomicity
- Consistency
- Isolation
- Durability
? All or nothing.

@@quiz
A CSV of product codes like `00123` loses its leading zeros. What happened?
- The CSV was corrupted
* Excel opened it and treated the codes as numbers; import the column as text
- CSV files can't store zeros
- The encoding was wrong
? Use Data → From Text/CSV and set the column type.

@@flashcards
table / row / column :: relational structure
primary key :: uniquely identifies a row
foreign key :: references another table's key
normalisation :: store each fact once
SQL :: language for relational databases
SELECT / WHERE / GROUP BY :: pick, filter, summarise
JOIN :: combine rows from tables
transaction :: changes that succeed or fail together
ACID :: atomicity, consistency, isolation, durability
NoSQL :: non-relational databases (document, key-value, graph...)
Redis :: in-memory key-value store
index :: speeds up lookups
CSV / JSON / YAML :: common data formats
APPI / GDPR :: Japanese / EU privacy laws

@@drill
Which SQL clause filters rows?
= WHERE
? HAVING filters groups after GROUP BY.

@@drill
What does the A in ACID stand for?
= atomicity
? All or nothing.

@@exercise Query a database with sqlite3
Python includes SQLite. Create an in-memory database with the `customers` and `orders` tables from the lesson, insert the sample rows, then run a query with `JOIN` and `GROUP BY` that prints each customer's name, number of orders and total spent, highest spender first.
@@lang python
@@starter
import sqlite3

db = sqlite3.connect(":memory:")
db.executescript("""
CREATE TABLE customers (id INTEGER PRIMARY KEY, name TEXT, city TEXT);
CREATE TABLE orders (id INTEGER PRIMARY KEY, customer_id INTEGER REFERENCES customers(id), total INTEGER, placed TEXT);
INSERT INTO customers VALUES (1, 'Sato Kenji', 'Osaka'), (2, 'Ana Silva', 'Lisbon'), (3, 'Priya Nair', 'Pune');
INSERT INTO orders VALUES (501, 1, 12000, '2026-03-01'), (502, 2, 4500, '2026-03-02'), (503, 1, 8000, '2026-03-04'), (504, 3, 15000, '2026-03-05');
""")
# your query here
@@expected
Sato Kenji: 2 orders, ¥20,000
Priya Nair: 1 order, ¥15,000
Ana Silva: 1 order, ¥4,500
@@requires
JOIN
GROUP BY
ORDER BY
@@hint
`for name, n, spent in db.execute("SELECT c.name, COUNT(*), SUM(o.total) FROM customers c JOIN orders o ON o.customer_id = c.id GROUP BY c.id ORDER BY SUM(o.total) DESC"): print(...)`
@@solution
import sqlite3

db = sqlite3.connect(":memory:")
db.executescript("""
CREATE TABLE customers (id INTEGER PRIMARY KEY, name TEXT, city TEXT);
CREATE TABLE orders (id INTEGER PRIMARY KEY, customer_id INTEGER REFERENCES customers(id), total INTEGER, placed TEXT);
INSERT INTO customers VALUES (1, 'Sato Kenji', 'Osaka'), (2, 'Ana Silva', 'Lisbon'), (3, 'Priya Nair', 'Pune');
INSERT INTO orders VALUES (501, 1, 12000, '2026-03-01'), (502, 2, 4500, '2026-03-02'), (503, 1, 8000, '2026-03-04'), (504, 3, 15000, '2026-03-05');
""")

query = """
SELECT c.name, COUNT(*) AS orders, SUM(o.total) AS spent
FROM customers c
JOIN orders o ON o.customer_id = c.id
GROUP BY c.id
ORDER BY spent DESC
"""
for name, orders, spent in db.execute(query):
    word = "order" if orders == 1 else "orders"
    print(f"{name}: {orders} {word}, ¥{spent:,}")

@@exercise CSV to JSON without losing zeros
Read the CSV text with the `csv` module (it handles the quoted comma in "Osaka, Umeda"), keep `asset_tag` as text so leading zeros survive, convert `price` to an integer, and print the result as JSON with `json.dumps(..., ensure_ascii=False, indent=2)`. Then print the total price of all assets.
@@lang python
@@starter
import csv, io, json

data = """asset_tag,model,location,price
00123,ThinkPad T14,"Osaka, Umeda",168000
00456,Dell P2723,Tokyo,42000
01007,Surface Pro,大阪,198000
"""
@@expected
[
  {
    "asset_tag": "00123",
    "model": "ThinkPad T14",
    "location": "Osaka, Umeda",
    "price": 168000
  },
  {
    "asset_tag": "00456",
    "model": "Dell P2723",
    "location": "Tokyo",
    "price": 42000
  },
  {
    "asset_tag": "01007",
    "model": "Surface Pro",
    "location": "大阪",
    "price": 198000
  }
]
Total: 408000
@@requires
csv.DictReader(
json.dumps(
ensure_ascii=False
@@hint
`rows = list(csv.DictReader(io.StringIO(data)))`, then `row["price"] = int(row["price"])` for each.
@@solution
import csv, io, json

data = """asset_tag,model,location,price
00123,ThinkPad T14,"Osaka, Umeda",168000
00456,Dell P2723,Tokyo,42000
01007,Surface Pro,大阪,198000
"""

rows = list(csv.DictReader(io.StringIO(data)))
for row in rows:
    row["price"] = int(row["price"])

print(json.dumps(rows, ensure_ascii=False, indent=2))
print("Total:", sum(row["price"] for row in rows))

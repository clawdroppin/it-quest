@@lesson s6-schema Designing a Database for an Application
@@concepts lists, errors
@@text
Behind almost every application is a database, and its **schema** (the tables, columns and rules) is the hardest part to change later. Code can be rewritten in a weekend; a table with 40 million rows and five applications reading it can't. This lesson is about designing schemas that keep bad data out from day one.

## Let the database enforce the rules

Application code has bugs, and sooner or later someone will write to the database with a script, an import or a second application. Rules written as **constraints** hold for all of them:

```sql
PRAGMA foreign_keys = ON;            -- SQLite: foreign keys are only enforced when this is on

CREATE TABLE customers (
    id         INTEGER PRIMARY KEY,
    email      TEXT    NOT NULL UNIQUE COLLATE NOCASE,          -- no duplicates, any case
    name       TEXT    NOT NULL CHECK (length(trim(name)) > 0),
    created_at TEXT    NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE orders (
    id          INTEGER PRIMARY KEY,
    customer_id INTEGER NOT NULL REFERENCES customers(id) ON DELETE RESTRICT,
    status      TEXT    NOT NULL DEFAULT 'new'
                CHECK (status IN ('new', 'paid', 'shipped', 'cancelled')),
    total_cents INTEGER NOT NULL CHECK (total_cents >= 0),       -- money as whole cents
    created_at  TEXT    NOT NULL DEFAULT (datetime('now'))
);

INSERT INTO customers (email, name) VALUES ('aiko@example.jp', 'Aiko Mori');
INSERT INTO orders (customer_id, total_cents) VALUES (1, 129900);
SELECT o.id, c.name, o.status, printf('%.2f', o.total_cents / 100.0) AS total
FROM orders o JOIN customers c ON c.id = o.customer_id;
```

| Constraint | Stops |
|---|---|
| `NOT NULL` | missing values |
| `UNIQUE` | duplicates (two accounts with one email) |
| `CHECK (...)` | impossible values (negative prices, unknown statuses) |
| `REFERENCES` (foreign key) | orders for customers that don't exist |
| `DEFAULT` | forgetting a value that has an obvious default |

## Design rules that save you later

- **Money is never a float.** `0.1 + 0.2` is `0.30000000000000004`. Store whole cents (or yen) in an `INTEGER`, or use `DECIMAL`/`NUMERIC` in PostgreSQL, SQL Server and MySQL.
- **Store time in UTC**, in one format (ISO 8601: `2026-11-01 09:30:00`), and convert to local time when displaying.
- **Surrogate keys**: an `id` with no meaning. Emails and usernames change; ids don't.
- **One fact in one place.** Don't store a customer's name in every order; store `customer_id`. (This is *normalization*.) Copy a value only on purpose: an order *should* keep the price at the time of purchase, even if the product's price changes.
- **Lists are rows, not columns.** Not `phone1, phone2, phone3`, but a `phones` table.
- **Many-to-many needs a join table**: products ↔ categories becomes `product_categories(product_id, category_id)` with both as the primary key.
- **Name things consistently**: plural table names, `snake_case`, foreign keys named `<table>_id`.

## What happens on delete?

A foreign key also says what to do when the parent row is deleted:

| Option | Deleting a customer... | Use for |
|---|---|---|
| `ON DELETE RESTRICT` (or no action) | is refused while they have orders | records you must keep (orders, invoices) |
| `ON DELETE CASCADE` | deletes their rows too | things that only exist as part of the parent (cart items, a ticket's comments) |
| `ON DELETE SET NULL` | keeps the rows, with the column set to NULL | optional links (a ticket's assignee leaves the company) |

Many applications never really delete important rows: they **soft delete** with a `deleted_at` column, so the history stays and mistakes can be undone.

## Enumerations: CHECK or a lookup table?

A short fixed list (`'new', 'paid', 'shipped'`) is fine as a `CHECK`. A list that business users change (product categories, office locations) belongs in its own table with a foreign key, so adding a value is an `INSERT`, not a schema change.

## Generated columns

A value computed from other columns can be declared once and never go out of date:

```sql
CREATE TABLE order_lines (
    order_id    INTEGER NOT NULL,
    product     TEXT    NOT NULL,
    qty         INTEGER NOT NULL CHECK (qty > 0),
    price_cents INTEGER NOT NULL,
    line_cents  INTEGER GENERATED ALWAYS AS (qty * price_cents) STORED
);
INSERT INTO order_lines (order_id, product, qty, price_cents) VALUES (1, 'USB-C dock', 2, 18900);
SELECT product, qty, line_cents FROM order_lines;
```

@@quiz
Why store prices as integer cents instead of a REAL/FLOAT column?
- Integers use less disk space
* Floating-point numbers can't represent most decimal fractions exactly, so sums drift
- SQLite has no REAL type
- Cents are required by law
? 0.1 + 0.2 = 0.30000000000000004 in floating point. Money must add up exactly.

@@quiz
A ticket has an optional assignee. The assignee's user account is deleted. What should the foreign key do?
- ON DELETE CASCADE: delete the ticket
* ON DELETE SET NULL: keep the ticket, now unassigned
- ON DELETE RESTRICT: refuse to delete the user forever
- Nothing: foreign keys don't apply to optional columns
? The ticket still matters; only the link goes away.

@@quiz
Why put rules in CHECK constraints when the application already validates input?
* Every writer is covered: other apps, imports, scripts and future bugs in this app
- CHECK constraints make queries faster
- Applications can't validate data
- It's required for foreign keys
? The database is the last line of defense, and it's shared by everything that touches it.

@@quiz
Products can be in several categories, and categories hold many products. How is that modelled?
- A `categories` TEXT column with comma-separated names
- `category1`, `category2`, `category3` columns on products
* A join table `product_categories(product_id, category_id)`
- One products table per category
? Many-to-many relationships always need a table in the middle.

@@flashcards
schema :: the tables, columns and constraints of a database
`NOT NULL` / `UNIQUE` / `CHECK` :: no missing values / no duplicates / only valid values
foreign key :: `REFERENCES parent(id)`: the parent must exist
`PRAGMA foreign_keys = ON` :: SQLite enforces foreign keys only with this
`ON DELETE CASCADE / SET NULL / RESTRICT` :: delete children / unlink them / refuse
surrogate key :: a meaningless `id` primary key
normalization :: one fact in one place
join table :: links two tables many-to-many
soft delete :: a `deleted_at` column instead of DELETE
money :: integer cents (or DECIMAL), never float
`GENERATED ALWAYS AS (...) STORED` :: a computed column
`COLLATE NOCASE` :: compare text ignoring case

@@exercise Constraints that refuse bad data
Write the `CREATE TABLE devices` statement so that the `INSERT OR IGNORE` statements (which skip any row that breaks a constraint) keep **only the valid devices**. Rules: `id` is the primary key; `hostname` is required and unique ignoring case; `kind` must be `laptop`, `desktop` or `phone`; `ram_gb` is required and must be between 4 and 256; `owner` is optional.
@@setup
PRAGMA foreign_keys = ON;
@@starter
-- TODO: CREATE TABLE devices (...);

INSERT OR IGNORE INTO devices (hostname, kind, ram_gb, owner) VALUES ('tk-lt-001', 'laptop', 16, 'aiko');
INSERT OR IGNORE INTO devices (hostname, kind, ram_gb, owner) VALUES ('TK-LT-001', 'laptop', 32, 'ken');
INSERT OR IGNORE INTO devices (hostname, kind, ram_gb, owner) VALUES ('tk-dt-007', 'desktop', 64, NULL);
INSERT OR IGNORE INTO devices (hostname, kind, ram_gb, owner) VALUES ('tk-tb-002', 'tablet', 8, 'yuki');
INSERT OR IGNORE INTO devices (hostname, kind, ram_gb, owner) VALUES (NULL, 'phone', 6, 'ken');
INSERT OR IGNORE INTO devices (hostname, kind, ram_gb, owner) VALUES ('tk-ph-003', 'phone', 6, 'ken');
INSERT OR IGNORE INTO devices (hostname, kind, ram_gb, owner) VALUES ('tk-lt-004', 'laptop', 2, 'mei');
INSERT OR IGNORE INTO devices (hostname, kind, ram_gb, owner) VALUES ('tk-lt-005', 'laptop', NULL, 'mei');

SELECT id, hostname, kind, ram_gb, owner FROM devices ORDER BY id;
@@expected
id | hostname  | kind    | ram_gb | owner
---+-----------+---------+--------+------
1  | tk-lt-001 | laptop  | 16     | aiko
2  | tk-dt-007 | desktop | 64     | NULL
3  | tk-ph-003 | phone   | 6      | ken
@@requires
CHECK
UNIQUE
NOT NULL
@@hint
`CREATE TABLE devices (id INTEGER PRIMARY KEY, hostname TEXT NOT NULL UNIQUE COLLATE NOCASE, kind TEXT NOT NULL CHECK (kind IN ('laptop', 'desktop', 'phone')), ram_gb INTEGER NOT NULL CHECK (ram_gb BETWEEN 4 AND 256), owner TEXT);`
@@solution
CREATE TABLE devices (
    id       INTEGER PRIMARY KEY,
    hostname TEXT    NOT NULL UNIQUE COLLATE NOCASE,
    kind     TEXT    NOT NULL CHECK (kind IN ('laptop', 'desktop', 'phone')),
    ram_gb   INTEGER NOT NULL CHECK (ram_gb BETWEEN 4 AND 256),
    owner    TEXT
);

INSERT OR IGNORE INTO devices (hostname, kind, ram_gb, owner) VALUES ('tk-lt-001', 'laptop', 16, 'aiko');
INSERT OR IGNORE INTO devices (hostname, kind, ram_gb, owner) VALUES ('TK-LT-001', 'laptop', 32, 'ken');
INSERT OR IGNORE INTO devices (hostname, kind, ram_gb, owner) VALUES ('tk-dt-007', 'desktop', 64, NULL);
INSERT OR IGNORE INTO devices (hostname, kind, ram_gb, owner) VALUES ('tk-tb-002', 'tablet', 8, 'yuki');
INSERT OR IGNORE INTO devices (hostname, kind, ram_gb, owner) VALUES (NULL, 'phone', 6, 'ken');
INSERT OR IGNORE INTO devices (hostname, kind, ram_gb, owner) VALUES ('tk-ph-003', 'phone', 6, 'ken');
INSERT OR IGNORE INTO devices (hostname, kind, ram_gb, owner) VALUES ('tk-lt-004', 'laptop', 2, 'mei');
INSERT OR IGNORE INTO devices (hostname, kind, ram_gb, owner) VALUES ('tk-lt-005', 'laptop', NULL, 'mei');

SELECT id, hostname, kind, ram_gb, owner FROM devices ORDER BY id;

@@exercise What happens on delete
`users` and `teams` exist. Create `tickets` (id, title, `reporter_id` that must be a user and whose tickets **disappear with the user**, and `assignee_id`, optional, which becomes **NULL** when that user is deleted) and `memberships` (user_id, team_id, both required foreign keys, primary key on the pair, and **deleting a team must be refused** while it has members). Foreign keys are switched on in the setup.
@@setup
PRAGMA foreign_keys = ON;
CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT NOT NULL);
CREATE TABLE teams (id INTEGER PRIMARY KEY, name TEXT NOT NULL);
INSERT INTO users VALUES (1, 'aiko'), (2, 'ken'), (3, 'yuki');
INSERT INTO teams VALUES (1, 'Network'), (2, 'Desktop');
@@starter
-- TODO: CREATE TABLE tickets (...);
-- TODO: CREATE TABLE memberships (...);

INSERT INTO tickets (id, title, reporter_id, assignee_id) VALUES
  (1, 'VPN drops', 1, 2), (2, 'New monitor', 2, NULL), (3, 'Printer jam', 3, 2), (4, 'Wi-Fi slow', 1, 3);
INSERT INTO memberships VALUES (2, 1), (3, 2);

DELETE FROM users WHERE id = 2;   -- ken leaves
DELETE FROM users WHERE id = 1;   -- aiko leaves
DELETE FROM teams WHERE id NOT IN (SELECT team_id FROM memberships);   -- only empty teams can go

SELECT id, title, reporter_id, assignee_id FROM tickets ORDER BY id;
SELECT t.name, COUNT(m.user_id) AS members FROM teams t LEFT JOIN memberships m ON m.team_id = t.id GROUP BY t.id ORDER BY t.id;
@@expected
id | title       | reporter_id | assignee_id
---+-------------+-------------+------------
3  | Printer jam | 3           | NULL

name    | members
--------+--------
Desktop | 1
@@requires
ON DELETE CASCADE
ON DELETE SET NULL
ON DELETE RESTRICT
@@hint
`reporter_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE`, `assignee_id INTEGER REFERENCES users(id) ON DELETE SET NULL`. In memberships: `user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE, team_id INTEGER NOT NULL REFERENCES teams(id) ON DELETE RESTRICT, PRIMARY KEY (user_id, team_id)`. Deleting ken also removes his membership, so the Network team is empty and the last DELETE removes it.
@@solution
CREATE TABLE tickets (
    id          INTEGER PRIMARY KEY,
    title       TEXT    NOT NULL,
    reporter_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    assignee_id INTEGER REFERENCES users(id) ON DELETE SET NULL
);
CREATE TABLE memberships (
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    team_id INTEGER NOT NULL REFERENCES teams(id) ON DELETE RESTRICT,
    PRIMARY KEY (user_id, team_id)
);

INSERT INTO tickets (id, title, reporter_id, assignee_id) VALUES
  (1, 'VPN drops', 1, 2), (2, 'New monitor', 2, NULL), (3, 'Printer jam', 3, 2), (4, 'Wi-Fi slow', 1, 3);
INSERT INTO memberships VALUES (2, 1), (3, 2);

DELETE FROM users WHERE id = 2;   -- ken leaves
DELETE FROM users WHERE id = 1;   -- aiko leaves
DELETE FROM teams WHERE id NOT IN (SELECT team_id FROM memberships);   -- only empty teams can go

SELECT id, title, reporter_id, assignee_id FROM tickets ORDER BY id;
SELECT t.name, COUNT(m.user_id) AS members FROM teams t LEFT JOIN memberships m ON m.team_id = t.id GROUP BY t.id ORDER BY t.id;

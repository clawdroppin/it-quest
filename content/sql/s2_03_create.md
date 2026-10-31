@@lesson s2-create Designing Tables: Types, Keys and Constraints
@@text
## CREATE TABLE

```sql
CREATE TABLE licenses (
    id         INTEGER PRIMARY KEY,
    product    TEXT NOT NULL,
    seats      INTEGER NOT NULL CHECK (seats > 0),
    expires    TEXT,
    cost_yen   INTEGER DEFAULT 0
);
INSERT INTO licenses (product, seats, expires) VALUES ('Microsoft 365 E3', 50, '2027-03-31');
SELECT * FROM licenses;
```

## Constraints: let the database enforce the rules

| Constraint | Guarantees |
|---|---|
| `PRIMARY KEY` | unique, not null: identifies the row |
| `NOT NULL` | a value is required |
| `UNIQUE` | no duplicates (`username`, `asset_tag`) |
| `CHECK (...)` | a condition, e.g. `priority BETWEEN 1 AND 4` |
| `DEFAULT x` | the value when none is given |
| `REFERENCES other(id)` | a **foreign key**: must point at an existing row |

Constraints are the last line of defence: whatever bug an application has, the database refuses data that breaks the rules.

```sql
CREATE TABLE seats (
    license_id  INTEGER NOT NULL REFERENCES licenses(id),
    employee_id INTEGER NOT NULL REFERENCES employees(id),
    PRIMARY KEY (license_id, employee_id)
);
```

A table like `seats` connects two tables in a **many-to-many** relationship (one license has many users; one user has many licenses). Its primary key is the *pair*, so nobody gets the same license twice.

(SQLite only enforces foreign keys after `PRAGMA foreign_keys = ON;`. Other databases always do.)

## Types

SQLite is relaxed about types; other databases are strict. Common ones: `INTEGER`, `DECIMAL(10,2)` for money (never floating point for money!), `VARCHAR(100)` / `TEXT`, `DATE`, `TIMESTAMP`, `BOOLEAN`.

## Changing and removing tables

```sql
ALTER TABLE devices ADD COLUMN location TEXT DEFAULT 'Tokyo';
SELECT asset_tag, location FROM devices LIMIT 2;
```

`DROP TABLE name;` deletes a table and all its data, with no undo. In production, schema changes go through **migrations**: versioned scripts, reviewed and tested like code.

## Indexes

An **index** is like the index of a book: it lets the database find rows without reading the whole table.

```sql
CREATE INDEX idx_logins_username ON logins(username);
EXPLAIN QUERY PLAN SELECT * FROM logins WHERE username = 'asilva';
```

Index the columns you search and join on. Each index costs disk space and makes writes a bit slower, so don't index everything. `UNIQUE` and `PRIMARY KEY` columns get one automatically.

@@quiz
Why put a `UNIQUE` constraint on `employees.username`?
* So the database itself refuses a duplicate username, even if an app has a bug
- To make the column faster to display
- Because usernames are text
- To sort by username
? Constraints guarantee rules no matter which program writes the data.

@@quiz
What type should money use?
* An exact type like DECIMAL or integer (yen, cents)
- FLOAT
- TEXT
- BOOLEAN
? Floating point can't represent many decimals exactly: 0.1 + 0.2 isn't 0.3.

@@flashcards
PRIMARY KEY :: Uniquely identifies each row; unique and not null
FOREIGN KEY (REFERENCES) :: Must point at an existing row in another table
UNIQUE / NOT NULL / CHECK :: No duplicates / value required / custom rule
junction table :: Connects two tables many-to-many, e.g. seats(license_id, employee_id)
index :: Speeds up searches on a column; costs space and write speed
ALTER TABLE :: Change a table's structure
DROP TABLE :: Delete a table and its data
migration :: A versioned script that changes the schema

@@exercise Software licences
Create a table `software` with: `id` integer primary key, `name` text not null **unique**, `vendor` text not null, `seats` integer not null that must be **at least 1**. Insert `Slack` by `Salesforce` with 40 seats and `Zoom` by `Zoom Video` with 25 seats, then show all rows ordered by name.
@@starter
-- CREATE TABLE, INSERT, SELECT

@@expected
id | name  | vendor     | seats
---+-------+------------+------
1  | Slack | Salesforce | 40
2  | Zoom  | Zoom Video | 25
@@requires
UNIQUE
CHECK
@@hint
`seats INTEGER NOT NULL CHECK (seats >= 1)`. One INSERT can add both rows: `VALUES ('Slack', 'Salesforce', 40), ('Zoom', 'Zoom Video', 25)`.
@@solution
CREATE TABLE software (
    id     INTEGER PRIMARY KEY,
    name   TEXT NOT NULL UNIQUE,
    vendor TEXT NOT NULL,
    seats  INTEGER NOT NULL CHECK (seats >= 1)
);
INSERT INTO software (name, vendor, seats) VALUES ('Slack', 'Salesforce', 40), ('Zoom', 'Zoom Video', 25);
SELECT * FROM software ORDER BY name;

@@exercise Track who uses what
Create a junction table `installs` with `device_id` (references devices) and `software` (text), whose primary key is the **pair**. Record that devices 1 and 2 have `Slack`, and device 1 also has `Zoom`. Then show each device's `asset_tag` and its number of installs as `installs`, ordered by asset tag.
@@starter
-- CREATE TABLE installs (...), INSERT, then a JOIN + GROUP BY

@@expected
asset_tag | installs
----------+---------
LT-0042   | 2
LT-0043   | 1
@@requires
PRIMARY KEY (
@@hint
`PRIMARY KEY (device_id, software)` at the end of the column list. Then `FROM installs AS i JOIN devices AS d ON d.id = i.device_id GROUP BY d.asset_tag`.
@@solution
CREATE TABLE installs (
    device_id INTEGER NOT NULL REFERENCES devices(id),
    software  TEXT NOT NULL,
    PRIMARY KEY (device_id, software)
);
INSERT INTO installs VALUES (1, 'Slack'), (2, 'Slack'), (1, 'Zoom');
SELECT d.asset_tag, COUNT(*) AS installs
FROM installs AS i
JOIN devices AS d ON d.id = i.device_id
GROUP BY d.asset_tag
ORDER BY d.asset_tag;

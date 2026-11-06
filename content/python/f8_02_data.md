@@lesson f8-data Application Data: SQLite, Repositories and Migrations
@@text
Real applications keep their data in a **database**. Python ships with `sqlite3`: a full SQL database in one file, with no server to install. It's in every phone, every browser and lots of desktop apps, and it's a perfect database for small web apps too. Everything you learn here carries over to PostgreSQL and MySQL.

## Connect, create, insert, query

```python
import sqlite3

conn = sqlite3.connect(":memory:")          # a file name like "app.db" keeps the data
conn.row_factory = sqlite3.Row             # rows behave like dicts: row["name"]
conn.execute("PRAGMA foreign_keys = ON")   # SQLite needs this to enforce REFERENCES

conn.executescript("""
CREATE TABLE users (
    id    INTEGER PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    name  TEXT NOT NULL
);
""")

with conn:   # a transaction: commits at the end, rolls back if an exception happens
    conn.execute("INSERT INTO users (email, name) VALUES (?, ?)", ("yuki@example.jp", "Yuki"))
    conn.execute("INSERT INTO users (email, name) VALUES (?, ?)", ("ken@example.jp", "Ken"))

for row in conn.execute("SELECT id, name FROM users ORDER BY name"):
    print(row["id"], row["name"])
```

## Always use parameters

The `?` placeholders are not decoration. They're what stops **SQL injection**, the classic way web apps get hacked:

```python
import sqlite3

conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE users (name TEXT, is_admin INTEGER)")
conn.execute("INSERT INTO users VALUES ('yuki', 1), ('ken', 0)")

attack = "nobody' OR '1'='1"

bad = f"SELECT name FROM users WHERE name = '{attack}'"         # NEVER
print("string-built:", conn.execute(bad).fetchall())             # returns every user!

good = conn.execute("SELECT name FROM users WHERE name = ?", (attack,)).fetchall()
print("parameterized:", good)                                    # nothing: no user has that name
```

The rule has no exceptions: values go in parameters. If you need a dynamic *column* or *ORDER BY*, pick it from a fixed allow-list in your code, never from user text.

## Transactions

Transferring stock from one warehouse to another is two updates. If the program crashes between them, stock disappears. A **transaction** makes them all-or-nothing:

```python
import sqlite3

conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE stock (site TEXT PRIMARY KEY, laptops INTEGER CHECK (laptops >= 0))")
conn.execute("INSERT INTO stock VALUES ('tokyo', 5), ('osaka', 1)")
conn.commit()

def transfer(n, src, dst):
    with conn:
        conn.execute("UPDATE stock SET laptops = laptops + ? WHERE site = ?", (n, dst))
        conn.execute("UPDATE stock SET laptops = laptops - ? WHERE site = ?", (n, src))

transfer(2, "tokyo", "osaka")
try:
    transfer(9, "tokyo", "osaka")      # the CHECK fails on the 2nd update...
except sqlite3.IntegrityError as e:
    print("refused:", e)
print(conn.execute("SELECT * FROM stock").fetchall())   # ...and the 1st update was rolled back too
```

## The repository pattern

Keep all SQL in one class per kind of thing. The rest of the app calls methods like `repo.get(7)` and gets **domain objects** back, never rows or SQL:

```python
import sqlite3
from dataclasses import dataclass


@dataclass
class Device:
    hostname: str
    owner: str
    id: int | None = None


class DeviceRepo:
    def __init__(self, conn):
        self.conn = conn

    def add(self, d: Device) -> Device:
        with self.conn:
            d.id = self.conn.execute("INSERT INTO devices (hostname, owner) VALUES (?, ?)", (d.hostname, d.owner)).lastrowid
        return d

    def by_owner(self, owner: str) -> list[Device]:
        rows = self.conn.execute("SELECT id, hostname, owner FROM devices WHERE owner = ? ORDER BY hostname", (owner,))
        return [Device(id=r[0], hostname=r[1], owner=r[2]) for r in rows]


conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE devices (id INTEGER PRIMARY KEY, hostname TEXT, owner TEXT)")
repo = DeviceRepo(conn)
repo.add(Device("pc-0142", "yuki"))
repo.add(Device("mac-0007", "yuki"))
print(repo.by_owner("yuki"))
```

When you later move to PostgreSQL or an ORM, only the repository changes. Tests can use a real SQLite `:memory:` database (fast, and it tests your SQL) or a fake repo (even faster).

## Migrations: changing the schema safely

Your app is in use, and now you need a new column. You can't delete the database and start again: it holds everybody's data. A **migration** is a numbered, permanent change script. The app remembers which ones have run and applies only the new ones. SQLite has a free integer in every database file for exactly this: `PRAGMA user_version`.

```python
import sqlite3

MIGRATIONS = [
    "CREATE TABLE notes (id INTEGER PRIMARY KEY, text TEXT NOT NULL);",
    "ALTER TABLE notes ADD COLUMN pinned INTEGER NOT NULL DEFAULT 0;",
    "CREATE INDEX idx_notes_pinned ON notes(pinned);",
]

def migrate(conn):
    version = conn.execute("PRAGMA user_version").fetchone()[0]
    for number, sql in enumerate(MIGRATIONS[version:], start=version + 1):
        conn.executescript(f"BEGIN; {sql} PRAGMA user_version = {number}; COMMIT;")
        print("applied migration", number)

conn = sqlite3.connect(":memory:")
migrate(conn)          # a new database gets all three
migrate(conn)          # running again does nothing
print([r[1] for r in conn.execute("PRAGMA table_info(notes)")])
```

Rules that keep migrations safe:

- **Never edit a migration that has shipped**: databases in the world already ran it. Add a new one.
- Every migration runs in a transaction with its version bump.
- Back up before migrating production. Test migrations on a copy of real data.
- Bigger projects use a tool: **Alembic** (with SQLAlchemy), Django's `makemigrations`, Flyway or Liquibase. Same idea.

## ORMs

An **ORM** (Object-Relational Mapper) such as **SQLAlchemy**, **Django ORM** or **SQLModel** writes the SQL for you from classes:

```python,norun
# SQLAlchemy 2.0 style
class Device(Base):
    __tablename__ = "devices"
    id: Mapped[int] = mapped_column(primary_key=True)
    hostname: Mapped[str] = mapped_column(unique=True)

devices = session.scalars(select(Device).where(Device.owner == "yuki")).all()
```

ORMs save lots of typing, and they parameterize for you. Learn SQL anyway: when a page is slow, the answer is in the SQL the ORM generated (look for the "N+1 queries" problem: one query for a list, then one more per item).

@@quiz
What does `with conn:` do with a sqlite3 connection?
- Opens and closes the database file
* Runs the block as a transaction: commit at the end, roll back on an exception
- Locks the database for other programs forever
- Turns on foreign keys
? The connection's context manager commits or rolls back. (It doesn't close the connection.)

@@quiz
Your app is live. You need a new `due` column. What's the right way?
- Edit the first CREATE TABLE migration to include `due`
- Delete the database file and let the app recreate it
* Add a new migration `ALTER TABLE tasks ADD COLUMN due TEXT`
- Ask every user to run the SQL themselves
? Migrations that already ran are history. A new migration upgrades every existing database, and new ones get all the migrations in order.

@@quiz
A user can sort a table by any column. How do you safely put the column in the SQL?
- With a `?` parameter
- With an f-string, after removing quotes from it
* Check it against a fixed list of allowed column names in your code
- Lower-case it first
? Parameters work for values only, not for column names. An allow-list means only text you wrote yourself ever becomes SQL.

@@quiz
What's the "N+1 queries" problem?
- Having one more table than needed
* Loading a list with 1 query, then running 1 more query per item, so the page gets slower as data grows
- A transaction that runs one time too many
- A migration numbering mistake
? 100 tickets → 101 queries. The fix is a JOIN, or the ORM's eager loading (`selectinload`, `select_related`).

@@drill
Which character is SQLite's placeholder for a query parameter in Python's sqlite3?
= ?
? cur.execute("SELECT * FROM t WHERE id = ?", (id,)).

@@flashcards
`sqlite3.connect("app.db")` :: open or create a SQLite database file
`conn.row_factory = sqlite3.Row` :: rows can be read by column name
`PRAGMA foreign_keys = ON` :: SQLite enforces REFERENCES only with this
`?` placeholders :: values passed separately: prevents SQL injection
transaction :: a group of changes that all happen or none do
repository pattern :: one class holds all the SQL for one kind of object
migration :: a numbered, permanent schema change; new ones are added, old ones never edited
`PRAGMA user_version` :: an integer in a SQLite file, handy as the migration version
ORM :: maps classes to tables: SQLAlchemy, Django ORM, Entity Framework
N+1 queries :: one query per list item: fix with a JOIN or eager loading

@@exercise Migrate a database
Write `migrate(conn, migrations)`, which applies the migrations (a list of SQL strings) that haven't run yet, using `PRAGMA user_version` as the count of migrations already applied. Each migration must run in one transaction with its version bump. Return the number of migrations it applied **this time**.
@@starter
import sqlite3


def migrate(conn, migrations):
    pass


STEPS = [
    "CREATE TABLE assets (id INTEGER PRIMARY KEY, tag TEXT NOT NULL UNIQUE);",
    "ALTER TABLE assets ADD COLUMN owner TEXT;",
]
conn = sqlite3.connect(":memory:")
print(migrate(conn, STEPS), migrate(conn, STEPS))
@@tests
_c = sqlite3.connect(":memory:")
_steps = ["CREATE TABLE t (id INTEGER PRIMARY KEY);", "ALTER TABLE t ADD COLUMN name TEXT;"]
check(migrate(_c, _steps), 2, "a new database gets both migrations")
check(_c.execute("PRAGMA user_version").fetchone()[0], 2, "user_version is 2")
check(migrate(_c, _steps), 0, "running again applies nothing")
_steps.append("ALTER TABLE t ADD COLUMN email TEXT;")
check(migrate(_c, _steps), 1, "only the new migration runs")
check([r[1] for r in _c.execute("PRAGMA table_info(t)")], ["id", "name", "email"], "the final columns")
_steps.append("ALTER TABLE nope ADD COLUMN x TEXT;")
try:
    migrate(_c, _steps)
except sqlite3.Error:
    pass
check(_c.execute("PRAGMA user_version").fetchone()[0], 3, "a failed migration doesn't bump the version")
@@hint
`version = conn.execute("PRAGMA user_version").fetchone()[0]`, then loop over `enumerate(migrations[version:], start=version + 1)` and run `conn.executescript(f"BEGIN; {sql} PRAGMA user_version = {number}; COMMIT;")`. Count how many you ran. If a migration fails, `executescript` raises before the COMMIT; call `conn.rollback()` in an `except` and re-raise.
@@solution
import sqlite3


def migrate(conn, migrations):
    version = conn.execute("PRAGMA user_version").fetchone()[0]
    applied = 0
    for number, sql in enumerate(migrations[version:], start=version + 1):
        try:
            conn.executescript(f"BEGIN; {sql} PRAGMA user_version = {number}; COMMIT;")
        except sqlite3.Error:
            conn.rollback()
            raise
        applied += 1
    return applied


STEPS = [
    "CREATE TABLE assets (id INTEGER PRIMARY KEY, tag TEXT NOT NULL UNIQUE);",
    "ALTER TABLE assets ADD COLUMN owner TEXT;",
]
conn = sqlite3.connect(":memory:")
print(migrate(conn, STEPS), migrate(conn, STEPS))

@@exercise A search with optional filters
Write `search(conn, text=None, site=None, min_ram=None)` for a `laptops` table (`id`, `model`, `site`, `ram_gb`). Each filter that isn't `None` narrows the result: `text` matches anywhere in `model` (case-insensitive), `site` must be equal, `min_ram` is a minimum. Return a list of models ordered by `id`. Use `?` parameters for **every** value.
@@starter
import sqlite3


def search(conn, text=None, site=None, min_ram=None):
    pass


conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE laptops (id INTEGER PRIMARY KEY, model TEXT, site TEXT, ram_gb INTEGER)")
conn.executemany("INSERT INTO laptops (model, site, ram_gb) VALUES (?, ?, ?)", [
    ("ThinkPad X1", "tokyo", 16), ("Let's note SV", "osaka", 8),
    ("ThinkPad T14", "osaka", 32), ("MacBook Air", "tokyo", 16),
])
print(search(conn, text="thinkpad"), search(conn, site="osaka", min_ram=16))
@@tests
_c = sqlite3.connect(":memory:")
_c.execute("CREATE TABLE laptops (id INTEGER PRIMARY KEY, model TEXT, site TEXT, ram_gb INTEGER)")
_c.executemany("INSERT INTO laptops (model, site, ram_gb) VALUES (?, ?, ?)", [
    ("ThinkPad X1", "tokyo", 16), ("Let's note SV", "osaka", 8),
    ("ThinkPad T14", "osaka", 32), ("MacBook Air", "tokyo", 16),
])
check(search(_c), ["ThinkPad X1", "Let's note SV", "ThinkPad T14", "MacBook Air"], "no filters: everything")
check(search(_c, text="THINKPAD"), ["ThinkPad X1", "ThinkPad T14"], "text, any case")
check(search(_c, site="osaka", min_ram=16), ["ThinkPad T14"], "two filters together")
check(search(_c, text="let's"), ["Let's note SV"], "a quote in the search text")
check(search(_c, site="x' OR '1'='1"), [], "injection attempt finds nothing")
@@hint
Start with `sql = "SELECT model FROM laptops WHERE 1 = 1"` and `params = []`. For each filter, add `" AND LOWER(model) LIKE ?"` with `params.append(f"%{text.lower()}%")`, `" AND site = ?"`, `" AND ram_gb >= ?"`. Finish with `" ORDER BY id"` and `[r[0] for r in conn.execute(sql, params)]`.
@@solution
import sqlite3


def search(conn, text=None, site=None, min_ram=None):
    sql = "SELECT model FROM laptops WHERE 1 = 1"
    params = []
    if text is not None:
        sql += " AND LOWER(model) LIKE ?"
        params.append(f"%{text.lower()}%")
    if site is not None:
        sql += " AND site = ?"
        params.append(site)
    if min_ram is not None:
        sql += " AND ram_gb >= ?"
        params.append(min_ram)
    sql += " ORDER BY id"
    return [row[0] for row in conn.execute(sql, params)]


conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE laptops (id INTEGER PRIMARY KEY, model TEXT, site TEXT, ram_gb INTEGER)")
conn.executemany("INSERT INTO laptops (model, site, ram_gb) VALUES (?, ?, ?)", [
    ("ThinkPad X1", "tokyo", 16), ("Let's note SV", "osaka", 8),
    ("ThinkPad T14", "osaka", 32), ("MacBook Air", "tokyo", 16),
])
print(search(conn, text="thinkpad"), search(conn, site="osaka", min_ram=16))

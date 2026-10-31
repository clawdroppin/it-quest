@@lesson t1-asset-inventory IT Asset Inventory
@@text
Every IT department has to answer the same questions, usually urgently: *Who has laptop LT-0052? Which warranties run out this quarter? How much did we spend on phones? Where did the laptop of the person who left last month go?* Small companies answer them from a spreadsheet that's always out of date. You'll build the real thing: a **database** with rules that keep the data honest, and a command-line tool on top of it.

```text
python main.py init --seed
python main.py devices --search thinkpad
python main.py assign LT-0003 msato
python main.py return LT-0003 --status repair
python main.py warranty --days 90
python main.py history LT-0003
```

## The design

| Table | Holds |
|---|---|
| `people` | who can receive equipment (`username` is unique) |
| `devices` | one row per physical thing, with a `status`: in-stock, assigned, repair or retired |
| `assignments` | the **history**: which device went to whom, when, and when it came back |

Why a separate `assignments` table instead of an `owner` column on `devices`? Because a column only knows the *current* owner. Auditors, security investigations and "where's the laptop Ben had?" all need the history. A row with `returned IS NULL` is an assignment that's still open.

The database enforces the rules itself:

- `asset_tag` must look like `LT-0001`: a `CHECK` with `GLOB '[A-Z][A-Z]-[0-9][0-9][0-9][0-9]'`.
- `type` and `status` must be one of a fixed list; `price` can't be negative.
- Foreign keys: an assignment must point at a real device and a real person (SQLite needs `PRAGMA foreign_keys = ON`).
- **One holder at a time**: a *partial unique index* `ON assignments(device_id) WHERE returned IS NULL` makes it impossible to have two open assignments for one device, whatever bug the Python code has.

## The code

`inventory.py` holds small functions that take a database connection. **Every query uses `?` or `:name` placeholders**, never f-strings: the search box is exactly where an attacker would type `' OR '1'='1`. Changes that touch two tables (assign: add an assignment *and* change the device's status) run in a **transaction**, so they can't half-happen.

Tests use `connect(":memory:")`: a fresh, empty database in memory for each test, so they're fast and never touch your real data.

@@project inventory IT Asset Inventory
1. Finish `schema.sql`: write the `assignments` table and the one-holder index (the comments say what they need).
2. Write the functions in `inventory.py` until `python -m unittest` passes.
3. Run `python main.py init --seed` and try the commands above. Open `inventory.db` in [DB Browser for SQLite](https://sqlitebrowser.org/) to look inside.
@@file schema.sql
-- IT asset inventory. Run on every start, so everything is "IF NOT EXISTS".

CREATE TABLE IF NOT EXISTS people (
    id         INTEGER PRIMARY KEY,
    name       TEXT NOT NULL,
    username   TEXT NOT NULL UNIQUE,
    department TEXT
);

CREATE TABLE IF NOT EXISTS devices (
    id             INTEGER PRIMARY KEY,
    asset_tag      TEXT NOT NULL UNIQUE CHECK (asset_tag GLOB '[A-Z][A-Z]-[0-9][0-9][0-9][0-9]'),
    type           TEXT NOT NULL CHECK (type IN ('laptop', 'desktop', 'monitor', 'phone', 'tablet', 'printer', 'network', 'server')),
    model          TEXT NOT NULL,
    serial         TEXT UNIQUE,
    purchased      TEXT NOT NULL,
    price          INTEGER NOT NULL CHECK (price >= 0),
    warranty_until TEXT,
    status         TEXT NOT NULL DEFAULT 'in-stock' CHECK (status IN ('in-stock', 'assigned', 'repair', 'retired'))
);

-- TODO: the assignments table:
--   id          integer primary key
--   device_id   required, references devices(id)
--   person_id   required, references people(id)
--   assigned    required text (a date)
--   returned    text, NULL while the person still has the device

-- TODO: a UNIQUE index on assignments(device_id), only for rows WHERE returned IS NULL,
--   so a device can never have two open assignments.
@@file inventory.py
"""An IT asset inventory on SQLite. Every function takes an open connection."""

import sqlite3
from contextlib import contextmanager
from pathlib import Path

SCHEMA = Path(__file__).with_name("schema.sql")


def connect(path="inventory.db"):
    """Open (or create) the database, with foreign keys on and rows usable like dicts."""
    con = sqlite3.connect(path, isolation_level=None)  # autocommit; transaction() groups changes
    con.row_factory = sqlite3.Row
    con.execute("PRAGMA foreign_keys = ON")
    con.executescript(SCHEMA.read_text(encoding="utf-8"))
    return con


@contextmanager
def transaction(con):
    """with transaction(con): ... commits at the end, or rolls back if anything raises."""
    con.execute("BEGIN")
    try:
        yield
    except BaseException:
        con.execute("ROLLBACK")
        raise
    con.execute("COMMIT")


def add_person(con, name, username, department=None):
    """Insert a person; return their id."""
    raise NotImplementedError("TODO: add_person")


def add_device(con, asset_tag, type, model, serial, purchased, price, warranty_until=None):
    """Insert a device (status in-stock); return its id. Bad data raises sqlite3.IntegrityError."""
    raise NotImplementedError("TODO: add_device")


def assign(con, asset_tag, username, day):
    """Give an in-stock device to a person, in one transaction: add an assignment
    and set the device's status to 'assigned'.
    Raise ValueError if the device or person doesn't exist, or the device isn't in-stock."""
    raise NotImplementedError("TODO: assign")


def return_device(con, asset_tag, day, status="in-stock"):
    """Close the device's open assignment (returned = day) and set its status
    (in-stock, repair or retired), in one transaction. ValueError if it isn't assigned."""
    raise NotImplementedError("TODO: return_device")


def holder(con, asset_tag):
    """The username who has the device now, or None."""
    raise NotImplementedError("TODO: holder")


def history(con, asset_tag):
    """[(username, assigned, returned), ...] for the device, oldest first."""
    raise NotImplementedError("TODO: history")


def warranty_report(con, today, days=90):
    """Devices that aren't retired whose warranty has ended or ends within `days` days of `today`:
    [{"asset_tag", "model", "warranty_until", "state"}, ...], state "expired" or "expiring",
    ordered by warranty_until, then asset_tag."""
    raise NotImplementedError("TODO: warranty_report")


def spend_by_type(con):
    """[(type, devices, total_price), ...] for devices that aren't retired, biggest total first."""
    raise NotImplementedError("TODO: spend_by_type")


def search(con, text):
    """Devices whose asset tag, model, serial or current holder's username contains `text`
    (any case): [{"asset_tag", "model", "status", "holder"}, ...] ordered by asset tag.
    holder is None for devices nobody has."""
    raise NotImplementedError("TODO: search")
@@file test_inventory.py
import sqlite3
import unittest

from inventory import (add_device, add_person, assign, connect, history, holder, return_device,
                       search, spend_by_type, transaction, warranty_report)


class InventoryTests(unittest.TestCase):
    def setUp(self):
        self.con = connect(":memory:")
        add_person(self.con, "Yuki Tanaka", "ytanaka", "IT")
        add_person(self.con, "Mei Sato", "msato", "Sales")
        add_device(self.con, "LT-0001", "laptop", "ThinkPad T14", "SN1", "2023-04-01", 189000, "2026-03-31")
        add_device(self.con, "LT-0002", "laptop", "ThinkPad T14", "SN2", "2024-04-01", 195000, "2026-12-15")
        add_device(self.con, "MN-0001", "monitor", "Dell P2723", "SN3", "2024-05-10", 42000, "2029-05-10")
        add_device(self.con, "PH-0001", "phone", "iPhone 15", "SN4", "2024-09-01", 124800)

    def tearDown(self):
        self.con.close()

    def status(self, tag):
        return self.con.execute("SELECT status FROM devices WHERE asset_tag = ?", (tag,)).fetchone()[0]

    def test_constraints(self):
        bad = [
            ("LT-0001", "laptop", "dup tag", "SN9", "2024-01-01", 1000),
            ("laptop1", "laptop", "bad tag", "SN9", "2024-01-01", 1000),
            ("TB-0001", "toaster", "bad type", "SN9", "2024-01-01", 1000),
            ("TB-0001", "tablet", "negative", "SN9", "2024-01-01", -5),
        ]
        for row in bad:
            with self.assertRaises(sqlite3.IntegrityError, msg=row[2]):
                add_device(self.con, *row)

    def test_assign_and_return(self):
        assign(self.con, "LT-0002", "ytanaka", "2026-10-01")
        self.assertEqual(holder(self.con, "LT-0002"), "ytanaka")
        self.assertEqual(self.status("LT-0002"), "assigned")
        with self.assertRaises(ValueError):
            assign(self.con, "LT-0002", "msato", "2026-10-02")
        return_device(self.con, "LT-0002", "2026-10-20", status="repair")
        self.assertIsNone(holder(self.con, "LT-0002"))
        self.assertEqual(self.status("LT-0002"), "repair")
        with self.assertRaises(ValueError):
            return_device(self.con, "LT-0002", "2026-10-21")

    def test_history(self):
        assign(self.con, "LT-0001", "ytanaka", "2026-01-10")
        return_device(self.con, "LT-0001", "2026-05-01")
        assign(self.con, "LT-0001", "msato", "2026-05-02")
        self.assertEqual(history(self.con, "LT-0001"),
                         [("ytanaka", "2026-01-10", "2026-05-01"), ("msato", "2026-05-02", None)])

    def test_unknown_device_or_person(self):
        with self.assertRaises(ValueError):
            assign(self.con, "LT-9999", "ytanaka", "2026-10-01")
        with self.assertRaises(ValueError):
            assign(self.con, "LT-0001", "nobody", "2026-10-01")
        self.assertEqual(self.status("LT-0001"), "in-stock")

    def test_schema_allows_one_holder_only(self):
        self.con.execute("INSERT INTO assignments (device_id, person_id, assigned) VALUES (1, 1, '2026-10-01')")
        with self.assertRaises(sqlite3.IntegrityError):
            self.con.execute("INSERT INTO assignments (device_id, person_id, assigned) VALUES (1, 2, '2026-10-02')")

    def test_schema_foreign_keys(self):
        with self.assertRaises(sqlite3.IntegrityError):
            self.con.execute("INSERT INTO assignments (device_id, person_id, assigned) VALUES (99, 1, '2026-10-01')")

    def test_transaction_rolls_back(self):
        with self.assertRaises(RuntimeError):
            with transaction(self.con):
                add_person(self.con, "Ghost", "ghost")
                raise RuntimeError("boom")
        self.assertIsNone(self.con.execute("SELECT id FROM people WHERE username = 'ghost'").fetchone())

    def test_warranty_report(self):
        self.assertEqual(warranty_report(self.con, "2026-10-31", days=90), [
            {"asset_tag": "LT-0001", "model": "ThinkPad T14", "warranty_until": "2026-03-31", "state": "expired"},
            {"asset_tag": "LT-0002", "model": "ThinkPad T14", "warranty_until": "2026-12-15", "state": "expiring"},
        ])
        self.assertEqual(len(warranty_report(self.con, "2026-10-31", days=30)), 1)

    def test_retired_devices_are_left_out(self):
        assign(self.con, "LT-0001", "ytanaka", "2026-01-10")
        return_device(self.con, "LT-0001", "2026-10-30", status="retired")
        self.assertEqual([r["asset_tag"] for r in warranty_report(self.con, "2026-10-31")], ["LT-0002"])
        self.assertEqual(spend_by_type(self.con),
                         [("laptop", 1, 195000), ("phone", 1, 124800), ("monitor", 1, 42000)])

    def test_spend_by_type(self):
        self.assertEqual(spend_by_type(self.con),
                         [("laptop", 2, 384000), ("phone", 1, 124800), ("monitor", 1, 42000)])

    def test_search(self):
        assign(self.con, "LT-0002", "msato", "2026-10-01")
        self.assertEqual([r["asset_tag"] for r in search(self.con, "thinkpad")], ["LT-0001", "LT-0002"])
        self.assertEqual(search(self.con, "msato"),
                         [{"asset_tag": "LT-0002", "model": "ThinkPad T14", "status": "assigned", "holder": "msato"}])
        self.assertEqual(search(self.con, "SN3")[0]["holder"], None)

    def test_search_is_not_injectable(self):
        self.assertEqual(search(self.con, "' OR '1'='1"), [])
        self.assertEqual(search(self.con, "x'; DROP TABLE devices; --"), [])
        self.assertEqual(len(search(self.con, "")), 4)
@@file seed.sql
-- Sample data for "python main.py init --seed".
INSERT INTO people (name, username, department) VALUES
    ('Yuki Tanaka', 'ytanaka', 'IT'),
    ('Mei Sato', 'msato', 'Sales'),
    ('Kenji Watanabe', 'kwatanabe', 'Finance'),
    ('Ana Silva', 'asilva', 'Development');

INSERT INTO devices (asset_tag, type, model, serial, purchased, price, warranty_until) VALUES
    ('LT-0001', 'laptop', 'ThinkPad T14 Gen 3', 'PF3AB001', '2022-04-01', 189000, '2025-03-31'),
    ('LT-0002', 'laptop', 'ThinkPad T14 Gen 4', 'PF4CD002', '2024-04-01', 198000, '2027-03-31'),
    ('LT-0003', 'laptop', 'ThinkPad X1 Carbon', 'PF5EF003', '2025-06-10', 265000, '2028-06-10'),
    ('LT-0004', 'laptop', 'MacBook Pro 14', 'C02GH004', '2024-09-20', 298800, '2027-09-20'),
    ('MN-0001', 'monitor', 'Dell P2723QE', 'CN0J1005', '2023-05-10', 52000, '2026-11-30'),
    ('MN-0002', 'monitor', 'Dell P2723QE', 'CN0J1006', '2023-05-10', 52000, '2026-11-30'),
    ('PH-0001', 'phone', 'iPhone 15', 'F2LK0007', '2024-10-01', 124800, NULL),
    ('PR-0001', 'printer', 'Brother MFC-L8900CDW', 'E7896008', '2021-02-15', 89000, '2024-02-15'),
    ('NW-0001', 'network', 'Yamaha RTX1300', 'S4X00009', '2024-01-20', 156000, '2029-01-20');
@@file main.py
"""Command line for the inventory. python main.py --help"""

import argparse
import sys
from datetime import date
from pathlib import Path

import inventory


def show(rows, columns):
    """Print dicts or tuples as an aligned table."""
    rows = [dict(zip(columns, r)) if isinstance(r, tuple) else r for r in rows]
    if not rows:
        print("(nothing)")
        return
    text = [[("" if r[c] is None else str(r[c])) for c in columns] for r in rows]
    widths = [max(len(c), *(len(t[i]) for t in text)) for i, c in enumerate(columns)]
    print("  ".join(c.ljust(w) for c, w in zip(columns, widths)))
    print("  ".join("-" * w for w in widths))
    for t in text:
        print("  ".join(v.ljust(w) for v, w in zip(t, widths)))


def cmd_init(con, args):
    if args.seed:
        if con.execute("SELECT COUNT(*) FROM devices").fetchone()[0]:
            print("The database already has devices; not seeding.")
            return 1
        con.executescript(Path(__file__).with_name("seed.sql").read_text(encoding="utf-8"))
        inventory.assign(con, "LT-0002", "msato", "2024-04-05")
        inventory.assign(con, "LT-0004", "asilva", "2024-09-25")
        inventory.assign(con, "MN-0001", "msato", "2024-04-05")
        inventory.assign(con, "LT-0001", "kwatanabe", "2022-04-03")
        inventory.return_device(con, "LT-0001", "2025-06-12", status="retired")
        inventory.assign(con, "LT-0003", "kwatanabe", "2025-06-12")
    print(f"Database ready: {args.db}")
    return 0


def cmd_devices(con, args):
    show(inventory.search(con, args.search), ["asset_tag", "model", "status", "holder"])
    return 0


def cmd_assign(con, args):
    inventory.assign(con, args.asset_tag, args.username, args.today)
    print(f"{args.asset_tag} -> {args.username}")
    return 0


def cmd_return(con, args):
    inventory.return_device(con, args.asset_tag, args.today, status=args.status)
    print(f"{args.asset_tag} returned ({args.status})")
    return 0


def cmd_warranty(con, args):
    rows = inventory.warranty_report(con, args.today, days=args.days)
    show(rows, ["asset_tag", "model", "warranty_until", "state"])
    return 0


def cmd_spend(con, args):
    show(inventory.spend_by_type(con), ["type", "devices", "total_yen"])
    return 0


def cmd_history(con, args):
    show(inventory.history(con, args.asset_tag), ["username", "assigned", "returned"])
    return 0


def main(argv=None):
    parser = argparse.ArgumentParser(description="IT asset inventory")
    parser.add_argument("--db", default="inventory.db", help="database file (default: inventory.db)")
    parser.add_argument("--today", default=date.today().isoformat(), help="pretend today is YYYY-MM-DD")
    sub = parser.add_subparsers(required=True)

    p = sub.add_parser("init", help="create the database")
    p.add_argument("--seed", action="store_true", help="add sample data")
    p.set_defaults(func=cmd_init)
    p = sub.add_parser("devices", help="list or search devices")
    p.add_argument("--search", default="", help="text in the tag, model, serial or holder")
    p.set_defaults(func=cmd_devices)
    p = sub.add_parser("assign", help="give a device to someone")
    p.add_argument("asset_tag")
    p.add_argument("username")
    p.set_defaults(func=cmd_assign)
    p = sub.add_parser("return", help="take a device back")
    p.add_argument("asset_tag")
    p.add_argument("--status", default="in-stock", choices=["in-stock", "repair", "retired"])
    p.set_defaults(func=cmd_return)
    p = sub.add_parser("warranty", help="warranties that ended or end soon")
    p.add_argument("--days", type=int, default=90)
    p.set_defaults(func=cmd_warranty)
    p = sub.add_parser("spend", help="hardware spending per type")
    p.set_defaults(func=cmd_spend)
    p = sub.add_parser("history", help="who had a device, and when")
    p.add_argument("asset_tag")
    p.set_defaults(func=cmd_history)

    args = parser.parse_args(argv)
    con = inventory.connect(args.db)
    try:
        return args.func(con, args)
    except ValueError as e:
        print(f"error: {e}", file=sys.stderr)
        return 1
    finally:
        con.close()


if __name__ == "__main__":
    sys.exit(main())
@@text
> Tips: in `assign`, look up the device and the person *inside* `with transaction(con):`, and raise `ValueError` before writing anything. `cur.rowcount` tells you how many rows an UPDATE changed: 0 in `return_device` means nothing was assigned. For `search`, use **one** named parameter for every column: `WHERE d.asset_tag LIKE :pattern OR d.model LIKE :pattern ...` with `{"pattern": f"%{text}%"}` (SQLite's LIKE ignores case for English letters). `dict(row)` turns a `sqlite3.Row` into a dict.

@@project inventory_finished IT Asset Inventory: the finished version
The complete project. Ideas to take it further: a `licenses` table with seats (a junction table, like the lesson), CSV import of a supplier's delivery note, an HTML report, or the same schema on PostgreSQL (change `?` to `%s` and use `psycopg`).
@@file schema.sql
-- IT asset inventory. Run on every start, so everything is "IF NOT EXISTS".

CREATE TABLE IF NOT EXISTS people (
    id         INTEGER PRIMARY KEY,
    name       TEXT NOT NULL,
    username   TEXT NOT NULL UNIQUE,
    department TEXT
);

CREATE TABLE IF NOT EXISTS devices (
    id             INTEGER PRIMARY KEY,
    asset_tag      TEXT NOT NULL UNIQUE CHECK (asset_tag GLOB '[A-Z][A-Z]-[0-9][0-9][0-9][0-9]'),
    type           TEXT NOT NULL CHECK (type IN ('laptop', 'desktop', 'monitor', 'phone', 'tablet', 'printer', 'network', 'server')),
    model          TEXT NOT NULL,
    serial         TEXT UNIQUE,
    purchased      TEXT NOT NULL,
    price          INTEGER NOT NULL CHECK (price >= 0),
    warranty_until TEXT,
    status         TEXT NOT NULL DEFAULT 'in-stock' CHECK (status IN ('in-stock', 'assigned', 'repair', 'retired'))
);

CREATE TABLE IF NOT EXISTS assignments (
    id        INTEGER PRIMARY KEY,
    device_id INTEGER NOT NULL REFERENCES devices(id),
    person_id INTEGER NOT NULL REFERENCES people(id),
    assigned  TEXT NOT NULL,
    returned  TEXT
);

-- A device can have only one open assignment.
CREATE UNIQUE INDEX IF NOT EXISTS one_holder ON assignments(device_id) WHERE returned IS NULL;
CREATE INDEX IF NOT EXISTS assignments_person ON assignments(person_id);
@@file inventory.py
"""An IT asset inventory on SQLite. Every function takes an open connection."""

import sqlite3
from contextlib import contextmanager
from pathlib import Path

SCHEMA = Path(__file__).with_name("schema.sql")


def connect(path="inventory.db"):
    """Open (or create) the database, with foreign keys on and rows usable like dicts."""
    con = sqlite3.connect(path, isolation_level=None)  # autocommit; transaction() groups changes
    con.row_factory = sqlite3.Row
    con.execute("PRAGMA foreign_keys = ON")
    con.executescript(SCHEMA.read_text(encoding="utf-8"))
    return con


@contextmanager
def transaction(con):
    """with transaction(con): ... commits at the end, or rolls back if anything raises."""
    con.execute("BEGIN")
    try:
        yield
    except BaseException:
        con.execute("ROLLBACK")
        raise
    con.execute("COMMIT")


def add_person(con, name, username, department=None):
    """Insert a person; return their id."""
    cur = con.execute("INSERT INTO people (name, username, department) VALUES (?, ?, ?)",
                      (name, username, department))
    return cur.lastrowid


def add_device(con, asset_tag, type, model, serial, purchased, price, warranty_until=None):
    """Insert a device (status in-stock); return its id. Bad data raises sqlite3.IntegrityError."""
    cur = con.execute(
        "INSERT INTO devices (asset_tag, type, model, serial, purchased, price, warranty_until)"
        " VALUES (?, ?, ?, ?, ?, ?, ?)",
        (asset_tag, type, model, serial, purchased, price, warranty_until),
    )
    return cur.lastrowid


def _device(con, asset_tag):
    row = con.execute("SELECT id, status FROM devices WHERE asset_tag = ?", (asset_tag,)).fetchone()
    if row is None:
        raise ValueError(f"no device {asset_tag}")
    return row


def assign(con, asset_tag, username, day):
    """Give an in-stock device to a person, in one transaction: add an assignment
    and set the device's status to 'assigned'.
    Raise ValueError if the device or person doesn't exist, or the device isn't in-stock."""
    with transaction(con):
        device = _device(con, asset_tag)
        if device["status"] != "in-stock":
            raise ValueError(f"{asset_tag} is {device['status']}, not in-stock")
        person = con.execute("SELECT id FROM people WHERE username = ?", (username,)).fetchone()
        if person is None:
            raise ValueError(f"no person {username}")
        con.execute("INSERT INTO assignments (device_id, person_id, assigned) VALUES (?, ?, ?)",
                    (device["id"], person["id"], day))
        con.execute("UPDATE devices SET status = 'assigned' WHERE id = ?", (device["id"],))


def return_device(con, asset_tag, day, status="in-stock"):
    """Close the device's open assignment (returned = day) and set its status
    (in-stock, repair or retired), in one transaction. ValueError if it isn't assigned."""
    with transaction(con):
        device = _device(con, asset_tag)
        cur = con.execute("UPDATE assignments SET returned = ? WHERE device_id = ? AND returned IS NULL",
                          (day, device["id"]))
        if cur.rowcount == 0:
            raise ValueError(f"{asset_tag} isn't assigned to anyone")
        con.execute("UPDATE devices SET status = ? WHERE id = ?", (status, device["id"]))


def holder(con, asset_tag):
    """The username who has the device now, or None."""
    row = con.execute(
        """SELECT p.username
           FROM assignments AS a
           JOIN devices AS d ON d.id = a.device_id
           JOIN people AS p ON p.id = a.person_id
           WHERE d.asset_tag = ? AND a.returned IS NULL""",
        (asset_tag,),
    ).fetchone()
    return row["username"] if row else None


def history(con, asset_tag):
    """[(username, assigned, returned), ...] for the device, oldest first."""
    rows = con.execute(
        """SELECT p.username, a.assigned, a.returned
           FROM assignments AS a
           JOIN devices AS d ON d.id = a.device_id
           JOIN people AS p ON p.id = a.person_id
           WHERE d.asset_tag = ?
           ORDER BY a.assigned, a.id""",
        (asset_tag,),
    )
    return [tuple(r) for r in rows]


def warranty_report(con, today, days=90):
    """Devices that aren't retired whose warranty has ended or ends within `days` days of `today`:
    [{"asset_tag", "model", "warranty_until", "state"}, ...], state "expired" or "expiring",
    ordered by warranty_until, then asset_tag."""
    rows = con.execute(
        """SELECT asset_tag, model, warranty_until,
                  CASE WHEN warranty_until < :today THEN 'expired' ELSE 'expiring' END AS state
           FROM devices
           WHERE status <> 'retired'
             AND warranty_until IS NOT NULL
             AND warranty_until <= date(:today, '+' || :days || ' days')
           ORDER BY warranty_until, asset_tag""",
        {"today": today, "days": days},
    )
    return [dict(r) for r in rows]


def spend_by_type(con):
    """[(type, devices, total_price), ...] for devices that aren't retired, biggest total first."""
    rows = con.execute(
        """SELECT type, COUNT(*), SUM(price) AS total
           FROM devices
           WHERE status <> 'retired'
           GROUP BY type
           ORDER BY total DESC, type"""
    )
    return [tuple(r) for r in rows]


def search(con, text):
    """Devices whose asset tag, model, serial or current holder's username contains `text`
    (any case): [{"asset_tag", "model", "status", "holder"}, ...] ordered by asset tag.
    holder is None for devices nobody has."""
    rows = con.execute(
        """SELECT d.asset_tag, d.model, d.status, p.username AS holder
           FROM devices AS d
           LEFT JOIN assignments AS a ON a.device_id = d.id AND a.returned IS NULL
           LEFT JOIN people AS p ON p.id = a.person_id
           WHERE d.asset_tag LIKE :pattern OR d.model LIKE :pattern
              OR d.serial LIKE :pattern OR p.username LIKE :pattern
           ORDER BY d.asset_tag""",
        {"pattern": f"%{text}%"},
    )
    return [dict(r) for r in rows]
@@file test_inventory.py
import sqlite3
import unittest

from inventory import (add_device, add_person, assign, connect, history, holder, return_device,
                       search, spend_by_type, transaction, warranty_report)


class InventoryTests(unittest.TestCase):
    def setUp(self):
        self.con = connect(":memory:")
        add_person(self.con, "Yuki Tanaka", "ytanaka", "IT")
        add_person(self.con, "Mei Sato", "msato", "Sales")
        add_device(self.con, "LT-0001", "laptop", "ThinkPad T14", "SN1", "2023-04-01", 189000, "2026-03-31")
        add_device(self.con, "LT-0002", "laptop", "ThinkPad T14", "SN2", "2024-04-01", 195000, "2026-12-15")
        add_device(self.con, "MN-0001", "monitor", "Dell P2723", "SN3", "2024-05-10", 42000, "2029-05-10")
        add_device(self.con, "PH-0001", "phone", "iPhone 15", "SN4", "2024-09-01", 124800)

    def tearDown(self):
        self.con.close()

    def status(self, tag):
        return self.con.execute("SELECT status FROM devices WHERE asset_tag = ?", (tag,)).fetchone()[0]

    def test_constraints(self):
        bad = [
            ("LT-0001", "laptop", "dup tag", "SN9", "2024-01-01", 1000),
            ("laptop1", "laptop", "bad tag", "SN9", "2024-01-01", 1000),
            ("TB-0001", "toaster", "bad type", "SN9", "2024-01-01", 1000),
            ("TB-0001", "tablet", "negative", "SN9", "2024-01-01", -5),
        ]
        for row in bad:
            with self.assertRaises(sqlite3.IntegrityError, msg=row[2]):
                add_device(self.con, *row)

    def test_assign_and_return(self):
        assign(self.con, "LT-0002", "ytanaka", "2026-10-01")
        self.assertEqual(holder(self.con, "LT-0002"), "ytanaka")
        self.assertEqual(self.status("LT-0002"), "assigned")
        with self.assertRaises(ValueError):
            assign(self.con, "LT-0002", "msato", "2026-10-02")
        return_device(self.con, "LT-0002", "2026-10-20", status="repair")
        self.assertIsNone(holder(self.con, "LT-0002"))
        self.assertEqual(self.status("LT-0002"), "repair")
        with self.assertRaises(ValueError):
            return_device(self.con, "LT-0002", "2026-10-21")

    def test_history(self):
        assign(self.con, "LT-0001", "ytanaka", "2026-01-10")
        return_device(self.con, "LT-0001", "2026-05-01")
        assign(self.con, "LT-0001", "msato", "2026-05-02")
        self.assertEqual(history(self.con, "LT-0001"),
                         [("ytanaka", "2026-01-10", "2026-05-01"), ("msato", "2026-05-02", None)])

    def test_unknown_device_or_person(self):
        with self.assertRaises(ValueError):
            assign(self.con, "LT-9999", "ytanaka", "2026-10-01")
        with self.assertRaises(ValueError):
            assign(self.con, "LT-0001", "nobody", "2026-10-01")
        self.assertEqual(self.status("LT-0001"), "in-stock")

    def test_schema_allows_one_holder_only(self):
        self.con.execute("INSERT INTO assignments (device_id, person_id, assigned) VALUES (1, 1, '2026-10-01')")
        with self.assertRaises(sqlite3.IntegrityError):
            self.con.execute("INSERT INTO assignments (device_id, person_id, assigned) VALUES (1, 2, '2026-10-02')")

    def test_schema_foreign_keys(self):
        with self.assertRaises(sqlite3.IntegrityError):
            self.con.execute("INSERT INTO assignments (device_id, person_id, assigned) VALUES (99, 1, '2026-10-01')")

    def test_transaction_rolls_back(self):
        with self.assertRaises(RuntimeError):
            with transaction(self.con):
                add_person(self.con, "Ghost", "ghost")
                raise RuntimeError("boom")
        self.assertIsNone(self.con.execute("SELECT id FROM people WHERE username = 'ghost'").fetchone())

    def test_warranty_report(self):
        self.assertEqual(warranty_report(self.con, "2026-10-31", days=90), [
            {"asset_tag": "LT-0001", "model": "ThinkPad T14", "warranty_until": "2026-03-31", "state": "expired"},
            {"asset_tag": "LT-0002", "model": "ThinkPad T14", "warranty_until": "2026-12-15", "state": "expiring"},
        ])
        self.assertEqual(len(warranty_report(self.con, "2026-10-31", days=30)), 1)

    def test_retired_devices_are_left_out(self):
        assign(self.con, "LT-0001", "ytanaka", "2026-01-10")
        return_device(self.con, "LT-0001", "2026-10-30", status="retired")
        self.assertEqual([r["asset_tag"] for r in warranty_report(self.con, "2026-10-31")], ["LT-0002"])
        self.assertEqual(spend_by_type(self.con),
                         [("laptop", 1, 195000), ("phone", 1, 124800), ("monitor", 1, 42000)])

    def test_spend_by_type(self):
        self.assertEqual(spend_by_type(self.con),
                         [("laptop", 2, 384000), ("phone", 1, 124800), ("monitor", 1, 42000)])

    def test_search(self):
        assign(self.con, "LT-0002", "msato", "2026-10-01")
        self.assertEqual([r["asset_tag"] for r in search(self.con, "thinkpad")], ["LT-0001", "LT-0002"])
        self.assertEqual(search(self.con, "msato"),
                         [{"asset_tag": "LT-0002", "model": "ThinkPad T14", "status": "assigned", "holder": "msato"}])
        self.assertEqual(search(self.con, "SN3")[0]["holder"], None)

    def test_search_is_not_injectable(self):
        self.assertEqual(search(self.con, "' OR '1'='1"), [])
        self.assertEqual(search(self.con, "x'; DROP TABLE devices; --"), [])
        self.assertEqual(len(search(self.con, "")), 4)
@@file seed.sql
-- Sample data for "python main.py init --seed".
INSERT INTO people (name, username, department) VALUES
    ('Yuki Tanaka', 'ytanaka', 'IT'),
    ('Mei Sato', 'msato', 'Sales'),
    ('Kenji Watanabe', 'kwatanabe', 'Finance'),
    ('Ana Silva', 'asilva', 'Development');

INSERT INTO devices (asset_tag, type, model, serial, purchased, price, warranty_until) VALUES
    ('LT-0001', 'laptop', 'ThinkPad T14 Gen 3', 'PF3AB001', '2022-04-01', 189000, '2025-03-31'),
    ('LT-0002', 'laptop', 'ThinkPad T14 Gen 4', 'PF4CD002', '2024-04-01', 198000, '2027-03-31'),
    ('LT-0003', 'laptop', 'ThinkPad X1 Carbon', 'PF5EF003', '2025-06-10', 265000, '2028-06-10'),
    ('LT-0004', 'laptop', 'MacBook Pro 14', 'C02GH004', '2024-09-20', 298800, '2027-09-20'),
    ('MN-0001', 'monitor', 'Dell P2723QE', 'CN0J1005', '2023-05-10', 52000, '2026-11-30'),
    ('MN-0002', 'monitor', 'Dell P2723QE', 'CN0J1006', '2023-05-10', 52000, '2026-11-30'),
    ('PH-0001', 'phone', 'iPhone 15', 'F2LK0007', '2024-10-01', 124800, NULL),
    ('PR-0001', 'printer', 'Brother MFC-L8900CDW', 'E7896008', '2021-02-15', 89000, '2024-02-15'),
    ('NW-0001', 'network', 'Yamaha RTX1300', 'S4X00009', '2024-01-20', 156000, '2029-01-20');
@@file main.py
"""Command line for the inventory. python main.py --help"""

import argparse
import sys
from datetime import date
from pathlib import Path

import inventory


def show(rows, columns):
    """Print dicts or tuples as an aligned table."""
    rows = [dict(zip(columns, r)) if isinstance(r, tuple) else r for r in rows]
    if not rows:
        print("(nothing)")
        return
    text = [[("" if r[c] is None else str(r[c])) for c in columns] for r in rows]
    widths = [max(len(c), *(len(t[i]) for t in text)) for i, c in enumerate(columns)]
    print("  ".join(c.ljust(w) for c, w in zip(columns, widths)))
    print("  ".join("-" * w for w in widths))
    for t in text:
        print("  ".join(v.ljust(w) for v, w in zip(t, widths)))


def cmd_init(con, args):
    if args.seed:
        if con.execute("SELECT COUNT(*) FROM devices").fetchone()[0]:
            print("The database already has devices; not seeding.")
            return 1
        con.executescript(Path(__file__).with_name("seed.sql").read_text(encoding="utf-8"))
        inventory.assign(con, "LT-0002", "msato", "2024-04-05")
        inventory.assign(con, "LT-0004", "asilva", "2024-09-25")
        inventory.assign(con, "MN-0001", "msato", "2024-04-05")
        inventory.assign(con, "LT-0001", "kwatanabe", "2022-04-03")
        inventory.return_device(con, "LT-0001", "2025-06-12", status="retired")
        inventory.assign(con, "LT-0003", "kwatanabe", "2025-06-12")
    print(f"Database ready: {args.db}")
    return 0


def cmd_devices(con, args):
    show(inventory.search(con, args.search), ["asset_tag", "model", "status", "holder"])
    return 0


def cmd_assign(con, args):
    inventory.assign(con, args.asset_tag, args.username, args.today)
    print(f"{args.asset_tag} -> {args.username}")
    return 0


def cmd_return(con, args):
    inventory.return_device(con, args.asset_tag, args.today, status=args.status)
    print(f"{args.asset_tag} returned ({args.status})")
    return 0


def cmd_warranty(con, args):
    rows = inventory.warranty_report(con, args.today, days=args.days)
    show(rows, ["asset_tag", "model", "warranty_until", "state"])
    return 0


def cmd_spend(con, args):
    show(inventory.spend_by_type(con), ["type", "devices", "total_yen"])
    return 0


def cmd_history(con, args):
    show(inventory.history(con, args.asset_tag), ["username", "assigned", "returned"])
    return 0


def main(argv=None):
    parser = argparse.ArgumentParser(description="IT asset inventory")
    parser.add_argument("--db", default="inventory.db", help="database file (default: inventory.db)")
    parser.add_argument("--today", default=date.today().isoformat(), help="pretend today is YYYY-MM-DD")
    sub = parser.add_subparsers(required=True)

    p = sub.add_parser("init", help="create the database")
    p.add_argument("--seed", action="store_true", help="add sample data")
    p.set_defaults(func=cmd_init)
    p = sub.add_parser("devices", help="list or search devices")
    p.add_argument("--search", default="", help="text in the tag, model, serial or holder")
    p.set_defaults(func=cmd_devices)
    p = sub.add_parser("assign", help="give a device to someone")
    p.add_argument("asset_tag")
    p.add_argument("username")
    p.set_defaults(func=cmd_assign)
    p = sub.add_parser("return", help="take a device back")
    p.add_argument("asset_tag")
    p.add_argument("--status", default="in-stock", choices=["in-stock", "repair", "retired"])
    p.set_defaults(func=cmd_return)
    p = sub.add_parser("warranty", help="warranties that ended or end soon")
    p.add_argument("--days", type=int, default=90)
    p.set_defaults(func=cmd_warranty)
    p = sub.add_parser("spend", help="hardware spending per type")
    p.set_defaults(func=cmd_spend)
    p = sub.add_parser("history", help="who had a device, and when")
    p.add_argument("asset_tag")
    p.set_defaults(func=cmd_history)

    args = parser.parse_args(argv)
    con = inventory.connect(args.db)
    try:
        return args.func(con, args)
    except ValueError as e:
        print(f"error: {e}", file=sys.stderr)
        return 1
    finally:
        con.close()


if __name__ == "__main__":
    sys.exit(main())
@@text
🎉 You've designed a schema that defends itself, written injection-proof queries, and used transactions for changes that must not half-happen. That's the daily work of a junior DBA or a backend developer, and a good story for an interview: *"I built an asset inventory where the database itself guarantees a device has only one holder."*

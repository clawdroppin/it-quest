@@lesson t2-shop Shop Order Database: SQL Behind a Real Application
@@text
In most applications the database does the hardest jobs: never selling the same last item twice, keeping an order's price when the catalogue changes, refusing impossible data, recording every status change, and answering the boss's "what sold best?" in milliseconds. In this project the database **is** the application: you write the SQL, and a thin Python layer only runs it.

```text
$ python main.py
Docks in stock:
  DCK-USBC-PRO   USB-C dock Pro         $  189.00  stock 4
  DCK-TB4        Thunderbolt 4 dock     $  329.00  stock 2

Order 1 placed: {'id': 1, 'customer_id': 1, 'status': 'new', 'total_cents': 21480, 'items': 3, ...}

Conflict: order 1 was changed by someone else; reload it

Sales by category:
  1. docks       1 units  $    189.00
  2. cables      2 units  $     25.80
```

## How it's built

| File | Job |
|---|---|
| `schema.sql` | tables with constraints, indexes, two triggers (status history, restock on cancel) and a view (`order_totals`) |
| `queries.sql` | every query the app runs, each under a `-- name: ...` line, with named parameters like `:product_id` |
| `seed.sql` | sample customers and products |
| `shop.py` | loads the queries, runs them in transactions, turns rows into Python values |
| `test_shop.py` | 18 tests (`python -m unittest -v`) |
| `main.py` | a short demo |

Keeping the SQL in `.sql` files, named and parameterized, is a real-world pattern (libraries like aiosql, yesql and sqlc work this way): the SQL can be read, reviewed and tested as SQL, and SQL injection is impossible because values are never pasted into the text.

Read `schema.sql` first: the triggers and the view do a lot of the work for you.

## Your tasks

Four queries in `queries.sql` say `-- TODO: write this query`. The comments above each one are the specification:

1. `reserve_stock`: a **conditional UPDATE**. The `CHECK (stock >= 0)` would also stop overselling, but with an error; your `WHERE` makes it a clean "0 rows changed" that `place_order` turns into `OutOfStock`, rolling back the whole order.
2. `search_products`: text search, an optional filter (`:category IS NULL OR ...`), and **keyset pagination**.
3. `set_status`: **optimistic locking** with the `version` column.
4. `sales_by_category`: joins, `GROUP BY`, and the **window function** `RANK() OVER (ORDER BY ...)`.

```text
python -m unittest -v
```

Then run `python main.py` (delete `shop.db` to start again). You can also open `shop.db` in **DB Browser for SQLite** or the `sqlite3` command-line shell and try the queries by hand.
@@project shop_db Shop Order Database
Write the four queries marked `TODO` in `queries.sql` until `python -m unittest` passes, then run `python main.py`.
@@file schema.sql
-- schema.sql: the database of a small online shop for IT accessories.
-- Run once on an empty database (shop.py does it for you).

PRAGMA foreign_keys = ON;

CREATE TABLE customers (
    id         INTEGER PRIMARY KEY,
    email      TEXT    NOT NULL UNIQUE COLLATE NOCASE,
    name       TEXT    NOT NULL CHECK (length(trim(name)) > 0)
);

CREATE TABLE products (
    id          INTEGER PRIMARY KEY,
    sku         TEXT    NOT NULL UNIQUE,
    name        TEXT    NOT NULL,
    category    TEXT    NOT NULL CHECK (category IN ('cables', 'docks', 'input', 'audio', 'displays')),
    price_cents INTEGER NOT NULL CHECK (price_cents > 0),
    stock       INTEGER NOT NULL DEFAULT 0 CHECK (stock >= 0)      -- can never go negative
);

CREATE TABLE orders (
    id          INTEGER PRIMARY KEY,
    customer_id INTEGER NOT NULL REFERENCES customers(id) ON DELETE RESTRICT,
    status      TEXT    NOT NULL DEFAULT 'new'
                CHECK (status IN ('new', 'paid', 'shipped', 'cancelled')),
    version     INTEGER NOT NULL DEFAULT 1,                         -- for optimistic locking
    created_at  TEXT    NOT NULL DEFAULT (datetime('now'))
);

-- The price is copied into the line: the order keeps the price it was sold at,
-- even if the product's price changes later.
CREATE TABLE order_lines (
    order_id    INTEGER NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    product_id  INTEGER NOT NULL REFERENCES products(id) ON DELETE RESTRICT,
    qty         INTEGER NOT NULL CHECK (qty > 0),
    price_cents INTEGER NOT NULL CHECK (price_cents > 0),
    PRIMARY KEY (order_id, product_id)
);

CREATE TABLE order_events (
    id         INTEGER PRIMARY KEY,
    order_id   INTEGER NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    old_status TEXT,
    new_status TEXT    NOT NULL,
    at         TEXT    NOT NULL DEFAULT (datetime('now'))
);

-- Indexes for the queries the application runs.
CREATE INDEX idx_products_category ON products (category, id);
CREATE INDEX idx_orders_customer ON orders (customer_id, created_at);
CREATE INDEX idx_order_lines_product ON order_lines (product_id);

-- Every status change is recorded, whichever code path made it.
CREATE TRIGGER trg_order_status AFTER UPDATE OF status ON orders
WHEN OLD.status IS NOT NEW.status
BEGIN
    INSERT INTO order_events (order_id, old_status, new_status) VALUES (NEW.id, OLD.status, NEW.status);
END;

-- A cancelled order puts its items back in stock.
CREATE TRIGGER trg_order_cancelled AFTER UPDATE OF status ON orders
WHEN NEW.status = 'cancelled' AND OLD.status <> 'cancelled'
BEGIN
    UPDATE products
    SET stock = stock + (SELECT qty FROM order_lines WHERE order_id = NEW.id AND product_id = products.id)
    WHERE id IN (SELECT product_id FROM order_lines WHERE order_id = NEW.id);
END;

-- One row per order with its total: reports and the app read this instead of repeating the SUM.
CREATE VIEW order_totals AS
SELECT o.id, o.customer_id, o.status, o.created_at,
       COALESCE(SUM(l.qty * l.price_cents), 0) AS total_cents,
       COALESCE(SUM(l.qty), 0) AS items
FROM orders o
LEFT JOIN order_lines l ON l.order_id = o.id
GROUP BY o.id;
@@file queries.sql
-- queries.sql: every query the application runs, by name. shop.py loads this file and runs
-- them with named parameters (:name), so values are never pasted into the SQL text.

-- name: create_order
-- A new order for :customer_id; returns its id.
INSERT INTO orders (customer_id) VALUES (:customer_id) RETURNING id;

-- name: reserve_stock
-- Takes :qty of :product_id out of stock, but only if there's enough: the check and the
-- change are one statement, so two customers can never both buy the last item.
-- shop.py looks at how many rows changed: 0 means "not enough stock".
-- TODO: write this query

-- name: add_line
-- Adds :qty of :product_id to order :order_id at the product's current price.
-- The same product twice in one order adds to the existing line instead of failing.
INSERT INTO order_lines (order_id, product_id, qty, price_cents)
SELECT :order_id, id, :qty, price_cents FROM products WHERE id = :product_id
ON CONFLICT (order_id, product_id) DO UPDATE SET qty = qty + excluded.qty;

-- name: search_products
-- One page of products (id, sku, name, category, price_cents, stock):
--   name contains :q (any case; an empty :q matches everything),
--   category is :category, or any category when :category is NULL,
--   only products with an id greater than :after (keyset pagination: the last id of the previous page),
--   ordered by id, at most :limit rows.
-- TODO: write this query

-- name: set_status
-- Changes order :id to :status, with optimistic locking: only if its version is still :version,
-- and the version goes up by one. 0 rows changed means someone else changed the order first.
-- TODO: write this query

-- name: get_order
SELECT id, customer_id, status, total_cents, items, created_at,
       (SELECT version FROM orders WHERE orders.id = order_totals.id) AS version
FROM order_totals WHERE id = :id;

-- name: order_history
SELECT old_status, new_status FROM order_events WHERE order_id = :id ORDER BY id;

-- name: sales_by_category
-- Revenue per category from orders that are paid or shipped (not new, not cancelled):
-- category, units (total qty), revenue_cents (qty * price of the lines), and rank
-- (1 = the highest revenue, with RANK() so equal revenues share a rank).
-- Highest revenue first, then by category name.
-- TODO: write this query

-- name: top_customers
-- Customers by money spent on paid or shipped orders, with their number of such orders.
SELECT c.name, COUNT(t.id) AS orders, SUM(t.total_cents) AS spent_cents
FROM customers c
JOIN order_totals t ON t.customer_id = c.id AND t.status IN ('paid', 'shipped')
GROUP BY c.id
ORDER BY spent_cents DESC, c.name
LIMIT :limit;
@@file seed.sql
-- seed.sql: sample data for trying the shop and for the tests.
INSERT INTO customers (id, email, name) VALUES
 (1, 'aiko@example.jp', 'Aiko Mori'),
 (2, 'ken@example.jp', 'Ken Sato'),
 (3, 'yuki@example.jp', 'Yuki Tanaka');

INSERT INTO products (id, sku, name, category, price_cents, stock) VALUES
 (1, 'CBL-HDMI-2M', 'HDMI cable 2 m', 'cables', 1290, 120),
 (2, 'CBL-USBC-1M', 'USB-C cable 1 m', 'cables', 990, 200),
 (3, 'DCK-USBC-PRO', 'USB-C dock Pro', 'docks', 18900, 4),
 (4, 'DCK-TB4', 'Thunderbolt 4 dock', 'docks', 32900, 2),
 (5, 'INP-KB-JIS', 'Keyboard (JIS layout)', 'input', 6490, 35),
 (6, 'INP-MOUSE-W', 'Wireless mouse', 'input', 2990, 50),
 (7, 'AUD-HEADSET', 'USB headset with mic', 'audio', 7990, 18),
 (8, 'DSP-27-4K', '27" 4K monitor', 'displays', 54800, 6),
 (9, 'CBL-DP-2M', 'DisplayPort cable 2 m', 'cables', 1490, 0),
 (10, 'INP-KB-US', 'Keyboard (US layout)', 'input', 6490, 12);
@@file shop.py
"""shop.py: the application's data layer. All SQL lives in schema.sql and queries.sql;
this file only runs it, inside transactions, and turns results into Python values."""
import re
import sqlite3
from pathlib import Path

HERE = Path(__file__).parent


class OutOfStock(Exception):
    pass


class Conflict(Exception):
    """The order was changed by someone else since it was loaded."""


def load_queries(path=HERE / "queries.sql"):
    """Splits queries.sql at its '-- name: x' lines into {name: sql}."""
    queries = {}
    for block in re.split(r"(?m)^-- name: ", path.read_text(encoding="utf-8"))[1:]:
        name, _, sql = block.partition("\n")
        queries[name.strip()] = sql.strip()
    return queries


class Shop:
    def __init__(self, path=":memory:"):
        self.db = sqlite3.connect(path, isolation_level=None)   # we BEGIN and COMMIT ourselves
        self.db.row_factory = sqlite3.Row
        self.db.execute("PRAGMA foreign_keys = ON")
        self.q = load_queries()
        if not self.db.execute("SELECT 1 FROM sqlite_master WHERE name = 'orders'").fetchone():
            self.db.executescript((HERE / "schema.sql").read_text(encoding="utf-8"))

    def run(self, name, **params):
        sql = self.q[name]
        if "TODO" in sql:
            raise NotImplementedError(f"TODO: write the {name} query in queries.sql")
        return self.db.execute(sql, params)

    def seed(self, path=HERE / "seed.sql"):
        self.db.executescript(path.read_text(encoding="utf-8"))

    def place_order(self, customer_id, items):
        """items: [(product_id, qty), ...]. Everything happens, or nothing does."""
        self.db.execute("BEGIN IMMEDIATE")      # take the write lock now, not halfway through
        try:
            order_id = self.run("create_order", customer_id=customer_id).fetchone()["id"]
            for product_id, qty in items:
                if self.run("reserve_stock", product_id=product_id, qty=qty).rowcount != 1:
                    raise OutOfStock(f"not enough stock for product {product_id}")
                self.run("add_line", order_id=order_id, product_id=product_id, qty=qty)
            self.db.execute("COMMIT")
            return order_id
        except BaseException:
            self.db.execute("ROLLBACK")
            raise

    def search(self, q="", category=None, after=0, limit=20):
        rows = self.run("search_products", q=q, category=category, after=after, limit=limit)
        return [dict(r) for r in rows]

    def get_order(self, order_id):
        row = self.run("get_order", id=order_id).fetchone()
        return dict(row) if row else None

    def set_status(self, order_id, status, version):
        if self.run("set_status", id=order_id, status=status, version=version).rowcount != 1:
            raise Conflict(f"order {order_id} was changed by someone else; reload it")

    def history(self, order_id):
        return [(r["old_status"], r["new_status"]) for r in self.run("order_history", id=order_id)]

    def sales_by_category(self):
        return [dict(r) for r in self.run("sales_by_category")]

    def top_customers(self, limit=3):
        return [dict(r) for r in self.run("top_customers", limit=limit)]
@@file test_shop.py
# Run: python -m unittest -v
import unittest

from shop import Conflict, OutOfStock, Shop


def stock(shop, product_id):
    return shop.db.execute("SELECT stock FROM products WHERE id = ?", (product_id,)).fetchone()[0]


class ShopTest(unittest.TestCase):
    def setUp(self):
        self.shop = Shop()
        self.shop.seed()


class SearchTest(ShopTest):
    def test_text_search_ignores_case(self):
        self.assertEqual([p["sku"] for p in self.shop.search("KEYBOARD")], ["INP-KB-JIS", "INP-KB-US"])

    def test_category_filter(self):
        self.assertEqual([p["id"] for p in self.shop.search(category="docks")], [3, 4])

    def test_no_filters_returns_everything_in_id_order(self):
        self.assertEqual([p["id"] for p in self.shop.search()], list(range(1, 11)))

    def test_keyset_pages(self):
        page1 = self.shop.search(category="cables", limit=2)
        page2 = self.shop.search(category="cables", after=page1[-1]["id"], limit=2)
        self.assertEqual([p["id"] for p in page1], [1, 2])
        self.assertEqual([p["id"] for p in page2], [9])

    def test_columns(self):
        self.assertEqual(self.shop.search("dock Pro")[0],
                         {"id": 3, "sku": "DCK-USBC-PRO", "name": "USB-C dock Pro", "category": "docks",
                          "price_cents": 18900, "stock": 4})


class OrderTest(ShopTest):
    def test_place_order_takes_stock_and_copies_prices(self):
        order_id = self.shop.place_order(1, [(3, 2), (1, 3)])
        self.assertEqual((stock(self.shop, 3), stock(self.shop, 1)), (2, 117))
        order = self.shop.get_order(order_id)
        self.assertEqual((order["status"], order["items"], order["total_cents"]), ("new", 5, 2 * 18900 + 3 * 1290))

    def test_the_order_keeps_its_price(self):
        order_id = self.shop.place_order(1, [(5, 1)])
        self.shop.db.execute("UPDATE products SET price_cents = 9999 WHERE id = 5")
        self.assertEqual(self.shop.get_order(order_id)["total_cents"], 6490)

    def test_same_product_twice_adds_up(self):
        order_id = self.shop.place_order(2, [(6, 1), (6, 2)])
        self.assertEqual(self.shop.get_order(order_id)["items"], 3)
        self.assertEqual(stock(self.shop, 6), 47)

    def test_out_of_stock_changes_nothing(self):
        with self.assertRaises(OutOfStock):
            self.shop.place_order(1, [(1, 1), (4, 3)])          # only 2 Thunderbolt docks
        self.assertEqual((stock(self.shop, 1), stock(self.shop, 4)), (120, 2))
        self.assertEqual(self.shop.db.execute("SELECT COUNT(*) FROM orders").fetchone()[0], 0)

    def test_exactly_the_last_items(self):
        self.shop.place_order(1, [(4, 2)])
        self.assertEqual(stock(self.shop, 4), 0)
        with self.assertRaises(OutOfStock):
            self.shop.place_order(2, [(4, 1)])

    def test_unknown_customer_is_refused(self):
        with self.assertRaises(Exception):
            self.shop.place_order(99, [(1, 1)])
        self.assertEqual(stock(self.shop, 1), 120)


class StatusTest(ShopTest):
    def test_status_with_version(self):
        order_id = self.shop.place_order(1, [(1, 1)])
        self.shop.set_status(order_id, "paid", version=1)
        order = self.shop.get_order(order_id)
        self.assertEqual((order["status"], order["version"]), ("paid", 2))

    def test_stale_version_is_a_conflict(self):
        order_id = self.shop.place_order(1, [(1, 1)])
        self.shop.set_status(order_id, "paid", version=1)       # Ken
        with self.assertRaises(Conflict):
            self.shop.set_status(order_id, "cancelled", version=1)   # Aiko, with the old version
        self.assertEqual(self.shop.get_order(order_id)["status"], "paid")

    def test_history_and_cancel_restock(self):
        order_id = self.shop.place_order(1, [(3, 2), (7, 1)])
        self.shop.set_status(order_id, "paid", version=1)
        self.shop.set_status(order_id, "cancelled", version=2)
        self.assertEqual(self.shop.history(order_id), [("new", "paid"), ("paid", "cancelled")])
        self.assertEqual((stock(self.shop, 3), stock(self.shop, 7)), (4, 18))

    def test_invalid_status_is_refused_by_the_database(self):
        order_id = self.shop.place_order(1, [(1, 1)])
        with self.assertRaises(Exception):
            self.shop.set_status(order_id, "lost", version=1)


class ReportTest(ShopTest):
    def setUp(self):
        super().setUp()
        s = self.shop
        a = s.place_order(1, [(8, 1), (1, 2)])       # displays 54800, cables 2580
        b = s.place_order(2, [(5, 2), (6, 1)])       # input 12980 + 2990 = 15970
        c = s.place_order(3, [(3, 1)])               # docks 18900, stays new: not counted
        d = s.place_order(3, [(7, 2)])               # audio 15980, cancelled: not counted
        e = s.place_order(2, [(10, 1), (2, 3)])      # input 6490, cables 2970
        for order_id in (a, b, e):
            s.set_status(order_id, "paid", version=1)
        s.set_status(e, "shipped", version=2)
        s.set_status(d, "cancelled", version=1)
        self.unused = c

    def test_sales_by_category(self):
        self.assertEqual(self.shop.sales_by_category(), [
            {"category": "displays", "units": 1, "revenue_cents": 54800, "rank": 1},
            {"category": "input", "units": 4, "revenue_cents": 22460, "rank": 2},
            {"category": "cables", "units": 5, "revenue_cents": 5550, "rank": 3},
        ])

    def test_equal_revenue_shares_a_rank(self):
        order_id = self.shop.place_order(1, [(4, 1), (2, 1)])   # docks 32900, cables 990
        self.shop.set_status(order_id, "paid", version=1)
        order_id = self.shop.place_order(1, [(2, 5)])           # cables 4950: cables 11490
        self.shop.set_status(order_id, "paid", version=1)
        ranks = {r["category"]: r["rank"] for r in self.shop.sales_by_category()}
        self.assertEqual(ranks, {"displays": 1, "docks": 2, "input": 3, "cables": 4})
        self.shop.db.execute("UPDATE order_lines SET price_cents = 22460 WHERE product_id = 4")
        ranks = [r["rank"] for r in self.shop.sales_by_category()]
        self.assertEqual(ranks, [1, 2, 2, 4])

    def test_top_customers(self):
        self.assertEqual(self.shop.top_customers(), [
            {"name": "Aiko Mori", "orders": 1, "spent_cents": 57380},
            {"name": "Ken Sato", "orders": 2, "spent_cents": 25430},
        ])


if __name__ == "__main__":
    unittest.main()
@@file main.py
"""A short demo: python main.py   (creates shop.db next to this file the first time)."""
from pathlib import Path

from shop import Conflict, OutOfStock, Shop

db_file = Path(__file__).parent / "shop.db"
first_time = not db_file.exists()
shop = Shop(str(db_file))
if first_time:
    shop.seed()

print("Docks in stock:")
for p in shop.search(category="docks"):
    print(f"  {p['sku']:<14} {p['name']:<22} ${p['price_cents'] / 100:>8,.2f}  stock {p['stock']}")

try:
    order_id = shop.place_order(1, [(3, 1), (1, 2)])
    print(f"\nOrder {order_id} placed: {shop.get_order(order_id)}")
    shop.set_status(order_id, "paid", version=1)
    shop.set_status(order_id, "paid", version=1)     # a second, stale save
except OutOfStock as e:
    print("\nCan't order:", e)
except Conflict as e:
    print("\nConflict:", e)

print("\nSales by category:")
for r in shop.sales_by_category():
    print(f"  {r['rank']}. {r['category']:<9} {r['units']:>3} units  ${r['revenue_cents'] / 100:>10,.2f}")
@@text
## Stretch goals

- **Migrations**: turn `schema.sql` into `migrations/001_initial.sql`, track the version with `PRAGMA user_version`, and write `002_add_discounts.sql` (a `discount_pct` column on orders, used by the `order_totals` view).
- **Full-text search** with SQLite's FTS5: a `products_fts` virtual table kept in sync by triggers, and `MATCH` instead of `LIKE`.
- **Monthly report**: revenue per month with a running total (`SUM(...) OVER (ORDER BY month)`) and the change from the previous month (`LAG`).
- **PostgreSQL**: run the same schema on PostgreSQL (`SERIAL` or `GENERATED ALWAYS AS IDENTITY`, `NUMERIC` money, `now()`), using psycopg. What has to change? What doesn't?
- **Concurrency test**: two threads, each with its own connection to a file database, buying the last dock at once. Exactly one must succeed.
@@project shop_db_finished Shop Order Database: the finished version
A complete solution to compare with yours.
@@file schema.sql
-- schema.sql: the database of a small online shop for IT accessories.
-- Run once on an empty database (shop.py does it for you).

PRAGMA foreign_keys = ON;

CREATE TABLE customers (
    id         INTEGER PRIMARY KEY,
    email      TEXT    NOT NULL UNIQUE COLLATE NOCASE,
    name       TEXT    NOT NULL CHECK (length(trim(name)) > 0)
);

CREATE TABLE products (
    id          INTEGER PRIMARY KEY,
    sku         TEXT    NOT NULL UNIQUE,
    name        TEXT    NOT NULL,
    category    TEXT    NOT NULL CHECK (category IN ('cables', 'docks', 'input', 'audio', 'displays')),
    price_cents INTEGER NOT NULL CHECK (price_cents > 0),
    stock       INTEGER NOT NULL DEFAULT 0 CHECK (stock >= 0)      -- can never go negative
);

CREATE TABLE orders (
    id          INTEGER PRIMARY KEY,
    customer_id INTEGER NOT NULL REFERENCES customers(id) ON DELETE RESTRICT,
    status      TEXT    NOT NULL DEFAULT 'new'
                CHECK (status IN ('new', 'paid', 'shipped', 'cancelled')),
    version     INTEGER NOT NULL DEFAULT 1,                         -- for optimistic locking
    created_at  TEXT    NOT NULL DEFAULT (datetime('now'))
);

-- The price is copied into the line: the order keeps the price it was sold at,
-- even if the product's price changes later.
CREATE TABLE order_lines (
    order_id    INTEGER NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    product_id  INTEGER NOT NULL REFERENCES products(id) ON DELETE RESTRICT,
    qty         INTEGER NOT NULL CHECK (qty > 0),
    price_cents INTEGER NOT NULL CHECK (price_cents > 0),
    PRIMARY KEY (order_id, product_id)
);

CREATE TABLE order_events (
    id         INTEGER PRIMARY KEY,
    order_id   INTEGER NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    old_status TEXT,
    new_status TEXT    NOT NULL,
    at         TEXT    NOT NULL DEFAULT (datetime('now'))
);

-- Indexes for the queries the application runs.
CREATE INDEX idx_products_category ON products (category, id);
CREATE INDEX idx_orders_customer ON orders (customer_id, created_at);
CREATE INDEX idx_order_lines_product ON order_lines (product_id);

-- Every status change is recorded, whichever code path made it.
CREATE TRIGGER trg_order_status AFTER UPDATE OF status ON orders
WHEN OLD.status IS NOT NEW.status
BEGIN
    INSERT INTO order_events (order_id, old_status, new_status) VALUES (NEW.id, OLD.status, NEW.status);
END;

-- A cancelled order puts its items back in stock.
CREATE TRIGGER trg_order_cancelled AFTER UPDATE OF status ON orders
WHEN NEW.status = 'cancelled' AND OLD.status <> 'cancelled'
BEGIN
    UPDATE products
    SET stock = stock + (SELECT qty FROM order_lines WHERE order_id = NEW.id AND product_id = products.id)
    WHERE id IN (SELECT product_id FROM order_lines WHERE order_id = NEW.id);
END;

-- One row per order with its total: reports and the app read this instead of repeating the SUM.
CREATE VIEW order_totals AS
SELECT o.id, o.customer_id, o.status, o.created_at,
       COALESCE(SUM(l.qty * l.price_cents), 0) AS total_cents,
       COALESCE(SUM(l.qty), 0) AS items
FROM orders o
LEFT JOIN order_lines l ON l.order_id = o.id
GROUP BY o.id;
@@file queries.sql
-- queries.sql: every query the application runs, by name. shop.py loads this file and runs
-- them with named parameters (:name), so values are never pasted into the SQL text.

-- name: create_order
-- A new order for :customer_id; returns its id.
INSERT INTO orders (customer_id) VALUES (:customer_id) RETURNING id;

-- name: reserve_stock
-- Takes :qty of :product_id out of stock, but only if there's enough: the check and the
-- change are one statement, so two customers can never both buy the last item.
-- shop.py looks at how many rows changed: 0 means "not enough stock".
UPDATE products SET stock = stock - :qty
WHERE id = :product_id AND stock >= :qty;

-- name: add_line
-- Adds :qty of :product_id to order :order_id at the product's current price.
-- The same product twice in one order adds to the existing line instead of failing.
INSERT INTO order_lines (order_id, product_id, qty, price_cents)
SELECT :order_id, id, :qty, price_cents FROM products WHERE id = :product_id
ON CONFLICT (order_id, product_id) DO UPDATE SET qty = qty + excluded.qty;

-- name: search_products
-- One page of products (id, sku, name, category, price_cents, stock):
--   name contains :q (any case; an empty :q matches everything),
--   category is :category, or any category when :category is NULL,
--   only products with an id greater than :after (keyset pagination: the last id of the previous page),
--   ordered by id, at most :limit rows.
SELECT id, sku, name, category, price_cents, stock
FROM products
WHERE name LIKE '%' || :q || '%'
  AND (:category IS NULL OR category = :category)
  AND id > :after
ORDER BY id
LIMIT :limit;

-- name: set_status
-- Changes order :id to :status, with optimistic locking: only if its version is still :version,
-- and the version goes up by one. 0 rows changed means someone else changed the order first.
UPDATE orders SET status = :status, version = version + 1
WHERE id = :id AND version = :version;

-- name: get_order
SELECT id, customer_id, status, total_cents, items, created_at,
       (SELECT version FROM orders WHERE orders.id = order_totals.id) AS version
FROM order_totals WHERE id = :id;

-- name: order_history
SELECT old_status, new_status FROM order_events WHERE order_id = :id ORDER BY id;

-- name: sales_by_category
-- Revenue per category from orders that are paid or shipped (not new, not cancelled):
-- category, units (total qty), revenue_cents (qty * price of the lines), and rank
-- (1 = the highest revenue, with RANK() so equal revenues share a rank).
-- Highest revenue first, then by category name.
SELECT p.category,
       SUM(l.qty) AS units,
       SUM(l.qty * l.price_cents) AS revenue_cents,
       RANK() OVER (ORDER BY SUM(l.qty * l.price_cents) DESC) AS rank
FROM order_lines l
JOIN orders o ON o.id = l.order_id
JOIN products p ON p.id = l.product_id
WHERE o.status IN ('paid', 'shipped')
GROUP BY p.category
ORDER BY revenue_cents DESC, p.category;

-- name: top_customers
-- Customers by money spent on paid or shipped orders, with their number of such orders.
SELECT c.name, COUNT(t.id) AS orders, SUM(t.total_cents) AS spent_cents
FROM customers c
JOIN order_totals t ON t.customer_id = c.id AND t.status IN ('paid', 'shipped')
GROUP BY c.id
ORDER BY spent_cents DESC, c.name
LIMIT :limit;
@@file seed.sql
-- seed.sql: sample data for trying the shop and for the tests.
INSERT INTO customers (id, email, name) VALUES
 (1, 'aiko@example.jp', 'Aiko Mori'),
 (2, 'ken@example.jp', 'Ken Sato'),
 (3, 'yuki@example.jp', 'Yuki Tanaka');

INSERT INTO products (id, sku, name, category, price_cents, stock) VALUES
 (1, 'CBL-HDMI-2M', 'HDMI cable 2 m', 'cables', 1290, 120),
 (2, 'CBL-USBC-1M', 'USB-C cable 1 m', 'cables', 990, 200),
 (3, 'DCK-USBC-PRO', 'USB-C dock Pro', 'docks', 18900, 4),
 (4, 'DCK-TB4', 'Thunderbolt 4 dock', 'docks', 32900, 2),
 (5, 'INP-KB-JIS', 'Keyboard (JIS layout)', 'input', 6490, 35),
 (6, 'INP-MOUSE-W', 'Wireless mouse', 'input', 2990, 50),
 (7, 'AUD-HEADSET', 'USB headset with mic', 'audio', 7990, 18),
 (8, 'DSP-27-4K', '27" 4K monitor', 'displays', 54800, 6),
 (9, 'CBL-DP-2M', 'DisplayPort cable 2 m', 'cables', 1490, 0),
 (10, 'INP-KB-US', 'Keyboard (US layout)', 'input', 6490, 12);
@@file shop.py
"""shop.py: the application's data layer. All SQL lives in schema.sql and queries.sql;
this file only runs it, inside transactions, and turns results into Python values."""
import re
import sqlite3
from pathlib import Path

HERE = Path(__file__).parent


class OutOfStock(Exception):
    pass


class Conflict(Exception):
    """The order was changed by someone else since it was loaded."""


def load_queries(path=HERE / "queries.sql"):
    """Splits queries.sql at its '-- name: x' lines into {name: sql}."""
    queries = {}
    for block in re.split(r"(?m)^-- name: ", path.read_text(encoding="utf-8"))[1:]:
        name, _, sql = block.partition("\n")
        queries[name.strip()] = sql.strip()
    return queries


class Shop:
    def __init__(self, path=":memory:"):
        self.db = sqlite3.connect(path, isolation_level=None)   # we BEGIN and COMMIT ourselves
        self.db.row_factory = sqlite3.Row
        self.db.execute("PRAGMA foreign_keys = ON")
        self.q = load_queries()
        if not self.db.execute("SELECT 1 FROM sqlite_master WHERE name = 'orders'").fetchone():
            self.db.executescript((HERE / "schema.sql").read_text(encoding="utf-8"))

    def run(self, name, **params):
        sql = self.q[name]
        if "TODO" in sql:
            raise NotImplementedError(f"TODO: write the {name} query in queries.sql")
        return self.db.execute(sql, params)

    def seed(self, path=HERE / "seed.sql"):
        self.db.executescript(path.read_text(encoding="utf-8"))

    def place_order(self, customer_id, items):
        """items: [(product_id, qty), ...]. Everything happens, or nothing does."""
        self.db.execute("BEGIN IMMEDIATE")      # take the write lock now, not halfway through
        try:
            order_id = self.run("create_order", customer_id=customer_id).fetchone()["id"]
            for product_id, qty in items:
                if self.run("reserve_stock", product_id=product_id, qty=qty).rowcount != 1:
                    raise OutOfStock(f"not enough stock for product {product_id}")
                self.run("add_line", order_id=order_id, product_id=product_id, qty=qty)
            self.db.execute("COMMIT")
            return order_id
        except BaseException:
            self.db.execute("ROLLBACK")
            raise

    def search(self, q="", category=None, after=0, limit=20):
        rows = self.run("search_products", q=q, category=category, after=after, limit=limit)
        return [dict(r) for r in rows]

    def get_order(self, order_id):
        row = self.run("get_order", id=order_id).fetchone()
        return dict(row) if row else None

    def set_status(self, order_id, status, version):
        if self.run("set_status", id=order_id, status=status, version=version).rowcount != 1:
            raise Conflict(f"order {order_id} was changed by someone else; reload it")

    def history(self, order_id):
        return [(r["old_status"], r["new_status"]) for r in self.run("order_history", id=order_id)]

    def sales_by_category(self):
        return [dict(r) for r in self.run("sales_by_category")]

    def top_customers(self, limit=3):
        return [dict(r) for r in self.run("top_customers", limit=limit)]
@@file test_shop.py
# Run: python -m unittest -v
import unittest

from shop import Conflict, OutOfStock, Shop


def stock(shop, product_id):
    return shop.db.execute("SELECT stock FROM products WHERE id = ?", (product_id,)).fetchone()[0]


class ShopTest(unittest.TestCase):
    def setUp(self):
        self.shop = Shop()
        self.shop.seed()


class SearchTest(ShopTest):
    def test_text_search_ignores_case(self):
        self.assertEqual([p["sku"] for p in self.shop.search("KEYBOARD")], ["INP-KB-JIS", "INP-KB-US"])

    def test_category_filter(self):
        self.assertEqual([p["id"] for p in self.shop.search(category="docks")], [3, 4])

    def test_no_filters_returns_everything_in_id_order(self):
        self.assertEqual([p["id"] for p in self.shop.search()], list(range(1, 11)))

    def test_keyset_pages(self):
        page1 = self.shop.search(category="cables", limit=2)
        page2 = self.shop.search(category="cables", after=page1[-1]["id"], limit=2)
        self.assertEqual([p["id"] for p in page1], [1, 2])
        self.assertEqual([p["id"] for p in page2], [9])

    def test_columns(self):
        self.assertEqual(self.shop.search("dock Pro")[0],
                         {"id": 3, "sku": "DCK-USBC-PRO", "name": "USB-C dock Pro", "category": "docks",
                          "price_cents": 18900, "stock": 4})


class OrderTest(ShopTest):
    def test_place_order_takes_stock_and_copies_prices(self):
        order_id = self.shop.place_order(1, [(3, 2), (1, 3)])
        self.assertEqual((stock(self.shop, 3), stock(self.shop, 1)), (2, 117))
        order = self.shop.get_order(order_id)
        self.assertEqual((order["status"], order["items"], order["total_cents"]), ("new", 5, 2 * 18900 + 3 * 1290))

    def test_the_order_keeps_its_price(self):
        order_id = self.shop.place_order(1, [(5, 1)])
        self.shop.db.execute("UPDATE products SET price_cents = 9999 WHERE id = 5")
        self.assertEqual(self.shop.get_order(order_id)["total_cents"], 6490)

    def test_same_product_twice_adds_up(self):
        order_id = self.shop.place_order(2, [(6, 1), (6, 2)])
        self.assertEqual(self.shop.get_order(order_id)["items"], 3)
        self.assertEqual(stock(self.shop, 6), 47)

    def test_out_of_stock_changes_nothing(self):
        with self.assertRaises(OutOfStock):
            self.shop.place_order(1, [(1, 1), (4, 3)])          # only 2 Thunderbolt docks
        self.assertEqual((stock(self.shop, 1), stock(self.shop, 4)), (120, 2))
        self.assertEqual(self.shop.db.execute("SELECT COUNT(*) FROM orders").fetchone()[0], 0)

    def test_exactly_the_last_items(self):
        self.shop.place_order(1, [(4, 2)])
        self.assertEqual(stock(self.shop, 4), 0)
        with self.assertRaises(OutOfStock):
            self.shop.place_order(2, [(4, 1)])

    def test_unknown_customer_is_refused(self):
        with self.assertRaises(Exception):
            self.shop.place_order(99, [(1, 1)])
        self.assertEqual(stock(self.shop, 1), 120)


class StatusTest(ShopTest):
    def test_status_with_version(self):
        order_id = self.shop.place_order(1, [(1, 1)])
        self.shop.set_status(order_id, "paid", version=1)
        order = self.shop.get_order(order_id)
        self.assertEqual((order["status"], order["version"]), ("paid", 2))

    def test_stale_version_is_a_conflict(self):
        order_id = self.shop.place_order(1, [(1, 1)])
        self.shop.set_status(order_id, "paid", version=1)       # Ken
        with self.assertRaises(Conflict):
            self.shop.set_status(order_id, "cancelled", version=1)   # Aiko, with the old version
        self.assertEqual(self.shop.get_order(order_id)["status"], "paid")

    def test_history_and_cancel_restock(self):
        order_id = self.shop.place_order(1, [(3, 2), (7, 1)])
        self.shop.set_status(order_id, "paid", version=1)
        self.shop.set_status(order_id, "cancelled", version=2)
        self.assertEqual(self.shop.history(order_id), [("new", "paid"), ("paid", "cancelled")])
        self.assertEqual((stock(self.shop, 3), stock(self.shop, 7)), (4, 18))

    def test_invalid_status_is_refused_by_the_database(self):
        order_id = self.shop.place_order(1, [(1, 1)])
        with self.assertRaises(Exception):
            self.shop.set_status(order_id, "lost", version=1)


class ReportTest(ShopTest):
    def setUp(self):
        super().setUp()
        s = self.shop
        a = s.place_order(1, [(8, 1), (1, 2)])       # displays 54800, cables 2580
        b = s.place_order(2, [(5, 2), (6, 1)])       # input 12980 + 2990 = 15970
        c = s.place_order(3, [(3, 1)])               # docks 18900, stays new: not counted
        d = s.place_order(3, [(7, 2)])               # audio 15980, cancelled: not counted
        e = s.place_order(2, [(10, 1), (2, 3)])      # input 6490, cables 2970
        for order_id in (a, b, e):
            s.set_status(order_id, "paid", version=1)
        s.set_status(e, "shipped", version=2)
        s.set_status(d, "cancelled", version=1)
        self.unused = c

    def test_sales_by_category(self):
        self.assertEqual(self.shop.sales_by_category(), [
            {"category": "displays", "units": 1, "revenue_cents": 54800, "rank": 1},
            {"category": "input", "units": 4, "revenue_cents": 22460, "rank": 2},
            {"category": "cables", "units": 5, "revenue_cents": 5550, "rank": 3},
        ])

    def test_equal_revenue_shares_a_rank(self):
        order_id = self.shop.place_order(1, [(4, 1), (2, 1)])   # docks 32900, cables 990
        self.shop.set_status(order_id, "paid", version=1)
        order_id = self.shop.place_order(1, [(2, 5)])           # cables 4950: cables 11490
        self.shop.set_status(order_id, "paid", version=1)
        ranks = {r["category"]: r["rank"] for r in self.shop.sales_by_category()}
        self.assertEqual(ranks, {"displays": 1, "docks": 2, "input": 3, "cables": 4})
        self.shop.db.execute("UPDATE order_lines SET price_cents = 22460 WHERE product_id = 4")
        ranks = [r["rank"] for r in self.shop.sales_by_category()]
        self.assertEqual(ranks, [1, 2, 2, 4])

    def test_top_customers(self):
        self.assertEqual(self.shop.top_customers(), [
            {"name": "Aiko Mori", "orders": 1, "spent_cents": 57380},
            {"name": "Ken Sato", "orders": 2, "spent_cents": 25430},
        ])


if __name__ == "__main__":
    unittest.main()
@@file main.py
"""A short demo: python main.py   (creates shop.db next to this file the first time)."""
from pathlib import Path

from shop import Conflict, OutOfStock, Shop

db_file = Path(__file__).parent / "shop.db"
first_time = not db_file.exists()
shop = Shop(str(db_file))
if first_time:
    shop.seed()

print("Docks in stock:")
for p in shop.search(category="docks"):
    print(f"  {p['sku']:<14} {p['name']:<22} ${p['price_cents'] / 100:>8,.2f}  stock {p['stock']}")

try:
    order_id = shop.place_order(1, [(3, 1), (1, 2)])
    print(f"\nOrder {order_id} placed: {shop.get_order(order_id)}")
    shop.set_status(order_id, "paid", version=1)
    shop.set_status(order_id, "paid", version=1)     # a second, stale save
except OutOfStock as e:
    print("\nCan't order:", e)
except Conflict as e:
    print("\nConflict:", e)

print("\nSales by category:")
for r in shop.sales_by_category():
    print(f"  {r['rank']}. {r['category']:<9} {r['units']:>3} units  ${r['revenue_cents'] / 100:>10,.2f}")

@@drill
Which clause returns the rows an INSERT or UPDATE changed?
= RETURNING
? INSERT ... RETURNING id.

@@text
🎉 Constraints, transactions, conditional updates, optimistic locking, triggers, views and window functions: you've used the database the way production systems do, letting it guarantee what application code can't.

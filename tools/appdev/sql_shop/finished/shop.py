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

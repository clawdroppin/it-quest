OUT = "content/sql/t2_01_shop.md"
BASE = "tools/appdev/sql_shop"
FILES = ["schema.sql", "queries.sql", "seed.sql", "shop.py", "test_shop.py", "main.py"]

PARTS = [
    ("text", r'''
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
'''),
    ("project", "shop_db", "Shop Order Database",
     "Write the four queries marked `TODO` in `queries.sql` until `python -m unittest` passes, then run `python main.py`.",
     BASE + "/starter", FILES),
    ("text", r'''
@@text
## Stretch goals

- **Migrations**: turn `schema.sql` into `migrations/001_initial.sql`, track the version with `PRAGMA user_version`, and write `002_add_discounts.sql` (a `discount_pct` column on orders, used by the `order_totals` view).
- **Full-text search** with SQLite's FTS5: a `products_fts` virtual table kept in sync by triggers, and `MATCH` instead of `LIKE`.
- **Monthly report**: revenue per month with a running total (`SUM(...) OVER (ORDER BY month)`) and the change from the previous month (`LAG`).
- **PostgreSQL**: run the same schema on PostgreSQL (`SERIAL` or `GENERATED ALWAYS AS IDENTITY`, `NUMERIC` money, `now()`), using psycopg. What has to change? What doesn't?
- **Concurrency test**: two threads, each with its own connection to a file database, buying the last dock at once. Exactly one must succeed.
'''),
    ("project", "shop_db_finished", "Shop Order Database: the finished version",
     "A complete solution to compare with yours.",
     BASE + "/finished", FILES),
    ("text", r'''
@@text
🎉 Constraints, transactions, conditional updates, optimistic locking, triggers, views and window functions: you've used the database the way production systems do, letting it guarantee what application code can't.
'''),
]

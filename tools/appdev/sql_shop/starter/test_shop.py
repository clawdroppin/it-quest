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

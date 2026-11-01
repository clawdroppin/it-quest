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

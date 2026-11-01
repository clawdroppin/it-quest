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

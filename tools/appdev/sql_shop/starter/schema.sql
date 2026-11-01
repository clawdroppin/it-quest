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

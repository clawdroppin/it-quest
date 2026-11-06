@@lesson n8-project Project: A Software Licence Compliance Database
@@concepts errors, automation
@@text
Software audits are a real IT pain: a vendor asks "prove you aren't using more seats than you paid for", and the honest answer is often "we don't know". In this project you'll build the database that answers it, using what this module covered: constraints that protect the data, a trigger that enforces a business rule, and reports for the IT manager and finance.

## The model

```text
products(id, name, vendor)
licences(id, product_id → products, seats, cost_yen, starts, expires)    -- one row per purchase / renewal
installs(id, product_id → products, device_tag, installed)               -- what the inventory scanner found
waitlist(id, product_id, device_tag, requested)                          -- installs that had to wait for seats
```

Design notes:

- A product can have **several** licence purchases (a 50-seat deal, then 10 more). Seats available = sum of seats of licences that are valid **today**.
- `installs` has a UNIQUE `(product_id, device_tag)`: a device installs a product once.
- Dates are ISO text; "today" is a parameter. In the exercises it's `2026-11-01`, so the results never change.
- Costs are integers in yen.

## Business rules

1. Never install beyond the available seats: a new install request goes to the **waitlist** instead (a trigger).
2. Report each product as `OK`, `OVER by n` (non-compliant: audit risk!) or `spare n` (paying for unused seats).
3. Show renewals due in the next 60 days with what they'll cost.

## Why put rules in the database?

The inventory scanner, the self-service portal and an admin's manual fix all write to `installs`. A trigger enforces the seat rule for **all** of them; a check in one application protects only that application.

@@quiz
Why compute "available seats" as a sum over valid licences instead of storing a seats column on products?
* Purchases and renewals have their own dates and quantities; summing the valid ones is always correct, while a stored total goes stale
- Sums are faster than columns
- Products can't have numeric columns
- To save space
? Derive totals from the facts unless performance demands a summary.

@@quiz
Why enforce the seat limit with a trigger rather than only in the self-service portal?
- Triggers are faster
* Every path that writes installs (scanner, portal, manual fixes) is covered by the same rule
- Portals can't count
- Triggers are required for foreign keys
? Rules in the database protect all writers.

@@quiz
What does a foreign key from installations to licences guarantee?
* Every installation refers to a licence that actually exists
- Each licence is installed once
- Licences are sorted
- Installations can't be deleted
? Referential integrity stops orphan rows.

@@drill
Which constraint stops a column from holding values outside a rule, such as negative seat counts?
= CHECK
? seats INTEGER CHECK (seats >= 0).

@@flashcards
true-up :: paying the vendor for over-deployed seats
valid licence :: starts <= today < expires
over-deployment :: installs > available seats
shelfware :: licences paid for but unused
renewal window :: licences expiring soon

@@exercise The compliance report
Using the starter data and "today" = `2026-11-01`, show for every product: `product`, `seats` (sum of seats of licences valid today: `starts <= today AND expires > today`; 0 if none), `installed` (installs count) and `status`: `OK` when equal, `OVER by n` when installs exceed seats, `spare n` when seats exceed installs. Sort by product.
@@setup
CREATE TABLE products (id INTEGER PRIMARY KEY, name TEXT NOT NULL UNIQUE, vendor TEXT NOT NULL);
CREATE TABLE licences (id INTEGER PRIMARY KEY, product_id INTEGER NOT NULL REFERENCES products(id),
    seats INTEGER NOT NULL CHECK (seats > 0), cost_yen INTEGER NOT NULL CHECK (cost_yen >= 0),
    starts TEXT NOT NULL, expires TEXT NOT NULL, CHECK (expires > starts));
CREATE TABLE installs (id INTEGER PRIMARY KEY, product_id INTEGER NOT NULL REFERENCES products(id),
    device_tag TEXT NOT NULL, installed TEXT NOT NULL, UNIQUE (product_id, device_tag));
CREATE TABLE waitlist (id INTEGER PRIMARY KEY, product_id INTEGER NOT NULL, device_tag TEXT NOT NULL, requested TEXT NOT NULL);
INSERT INTO products VALUES (1, 'Office Suite', 'Microsoft'), (2, 'Acrobat Pro', 'Adobe'), (3, 'AutoCAD LT', 'Autodesk'), (4, 'Visio', 'Microsoft'), (5, 'Slack Pro', 'Salesforce');
INSERT INTO licences VALUES
  (1, 1, 10, 330000, '2026-04-01', '2027-04-01'),
  (2, 1, 2, 66000, '2026-09-01', '2027-04-01'),
  (3, 2, 3, 81000, '2025-12-01', '2026-12-01'),
  (4, 3, 2, 180000, '2025-11-15', '2026-11-15'),
  (5, 4, 2, 40000, '2024-10-01', '2025-10-01'),
  (6, 5, 20, 210000, '2026-01-01', '2027-01-01');
INSERT INTO installs (product_id, device_tag, installed) VALUES
  (1, 'LT-0042', '2026-04-02'), (1, 'LT-0043', '2026-04-02'), (1, 'LT-0050', '2026-05-21'), (1, 'LT-0051', '2026-01-16'),
  (1, 'LT-0052', '2026-04-02'), (1, 'LT-0060', '2026-09-02'), (1, 'LT-0061', '2026-09-02'), (1, 'DT-0007', '2026-04-03'),
  (1, 'DT-0008', '2026-04-03'), (1, 'SV-0002', '2026-06-01'), (1, 'PH-0101', '2026-04-04'), (1, 'PH-0102', '2026-04-04'),
  (2, 'LT-0042', '2026-01-10'), (2, 'LT-0051', '2026-01-16'), (2, 'DT-0007', '2026-02-01'), (2, 'DT-0008', '2026-02-01'),
  (3, 'LT-0051', '2025-11-20'),
  (4, 'LT-0042', '2025-02-01');
@@starter
SELECT p.name AS product, COUNT(i.id) AS installed
FROM products p LEFT JOIN installs i ON i.product_id = p.id
GROUP BY p.id ORDER BY product;
@@expected
product      | seats | installed | status
-------------+-------+-----------+----------
Acrobat Pro  | 3     | 4         | OVER by 1
AutoCAD LT   | 2     | 1         | spare 1
Office Suite | 12    | 12        | OK
Slack Pro    | 20    | 0         | spare 20
Visio        | 0     | 1         | OVER by 1
@@requires
CASE
expires > '2026-11-01'
@@hint
Aggregate seats and installs **separately** (two CTEs grouped by product) and LEFT JOIN both to products, or you'll get fan-out (each licence row repeated per install). Then `CASE WHEN installed = seats THEN 'OK' WHEN installed > seats THEN 'OVER by ' || (installed - seats) ELSE 'spare ' || (seats - installed) END`.
@@solution
WITH seats AS (
    SELECT product_id, SUM(seats) AS seats FROM licences
    WHERE starts <= '2026-11-01' AND expires > '2026-11-01'
    GROUP BY product_id
),
used AS (
    SELECT product_id, COUNT(*) AS installed FROM installs GROUP BY product_id
)
SELECT p.name AS product,
       COALESCE(s.seats, 0) AS seats,
       COALESCE(u.installed, 0) AS installed,
       CASE WHEN COALESCE(u.installed, 0) = COALESCE(s.seats, 0) THEN 'OK'
            WHEN COALESCE(u.installed, 0) > COALESCE(s.seats, 0) THEN 'OVER by ' || (COALESCE(u.installed, 0) - COALESCE(s.seats, 0))
            ELSE 'spare ' || (COALESCE(s.seats, 0) - COALESCE(u.installed, 0)) END AS status
FROM products p
LEFT JOIN seats s ON s.product_id = p.id
LEFT JOIN used u ON u.product_id = p.id
ORDER BY product;

@@exercise Renewals due soon
Show licences that expire within **60 days** after `2026-11-01` (expires > today and expires <= today + 60 days): `product`, `vendor`, `expires`, `days_left` (integer) and the renewal `cost_yen` formatted with thousands separators (`printf('%,d', cost_yen)`). Add a final row `TOTAL` with the summed cost (other columns empty strings). Sort by expires, with the total last.
@@setup
CREATE TABLE products (id INTEGER PRIMARY KEY, name TEXT NOT NULL UNIQUE, vendor TEXT NOT NULL);
CREATE TABLE licences (id INTEGER PRIMARY KEY, product_id INTEGER NOT NULL REFERENCES products(id),
    seats INTEGER NOT NULL, cost_yen INTEGER NOT NULL, starts TEXT NOT NULL, expires TEXT NOT NULL);
INSERT INTO products VALUES (1, 'Office Suite', 'Microsoft'), (2, 'Acrobat Pro', 'Adobe'), (3, 'AutoCAD LT', 'Autodesk'), (4, 'Visio', 'Microsoft'), (5, 'Slack Pro', 'Salesforce');
INSERT INTO licences VALUES
  (1, 1, 10, 330000, '2026-04-01', '2027-04-01'),
  (2, 1, 2, 66000, '2026-09-01', '2027-04-01'),
  (3, 2, 3, 81000, '2025-12-01', '2026-12-01'),
  (4, 3, 2, 180000, '2025-11-15', '2026-11-15'),
  (5, 4, 2, 40000, '2024-10-01', '2025-10-01'),
  (6, 5, 20, 210000, '2026-01-01', '2027-01-01');
@@starter
SELECT p.name AS product, l.expires FROM licences l JOIN products p ON p.id = l.product_id ORDER BY l.expires;
@@expected
product     | vendor   | expires    | days_left | cost_yen
------------+----------+------------+-----------+---------
AutoCAD LT  | Autodesk | 2026-11-15 | 14        | 180,000
Acrobat Pro | Adobe    | 2026-12-01 | 30        | 81,000
TOTAL       |          |            |           | 261,000
@@requires
julianday
UNION ALL
printf('%,d'
@@hint
CTE `due` with the filter `l.expires > '2026-11-01' AND l.expires <= date('2026-11-01', '+60 days')`. Days left: `CAST(julianday(l.expires) - julianday('2026-11-01') AS INTEGER)`. For the total row, UNION ALL `SELECT 'TOTAL', '', '', '', printf('%,d', SUM(cost_yen)) FROM due`, and sort with a hidden sort key (0 for rows, 1 for the total).
@@solution
WITH due AS (
    SELECT p.name AS product, p.vendor, l.expires,
           CAST(julianday(l.expires) - julianday('2026-11-01') AS INTEGER) AS days_left, l.cost_yen
    FROM licences l JOIN products p ON p.id = l.product_id
    WHERE l.expires > '2026-11-01' AND l.expires <= date('2026-11-01', '+60 days')
)
SELECT product, vendor, expires, days_left, cost_yen FROM (
    SELECT 0 AS k, product, vendor, expires, days_left, printf('%,d', cost_yen) AS cost_yen FROM due
    UNION ALL
    SELECT 1, 'TOTAL', '', '', '', printf('%,d', SUM(cost_yen)) FROM due
)
ORDER BY k, expires;

@@exercise Enforcing seats with a trigger
Add a `BEFORE INSERT` trigger on `installs` that checks the seats valid on the install date (`NEW.installed`). If the product has no free seat, it records the request in `waitlist` and skips the install with `RAISE(IGNORE)`. Then process the starter's four install requests and show the installs per product and the waitlist.
@@setup
CREATE TABLE products (id INTEGER PRIMARY KEY, name TEXT NOT NULL UNIQUE);
CREATE TABLE licences (id INTEGER PRIMARY KEY, product_id INTEGER NOT NULL, seats INTEGER NOT NULL, starts TEXT NOT NULL, expires TEXT NOT NULL);
CREATE TABLE installs (id INTEGER PRIMARY KEY, product_id INTEGER NOT NULL, device_tag TEXT NOT NULL, installed TEXT NOT NULL, UNIQUE (product_id, device_tag));
CREATE TABLE waitlist (id INTEGER PRIMARY KEY, product_id INTEGER NOT NULL, device_tag TEXT NOT NULL, requested TEXT NOT NULL);
INSERT INTO products VALUES (1, 'Acrobat Pro'), (2, 'AutoCAD LT');
INSERT INTO licences VALUES (1, 1, 3, '2025-12-01', '2026-12-01'), (2, 2, 1, '2025-11-15', '2026-11-15');
INSERT INTO installs (product_id, device_tag, installed) VALUES (1, 'LT-0042', '2026-01-10'), (1, 'LT-0051', '2026-01-16');
@@starter
-- CREATE TRIGGER installs_check_seats ...

INSERT INTO installs (product_id, device_tag, installed) VALUES (1, 'DT-0007', '2026-11-01');
INSERT INTO installs (product_id, device_tag, installed) VALUES (1, 'DT-0008', '2026-11-01');
INSERT INTO installs (product_id, device_tag, installed) VALUES (2, 'LT-0050', '2026-11-01');
INSERT INTO installs (product_id, device_tag, installed) VALUES (2, 'LT-0060', '2026-11-02');

SELECT p.name AS product, COUNT(i.id) AS installs
FROM products p LEFT JOIN installs i ON i.product_id = p.id GROUP BY p.id ORDER BY p.name;
SELECT p.name AS product, w.device_tag, w.requested
FROM waitlist w JOIN products p ON p.id = w.product_id ORDER BY w.id;
@@expected
product     | installs
------------+---------
Acrobat Pro | 3
AutoCAD LT  | 1

product     | device_tag | requested
------------+------------+-----------
Acrobat Pro | DT-0008    | 2026-11-01
AutoCAD LT  | LT-0060    | 2026-11-02
@@requires
BEFORE INSERT ON installs
RAISE(IGNORE)
INSERT INTO waitlist
@@hint
`WHEN (SELECT COUNT(*) FROM installs WHERE product_id = NEW.product_id) >= (SELECT COALESCE(SUM(seats), 0) FROM licences WHERE product_id = NEW.product_id AND starts <= NEW.installed AND expires > NEW.installed)`. In the body: `INSERT INTO waitlist (product_id, device_tag, requested) VALUES (NEW.product_id, NEW.device_tag, NEW.installed); SELECT RAISE(IGNORE);`
@@solution
CREATE TRIGGER installs_check_seats BEFORE INSERT ON installs
WHEN (SELECT COUNT(*) FROM installs WHERE product_id = NEW.product_id)
   >= (SELECT COALESCE(SUM(seats), 0) FROM licences
       WHERE product_id = NEW.product_id AND starts <= NEW.installed AND expires > NEW.installed)
BEGIN
    INSERT INTO waitlist (product_id, device_tag, requested) VALUES (NEW.product_id, NEW.device_tag, NEW.installed);
    SELECT RAISE(IGNORE);
END;

INSERT INTO installs (product_id, device_tag, installed) VALUES (1, 'DT-0007', '2026-11-01');
INSERT INTO installs (product_id, device_tag, installed) VALUES (1, 'DT-0008', '2026-11-01');
INSERT INTO installs (product_id, device_tag, installed) VALUES (2, 'LT-0050', '2026-11-01');
INSERT INTO installs (product_id, device_tag, installed) VALUES (2, 'LT-0060', '2026-11-02');

SELECT p.name AS product, COUNT(i.id) AS installs
FROM products p LEFT JOIN installs i ON i.product_id = p.id GROUP BY p.id ORDER BY p.name;
SELECT p.name AS product, w.device_tag, w.requested
FROM waitlist w JOIN products p ON p.id = w.product_id ORDER BY w.id;

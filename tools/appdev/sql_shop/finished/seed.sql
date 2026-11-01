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

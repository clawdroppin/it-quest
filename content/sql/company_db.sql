-- Sakura Systems K.K.: the sample company database used by every SQL lesson.
CREATE TABLE departments (
    id       INTEGER PRIMARY KEY,
    name     TEXT NOT NULL UNIQUE,
    location TEXT NOT NULL
);
INSERT INTO departments VALUES
    (1, 'IT', 'Tokyo'),
    (2, 'Sales', 'Tokyo'),
    (3, 'Finance', 'Osaka'),
    (4, 'Support', 'Osaka'),
    (5, 'Marketing', 'Fukuoka');

CREATE TABLE employees (
    id         INTEGER PRIMARY KEY,
    name       TEXT NOT NULL,
    username   TEXT NOT NULL UNIQUE,
    dept_id    INTEGER REFERENCES departments(id),
    title      TEXT NOT NULL,
    hired      TEXT NOT NULL,          -- 'YYYY-MM-DD'
    salary     INTEGER NOT NULL,       -- yearly, in yen
    manager_id INTEGER REFERENCES employees(id)
);
INSERT INTO employees VALUES
    (1,  'Kenji Tanaka',   'ktanaka',  1, 'IT Manager',         '2015-04-01', 9200000, NULL),
    (2,  'Ana Silva',      'asilva',   1, 'Systems Engineer',   '2019-10-01', 6400000, 1),
    (3,  'Mia Lee',        'mlee',     1, 'Help Desk Analyst',  '2023-04-01', 4100000, 1),
    (4,  'Hiro Sato',      'hsato',    2, 'Sales Manager',      '2016-07-15', 8800000, NULL),
    (5,  'Ben Jones',      'bjones',   2, 'Account Executive',  '2021-01-10', 5600000, 4),
    (6,  'Yuki Kato',      'ykato',    2, 'Account Executive',  '2022-06-01', 5200000, 4),
    (7,  'Rina Chen',      'rchen',    3, 'Finance Manager',    '2014-02-01', 9000000, NULL),
    (8,  'Taro Yamada',    'tyamada',  3, 'Accountant',         '2020-04-01', 5500000, 7),
    (9,  'Sara Ito',       'sito',     4, 'Support Lead',       '2018-09-01', 6100000, NULL),
    (10, 'Leo Martin',     'lmartin',  4, 'Support Agent',      '2024-04-01', 3900000, 9),
    (11, 'Emi Suzuki',     'esuzuki',  4, 'Support Agent',      '2023-10-01', 4000000, 9),
    (12, 'Omar Haddad',    'ohaddad',  NULL, 'Contractor',      '2026-09-01', 4500000, NULL);

CREATE TABLE devices (
    id             INTEGER PRIMARY KEY,
    asset_tag      TEXT NOT NULL UNIQUE,
    type           TEXT NOT NULL,      -- laptop, desktop, phone, server, printer
    model          TEXT NOT NULL,
    os             TEXT,
    purchased      TEXT NOT NULL,
    price          INTEGER NOT NULL,   -- yen
    warranty_until TEXT,
    employee_id    INTEGER REFERENCES employees(id)
);
INSERT INTO devices VALUES
    (1,  'LT-0042', 'laptop',  'ThinkPad T14',     'Windows 11', '2023-04-01', 210000, '2026-03-31', 2),
    (2,  'LT-0043', 'laptop',  'ThinkPad T14',     'Windows 11', '2023-04-01', 210000, '2026-03-31', 3),
    (3,  'LT-0050', 'laptop',  'MacBook Air',      'macOS 15',   '2024-05-20', 198000, '2027-05-19', 5),
    (4,  'LT-0051', 'laptop',  'ThinkPad X1',      'Windows 11', '2025-01-15', 285000, '2028-01-14', 1),
    (5,  'LT-0052', 'laptop',  'ThinkPad T14',     'Windows 10', '2020-04-01', 180000, '2023-03-31', 8),
    (6,  'LT-0060', 'laptop',  'Dell Latitude',    'Windows 11', '2026-09-01', 175000, '2029-08-31', NULL),
    (7,  'DT-0007', 'desktop', 'OptiPlex 7010',    'Windows 11', '2022-10-01', 140000, '2025-09-30', 10),
    (8,  'DT-0008', 'desktop', 'OptiPlex 7010',    'Windows 11', '2022-10-01', 140000, '2025-09-30', 11),
    (9,  'PH-0101', 'phone',   'iPhone 15',        'iOS 18',     '2024-01-10', 124800, '2025-01-09', 4),
    (10, 'PH-0102', 'phone',   'iPhone 15',        'iOS 18',     '2024-01-10', 124800, '2025-01-09', 5),
    (11, 'SV-0001', 'server',  'PowerEdge R650',   'Ubuntu 24.04', '2023-06-01', 1450000, '2028-05-31', NULL),
    (12, 'SV-0002', 'server',  'PowerEdge R650',   'Windows Server 2022', '2023-06-01', 1520000, '2028-05-31', NULL),
    (13, 'PR-0003', 'printer', 'imageRUNNER C3326', NULL,        '2021-03-01', 380000, '2024-02-28', NULL),
    (14, 'LT-0061', 'laptop',  'Dell Latitude',    'Windows 11', '2026-09-01', 175000, '2029-08-31', 12);

CREATE TABLE tickets (
    id           INTEGER PRIMARY KEY,
    title        TEXT NOT NULL,
    category     TEXT NOT NULL,        -- hardware, software, network, account, security
    priority     INTEGER NOT NULL CHECK (priority BETWEEN 1 AND 4),
    status       TEXT NOT NULL,        -- open, in-progress, resolved, closed
    opened       TEXT NOT NULL,        -- 'YYYY-MM-DD HH:MM'
    closed       TEXT,
    requester_id INTEGER REFERENCES employees(id),
    assignee_id  INTEGER REFERENCES employees(id)
);
INSERT INTO tickets VALUES
    (1,  'Printer jam on 3F',            'hardware', 3, 'closed',      '2026-10-01 09:12', '2026-10-01 10:05', 8,  3),
    (2,  'VPN keeps disconnecting',      'network',  2, 'resolved',    '2026-10-03 14:30', '2026-10-04 11:00', 5,  2),
    (3,  'Locked out of account',        'account',  2, 'closed',      '2026-10-06 08:45', '2026-10-06 08:58', 10, 3),
    (4,  'Excel crashes on open',        'software', 3, 'closed',      '2026-10-08 13:20', '2026-10-09 16:40', 7,  3),
    (5,  'Phishing email reported',      'security', 1, 'closed',      '2026-10-10 10:02', '2026-10-10 12:30', 6,  2),
    (6,  'New laptop for contractor',    'hardware', 3, 'resolved',    '2026-10-12 09:00', '2026-10-15 17:00', 1,  3),
    (7,  'Wi-Fi slow in Osaka office',   'network',  2, 'in-progress', '2026-10-20 11:15', NULL,               9,  2),
    (8,  'Password reset',               'account',  4, 'closed',      '2026-10-21 09:30', '2026-10-21 09:41', 11, 3),
    (9,  'File server almost full',      'hardware', 1, 'in-progress', '2026-10-24 07:50', NULL,               1,  2),
    (10, 'Teams camera not working',     'software', 3, 'open',        '2026-10-27 15:05', NULL,               4,  NULL),
    (11, 'Suspicious login from abroad', 'security', 1, 'open',        '2026-10-29 22:40', NULL,               2,  1),
    (12, 'Install accounting software',  'software', 4, 'open',        '2026-10-30 10:00', NULL,               8,  NULL),
    (13, 'Monitor flickering',           'hardware', 4, 'open',        '2026-10-30 16:20', NULL,               10, 3),
    (14, 'Locked out again',             'account',  3, 'resolved',    '2026-10-31 08:05', '2026-10-31 08:20', 10, 3),
    (15, 'Shared drive access request',  'account',  3, 'open',        '2026-10-31 09:10', NULL,               12, NULL);

CREATE TABLE logins (
    id       INTEGER PRIMARY KEY,
    username TEXT NOT NULL,
    ip       TEXT NOT NULL,
    time     TEXT NOT NULL,            -- 'YYYY-MM-DD HH:MM:SS'
    success  INTEGER NOT NULL          -- 1 = success, 0 = failure
);
INSERT INTO logins VALUES
    (1,  'asilva',  '10.0.0.21',    '2026-10-29 08:01:10', 1),
    (2,  'mlee',    '10.0.0.34',    '2026-10-29 08:55:02', 1),
    (3,  'admin',   '203.0.113.7',  '2026-10-29 22:31:00', 0),
    (4,  'admin',   '203.0.113.7',  '2026-10-29 22:31:04', 0),
    (5,  'root',    '203.0.113.7',  '2026-10-29 22:31:09', 0),
    (6,  'asilva',  '203.0.113.7',  '2026-10-29 22:31:15', 0),
    (7,  'asilva',  '203.0.113.7',  '2026-10-29 22:31:22', 0),
    (8,  'asilva',  '203.0.113.7',  '2026-10-29 22:31:30', 1),
    (9,  'bjones',  '10.0.0.40',    '2026-10-30 09:02:44', 1),
    (10, 'lmartin', '10.0.1.15',    '2026-10-30 09:15:00', 0),
    (11, 'lmartin', '10.0.1.15',    '2026-10-30 09:15:20', 1),
    (12, 'test',    '198.51.100.23','2026-10-30 03:12:00', 0),
    (13, 'test',    '198.51.100.23','2026-10-30 03:12:03', 0),
    (14, 'ktanaka', '10.0.0.10',    '2026-10-31 07:45:00', 1),
    (15, 'oracle',  '198.51.100.23','2026-10-31 03:00:00', 0),
    (16, 'mlee',    '10.0.0.34',    '2026-10-31 08:50:00', 1);

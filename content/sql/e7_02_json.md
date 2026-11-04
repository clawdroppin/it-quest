@@lesson e7-json JSON in the Database
@@concepts json, maps
@@text
Modern systems speak JSON: API responses, application logs, configuration, device inventories from MDM tools. All major databases can now store JSON and query inside it, which lets you mix the reliability of tables with the flexibility of documents.

## When to use JSON columns

Good fits:

- **Raw payloads** you want to keep exactly as received (webhook bodies, audit details).
- **Sparse or varying attributes**: a printer has `toner_level`, a phone has `imei`, a server has `rack`. One JSON `attrs` column beats 40 mostly-NULL columns.
- **Data whose shape you don't control** (vendor APIs).

Bad fits: anything you join on, filter on constantly, or need constraints for (foreign keys, NOT NULL, UNIQUE). Those belong in real columns. A common design is a table with real columns for the core fields **plus** one JSON column for the rest.

## JSON in SQLite

SQLite stores JSON as TEXT and provides functions (built in since 3.38, and included in practically every earlier build):

| Function | Does |
|---|---|
| `json_extract(doc, '$.a.b')` | value at a path (JSON arrays/objects come back as text) |
| `doc ->> '$.a'` | same, as an SQL value (3.38+) |
| `doc -> '$.a'` | the JSON text of the value (3.38+) |
| `json_type(doc, path)` | 'object', 'array', 'text', 'integer', 'null', ... |
| `json_array_length(doc, path)` | number of elements |
| `json_object('k', v, ...)` / `json_array(...)` | build JSON |
| `json_group_array(x)` / `json_group_object(k, v)` | aggregate rows into JSON |
| `json_each(doc)` / `json_tree(doc)` | table-valued: one row per element |
| `json_set`, `json_insert`, `json_replace`, `json_remove` | modified copies |
| `json_valid(doc)` | 1 if well-formed |

Paths start with `$`: `$.os.version`, `$.tags[0]`, `$.disks[#-1]` (the last element).

```sql
SELECT json_extract('{"host": "sv-01", "disks": [{"size": 512}, {"size": 2048}]}', '$.disks[1].size') AS second_disk;
```

## Rows from arrays: json_each

`json_each` turns an array (or object) into rows with columns `key`, `value`, `type` and more. Join it to the table it came from:

```sql
WITH inv(host, doc) AS (VALUES
    ('sv-01', '{"tags": ["prod", "db"]}'),
    ('sv-02', '{"tags": ["test"]}'))
SELECT inv.host, j.value AS tag
FROM inv, json_each(inv.doc, '$.tags') AS j
ORDER BY host, tag;
```

## Building JSON for APIs

The reverse is just as useful: produce a JSON document straight from SQL, for an API response or an export:

```sql
SELECT json_object('department', d.name,
                   'people', json_group_array(e.username)) AS doc
FROM departments d JOIN employees e ON e.dept_id = d.id
WHERE d.id = 1
GROUP BY d.id;
```

(The order inside `json_group_array` follows the order rows arrive in; sort in a subquery when it matters.)

## Indexing JSON

Filtering on `json_extract(attrs, '$.rack')` scans every row. Fix it with an **expression index** or a **generated column**:

```sql,norun
CREATE INDEX devices_rack ON devices (json_extract(attrs, '$.rack'));
-- or
ALTER TABLE devices ADD COLUMN rack TEXT GENERATED ALWAYS AS (json_extract(attrs, '$.rack'));
CREATE INDEX devices_rack2 ON devices (rack);
```

## JSON in PostgreSQL

PostgreSQL has two types: `json` (text, kept as written) and **`jsonb`** (parsed binary; use this one). Operators: `->` (JSON value), `->>` (text), `#>` (path), `@>` (contains), `?` (has key). A **GIN** index on a jsonb column makes `@>` containment queries fast across millions of rows:

```sql,norun
CREATE INDEX ON devices USING GIN (attrs);
SELECT * FROM devices WHERE attrs @> '{"tags": ["prod"]}';
SELECT attrs->>'serial' FROM devices WHERE attrs ? 'serial';
```

SQL Server uses `JSON_VALUE`, `JSON_QUERY` and `OPENJSON`; MySQL has a `JSON` type with `->` and `->>`. The ideas carry over; the spelling differs.

## Validating JSON

Since JSON is just text to SQLite, add a CHECK: `attrs TEXT CHECK (json_valid(attrs))`. For required keys, add more checks (`json_type(attrs, '$.serial') = 'text'`). PostgreSQL rejects invalid JSON automatically for json/jsonb columns.

@@quiz
Which is a good use of a JSON column?
- A customer_id you join on in every query
* Device attributes that differ by device type and are rarely filtered on
- An email column that must be UNIQUE
- A price used in every report
? Keep join keys and constrained fields as real columns.

@@quiz
What does `json_each(doc, '$.tags')` return?
- The number of tags
* One row per element of the tags array, with key and value columns
- The tags as a comma-separated string
- A JSON array
? It's a table-valued function you can join to.

@@quiz
How do you make filtering on `json_extract(attrs, '$.rack')` fast in SQLite?
- You can't
- Store attrs as BLOB
* Create an index on that expression, or on a generated column that extracts it
- Use LIKE
? Expression indexes and generated columns both work.

@@flashcards
`json_extract(doc, '$.a.b')` :: read a value at a path
`->>` :: extract as SQL value (SQLite 3.38+, PostgreSQL, MySQL)
`json_each` :: rows from an array
`json_group_array` :: aggregate rows into a JSON array
`json_object` :: build an object
`json_valid` :: CHECK that text is JSON
`jsonb` :: PostgreSQL's binary JSON (use it)
`@>` :: jsonb containment
GIN index :: fast jsonb searches
`$.disks[#-1]` :: last element (SQLite)

@@exercise Reading MDM inventory
An MDM tool sends one JSON document per device. Show `serial`, `os` (the `name` and `version` joined with a space), `ram_gb` and the `disks` count for each device, sorted by serial.
@@setup
CREATE TABLE mdm (id INTEGER PRIMARY KEY, payload TEXT CHECK (json_valid(payload)));
INSERT INTO mdm (payload) VALUES
 ('{"serial": "PF3K9", "os": {"name": "Windows", "version": "11 24H2"}, "ram_gb": 16, "disks": [{"size": 512, "type": "ssd"}]}'),
 ('{"serial": "C02ZX", "os": {"name": "macOS", "version": "15.1"}, "ram_gb": 8, "disks": [{"size": 256, "type": "ssd"}]}'),
 ('{"serial": "R650A", "os": {"name": "Ubuntu", "version": "24.04"}, "ram_gb": 256, "disks": [{"size": 960, "type": "ssd"}, {"size": 960, "type": "ssd"}, {"size": 8000, "type": "hdd"}]}');
@@starter
SELECT payload FROM mdm;
@@expected
serial | os              | ram_gb | disks
-------+-----------------+--------+------
C02ZX  | macOS 15.1      | 8      | 1
PF3K9  | Windows 11 24H2 | 16     | 1
R650A  | Ubuntu 24.04    | 256    | 3
@@requires
json_extract
json_array_length
@@hint
`json_extract(payload, '$.os.name') || ' ' || json_extract(payload, '$.os.version')`, `json_array_length(payload, '$.disks')`.
@@solution
SELECT json_extract(payload, '$.serial') AS serial,
       json_extract(payload, '$.os.name') || ' ' || json_extract(payload, '$.os.version') AS os,
       json_extract(payload, '$.ram_gb') AS ram_gb,
       json_array_length(payload, '$.disks') AS disks
FROM mdm
ORDER BY serial;

@@exercise Storage per disk type
Using the same MDM data, unpack every disk with `json_each` and show, per `disk_type`, the number of `disks` and the `total_gb`. Sort by disk_type.
@@setup
CREATE TABLE mdm (id INTEGER PRIMARY KEY, payload TEXT CHECK (json_valid(payload)));
INSERT INTO mdm (payload) VALUES
 ('{"serial": "PF3K9", "os": {"name": "Windows", "version": "11 24H2"}, "ram_gb": 16, "disks": [{"size": 512, "type": "ssd"}]}'),
 ('{"serial": "C02ZX", "os": {"name": "macOS", "version": "15.1"}, "ram_gb": 8, "disks": [{"size": 256, "type": "ssd"}]}'),
 ('{"serial": "R650A", "os": {"name": "Ubuntu", "version": "24.04"}, "ram_gb": 256, "disks": [{"size": 960, "type": "ssd"}, {"size": 960, "type": "ssd"}, {"size": 8000, "type": "hdd"}]}');
@@starter
SELECT json_extract(payload, '$.disks') FROM mdm;
@@expected
disk_type | disks | total_gb
----------+-------+---------
hdd       | 1     | 8000
ssd       | 4     | 2688
@@requires
json_each
GROUP BY
@@hint
`FROM mdm, json_each(mdm.payload, '$.disks') AS d`; each `d.value` is a disk object, so read `json_extract(d.value, '$.type')` and `'$.size'`. Don't call the result column `type`: json_each has its own `type` column (`'object'`), and `GROUP BY type` would group by that instead.
@@solution
SELECT json_extract(d.value, '$.type') AS disk_type,
       COUNT(*) AS disks,
       SUM(json_extract(d.value, '$.size')) AS total_gb
FROM mdm, json_each(mdm.payload, '$.disks') AS d
GROUP BY disk_type
ORDER BY disk_type;

@@exercise An API response from SQL
Produce one JSON document per department that has employees: `{"dept": ..., "location": ..., "headcount": n, "staff": [usernames sorted]}`. Show the column `doc`, one row per department, sorted by department name.
@@starter
SELECT name FROM departments ORDER BY name;
@@expected
doc
----------------------------------------------------------------------------------------
{"dept":"Finance","location":"Osaka","headcount":2,"staff":["rchen","tyamada"]}
{"dept":"IT","location":"Tokyo","headcount":3,"staff":["asilva","ktanaka","mlee"]}
{"dept":"Sales","location":"Tokyo","headcount":3,"staff":["bjones","hsato","ykato"]}
{"dept":"Support","location":"Osaka","headcount":3,"staff":["esuzuki","lmartin","sito"]}
@@requires
json_object
json_group_array
@@hint
Sort usernames in a subquery first: `FROM (SELECT * FROM employees ORDER BY username) e JOIN departments d ON d.id = e.dept_id GROUP BY d.id`. Then `json_object('dept', d.name, 'location', d.location, 'headcount', COUNT(*), 'staff', json_group_array(e.username))`.
@@solution
SELECT json_object('dept', d.name, 'location', d.location,
                   'headcount', COUNT(*), 'staff', json_group_array(e.username)) AS doc
FROM (SELECT * FROM employees ORDER BY username) e
JOIN departments d ON d.id = e.dept_id
GROUP BY d.id
ORDER BY d.name;

@@lesson m8-databases Databases from Node.js: SQLite, PostgreSQL, Migrations and ORMs
@@concepts files, objects
@@text
Almost every real application stores data in a database. This lesson covers how Node.js talks to databases safely and efficiently, from the built-in SQLite module to PostgreSQL connection pools, migrations and ORMs.

## Built-in SQLite

Node.js 22.5+ ships `node:sqlite`, a synchronous SQLite driver (marked experimental, but stable enough for tools and tests):

```javascript,norun
const { DatabaseSync } = require("node:sqlite");
const db = new DatabaseSync("inventory.db");            // or ":memory:"
db.exec(`CREATE TABLE IF NOT EXISTS assets (
  tag TEXT PRIMARY KEY, kind TEXT NOT NULL, owner TEXT, purchased TEXT)`);

const insert = db.prepare("INSERT INTO assets (tag, kind, owner) VALUES (?, ?, ?)");
insert.run("IT-00042", "laptop", "Ana");

const byOwner = db.prepare("SELECT tag, kind FROM assets WHERE owner = ? ORDER BY tag");
console.log(byOwner.all("Ana"));                        // [{ tag: 'IT-00042', kind: 'laptop' }]
```

SQLite is perfect for CLI tools, desktop apps (Electron/Tauri), tests and small sites. `better-sqlite3` is the popular, faster third-party alternative.

## PostgreSQL with a pool

Servers use a client/server database (PostgreSQL, MySQL, SQL Server) and a **connection pool**: opening a connection takes tens of milliseconds and the server allows a limited number, so a pool keeps a few open and lends them out.

```javascript,norun
const { Pool } = require("pg");
const pool = new Pool({ connectionString: process.env.DATABASE_URL, max: 10 });

const { rows } = await pool.query(
  "SELECT id, title FROM tickets WHERE status = $1 AND team = $2 LIMIT $3",
  ["open", team, 50],                                    // parameters: never string-concatenate
);

const client = await pool.connect();                    // a transaction needs one connection
try {
  await client.query("BEGIN");
  await client.query("UPDATE accounts SET balance = balance - $1 WHERE id = $2", [amount, from]);
  await client.query("UPDATE accounts SET balance = balance + $1 WHERE id = $2", [amount, to]);
  await client.query("COMMIT");
} catch (e) {
  await client.query("ROLLBACK");
  throw e;
} finally {
  client.release();                                     // always give it back to the pool
}
```

## Parameters, always

Placeholders (`?` in SQLite/MySQL, `$1` in PostgreSQL) send values separately from the SQL text, so input can never change the query. Table and column **names** can't be parameters: if they come from input (a sort column, say), check them against an allow-list.

## The N+1 problem

```javascript,norun
const tickets = await db.query("SELECT * FROM tickets LIMIT 100");
for (const t of tickets.rows) {
  t.user = (await db.query("SELECT name FROM users WHERE id = $1", [t.user_id])).rows[0];   // 100 more queries!
}
```

One query for the list plus one per row: with network latency of 1 ms, that's 100 ms wasted. Fix it with a **JOIN**, or a second query with `WHERE id = ANY($1)` and merging in JavaScript (the "data loader" pattern).

## Migrations

Schema changes must be versioned and repeatable, like code. A **migration** is a numbered script (`003_add_priority.sql`); a migration tool records which ones ran in a table and applies the rest in order, each in a transaction. Tools: Knex, node-pg-migrate, Prisma Migrate, Drizzle Kit. Rules: never edit a migration that has run in production (add a new one), and make changes backwards compatible so old and new app versions can run during a deploy.

## Query builders and ORMs

| Tool | Style |
|---|---|
| raw driver (`pg`, `node:sqlite`) | SQL strings with parameters |
| **Knex**, **Kysely** | query builders: `db.selectFrom("tickets").where("status", "=", "open")` (Kysely is fully typed) |
| **Drizzle** | typed, SQL-like schema and queries |
| **Prisma** | schema file, generated client: `prisma.ticket.findMany({ where: { status: "open" }, include: { user: true } })` |
| TypeORM, Sequelize, MikroORM | classic ORMs with entity classes |

ORMs speed up common CRUD and give types; raw SQL gives full control for reports and performance. Most teams use both. Whatever you use, learn to read the SQL it generates and use `EXPLAIN` on slow queries (see the SQL track).

## Other stores

- **Redis**: in-memory key-value store for caches, sessions, rate limits, queues (BullMQ).
- **MongoDB**: document database; flexible JSON-like documents.
- **Search**: OpenSearch/Elasticsearch, Meilisearch for full-text search.

@@quiz
Why does a server use a connection pool?
* Opening connections is slow and limited, so a pool reuses a few open ones
- Pools encrypt queries
- Databases require it
- To run queries in order
? Always release a borrowed client, typically in `finally`.

@@quiz
A page loads 100 tickets, then runs one query per ticket for its user. What's this called and how is it fixed?
- Deadlock; add an index
* The N+1 problem; use a JOIN or one batched query for all users
- SQL injection; use parameters
- A race condition; use a transaction
? Each extra round trip adds latency.

@@quiz
Can a column name in ORDER BY be passed as a query parameter?
- Yes, like any value
* No; parameters are only for values, so validate identifiers against an allow-list
- Only in SQLite
- Only with an ORM
? Building SQL from unchecked identifiers is still injection.

@@flashcards
`node:sqlite` :: built-in synchronous SQLite driver
Prepared statement :: compiled SQL with placeholders, run with values
Connection pool :: reusable set of open DB connections
Transaction :: BEGIN … COMMIT/ROLLBACK as one unit
N+1 problem :: one query per row instead of one for all
Migration :: versioned schema change script
Query builder :: builds SQL with method calls (Knex, Kysely)
ORM :: maps tables to objects (Prisma, TypeORM)
Redis :: in-memory store for caches, sessions and queues

@@exercise A safe query builder
Write `buildSelect({ table, columns, where = {}, orderBy, limit })` returning `{ sql, params }` for PostgreSQL:

- `table`, `columns` and `orderBy` are **identifiers**: each must be in the `ALLOWED` map (table → its allowed columns), else throw `Error("not allowed: <name>")`
- `where` is an object of column → value; each becomes `"col" = $n` joined with ` AND `; a value that is an array becomes `"col" = ANY($n)`; a `null` value becomes `"col" IS NULL` (no parameter)
- `orderBy` like `"created"` or `"-created"` (descending)
- `limit`: an integer 1–1000, passed as a parameter
- identifiers are double-quoted
@@starter
const ALLOWED = {
  tickets: ["id", "title", "status", "team", "created", "assignee"],
  users: ["id", "name", "email"],
};

function buildSelect({ table, columns, where = {}, orderBy, limit }) {
  throw new Error("TODO: not done yet");
}

@@tests
check(buildSelect({ table: "tickets", columns: ["id", "title"], where: { status: "open", team: ["net", "desk"], assignee: null }, orderBy: "-created", limit: 50 }), {
  sql: 'SELECT "id", "title" FROM "tickets" WHERE "status" = $1 AND "team" = ANY($2) AND "assignee" IS NULL ORDER BY "created" DESC LIMIT $3',
  params: ["open", ["net", "desk"], 50],
}, "full query");
check(buildSelect({ table: "users", columns: ["name"] }), { sql: 'SELECT "name" FROM "users"', params: [] }, "minimal");
const attempts = [
  { table: "users; DROP TABLE users", columns: ["id"] },
  { table: "users", columns: ["password"] },
  { table: "users", columns: ["id"], orderBy: "name; --" },
  { table: "users", columns: ["id"], where: { "1=1 OR id": 1 } },
];
check(attempts.map((a) => { try { buildSelect(a); return "built"; } catch (e) { return e.message.split(":")[0]; } }), ["not allowed", "not allowed", "not allowed", "not allowed"], "injection attempts refused");
let msg = "";
try { buildSelect({ table: "users", columns: ["id"], limit: 5000 }); } catch (e) { msg = e.message; }
check(msg, "invalid limit", "limit range");
@@hint
Write `col(name)` that checks `ALLOWED[table].includes(name)` and returns `"${name}"` in double quotes. Keep `params = []` and use `params.push(value)` to get each placeholder number: `$${params.length}`.
@@solution
const ALLOWED = {
  tickets: ["id", "title", "status", "team", "created", "assignee"],
  users: ["id", "name", "email"],
};

function buildSelect({ table, columns, where = {}, orderBy, limit }) {
  if (!Object.hasOwn(ALLOWED, table)) throw new Error(`not allowed: ${table}`);
  const col = (name) => {
    if (!ALLOWED[table].includes(name)) throw new Error(`not allowed: ${name}`);
    return `"${name}"`;
  };
  const params = [];
  let sql = `SELECT ${columns.map(col).join(", ")} FROM "${table}"`;
  const conditions = Object.entries(where).map(([name, value]) => {
    if (value === null) return `${col(name)} IS NULL`;
    params.push(value);
    return Array.isArray(value) ? `${col(name)} = ANY($${params.length})` : `${col(name)} = $${params.length}`;
  });
  if (conditions.length > 0) sql += ` WHERE ${conditions.join(" AND ")}`;
  if (orderBy) {
    const desc = orderBy.startsWith("-");
    sql += ` ORDER BY ${col(desc ? orderBy.slice(1) : orderBy)}${desc ? " DESC" : ""}`;
  }
  if (limit !== undefined) {
    if (!Number.isInteger(limit) || limit < 1 || limit > 1000) throw new Error("invalid limit");
    params.push(limit);
    sql += ` LIMIT $${params.length}`;
  }
  return { sql, params };
}

@@exercise A migration runner
Write `async function migrate(db, migrations)` where `db` has `async query(sql)` and `async applied()` (returns the list of migration names already applied), and `migrations` is a list of `{ name, up }` (`up` is SQL).

- refuse to run if names aren't strictly increasing (`"001_x" < "002_y"` as strings): throw `Error("migrations out of order")`
- refuse if an applied name isn't in the list (someone deleted or renamed a migration): throw `Error("unknown applied migration <name>")`
- for each migration not yet applied, in order: run `BEGIN`, the `up` SQL, `INSERT INTO schema_migrations (name) VALUES ('<name>')`, then `COMMIT`; if any query fails, run `ROLLBACK` and throw `Error("migration <name> failed: <original message>")` with the original error as `cause`
- return the names that were applied in this run
@@starter
async function migrate(db, migrations) {
  throw new Error("TODO: not done yet");
}

@@tests
function fakeDb(applied, failOn) {
  const log = [];
  return {
    log,
    async applied() { return [...applied]; },
    async query(sql) { if (failOn && sql.includes(failOn)) throw new Error("syntax error"); log.push(sql); },
  };
}
const migrations = [
  { name: "001_tickets", up: "CREATE TABLE tickets (id int)" },
  { name: "002_priority", up: "ALTER TABLE tickets ADD priority int" },
  { name: "003_index", up: "CREATE INDEX ix ON tickets (priority)" },
];
(async () => {
  const db = fakeDb(["001_tickets"]);
  check(await migrate(db, migrations), ["002_priority", "003_index"], "applies the rest");
  check(db.log.slice(0, 4), ["BEGIN", "ALTER TABLE tickets ADD priority int", "INSERT INTO schema_migrations (name) VALUES ('002_priority')", "COMMIT"], "transaction per migration");
  const bad = fakeDb([], "INDEX");
  let err;
  try { await migrate(bad, migrations); } catch (e) { err = e; }
  check([err.message, err.cause.message, bad.log.at(-1)], ["migration 003_index failed: syntax error", "syntax error", "ROLLBACK"], "rollback on failure");
  for (const [applied, list] of [[[], [migrations[1], migrations[0]]], [["000_gone"], migrations]]) {
    try { await migrate(fakeDb(applied), list); err = null; } catch (e) { err = e; }
    check(err && err.message.split(" ")[0] !== "", true, "refuses bad state");
  }
})();
@@hint
Check the order with `migrations.every((m, i) => i === 0 || migrations[i - 1].name < m.name)`. Make a `Set` of names to check the applied list. In the loop, wrap the four queries in `try`, and in `catch` await `db.query("ROLLBACK")` then throw the new error with `{ cause: e }`.
@@solution
async function migrate(db, migrations) {
  if (!migrations.every((m, i) => i === 0 || migrations[i - 1].name < m.name)) {
    throw new Error("migrations out of order");
  }
  const names = new Set(migrations.map((m) => m.name));
  const applied = new Set(await db.applied());
  for (const name of applied) {
    if (!names.has(name)) throw new Error(`unknown applied migration ${name}`);
  }
  const ran = [];
  for (const m of migrations) {
    if (applied.has(m.name)) continue;
    try {
      await db.query("BEGIN");
      await db.query(m.up);
      await db.query(`INSERT INTO schema_migrations (name) VALUES ('${m.name}')`);
      await db.query("COMMIT");
    } catch (e) {
      await db.query("ROLLBACK");
      throw new Error(`migration ${m.name} failed: ${e.message}`, { cause: e });
    }
    ran.push(m.name);
  }
  return ran;
}

@@lesson p7-database Databases with database/sql: Queries, Transactions and Migrations
@@concepts modules, errors
@@text
`database/sql` is Go's standard interface to SQL databases. You import a **driver** for your database and use the same API everywhere:

| Database | Common driver |
|---|---|
| PostgreSQL | `github.com/jackc/pgx/v5/stdlib` |
| SQLite | `modernc.org/sqlite` (pure Go) or `github.com/mattn/go-sqlite3` (cgo) |
| MySQL / MariaDB | `github.com/go-sql-driver/mysql` |
| SQL Server | `github.com/microsoft/go-mssqldb` |

The standard library contains no drivers, so this lesson's code blocks are for reading; the exercises practise the logic around them.

## Opening and querying

```go,norun
import (
	"database/sql"
	_ "modernc.org/sqlite" // registers the "sqlite" driver
)

db, err := sql.Open("sqlite", "inventory.db")
if err != nil { return err }
defer db.Close()
if err := db.PingContext(ctx); err != nil { return err } // Open doesn't connect yet

rows, err := db.QueryContext(ctx, "SELECT name, port FROM hosts WHERE env = ?", env)
if err != nil { return err }
defer rows.Close() // releases the connection
for rows.Next() {
	var h Host
	if err := rows.Scan(&h.Name, &h.Port); err != nil { return err }
	hosts = append(hosts, h)
}
return rows.Err() // errors during iteration surface here
```

- `sql.DB` is a **connection pool**, safe for concurrent use. Create it once and share it; tune it with `SetMaxOpenConns`, `SetMaxIdleConns` and `SetConnMaxLifetime`.
- `QueryRowContext(...).Scan(...)` reads a single row and returns `sql.ErrNoRows` when there's none: check it with `errors.Is`.
- `ExecContext` for INSERT/UPDATE/DELETE; `res.RowsAffected()`, `res.LastInsertId()`.
- Nullable columns: `sql.NullString`, `sql.Null[T]` (Go 1.22), or pointers.

## Placeholders, never string concatenation

```go,norun
// NEVER: SQL injection
db.Query("SELECT * FROM users WHERE name = '" + name + "'")
// ALWAYS: the driver sends the value separately
db.QueryContext(ctx, "SELECT * FROM users WHERE name = $1", name) // Postgres style; SQLite/MySQL use ?
```

Placeholders work only for **values**. Table names, column names and `ORDER BY` directions can't be parameters; if they come from user input, check them against an **allowlist**.

## Transactions

```go,norun
tx, err := db.BeginTx(ctx, nil)
if err != nil { return err }
defer tx.Rollback() // no-op after a successful Commit

if _, err := tx.ExecContext(ctx, "UPDATE accounts SET balance = balance - $1 WHERE id = $2", amt, from); err != nil {
	return err
}
if _, err := tx.ExecContext(ctx, "UPDATE accounts SET balance = balance + $1 WHERE id = $2", amt, to); err != nil {
	return err
}
return tx.Commit()
```

## Migrations

Schema changes are versioned SQL files (`0001_create_hosts.sql`, `0002_add_owner.sql`) applied in order and recorded in a table (`schema_migrations`). Tools: `golang-migrate`, `goose`, `atlas`. Embed the files in your binary with `//go:embed migrations/*.sql`.

## Beyond raw SQL

- `sqlc` generates type-safe Go code from SQL queries: you write SQL, it writes the Scan boilerplate.
- `sqlx` adds struct scanning to database/sql.
- ORMs like GORM and ent exist, but many Go teams prefer SQL-first tools.
- Keep data access behind a small interface (`HostStore`) so business logic can be tested with an in-memory fake.

@@quiz
What is `sql.DB`?
- One database connection
* A concurrency-safe connection pool you create once and share
- A transaction
- A driver
? Opening one per request is a classic mistake.

@@quiz
Why must you check `rows.Err()` after the loop?
* Errors during iteration (like a dropped connection) are only reported there
- It closes the rows
- It's optional style
- It returns the row count
? rows.Next() returns false on both "done" and "error".

@@quiz
Can a table name be passed as a placeholder (`SELECT * FROM $1`)?
- Yes, always
* No; placeholders are for values. Validate identifiers against an allowlist
- Only in SQLite
- Only in transactions
? The database must know the structure of the query before binding values.

@@flashcards
`sql.Open` :: create a pool (doesn't connect)
`PingContext` :: check the connection
`QueryContext` :: many rows
`QueryRowContext` :: one row
`sql.ErrNoRows` :: QueryRow found nothing
`rows.Close()` :: return the connection
`ExecContext` :: statements without rows
`BeginTx` / `Commit` / `Rollback` :: transactions
Placeholder :: `?` or `$1`, for values only
sqlc :: generate Go from SQL

@@exercise A safe query builder
Write a small builder for listing hosts with optional filters:

`Build(q Query) (string, []any, error)` where `Query{Filters map[string]any; OrderBy string; Desc bool; Limit int}`.

- only the columns `name`, `env`, `owner`, `port` are allowed (for filters and `OrderBy`); otherwise return `unknown column "<c>"`
- filters are applied in **sorted column order**; a `[]string` value becomes `col IN ($1, $2)`, anything else `col = $n`
- placeholders are numbered `$1, $2...` in the order arguments are added
- `ORDER BY` defaults to `name`; `Desc` appends `DESC`; `Limit > 0` adds `LIMIT $n`
- the result starts with `SELECT name, env, owner, port FROM hosts`
@@starter
package main

import (
	"fmt"
	"maps"
	"slices"
	"strings"
)

type Query struct {
	Filters map[string]any
	OrderBy string
	Desc    bool
	Limit   int
}

var allowed = map[string]bool{"name": true, "env": true, "owner": true, "port": true}

func Build(q Query) (string, []any, error) {
	_ = maps.Keys(allowed)
	_ = slices.Sorted[string]
	_ = strings.Join
	return "", nil, nil
}

func main() {
	queries := []Query{
		{},
		{Filters: map[string]any{"env": "prod", "port": 443}, Limit: 10},
		{Filters: map[string]any{"owner": []string{"ana", "kenji"}}, OrderBy: "port", Desc: true},
		{Filters: map[string]any{"name; DROP TABLE hosts": "x"}},
		{OrderBy: "random()"},
	}
	for _, q := range queries {
		sql, args, err := Build(q)
		if err != nil {
			fmt.Println("error:", err)
			continue
		}
		fmt.Println(sql)
		fmt.Println("  args:", args)
	}
}
@@expected
SELECT name, env, owner, port FROM hosts ORDER BY name
  args: []
SELECT name, env, owner, port FROM hosts WHERE env = $1 AND port = $2 ORDER BY name LIMIT $3
  args: [prod 443 10]
SELECT name, env, owner, port FROM hosts WHERE owner IN ($1, $2) ORDER BY port DESC
  args: [ana kenji]
error: unknown column "name; DROP TABLE hosts"
error: unknown column "random()"
@@hint
Keep `args := []any{}` and a helper `next := func(v any) string { args = append(args, v); return fmt.Sprintf("$%d", len(args)) }`. Loop over `slices.Sorted(maps.Keys(q.Filters))`; a type switch on the value handles `[]string`. Collect conditions and join them with `" AND "`.
@@solution
package main

import (
	"fmt"
	"maps"
	"slices"
	"strings"
)

type Query struct {
	Filters map[string]any
	OrderBy string
	Desc    bool
	Limit   int
}

var allowed = map[string]bool{"name": true, "env": true, "owner": true, "port": true}

func Build(q Query) (string, []any, error) {
	args := []any{}
	next := func(v any) string {
		args = append(args, v)
		return fmt.Sprintf("$%d", len(args))
	}
	var b strings.Builder
	b.WriteString("SELECT name, env, owner, port FROM hosts")
	var conds []string
	for _, col := range slices.Sorted(maps.Keys(q.Filters)) {
		if !allowed[col] {
			return "", nil, fmt.Errorf("unknown column %q", col)
		}
		switch v := q.Filters[col].(type) {
		case []string:
			var ph []string
			for _, s := range v {
				ph = append(ph, next(s))
			}
			conds = append(conds, col+" IN ("+strings.Join(ph, ", ")+")")
		default:
			conds = append(conds, col+" = "+next(v))
		}
	}
	if len(conds) > 0 {
		b.WriteString(" WHERE " + strings.Join(conds, " AND "))
	}
	order := q.OrderBy
	if order == "" {
		order = "name"
	}
	if !allowed[order] {
		return "", nil, fmt.Errorf("unknown column %q", order)
	}
	b.WriteString(" ORDER BY " + order)
	if q.Desc {
		b.WriteString(" DESC")
	}
	if q.Limit > 0 {
		b.WriteString(" LIMIT " + next(q.Limit))
	}
	return b.String(), args, nil
}

func main() {
	queries := []Query{
		{},
		{Filters: map[string]any{"env": "prod", "port": 443}, Limit: 10},
		{Filters: map[string]any{"owner": []string{"ana", "kenji"}}, OrderBy: "port", Desc: true},
		{Filters: map[string]any{"name; DROP TABLE hosts": "x"}},
		{OrderBy: "random()"},
	}
	for _, q := range queries {
		sql, args, err := Build(q)
		if err != nil {
			fmt.Println("error:", err)
			continue
		}
		fmt.Println(sql)
		fmt.Println("  args:", args)
	}
}

@@exercise A migrations runner
Write `Migrate(ctx context.Context, db Execer, all []Migration, applied map[int]bool) ([]int, error)`:

- `Migration{Version int; Name, SQL string}`; `Execer` is an interface with `Exec(ctx context.Context, query string, args ...any) error` (like `*sql.Tx`)
- validate first: duplicate versions → `duplicate migration version <v>`
- apply pending migrations in **ascending version order**: run `m.SQL`, then `INSERT INTO schema_migrations (version) VALUES ($1)` with the version
- stop at the first failure and return the versions applied so far plus `migration <v> (<name>): <err>` (wrapped with `%w`)

The provided `FakeDB` records statements and fails on any SQL containing `FAIL`.
@@starter
package main

import (
	"context"
	"errors"
	"fmt"
	"slices"
	"strings"
)

type Migration struct {
	Version   int
	Name, SQL string
}

type Execer interface {
	Exec(ctx context.Context, query string, args ...any) error
}

type FakeDB struct{ Log []string }

var ErrSyntax = errors.New("syntax error")

func (f *FakeDB) Exec(ctx context.Context, query string, args ...any) error {
	if strings.Contains(query, "FAIL") {
		return ErrSyntax
	}
	f.Log = append(f.Log, fmt.Sprint(query, " ", args))
	return nil
}

func Migrate(ctx context.Context, db Execer, all []Migration, applied map[int]bool) ([]int, error) {
	_ = slices.SortFunc[[]Migration]
	return nil, nil
}

func main() {
	ctx := context.Background()
	all := []Migration{
		{3, "add_owner", "ALTER TABLE hosts ADD owner TEXT"},
		{1, "create_hosts", "CREATE TABLE hosts (name TEXT)"},
		{2, "index_name", "CREATE INDEX hosts_name ON hosts(name)"},
		{4, "broken", "ALTER TABLE FAIL"},
		{5, "never", "SELECT 1"},
	}
	db := &FakeDB{}
	done, err := Migrate(ctx, db, all, map[int]bool{1: true})
	for _, l := range db.Log {
		fmt.Println(l)
	}
	fmt.Println("applied:", done)
	fmt.Println("error:", err, errors.Is(err, ErrSyntax))
	_, err = Migrate(ctx, &FakeDB{}, append(all, Migration{2, "again", ""}), nil)
	fmt.Println("error:", err)
}
@@expected
CREATE INDEX hosts_name ON hosts(name) []
INSERT INTO schema_migrations (version) VALUES ($1) [2]
ALTER TABLE hosts ADD owner TEXT []
INSERT INTO schema_migrations (version) VALUES ($1) [3]
applied: [2 3]
error: migration 4 (broken): syntax error true
error: duplicate migration version 2
@@requires
%w
@@hint
Copy the slice before sorting (`slices.Clone`) so you don't reorder the caller's data. Check for duplicates with a `seen` map. In the loop, skip `applied[m.Version]`.
@@solution
package main

import (
	"context"
	"errors"
	"fmt"
	"slices"
	"strings"
)

type Migration struct {
	Version   int
	Name, SQL string
}

type Execer interface {
	Exec(ctx context.Context, query string, args ...any) error
}

type FakeDB struct{ Log []string }

var ErrSyntax = errors.New("syntax error")

func (f *FakeDB) Exec(ctx context.Context, query string, args ...any) error {
	if strings.Contains(query, "FAIL") {
		return ErrSyntax
	}
	f.Log = append(f.Log, fmt.Sprint(query, " ", args))
	return nil
}

func Migrate(ctx context.Context, db Execer, all []Migration, applied map[int]bool) ([]int, error) {
	seen := map[int]bool{}
	for _, m := range all {
		if seen[m.Version] {
			return nil, fmt.Errorf("duplicate migration version %d", m.Version)
		}
		seen[m.Version] = true
	}
	sorted := slices.Clone(all)
	slices.SortFunc(sorted, func(a, b Migration) int { return a.Version - b.Version })
	var done []int
	for _, m := range sorted {
		if applied[m.Version] {
			continue
		}
		if err := db.Exec(ctx, m.SQL); err != nil {
			return done, fmt.Errorf("migration %d (%s): %w", m.Version, m.Name, err)
		}
		if err := db.Exec(ctx, "INSERT INTO schema_migrations (version) VALUES ($1)", m.Version); err != nil {
			return done, fmt.Errorf("migration %d (%s): %w", m.Version, m.Name, err)
		}
		done = append(done, m.Version)
	}
	return done, nil
}

func main() {
	ctx := context.Background()
	all := []Migration{
		{3, "add_owner", "ALTER TABLE hosts ADD owner TEXT"},
		{1, "create_hosts", "CREATE TABLE hosts (name TEXT)"},
		{2, "index_name", "CREATE INDEX hosts_name ON hosts(name)"},
		{4, "broken", "ALTER TABLE FAIL"},
		{5, "never", "SELECT 1"},
	}
	db := &FakeDB{}
	done, err := Migrate(ctx, db, all, map[int]bool{1: true})
	for _, l := range db.Log {
		fmt.Println(l)
	}
	fmt.Println("applied:", done)
	fmt.Println("error:", err, errors.Is(err, ErrSyntax))
	_, err = Migrate(ctx, &FakeDB{}, append(all, Migration{2, "again", ""}), nil)
	fmt.Println("error:", err)
}

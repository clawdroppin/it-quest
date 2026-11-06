@@lesson g5-structure Structuring a Go Application: Packages, Interfaces, Errors and Databases
@@concepts modules, errors, objects
@@text
A 100-line `main.go` is fine. A service that a team works on for years needs structure: where code lives, how parts depend on each other, how errors travel, and how the database fits in. Go has strong conventions for all of these.

## Project layout

```text
deviceapi/
├── go.mod                      module example.com/deviceapi
├── cmd/
│   ├── deviceapi/main.go       the server: reads config, wires everything together, starts
│   └── devicectl/main.go       a second program (a CLI) sharing the same packages
├── internal/                   packages only this module can import
│   ├── device/                 the domain: types, validation, rules (no HTTP, no SQL)
│   ├── store/                  database access (postgres.go, memory.go)
│   └── httpapi/                handlers, routes, middleware
└── migrations/                 001_init.sql, 002_add_owner.sql
```

Keep it as flat as you can while it's small: one package is fine for a few thousand lines. Split when a part has a clear job of its own. The `internal/` folder is enforced by the compiler: other modules can't import it, so you can change it freely.

## Interfaces are defined where they're used

In Go, a type satisfies an interface just by having the methods; it never says `implements`. So the convention is the opposite of Java and C#: the **consumer** defines a small interface with only what it needs.

```go,norun
// In package httpapi: what the handlers need, nothing more.
type DeviceStore interface {
	Get(ctx context.Context, id int) (device.Device, error)
	List(ctx context.Context, f device.Filter) ([]device.Device, error)
}

// store.Postgres satisfies it without knowing it exists; so does a fake in the tests.
api := httpapi.New(store.NewPostgres(db), logger)
```

"Accept interfaces, return structs": functions take the smallest interface they need, and constructors return concrete types.

## Dependency injection without a framework

`main` builds everything and passes it in. No global variables, no magic:

```go,norun
func main() {
	cfg := loadConfig()                          // from environment variables and flags
	db, err := sql.Open("pgx", cfg.DatabaseURL)
	if err != nil { ... }
	logger := slog.New(slog.NewJSONHandler(os.Stdout, nil))
	devices := store.NewPostgres(db)
	api := httpapi.New(devices, logger)          // the API gets what it needs, as parameters
	...
}
```

## Errors: wrap them, then inspect them

Go errors are values. As an error travels up, each layer **adds context** with `%w`, which keeps the original inside:

```go
package main

import (
	"errors"
	"fmt"
	"io/fs"
	"os"
)

var ErrNotFound = errors.New("not found")          // a sentinel error, compared with errors.Is

func findDevice(id int) error {
	return fmt.Errorf("device %d: %w", id, ErrNotFound)
}

func loadConfig(path string) error {
	_, err := os.ReadFile(path)
	if err != nil {
		return fmt.Errorf("loading config: %w", err)
	}
	return nil
}

func main() {
	err := fmt.Errorf("GET /devices/7: %w", findDevice(7))
	fmt.Println(err)
	fmt.Println(errors.Is(err, ErrNotFound))                 // looks through every layer

	err = loadConfig("/no/such/file.json")
	var pathErr *fs.PathError
	fmt.Println(errors.Is(err, fs.ErrNotExist), errors.As(err, &pathErr))
	if errors.As(err, &pathErr) {                            // get the typed error out
		fmt.Println("operation:", pathErr.Op)
	}
}
```

| Tool | Use |
|---|---|
| `errors.New("...")` | a sentinel error value (`ErrNotFound`) |
| `fmt.Errorf("context: %w", err)` | add context, keep the original |
| `errors.Is(err, ErrNotFound)` | "is it (or does it wrap) this value?" |
| `errors.As(err, &target)` | "is there an error of this type inside? give it to me" |
| a custom type with `Error() string` | errors that carry data (a list of validation problems) |

The HTTP layer is the one place that turns errors into status codes: `ErrNotFound` → 404, a validation error → 422, anything unknown → 500 (and logged, with the details kept out of the response).

## Databases with database/sql

```go,norun
import (
	"database/sql"
	_ "github.com/jackc/pgx/v5/stdlib"     // the driver registers itself; also modernc.org/sqlite, go-sql-driver/mysql
)

db, err := sql.Open("pgx", os.Getenv("DATABASE_URL"))   // a connection POOL, not one connection
db.SetMaxOpenConns(20)

func (s *Postgres) Get(ctx context.Context, id int) (device.Device, error) {
	var d device.Device
	err := s.db.QueryRowContext(ctx,
		`SELECT id, hostname, kind FROM devices WHERE id = $1`, id,   // always placeholders
	).Scan(&d.ID, &d.Hostname, &d.Kind)
	if errors.Is(err, sql.ErrNoRows) {
		return device.Device{}, fmt.Errorf("device %d: %w", id, device.ErrNotFound)
	}
	return d, err
}

// Several rows: always close them, always check rows.Err()
rows, err := s.db.QueryContext(ctx, `SELECT id, hostname FROM devices ORDER BY hostname`)
if err != nil { return nil, err }
defer rows.Close()
for rows.Next() { ... rows.Scan(...) ... }
return list, rows.Err()

// Transactions
tx, err := s.db.BeginTx(ctx, nil)
if err != nil { return err }
defer tx.Rollback()                       // does nothing after a successful Commit
... tx.ExecContext(ctx, ...) ...
return tx.Commit()
```

Popular helpers on top: **sqlc** generates type-safe Go code from your SQL queries; **sqlx** adds scanning into structs; **pgx** is the fastest PostgreSQL driver; **goose** and **golang-migrate** run migrations. Go developers generally prefer writing SQL to using a heavy ORM (GORM exists, but is less common in services).

## Configuration

The twelve-factor convention, used by every container platform: **configuration comes from environment variables**, with sensible defaults for development. Never commit secrets; read them from the environment or a secrets manager.

```go,norun
type Config struct {
	Addr        string
	DatabaseURL string
	LogLevel    slog.Level
}

func loadConfig() (Config, error) {
	cfg := Config{Addr: envOr("ADDR", ":8080"), DatabaseURL: os.Getenv("DATABASE_URL")}
	if cfg.DatabaseURL == "" {
		return cfg, errors.New("DATABASE_URL is required")      // fail at startup, not on the first request
	}
	return cfg, nil
}
```

@@quiz
Where does Go convention put a small interface like `DeviceStore`?
- In the package that implements it
* In the package that uses it (the consumer), with only the methods it needs
- In a shared `interfaces` package
- Go has no interfaces
? Types satisfy interfaces implicitly, so the consumer can describe exactly what it needs.

@@quiz
`err := fmt.Errorf("loading: %w", ErrNotFound)`. What does `errors.Is(err, ErrNotFound)` return?
* true: `%w` wraps the original, and errors.Is looks through the wrapping
- false: the message is different
- It doesn't compile
- true only if the messages match
? With `%v` instead of `%w`, the original would be lost and it would be false.

@@quiz
What does `sql.Open` return?
- One open connection
* A pool that opens and reuses connections as needed (it may not even connect yet)
- A transaction
- A query result
? Create it once at startup and share it. Use db.PingContext to check the connection works.

@@quiz
What's wrong with this? `db.Query("SELECT * FROM devices WHERE owner = '" + owner + "'")`
- Nothing
- It should use QueryRow
* SQL injection: values must be passed as placeholder parameters ($1 or ?)
- Strings can't be concatenated in Go
? `db.QueryContext(ctx, "... WHERE owner = $1", owner)` sends the value separately from the SQL.

@@drill
By convention, which top-level folder holds packages other modules can't import?
= internal
= internal/
? The compiler enforces it.

@@flashcards
`cmd/<name>/main.go` :: one folder per program in a module
`internal/` :: packages that only this module can import
accept interfaces, return structs :: small interfaces as parameters, concrete types from constructors
`fmt.Errorf("...: %w", err)` :: wrap an error with context
`errors.Is(err, target)` :: does err match or wrap target?
`errors.As(err, &typed)` :: find an error of a type inside err
sentinel error :: `var ErrNotFound = errors.New("not found")`
`database/sql` :: the standard database interface; drivers plug in
`sql.ErrNoRows` :: QueryRow found nothing
`defer rows.Close()` + `rows.Err()` :: always, when looping over rows
`defer tx.Rollback()` :: safe: does nothing after Commit
sqlc :: generates type-safe Go from SQL queries
twelve-factor config :: settings from environment variables

@@exercise Errors to status codes
Write `statusFor(err error) int`: `nil` → 200, anything that wraps `ErrNotFound` → 404, anything that wraps `ErrConflict` → 409, a `*ValidationError` anywhere inside → 422, everything else → 500. Use `errors.Is` and `errors.As`.
@@starter
package main

import (
	"errors"
	"fmt"
)

var (
	ErrNotFound = errors.New("not found")
	ErrConflict = errors.New("conflict")
)

type ValidationError struct{ Field string }

func (e *ValidationError) Error() string { return "invalid " + e.Field }

func statusFor(err error) int {
	// TODO
	return 0
}

func main() {
	errs := []error{
		nil,
		ErrNotFound,
		fmt.Errorf("device 7: %w", ErrNotFound),
		fmt.Errorf("handler: %w", fmt.Errorf("store: %w", ErrConflict)),
		&ValidationError{Field: "ip"},
		fmt.Errorf("create: %w", &ValidationError{Field: "hostname"}),
		errors.New("disk full"),
		fmt.Errorf("device 7: %v", ErrNotFound),
	}
	for _, err := range errs {
		fmt.Println(statusFor(err), err)
	}
}
@@expected
200 <nil>
404 not found
404 device 7: not found
409 handler: store: conflict
422 invalid ip
422 create: invalid hostname
500 disk full
500 device 7: not found
@@requires
errors.Is
errors.As
@@hint
`var verr *ValidationError; switch { case err == nil: return 200; case errors.As(err, &verr): return 422; case errors.Is(err, ErrNotFound): return 404; case errors.Is(err, ErrConflict): return 409; default: return 500 }`. The last error used `%v`, which doesn't wrap, so it's a 500.
@@solution
package main

import (
	"errors"
	"fmt"
)

var (
	ErrNotFound = errors.New("not found")
	ErrConflict = errors.New("conflict")
)

type ValidationError struct{ Field string }

func (e *ValidationError) Error() string { return "invalid " + e.Field }

func statusFor(err error) int {
	var verr *ValidationError
	switch {
	case err == nil:
		return 200
	case errors.As(err, &verr):
		return 422
	case errors.Is(err, ErrNotFound):
		return 404
	case errors.Is(err, ErrConflict):
		return 409
	default:
		return 500
	}
}

func main() {
	errs := []error{
		nil,
		ErrNotFound,
		fmt.Errorf("device 7: %w", ErrNotFound),
		fmt.Errorf("handler: %w", fmt.Errorf("store: %w", ErrConflict)),
		&ValidationError{Field: "ip"},
		fmt.Errorf("create: %w", &ValidationError{Field: "hostname"}),
		errors.New("disk full"),
		fmt.Errorf("device 7: %v", ErrNotFound),
	}
	for _, err := range errs {
		fmt.Println(statusFor(err), err)
	}
}

@@exercise A consumer-side interface and a fake
`CheckDisks` needs something that can send alerts. Define the interface `Notifier` with one method `Notify(msg string) error`, write `CheckDisks(usage map[string]int, n Notifier) int` that calls `Notify("<disk> is <pct>% full")` for every disk at **90% or more**, in **sorted disk order**, and returns how many alerts were sent. Then write `fakeNotifier`, which records the messages in a slice, so `main` can check what was sent.
@@starter
package main

import (
	"fmt"
	"sort"
)

// TODO: Notifier, CheckDisks, fakeNotifier

func main() {
	usage := map[string]int{"/var": 97, "/": 42, "/home": 90, "/data": 89}
	fake := &fakeNotifier{}
	n := CheckDisks(usage, fake)
	fmt.Println(n, "alerts")
	for _, m := range fake.sent {
		fmt.Println(m)
	}
	_ = sort.Strings
}
@@expected
2 alerts
/home is 90% full
/var is 97% full
@@requires
type Notifier interface
@@hint
`type Notifier interface { Notify(msg string) error }`. In CheckDisks, collect the keys, `sort.Strings(keys)`, loop and call `n.Notify(fmt.Sprintf("%s is %d%% full", k, usage[k]))` (`%%` prints a percent sign). `type fakeNotifier struct{ sent []string }` with `func (f *fakeNotifier) Notify(msg string) error { f.sent = append(f.sent, msg); return nil }`.
@@solution
package main

import (
	"fmt"
	"sort"
)

type Notifier interface {
	Notify(msg string) error
}

func CheckDisks(usage map[string]int, n Notifier) int {
	keys := make([]string, 0, len(usage))
	for k := range usage {
		keys = append(keys, k)
	}
	sort.Strings(keys)
	sent := 0
	for _, k := range keys {
		if usage[k] >= 90 {
			if err := n.Notify(fmt.Sprintf("%s is %d%% full", k, usage[k])); err == nil {
				sent++
			}
		}
	}
	return sent
}

type fakeNotifier struct{ sent []string }

func (f *fakeNotifier) Notify(msg string) error {
	f.sent = append(f.sent, msg)
	return nil
}

func main() {
	usage := map[string]int{"/var": 97, "/": 42, "/home": 90, "/data": 89}
	fake := &fakeNotifier{}
	n := CheckDisks(usage, fake)
	fmt.Println(n, "alerts")
	for _, m := range fake.sent {
		fmt.Println(m)
	}
	_ = sort.Strings
}

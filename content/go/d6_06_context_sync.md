@@lesson d6-context-sync context, Mutexes, Once, Atomics and the Race Detector
@@concepts concurrency
@@text
Channels are for passing data between goroutines. For **shared state** and **cancellation**, Go has the `sync`, `sync/atomic` and `context` packages.

## context: cancellation and deadlines

A `context.Context` carries a cancellation signal and deadline through a call chain. Every function that does I/O or waits should take `ctx context.Context` as its **first parameter** and stop when `ctx.Done()` closes:

```go
package main

import (
	"context"
	"errors"
	"fmt"
	"time"
)

func slowQuery(ctx context.Context, d time.Duration) (string, error) {
	select {
	case <-time.After(d):
		return "42 rows", nil
	case <-ctx.Done():
		return "", ctx.Err() // context.Canceled or context.DeadlineExceeded
	}
}

func main() {
	ctx, cancel := context.WithTimeout(context.Background(), 50*time.Millisecond)
	defer cancel() // always release the context's resources
	_, err := slowQuery(ctx, time.Second)
	fmt.Println(err, errors.Is(err, context.DeadlineExceeded))
	r, err := slowQuery(context.Background(), 10*time.Millisecond)
	fmt.Println(r, err)
}
```

- `WithCancel`, `WithTimeout`, `WithDeadline`, `WithCancelCause` derive child contexts; cancelling a parent cancels all children.
- HTTP servers give each request a context that's cancelled when the client disconnects (`r.Context()`), and `http.NewRequestWithContext`, `database/sql`'s `QueryContext` and `exec.CommandContext` all honour it.
- `context.WithValue` carries **request-scoped** data (request id, auth user) across API boundaries; don't use it for optional parameters.

## Mutexes

```go
package main

import (
	"fmt"
	"sync"
)

type Counters struct {
	mu sync.Mutex
	m  map[string]int
}

func (c *Counters) Inc(key string) {
	c.mu.Lock()
	defer c.mu.Unlock()
	c.m[key]++
}

func main() {
	c := Counters{m: map[string]int{}}
	var wg sync.WaitGroup
	for i := range 100 {
		wg.Go(func() { c.Inc([]string{"get", "post"}[i%2]) })
	}
	wg.Wait()
	fmt.Println(c.m["get"], c.m["post"])
}
```

- Keep critical sections small; never call slow I/O while holding a lock.
- `sync.RWMutex` allows many readers or one writer: good for read-heavy caches.
- A struct containing a mutex must not be copied (pointer receivers; `go vet` warns).

## Once, OnceValue

`sync.Once` runs initialization exactly once, even with concurrent callers. `sync.OnceValue(f)` (Go 1.21) returns a function that computes and caches a value:

```go,norun
var loadConfig = sync.OnceValue(func() Config { return parse(os.ReadFile(...)) })
cfg := loadConfig() // first call parses; later calls return the cached value
```

## Atomics

`sync/atomic` types (`atomic.Int64`, `atomic.Bool`, `atomic.Pointer[T]`) give lock-free updates of single values: counters, flags, swapping a config pointer. For anything involving more than one value, use a mutex.

## The race detector

A **data race** is two goroutines accessing the same memory at the same time with at least one write. Races cause bizarre, rare bugs. Go ships a detector:

```text
go test -race ./...
go run -race .
```

Run your tests with `-race` in CI. It only finds races that actually happen during the run, so concurrent code needs tests that exercise it concurrently.

## Which tool?

| Situation | Use |
|---|---|
| hand work or results to another goroutine | channel |
| protect a map/struct shared by goroutines | `sync.Mutex` / `RWMutex` |
| one counter or flag | `atomic` |
| one-time lazy init | `sync.Once` / `OnceValue` |
| wait for N goroutines | `sync.WaitGroup` (`wg.Go`) |
| stop work on timeout or cancellation | `context` |

@@quiz
Where should `ctx context.Context` go in a function signature?
* First parameter, conventionally named ctx
- Last parameter
- In a struct field
- In a global variable
? `func Fetch(ctx context.Context, url string) error`.

@@quiz
Why `defer cancel()` after `context.WithTimeout`?
- It starts the timer
* It releases the context's resources even if the work finishes early
- It's required for compilation only
- It cancels the parent
? Forgetting it leaks timers until the timeout fires.

@@quiz
What does `go test -race` do?
- Runs tests in parallel faster
* Instruments memory accesses to detect data races that happen during the run
- Proves the code has no races
- Benchmarks goroutines
? It only finds races that actually occur in that run.

@@drill
Which flag runs Go tests with the race detector?
= -race
? go test -race ./...

@@flashcards
`context.Context` :: cancellation, deadline and request values
`ctx.Done()` :: channel closed on cancellation
`ctx.Err()` :: Canceled or DeadlineExceeded
`WithTimeout` / `WithCancel` :: derive cancellable contexts
`sync.Mutex` :: exclusive lock
`sync.RWMutex` :: many readers or one writer
`sync.OnceValue` :: compute once, cache the result
`atomic.Int64` :: lock-free counter
Data race :: concurrent access with at least one write
`-race` :: race detector flag

@@exercise First answer wins, with cancellation
Write `Query(ctx context.Context, servers []Server) (string, error)`:

- query every server concurrently with a **child context** that's cancelled as soon as one answer arrives (`defer cancel()`)
- `type Server struct { Name string; Delay time.Duration; Fail bool }`; a server waits `Delay` (or until its context is done, returning `ctx.Err()`), then returns `"<name>: ok"` or an error `<name>: failed`
- return the first successful answer; if all fail, return `errors.Join` of the failures; if the parent context ends first, return its error
- count how many servers noticed the cancellation (returned `context.Canceled`) with an `atomic.Int32` declared at package level, and wait for them all before returning so the count is final

Results: the channel must be buffered so late servers never block (no goroutine leaks).
@@starter
package main

import (
	"context"
	"errors"
	"fmt"
	"sync"
	"sync/atomic"
	"time"
)

type Server struct {
	Name  string
	Delay time.Duration
	Fail  bool
}

var cancelled atomic.Int32

func Query(ctx context.Context, servers []Server) (string, error) {
	_ = errors.Join
	_ = sync.WaitGroup{}
	return "", nil
}

func main() {
	ms := time.Millisecond
	r, err := Query(context.Background(), []Server{{"tokyo", 300 * ms, false}, {"osaka", 20 * ms, false}, {"sapporo", 400 * ms, false}})
	fmt.Println(r, err, "cancelled:", cancelled.Load())
	r, err = Query(context.Background(), []Server{{"a", 10 * ms, true}, {"b", 30 * ms, true}})
	fmt.Printf("%q\n%v\n", r, err)
	ctx, cancel := context.WithTimeout(context.Background(), 30*ms)
	defer cancel()
	_, err = Query(ctx, []Server{{"slow", time.Second, false}})
	fmt.Println(err)
}
@@expected
osaka: ok <nil> cancelled: 2
""
a: failed
b: failed
context deadline exceeded
@@requires
context.WithCancel
ctx.Done()
@@hint
Inside `Query`: `ctx, cancel := context.WithCancel(ctx); defer cancel()`. Each goroutine sends a `result{answer string; err error}` on `results := make(chan result, len(servers))`. Read `len(servers)` results; on the first success call `cancel()`, remember the answer, and keep reading (so you wait for everyone). Afterwards: answer found → return it; parent `ctx.Err() != nil` → return that; else `errors.Join(failures...)`. Collect failures in the order they arrive. A server function: `select { case <-time.After(s.Delay): ...; case <-ctx.Done(): if errors.Is(ctx.Err(), context.Canceled) { cancelled.Add(1) }; return ctx.Err() }`. Note: the parent's error check must look at the **parent** context, so keep a reference to it before shadowing.
@@solution
package main

import (
	"context"
	"errors"
	"fmt"
	"sync"
	"sync/atomic"
	"time"
)

type Server struct {
	Name  string
	Delay time.Duration
	Fail  bool
}

var cancelled atomic.Int32

type result struct {
	answer string
	err    error
}

func ask(ctx context.Context, s Server) (string, error) {
	select {
	case <-time.After(s.Delay):
		if s.Fail {
			return "", errors.New(s.Name + ": failed")
		}
		return s.Name + ": ok", nil
	case <-ctx.Done():
		if errors.Is(ctx.Err(), context.Canceled) {
			cancelled.Add(1)
		}
		return "", ctx.Err()
	}
}

func Query(parent context.Context, servers []Server) (string, error) {
	ctx, cancel := context.WithCancel(parent)
	defer cancel()
	results := make(chan result, len(servers))
	var wg sync.WaitGroup
	for _, s := range servers {
		wg.Add(1)
		go func() {
			defer wg.Done()
			a, err := ask(ctx, s)
			results <- result{a, err}
		}()
	}
	answer := ""
	var failures []error
	for range servers {
		r := <-results
		switch {
		case r.err == nil && answer == "":
			answer = r.answer
			cancel()
		case r.err != nil && !errors.Is(r.err, context.Canceled) && !errors.Is(r.err, context.DeadlineExceeded):
			failures = append(failures, r.err)
		}
	}
	wg.Wait()
	if answer != "" {
		return answer, nil
	}
	if err := parent.Err(); err != nil {
		return "", err
	}
	return "", errors.Join(failures...)
}

func main() {
	ms := time.Millisecond
	r, err := Query(context.Background(), []Server{{"tokyo", 300 * ms, false}, {"osaka", 20 * ms, false}, {"sapporo", 400 * ms, false}})
	fmt.Println(r, err, "cancelled:", cancelled.Load())
	r, err = Query(context.Background(), []Server{{"a", 10 * ms, true}, {"b", 30 * ms, true}})
	fmt.Printf("%q\n%v\n", r, err)
	ctx, cancel := context.WithTimeout(context.Background(), 30*ms)
	defer cancel()
	_, err = Query(ctx, []Server{{"slow", time.Second, false}})
	fmt.Println(err)
}

@@exercise A thread-safe metrics registry
Write `Registry` for an HTTP service:

- `Inc(name string, labels ...string)`: increments a counter keyed by the name plus `key=value` labels (labels sorted, e.g. `http_requests{method=GET,status=200}`); input labels come as pairs `"method", "GET", "status", "200"`
- `Snapshot() map[string]int64`: a **copy**
- `Render() string`: lines `<key> <value>` sorted by key (like Prometheus text format)
- protect the map with a `sync.RWMutex` (`Snapshot` and `Render` take the read lock)

Also make `loadLimits` a `sync.OnceValue` that counts how often it really ran (in an `atomic.Int32`).

`main` hammers the registry from 200 goroutines and prints the result. Run it with `-race` on your machine to check there are no data races.
@@starter
package main

import (
	"fmt"
	"maps"
	"slices"
	"strings"
	"sync"
	"sync/atomic"
)

type Registry struct {
	counters map[string]int64
}

func NewRegistry() *Registry { return &Registry{counters: map[string]int64{}} }

var loads atomic.Int32

func main() {
	_ = maps.Clone(map[string]int{})
	_ = slices.Sorted[string]
	_ = strings.Join
	_ = sync.OnceValue[int]
	fmt.Println("TODO")
}
@@expected
http_requests{method=GET,status=200} 100
http_requests{method=GET,status=500} 33
http_requests{method=POST,status=200} 67
login_failures 200
limit 100 loads 1
snapshot is a copy: true
@@hint
Build the key with a helper: collect `k=v` strings from the pairs, sort them, `name + "{" + strings.Join(parts, ",") + "}"` (no braces without labels). In `main`: for `i` in `range 200`, with `wg.Go`, call `loadLimits()` and `Inc("login_failures")`; `i%2 == 0` → GET 200; otherwise `i%3 == 0` → GET 500, else POST 200. `loadLimits := sync.OnceValue(func() int { loads.Add(1); return 100 })`.
@@solution
package main

import (
	"fmt"
	"maps"
	"slices"
	"strings"
	"sync"
	"sync/atomic"
)

type Registry struct {
	mu       sync.RWMutex
	counters map[string]int64
}

func NewRegistry() *Registry { return &Registry{counters: map[string]int64{}} }

func key(name string, labels []string) string {
	if len(labels) == 0 {
		return name
	}
	var parts []string
	for i := 0; i+1 < len(labels); i += 2 {
		parts = append(parts, labels[i]+"="+labels[i+1])
	}
	slices.Sort(parts)
	return name + "{" + strings.Join(parts, ",") + "}"
}

func (r *Registry) Inc(name string, labels ...string) {
	k := key(name, labels)
	r.mu.Lock()
	defer r.mu.Unlock()
	r.counters[k]++
}

func (r *Registry) Snapshot() map[string]int64 {
	r.mu.RLock()
	defer r.mu.RUnlock()
	return maps.Clone(r.counters)
}

func (r *Registry) Render() string {
	snap := r.Snapshot()
	var b strings.Builder
	for _, k := range slices.Sorted(maps.Keys(snap)) {
		fmt.Fprintf(&b, "%s %d\n", k, snap[k])
	}
	return b.String()
}

var loads atomic.Int32

func main() {
	reg := NewRegistry()
	loadLimits := sync.OnceValue(func() int {
		loads.Add(1)
		return 100
	})
	var wg sync.WaitGroup
	for i := range 200 {
		wg.Go(func() {
			_ = loadLimits()
			reg.Inc("login_failures")
			switch {
			case i%2 == 0:
				reg.Inc("http_requests", "method", "GET", "status", "200")
			case i%3 == 0:
				reg.Inc("http_requests", "status", "500", "method", "GET")
			default:
				reg.Inc("http_requests", "method", "POST", "status", "200")
			}
		})
	}
	wg.Wait()
	fmt.Print(reg.Render())
	fmt.Println("limit", loadLimits(), "loads", loads.Load())
	snap := reg.Snapshot()
	snap["login_failures"] = 0
	fmt.Println("snapshot is a copy:", reg.Snapshot()["login_failures"] == 200)
}

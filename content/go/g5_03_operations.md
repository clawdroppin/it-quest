@@lesson g5-operations Running Go Services: Testing, Logging, Timeouts and Containers
@@concepts testing, concurrency, processes
@@text
Writing the service is half the job. The other half is knowing it works (tests), knowing what it's doing in production (logs, metrics, health checks), making sure it fails gracefully (timeouts), and shipping it (containers).

## Testing HTTP handlers

```go,norun
func TestGetDevice(t *testing.T) {
	api := NewAPI(fakeStore{devices: map[int]Device{7: {ID: 7, Hostname: "tk-lt-007"}}}, discardLogger)

	tests := []struct {
		name, path string
		wantStatus int
	}{
		{"found", "/devices/7", 200},
		{"missing", "/devices/8", 404},
		{"not a number", "/devices/abc", 400},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			rec := httptest.NewRecorder()
			api.ServeHTTP(rec, httptest.NewRequest("GET", tt.path, nil))
			if rec.Code != tt.wantStatus {
				t.Fatalf("status %d, want %d: %s", rec.Code, tt.wantStatus, rec.Body)
			}
		})
	}
}
```

`httptest.NewServer(handler)` starts a real server on a random local port when you need to test a client, or the whole stack including the network.

Run tests with the **race detector** in CI: `go test -race ./...`. It finds data races (two goroutines touching the same data without a lock), which every HTTP server is exposed to because each request runs in its own goroutine.

Other built-in tools: `go test -cover` (coverage), benchmarks (`func BenchmarkX(b *testing.B)`, run with `go test -bench .`), fuzzing (`func FuzzParse(f *testing.F)`, run with `go test -fuzz FuzzParse`), and `go vet` for suspicious code. `staticcheck` and `golangci-lint` add many more checks.

## Structured logging with log/slog

Production logs are read by machines (Elastic, Loki, Datadog, CloudWatch), so they should be **structured**: key-value pairs, usually as JSON, not sentences.

```go
package main

import (
	"log/slog"
	"os"
)

func main() {
	logger := slog.New(slog.NewJSONHandler(os.Stdout, &slog.HandlerOptions{Level: slog.LevelDebug}))
	logger.Info("request", "method", "GET", "path", "/devices", "status", 200, "ms", 3)

	reqLogger := logger.With("request_id", "a1b2c3")      // attach fields to every line from here
	reqLogger.Warn("slow database query", "ms", 1250)
	reqLogger.Debug("cache miss", "key", "device:7")
}
```

Log levels: `Debug` (off in production), `Info` (normal events), `Warn` (something odd but handled), `Error` (needs a human). Never log passwords, tokens or personal data.

## Timeouts everywhere

Every call that goes over the network can hang forever unless you say otherwise. The `context` package carries deadlines through your whole program:

```go,norun
// An outgoing call with a deadline
ctx, cancel := context.WithTimeout(r.Context(), 2*time.Second)
defer cancel()
req, _ := http.NewRequestWithContext(ctx, "GET", "https://inventory.internal/api/devices", nil)
resp, err := client.Do(req)       // fails with context.DeadlineExceeded after 2 s

// A shared client with its own overall timeout (never use http.DefaultClient in production: it has none)
var client = &http.Client{Timeout: 10 * time.Second}
```

Combined with **retries with backoff** for temporary failures, and a **circuit breaker** (stop calling a service that keeps failing, try again later), this is how services stay up when their dependencies don't.

## Health checks and metrics

Load balancers and Kubernetes ask the service whether it's alive:

- **liveness** (`/healthz`): "the process works": just return 200.
- **readiness** (`/readyz`): "I can serve traffic": check the database connection (`db.PingContext`); return 503 during startup and shutdown.

**Metrics** are numbers over time: requests per second, error rate, latency percentiles, goroutines, memory. The standard is **Prometheus**: the service exposes `/metrics` (with `github.com/prometheus/client_golang`) and Grafana draws the dashboards. **OpenTelemetry** adds traces: following one request across several services.

Go also has a built-in profiler: import `net/http/pprof` (on an internal-only port!) and run `go tool pprof http://localhost:6060/debug/pprof/profile` to see where CPU time goes.

## Shipping in a container

Go compiles to one static binary, so the container image can be tiny: a few megabytes, with no shell or package manager for an attacker to use.

```text
# Dockerfile: a multi-stage build
FROM golang:1.23 AS build
WORKDIR /src
COPY go.mod go.sum ./
RUN go mod download                      # cached until go.mod changes
COPY . .
RUN CGO_ENABLED=0 go build -trimpath -ldflags="-s -w" -o /deviceapi ./cmd/deviceapi

FROM gcr.io/distroless/static-debian12   # no shell, runs as non-root
COPY --from=build /deviceapi /deviceapi
EXPOSE 8080
ENV ADDR=:8080
ENTRYPOINT ["/deviceapi"]
```

```text
docker build -t deviceapi:1.0 .
docker run --rm -p 8080:8080 -e DATABASE_URL=... deviceapi:1.0
```

Without containers: `GOOS=linux GOARCH=amd64 go build` cross-compiles for a Linux server from Windows or macOS, and a systemd unit runs it.

@@quiz
Why run `go test -race` in CI for an HTTP service?
- It makes the tests faster
* Each request runs in its own goroutine, and the race detector finds unsynchronized access to shared data
- It's required for httptest
- It checks for slow requests
? Data races cause rare, random corruption that normal tests almost never catch.

@@quiz
Why is `http.DefaultClient` a problem in production code?
* It has no timeout, so a stuck server can block the call forever
- It doesn't support HTTPS
- It's deprecated
- It can't send JSON
? Create a client with a Timeout, and use contexts with deadlines for each call.

@@quiz
What's the difference between a liveness and a readiness check?
- None
* Liveness: the process is alive (restart it if not). Readiness: it can serve traffic right now (don't send requests if not)
- Liveness checks the database, readiness checks the CPU
- Readiness is only for Windows
? During startup or when the database is down, a service is alive but not ready.

@@quiz
Why use a multi-stage Dockerfile for a Go service?
- Go requires it
* The final image contains only the compiled binary, not the compiler and source, so it's tiny and has less to attack
- It makes the code run faster
- It allows running several services in one container
? The build stage has the Go toolchain; the final stage has just the binary.

@@flashcards
`httptest.NewRecorder` / `httptest.NewServer` :: test a handler in memory / with a real local server
`go test -race` :: detect data races
`go test -cover` / `-bench` / `-fuzz` :: coverage / benchmarks / fuzzing
`log/slog` :: structured logging in the standard library
`slog.NewJSONHandler` :: JSON log lines for log platforms
`logger.With(k, v)` :: a logger that adds fields to every line
`context.WithTimeout` :: a deadline for everything below it
`&http.Client{Timeout: ...}` :: an HTTP client that can't hang forever
liveness / readiness :: alive / able to serve traffic
Prometheus `/metrics` :: numbers over time, scraped and graphed
`net/http/pprof` :: built-in CPU and memory profiling
multi-stage Docker build :: compile in one image, ship only the binary
`CGO_ENABLED=0` :: build a fully static binary
distroless :: minimal base images without a shell

@@exercise The fastest answer, with a deadline
`firstAnswer` asks several mirrors at once and returns the name of the **first one to answer**, or the context's error if the deadline passes first. Each mirror is simulated by `ask`, which takes `delay` to answer. Start one goroutine per mirror sending to a **buffered** channel (so the slow ones don't block forever), then `select` on the channel and `ctx.Done()`.
@@starter
package main

import (
	"context"
	"fmt"
	"time"
)

type mirror struct {
	name  string
	delay time.Duration
}

func ask(m mirror) string {
	time.Sleep(m.delay)
	return m.name
}

func firstAnswer(ctx context.Context, mirrors []mirror) (string, error) {
	// TODO
	return "", nil
}

func main() {
	mirrors := []mirror{{"tokyo", 300 * time.Millisecond}, {"osaka", 60 * time.Millisecond}, {"seoul", 500 * time.Millisecond}}

	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()
	name, err := firstAnswer(ctx, mirrors)
	fmt.Printf("answer=%q err=%v\n", name, err)

	ctx2, cancel2 := context.WithTimeout(context.Background(), 10*time.Millisecond)
	defer cancel2()
	name, err = firstAnswer(ctx2, mirrors)
	fmt.Printf("answer=%q err=%v\n", name, err)
}
@@expected
answer="osaka" err=<nil>
answer="" err=context deadline exceeded
@@requires
select
ctx.Done()
@@hint
`answers := make(chan string, len(mirrors))`, then `for _, m := range mirrors { go func(m mirror) { answers <- ask(m) }(m) }`, then `select { case name := <-answers: return name, nil; case <-ctx.Done(): return "", ctx.Err() }`.
@@solution
package main

import (
	"context"
	"fmt"
	"time"
)

type mirror struct {
	name  string
	delay time.Duration
}

func ask(m mirror) string {
	time.Sleep(m.delay)
	return m.name
}

func firstAnswer(ctx context.Context, mirrors []mirror) (string, error) {
	answers := make(chan string, len(mirrors))
	for _, m := range mirrors {
		go func(m mirror) { answers <- ask(m) }(m)
	}
	select {
	case name := <-answers:
		return name, nil
	case <-ctx.Done():
		return "", ctx.Err()
	}
}

func main() {
	mirrors := []mirror{{"tokyo", 300 * time.Millisecond}, {"osaka", 60 * time.Millisecond}, {"seoul", 500 * time.Millisecond}}

	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()
	name, err := firstAnswer(ctx, mirrors)
	fmt.Printf("answer=%q err=%v\n", name, err)

	ctx2, cancel2 := context.WithTimeout(context.Background(), 10*time.Millisecond)
	defer cancel2()
	name, err = firstAnswer(ctx2, mirrors)
	fmt.Printf("answer=%q err=%v\n", name, err)
}

@@exercise Structured logs
Create a JSON `slog` logger writing to `os.Stdout` that shows **Info and above** and **leaves out the time** (use `HandlerOptions.ReplaceAttr` to drop the attribute whose key is `slog.TimeKey`, so the output is the same every run). Then log the three lines in `main` as described.
@@starter
package main

import (
	"log/slog"
	"os"
)

func newLogger() *slog.Logger {
	// TODO
	return slog.New(slog.NewTextHandler(os.Stdout, nil))
}

func main() {
	logger := newLogger()
	logger.Debug("cache miss", "key", "device:7")                         // must not appear
	logger.Info("request", "method", "GET", "path", "/devices", "status", 200)
	logger.With("request_id", "a1b2").Warn("slow query", "ms", 1250)
}
@@expected
{"level":"INFO","msg":"request","method":"GET","path":"/devices","status":200}
{"level":"WARN","msg":"slow query","request_id":"a1b2","ms":1250}
@@requires
slog.NewJSONHandler
ReplaceAttr
slog.TimeKey
@@hint
`slog.New(slog.NewJSONHandler(os.Stdout, &slog.HandlerOptions{Level: slog.LevelInfo, ReplaceAttr: func(groups []string, a slog.Attr) slog.Attr { if a.Key == slog.TimeKey && len(groups) == 0 { return slog.Attr{} }; return a }}))`. An empty `slog.Attr{}` is dropped from the output.
@@solution
package main

import (
	"log/slog"
	"os"
)

func newLogger() *slog.Logger {
	return slog.New(slog.NewJSONHandler(os.Stdout, &slog.HandlerOptions{
		Level: slog.LevelInfo,
		ReplaceAttr: func(groups []string, a slog.Attr) slog.Attr {
			if a.Key == slog.TimeKey && len(groups) == 0 {
				return slog.Attr{}
			}
			return a
		},
	}))
}

func main() {
	logger := newLogger()
	logger.Debug("cache miss", "key", "device:7")                         // must not appear
	logger.Info("request", "method", "GET", "path", "/devices", "status", 200)
	logger.With("request_id", "a1b2").Warn("slow query", "ms", 1250)
}

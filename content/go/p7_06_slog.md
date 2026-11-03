@@lesson p7-slog Observability: Structured Logging with slog, Metrics and Tracing
@@concepts text-processing, json
@@text
In production, logs are read by machines (Loki, Elasticsearch, CloudWatch) before humans. **Structured logs** are key-value records, not free text, so you can query `status>=500 AND path="/login"`.

## log/slog

```go
package main

import (
	"log/slog"
	"os"
)

func main() {
	opts := &slog.HandlerOptions{
		ReplaceAttr: func(groups []string, a slog.Attr) slog.Attr {
			if a.Key == slog.TimeKey && len(groups) == 0 {
				return slog.Attr{} // drop the timestamp so this example's output is stable
			}
			return a
		},
	}
	logger := slog.New(slog.NewJSONHandler(os.Stdout, opts))
	logger.Info("server started", "addr", ":8080", "workers", 4)

	text := slog.New(slog.NewTextHandler(os.Stdout, opts))
	reqLog := text.With("request_id", "req-42") // attributes on every record
	reqLog.Warn("slow query", slog.Group("db", "table", "hosts", "ms", 320))
}
```

- Levels: `Debug`, `Info`, `Warn`, `Error`. `HandlerOptions.Level` sets the minimum; a `*slog.LevelVar` can be changed at run time (e.g. from an admin endpoint).
- `logger.With(...)` returns a child logger with attributes attached to every record: request ids, user ids, component names.
- `slog.Group` nests attributes; `slog.Int`, `slog.String`, `slog.Duration`... avoid allocations and key/value mismatches.
- `slog.SetDefault(logger)` also routes the old `log.Printf` through slog.
- `InfoContext(ctx, ...)` passes the context to the handler, where a custom handler can pull out a trace id or request id.

## Don't log secrets

Passwords, tokens and personal data in logs are a classic breach source (and a GDPR/APPI issue). Implement `slog.LogValuer` on sensitive types so they always render redacted, wherever they're logged:

```go,norun
type Token string

func (Token) LogValue() slog.Value { return slog.StringValue("[REDACTED]") }
```

## Custom handlers

A `slog.Handler` has four methods: `Enabled`, `Handle`, `WithAttrs` and `WithGroup`. Wrapping an existing handler is the common pattern, e.g. to add a request id from the context to every record.

## Metrics and tracing

Logs answer "what happened to this request?". **Metrics** answer "how is the system doing?":

- `expvar` (standard library) exposes counters as JSON at `/debug/vars`.
- `github.com/prometheus/client_golang` exposes counters, gauges and histograms for Prometheus to scrape; RED metrics (Rate, Errors, Duration) per endpoint are the baseline.

**Tracing** follows one request across services. OpenTelemetry (`go.opentelemetry.io/otel`) is the standard: spans with timings and attributes, propagated through `context.Context` and HTTP/gRPC headers, viewed in Jaeger, Tempo or a cloud APM. Put the trace id in your logs to jump between the two.

@@quiz
Why prefer structured logs over `log.Printf` strings?
* They can be filtered and aggregated by field in log systems
- They're shorter
- They're required by Go
- They're encrypted
? `status=500 path=/login` beats parsing free text with regexes.

@@quiz
What's the cleanest way to stop a `Password` type from ever appearing in logs?
- Remember not to log it
* Implement `LogValue()` returning a redacted value
- Use Debug level
- Base64-encode it
? It applies everywhere the value is logged, even by accident.

@@quiz
What does `logger.With("request_id", id)` return?
- A context
* A new logger that adds the attribute to every record
- The same logger, modified
- A handler
? Loggers are immutable; With creates a child.

@@flashcards
`slog.NewJSONHandler` :: JSON lines output
`slog.NewTextHandler` :: key=value output
`logger.With` :: child logger with attributes
`slog.Group` :: nested attributes
`slog.LevelVar` :: level changeable at run time
`ReplaceAttr` :: rewrite or drop attributes
`LogValuer` :: control how a type is logged
`InfoContext` :: pass context to the handler
expvar :: stdlib metrics endpoint
OpenTelemetry :: traces, metrics, logs standard

@@exercise Safe JSON logging
Configure a logger for an auth service:

- JSON handler writing to stdout, level from a `*slog.LevelVar` (initially Info), with `ReplaceAttr` dropping the top-level time
- every record has `service=auth` (use `With`)
- `Secret` is a string type whose `LogValue` is `[REDACTED]`
- `User{Name, Email}` logs as a group `{"name":..., "email":...}` with the email masked to the first letter + `***` + the domain (`a***@example.com`)

`main` logs several records (provided); the Debug one only appears after the level is lowered.
@@starter
package main

import (
	"errors"
	"log/slog"
	"os"
	"strings"
)

type Secret string

type User struct {
	Name, Email string
}

func main() {
	_ = strings.Cut
	level := new(slog.LevelVar)
	logger := slog.New(slog.NewJSONHandler(os.Stdout, nil))
	_ = level
	logger.Info("login", "user", "ana", "password", Secret("hunter2"))
	logger.Debug("cache miss", "key", "u:ana")
	// TODO: level.Set(slog.LevelDebug)
	logger.Debug("cache miss", "key", "u:ana")
	logger.Warn("slow query", slog.Group("db", "table", "users", "ms", 250))
	logger.Info("signup", "user", User{"ana", "ana@example.com"})
	logger.Error("token check failed", "err", errors.New("expired"), "token", Secret("abc"))
}
@@expected
{"level":"INFO","msg":"login","service":"auth","user":"ana","password":"[REDACTED]"}
{"level":"DEBUG","msg":"cache miss","service":"auth","key":"u:ana"}
{"level":"WARN","msg":"slow query","service":"auth","db":{"table":"users","ms":250}}
{"level":"INFO","msg":"signup","service":"auth","user":{"name":"ana","email":"a***@example.com"}}
{"level":"ERROR","msg":"token check failed","service":"auth","err":"expired","token":"[REDACTED]"}
@@requires
LogValue()
ReplaceAttr
LevelVar
@@hint
`func (Secret) LogValue() slog.Value { return slog.StringValue("[REDACTED]") }`. For `User`: `return slog.GroupValue(slog.String("name", u.Name), slog.String("email", mask(u.Email)))` where `mask` uses `strings.Cut(email, "@")`. Handler options: `&slog.HandlerOptions{Level: level, ReplaceAttr: ...}`.
@@solution
package main

import (
	"errors"
	"log/slog"
	"os"
	"strings"
)

type Secret string

func (Secret) LogValue() slog.Value { return slog.StringValue("[REDACTED]") }

type User struct {
	Name, Email string
}

func mask(email string) string {
	local, domain, ok := strings.Cut(email, "@")
	if !ok || local == "" {
		return "***"
	}
	return local[:1] + "***@" + domain
}

func (u User) LogValue() slog.Value {
	return slog.GroupValue(slog.String("name", u.Name), slog.String("email", mask(u.Email)))
}

func main() {
	level := new(slog.LevelVar)
	opts := &slog.HandlerOptions{
		Level: level,
		ReplaceAttr: func(groups []string, a slog.Attr) slog.Attr {
			if a.Key == slog.TimeKey && len(groups) == 0 {
				return slog.Attr{}
			}
			return a
		},
	}
	logger := slog.New(slog.NewJSONHandler(os.Stdout, opts)).With("service", "auth")
	logger.Info("login", "user", "ana", "password", Secret("hunter2"))
	logger.Debug("cache miss", "key", "u:ana")
	level.Set(slog.LevelDebug)
	logger.Debug("cache miss", "key", "u:ana")
	logger.Warn("slow query", slog.Group("db", "table", "users", "ms", 250))
	logger.Info("signup", "user", User{"ana", "ana@example.com"})
	logger.Error("token check failed", "err", errors.New("expired"), "token", Secret("abc"))
}

@@exercise Request logging middleware
Build HTTP middleware for access logs:

- `RequestID(next)`: takes `X-Request-ID` from the request or generates `req-1`, `req-2`...; stores it in the request context and sets the response header
- `AccessLog(logger, now, next)`: wraps the `ResponseWriter` to capture the status (default 200), and logs `request` with `method`, `path`, `status`, `duration` after the handler; level Info, Warn for 4xx, Error for 5xx
- `ctxHandler`: a `slog.Handler` wrapper whose `Handle` adds `request_id` from the context to every record (use `InfoContext`-style `Log(ctx, ...)` calls so it gets the context)

The clock is fake: each `now()` call advances 1 ms, and `/slow` advances it 250 ms. Requests are run with `httptest` (no network).
@@starter
package main

import (
	"context"
	"fmt"
	"log/slog"
	"net/http"
	"net/http/httptest"
	"os"
	"time"
)

type Clock struct{ t time.Time }

func (c *Clock) Now() time.Time {
	c.t = c.t.Add(time.Millisecond)
	return c.t
}
func (c *Clock) Advance(d time.Duration) { c.t = c.t.Add(d) }

func main() {
	clk := &Clock{}
	mux := http.NewServeMux()
	mux.HandleFunc("GET /health", func(w http.ResponseWriter, r *http.Request) { fmt.Fprint(w, "ok") })
	mux.HandleFunc("GET /slow", func(w http.ResponseWriter, r *http.Request) { clk.Advance(250 * time.Millisecond) })
	mux.HandleFunc("GET /fail", func(w http.ResponseWriter, r *http.Request) { http.Error(w, "boom", 500) })
	_ = context.Background
	_ = slog.New
	_ = os.Stdout
	_ = httptest.NewRecorder
	fmt.Println("TODO")
}
@@expected
level=INFO msg=request method=GET path=/health status=200 duration=1ms request_id=abc
-> 200 abc
level=INFO msg=request method=GET path=/slow status=200 duration=251ms request_id=req-1
-> 200 req-1
level=ERROR msg=request method=GET path=/fail status=500 duration=1ms request_id=req-2
-> 500 req-2
level=WARN msg=request method=GET path=/nope status=404 duration=1ms request_id=req-3
-> 404 req-3
level=WARN msg=request method=POST path=/health status=405 duration=1ms request_id=req-4
-> 405 req-4
@@requires
http.ResponseWriter
slog.Handler
@@hint
`type statusWriter struct { http.ResponseWriter; status int }` with `WriteHeader(code)` saving the code. `type ctxKey struct{}`. `ctxHandler struct{ slog.Handler }` with `Handle(ctx, r)` doing `if id, ok := ctx.Value(ctxKey{}).(string); ok { r.AddAttrs(slog.String("request_id", id)) }` then `h.Handler.Handle(ctx, r)`; also override `WithAttrs`/`WithGroup` to keep the wrapper. Log with `logger.Log(r.Context(), level, "request", ...)`. Drop the time with `ReplaceAttr`. Requests: `/health` with header `X-Request-ID: abc`, then `/slow`, `/fail`, `/nope`, and `POST /health`; print `-> <code> <X-Request-ID header>` after each.
@@solution
package main

import (
	"context"
	"fmt"
	"log/slog"
	"net/http"
	"net/http/httptest"
	"os"
	"time"
)

type Clock struct{ t time.Time }

func (c *Clock) Now() time.Time {
	c.t = c.t.Add(time.Millisecond)
	return c.t
}
func (c *Clock) Advance(d time.Duration) { c.t = c.t.Add(d) }

type ctxKey struct{}

type ctxHandler struct{ slog.Handler }

func (h ctxHandler) Handle(ctx context.Context, r slog.Record) error {
	if id, ok := ctx.Value(ctxKey{}).(string); ok {
		r.AddAttrs(slog.String("request_id", id))
	}
	return h.Handler.Handle(ctx, r)
}
func (h ctxHandler) WithAttrs(as []slog.Attr) slog.Handler { return ctxHandler{h.Handler.WithAttrs(as)} }
func (h ctxHandler) WithGroup(name string) slog.Handler    { return ctxHandler{h.Handler.WithGroup(name)} }

func RequestID(next http.Handler) http.Handler {
	n := 0
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		id := r.Header.Get("X-Request-ID")
		if id == "" {
			n++
			id = fmt.Sprintf("req-%d", n)
		}
		w.Header().Set("X-Request-ID", id)
		next.ServeHTTP(w, r.WithContext(context.WithValue(r.Context(), ctxKey{}, id)))
	})
}

type statusWriter struct {
	http.ResponseWriter
	status int
}

func (s *statusWriter) WriteHeader(code int) {
	s.status = code
	s.ResponseWriter.WriteHeader(code)
}

func AccessLog(logger *slog.Logger, now func() time.Time, next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		start := now()
		sw := &statusWriter{ResponseWriter: w, status: http.StatusOK}
		next.ServeHTTP(sw, r)
		level := slog.LevelInfo
		switch {
		case sw.status >= 500:
			level = slog.LevelError
		case sw.status >= 400:
			level = slog.LevelWarn
		}
		logger.Log(r.Context(), level, "request",
			"method", r.Method, "path", r.URL.Path, "status", sw.status, "duration", now().Sub(start))
	})
}

func main() {
	clk := &Clock{}
	mux := http.NewServeMux()
	mux.HandleFunc("GET /health", func(w http.ResponseWriter, r *http.Request) { fmt.Fprint(w, "ok") })
	mux.HandleFunc("GET /slow", func(w http.ResponseWriter, r *http.Request) { clk.Advance(250 * time.Millisecond) })
	mux.HandleFunc("GET /fail", func(w http.ResponseWriter, r *http.Request) { http.Error(w, "boom", 500) })

	opts := &slog.HandlerOptions{ReplaceAttr: func(groups []string, a slog.Attr) slog.Attr {
		if a.Key == slog.TimeKey && len(groups) == 0 {
			return slog.Attr{}
		}
		return a
	}}
	logger := slog.New(ctxHandler{slog.NewTextHandler(os.Stdout, opts)})
	app := RequestID(AccessLog(logger, clk.Now, mux))

	reqs := []*http.Request{
		httptest.NewRequest("GET", "/health", nil),
		httptest.NewRequest("GET", "/slow", nil),
		httptest.NewRequest("GET", "/fail", nil),
		httptest.NewRequest("GET", "/nope", nil),
		httptest.NewRequest("POST", "/health", nil),
	}
	reqs[0].Header.Set("X-Request-ID", "abc")
	for _, r := range reqs {
		rec := httptest.NewRecorder()
		app.ServeHTTP(rec, r)
		fmt.Println("->", rec.Code, rec.Header().Get("X-Request-ID"))
	}
}

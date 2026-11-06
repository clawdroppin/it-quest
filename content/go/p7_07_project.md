@@lesson p7-project Project: A Production-Ready HTTP Service
@@concepts networking, json
@@text
Time to put the production pieces together: routing, JSON validation, proper status codes, safe concurrent storage, and a server that shuts down without dropping requests.

## Routing with net/http (Go 1.22+)

The standard `ServeMux` supports methods and path wildcards, so many services need no router library:

```go,norun
mux := http.NewServeMux()
mux.HandleFunc("GET /hosts", s.list)
mux.HandleFunc("POST /hosts", s.create)
mux.HandleFunc("GET /hosts/{name}", s.get)       // r.PathValue("name")
mux.HandleFunc("DELETE /hosts/{name}", s.delete)
```

Wrong methods get `405 Method Not Allowed` with an `Allow` header automatically; unknown paths get `404`.

## Reading JSON safely

```go,norun
r.Body = http.MaxBytesReader(w, r.Body, 1<<20) // limit body size
dec := json.NewDecoder(r.Body)
dec.DisallowUnknownFields()                     // typos become errors
if err := dec.Decode(&in); err != nil { badRequest(w, err); return }
```

Validate everything (`netip.ParseAddr` for addresses), and return consistent JSON errors: `{"error": "..."}` with the right status: 400 invalid input, 404 missing, 409 conflict, 201 + `Location` for created resources, 204 for deletes.

## Server configuration

`http.ListenAndServe` has **no timeouts**, so slow clients can hold connections forever. Configure a server:

```go,norun
srv := &http.Server{
	Addr:              ":8080",
	Handler:           app,
	ReadHeaderTimeout: 5 * time.Second,
	ReadTimeout:       30 * time.Second,
	WriteTimeout:      30 * time.Second,
	IdleTimeout:       120 * time.Second,
}
```

## Graceful shutdown

When Kubernetes or systemd stops your service it sends SIGTERM. A graceful service:

1. marks itself **not ready** (`/readyz` returns 503) so the load balancer stops sending traffic
2. calls `srv.Shutdown(ctx)`: stop accepting connections, wait for in-flight requests to finish (up to the context deadline)
3. closes databases and flushes logs, then exits

```go,norun
ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
defer stop()
go func() {
	if err := srv.ListenAndServe(); err != nil && !errors.Is(err, http.ErrServerClosed) {
		log.Fatal(err)
	}
}()
<-ctx.Done()
ready.Store(false)
shutdownCtx, cancel := context.WithTimeout(context.Background(), 20*time.Second)
defer cancel()
srv.Shutdown(shutdownCtx)
```

`/healthz` (liveness: the process works) and `/readyz` (readiness: it should receive traffic) are separate on purpose.

## Shipping it

```text
# Dockerfile: multi-stage, tiny final image
FROM golang:1.27 AS build
WORKDIR /src
COPY . .
RUN CGO_ENABLED=0 go build -trimpath -ldflags="-s -w" -o /app ./cmd/server

FROM gcr.io/distroless/static-debian12:nonroot
COPY --from=build /app /app
USER nonroot
ENTRYPOINT ["/app"]
```

A static Go binary in a distroless or `scratch` image is often under 20 MB with almost no attack surface. Static files (templates, a web UI) can be compiled into the binary with `//go:embed`.

@@quiz
What does `srv.Shutdown(ctx)` do?
* Stops accepting new connections and waits for in-flight requests until ctx expires
- Kills all connections immediately
- Restarts the server
- Only closes idle connections
? `srv.Close()` is the immediate version.

@@quiz
Why mark the service not ready before shutting down?
- It's faster
* So the load balancer stops routing new requests to it while it drains
- To pass the race detector
- To restart it
? Otherwise new requests may hit a closing listener.

@@quiz
Which status fits creating a resource successfully?
- 200 with no body
* 201 Created with a Location header
- 204 No Content
- 302 Found
? 204 is typical for a successful DELETE.

@@drill
Which `http.Server` method stops accepting connections and waits for active ones?
= Shutdown
= Shutdown()
? srv.Shutdown(ctx) gives in-flight requests time to finish.

@@flashcards
`"GET /hosts/{name}"` :: method + wildcard pattern (Go 1.22)
`r.PathValue("name")` :: read a wildcard
`MaxBytesReader` :: cap request body size
`DisallowUnknownFields` :: reject unexpected JSON keys
`ReadHeaderTimeout` :: protect against slow clients
`srv.Shutdown(ctx)` :: graceful stop
`http.ErrServerClosed` :: normal result after Shutdown
`/healthz` vs `/readyz` :: alive vs ready for traffic
distroless :: minimal container base image
`//go:embed` :: compile files into the binary

@@exercise The hosts API
Implement a JSON API over an in-memory store protected by a `sync.RWMutex`:

- `GET /hosts`: 200, array sorted by name
- `POST /hosts`: body `{"name","ip"}`, unknown fields rejected; errors with 400 `{"error":...}`: `name is required`, `invalid ip "<ip>"`, or `bad request body: <decoder error>`; duplicate name → 409 `host "<name>" already exists`; success → 201, `Location: /hosts/<name>`, the host as JSON
- `GET /hosts/{name}`: 200 or 404 `host "<name>" not found`
- `DELETE /hosts/{name}`: 204 or 404

`main` (provided) drives the API with `httptest` and prints each status and body.
@@starter
package main

import (
	"encoding/json"
	"fmt"
	"net/http"
	"net/http/httptest"
	"net/netip"
	"strings"
	"sync"
)

type Host struct {
	Name string `json:"name"`
	IP   string `json:"ip"`
}

type Server struct {
	mu    sync.RWMutex
	hosts map[string]Host
}

func NewServer() *Server { return &Server{hosts: map[string]Host{}} }

func (s *Server) Routes() http.Handler {
	mux := http.NewServeMux()
	_ = json.NewEncoder
	_ = netip.ParseAddr
	return mux
}

func main() {
	app := NewServer().Routes()
	do := func(method, path, body string) {
		rec := httptest.NewRecorder()
		app.ServeHTTP(rec, httptest.NewRequest(method, path, strings.NewReader(body)))
		loc := ""
		if l := rec.Header().Get("Location"); l != "" {
			loc = " Location=" + l
		}
		fmt.Printf("%s %s -> %d%s %s\n", method, path, rec.Code, loc, strings.TrimSpace(rec.Body.String()))
	}
	do("POST", "/hosts", `{"name":"web01","ip":"10.0.0.5"}`)
	do("POST", "/hosts", `{"name":"db01","ip":"10.0.0.9"}`)
	do("POST", "/hosts", `{"name":"web01","ip":"10.0.0.6"}`)
	do("POST", "/hosts", `{"name":"","ip":"x"}`)
	do("POST", "/hosts", `{"name":"x1","ip":"999.1.1.1"}`)
	do("POST", "/hosts", `{"name":"x2","ip":"10.0.0.1","extra":1}`)
	do("GET", "/hosts", "")
	do("GET", "/hosts/web01", "")
	do("DELETE", "/hosts/web01", "")
	do("GET", "/hosts/web01", "")
	do("DELETE", "/hosts/web01", "")
	do("PUT", "/hosts", "")
}
@@expected
POST /hosts -> 201 Location=/hosts/web01 {"name":"web01","ip":"10.0.0.5"}
POST /hosts -> 201 Location=/hosts/db01 {"name":"db01","ip":"10.0.0.9"}
POST /hosts -> 409 {"error":"host \"web01\" already exists"}
POST /hosts -> 400 {"error":"name is required"}
POST /hosts -> 400 {"error":"invalid ip \"999.1.1.1\""}
POST /hosts -> 400 {"error":"bad request body: json: unknown field \"extra\""}
GET /hosts -> 200 [{"name":"db01","ip":"10.0.0.9"},{"name":"web01","ip":"10.0.0.5"}]
GET /hosts/web01 -> 200 {"name":"web01","ip":"10.0.0.5"}
DELETE /hosts/web01 -> 204
GET /hosts/web01 -> 404 {"error":"host \"web01\" not found"}
DELETE /hosts/web01 -> 404 {"error":"host \"web01\" not found"}
PUT /hosts -> 405 Method Not Allowed
@@requires
r.PathValue("name")
DisallowUnknownFields
@@hint
A helper `writeJSON(w, status, v)` sets `Content-Type`, calls `w.WriteHeader(status)` and encodes `v`; `writeErr(w, status, msg)` sends `map[string]string{"error": msg}`. For the list, collect the map values and sort with `slices.SortFunc` by name (or sort the keys). Take the write lock for create and delete, the read lock for get and list.
@@solution
package main

import (
	"encoding/json"
	"fmt"
	"maps"
	"net/http"
	"net/http/httptest"
	"net/netip"
	"slices"
	"strings"
	"sync"
)

type Host struct {
	Name string `json:"name"`
	IP   string `json:"ip"`
}

type Server struct {
	mu    sync.RWMutex
	hosts map[string]Host
}

func NewServer() *Server { return &Server{hosts: map[string]Host{}} }

func writeJSON(w http.ResponseWriter, status int, v any) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)
	json.NewEncoder(w).Encode(v)
}

func writeErr(w http.ResponseWriter, status int, msg string) {
	writeJSON(w, status, map[string]string{"error": msg})
}

func (s *Server) list(w http.ResponseWriter, r *http.Request) {
	s.mu.RLock()
	defer s.mu.RUnlock()
	out := []Host{}
	for _, name := range slices.Sorted(maps.Keys(s.hosts)) {
		out = append(out, s.hosts[name])
	}
	writeJSON(w, http.StatusOK, out)
}

func (s *Server) create(w http.ResponseWriter, r *http.Request) {
	r.Body = http.MaxBytesReader(w, r.Body, 1<<20)
	dec := json.NewDecoder(r.Body)
	dec.DisallowUnknownFields()
	var h Host
	if err := dec.Decode(&h); err != nil {
		writeErr(w, http.StatusBadRequest, "bad request body: "+err.Error())
		return
	}
	if h.Name == "" {
		writeErr(w, http.StatusBadRequest, "name is required")
		return
	}
	if _, err := netip.ParseAddr(h.IP); err != nil {
		writeErr(w, http.StatusBadRequest, fmt.Sprintf("invalid ip %q", h.IP))
		return
	}
	s.mu.Lock()
	defer s.mu.Unlock()
	if _, exists := s.hosts[h.Name]; exists {
		writeErr(w, http.StatusConflict, fmt.Sprintf("host %q already exists", h.Name))
		return
	}
	s.hosts[h.Name] = h
	w.Header().Set("Location", "/hosts/"+h.Name)
	writeJSON(w, http.StatusCreated, h)
}

func (s *Server) get(w http.ResponseWriter, r *http.Request) {
	name := r.PathValue("name")
	s.mu.RLock()
	defer s.mu.RUnlock()
	h, ok := s.hosts[name]
	if !ok {
		writeErr(w, http.StatusNotFound, fmt.Sprintf("host %q not found", name))
		return
	}
	writeJSON(w, http.StatusOK, h)
}

func (s *Server) delete(w http.ResponseWriter, r *http.Request) {
	name := r.PathValue("name")
	s.mu.Lock()
	defer s.mu.Unlock()
	if _, ok := s.hosts[name]; !ok {
		writeErr(w, http.StatusNotFound, fmt.Sprintf("host %q not found", name))
		return
	}
	delete(s.hosts, name)
	w.WriteHeader(http.StatusNoContent)
}

func (s *Server) Routes() http.Handler {
	mux := http.NewServeMux()
	mux.HandleFunc("GET /hosts", s.list)
	mux.HandleFunc("POST /hosts", s.create)
	mux.HandleFunc("GET /hosts/{name}", s.get)
	mux.HandleFunc("DELETE /hosts/{name}", s.delete)
	return mux
}

func main() {
	app := NewServer().Routes()
	do := func(method, path, body string) {
		rec := httptest.NewRecorder()
		app.ServeHTTP(rec, httptest.NewRequest(method, path, strings.NewReader(body)))
		loc := ""
		if l := rec.Header().Get("Location"); l != "" {
			loc = " Location=" + l
		}
		fmt.Printf("%s %s -> %d%s %s\n", method, path, rec.Code, loc, strings.TrimSpace(rec.Body.String()))
	}
	do("POST", "/hosts", `{"name":"web01","ip":"10.0.0.5"}`)
	do("POST", "/hosts", `{"name":"db01","ip":"10.0.0.9"}`)
	do("POST", "/hosts", `{"name":"web01","ip":"10.0.0.6"}`)
	do("POST", "/hosts", `{"name":"","ip":"x"}`)
	do("POST", "/hosts", `{"name":"x1","ip":"999.1.1.1"}`)
	do("POST", "/hosts", `{"name":"x2","ip":"10.0.0.1","extra":1}`)
	do("GET", "/hosts", "")
	do("GET", "/hosts/web01", "")
	do("DELETE", "/hosts/web01", "")
	do("GET", "/hosts/web01", "")
	do("DELETE", "/hosts/web01", "")
	do("PUT", "/hosts", "")
}

@@exercise Graceful shutdown on a real socket
Run a real `http.Server` on `127.0.0.1:0` (a random free port) and shut it down gracefully while a request is in flight:

- `GET /readyz`: `ready` or `draining` (503), based on an `atomic.Bool`
- `GET /work`: signals on a `started` channel, sleeps 150 ms, writes `done`
- `Drain(srv, ready, timeout) error`: sets ready to false, then calls `srv.Shutdown` with a timeout context

`main`: check readiness, start a `/work` request in a goroutine, wait until it has started, mark draining and check `/readyz` again (503), then shut down. The in-flight request must still complete; `Serve` must return `http.ErrServerClosed`; and a new request afterwards must fail.
@@starter
package main

import (
	"context"
	"errors"
	"fmt"
	"io"
	"net"
	"net/http"
	"sync/atomic"
	"time"
)

func main() {
	ln, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		fmt.Println(err)
		return
	}
	base := "http://" + ln.Addr().String()
	var ready atomic.Bool
	ready.Store(true)
	_ = base
	_ = context.Background
	_ = errors.Is
	_ = io.ReadAll
	_ = http.NewServeMux
	_ = time.Second
	fmt.Println("TODO")
}
@@expected
readyz before: 200 ready
readyz while draining: 503 draining
shutdown error: <nil>
in-flight request: 200 done
serve returned ErrServerClosed: true
request after shutdown fails: true
@@requires
srv.Shutdown(
ReadHeaderTimeout
@@hint
Build `srv := &http.Server{Handler: mux, ReadHeaderTimeout: 5 * time.Second}` and `go func() { serveErr <- srv.Serve(ln) }()`. A small `get(path) (int, string, error)` helper reads the body. Order in `main`: readyz → start `/work` in a goroutine sending its result on a channel → `<-started` → `ready.Store(false)` and check readyz (the server still accepts while only marked draining) → `srv.Shutdown(ctx)` → print the work result and `errors.Is(<-serveErr, http.ErrServerClosed)` → one more `get` should return an error. Trim the body with `strings.TrimSpace` (http.Error adds a newline). Setting `ready` again inside `Drain` is harmless and keeps `Drain` correct when called on its own.
@@solution
package main

import (
	"context"
	"errors"
	"fmt"
	"io"
	"net"
	"net/http"
	"strings"
	"sync/atomic"
	"time"
)

func Drain(srv *http.Server, ready *atomic.Bool, timeout time.Duration) error {
	ready.Store(false)
	ctx, cancel := context.WithTimeout(context.Background(), timeout)
	defer cancel()
	return srv.Shutdown(ctx)
}

func main() {
	ln, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		fmt.Println(err)
		return
	}
	base := "http://" + ln.Addr().String()
	var ready atomic.Bool
	ready.Store(true)
	started := make(chan struct{})

	mux := http.NewServeMux()
	mux.HandleFunc("GET /readyz", func(w http.ResponseWriter, r *http.Request) {
		if !ready.Load() {
			http.Error(w, "draining", http.StatusServiceUnavailable)
			return
		}
		fmt.Fprint(w, "ready")
	})
	mux.HandleFunc("GET /work", func(w http.ResponseWriter, r *http.Request) {
		close(started)
		time.Sleep(150 * time.Millisecond)
		fmt.Fprint(w, "done")
	})
	srv := &http.Server{Handler: mux, ReadHeaderTimeout: 5 * time.Second}
	serveErr := make(chan error, 1)
	go func() { serveErr <- srv.Serve(ln) }()

	client := &http.Client{Timeout: 2 * time.Second}
	get := func(path string) (int, string, error) {
		resp, err := client.Get(base + path)
		if err != nil {
			return 0, "", err
		}
		defer resp.Body.Close()
		b, err := io.ReadAll(resp.Body)
		return resp.StatusCode, strings.TrimSpace(string(b)), err
	}

	code, body, _ := get("/readyz")
	fmt.Println("readyz before:", code, body)

	type result struct {
		code int
		body string
	}
	work := make(chan result, 1)
	go func() {
		c, b, _ := get("/work")
		work <- result{c, b}
	}()
	<-started

	ready.Store(false)
	code, body, _ = get("/readyz")
	fmt.Println("readyz while draining:", code, body)

	fmt.Println("shutdown error:", Drain(srv, &ready, 2*time.Second))
	r := <-work
	fmt.Println("in-flight request:", r.code, r.body)
	fmt.Println("serve returned ErrServerClosed:", errors.Is(<-serveErr, http.ErrServerClosed))
	_, _, err = get("/readyz")
	fmt.Println("request after shutdown fails:", err != nil)
}

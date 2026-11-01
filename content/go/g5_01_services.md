@@lesson g5-services HTTP Services in Go: Routing, JSON and Middleware
@@concepts networking, json, functions
@@text
Go was designed at Google for exactly this job: network services that are fast, use little memory, and compile to one file you copy onto a server. Docker, Kubernetes, Terraform, Prometheus, Grafana and Caddy are all written in Go. And the standard library alone is enough to build a production HTTP API.

## Routing with Go 1.22+ patterns

Since Go 1.22, `http.ServeMux` understands methods and path wildcards:

```go,norun
mux := http.NewServeMux()
mux.HandleFunc("GET /devices", listDevices)              // only GET (and HEAD)
mux.HandleFunc("POST /devices", createDevice)
mux.HandleFunc("GET /devices/{id}", getDevice)           // {id} matches one path segment
mux.HandleFunc("GET /files/{path...}", serveFile)        // {path...} matches the rest of the path

func getDevice(w http.ResponseWriter, r *http.Request) {
	id := r.PathValue("id")                              // "42" for /devices/42
	...
}
```

A request with the wrong method gets **405 Method Not Allowed** automatically, and an unknown path gets 404. (Your `go.mod` must say `go 1.22` or later for these patterns.)

## Testing a handler without a network

`net/http/httptest` calls a handler directly, in memory. It's how Go services are tested, and it lets you try handlers right here:

```go
package main

import (
	"fmt"
	"net/http"
	"net/http/httptest"
	"strings"
)

func main() {
	mux := http.NewServeMux()
	mux.HandleFunc("GET /devices/{id}", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintf(w, "device %s", r.PathValue("id"))
	})

	for _, path := range []string{"/devices/42", "/devices", "/devices/7"} {
		rec := httptest.NewRecorder()
		mux.ServeHTTP(rec, httptest.NewRequest("GET", path, nil))
		fmt.Println(path, rec.Code, strings.TrimSpace(rec.Body.String()))
	}
}
```

## JSON in and out

```go,norun
type Device struct {
	ID       int    `json:"id"`
	Hostname string `json:"hostname"`
	Owner    string `json:"owner,omitempty"`   // left out of the JSON when empty
}

func writeJSON(w http.ResponseWriter, status int, v any) {
	w.Header().Set("Content-Type", "application/json")   // headers before WriteHeader
	w.WriteHeader(status)                                 // the status before the body
	json.NewEncoder(w).Encode(v)
}

func createDevice(w http.ResponseWriter, r *http.Request) {
	var d Device
	dec := json.NewDecoder(io.LimitReader(r.Body, 1<<20))   // never read an unlimited body
	dec.DisallowUnknownFields()                             // "hostnme" is an error, not ignored
	if err := dec.Decode(&d); err != nil {
		writeJSON(w, http.StatusBadRequest, map[string]string{"error": "invalid JSON"})
		return                                              // don't forget to return!
	}
	...
	writeJSON(w, http.StatusCreated, d)
}
```

The order matters: set headers, then call `WriteHeader`, then write the body. Once the body has started, the status and headers are already sent. If you write without calling `WriteHeader`, the status is 200.

## Middleware: wrapping handlers

Logging, authentication, panic recovery, CORS and rate limiting apply to every route. In Go they're **middleware**: a function that takes a handler and returns a new handler that does something before and/or after calling it.

```go,norun
func requireAPIKey(key string, next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if subtle.ConstantTimeCompare([]byte(r.Header.Get("X-API-Key")), []byte(key)) != 1 {
			http.Error(w, "unauthorized", http.StatusUnauthorized)
			return                        // stop here: next is never called
		}
		next.ServeHTTP(w, r)              // allowed: carry on
	})
}

handler := recoverPanics(logRequests(requireAPIKey(os.Getenv("API_KEY"), mux)))
```

(`subtle.ConstantTimeCompare` takes the same time whether the first or the last character is wrong, so an attacker can't guess a key one character at a time by measuring response times.)

## A real server needs timeouts

`http.ListenAndServe(":8080", mux)` has **no timeouts**: a client that opens connections and sends nothing can hold them open forever. Production servers set them:

```go,norun
srv := &http.Server{
	Addr:              ":8080",
	Handler:           handler,
	ReadHeaderTimeout: 5 * time.Second,
	ReadTimeout:       10 * time.Second,
	WriteTimeout:      30 * time.Second,
	IdleTimeout:       time.Minute,
}
```

## Graceful shutdown

When Docker, systemd or Kubernetes stops a service, it sends **SIGTERM** and waits a few seconds. A good service stops accepting new connections and finishes the requests in progress:

```go,norun
ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
defer stop()

go func() {
	if err := srv.ListenAndServe(); err != nil && !errors.Is(err, http.ErrServerClosed) {
		log.Fatal(err)
	}
}()

<-ctx.Done()                                       // wait for Ctrl+C or SIGTERM
shutdownCtx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
defer cancel()
srv.Shutdown(shutdownCtx)                          // finish in-flight requests, then return
```

## Every request has a context

`r.Context()` is cancelled when the client disconnects or the server shuts down. Pass it to everything slow (database queries, outgoing HTTP calls), so abandoned work stops:

```go,norun
rows, err := db.QueryContext(r.Context(), "SELECT ...")
req, _ := http.NewRequestWithContext(r.Context(), "GET", "https://api.example.com/x", nil)
```

@@quiz
With Go 1.22+ routing, a `DELETE /devices/7` request arrives, but only `GET /devices/{id}` is registered. What happens?
- 404 Not Found
* 405 Method Not Allowed, automatically
- The GET handler runs
- The server panics
? The path matches a pattern, but not with that method.

@@quiz
A handler writes the body, then calls `w.WriteHeader(201)`. What status does the client get?
* 200, because writing the body already sent the headers with status 200
- 201
- 500
- It depends on the client
? Order: headers, then WriteHeader, then body. Go logs "superfluous WriteHeader call" when you get it wrong.

@@quiz
What is middleware in a Go HTTP service?
- A database layer
* A function that wraps an http.Handler to run code before and/or after it
- A separate proxy server
- A JSON library
? `func(next http.Handler) http.Handler`: logging, auth, recovery and so on, applied to every route.

@@quiz
Why use `signal.NotifyContext` and `srv.Shutdown`?
- To restart the server automatically
* So on Ctrl+C or SIGTERM the server stops accepting connections and lets running requests finish
- To make the server faster
- So the server ignores signals
? Without it, a deployment cuts off requests halfway through.

@@flashcards
`mux.HandleFunc("GET /x/{id}", h)` :: Go 1.22+ route with a method and a wildcard
`r.PathValue("id")` :: the value of a path wildcard
`{path...}` :: a wildcard matching the rest of the path
`httptest.NewRecorder()` :: an in-memory ResponseWriter for tests
`httptest.NewRequest(method, url, body)` :: a request for calling a handler directly
`w.WriteHeader(status)` :: send the status (after headers, before the body)
`json.NewDecoder(r.Body).Decode(&v)` :: read a JSON request body
`DisallowUnknownFields()` :: reject unexpected JSON fields
`io.LimitReader` :: cap how much of the body is read
middleware :: `func(next http.Handler) http.Handler`
`http.Server` timeouts :: ReadHeaderTimeout, ReadTimeout, WriteTimeout, IdleTimeout
`srv.Shutdown(ctx)` :: graceful shutdown
`r.Context()` :: cancelled when the client goes away

@@exercise Routes with path values
Register two routes on the mux: `GET /hello/{name}` answers `Hello, <name>!`, and `GET /ports/{port}` answers `port <n>` when the port is a whole number from 1 to 65535, or a **400** with `http.Error(w, "bad port", http.StatusBadRequest)` otherwise. `main` sends the requests and prints the results.
@@starter
package main

import (
	"fmt"
	"net/http"
	"net/http/httptest"
	"strconv"
	"strings"
)

func newMux() *http.ServeMux {
	mux := http.NewServeMux()
	// TODO: the two routes
	_ = strconv.Atoi
	return mux
}

func main() {
	mux := newMux()
	requests := [][2]string{
		{"GET", "/hello/aiko"}, {"GET", "/ports/443"}, {"GET", "/ports/http"},
		{"GET", "/ports/70000"}, {"POST", "/hello/aiko"},
	}
	for _, req := range requests {
		rec := httptest.NewRecorder()
		mux.ServeHTTP(rec, httptest.NewRequest(req[0], req[1], nil))
		fmt.Printf("%s %s -> %d %s\n", req[0], req[1], rec.Code, strings.TrimSpace(rec.Body.String()))
	}
}
@@expected
GET /hello/aiko -> 200 Hello, aiko!
GET /ports/443 -> 200 port 443
GET /ports/http -> 400 bad port
GET /ports/70000 -> 400 bad port
POST /hello/aiko -> 405 Method Not Allowed
@@requires
PathValue
@@hint
`mux.HandleFunc("GET /hello/{name}", func(w http.ResponseWriter, r *http.Request) { fmt.Fprintf(w, "Hello, %s!", r.PathValue("name")) })`. For the port: `n, err := strconv.Atoi(r.PathValue("port")); if err != nil || n < 1 || n > 65535 { http.Error(w, "bad port", http.StatusBadRequest); return }`.
@@solution
package main

import (
	"fmt"
	"net/http"
	"net/http/httptest"
	"strconv"
	"strings"
)

func newMux() *http.ServeMux {
	mux := http.NewServeMux()
	mux.HandleFunc("GET /hello/{name}", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintf(w, "Hello, %s!", r.PathValue("name"))
	})
	mux.HandleFunc("GET /ports/{port}", func(w http.ResponseWriter, r *http.Request) {
		n, err := strconv.Atoi(r.PathValue("port"))
		if err != nil || n < 1 || n > 65535 {
			http.Error(w, "bad port", http.StatusBadRequest)
			return
		}
		fmt.Fprintf(w, "port %d", n)
	})
	return mux
}

func main() {
	mux := newMux()
	requests := [][2]string{
		{"GET", "/hello/aiko"}, {"GET", "/ports/443"}, {"GET", "/ports/http"},
		{"GET", "/ports/70000"}, {"POST", "/hello/aiko"},
	}
	for _, req := range requests {
		rec := httptest.NewRecorder()
		mux.ServeHTTP(rec, httptest.NewRequest(req[0], req[1], nil))
		fmt.Printf("%s %s -> %d %s\n", req[0], req[1], rec.Code, strings.TrimSpace(rec.Body.String()))
	}
}

@@exercise Logging middleware
Write `logging(next http.Handler) http.Handler` that, **after** the request is handled, prints `<method> <path> <status>`. To know the status, wrap the `ResponseWriter` in `statusRecorder` (already declared) and finish its `WriteHeader` method. Remember: a handler that never calls `WriteHeader` sent **200**.
@@starter
package main

import (
	"fmt"
	"net/http"
	"net/http/httptest"
)

type statusRecorder struct {
	http.ResponseWriter
	status int
}

func (s *statusRecorder) WriteHeader(code int) {
	// TODO: remember the code, then pass it on
}

func logging(next http.Handler) http.Handler {
	// TODO
	return next
}

func main() {
	mux := http.NewServeMux()
	mux.HandleFunc("GET /ok", func(w http.ResponseWriter, r *http.Request) { fmt.Fprint(w, "fine") })
	mux.HandleFunc("GET /teapot", func(w http.ResponseWriter, r *http.Request) { w.WriteHeader(http.StatusTeapot) })
	h := logging(mux)
	for _, path := range []string{"/ok", "/teapot", "/nope"} {
		h.ServeHTTP(httptest.NewRecorder(), httptest.NewRequest("GET", path, nil))
	}
}
@@expected
GET /ok 200
GET /teapot 418
GET /nope 404
@@requires
http.HandlerFunc
next.ServeHTTP
@@hint
`WriteHeader`: `s.status = code; s.ResponseWriter.WriteHeader(code)`. `logging`: `return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) { rec := &statusRecorder{ResponseWriter: w, status: http.StatusOK}; next.ServeHTTP(rec, r); fmt.Println(r.Method, r.URL.Path, rec.status) })`.
@@solution
package main

import (
	"fmt"
	"net/http"
	"net/http/httptest"
)

type statusRecorder struct {
	http.ResponseWriter
	status int
}

func (s *statusRecorder) WriteHeader(code int) {
	s.status = code
	s.ResponseWriter.WriteHeader(code)
}

func logging(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		rec := &statusRecorder{ResponseWriter: w, status: http.StatusOK}
		next.ServeHTTP(rec, r)
		fmt.Println(r.Method, r.URL.Path, rec.status)
	})
}

func main() {
	mux := http.NewServeMux()
	mux.HandleFunc("GET /ok", func(w http.ResponseWriter, r *http.Request) { fmt.Fprint(w, "fine") })
	mux.HandleFunc("GET /teapot", func(w http.ResponseWriter, r *http.Request) { w.WriteHeader(http.StatusTeapot) })
	h := logging(mux)
	for _, path := range []string{"/ok", "/teapot", "/nope"} {
		h.ServeHTTP(httptest.NewRecorder(), httptest.NewRequest("GET", path, nil))
	}
}

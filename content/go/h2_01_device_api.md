@@lesson g9-device-api Device Inventory REST API
@@text
Build a production-shaped REST API in Go with only the standard library: the kind of internal service an IT team runs to keep track of every switch, server, laptop and printer, and that scripts, dashboards and other services call.

```text
$ go run .
level=INFO msg=listening url=http://127.0.0.1:8080 data=devices.json

$ curl -X POST localhost:8080/devices -d '{"hostname":"TK-SW-CORE","ip":"10.0.0.2","kind":"switch"}'
{"id":1,"hostname":"tk-sw-core","ip":"10.0.0.2","kind":"switch"}

$ curl -X POST localhost:8080/devices -d '{"hostname":"bad host","ip":"10.0.0.999","kind":"toaster"}'
{"error":"invalid device: hostname \"bad host\" must be ...","problems":[...3 problems...]}

$ curl 'localhost:8080/devices?kind=switch&q=tk'
[{"id":1,"hostname":"tk-sw-core","ip":"10.0.0.2","kind":"switch"}]
```

(On Windows PowerShell, use `curl.exe`, or `Invoke-RestMethod`.)

## The API

| Request | Response |
|---|---|
| `GET /healthz` | 200 `{"status":"ok"}` |
| `GET /devices?kind=&owner=&q=` | 200, the matching devices sorted by hostname |
| `POST /devices` | 201 with the new device and a `Location` header; 422 invalid; 409 hostname taken; 400 bad JSON |
| `GET /devices/{id}` | 200, or 404 |
| `PUT /devices/{id}` | 200 with the updated device; 404, 409, 422 or 400 |
| `DELETE /devices/{id}` | 204, or 404 |

## How it's built

| File | Job |
|---|---|
| `device.go` | the domain: the `Device` type, `Normalize`, `Validate`, and the error values |
| `store.go` | the `Store` interface and `MemoryStore`: a map behind a `sync.RWMutex`, saved to a JSON file with an atomic rename |
| `api.go` | routes (Go 1.22 patterns), handlers, JSON helpers, error → status mapping, and middleware for logging and panics |
| `api_test.go` | 15 tests with `httptest`, including a fake store and 50 concurrent requests |
| `main.go` | configuration from flags and environment variables, server timeouts, graceful shutdown |

## Your tasks

Four functions `panic("TODO: not done yet")`. The comment above each one is its specification:

1. `Validate` (device.go): collect *every* problem, not just the first. A client that sends three mistakes should hear about all three.
2. `Filter.matches` (store.go): the search filters.
3. `statusFor` (api.go): `errors.As` for the validation error type, `errors.Is` for the others.
4. `update` (api.go): the PUT handler. `create` right above it does almost the same thing.

```text
go test ./...                 all the tests
go test -run TestValidate -v  just one
go test -race ./...           with the race detector (needs a C compiler on Windows; works out of the box on Linux and macOS)
go run .                      the server
```

The first unfinished function panics and stops the test run, so work through them in order: `Validate`, `matches`, `statusFor`, then `update`.
@@project device_api Device Inventory REST API
Finish the four functions that panic with `TODO` until `go test ./...` passes, then `go run .` and try it with curl.
@@file go.mod
module deviceapi

go 1.22
@@file device.go
package main

import (
	"errors"
	"fmt"
	"net/netip"
	"regexp"
	"strings"
)

// Device is one piece of network-connected equipment. The struct tags are the JSON field names.
type Device struct {
	ID       int    `json:"id"`
	Hostname string `json:"hostname"`
	IP       string `json:"ip"`
	Kind     string `json:"kind"`
	Owner    string `json:"owner,omitempty"`
}

func (d Device) String() string {
	return fmt.Sprintf("#%d %s (%s, %s)", d.ID, d.Hostname, d.Kind, d.IP)
}

// Errors the store returns. Callers check them with errors.Is, even when they're wrapped.
var (
	ErrNotFound  = errors.New("not found")
	ErrDuplicate = errors.New("already exists")
)

// ValidationError lists every problem with a device, so a client can fix them all at once.
type ValidationError struct {
	Problems []string
}

func (e *ValidationError) Error() string {
	return "invalid device: " + strings.Join(e.Problems, "; ")
}

var hostnamePattern = regexp.MustCompile(`^[a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?$`)

var kinds = map[string]bool{"desktop": true, "laptop": true, "printer": true, "server": true, "switch": true}

// Normalize tidies what a client sent: spaces trimmed everywhere, the hostname and kind
// in lower case, and a valid IP address in its standard form (2001:DB8::1 -> 2001:db8::1).
func Normalize(d Device) Device {
	d.Hostname = strings.ToLower(strings.TrimSpace(d.Hostname))
	d.Kind = strings.ToLower(strings.TrimSpace(d.Kind))
	d.Owner = strings.TrimSpace(d.Owner)
	d.IP = strings.TrimSpace(d.IP)
	if addr, err := netip.ParseAddr(d.IP); err == nil {
		d.IP = addr.String()
	}
	return d
}

// Validate returns nil for a valid (normalized) device, or a *ValidationError listing every
// problem, in this order and with exactly these messages:
//
//	hostname "<h>" must be 1-63 lower-case letters, digits or dashes, and not start or end with a dash
//	ip "<ip>" is not an IP address
//	kind "<k>" must be one of: desktop, laptop, printer, server, switch
//
// Use hostnamePattern, netip.ParseAddr and the kinds map.
func Validate(d Device) error {
	panic("TODO: not done yet")
}
@@file store.go
package main

import (
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"slices"
	"strings"
	"sync"
)

// Filter selects devices. Empty fields match everything.
type Filter struct {
	Kind  string // exactly this kind
	Owner string // exactly this owner, ignoring case
	Query string // part of the hostname, ignoring case
}

// Store is what the API needs from storage. The API only knows this interface, so the
// tests (or a future PostgreSQL version) can give it something else.
type Store interface {
	List(f Filter) []Device
	Get(id int) (Device, error)
	Create(d Device) (Device, error)
	Update(d Device) (Device, error)
	Delete(id int) error
}

// MemoryStore keeps the devices in a map, guarded by a mutex because every HTTP request
// runs in its own goroutine. With a path, it saves to a JSON file after every change.
type MemoryStore struct {
	mu      sync.RWMutex
	devices map[int]Device
	nextID  int
	path    string
}

func NewMemoryStore() *MemoryStore {
	return &MemoryStore{devices: map[int]Device{}, nextID: 1}
}

// OpenFileStore loads the devices from path, if the file exists, and saves there from now on.
func OpenFileStore(path string) (*MemoryStore, error) {
	s := NewMemoryStore()
	s.path = path
	data, err := os.ReadFile(path)
	if errors.Is(err, os.ErrNotExist) {
		return s, nil
	}
	if err != nil {
		return nil, err
	}
	var list []Device
	if err := json.Unmarshal(data, &list); err != nil {
		return nil, fmt.Errorf("reading %s: %w", path, err)
	}
	for _, d := range list {
		s.devices[d.ID] = d
		if d.ID >= s.nextID {
			s.nextID = d.ID + 1
		}
	}
	return s, nil
}

// matches reports whether d passes every filter that is set: Kind equal, Owner equal ignoring
// case (strings.EqualFold), and Query contained in the hostname ignoring case.
func (f Filter) matches(d Device) bool {
	panic("TODO: not done yet")
}

// List returns the matching devices sorted by hostname (an empty slice, never nil, so the
// JSON is [] and not null).
func (s *MemoryStore) List(f Filter) []Device {
	s.mu.RLock()
	defer s.mu.RUnlock()
	return s.sorted(f)
}

func (s *MemoryStore) sorted(f Filter) []Device {
	out := []Device{}
	for _, d := range s.devices {
		if f.matches(d) {
			out = append(out, d)
		}
	}
	slices.SortFunc(out, func(a, b Device) int { return strings.Compare(a.Hostname, b.Hostname) })
	return out
}

func (s *MemoryStore) Get(id int) (Device, error) {
	s.mu.RLock()
	defer s.mu.RUnlock()
	d, ok := s.devices[id]
	if !ok {
		return Device{}, fmt.Errorf("device %d: %w", id, ErrNotFound)
	}
	return d, nil
}

// hostnameTaken reports whether another device (not id) has this hostname. Caller holds the lock.
func (s *MemoryStore) hostnameTaken(hostname string, id int) bool {
	for _, d := range s.devices {
		if d.Hostname == hostname && d.ID != id {
			return true
		}
	}
	return false
}

func (s *MemoryStore) Create(d Device) (Device, error) {
	s.mu.Lock()
	defer s.mu.Unlock()
	if s.hostnameTaken(d.Hostname, 0) {
		return Device{}, fmt.Errorf("hostname %s: %w", d.Hostname, ErrDuplicate)
	}
	d.ID = s.nextID
	s.devices[d.ID] = d
	if err := s.save(); err != nil {
		delete(s.devices, d.ID)
		return Device{}, err
	}
	s.nextID++
	return d, nil
}

func (s *MemoryStore) Update(d Device) (Device, error) {
	s.mu.Lock()
	defer s.mu.Unlock()
	old, ok := s.devices[d.ID]
	if !ok {
		return Device{}, fmt.Errorf("device %d: %w", d.ID, ErrNotFound)
	}
	if s.hostnameTaken(d.Hostname, d.ID) {
		return Device{}, fmt.Errorf("hostname %s: %w", d.Hostname, ErrDuplicate)
	}
	s.devices[d.ID] = d
	if err := s.save(); err != nil {
		s.devices[d.ID] = old
		return Device{}, err
	}
	return d, nil
}

func (s *MemoryStore) Delete(id int) error {
	s.mu.Lock()
	defer s.mu.Unlock()
	old, ok := s.devices[id]
	if !ok {
		return fmt.Errorf("device %d: %w", id, ErrNotFound)
	}
	delete(s.devices, id)
	if err := s.save(); err != nil {
		s.devices[id] = old
		return err
	}
	return nil
}

// save writes every device to a temporary file and renames it over the real one, so a crash
// can't leave a half-written file. The caller holds the lock.
func (s *MemoryStore) save() error {
	if s.path == "" {
		return nil
	}
	data, err := json.MarshalIndent(s.sorted(Filter{}), "", "  ")
	if err != nil {
		return err
	}
	tmp := s.path + ".tmp"
	if err := os.WriteFile(tmp, data, 0o644); err != nil {
		return err
	}
	return os.Rename(tmp, s.path)
}
@@file api.go
package main

import (
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"log/slog"
	"net/http"
	"strconv"
	"time"
)

// API holds what the handlers need. Nothing is global, so tests can build as many as they like.
type API struct {
	store Store
	log   *slog.Logger
}

// NewAPI returns the whole HTTP application: the routes, wrapped in the middleware.
func NewAPI(store Store, log *slog.Logger) http.Handler {
	a := &API{store: store, log: log}
	mux := http.NewServeMux()
	// Go 1.22+ patterns: a method, a path, and {wildcards} read with r.PathValue.
	mux.HandleFunc("GET /healthz", a.health)
	mux.HandleFunc("GET /devices", a.list)
	mux.HandleFunc("POST /devices", a.create)
	mux.HandleFunc("GET /devices/{id}", a.get)
	mux.HandleFunc("PUT /devices/{id}", a.update)
	mux.HandleFunc("DELETE /devices/{id}", a.delete)
	return a.recoverPanics(a.logRequests(mux))
}

func (a *API) health(w http.ResponseWriter, r *http.Request) {
	writeJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}

// GET /devices?kind=laptop&owner=aiko&q=tk-
func (a *API) list(w http.ResponseWriter, r *http.Request) {
	q := r.URL.Query()
	writeJSON(w, http.StatusOK, a.store.List(Filter{Kind: q.Get("kind"), Owner: q.Get("owner"), Query: q.Get("q")}))
}

func (a *API) get(w http.ResponseWriter, r *http.Request) {
	id, err := pathID(r)
	if err != nil {
		a.fail(w, err)
		return
	}
	d, err := a.store.Get(id)
	if err != nil {
		a.fail(w, err)
		return
	}
	writeJSON(w, http.StatusOK, d)
}

// POST /devices with a JSON device (without id): 201 Created, the device with its new id,
// and a Location header pointing at it.
func (a *API) create(w http.ResponseWriter, r *http.Request) {
	var d Device
	if err := decodeJSON(r, &d); err != nil {
		a.fail(w, err)
		return
	}
	d = Normalize(d)
	if err := Validate(d); err != nil {
		a.fail(w, err)
		return
	}
	created, err := a.store.Create(d)
	if err != nil {
		a.fail(w, err)
		return
	}
	w.Header().Set("Location", fmt.Sprintf("/devices/%d", created.ID))
	writeJSON(w, http.StatusCreated, created)
}

// PUT /devices/{id} with a JSON device: replaces the device, keeping the id from the URL
// (any id in the body is ignored). Same steps as create: decode, normalize, validate, then
// a.store.Update. 200 OK with the updated device. Every error goes through a.fail.
func (a *API) update(w http.ResponseWriter, r *http.Request) {
	panic("TODO: not done yet")
}

// DELETE /devices/{id}: 204 No Content.
func (a *API) delete(w http.ResponseWriter, r *http.Request) {
	id, err := pathID(r)
	if err != nil {
		a.fail(w, err)
		return
	}
	if err := a.store.Delete(id); err != nil {
		a.fail(w, err)
		return
	}
	w.WriteHeader(http.StatusNoContent)
}

// errBadRequest marks errors caused by the request itself (bad JSON, a bad id).
var errBadRequest = errors.New("bad request")

func pathID(r *http.Request) (int, error) {
	id, err := strconv.Atoi(r.PathValue("id"))
	if err != nil || id < 1 {
		return 0, fmt.Errorf("%w: the id must be a positive whole number", errBadRequest)
	}
	return id, nil
}

// decodeJSON reads one JSON object from the body: at most 1 MB, and no unknown fields
// (a typo like "hostnme" is an error, not silently ignored).
func decodeJSON(r *http.Request, v any) error {
	dec := json.NewDecoder(io.LimitReader(r.Body, 1<<20))
	dec.DisallowUnknownFields()
	if err := dec.Decode(v); err != nil {
		return fmt.Errorf("%w: invalid JSON: %v", errBadRequest, err)
	}
	return nil
}

// statusFor chooses the HTTP status for an error, looking through wrapping:
// a *ValidationError -> 422 Unprocessable Entity, errBadRequest -> 400, ErrNotFound -> 404,
// ErrDuplicate -> 409 Conflict, anything else -> 500. Use errors.As and errors.Is.
func statusFor(err error) int {
	panic("TODO: not done yet")
}

// fail sends an error as JSON: {"error": "..."}, plus "problems" for validation errors.
// Unexpected errors are logged, and the client only gets a generic message: internal details
// (file paths, SQL) must never leak out.
func (a *API) fail(w http.ResponseWriter, err error) {
	status := statusFor(err)
	body := map[string]any{"error": err.Error()}
	var verr *ValidationError
	if errors.As(err, &verr) {
		body["problems"] = verr.Problems
	}
	if status == http.StatusInternalServerError {
		a.log.Error("internal error", "err", err)
		body = map[string]any{"error": "internal server error"}
	}
	writeJSON(w, status, body)
}

func writeJSON(w http.ResponseWriter, status int, v any) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)
	_ = json.NewEncoder(w).Encode(v) // the client went away; nothing useful to do
}

// statusRecorder remembers the status a handler sent, for the request log.
type statusRecorder struct {
	http.ResponseWriter
	status int
}

func (s *statusRecorder) WriteHeader(code int) {
	s.status = code
	s.ResponseWriter.WriteHeader(code)
}

// logRequests is middleware: it wraps a handler and logs every request after it's handled.
func (a *API) logRequests(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		start := time.Now()
		rec := &statusRecorder{ResponseWriter: w, status: http.StatusOK}
		next.ServeHTTP(rec, r)
		a.log.Info("request", "method", r.Method, "path", r.URL.Path, "status", rec.status,
			"duration", time.Since(start))
	})
}

// recoverPanics turns a panic in a handler into a 500 response instead of a dropped connection.
func (a *API) recoverPanics(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		defer func() {
			if v := recover(); v != nil {
				a.log.Error("panic", "value", v, "path", r.URL.Path)
				writeJSON(w, http.StatusInternalServerError, map[string]string{"error": "internal server error"})
			}
		}()
		next.ServeHTTP(w, r)
	})
}
@@file api_test.go
package main

import (
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"log/slog"
	"net/http"
	"net/http/httptest"
	"path/filepath"
	"strings"
	"sync"
	"testing"
)

// ---- helpers ----

func newTestAPI(store Store) http.Handler {
	return NewAPI(store, slog.New(slog.NewTextHandler(io.Discard, nil)))
}

// call sends one request straight to the handler (no network) and returns the recorded response.
func call(h http.Handler, method, path, body string) *httptest.ResponseRecorder {
	var r io.Reader
	if body != "" {
		r = strings.NewReader(body)
	}
	rec := httptest.NewRecorder()
	h.ServeHTTP(rec, httptest.NewRequest(method, path, r))
	return rec
}

func decode[T any](t *testing.T, rec *httptest.ResponseRecorder) T {
	t.Helper()
	var v T
	if err := json.Unmarshal(rec.Body.Bytes(), &v); err != nil {
		t.Fatalf("response is not JSON: %v\n%s", err, rec.Body.String())
	}
	return v
}

func wantStatus(t *testing.T, rec *httptest.ResponseRecorder, want int) {
	t.Helper()
	if rec.Code != want {
		t.Fatalf("status %d, want %d; body: %s", rec.Code, want, strings.TrimSpace(rec.Body.String()))
	}
}

const laptop = `{"hostname": "tk-lt-001", "ip": "10.0.1.21", "kind": "laptop", "owner": "aiko"}`

func seeded(t *testing.T) http.Handler {
	t.Helper()
	h := newTestAPI(NewMemoryStore())
	for _, body := range []string{
		laptop,
		`{"hostname": "tk-sw-core", "ip": "10.0.0.2", "kind": "switch"}`,
		`{"hostname": "os-lt-007", "ip": "10.1.1.7", "kind": "laptop", "owner": "Ken"}`,
		`{"hostname": "tk-prn-3f", "ip": "10.0.3.50", "kind": "printer"}`,
	} {
		wantStatus(t, call(h, "POST", "/devices", body), http.StatusCreated)
	}
	return h
}

func hostnames(devices []Device) string {
	var names []string
	for _, d := range devices {
		names = append(names, d.Hostname)
	}
	return strings.Join(names, ",")
}

// ---- the rules ----

func TestNormalize(t *testing.T) {
	got := Normalize(Device{Hostname: "  TK-LT-001 ", IP: " 2001:DB8::0001 ", Kind: "Laptop ", Owner: " aiko "})
	want := Device{Hostname: "tk-lt-001", IP: "2001:db8::1", Kind: "laptop", Owner: "aiko"}
	if got != want {
		t.Fatalf("got %+v, want %+v", got, want)
	}
}

func TestValidate(t *testing.T) {
	ok := Device{Hostname: "tk-lt-001", IP: "10.0.1.21", Kind: "laptop"}
	tests := []struct {
		name   string
		change func(d *Device)
		want   []string
	}{
		{"valid", func(d *Device) {}, nil},
		{"one-letter hostname", func(d *Device) { d.Hostname = "a" }, nil},
		{"ipv6", func(d *Device) { d.IP = "2001:db8::1" }, nil},
		{"hostname with a dot", func(d *Device) { d.Hostname = "tk.lt" },
			[]string{`hostname "tk.lt" must be 1-63 lower-case letters, digits or dashes, and not start or end with a dash`}},
		{"hostname ending with a dash", func(d *Device) { d.Hostname = "tk-" },
			[]string{`hostname "tk-" must be 1-63 lower-case letters, digits or dashes, and not start or end with a dash`}},
		{"bad ip", func(d *Device) { d.IP = "10.0.1.300" }, []string{`ip "10.0.1.300" is not an IP address`}},
		{"bad kind", func(d *Device) { d.Kind = "toaster" },
			[]string{`kind "toaster" must be one of: desktop, laptop, printer, server, switch`}},
		{"everything wrong", func(d *Device) { d.Hostname = ""; d.IP = ""; d.Kind = "" }, []string{
			`hostname "" must be 1-63 lower-case letters, digits or dashes, and not start or end with a dash`,
			`ip "" is not an IP address`,
			`kind "" must be one of: desktop, laptop, printer, server, switch`,
		}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			d := ok
			tt.change(&d)
			err := Validate(d)
			if tt.want == nil {
				if err != nil {
					t.Fatalf("want no error, got %v", err)
				}
				return
			}
			var verr *ValidationError
			if !errors.As(err, &verr) {
				t.Fatalf("want a *ValidationError, got %v", err)
			}
			if strings.Join(verr.Problems, "|") != strings.Join(tt.want, "|") {
				t.Fatalf("problems:\n got %q\nwant %q", verr.Problems, tt.want)
			}
		})
	}
}

func TestFilterMatches(t *testing.T) {
	d := Device{Hostname: "tk-lt-001", Kind: "laptop", Owner: "Aiko"}
	tests := []struct {
		f    Filter
		want bool
	}{
		{Filter{}, true},
		{Filter{Kind: "laptop"}, true},
		{Filter{Kind: "switch"}, false},
		{Filter{Owner: "aiko"}, true},
		{Filter{Owner: "ken"}, false},
		{Filter{Query: "LT-0"}, true},
		{Filter{Query: "sw"}, false},
		{Filter{Kind: "laptop", Owner: "AIKO", Query: "tk"}, true},
		{Filter{Kind: "laptop", Owner: "aiko", Query: "os-"}, false},
	}
	for _, tt := range tests {
		if got := d.matchesFilter(tt.f); got != tt.want {
			t.Errorf("%+v: got %v, want %v", tt.f, got, tt.want)
		}
	}
}

// matchesFilter reads better in the table above.
func (d Device) matchesFilter(f Filter) bool { return f.matches(d) }

func TestStatusFor(t *testing.T) {
	tests := []struct {
		err  error
		want int
	}{
		{&ValidationError{Problems: []string{"x"}}, 422},
		{fmt.Errorf("saving: %w", &ValidationError{}), 422},
		{fmt.Errorf("%w: bad id", errBadRequest), 400},
		{fmt.Errorf("device 7: %w", ErrNotFound), 404},
		{fmt.Errorf("hostname x: %w", ErrDuplicate), 409},
		{errors.New("disk full"), 500},
	}
	for _, tt := range tests {
		if got := statusFor(tt.err); got != tt.want {
			t.Errorf("statusFor(%v) = %d, want %d", tt.err, got, tt.want)
		}
	}
}

// ---- the HTTP API ----

func TestHealth(t *testing.T) {
	rec := call(newTestAPI(NewMemoryStore()), "GET", "/healthz", "")
	wantStatus(t, rec, 200)
	if decode[map[string]string](t, rec)["status"] != "ok" {
		t.Fatal(rec.Body.String())
	}
}

func TestCreateAndGet(t *testing.T) {
	h := newTestAPI(NewMemoryStore())
	rec := call(h, "POST", "/devices", `{"hostname": " TK-LT-001", "ip": "10.0.1.21", "kind": "Laptop", "owner": "aiko"}`)
	wantStatus(t, rec, http.StatusCreated)
	if loc := rec.Header().Get("Location"); loc != "/devices/1" {
		t.Fatalf("Location %q", loc)
	}
	if ct := rec.Header().Get("Content-Type"); ct != "application/json" {
		t.Fatalf("Content-Type %q", ct)
	}
	want := Device{ID: 1, Hostname: "tk-lt-001", IP: "10.0.1.21", Kind: "laptop", Owner: "aiko"}
	if got := decode[Device](t, rec); got != want {
		t.Fatalf("created %+v", got)
	}
	rec = call(h, "GET", "/devices/1", "")
	wantStatus(t, rec, 200)
	if got := decode[Device](t, rec); got != want {
		t.Fatalf("got %+v", got)
	}
}

func TestCreateErrors(t *testing.T) {
	h := seeded(t)
	rec := call(h, "POST", "/devices", `{"hostname": "bad host", "ip": "10.0.0.999", "kind": "laptop"}`)
	wantStatus(t, rec, http.StatusUnprocessableEntity)
	body := decode[struct {
		Error    string   `json:"error"`
		Problems []string `json:"problems"`
	}](t, rec)
	if len(body.Problems) != 2 || !strings.HasPrefix(body.Error, "invalid device: ") {
		t.Fatalf("body %+v", body)
	}

	wantStatus(t, call(h, "POST", "/devices", `{"hostname": "TK-LT-001", "ip": "10.0.9.9", "kind": "laptop"}`), http.StatusConflict)
	wantStatus(t, call(h, "POST", "/devices", `{"hostname": "x", `), http.StatusBadRequest)
	wantStatus(t, call(h, "POST", "/devices", `{"hostnme": "typo", "ip": "10.0.0.1", "kind": "laptop"}`), http.StatusBadRequest)
	wantStatus(t, call(h, "POST", "/devices", ""), http.StatusBadRequest)
	if n := len(decode[[]Device](t, call(h, "GET", "/devices", ""))); n != 4 {
		t.Fatalf("%d devices after the failed requests, want 4", n)
	}
}

func TestGetErrors(t *testing.T) {
	h := seeded(t)
	wantStatus(t, call(h, "GET", "/devices/99", ""), http.StatusNotFound)
	wantStatus(t, call(h, "GET", "/devices/abc", ""), http.StatusBadRequest)
	wantStatus(t, call(h, "GET", "/devices/-1", ""), http.StatusBadRequest)
	wantStatus(t, call(h, "PATCH", "/devices/1", "{}"), http.StatusMethodNotAllowed)
}

func TestListAndFilters(t *testing.T) {
	h := seeded(t)
	tests := map[string]string{
		"/devices":                         "os-lt-007,tk-lt-001,tk-prn-3f,tk-sw-core",
		"/devices?kind=laptop":             "os-lt-007,tk-lt-001",
		"/devices?owner=ken":               "os-lt-007",
		"/devices?q=TK-":                   "tk-lt-001,tk-prn-3f,tk-sw-core",
		"/devices?kind=laptop&q=tk":        "tk-lt-001",
		"/devices?kind=server":             "",
		"/devices?kind=printer&owner=aiko": "",
	}
	for path, want := range tests {
		rec := call(h, "GET", path, "")
		wantStatus(t, rec, 200)
		if got := hostnames(decode[[]Device](t, rec)); got != want {
			t.Errorf("%s: got %q, want %q", path, got, want)
		}
	}
	if body := strings.TrimSpace(call(h, "GET", "/devices?kind=server", "").Body.String()); body != "[]" {
		t.Errorf("an empty list must be [], got %s", body)
	}
}

func TestUpdate(t *testing.T) {
	h := seeded(t)
	rec := call(h, "PUT", "/devices/1", `{"id": 99, "hostname": "tk-lt-001", "ip": "10.0.1.22", "kind": "Laptop", "owner": "yuki"}`)
	wantStatus(t, rec, 200)
	want := Device{ID: 1, Hostname: "tk-lt-001", IP: "10.0.1.22", Kind: "laptop", Owner: "yuki"}
	if got := decode[Device](t, rec); got != want {
		t.Fatalf("updated %+v", got)
	}
	if got := decode[Device](t, call(h, "GET", "/devices/1", "")); got != want {
		t.Fatalf("stored %+v", got)
	}
	wantStatus(t, call(h, "PUT", "/devices/1", `{"hostname": "tk-sw-core", "ip": "10.0.1.22", "kind": "laptop"}`), http.StatusConflict)
	wantStatus(t, call(h, "PUT", "/devices/1", `{"hostname": "tk-lt-001", "ip": "nope", "kind": "laptop"}`), http.StatusUnprocessableEntity)
	wantStatus(t, call(h, "PUT", "/devices/42", `{"hostname": "new-one", "ip": "10.0.0.42", "kind": "server"}`), http.StatusNotFound)
	wantStatus(t, call(h, "PUT", "/devices/x", laptop), http.StatusBadRequest)
	wantStatus(t, call(h, "PUT", "/devices/1", `not json`), http.StatusBadRequest)
}

func TestDelete(t *testing.T) {
	h := seeded(t)
	rec := call(h, "DELETE", "/devices/2", "")
	wantStatus(t, rec, http.StatusNoContent)
	if rec.Body.Len() != 0 {
		t.Fatalf("204 must have no body, got %q", rec.Body.String())
	}
	wantStatus(t, call(h, "GET", "/devices/2", ""), http.StatusNotFound)
	wantStatus(t, call(h, "DELETE", "/devices/2", ""), http.StatusNotFound)
}

// panickyStore is a fake Store whose List panics, to test the recovery middleware.
type panickyStore struct{ Store }

func (panickyStore) List(Filter) []Device { panic("database exploded") }

func TestPanicsBecome500(t *testing.T) {
	h := newTestAPI(panickyStore{NewMemoryStore()})
	rec := call(h, "GET", "/devices", "")
	wantStatus(t, rec, http.StatusInternalServerError)
	if strings.Contains(rec.Body.String(), "exploded") {
		t.Fatal("internal details leaked to the client")
	}
	wantStatus(t, call(h, "GET", "/healthz", ""), 200) // the server still works
}

func TestConcurrentCreates(t *testing.T) {
	store := NewMemoryStore()
	h := newTestAPI(store)
	var wg sync.WaitGroup
	codes := make(chan int, 50)
	for i := 0; i < 50; i++ {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()
			body := fmt.Sprintf(`{"hostname": "host-%02d", "ip": "10.9.0.%d", "kind": "server"}`, i, i+1)
			codes <- call(h, "POST", "/devices", body).Code
		}(i)
	}
	wg.Wait()
	close(codes)
	for code := range codes {
		if code != http.StatusCreated {
			t.Fatalf("a create returned %d", code)
		}
	}
	ids := map[int]bool{}
	for _, d := range store.List(Filter{}) {
		ids[d.ID] = true
	}
	if len(ids) != 50 {
		t.Fatalf("%d distinct ids, want 50", len(ids))
	}
}

func TestFileStoreSurvivesRestart(t *testing.T) {
	path := filepath.Join(t.TempDir(), "devices.json")
	store, err := OpenFileStore(path)
	if err != nil {
		t.Fatal(err)
	}
	h := newTestAPI(store)
	wantStatus(t, call(h, "POST", "/devices", laptop), http.StatusCreated)
	wantStatus(t, call(h, "POST", "/devices", `{"hostname": "tk-sw-core", "ip": "10.0.0.2", "kind": "switch"}`), http.StatusCreated)
	wantStatus(t, call(h, "DELETE", "/devices/1", ""), http.StatusNoContent)

	reopened, err := OpenFileStore(path)
	if err != nil {
		t.Fatal(err)
	}
	if got := hostnames(reopened.List(Filter{})); got != "tk-sw-core" {
		t.Fatalf("after restart: %q", got)
	}
	d, err := reopened.Create(Device{Hostname: "new", IP: "10.0.0.3", Kind: "server"})
	if err != nil || d.ID != 3 {
		t.Fatalf("ids must continue after the highest: got %d, %v", d.ID, err)
	}
}
@@file main.go
// Device Inventory API: go run .     then try:  curl http://127.0.0.1:8080/devices
package main

import (
	"context"
	"errors"
	"flag"
	"log/slog"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"
)

func main() {
	// Settings: a flag, or an environment variable (what containers use), or the default.
	addr := flag.String("addr", envOr("ADDR", "127.0.0.1:8080"), "address to listen on")
	data := flag.String("data", envOr("DATA_FILE", "devices.json"), "JSON file to keep the devices in")
	flag.Parse()

	log := slog.New(slog.NewTextHandler(os.Stdout, nil))
	store, err := OpenFileStore(*data)
	if err != nil {
		log.Error("cannot open the store", "err", err)
		os.Exit(1)
	}

	// Timeouts stop slow or stuck clients from holding connections open forever.
	srv := &http.Server{
		Addr:              *addr,
		Handler:           NewAPI(store, log),
		ReadHeaderTimeout: 5 * time.Second,
		ReadTimeout:       10 * time.Second,
		WriteTimeout:      10 * time.Second,
		IdleTimeout:       time.Minute,
	}

	// Ctrl+C (or SIGTERM from Docker, systemd or Kubernetes) cancels ctx.
	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
	defer stop()

	go func() {
		log.Info("listening", "url", "http://"+*addr, "data", *data)
		if err := srv.ListenAndServe(); err != nil && !errors.Is(err, http.ErrServerClosed) {
			log.Error("server failed", "err", err)
			os.Exit(1)
		}
	}()

	<-ctx.Done()
	// Graceful shutdown: stop accepting connections, let requests in progress finish.
	log.Info("shutting down")
	shutdownCtx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()
	if err := srv.Shutdown(shutdownCtx); err != nil {
		log.Error("shutdown", "err", err)
	}
}

func envOr(key, fallback string) string {
	if v := os.Getenv(key); v != "" {
		return v
	}
	return fallback
}
@@text
## Stretch goals

- **PostgreSQL or SQLite storage**: a `SQLStore` that satisfies the same `Store` interface with `database/sql` (pgx, or modernc.org/sqlite which needs no C compiler). The API code shouldn't change at all.
- **Authentication**: middleware that checks an `Authorization: Bearer` token with `subtle.ConstantTimeCompare`, so reads are public but changes need a key.
- **Pagination**: `?limit=50&after=<hostname>` with keyset pagination, and a `next` link in the response.
- **Metrics**: count requests by route and status, and expose them at `/metrics` (by hand, or with the Prometheus client library).
- **A Dockerfile** with a multi-stage build and a distroless image, plus a `/readyz` that fails during shutdown.
- **An OpenAPI description** (`openapi.yaml`) of the API, so other teams can generate clients for it.
@@project device_api_finished Device Inventory REST API: the finished version
A complete solution to compare with yours.
@@file go.mod
module deviceapi

go 1.22
@@file device.go
package main

import (
	"errors"
	"fmt"
	"net/netip"
	"regexp"
	"strings"
)

// Device is one piece of network-connected equipment. The struct tags are the JSON field names.
type Device struct {
	ID       int    `json:"id"`
	Hostname string `json:"hostname"`
	IP       string `json:"ip"`
	Kind     string `json:"kind"`
	Owner    string `json:"owner,omitempty"`
}

func (d Device) String() string {
	return fmt.Sprintf("#%d %s (%s, %s)", d.ID, d.Hostname, d.Kind, d.IP)
}

// Errors the store returns. Callers check them with errors.Is, even when they're wrapped.
var (
	ErrNotFound  = errors.New("not found")
	ErrDuplicate = errors.New("already exists")
)

// ValidationError lists every problem with a device, so a client can fix them all at once.
type ValidationError struct {
	Problems []string
}

func (e *ValidationError) Error() string {
	return "invalid device: " + strings.Join(e.Problems, "; ")
}

var hostnamePattern = regexp.MustCompile(`^[a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?$`)

var kinds = map[string]bool{"desktop": true, "laptop": true, "printer": true, "server": true, "switch": true}

// Normalize tidies what a client sent: spaces trimmed everywhere, the hostname and kind
// in lower case, and a valid IP address in its standard form (2001:DB8::1 -> 2001:db8::1).
func Normalize(d Device) Device {
	d.Hostname = strings.ToLower(strings.TrimSpace(d.Hostname))
	d.Kind = strings.ToLower(strings.TrimSpace(d.Kind))
	d.Owner = strings.TrimSpace(d.Owner)
	d.IP = strings.TrimSpace(d.IP)
	if addr, err := netip.ParseAddr(d.IP); err == nil {
		d.IP = addr.String()
	}
	return d
}

// Validate returns nil for a valid (normalized) device, or a *ValidationError listing every
// problem, in this order and with exactly these messages:
//
//	hostname "<h>" must be 1-63 lower-case letters, digits or dashes, and not start or end with a dash
//	ip "<ip>" is not an IP address
//	kind "<k>" must be one of: desktop, laptop, printer, server, switch
//
// Use hostnamePattern, netip.ParseAddr and the kinds map.
func Validate(d Device) error {
	var problems []string
	if !hostnamePattern.MatchString(d.Hostname) {
		problems = append(problems, fmt.Sprintf("hostname %q must be 1-63 lower-case letters, digits or dashes, and not start or end with a dash", d.Hostname))
	}
	if _, err := netip.ParseAddr(d.IP); err != nil {
		problems = append(problems, fmt.Sprintf("ip %q is not an IP address", d.IP))
	}
	if !kinds[d.Kind] {
		problems = append(problems, fmt.Sprintf("kind %q must be one of: desktop, laptop, printer, server, switch", d.Kind))
	}
	if len(problems) > 0 {
		return &ValidationError{Problems: problems}
	}
	return nil
}
@@file store.go
package main

import (
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"slices"
	"strings"
	"sync"
)

// Filter selects devices. Empty fields match everything.
type Filter struct {
	Kind  string // exactly this kind
	Owner string // exactly this owner, ignoring case
	Query string // part of the hostname, ignoring case
}

// Store is what the API needs from storage. The API only knows this interface, so the
// tests (or a future PostgreSQL version) can give it something else.
type Store interface {
	List(f Filter) []Device
	Get(id int) (Device, error)
	Create(d Device) (Device, error)
	Update(d Device) (Device, error)
	Delete(id int) error
}

// MemoryStore keeps the devices in a map, guarded by a mutex because every HTTP request
// runs in its own goroutine. With a path, it saves to a JSON file after every change.
type MemoryStore struct {
	mu      sync.RWMutex
	devices map[int]Device
	nextID  int
	path    string
}

func NewMemoryStore() *MemoryStore {
	return &MemoryStore{devices: map[int]Device{}, nextID: 1}
}

// OpenFileStore loads the devices from path, if the file exists, and saves there from now on.
func OpenFileStore(path string) (*MemoryStore, error) {
	s := NewMemoryStore()
	s.path = path
	data, err := os.ReadFile(path)
	if errors.Is(err, os.ErrNotExist) {
		return s, nil
	}
	if err != nil {
		return nil, err
	}
	var list []Device
	if err := json.Unmarshal(data, &list); err != nil {
		return nil, fmt.Errorf("reading %s: %w", path, err)
	}
	for _, d := range list {
		s.devices[d.ID] = d
		if d.ID >= s.nextID {
			s.nextID = d.ID + 1
		}
	}
	return s, nil
}

// matches reports whether d passes every filter that is set: Kind equal, Owner equal ignoring
// case (strings.EqualFold), and Query contained in the hostname ignoring case.
func (f Filter) matches(d Device) bool {
	if f.Kind != "" && d.Kind != f.Kind {
		return false
	}
	if f.Owner != "" && !strings.EqualFold(d.Owner, f.Owner) {
		return false
	}
	if f.Query != "" && !strings.Contains(d.Hostname, strings.ToLower(f.Query)) {
		return false
	}
	return true
}

// List returns the matching devices sorted by hostname (an empty slice, never nil, so the
// JSON is [] and not null).
func (s *MemoryStore) List(f Filter) []Device {
	s.mu.RLock()
	defer s.mu.RUnlock()
	return s.sorted(f)
}

func (s *MemoryStore) sorted(f Filter) []Device {
	out := []Device{}
	for _, d := range s.devices {
		if f.matches(d) {
			out = append(out, d)
		}
	}
	slices.SortFunc(out, func(a, b Device) int { return strings.Compare(a.Hostname, b.Hostname) })
	return out
}

func (s *MemoryStore) Get(id int) (Device, error) {
	s.mu.RLock()
	defer s.mu.RUnlock()
	d, ok := s.devices[id]
	if !ok {
		return Device{}, fmt.Errorf("device %d: %w", id, ErrNotFound)
	}
	return d, nil
}

// hostnameTaken reports whether another device (not id) has this hostname. Caller holds the lock.
func (s *MemoryStore) hostnameTaken(hostname string, id int) bool {
	for _, d := range s.devices {
		if d.Hostname == hostname && d.ID != id {
			return true
		}
	}
	return false
}

func (s *MemoryStore) Create(d Device) (Device, error) {
	s.mu.Lock()
	defer s.mu.Unlock()
	if s.hostnameTaken(d.Hostname, 0) {
		return Device{}, fmt.Errorf("hostname %s: %w", d.Hostname, ErrDuplicate)
	}
	d.ID = s.nextID
	s.devices[d.ID] = d
	if err := s.save(); err != nil {
		delete(s.devices, d.ID)
		return Device{}, err
	}
	s.nextID++
	return d, nil
}

func (s *MemoryStore) Update(d Device) (Device, error) {
	s.mu.Lock()
	defer s.mu.Unlock()
	old, ok := s.devices[d.ID]
	if !ok {
		return Device{}, fmt.Errorf("device %d: %w", d.ID, ErrNotFound)
	}
	if s.hostnameTaken(d.Hostname, d.ID) {
		return Device{}, fmt.Errorf("hostname %s: %w", d.Hostname, ErrDuplicate)
	}
	s.devices[d.ID] = d
	if err := s.save(); err != nil {
		s.devices[d.ID] = old
		return Device{}, err
	}
	return d, nil
}

func (s *MemoryStore) Delete(id int) error {
	s.mu.Lock()
	defer s.mu.Unlock()
	old, ok := s.devices[id]
	if !ok {
		return fmt.Errorf("device %d: %w", id, ErrNotFound)
	}
	delete(s.devices, id)
	if err := s.save(); err != nil {
		s.devices[id] = old
		return err
	}
	return nil
}

// save writes every device to a temporary file and renames it over the real one, so a crash
// can't leave a half-written file. The caller holds the lock.
func (s *MemoryStore) save() error {
	if s.path == "" {
		return nil
	}
	data, err := json.MarshalIndent(s.sorted(Filter{}), "", "  ")
	if err != nil {
		return err
	}
	tmp := s.path + ".tmp"
	if err := os.WriteFile(tmp, data, 0o644); err != nil {
		return err
	}
	return os.Rename(tmp, s.path)
}
@@file api.go
package main

import (
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"log/slog"
	"net/http"
	"strconv"
	"time"
)

// API holds what the handlers need. Nothing is global, so tests can build as many as they like.
type API struct {
	store Store
	log   *slog.Logger
}

// NewAPI returns the whole HTTP application: the routes, wrapped in the middleware.
func NewAPI(store Store, log *slog.Logger) http.Handler {
	a := &API{store: store, log: log}
	mux := http.NewServeMux()
	// Go 1.22+ patterns: a method, a path, and {wildcards} read with r.PathValue.
	mux.HandleFunc("GET /healthz", a.health)
	mux.HandleFunc("GET /devices", a.list)
	mux.HandleFunc("POST /devices", a.create)
	mux.HandleFunc("GET /devices/{id}", a.get)
	mux.HandleFunc("PUT /devices/{id}", a.update)
	mux.HandleFunc("DELETE /devices/{id}", a.delete)
	return a.recoverPanics(a.logRequests(mux))
}

func (a *API) health(w http.ResponseWriter, r *http.Request) {
	writeJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}

// GET /devices?kind=laptop&owner=aiko&q=tk-
func (a *API) list(w http.ResponseWriter, r *http.Request) {
	q := r.URL.Query()
	writeJSON(w, http.StatusOK, a.store.List(Filter{Kind: q.Get("kind"), Owner: q.Get("owner"), Query: q.Get("q")}))
}

func (a *API) get(w http.ResponseWriter, r *http.Request) {
	id, err := pathID(r)
	if err != nil {
		a.fail(w, err)
		return
	}
	d, err := a.store.Get(id)
	if err != nil {
		a.fail(w, err)
		return
	}
	writeJSON(w, http.StatusOK, d)
}

// POST /devices with a JSON device (without id): 201 Created, the device with its new id,
// and a Location header pointing at it.
func (a *API) create(w http.ResponseWriter, r *http.Request) {
	var d Device
	if err := decodeJSON(r, &d); err != nil {
		a.fail(w, err)
		return
	}
	d = Normalize(d)
	if err := Validate(d); err != nil {
		a.fail(w, err)
		return
	}
	created, err := a.store.Create(d)
	if err != nil {
		a.fail(w, err)
		return
	}
	w.Header().Set("Location", fmt.Sprintf("/devices/%d", created.ID))
	writeJSON(w, http.StatusCreated, created)
}

// PUT /devices/{id} with a JSON device: replaces the device, keeping the id from the URL
// (any id in the body is ignored). Same steps as create: decode, normalize, validate, then
// a.store.Update. 200 OK with the updated device. Every error goes through a.fail.
func (a *API) update(w http.ResponseWriter, r *http.Request) {
	id, err := pathID(r)
	if err != nil {
		a.fail(w, err)
		return
	}
	var d Device
	if err := decodeJSON(r, &d); err != nil {
		a.fail(w, err)
		return
	}
	d = Normalize(d)
	d.ID = id
	if err := Validate(d); err != nil {
		a.fail(w, err)
		return
	}
	updated, err := a.store.Update(d)
	if err != nil {
		a.fail(w, err)
		return
	}
	writeJSON(w, http.StatusOK, updated)
}

// DELETE /devices/{id}: 204 No Content.
func (a *API) delete(w http.ResponseWriter, r *http.Request) {
	id, err := pathID(r)
	if err != nil {
		a.fail(w, err)
		return
	}
	if err := a.store.Delete(id); err != nil {
		a.fail(w, err)
		return
	}
	w.WriteHeader(http.StatusNoContent)
}

// errBadRequest marks errors caused by the request itself (bad JSON, a bad id).
var errBadRequest = errors.New("bad request")

func pathID(r *http.Request) (int, error) {
	id, err := strconv.Atoi(r.PathValue("id"))
	if err != nil || id < 1 {
		return 0, fmt.Errorf("%w: the id must be a positive whole number", errBadRequest)
	}
	return id, nil
}

// decodeJSON reads one JSON object from the body: at most 1 MB, and no unknown fields
// (a typo like "hostnme" is an error, not silently ignored).
func decodeJSON(r *http.Request, v any) error {
	dec := json.NewDecoder(io.LimitReader(r.Body, 1<<20))
	dec.DisallowUnknownFields()
	if err := dec.Decode(v); err != nil {
		return fmt.Errorf("%w: invalid JSON: %v", errBadRequest, err)
	}
	return nil
}

// statusFor chooses the HTTP status for an error, looking through wrapping:
// a *ValidationError -> 422 Unprocessable Entity, errBadRequest -> 400, ErrNotFound -> 404,
// ErrDuplicate -> 409 Conflict, anything else -> 500. Use errors.As and errors.Is.
func statusFor(err error) int {
	var verr *ValidationError
	switch {
	case errors.As(err, &verr):
		return http.StatusUnprocessableEntity
	case errors.Is(err, errBadRequest):
		return http.StatusBadRequest
	case errors.Is(err, ErrNotFound):
		return http.StatusNotFound
	case errors.Is(err, ErrDuplicate):
		return http.StatusConflict
	default:
		return http.StatusInternalServerError
	}
}

// fail sends an error as JSON: {"error": "..."}, plus "problems" for validation errors.
// Unexpected errors are logged, and the client only gets a generic message: internal details
// (file paths, SQL) must never leak out.
func (a *API) fail(w http.ResponseWriter, err error) {
	status := statusFor(err)
	body := map[string]any{"error": err.Error()}
	var verr *ValidationError
	if errors.As(err, &verr) {
		body["problems"] = verr.Problems
	}
	if status == http.StatusInternalServerError {
		a.log.Error("internal error", "err", err)
		body = map[string]any{"error": "internal server error"}
	}
	writeJSON(w, status, body)
}

func writeJSON(w http.ResponseWriter, status int, v any) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)
	_ = json.NewEncoder(w).Encode(v) // the client went away; nothing useful to do
}

// statusRecorder remembers the status a handler sent, for the request log.
type statusRecorder struct {
	http.ResponseWriter
	status int
}

func (s *statusRecorder) WriteHeader(code int) {
	s.status = code
	s.ResponseWriter.WriteHeader(code)
}

// logRequests is middleware: it wraps a handler and logs every request after it's handled.
func (a *API) logRequests(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		start := time.Now()
		rec := &statusRecorder{ResponseWriter: w, status: http.StatusOK}
		next.ServeHTTP(rec, r)
		a.log.Info("request", "method", r.Method, "path", r.URL.Path, "status", rec.status,
			"duration", time.Since(start))
	})
}

// recoverPanics turns a panic in a handler into a 500 response instead of a dropped connection.
func (a *API) recoverPanics(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		defer func() {
			if v := recover(); v != nil {
				a.log.Error("panic", "value", v, "path", r.URL.Path)
				writeJSON(w, http.StatusInternalServerError, map[string]string{"error": "internal server error"})
			}
		}()
		next.ServeHTTP(w, r)
	})
}
@@file api_test.go
package main

import (
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"log/slog"
	"net/http"
	"net/http/httptest"
	"path/filepath"
	"strings"
	"sync"
	"testing"
)

// ---- helpers ----

func newTestAPI(store Store) http.Handler {
	return NewAPI(store, slog.New(slog.NewTextHandler(io.Discard, nil)))
}

// call sends one request straight to the handler (no network) and returns the recorded response.
func call(h http.Handler, method, path, body string) *httptest.ResponseRecorder {
	var r io.Reader
	if body != "" {
		r = strings.NewReader(body)
	}
	rec := httptest.NewRecorder()
	h.ServeHTTP(rec, httptest.NewRequest(method, path, r))
	return rec
}

func decode[T any](t *testing.T, rec *httptest.ResponseRecorder) T {
	t.Helper()
	var v T
	if err := json.Unmarshal(rec.Body.Bytes(), &v); err != nil {
		t.Fatalf("response is not JSON: %v\n%s", err, rec.Body.String())
	}
	return v
}

func wantStatus(t *testing.T, rec *httptest.ResponseRecorder, want int) {
	t.Helper()
	if rec.Code != want {
		t.Fatalf("status %d, want %d; body: %s", rec.Code, want, strings.TrimSpace(rec.Body.String()))
	}
}

const laptop = `{"hostname": "tk-lt-001", "ip": "10.0.1.21", "kind": "laptop", "owner": "aiko"}`

func seeded(t *testing.T) http.Handler {
	t.Helper()
	h := newTestAPI(NewMemoryStore())
	for _, body := range []string{
		laptop,
		`{"hostname": "tk-sw-core", "ip": "10.0.0.2", "kind": "switch"}`,
		`{"hostname": "os-lt-007", "ip": "10.1.1.7", "kind": "laptop", "owner": "Ken"}`,
		`{"hostname": "tk-prn-3f", "ip": "10.0.3.50", "kind": "printer"}`,
	} {
		wantStatus(t, call(h, "POST", "/devices", body), http.StatusCreated)
	}
	return h
}

func hostnames(devices []Device) string {
	var names []string
	for _, d := range devices {
		names = append(names, d.Hostname)
	}
	return strings.Join(names, ",")
}

// ---- the rules ----

func TestNormalize(t *testing.T) {
	got := Normalize(Device{Hostname: "  TK-LT-001 ", IP: " 2001:DB8::0001 ", Kind: "Laptop ", Owner: " aiko "})
	want := Device{Hostname: "tk-lt-001", IP: "2001:db8::1", Kind: "laptop", Owner: "aiko"}
	if got != want {
		t.Fatalf("got %+v, want %+v", got, want)
	}
}

func TestValidate(t *testing.T) {
	ok := Device{Hostname: "tk-lt-001", IP: "10.0.1.21", Kind: "laptop"}
	tests := []struct {
		name   string
		change func(d *Device)
		want   []string
	}{
		{"valid", func(d *Device) {}, nil},
		{"one-letter hostname", func(d *Device) { d.Hostname = "a" }, nil},
		{"ipv6", func(d *Device) { d.IP = "2001:db8::1" }, nil},
		{"hostname with a dot", func(d *Device) { d.Hostname = "tk.lt" },
			[]string{`hostname "tk.lt" must be 1-63 lower-case letters, digits or dashes, and not start or end with a dash`}},
		{"hostname ending with a dash", func(d *Device) { d.Hostname = "tk-" },
			[]string{`hostname "tk-" must be 1-63 lower-case letters, digits or dashes, and not start or end with a dash`}},
		{"bad ip", func(d *Device) { d.IP = "10.0.1.300" }, []string{`ip "10.0.1.300" is not an IP address`}},
		{"bad kind", func(d *Device) { d.Kind = "toaster" },
			[]string{`kind "toaster" must be one of: desktop, laptop, printer, server, switch`}},
		{"everything wrong", func(d *Device) { d.Hostname = ""; d.IP = ""; d.Kind = "" }, []string{
			`hostname "" must be 1-63 lower-case letters, digits or dashes, and not start or end with a dash`,
			`ip "" is not an IP address`,
			`kind "" must be one of: desktop, laptop, printer, server, switch`,
		}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			d := ok
			tt.change(&d)
			err := Validate(d)
			if tt.want == nil {
				if err != nil {
					t.Fatalf("want no error, got %v", err)
				}
				return
			}
			var verr *ValidationError
			if !errors.As(err, &verr) {
				t.Fatalf("want a *ValidationError, got %v", err)
			}
			if strings.Join(verr.Problems, "|") != strings.Join(tt.want, "|") {
				t.Fatalf("problems:\n got %q\nwant %q", verr.Problems, tt.want)
			}
		})
	}
}

func TestFilterMatches(t *testing.T) {
	d := Device{Hostname: "tk-lt-001", Kind: "laptop", Owner: "Aiko"}
	tests := []struct {
		f    Filter
		want bool
	}{
		{Filter{}, true},
		{Filter{Kind: "laptop"}, true},
		{Filter{Kind: "switch"}, false},
		{Filter{Owner: "aiko"}, true},
		{Filter{Owner: "ken"}, false},
		{Filter{Query: "LT-0"}, true},
		{Filter{Query: "sw"}, false},
		{Filter{Kind: "laptop", Owner: "AIKO", Query: "tk"}, true},
		{Filter{Kind: "laptop", Owner: "aiko", Query: "os-"}, false},
	}
	for _, tt := range tests {
		if got := d.matchesFilter(tt.f); got != tt.want {
			t.Errorf("%+v: got %v, want %v", tt.f, got, tt.want)
		}
	}
}

// matchesFilter reads better in the table above.
func (d Device) matchesFilter(f Filter) bool { return f.matches(d) }

func TestStatusFor(t *testing.T) {
	tests := []struct {
		err  error
		want int
	}{
		{&ValidationError{Problems: []string{"x"}}, 422},
		{fmt.Errorf("saving: %w", &ValidationError{}), 422},
		{fmt.Errorf("%w: bad id", errBadRequest), 400},
		{fmt.Errorf("device 7: %w", ErrNotFound), 404},
		{fmt.Errorf("hostname x: %w", ErrDuplicate), 409},
		{errors.New("disk full"), 500},
	}
	for _, tt := range tests {
		if got := statusFor(tt.err); got != tt.want {
			t.Errorf("statusFor(%v) = %d, want %d", tt.err, got, tt.want)
		}
	}
}

// ---- the HTTP API ----

func TestHealth(t *testing.T) {
	rec := call(newTestAPI(NewMemoryStore()), "GET", "/healthz", "")
	wantStatus(t, rec, 200)
	if decode[map[string]string](t, rec)["status"] != "ok" {
		t.Fatal(rec.Body.String())
	}
}

func TestCreateAndGet(t *testing.T) {
	h := newTestAPI(NewMemoryStore())
	rec := call(h, "POST", "/devices", `{"hostname": " TK-LT-001", "ip": "10.0.1.21", "kind": "Laptop", "owner": "aiko"}`)
	wantStatus(t, rec, http.StatusCreated)
	if loc := rec.Header().Get("Location"); loc != "/devices/1" {
		t.Fatalf("Location %q", loc)
	}
	if ct := rec.Header().Get("Content-Type"); ct != "application/json" {
		t.Fatalf("Content-Type %q", ct)
	}
	want := Device{ID: 1, Hostname: "tk-lt-001", IP: "10.0.1.21", Kind: "laptop", Owner: "aiko"}
	if got := decode[Device](t, rec); got != want {
		t.Fatalf("created %+v", got)
	}
	rec = call(h, "GET", "/devices/1", "")
	wantStatus(t, rec, 200)
	if got := decode[Device](t, rec); got != want {
		t.Fatalf("got %+v", got)
	}
}

func TestCreateErrors(t *testing.T) {
	h := seeded(t)
	rec := call(h, "POST", "/devices", `{"hostname": "bad host", "ip": "10.0.0.999", "kind": "laptop"}`)
	wantStatus(t, rec, http.StatusUnprocessableEntity)
	body := decode[struct {
		Error    string   `json:"error"`
		Problems []string `json:"problems"`
	}](t, rec)
	if len(body.Problems) != 2 || !strings.HasPrefix(body.Error, "invalid device: ") {
		t.Fatalf("body %+v", body)
	}

	wantStatus(t, call(h, "POST", "/devices", `{"hostname": "TK-LT-001", "ip": "10.0.9.9", "kind": "laptop"}`), http.StatusConflict)
	wantStatus(t, call(h, "POST", "/devices", `{"hostname": "x", `), http.StatusBadRequest)
	wantStatus(t, call(h, "POST", "/devices", `{"hostnme": "typo", "ip": "10.0.0.1", "kind": "laptop"}`), http.StatusBadRequest)
	wantStatus(t, call(h, "POST", "/devices", ""), http.StatusBadRequest)
	if n := len(decode[[]Device](t, call(h, "GET", "/devices", ""))); n != 4 {
		t.Fatalf("%d devices after the failed requests, want 4", n)
	}
}

func TestGetErrors(t *testing.T) {
	h := seeded(t)
	wantStatus(t, call(h, "GET", "/devices/99", ""), http.StatusNotFound)
	wantStatus(t, call(h, "GET", "/devices/abc", ""), http.StatusBadRequest)
	wantStatus(t, call(h, "GET", "/devices/-1", ""), http.StatusBadRequest)
	wantStatus(t, call(h, "PATCH", "/devices/1", "{}"), http.StatusMethodNotAllowed)
}

func TestListAndFilters(t *testing.T) {
	h := seeded(t)
	tests := map[string]string{
		"/devices":                         "os-lt-007,tk-lt-001,tk-prn-3f,tk-sw-core",
		"/devices?kind=laptop":             "os-lt-007,tk-lt-001",
		"/devices?owner=ken":               "os-lt-007",
		"/devices?q=TK-":                   "tk-lt-001,tk-prn-3f,tk-sw-core",
		"/devices?kind=laptop&q=tk":        "tk-lt-001",
		"/devices?kind=server":             "",
		"/devices?kind=printer&owner=aiko": "",
	}
	for path, want := range tests {
		rec := call(h, "GET", path, "")
		wantStatus(t, rec, 200)
		if got := hostnames(decode[[]Device](t, rec)); got != want {
			t.Errorf("%s: got %q, want %q", path, got, want)
		}
	}
	if body := strings.TrimSpace(call(h, "GET", "/devices?kind=server", "").Body.String()); body != "[]" {
		t.Errorf("an empty list must be [], got %s", body)
	}
}

func TestUpdate(t *testing.T) {
	h := seeded(t)
	rec := call(h, "PUT", "/devices/1", `{"id": 99, "hostname": "tk-lt-001", "ip": "10.0.1.22", "kind": "Laptop", "owner": "yuki"}`)
	wantStatus(t, rec, 200)
	want := Device{ID: 1, Hostname: "tk-lt-001", IP: "10.0.1.22", Kind: "laptop", Owner: "yuki"}
	if got := decode[Device](t, rec); got != want {
		t.Fatalf("updated %+v", got)
	}
	if got := decode[Device](t, call(h, "GET", "/devices/1", "")); got != want {
		t.Fatalf("stored %+v", got)
	}
	wantStatus(t, call(h, "PUT", "/devices/1", `{"hostname": "tk-sw-core", "ip": "10.0.1.22", "kind": "laptop"}`), http.StatusConflict)
	wantStatus(t, call(h, "PUT", "/devices/1", `{"hostname": "tk-lt-001", "ip": "nope", "kind": "laptop"}`), http.StatusUnprocessableEntity)
	wantStatus(t, call(h, "PUT", "/devices/42", `{"hostname": "new-one", "ip": "10.0.0.42", "kind": "server"}`), http.StatusNotFound)
	wantStatus(t, call(h, "PUT", "/devices/x", laptop), http.StatusBadRequest)
	wantStatus(t, call(h, "PUT", "/devices/1", `not json`), http.StatusBadRequest)
}

func TestDelete(t *testing.T) {
	h := seeded(t)
	rec := call(h, "DELETE", "/devices/2", "")
	wantStatus(t, rec, http.StatusNoContent)
	if rec.Body.Len() != 0 {
		t.Fatalf("204 must have no body, got %q", rec.Body.String())
	}
	wantStatus(t, call(h, "GET", "/devices/2", ""), http.StatusNotFound)
	wantStatus(t, call(h, "DELETE", "/devices/2", ""), http.StatusNotFound)
}

// panickyStore is a fake Store whose List panics, to test the recovery middleware.
type panickyStore struct{ Store }

func (panickyStore) List(Filter) []Device { panic("database exploded") }

func TestPanicsBecome500(t *testing.T) {
	h := newTestAPI(panickyStore{NewMemoryStore()})
	rec := call(h, "GET", "/devices", "")
	wantStatus(t, rec, http.StatusInternalServerError)
	if strings.Contains(rec.Body.String(), "exploded") {
		t.Fatal("internal details leaked to the client")
	}
	wantStatus(t, call(h, "GET", "/healthz", ""), 200) // the server still works
}

func TestConcurrentCreates(t *testing.T) {
	store := NewMemoryStore()
	h := newTestAPI(store)
	var wg sync.WaitGroup
	codes := make(chan int, 50)
	for i := 0; i < 50; i++ {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()
			body := fmt.Sprintf(`{"hostname": "host-%02d", "ip": "10.9.0.%d", "kind": "server"}`, i, i+1)
			codes <- call(h, "POST", "/devices", body).Code
		}(i)
	}
	wg.Wait()
	close(codes)
	for code := range codes {
		if code != http.StatusCreated {
			t.Fatalf("a create returned %d", code)
		}
	}
	ids := map[int]bool{}
	for _, d := range store.List(Filter{}) {
		ids[d.ID] = true
	}
	if len(ids) != 50 {
		t.Fatalf("%d distinct ids, want 50", len(ids))
	}
}

func TestFileStoreSurvivesRestart(t *testing.T) {
	path := filepath.Join(t.TempDir(), "devices.json")
	store, err := OpenFileStore(path)
	if err != nil {
		t.Fatal(err)
	}
	h := newTestAPI(store)
	wantStatus(t, call(h, "POST", "/devices", laptop), http.StatusCreated)
	wantStatus(t, call(h, "POST", "/devices", `{"hostname": "tk-sw-core", "ip": "10.0.0.2", "kind": "switch"}`), http.StatusCreated)
	wantStatus(t, call(h, "DELETE", "/devices/1", ""), http.StatusNoContent)

	reopened, err := OpenFileStore(path)
	if err != nil {
		t.Fatal(err)
	}
	if got := hostnames(reopened.List(Filter{})); got != "tk-sw-core" {
		t.Fatalf("after restart: %q", got)
	}
	d, err := reopened.Create(Device{Hostname: "new", IP: "10.0.0.3", Kind: "server"})
	if err != nil || d.ID != 3 {
		t.Fatalf("ids must continue after the highest: got %d, %v", d.ID, err)
	}
}
@@file main.go
// Device Inventory API: go run .     then try:  curl http://127.0.0.1:8080/devices
package main

import (
	"context"
	"errors"
	"flag"
	"log/slog"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"
)

func main() {
	// Settings: a flag, or an environment variable (what containers use), or the default.
	addr := flag.String("addr", envOr("ADDR", "127.0.0.1:8080"), "address to listen on")
	data := flag.String("data", envOr("DATA_FILE", "devices.json"), "JSON file to keep the devices in")
	flag.Parse()

	log := slog.New(slog.NewTextHandler(os.Stdout, nil))
	store, err := OpenFileStore(*data)
	if err != nil {
		log.Error("cannot open the store", "err", err)
		os.Exit(1)
	}

	// Timeouts stop slow or stuck clients from holding connections open forever.
	srv := &http.Server{
		Addr:              *addr,
		Handler:           NewAPI(store, log),
		ReadHeaderTimeout: 5 * time.Second,
		ReadTimeout:       10 * time.Second,
		WriteTimeout:      10 * time.Second,
		IdleTimeout:       time.Minute,
	}

	// Ctrl+C (or SIGTERM from Docker, systemd or Kubernetes) cancels ctx.
	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
	defer stop()

	go func() {
		log.Info("listening", "url", "http://"+*addr, "data", *data)
		if err := srv.ListenAndServe(); err != nil && !errors.Is(err, http.ErrServerClosed) {
			log.Error("server failed", "err", err)
			os.Exit(1)
		}
	}()

	<-ctx.Done()
	// Graceful shutdown: stop accepting connections, let requests in progress finish.
	log.Info("shutting down")
	shutdownCtx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()
	if err := srv.Shutdown(shutdownCtx); err != nil {
		log.Error("shutdown", "err", err)
	}
}

func envOr(key, fallback string) string {
	if v := os.Getenv(key); v != "" {
		return v
	}
	return fallback
}

@@drill
Which HTTP status code means a request conflicts with existing data, such as a duplicate ID?
= 409
? 409 Conflict.

@@text
🎉 Routing, validation, error mapping, middleware, concurrency-safe storage, graceful shutdown and tests: this is the skeleton of most Go services running in production.

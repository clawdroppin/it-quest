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

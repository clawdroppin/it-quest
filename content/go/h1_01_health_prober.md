@@lesson g9-health-prober Service Health Prober
@@text
"Is everything up?" is the first question in every incident. You'll build the tool that answers it in a second: give it a list of services, and it checks them **all at once** (TCP ports and HTTP endpoints), with a timeout for each, and prints a status table and latency percentiles:

```text
  UP    https://example.com                      143ms
  UP    https://www.iana.org/help/example-domains 211ms
  UP    tcp://one.one.one.one:53                 9ms
  DOWN  tcp://localhost:22                       dial tcp [::1]:22: connectex: No connection could be made...
  DOWN  http://localhost:8080/health             Get "http://localhost:8080/health": dial tcp ...
3/5 up, p50 143ms, p95 211ms
```

It exits with code **2** when something is down, so a cron job, a scheduled task or a CI pipeline can raise an alert, and `-json` prints machine-readable output for a dashboard.

This is a very Go-shaped program: a **worker pool** of goroutines that limits how many checks run at once, `context` timeouts, `net` and `net/http`, and one static binary at the end that you can copy to any server.

The important design choice: `Run` takes the prober as a **function parameter**. The real `NetProber` touches the network; the tests pass a fake one that answers from a table and measures how many probes run at the same time. So the tests are fast, reliable and work offline.

1. `go test ./...`: everything fails (`TODO`).
2. Write `ParseTarget`, `Run`, `Percentile` and `Summary` in `probe.go`. The comment above each says what to do. Delete the lines in the `var (...)` block at the bottom as you start using those packages.
3. When the tests pass: `go run .`, then `go run . -json`.
4. Build for a Linux server: `GOOS=linux go build -o healthprobe` (PowerShell: `$env:GOOS="linux"; go build -o healthprobe`).
@@project health_prober Service Health Prober
Write the four functions in `probe.go` until `go test ./...` passes (add `-race` to check your worker pool for data races), then run it on `targets.txt`, and on your own services.
@@file probe.go
package main

import (
	"context"
	"fmt"
	"net"
	"net/http"
	"net/url"
	"sort"
	"strconv"
	"strings"
	"sync"
	"time"
)

// Target is one thing to check: a TCP port, or an HTTP(S) URL.
type Target struct {
	Raw  string // the original text, e.g. "https://example.jp/health"
	Kind string // "tcp", "http" or "https"
	Host string
	Port int
	Path string // HTTP only; "/" when the URL has none
}

// Result of probing one target.
type Result struct {
	Target  Target
	Up      bool
	Latency time.Duration
	Err     string // empty when Up
}

// A Prober checks one target and returns how long it took.
type Prober func(ctx context.Context, t Target) (time.Duration, error)

// ParseTarget reads "tcp://db01:5432", "http://intranet" or "https://example.jp:8443/health".
//   - tcp needs a port; http defaults to 80 and https to 443.
//   - Path is "/" for http(s) URLs without one, and empty for tcp.
//   - Spaces around the text are ignored; Raw keeps the trimmed text.
//   - Anything else returns an error starting with "bad target".
func ParseTarget(s string) (Target, error) {
	return Target{}, fmt.Errorf("TODO: not done yet")
}

// Run probes every target using at most `workers` goroutines at once, and returns the
// results in the same order as targets. A probe error makes that result down, with Err
// set to the error's text. Up results have the latency the prober returned.
func Run(ctx context.Context, targets []Target, probe Prober, workers int) []Result {
	return nil // TODO: not done yet
}

// Percentile returns the p-th percentile (0-100) of the durations by the nearest-rank
// method: sort, then take the item at rank ceil(p/100 * n), counting from 1.
// p <= 0 gives the smallest; an empty list gives 0. Don't change the caller's slice.
func Percentile(ds []time.Duration, p float64) time.Duration {
	return 0 // TODO: not done yet
}

// Summary is the last line of the report, e.g. "3/4 up, p50 12ms, p95 48ms" using the
// latencies of the up results only, rounded to the millisecond. With nothing up: "0/4 up".
func Summary(results []Result) string {
	return "TODO: not done yet"
}

// NetProber really connects: a TCP dial, or an HTTP GET that must answer below 400.
func NetProber(timeout time.Duration) Prober {
	client := &http.Client{Timeout: timeout}
	return func(ctx context.Context, t Target) (time.Duration, error) {
		ctx, cancel := context.WithTimeout(ctx, timeout)
		defer cancel()
		start := time.Now()
		if t.Kind == "tcp" {
			var d net.Dialer
			conn, err := d.DialContext(ctx, "tcp", net.JoinHostPort(t.Host, strconv.Itoa(t.Port)))
			if err != nil {
				return 0, err
			}
			conn.Close()
			return time.Since(start), nil
		}
		req, err := http.NewRequestWithContext(ctx, http.MethodGet, t.Raw, nil)
		if err != nil {
			return 0, err
		}
		resp, err := client.Do(req)
		if err != nil {
			return 0, err
		}
		resp.Body.Close()
		if resp.StatusCode >= 400 {
			return 0, fmt.Errorf("HTTP %d", resp.StatusCode)
		}
		return time.Since(start), nil
	}
}

// Packages you'll need. Delete each line once your code uses that package
// (Go refuses to compile a file with an unused import).
var (
	_ = url.Parse
	_ = sort.Slice
	_ = strings.TrimSpace
	_ sync.WaitGroup
)
@@file go.mod
module healthprobe

go 1.22
@@file probe_test.go
package main

import (
	"context"
	"errors"
	"strings"
	"sync/atomic"
	"testing"
	"time"
)

func TestParseTarget(t *testing.T) {
	tests := []struct {
		in   string
		want Target
	}{
		{"tcp://db01:5432", Target{Raw: "tcp://db01:5432", Kind: "tcp", Host: "db01", Port: 5432}},
		{"http://intranet", Target{Raw: "http://intranet", Kind: "http", Host: "intranet", Port: 80, Path: "/"}},
		{"https://example.jp/health", Target{Raw: "https://example.jp/health", Kind: "https", Host: "example.jp", Port: 443, Path: "/health"}},
		{"  https://example.jp:8443/api/ping ", Target{Raw: "https://example.jp:8443/api/ping", Kind: "https", Host: "example.jp", Port: 8443, Path: "/api/ping"}},
	}
	for _, tt := range tests {
		t.Run(tt.in, func(t *testing.T) {
			got, err := ParseTarget(tt.in)
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if got != tt.want {
				t.Errorf("got %+v, want %+v", got, tt.want)
			}
		})
	}
}

func TestParseTargetErrors(t *testing.T) {
	for _, in := range []string{"tcp://db01", "ftp://old", "db01:22", "", "tcp://db01:99999", "http://"} {
		t.Run(in, func(t *testing.T) {
			_, err := ParseTarget(in)
			if err == nil || !strings.HasPrefix(err.Error(), "bad target") {
				t.Errorf("ParseTarget(%q) error = %v, want one starting with \"bad target\"", in, err)
			}
		})
	}
}

// fakeProber answers from a table instead of the network, and tracks concurrency.
func fakeProber(latencies map[string]time.Duration, running, peak *int32) Prober {
	return func(ctx context.Context, t Target) (time.Duration, error) {
		n := atomic.AddInt32(running, 1)
		for {
			p := atomic.LoadInt32(peak)
			if n <= p || atomic.CompareAndSwapInt32(peak, p, n) {
				break
			}
		}
		time.Sleep(20 * time.Millisecond)
		atomic.AddInt32(running, -1)
		d, ok := latencies[t.Host]
		if !ok {
			return 0, errors.New("connection refused")
		}
		return d, nil
	}
}

func targets(hosts ...string) []Target {
	var ts []Target
	for _, h := range hosts {
		ts = append(ts, Target{Raw: "tcp://" + h + ":22", Kind: "tcp", Host: h, Port: 22})
	}
	return ts
}

func TestRunKeepsOrderAndErrors(t *testing.T) {
	var running, peak int32
	lat := map[string]time.Duration{"a": 5 * time.Millisecond, "c": 9 * time.Millisecond}
	got := Run(context.Background(), targets("a", "b", "c"), fakeProber(lat, &running, &peak), 2)
	if len(got) != 3 {
		t.Fatalf("got %d results, want 3", len(got))
	}
	if got[0].Target.Host != "a" || got[1].Target.Host != "b" || got[2].Target.Host != "c" {
		t.Errorf("results not in target order: %+v", got)
	}
	if !got[0].Up || got[0].Latency != 5*time.Millisecond || got[0].Err != "" {
		t.Errorf("a: got %+v", got[0])
	}
	if got[1].Up || got[1].Err != "connection refused" {
		t.Errorf("b should be down with the error text, got %+v", got[1])
	}
}

func TestRunLimitsWorkers(t *testing.T) {
	var running, peak int32
	lat := map[string]time.Duration{}
	start := time.Now()
	Run(context.Background(), targets("1", "2", "3", "4", "5", "6"), fakeProber(lat, &running, &peak), 3)
	if peak > 3 {
		t.Errorf("%d probes ran at once, want at most 3", peak)
	}
	if peak < 2 {
		t.Errorf("probes ran one at a time (peak %d): use goroutines", peak)
	}
	if time.Since(start) > 100*time.Millisecond {
		t.Errorf("took %v: the probes should overlap", time.Since(start))
	}
}

func TestPercentile(t *testing.T) {
	ms := func(ns ...int) []time.Duration {
		var ds []time.Duration
		for _, n := range ns {
			ds = append(ds, time.Duration(n)*time.Millisecond)
		}
		return ds
	}
	data := ms(40, 10, 30, 20, 50)
	cases := []struct {
		p    float64
		want int
	}{{50, 30}, {95, 50}, {20, 10}, {21, 20}, {0, 10}, {100, 50}}
	for _, c := range cases {
		if got := Percentile(data, c.p); got != time.Duration(c.want)*time.Millisecond {
			t.Errorf("p%v = %v, want %dms", c.p, got, c.want)
		}
	}
	if data[0] != 40*time.Millisecond {
		t.Errorf("Percentile changed the caller's slice")
	}
	if Percentile(nil, 50) != 0 {
		t.Errorf("empty list should give 0")
	}
}

func TestSummary(t *testing.T) {
	rs := []Result{
		{Up: true, Latency: 12 * time.Millisecond},
		{Up: false, Err: "timeout"},
		{Up: true, Latency: 48*time.Millisecond + 300*time.Microsecond},
		{Up: true, Latency: 20 * time.Millisecond},
	}
	if got, want := Summary(rs), "3/4 up, p50 20ms, p95 48ms"; got != want {
		t.Errorf("got %q, want %q", got, want)
	}
	if got, want := Summary([]Result{{Up: false}, {Up: false}}), "0/2 up"; got != want {
		t.Errorf("got %q, want %q", got, want)
	}
}
@@file main.go
// healthprobe: check many services at once.
//
//	go run . -file targets.txt -workers 8 -timeout 3s
//	go run . -json > status.json
package main

import (
	"bufio"
	"context"
	"encoding/json"
	"flag"
	"fmt"
	"os"
	"strings"
	"time"
)

func main() {
	file := flag.String("file", "targets.txt", "file with one target per line (# for comments)")
	workers := flag.Int("workers", 8, "how many checks run at the same time")
	timeout := flag.Duration("timeout", 3*time.Second, "timeout for each check")
	asJSON := flag.Bool("json", false, "print JSON instead of a table")
	flag.Parse()

	targets, err := readTargets(*file)
	if err != nil {
		fmt.Fprintln(os.Stderr, "error:", err)
		os.Exit(1)
	}

	results := Run(context.Background(), targets, NetProber(*timeout), *workers)

	if *asJSON {
		enc := json.NewEncoder(os.Stdout)
		enc.SetIndent("", "  ")
		enc.Encode(results)
	} else {
		for _, r := range results {
			if r.Up {
				fmt.Printf("  UP    %-40s %v\n", r.Target.Raw, r.Latency.Round(time.Millisecond))
			} else {
				fmt.Printf("  DOWN  %-40s %s\n", r.Target.Raw, r.Err)
			}
		}
		fmt.Println(Summary(results))
	}

	for _, r := range results {
		if !r.Up {
			os.Exit(2) // something is down: let a cron job or pipeline notice
		}
	}
}

func readTargets(path string) ([]Target, error) {
	f, err := os.Open(path)
	if err != nil {
		return nil, err
	}
	defer f.Close()
	var targets []Target
	sc := bufio.NewScanner(f)
	for n := 1; sc.Scan(); n++ {
		line := strings.TrimSpace(strings.SplitN(sc.Text(), "#", 2)[0])
		if line == "" {
			continue
		}
		t, err := ParseTarget(line)
		if err != nil {
			return nil, fmt.Errorf("%s line %d: %w", path, n, err)
		}
		targets = append(targets, t)
	}
	return targets, sc.Err()
}
@@file targets.txt
# One target per line: tcp://host:port, http://host[/path] or https://host[:port][/path]
https://example.com              # a public web site
https://www.iana.org/help/example-domains
tcp://one.one.one.one:53         # Cloudflare DNS over TCP
tcp://localhost:22               # SSH on this machine (probably down on Windows)
http://localhost:8080/health     # your own service, if it's running
@@text
> Tips: `url.Parse` does most of `ParseTarget`'s work: look at `u.Scheme`, `u.Hostname()`, `u.Port()` and `u.Path`. For `Run`, make the `results` slice first, start `workers` goroutines that `range` over a `jobs` channel of indexes, send every index, `close(jobs)`, then `wg.Wait()`. For `Percentile`, copy the slice before sorting (`append([]time.Duration(nil), ds...)`); the rank is `math.Ceil(p / 100 * n)`, at least 1.
@@project health_prober_finished Service Health Prober: the finished version
The complete prober. Ideas: retries before marking something down, TLS certificate expiry for https targets (`resp.TLS.PeerCertificates[0].NotAfter`), a `-watch 30s` mode that re-checks forever, a `/metrics` endpoint in Prometheus format, or a tiny distroless container image.
@@file probe.go
package main

import (
	"context"
	"fmt"
	"net"
	"net/http"
	"net/url"
	"sort"
	"strconv"
	"strings"
	"sync"
	"time"
)

// Target is one thing to check: a TCP port, or an HTTP(S) URL.
type Target struct {
	Raw  string // the original text, e.g. "https://example.jp/health"
	Kind string // "tcp", "http" or "https"
	Host string
	Port int
	Path string // HTTP only; "/" when the URL has none
}

// Result of probing one target.
type Result struct {
	Target  Target
	Up      bool
	Latency time.Duration
	Err     string // empty when Up
}

// A Prober checks one target and returns how long it took.
type Prober func(ctx context.Context, t Target) (time.Duration, error)

// ParseTarget reads "tcp://db01:5432", "http://intranet" or "https://example.jp:8443/health".
//   - tcp needs a port; http defaults to 80 and https to 443.
//   - Path is "/" for http(s) URLs without one, and empty for tcp.
//   - Spaces around the text are ignored; Raw keeps the trimmed text.
//   - Anything else returns an error starting with "bad target".
func ParseTarget(s string) (Target, error) {
	s = strings.TrimSpace(s)
	u, err := url.Parse(s)
	if err != nil || u.Hostname() == "" {
		return Target{}, fmt.Errorf("bad target %q", s)
	}
	t := Target{Raw: s, Kind: u.Scheme, Host: u.Hostname()}
	switch u.Scheme {
	case "tcp":
		if u.Port() == "" {
			return Target{}, fmt.Errorf("bad target %q: tcp needs a port", s)
		}
	case "http", "https":
		t.Path = u.Path
		if t.Path == "" {
			t.Path = "/"
		}
	default:
		return Target{}, fmt.Errorf("bad target %q: unknown scheme %q", s, u.Scheme)
	}
	switch {
	case u.Port() != "":
		p, err := strconv.Atoi(u.Port())
		if err != nil || p < 1 || p > 65535 {
			return Target{}, fmt.Errorf("bad target %q: bad port", s)
		}
		t.Port = p
	case u.Scheme == "https":
		t.Port = 443
	default:
		t.Port = 80
	}
	return t, nil
}

// Run probes every target using at most `workers` goroutines at once, and returns the
// results in the same order as targets. A probe error makes that result down, with Err
// set to the error's text. Up results have the latency the prober returned.
func Run(ctx context.Context, targets []Target, probe Prober, workers int) []Result {
	results := make([]Result, len(targets))
	jobs := make(chan int)
	var wg sync.WaitGroup
	for w := 0; w < max(1, workers); w++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for i := range jobs {
				latency, err := probe(ctx, targets[i])
				r := Result{Target: targets[i], Up: err == nil, Latency: latency}
				if err != nil {
					r.Err = err.Error()
				}
				results[i] = r // each index is written by exactly one goroutine
			}
		}()
	}
	for i := range targets {
		jobs <- i
	}
	close(jobs)
	wg.Wait()
	return results
}

// Percentile returns the p-th percentile (0-100) of the durations by the nearest-rank
// method: sort, then take the item at rank ceil(p/100 * n), counting from 1.
// p <= 0 gives the smallest; an empty list gives 0. Don't change the caller's slice.
func Percentile(ds []time.Duration, p float64) time.Duration {
	if len(ds) == 0 {
		return 0
	}
	sorted := append([]time.Duration(nil), ds...)
	sort.Slice(sorted, func(i, j int) bool { return sorted[i] < sorted[j] })
	rank := int(p / 100 * float64(len(sorted)))
	if float64(rank) < p/100*float64(len(sorted)) {
		rank++ // ceil
	}
	if rank < 1 {
		rank = 1
	}
	if rank > len(sorted) {
		rank = len(sorted)
	}
	return sorted[rank-1]
}

// Summary is the last line of the report, e.g. "3/4 up, p50 12ms, p95 48ms" using the
// latencies of the up results only, rounded to the millisecond. With nothing up: "0/4 up".
func Summary(results []Result) string {
	var latencies []time.Duration
	for _, r := range results {
		if r.Up {
			latencies = append(latencies, r.Latency)
		}
	}
	line := fmt.Sprintf("%d/%d up", len(latencies), len(results))
	if len(latencies) == 0 {
		return line
	}
	p50 := Percentile(latencies, 50).Round(time.Millisecond)
	p95 := Percentile(latencies, 95).Round(time.Millisecond)
	return fmt.Sprintf("%s, p50 %v, p95 %v", line, p50, p95)
}

// NetProber really connects: a TCP dial, or an HTTP GET that must answer below 400.
func NetProber(timeout time.Duration) Prober {
	client := &http.Client{Timeout: timeout}
	return func(ctx context.Context, t Target) (time.Duration, error) {
		ctx, cancel := context.WithTimeout(ctx, timeout)
		defer cancel()
		start := time.Now()
		if t.Kind == "tcp" {
			var d net.Dialer
			conn, err := d.DialContext(ctx, "tcp", net.JoinHostPort(t.Host, strconv.Itoa(t.Port)))
			if err != nil {
				return 0, err
			}
			conn.Close()
			return time.Since(start), nil
		}
		req, err := http.NewRequestWithContext(ctx, http.MethodGet, t.Raw, nil)
		if err != nil {
			return 0, err
		}
		resp, err := client.Do(req)
		if err != nil {
			return 0, err
		}
		resp.Body.Close()
		if resp.StatusCode >= 400 {
			return 0, fmt.Errorf("HTTP %d", resp.StatusCode)
		}
		return time.Since(start), nil
	}
}
@@file go.mod
module healthprobe

go 1.22
@@file probe_test.go
package main

import (
	"context"
	"errors"
	"strings"
	"sync/atomic"
	"testing"
	"time"
)

func TestParseTarget(t *testing.T) {
	tests := []struct {
		in   string
		want Target
	}{
		{"tcp://db01:5432", Target{Raw: "tcp://db01:5432", Kind: "tcp", Host: "db01", Port: 5432}},
		{"http://intranet", Target{Raw: "http://intranet", Kind: "http", Host: "intranet", Port: 80, Path: "/"}},
		{"https://example.jp/health", Target{Raw: "https://example.jp/health", Kind: "https", Host: "example.jp", Port: 443, Path: "/health"}},
		{"  https://example.jp:8443/api/ping ", Target{Raw: "https://example.jp:8443/api/ping", Kind: "https", Host: "example.jp", Port: 8443, Path: "/api/ping"}},
	}
	for _, tt := range tests {
		t.Run(tt.in, func(t *testing.T) {
			got, err := ParseTarget(tt.in)
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if got != tt.want {
				t.Errorf("got %+v, want %+v", got, tt.want)
			}
		})
	}
}

func TestParseTargetErrors(t *testing.T) {
	for _, in := range []string{"tcp://db01", "ftp://old", "db01:22", "", "tcp://db01:99999", "http://"} {
		t.Run(in, func(t *testing.T) {
			_, err := ParseTarget(in)
			if err == nil || !strings.HasPrefix(err.Error(), "bad target") {
				t.Errorf("ParseTarget(%q) error = %v, want one starting with \"bad target\"", in, err)
			}
		})
	}
}

// fakeProber answers from a table instead of the network, and tracks concurrency.
func fakeProber(latencies map[string]time.Duration, running, peak *int32) Prober {
	return func(ctx context.Context, t Target) (time.Duration, error) {
		n := atomic.AddInt32(running, 1)
		for {
			p := atomic.LoadInt32(peak)
			if n <= p || atomic.CompareAndSwapInt32(peak, p, n) {
				break
			}
		}
		time.Sleep(20 * time.Millisecond)
		atomic.AddInt32(running, -1)
		d, ok := latencies[t.Host]
		if !ok {
			return 0, errors.New("connection refused")
		}
		return d, nil
	}
}

func targets(hosts ...string) []Target {
	var ts []Target
	for _, h := range hosts {
		ts = append(ts, Target{Raw: "tcp://" + h + ":22", Kind: "tcp", Host: h, Port: 22})
	}
	return ts
}

func TestRunKeepsOrderAndErrors(t *testing.T) {
	var running, peak int32
	lat := map[string]time.Duration{"a": 5 * time.Millisecond, "c": 9 * time.Millisecond}
	got := Run(context.Background(), targets("a", "b", "c"), fakeProber(lat, &running, &peak), 2)
	if len(got) != 3 {
		t.Fatalf("got %d results, want 3", len(got))
	}
	if got[0].Target.Host != "a" || got[1].Target.Host != "b" || got[2].Target.Host != "c" {
		t.Errorf("results not in target order: %+v", got)
	}
	if !got[0].Up || got[0].Latency != 5*time.Millisecond || got[0].Err != "" {
		t.Errorf("a: got %+v", got[0])
	}
	if got[1].Up || got[1].Err != "connection refused" {
		t.Errorf("b should be down with the error text, got %+v", got[1])
	}
}

func TestRunLimitsWorkers(t *testing.T) {
	var running, peak int32
	lat := map[string]time.Duration{}
	start := time.Now()
	Run(context.Background(), targets("1", "2", "3", "4", "5", "6"), fakeProber(lat, &running, &peak), 3)
	if peak > 3 {
		t.Errorf("%d probes ran at once, want at most 3", peak)
	}
	if peak < 2 {
		t.Errorf("probes ran one at a time (peak %d): use goroutines", peak)
	}
	if time.Since(start) > 100*time.Millisecond {
		t.Errorf("took %v: the probes should overlap", time.Since(start))
	}
}

func TestPercentile(t *testing.T) {
	ms := func(ns ...int) []time.Duration {
		var ds []time.Duration
		for _, n := range ns {
			ds = append(ds, time.Duration(n)*time.Millisecond)
		}
		return ds
	}
	data := ms(40, 10, 30, 20, 50)
	cases := []struct {
		p    float64
		want int
	}{{50, 30}, {95, 50}, {20, 10}, {21, 20}, {0, 10}, {100, 50}}
	for _, c := range cases {
		if got := Percentile(data, c.p); got != time.Duration(c.want)*time.Millisecond {
			t.Errorf("p%v = %v, want %dms", c.p, got, c.want)
		}
	}
	if data[0] != 40*time.Millisecond {
		t.Errorf("Percentile changed the caller's slice")
	}
	if Percentile(nil, 50) != 0 {
		t.Errorf("empty list should give 0")
	}
}

func TestSummary(t *testing.T) {
	rs := []Result{
		{Up: true, Latency: 12 * time.Millisecond},
		{Up: false, Err: "timeout"},
		{Up: true, Latency: 48*time.Millisecond + 300*time.Microsecond},
		{Up: true, Latency: 20 * time.Millisecond},
	}
	if got, want := Summary(rs), "3/4 up, p50 20ms, p95 48ms"; got != want {
		t.Errorf("got %q, want %q", got, want)
	}
	if got, want := Summary([]Result{{Up: false}, {Up: false}}), "0/2 up"; got != want {
		t.Errorf("got %q, want %q", got, want)
	}
}
@@file main.go
// healthprobe: check many services at once.
//
//	go run . -file targets.txt -workers 8 -timeout 3s
//	go run . -json > status.json
package main

import (
	"bufio"
	"context"
	"encoding/json"
	"flag"
	"fmt"
	"os"
	"strings"
	"time"
)

func main() {
	file := flag.String("file", "targets.txt", "file with one target per line (# for comments)")
	workers := flag.Int("workers", 8, "how many checks run at the same time")
	timeout := flag.Duration("timeout", 3*time.Second, "timeout for each check")
	asJSON := flag.Bool("json", false, "print JSON instead of a table")
	flag.Parse()

	targets, err := readTargets(*file)
	if err != nil {
		fmt.Fprintln(os.Stderr, "error:", err)
		os.Exit(1)
	}

	results := Run(context.Background(), targets, NetProber(*timeout), *workers)

	if *asJSON {
		enc := json.NewEncoder(os.Stdout)
		enc.SetIndent("", "  ")
		enc.Encode(results)
	} else {
		for _, r := range results {
			if r.Up {
				fmt.Printf("  UP    %-40s %v\n", r.Target.Raw, r.Latency.Round(time.Millisecond))
			} else {
				fmt.Printf("  DOWN  %-40s %s\n", r.Target.Raw, r.Err)
			}
		}
		fmt.Println(Summary(results))
	}

	for _, r := range results {
		if !r.Up {
			os.Exit(2) // something is down: let a cron job or pipeline notice
		}
	}
}

func readTargets(path string) ([]Target, error) {
	f, err := os.Open(path)
	if err != nil {
		return nil, err
	}
	defer f.Close()
	var targets []Target
	sc := bufio.NewScanner(f)
	for n := 1; sc.Scan(); n++ {
		line := strings.TrimSpace(strings.SplitN(sc.Text(), "#", 2)[0])
		if line == "" {
			continue
		}
		t, err := ParseTarget(line)
		if err != nil {
			return nil, fmt.Errorf("%s line %d: %w", path, n, err)
		}
		targets = append(targets, t)
	}
	return targets, sc.Err()
}
@@file targets.txt
# One target per line: tcp://host:port, http://host[/path] or https://host[:port][/path]
https://example.com              # a public web site
https://www.iana.org/help/example-domains
tcp://one.one.one.one:53         # Cloudflare DNS over TCP
tcp://localhost:22               # SSH on this machine (probably down on Windows)
http://localhost:8080/health     # your own service, if it's running

@@drill
Which `http.Client` field limits how long a whole request may take?
= Timeout
? The default client has no timeout.

@@text
🎉 You've written the kind of small, fast, dependency-free tool Go is famous for. Put it in a container, schedule it, and point it at the services you look after: it will notice the outage before your users do.

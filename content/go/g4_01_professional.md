@@lesson g4-professional Professional Go: Modules, Tests and Shipping
@@concepts modules, testing
@@text
## Modules and packages

A Go project is a **module**, described by `go.mod`:

```bash,norun
mkdir healthcheck && cd healthcheck
go mod init github.com/aiko/healthcheck   # the module path: usually where the code lives
go get github.com/spf13/cobra@latest      # add a dependency (recorded in go.mod and go.sum)
go mod tidy                               # add missing and remove unused dependencies
```

```text
healthcheck/
├── go.mod
├── go.sum              checksums of every dependency: commit it
├── main.go             package main
└── probe/
    ├── probe.go        package probe
    └── probe_test.go
```

Each folder is one **package**. `main.go` imports it as `"github.com/aiko/healthcheck/probe"` and calls `probe.Run(...)`: only capitalized names are visible. A folder called `internal/` can only be imported by code in the same module.

Go's standard library is big enough that many tools need no dependencies at all. Every dependency is code you trust, so check its maintainers, activity and licence. `govulncheck ./...` reports known vulnerabilities in the ones you use.

## The go command

| Command | Does |
|---|---|
| `go run .` | compile and run the package in this folder |
| `go build -o hc` | build an executable |
| `go test ./...` | run every test in the module |
| `go vet ./...` | find suspicious code (wrong Printf verbs, copied locks...) |
| `gofmt -w .` / `go fmt ./...` | format everything |
| `go doc strings.Cut` | show documentation |

## Tests

Tests live in `_test.go` files next to the code, and `go test` runs every `TestXxx` function. Go's style is the **table-driven test**:

```go,norun
package probe

import "testing"

func TestParseTarget(t *testing.T) {
	tests := []struct {
		in      string
		want    Target
		wantErr bool
	}{
		{"tcp://db01:5432", Target{Kind: "tcp", Host: "db01", Port: 5432}, false},
		{"https://example.jp/health", Target{Kind: "https", Host: "example.jp", Port: 443, Path: "/health"}, false},
		{"ftp://old", Target{}, true},
	}
	for _, tt := range tests {
		t.Run(tt.in, func(t *testing.T) {
			got, err := ParseTarget(tt.in)
			if (err != nil) != tt.wantErr {
				t.Fatalf("error = %v, wantErr %v", err, tt.wantErr)
			}
			if got != tt.want {
				t.Errorf("got %+v, want %+v", got, tt.want)
			}
		})
	}
}
```

Adding a case is one line. `t.Errorf` records a failure and continues; `t.Fatalf` stops that test. `go test -cover` shows how much code the tests run, and `func BenchmarkX(b *testing.B)` measures speed.

To test code that does I/O, pass the I/O in as a function or interface. The build project's `Run(targets, prober, workers)` takes the prober as a parameter, so the tests use a fake one, with no network.

## Generics

Since Go 1.18, functions and types can have type parameters:

```go
package main

import (
	"cmp"
	"fmt"
)

func MaxBy[T any, K cmp.Ordered](items []T, key func(T) K) T {
	best := items[0]
	for _, it := range items[1:] {
		if key(it) > key(best) {
			best = it
		}
	}
	return best
}

type Server struct {
	Name string
	CPU  int
}

func main() {
	servers := []Server{{"web01", 35}, {"web02", 92}, {"db01", 71}}
	fmt.Println(MaxBy(servers, func(s Server) int { return s.CPU }).Name)
}
```

The `slices` and `maps` packages are generic. Go culture uses generics sparingly: for containers and helpers, not everywhere.

## Shipping a binary

```bash,norun
go build -o healthcheck .                                  # for this machine
GOOS=linux GOARCH=amd64 go build -o healthcheck-linux .    # for a Linux server (from any OS)
GOOS=windows GOARCH=amd64 go build -o healthcheck.exe .    # for Windows
CGO_ENABLED=0 go build -ldflags="-s -w" -o hc .            # static and smaller: ideal for containers
```

In PowerShell, set the variables first: `$env:GOOS="linux"; go build`.

A static Go binary runs in a container with **nothing else** in it:

```dockerfile
FROM golang:1.25 AS build
WORKDIR /src
COPY . .
RUN CGO_ENABLED=0 go build -o /hc .

FROM gcr.io/distroless/static
COPY --from=build /hc /hc
ENTRYPOINT ["/hc"]
```

The final image is a few megabytes, with no shell or package manager for an attacker to use.

## Logging and flags

```go,norun
port := flag.Int("port", 8080, "port to listen on")
verbose := flag.Bool("v", false, "verbose output")
flag.Parse()

logger := slog.New(slog.NewJSONHandler(os.Stderr, nil))
logger.Info("starting", "port", *port, "verbose", *verbose)
// {"time":"...","level":"INFO","msg":"starting","port":8080,"verbose":false}
```

`flag` parses command-line options, and `log/slog` writes structured (JSON) logs that tools like Loki, Elasticsearch and CloudWatch can search.

@@quiz
Which file records the exact checksums of every dependency?
* `go.sum`
- `go.mod`
- `vendor.txt`
- `package-lock.json`
? Commit both go.mod and go.sum.

@@quiz
How do you build a Linux binary from a Windows machine?
* Set `GOOS=linux` (and `GOARCH`) and run `go build`
- You can't: you need a Linux machine
- Install a Linux compiler
- Use Docker only
? Cross-compiling is built into Go.

@@quiz
What does a table-driven test do?
* Runs the same checks over a list of input/expected cases, usually with `t.Run` per case
- Tests database tables
- Generates a table of results
- Skips slow tests
? It's the standard Go testing pattern.

@@quiz
Why does the build project's `Run` take the prober function as a parameter?
* So tests can pass a fake prober and run without a network
- To make it faster
- Go requires it for goroutines
- For logging
? Dependency injection with plain functions is idiomatic Go.

@@flashcards
`go mod init <path>` :: start a module
`go mod tidy` :: sync dependencies with the imports
`go test ./...` :: run all tests
`go vet ./...` :: find suspicious code
`t.Run(name, func(t *testing.T) {...})` :: a subtest
`GOOS=linux go build` :: cross-compile for Linux
`CGO_ENABLED=0` :: a fully static binary
`log/slog` :: structured logging

@@exercise Generic filter
Write a generic `Filter[T any](items []T, keep func(T) bool) []T` returning the items for which `keep` is true. It must work for both calls in `main`.
@@starter
package main

import (
	"fmt"
	"strings"
)

// TODO: Filter

func main() {
	ports := []int{22, 80, 443, 8080, 3389}
	hosts := []string{"web01", "db01", "web02"}
	fmt.Println(ports, hosts, strings.HasPrefix("", "")) // TODO: use Filter
}
@@expected
[8080 3389]
[web01 web02]
@@requires
[T any]
@@hint
`func Filter[T any](items []T, keep func(T) bool) []T { var out []T; for _, it := range items { if keep(it) { out = append(out, it) } }; return out }`. Then `fmt.Println(Filter(ports, func(p int) bool { return p >= 1024 }))` and the same for hosts starting with "web".
@@solution
package main

import (
	"fmt"
	"strings"
)

func Filter[T any](items []T, keep func(T) bool) []T {
	var out []T
	for _, it := range items {
		if keep(it) {
			out = append(out, it)
		}
	}
	return out
}

func main() {
	ports := []int{22, 80, 443, 8080, 3389}
	hosts := []string{"web01", "db01", "web02"}
	fmt.Println(Filter(ports, func(p int) bool { return p >= 1024 }))
	fmt.Println(Filter(hosts, func(h string) bool { return strings.HasPrefix(h, "web") }))
}

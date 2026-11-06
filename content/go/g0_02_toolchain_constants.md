@@lesson g0-toolchain How Go Works: the Toolchain, Zero Values and Constants
@@concepts variables, numbers
@@text
Go was designed at Google for large teams and large codebases: fast builds, one formatting style, simple rules, and a single static binary at the end. Knowing how the toolchain works explains many of Go's "strict" rules.

## From source to binary

```text
main.go ──go build──► compiler (type checks, optimizes) ──► linker ──► one static executable
```

- `go run .` compiles to a temporary binary and runs it; `go build` writes the binary.
- The binary contains your code, the libraries **and** the Go runtime (garbage collector, scheduler). Copy it to a server and run it: no interpreter, no dependencies. That's a big reason DevOps tools (Docker, Kubernetes, Terraform, Prometheus) are written in Go.
- Cross-compiling is one environment variable away: `GOOS=linux GOARCH=arm64 go build`.

## The strict compiler is a feature

- **Unused variables and imports are errors.** Dead code is a common source of bugs, so Go refuses it.
- `gofmt` defines the one true formatting; nobody argues about style.
- `go vet` finds suspicious code (wrong `Printf` verbs, copied locks, unreachable code).

## Packages

Every file starts with `package <name>`. The program entry point is `func main()` in `package main`. Names starting with a **capital letter** are exported (visible to other packages); lowercase names are private to the package. There are no `public`/`private` keywords.

## Zero values

Every type has a **zero value**, and declared variables always start with it. There's no "uninitialized" memory:

```go
package main

import "fmt"

type Server struct {
	Name  string
	Port  int
	TLS   bool
	Tags  []string
	Owner *string
}

func main() {
	var s Server
	fmt.Printf("%q %d %t %v %v\n", s.Name, s.Port, s.TLS, s.Tags == nil, s.Owner == nil)
	var counts map[string]int
	fmt.Println(counts["missing"], len(counts)) // reading a nil map is fine; writing panics
}
```

Good Go types are designed so the zero value is useful: a zero `sync.Mutex` is unlocked, a zero `bytes.Buffer` is empty and ready, a nil slice can be appended to.

## var, := and constants

```go
package main

import "fmt"

const Timeout = 30         // untyped constant: takes whatever numeric type it's used as
const Pi float64 = 3.14159 // typed constant

func main() {
	var retries int = 3 // explicit
	var host = "web01"  // type inferred
	port := 8080        // short declaration, only inside functions
	var ms int64 = Timeout * 1000
	var f float64 = Timeout / 7.0
	fmt.Println(retries, host, port, ms, f > 4)
}
```

Untyped constants have arbitrary precision until they're used, so `const Big = 1 << 100` is fine as long as you only use it in ways that fit.

## iota: enumerations

`iota` counts up inside a `const` block. Combined with a named type and a `String()` method it's Go's enum:

```go
package main

import "fmt"

type Level int

const (
	Debug Level = iota // 0
	Info               // 1
	Warn               // 2
	Error              // 3
)

func (l Level) String() string {
	switch l {
	case Debug:
		return "DEBUG"
	case Info:
		return "INFO"
	case Warn:
		return "WARN"
	case Error:
		return "ERROR"
	}
	return fmt.Sprintf("Level(%d)", int(l))
}

func main() {
	fmt.Println(Warn, Level(7))
	fmt.Println(Warn > Info)
}
```

Because `Level` has a `String()` method, `fmt` uses it automatically. The `stringer` tool can generate these methods.

@@quiz
Why does Go reject unused imports and variables?
* Unused code often hides bugs, and the rule keeps codebases clean
- The compiler can't handle them
- They slow down the program at runtime
- It's a gofmt setting
? It's deliberate, not a limitation.

@@quiz
What's in a Go binary built with `go build`?
- Only your code; Go must be installed to run it
* Your code, its libraries and the Go runtime, in one executable
- Bytecode for a virtual machine
- A script and an interpreter
? Copy it anywhere with the same OS and CPU and run it.

@@quiz
What does `var s []string` hold?
- An empty slice with capacity 10
* nil, which behaves like an empty slice (len 0, can append)
- An error
- A pointer to nothing that panics on len
? Nil slices are usable zero values.

@@quiz
How are names exported from a Go package?
- With the `export` keyword
- With `public`
* By starting them with a capital letter
- By listing them in go.mod
? `Parse` is exported, `parse` isn't.

@@drill
Which identifier counts up automatically inside a `const` block?
= iota
? Used for enumerations.

@@flashcards
`go run .` :: compile and run
`go build` :: produce a static binary
`GOOS` / `GOARCH` :: cross-compile target OS / CPU
`gofmt` :: the standard formatter
`go vet` :: finds suspicious code
Zero value :: the value every variable starts with
`:=` :: short variable declaration (inside functions)
Untyped constant :: arbitrary precision until used
`iota` :: counter inside a const block
Exported name :: starts with a capital letter

@@exercise Log levels with iota
Create `type Level int` with constants `Trace`, `Debug`, `Info`, `Warn`, `Error`, `Fatal` using `iota`. Give it:

- `String()` returning upper-case names (`"Level(9)"` for unknown values)
- `ParseLevel(s string) (Level, bool)`: case-insensitive; also accepts `"warning"` for `Warn`
- `Enabled(min Level) bool`: true if the level is at least `min`

`main` (given) prints the results.
@@starter
package main

import (
	"fmt"
	"strings"
)

type Level int

// TODO: constants with iota, String, ParseLevel and Enabled

func ParseLevel(s string) (Level, bool) {
	_ = strings.ToLower(s)
	return 0, false
}

func main() {
	for _, s := range []string{"info", "WARNING", "Fatal", "verbose"} {
		l, ok := ParseLevel(s)
		fmt.Println(s, "->", l, ok)
	}
	fmt.Println(Level(9))
	// fmt.Println(Debug.Enabled(Info), Error.Enabled(Info))
}
@@expected
info -> INFO true
WARNING -> WARN true
Fatal -> FATAL true
verbose -> TRACE false
Level(9)
false true
@@requires
iota
func (l Level) String() string
func (l Level) Enabled(
@@hint
Keep the names in a slice `var names = []string{"TRACE", "DEBUG", ...}` indexed by the level; `String` checks the range first. In `ParseLevel`, lower-case the input, treat `"warning"` as `"warn"`, then loop over the names with `strings.EqualFold`. Uncomment the last line of `main`.
@@solution
package main

import (
	"fmt"
	"strings"
)

type Level int

const (
	Trace Level = iota
	Debug
	Info
	Warn
	Error
	Fatal
)

var names = []string{"TRACE", "DEBUG", "INFO", "WARN", "ERROR", "FATAL"}

func (l Level) String() string {
	if l < 0 || int(l) >= len(names) {
		return fmt.Sprintf("Level(%d)", int(l))
	}
	return names[l]
}

func ParseLevel(s string) (Level, bool) {
	s = strings.ToLower(s)
	if s == "warning" {
		s = "warn"
	}
	for i, n := range names {
		if strings.EqualFold(n, s) {
			return Level(i), true
		}
	}
	return Trace, false
}

func (l Level) Enabled(min Level) bool { return l >= min }

func main() {
	for _, s := range []string{"info", "WARNING", "Fatal", "verbose"} {
		l, ok := ParseLevel(s)
		fmt.Println(s, "->", l, ok)
	}
	fmt.Println(Level(9))
	fmt.Println(Debug.Enabled(Info), Error.Enabled(Info))
}

@@exercise Byte sizes with typed constants
Define `type ByteSize int64` and constants `KB`, `MB`, `GB`, `TB` with `iota` and bit shifts (`KB ByteSize = 1 << (10 * (iota + 1))`). Write:

- `func (b ByteSize) String() string`: the largest unit that keeps the value ≥ 1, with one decimal (`"1.5 GB"`); below 1 KB print `"<n> B"`
- `ParseSize(s string) (ByteSize, error)`: accepts `"512"`, `"10KB"`, `"1.5 GB"`, `"2tb"` (optional space, case-insensitive unit, decimal number); errors look like `invalid size "x"`
@@starter
package main

import (
	"fmt"
	"strconv"
	"strings"
)

type ByteSize int64

// TODO: KB, MB, GB, TB constants

func (b ByteSize) String() string {
	return strconv.FormatInt(int64(b), 10)
}

func ParseSize(s string) (ByteSize, error) {
	_ = strings.TrimSpace(s)
	return 0, nil
}

func main() {
	for _, b := range []ByteSize{512, 1536, 5 << 20, 3 << 30 / 2, 2 << 40} {
		fmt.Println(int64(b), "=", b)
	}
	for _, s := range []string{"512", "10KB", "1.5 GB", "2tb", "lots", "5 PB"} {
		v, err := ParseSize(s)
		if err != nil {
			fmt.Println("error:", err)
			continue
		}
		fmt.Println(s, "=", int64(v))
	}
}
@@expected
512 = 512 B
1536 = 1.5 KB
5242880 = 5.0 MB
1610612736 = 1.5 GB
2199023255552 = 2.0 TB
512 = 512
10KB = 10240
1.5 GB = 1610612736
2tb = 2199023255552
error: invalid size "lots"
error: invalid size "5 PB"
@@requires
iota
@@hint
Loop over the units from the largest down: `for i := len(units) - 1; i >= 0; i-- { if b >= units[i].size { return fmt.Sprintf("%.1f %s", float64(b)/float64(units[i].size), units[i].name) } }`. To parse, split the string where the digits and dot end (`strings.IndexFunc(s, func(r rune) bool { return (r < '0' || r > '9') && r != '.' })`), parse the number with `strconv.ParseFloat`, and look up the trimmed, upper-cased unit.
@@solution
package main

import (
	"fmt"
	"strconv"
	"strings"
)

type ByteSize int64

const (
	KB ByteSize = 1 << (10 * (iota + 1))
	MB
	GB
	TB
)

var units = []struct {
	name string
	size ByteSize
}{{"KB", KB}, {"MB", MB}, {"GB", GB}, {"TB", TB}}

func (b ByteSize) String() string {
	for i := len(units) - 1; i >= 0; i-- {
		if b >= units[i].size {
			return fmt.Sprintf("%.1f %s", float64(b)/float64(units[i].size), units[i].name)
		}
	}
	return fmt.Sprintf("%d B", int64(b))
}

func ParseSize(s string) (ByteSize, error) {
	t := strings.TrimSpace(s)
	cut := strings.IndexFunc(t, func(r rune) bool { return (r < '0' || r > '9') && r != '.' })
	num, unit := t, ""
	if cut >= 0 {
		num, unit = t[:cut], strings.ToUpper(strings.TrimSpace(t[cut:]))
	}
	n, err := strconv.ParseFloat(num, 64)
	if err != nil {
		return 0, fmt.Errorf("invalid size %q", s)
	}
	mult := ByteSize(1)
	if unit != "" && unit != "B" {
		found := false
		for _, u := range units {
			if u.name == unit {
				mult, found = u.size, true
			}
		}
		if !found {
			return 0, fmt.Errorf("invalid size %q", s)
		}
	}
	return ByteSize(n * float64(mult)), nil
}

func main() {
	for _, b := range []ByteSize{512, 1536, 5 << 20, 3 << 30 / 2, 2 << 40} {
		fmt.Println(int64(b), "=", b)
	}
	for _, s := range []string{"512", "10KB", "1.5 GB", "2tb", "lots", "5 PB"} {
		v, err := ParseSize(s)
		if err != nil {
			fmt.Println("error:", err)
			continue
		}
		fmt.Println(s, "=", int64(v))
	}
}

@@lesson p7-testing Testing in Depth: Table Tests, Benchmarks, Fuzzing and Examples
@@concepts testing
@@text
Go's testing tools are built into the toolchain: no framework to choose, and every Go developer knows the conventions.

## The basics

Tests live next to the code in files ending in `_test.go`, in functions named `TestXxx(t *testing.T)`:

```go,norun
// port.go
func ParsePort(s string) (int, error) { ... }

// port_test.go
package netcfg

import "testing"

func TestParsePort(t *testing.T) {
	got, err := ParsePort("8080")
	if err != nil {
		t.Fatalf("unexpected error: %v", err) // stop this test
	}
	if got != 8080 {
		t.Errorf("ParsePort(\"8080\") = %d, want 8080", got) // record, continue
	}
}
```

```text
go test ./...                 # all packages
go test -run TestParsePort -v # one test, verbose
go test -race -cover ./...    # race detector + coverage
go test -count=1 ./...        # bypass the test cache
```

## Table-driven tests and subtests

The idiomatic shape: a slice of cases, one `t.Run` per case. Each subtest gets its own name, can be run alone (`-run TestParsePort/too_big`) and keeps going after a failure:

```go,norun
func TestParsePort(t *testing.T) {
	tests := []struct {
		name    string
		in      string
		want    int
		wantErr bool
	}{
		{"http", "80", 80, false},
		{"too big", "70000", 0, true},
		{"not a number", "http", 0, true},
	}
	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			t.Parallel() // safe: each iteration has its own tc (Go 1.22+)
			got, err := ParsePort(tc.in)
			if (err != nil) != tc.wantErr {
				t.Fatalf("err = %v, wantErr %v", err, tc.wantErr)
			}
			if got != tc.want {
				t.Errorf("got %d, want %d", got, tc.want)
			}
		})
	}
}
```

Useful helpers: `t.Helper()` (report failures at the caller's line), `t.Cleanup(f)`, `t.TempDir()` (auto-deleted), `t.Setenv`, `t.Context()` (Go 1.24, cancelled when the test ends). Test data files go in a `testdata/` directory, which the go tool ignores. **Golden files** store expected output there and are regenerated with a `-update` flag.

Compare complex values with `reflect.DeepEqual` or, in many projects, `github.com/google/go-cmp/cmp.Diff` for readable diffs.

## Benchmarks

```go,norun
func BenchmarkParsePort(b *testing.B) {
	for b.Loop() { // Go 1.24; older code uses for i := 0; i < b.N; i++
		ParsePort("8080")
	}
}
```

```text
go test -bench=. -benchmem          # ns/op, B/op, allocs/op
go test -bench=. -count=10 > new.txt && benchstat old.txt new.txt
```

## Fuzzing

Fuzz tests generate inputs to find crashes and broken invariants:

```go,norun
func FuzzReverse(f *testing.F) {
	f.Add("hello") // seed corpus
	f.Fuzz(func(t *testing.T, s string) {
		r := Reverse(s)
		if Reverse(r) != s {
			t.Errorf("double reverse of %q = %q", s, Reverse(r))
		}
		if utf8.ValidString(s) && !utf8.ValidString(r) {
			t.Errorf("reverse produced invalid UTF-8: %q", r)
		}
	})
}
```

`go test -fuzz=FuzzReverse` runs until it finds a failure, saving the input in `testdata/fuzz/` so it becomes a regression test forever.

## Examples are tests too

```go,norun
func ExampleParsePort() {
	p, _ := ParsePort("443")
	fmt.Println(p)
	// Output: 443
}
```

`go test` checks the output, and `go doc` / pkg.go.dev show it as documentation.

## Testing time and concurrency

`testing/synctest` (Go 1.25) runs code in a bubble with a fake clock: `time.Sleep(time.Hour)` returns instantly once every goroutine is blocked, so timeout and retry logic can be tested quickly and deterministically. Otherwise, inject a clock or sleep function, as you did in the previous module.

@@quiz
What's the difference between `t.Errorf` and `t.Fatalf`?
* Errorf records a failure and continues; Fatalf stops the current test
- Fatalf stops all tests in the package
- Errorf only logs
- None
? Use Fatalf when continuing makes no sense (e.g. a nil result).

@@quiz
How do you run only the "too big" case of `TestParsePort`?
- go test TestParsePort too big
* go test -run 'TestParsePort/too_big'
- go test -case too_big
- You can't
? Spaces in subtest names become underscores.

@@quiz
What does a fuzz test do with a failing input it finds?
- Prints it and forgets it
* Saves it under testdata/fuzz so it's rerun as a regular test
- Uploads it
- Ignores it
? Every crash becomes a permanent regression test.

@@drill
Which test function prefix makes Go run a fuzz test?
= Fuzz
= FuzzXxx
? func FuzzParse(f *testing.F).

@@flashcards
`_test.go` :: test file, excluded from normal builds
`t.Run(name, f)` :: subtest
Table-driven test :: slice of cases + loop
`t.Helper()` :: report failures at the caller
`t.TempDir()` :: auto-cleaned temp directory
`testdata/` :: fixtures, ignored by the go tool
`b.Loop()` :: benchmark loop (Go 1.24)
`-benchmem` :: show allocations per op
`FuzzXxx(f *testing.F)` :: fuzz test
`// Output:` :: checked example output

@@exercise Table tests for an uptime parser
Normal `_test.go` files can't run here, so the starter includes a tiny `T` that behaves like `testing.T` (`Errorf`, `Fatalf`-like `Fail`, `Run`).

1. Implement `ParseUptime(s string) (time.Duration, error)`: like `time.ParseDuration` but also accepts `d` (24h) and only the units `d`, `h`, `m`, `s`, each preceded by a number. Errors: `invalid uptime "<s>"`.
2. Write `TestParseUptime(t *T)` as a **table-driven test** with these cases (name, input, want, wantErr):
   `seconds "45s" 45s`, `minutes "5m" 5m`, `mixed "1h30m" 1h30m`, `days "2d" 48h`, `everything "1d2h3m4s" 26h3m4s`, `empty "" error`, `bad unit "3x" error`, `missing number "d" error`.

All subtests must pass.
@@starter
package main

import (
	"fmt"
	"strconv"
	"time"
)

type T struct {
	name   string
	failed bool
	logs   []string
}

func (t *T) Errorf(format string, args ...any) {
	t.failed = true
	t.logs = append(t.logs, fmt.Sprintf(format, args...))
}

func (t *T) Run(name string, f func(t *T)) {
	sub := &T{name: t.name + "/" + name}
	f(sub)
	status := "PASS"
	if sub.failed {
		status = "FAIL"
		t.failed = true
	}
	fmt.Printf("--- %s: %s\n", status, sub.name)
	for _, l := range sub.logs {
		fmt.Println("    " + l)
	}
}

func ParseUptime(s string) (time.Duration, error) {
	_ = strconv.Atoi
	return 0, nil
}

func TestParseUptime(t *T) {
	// TODO: table of cases + t.Run for each
}

func main() {
	t := &T{name: "TestParseUptime"}
	TestParseUptime(t)
	if t.failed {
		fmt.Println("FAIL")
	} else {
		fmt.Println("PASS")
	}
}
@@expected
--- PASS: TestParseUptime/seconds
--- PASS: TestParseUptime/minutes
--- PASS: TestParseUptime/mixed
--- PASS: TestParseUptime/days
--- PASS: TestParseUptime/everything
--- PASS: TestParseUptime/empty
--- PASS: TestParseUptime/bad_unit
--- PASS: TestParseUptime/missing_number
PASS
@@requires
t.Run(
range tests
@@hint
Name the subtests like Go does, replacing spaces with underscores (`strings.ReplaceAll(tc.name, " ", "_")`), or just use underscores in the table. Parse by scanning: collect digits into a number, then the next byte must be a unit; reject a unit with no digits, unknown units and leftover digits at the end.
@@solution
package main

import (
	"fmt"
	"strconv"
	"strings"
	"time"
)

type T struct {
	name   string
	failed bool
	logs   []string
}

func (t *T) Errorf(format string, args ...any) {
	t.failed = true
	t.logs = append(t.logs, fmt.Sprintf(format, args...))
}

func (t *T) Run(name string, f func(t *T)) {
	sub := &T{name: t.name + "/" + name}
	f(sub)
	status := "PASS"
	if sub.failed {
		status = "FAIL"
		t.failed = true
	}
	fmt.Printf("--- %s: %s\n", status, sub.name)
	for _, l := range sub.logs {
		fmt.Println("    " + l)
	}
}

var units = map[byte]time.Duration{'d': 24 * time.Hour, 'h': time.Hour, 'm': time.Minute, 's': time.Second}

func ParseUptime(s string) (time.Duration, error) {
	bad := fmt.Errorf("invalid uptime %q", s)
	if s == "" {
		return 0, bad
	}
	var total time.Duration
	digits := ""
	for i := 0; i < len(s); i++ {
		c := s[i]
		if c >= '0' && c <= '9' {
			digits += string(c)
			continue
		}
		unit, ok := units[c]
		if !ok || digits == "" {
			return 0, bad
		}
		n, err := strconv.Atoi(digits)
		if err != nil {
			return 0, bad
		}
		total += time.Duration(n) * unit
		digits = ""
	}
	if digits != "" {
		return 0, bad
	}
	return total, nil
}

func TestParseUptime(t *T) {
	tests := []struct {
		name    string
		in      string
		want    time.Duration
		wantErr bool
	}{
		{"seconds", "45s", 45 * time.Second, false},
		{"minutes", "5m", 5 * time.Minute, false},
		{"mixed", "1h30m", 90 * time.Minute, false},
		{"days", "2d", 48 * time.Hour, false},
		{"everything", "1d2h3m4s", 26*time.Hour + 3*time.Minute + 4*time.Second, false},
		{"empty", "", 0, true},
		{"bad unit", "3x", 0, true},
		{"missing number", "d", 0, true},
	}
	for _, tc := range tests {
		t.Run(strings.ReplaceAll(tc.name, " ", "_"), func(t *T) {
			got, err := ParseUptime(tc.in)
			if (err != nil) != tc.wantErr {
				t.Errorf("ParseUptime(%q) err = %v, wantErr %v", tc.in, err, tc.wantErr)
				return
			}
			if got != tc.want {
				t.Errorf("ParseUptime(%q) = %v, want %v", tc.in, got, tc.want)
			}
		})
	}
}

func main() {
	t := &T{name: "TestParseUptime"}
	TestParseUptime(t)
	if t.failed {
		fmt.Println("FAIL")
	} else {
		fmt.Println("PASS")
	}
}

@@exercise Property-based checks, the fuzzing way
A fuzzer feeds random inputs and checks **properties** that must always hold. Do the same by hand with a seeded generator:

1. The starter's `Reverse` reverses **bytes**, which breaks multi-byte characters. Fix it to reverse **runes**.
2. Write `check(s string) error` that verifies three properties: `Reverse(Reverse(s)) == s`, the result is valid UTF-8, and `utf8.RuneCountInString` is unchanged.
3. Generate 1000 random strings (length 0–12) from the alphabet `"aZ9 -é日本🚀"` using `rand.New(rand.NewPCG(1, 2))`, and count failures.
@@starter
package main

import (
	"fmt"
	"math/rand/v2"
	"unicode/utf8"
)

func Reverse(s string) string {
	b := []byte(s)
	for i, j := 0, len(b)-1; i < j; i, j = i+1, j-1 {
		b[i], b[j] = b[j], b[i]
	}
	return string(b)
}

func main() {
	_ = rand.New(rand.NewPCG(1, 2))
	_ = utf8.ValidString
	fmt.Println(Reverse("東京"))
}
@@expected
京東
checked 1000 inputs, failures: 0
@@requires
rand.NewPCG(1, 2)
utf8.ValidString
@@hint
`r := []rune(s)`, swap in place, `return string(r)`. Generator: `alphabet := []rune("aZ9 -é日本🚀")`; for each input, `n := rng.IntN(13)` and pick `alphabet[rng.IntN(len(alphabet))]` n times. Print `Reverse("東京")` first.
@@solution
package main

import (
	"errors"
	"fmt"
	"math/rand/v2"
	"unicode/utf8"
)

func Reverse(s string) string {
	r := []rune(s)
	for i, j := 0, len(r)-1; i < j; i, j = i+1, j-1 {
		r[i], r[j] = r[j], r[i]
	}
	return string(r)
}

func check(s string) error {
	r := Reverse(s)
	if Reverse(r) != s {
		return fmt.Errorf("double reverse of %q = %q", s, Reverse(r))
	}
	if !utf8.ValidString(r) {
		return errors.New("invalid UTF-8")
	}
	if utf8.RuneCountInString(r) != utf8.RuneCountInString(s) {
		return errors.New("rune count changed")
	}
	return nil
}

func main() {
	fmt.Println(Reverse("東京"))
	rng := rand.New(rand.NewPCG(1, 2))
	alphabet := []rune("aZ9 -é日本🚀")
	failures := 0
	for range 1000 {
		n := rng.IntN(13)
		r := make([]rune, n)
		for i := range r {
			r[i] = alphabet[rng.IntN(len(alphabet))]
		}
		if err := check(string(r)); err != nil {
			failures++
		}
	}
	fmt.Printf("checked 1000 inputs, failures: %d\n", failures)
}

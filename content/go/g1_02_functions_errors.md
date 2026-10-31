@@lesson g1-functions-errors Functions and Errors
@@concepts functions, errors
@@text
## Functions

```go
package main

import "fmt"

func toGB(bytes int64) float64 {
	return float64(bytes) / (1 << 30)
}

// Several parameters of the same type can share it; several results are normal.
func divmod(a, b int) (int, int) {
	return a / b, a % b
}

func main() {
	fmt.Printf("%.1f GB\n", toGB(8589934592))
	q, r := divmod(17, 5)
	fmt.Println(q, r)
	_, rem := divmod(10, 3) // _ ignores a result
	fmt.Println(rem)
}
```

Names that start with a **capital letter** are exported (visible from other packages): `fmt.Println`, `strings.Split`. Lower-case names are private to their package.

Functions are values: `check := func(h string) bool { return h != "" }`. Closures capture variables from around them.

## Errors are values

Go has no exceptions for normal errors. A function that can fail returns an `error` as its **last result**, and the caller checks it right away:

```go
package main

import (
	"errors"
	"fmt"
	"strconv"
)

func parsePort(s string) (int, error) {
	p, err := strconv.Atoi(s)
	if err != nil {
		return 0, fmt.Errorf("port %q is not a number: %w", s, err)
	}
	if p < 1 || p > 65535 {
		return 0, errors.New("port out of range")
	}
	return p, nil
}

func main() {
	for _, s := range []string{"443", "http", "70000"} {
		p, err := parsePort(s)
		if err != nil {
			fmt.Println("error:", err)
			continue
		}
		fmt.Println("port", p)
	}
}
```

- `nil` means "no error".
- `errors.New("...")` makes a simple error; `fmt.Errorf("...: %w", err)` **wraps** another error, adding context while keeping the original.
- Error messages start lower-case and have no full stop, because they get chained: `open config: permission denied`.

`if err != nil { return ..., err }` appears constantly in Go. It's verbose, but every failure path is visible.

## Checking for specific errors

```go
package main

import (
	"errors"
	"fmt"
	"io/fs"
	"os"
)

var ErrNotFound = errors.New("host not found") // a "sentinel" error to compare against

func lookup(host string) (string, error) {
	if host != "web01" {
		return "", fmt.Errorf("lookup %s: %w", host, ErrNotFound)
	}
	return "10.0.0.11", nil
}

func main() {
	_, err := lookup("db09")
	fmt.Println(err)
	fmt.Println(errors.Is(err, ErrNotFound)) // true, even though it's wrapped

	_, err = os.ReadFile("does-not-exist.txt")
	fmt.Println(errors.Is(err, fs.ErrNotExist))
}
```

`errors.Is` looks through the wrapping. `errors.As(err, &target)` extracts a specific error *type*.

## defer

`defer` runs a call when the surrounding function returns, however it returns. It's how Go closes files and releases locks:

```go
package main

import "fmt"

func backup() {
	fmt.Println("lock")
	defer fmt.Println("unlock") // runs last
	fmt.Println("copying files")
}

func main() {
	backup()
}
```

```go,norun
f, err := os.Open("app.log")
if err != nil {
	return err
}
defer f.Close() // closed whatever happens below
```

## panic

`panic` stops the program with a stack trace. It's for **bugs** (an impossible state, an index out of range), not for errors you expect, like a missing file or bad input. Those are `error` values.

@@quiz
How does a Go function usually report failure?
* It returns an `error` as its last result, and the caller checks `if err != nil`
- It throws an exception
- It returns -1
- It calls `panic`
? Errors are ordinary values in Go.

@@quiz
What does `fmt.Errorf("read config: %w", err)` do?
* Makes a new error with context that wraps the original, so `errors.Is` can still find it
- Prints the error
- Ignores the error
- Panics
? `%w` wraps; `%v` would only copy the text.

@@quiz
When does a deferred call run?
* When the surrounding function returns, even after an early return
- Immediately
- When the program ends
- After one second
? Deferred calls run in last-in, first-out order.

@@quiz
Which identifier is exported from a Go package?
* `ParsePort`
- `parsePort`
- `_parsePort`
- `parse_port`
? A capital first letter means exported.

@@flashcards
`func f(a, b int) (int, error)` :: two parameters, two results
`if err != nil { return err }` :: the standard error check
`errors.New("msg")` :: a simple error
`fmt.Errorf("ctx: %w", err)` :: wrap an error with context
`errors.Is(err, ErrX)` :: is ErrX anywhere in the chain?
`defer f.Close()` :: run when the function returns
`panic` :: for bugs, not expected errors
Capitalized name :: exported from the package

@@exercise Parse key=value
Write `parseLine(line string) (string, int, error)` that splits `"timeout = 30"` into the key and the number, trimming spaces. Return an error `bad line: <line>` if there's no `=` or the value isn't a whole number. The given `main` prints the results.
@@starter
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func parseLine(line string) (string, int, error) {
	_, _ = strconv.Atoi, strings.Cut // remove this line when you use them
	return "", 0, nil                // TODO: not done yet
}

func main() {
	for _, l := range []string{"timeout = 30", "retries=3", "verbose", "port = eighty"} {
		k, v, err := parseLine(l)
		if err != nil {
			fmt.Println(err)
			continue
		}
		fmt.Printf("%s -> %d\n", k, v)
	}
}
@@expected
timeout -> 30
retries -> 3
bad line: verbose
bad line: port = eighty
@@requires
fmt.Errorf
@@hint
`key, value, found := strings.Cut(line, "=")` splits at the first `=`. If `!found`, or `strconv.Atoi(strings.TrimSpace(value))` fails, `return "", 0, fmt.Errorf("bad line: %s", line)`.
@@solution
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func parseLine(line string) (string, int, error) {
	key, value, found := strings.Cut(line, "=")
	if !found {
		return "", 0, fmt.Errorf("bad line: %s", line)
	}
	n, err := strconv.Atoi(strings.TrimSpace(value))
	if err != nil {
		return "", 0, fmt.Errorf("bad line: %s", line)
	}
	return strings.TrimSpace(key), n, nil
}

func main() {
	for _, l := range []string{"timeout = 30", "retries=3", "verbose", "port = eighty"} {
		k, v, err := parseLine(l)
		if err != nil {
			fmt.Println(err)
			continue
		}
		fmt.Printf("%s -> %d\n", k, v)
	}
}

@@exercise Wrap and check
`lookup` fails with a wrapped `ErrNotFound` for unknown hosts. In `main`, for each host print the IP, or `<host>: not in inventory` when `errors.Is(err, ErrNotFound)`.
@@starter
package main

import (
	"errors"
	"fmt"
)

var ErrNotFound = errors.New("not found")

var inventory = map[string]string{"web01": "10.0.0.11", "db01": "10.0.0.21"}

func lookup(host string) (string, error) {
	ip, ok := inventory[host]
	if !ok {
		return "", fmt.Errorf("lookup %s: %w", host, ErrNotFound)
	}
	return ip, nil
}

func main() {
	for _, h := range []string{"web01", "dns01", "db01"} {
		ip, _ := lookup(h)
		fmt.Println(ip) // TODO: handle the error
	}
}
@@expected
10.0.0.11
dns01: not in inventory
10.0.0.21
@@requires
errors.Is
@@hint
`ip, err := lookup(h); if errors.Is(err, ErrNotFound) { fmt.Printf("%s: not in inventory\n", h); continue }`
@@solution
package main

import (
	"errors"
	"fmt"
)

var ErrNotFound = errors.New("not found")

var inventory = map[string]string{"web01": "10.0.0.11", "db01": "10.0.0.21"}

func lookup(host string) (string, error) {
	ip, ok := inventory[host]
	if !ok {
		return "", fmt.Errorf("lookup %s: %w", host, ErrNotFound)
	}
	return ip, nil
}

func main() {
	for _, h := range []string{"web01", "dns01", "db01"} {
		ip, err := lookup(h)
		if errors.Is(err, ErrNotFound) {
			fmt.Printf("%s: not in inventory\n", h)
			continue
		}
		fmt.Println(ip)
	}
}

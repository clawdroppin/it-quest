@@lesson d6-errors Errors in Depth: Wrapping, errors.Is/As, Joined Errors and panic
@@concepts errors
@@text
Go treats errors as ordinary **values** returned alongside results. That's verbose, but it makes every failure path visible. Doing it well means adding context as errors travel up, while keeping them inspectable.

## Wrapping with %w

```go
package main

import (
	"errors"
	"fmt"
	"io/fs"
	"os"
)

func loadConfig(path string) error {
	_, err := os.ReadFile(path)
	if err != nil {
		return fmt.Errorf("load config %s: %w", path, err) // %w wraps: keeps the original inside
	}
	return nil
}

func main() {
	err := loadConfig("/nope/app.toml")
	fmt.Println(err != nil)
	fmt.Println(errors.Is(err, fs.ErrNotExist)) // true: Is looks through the wrapping chain
}
```

Each layer adds **what it was doing**, giving messages like `start server: load config /etc/app.toml: open /etc/app.toml: permission denied`. Use `%v` instead of `%w` when you deliberately **don't** want callers to depend on the inner error (e.g. hiding a database driver's errors behind your own API).

## Three kinds of errors

1. **Sentinel errors**: package-level values to compare against: `io.EOF`, `sql.ErrNoRows`, `fs.ErrNotExist`. Check with `errors.Is(err, ErrNotFound)`, never with `==` (wrapping breaks `==`).
2. **Error types**: structs carrying details. Extract with `errors.As`:

```go
package main

import (
	"errors"
	"fmt"
)

type HTTPError struct {
	Status int
	URL    string
}

func (e *HTTPError) Error() string { return fmt.Sprintf("%s: HTTP %d", e.URL, e.Status) }

func fetch(url string) error {
	return fmt.Errorf("fetch inventory: %w", &HTTPError{503, url})
}

func main() {
	err := fetch("https://cmdb.local/api")
	var he *HTTPError
	if errors.As(err, &he) && he.Status >= 500 {
		fmt.Println("retryable:", he.Status, "|", err)
	}
}
```

3. **Opaque errors**: just a message. Fine when callers only need to know "it failed".

Go 1.26 added a generic `errors.AsType[*HTTPError](err)` that returns the value directly, avoiding the pointer-to-pointer dance.

## Joining errors

`errors.Join(errs...)` combines several errors into one (nil if all are nil). `errors.Is` and `errors.As` search all of them, which is ideal for validation and cleanup:

```go
package main

import (
	"errors"
	"fmt"
)

var ErrRequired = errors.New("required")

func validate(name, email string) error {
	var errs []error
	if name == "" {
		errs = append(errs, fmt.Errorf("name: %w", ErrRequired))
	}
	if email == "" {
		errs = append(errs, fmt.Errorf("email: %w", ErrRequired))
	}
	return errors.Join(errs...)
}

func main() {
	err := validate("", "")
	fmt.Println(err)
	fmt.Println(errors.Is(err, ErrRequired), validate("ana", "a@x") == nil)
}
```

Custom error types can implement `Unwrap() error` (one inner error) or `Unwrap() []error` (several), and `Is(target error) bool` for custom matching.

## Handle once

An error should be **either** returned (with context) **or** handled (logged, turned into a response, retried), not both. Logging and returning produces the same failure in the logs many times.

## panic and recover

`panic` is for **programmer errors** and impossible states (index out of range, nil map write, a broken invariant), not for expected failures. A panic unwinds the stack running deferred functions; `recover()` inside a deferred function stops it. Servers use this at the top of each request so one bug doesn't crash the process:

```go
package main

import "fmt"

func safely(name string, f func()) (err error) {
	defer func() {
		if r := recover(); r != nil {
			err = fmt.Errorf("%s panicked: %v", name, r)
		}
	}()
	f()
	return nil
}

func main() {
	fmt.Println(safely("job1", func() {}))
	fmt.Println(safely("job2", func() { var m map[string]int; m["x"] = 1 }))
}
```

Note the **named result** `err`: a deferred function can change what the function returns.

@@quiz
Why use `errors.Is(err, ErrNotFound)` instead of `err == ErrNotFound`?
* `errors.Is` looks through wrapped errors; `==` only matches the outermost value
- `==` doesn't compile for errors
- `errors.Is` is faster
- They're identical
? Wrapping with %w breaks plain equality.

@@quiz
What's the difference between `%w` and `%v` in `fmt.Errorf`?
- None
* `%w` wraps the error so callers can inspect it with Is/As; `%v` only includes its text
- `%v` wraps and `%w` doesn't
- `%w` adds a stack trace
? Use %v to hide implementation details.

@@quiz
When is `panic` appropriate?
- For any error
- For file-not-found
* For programmer errors and impossible states, not expected failures
- Instead of returning errors from main
? Expected failures are returned as error values.

@@quiz
What does "handle errors once" mean?
* Either return the error with context or handle it (log/respond), not both
- Check every error exactly once
- Only the main function handles errors
- Never wrap errors
? Logging and returning creates duplicate log entries.

@@flashcards
`fmt.Errorf("...: %w", err)` :: wrap with context
Sentinel error :: package-level error value to compare with errors.Is
`errors.Is` :: matches anywhere in the wrap chain
`errors.As` :: extracts a typed error from the chain
`errors.Join` :: combine several errors
`Unwrap() error` :: custom wrapping
panic :: programmer errors / impossible states
`recover()` :: stop a panic inside a deferred function
Named result :: lets defer change the returned error

@@exercise A typed error hierarchy for a backup tool
Write:

- sentinel `ErrNotFound = errors.New("not found")`
- `type QuotaError struct { Used, Limit int64 }` whose `Error()` is `quota exceeded: <used>/<limit> bytes`
- `type RetryableError struct { Err error; After time.Duration }` with `Error()` = `<inner> (retry in <after>)` and `Unwrap() error`
- `Upload(name string, size int64) error` returning (wrapped with `upload <name>: `): `ErrNotFound` for names starting with `missing`, a `*QuotaError{size, 1000}` when size > 1000, a `*RetryableError{errors.New("connection reset"), 2 * time.Second}` for names starting with `flaky`, else nil
- `Classify(err error) string`: `"ok"`, `"skip: <err>"` for not found, `"fail: over by <n> bytes"` for quota errors, `"retry after <d>"` for retryable errors, else `"fail: <err>"`
@@starter
package main

import (
	"errors"
	"fmt"
	"time"
)

func Upload(name string, size int64) error {
	_ = time.Second
	_ = errors.New
	return nil
}

func Classify(err error) string { return "ok" }

func main() {
	jobs := []struct {
		name string
		size int64
	}{{"db.dump", 400}, {"missing.tar", 10}, {"video.mkv", 4096}, {"flaky-share.zip", 50}}
	for _, j := range jobs {
		err := Upload(j.name, j.size)
		fmt.Printf("%-16s %s\n", j.name, Classify(err))
	}
	fmt.Println(Classify(fmt.Errorf("outer: %w", errors.New("disk on fire"))))
}
@@expected
db.dump          ok
missing.tar      skip: upload missing.tar: not found
video.mkv        fail: over by 3096 bytes
flaky-share.zip  retry after 2s
fail: outer: disk on fire
@@requires
errors.Is(
errors.As(
Unwrap() error
@@hint
`Classify`: `if err == nil { return "ok" }`, then `errors.Is(err, ErrNotFound)`, then `var qe *QuotaError; if errors.As(err, &qe) {...}`, then the same for `*RetryableError`.
@@solution
package main

import (
	"errors"
	"fmt"
	"strings"
	"time"
)

var ErrNotFound = errors.New("not found")

type QuotaError struct{ Used, Limit int64 }

func (e *QuotaError) Error() string {
	return fmt.Sprintf("quota exceeded: %d/%d bytes", e.Used, e.Limit)
}

type RetryableError struct {
	Err   error
	After time.Duration
}

func (e *RetryableError) Error() string { return fmt.Sprintf("%v (retry in %v)", e.Err, e.After) }
func (e *RetryableError) Unwrap() error { return e.Err }

func Upload(name string, size int64) error {
	var err error
	switch {
	case strings.HasPrefix(name, "missing"):
		err = ErrNotFound
	case size > 1000:
		err = &QuotaError{size, 1000}
	case strings.HasPrefix(name, "flaky"):
		err = &RetryableError{errors.New("connection reset"), 2 * time.Second}
	default:
		return nil
	}
	return fmt.Errorf("upload %s: %w", name, err)
}

func Classify(err error) string {
	if err == nil {
		return "ok"
	}
	if errors.Is(err, ErrNotFound) {
		return "skip: " + err.Error()
	}
	var qe *QuotaError
	if errors.As(err, &qe) {
		return fmt.Sprintf("fail: over by %d bytes", qe.Used-qe.Limit)
	}
	var re *RetryableError
	if errors.As(err, &re) {
		return fmt.Sprintf("retry after %v", re.After)
	}
	return "fail: " + err.Error()
}

func main() {
	jobs := []struct {
		name string
		size int64
	}{{"db.dump", 400}, {"missing.tar", 10}, {"video.mkv", 4096}, {"flaky-share.zip", 50}}
	for _, j := range jobs {
		err := Upload(j.name, j.size)
		fmt.Printf("%-16s %s\n", j.name, Classify(err))
	}
	fmt.Println(Classify(fmt.Errorf("outer: %w", errors.New("disk on fire"))))
}

@@exercise Validate everything, recover from panics
Write:

- `ValidateUser(u User) error` that checks **all** rules and returns `errors.Join` of every problem (nil if valid): `name: required` (empty name), `email: invalid` (no `@`), `age: must be 18-120`, and for each role not in `admin|operator|viewer`: `role <r>: unknown`. Wrap a sentinel: `name: required` must satisfy `errors.Is(err, ErrRequired)`
- `RunJobs(jobs map[string]func() error) []string`: runs jobs in sorted name order; each result line is `<name>: ok`, `<name>: error: <err>` or `<name>: panic: <value>`; a panicking job must not stop the others (use `recover` with a named result)
@@starter
package main

import (
	"errors"
	"fmt"
	"maps"
	"slices"
	"strings"
)

var ErrRequired = errors.New("required")

type User struct {
	Name  string
	Email string
	Age   int
	Roles []string
}

func ValidateUser(u User) error {
	_ = strings.Contains
	return nil
}

func RunJobs(jobs map[string]func() error) []string {
	_ = slices.Sorted(maps.Keys(jobs))
	return nil
}

func main() {
	err := ValidateUser(User{Email: "ana.example.com", Age: 15, Roles: []string{"admin", "root", "god"}})
	fmt.Println(err)
	fmt.Println(errors.Is(err, ErrRequired), ValidateUser(User{"ana", "ana@corp", 30, []string{"viewer"}}) == nil)
	var hosts []string
	results := RunJobs(map[string]func() error{
		"backup":  func() error { return nil },
		"cleanup": func() error { return errors.New("disk busy") },
		"report":  func() error { fmt.Sprint(hosts[3]); return nil },
		"audit":   func() error { panic("unexpected state") },
	})
	for _, r := range results {
		fmt.Println(r)
	}
}
@@expected
name: required
email: invalid
age: must be 18-120
role root: unknown
role god: unknown
true true
audit: panic: unexpected state
backup: ok
cleanup: error: disk busy
report: panic: runtime error: index out of range [3] with length 0
@@requires
errors.Join
recover()
@@hint
Write a helper `func run(f func() error) (err error, panicked any)` with `defer func() { if r := recover(); r != nil { panicked = r } }()`. A runtime panic's value is a `runtime.Error`, and printing it with `%v` gives the message.
@@solution
package main

import (
	"errors"
	"fmt"
	"maps"
	"slices"
	"strings"
)

var ErrRequired = errors.New("required")

type User struct {
	Name  string
	Email string
	Age   int
	Roles []string
}

var knownRoles = []string{"admin", "operator", "viewer"}

func ValidateUser(u User) error {
	var errs []error
	if u.Name == "" {
		errs = append(errs, fmt.Errorf("name: %w", ErrRequired))
	}
	if !strings.Contains(u.Email, "@") {
		errs = append(errs, errors.New("email: invalid"))
	}
	if u.Age < 18 || u.Age > 120 {
		errs = append(errs, errors.New("age: must be 18-120"))
	}
	for _, r := range u.Roles {
		if !slices.Contains(knownRoles, r) {
			errs = append(errs, fmt.Errorf("role %s: unknown", r))
		}
	}
	return errors.Join(errs...)
}

func run(f func() error) (err error, panicked any) {
	defer func() {
		if r := recover(); r != nil {
			panicked = r
		}
	}()
	return f(), nil
}

func RunJobs(jobs map[string]func() error) []string {
	var out []string
	for _, name := range slices.Sorted(maps.Keys(jobs)) {
		err, p := run(jobs[name])
		switch {
		case p != nil:
			out = append(out, fmt.Sprintf("%s: panic: %v", name, p))
		case err != nil:
			out = append(out, fmt.Sprintf("%s: error: %v", name, err))
		default:
			out = append(out, name+": ok")
		}
	}
	return out
}

func main() {
	err := ValidateUser(User{Email: "ana.example.com", Age: 15, Roles: []string{"admin", "root", "god"}})
	fmt.Println(err)
	fmt.Println(errors.Is(err, ErrRequired), ValidateUser(User{"ana", "ana@corp", 30, []string{"viewer"}}) == nil)
	var hosts []string
	results := RunJobs(map[string]func() error{
		"backup":  func() error { return nil },
		"cleanup": func() error { return errors.New("disk busy") },
		"report":  func() error { fmt.Sprint(hosts[3]); return nil },
		"audit":   func() error { panic("unexpected state") },
	})
	for _, r := range results {
		fmt.Println(r)
	}
}

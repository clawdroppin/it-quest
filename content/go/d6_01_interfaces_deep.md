@@lesson d6-interfaces Interfaces in Depth: Implicit Satisfaction, Embedding and Type Switches
@@concepts objects
@@text
Interfaces are the heart of Go's design. They're small, **satisfied implicitly** (no `implements` keyword), and usually defined by the code that **uses** them, not by the code that implements them.

## Implicit satisfaction

```go
package main

import (
	"fmt"
	"strings"
)

type Notifier interface {
	Notify(msg string) error
}

type Slack struct{ Channel string }

func (s Slack) Notify(msg string) error {
	fmt.Println("slack", s.Channel+":", msg)
	return nil
}

type Recorder struct{ Sent []string }

func (r *Recorder) Notify(msg string) error {
	r.Sent = append(r.Sent, strings.ToUpper(msg))
	return nil
}

func alertAll(ns []Notifier, msg string) {
	for _, n := range ns {
		_ = n.Notify(msg)
	}
}

func main() {
	rec := &Recorder{}
	alertAll([]Notifier{Slack{"#ops"}, rec}, "disk full on db01")
	fmt.Println(rec.Sent)
}
```

`Slack` never mentions `Notifier`; it fits because it has the method. Note that **`*Recorder`** implements the interface (pointer receiver), so you must put `&Recorder{}` in the slice, not `Recorder{}`.

## Small interfaces, defined by the consumer

The standard library's most powerful interfaces have one method: `io.Reader`, `io.Writer`, `fmt.Stringer`, `error`, `http.Handler`. Small interfaces are easy to implement and to fake in tests.

> "Accept interfaces, return structs." A function takes the narrowest interface it needs (`io.Reader`, not `*os.File`) and returns concrete types, so callers get the full API.

## Embedding

Interfaces combine by embedding (`io.ReadWriter` = `Reader` + `Writer`). Structs can embed other types and **promote** their methods, Go's alternative to inheritance:

```go
package main

import (
	"fmt"
	"sync"
)

type Stats struct {
	sync.Mutex // embedded: Lock and Unlock are promoted
	hits       int
}

func (s *Stats) Hit() {
	s.Lock()
	defer s.Unlock()
	s.hits++
}

func main() {
	var s Stats
	for range 5 {
		s.Hit()
	}
	fmt.Println(s.hits)
}
```

Embedding is composition, not inheritance: the embedded value doesn't know about the outer struct, and there's no virtual dispatch back into it. (Embedding a mutex exports `Lock`/`Unlock` too; in library code, prefer a named field `mu sync.Mutex`.)

## Type assertions and type switches

```go
package main

import (
	"fmt"
	"io"
	"strings"
)

func describe(v any) string {
	switch x := v.(type) {
	case nil:
		return "nothing"
	case int, int64:
		return fmt.Sprintf("integer %v", x)
	case string:
		return "text of length " + fmt.Sprint(len(x))
	case fmt.Stringer:
		return "stringer " + x.String()
	case error:
		return "error " + x.Error()
	default:
		return fmt.Sprintf("other %T", x)
	}
}

func main() {
	fmt.Println(describe(42), "|", describe("東京"), "|", describe(io.EOF), "|", describe(3.5), "|", describe(nil))
	var r io.Reader = strings.NewReader("x")
	if s, ok := r.(io.Seeker); ok { // optional capability check
		_, _ = s.Seek(0, io.SeekStart)
		fmt.Println("seekable")
	}
}
```

The `v.(io.Seeker)` form checks for an **optional capability**: the standard library does this a lot (e.g. `io.Copy` uses `WriterTo` when available).

## The nil interface gotcha

An interface value is a pair (type, value). It's only `nil` if **both** are nil. A nil `*MyError` stored in an `error` is **not** a nil error:

```go
package main

import "fmt"

type MyError struct{}

func (*MyError) Error() string { return "boom" }

func mayFail(fail bool) error {
	var e *MyError
	if fail {
		e = &MyError{}
	}
	return e // wrong: returns a non-nil interface holding a nil pointer
}

func main() {
	fmt.Println(mayFail(false) == nil) // false!
}
```

Return a literal `nil` for "no error": `if !fail { return nil }`.

## Compile-time checks

`var _ Notifier = (*Recorder)(nil)` makes the compiler verify that a type implements an interface, right next to the type.

@@quiz
How does a type implement an interface in Go?
- With an `implements` clause
* By having all the interface's methods; it's implicit
- By embedding the interface
- By registering it at init
? Implicit satisfaction lets you define interfaces where they're used.

@@quiz
`func f() error { var p *MyErr; return p }`. Is `f() == nil`?
- Yes
* No; the interface holds a type (*MyErr) with a nil value, so it's not nil
- It panics
- It doesn't compile
? Return a literal nil for success.

@@quiz
What does "accept interfaces, return structs" mean?
* Parameters use the narrowest interface needed; results are concrete types
- Never return interfaces
- Only structs can implement interfaces
- Interfaces must be exported
? Callers get flexibility in and full APIs out.

@@flashcards
Implicit interface :: satisfied by having the methods
`any` :: alias for `interface{}`
Type assertion :: `v.(T)` or `v, ok := v.(T)`
Type switch :: `switch x := v.(type) { ... }`
Embedding :: promotes the embedded type's methods
Interface value :: (dynamic type, value) pair
Nil interface gotcha :: typed nil pointer ≠ nil interface
`var _ I = (*T)(nil)` :: compile-time implementation check
`io.Reader` / `io.Writer` :: the core one-method interfaces

@@exercise A pluggable output pipeline
Write writers that wrap any `io.Writer`:

- `type CountingWriter struct { W io.Writer; Bytes int; Lines int }` with a `Write` method that forwards the data and counts bytes and newlines
- `type PrefixWriter struct { W io.Writer; Prefix string; atLineStart bool }` that inserts `Prefix` at the start of every line (even when lines arrive split across several `Write` calls); construct it with `NewPrefixWriter(w, prefix) *PrefixWriter`
- add compile-time checks that both implement `io.Writer`

`Write` must return `len(p)` on success (not the number of bytes after adding prefixes), or `io.Copy` and friends get confused.
@@starter
package main

import (
	"fmt"
	"io"
	"os"
	"strings"
)

type CountingWriter struct {
	W     io.Writer
	Bytes int
	Lines int
}

type PrefixWriter struct {
	W           io.Writer
	Prefix      string
	atLineStart bool
}

func NewPrefixWriter(w io.Writer, prefix string) *PrefixWriter { return &PrefixWriter{W: w, Prefix: prefix} }

func main() {
	counter := &CountingWriter{W: os.Stdout}
	_ = counter
	var out io.Writer = os.Stdout // TODO: NewPrefixWriter(counter, "[db01] ")
	fmt.Fprint(out, "starting\nconnected to ")
	fmt.Fprintf(out, "%s\n", "replica")
	io.Copy(out, strings.NewReader("sync done\n"))
	fmt.Println(counter.Bytes, counter.Lines)
}
@@expected
[db01] starting
[db01] connected to replica
[db01] sync done
61 3
@@requires
var _ io.Writer
func (w *PrefixWriter) Write(p []byte) (int, error)
@@hint
`PrefixWriter.Write`: loop over the bytes; when `atLineStart` is true, write the prefix first; write each byte (or each chunk up to and including a newline) and set `atLineStart = b == '\n'`. The constructor sets `atLineStart: true`. Return `len(p), nil` at the end.
@@solution
package main

import (
	"bytes"
	"fmt"
	"io"
	"os"
	"strings"
)

type CountingWriter struct {
	W     io.Writer
	Bytes int
	Lines int
}

func (c *CountingWriter) Write(p []byte) (int, error) {
	n, err := c.W.Write(p)
	c.Bytes += n
	c.Lines += bytes.Count(p[:n], []byte("\n"))
	return n, err
}

type PrefixWriter struct {
	W           io.Writer
	Prefix      string
	atLineStart bool
}

func NewPrefixWriter(w io.Writer, prefix string) *PrefixWriter {
	return &PrefixWriter{W: w, Prefix: prefix, atLineStart: true}
}

func (w *PrefixWriter) Write(p []byte) (int, error) {
	rest := p
	for len(rest) > 0 {
		if w.atLineStart {
			if _, err := io.WriteString(w.W, w.Prefix); err != nil {
				return 0, err
			}
		}
		chunk := rest
		if i := bytes.IndexByte(rest, '\n'); i >= 0 {
			chunk = rest[:i+1]
		}
		if _, err := w.W.Write(chunk); err != nil {
			return 0, err
		}
		w.atLineStart = chunk[len(chunk)-1] == '\n'
		rest = rest[len(chunk):]
	}
	return len(p), nil
}

var _ io.Writer = (*CountingWriter)(nil)
var _ io.Writer = (*PrefixWriter)(nil)

func main() {
	counter := &CountingWriter{W: os.Stdout}
	var out io.Writer = NewPrefixWriter(counter, "[db01] ")
	fmt.Fprint(out, "starting\nconnected to ")
	fmt.Fprintf(out, "%s\n", "replica")
	io.Copy(out, strings.NewReader("sync done\n"))
	fmt.Println(counter.Bytes, counter.Lines)
}

@@exercise Shapes of alerts with a type switch
Alerts arrive as `any` values from different sources. Write `Normalize(v any) (Alert, error)` returning `Alert{Source, Severity, Text string}`:

- `Alert` → itself
- `*Alert` → the pointed-to value (nil pointer → error `nil alert`)
- `string` → `Alert{"text", "info", s}`, but a `"CRIT:"` prefix gives severity `critical` and strips the prefix (and the following space)
- `error` → `Alert{"error", "warning", err.Error()}`
- `fmt.Stringer` → `Alert{"stringer", "info", v.String()}`
- `map[string]string` → fields from keys `source`, `severity` (default `info`), `text`
- anything else → error `unsupported alert type <%T>`

Watch the order: an `error` type might also be a `Stringer`.
@@starter
package main

import (
	"errors"
	"fmt"
	"time"
)

type Alert struct{ Source, Severity, Text string }

func Normalize(v any) (Alert, error) {
	return Alert{}, nil
}

type diskEvent struct{ Host string }

func (d diskEvent) String() string { return "disk full on " + d.Host }

func main() {
	var nilAlert *Alert
	inputs := []any{
		Alert{"nagios", "critical", "web01 down"},
		&Alert{"zabbix", "warning", "load high"},
		nilAlert,
		"CRIT: db01 unreachable",
		"backup finished",
		errors.New("connection reset"),
		diskEvent{"nas01"},
		map[string]string{"source": "cron", "text": "job ran"},
		42,
		time.Second,
	}
	for _, in := range inputs {
		a, err := Normalize(in)
		if err != nil {
			fmt.Println("error:", err)
			continue
		}
		fmt.Printf("%s/%s: %s\n", a.Source, a.Severity, a.Text)
	}
}
@@expected
nagios/critical: web01 down
zabbix/warning: load high
error: nil alert
text/critical: db01 unreachable
text/info: backup finished
error/warning: connection reset
stringer/info: disk full on nas01
cron/info: job ran
error: unsupported alert type int
stringer/info: 1s
@@requires
switch x := v.(type)
@@hint
Put `case error:` before `case fmt.Stringer:`. For the string case, `if rest, ok := strings.CutPrefix(x, "CRIT:"); ok { ... strings.TrimSpace(rest) }`. Map lookups of missing keys give `""`, so default the severity with an `if`.
@@solution
package main

import (
	"errors"
	"fmt"
	"strings"
	"time"
)

type Alert struct{ Source, Severity, Text string }

func Normalize(v any) (Alert, error) {
	switch x := v.(type) {
	case Alert:
		return x, nil
	case *Alert:
		if x == nil {
			return Alert{}, errors.New("nil alert")
		}
		return *x, nil
	case string:
		if rest, ok := strings.CutPrefix(x, "CRIT:"); ok {
			return Alert{"text", "critical", strings.TrimSpace(rest)}, nil
		}
		return Alert{"text", "info", x}, nil
	case error:
		return Alert{"error", "warning", x.Error()}, nil
	case fmt.Stringer:
		return Alert{"stringer", "info", x.String()}, nil
	case map[string]string:
		sev := x["severity"]
		if sev == "" {
			sev = "info"
		}
		return Alert{x["source"], sev, x["text"]}, nil
	default:
		return Alert{}, fmt.Errorf("unsupported alert type %T", v)
	}
}

type diskEvent struct{ Host string }

func (d diskEvent) String() string { return "disk full on " + d.Host }

func main() {
	var nilAlert *Alert
	inputs := []any{
		Alert{"nagios", "critical", "web01 down"},
		&Alert{"zabbix", "warning", "load high"},
		nilAlert,
		"CRIT: db01 unreachable",
		"backup finished",
		errors.New("connection reset"),
		diskEvent{"nas01"},
		map[string]string{"source": "cron", "text": "job ran"},
		42,
		time.Second,
	}
	for _, in := range inputs {
		a, err := Normalize(in)
		if err != nil {
			fmt.Println("error:", err)
			continue
		}
		fmt.Printf("%s/%s: %s\n", a.Source, a.Severity, a.Text)
	}
}

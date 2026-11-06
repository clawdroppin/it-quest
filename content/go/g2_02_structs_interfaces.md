@@lesson g2-structs-interfaces Structs, Methods and Interfaces
@@concepts objects
@@text
## Structs

Go has no classes. A **struct** groups fields; **methods** are functions with a receiver.

```go
package main

import "fmt"

type Server struct {
	Name  string
	IP    string
	CPU   int
	Tags  []string
}

// A method with a value receiver: works on a copy, can't change the server.
func (s Server) Healthy() bool {
	return s.CPU < 90
}

// A pointer receiver: can change the original.
func (s *Server) AddTag(t string) {
	s.Tags = append(s.Tags, t)
}

func main() {
	web := Server{Name: "web01", IP: "10.0.0.11", CPU: 35}
	web.AddTag("prod") // Go takes &web for you
	fmt.Println(web.Healthy(), web.Tags)
	fmt.Printf("%v\n%+v\n", web, web) // %+v shows the field names

	p := &web
	p.CPU = 95 // fields through a pointer: no -> needed
	fmt.Println(web.Healthy())
}
```

Rules of thumb for receivers: use a **pointer receiver** when the method changes the struct or the struct is big; be consistent within a type.

A "constructor" is just a function, by convention called `NewThing`, that checks its inputs:

```go
package main

import (
	"errors"
	"fmt"
)

type Vlan struct {
	ID   int
	Name string
}

func NewVlan(id int, name string) (*Vlan, error) {
	if id < 1 || id > 4094 {
		return nil, errors.New("vlan id must be 1-4094")
	}
	return &Vlan{ID: id, Name: name}, nil
}

func main() {
	v, err := NewVlan(10, "users")
	fmt.Println(v.Name, err)
	_, err = NewVlan(5000, "oops")
	fmt.Println(err)
}
```

## Interfaces

An interface lists methods. A type satisfies it **automatically** by having those methods: there's no `implements` keyword.

```go
package main

import "fmt"

type Check interface {
	Name() string
	Run() bool
}

type DiskCheck struct{ UsedPercent int }

func (d DiskCheck) Name() string { return "disk" }
func (d DiskCheck) Run() bool    { return d.UsedPercent < 90 }

type PingCheck struct{ Host string }

func (p PingCheck) Name() string { return "ping " + p.Host }
func (p PingCheck) Run() bool    { return true }

func main() {
	checks := []Check{DiskCheck{95}, PingCheck{"10.0.0.1"}}
	for _, c := range checks {
		status := "PASS"
		if !c.Run() {
			status = "FAIL"
		}
		fmt.Printf("%s: %s\n", c.Name(), status)
	}
}
```

Go interfaces are usually **small**, often one method. The standard library is built on them:

- `fmt.Stringer`: `String() string`. Give your type one and `fmt.Println` uses it.
- `error`: `Error() string`. Any type with that method is an error.
- `io.Reader` / `io.Writer`: `Read`/`Write` bytes. Files, network connections, HTTP bodies, buffers and gzip streams all fit, so code that takes an `io.Reader` works with all of them.

```go
package main

import (
	"fmt"
	"strings"
)

type Status int

const (
	Up Status = iota // iota counts 0, 1, 2...: Go's enums
	Degraded
	Down
)

func (s Status) String() string {
	return [...]string{"up", "degraded", "down"}[s]
}

func main() {
	fmt.Println(Degraded, Down)
	var sb strings.Builder // a strings.Builder is an io.Writer
	fmt.Fprintf(&sb, "service is %v", Up)
	fmt.Println(sb.String())
}
```

## Embedding

A struct can **embed** another type, and gets its fields and methods ("composition over inheritance"):

```go
package main

import "fmt"

type Base struct{ ID int }

func (b Base) Describe() string { return fmt.Sprintf("asset #%d", b.ID) }

type Laptop struct {
	Base
	Owner string
}

func main() {
	l := Laptop{Base{42}, "aiko"}
	fmt.Println(l.ID, l.Describe(), l.Owner)
}
```

## Type switches

```go
package main

import "fmt"

func describe(v any) string { // any = interface{}: can hold any value
	switch x := v.(type) {
	case int:
		return fmt.Sprintf("int %d", x)
	case string:
		return fmt.Sprintf("string %q", x)
	case error:
		return "error: " + x.Error()
	default:
		return fmt.Sprintf("other %T", x)
	}
}

func main() {
	fmt.Println(describe(5), describe("db01"), describe(3.5))
}
```

@@quiz
How does a Go type implement an interface?
* By having all its methods: no declaration needed
- With the `implements` keyword
- By embedding the interface
- By registering it
? Interfaces are satisfied implicitly.

@@quiz
When should a method have a pointer receiver `(s *Server)`?
* When it modifies the struct (or the struct is large)
- Always
- Never
- Only for exported methods
? A value receiver works on a copy, so changes are lost.

@@quiz
Your type has a method `String() string`. What happens when you `fmt.Println` it?
* fmt uses your String method
- Nothing special
- A compile error
- It prints the memory address
? That's the `fmt.Stringer` interface.

@@drill
Which interface does a type implement by having `Error() string`?
= error
? Go interfaces are satisfied implicitly.

@@flashcards
`type T struct { ... }` :: a struct type
`func (t T) M()` :: a method with a value receiver
`func (t *T) M()` :: a pointer receiver: can modify t
`NewT(...)` :: the constructor naming convention
`fmt.Stringer` :: `String() string`
`io.Reader` :: anything you can read bytes from
`iota` :: auto-incrementing constant (enums)
`switch x := v.(type)` :: a type switch

@@exercise Bandwidth counter
Write the methods of `Counter`: `Add(bytes int)` (pointer receiver: it changes the counter), `Total() int`, and `Average() float64` (0 if nothing was added). Then give `Counter` a `String()` method returning `total=<t> avg=<a>` with one decimal.
@@starter
package main

import "fmt"

type Counter struct {
	total int
	calls int
}

// TODO: Add, Total, Average and String

func main() {
	var c Counter
	c.total = 0 // remove this line when you write the methods
	fmt.Println(c)
}
@@expected
total=0 avg=0.0
total=3000 avg=1000.0
@@requires
func (c *Counter) Add
String() string
@@hint
`func (c *Counter) Add(bytes int) { c.total += bytes; c.calls++ }`. In `String`, `fmt.Sprintf("total=%d avg=%.1f", c.Total(), c.Average())`. Change `main` to print `c`, add 1000, 500 and 1500, and print `c` again.
@@solution
package main

import "fmt"

type Counter struct {
	total int
	calls int
}

func (c *Counter) Add(bytes int) {
	c.total += bytes
	c.calls++
}

func (c Counter) Total() int { return c.total }

func (c Counter) Average() float64 {
	if c.calls == 0 {
		return 0
	}
	return float64(c.total) / float64(c.calls)
}

func (c Counter) String() string {
	return fmt.Sprintf("total=%d avg=%.1f", c.Total(), c.Average())
}

func main() {
	var c Counter
	fmt.Println(c)
	c.Add(1000)
	c.Add(500)
	c.Add(1500)
	fmt.Println(c)
}

@@exercise Notifier interface
Define a `Notifier` interface with `Send(msg string) string`, and two types: `Email{To string}` returning `email to <To>: <msg>` and `Slack{Channel string}` returning `slack #<Channel>: <msg>`. Make the given loop work.
@@starter
package main

import "fmt"

// TODO: Notifier, Email and Slack

func main() {
	fmt.Println("TODO") // then replace with the loop below
	// notifiers := []Notifier{Email{"oncall@example.jp"}, Slack{"ops"}}
	// for _, n := range notifiers {
	// 	fmt.Println(n.Send("disk full on db01"))
	// }
}
@@expected
email to oncall@example.jp: disk full on db01
slack #ops: disk full on db01
@@requires
type Notifier interface
@@hint
`type Email struct{ To string }` and `func (e Email) Send(msg string) string { return "email to " + e.To + ": " + msg }`.
@@solution
package main

import "fmt"

type Notifier interface {
	Send(msg string) string
}

type Email struct{ To string }

func (e Email) Send(msg string) string { return "email to " + e.To + ": " + msg }

type Slack struct{ Channel string }

func (s Slack) Send(msg string) string { return "slack #" + s.Channel + ": " + msg }

func main() {
	notifiers := []Notifier{Email{"oncall@example.jp"}, Slack{"ops"}}
	for _, n := range notifiers {
		fmt.Println(n.Send("disk full on db01"))
	}
}

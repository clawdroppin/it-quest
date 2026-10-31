@@lesson g0-welcome Welcome to Go
@@concepts hello, variables
@@text
## Why Go?

Go (often "Golang") was created at Google for servers and tools. It's the language of **cloud infrastructure**: Docker, Kubernetes, Terraform, Prometheus, Grafana, etcd, Caddy, Hugo, and most CNCF projects are written in Go. If you work in DevOps, SRE or cloud, you'll read Go code, and writing small Go tools is a superpower:

- A Go program compiles to **one executable file** with no runtime to install. Copy it to a server and run it.
- **Cross-compiling** is one command: build a Linux binary on Windows with `GOOS=linux go build`.
- Concurrency (doing many things at once) is built into the language, which is perfect for network tools.
- The language is small on purpose. You can learn all of it, and everyone's code looks alike.

You need Go installed for this track: get it from https://go.dev/dl, then restart the app.

## Your first program

```go
package main

import "fmt"

func main() {
	fmt.Println("Hello, world!")
	fmt.Println("こんにちは、Go!")
}
```

- Every Go file starts with `package`. An executable program is `package main` with a `func main()`.
- `import "fmt"` brings in the formatting package from the standard library.
- Go uses **tabs** for indentation, and `gofmt` formats every file the same way. Nobody argues about style in Go.

Compile and run from a terminal: `go run main.go`, or build an executable with `go build`.

## Variables

```go
package main

import "fmt"

func main() {
	var host string = "web01"
	var port int = 443
	cpu := 73.5         // := declares and infers the type (float64): the usual way inside functions
	online := true

	fmt.Println(host, port, cpu, online)
	fmt.Printf("%s:%d cpu=%.1f%% online=%t\n", host, port, cpu, online)

	port = 8443         // = assigns to an existing variable
	const maxRetries = 3
	fmt.Println(port, maxRetries)
}
```

- `:=` is short for "declare a new variable with the type of this value". Use it inside functions.
- `Printf` formats with **verbs**: `%s` string, `%d` integer, `%.1f` float with 1 decimal, `%t` bool, `%v` any value, `%q` quoted string, `%%` a literal %.
- `Println` puts spaces between its arguments and a new line at the end.

## Zero values

A variable declared without a value gets its type's **zero value**, never garbage: `0`, `""`, `false`, or `nil`.

```go
package main

import "fmt"

func main() {
	var count int
	var name string
	var ok bool
	fmt.Printf("%d %q %t\n", count, name, ok)   // 0 "" false
}
```

## Go is strict, on purpose

These are **compile errors** in Go:

- an **unused variable** (`x declared and not used`);
- an **unused import** (`"os" imported and not used`);
- mixing types without converting: `int` + `float64` needs `float64(n)`.

It feels fussy at first, but it keeps code clean. Use `_` for a value you want to ignore.

@@quiz
What does `x := 5` do?
* Declares a new variable `x` of type `int` with the value 5
- Assigns 5 to an existing `x`
- Compares x with 5
- Declares a constant
? `:=` declares; `=` assigns to a variable that already exists.

@@quiz
A Go program imports `"os"` but never uses it. What happens?
* It doesn't compile: unused imports are errors
- A warning
- Nothing
- It runs, but slower
? Go refuses unused imports and unused local variables.

@@quiz
What's printed by `var n int; fmt.Println(n)`?
* `0`
- Random memory
- `nil`
- A compile error
? Every type has a zero value. Ints start at 0.

@@flashcards
`package main` + `func main()` :: an executable program
`x := v` :: declare and infer the type
`fmt.Printf("%d", n)` :: formatted printing
`%v` :: print any value in its default format
zero value :: what an unset variable holds: 0, "", false, nil
`go run main.go` :: compile and run
`go build` :: make an executable
`gofmt` :: the standard formatter

@@exercise Server card
Print exactly these lines with `Printf`, using variables `host`, `ip` and `ram`:

```text
Host: app01
IP:   10.0.0.15
RAM:  16 GB
```
@@starter
package main

import "fmt"

func main() {
	host := "app01"
	fmt.Println(host) // replace this with the three Printf lines
}
@@expected
Host: app01
IP:   10.0.0.15
RAM:  16 GB
@@requires
fmt.Printf
@@hint
`fmt.Printf("RAM:  %d GB\n", ram)`: remember the `\n`, Printf doesn't add one.
@@solution
package main

import "fmt"

func main() {
	host := "app01"
	ip := "10.0.0.15"
	ram := 16
	fmt.Printf("Host: %s\n", host)
	fmt.Printf("IP:   %s\n", ip)
	fmt.Printf("RAM:  %d GB\n", ram)
}

@@exercise Fix the build
This doesn't compile (Go found an unused variable), and even then the average would be wrong: `%.1f` needs a float, but `total / count` is an int. Make it print `average: 2.5`.
@@starter
package main

import "fmt"

func main() {
	total := 10
	count := 4
	unused := "debug"
	avg := total / count
	fmt.Printf("average: %.1f\n", avg)
}
@@expected
average: 2.5
@@hint
Delete the unused variable. Convert before dividing: `float64(total) / float64(count)`. Dividing two ints gives an int (2).
@@solution
package main

import "fmt"

func main() {
	total := 10
	count := 4
	avg := float64(total) / float64(count)
	fmt.Printf("average: %.1f\n", avg)
}

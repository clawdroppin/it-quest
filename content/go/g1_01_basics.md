@@lesson g1-basics Types, Strings and Control Flow
@@concepts numbers, strings, conditions, loops
@@text
## Numbers

Go has `int` (64 bits on 64-bit systems), sized types like `int64`, `uint8` (= `byte`), `uint32`, and `float64` for decimals. There's **no automatic conversion**: convert explicitly.

```go
package main

import (
	"fmt"
	"math"
)

func main() {
	fmt.Println(7/2, 7.0/2, 7%2)            // 3 3.5 1
	n := 3
	ratio := float64(n) / 4
	fmt.Println(ratio, math.Round(2.567*10)/10, math.MaxInt32)
	var ttl uint8 = 255
	ttl++
	fmt.Println(ttl)                        // 0: unsigned types wrap around
}
```

## Strings, bytes and runes

A Go string is a read-only sequence of **bytes**, usually UTF-8 text. `len` counts bytes; a **rune** is one Unicode character.

```go
package main

import (
	"fmt"
	"strings"
	"unicode/utf8"
)

func main() {
	fqdn := "  Web01.Contoso.Local "
	clean := strings.ToLower(strings.TrimSpace(fqdn))
	fmt.Println(clean)
	fmt.Println(strings.Split(clean, ".")[0])
	fmt.Println(strings.Contains(clean, "contoso"), strings.HasPrefix(clean, "web"), strings.HasSuffix(clean, ".local"))
	fmt.Println(strings.ReplaceAll(clean, ".local", ".jp"))
	fmt.Println(strings.Join([]string{"dns", "dhcp", "ntp"}, ", "))
	fmt.Println(strings.Fields("  INFO   disk  ok "))   // split on any whitespace: [INFO disk ok]

	jp := "東京"
	fmt.Println(len(jp), utf8.RuneCountInString(jp))     // 6 bytes, 2 characters
	for i, r := range jp {
		fmt.Println(i, string(r))                       // range over a string gives runes
	}
}
```

To build a string in a loop, use `strings.Builder` (repeated `+` copies every time).

## Converting with strconv

```go
package main

import (
	"fmt"
	"strconv"
)

func main() {
	port, err := strconv.Atoi("443")      // string -> int, plus an error
	fmt.Println(port+1, err)
	_, err = strconv.Atoi("44x")
	fmt.Println(err)
	fmt.Println(strconv.Itoa(8080) + "/tcp")
	f, _ := strconv.ParseFloat("2.5", 64)
	fmt.Println(f * 2)
	fmt.Println(strconv.FormatInt(255, 2), strconv.FormatInt(255, 16))   // 11111111 ff
}
```

Functions that can fail return a **second value, the `error`**. You'll see this everywhere in Go (next lesson).

## if, for and switch

```go
package main

import "fmt"

func main() {
	cpu := 91
	if cpu >= 90 {
		fmt.Println("CRITICAL")
	} else if cpu >= 75 {
		fmt.Println("WARNING")
	} else {
		fmt.Println("OK")
	}

	// for is Go's only loop
	for i := 1; i <= 3; i++ {
		fmt.Println("attempt", i)
	}
	n := 0
	for n < 10 { // like while
		n += 3
	}
	fmt.Println(n)

	for _, h := range []string{"web01", "db01"} {
		fmt.Println("checking", h)
	}

	port := 8080
	switch port {
	case 22:
		fmt.Println("SSH")
	case 80, 8080:
		fmt.Println("HTTP")
	case 443:
		fmt.Println("HTTPS")
	default:
		fmt.Println("unknown")
	}

	switch { // a switch with no value: the first true case runs
	case cpu > 90:
		fmt.Println("hot")
	case cpu > 50:
		fmt.Println("busy")
	}
}
```

- No parentheses around conditions; braces are required.
- `switch` cases **don't fall through**, so no `break` needed.
- `if v, err := strconv.Atoi(s); err == nil { ... }`: an `if` can start with a short statement, whose variables exist only inside the `if`.

@@quiz
What's `len("東京")` in Go?
* `6`: len counts bytes, and each of these characters is 3 bytes in UTF-8
- `2`
- `4`
- an error
? Use `utf8.RuneCountInString` to count characters.

@@quiz
Which loop keywords does Go have?
* Only `for`, in several forms
- `for`, `while` and `do`
- `for` and `foreach`
- `loop`
? `for cond { }` is Go's while loop; `for { }` loops forever.

@@quiz
Does a Go `switch` case fall through into the next one?
* No: each case ends on its own (use `fallthrough` if you really want it)
- Yes, unless you write `break`
- Only for strings
- Only in the default case
? The opposite of C, Java and JavaScript.

@@drill
Which `strconv` function converts a string to an int?
= strconv.Atoi
= Atoi
? n, err := strconv.Atoi("42").

@@flashcards
`strings.TrimSpace(s)` :: remove spaces at both ends
`strings.Fields(s)` :: split on any whitespace
`strings.Split(s, ",")` :: split on a separator
`strconv.Atoi(s)` :: text to int, with an error
`strconv.Itoa(n)` :: int to text
rune :: one Unicode character (an int32)
`for i, r := range s` :: loop over runes
`switch { case x > 1: }` :: a switch on conditions

@@exercise Disk usage line
Given `usedGB := 412.6` and `totalGB := 512.0`, print exactly `C: 412.6 / 512 GB (80.6%)` with one `Printf`, calculating the percentage.
@@starter
package main

import "fmt"

func main() {
	usedGB := 412.6
	totalGB := 512.0
	fmt.Println(usedGB, totalGB) // replace with one Printf
}
@@expected
C: 412.6 / 512 GB (80.6%)
@@requires
Printf
fmt.Printf(
@@hint
`%.1f` for one decimal, `%.0f` for none, `%%` for a percent sign: `fmt.Printf("C: %.1f / %.0f GB (%.1f%%)\n", ...)`.
@@solution
package main

import "fmt"

func main() {
	usedGB := 412.6
	totalGB := 512.0
	fmt.Printf("C: %.1f / %.0f GB (%.1f%%)\n", usedGB, totalGB, usedGB/totalGB*100)
}

@@exercise HTTP status classes
Print each code with its class, using a `switch` with no value: 200–299 `success`, 300–399 `redirect`, 400–499 `client error`, 500–599 `server error`, else `invalid`.
@@starter
package main

import "fmt"

func main() {
	for _, code := range []int{200, 301, 404, 503, 42} {
		class := "?"
		// set class with a switch
		fmt.Printf("%d: %s\n", code, class)
	}
}
@@expected
200: success
301: redirect
404: client error
503: server error
42: invalid
@@requires
switch
case code
@@hint
`switch { case code >= 200 && code <= 299: class = "success" ... default: class = "invalid" }`
@@solution
package main

import "fmt"

func main() {
	for _, code := range []int{200, 301, 404, 503, 42} {
		class := "?"
		switch {
		case code >= 200 && code <= 299:
			class = "success"
		case code >= 300 && code <= 399:
			class = "redirect"
		case code >= 400 && code <= 499:
			class = "client error"
		case code >= 500 && code <= 599:
			class = "server error"
		default:
			class = "invalid"
		}
		fmt.Printf("%d: %s\n", code, class)
	}
}

@@exercise Count the levels
Count the log levels (the first word of each line) and print them in the order `ERROR`, `WARN`, `INFO`, like `ERROR=2`. Use `strings.Fields`.
@@starter
package main

import (
	"fmt"
	"strings"
)

func main() {
	logText := "INFO boot ok\nERROR disk full\nWARN temp high\nINFO login aiko\nERROR fan failed\nINFO logout"
	errors, warns, infos := 0, 0, 0
	for _, line := range strings.Split(logText, "\n") {
		_ = line // look at the first field of each line
	}
	fmt.Printf("ERROR=%d WARN=%d INFO=%d\n", errors, warns, infos)
}
@@expected
ERROR=2 WARN=1 INFO=3
@@hint
`fields := strings.Fields(line)`, then `switch fields[0] { case "ERROR": errors++ ... }`. (Check `len(fields) > 0` for empty lines.)
@@solution
package main

import (
	"fmt"
	"strings"
)

func main() {
	logText := "INFO boot ok\nERROR disk full\nWARN temp high\nINFO login aiko\nERROR fan failed\nINFO logout"
	errors, warns, infos := 0, 0, 0
	for _, line := range strings.Split(logText, "\n") {
		fields := strings.Fields(line)
		if len(fields) == 0 {
			continue
		}
		switch fields[0] {
		case "ERROR":
			errors++
		case "WARN":
			warns++
		case "INFO":
			infos++
		}
	}
	fmt.Printf("ERROR=%d WARN=%d INFO=%d\n", errors, warns, infos)
}

@@lesson d6-iterators Iterators: range over Functions, iter.Seq and Lazy Pipelines
@@concepts loops, functions
@@text
Go 1.23 added **range-over-func**: `for x := range f` works when `f` is a function that "pushes" values to a `yield` callback. This gives every collection, file reader or database cursor the same simple `for ... range` syntax, and makes **lazy** pipelines possible.

## The shape of an iterator

```go
package main

import (
	"fmt"
	"iter"
)

// Countdown yields n, n-1, ..., 1.
func Countdown(n int) iter.Seq[int] {
	return func(yield func(int) bool) {
		for i := n; i >= 1; i-- {
			if !yield(i) { // the loop body ran `break` (or returned): stop
				return
			}
		}
	}
}

func main() {
	for i := range Countdown(5) {
		if i == 2 {
			break
		}
		fmt.Print(i, " ")
	}
	fmt.Println()
}
```

- `iter.Seq[V]` is `func(yield func(V) bool)`; `iter.Seq2[K, V]` yields pairs (like `for k, v := range`).
- `yield` returns `false` when the loop stops early. An iterator **must** stop calling `yield` then (the runtime panics otherwise).
- Cleanup (closing a file, a DB cursor) goes in a `defer` inside the iterator function and runs when the loop ends, even on `break`.

## Standard library iterators

```go
package main

import (
	"fmt"
	"maps"
	"slices"
	"strings"
)

func main() {
	ports := map[string]int{"ssh": 22, "https": 443, "dns": 53}
	for name := range maps.Keys(ports) { // unordered
		_ = name
	}
	sorted := slices.Sorted(maps.Keys(ports))   // collect + sort
	fmt.Println(sorted)
	for i, v := range slices.Backward([]string{"a", "b", "c"}) {
		fmt.Print(i, v, " ")
	}
	fmt.Println()
	for line := range strings.Lines("one\ntwo\n") {
		fmt.Printf("%q ", line)
	}
	fmt.Println()
	for part := range strings.SplitSeq("a,b,c", ",") { // no intermediate slice
		fmt.Print(part)
	}
	fmt.Println()
}
```

Also: `slices.All`, `slices.Values`, `slices.Collect`, `slices.Chunk`, `maps.All`, `maps.Collect`, `strings.FieldsSeq`, `bytes.Lines`.

## Lazy pipelines

Iterator adapters wrap one sequence in another. Nothing runs until a `for` loop pulls values, and only as many values as needed are produced:

```go
package main

import (
	"fmt"
	"iter"
)

func Filter[V any](seq iter.Seq[V], keep func(V) bool) iter.Seq[V] {
	return func(yield func(V) bool) {
		for v := range seq {
			if keep(v) && !yield(v) {
				return
			}
		}
	}
}

func Take[V any](seq iter.Seq[V], n int) iter.Seq[V] {
	return func(yield func(V) bool) {
		if n <= 0 {
			return
		}
		i := 0
		for v := range seq {
			if !yield(v) {
				return
			}
			i++
			if i == n {
				return
			}
		}
	}
}

func Naturals() iter.Seq[int] { // infinite!
	return func(yield func(int) bool) {
		for i := 1; ; i++ {
			if !yield(i) {
				return
			}
		}
	}
}

func main() {
	even := Filter(Naturals(), func(n int) bool { return n%2 == 0 })
	for n := range Take(even, 3) {
		fmt.Print(n, " ")
	}
	fmt.Println()
}
```

## Pull iterators

Sometimes you need to step through two sequences at once (merging, zipping). `iter.Pull` converts a push iterator into `next()`/`stop()` functions:

```go,norun
next, stop := iter.Pull(seq)
defer stop()
v, ok := next()
```

## When to use iterators

Return an `iter.Seq` from a type that holds a collection (`func (r *Registry) All() iter.Seq2[string, Host]`) instead of exposing the internal map or slice. Use them to stream large data (log files, query results) without loading it all into memory. For small data, a plain slice is still simplest.

@@quiz
What does `yield` returning false mean inside an iterator?
* The loop body stopped early (break/return), so the iterator must stop
- An error happened
- The value was rejected by a filter
- The sequence is empty
? Keep calling yield after false and the runtime panics.

@@quiz
What's `iter.Seq2[K, V]`?
- A sequence of two values only
* An iterator that yields pairs, used as `for k, v := range seq`
- A slice of pairs
- A map type
? Like ranging over a map or a slice with index.

@@quiz
Why can an iterator over `Naturals()` (infinite) be used safely with `Take`?
* Iterators are lazy: values are only produced while the loop asks for more
- Go limits loops to a million iterations
- Take copies the sequence first
- It can't
? Laziness lets pipelines stop early.

@@drill
Which package defines `Seq` and `Seq2` iterator types?
= iter
? func(yield func(V) bool).

@@flashcards
Range-over-func :: `for v := range f` where f takes a yield callback
`iter.Seq[V]` :: `func(yield func(V) bool)`
`iter.Seq2[K, V]` :: iterator of pairs
yield returns false :: stop iterating
`slices.Collect` :: iterator → slice
`slices.Sorted` :: collect and sort
`maps.Keys` / `maps.Values` :: iterators over a map
`strings.Lines` / `SplitSeq` :: lazy string iteration
`iter.Pull` :: turn push into next/stop

@@exercise Streaming log analysis
Write iterator helpers and use them on a log:

- `Lines(text string) iter.Seq2[int, string]`: yields (1-based line number, line without the trailing newline), skipping empty lines
- `Parse(lines iter.Seq2[int, string]) iter.Seq[Entry]`: each line is `<level> <service> <message>`; lines with fewer than three fields are skipped
- `Where(seq iter.Seq[Entry], keep func(Entry) bool) iter.Seq[Entry]`
- `CountBy(seq iter.Seq[Entry], key func(Entry) string) map[string]int`

In `main`, print each ERROR entry as `line <n>: <service>: <message>`, but **stop after the second one** (break), then print the per-service counts of WARN and ERROR entries sorted by service. Your iterators must respect an early `break`.
@@starter
package main

import (
	"fmt"
	"iter"
	"maps"
	"slices"
	"strings"
)

type Entry struct {
	Line    int
	Level   string
	Service string
	Message string
}

const logText = `INFO api started
WARN db slow query 1200ms
ERROR api upstream timeout

ERROR auth token expired for ana
garbage
WARN api retrying
ERROR db connection reset
INFO auth login ok`

func main() {
	_ = iter.Seq[int](nil)
	_ = maps.Clone(map[string]int{})
	_ = slices.Sorted[string]
	_ = strings.Fields
	fmt.Println(len(logText))
}
@@expected
line 3: api: upstream timeout
line 5: auth: token expired for ana
api 2
auth 1
db 2
@@requires
iter.Seq2[int, string]
break
@@hint
`Lines`: `for i, line := range strings.Split(text, "\n") { if line == "" { continue }; if !yield(i+1, line) { return } }`. `Parse` uses `strings.SplitN(line, " ", 3)`. For the counts: `CountBy(Where(Parse(Lines(logText)), func(e Entry) bool { return e.Level != "INFO" }), func(e Entry) string { return e.Service })`. Remove the placeholder lines in `main`.
@@solution
package main

import (
	"fmt"
	"iter"
	"maps"
	"slices"
	"strings"
)

type Entry struct {
	Line    int
	Level   string
	Service string
	Message string
}

func Lines(text string) iter.Seq2[int, string] {
	return func(yield func(int, string) bool) {
		for i, line := range strings.Split(text, "\n") {
			if line == "" {
				continue
			}
			if !yield(i+1, line) {
				return
			}
		}
	}
}

func Parse(lines iter.Seq2[int, string]) iter.Seq[Entry] {
	return func(yield func(Entry) bool) {
		for n, line := range lines {
			parts := strings.SplitN(line, " ", 3)
			if len(parts) < 3 {
				continue
			}
			if !yield(Entry{n, parts[0], parts[1], parts[2]}) {
				return
			}
		}
	}
}

func Where(seq iter.Seq[Entry], keep func(Entry) bool) iter.Seq[Entry] {
	return func(yield func(Entry) bool) {
		for e := range seq {
			if keep(e) && !yield(e) {
				return
			}
		}
	}
}

func CountBy(seq iter.Seq[Entry], key func(Entry) string) map[string]int {
	counts := map[string]int{}
	for e := range seq {
		counts[key(e)]++
	}
	return counts
}

const logText = `INFO api started
WARN db slow query 1200ms
ERROR api upstream timeout

ERROR auth token expired for ana
garbage
WARN api retrying
ERROR db connection reset
INFO auth login ok`

func main() {
	shown := 0
	for e := range Where(Parse(Lines(logText)), func(e Entry) bool { return e.Level == "ERROR" }) {
		fmt.Printf("line %d: %s: %s\n", e.Line, e.Service, e.Message)
		shown++
		if shown == 2 {
			break
		}
	}
	counts := CountBy(Where(Parse(Lines(logText)), func(e Entry) bool { return e.Level != "INFO" }), func(e Entry) string { return e.Service })
	for _, svc := range slices.Sorted(maps.Keys(counts)) {
		fmt.Println(svc, counts[svc])
	}
}

@@exercise Merge sorted streams
Two monitoring systems each produce events sorted by time. Write:

- `FromSlice[V any](s []V) iter.Seq[V]`
- `Merge[V any](a, b iter.Seq[V], less func(x, y V) bool) iter.Seq[V]`: yields all values in sorted order (ties: `a` first), using `iter.Pull` on both inputs and calling both `stop` functions when done
- `Dedupe[V comparable](seq iter.Seq[V]) iter.Seq[V]`: drops values equal to the previous one

`main` merges the two event streams, removes duplicates and prints them, then prints only the first 3 of another merge to show early stopping works.
@@starter
package main

import (
	"fmt"
	"iter"
)

type Event struct {
	At   int
	Text string
}

func FromSlice[V any](s []V) iter.Seq[V] {
	return func(yield func(V) bool) {}
}

func main() {
	_ = iter.Pull[int]
	fmt.Println("TODO")
}
@@expected
09:00 web01 up
09:02 db01 slow
09:05 web01 down
09:05 pager sent
09:07 web01 up
---
1 2 3
@@requires
iter.Pull
defer stopA()
@@hint
In `Merge`: `nextA, stopA := iter.Pull(a); defer stopA()` (same for b). Read one value from each, then loop: while both are present yield the smaller (`!less(vb, va)` means a goes first), advancing that side; then drain whichever is left. In `main`, events are formatted with `fmt.Printf("%02d:%02d %s\n", e.At/60, e.At%60, e.Text)`; the inputs are `{540, "web01 up"}, {545, "web01 down"}, {547, "web01 up"}` and `{542, "db01 slow"}, {545, "pager sent"}, {547, "web01 up"}`. For the last line merge `FromSlice([]int{1, 3, 5, 7})` and `FromSlice([]int{2, 4, 6})` and break after 3 values.
@@solution
package main

import (
	"fmt"
	"iter"
)

type Event struct {
	At   int
	Text string
}

func FromSlice[V any](s []V) iter.Seq[V] {
	return func(yield func(V) bool) {
		for _, v := range s {
			if !yield(v) {
				return
			}
		}
	}
}

func Merge[V any](a, b iter.Seq[V], less func(x, y V) bool) iter.Seq[V] {
	return func(yield func(V) bool) {
		nextA, stopA := iter.Pull(a)
		defer stopA()
		nextB, stopB := iter.Pull(b)
		defer stopB()
		va, okA := nextA()
		vb, okB := nextB()
		for okA || okB {
			if okA && (!okB || !less(vb, va)) {
				if !yield(va) {
					return
				}
				va, okA = nextA()
			} else {
				if !yield(vb) {
					return
				}
				vb, okB = nextB()
			}
		}
	}
}

func Dedupe[V comparable](seq iter.Seq[V]) iter.Seq[V] {
	return func(yield func(V) bool) {
		var prev V
		first := true
		for v := range seq {
			if !first && v == prev {
				continue
			}
			first, prev = false, v
			if !yield(v) {
				return
			}
		}
	}
}

func main() {
	a := []Event{{540, "web01 up"}, {545, "web01 down"}, {547, "web01 up"}}
	b := []Event{{542, "db01 slow"}, {545, "pager sent"}, {547, "web01 up"}}
	byTime := func(x, y Event) bool { return x.At < y.At }
	for e := range Dedupe(Merge(FromSlice(a), FromSlice(b), byTime)) {
		fmt.Printf("%02d:%02d %s\n", e.At/60, e.At%60, e.Text)
	}
	fmt.Println("---")
	n := 0
	for v := range Merge(FromSlice([]int{1, 3, 5, 7}), FromSlice([]int{2, 4, 6}), func(x, y int) bool { return x < y }) {
		fmt.Print(v)
		n++
		if n == 3 {
			break
		}
		fmt.Print(" ")
	}
	fmt.Println()
}

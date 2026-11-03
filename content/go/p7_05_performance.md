@@lesson p7-performance Performance: Profiling, Allocations and the Garbage Collector
@@concepts testing
@@text
Go is fast by default, but services that handle thousands of requests per second eventually need tuning. The rule: **measure, don't guess**.

## The workflow

1. Write a **benchmark** that reproduces the slow path (`go test -bench=. -benchmem`).
2. **Profile** it to find where the time or memory goes.
3. Change one thing, rerun, and compare with `benchstat`.

## pprof

```text
go test -bench=BenchmarkParse -cpuprofile cpu.out -memprofile mem.out
go tool pprof -http=:8081 cpu.out     # flame graph and source view in the browser
```

In a running service, import `net/http/pprof` on an **internal** port and grab profiles live:

```go,norun
import _ "net/http/pprof"

go http.ListenAndServe("localhost:6060", nil) // never expose publicly
```

```text
go tool pprof http://localhost:6060/debug/pprof/profile?seconds=30   # CPU
go tool pprof http://localhost:6060/debug/pprof/heap                 # memory
curl localhost:6060/debug/pprof/goroutine?debug=1                    # goroutine dump: find leaks
```

`runtime/trace` (`go tool trace`) shows scheduling, GC pauses and goroutine blocking over time.

## Allocations

Every heap allocation costs time now and GC work later. Common wins:

- **Preallocate**: `make([]T, 0, n)` and `make(map[K]V, n)` when you know the size.
- **strings.Builder** (with `Grow`) instead of `s += ...` in loops.
- Avoid converting between `[]byte` and `string` repeatedly.
- Reuse buffers with `sync.Pool` in hot paths (encoders, request buffers).
- Pass small structs by value; pointers often force data onto the heap.

## Escape analysis

The compiler decides whether a value can live on the **stack** (free) or must **escape** to the heap:

```text
go build -gcflags=-m ./...
./main.go:12:9: &Host{...} escapes to heap
./main.go:20:13: buf does not escape
```

Returning a pointer to a local, storing values in interfaces and capturing variables in long-lived closures commonly cause escapes.

## Measuring allocations from code

`testing.AllocsPerRun(runs, f)` returns the average number of allocations per call. It works in normal programs too, which is what the exercise uses.

## The garbage collector

Go's GC is concurrent with short pauses. Two knobs:

- `GOGC` (default 100): collect when the heap grows by that percentage. Higher means less GC CPU and more memory.
- `GOMEMLIMIT` (e.g. `GOMEMLIMIT=900MiB`): a soft memory cap; important in containers so the GC works harder before the OOM killer strikes.

## Profile-guided optimization

Put a CPU profile from production at `default.pgo` in your main package, and `go build` uses it to inline and optimize hot paths: typically 2–14% faster with no code changes.

## Choosing algorithms

Profiling often points at the algorithm, not micro-optimizations: a map instead of a linear search, a heap (`container/heap`) for top-K instead of sorting everything, batching database calls.

@@quiz
What's the first step in optimizing Go code?
- Rewrite in assembly
* Measure with a benchmark and a profile to find the actual hot spot
- Add sync.Pool everywhere
- Disable the GC
? Intuition about bottlenecks is usually wrong.

@@quiz
Why set `GOMEMLIMIT` in a container?
* So the GC collects more aggressively before the container's memory limit is hit
- It limits the number of goroutines
- It's required by Docker
- It disables the GC
? Without it, the heap may grow past the container's limit and get OOM-killed.

@@quiz
What does `go build -gcflags=-m` show?
- Memory usage
* The compiler's inlining and escape analysis decisions
- GC pauses
- CPU profile
? "escapes to heap" lines show allocations.

@@flashcards
`-benchmem` :: allocations per op in benchmarks
benchstat :: statistically compare benchmark runs
`go tool pprof` :: analyse profiles
`net/http/pprof` :: live profiles over HTTP
Escape analysis :: stack vs heap decision
`-gcflags=-m` :: print escape decisions
`sync.Pool` :: reuse temporary objects
`GOGC` :: GC frequency
`GOMEMLIMIT` :: soft memory limit
`default.pgo` :: profile-guided optimization

@@exercise Counting allocations
Write two versions of building a CSV line from fields:

- `JoinNaive(fields []string) string` using `s += ...`
- `JoinFast(fields []string) string` using a `strings.Builder`, sized up front with `Grow`, so it performs **exactly one** allocation

And two versions of collecting squares: `SquaresNaive(n)` appending to a nil slice, `SquaresFast(n)` preallocating with `make([]int, 0, n)`.

Measure with `testing.AllocsPerRun(100, ...)` and print the results (store results in the provided `sink` variables so the compiler can't remove the work).
@@starter
package main

import (
	"fmt"
	"strings"
	"testing"
)

var sinkS string
var sinkI []int

func JoinNaive(fields []string) string {
	s := ""
	for i, f := range fields {
		if i > 0 {
			s += ","
		}
		s += f
	}
	return s
}

func JoinFast(fields []string) string {
	_ = strings.Builder{}
	return JoinNaive(fields)
}

func main() {
	fields := strings.Fields("web01 10.0.0.5 443 prod eu-west ana 2026-11-03 ok")
	_ = testing.AllocsPerRun
	fmt.Println(JoinFast(fields))
}
@@expected
web01,10.0.0.5,443,prod,eu-west,ana,2026-11-03,ok
same output: true
fast join allocs: 1
naive join allocates more: true
fast squares allocs: 1
naive squares allocates more: true
@@requires
Grow(
testing.AllocsPerRun
@@hint
Compute the final length first: the sum of `len(f)` plus `len(fields)-1` commas. Measure like `fast := testing.AllocsPerRun(100, func() { sinkS = JoinFast(fields) })`. Use `n := 1000` for the squares.
@@solution
package main

import (
	"fmt"
	"strings"
	"testing"
)

var sinkS string
var sinkI []int

func JoinNaive(fields []string) string {
	s := ""
	for i, f := range fields {
		if i > 0 {
			s += ","
		}
		s += f
	}
	return s
}

func JoinFast(fields []string) string {
	n := max(len(fields)-1, 0)
	for _, f := range fields {
		n += len(f)
	}
	var b strings.Builder
	b.Grow(n)
	for i, f := range fields {
		if i > 0 {
			b.WriteByte(',')
		}
		b.WriteString(f)
	}
	return b.String()
}

func SquaresNaive(n int) []int {
	var out []int
	for i := range n {
		out = append(out, i*i)
	}
	return out
}

func SquaresFast(n int) []int {
	out := make([]int, 0, n)
	for i := range n {
		out = append(out, i*i)
	}
	return out
}

func main() {
	fields := strings.Fields("web01 10.0.0.5 443 prod eu-west ana 2026-11-03 ok")
	fmt.Println(JoinFast(fields))
	fmt.Println("same output:", JoinFast(fields) == JoinNaive(fields))
	fast := testing.AllocsPerRun(100, func() { sinkS = JoinFast(fields) })
	naive := testing.AllocsPerRun(100, func() { sinkS = JoinNaive(fields) })
	fmt.Println("fast join allocs:", fast)
	fmt.Println("naive join allocates more:", naive > fast)
	n := 1000
	fastSq := testing.AllocsPerRun(100, func() { sinkI = SquaresFast(n) })
	naiveSq := testing.AllocsPerRun(100, func() { sinkI = SquaresNaive(n) })
	fmt.Println("fast squares allocs:", fastSq)
	fmt.Println("naive squares allocates more:", naiveSq > fastSq)
}

@@exercise Top-K with a heap
Counting log words and sorting **all** of them to get the top 5 is O(n log n). A min-heap of size k is O(n log k) and uses less memory.

Write `TopK(words []string, k int) []WordCount` using `container/heap`:

- count words in a map
- keep a min-heap of at most `k` entries whose top is the "worst" kept entry (lowest count; on ties, the alphabetically **later** word)
- return the entries sorted by count descending, then word ascending

Also write `TopKNaive` (sort everything) and check both agree on 100,000 Zipf-distributed words from a seeded generator.
@@starter
package main

import (
	"container/heap"
	"fmt"
	"math/rand/v2"
	"slices"
	"strings"
)

type WordCount struct {
	Word  string
	Count int
}

func TopK(words []string, k int) []WordCount {
	_ = heap.Init
	return nil
}

func main() {
	vocab := strings.Fields("error warn info debug timeout retry ok refused reset denied closed open")
	rng := rand.New(rand.NewPCG(7, 11))
	zipf := rand.NewZipf(rng, 1.3, 1, uint64(len(vocab)-1))
	words := make([]string, 100000)
	for i := range words {
		words[i] = vocab[zipf.Uint64()]
	}
	_ = slices.SortFunc[[]WordCount]
	fmt.Println(len(words))
}
@@expected
top 3: error warn info
matches naive: true
k larger than vocabulary: 12
small example: [{b 3} {a 2} {c 2}]
@@hint
A heap type `type minHeap []WordCount` with `Len`, `Swap`, `Less(i, j)` (count lower, or equal count and word greater), `Push(x any)` and `Pop() any`. For each entry: `heap.Push`, and if `h.Len() > k` then `heap.Pop`. Afterwards sort with `slices.SortFunc` (count desc, then `strings.Compare`). Iterate over the map in any order: the heap makes the result deterministic. The small example is `TopK(strings.Fields("a b c b a b c d"), 3)`.
@@solution
package main

import (
	"cmp"
	"container/heap"
	"fmt"
	"math/rand/v2"
	"slices"
	"strings"
)

type WordCount struct {
	Word  string
	Count int
}

type minHeap []WordCount

func (h minHeap) Len() int { return len(h) }
func (h minHeap) Less(i, j int) bool {
	if h[i].Count != h[j].Count {
		return h[i].Count < h[j].Count
	}
	return h[i].Word > h[j].Word
}
func (h minHeap) Swap(i, j int) { h[i], h[j] = h[j], h[i] }
func (h *minHeap) Push(x any)   { *h = append(*h, x.(WordCount)) }
func (h *minHeap) Pop() any {
	old := *h
	x := old[len(old)-1]
	*h = old[:len(old)-1]
	return x
}

func byRank(a, b WordCount) int {
	if c := cmp.Compare(b.Count, a.Count); c != 0 {
		return c
	}
	return strings.Compare(a.Word, b.Word)
}

func count(words []string) map[string]int {
	m := map[string]int{}
	for _, w := range words {
		m[w]++
	}
	return m
}

func TopK(words []string, k int) []WordCount {
	h := &minHeap{}
	for w, c := range count(words) {
		heap.Push(h, WordCount{w, c})
		if h.Len() > k {
			heap.Pop(h)
		}
	}
	out := []WordCount(*h)
	slices.SortFunc(out, byRank)
	return out
}

func TopKNaive(words []string, k int) []WordCount {
	var all []WordCount
	for w, c := range count(words) {
		all = append(all, WordCount{w, c})
	}
	slices.SortFunc(all, byRank)
	return all[:min(k, len(all))]
}

func main() {
	vocab := strings.Fields("error warn info debug timeout retry ok refused reset denied closed open")
	rng := rand.New(rand.NewPCG(7, 11))
	zipf := rand.NewZipf(rng, 1.3, 1, uint64(len(vocab)-1))
	words := make([]string, 100000)
	for i := range words {
		words[i] = vocab[zipf.Uint64()]
	}
	top := TopK(words, 5)
	fmt.Println("top 3:", top[0].Word, top[1].Word, top[2].Word)
	fmt.Println("matches naive:", slices.Equal(top, TopKNaive(words, 5)))
	fmt.Println("k larger than vocabulary:", len(TopK(words, 50)))
	fmt.Println("small example:", TopK(strings.Fields("a b c b a b c d"), 3))
}

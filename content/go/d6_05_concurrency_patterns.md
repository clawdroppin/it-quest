@@lesson d6-patterns Concurrency Patterns: select, Pipelines, Fan-out/Fan-in and Bounded Work
@@concepts concurrency
@@text
Goroutines and channels are simple building blocks; the skill is combining them into patterns that don't leak, deadlock or overload the systems you're talking to.

## select

`select` waits on several channel operations and runs the first one that's ready (randomly among ties). With `default` it never blocks:

```go
package main

import (
	"fmt"
	"time"
)

func main() {
	results := make(chan string)
	go func() {
		time.Sleep(20 * time.Millisecond)
		results <- "pong"
	}()
	select {
	case r := <-results:
		fmt.Println("got", r)
	case <-time.After(500 * time.Millisecond):
		fmt.Println("timeout")
	}

	queue := make(chan int, 1)
	select {
	case queue <- 1:
		fmt.Println("queued")
	default:
		fmt.Println("full, dropping") // non-blocking send
	}
}
```

## Pipelines

A pipeline is a chain of stages connected by channels. Each stage **owns** its output channel and **closes** it when done, so the next stage's `range` loop ends:

```go
package main

import "fmt"

func gen(nums ...int) <-chan int {
	out := make(chan int)
	go func() {
		defer close(out)
		for _, n := range nums {
			out <- n
		}
	}()
	return out
}

func square(in <-chan int) <-chan int {
	out := make(chan int)
	go func() {
		defer close(out)
		for n := range in {
			out <- n * n
		}
	}()
	return out
}

func main() {
	for v := range square(square(gen(1, 2, 3))) {
		fmt.Print(v, " ")
	}
	fmt.Println()
}
```

Directional channel types (`<-chan int` receive-only, `chan<- int` send-only) let the compiler enforce who sends and who closes. Rule: **only the sender closes**, and only when no more sends can happen.

## Fan-out, fan-in

Run N workers reading from the same input channel (fan-out), and merge their outputs into one channel (fan-in). A `sync.WaitGroup` tells you when all workers are done, so you can close the merged channel:

```go,norun
var wg sync.WaitGroup
out := make(chan Result)
for range workers {
	wg.Go(func() {             // Go 1.25: Add(1) + go + defer Done() in one call
		for job := range jobs {
			out <- process(job)
		}
	})
}
go func() { wg.Wait(); close(out) }()
for r := range out { ... }
```

Results arrive in **completion order**, not input order. If order matters, carry an index and sort, or write into a pre-sized slice at that index.

## Bounding concurrency

Starting one goroutine per item is fine for 100 items and a disaster for 100,000 network calls. Bound the work:

- a fixed **worker pool** (above), or
- a **semaphore**: a buffered channel of size N; acquire with `sem <- struct{}{}`, release with `<-sem`.
- `golang.org/x/sync/errgroup` with `SetLimit(n)`: goroutines + first error + limit, the most common choice in real code.

## Rate limiting

`time.Ticker` paces work (`for range ticker.C`), and `golang.org/x/time/rate` provides token buckets with bursts. Be a good citizen toward APIs and DNS servers.

## Avoiding leaks and deadlocks

- A goroutine blocked forever on a send nobody receives is a **leak**: it never exits. Give every goroutine a way to stop (closed input, context cancellation).
- `fatal error: all goroutines are asleep - deadlock!` means everything is waiting on channels: often a missing `close` or an unbuffered send with no receiver.
- Prefer **unbuffered** channels unless you have a reason (decoupling bursts, semaphores); buffers hide synchronization bugs.

@@quiz
Who should close a channel?
* The sender, when it will send no more values
- The receiver, when it has read enough
- Anyone, at any time
- Nobody; channels close themselves
? Sending on a closed channel panics.

@@quiz
In a fan-out/fan-in setup, in what order do results arrive?
- Input order
* Completion order; sort or index them if order matters
- Reverse order
- Random but repeatable
? Workers finish at different times.

@@quiz
How can a buffered channel act as a semaphore?
* Its capacity is the limit: send to acquire a slot, receive to release it
- It can't
- By closing it
- With select default only
? `sem := make(chan struct{}, 5)`.

@@drill
Which statement waits on several channel operations at once?
= select
? Add a default case to make it non-blocking.

@@flashcards
`select` :: wait on several channel operations
`default` in select :: don't block
`time.After(d)` :: channel that fires once after d
Pipeline stage :: reads one channel, writes another, closes its output
`<-chan T` / `chan<- T` :: receive-only / send-only
Fan-out :: several workers on one input
Fan-in :: merge outputs into one channel
`wg.Go(f)` :: start a goroutine tracked by a WaitGroup (Go 1.25)
Semaphore :: buffered channel bounding concurrency
Goroutine leak :: goroutine blocked forever

@@exercise A concurrent enrichment pipeline
Build a three-stage pipeline:

1. `Source(lines []string) <-chan string`
2. `Parse(in <-chan string) <-chan Record`: lines are `index,host,bytes`; malformed lines are dropped
3. `Enrich(in <-chan Record, workers int, lookup func(string) string) <-chan Record`: `workers` goroutines fill in `Owner` with `lookup(host)`; the output is closed when all workers finish

`main` collects the records, **sorts them by index** and prints them, plus the total bytes per owner (sorted by owner). The lookup is slow (sleeps), so 4 workers must make it run in well under the sequential time; the program prints whether it did.
@@starter
package main

import (
	"fmt"
	"maps"
	"slices"
	"strconv"
	"strings"
	"sync"
	"time"
)

type Record struct {
	Index int
	Host  string
	Bytes int
	Owner string
}

func Source(lines []string) <-chan string {
	out := make(chan string)
	close(out)
	return out
}

func main() {
	_ = maps.Clone(map[string]int{})
	_ = slices.Sorted[string]
	_ = strconv.Atoi
	_ = strings.Split
	_ = sync.WaitGroup{}
	_ = time.Now
	fmt.Println("TODO")
}
@@expected
1 web01 1200 ana
2 db01 800 kenji
3 web02 300 ana
5 mail01 50 unknown
6 db01 1000 kenji
ana 1500
kenji 1800
unknown 50
fast enough: true
@@hint
Each stage: `out := make(chan T); go func() { defer close(out); for x := range in { ... out <- y } }(); return out`. In `Enrich`, start the workers with a `sync.WaitGroup` and close `out` from a separate goroutine after `wg.Wait()`. In `main`, the input is `[]string{"1,web01,1200", "2,db01,800", "3,web02,300", "4,broken", "5,mail01,50", "6,db01,1000"}`; the lookup sleeps 50 ms and maps web01/web02 → ana, db01 → kenji, others → unknown. Five lookups take 250 ms sequentially; check `time.Since(start) < 200*time.Millisecond`.
@@solution
package main

import (
	"fmt"
	"maps"
	"slices"
	"strconv"
	"strings"
	"sync"
	"time"
)

type Record struct {
	Index int
	Host  string
	Bytes int
	Owner string
}

func Source(lines []string) <-chan string {
	out := make(chan string)
	go func() {
		defer close(out)
		for _, l := range lines {
			out <- l
		}
	}()
	return out
}

func Parse(in <-chan string) <-chan Record {
	out := make(chan Record)
	go func() {
		defer close(out)
		for line := range in {
			parts := strings.Split(line, ",")
			if len(parts) != 3 {
				continue
			}
			idx, err1 := strconv.Atoi(parts[0])
			n, err2 := strconv.Atoi(parts[2])
			if err1 != nil || err2 != nil {
				continue
			}
			out <- Record{Index: idx, Host: parts[1], Bytes: n}
		}
	}()
	return out
}

func Enrich(in <-chan Record, workers int, lookup func(string) string) <-chan Record {
	out := make(chan Record)
	var wg sync.WaitGroup
	for range workers {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for r := range in {
				r.Owner = lookup(r.Host)
				out <- r
			}
		}()
	}
	go func() {
		wg.Wait()
		close(out)
	}()
	return out
}

func main() {
	owners := map[string]string{"web01": "ana", "web02": "ana", "db01": "kenji"}
	lookup := func(host string) string {
		time.Sleep(50 * time.Millisecond)
		if o, ok := owners[host]; ok {
			return o
		}
		return "unknown"
	}
	lines := []string{"1,web01,1200", "2,db01,800", "3,web02,300", "4,broken", "5,mail01,50", "6,db01,1000"}
	start := time.Now()
	var records []Record
	for r := range Enrich(Parse(Source(lines)), 4, lookup) {
		records = append(records, r)
	}
	elapsed := time.Since(start)
	slices.SortFunc(records, func(a, b Record) int { return a.Index - b.Index })
	totals := map[string]int{}
	for _, r := range records {
		fmt.Println(r.Index, r.Host, r.Bytes, r.Owner)
		totals[r.Owner] += r.Bytes
	}
	for _, o := range slices.Sorted(maps.Keys(totals)) {
		fmt.Println(o, totals[o])
	}
	fmt.Println("fast enough:", elapsed < 200*time.Millisecond)
}

@@exercise An error group with a limit
Write a minimal version of `errgroup`:

- `type Group struct` with `SetLimit(n int)`, `Go(f func() error)` and `Wait() error`
- `Go` blocks while `n` goroutines are already running (use a buffered channel as a semaphore)
- `Wait` waits for all goroutines and returns the **first** error that occurred (by time), or nil

`main` runs 8 "pings" with a limit of 3, tracking the current and maximum number running with `sync/atomic`, and checks that the maximum never exceeded 3. Pings to hosts containing `"bad"` fail.
@@starter
package main

import (
	"errors"
	"fmt"
	"strings"
	"sync"
	"sync/atomic"
	"time"
)

type Group struct {
	wg sync.WaitGroup
}

func (g *Group) SetLimit(n int) {}

func (g *Group) Go(f func() error) {
	g.wg.Add(1)
	go func() {
		defer g.wg.Done()
		_ = f()
	}()
}

func (g *Group) Wait() error {
	g.wg.Wait()
	return nil
}

func main() {
	_ = errors.New
	_ = strings.Contains
	var running, peak atomic.Int32
	_ = &running
	_ = &peak
	_ = time.Millisecond
	fmt.Println("TODO")
}
@@expected
first error: ping bad01: unreachable
pings done: 8
peak within limit: true
peak above 1: true
@@hint
Fields: `sem chan struct{}`, `once sync.Once`, `err error`. `SetLimit`: `g.sem = make(chan struct{}, n)`. In `Go`: `if g.sem != nil { g.sem <- struct{}{} }` before starting, and `<-g.sem` in the goroutine's defer; on error, `g.once.Do(func() { g.err = err })`. In each ping: `n := running.Add(1)`, then update the peak with a CompareAndSwap loop (`for { p := peak.Load(); if n <= p || peak.CompareAndSwap(p, n) { break } }`), sleep 20 ms (`bad01` fails after 5 ms, `bad02` after 60 ms), then `running.Add(-1)`. Hosts: web01..web04, bad01, db01, bad02, db02; count completed pings with another atomic.
@@solution
package main

import (
	"errors"
	"fmt"
	"strings"
	"sync"
	"sync/atomic"
	"time"
)

type Group struct {
	wg   sync.WaitGroup
	sem  chan struct{}
	once sync.Once
	err  error
}

func (g *Group) SetLimit(n int) { g.sem = make(chan struct{}, n) }

func (g *Group) Go(f func() error) {
	if g.sem != nil {
		g.sem <- struct{}{}
	}
	g.wg.Add(1)
	go func() {
		defer g.wg.Done()
		if g.sem != nil {
			defer func() { <-g.sem }()
		}
		if err := f(); err != nil {
			g.once.Do(func() { g.err = err })
		}
	}()
}

func (g *Group) Wait() error {
	g.wg.Wait()
	return g.err
}

func main() {
	var running, peak, done atomic.Int32
	ping := func(host string) error {
		n := running.Add(1)
		defer running.Add(-1)
		defer done.Add(1)
		for {
			p := peak.Load()
			if n <= p || peak.CompareAndSwap(p, n) {
				break
			}
		}
		switch host {
		case "bad01":
			time.Sleep(5 * time.Millisecond)
		case "bad02":
			time.Sleep(60 * time.Millisecond)
		default:
			time.Sleep(20 * time.Millisecond)
		}
		if strings.Contains(host, "bad") {
			return errors.New("ping " + host + ": unreachable")
		}
		return nil
	}
	var g Group
	g.SetLimit(3)
	for _, h := range []string{"web01", "web02", "web03", "web04", "bad01", "db01", "bad02", "db02"} {
		g.Go(func() error { return ping(h) })
	}
	err := g.Wait()
	fmt.Println("first error:", err)
	fmt.Println("pings done:", done.Load())
	fmt.Println("peak within limit:", peak.Load() <= 3)
	fmt.Println("peak above 1:", peak.Load() > 1)
}

@@lesson g3-concurrency Goroutines and Channels
@@concepts concurrency
@@text
## Goroutines

Put `go` before a function call and it runs **concurrently**, in a goroutine. Goroutines are cheap: a program can run hundreds of thousands of them. This is why Go is so popular for network tools and servers.

```go
package main

import (
	"fmt"
	"sync"
	"time"
)

func check(host string) {
	time.Sleep(100 * time.Millisecond) // stands in for a network call
	fmt.Println("checked", host)
}

func main() {
	start := time.Now()
	var wg sync.WaitGroup
	for _, h := range []string{"web01", "web02", "db01"} {
		wg.Add(1)
		go func() {
			defer wg.Done()
			check(h)
		}()
	}
	wg.Wait() // wait until every goroutine has called Done
	fmt.Println("all done in about", time.Since(start).Round(100*time.Millisecond))
}
```

Three 100 ms checks finish in about 100 ms, not 300. The "checked" lines can appear in any order: that's concurrency.

When `main` returns, the program ends, even if goroutines are still running. That's why we `Wait`.

## Channels

A **channel** passes values between goroutines safely. "Don't communicate by sharing memory; share memory by communicating."

```go
package main

import "fmt"

type Result struct {
	Host string
	Up   bool
}

func main() {
	hosts := []string{"web01", "old-ftp", "db01"}
	results := make(chan Result)

	for _, h := range hosts {
		go func() {
			results <- Result{h, h != "old-ftp"} // send
		}()
	}

	up := 0
	for range hosts {
		r := <-results // receive: waits until a value arrives
		if r.Up {
			up++
		}
	}
	fmt.Printf("%d/%d up\n", up, len(hosts))
}
```

- `ch <- v` sends, `v := <-ch` receives. On an unbuffered channel, both wait for each other.
- `make(chan T, 10)` is **buffered**: sends only wait when 10 values are queued.
- `close(ch)` says "no more values"; `for v := range ch` reads until it's closed.

## A worker pool

Don't start 10,000 connections at once. A fixed number of workers takes jobs from a channel:

```go
package main

import (
	"fmt"
	"sync"
)

func main() {
	jobs := make(chan int)
	results := make(chan string)
	var wg sync.WaitGroup

	for w := 1; w <= 3; w++ { // 3 workers
		wg.Add(1)
		go func() {
			defer wg.Done()
			for port := range jobs {
				results <- fmt.Sprintf("port %d checked", port)
			}
		}()
	}

	go func() { // feed the jobs, then close the channel
		for _, p := range []int{22, 80, 443, 3389, 8080} {
			jobs <- p
		}
		close(jobs)
	}()

	go func() { // close results once every worker has finished
		wg.Wait()
		close(results)
	}()

	count := 0
	for range results {
		count++
	}
	fmt.Println(count, "ports checked by 3 workers")
}
```

## Mutexes

When goroutines must share a variable, protect it with a `sync.Mutex`, or for counters use `sync/atomic`:

```go
package main

import (
	"fmt"
	"sync"
)

func main() {
	var mu sync.Mutex
	var wg sync.WaitGroup
	seen := map[string]int{}

	for i := 0; i < 100; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			mu.Lock()
			seen["requests"]++ // without the lock: a data race, and maps can crash
			mu.Unlock()
		}()
	}
	wg.Wait()
	fmt.Println(seen["requests"])
}
```

`go run -race main.go` and `go test -race` turn on the **race detector**, which reports unprotected sharing. Use it whenever you test concurrent code.

## Timeouts with select and context

`select` waits on several channels at once. With `time.After` it becomes a timeout:

```go
package main

import (
	"context"
	"fmt"
	"time"
)

func slowLookup(ctx context.Context) (string, error) {
	select {
	case <-time.After(500 * time.Millisecond): // the "work"
		return "10.0.0.11", nil
	case <-ctx.Done(): // cancelled or timed out
		return "", ctx.Err()
	}
}

func main() {
	ctx, cancel := context.WithTimeout(context.Background(), 100*time.Millisecond)
	defer cancel()
	_, err := slowLookup(ctx)
	fmt.Println(err) // context deadline exceeded
}
```

A `context.Context` is passed as the **first parameter** to anything that does I/O, so a whole chain of calls can be cancelled together. Every `net/http` request has one.

@@quiz
`main` starts 10 goroutines and returns immediately. What happens to them?
* The program exits and they're stopped, possibly before they've finished
- main waits for them automatically
- They keep running in the background
- A compile error
? Use a `sync.WaitGroup` or channels to wait.

@@quiz
What does `for v := range ch` do?
* Receives values until the channel is closed
- Loops over a copy of the channel
- Sends values into ch
- Loops forever even after close
? The sender calls `close(ch)` when done; the loop then ends.

@@quiz
Several goroutines do `counts[k]++` on the same map with no lock. What's the risk?
* A data race: wrong counts, and Go may crash with "concurrent map writes"
- Nothing, maps are thread-safe
- It's slow but correct
- A compile error
? Protect it with a `sync.Mutex`, or use `sync.Map`. Find races with `-race`.

@@flashcards
`go f()` :: run f in a new goroutine
`sync.WaitGroup` :: wait for a group of goroutines
`make(chan T)` :: an unbuffered channel
`ch <- v` / `<-ch` :: send / receive
`close(ch)` :: no more values will be sent
`select` :: wait on several channel operations
`context.WithTimeout` :: a context that cancels itself after a time
`go test -race` :: run with the race detector

@@exercise Parallel probes
Probe every host concurrently with goroutines and a `sync.WaitGroup`, storing each answer in `results[i]` (each goroutine writes only its own index, so no lock is needed). Then print each host's state in order, and the count of hosts up.
@@starter
package main

import (
	"fmt"
	"strings"
	"sync"
	"time"
)

func probe(host string) bool {
	time.Sleep(50 * time.Millisecond)
	return !strings.HasPrefix(host, "old")
}

func main() {
	hosts := []string{"web01", "old-ftp", "db01", "old-mail"}
	results := make([]bool, len(hosts))
	var wg sync.WaitGroup
	wg.Wait() // TODO: start one goroutine per host before this
	for i, h := range hosts {
		fmt.Println(h, results[i])
	}
}
@@expected
web01: up
old-ftp: down
db01: up
old-mail: down
2/4 up
@@requires
go func
wg.Add
@@hint
In a loop `for i, h := range hosts { wg.Add(1); go func() { defer wg.Done(); results[i] = probe(h) }() }`. After `wg.Wait()`, print `up`/`down` and count.
@@solution
package main

import (
	"fmt"
	"strings"
	"sync"
	"time"
)

func probe(host string) bool {
	time.Sleep(50 * time.Millisecond)
	return !strings.HasPrefix(host, "old")
}

func main() {
	hosts := []string{"web01", "old-ftp", "db01", "old-mail"}
	results := make([]bool, len(hosts))
	var wg sync.WaitGroup
	for i, h := range hosts {
		wg.Add(1)
		go func() {
			defer wg.Done()
			results[i] = probe(h)
		}()
	}
	wg.Wait()
	up := 0
	for i, h := range hosts {
		state := "down"
		if results[i] {
			state = "up"
			up++
		}
		fmt.Printf("%s: %s\n", h, state)
	}
	fmt.Printf("%d/%d up\n", up, len(hosts))
}

@@exercise Timeout with select
`fetch(ms)` returns a channel that gets `"done"` after `ms` milliseconds. Write `withTimeout(ms, limit int) string` that returns `"done"` if the value arrives within `limit` milliseconds, otherwise `"timeout"`, using `select` and `time.After`.
@@starter
package main

import (
	"fmt"
	"time"
)

func fetch(ms int) <-chan string {
	ch := make(chan string, 1)
	go func() {
		time.Sleep(time.Duration(ms) * time.Millisecond)
		ch <- "done"
	}()
	return ch
}

func withTimeout(ms, limit int) string {
	return "TODO: not done yet"
}

func main() {
	fmt.Println(withTimeout(20, 500))
	fmt.Println(withTimeout(800, 100))
}
@@expected
done
timeout
@@requires
select
time.After
@@hint
`select { case v := <-fetch(ms): return v; case <-time.After(time.Duration(limit) * time.Millisecond): return "timeout" }`
@@solution
package main

import (
	"fmt"
	"time"
)

func fetch(ms int) <-chan string {
	ch := make(chan string, 1)
	go func() {
		time.Sleep(time.Duration(ms) * time.Millisecond)
		ch <- "done"
	}()
	return ch
}

func withTimeout(ms, limit int) string {
	select {
	case v := <-fetch(ms):
		return v
	case <-time.After(time.Duration(limit) * time.Millisecond):
		return "timeout"
	}
}

func main() {
	fmt.Println(withTimeout(20, 500))
	fmt.Println(withTimeout(800, 100))
}

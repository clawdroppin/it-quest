@@lesson d6-project Project: A Concurrent Link Checker with Retries
@@concepts concurrency, networking
@@text
This project combines the module: interfaces for testability, bounded concurrency, context timeouts, errors you can classify, and deterministic reporting from concurrent work.

## The design

A link checker crawls a site from a start page, follows internal links up to a maximum depth, and reports broken links. In production the fetcher would wrap `net/http`:

```go,norun
type HTTPFetcher struct{ Client *http.Client }

func (f HTTPFetcher) Fetch(ctx context.Context, url string) (int, []string, error) {
	req, err := http.NewRequestWithContext(ctx, http.MethodGet, url, nil)
	if err != nil {
		return 0, nil, err
	}
	resp, err := f.Client.Do(req)
	if err != nil {
		return 0, nil, err
	}
	defer resp.Body.Close()
	links := extractLinks(resp.Body) // golang.org/x/net/html in real code
	return resp.StatusCode, links, nil
}
```

Because the crawler depends on a small `Fetcher` **interface**, we can test it with a fake, in-memory site that has slow pages and missing pages: no network, perfectly repeatable.

## Crawling level by level

A breadth-first crawl processes one depth at a time:

1. the frontier starts as `[start]` at depth 0
2. fetch every URL in the frontier concurrently (bounded by a semaphore), each with its own timeout
3. collect new, unseen links as the next frontier; stop after `maxDepth`

A mutex protects the `seen` set and the results slice. Sorting the results by URL at the end makes the report deterministic even though fetches complete in any order.

## Retries with backoff

Networks fail transiently. The second exercise adds a retry helper that retries only **temporary** errors, waits with exponential backoff (`base`, `2×base`, `4×base`... capped), and stops immediately when the context is cancelled. The sleep function is injected, so tests run instantly and can record exactly what would have happened.

In production, add **jitter** (a random fraction of the delay) so thousands of clients don't retry in lockstep.

@@quiz
Why does the crawler take a `Fetcher` interface instead of calling `http.Get`?
* So it can be tested with a fast, deterministic fake site
- Interfaces are faster
- http.Get can't be used concurrently
- It's required by context
? Small interfaces at the consumer are idiomatic Go.

@@quiz
Why add jitter to backoff delays?
- To make retries faster
* So many clients that failed together don't all retry at the same instant
- To satisfy the race detector
- It isn't useful
? Synchronized retries cause "thundering herd" load spikes.

@@quiz
How does the link checker avoid checking the same URL twice?
* It keeps a set (map) of URLs already seen
- It sorts the URLs
- It checks each URL only on even levels
- HTTP caches do it automatically
? Normalize URLs before adding them so trivial differences don't defeat the check.

@@drill
What's the name of the retry strategy that doubles the wait after each failure?
= exponential backoff
= backoff
? Add jitter so clients don't retry in sync.

@@flashcards
BFS crawl :: process the frontier one depth at a time
Frontier :: URLs to visit at the current depth
Per-request timeout :: `context.WithTimeout` around each fetch
Exponential backoff :: base, 2×base, 4×base... capped
Jitter :: randomness that spreads out retries
Injected sleep :: makes retry logic testable instantly

@@exercise The crawler
Implement `Crawl(ctx context.Context, f Fetcher, start string, workers, maxDepth int) []Result`:

- breadth-first, one depth at a time, never visiting a URL twice
- at most `workers` fetches at once (semaphore), each with a 50 ms `context.WithTimeout`
- a timed-out fetch records `Err: "timeout"`; a status ≥ 400 counts as broken, as does any error
- links on pages at `maxDepth` are not followed
- return the results sorted by URL

`main` (provided) prints each result as `url depth status-or-error` and the number of broken links.
@@starter
package main

import (
	"context"
	"errors"
	"fmt"
	"slices"
	"strings"
	"sync"
	"time"
)

type Fetcher interface {
	Fetch(ctx context.Context, url string) (status int, links []string, err error)
}

type page struct {
	delay time.Duration
	links []string
}

type Site map[string]page

func (s Site) Fetch(ctx context.Context, url string) (int, []string, error) {
	p, ok := s[url]
	if !ok {
		return 404, nil, nil
	}
	select {
	case <-time.After(p.delay):
		return 200, p.links, nil
	case <-ctx.Done():
		return 0, nil, ctx.Err()
	}
}

type Result struct {
	URL    string
	Depth  int
	Status int
	Err    string
}

func Crawl(ctx context.Context, f Fetcher, start string, workers, maxDepth int) []Result {
	_ = errors.Is
	_ = slices.SortFunc[[]Result]
	_ = strings.Compare
	_ = sync.Mutex{}
	return nil
}

func main() {
	ms := time.Millisecond
	site := Site{
		"/":              {5 * ms, []string{"/docs", "/blog", "/missing"}},
		"/docs":          {5 * ms, []string{"/docs/install", "/", "/slow"}},
		"/blog":          {5 * ms, []string{"/blog/post-1", "/docs"}},
		"/docs/install":  {5 * ms, []string{"/docs/advanced"}},
		"/blog/post-1":   {5 * ms, []string{"/blog"}},
		"/slow":          {300 * ms, nil},
		"/docs/advanced": {5 * ms, nil},
	}
	results := Crawl(context.Background(), site, "/", 3, 2)
	broken := 0
	for _, r := range results {
		state := fmt.Sprint(r.Status)
		if r.Err != "" {
			state = r.Err
		}
		if r.Err != "" || r.Status >= 400 {
			broken++
		}
		fmt.Println(r.URL, r.Depth, state)
	}
	fmt.Println("broken:", broken)
}
@@expected
/ 0 200
/blog 1 200
/blog/post-1 2 200
/docs 1 200
/docs/install 2 200
/missing 1 404
/slow 2 timeout
broken: 2
@@requires
context.WithTimeout
@@hint
`seen := map[string]bool{start: true}`, `frontier := []string{start}`. For `depth := 0; len(frontier) > 0 && depth <= maxDepth; depth++`: create `var next []string`, a `sync.WaitGroup` and `sem := make(chan struct{}, workers)`; for each URL, `sem <- struct{}{}` then start a goroutine that fetches with a timeout context, appends its `Result` under the mutex and, if `depth < maxDepth`, adds unseen links to `next` (also under the mutex). After `wg.Wait()`, `frontier = next`. Sort with `strings.Compare(a.URL, b.URL)`.
@@solution
package main

import (
	"context"
	"errors"
	"fmt"
	"slices"
	"strings"
	"sync"
	"time"
)

type Fetcher interface {
	Fetch(ctx context.Context, url string) (status int, links []string, err error)
}

type page struct {
	delay time.Duration
	links []string
}

type Site map[string]page

func (s Site) Fetch(ctx context.Context, url string) (int, []string, error) {
	p, ok := s[url]
	if !ok {
		return 404, nil, nil
	}
	select {
	case <-time.After(p.delay):
		return 200, p.links, nil
	case <-ctx.Done():
		return 0, nil, ctx.Err()
	}
}

type Result struct {
	URL    string
	Depth  int
	Status int
	Err    string
}

func Crawl(ctx context.Context, f Fetcher, start string, workers, maxDepth int) []Result {
	var mu sync.Mutex
	seen := map[string]bool{start: true}
	var results []Result
	frontier := []string{start}
	sem := make(chan struct{}, workers)
	for depth := 0; len(frontier) > 0 && depth <= maxDepth; depth++ {
		var next []string
		var wg sync.WaitGroup
		for _, url := range frontier {
			sem <- struct{}{}
			wg.Add(1)
			go func() {
				defer wg.Done()
				defer func() { <-sem }()
				rctx, cancel := context.WithTimeout(ctx, 50*time.Millisecond)
				defer cancel()
				status, links, err := f.Fetch(rctx, url)
				r := Result{URL: url, Depth: depth, Status: status}
				if errors.Is(err, context.DeadlineExceeded) {
					r.Err = "timeout"
				} else if err != nil {
					r.Err = err.Error()
				}
				mu.Lock()
				defer mu.Unlock()
				results = append(results, r)
				if depth < maxDepth {
					for _, l := range links {
						if !seen[l] {
							seen[l] = true
							next = append(next, l)
						}
					}
				}
			}()
		}
		wg.Wait()
		frontier = next
	}
	slices.SortFunc(results, func(a, b Result) int { return strings.Compare(a.URL, b.URL) })
	return results
}

func main() {
	ms := time.Millisecond
	site := Site{
		"/":              {5 * ms, []string{"/docs", "/blog", "/missing"}},
		"/docs":          {5 * ms, []string{"/docs/install", "/", "/slow"}},
		"/blog":          {5 * ms, []string{"/blog/post-1", "/docs"}},
		"/docs/install":  {5 * ms, []string{"/docs/advanced"}},
		"/blog/post-1":   {5 * ms, []string{"/blog"}},
		"/slow":          {300 * ms, nil},
		"/docs/advanced": {5 * ms, nil},
	}
	results := Crawl(context.Background(), site, "/", 3, 2)
	broken := 0
	for _, r := range results {
		state := fmt.Sprint(r.Status)
		if r.Err != "" {
			state = r.Err
		}
		if r.Err != "" || r.Status >= 400 {
			broken++
		}
		fmt.Println(r.URL, r.Depth, state)
	}
	fmt.Println("broken:", broken)
}

@@exercise Retries with exponential backoff
Write `Retry(ctx context.Context, attempts int, base time.Duration, sleep func(context.Context, time.Duration) error, op func(attempt int) error) error`:

- call `op` with attempt numbers starting at 1; success returns nil
- retry only errors that have a `Temporary() bool` method returning true (find it with `errors.As`); other errors are returned immediately
- the delay before retry n is `base * 2^(n-1)`, capped at 1 s
- after the last attempt, return `giving up after N attempts: <err>` (wrapped with `%w`)
- if `sleep` returns an error (the context was cancelled), return it

The provided `fakeSleep` doesn't sleep; it prints the delay, which makes the backoff schedule visible.
@@starter
package main

import (
	"context"
	"errors"
	"fmt"
	"time"
)

type tempErr struct{ msg string }

func (e tempErr) Error() string   { return "temporary: " + e.msg }
func (e tempErr) Temporary() bool { return true }

func fakeSleep(ctx context.Context, d time.Duration) error {
	if err := ctx.Err(); err != nil {
		return err
	}
	fmt.Println("sleep", d)
	return nil
}

func Retry(ctx context.Context, attempts int, base time.Duration, sleep func(context.Context, time.Duration) error, op func(attempt int) error) error {
	_ = errors.As
	return op(1)
}

func main() {
	ms := time.Millisecond
	ctx := context.Background()
	n := 0
	err := Retry(ctx, 5, 100*ms, fakeSleep, func(a int) error {
		n = a
		if a < 3 {
			return tempErr{"connection reset"}
		}
		return nil
	})
	fmt.Println("result:", err, "attempts:", n)

	err = Retry(ctx, 5, 100*ms, fakeSleep, func(a int) error {
		n = a
		return errors.New("permanent: bad credentials")
	})
	fmt.Println("result:", err, "attempts:", n)

	err = Retry(ctx, 5, 300*ms, fakeSleep, func(a int) error {
		n = a
		return tempErr{"503"}
	})
	fmt.Println("result:", err, "attempts:", n)
	var te tempErr
	fmt.Println("still a tempErr:", errors.As(err, &te))

	cancelled, cancel := context.WithCancel(ctx)
	cancel()
	err = Retry(cancelled, 5, 100*ms, fakeSleep, func(a int) error {
		n = a
		return tempErr{"timeout"}
	})
	fmt.Println("result:", err, "attempts:", n, "canceled:", errors.Is(err, context.Canceled))
}
@@expected
sleep 100ms
sleep 200ms
result: <nil> attempts: 3
result: permanent: bad credentials attempts: 1
sleep 300ms
sleep 600ms
sleep 1s
sleep 1s
result: giving up after 5 attempts: temporary: 503 attempts: 5
still a tempErr: true
result: context canceled attempts: 1 canceled: true
@@requires
errors.As
.Temporary()
@@hint
`var t interface{ Temporary() bool }` then `errors.As(err, &t) && t.Temporary()`. Delay: `min(base<<(attempt-1), time.Second)`. Loop `for attempt := 1; ; attempt++`, returning from inside.
@@solution
package main

import (
	"context"
	"errors"
	"fmt"
	"time"
)

type tempErr struct{ msg string }

func (e tempErr) Error() string   { return "temporary: " + e.msg }
func (e tempErr) Temporary() bool { return true }

func fakeSleep(ctx context.Context, d time.Duration) error {
	if err := ctx.Err(); err != nil {
		return err
	}
	fmt.Println("sleep", d)
	return nil
}

func Retry(ctx context.Context, attempts int, base time.Duration, sleep func(context.Context, time.Duration) error, op func(attempt int) error) error {
	for attempt := 1; ; attempt++ {
		err := op(attempt)
		if err == nil {
			return nil
		}
		var t interface{ Temporary() bool }
		if !errors.As(err, &t) || !t.Temporary() {
			return err
		}
		if attempt >= attempts {
			return fmt.Errorf("giving up after %d attempts: %w", attempts, err)
		}
		if err := sleep(ctx, min(base<<(attempt-1), time.Second)); err != nil {
			return err
		}
	}
}

func main() {
	ms := time.Millisecond
	ctx := context.Background()
	n := 0
	err := Retry(ctx, 5, 100*ms, fakeSleep, func(a int) error {
		n = a
		if a < 3 {
			return tempErr{"connection reset"}
		}
		return nil
	})
	fmt.Println("result:", err, "attempts:", n)

	err = Retry(ctx, 5, 100*ms, fakeSleep, func(a int) error {
		n = a
		return errors.New("permanent: bad credentials")
	})
	fmt.Println("result:", err, "attempts:", n)

	err = Retry(ctx, 5, 300*ms, fakeSleep, func(a int) error {
		n = a
		return tempErr{"503"}
	})
	fmt.Println("result:", err, "attempts:", n)
	var te tempErr
	fmt.Println("still a tempErr:", errors.As(err, &te))

	cancelled, cancel := context.WithCancel(ctx)
	cancel()
	err = Retry(cancelled, 5, 100*ms, fakeSleep, func(a int) error {
		n = a
		return tempErr{"timeout"}
	})
	fmt.Println("result:", err, "attempts:", n, "canceled:", errors.Is(err, context.Canceled))
}

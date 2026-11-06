@@lesson f8-ui-concurrency Background Work in GUIs: the UI Thread, Progress, Cancellation and Debouncing
@@concepts concurrency
@@text
GUI toolkits have one rule in common: **only the UI thread touches widgets**. Long work (network calls, file scans) must run in goroutines, or the window freezes. The results then have to get back to the UI thread.

## fyne.Do

Since Fyne 2.6, widget updates from other goroutines must be wrapped:

```go,norun
scan := widget.NewButton("Scan network", nil)
progress := widget.NewProgressBar()
scan.OnTapped = func() {
	scan.Disable()
	go func() { // never block the UI thread
		for i, host := range hosts {
			probe(host)
			fyne.Do(func() { progress.SetValue(float64(i+1) / float64(len(hosts))) })
		}
		fyne.Do(func() { scan.Enable() })
	}()
}
```

- `fyne.Do(f)` queues `f` to run on the UI thread and returns immediately.
- `fyne.DoAndWait(f)` waits until it has run.
- Bindings set from goroutines are delivered on the UI thread for you.

Other toolkits do the same: `Dispatcher.Invoke` in WPF, `runOnUiThread` on Android, `DispatchQueue.main` on iOS, `invoke` events in Tauri. The model underneath is always a **queue of functions processed by one goroutine**, which you'll build in the first exercise.

## Cancellation

Every long operation needs a Cancel button. Pass a `context.Context` into the work and call its `cancel` from the button. The worker checks `ctx.Err()` between steps (or passes ctx to network calls) and reports back "cancelled".

## Debouncing input

Searching on every keystroke hammers servers and makes the UI stutter. **Debounce**: wait until the user has stopped typing for e.g. 300 ms:

```go,norun
var timer *time.Timer
search.OnChanged = func(s string) {
	if timer != nil {
		timer.Stop()
	}
	timer = time.AfterFunc(300*time.Millisecond, func() {
		results := find(s)                    // background goroutine
		fyne.Do(func() { show(results) })     // back on the UI thread
	})
}
```

The timing rules are easy to get wrong, so the second exercise implements them as a pure function over timestamped keystrokes, which is testable without real time.

## Other responsiveness tips

- Show progress for anything over ~100 ms, and an indeterminate spinner when you can't measure it.
- Disable buttons while their action runs to prevent double submits.
- Throttle very frequent updates (e.g. log lines) by batching them every 100 ms.

@@quiz
Why must long work run outside the UI thread?
* While the UI thread is busy, the window can't redraw or respond to input
- Goroutines are faster
- Fyne forbids long functions
- To use more memory
? The OS may even mark the window "Not responding".

@@quiz
What does `fyne.Do(f)` do?
- Runs f in a new goroutine
* Queues f to run on the UI thread
- Runs f after a delay
- Cancels f
? Use it for widget updates from goroutines.

@@quiz
With a 300 ms debounce, the user types at 0, 100 and 150 ms and then stops. When does the search run?
- At 300 ms
* At 450 ms
- Three times
- At 0 ms
? 300 ms after the last keystroke.

@@drill
Which Fyne function runs code on the UI thread from a goroutine?
= fyne.Do
= fyne.DoAndWait
? Never touch widgets directly from background goroutines.

@@flashcards
UI thread :: the only goroutine allowed to touch widgets
`fyne.Do` :: run on the UI thread, don't wait
`fyne.DoAndWait` :: run on the UI thread and wait
Progress bar :: `SetValue(0..1)`
Cancel button :: calls a context's cancel
Debounce :: act after input stops for a while
Throttle :: act at most once per interval
`time.AfterFunc` :: run f after a delay; Stop to cancel

@@exercise A UI event loop with cancellable jobs
Simulate a download manager window:

- `UI` has a `queue chan func()`; `Do(f)` enqueues, `DoAndWait(f)` enqueues and waits; `Run()` executes queued functions in order until the queue is closed. The main goroutine plays the UI thread.
- Jobs run in goroutines with a context: each step sleeps 1 ms, then reports `progress = step*100/steps` via `DoAndWait`. Before each step the job checks `ctx.Err()` and, if cancelled, reports `cancelled` and stops; at the end it reports `done`.
- **UI state is only touched inside queued functions.** When `backup.tar` reaches ≥ 50%, the UI cancels it (as if the user clicked Cancel).
- After all jobs finish, close the queue (from a goroutine waiting on a `WaitGroup`), then print each job (sorted by name), a status-bar summary and the number of UI updates.

Jobs: `logs.zip` 4 steps, `backup.tar` 10 steps, `config.json` 2 steps.
@@starter
package main

import (
	"context"
	"fmt"
	"slices"
	"sync"
	"time"
)

type UI struct{ queue chan func() }

type Job struct {
	Name     string
	Steps    int
	Progress int
	Status   string
}

func main() {
	_ = context.Background
	_ = slices.Sorted[string]
	_ = sync.WaitGroup{}
	_ = time.Millisecond
	fmt.Println("TODO")
}
@@expected
backup.tar   cancelled 50%
config.json  done      100%
logs.zip     done      100%
status: 2 done, 1 cancelled
UI updates: 14
@@requires
DoAndWait(
ctx.Err()
@@hint
`DoAndWait`: `done := make(chan struct{}); u.queue <- func() { f(); close(done) }; <-done`. Keep `jobs map[string]*Job` and `cancels map[string]context.CancelFunc`, created before starting goroutines. Count updates in the UI functions (no locks needed: only the UI goroutine touches them). Print with `"%-12s %-9s %d%%\n"`.
@@solution
package main

import (
	"context"
	"fmt"
	"maps"
	"slices"
	"sync"
	"time"
)

type UI struct{ queue chan func() }

func NewUI() *UI { return &UI{queue: make(chan func(), 16)} }

func (u *UI) Do(f func()) { u.queue <- f }

func (u *UI) DoAndWait(f func()) {
	done := make(chan struct{})
	u.queue <- func() {
		f()
		close(done)
	}
	<-done
}

func (u *UI) Run() {
	for f := range u.queue {
		f()
	}
}

type Job struct {
	Name     string
	Steps    int
	Progress int
	Status   string
}

func main() {
	ui := NewUI()
	jobs := map[string]*Job{}
	cancels := map[string]context.CancelFunc{}
	updates := 0
	var wg sync.WaitGroup

	for _, j := range []*Job{{Name: "logs.zip", Steps: 4}, {Name: "backup.tar", Steps: 10}, {Name: "config.json", Steps: 2}} {
		j.Status = "running"
		jobs[j.Name] = j
		ctx, cancel := context.WithCancel(context.Background())
		cancels[j.Name] = cancel
		name, steps := j.Name, j.Steps
		wg.Go(func() {
			defer cancel()
			for step := 1; step <= steps; step++ {
				if ctx.Err() != nil {
					ui.DoAndWait(func() {
						updates++
						jobs[name].Status = "cancelled"
					})
					return
				}
				time.Sleep(time.Millisecond)
				p := step * 100 / steps
				ui.DoAndWait(func() {
					updates++
					jobs[name].Progress = p
					if name == "backup.tar" && p >= 50 {
						cancels[name]()
					}
				})
			}
			ui.DoAndWait(func() {
				updates++
				jobs[name].Status = "done"
			})
		})
	}
	go func() {
		wg.Wait()
		close(ui.queue)
	}()
	ui.Run()

	done, cancelled := 0, 0
	for _, name := range slices.Sorted(maps.Keys(jobs)) {
		j := jobs[name]
		fmt.Printf("%-12s %-9s %d%%\n", j.Name, j.Status, j.Progress)
		switch j.Status {
		case "done":
			done++
		case "cancelled":
			cancelled++
		}
	}
	fmt.Printf("status: %d done, %d cancelled\n", done, cancelled)
	fmt.Println("UI updates:", updates)
}

@@exercise Debouncing a search box
Write `Debounce(keys []Keystroke, wait time.Duration) []Event` that decides what a debounced search box does:

- `Keystroke{At time.Duration; Text string}` is the box's content after each key, in time order
- a keystroke "fires" at `At + wait` unless the next keystroke comes **before** that time
- when it fires, the query is the trimmed text: empty → event `clear`; same as the last search that ran → `skip duplicate`; otherwise `search`
- `Event{At time.Duration; Kind, Query string}`

`main` prints each event and how many searches ran.
@@starter
package main

import (
	"fmt"
	"strings"
	"time"
)

type Keystroke struct {
	At   time.Duration
	Text string
}

type Event struct {
	At          time.Duration
	Kind, Query string
}

func Debounce(keys []Keystroke, wait time.Duration) []Event {
	_ = strings.TrimSpace
	return nil
}

func main() {
	ms := time.Millisecond
	keys := []Keystroke{
		{0, "t"}, {100 * ms, "to"}, {150 * ms, "tok"},
		{600 * ms, "toky"}, {650 * ms, "tokyo"},
		{2000 * ms, "tokyo "}, {2500 * ms, ""},
	}
	searches := 0
	for _, e := range Debounce(keys, 300*ms) {
		fmt.Printf("%v %s %q\n", e.At, e.Kind, e.Query)
		if e.Kind == "search" {
			searches++
		}
	}
	fmt.Println("searches run:", searches)
}
@@expected
450ms search "tok"
950ms search "tokyo"
2.3s skip duplicate "tokyo"
2.8s clear ""
searches run: 2
@@hint
Loop with the index: `fire := k.At + wait`; if `i+1 < len(keys) && keys[i+1].At < fire`, continue. Track `last string` (the last query that actually ran).
@@solution
package main

import (
	"fmt"
	"strings"
	"time"
)

type Keystroke struct {
	At   time.Duration
	Text string
}

type Event struct {
	At          time.Duration
	Kind, Query string
}

func Debounce(keys []Keystroke, wait time.Duration) []Event {
	var out []Event
	last := ""
	for i, k := range keys {
		fire := k.At + wait
		if i+1 < len(keys) && keys[i+1].At < fire {
			continue
		}
		q := strings.TrimSpace(k.Text)
		switch {
		case q == "":
			out = append(out, Event{fire, "clear", ""})
		case q == last:
			out = append(out, Event{fire, "skip duplicate", q})
		default:
			out = append(out, Event{fire, "search", q})
			last = q
		}
	}
	return out
}

func main() {
	ms := time.Millisecond
	keys := []Keystroke{
		{0, "t"}, {100 * ms, "to"}, {150 * ms, "tok"},
		{600 * ms, "toky"}, {650 * ms, "tokyo"},
		{2000 * ms, "tokyo "}, {2500 * ms, ""},
	}
	searches := 0
	for _, e := range Debounce(keys, 300*ms) {
		fmt.Printf("%v %s %q\n", e.At, e.Kind, e.Query)
		if e.Kind == "search" {
			searches++
		}
	}
	fmt.Println("searches run:", searches)
}

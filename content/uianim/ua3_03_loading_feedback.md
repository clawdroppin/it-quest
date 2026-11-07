@@lesson ua3-loading-feedback Loading and Feedback Motion: Spinners, Skeletons, Progress and Optimistic UI
@@lang python
@@text
Some of the most important animations in an app are about **waiting**: spinners, skeleton screens, progress bars and the small confirmations after an action. Done badly, they make the app feel slower (a spinner that flashes for 50 ms) or dishonest (a progress bar stuck at 99%). The rules below come from response-time research and a few practical tricks.

## Response-time thresholds

- **< 100 ms**: feels instant. Show no loading indicator, just the result.
- **100 ms – 1 s**: the user notices but stays in flow. A subtle state change (a pressed button, dimmed content) is enough.
- **1 – 10 s**: show a **spinner or skeleton**, so the app doesn't look frozen.
- **> 10 s**: show **determinate progress** (a percentage or time remaining) and let the user do something else or cancel.

## Avoiding flicker

- **Delay** before showing a spinner (about 300–400 ms): fast responses never show it.
- **Minimum display time** once shown (about 500 ms): a spinner that appears and vanishes 80 ms later looks like a glitch.
- **Skeleton screens** (grey shapes where the content will be) feel faster than spinners for content loads, because the layout is already there and nothing jumps. A gentle **shimmer** (a moving gradient) signals activity; turn it off for reduced motion.

## Honest progress

- Progress bars should move **steadily**. Raw progress often jumps (fast at first, then stalls). Smoothing the rate with an **exponential moving average** gives a stable time estimate.
- Don't let estimates count up. Never sit at 99% for a long time: reserve the last part for work you can measure.
- **Indeterminate** bars (sliding stripes) are for when you really can't measure.

## Optimistic UI

Show the result **before the server confirms** (a "like" turns red immediately), then roll back with a clear message if the request fails. Animate the rollback gently, so the change is noticed.

## Matching feedback to the wait

The right feedback depends on how long the user waits, and you rarely know that in advance. A practical policy:

| Expected wait | Feedback |
|---|---|
| under ~100 ms | nothing; the change feels instant |
| 100 ms to 1 s | a subtle change on the control itself (a pressed state, a small inline spinner) |
| 1 to ~5 s | a skeleton or spinner in the area that will change |
| over ~5 s, or unknown | a determinate progress bar with text, and a way to cancel or keep working |

Two timing tricks make this robust:

- **Delay before showing.** Don't show a spinner until about 300 ms have passed. Fast responses then never flash a spinner at all, which looks calmer than a 50 ms flicker.
- **Minimum display time.** Once a spinner is shown, keep it for at least about 500 ms, so it doesn't appear and vanish in a jarring blink.

## Optimistic UI

For actions that almost always succeed (liking a post, ticking a to-do, renaming a file), you can update the interface **immediately** and send the request in the background. If the request fails, roll the change back and say so clearly ("Couldn't save, try again"). Optimistic updates remove waiting entirely, but only use them where a rollback is cheap and understandable; never for payments or deletes the user can't undo.

## Success and error states

Feedback doesn't end when the wait does:

- **Success** should be quick and quiet: a checkmark that draws itself in 300 ms, a toast that disappears after a few seconds, or simply the new content appearing.
- **Errors** must stay until the user has read them, say what happened in plain words, and offer the next step (retry, edit the input, contact support). Shaking a field is a nice cue, but always pair it with text, and keep the shake small and short so it's not distracting or a problem for motion-sensitive users.

@@quiz
A request usually finishes in 60 ms but sometimes takes 2 s. How do you avoid a flickering spinner?
* Delay showing the spinner by ~300 ms, and keep it visible at least ~500 ms once shown
- Always show it for 2 s
- Never show a spinner
- Show it only after 10 s
? Fast responses then never show it.

@@quiz
What should a 30-second export show?
- Nothing
- A spinner only
* Determinate progress with a time estimate and a cancel option
- A full-screen blocking dialog with no information
? Over 10 seconds, show progress and allow other work.

@@quiz
Why do skeleton screens feel faster than spinners for content?
* The layout appears immediately and nothing jumps when content arrives
- They load data faster
- They use less CPU
- They hide errors
? Perceived performance.

@@quiz
What is optimistic UI?
* Showing the expected result immediately and rolling back if the server fails
- Hiding all errors
- Preloading every page
- Assuming the network is always fast
? Likes, toggles and reorders are good candidates.

@@quiz
Below what response time does an action feel instant?
* About 100 ms
- 1 second
- 3 seconds
- 10 seconds
? No indicator is needed.

@@flashcards
< 100 ms :: feels instant, no indicator
100 ms – 1 s :: subtle state change
1 – 10 s :: spinner or skeleton
> 10 s :: determinate progress, cancel
spinner delay :: about 300–400 ms before showing
minimum display :: about 500 ms once shown
skeleton screen :: placeholder shapes of the layout
shimmer :: moving gradient; off for reduced motion
EMA :: smooths a rate for stable estimates
indeterminate bar :: when you can't measure
optimistic UI :: show first, roll back on failure

@@drill
A spinner is delayed 300 ms and the response arrives at 250 ms. Is the spinner shown? (yes/no)
= no
= No
? It never appears.

@@drill
Above how many seconds should you show determinate progress?
= 10
? Below that a spinner or skeleton is fine.

@@exercise Spinner without flicker
Simulate loading indicators for requests of different durations: the spinner appears only after a delay, stays visible for a minimum time once shown, and the content appears only when both the response has arrived and the minimum time has passed. Report when the spinner shows and hides, and how much extra time the minimum added.
@@concepts conditions, functions, numbers, loops, lists
@@starter
DELAY_MS, MIN_VISIBLE_MS = 300, 500
requests = {"cache hit": 40, "fast API": 280, "slow API": 340, "search": 1200, "report": 4200}

def plan(duration):
    raise NotImplementedError
@@expected
cache hit     40 ms: no spinner, content at 40 ms
fast API     280 ms: no spinner, content at 280 ms
slow API     340 ms: spinner 300-800 ms, content at 800 ms (+460 ms)
search      1200 ms: spinner 300-1200 ms, content at 1200 ms (+0 ms)  (consider a skeleton)
report      4200 ms: spinner 300-4200 ms, content at 4200 ms (+0 ms)  (consider a skeleton)
@@requires
def plan
DELAY_MS
MIN_VISIBLE_MS
max(
@@hint
If `duration < DELAY_MS`, the spinner never shows and content appears at `duration`. Otherwise it shows at DELAY_MS and hides at `max(duration, DELAY_MS + MIN_VISIBLE_MS)`. The extra wait is the hide time minus the duration.
@@solution
DELAY_MS, MIN_VISIBLE_MS = 300, 500
requests = {"cache hit": 40, "fast API": 280, "slow API": 340, "search": 1200, "report": 4200}

def plan(duration):
    if duration < DELAY_MS:
        return None, duration
    hide = max(duration, DELAY_MS + MIN_VISIBLE_MS)
    return (DELAY_MS, hide), hide

for name, duration in requests.items():
    spinner, content_at = plan(duration)
    if spinner is None:
        print(f"{name:<10} {duration:>5} ms: no spinner, content at {content_at} ms")
    else:
        extra = content_at - duration
        hint = "  (consider a skeleton)" if duration > 1000 else ""
        print(f"{name:<10} {duration:>5} ms: spinner {spinner[0]}-{spinner[1]} ms, content at {content_at} ms (+{extra} ms){hint}")

@@exercise Smooth a progress estimate
Raw progress arrives in uneven jumps. Estimate the time remaining with an exponential moving average of the rate (alpha 0.3), never let the displayed estimate go up by more than 10% between updates, and compare it with the naive estimate from the latest jump only.
@@starter
# (seconds elapsed, fraction done)
samples = [(1, 0.20), (2, 0.31), (3, 0.34), (4, 0.36), (5, 0.52), (6, 0.55), (7, 0.70), (8, 0.74), (9, 0.90), (10, 0.93)]
ALPHA = 0.3

def ema(previous, value):
    raise NotImplementedError
@@expected
t= 1s  20%  naive   4.0s  smoothed   4.0s
t= 2s  31%  naive   6.3s  smoothed   4.0s
t= 3s  34%  naive  22.0s  smoothed   4.4s
t= 4s  36%  naive  32.0s  smoothed   4.8s
t= 5s  52%  naive   3.0s  smoothed   4.1s
t= 6s  55%  naive  15.0s  smoothed   4.6s
t= 7s  70%  naive   2.0s  smoothed   2.8s
t= 8s  74%  naive   6.5s  smoothed   3.0s
t= 9s  90%  naive   0.6s  smoothed   0.9s
t=10s  93%  naive   2.3s  smoothed   0.8s
@@requires
def ema
ALPHA
1.1
@@hint
The per-second rate is `done - previous_done`. `ema` returns `ALPHA * value + (1 - ALPHA) * previous` (or the value if previous is None). Remaining = `(1 - done) / rate`. Cap the shown estimate at `last_shown * 1.1`. Naive = `(1 - done) / latest_rate`.
@@solution
# (seconds elapsed, fraction done)
samples = [(1, 0.20), (2, 0.31), (3, 0.34), (4, 0.36), (5, 0.52), (6, 0.55), (7, 0.70), (8, 0.74), (9, 0.90), (10, 0.93)]
ALPHA = 0.3

def ema(previous, value):
    return value if previous is None else ALPHA * value + (1 - ALPHA) * previous

rate, shown, prev_t, prev_done = None, None, 0, 0.0
for t, done in samples:
    latest = (done - prev_done) / (t - prev_t)
    rate = ema(rate, latest)
    estimate = (1 - done) / rate
    if shown is not None:
        estimate = min(estimate, shown * 1.1)
    shown = estimate
    naive = (1 - done) / latest
    print(f"t={t:>2}s {done:4.0%}  naive {naive:5.1f}s  smoothed {shown:5.1f}s")
    prev_t, prev_done = t, done

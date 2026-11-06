@@lesson n7-project Project: A Log Shipping Agent
@@concepts automation, networking
@@text
Log shippers (Fluent Bit, Vector, the Splunk forwarder, the Datadog agent) are some of the most widely deployed C/C++ and Rust programs in IT. They run on every server, so they must be small, fast, and never lose data. This project builds the heart of one.

```text,norun
 tail files ──▶ parse/filter ──▶ batcher ──▶ shipper ──▶ HTTPS endpoint
                                     │            │ retry with backoff
                                     │            ├─ circuit breaker
                                     ▼            ▼
                               flush timer    disk spool (replayed later)
```

## Batching

Sending every line separately wastes requests; waiting too long delays alerts. A batcher flushes on whichever comes first:

- **count**: e.g. 500 events
- **bytes**: stay under the endpoint's request limit (flush *before* adding an event that would overflow)
- **age**: the oldest event has waited e.g. 2 seconds (checked by a timer tick)

## Retrying safely

- **exponential backoff**: wait 100 ms, 200, 400, 800... capped at a maximum, so a struggling endpoint isn't hammered
- **jitter**: randomise each wait slightly so thousands of agents don't retry in lockstep (left out here to keep output deterministic)
- **limited attempts**, then **spool to disk** so nothing is lost
- **circuit breaker**: after several consecutive failed batches, stop trying for a cooldown period ("open"), spool immediately, then let one trial request through ("half-open"); success closes the circuit and the spool is replayed

## Testability

All of this depends on time. Inject a clock (here a struct whose `sleep` just advances a counter) and the sender (`std::function`), and the whole agent becomes deterministic in tests, even a 10-second cooldown takes no real time.

@@quiz
Why flush a batch *before* adding an event that would exceed the byte limit?
* So no batch is ever larger than the endpoint accepts
- To save memory
- Because events can't be split
- It doesn't matter
? The new event starts the next batch.

@@quiz
What does an open circuit breaker do?
- Retries faster
* Stops sending for a cooldown and spools immediately, so a failing endpoint isn't hammered
- Drops all data
- Restarts the agent
? After the cooldown, one trial request decides whether to close it.

@@quiz
Why give each event an ID when shipping logs with retries?
* So the server can drop duplicates when a batch is retried after a timeout
- To sort them alphabetically
- To compress them
- IDs make batches smaller
? Retries plus idempotent IDs give at-least-once delivery without duplicates.

@@drill
What do you call sending several log events in one request instead of one at a time?
= batching
= batch
? Fewer requests, less overhead.

@@flashcards
Batching :: flush on count, bytes or age
Exponential backoff :: double the wait after each failure
Jitter :: randomise waits to avoid thundering herds
Spool :: durable local buffer of unsent data
Circuit breaker :: closed → open → half-open
Injected clock :: deterministic time in tests
At-least-once delivery :: may duplicate, never lose

@@exercise A batcher with count, byte and age limits
Write `Batcher(max_events, max_bytes, max_age, sink)` using `std::chrono::milliseconds` for time:

- `add(event, now)`: if the pending batch is non-empty and adding the event would exceed `max_bytes`, flush first (reason `bytes`); add it (remembering the time of the oldest pending event); if the batch now has `max_events` events, flush (reason `count`)
- `tick(now)`: flush (reason `age`) if the oldest pending event is at least `max_age` old
- the sink receives a `Batch { id, events, bytes, reason, at }` (ids start at 1)

Limits: 3 events, 40 bytes, 500 ms.
@@starter
#include <chrono>
#include <functional>
#include <iostream>
#include <string>
#include <vector>

using std::chrono::milliseconds;

int main() {
    std::cout << "TODO\n";
}
@@expected
t=200ms flush #1: events=3 bytes=30 (count)
t=350ms flush #2: events=1 bytes=25 (bytes)
t=900ms flush #3: events=1 bytes=20 (age)
pending at t=1000ms: 0
@@requires
std::chrono::milliseconds
std::function
@@hint
Events in main: `std::string(10, 'a')` at 0 ms, 10 × `b` at 100, 10 × `c` at 200, 25 × `d` at 300, 20 × `e` at 350, then `tick(900)` and `tick(1000)`. Print `at.count()` for the time. A private `flush(reason, now)` builds the batch, calls the sink and clears the pending state.
@@solution
#include <chrono>
#include <functional>
#include <iostream>
#include <string>
#include <vector>

using std::chrono::milliseconds;

struct Batch {
    int id;
    std::vector<std::string> events;
    size_t bytes;
    std::string reason;
    milliseconds at;
};

class Batcher {
public:
    Batcher(size_t max_events, size_t max_bytes, milliseconds max_age, std::function<void(const Batch&)> sink)
        : max_events_(max_events), max_bytes_(max_bytes), max_age_(max_age), sink_(std::move(sink)) {}

    void add(std::string event, milliseconds now) {
        if (!pending_.empty() && bytes_ + event.size() > max_bytes_) flush("bytes", now);
        if (pending_.empty()) oldest_ = now;
        bytes_ += event.size();
        pending_.push_back(std::move(event));
        if (pending_.size() >= max_events_) flush("count", now);
    }

    void tick(milliseconds now) {
        if (!pending_.empty() && now - oldest_ >= max_age_) flush("age", now);
    }

    size_t pending() const { return pending_.size(); }

private:
    void flush(const std::string& reason, milliseconds now) {
        sink_(Batch{++next_id_, std::move(pending_), bytes_, reason, now});
        pending_.clear();
        bytes_ = 0;
    }

    size_t max_events_, max_bytes_;
    milliseconds max_age_;
    std::function<void(const Batch&)> sink_;
    std::vector<std::string> pending_;
    size_t bytes_ = 0;
    milliseconds oldest_{0};
    int next_id_ = 0;
};

int main() {
    Batcher batcher(3, 40, milliseconds(500), [](const Batch& b) {
        std::cout << "t=" << b.at.count() << "ms flush #" << b.id << ": events=" << b.events.size()
                  << " bytes=" << b.bytes << " (" << b.reason << ")\n";
    });
    batcher.add(std::string(10, 'a'), milliseconds(0));
    batcher.add(std::string(10, 'b'), milliseconds(100));
    batcher.add(std::string(10, 'c'), milliseconds(200));
    batcher.add(std::string(25, 'd'), milliseconds(300));
    batcher.add(std::string(20, 'e'), milliseconds(350));
    batcher.tick(milliseconds(900));
    batcher.tick(milliseconds(1000));
    std::cout << "pending at t=1000ms: " << batcher.pending() << '\n';
}

@@exercise Retries, backoff and a circuit breaker
Write `Shipper(Clock&, std::function<bool(int)> send)` whose `ship(batch)` returns a description:

- try up to 5 attempts; between attempts `clock.sleep(backoff)` with backoff 100 ms doubling each time (capped at 2 s)
- success: `sent (attempts N)`; if the circuit was half-open, append `; circuit closed`
- failure: spool the batch and return `failed after N attempts, waited Xms, spooled`; after 3 consecutive failed batches, open the circuit for 10 s and append `; circuit open`
- while open (now < open_until): spool immediately, `circuit open, spooled`
- after the cooldown the next batch is a single trial (half-open): one attempt; failure re-opens the circuit
- `replay()` sends the spooled batches in order and returns the ones that got through

Simulate an outage: batch 1 succeeds, batches 2–5 hit the outage, then 10 s pass, the endpoint recovers, batch 6 succeeds and the spool is replayed.
@@starter
#include <algorithm>
#include <chrono>
#include <functional>
#include <iostream>
#include <string>
#include <vector>

using namespace std::chrono_literals;
using std::chrono::milliseconds;

struct Clock {
    milliseconds now{0};
    void sleep(milliseconds d) { now += d; }
};

int main() {
    std::cout << "TODO\n";
}
@@expected
batch 1: sent (attempts 1)
batch 2: failed after 5 attempts, waited 1500ms, spooled
batch 3: failed after 5 attempts, waited 1500ms, spooled
batch 4: failed after 5 attempts, waited 1500ms, spooled; circuit open
batch 5: circuit open, spooled
batch 6: sent (attempts 1); circuit closed
replayed spool: 2 3 4 5
@@requires
std::min
open_until
@@hint
State: `bool open_`, `milliseconds open_until_`, `int consecutive_failures_`, `std::vector<int> spool_`. At the start of `ship`: `if (open_ && clock_.now < open_until_) { spool; return ...; } bool half_open = open_;`. In the loop, break after a failure when `attempts == 5 || half_open`. Main: `bool outage = false; Shipper s(clock, [&](int) { return !outage; });`.
@@solution
#include <algorithm>
#include <chrono>
#include <functional>
#include <iostream>
#include <string>
#include <vector>

using namespace std::chrono_literals;
using std::chrono::milliseconds;

struct Clock {
    milliseconds now{0};
    void sleep(milliseconds d) { now += d; }
};

class Shipper {
public:
    Shipper(Clock& clock, std::function<bool(int)> send) : clock_(clock), send_(std::move(send)) {}

    std::string ship(int batch) {
        if (open_ && clock_.now < open_until_) {
            spool_.push_back(batch);
            return "circuit open, spooled";
        }
        bool half_open = open_;
        int attempts = 0;
        milliseconds waited{0}, backoff{100};
        while (true) {
            ++attempts;
            if (send_(batch)) {
                consecutive_failures_ = 0;
                std::string result = "sent (attempts " + std::to_string(attempts) + ")";
                if (half_open) {
                    open_ = false;
                    result += "; circuit closed";
                }
                return result;
            }
            if (attempts == max_attempts_ || half_open) break;
            clock_.sleep(backoff);
            waited += backoff;
            backoff = std::min(backoff * 2, milliseconds(2000));
        }
        spool_.push_back(batch);
        ++consecutive_failures_;
        std::string result = "failed after " + std::to_string(attempts) + " attempts, waited " +
                             std::to_string(waited.count()) + "ms, spooled";
        if (half_open || consecutive_failures_ >= 3) {
            open_ = true;
            open_until_ = clock_.now + 10s;
            result += "; circuit open";
        }
        return result;
    }

    std::vector<int> replay() {
        std::vector<int> sent, kept;
        for (int b : spool_) (send_(b) ? sent : kept).push_back(b);
        spool_ = kept;
        return sent;
    }

private:
    Clock& clock_;
    std::function<bool(int)> send_;
    const int max_attempts_ = 5;
    bool open_ = false;
    milliseconds open_until_{0};
    int consecutive_failures_ = 0;
    std::vector<int> spool_;
};

int main() {
    Clock clock;
    bool outage = false;
    Shipper shipper(clock, [&](int) { return !outage; });

    std::cout << "batch 1: " << shipper.ship(1) << '\n';
    outage = true;
    for (int b = 2; b <= 5; ++b) std::cout << "batch " << b << ": " << shipper.ship(b) << '\n';
    clock.sleep(10s);
    outage = false;
    std::cout << "batch 6: " << shipper.ship(6) << '\n';
    std::cout << "replayed spool:";
    for (int b : shipper.replay()) std::cout << ' ' << b;
    std::cout << '\n';
}

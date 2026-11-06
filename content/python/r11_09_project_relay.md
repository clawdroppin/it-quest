@@lesson r11-relay 🏆 Module Project: An Alert Relay Service
@@concepts networking, automation, errors
@@text
Monitoring tools fire alerts as webhooks; somebody has to receive them, check they're genuine, filter the noise, and forward the important ones to chat or a pager without flooding anyone. This project builds the core of such an **alert relay**, using most of this module: validation, security, configuration, logging, dates and tests.

## Architecture

```text
monitoring ──POST /alerts──▶  verify HMAC ─▶ validate JSON ─▶ dedupe ─▶ rate limit ─▶ outbox ─▶ Slack / Teams / pager
                                (lesson 7)      (lesson 1)      (window)   (token bucket)   (retries)
```

1. **Verify** the signature and timestamp (the webhook exercise from the security lesson).
2. **Validate** the payload (`host`, `severity`, `message`) and reject bad requests with 400/422.
3. **Deduplicate**: the same alert (host + message) within 10 minutes is counted, not re-sent. Alert storms often repeat the same line hundreds of times.
4. **Rate limit** per destination with a **token bucket**: at most N messages per minute, with short bursts allowed.
5. Put accepted alerts in an **outbox** and deliver them with retries and backoff (the job-runner project), so a chat outage doesn't lose alerts.

In production the HTTP layer would be FastAPI, the outbox a database table or a queue (Redis, SQS, Azure Service Bus), and the whole thing a container with structured logs and a `/healthz` endpoint.

## The token bucket

A bucket holds up to `capacity` tokens and refills at `rate` tokens per second. Each message takes one token; if the bucket is empty, the message waits (or is dropped/summarised). This allows short bursts (up to `capacity`) while limiting the average rate. Cloud APIs, NGINX `limit_req` and API gateways all use this algorithm, and it's what the 429 status code usually means.

The refill is computed lazily from the elapsed time whenever someone asks, so no timer thread is needed:

```text
tokens = min(capacity, tokens + (now - last) * rate)
```

## Time as a parameter

Both the deduplication window and the token bucket depend on time. Passing `now` in, instead of calling `time.monotonic()` inside, makes them deterministic and testable (the same trick as the Rust key-value store and the SLA exercise).

## Extensions

- Group repeats into a summary ("disk full on web01 ×37 in the last 10 min").
- Route by severity and team (`severity >= 4` pages on-call; others go to chat).
- Quiet hours with time zones from the dates lesson.
- Metrics: alerts received, deduplicated, rate-limited, delivered, failed.

@@quiz
What does a token bucket allow that a fixed "one per N seconds" rule doesn't?
* Short bursts up to the bucket's capacity while keeping the average rate limited
- Unlimited messages
- Exactly one message ever
- Encryption
? Capacity sets the burst size; rate sets the average.

@@quiz
Why put accepted alerts in an outbox before delivering them?
- To delay all alerts
* So delivery can be retried and nothing is lost if the destination is down
- To encrypt them
- Because HTTP requires it
? The outbox decouples receiving from delivering.

@@quiz
Why deduplicate alerts within a time window?
- To save disk space
* Alert storms repeat the same alert many times; sending each would flood people and hide other alerts
- It's required by Slack
- To make alerts faster
? Count repeats instead of re-sending.

@@drill
What's the name of the rate-limiting algorithm that refills tokens at a steady rate?
= token bucket
= token bucket algorithm
? Each request spends a token; bursts use the saved ones.

@@flashcards
Webhook :: an HTTP request one system sends another when something happens
Token bucket :: rate limiter with burst capacity and a refill rate
429 Too Many Requests :: status code for rate limiting
Deduplication window :: period during which identical events are merged
Outbox :: stored queue of messages awaiting delivery
Alert storm :: a flood of related alerts from one incident

@@exercise A token bucket rate limiter
Write `TokenBucket(capacity, rate)` that starts full:

- `allow(now) -> bool`: refill based on the time since the last call (`tokens = min(capacity, tokens + elapsed * rate)`), then if at least 1 token is available take it and return True, else False
- `wait_time(now) -> float`: seconds until one token will be available (0 if available now), rounded to 3 decimals; it must not consume anything

`now` is a float number of seconds (from a monotonic clock in real use).
@@starter
class TokenBucket:
    pass


bucket = TokenBucket(capacity=3, rate=0.5)
print([bucket.allow(0.0) for _ in range(4)], bucket.wait_time(0.0))
print(bucket.allow(1.0), bucket.allow(2.0), bucket.allow(2.1))
@@expected
[True, True, True, False] 2.0
False True False
@@tests
b = TokenBucket(capacity=2, rate=1)
check(b.allow(0), True, "first")
check(b.allow(0), True, "burst")
check(b.allow(0), False, "empty")
check(b.wait_time(0.25), 0.75, "wait time")
check(b.allow(1.0), True, "refilled one")
check(b.allow(100), True, "long idle")
check(b.allow(100), True, "capped at capacity")
check(b.allow(100), False, "capacity is the max burst")
@@hint
Store `self.tokens = capacity` and `self.last = None`. A `_refill(now)` helper: if `self.last` is not None, add `(now - self.last) * self.rate`, cap at capacity; set `self.last = now`. `wait_time` must refill-compute without saving: calculate the would-be tokens locally, then `max(0, (1 - tokens) / rate)`.
@@solution
class TokenBucket:
    def __init__(self, capacity, rate):
        self.capacity = capacity
        self.rate = rate
        self.tokens = float(capacity)
        self.last = None

    def _tokens_at(self, now):
        if self.last is None:
            return self.tokens
        return min(self.capacity, self.tokens + (now - self.last) * self.rate)

    def allow(self, now):
        self.tokens = self._tokens_at(now)
        self.last = now
        if self.tokens >= 1:
            self.tokens -= 1
            return True
        return False

    def wait_time(self, now):
        tokens = self._tokens_at(now)
        return round(max(0.0, (1 - tokens) / self.rate), 3)


bucket = TokenBucket(capacity=3, rate=0.5)
print([bucket.allow(0.0) for _ in range(4)], bucket.wait_time(0.0))
print(bucket.allow(1.0), bucket.allow(2.0), bucket.allow(2.1))

@@exercise The relay pipeline
Write `Relay(window, bucket)` with `receive(alert, now) -> str` for already-verified alert dicts:

1. Validate: `host` (non-empty str), `severity` (int 1–5), `message` (non-empty str). Invalid → `"rejected: <first problem>"` where the problem is `"host"`, `"severity"` or `"message"` (checked in that order).
2. Deduplicate: if the same `(host, message)` was **sent** less than `window` seconds ago, count it and return `"duplicate (<n>)"` where n is how many duplicates of it have arrived since it was sent.
3. Rate limit: alerts with severity 5 always go through; others need `bucket.allow(now)`, else `"rate limited"`.
4. Otherwise append `f"[sev{severity}] {host}: {message}"` to `self.outbox`, remember when it was sent (resetting its duplicate count) and return `"sent"`.

A `TokenBucket` class is provided.
@@starter
import json


class TokenBucket:
    def __init__(self, capacity, rate):
        self.capacity, self.rate, self.tokens, self.last = capacity, rate, float(capacity), None

    def allow(self, now):
        if self.last is not None:
            self.tokens = min(self.capacity, self.tokens + (now - self.last) * self.rate)
        self.last = now
        if self.tokens >= 1:
            self.tokens -= 1
            return True
        return False


class Relay:
    pass


r = Relay(window=600, bucket=TokenBucket(capacity=1, rate=1 / 60))
events = [
    ({"host": "web01", "severity": 3, "message": "disk 91%"}, 0),
    ({"host": "web01", "severity": 3, "message": "disk 91%"}, 30),
    ({"host": "web01", "severity": 3, "message": "disk 91%"}, 60),
    ({"host": "db01", "severity": 2, "message": "slow queries"}, 61),
    ({"host": "app01", "severity": 2, "message": "high memory"}, 62),
    ({"host": "db01", "severity": 5, "message": "replication broken"}, 63),
    ({"host": "", "severity": 3, "message": "x"}, 64),
    ({"host": "web01", "severity": 3, "message": "disk 91%"}, 700),
]
for alert, t in events:
    print(t, r.receive(alert, t))
print(json.dumps(r.outbox))
@@expected
0 sent
30 duplicate (1)
60 duplicate (2)
61 sent
62 rate limited
63 sent
64 rejected: host
700 sent
["[sev3] web01: disk 91%", "[sev2] db01: slow queries", "[sev5] db01: replication broken", "[sev3] web01: disk 91%"]
@@tests
r = Relay(window=10, bucket=TokenBucket(capacity=5, rate=1))
check(r.receive({"host": "a", "severity": 9, "message": "m"}, 0), "rejected: severity", "bad severity")
check(r.receive({"host": "a", "severity": True, "message": "m"}, 0), "rejected: severity", "bool is not a severity")
check(r.receive({"host": "a", "severity": 2}, 0), "rejected: message", "missing message")
check(r.receive({"host": "a", "severity": 2, "message": "m"}, 0), "sent", "sent")
check(r.receive({"host": "a", "severity": 2, "message": "m"}, 9.9), "duplicate (1)", "inside window")
check(r.receive({"host": "a", "severity": 2, "message": "m"}, 10), "sent", "window over")
check(r.receive({"host": "a", "severity": 2, "message": "m"}, 11), "duplicate (1)", "count reset after send")
@@hint
Keep `self.last_sent = {}` mapping `(host, message)` to `[time_sent, duplicates]`. Validate with small checks: `isinstance(a.get("host"), str) and a["host"]`; `type(a.get("severity")) is int and 1 <= sev <= 5` (so `True` is rejected). A duplicate is `now - time_sent < window`. Severity 5 skips `bucket.allow` entirely.
@@solution
import json


class TokenBucket:
    def __init__(self, capacity, rate):
        self.capacity, self.rate, self.tokens, self.last = capacity, rate, float(capacity), None

    def allow(self, now):
        if self.last is not None:
            self.tokens = min(self.capacity, self.tokens + (now - self.last) * self.rate)
        self.last = now
        if self.tokens >= 1:
            self.tokens -= 1
            return True
        return False


class Relay:
    def __init__(self, window, bucket):
        self.window = window
        self.bucket = bucket
        self.outbox = []
        self.last_sent = {}

    def _problem(self, a):
        if not (isinstance(a.get("host"), str) and a["host"]):
            return "host"
        sev = a.get("severity")
        if type(sev) is not int or not 1 <= sev <= 5:
            return "severity"
        if not (isinstance(a.get("message"), str) and a["message"]):
            return "message"
        return None

    def receive(self, alert, now):
        problem = self._problem(alert)
        if problem:
            return f"rejected: {problem}"
        key = (alert["host"], alert["message"])
        seen = self.last_sent.get(key)
        if seen is not None and now - seen[0] < self.window:
            seen[1] += 1
            return f"duplicate ({seen[1]})"
        if alert["severity"] < 5 and not self.bucket.allow(now):
            return "rate limited"
        self.outbox.append(f"[sev{alert['severity']}] {alert['host']}: {alert['message']}")
        self.last_sent[key] = [now, 0]
        return "sent"


r = Relay(window=600, bucket=TokenBucket(capacity=1, rate=1 / 60))
events = [
    ({"host": "web01", "severity": 3, "message": "disk 91%"}, 0),
    ({"host": "web01", "severity": 3, "message": "disk 91%"}, 30),
    ({"host": "web01", "severity": 3, "message": "disk 91%"}, 60),
    ({"host": "db01", "severity": 2, "message": "slow queries"}, 61),
    ({"host": "app01", "severity": 2, "message": "high memory"}, 62),
    ({"host": "db01", "severity": 5, "message": "replication broken"}, 63),
    ({"host": "", "severity": 3, "message": "x"}, 64),
    ({"host": "web01", "severity": 3, "message": "disk 91%"}, 700),
]
for alert, t in events:
    print(t, r.receive(alert, t))
print(json.dumps(r.outbox))

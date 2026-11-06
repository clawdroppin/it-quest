@@lesson cl2-cloud-native Cloud-Native Design and Serverless
@@lang python
@@text
Cloud-native means building for the cloud's realities: machines are **cattle, not pets** (replaced, not repaired), anything can fail, and scale goes up *and* down.

## The twelve-factor app (the important ones)

| Factor | In practice |
|---|---|
| **Config in the environment** | the same image everywhere; settings from env vars or a config service, never hard-coded |
| **Backing services as attached resources** | the database is a URL; swap it without code changes |
| **Stateless processes** | no session data or uploads on local disk; keep state in a database, cache or object storage |
| **Disposability** | start fast, shut down gracefully on SIGTERM |
| **Logs as event streams** | write to stdout; the platform collects them |
| **Dev/prod parity** | same images, same backing service types |
| **Build, release, run** | build an immutable artifact once, combine with config, run |

## Resilience patterns

- **Health checks** and automatic replacement (load balancers, Kubernetes probes, Auto Scaling).
- **Retries with exponential backoff and jitter** (wait 1 s, 2 s, 4 s... plus randomness, so clients don't retry in lockstep). Only retry **idempotent** operations safely.
- **Timeouts** on every network call; a **circuit breaker** stops calling a failing dependency for a while.
- **Queues** (SQS, Service Bus, RabbitMQ, Kafka) decouple producers and consumers and absorb bursts.
- **Caching** (Redis, CDN), **read replicas**, **multi-AZ**.
- **Immutable infrastructure**: never patch a running server by hand; build a new image and replace.

## Serverless

**Functions as a Service** (AWS Lambda, Azure Functions, Google Cloud Functions) run code per **event** (an HTTP request, a file upload, a queue message, a timer), scale to zero, and bill per request and millisecond. Great for glue code, APIs with spiky traffic and automation. Watch out for **cold starts**, execution time limits (Lambda: 15 minutes), and vendor lock-in. **Serverless containers** (Cloud Run, Azure Container Apps, Fargate) give the same "no servers to manage" with any container.

## Architecture styles

**Monolith** (one deployable app: simple, often the right start) vs **microservices** (many small services owned by small teams: independent deploys and scaling, but networking, observability and data consistency get hard). **Event-driven** architecture connects services through events instead of direct calls.

@@exercise Retry with backoff
Write `backoff_delays(retries, base, cap)` that returns the list of wait times for exponential backoff: `base * 2**attempt` for attempt 0, 1, 2..., never more than `cap`. Print the delays for 6 retries with base 0.5 and cap 8.
@@starter
def backoff_delays(retries, base, cap):
    # attempt 0 -> base, attempt 1 -> base*2, attempt 2 -> base*4 ... but at most cap
    return []


print(backoff_delays(6, 0.5, 8))
@@expected
[0.5, 1.0, 2.0, 4.0, 8, 8]
@@hint
A list comprehension: `[min(cap, base * 2 ** i) for i in range(retries)]`. Note that `min` returns `cap` itself (the int 8) when it's smaller.
@@solution
def backoff_delays(retries, base, cap):
    return [min(cap, base * 2 ** attempt) for attempt in range(retries)]


print(backoff_delays(6, 0.5, 8))

@@quiz
Users get logged out randomly when the load balancer sends them to a different server. What's the cloud-native fix?
* Store session state outside the app servers (a database or cache like Redis), keeping the servers stateless
- Turn off the load balancer
- Use bigger servers
- Write sessions to each server's local disk
? Sticky sessions are a workaround, but stateless servers can scale and be replaced freely.

@@quiz
Why add jitter (randomness) to retry delays?
* So many clients don't all retry at exactly the same moment and overload the service again
- To make debugging harder
- Because timers are inaccurate
- To encrypt the request
? A "thundering herd" of synchronized retries can keep a recovering service down.

@@quiz
Where should a twelve-factor app get its database password?
* From the environment (injected from a secret store at runtime)
- Hard-coded in the source
- Baked into the image
- From a comment in the Dockerfile
? Config differs between environments; code and images don't.

@@quiz
A function resizes images uploaded to object storage, a few hundred times a day. What fits best?
* A serverless function triggered by the upload event
- A large VM running all the time
- A Kubernetes cluster
- A mainframe
? Pay per execution, scale to zero when idle.

@@quiz
A web front end and an order processor must keep working when traffic spikes, without losing orders. What helps most?
* A message queue between them
- A shared local disk
- Synchronous calls with no timeout
- Removing health checks
? The queue absorbs bursts; the processor works through it at its own pace.

@@quiz
What does a circuit breaker do?
* Stops calling a failing dependency for a while, failing fast instead of piling up waiting requests
- Cuts power to a server
- Encrypts traffic
- Restarts the database
? It "half-opens" later to test whether the dependency has recovered.

@@flashcards
cattle, not pets :: Servers are replaced, not nursed back to health
twelve-factor :: Cloud-native app methodology
stateless :: No local state: any instance can serve any request
exponential backoff :: Retry waits double each time
jitter :: Randomness added to retry timing
idempotent :: Repeating it has the same effect as doing it once
circuit breaker :: Fail fast while a dependency is down
immutable infrastructure :: Replace, don't patch in place
FaaS :: Functions as a Service: Lambda, Azure Functions
cold start :: Delay when a new function instance starts
monolith vs microservices :: One deployable vs many small services
event-driven :: Services react to events, loosely coupled

@@drill
What's the word for an operation that has the same effect whether you do it once or many times?
= idempotent
? PUT and DELETE should be idempotent; POST usually isn't.

@@exercise Circuit breaker
@@concepts objects, conditions, loops
A circuit breaker stops a service from hammering a dependency that is down. **Closed**: calls go through, and after 3 consecutive failures it **opens**. **Open**: calls fail fast without touching the dependency until a 10-second cool-down has passed, then the breaker goes **half-open**. **Half-open**: one trial call goes through; success closes the breaker, failure opens it again for another cool-down. Run the timeline of calls and print what happens to each.
@@starter
class CircuitBreaker:
    def __init__(self, threshold=3, cooldown=10):
        self.threshold = threshold
        self.cooldown = cooldown
        self.state = "closed"
        self.failures = 0
        self.opened_at = None

    def call(self, now, dependency_ok):
        raise NotImplementedError

calls = [(0, True), (1, False), (2, False), (3, False), (4, True), (8, True),
         (13, False), (15, True), (24, True), (25, True), (26, False)]
cb = CircuitBreaker()
for now, ok in calls:
    result = cb.call(now, ok)
    print(f"t={now:>2}s dependency {'up  ' if ok else 'down'} -> {result:<32} state={cb.state}")
@@expected
t= 0s dependency up   -> success                          state=closed
t= 1s dependency down -> failure (1/3)                    state=closed
t= 2s dependency down -> failure (2/3)                    state=closed
t= 3s dependency down -> failure, breaker opens           state=open
t= 4s dependency up   -> rejected (fail fast)             state=open
t= 8s dependency up   -> rejected (fail fast)             state=open
t=13s dependency down -> failure, breaker opens again     state=open
t=15s dependency up   -> rejected (fail fast)             state=open
t=24s dependency up   -> success (trial)                  state=closed
t=25s dependency up   -> success                          state=closed
t=26s dependency down -> failure (1/3)                    state=closed
@@requires
def call
self.state = "half-open"
@@hint
If the state is "open" and `now - self.opened_at < self.cooldown`, return "rejected (fail fast)". Otherwise, if it's open, switch to "half-open" first. Then make the call: on success reset `failures` and set "closed"; on failure, increase `failures`, and open the breaker (recording `opened_at = now`) if it was half-open or the failures reached the threshold.
@@solution
class CircuitBreaker:
    def __init__(self, threshold=3, cooldown=10):
        self.threshold = threshold
        self.cooldown = cooldown
        self.state = "closed"
        self.failures = 0
        self.opened_at = None

    def call(self, now, dependency_ok):
        if self.state == "open":
            if now - self.opened_at < self.cooldown:
                return "rejected (fail fast)"
            self.state = "half-open"
        trial = self.state == "half-open"
        if dependency_ok:
            self.failures = 0
            self.state = "closed"
            return "success (trial)" if trial else "success"
        self.failures += 1
        if trial or self.failures >= self.threshold:
            self.state = "open"
            self.opened_at = now
            return "failure, breaker opens" + (" again" if trial else "")
        return f"failure ({self.failures}/{self.threshold})"

calls = [(0, True), (1, False), (2, False), (3, False), (4, True), (8, True),
         (13, False), (15, True), (24, True), (25, True), (26, False)]
cb = CircuitBreaker()
for now, ok in calls:
    result = cb.call(now, ok)
    print(f"t={now:>2}s dependency {'up  ' if ok else 'down'} -> {result:<32} state={cb.state}")

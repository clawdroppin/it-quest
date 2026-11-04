@@lesson d1-deploy-strategies Deployment Strategies: Rolling, Blue-Green, Canary and Feature Flags
@@text
Deploying is the riskiest routine thing a team does. Most outages start with a change, so mature teams make deploys small, gradual and reversible, and separate **deploying** code from **releasing** features. This lesson compares the strategies, explains how automated canary analysis decides whether to continue, and shows how feature flags roll a feature out to a stable percentage of users.

## The strategies

| Strategy | How | Rollback | Cost / risk |
|---|---|---|---|
| **Recreate** | stop old, start new | redeploy old | downtime |
| **Rolling** | replace instances in batches (Kubernetes default) | roll back in batches | two versions live at once; needs compatible APIs/schemas |
| **Blue-green** | full new environment (green), switch traffic at once | **switch back instantly** | double capacity during the switch |
| **Canary** | send a small % of traffic to the new version, increase in steps while metrics are healthy | shift traffic back | needs good metrics and traffic splitting |
| **Shadow / mirroring** | copy production traffic to the new version, discard responses | nothing to roll back | extra load; careful with side effects |
| **A/B testing** | split users to compare **business** outcomes | — | a product experiment, not a safety mechanism |

## Canary analysis

A canary is compared with the **baseline** (current version) on the same metrics during the same time: error rate, latency percentiles, saturation, and business metrics. Tools (Argo Rollouts, Flagger, Spinnaker/Kayenta) automate it:
- Each step (e.g. 5% → 25% → 50% → 100%) waits for an **analysis window**.
- A metric **fails** if the canary is worse than the baseline by more than a threshold (e.g. error rate more than 1.5× baseline and above an absolute floor; p99 latency more than 20% higher).
- Any failing metric → **automatic rollback**; all passing → next step.
- Too little traffic → inconclusive; wait or extend the window.

## Database changes

Use **expand and contract** (parallel change): add new columns/tables (expand), deploy code that writes both and reads new, backfill, switch reads, then remove the old (contract) in a later release. Never deploy a schema change that the currently running version can't handle.

## Feature flags

- Deploy code **dark** (flag off), then release by turning the flag on: for internal users, a percentage, a region, or a customer segment. **Kill switch** to turn a broken feature off without deploying.
- Percentage rollouts must be **sticky**: hash a stable user ID with the flag name into a bucket 0–99 and enable the flag when bucket < percentage. Increasing the percentage only adds users; nobody flips back and forth.
- Flag types: release, experiment, ops (kill switches, degradation), permission. Remove release flags after rollout: stale flags are technical debt.

## Progressive delivery checklist

Health checks and readiness probes, automated rollback, observability per version (label metrics with the version), small batches, deploy during working hours, a tested rollback path, and a change log linked to deploys.

@@quiz
Which strategy allows an instant rollback by switching traffic back to the old environment?
- Rolling
* Blue-green
- Recreate
- A/B testing
? Both environments exist during the switch.

@@quiz
What does a canary compare against during analysis?
- Last week's metrics
* The baseline version running at the same time
- The SLA
- The load test
? Same time, same traffic conditions.

@@quiz
A schema change renames a column used by the running version. What approach avoids downtime?
- Deploy schema and code at the same time
* Expand and contract: add the new column, migrate, switch, remove later
- Blue-green with a shared database and the rename
- Recreate deployment
? Each step stays compatible with the running code.

@@quiz
Why hash the user ID for percentage rollouts?
* So each user consistently gets the same experience as the percentage grows
- To encrypt the user ID
- To speed up the flag service
- To randomise every request
? Sticky bucketing.

@@quiz
Which is a product experiment rather than a safety mechanism?
- Canary
- Blue-green
* A/B testing
- Shadow traffic
? It compares business outcomes.

@@flashcards
deploy vs release :: ship code vs enable feature
recreate :: downtime
rolling :: batches, two versions live
blue-green :: switch traffic, instant rollback
canary :: small % first, grow with analysis
shadow traffic :: mirror, discard responses
A/B test :: business experiment
baseline :: current version for comparison
analysis window :: time per canary step
expand and contract :: compatible schema changes
feature flag :: runtime toggle
kill switch :: turn feature off without deploy
sticky bucketing :: hash user + flag
stale flag :: remove after rollout
Argo Rollouts :: Kubernetes progressive delivery

@@drill
Which deployment strategy mirrors production traffic to the new version and discards its responses?
= shadow
= shadowing
= traffic mirroring
= mirroring
= shadow deployment
? Useful for testing under real load.

@@drill
What is the database pattern of adding new structures, migrating, then removing old ones?
= expand and contract
= parallel change
? Each step is backwards compatible.

@@exercise Automated canary analysis
Simulate a canary rollout through `STEPS`. At each step, compare canary and baseline metrics: error rate fails if canary > baseline × 1.5 and > 0.5%; p99 latency fails if canary > baseline × 1.2; fewer than `MIN_REQUESTS` canary requests makes the step inconclusive (repeat it once, then fail). Print each step's decision and the final outcome for each release.
@@lang python
@@starter
STEPS = [5, 25, 50, 100]
MIN_REQUESTS = 500
# per step: (canary requests, canary errors, canary p99 ms, baseline error rate %, baseline p99 ms)
releases = {
    "v2.3.0": [(800, 2, 210, 0.3, 200), (4000, 12, 205, 0.3, 200), (8000, 30, 215, 0.35, 205), (16000, 50, 210, 0.3, 200)],
    "v2.4.0": [(900, 3, 230, 0.3, 200), (4200, 40, 260, 0.3, 200)],
    "v2.4.1": [(300, 0, 190, 0.3, 200), (700, 1, 195, 0.3, 200), (3800, 9, 199, 0.3, 200), (7900, 20, 202, 0.3, 200), (15800, 41, 201, 0.3, 200)],
}

def judge(sample):
    raise NotImplementedError
@@expected
== v2.3.0
    5%: pass (errors 0.25%, p99 210 ms)
   25%: pass (errors 0.30%, p99 205 ms)
   50%: pass (errors 0.38%, p99 215 ms)
  100%: pass (errors 0.31%, p99 210 ms)
  result: promoted to 100%
== v2.4.0
    5%: pass (errors 0.33%, p99 230 ms)
   25%: fail (error rate 0.95% vs baseline 0.3%; p99 260 ms vs baseline 200 ms)
  result: rolled back at 25%
== v2.4.1
    5%: inconclusive (only 300 requests)
    5%: pass (errors 0.14%, p99 195 ms)
   25%: pass (errors 0.24%, p99 199 ms)
   50%: pass (errors 0.25%, p99 202 ms)
  100%: pass (errors 0.26%, p99 201 ms)
  result: promoted to 100%
@@requires
def judge
MIN_REQUESTS
1.5
@@hint
`judge` returns `("pass" | "fail" | "inconclusive", reasons)`. Walk the samples with an index: an inconclusive step consumes one sample and is retried with the next sample at the same weight; a second inconclusive result fails.
@@solution
STEPS = [5, 25, 50, 100]
MIN_REQUESTS = 500
releases = {
    "v2.3.0": [(800, 2, 210, 0.3, 200), (4000, 12, 205, 0.3, 200), (8000, 30, 215, 0.35, 205), (16000, 50, 210, 0.3, 200)],
    "v2.4.0": [(900, 3, 230, 0.3, 200), (4200, 40, 260, 0.3, 200)],
    "v2.4.1": [(300, 0, 190, 0.3, 200), (700, 1, 195, 0.3, 200), (3800, 9, 199, 0.3, 200), (7900, 20, 202, 0.3, 200), (15800, 41, 201, 0.3, 200)],
}

def judge(sample):
    requests, errors, p99, base_err, base_p99 = sample
    if requests < MIN_REQUESTS:
        return "inconclusive", [f"only {requests} requests"]
    rate = errors / requests * 100
    reasons = []
    if rate > base_err * 1.5 and rate > 0.5:
        reasons.append(f"error rate {rate:.2f}% vs baseline {base_err}%")
    if p99 > base_p99 * 1.2:
        reasons.append(f"p99 {p99} ms vs baseline {base_p99} ms")
    return ("fail" if reasons else "pass"), reasons or [f"errors {rate:.2f}%, p99 {p99} ms"]

for release, samples in releases.items():
    print(f"== {release}")
    i, outcome = 0, "promoted to 100%"
    for weight in STEPS:
        retried = False
        while True:
            verdict, reasons = judge(samples[i])
            i += 1
            print(f"  {weight:>3}%: {verdict} ({'; '.join(reasons)})")
            if verdict == "inconclusive" and not retried:
                retried = True
                continue
            break
        if verdict != "pass":
            outcome = f"rolled back at {weight}%"
            break
    print("  result:", outcome)

@@exercise Sticky percentage rollouts
Implement feature flag evaluation: a user is in the rollout when `bucket(user, flag) < percentage`, where the bucket is the first 4 bytes of SHA-256 of `"flag:user"` modulo 100. Allow-listed users always get the flag, and a kill switch turns it off for everyone. Show that raising the percentage only adds users.
@@lang python
@@starter
import hashlib

users = [f"user-{n:03d}" for n in range(1, 21)]
flag = "new-checkout"
allow_list = {"user-007"}

def bucket(user, flag):
    raise NotImplementedError
@@expected
 10%:  2 users on, newly added: user-007, user-013; nobody removed: True
 25%:  7 users on, newly added: user-001, user-006, user-008, user-017, user-019; nobody removed: True
 50%: 10 users on, newly added: user-002, user-018, user-020; nobody removed: True
100%: 20 users on, newly added: user-003, user-004, user-005, user-009, user-010, user-011, user-012, user-014, user-015, user-016; nobody removed: True
kill switch: 0 users on
@@requires
def bucket
hashlib.sha256(
int.from_bytes(
@@hint
`int.from_bytes(hashlib.sha256(f"{flag}:{user}".encode()).digest()[:4], "big") % 100`. For each percentage, compute the enabled set; check that each set contains the previous one.
@@solution
import hashlib

users = [f"user-{n:03d}" for n in range(1, 21)]
flag = "new-checkout"
allow_list = {"user-007"}

def bucket(user, flag):
    return int.from_bytes(hashlib.sha256(f"{flag}:{user}".encode()).digest()[:4], "big") % 100

def enabled(user, percentage, kill_switch=False):
    if kill_switch:
        return False
    return user in allow_list or bucket(user, flag) < percentage

previous = set()
for pct in (10, 25, 50, 100):
    on = {u for u in users if enabled(u, pct)}
    added = sorted(on - previous)
    print(f"{pct:>3}%: {len(on):>2} users on, newly added: {', '.join(added) or '-'}; nobody removed: {previous <= on}")
    previous = on
print("kill switch:", sum(enabled(u, 100, kill_switch=True) for u in users), "users on")

@@lesson gc1-sre Operations and SRE: Monitoring, Logging, SLOs and Error Budgets
@@text
Google invented Site Reliability Engineering, and its exams expect you to think in SRE terms: define reliability with SLIs and SLOs, spend an error budget deliberately, alert on symptoms users feel, and run blameless incident reviews. This lesson combines those ideas with Google Cloud Observability (Cloud Monitoring, Logging, Trace, Profiler).

## SLI, SLO, SLA

- **SLI** (indicator): a measured ratio of good events to valid events. Availability = successful requests / total requests; latency = requests faster than 300 ms / total.
- **SLO** (objective): a target for the SLI over a window: "99.9% of requests succeed over 28 days".
- **SLA** (agreement): a contract with penalties; set it looser than the internal SLO.
- **Error budget** = 1 − SLO. At 99.9% over 28 days: 0.1% of requests may fail, or about **40 minutes** of full downtime (28 × 24 × 60 × 0.001 ≈ 40.3).

| SLO | Downtime per 30 days |
|---|---|
| 99% | 7.2 hours |
| 99.9% | 43.2 minutes |
| 99.95% | 21.6 minutes |
| 99.99% | 4.3 minutes |

**Error budget policy**: if the budget is spent, freeze risky releases and prioritise reliability work; if plenty remains, ship faster. That converts the "dev vs ops" argument into data.

## Alerting on burn rate

**Burn rate** = observed error rate / allowed error rate. Burn rate 1 spends the budget exactly over the window; burn rate 14.4 spends 2% of a 30-day budget in one hour. Good practice is **multi-window, multi-burn-rate** alerts:
- Page: burn rate > 14.4 over 1 hour (and > 14.4 over 5 minutes to confirm it's still happening).
- Page: burn rate > 6 over 6 hours.
- Ticket: burn rate > 1 over 3 days.

Alert on **symptoms** (users see errors/latency), not causes (CPU at 90%), to reduce noise. Every page should be actionable.

## Cloud Monitoring

- Metrics from Google services automatically; VM OS/app metrics via the **Ops Agent**; custom metrics; **Managed Service for Prometheus**.
- **Dashboards**, **uptime checks** (global HTTP/TCP probes), **alerting policies** (conditions on metrics, log-based metrics, or **SLO burn rate**) with **notification channels** (email, SMS, Slack, PagerDuty, Pub/Sub, webhooks).
- **Service monitoring** defines services and SLOs in the console.
- Multi-project views with **metrics scopes**.

## Cloud Logging

- **Log Router** sends entries through **sinks** to log buckets, BigQuery, Cloud Storage or Pub/Sub; exclusion filters drop noisy logs (cost).
- Buckets: `_Required` (Admin Activity/System Event audit logs, **400 days**, can't change), `_Default` (**30 days** default, configurable), user-defined buckets (custom retention, **Log Analytics** with SQL).
- **Log-based metrics** count or extract values from logs for alerting.
- Query with the Logging query language: `resource.type="cloud_run_revision" severity>=ERROR`.

## Tracing and profiling

**Cloud Trace** (latency per request across services, OpenTelemetry), **Cloud Profiler** (CPU/heap in production, low overhead), **Error Reporting** (groups exceptions).

## Incident management

Roles: **incident commander**, communications lead, operations lead. Mitigate first (roll back, drain traffic), then fix. Afterwards, a **blameless postmortem**: timeline, impact, root cause(s), what went well, action items with owners. Reduce **toil** (manual, repetitive, automatable work; SRE aims to keep it under 50% of time).

Release safety: canary releases, progressive rollouts (Cloud Deploy), feature flags, automatic rollback on SLO regression.

@@quiz
An SLO of 99.9% over 30 days allows how much full downtime?
- 4.3 minutes
* About 43 minutes
- 7.2 hours
- 1 day
? 30 × 24 × 60 × 0.001.

@@quiz
The error budget for the month is exhausted. What does a typical error budget policy say?
- Lower the SLO
* Freeze risky releases and prioritise reliability work until the budget recovers
- Add more alerts
- Increase the SLA
? The budget makes the trade-off explicit.

@@quiz
Which alert is most likely to be actionable and low-noise?
- CPU above 80% on any VM
* Error budget burn rate above 14.4 over the last hour
- Disk I/O above average
- Any log line with WARNING
? Alert on user-facing symptoms.

@@quiz
How long does the `_Default` log bucket keep logs by default?
- 7 days
* 30 days
- 400 days
- Forever
? `_Required` keeps audit logs 400 days.

@@quiz
Which tool shows where time is spent across microservices for one slow request?
- Cloud Profiler
* Cloud Trace
- Error Reporting
- Uptime checks
? Distributed tracing.

@@flashcards
SLI :: good events / valid events
SLO :: target for an SLI over a window
SLA :: contract with penalties
error budget :: 1 − SLO
99.9% per 30 days :: 43.2 minutes
99.99% per 30 days :: 4.3 minutes
burn rate :: observed / allowed error rate
burn rate 14.4 for 1 h :: 2% of 30-day budget
alert on :: symptoms, not causes
Ops Agent :: VM metrics and logs
uptime check :: external probe
notification channel :: where alerts go
Log Router sink :: send logs to destinations
_Required bucket :: audit logs, 400 days
_Default bucket :: 30 days
log-based metric :: metric from log entries
Cloud Trace :: request latency across services
Cloud Profiler :: CPU/heap in production
blameless postmortem :: learn without blame
toil :: manual, repetitive, automatable work

@@drill
What is the error budget of a 99.95% SLO, as a percentage?
= 0.05%
= 0.05
? 1 − SLO.

@@drill
How many days does the _Required log bucket keep audit logs?
= 400
= 400 days
? It can't be changed.

@@exercise Track an error budget
Given daily request and error counts for a 28-day SLO window, compute the SLO compliance so far, the error budget (allowed failed requests for the requests so far), how much of it is used, and the burn rate of the last 3 days. Print a status: `freeze releases` if more than 100% is used, `caution` above 75%, else `ok`.
@@lang python
@@starter
SLO = 0.999
days = [(1_200_000, 300), (1_180_000, 250), (1_250_000, 400), (1_300_000, 2_900), (1_220_000, 350),
        (1_210_000, 280), (1_190_000, 310), (1_260_000, 1_700), (1_240_000, 2_100), (1_230_000, 1_900)]
@@expected
Requests 12,280,000, errors 10,490
Compliance 99.9146% (SLO 99.9%)
Error budget 12,280 failed requests, used 85%
Burn rate last 3 days: 1.53
Status: caution
@@requires
1 - SLO
burn
@@hint
Budget = `requests * (1 - SLO)`. Burn rate for recent days = `(errors / requests) / (1 - SLO)` over those days.
@@solution
SLO = 0.999
days = [(1_200_000, 300), (1_180_000, 250), (1_250_000, 400), (1_300_000, 2_900), (1_220_000, 350),
        (1_210_000, 280), (1_190_000, 310), (1_260_000, 1_700), (1_240_000, 2_100), (1_230_000, 1_900)]

requests = sum(r for r, _ in days)
errors = sum(e for _, e in days)
compliance = 1 - errors / requests
budget = requests * (1 - SLO)
used = errors / budget
recent_r = sum(r for r, _ in days[-3:])
recent_e = sum(e for _, e in days[-3:])
burn = (recent_e / recent_r) / (1 - SLO)

print(f"Requests {requests:,}, errors {errors:,}")
print(f"Compliance {compliance:.4%} (SLO {SLO:.1%})")
print(f"Error budget {budget:,.0f} failed requests, used {used:.0%}")
print(f"Burn rate last 3 days: {burn:.2f}")
status = "freeze releases" if used > 1 else "caution" if used > 0.75 else "ok"
print("Status:", status)

@@exercise Multi-window burn-rate alerts
Simulate alert evaluation at the end of each hour for a 30-day SLO of 99.9%. Hourly data is `(requests, errors)`. Rules: **page** when the burn rate over the last 1 hour exceeds 14.4 **and** over the last 6 hours exceeds 6; **ticket** when the burn rate over the last 24 hours (or all hours so far) exceeds 1. Print each hour where the state changes.
@@lang python
@@starter
SLO = 0.999
hours = [(50_000, 20)] * 6 + [(52_000, 900), (51_000, 1_100), (50_000, 60), (49_000, 40)] + [(50_000, 70)] * 10 + [(50_000, 30)] * 4
@@expected
hour  6: ok -> ticket (1h 17.3, 6h 3.3, 24h 2.90)
hour  7: ticket -> page (1h 21.6, 6h 6.9, 24h 5.26)
hour  8: page -> ticket (1h 1.2, 6h 7.0, 24h 4.81)
final: ticket
@@requires
def burn_rate
14.4
@@hint
`burn_rate(window)` sums requests and errors over the slice and divides the error ratio by `1 - SLO`. Use `hours[max(0, i - n + 1): i + 1]` for the last n hours.
@@solution
SLO = 0.999
hours = [(50_000, 20)] * 6 + [(52_000, 900), (51_000, 1_100), (50_000, 60), (49_000, 40)] + [(50_000, 70)] * 10 + [(50_000, 30)] * 4

def burn_rate(window):
    r = sum(x for x, _ in window)
    e = sum(y for _, y in window)
    return (e / r) / (1 - SLO)

state = "ok"
for i in range(len(hours)):
    last = lambda n: hours[max(0, i - n + 1): i + 1]
    b1, b6, b24 = burn_rate(last(1)), burn_rate(last(6)), burn_rate(last(24))
    if b1 > 14.4 and b6 > 6:
        new = "page"
    elif b24 > 1:
        new = "ticket"
    else:
        new = "ok"
    if new != state:
        print(f"hour {i:>2}: {state} -> {new} (1h {b1:.1f}, 6h {b6:.1f}, 24h {b24:.2f})")
        state = new
print("final:", state)

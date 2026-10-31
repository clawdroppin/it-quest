@@lesson d2-observability Observability, SRE and Incidents
@@lang python
@@text
**Site Reliability Engineering (SRE)**, from Google, treats operations as a software problem. The key idea: 100% reliability is the wrong target. It's impossibly expensive, and users can't tell 99.99% from 100% anyway. Instead, agree on *how reliable is reliable enough*, and spend the rest on shipping features.

## SLI, SLO, SLA, error budget

| Term | Meaning | Example |
|---|---|---|
| **SLI** (indicator) | a measurement of user experience | % of requests that succeed; % served under 300 ms |
| **SLO** (objective) | the target for an SLI | 99.5% of requests succeed over 30 days |
| **SLA** (agreement) | a contract with consequences (refunds) | 99.0%, deliberately looser than the SLO |
| **Error budget** | 100% − SLO: the failure you're *allowed* | 0.5% of requests, ~3.6 hours a month of full outage |

When the budget is healthy, ship faster and take risks. When it's spent, freeze risky releases and fix reliability. That turns "dev vs ops" arguments into a number both sides agreed on. **Burn rate** = how fast the budget is being used (1.0 = exactly on pace to use it all by the end of the window); alerting on high burn rates catches real problems without paging on every blip.

| Availability | Downtime per year | per 30 days |
|---|---|---|
| 99% ("two nines") | 3.65 days | 7.2 h |
| 99.9% | 8.76 h | 43 min |
| 99.95% | 4.38 h | 22 min |
| 99.99% | 52.6 min | 4.3 min |

## The three pillars of observability

- **Metrics**: numbers over time (CPU, request rate, error rate, latency **percentiles**). **Prometheus** scrapes them; **Grafana** graphs them; cloud: CloudWatch, Azure Monitor. Watch **p95/p99**, not averages: an average hides the slow requests real users feel.
- **Logs**: events with detail. **Structured** (JSON) logs can be searched and aggregated: ELK/Elastic, Loki, Splunk, CloudWatch Logs, Log Analytics.
- **Traces**: one request's path through many services, with timings (Jaeger, Tempo, X-Ray). **OpenTelemetry** is the open standard for producing all three.

What to watch: Google's **four golden signals**: **latency, traffic, errors, saturation**. For resources, **USE** (utilization, saturation, errors); for services, **RED** (rate, errors, duration).

## Alerting and on-call

- Alert on **symptoms users feel** (error rate, latency, SLO burn), not on every cause (CPU 80% is not an emergency by itself).
- Every page must be **actionable** and link to a **runbook**. Alert fatigue makes people ignore the alert that matters.
- On-call rotations with tools like PagerDuty, Opsgenie, or Grafana OnCall.

## Incidents

1. **Detect** (alert or user report) → declare an incident, assign an **incident commander**, open a channel.
2. **Mitigate first** (roll back, fail over, scale up), and find the root cause *later*.
3. Communicate regularly (status page, stakeholders).
4. **Resolve**, then write a **blameless postmortem**: timeline, impact, root cause and contributing factors, what went well, **action items** with owners. Blameless means asking *"why did the system let this happen?"*, not *"who did it?"*, so people report problems honestly.

Measures: **MTTD** (time to detect), **MTTR** (time to restore/repair), **MTBF** (between failures).

**Toil** is manual, repetitive, automatable operational work. SRE teams cap it (Google: under 50%) and automate the rest.

@@exercise Downtime budget
Write `allowed_downtime_minutes(slo, days)` returning how many minutes of full outage an availability SLO allows over a period, rounded to 1 decimal. Print it for 99.9% over 30 days and 99.99% over 365 days.
@@starter
def allowed_downtime_minutes(slo, days):
    # total minutes in the period, times the fraction that may fail
    return 0


print(allowed_downtime_minutes(0.999, 30))
print(allowed_downtime_minutes(0.9999, 365))
@@expected
43.2
52.6
@@hint
A day has 24 * 60 minutes. The budget is (1 - slo) of that. Use `round(value, 1)`.
@@solution
def allowed_downtime_minutes(slo, days):
    return round(days * 24 * 60 * (1 - slo), 1)


print(allowed_downtime_minutes(0.999, 30))
print(allowed_downtime_minutes(0.9999, 365))

@@quiz
The SLO is 99.9% of requests successful over 30 days. What's the error budget?
* 0.1% of requests can fail in that window
- 99.9% of requests can fail
- No requests may ever fail
- 0.1 requests per day
? 100% − SLO.

@@quiz
Why should the SLA usually be looser than the internal SLO?
* So the team gets warned and acts before contractual penalties apply
- Because SLAs are never measured
- To make customers unhappy
- It should always be stricter
? The SLO is the early-warning line.

@@quiz
The error budget for this month is used up. What does the SRE approach suggest?
* Pause risky feature releases and prioritize reliability work
- Raise the SLO
- Ignore it until next month
- Turn off monitoring
? The budget is the agreed rule for balancing speed and stability.

@@quiz
Why look at p99 latency instead of the average?
* Averages hide the slow requests that some users actually experience
- p99 is always lower
- Averages can't be calculated
- p99 is required by law
? With 1 million requests, 1% is 10,000 slow experiences.

@@quiz
Which are the four golden signals?
* Latency, traffic, errors, saturation
- CPU, RAM, disk, network
- Logs, metrics, traces, alerts
- Plan, do, check, act
? From Google's SRE book.

@@quiz
During an outage, what comes first?
* Mitigate (e.g. roll back) to restore service, then find the root cause
- Find who caused it
- Write the postmortem
- Wait for more data
? Restoring users comes before understanding.

@@quiz
What is a blameless postmortem?
* A review that looks for system and process causes and actions, not someone to punish
- A meeting where the guilty person apologizes
- A document nobody reads
- A legal report
? Blame makes people hide mistakes; learning needs honesty.

@@quiz
Which open standard produces metrics, logs and traces in a vendor-neutral way?
* OpenTelemetry
- SNMP
- Syslog only
- ITIL
? Instrument once, send to any backend.

@@flashcards
SRE :: Site Reliability Engineering: operations as software engineering
SLI :: Measured indicator of user experience
SLO :: Target for an SLI
SLA :: Contract with penalties; looser than the SLO
error budget :: 100% − SLO: allowed unreliability
burn rate :: Speed of budget consumption (1.0 = on pace)
99.9% per 30 days :: About 43 minutes of downtime
metrics / logs / traces :: Numbers over time / events / request paths
Prometheus / Grafana :: Metrics collection / dashboards
OpenTelemetry :: Open standard for telemetry
four golden signals :: Latency, traffic, errors, saturation
USE / RED :: Utilization-saturation-errors / rate-errors-duration
runbook :: Step-by-step response for an alert
incident commander :: Coordinates the incident response
blameless postmortem :: Learning review without blame
MTTR / MTTD / MTBF :: Time to restore / to detect / between failures
toil :: Manual, repetitive, automatable work

@@drill
What's the error budget, in percent, for an SLO of 99.95%? (just the number)
= 0.05
? 100 − 99.95 = 0.05%.

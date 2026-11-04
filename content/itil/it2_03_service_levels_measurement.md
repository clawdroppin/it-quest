@@lesson it2-service-levels Service Levels, Availability, Capacity, Continuity and Measurement
@@text
Users judge a service by whether it's there when they need it, fast enough, and back quickly when it fails. ITIL 4 spreads this over several practices: **service level management**, **availability management**, **capacity and performance management**, **service continuity management** and **measurement and reporting**, plus **relationship** and **supplier management**. This lesson gives their exam definitions and the numbers practitioners use (marked **(beyond Foundation)**).

## Service level management (exam)

- Purpose: set clear **business-based targets** for service levels, and ensure delivery is properly assessed, monitored and managed against them.
- **SLA**: a documented agreement between a service provider and a customer that identifies both the services required and the expected level of service.
- Good SLAs relate to defined services, reflect **outcomes** (not just operational metrics), are written in simple language, and are agreed by all parties.
- The **watermelon effect**: green metrics, unhappy users. Combat it with customer engagement (listening, discovery, feedback surveys, business metrics).
- Related (beyond Foundation): **OLAs** with internal teams and **underpinning contracts** with suppliers must support the SLA.

## Availability (exam + practice)

- **Availability management** ensures services deliver agreed levels of availability to meet the needs of customers and users.
- **Availability** = agreed service time − downtime, divided by agreed service time. Only downtime **inside** the agreed hours counts.
- **MTBF** (mean time between failures) measures reliability; **MTRS/MTTR** (mean time to restore service) measures how quickly service comes back. Availability ≈ MTBF / (MTBF + MTRS).
- Components in **series** multiply: two 99.9% components give 99.8%. Components in **parallel** (redundant) give 1 − (1 − A)².

## Capacity and performance (exam)

Ensure services achieve agreed and expected performance, satisfying current and future demand **cost-effectively**. Activities: forecasting demand, performance monitoring, tuning, capacity planning, modelling.

## Service continuity (exam + practice)

- Ensure availability and performance are maintained at sufficient levels **in case of a disaster**.
- **RTO** (recovery time objective): maximum acceptable time before the service is restored after a disruption.
- **RPO** (recovery point objective): the point to which information must be restored (maximum acceptable data loss).
- **Business impact analysis (BIA)**: identifies vital business functions and their dependencies, to set RTOs and RPOs.
- Disaster: a sudden unplanned event that causes great damage or serious loss to an organisation.

## Measurement and reporting (exam)

Support good decision-making and continual improvement by decreasing uncertainty. Key terms:
- **Metric**: a measurement or calculation that is monitored or reported for management and improvement.
- **Critical success factor (CSF)**: a necessary precondition for achieving intended results.
- **Key performance indicator (KPI)**: an important metric used to evaluate the success in meeting an objective.

Example chain: objective "fast recovery" → CSF "quickly restore service" → KPIs "mean time to restore P1 incidents < 2 h", "% incidents resolved at first contact".

## Relationship and supplier management (exam)

- **Relationship management**: establish and nurture links between the organisation and stakeholders at strategic and tactical levels.
- **Supplier management**: ensure suppliers and their performance are managed appropriately to support seamless products and services. Sourcing models: insourcing, outsourcing, single source or partnership, multi-sourcing.

## Experience level agreements (beyond Foundation)

**XLAs** measure user experience ("I could start working within 5 minutes of reporting a problem", satisfaction scores, lost productive time) alongside technical SLA metrics, to avoid the watermelon effect.

@@quiz
What is the watermelon effect?
- SLAs being too strict
* SLA metrics look green while users are unhappy
- Services failing in series
- Too many KPIs
? Green outside, red inside.

@@quiz
Agreed service time is 200 hours in a month. Downtime totalled 4 hours, of which 1 hour was outside agreed hours. What is the availability?
- 98.0%
* 98.5%
- 99.0%
- 99.5%
? Only 3 hours count: (200 − 3) / 200.

@@quiz
Which term describes the maximum acceptable data loss after a disruption?
- RTO
* RPO
- MTBF
- MTD
? Recovery point objective.

@@quiz
Which statement about a KPI is correct?
* It's an important metric used to evaluate success in meeting an objective
- It's a precondition for achieving intended results
- It's a contract with a supplier
- It's the same as an SLA
? A CSF is the precondition.

@@quiz
Two components each 99% available are both needed for a service. What is the service availability?
- 99%
* 98.01%
- 99.99%
- 98.5%
? Series: multiply.

@@quiz
Which practice ensures services achieve agreed performance and meet future demand cost-effectively?
- Availability management
* Capacity and performance management
- Service level management
- Supplier management
? Forecasting, monitoring, tuning.

@@flashcards
SLM purpose :: business-based service level targets
SLA :: documented provider-customer agreement
watermelon effect :: green metrics, unhappy users
OLA :: agreement with an internal team
underpinning contract :: agreement with a supplier
availability :: (agreed time − downtime) / agreed time
MTBF :: reliability
MTRS :: speed of restoring service
series availability :: multiply
parallel availability :: 1 − (1 − A)^n
capacity mgmt :: performance and demand, cost-effectively
continuity mgmt :: service in case of disaster
RTO :: max time to restore
RPO :: max data loss
BIA :: vital functions and impacts
metric :: measurement monitored or reported
CSF :: necessary precondition
KPI :: metric for an objective
relationship mgmt :: links with stakeholders
supplier mgmt :: manage suppliers' performance
XLA :: experience level agreement

@@drill
What does RTO stand for?
= recovery time objective
= Recovery Time Objective
? RPO is the recovery point objective.

@@drill
What is the availability of two 99.9% components in series, as a percentage to two decimals?
= 99.80%
= 99.8%
= 99.80
? 0.999 × 0.999 = 0.998001.

@@exercise Availability against agreed service hours
The SLA covers Monday–Friday 08:00–20:00 (Japan time, ignoring holidays). Given outages as start and end datetimes, count only the minutes inside agreed hours, then report availability per service against its target, plus MTBF and MTRS in hours (failures = outages that overlapped agreed hours).
@@lang python
@@starter
from datetime import datetime, timedelta

MONTH_START, MONTH_END = datetime(2026, 11, 2), datetime(2026, 11, 28)
TARGETS = {"email": 99.0, "erp": 99.0}
outages = {
    "email": [("2026-11-04 09:00", "2026-11-04 10:30"), ("2026-11-07 10:00", "2026-11-07 15:00"), ("2026-11-18 19:30", "2026-11-18 21:00")],
    "erp": [("2026-11-10 07:00", "2026-11-10 09:00"), ("2026-11-12 13:00", "2026-11-12 16:00"), ("2026-11-24 11:00", "2026-11-24 12:00")],
}

def agreed_minutes(start, end):
    raise NotImplementedError
@@expected
Agreed service time: 240 hours
email: downtime 2.0 h in agreed hours, availability 99.17% (target 99.0%) met
  failures 2, MTBF 119.0 h, MTRS 1.0 h
erp: downtime 5.0 h in agreed hours, availability 97.92% (target 99.0%) MISSED
  failures 3, MTBF 78.3 h, MTRS 1.7 h
@@requires
def agreed_minutes
weekday()
@@hint
Walk day by day from `start.date()` to `end.date()`. For weekdays (`weekday() < 5`), overlap `[start, end)` with `[day 08:00, day 20:00)` and add the minutes. Agreed time for the month is `agreed_minutes(MONTH_START, MONTH_END)`.
@@solution
from datetime import datetime, timedelta

MONTH_START, MONTH_END = datetime(2026, 11, 2), datetime(2026, 11, 28)
TARGETS = {"email": 99.0, "erp": 99.0}
outages = {
    "email": [("2026-11-04 09:00", "2026-11-04 10:30"), ("2026-11-07 10:00", "2026-11-07 15:00"), ("2026-11-18 19:30", "2026-11-18 21:00")],
    "erp": [("2026-11-10 07:00", "2026-11-10 09:00"), ("2026-11-12 13:00", "2026-11-12 16:00"), ("2026-11-24 11:00", "2026-11-24 12:00")],
}

def agreed_minutes(start, end):
    total = 0
    day = datetime.combine(start.date(), datetime.min.time())
    while day < end:
        if day.weekday() < 5:
            open_, close = day + timedelta(hours=8), day + timedelta(hours=20)
            overlap = (min(end, close) - max(start, open_)).total_seconds() / 60
            total += max(0, overlap)
        day += timedelta(days=1)
    return total

agreed = agreed_minutes(MONTH_START, MONTH_END)
print(f"Agreed service time: {agreed / 60:.0f} hours")
for service, spans in outages.items():
    parsed = [(datetime.strptime(a, "%Y-%m-%d %H:%M"), datetime.strptime(b, "%Y-%m-%d %H:%M")) for a, b in spans]
    counted = [agreed_minutes(a, b) for a, b in parsed]
    down = sum(counted)
    failures = sum(1 for c in counted if c > 0)
    availability = (agreed - down) / agreed * 100
    mtbf = (agreed - down) / 60 / failures
    mtrs = down / 60 / failures
    verdict = "met" if availability >= TARGETS[service] else "MISSED"
    print(f"{service}: downtime {down / 60:.1f} h in agreed hours, availability {availability:.2f}% (target {TARGETS[service]}%) {verdict}")
    print(f"  failures {failures}, MTBF {mtbf:.1f} h, MTRS {mtrs:.1f} h")

@@exercise KPIs and the watermelon effect
For each month, compute KPIs from ticket data: % resolved within SLA, first-contact resolution rate, and average satisfaction (1–5). Flag a **watermelon** month when the SLA KPI is green (≥ 95%) but satisfaction is below 3.5 or first-contact resolution below 60%. Print a small report.
@@lang python
@@starter
months = {
    "Aug": [(True, True, 5), (True, True, 4), (True, False, 4), (True, True, 5), (False, True, 3)],
    "Sep": [(True, False, 2), (True, False, 3), (True, True, 4), (True, False, 3), (True, False, 2), (True, True, 4)],
    "Oct": [(True, True, 4), (False, False, 2), (True, False, 3), (False, True, 3), (True, True, 5)],
}

def kpis(tickets):
    raise NotImplementedError
@@expected
Month  SLA   FCR   CSAT  Status
Aug     80%   80%   4.2  SLA red
Sep    100%   33%   3.0  watermelon: green SLA, unhappy users
Oct     60%   60%   3.4  SLA red
@@requires
def kpis
watermelon
@@hint
Each ticket is `(within_sla, first_contact, satisfaction)`. Return three numbers; then classify: SLA green if ≥ 0.95, watermelon if green but satisfaction < 3.5 or FCR < 0.6.
@@solution
months = {
    "Aug": [(True, True, 5), (True, True, 4), (True, False, 4), (True, True, 5), (False, True, 3)],
    "Sep": [(True, False, 2), (True, False, 3), (True, True, 4), (True, False, 3), (True, False, 2), (True, True, 4)],
    "Oct": [(True, True, 4), (False, False, 2), (True, False, 3), (False, True, 3), (True, True, 5)],
}

def kpis(tickets):
    n = len(tickets)
    sla = sum(t[0] for t in tickets) / n
    fcr = sum(t[1] for t in tickets) / n
    csat = sum(t[2] for t in tickets) / n
    return sla, fcr, csat

print("Month  SLA   FCR   CSAT  Status")
for month, tickets in months.items():
    sla, fcr, csat = kpis(tickets)
    if sla >= 0.95 and (csat < 3.5 or fcr < 0.6):
        status = "watermelon: green SLA, unhappy users"
    elif sla >= 0.95:
        status = "green"
    else:
        status = "SLA red"
    print(f"{month:<6} {sla:4.0%}  {fcr:4.0%}  {csat:4.1f}  {status}")

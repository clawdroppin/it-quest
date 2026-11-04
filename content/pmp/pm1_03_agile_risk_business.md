@@lesson pm1-agile-risk-business Agile Delivery, Risk Decisions, Procurement and the Business Environment
@@text
About half of the PMP exam is agile or hybrid, and the Business Environment domain (26%) covers benefits, compliance, organisational change and procurement. This lesson goes deeper into agile metrics and forecasting, quantitative risk decisions (EMV and decision trees), contract types and who carries the risk, and business justification (NPV, payback, benefits realisation).

## Agile delivery

- **Scrum**: product owner (maximises value, owns the backlog), scrum master (servant leader, coach, removes impediments), developers (self-managing). Events: sprint (≤ 1 month), sprint planning, **daily scrum** (15 min, for the developers), sprint review (inspect the increment with stakeholders), retrospective (improve the process). Artefacts: product backlog (product goal), sprint backlog (sprint goal), increment (definition of done).
- **Kanban**: visualise work, **limit work in progress (WIP)**, manage flow, explicit policies; measure **lead time**, **cycle time** and **throughput** (Little's law: average WIP = throughput × cycle time).
- **Velocity**: story points completed per sprint; use the team's own **average** for forecasting; never compare teams' velocities. **Burndown** (work remaining) and **burnup** (work done vs scope: shows scope changes) charts.
- Prioritisation: MoSCoW (must, should, could, won't), Kano, WSJF (cost of delay / job size), value vs effort. **MVP** and **MMF** (minimum marketable feature).
- Hybrid: predictive for hardware/procurement/compliance parts, agile for software; release planning with a fixed date and flexible scope.

## Quantitative risk decisions

- **EMV** (expected monetary value) = probability × impact; threats negative, opportunities positive. **Decision trees** pick the branch with the best EMV including its cost.
- Monte Carlo simulation, sensitivity analysis (tornado diagrams).
- Threat responses: **escalate, avoid, transfer, mitigate, accept**; opportunity responses: **escalate, exploit, share, enhance, accept**. **Risk owner** for each; **residual** and **secondary** risks; **workarounds** for unplanned, occurred risks; **risk triggers** signal a risk is about to happen.
- **Risk appetite/threshold**, issue log (risks that have occurred), and the risk register's living nature: review risks at every iteration/status meeting.

## Procurement and contract types

| Type | Risk mostly on | Use |
|---|---|---|
| **Firm fixed price (FFP)** | seller | well-defined scope |
| **Fixed price incentive fee (FPIF)** | shared, ceiling price | fixed price with performance incentive |
| **Fixed price with economic price adjustment** | shared | long contracts, inflation |
| **Cost plus fixed fee (CPFF)** | buyer | uncertain scope (R&D) |
| **Cost plus incentive fee (CPIF)** | shared by ratio | uncertain scope, cost incentive |
| **Cost plus award fee (CPAF)** | buyer | subjective performance award |
| **Time and materials (T&M)** | buyer | small, quick work; staff augmentation; add a not-to-exceed |

Process: make-or-buy analysis → procurement documents (RFI, **RFQ** for price, **RFP** for solutions) → bidder conferences (equal information to all) → source selection criteria → contract → control (inspections, claims administration, payments) → closure. Agile contracts: fixed price per sprint/feature, capped T&M, money for nothing/change for free.

## Business environment

- **Business case** (why: justification) and **benefits management plan** (when and how benefits are realised and measured, often after the project closes).
- **NPV**: present value of future cash flows minus investment; choose the **higher NPV** (years are already accounted for). **IRR**: higher is better. **Payback period**: shorter is better (ignores time value). **BCR**: > 1 means benefits exceed costs. **Opportunity cost**: the value of the option **not** chosen.
- Compliance: identify requirements, classify, plan, audit; consequences of non-compliance.
- Organisational change: support adoption (ADKAR: awareness, desire, knowledge, ability, reinforcement), transition plans, training.
- Governance: phase gates, PMO types (supportive, controlling, directive).

@@quiz
A team's last four sprints delivered 30, 34, 26 and 30 points. 240 points remain. How many sprints should the PM forecast?
- 6
* 8
- 9
- 10
? Average velocity 30.

@@quiz
Which chart shows scope changes clearly?
- Burndown
* Burnup
- Gantt
- Tornado
? Total scope is a separate line.

@@quiz
A risk has a 30% chance of costing $50,000 and an opportunity a 20% chance of saving $40,000. What is the combined EMV?
- −$23,000
* −$7,000
- $7,000
- −$15,000
? −15,000 + 8,000.

@@quiz
Under which contract type does the seller carry the most cost risk?
* Firm fixed price
- Cost plus fixed fee
- Time and materials
- Cost plus award fee
? The price doesn't change if costs rise.

@@quiz
Project A has NPV $120,000 over 5 years; project B has NPV $100,000 over 3 years. Which should be chosen on NPV alone?
* A
- B
- Neither; compare annual NPV
- Both
? NPV already includes the time value; years don't matter.

@@quiz
Who is responsible for maximising the value of the product and ordering the backlog in Scrum?
- Scrum master
* Product owner
- Developers
- Project sponsor
? The scrum master is a servant leader and coach.

@@quiz
A risk that arises as a direct result of implementing a risk response is called?
- Residual risk
* Secondary risk
- Trigger
- Workaround
? Residual risk remains after the response.

@@flashcards
product owner :: value, backlog order
scrum master :: servant leader, coach
daily scrum :: 15 min, for developers
sprint review :: inspect increment with stakeholders
retrospective :: improve the process
WIP limit :: Kanban flow control
Little's law :: WIP = throughput × cycle time
velocity :: team's own points per sprint
burnup :: shows scope changes
MoSCoW :: must, should, could, won't
WSJF :: cost of delay / job size
EMV :: probability × impact
threat responses :: escalate, avoid, transfer, mitigate, accept
opportunity responses :: escalate, exploit, share, enhance, accept
secondary risk :: caused by a response
workaround :: unplanned response to an occurred risk
FFP :: seller's risk
CPFF :: buyer's risk
T&M :: add a not-to-exceed
RFQ :: price quote
RFP :: proposed solution
bidder conference :: equal information
NPV :: higher is better, years included
payback :: shorter is better
BCR > 1 :: benefits exceed costs
opportunity cost :: value of option not chosen
benefits management plan :: how benefits are realised

@@drill
What is the EMV of a 25% chance of a $40,000 loss?
= -10000
= -$10,000
= −10,000
= -10,000
? Threats are negative.

@@drill
Which Scrum event inspects the increment with stakeholders?
= sprint review
= Sprint Review
= the sprint review
? The retrospective improves the process.

@@exercise Forecast a release with velocity
Use the average of the last `WINDOW` sprints' velocity (and the minimum and maximum of that window for a range) to forecast how many sprints the remaining backlog needs, including scope added each sprint. Print the forecast range and whether the release date (in sprints) is likely, at risk or unlikely, plus the Kanban cycle time from Little's law.
@@lang python
@@starter
import math

WINDOW = 4
velocities = [18, 31, 28, 34, 26, 32]
remaining = 210
scope_growth_per_sprint = 5
sprints_until_release = 8
wip, throughput_per_day = 12, 1.5

def forecast(velocity):
    raise NotImplementedError
@@expected
Velocity (last 4): average 30.0, range 26-34
Forecast: 8 to 10 sprints, expected 9
Release in 8 sprints: unlikely: cut scope (MoSCoW) or move the date
Kanban: WIP 12, throughput 1.5/day -> average cycle time 8.0 days
@@requires
def forecast
math.ceil(
WINDOW
@@hint
With scope growing each sprint, the effective burn is `velocity - scope_growth_per_sprint`; sprints = `ceil(remaining / effective)`. Likely if even the pessimistic (minimum velocity) forecast fits, at risk if only the average fits, unlikely otherwise.
@@solution
import math

WINDOW = 4
velocities = [18, 31, 28, 34, 26, 32]
remaining = 210
scope_growth_per_sprint = 5
sprints_until_release = 8
wip, throughput_per_day = 12, 1.5

def forecast(velocity):
    effective = velocity - scope_growth_per_sprint
    return math.ceil(remaining / effective) if effective > 0 else None

recent = velocities[-WINDOW:]
average = sum(recent) / len(recent)
low, high = min(recent), max(recent)
best, expected, worst = forecast(high), forecast(average), forecast(low)
print(f"Velocity (last {WINDOW}): average {average:.1f}, range {low}-{high}")
print(f"Forecast: {best} to {worst} sprints, expected {expected}")
if worst <= sprints_until_release:
    verdict = "likely"
elif expected <= sprints_until_release:
    verdict = "at risk"
else:
    verdict = "unlikely: cut scope (MoSCoW) or move the date"
print(f"Release in {sprints_until_release} sprints: {verdict}")
print(f"Kanban: WIP {wip}, throughput {throughput_per_day}/day -> average cycle time {wip / throughput_per_day:.1f} days")

@@exercise Decide with EMV and a decision tree
Evaluate decision options. Each option has a cost and a list of outcomes with probability and value (negative for losses). Check probabilities sum to 1, compute EMV of the outcomes, subtract the cost, and choose the best net EMV. Then evaluate a contract choice: compute the buyer's final cost under FPIF (target cost, target fee, share ratio, ceiling price) for several actual costs.
@@lang python
@@starter
decisions = {
    "build in house": (120_000, [(0.6, 300_000), (0.4, 100_000)]),
    "buy package": (180_000, [(0.9, 280_000), (0.1, 150_000)]),
    "do nothing": (0, [(1.0, 0)]),
}
TARGET_COST, TARGET_FEE, BUYER_SHARE, CEILING = 100_000, 10_000, 0.7, 120_000
actual_costs = [90_000, 100_000, 110_000, 130_000]

def emv(outcomes):
    raise NotImplementedError
@@expected
build in house  EMV    220,000 - cost  120,000 = net    100,000
buy package     EMV    267,000 - cost  180,000 = net     87,000
do nothing      EMV          0 - cost        0 = net          0
Choose: build in house
FPIF actual 90,000: fee 13,000, buyer pays 103,000
FPIF actual 100,000: fee 10,000, buyer pays 110,000
FPIF actual 110,000: fee 7,000, buyer pays 117,000
FPIF actual 130,000: fee 1,000, buyer pays 120,000 (ceiling reached; seller absorbs the rest)
@@requires
def emv
def fpif_price
CEILING
@@hint
`abs(sum(p for p, _ in outcomes) - 1) < 1e-9` checks the probabilities. FPIF: fee = target fee + (target cost − actual) × seller share (1 − buyer share); price = actual + fee, capped at the ceiling.
@@solution
decisions = {
    "build in house": (120_000, [(0.6, 300_000), (0.4, 100_000)]),
    "buy package": (180_000, [(0.9, 280_000), (0.1, 150_000)]),
    "do nothing": (0, [(1.0, 0)]),
}
TARGET_COST, TARGET_FEE, BUYER_SHARE, CEILING = 100_000, 10_000, 0.7, 120_000
actual_costs = [90_000, 100_000, 110_000, 130_000]

def emv(outcomes):
    assert abs(sum(p for p, _ in outcomes) - 1) < 1e-9, "probabilities must sum to 1"
    return sum(p * v for p, v in outcomes)

results = {}
for name, (cost, outcomes) in decisions.items():
    results[name] = emv(outcomes) - cost
    print(f"{name:<15} EMV {emv(outcomes):>10,.0f} - cost {cost:>8,} = net {results[name]:>10,.0f}")
best = max(results, key=results.get)
print(f"Choose: {best}")

def fpif_price(actual):
    fee = TARGET_FEE + (TARGET_COST - actual) * (1 - BUYER_SHARE)
    return min(actual + fee, CEILING), fee

for actual in actual_costs:
    price, fee = fpif_price(actual)
    capped = " (ceiling reached; seller absorbs the rest)" if price == CEILING else ""
    print(f"FPIF actual {actual:,}: fee {fee:,.0f}, buyer pays {price:,.0f}{capped}")

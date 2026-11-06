@@lesson pm2-benefits-change Business Value and Organisational Change: Project Selection, Benefits Realisation and Transition
@@concepts numbers, functions, lists
@@lang python
@@text
The Business Environment domain is only 8% of the PMP exam, but it ties everything else to the reason projects exist: **delivering value to the organisation**. A project that finishes on time and on budget but whose benefits never appear has failed. This lesson covers how projects are chosen (the financial measures), how benefits are planned and tracked after the project ends, and how to manage the people side of change so the new system is actually used.

## Choosing projects: the financial measures

- **Payback period**: how long until cumulative cash inflows repay the investment. Simple and easy to explain, but it ignores the time value of money and anything after the payback point.
- **Net present value (NPV)**: the sum of each year's cash flow discounted to today, minus the investment: NPV = Σ CFₜ / (1 + r)ᵗ − I.
  - Positive NPV adds value, and the **higher NPV wins**.
  - On the exam, NPV already accounts for the project length, so don't divide by years.
- **Internal rate of return (IRR)**: the discount rate at which NPV is zero. Higher is better. Compare it with the organisation's hurdle rate.
- **Benefit-cost ratio (BCR)**: present value of benefits ÷ present value of costs. Above 1 is worthwhile.
- **Return on investment (ROI)**: (gain − cost) ÷ cost.
- **Opportunity cost**: the value of the project you **didn't** choose. If you pick A (NPV ¥5M) over B (NPV ¥3M), the opportunity cost is ¥3M.
- **Sunk costs** (money already spent) are irrelevant to the decision to continue: decide on future costs and benefits only.
- Non-financial criteria matter too: strategic alignment, compliance (mandatory projects aren't optional), risk, and capacity. Many organisations score projects with a **weighted scoring model**.

## Benefits management

- The **business case** justifies the project (problem, options, costs, benefits, risks), and the **benefits management plan** says how and when benefits will be measured and **who owns them**.
- Benefits usually appear **after** the project closes, so ownership often moves from the project manager to a business owner (the product owner or an operations manager).
- Benefit types: **tangible** (cost savings, revenue, fewer incidents) and **intangible** (satisfaction, brand, morale), but measure intangible ones with proxies such as surveys.
- Track planned vs actual benefits over time and report **benefit realisation** to the sponsor or the portfolio board. If benefits are drifting, act early: more training, process changes, or a decision to stop.
- **Value-driven delivery** in agile: deliver the highest-value features first (with an MVP to test assumptions) so benefits start early and the business can stop when enough value has been delivered.

## Organisational change management

People don't adopt new systems because a project ended. Models the exam mentions:
- **ADKAR** (Prosci): **A**wareness of the need, **D**esire to support it, **K**nowledge of how to change, **A**bility to do it, **R**einforcement to sustain it. Find the first missing element for a group and address that: training (knowledge) won't help people who don't see why the change is needed (awareness).
- **Kotter's 8 steps**: create urgency, build a guiding coalition, form a vision, communicate it, empower action and remove obstacles, create short-term wins, build on the change, anchor it in the culture.
- **Lewin**: unfreeze → change → refreeze.
- **Virginia Satir's change model**: late status quo → resistance → chaos → integration → new status quo, with performance dropping before it improves.
- **Bridges' transition model**: ending → neutral zone → new beginning. Change is external, while transition is the internal, psychological process.

## Transition and closing

- **Transition** hands the product to operations and users: training, documentation, support arrangements, an acceptance sign-off, and **knowledge transfer**.
- In IT this means the operations team accepts the service (an operational readiness review: monitoring, runbooks, backups, on-call). ITIL calls it a transition to live.
- **Compliance** is part of the business environment: regulatory requirements (privacy, accessibility, industry rules) must be identified early, assigned owners and checked at quality gates.
- Projects can also be **terminated early** when the business case no longer holds. That is a success of governance, not a failure of management. Close them formally: archive, release resources, capture lessons learned.

@@quiz
Project A has an NPV of ¥8M over 5 years, project B an NPV of ¥6M over 2 years. Which should be chosen on NPV alone?
* A, because NPV already accounts for time
- B, because it's shorter
- B, because ¥3M per year is higher
- Neither
? Don't divide NPV by years.

@@quiz
You choose project A (NPV ¥8M) over B (NPV ¥6M). What is the opportunity cost?
* ¥6M
- ¥8M
- ¥2M
- ¥14M
? The value of the option not chosen.

@@quiz
A project has spent ¥20M and needs ¥5M more to finish; its remaining benefits are worth ¥3M. What should guide the decision?
* Only future costs and benefits: ¥5M more for ¥3M is not worth it
- The ¥20M already spent
- The original business case only
- The team's preference
? Sunk costs are irrelevant.

@@quiz
Users completed training but still don't use the new system because they don't see why it was introduced. Which ADKAR element is missing?
* Awareness
- Knowledge
- Ability
- Reinforcement
? Address the first missing element.

@@quiz
Who usually owns benefits after the project closes?
* A business owner, such as the product or operations owner
- The project manager
- The PMO only
- The vendor
? Benefits often appear after closure.

@@quiz
What does a benefit-cost ratio of 0.8 mean?
* The present value of costs exceeds the benefits
- The project returns 80% profit
- The project is worthwhile
- The payback period is 0.8 years
? Above 1 is worthwhile.

@@quiz
Which change model describes performance dropping into chaos before integration?
* Satir's change model
- Lewin's model
- ADKAR
- Kotter's 8 steps
? Late status quo → resistance → chaos → integration → new status quo.

@@flashcards
payback period :: time to recover the investment
NPV :: Σ CF / (1 + r)^t − investment
IRR :: discount rate where NPV = 0
BCR :: PV benefits ÷ PV costs, > 1 is good
ROI :: (gain − cost) ÷ cost
opportunity cost :: value of the option not chosen
sunk cost :: already spent, ignore it
business case :: why the project is worth doing
benefits management plan :: how, when and who measures benefits
ADKAR :: awareness, desire, knowledge, ability, reinforcement
Kotter :: urgency, coalition, vision, communicate, empower, wins, build, anchor
Lewin :: unfreeze, change, refreeze
Satir :: status quo, resistance, chaos, integration, new status quo
Bridges :: ending, neutral zone, new beginning
transition :: hand over to operations and users
early termination :: governance working, close it formally

@@drill
An investment of ¥10M returns ¥4M a year. What is the payback period in years?
= 2.5
? 10 ÷ 4.

@@drill
A cash flow of ¥1,100,000 in one year, at a 10% discount rate, has what present value in yen? (number only)
= 1000000
= 1,000,000
? 1,100,000 ÷ 1.1.

@@exercise Compare projects with NPV, IRR, payback and BCR
For each candidate project (an initial investment and yearly net cash flows), compute NPV at the organisation's 8% rate, IRR by bisection, the payback period (with a fraction for the final year), and a simple BCR (PV of inflows ÷ investment). Rank the projects by NPV and mark the ones below the 12% hurdle rate.
@@starter
RATE, HURDLE = 0.08, 0.12
projects = {
    "Self-service portal": (12_000_000, [3_000_000, 4_000_000, 5_000_000, 5_000_000]),
    "Data centre exit": (30_000_000, [6_000_000, 9_000_000, 10_000_000, 10_000_000, 10_000_000]),
    "Chatbot pilot": (4_000_000, [2_500_000, 2_000_000]),
}

def npv(rate, investment, flows):
    raise NotImplementedError

def irr(investment, flows, lo=-0.99, hi=1.0):
    raise NotImplementedError

def payback(investment, flows):
    raise NotImplementedError

rows = []
for name, (inv, flows) in projects.items():
    rows.append((npv(RATE, inv, flows), name, irr(inv, flows), payback(inv, flows), (npv(RATE, 0, flows)) / inv))
for value, name, r, pb, bcr in sorted(rows, reverse=True):
    flag = "" if r >= HURDLE else "  below hurdle"
    print(f"{name:<20} NPV ¥{value:>12,.0f}  IRR {r:6.1%}  payback {pb:.2f} y  BCR {bcr:.2f}{flag}")
@@expected
Data centre exit     NPV ¥   5,366,058  IRR  14.1%  payback 3.50 y  BCR 1.18
Self-service portal  NPV ¥   1,851,444  IRR  14.2%  payback 3.00 y  BCR 1.15
Chatbot pilot        NPV ¥      29,492  IRR   8.6%  payback 1.75 y  BCR 1.01  below hurdle
@@requires
def npv
def irr
(1 + rate) **
@@hint
`npv = sum(cf / (1 + rate) ** t for t, cf in enumerate(flows, start=1)) - investment`. For `irr`, bisect: 60 times, take `mid = (lo + hi) / 2`; if `npv(mid, ...) > 0` the rate is too low, so set `lo = mid`, else `hi = mid`. For `payback`, accumulate the flows; in the year the total reaches the investment, add `(investment - before) / flow` as the fraction.
@@solution
RATE, HURDLE = 0.08, 0.12
projects = {
    "Self-service portal": (12_000_000, [3_000_000, 4_000_000, 5_000_000, 5_000_000]),
    "Data centre exit": (30_000_000, [6_000_000, 9_000_000, 10_000_000, 10_000_000, 10_000_000]),
    "Chatbot pilot": (4_000_000, [2_500_000, 2_000_000]),
}

def npv(rate, investment, flows):
    return sum(cf / (1 + rate) ** t for t, cf in enumerate(flows, start=1)) - investment

def irr(investment, flows, lo=-0.99, hi=1.0):
    for _ in range(60):
        mid = (lo + hi) / 2
        if npv(mid, investment, flows) > 0:
            lo = mid
        else:
            hi = mid
    return (lo + hi) / 2

def payback(investment, flows):
    total = 0
    for year, cf in enumerate(flows, start=1):
        if total + cf >= investment:
            return year - 1 + (investment - total) / cf
        total += cf
    return float("inf")

rows = []
for name, (inv, flows) in projects.items():
    rows.append((npv(RATE, inv, flows), name, irr(inv, flows), payback(inv, flows), (npv(RATE, 0, flows)) / inv))
for value, name, r, pb, bcr in sorted(rows, reverse=True):
    flag = "" if r >= HURDLE else "  below hurdle"
    print(f"{name:<20} NPV ¥{value:>12,.0f}  IRR {r:6.1%}  payback {pb:.2f} y  BCR {bcr:.2f}{flag}")

@@exercise Track benefits and find the ADKAR gap
Part 1: compare planned and actual benefits per quarter after go-live, show cumulative realisation, and flag benefits below 80% of plan. Part 2: for each user group, find the first ADKAR element scoring below 3 (out of 5) and recommend the matching action.
@@starter
benefits = {
    "Fewer password tickets": {"owner": "Service desk lead", "planned": [300, 400, 400, 400], "actual": [280, 390, 410, 420]},
    "Faster laptop delivery (days saved)": {"owner": "IT ops manager", "planned": [200, 300, 300, 300], "actual": [90, 150, 180, 200]},
    "Licence savings (¥k)": {"owner": "Finance BP", "planned": [500, 500, 500, 500], "actual": [0, 450, 600, 650]},
}
ACTIONS = {
    "awareness": "explain why the change is happening, from leaders they trust",
    "desire": "involve them, address what's in it for them and their concerns",
    "knowledge": "train them on how to use it",
    "ability": "practice time, coaching and floor-walkers",
    "reinforcement": "recognise adoption, measure it, remove the old way",
}
groups = {
    "Sales": {"awareness": 4, "desire": 2, "knowledge": 4, "ability": 3, "reinforcement": 2},
    "Finance": {"awareness": 2, "desire": 3, "knowledge": 4, "ability": 4, "reinforcement": 3},
    "Engineering": {"awareness": 5, "desire": 4, "knowledge": 4, "ability": 4, "reinforcement": 4},
    "Warehouse": {"awareness": 4, "desire": 4, "knowledge": 2, "ability": 2, "reinforcement": 3},
}

def realisation(planned, actual):
    raise NotImplementedError

def adkar_gap(scores):
    raise NotImplementedError

for name, b in benefits.items():
    pct = realisation(b["planned"], b["actual"])
    flag = "  BELOW 80%: tell the sponsor" if pct < 0.8 else ""
    print(f"{name:<38} {pct:5.0%} of plan (owner: {b['owner']}){flag}")
for group, scores in groups.items():
    gap = adkar_gap(scores)
    print(f"{group:<12} " + (f"{gap}: {ACTIONS[gap]}" if gap else "no gap: keep reinforcing"))
@@expected
Fewer password tickets                  100% of plan (owner: Service desk lead)
Faster laptop delivery (days saved)      56% of plan (owner: IT ops manager)  BELOW 80%: tell the sponsor
Licence savings (¥k)                     85% of plan (owner: Finance BP)
Sales        desire: involve them, address what's in it for them and their concerns
Finance      awareness: explain why the change is happening, from leaders they trust
Engineering  no gap: keep reinforcing
Warehouse    knowledge: train them on how to use it
@@requires
def realisation
def adkar_gap
sum(actual)
@@hint
Realisation is `sum(actual) / sum(planned)`. For ADKAR, loop over the elements in order (`["awareness", "desire", "knowledge", "ability", "reinforcement"]`) and return the first with a score below 3, or `None`.
@@solution
benefits = {
    "Fewer password tickets": {"owner": "Service desk lead", "planned": [300, 400, 400, 400], "actual": [280, 390, 410, 420]},
    "Faster laptop delivery (days saved)": {"owner": "IT ops manager", "planned": [200, 300, 300, 300], "actual": [90, 150, 180, 200]},
    "Licence savings (¥k)": {"owner": "Finance BP", "planned": [500, 500, 500, 500], "actual": [0, 450, 600, 650]},
}
ACTIONS = {
    "awareness": "explain why the change is happening, from leaders they trust",
    "desire": "involve them, address what's in it for them and their concerns",
    "knowledge": "train them on how to use it",
    "ability": "practice time, coaching and floor-walkers",
    "reinforcement": "recognise adoption, measure it, remove the old way",
}
groups = {
    "Sales": {"awareness": 4, "desire": 2, "knowledge": 4, "ability": 3, "reinforcement": 2},
    "Finance": {"awareness": 2, "desire": 3, "knowledge": 4, "ability": 4, "reinforcement": 3},
    "Engineering": {"awareness": 5, "desire": 4, "knowledge": 4, "ability": 4, "reinforcement": 4},
    "Warehouse": {"awareness": 4, "desire": 4, "knowledge": 2, "ability": 2, "reinforcement": 3},
}
ORDER = ["awareness", "desire", "knowledge", "ability", "reinforcement"]

def realisation(planned, actual):
    return sum(actual) / sum(planned)

def adkar_gap(scores):
    for element in ORDER:
        if scores[element] < 3:
            return element
    return None

for name, b in benefits.items():
    pct = realisation(b["planned"], b["actual"])
    flag = "  BELOW 80%: tell the sponsor" if pct < 0.8 else ""
    print(f"{name:<38} {pct:5.0%} of plan (owner: {b['owner']}){flag}")
for group, scores in groups.items():
    gap = adkar_gap(scores)
    print(f"{group:<12} " + (f"{gap}: {ACTIONS[gap]}" if gap else "no gap: keep reinforcing"))

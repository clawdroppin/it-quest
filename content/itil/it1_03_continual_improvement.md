@@lesson it1-continual-improvement Continual Improvement in Practice: The Model, the Register, Baselines and Measurement
@@concepts lists, numbers, functions
@@lang python
@@text
Continual improvement appears in ITIL 4 twice: as a **component of the service value system** (it applies to everything) and as a **practice** (the way an organisation actually runs improvement work). It is also one of the most-tested topics in the Foundation exam. In real jobs it is what separates a service desk that gets steadily better from one that fights the same fires every month.

## The continual improvement model

The model has seven steps, each a question:

1. **What is the vision?** Link the improvement to the organisation's goals and strategy, e.g. "customers can resolve common problems themselves at any hour."
2. **Where are we now?** Take a **baseline**: measure the current state objectively (metrics, surveys, process assessments, maturity models). Without a baseline you can't prove improvement.
3. **Where do we want to be?** Set measurable targets: KPIs, critical success factors (CSFs), and objectives and key results (OKRs).
4. **How do we get there?** Plan the improvement, often as a series of small iterations rather than one big project.
5. **Take action.** Do the work, using whatever method fits (Agile, Lean, waterfall).
6. **Did we get there?** Measure again and compare with the baseline and the target.
7. **How do we keep the momentum going?** Share the success, embed the change, and pick the next improvement.

The model works at every scale: a single team's ticket form or the whole organisation's operating model.

## The continual improvement register (CIR)

- A **CIR** records improvement ideas with a description, the expected value, effort, priority, owner and status.
- There can be several (per team, per practice, one for the organisation), and the ideas can come from anyone: incident reviews, problem management, customer feedback, audits, staff suggestions.
- **Prioritise** by value against effort or cost. A classic approach is to score value and effort and sort by the ratio. Agile teams use **WSJF** (weighted shortest job first): cost of delay divided by job size.
- Review the register regularly. Old ideas that never get prioritised should be closed, not left to rot.

## Methods ITIL works with

- **PDCA** (Plan, Do, Check, Act), the Deming cycle: the root of most improvement methods.
- **Lean**: remove waste (waiting, rework, handoffs, unnecessary approvals), and map the value stream.
- **Agile**: short iterations, feedback, and working results early.
- **DevOps**: automation and shared responsibility between development and operations.
- **Kaizen**: many small, continuous improvements by the people doing the work.
- **SWOT** analysis and **balanced scorecard** help with "where are we now?" at the strategic level.

## Measuring improvement properly

- Measure the **outcome**, not just activity: "median time to restore service fell from 6 h to 3.5 h" means more than "we closed 40 improvement tickets".
- Compare against the **baseline** and the **target**: progress = (current − baseline) ÷ (target − baseline). 100% means the target is reached, and a negative value means things got worse.
- Watch for **side effects**: faster incident closure can come with more reopened tickets. Pair metrics (speed with quality).
- Remember the **watermelon effect**: green metrics but unhappy users. Include experience measures such as satisfaction surveys alongside the operational ones.

## Responsibilities

- Everyone in the organisation is responsible for continual improvement, but someone must own the practice: maintaining registers, coaching teams, and reporting results to leadership.
- Leaders must give time and budget. "Improve in your spare time" never works.

@@quiz
Which step of the continual improvement model establishes a baseline?
- What is the vision?
* Where are we now?
- Where do we want to be?
- Did we get there?
? You can't prove improvement without one.

@@quiz
What is a continual improvement register used for?
* Recording, prioritising and tracking improvement ideas
- Storing configuration items
- Logging incidents
- Recording changes for approval
? There can be several registers.

@@quiz
Which step comes right after "Take action"?
- How do we get there?
* Did we get there?
- Where are we now?
- What is the vision?
? Measure against the baseline and target.

@@quiz
Who is responsible for continual improvement?
* Everyone, though the practice has an owner
- Only the service desk
- Only the CIO
- Only external consultants
? Ideas come from everywhere.

@@quiz
A team closes incidents faster, but reopened tickets doubled. What does this show?
* Metrics should be paired so that improving one doesn't hide damage elsewhere
- The improvement succeeded
- Reopened tickets don't matter
- The baseline was wrong
? Speed with quality.

@@quiz
WSJF prioritises work by:
* Cost of delay divided by job size
- Oldest first
- The loudest stakeholder
- Effort alone
? Weighted shortest job first.

@@flashcards
CI model step 1 :: What is the vision?
CI model step 2 :: Where are we now? (baseline)
CI model step 3 :: Where do we want to be? (targets)
CI model step 4 :: How do we get there? (plan)
CI model step 5 :: Take action
CI model step 6 :: Did we get there? (measure)
CI model step 7 :: How do we keep the momentum going?
CIR :: continual improvement register
baseline :: measured starting point
CSF :: critical success factor
OKR :: objectives and key results
WSJF :: cost of delay ÷ job size
PDCA :: plan, do, check, act
Kaizen :: many small continuous improvements
progress :: (current − baseline) ÷ (target − baseline)

@@drill
How many steps does the ITIL continual improvement model have?
= 7
= seven
? From vision to momentum.

@@drill
The baseline is 6 hours, the target 3 hours and the current value 4.5 hours. What percentage of the way to the target is that? (number only)
= 50
? (4.5 − 6) ÷ (3 − 6) = 0.5.

@@exercise Prioritise the improvement register
Score each idea in the register by value ÷ effort, break ties by the lower effort, and plan the top ideas that fit in the team's capacity for the quarter (total effort points). Ideas that don't fit stay in the register; ideas older than 180 days with a score below 1 are proposed for closure.
@@starter
from datetime import date

today = date(2026, 7, 1)
CAPACITY = 16
register = [
    {"id": "CI-01", "idea": "Self-service password reset", "value": 13, "effort": 5, "raised": date(2026, 5, 2)},
    {"id": "CI-02", "idea": "Merge duplicate request forms", "value": 5, "effort": 2, "raised": date(2026, 6, 1)},
    {"id": "CI-03", "idea": "New ITSM platform", "value": 20, "effort": 20, "raised": date(2025, 11, 3)},
    {"id": "CI-04", "idea": "Knowledge article for VPN errors", "value": 8, "effort": 1, "raised": date(2026, 6, 20)},
    {"id": "CI-05", "idea": "Colour-coded queues", "value": 2, "effort": 3, "raised": date(2025, 10, 9)},
    {"id": "CI-06", "idea": "Automated laptop provisioning", "value": 13, "effort": 8, "raised": date(2026, 3, 15)},
    {"id": "CI-07", "idea": "Weekly problem review meeting", "value": 5, "effort": 5, "raised": date(2026, 6, 10)},
]

def score(item):
    raise NotImplementedError

def plan(register, capacity):
    raise NotImplementedError

planned, waiting, close = plan(register, CAPACITY)
print("planned:")
for i in planned:
    print(f"  {i['id']} {i['idea']:<34} score {score(i):.2f} effort {i['effort']}")
print(f"  total effort {sum(i['effort'] for i in planned)} of {CAPACITY}")
print("waiting:", ", ".join(i["id"] for i in waiting))
print("propose closing:", ", ".join(i["id"] for i in close))
@@expected
planned:
  CI-04 Knowledge article for VPN errors   score 8.00 effort 1
  CI-01 Self-service password reset        score 2.60 effort 5
  CI-02 Merge duplicate request forms      score 2.50 effort 2
  CI-06 Automated laptop provisioning      score 1.62 effort 8
  total effort 16 of 16
waiting: CI-07, CI-03, CI-05
propose closing: CI-05
@@requires
def score
def plan
sorted(
@@hint
`score` is `value / effort`. Sort by `(-score(i), i["effort"])`. Walk the sorted list, adding an idea to `planned` if it fits in the remaining capacity (keep looking at later, smaller ideas when one doesn't fit). Everything else is `waiting`, and the closure list is the waiting ideas with `(today - raised).days > 180` and a score below 1.
@@solution
from datetime import date

today = date(2026, 7, 1)
CAPACITY = 16
register = [
    {"id": "CI-01", "idea": "Self-service password reset", "value": 13, "effort": 5, "raised": date(2026, 5, 2)},
    {"id": "CI-02", "idea": "Merge duplicate request forms", "value": 5, "effort": 2, "raised": date(2026, 6, 1)},
    {"id": "CI-03", "idea": "New ITSM platform", "value": 20, "effort": 20, "raised": date(2025, 11, 3)},
    {"id": "CI-04", "idea": "Knowledge article for VPN errors", "value": 8, "effort": 1, "raised": date(2026, 6, 20)},
    {"id": "CI-05", "idea": "Colour-coded queues", "value": 2, "effort": 3, "raised": date(2025, 10, 9)},
    {"id": "CI-06", "idea": "Automated laptop provisioning", "value": 13, "effort": 8, "raised": date(2026, 3, 15)},
    {"id": "CI-07", "idea": "Weekly problem review meeting", "value": 5, "effort": 5, "raised": date(2026, 6, 10)},
]

def score(item):
    return item["value"] / item["effort"]

def plan(register, capacity):
    planned, waiting, left = [], [], capacity
    for item in sorted(register, key=lambda i: (-score(i), i["effort"])):
        if item["effort"] <= left:
            planned.append(item)
            left -= item["effort"]
        else:
            waiting.append(item)
    close = [i for i in waiting if (today - i["raised"]).days > 180 and score(i) < 1]
    return planned, waiting, close

planned, waiting, close = plan(register, CAPACITY)
print("planned:")
for i in planned:
    print(f"  {i['id']} {i['idea']:<34} score {score(i):.2f} effort {i['effort']}")
print(f"  total effort {sum(i['effort'] for i in planned)} of {CAPACITY}")
print("waiting:", ", ".join(i["id"] for i in waiting))
print("propose closing:", ", ".join(i["id"] for i in close))

@@exercise Did we get there?
For each improvement, compare the current measurement with the baseline and the target: compute progress as a percentage, and report "target met", "improving", "no change" or "worse". Then check the paired quality metric: flag the improvement if the quality metric got worse by more than 10%.
@@starter
improvements = [
    {"name": "Restore time (h)", "baseline": 6.0, "target": 3.0, "current": 3.5, "quality": ("reopen rate %", 4.0, 4.2)},
    {"name": "First contact resolution %", "baseline": 52, "target": 70, "current": 73, "quality": ("CSAT", 4.1, 4.3)},
    {"name": "Change success rate %", "baseline": 91, "target": 97, "current": 90, "quality": ("emergency changes", 12, 11)},
    {"name": "Time to fulfil laptop (days)", "baseline": 10, "target": 3, "current": 4, "quality": ("returns for faults", 5, 9)},
    {"name": "Self-service share %", "baseline": 20, "target": 40, "current": 20, "quality": ("CSAT", 4.0, 4.0)},
]

def progress(baseline, target, current):
    raise NotImplementedError

def status(p):
    raise NotImplementedError

def quality_flag(name, before, after, lower_is_better=True):
    raise NotImplementedError

LOWER_IS_BETTER = {"reopen rate %": True, "CSAT": False, "emergency changes": True, "returns for faults": True}
for imp in improvements:
    p = progress(imp["baseline"], imp["target"], imp["current"])
    qname, qb, qa = imp["quality"]
    flag = quality_flag(qname, qb, qa, LOWER_IS_BETTER[qname])
    print(f"{imp['name']:<29} {p:>5.0f}%  {status(p):<10} {flag}".rstrip())
@@expected
Restore time (h)                 83%  improving
First contact resolution %      117%  target met
Change success rate %           -17%  worse
Time to fulfil laptop (days)     86%  improving  check returns for faults: 5 -> 9
Self-service share %              0%  no change
@@requires
def progress
def quality_flag
(target - baseline)
@@hint
`progress = (current - baseline) / (target - baseline) * 100`; this works whether lower or higher is better. Status: `>= 100` "target met", `> 0` "improving", `== 0` "no change", otherwise "worse". For the quality flag, compute the relative change `(after - before) / before`; it got worse if it rose by more than 10% when lower is better, or fell by more than 10% when higher is better. Return a message like `f"check {name}: {before} -> {after}"` or an empty string.
@@solution
improvements = [
    {"name": "Restore time (h)", "baseline": 6.0, "target": 3.0, "current": 3.5, "quality": ("reopen rate %", 4.0, 4.2)},
    {"name": "First contact resolution %", "baseline": 52, "target": 70, "current": 73, "quality": ("CSAT", 4.1, 4.3)},
    {"name": "Change success rate %", "baseline": 91, "target": 97, "current": 90, "quality": ("emergency changes", 12, 11)},
    {"name": "Time to fulfil laptop (days)", "baseline": 10, "target": 3, "current": 4, "quality": ("returns for faults", 5, 9)},
    {"name": "Self-service share %", "baseline": 20, "target": 40, "current": 20, "quality": ("CSAT", 4.0, 4.0)},
]

def progress(baseline, target, current):
    return (current - baseline) / (target - baseline) * 100

def status(p):
    if p >= 100:
        return "target met"
    if p > 0:
        return "improving"
    if p == 0:
        return "no change"
    return "worse"

def quality_flag(name, before, after, lower_is_better=True):
    change = (after - before) / before
    worse = change > 0.10 if lower_is_better else change < -0.10
    return f"check {name}: {before} -> {after}" if worse else ""

LOWER_IS_BETTER = {"reopen rate %": True, "CSAT": False, "emergency changes": True, "returns for faults": True}
for imp in improvements:
    p = progress(imp["baseline"], imp["target"], imp["current"])
    qname, qb, qa = imp["quality"]
    flag = quality_flag(qname, qb, qa, LOWER_IS_BETTER[qname])
    print(f"{imp['name']:<29} {p:>5.0f}%  {status(p):<10} {flag}".rstrip())

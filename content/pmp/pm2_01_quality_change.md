@@lesson pm2-quality-change Quality, Integrated Change Control and Closing
@@text
Quality, change control and closing are where predictive projects are won or lost, and they show up across the PMP exam's Process domain (with hybrid twists: a backlog absorbs change differently from a change control board). This lesson covers the quality tools you're expected to read (control charts, Pareto, cause-and-effect, cost of quality), how integrated change control works step by step, and what closing a project or phase involves.

## Quality concepts

- **Quality** = conformance to requirements and fitness for use. **Grade** = a category of features (a basic phone can be high quality and low grade).
- **Prevention over inspection**: it's cheaper to build quality in than to find defects.
- **Cost of quality (CoQ)**: *cost of conformance* (prevention: training, standards, design reviews; appraisal: testing, inspections) vs *cost of nonconformance* (internal failures: rework, scrap; external failures: warranty, recalls, lost reputation). Spending on prevention reduces failure costs most.
- **Manage Quality** (quality assurance: are we following the process? audits) vs **Control Quality** (quality control: does the deliverable meet requirements? inspection, testing). Validated deliverables then go to **Validate Scope** for customer acceptance.
- In agile, quality is built in through the **Definition of Done**, automated tests, reviews and retrospectives.

## The seven basic quality tools

| Tool | Answers |
|---|---|
| **Cause-and-effect** (Ishikawa, fishbone) | what could be causing this problem? (5 Whys for depth) |
| **Flowchart** | where in the process does it happen? |
| **Check sheet** | how often does each defect occur? (tally) |
| **Pareto chart** | which few causes produce most defects? (80/20) |
| **Histogram** | what's the distribution? |
| **Control chart** | is the process stable and predictable? |
| **Scatter diagram** | are two variables related? |

### Control charts

A centre line (mean) and **control limits** at ±3σ: the voice of the process. **Specification limits** come from the customer and are separate. The process is **out of control** when a point falls outside the control limits, or with the **rule of seven**: seven consecutive points on one side of the mean (a non-random trend), even if within limits. Out of control → investigate an assignable (special) cause; within limits → common-cause variation, don't tamper.

## Integrated change control

1. Someone raises a **change request** (corrective action, preventive action, defect repair or update).
2. The PM **analyses the impact** on scope, schedule, cost, quality, risk and resources.
3. The **Change Control Board** (or the sponsor, per the change management plan) approves or rejects.
4. Approved changes update the **baselines** and plans, are communicated, and implemented; everything is logged in the **change log**.
Exam answers: never just make the change, even if a senior stakeholder asks; evaluate the impact first, follow the process. Gold plating (adding extras nobody asked for) is also a no. In agile, changes go to the product owner, who reprioritises the backlog.

## Closing

Confirm acceptance of deliverables (formal sign-off), close procurements (contract closure, final payments, audits), transfer the product to operations, release resources, archive documents, write the **final report**, update **lessons learned** (organizational process assets) and celebrate. A project terminated early still needs closing.

@@quiz
Seven consecutive measurements fall just above the mean, all within control limits. What does this indicate?
- The process is in control
* The process may be out of control (rule of seven)
- The specification limits are wrong
- Nothing; it's random
? A non-random run.

@@quiz
Which spending reduces the total cost of quality most?
* Prevention
- Appraisal
- Internal failure
- External failure
? Build quality in.

@@quiz
A senior stakeholder asks you to add a feature directly. What do you do first?
- Add it; they're senior
- Refuse
* Raise or document a change request and analyse its impact
- Ask the team to work overtime
? Follow integrated change control.

@@quiz
Which tool identifies the few causes behind most defects?
- Scatter diagram
* Pareto chart
- Flowchart
- Control chart
? 80/20.

@@quiz
What's the difference between Control Quality and Validate Scope?
- They're the same
* Control Quality checks correctness internally; Validate Scope is the customer's formal acceptance
- Validate Scope happens first
- Control Quality is only for agile
? Correctness, then acceptance.

@@quiz
A project is cancelled halfway. What should the PM do?
- Stop all work and leave
* Perform closing: document status, lessons learned, release resources, close contracts
- Restart it
- Delete the documents
? Terminated projects still close.

@@flashcards
quality vs grade :: conformance vs feature category
prevention over inspection :: build quality in
cost of conformance :: prevention + appraisal
cost of nonconformance :: internal + external failures
Manage Quality :: process audits (QA)
Control Quality :: inspect deliverables (QC)
Validate Scope :: formal customer acceptance
Ishikawa :: cause-and-effect diagram
Pareto :: few causes, most defects
control limits :: ±3σ, voice of the process
specification limits :: voice of the customer
rule of seven :: seven points on one side
assignable cause :: special cause to investigate
change control board :: approves or rejects
change log :: record of all changes
gold plating :: unrequested extras
closing :: acceptance, contracts, lessons learned

@@drill
How many consecutive points on one side of the mean signal a non-random process?
= 7
= seven
? The rule of seven.

@@drill
Which group approves or rejects change requests in a predictive project?
= change control board
= CCB
= the change control board
? Per the change management plan.

@@exercise Read a control chart
Compute the mean and the ±3σ control limits from a baseline period, then check new measurements: flag points outside the limits and runs of seven or more consecutive points on the same side of the mean. Also compare with the customer's specification limits.
@@lang python
@@starter
from statistics import mean, pstdev

baseline = [10.1, 9.9, 10.0, 10.2, 9.8, 10.0, 10.1, 9.9, 10.0, 10.1, 9.9, 10.0]
new = [10.0, 10.1, 9.9, 10.15, 10.1, 10.05, 10.2, 10.1, 10.12, 10.08, 9.7, 10.65, 10.0]
SPEC = (9.5, 10.5)

def limits(data):
    raise NotImplementedError
@@expected
centre 10.000, control limits 9.676 to 10.324, spec 9.5 to 10.5
  # 1 10.00 ok
  # 2 10.10 ok
  # 3  9.90 ok
  # 4 10.15 ok
  # 5 10.10 ok
  # 6 10.05 ok
  # 7 10.20 ok
  # 8 10.10 ok
  # 9 10.12 ok
  #10 10.08 rule of seven: points 4-10 all above the mean
  #11  9.70 ok
  #12 10.65 OUTSIDE control limits: investigate an assignable cause; out of spec: defect
  #13 10.00 ok
@@requires
def limits
pstdev(
7
@@hint
`limits` returns `(lcl, centre, ucl)` with `centre ± 3 * pstdev(data)`. Track the current run: +1 above the mean, −1 below (a point exactly on the mean breaks the run). Report a run when it reaches 7 and say where it started.
@@solution
from statistics import mean, pstdev

baseline = [10.1, 9.9, 10.0, 10.2, 9.8, 10.0, 10.1, 9.9, 10.0, 10.1, 9.9, 10.0]
new = [10.0, 10.1, 9.9, 10.15, 10.1, 10.05, 10.2, 10.1, 10.12, 10.08, 9.7, 10.65, 10.0]
SPEC = (9.5, 10.5)

def limits(data):
    centre = mean(data)
    sigma = pstdev(data)
    return centre - 3 * sigma, centre, centre + 3 * sigma

lcl, centre, ucl = limits(baseline)
print(f"centre {centre:.3f}, control limits {lcl:.3f} to {ucl:.3f}, spec {SPEC[0]} to {SPEC[1]}")
side, run, start = 0, 0, 0
for i, x in enumerate(new, 1):
    notes = []
    if not lcl <= x <= ucl:
        notes.append("OUTSIDE control limits: investigate an assignable cause")
    if not SPEC[0] <= x <= SPEC[1]:
        notes.append("out of spec: defect")
    s = (x > centre) - (x < centre)
    if s != 0 and s == side:
        run += 1
    else:
        side, run, start = s, (1 if s else 0), i
    if run == 7:
        notes.append(f"rule of seven: points {start}-{i} all {'above' if side > 0 else 'below'} the mean")
    print(f"  #{i:>2} {x:5.2f} {'; '.join(notes) or 'ok'}")

@@exercise Pareto analysis and a change request
First, sort defect categories by count, compute the cumulative percentage, and list the "vital few" categories that together reach 80%. Then evaluate change requests: estimate each one's schedule and cost impact, and route it by the change management plan's thresholds (PM can approve small ones; the CCB approves the rest; anything touching scope goes to the CCB).
@@lang python
@@starter
defects = {"login timeout": 12, "wrong totals": 48, "typos": 7, "slow report": 21, "export fails": 33, "layout glitch": 9}
requests = [
    {"id": "CR-01", "what": "fix export encoding", "days": 2, "cost": 1200, "scope": False},
    {"id": "CR-02", "what": "add SSO login", "days": 15, "cost": 18000, "scope": True},
    {"id": "CR-03", "what": "move server room date", "days": 6, "cost": 4000, "scope": False},
]
PM_LIMIT = {"days": 3, "cost": 5000}

def pareto(counts):
    raise NotImplementedError
@@expected
wrong totals    48  cumulative  36.9%
export fails    33  cumulative  62.3%
slow report     21  cumulative  78.5%
login timeout   12  cumulative  87.7%
layout glitch    9  cumulative  94.6%
typos            7  cumulative 100.0%
vital few: wrong totals, export fails, slow report, login timeout
CR-01 fix export encoding: +2 days, +1200 cost -> PM (log it in the change log)
CR-02 add SSO login: +15 days, +18000 cost -> CCB (update baselines if approved)
CR-03 move server room date: +6 days, +4000 cost -> CCB (update baselines if approved)
@@requires
def pareto
80
def route
@@hint
`pareto` returns a list of `(category, count, cumulative_pct)` sorted by count descending. The vital few are the categories up to and including the first one whose cumulative percentage reaches 80. `route` returns "CCB" if `scope` is true or either limit is exceeded, otherwise "PM".
@@solution
defects = {"login timeout": 12, "wrong totals": 48, "typos": 7, "slow report": 21, "export fails": 33, "layout glitch": 9}
requests = [
    {"id": "CR-01", "what": "fix export encoding", "days": 2, "cost": 1200, "scope": False},
    {"id": "CR-02", "what": "add SSO login", "days": 15, "cost": 18000, "scope": True},
    {"id": "CR-03", "what": "move server room date", "days": 6, "cost": 4000, "scope": False},
]
PM_LIMIT = {"days": 3, "cost": 5000}

def pareto(counts):
    total = sum(counts.values())
    rows, running = [], 0
    for name, n in sorted(counts.items(), key=lambda kv: -kv[1]):
        running += n
        rows.append((name, n, running / total * 100))
    return rows

def route(cr):
    if cr["scope"] or cr["days"] > PM_LIMIT["days"] or cr["cost"] > PM_LIMIT["cost"]:
        return "CCB"
    return "PM"

vital = []
for name, n, cum in pareto(defects):
    if not vital or vital[-1][2] < 80:
        vital.append((name, n, cum))
    print(f"{name:<14} {n:>3}  cumulative {cum:5.1f}%")
print("vital few:", ", ".join(v[0] for v in vital))
for cr in requests:
    who = route(cr)
    print(f"{cr['id']} {cr['what']}: +{cr['days']} days, +{cr['cost']} cost -> {who}"
          + (" (update baselines if approved)" if who == "CCB" else " (log it in the change log)"))

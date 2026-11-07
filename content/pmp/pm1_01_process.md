@@lesson pm1-process Scope, Schedule, Cost, Risk and Quality
@@text
## Scope

- **Project charter**: authorizes the project and names the project manager. Signed by the **sponsor**.
- **Requirements** → **scope statement** → **WBS** (work breakdown structure): the deliverables broken down into **work packages**. The WBS plus its dictionary and the scope statement make the **scope baseline**.
- **Scope creep**: changes added without control. **Gold plating**: the team adds extras nobody asked for. Both are wrong.
- **Validate scope** (the customer accepts deliverables) versus **control quality** (the team checks they're correct, done first).
- In agile, scope lives in the **product backlog**, ordered by the **product owner** by value. The **definition of done** says when an item is complete.

## Schedule

- **Activities** with durations and **dependencies**: finish-to-start (the usual), start-to-start, finish-to-finish, start-to-finish. **Lead** = overlap, **lag** = wait.
- **Critical path method**: the longest path through the network is the shortest possible project duration. Activities on it have zero **float** (slack). A **forward pass** gives early start/finish, a **backward pass** late start/finish; float = LS − ES. (You'll compute this in the project.)
- **Estimating**: analogous (from past projects, quick, rough), parametric (rate × quantity), **three-point**: PERT **(O + 4M + P) / 6**, triangular (O + M + P) / 3.
- **Compression**: **crashing** adds resources to critical activities (costs more); **fast tracking** runs activities in parallel (more risk).
- Agile: **velocity** (story points per iteration), **release burndown/burnup charts**, timeboxed **sprints** (usually 1–4 weeks).

## Cost and earned value

**Earned value management** compares the plan with reality in money:

| Term | Meaning | Formula |
|---|---|---|
| **PV** | planned value: work that should be done by now | |
| **EV** | earned value: work actually done, at its planned cost | % complete × BAC |
| **AC** | actual cost of that work | |
| **CV** / **SV** | cost / schedule variance | EV − AC / EV − PV |
| **CPI** / **SPI** | cost / schedule performance index | EV / AC, EV / PV |
| **EAC** | estimate at completion | BAC / CPI (if the current efficiency continues) |
| **ETC** | estimate to complete | EAC − AC |
| **VAC** | variance at completion | BAC − EAC |
| **TCPI** | efficiency needed to finish on budget | (BAC − EV) / (BAC − AC) |

Below 1 (or negative variances) is bad: CPI 0.8 means every ¥1 spent produced ¥0.80 of work.

Example: BAC ¥10,000,000; at month 4, PV ¥4,000,000, EV ¥3,000,000, AC ¥3,750,000. CPI = 0.8, SPI = 0.75: over budget *and* behind. EAC = 10,000,000 / 0.8 = ¥12,500,000.

## Risk

- **Risk register**: each risk with probability, impact, owner and response. **Qualitative** analysis (probability × impact matrix), then **quantitative** for the big ones (**expected monetary value** = probability × impact, Monte Carlo, decision trees).
- **Threat responses**: escalate, avoid, transfer, mitigate, accept. **Opportunity responses**: escalate, exploit, share, enhance, accept.
- **Contingency reserves** (for known risks, "known unknowns", controlled by the PM) versus **management reserves** (for unknown unknowns, controlled by management). Cost baseline + management reserve = **project budget**.
- A **risk** is uncertain and in the future; an **issue** has happened (issue log).

## Quality and change

- **Cost of quality**: prevention + appraisal (cost of conformance) versus internal + external failure (cost of nonconformance). Prevention is cheapest.
- Tools: Pareto charts (80/20), fishbone (Ishikawa: very Japanese, from Kaoru Ishikawa), control charts (rule of seven), checklists. **Kaizen** (continuous improvement) and **PDCA** come from Japanese quality practice too.
- **Integrated change control** (predictive): change request → impact analysis → **change control board** decides → update baselines → communicate. The PM never approves a scope change alone, and never lets the team make it informally.

## Agile frameworks

- **Scrum**: roles (product owner, Scrum Master, developers), events (sprint, sprint planning, daily scrum, sprint review, retrospective), artifacts (product backlog, sprint backlog, increment).
- **Kanban**: visualize work, **limit work in progress**, manage flow (cycle time, lead time).
- Scaling: SAFe, LeSS. XP practices: pair programming, TDD, continuous integration.

@@quiz
A project has BAC $200,000, EV $80,000, AC $100,000 and PV $90,000. What's the CPI?
* 0.8
- 0.89
- 1.25
- 0.9
? CPI = EV / AC = 80,000 / 100,000.

@@quiz
Using that CPI, what's the EAC if the current performance continues?
* $250,000
- $220,000
- $200,000
- $180,000
? EAC = BAC / CPI = 200,000 / 0.8.

@@quiz
The schedule must be shortened without adding cost. Which technique fits?
* Fast tracking
- Crashing
- Adding management reserve
- Reducing quality
? Fast tracking overlaps activities, adding risk but not cost.

@@quiz
A stakeholder asks a developer directly for an extra feature, and the developer adds it. What is this?
* Scope creep, which should go through change control or the product owner
- Gold plating
- Fast tracking
- Good customer service
? Gold plating would be the team adding features nobody asked for.

@@quiz
A PERT estimate has O = 4, M = 6, P = 14 days. What's the expected duration?
* 7 days
- 8 days
- 6 days
- 24 days
? (4 + 4 × 6 + 14) / 6 = 42 / 6 = 7.

@@quiz
A risk you identified has now occurred. Where is it recorded?
* In the issue log, and the planned response is carried out
- Nowhere: it's too late
- Only in the lessons learned
- In the WBS
? Risks become issues when they happen.

@@flashcards
project charter :: authorizes the project, signed by the sponsor
scope baseline :: scope statement + WBS + WBS dictionary
critical path :: the longest path; zero float
float :: LS − ES
PERT :: (O + 4M + P) / 6
crashing / fast tracking :: add resources / overlap activities
CPI / SPI :: EV / AC, EV / PV
EAC (typical) :: BAC / CPI
TCPI :: (BAC − EV) / (BAC − AC)
contingency vs management reserve :: known risks (PM) / unknown risks (management)
EMV :: probability × impact
cost of conformance :: prevention + appraisal
Scrum events :: sprint, planning, daily scrum, review, retrospective
Kanban :: visualize work and limit WIP

@@drill
BAC is ¥1,000,000, EV ¥400,000 and AC ¥500,000. What is the VAC in yen, if the current CPI continues? (a number, negative allowed)
= -250000
= -250,000
? CPI = 0.8; EAC = 1,250,000; VAC = BAC − EAC.

@@exercise Pick a risk response from the P-I matrix
Qualitative risk analysis scores each risk as probability × impact (both 1 to 5). Write `response(risk)` that suggests a response from the lesson's lists:

- outside the project manager's authority (`outside_scope`) → `escalate`, threat or opportunity;
- **threats**: score 15+ → `avoid` if the activity can be dropped (`optional`), else `transfer` if it can be insured or contracted (`transferable`), else `mitigate`; score 6–14 → `mitigate`; below 6 → `accept`;
- **opportunities**: 15+ → `exploit`; 6–14 → `enhance`, or `share` if a partner is needed (`needs_partner`); below 6 → `accept`.

Print the register sorted by score (highest first) with each response, and the risks that need an owner.
@@lang python
@@starter
risks = [
    {"id": "R1", "desc": "Key engineer leaves mid-project", "kind": "threat", "p": 3, "i": 5, "transferable": False, "owner": "PM"},
    {"id": "R2", "desc": "Vendor delivers servers late", "kind": "threat", "p": 4, "i": 4, "transferable": True, "owner": ""},
    {"id": "R3", "desc": "Optional AI feature may miss the legal review", "kind": "threat", "p": 4, "i": 5, "optional": True, "owner": "Product owner"},
    {"id": "R4", "desc": "Office move disrupts one sprint", "kind": "threat", "p": 2, "i": 2, "owner": "PM"},
    {"id": "R5", "desc": "Early cloud credits could fund extra testing", "kind": "opportunity", "p": 4, "i": 4, "owner": ""},
    {"id": "R6", "desc": "Joint launch with a partner doubles reach", "kind": "opportunity", "p": 2, "i": 4, "needs_partner": True, "owner": "Sales"},
    {"id": "R7", "desc": "Company-wide budget freeze", "kind": "threat", "p": 3, "i": 5, "outside_scope": True, "owner": ""},
]

def response(risk):
    raise NotImplementedError


for r in sorted(risks, key=lambda r: -r["p"] * r["i"]):
    print(f"{r['id']} {r['kind']:<12} {r['p'] * r['i']:>2}  {response(r):<9} {r['desc']}")
print("Needs an owner: " + ", ".join(r["id"] for r in risks if not r["owner"]))
@@expected
R3 threat       20  avoid     Optional AI feature may miss the legal review
R2 threat       16  transfer  Vendor delivers servers late
R5 opportunity  16  exploit   Early cloud credits could fund extra testing
R1 threat       15  mitigate  Key engineer leaves mid-project
R7 threat       15  escalate  Company-wide budget freeze
R6 opportunity   8  share     Joint launch with a partner doubles reach
R4 threat        4  accept    Office move disrupts one sprint
Needs an owner: R2, R5, R7
@@requires
def response
"escalate"
@@hint
Compute the score once, handle escalation first, then branch on threat or opportunity and on the score bands.
@@solution
risks = [
    {"id": "R1", "desc": "Key engineer leaves mid-project", "kind": "threat", "p": 3, "i": 5, "transferable": False, "owner": "PM"},
    {"id": "R2", "desc": "Vendor delivers servers late", "kind": "threat", "p": 4, "i": 4, "transferable": True, "owner": ""},
    {"id": "R3", "desc": "Optional AI feature may miss the legal review", "kind": "threat", "p": 4, "i": 5, "optional": True, "owner": "Product owner"},
    {"id": "R4", "desc": "Office move disrupts one sprint", "kind": "threat", "p": 2, "i": 2, "owner": "PM"},
    {"id": "R5", "desc": "Early cloud credits could fund extra testing", "kind": "opportunity", "p": 4, "i": 4, "owner": ""},
    {"id": "R6", "desc": "Joint launch with a partner doubles reach", "kind": "opportunity", "p": 2, "i": 4, "needs_partner": True, "owner": "Sales"},
    {"id": "R7", "desc": "Company-wide budget freeze", "kind": "threat", "p": 3, "i": 5, "outside_scope": True, "owner": ""},
]

def response(risk):
    score = risk["p"] * risk["i"]
    if risk.get("outside_scope"):
        return "escalate"
    if risk["kind"] == "threat":
        if score >= 15:
            if risk.get("optional"):
                return "avoid"
            return "transfer" if risk.get("transferable") else "mitigate"
        return "mitigate" if score >= 6 else "accept"
    if score >= 15:
        return "exploit"
    if score >= 6:
        return "share" if risk.get("needs_partner") else "enhance"
    return "accept"

for r in sorted(risks, key=lambda r: -r["p"] * r["i"]):
    print(f"{r['id']} {r['kind']:<12} {r['p'] * r['i']:>2}  {response(r):<9} {r['desc']}")
print("Needs an owner: " + ", ".join(r["id"] for r in risks if not r["owner"]))

@@exercise Sort costs into the cost of quality
Cost of quality has four parts: **prevention** and **appraisal** (the cost of conformance) and **internal failure** and **external failure** (the cost of nonconformance). Write `category(item)` by keyword: `training`, `standards`, `design review` → prevention; `testing`, `inspection`, `audit` → appraisal; `rework`, `scrap`, `retest` → internal failure; `warranty`, `recall`, `complaint`, `lost customer` → external failure.

Print the total per category, conformance versus nonconformance, and what share of the total the failures are.
@@lang python
@@starter
items = [
    ("Training testers on the new framework", 4_000),
    ("Coding standards workshop", 1_500),
    ("Design review sessions", 3_000),
    ("Automated testing infrastructure", 6_000),
    ("Security audit", 5_000),
    ("Rework after failed acceptance", 9_000),
    ("Retest of the payment module", 2_500),
    ("Warranty fixes after go-live", 12_000),
    ("Customer complaint handling", 3_500),
]
RULES = [
    ("prevention", ["training", "standards", "design review"]),
    ("appraisal", ["testing", "inspection", "audit"]),
    ("internal failure", ["rework", "scrap", "retest"]),
    ("external failure", ["warranty", "recall", "complaint", "lost customer"]),
]

def category(item):
    raise NotImplementedError


totals = {name: 0 for name, _ in RULES}
for item, cost in items:
    totals[category(item)] += cost
for name, cost in totals.items():
    print(f"{name:<17} ${cost:>7,}")
conformance = totals["prevention"] + totals["appraisal"]
nonconformance = totals["internal failure"] + totals["external failure"]
print(f"conformance ${conformance:,}, nonconformance ${nonconformance:,}")
print(f"failures are {nonconformance * 100 / (conformance + nonconformance):.0f}% of the cost of quality")
@@expected
prevention        $  8,500
appraisal         $ 11,000
internal failure  $ 11,500
external failure  $ 15,500
conformance $19,500, nonconformance $27,000
failures are 58% of the cost of quality
@@requires
def category
.lower()
@@hint
Check the keyword lists in order and add each cost into a dict keyed by category. Conformance is prevention + appraisal; nonconformance is the two failure categories.
@@solution
items = [
    ("Training testers on the new framework", 4_000),
    ("Coding standards workshop", 1_500),
    ("Design review sessions", 3_000),
    ("Automated testing infrastructure", 6_000),
    ("Security audit", 5_000),
    ("Rework after failed acceptance", 9_000),
    ("Retest of the payment module", 2_500),
    ("Warranty fixes after go-live", 12_000),
    ("Customer complaint handling", 3_500),
]
RULES = [
    ("prevention", ["training", "standards", "design review"]),
    ("appraisal", ["testing", "inspection", "audit"]),
    ("internal failure", ["rework", "scrap", "retest"]),
    ("external failure", ["warranty", "recall", "complaint", "lost customer"]),
]

def category(item):
    text = item.lower()
    for name, words in RULES:
        if any(w in text for w in words):
            return name
    return "unclassified"

totals = {name: 0 for name, _ in RULES}
for item, cost in items:
    totals[category(item)] += cost
for name, cost in totals.items():
    print(f"{name:<17} ${cost:>7,}")
conformance = totals["prevention"] + totals["appraisal"]
nonconformance = totals["internal failure"] + totals["external failure"]
print(f"conformance ${conformance:,}, nonconformance ${nonconformance:,}")
print(f"failures are {nonconformance * 100 / (conformance + nonconformance):.0f}% of the cost of quality")

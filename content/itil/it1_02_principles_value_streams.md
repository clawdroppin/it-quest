@@lesson it1-principles-in-practice Guiding Principles and Value Streams in Practice
@@text
Many Foundation questions are scenarios: "a team wants to replace the whole ticketing tool at once; which guiding principle should they consider?" To answer them quickly you need each principle's key phrases, the traps between similar principles, and a feel for how value streams use the value chain. This lesson drills that, then shows (**beyond Foundation**) how to measure a value stream's flow.

## The seven guiding principles: key phrases

| Principle | Key phrases in a question | Typical right answer |
|---|---|---|
| **Focus on value** | who is the consumer, what do they value, customer experience | ask what outcome the user needs |
| **Start where you are** | existing services, don't start from scratch, measure first | assess current state by **direct observation** before replacing |
| **Progress iteratively with feedback** | big project, huge change, all at once | split into smaller pieces, get feedback after each |
| **Collaborate and promote visibility** | silos, hidden work, stakeholders not informed | involve the right people, make work visible |
| **Think and work holistically** | one team optimises its part, end-to-end | consider the whole system and four dimensions |
| **Keep it simple and practical** | too many steps, controls nobody uses | remove steps that don't add value; outcome-based thinking |
| **Optimize and automate** | manual repetitive work | **optimise first**, then automate; use human intervention only where it adds value |

Traps:
- "Automate the current process" → **optimise first**.
- "Replace the tool and start fresh" → **start where you are**.
- "Measure everything" → keep it simple; measurement can distort behaviour (start where you are warns that measurement affects what is measured).
- The principles **interact**: apply them together, not one at a time.

## The value chain and value streams

Six value chain activities: **plan**, **improve**, **engage**, **design and transition**, **obtain/build**, **deliver and support**. Every value stream is a sequence of these activities for one scenario:

| Scenario | Typical value stream |
|---|---|
| User reports a broken laptop | engage → deliver and support → (obtain/build: replacement) → deliver and support → engage → improve |
| New application feature | engage → plan → design and transition → obtain/build → design and transition → deliver and support → improve |
| New employee needs equipment and access | engage → obtain/build → deliver and support → engage |

Remember which activity does what:
- **Engage**: understanding stakeholder needs, transparency, continual engagement and good relationships with all stakeholders (the main interface with users).
- **Plan**: shared understanding of vision, current status and improvement direction across all four dimensions.
- **Improve**: continual improvement of products, services and practices.
- **Design and transition**: products and services continually meet expectations for quality, costs and time to market.
- **Obtain/build**: service components are available when and where needed and meet specifications.
- **Deliver and support**: services are delivered and supported according to agreed specifications and stakeholder expectations.

## Measuring a value stream (beyond Foundation)

**Value stream mapping** lists each step with its **work time** (touch time) and the **wait time** before the next step.
- **Lead time** = total time from request to value delivered (work + wait).
- **Flow efficiency** = work time / lead time. Service desks often find under 10%: most time is waiting in queues and approvals.
- The **bottleneck** is where work waits longest; improving anywhere else doesn't speed up the flow. Remove waste (waiting, rework, handoffs, unnecessary approvals) and then automate (**optimise and automate**).

## The continual improvement model

1. What is the **vision**? 2. Where are we **now**? 3. Where do we want to **be**? 4. How do we **get there**? 5. **Take action**. 6. Did we **get there**? 7. How do we **keep the momentum** going?

@@quiz
A team plans to automate its approval process exactly as it is. Which principle should they consider first?
- Focus on value
* Optimize and automate (optimise before automating)
- Collaborate and promote visibility
- Start where you are
? Automating waste just produces waste faster.

@@quiz
Management wants to throw away the existing service desk process and design a new one from scratch. Which principle applies?
* Start where you are
- Progress iteratively with feedback
- Think and work holistically
- Keep it simple and practical
? Assess what exists and reuse what works.

@@quiz
A large improvement initiative is planned as one big delivery in 18 months. Which principle suggests a better approach?
- Focus on value
- Keep it simple and practical
* Progress iteratively with feedback
- Optimize and automate
? Smaller pieces, feedback after each.

@@quiz
Which value chain activity provides the main interface with users and other stakeholders?
- Plan
* Engage
- Deliver and support
- Improve
? Understanding needs and transparency.

@@quiz
Which value chain activity ensures service components are available when and where they are needed and meet agreed specifications?
- Design and transition
* Obtain/build
- Deliver and support
- Plan
? Including buying or building components.

@@quiz
In the continual improvement model, which step comes right after "Where are we now?"
- Take action
* Where do we want to be?
- Did we get there?
- How do we get there?
? Vision, now, target, plan, act, check, momentum.

@@flashcards
focus on value :: consumer outcomes and experience
start where you are :: observe, reuse what works
progress iteratively :: small steps with feedback
collaborate and promote visibility :: involve, make work visible
think holistically :: end-to-end, four dimensions
keep it simple :: remove steps without value
optimize and automate :: optimise first
engage :: stakeholder needs and relationships
plan :: shared vision and direction
improve :: continual improvement
design and transition :: quality, cost, time to market
obtain/build :: components available and to spec
deliver and support :: services as agreed
value stream :: activities for one scenario
lead time :: request to value delivered
flow efficiency :: work time / lead time
bottleneck :: longest wait limits flow
CI model step 1 :: What is the vision?
CI model step 7 :: How do we keep the momentum going?

@@drill
Which guiding principle says to optimise before you automate?
= optimize and automate
= optimise and automate
= Optimize and automate
? Use human intervention only where it adds value.

@@drill
How many activities are in the service value chain?
= 6
= six
? Plan, improve, engage, design and transition, obtain/build, deliver and support.

@@exercise Map a value stream and find the bottleneck
A "new employee laptop" value stream has steps with work and wait minutes and the value chain activity each belongs to. Compute lead time, flow efficiency and the bottleneck (largest wait). Then apply proposed improvements (remove a step, or cut a wait) and print the new lead time and efficiency. Also print the sequence of value chain activities, merging consecutive duplicates.
@@lang python
@@starter
steps = [
    ("request submitted", "engage", 10, 60),
    ("manager approval", "engage", 5, 1440),
    ("second approval (finance)", "engage", 5, 960),
    ("order laptop", "obtain/build", 15, 2880),
    ("image and configure", "obtain/build", 90, 240),
    ("create accounts", "deliver and support", 30, 480),
    ("hand over and explain", "engage", 20, 0),
]
improvements = [("remove", "second approval (finance)"), ("wait", "order laptop", 0), ("wait", "manager approval", 120)]

def measure(stream):
    raise NotImplementedError
@@expected
Value chain: engage -> obtain/build -> deliver and support -> engage
Before: lead time 103.9 h, work 2.9 h, flow efficiency 2.8%, bottleneck 'order laptop' (48 h wait)
After: lead time 17.8 h, work 2.8 h, flow efficiency 15.9%, bottleneck 'create accounts' (8 h wait)
@@requires
def measure
flow efficiency
@@hint
Each step is `(name, activity, work, wait)`. Lead time = sum of work + wait. For "order laptop", setting the wait to 0 models keeping a stock of pre-ordered laptops.
@@solution
steps = [
    ("request submitted", "engage", 10, 60),
    ("manager approval", "engage", 5, 1440),
    ("second approval (finance)", "engage", 5, 960),
    ("order laptop", "obtain/build", 15, 2880),
    ("image and configure", "obtain/build", 90, 240),
    ("create accounts", "deliver and support", 30, 480),
    ("hand over and explain", "engage", 20, 0),
]
improvements = [("remove", "second approval (finance)"), ("wait", "order laptop", 0), ("wait", "manager approval", 120)]

def measure(stream):
    work = sum(s[2] for s in stream)
    lead = work + sum(s[3] for s in stream)
    bottleneck = max(stream, key=lambda s: s[3])
    return work, lead, bottleneck

def report(label, stream):
    work, lead, bottleneck = measure(stream)
    print(f"{label}: lead time {lead / 60:.1f} h, work {work / 60:.1f} h, flow efficiency {work / lead:.1%}, bottleneck '{bottleneck[0]}' ({bottleneck[3] / 60:.0f} h wait)")

activities = []
for s in steps:
    if not activities or activities[-1] != s[1]:
        activities.append(s[1])
print("Value chain:", " -> ".join(activities))
report("Before", steps)

current = list(steps)
for change in improvements:
    if change[0] == "remove":
        current = [s for s in current if s[0] != change[1]]
    else:
        current = [(n, a, w, change[2]) if n == change[1] else (n, a, w, wt) for n, a, w, wt in current]
report("After", current)

@@exercise Route changes to the right authority
Classify each change and route it: changes on the standard change catalogue are **standard** (no further authorisation); changes fixing a P1 incident or an actively exploited vulnerability are **emergency** (emergency change authority, documentation may follow); everything else is **normal**, with authority by risk score = impact (1–3) × likelihood of failure (1–3): 1–2 automated pipeline approval, 3–4 team lead, 6 change advisory board, 9 CIO. Also flag normal changes scheduled in the freeze window.
@@lang python
@@starter
STANDARD_CATALOGUE = {"add memory to desktop", "reset MFA token", "add user to distribution list"}
FREEZE = (20, 31)
changes = [
    ("CHG3001", "reset MFA token", 2, 1, 5, False, False),
    ("CHG3002", "patch actively exploited VPN flaw", 3, 2, 21, False, True),
    ("CHG3003", "update web app config flag", 1, 1, 12, False, False),
    ("CHG3004", "upgrade database engine", 3, 2, 14, False, False),
    ("CHG3005", "migrate ERP to new data centre", 3, 3, 25, False, False),
    ("CHG3006", "restart failed payment service", 3, 1, 22, True, False),
    ("CHG3007", "replace core switch firmware", 2, 2, 23, False, False),
]

def classify(change):
    raise NotImplementedError
@@expected
CHG3001 reset MFA token                      standard  pre-authorised
CHG3002 patch actively exploited VPN flaw    emergency emergency change authority
CHG3003 update web app config flag           normal    risk 1: automated pipeline approval
CHG3004 upgrade database engine              normal    risk 6: change advisory board
CHG3005 migrate ERP to new data centre       normal    risk 9: CIO  !! day 25 is in the change freeze, reschedule
CHG3006 restart failed payment service       emergency emergency change authority
CHG3007 replace core switch firmware         normal    risk 4: team lead  !! day 23 is in the change freeze, reschedule
@@requires
def classify
STANDARD_CATALOGUE
FREEZE
@@hint
Each change is `(id, title, impact, likelihood, day, fixes_p1, exploited_vuln)`. Check standard first, then emergency, then compute the risk score for normal changes. Only normal changes are blocked by the freeze; emergencies go ahead.
@@solution
STANDARD_CATALOGUE = {"add memory to desktop", "reset MFA token", "add user to distribution list"}
FREEZE = (20, 31)
changes = [
    ("CHG3001", "reset MFA token", 2, 1, 5, False, False),
    ("CHG3002", "patch actively exploited VPN flaw", 3, 2, 21, False, True),
    ("CHG3003", "update web app config flag", 1, 1, 12, False, False),
    ("CHG3004", "upgrade database engine", 3, 2, 14, False, False),
    ("CHG3005", "migrate ERP to new data centre", 3, 3, 25, False, False),
    ("CHG3006", "restart failed payment service", 3, 1, 22, True, False),
    ("CHG3007", "replace core switch firmware", 2, 2, 23, False, False),
]

def classify(change):
    _, title, impact, likelihood, day, fixes_p1, exploited = change
    if title in STANDARD_CATALOGUE:
        return "standard", "pre-authorised"
    if fixes_p1 or exploited:
        return "emergency", "emergency change authority"
    risk = impact * likelihood
    if risk <= 2:
        authority = "automated pipeline approval"
    elif risk <= 4:
        authority = "team lead"
    elif risk <= 6:
        authority = "change advisory board"
    else:
        authority = "CIO"
    return "normal", f"risk {risk}: {authority}"

for change in changes:
    kind, route = classify(change)
    note = ""
    if kind == "normal" and FREEZE[0] <= change[4] <= FREEZE[1]:
        note = f"  !! day {change[4]} is in the change freeze, reschedule"
    print(f"{change[0]} {change[1]:<36} {kind:<9} {route}{note}")

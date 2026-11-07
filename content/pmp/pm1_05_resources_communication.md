@@lesson pm1-resources-comms Resources and Communication in Depth: Leveling, Smoothing, Channels and Reports
@@text
Resource and communication questions on the PMP mix judgement ("what should the project manager do?") with a few calculations. This lesson covers the calculations (resource histograms, leveling vs smoothing, communication channels) and the judgement rules that go with them.

## Resource optimisation

- A **resource histogram** shows how many units of a resource each period needs. Peaks above availability are **over-allocation**.
- **Resource leveling**: adjust start and finish dates to fit the resource limits. It **can change the critical path** and lengthen the project.
- **Resource smoothing**: adjust activities **only within their float**, so the end date and the critical path **don't change**, but not every peak may disappear.
- **Free float**: how long an activity can slip without delaying its successor. **Total float**: how long without delaying the project.
- Before leveling, consider alternatives: add resources (crashing), fast-track, or reduce scope through change control.

## Team resources

- **Resource calendar**: when people are actually available (holidays, part-time).
- **Responsibility assignment matrix (RAM / RACI)**: exactly one A (accountable) per activity.
- Tuckman (forming, storming, norming, performing, adjourning) and motivation theories are covered in the people lesson. In practice, the PM removes impediments and builds psychological safety.

## Communication

- **Channels** = n(n − 1) ÷ 2. Going from 10 to 15 stakeholders increases channels from 45 to 105.
- Methods: **interactive** (meetings, calls: for complex or sensitive topics), **push** (email, memos: sent but not necessarily read), **pull** (intranets, wikis: large audiences fetch information when they need it).
- The **communications management plan** says who needs what, when, in what format, from whom and how. Tailor it per stakeholder: the sponsor wants exceptions and decisions, while the team wants detail.
- Reports: **status** (where we are now), **progress** (what was done in the period), **forecast** (EAC, finish date). In agile, use information radiators: burndown and burnup charts and a visible Kanban board.
- Emotional intelligence and active listening: on the exam, talking to people directly is almost always better than sending another email.

@@quiz
Leveling the plan pushes the project end date out by a week. Is that possible?
* Yes: resource leveling can change the critical path and the end date
- No: leveling never changes dates
- Only with smoothing
- Only in agile projects
? Smoothing is the technique that keeps the end date.

@@quiz
The team grows from 6 to 9 people (the PM included in both counts). How many communication channels are added?
- 3
- 15
* 21
- 36
? 9 × 8 ÷ 2 = 36, minus 6 × 5 ÷ 2 = 15.

@@quiz
Which communication method suits a large audience that needs information occasionally?
- Interactive
- Push
* Pull
- Formal written only
? A wiki or portal.

@@quiz
A key engineer is double-booked across two projects. What should the PM do first?
- Escalate to the sponsor immediately
* Review the resource calendars and negotiate with the other project or functional manager
- Crash the schedule
- Remove the engineer's tasks from scope
? Analyse and negotiate before escalating.

@@quiz
What does free float measure?
* How long an activity can slip without delaying its immediate successor
- How long the project can slip
- Unused budget
- Time between sprints
? Total float is measured against the project end.

@@flashcards
resource histogram :: units needed per period
over-allocation :: demand above availability
resource leveling :: may extend the end date
resource smoothing :: within float, end date fixed
free float :: slip without delaying the successor
total float :: slip without delaying the project
resource calendar :: actual availability
channels :: n(n − 1) ÷ 2
interactive :: meetings, for complex topics
push :: email, sent but not necessarily read
pull :: wiki/portal, fetched when needed
status vs progress :: where we are vs what got done
information radiator :: visible burndown, board

@@drill
How many communication channels does a project with 12 stakeholders have?
= 66
? 12 × 11 ÷ 2.

@@drill
Which resource optimisation technique keeps the critical path unchanged?
= resource smoothing
= smoothing
? Leveling may change it.

@@exercise Smooth a resource histogram
Build a daily histogram of developers needed from activities with start days, durations, people and total float, then smooth it: delay activities only within their float (non-critical activities first, largest float first) to bring peaks down to the available limit, and report peaks that only leveling (extending the end date) could remove.
@@concepts lists, loops, functions, conditions, maps
@@lang python
@@starter
LIMIT = 4
# name: (start day, duration, people, total float)
activities = {
    "API": (0, 5, 2, 0),
    "UI": (0, 4, 2, 0),
    "Docs": (1, 3, 1, 3),
    "Data migration": (1, 3, 2, 3),
    "Load tests": (5, 2, 2, 0),
    "Security review": (3, 2, 1, 2),
}

def histogram(starts):
    raise NotImplementedError
@@expected
before: [4, 7, 7, 8, 3, 2, 2] peak 8
delay Docs by 3 day(s) (float 3)
delay Data migration by 3 day(s) (float 3)
after:  [4, 4, 4, 5, 6, 5, 5] peak 6
over the limit on days: [3, 4, 5, 6] -> leveling needed (end date moves)
@@requires
def histogram
LIMIT
float
sorted(
@@hint
`histogram` returns a list of people per day, from start day 0 to the last finish. To smooth: for each non-critical activity (largest float first), try every delay from 0 up to its float and keep the one that gives the lowest peak, then the lowest sum of overloads. Report days still above `LIMIT`.
@@solution
LIMIT = 4
# name: (start day, duration, people, total float)
activities = {
    "API": (0, 5, 2, 0),
    "UI": (0, 4, 2, 0),
    "Docs": (1, 3, 1, 3),
    "Data migration": (1, 3, 2, 3),
    "Load tests": (5, 2, 2, 0),
    "Security review": (3, 2, 1, 2),
}

def histogram(starts):
    end = max(starts[a] + activities[a][1] for a in activities)
    days = [0] * end
    for name, (_, duration, people, _) in activities.items():
        for d in range(starts[name], starts[name] + duration):
            days[d] += people
    return days

def score(starts):
    h = histogram(starts)
    return max(h), sum(max(0, x - LIMIT) for x in h)

starts = {name: a[0] for name, a in activities.items()}
print("before:", histogram(starts), "peak", max(histogram(starts)))
total_float = {name: a[3] for name, a in activities.items()}
for name in sorted((n for n in activities if total_float[n] > 0), key=lambda n: -total_float[n]):
    base = activities[name][0]
    best = min(range(total_float[name] + 1), key=lambda d: (score({**starts, name: base + d}), d))
    starts[name] = base + best
    if best:
        print(f"delay {name} by {best} day(s) (float {total_float[name]})")
after = histogram(starts)
print("after: ", after, "peak", max(after))
over = [d for d, x in enumerate(after) if x > LIMIT]
print("over the limit on days:", over if over else "none", "->", "leveling needed (end date moves)" if over else "smoothing was enough")

@@exercise Plan communications
Compute the channels as the team grows, then pick a communication method and frequency for each stakeholder from their interest, influence, audience size and how complex or sensitive the topic is.
@@lang python
@@starter
stakeholders = [
    ("Sponsor", "high", "high", 1, "budget overrun decision", True),
    ("Core team", "high", "medium", 8, "daily coordination", False),
    ("All staff", "low", "low", 600, "new system go-live date", False),
    ("Works council", "medium", "high", 5, "monitoring of employees", True),
    ("Vendor", "medium", "medium", 3, "delivery schedule", False),
]  # name, interest, influence, audience size, topic, complex or sensitive

def channels(n):
    raise NotImplementedError

def method(stakeholder):
    raise NotImplementedError
@@expected
5 people -> 10 channels
10 people -> 45 channels
15 people -> 105 channels
Sponsor        budget overrun decision    interactive (face-to-face meeting)        daily or weekly
Core team      daily coordination         interactive (stand-up / working session)  daily or weekly
All staff      new system go-live date    pull (intranet page) + push announcement  at milestones
Works council  monitoring of employees    interactive (face-to-face meeting)        every two weeks
Vendor         delivery schedule          push (email status report)                every two weeks
@@requires
def channels
def method
interactive
pull
@@hint
`channels(n)` = n × (n − 1) // 2. Method: complex or sensitive topics → interactive (meeting). Large audiences (over 50) → pull (intranet), plus a push announcement. High interest with high or medium influence → interactive and regular. Otherwise → push (email or report). Choose the frequency from the interest level.
@@solution
stakeholders = [
    ("Sponsor", "high", "high", 1, "budget overrun decision", True),
    ("Core team", "high", "medium", 8, "daily coordination", False),
    ("All staff", "low", "low", 600, "new system go-live date", False),
    ("Works council", "medium", "high", 5, "monitoring of employees", True),
    ("Vendor", "medium", "medium", 3, "delivery schedule", False),
]  # name, interest, influence, audience size, topic, complex or sensitive

def channels(n):
    return n * (n - 1) // 2

def method(stakeholder):
    name, interest, influence, size, topic, sensitive = stakeholder
    if sensitive:
        return "interactive (face-to-face meeting)"
    if size > 50:
        return "pull (intranet page) + push announcement"
    if interest == "high" and influence in ("high", "medium"):
        return "interactive (stand-up / working session)"
    return "push (email status report)"

FREQUENCY = {"high": "daily or weekly", "medium": "every two weeks", "low": "at milestones"}
for n in (5, 10, 15):
    print(f"{n} people -> {channels(n)} channels")
for s in stakeholders:
    print(f"{s[0]:<14} {s[4]:<26} {method(s):<41} {FREQUENCY[s[1]]}")

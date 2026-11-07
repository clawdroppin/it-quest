@@lesson pm0-pmp-people PMP: The Exam, Predictive and Agile, and People
@@text
## The PMP

The **Project Management Professional** (PMP) from PMI is the best-known project management certification in the world, and it's respected in Japan too (many SIers ask project managers and team leads for it; Japan has one of the largest PMP communities in Asia). Requirements: 35 hours of project management education and, with a four-year degree, **36 months** of experience leading projects (60 without one).

The exam is about **180 questions in 230 minutes**, with two 10-minute breaks. It has three domains. PMI revised the outline in 2026, moving weight towards the business environment; the older outline (still in many study books) used People 42%, Process 50%, Business environment 8%:

| Domain | Share (2026 outline) | About |
|---|---|---|
| **People** | about 33% | leading and supporting the team, conflict, stakeholders |
| **Process** | about 41% | planning and running the work: scope, schedule, cost, risk, quality |
| **Business environment** | about 26% | strategy and value, compliance, benefits, organizational change |

Always check PMI's current Exam Content Outline before you study: the exact numbers matter less than the mindset below.

About **half the questions are agile or hybrid**. The rest are **predictive** (traditional, "waterfall").

## Predictive, agile and hybrid

| | Predictive | Agile | Hybrid |
|---|---|---|---|
| Scope | defined up front | emerges, reprioritized often | stable parts planned, the rest iterative |
| Delivery | at the end (or in phases) | small increments every iteration | mixed |
| Change | through formal change control | welcomed, via the backlog | depends on the part |
| Fits | construction, regulated work, stable requirements | software, uncertain requirements | most real IT projects |

## The PMP mindset

Like CISSP, many questions have several plausible answers. The best one is usually:

- **Servant leadership**: support and unblock the team rather than command it. Coach first.
- **Assess before you act**: understand the situation, look at the plan or the risk register, talk to people, *then* decide.
- **Address problems early and directly**, face to face, at the lowest level, before escalating to the sponsor.
- **Follow the process**: changes to the baseline go through change control (predictive) or the product owner and backlog (agile).
- **Never** ignore a problem, hide it, or blame people.

## The team

- **Tuckman's stages**: forming → storming → norming → performing → adjourning.
- **Conflict resolution** (best long-term first): **collaborate/problem-solve** (win-win), **compromise** (both give something), **smooth/accommodate** (emphasize agreement), **force/direct** (win-lose, for emergencies), **withdraw/avoid** (postpone).
- **Motivation**: Maslow's hierarchy, Herzberg (hygiene factors versus motivators), McGregor's Theory X and Y, and Daniel Pink's autonomy, mastery and purpose.
- **Emotional intelligence**: self-awareness, self-management, social awareness, relationship management.
- **Team charter**: values, ground rules and working agreements, made *by* the team.
- **Virtual and distributed teams** (very common across Japan and overseas offices): agree on time zones, tools and response times; overlap hours for meetings.

## Stakeholders

- **Identify** them early and keep a **stakeholder register**.
- **Power/interest grid**: high power + high interest → manage closely; high power + low interest → keep satisfied; low power + high interest → keep informed; low + low → monitor.
- **Engagement assessment matrix**: unaware, resistant, neutral, supportive, leading; compare current (C) and desired (D) levels.
- **Communication**: the number of channels is **n(n − 1) / 2**. A team of 10 has 45 channels; adding 5 people brings it to 105.

@@quiz
Two team members argue about the technical approach. What does the project manager do first?
* Meet with both to understand the disagreement and help them find a solution together
- Escalate to the sponsor
- Choose the approach the project manager prefers
- Remove one of them from the team
? Collaborate and solve at the lowest level first. Forcing is for emergencies.

@@quiz
A team of 8 grows to 12 people. How many more communication channels are there?
* 38
- 4
- 66
- 28
? 8 × 7 / 2 = 28; 12 × 11 / 2 = 66; 66 − 28 = 38.

@@quiz
A stakeholder has high power but little interest in the project. How should they be managed?
* Keep satisfied
- Manage closely
- Keep informed
- Monitor
? Don't flood them with detail, but make sure their needs are met.

@@quiz
In an agile team, a developer is blocked by another department. What does a servant leader do?
* Work to remove the impediment for the team
- Tell the developer to work overtime
- Assign the task to someone else without asking
- Report the developer's low velocity to management
? Removing impediments is one of the servant leader's main jobs.

@@quiz
Which stage of team development is marked by conflict about roles and ways of working?
* Storming
- Forming
- Norming
- Adjourning
? Then the team settles its norms and starts performing.

@@flashcards
PMP domains :: People, Process, Business environment
servant leadership :: support, coach and unblock the team
Tuckman :: forming, storming, norming, performing, adjourning
best conflict technique :: collaborate / problem-solve
forcing :: win-lose, for emergencies only
power/interest grid :: manage closely, keep satisfied, keep informed, monitor
communication channels :: n(n − 1) / 2
team charter :: values and working agreements, made by the team
Herzberg hygiene factors :: salary, conditions: their absence demotivates
Theory X / Y :: people avoid work / people want to work

@@drill
How many communication channels does a team of 15 people have?
= 105

@@exercise Choose a delivery approach
Use the lesson's table to suggest **predictive**, **agile** or **hybrid** for a project. Score each answer: stable, known requirements and formal regulation push towards predictive; frequent change, a need for early feedback and a software product push towards agile.

Write `approach(project)` that adds +1 for each predictive signal and −1 for each agile signal, then returns `predictive` (score ≥ 2), `agile` (≤ −2) or `hybrid`. Also print the team's number of communication channels, n(n − 1) / 2, since a big team needs a communications plan either way.
@@lang python
@@starter
PREDICTIVE = ["requirements_stable", "regulated", "fixed_contract"]
AGILE = ["changes_often", "needs_early_feedback", "software"]
projects = [
    {"name": "Data centre move", "team": 12, "requirements_stable": True, "regulated": True, "fixed_contract": True},
    {"name": "Customer mobile app", "team": 7, "changes_often": True, "needs_early_feedback": True, "software": True},
    {"name": "ERP rollout with custom reports", "team": 20, "requirements_stable": True, "regulated": True,
     "software": True, "needs_early_feedback": True},
    {"name": "Internal chatbot pilot", "team": 4, "changes_often": True, "software": True, "fixed_contract": True,
     "needs_early_feedback": True},
]

def approach(project):
    raise NotImplementedError


def channels(n):
    raise NotImplementedError


for p in projects:
    score, kind = approach(p)
    print(f"{p['name']}: score {score:+d} -> {kind}; {p['team']} people, {channels(p['team'])} channels")
@@expected
Data centre move: score +3 -> predictive; 12 people, 66 channels
Customer mobile app: score -3 -> agile; 7 people, 21 channels
ERP rollout with custom reports: score +0 -> hybrid; 20 people, 190 channels
Internal chatbot pilot: score -2 -> agile; 4 people, 6 channels
@@requires
def approach
def channels
sum(
@@hint
`True` counts as 1 in `sum`, so `sum(project.get(signal, False) for signal in PREDICTIVE)` counts the predictive signals.
@@solution
PREDICTIVE = ["requirements_stable", "regulated", "fixed_contract"]
AGILE = ["changes_often", "needs_early_feedback", "software"]
projects = [
    {"name": "Data centre move", "team": 12, "requirements_stable": True, "regulated": True, "fixed_contract": True},
    {"name": "Customer mobile app", "team": 7, "changes_often": True, "needs_early_feedback": True, "software": True},
    {"name": "ERP rollout with custom reports", "team": 20, "requirements_stable": True, "regulated": True,
     "software": True, "needs_early_feedback": True},
    {"name": "Internal chatbot pilot", "team": 4, "changes_often": True, "software": True, "fixed_contract": True,
     "needs_early_feedback": True},
]

def approach(project):
    score = sum(project.get(s, False) for s in PREDICTIVE) - sum(project.get(s, False) for s in AGILE)
    if score >= 2:
        return score, "predictive"
    if score <= -2:
        return score, "agile"
    return score, "hybrid"

def channels(n):
    return n * (n - 1) // 2

for p in projects:
    score, kind = approach(p)
    print(f"{p['name']}: score {score:+d} -> {kind}; {p['team']} people, {channels(p['team'])} channels")

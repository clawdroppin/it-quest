@@lesson pm0-people-deep People in Depth: Leadership, Teams, Conflict, Stakeholders and Communication
@@text
The **People** domain is 33% of the PMP exam (Process 41%, Business Environment 26%). Questions are situational: "a team member is consistently late with deliverables; what should the project manager do first?" The answer is almost always a servant leader's: talk privately, understand the cause, coach, collaborate, then escalate only if needed. This lesson gives the models behind those answers, plus the numbers the exam can ask about.

## Servant leadership and emotional intelligence

- Servant leaders **remove impediments**, shield the team from distractions, coach and develop, encourage self-organisation, and lead by example.
- **Emotional intelligence**: self-awareness, self-management, social awareness, relationship management.
- Power types: **formal/legitimate**, reward, coercive/penalty, **expert**, **referent** (respect, admiration), informational. Expert and referent power are the most effective long term.

## Team development

**Tuckman ladder**: **Forming** (polite, unclear roles) → **Storming** (conflict over approach) → **Norming** (agreed ways of working) → **Performing** (high productivity) → **Adjourning** (closing). Teams can slip back when membership changes.

- **Team charter** / working agreement: values, ground rules, decision process, communication norms, definition of done (agile).
- Virtual teams: time zones, overlap hours, tools, cultural awareness; build trust deliberately.
- Motivation: Herzberg (hygiene factors vs motivators), McClelland (achievement, affiliation, power), Theory X/Y, Maslow, **Daniel Pink** (autonomy, mastery, purpose).
- Training needs, recognition and rewards that fit the culture (public praise isn't universally welcome; in Japanese teams group recognition often works better than individual spotlighting).

## Conflict management

Conflict is normal; address it early and privately. Thomas-Kilmann modes:

| Mode | When |
|---|---|
| **Collaborate / problem solve** (win-win) | default best: important issue, time to find a solution together |
| **Compromise / reconcile** | both give something; equal power, time pressure |
| **Accommodate / smooth** | emphasise agreement; preserve relationship, issue less important |
| **Force / direct** (win-lose) | emergencies, safety, legal; quick decisive action |
| **Avoid / withdraw** | cool off, issue trivial or will resolve itself |

Common sources: schedules, priorities, resources, technical opinions; personality is the **least** common source.

## Stakeholders

- **Identify early and continuously**; register with interest, influence, impact, attitude.
- **Power/interest grid**: high power + high interest → **manage closely**; high power + low interest → **keep satisfied**; low power + high interest → **keep informed**; low power + low interest → **monitor**.
- **Engagement assessment matrix**: Unaware, Resistant, Neutral, Supportive, Leading; compare **current (C)** with **desired (D)** and plan actions for the gaps.

## Communication

- **Channels** = n(n − 1)/2 for n people (including the PM). Adding people increases channels quadratically.
- Methods: **interactive** (meetings, calls: best for complex or sensitive issues), **push** (email, reports), **pull** (portal, wiki: large audiences).
- Communications management plan: who needs what information, when, in what format, from whom.

## Responsibility assignment

**RACI**: **R**esponsible (does the work), **A**ccountable (one owner who approves; **exactly one A** per activity), **C**onsulted (two-way), **I**nformed (one-way).

## Negotiation and decisions

Aim for win-win; know your BATNA (best alternative to a negotiated agreement). Group decisions: unanimity, majority, plurality, dictatorship/autocratic, **fist of five** and roman voting in agile teams; **Delphi** for expert consensus.

@@quiz
A team member keeps missing deadlines. What should the project manager do first?
- Report them to their functional manager
- Replace them
* Meet privately to understand the cause and offer support
- Discuss it in the next team meeting
? Servant leadership: understand before acting.

@@quiz
The team is arguing about which technical approach to use. Which Tuckman stage is this?
- Forming
* Storming
- Norming
- Performing
? Conflict over how to work.

@@quiz
A project team grows from 6 to 10 people. How many additional communication channels are there?
- 4
- 16
* 30
- 45
? 15 → 45 channels.

@@quiz
Which conflict resolution mode is generally the most effective for an important issue?
- Forcing
- Smoothing
* Collaborating / problem solving
- Withdrawing
? It aims for a win-win outcome.

@@quiz
A regulator has high power but little interest in the project's details. How should they be managed?
- Manage closely
* Keep satisfied
- Keep informed
- Monitor
? Power/interest grid.

@@quiz
In a RACI chart, how many people should be Accountable for an activity?
* Exactly one
- At least one
- Everyone who is Responsible
- None, if a team owns it
? A single owner approves the work.

@@quiz
Which type of power comes from respect and admiration for the project manager?
- Formal
- Coercive
* Referent
- Reward
? Expert power comes from knowledge.

@@flashcards
People domain :: 33% of PMP
servant leader :: removes impediments, coaches
expert power :: from knowledge
referent power :: from respect
Tuckman :: forming, storming, norming, performing, adjourning
team charter :: values, ground rules, agreements
Pink :: autonomy, mastery, purpose
Herzberg :: hygiene factors vs motivators
collaborate :: win-win problem solving
compromise :: both give something
smooth :: emphasise agreement
force :: win-lose, emergencies
avoid :: postpone or withdraw
least common conflict source :: personality
manage closely :: high power, high interest
keep satisfied :: high power, low interest
keep informed :: low power, high interest
monitor :: low power, low interest
engagement matrix :: current vs desired
channels :: n(n − 1)/2
interactive communication :: complex or sensitive
pull communication :: large audiences
RACI A :: exactly one per activity
BATNA :: best alternative to agreement

@@drill
How many communication channels are there for 8 people?
= 28
? 8 × 7 / 2.

@@drill
Which power/interest grid strategy applies to low power, high interest stakeholders?
= keep informed
= Keep informed
? High power, high interest: manage closely.

@@exercise Plan stakeholder engagement
Place each stakeholder on the power/interest grid (score ≥ 3 out of 5 counts as high) and choose the strategy. Compare current and desired engagement levels (Unaware, Resistant, Neutral, Supportive, Leading) and print the gap direction. Finally compute communication channels for the team before and after adding new members.
@@lang python
@@starter
LEVELS = ["Unaware", "Resistant", "Neutral", "Supportive", "Leading"]
stakeholders = [
    ("CFO (sponsor)", 5, 5, "Supportive", "Leading"),
    ("Regulator", 5, 1, "Unaware", "Neutral"),
    ("Call centre staff", 2, 5, "Resistant", "Supportive"),
    ("Facilities", 1, 2, "Neutral", "Neutral"),
    ("IT security", 4, 4, "Neutral", "Supportive"),
]
team_before, team_added = 7, 5

def strategy(power, interest):
    raise NotImplementedError
@@expected
CFO (sponsor)      P5 I5  manage closely  raise Supportive -> Leading (1 step)
Regulator          P5 I1  keep satisfied  raise Unaware -> Neutral (2 steps)
Call centre staff  P2 I5  keep informed   raise Resistant -> Supportive (2 steps)
Facilities         P1 I2  monitor         maintain
IT security        P4 I4  manage closely  raise Neutral -> Supportive (1 step)
Channels: 21 with 7 people, 66 with 12 (+45)
@@requires
def strategy
def channels
LEVELS.index(
@@hint
`channels(n) = n * (n - 1) // 2`. The gap is `LEVELS.index(desired) - LEVELS.index(current)`: positive means raise engagement.
@@solution
LEVELS = ["Unaware", "Resistant", "Neutral", "Supportive", "Leading"]
stakeholders = [
    ("CFO (sponsor)", 5, 5, "Supportive", "Leading"),
    ("Regulator", 5, 1, "Unaware", "Neutral"),
    ("Call centre staff", 2, 5, "Resistant", "Supportive"),
    ("Facilities", 1, 2, "Neutral", "Neutral"),
    ("IT security", 4, 4, "Neutral", "Supportive"),
]
team_before, team_added = 7, 5

def strategy(power, interest):
    if power >= 3 and interest >= 3:
        return "manage closely"
    if power >= 3:
        return "keep satisfied"
    if interest >= 3:
        return "keep informed"
    return "monitor"

def channels(n):
    return n * (n - 1) // 2

for name, power, interest, current, desired in stakeholders:
    gap = LEVELS.index(desired) - LEVELS.index(current)
    plan = f"raise {current} -> {desired} ({gap} step{'s' if gap > 1 else ''})" if gap > 0 else "maintain"
    print(f"{name:<18} P{power} I{interest}  {strategy(power, interest):<15} {plan}")

after = team_before + team_added
print(f"Channels: {channels(team_before)} with {team_before} people, {channels(after)} with {after} (+{channels(after) - channels(team_before)})")

@@exercise Validate a RACI matrix
Check a RACI matrix: every activity needs exactly one A and at least one R; flag people who are R or A on more than `MAX_LOAD` activities (overload), and activities where more than half the team is C (too many consulted, slow decisions). Print the problems, or that the matrix is valid.
@@lang python
@@starter
MAX_LOAD = 3
people = ["Aiko", "Ben", "Chen", "Dana", "Eiji"]
raci = {
    "Requirements": {"Aiko": "A", "Ben": "R", "Chen": "C", "Dana": "I"},
    "Architecture": {"Aiko": "A", "Chen": "R", "Ben": "C", "Dana": "C", "Eiji": "C"},
    "Build": {"Chen": "R", "Ben": "R", "Eiji": "I"},
    "Test": {"Dana": "A", "Eiji": "R", "Chen": "R", "Aiko": "A"},
    "Deploy": {"Chen": "A", "Eiji": "R"},
    "Training": {"Dana": "R", "Aiko": "I"},
}

def check(raci):
    raise NotImplementedError
@@expected
Architecture: 3 people Consulted, decisions will be slow
Build: 0 Accountable (needs exactly 1)
Test: 2 Accountable (needs exactly 1)
Training: 0 Accountable (needs exactly 1)
Chen: R or A on 4 activities (max 3)
@@requires
def check
MAX_LOAD
"A"
@@hint
Count `list(roles.values()).count("A")` per activity. For load, count activities where the person's role is R or A. "More than half the team" is `consulted > len(people) / 2`.
@@solution
MAX_LOAD = 3
people = ["Aiko", "Ben", "Chen", "Dana", "Eiji"]
raci = {
    "Requirements": {"Aiko": "A", "Ben": "R", "Chen": "C", "Dana": "I"},
    "Architecture": {"Aiko": "A", "Chen": "R", "Ben": "C", "Dana": "C", "Eiji": "C"},
    "Build": {"Chen": "R", "Ben": "R", "Eiji": "I"},
    "Test": {"Dana": "A", "Eiji": "R", "Chen": "R", "Aiko": "A"},
    "Deploy": {"Chen": "A", "Eiji": "R"},
    "Training": {"Dana": "R", "Aiko": "I"},
}

def check(raci):
    problems = []
    for activity, roles in raci.items():
        values = list(roles.values())
        if values.count("A") != 1:
            problems.append(f"{activity}: {values.count('A')} Accountable (needs exactly 1)")
        if "R" not in values:
            problems.append(f"{activity}: nobody Responsible")
        if values.count("C") > len(people) / 2:
            problems.append(f"{activity}: {values.count('C')} people Consulted, decisions will be slow")
    for person in people:
        load = sum(1 for roles in raci.values() if roles.get(person) in ("R", "A"))
        if load > MAX_LOAD:
            problems.append(f"{person}: R or A on {load} activities (max {MAX_LOAD})")
    return problems

found = check(raci)
print("RACI valid" if not found else "\n".join(found))

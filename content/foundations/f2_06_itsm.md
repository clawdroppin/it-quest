@@lesson f2-itsm IT Service Management: Incidents, Changes and Documentation
@@text
An IT department isn't a group of people fixing things whenever someone shouts. It's a service with promises (how fast, how reliable), processes that keep it predictable, and records that let anyone pick up where someone else left off. **IT Service Management (ITSM)** is the name for this, and **ITIL** is the most widely used framework for it (there's a full ITIL 4 track later). This lesson covers the parts you'll touch from your first day.

## The service desk

The **service desk** is the single point of contact between IT and its users: a portal, an email address, a phone line, a chat. Everything goes through a **ticketing system** (ServiceNow, Jira Service Management, Freshservice, Zendesk, TOPdesk). Tickets are the memory of the department: what happened, what was tried, who fixed it, how long it took.

**Tiers**:

- **Tier 1 (L1)**: takes the contact, logs it, solves common issues (password resets, how-to questions, known fixes from the knowledge base), escalates the rest.
- **Tier 2 (L2)**: deeper technical support: desktop, server, network specialists.
- **Tier 3 (L3)**: engineers, architects, or the vendor.

**Shift-left** means moving fixes to earlier tiers and to self-service (knowledge articles, a password reset portal), which is cheaper and faster for everyone.

## Incidents vs service requests vs problems

| Record | Meaning | Example | Goal |
|---|---|---|---|
| **Incident** | something is broken or degraded | "Outlook crashes on start" | restore service fast |
| **Service request** | a normal, expected ask | "I need Visio", "new starter on Monday" | fulfil it |
| **Problem** | the underlying cause of one or more incidents | "Outlook crashes because of add-in X" | find and remove the root cause |
| **Change** | adding, modifying or removing something | "update the add-in on all PCs" | do it safely |

A **workaround** restores service without fixing the cause ("disable the add-in"); a **known error** is a problem with a documented cause and workaround.

## Priority = impact × urgency

| | High urgency | Medium urgency | Low urgency |
|---|---|---|---|
| **High impact** (many users, or a critical service) | P1 | P2 | P3 |
| **Medium impact** (a department) | P2 | P3 | P4 |
| **Low impact** (one user) | P3 | P4 | P5 |

The CEO's broken mouse is still one user with a workaround (a spare mouse). Payroll down on payday is P1. Priorities drive **SLA** targets, such as P1: respond in 15 minutes and resolve in 4 hours; P4: respond in 1 business day.

A **major incident** (a P1 affecting many people) gets its own process: an incident manager, a bridge call, regular status updates to users, and a **post-incident review** afterwards. Good reviews are **blameless**: they ask what in the system let this happen, not who to punish.

## Change management

Most outages are caused by changes. Change management keeps the speed but removes surprises:

- **Standard changes**: pre-approved, low risk, repeatable (adding a user to a group, deploying a tested patch to a pilot group).
- **Normal changes**: assessed and approved, often by a **Change Advisory Board (CAB)** or an automated pipeline with peer review.
- **Emergency changes**: needed now to fix a major incident; approved quickly and reviewed afterwards.

A good change request answers: what and why, which systems and users are affected, **when** (a maintenance window), how it was tested, **how to roll back**, and who will check it worked. Users are told in advance.

## Documentation and the knowledge base

If it isn't written down, it doesn't exist the day you're on holiday.

- **Knowledge articles** for users ("How to connect to the VPN") and for IT staff ("Fixing error 0x800CCC0E in Outlook").
- **Runbooks**: step-by-step procedures for recurring operational tasks.
- **Configuration records**: what you have and how it's connected: the **CMDB** (configuration management database) or asset inventory: hardware, software, licences, owners, warranty dates.
- **Network diagrams**, IP address plans, vendor contacts, contract dates.

A good knowledge article has: a clear title using the user's words ("Can't print to the 3rd floor printer"), the symptoms, the cause if known, numbered steps with screenshots, and a date and owner. Check articles every year; a wrong article is worse than none.

## Writing a good ticket note

Every update should let a colleague continue without calling you:

> 2026-03-05 10:42 – Called Ana (ext. 2214). Outlook crashes on start since this morning's update. Safe mode (`outlook.exe /safe`) works → add-in suspected. Disabled "PDF Maker" add-in: Outlook starts normally. Workaround applied, linked to problem PRB-0042 (same add-in on 12 PCs). Ana confirmed working. Resolved.

What you checked, what you found, what you changed, what the user confirmed.

## Metrics

- **First contact resolution (FCR)**: solved during the first contact.
- **MTTR** (mean time to resolve/restore).
- **SLA compliance**: percentage of tickets meeting their targets.
- **Backlog** and ticket **ageing**.
- **Customer satisfaction (CSAT)** surveys after resolution.

Metrics guide improvements; chasing one blindly (closing tickets too early to hit SLA) makes service worse.

@@quiz
Several users report Outlook crashing after an update. What's the relationship between their tickets and the cause?
* Each report is an incident; the shared underlying cause is a problem
- Each report is a problem
- It's a change request
- They're service requests
? Problem management finds the root cause behind incidents.

@@quiz
Payroll is down for the whole company on payday. What priority is this?
* P1: high impact and high urgency
- P3
- P5
- Depends on who reported it
? Priority = impact × urgency, not the caller's rank.

@@quiz
What must every normal change request include?
- The user's password
* What, why, impact, schedule, test evidence and a rollback plan
- Only the date
- A list of tickets closed this week
? The rollback plan is the part people forget.

@@quiz
What is a blameless post-incident review?
* A review focused on the system and process gaps that allowed the incident, not on punishing a person
- A review with no notes
- Skipping the review
- A review where nobody is allowed to speak
? People hide mistakes when they're punished, and the system never improves.

@@flashcards
service desk :: single point of contact for users
L1 / L2 / L3 :: support tiers
shift-left :: move resolution to earlier tiers and self-service
incident :: unplanned interruption or degradation
service request :: a normal, expected ask
problem :: underlying cause of incidents
workaround :: restores service without fixing the cause
known error :: a problem with known cause and workaround
priority :: impact × urgency
SLA :: agreed response/resolution targets
major incident :: high-impact incident with its own process
standard change :: pre-approved, low risk
CAB :: Change Advisory Board
CMDB :: database of configuration items
FCR :: first contact resolution
MTTR :: mean time to resolve

@@drill
What type of change is pre-approved and low risk?
= standard
= standard change
? For example adding a user to a group.

@@drill
Priority is calculated from impact and what?
= urgency
? Impact × urgency.

@@exercise A priority matrix and SLA deadlines
Implement the lesson's priority matrix as `priority(impact, urgency)` where both are `"high"`, `"medium"` or `"low"` (hint: score high=1, medium=2, low=3 and `P = impact + urgency - 1`). Using the SLA resolve targets (P1 4 h, P2 8 h, P3 24 h, P4 72 h, P5 120 h), print each ticket's priority and resolve-by time from its opened time with `datetime` and `timedelta`, ignoring business hours.
@@lang python
@@starter
from datetime import datetime, timedelta

tickets = [
    ("INC-101", "Payroll down for everyone", "high", "high", "2026-03-05 09:00"),
    ("INC-102", "Finance shared drive slow", "medium", "medium", "2026-03-05 10:30"),
    ("INC-103", "Ana's second monitor flickers", "low", "medium", "2026-03-05 11:15"),
    ("INC-104", "Wallpaper request", "low", "low", "2026-03-05 12:00"),
]
SLA_HOURS = {1: 4, 2: 8, 3: 24, 4: 72, 5: 120}

def priority(impact, urgency):
    raise NotImplementedError
@@expected
INC-101 P1 Payroll down for everyone: resolve by 2026-03-05 13:00
INC-102 P3 Finance shared drive slow: resolve by 2026-03-06 10:30
INC-103 P4 Ana's second monitor flickers: resolve by 2026-03-08 11:15
INC-104 P5 Wallpaper request: resolve by 2026-03-10 12:00
@@requires
def priority
timedelta(hours=
strptime(
@@hint
`opened = datetime.strptime(text, "%Y-%m-%d %H:%M")`, `due = opened + timedelta(hours=SLA_HOURS[p])`, then `due.strftime("%Y-%m-%d %H:%M")`.
@@solution
from datetime import datetime, timedelta

tickets = [
    ("INC-101", "Payroll down for everyone", "high", "high", "2026-03-05 09:00"),
    ("INC-102", "Finance shared drive slow", "medium", "medium", "2026-03-05 10:30"),
    ("INC-103", "Ana's second monitor flickers", "low", "medium", "2026-03-05 11:15"),
    ("INC-104", "Wallpaper request", "low", "low", "2026-03-05 12:00"),
]
SLA_HOURS = {1: 4, 2: 8, 3: 24, 4: 72, 5: 120}
SCORE = {"high": 1, "medium": 2, "low": 3}

def priority(impact, urgency):
    return SCORE[impact] + SCORE[urgency] - 1

for ticket_id, title, impact, urgency, opened_text in tickets:
    p = priority(impact, urgency)
    opened = datetime.strptime(opened_text, "%Y-%m-%d %H:%M")
    due = opened + timedelta(hours=SLA_HOURS[p])
    print(f"{ticket_id} P{p} {title}: resolve by {due.strftime('%Y-%m-%d %H:%M')}")

@@exercise Service desk metrics
From the closed tickets, compute and print: the number of tickets, first contact resolution rate (percentage with `contacts == 1`), the mean time to resolve in hours (one decimal), SLA compliance (percentage resolved within their priority's target from the previous exercise), and the number of tickets per category, most common first.
@@lang python
@@starter
SLA_HOURS = {1: 4, 2: 8, 3: 24, 4: 72, 5: 120}
closed = [
    {"id": 1, "category": "password", "priority": 4, "hours": 0.2, "contacts": 1},
    {"id": 2, "category": "email", "priority": 3, "hours": 30.0, "contacts": 3},
    {"id": 3, "category": "password", "priority": 4, "hours": 0.1, "contacts": 1},
    {"id": 4, "category": "network", "priority": 1, "hours": 3.5, "contacts": 2},
    {"id": 5, "category": "printer", "priority": 4, "hours": 5.0, "contacts": 1},
    {"id": 6, "category": "password", "priority": 4, "hours": 0.3, "contacts": 1},
    {"id": 7, "category": "email", "priority": 2, "hours": 9.0, "contacts": 2},
]
@@expected
Tickets: 7
First contact resolution: 57%
MTTR: 6.9 hours
SLA compliance: 71%
  password: 3
  email: 2
  network: 1
  printer: 1
@@requires
Counter(
most_common(
@@hint
`from collections import Counter`. SLA met: `t["hours"] <= SLA_HOURS[t["priority"]]`. Percentages: `round(100 * met / len(closed))`.
@@solution
from collections import Counter

SLA_HOURS = {1: 4, 2: 8, 3: 24, 4: 72, 5: 120}
closed = [
    {"id": 1, "category": "password", "priority": 4, "hours": 0.2, "contacts": 1},
    {"id": 2, "category": "email", "priority": 3, "hours": 30.0, "contacts": 3},
    {"id": 3, "category": "password", "priority": 4, "hours": 0.1, "contacts": 1},
    {"id": 4, "category": "network", "priority": 1, "hours": 3.5, "contacts": 2},
    {"id": 5, "category": "printer", "priority": 4, "hours": 5.0, "contacts": 1},
    {"id": 6, "category": "password", "priority": 4, "hours": 0.3, "contacts": 1},
    {"id": 7, "category": "email", "priority": 2, "hours": 9.0, "contacts": 2},
]

n = len(closed)
fcr = sum(1 for t in closed if t["contacts"] == 1)
mttr = sum(t["hours"] for t in closed) / n
met = sum(1 for t in closed if t["hours"] <= SLA_HOURS[t["priority"]])

print("Tickets:", n)
print(f"First contact resolution: {round(100 * fcr / n)}%")
print(f"MTTR: {mttr:.1f} hours")
print(f"SLA compliance: {round(100 * met / n)}%")
for category, count in Counter(t["category"] for t in closed).most_common():
    print(f"  {category}: {count}")

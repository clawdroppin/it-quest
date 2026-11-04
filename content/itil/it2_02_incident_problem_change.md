@@lesson it2-incident-problem-change Incident, Problem and Change in Practice
@@text
The Foundation exam tests the **purpose** and **key terms** of the practices; a service desk job tests whether you can run them. This lesson does both: it restates exactly what the exam expects, then shows how real service desks prioritise incidents, run major incidents, move from incidents to problems and known errors, and decide which changes need which authorisation. Sections marked **(beyond Foundation)** are workplace practice that the exam doesn't test directly.

## Incident management: what the exam expects

- Purpose: minimise the negative impact of incidents by **restoring normal service operation as quickly as possible**.
- Incidents are **prioritised** based on agreed classification, so the highest business impact is resolved first.
- Incidents may be resolved by the service desk, escalated to support teams, handled by **swarming**, or by suppliers/partners.
- A **major incident** has its own procedure, often with a separate team and communication plan.
- Use **scripts** for diagnosing and resolving simple, common incidents; use the **knowledge base**.
- Users should be kept informed; **records** are kept in a suitable tool and linked to configuration items, changes, problems and known errors.

## Prioritisation with impact and urgency (beyond Foundation)

**Impact** = how much of the business is affected (one user, a department, everyone; revenue, safety, reputation). **Urgency** = how quickly the business needs it resolved. **Priority** comes from a matrix:

| Impact \ Urgency | High | Medium | Low |
|---|---|---|---|
| **High** | P1 | P2 | P3 |
| **Medium** | P2 | P3 | P4 |
| **Low** | P3 | P4 | P5 |

Each priority has **response** and **resolution** targets in the SLA (e.g. P1: respond 15 min, resolve 4 h, 24×7; P4: respond 4 business hours, resolve 3 business days). Don't let users self-select priority without validation: "everything is urgent".

**Functional escalation** moves the ticket to a team with more expertise; **hierarchical escalation** informs management (breach risk, VIP, major incident).

## Major incidents (beyond Foundation)

Declare early; assign a **major incident manager**; open a bridge; communicate on a fixed cadence (status page, stakeholders); focus on **restoring service** (rollback, failover, workaround) before root cause; hand over to **problem management** afterwards; hold a blameless review.

## Problem management: what the exam expects

- Purpose: reduce the **likelihood and impact of incidents** by identifying actual and potential causes, and managing workarounds and known errors.
- **Problem**: a cause, or potential cause, of one or more incidents.
- **Known error**: a problem that has been **analysed but not resolved**.
- **Workaround**: a solution that reduces or eliminates the impact of an incident or problem for which a full resolution is not yet available. Some workarounds are documented in problem records; effective ones may become permanent.
- Phases: **problem identification** (trend analysis, recurring incidents, supplier info, testing) → **problem control** (analysis, documenting workarounds and known errors, prioritised by risk) → **error control** (managing known errors, assessing permanent solutions, which may need a **change request**).
- Problem management is **reactive** (after incidents) and **proactive** (before they happen).

Techniques (beyond Foundation): **5 whys**, Ishikawa (fishbone) diagrams, Kepner-Tregoe, fault tree analysis, timeline reconstruction, Pareto analysis of incident categories.

## Change enablement: what the exam expects

- Purpose: **maximise the number of successful service and product changes** by ensuring that risks have been properly assessed, authorising changes to proceed, and managing the change schedule.
- **Change**: the addition, modification or removal of anything that could have a direct or indirect effect on services.
- **Change authority**: the person or group that authorises a change; it varies by change type and should be as **close to the work** as possible (decentralised), to keep flow fast.
- **Standard changes**: low-risk, pre-authorised, well understood and documented; can be implemented without additional authorisation; often initiated as service requests.
- **Normal changes**: need to be scheduled, assessed and authorised following a process; low-risk ones can have fast authorisation (often automated in pipelines), high-risk ones may need a very senior authority.
- **Emergency changes**: must be implemented as soon as possible (e.g. resolving an incident or a security patch); not usually in the change schedule; assessment and authorisation are expedited, possibly with a separate authority, and documentation may be completed afterwards.
- The **change schedule** plans changes, helps avoid conflicts, and communicates.

## Linking it together

Incident → (recurring) → problem → known error with workaround → **change** to fix permanently → incident volume drops. Configuration items (**service configuration management**) connect them; the **service desk** is where most of it starts. Measure: incidents per known error, repeat incidents, change success rate, emergency change ratio, mean time to restore.

@@quiz
What is the purpose of incident management?
- To find the root cause of incidents
* To minimise the negative impact of incidents by restoring normal service operation as quickly as possible
- To authorise changes
- To handle pre-defined user requests
? Root cause is problem management.

@@quiz
What is a known error?
- An incident that keeps recurring
* A problem that has been analysed but not resolved
- A change that failed
- An error in the knowledge base
? It often has a documented workaround.

@@quiz
Which phase of problem management assesses permanent solutions and may raise a change request?
- Problem identification
- Problem control
* Error control
- Incident control
? Identification → control → error control.

@@quiz
A pre-authorised, low-risk change that follows a documented procedure is which type?
* Standard change
- Normal change
- Emergency change
- Minor change
? Often initiated as a service request.

@@quiz
Where should change authority be placed, according to ITIL 4?
- Always with a central CAB
* As close as possible to the work, to keep changes flowing
- With the service desk
- With the supplier
? Decentralised authority speeds up flow.

@@quiz
An outage affects the whole company's email and must be fixed immediately. Using a standard matrix, what's the priority?
* P1 (high impact, high urgency)
- P2
- P3
- P5
? Impact × urgency.

@@quiz
Which incident management technique brings people with different skills together to work on a complex incident?
- Escalation
* Swarming
- Scripting
- Triage
? Instead of passing the ticket down a line of teams.

@@flashcards
incident :: unplanned interruption or reduced quality
incident mgmt purpose :: restore service quickly
priority :: impact × urgency
impact :: how much of the business is affected
urgency :: how fast it's needed
functional escalation :: to more expertise
hierarchical escalation :: to management
major incident :: separate procedure
swarming :: experts work it together
problem :: cause of one or more incidents
known error :: analysed, not resolved
workaround :: reduces impact without a full fix
problem phases :: identification, control, error control
proactive problem mgmt :: before incidents occur
5 whys :: root cause technique
change :: add, modify, remove anything affecting services
change authority :: person or group that authorises
standard change :: pre-authorised, low risk
normal change :: assessed and authorised
emergency change :: ASAP, expedited
change schedule :: plan and communicate changes

@@drill
What is the name for a problem that has been analysed but not resolved?
= known error
= a known error
? Often with a workaround.

@@drill
Which type of change is pre-authorised and low risk?
= standard
= standard change
? Normal changes need authorisation.

@@exercise Prioritise incidents and check SLA targets
Compute each incident's priority from the impact × urgency matrix, then compare its response and resolution times (in minutes) with the SLA targets for that priority. Print breaches, and a summary of the percentage of incidents resolved within target per priority.
@@lang python
@@starter
MATRIX = {("high", "high"): 1, ("high", "medium"): 2, ("high", "low"): 3,
          ("medium", "high"): 2, ("medium", "medium"): 3, ("medium", "low"): 4,
          ("low", "high"): 3, ("low", "medium"): 4, ("low", "low"): 5}
TARGETS = {1: (15, 240), 2: (30, 480), 3: (60, 1440), 4: (240, 4320), 5: (480, 7200)}
incidents = [
    ("INC1001", "high", "high", 10, 200), ("INC1002", "medium", "high", 45, 300), ("INC1003", "low", "low", 300, 2000),
    ("INC1004", "high", "medium", 20, 600), ("INC1005", "medium", "medium", 30, 1000), ("INC1006", "high", "high", 25, 230),
    ("INC1007", "low", "medium", 100, 5000), ("INC1008", "medium", "low", 200, 3000),
]

def priority(impact, urgency):
    raise NotImplementedError
@@expected
INC1001 P1: within SLA
INC1002 P2: BREACH response 45 > 30 min
INC1003 P5: within SLA
INC1004 P2: BREACH resolution 600 > 480 min
INC1005 P3: within SLA
INC1006 P1: BREACH response 25 > 15 min
INC1007 P4: BREACH resolution 5000 > 4320 min
INC1008 P4: within SLA
P1: 2/2 resolved within target (100%)
P2: 1/2 resolved within target (50%)
P3: 1/1 resolved within target (100%)
P4: 1/2 resolved within target (50%)
P5: 1/1 resolved within target (100%)
@@requires
def priority
MATRIX[
TARGETS[
@@hint
`priority` is just a dictionary lookup. Keep a dict `stats[p] = [total, resolved_in_time]` and print the ones that exist, sorted by priority.
@@solution
MATRIX = {("high", "high"): 1, ("high", "medium"): 2, ("high", "low"): 3,
          ("medium", "high"): 2, ("medium", "medium"): 3, ("medium", "low"): 4,
          ("low", "high"): 3, ("low", "medium"): 4, ("low", "low"): 5}
TARGETS = {1: (15, 240), 2: (30, 480), 3: (60, 1440), 4: (240, 4320), 5: (480, 7200)}
incidents = [
    ("INC1001", "high", "high", 10, 200), ("INC1002", "medium", "high", 45, 300), ("INC1003", "low", "low", 300, 2000),
    ("INC1004", "high", "medium", 20, 600), ("INC1005", "medium", "medium", 30, 1000), ("INC1006", "high", "high", 25, 230),
    ("INC1007", "low", "medium", 100, 5000), ("INC1008", "medium", "low", 200, 3000),
]

def priority(impact, urgency):
    return MATRIX[(impact, urgency)]

stats = {}
for ticket, impact, urgency, response, resolution in incidents:
    p = priority(impact, urgency)
    target_response, target_resolution = TARGETS[p]
    problems = []
    if response > target_response:
        problems.append(f"response {response} > {target_response} min")
    if resolution > target_resolution:
        problems.append(f"resolution {resolution} > {target_resolution} min")
    print(f"{ticket} P{p}: {'BREACH ' + '; '.join(problems) if problems else 'within SLA'}")
    total, ok = stats.get(p, (0, 0))
    stats[p] = (total + 1, ok + (resolution <= target_resolution))

for p in sorted(stats):
    total, ok = stats[p]
    print(f"P{p}: {ok}/{total} resolved within target ({ok / total:.0%})")

@@exercise From incidents to problems
Group incidents by configuration item and symptom. Raise a problem when the same CI and symptom recur at least `THRESHOLD` times within `WINDOW_DAYS`. If a known error already exists for that CI and symptom, link the incidents to it instead and report the workaround. Print the problems raised, the incidents linked to known errors, and the rest as isolated incidents.
@@lang python
@@starter
THRESHOLD = 3
WINDOW_DAYS = 14
incidents = [
    ("INC2001", 1, "mail-gw-01", "mail delayed"), ("INC2002", 3, "vpn-01", "disconnects"), ("INC2003", 4, "mail-gw-01", "mail delayed"),
    ("INC2004", 9, "mail-gw-01", "mail delayed"), ("INC2005", 10, "printer-3f", "paper jam"), ("INC2006", 12, "vpn-01", "disconnects"),
    ("INC2007", 15, "erp-app", "slow reports"), ("INC2008", 30, "vpn-01", "disconnects"), ("INC2009", 31, "erp-app", "slow reports"),
    ("INC2010", 33, "erp-app", "slow reports"), ("INC2011", 34, "erp-app", "slow reports"),
]
known_errors = {("vpn-01", "disconnects"): "KE-17: reconnect with the backup gateway vpn-02"}

def recurring(group):
    raise NotImplementedError
@@expected
PRB0001 raised for mail-gw-01 / mail delayed: INC2001, INC2003, INC2004 within 14 days
Known error vpn-01 / disconnects: linked INC2002, INC2006, INC2008; workaround KE-17: reconnect with the backup gateway vpn-02
PRB0002 raised for erp-app / slow reports: INC2009, INC2010, INC2011 within 14 days
Isolated incidents: INC2005
@@requires
def recurring
WINDOW_DAYS
known_errors
@@hint
For each group (sorted by day), slide a window: for each starting incident, take the incidents with `day - start_day < WINDOW_DAYS`; if there are at least `THRESHOLD`, return them.
@@solution
THRESHOLD = 3
WINDOW_DAYS = 14
incidents = [
    ("INC2001", 1, "mail-gw-01", "mail delayed"), ("INC2002", 3, "vpn-01", "disconnects"), ("INC2003", 4, "mail-gw-01", "mail delayed"),
    ("INC2004", 9, "mail-gw-01", "mail delayed"), ("INC2005", 10, "printer-3f", "paper jam"), ("INC2006", 12, "vpn-01", "disconnects"),
    ("INC2007", 15, "erp-app", "slow reports"), ("INC2008", 30, "vpn-01", "disconnects"), ("INC2009", 31, "erp-app", "slow reports"),
    ("INC2010", 33, "erp-app", "slow reports"), ("INC2011", 34, "erp-app", "slow reports"),
]
known_errors = {("vpn-01", "disconnects"): "KE-17: reconnect with the backup gateway vpn-02"}

def recurring(group):
    group = sorted(group, key=lambda i: i[1])
    for start in group:
        window = [i for i in group if 0 <= i[1] - start[1] < WINDOW_DAYS]
        if len(window) >= THRESHOLD:
            return window
    return None

groups = {}
for inc in incidents:
    groups.setdefault((inc[2], inc[3]), []).append(inc)

isolated = []
number = 1
for key, group in groups.items():
    ids = ", ".join(i[0] for i in group)
    if key in known_errors:
        print(f"Known error {key[0]} / {key[1]}: linked {ids}; workaround {known_errors[key]}")
        continue
    window = recurring(group)
    if window:
        print(f"PRB{number:04d} raised for {key[0]} / {key[1]}: {', '.join(i[0] for i in window)} within {WINDOW_DAYS} days")
        number += 1
    else:
        isolated += [i[0] for i in group]
print("Isolated incidents:", ", ".join(isolated))

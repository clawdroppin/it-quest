@@lesson it2-practices The ITIL Practices You Use Every Day
@@text
ITIL 4 has 34 practices. The Foundation exam asks you to *recall the purpose* of 15 and to *understand* 7 in detail. The ones in **bold** below are the 7: they're also the daily work of a service desk or operations team.

## The seven in detail

**Incident management**: minimize the negative impact of incidents by restoring normal service operation **as quickly as possible**. An **incident** is an unplanned interruption to a service, or a reduction in its quality. Log every incident, **prioritize** by impact and urgency, use **scripts** for common ones, **swarming** for complex ones (people with different skills work on it together), and agree target resolution times. Major incidents get a separate procedure.

**Problem management**: reduce the likelihood and impact of incidents by identifying actual and potential **causes** and managing **workarounds** and **known errors**.
- **Problem**: a cause, or potential cause, of one or more incidents.
- **Known error**: a problem that has been analysed but not resolved.
- **Workaround**: reduces or eliminates the impact of an incident or problem for which a full resolution isn't yet available.
- Three phases: **problem identification → problem control → error control**.

**Service request management**: support the agreed quality of a service by handling all pre-defined, user-initiated service requests effectively and in a user-friendly manner. Requests (a new laptop, access to a folder, a password reset, information) are a normal part of service delivery, **not failures**. They should be standardized and automated where possible, with workflows and approvals.

**Service desk**: capture demand for incident resolution and service requests; the single point of contact between the provider and all its users. Channels: phone, email, chat, portal, walk-in. The desk needs **empathy, communication and business understanding**, not just technical knowledge. It can be centralized, local, virtual or follow-the-sun.

**Change enablement**: maximize the number of successful service and product changes by ensuring risks are properly assessed, authorizing changes, and managing the change schedule.
- **Change**: the addition, modification or removal of anything that could have a direct or indirect effect on services.
- **Standard change**: low risk, pre-authorized, follows a procedure (e.g. adding memory to a desktop).
- **Normal change**: needs assessment and authorization by a **change authority** (which varies with the type of change).
- **Emergency change**: must be implemented as soon as possible (e.g. a security patch), with faster assessment.

**Service level management**: set clear business-based targets for service levels, and ensure delivery is properly assessed, monitored and managed. **SLA**: a documented agreement between a provider and a customer that identifies services and expected levels. Beware the **watermelon effect**: SLA metrics green on the outside, users red (unhappy) inside. Measure what matters to users, with customer engagement and feedback.

**Continual improvement**: align practices and services with changing business needs through ongoing improvement. Everyone's responsibility; a **continual improvement register (CIR)** tracks ideas.

## The other eight to recall

| Practice | Purpose |
|---|---|
| **Information security management** | protect information: confidentiality, integrity, availability (plus authentication and non-repudiation) |
| **Relationship management** | establish and nurture links with stakeholders |
| **Supplier management** | ensure suppliers and their performance are managed appropriately |
| **IT asset management** | plan and manage the full lifecycle of all IT assets: maximize value, control costs and risks |
| **Monitoring and event management** | observe services and components, record and report changes of state (**events**: informational, warning, exception) |
| **Release management** | make new and changed services and features **available for use** |
| **Service configuration management** | accurate information on the configuration of services and the **configuration items (CIs)** supporting them, in a **CMDB** |
| **Deployment management** | **move** new or changed hardware, software, documentation or processes to live environments |

Note the difference: **deployment** moves components into an environment; **release** makes them available to users. With feature flags, code can be deployed today and released next month.

## In Japanese workplaces

| English | 日本語 |
|---|---|
| incident | インシデント / 障害 |
| problem management | 問題管理 (再発防止) |
| service request | サービス要求 |
| service desk | サービスデスク / ヘルプデスク |
| change | 変更 (変更管理) |
| escalation | エスカレーション |
| workaround | 回避策 / 暫定対応 |
| permanent fix | 恒久対応 |
| operations and maintenance | 運用保守 |

**暫定対応 → 恒久対応** (workaround → permanent fix) is how Japanese reports describe the incident → problem flow. Learn it.

@@quiz
What is the purpose of incident management?
* To minimize the negative impact of incidents by restoring normal service operation as quickly as possible
- To find the root cause of every incident
- To authorize changes
- To manage supplier contracts
? Root causes are problem management's job.

@@quiz
A problem has been analysed, its cause is known, but it isn't fixed yet. What is it called?
* A known error
- An incident
- A standard change
- An event
? A workaround may reduce its impact meanwhile.

@@quiz
A user asks for access to a shared folder. What is this?
* A service request
- An incident
- A problem
- An emergency change
? Nothing is broken; it's a normal, pre-defined request.

@@quiz
Replacing a failed hard disk in a desktop following a pre-approved, low-risk procedure is...
* A standard change
- A normal change
- An emergency change
- A problem
? Standard changes are pre-authorized.

@@quiz
A critical security vulnerability must be patched on all servers tonight. What type of change?
* An emergency change
- A standard change
- A service request
- A release
? Emergency changes still get assessed, just faster, often by a smaller authority.

@@quiz
Which practice is the single point of contact for users?
* Service desk
- Relationship management
- Supplier management
- Service level management
? It captures demand for incidents and service requests.

@@quiz
Several experts with different skills work together on a complex incident at the same time. What's this called?
* Swarming
- Escalation
- Change enablement
- Problem control
? An alternative to passing the ticket from team to team.

@@quiz
What does the "watermelon effect" describe?
* SLA metrics look green while customers are actually unhappy
- Services that are cheap but slow
- Incidents that turn into problems
- Too many standard changes
? Measure outcomes users care about, not just easy numbers.

@@quiz
Which practice makes new or changed services and features available for use?
* Release management
- Deployment management
- Service configuration management
- IT asset management
? Deployment moves them into an environment; release makes them available.

@@quiz
Where is information about configuration items and their relationships kept?
* In the CMDB, maintained by service configuration management
- In the SLA
- In the continual improvement register
- In the known error database only
? CI = configuration item.

@@quiz
Which practice records and reports changes of state, classifying them as informational, warning or exception?
* Monitoring and event management
- Incident management
- Problem management
- Relationship management
? Not every event is an incident.

@@quiz
What are the three phases of problem management?
* Problem identification, problem control, error control
- Detect, mitigate, resolve
- Plan, do, check
- Log, escalate, close
? Error control manages known errors and workarounds.

@@flashcards
incident :: Unplanned interruption or reduced quality of a service
incident management purpose :: Restore normal service as quickly as possible
problem :: Cause or potential cause of one or more incidents
known error :: Analysed problem not yet resolved
workaround :: Reduces impact until a full fix exists
problem management phases :: Identification, control, error control
service request :: Pre-defined user request, normal part of delivery
service desk :: Single point of contact for users
change :: Add, modify or remove anything that could affect services
standard change :: Low risk, pre-authorized
normal change :: Assessed and authorized by a change authority
emergency change :: Implemented ASAP, expedited assessment
change authority :: Person or group that authorizes a change
SLA :: Documented agreement on services and expected levels
watermelon effect :: Green metrics, unhappy users
CIR :: Continual improvement register
CMDB / CI :: Configuration database / configuration item
release vs deployment :: Available for use vs moved into an environment
event :: Change of state significant for a service or CI
swarming :: Experts collaborate at once on an issue
暫定対応 / 恒久対応 :: Workaround / permanent fix

@@drill
What ITIL term describes a problem that has been analysed but not yet resolved? (two words)
= known error
? Kept in a known error database, often with a workaround.

@@exercise Name the practice for each piece of work
The service desk receives mixed work. Write `practice(item)` that names the ITIL practice that should own it, checking in this order:

1. a fault with a known root cause still being worked on, or a repeat of the same fault (`repeats` 3 or more) → **problem management**
2. something broken or degraded now (`type: fault`) → **incident management**
3. a request for something already in the catalogue (`in_catalogue`) → **service request management**
4. a request to add, modify or remove something not in the catalogue → **change enablement**
5. feedback or an improvement idea → **continual improvement**
6. a question about agreed targets (`about_sla`) → **service level management**

Print each item with its practice, then how many items each practice received.
@@lang python
@@starter
items = [
    {"text": "Email down for the whole sales team", "type": "fault", "repeats": 1},
    {"text": "New starter needs a laptop", "type": "request", "in_catalogue": True},
    {"text": "VPN drops every Monday morning, fourth time", "type": "fault", "repeats": 4},
    {"text": "Upgrade the firewall firmware", "type": "request", "in_catalogue": False},
    {"text": "Password reset", "type": "request", "in_catalogue": True},
    {"text": "Survey: users want a self-service portal", "type": "feedback"},
    {"text": "Customer asks why resolution took 3 days when the target is 1", "type": "question", "about_sla": True},
    {"text": "Printer jam on floor 3", "type": "fault", "repeats": 1},
]

def practice(item):
    raise NotImplementedError


counts = {}
for it in items:
    p = practice(it)
    counts[p] = counts.get(p, 0) + 1
    print(f"{it['text']} -> {p}")
print()
for p, n in sorted(counts.items(), key=lambda kv: (-kv[1], kv[0])):
    print(f"{n}  {p}")
@@expected
Email down for the whole sales team -> incident management
New starter needs a laptop -> service request management
VPN drops every Monday morning, fourth time -> problem management
Upgrade the firewall firmware -> change enablement
Password reset -> service request management
Survey: users want a self-service portal -> continual improvement
Customer asks why resolution took 3 days when the target is 1 -> service level management
Printer jam on floor 3 -> incident management

2  incident management
2  service request management
1  change enablement
1  continual improvement
1  problem management
1  service level management
@@requires
def practice
@@hint
Test the repeat count before the plain fault check: a recurring fault is still an incident each time, but the pattern belongs to problem management.
@@solution
items = [
    {"text": "Email down for the whole sales team", "type": "fault", "repeats": 1},
    {"text": "New starter needs a laptop", "type": "request", "in_catalogue": True},
    {"text": "VPN drops every Monday morning, fourth time", "type": "fault", "repeats": 4},
    {"text": "Upgrade the firewall firmware", "type": "request", "in_catalogue": False},
    {"text": "Password reset", "type": "request", "in_catalogue": True},
    {"text": "Survey: users want a self-service portal", "type": "feedback"},
    {"text": "Customer asks why resolution took 3 days when the target is 1", "type": "question", "about_sla": True},
    {"text": "Printer jam on floor 3", "type": "fault", "repeats": 1},
]

def practice(item):
    kind = item["type"]
    if kind == "fault" and item.get("repeats", 0) >= 3:
        return "problem management"
    if kind == "fault":
        return "incident management"
    if kind == "request":
        return "service request management" if item.get("in_catalogue") else "change enablement"
    if kind == "feedback":
        return "continual improvement"
    if item.get("about_sla"):
        return "service level management"
    return "service desk: clarify with the user"

counts = {}
for it in items:
    p = practice(it)
    counts[p] = counts.get(p, 0) + 1
    print(f"{it['text']} -> {p}")
print()
for p, n in sorted(counts.items(), key=lambda kv: (-kv[1], kv[0])):
    print(f"{n}  {p}")

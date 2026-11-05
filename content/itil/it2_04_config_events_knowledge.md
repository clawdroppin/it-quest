@@lesson it2-config-events Configuration, Monitoring and Events, Releases and Knowledge
@@text
Beyond the "big three" (incident, problem, change), ITIL 4 Foundation expects you to know the purpose and key terms of several more practices. This lesson covers the ones that matter most in day-to-day operations: service configuration management (and the CMDB), IT asset management, monitoring and event management, release and deployment management, and knowledge management, with exercises that do impact analysis on a configuration model and filter monitoring events.

## Service configuration management

- **Purpose**: ensure accurate and reliable information about the configuration of services and the **configuration items (CIs)** that support them is available when and where needed.
- A **CI** is any component that needs to be managed to deliver an IT service: servers, applications, network devices, documentation, even contracts.
- CIs and their **relationships** (runs on, depends on, connects to) are recorded in a **CMS/CMDB**. The value is in the relationships: they answer "what's affected if this fails?" (impact analysis) and "what changed?" during incidents.
- Accuracy needs discovery tools plus discipline: changes update the CMDB.

## IT asset management

- **Purpose**: plan and manage the full **lifecycle** of all IT assets to maximise value, control costs, manage risks and support decisions about purchase, reuse, retirement and disposal, and meet regulatory and contractual requirements (software licence compliance).
- An **IT asset** is any financially valuable component that can contribute to an IT product or service. Assets and CIs overlap but aren't the same: a spare laptop in storage is an asset but not (yet) a CI.

## Monitoring and event management

- **Purpose**: systematically observe services and components, and record and report selected changes of state identified as **events**.
- An **event** is any change of state that has significance for the management of a service or CI.
- Event types: **informational** (no action: a backup finished), **warning** (a threshold is approaching: disk 85% full), **exception** (a breach needing action: service down), which may raise an incident.
- Good practice: thresholds tied to service impact, **filtering** noise, **correlation** (one root event behind many symptoms), deduplication, and automated responses.

## Release management and deployment management

- **Release management**: make new and changed services and features **available for use**. A **release** is a version of a service or other CI, or a collection of them, made available.
- **Deployment management**: **move** new or changed hardware, software, documentation, processes or any other component to live (or other) environments.
- In modern practice they're decoupled: deploy continuously, release with feature flags (see the DevOps track).

## Knowledge management

- **Purpose**: maintain and improve the effective, efficient and convenient use of information and knowledge across the organisation.
- Data → information → knowledge → wisdom (DIKW). Knowledge articles for the service desk, known errors and workarounds from problem management, and self-service portals. Knowledge-Centered Service (KCS) creates articles as part of solving tickets.

@@quiz
Which practice ensures information about CIs and their relationships is available when needed?
* Service configuration management
- IT asset management
- Release management
- Service level management
? The CMDB is its main tool.

@@quiz
A disk reaches 85% of its capacity threshold. What kind of event is this?
- Informational
* Warning
- Exception
- Incident
? Approaching a threshold.

@@quiz
What does deployment management do?
* Moves new or changed components to live or other environments
- Makes features available to users
- Approves changes
- Manages contracts
? Release management makes things available for use.

@@quiz
Which is an IT asset but not necessarily a CI?
- The production database server
* A spare laptop in storage
- The email service
- A network switch in use
? It doesn't yet support a service.

@@quiz
What is event correlation?
- Deleting all warnings
* Recognising that many symptom events share one underlying cause
- Logging every event as an incident
- Sending events to customers
? It reduces noise and speeds diagnosis.

@@flashcards
configuration item :: component managed to deliver a service
CMDB / CMS :: CI records and relationships
impact analysis :: what's affected if this fails
IT asset :: financially valuable component
asset lifecycle :: plan, acquire, use, retire, dispose
event :: significant change of state
informational event :: no action needed
warning event :: approaching a threshold
exception event :: breach, action needed
correlation :: many symptoms, one cause
release management :: make available for use
deployment management :: move to environments
knowledge management :: use information effectively
DIKW :: data, information, knowledge, wisdom
KCS :: knowledge-centered service

@@drill
Which type of event signals that a threshold has been breached and action is needed?
= exception
= an exception
? It may raise an incident.

@@drill
What does CI stand for in configuration management?
= configuration item
? Not continuous integration here.

@@exercise Impact analysis with a CMDB
CIs depend on other CIs (`app depends on db`). Given a failed CI, walk the relationships upwards to find every CI and business service affected, and report the affected services sorted by priority. Then find single points of failure: CIs whose failure would affect every service.
@@lang python
@@starter
depends_on = {
    "svc:online-shop": ["app:shop-web", "app:payments"],
    "svc:intranet": ["app:wiki"],
    "svc:email": ["app:mail"],
    "app:shop-web": ["vm:web-1", "db:orders"],
    "app:payments": ["vm:pay-1", "db:orders"],
    "app:wiki": ["vm:wiki-1"],
    "app:mail": ["saas:m365"],
    "vm:web-1": ["host:esx-1"], "vm:pay-1": ["host:esx-2"], "vm:wiki-1": ["host:esx-1"],
    "db:orders": ["host:esx-2", "san:main"],
    "host:esx-1": ["net:core-switch", "san:main"], "host:esx-2": ["net:core-switch", "san:main"],
    "saas:m365": ["net:internet"], "net:core-switch": [], "san:main": [], "net:internet": [],
}
PRIORITY = {"svc:online-shop": 1, "svc:intranet": 3, "svc:email": 2}

def affected_by(ci):
    raise NotImplementedError
@@expected
db:orders fails -> 3 CIs affected; services: svc:online-shop (P1)
host:esx-1 fails -> 6 CIs affected; services: svc:online-shop (P1), svc:intranet (P3)
net:internet fails -> 3 CIs affected; services: svc:email (P2)
san:main fails -> 11 CIs affected; services: svc:online-shop (P1), svc:intranet (P3)
single points of failure for every service: none
shared by the shop and the intranet: host:esx-1, net:core-switch, san:main
@@requires
def affected_by
PRIORITY
svc:
@@hint
Build the reverse map (CI → CIs that depend on it). From the failed CI, walk the reverse map with a stack, collecting everything reachable. Services are the names starting with `svc:`. A single point of failure is a CI whose affected services equal all services.
@@solution
depends_on = {
    "svc:online-shop": ["app:shop-web", "app:payments"],
    "svc:intranet": ["app:wiki"],
    "svc:email": ["app:mail"],
    "app:shop-web": ["vm:web-1", "db:orders"],
    "app:payments": ["vm:pay-1", "db:orders"],
    "app:wiki": ["vm:wiki-1"],
    "app:mail": ["saas:m365"],
    "vm:web-1": ["host:esx-1"], "vm:pay-1": ["host:esx-2"], "vm:wiki-1": ["host:esx-1"],
    "db:orders": ["host:esx-2", "san:main"],
    "host:esx-1": ["net:core-switch", "san:main"], "host:esx-2": ["net:core-switch", "san:main"],
    "saas:m365": ["net:internet"], "net:core-switch": [], "san:main": [], "net:internet": [],
}
PRIORITY = {"svc:online-shop": 1, "svc:intranet": 3, "svc:email": 2}

used_by = {ci: [] for ci in depends_on}
for ci, deps in depends_on.items():
    for d in deps:
        used_by[d].append(ci)

def affected_by(ci):
    seen, stack = set(), [ci]
    while stack:
        for parent in used_by[stack.pop()]:
            if parent not in seen:
                seen.add(parent)
                stack.append(parent)
    return seen

for failed in ("db:orders", "host:esx-1", "net:internet", "san:main"):
    hit = affected_by(failed)
    services = sorted((s for s in hit if s.startswith("svc:")), key=PRIORITY.get)
    print(f"{failed} fails -> {len(hit)} CIs affected; services: {', '.join(f'{s} (P{PRIORITY[s]})' for s in services) or 'none'}")
all_services = set(PRIORITY)
spof = sorted(ci for ci in depends_on if not ci.startswith("svc:") and all_services <= affected_by(ci))
internal = sorted(ci for ci in depends_on if not ci.startswith("svc:") and {"svc:online-shop", "svc:intranet"} <= affected_by(ci))
print("single points of failure for every service:", ", ".join(spof) or "none")
print("shared by the shop and the intranet:", ", ".join(internal))

@@exercise Filter and correlate monitoring events
Classify raw monitoring events as informational, warning or exception from their metric and thresholds, deduplicate repeats from the same source and check within 5 minutes, and correlate: when a host is down, suppress the exceptions from services running on it and raise one incident for the host instead.
@@lang python
@@starter
THRESHOLDS = {"disk_pct": (80, 95), "cpu_pct": (85, 98)}  # (warning, exception)
RUNS_ON = {"web-1": "esx-1", "wiki-1": "esx-1", "pay-1": "esx-2"}
events = [
    (0, "backup-job", "status", "completed"),
    (60, "db-1", "disk_pct", 86),
    (120, "db-1", "disk_pct", 87),
    (400, "db-1", "disk_pct", 96),
    (500, "esx-1", "host", "down"),
    (505, "web-1", "http", "down"),
    (507, "wiki-1", "http", "down"),
    (510, "pay-1", "cpu_pct", 99),
    (530, "web-1", "http", "down"),
]

def classify(event):
    raise NotImplementedError
@@expected
t=  0 backup-job status=completed: informational
t= 60 db-1 disk_pct=86: warning
t=120 db-1 disk_pct=87: duplicate, suppressed
t=400 db-1 disk_pct=96: exception -> raise incident
t=500 esx-1 DOWN: exception -> raise incident for host esx-1
t=505 web-1 http=down: correlated with host esx-1 down, no new incident
t=507 wiki-1 http=down: correlated with host esx-1 down, no new incident
t=510 pay-1 cpu_pct=99: exception -> raise incident
t=530 web-1 http=down: duplicate, suppressed
@@requires
def classify
THRESHOLDS
RUNS_ON
@@hint
`classify` returns "informational", "warning" or "exception": numeric metrics use the thresholds, "down" is an exception, anything else informational. Deduplicate with a dict of the last time each `(source, check, level)` was seen (skip if within 300 s). Track hosts that are down; a service exception whose host is down is suppressed and linked to the host incident.
@@solution
THRESHOLDS = {"disk_pct": (80, 95), "cpu_pct": (85, 98)}  # (warning, exception)
RUNS_ON = {"web-1": "esx-1", "wiki-1": "esx-1", "pay-1": "esx-2"}
events = [
    (0, "backup-job", "status", "completed"),
    (60, "db-1", "disk_pct", 86),
    (120, "db-1", "disk_pct", 87),
    (400, "db-1", "disk_pct", 96),
    (500, "esx-1", "host", "down"),
    (505, "web-1", "http", "down"),
    (507, "wiki-1", "http", "down"),
    (510, "pay-1", "cpu_pct", 99),
    (530, "web-1", "http", "down"),
]

def classify(event):
    _, _, check, value = event
    if check in THRESHOLDS:
        warn, exc = THRESHOLDS[check]
        return "exception" if value >= exc else "warning" if value >= warn else "informational"
    return "exception" if value == "down" else "informational"

last_seen, hosts_down = {}, set()
for event in events:
    t, source, check, value = event
    level = classify(event)
    key = (source, check, level)
    if key in last_seen and t - last_seen[key] < 300:
        print(f"t={t:>3} {source} {check}={value}: duplicate, suppressed")
        continue
    last_seen[key] = t
    if check == "host" and value == "down":
        hosts_down.add(source)
        print(f"t={t:>3} {source} DOWN: exception -> raise incident for host {source}")
    elif level == "exception" and RUNS_ON.get(source) in hosts_down:
        print(f"t={t:>3} {source} {check}={value}: correlated with host {RUNS_ON[source]} down, no new incident")
    elif level == "exception":
        print(f"t={t:>3} {source} {check}={value}: exception -> raise incident")
    else:
        print(f"t={t:>3} {source} {check}={value}: {level}")

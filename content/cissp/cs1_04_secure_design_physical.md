@@lesson cs1-secure-design-physical Secure Design Principles, Cloud and Physical Security (Domain 3)
@@concepts conditions, maps, lists
@@lang python
@@text
Domain 3 asks you to design systems that stay secure when parts fail: principles that guide every design decision, the shared responsibility split in the cloud, the weak spots of virtualisation, embedded and industrial systems, and the physical building that houses it all. Exam questions here reward the principle, not the product name.

## Secure design principles

- **Least privilege** and **need to know**: grant the minimum, for the minimum time.
- **Defence in depth**: layered controls, so one failure isn't a breach.
- **Separation of duties**: no single person can complete a sensitive task alone.
- **Secure defaults** and **fail securely**. A firewall that fails should deny (fail closed). A fire door should unlock (**fail safe**, because people come first).
- **Keep it simple** (economy of mechanism), **complete mediation** (check every access, every time), **open design** (security mustn't depend on secrecy of the design, which is Kerckhoffs's principle).
- **Zero trust**: never trust by network location; verify explicitly, use least privilege, assume breach. A policy decision point and policy enforcement points sit in front of every resource.
- **Privacy by design** (Cavoukian's 7 principles): proactive, privacy as the default, embedded into design, full functionality, end-to-end security, visibility, respect for users.
- **Trust but verify** and **shared responsibility** apply to suppliers and cloud providers alike.

## Cloud and virtualisation

- **Shared responsibility**: the provider secures the cloud, and you secure what you put in it. The split moves with the service model:
  - **IaaS**: you handle the OS, patching, the runtime, the application, the data, identities and network rules.
  - **PaaS**: the provider takes over the OS and runtime, and you keep the application, data and identities.
  - **SaaS**: you keep the data, identities and access configuration.
  - **Data, accounts and access are always yours.**
- Deployment models: public, private, community and hybrid.
- Virtualisation risks:
  - **VM escape** (code in a guest reaching the hypervisor).
  - **VM sprawl** (forgotten, unpatched VMs).
  - Snapshots that contain secrets.
  - Noisy neighbours.
  - Type 1 (bare metal) hypervisors have a smaller attack surface than type 2 (hosted).
- Containers share a kernel, so the isolation is weaker than with VMs. Scan images, run as non-root and keep the orchestrator's API locked down.
- **Serverless**: less to patch, but more IAM to get right and logs to collect.

## Embedded, IoT and industrial systems

- **ICS/SCADA**, PLCs and OT networks put availability and safety above confidentiality, and are hard to patch.
  - Segment them (the **Purdue model** levels).
  - Allow only required protocols through **data diodes** or tightly filtered conduits.
  - Monitor passively.
- **IoT**: default credentials, no updates, weak crypto. Put devices on their own network, change the credentials, and buy from vendors with a security update policy.

## Physical security and site design

- **CPTED** (Crime Prevention Through Environmental Design): natural surveillance (visibility), natural access control (landscaping and paths that guide people), territorial reinforcement (signs and fences that mark ownership).
- **Layers**: perimeter (fences, bollards, lighting), building (locks, mantraps, guards, CCTV), and secure areas such as the data centre (badge plus PIN or biometrics, logging).
- **Fire**: detection (ionisation, photoelectric, heat), and suppression by fire class:
  - **Class A** is ordinary combustibles: water or foam.
  - **Class B** is flammable liquids: CO₂, foam or dry chemical.
  - **Class C** (US naming) is electrical: CO₂ or clean agent, never water while it's live.
  - **Class D** is combustible metals: dry powder.
  - **Class K** is kitchen oils: wet chemical.
- Sprinkler types:
  - **Wet pipe** is always full, so it's fast but can leak.
  - **Dry pipe** fills on activation, which suits cold areas.
  - **Pre-action** (the data centre choice) fills on detection and releases at the head, so two events must happen before water flows.
  - **Deluge** opens every head at once.
- Clean agents (FM-200, Novec 1230, inert gases) protect equipment. Halon is banned for new systems.
- **Environment**: ASHRAE recommends roughly **18–27 °C** inlet temperature and a dew point between about −9 °C and 15 °C (relative humidity up to about 60%) for data centres. Too dry means static, too humid means condensation and corrosion.
- **Power**: UPS for short outages and conditioning, generators for long ones. Know the problems: a **sag** (brief low voltage), **brownout** (prolonged low), **spike** (brief high), **surge** (prolonged high), **fault** (brief loss), **blackout** (prolonged loss).

@@quiz
A data centre door must let people out during a fire, even if it means the room is briefly unsecured. Which principle is this?
* Fail safe: people's safety comes first
- Fail secure
- Least privilege
- Defence in depth
? A firewall fails closed, a fire exit fails open.

@@quiz
In which service model is the customer responsible for patching the operating system?
* IaaS
- PaaS
- SaaS
- None
? The provider patches it in PaaS and SaaS.

@@quiz
Which sprinkler type is preferred for data centres because water only enters the pipes after detection?
- Wet pipe
* Pre-action
- Deluge
- Dry pipe only
? Two events are needed before water flows.

@@quiz
What does zero trust mean for a user on the internal office network?
* They are verified for every request, like a user on the internet
- They are trusted because they're inside the firewall
- They need no MFA
- They can reach every server
? Location doesn't grant trust.

@@quiz
An electrical fire in a live server rack. What should NOT be used?
* Water
- CO₂
- A clean agent
- Nothing should be used
? Cut power, then suppress with a non-conductive agent.

@@quiz
What does CPTED's natural surveillance mean?
* Designing spaces so people can be seen, e.g. lighting and clear sight lines
- Installing more cameras
- Employing more guards
- Hiding the building
? Environmental design, not technology.

@@quiz
What is a brownout?
* A prolonged period of low voltage
- A brief spike
- A total loss of power
- A brief loss of power
? A sag is the brief version.

@@flashcards
least privilege :: minimum access, minimum time
defence in depth :: layered controls
fail secure / fail safe :: deny access / protect people
complete mediation :: check every access
open design :: Kerckhoffs: don't rely on secrecy of the design
zero trust :: verify explicitly, least privilege, assume breach
privacy by design :: privacy as the default, built in
shared responsibility :: data, accounts and access are always yours
VM escape :: guest code reaches the hypervisor
VM sprawl :: forgotten, unpatched VMs
Purdue model :: levels for segmenting ICS networks
CPTED :: surveillance, access control, territorial reinforcement
class A / B / C / D / K :: combustibles / liquids / electrical / metals / kitchen
pre-action sprinkler :: detection first, then water
ASHRAE inlet :: about 18–27 °C
sag / brownout :: brief / prolonged low voltage
spike / surge :: brief / prolonged high voltage

@@drill
Which fire class covers flammable liquids?
= B
= class B
? CO₂, foam or dry chemical.

@@drill
In SaaS, is managing user accounts and access the customer's or the provider's responsibility?
= customer
= the customer
= customer's
? Always yours.

@@exercise Check a shared responsibility matrix
A team listed who they think handles each security task for three cloud services. Compare their answers with the standard split for IaaS, PaaS and SaaS and print every mistake, flagging the dangerous ones (where the team assumed the provider does a task that is really theirs).
@@starter
RESPONSIBILITY = {
    "physical security": {"IaaS": "provider", "PaaS": "provider", "SaaS": "provider"},
    "hypervisor": {"IaaS": "provider", "PaaS": "provider", "SaaS": "provider"},
    "OS patching": {"IaaS": "customer", "PaaS": "provider", "SaaS": "provider"},
    "runtime and middleware": {"IaaS": "customer", "PaaS": "provider", "SaaS": "provider"},
    "application code": {"IaaS": "customer", "PaaS": "customer", "SaaS": "provider"},
    "data classification and backups": {"IaaS": "customer", "PaaS": "customer", "SaaS": "customer"},
    "user accounts and access": {"IaaS": "customer", "PaaS": "customer", "SaaS": "customer"},
}

team = {
    "VM fleet (IaaS)": ("IaaS", {"OS patching": "provider", "hypervisor": "provider", "data classification and backups": "customer"}),
    "Web app (PaaS)": ("PaaS", {"runtime and middleware": "customer", "application code": "customer", "user accounts and access": "provider"}),
    "CRM (SaaS)": ("SaaS", {"data classification and backups": "provider", "physical security": "provider", "application code": "provider"}),
}

def review(model, answers):
    raise NotImplementedError

for service, (model, answers) in team.items():
    problems = review(model, answers)
    print(f"{service}: ok" if not problems else f"{service}:")
    for p in problems:
        print("  " + p)
@@expected
VM fleet (IaaS):
  DANGEROUS OS patching: they said provider, it's the customer
Web app (PaaS):
  runtime and middleware: they said customer, it's the provider
  DANGEROUS user accounts and access: they said provider, it's the customer
CRM (SaaS):
  DANGEROUS data classification and backups: they said provider, it's the customer
@@requires
def review
DANGEROUS
@@hint
For each `(task, who)` in the answers, look up `RESPONSIBILITY[task][model]`. If it differs, build a message like `f"{task}: they said {who}, it's the {actual}"`, prefixed with `"DANGEROUS "` when `who == "provider"` and the actual owner is the customer.
@@solution
RESPONSIBILITY = {
    "physical security": {"IaaS": "provider", "PaaS": "provider", "SaaS": "provider"},
    "hypervisor": {"IaaS": "provider", "PaaS": "provider", "SaaS": "provider"},
    "OS patching": {"IaaS": "customer", "PaaS": "provider", "SaaS": "provider"},
    "runtime and middleware": {"IaaS": "customer", "PaaS": "provider", "SaaS": "provider"},
    "application code": {"IaaS": "customer", "PaaS": "customer", "SaaS": "provider"},
    "data classification and backups": {"IaaS": "customer", "PaaS": "customer", "SaaS": "customer"},
    "user accounts and access": {"IaaS": "customer", "PaaS": "customer", "SaaS": "customer"},
}

team = {
    "VM fleet (IaaS)": ("IaaS", {"OS patching": "provider", "hypervisor": "provider", "data classification and backups": "customer"}),
    "Web app (PaaS)": ("PaaS", {"runtime and middleware": "customer", "application code": "customer", "user accounts and access": "provider"}),
    "CRM (SaaS)": ("SaaS", {"data classification and backups": "provider", "physical security": "provider", "application code": "provider"}),
}

def review(model, answers):
    problems = []
    for task, who in answers.items():
        actual = RESPONSIBILITY[task][model]
        if who != actual:
            prefix = "DANGEROUS " if who == "provider" and actual == "customer" else ""
            problems.append(f"{prefix}{task}: they said {who}, it's the {actual}")
    return problems

for service, (model, answers) in team.items():
    problems = review(model, answers)
    print(f"{service}: ok" if not problems else f"{service}:")
    for p in problems:
        print("  " + p)

@@exercise Data centre environment and fire suppression check
Check sensor readings against the recommended inlet range (18–27 °C) and a relative humidity range of 20–60%, classify each power reading (nominal 230 V, ±10% tolerance; a deviation lasting under 1 second is brief, otherwise prolonged), and pick a suppression agent for each fire class.
@@starter
readings = [
    {"room": "DC-1 row A", "temp": 22.5, "rh": 45, "volts": 231, "secs": 0},
    {"room": "DC-1 row B", "temp": 29.0, "rh": 38, "volts": 198, "secs": 0.2},
    {"room": "DC-2 row A", "temp": 17.0, "rh": 15, "volts": 194, "secs": 45},
    {"room": "Comms room", "temp": 24.0, "rh": 72, "volts": 260, "secs": 0.05},
    {"room": "UPS room", "temp": 25.0, "rh": 50, "volts": 0, "secs": 600},
]
AGENTS = {"A": "water or foam", "B": "CO2, foam or dry chemical", "C": "CO2 or clean agent (no water)", "D": "dry powder", "K": "wet chemical"}

def environment(r):
    raise NotImplementedError

def power_event(volts, secs):
    raise NotImplementedError

for r in readings:
    issues = environment(r)
    print(f"{r['room']:<11} {', '.join(issues) if issues else 'ok':<34} power: {power_event(r['volts'], r['secs'])}")
for cls in "ACK":
    print(f"class {cls}: {AGENTS[cls]}")
@@expected
DC-1 row A  ok                                 power: normal
DC-1 row B  too hot                            power: sag
DC-2 row A  too cold, too dry (static)         power: brownout
Comms room  too humid (condensation)           power: spike
UPS room    ok                                 power: blackout
class A: water or foam
class C: CO2 or clean agent (no water)
class K: wet chemical
@@requires
def environment
def power_event
"brownout"
@@hint
`environment`: append "too hot", "too cold", "too dry (static)" or "too humid (condensation)" as needed. `power_event`: 0 V is a "fault" if brief and a "blackout" otherwise; below 207 V is a "sag" or "brownout"; above 253 V is a "spike" or "surge"; otherwise "normal".
@@solution
readings = [
    {"room": "DC-1 row A", "temp": 22.5, "rh": 45, "volts": 231, "secs": 0},
    {"room": "DC-1 row B", "temp": 29.0, "rh": 38, "volts": 198, "secs": 0.2},
    {"room": "DC-2 row A", "temp": 17.0, "rh": 15, "volts": 194, "secs": 45},
    {"room": "Comms room", "temp": 24.0, "rh": 72, "volts": 260, "secs": 0.05},
    {"room": "UPS room", "temp": 25.0, "rh": 50, "volts": 0, "secs": 600},
]
AGENTS = {"A": "water or foam", "B": "CO2, foam or dry chemical", "C": "CO2 or clean agent (no water)", "D": "dry powder", "K": "wet chemical"}

def environment(r):
    issues = []
    if r["temp"] > 27:
        issues.append("too hot")
    elif r["temp"] < 18:
        issues.append("too cold")
    if r["rh"] < 20:
        issues.append("too dry (static)")
    elif r["rh"] > 60:
        issues.append("too humid (condensation)")
    return issues

def power_event(volts, secs):
    brief = secs < 1
    if volts == 0:
        return "fault" if brief else "blackout"
    if volts < 230 * 0.9:
        return "sag" if brief else "brownout"
    if volts > 230 * 1.1:
        return "spike" if brief else "surge"
    return "normal"

for r in readings:
    issues = environment(r)
    print(f"{r['room']:<11} {', '.join(issues) if issues else 'ok':<34} power: {power_event(r['volts'], r['secs'])}")
for cls in "ACK":
    print(f"class {cls}: {AGENTS[cls]}")

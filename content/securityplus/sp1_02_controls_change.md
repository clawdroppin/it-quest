@@lesson sp1-controls Security Controls, Physical Security and Change Management
@@text
Domain 1 (general security concepts, 12%) opens with classifying controls, and the exam loves "which type of control is this?" questions. This lesson builds the full control matrix, then covers physical security, deception technology and the change management process the objectives single out.

## Control categories (what kind of thing it is)

| Category | Examples |
|---|---|
| **Technical** (logical) | firewalls, encryption, MFA, EDR, ACLs |
| **Managerial** (administrative) | policies, risk assessments, security awareness programmes, procedures |
| **Operational** | guards, change management, backups run by people, configuration reviews |
| **Physical** | fences, locks, bollards, cameras, mantraps, lighting |

## Control types (what it does)

| Type | Purpose | Examples |
|---|---|---|
| **Preventive** | stops it happening | firewall rule, door lock, MFA |
| **Deterrent** | discourages attempts | warning banner, visible camera, "guard dog" sign |
| **Detective** | finds it during/after | IDS, log review, camera recordings, audits |
| **Corrective** | fixes it after | restoring from backup, patching, reimaging |
| **Compensating** | alternative when the main control isn't possible | isolate an unpatchable system on its own VLAN |
| **Directive** | tells people what to do | policy, "authorised personnel only" sign, training |

One control can be several types: a camera is detective, and deterrent if visible. Answer with the type that best fits the scenario's purpose.

## Gap analysis

Compare the current state with a target (a framework such as NIST CSF, ISO 27001, CIS Controls) to find missing controls, then plan and prioritise. The output is a list of gaps with owners and dates.

## Physical security

- **Bollards** stop vehicles; **fencing**, **lighting**; **access control vestibules** (mantraps) stop tailgating.
- **Badges/access cards**, **guards**, **video surveillance**, **visitor logs and escorts**.
- **Sensors**: infrared, pressure, microwave, ultrasonic.
- **Locks**: physical, electronic, biometric. **Fail-safe** (unlocks on power loss: life safety) vs **fail-secure** (stays locked: protects assets).
- **Secure areas**: data centres with **cable locks**, **locked racks**, **Faraday cages** (block RF), **air-gapped** systems.

## Deception and disruption technology

- **Honeypot**: a fake system that attracts attackers; any interaction is suspicious.
- **Honeynet**: a network of honeypots.
- **Honeyfile**: a tempting file ("passwords.xlsx") that alerts when opened.
- **Honeytoken**: fake credentials or data (a fake API key, a fake database row) that raise an alert if used anywhere.

These detect attackers who are already inside with very few false positives.

## Change management

Changes are a top cause of outages and security gaps, so Security+ treats change management as a security control.

**Business processes**:
- **Approval process** and **ownership** (who is responsible).
- **Stakeholders** consulted; **impact analysis**.
- **Test results** from a non-production environment.
- **Backout plan** (how to roll back).
- **Maintenance window**.
- **Standard operating procedure** for routine (pre-approved "standard") changes.

**Technical implications**:
- **Allow lists/deny lists** may need updating.
- **Restricted activities**, **downtime**, **service and application restarts**.
- **Legacy applications** and **dependencies** that may break.

**Documentation**: update diagrams, policies and procedures; **version control** for configs and code.

## Other concepts in domain 1

- **Non-repudiation** (digital signatures), **authentication/authorisation/accounting** models.
- **Zero trust**: control plane (adaptive identity, threat scope reduction, **policy engine**, **policy administrator**) and data plane (implicit trust zones, subject/system, **policy enforcement point**).

@@quiz
A company can't patch an old machine controlling a lab instrument, so it isolates it on a separate VLAN with strict rules. What type of control is the isolation?
- Preventive only
* Compensating
- Deterrent
- Directive
? It replaces the control (patching) that isn't possible.

@@quiz
A sign says "This area is monitored by CCTV". What type of control is the sign?
* Deterrent
- Corrective
- Detective
- Technical
? The recordings themselves would be detective.

@@quiz
A data centre door must stay locked during a power failure to protect equipment. Which lock behaviour is this?
- Fail-safe
* Fail-secure
- Fail-open
- Fail-soft
? Fail-safe unlocks for life safety.

@@quiz
A fake AWS key is planted in a code repository; any use triggers an alert. What is it?
- Honeypot
- Honeynet
* Honeytoken
- Sandbox
? Fake credentials or data that alert when used.

@@quiz
In change management, what describes how to return to the previous state if a change fails?
- Impact analysis
* Backout plan
- Maintenance window
- Stakeholder analysis
? Plan it before implementing.

@@flashcards
technical control :: firewall, encryption, MFA
managerial control :: policies, risk assessments
operational control :: guards, change management by people
physical control :: locks, fences, bollards
preventive :: stops an incident
deterrent :: discourages an attempt
detective :: finds an incident
corrective :: fixes after an incident
compensating :: alternative control
directive :: tells people what to do
gap analysis :: current state vs target framework
fail-safe vs fail-secure :: unlocks vs stays locked on failure
access control vestibule :: stops tailgating
honeypot / honeyfile / honeytoken :: fake system / file / credential
backout plan :: how to undo a change
policy enforcement point :: zero trust data plane gatekeeper

@@drill
What type of control is restoring a server from backup after ransomware?
= corrective
? It fixes the damage after the incident.

@@drill
Which physical barrier stops vehicles from approaching a building?
= bollards
= bollard
? Concrete or steel posts.

@@exercise Classify controls
Classify each control by category and type using the keyword tables (first match in each table wins, checking keywords in the order listed). Print `control: category / type`, then a count of controls per type, sorted by type name.
@@lang python
@@starter
CATEGORY = [
    ("policy", "managerial"), ("risk assessment", "managerial"), ("training", "managerial"),
    ("guard", "operational"), ("change review", "operational"),
    ("fence", "physical"), ("lock", "physical"), ("camera", "physical"), ("bollard", "physical"),
    ("firewall", "technical"), ("mfa", "technical"), ("backup restore", "technical"), ("ids", "technical"), ("banner", "technical"),
]
TYPE = [
    ("restore", "corrective"), ("patch", "corrective"),
    ("sign", "deterrent"), ("banner", "deterrent"),
    ("ids", "detective"), ("camera", "detective"), ("log review", "detective"), ("review", "detective"),
    ("policy", "directive"), ("training", "directive"),
    ("firewall", "preventive"), ("mfa", "preventive"), ("lock", "preventive"), ("fence", "preventive"), ("bollard", "preventive"), ("guard", "preventive"),
]
controls = ["Perimeter fence", "Login warning banner", "Network IDS", "Acceptable use policy", "MFA on VPN",
            "Security guard at reception", "Backup restore after ransomware", "Server room door lock", "Weekly change review", "Bollards at entrance"]
@@expected
Perimeter fence: physical / preventive
Login warning banner: technical / deterrent
Network IDS: technical / detective
Acceptable use policy: managerial / directive
MFA on VPN: technical / preventive
Security guard at reception: operational / preventive
Backup restore after ransomware: technical / corrective
Server room door lock: physical / preventive
Weekly change review: operational / detective
Bollards at entrance: physical / preventive
corrective: 1
detective: 2
deterrent: 1
directive: 1
preventive: 5
@@requires
def classify
Counter(
@@hint
Write `classify(text, table)` that returns the label of the first `(keyword, label)` whose keyword is in `text.lower()`, or `"unknown"`.
@@solution
from collections import Counter

CATEGORY = [
    ("policy", "managerial"), ("risk assessment", "managerial"), ("training", "managerial"),
    ("guard", "operational"), ("change review", "operational"),
    ("fence", "physical"), ("lock", "physical"), ("camera", "physical"), ("bollard", "physical"),
    ("firewall", "technical"), ("mfa", "technical"), ("backup restore", "technical"), ("ids", "technical"), ("banner", "technical"),
]
TYPE = [
    ("restore", "corrective"), ("patch", "corrective"),
    ("sign", "deterrent"), ("banner", "deterrent"),
    ("ids", "detective"), ("camera", "detective"), ("log review", "detective"), ("review", "detective"),
    ("policy", "directive"), ("training", "directive"),
    ("firewall", "preventive"), ("mfa", "preventive"), ("lock", "preventive"), ("fence", "preventive"), ("bollard", "preventive"), ("guard", "preventive"),
]
controls = ["Perimeter fence", "Login warning banner", "Network IDS", "Acceptable use policy", "MFA on VPN",
            "Security guard at reception", "Backup restore after ransomware", "Server room door lock", "Weekly change review", "Bollards at entrance"]

def classify(text, table):
    text = text.lower()
    return next((label for keyword, label in table if keyword in text), "unknown")

types = Counter()
for control in controls:
    category = classify(control, CATEGORY)
    kind = classify(control, TYPE)
    types[kind] += 1
    print(f"{control}: {category} / {kind}")
for kind in sorted(types):
    print(f"{kind}: {types[kind]}")

@@exercise Review a change request
A change request must have: an owner, an approver who is not the owner, test results marked `passed`, a backout plan, and a maintenance window inside the allowed hours (22:00–05:00, which crosses midnight). Standard changes (`"type": "standard"`) skip the approver check. Print `APPROVE` or `REJECT` with every reason for each request.
@@lang python
@@starter
requests = [
    {"id": "CHG-101", "type": "normal", "owner": "ana", "approver": "ken", "tests": "passed", "backout": "restore config backup", "window": "23:00"},
    {"id": "CHG-102", "type": "normal", "owner": "ken", "approver": "ken", "tests": "passed", "backout": "", "window": "14:00"},
    {"id": "CHG-103", "type": "standard", "owner": "mei", "approver": "", "tests": "passed", "backout": "uninstall package", "window": "02:30"},
    {"id": "CHG-104", "type": "normal", "owner": "", "approver": "ana", "tests": "failed", "backout": "revert VM snapshot", "window": "04:00"},
]

def in_window(hhmm):
    raise NotImplementedError
@@expected
CHG-101: APPROVE
CHG-102: REJECT: owner approved their own change; no backout plan; 14:00 is outside the maintenance window
CHG-103: APPROVE
CHG-104: REJECT: no owner; tests failed
@@requires
def in_window
>= 22
@@hint
`hour = int(hhmm[:2])`; the window crosses midnight, so it's `hour >= 22 or hour < 5`.
@@solution
requests = [
    {"id": "CHG-101", "type": "normal", "owner": "ana", "approver": "ken", "tests": "passed", "backout": "restore config backup", "window": "23:00"},
    {"id": "CHG-102", "type": "normal", "owner": "ken", "approver": "ken", "tests": "passed", "backout": "", "window": "14:00"},
    {"id": "CHG-103", "type": "standard", "owner": "mei", "approver": "", "tests": "passed", "backout": "uninstall package", "window": "02:30"},
    {"id": "CHG-104", "type": "normal", "owner": "", "approver": "ana", "tests": "failed", "backout": "revert VM snapshot", "window": "04:00"},
]

def in_window(hhmm):
    hour = int(hhmm[:2])
    return hour >= 22 or hour < 5

for r in requests:
    reasons = []
    if not r["owner"]:
        reasons.append("no owner")
    if r["type"] != "standard":
        if not r["approver"]:
            reasons.append("no approver")
        elif r["approver"] == r["owner"]:
            reasons.append("owner approved their own change")
    if r["tests"] != "passed":
        reasons.append(f"tests {r['tests']}")
    if not r["backout"]:
        reasons.append("no backout plan")
    if not in_window(r["window"]):
        reasons.append(f"{r['window']} is outside the maintenance window")
    print(f"{r['id']}: " + ("REJECT: " + "; ".join(reasons) if reasons else "APPROVE"))

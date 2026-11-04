@@lesson sp4-tools Security Operations Tools: SIEM, EDR, Hardening and Automation
@@text
Day-to-day security operations means hardening systems against a baseline, watching them with a SIEM and EDR, managing assets from purchase to disposal, and automating repetitive work. This lesson covers the domain 4 objectives on monitoring, asset management, hardening and automation.

## Secure baselines and hardening targets

A **secure baseline** is the approved configuration for a system type, often from **CIS Benchmarks** or vendor guides (Microsoft Security Baselines, DISA STIGs). Life cycle: **establish**, **deploy** (Group Policy, Intune, Ansible), **maintain** (update as threats change, detect drift).

Hardening targets the objectives list: **mobile devices**, **workstations**, **switches**, **routers**, **cloud infrastructure**, **servers**, **ICS/SCADA**, **embedded systems**, **RTOS**, **IoT devices**. Common steps: remove unneeded software and services, close ports, change defaults, enable host firewall and EDR, enforce encryption, patch, log centrally.

## Wireless and mobile

- **Site surveys** and **heat maps** for coverage and rogue APs.
- **WPA3**, **RADIUS/802.1X** (EAP-TLS), **AAA**.
- **MDM**: enforce passcodes, encryption, remote wipe, app restrictions. Deployment models: **BYOD**, **COPE** (corporate-owned, personally enabled), **CYOD** (choose your own device). Connection methods: cellular, Wi-Fi, Bluetooth (pairing risks).

## Asset management

- **Acquisition/procurement** with security requirements.
- **Assignment/accounting**: **ownership** and **classification** of each asset.
- **Monitoring/asset tracking**: **inventory** and **enumeration** (you can't protect what you don't know about).
- **Disposal/decommissioning**: **sanitisation** (wipe), **destruction**, **certification** of destruction, **data retention** requirements.

## Monitoring and alerting

| Tool | Does |
|---|---|
| **SIEM** | collects logs from everything, normalises, **correlates**, alerts, dashboards, retention for compliance |
| **SOAR** | automates response playbooks (enrich an alert, block an IP, disable an account) |
| **EDR** | records endpoint behaviour (processes, files, network), detects and responds (isolate host, kill process) |
| **XDR** | extends EDR across email, identity, cloud and network |
| **UEBA** | baselines user behaviour, flags anomalies |
| **SCAP** | standard format for checking configurations against baselines |
| **NetFlow**, **vulnerability scanners**, **antivirus**, **DLP**, **SNMP traps** | other data sources |
| **Agent vs agentless** | software on the host vs remote checks |

Activities: **log aggregation**, **alerting**, **scanning**, **reporting**, **archiving**, **alert response and remediation/validation** (**quarantine**, **alert tuning** to cut false positives).

## Automation and orchestration

Use cases: **user provisioning**, **resource provisioning**, **guard rails** (prevent insecure configurations), **security groups**, **ticket creation**, **escalation**, **enabling/disabling services and access**, **continuous integration and testing**, **integrations and APIs**.

Benefits: efficiency, **enforcing baselines**, **standard infrastructure configurations**, **scaling securely**, **employee retention** (less tedious work), **reaction time**, **workforce multiplier**.

Considerations: **complexity**, **cost**, **single point of failure**, **technical debt**, **ongoing supportability**.

## Other operational controls

- **File integrity monitoring (FIM)**: hashes important files and alerts on changes.
- **Application allow listing**, **sandboxing** suspicious files.
- **Web filtering**, **DNS filtering**.
- **Group Policy** and **SELinux** for OS-level enforcement.
- **Secure protocols** and **port selection**.

@@quiz
Which tool collects logs from many sources, correlates events and raises alerts?
* SIEM
- EDR
- DLP
- HSM
? SOAR then automates the response.

@@quiz
An alert automatically triggers a playbook that enriches it with threat intel, disables the user and opens a ticket. Which tool does this?
- SIEM
* SOAR
- NAC
- FIM
? Security orchestration, automation and response.

@@quiz
Which deployment model has the company own the phone but let employees use it personally?
- BYOD
* COPE
- CYOD
- VDI
? Corporate-owned, personally enabled.

@@quiz
Which is a risk of heavy security automation?
- Faster reaction time
- Enforced baselines
* It can become a single point of failure and add technical debt
- Workforce multiplier
? Automation needs ongoing support.

@@quiz
Which source gives a standard, published configuration checklist for hardening Windows Server?
- A SOAR playbook
* CIS Benchmark
- CVSS
- An SLA
? Also Microsoft Security Baselines and DISA STIGs.

@@flashcards
secure baseline :: approved hardened configuration
CIS Benchmarks :: published hardening guides
STIG :: DISA hardening guides
SIEM :: log collection, correlation, alerting
SOAR :: automated response playbooks
EDR :: endpoint detection and response
XDR :: EDR across email, identity, cloud
UEBA :: user behaviour anomalies
SCAP :: standard config checking
alert tuning :: reduce false positives
BYOD / COPE / CYOD :: personal / corporate-owned / choose from a list
asset inventory :: you can't protect what you don't know
sanitisation :: wiping media before reuse
FIM :: alerts on changed files
guard rails :: automation that blocks insecure configs
technical debt :: future cost of shortcuts

@@drill
Which tool type automates response playbooks after an alert?
= SOAR
= soar
? Security orchestration, automation and response.

@@drill
What does the E in EDR stand for?
= Endpoint
= endpoint
? Endpoint detection and response.

@@exercise Correlate SIEM events
Write a correlation rule: alert when one user has **5 or more failed logins** followed by a **successful login** within 10 minutes of the first failure, and alert separately on any successful login from a country not in the user's `usual` set. Print the alerts in time order.
@@lang python
@@starter
usual = {"ana": {"JP"}, "ken": {"JP", "US"}}
events = [
    (0, "ken", "fail", "JP"), (1, "ana", "fail", "JP"), (2, "ana", "fail", "JP"), (3, "ana", "fail", "JP"),
    (4, "ana", "fail", "JP"), (5, "ana", "fail", "JP"), (6, "ana", "success", "JP"),
    (20, "ken", "success", "US"), (31, "ana", "success", "RU"),
    (40, "ken", "fail", "JP"), (41, "ken", "fail", "JP"), (55, "ken", "fail", "JP"), (56, "ken", "fail", "JP"), (57, "ken", "fail", "JP"), (58, "ken", "success", "JP"),
]
@@expected
t=6: ALERT ana: 5 failures then success (possible password guessing)
t=31: ALERT ana: login from unusual country RU
@@requires
fails[
usual[
@@hint
Keep `fails = {user: [minutes]}`. On success, count failures within the last 10 minutes (`t - f <= 10`), then clear that user's list.
@@solution
usual = {"ana": {"JP"}, "ken": {"JP", "US"}}
events = [
    (0, "ken", "fail", "JP"), (1, "ana", "fail", "JP"), (2, "ana", "fail", "JP"), (3, "ana", "fail", "JP"),
    (4, "ana", "fail", "JP"), (5, "ana", "fail", "JP"), (6, "ana", "success", "JP"),
    (20, "ken", "success", "US"), (31, "ana", "success", "RU"),
    (40, "ken", "fail", "JP"), (41, "ken", "fail", "JP"), (55, "ken", "fail", "JP"), (56, "ken", "fail", "JP"), (57, "ken", "fail", "JP"), (58, "ken", "success", "JP"),
]

fails = {}
for t, user, result, country in sorted(events):
    if result == "fail":
        fails.setdefault(user, []).append(t)
        continue
    recent = [f for f in fails.get(user, []) if t - f <= 10]
    if len(recent) >= 5:
        print(f"t={t}: ALERT {user}: {len(recent)} failures then success (possible password guessing)")
    fails[user] = []
    if country not in usual[user]:
        print(f"t={t}: ALERT {user}: login from unusual country {country}")

@@exercise Check hosts against a secure baseline
Compare each host's settings with the baseline and compute a compliance percentage. Settings missing from a host count as non-compliant. Print each host's percentage and its drifted settings (`setting: actual (want expected)`), then the fleet's average compliance.
@@lang python
@@starter
BASELINE = {
    "smbv1": "disabled",
    "firewall": "on",
    "bitlocker": "on",
    "rdp_nla": "required",
    "guest": "disabled",
    "audit_logon": "success,failure",
    "edr": "running",
    "autorun": "disabled",
}
hosts = {
    "srv-01": {"smbv1": "disabled", "firewall": "on", "bitlocker": "on", "rdp_nla": "required", "guest": "disabled", "audit_logon": "success,failure", "edr": "running", "autorun": "disabled"},
    "srv-02": {"smbv1": "enabled", "firewall": "on", "bitlocker": "off", "rdp_nla": "required", "guest": "disabled", "audit_logon": "failure", "edr": "running"},
    "pc-117": {"smbv1": "disabled", "firewall": "off", "bitlocker": "on", "rdp_nla": "required", "guest": "disabled", "audit_logon": "success,failure", "edr": "stopped", "autorun": "disabled"},
}
@@expected
srv-01: 100% compliant
srv-02: 50% compliant
  smbv1: enabled (want disabled)
  bitlocker: off (want on)
  audit_logon: failure (want success,failure)
  autorun: missing (want disabled)
pc-117: 75% compliant
  firewall: off (want on)
  edr: stopped (want running)
Fleet average: 75%
@@requires
BASELINE.items()
.get(
@@hint
`actual = settings.get(key, "missing")`; compliance = matches ÷ `len(BASELINE)`.
@@solution
BASELINE = {
    "smbv1": "disabled",
    "firewall": "on",
    "bitlocker": "on",
    "rdp_nla": "required",
    "guest": "disabled",
    "audit_logon": "success,failure",
    "edr": "running",
    "autorun": "disabled",
}
hosts = {
    "srv-01": {"smbv1": "disabled", "firewall": "on", "bitlocker": "on", "rdp_nla": "required", "guest": "disabled", "audit_logon": "success,failure", "edr": "running", "autorun": "disabled"},
    "srv-02": {"smbv1": "enabled", "firewall": "on", "bitlocker": "off", "rdp_nla": "required", "guest": "disabled", "audit_logon": "failure", "edr": "running"},
    "pc-117": {"smbv1": "disabled", "firewall": "off", "bitlocker": "on", "rdp_nla": "required", "guest": "disabled", "audit_logon": "success,failure", "edr": "stopped", "autorun": "disabled"},
}

scores = []
for host, settings in hosts.items():
    drift = []
    for key, expected in BASELINE.items():
        actual = settings.get(key, "missing")
        if actual != expected:
            drift.append(f"{key}: {actual} (want {expected})")
    score = (len(BASELINE) - len(drift)) / len(BASELINE)
    scores.append(score)
    print(f"{host}: {score:.0%} compliant")
    for d in drift:
        print(f"  {d}")
print(f"Fleet average: {sum(scores) / len(scores):.0%}")

@@lesson sp4-operations Security Operations, IAM and Incident Response
@@text
Security operations is the biggest domain (28%), and it's what a SOC analyst does every day.

## Hardening and secure baselines

Establish a **baseline** (a standard secure configuration, e.g. **CIS Benchmarks**), deploy it, and keep checking for drift. Harden everything: workstations, servers, switches, routers, cloud, **mobile** (MDM, **COPE**/**CYOD**/**BYOD** policies), wireless (WPA3, RADIUS), applications (input validation, secure cookies, **code signing**, **sandboxing**).

**Asset management**: you can't protect what you don't know about. Inventory, ownership, classification, and **secure disposal** (sanitization, destruction, certification, retention requirements).

## Vulnerability management

1. **Identify**: vulnerability **scans** (credentialed are deeper than non-credentialed), **penetration tests**, **bug bounties**, threat feeds (OSINT, ISACs), **SCA** for libraries.
2. **Analyse**: confirm (watch for **false positives**), prioritize by **CVSS** score (0–10), **CVE** IDs, exposure, environment, and risk tolerance.
3. **Respond**: patch, **insurance**, **segmentation**, **compensating controls**, or accept an **exception** formally.
4. **Validate**: rescan, audit, verify. **Report.**

## Monitoring

- Sources: logs from systems, apps, firewalls, IPS, endpoints; **NetFlow**; **vulnerability scans**; **packet captures**.
- **SIEM** collects and correlates logs and raises **alerts**; **SOAR** automates responses with **playbooks**.
- **EDR/XDR** watch endpoints' behaviour and can isolate a host. **UBA** (user behaviour analytics) finds unusual activity.
- **Alert tuning** reduces false positives; **archiving** keeps logs for investigations and compliance.
- Others: **DNS filtering**, **email security** (SPF, DKIM, **DMARC**, secure gateways), **FIM** (file integrity monitoring), **SNMP traps**, **NAC**.

## Identity and access management

- **Provisioning/deprovisioning** accounts (disable on the employee's last day!), **permission assignments**, **identity proofing**.
- **SSO** with **SAML** (enterprise web SSO, XML), **OAuth** (authorization: "let this app use my calendar"), **OpenID Connect** (authentication on top of OAuth), **LDAP** (directory), **federation** (trust between organizations).
- Access models: **RBAC** (by role), **ABAC** (by attributes: department, time, location), **MAC** (mandatory: labels like secret, set by the system), **DAC** (discretionary: the owner decides), **rule-based**, **time-of-day** restrictions.
- **MFA**: hardware tokens (FIDO2/WebAuthn **passkeys**: phishing-resistant), authenticator apps (TOTP), push notifications (watch for **MFA fatigue**), SMS (weakest).
- **PAM** (privileged access management): **just-in-time** permissions, **password vaulting**, **ephemeral credentials**.
- Password best practice (NIST): length over complexity, check against breached passwords, no forced periodic changes without cause, password managers.

## Incident response

The process: **Preparation → Detection → Analysis → Containment → Eradication → Recovery → Lessons learned**.

- **Containment** first limits damage (isolate the host, disable the account), *before* you remove the cause.
- **Digital forensics**: **legal hold**, **chain of custody**, **acquisition** in **order of volatility** (CPU cache and RAM first, then disk, then backups), preservation (hash images to prove integrity), **e-discovery**, reporting.
- **Root cause analysis**, **threat hunting** (proactively searching for hidden attackers), and testing the plan (tabletop, simulation).

## Automation

Scripts and orchestration for user provisioning, guard rails, ticket creation, escalation, enabling/disabling services and access, CI/CD security checks, and API integrations: faster, consistent, a **force multiplier** for small teams. Risks: complexity, cost, a **single point of failure**, technical debt.

@@quiz
A SOC analyst wants one place that collects logs from firewalls, servers and endpoints, correlates them, and raises alerts. What tool?
* SIEM
- WAF
- DLP
- PAM
? Security information and event management. SOAR then automates the response.

@@quiz
A workstation is confirmed infected with ransomware that's spreading. What's the immediate next phase?
* Containment: isolate the workstation from the network
- Eradication: reimage it
- Lessons learned
- Preparation
? Stop the spread first, then remove the cause.

@@quiz
A forensic investigator is collecting evidence from a running server. What should be captured first?
* The contents of RAM
- The hard disk
- Backup tapes
- Printed logs
? Order of volatility: the most volatile data disappears first.

@@quiz
Users get repeated push notifications asking them to approve a login they didn't start, until one finally taps "Approve". What attack is this?
* MFA fatigue
- Credential stuffing
- SIM swapping
- Pass the hash
? Use number matching or phishing-resistant MFA like FIDO2 keys.

@@quiz
"Allow this app to read your Google calendar" without sharing your password uses which standard?
* OAuth
- SAML
- LDAP
- RADIUS
? OAuth is delegated authorization; OpenID Connect adds authentication on top.

@@quiz
Access to a finance system depends on the user's department, the device being company-managed, and office hours. Which access model?
* ABAC (attribute-based access control)
- DAC
- MAC
- RBAC only
? Several attributes evaluated together.

@@quiz
Admins get domain admin rights only for 2 hours when they request them for a specific change. What is this?
* Just-in-time privileged access (PAM)
- Separation of duties
- Federation
- Password spraying
? Standing privileges are what attackers hunt for.

@@quiz
A vulnerability scanner reports a critical flaw in a service that isn't installed on the server. What is this?
* A false positive
- A false negative
- A zero-day
- A true positive
? Validate findings before acting; credentialed scans reduce false positives.

@@quiz
Which scoring system rates vulnerability severity from 0 to 10?
* CVSS
- CVE
- CIS
- SCAP
? CVE is the identifier (CVE-2024-3094); CVSS is the score.

@@flashcards
CIS Benchmarks :: Secure configuration baselines
credentialed scan :: Vulnerability scan that logs in; deeper, fewer false positives
CVE / CVSS :: Vulnerability ID / severity score 0-10
false positive :: Alert for something that isn't a problem
SIEM :: Collects and correlates logs, raises alerts
SOAR :: Automates responses with playbooks
EDR / XDR :: Endpoint / extended detection and response
FIM :: File integrity monitoring
SAML :: XML-based enterprise SSO
OAuth :: Delegated authorization
OpenID Connect :: Authentication on top of OAuth
RBAC / ABAC :: Access by role / by attributes
MAC / DAC :: Labels set by the system / owner decides
FIDO2 / passkeys :: Phishing-resistant MFA
MFA fatigue :: Spamming push prompts until approved
PAM :: Privileged access management: JIT, vaulting
incident response :: Preparation, detection, analysis, containment, eradication, recovery, lessons learned
order of volatility :: Collect the most volatile evidence first (RAM before disk)
chain of custody :: Record of who handled evidence, when
legal hold :: Preserve data for litigation; don't delete
threat hunting :: Proactively searching for undetected attackers

@@drill
What is the last phase of the incident response process? (two words)
= lessons learned
? Improve the plan and controls so it doesn't happen again.

@@exercise Run a SOAR playbook on alerts
A SOAR platform runs a playbook for each SIEM alert. Write `run_playbook(alert)` that returns the list of actions taken, in order:

1. Every alert: `enrich: look up <host or user> in the CMDB`.
2. `malware` with severity `high` or `critical`: `isolate host <host>`, then `open P1 ticket`.
3. `malware` lower than high: `run full scan on <host>`.
4. `impossible travel`: `disable account <user>`, `revoke sessions for <user>`, `open P2 ticket`.
5. `phishing reported`: `remove email from all mailboxes`, `block sender domain <domain>`.
6. Anything else: `send to analyst queue`.
7. Every `critical` alert also gets `page the on-call analyst` at the end.

Print each alert and its actions, then how many alerts needed a human.
@@lang python
@@starter
alerts = [
    {"id": 101, "type": "malware", "severity": "critical", "host": "fin-laptop-07"},
    {"id": 102, "type": "malware", "severity": "low", "host": "kiosk-02"},
    {"id": 103, "type": "impossible travel", "severity": "high", "user": "ksato"},
    {"id": 104, "type": "phishing reported", "severity": "medium", "user": "aiko", "domain": "examp1e-pay.com"},
    {"id": 105, "type": "dns tunnelling", "severity": "critical", "host": "web03"},
]
SEVERE = {"high", "critical"}

def run_playbook(alert):
    raise NotImplementedError


human = 0
for a in alerts:
    actions = run_playbook(a)
    if any("analyst" in step for step in actions):
        human += 1
    print(f"#{a['id']} {a['type']} ({a['severity']}):")
    for step in actions:
        print("  - " + step)
print(f"{human} of {len(alerts)} alerts needed a human")
@@expected
#101 malware (critical):
  - enrich: look up fin-laptop-07 in the CMDB
  - isolate host fin-laptop-07
  - open P1 ticket
  - page the on-call analyst
#102 malware (low):
  - enrich: look up kiosk-02 in the CMDB
  - run full scan on kiosk-02
#103 impossible travel (high):
  - enrich: look up ksato in the CMDB
  - disable account ksato
  - revoke sessions for ksato
  - open P2 ticket
#104 phishing reported (medium):
  - enrich: look up aiko in the CMDB
  - remove email from all mailboxes
  - block sender domain examp1e-pay.com
#105 dns tunnelling (critical):
  - enrich: look up web03 in the CMDB
  - send to analyst queue
  - page the on-call analyst
2 of 5 alerts needed a human
@@requires
def run_playbook
elif
@@hint
Start the list with the enrichment step, add one branch's steps with an `if`/`elif` chain, and append the paging step after the chain so it applies to every critical alert.
@@solution
alerts = [
    {"id": 101, "type": "malware", "severity": "critical", "host": "fin-laptop-07"},
    {"id": 102, "type": "malware", "severity": "low", "host": "kiosk-02"},
    {"id": 103, "type": "impossible travel", "severity": "high", "user": "ksato"},
    {"id": 104, "type": "phishing reported", "severity": "medium", "user": "aiko", "domain": "examp1e-pay.com"},
    {"id": 105, "type": "dns tunnelling", "severity": "critical", "host": "web03"},
]
SEVERE = {"high", "critical"}

def run_playbook(alert):
    subject = alert.get("host") or alert.get("user")
    actions = [f"enrich: look up {subject} in the CMDB"]
    kind = alert["type"]
    if kind == "malware" and alert["severity"] in SEVERE:
        actions += [f"isolate host {alert['host']}", "open P1 ticket"]
    elif kind == "malware":
        actions.append(f"run full scan on {alert['host']}")
    elif kind == "impossible travel":
        user = alert["user"]
        actions += [f"disable account {user}", f"revoke sessions for {user}", "open P2 ticket"]
    elif kind == "phishing reported":
        actions += ["remove email from all mailboxes", f"block sender domain {alert['domain']}"]
    else:
        actions.append("send to analyst queue")
    if alert["severity"] == "critical":
        actions.append("page the on-call analyst")
    return actions

human = 0
for a in alerts:
    actions = run_playbook(a)
    if any("analyst" in step for step in actions):
        human += 1
    print(f"#{a['id']} {a['type']} ({a['severity']}):")
    for step in actions:
        print("  - " + step)
print(f"{human} of {len(alerts)} alerts needed a human")

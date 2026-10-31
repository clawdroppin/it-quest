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

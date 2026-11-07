@@lesson sp5-governance Governance, Risk and Compliance
@@text
## Governance

- **Policies** (high-level rules: acceptable use, information security, business continuity, incident response, SDLC, change management), **standards** (specific requirements: password length, encryption), **procedures** (step-by-step: onboarding, offboarding, playbooks) and **guidelines** (recommendations).
- Governance structures: boards, committees, government entities; centralized vs decentralized.
- **Roles for data**: **owner** (accountable, decides classification), **controller** (decides why and how personal data is processed), **processor** (processes it for the controller), **custodian/steward** (looks after it day to day).

## Risk management

- **Identify** risks, **assess** them (**qualitative**: high/medium/low; **quantitative**: money), keep a **risk register** with owners, thresholds and **key risk indicators**.
- Quantitative terms: **SLE** (single loss expectancy) = asset value × exposure factor; **ARO** (annualized rate of occurrence); **ALE** = SLE × ARO.
- **Risk appetite** (how much risk the organization wants: expansionary, conservative, neutral) and **tolerance**.
- Responses: **mitigate** (add controls), **transfer** (insurance, outsourcing), **avoid** (stop the activity), **accept** (with an **exception** or **exemption**, documented).
- **BIA** (business impact analysis): **RTO**, **RPO**, **MTTR**, **MTBF**.

## Third parties

Vendor assessment (penetration tests, **right-to-audit** clauses, independent assessments, supply chain analysis), due diligence, conflict of interest. Agreements: **SLA** (service levels), **MOA/MOU** (memorandum of agreement/understanding), **MSA** (master service agreement), **SOW** (statement of work), **NDA**, **BPA** (business partners agreement). Rules of engagement for testers.

## Compliance and privacy

- Reporting (internal and external), consequences of non-compliance (fines, sanctions, reputational damage, loss of licence, contractual impacts), **attestation**, monitoring.
- Laws and standards you'll hear: **GDPR** (EU), Japan's **APPI** (Act on the Protection of Personal Information), **PCI DSS** (card payments), **HIPAA** (US health), **SOX** (US financial reporting), **ISO/IEC 27001** (information security management), **NIST** frameworks.
- Privacy: data subject rights, **right to be forgotten**, data inventory and retention.

## Audits and assessments

Internal (compliance, audit committee, self-assessments) and external (regulators, examinations, independent third-party audits). **Penetration testing**: **known** environment (white box: full information), **partially known** (grey box), **unknown** (black box); offensive (**red team**), defensive (**blue team**), integrated (**purple team**); passive and active **reconnaissance**.

## Security awareness

Phishing campaigns and recognition, **anomalous behaviour** recognition (risky, unexpected or unintentional), user guidance (policies, situational awareness, insider threat, password management, removable media, social engineering, operational security, hybrid/remote work), and reporting and monitoring of the program's effectiveness.

@@quiz
A stolen laptop is a total loss: ¥200,000 (asset value ¥200,000 × exposure factor 1.0). The company expects 3 thefts a year. What is the ALE?
* ¥600,000
- ¥200,000
- ¥100,000
- ¥66,667
? ALE = SLE × ARO = 200,000 × 3.

@@quiz
A company buys cyber insurance to cover ransomware losses. Which risk response is this?
* Transfer
- Mitigate
- Avoid
- Accept
? The financial impact moves to the insurer.

@@quiz
Which document describes step-by-step how to onboard a new employee's accounts?
* A procedure
- A policy
- A guideline
- A standard
? Policies say what and why; procedures say exactly how.

@@quiz
A penetration tester is given network diagrams, source code and credentials before testing. What kind of test is this?
* Known environment (white box)
- Unknown environment (black box)
- Partially known environment
- Red team only
? Full information lets testers go deep quickly.

@@quiz
Under GDPR, which role decides why and how personal data is processed?
* Data controller
- Data processor
- Data custodian
- Data subject
? Processors act on the controller's instructions.

@@quiz
Which agreement defines measurable service levels, like 99.9% uptime and a 4-hour response time?
* SLA
- NDA
- MOU
- BPA
? Service level agreement.

@@quiz
Which international standard specifies an information security management system (ISMS)?
* ISO/IEC 27001
- PCI DSS
- HIPAA
- SOX
? Many Japanese companies hold ISO 27001 (ISMS) certification.

@@flashcards
policy / standard / procedure / guideline :: Rule / specific requirement / how-to steps / recommendation
data owner :: Accountable for the data and its classification
data controller / processor :: Decides how personal data is processed / processes it for them
SLE :: Asset value × exposure factor
ARO :: Expected occurrences per year
ALE :: SLE × ARO
risk responses :: Mitigate, transfer, avoid, accept
risk register :: List of risks, owners, ratings, responses
BIA :: Business impact analysis: RTO, RPO, MTTR, MTBF
SLA :: Measurable service levels
MOU :: Memorandum of understanding (not legally binding)
SOW :: Statement of work for a project
right-to-audit clause :: Lets you audit a vendor
known / partially known / unknown environment :: White / grey / black box testing
red / blue / purple team :: Attack / defend / work together
APPI :: Japan's personal information protection law
ISO 27001 :: Information security management system standard

@@drill
What do you multiply SLE by to get ALE? (abbreviation)
= ARO
? Annualized rate of occurrence.

@@exercise Track risk acceptance exceptions
When a risk is **accepted**, the lesson says to document it as an exception. Write `review(exceptions, today)` that checks each one and returns its status:

- `invalid: no owner` if it has no risk owner, or `invalid: no compensating control` for a `high` risk without one;
- `expired` if `expires` is before today (the risk is now unmanaged);
- `renew soon` if it expires within 30 days;
- otherwise `active`.

High risks may be accepted for at most 180 days: also flag `too long for a high risk` when `expires - approved` is longer. Print each exception and finish with the count per status.
@@lang python
@@starter
from datetime import date

exceptions = [
    {"id": "EX-01", "risk": "Legacy XP machine in the lab", "level": "high", "owner": "Lab manager",
     "control": "isolated VLAN, no internet", "approved": date(2026, 6, 1), "expires": date(2026, 11, 20)},
    {"id": "EX-02", "risk": "Vendor portal without SSO", "level": "medium", "owner": "IT manager",
     "control": "", "approved": date(2026, 1, 10), "expires": date(2027, 1, 10)},
    {"id": "EX-03", "risk": "Unpatched print server", "level": "high", "owner": "",
     "control": "ACL to admin subnet", "approved": date(2026, 9, 1), "expires": date(2027, 1, 1)},
    {"id": "EX-04", "risk": "TLS 1.1 on old API", "level": "high", "owner": "API lead",
     "control": "", "approved": date(2026, 8, 1), "expires": date(2026, 12, 31)},
    {"id": "EX-05", "risk": "Shared kiosk account", "level": "low", "owner": "Store manager",
     "control": "kiosk mode", "approved": date(2025, 10, 1), "expires": date(2026, 10, 1)},
    {"id": "EX-06", "risk": "Old CMS plugin", "level": "high", "owner": "Web lead",
     "control": "WAF virtual patch", "approved": date(2026, 2, 1), "expires": date(2027, 2, 1)},
]

def review(exc, today):
    raise NotImplementedError


today = date(2026, 11, 7)
counts = {}
for e in exceptions:
    flags = review(e, today)
    counts[flags[0]] = counts.get(flags[0], 0) + 1
    print(f"{e['id']} {e['risk']}: {'; '.join(flags)}")
print(", ".join(f"{n} {status}" for status, n in sorted(counts.items())))
@@expected
EX-01 Legacy XP machine in the lab: renew soon
EX-02 Vendor portal without SSO: active
EX-03 Unpatched print server: invalid: no owner
EX-04 TLS 1.1 on old API: invalid: no compensating control
EX-05 Shared kiosk account: expired
EX-06 Old CMS plugin: active; too long for a high risk
2 active, 1 expired, 1 invalid: no compensating control, 1 invalid: no owner, 1 renew soon
@@requires
def review
.days
@@hint
Subtracting two `date` objects gives a `timedelta`; its `.days` is the number of days between them.
@@solution
from datetime import date

exceptions = [
    {"id": "EX-01", "risk": "Legacy XP machine in the lab", "level": "high", "owner": "Lab manager",
     "control": "isolated VLAN, no internet", "approved": date(2026, 6, 1), "expires": date(2026, 11, 20)},
    {"id": "EX-02", "risk": "Vendor portal without SSO", "level": "medium", "owner": "IT manager",
     "control": "", "approved": date(2026, 1, 10), "expires": date(2027, 1, 10)},
    {"id": "EX-03", "risk": "Unpatched print server", "level": "high", "owner": "",
     "control": "ACL to admin subnet", "approved": date(2026, 9, 1), "expires": date(2027, 1, 1)},
    {"id": "EX-04", "risk": "TLS 1.1 on old API", "level": "high", "owner": "API lead",
     "control": "", "approved": date(2026, 8, 1), "expires": date(2026, 12, 31)},
    {"id": "EX-05", "risk": "Shared kiosk account", "level": "low", "owner": "Store manager",
     "control": "kiosk mode", "approved": date(2025, 10, 1), "expires": date(2026, 10, 1)},
    {"id": "EX-06", "risk": "Old CMS plugin", "level": "high", "owner": "Web lead",
     "control": "WAF virtual patch", "approved": date(2026, 2, 1), "expires": date(2027, 2, 1)},
]

def review(exc, today):
    flags = []
    if not exc["owner"]:
        return ["invalid: no owner"]
    if exc["level"] == "high" and not exc["control"]:
        return ["invalid: no compensating control"]
    if exc["expires"] < today:
        flags.append("expired")
    elif (exc["expires"] - today).days <= 30:
        flags.append("renew soon")
    else:
        flags.append("active")
    if exc["level"] == "high" and (exc["expires"] - exc["approved"]).days > 180:
        flags.append("too long for a high risk")
    return flags

today = date(2026, 11, 7)
counts = {}
for e in exceptions:
    flags = review(e, today)
    counts[flags[0]] = counts.get(flags[0], 0) + 1
    print(f"{e['id']} {e['risk']}: {'; '.join(flags)}")
print(", ".join(f"{n} {status}" for status, n in sorted(counts.items())))

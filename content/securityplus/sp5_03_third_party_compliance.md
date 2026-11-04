@@lesson sp5-thirdparty Third-Party Risk, Agreements, Compliance and Audits
@@text
Few organisations run everything themselves: suppliers, cloud providers, MSPs and contractors all touch company data. Domain 5 covers assessing those third parties, the agreements that govern them, compliance and privacy obligations, and the audits and assessments that prove controls work.

## Vendor assessment

- **Penetration testing** of the vendor (or reviewing their reports).
- **Right-to-audit clause** in the contract.
- **Evidence of internal audits**; independent assessments such as **SOC 2 Type II** reports or **ISO 27001** certification.
- **Independent assessments**, **supply chain analysis** (their suppliers, hardware and software provenance).
- **Due diligence** before signing; **conflict of interest** checks.
- **Questionnaires** (security questionnaires, standard ones such as SIG or CAIQ).
- **Rules of engagement** for any testing.
- **Vendor monitoring** after onboarding, not only once: breaches in the news, certification expiry, changes in ownership.

## Agreement types

| Agreement | Covers |
|---|---|
| **SLA** (service-level agreement) | measurable service levels: uptime, response times, penalties |
| **MOA / MOU** (memorandum of agreement/understanding) | intent and responsibilities between parties; MOU is often non-binding |
| **MSA** (master service agreement) | the overall terms for future work |
| **SOW / WO** (statement of work / work order) | a specific project's deliverables, timeline and cost under the MSA |
| **NDA** (non-disclosure agreement) | confidentiality |
| **BPA** (business partners agreement) | how partners share profits, responsibilities and decisions |
| **DPA** (data processing agreement) | how a processor handles personal data for a controller (required by GDPR) |

## Compliance

- **Compliance reporting**: **internal** and **external** (to regulators, customers).
- **Consequences of non-compliance**: **fines**, **sanctions**, **reputational damage**, **loss of licence**, **contractual impacts**.
- **Compliance monitoring**: **due diligence/care**, **attestation and acknowledgement** (staff sign that they read policies), **internal and external** checks, **automation**.
- Regulations and standards to recognise: **GDPR** (EU personal data), **PCI DSS** (card payments), **HIPAA** (US health), **SOX** (US financial reporting), Japan's **APPI** (Act on the Protection of Personal Information), **ISO 27001/27002**, **NIST CSF**, **CIS Controls**.

## Privacy

- **Legal implications**: **local/regional**, **national**, **global** laws.
- **Data subject**: the person the data is about, with rights (access, correction, deletion: the **right to be forgotten**).
- **Controller** decides why and how data is processed; **processor** processes it on the controller's behalf (a payroll SaaS is a processor for the employer, the controller).
- **Ownership** and **data inventory and retention**: know what personal data you have, where, why, and delete it when no longer needed.
- **Data sovereignty**: data is subject to the laws of the country where it's stored.

## Audits and assessments

- **Attestation**: a formal statement that controls are in place.
- **Internal**: **compliance**, **audit committee**, **self-assessments**.
- **External**: **regulatory**, **examinations**, **assessment**, **independent third-party audit**.
- **Penetration testing** (authorised and scoped):
  - **Physical**, **offensive** (red team), **defensive** (blue team), **integrated** (purple team).
  - Knowledge: **known environment** (full information, formerly white box), **partially known** (grey box), **unknown environment** (black box).
  - **Reconnaissance**: **passive** (public information, no contact) vs **active** (scanning the target).
  - Always governed by written authorisation and **rules of engagement** (scope, times, contacts, what's off limits).

## Security awareness and policies (programme view)

Policies: **AUP**, **information security policy**, **business continuity**, **disaster recovery**, **incident response**, **SDLC**, **change management**. Standards (passwords, access control, physical security, encryption) and procedures (onboarding/offboarding, playbooks). External considerations: **regulatory**, **legal**, **industry**, **local/regional/national/global**. Governance structures: **boards**, **committees**, **government entities**, **centralised/decentralised**. Roles: **owners**, **controllers**, **processors**, **custodians/stewards**.

@@quiz
Which agreement lists a vendor's guaranteed uptime and the penalties for missing it?
* SLA
- NDA
- MOU
- BPA
? Service-level agreement.

@@quiz
Under GDPR, a company uses a payroll SaaS to process employee data. What is the payroll provider?
- Controller
* Processor
- Data subject
- Custodian
? The employer (controller) decides why the data is processed.

@@quiz
Which contract clause lets a customer inspect a vendor's security controls?
- Non-compete clause
* Right-to-audit clause
- Indemnity clause
- Force majeure
? Or rely on independent reports such as SOC 2 Type II.

@@quiz
A penetration tester is given network diagrams and source code before testing. What kind of test is this?
* Known environment
- Unknown environment
- Partially known environment
- Passive reconnaissance
? Formerly called white box.

@@quiz
Customer data stored in a Frankfurt data centre is subject to EU and German law. Which concept is this?
- Data minimisation
* Data sovereignty
- Data masking
- Data retention
? Location determines which laws apply.

@@flashcards
SLA :: measurable service levels
MOU / MOA :: intentions / agreed responsibilities
MSA :: overall terms for future work
SOW :: one project's deliverables
NDA :: confidentiality
BPA :: business partner responsibilities
right-to-audit :: customer may inspect vendor controls
SOC 2 Type II :: independent controls report over time
due diligence :: investigate before committing
controller :: decides why/how data is processed
processor :: processes data for the controller
data subject :: person the data is about
right to be forgotten :: request deletion
data sovereignty :: laws of the storage location
APPI :: Japan's personal data law
attestation :: formal statement controls exist
known / unknown environment :: white / black box testing
rules of engagement :: scope and limits of a test

@@drill
Which agreement covers a specific project's deliverables under an MSA?
= SOW
= sow
= statement of work
? Also called a work order.

@@drill
What is Japan's main personal data protection law called (acronym)?
= APPI
= appi
? Act on the Protection of Personal Information.

@@exercise Score vendor security questionnaires
Each vendor answered questions with weights. Score = sum of weights for `yes` answers ÷ total weight × 100. Any `no` on a **critical** question (MFA, encryption at rest, breach notification) makes the vendor `REJECT` regardless of score; otherwise 80+ is `APPROVE`, 60–79 is `APPROVE WITH CONDITIONS`, below 60 `REJECT`. Print each vendor's score, decision and the questions they failed.
@@lang python
@@starter
QUESTIONS = {
    "mfa": (20, True),
    "encryption_at_rest": (20, True),
    "breach_notice_72h": (15, True),
    "soc2_type2": (15, False),
    "pentest_yearly": (10, False),
    "sso_saml": (10, False),
    "data_in_japan": (10, False),
}
answers = {
    "PayrollCloud": {"mfa": "yes", "encryption_at_rest": "yes", "breach_notice_72h": "yes", "soc2_type2": "yes", "pentest_yearly": "yes", "sso_saml": "no", "data_in_japan": "yes"},
    "QuickSurvey": {"mfa": "no", "encryption_at_rest": "yes", "breach_notice_72h": "yes", "soc2_type2": "yes", "pentest_yearly": "yes", "sso_saml": "yes", "data_in_japan": "yes"},
    "PrintHub": {"mfa": "yes", "encryption_at_rest": "yes", "breach_notice_72h": "yes", "soc2_type2": "no", "pentest_yearly": "no", "sso_saml": "no", "data_in_japan": "yes"},
}
@@expected
PayrollCloud: 90 -> APPROVE; failed: sso_saml
QuickSurvey: 80 -> REJECT (critical control missing); failed: mfa
PrintHub: 65 -> APPROVE WITH CONDITIONS; failed: soc2_type2, pentest_yearly, sso_saml
@@requires
QUESTIONS.items()
critical
@@hint
Each question is `(weight, critical)`. Total weight is `sum(w for w, _ in QUESTIONS.values())`.
@@solution
QUESTIONS = {
    "mfa": (20, True),
    "encryption_at_rest": (20, True),
    "breach_notice_72h": (15, True),
    "soc2_type2": (15, False),
    "pentest_yearly": (10, False),
    "sso_saml": (10, False),
    "data_in_japan": (10, False),
}
answers = {
    "PayrollCloud": {"mfa": "yes", "encryption_at_rest": "yes", "breach_notice_72h": "yes", "soc2_type2": "yes", "pentest_yearly": "yes", "sso_saml": "no", "data_in_japan": "yes"},
    "QuickSurvey": {"mfa": "no", "encryption_at_rest": "yes", "breach_notice_72h": "yes", "soc2_type2": "yes", "pentest_yearly": "yes", "sso_saml": "yes", "data_in_japan": "yes"},
    "PrintHub": {"mfa": "yes", "encryption_at_rest": "yes", "breach_notice_72h": "yes", "soc2_type2": "no", "pentest_yearly": "no", "sso_saml": "no", "data_in_japan": "yes"},
}

total = sum(w for w, _ in QUESTIONS.values())
for vendor, given in answers.items():
    earned = 0
    failed, critical_fail = [], False
    for question, (weight, critical) in QUESTIONS.items():
        if given.get(question) == "yes":
            earned += weight
        else:
            failed.append(question)
            critical_fail = critical_fail or critical
    score = earned / total * 100
    if critical_fail:
        decision = "REJECT (critical control missing)"
    elif score >= 80:
        decision = "APPROVE"
    elif score >= 60:
        decision = "APPROVE WITH CONDITIONS"
    else:
        decision = "REJECT"
    print(f"{vendor}: {score:.0f} -> {decision}" + (f"; failed: {', '.join(failed)}" if failed else ""))

@@exercise Track agreements and renewals
Given vendor agreements with end dates, print those expiring within 60 days of `TODAY` (and how many days remain), those already expired, and vendors handling personal data who lack a signed NDA or DPA (`docs` must include `"NDA"` and `"DPA"`).
@@lang python
@@starter
from datetime import date

TODAY = date(2026, 11, 4)
agreements = [
    {"vendor": "PayrollCloud", "type": "SLA", "ends": date(2026, 12, 15), "personal_data": True, "docs": {"MSA", "SLA", "NDA", "DPA"}},
    {"vendor": "PrintHub", "type": "MSA", "ends": date(2027, 6, 30), "personal_data": False, "docs": {"MSA"}},
    {"vendor": "QuickSurvey", "type": "SOW", "ends": date(2026, 10, 1), "personal_data": True, "docs": {"SOW", "NDA"}},
    {"vendor": "CleanCo", "type": "SLA", "ends": date(2026, 11, 30), "personal_data": False, "docs": {"SLA", "NDA"}},
    {"vendor": "RecruitPro", "type": "MSA", "ends": date(2027, 3, 1), "personal_data": True, "docs": {"MSA"}},
]
@@expected
EXPIRED: QuickSurvey SOW ended 2026-10-01 (34 days ago)
RENEW SOON: CleanCo SLA ends 2026-11-30 (26 days)
RENEW SOON: PayrollCloud SLA ends 2026-12-15 (41 days)
MISSING: QuickSurvey handles personal data without DPA
MISSING: RecruitPro handles personal data without DPA, NDA
@@requires
.days
{"NDA", "DPA"}
@@hint
`(a["ends"] - TODAY).days` gives days remaining (negative if expired). Missing documents: `{"NDA", "DPA"} - a["docs"]`.
@@solution
from datetime import date

TODAY = date(2026, 11, 4)
agreements = [
    {"vendor": "PayrollCloud", "type": "SLA", "ends": date(2026, 12, 15), "personal_data": True, "docs": {"MSA", "SLA", "NDA", "DPA"}},
    {"vendor": "PrintHub", "type": "MSA", "ends": date(2027, 6, 30), "personal_data": False, "docs": {"MSA"}},
    {"vendor": "QuickSurvey", "type": "SOW", "ends": date(2026, 10, 1), "personal_data": True, "docs": {"SOW", "NDA"}},
    {"vendor": "CleanCo", "type": "SLA", "ends": date(2026, 11, 30), "personal_data": False, "docs": {"SLA", "NDA"}},
    {"vendor": "RecruitPro", "type": "MSA", "ends": date(2027, 3, 1), "personal_data": True, "docs": {"MSA"}},
]

for a in sorted(agreements, key=lambda a: a["ends"]):
    days = (a["ends"] - TODAY).days
    if days < 0:
        print(f"EXPIRED: {a['vendor']} {a['type']} ended {a['ends']} ({-days} days ago)")
    elif days <= 60:
        print(f"RENEW SOON: {a['vendor']} {a['type']} ends {a['ends']} ({days} days)")

for a in agreements:
    missing = {"NDA", "DPA"} - a["docs"]
    if a["personal_data"] and missing:
        print(f"MISSING: {a['vendor']} handles personal data without {', '.join(sorted(missing))}")

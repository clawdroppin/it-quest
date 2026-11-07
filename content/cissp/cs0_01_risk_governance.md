@@lesson cs0-risk-governance CISSP: The Exam, Governance, Risk and Assets
@@text
## The CISSP

The **CISSP** (Certified Information Systems Security Professional, from ISC2) is the best-known *management-level* security certification. It's a **CAT** (computerized adaptive testing) exam: 100–150 questions in 3 hours, harder or easier depending on your answers, and you need 700 of 1000 points. To be certified you also need **five years** of paid work in at least two of the eight domains (four with a degree or another approved certification). Pass without the experience and you become an **Associate of ISC2** while you earn it. The exam follows the outline ISC2 published in April 2024; a new outline takes effect on **15 April 2027**, keeping the eight domains with adjusted weights.

| # | Domain | Weight |
|---|---|---|
| 1 | Security and Risk Management | 16% |
| 2 | Asset Security | 10% |
| 3 | Security Architecture and Engineering | 13% |
| 4 | Communication and Network Security | 13% |
| 5 | Identity and Access Management | 13% |
| 6 | Security Assessment and Testing | 12% |
| 7 | Security Operations | 13% |
| 8 | Software Development Security | 10% |

**Think like a manager**, not a technician. When several answers are technically right, the best one usually: protects **people's safety first**, follows **policy and management's decisions**, addresses the **root cause**, and is about **risk** and **business value**. "Shut down the server yourself" is rarely the answer; "inform management / follow the incident response plan" often is.

For Japan: CISSP is respected at global companies, consultancies and the security teams of large Japanese firms. Japan also has its own national security qualification, **情報処理安全確保支援士** (Registered Information Security Specialist, RISS), from IPA.

## Security fundamentals

- **CIA triad**: **confidentiality** (only authorized people see it), **integrity** (it isn't changed improperly), **availability** (it's there when needed). Plus **authenticity** and **non-repudiation** (you can't deny you did it: digital signatures).
- **Due care** (doing what a reasonable person would: the controls) and **due diligence** (checking that they work: audits, assessments).
- **Defense in depth**, **least privilege**, **separation of duties**, **need to know**, **job rotation**, **mandatory vacation** (fraud shows up when the person is away).
- **Governance**: the board and senior management are ultimately accountable. Documents from broad to specific: **policy** (mandatory, high-level, approved by management) → **standards** (mandatory specifics: "AES-256") → **procedures** (step by step) → **guidelines** (recommended, optional). **Baselines** are minimum configurations.
- Frameworks: **ISO/IEC 27001** (ISMS, certifiable; widely used in Japan as ISMS認証) and 27002 (controls), **NIST CSF 2.0** (Govern, Identify, Protect, Detect, Respond, Recover), **NIST SP 800-53** (controls), **COBIT** (IT governance), **SOC 2** reports for service providers.

## Risk management

**Risk = threat × vulnerability** (× impact). The vocabulary:

- **Asset value (AV)**: what it's worth.
- **Exposure factor (EF)**: the share of the value lost in one incident (0–100%).
- **Single loss expectancy: SLE = AV × EF**.
- **Annualized rate of occurrence (ARO)**: how many times a year (0.1 = once in ten years).
- **Annualized loss expectancy: ALE = SLE × ARO**.
- A control is worth it when **ALE before − ALE after − annual cost of the control > 0**. That's the "value of the safeguard".

Example: a ¥50,000,000 customer database, a breach would expose 40% of its value (EF 0.4), expected once every 5 years (ARO 0.2). SLE = ¥20,000,000, ALE = ¥4,000,000 a year. An ¥1,500,000-a-year DLP system that cuts ARO to 0.05 brings ALE to ¥1,000,000: value ¥4,000,000 − ¥1,000,000 − ¥1,500,000 = **¥1,500,000 a year**. Worth it. You'll build a calculator for this in the project.

**Risk responses**: **mitigate** (reduce with controls), **transfer** (insurance, outsourcing: the *accountability* stays with you), **avoid** (stop the activity), **accept** (a documented management decision, for risks below the **risk appetite**). Ignoring a risk is never acceptable. What's left after controls is **residual risk**.

Qualitative analysis ranks risks (high/medium/low, a heat map); quantitative puts money on them (ALE). Most organizations combine both.

**Control types**: by category, *administrative* (policies, training), *technical/logical* (firewalls, encryption), *physical* (locks, guards); by function, **preventive, detective, corrective, deterrent, recovery, compensating** (an alternative when the usual control isn't possible), directive.

## Business continuity

- **BIA (business impact analysis)** first: which processes matter, and for how long the business can survive without them.
- **MTD** (maximum tolerable downtime) ≥ **RTO** (recovery time objective) + **WRT** (work recovery time). **RPO**: how much data loss is acceptable (drives backup frequency).
- Sites: **hot** (ready in minutes or hours), **warm** (hardware, needs data), **cold** (space and power only), **mirrored/redundant** (active-active), cloud.
- Tests from least to most disruptive: **checklist/read-through**, **tabletop (walk-through)**, **simulation**, **parallel** (the alternate site runs alongside), **full interruption** (production fails over for real).

## Law, ethics and privacy

- **ISC2 Code of Ethics** canons, in order of priority: protect **society**, the common good and infrastructure; act **honorably**, honestly, justly, responsibly and legally; provide diligent and competent service to **principals**; advance and protect the **profession**.
- Privacy laws: **GDPR** (EU: lawful basis, data subject rights, breach notification within 72 hours, fines up to 4% of global turnover), Japan's **APPI** (個人情報保護法: notify the Personal Information Protection Commission of serious breaches), HIPAA (US health), PCI DSS (a card-industry *standard*, not a law).
- Intellectual property: copyright, trademarks, patents, **trade secrets**. Licensing and export controls (Wassenaar) for cryptography.
- Supply chain risk: third-party assessments, SLAs, the right to audit, SBOMs.

## Asset security (domain 2)

- **Classification**: government (Top Secret, Secret, Confidential, Unclassified) or commercial (Confidential/Proprietary, Private, Sensitive, Public). Classify by value and sensitivity, label, and handle accordingly.
- **Roles**: the **data owner** (a senior business person, accountable, decides the classification), the **data custodian** (IT: implements the controls, backups), the **data steward** (data quality), **data processor** (processes on behalf of a **controller**, in GDPR terms), users.
- **Data states**: at rest (encryption), in transit (TLS, VPN), in use (access controls, confidential computing).
- **Retention and destruction**: keep data only as long as required. **Data remanence**: clearing (overwriting), **purging** (degaussing, crypto-erase: resists lab attacks), **destruction** (shredding, incineration: the surest). SSDs: crypto-erase or physical destruction, since overwriting doesn't reliably reach every cell.
- **DLP**, **DRM**, **CASB** (cloud access security brokers) protect data wherever it goes.

@@quiz
A fire breaks out in the data centre during a security incident. What's your first priority?
* Make sure people are safe and get out
- Preserve the evidence on the servers
- Start the backup generator
- Notify the insurance company
? Human safety always comes first on the CISSP.

@@quiz
An asset is worth $200,000, a flood would destroy 25% of it, and floods happen once every 10 years. What's the ALE?
* $5,000
- $50,000
- $20,000
- $2,000
? SLE = 200,000 × 0.25 = 50,000; ALE = 50,000 × 0.1 = 5,000.

@@quiz
A company buys cyber insurance for a risk. Which response is this, and who is accountable for the risk?
* Transfer; the company remains accountable
- Transfer; the insurer becomes accountable
- Avoidance; nobody
- Acceptance; the insurer
? You can transfer the financial impact, never the accountability.

@@quiz
Who decides how a dataset is classified?
* The data owner
- The data custodian
- The security administrator
- The end users
? The owner is accountable; custodians implement the protection.

@@quiz
Which is the most secure way to sanitize SSDs that held secret data before they leave the company?
* Physical destruction (or verified cryptographic erasure)
- Deleting the files
- A quick format
- Degaussing
? Degaussing doesn't work on flash memory, and overwriting may miss spare cells.

@@quiz
Which business continuity test involves the most risk to production?
* Full interruption
- Parallel
- Tabletop
- Checklist
? Full interruption actually shuts down the primary site.

@@quiz
Which document is mandatory and gives high-level direction from management?
* A policy
- A guideline
- A procedure
- A baseline
? Standards and procedures implement policy; guidelines are optional.

@@flashcards
CIA :: confidentiality, integrity, availability
due care / due diligence :: doing the right thing / checking that it's done
SLE :: AV × EF
ALE :: SLE × ARO
value of a safeguard :: ALE before − ALE after − annual cost
risk responses :: mitigate, transfer, avoid, accept
residual risk :: what remains after controls
compensating control :: an alternative when the normal one isn't possible
MTD :: maximum tolerable downtime (≥ RTO + WRT)
RPO :: acceptable data loss
hot / warm / cold site :: ready now / hardware without data / space and power
ISC2 canon #1 :: protect society, the common good and infrastructure
data owner / custodian :: accountable, classifies / implements controls
purging :: sanitization that resists laboratory recovery
GDPR breach notification :: within 72 hours

@@drill
A control costs ¥300,000 a year and lowers ALE from ¥1,000,000 to ¥400,000. What's its annual value in yen? (just the number)
= 300000
= 300,000
? 1,000,000 − 400,000 − 300,000 = 300,000.

@@exercise Think like a manager
CISSP questions often have several technically correct answers; the best one follows the manager's priorities: human safety first, then legal and regulatory obligations, then governance (policy, senior management decisions), then business needs, and technical fixes last. Within the same priority, assessing and planning comes before acting. Pick the best answer for each scenario.
@@lang python
@@starter
PRIORITY = ["safety", "legal", "governance", "business", "technical"]

scenarios = {
    "Fire alarm during a ransomware outbreak": [
        ("Isolate infected servers first", "technical", "act"),
        ("Evacuate staff from the building", "safety", "act"),
        ("Notify the regulator", "legal", "act"),
    ],
    "New privacy law affects customer data": [
        ("Encrypt every database now", "technical", "act"),
        ("Assess which data and processes the law covers", "legal", "assess"),
        ("Update the privacy policy immediately", "legal", "act"),
    ],
    "Business wants to skip MFA for executives": [
        ("Disable MFA as asked", "business", "act"),
        ("Present the risk to senior management and record their decision", "governance", "assess"),
        ("Secretly keep MFA enabled", "technical", "act"),
    ],
}

def best_answer(options):
    raise NotImplementedError

for name, options in scenarios.items():
    text, kind, step = best_answer(options)
    print(f"{name}\n  -> {text} ({kind}, {step})")
@@expected
Fire alarm during a ransomware outbreak
  -> Evacuate staff from the building (safety, act)
New privacy law affects customer data
  -> Assess which data and processes the law covers (legal, assess)
Business wants to skip MFA for executives
  -> Present the risk to senior management and record their decision (governance, assess)
@@requires
def best_answer
PRIORITY.index(
@@hint
Sort by a key of `(PRIORITY.index(kind), 0 if step == "assess" else 1)` and take the first option.
@@solution
PRIORITY = ["safety", "legal", "governance", "business", "technical"]

scenarios = {
    "Fire alarm during a ransomware outbreak": [
        ("Isolate infected servers first", "technical", "act"),
        ("Evacuate staff from the building", "safety", "act"),
        ("Notify the regulator", "legal", "act"),
    ],
    "New privacy law affects customer data": [
        ("Encrypt every database now", "technical", "act"),
        ("Assess which data and processes the law covers", "legal", "assess"),
        ("Update the privacy policy immediately", "legal", "act"),
    ],
    "Business wants to skip MFA for executives": [
        ("Disable MFA as asked", "business", "act"),
        ("Present the risk to senior management and record their decision", "governance", "assess"),
        ("Secretly keep MFA enabled", "technical", "act"),
    ],
}

def best_answer(options):
    return min(options, key=lambda o: (PRIORITY.index(o[1]), 0 if o[2] == "assess" else 1))

for name, options in scenarios.items():
    text, kind, step = best_answer(options)
    print(f"{name}\n  -> {text} ({kind}, {step})")

@@exercise Who is the owner, custodian or processor?
Assign the lesson's data roles. Each statement describes what a person or company does with the payroll data. Write `role(statement)` that returns the role, checking in this order:

- decides why and how personal data is processed (`decides the purpose`) → **data controller**;
- processes it on someone else's behalf (`on behalf of`) → **data processor**;
- accountable, classifies it, approves access (`accountable`, `classifies`, `approves access`) → **data owner**;
- implements controls, backups, patches (`backups`, `implements`, `encryption`) → **data custodian**;
- keeps it accurate and consistent (`quality`, `accurate`) → **data steward**;
- the person the data is about (`about them`) → **data subject**.

Print each statement and role, then check the classic exam trap: the IT team should never be the owner.
@@lang python
@@starter
RULES = [
    ("data controller", ["decides the purpose"]),
    ("data processor", ["on behalf of"]),
    ("data owner", ["accountable", "classifies", "approves access"]),
    ("data custodian", ["backups", "implements", "encryption"]),
    ("data steward", ["quality", "accurate"]),
    ("data subject", ["about them"]),
]
statements = [
    ("Head of HR", "is accountable for payroll data and classifies it as confidential"),
    ("IT operations", "runs the nightly backups and implements disk encryption"),
    ("Payroll SaaS vendor", "processes salaries on behalf of the company"),
    ("Our company", "decides the purpose of collecting bank details"),
    ("Data team", "checks the records are accurate and consistent"),
    ("Employee", "has data about them in the payroll system"),
    ("IT manager", "approves access to the payroll share"),
]

def role(statement):
    raise NotImplementedError


for who, what in statements:
    r = role(what)
    note = "  <- trap: owners are business people, not IT" if r == "data owner" and "IT" in who else ""
    print(f"{who}: {r}{note}")
@@expected
Head of HR: data owner
IT operations: data custodian
Payroll SaaS vendor: data processor
Our company: data controller
Data team: data steward
Employee: data subject
IT manager: data owner  <- trap: owners are business people, not IT
@@requires
def role
any(
@@hint
An ordered list of (role, keywords) and a loop that returns the first match. The trap check only needs the person's name and the role you found.
@@solution
RULES = [
    ("data controller", ["decides the purpose"]),
    ("data processor", ["on behalf of"]),
    ("data owner", ["accountable", "classifies", "approves access"]),
    ("data custodian", ["backups", "implements", "encryption"]),
    ("data steward", ["quality", "accurate"]),
    ("data subject", ["about them"]),
]
statements = [
    ("Head of HR", "is accountable for payroll data and classifies it as confidential"),
    ("IT operations", "runs the nightly backups and implements disk encryption"),
    ("Payroll SaaS vendor", "processes salaries on behalf of the company"),
    ("Our company", "decides the purpose of collecting bank details"),
    ("Data team", "checks the records are accurate and consistent"),
    ("Employee", "has data about them in the payroll system"),
    ("IT manager", "approves access to the payroll share"),
]

def role(statement):
    for name, words in RULES:
        if any(w in statement for w in words):
            return name
    return "unclear"

for who, what in statements:
    r = role(what)
    note = "  <- trap: owners are business people, not IT" if r == "data owner" and "IT" in who else ""
    print(f"{who}: {r}{note}")

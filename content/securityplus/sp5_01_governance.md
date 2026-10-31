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

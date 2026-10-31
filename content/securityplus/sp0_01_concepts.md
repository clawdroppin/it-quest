@@lesson sp0-concepts Security+ and Core Security Concepts
@@text
**CompTIA Security+** is the baseline security certification for IT jobs. It's required for many government roles worldwide, and it's what employers expect before any security-flavoured job: SOC analyst, security administrator, or a sysadmin who's trusted with security. The current exam is **SY0-701**: up to 90 questions in 90 minutes, passing score **750** (100–900).

| Domain | Weight |
|---|---|
| 1 General security concepts | 12% |
| 2 Threats, vulnerabilities and mitigations | 22% |
| 3 Security architecture | 18% |
| 4 Security operations | 28% |
| 5 Security program management and oversight | 20% |

## CIA, AAA and non-repudiation

You know the **CIA triad** from Foundations. Add:

- **Non-repudiation**: someone can't deny they did something (digital signatures, audit logs).
- **AAA**: **Authentication** (who are you?), **Authorization** (what may you do?), **Accounting** (what did you do? logs). RADIUS and TACACS+ are AAA protocols.

## Security controls

Every control has a **category** (what kind) and a **type** (what it does):

| Category | Examples |
|---|---|
| **Technical** | firewalls, encryption, MFA, EDR |
| **Managerial** | policies, risk assessments, security awareness programs |
| **Operational** | guards, procedures people follow, backups being done |
| **Physical** | locks, fences, badges, cameras |

| Type | Purpose | Example |
|---|---|---|
| **Preventive** | stops it happening | firewall rule, door lock |
| **Deterrent** | discourages it | warning sign, visible camera |
| **Detective** | notices it | IDS, log review, motion sensor |
| **Corrective** | fixes it after | restore from backup, patch |
| **Compensating** | an alternative when the normal control isn't possible | extra monitoring on an old system that can't be patched |
| **Directive** | tells people what to do | policy, "authorized personnel only" |

## Zero trust

"Never trust, always verify." No user or device is trusted because it's *inside* the network. Each request is checked by identity, device health and context.

- **Control plane**: the **policy engine** decides, the **policy administrator** tells the enforcement point.
- **Data plane**: the **policy enforcement point** sits between the subject and the resource.
- Key ideas: **adaptive identity**, **threat scope reduction**, **implicit trust zones** eliminated, microsegmentation.

## Other must-know concepts

- **Least privilege** and **separation of duties** (no single person can both request and approve a payment).
- **Defence in depth**: layers, so one failure isn't a breach.
- **Gap analysis**: where are we compared to where we should be (a framework like NIST CSF or ISO 27001)?
- **Deception**: **honeypots** (fake systems), **honeynets**, **honeyfiles**, **honeytokens** (fake credentials that alert when used).
- **Change management** for security: approval, impact analysis, test results, backout plan, maintenance windows, updated documentation; know that a change may require a **service restart** or downtime, and that **allow/deny lists** and **dependencies** may be affected.

@@quiz
A company puts up a sign saying "This area is under 24-hour video surveillance". What type of control is the sign?
* Deterrent
- Detective
- Corrective
- Compensating
? The sign discourages; the cameras themselves recording are detective.

@@quiz
A legacy machine can't be patched, so it's placed on an isolated VLAN with extra monitoring. What type of control is this?
* Compensating
- Preventive only
- Directive
- Deterrent
? An alternative control when the normal one (patching) isn't possible.

@@quiz
Which provides non-repudiation for an email?
* A digital signature with the sender's private key
- Sending it over TLS
- A strong password on the mailbox
- A spam filter
? Only the holder of the private key could have signed it.

@@quiz
In a zero trust architecture, which component makes the access decision?
* The policy engine
- The policy enforcement point
- The subject
- The implicit trust zone
? The policy engine decides; the administrator communicates; the enforcement point enforces.

@@quiz
Fake admin credentials are planted in a file. Any attempt to use them raises an alert. What is this?
* A honeytoken
- A honeypot
- A digital signature
- A compensating control
? Honeytokens are fake data that shouldn't ever be used legitimately.

@@quiz
An accounts employee can create vendors but not approve payments to them. Which principle is this?
* Separation of duties
- Least privilege only
- Defence in depth
- Non-repudiation
? Splitting critical tasks prevents a single person committing fraud.

@@quiz
A security awareness training program belongs to which control category?
* Managerial
- Technical
- Physical
- Corrective
? (Some sources classify the training *delivery* as operational; the program and policy are managerial.)

@@flashcards
CIA :: Confidentiality, Integrity, Availability
non-repudiation :: Proof someone did something; they can't deny it
AAA :: Authentication, Authorization, Accounting
control categories :: Technical, managerial, operational, physical
control types :: Preventive, deterrent, detective, corrective, compensating, directive
compensating control :: Alternative when the standard control isn't feasible
zero trust :: Never trust, always verify; no implicit trust zones
policy engine / administrator / enforcement point :: Decides / communicates / enforces
separation of duties :: Split critical tasks between people
defence in depth :: Several layers of controls
gap analysis :: Compare the current state to a target framework
honeypot / honeytoken :: Fake system / fake data that alerts when touched

@@drill
Which control type is a firewall rule that blocks traffic?
= preventive
? It stops the event from happening.

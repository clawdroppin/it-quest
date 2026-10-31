@@lesson cs2-assessment-operations-software Assessment, Operations and Secure Software
@@text
## Security assessment and testing (domain 6)

- **Vulnerability assessment** (scanning, finding weaknesses) versus **penetration test** (exploiting them, with written **authorization** and agreed **rules of engagement**). Knowledge: black box (no information), gray box, white box (full information).
- **Audits**: internal, external, third-party. **SOC reports** for service providers: **SOC 1** (financial reporting controls), **SOC 2** (security, availability, processing integrity, confidentiality, privacy: **Type I** = design at a point in time, **Type II** = operating effectiveness over a period, usually 6–12 months), **SOC 3** (a public summary).
- **Software testing**: static (**SAST**: code without running it), dynamic (**DAST**: the running app), **IAST**, **fuzzing**, **SCA** (dependencies and their CVEs), code review, **misuse case** testing, regression, **synthetic transactions** and **RUM** (real user monitoring) in production.
- **Log reviews** and **KPIs/KRIs** (key performance and risk indicators) feed reports to management.
- **CVSS** scores vulnerabilities from 0 to 10; **CVE** names them; **CWE** classifies weakness types.

## Security operations (domain 7)

- **Incident response** (the CISSP's version): **detection → response → mitigation → reporting → recovery → remediation → lessons learned**. (NIST SP 800-61 uses preparation; detection and analysis; containment, eradication and recovery; post-incident activity.) Containment first limits the damage.
- **Forensics and evidence**: preserve the **chain of custody** (who had it, when, why); collect from most to least volatile (**order of volatility**: CPU registers and cache, RAM, network state, disk, logs on remote systems, backups); work on **bit-for-bit copies** verified with hashes; use write blockers. Evidence must be relevant, reliable and admissible.
- Investigation types: administrative, criminal (beyond reasonable doubt), civil (preponderance of evidence), regulatory.
- **Logging and monitoring**: SIEM, **SOAR** (automated playbooks), UEBA, threat intelligence, **threat hunting**, egress monitoring, honeypots.
- **Change management**: request, review, approve (CAB), test, implement, document, with a rollback plan. **Configuration management** and baselines. **Patch management**.
- **Backups**: full, incremental (since the last backup of any kind: fast backup, slow restore), differential (since the last full: grows each day, faster restore). **3-2-1 rule**: three copies, two media types, one off-site (plus one immutable or offline against ransomware). Test restores.
- **Disaster recovery**: RAID (RAID 1 mirroring, RAID 5 parity with one failure tolerated, RAID 6 two, RAID 10 mirrored stripes; RAID is *not* a backup), clustering, redundant power (UPS, generators).
- **Personnel**: background checks, NDAs, onboarding and **offboarding** (disable access *before or during* the termination meeting), awareness training, travel security, duress codes.

## Software development security (domain 8)

- **SDLC** with security in every phase: requirements (security requirements, privacy), design (**threat modeling**: STRIDE: spoofing, tampering, repudiation, information disclosure, denial of service, elevation of privilege), development (secure coding, code review), testing, deployment, operations, disposal.
- Methods: waterfall, **Agile**/Scrum, **DevSecOps** (security automated in the CI/CD pipeline: "shift left"). Maturity models: **CMMI** (levels 1 initial, 2 managed, 3 defined, 4 quantitatively managed, 5 optimizing), **OWASP SAMM**, BSIMM.
- **OWASP Top 10** families: broken access control, cryptographic failures, injection (SQL, OS command, LDAP: fix with **parameterized queries** and input validation), insecure design, security misconfiguration, vulnerable components, authentication failures, integrity failures (unsigned updates, insecure deserialization), logging failures, SSRF.
- **XSS** (stored, reflected, DOM: fix with **output encoding** and CSP), **CSRF** (anti-CSRF tokens, SameSite cookies), buffer overflows (bounds checks, ASLR, DEP/NX, memory-safe languages), race conditions.
- **Supply chain**: dependency scanning, **SBOM**, signed artifacts, code repositories with branch protection, secrets kept out of code, APIs authenticated and rate-limited.
- **Acquired software**: COTS, open source licensing, SaaS: assess the vendor, contract for security (escrow for source code if the vendor disappears).
- Databases: **aggregation** (combining low-sensitivity data reveals something sensitive) and **inference**; **polyinstantiation** (several records at different classification levels to stop inference); ACID transactions.

@@quiz
A company wants assurance that a cloud provider's security controls *operated effectively over the last year*. Which report should it ask for?
* SOC 2 Type II
- SOC 2 Type I
- SOC 1 Type I
- SOC 3
? Type I only covers design at a single point in time.

@@quiz
During an investigation, which should be collected first?
* The contents of RAM
- The hard disk image
- Backup tapes
- Printed logs
? Follow the order of volatility: the most volatile evidence disappears first.

@@quiz
Backups: a full backup on Sunday and incrementals every other day. The server fails on Thursday. What do you restore?
* Sunday's full, then the incrementals of Monday, Tuesday and Wednesday, in order
- Only Wednesday's incremental
- Sunday's full and Wednesday's incremental
- Only Sunday's full
? With differentials you'd need only the full and the latest differential.

@@quiz
An application builds SQL queries by concatenating user input. What's the best fix?
* Parameterized queries (prepared statements)
- A WAF in front of it
- Hiding error messages
- Encrypting the database
? A WAF and input validation help, but parameterization removes the cause.

@@quiz
Which threat-modeling model uses the categories spoofing, tampering, repudiation, information disclosure, denial of service and elevation of privilege?
* STRIDE
- DREAD
- PASTA
- CVSS
? DREAD rates risks; PASTA is a risk-centric process.

@@quiz
An employee is being fired. When should their access be disabled?
* Before or during the termination meeting
- At the end of their notice period
- A week after they leave
- When their manager remembers
? Disgruntled ex-employees with access are a classic insider threat.

@@flashcards
SOC 2 Type I / Type II :: design at a point in time / effectiveness over a period
SAST / DAST :: source code / running application
CVE / CWE / CVSS :: a vulnerability / a weakness type / a severity score
chain of custody :: who handled evidence, when and why
order of volatility :: registers, cache, RAM, network, disk, remote logs, backups
incremental vs differential :: since last backup / since last full
3-2-1 backups :: 3 copies, 2 media, 1 off-site
RAID 5 :: striping with parity, survives one disk
STRIDE :: spoofing, tampering, repudiation, info disclosure, DoS, elevation
CMMI level 5 :: optimizing
SQL injection fix :: parameterized queries
XSS fix :: output encoding and CSP
CSRF fix :: anti-CSRF tokens, SameSite cookies
aggregation / inference :: combining data / deducing hidden data
polyinstantiation :: different records per classification level

@@drill
Which CMMI level is "defined"? (a number)
= 3

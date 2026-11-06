@@lesson cs2-software-security-deep Software Development Security in Depth: SDLC, Testing Tools, Supply Chain and APIs (Domain 8)
@@concepts lists, maps, conditions
@@lang python
@@text
Domain 8 asks how security gets **built into** software rather than bolted on at the end. A CISSP doesn't need to write exploit code. They need to know where in the life cycle each control belongs, which testing tool finds what, how to judge the maturity of a development organisation, and how to manage the risk of code you didn't write.

## Security in the life cycle

- **Requirements**: security and privacy requirements, abuse cases ("as an attacker I want..."), compliance needs.
- **Design**: threat modelling (**STRIDE**: spoofing, tampering, repudiation, information disclosure, denial of service, elevation of privilege; or **PASTA**, attack trees), secure design patterns, a security architecture review.
- **Implementation**: secure coding standards, peer review, **SAST**, secret scanning, dependency checks (**SCA**).
- **Testing**: **DAST**, **IAST**, fuzzing, penetration tests, and regression tests for previously fixed vulnerabilities.
- **Deployment and operations**: hardened configuration, a **WAF** or **RASP** where useful, monitoring, patching, and a vulnerability disclosure policy.
- **Disposal**: retire data and credentials and turn off endpoints.
- Fixing a flaw in design costs a fraction of fixing it in production. That is the business case for "shift left".

## Development methodologies

- **Waterfall**: sequential phases with security gates. Predictable, but slow feedback.
- **Agile / Scrum**: short sprints. Security works as backlog items, a definition of done that includes security checks, and security champions in each team.
- **DevOps / DevSecOps**: automation in the CI/CD pipeline, with SAST, SCA, container image scanning and infrastructure-as-code scanning on every commit and **gates** that block critical findings.
- **Maturity models**: **SAMM** (OWASP Software Assurance Maturity Model: governance, design, implementation, verification, operations), **BSIMM** (a descriptive survey of what real firms do), and **CMMI** (initial, managed, defined, quantitatively managed, optimising).

## Which tool finds what

| Tool | How | Finds | Misses |
|---|---|---|---|
| **SAST** | Reads source code or binaries | Injection patterns, hard-coded secrets, unsafe functions, early in the cycle | Runtime/config issues, many false positives |
| **DAST** | Attacks the running app from outside | Real exploitable issues, misconfiguration, missing headers | Code it can't reach; finds problems late |
| **IAST** | An agent inside the running app during tests | Precise locations with few false positives | Needs good test coverage |
| **SCA** | Compares dependencies against vulnerability databases | Known CVEs and licence problems | Your own code's bugs |
| **Fuzzing** | Sends malformed or random input | Crashes and memory bugs, parser flaws | Logic flaws |

## Common weaknesses (OWASP Top 10 style)

- Broken access control (the most common: IDOR, missing checks on the server).
- Cryptographic failures, injection (SQL, OS command, LDAP), insecure design, security misconfiguration, vulnerable components, authentication failures, integrity failures (unsigned updates, insecure deserialisation), logging and monitoring failures, SSRF.
- **Defences**: parameterised queries, output encoding, server-side authorisation on every request, least-privilege service accounts, safe defaults, and input validation by allow-list.

## Supply chain and third-party code

- Keep an **SBOM** (software bill of materials, in SPDX or CycloneDX format) for every release, so that when a new CVE appears you know at once which products are affected.
- Pin versions, verify signatures and checksums, mirror dependencies internally, and watch for **typosquatting** and **dependency confusion** (a public package with the same name as an internal one).
- **SLSA** levels describe build integrity (provenance, isolated builds).
- **Code signing** proves origin and integrity. Protect signing keys in an HSM.
- For COTS software and SaaS: request security attestations (SOC 2, ISO 27001), check how fast they patch, and keep **escrow** for critical software from small vendors.

## API security

- Authenticate every call (OAuth 2.0 access tokens, mTLS between services).
- Authorise at the object level, which prevents **BOLA**, the API version of IDOR.
- Rate-limit, validate schemas, avoid exposing internal identifiers, and inventory every API, including old versions ("zombie APIs").

@@quiz
Which testing approach analyses source code without running it?
* SAST
- DAST
- IAST
- Fuzzing
? Early feedback, more false positives.

@@quiz
A new critical CVE is announced in a logging library. What lets you find affected products fastest?
* An SBOM for every release
- A penetration test
- DAST scans
- The change advisory board
? Know what's inside your software.

@@quiz
Which STRIDE threat is addressed by digital signatures and audit logs that prove who did something?
- Spoofing
- Tampering
* Repudiation
- Elevation of privilege
? Non-repudiation.

@@quiz
An attacker publishes a public package with the same name as your internal package, and the build fetches it. What is this called?
* Dependency confusion
- Typosquatting
- SQL injection
- Code signing
? Use scoped names and internal mirrors.

@@quiz
Which maturity model describes what real organisations actually do rather than prescribing what to do?
- SAMM
* BSIMM
- CMMI
- STRIDE
? BSIMM is a descriptive survey.

@@quiz
Why is fixing security flaws during design preferred?
* It costs far less than fixing them after release
- Design flaws can't be fixed later
- Testing tools can't find design flaws
- It's required by GDPR
? Shift left.

@@quiz
Which defence prevents SQL injection most reliably?
* Parameterised queries
- Client-side validation
- Hiding error messages
- A longer database password
? Data never becomes code.

@@flashcards
shift left :: find and fix flaws earlier in the life cycle
abuse case :: requirement written from the attacker's view
STRIDE :: spoofing, tampering, repudiation, disclosure, DoS, elevation
SAST :: source analysis, no running
DAST :: attacks the running app
IAST :: agent inside the running app
SCA :: known-vulnerable dependencies and licences
fuzzing :: malformed input to find crashes
SBOM :: list of components in a release (SPDX, CycloneDX)
dependency confusion :: public package shadows an internal name
SLSA :: build integrity levels
SAMM / BSIMM :: prescriptive / descriptive maturity models
CMMI levels :: initial, managed, defined, quantitative, optimising
BOLA :: broken object-level authorisation in APIs
code escrow :: source held by a third party for critical software

@@drill
Which STRIDE letter covers gaining rights you shouldn't have?
= E
= elevation of privilege
? Elevation of privilege.

@@drill
Which tool type compares your dependencies against vulnerability databases? (acronym)
= SCA
? Software composition analysis.

@@exercise Pipeline security gate
Findings from SAST, SCA and secret scanning arrive with a severity and whether they're new in this change. Implement the gate: block on any new critical or high finding, and on any secret (new or old); report medium findings as warnings; ignore low ones. Also compute the remediation due date for each blocking or warning finding from the policy (critical 7 days, high 30, medium 90, secret 1).
@@starter
from datetime import date, timedelta

POLICY_DAYS = {"critical": 7, "high": 30, "medium": 90, "secret": 1}
today = date(2026, 5, 4)

findings = [
    {"id": "SAST-12", "tool": "sast", "severity": "high", "new": True, "title": "SQL built from request parameter"},
    {"id": "SCA-3", "tool": "sca", "severity": "critical", "new": False, "title": "log4j-core 2.14 (CVE-2021-44228)"},
    {"id": "SEC-1", "tool": "secrets", "severity": "secret", "new": False, "title": "AWS access key in config/old.yml"},
    {"id": "SAST-40", "tool": "sast", "severity": "medium", "new": True, "title": "Missing output encoding in template"},
    {"id": "SCA-9", "tool": "sca", "severity": "low", "new": True, "title": "Outdated minor version of a date library"},
]

def classify(f):
    raise NotImplementedError

def gate(findings):
    raise NotImplementedError

decision, rows = gate(findings)
for verdict, f, due in rows:
    print(f"{verdict:<7} {f['id']:<8} {f['title']:<40} fix by {due}")
print("pipeline:", decision)
@@expected
block   SAST-12  SQL built from request parameter         fix by 2026-06-03
warn    SCA-3    log4j-core 2.14 (CVE-2021-44228)         fix by 2026-05-11
block   SEC-1    AWS access key in config/old.yml         fix by 2026-05-05
warn    SAST-40  Missing output encoding in template      fix by 2026-08-02
pipeline: FAIL
@@requires
def classify
def gate
timedelta(days=
@@hint
`classify` returns "block", "warn" or "ignore": secrets always block; new critical/high block; existing critical/high warn (they're already known, but still tracked); medium warns; low is ignored. In `gate`, build rows for everything that isn't ignored with `today + timedelta(days=POLICY_DAYS[severity])`, and the decision is "FAIL" if any row is "block".
@@solution
from datetime import date, timedelta

POLICY_DAYS = {"critical": 7, "high": 30, "medium": 90, "secret": 1}
today = date(2026, 5, 4)

findings = [
    {"id": "SAST-12", "tool": "sast", "severity": "high", "new": True, "title": "SQL built from request parameter"},
    {"id": "SCA-3", "tool": "sca", "severity": "critical", "new": False, "title": "log4j-core 2.14 (CVE-2021-44228)"},
    {"id": "SEC-1", "tool": "secrets", "severity": "secret", "new": False, "title": "AWS access key in config/old.yml"},
    {"id": "SAST-40", "tool": "sast", "severity": "medium", "new": True, "title": "Missing output encoding in template"},
    {"id": "SCA-9", "tool": "sca", "severity": "low", "new": True, "title": "Outdated minor version of a date library"},
]

def classify(f):
    sev = f["severity"]
    if sev == "secret":
        return "block"
    if sev in ("critical", "high"):
        return "block" if f["new"] else "warn"
    if sev == "medium":
        return "warn"
    return "ignore"

def gate(findings):
    rows = []
    for f in findings:
        verdict = classify(f)
        if verdict != "ignore":
            rows.append((verdict, f, today + timedelta(days=POLICY_DAYS[f["severity"]])))
    decision = "FAIL" if any(v == "block" for v, _, _ in rows) else "PASS"
    return decision, rows

decision, rows = gate(findings)
for verdict, f, due in rows:
    print(f"{verdict:<7} {f['id']:<8} {f['title']:<40} fix by {due}")
print("pipeline:", decision)

@@exercise Match an SBOM against advisories
Each product has an SBOM of `(package, version)` pairs. Each advisory names a package and an affected version range `[introduced, fixed)`. Compare versions numerically (so 2.10 is newer than 2.9), and print which products are affected, by which advisory, and the version to upgrade to.
@@starter
sboms = {
    "billing-api 3.2": [("log4j-core", "2.14.1"), ("jackson-databind", "2.13.3"), ("netty", "4.1.90")],
    "portal 1.8": [("log4j-core", "2.17.1"), ("spring-web", "5.3.20")],
    "batch 0.9": [("jackson-databind", "2.9.10"), ("netty", "4.1.100")],
}
advisories = [
    {"id": "CVE-2021-44228", "package": "log4j-core", "introduced": "2.0", "fixed": "2.15.0"},
    {"id": "CVE-2020-36518", "package": "jackson-databind", "introduced": "2.0", "fixed": "2.12.6.1"},
    {"id": "CVE-2023-44487", "package": "netty", "introduced": "4.1.0", "fixed": "4.1.100"},
]

def version_key(v):
    raise NotImplementedError

def affected(version, adv):
    raise NotImplementedError

for product, components in sboms.items():
    hits = [(adv["id"], name, ver, adv["fixed"]) for name, ver in components for adv in advisories
            if adv["package"] == name and affected(ver, adv)]
    print(f"{product}: {'not affected' if not hits else ''}".rstrip())
    for cve, name, ver, fixed in hits:
        print(f"  {cve}: {name} {ver} -> upgrade to {fixed} or later")
@@expected
billing-api 3.2:
  CVE-2021-44228: log4j-core 2.14.1 -> upgrade to 2.15.0 or later
  CVE-2023-44487: netty 4.1.90 -> upgrade to 4.1.100 or later
portal 1.8: not affected
batch 0.9:
  CVE-2020-36518: jackson-databind 2.9.10 -> upgrade to 2.12.6.1 or later
@@requires
def version_key
def affected
tuple(int(
@@hint
`version_key("2.14.1")` is `tuple(int(p) for p in v.split("."))`. Python compares tuples element by element, so `(2, 9, 10) < (2, 12, 6)`. `affected` is `version_key(introduced) <= version_key(version) < version_key(fixed)`.
@@solution
sboms = {
    "billing-api 3.2": [("log4j-core", "2.14.1"), ("jackson-databind", "2.13.3"), ("netty", "4.1.90")],
    "portal 1.8": [("log4j-core", "2.17.1"), ("spring-web", "5.3.20")],
    "batch 0.9": [("jackson-databind", "2.9.10"), ("netty", "4.1.100")],
}
advisories = [
    {"id": "CVE-2021-44228", "package": "log4j-core", "introduced": "2.0", "fixed": "2.15.0"},
    {"id": "CVE-2020-36518", "package": "jackson-databind", "introduced": "2.0", "fixed": "2.12.6.1"},
    {"id": "CVE-2023-44487", "package": "netty", "introduced": "4.1.0", "fixed": "4.1.100"},
]

def version_key(v):
    return tuple(int(p) for p in v.split("."))

def affected(version, adv):
    return version_key(adv["introduced"]) <= version_key(version) < version_key(adv["fixed"])

for product, components in sboms.items():
    hits = [(adv["id"], name, ver, adv["fixed"]) for name, ver in components for adv in advisories
            if adv["package"] == name and affected(ver, adv)]
    print(f"{product}: {'not affected' if not hits else ''}".rstrip())
    for cve, name, ver, fixed in hits:
        print(f"  {cve}: {name} {ver} -> upgrade to {fixed} or later")

@@lesson sp4-vulnmgmt Vulnerability Management: Scanning, CVSS and Prioritisation
@@text
Domain 4 (security operations) is the largest at 28%, and vulnerability management is a big part of it. You need to know where vulnerability information comes from, how scanners work, how to read a CVSS score, how to prioritise, and how to prove a fix worked. It's also one of the most common entry-level security jobs.

## Identification methods

- **Vulnerability scans**: automated tools (Nessus, Qualys, OpenVAS, Microsoft Defender Vulnerability Management) compare what's installed and configured with a database of known issues.
  - **Credentialed** scans log in and see installed software and configs: far more accurate.
  - **Non-credentialed** scans see what an outsider sees.
  - **Agent-based** scanning on endpoints, **network-based** scanning from a scanner.
- **Application security testing**: **SAST**, **DAST**, **package monitoring** (dependency/SCA tools that track vulnerable libraries).
- **Threat feeds**: **OSINT**, **proprietary/third-party**, **information-sharing organisations** (ISACs), the **dark web** (for leaked credentials).
- **Penetration testing** (authorised, scoped testing; covered in governance) and **bug bounty programmes** / **responsible disclosure**.
- **System/process audits**.

## CVE and CVSS

- **CVE** (Common Vulnerabilities and Exposures): an ID for a specific public vulnerability, e.g. `CVE-2024-3400`.
- **CVSS** (Common Vulnerability Scoring System) scores severity 0–10:

| Score | Severity |
|---|---|
| 0.0 | None |
| 0.1–3.9 | Low |
| 4.0–6.9 | Medium |
| 7.0–8.9 | High |
| 9.0–10.0 | Critical |

The **base** score uses metrics like **attack vector** (network, adjacent, local, physical), **attack complexity**, **privileges required**, **user interaction**, **scope**, and impact on **confidentiality, integrity, availability**. A network-reachable flaw that needs no privileges and no user interaction is the worst case.

Other inputs: **CISA KEV** (Known Exploited Vulnerabilities catalogue: actively exploited, fix first) and **EPSS** (probability of exploitation in the next 30 days).

## Analysis

- **Confirm** results: **false positive** (reported but not real) vs **false negative** (real but missed: the more dangerous one). True positive/negative.
- **Prioritise** by severity **and** context: is it exploited in the wild? Internet-facing? What data does the system hold? Are there compensating controls?
- **Vulnerability classification**, **exposure factor**, **environmental variables** (the CVSS environmental score adjusts for your situation), **industry/organisational impact**, **risk tolerance**.

## Response and remediation

- **Patching** (most common).
- **Insurance** (cyber insurance transfers some risk).
- **Segmentation** and other **compensating controls** when you can't patch yet.
- **Exceptions and exemptions**: documented, approved, time-limited acceptance of a risk (for example a vendor-certified system that can't be updated until the vendor releases a version).

## Validation and reporting

- **Rescanning** after the fix.
- **Audit** and **verification** that the fix applied everywhere.
- **Reporting**: trends over time (mean time to remediate, open criticals by team), SLA compliance.

Typical remediation SLAs: **critical 7–15 days** (or 48 hours if exploited), **high 30 days**, **medium 90 days**, **low** best effort.

## Patch management process

Inventory → identify missing patches → test in a pilot group → schedule (change management) → deploy → verify → report. **Automate** where possible (WSUS/Intune/Autopatch, package managers, configuration management). Track **end-of-life** software that no longer gets patches.

@@quiz
Which scan type gives the most accurate list of missing patches on servers?
* Credentialed scan
- Non-credentialed scan
- Ping sweep
- Passive packet capture
? Logging in reveals installed software and configuration.

@@quiz
A CVSS base score is 8.1. What is its severity rating?
- Medium
* High
- Critical
- Low
? 7.0–8.9 is High; 9.0+ is Critical.

@@quiz
Which finding is more dangerous for a security programme?
- A false positive
* A false negative
- A true negative
- An informational finding
? A real vulnerability went undetected.

@@quiz
A medium-severity flaw is in CISA's Known Exploited Vulnerabilities catalogue, and a critical flaw is on an isolated lab PC with no network. Which should usually be fixed first?
* The actively exploited medium on exposed systems
- The critical on the isolated PC
- Neither: wait for the next cycle
- Whichever has the older CVE number
? Context and exploitation matter, not only the score.

@@quiz
A system can't be patched until the vendor certifies the update. What is the right way to handle it?
- Ignore it
* Document a time-limited exception with compensating controls such as segmentation
- Disable vulnerability scanning on it
- Patch it anyway without testing
? Exceptions must be approved and reviewed.

@@flashcards
credentialed scan :: logs in; accurate
non-credentialed scan :: outsider's view
CVE :: ID for a public vulnerability
CVSS :: 0–10 severity score
CVSS 9.0–10.0 :: Critical
CVSS 7.0–8.9 :: High
CVSS 4.0–6.9 :: Medium
attack vector :: network, adjacent, local, physical
CISA KEV :: actively exploited vulnerabilities
EPSS :: exploit probability score
false positive :: reported but not real
false negative :: real but missed
exception :: approved, time-limited risk acceptance
compensating control :: e.g. segmentation until patched
rescan :: validates remediation
ISAC :: industry information-sharing organisation
SCA :: tracks vulnerable dependencies

@@drill
What is the lowest CVSS score that counts as Critical?
= 9.0
= 9
? 7.0–8.9 is High.

@@drill
What does the C in CVE stand for?
= Common
= common
? Common Vulnerabilities and Exposures.

@@exercise Prioritise scanner findings
Compute a priority score for each finding: CVSS base × 10, +30 if it's in KEV, +20 if the asset is internet-facing, +10 if the asset holds sensitive data, −15 if a compensating control is in place. Assign the remediation SLA from the CVSS severity (`critical` 15 days, `high` 30, `medium` 90, `low` 180), but use 2 days for anything in KEV. Print the findings ordered by priority (highest first).
@@lang python
@@starter
findings = [
    {"cve": "CVE-2026-1001", "host": "web01", "cvss": 9.8, "kev": True, "internet": True, "sensitive": False, "compensating": False},
    {"cve": "CVE-2026-1002", "host": "lab07", "cvss": 9.1, "kev": False, "internet": False, "sensitive": False, "compensating": True},
    {"cve": "CVE-2025-2207", "host": "vpn01", "cvss": 6.5, "kev": True, "internet": True, "sensitive": False, "compensating": False},
    {"cve": "CVE-2026-0420", "host": "db01", "cvss": 7.5, "kev": False, "internet": False, "sensitive": True, "compensating": False},
    {"cve": "CVE-2024-9999", "host": "printer3", "cvss": 3.7, "kev": False, "internet": False, "sensitive": False, "compensating": False},
]

def severity(cvss):
    raise NotImplementedError
@@expected
  148  CVE-2026-1001 on web01: critical (9.8), fix within 2 days
  115  CVE-2025-2207 on vpn01: medium (6.5), fix within 2 days
   85  CVE-2026-0420 on db01: high (7.5), fix within 30 days
   76  CVE-2026-1002 on lab07: critical (9.1), fix within 15 days
   37  CVE-2024-9999 on printer3: low (3.7), fix within 180 days
@@requires
def severity
sorted(
@@hint
`severity` returns `"critical"` for 9.0+, `"high"` for 7.0+, `"medium"` for 4.0+, else `"low"`.
@@solution
findings = [
    {"cve": "CVE-2026-1001", "host": "web01", "cvss": 9.8, "kev": True, "internet": True, "sensitive": False, "compensating": False},
    {"cve": "CVE-2026-1002", "host": "lab07", "cvss": 9.1, "kev": False, "internet": False, "sensitive": False, "compensating": True},
    {"cve": "CVE-2025-2207", "host": "vpn01", "cvss": 6.5, "kev": True, "internet": True, "sensitive": False, "compensating": False},
    {"cve": "CVE-2026-0420", "host": "db01", "cvss": 7.5, "kev": False, "internet": False, "sensitive": True, "compensating": False},
    {"cve": "CVE-2024-9999", "host": "printer3", "cvss": 3.7, "kev": False, "internet": False, "sensitive": False, "compensating": False},
]
SLA = {"critical": 15, "high": 30, "medium": 90, "low": 180}

def severity(cvss):
    if cvss >= 9.0:
        return "critical"
    if cvss >= 7.0:
        return "high"
    if cvss >= 4.0:
        return "medium"
    return "low"

def priority(f):
    score = f["cvss"] * 10
    score += 30 if f["kev"] else 0
    score += 20 if f["internet"] else 0
    score += 10 if f["sensitive"] else 0
    score -= 15 if f["compensating"] else 0
    return score

for f in sorted(findings, key=priority, reverse=True):
    sev = severity(f["cvss"])
    days = 2 if f["kev"] else SLA[sev]
    print(f"{priority(f):5.0f}  {f['cve']} on {f['host']}: {sev} ({f['cvss']}), fix within {days} days")

@@exercise Compare scans and track remediation
Compare last month's and this month's scan results (sets of `(host, cve)`). Print which findings were **fixed**, which are **new**, and which are **still open**, then the remediation rate (fixed ÷ last month's total) as a percentage. Flag any still-open finding that has been open longer than its SLA, using `first_seen` days and `SLA_DAYS`.
@@lang python
@@starter
SLA_DAYS = {"CVE-2026-1001": 15, "CVE-2026-0420": 30, "CVE-2025-2207": 2, "CVE-2024-9999": 180, "CVE-2026-3333": 30}
last_month = {("web01", "CVE-2026-1001"), ("db01", "CVE-2026-0420"), ("vpn01", "CVE-2025-2207"), ("printer3", "CVE-2024-9999")}
this_month = {("db01", "CVE-2026-0420"), ("vpn01", "CVE-2025-2207"), ("printer3", "CVE-2024-9999"), ("web02", "CVE-2026-3333")}
first_seen = {("db01", "CVE-2026-0420"): 41, ("vpn01", "CVE-2025-2207"): 35, ("printer3", "CVE-2024-9999"): 64, ("web02", "CVE-2026-3333"): 3}
@@expected
Fixed: web01 CVE-2026-1001
New: web02 CVE-2026-3333
Still open: db01 CVE-2026-0420
Still open: printer3 CVE-2024-9999
Still open: vpn01 CVE-2025-2207
Remediation rate: 25%
OVERDUE: db01 CVE-2026-0420 open 41 days (SLA 30)
OVERDUE: vpn01 CVE-2025-2207 open 35 days (SLA 2)
@@requires
last_month - this_month
this_month & last_month
@@hint
Set difference and intersection do most of the work. Sort each set before printing so the output is stable.
@@solution
SLA_DAYS = {"CVE-2026-1001": 15, "CVE-2026-0420": 30, "CVE-2025-2207": 2, "CVE-2024-9999": 180, "CVE-2026-3333": 30}
last_month = {("web01", "CVE-2026-1001"), ("db01", "CVE-2026-0420"), ("vpn01", "CVE-2025-2207"), ("printer3", "CVE-2024-9999")}
this_month = {("db01", "CVE-2026-0420"), ("vpn01", "CVE-2025-2207"), ("printer3", "CVE-2024-9999"), ("web02", "CVE-2026-3333")}
first_seen = {("db01", "CVE-2026-0420"): 41, ("vpn01", "CVE-2025-2207"): 35, ("printer3", "CVE-2024-9999"): 64, ("web02", "CVE-2026-3333"): 3}

fixed = last_month - this_month
new = this_month - last_month
still_open = this_month & last_month

for label, items in (("Fixed", fixed), ("New", new), ("Still open", still_open)):
    for host, cve in sorted(items):
        print(f"{label}: {host} {cve}")
print(f"Remediation rate: {len(fixed) / len(last_month):.0%}")

for host, cve in sorted(still_open):
    age = first_seen[(host, cve)]
    if age > SLA_DAYS[cve]:
        print(f"OVERDUE: {host} {cve} open {age} days (SLA {SLA_DAYS[cve]})")

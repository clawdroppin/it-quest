@@lesson cs2-operations-deep Security Operations, Testing and Software Security in Depth (Domains 6–8)
@@text
Domain 7, Security Operations, is about 13% of the exam and the most practical: incident response, investigations and evidence, logging and monitoring, backups and recovery, patch and change management. Domains 6 (Assessment and Testing, 12%) and 8 (Software Development Security, 10%) complete the operational picture. This lesson collects their key processes and rules, from a manager's perspective and strictly defensively.

## Incident response

The CISSP (ISC)² lifecycle: **Detection → Response → Mitigation → Reporting → Recovery → Remediation → Lessons learned**. (NIST SP 800-61: Preparation; Detection and Analysis; Containment, Eradication and Recovery; Post-incident activity.)

- **Preparation** first: policy, team (CSIRT), contacts, tools, playbooks, exercises.
- **Containment** limits damage (isolate hosts, disable accounts) while preserving evidence.
- **Root cause** goes to remediation and problem management; **lessons learned** improve controls.
- Report as required by law/contract (regulators, customers, law enforcement), through legal and management.

## Investigations and evidence

- Types: administrative (internal), criminal (beyond reasonable doubt), civil (preponderance of evidence), regulatory.
- Evidence must be **relevant, material and competent** (legally obtained); best evidence rule (originals), parol evidence rule, hearsay.
- **Chain of custody**: who collected, handled, stored and transferred each item, when, and why; seal and log.
- **Order of volatility** (collect most volatile first): CPU registers and cache → memory (RAM) and routing/ARP tables, process lists → temporary files/swap → disk → remote logs → archival media/backups.
- Work on **forensic copies** verified with hashes, use write blockers, never the original. Don't "power off and look" unless directed: memory evidence is lost.

## Logging and monitoring

SIEM (correlation), SOAR (automated playbooks), UEBA, IDS/IPS, egress monitoring and DLP, continuous monitoring, **threat intelligence** and threat hunting. Clock synchronisation (NTP) is essential for correlating logs. Protect logs from tampering (WORM, separate accounts). **Clipping levels** reduce noise by alerting only above a threshold.

## Backups and recovery

- **Full**: everything; clears the archive bit. **Incremental**: changes since the last backup of any kind; clears the bit; fast backup, slow restore (full + every incremental). **Differential**: changes since the last full; doesn't clear the bit; restore = full + latest differential.
- **3-2-1 rule**: 3 copies, 2 media types, 1 offsite (plus 1 immutable/offline against ransomware). Test restores.
- RAID: 0 (striping, no redundancy), 1 (mirroring), 5 (striping with parity, one disk can fail), 6 (two parity, two disks), 10 (mirror + stripe).
- Recovery sites and BCP from domain 1; **disaster recovery** focuses on IT restoration.

## Operational controls

Least privilege, need to know, separation of duties, two-person integrity, job rotation, privileged account management, **change management** (request, review, approve, test, implement, document), **patch management** (evaluate, test, approve, deploy, verify), configuration management with baselines, media handling and sanitisation (clear, purge, destroy).

## Security assessment and testing (Domain 6)

- Vulnerability assessments (credentialed scans find more, with fewer false positives), **penetration tests** (authorised, scoped, with rules of engagement and written approval; black/grey/white box), code reviews (manual, SAST), DAST, fuzzing, interface testing, misuse case testing, test coverage analysis.
- Audits: internal, external, third party; **SOC 1** (financial reporting controls), **SOC 2** (security, availability, processing integrity, confidentiality, privacy; Type I = design at a point in time, **Type II = operating effectiveness over a period**), SOC 3 (public summary).
- KPIs and KRIs; account management reviews; backup verification; training metrics.

## Software development security (Domain 8)

- SDLC models (waterfall, agile, DevOps/DevSecOps), **shift left**: security requirements, threat modelling (STRIDE), secure coding standards, SAST/DAST/SCA, code signing, SBOM.
- Maturity: **CMMI** levels 1 initial → 2 managed → 3 defined → 4 quantitatively managed → 5 optimising; OWASP SAMM; BSIMM.
- OWASP Top 10 categories (broken access control, cryptographic failures, injection...), input validation, parameterised queries, output encoding, secure session management.
- Assess acquired software and third-party components; API security; databases: ACID, views, aggregation and inference controls.

@@quiz
During an incident, what should usually happen before eradication?
- Lessons learned
* Containment, preserving evidence
- Reporting to the press
- Restoring from backup
? Stop the spread first.

@@quiz
Which should be collected first from a running compromised system?
* Memory contents
- Disk image
- Backup tapes
- Printed logs
? Order of volatility.

@@quiz
Backups are a full on Sunday and differentials Monday to Saturday. The server fails on Thursday. What do you restore?
- Sunday's full and every differential
* Sunday's full and Wednesday's differential
- Only Wednesday's differential
- Sunday's full only
? A differential holds everything since the last full.

@@quiz
Which SOC report gives assurance on how well security controls operated over a period of time?
- SOC 1 Type I
- SOC 2 Type I
* SOC 2 Type II
- SOC 3
? Type I is design at a point in time.

@@quiz
What must be in place before a penetration test starts?
- A SIEM
* Written authorisation from management with scope and rules of engagement
- A SOC 2 report
- A bug bounty
? Without authorisation it's an attack.

@@quiz
Which RAID level survives two simultaneous disk failures with parity?
- RAID 1
- RAID 5
* RAID 6
- RAID 0
? Double parity.

@@quiz
At which CMMI level are processes measured and controlled with quantitative data?
- 2 Managed
- 3 Defined
* 4 Quantitatively managed
- 5 Optimising
? Level 5 focuses on continuous process improvement.

@@flashcards
IR lifecycle (ISC)² :: detect, respond, mitigate, report, recover, remediate, lessons learned
containment :: limit damage, preserve evidence
chain of custody :: who, when, where, why for evidence
order of volatility :: registers, RAM, temp, disk, logs, backups
forensic copy :: hashed, write-blocked
criminal standard :: beyond reasonable doubt
civil standard :: preponderance of evidence
clipping level :: alert above a threshold
SOAR :: automated response playbooks
full backup :: clears archive bit
incremental :: since last backup, clears bit
differential :: since last full, keeps bit
incremental restore :: full + all incrementals
differential restore :: full + latest differential
3-2-1 :: 3 copies, 2 media, 1 offsite
RAID 5 :: one disk can fail
RAID 6 :: two disks can fail
SOC 2 Type II :: effectiveness over a period
credentialed scan :: deeper, fewer false positives
pen test prerequisite :: written authorisation
CMMI 5 :: optimising
STRIDE :: spoofing, tampering, repudiation, info disclosure, DoS, elevation
SCA :: third-party component analysis

@@drill
Which backup type copies changes since the last full backup and does not clear the archive bit?
= differential
= differential backup
? Incremental clears it.

@@drill
Which SOC 2 report type covers operating effectiveness over a period?
= Type II
= Type 2
= type ii
? Type I is a point in time.

@@exercise Build a restore chain
For a backup schedule over two weeks, compute which backup sets must be restored to recover to the end of a given day: the latest full on or before that day, then either every incremental after it (up to that day) or the latest differential after it. Mixed schedules are possible: after a full, differentials hold everything since that full; incrementals hold everything since the previous backup of any type. Print each restore chain and how many sets it needs.
@@lang python
@@starter
schedules = {
    "full + incrementals": {0: "full", 1: "inc", 2: "inc", 3: "inc", 4: "inc", 5: "inc", 6: "inc", 7: "full", 8: "inc", 9: "inc", 10: "inc"},
    "full + differentials": {0: "full", 1: "diff", 2: "diff", 3: "diff", 4: "diff", 5: "diff", 6: "diff", 7: "full", 8: "diff", 9: "diff", 10: "diff"},
    "mixed": {0: "full", 1: "inc", 2: "inc", 3: "diff", 4: "inc", 5: "inc", 6: "diff", 7: "inc"},
}
targets = [5, 9, 7]

def chain(schedule, day):
    raise NotImplementedError
@@expected
full + incrementals, day 5: full d0 + inc d1 + inc d2 + inc d3 + inc d4 + inc d5 (6 sets)
full + incrementals, day 9: full d7 + inc d8 + inc d9 (3 sets)
full + incrementals, day 7: full d7 (1 set)
full + differentials, day 5: full d0 + diff d5 (2 sets)
full + differentials, day 9: full d7 + diff d9 (2 sets)
full + differentials, day 7: full d7 (1 set)
mixed, day 5: full d0 + diff d3 + inc d4 + inc d5 (4 sets)
mixed, day 7: full d0 + diff d6 + inc d7 (3 sets)
@@requires
def chain
"diff"
"inc"
sorted(
@@hint
Start from the latest full ≤ day. Walk forward: a differential replaces everything after the full (reset the chain to `[full, diff]`), an incremental is appended.
@@solution
schedules = {
    "full + incrementals": {0: "full", 1: "inc", 2: "inc", 3: "inc", 4: "inc", 5: "inc", 6: "inc", 7: "full", 8: "inc", 9: "inc", 10: "inc"},
    "full + differentials": {0: "full", 1: "diff", 2: "diff", 3: "diff", 4: "diff", 5: "diff", 6: "diff", 7: "full", 8: "diff", 9: "diff", 10: "diff"},
    "mixed": {0: "full", 1: "inc", 2: "inc", 3: "diff", 4: "inc", 5: "inc", 6: "diff", 7: "inc"},
}
targets = [5, 9, 7]

def chain(schedule, day):
    full = max(d for d, kind in schedule.items() if kind == "full" and d <= day)
    result = [(full, "full")]
    for d in sorted(schedule):
        if full < d <= day:
            if schedule[d] == "diff":
                result = [(full, "full"), (d, "diff")]
            elif schedule[d] == "inc":
                result.append((d, "inc"))
    return result

for name, schedule in schedules.items():
    for day in targets:
        if day not in schedule:
            continue
        sets = chain(schedule, day)
        print(f"{name}, day {day}: {' + '.join(f'{k} d{d}' for d, k in sets)} ({len(sets)} set{'s' if len(sets) > 1 else ''})")

@@exercise Order evidence collection and check the chain of custody
Sort evidence sources by order of volatility, then validate a chain-of-custody log: every transfer must start with the person currently holding the item, timestamps must increase, and the SHA-256 recorded at each step must equal the hash taken at collection. Print the collection order and any custody problems.
@@lang python
@@starter
import hashlib

VOLATILITY = ["cpu registers and cache", "memory", "network state", "running processes", "temporary files", "disk", "remote logs", "backups"]
sources = ["disk", "remote logs", "memory", "running processes", "backups", "network state"]
image = b"forensic image of laptop LT-0423"
good = hashlib.sha256(image).hexdigest()
custody = [
    ("2026-11-02 09:10", None, "Sato", "collected at desk 4F-12", good),
    ("2026-11-02 11:00", "Sato", "Evidence locker", "sealed bag E-77", good),
    ("2026-11-03 08:30", "Evidence locker", "Tanaka", "checked out for analysis", good),
    ("2026-11-03 08:00", "Tanaka", "Lab", "imaging", good),
    ("2026-11-04 17:00", "Suzuki", "Evidence locker", "returned", hashlib.sha256(image + b"!").hexdigest()),
]

def check_custody(log):
    raise NotImplementedError
@@expected
Collect in order: memory -> network state -> running processes -> disk -> remote logs -> backups
Chain of custody: 3 problem(s)
  step 4: time 2026-11-03 08:00 is not after 2026-11-03 08:30
  step 5: transfer from Suzuki, but Lab holds the item
  step 5: hash changed, integrity can't be shown
@@requires
def check_custody
VOLATILITY.index
hashlib.sha256(
@@hint
Track `holder` (the `to` of the previous entry) and `last_time`; ISO-style timestamps compare correctly as strings. Report the step number with each problem.
@@solution
import hashlib

VOLATILITY = ["cpu registers and cache", "memory", "network state", "running processes", "temporary files", "disk", "remote logs", "backups"]
sources = ["disk", "remote logs", "memory", "running processes", "backups", "network state"]
image = b"forensic image of laptop LT-0423"
good = hashlib.sha256(image).hexdigest()
custody = [
    ("2026-11-02 09:10", None, "Sato", "collected at desk 4F-12", good),
    ("2026-11-02 11:00", "Sato", "Evidence locker", "sealed bag E-77", good),
    ("2026-11-03 08:30", "Evidence locker", "Tanaka", "checked out for analysis", good),
    ("2026-11-03 08:00", "Tanaka", "Lab", "imaging", good),
    ("2026-11-04 17:00", "Suzuki", "Evidence locker", "returned", hashlib.sha256(image + b"!").hexdigest()),
]

def check_custody(log):
    problems = []
    holder, last_time, original = None, "", log[0][4]
    for i, (time, src, dst, note, digest) in enumerate(log, 1):
        if src != holder:
            problems.append(f"step {i}: transfer from {src}, but {holder} holds the item")
        if time <= last_time:
            problems.append(f"step {i}: time {time} is not after {last_time}")
        if digest != original:
            problems.append(f"step {i}: hash changed, integrity can't be shown")
        holder, last_time = dst, time
    return problems

print("Collect in order:", " -> ".join(sorted(sources, key=VOLATILITY.index)))
found = check_custody(custody)
print("Chain of custody:", "intact" if not found else f"{len(found)} problem(s)")
for p in found:
    print("  " + p)

@@lesson sp4-ir Incident Response, Digital Forensics and Investigations
@@text
When prevention fails, a practised response limits the damage. Security+ asks for the incident response phases in order, the difference between containment and eradication, how evidence is collected and preserved, and which data sources answer which questions during an investigation.

## Incident response process

1. **Preparation**: plans, playbooks, contacts, tools, logging in place, training, jump bags.
2. **Detection**: alerts, user reports, threat intel; decide whether it's an incident.
3. **Analysis**: scope, severity, affected systems, indicators; prioritise.
4. **Containment**: stop the spread. **Short-term** (isolate a host, block an IP, disable an account) then **long-term** (rebuild in a clean segment). Balance evidence preservation and business impact.
5. **Eradication**: remove the cause: malware, attacker accounts, backdoors; patch the vulnerability.
6. **Recovery**: restore systems from clean backups, monitor closely, return to normal.
7. **Lessons learned**: a blameless review within about two weeks: what happened, what worked, what to change.

(NIST SP 800-61 groups these as Preparation; Detection & Analysis; Containment, Eradication & Recovery; Post-Incident Activity.)

**Training and testing**: **tabletop exercises** (discussion), **simulations** (realistic drills). **Root cause analysis** answers why it happened (the 5 whys). **Threat hunting** proactively searches for attackers who evaded alerts, based on a hypothesis.

## Playbooks and roles

A **playbook** is a step-by-step response for one scenario (ransomware, phishing, lost laptop, BEC). The **incident response team** (CSIRT) includes a lead, analysts, IT, legal, communications, HR, management. Know who can declare an incident, who talks to customers, regulators and police, and the **notification deadlines** (GDPR: 72 hours to the regulator).

## Digital forensics

- **Legal hold**: preserve all relevant data when litigation is expected; suspend deletion.
- **Chain of custody**: a documented record of who handled the evidence, when, and why, from collection to court. A break in the chain can make evidence inadmissible.
- **Acquisition**: make forensic images with a **write blocker**; **hash** (SHA-256) the original and the copy to prove they're identical.
- **Order of volatility** (collect the most volatile first):
  1. CPU registers and cache
  2. **RAM** (running processes, network connections, encryption keys)
  3. Swap/page file, temporary files
  4. **Disk**
  5. Remote logs and monitoring data
  6. Archives and backups
- **Preservation**, **reporting**, **e-discovery** (finding electronic evidence for legal cases).
- Don't power off a running system before capturing memory if the investigation needs it; don't investigate on the original image.
- **Timeline** analysis combines timestamps from logs, file systems and memory (with time zones normalised).

## Data sources for investigations

| Source | Answers |
|---|---|
| **Firewall logs** | which connections were allowed/blocked |
| **Application logs** | what users did in the app |
| **Endpoint logs / EDR** | processes, file changes, persistence |
| **OS security logs** | logons (4624/4625), privilege use, account changes |
| **IPS/IDS logs** | detected attacks |
| **Network logs**, **NetFlow** | who talked to whom, how much data |
| **Metadata** | email headers, file creation times, document authors |
| **Vulnerability scans** | was the system exposed? |
| **Automated reports**, **dashboards** | trends and summaries |
| **Packet captures** | the full conversation |

## Indicators of malicious activity

Account lockouts, concurrent session usage, blocked content, **impossible travel**, resource consumption spikes, resource inaccessibility, **out-of-cycle logging** (logs at strange times), **missing logs** (attackers delete them), **published/documented** indicators (threat intel IoCs).

@@quiz
What's the correct order of the incident response phases after analysis?
* Containment, eradication, recovery, lessons learned
- Eradication, containment, lessons learned, recovery
- Recovery, containment, eradication, lessons learned
- Lessons learned, containment, recovery, eradication
? Stop the spread before removing the cause.

@@quiz
During an investigation, which should be collected first?
- Disk image
- Backups
* RAM contents
- Remote logs
? Order of volatility: most volatile first.

@@quiz
What proves that a forensic copy is identical to the original drive?
- The file sizes match
* Matching cryptographic hashes of both
- The chain of custody form
- A write blocker label
? Hash before and after imaging.

@@quiz
A lawsuit is expected, so the company must stop deleting relevant emails. What is this?
- Chain of custody
* Legal hold
- E-discovery
- Data retention policy
? Preserve everything relevant.

@@quiz
Isolating an infected laptop from the network is part of which phase?
- Preparation
- Eradication
* Containment
- Recovery
? Eradication removes the malware itself.

@@flashcards
IR phases :: preparation, detection, analysis, containment, eradication, recovery, lessons learned
containment :: stop the spread
eradication :: remove the cause
recovery :: restore and monitor
lessons learned :: blameless post-incident review
tabletop exercise :: discussion-based drill
threat hunting :: proactive hypothesis-driven search
playbook :: steps for one scenario
legal hold :: preserve data for litigation
chain of custody :: who handled evidence, when, why
write blocker :: prevents changes during imaging
order of volatility :: registers, RAM, swap, disk, remote logs, backups
e-discovery :: finding electronic evidence
root cause analysis :: why it happened
impossible travel :: logins too far apart
missing logs :: possible log tampering
GDPR breach notice :: 72 hours

@@drill
Which IR phase removes malware and attacker accounts?
= eradication
? Containment comes before it.

@@drill
Which device prevents writes to evidence drives during imaging?
= write blocker
= a write blocker
? Hardware or software.

@@exercise Build an incident timeline
Events from several sources use different time zones (`+09:00` Tokyo, `+00:00` UTC, `-05:00` New York). Convert all to UTC with `datetime.fromisoformat`, sort them, print a timeline, and print the dwell time between the first malicious event and the first detection (`kind` is `detect`).
@@lang python
@@starter
from datetime import datetime, timezone

events = [
    ("2026-11-03T22:14:00+09:00", "email-gw", "malicious", "Phishing email delivered to ken"),
    ("2026-11-03T13:20:00+00:00", "edr", "malicious", "Macro spawned powershell on PC-KEN"),
    ("2026-11-03T08:45:00-05:00", "firewall", "malicious", "PC-KEN connects to 203.0.113.66:443"),
    ("2026-11-04T09:05:00+09:00", "siem", "detect", "Alert: beaconing from PC-KEN"),
    ("2026-11-03T23:30:00+00:00", "ad", "malicious", "ken account added to Remote Desktop Users"),
    ("2026-11-04T09:20:00+09:00", "soc", "response", "PC-KEN isolated"),
]
@@expected
2026-11-03 13:14 UTC [email-gw] Phishing email delivered to ken
2026-11-03 13:20 UTC [edr] Macro spawned powershell on PC-KEN
2026-11-03 13:45 UTC [firewall] PC-KEN connects to 203.0.113.66:443
2026-11-03 23:30 UTC [ad] ken account added to Remote Desktop Users
2026-11-04 00:05 UTC [siem] Alert: beaconing from PC-KEN
2026-11-04 00:20 UTC [soc] PC-KEN isolated
Dwell time: 10:51:00
@@requires
fromisoformat(
astimezone(
@@hint
`datetime.fromisoformat(ts).astimezone(timezone.utc)`; dwell time is the difference between two datetimes (a `timedelta`).
@@solution
from datetime import datetime, timezone

events = [
    ("2026-11-03T22:14:00+09:00", "email-gw", "malicious", "Phishing email delivered to ken"),
    ("2026-11-03T13:20:00+00:00", "edr", "malicious", "Macro spawned powershell on PC-KEN"),
    ("2026-11-03T08:45:00-05:00", "firewall", "malicious", "PC-KEN connects to 203.0.113.66:443"),
    ("2026-11-04T09:05:00+09:00", "siem", "detect", "Alert: beaconing from PC-KEN"),
    ("2026-11-03T23:30:00+00:00", "ad", "malicious", "ken account added to Remote Desktop Users"),
    ("2026-11-04T09:20:00+09:00", "soc", "response", "PC-KEN isolated"),
]

timeline = sorted((datetime.fromisoformat(ts).astimezone(timezone.utc), src, kind, text) for ts, src, kind, text in events)
for when, src, kind, text in timeline:
    print(f"{when:%Y-%m-%d %H:%M} UTC [{src}] {text}")

first_bad = next(t for t, _, kind, _ in timeline if kind == "malicious")
first_detect = next(t for t, _, kind, _ in timeline if kind == "detect")
print("Dwell time:", first_detect - first_bad)

@@exercise Verify evidence integrity and chain of custody
Each evidence item has the SHA-256 hash recorded at acquisition and a custody log of `(time, from, to)` transfers. Recompute the hash of the item's content (given as bytes), and check the custody chain: each transfer's `from` must equal the previous transfer's `to`, starting from the collector. Print `intact` or the problems for each item.
@@lang python
@@starter
import hashlib

evidence = [
    {"id": "EV-01", "content": b"disk image block data v1", "collector": "analyst-ana",
     "recorded": hashlib.sha256(b"disk image block data v1").hexdigest(),
     "custody": [("09:00", "analyst-ana", "locker-3"), ("13:00", "locker-3", "examiner-ken")]},
    {"id": "EV-02", "content": b"memory dump modified", "collector": "analyst-ana",
     "recorded": hashlib.sha256(b"memory dump original").hexdigest(),
     "custody": [("09:10", "analyst-ana", "locker-3")]},
    {"id": "EV-03", "content": b"phone extraction", "collector": "analyst-mei",
     "recorded": hashlib.sha256(b"phone extraction").hexdigest(),
     "custody": [("10:00", "analyst-mei", "locker-1"), ("15:00", "locker-3", "examiner-ken")]},
]
@@expected
EV-01: intact, now with examiner-ken
EV-02: hash mismatch: evidence altered
EV-03: custody gap at 15:00: locker-3 handed over but locker-1 held it
@@requires
hashlib.sha256(
holder
@@hint
Track `holder = item["collector"]` and update it to each transfer's `to`; a mismatch means a gap in custody.
@@solution
import hashlib

evidence = [
    {"id": "EV-01", "content": b"disk image block data v1", "collector": "analyst-ana",
     "recorded": hashlib.sha256(b"disk image block data v1").hexdigest(),
     "custody": [("09:00", "analyst-ana", "locker-3"), ("13:00", "locker-3", "examiner-ken")]},
    {"id": "EV-02", "content": b"memory dump modified", "collector": "analyst-ana",
     "recorded": hashlib.sha256(b"memory dump original").hexdigest(),
     "custody": [("09:10", "analyst-ana", "locker-3")]},
    {"id": "EV-03", "content": b"phone extraction", "collector": "analyst-mei",
     "recorded": hashlib.sha256(b"phone extraction").hexdigest(),
     "custody": [("10:00", "analyst-mei", "locker-1"), ("15:00", "locker-3", "examiner-ken")]},
]

for item in evidence:
    problems = []
    if hashlib.sha256(item["content"]).hexdigest() != item["recorded"]:
        problems.append("hash mismatch: evidence altered")
    holder = item["collector"]
    for time, giver, receiver in item["custody"]:
        if giver != holder:
            problems.append(f"custody gap at {time}: {giver} handed over but {holder} held it")
        holder = receiver
    print(f"{item['id']}: " + ("; ".join(problems) if problems else f"intact, now with {holder}"))

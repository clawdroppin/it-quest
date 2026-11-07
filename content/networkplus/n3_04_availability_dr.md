@@lesson n3-ha High Availability, Disaster Recovery and Documentation
@@text
Network+ domain 3 also covers keeping the network running when things fail, recovering when they fail badly, and the documentation and processes that make both possible. Expect calculations (MTBF, MTTR, availability) and scenario questions about site types and redundancy.

## Measuring reliability

- **MTBF** (mean time between failures): average running time between failures of a repairable system. Higher is better.
- **MTTF** (mean time to failure): for things you replace, not repair (a disk).
- **MTTR** (mean time to repair/recover): average time to fix. Lower is better.
- **Availability** ≈ MTBF ÷ (MTBF + MTTR).
- **RPO** (recovery point objective): the maximum acceptable **data loss**, measured in time (RPO 1 h → back up or replicate at least hourly).
- **RTO** (recovery time objective): the maximum acceptable **downtime** before the service is back.

## Redundancy

- **Device**: two firewalls, two core switches (stacked or with **FHRP**), dual supervisors.
- **Link**: LACP bundles, two ISPs (**multihoming**, with BGP or SD-WAN), diverse physical paths (two fibre routes that don't share a trench).
- **Power**: dual power supplies on different circuits, **UPS** (battery for minutes; **online/double-conversion** gives the cleanest power), **generators** for hours/days, **PDUs**.
- **Active-active**: both units carry traffic (more capacity, but each must handle the full load if one fails). **Active-passive**: the standby waits.
- Avoid **single points of failure** (SPOF): the one switch, link, server or person everything depends on.
- **Clustering** and **load balancing** for servers; **NIC teaming**.

## Disaster recovery sites

| Site | Ready in | Cost |
|---|---|---|
| **Hot** | minutes–hours: equipment running, data replicated | highest |
| **Warm** | hours–days: hardware in place, data needs restoring | medium |
| **Cold** | days–weeks: space and power only | lowest |
| **Cloud site** | variable: infrastructure as code rebuilds on demand | pay as you go |

## Backups and testing

- **Full**, **incremental** (since the last backup of any kind; fast backup, slow restore), **differential** (since the last full; restore needs full + latest differential).
- Back up **device configs** too (and keep **golden configs**/known-good baselines).
- Testing DR plans: **tabletop exercise** (talk through a scenario), **validation tests**, **failover tests**, full **simulation**.

## Documentation

- **Physical diagrams** (cabling, racks: **rack diagrams** with U positions), **logical diagrams** (VLANs, subnets, routing), **floor plans**, **wiring/port maps**, **patch panel labelling**.
- **IPAM**, **asset inventory** (hardware, software, licences, warranty/support dates), **site surveys**.
- **Baseline configurations**, **SOPs**.
- **Agreements**: **SLA** (service levels), **MOU**, **NDA**, **SOW**, **MSA**.
- **Life-cycle management**: **EOL/EOS** (end of life/support) dates, **software management** (patches, firmware, upgrades), **decommissioning** (wipe configs, update inventory).

## Change management

1. Request (what, why, risk, rollback plan).
2. Review and approval (**CAB**: change advisory board).
3. Scheduled **maintenance window**.
4. Implement, test, document.
5. Close (or **roll back** if tests fail).

Emergency changes still get documented afterwards.

## Policies you should recognise

**Acceptable use policy (AUP)**, **password policy**, **BYOD policy**, **remote access policy**, **incident response plan**, **business continuity plan (BCP)** vs **disaster recovery plan (DRP)** (BCP keeps the business running; DRP restores IT), **data loss prevention**, **privileged access**.

@@quiz
A business can tolerate losing at most 15 minutes of order data. Which objective is this?
* RPO
- RTO
- MTBF
- MTTR
? RPO = data loss; RTO = downtime.

@@quiz
Which DR site has hardware in place but needs data restored before use?
- Hot site
* Warm site
- Cold site
- Colocation
? Hot sites have current data; cold sites have only space and power.

@@quiz
Restoring from backups requires the last full backup plus every backup since. Which type was used?
- Full
- Differential
* Incremental
- Snapshot only
? Differential needs only the full plus the latest differential.

@@quiz
Two core switches both forward traffic; if one fails the other carries everything. What design is this?
* Active-active
- Active-passive
- Cold standby
- Single point of failure
? Size each unit for the full load.

@@quiz
Which document defines the service levels a provider must meet, such as 99.95% uptime and 4-hour response?
- NDA
- AUP
* SLA
- MOU
? Service level agreement.

@@flashcards
MTBF :: average time between failures
MTTR :: average time to repair
availability :: MTBF ÷ (MTBF + MTTR)
RPO :: maximum data loss
RTO :: maximum downtime
hot / warm / cold site :: ready now / hardware only / space only
incremental :: since last backup; slow restore
differential :: since last full; full + latest diff to restore
SPOF :: single point of failure
multihoming :: two or more ISPs
online UPS :: double conversion, cleanest power
tabletop exercise :: talk through a DR scenario
CAB :: change advisory board
BCP vs DRP :: keep business running vs restore IT
EOL / EOS :: end of life / end of support
SLA :: service level agreement

@@drill
A device has an MTBF of 9,900 hours and an MTTR of 100 hours. What is its availability in percent?
= 99
= 99%
? 9900 / (9900 + 100).

@@drill
Which DR site type is the cheapest but slowest to bring online?
= cold
= cold site
? Space, power and connectivity only.

@@exercise Availability of a chain
Components in **series** must all work, so availability multiplies. **Redundant pairs** (parallel) fail only if both fail: `1 - (1 - a) ** 2`. Compute each component's availability from its MTBF and MTTR, then the end-to-end availability of the path (each element is a single component or a redundant pair), and the expected downtime per year in hours (8,766 h/year). Print each element and the totals.
@@lang python
@@starter
HOURS_PER_YEAR = 8766
components = {
    "ISP link": (2000, 8),
    "Firewall": (50000, 4),
    "Core switch": (100000, 4),
    "Access switch": (80000, 2),
}
path = [("ISP link", 2), ("Firewall", 2), ("Core switch", 2), ("Access switch", 1)]
@@expected
ISP link x2: single 99.60159%, element 99.9984127%
Firewall x2: single 99.99200%, element 99.9999994%
Core switch x2: single 99.99600%, element 99.9999998%
Access switch x1: single 99.99750%, element 99.9975001%
End to end: 99.9959120%
Expected downtime: 0.36 h/year
@@requires
** count
HOURS_PER_YEAR
@@hint
`a = mtbf / (mtbf + mttr)`, `element = 1 - (1 - a) ** count`, then multiply the elements together.
@@solution
HOURS_PER_YEAR = 8766
components = {
    "ISP link": (2000, 8),
    "Firewall": (50000, 4),
    "Core switch": (100000, 4),
    "Access switch": (80000, 2),
}
path = [("ISP link", 2), ("Firewall", 2), ("Core switch", 2), ("Access switch", 1)]

total = 1.0
for name, count in path:
    mtbf, mttr = components[name]
    a = mtbf / (mtbf + mttr)
    element = 1 - (1 - a) ** count
    total *= element
    print(f"{name} x{count}: single {a:.5%}, element {element:.7%}")
print(f"End to end: {total:.7%}")
print(f"Expected downtime: {(1 - total) * HOURS_PER_YEAR:.2f} h/year")

@@exercise Choose a DR site and backup schedule
For each service, choose the cheapest DR site whose recovery time meets the RTO (`hot` ≤ 1 h, `warm` ≤ 24 h, `cold` ≤ 168 h), and the backup interval: the largest of `[15 min, 1 h, 4 h, 24 h]` (in minutes) that doesn't exceed the RPO. If no site meets the RTO, print `needs active-active`.
@@lang python
@@starter
SITES = [("cold", 168), ("warm", 24), ("hot", 1)]
INTERVALS = [15, 60, 240, 1440]
services = [
    ("Online shop", 0.25, 5),
    ("ERP", 4, 60),
    ("HR portal", 48, 1440),
    ("File archive", 120, 2880),
    ("Email", 8, 240),
]
@@expected
Online shop (RTO 0.25 h, RPO 5 min): needs active-active, needs continuous replication
ERP (RTO 4 h, RPO 60 min): hot site, back up every 60 min
HR portal (RTO 48 h, RPO 1440 min): warm site, back up every 1440 min
File archive (RTO 120 h, RPO 2880 min): warm site, back up every 1440 min
Email (RTO 8 h, RPO 240 min): hot site, back up every 240 min
@@requires
SITES
INTERVALS
next(
@@hint
`next((name for name, hours in SITES if hours <= rto), None)` gives the cheapest suitable site because SITES is cheapest first. For the interval, `max((i for i in INTERVALS if i <= rpo), default=None)`.
@@solution
SITES = [("cold", 168), ("warm", 24), ("hot", 1)]
INTERVALS = [15, 60, 240, 1440]
services = [
    ("Online shop", 0.25, 5),
    ("ERP", 4, 60),
    ("HR portal", 48, 1440),
    ("File archive", 120, 2880),
    ("Email", 8, 240),
]

for name, rto_hours, rpo_minutes in services:
    site = next((s for s, hours in SITES if hours <= rto_hours), None)
    interval = max((i for i in INTERVALS if i <= rpo_minutes), default=None)
    site_text = f"{site} site" if site else "needs active-active"
    backup_text = f"back up every {interval} min" if interval else "needs continuous replication"
    print(f"{name} (RTO {rto_hours} h, RPO {rpo_minutes} min): {site_text}, {backup_text}")

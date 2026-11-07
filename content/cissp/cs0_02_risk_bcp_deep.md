@@lesson cs0-risk-bcp-deep Risk, Continuity and Governance in Depth (Domain 1)
@@text
Domain 1, Security and Risk Management, is about 16% of the CISSP and sets the mindset for the whole exam: you are an adviser to senior management, protecting the business, and human safety always comes first. This lesson goes deeper into quantitative and qualitative risk analysis, control selection, frameworks, the BCP process and its timing metrics, and supply chain and personnel security.

## Quantitative risk analysis

| Term | Formula |
|---|---|
| **AV** asset value | money |
| **EF** exposure factor | % of the asset lost in one event |
| **SLE** single loss expectancy | AV × EF |
| **ARO** annualised rate of occurrence | events per year (once in 50 years = 0.02) |
| **ALE** annualised loss expectancy | SLE × ARO |
| **Safeguard value** | ALE before − ALE after − annual cost of the safeguard |

A safeguard is worth it when its value is **positive**. Never spend more on a control than the asset is worth. Conceptually, **total risk** = threats × vulnerabilities × asset value, and **residual risk** = total risk × controls gap: the risk left after controls, which management must **accept** formally.

**Qualitative** analysis ranks risks with scales (likelihood × impact: low/medium/high), using techniques such as the **Delphi technique** (anonymous expert rounds), surveys and workshops. Most organisations use both.

## Risk responses

- **Mitigate/reduce** (controls), **transfer/share** (insurance, outsourcing: accountability stays with you), **avoid** (stop the activity), **accept** (documented decision by an owner with authority, within **risk appetite**). **Rejecting/ignoring** risk is never acceptable.
- Control types by function: **preventive, detective, corrective, deterrent, recovery, compensating, directive**; by category: **administrative** (policies, training), **technical/logical** (encryption, ACLs), **physical** (locks, guards).

## Frameworks and governance

- **NIST RMF** (SP 800-37): prepare, categorise, select, implement, assess, authorise, monitor. **NIST CSF 2.0**: Govern, Identify, Protect, Detect, Respond, Recover.
- **ISO/IEC 27001** (ISMS requirements, certifiable) and **27002** (controls guidance); **COBIT** (IT governance); **ISO 31000** (risk management).
- **Due care** = doing the right thing (acting as a prudent person would); **due diligence** = investigating and verifying (knowing what the right thing is and that it's done).
- Policy hierarchy: **policy** (mandatory, high level) → **standards** (mandatory specifics) → **baselines** (minimum configuration) → **guidelines** (recommended) → **procedures** (step by step).
- Senior management holds **ultimate responsibility** for security.

## Business continuity planning

Steps: project scope and planning (senior management support, BCP team) → **business impact analysis** → continuity strategy → plan approval and implementation → training, testing, maintenance.

| Metric | Meaning |
|---|---|
| **MTD** / MAO | maximum tolerable downtime before the business suffers unacceptable harm |
| **RTO** | target time to restore the function (must be ≤ MTD) |
| **WRT** | work recovery time: verifying data and getting back to business after systems are up |
| **RPO** | maximum acceptable data loss, measured in time |

**RTO + WRT ≤ MTD**. The backup interval must be ≤ RPO. Recovery sites: **hot** (minutes to hours, expensive), **warm** (hours to days), **cold** (days to weeks), mobile, reciprocal agreements, cloud. Test types from least to most disruptive: **read-through/checklist → structured walkthrough (tabletop) → simulation → parallel → full interruption**.

## People and suppliers

- Hiring: background checks, NDAs; during employment: **least privilege, separation of duties, job rotation, mandatory vacations** (detect fraud); termination: disable access **at or before** notification, collect assets, exit interview.
- Supply chain: assess suppliers, contractual security requirements (right to audit, breach notification), **SLAs**, SOC 2 Type II reports, software bills of materials (SBOM), hardware provenance.
- Security awareness, training and education: awareness (everyone), training (job skills), education (deep understanding).

## Ethics

(ISC)² canons in order: **protect society**, the common good, necessary public trust and confidence, and the infrastructure; act honourably, honestly, justly, responsibly and legally; provide diligent and competent service to principals; advance and protect the profession.

@@quiz
A server worth $200,000 would lose 25% of its value in a flood expected once every 20 years. What is the ALE?
- $50,000
* $2,500
- $10,000
- $5,000
? SLE = 50,000; ARO = 0.05; ALE = 2,500.

@@quiz
A control costs $8,000 a year and reduces ALE from $30,000 to $12,000. What is its value?
- $18,000
* $10,000
- $22,000
- −$8,000
? 30,000 − 12,000 − 8,000.

@@quiz
The business buys cyber insurance for a breach risk. Which response is this, and who remains accountable?
- Avoidance; the insurer
* Transference; the organisation remains accountable
- Acceptance; the insurer
- Mitigation; the board
? Accountability can't be transferred.

@@quiz
MTD for order processing is 8 hours and work recovery time is 3 hours. What is the maximum RTO?
- 8 hours
* 5 hours
- 11 hours
- 3 hours
? RTO + WRT ≤ MTD.

@@quiz
Which BCP test is the least disruptive?
* Read-through (checklist)
- Simulation
- Parallel test
- Full interruption
? Then walkthrough, simulation, parallel, full interruption.

@@quiz
Which administrative control helps detect fraud by forcing someone else to perform a person's duties?
- Least privilege
* Mandatory vacations (and job rotation)
- Need to know
- Background checks
? Fraud often needs continuous presence to hide.

@@quiz
What is the first (ISC)² canon?
- Act honourably and legally
* Protect society, the common good, necessary public trust and confidence, and the infrastructure
- Provide diligent service to principals
- Advance the profession
? The canons are applied in order.

@@flashcards
SLE :: AV × EF
ALE :: SLE × ARO
ARO once in 50 years :: 0.02
safeguard value :: ALE before − ALE after − cost
residual risk :: left after controls, formally accepted
Delphi technique :: anonymous expert consensus
transfer risk :: accountability stays
reject risk :: never acceptable
compensating control :: alternative when primary isn't possible
due care :: doing the right thing
due diligence :: verifying it's done
policy :: mandatory, high level
baseline :: minimum configuration
guideline :: recommended, optional
NIST CSF 2.0 functions :: Govern, Identify, Protect, Detect, Respond, Recover
MTD :: maximum tolerable downtime
RTO + WRT :: must be ≤ MTD
RPO :: backup interval must be ≤
hot site :: ready in hours
BCP tests :: read-through, walkthrough, simulation, parallel, full
mandatory vacation :: detect fraud
first canon :: protect society

@@drill
What is the formula for ALE?
= SLE × ARO
= SLE x ARO
= SLE * ARO
= ALE = SLE × ARO
? SLE = AV × EF.

@@drill
Which BCP test runs the recovery site in parallel without stopping production?
= parallel
= parallel test
? Full interruption actually stops primary operations.

@@exercise Choose safeguards with ALE
For each risk, compute SLE and ALE, then the value of each candidate safeguard (ALE before − ALE after − annual cost), where a safeguard can change the ARO or the EF. Choose the best positive-value safeguard; with none, recommend **accept** if the ALE is within the appetite, otherwise **transfer**. Print the register, then the totals before and after.
@@lang python
@@starter
APPETITE = 20_000
risks = [
    {"id": "R1", "asset": "customer DB", "av": 2_000_000, "ef": 0.4, "aro": 0.1,
     "controls": [("DB activity monitoring", 25_000, {"aro": 0.05}), ("tokenise card data", 60_000, {"ef": 0.05})]},
    {"id": "R2", "asset": "file servers", "av": 500_000, "ef": 0.8, "aro": 0.3,
     "controls": [("immutable backups", 15_000, {"ef": 0.1}), ("EDR", 40_000, {"aro": 0.15})]},
    {"id": "R3", "asset": "HQ building", "av": 10_000_000, "ef": 0.5, "aro": 0.01,
     "controls": [("seismic retrofit", 120_000, {"ef": 0.1})]},
    {"id": "R4", "asset": "laptops", "av": 2_000, "ef": 1.0, "aro": 6,
     "controls": [("cable locks", 3_000, {"aro": 4})]},
]

def ale(av, ef, aro):
    raise NotImplementedError
@@expected
R1 customer DB   SLE $     800,000 ARO 0.1   ALE $   80,000 -> mitigate with DB activity monitoring (value $15,000/yr)
R2 file servers  SLE $     400,000 ARO 0.3   ALE $  120,000 -> mitigate with immutable backups (value $90,000/yr)
R3 HQ building   SLE $   5,000,000 ARO 0.01  ALE $   50,000 -> transfer (insurance) or avoid
R4 laptops       SLE $       2,000 ARO 6     ALE $   12,000 -> mitigate with cable locks (value $1,000/yr)
Annual cost of risk: $262,000 before, $156,000 after (including control costs)
@@requires
def ale
def safeguard_value
APPETITE
@@hint
`safeguard_value(risk, cost, change)` recomputes ALE with `{**{"ef": risk["ef"], "aro": risk["aro"]}, **change}`. Pick `max` by value among controls with value > 0.
@@solution
APPETITE = 20_000
risks = [
    {"id": "R1", "asset": "customer DB", "av": 2_000_000, "ef": 0.4, "aro": 0.1,
     "controls": [("DB activity monitoring", 25_000, {"aro": 0.05}), ("tokenise card data", 60_000, {"ef": 0.05})]},
    {"id": "R2", "asset": "file servers", "av": 500_000, "ef": 0.8, "aro": 0.3,
     "controls": [("immutable backups", 15_000, {"ef": 0.1}), ("EDR", 40_000, {"aro": 0.15})]},
    {"id": "R3", "asset": "HQ building", "av": 10_000_000, "ef": 0.5, "aro": 0.01,
     "controls": [("seismic retrofit", 120_000, {"ef": 0.1})]},
    {"id": "R4", "asset": "laptops", "av": 2_000, "ef": 1.0, "aro": 6,
     "controls": [("cable locks", 3_000, {"aro": 4})]},
]

def ale(av, ef, aro):
    return av * ef * aro

def safeguard_value(risk, cost, change):
    params = {**{"ef": risk["ef"], "aro": risk["aro"]}, **change}
    after = ale(risk["av"], params["ef"], params["aro"])
    return ale(risk["av"], risk["ef"], risk["aro"]) - after - cost, after

total_before = total_after = 0
for r in risks:
    before = ale(r["av"], r["ef"], r["aro"])
    sle = r["av"] * r["ef"]
    options = []
    for name, cost, change in r["controls"]:
        value, after = safeguard_value(r, cost, change)
        options.append((value, name, cost, after))
    good = [o for o in options if o[0] > 0]
    total_before += before
    if good:
        value, name, cost, after = max(good)
        decision = f"mitigate with {name} (value ${value:,.0f}/yr)"
        total_after += after + cost
    else:
        decision = "accept" if before <= APPETITE else "transfer (insurance) or avoid"
        total_after += before
    print(f"{r['id']} {r['asset']:<13} SLE ${sle:>12,.0f} ARO {r['aro']:<5} ALE ${before:>9,.0f} -> {decision}")
print(f"Annual cost of risk: ${total_before:,.0f} before, ${total_after:,.0f} after (including control costs)")

@@exercise Check continuity metrics from the BIA
For each business function, check: RTO + WRT ≤ MTD, backup interval ≤ RPO, and that the chosen recovery site can be ready within the RTO (hot 4 h, warm 24 h, cold 168 h). Suggest the cheapest site type that meets the RTO when the current one doesn't. Print each function's problems or "OK".
@@lang python
@@starter
SITE_READY = {"hot": 4, "warm": 24, "cold": 168}
functions = [
    {"name": "online orders", "mtd": 8, "rto": 6, "wrt": 3, "rpo": 0.25, "backup_every": 1, "site": "warm"},
    {"name": "payroll", "mtd": 72, "rto": 48, "wrt": 8, "rpo": 24, "backup_every": 24, "site": "warm"},
    {"name": "email", "mtd": 24, "rto": 8, "wrt": 2, "rpo": 4, "backup_every": 4, "site": "hot"},
    {"name": "data warehouse", "mtd": 336, "rto": 120, "wrt": 24, "rpo": 24, "backup_every": 24, "site": "cold"},
]

def problems(f):
    raise NotImplementedError
@@expected
online orders:
  - RTO 6h + WRT 3h exceeds MTD 8h
  - backups every 1h can't meet RPO 0.25h
  - warm site needs 24h > RTO 6h: use a hot site
payroll: OK
email: OK
data warehouse:
  - cold site needs 168h > RTO 120h: use a warm site
@@requires
def problems
SITE_READY
"wrt"
sorted(
@@hint
Sort `SITE_READY` items by readiness time descending (cheapest first: cold, warm, hot) and pick the first whose time ≤ RTO.
@@solution
SITE_READY = {"hot": 4, "warm": 24, "cold": 168}
functions = [
    {"name": "online orders", "mtd": 8, "rto": 6, "wrt": 3, "rpo": 0.25, "backup_every": 1, "site": "warm"},
    {"name": "payroll", "mtd": 72, "rto": 48, "wrt": 8, "rpo": 24, "backup_every": 24, "site": "warm"},
    {"name": "email", "mtd": 24, "rto": 8, "wrt": 2, "rpo": 4, "backup_every": 4, "site": "hot"},
    {"name": "data warehouse", "mtd": 336, "rto": 120, "wrt": 24, "rpo": 24, "backup_every": 24, "site": "cold"},
]

def problems(f):
    found = []
    if f["rto"] + f["wrt"] > f["mtd"]:
        found.append(f"RTO {f['rto']}h + WRT {f['wrt']}h exceeds MTD {f['mtd']}h")
    if f["backup_every"] > f["rpo"]:
        found.append(f"backups every {f['backup_every']}h can't meet RPO {f['rpo']}h")
    if SITE_READY[f["site"]] > f["rto"]:
        cheapest = next((s for s, t in sorted(SITE_READY.items(), key=lambda kv: -kv[1]) if t <= f["rto"]), None)
        fix = f"use a {cheapest} site" if cheapest else "no site type is fast enough; redesign (active-active)"
        found.append(f"{f['site']} site needs {SITE_READY[f['site']]}h > RTO {f['rto']}h: {fix}")
    return found

for f in functions:
    found = problems(f)
    print(f"{f['name']}: OK" if not found else f"{f['name']}:")
    for p in found:
        print("  - " + p)

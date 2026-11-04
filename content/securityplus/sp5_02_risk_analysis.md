@@lesson sp5-risk Risk Analysis: SLE, ALE, Risk Registers and Business Impact
@@text
Domain 5 (security programme management, 20%) includes quantitative risk calculations that appear on almost every exam: SLE, ARO and ALE. It also covers how organisations record and treat risk (the risk register, appetite and tolerance, the four strategies) and the business impact analysis that sets recovery targets.

## Risk identification and assessment

- **Risk** = likelihood × impact. A **threat** exploits a **vulnerability** to harm an **asset**.
- **Assessment types**: **ad hoc**, **recurring**, **one-time**, **continuous**.
- **Qualitative** analysis uses ratings (low/medium/high, a heat map). Fast, subjective.
- **Quantitative** analysis uses money and numbers. Slower, needs data, easy to compare with the cost of controls.

## Quantitative formulas

| Term | Meaning | Formula |
|---|---|---|
| **AV** | asset value | |
| **EF** | exposure factor: % of the asset lost in one incident | |
| **SLE** | single loss expectancy: cost of one incident | **AV × EF** |
| **ARO** | annualised rate of occurrence: incidents per year | (once every 4 years = 0.25) |
| **ALE** | annualised loss expectancy | **SLE × ARO** |

Example: a ¥40,000,000 warehouse, a flood destroys 25% (EF 0.25) → SLE ¥10,000,000. Floods happen once every 20 years (ARO 0.05) → ALE ¥500,000 per year. A flood barrier costing ¥300,000/year that removes most of the risk is worth it; one costing ¥2,000,000/year isn't (unless there are other reasons, like safety).

**Control value** = ALE before − ALE after − annual cost of the control. Positive means the control pays for itself.

## Risk register

A table of risks with: **description**, **owner** (**risk owner**, accountable), **likelihood**, **impact**, **risk score**, **key risk indicators (KRIs)** (metrics that warn the risk is growing), **treatment**, **status**, **residual risk**. Reviewed regularly.

- **Inherent risk**: before controls.
- **Residual risk**: what remains after controls.
- **Risk appetite**: how much risk the organisation is willing to pursue for its goals (**expansionary**, **conservative**, **neutral**).
- **Risk tolerance**: acceptable variation around the appetite for a specific risk.
- **Risk threshold**: the level that triggers action.

## Risk management strategies

| Strategy | What it means | Example |
|---|---|---|
| **Transfer** | someone else carries the financial impact | cyber insurance, outsourcing |
| **Accept** | live with it (documented; **exemption** or **exception**) | low risk, control costs more than the loss |
| **Avoid** | stop the risky activity | don't store card numbers at all |
| **Mitigate** | reduce likelihood or impact with controls | MFA, backups, patching |

Risk reporting goes to management and the board in business terms.

## Business impact analysis (BIA)

The BIA identifies critical business functions and what happens when they stop:

- **RTO** (recovery time objective): how quickly the function must be restored.
- **RPO** (recovery point objective): how much data loss (in time) is acceptable.
- **MTTR** (mean time to repair) and **MTBF** (mean time between failures).
- **Maximum tolerable downtime (MTD)**: beyond this, the business suffers severe harm. RTO must be shorter than MTD.
- Identify **mission-essential functions**, **critical systems** and their **dependencies** (a payroll app depends on the database, AD, the network, a SaaS tax service).

The BIA feeds the **business continuity plan** and **disaster recovery plan**, and the priorities for backups and redundancy.

@@quiz
A server worth ¥2,000,000 would lose 40% of its value in a fire. What is the SLE?
- ¥2,000,000
* ¥800,000
- ¥400,000
- ¥80,000
? SLE = AV × EF.

@@quiz
The SLE is ¥800,000 and the fire is expected once every 10 years. What is the ALE?
- ¥8,000,000
- ¥800,000
* ¥80,000
- ¥8,000
? ALE = SLE × ARO (0.1).

@@quiz
A company buys cyber insurance for ransomware losses. Which strategy is this?
- Avoid
- Accept
* Transfer
- Mitigate
? The financial impact moves to the insurer.

@@quiz
What is the risk that remains after controls are applied?
- Inherent risk
* Residual risk
- Risk appetite
- Control risk
? Inherent risk is before controls.

@@quiz
Which BIA value is the longest a function can be down before the business suffers severe harm?
- RPO
- RTO
* MTD
- MTBF
? RTO must be shorter than MTD.

@@flashcards
SLE :: AV × EF
ALE :: SLE × ARO
ARO once every 5 years :: 0.2
EF :: % of asset lost per incident
qualitative :: ratings, heat maps
quantitative :: money and numbers
inherent risk :: before controls
residual risk :: after controls
risk appetite :: risk willing to pursue
risk tolerance :: acceptable variation
KRI :: key risk indicator
risk owner :: accountable for a risk
transfer :: insurance, outsourcing
avoid :: stop the activity
mitigate :: apply controls
accept :: document and live with it
MTD :: maximum tolerable downtime
BIA :: business impact analysis

@@drill
What is the ARO for an event expected once every 4 years?
= 0.25
= .25
? 1 ÷ 4.

@@drill
An SLE is 500,000 and the ARO is 0.2. What is the ALE?
= 100000
= 100,000
? SLE × ARO.

@@exercise Calculate ALE and control value
For each risk, compute SLE, ALE before, ALE after the proposed control (the control reduces ARO by `reduction`, a fraction), and the control's net annual value (ALE before − ALE after − control cost). Recommend `implement` if the value is positive, else `accept or find cheaper control`. Use yen amounts with thousands separators.
@@lang python
@@starter
risks = [
    {"name": "Laptop theft (data breach)", "av": 30_000_000, "ef": 0.5, "aro": 0.5, "control": "Full-disk encryption", "cost": 1_200_000, "reduction": 0.9},
    {"name": "Ransomware on file server", "av": 80_000_000, "ef": 0.6, "aro": 0.1, "control": "Immutable backups + EDR", "cost": 2_500_000, "reduction": 0.8},
    {"name": "Office flood", "av": 40_000_000, "ef": 0.25, "aro": 0.05, "control": "Flood barrier", "cost": 2_000_000, "reduction": 0.9},
]
@@expected
Laptop theft (data breach)
  SLE ¥15,000,000, ALE before ¥7,500,000, after ¥750,000
  Full-disk encryption costs ¥1,200,000/yr, net value ¥5,550,000 -> implement
Ransomware on file server
  SLE ¥48,000,000, ALE before ¥4,800,000, after ¥960,000
  Immutable backups + EDR costs ¥2,500,000/yr, net value ¥1,340,000 -> implement
Office flood
  SLE ¥10,000,000, ALE before ¥500,000, after ¥50,000
  Flood barrier costs ¥2,000,000/yr, net value -¥1,550,000 -> accept or find cheaper control
@@requires
:,.0f
["reduction"]
@@hint
`sle = av * ef`, `ale_before = sle * aro`, `ale_after = sle * aro * (1 - reduction)`.
@@solution
risks = [
    {"name": "Laptop theft (data breach)", "av": 30_000_000, "ef": 0.5, "aro": 0.5, "control": "Full-disk encryption", "cost": 1_200_000, "reduction": 0.9},
    {"name": "Ransomware on file server", "av": 80_000_000, "ef": 0.6, "aro": 0.1, "control": "Immutable backups + EDR", "cost": 2_500_000, "reduction": 0.8},
    {"name": "Office flood", "av": 40_000_000, "ef": 0.25, "aro": 0.05, "control": "Flood barrier", "cost": 2_000_000, "reduction": 0.9},
]

for r in risks:
    sle = r["av"] * r["ef"]
    ale_before = sle * r["aro"]
    ale_after = sle * r["aro"] * (1 - r["reduction"])
    value = ale_before - ale_after - r["cost"]
    advice = "implement" if value > 0 else "accept or find cheaper control"
    sign = "-" if value < 0 else ""
    print(r["name"])
    print(f"  SLE ¥{sle:,.0f}, ALE before ¥{ale_before:,.0f}, after ¥{ale_after:,.0f}")
    print(f"  {r['control']} costs ¥{r['cost']:,.0f}/yr, net value {sign}¥{abs(value):,.0f} -> {advice}")

@@exercise Maintain a risk register
Score each risk as likelihood × impact (both 1–5). Rate the score: 15+ `high`, 8–14 `medium`, else `low`. Flag risks whose score exceeds the organisation's tolerance for their category, and risks with no owner. Print the register sorted by score (highest first), then the treatment each high risk needs per `STRATEGY`.
@@lang python
@@starter
TOLERANCE = {"security": 8, "operations": 12, "compliance": 6}
STRATEGY = {"security": "mitigate", "operations": "mitigate", "compliance": "avoid or mitigate"}
register = [
    {"id": "R1", "risk": "Phishing leads to account takeover", "cat": "security", "likelihood": 4, "impact": 4, "owner": "CISO"},
    {"id": "R2", "risk": "Single ISP link fails", "cat": "operations", "likelihood": 2, "impact": 4, "owner": "Network lead"},
    {"id": "R3", "risk": "Personal data kept beyond retention period", "cat": "compliance", "likelihood": 3, "impact": 3, "owner": ""},
    {"id": "R4", "risk": "Unpatched VPN appliance", "cat": "security", "likelihood": 3, "impact": 5, "owner": "Infra manager"},
    {"id": "R5", "risk": "Key staff member leaves", "cat": "operations", "likelihood": 2, "impact": 2, "owner": "HR"},
]
@@expected
R1 16 high   Phishing leads to account takeover [over tolerance]
R4 15 high   Unpatched VPN appliance [over tolerance]
R3  9 medium Personal data kept beyond retention period [over tolerance, NO OWNER]
R2  8 medium Single ISP link fails
R5  4 low    Key staff member leaves
R1: mitigate
R4: mitigate
@@requires
TOLERANCE[
sorted(
@@hint
Compute the score first and store it on each entry, then sort with `key=lambda r: r["score"], reverse=True`.
@@solution
TOLERANCE = {"security": 8, "operations": 12, "compliance": 6}
STRATEGY = {"security": "mitigate", "operations": "mitigate", "compliance": "avoid or mitigate"}
register = [
    {"id": "R1", "risk": "Phishing leads to account takeover", "cat": "security", "likelihood": 4, "impact": 4, "owner": "CISO"},
    {"id": "R2", "risk": "Single ISP link fails", "cat": "operations", "likelihood": 2, "impact": 4, "owner": "Network lead"},
    {"id": "R3", "risk": "Personal data kept beyond retention period", "cat": "compliance", "likelihood": 3, "impact": 3, "owner": ""},
    {"id": "R4", "risk": "Unpatched VPN appliance", "cat": "security", "likelihood": 3, "impact": 5, "owner": "Infra manager"},
    {"id": "R5", "risk": "Key staff member leaves", "cat": "operations", "likelihood": 2, "impact": 2, "owner": "HR"},
]

for r in register:
    r["score"] = r["likelihood"] * r["impact"]
    r["rating"] = "high" if r["score"] >= 15 else "medium" if r["score"] >= 8 else "low"

for r in sorted(register, key=lambda r: r["score"], reverse=True):
    flags = []
    if r["score"] > TOLERANCE[r["cat"]]:
        flags.append("over tolerance")
    if not r["owner"]:
        flags.append("NO OWNER")
    extra = f" [{', '.join(flags)}]" if flags else ""
    print(f"{r['id']} {r['score']:>2} {r['rating']:<6} {r['risk']}{extra}")

for r in register:
    if r["rating"] == "high":
        print(f"{r['id']}: {STRATEGY[r['cat']]}")

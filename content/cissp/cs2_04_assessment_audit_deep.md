@@lesson cs2-assessment-audit-deep Security Assessment and Audit in Depth: Strategies, SOC Reports, Sampling and Metrics (Domain 6)
@@concepts numbers, lists, functions
@@lang python
@@text
Domain 6 is about proving that controls work, to management, auditors, customers and regulators. You need to know the kinds of assessment and who performs them, how audit reports work (especially SOC reports, which every cloud customer asks for), how auditors test with samples, and which metrics tell leadership the truth.

## Assessment strategies

- **Internal** assessments are done by your own staff: cheaper and more frequent, but less independent.
- **External** assessments are done by a firm you hire: independent, and their report is credible to others.
- **Third-party** assessments are done for someone else's benefit, such as a customer auditing a supplier or a regulator auditing you.
- Kinds of test:
  - **vulnerability assessment**: find and rate weaknesses, often automated.
  - **penetration test**: exploit them to show the real impact, always with written authorisation, an agreed scope and rules of engagement.
  - **red team**: an objective-based adversary simulation that also tests detection and response.
  - **tabletop exercise**: walk through a scenario on paper.
  - **code review** and **log reviews**.
- Pen test knowledge levels: **black box** (no knowledge), **grey box** (some, e.g. a user account) and **white box** (full documentation and code).
- **Synthetic transactions** (scripted user actions run constantly) and **real user monitoring** measure availability and performance.
- **Breach and attack simulation** tools continuously test whether controls detect known techniques (mapped to MITRE ATT&CK).

## Audit reports

- **SOC 1**: controls relevant to the customer's **financial reporting** (for their auditors, e.g. under SOX).
- **SOC 2**: the **Trust Services Criteria**: security (always included), availability, processing integrity, confidentiality and privacy. Restricted distribution, under NDA.
- **SOC 3**: a public summary of a SOC 2, without the detail.
- **Type I**: controls are suitably **designed** at a point in time. **Type II**: controls **operated effectively over a period** (usually 6–12 months). Type II is what customers should ask for.
- **Bridge letter**: covers the gap between the end of the last report period and today.
- Read the **exceptions** and the **complementary user entity controls (CUECs)**: things the provider assumes *you* do, such as reviewing your own users' access.
- Other standards: ISO/IEC 27001 certification (an ISMS audit), ISAE 3402 (the international SOC 1 equivalent), PCI DSS ROC (Report on Compliance) by a QSA.

## How auditors test: sampling

- Auditors rarely test every item. They pick a **sample**, test it, and draw a conclusion about the population.
- **Statistical sampling** (random selection, with a defined confidence) supports conclusions about the whole population. **Judgmental** sampling relies on the auditor's experience.
- Typical approach: for a control that runs many times a day, test 25–60 instances. For a quarterly control, test 2. For an annual control, test 1.
- An **exception rate** above the **tolerable deviation rate** means the control is not operating effectively.

## Metrics: KPIs and KRIs

- **KPIs** measure how well the security programme performs: patch latency, mean time to detect (**MTTD**), mean time to respond or remediate (**MTTR**), the percentage of systems with EDR, phishing report rate, training completion.
- **KRIs** (key risk indicators) are leading signals that risk is rising: the number of critical vulnerabilities older than SLA, privileged accounts without MFA, unsupported OS count, third parties without assessments.
- Good metrics have a target, a trend and an owner. Report risk in business terms (what it means, money, probability) to executives, and technical detail to the teams fixing things.
- **Coverage matters**: "98% of scanned hosts are patched" hides hosts that were never scanned. Always report coverage alongside the result.

@@quiz
A customer wants evidence that a cloud provider's controls worked over the past year. Which report should they ask for?
* SOC 2 Type II
- SOC 2 Type I
- SOC 3
- A vendor questionnaire only
? Type II covers operating effectiveness over a period.

@@quiz
What are complementary user entity controls in a SOC report?
* Controls the provider assumes the customer performs
- Controls the auditor added
- Extra controls the provider sells
- Controls that failed testing
? You must implement them yourself.

@@quiz
Which test gives the tester full documentation and source code?
- Black box
- Grey box
* White box
- Red team
? Most thorough per hour spent.

@@quiz
What must exist before any penetration test begins?
* Written authorisation with an agreed scope and rules of engagement
- A SOC 2 report
- A bridge letter
- A red team
? Without it, testing may be illegal.

@@quiz
Which metric is a key risk indicator rather than a performance indicator?
* Number of critical vulnerabilities past their remediation SLA
- Training completion rate
- Number of tickets closed
- Average call duration
? It signals rising risk.

@@quiz
The sample exception rate is 6% and the tolerable deviation rate is 5%. What does the auditor conclude?
* The control is not operating effectively
- The control is effective
- The sample is too large
- Nothing can be concluded
? Above tolerable means ineffective.

@@quiz
What does a bridge letter cover?
* The period between the end of the last SOC report and now
- The connection between two data centres
- The CUECs
- A pen test's scope
? Issued by the provider's management.

@@flashcards
internal / external / third-party :: own staff / hired firm / for someone else
vulnerability assessment :: find and rate weaknesses
penetration test :: exploit to show impact, with authorisation
red team :: objective-based adversary simulation
black / grey / white box :: none / some / full knowledge
synthetic transactions :: scripted checks of a service
SOC 1 :: financial reporting controls
SOC 2 :: Trust Services Criteria, restricted
SOC 3 :: public summary
Type I / Type II :: design at a point / operation over a period
bridge letter :: covers the gap since the last report
CUEC :: controls the customer must perform
statistical sampling :: random, supports population conclusions
tolerable deviation rate :: exception rate the auditor accepts
KPI / KRI :: performance / rising risk
MTTD / MTTR :: mean time to detect / to respond or remediate
coverage :: how much of the population was measured

@@drill
Which SOC report type tests operating effectiveness over a period: I or II?
= II
= Type II
= 2
? Usually 6–12 months.

@@drill
A daily control has 40 sampled instances with 3 exceptions. What is the exception rate in percent? (one decimal)
= 7.5
? 3 ÷ 40.

@@exercise Evaluate control testing samples
For each control, choose the sample size from its frequency (annual 1, quarterly 2, monthly 2, weekly 5, daily 25, many times a day 40), test the given results (only the first `sample size` items count), compute the exception rate, and conclude "effective" if it is at most the tolerable rate (5%; for annual and quarterly controls a single exception fails).
@@starter
SAMPLE = {"annual": 1, "quarterly": 2, "monthly": 2, "weekly": 5, "daily": 25, "multiple": 40}

controls = [
    {"id": "AC-02 access review", "freq": "quarterly", "results": [True, True, True, True]},
    {"id": "CM-03 change approval", "freq": "multiple", "results": [True] * 37 + [False, True, False] + [True] * 20},
    {"id": "BK-01 backup check", "freq": "daily", "results": [True] * 24 + [False] + [True] * 5},
    {"id": "IR-04 DR test", "freq": "annual", "results": [False]},
    {"id": "LG-07 log review", "freq": "weekly", "results": [True, True, False, True, True, True]},
]

def evaluate(control, tolerable=0.05):
    raise NotImplementedError

for c in controls:
    size, exceptions, rate, verdict = evaluate(c)
    print(f"{c['id']:<22} sample {size:>2}  exceptions {exceptions}  rate {rate:5.1%}  {verdict}")
@@expected
AC-02 access review    sample  2  exceptions 0  rate  0.0%  effective
CM-03 change approval  sample 40  exceptions 2  rate  5.0%  effective
BK-01 backup check     sample 25  exceptions 1  rate  4.0%  effective
IR-04 DR test          sample  1  exceptions 1  rate 100.0%  NOT effective
LG-07 log review       sample  5  exceptions 1  rate 20.0%  NOT effective
@@requires
def evaluate
SAMPLE[
@@hint
`size = SAMPLE[control["freq"]]`, take `control["results"][:size]`, count the `False` values. For annual and quarterly controls any exception means "not effective"; otherwise compare `rate = exceptions / size` with the tolerable rate.
@@solution
SAMPLE = {"annual": 1, "quarterly": 2, "monthly": 2, "weekly": 5, "daily": 25, "multiple": 40}

controls = [
    {"id": "AC-02 access review", "freq": "quarterly", "results": [True, True, True, True]},
    {"id": "CM-03 change approval", "freq": "multiple", "results": [True] * 37 + [False, True, False] + [True] * 20},
    {"id": "BK-01 backup check", "freq": "daily", "results": [True] * 24 + [False] + [True] * 5},
    {"id": "IR-04 DR test", "freq": "annual", "results": [False]},
    {"id": "LG-07 log review", "freq": "weekly", "results": [True, True, False, True, True, True]},
]

def evaluate(control, tolerable=0.05):
    size = SAMPLE[control["freq"]]
    tested = control["results"][:size]
    exceptions = tested.count(False)
    rate = exceptions / size
    if control["freq"] in ("annual", "quarterly"):
        ok = exceptions == 0
    else:
        ok = rate <= tolerable
    return size, exceptions, rate, "effective" if ok else "NOT effective"

for c in controls:
    size, exceptions, rate, verdict = evaluate(c)
    print(f"{c['id']:<22} sample {size:>2}  exceptions {exceptions}  rate {rate:5.1%}  {verdict}")

@@exercise Vulnerability metrics with coverage
From a list of hosts (some never scanned) and the vulnerabilities found, compute the KPIs and KRIs for a monthly report: scan coverage, MTTR for closed findings in days, the percentage of closed findings fixed within SLA (critical 15 days, high 30), open findings past SLA (a KRI) as of the report date, and patched percentage both of scanned hosts and of all hosts.
@@starter
from datetime import date

report_day = date(2026, 6, 30)
SLA = {"critical": 15, "high": 30}
hosts = {"web1": True, "web2": True, "db1": True, "app1": True, "legacy1": False, "ot-gw": False}
vulns = [
    {"host": "web1", "sev": "critical", "found": date(2026, 5, 2), "fixed": date(2026, 5, 10)},
    {"host": "web2", "sev": "high", "found": date(2026, 5, 5), "fixed": date(2026, 6, 20)},
    {"host": "db1", "sev": "critical", "found": date(2026, 6, 1), "fixed": None},
    {"host": "app1", "sev": "high", "found": date(2026, 6, 10), "fixed": None},
    {"host": "web1", "sev": "high", "found": date(2026, 6, 3), "fixed": date(2026, 6, 18)},
]

def metrics(hosts, vulns, today):
    raise NotImplementedError

for name, value in metrics(hosts, vulns, report_day).items():
    print(f"{name:<34} {value}")
@@expected
scan coverage                      67% (4 of 6 hosts)
MTTR (closed findings)             23.0 days
closed within SLA                  67%
KRI: open findings past SLA        1 (db1)
patched, of scanned hosts          50%
patched, of all hosts              33% (unscanned hosts count as unknown)
@@requires
def metrics
.days
@@hint
Closed findings have `fixed`; their age is `(fixed - found).days`. Open ones are past SLA if `(today - found).days > SLA[sev]`. A scanned host counts as patched if it has no open findings. Format percentages with `f"{x:.0%}"` and the MTTR with one decimal.
@@solution
from datetime import date

report_day = date(2026, 6, 30)
SLA = {"critical": 15, "high": 30}
hosts = {"web1": True, "web2": True, "db1": True, "app1": True, "legacy1": False, "ot-gw": False}
vulns = [
    {"host": "web1", "sev": "critical", "found": date(2026, 5, 2), "fixed": date(2026, 5, 10)},
    {"host": "web2", "sev": "high", "found": date(2026, 5, 5), "fixed": date(2026, 6, 20)},
    {"host": "db1", "sev": "critical", "found": date(2026, 6, 1), "fixed": None},
    {"host": "app1", "sev": "high", "found": date(2026, 6, 10), "fixed": None},
    {"host": "web1", "sev": "high", "found": date(2026, 6, 3), "fixed": date(2026, 6, 18)},
]

def metrics(hosts, vulns, today):
    scanned = [h for h, s in hosts.items() if s]
    closed = [v for v in vulns if v["fixed"]]
    open_ = [v for v in vulns if not v["fixed"]]
    ages = [(v["fixed"] - v["found"]).days for v in closed]
    within = [v for v, age in zip(closed, ages) if age <= SLA[v["sev"]]]
    overdue = [v for v in open_ if (today - v["found"]).days > SLA[v["sev"]]]
    unpatched = {v["host"] for v in open_}
    patched = [h for h in scanned if h not in unpatched]
    return {
        "scan coverage": f"{len(scanned) / len(hosts):.0%} ({len(scanned)} of {len(hosts)} hosts)",
        "MTTR (closed findings)": f"{sum(ages) / len(ages):.1f} days",
        "closed within SLA": f"{len(within) / len(closed):.0%}",
        "KRI: open findings past SLA": f"{len(overdue)} ({', '.join(v['host'] for v in overdue)})",
        "patched, of scanned hosts": f"{len(patched) / len(scanned):.0%}",
        "patched, of all hosts": f"{len(patched) / len(hosts):.0%} (unscanned hosts count as unknown)",
    }

for name, value in metrics(hosts, vulns, report_day).items():
    print(f"{name:<34} {value}")

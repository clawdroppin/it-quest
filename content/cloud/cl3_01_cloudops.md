@@lesson cl3-cloudops CloudOps: Running Cloud Infrastructure Day to Day
@@concepts maps, numbers, automation
@@lang python
@@text
Building something in the cloud takes an afternoon. **Running** it for years (secure, monitored, backed up, patched, affordable, and with everyone able to find what they need) is the job of **CloudOps** (cloud operations). This lesson is the operating manual, with AWS names first and the Azure and Google Cloud equivalents alongside.

## A landing zone: the foundation

Before the first workload, set up a **landing zone**: the accounts, identity, network and guardrails everything else lands in.

- **Many accounts, not one.** Separate accounts (AWS) / subscriptions (Azure) / projects (GCP) per environment and team: a mistake or breach in *dev* can't touch *prod*, and costs split naturally. Group them with **AWS Organizations** (OUs), **Azure management groups** or the **GCP resource hierarchy** (organization → folders → projects).
- **Guardrails** at the top: **Service Control Policies** (AWS), **Azure Policy**, **GCP Organization Policies**. Examples: only allowed regions, no disabling the audit log, no public storage, encryption required.
- **Central logging and security accounts**: the audit trail (CloudTrail / Activity Log / Cloud Audit Logs) of every account goes to a locked-down log account nobody can edit.
- Tools that build it for you: **AWS Control Tower**, the **Azure Landing Zone** accelerator, Google's **Cloud Foundation Fabric**, all in Terraform.

## Identity and access

- People sign in through **single sign-on** (IAM Identity Center, Entra ID, Google Workspace/Cloud Identity) with **MFA**, and get **roles** with temporary credentials. No IAM users with long-lived keys.
- Machines use **roles** too: EC2 instance profiles, Lambda execution roles, Azure **managed identities**, GCP **service accounts** with workload identity.
- **Least privilege**: start from nothing and add what's needed; review with IAM Access Analyzer / Entra access reviews; use **permission boundaries** for delegated admins.
- Keep a sealed **break-glass** account for emergencies, with alerts whenever it's used.

## Tagging: knowing what everything is

Every resource gets **tags** (labels in GCP): `Owner`, `Environment`, `CostCenter`, `Application`, `DataClassification`. Tags drive cost reports, automation ("stop every `Environment=dev` instance at 20:00"), access control (ABAC) and incident response ("who owns this?"). Enforce them with policy (AWS tag policies + SCPs, Azure Policy "require a tag", Terraform `default_tags`), and report on resources that are missing them; the first exercise builds that report.

## Monitoring, logging and audit

| Need | AWS | Azure | Google Cloud |
|---|---|---|---|
| Metrics, dashboards, alarms | CloudWatch | Azure Monitor | Cloud Monitoring |
| Logs | CloudWatch Logs | Log Analytics (KQL) | Cloud Logging |
| Audit (who did what) | CloudTrail | Activity Log | Cloud Audit Logs |
| Config history and compliance | AWS Config | Azure Policy / Resource Graph | Cloud Asset Inventory |
| Threat detection | GuardDuty, Security Hub | Defender for Cloud | Security Command Center |
| Tracing | X-Ray | Application Insights | Cloud Trace |

Many teams also send everything to one place (Prometheus/Grafana, Datadog, Elastic) to see all clouds and on-premises together. Alarms go to an on-call tool, and every alarm has a runbook.

## Patching and configuration

- **Immutable** infrastructure where possible: don't patch servers, replace them. Build a new image (Packer, EC2 Image Builder), roll it out through the Auto Scaling group, delete the old instances.
- For long-lived servers: **AWS Systems Manager** (Patch Manager, Run Command, Session Manager instead of SSH and bastion hosts), **Azure Update Manager** and **Azure Arc**, **GCP OS Config / VM Manager**.
- Managed services (RDS, Aurora, Azure SQL, Cloud SQL, managed Kubernetes) patch themselves in **maintenance windows** you choose.

## Backup and disaster recovery

Two numbers drive every decision:

- **RPO** (recovery point objective): how much data you can lose, measured in time. RPO 1 hour = backups at least hourly.
- **RTO** (recovery time objective): how long you can be down before it's back.

The four classic DR strategies, from cheap and slow to expensive and fast:

| Strategy | How | Typical RPO / RTO |
|---|---|---|
| **Backup and restore** | backups copied to another region; rebuild everything with IaC when needed | hours / hours to a day |
| **Pilot light** | data replicated continuously; core servers off or tiny in the second region | minutes / tens of minutes |
| **Warm standby** | a scaled-down but running copy in the second region; scale up and switch | seconds to minutes / minutes |
| **Multi-site active/active** | full capacity in two or more regions, traffic in all of them | near zero / near zero |

Rules that matter more than the strategy: backups are **automated** (AWS Backup, Azure Backup, Backup and DR Service), **copied to another account and region**, **immutable** (vault lock / soft delete, against ransomware), **encrypted**, and **restores are tested** regularly. An untested backup is a hope, not a backup. The second exercise picks a strategy from RPO and RTO.

## Cost management (FinOps)

- **Visibility**: Cost Explorer / Cost Management / Billing reports, split by tags and accounts; **budgets** with alerts at 50%, 80% and 100%; anomaly detection.
- **Rightsizing**: most servers are oversized. If an instance's CPU stays under ~20% (look at p95, not the average, and memory too), move it down a size; the third exercise does this.
- **Pricing models**: Savings Plans / Reserved Instances (1 or 3 years, up to ~70% off) for the steady base; **Spot** / preemptible (up to ~90% off, can be taken away) for batch jobs, CI runners and stateless workers; on-demand for the rest.
- **Waste**: unattached disks, old snapshots, idle load balancers, unused Elastic IPs, dev environments running all weekend, data transfer between regions and through NAT gateways.
- **Storage tiers and lifecycle rules**: S3 Standard → Infrequent Access → Glacier after 30/90 days; the same in Azure Blob (hot/cool/cold/archive) and GCS (standard/nearline/coldline/archive).

## Change, incidents and documentation

- Every change goes through **IaC and a pull request**; console clicks in production are for emergencies only, and drift is detected and fixed.
- **Runbooks** for common tasks and alarms, kept next to the code; automate the most frequent ones (Systems Manager Automation, Azure Automation runbooks).
- Incidents follow the process from the DevOps track: declare, mitigate, communicate, blameless postmortem.
- Read your providers' **status pages** and **health dashboards** (AWS Health, Azure Service Health) and subscribe to their events.
- The **Well-Architected Framework** (AWS, Azure and Google each have one) is the checklist for reviews: operational excellence, security, reliability, performance, cost, sustainability.

@@quiz
Why use separate accounts or subscriptions per environment?
- Each account gets free credits
* Isolation: a mistake or breach in dev can't reach prod, permissions are simpler and costs split naturally
- The cloud allows only one VPC per account
- It's required for tagging
? Organizations / management groups plus guardrail policies manage many accounts centrally.

@@quiz
A system can lose at most 15 minutes of data and must be back within 1 hour. Which values are these?
* RPO 15 minutes, RTO 1 hour
- RTO 15 minutes, RPO 1 hour
- SLA 15 minutes, SLO 1 hour
- MTTR 15 minutes, MTBF 1 hour
? RPO is about data (the recovery point), RTO about time (to recover).

@@quiz
Which workload fits Spot or preemptible instances best?
- The production database
* CI build runners and batch jobs that can be retried when an instance is taken away
- The domain controller
- A single-instance web server with no load balancer
? Spot is cheap because it can be reclaimed at short notice.

@@quiz
What makes a backup strategy trustworthy?
- Taking backups every minute
* Automated, copied to another account and region, immutable, and regularly tested by actually restoring
- Storing backups on the same disk for speed
- Using the most expensive storage class
? Ransomware targets backups too; immutability and separation protect them.

@@quiz
What does AWS Systems Manager Session Manager replace?
- CloudWatch
* SSH keys, open port 22 and bastion hosts: shell access through IAM, with every session logged
- The AWS console
- IAM roles
? Azure Bastion and GCP IAP TCP forwarding play the same role.

@@flashcards
landing zone :: accounts, identity, network and guardrails set up before workloads
AWS Organizations / Azure management groups / GCP folders :: grouping many accounts or subscriptions
SCP / Azure Policy / Organization Policy :: guardrails for whole groups of accounts
break-glass account :: sealed emergency access, alarmed when used
tagging strategy :: Owner, Environment, CostCenter, Application on every resource
CloudTrail / Activity Log / Cloud Audit Logs :: who did what, when
AWS Config :: configuration history and compliance rules
GuardDuty / Defender for Cloud / SCC :: managed threat detection
immutable infrastructure :: replace servers with new images instead of patching them
Session Manager :: shell access without SSH keys or bastions
RPO / RTO :: acceptable data loss / acceptable downtime
backup and restore → pilot light → warm standby → active/active :: DR strategies, cheap to fast
immutable backups :: vault lock / soft delete against ransomware
rightsizing :: move underused instances to a smaller size
Savings Plans / Reserved Instances :: commit 1 or 3 years for big discounts
Spot / preemptible :: spare capacity, very cheap, can be reclaimed
lifecycle rules :: move or delete objects by age automatically
Well-Architected Framework :: the providers' checklists for good designs

@@exercise A tag compliance report
Every resource must have the tags `Owner`, `Environment` and `CostCenter`, and `Environment` must be one of `dev`, `staging` or `prod`. Write `report(resources)` that returns a list of strings, one per non-compliant resource in input order: `"<id>: missing Owner, CostCenter"` (missing tags in the order of the required list) and/or `"<id>: bad Environment 'qa'"`. If a resource has both problems, join them with `"; "` after the id: `"<id>: missing Owner; bad Environment 'qa'"`. Tag **names** are case-sensitive, values are not (`Prod` is fine).
@@starter
REQUIRED = ["Owner", "Environment", "CostCenter"]
ENVIRONMENTS = {"dev", "staging", "prod"}


def report(resources):
    pass


resources = [
    {"id": "i-0a1", "tags": {"Owner": "aiko", "Environment": "prod", "CostCenter": "IT-100"}},
    {"id": "i-0b2", "tags": {"Environment": "dev"}},
    {"id": "vol-9c3", "tags": {"Owner": "ken", "Environment": "qa", "CostCenter": "IT-200"}},
    {"id": "bucket-logs", "tags": {}},
]
for line in report(resources):
    print(line)
@@tests
resources = [
    {"id": "i-0a1", "tags": {"Owner": "aiko", "Environment": "prod", "CostCenter": "IT-100"}},
    {"id": "i-0b2", "tags": {"Environment": "dev"}},
    {"id": "vol-9c3", "tags": {"Owner": "ken", "Environment": "qa", "CostCenter": "IT-200"}},
    {"id": "bucket-logs", "tags": {}},
]
check(report(resources), ["i-0b2: missing Owner, CostCenter", "vol-9c3: bad Environment 'qa'", "bucket-logs: missing Owner, Environment, CostCenter"], "the sample")
check(report([{"id": "x", "tags": {"Owner": "a", "Environment": "Prod", "CostCenter": "c"}}]), [], "values are case-insensitive")
check(report([{"id": "y", "tags": {"owner": "a", "Environment": "test", "CostCenter": "c"}}]), ["y: missing Owner; bad Environment 'test'"], "both problems; names are case-sensitive")
@@hint
For each resource: `missing = [t for t in REQUIRED if t not in tags]`. The environment is bad if it's present and `env.lower() not in ENVIRONMENTS`. Build a list of problem strings and join them with `"; "`.
@@solution
REQUIRED = ["Owner", "Environment", "CostCenter"]
ENVIRONMENTS = {"dev", "staging", "prod"}


def report(resources):
    lines = []
    for r in resources:
        tags = r["tags"]
        problems = []
        missing = [t for t in REQUIRED if t not in tags]
        if missing:
            problems.append("missing " + ", ".join(missing))
        env = tags.get("Environment")
        if env is not None and env.lower() not in ENVIRONMENTS:
            problems.append(f"bad Environment '{env}'")
        if problems:
            lines.append(f"{r['id']}: " + "; ".join(problems))
    return lines


resources = [
    {"id": "i-0a1", "tags": {"Owner": "aiko", "Environment": "prod", "CostCenter": "IT-100"}},
    {"id": "i-0b2", "tags": {"Environment": "dev"}},
    {"id": "vol-9c3", "tags": {"Owner": "ken", "Environment": "qa", "CostCenter": "IT-200"}},
    {"id": "bucket-logs", "tags": {}},
]
for line in report(resources):
    print(line)

@@exercise Pick a disaster recovery strategy
Write `strategy(rpo_minutes, rto_minutes)` that returns the **cheapest** strategy that meets both targets, using these capabilities:

| Strategy | Best RPO it can reach | Best RTO it can reach |
|---|---|---|
| `"backup and restore"` | 1440 (24 h) | 480 (8 h) |
| `"pilot light"` | 15 | 60 |
| `"warm standby"` | 1 | 15 |
| `"active/active"` | 0 | 0 |

A strategy meets a target when its value is **less than or equal to** the target. Also write `monthly_cost(strategy, prod_cost)`: the extra monthly cost of the DR copy as a share of production's cost: 5%, 20%, 50% and 100% respectively, rounded to whole currency units.
@@starter
STRATEGIES = [
    ("backup and restore", 1440, 480, 0.05),
    ("pilot light", 15, 60, 0.20),
    ("warm standby", 1, 15, 0.50),
    ("active/active", 0, 0, 1.00),
]


def strategy(rpo_minutes, rto_minutes):
    pass


def monthly_cost(strategy_name, prod_cost):
    pass


s = strategy(30, 120)
print(s, monthly_cost(s, 12000))
@@tests
check(strategy(2880, 1440), "backup and restore", "relaxed targets: the cheapest")
check(strategy(30, 120), "pilot light", "half an hour of data, two hours down")
check(strategy(5, 30), "warm standby", "minutes")
check(strategy(0, 5), "active/active", "zero data loss")
check(strategy(15, 60), "pilot light", "exactly on the limit")
check(monthly_cost("pilot light", 12000), 2400, "20% of production")
check(monthly_cost("backup and restore", 999), 50, "rounded")
@@hint
Loop over `STRATEGIES` in order (cheapest first) and return the first name whose RPO and RTO are both `<=` the targets. For the cost, find the row by name and `round(prod_cost * share)`.
@@solution
STRATEGIES = [
    ("backup and restore", 1440, 480, 0.05),
    ("pilot light", 15, 60, 0.20),
    ("warm standby", 1, 15, 0.50),
    ("active/active", 0, 0, 1.00),
]


def strategy(rpo_minutes, rto_minutes):
    for name, rpo, rto, _ in STRATEGIES:
        if rpo <= rpo_minutes and rto <= rto_minutes:
            return name
    return "active/active"


def monthly_cost(strategy_name, prod_cost):
    for name, _, _, share in STRATEGIES:
        if name == strategy_name:
            return round(prod_cost * share)
    raise ValueError(strategy_name)


s = strategy(30, 120)
print(s, monthly_cost(s, 12000))

@@exercise Rightsizing recommendations
Instance sizes in one family go `nano, micro, small, medium, large, xlarge, 2xlarge`, and each step down halves the price. Write `p95(values)` (sort the values and take the element at index `ceil(0.95 * n) - 1`) and `recommend(instances)`. Each instance has `id`, `type` like `"m7i.xlarge"`, `price` per month, and lists `cpu` and `mem` of utilization percentages. Recommend **one size down** when both the CPU p95 and the memory p95 are below 40, and the instance isn't already the smallest size. Return a list of `(id, new_type, monthly_saving)`; the saving is half the price, rounded to 2 decimals.
@@starter
import math

SIZES = ["nano", "micro", "small", "medium", "large", "xlarge", "2xlarge"]


def p95(values):
    pass


def recommend(instances):
    pass


fleet = [
    {"id": "i-web1", "type": "m7i.xlarge", "price": 140.16, "cpu": [12, 18, 25, 31, 22, 15, 19, 28, 35, 20], "mem": [30, 32, 35, 33, 31, 30, 29, 36, 38, 34]},
    {"id": "i-db1", "type": "r7i.2xlarge", "price": 385.44, "cpu": [55, 61, 70, 66, 72, 58, 64, 69, 75, 60], "mem": [80, 82, 85, 81, 84, 83, 86, 82, 85, 84]},
    {"id": "i-tiny", "type": "t4g.nano", "price": 3.07, "cpu": [1, 2, 1, 3, 2, 1, 1, 2, 1, 1], "mem": [20, 21, 20, 22, 21, 20, 20, 21, 20, 20]},
]
print(recommend(fleet))
@@tests
check(p95([5, 1, 3, 2, 4]), 5, "five values: the highest")
check(p95(list(range(1, 101))), 95, "1..100")
fleet = [
    {"id": "i-web1", "type": "m7i.xlarge", "price": 140.16, "cpu": [12, 18, 25, 31, 22, 15, 19, 28, 35, 20], "mem": [30, 32, 35, 33, 31, 30, 29, 36, 38, 34]},
    {"id": "i-db1", "type": "r7i.2xlarge", "price": 385.44, "cpu": [55, 61, 70, 66, 72, 58, 64, 69, 75, 60], "mem": [80, 82, 85, 81, 84, 83, 86, 82, 85, 84]},
    {"id": "i-tiny", "type": "t4g.nano", "price": 3.07, "cpu": [1, 2, 1, 3, 2, 1, 1, 2, 1, 1], "mem": [20, 21, 20, 22, 21, 20, 20, 21, 20, 20]},
]
check(recommend(fleet), [("i-web1", "m7i.large", 70.08)], "only the web server")
check(recommend([{"id": "a", "type": "c7g.medium", "price": 25.0, "cpu": [10] * 20 + [90], "mem": [10] * 21}]), [("a", "c7g.small", 12.5)], "one spike doesn't count at p95")
check(recommend([{"id": "b", "type": "m7i.large", "price": 70.0, "cpu": [10] * 10, "mem": [45] * 10}]), [], "memory is too busy")
@@hint
`p95`: `s = sorted(values)` then `s[math.ceil(0.95 * len(s)) - 1]`. Split the type with `family, size = t.split(".")`; the new size is `SIZES[SIZES.index(size) - 1]` when the index is above 0.
@@solution
import math

SIZES = ["nano", "micro", "small", "medium", "large", "xlarge", "2xlarge"]


def p95(values):
    s = sorted(values)
    return s[math.ceil(0.95 * len(s)) - 1]


def recommend(instances):
    out = []
    for inst in instances:
        family, size = inst["type"].split(".")
        i = SIZES.index(size)
        if i > 0 and p95(inst["cpu"]) < 40 and p95(inst["mem"]) < 40:
            out.append((inst["id"], f"{family}.{SIZES[i - 1]}", round(inst["price"] / 2, 2)))
    return out


fleet = [
    {"id": "i-web1", "type": "m7i.xlarge", "price": 140.16, "cpu": [12, 18, 25, 31, 22, 15, 19, 28, 35, 20], "mem": [30, 32, 35, 33, 31, 30, 29, 36, 38, 34]},
    {"id": "i-db1", "type": "r7i.2xlarge", "price": 385.44, "cpu": [55, 61, 70, 66, 72, 58, 64, 69, 75, 60], "mem": [80, 82, 85, 81, 84, 83, 86, 82, 85, 84]},
    {"id": "i-tiny", "type": "t4g.nano", "price": 3.07, "cpu": [1, 2, 1, 3, 2, 1, 1, 2, 1, 1], "mem": [20, 21, 20, 22, 21, 20, 20, 21, 20, 20]},
]
print(recommend(fleet))

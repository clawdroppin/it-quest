@@lesson aw2-cost Pricing, Cost Management and Optimisation
@@text
Billing and pricing make up about 12% of Cloud Practitioner, and cost optimisation is a whole domain of the Solutions Architect exam. In real jobs, "why is the bill so high?" is one of the first questions you'll be asked. This lesson covers how AWS charges, the tools that show and control spend, and the optimisation levers.

## How AWS charges

Three fundamental drivers: **compute** (per second/hour, per request, per GB-second), **storage** (per GB-month, plus requests), and **data transfer**.

Data transfer rules of thumb:
- **Inbound** from the internet: free.
- **Outbound** to the internet: charged per GB (tiered, with a monthly free allowance).
- Between **AZs**: charged in each direction (so chatty multi-AZ designs cost money).
- Between **Regions**: charged.
- Within the same AZ over private IPs: free.
- **CloudFront** egress is usually cheaper than direct S3/EC2 egress; S3 → CloudFront transfer is free.
- **NAT gateway** charges per hour **and per GB processed**: route S3/DynamoDB traffic through **gateway endpoints** instead.

**Free Tier**: always-free offers (e.g. Lambda 1M requests/month), 12-month offers for new accounts (on older accounts), and short trials; newer accounts get credits-based free plans. Set a budget anyway.

## Seeing the money

| Tool | Use |
|---|---|
| **Billing and Cost Management console** / bills | the invoice, by service and Region |
| **Cost Explorer** | visualise and filter costs (by service, account, tag), forecasts, RI/SP recommendations and coverage |
| **AWS Budgets** | alerts on actual or **forecasted** cost/usage/RI/SP thresholds; **budget actions** can apply SCPs or stop instances |
| **Cost Anomaly Detection** | ML alerts on unusual spend |
| **Cost and Usage Report / Data Exports** | most detailed line items to S3, query with Athena/QuickSight |
| **Pricing Calculator** | estimate before you build |
| **Cost allocation tags** | must be **activated** in billing before they appear in reports |
| **Compute Optimizer** | right-sizing recommendations for EC2, ASGs, EBS, Lambda, ECS |
| **Trusted Advisor** | idle resources, underused EBS, unassociated Elastic IPs |

**Consolidated billing** (Organizations): one bill, combined usage for volume tiers, RI/SP discount sharing across accounts.

## Tagging strategy

Tag every resource with owner/team, environment, cost centre and application (`CostCenter=4410`, `Environment=prod`). Enforce with **tag policies**, SCPs (deny create without tags) and Config rules. Untagged resources are the usual source of "nobody knows what this is" spend.

## Optimisation levers

1. **Turn off what isn't used**: idle instances, unattached EBS volumes, old snapshots, unused Elastic IPs (public IPv4 addresses are charged hourly), forgotten load balancers and NAT gateways, dev environments at night (Instance Scheduler).
2. **Right-size**: pick the smaller instance that meets demand (Compute Optimizer); move to **Graviton** for better price/performance; gp2 → **gp3**.
3. **Pricing models**: Savings Plans/RIs for the steady baseline, **Spot** for interruptible work, On-Demand for the rest.
4. **Elasticity**: Auto Scaling, serverless for spiky or low-volume workloads.
5. **Storage tiers**: S3 lifecycle and Intelligent-Tiering, log retention limits, EBS snapshot archive.
6. **Architecture**: CloudFront caching, gateway endpoints, fewer cross-AZ hops for chatty traffic, managed services to cut operational cost.

## FinOps

Cost is a shared responsibility: engineers see the cost of their designs (showback/chargeback by tags), finance gets forecasts, and the organisation reviews unit costs (cost per order, per user). The cycle is **inform → optimise → operate**.

## Support plans (recap)

**Basic** (free: docs, Health Dashboard, core Trusted Advisor checks), **Developer** (business-hours email), **Business** (24/7 phone/chat, full Trusted Advisor, < 1 h for production down), **Enterprise On-Ramp** (pool of TAMs, < 30 min business-critical), **Enterprise** (designated **TAM**, < 15 min business-critical, Concierge).

@@quiz
Which data transfer is normally free?
* Inbound from the internet into AWS
- Outbound from EC2 to the internet
- Between two Regions
- Between two AZs
? Ingress is free; egress is charged.

@@quiz
Which tool alerts you when *forecasted* monthly spend will exceed $500?
- Cost Explorer
* AWS Budgets
- Trusted Advisor
- Pricing Calculator
? Budgets support actual and forecasted alerts.

@@quiz
Tags exist on resources but don't appear in Cost Explorer. What's missing?
- A tag policy
* Activating them as cost allocation tags
- Enabling CloudTrail
- Consolidated billing
? User-defined tags must be activated.

@@quiz
A private subnet's instances download terabytes from S3 through a NAT gateway. How do you cut the cost?
- Use a bigger NAT gateway
* Add an S3 gateway endpoint
- Move the bucket to another Region
- Use Transfer Acceleration
? Gateway endpoints are free and bypass NAT processing charges.

@@quiz
Which service gives right-sizing recommendations for EC2 instances and Lambda memory?
- AWS Budgets
* Compute Optimizer
- Cost and Usage Report
- AWS Config
? Based on CloudWatch utilisation.

@@flashcards
cost drivers :: compute, storage, data transfer
inbound data :: free
cross-AZ traffic :: charged both ways
NAT gateway cost :: per hour + per GB
Cost Explorer :: visualise, forecast, RI/SP recommendations
AWS Budgets :: alerts on actual/forecast
budget actions :: apply SCPs or stop instances
Cost Anomaly Detection :: ML spend alerts
Cost and Usage Report :: detailed line items in S3
cost allocation tags :: must be activated
Pricing Calculator :: estimates before building
Compute Optimizer :: right-sizing
consolidated billing :: shared volume discounts
public IPv4 :: charged hourly
Graviton :: better price/performance
FinOps cycle :: inform, optimise, operate
Enterprise support :: designated TAM

@@drill
Which AWS tool estimates the cost of an architecture before you build it?
= Pricing Calculator
= AWS Pricing Calculator
? Cost Explorer only shows past and forecast spend.

@@drill
Which support plan includes a designated Technical Account Manager?
= Enterprise
? On-Ramp has a pool of TAMs.

@@exercise Build a monthly cost estimate
Estimate the monthly bill from the usage lines using the example prices (illustrative, not current). Data transfer out uses tiers: the first 100 GB free, then $0.09/GB. Print each line's cost, the total by tag `team`, and the three most expensive lines.
@@lang python
@@starter
usage = [
    {"item": "EC2 m7g.large x4", "team": "web", "qty": 4 * 730, "unit_price": 0.0816},
    {"item": "EBS gp3 GB-month", "team": "web", "qty": 800, "unit_price": 0.08},
    {"item": "RDS db.r7g.large Multi-AZ", "team": "data", "qty": 730, "unit_price": 0.48},
    {"item": "S3 Standard GB-month", "team": "data", "qty": 5000, "unit_price": 0.023},
    {"item": "NAT gateway hours", "team": "platform", "qty": 2 * 730, "unit_price": 0.045},
    {"item": "NAT gateway GB processed", "team": "platform", "qty": 3000, "unit_price": 0.045},
    {"item": "Public IPv4 addresses", "team": "platform", "qty": 6 * 730, "unit_price": 0.005},
    {"item": "Data transfer out GB", "team": "web", "qty": 1800, "unit_price": None},
]

def transfer_cost(gb):
    raise NotImplementedError
@@expected
EC2 m7g.large x4            $  238.27
EBS gp3 GB-month            $   64.00
RDS db.r7g.large Multi-AZ   $  350.40
S3 Standard GB-month        $  115.00
NAT gateway hours           $   65.70
NAT gateway GB processed    $  135.00
Public IPv4 addresses       $   21.90
Data transfer out GB        $  153.00
Total                       $1,143.27
  team data      $465.40
  team platform  $222.60
  team web       $455.27
Top 3: RDS db.r7g.large Multi-AZ $350.40; EC2 m7g.large x4 $238.27; Data transfer out GB $153.00
@@requires
def transfer_cost
max(0,
@@hint
`transfer_cost(gb) = max(0, gb - 100) * 0.09`. Collect `(cost, item)` pairs and use `sorted(..., reverse=True)[:3]`.
@@solution
usage = [
    {"item": "EC2 m7g.large x4", "team": "web", "qty": 4 * 730, "unit_price": 0.0816},
    {"item": "EBS gp3 GB-month", "team": "web", "qty": 800, "unit_price": 0.08},
    {"item": "RDS db.r7g.large Multi-AZ", "team": "data", "qty": 730, "unit_price": 0.48},
    {"item": "S3 Standard GB-month", "team": "data", "qty": 5000, "unit_price": 0.023},
    {"item": "NAT gateway hours", "team": "platform", "qty": 2 * 730, "unit_price": 0.045},
    {"item": "NAT gateway GB processed", "team": "platform", "qty": 3000, "unit_price": 0.045},
    {"item": "Public IPv4 addresses", "team": "platform", "qty": 6 * 730, "unit_price": 0.005},
    {"item": "Data transfer out GB", "team": "web", "qty": 1800, "unit_price": None},
]

def transfer_cost(gb):
    return max(0, gb - 100) * 0.09

lines = []
teams = {}
for u in usage:
    cost = transfer_cost(u["qty"]) if u["unit_price"] is None else u["qty"] * u["unit_price"]
    lines.append((cost, u["item"]))
    teams[u["team"]] = teams.get(u["team"], 0) + cost
    print(f"{u['item']:<27} ${cost:>8,.2f}")
print(f"{'Total':<27} ${sum(c for c, _ in lines):>8,.2f}")
for team in sorted(teams):
    print(f"  team {team:<9} ${teams[team]:,.2f}")
print("Top 3:", "; ".join(f"{item} ${cost:,.2f}" for cost, item in sorted(lines, reverse=True)[:3]))

@@exercise Find waste in a resource inventory
Scan the inventory and list savings opportunities with the monthly amount: stopped instances still paying for EBS (`ebs_gb` × $0.08), unattached volumes (size × $0.08), unassociated Elastic IPs ($3.65 each), snapshots older than 365 days (size × $0.05), running instances with average CPU under 10% (suggest the next size down, saving half the instance cost), and resources missing the `CostCenter` tag (no saving, but flag them). Print the findings and the total possible monthly saving.
@@lang python
@@starter
resources = [
    {"id": "i-01", "type": "instance", "state": "running", "cpu": 6, "monthly": 120.0, "ebs_gb": 50, "tags": {"CostCenter": "4410"}},
    {"id": "i-02", "type": "instance", "state": "stopped", "cpu": 0, "monthly": 0.0, "ebs_gb": 200, "tags": {"CostCenter": "4410"}},
    {"id": "i-03", "type": "instance", "state": "running", "cpu": 55, "monthly": 240.0, "ebs_gb": 100, "tags": {}},
    {"id": "vol-07", "type": "volume", "attached": False, "size_gb": 500, "tags": {"CostCenter": "5120"}},
    {"id": "vol-08", "type": "volume", "attached": True, "size_gb": 100, "tags": {"CostCenter": "5120"}},
    {"id": "eipalloc-1", "type": "eip", "associated": False, "tags": {}},
    {"id": "snap-old", "type": "snapshot", "age_days": 900, "size_gb": 300, "tags": {"CostCenter": "4410"}},
    {"id": "snap-new", "type": "snapshot", "age_days": 20, "size_gb": 300, "tags": {"CostCenter": "4410"}},
]
@@expected
i-01        average CPU 6%: downsize one step        $60.00
i-02        stopped instance still paying for EBS    $16.00
i-03        missing CostCenter tag                   $0.00
vol-07      unattached volume                        $40.00
eipalloc-1  unassociated Elastic IP                  $3.65
eipalloc-1  missing CostCenter tag                   $0.00
snap-old    snapshot 900 days old                    $15.00
Possible monthly saving: $134.65
@@requires
def findings
"CostCenter" not in
@@hint
Have `findings(r)` return a list of `(text, saving)` tuples; sum all savings at the end.
@@solution
resources = [
    {"id": "i-01", "type": "instance", "state": "running", "cpu": 6, "monthly": 120.0, "ebs_gb": 50, "tags": {"CostCenter": "4410"}},
    {"id": "i-02", "type": "instance", "state": "stopped", "cpu": 0, "monthly": 0.0, "ebs_gb": 200, "tags": {"CostCenter": "4410"}},
    {"id": "i-03", "type": "instance", "state": "running", "cpu": 55, "monthly": 240.0, "ebs_gb": 100, "tags": {}},
    {"id": "vol-07", "type": "volume", "attached": False, "size_gb": 500, "tags": {"CostCenter": "5120"}},
    {"id": "vol-08", "type": "volume", "attached": True, "size_gb": 100, "tags": {"CostCenter": "5120"}},
    {"id": "eipalloc-1", "type": "eip", "associated": False, "tags": {}},
    {"id": "snap-old", "type": "snapshot", "age_days": 900, "size_gb": 300, "tags": {"CostCenter": "4410"}},
    {"id": "snap-new", "type": "snapshot", "age_days": 20, "size_gb": 300, "tags": {"CostCenter": "4410"}},
]

def findings(r):
    found = []
    kind = r["type"]
    if kind == "instance" and r["state"] == "stopped":
        found.append(("stopped instance still paying for EBS", r["ebs_gb"] * 0.08))
    if kind == "instance" and r["state"] == "running" and r["cpu"] < 10:
        found.append((f"average CPU {r['cpu']}%: downsize one step", r["monthly"] / 2))
    if kind == "volume" and not r["attached"]:
        found.append(("unattached volume", r["size_gb"] * 0.08))
    if kind == "eip" and not r["associated"]:
        found.append(("unassociated Elastic IP", 3.65))
    if kind == "snapshot" and r["age_days"] > 365:
        found.append((f"snapshot {r['age_days']} days old", r["size_gb"] * 0.05))
    if "CostCenter" not in r["tags"]:
        found.append(("missing CostCenter tag", 0.0))
    return found

total = 0.0
for r in resources:
    for text, saving in findings(r):
        total += saving
        print(f"{r['id']:<11} {text:<40} ${saving:,.2f}")
print(f"Possible monthly saving: ${total:,.2f}")

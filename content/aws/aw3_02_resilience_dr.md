@@lesson aw3-resilience Resilience, Disaster Recovery and Route 53
@@text
"Design resilient architectures" is about 26% of the Solutions Architect exam. You need to translate business requirements (RTO, RPO, budget) into a design: multi-AZ for everyday failures, backups and multi-Region strategies for disasters, and Route 53 routing policies to steer users. This lesson ties those decisions together.

## Failure domains

- **Instance/host**: Auto Scaling replaces it; EC2 auto recovery.
- **Availability Zone**: deploy across **at least two AZs** (ALB + ASG across AZs, RDS Multi-AZ, EFS, S3, DynamoDB are multi-AZ by design).
- **Region**: rare, but requires a multi-Region design (backups copied to another Region, cross-Region replicas, Aurora Global Database, DynamoDB Global Tables, S3 CRR).
- **Account/human error**: backups in a separate account with **vault lock**, versioning, Object Lock, least privilege.

Design principles: **loose coupling** (queues), **stateless** app tiers (sessions in ElastiCache/DynamoDB), **automatic recovery**, **health checks**, test failures (**AWS Fault Injection Service**), avoid single points of failure (one NAT gateway, one AZ, one instance).

## RTO and RPO

- **RPO** (Recovery Point Objective): how much **data** loss is acceptable (time since the last good copy).
- **RTO** (Recovery Time Objective): how long until the service is **back**.

## DR strategies

| Strategy | RPO / RTO | Cost | How |
|---|---|---|---|
| **Backup and restore** | hours / hours to a day | lowest | backups (AWS Backup, snapshots) copied to the DR Region; rebuild with IaC |
| **Pilot light** | minutes / tens of minutes | low | core data replicated live (DB replica); app servers off or not deployed, start on failover |
| **Warm standby** | seconds–minutes / minutes | medium | a scaled-down but fully working copy runs; scale up on failover |
| **Multi-site active/active** | near zero / near zero | highest | full capacity in two or more Regions serving traffic |

**AWS Elastic Disaster Recovery** continuously replicates servers (on-premises or EC2) to a low-cost staging area for fast recovery. **AWS Backup** centralises policies, cross-Region and cross-account copies, and **Vault Lock** (WORM).

## Route 53

- Hosted zones (public/private), records (A, AAAA, CNAME, **alias** to AWS resources, including the zone apex, free queries), TTLs.
- **Health checks**: endpoint (HTTP/HTTPS/TCP from global checkers), calculated (combine checks), or based on a **CloudWatch alarm** (for private resources).

| Routing policy | Behaviour | Use |
|---|---|---|
| **Simple** | one record, can return several values (random) | single resource |
| **Weighted** | split traffic by weight | canary/blue-green, A/B testing |
| **Latency** | lowest latency Region for the user | global apps |
| **Failover** | primary/secondary with health checks | active-passive DR |
| **Geolocation** | by user's continent/country/state | content localisation, compliance |
| **Geoproximity** | by distance with a **bias** (Traffic Flow) | shift traffic between Regions |
| **Multivalue answer** | up to 8 healthy records | simple client-side load spreading |
| **IP-based** | by client CIDR | ISP-specific routing |

Records can be combined (latency records each pointing to failover pairs). Remember **DNS caching**: failover speed depends on TTLs and health-check intervals.

## Other edge services

**CloudFront** (caching, origin failover groups, OAC for S3), **Global Accelerator** (anycast static IPs over the AWS backbone, fast regional failover for TCP/UDP, no caching), **Shield** (DDoS: Standard free, Advanced paid with response team and cost protection), **WAF** (layer 7 rules on CloudFront, ALB, API Gateway).

## Example designs

- **RPO 24 h, RTO 24 h, minimal cost**: nightly AWS Backup with cross-Region copy, CloudFormation templates to rebuild.
- **RPO 15 min, RTO 1 h**: pilot light with a cross-Region RDS read replica and AMIs/launch templates ready; Route 53 failover.
- **RPO seconds, RTO minutes**: warm standby with Aurora Global Database and a small ASG in the DR Region.
- **No downtime acceptable**: active-active with DynamoDB Global Tables, latency routing and health checks.

@@quiz
A business can lose at most 15 minutes of data. Which objective is that?
* RPO of 15 minutes
- RTO of 15 minutes
- SLA of 15 minutes
- MTTR of 15 minutes
? RPO is about data, RTO about time to recover.

@@quiz
The cheapest DR strategy that keeps a live database replica in the DR Region, with app servers started only during a disaster, is...
- Backup and restore
* Pilot light
- Warm standby
- Multi-site active/active
? Only the core (data) is running.

@@quiz
Which Route 53 policy sends 10% of users to a new version?
- Latency
- Failover
* Weighted
- Geolocation
? Weights 90 and 10.

@@quiz
Users in Germany must be served from eu-central-1 for compliance. Which routing policy?
- Latency
* Geolocation
- Weighted
- Multivalue
? Latency could route them elsewhere.

@@quiz
Which record type can point the zone apex (example.com) at an ALB?
- CNAME
* Alias
- MX
- PTR
? CNAMEs aren't allowed at the apex.

@@quiz
A gaming service needs static anycast IPs and fast regional failover for UDP traffic. Which service?
- CloudFront
* Global Accelerator
- Route 53 latency routing
- Direct Connect
? It uses the AWS backbone and health checks without DNS caching delays.

@@flashcards
RPO :: acceptable data loss
RTO :: acceptable downtime
backup and restore :: cheapest, hours
pilot light :: live data, app off
warm standby :: scaled-down copy running
multi-site :: active/active, near zero
AWS Backup :: central policies, cross-Region copies
Vault Lock :: WORM backups
Elastic Disaster Recovery :: continuous server replication
alias record :: AWS targets, zone apex, free queries
weighted :: canary, A/B
latency :: lowest latency Region
failover :: active-passive with health checks
geolocation :: by user location
geoproximity :: distance with bias
multivalue :: up to 8 healthy answers
Global Accelerator :: anycast IPs, fast failover
Shield Advanced :: DDoS response team
Fault Injection Service :: chaos testing

@@drill
Which Route 53 routing policy is used for active-passive disaster recovery?
= failover
= Failover
? With health checks on the primary.

@@drill
Which DR strategy runs full capacity in two Regions at once?
= multi-site
= active/active
= multi-site active/active
= Multi-site
? The most expensive option.

@@exercise Choose a DR strategy
For each application pick the cheapest strategy that meets both its RPO and RTO (in minutes), using the typical capabilities in `STRATEGIES` (ordered cheapest first). Print the choice and its relative cost, or `no strategy meets this` if even multi-site can't. Then print the total relative cost.
@@lang python
@@starter
STRATEGIES = [
    ("backup and restore", 24 * 60, 24 * 60, 1),
    ("pilot light", 15, 60, 3),
    ("warm standby", 1, 15, 6),
    ("multi-site active/active", 0, 1, 10),
]
apps = [("intranet wiki", 1440, 2880), ("order system", 5, 30), ("payments", 0, 1), ("reporting", 60, 240), ("trading engine", 0, 0)]
@@expected
intranet wiki   RPO 1440 min, RTO 2880 min -> backup and restore (cost x1)
order system    RPO    5 min, RTO   30 min -> warm standby (cost x6)
payments        RPO    0 min, RTO    1 min -> multi-site active/active (cost x10)
reporting       RPO   60 min, RTO  240 min -> pilot light (cost x3)
trading engine  RPO    0 min, RTO    0 min -> no strategy meets this
Total relative cost: 20
@@requires
def choose
STRATEGIES
@@hint
Each strategy is `(name, best_rpo, best_rto, cost)`. It meets an app when `best_rpo <= rpo and best_rto <= rto`. Return the first one that meets it.
@@solution
STRATEGIES = [
    ("backup and restore", 24 * 60, 24 * 60, 1),
    ("pilot light", 15, 60, 3),
    ("warm standby", 1, 15, 6),
    ("multi-site active/active", 0, 1, 10),
]
apps = [("intranet wiki", 1440, 2880), ("order system", 5, 30), ("payments", 0, 1), ("reporting", 60, 240), ("trading engine", 0, 0)]

def choose(rpo, rto):
    for name, best_rpo, best_rto, cost in STRATEGIES:
        if best_rpo <= rpo and best_rto <= rto:
            return name, cost
    return None, 0

total = 0
for app, rpo, rto in apps:
    name, cost = choose(rpo, rto)
    total += cost
    if name:
        print(f"{app:<15} RPO {rpo:>4} min, RTO {rto:>4} min -> {name} (cost x{cost})")
    else:
        print(f"{app:<15} RPO {rpo:>4} min, RTO {rto:>4} min -> no strategy meets this")
print("Total relative cost:", total)

@@exercise Resolve Route 53 routing policies
Implement DNS answers for three record sets: `failover` (primary if healthy, else secondary if healthy, else primary anyway), `latency` (the healthy Region with the lowest latency for the user's location), and `weighted` (deterministic: the user's `bucket` 0–99 falls into cumulative weight ranges, skipping unhealthy records and rescaling). Print the answer for each query.
@@lang python
@@starter
health = {"eu-west-1": True, "us-east-1": True, "ap-northeast-1": False}
failover = {"primary": "eu-west-1", "secondary": "us-east-1"}
latency_ms = {
    "Tokyo": {"eu-west-1": 230, "us-east-1": 160, "ap-northeast-1": 8},
    "Dublin": {"eu-west-1": 5, "us-east-1": 75, "ap-northeast-1": 220},
    "New York": {"eu-west-1": 70, "us-east-1": 6, "ap-northeast-1": 170},
}
weighted = [("blue", "eu-west-1", 90), ("green", "us-east-1", 10)]
queries = [("failover", None), ("latency", "Tokyo"), ("latency", "Dublin"), ("latency", "New York"), ("weighted", 42), ("weighted", 95)]

def resolve(policy, arg):
    raise NotImplementedError
@@expected
failover          -> eu-west-1
latency Tokyo     -> us-east-1
latency Dublin    -> eu-west-1
latency New York  -> us-east-1
weighted 42       -> blue (eu-west-1)
weighted 95       -> green (us-east-1)
eu-west-1 unhealthy:
  failover -> us-east-1
  latency -> us-east-1
  weighted -> green (us-east-1)
@@requires
def resolve
health[
@@hint
For weighted: keep the healthy records, scale `bucket` to `bucket * total / 100` and walk the cumulative weights. Also try flipping `health["eu-west-1"]` to see failover.
@@solution
health = {"eu-west-1": True, "us-east-1": True, "ap-northeast-1": False}
failover = {"primary": "eu-west-1", "secondary": "us-east-1"}
latency_ms = {
    "Tokyo": {"eu-west-1": 230, "us-east-1": 160, "ap-northeast-1": 8},
    "Dublin": {"eu-west-1": 5, "us-east-1": 75, "ap-northeast-1": 220},
    "New York": {"eu-west-1": 70, "us-east-1": 6, "ap-northeast-1": 170},
}
weighted = [("blue", "eu-west-1", 90), ("green", "us-east-1", 10)]
queries = [("failover", None), ("latency", "Tokyo"), ("latency", "Dublin"), ("latency", "New York"), ("weighted", 42), ("weighted", 95)]

def resolve(policy, arg):
    if policy == "failover":
        if health[failover["primary"]]:
            return failover["primary"]
        if health[failover["secondary"]]:
            return failover["secondary"]
        return failover["primary"]
    if policy == "latency":
        healthy = {region: ms for region, ms in latency_ms[arg].items() if health[region]}
        return min(healthy, key=healthy.get)
    records = [r for r in weighted if health[r[1]]]
    total = sum(w for _, _, w in records)
    point = arg * total / 100
    running = 0
    for name, region, w in records:
        running += w
        if point < running:
            return f"{name} ({region})"
    return f"{records[-1][0]} ({records[-1][1]})"

for policy, arg in queries:
    label = policy if arg is None else f"{policy} {arg}"
    print(f"{label:<17} -> {resolve(policy, arg)}")
health["eu-west-1"] = False
print("eu-west-1 unhealthy:")
for policy, arg in [("failover", None), ("latency", "Dublin"), ("weighted", 42)]:
    print(f"  {policy} -> {resolve(policy, arg)}")

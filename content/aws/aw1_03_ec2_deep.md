@@lesson aw1-ec2-deep EC2 in Depth: Instance Types, Purchasing, EBS and Scaling
@@text
EC2 questions on Cloud Practitioner and Solutions Architect Associate are about choosing: which instance family, which purchasing option, which EBS volume, which load balancer, and how Auto Scaling reacts. This lesson gives you the decision rules and the numbers behind them.

## Instance types

Names read as **family + generation + options . size**: `m7g.large` = general purpose (m), 7th generation, Graviton/ARM (g), large.

| Family | Optimised for | Typical use |
|---|---|---|
| **t** (t3, t4g) | burstable CPU credits | small web servers, dev/test |
| **m** | balanced general purpose | app servers, mid-size databases |
| **c** | compute | batch, media encoding, HPC, game servers |
| **r**, **x** | memory | in-memory caches, large databases, analytics |
| **i**, **d** | storage (local NVMe / HDD) | NoSQL, data warehousing |
| **p**, **g**, **inf**, **trn** | accelerators (GPU, Inferentia, Trainium) | ML training/inference, graphics |

Option letters: **g** Graviton (ARM, better price/performance), **a** AMD, **i** Intel, **d** local NVMe instance store, **n** enhanced networking. Sizes double each step (large → xlarge → 2xlarge...).

**Burstable (t)**: earn CPU credits below baseline, spend them above it. In **unlimited** mode extra usage is billed instead of throttled.

## Purchasing options

| Option | Discount vs On-Demand | Commitment | Best for |
|---|---|---|---|
| **On-Demand** | 0% | none, per second | spiky or unknown workloads, short tests |
| **Savings Plans** (Compute / EC2 Instance) | up to ~66% / ~72% | $/hour for 1 or 3 years | steady usage; Compute SP also covers Fargate and Lambda |
| **Reserved Instances** (Standard / Convertible) | up to ~72% / ~66% | 1 or 3 years | steady usage of specific types; also RDS, ElastiCache, Redshift |
| **Spot** | up to ~90% | none, can be **interrupted with 2 minutes' notice** | fault-tolerant batch, CI, stateless workers |
| **Dedicated Hosts** | varies | optional reservation | per-socket/core licensing (BYOL), compliance |
| **Dedicated Instances** | premium | none | hardware not shared with other customers |
| **Capacity Reservations** | none (On-Demand price) | none | guaranteed capacity in an AZ |

Payment for RIs/SPs: All Upfront > Partial > No Upfront (bigger upfront = bigger discount).

## AMIs, user data and metadata

- **AMI**: a template (root volume snapshot + launch permissions + block device mapping). AMIs are **regional**: copy them to use elsewhere.
- **User data**: a script run at first boot (installs packages, pulls config).
- **Instance metadata** at `169.254.169.254` gives instance ID, role credentials and more; require **IMDSv2** (session tokens) to block SSRF-style credential theft.
- **Launch templates** capture all settings (AMI, type, SGs, user data) for Auto Scaling.

## Storage for EC2

| Volume | Type | Performance | Use |
|---|---|---|---|
| **gp3** | SSD | 3,000 IOPS and 125 MB/s baseline included, provisioned up to 16,000 IOPS / 1,000 MB/s | default for boot and most apps |
| gp2 | SSD | 3 IOPS per GiB (burst to 3,000) | legacy; migrate to gp3 |
| **io2 Block Express** / io1 | provisioned IOPS SSD | up to 256,000 IOPS, sub-millisecond, Multi-Attach | critical databases |
| **st1** | throughput HDD | high MB/s, sequential | big data, logs (not boot) |
| **sc1** | cold HDD | lowest cost | rarely accessed (not boot) |

EBS volumes live in **one AZ**; move them with **snapshots** (incremental, stored in S3, copyable across Regions). **Instance store** is physically attached and very fast but **lost when the instance stops or terminates**. **EFS** is a shared NFS file system across AZs (Linux); **FSx** gives Windows File Server, Lustre (HPC), NetApp ONTAP and OpenZFS.

## Placement groups

**Cluster** (same rack, lowest latency, HPC), **spread** (separate hardware, max 7 instances per AZ per group, critical instances), **partition** (groups of racks, for HDFS/Cassandra/Kafka).

## Elastic Load Balancing

| | ALB | NLB | GWLB |
|---|---|---|---|
| Layer | 7 (HTTP/HTTPS/gRPC) | 4 (TCP/UDP/TLS) | 3 (IP, GENEVE) |
| Routing | path, host, headers, query | ports | to appliance fleets |
| Notes | WebSockets, Lambda targets, auth with Cognito/OIDC | static IP / Elastic IP per AZ, millions of requests/s, preserves source IP | third-party firewalls, IDS |

Health checks remove unhealthy targets; **cross-zone load balancing** spreads evenly across AZs; **connection draining (deregistration delay)** lets in-flight requests finish.

## Auto Scaling

An **Auto Scaling group** keeps between **min** and **max** instances, aiming at **desired**, across AZs. Scaling policies:

- **Target tracking**: "keep average CPU at 50%" (simplest, recommended).
- **Step scaling**: add 2 when CPU > 70%, 4 when > 90%.
- **Scheduled**: scale out at 08:00 on weekdays.
- **Predictive**: forecasts from history.

**Cooldown/warm-up** prevents flapping. ASGs replace instances that fail **EC2 or ELB health checks**. Termination policy (default): balance AZs first, then the oldest launch template/configuration.

@@quiz
A nightly batch job can be restarted if interrupted and must be as cheap as possible. Which option?
- On-Demand
- Reserved Instances
* Spot Instances
- Dedicated Hosts
? Up to 90% off, 2-minute interruption notice.

@@quiz
A database server needs 40,000 IOPS with consistent sub-millisecond latency. Which EBS volume?
- gp2
- st1
* io2 Block Express
- sc1
? gp3 tops out at 16,000 IOPS.

@@quiz
What happens to data on an instance store volume when the instance is stopped?
- It's snapshotted to S3
* It is lost
- It moves to EBS
- It is kept for 30 days
? Instance store is ephemeral.

@@quiz
A web app needs routing by URL path (`/api` vs `/images`). Which load balancer?
* Application Load Balancer
- Network Load Balancer
- Gateway Load Balancer
- Classic Load Balancer
? Path routing is layer 7.

@@quiz
Which purchasing option also applies discounts to Fargate and Lambda?
- Standard Reserved Instances
- EC2 Instance Savings Plans
* Compute Savings Plans
- Spot
? Compute SP is the most flexible.

@@quiz
How do you use an EBS volume's data in another Availability Zone?
- Detach and attach it to an instance in the other AZ
* Create a snapshot and restore a volume from it in the other AZ
- Enable cross-zone load balancing
- Use instance store
? EBS volumes are AZ-scoped.

@@flashcards
m7g.large :: general purpose, gen 7, Graviton, large
t instances :: burstable CPU credits
c / r :: compute / memory optimised
p, g, inf, trn :: accelerated computing
Spot :: up to 90% off, 2-min interruption
Savings Plans :: $/hour commitment, 1 or 3 years
Compute SP :: covers EC2, Fargate, Lambda
Dedicated Host :: BYOL per-socket licences
Capacity Reservation :: guaranteed capacity, no discount
AMI :: regional template
user data :: first-boot script
IMDSv2 :: token-based metadata
gp3 baseline :: 3,000 IOPS, 125 MB/s
io2 Block Express :: up to 256,000 IOPS
st1 / sc1 :: throughput / cold HDD, not boot
instance store :: fast, ephemeral
EBS snapshot :: incremental, cross-AZ/Region
EFS :: shared NFS across AZs
cluster / spread / partition :: latency / isolation / big data
ALB / NLB / GWLB :: L7 / L4 / appliances
target tracking :: keep a metric at a value

@@drill
How much notice does AWS give before reclaiming a Spot Instance?
= 2 minutes
= two minutes
= 2
? Via instance metadata and EventBridge.

@@drill
What IP address serves EC2 instance metadata?
= 169.254.169.254
? Use IMDSv2.

@@exercise Compare purchasing options for a fleet
Using the example hourly prices (illustrative, not current list prices), compute the yearly cost of each workload under On-Demand, a 1-year Compute Savings Plan (−30%), a 3-year one (−50%) and Spot (−70%, only if `interruptible`). Hours per year = `hours_per_day * days`. Recommend: Spot if interruptible; otherwise a 3-year Savings Plan if it runs at least 70% of the year (6,132 hours) and is `long_term`, a 1-year plan if at least 50% (4,380 hours), otherwise On-Demand. Print one line per workload and the total yearly cost of the recommendations.
@@lang python
@@starter
PRICE = {"m7g.large": 0.0816, "c7g.xlarge": 0.145, "r7g.2xlarge": 0.4284, "t4g.medium": 0.0336}
workloads = [
    {"name": "web", "type": "m7g.large", "count": 4, "hours_per_day": 24, "days": 365, "interruptible": False, "long_term": True},
    {"name": "batch", "type": "c7g.xlarge", "count": 10, "hours_per_day": 6, "days": 365, "interruptible": True, "long_term": True},
    {"name": "db-cache", "type": "r7g.2xlarge", "count": 2, "hours_per_day": 24, "days": 365, "interruptible": False, "long_term": False},
    {"name": "office-tools", "type": "t4g.medium", "count": 3, "hours_per_day": 12, "days": 260, "interruptible": False, "long_term": True},
]
@@expected
web            8760 h  on-demand $ 2,859.26 -> 3-year Savings Plan: $1,429.63
batch          2190 h  on-demand $ 3,175.50 -> Spot: $952.65
db-cache       8760 h  on-demand $ 7,505.57 -> 1-year Savings Plan: $5,253.90
office-tools   3120 h  on-demand $   314.50 -> On-Demand: $314.50
Total with recommendations: $7,950.68
@@requires
def recommend
PRICE[
@@hint
`on_demand = PRICE[w["type"]] * w["count"] * hours`. Return `(name, cost)` from `recommend(w)` and add up the costs.
@@solution
PRICE = {"m7g.large": 0.0816, "c7g.xlarge": 0.145, "r7g.2xlarge": 0.4284, "t4g.medium": 0.0336}
workloads = [
    {"name": "web", "type": "m7g.large", "count": 4, "hours_per_day": 24, "days": 365, "interruptible": False, "long_term": True},
    {"name": "batch", "type": "c7g.xlarge", "count": 10, "hours_per_day": 6, "days": 365, "interruptible": True, "long_term": True},
    {"name": "db-cache", "type": "r7g.2xlarge", "count": 2, "hours_per_day": 24, "days": 365, "interruptible": False, "long_term": False},
    {"name": "office-tools", "type": "t4g.medium", "count": 3, "hours_per_day": 12, "days": 260, "interruptible": False, "long_term": True},
]

def recommend(w):
    hours = w["hours_per_day"] * w["days"]
    on_demand = PRICE[w["type"]] * w["count"] * hours
    if w["interruptible"]:
        return "Spot", on_demand * 0.30
    if hours >= 6132 and w["long_term"]:
        return "3-year Savings Plan", on_demand * 0.50
    if hours >= 4380:
        return "1-year Savings Plan", on_demand * 0.70
    return "On-Demand", on_demand

total = 0
for w in workloads:
    hours = w["hours_per_day"] * w["days"]
    on_demand = PRICE[w["type"]] * w["count"] * hours
    name, cost = recommend(w)
    total += cost
    print(f"{w['name']:<13} {hours:>5} h  on-demand ${on_demand:>9,.2f} -> {name}: ${cost:,.2f}")
print(f"Total with recommendations: ${total:,.2f}")

@@exercise Pick an EBS volume and size it
For each requirement choose the cheapest suitable volume type: boot volumes and anything needing random IOPS must be SSD; `gp3` if IOPS ≤ 16,000 and throughput ≤ 1,000 MB/s, otherwise `io2`. Non-boot sequential workloads use `st1` if they need ≥ 250 MB/s, or `sc1` if `rarely_accessed`. For gp3 also print the extra IOPS and throughput to provision above the free baseline (3,000 IOPS, 125 MB/s).
@@lang python
@@starter
needs = [
    {"name": "web-root", "boot": True, "iops": 1000, "mbps": 60, "pattern": "random"},
    {"name": "orders-db", "boot": False, "iops": 40000, "mbps": 800, "pattern": "random"},
    {"name": "app-db", "boot": False, "iops": 8000, "mbps": 250, "pattern": "random"},
    {"name": "log-store", "boot": False, "iops": 500, "mbps": 400, "pattern": "sequential"},
    {"name": "archive", "boot": False, "iops": 100, "mbps": 50, "pattern": "sequential", "rarely_accessed": True},
]
@@expected
web-root: gp3 (+0 IOPS, +0 MB/s above baseline)
orders-db: io2
app-db: gp3 (+5000 IOPS, +125 MB/s above baseline)
log-store: st1
archive: sc1
@@requires
def choose
max(0,
@@hint
Handle sequential non-boot volumes first; everything else follows the SSD rules. `extra_iops = max(0, iops - 3000)`.
@@solution
needs = [
    {"name": "web-root", "boot": True, "iops": 1000, "mbps": 60, "pattern": "random"},
    {"name": "orders-db", "boot": False, "iops": 40000, "mbps": 800, "pattern": "random"},
    {"name": "app-db", "boot": False, "iops": 8000, "mbps": 250, "pattern": "random"},
    {"name": "log-store", "boot": False, "iops": 500, "mbps": 400, "pattern": "sequential"},
    {"name": "archive", "boot": False, "iops": 100, "mbps": 50, "pattern": "sequential", "rarely_accessed": True},
]

def choose(n):
    if not n["boot"] and n["pattern"] == "sequential":
        if n.get("rarely_accessed"):
            return "sc1"
        if n["mbps"] >= 250:
            return "st1"
    if n["iops"] <= 16000 and n["mbps"] <= 1000:
        return "gp3"
    return "io2"

for n in needs:
    vol = choose(n)
    line = f"{n['name']}: {vol}"
    if vol == "gp3":
        extra_iops = max(0, n["iops"] - 3000)
        extra_mbps = max(0, n["mbps"] - 125)
        line += f" (+{extra_iops} IOPS, +{extra_mbps} MB/s above baseline)"
    print(line)

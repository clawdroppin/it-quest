@@lesson aw0-concepts AWS Certifications and Cloud Concepts
@@text
**Amazon Web Services** is the largest cloud provider, and AWS skills are in demand everywhere, very much including Japan (the **Tokyo** region `ap-northeast-1` is one of AWS's biggest; **Osaka** is `ap-northeast-3`). The usual path:

| Exam | Code | For |
|---|---|---|
| **Cloud Practitioner** | CLF-C02 | foundational: concepts, services, security, billing. 65 questions, 90 min, pass 700/1000 |
| **Solutions Architect – Associate** | SAA-C03 | designing secure, resilient, high-performing, cost-optimized systems. 65 questions, 130 min, pass 720 |
| SysOps Administrator / Developer – Associate | | operations / building apps |

CLF-C02 domains: cloud concepts (24%), **security and compliance (30%)**, cloud technology and services (34%), billing, pricing and support (12%). Check aws.amazon.com/certification for current details. AWS Skill Builder has free training and an official practice question set.

**Make a free-tier account for practice**, and on day one: enable **MFA on the root user**, create an admin user in IAM Identity Center for daily work, and set a **budget alert** (Billing → Budgets → ¥1,000). Forgotten resources are how students get surprise bills.

## Why the cloud

Trade **capital expense** (buying servers) for **variable expense** (paying for use); benefit from **economies of scale**; **stop guessing capacity**; increase speed and agility; stop spending money running data centres; **go global in minutes**.

## Global infrastructure

- A **Region** is a geographic area (Tokyo, Frankfurt, N. Virginia). Choose by **compliance/data residency**, **latency** to users, **service availability**, and **price**.
- Each region has multiple **Availability Zones** (AZs): separate data centres with independent power and networking, connected by fast links. Deploy across **at least two AZs** for high availability.
- **Edge locations** (hundreds worldwide) serve **CloudFront** (CDN) and Route 53. **Local Zones**, **Wavelength** (5G) and **Outposts** (AWS racks in your own data centre) bring AWS closer.

## The Well-Architected Framework: 6 pillars

1. **Operational excellence**: run and monitor systems, improve processes (infrastructure as code, small reversible changes).
2. **Security**: identity, traceability, protect data in transit and at rest, prepare for incidents.
3. **Reliability**: recover from failure, scale to demand (multi-AZ, automatic recovery, test recovery).
4. **Performance efficiency**: use the right resource types and sizes; go serverless; experiment.
5. **Cost optimization**: pay only for what you need; measure; use managed services.
6. **Sustainability**: minimize environmental impact.

The **Cloud Adoption Framework (CAF)** has six perspectives: business, people, governance, platform, security, operations. Migration strategies, the **7 Rs**: retire, retain, **rehost** (lift and shift), relocate, repurchase (move to SaaS), **replatform** (lift, tinker and shift), **refactor** (re-architect).

@@quiz
A company must keep customer data inside Japan for legal reasons. What's the main factor in choosing the AWS Region?
* Compliance and data residency: use Tokyo (or Osaka)
- The cheapest region worldwide
- The region with the most edge locations
- Any region, since AWS encrypts data
? Compliance comes first, then latency, services and price.

@@quiz
What is an Availability Zone?
* One or more discrete data centres with redundant power and networking within a Region
- A country
- An edge location for CloudFront
- A single server rack
? Spread workloads across AZs to survive a data-centre failure.

@@quiz
Which Well-Architected pillar is about recovering from failures and meeting demand?
* Reliability
- Performance efficiency
- Operational excellence
- Sustainability
? Performance efficiency is about using resources efficiently.

@@quiz
A company moves its VMs to EC2 without changing them. Which migration strategy is this?
* Rehost (lift and shift)
- Refactor
- Repurchase
- Retire
? Refactor means re-architecting, e.g. to serverless.

@@quiz
Which is a benefit of the cloud listed by AWS?
* Trade capital expense for variable expense
- You no longer need any security
- Fixed monthly prices regardless of usage
- You must buy capacity three years ahead
? Pay as you go instead of buying hardware up front.

@@quiz
What should you do first after creating a new AWS account?
* Enable MFA on the root user and stop using root for daily work
- Create access keys for the root user
- Share the root password with the team
- Launch the largest EC2 instance to test it
? Root can do anything, including closing the account.

@@flashcards
Region :: Geographic area with several AZs (Tokyo = ap-northeast-1)
Availability Zone :: Isolated data centre(s) within a region
edge location :: CDN and DNS points of presence
Outposts :: AWS infrastructure in your own data centre
Well-Architected pillars :: Operational excellence, security, reliability, performance efficiency, cost optimization, sustainability
CAF perspectives :: Business, people, governance, platform, security, operations
7 Rs :: Retire, retain, rehost, relocate, repurchase, replatform, refactor
rehost :: Lift and shift
CLF-C02 :: Cloud Practitioner: 65 questions, 90 min, 700/1000
SAA-C03 :: Solutions Architect Associate: 65 questions, 130 min, 720/1000

@@drill
How many pillars does the AWS Well-Architected Framework have?
= 6
= six
? Sustainability was added as the sixth in 2021.

@@exercise Choose a Region for each workload
Pick a Region the way the lesson orders the factors. Write `choose_region(workload, regions)`:

1. **Compliance**: if the workload has a `country`, only Regions in that country qualify.
2. **Service availability**: the Region must offer every service in `needs`.
3. **Latency**: among those, keep Regions within `max_ms` of the users.
4. **Price**: of the rest, take the cheapest (lowest `price` index). Ties go to the lowest latency.

Return the Region code, or the first factor that ruled everything out (`no region: compliance`, `no region: services` or `no region: latency`).
@@lang python
@@starter
regions = {
    "ap-northeast-1": {"country": "JP", "services": {"EC2", "RDS", "Bedrock", "EKS"}, "price": 1.12, "ms": {"Tokyo": 5, "Osaka": 12, "Frankfurt": 230}},
    "ap-northeast-3": {"country": "JP", "services": {"EC2", "RDS", "EKS"}, "price": 1.12, "ms": {"Tokyo": 12, "Osaka": 4, "Frankfurt": 235}},
    "eu-central-1": {"country": "DE", "services": {"EC2", "RDS", "Bedrock", "EKS"}, "price": 1.08, "ms": {"Tokyo": 230, "Osaka": 235, "Frankfurt": 3}},
    "us-east-1": {"country": "US", "services": {"EC2", "RDS", "Bedrock", "EKS"}, "price": 1.00, "ms": {"Tokyo": 160, "Osaka": 165, "Frankfurt": 95}},
}
workloads = [
    ("Japanese bank records", {"country": "JP", "needs": {"RDS"}, "users": "Osaka", "max_ms": 50}),
    ("Tokyo chatbot", {"country": "JP", "needs": {"EC2", "Bedrock"}, "users": "Tokyo", "max_ms": 50}),
    ("Global batch reports", {"country": None, "needs": {"EC2"}, "users": "Frankfurt", "max_ms": 200}),
    ("German shop", {"country": "DE", "needs": {"EKS"}, "users": "Frankfurt", "max_ms": 20}),
    ("Korean records", {"country": "KR", "needs": {"EC2"}, "users": "Tokyo", "max_ms": 50}),
    ("Osaka game server", {"country": "JP", "needs": {"Bedrock"}, "users": "Osaka", "max_ms": 10}),
]

def choose_region(workload, regions):
    raise NotImplementedError


for name, w in workloads:
    print(f"{name}: {choose_region(w, regions)}")
@@expected
Japanese bank records: ap-northeast-3
Tokyo chatbot: ap-northeast-1
Global batch reports: us-east-1
German shop: eu-central-1
Korean records: no region: compliance
Osaka game server: no region: latency
@@requires
def choose_region
min(
@@hint
Narrow the list of names one factor at a time and return early when it becomes empty. The final pick is `min` with the key `(price, latency)`.
@@solution
regions = {
    "ap-northeast-1": {"country": "JP", "services": {"EC2", "RDS", "Bedrock", "EKS"}, "price": 1.12, "ms": {"Tokyo": 5, "Osaka": 12, "Frankfurt": 230}},
    "ap-northeast-3": {"country": "JP", "services": {"EC2", "RDS", "EKS"}, "price": 1.12, "ms": {"Tokyo": 12, "Osaka": 4, "Frankfurt": 235}},
    "eu-central-1": {"country": "DE", "services": {"EC2", "RDS", "Bedrock", "EKS"}, "price": 1.08, "ms": {"Tokyo": 230, "Osaka": 235, "Frankfurt": 3}},
    "us-east-1": {"country": "US", "services": {"EC2", "RDS", "Bedrock", "EKS"}, "price": 1.00, "ms": {"Tokyo": 160, "Osaka": 165, "Frankfurt": 95}},
}
workloads = [
    ("Japanese bank records", {"country": "JP", "needs": {"RDS"}, "users": "Osaka", "max_ms": 50}),
    ("Tokyo chatbot", {"country": "JP", "needs": {"EC2", "Bedrock"}, "users": "Tokyo", "max_ms": 50}),
    ("Global batch reports", {"country": None, "needs": {"EC2"}, "users": "Frankfurt", "max_ms": 200}),
    ("German shop", {"country": "DE", "needs": {"EKS"}, "users": "Frankfurt", "max_ms": 20}),
    ("Korean records", {"country": "KR", "needs": {"EC2"}, "users": "Tokyo", "max_ms": 50}),
    ("Osaka game server", {"country": "JP", "needs": {"Bedrock"}, "users": "Osaka", "max_ms": 10}),
]

def choose_region(workload, regions):
    names = list(regions)
    if workload["country"]:
        names = [n for n in names if regions[n]["country"] == workload["country"]]
        if not names:
            return "no region: compliance"
    names = [n for n in names if workload["needs"] <= regions[n]["services"]]
    if not names:
        return "no region: services"
    names = [n for n in names if regions[n]["ms"][workload["users"]] <= workload["max_ms"]]
    if not names:
        return "no region: latency"
    return min(names, key=lambda n: (regions[n]["price"], regions[n]["ms"][workload["users"]]))

for name, w in workloads:
    print(f"{name}: {choose_region(w, regions)}")

@@exercise Map review findings to the Well-Architected pillars
A Well-Architected review produced findings. Write `pillar(finding)` that assigns each one to one of the six pillars by keyword, checking in this order:

1. **Security**: `IAM`, `encrypt`, `public`, `MFA`, `root`
2. **Reliability**: `single AZ`, `backup`, `failover`, `recovery`
3. **Cost optimization**: `idle`, `unused`, `on-demand`, `oversized`
4. **Performance efficiency**: `latency`, `cache`, `instance type`
5. **Sustainability**: `utilisation`, `region`, `energy`
6. **Operational excellence**: `manual`, `runbook`, `monitoring`, `deploy`

Print the findings grouped by pillar in the framework's order, with a count, and name the pillar(s) with the most findings (there can be a tie).
@@lang python
@@starter
PILLARS = [
    ("Operational excellence", ["manual", "runbook", "monitoring", "deploy"]),
    ("Security", ["IAM", "encrypt", "public", "MFA", "root"]),
    ("Reliability", ["single AZ", "backup", "failover", "recovery"]),
    ("Performance efficiency", ["latency", "cache", "instance type"]),
    ("Cost optimization", ["idle", "unused", "on-demand", "oversized"]),
    ("Sustainability", ["utilisation", "region", "energy"]),
]
CHECK_ORDER = ["Security", "Reliability", "Cost optimization", "Performance efficiency", "Sustainability", "Operational excellence"]
findings = [
    "Root account has no MFA",
    "Database runs in a single AZ",
    "Deployments are manual SSH sessions",
    "Twelve idle load balancers",
    "S3 bucket with customer files is public",
    "No backup of the build server",
    "Users in Osaka see 300 ms latency; no cache in front of the API",
    "Average CPU utilisation is 4% on the batch fleet",
    "No runbook for the payment outage",
    "Oversized RDS instance for a test database",
]

def pillar(finding):
    raise NotImplementedError


groups = {name: [] for name, _ in PILLARS}
for f in findings:
    groups[pillar(f)].append(f)
for name, items in groups.items():
    print(f"{name} ({len(items)})")
    for f in items:
        print(f"  - {f}")
most = max(len(v) for v in groups.values())
print("Most findings:", ", ".join(n for n, v in groups.items() if len(v) == most))
@@expected
Operational excellence (2)
  - Deployments are manual SSH sessions
  - No runbook for the payment outage
Security (2)
  - Root account has no MFA
  - S3 bucket with customer files is public
Reliability (2)
  - Database runs in a single AZ
  - No backup of the build server
Performance efficiency (1)
  - Users in Osaka see 300 ms latency; no cache in front of the API
Cost optimization (2)
  - Twelve idle load balancers
  - Oversized RDS instance for a test database
Sustainability (1)
  - Average CPU utilisation is 4% on the batch fleet
Most findings: Operational excellence, Security, Reliability, Cost optimization
@@requires
def pillar
.lower()
@@hint
Check the pillars in the given order, but print them in the framework's order: keep both lists. Lowercase both the finding and the keywords so `MFA` matches `mfa`.
@@solution
PILLARS = [
    ("Operational excellence", ["manual", "runbook", "monitoring", "deploy"]),
    ("Security", ["IAM", "encrypt", "public", "MFA", "root"]),
    ("Reliability", ["single AZ", "backup", "failover", "recovery"]),
    ("Performance efficiency", ["latency", "cache", "instance type"]),
    ("Cost optimization", ["idle", "unused", "on-demand", "oversized"]),
    ("Sustainability", ["utilisation", "region", "energy"]),
]
CHECK_ORDER = ["Security", "Reliability", "Cost optimization", "Performance efficiency", "Sustainability", "Operational excellence"]
findings = [
    "Root account has no MFA",
    "Database runs in a single AZ",
    "Deployments are manual SSH sessions",
    "Twelve idle load balancers",
    "S3 bucket with customer files is public",
    "No backup of the build server",
    "Users in Osaka see 300 ms latency; no cache in front of the API",
    "Average CPU utilisation is 4% on the batch fleet",
    "No runbook for the payment outage",
    "Oversized RDS instance for a test database",
]

def pillar(finding):
    words = dict(PILLARS)
    text = finding.lower()
    for name in CHECK_ORDER:
        if any(w.lower() in text for w in words[name]):
            return name
    return "Operational excellence"

groups = {name: [] for name, _ in PILLARS}
for f in findings:
    groups[pillar(f)].append(f)
for name, items in groups.items():
    print(f"{name} ({len(items)})")
    for f in items:
        print(f"  - {f}")
most = max(len(v) for v in groups.values())
print("Most findings:", ", ".join(n for n, v in groups.items() if len(v) == most))

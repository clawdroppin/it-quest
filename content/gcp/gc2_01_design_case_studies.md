@@lesson gc2-design-case-studies Designing for Case Studies: Requirements to Architecture
@@text
## How to read a case study

Every Professional Cloud Architect case study has the same parts: a **company overview**, a **solution concept**, the **existing technical environment**, **business requirements** and **technical requirements**, plus statements from executives. Questions then ask "which design best meets the requirements?".

Read each requirement as a keyword that points to services:

| The case study says | Think of |
|---|---|
| "global users", "low latency worldwide" | global external Application LB, Cloud CDN, multi-region Spanner or Firestore |
| "unpredictable traffic", "spiky", "scale to zero" | Cloud Run, autoscaling MIGs, Pub/Sub buffering |
| "minimize operational overhead", "managed" | serverless and managed services over VMs |
| "lift and shift quickly", "legacy", "licensing" | Compute Engine, Migrate to VMs, sole-tenant nodes |
| "containerized microservices", "portability" | GKE (Autopilot to reduce ops) |
| "analytics", "data warehouse", "ad-hoc SQL" | BigQuery, with Looker |
| "real-time streaming", "IoT telemetry" | Pub/Sub → Dataflow → BigQuery or Bigtable |
| "machine learning", "predictions" | Vertex AI, BigQuery ML |
| "regulatory", "data must stay in the country" | regional resources, organization policy on locations, CMEK, VPC Service Controls |
| "reduce cost" | committed use discounts, Spot VMs for batch, autoscaling, storage classes and lifecycle rules |
| "hybrid", "on-premises data centre stays" | Cloud Interconnect or HA VPN, Cloud Router, Anthos/GKE Enterprise |
| "CI/CD", "release faster" | Cloud Build, Artifact Registry, Cloud Deploy, canary releases |
| "recover from a regional outage" | multi-region design, cross-region replicas, backups in another region, defined RTO/RPO |

## A method for any design question

1. **Underline the constraints**: what must be true (compliance, latency, budget, existing skills).
2. **Eliminate** answers that break a constraint, even if they're technically clever.
3. Between the rest, prefer the **managed**, **simplest** and **most secure** option, and the one that follows Google's recommended practice (groups for IAM, no service account keys, least privilege, infrastructure as code).
4. Watch for **time words**: "quickly" favours lift and shift now, modernize later; "long-term" favours re-architecting.

## Example: a retail company

*A Japanese retailer runs its online shop on VMs in its own Osaka data centre. Traffic grows ten times during sales. The CEO wants to "stop buying servers for peak days". The CTO wants the team to spend less time on patching. Customer data must stay in Japan. They want daily sales dashboards.*

A design that meets every point:

- **Frontend and API**: containers on **Cloud Run** in asia-northeast1 and asia-northeast2 behind a **global external Application Load Balancer** with Cloud CDN and **Cloud Armor**. Scales with sales, no servers to patch.
- **Orders database**: **Cloud SQL for PostgreSQL** with HA (or AlloyDB), in Tokyo with a cross-region replica in Osaka.
- **Data residency**: an **organization policy** restricting resource locations to the Japanese regions.
- **Dashboards**: orders streamed (or exported daily) to **BigQuery**, visualized in Looker Studio.
- **Migration**: first move the database with **Database Migration Service**, run both in parallel, then switch DNS.

## Example: an IoT manufacturer

*Factories send sensor readings every second from 50,000 machines. Engineers need the last 30 days at millisecond latency for dashboards; data scientists need five years of history for analysis.*

- Ingest with **Pub/Sub**, process with **Dataflow**.
- Hot path: **Bigtable** (row key like `machine#reversed_timestamp`) for fast recent reads.
- Cold path: **BigQuery**, partitioned by date, for history and ML; old raw files in **Cloud Storage Archive**.

@@quiz
A case study says "minimize operational overhead" for a stateless web API with spiky traffic. Which compute option fits best?
* Cloud Run
- Compute Engine VMs with a custom autoscaler
- A self-managed Kubernetes cluster on VMs
- Sole-tenant nodes
? Serverless containers scale automatically and need no patching.

@@quiz
Customer data must never be stored outside Japan. Which control enforces this across the organization?
* An organization policy restricting resource locations to Japanese regions
- A label on every bucket
- A firewall rule
- Cloud Armor
? The resource locations constraint stops anyone from creating resources elsewhere.

@@quiz
A company must move 200 TB from its data centre with a slow internet link within a month. What do you recommend?
* Transfer Appliance
- gsutil over the existing link
- Storage Transfer Service over the internet
- Email the data
? A physical appliance is the fastest way when the network is the bottleneck.

@@quiz
Engineers need millisecond reads of recent sensor data per machine; analysts need SQL over years of data. Which pair fits?
* Bigtable for recent data and BigQuery for history
- Cloud SQL for both
- Firestore for both
- Memorystore and Cloud Storage
? A hot path and a cold path, each on the right store.

@@quiz
The question says the company wants to move "as quickly as possible" and modernize "later". Which approach?
* Lift and shift to Compute Engine first
- Rewrite everything as microservices on GKE first
- Wait for a new budget year
- Move only the data
? Time words matter: quick now, modernize afterwards.

@@drill
Which GCP service is a fully managed, globally consistent relational database?
= Cloud Spanner
= Spanner
? Cloud SQL is regional; Spanner scales globally.

@@flashcards
case study parts :: overview, solution concept, environment, business and technical requirements
"minimize operational overhead" :: managed or serverless services
"global, low latency" :: global LB, Cloud CDN, multi-region databases
"data must stay in Japan" :: organization policy on resource locations
hot path / cold path :: Bigtable (recent, fast) / BigQuery (history, analytics)
"quickly" vs "long-term" :: lift and shift now / re-architect later
200 TB, slow link :: Transfer Appliance
streaming pattern :: Pub/Sub → Dataflow → BigQuery

@@exercise Turn case-study statements into requirements
The Professional Cloud Architect case studies mix **business** requirements (cost, compliance, time to market) with **technical** ones (latency, availability, scale). Sort each statement into a category with keyword rules, mark it business or technical, and attach the Google Cloud capability it points to. Then print the requirements grouped by category, in the order the categories first appear.
@@lang python
@@starter
RULES = [
    ("compliance", "business", ("gdpr", "residency", "audit", "pci"), "Assured Workloads, org policy resource locations, Cloud Audit Logs"),
    ("cost", "business", ("cost", "budget", "capex", "spend"), "committed use discounts, autoscaling, budgets and alerts"),
    ("availability", "technical", ("downtime", "99.9", "outage", "failover"), "regional or multi-region deployment, managed instance groups, Cloud SQL HA"),
    ("latency", "technical", ("latency", " ms", "global users"), "global external Application Load Balancer, Cloud CDN"),
    ("scale", "technical", ("spike", "peak", "millions", "scale"), "autoscaling, Cloud Run, Pub/Sub buffering"),
    ("operations", "business", ("ops team", "managed", "toil", "small team"), "managed services (Cloud Run, GKE Autopilot, BigQuery)"),
]
statements = [
    "Customer data for EU users must stay in the EU to meet GDPR.",
    "Reduce infrastructure spend by 30% within a year.",
    "The checkout service must survive a zone outage with no downtime.",
    "Global users complain about page latency above 400 ms.",
    "Traffic spikes 20x during the annual sale.",
    "We have a small team and want less time patching servers.",
    "Move off the ageing data centre before the lease ends.",
]

def classify(statement):
    raise NotImplementedError

groups = {}
for s in statements:
    raise NotImplementedError
for category, items in groups.items():
    print(category.upper())
    for kind, statement, capability in items:
        print(f"  [{kind}] {statement}")
        print(f"      -> {capability}")
@@expected
COMPLIANCE
  [business] Customer data for EU users must stay in the EU to meet GDPR.
      -> Assured Workloads, org policy resource locations, Cloud Audit Logs
COST
  [business] Reduce infrastructure spend by 30% within a year.
      -> committed use discounts, autoscaling, budgets and alerts
AVAILABILITY
  [technical] The checkout service must survive a zone outage with no downtime.
      -> regional or multi-region deployment, managed instance groups, Cloud SQL HA
LATENCY
  [technical] Global users complain about page latency above 400 ms.
      -> global external Application Load Balancer, Cloud CDN
SCALE
  [technical] Traffic spikes 20x during the annual sale.
      -> autoscaling, Cloud Run, Pub/Sub buffering
OPERATIONS
  [business] We have a small team and want less time patching servers.
      -> managed services (Cloud Run, GKE Autopilot, BigQuery)
OTHER
  [business] Move off the ageing data centre before the lease ends.
      -> clarify with the stakeholders
@@requires
def classify
groups.setdefault(
@@hint
`classify` lowercases the statement and returns the first rule whose keywords appear in it, as `(category, kind, capability)`; return `("other", "business", "clarify with the stakeholders")` when nothing matches. In the loop, `groups.setdefault(category, []).append((kind, statement, capability))` keeps first-appearance order.
@@solution
RULES = [
    ("compliance", "business", ("gdpr", "residency", "audit", "pci"), "Assured Workloads, org policy resource locations, Cloud Audit Logs"),
    ("cost", "business", ("cost", "budget", "capex", "spend"), "committed use discounts, autoscaling, budgets and alerts"),
    ("availability", "technical", ("downtime", "99.9", "outage", "failover"), "regional or multi-region deployment, managed instance groups, Cloud SQL HA"),
    ("latency", "technical", ("latency", " ms", "global users"), "global external Application Load Balancer, Cloud CDN"),
    ("scale", "technical", ("spike", "peak", "millions", "scale"), "autoscaling, Cloud Run, Pub/Sub buffering"),
    ("operations", "business", ("ops team", "managed", "toil", "small team"), "managed services (Cloud Run, GKE Autopilot, BigQuery)"),
]
statements = [
    "Customer data for EU users must stay in the EU to meet GDPR.",
    "Reduce infrastructure spend by 30% within a year.",
    "The checkout service must survive a zone outage with no downtime.",
    "Global users complain about page latency above 400 ms.",
    "Traffic spikes 20x during the annual sale.",
    "We have a small team and want less time patching servers.",
    "Move off the ageing data centre before the lease ends.",
]

def classify(statement):
    text = statement.lower()
    for category, kind, keywords, capability in RULES:
        if any(k in text for k in keywords):
            return category, kind, capability
    return "other", "business", "clarify with the stakeholders"

groups = {}
for s in statements:
    category, kind, capability = classify(s)
    groups.setdefault(category, []).append((kind, s, capability))
for category, items in groups.items():
    print(category.upper())
    for kind, statement, capability in items:
        print(f"  [{kind}] {statement}")
        print(f"      -> {capability}")

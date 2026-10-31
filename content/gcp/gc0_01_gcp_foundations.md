@@lesson gc0-gcp-foundations Google Cloud: The Exam, Hierarchy and Core Services
@@text
## The Professional Cloud Architect exam

The **Google Cloud Professional Cloud Architect** (PCA) exam is 2 hours with 50–60 multiple-choice and multiple-select questions. About a quarter of them are about **case studies**: fictional companies that Google publishes in advance (currently *Altostrat Media*, *Cymbal Retail*, *EHR Healthcare* and *KnightMotives Automotive*). Google replaces them from time to time, so check the current exam guide, and read them before the exam: the questions expect you to know each company's business and technical requirements.

Domains: **designing and planning** a solution architecture (~25%), **managing and provisioning** infrastructure (~17.5%), **security and compliance** (~17.5%), **analyzing and optimizing** technical and business processes (~15%), **managing implementation** (~12.5%), **ensuring reliability** (~12.5%).

It's a design exam: most questions describe requirements and ask for the *best* option, which is usually the **managed** service that meets every requirement with the least operational work. If you know AWS, the table at the end of this lesson maps the services.

Google Cloud is used by many Japanese companies (for example in retail, gaming and media), and Google has regions in **Tokyo (asia-northeast1)** and **Osaka (asia-northeast2)**.

## Resource hierarchy

```text
Organization (example.com)          ← tied to a Cloud Identity / Workspace domain
 ├── Folder: Production
 │    ├── Project: shop-prod        ← billing, APIs and quotas are per project
 │    └── Project: data-prod
 └── Folder: Development
      └── Project: shop-dev
```

- **IAM policies are inherited** downwards: a role granted on a folder applies to every project in it. A child can't remove what a parent grants (except with *deny policies*).
- **Organization policies** (constraints) restrict *what* can be done, whatever IAM says: for example "VMs may not have external IPs", "resources only in asia-northeast1", "no service account keys".
- A project has a **name**, a globally unique **project ID** (permanent) and a **project number**.
- **Labels** (key: value) on resources are used for cost reporting.

## Regions, zones and the network

- A **region** (asia-northeast1) has 3 or more **zones** (asia-northeast1-a, -b, -c). Zonal resources (a VM, a persistent disk) die with their zone; regional (regional disk, Cloud SQL HA, regional MIG) survive a zone failure; multi-region (Cloud Storage `asia`, Spanner multi-region configs) survive a region failure.
- A **VPC network is global** (unlike AWS): one VPC can have **subnets** in every region, and VMs in different regions talk over Google's private backbone with no peering needed. Subnets are regional.
- **Firewall rules** belong to the VPC and target instances by **network tags** or service accounts. There's an implied *deny all ingress* and *allow all egress*.
- Connecting VPCs: **VPC Network Peering** (not transitive), **Shared VPC** (a host project owns the network; service projects use its subnets: central network control for an organization), **Private Service Connect**.
- Hybrid: **Cloud VPN** (HA VPN: 99.99% SLA with two tunnels), **Cloud Interconnect** (Dedicated 10/100 Gbps, or Partner), routes exchanged with **Cloud Router** (BGP).
- **Private Google Access** lets VMs without external IPs reach Google APIs; **Cloud NAT** gives them outbound internet access.

## Compute: choosing the right one

| Service | What you manage | Best for |
|---|---|---|
| **Compute Engine** | VMs (OS and up) | lift-and-shift, custom OS, licensed software |
| **Managed instance groups** | an instance template | autoscaling, autohealing, rolling updates of VMs |
| **GKE** (Standard / Autopilot) | containers (Autopilot also hides the nodes) | microservices, portability, complex container apps |
| **Cloud Run** | a container image | stateless HTTP services and jobs, scales to zero |
| **Cloud Run functions** (formerly Cloud Functions) | a function | event glue: "when a file lands in a bucket…" |
| **App Engine** | code (standard or flexible) | older PaaS web apps |

**Spot VMs** (formerly preemptible) are up to 60–91% cheaper but can be stopped at any time: batch jobs, rendering, CI. **Sole-tenant nodes** for licensing or compliance. **Committed use discounts** (1 or 3 years) and automatic **sustained use discounts** lower VM prices.

## Storage and databases

| Need | Service |
|---|---|
| objects, files, backups, data lake | **Cloud Storage** (classes: Standard, Nearline 30 days, Coldline 90 days, Archive 365 days) |
| block disk for a VM | **Persistent Disk** / **Hyperdisk**; Local SSD (fast, but lost when the VM stops) |
| shared file system (NFS) | **Filestore** |
| relational, regional (MySQL, PostgreSQL, SQL Server) | **Cloud SQL** (HA with a standby in another zone) |
| PostgreSQL-compatible, high performance | **AlloyDB** |
| relational, **global**, horizontally scalable, strong consistency | **Cloud Spanner** |
| NoSQL documents, mobile and web apps | **Firestore** |
| wide-column, huge throughput, low latency (IoT, time series, ads) | **Bigtable** |
| in-memory cache | **Memorystore** (Redis, Memcached) |
| analytics data warehouse, SQL over petabytes | **BigQuery** |

Exam keywords: "global, relational, consistent, 99.999%" → **Spanner**. "Time series / IoT, millions of writes per second, single-digit ms" → **Bigtable**. "Analytics / data warehouse / ad-hoc SQL" → **BigQuery**. "Existing MySQL app, lift and shift" → **Cloud SQL**.

## Data and messaging

- **Pub/Sub**: global messaging, decouples producers and consumers (like SNS + SQS).
- **Dataflow**: managed Apache Beam, for batch and streaming pipelines.
- **Dataproc**: managed Hadoop/Spark (for existing Spark jobs).
- **Cloud Composer**: managed Apache Airflow (workflow orchestration).
- A classic streaming design: **Pub/Sub → Dataflow → BigQuery**, with Looker for dashboards.

## AWS → Google Cloud

| AWS | Google Cloud |
|---|---|
| EC2 / Auto Scaling group | Compute Engine / managed instance group |
| EKS / Fargate / Lambda | GKE / Cloud Run / Cloud Run functions |
| S3 / EBS / EFS | Cloud Storage / Persistent Disk / Filestore |
| RDS / Aurora / DynamoDB | Cloud SQL / AlloyDB / Firestore or Bigtable |
| Redshift / Kinesis / SQS+SNS | BigQuery / Dataflow + Pub/Sub / Pub/Sub |
| VPC (regional) | VPC (**global**) |
| Route 53 / CloudFront | Cloud DNS / Cloud CDN |
| CloudWatch / CloudTrail | Cloud Monitoring and Logging / Cloud Audit Logs |
| CloudFormation | Infrastructure Manager (Terraform) |

@@quiz
A company needs a relational database for a global application with strong consistency and 99.999% availability. Which service fits?
* Cloud Spanner
- Cloud SQL
- Bigtable
- Firestore
? Cloud SQL is regional. Bigtable and Firestore aren't relational.

@@quiz
Which statement about VPC networks in Google Cloud is true?
* A VPC is global; its subnets are regional
- A VPC is regional, like in AWS
- Subnets are zonal
- VMs in different regions need VPC peering to talk
? This is one of the biggest differences from AWS.

@@quiz
A central network team must control all firewall rules and subnets, while application teams deploy VMs in their own projects. What should they use?
* Shared VPC with a host project and service projects
- VPC Network Peering between every project
- One project for everyone
- Cloud VPN between projects
? Shared VPC centralizes the network but keeps billing and IAM per project.

@@quiz
A batch rendering job can be interrupted and restarted. How do you make it as cheap as possible?
* Run it on Spot VMs
- Use sole-tenant nodes
- Buy a 3-year commitment
- Use Local SSDs
? Spot VMs are heavily discounted but can be reclaimed at any time.

@@quiz
Security requires that no VM in the organization can have an external IP address, whatever IAM roles people have. What enforces this?
* An organization policy constraint
- An IAM deny on the Compute Admin role
- A firewall rule
- A label on every project
? Organization policies restrict configurations; IAM controls who can act.

@@quiz
Sensor data arrives at 2 million writes per second and must be read back by device and time in a few milliseconds. Which database?
* Bigtable
- BigQuery
- Cloud SQL
- Spanner
? BigQuery is for analytics, not low-latency serving.

@@flashcards
resource hierarchy :: organization → folders → projects → resources
organization policy :: constraints on what can be configured
project ID :: globally unique, permanent
VPC scope :: global (subnets are regional)
Shared VPC :: host project owns the network, service projects use it
HA VPN SLA :: 99.99% (two tunnels)
Cloud Router :: dynamic BGP routes for VPN and Interconnect
Private Google Access :: VMs without external IPs reach Google APIs
Cloud Run :: serverless containers, scale to zero
Spanner :: global, relational, strongly consistent
Bigtable :: wide-column NoSQL, huge throughput, low latency
BigQuery :: serverless data warehouse
Pub/Sub → Dataflow → BigQuery :: the classic streaming pipeline
Cloud Storage classes :: Standard, Nearline (30 d), Coldline (90 d), Archive (365 d)

@@drill
What's the minimum storage duration (in days) of the Coldline class?
= 90

@@drill
What's the region name of Tokyo in Google Cloud?
= asia-northeast1

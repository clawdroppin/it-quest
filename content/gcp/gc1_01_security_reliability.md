@@lesson gc1-security-reliability Security, IAM and Reliability on Google Cloud
@@text
## IAM

A **policy** on a resource is a list of **bindings**: a **role** given to **members** (principals).

```json
{
  "bindings": [
    { "role": "roles/storage.objectViewer", "members": ["group:analysts@example.com"] },
    { "role": "roles/run.invoker", "members": ["serviceAccount:frontend@shop-prod.iam.gserviceaccount.com"] }
  ]
}
```

- **Members**: `user:`, `group:` (best practice: grant to groups, not people), `serviceAccount:`, `domain:`, and the dangerous `allUsers` (anyone on the internet) and `allAuthenticatedUsers` (anyone with *any* Google account).
- **Roles**:
  - **basic** (formerly primitive): `roles/owner`, `roles/editor`, `roles/viewer`. Far too broad for production.
  - **predefined**: per service, like `roles/compute.instanceAdmin.v1` or `roles/bigquery.dataViewer`.
  - **custom**: your own list of permissions.
- **Conditions** limit a binding, for example until a date or only for resources with a certain name prefix.
- **Service accounts** are identities for workloads. Attach them to VMs, Cloud Run services or GKE workloads (**Workload Identity**) instead of creating **keys**: a downloaded JSON key is a long-lived secret that leaks. For outside systems (GitHub Actions, AWS), use **Workload Identity Federation**.
- **Separation of duties**: the person who approves billing isn't the one who deploys.

You'll write an auditor for exactly these policy files in this track's project.

## Protecting data and networks

- **Encryption at rest** is always on (Google-managed keys). For control: **CMEK** (customer-managed keys in **Cloud KMS**), or **CSEK**/**EKM** (keys that never live in Google). **Cloud HSM** for FIPS 140-2 level 3.
- **Secret Manager** for passwords, API keys and certificates: versioned, IAM-controlled, audited.
- **VPC Service Controls**: a perimeter around Google APIs (BigQuery, Cloud Storage) that stops data from being copied out to projects outside it, even by someone with valid credentials. The answer to "prevent data exfiltration".
- **Identity-Aware Proxy (IAP)**: access to internal web apps and SSH by identity and context, with no VPN (BeyondCorp, zero trust).
- **Cloud Armor**: WAF and DDoS protection on the external HTTP(S) load balancer.
- **Security Command Center**: finds misconfigurations and threats across the organization.
- **Cloud Audit Logs**: *Admin Activity* (always on, free), *Data Access* (off by default except BigQuery: turn on for sensitive data), *System Event*, *Policy Denied*. Export them with a **log sink** to Cloud Storage (long retention), BigQuery (analysis) or Pub/Sub (a SIEM).
- **Sensitive Data Protection** (formerly DLP) finds and masks personal data like credit card numbers or My Number (マイナンバー).

Compliance in Japan: personal data falls under the **APPI** (個人情報保護法); Google Cloud also supports ISMAP (the government's cloud security list) and FISC guidelines for financial institutions.

## Load balancing

| Load balancer | Layer | Scope |
|---|---|---|
| **Global external Application LB** | 7 (HTTP/S) | one anycast IP worldwide, routes to the nearest healthy backend; Cloud CDN, Cloud Armor |
| Regional external/internal Application LB | 7 | one region |
| **External passthrough Network LB** | 4 (TCP/UDP) | regional, keeps client IPs |
| **Internal passthrough Network LB** | 4 | private, regional |
| Proxy Network LB | 4 (TCP with SSL offload) | global or regional |

## Reliability

- **SLA / SLO / SLI**: the SLI is the measurement (the share of successful requests), the SLO the target (99.9% over 30 days), the SLA the contract with penalties. The **error budget** is 100% minus the SLO.
- **Composite availability**: services in series multiply. A 99.95% load balancer in front of a 99.95% VM group in front of a 99.95% Cloud SQL: 0.9995³ ≈ **99.85%**. Two independent copies in parallel: 1 − (1 − A)². Redundancy across **zones**, then **regions**, raises it.
- **Disaster recovery**: define **RTO** (how long you can be down) and **RPO** (how much data you can lose). Patterns from cheap and slow to expensive and fast: **cold** (backups restored on demand), **warm** (a scaled-down copy running), **hot** (active-active in several regions).
- Backups: Cloud SQL automated backups and point-in-time recovery, persistent disk **snapshots** (can be multi-regional), Cloud Storage **versioning** and **retention policies / bucket lock** (WORM).
- **Operations**: Cloud Monitoring (metrics, uptime checks, alerting), Cloud Logging, Cloud Trace and Profiler. Roll out safely with **canary** or **blue/green** deployments (Cloud Deploy, traffic splitting in Cloud Run).

## Migration and cost

- Migration approaches: **lift and shift** (Migrate to Virtual Machines), **improve and move** (replatform: containers, managed databases), **rip and replace** (rewrite cloud-native). Database Migration Service for MySQL/PostgreSQL/SQL Server; **Storage Transfer Service** or **Transfer Appliance** (a physical device, for hundreds of TB) for data.
- Cost: the **pricing calculator**, **budgets and alerts** (they alert; they don't stop spending on their own), **billing export to BigQuery** with labels for showback, committed use discounts, rightsizing **recommendations**, autoscaling to zero with Cloud Run.

@@quiz
A data scientist's laptop has a downloaded service account key. What's the better practice?
* Don't create keys: use the person's own identity, or Workload Identity Federation for outside systems
- Rotate the key every year
- Email the key to the team so it's backed up
- Give the service account Owner so it never fails
? Long-lived keys are the most common cause of Google Cloud credential leaks.

@@quiz
A company must ensure that data in BigQuery can't be copied to a project outside the company, even by an insider with valid credentials. What should they use?
* VPC Service Controls
- Firewall rules
- Cloud Armor
- Customer-managed encryption keys
? VPC Service Controls draw a perimeter around Google-managed APIs.

@@quiz
Which IAM member gives access to anyone on the internet, without signing in?
* allUsers
- allAuthenticatedUsers
- domain:example.com
- group:everyone@example.com
? allAuthenticatedUsers is anyone with any Google account, which is almost as bad.

@@quiz
Three services in series each have 99.9% availability. What's the approximate availability of the whole?
* 99.7%
- 99.9%
- 99.99%
- 97%
? 0.999³ ≈ 0.997.

@@quiz
Employees need access to an internal web app from anywhere, based on their identity and device, without a VPN. What fits?
* Identity-Aware Proxy (IAP)
- Cloud VPN
- Cloud NAT
- A public IP with a password
? IAP is Google's zero-trust (BeyondCorp) access.

@@quiz
Users worldwide must reach a web application on one IP address, routed to the nearest healthy region, with WAF protection. What do you deploy?
* A global external Application Load Balancer with Cloud Armor
- A regional passthrough Network Load Balancer
- Cloud DNS round robin
- An internal Application Load Balancer
? The global Application LB has one anycast IP; Cloud Armor attaches to it.

@@flashcards
basic roles :: owner, editor, viewer: too broad for production
allUsers / allAuthenticatedUsers :: anyone / anyone with a Google account
Workload Identity Federation :: outside systems use Google Cloud without keys
CMEK :: customer-managed keys in Cloud KMS
VPC Service Controls :: perimeter against data exfiltration
IAP :: identity-based access without a VPN
Cloud Armor :: WAF and DDoS protection
Data Access audit logs :: off by default (except BigQuery)
log sink :: export logs to Storage, BigQuery or Pub/Sub
composite availability in series :: multiply the availabilities
RTO / RPO :: downtime allowed / data loss allowed
Transfer Appliance :: physical device for huge data migrations
budget alerts :: notify; they don't stop spending

@@drill
Two independent copies with 99% availability each run in parallel. What's the combined availability, in percent?
= 99.99
= 99.99%
? 1 − (0.01 × 0.01) = 0.9999.

@@exercise Evaluate a VPC Service Controls perimeter
A service perimeter protects **restricted services** (here Cloud Storage and BigQuery) for the projects inside it. A call to a restricted service is allowed when the caller and the target project are in the same perimeter. A call from **outside** into the perimeter needs a matching **ingress rule**; a call from **inside** to a project outside needs a matching **egress rule**. Services that aren't restricted aren't checked by the perimeter (IAM still applies). Evaluate each request and say why.
@@lang python
@@starter
perimeter = {
    "projects": {"prj-data", "prj-analytics"},
    "restricted": {"storage.googleapis.com", "bigquery.googleapis.com"},
    "ingress": [{"identity": "sa-etl@prj-ingest.iam.gserviceaccount.com", "source": "prj-ingest", "service": "storage.googleapis.com"}],
    "egress": [{"identity": "sa-report@prj-analytics.iam.gserviceaccount.com", "target": "prj-partner", "service": "bigquery.googleapis.com"}],
}
requests = [
    ("sa-bi@prj-analytics.iam.gserviceaccount.com", "prj-analytics", "prj-data", "bigquery.googleapis.com"),
    ("sa-etl@prj-ingest.iam.gserviceaccount.com", "prj-ingest", "prj-data", "storage.googleapis.com"),
    ("sa-etl@prj-ingest.iam.gserviceaccount.com", "prj-ingest", "prj-data", "bigquery.googleapis.com"),
    ("dev@example.com", "laptop", "prj-data", "storage.googleapis.com"),
    ("sa-report@prj-analytics.iam.gserviceaccount.com", "prj-analytics", "prj-partner", "bigquery.googleapis.com"),
    ("sa-bi@prj-analytics.iam.gserviceaccount.com", "prj-analytics", "prj-partner", "storage.googleapis.com"),
    ("dev@example.com", "laptop", "prj-data", "compute.googleapis.com"),
]

def evaluate(identity, source, target, service):
    raise NotImplementedError

for identity, source, target, service in requests:
    print(f"{identity.split('@')[0]:<10} {source:>13} -> {target:<13} {service.split('.')[0]:<9} {evaluate(identity, source, target, service)}")
@@expected
sa-bi      prj-analytics -> prj-data      bigquery  allowed (both inside the perimeter)
sa-etl        prj-ingest -> prj-data      storage   allowed (ingress rule)
sa-etl        prj-ingest -> prj-data      bigquery  DENIED (no ingress rule)
dev               laptop -> prj-data      storage   DENIED (no ingress rule)
sa-report  prj-analytics -> prj-partner   bigquery  allowed (egress rule)
sa-bi      prj-analytics -> prj-partner   storage   DENIED (no egress rule)
dev               laptop -> prj-data      compute   allowed (service not restricted; IAM still applies)
@@requires
def evaluate
perimeter["egress"]
@@hint
If the service isn't in `restricted`, return "allowed (service not restricted)". Work out whether the source and target are inside (`in perimeter["projects"]`). Both inside: allowed. Outside to inside: look for an ingress rule matching identity, source and service. Inside to outside: look in `perimeter["egress"]` for a rule matching identity, target and service. Otherwise it's denied, naming the missing rule.
@@solution
perimeter = {
    "projects": {"prj-data", "prj-analytics"},
    "restricted": {"storage.googleapis.com", "bigquery.googleapis.com"},
    "ingress": [{"identity": "sa-etl@prj-ingest.iam.gserviceaccount.com", "source": "prj-ingest", "service": "storage.googleapis.com"}],
    "egress": [{"identity": "sa-report@prj-analytics.iam.gserviceaccount.com", "target": "prj-partner", "service": "bigquery.googleapis.com"}],
}
requests = [
    ("sa-bi@prj-analytics.iam.gserviceaccount.com", "prj-analytics", "prj-data", "bigquery.googleapis.com"),
    ("sa-etl@prj-ingest.iam.gserviceaccount.com", "prj-ingest", "prj-data", "storage.googleapis.com"),
    ("sa-etl@prj-ingest.iam.gserviceaccount.com", "prj-ingest", "prj-data", "bigquery.googleapis.com"),
    ("dev@example.com", "laptop", "prj-data", "storage.googleapis.com"),
    ("sa-report@prj-analytics.iam.gserviceaccount.com", "prj-analytics", "prj-partner", "bigquery.googleapis.com"),
    ("sa-bi@prj-analytics.iam.gserviceaccount.com", "prj-analytics", "prj-partner", "storage.googleapis.com"),
    ("dev@example.com", "laptop", "prj-data", "compute.googleapis.com"),
]

def evaluate(identity, source, target, service):
    if service not in perimeter["restricted"]:
        return "allowed (service not restricted; IAM still applies)"
    src_in = source in perimeter["projects"]
    dst_in = target in perimeter["projects"]
    if src_in and dst_in:
        return "allowed (both inside the perimeter)"
    if dst_in:
        for rule in perimeter["ingress"]:
            if (rule["identity"], rule["source"], rule["service"]) == (identity, source, service):
                return "allowed (ingress rule)"
        return "DENIED (no ingress rule)"
    if src_in:
        for rule in perimeter["egress"]:
            if (rule["identity"], rule["target"], rule["service"]) == (identity, target, service):
                return "allowed (egress rule)"
        return "DENIED (no egress rule)"
    return "allowed (neither side is in the perimeter)"

for identity, source, target, service in requests:
    print(f"{identity.split('@')[0]:<10} {source:>13} -> {target:<13} {service.split('.')[0]:<9} {evaluate(identity, source, target, service)}")

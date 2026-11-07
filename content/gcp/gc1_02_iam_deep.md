@@lesson gc1-iam-deep IAM, Organization Policies and Security Operations in Depth
@@text
Google Cloud security questions revolve around three ideas: IAM policies inherit down the resource hierarchy and are additive; service accounts are identities for workloads (and their keys are a liability); and organisation policies restrict *what* can be configured regardless of who has permission. This lesson adds deny policies, conditions, audit logs and the security services you'll configure.

## IAM recap and inheritance

- A policy binds **principals** (Google accounts, groups, service accounts, Workspace/Cloud Identity domains, `allUsers`, `allAuthenticatedUsers`, workforce/workload identity pools) to **roles** on a resource.
- Policies set on the **organisation, folders, projects and resources** are inherited downward; effective access is the **union** of all allow policies above. You **can't remove** an inherited grant at a lower level with an allow policy.
- Role types: **basic** (Owner, Editor, Viewer: too broad for production), **predefined** (`roles/storage.objectViewer`, `roles/compute.instanceAdmin.v1`), **custom** (your list of permissions; can't be used at folder level if they include project-only permissions, and need maintenance as permissions change).
- **Grant to groups**, at the lowest level that works, with predefined roles; review with **IAM Recommender** (removes unused permissions) and **Policy Analyzer** ("who can access this?").

## Deny policies and conditions

- **IAM deny policies** (attached at org, folder or project) **override allows**: e.g. deny `iam.serviceAccountKeys.create` to everyone except a break-glass group. Evaluation: deny policies first, then allow policies.
- **IAM Conditions** add CEL expressions to a binding: time-limited access (`request.time < timestamp("2026-12-31T00:00:00Z")`), resource name prefixes, tags. Basic roles can't be conditional.
- **Privileged Access Manager** grants temporary elevated roles with approval.

## Service accounts

- An identity for workloads: attached to VMs, Cloud Run services, GKE (via Workload Identity Federation for GKE), Cloud Run functions.
- **Avoid service account keys** (long-lived JSON credentials that leak). Alternatives: attached service accounts, **impersonation** (`roles/iam.serviceAccountTokenCreator` + `--impersonate-service-account`), **Workload Identity Federation** for workloads in AWS, Azure, GitHub Actions or on-premises (OIDC/SAML exchange for short-lived tokens).
- Default compute service account gets Editor in older projects: replace with dedicated least-privilege accounts per workload (an org policy can stop automatic grants).
- `roles/iam.serviceAccountUser` lets someone **act as** (deploy with) a service account: grant it carefully.

## Organization policies

Constraints set at org/folder/project that limit configuration for **everyone**:

| Constraint | Effect |
|---|---|
| `gcp.resourceLocations` | allowed regions for resources |
| `iam.disableServiceAccountKeyCreation` | no new SA keys |
| `iam.allowedPolicyMemberDomains` | domain-restricted sharing (only your org's identities) |
| `compute.vmExternalIpAccess` | which VMs may have external IPs |
| `storage.publicAccessPrevention` | no public buckets |
| `compute.skipDefaultNetworkCreation` | no default network in new projects |
| `sql.restrictPublicIp` | Cloud SQL without public IP |

List constraints allow/deny values; boolean constraints enforce on/off. Policies inherit, and lower levels can **merge** with or **replace** the parent policy unless prevented. **Custom constraints** use CEL on resource fields.

## Audit logs

| Log | Enabled | Contains |
|---|---|---|
| **Admin Activity** | always, free, 400-day retention | configuration changes (create VM, set IAM policy) |
| **System Event** | always | Google-initiated changes (live migration) |
| **Data Access** | **off by default** (except BigQuery), can be costly | reads of configuration/data (`storage.objects.get`) |
| **Policy Denied** | on | requests denied by VPC Service Controls/org policy |

Route logs with **sinks** to Cloud Storage (archive), BigQuery (analysis) or Pub/Sub (SIEM); aggregate at org level.

## Security services

**Security Command Center** (posture, misconfigurations, threats, attack paths), **Cloud KMS** (CMEK, HSM, Cloud External Key Manager), **Secret Manager** (versioned secrets with IAM), **VPC Service Controls** (service perimeters around APIs to stop data exfiltration), **Identity-Aware Proxy** (context-aware access to apps and SSH without VPN), **Cloud Armor**, **Binary Authorization** (only signed images on GKE/Cloud Run), **Sensitive Data Protection** (DLP), **Assured Workloads** (compliance regimes).

@@quiz
A user has Viewer on a folder. A project admin wants to remove that access for one project inside it. What works?
- Remove Viewer from the project's allow policy
* Use an IAM deny policy on the project (or change the folder grant)
- Grant a custom role with fewer permissions
- Nothing can restrict it
? Allow policies are additive; inherited grants can't be removed below.

@@quiz
A CI pipeline in GitHub Actions needs to deploy to Google Cloud without long-lived keys. What do you use?
- A service account JSON key in repository secrets
* Workload Identity Federation with GitHub's OIDC tokens
- A user's personal credentials
- allAuthenticatedUsers
? Short-lived tokens exchanged for SA access.

@@quiz
Which audit logs are disabled by default and must be enabled per service?
- Admin Activity
- System Event
* Data Access
- Policy Denied
? Except BigQuery, which logs data access by default.

@@quiz
Company policy: only identities from the company domain may be granted IAM roles. Which control?
- VPC Service Controls
* The organisation policy `iam.allowedPolicyMemberDomains`
- An IAM condition
- Cloud Armor
? Domain-restricted sharing.

@@quiz
Which service stops data in BigQuery and Cloud Storage being copied to projects outside a trusted boundary?
* VPC Service Controls
- Cloud NAT
- Firewall rules
- IAM Recommender
? Service perimeters protect Google APIs.

@@quiz
Which role lets a developer deploy a Cloud Run service that runs as a given service account?
- roles/iam.serviceAccountKeyAdmin
* roles/iam.serviceAccountUser on that service account
- roles/viewer
- roles/iam.securityReviewer
? Acting as a service account needs this permission.

@@flashcards
IAM inheritance :: union of allow policies downward
basic roles :: Owner, Editor, Viewer (avoid)
predefined roles :: service-specific
custom roles :: your permission list
deny policy :: overrides allows
IAM Conditions :: CEL on bindings
SA keys :: avoid, long-lived secrets
impersonation :: serviceAccountTokenCreator
Workload Identity Federation :: external workloads, no keys
serviceAccountUser :: act as a service account
org policy :: constrain configuration for everyone
gcp.resourceLocations :: allowed regions
disableServiceAccountKeyCreation :: no SA keys
allowedPolicyMemberDomains :: domain-restricted sharing
Admin Activity logs :: always on, 400 days
Data Access logs :: off by default
log sink :: route logs to GCS, BigQuery, Pub/Sub
Security Command Center :: posture and threats
VPC Service Controls :: perimeters against exfiltration
IAP :: context-aware app and SSH access
Binary Authorization :: only trusted images

@@drill
Which basic role should you avoid granting because it allows modifying almost everything?
= Editor
= roles/editor
? Owner can also manage IAM.

@@drill
Which audit log type is always enabled and records configuration changes?
= Admin Activity
= admin activity
? Retained 400 days.

@@exercise Effective IAM with inheritance and deny policies
Compute whether each principal has a permission on a resource. Walk from the resource up to the organisation collecting allow bindings (role → principals, roles expand to permissions) and deny rules (permission, denied principals, exception principals). A deny on any ancestor overrides allows unless the principal is an exception. Group membership counts (`groups`). Print `ALLOW via role at node`, `DENY by policy at node` or `no grant`.
@@lang python
@@starter
parents = {"bucket:finance-reports": "project:fin-prod", "project:fin-prod": "folder:finance", "project:web-prod": "folder:engineering",
           "folder:finance": "org:example.com", "folder:engineering": "org:example.com"}
roles = {
    "roles/viewer": {"storage.objects.list", "compute.instances.list"},
    "roles/storage.objectViewer": {"storage.objects.get", "storage.objects.list"},
    "roles/storage.admin": {"storage.objects.get", "storage.objects.list", "storage.objects.delete", "storage.buckets.delete"},
}
allow = {
    "org:example.com": [("roles/viewer", {"group:all-staff"})],
    "folder:finance": [("roles/storage.objectViewer", {"group:finance"})],
    "project:fin-prod": [("roles/storage.admin", {"user:ops-bot@", "group:platform"})],
}
deny = {"folder:finance": [("storage.objects.delete", {"group:all-staff"}, {"group:records-admins"})]}
groups = {"user:aiko": {"group:all-staff", "group:finance"}, "user:ben": {"group:all-staff", "group:platform"},
          "user:chen": {"group:all-staff", "group:platform", "group:records-admins"}, "user:dana": {"group:all-staff"}}
checks = [("user:aiko", "storage.objects.get", "bucket:finance-reports"), ("user:dana", "storage.objects.get", "bucket:finance-reports"),
          ("user:dana", "compute.instances.list", "project:web-prod"), ("user:ben", "storage.objects.delete", "bucket:finance-reports"),
          ("user:chen", "storage.objects.delete", "bucket:finance-reports")]

def ancestors(node):
    raise NotImplementedError
@@expected
user:aiko storage.objects.get on bucket:finance-reports: ALLOW via roles/storage.objectViewer at folder:finance
user:dana storage.objects.get on bucket:finance-reports: no grant
user:dana compute.instances.list on project:web-prod: ALLOW via roles/viewer at org:example.com
user:ben storage.objects.delete on bucket:finance-reports: DENY by policy at folder:finance
user:chen storage.objects.delete on bucket:finance-reports: ALLOW via roles/storage.admin at project:fin-prod
@@requires
def ancestors
def check
@@hint
`ancestors` returns `[node, parent, grandparent, ...]` using `parents.get`. A principal's identities are `{principal} | groups.get(principal, set())`. Check all deny rules on all ancestors before any allow.
@@solution
parents = {"bucket:finance-reports": "project:fin-prod", "project:fin-prod": "folder:finance", "project:web-prod": "folder:engineering",
           "folder:finance": "org:example.com", "folder:engineering": "org:example.com"}
roles = {
    "roles/viewer": {"storage.objects.list", "compute.instances.list"},
    "roles/storage.objectViewer": {"storage.objects.get", "storage.objects.list"},
    "roles/storage.admin": {"storage.objects.get", "storage.objects.list", "storage.objects.delete", "storage.buckets.delete"},
}
allow = {
    "org:example.com": [("roles/viewer", {"group:all-staff"})],
    "folder:finance": [("roles/storage.objectViewer", {"group:finance"})],
    "project:fin-prod": [("roles/storage.admin", {"user:ops-bot@", "group:platform"})],
}
deny = {"folder:finance": [("storage.objects.delete", {"group:all-staff"}, {"group:records-admins"})]}
groups = {"user:aiko": {"group:all-staff", "group:finance"}, "user:ben": {"group:all-staff", "group:platform"},
          "user:chen": {"group:all-staff", "group:platform", "group:records-admins"}, "user:dana": {"group:all-staff"}}
checks = [("user:aiko", "storage.objects.get", "bucket:finance-reports"), ("user:dana", "storage.objects.get", "bucket:finance-reports"),
          ("user:dana", "compute.instances.list", "project:web-prod"), ("user:ben", "storage.objects.delete", "bucket:finance-reports"),
          ("user:chen", "storage.objects.delete", "bucket:finance-reports")]

def ancestors(node):
    chain = []
    while node:
        chain.append(node)
        node = parents.get(node)
    return chain

def check(principal, permission, resource):
    ids = {principal} | groups.get(principal, set())
    chain = ancestors(resource)
    for node in chain:
        for perm, denied, exceptions in deny.get(node, []):
            if perm == permission and ids & denied and not ids & exceptions:
                return f"DENY by policy at {node}"
    for node in chain:
        for role, members in allow.get(node, []):
            if permission in roles[role] and ids & members:
                return f"ALLOW via {role} at {node}"
    return "no grant"

for principal, permission, resource in checks:
    print(f"{principal} {permission} on {resource}: {check(principal, permission, resource)}")

@@exercise Check resources against organisation policies
Evaluate proposed resources against org policy constraints inherited down the hierarchy, where a lower node's policy `replace`s the parent's or `merge`s its allowed values with it. Constraints: allowed locations (list), `vmExternalIp` (boolean deny), `publicAccessPrevention` (boolean enforce). Print each resource's violations or `compliant`.
@@lang python
@@starter
parents = {"project:web-prod": "folder:prod", "project:lab-1": "folder:lab", "folder:prod": "org", "folder:lab": "org"}
policies = {
    "org": {"locations": ("replace", {"europe-west1", "europe-west4"}), "vmExternalIp": True, "publicAccessPrevention": True},
    "folder:lab": {"locations": ("merge", {"us-central1"}), "vmExternalIp": False},
    "project:web-prod": {"locations": ("replace", {"europe-west1"})},
}
resources = [
    {"name": "vm web-1", "project": "project:web-prod", "location": "europe-west1", "external_ip": True},
    {"name": "bucket assets", "project": "project:web-prod", "location": "europe-west4", "public": True},
    {"name": "vm test-1", "project": "project:lab-1", "location": "us-central1", "external_ip": True},
    {"name": "bucket lab-data", "project": "project:lab-1", "location": "asia-northeast1", "public": False},
]

def effective(project):
    raise NotImplementedError
@@expected
vm web-1: external IP denied
bucket assets: location europe-west4 not in ['europe-west1']; public access prevented
vm test-1: compliant
bucket lab-data: location asia-northeast1 not in ['europe-west1', 'europe-west4', 'us-central1']
@@requires
def effective
"merge"
.get(
@@hint
Walk from the org down to the project (reverse of the ancestor chain). For locations, `replace` sets the allowed set, `merge` unions it. Boolean constraints: the nearest node that sets a value wins.
@@solution
parents = {"project:web-prod": "folder:prod", "project:lab-1": "folder:lab", "folder:prod": "org", "folder:lab": "org"}
policies = {
    "org": {"locations": ("replace", {"europe-west1", "europe-west4"}), "vmExternalIp": True, "publicAccessPrevention": True},
    "folder:lab": {"locations": ("merge", {"us-central1"}), "vmExternalIp": False},
    "project:web-prod": {"locations": ("replace", {"europe-west1"})},
}
resources = [
    {"name": "vm web-1", "project": "project:web-prod", "location": "europe-west1", "external_ip": True},
    {"name": "bucket assets", "project": "project:web-prod", "location": "europe-west4", "public": True},
    {"name": "vm test-1", "project": "project:lab-1", "location": "us-central1", "external_ip": True},
    {"name": "bucket lab-data", "project": "project:lab-1", "location": "asia-northeast1", "public": False},
]

def effective(project):
    chain = []
    node = project
    while node:
        chain.append(node)
        node = parents.get(node)
    result = {"locations": None, "vmExternalIp": False, "publicAccessPrevention": False}
    for node in reversed(chain):
        p = policies.get(node, {})
        if "locations" in p:
            mode, values = p["locations"]
            if mode == "merge" and result["locations"] is not None:
                result["locations"] = result["locations"] | values
            else:
                result["locations"] = set(values)
        for key in ("vmExternalIp", "publicAccessPrevention"):
            if key in p:
                result[key] = p[key]
    return result

for r in resources:
    pol = effective(r["project"])
    problems = []
    if pol["locations"] is not None and r["location"] not in pol["locations"]:
        problems.append(f"location {r['location']} not in {sorted(pol['locations'])}")
    if r.get("external_ip") and pol["vmExternalIp"]:
        problems.append("external IP denied")
    if r.get("public") and pol["publicAccessPrevention"]:
        problems.append("public access prevented")
    print(f"{r['name']}: " + ("; ".join(problems) if problems else "compliant"))

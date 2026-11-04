@@lesson az1-policy-cost Governance in Depth: Management Groups, Policy, Locks, Tags and Cost
@@text
Governance is how an organisation keeps hundreds of subscriptions consistent and affordable. AZ-900 asks what each tool does; AZ-104 asks you to configure them and predict their effect. This lesson covers the hierarchy, Azure Policy effects, resource locks, tagging and the cost tools.

## The hierarchy

**Tenant root management group → management groups (up to 6 levels deep) → subscriptions → resource groups → resources.**

- A resource group is a **lifecycle and permission boundary**: delete it and everything in it goes. Resources can be **moved** between groups and subscriptions (most types).
- A resource group has a location (for its metadata), but its resources can be in other regions.
- **Subscriptions** are billing and quota boundaries; split them by environment or business unit (Cloud Adoption Framework landing zones).

## Azure Policy

A **policy definition** describes a rule and its **effect**; an **initiative** (policy set) groups definitions (e.g. "ISO 27001"); an **assignment** applies it at a scope (with **exclusions** and parameters). Policies evaluate on create/update and in periodic **compliance scans**.

| Effect | What it does |
|---|---|
| **Deny** | blocks non-compliant create/update requests |
| **Audit** | allows, but marks the resource non-compliant |
| **Append** | adds fields (e.g. an IP rule) during the request |
| **Modify** | adds/replaces tags or properties (needs a managed identity for **remediation tasks** on existing resources) |
| **AuditIfNotExists** | audits if a related resource is missing (e.g. no diagnostic settings) |
| **DeployIfNotExists** | deploys the missing related resource (e.g. the monitoring agent) |
| **DenyAction** | blocks specific actions such as delete |
| **Disabled** | turns the assignment's rule off |

Evaluation order: Disabled → Append/Modify → Deny → Audit. Common built-ins: **Allowed locations**, **Allowed virtual machine size SKUs**, **Require a tag and its value**, **Inherit a tag from the resource group if missing**, storage accounts should disable public access.

**Policy vs RBAC**: RBAC decides **who** may act; Policy decides **what** resources may look like, regardless of who creates them.

## Resource locks

| Lock | Blocks |
|---|---|
| **CanNotDelete** (Delete) | deletion; reads and updates still work |
| **ReadOnly** | any **write or delete**; also some POST operations (e.g. listing storage account keys) |

Locks apply to **everyone, including Owners**, and **inherit** to child resources; the most restrictive lock wins. To delete a locked resource, remove the lock first (needs `Microsoft.Authorization/locks/*`, held by Owner and User Access Administrator).

## Tags

Name/value pairs on resources, resource groups and subscriptions. **Tags don't inherit by default**: use the Policy "inherit a tag from the resource group". Limits: 50 tags per resource. Use them for cost centre, environment, owner, application.

## Cost management

- **Pricing calculator** (estimate) and **TCO calculator** (compare with on-premises).
- **Cost Management + Billing**: cost analysis (by tag, resource group, service), **budgets** with alerts to **action groups** (which can run automation), exports, Advisor cost recommendations.
- Saving options:
  - **Reservations** (1 or 3 years, specific VM family/region, up to ~72%)
  - **Azure savings plan for compute** (hourly commitment, flexible)
  - **Azure Hybrid Benefit** (reuse Windows Server/SQL Server licences with Software Assurance)
  - **Spot VMs** (evictable, deep discounts)
  - **Dev/Test pricing**
  - deallocate VMs (a stopped but not deallocated VM still bills compute)
  - autoscale; right-size with **Azure Advisor**
- Factors affecting cost: resource type and SKU, consumption, region, **bandwidth** (inbound free, outbound charged, inter-region charged), Marketplace items.

@@quiz
Company policy says resources may only be created in West Europe and North Europe. Which tool enforces it?
- An RBAC custom role
* Azure Policy with the Allowed locations definition (Deny)
- A ReadOnly lock
- A budget
? Policy controls what resources look like.

@@quiz
A resource group has a CanNotDelete lock. What can an Owner do with a VM inside it?
- Nothing
* Start, stop and resize it, but not delete it
- Delete it, because Owners bypass locks
- Only read it
? Locks apply to everyone and inherit.

@@quiz
Which policy effect can add a missing tag to existing resources through a remediation task?
- Audit
- Deny
* Modify
- Disabled
? Remediation needs a managed identity.

@@quiz
A storage account has a ReadOnly lock and an admin can't list its access keys. Why?
* Listing keys is a POST operation that ReadOnly blocks
- RBAC denies it
- The account is in the Archive tier
- Keys can't be listed in the portal
? ReadOnly blocks more than people expect.

@@quiz
Which option lets you reuse on-premises Windows Server licences with Software Assurance on Azure VMs?
- Reservations
- Spot VMs
* Azure Hybrid Benefit
- Dev/Test pricing
? It removes the Windows licence cost from the VM price.

@@flashcards
hierarchy :: root MG, MGs, subscriptions, RGs, resources
management group depth :: up to 6 levels
resource group :: lifecycle and permission boundary
initiative :: group of policy definitions
policy assignment :: definition + scope + parameters
Deny :: blocks non-compliant requests
Audit :: flags, allows
Modify :: changes tags/properties, remediation
DeployIfNotExists :: deploys missing related resource
policy vs RBAC :: what vs who
CanNotDelete :: no delete, updates OK
ReadOnly :: no writes or deletes
locks apply to :: everyone including Owners
tag inheritance :: not automatic, use Policy
TCO calculator :: compare with on-premises
budgets :: alerts via action groups
reservations :: 1/3-year discount
Hybrid Benefit :: reuse licences with SA
stopped vs deallocated :: only deallocated stops compute billing

@@drill
Which lock type still allows updates but blocks deletion?
= CanNotDelete
= Delete
? ReadOnly blocks updates too.

@@drill
What is a group of Azure Policy definitions called?
= initiative
= an initiative
= policy set
? Assigned like a single policy.

@@exercise Evaluate Azure Policy assignments
Evaluate each new resource request against the assignments (inherited by scope prefix, skipping excluded scopes). Apply effects in order: Modify (add tag `env` from the resource group if missing), Deny (allowed locations and allowed VM sizes), then Audit (require tag `costCenter`). Print `DENIED by <policy>` or `created` with any audit findings and modifications.
@@lang python
@@starter
assignments = [
    {"name": "allowed-locations", "scope": "/mg-corp", "effect": "deny", "allowed": {"westeurope", "northeurope"}, "exclude": ["/mg-corp/sub-lab"]},
    {"name": "allowed-vm-sizes", "scope": "/mg-corp/sub-prod", "effect": "deny", "allowed": {"Standard_D2s_v5", "Standard_D4s_v5"}},
    {"name": "inherit-env-tag", "scope": "/mg-corp", "effect": "modify"},
    {"name": "require-costcenter", "scope": "/mg-corp", "effect": "audit"},
]
rg_tags = {"/mg-corp/sub-prod/rg-web": {"env": "prod"}, "/mg-corp/sub-lab/rg-test": {"env": "lab"}}
requests = [
    {"name": "vm-web1", "rg": "/mg-corp/sub-prod/rg-web", "location": "westeurope", "type": "vm", "size": "Standard_D2s_v5", "tags": {"costCenter": "4410"}},
    {"name": "vm-web2", "rg": "/mg-corp/sub-prod/rg-web", "location": "westeurope", "type": "vm", "size": "Standard_E64s_v5", "tags": {}},
    {"name": "st-logs", "rg": "/mg-corp/sub-prod/rg-web", "location": "eastus", "type": "storage", "tags": {}},
    {"name": "vm-test", "rg": "/mg-corp/sub-lab/rg-test", "location": "eastus", "type": "vm", "size": "Standard_B1s", "tags": {}},
]
@@expected
vm-web1: created (modified: env=prod)
vm-web2: DENIED by allowed-vm-sizes
st-logs: DENIED by allowed-locations
vm-test: created (modified: env=lab; non-compliant: require-costcenter)
@@requires
def applies
startswith(
@@hint
`applies(a, rg)`: `rg.startswith(a["scope"])` and not under any excluded scope. Copy the tags dict before modifying it.
@@solution
assignments = [
    {"name": "allowed-locations", "scope": "/mg-corp", "effect": "deny", "allowed": {"westeurope", "northeurope"}, "exclude": ["/mg-corp/sub-lab"]},
    {"name": "allowed-vm-sizes", "scope": "/mg-corp/sub-prod", "effect": "deny", "allowed": {"Standard_D2s_v5", "Standard_D4s_v5"}},
    {"name": "inherit-env-tag", "scope": "/mg-corp", "effect": "modify"},
    {"name": "require-costcenter", "scope": "/mg-corp", "effect": "audit"},
]
rg_tags = {"/mg-corp/sub-prod/rg-web": {"env": "prod"}, "/mg-corp/sub-lab/rg-test": {"env": "lab"}}
requests = [
    {"name": "vm-web1", "rg": "/mg-corp/sub-prod/rg-web", "location": "westeurope", "type": "vm", "size": "Standard_D2s_v5", "tags": {"costCenter": "4410"}},
    {"name": "vm-web2", "rg": "/mg-corp/sub-prod/rg-web", "location": "westeurope", "type": "vm", "size": "Standard_E64s_v5", "tags": {}},
    {"name": "st-logs", "rg": "/mg-corp/sub-prod/rg-web", "location": "eastus", "type": "storage", "tags": {}},
    {"name": "vm-test", "rg": "/mg-corp/sub-lab/rg-test", "location": "eastus", "type": "vm", "size": "Standard_B1s", "tags": {}},
]

def applies(a, rg):
    if not rg.startswith(a["scope"]):
        return False
    return not any(rg.startswith(ex) for ex in a.get("exclude", []))

for req in requests:
    active = [a for a in assignments if applies(a, req["rg"])]
    tags = dict(req["tags"])
    notes = []
    for a in active:
        if a["effect"] == "modify" and "env" not in tags and "env" in rg_tags.get(req["rg"], {}):
            tags["env"] = rg_tags[req["rg"]]["env"]
            notes.append(f"modified: env={tags['env']}")
    denied = None
    for a in active:
        if a["effect"] != "deny":
            continue
        if a["name"] == "allowed-locations" and req["location"] not in a["allowed"]:
            denied = a["name"]
        elif a["name"] == "allowed-vm-sizes" and req["type"] == "vm" and req["size"] not in a["allowed"]:
            denied = a["name"]
        if denied:
            break
    if denied:
        print(f"{req['name']}: DENIED by {denied}")
        continue
    for a in active:
        if a["effect"] == "audit" and "costCenter" not in tags:
            notes.append(f"non-compliant: {a['name']}")
    print(f"{req['name']}: created" + (" (" + "; ".join(notes) + ")" if notes else ""))

@@exercise Predict the effect of resource locks
Locks can be on a subscription, resource group or resource; they inherit downward and the most restrictive applies (`ReadOnly` > `CanNotDelete`). For each operation (`read`, `write`, `delete`, `listKeys`), decide whether it succeeds. `listKeys` counts as a write for ReadOnly. Print the effective lock and the result for each operation.
@@lang python
@@starter
locks = {"/sub-prod/rg-data": "CanNotDelete", "/sub-prod/rg-data/st-archive": "ReadOnly", "/sub-prod/rg-net/vnet-hub": "CanNotDelete"}
operations = [
    ("/sub-prod/rg-data/st-main", "write"),
    ("/sub-prod/rg-data/st-main", "delete"),
    ("/sub-prod/rg-data/st-archive", "read"),
    ("/sub-prod/rg-data/st-archive", "listKeys"),
    ("/sub-prod/rg-net/vnet-hub", "write"),
    ("/sub-prod/rg-net/vnet-spoke", "delete"),
    ("/sub-prod/rg-data", "delete"),
]
STRENGTH = {None: 0, "CanNotDelete": 1, "ReadOnly": 2}
@@expected
write    /sub-prod/rg-data/st-main      lock=CanNotDelete             succeeds
delete   /sub-prod/rg-data/st-main      lock=CanNotDelete             blocked
read     /sub-prod/rg-data/st-archive   lock=ReadOnly                 succeeds
listKeys /sub-prod/rg-data/st-archive   lock=ReadOnly                 blocked
write    /sub-prod/rg-net/vnet-hub      lock=CanNotDelete             succeeds
delete   /sub-prod/rg-net/vnet-spoke    lock=none                     succeeds
delete   /sub-prod/rg-data              lock=CanNotDelete             blocked
@@requires
def effective_lock
STRENGTH
@@hint
Check every lock whose path equals the target or is a parent of it (`target.startswith(path + "/")`) and keep the strongest. Deleting a container such as a resource group also fails if any lock exists on a resource inside it.
@@solution
locks = {"/sub-prod/rg-data": "CanNotDelete", "/sub-prod/rg-data/st-archive": "ReadOnly", "/sub-prod/rg-net/vnet-hub": "CanNotDelete"}
operations = [
    ("/sub-prod/rg-data/st-main", "write"),
    ("/sub-prod/rg-data/st-main", "delete"),
    ("/sub-prod/rg-data/st-archive", "read"),
    ("/sub-prod/rg-data/st-archive", "listKeys"),
    ("/sub-prod/rg-net/vnet-hub", "write"),
    ("/sub-prod/rg-net/vnet-spoke", "delete"),
    ("/sub-prod/rg-data", "delete"),
]
STRENGTH = {None: 0, "CanNotDelete": 1, "ReadOnly": 2}

def effective_lock(target):
    best = None
    for path, lock in locks.items():
        if target == path or target.startswith(path + "/"):
            if STRENGTH[lock] > STRENGTH[best]:
                best = lock
    return best

for target, op in operations:
    lock = effective_lock(target)
    if op == "delete" and any(p.startswith(target + "/") for p in locks):
        lock = lock or "lock on a child resource"
        ok = False
    elif op == "read":
        ok = True
    elif lock == "ReadOnly":
        ok = False
    elif lock == "CanNotDelete":
        ok = op != "delete"
    else:
        ok = True
    print(f"{op:<8} {target:<30} lock={lock or 'none':<24} {'succeeds' if ok else 'blocked'}")

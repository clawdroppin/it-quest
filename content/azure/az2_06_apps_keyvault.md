@@lesson az2-apps-keyvault App Hosting and Secrets: App Service, Functions, Containers, Managed Identities and Key Vault
@@concepts numbers, conditions, maps
@@lang python
@@text
VMs are only one way to run code on Azure. Most new applications run on **platform services** that handle the OS, patching and scaling: App Service for web apps, Azure Functions for event-driven code, Container Apps and AKS for containers. They all need secrets (connection strings, API keys, certificates), and the right answer on AZ-104 and AZ-204, and in real projects, is almost always **managed identity plus Key Vault**. This lesson covers the hosting options, how they scale and cost, and how to handle secrets properly.

## App Service

- An **App Service plan** is the set of VMs your apps run on. Its **tier** sets the features and price:
  - **Free/Shared**: no SLA, shared infrastructure.
  - **Basic**: dedicated VMs, manual scale up to 3 instances.
  - **Standard**: autoscale up to 10 instances, **deployment slots** (5), daily backups.
  - **Premium v3**: more instances (up to 30), more slots (20), zone redundancy.
  - **Isolated v2** (App Service Environment): a single-tenant deployment inside your VNet.
- Several apps can share one plan, and they share its CPU and memory.
- **Scale up** changes the tier or size. **Scale out** adds instances, manually or with autoscale rules (e.g. CPU above 70% for 10 minutes → +1 instance).
- **Deployment slots**: deploy to `staging`, warm it up, then **swap** with production. The swap is a switch with no downtime, and swapping back is the rollback. Settings marked **slot setting** (sticky) stay with the slot, e.g. a staging database connection string, and all other settings travel with the code.
- **VNet integration** lets the app reach private resources. **Private endpoints** make the app itself reachable only privately. **Access restrictions** allow-list callers.

## Azure Functions

- Event-driven code: HTTP requests, queue messages, timers, Blob events, Event Grid. **Triggers** start a function, and **bindings** connect inputs and outputs without boilerplate.
- Hosting plans:
  - **Consumption**: scales to zero, and you pay per execution and GB-second.
  - **Flex Consumption**: the newer serverless plan, with VNet support and always-ready instances.
  - **Premium**: pre-warmed instances so there are no cold starts, plus VNet access.
  - **Dedicated**: runs on an App Service plan.
- Consumption pricing (the classic model): about **$0.20 per million executions** and **$0.000016 per GB-second**, with a monthly free grant of **1 million executions and 400,000 GB-s**. Memory is rounded **up to the next 128 MB**, and each execution counts at least **100 ms**.
- **Durable Functions** add stateful workflows (function chaining, fan-out/fan-in, human approval with timeouts).

## Containers

- **Azure Container Instances (ACI)**: run a container quickly, without orchestration. It suits batch jobs and simple tasks.
- **Azure Container Apps**: serverless containers built on Kubernetes (KEDA autoscaling, Dapr, revisions with traffic splitting). There's no cluster to manage, and it's a good default for microservices.
- **AKS**: managed Kubernetes. You get full control, and you also do the upgrades, node pools and networking yourself.
- **Azure Container Registry (ACR)** stores images. Grant pull access to the app's managed identity with the **AcrPull** role instead of using admin credentials.

## Managed identities and Key Vault

- A **managed identity** is an identity in Entra ID for an Azure resource, with credentials Azure rotates automatically.
  - A **system-assigned** identity lives and dies with the resource.
  - A **user-assigned** identity is a separate resource that several apps can share, and it survives redeployments.
  - Grant it RBAC roles like any user. There are no secrets in code at all.
- **Key Vault** stores **secrets**, **keys** (encryption keys, optionally HSM-backed in the Premium tier or Managed HSM) and **certificates** (with auto-renewal).
- Permission models:
  - **Azure RBAC** is recommended: roles like *Key Vault Secrets User* (read secret values) and *Key Vault Secrets Officer* (manage), at vault or individual secret scope.
  - The legacy **access policies** model set per-vault permissions.
- **Soft delete** (always on now) keeps deleted vaults and objects for **7–90 days** (90 by default) so they can be recovered. **Purge protection** stops anyone, even admins, from permanently deleting them during that period. Turn it on for production vaults.
- Apps reference secrets without code: App Service and Functions support **Key Vault references** in app settings (`@Microsoft.KeyVault(SecretUri=...)`), resolved with the app's managed identity.
- Lock down the vault's network (private endpoint or firewall), and send its logs to Log Analytics.

@@quiz
Which App Service tier is the lowest that supports deployment slots and autoscale?
- Basic
* Standard
- Free
- Shared
? Premium adds more slots and instances.

@@quiz
A staging slot needs its own database connection string that must not move to production during a swap. What should you do?
* Mark the setting as a deployment slot setting (sticky)
- Use a separate App Service plan
- Put the string in code
- Disable swapping
? Sticky settings stay with the slot.

@@quiz
Which Functions plan avoids cold starts with pre-warmed instances?
* Premium
- Consumption
- Free
- Shared
? Flex Consumption can also keep always-ready instances.

@@quiz
What is the recommended way for a web app to read a database password from Key Vault?
* Give the app a managed identity and the Key Vault Secrets User role, and use a Key Vault reference
- Store the vault's admin password in app settings
- Copy the secret into the code
- Make the vault public
? No secrets in code or config.

@@quiz
A secret was deleted by mistake yesterday. What makes recovery possible?
* Soft delete, which keeps it for the retention period
- Purge protection alone
- Resource locks
- Azure Backup of the app
? Purge protection prevents permanent deletion during that period.

@@quiz
Which service runs containers serverlessly with KEDA-based autoscaling and revisions, without managing a cluster?
* Azure Container Apps
- AKS
- Azure Batch
- Virtual Machine Scale Sets
? ACI suits single, simple containers.

@@quiz
How should an App Service app pull images from ACR?
* Use its managed identity with the AcrPull role
- Enable the ACR admin user and paste the password
- Make the registry public
- Copy images to Blob storage
? No shared credentials.

@@flashcards
App Service plan :: VMs your apps run on
Basic / Standard / Premium v3 :: manual 3 / autoscale 10 + 5 slots / 30 + 20 slots
deployment slot :: deploy, warm up, swap
slot setting :: sticky, stays with the slot
scale up / scale out :: bigger tier / more instances
Functions Consumption :: scale to zero, pay per execution and GB-s
free grant :: 1M executions, 400,000 GB-s per month
memory rounding :: up to the next 128 MB, minimum 100 ms
Premium plan :: pre-warmed, no cold starts
Durable Functions :: stateful workflows
ACI :: single containers quickly
Container Apps :: serverless containers, KEDA, revisions
AKS :: managed Kubernetes
AcrPull :: role to pull images
system-assigned identity :: tied to the resource's life
user-assigned identity :: separate, shareable
Key Vault Secrets User :: read secret values
soft delete :: 7–90 days recovery
purge protection :: no permanent delete during retention
Key Vault reference :: @Microsoft.KeyVault(SecretUri=...)

@@drill
A function uses 200 MB of memory. What memory size is it billed at in MB?
= 256
? Rounded up to the next 128 MB.

@@drill
What is the default soft-delete retention for Key Vault, in days?
= 90
? Configurable from 7 to 90.

@@exercise Choose an App Service tier and simulate a slot swap
Pick the cheapest tier that meets each app's requirements (instances, slots, autoscale, zone redundancy), then simulate a swap between staging and production: app settings move with the code unless they are marked as slot settings, which stay where they are.
@@starter
TIERS = [
    ("Basic", {"max_instances": 3, "slots": 0, "autoscale": False, "zones": False}),
    ("Standard", {"max_instances": 10, "slots": 5, "autoscale": True, "zones": False}),
    ("PremiumV3", {"max_instances": 30, "slots": 20, "autoscale": True, "zones": True}),
]
apps = {
    "intranet": {"instances": 2, "slots": 0, "autoscale": False, "zones": False},
    "shop": {"instances": 6, "slots": 2, "autoscale": True, "zones": False},
    "payments": {"instances": 12, "slots": 3, "autoscale": True, "zones": True},
}

def choose_tier(req):
    raise NotImplementedError

def swap(production, staging, sticky):
    raise NotImplementedError

for name, req in apps.items():
    print(f"{name:<9} -> {choose_tier(req)}")

prod = {"version": "1.4.2", "DB": "prod-sql", "FEATURE_X": "off", "APPINSIGHTS": "prod-ai"}
stage = {"version": "1.5.0", "DB": "stage-sql", "FEATURE_X": "on", "APPINSIGHTS": "stage-ai"}
new_prod, new_stage = swap(prod, stage, sticky={"DB", "APPINSIGHTS"})
print("production after swap:", new_prod)
print("staging after swap:   ", new_stage)
@@expected
intranet  -> Basic
shop      -> Standard
payments  -> PremiumV3
production after swap: {'version': '1.5.0', 'DB': 'prod-sql', 'FEATURE_X': 'on', 'APPINSIGHTS': 'prod-ai'}
staging after swap:    {'version': '1.4.2', 'DB': 'stage-sql', 'FEATURE_X': 'off', 'APPINSIGHTS': 'stage-ai'}
@@requires
def choose_tier
def swap
sticky
dict(
@@hint
`choose_tier` returns the first tier where `max_instances >= instances`, `slots >= slots`, and autoscale and zones are available if required. In `swap`, start each side as a copy of the other side's settings, then put back each sticky key from the original side: `new_prod[k] = production[k]` and `new_stage[k] = staging[k]`.
@@solution
TIERS = [
    ("Basic", {"max_instances": 3, "slots": 0, "autoscale": False, "zones": False}),
    ("Standard", {"max_instances": 10, "slots": 5, "autoscale": True, "zones": False}),
    ("PremiumV3", {"max_instances": 30, "slots": 20, "autoscale": True, "zones": True}),
]
apps = {
    "intranet": {"instances": 2, "slots": 0, "autoscale": False, "zones": False},
    "shop": {"instances": 6, "slots": 2, "autoscale": True, "zones": False},
    "payments": {"instances": 12, "slots": 3, "autoscale": True, "zones": True},
}

def choose_tier(req):
    for name, t in TIERS:
        if (t["max_instances"] >= req["instances"] and t["slots"] >= req["slots"]
                and (t["autoscale"] or not req["autoscale"]) and (t["zones"] or not req["zones"])):
            return name
    return "Isolated v2 (App Service Environment)"

def swap(production, staging, sticky):
    new_prod, new_stage = dict(staging), dict(production)
    for k in sticky:
        new_prod[k] = production[k]
        new_stage[k] = staging[k]
    return new_prod, new_stage

for name, req in apps.items():
    print(f"{name:<9} -> {choose_tier(req)}")

prod = {"version": "1.4.2", "DB": "prod-sql", "FEATURE_X": "off", "APPINSIGHTS": "prod-ai"}
stage = {"version": "1.5.0", "DB": "stage-sql", "FEATURE_X": "on", "APPINSIGHTS": "stage-ai"}
new_prod, new_stage = swap(prod, stage, sticky={"DB", "APPINSIGHTS"})
print("production after swap:", new_prod)
print("staging after swap:   ", new_stage)

@@exercise Estimate Azure Functions consumption cost
Estimate the monthly cost of functions on the classic Consumption plan: round memory up to the next 128 MB, count at least 100 ms per execution, compute GB-seconds, subtract the free grant (1,000,000 executions and 400,000 GB-s, shared across the subscription's functions), and price executions at $0.20 per million and GB-s at $0.000016.
@@starter
import math

functions = [
    {"name": "resize-image", "executions": 2_500_000, "avg_ms": 850, "memory_mb": 300},
    {"name": "webhook", "executions": 6_000_000, "avg_ms": 40, "memory_mb": 128},
    {"name": "nightly-report", "executions": 30, "avg_ms": 240_000, "memory_mb": 1500},
]
FREE_EXEC, FREE_GBS = 1_000_000, 400_000
PRICE_EXEC, PRICE_GBS = 0.20 / 1_000_000, 0.000016

def gb_seconds(f):
    raise NotImplementedError

def monthly_cost(functions):
    raise NotImplementedError

for f in functions:
    print(f"{f['name']:<15} {f['executions']:>10,} runs  {gb_seconds(f):>12,.0f} GB-s")
execs, gbs, cost = monthly_cost(functions)
print(f"billable: {execs:,} executions, {gbs:,.0f} GB-s -> ${cost:,.2f} a month")
@@expected
resize-image     2,500,000 runs       796,875 GB-s
webhook          6,000,000 runs        75,000 GB-s
nightly-report          30 runs        10,800 GB-s
billable: 7,500,030 executions, 482,675 GB-s -> $9.22 a month
@@requires
def gb_seconds
def monthly_cost
math.ceil(
@@hint
Memory in GB is `math.ceil(memory_mb / 128) * 128 / 1024`; duration is `max(avg_ms, 100) / 1000` seconds; GB-s = executions × duration × memory. Billable amounts are `max(0, total - free)`; cost = billable executions × PRICE_EXEC + billable GB-s × PRICE_GBS.
@@solution
import math

functions = [
    {"name": "resize-image", "executions": 2_500_000, "avg_ms": 850, "memory_mb": 300},
    {"name": "webhook", "executions": 6_000_000, "avg_ms": 40, "memory_mb": 128},
    {"name": "nightly-report", "executions": 30, "avg_ms": 240_000, "memory_mb": 1500},
]
FREE_EXEC, FREE_GBS = 1_000_000, 400_000
PRICE_EXEC, PRICE_GBS = 0.20 / 1_000_000, 0.000016

def gb_seconds(f):
    memory_gb = math.ceil(f["memory_mb"] / 128) * 128 / 1024
    seconds = max(f["avg_ms"], 100) / 1000
    return f["executions"] * seconds * memory_gb

def monthly_cost(functions):
    execs = max(0, sum(f["executions"] for f in functions) - FREE_EXEC)
    gbs = max(0, sum(gb_seconds(f) for f in functions) - FREE_GBS)
    return execs, gbs, execs * PRICE_EXEC + gbs * PRICE_GBS

for f in functions:
    print(f"{f['name']:<15} {f['executions']:>10,} runs  {gb_seconds(f):>12,.0f} GB-s")
execs, gbs, cost = monthly_cost(functions)
print(f"billable: {execs:,} executions, {gbs:,.0f} GB-s -> ${cost:,.2f} a month")

@@exercise Key Vault access and recovery
Decide whether each identity can perform each operation on a vault that uses Azure RBAC (roles grant actions at the vault scope or on a single secret), and work out whether deleted secrets can still be recovered or purged, given the soft-delete retention and purge protection.
@@starter
from datetime import date, timedelta

ROLES = {
    "Key Vault Secrets User": {"get"},
    "Key Vault Secrets Officer": {"get", "set", "delete", "recover", "purge"},
    "Key Vault Reader": {"list-metadata"},
}
assignments = [
    ("app-shop (managed identity)", "Key Vault Secrets User", "secret:db-password"),
    ("ops-team", "Key Vault Secrets Officer", "vault"),
    ("auditor", "Key Vault Reader", "vault"),
]
requests = [
    ("app-shop (managed identity)", "get", "db-password"),
    ("app-shop (managed identity)", "get", "stripe-key"),
    ("auditor", "get", "db-password"),
    ("ops-team", "set", "stripe-key"),
]
vault = {"retention_days": 90, "purge_protection": True}
deleted = {"old-api-key": date(2026, 2, 1), "test-secret": date(2026, 5, 20)}
today = date(2026, 6, 1)

def allowed(identity, action, secret):
    raise NotImplementedError

def deleted_status(deleted_on):
    raise NotImplementedError

for who, action, secret in requests:
    print(f"{who:<28} {action:<4} {secret:<12} {'allowed' if allowed(who, action, secret) else 'DENIED'}")
for name, when in deleted.items():
    print(f"{name}: {deleted_status(when)}")
@@expected
app-shop (managed identity)  get  db-password  allowed
app-shop (managed identity)  get  stripe-key   DENIED
auditor                      get  db-password  DENIED
ops-team                     set  stripe-key   allowed
old-api-key: permanently purged on 2026-05-02
test-secret: recoverable until 2026-08-17; cannot be purged early (purge protection)
@@requires
def allowed
def deleted_status
timedelta(days=
@@hint
An assignment applies if its scope is "vault" or `f"secret:{secret}"`, and the role's actions include the action. For a deleted secret, `purge_date = deleted_on + timedelta(days=retention_days)`; before that date it's recoverable (and, with purge protection, can't be purged early); after it, it's gone for good.
@@solution
from datetime import date, timedelta

ROLES = {
    "Key Vault Secrets User": {"get"},
    "Key Vault Secrets Officer": {"get", "set", "delete", "recover", "purge"},
    "Key Vault Reader": {"list-metadata"},
}
assignments = [
    ("app-shop (managed identity)", "Key Vault Secrets User", "secret:db-password"),
    ("ops-team", "Key Vault Secrets Officer", "vault"),
    ("auditor", "Key Vault Reader", "vault"),
]
requests = [
    ("app-shop (managed identity)", "get", "db-password"),
    ("app-shop (managed identity)", "get", "stripe-key"),
    ("auditor", "get", "db-password"),
    ("ops-team", "set", "stripe-key"),
]
vault = {"retention_days": 90, "purge_protection": True}
deleted = {"old-api-key": date(2026, 2, 1), "test-secret": date(2026, 5, 20)}
today = date(2026, 6, 1)

def allowed(identity, action, secret):
    for who, role, scope in assignments:
        if who == identity and scope in ("vault", f"secret:{secret}") and action in ROLES[role]:
            return True
    return False

def deleted_status(deleted_on):
    purge_date = deleted_on + timedelta(days=vault["retention_days"])
    if today >= purge_date:
        return f"permanently purged on {purge_date}"
    note = "cannot be purged early (purge protection)" if vault["purge_protection"] else "an officer could purge it now"
    return f"recoverable until {purge_date - timedelta(days=1)}; {note}"

for who, action, secret in requests:
    print(f"{who:<28} {action:<4} {secret:<12} {'allowed' if allowed(who, action, secret) else 'DENIED'}")
for name, when in deleted.items():
    print(f"{name}: {deleted_status(when)}")

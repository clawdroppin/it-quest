@@lesson gc0-compute-deep Compute in Depth: Compute Engine, MIGs, GKE, Cloud Run and Pricing
@@text
Choosing and sizing compute is a large part of both the Associate Cloud Engineer and Professional Cloud Architect exams. This lesson covers Compute Engine machine families and discounts, managed instance groups and autoscaling, and how GKE, Cloud Run, Cloud Run functions and App Engine compare.

## Compute Engine machine families

| Family | Series | Use |
|---|---|---|
| **General purpose** | **E2** (cost-optimised, shared-core e2-micro/small/medium), **N2/N2D/N4**, **C4**, **Tau T2D/T2A** (scale-out, Arm) | web servers, apps, dev/test |
| **Compute optimised** | **C2/C2D/H3** | HPC, gaming, high per-core performance |
| **Memory optimised** | **M1/M2/M3** | SAP HANA, large in-memory databases |
| **Accelerator optimised** | **A2/A3** (NVIDIA GPUs), **G2** | ML training/inference, rendering |
| **Storage optimised** | **Z3** | high local SSD throughput |

Machine types: predefined (`n2-standard-4` = 4 vCPU, 16 GB), **custom machine types** (choose vCPU and memory), shared-core. **Live migration** moves VMs during host maintenance without reboot (not for GPU VMs).

Disks: **Persistent Disk** (pd-standard, pd-balanced, pd-ssd), **Hyperdisk** (tune IOPS/throughput separately), **Local SSD** (ephemeral, highest performance). **Regional persistent disks** replicate synchronously across two zones. Snapshots are global; **images** and **machine images** capture whole VMs.

## Discounts

| Option | Discount | Notes |
|---|---|---|
| **Sustained use discounts** | automatic, up to about 30% | for running most of the month on N1/N2/N2D/C2 etc.; not E2 or A2 |
| **Committed use discounts** (resource-based) | about 37% (1 year), 55% (3 years), more for memory-optimised | commit to vCPU/memory in a region |
| **Flexible (spend-based) CUDs** | about 28% / 46% | commit to hourly spend across families |
| **Spot VMs** | 60–91% | can be **preempted** with 30 seconds' notice; replace preemptible VMs (24 h max) |

## Instance templates and managed instance groups (MIGs)

- **Instance template**: immutable definition (machine type, image, disks, metadata/startup script, service account, network tags).
- **MIG**: identical VMs from a template, **zonal or regional** (spread across zones in a region: recommended).
  - **Autoscaling** on CPU, load balancer serving capacity, Cloud Monitoring metrics or schedules; with **cool-down/initialisation period** and scale-in controls.
  - **Autohealing** with a health check recreates unhealthy VMs.
  - **Rolling updates** (`maxSurge`, `maxUnavailable`) and canary updates with two templates.
- **Unmanaged instance groups** only group existing, different VMs for load balancing.

```bash,norun
gcloud compute instance-templates create web-tpl --machine-type=e2-medium --image-family=debian-12 --image-project=debian-cloud --tags=http-server --metadata-from-file=startup-script=startup.sh
gcloud compute instance-groups managed create web-mig --template=web-tpl --size=2 --region=europe-west1
gcloud compute instance-groups managed set-autoscaling web-mig --region=europe-west1 --min-num-replicas=2 --max-num-replicas=10 --target-cpu-utilization=0.6
```

## Containers and serverless

| Service | What you manage | Good for |
|---|---|---|
| **GKE Standard** | node pools, upgrades (with help), cluster config | full Kubernetes control, special nodes (GPU), DaemonSets |
| **GKE Autopilot** | only workloads; Google manages nodes, pay per pod resources | most Kubernetes workloads with less ops |
| **Cloud Run** | a container image; scales to zero, per-request (or instance-based) billing | stateless HTTP services, jobs, event-driven containers |
| **Cloud Run functions** (formerly Cloud Functions) | a function | small event handlers (Pub/Sub, Cloud Storage, Eventarc) |
| **App Engine** Standard/Flexible | code | legacy PaaS apps; new projects usually choose Cloud Run |
| **Compute Engine** | everything | lift-and-shift, custom OS, licensing |

GKE features: regional clusters (control plane in three zones), node auto-provisioning, **cluster autoscaler** + **Horizontal Pod Autoscaler**, **Workload Identity Federation for GKE** (pods use IAM service accounts without keys), private clusters, release channels.

Cloud Run: concurrency per instance (default 80), min instances (avoid cold starts), max instances, CPU always-allocated vs request-only, ingress settings, **VPC connectors / Direct VPC egress**, revisions with traffic splitting.

@@quiz
A workload runs continuously on N2 VMs for 3 years. Which option gives the biggest predictable discount?
- Sustained use discounts
* 3-year committed use discount
- Spot VMs
- E2 shared-core
? SUDs apply automatically but are smaller.

@@quiz
What happens to a Spot VM when Compute Engine needs the capacity back?
- It is live-migrated
* It is preempted with about 30 seconds' notice
- It keeps running at On-Demand price
- It moves to another region
? Design Spot workloads to be interruptible.

@@quiz
A MIG should keep VMs running across zone failures. Which option?
- Zonal MIG with autohealing
* Regional MIG
- Unmanaged instance group
- One large VM with a regional disk
? Regional MIGs spread instances across zones.

@@quiz
A team wants Kubernetes without managing nodes and pays per pod resource requests. Which option?
- GKE Standard
* GKE Autopilot
- Compute Engine with kubeadm
- App Engine Flexible
? Google manages the nodes.

@@quiz
A stateless HTTP API in a container should scale to zero overnight. Which service is simplest?
* Cloud Run
- GKE Standard
- Compute Engine MIG
- Bare Metal Solution
? Per-request billing and scale to zero.

@@flashcards
E2 :: cost-optimised general purpose
N2/N4 :: balanced general purpose
C2/C2D/H3 :: compute optimised
M-series :: memory optimised (SAP HANA)
A2/A3 :: GPU accelerated
custom machine type :: choose vCPU and memory
live migration :: maintenance without reboot
regional persistent disk :: sync across two zones
local SSD :: ephemeral, fastest
sustained use discount :: automatic monthly discount
CUD :: 1/3-year commitment
Spot VM :: up to 91% off, 30 s preemption
instance template :: immutable VM definition
regional MIG :: VMs across zones
autohealing :: recreate on failed health check
rolling update :: maxSurge / maxUnavailable
GKE Autopilot :: Google manages nodes
Workload Identity :: pods use IAM without keys
Cloud Run :: serverless containers, scale to zero
Cloud Run concurrency default :: 80 requests

@@drill
How much notice does a Spot VM get before preemption?
= 30 seconds
= 30
? Use a shutdown script to clean up.

@@drill
Which GKE mode manages the nodes for you?
= Autopilot
= autopilot
? Standard gives node-level control.

@@exercise Choose a compute service
Recommend a compute option for each workload with rules checked in order: `needs_os_control` or `licensing` → Compute Engine (add "MIG" if `scales`); `kubernetes` → GKE Autopilot, or GKE Standard if it needs `gpu` or `daemonsets`; `event_function` → Cloud Run functions; `container` and `stateless` → Cloud Run; otherwise Compute Engine. Then add "+ Spot" when `interruptible`. Print each recommendation.
@@lang python
@@starter
workloads = [
    {"name": "legacy ERP", "needs_os_control": True, "licensing": True},
    {"name": "storefront API", "container": True, "stateless": True},
    {"name": "ML training", "kubernetes": True, "gpu": True, "interruptible": True},
    {"name": "microservices", "kubernetes": True},
    {"name": "thumbnailer", "event_function": True},
    {"name": "render farm", "needs_os_control": True, "scales": True, "interruptible": True},
    {"name": "log agent fleet", "kubernetes": True, "daemonsets": True},
]
@@expected
legacy ERP: Compute Engine
storefront API: Cloud Run
ML training: GKE Standard + Spot
microservices: GKE Autopilot
thumbnailer: Cloud Run functions
render farm: Compute Engine MIG + Spot
log agent fleet: GKE Standard
@@requires
def recommend
.get(
@@hint
Use `w.get("key")` for the optional flags and append " + Spot" at the end.
@@solution
workloads = [
    {"name": "legacy ERP", "needs_os_control": True, "licensing": True},
    {"name": "storefront API", "container": True, "stateless": True},
    {"name": "ML training", "kubernetes": True, "gpu": True, "interruptible": True},
    {"name": "microservices", "kubernetes": True},
    {"name": "thumbnailer", "event_function": True},
    {"name": "render farm", "needs_os_control": True, "scales": True, "interruptible": True},
    {"name": "log agent fleet", "kubernetes": True, "daemonsets": True},
]

def recommend(w):
    if w.get("needs_os_control") or w.get("licensing"):
        choice = "Compute Engine" + (" MIG" if w.get("scales") else "")
    elif w.get("kubernetes"):
        choice = "GKE Standard" if w.get("gpu") or w.get("daemonsets") else "GKE Autopilot"
    elif w.get("event_function"):
        choice = "Cloud Run functions"
    elif w.get("container") and w.get("stateless"):
        choice = "Cloud Run"
    else:
        choice = "Compute Engine"
    if w.get("interruptible"):
        choice += " + Spot"
    return choice

for w in workloads:
    print(f"{w['name']}: {recommend(w)}")

@@exercise Compare discounts for a VM fleet
Compute the monthly cost of each VM group (730 hours per month at full usage) using the example hourly prices (illustrative). Apply: Spot (−70%) if `spot`; a 3-year CUD (−55%) or 1-year CUD (−37%) if `commit` is 3 or 1; otherwise a sustained use discount for eligible families (not `e2`) when `hours` ≥ 25% of the month, scaling linearly up to −30% at a full month. Print each group's list cost, discount and final cost, then the total.
@@lang python
@@starter
PRICE = {"e2-standard-4": 0.134, "n2-standard-8": 0.388, "c2-standard-16": 0.835, "n2-highmem-16": 1.048}
groups = [
    {"name": "web", "type": "e2-standard-4", "count": 6, "hours": 730, "commit": 1, "spot": False},
    {"name": "api", "type": "n2-standard-8", "count": 4, "hours": 730, "commit": 0, "spot": False},
    {"name": "batch", "type": "c2-standard-16", "count": 10, "hours": 200, "commit": 0, "spot": True},
    {"name": "analytics", "type": "n2-highmem-16", "count": 2, "hours": 365, "commit": 0, "spot": False},
    {"name": "db", "type": "n2-highmem-16", "count": 2, "hours": 730, "commit": 3, "spot": False},
    {"name": "ci", "type": "n2-standard-8", "count": 3, "hours": 120, "commit": 0, "spot": False},
]

def discount(group):
    raise NotImplementedError
@@expected
web        $   586.92  1-year CUD    -37.0%  $   369.76
api        $ 1,132.96  sustained use -30.0%  $   793.07
batch      $ 1,670.00  Spot          -70.0%  $   501.00
analytics  $   765.04  sustained use -10.0%  $   688.54
db         $ 1,530.08  3-year CUD    -55.0%  $   688.54
ci         $   139.68  none          - 0.0%  $   139.68
Total: $3,180.58/month
@@requires
def discount
startswith("e2")
@@hint
SUD fraction: `usage = hours / 730`; if `usage >= 0.25`, discount `0.30 * (usage - 0.25) / 0.75`. Return `(label, fraction)`.
@@solution
PRICE = {"e2-standard-4": 0.134, "n2-standard-8": 0.388, "c2-standard-16": 0.835, "n2-highmem-16": 1.048}
groups = [
    {"name": "web", "type": "e2-standard-4", "count": 6, "hours": 730, "commit": 1, "spot": False},
    {"name": "api", "type": "n2-standard-8", "count": 4, "hours": 730, "commit": 0, "spot": False},
    {"name": "batch", "type": "c2-standard-16", "count": 10, "hours": 200, "commit": 0, "spot": True},
    {"name": "analytics", "type": "n2-highmem-16", "count": 2, "hours": 365, "commit": 0, "spot": False},
    {"name": "db", "type": "n2-highmem-16", "count": 2, "hours": 730, "commit": 3, "spot": False},
    {"name": "ci", "type": "n2-standard-8", "count": 3, "hours": 120, "commit": 0, "spot": False},
]

def discount(group):
    if group["spot"]:
        return "Spot", 0.70
    if group["commit"] == 3:
        return "3-year CUD", 0.55
    if group["commit"] == 1:
        return "1-year CUD", 0.37
    usage = group["hours"] / 730
    if not group["type"].startswith("e2") and usage >= 0.25:
        return "sustained use", 0.30 * (usage - 0.25) / 0.75
    return "none", 0.0

total = 0
for g in groups:
    list_cost = PRICE[g["type"]] * g["count"] * g["hours"]
    label, fraction = discount(g)
    final = list_cost * (1 - fraction)
    total += final
    print(f"{g['name']:<10} ${list_cost:>9,.2f}  {label:<13} -{fraction:5.1%}  ${final:>9,.2f}")
print(f"Total: ${total:,.2f}/month")

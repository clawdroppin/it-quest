@@lesson az2-compute-deep Azure Compute in Depth: VMs, Availability, Scale Sets, App Service and Containers
@@text
"Deploy and manage Azure compute resources" is 20–25% of AZ-104. This lesson covers VM sizing and disks, the availability options and their SLAs, Virtual Machine Scale Sets with autoscale, App Service plans and slots, and the container options (ACI, Container Apps, AKS).

## Virtual machines

- **Sizes** by series: **B** (burstable), **D** (general purpose), **E** (memory), **F** (compute), **L** (storage), **N** (GPU), **M** (very large memory). Names like `Standard_D4s_v5`: D series, 4 vCPUs, **s** = supports premium SSD, version 5.
- **Disks**: OS disk, optional **data disks**, temporary disk (lost on redeploy/deallocation). Managed disk types: **Standard HDD**, **Standard SSD**, **Premium SSD** (v2 lets you set IOPS/throughput independently), **Ultra Disk**. Disk encryption: server-side (platform or customer-managed keys) and **Azure Disk Encryption** (BitLocker/dm-crypt) or encryption at host.
- **Resizing** may require a restart (and deallocation if the new size isn't available on the current hardware cluster).
- **Stopped (allocated)** still bills compute; **Stopped (deallocated)** doesn't (disks still bill).
- Connect with RDP/SSH through **Azure Bastion** (no public IP needed) or JIT VM access in Defender for Cloud.
- **Custom script extension**, **cloud-init**, VM Applications and **Azure Compute Gallery** images automate configuration.

## Availability options

| Option | What it does | VM SLA |
|---|---|---|
| Single VM with Premium SSD/Ultra disks | nothing redundant | 99.9% |
| **Availability set** | spreads VMs across **fault domains** (up to 3: separate power/network racks) and **update domains** (up to 20, default 5: rebooted one at a time during platform maintenance) in **one datacenter** | 99.95% |
| **Availability zones** | VMs in physically separate zones of a region | 99.99% |
| Region pairs / Site Recovery | protect against regional disasters | DR, not an SLA |

A VM's availability set is chosen **at creation** (to change it, recreate the VM).

## Virtual Machine Scale Sets

Identical (or flexible) VMs managed as a group across zones and fault domains, behind a load balancer. **Orchestration modes**: **Flexible** (recommended: mix of VM types, standard VM APIs) or Uniform.

**Autoscale** rules on metrics (CPU, queue length, custom) or schedules:
- Scale **out** when average CPU > 75% over 10 minutes, by 2 instances; scale **in** when < 25%, by 1.
- **Minimum**, **maximum**, **default** instance counts; **cooldown** (default 5 minutes) between actions.
- Pair scale-out and scale-in rules with a gap between thresholds to avoid **flapping**.
- Upgrade policies: automatic, rolling, manual.

## App Service

PaaS for web apps and APIs (.NET, Java, Node.js, Python, PHP, containers).

- An **App Service plan** defines the region, OS and compute (tier): **Free/Shared** (dev), **Basic** (no autoscale, no slots), **Standard** (autoscale, **5 slots**, daily backups), **Premium v3** (more scale, **20 slots**, zone redundancy), **Isolated v2** (App Service Environment, dedicated VNet). Several apps can share a plan.
- **Deployment slots**: staging slot, warm up, **swap** into production with zero downtime; **slot settings** (sticky) stay with the slot; swap back to roll back.
- Scale **up** (bigger tier) vs scale **out** (more instances, manual or autoscale).
- Networking: **VNet integration** (outbound to private resources), **private endpoints** (inbound), access restrictions; custom domains and TLS certificates (free managed certificates).
- Backups, diagnostic logs, Application Insights.

## Containers

| Service | Use |
|---|---|
| **Azure Container Registry** | private image registry (Basic/Standard/Premium; geo-replication in Premium) |
| **Azure Container Instances** | run one container (group) quickly, per-second billing, no orchestration |
| **Azure Container Apps** | serverless containers with autoscale (KEDA), revisions, ingress, scale to zero |
| **AKS** | managed Kubernetes for full control |
| Web App for Containers | run a container on App Service |

**Azure Functions** runs event-driven code: **Flex Consumption/Consumption** (pay per execution, scale to zero), **Premium** (pre-warmed, VNet), **Dedicated** (App Service plan).

@@quiz
A VM is shut down from inside the OS. Is compute still billed?
* Yes, it's stopped (allocated); deallocate it from Azure to stop compute charges
- No, any stopped VM is free
- Only the temporary disk is billed
- Only if it has a public IP
? Only deallocated VMs stop compute billing.

@@quiz
Which option protects VMs against a whole datacenter failure within a region?
- Availability set
* Availability zones
- Scale set with one fault domain
- Premium SSD
? Availability sets stay within one datacenter.

@@quiz
What do update domains protect against?
- Power failure in a rack
* Planned maintenance rebooting all VMs at once
- Region outages
- Disk corruption
? Fault domains cover power/network failures.

@@quiz
Which App Service tier is the lowest that supports deployment slots and autoscale?
- Free
- Basic
* Standard
- Shared
? Standard includes 5 slots.

@@quiz
A team needs a containerised API that scales to zero when idle and scales on HTTP traffic, without managing Kubernetes. Which service?
- AKS
- Azure Container Instances
* Azure Container Apps
- VM Scale Sets
? Container Apps uses KEDA-based autoscale.

@@quiz
After swapping a staging slot into production, a connection string must stay with the production slot. How?
* Mark it as a deployment slot setting
- Store it in the code
- Use a different plan
- Disable swap
? Slot settings are sticky.

@@flashcards
Standard_D4s_v5 :: D series, 4 vCPU, premium SSD, v5
B series :: burstable
temporary disk :: lost on redeploy/deallocate
deallocated :: no compute charges
Bastion :: RDP/SSH without public IP
fault domain :: separate power/network rack
update domain :: maintenance reboot group
availability set SLA :: 99.95%
availability zones SLA :: 99.99%
single VM premium SSD SLA :: 99.9%
VMSS :: identical VMs with autoscale
autoscale cooldown :: default 5 minutes
App Service plan :: compute for apps
Standard tier :: autoscale, 5 slots
Premium v3 :: 20 slots, zone redundancy
deployment slot swap :: zero-downtime release
slot setting :: sticky to the slot
ACR :: private image registry
ACI :: single containers, per second
Container Apps :: serverless containers, KEDA
AKS :: managed Kubernetes

@@drill
How many fault domains can an availability set have at most?
= 3
= three
? Update domains: up to 20.

@@drill
Which App Service feature swaps a staging deployment into production?
= deployment slots
= slots
= slot swap
= Deployment slots
? Swap back to roll back.

@@exercise Place VMs in an availability set
Place VMs into an availability set with `FAULT_DOMAINS` fault domains and `UPDATE_DOMAINS` update domains: VM i goes to fault domain `i % FAULT_DOMAINS` and update domain `i % UPDATE_DOMAINS`. Print each VM's placement, then the worst case: how many VMs stay up when one fault domain fails, and during planned maintenance of one update domain (the domain with the most VMs).
@@lang python
@@starter
FAULT_DOMAINS = 2
UPDATE_DOMAINS = 5
vms = [f"web{i}" for i in range(1, 8)]
@@expected
web1: FD 0, UD 0
web2: FD 1, UD 1
web3: FD 0, UD 2
web4: FD 1, UD 3
web5: FD 0, UD 4
web6: FD 1, UD 0
web7: FD 0, UD 1
Fault domain failure: 3 of 7 VMs stay up
Update domain maintenance: 5 of 7 VMs stay up
@@requires
FAULT_DOMAINS
UPDATE_DOMAINS
Counter(
@@hint
`from collections import Counter`; count VMs per domain and subtract the largest count from the total.
@@solution
from collections import Counter

FAULT_DOMAINS = 2
UPDATE_DOMAINS = 5
vms = [f"web{i}" for i in range(1, 8)]

placement = {}
for i, vm in enumerate(vms):
    placement[vm] = (i % FAULT_DOMAINS, i % UPDATE_DOMAINS)
    print(f"{vm}: FD {i % FAULT_DOMAINS}, UD {i % UPDATE_DOMAINS}")

fd = Counter(f for f, _ in placement.values())
ud = Counter(u for _, u in placement.values())
print(f"Fault domain failure: {len(vms) - max(fd.values())} of {len(vms)} VMs stay up")
print(f"Update domain maintenance: {len(vms) - max(ud.values())} of {len(vms)} VMs stay up")

@@exercise Simulate VMSS autoscale
Simulate autoscale on 5-minute average CPU samples. Rules: scale out by `OUT_STEP` when CPU > `OUT_THRESHOLD`, scale in by `IN_STEP` when CPU < `IN_THRESHOLD`, never below `MIN` or above `MAX`, and do nothing during the `COOLDOWN` minutes after a scale action. Assume load spreads evenly, so after a change the next samples' CPU is scaled by `old_count / new_count`. Print every scale action and the final count.
@@lang python
@@starter
MIN, MAX, DEFAULT = 2, 10, 2
OUT_THRESHOLD, IN_THRESHOLD = 75, 25
OUT_STEP, IN_STEP = 2, 1
COOLDOWN = 10
SAMPLE_MINUTES = 5
cpu_at_default = [40, 60, 85, 95, 90, 80, 70, 50, 30, 20, 15, 10, 10, 10]
@@expected
minute 10: CPU 85% -> 2 to 4 instances
minute 40: CPU 15% -> 4 to 3 instances
minute 50: CPU 10% -> 3 to 2 instances
Final instance count: 2
@@requires
COOLDOWN
min(MAX,
max(MIN,
@@hint
The demand in "instance units" is `cpu_at_default[i] * DEFAULT`; the observed CPU with `count` instances is that divided by `count`. Track `last_action_minute` for the cooldown.
@@solution
MIN, MAX, DEFAULT = 2, 10, 2
OUT_THRESHOLD, IN_THRESHOLD = 75, 25
OUT_STEP, IN_STEP = 2, 1
COOLDOWN = 10
SAMPLE_MINUTES = 5
cpu_at_default = [40, 60, 85, 95, 90, 80, 70, 50, 30, 20, 15, 10, 10, 10]

count = DEFAULT
last_action = -COOLDOWN
for i, base in enumerate(cpu_at_default):
    minute = i * SAMPLE_MINUTES
    cpu = base * DEFAULT / count
    if minute - last_action < COOLDOWN:
        continue
    new = count
    if cpu > OUT_THRESHOLD:
        new = min(MAX, count + OUT_STEP)
    elif cpu < IN_THRESHOLD:
        new = max(MIN, count - IN_STEP)
    if new != count:
        print(f"minute {minute:>2}: CPU {cpu:.0f}% -> {count} to {new} instances")
        count = new
        last_action = minute
print("Final instance count:", count)

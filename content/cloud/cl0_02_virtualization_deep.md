@@lesson cl0-virtualization-deep Virtualization, Capacity and Migration in Depth
@@text
Cloud+ (CV0-004) and every cloud job interview expect you to size workloads, understand what a hypervisor really shares, and plan a migration. This lesson goes deeper into hypervisor resources and overcommitment, high availability on virtual hosts, the shared responsibility model, and the migration strategies known as the "7 Rs".

## What a hypervisor shares

- **Type 1** (bare metal: ESXi, Hyper-V, KVM, Xen, Nitro) vs **Type 2** (hosted: VirtualBox, VMware Workstation).
- **CPU**: vCPUs are scheduled onto physical cores (threads). **Overcommit** ratios such as 4:1 vCPU:pCPU are normal for general workloads; latency-sensitive workloads need 1:1 or **CPU pinning**. Watch **CPU ready / steal time**.
- **Memory**: overcommit uses ballooning, page sharing, compression and swapping; heavy memory overcommit hurts badly, so many designs keep it at 1:1. Reserve memory for the hypervisor itself.
- **Storage**: thin provisioning (allocate on write: watch for running out), thick provisioning; snapshots are not backups (they depend on the base disk and slow it down).
- **Network**: virtual switches, VLAN tagging, SR-IOV for near-native performance.
- **NUMA**: keep a VM's vCPUs and memory within one NUMA node when possible.

## High availability on hosts

- **N+1** capacity: the cluster must run all VMs with one host failed (N+2 during maintenance).
- **Live migration** (vMotion) moves running VMs; HA restarts VMs on surviving hosts after a failure; **anti-affinity rules** keep replicas on different hosts.
- In the cloud, the same ideas appear as placement groups (spread, cluster, partition), availability zones and auto scaling groups.

## Service models and responsibility

| You manage | IaaS | PaaS | SaaS |
|---|---|---|---|
| Data, access, identities | ✔ | ✔ | ✔ |
| Application | ✔ | ✔ | |
| Runtime, middleware | ✔ | | |
| OS and patching | ✔ | | |
| Virtualization, hardware, facility | | | |

The customer is **always** responsible for data, identities and access configuration. Deployment models: public, private, hybrid, multicloud, community.

## Migration: the 7 Rs

| Strategy | What | When |
|---|---|---|
| **Retire** | switch it off | unused or duplicated |
| **Retain** | leave on premises for now | compliance, recent investment, too risky |
| **Rehost** | lift and shift to VMs | fast exit from a data centre |
| **Relocate** | move the hypervisor platform (e.g. VMware Cloud) | VMware estates, no change |
| **Replatform** | lift, tinker and shift (managed DB, containers) | quick wins without rewriting |
| **Repurchase** | move to SaaS | commodity apps (CRM, email) |
| **Refactor / re-architect** | rebuild cloud-native | scale, agility, cost needs justify effort |

Migration phases: assess (discovery, dependencies, TCO) → mobilise (landing zone, skills, pilot) → migrate in **waves** grouped by dependencies → optimise. Data transfer: online (VPN, Direct Connect/ExpressRoute/Interconnect) vs offline (Snowball, Data Box) when `size / bandwidth` takes too long.

## Transfer time

**time = data size (bits) / (bandwidth × utilisation)**. 100 TB over 1 Gbps at 80% utilisation ≈ 100 × 8 × 10¹² / (0.8 × 10⁹) s ≈ **11.6 days**: often too slow, so ship an appliance.

@@quiz
A host has 32 physical cores and the policy allows 4:1 vCPU overcommit. How many vCPUs can be allocated?
- 32
- 64
* 128
- 256
? 32 × 4.

@@quiz
Why are VM snapshots not backups?
- They are too small
* They depend on the original disk and live on the same storage
- They can't be restored
- They are encrypted
? Lose the datastore and you lose both.

@@quiz
A cluster of 5 hosts must tolerate one host failure. How much of its total capacity can be used?
- 100%
* 80%
- 50%
- 60%
? N+1: 4 of 5 hosts must carry the load.

@@quiz
A company moves a commodity CRM to a SaaS product. Which strategy is this?
- Rehost
- Replatform
* Repurchase
- Retain
? Drop and shop.

@@quiz
Which is always the customer's responsibility, even in SaaS?
- Physical security
- Hypervisor patching
* Data and identity/access management
- OS patching
? Shared responsibility.

@@quiz
Moving 500 TB over a 1 Gbps link would take about two months. What's the usual alternative?
- Compress harder
* An offline transfer appliance (Snowball, Data Box)
- A second VPN
- Rehost instead
? Ship the disks.

@@flashcards
Type 1 hypervisor :: bare metal
Type 2 hypervisor :: hosted on an OS
vCPU overcommit :: e.g. 4:1 general workloads
CPU ready / steal :: waiting for a physical CPU
memory overcommit :: ballooning, swapping, risky
thin provisioning :: allocate on write
snapshot :: not a backup
SR-IOV :: near-native NIC performance
NUMA :: keep vCPU and memory local
N+1 :: survive one host failure
live migration :: move running VMs
anti-affinity :: replicas on different hosts
SaaS customer duty :: data, identities, access
Retire :: switch off
Retain :: keep for now
Rehost :: lift and shift
Relocate :: move hypervisor platform
Replatform :: lift, tinker, shift
Repurchase :: move to SaaS
Refactor :: rebuild cloud-native
transfer time :: bits / (bandwidth × utilisation)

@@drill
Which migration strategy is "lift and shift"?
= Rehost
= rehost
? Replatform changes a little.

@@drill
With N+1 on a 4-host cluster, what percentage of total capacity is usable?
= 75%
= 75
? 3 of 4 hosts.

@@exercise Pack VMs onto hosts
Place VMs on hosts with first-fit decreasing (largest memory first, then name), respecting each host's limits: vCPUs ≤ cores × `CPU_RATIO`, memory ≤ host RAM − `HYPERVISOR_RESERVE_GB` (no memory overcommit), and anti-affinity groups (two VMs in the same group can't share a host). Then check N+1: could all VMs still fit (by totals) if the largest host failed? Print placements, per-host use and the N+1 verdict.
@@lang python
@@starter
CPU_RATIO = 4
HYPERVISOR_RESERVE_GB = 8
hosts = {"esx1": (32, 256), "esx2": (32, 256), "esx3": (24, 192)}
vms = [("db1", 16, 128, "db"), ("db2", 16, 128, "db"), ("app1", 8, 32, "app"), ("app2", 8, 32, "app"), ("app3", 8, 32, "app"),
       ("cache", 4, 64, None), ("build", 32, 48, None), ("web1", 2, 8, "web"), ("web2", 2, 8, "web"), ("big", 64, 200, None)]

def fits(host, vm, used):
    raise NotImplementedError
@@expected
big    -> esx1
db1    -> esx2
db2    -> esx3
cache  -> esx2
build  -> esx1
app1   -> esx2
app2   -> esx3
app3   -> unplaced
web1   -> esx2
web2   -> esx3
esx1: vCPU 96/128, memory 248/248 GB
esx2: vCPU 30/128, memory 232/248 GB
esx3: vCPU 26/96, memory 168/184 GB
N+1 without esx1: need 152 vCPU / 648 GB, have 224 / 432 -> NOT MET
@@requires
def fits
CPU_RATIO
HYPERVISOR_RESERVE_GB
@@hint
Track `used[host] = {"cpu": 0, "mem": 0, "groups": set()}`. Sort with `key=lambda v: (-v[2], v[0])`. For N+1, compare total VM vCPU and memory with the capacity of the remaining hosts after removing the one with the most RAM.
@@solution
CPU_RATIO = 4
HYPERVISOR_RESERVE_GB = 8
hosts = {"esx1": (32, 256), "esx2": (32, 256), "esx3": (24, 192)}
vms = [("db1", 16, 128, "db"), ("db2", 16, 128, "db"), ("app1", 8, 32, "app"), ("app2", 8, 32, "app"), ("app3", 8, 32, "app"),
       ("cache", 4, 64, None), ("build", 32, 48, None), ("web1", 2, 8, "web"), ("web2", 2, 8, "web"), ("big", 64, 200, None)]

def capacity(host):
    cores, ram = hosts[host]
    return cores * CPU_RATIO, ram - HYPERVISOR_RESERVE_GB

def fits(host, vm, used):
    _, cpu, mem, group = vm
    max_cpu, max_mem = capacity(host)
    u = used[host]
    return u["cpu"] + cpu <= max_cpu and u["mem"] + mem <= max_mem and (group is None or group not in u["groups"])

used = {h: {"cpu": 0, "mem": 0, "groups": set()} for h in hosts}
placed = []
for vm in sorted(vms, key=lambda v: (-v[2], v[0])):
    host = next((h for h in hosts if fits(h, vm, used)), None)
    if host is None:
        print(f"{vm[0]:<6} -> unplaced")
        continue
    used[host]["cpu"] += vm[1]
    used[host]["mem"] += vm[2]
    if vm[3]:
        used[host]["groups"].add(vm[3])
    placed.append(vm)
    print(f"{vm[0]:<6} -> {host}")

for h in hosts:
    max_cpu, max_mem = capacity(h)
    print(f"{h}: vCPU {used[h]['cpu']}/{max_cpu}, memory {used[h]['mem']}/{max_mem} GB")

largest = max(hosts, key=lambda h: hosts[h][1])
rest = [capacity(h) for h in hosts if h != largest]
need_cpu, need_mem = sum(v[1] for v in placed), sum(v[2] for v in placed)
ok = need_cpu <= sum(c for c, _ in rest) and need_mem <= sum(m for _, m in rest)
print(f"N+1 without {largest}: need {need_cpu} vCPU / {need_mem} GB, have {sum(c for c, _ in rest)} / {sum(m for _, m in rest)} -> {'OK' if ok else 'NOT MET'}")

@@exercise Choose a migration strategy and transfer method
For each application, pick a strategy with these rules in order: unused for over 180 days → Retire; compliance hold → Retain; commodity with a SaaS alternative → Repurchase; runs on VMware and must move fast → Relocate; needs to scale and the business funds a rewrite → Refactor; uses a self-managed database → Replatform; otherwise Rehost. Then compute online transfer days for its data at 80% link utilisation and choose an offline appliance if it would take more than `MAX_ONLINE_DAYS`.
@@lang python
@@starter
LINK_GBPS = 1
UTILISATION = 0.8
MAX_ONLINE_DAYS = 7
apps = [
    {"name": "intranet-wiki", "idle_days": 400, "data_tb": 0.2},
    {"name": "payroll", "compliance_hold": True, "data_tb": 2},
    {"name": "crm", "saas_alternative": True, "data_tb": 1.5},
    {"name": "erp", "vmware": True, "deadline_urgent": True, "data_tb": 40},
    {"name": "shop", "needs_scale": True, "rewrite_funded": True, "data_tb": 5},
    {"name": "reporting", "self_managed_db": True, "data_tb": 120},
    {"name": "file-share", "data_tb": 300},
]

def strategy(app):
    raise NotImplementedError
@@expected
intranet-wiki  Retire      no data transfer
payroll        Retain      no data transfer
crm            Repurchase     1.5 TB,   0.2 days online -> online
erp            Relocate        40 TB,   4.6 days online -> online
shop           Refactor         5 TB,   0.6 days online -> online
reporting      Replatform     120 TB,  13.9 days online -> offline appliance
file-share     Rehost         300 TB,  34.7 days online -> offline appliance
@@requires
def strategy
def transfer_days
MAX_ONLINE_DAYS
@@hint
`app.get("idle_days", 0) > 180` and so on, in order. Days = `tb * 8e12 / (LINK_GBPS * 1e9 * UTILISATION) / 86400`. Retired and retained apps don't transfer data.
@@solution
LINK_GBPS = 1
UTILISATION = 0.8
MAX_ONLINE_DAYS = 7
apps = [
    {"name": "intranet-wiki", "idle_days": 400, "data_tb": 0.2},
    {"name": "payroll", "compliance_hold": True, "data_tb": 2},
    {"name": "crm", "saas_alternative": True, "data_tb": 1.5},
    {"name": "erp", "vmware": True, "deadline_urgent": True, "data_tb": 40},
    {"name": "shop", "needs_scale": True, "rewrite_funded": True, "data_tb": 5},
    {"name": "reporting", "self_managed_db": True, "data_tb": 120},
    {"name": "file-share", "data_tb": 300},
]

def strategy(app):
    if app.get("idle_days", 0) > 180:
        return "Retire"
    if app.get("compliance_hold"):
        return "Retain"
    if app.get("saas_alternative"):
        return "Repurchase"
    if app.get("vmware") and app.get("deadline_urgent"):
        return "Relocate"
    if app.get("needs_scale") and app.get("rewrite_funded"):
        return "Refactor"
    if app.get("self_managed_db"):
        return "Replatform"
    return "Rehost"

def transfer_days(tb):
    return tb * 8e12 / (LINK_GBPS * 1e9 * UTILISATION) / 86400

for app in apps:
    s = strategy(app)
    if s in ("Retire", "Retain"):
        print(f"{app['name']:<14} {s:<11} no data transfer")
        continue
    days = transfer_days(app["data_tb"])
    method = "offline appliance" if days > MAX_ONLINE_DAYS else "online"
    print(f"{app['name']:<14} {s:<11} {app['data_tb']:>6} TB, {days:5.1f} days online -> {method}")

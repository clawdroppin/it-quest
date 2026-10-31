@@lesson cl0-virtualization Virtualization and Cloud Models
@@text
Before AWS and Azure, there's the idea underneath them: **virtualization**, which means running many isolated computers on one physical machine. Every cloud is virtualization at enormous scale, plus an API.

## Hypervisors

| | Type 1 (bare metal) | Type 2 (hosted) |
|---|---|---|
| Runs on | the hardware directly | a normal OS |
| Examples | VMware ESXi, Microsoft Hyper-V, KVM, Xen, Proxmox VE | VirtualBox, VMware Workstation, Parallels |
| Used for | servers, data centres, clouds | labs and testing on a laptop |

The CPU needs hardware virtualization (**Intel VT-x / AMD-V**, enabled in the UEFI). Each **VM** has virtual CPUs, RAM, disks (files like `.vmdk`, `.vhdx`, `.qcow2`) and NICs connected to **virtual switches**.

VM operations you'll do at work: **snapshots** (a point in time to roll back to; *not* a backup, because it lives on the same storage), **cloning** and **templates** (golden images), **live migration** (vMotion: move a running VM to another host), **overcommitting** (more vCPUs/RAM than physical; fine until everyone gets busy), and **high availability** (restart VMs on another host if one fails).

## VMs vs containers

```text
   VMs                                   Containers
┌──────┐┌──────┐┌──────┐            ┌──────┐┌──────┐┌──────┐
│ App  ││ App  ││ App  │            │ App  ││ App  ││ App  │
│ Libs ││ Libs ││ Libs │            │ Libs ││ Libs ││ Libs │
│Guest ││Guest ││Guest │            └──────┘└──────┘└──────┘
│  OS  ││  OS  ││  OS  │            ┌──────────────────────┐
└──────┘└──────┘└──────┘            │ container runtime     │
┌──────────────────────┐            ├──────────────────────┤
│      hypervisor      │            │   host OS (one kernel)│
├──────────────────────┤            ├──────────────────────┤
│       hardware       │            │       hardware       │
└──────────────────────┘            └──────────────────────┘
```

A **container** shares the host's kernel and isolates processes with Linux **namespaces** (own process list, network, file system view) and **cgroups** (CPU and memory limits). It starts in milliseconds and weighs megabytes, not gigabytes. The trade-off: weaker isolation than a VM, and Linux containers need a Linux kernel (Docker Desktop on Windows runs one in a small VM, via **WSL 2**).

## Cloud models, one more time

- **IaaS** (VMs, networks, disks) → **PaaS** (App Service, Elastic Beanstalk, managed databases) → **serverless / FaaS** (Lambda, Functions) → **SaaS** (Microsoft 365, Salesforce). Moving right: less to manage, less control.
- **Containers as a service**: ECS/Fargate, Azure Container Apps, Google Cloud Run. **Managed Kubernetes**: EKS, AKS, GKE.
- Deployment: **public**, **private** (OpenStack, VMware in your data centre), **hybrid**, **multi-cloud**; **community** cloud (shared by organizations with the same needs, e.g. Japanese government's **ガバメントクラウド**, which uses public cloud providers).
- NIST's five characteristics: **on-demand self-service**, **broad network access**, **resource pooling**, **rapid elasticity**, **measured service**.

## Migration: the "7 Rs"

| R | Means |
|---|---|
| **Retire** | switch it off |
| **Retain** | keep it on-premises for now |
| **Rehost** | "lift and shift" the VM as it is |
| **Relocate** | move the whole VMware platform (e.g. VMware Cloud on AWS) |
| **Replatform** | small changes ("lift and reshape"): e.g. move the database to RDS |
| **Repurchase** | switch to SaaS (Exchange → Exchange Online) |
| **Refactor / re-architect** | rewrite as cloud-native |

## Cost: FinOps basics

Cloud turns cost into an engineering problem. **FinOps** practices: tag everything for **showback/chargeback**, **right-size** (most VMs are oversized), switch off dev at night, **reserved instances / savings plans** for steady loads, **spot** for interruptible work, storage **lifecycle** rules, watch **egress** (data out), and set **budgets** with alerts. A forgotten GPU VM can cost more than your monthly salary.

@@quiz
Which is a Type 1 hypervisor?
* VMware ESXi
- VirtualBox
- VMware Workstation
- Parallels Desktop
? Type 1 runs directly on the hardware; the others run on top of a desktop OS.

@@quiz
A technician takes a snapshot of a VM and says "good, it's backed up". What's wrong?
* A snapshot lives on the same storage; if that fails, the snapshot is gone too
- Snapshots can't be restored
- Snapshots only save RAM
- Nothing, a snapshot is a backup
? Snapshots are for short-term rollback (before an update). Long-running snapshots also slow the VM down.

@@quiz
Why do containers start much faster than VMs?
* They share the host kernel instead of booting their own OS
- They have no file system
- They run in the CPU cache
- They skip networking
? A container is an isolated process, not a whole computer.

@@quiz
Which Linux kernel features isolate containers and limit their resources?
* Namespaces and cgroups
- SELinux and AppArmor only
- iptables and routes
- cron and systemd
? SELinux/AppArmor add extra protection on top.

@@quiz
A company moves its VMs to EC2 without changing them. Which migration strategy is this?
* Rehost (lift and shift)
- Refactor
- Repurchase
- Retire
? Fastest to do, but doesn't use cloud features.

@@quiz
Replacing an on-premises Exchange server with Exchange Online is...
* Repurchase (moving to SaaS)
- Rehost
- Retain
- Relocate
? You buy the capability as a service instead of running the software.

@@quiz
Which NIST cloud characteristic means you're billed for exactly what you use, and usage is monitored?
* Measured service
- Broad network access
- Resource pooling
- On-demand self-service
? Like a utility meter.

@@quiz
What must be enabled in the UEFI for a 64-bit hypervisor to run VMs?
* Hardware virtualization (Intel VT-x / AMD-V)
- Secure Boot
- Fast boot
- Legacy CSM
? It's often off by default on desktop PCs.

@@flashcards
Type 1 hypervisor :: Runs on bare metal: ESXi, Hyper-V, KVM, Proxmox
Type 2 hypervisor :: Runs on an OS: VirtualBox, VMware Workstation
VT-x / AMD-V :: CPU virtualization extensions
snapshot :: Short-term rollback point; not a backup
template / golden image :: Pre-built VM to clone from
live migration :: Moving a running VM to another host
container :: Isolated process sharing the host kernel
namespaces :: Isolate what a container can see
cgroups :: Limit what a container can use
WSL 2 :: Runs a real Linux kernel on Windows (Docker Desktop uses it)
rehost :: Lift and shift
replatform :: Lift and reshape (small cloud changes)
refactor :: Re-architect as cloud-native
repurchase :: Move to SaaS
FinOps :: Managing cloud cost as a shared engineering practice
egress :: Data leaving the cloud: usually charged

@@drill
What kind of hypervisor is VirtualBox? (Type 1 or Type 2)
= Type 2
? It runs as an application on Windows, macOS or Linux.

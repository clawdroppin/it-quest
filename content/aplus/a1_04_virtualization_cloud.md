@@lesson a1-cloud Virtualization and Cloud for A+
@@text
(The basics are in the Foundations track: hypervisor types, IaaS/PaaS/SaaS.)

## Virtualization details

- **Requirements**: CPU virtualization (VT-x / AMD-V) on in the UEFI, enough **RAM** (each VM takes its own), disk space, and a 64-bit host.
- **Uses**: a sandbox to test software or open suspicious files, running old apps on an old OS, labs, servers.
- **Virtual networking**: *NAT* (VM shares the host's connection), *bridged* (VM appears on the real network with its own IP), *internal/host-only* (VMs talk only to each other/the host).
- **Snapshots** save a VM's state so you can roll back: perfect before an update. They're not backups.
- **VDI** (virtual desktop infrastructure): users' desktops run as VMs in the data centre; any device can connect. Azure Virtual Desktop, Citrix, VMware Horizon.
- **Containers** share the host kernel; VMs each have their own OS.

## Cloud characteristics

| Term | Meaning |
|---|---|
| **On-demand self-service** | spin up resources yourself, no ticket to a vendor |
| **Rapid elasticity** | scale up and down automatically with load |
| **Metered / measured service** | pay for what you use |
| **Resource pooling** | the provider shares hardware among many customers (multitenancy) |
| **High availability** | runs in several data centres, survives failures |
| **File synchronization** | OneDrive, Google Drive, Dropbox keep files in sync |

Deployment models: **public** (AWS, Azure), **private** (a company's own cloud), **hybrid** (both, connected), **community** (shared by organizations with common needs).
## Troubleshooting virtual machines

A+ scenarios often describe a VM that won't behave. The usual causes:

| Symptom | Likely cause |
|---|---|
| The hypervisor says virtualization isn't supported | VT-x / AMD-V is turned off in the UEFI settings |
| The host slows to a crawl when VMs start | Not enough RAM: each VM reserves its own memory |
| The VM has no internet access | The virtual switch or network mode is wrong (internal or host-only instead of NAT or bridged) |
| Other PCs on the LAN can't reach the VM | It's on NAT; switch to bridged so it gets its own address on the real network |
| The host's disk fills up | Dynamically expanding virtual disks and old snapshots keep growing |

**Snapshots are not backups.** A snapshot depends on the original virtual disk. If that disk is lost, the snapshot is lost with it. Delete snapshots once you no longer need them, because long chains slow the VM down.

## Who is responsible for what

In the cloud, security is a **shared responsibility**. The provider always secures the physical data centre and hardware. What the customer secures depends on the service model:

- **IaaS:** the customer manages the operating system, patches, applications and data.
- **PaaS:** the provider manages the OS; the customer manages the application and data.
- **SaaS:** the provider runs nearly everything. The customer still manages **users, permissions and their data**.

Whatever the model, user accounts and the data people put into the service are always the customer's job.

@@quiz
A technician wants VMs that appear on the office network with their own IP addresses from the office DHCP server. Which virtual network mode?
* Bridged
- NAT
- Internal
- Host-only
? Bridged connects the VM to the physical network as if it were its own machine.

@@quiz
A web shop's cloud servers automatically add capacity during a sale and remove it afterwards. Which cloud characteristic is this?
* Rapid elasticity
- Resource pooling
- Private cloud
- File synchronization
? Elasticity is scaling with demand, often automatically.

@@quiz
A company keeps sensitive databases in its own data centre but runs its website in AWS, connected by VPN. Which deployment model?
* Hybrid cloud
- Public cloud
- Private cloud
- Community cloud
? A mix of private infrastructure and public cloud is hybrid.

@@quiz
Why open a suspicious email attachment inside a VM instead of on your PC?
* The VM is a sandbox: if it's malware, you can roll it back with a snapshot
- VMs can't run programs
- VMs are faster
- Antivirus doesn't work on PCs
? Isolation is one of virtualization's main security uses (use an isolated network too).

@@quiz
A VM needs to be reachable from other computers on the office network with its own IP address. Which network mode should you use?
* Bridged
- NAT
- Host-only
- Internal
? Bridged puts the VM directly on the physical network; NAT hides it behind the host.

@@quiz
In a SaaS email service, what is still the customer's responsibility?
* Managing user accounts, permissions and the data users store
- Patching the mail servers
- Physical security of the data centre
- Replacing failed disks
? The provider runs the platform; identities and data stay with the customer.

@@drill
Which VM network mode puts the VM directly on the physical network with its own IP?
= bridged
= bridged mode
? NAT hides the VM behind the host.

@@flashcards
snapshot :: Saved VM state for rollback; not a backup
bridged networking :: VM gets its own address on the real network
NAT networking :: VM shares the host's connection
VDI :: Desktops running as VMs in the data centre
rapid elasticity :: Scaling up and down with demand
measured service :: Pay per use
multitenancy :: Many customers share the same hardware
hybrid cloud :: On-premises/private plus public cloud, connected
sandbox :: Isolated environment for testing untrusted software

@@exercise Will these VMs fit on the host?
A host has 32 GB of RAM and 8 CPU cores. The hypervisor itself needs 4 GB. Write `plan(vms)` that adds VMs in order and returns the names that fit and the names that don't. A VM fits if the remaining RAM is enough (RAM can't be overcommitted here) and the total vCPUs stay within 2 per physical core (16).

Print the VMs that fit, the ones that don't, and the RAM left over.
@@lang python
@@starter
HOST_RAM, HOST_CORES, HYPERVISOR_RAM = 32, 8, 4
vms = [("dc01", 4, 2), ("web01", 8, 4), ("sql01", 12, 6), ("test01", 8, 2), ("jump01", 2, 2), ("build01", 2, 4)]

def plan(vms):
    raise NotImplementedError


fits, rejected, ram_left = plan(vms)
print("Fits:", ", ".join(fits))
print("Doesn't fit:", ", ".join(rejected))
print(f"RAM left: {ram_left} GB")
@@expected
Fits: dc01, web01, sql01, jump01
Doesn't fit: test01, build01
RAM left: 2 GB
@@requires
def plan
HOST_CORES * 2
@@hint
Keep two running totals, RAM left and vCPUs used, and only update them when a VM is accepted.
@@solution
HOST_RAM, HOST_CORES, HYPERVISOR_RAM = 32, 8, 4
vms = [("dc01", 4, 2), ("web01", 8, 4), ("sql01", 12, 6), ("test01", 8, 2), ("jump01", 2, 2), ("build01", 2, 4)]

def plan(vms):
    ram_left = HOST_RAM - HYPERVISOR_RAM
    vcpus = 0
    fits, rejected = [], []
    for name, ram, cpu in vms:
        if ram <= ram_left and vcpus + cpu <= HOST_CORES * 2:
            fits.append(name)
            ram_left -= ram
            vcpus += cpu
        else:
            rejected.append(name)
    return fits, rejected, ram_left

fits, rejected, ram_left = plan(vms)
print("Fits:", ", ".join(fits))
print("Doesn't fit:", ", ".join(rejected))
print(f"RAM left: {ram_left} GB")

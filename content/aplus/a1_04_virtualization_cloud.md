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

@@lesson az1-services Azure Compute, Networking and Storage
@@text
## Compute

| Service | What |
|---|---|
| **Virtual Machines** | IaaS VMs (Windows, Linux) |
| **Availability sets** | spread VMs over **fault domains** (racks) and **update domains** (maintenance) in one data centre |
| **VM Scale Sets** | identical VMs that scale automatically |
| **Azure Virtual Desktop** | Windows desktops and apps in the cloud (multi-session Windows 11) |
| **App Service** | PaaS for web apps and APIs (.NET, Java, Node, Python, PHP) |
| **Azure Functions** | serverless, event-driven code |
| **Container Instances (ACI)** | run a container without managing servers |
| **Container Apps** | serverless containers with scaling |
| **AKS** | managed Kubernetes |

## Networking

- **Virtual Network (VNet)** with **subnets**; **VNet peering** connects VNets (also across regions: global peering).
- **NSG** (network security group): allow/deny rules by priority (lower number first) on subnets or NICs. **ASGs** group VMs by role for rules.
- **Azure Firewall** (managed, stateful, central), **DDoS Protection**, **Azure Bastion** (RDP/SSH through the portal, no public IPs on VMs).
- Hybrid: **VPN Gateway** (site-to-site, point-to-site over the internet), **ExpressRoute** (private connection through a provider; doesn't cross the public internet).
- **Azure DNS**; **private endpoints** (a PaaS service gets a private IP in your VNet) and service endpoints.
- Load balancing: **Load Balancer** (layer 4, regional), **Application Gateway** (layer 7, with WAF), **Front Door** (global layer 7 + CDN), **Traffic Manager** (DNS-based).

## Storage

A **storage account** holds:

| Service | For |
|---|---|
| **Blob** | objects: files, images, backups, data lakes (Data Lake Storage Gen2) |
| **Azure Files** | SMB/NFS file shares; **Azure File Sync** caches them on Windows servers |
| **Queue** | simple message queues |
| **Table** | NoSQL key-value |
| (Disks) | managed disks for VMs |

**Redundancy**:

| Option | Copies |
|---|---|
| **LRS** | 3 copies in one data centre |
| **ZRS** | 3 copies across availability zones |
| **GRS** | LRS + asynchronous copy to the **paired region** (6 total) |
| **GZRS** | ZRS + copy to the paired region |
| **RA-GRS / RA-GZRS** | + read access in the secondary region |

Blob **access tiers**: **Hot** (frequent), **Cool** (30+ days), **Cold** (90+ days), **Archive** (180+ days, offline: hours to **rehydrate**). **Lifecycle management** moves blobs between tiers. Moving data: **AzCopy**, **Storage Explorer**, **Azure Migrate**, **Data Box** (physical devices).

## Databases

**Azure SQL Database** (PaaS SQL Server), **SQL Managed Instance** (near-full compatibility for migrations), SQL Server on VMs (IaaS), **Cosmos DB** (globally distributed NoSQL, several APIs), Azure Database for **PostgreSQL** / **MySQL**.

@@quiz
Admins need RDP access to VMs without giving the VMs public IP addresses. Which service?
* Azure Bastion
- Azure Firewall
- VPN Gateway only
- Front Door
? Bastion provides RDP/SSH through the portal over TLS.

@@quiz
A company needs a private connection to Azure that doesn't go over the public internet. Which service?
* ExpressRoute
- Site-to-site VPN Gateway
- VNet peering
- Azure DNS
? VPN Gateway tunnels are encrypted but travel over the internet.

@@quiz
Data must survive the loss of an entire region, with read access in the secondary region during an outage. Which redundancy?
* RA-GRS (or RA-GZRS)
- LRS
- ZRS
- GRS
? Plain GRS replicates to the paired region but doesn't let you read the secondary until a failover.

@@quiz
Backups must be kept for 7 years and are almost never read. Which blob tier is cheapest?
* Archive
- Hot
- Cool
- Premium
? Archive is offline; rehydration takes hours.

@@quiz
A web app's developers want to deploy code without managing any OS. Which service?
* App Service
- Virtual Machines
- Availability sets
- VM Scale Sets
? App Service is PaaS.

@@quiz
Which service replaces on-premises Windows file servers with cloud SMB shares, optionally cached on local servers?
* Azure Files with Azure File Sync
- Blob Archive
- Queue storage
- Cosmos DB
? File Sync keeps frequently used files on a local Windows Server.

@@quiz
What do availability sets protect against?
* Hardware (rack) failures and planned maintenance within one data centre
- A whole region going down
- Accidental deletion
- SQL injection
? Availability zones protect against a data-centre failure; region pairs against a region failure.

@@flashcards
VM Scale Sets :: Auto-scaling identical VMs
availability set :: Fault domains + update domains in one data centre
Azure Virtual Desktop :: Cloud-hosted Windows desktops
App Service :: PaaS web hosting
Azure Functions :: Serverless functions
ACI / AKS :: Single containers / managed Kubernetes
VNet peering :: Connect VNets privately
NSG :: Allow/deny rules by priority on subnets/NICs
Azure Bastion :: RDP/SSH via the portal, no public IPs
VPN Gateway :: Encrypted tunnels over the internet
ExpressRoute :: Private connection, not over the internet
private endpoint :: PaaS service with a private IP in your VNet
Load Balancer / Application Gateway / Front Door :: L4 regional / L7 regional + WAF / L7 global + CDN
storage account services :: Blob, Files, Queue, Table
LRS / ZRS / GRS / GZRS :: 1 DC / 3 zones / + paired region / zones + paired region
RA-GRS :: GRS with read access to the secondary
blob tiers :: Hot, Cool, Cold, Archive
AzCopy :: Command-line tool to copy data to storage
Azure SQL Database :: PaaS SQL Server
Cosmos DB :: Global multi-model NoSQL

@@drill
Which storage redundancy keeps three copies within a single data centre? (abbreviation)
= LRS
? Locally redundant storage: cheapest, least resilient.

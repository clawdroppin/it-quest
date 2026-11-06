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

@@exercise Choose a database service
Choose an Azure database service for each workload with simple rules: SQL Server features such as SQL Agent jobs, cross-database queries or CLR mean **Azure SQL Managed Instance**; a lift-and-shift needing OS access means **SQL Server on an Azure VM**; global distribution with multi-region writes or a document model means **Azure Cosmos DB**; PostgreSQL or MySQL apps use the **Azure Database for PostgreSQL/MySQL flexible server**; otherwise a new relational app gets **Azure SQL Database**, on the serverless tier if usage is intermittent.
@@lang python
@@starter
workloads = {
    "HR app (new, relational, busy 9-18 only)": {"engine": "sqlserver", "intermittent": True},
    "Legacy ERP using SQL Agent and cross-db queries": {"engine": "sqlserver", "features": {"sql_agent", "cross_db"}},
    "Global shopping cart, writes in Tokyo and Frankfurt": {"model": "document", "multi_region_writes": True},
    "Django site on PostgreSQL": {"engine": "postgresql"},
    "Vendor app that needs a custom OS agent on the DB server": {"engine": "sqlserver", "os_access": True},
    "New order system, steady load": {"engine": "sqlserver"},
}

def choose(w):
    raise NotImplementedError

for name, w in workloads.items():
    print(f"{name:<55} -> {choose(w)}")
@@expected
HR app (new, relational, busy 9-18 only)                -> Azure SQL Database (serverless)
Legacy ERP using SQL Agent and cross-db queries         -> Azure SQL Managed Instance
Global shopping cart, writes in Tokyo and Frankfurt     -> Azure Cosmos DB
Django site on PostgreSQL                               -> Azure Database for PostgreSQL flexible server
Vendor app that needs a custom OS agent on the DB server -> SQL Server on an Azure VM
New order system, steady load                           -> Azure SQL Database
@@requires
def choose
"Azure SQL Managed Instance"
@@hint
Check the rules in this order: `os_access`, then multi-region writes or a document model, then the PostgreSQL/MySQL engines, then any of the SQL Server instance features, and finally Azure SQL Database (serverless if `intermittent`). Use `w.get(key)` so missing keys are fine.
@@solution
INSTANCE_FEATURES = {"sql_agent", "cross_db", "clr", "service_broker"}

workloads = {
    "HR app (new, relational, busy 9-18 only)": {"engine": "sqlserver", "intermittent": True},
    "Legacy ERP using SQL Agent and cross-db queries": {"engine": "sqlserver", "features": {"sql_agent", "cross_db"}},
    "Global shopping cart, writes in Tokyo and Frankfurt": {"model": "document", "multi_region_writes": True},
    "Django site on PostgreSQL": {"engine": "postgresql"},
    "Vendor app that needs a custom OS agent on the DB server": {"engine": "sqlserver", "os_access": True},
    "New order system, steady load": {"engine": "sqlserver"},
}

def choose(w):
    if w.get("os_access"):
        return "SQL Server on an Azure VM"
    if w.get("multi_region_writes") or w.get("model") == "document":
        return "Azure Cosmos DB"
    if w.get("engine") in ("postgresql", "mysql"):
        return f"Azure Database for {'PostgreSQL' if w['engine'] == 'postgresql' else 'MySQL'} flexible server"
    if w.get("features", set()) & INSTANCE_FEATURES:
        return "Azure SQL Managed Instance"
    return "Azure SQL Database" + (" (serverless)" if w.get("intermittent") else "")

for name, w in workloads.items():
    print(f"{name:<55} -> {choose(w)}")

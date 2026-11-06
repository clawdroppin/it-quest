@@lesson az0-concepts Azure Certifications, Concepts and Architecture
@@text
**Microsoft Azure** is the second-largest cloud, and the natural one for companies already on Windows Server, Active Directory and Microsoft 365, which is a very large share of Japanese enterprises. Regions in Japan: **Japan East** (Tokyo/Saitama) and **Japan West** (Osaka), a **region pair**.

| Exam | Code | For |
|---|---|---|
| **Azure Fundamentals** | AZ-900 | concepts, core services, management and governance |
| **Azure Administrator Associate** | AZ-104 | identities, governance, storage, compute, networking, monitoring |
| Microsoft 365 Fundamentals / Administrator | MS-900 / MS-102 | Microsoft 365, often the same job |
| Security, Compliance and Identity Fundamentals | SC-900 | Entra ID, Defender, Purview |

AZ-900 weights: cloud concepts 25–30%, **architecture and services 35–40%**, management and governance 30–35%; passing score **700**. Microsoft **Learn** has free learning paths and a free practice assessment for every exam, and **Microsoft Virtual Training Days** sometimes give free exam vouchers. Microsoft certifications at the associate level must be **renewed yearly** with a free online assessment.

## Cloud concepts in Microsoft's words

- **Shared responsibility**: you're always responsible for your **data, devices, accounts and identities**; the physical data centre is always Microsoft's; everything between shifts with IaaS → PaaS → SaaS.
- **CapEx vs OpEx**: buying hardware vs paying for use (the **consumption-based model**).
- Benefits: **high availability** (SLAs), **scalability** (vertical: bigger; horizontal: more), **elasticity**, **reliability**, **predictability** (performance and cost), **security and governance**, **manageability** (*of* the cloud: scaling, monitoring; *in* the cloud: portal, CLI, APIs).
- Models: public, private, **hybrid**, and multi-cloud; **Azure Arc** manages servers and Kubernetes anywhere from Azure.

## Architecture

- **Regions** and **region pairs** (at least 300 miles apart where possible; updates roll out one at a time; replication for GRS). **Sovereign regions**: Azure Government (US), Azure operated by 21Vianet (China).
- **Availability zones**: at least three physically separate data centres in a region. Services are **zonal** (pinned to a zone), **zone-redundant** (spread automatically) or non-regional.
- **The hierarchy**:

```text
Management groups      (apply policy and access across many subscriptions)
└─ Subscriptions       (a billing and access boundary)
   └─ Resource groups  (a lifecycle container: delete the group, delete everything in it)
      └─ Resources     (a VM, a storage account, a VNet...)
```

Settings and permissions **inherit** downwards. A resource belongs to exactly one resource group; resource groups can't be nested. **Azure Resource Manager (ARM)** is the deployment and management layer behind the portal, PowerShell, CLI, ARM templates and **Bicep**.

@@quiz
In Azure, which do you ALWAYS remain responsible for, whatever the service model?
* Your data, accounts and identities, and devices
- The physical hosts
- The data centre network
- The hypervisor
? Even in SaaS like Microsoft 365, who has access to your data is your responsibility.

@@quiz
A project is over. How can all its VMs, disks and networks be deleted together?
* Delete their resource group
- Delete the subscription
- Delete the region
- Delete the management group
? A resource group shares a lifecycle.

@@quiz
A company wants one Azure Policy to apply to all 15 of its subscriptions. Where should it be assigned?
* At a management group containing the subscriptions
- At each resource
- At a resource group
- In each VM
? Management groups sit above subscriptions; assignments inherit.

@@quiz
Adding more VMs to handle load is...
* Horizontal scaling (scaling out)
- Vertical scaling (scaling up)
- Elasticity reduction
- A region pair
? Vertical scaling would make one VM bigger.

@@quiz
What's the minimum number of availability zones in an Azure region that supports them?
* Three
- Two
- One
- Five
? Zone-redundant services spread across them.

@@quiz
Which tool manages on-premises servers and Kubernetes clusters in other clouds from the Azure portal?
* Azure Arc
- Azure Advisor
- Azure Bastion
- Azure Front Door
? Arc extends Azure management to resources anywhere.

@@flashcards
AZ-900 :: Azure Fundamentals; pass 700
AZ-104 :: Azure Administrator Associate
Japan East / Japan West :: Tokyo-Saitama / Osaka, a region pair
region pair :: Two regions in one geography for DR and staged updates
availability zone :: Separate data centre in a region; 3+ per region
management group :: Groups subscriptions for policy and access
subscription :: Billing and access boundary
resource group :: Lifecycle container for resources
Azure Resource Manager :: The deployment and management layer
Bicep :: Azure's infrastructure-as-code language (compiles to ARM JSON)
horizontal vs vertical scaling :: More instances vs bigger instances
consumption-based model :: Pay for what you use (OpEx)
Azure Arc :: Manage resources outside Azure

@@drill
What is the container in Azure that you delete to remove a whole project's resources at once? (two words)
= resource group
? Resources can be moved between groups, but each is in exactly one.

@@exercise Composite SLAs
Work out the availability of an architecture: services that all must work multiply their SLAs (in series), and redundant copies that can each serve traffic combine as 1 − (1 − a)ⁿ (in parallel). Compute the composite SLA for a single-region design and for an active-active design across two regions behind Front Door, and the downtime each allows per month (30 days).
@@lang python
@@starter
SLA = {"front_door": 0.9999, "app_service": 0.9995, "sql_db": 0.9999, "storage": 0.999}

def series(*slas):
    raise NotImplementedError

def parallel(sla, copies):
    raise NotImplementedError

def downtime_minutes(sla, days=30):
    return (1 - sla) * days * 24 * 60

region = series(SLA["app_service"], SLA["sql_db"], SLA["storage"])
designs = {
    "single region": region,
    "single region + Front Door": series(SLA["front_door"], region),
    "two regions active-active": series(SLA["front_door"], parallel(region, 2)),
}
for name, sla in designs.items():
    print(f"{name:<28} {sla * 100:.4f}%  ~{downtime_minutes(sla):6.1f} min/month")
@@expected
single region                99.8401%  ~  69.1 min/month
single region + Front Door   99.8301%  ~  73.4 min/month
two regions active-active    99.9897%  ~   4.4 min/month
@@requires
def series
def parallel
** copies
@@hint
`series` multiplies all the values together (start from 1.0). `parallel` is `1 - (1 - sla) ** copies`.
@@solution
SLA = {"front_door": 0.9999, "app_service": 0.9995, "sql_db": 0.9999, "storage": 0.999}

def series(*slas):
    result = 1.0
    for s in slas:
        result *= s
    return result

def parallel(sla, copies):
    return 1 - (1 - sla) ** copies

def downtime_minutes(sla, days=30):
    return (1 - sla) * days * 24 * 60

region = series(SLA["app_service"], SLA["sql_db"], SLA["storage"])
designs = {
    "single region": region,
    "single region + Front Door": series(SLA["front_door"], region),
    "two regions active-active": series(SLA["front_door"], parallel(region, 2)),
}
for name, sla in designs.items():
    print(f"{name:<28} {sla * 100:.4f}%  ~{downtime_minutes(sla):6.1f} min/month")

@@lesson az2-admin Administering Azure (AZ-104)
@@lang powershell
@@text
AZ-104 is a hands-on administrator exam: identities and governance, storage, compute, virtual networking, monitoring and backup. It includes labs and case studies, so you must have actually done these tasks in a subscription (a free account gives credit for the first month; delete resources when you're done).

## Three ways to do everything

| | Example: create a resource group |
|---|---|
| **Portal** | portal.azure.com → Resource groups → Create |
| **Azure PowerShell** (Az module) | `New-AzResourceGroup -Name rg-lab -Location japaneast` |
| **Azure CLI** | `az group create --name rg-lab --location japaneast` |
| **Bicep / ARM templates** | declare it in a file, deploy with `New-AzResourceGroupDeployment` or `az deployment group create` |

**Cloud Shell** (the `>_` icon in the portal) has both PowerShell and the CLI ready, already signed in.

```powershell,norun
Install-Module Az -Scope CurrentUser
Connect-AzAccount
Get-AzSubscription ; Set-AzContext -Subscription "Lab"

New-AzResourceGroup -Name rg-lab -Location japaneast -Tag @{ env = 'lab'; owner = 'ytanaka' }
New-AzVM -ResourceGroupName rg-lab -Name vm-web01 -Location japaneast -Image Ubuntu2204 `
    -Size Standard_B1s -Credential (Get-Credential) -OpenPorts 22
Get-AzVM -ResourceGroupName rg-lab -Status
Stop-AzVM -ResourceGroupName rg-lab -Name vm-web01 -Force      # deallocated: compute billing stops
Remove-AzResourceGroup -Name rg-lab -Force                     # clean up everything
```

```bash,norun
az vm create -g rg-lab -n vm-web01 --image Ubuntu2204 --size Standard_B1s --admin-username azureuser --generate-ssh-keys
az vm list -d -o table
```

## Typical AZ-104 tasks

- **Identities**: create users and groups (bulk with CSV), **dynamic groups** (rules like `user.department -eq "Sales"`), assign licences, configure SSPR, manage guest users, assign RBAC roles, create custom roles.
- **Governance**: Policy assignments, locks, tags, management groups, cost budgets and alerts.
- **Storage**: storage accounts, redundancy, access tiers and lifecycle rules, **SAS tokens** (time-limited delegated access) vs access keys, stored access policies, firewalls and private endpoints, Azure Files and File Sync, AzCopy, soft delete and versioning.
- **Compute**: VMs (sizes, disks, availability sets/zones, extensions), resizing, VM Scale Sets with autoscale rules, App Service plans and **deployment slots** (swap staging into production), container instances, Bicep/ARM deployments.
- **Networking**: VNets and subnets, peering, **NSG** rules and **effective security rules**, Azure DNS (public and private zones), **user-defined routes**, load balancers, Bastion, VPN gateways, **Network Watcher** (IP flow verify, next hop, connection troubleshoot).
- **Monitor and maintain**: Azure Monitor metrics and alerts with **action groups**, Log Analytics and KQL, **Recovery Services vault** with Azure Backup (VMs, files, SQL), **Azure Site Recovery** (replicating VMs to another region), Update Manager.

A KQL query you'll see:

```text
Heartbeat
| where TimeGenerated > ago(1h)
| summarize LastSeen = max(TimeGenerated) by Computer
| where LastSeen < ago(10m)
```

@@quiz
A VM is no longer needed tonight but will be used tomorrow. How do you stop paying for its compute?
* Stop (deallocate) it, e.g. Stop-AzVM; disks are still billed
- Shut it down from inside the OS only
- Delete its resource group
- Change its tags
? Shutting down from inside the guest leaves it allocated and billed.

@@quiz
A partner needs to upload files to one blob container for the next 24 hours, without an account in your directory. What should you give them?
* A SAS token limited to that container, write permission and 24 hours
- The storage account access key
- Owner role on the subscription
- The Global Administrator password
? Access keys give full control of the whole account.

@@quiz
New users in the Sales department should be added to the Sales group automatically. What should you create?
* A dynamic group with a membership rule on department
- A management group
- A resource lock
- An NSG
? Dynamic membership needs Entra ID P1.

@@quiz
A web app's new version should be tested in a production-like environment and then switched live with no downtime. Which App Service feature?
* Deployment slots (swap staging and production)
- VM Scale Sets
- Availability sets
- Resource locks
? Swapping also allows a quick rollback by swapping back.

@@quiz
A VM can't be reached on port 443, and you suspect an NSG. Which Network Watcher tool checks whether a packet would be allowed or denied?
* IP flow verify
- Next hop
- Packet capture only
- Azure Advisor
? It also tells you which rule allowed or denied it. "Effective security rules" on the NIC helps too.

@@quiz
VMs must be replicated to another region so they can be failed over during a regional disaster. Which service?
* Azure Site Recovery
- Azure Backup only
- Azure Files
- Azure Migrate
? Backup restores data; Site Recovery replicates running workloads for failover.

@@flashcards
Connect-AzAccount :: Sign in with Azure PowerShell
az login :: Sign in with Azure CLI
Cloud Shell :: Browser shell with PowerShell and CLI preinstalled
New-AzResourceGroup / az group create :: Create a resource group
Stop-AzVM :: Deallocate a VM: compute billing stops
SAS token :: Time-limited, scoped delegated access to storage
dynamic group :: Membership by rule, e.g. department
deployment slots :: Staging/production swap for App Service
user-defined route :: Custom route table entry, e.g. through a firewall
Network Watcher IP flow verify :: Would a packet be allowed by NSGs?
action group :: Who to notify / what to run when an alert fires
Recovery Services vault :: Azure Backup and Site Recovery container
Azure Site Recovery :: Replicate VMs for disaster recovery failover
KQL :: Kusto Query Language for Log Analytics

@@drill
Which Azure PowerShell cmdlet signs you in?
= Connect-AzAccount
? Then Set-AzContext chooses the subscription.

@@exercise Translate Azure CLI commands to PowerShell
Runbooks mix the Azure CLI and Azure PowerShell. Write `parse(cmd)` that splits an `az` command into its command words and its `--option value` pairs, then `to_powershell(cmd)` that translates the commands in `MAP`: each CLI option becomes a PowerShell parameter (`--resource-group` → `-ResourceGroupName`, `--name` → `-Name`, `--location` → `-Location`, others in `PARAMS`). A command or option that isn't in the tables comes back as `# no mapping for: ...`.
@@lang python
@@starter
MAP = {
    "group create": "New-AzResourceGroup",
    "group delete": "Remove-AzResourceGroup",
    "vm start": "Start-AzVM",
    "vm stop": "Stop-AzVM",
    "storage account create": "New-AzStorageAccount",
    "vm list": "Get-AzVM",
}
PARAMS = {
    "--name": "-Name", "--resource-group": "-ResourceGroupName", "--location": "-Location",
    "--sku": "-SkuName", "--kind": "-Kind",
}
runbook = [
    "az group create --name rg-web --location japaneast",
    "az storage account create --name stwebdata01 --resource-group rg-web --location japaneast --sku Standard_LRS --kind StorageV2",
    "az vm start --name vm-web01 --resource-group rg-web",
    "az vm list --resource-group rg-web",
    "az vm resize --name vm-web01 --size Standard_B2s",
    "az group delete --name rg-old --no-wait",
]

def parse(cmd):
    raise NotImplementedError


def to_powershell(cmd):
    raise NotImplementedError


for line in runbook:
    print(line)
    print("  " + to_powershell(line))
@@expected
az group create --name rg-web --location japaneast
  New-AzResourceGroup -Name rg-web -Location japaneast
az storage account create --name stwebdata01 --resource-group rg-web --location japaneast --sku Standard_LRS --kind StorageV2
  New-AzStorageAccount -Name stwebdata01 -ResourceGroupName rg-web -Location japaneast -SkuName Standard_LRS -Kind StorageV2
az vm start --name vm-web01 --resource-group rg-web
  Start-AzVM -Name vm-web01 -ResourceGroupName rg-web
az vm list --resource-group rg-web
  Get-AzVM -ResourceGroupName rg-web
az vm resize --name vm-web01 --size Standard_B2s
  # no mapping for: az vm resize
az group delete --name rg-old --no-wait
  # no mapping for: --no-wait (in Remove-AzResourceGroup)
@@requires
def parse
def to_powershell
startswith("--")
@@hint
Walk the words with an index: a word starting with `--` is an option, and the next word is its value unless it is another option. Everything else is part of the command name.
@@solution
MAP = {
    "group create": "New-AzResourceGroup",
    "group delete": "Remove-AzResourceGroup",
    "vm start": "Start-AzVM",
    "vm stop": "Stop-AzVM",
    "storage account create": "New-AzStorageAccount",
    "vm list": "Get-AzVM",
}
PARAMS = {
    "--name": "-Name", "--resource-group": "-ResourceGroupName", "--location": "-Location",
    "--sku": "-SkuName", "--kind": "-Kind",
}
runbook = [
    "az group create --name rg-web --location japaneast",
    "az storage account create --name stwebdata01 --resource-group rg-web --location japaneast --sku Standard_LRS --kind StorageV2",
    "az vm start --name vm-web01 --resource-group rg-web",
    "az vm list --resource-group rg-web",
    "az vm resize --name vm-web01 --size Standard_B2s",
    "az group delete --name rg-old --no-wait",
]

def parse(cmd):
    words = cmd.split()[1:]
    command, options, i = [], {}, 0
    while i < len(words):
        if words[i].startswith("--"):
            has_value = i + 1 < len(words) and not words[i + 1].startswith("--")
            options[words[i]] = words[i + 1] if has_value else None
            i += 2 if has_value else 1
        else:
            command.append(words[i])
            i += 1
    return " ".join(command), options

def to_powershell(cmd):
    command, options = parse(cmd)
    if command not in MAP:
        return f"# no mapping for: az {command}"
    parts = [MAP[command]]
    for opt, value in options.items():
        if opt not in PARAMS:
            return f"# no mapping for: {opt} (in {MAP[command]})"
        parts.append(PARAMS[opt] + (f" {value}" if value else ""))
    return " ".join(parts)

for line in runbook:
    print(line)
    print("  " + to_powershell(line))

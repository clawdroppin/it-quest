@@lesson az9-governance-lab Azure Governance Audit
@@text
A real first task for a junior cloud admin: *"Our Azure bill doubled and nobody knows who owns half of it. Find out."* You'll build **AzAudit**, a PowerShell tool that checks every resource against the company's rules and estimates the monthly cost per cost centre:

| Rule | Why it matters |
|---|---|
| **location**: only Japan East / Japan West | data residency, latency, and the company's DR plan |
| **tags**: `env`, `owner`, `costcenter` | no owner = nobody to ask; no cost centre = nobody pays |
| **public-ip** | every public IP is an attack surface; use **Bastion** for admin access |
| **public-blob** | anonymous blob access is how data leaks make the news |
| **orphan-disk** | deleting a VM leaves its disks, and they keep billing |
| **open-mgmt-port**: RDP/SSH from the internet | the most attacked ports on earth |

It runs **offline** on `inventory.json` (a sanitised export of a messy subscription), so you can test it without an account, and **live** with `-Live`, which reads your subscription with `Get-AzResource` and reshapes the results into the same simple form. That split again: the logic is plain functions on plain objects, and only the edge touches the cloud.

The lab then deploys `main.bicep`, a small **infrastructure-as-code** template that follows all the rules, breaks some on purpose, catches them with your audit, and finally lets **Azure Policy**, **locks** and **budgets** enforce them for you. That's the AZ-104 governance domain, done by hand.
@@project az_audit Azure Governance Audit
Write `Get-AuditFinding`, `Get-CostByTag` and `Format-AuditReport` in `AzAudit.ps1` until `tests.ps1` passes, then run `.\Invoke-Audit.ps1` on the sample inventory. Then follow `LAB.md` in a real (free) subscription, and **delete the resource group at the end**.
@@file AzAudit.ps1
# AzAudit.ps1: governance checks for an Azure resource inventory.
# Works offline on an exported inventory (inventory.json) or live on Get-AzResource output.
# Each resource looks like: Name, ResourceGroupName, ResourceType, Location, Tags, Sku, Properties

function Get-TagValue($Resource, [string]$Tag) {
    # Azure tag names are case-insensitive. Tags may be $null, a hashtable or a PSCustomObject.
    if ($null -eq $Resource.Tags) { return $null }
    if ($Resource.Tags -is [System.Collections.IDictionary]) {
        foreach ($k in $Resource.Tags.Keys) { if ($k -eq $Tag) { return $Resource.Tags[$k] } }
        return $null
    }
    $p = $Resource.Tags.PSObject.Properties | Where-Object { $_.Name -eq $Tag } | Select-Object -First 1
    if ($p) { return $p.Value }
    return $null
}

function Get-AuditFinding {
    param(
        [Parameter(Mandatory)][object[]]$Resources,
        [string[]]$AllowedLocations = @('japaneast', 'japanwest'),
        [string[]]$RequiredTags = @('env', 'owner', 'costcenter')
    )
    # For each resource, output [pscustomobject]@{ Resource; ResourceGroup; Rule; Detail } per problem,
    # in this order:
    #   location       "<location> is not an allowed region"   (skip Location 'global')
    #   tags           "missing: env, owner"                    (RequiredTags order; empty value = missing)
    #   public-ip      any Microsoft.Network/publicIPAddresses
    #   public-blob    storage account with Properties.allowBlobPublicAccess = $true
    #   orphan-disk    Microsoft.Compute/disks with Properties.diskState 'Unattached'
    #   open-mgmt-port one per NSG rule in Properties.securityRules that is Allow + Inbound,
    #                  source '*', 'Internet' or '0.0.0.0/0', port '22', '3389' or '*'
    #                  Detail: "rule '<name>' allows port <port> from the internet"
    throw 'TODO: not done yet'
}

function Get-CostByTag {
    param(
        [Parameter(Mandatory)][object[]]$Resources,
        [Parameter(Mandatory)]$Prices,
        [string]$Tag = 'costcenter'
    )
    # Price lookup: try "<ResourceType>|<Sku>", then "<ResourceType>", else 0.
    # Group by the tag's value ('(untagged)' if missing or empty).
    # Output [pscustomobject]@{ Group; Monthly }, biggest Monthly first, ties by Group name.
    throw 'TODO: not done yet'
}

function Format-AuditReport {
    param([object[]]$Findings = @())
    # No findings: 'No findings. Nice and tidy!'
    # Otherwise, for each rule (sorted by name): "[rule]", then "  <resource>: <detail>" lines,
    # and a final line "<n> finding(s) on <m> resource(s)". Join with "`n".
    throw 'TODO: not done yet'
}
@@file inventory.json
[
  {
    "Name": "vm-web01", "ResourceGroupName": "rg-web-prod", "ResourceType": "Microsoft.Compute/virtualMachines",
    "Location": "japaneast", "Sku": "Standard_B2s",
    "Tags": { "env": "prod", "owner": "ytanaka", "costcenter": "web" }, "Properties": {}
  },
  {
    "Name": "vm-web01-osdisk", "ResourceGroupName": "rg-web-prod", "ResourceType": "Microsoft.Compute/disks",
    "Location": "japaneast", "Sku": "StandardSSD_LRS",
    "Tags": { "env": "prod", "owner": "ytanaka", "costcenter": "web" }, "Properties": { "diskState": "Attached" }
  },
  {
    "Name": "pip-web01", "ResourceGroupName": "rg-web-prod", "ResourceType": "Microsoft.Network/publicIPAddresses",
    "Location": "japaneast", "Sku": "Standard",
    "Tags": { "env": "prod", "owner": "ytanaka", "costcenter": "web" }, "Properties": {}
  },
  {
    "Name": "nsg-web", "ResourceGroupName": "rg-web-prod", "ResourceType": "Microsoft.Network/networkSecurityGroups",
    "Location": "japaneast", "Sku": null,
    "Tags": { "env": "prod", "owner": "ytanaka", "costcenter": "web" },
    "Properties": { "securityRules": [
      { "name": "allow-https", "access": "Allow", "direction": "Inbound", "sourceAddressPrefix": "Internet", "destinationPortRange": "443" },
      { "name": "allow-rdp-temp", "access": "Allow", "direction": "Inbound", "sourceAddressPrefix": "*", "destinationPortRange": "3389" },
      { "name": "allow-ssh-office", "access": "Allow", "direction": "Inbound", "sourceAddressPrefix": "203.0.113.0/24", "destinationPortRange": "22" }
    ] }
  },
  {
    "Name": "stwebassets01", "ResourceGroupName": "rg-web-prod", "ResourceType": "Microsoft.Storage/storageAccounts",
    "Location": "japaneast", "Sku": "Standard_LRS",
    "Tags": { "Env": "prod", "Owner": "ytanaka", "CostCenter": "web" }, "Properties": { "allowBlobPublicAccess": true }
  },
  {
    "Name": "vm-test-old", "ResourceGroupName": "rg-sandbox", "ResourceType": "Microsoft.Compute/virtualMachines",
    "Location": "eastus", "Sku": "Standard_D4s_v5",
    "Tags": { "env": "dev" }, "Properties": {}
  },
  {
    "Name": "disk-test-old-data", "ResourceGroupName": "rg-sandbox", "ResourceType": "Microsoft.Compute/disks",
    "Location": "eastus", "Sku": "Premium_LRS",
    "Tags": null, "Properties": { "diskState": "Unattached" }
  },
  {
    "Name": "kv-it-prod", "ResourceGroupName": "rg-shared", "ResourceType": "Microsoft.KeyVault/vaults",
    "Location": "japaneast", "Sku": "standard",
    "Tags": { "env": "prod", "owner": "itops", "costcenter": "it" }, "Properties": {}
  },
  {
    "Name": "stbackupit01", "ResourceGroupName": "rg-shared", "ResourceType": "Microsoft.Storage/storageAccounts",
    "Location": "japanwest", "Sku": "Standard_GRS",
    "Tags": { "env": "prod", "owner": "itops", "costcenter": "it" }, "Properties": { "allowBlobPublicAccess": false }
  },
  {
    "Name": "dns-example-jp", "ResourceGroupName": "rg-shared", "ResourceType": "Microsoft.Network/dnszones",
    "Location": "global", "Sku": null,
    "Tags": { "env": "prod", "owner": "itops", "costcenter": "" }, "Properties": {}
  }
]
@@file prices.json
{
  "_note": "Rough monthly prices in JPY for the exercise only. Use the Azure pricing calculator for real numbers.",
  "Microsoft.Compute/virtualMachines|Standard_B2s": 6200,
  "Microsoft.Compute/virtualMachines|Standard_D4s_v5": 30500,
  "Microsoft.Compute/disks|StandardSSD_LRS": 1500,
  "Microsoft.Compute/disks|Premium_LRS": 3200,
  "Microsoft.Network/publicIPAddresses": 600,
  "Microsoft.Storage/storageAccounts|Standard_LRS": 450,
  "Microsoft.Storage/storageAccounts|Standard_GRS": 900,
  "Microsoft.KeyVault/vaults": 150,
  "Microsoft.Network/dnszones": 80
}
@@file tests.ps1
# Run: powershell -ExecutionPolicy Bypass -File tests.ps1
. "$PSScriptRoot\AzAudit.ps1"
$script:failed = 0

function Test-That([string]$Name, [scriptblock]$Body) {
    try {
        $result = & $Body
        if ($result -eq $true) { Write-Host "  ok    $Name" -ForegroundColor Green }
        else { Write-Host "  FAIL  $Name" -ForegroundColor Red; $script:failed++ }
    }
    catch {
        Write-Host "  FAIL  $Name : $($_.Exception.Message)" -ForegroundColor Red
        $script:failed++
    }
}

$inv = Get-Content -Raw "$PSScriptRoot\inventory.json" | ConvertFrom-Json
$prices = Get-Content -Raw "$PSScriptRoot\prices.json" | ConvertFrom-Json
$all = @(Get-AuditFinding -Resources $inv)
function Rules($name) { @($all | Where-Object Resource -eq $name | ForEach-Object Rule) -join ',' }

Write-Host 'Findings'
Test-That 'clean VM has no findings' { (Rules 'vm-web01') -eq '' }
Test-That 'public IP flagged' { (Rules 'pip-web01') -eq 'public-ip' }
Test-That 'RDP from anywhere flagged, office SSH and HTTPS are fine' {
    $f = @($all | Where-Object Resource -eq 'nsg-web')
    $f.Count -eq 1 -and $f[0].Rule -eq 'open-mgmt-port' -and $f[0].Detail -like "*'allow-rdp-temp'*3389*"
}
Test-That 'tag names are case-insensitive; public blob flagged' { (Rules 'stwebassets01') -eq 'public-blob' }
Test-That 'wrong region and missing tags, in order' {
    $f = @($all | Where-Object Resource -eq 'vm-test-old')
    (Rules 'vm-test-old') -eq 'location,tags' -and $f[1].Detail -eq 'missing: owner, costcenter'
}
Test-That 'null tags and orphan disk' {
    (Rules 'disk-test-old-data') -eq 'location,tags,orphan-disk' -and
        @($all | Where-Object Resource -eq 'disk-test-old-data')[1].Detail -eq 'missing: env, owner, costcenter'
}
Test-That 'global resources skip the region check; empty tag counts as missing' { (Rules 'dns-example-jp') -eq 'tags' }
Test-That 'findings carry the resource group' { @($all | Where-Object Resource -eq 'pip-web01')[0].ResourceGroup -eq 'rg-web-prod' }
Test-That 'nine findings in total' { $all.Count -eq 9 }
Test-That 'parameters change the rules' {
    @(Get-AuditFinding -Resources $inv -AllowedLocations 'eastus', 'japaneast', 'japanwest' -RequiredTags 'env' |
        Where-Object Rule -in 'location', 'tags').Count -eq 1
}

Write-Host 'Cost'
$cost = @(Get-CostByTag -Resources $inv -Prices $prices)
Test-That 'three groups' { $cost.Count -eq 3 }
Test-That 'untagged is the biggest spender' { $cost[0].Group -eq '(untagged)' -and $cost[0].Monthly -eq 33780 }
Test-That 'web, then it' { $cost[1].Group -eq 'web' -and $cost[1].Monthly -eq 8750 -and $cost[2].Group -eq 'it' -and $cost[2].Monthly -eq 1050 }
Test-That 'group by another tag' { @(Get-CostByTag -Resources $inv -Prices $prices -Tag env)[0].Group -eq 'dev' }

Write-Host 'Report'
$report = Format-AuditReport -Findings $all
$lines = $report -split "`n"
Test-That 'rules sorted, first is location' { $lines[0] -eq '[location]' }
Test-That 'indented resource lines' { $lines[1] -eq '  vm-test-old: eastus is not an allowed region' }
Test-That 'summary line' { $lines[-1] -eq '9 finding(s) on 6 resource(s)' }
Test-That 'empty report' { (Format-AuditReport -Findings @()) -eq 'No findings. Nice and tidy!' }

if ($script:failed) { Write-Host "$script:failed test(s) failed" -ForegroundColor Red; exit 1 }
Write-Host 'All tests passed' -ForegroundColor Green
@@file Invoke-Audit.ps1
<#
.SYNOPSIS
Audit Azure resources for governance problems and estimate cost per cost centre.
.EXAMPLE
.\Invoke-Audit.ps1                              # offline: inventory.json
.EXAMPLE
.\Invoke-Audit.ps1 -Live -ResourceGroup rg-lab  # real subscription (Connect-AzAccount first)
#>
param(
    [string]$InventoryPath = "$PSScriptRoot\inventory.json",
    [switch]$Live,
    [string]$ResourceGroup,
    [string]$CsvPath
)
. "$PSScriptRoot\AzAudit.ps1"

if ($Live) {
    if (-not (Get-Command Get-AzResource -ErrorAction SilentlyContinue)) {
        throw 'The Az module is missing: Install-Module Az -Scope CurrentUser'
    }
    $params = @{ ExpandProperties = $true }
    if ($ResourceGroup) { $params.ResourceGroupName = $ResourceGroup }
    # Reshape into the same simple form as inventory.json
    $resources = Get-AzResource @params | ForEach-Object {
        [pscustomobject]@{
            Name = $_.Name; ResourceGroupName = $_.ResourceGroupName; ResourceType = $_.ResourceType
            Location = $_.Location; Tags = $_.Tags; Sku = $_.Sku.Name; Properties = $_.Properties
        }
    }
}
else {
    $resources = Get-Content -Raw $InventoryPath | ConvertFrom-Json
}

$findings = @(Get-AuditFinding -Resources $resources)
Write-Host (Format-AuditReport -Findings $findings)
Write-Host ''

$prices = Get-Content -Raw "$PSScriptRoot\prices.json" | ConvertFrom-Json
Write-Host 'Estimated monthly cost by cost centre (JPY):'
Get-CostByTag -Resources $resources -Prices $prices | Format-Table -AutoSize | Out-String | Write-Host

if ($CsvPath) {
    $findings | Export-Csv -Path $CsvPath -NoTypeInformation -Encoding UTF8
    Write-Host "Findings saved to $CsvPath"
}
if ($findings.Count) { exit 2 }   # non-zero, so a pipeline or scheduled task notices
@@file main.bicep
// main.bicep: a small, well-governed lab: a VNet with an NSG and a locked-down storage account.
// Deploy:  New-AzResourceGroupDeployment -ResourceGroupName rg-lab -TemplateFile main.bicep -owner <you>
//     or:  az deployment group create -g rg-lab -f main.bicep -p owner=<you>

@description('Who to ask about these resources')
param owner string

@allowed([ 'japaneast', 'japanwest' ])
param location string = 'japaneast'

@description('Your public IP range allowed to SSH, e.g. 203.0.113.10/32')
param adminSourcePrefix string = '203.0.113.10/32'

var tags = {
  env: 'lab'
  owner: owner
  costcenter: 'it'
}

// Storage names must be globally unique, 3-24 lowercase letters and digits
var storageName = 'stlab${uniqueString(resourceGroup().id)}'

resource nsg 'Microsoft.Network/networkSecurityGroups@2023-11-01' = {
  name: 'nsg-lab'
  location: location
  tags: tags
  properties: {
    securityRules: [
      {
        name: 'allow-ssh-admin'
        properties: {
          priority: 100
          access: 'Allow'
          direction: 'Inbound'
          protocol: 'Tcp'
          sourceAddressPrefix: adminSourcePrefix
          sourcePortRange: '*'
          destinationAddressPrefix: '*'
          destinationPortRange: '22'
        }
      }
    ]
  }
}

resource vnet 'Microsoft.Network/virtualNetworks@2023-11-01' = {
  name: 'vnet-lab'
  location: location
  tags: tags
  properties: {
    addressSpace: { addressPrefixes: [ '10.20.0.0/16' ] }
    subnets: [
      {
        name: 'snet-servers'
        properties: {
          addressPrefix: '10.20.1.0/24'
          networkSecurityGroup: { id: nsg.id }
        }
      }
    ]
  }
}

resource storage 'Microsoft.Storage/storageAccounts@2023-05-01' = {
  name: storageName
  location: location
  tags: tags
  sku: { name: 'Standard_LRS' }
  kind: 'StorageV2'
  properties: {
    accessTier: 'Cool'
    allowBlobPublicAccess: false
    minimumTlsVersion: 'TLS1_2'
    supportsHttpsTrafficOnly: true
  }
}

output storageAccountName string = storage.name
output subnetId string = vnet.properties.subnets[0].id
@@file LAB.md
# Azure Governance Lab

Everything here fits in the free account credit. **Do step 8 when you finish.**

## 0. Offline first
```
powershell -ExecutionPolicy Bypass -File tests.ps1
.\Invoke-Audit.ps1
```
Read every finding in the sample inventory and decide how you would fix it.

## 1. Sign in
Open **Cloud Shell** (PowerShell) in the portal, or locally:
```
Install-Module Az -Scope CurrentUser
Connect-AzAccount
```

## 2. A resource group with tags
```
New-AzResourceGroup -Name rg-lab -Location japaneast -Tag @{ env='lab'; owner='<you>'; costcenter='it' }
```

## 3. Deploy the Bicep template
Upload `main.bicep` (Cloud Shell: Manage files → Upload), then:
```
New-AzResourceGroupDeployment -ResourceGroupName rg-lab -TemplateFile main.bicep -owner <you> -adminSourcePrefix <your IP>/32
```
Run it a second time: nothing changes. That's **idempotent** infrastructure as code.

## 4. Break the rules on purpose
In the portal, create a public IP called `pip-oops` in rg-lab **without tags**, and add an NSG rule
`allow-rdp-any` (Inbound, source Any, port 3389) to `nsg-lab`.

## 5. Audit for real
Upload the project files, then:
```
.\Invoke-Audit.ps1 -Live -ResourceGroup rg-lab -CsvPath findings.csv
```
You should see `pip-oops` (public-ip, tags) and `allow-rdp-any` (open-mgmt-port).

## 6. Let Azure enforce it
- Policy → Definitions → **Require a tag on resources** → Assign to rg-lab with tag `owner`.
  Try creating an untagged public IP again: it's denied.
- Policy → **Allowed locations** → japaneast, japanwest.
- rg-lab → Locks → add a **CanNotDelete** lock. Try deleting the storage account. Then remove the lock.
- Cost Management → Budgets → a ¥1,000 monthly budget with an email alert at 80%.

## 7. RBAC
rg-lab → Access control (IAM) → Add role assignment → **Reader** for a test user (or a friend as a B2B guest).
Check what they can and can't do. "Check access" shows effective permissions.

## 8. Clean up
```
Remove-AzResourceGroup -Name rg-lab -Force
```
Also remove the policy assignments and budget if you created them at subscription scope.

## For your portfolio
Screenshots of the audit output before and after, the policy denying an untagged resource,
and the Bicep file. Explain in one paragraph *why* each rule matters (cost, security, ownership).
@@text
> Tips: `$found += , @('rule', 'detail')` adds a pair (the comma stops PowerShell flattening it). `-notcontains` checks membership in an array. `switch ($r.ResourceType) { 'Microsoft.Compute/disks' { ... } }` reads nicely. For cost, an `[ordered]@{}` hashtable keeps insertion order before you sort. `Group-Object Rule | Sort-Object Name` groups the findings for the report.
@@project az_audit_finished Azure Governance Audit: the finished version
The complete tool. Next steps: run it every morning with an Azure Automation runbook (managed identity + Reader role), post the report to Teams, add a rule for VMs without backup, or a `-Fix` switch that adds missing tags after asking.
@@file AzAudit.ps1
# AzAudit.ps1: governance checks for an Azure resource inventory.
# Works offline on an exported inventory (inventory.json) or live on Get-AzResource output.
# Each resource looks like: Name, ResourceGroupName, ResourceType, Location, Tags, Sku, Properties

function Get-TagValue($Resource, [string]$Tag) {
    # Azure tag names are case-insensitive. Tags may be $null, a hashtable or a PSCustomObject.
    if ($null -eq $Resource.Tags) { return $null }
    if ($Resource.Tags -is [System.Collections.IDictionary]) {
        foreach ($k in $Resource.Tags.Keys) { if ($k -eq $Tag) { return $Resource.Tags[$k] } }
        return $null
    }
    $p = $Resource.Tags.PSObject.Properties | Where-Object { $_.Name -eq $Tag } | Select-Object -First 1
    if ($p) { return $p.Value }
    return $null
}

function Get-AuditFinding {
    param(
        [Parameter(Mandatory)][object[]]$Resources,
        [string[]]$AllowedLocations = @('japaneast', 'japanwest'),
        [string[]]$RequiredTags = @('env', 'owner', 'costcenter')
    )
    foreach ($r in $Resources) {
        $found = @()

        if ($r.Location -and $r.Location -ne 'global' -and $AllowedLocations -notcontains $r.Location) {
            $found += , @('location', "$($r.Location) is not an allowed region")
        }

        $missing = @($RequiredTags | Where-Object { [string]::IsNullOrWhiteSpace((Get-TagValue $r $_)) })
        if ($missing.Count) { $found += , @('tags', "missing: $($missing -join ', ')") }

        switch ($r.ResourceType) {
            'Microsoft.Network/publicIPAddresses' {
                $found += , @('public-ip', 'public IP address: is it needed? (Bastion avoids them for admin access)')
            }
            'Microsoft.Storage/storageAccounts' {
                if ($r.Properties.allowBlobPublicAccess -eq $true) {
                    $found += , @('public-blob', 'anonymous blob access is allowed')
                }
            }
            'Microsoft.Compute/disks' {
                if ($r.Properties.diskState -eq 'Unattached') {
                    $found += , @('orphan-disk', 'unattached disk still costs money')
                }
            }
            'Microsoft.Network/networkSecurityGroups' {
                foreach ($rule in @($r.Properties.securityRules)) {
                    if ($null -eq $rule) { continue }
                    $open = $rule.access -eq 'Allow' -and $rule.direction -eq 'Inbound' -and
                        @('*', 'Internet', '0.0.0.0/0') -contains $rule.sourceAddressPrefix
                    if ($open -and @('22', '3389', '*') -contains [string]$rule.destinationPortRange) {
                        $found += , @('open-mgmt-port', "rule '$($rule.name)' allows port $($rule.destinationPortRange) from the internet")
                    }
                }
            }
        }

        foreach ($f in $found) {
            [pscustomobject]@{ Resource = $r.Name; ResourceGroup = $r.ResourceGroupName; Rule = $f[0]; Detail = $f[1] }
        }
    }
}

function Get-CostByTag {
    param(
        [Parameter(Mandatory)][object[]]$Resources,
        [Parameter(Mandatory)]$Prices,
        [string]$Tag = 'costcenter'
    )
    # Prices keys are "ResourceType|Sku" or just "ResourceType"; values are monthly JPY.
    $totals = [ordered]@{}
    foreach ($r in $Resources) {
        $price = 0
        foreach ($key in @("$($r.ResourceType)|$($r.Sku)", $r.ResourceType)) {
            $p = $Prices.PSObject.Properties | Where-Object { $_.Name -eq $key } | Select-Object -First 1
            if ($p) { $price = [double]$p.Value; break }
        }
        $group = Get-TagValue $r $Tag
        if ([string]::IsNullOrWhiteSpace($group)) { $group = '(untagged)' }
        if (-not $totals.Contains($group)) { $totals[$group] = 0 }
        $totals[$group] += $price
    }
    $totals.GetEnumerator() |
        ForEach-Object { [pscustomobject]@{ Group = $_.Key; Monthly = [math]::Round($_.Value, 2) } } |
        Sort-Object -Property @{ Expression = 'Monthly'; Descending = $true }, @{ Expression = 'Group'; Descending = $false }
}

function Format-AuditReport {
    param([object[]]$Findings = @())
    $Findings = @($Findings | Where-Object { $_ })
    if ($Findings.Count -eq 0) { return 'No findings. Nice and tidy!' }
    $lines = foreach ($group in ($Findings | Group-Object Rule | Sort-Object Name)) {
        "[$($group.Name)]"
        foreach ($f in $group.Group) { "  $($f.Resource): $($f.Detail)" }
    }
    $resources = @($Findings | Select-Object -ExpandProperty Resource -Unique).Count
    (@($lines) + "$($Findings.Count) finding(s) on $resources resource(s)") -join "`n"
}
@@file inventory.json
[
  {
    "Name": "vm-web01", "ResourceGroupName": "rg-web-prod", "ResourceType": "Microsoft.Compute/virtualMachines",
    "Location": "japaneast", "Sku": "Standard_B2s",
    "Tags": { "env": "prod", "owner": "ytanaka", "costcenter": "web" }, "Properties": {}
  },
  {
    "Name": "vm-web01-osdisk", "ResourceGroupName": "rg-web-prod", "ResourceType": "Microsoft.Compute/disks",
    "Location": "japaneast", "Sku": "StandardSSD_LRS",
    "Tags": { "env": "prod", "owner": "ytanaka", "costcenter": "web" }, "Properties": { "diskState": "Attached" }
  },
  {
    "Name": "pip-web01", "ResourceGroupName": "rg-web-prod", "ResourceType": "Microsoft.Network/publicIPAddresses",
    "Location": "japaneast", "Sku": "Standard",
    "Tags": { "env": "prod", "owner": "ytanaka", "costcenter": "web" }, "Properties": {}
  },
  {
    "Name": "nsg-web", "ResourceGroupName": "rg-web-prod", "ResourceType": "Microsoft.Network/networkSecurityGroups",
    "Location": "japaneast", "Sku": null,
    "Tags": { "env": "prod", "owner": "ytanaka", "costcenter": "web" },
    "Properties": { "securityRules": [
      { "name": "allow-https", "access": "Allow", "direction": "Inbound", "sourceAddressPrefix": "Internet", "destinationPortRange": "443" },
      { "name": "allow-rdp-temp", "access": "Allow", "direction": "Inbound", "sourceAddressPrefix": "*", "destinationPortRange": "3389" },
      { "name": "allow-ssh-office", "access": "Allow", "direction": "Inbound", "sourceAddressPrefix": "203.0.113.0/24", "destinationPortRange": "22" }
    ] }
  },
  {
    "Name": "stwebassets01", "ResourceGroupName": "rg-web-prod", "ResourceType": "Microsoft.Storage/storageAccounts",
    "Location": "japaneast", "Sku": "Standard_LRS",
    "Tags": { "Env": "prod", "Owner": "ytanaka", "CostCenter": "web" }, "Properties": { "allowBlobPublicAccess": true }
  },
  {
    "Name": "vm-test-old", "ResourceGroupName": "rg-sandbox", "ResourceType": "Microsoft.Compute/virtualMachines",
    "Location": "eastus", "Sku": "Standard_D4s_v5",
    "Tags": { "env": "dev" }, "Properties": {}
  },
  {
    "Name": "disk-test-old-data", "ResourceGroupName": "rg-sandbox", "ResourceType": "Microsoft.Compute/disks",
    "Location": "eastus", "Sku": "Premium_LRS",
    "Tags": null, "Properties": { "diskState": "Unattached" }
  },
  {
    "Name": "kv-it-prod", "ResourceGroupName": "rg-shared", "ResourceType": "Microsoft.KeyVault/vaults",
    "Location": "japaneast", "Sku": "standard",
    "Tags": { "env": "prod", "owner": "itops", "costcenter": "it" }, "Properties": {}
  },
  {
    "Name": "stbackupit01", "ResourceGroupName": "rg-shared", "ResourceType": "Microsoft.Storage/storageAccounts",
    "Location": "japanwest", "Sku": "Standard_GRS",
    "Tags": { "env": "prod", "owner": "itops", "costcenter": "it" }, "Properties": { "allowBlobPublicAccess": false }
  },
  {
    "Name": "dns-example-jp", "ResourceGroupName": "rg-shared", "ResourceType": "Microsoft.Network/dnszones",
    "Location": "global", "Sku": null,
    "Tags": { "env": "prod", "owner": "itops", "costcenter": "" }, "Properties": {}
  }
]
@@file prices.json
{
  "_note": "Rough monthly prices in JPY for the exercise only. Use the Azure pricing calculator for real numbers.",
  "Microsoft.Compute/virtualMachines|Standard_B2s": 6200,
  "Microsoft.Compute/virtualMachines|Standard_D4s_v5": 30500,
  "Microsoft.Compute/disks|StandardSSD_LRS": 1500,
  "Microsoft.Compute/disks|Premium_LRS": 3200,
  "Microsoft.Network/publicIPAddresses": 600,
  "Microsoft.Storage/storageAccounts|Standard_LRS": 450,
  "Microsoft.Storage/storageAccounts|Standard_GRS": 900,
  "Microsoft.KeyVault/vaults": 150,
  "Microsoft.Network/dnszones": 80
}
@@file tests.ps1
# Run: powershell -ExecutionPolicy Bypass -File tests.ps1
. "$PSScriptRoot\AzAudit.ps1"
$script:failed = 0

function Test-That([string]$Name, [scriptblock]$Body) {
    try {
        $result = & $Body
        if ($result -eq $true) { Write-Host "  ok    $Name" -ForegroundColor Green }
        else { Write-Host "  FAIL  $Name" -ForegroundColor Red; $script:failed++ }
    }
    catch {
        Write-Host "  FAIL  $Name : $($_.Exception.Message)" -ForegroundColor Red
        $script:failed++
    }
}

$inv = Get-Content -Raw "$PSScriptRoot\inventory.json" | ConvertFrom-Json
$prices = Get-Content -Raw "$PSScriptRoot\prices.json" | ConvertFrom-Json
$all = @(Get-AuditFinding -Resources $inv)
function Rules($name) { @($all | Where-Object Resource -eq $name | ForEach-Object Rule) -join ',' }

Write-Host 'Findings'
Test-That 'clean VM has no findings' { (Rules 'vm-web01') -eq '' }
Test-That 'public IP flagged' { (Rules 'pip-web01') -eq 'public-ip' }
Test-That 'RDP from anywhere flagged, office SSH and HTTPS are fine' {
    $f = @($all | Where-Object Resource -eq 'nsg-web')
    $f.Count -eq 1 -and $f[0].Rule -eq 'open-mgmt-port' -and $f[0].Detail -like "*'allow-rdp-temp'*3389*"
}
Test-That 'tag names are case-insensitive; public blob flagged' { (Rules 'stwebassets01') -eq 'public-blob' }
Test-That 'wrong region and missing tags, in order' {
    $f = @($all | Where-Object Resource -eq 'vm-test-old')
    (Rules 'vm-test-old') -eq 'location,tags' -and $f[1].Detail -eq 'missing: owner, costcenter'
}
Test-That 'null tags and orphan disk' {
    (Rules 'disk-test-old-data') -eq 'location,tags,orphan-disk' -and
        @($all | Where-Object Resource -eq 'disk-test-old-data')[1].Detail -eq 'missing: env, owner, costcenter'
}
Test-That 'global resources skip the region check; empty tag counts as missing' { (Rules 'dns-example-jp') -eq 'tags' }
Test-That 'findings carry the resource group' { @($all | Where-Object Resource -eq 'pip-web01')[0].ResourceGroup -eq 'rg-web-prod' }
Test-That 'nine findings in total' { $all.Count -eq 9 }
Test-That 'parameters change the rules' {
    @(Get-AuditFinding -Resources $inv -AllowedLocations 'eastus', 'japaneast', 'japanwest' -RequiredTags 'env' |
        Where-Object Rule -in 'location', 'tags').Count -eq 1
}

Write-Host 'Cost'
$cost = @(Get-CostByTag -Resources $inv -Prices $prices)
Test-That 'three groups' { $cost.Count -eq 3 }
Test-That 'untagged is the biggest spender' { $cost[0].Group -eq '(untagged)' -and $cost[0].Monthly -eq 33780 }
Test-That 'web, then it' { $cost[1].Group -eq 'web' -and $cost[1].Monthly -eq 8750 -and $cost[2].Group -eq 'it' -and $cost[2].Monthly -eq 1050 }
Test-That 'group by another tag' { @(Get-CostByTag -Resources $inv -Prices $prices -Tag env)[0].Group -eq 'dev' }

Write-Host 'Report'
$report = Format-AuditReport -Findings $all
$lines = $report -split "`n"
Test-That 'rules sorted, first is location' { $lines[0] -eq '[location]' }
Test-That 'indented resource lines' { $lines[1] -eq '  vm-test-old: eastus is not an allowed region' }
Test-That 'summary line' { $lines[-1] -eq '9 finding(s) on 6 resource(s)' }
Test-That 'empty report' { (Format-AuditReport -Findings @()) -eq 'No findings. Nice and tidy!' }

if ($script:failed) { Write-Host "$script:failed test(s) failed" -ForegroundColor Red; exit 1 }
Write-Host 'All tests passed' -ForegroundColor Green
@@file Invoke-Audit.ps1
<#
.SYNOPSIS
Audit Azure resources for governance problems and estimate cost per cost centre.
.EXAMPLE
.\Invoke-Audit.ps1                              # offline: inventory.json
.EXAMPLE
.\Invoke-Audit.ps1 -Live -ResourceGroup rg-lab  # real subscription (Connect-AzAccount first)
#>
param(
    [string]$InventoryPath = "$PSScriptRoot\inventory.json",
    [switch]$Live,
    [string]$ResourceGroup,
    [string]$CsvPath
)
. "$PSScriptRoot\AzAudit.ps1"

if ($Live) {
    if (-not (Get-Command Get-AzResource -ErrorAction SilentlyContinue)) {
        throw 'The Az module is missing: Install-Module Az -Scope CurrentUser'
    }
    $params = @{ ExpandProperties = $true }
    if ($ResourceGroup) { $params.ResourceGroupName = $ResourceGroup }
    # Reshape into the same simple form as inventory.json
    $resources = Get-AzResource @params | ForEach-Object {
        [pscustomobject]@{
            Name = $_.Name; ResourceGroupName = $_.ResourceGroupName; ResourceType = $_.ResourceType
            Location = $_.Location; Tags = $_.Tags; Sku = $_.Sku.Name; Properties = $_.Properties
        }
    }
}
else {
    $resources = Get-Content -Raw $InventoryPath | ConvertFrom-Json
}

$findings = @(Get-AuditFinding -Resources $resources)
Write-Host (Format-AuditReport -Findings $findings)
Write-Host ''

$prices = Get-Content -Raw "$PSScriptRoot\prices.json" | ConvertFrom-Json
Write-Host 'Estimated monthly cost by cost centre (JPY):'
Get-CostByTag -Resources $resources -Prices $prices | Format-Table -AutoSize | Out-String | Write-Host

if ($CsvPath) {
    $findings | Export-Csv -Path $CsvPath -NoTypeInformation -Encoding UTF8
    Write-Host "Findings saved to $CsvPath"
}
if ($findings.Count) { exit 2 }   # non-zero, so a pipeline or scheduled task notices
@@file main.bicep
// main.bicep: a small, well-governed lab: a VNet with an NSG and a locked-down storage account.
// Deploy:  New-AzResourceGroupDeployment -ResourceGroupName rg-lab -TemplateFile main.bicep -owner <you>
//     or:  az deployment group create -g rg-lab -f main.bicep -p owner=<you>

@description('Who to ask about these resources')
param owner string

@allowed([ 'japaneast', 'japanwest' ])
param location string = 'japaneast'

@description('Your public IP range allowed to SSH, e.g. 203.0.113.10/32')
param adminSourcePrefix string = '203.0.113.10/32'

var tags = {
  env: 'lab'
  owner: owner
  costcenter: 'it'
}

// Storage names must be globally unique, 3-24 lowercase letters and digits
var storageName = 'stlab${uniqueString(resourceGroup().id)}'

resource nsg 'Microsoft.Network/networkSecurityGroups@2023-11-01' = {
  name: 'nsg-lab'
  location: location
  tags: tags
  properties: {
    securityRules: [
      {
        name: 'allow-ssh-admin'
        properties: {
          priority: 100
          access: 'Allow'
          direction: 'Inbound'
          protocol: 'Tcp'
          sourceAddressPrefix: adminSourcePrefix
          sourcePortRange: '*'
          destinationAddressPrefix: '*'
          destinationPortRange: '22'
        }
      }
    ]
  }
}

resource vnet 'Microsoft.Network/virtualNetworks@2023-11-01' = {
  name: 'vnet-lab'
  location: location
  tags: tags
  properties: {
    addressSpace: { addressPrefixes: [ '10.20.0.0/16' ] }
    subnets: [
      {
        name: 'snet-servers'
        properties: {
          addressPrefix: '10.20.1.0/24'
          networkSecurityGroup: { id: nsg.id }
        }
      }
    ]
  }
}

resource storage 'Microsoft.Storage/storageAccounts@2023-05-01' = {
  name: storageName
  location: location
  tags: tags
  sku: { name: 'Standard_LRS' }
  kind: 'StorageV2'
  properties: {
    accessTier: 'Cool'
    allowBlobPublicAccess: false
    minimumTlsVersion: 'TLS1_2'
    supportsHttpsTrafficOnly: true
  }
}

output storageAccountName string = storage.name
output subnetId string = vnet.properties.subnets[0].id
@@file LAB.md
# Azure Governance Lab

Everything here fits in the free account credit. **Do step 8 when you finish.**

## 0. Offline first
```
powershell -ExecutionPolicy Bypass -File tests.ps1
.\Invoke-Audit.ps1
```
Read every finding in the sample inventory and decide how you would fix it.

## 1. Sign in
Open **Cloud Shell** (PowerShell) in the portal, or locally:
```
Install-Module Az -Scope CurrentUser
Connect-AzAccount
```

## 2. A resource group with tags
```
New-AzResourceGroup -Name rg-lab -Location japaneast -Tag @{ env='lab'; owner='<you>'; costcenter='it' }
```

## 3. Deploy the Bicep template
Upload `main.bicep` (Cloud Shell: Manage files → Upload), then:
```
New-AzResourceGroupDeployment -ResourceGroupName rg-lab -TemplateFile main.bicep -owner <you> -adminSourcePrefix <your IP>/32
```
Run it a second time: nothing changes. That's **idempotent** infrastructure as code.

## 4. Break the rules on purpose
In the portal, create a public IP called `pip-oops` in rg-lab **without tags**, and add an NSG rule
`allow-rdp-any` (Inbound, source Any, port 3389) to `nsg-lab`.

## 5. Audit for real
Upload the project files, then:
```
.\Invoke-Audit.ps1 -Live -ResourceGroup rg-lab -CsvPath findings.csv
```
You should see `pip-oops` (public-ip, tags) and `allow-rdp-any` (open-mgmt-port).

## 6. Let Azure enforce it
- Policy → Definitions → **Require a tag on resources** → Assign to rg-lab with tag `owner`.
  Try creating an untagged public IP again: it's denied.
- Policy → **Allowed locations** → japaneast, japanwest.
- rg-lab → Locks → add a **CanNotDelete** lock. Try deleting the storage account. Then remove the lock.
- Cost Management → Budgets → a ¥1,000 monthly budget with an email alert at 80%.

## 7. RBAC
rg-lab → Access control (IAM) → Add role assignment → **Reader** for a test user (or a friend as a B2B guest).
Check what they can and can't do. "Check access" shows effective permissions.

## 8. Clean up
```
Remove-AzResourceGroup -Name rg-lab -Force
```
Also remove the policy assignments and budget if you created them at subscription scope.

## For your portfolio
Screenshots of the audit output before and after, the policy denying an untagged resource,
and the Bicep file. Explain in one paragraph *why* each rule matters (cost, security, ownership).
@@text
🎉 You can now explain RBAC, Policy, tags, locks, budgets, NSGs and Bicep with something you built. In an interview, *"I wrote a script that found ¥33,780 a month of untagged spend"* is worth more than any certificate line.

@@lesson xp-naming_audit Azure Naming and Tagging Audit
@@text
Export your resources with `az resource list -o json > resources.json` (or use the sample) and check every name against a naming convention like `rg-shop-prod-jpe-001` (type, workload, environment, region, instance) and every resource for required tags (Environment, Owner, CostCenter). Produce a list of fixes.

**You'll practise:** Azure resource types · Cloud Adoption Framework naming · tags and Azure Policy · PowerShell JSON handling

## Milestones

1. **Prefixes**: map resource types to CAF abbreviations (rg, vnet, vm, st, kv, app...).
2. **Parse**: split names into parts and check each against allowed values.
3. **Tags**: report missing required tags per resource.
4. **Report**: a table and a CSV of resources to fix.
5. **Prevent**: write the Azure Policy definition (JSON) that would deny untagged resources.

## You're done when

- Every sample resource is classified correctly, and you've written the matching Azure Policy.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"What are the CAF abbreviations for storage accounts and key vaults, and why do storage names have no dashes?"*
- *"Explain the difference between Audit and Deny effects in Azure Policy."*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project naming_audit Azure Naming and Tagging Audit
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Azure Naming and Tagging Audit

Export your resources with `az resource list -o json > resources.json` (or use the sample) and check every name against a naming convention like `rg-shop-prod-jpe-001` (type, workload, environment, region, instance) and every resource for required tags (Environment, Owner, CostCenter). Produce a list of fixes.

## Milestones

- [ ] 1. Prefixes: map resource types to CAF abbreviations (rg, vnet, vm, st, kv, app...).
- [ ] 2. Parse: split names into parts and check each against allowed values.
- [ ] 3. Tags: report missing required tags per resource.
- [ ] 4. Report: a table and a CSV of resources to fix.
- [ ] 5. Prevent: write the Azure Policy definition (JSON) that would deny untagged resources.

## Done when

- [ ] Every sample resource is classified correctly, and you've written the matching Azure Policy.
@@file Test-AzureNaming.ps1
param([string]$Path = "resources.json")

$prefixes = @{
    "Microsoft.Resources/resourceGroups"   = "rg"
    "Microsoft.Network/virtualNetworks"    = "vnet"
    "Microsoft.Compute/virtualMachines"    = "vm"
    "Microsoft.KeyVault/vaults"            = "kv"
    "Microsoft.Web/sites"                  = "app"
}
$requiredTags = "Environment", "Owner", "CostCenter"

$resources = Get-Content $Path -Raw | ConvertFrom-Json
foreach ($r in $resources) {
    # TODO: check the name pattern  <prefix>-<workload>-<env>-<region>-<nnn>
    # TODO: check required tags
    Write-Output "$($r.type)  $($r.name)"
}
@@file resources.json
[
  { "name": "rg-shop-prod-jpe-001", "type": "Microsoft.Resources/resourceGroups", "tags": { "Environment": "prod", "Owner": "ann", "CostCenter": "4100" } },
  { "name": "vnet-shop-prod-jpe-001", "type": "Microsoft.Network/virtualNetworks", "tags": { "Environment": "prod", "Owner": "ann" } },
  { "name": "myVM1", "type": "Microsoft.Compute/virtualMachines", "tags": {} },
  { "name": "kv-shop-dev-jpe-001", "type": "Microsoft.KeyVault/vaults", "tags": { "Environment": "dev", "Owner": "ken", "CostCenter": "4100" } },
  { "name": "app-shop-prd-jpw-1", "type": "Microsoft.Web/sites", "tags": { "Environment": "prod", "Owner": "ann", "CostCenter": "4100" } }
]

@@lesson az1-identity Identity, Security, Governance and Cost
@@text
## Microsoft Entra ID

**Entra ID** (formerly Azure Active Directory) is the cloud identity service behind Azure, Microsoft 365 and thousands of SaaS apps. It's *not* the same as on-premises Active Directory Domain Services: there are no OUs or Group Policy, and it speaks web protocols (SAML, OAuth 2.0, OpenID Connect) instead of Kerberos and LDAP.

- **Hybrid identity**: **Entra Connect** (or Cloud Sync) synchronizes on-premises AD users to Entra ID, with password hash sync, pass-through authentication or federation. **Entra Domain Services** offers managed domain services (domain join, LDAP, Kerberos) in Azure.
- **MFA**, passwordless (Windows Hello for Business, FIDO2 keys, Microsoft Authenticator), **SSPR** (self-service password reset: fewer help desk calls!).
- **Conditional Access**: *if* (user, group, location, device compliance, risk, app) *then* (allow, require MFA, require a compliant device, block). The heart of Microsoft's **zero trust**.
- **PIM** (Privileged Identity Management): just-in-time admin roles with approval. **Identity Protection** detects risky sign-ins.
- External identities: **B2B** (guest users from partners); customer identity (External ID).
- Licences: Free, **P1** (Conditional Access), **P2** (PIM, Identity Protection), often bundled in Microsoft 365 E3/E5.

## Azure RBAC

**Role-based access control**: assign a **role** to a **security principal** (user, group, service principal, **managed identity**) at a **scope** (management group, subscription, resource group, resource). Built-in roles: **Owner** (everything including access), **Contributor** (everything except granting access), **Reader**, **User Access Administrator**; plus hundreds of specific ones (Virtual Machine Contributor...). **Managed identities** let Azure resources authenticate to other services without stored secrets. Note: **Entra roles** (Global Administrator, User Administrator) manage the directory; **Azure roles** manage resources.

## Governance

| Tool | Does |
|---|---|
| **Azure Policy** | enforces rules on resources (allowed regions, required tags, no public IPs), audits compliance, can auto-remediate; **initiatives** group policies |
| **Resource locks** | **CanNotDelete** (Delete) or **ReadOnly**: protect against accidents, even for Owners |
| **Tags** | name/value metadata for cost reports and automation (`env=prod`, `costcenter=it`) |
| **Microsoft Purview** | data governance, classification, compliance |
| **Service Trust Portal** | Microsoft's audit reports and compliance documents |

## Security and monitoring

- **Microsoft Defender for Cloud**: security posture (secure score) and workload protection. **Microsoft Sentinel**: cloud SIEM and SOAR.
- **Azure Monitor**: metrics, **Log Analytics** (query logs with **KQL**), alerts, **Application Insights** (app performance).
- **Azure Advisor**: recommendations for reliability, security, performance, cost, operational excellence.
- **Azure Service Health**: outages and planned maintenance that affect *your* resources.
- **Key Vault**: secrets, keys and certificates.

## Cost

- Factors: resource type, size, region, **bandwidth** (data out costs; in is free), reserved capacity, licensing.
- **Pricing calculator** (estimate a new design), **TCO calculator** (compare with on-premises), **Cost Management** (analyse, **budgets**, alerts).
- Save with **Azure Reservations** (1 or 3 years), **Savings plans**, **spot VMs**, **Azure Hybrid Benefit** (reuse Windows Server and SQL Server licences), auto-shutdown of dev VMs, and right-sizing.
- **SLAs**: e.g. 99.9% vs 99.99%; combining services multiplies their SLAs (two 99.9% services in series ≈ 99.8%).

@@quiz
Users should be prompted for MFA only when signing in from outside the office network. Which feature does this?
* Conditional Access
- Azure Policy
- Resource locks
- RBAC
? Conditional Access policies evaluate signals like location and device.

@@quiz
A team member must manage VMs in one resource group but must not be able to grant access to others. Which role, at which scope?
* Contributor (or Virtual Machine Contributor) on that resource group
- Owner on the subscription
- Reader on the resource group
- User Access Administrator on the subscription
? Contributor can manage resources but not assign roles; scope it as narrowly as possible.

@@quiz
Only resources in Japan East and Japan West may be created. Which tool enforces this?
* Azure Policy (allowed locations)
- Resource tags
- Azure Advisor
- Cost Management budgets
? Policy denies non-compliant deployments.

@@quiz
A production database must never be deleted by accident, even by an Owner. What should you add?
* A CanNotDelete resource lock
- A tag
- A ReadOnly role
- A management group
? Locks apply to everyone; they must be removed before deletion.

@@quiz
Which service synchronizes on-premises Active Directory users to Entra ID?
* Microsoft Entra Connect (or Cloud Sync)
- Azure Bastion
- Azure Arc
- Azure Migrate
? Users then sign in to Microsoft 365 with their normal company password.

@@quiz
An app on an Azure VM needs to read secrets from Key Vault without storing any credentials. What should it use?
* A managed identity
- A shared admin password in a config file
- A storage account key
- An Entra B2B guest account
? Azure handles the identity's credentials automatically.

@@quiz
A company wants to reuse its existing Windows Server licences with Software Assurance on Azure VMs to save money. Which benefit?
* Azure Hybrid Benefit
- Azure Reservations
- Spot VMs
- The TCO calculator
? It removes the Windows licence part of the VM price.

@@quiz
Which tool shows Azure outages and planned maintenance affecting your own resources?
* Azure Service Health
- Azure Advisor
- Microsoft Purview
- Azure Policy
? The public Azure status page shows only broad outages.

@@flashcards
Entra ID :: Microsoft's cloud identity service (formerly Azure AD)
Entra Connect :: Syncs on-premises AD to Entra ID
Conditional Access :: If-then access policies: MFA, compliant device, block
PIM :: Just-in-time privileged roles
SSPR :: Self-service password reset
B2B :: Guest access for partner organizations
Azure RBAC :: Role + principal + scope
Owner / Contributor / Reader :: Full incl. access / manage resources, no access grants / view only
managed identity :: Credential-free identity for Azure resources
Azure Policy :: Enforce and audit rules on resources
resource lock :: CanNotDelete or ReadOnly
tags :: Metadata for cost and organization
Defender for Cloud :: Security posture and workload protection
Microsoft Sentinel :: Cloud SIEM/SOAR
Azure Monitor / Log Analytics / KQL :: Metrics and logs / log workspace / query language
Azure Advisor :: Best-practice recommendations
Service Health :: Outages affecting your resources
Key Vault :: Secrets, keys, certificates
TCO calculator :: Compare on-premises with Azure costs
Azure Hybrid Benefit :: Reuse Windows Server / SQL Server licences
composite SLA :: Multiply the SLAs of dependent services

@@drill
Which built-in Azure role can manage everything, including granting access to others?
= Owner
? Contributor can manage resources but not access.

@@exercise Evaluate Conditional Access policies
Evaluate sign-ins against Conditional Access policies. Each policy has conditions (users, apps, locations, client type) and a grant control: block, or require MFA and/or a compliant device. A sign-in must satisfy **every** policy that applies, and any applicable block wins. Print the outcome for each sign-in.
@@lang python
@@starter
policies = [
    {"name": "Block legacy auth", "users": "all", "apps": "all", "client": "legacy", "locations": "any", "grant": {"block"}},
    {"name": "MFA outside office", "users": "all", "apps": "all", "client": "modern", "locations": "untrusted", "grant": {"mfa"}},
    {"name": "Admins: MFA + compliant device", "users": "admins", "apps": "azure-portal", "client": "modern", "locations": "any", "grant": {"mfa", "compliant"}},
]
signins = [
    {"user": "aiko", "admin": False, "app": "outlook", "client": "modern", "trusted": True, "mfa": False, "compliant": False},
    {"user": "aiko", "admin": False, "app": "outlook", "client": "modern", "trusted": False, "mfa": True, "compliant": False},
    {"user": "ben", "admin": False, "app": "outlook", "client": "legacy", "trusted": True, "mfa": False, "compliant": True},
    {"user": "chen", "admin": True, "app": "azure-portal", "client": "modern", "trusted": True, "mfa": True, "compliant": False},
    {"user": "chen", "admin": True, "app": "azure-portal", "client": "modern", "trusted": False, "mfa": True, "compliant": True},
]

def applies(policy, s):
    raise NotImplementedError

def evaluate(s):
    raise NotImplementedError

for s in signins:
    where = "office" if s["trusted"] else "outside"
    print(f"{s['user']:<5} {s['app']:<13} {s['client']:<7} {where:<8} -> {evaluate(s)}")
@@expected
aiko  outlook       modern  office   -> granted (no policy applies)
aiko  outlook       modern  outside  -> granted (1 policy satisfied)
ben   outlook       legacy  office   -> BLOCKED by Block legacy auth
chen  azure-portal  modern  office   -> needs compliant device
chen  azure-portal  modern  outside  -> granted (2 policies satisfied)
@@requires
def applies
def evaluate
"block" in
@@hint
A policy applies when the users match (`"all"`, or `"admins"` and the user is an admin), the app matches (`"all"` or equal), the client type matches, and the location matches (`"any"`, or `"untrusted"` and the sign-in isn't trusted). In `evaluate`, collect the applicable policies; if any has `"block" in grant`, the result is blocked; otherwise list the unmet requirements (mfa, compliant device), or "granted".
@@solution
policies = [
    {"name": "Block legacy auth", "users": "all", "apps": "all", "client": "legacy", "locations": "any", "grant": {"block"}},
    {"name": "MFA outside office", "users": "all", "apps": "all", "client": "modern", "locations": "untrusted", "grant": {"mfa"}},
    {"name": "Admins: MFA + compliant device", "users": "admins", "apps": "azure-portal", "client": "modern", "locations": "any", "grant": {"mfa", "compliant"}},
]
signins = [
    {"user": "aiko", "admin": False, "app": "outlook", "client": "modern", "trusted": True, "mfa": False, "compliant": False},
    {"user": "aiko", "admin": False, "app": "outlook", "client": "modern", "trusted": False, "mfa": True, "compliant": False},
    {"user": "ben", "admin": False, "app": "outlook", "client": "legacy", "trusted": True, "mfa": False, "compliant": True},
    {"user": "chen", "admin": True, "app": "azure-portal", "client": "modern", "trusted": True, "mfa": True, "compliant": False},
    {"user": "chen", "admin": True, "app": "azure-portal", "client": "modern", "trusted": False, "mfa": True, "compliant": True},
]

def applies(policy, s):
    users_ok = policy["users"] == "all" or (policy["users"] == "admins" and s["admin"])
    app_ok = policy["apps"] in ("all", s["app"])
    client_ok = policy["client"] == s["client"]
    location_ok = policy["locations"] == "any" or (policy["locations"] == "untrusted" and not s["trusted"])
    return users_ok and app_ok and client_ok and location_ok

def evaluate(s):
    matching = [p for p in policies if applies(p, s)]
    if any("block" in p["grant"] for p in matching):
        return "BLOCKED by " + ", ".join(p["name"] for p in matching if "block" in p["grant"])
    missing = set()
    for p in matching:
        if "mfa" in p["grant"] and not s["mfa"]:
            missing.add("MFA")
        if "compliant" in p["grant"] and not s["compliant"]:
            missing.add("compliant device")
    if missing:
        return "needs " + " and ".join(sorted(missing))
    return "granted" + (f" ({len(matching)} {'policy' if len(matching) == 1 else 'policies'} satisfied)" if matching else " (no policy applies)")

for s in signins:
    where = "office" if s["trusted"] else "outside"
    print(f"{s['user']:<5} {s['app']:<13} {s['client']:<7} {where:<8} -> {evaluate(s)}")

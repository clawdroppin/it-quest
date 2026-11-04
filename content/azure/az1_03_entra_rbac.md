@@lesson az1-rbac-deep Entra ID and Azure RBAC in Depth
@@text
Identity is the biggest single topic of AZ-104 ("manage Azure identities and governance", about 20–25%). This lesson separates the two permission systems people confuse (Entra roles vs Azure roles), shows how RBAC assignments inherit down the scope hierarchy, and covers groups, Conditional Access, PIM and managed identities.

## Two permission systems

| | **Microsoft Entra roles** | **Azure RBAC roles** |
|---|---|---|
| Control | the directory: users, groups, app registrations, licences | Azure **resources**: VMs, storage, networks |
| Examples | Global Administrator, User Administrator, Groups Administrator | **Owner**, **Contributor**, **Reader**, **User Access Administrator** |
| Scope | tenant (or administrative units) | management group, subscription, resource group, resource |

A Global Administrator has **no access to Azure resources by default** but can **elevate** to User Access Administrator at the root scope (for emergencies; turn it off afterwards).

## Users and groups

- User types: **members** (cloud-only or synced from AD with **Entra Connect / Cloud Sync**) and **guests** (B2B collaboration).
- Group types: **Security** and **Microsoft 365**. Membership: **assigned**, **dynamic user** or **dynamic device** (requires Entra ID P1), e.g. `(user.department -eq "Sales") -and (user.country -eq "JP")`.
- **Bulk operations** with CSV, self-service password reset (SSPR), licences assigned to groups.
- **Administrative units** limit an admin's scope to a subset of users (e.g. one country).

## RBAC building blocks

A **role assignment** = **security principal** (user, group, service principal, managed identity) + **role definition** + **scope**.

Built-in roles:
- **Owner**: everything, including assigning roles.
- **Contributor**: create and manage resources, **cannot assign roles**.
- **Reader**: view only.
- **User Access Administrator**: manage role assignments only.
- Service-specific roles: Virtual Machine Contributor, Storage Blob Data Reader (a **data-plane** role), Network Contributor, and many more.

Role definitions contain `Actions`, `NotActions`, `DataActions`, `NotDataActions` and `AssignableScopes`. **NotActions is not a deny**: it subtracts from that role's Actions only; another role can still grant it. Real denies come from **deny assignments** (created by Blueprints/managed apps/deployment stacks).

```json,norun
{
  "Name": "VM Operator",
  "IsCustom": true,
  "Actions": ["Microsoft.Compute/virtualMachines/read", "Microsoft.Compute/virtualMachines/start/action",
              "Microsoft.Compute/virtualMachines/restart/action", "Microsoft.Compute/virtualMachines/deallocate/action"],
  "NotActions": [],
  "AssignableScopes": ["/subscriptions/00000000-0000-0000-0000-000000000000"]
}
```

## Scope and inheritance

**Management group → subscription → resource group → resource.** Assignments **inherit downward**. Effective permissions are the **union** of all assignments at that scope and above (minus deny assignments). Assign to **groups**, at the **narrowest** scope that works. **Check access** in the portal (IAM blade) or `Get-AzRoleAssignment -SignInName ana@contoso.com`.

```powershell,norun
New-AzRoleAssignment -ObjectId $group.Id -RoleDefinitionName "Contributor" -ResourceGroupName "rg-web-prod"
az role assignment create --assignee ana@contoso.com --role "Reader" --scope /subscriptions/<id>/resourceGroups/rg-web-prod
```

## Protecting identities

- **MFA** and **Security defaults** (free baseline) or **Conditional Access** (P1): if user/group + app + condition (location, device compliance, sign-in risk) → grant with MFA/compliant device, or block.
- **Privileged Identity Management** (P2): **just-in-time** activation of roles with approval, time limits and justification; access reviews.
- **Identity Protection** (P2): user and sign-in risk.
- **Managed identities** let Azure resources authenticate to other services without secrets: **system-assigned** (tied to one resource's lifecycle) or **user-assigned** (standalone, shareable).
- **Service principals** represent app registrations; prefer certificates or federated credentials to client secrets.

@@quiz
A Contributor on a resource group tries to give a colleague access to it. What happens?
- It works
* It fails: Contributor can't assign roles
- It works only for Reader
- It requires PIM
? Owner or User Access Administrator can assign roles.

@@quiz
A user is Reader at the subscription and Contributor on rg-web. What can they do in rg-web?
- Only read
* Manage resources (Contributor), since permissions are the union of assignments
- Nothing, the assignments conflict
- Assign roles
? RBAC is additive.

@@quiz
Which statement about NotActions is true?
- It denies the action everywhere
* It removes actions from that role only; another role can still grant them
- It overrides Owner
- It applies only to data actions
? Use deny assignments for a real deny.

@@quiz
Which feature gives admins time-limited, approved activation of privileged roles?
- Conditional Access
* Privileged Identity Management
- Administrative units
- Security defaults
? Requires Entra ID P2.

@@quiz
A VM needs to read secrets from Key Vault without storing credentials. What should you use?
- A service principal with a client secret in app settings
* A managed identity with a Key Vault access role
- The storage account key
- A guest user
? Managed identities have no secrets to manage.

@@quiz
Which group membership type adds all users whose department is Sales automatically?
- Assigned
* Dynamic user
- Dynamic device
- Microsoft 365 assigned
? Requires P1 licences.

@@flashcards
Entra roles :: manage the directory
Azure RBAC :: manage resources
Global Admin resource access :: none by default, can elevate
role assignment :: principal + role + scope
Owner :: everything incl. role assignments
Contributor :: manage resources, no role assignments
User Access Administrator :: manage access only
scope order :: management group, subscription, RG, resource
inheritance :: downward, union of assignments
NotActions :: subtract, not deny
deny assignment :: real deny
data-plane role :: e.g. Storage Blob Data Reader
dynamic group :: rule-based membership (P1)
administrative unit :: scoped directory admin
Conditional Access :: if conditions then grant/block
PIM :: just-in-time privileged roles (P2)
system-assigned identity :: lifecycle tied to resource
user-assigned identity :: standalone, shareable
Entra Connect :: sync on-prem AD

@@drill
Which built-in role can manage resources but not grant access?
= Contributor
? Owner can do both.

@@drill
Which kind of managed identity can be shared by several resources?
= user-assigned
= user assigned
= User-assigned
? System-assigned belongs to one resource.

@@exercise Compute effective RBAC permissions
Given role definitions (with `*` wildcards in Actions/NotActions) and assignments at different scopes, decide for each check whether the principal may perform the action at the target scope. An assignment applies when its scope is a prefix of the target (inheritance), and a role grants an action if it matches an Action and no NotAction of the same role. Deny assignments win. Print `allowed by <role> at <scope>` or `denied`.
@@lang python
@@starter
from fnmatch import fnmatchcase

roles = {
    "Reader": {"actions": ["*/read"], "not_actions": []},
    "Contributor": {"actions": ["*"], "not_actions": ["Microsoft.Authorization/*/write", "Microsoft.Authorization/*/delete"]},
    "VM Operator": {"actions": ["Microsoft.Compute/virtualMachines/read", "Microsoft.Compute/virtualMachines/start/action",
                                "Microsoft.Compute/virtualMachines/restart/action"], "not_actions": []},
}
SUB = "/subscriptions/1111"
assignments = [
    ("ana", "Reader", SUB),
    ("ana", "Contributor", SUB + "/resourceGroups/rg-web"),
    ("ops-team", "VM Operator", SUB + "/resourceGroups/rg-app"),
]
deny = [("ana", "Microsoft.Network/*/delete", SUB + "/resourceGroups/rg-web/providers/Microsoft.Network/virtualNetworks/vnet-hub")]
checks = [
    ("ana", "Microsoft.Compute/virtualMachines/read", SUB + "/resourceGroups/rg-app/providers/Microsoft.Compute/virtualMachines/vm1"),
    ("ana", "Microsoft.Compute/virtualMachines/write", SUB + "/resourceGroups/rg-app/providers/Microsoft.Compute/virtualMachines/vm1"),
    ("ana", "Microsoft.Compute/virtualMachines/write", SUB + "/resourceGroups/rg-web/providers/Microsoft.Compute/virtualMachines/web1"),
    ("ana", "Microsoft.Authorization/roleAssignments/write", SUB + "/resourceGroups/rg-web"),
    ("ana", "Microsoft.Network/virtualNetworks/delete", SUB + "/resourceGroups/rg-web/providers/Microsoft.Network/virtualNetworks/vnet-hub"),
    ("ops-team", "Microsoft.Compute/virtualMachines/restart/action", SUB + "/resourceGroups/rg-app/providers/Microsoft.Compute/virtualMachines/vm1"),
    ("ops-team", "Microsoft.Compute/virtualMachines/delete", SUB + "/resourceGroups/rg-app/providers/Microsoft.Compute/virtualMachines/vm1"),
]
@@expected
ana Microsoft.Compute/virtualMachines/read on ~/resourceGroups/rg-app/providers/Microsoft.Compute/virtualMachines/vm1: allowed by Reader at /subscriptions/1111
ana Microsoft.Compute/virtualMachines/write on ~/resourceGroups/rg-app/providers/Microsoft.Compute/virtualMachines/vm1: denied
ana Microsoft.Compute/virtualMachines/write on ~/resourceGroups/rg-web/providers/Microsoft.Compute/virtualMachines/web1: allowed by Contributor at /subscriptions/1111/resourceGroups/rg-web
ana Microsoft.Authorization/roleAssignments/write on ~/resourceGroups/rg-web: denied
ana Microsoft.Network/virtualNetworks/delete on ~/resourceGroups/rg-web/providers/Microsoft.Network/virtualNetworks/vnet-hub: denied (deny assignment)
ops-team Microsoft.Compute/virtualMachines/restart/action on ~/resourceGroups/rg-app/providers/Microsoft.Compute/virtualMachines/vm1: allowed by VM Operator at /subscriptions/1111/resourceGroups/rg-app
ops-team Microsoft.Compute/virtualMachines/delete on ~/resourceGroups/rg-app/providers/Microsoft.Compute/virtualMachines/vm1: denied
@@requires
def grants
startswith(
fnmatchcase(
@@hint
`grants(role, action)`: any Action pattern matches and no NotAction pattern matches. A scope applies when `target == scope or target.startswith(scope + "/")`.
@@solution
from fnmatch import fnmatchcase

roles = {
    "Reader": {"actions": ["*/read"], "not_actions": []},
    "Contributor": {"actions": ["*"], "not_actions": ["Microsoft.Authorization/*/write", "Microsoft.Authorization/*/delete"]},
    "VM Operator": {"actions": ["Microsoft.Compute/virtualMachines/read", "Microsoft.Compute/virtualMachines/start/action",
                                "Microsoft.Compute/virtualMachines/restart/action"], "not_actions": []},
}
SUB = "/subscriptions/1111"
assignments = [
    ("ana", "Reader", SUB),
    ("ana", "Contributor", SUB + "/resourceGroups/rg-web"),
    ("ops-team", "VM Operator", SUB + "/resourceGroups/rg-app"),
]
deny = [("ana", "Microsoft.Network/*/delete", SUB + "/resourceGroups/rg-web/providers/Microsoft.Network/virtualNetworks/vnet-hub")]
checks = [
    ("ana", "Microsoft.Compute/virtualMachines/read", SUB + "/resourceGroups/rg-app/providers/Microsoft.Compute/virtualMachines/vm1"),
    ("ana", "Microsoft.Compute/virtualMachines/write", SUB + "/resourceGroups/rg-app/providers/Microsoft.Compute/virtualMachines/vm1"),
    ("ana", "Microsoft.Compute/virtualMachines/write", SUB + "/resourceGroups/rg-web/providers/Microsoft.Compute/virtualMachines/web1"),
    ("ana", "Microsoft.Authorization/roleAssignments/write", SUB + "/resourceGroups/rg-web"),
    ("ana", "Microsoft.Network/virtualNetworks/delete", SUB + "/resourceGroups/rg-web/providers/Microsoft.Network/virtualNetworks/vnet-hub"),
    ("ops-team", "Microsoft.Compute/virtualMachines/restart/action", SUB + "/resourceGroups/rg-app/providers/Microsoft.Compute/virtualMachines/vm1"),
    ("ops-team", "Microsoft.Compute/virtualMachines/delete", SUB + "/resourceGroups/rg-app/providers/Microsoft.Compute/virtualMachines/vm1"),
]

def in_scope(target, scope):
    return target == scope or target.startswith(scope + "/")

def grants(role, action):
    r = roles[role]
    if not any(fnmatchcase(action, a) for a in r["actions"]):
        return False
    return not any(fnmatchcase(action, n) for n in r["not_actions"])

def check(principal, action, target):
    for p, pattern, scope in deny:
        if p == principal and fnmatchcase(action, pattern) and in_scope(target, scope):
            return "denied (deny assignment)"
    for p, role, scope in assignments:
        if p == principal and in_scope(target, scope) and grants(role, action):
            return f"allowed by {role} at {scope}"
    return "denied"

for principal, action, target in checks:
    short = target.replace(SUB, "~")
    print(f"{principal} {action} on {short}: {check(principal, action, target)}")

@@exercise Evaluate dynamic group rules
Implement a tiny evaluator for dynamic membership rules of the form `(user.attr -op "value") -and/-or (...)` with operators `-eq`, `-ne`, `-startsWith` and `-contains` (case-insensitive, like Entra). Rules use only one kind of connector (`-and` or `-or`). Print the members of each group.
@@lang python
@@starter
import re

users = [
    {"name": "Aiko", "department": "Sales", "country": "JP", "jobTitle": "Account Manager"},
    {"name": "Ben", "department": "sales", "country": "UK", "jobTitle": "Sales Engineer"},
    {"name": "Chen", "department": "IT", "country": "JP", "jobTitle": "Cloud Engineer"},
    {"name": "Dara", "department": "IT", "country": "DE", "jobTitle": "Service Desk Analyst"},
]
groups = {
    "Sales-JP": '(user.department -eq "Sales") -and (user.country -eq "JP")',
    "All-Sales": '(user.department -eq "sales")',
    "Engineers": '(user.jobTitle -contains "Engineer")',
    "JP-or-IT": '(user.country -eq "JP") -or (user.department -eq "IT")',
    "IT-Service-Desk": '(user.department -eq "IT") -and (user.jobTitle -startsWith "Service")',
}

def evaluate(rule, user):
    raise NotImplementedError
@@expected
Sales-JP: Aiko
All-Sales: Aiko, Ben
Engineers: Ben, Chen
JP-or-IT: Aiko, Chen, Dara
IT-Service-Desk: Dara
@@requires
def evaluate
re.findall(
.lower()
@@hint
`re.findall(r'\(user\.(\w+) -(\w+) "([^"]*)"\)', rule)` gives `(attr, op, value)` triples. Compare lowercase strings. Combine with `all` for `-and`, `any` for `-or`.
@@solution
import re

users = [
    {"name": "Aiko", "department": "Sales", "country": "JP", "jobTitle": "Account Manager"},
    {"name": "Ben", "department": "sales", "country": "UK", "jobTitle": "Sales Engineer"},
    {"name": "Chen", "department": "IT", "country": "JP", "jobTitle": "Cloud Engineer"},
    {"name": "Dara", "department": "IT", "country": "DE", "jobTitle": "Service Desk Analyst"},
]
groups = {
    "Sales-JP": '(user.department -eq "Sales") -and (user.country -eq "JP")',
    "All-Sales": '(user.department -eq "sales")',
    "Engineers": '(user.jobTitle -contains "Engineer")',
    "JP-or-IT": '(user.country -eq "JP") -or (user.department -eq "IT")',
    "IT-Service-Desk": '(user.department -eq "IT") -and (user.jobTitle -startsWith "Service")',
}

def test(user, attr, op, value):
    actual = user.get(attr, "").lower()
    value = value.lower()
    if op == "eq":
        return actual == value
    if op == "ne":
        return actual != value
    if op == "startsWith":
        return actual.startswith(value)
    if op == "contains":
        return value in actual
    raise ValueError(op)

def evaluate(rule, user):
    results = [test(user, a, op, v) for a, op, v in re.findall(r'\(user\.(\w+) -(\w+) "([^"]*)"\)', rule)]
    return any(results) if " -or " in rule else all(results)

for group, rule in groups.items():
    members = [u["name"] for u in users if evaluate(rule, u)]
    print(f"{group}: {', '.join(members) or '(none)'}")

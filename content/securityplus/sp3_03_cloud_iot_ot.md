@@lesson sp3-cloudot Cloud, Virtualisation, IoT and OT Security
@@text
Security architecture now spans data centres, several clouds, containers, serverless functions, and devices from smart cameras to factory controllers. Security+ asks how each model shifts responsibility and risk, and which controls fit. This lesson goes through them one by one.

## Shared responsibility

| Layer | IaaS | PaaS | SaaS |
|---|---|---|---|
| Data, user access, accounts | **customer** | **customer** | **customer** |
| Applications | customer | customer | provider |
| OS, runtime, middleware | customer | provider | provider |
| Virtualisation, servers, network, physical | provider | provider | provider |

The customer is **always** responsible for their data, identities and access configuration. Most cloud breaches are **customer misconfigurations**: public storage buckets, over-privileged keys, open security groups, no MFA on admin accounts.

A **responsibility matrix** for your contract spells this out. **Third-party vendors** in the chain (MSPs, SaaS integrations) add their own responsibilities.

## Cloud deployment and architecture

- **Public, private, hybrid, community, multi-cloud**.
- **Infrastructure as code (IaC)**: Terraform, CloudFormation, Bicep. Benefits: consistent, reviewable, scanned for misconfigurations before deployment. Risk: one bad template deploys the mistake everywhere.
- **Serverless** (functions as a service): no servers to patch, but you still secure code, permissions (each function's role) and secrets.
- **Microservices**: small services talking over APIs; secure the APIs (authentication, rate limiting, input validation), use a service mesh for mTLS.
- **Containers**: a container shares the host kernel (lighter isolation than VMs). Use minimal, signed, scanned images; don't run as root; limit capabilities; orchestration (Kubernetes) with RBAC and network policies.
- **Software-defined networking** and **cloud-native firewalls** (security groups).
- **CASB** (cloud access security broker): visibility and control over SaaS use (shadow IT, DLP for cloud apps).

## Virtualisation risks

- **VM escape**: code breaks out of a guest to the hypervisor and other guests. Patch hypervisors.
- **VM sprawl**: forgotten VMs that nobody patches.
- **Resource reuse**: memory or disk not wiped between tenants.
- **Type 1** (bare-metal: ESXi, Hyper-V, KVM) vs **Type 2** (hosted: VirtualBox, VMware Workstation) hypervisors.

## On-premises and centralised vs decentralised

On-premises means full control and full responsibility. **Centralised** architectures are easier to manage but have single points of failure; **decentralised** (distributed) ones are resilient but harder to secure consistently.

## Embedded, IoT, ICS/SCADA and RTOS

| System | Examples | Security concerns |
|---|---|---|
| **IoT** | cameras, smart TVs, sensors, building systems | default passwords, no updates, weak vendors, data collection |
| **ICS / SCADA** (operational technology, OT) | power grids, water treatment, factories, PLCs | availability and safety first; legacy protocols with no auth (Modbus, DNP3); can't be patched or rebooted easily |
| **RTOS** | medical devices, car systems, industrial controllers | deterministic timing; limited resources for security tools |
| **Embedded systems** | printers, routers, appliances | firmware updates, hard-coded credentials |

Considerations: **power** (battery-limited devices can't run heavy crypto), **compute**, **network** constraints, **cryptographic** limits, **patch availability**, **authentication** limits, **range**, **implied trust**, **cost**, and **inability to patch**.

Controls: **segmentation** (separate IoT/OT networks with strict firewall rules, often a one-way data diode for the most critical), **change default credentials**, **firmware updates**, **monitor** with OT-aware IDS, **disable unused features**, **vendor security requirements** in contracts, **asset inventory**.

## High availability in architecture

**Load balancing vs clustering**, **geographic dispersion** (regions/availability zones), **platform diversity** (not all eggs in one vendor), **multi-cloud**, **continuity of operations**, **capacity planning** (people, technology, infrastructure).

@@quiz
A company stores customer files in a SaaS app. Who is responsible for which employees can access those files?
- The SaaS provider
* The customer
- The cloud hosting provider
- Nobody: SaaS handles it automatically
? Data and access are always the customer's responsibility.

@@quiz
In IaaS, who patches the guest operating system?
* The customer
- The provider
- The hypervisor vendor
- The ISP
? In PaaS and SaaS, the provider does.

@@quiz
Which risk describes code inside a VM breaking out to affect the hypervisor and other VMs?
- VM sprawl
* VM escape
- Resource reuse
- Container drift
? Patch hypervisors and limit guest privileges.

@@quiz
A water treatment plant's controllers can't be patched and use protocols without authentication. What's the most important control?
- Install antivirus on each PLC
* Isolate the OT network with strict segmentation and monitoring
- Expose them through a VPN to all staff
- Reboot them weekly
? Availability and safety come first; isolate and monitor.

@@quiz
Which tool gives visibility into which SaaS apps employees use and can enforce policies on them?
- WAF
* CASB
- HSM
- RTOS
? Cloud access security broker.

@@flashcards
shared responsibility :: provider secures the cloud, customer secures in the cloud
customer always owns :: data, identities, access
IaaS customer patches :: the guest OS
IaC :: infrastructure defined in code
serverless risk :: function permissions and code
container :: shares the host kernel
VM escape :: guest breaks out to host
VM sprawl :: unmanaged forgotten VMs
type 1 hypervisor :: bare metal
type 2 hypervisor :: runs on a host OS
CASB :: controls SaaS use
ICS/SCADA :: industrial control, availability first
PLC :: programmable logic controller
RTOS :: real-time operating system
data diode :: one-way network link
IoT controls :: segment, change defaults, update firmware
geographic dispersion :: spread across regions

@@drill
In which cloud model does the provider manage the application itself?
= SaaS
= saas
? IaaS and PaaS leave the application to the customer.

@@drill
Which type of hypervisor runs directly on the hardware?
= type 1
= Type 1
= 1
? Type 2 runs on top of an OS.

@@exercise Who is responsible?
For each task and service model, print who is responsible using the responsibility layers: `data` and `access` are always the customer; `application` is the customer in IaaS and PaaS; `os` and `runtime` are the customer in IaaS only; `virtualisation`, `hardware` and `physical` are always the provider. Then print, for each model, how many of the tasks belong to the customer.
@@lang python
@@starter
CUSTOMER_LAYERS = {
    "IaaS": {"data", "access", "application", "os", "runtime"},
    "PaaS": {"data", "access", "application"},
    "SaaS": {"data", "access"},
}
tasks = [
    ("Patch the web server OS", "os"),
    ("Enable MFA for admins", "access"),
    ("Replace a failed disk", "hardware"),
    ("Classify and back up records", "data"),
    ("Fix a bug in the order app", "application"),
    ("Update the Python runtime", "runtime"),
    ("Guard the data centre", "physical"),
]
@@expected
Patch the web server OS -> IaaS: customer, PaaS: provider, SaaS: provider
Enable MFA for admins -> IaaS: customer, PaaS: customer, SaaS: customer
Replace a failed disk -> IaaS: provider, PaaS: provider, SaaS: provider
Classify and back up records -> IaaS: customer, PaaS: customer, SaaS: customer
Fix a bug in the order app -> IaaS: customer, PaaS: customer, SaaS: provider
Update the Python runtime -> IaaS: customer, PaaS: provider, SaaS: provider
Guard the data centre -> IaaS: provider, PaaS: provider, SaaS: provider
IaaS: customer owns 5 of 7 tasks
PaaS: customer owns 3 of 7 tasks
SaaS: customer owns 2 of 7 tasks
@@requires
CUSTOMER_LAYERS[
@@hint
Loop over the models and tasks; `"customer" if layer in CUSTOMER_LAYERS[model] else "provider"`.
@@solution
CUSTOMER_LAYERS = {
    "IaaS": {"data", "access", "application", "os", "runtime"},
    "PaaS": {"data", "access", "application"},
    "SaaS": {"data", "access"},
}
tasks = [
    ("Patch the web server OS", "os"),
    ("Enable MFA for admins", "access"),
    ("Replace a failed disk", "hardware"),
    ("Classify and back up records", "data"),
    ("Fix a bug in the order app", "application"),
    ("Update the Python runtime", "runtime"),
    ("Guard the data centre", "physical"),
]

for task, layer in tasks:
    owners = [f"{m}: {'customer' if layer in CUSTOMER_LAYERS[m] else 'provider'}" for m in CUSTOMER_LAYERS]
    print(f"{task} -> " + ", ".join(owners))
for model in CUSTOMER_LAYERS:
    n = sum(1 for _, layer in tasks if layer in CUSTOMER_LAYERS[model])
    print(f"{model}: customer owns {n} of {len(tasks)} tasks")

@@exercise Scan an IaC template for misconfigurations
A simplified cloud template lists resources as dicts. Flag: storage buckets with `public: True` or without `encryption`; security groups with an ingress rule from `0.0.0.0/0` on port 22 or 3389; IAM policies with `"*"` in both actions and resources; databases with `backup_days` under 7. Print findings per resource and a final `PASS`/`FAIL` for the template.
@@lang python
@@starter
template = [
    {"type": "bucket", "name": "logs", "public": False, "encryption": "aes256"},
    {"type": "bucket", "name": "marketing-assets", "public": True, "encryption": None},
    {"type": "security_group", "name": "web-sg", "ingress": [("0.0.0.0/0", 443), ("0.0.0.0/0", 22)]},
    {"type": "iam_policy", "name": "deploy-role", "actions": ["*"], "resources": ["*"]},
    {"type": "iam_policy", "name": "read-logs", "actions": ["s3:GetObject"], "resources": ["arn:logs/*"]},
    {"type": "database", "name": "orders-db", "backup_days": 1},
]
@@expected
bucket logs: ok
bucket marketing-assets: public bucket; no encryption
security_group web-sg: port 22 open to the internet
iam_policy deploy-role: full admin wildcard policy
iam_policy read-logs: ok
database orders-db: backup retention 1 day(s)
FAIL
@@requires
"0.0.0.0/0"
"*" in
@@hint
Branch on `r["type"]`, collecting messages in a list. The template fails if any resource has findings.
@@solution
template = [
    {"type": "bucket", "name": "logs", "public": False, "encryption": "aes256"},
    {"type": "bucket", "name": "marketing-assets", "public": True, "encryption": None},
    {"type": "security_group", "name": "web-sg", "ingress": [("0.0.0.0/0", 443), ("0.0.0.0/0", 22)]},
    {"type": "iam_policy", "name": "deploy-role", "actions": ["*"], "resources": ["*"]},
    {"type": "iam_policy", "name": "read-logs", "actions": ["s3:GetObject"], "resources": ["arn:logs/*"]},
    {"type": "database", "name": "orders-db", "backup_days": 1},
]

failed = False
for r in template:
    findings = []
    if r["type"] == "bucket":
        if r["public"]:
            findings.append("public bucket")
        if not r["encryption"]:
            findings.append("no encryption")
    elif r["type"] == "security_group":
        for source, port in r["ingress"]:
            if source == "0.0.0.0/0" and port in (22, 3389):
                findings.append(f"port {port} open to the internet")
    elif r["type"] == "iam_policy":
        if "*" in r["actions"] and "*" in r["resources"]:
            findings.append("full admin wildcard policy")
    elif r["type"] == "database":
        if r["backup_days"] < 7:
            findings.append(f"backup retention {r['backup_days']} day(s)")
    if findings:
        failed = True
    print(f"{r['type']} {r['name']}: " + ("; ".join(findings) if findings else "ok"))
print("FAIL" if failed else "PASS")

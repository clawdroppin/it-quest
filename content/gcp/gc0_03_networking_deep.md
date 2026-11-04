@@lesson gc0-network-deep VPC Networking in Depth: Firewalls, Routes, Shared VPC and Hybrid
@@text
Google Cloud's VPC is different from AWS and Azure in one big way: a VPC network is **global**, and its subnets are **regional**. This lesson covers subnet planning, firewall rules and policies, routes and Cloud NAT, private access to Google APIs, Shared VPC and peering, hybrid connectivity and the load balancer family.

## VPC networks and subnets

- **Auto mode** creates one subnet per region from 10.128.0.0/9 (fine for experiments; overlaps with every other auto-mode network). **Custom mode** is recommended for production: you choose regions and ranges.
- Subnets can be expanded (not shrunk) without downtime; **secondary ranges** are used for GKE Pods and Services (alias IPs).
- Google reserves **4 addresses** per primary subnet range: network, **default gateway (.1)**, the **second-to-last** address, and broadcast.
- VMs in different regions of the same VPC talk over internal IPs on Google's backbone with no extra configuration.
- The **default** network (auto mode, with permissive firewall rules) should be deleted in production projects (an org policy can skip creating it).

## Firewall rules

- Apply to the whole VPC, enforced at each VM, **stateful**.
- **Priority 0–65535** (lower wins; default 1000). Each rule: direction (ingress/egress), action (allow/deny), targets (all instances, **network tags**, or **service accounts**), source/destination ranges or tags/service accounts, protocols and ports.
- **Implied rules** (priority 65535, invisible): **allow all egress**, **deny all ingress**.
- The default network adds `default-allow-internal`, `-ssh`, `-rdp`, `-icmp` (priority 65534).
- **Hierarchical firewall policies** at organisation/folder level (with `goto_next` to delegate), **network firewall policies**, FQDN and geo-location objects, threat intelligence (Cloud NGFW).
- Allow health checks from **35.191.0.0/16** and **130.211.0.0/22** for load balancers; allow **35.235.240.0/20** for **IAP TCP forwarding** (SSH without public IPs).
- **Firewall Rules Logging** and **Firewall Insights** show hits and shadowed rules.

## Routes, NAT and private access

- **System routes**: subnet routes and a default route `0.0.0.0/0` → default internet gateway. **Custom static routes** (next hop: instance, internal load balancer, VPN tunnel) and **dynamic routes** from **Cloud Router** (BGP). Most specific prefix wins; then priority (lower wins).
- **Cloud NAT**: managed outbound NAT for VMs without external IPs (regional, per Cloud Router).
- **Private Google Access** (per subnet): VMs without external IPs reach Google APIs (Cloud Storage, BigQuery). **Private Service Connect** gives private endpoints for Google APIs and published services. **Private services access** connects managed services such as Cloud SQL with private IPs.

## Sharing networks

- **Shared VPC**: a **host project** owns the network; **service projects** deploy resources into its subnets. Central network team, separate app teams (grant `compute.networkUser` on specific subnets).
- **VPC Network Peering**: connects two VPCs (even across organisations); **not transitive**; no overlapping ranges; each side controls its firewall.
- **Network Connectivity Center** for hub-and-spoke at scale.

## Hybrid connectivity

| Option | Bandwidth | SLA / notes |
|---|---|---|
| **HA VPN** | ~3 Gb/s per tunnel | **99.99%** with two interfaces and BGP via Cloud Router |
| Classic VPN | | 99.9%, being phased out for most uses |
| **Dedicated Interconnect** | 10 or 100 Gb/s circuits | direct physical connection in a colocation facility |
| **Partner Interconnect** | 50 Mb/s – 50 Gb/s | through a service provider |
| **Cross-Cloud Interconnect** | 10/100 Gb/s | to AWS, Azure, Oracle |

Interconnect traffic isn't encrypted by default: use HA VPN over Interconnect or MACsec.

## Load balancing

| Load balancer | Layer | Scope |
|---|---|---|
| **Global external Application LB** | 7 HTTP(S) | global anycast IP, Cloud CDN, Cloud Armor (WAF/DDoS) |
| Regional external/internal Application LB | 7 | regional (Envoy-based) |
| **External passthrough Network LB** | 4 TCP/UDP | regional, preserves client IP |
| External proxy Network LB | 4 TCP/SSL | global or regional |
| **Internal passthrough Network LB** | 4 | regional, internal |

**Cloud DNS** (public and private zones, DNS peering and forwarding), **Cloud CDN**, **Cloud Armor** security policies.

@@quiz
What is the scope of a VPC network in Google Cloud?
* Global; subnets are regional
- Regional; subnets are zonal
- Zonal
- Per project and region
? One VPC can span all regions.

@@quiz
A firewall rule with priority 500 denies TCP 22 and another with priority 1000 allows it from 10.0.0.0/8. What happens to SSH from 10.1.2.3?
- Allowed, allow wins
* Denied, the lower priority number wins
- Both apply
- Depends on rule creation order
? Priority decides.

@@quiz
VMs without external IPs need to read from Cloud Storage. What do you enable on their subnet?
- Cloud NAT only
* Private Google Access
- VPC peering
- An external load balancer
? Lets internal-only VMs reach Google APIs.

@@quiz
A central network team manages subnets used by many application projects. Which feature?
* Shared VPC
- VPC Network Peering
- Cloud VPN
- Private Service Connect
? Host project and service projects.

@@quiz
Which hybrid option gives a 99.99% SLA over the internet with BGP?
- Classic VPN
* HA VPN with Cloud Router
- Partner Interconnect with one VLAN
- VPC peering
? Two interfaces, two tunnels.

@@quiz
How many addresses does Google reserve in each primary subnet range?
- 5
* 4
- 3
- 2
? Network, gateway, second-to-last, broadcast.

@@flashcards
VPC scope :: global
subnet scope :: regional
auto mode :: one subnet per region, 10.128.0.0/9
custom mode :: recommended for production
reserved per subnet :: 4
firewall priority :: 0–65535, lower wins, default 1000
implied rules :: allow egress, deny ingress
firewall targets :: tags or service accounts
health check ranges :: 35.191.0.0/16, 130.211.0.0/22
IAP TCP forwarding range :: 35.235.240.0/20
Cloud Router :: BGP dynamic routes
Cloud NAT :: outbound for private VMs
Private Google Access :: Google APIs without external IP
Private Service Connect :: private endpoints for APIs/services
Shared VPC :: host and service projects
VPC peering :: not transitive
HA VPN SLA :: 99.99%
Dedicated Interconnect :: 10/100 Gb/s circuits
Partner Interconnect :: via a provider
global external Application LB :: anycast, CDN, Cloud Armor
Cloud Armor :: WAF and DDoS policies

@@drill
What is the default priority of a new VPC firewall rule?
= 1000
? Lower numbers win.

@@drill
Which service provides outbound internet access for VMs without external IPs?
= Cloud NAT
= cloud nat
? Regional, per Cloud Router.

@@exercise Evaluate VPC firewall rules
Evaluate ingress connections to VMs against the rules (lowest priority number first; on equal priority, deny beats allow) plus the implied deny-all ingress at 65535. A rule applies when the VM matches its target (`all`, a network tag, or a service account), the source IP is in one of its ranges, and the protocol/port matches. Print the deciding rule and action.
@@lang python
@@starter
import ipaddress

rules = [
    {"name": "allow-lb-health", "priority": 1000, "action": "allow", "target": ("tag", "web"), "sources": ["35.191.0.0/16", "130.211.0.0/22"], "proto": "tcp", "ports": [80]},
    {"name": "allow-https", "priority": 1000, "action": "allow", "target": ("tag", "web"), "sources": ["0.0.0.0/0"], "proto": "tcp", "ports": [443]},
    {"name": "allow-iap-ssh", "priority": 900, "action": "allow", "target": ("all", None), "sources": ["35.235.240.0/20"], "proto": "tcp", "ports": [22]},
    {"name": "deny-ssh", "priority": 1000, "action": "deny", "target": ("all", None), "sources": ["0.0.0.0/0"], "proto": "tcp", "ports": [22]},
    {"name": "allow-db-from-app", "priority": 1000, "action": "allow", "target": ("sa", "db-sa"), "sources": ["10.10.1.0/24"], "proto": "tcp", "ports": [5432]},
]
vms = {"web-1": {"tags": {"web"}, "sa": "web-sa"}, "db-1": {"tags": set(), "sa": "db-sa"}}
connections = [("web-1", "35.191.4.9", "tcp", 80), ("web-1", "198.51.100.3", "tcp", 443), ("web-1", "198.51.100.3", "tcp", 22),
               ("db-1", "35.235.241.10", "tcp", 22), ("db-1", "10.10.1.7", "tcp", 5432), ("db-1", "10.10.2.7", "tcp", 5432)]

def targets(rule, vm):
    raise NotImplementedError
@@expected
35.191.4.9 -> web-1:80: allow by allow-lb-health (1000)
198.51.100.3 -> web-1:443: allow by allow-https (1000)
198.51.100.3 -> web-1:22: deny by deny-ssh (1000)
35.235.241.10 -> db-1:22: allow by allow-iap-ssh (900)
10.10.1.7 -> db-1:5432: allow by allow-db-from-app (1000)
10.10.2.7 -> db-1:5432: deny by implied-deny-ingress (65535)
@@requires
def targets
sorted(
@@hint
Sort with `key=lambda r: (r["priority"], r["action"] != "deny")` so denies come first within a priority.
@@solution
import ipaddress

rules = [
    {"name": "allow-lb-health", "priority": 1000, "action": "allow", "target": ("tag", "web"), "sources": ["35.191.0.0/16", "130.211.0.0/22"], "proto": "tcp", "ports": [80]},
    {"name": "allow-https", "priority": 1000, "action": "allow", "target": ("tag", "web"), "sources": ["0.0.0.0/0"], "proto": "tcp", "ports": [443]},
    {"name": "allow-iap-ssh", "priority": 900, "action": "allow", "target": ("all", None), "sources": ["35.235.240.0/20"], "proto": "tcp", "ports": [22]},
    {"name": "deny-ssh", "priority": 1000, "action": "deny", "target": ("all", None), "sources": ["0.0.0.0/0"], "proto": "tcp", "ports": [22]},
    {"name": "allow-db-from-app", "priority": 1000, "action": "allow", "target": ("sa", "db-sa"), "sources": ["10.10.1.0/24"], "proto": "tcp", "ports": [5432]},
]
vms = {"web-1": {"tags": {"web"}, "sa": "web-sa"}, "db-1": {"tags": set(), "sa": "db-sa"}}
connections = [("web-1", "35.191.4.9", "tcp", 80), ("web-1", "198.51.100.3", "tcp", 443), ("web-1", "198.51.100.3", "tcp", 22),
               ("db-1", "35.235.241.10", "tcp", 22), ("db-1", "10.10.1.7", "tcp", 5432), ("db-1", "10.10.2.7", "tcp", 5432)]

def targets(rule, vm):
    kind, value = rule["target"]
    if kind == "all":
        return True
    if kind == "tag":
        return value in vm["tags"]
    return vm["sa"] == value

for name, src, proto, port in connections:
    vm = vms[name]
    decision = ("implied-deny-ingress", 65535, "deny")
    for r in sorted(rules, key=lambda r: (r["priority"], r["action"] != "deny")):
        if not targets(r, vm) or r["proto"] != proto or port not in r["ports"]:
            continue
        if any(ipaddress.ip_address(src) in ipaddress.ip_network(s) for s in r["sources"]):
            decision = (r["name"], r["priority"], r["action"])
            break
    print(f"{src} -> {name}:{port}: {decision[2]} by {decision[0]} ({decision[1]})")

@@exercise Plan subnets in a global VPC
Allocate one subnet per region from `BASE` in order of size (largest host count first), each the smallest prefix (no larger than /29) where usable hosts (total − 4 reserved) cover the need, plus, for GKE regions, a /20 Pod range allocated from the start of `SECONDARY_BASE` and a /24 Service range allocated from `SERVICES_BASE` (separate pointers keep each range aligned). Print each subnet with its usable host count and its reserved addresses (network, gateway, second-to-last, broadcast).
@@lang python
@@starter
import ipaddress

BASE = ipaddress.ip_network("10.10.0.0/16")
SECONDARY_BASE = ipaddress.ip_network("100.64.0.0/14")
SERVICES_BASE = ipaddress.ip_network("100.67.0.0/16")
needs = [("europe-west1", 1000, True), ("asia-northeast1", 3000, True), ("us-central1", 200, False), ("europe-west4", 50, False)]
@@expected
asia-northeast1  10.10.0.0/20    4092 usable; reserved 10.10.0.0, 10.10.0.1, 10.10.15.254, 10.10.15.255
                 pods 100.64.0.0/20, services 100.67.0.0/24
europe-west1     10.10.16.0/22   1020 usable; reserved 10.10.16.0, 10.10.16.1, 10.10.19.254, 10.10.19.255
                 pods 100.64.16.0/20, services 100.67.1.0/24
us-central1      10.10.20.0/24   252 usable; reserved 10.10.20.0, 10.10.20.1, 10.10.20.254, 10.10.20.255
europe-west4     10.10.21.0/26   60 usable; reserved 10.10.21.0, 10.10.21.1, 10.10.21.62, 10.10.21.63
@@requires
- 4
def prefix_for
@@hint
`prefix_for(hosts)`: start at 29 and decrease until `2 ** (32 - p) - 4 >= hosts`. Walk a pointer through each base, creating `ipaddress.ip_network(f"{pointer}/{p}")`.
@@solution
import ipaddress

BASE = ipaddress.ip_network("10.10.0.0/16")
SECONDARY_BASE = ipaddress.ip_network("100.64.0.0/14")
SERVICES_BASE = ipaddress.ip_network("100.67.0.0/16")
needs = [("europe-west1", 1000, True), ("asia-northeast1", 3000, True), ("us-central1", 200, False), ("europe-west4", 50, False)]

def prefix_for(hosts):
    p = 29
    while 2 ** (32 - p) - 4 < hosts:
        p -= 1
    return p

pointer = BASE.network_address
pods_ptr = SECONDARY_BASE.network_address
services_ptr = SERVICES_BASE.network_address
for region, hosts, gke in sorted(needs, key=lambda n: -n[1]):
    net = ipaddress.ip_network(f"{pointer}/{prefix_for(hosts)}")
    pointer = net.broadcast_address + 1
    reserved = [net[0], net[1], net[-2], net[-1]]
    print(f"{region:<16} {str(net):<15} {net.num_addresses - 4} usable; reserved {', '.join(map(str, reserved))}")
    if gke:
        pods = ipaddress.ip_network(f"{pods_ptr}/20")
        services = ipaddress.ip_network(f"{services_ptr}/24")
        pods_ptr = pods.broadcast_address + 1
        services_ptr = services.broadcast_address + 1
        print(f"{'':<16} pods {pods}, services {services}")

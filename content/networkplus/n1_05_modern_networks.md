@@lesson n1-modern Data Centres, SDN, SD-WAN and Cloud Networking
@@text
Network+ N10-009 added more modern architecture: spine-and-leaf data centres, software-defined networking, SD-WAN, VXLAN, zero trust and SASE, plus cloud networking concepts and infrastructure as code. You'll meet all of these in any company that runs workloads in a data centre or the cloud.

## Topologies

| Topology | Where | Notes |
|---|---|---|
| **Three-tier** (core, distribution, access) | campus networks | classic hierarchical design |
| **Collapsed core** | smaller sites | core and distribution in one layer |
| **Spine and leaf** | data centres | every leaf connects to every spine; any server is two hops from any other; scales by adding spines/leaves |
| **Star / hub-and-spoke** | branch WANs | all sites through HQ |
| **Mesh** (full or partial) | WAN cores, wireless mesh | redundancy at the cost of links: full mesh needs n(n-1)/2 links |
| **Point-to-point** | site links | |

**North-south** traffic goes in and out of the data centre (clients to servers); **east-west** goes between servers inside it (now the majority, which is why spine-and-leaf replaced three-tier there).

Other data-centre terms: **top-of-rack (ToR)** switches, **SAN** with Fibre Channel or iSCSI, **FCoE**, **branch office vs on-premises vs colocation**.

## Software-defined networking (SDN)

SDN separates the **control plane** (deciding where traffic goes: routing protocols, the SDN controller) from the **data plane** (forwarding packets in hardware). A **management plane** configures devices. An **application layer** asks the controller for outcomes through **northbound APIs**; the controller programs devices through **southbound** interfaces (OpenFlow, NETCONF, vendor APIs).

Benefits: central policy, automation, consistent configuration.

## SD-WAN

Connects branches over any mix of links (broadband, LTE/5G, MPLS) with central management. It's **application-aware** (Teams goes over the best-latency link; backups over the cheapest), **transport-agnostic**, and supports **zero-touch provisioning** of branch boxes and **central policy**.

## VXLAN

**Virtual Extensible LAN** tunnels layer 2 frames over a layer 3 network (UDP **4789**). It has a 24-bit **VNI**, so about **16 million** segments versus 4,094 VLANs. It's used for multi-tenant data centres and stretching L2 between racks or sites. **VTEPs** encapsulate and decapsulate.

## Zero trust, ZTNA and SASE

- **Zero trust architecture**: no implicit trust from network location; verify user, device and context on every request; least privilege; **microsegmentation**; continuous monitoring.
- **ZTNA** (zero trust network access): users connect to **specific applications**, not the whole network as with a traditional VPN.
- **SASE** (secure access service edge): cloud-delivered networking plus security (SD-WAN, secure web gateway, CASB, firewall as a service, ZTNA) close to the user. **SSE** is the security part alone.

## Cloud networking concepts

- **VPC / VNet**: your private network in the cloud, split into **subnets** (public with a route to an **internet gateway**, private without).
- **NAT gateway**: lets private subnets reach out without being reachable.
- **Network security groups / security groups** (stateful, per interface or instance) and **network ACLs** (stateless, per subnet).
- **VPN gateway** (site-to-site IPsec) or **Direct Connect / ExpressRoute / Interconnect** (private dedicated circuits).
- **Peering** between VPCs, **transit gateways** for hub-and-spoke.
- **Cloud models**: public, private, hybrid, community; **IaaS, PaaS, SaaS**.
- **Elasticity** (scales automatically with demand) vs **scalability** (can grow); **multitenancy**.

## Infrastructure as code and automation

Network and cloud config written as code: **Terraform**, **Ansible** playbooks, CloudFormation. Benefits: repeatable, reviewed in **version control**, no **configuration drift**. **Playbooks/templates**, **source control** (Git), **central repositories**, and **dynamic inventories** appear in the objectives. Watch for drift: someone changing a device by hand so it no longer matches the code.

## Other environments

**IoT** and **IIoT**, **SCADA/ICS/OT** (isolate them), **VoIP** (QoS, voice VLAN), **guest** networks, **BYOD**.

@@quiz
Why is spine-and-leaf preferred over three-tier in modern data centres?
* Every leaf connects to every spine, giving predictable low latency for heavy east-west traffic
- It uses fewer cables
- It removes the need for routing
- It only works with wireless
? Any server is two hops from any other.

@@quiz
Which technology lets a branch send voice over the lowest-latency link and backups over the cheapest, managed centrally?
- VXLAN
* SD-WAN
- STP
- PoE
? Application-aware path selection.

@@quiz
How many segments can VXLAN identify?
- 4,094
- 65,535
* About 16 million
- 1,024
? 24-bit VNI vs 12-bit VLAN ID.

@@quiz
In SDN, which plane makes forwarding decisions centrally?
- Data plane
* Control plane
- Management plane
- Physical plane
? The data plane forwards; the control plane decides.

@@quiz
What does a cloud NAT gateway provide?
- Inbound access from the internet to private instances
* Outbound internet access for private subnets without making them reachable from outside
- Encryption between VPCs
- DNS resolution
? Inbound needs a load balancer or public IP.

@@flashcards
spine and leaf :: every leaf to every spine, data centres
three-tier :: core, distribution, access
north-south / east-west :: in-out vs server-to-server traffic
full mesh links :: n(n-1)/2
control plane :: decides where traffic goes
data plane :: forwards packets
northbound / southbound API :: apps to controller / controller to devices
SD-WAN :: app-aware, transport-agnostic WAN
VXLAN :: L2 over L3, UDP 4789, 16M segments
VTEP :: VXLAN tunnel endpoint
ZTNA :: per-application access instead of VPN
SASE :: cloud-delivered networking plus security
VPC :: private cloud network
security group :: stateful instance firewall
network ACL :: stateless subnet filter
IaC :: network/cloud config as code
configuration drift :: device no longer matches code

@@drill
How many links does a full mesh of 6 sites need?
= 15
? 6 × 5 / 2.

@@drill
What UDP port does VXLAN use?
= 4789
? IANA-assigned.

@@exercise Spine and leaf sizing
A spine-and-leaf fabric connects every leaf to every spine. For each design, compute the number of fabric links, the number of server ports (leaves × server ports per leaf), and the **oversubscription ratio** (server bandwidth per leaf ÷ uplink bandwidth per leaf, rounded to 2 decimals). Flag designs above 3:1 as `TOO OVERSUBSCRIBED`, and designs where a spine doesn't have enough ports for all leaves as `NOT ENOUGH SPINE PORTS`.
@@lang python
@@starter
designs = [
    {"name": "small", "spines": 2, "leaves": 6, "spine_ports": 32, "server_ports": 48, "server_gbps": 10, "uplink_gbps": 100},
    {"name": "medium", "spines": 4, "leaves": 20, "spine_ports": 32, "server_ports": 48, "server_gbps": 25, "uplink_gbps": 100},
    {"name": "big", "spines": 4, "leaves": 40, "spine_ports": 32, "server_ports": 48, "server_gbps": 25, "uplink_gbps": 400},
    {"name": "cheap", "spines": 2, "leaves": 8, "spine_ports": 16, "server_ports": 48, "server_gbps": 10, "uplink_gbps": 40},
]
@@expected
small: 12 fabric links, 288 server ports, 2.4:1 -> OK
medium: 80 fabric links, 960 server ports, 3.0:1 -> OK
big: 160 fabric links, 1920 server ports, 0.75:1 -> NOT ENOUGH SPINE PORTS
cheap: 16 fabric links, 384 server ports, 6.0:1 -> TOO OVERSUBSCRIBED
@@requires
round(
"TOO OVERSUBSCRIBED"
@@hint
Per leaf: server bandwidth = server_ports × server_gbps; uplink bandwidth = spines × uplink_gbps. Each spine needs one port per leaf.
@@solution
designs = [
    {"name": "small", "spines": 2, "leaves": 6, "spine_ports": 32, "server_ports": 48, "server_gbps": 10, "uplink_gbps": 100},
    {"name": "medium", "spines": 4, "leaves": 20, "spine_ports": 32, "server_ports": 48, "server_gbps": 25, "uplink_gbps": 100},
    {"name": "big", "spines": 4, "leaves": 40, "spine_ports": 32, "server_ports": 48, "server_gbps": 25, "uplink_gbps": 400},
    {"name": "cheap", "spines": 2, "leaves": 8, "spine_ports": 16, "server_ports": 48, "server_gbps": 10, "uplink_gbps": 40},
]

for d in designs:
    links = d["spines"] * d["leaves"]
    servers = d["leaves"] * d["server_ports"]
    down = d["server_ports"] * d["server_gbps"]
    up = d["spines"] * d["uplink_gbps"]
    ratio = round(down / up, 2)
    problems = []
    if ratio > 3:
        problems.append("TOO OVERSUBSCRIBED")
    if d["leaves"] > d["spine_ports"]:
        problems.append("NOT ENOUGH SPINE PORTS")
    status = ", ".join(problems) if problems else "OK"
    print(f"{d['name']}: {links} fabric links, {servers} server ports, {ratio}:1 -> {status}")

@@exercise Check cloud security group rules
Security group rules are `(direction, protocol, port, source)`. Flag risky inbound rules: management ports (22, 3389) or database ports (1433, 3306, 5432) open to `0.0.0.0/0`, and any rule allowing all ports (`port` is `"all"`) from anywhere. Web ports 80 and 443 from anywhere are fine on the `web` group only. Print each group's findings or `no findings`.
@@lang python
@@starter
MGMT = {22, 3389}
DB = {1433, 3306, 5432}
groups = {
    "web": [("in", "tcp", 443, "0.0.0.0/0"), ("in", "tcp", 80, "0.0.0.0/0"), ("in", "tcp", 22, "0.0.0.0/0")],
    "app": [("in", "tcp", 8080, "10.0.1.0/24"), ("in", "tcp", 443, "0.0.0.0/0")],
    "db": [("in", "tcp", 5432, "10.0.2.0/24"), ("in", "tcp", 3306, "0.0.0.0/0")],
    "bastion": [("in", "tcp", 22, "198.51.100.7/32"), ("out", "all", "all", "0.0.0.0/0")],
    "legacy": [("in", "all", "all", "0.0.0.0/0")],
}
@@expected
web: management port 22 open to the internet
app: web port 443 open on a non-web group
db: database port 3306 open to the internet
bastion: no findings
legacy: all ports open to the internet
@@requires
MGMT
DB
"0.0.0.0/0"
@@hint
Skip outbound rules. Check `source == "0.0.0.0/0"` first, then classify the port.
@@solution
MGMT = {22, 3389}
DB = {1433, 3306, 5432}
groups = {
    "web": [("in", "tcp", 443, "0.0.0.0/0"), ("in", "tcp", 80, "0.0.0.0/0"), ("in", "tcp", 22, "0.0.0.0/0")],
    "app": [("in", "tcp", 8080, "10.0.1.0/24"), ("in", "tcp", 443, "0.0.0.0/0")],
    "db": [("in", "tcp", 5432, "10.0.2.0/24"), ("in", "tcp", 3306, "0.0.0.0/0")],
    "bastion": [("in", "tcp", 22, "198.51.100.7/32"), ("out", "all", "all", "0.0.0.0/0")],
    "legacy": [("in", "all", "all", "0.0.0.0/0")],
}

for name, rules in groups.items():
    findings = []
    for direction, proto, port, source in rules:
        if direction != "in" or source != "0.0.0.0/0":
            continue
        if port == "all":
            findings.append("all ports open to the internet")
        elif port in MGMT:
            findings.append(f"management port {port} open to the internet")
        elif port in DB:
            findings.append(f"database port {port} open to the internet")
        elif port in (80, 443) and name != "web":
            findings.append(f"web port {port} open on a non-web group")
    print(f"{name}: " + ("; ".join(findings) if findings else "no findings"))

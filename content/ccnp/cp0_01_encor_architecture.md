@@lesson cp0-encor-architecture CCNP Enterprise: ENCOR and Network Architecture
@@text
## The CCNP Enterprise certification

CCNP Enterprise takes **two exams**:

1. **350-401 ENCOR** (Implementing Cisco Enterprise Network Core Technologies): the core exam, 120 minutes. Passing it also qualifies you for the CCIE Enterprise lab.
2. One **concentration exam** of your choice, for example **300-410 ENARSI** (advanced routing and services), 300-415 ENSDWI (SD-WAN), 300-420 ENSLD (design), 300-425 ENWLSD (wireless design) or 300-435 ENAUTO (automation).

ENCOR's domains: **Architecture** (15%), **Virtualization** (10%), **Infrastructure** (30%: layer 2, layer 3, wireless, IP services), **Network Assurance** (10%), **Security** (20%), **Automation** (15%). It assumes you know CCNA well: VLANs, STP, OSPF and ACLs are only the starting point.

In Japan, CCNP is a strong signal for network engineer (ネットワークエンジニア) roles at SIers, carriers (NTT, KDDI, SoftBank) and large enterprises, often paying well above entry level.

## Campus design: tiers

| Design | Layers | Where |
|---|---|---|
| **Three-tier** | access (users), distribution (policy, routing between VLANs), core (fast transport) | large campuses |
| **Collapsed core** | access + combined core/distribution | small and medium sites |
| **Spine-leaf** | every leaf connects to every spine: any server is two hops from any other | data centres (east-west traffic) |

Design goals: **redundancy** without loops (pairs of distribution switches, first-hop redundancy with HSRP/VRRP/GLBP), a **layer 3 boundary** at distribution (or at access, "routed access", which removes STP from uplinks), and **summarization** at the distribution layer.

**High availability** features: **StackWise** (several switches act as one), **VSS/StackWise Virtual** (two chassis as one logical switch, so EtherChannels span both), **SSO/NSF** (a standby supervisor takes over without dropping routing adjacencies).

## Packet forwarding

- **Process switching**: the CPU handles each packet. Slow.
- **CEF (Cisco Express Forwarding)**: the default. The **FIB** (from the routing table) and the **adjacency table** (from ARP) are precomputed, so forwarding is done in hardware (ASICs and TCAM).
- **TCAM** (ternary content-addressable memory) matches routes and ACLs in one lookup, at line rate.

`show ip cef 10.1.1.0/24 detail` shows the forwarding decision for a prefix.

## Wireless architecture

- **Autonomous APs** (configured one by one) versus **lightweight APs** with a **WLC** (wireless LAN controller), connected by **CAPWAP** tunnels (UDP 5246 control, 5247 data).
- Deployments: centralized (controller in the data centre), **FlexConnect** (APs at branches switch traffic locally if the WAN fails), **embedded wireless controller** on Catalyst 9000 APs or switches, and cloud (Meraki).
- **RF basics** for the exam: 2.4 GHz has only three non-overlapping channels (1, 6, 11); 5 GHz and 6 GHz (Wi-Fi 6E) have many more. **RSSI** around −67 dBm is a typical minimum for voice, and SNR above 25 dB is good. dB arithmetic: +3 dB doubles the power, +10 dB is ten times.
- **Roaming**: intra-controller, inter-controller layer 2, and layer 3 (with anchor/foreign controllers).

## SD-Access and SD-WAN

**Cisco SD-Access** (campus): **Catalyst Center** (formerly DNA Center) manages a **fabric**:

- **Underlay**: a routed IS-IS network between switches.
- **Overlay**: **VXLAN** tunnels carry traffic; **LISP** is the control plane that tracks where each endpoint is.
- **Policy**: users and devices are put in groups (**Scalable Group Tags**, SGTs) by **ISE**, and policy is written between groups, not IP addresses.
- Roles: **control plane node** (LISP map server), **border node** (exits the fabric), **edge node** (where users connect).

**Cisco SD-WAN** (Catalyst SD-WAN, formerly Viptela) for branches:

| Component | Plane | Job |
|---|---|---|
| **vManage** (SD-WAN Manager) | management | the GUI and API: configuration templates, monitoring |
| **vSmart** (SD-WAN Controller) | control | distributes routes and policy with **OMP** |
| **vBond** (SD-WAN Validator) | orchestration | authenticates devices; helps them through NAT |
| **WAN edge** (vEdge / cEdge) | data | the branch routers; IPsec tunnels over any transport (MPLS, internet, LTE) |

The benefit: cheap internet links alongside MPLS, with **application-aware routing** that moves traffic when a link's latency or loss gets bad.

## Virtualization

- **Hypervisors**: type 1 (bare metal: ESXi, Hyper-V, KVM) versus type 2 (on an OS: VirtualBox).
- **Virtual switching**: vSwitches inside the host; **NFV** (network functions virtualization) runs routers and firewalls as VMs (Catalyst 8000V, ASAv).
- **VRF** (virtual routing and forwarding): several separate routing tables on one router, for customers or departments with overlapping addresses. `ip vrf forwarding GUEST` on an interface.
- **GRE** tunnels (protocol 47) carry any traffic, including multicast and routing protocols, but aren't encrypted; **IPsec** encrypts (IKEv2, ESP). **GRE over IPsec** or **VTI** (`tunnel mode ipsec ipv4`) gets both.
- **VXLAN**: a layer 2 overlay over layer 3, with a 24-bit VNI (16 million segments, versus 4094 VLANs).

@@quiz
Which ENCOR domain carries the most weight?
* Infrastructure (layer 2, layer 3, wireless and IP services)
- Architecture
- Automation
- Virtualization
? Infrastructure is 30%, Security 20%, Architecture and Automation 15% each.

@@quiz
In Cisco SD-WAN, which component distributes routing information and policy to the WAN edge routers using OMP?
* vSmart (SD-WAN Controller)
- vManage
- vBond
- the WAN edge itself
? vManage is management, vBond orchestration and authentication, vSmart the control plane.

@@quiz
In SD-Access, what is LISP used for?
* The control plane: mapping endpoints to the fabric edge node they're behind
- Encrypting traffic
- The underlay routing protocol
- Assigning SGTs
? The underlay uses IS-IS, the data plane VXLAN, and policy SGTs from ISE.

@@quiz
Why are spine-leaf designs used in data centres?
* Every leaf connects to every spine, giving predictable, low latency for east-west (server-to-server) traffic
- They need fewer cables
- They remove the need for routing
- They're required for Wi-Fi
? Any two servers are always leaf → spine → leaf apart.

@@quiz
Which statement about GRE is true?
* It can carry multicast and routing protocols but provides no encryption on its own
- It encrypts traffic with AES
- It only carries IPv6
- It runs on TCP port 47
? GRE is IP protocol 47. Combine it with IPsec for confidentiality.

@@flashcards
350-401 ENCOR :: CCNP Enterprise core exam (also the CCIE written)
ENARSI :: concentration exam: advanced routing and services
collapsed core :: core and distribution in one layer
spine-leaf :: data-centre design: every leaf to every spine
CEF :: hardware forwarding using the FIB and adjacency table
CAPWAP :: AP-to-WLC tunnel (UDP 5246/5247)
FlexConnect :: branch APs switch locally when the WAN fails
SD-Access underlay / overlay :: IS-IS / VXLAN with LISP
SGT :: Scalable Group Tag: policy by group, from ISE
vManage / vSmart / vBond :: SD-WAN management / control / orchestration
OMP :: Overlay Management Protocol (SD-WAN routing)
VRF :: separate routing tables on one router
VXLAN VNI :: 24-bit segment ID: 16 million segments

@@drill
How many dB of gain doubles the signal power?
= 3
= 3 dB
? +3 dB doubles, +10 dB is ten times; −3 dB halves.

@@drill
Which IP protocol number does GRE use?
= 47

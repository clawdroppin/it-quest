@@lesson c1-routing Configuring Routing: Static, OSPF, Inter-VLAN and FHRP
@@text
## Reading the routing table

```text
R1# show ip route
Codes: C - connected, L - local, S - static, O - OSPF, * - candidate default
Gateway of last resort is 203.0.113.1 to network 0.0.0.0

S*    0.0.0.0/0 [1/0] via 203.0.113.1
      10.0.0.0/8 is variably subnetted
C        10.1.1.0/24 is directly connected, GigabitEthernet0/0
L        10.1.1.1/32 is directly connected, GigabitEthernet0/0
O        10.2.0.0/16 [110/2] via 10.1.12.2, 00:12:44, GigabitEthernet0/1
```

`[110/2]` = **[administrative distance / metric]**. The router picks the **longest prefix match**; among equal prefixes from different sources, the **lowest AD**; within one protocol, the **lowest metric**.

## Static routes

```text
ip route 10.2.0.0 255.255.0.0 10.1.12.2           ! next hop
ip route 0.0.0.0 0.0.0.0 203.0.113.1              ! default route
ip route 10.2.0.0 255.255.0.0 10.1.13.3 5         ! floating static: AD 5, a backup path
ipv6 route ::/0 2001:db8:12::2
```

## OSPF (single area)

```text
router ospf 1
 router-id 1.1.1.1
 network 10.1.1.0 0.0.0.255 area 0          ! wildcard mask = inverse of the subnet mask
 network 10.1.12.0 0.0.0.3 area 0
 passive-interface g0/0                     ! advertise the LAN, but send no hellos there
 default-information originate              ! share this router's default route
! or per interface:  interface g0/1 → ip ospf 1 area 0
```

Neighbours must agree on **area**, **subnet**, **hello/dead timers** (10/40 on Ethernet), authentication and MTU. The **router ID** is set manually, else the highest loopback IP, else the highest active interface IP. On multi-access networks a **DR** and **BDR** are elected (highest priority, then router ID). Cost = reference bandwidth (100 Mb/s) ÷ interface bandwidth, so set `auto-cost reference-bandwidth 10000` on gigabit networks. Check with `show ip ospf neighbor` (look for **FULL**), `show ip ospf interface brief`, `show ip protocols`.

## Inter-VLAN routing

**Router on a stick** (one trunk to a router with sub-interfaces):

```text
interface g0/0.10
 encapsulation dot1Q 10
 ip address 10.10.10.1 255.255.255.0
interface g0/0.20
 encapsulation dot1Q 20
 ip address 10.10.20.1 255.255.255.0
```

Or on a **layer 3 switch** with SVIs: `ip routing`, then `interface vlan 10` → `ip address ...` → `no shutdown`. A **routed port**: `no switchport` then an IP.

## First hop redundancy: HSRP

```text
interface vlan 10
 ip address 10.10.10.2 255.255.255.0
 standby 10 ip 10.10.10.1          ! the virtual gateway hosts use
 standby 10 priority 110           ! higher wins (default 100)
 standby 10 preempt                ! take over again after recovering
```

The other router has `.3`, the same `standby 10 ip`, and the default priority. **VRRP** is the open standard; **GLBP** (Cisco) also load-balances.

@@quiz
A routing table shows `O 10.2.0.0/16 [110/20]`. What do 110 and 20 mean?
* Administrative distance 110 (OSPF) and metric (cost) 20
- Metric 110 and hop count 20
- VLAN 110 and port 20
- Area 110 and router ID 20
? [AD/metric].

@@quiz
What's the wildcard mask for a /30 network in an OSPF network statement?
* 0.0.0.3
- 255.255.255.252
- 0.0.0.252
- 0.0.0.4
? Wildcard = 255.255.255.255 − subnet mask.

@@quiz
Two OSPF routers on the same link don't become neighbours. Which mismatch would cause this?
* Different hello/dead timers or different areas on the link
- Different router IDs
- Different hostnames
- Different interface descriptions
? Router IDs must be *different*; timers, area, subnet and authentication must match.

@@quiz
A backup internet link should be used only if the primary static default route fails. How?
* A second default route with a higher administrative distance (a floating static route)
- A second default route with the same AD
- An OSPF network statement
- HSRP on the WAN
? The higher-AD route is installed only when the better one disappears.

@@quiz
In router-on-a-stick, which command tags a sub-interface for VLAN 20?
* encapsulation dot1Q 20
- switchport access vlan 20
- vlan 20
- standby 20
? Each sub-interface handles one VLAN on the trunk.

@@quiz
An HSRP router with priority 110 recovers after a failure but doesn't become active again. What's missing?
* standby preempt
- A higher hello timer
- ip routing
- passive-interface
? Without preempt, the current active router keeps the role.

@@quiz
How is the OSPF router ID chosen when none is configured?
* The highest loopback IP, otherwise the highest active physical interface IP
- The lowest MAC address
- The first interface configured
- Randomly
? Configure it explicitly so it never changes unexpectedly.

@@flashcards
[AD/metric] :: Shown in brackets in show ip route
connected / static / OSPF AD :: 0 / 1 / 110
ip route 0.0.0.0 0.0.0.0 next-hop :: Default route
floating static route :: Static route with a higher AD as a backup
wildcard mask :: Inverse of the subnet mask
passive-interface :: Advertise a network but send no hellos there
OSPF neighbour requirements :: Same area, subnet, timers, authentication, MTU
OSPF router ID :: Manual, else highest loopback, else highest interface IP
DR / BDR :: Designated routers on multi-access networks
show ip ospf neighbor :: Neighbour states (want FULL)
router on a stick :: Sub-interfaces with encapsulation dot1Q
ip routing :: Enable routing on a layer 3 switch
HSRP :: Cisco FHRP: standby group, virtual IP, priority, preempt
VRRP / GLBP :: Open-standard FHRP / Cisco load-balancing FHRP

@@drill
What wildcard mask matches a /24 network?
= 0.0.0.255
? 255.255.255.255 − 255.255.255.0.

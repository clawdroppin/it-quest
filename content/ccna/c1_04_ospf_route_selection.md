@@lesson c1-ospf-deep OSPF in Depth and How Routers Choose Routes
@@text
IP connectivity is the largest CCNA domain (25%). You need to read a routing table cold, explain why one route beats another, and understand OSPFv2 neighbours, DR/BDR elections, cost and the reasons adjacencies fail. This lesson goes beyond the basic OSPF configuration.

## How a router picks a route

Three separate decisions, in this order:

1. **Longest prefix match** decides which route is *used* for a packet. A /28 beats a /24 beats a /0, regardless of protocol or metric.
2. **Administrative distance (AD)** decides which source's route goes into the routing table when two protocols offer the **same prefix and length**.
3. **Metric** decides between routes to the same prefix from the **same** protocol (equal metrics → **ECMP**, load sharing).

| Source | AD |
|---|---|
| Connected | 0 |
| Static | 1 |
| eBGP | 20 |
| EIGRP (internal) | 90 |
| OSPF | 110 |
| IS-IS | 115 |
| RIP | 120 |
| EIGRP external | 170 |
| iBGP | 200 |
| Unusable | 255 |

A **floating static route** has a higher AD than the dynamic route (`ip route 0.0.0.0 0.0.0.0 203.0.113.1 130`), so it's only installed when the OSPF route disappears: a backup path.

## Reading `show ip route`

```text
Gateway of last resort is 10.0.0.1 to network 0.0.0.0

O*E2  0.0.0.0/0 [110/1] via 10.0.0.1, 00:12:44, GigabitEthernet0/0
      10.0.0.0/8 is variably subnetted, 6 subnets, 3 masks
C        10.0.0.0/30 is directly connected, GigabitEthernet0/0
L        10.0.0.2/32 is directly connected, GigabitEthernet0/0
O        10.1.0.0/16 [110/2] via 10.0.0.1, 00:12:44, GigabitEthernet0/0
O        10.1.5.0/24 [110/3] via 10.0.0.5, 00:03:10, GigabitEthernet0/1
S        10.1.5.64/26 [1/0] via 10.0.0.9
O IA     10.2.0.0/16 [110/12] via 10.0.0.1, 00:12:44, GigabitEthernet0/0
```

`[110/3]` = [AD/metric]. Codes: `C` connected, `L` local (/32 of the interface's own address), `S` static, `O` OSPF intra-area, `O IA` inter-area, `O E1/E2` external, `*` candidate default. A packet to 10.1.5.70 matches the /16, /24 and /26: the **/26 static** wins on prefix length.

## OSPF essentials

- **Link-state**: each router floods **LSAs**, builds the same **LSDB** for the area and runs **SPF (Dijkstra)**.
- **Areas**: area 0 is the **backbone**; other areas attach to it through **ABRs**. **ASBRs** inject external routes. CCNA focuses on single-area OSPFv2.
- **Router ID**: manually set (`router-id 1.1.1.1`) > highest loopback IP > highest active interface IP. Changes need `clear ip ospf process`.
- **Cost** = reference bandwidth / interface bandwidth; default reference **100 Mb/s**, so FastEthernet, Gigabit and 10 Gigabit all cost **1**. Fix with `auto-cost reference-bandwidth 100000` (in Mb/s) on **every** router, or `ip ospf cost` per interface.

## Neighbours and adjacencies

States: **Down → Init → 2-Way → ExStart → Exchange → Loading → Full**. Hellos every **10 s**, dead after **40 s** on broadcast/point-to-point links. Sent to **224.0.0.5** (all OSPF routers); DR/BDR listen on **224.0.0.6**.

To become neighbours, these must **match**: area ID, **subnet and mask**, **hello/dead timers**, authentication, area type (stub flags), and MTU (mismatch sticks in ExStart/Exchange). Router IDs must be **unique**. A **passive interface** sends no hellos (use on LAN/user ports and loopbacks).

## DR and BDR

On multi-access (Ethernet) segments, routers elect a **Designated Router** and **Backup DR** to reduce flooding: others ("DROTHERs") are Full only with the DR/BDR and stay **2-Way** with each other (that's normal).

- Highest **interface OSPF priority** wins (default 1; **0** = never DR), then highest **router ID**.
- **Not preemptive**: a better router that joins later doesn't take over until the DR fails or the process is cleared.
- `ip ospf network point-to-point` on a link between two routers removes the election and speeds up adjacency.

```text
R1# show ip ospf neighbor
Neighbor ID     Pri   State           Dead Time   Address         Interface
2.2.2.2           1   FULL/DR         00:00:35    10.1.1.2        Gi0/0
3.3.3.3           1   2WAY/DROTHER    00:00:33    10.1.1.3        Gi0/0
4.4.4.4           0   FULL/  -        00:00:38    10.0.0.6        Gi0/1
```

## Configuration patterns

```text
router ospf 1
 router-id 1.1.1.1
 auto-cost reference-bandwidth 100000
 passive-interface default
 no passive-interface g0/0
 network 10.1.1.0 0.0.0.255 area 0
 default-information originate          ! advertise this router's default route
interface g0/1
 ip ospf 1 area 0                       ! interface-level alternative to network
 ip ospf network point-to-point
 ip ospf priority 0
```

## FHRP recap

HSRP (Cisco: active/standby, virtual MAC 0000.0c07.acXX v1), VRRP (open: master/backup), GLBP (Cisco: load balances across gateways). Higher priority wins (default 100); `preempt` lets it take the role back.

@@quiz
A router has 10.1.0.0/16 via OSPF and 10.1.5.0/24 via RIP. Which is used for a packet to 10.1.5.9?
- The OSPF route, because its AD is lower
* The RIP /24, because it is the longest prefix match
- Both, load balanced
- Neither, they conflict
? AD only compares routes to the same prefix.

@@quiz
Static and OSPF routes exist for the same prefix. Which goes in the routing table by default?
* The static route (AD 1)
- The OSPF route (AD 110)
- Both
- The one learned first
? Lower AD wins.

@@quiz
Why do Fast Ethernet and 10 Gigabit interfaces both have OSPF cost 1 by default?
- OSPF ignores bandwidth
* The default reference bandwidth is 100 Mb/s, so anything ≥ 100 Mb/s costs 1
- Cost is based on delay
- They're both copper
? Raise it with auto-cost reference-bandwidth.

@@quiz
Two OSPF routers on the same LAN stay in EXSTART. What is the most likely cause?
- Area mismatch
* MTU mismatch
- Different process IDs
- Hello timer mismatch
? Timer or area mismatches prevent neighbours from forming at all.

@@quiz
On a LAN with a DR and BDR, two DROTHERs show "2WAY/DROTHER". What does it mean?
- A fault: they should be FULL
* Normal: DROTHERs only become FULL with the DR and BDR
- An MTU problem
- Authentication failed
? This reduces flooding on the segment.

@@quiz
What does an interface OSPF priority of 0 do?
* Prevents the router from becoming DR or BDR on that segment
- Makes it the DR
- Disables OSPF
- Sets the cost to 0
? Default priority is 1.

@@flashcards
route choice order :: longest prefix, AD, metric
AD connected / static :: 0 / 1
AD eBGP / EIGRP :: 20 / 90
AD OSPF / RIP :: 110 / 120
AD iBGP :: 200
floating static :: higher AD backup route
[110/3] :: AD / metric
L route :: interface's own /32
O IA :: inter-area OSPF
O E2 :: external OSPF
router ID order :: manual, loopback, highest interface
OSPF cost :: reference bw / interface bw
default reference bw :: 100 Mb/s
hello / dead :: 10 / 40 s
224.0.0.5 / .6 :: all OSPF / DR routers
must match for adjacency :: area, subnet, timers, auth, MTU
stuck in ExStart :: MTU mismatch
DR election :: priority, then router ID
DR preemption :: none
passive-interface :: no hellos sent

@@drill
What is the administrative distance of OSPF?
= 110
? EIGRP is 90.

@@drill
What is the default OSPF dead interval on Ethernet, in seconds?
= 40
? Four times the hello.

@@exercise Forwarding decision with longest prefix match and AD
Build the routing table from `candidates` (`(prefix, source, metric, next_hop)`): for each prefix keep only the routes from the source with the lowest AD, then the lowest metric (keep all equal ones for ECMP). Print the installed table, then for each destination find the longest matching prefix and print its next hop(s). Use the `ipaddress` module.
@@lang python
@@starter
import ipaddress

AD = {"connected": 0, "static": 1, "eigrp": 90, "ospf": 110, "rip": 120}
candidates = [
    ("0.0.0.0/0", "ospf", 1, "10.0.0.1"),
    ("0.0.0.0/0", "static", 0, "203.0.113.1"),
    ("10.1.0.0/16", "ospf", 2, "10.0.0.1"),
    ("10.1.5.0/24", "ospf", 3, "10.0.0.5"),
    ("10.1.5.0/24", "ospf", 3, "10.0.0.9"),
    ("10.1.5.0/24", "rip", 1, "10.0.0.13"),
    ("10.1.5.64/26", "static", 0, "10.0.0.9"),
    ("10.0.0.0/30", "connected", 0, "Gi0/0"),
]
destinations = ["10.1.5.70", "10.1.5.20", "10.1.200.1", "8.8.8.8", "10.0.0.1"]
@@expected
Routing table:
  static    0.0.0.0/0       [1/0] via 203.0.113.1
  connected 10.0.0.0/30     [0/0] via Gi0/0
  ospf      10.1.0.0/16     [110/2] via 10.0.0.1
  ospf      10.1.5.0/24     [110/3] via 10.0.0.5, 10.0.0.9
  static    10.1.5.64/26    [1/0] via 10.0.0.9
10.1.5.70 -> 10.1.5.64/26 via 10.0.0.9
10.1.5.20 -> 10.1.5.0/24 via 10.0.0.5, 10.0.0.9
10.1.200.1 -> 10.1.0.0/16 via 10.0.0.1
8.8.8.8 -> 0.0.0.0/0 via 203.0.113.1
10.0.0.1 -> 10.0.0.0/30 via Gi0/0
@@requires
prefixlen
AD[
@@hint
Group by prefix, find `best = min((AD[src], metric) ...)`, keep routes equal to `best`. For lookup, filter networks where `ip in net` and take the one with the largest `net.prefixlen`.
@@solution
import ipaddress

AD = {"connected": 0, "static": 1, "eigrp": 90, "ospf": 110, "rip": 120}
candidates = [
    ("0.0.0.0/0", "ospf", 1, "10.0.0.1"),
    ("0.0.0.0/0", "static", 0, "203.0.113.1"),
    ("10.1.0.0/16", "ospf", 2, "10.0.0.1"),
    ("10.1.5.0/24", "ospf", 3, "10.0.0.5"),
    ("10.1.5.0/24", "ospf", 3, "10.0.0.9"),
    ("10.1.5.0/24", "rip", 1, "10.0.0.13"),
    ("10.1.5.64/26", "static", 0, "10.0.0.9"),
    ("10.0.0.0/30", "connected", 0, "Gi0/0"),
]
destinations = ["10.1.5.70", "10.1.5.20", "10.1.200.1", "8.8.8.8", "10.0.0.1"]

by_prefix = {}
for prefix, source, metric, hop in candidates:
    by_prefix.setdefault(prefix, []).append((AD[source], metric, source, hop))

table = {}
for prefix, routes in by_prefix.items():
    best = min((ad, metric) for ad, metric, _, _ in routes)
    table[ipaddress.ip_network(prefix)] = [r for r in routes if (r[0], r[1]) == best]

print("Routing table:")
for net in sorted(table, key=lambda n: (n.network_address, n.prefixlen)):
    routes = table[net]
    hops = ", ".join(r[3] for r in routes)
    print(f"  {routes[0][2]:<9} {str(net):<15} [{routes[0][0]}/{routes[0][1]}] via {hops}")

for dest in destinations:
    ip = ipaddress.ip_address(dest)
    matches = [net for net in table if ip in net]
    net = max(matches, key=lambda n: n.prefixlen)
    hops = ", ".join(r[3] for r in table[net])
    print(f"{dest} -> {net} via {hops}")

@@exercise Diagnose OSPF neighbour problems
Each pair of interface configs in `links` should form an OSPF adjacency. Compare the parameters that must match (`area`, network from `ip`/`mask`, `hello`, `dead`, `auth`, `mtu`) and check for duplicate router IDs and passive interfaces. Print every problem for each link with the state you'd expect (`no neighbour` for most, `stuck in EXSTART` for MTU), or `FULL expected`. Then, for the LAN segment `lan`, elect the DR and BDR (highest priority, then highest router ID compared numerically; priority 0 never wins).
@@lang python
@@starter
import ipaddress

links = [
    ({"rid": "1.1.1.1", "ip": "10.0.12.1", "mask": 30, "area": 0, "hello": 10, "dead": 40, "auth": None, "mtu": 1500, "passive": False},
     {"rid": "2.2.2.2", "ip": "10.0.12.2", "mask": 30, "area": 0, "hello": 10, "dead": 40, "auth": None, "mtu": 1500, "passive": False}),
    ({"rid": "1.1.1.1", "ip": "10.0.13.1", "mask": 30, "area": 0, "hello": 10, "dead": 40, "auth": None, "mtu": 1500, "passive": False},
     {"rid": "3.3.3.3", "ip": "10.0.13.2", "mask": 30, "area": 1, "hello": 5, "dead": 20, "auth": None, "mtu": 1500, "passive": False}),
    ({"rid": "2.2.2.2", "ip": "10.0.24.1", "mask": 30, "area": 0, "hello": 10, "dead": 40, "auth": None, "mtu": 1500, "passive": False},
     {"rid": "4.4.4.4", "ip": "10.0.24.2", "mask": 30, "area": 0, "hello": 10, "dead": 40, "auth": None, "mtu": 9000, "passive": False}),
    ({"rid": "3.3.3.3", "ip": "10.0.34.1", "mask": 24, "area": 0, "hello": 10, "dead": 40, "auth": "md5", "mtu": 1500, "passive": True},
     {"rid": "3.3.3.3", "ip": "10.0.34.2", "mask": 30, "area": 0, "hello": 10, "dead": 40, "auth": None, "mtu": 1500, "passive": False}),
]
lan = [("1.1.1.1", 1), ("2.2.2.2", 1), ("5.5.5.5", 0), ("4.4.4.4", 1), ("3.3.3.3", 50)]
@@expected
1.1.1.1 10.0.12.1 <-> 2.2.2.2 10.0.12.2: FULL expected
1.1.1.1 10.0.13.1 <-> 3.3.3.3 10.0.13.2: no neighbour: area 0 vs 1, hello 10 vs 5, dead 40 vs 20
2.2.2.2 10.0.24.1 <-> 4.4.4.4 10.0.24.2: stuck in EXSTART: mtu 1500 vs 9000
3.3.3.3 10.0.34.1 <-> 3.3.3.3 10.0.34.2: no neighbour: passive interface, duplicate router ID, subnet 10.0.34.0/24 vs 10.0.34.0/30, auth md5 vs None
DR 3.3.3.3, BDR 4.4.4.4
@@requires
def check
ip_interface(
@@hint
Compare `ipaddress.ip_interface(f"{a['ip']}/{a['mask']}").network` on both sides. For the election, sort the eligible routers by `(priority, ipaddress.ip_address(rid))` descending.
@@solution
import ipaddress

links = [
    ({"rid": "1.1.1.1", "ip": "10.0.12.1", "mask": 30, "area": 0, "hello": 10, "dead": 40, "auth": None, "mtu": 1500, "passive": False},
     {"rid": "2.2.2.2", "ip": "10.0.12.2", "mask": 30, "area": 0, "hello": 10, "dead": 40, "auth": None, "mtu": 1500, "passive": False}),
    ({"rid": "1.1.1.1", "ip": "10.0.13.1", "mask": 30, "area": 0, "hello": 10, "dead": 40, "auth": None, "mtu": 1500, "passive": False},
     {"rid": "3.3.3.3", "ip": "10.0.13.2", "mask": 30, "area": 1, "hello": 5, "dead": 20, "auth": None, "mtu": 1500, "passive": False}),
    ({"rid": "2.2.2.2", "ip": "10.0.24.1", "mask": 30, "area": 0, "hello": 10, "dead": 40, "auth": None, "mtu": 1500, "passive": False},
     {"rid": "4.4.4.4", "ip": "10.0.24.2", "mask": 30, "area": 0, "hello": 10, "dead": 40, "auth": None, "mtu": 9000, "passive": False}),
    ({"rid": "3.3.3.3", "ip": "10.0.34.1", "mask": 24, "area": 0, "hello": 10, "dead": 40, "auth": "md5", "mtu": 1500, "passive": True},
     {"rid": "3.3.3.3", "ip": "10.0.34.2", "mask": 30, "area": 0, "hello": 10, "dead": 40, "auth": None, "mtu": 1500, "passive": False}),
]
lan = [("1.1.1.1", 1), ("2.2.2.2", 1), ("5.5.5.5", 0), ("4.4.4.4", 1), ("3.3.3.3", 50)]

def check(a, b):
    problems = []
    if a["passive"] or b["passive"]:
        problems.append("passive interface")
    if a["rid"] == b["rid"]:
        problems.append("duplicate router ID")
    net_a = ipaddress.ip_interface(f"{a['ip']}/{a['mask']}").network
    net_b = ipaddress.ip_interface(f"{b['ip']}/{b['mask']}").network
    if net_a != net_b:
        problems.append(f"subnet {net_a} vs {net_b}")
    for key in ("area", "hello", "dead", "auth"):
        if a[key] != b[key]:
            problems.append(f"{key} {a[key]} vs {b[key]}")
    if problems:
        return "no neighbour: " + ", ".join(problems)
    if a["mtu"] != b["mtu"]:
        return f"stuck in EXSTART: mtu {a['mtu']} vs {b['mtu']}"
    return "FULL expected"

for a, b in links:
    print(f"{a['rid']} {a['ip']} <-> {b['rid']} {b['ip']}: {check(a, b)}")

eligible = sorted((r for r in lan if r[1] > 0), key=lambda r: (r[1], ipaddress.ip_address(r[0])), reverse=True)
print(f"DR {eligible[0][0]}, BDR {eligible[1][0]}")

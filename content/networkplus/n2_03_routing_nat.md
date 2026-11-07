@@lesson n2-routing Routing Protocols, Route Selection, NAT and FHRP
@@text
The switching lesson introduced routing. This one covers what Network+ actually asks: how a router picks between routes, how the main dynamic protocols differ, how NAT and PAT translate addresses, and how first-hop redundancy keeps the gateway up.

## How a router chooses a route

For each packet, in this order:

1. **Longest prefix match**: the most specific route wins. A packet to 10.1.2.5 matches both `10.0.0.0/8` and `10.1.2.0/24`; the **/24** wins.
2. If the same prefix is learned from different sources, the lowest **administrative distance (AD)** wins (how trustworthy the source is).
3. If the same protocol offers several paths, the lowest **metric** wins (or equal metrics are load-balanced: **ECMP**).

| Source | Default AD (Cisco) |
|---|---|
| Directly connected | 0 |
| Static route | 1 |
| eBGP | 20 |
| EIGRP (internal) | 90 |
| OSPF | 110 |
| RIP | 120 |
| iBGP | 200 |
| Unusable | 255 |

The **default route** `0.0.0.0/0` (`::/0`) matches everything and is used when nothing more specific exists: the "gateway of last resort".

## Static vs dynamic routing

- **Static**: configured by hand. Predictable, no overhead, doesn't adapt. Used for default routes, stub networks, and **floating static** routes (a higher AD backup, e.g. AD 200 over LTE that only appears when the main route disappears).
- **Dynamic**: routers exchange routes and adapt to failures.

## Dynamic routing protocols

| Protocol | Type | Metric | Use |
|---|---|---|---|
| **RIP v2** | distance vector | hop count (max 15) | legacy, small |
| **OSPF** | link state | cost (based on bandwidth) | enterprise IGP; areas (area 0 backbone); fast convergence |
| **EIGRP** | advanced distance vector (hybrid) | bandwidth + delay | Cisco networks |
| **BGP** | path vector | attributes (AS path, local preference, MED...) | between organisations (**EGP**), the internet; also inside large data centres |
| **IS-IS** | link state | cost | service provider cores |

- **IGP** (interior: OSPF, EIGRP, RIP, IS-IS) vs **EGP** (exterior: BGP).
- **Convergence**: the time for all routers to agree after a change. Link state converges fast.
- **Autonomous system (AS)**: a network under one administration with a public **ASN**.
- Routing loops are prevented by split horizon, route poisoning and TTL (the packet's TTL drops by 1 per hop; at 0 it's discarded and an ICMP Time Exceeded is sent, which is how `traceroute` works).

## NAT and PAT

- **Static NAT**: one private ↔ one public address (publishing a server).
- **Dynamic NAT**: private addresses take from a pool of public ones.
- **PAT / NAT overload**: many private hosts share **one** public IP, distinguished by **source port**. This is what every home and office router does.
- **Port forwarding**: inbound traffic to public IP:port goes to an internal host:port.
- **NAT64** helps IPv6-only clients reach IPv4 services.
- Terms: **inside local** (the host's private IP), **inside global** (its public IP as seen outside).

## First hop redundancy (FHRP)

If the single default gateway fails, a whole subnet is offline. FHRPs let two or more routers share a **virtual IP** (and virtual MAC) that hosts use as their gateway:

- **VRRP** (open standard), **HSRP** (Cisco), **GLBP** (Cisco, load balances).
- One is **active/master**, others **standby/backup**; they exchange hellos and take over within seconds.

## Other routing concepts

- **Subinterfaces** and **router-on-a-stick**: one physical link carries several VLANs (802.1Q) to a router.
- **Layer 3 switches** and **SVIs** (switch virtual interfaces) route between VLANs at wire speed.
- **QoS**: classify and mark (DSCP), queue, police/shape traffic so voice and video get priority. Voice wants < 150 ms one-way latency, < 30 ms jitter, < 1% loss.
- **Traffic shaping** delays excess traffic (smooths it); **policing** drops it.

@@quiz
A router has routes `10.0.0.0/8` via R1, `10.20.0.0/16` via R2 and `10.20.30.0/24` via R3. Where does it send a packet for 10.20.30.40?
- R1
- R2
* R3
- It load balances across all three
? Longest prefix match.

@@quiz
The same prefix is learned from OSPF and from a static route. Which is installed by default?
* The static route (AD 1)
- The OSPF route (AD 110)
- Both, load balanced
- Neither
? Lower administrative distance is more trusted.

@@quiz
A company wants an LTE backup link used only when the fibre route disappears. What's the simplest configuration?
- Two equal static routes
* A floating static route with a higher administrative distance via LTE
- RIP over LTE
- PAT
? It stays out of the table until the primary route is gone.

@@quiz
Which protocol exchanges routes between internet service providers?
- OSPF
- RIP
* BGP
- EIGRP
? BGP is the internet's exterior gateway protocol.

@@quiz
200 office PCs reach the internet through one public IP. Which translation is used?
- Static NAT
- Dynamic NAT
* PAT (NAT overload)
- NAT64
? Source ports distinguish the sessions.

@@flashcards
longest prefix match :: most specific route wins
administrative distance :: trust of a route source; lower wins
AD connected / static :: 0 / 1
AD eBGP / EIGRP / OSPF / RIP :: 20 / 90 / 110 / 120
metric :: picks among paths from one protocol
ECMP :: load balance equal-cost paths
default route :: 0.0.0.0/0
floating static :: backup route with higher AD
OSPF :: link state, cost, areas
BGP :: path vector, between ASes
RIP max hops :: 15
PAT :: many hosts, one public IP, by port
port forwarding :: inbound public port to an internal host
VRRP / HSRP :: virtual gateway redundancy
SVI :: VLAN interface on a layer 3 switch
shaping vs policing :: delay vs drop excess

@@drill
What is OSPF's default administrative distance?
= 110
? EIGRP is 90, RIP is 120.

@@drill
What is the maximum hop count RIP allows before a route is unreachable?
= 15
? 16 means infinity.

@@exercise Build a routing decision engine
Implement `lookup(table, ip)` that returns the chosen route for an IP: filter routes that contain the IP, keep the longest prefix, then the lowest AD, then the lowest metric. Return `None` if nothing matches. Print the decision for each destination. Use the `ipaddress` module.
@@lang python
@@starter
import ipaddress

table = [
    ("0.0.0.0/0", "ISP", "static", 1, 0),
    ("10.0.0.0/8", "R1", "ospf", 110, 20),
    ("10.20.0.0/16", "R2", "ospf", 110, 30),
    ("10.20.0.0/16", "R4", "static", 1, 0),
    ("10.20.30.0/24", "R3", "rip", 120, 2),
    ("10.20.30.0/24", "R5", "ospf", 110, 40),
    ("10.20.30.0/24", "R6", "ospf", 110, 10),
]
destinations = ["10.20.30.40", "10.20.99.1", "10.99.0.1", "8.8.8.8"]

def lookup(table, ip):
    raise NotImplementedError
@@expected
10.20.30.40 -> R6 via 10.20.30.0/24 (ospf, AD 110, metric 10)
10.20.99.1 -> R4 via 10.20.0.0/16 (static, AD 1, metric 0)
10.99.0.1 -> R1 via 10.0.0.0/8 (ospf, AD 110, metric 20)
8.8.8.8 -> ISP via 0.0.0.0/0 (static, AD 1, metric 0)
@@requires
def lookup
prefixlen
@@hint
`candidates = [r for r in table if addr in ipaddress.ip_network(r[0])]`, then `min(candidates, key=lambda r: (-ipaddress.ip_network(r[0]).prefixlen, r[3], r[4]))`.
@@solution
import ipaddress

table = [
    ("0.0.0.0/0", "ISP", "static", 1, 0),
    ("10.0.0.0/8", "R1", "ospf", 110, 20),
    ("10.20.0.0/16", "R2", "ospf", 110, 30),
    ("10.20.0.0/16", "R4", "static", 1, 0),
    ("10.20.30.0/24", "R3", "rip", 120, 2),
    ("10.20.30.0/24", "R5", "ospf", 110, 40),
    ("10.20.30.0/24", "R6", "ospf", 110, 10),
]
destinations = ["10.20.30.40", "10.20.99.1", "10.99.0.1", "8.8.8.8"]

def lookup(table, ip):
    addr = ipaddress.ip_address(ip)
    candidates = [r for r in table if addr in ipaddress.ip_network(r[0])]
    if not candidates:
        return None
    return min(candidates, key=lambda r: (-ipaddress.ip_network(r[0]).prefixlen, r[3], r[4]))

for dest in destinations:
    route = lookup(table, dest)
    prefix, hop, proto, ad, metric = route
    print(f"{dest} -> {hop} via {prefix} ({proto}, AD {ad}, metric {metric})")

@@exercise Simulate PAT
A router with public IP 203.0.113.5 translates outbound connections. Each new `(inside_ip, inside_port, destination)` gets the next free public port starting at 50000; repeated connections reuse their entry. Then deliver the inbound replies: a reply to a public port in the table goes to the matching inside host; anything else is dropped. Print the translation table and the reply results.
@@lang python
@@starter
PUBLIC_IP = "203.0.113.5"
outbound = [
    ("192.168.1.10", 51515, "93.184.216.34:443"),
    ("192.168.1.11", 51515, "93.184.216.34:443"),
    ("192.168.1.10", 51600, "8.8.8.8:53"),
    ("192.168.1.10", 51515, "93.184.216.34:443"),
]
replies = [50000, 50001, 50002, 50077]
@@expected
192.168.1.10:51515 -> 93.184.216.34:443 as 203.0.113.5:50000
192.168.1.11:51515 -> 93.184.216.34:443 as 203.0.113.5:50001
192.168.1.10:51600 -> 8.8.8.8:53 as 203.0.113.5:50002
192.168.1.10:51515 -> 93.184.216.34:443 as 203.0.113.5:50000
reply to 203.0.113.5:50000 from 93.184.216.34:443 -> 192.168.1.10:51515
reply to 203.0.113.5:50001 from 93.184.216.34:443 -> 192.168.1.11:51515
reply to 203.0.113.5:50002 from 8.8.8.8:53 -> 192.168.1.10:51600
reply to 203.0.113.5:50077 -> dropped (no translation)
@@requires
PUBLIC_IP
50000
+= 1
@@hint
Keep `table = {}` keyed by `(ip, port, dest)` and `reverse = {}` from public port back to the key.
@@solution
PUBLIC_IP = "203.0.113.5"
outbound = [
    ("192.168.1.10", 51515, "93.184.216.34:443"),
    ("192.168.1.11", 51515, "93.184.216.34:443"),
    ("192.168.1.10", 51600, "8.8.8.8:53"),
    ("192.168.1.10", 51515, "93.184.216.34:443"),
]
replies = [50000, 50001, 50002, 50077]

table = {}
reverse = {}
next_port = 50000
for key in outbound:
    if key not in table:
        table[key] = next_port
        reverse[next_port] = key
        next_port += 1
    ip, port, dest = key
    print(f"{ip}:{port} -> {dest} as {PUBLIC_IP}:{table[key]}")

for port in replies:
    if port in reverse:
        ip, inside_port, dest = reverse[port]
        print(f"reply to {PUBLIC_IP}:{port} from {dest} -> {ip}:{inside_port}")
    else:
        print(f"reply to {PUBLIC_IP}:{port} -> dropped (no translation)")

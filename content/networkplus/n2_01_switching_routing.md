@@lesson n2-switching Switching, VLANs and Routing
@@text
## How a switch works

A switch learns which MAC address is on which port (its **MAC address table** / CAM table) by reading the *source* MAC of each frame. Frames to a known MAC go out one port; unknown destinations and broadcasts are **flooded** out of all ports in the VLAN. **ARP** finds the MAC for an IP address with a broadcast: "who has 192.168.1.1?"

## VLANs

A **VLAN** splits one physical switch into several separate networks: staff, guests, phones, cameras and servers can share hardware but not traffic. Each VLAN is its own subnet and **broadcast domain**; traffic between VLANs must go through a **router** or **layer 3 switch** (*inter-VLAN routing*, or "router on a stick" with sub-interfaces).

- **Access port**: belongs to one VLAN (a PC, a printer). A **voice VLAN** lets an IP phone and the PC behind it use different VLANs on one port.
- **Trunk port**: carries many VLANs between switches, **tagging** frames with **802.1Q** tags. The **native VLAN** is untagged; change it from VLAN 1 for security.

## Loops and Spanning Tree

Two links between switches make a **loop**: broadcasts circle forever (a **broadcast storm**) and the network dies in seconds. **STP** (802.1D) and the faster **RSTP** (802.1w) block redundant links, elect a **root bridge**, and unblock a backup link when the active one fails. Port states: blocking, listening, learning, forwarding (RSTP: discarding, learning, forwarding).

**Link aggregation** (**LACP**, 802.3ad) bundles several cables into one logical link for more bandwidth and redundancy. **Port security** limits which MAC addresses may use a port. **PoE** standards: 802.3af (15.4 W), 802.3at PoE+ (30 W), 802.3bt (60–100 W).

## Routing

A router looks up the destination IP in its **routing table** and picks the **most specific** match (longest prefix). If several sources offer a route, the one with the lowest **administrative distance** wins (connected 0, static 1, eBGP 20, OSPF 110, RIP 120).

| Type | Notes |
|---|---|
| **Static** routes | typed in by hand; a **default route** `0.0.0.0/0` sends everything else to one next hop |
| **RIP** | distance vector, hop count (max 15); legacy |
| **OSPF** | link state, uses **cost** (bandwidth); areas; the common choice inside companies |
| **EIGRP** | Cisco's advanced distance vector |
| **BGP** | the protocol of the **internet** between organizations (autonomous systems) |

**FHRP** protocols (**VRRP**, HSRP) let two routers share a virtual gateway IP, so a failed router doesn't take a subnet offline. **NAT/PAT** lets many private hosts share one public IP using different ports. **QoS** prioritizes voice and video (DSCP markings).

@@quiz
Two switches were connected with two cables, and the network suddenly stopped working with all link lights flashing. What protocol should have prevented this?
* Spanning Tree Protocol (STP/RSTP)
- OSPF
- LACP
- DHCP
? A layer 2 loop causes a broadcast storm; STP blocks redundant paths.

@@quiz
Which port type carries traffic for many VLANs between switches?
* A trunk port using 802.1Q tags
- An access port
- A voice port
- A console port
? Access ports carry one VLAN; trunks carry many, tagged.

@@quiz
PCs in VLAN 10 can't reach the printer in VLAN 20, though both VLANs work on their own. What's missing?
* Inter-VLAN routing (a router or layer 3 switch)
- A second switch
- A crossover cable
- STP
? Each VLAN is a separate network; only routing connects them.

@@quiz
A router has routes to 10.0.0.0/8 and 10.1.2.0/24. Which does it use for traffic to 10.1.2.5?
* 10.1.2.0/24, the longest prefix match
- 10.0.0.0/8, because it was added first
- Both, alternating
- Neither; it uses the default route
? The most specific route wins.

@@quiz
Which routing protocol connects organizations across the internet?
* BGP
- OSPF
- RIP
- EIGRP
? BGP is the internet's exterior gateway protocol.

@@quiz
Which protocol bundles several physical links into one logical link?
* LACP (802.3ad)
- STP
- VRRP
- 802.1Q
? Link aggregation adds bandwidth and survives a cable failure.

@@quiz
What does VRRP provide?
* A shared virtual default gateway between two routers, for redundancy
- Encrypted routing updates
- VLAN tagging
- Wireless roaming
? A first-hop redundancy protocol, like Cisco's HSRP.

@@quiz
Which uses less administrative distance, and so is preferred: a static route or an OSPF route to the same network?
* The static route (AD 1 vs 110)
- The OSPF route
- Whichever has more hops
- They're load-balanced
? Lower administrative distance means more trusted.

@@flashcards
MAC address table :: Switch's map of MAC addresses to ports
ARP :: Finds the MAC address for an IPv4 address
VLAN :: A separate layer 2 network / broadcast domain on shared switches
802.1Q :: VLAN tagging on trunk links
native VLAN :: The untagged VLAN on a trunk
STP / RSTP :: Prevent layer 2 loops
broadcast storm :: Broadcasts looping endlessly: the network collapses
LACP :: Link aggregation (802.3ad)
PoE / PoE+ / PoE++ :: 802.3af 15.4 W / 802.3at 30 W / 802.3bt up to 100 W
longest prefix match :: The most specific route wins
administrative distance :: Trust in a route source; lower wins (static 1, OSPF 110)
OSPF :: Link-state routing using cost; inside organizations
BGP :: Routing between autonomous systems: the internet
default route :: 0.0.0.0/0
VRRP / HSRP :: Redundant default gateway
PAT :: Many private hosts share one public IP via ports
QoS :: Prioritize traffic like voice

@@drill
What IEEE standard tags frames with their VLAN on a trunk?
= 802.1Q
= 802.1q
= ieee 802.1q
? The tag holds a 12-bit VLAN ID: VLANs 1–4094.

@@exercise Choose a route by longest prefix match
A router picks the most specific matching route (the longest prefix), and the default route `0.0.0.0/0` only when nothing else matches. Write `best_route(table, address)` using the `ipaddress` module and print the next hop for each destination.
@@lang python
@@starter
import ipaddress

table = [
    ("0.0.0.0/0", "203.0.113.1"),
    ("10.0.0.0/8", "10.255.0.1"),
    ("10.20.0.0/16", "10.20.0.1"),
    ("10.20.30.0/24", "10.20.30.1"),
    ("192.168.1.0/24", "connected"),
]
destinations = ["10.20.30.40", "10.20.99.1", "10.1.1.1", "192.168.1.77", "8.8.8.8"]

def best_route(table, address):
    raise NotImplementedError


for d in destinations:
    net, hop = best_route(table, d)
    print(f"{d} -> {hop} (via {net})")
@@expected
10.20.30.40 -> 10.20.30.1 (via 10.20.30.0/24)
10.20.99.1 -> 10.20.0.1 (via 10.20.0.0/16)
10.1.1.1 -> 10.255.0.1 (via 10.0.0.0/8)
192.168.1.77 -> connected (via 192.168.1.0/24)
8.8.8.8 -> 203.0.113.1 (via 0.0.0.0/0)
@@requires
def best_route
prefixlen
@@hint
Loop over every route that contains the address and keep the one with the largest `prefixlen`.
@@solution
import ipaddress

table = [
    ("0.0.0.0/0", "203.0.113.1"),
    ("10.0.0.0/8", "10.255.0.1"),
    ("10.20.0.0/16", "10.20.0.1"),
    ("10.20.30.0/24", "10.20.30.1"),
    ("192.168.1.0/24", "connected"),
]
destinations = ["10.20.30.40", "10.20.99.1", "10.1.1.1", "192.168.1.77", "8.8.8.8"]

def best_route(table, address):
    ip = ipaddress.ip_address(address)
    best = None
    for prefix, hop in table:
        net = ipaddress.ip_network(prefix)
        if ip in net and (best is None or net.prefixlen > best[0].prefixlen):
            best = (net, hop)
    return best

for d in destinations:
    net, hop = best_route(table, d)
    print(f"{d} -> {hop} (via {net})")

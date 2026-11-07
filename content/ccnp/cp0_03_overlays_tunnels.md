@@lesson cp0-overlays-tunnels Overlays in Depth: GRE, IPsec, VXLAN, LISP and the MTU Problem
@@text
ENCOR's virtualization section (about 10% of the exam) covers several tunnelling technologies, and nearly all of their real-world failures come down to two things: **encapsulation overhead** and **how the overlay finds the underlay endpoint**. This lesson gives you the overhead numbers to memorise and the lookup logic behind LISP and VXLAN.

## Encapsulation overhead (bytes added to each packet)

| Encapsulation | Added header | Typical total |
|---|---|---|
| GRE over IPv4 | new IP (20) + GRE (4) | **24** |
| GRE with key | + 4 | 28 |
| IPsec ESP tunnel mode (AES-CBC, SHA-1) | new IP 20 + ESP 8 + IV 16 + pad 0–15 + trailer 2 + ICV 12 | ~52–73 |
| GRE over IPsec transport mode | GRE 24 + ESP overhead without a new IP | ~56–60 |
| VXLAN | outer Ethernet 14 + IP 20 + UDP 8 + VXLAN 8 | **50** (the inner frame keeps its 14-byte Ethernet header) |
| LISP | IP 20 + UDP 8 + LISP 8 | **36** |

**Consequences**: with an underlay MTU of 1500, a GRE tunnel interface can only carry 1476-byte packets. Common fixes:
- `ip mtu 1400` on the tunnel, so the router fragments or sends ICMP "fragmentation needed" before encapsulating
- `ip tcp adjust-mss 1360`: rewrite the TCP MSS in SYNs (MSS = MTU − 40 for the IPv4 and TCP headers)
- raise the underlay MTU (jumbo frames, 9000 or more in data-centre fabrics for VXLAN)

Path MTU discovery (PMTUD) relies on ICMP type 3 code 4. Firewalls that drop ICMP create **black holes**: small packets work, large transfers hang.

## GRE and IPsec

- **GRE** carries any protocol, including multicast, so routing protocols can run over it, but it isn't encrypted.
- **IPsec** encrypts but (natively) handles only unicast IP. **GRE over IPsec** combines both: routing protocols plus encryption. Static VTI (`tunnel mode ipsec ipv4`) is the modern alternative.
- **IKEv2** negotiates the SAs (phase 1/IKE_SA_INIT, then CREATE_CHILD_SA). Proposals must match: encryption, integrity, DH group, lifetime.

## VXLAN

- A 24-bit **VNI** gives 16 million segments, compared with 4,094 VLANs.
- **VTEPs** encapsulate and decapsulate. The outer UDP destination port is **4789**, and the source port is a hash of the inner flow (for ECMP entropy).
- Control plane options: flood and learn (multicast), or **BGP EVPN** (MAC/IP routes advertised). SD-Access uses VXLAN with LISP as its control plane.

## LISP

- Splits identity from location. **EIDs** (endpoint IDs) are the host prefixes, and **RLOCs** are the routers' underlay addresses.
- **ITR** (ingress tunnel router) asks the **Map-Resolver/Map-Server** for an EID-to-RLOC mapping and caches it (a **map-cache**, longest-prefix match). The **ETR** registers its EIDs with the Map-Server.
- Each mapping can list several RLOCs with **priority** (lower is preferred) and **weight** (load share among equal priorities).
- If there's no mapping, the ITR sends a Map-Request and (on IOS) drops or forwards natively until the reply arrives. A negative Map-Reply means the destination is non-LISP, so traffic goes via the PxTR.

@@quiz
How many bytes does basic GRE over IPv4 add?
- 20
* 24
- 36
- 50
? A new 20-byte IP header plus a 4-byte GRE header.

@@quiz
What is the VXLAN UDP destination port?
- 4500
- 8472
* 4789
- 4341
? 4341 and 4342 are LISP data and control.

@@quiz
Small pings across a tunnel work, but large file transfers hang. What is the most likely cause?
- Wrong VNI
* PMTUD black hole: ICMP fragmentation-needed messages are filtered
- An IKE proposal mismatch
- Duplicate EIDs
? Fix it by allowing ICMP, setting ip mtu, or adjusting the MSS.

@@quiz
Why run GRE over IPsec instead of plain IPsec tunnel mode?
* To carry multicast routing protocols such as OSPF over an encrypted tunnel
- GRE adds encryption
- It removes all overhead
- IPsec can't carry TCP
? Static VTIs are a newer way to get the same result.

@@quiz
Two RLOCs for an EID have priorities 1 and 2. Which is used?
* The priority 1 RLOC; the other is a backup
- Both, equally
- The priority 2 RLOC
- Whichever has the higher weight
? Lower priority wins, and weight balances load among equals.

@@flashcards
GRE overhead :: 24 bytes
VXLAN overhead :: 50 bytes
LISP overhead :: 36 bytes
TCP MSS :: MTU − 40
ip tcp adjust-mss :: rewrites MSS in SYNs
PMTUD :: ICMP type 3 code 4
black hole :: big packets lost, ICMP filtered
VNI :: 24-bit VXLAN segment ID
VTEP :: VXLAN tunnel endpoint
UDP 4789 :: VXLAN
BGP EVPN :: VXLAN control plane
EID :: LISP endpoint identifier
RLOC :: LISP routing locator
ITR / ETR :: LISP ingress / egress tunnel router
Map-Server / Map-Resolver :: LISP mapping system
priority / weight :: lower wins / load share

@@drill
An underlay MTU of 1500 carries GRE (24 bytes). What TCP MSS should you set for the tunnelled traffic?
= 1436
? 1500 − 24 − 40. Many designs round down to 1360 to leave room for IPsec.

@@drill
How many bits is a VXLAN Network Identifier?
= 24
? That gives about 16 million segments.

@@exercise Size tunnel MTU and MSS
Stack encapsulations over an underlay MTU, compute the largest inner packet and the TCP MSS for each design, and flag designs where common packet sizes (1500-byte inner packets from hosts) won't fit without fragmentation or MSS clamping.
@@concepts numbers, lists, functions, networking, maps, conditions
@@lang python
@@starter
HEADERS = {"ip": 20, "gre": 4, "gre_key": 4, "udp": 8, "vxlan": 8, "eth": 14, "lisp": 8, "esp_fixed": 8 + 16 + 2 + 12}
designs = [
    ("GRE over the internet", 1500, ["ip", "gre"]),
    ("GRE over IPsec (tunnel mode)", 1500, ["ip", "gre", "ip", "esp_fixed"]),
    ("VXLAN fabric, standard MTU", 1500, ["ip", "udp", "vxlan", "eth"]),
    ("VXLAN fabric, jumbo underlay", 9216, ["ip", "udp", "vxlan", "eth"]),
    ("LISP over a WAN", 1500, ["ip", "udp", "lisp"]),
]

def inner_mtu(underlay, stack):
    raise NotImplementedError
@@expected
GRE over the internet          overhead  24, inner MTU 1476, MSS 1436: set ip mtu 1476 and ip tcp adjust-mss 1436
GRE over IPsec (tunnel mode)   overhead  92, inner MTU 1408, MSS 1368: set ip mtu 1408 and ip tcp adjust-mss 1368
VXLAN fabric, standard MTU     overhead  50, inner MTU 1450, MSS 1410: set ip mtu 1450 and ip tcp adjust-mss 1410
VXLAN fabric, jumbo underlay   overhead  50, inner MTU 9166, MSS 9126: 1500-byte packets fit
LISP over a WAN                overhead  36, inner MTU 1464, MSS 1424: set ip mtu 1464 and ip tcp adjust-mss 1424
@@requires
def inner_mtu
HEADERS
- 40
@@hint
Subtract each header in the stack from the underlay MTU. ESP also pads the encrypted payload to a multiple of 16 bytes, so round the result down to a multiple of 16 when `esp_fixed` is in the stack (a simplification). MSS = inner MTU − 40. The design "fits" if the inner MTU is at least 1500.
@@solution
HEADERS = {"ip": 20, "gre": 4, "gre_key": 4, "udp": 8, "vxlan": 8, "eth": 14, "lisp": 8, "esp_fixed": 8 + 16 + 2 + 12}
designs = [
    ("GRE over the internet", 1500, ["ip", "gre"]),
    ("GRE over IPsec (tunnel mode)", 1500, ["ip", "gre", "ip", "esp_fixed"]),
    ("VXLAN fabric, standard MTU", 1500, ["ip", "udp", "vxlan", "eth"]),
    ("VXLAN fabric, jumbo underlay", 9216, ["ip", "udp", "vxlan", "eth"]),
    ("LISP over a WAN", 1500, ["ip", "udp", "lisp"]),
]

def inner_mtu(underlay, stack):
    mtu = underlay - sum(HEADERS[h] for h in stack)
    if "esp_fixed" in stack:
        mtu -= mtu % 16
    return mtu

for name, underlay, stack in designs:
    mtu = inner_mtu(underlay, stack)
    overhead = underlay - mtu
    verdict = "1500-byte packets fit" if mtu >= 1500 else f"set ip mtu {mtu} and ip tcp adjust-mss {mtu - 40}"
    print(f"{name:<30} overhead {overhead:>3}, inner MTU {mtu:>4}, MSS {mtu - 40:>4}: {verdict}")

@@exercise LISP map-cache lookups
An ITR holds a map-cache of EID prefixes to RLOC sets. For each destination, do a longest-prefix match. Choose RLOCs by lowest priority among those that are up, and split traffic by weight. If there's no match, send a Map-Request, and if the mapping system's reply is negative, forward via the proxy (PxTR).
@@lang python
@@starter
import ipaddress
map_cache = {
    "10.1.0.0/16": [("192.0.2.1", 1, 50, True), ("192.0.2.2", 1, 50, True), ("198.51.100.9", 2, 100, True)],
    "10.1.5.0/24": [("192.0.2.7", 1, 100, False), ("203.0.113.4", 2, 100, True)],
    "10.2.0.0/16": [("198.51.100.20", 1, 30, True), ("198.51.100.21", 1, 70, True)],
}
mapping_system = {"10.3.0.0/16": [("203.0.113.50", 1, 100, True)]}  # anything else: negative reply

def lookup(dest):
    raise NotImplementedError
@@expected
10.1.9.9    map-cache hit 10.1.0.0/16 -> 192.0.2.1 (50%), 192.0.2.2 (50%) at priority 1
10.1.5.20   map-cache hit 10.1.5.0/24 -> 203.0.113.4 (100%) at priority 2
10.2.1.1    map-cache hit 10.2.0.0/16 -> 198.51.100.20 (30%), 198.51.100.21 (70%) at priority 1
10.3.4.4    miss: Map-Request, reply 10.3.0.0/16 installed -> 203.0.113.50 (100%) at priority 1
10.3.8.8    map-cache hit 10.3.0.0/16 -> 203.0.113.50 (100%) at priority 1
172.16.0.1  miss: Map-Request, negative reply -> forward natively via the PxTR
@@requires
def lookup
prefixlen
priority
@@hint
Keep the cache prefixes whose network contains the destination and pick the one with the largest `prefixlen`. From its RLOCs `(address, priority, weight, up)`, keep those that are up, take the lowest priority, and report each one's weight share. On a miss, check `mapping_system` the same way: on a hit, install the mapping in the cache (so the next lookup hits), otherwise use the PxTR.
@@solution
import ipaddress
map_cache = {
    "10.1.0.0/16": [("192.0.2.1", 1, 50, True), ("192.0.2.2", 1, 50, True), ("198.51.100.9", 2, 100, True)],
    "10.1.5.0/24": [("192.0.2.7", 1, 100, False), ("203.0.113.4", 2, 100, True)],
    "10.2.0.0/16": [("198.51.100.20", 1, 30, True), ("198.51.100.21", 1, 70, True)],
}
mapping_system = {"10.3.0.0/16": [("203.0.113.50", 1, 100, True)]}  # anything else: negative reply

def longest_match(table, dest):
    ip = ipaddress.ip_address(dest)
    hits = [ipaddress.ip_network(p) for p in table if ip in ipaddress.ip_network(p)]
    return str(max(hits, key=lambda n: n.prefixlen)) if hits else None

def choose(rlocs):
    up = [r for r in rlocs if r[3]]
    best = min(r[1] for r in up)
    chosen = [r for r in up if r[1] == best]
    total = sum(r[2] for r in chosen)
    return ", ".join(f"{a} ({w * 100 // total}%)" for a, p, w, _ in chosen) + f" at priority {best}"

def lookup(dest):
    prefix = longest_match(map_cache, dest)
    if prefix:
        return f"map-cache hit {prefix} -> {choose(map_cache[prefix])}"
    found = longest_match(mapping_system, dest)
    if found:
        map_cache[found] = mapping_system[found]
        return f"miss: Map-Request, reply {found} installed -> {choose(map_cache[found])}"
    return "miss: Map-Request, negative reply -> forward natively via the PxTR"

for dest in ("10.1.9.9", "10.1.5.20", "10.2.1.1", "10.3.4.4", "10.3.8.8", "172.16.0.1"):
    print(f"{dest:<11} {lookup(dest)}")

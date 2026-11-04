@@lesson cp1-switching-security-deep Switching, FHRP and Infrastructure Security in Depth
@@text
ENCOR's infrastructure (30%) and security (20%) domains test the details that break real networks: which switch becomes STP root and why, which ports block, which router is the active gateway, and whether an ACL actually permits what you think. This lesson covers STP election and port roles, FHRP priorities and preemption, ACL processing with wildcard masks, and device hardening.

## Spanning tree election

1. **Root bridge**: lowest **bridge ID** = priority (default **32768** + VLAN ID in PVST+, set in steps of **4096**) then lowest **MAC**.
2. **Root port** on each non-root switch: lowest **root path cost**, then lowest sender bridge ID, then lowest sender port priority/number.
3. **Designated port** on each segment: the switch with the lowest root path cost to the root (tie: lowest bridge ID). All other ports are **alternate/blocking**.

Costs (802.1D short / 802.1t long): 10 Mbps 100 / 2,000,000; 100 Mbps **19** / 200,000; 1 Gbps **4** / 20,000; 10 Gbps **2** / 2,000.

Protection: **PortFast** (edge ports), **BPDU Guard** (err-disable if a BPDU arrives on an edge port), **Root Guard** (stop a downstream switch becoming root), **Loop Guard** and UDLD (unidirectional links). Variants: PVST+, **Rapid PVST+** (802.1w: discarding/learning/forwarding, proposal/agreement), **MST** (802.1s: instances map many VLANs).

## First-hop redundancy

| | HSRP | VRRP | GLBP |
|---|---|---|---|
| Standard | Cisco | IETF | Cisco |
| Roles | active/standby | master/backup | AVG + up to 4 AVFs (load balancing) |
| Default priority | 100 | 100 | 100 |
| Preemption | **off** by default | **on** by default | off |
| Virtual MAC | 0000.0c07.acXX (v1) / 0000.0c9f.fXXX (v2) | 0000.5e00.01XX | 0007.b400.XXYY |

Highest priority wins; tie: highest IP address. **Object tracking** lowers priority when an uplink fails, so a peer with preemption can take over.

## ACLs

- Processed **top-down, first match**, then **implicit deny any**.
- **Wildcard masks**: 0 = must match, 1 = ignore. `10.1.0.0 0.0.255.255` = 10.1.0.0/16; `host 10.1.1.1` = 0.0.0.0 wildcard; `any` = 255.255.255.255.
- Standard ACLs (source only) near the destination; extended ACLs (protocol, source, destination, ports) near the source.
- Control plane protection: **CoPP**, infrastructure ACLs; management: SSH v2 only, AAA with **TACACS+** (TCP 49, encrypts the whole body, per-command authorisation) or **RADIUS** (UDP 1812/1813, combines authentication and authorisation, used for 802.1X network access).
- Layer 2 security: port security, **DHCP snooping** (trusted ports towards the DHCP server), **Dynamic ARP Inspection** (uses the snooping binding table), IP Source Guard, storm control, private VLANs.
- **802.1X**: supplicant, authenticator (switch), authentication server (RADIUS, e.g. ISE); MAB as fallback for devices without a supplicant.

@@quiz
Switch A has priority 32768 and MAC 0011.1111.1111; switch B has priority 32768 and MAC 0011.0000.0001. Which is root (same VLAN)?
- A
* B
- Both
- Neither until configured
? Equal priority: lowest MAC.

@@quiz
Which STP feature err-disables an access port that receives a BPDU?
- Root Guard
* BPDU Guard
- Loop Guard
- PortFast
? Root Guard puts the port in root-inconsistent state instead.

@@quiz
Which FHRP has preemption enabled by default?
- HSRP
* VRRP
- GLBP
- None
? HSRP needs `standby preempt`.

@@quiz
What does wildcard mask 0.0.0.255 with 192.168.5.0 match?
* 192.168.5.0–192.168.5.255
- Only 192.168.5.0
- 192.168.0.0/16
- Everything
? 1 bits are ignored.

@@quiz
Which AAA protocol encrypts the whole packet body and supports per-command authorisation?
* TACACS+
- RADIUS
- LDAP
- Kerberos
? RADIUS encrypts only the password.

@@quiz
DAI relies on which feature's binding table?
- Port security
* DHCP snooping
- 802.1X
- CDP
? IP-to-MAC bindings learned from DHCP.

@@flashcards
root bridge :: lowest priority, then MAC
default bridge priority :: 32768 (+ VLAN)
priority step :: 4096
root port :: lowest cost to root
designated port :: best port per segment
STP cost 1 Gbps short :: 4
STP cost 100 Mbps short :: 19
BPDU Guard :: err-disable on BPDU
Root Guard :: block superior BPDUs
Rapid PVST+ :: 802.1w per VLAN
MST :: 802.1s instances
HSRP preemption :: off by default
VRRP preemption :: on by default
GLBP :: load-balancing FHRP
FHRP tie :: highest IP
ACL processing :: first match, implicit deny
wildcard 0 bit :: must match
standard ACL :: near destination
extended ACL :: near source
TACACS+ :: TCP 49, full encryption
RADIUS :: UDP 1812/1813
DHCP snooping :: trusted server ports
DAI :: ARP checked against bindings
802.1X roles :: supplicant, authenticator, server

@@drill
What is the default STP bridge priority?
= 32768
? Plus the VLAN ID in PVST+.

@@drill
Which TCP port does TACACS+ use?
= 49
? RADIUS uses UDP 1812/1813.

@@exercise Elect the STP root and port roles
Given switches (priority, MAC) and links with costs, elect the root bridge, compute each switch's root path cost and root port (lowest cost, then lowest neighbour bridge ID), and then the designated port on each link (the end whose switch has the lower root cost, tie lowest bridge ID). All remaining link ends are blocking.
@@lang python
@@starter
switches = {"SW1": (32768, "0011.aaaa.0001"), "SW2": (28672, "0011.aaaa.0009"), "SW3": (32768, "0011.aaaa.0003"), "SW4": (32768, "0011.aaaa.0002")}
links = [("SW1", "SW2", 4), ("SW1", "SW3", 4), ("SW2", "SW3", 19), ("SW2", "SW4", 4), ("SW3", "SW4", 4), ("SW1", "SW4", 19)]

def bridge_id(name):
    raise NotImplementedError
@@expected
Root bridge: SW2 (28672, '0011.aaaa.0009')
SW1: root cost 4, root port towards SW2
SW3: root cost 8, root port towards SW1
SW4: root cost 4, root port towards SW2
link SW1-SW2: Designated SW2, SW1 root port
link SW1-SW3: Designated SW1, SW3 root port
link SW2-SW3: Designated SW2, SW3 BLOCKING
link SW2-SW4: Designated SW2, SW4 root port
link SW3-SW4: Designated SW4, SW3 BLOCKING
link SW1-SW4: Designated SW1, SW4 BLOCKING
@@requires
def bridge_id
root
Designated
@@hint
Root path costs: a simple Dijkstra (or Bellman-Ford over the small graph) from the root. Root port of switch S: among its links (S, N, cost), minimise `(cost_to_root[N] + cost, bridge_id(N))`. On each link, the designated end is the switch with lower `(cost_to_root, bridge_id)`.
@@solution
switches = {"SW1": (32768, "0011.aaaa.0001"), "SW2": (28672, "0011.aaaa.0009"), "SW3": (32768, "0011.aaaa.0003"), "SW4": (32768, "0011.aaaa.0002")}
links = [("SW1", "SW2", 4), ("SW1", "SW3", 4), ("SW2", "SW3", 19), ("SW2", "SW4", 4), ("SW3", "SW4", 4), ("SW1", "SW4", 19)]

def bridge_id(name):
    priority, mac = switches[name]
    return priority, mac

def neighbours(s):
    for a, b, c in links:
        if a == s:
            yield b, c, (a, b)
        elif b == s:
            yield a, c, (a, b)

root = min(switches, key=bridge_id)
cost = {s: float("inf") for s in switches}
cost[root] = 0
for _ in switches:
    for a, b, c in links:
        cost[b] = min(cost[b], cost[a] + c)
        cost[a] = min(cost[a], cost[b] + c)
print(f"Root bridge: {root} {bridge_id(root)}")

root_ports = {}
for s in sorted(switches):
    if s == root:
        continue
    n, c, link = min(neighbours(s), key=lambda x: (cost[x[0]] + x[1], bridge_id(x[0])))
    root_ports[s] = link
    print(f"{s}: root cost {cost[s]}, root port towards {n}")

for a, b, c in links:
    designated = min((a, b), key=lambda s: (cost[s], bridge_id(s)))
    other = b if designated == a else a
    if root_ports.get(other) == (a, b):
        role = f"{other} root port"
    else:
        role = f"{other} BLOCKING"
    print(f"link {a}-{b}: Designated {designated}, {role}")

@@exercise Evaluate an extended ACL with wildcard masks
Implement extended ACL matching (protocol, source and destination with wildcard masks or `any`/`host`, and an optional destination port), first match wins, implicit deny. Print the decision and the matching line for each packet.
@@lang python
@@starter
import ipaddress

acl = [
    (10, "permit", "tcp", "10.1.0.0 0.0.255.255", "host 10.9.9.10", 443),
    (20, "deny", "tcp", "any", "10.9.9.0 0.0.0.255", 22),
    (30, "permit", "udp", "10.1.0.0 0.0.255.255", "host 10.9.9.53", 53),
    (40, "permit", "icmp", "10.0.0.0 0.255.255.255", "any", None),
    (50, "deny", "ip", "10.1.2.0 0.0.0.255", "any", None),
    (60, "permit", "ip", "10.1.0.0 0.0.255.255", "10.9.0.0 0.0.255.255", None),
]
packets = [("tcp", "10.1.5.20", "10.9.9.10", 443), ("tcp", "10.1.5.20", "10.9.9.11", 22), ("udp", "10.1.2.7", "10.9.9.53", 53),
           ("icmp", "10.200.1.1", "8.8.8.8", None), ("tcp", "10.1.2.7", "10.9.1.1", 8080), ("tcp", "10.1.3.7", "10.9.1.1", 8080), ("tcp", "172.16.0.5", "10.9.9.10", 443)]

def addr_match(spec, ip):
    raise NotImplementedError
@@expected
tcp    10.1.5.20 -> 10.9.9.10     443: PERMIT (line 10)
tcp    10.1.5.20 -> 10.9.9.11      22: DENY (line 20)
udp     10.1.2.7 -> 10.9.9.53      53: PERMIT (line 30)
icmp  10.200.1.1 -> 8.8.8.8          : PERMIT (line 40)
tcp     10.1.2.7 -> 10.9.1.1     8080: DENY (line 50)
tcp     10.1.3.7 -> 10.9.1.1     8080: PERMIT (line 60)
tcp   172.16.0.5 -> 10.9.9.10     443: DENY (implicit deny)
@@requires
def addr_match
int(ipaddress.IPv4Address(
implicit deny
@@hint
With integers: an address matches `base wildcard` when `(ip & ~wildcard) == (base & ~wildcard)` (mask to 32 bits). `any` matches everything; `host X` means wildcard 0. Protocol `ip` matches all protocols.
@@solution
import ipaddress

acl = [
    (10, "permit", "tcp", "10.1.0.0 0.0.255.255", "host 10.9.9.10", 443),
    (20, "deny", "tcp", "any", "10.9.9.0 0.0.0.255", 22),
    (30, "permit", "udp", "10.1.0.0 0.0.255.255", "host 10.9.9.53", 53),
    (40, "permit", "icmp", "10.0.0.0 0.255.255.255", "any", None),
    (50, "deny", "ip", "10.1.2.0 0.0.0.255", "any", None),
    (60, "permit", "ip", "10.1.0.0 0.0.255.255", "10.9.0.0 0.0.255.255", None),
]
packets = [("tcp", "10.1.5.20", "10.9.9.10", 443), ("tcp", "10.1.5.20", "10.9.9.11", 22), ("udp", "10.1.2.7", "10.9.9.53", 53),
           ("icmp", "10.200.1.1", "8.8.8.8", None), ("tcp", "10.1.2.7", "10.9.1.1", 8080), ("tcp", "10.1.3.7", "10.9.1.1", 8080), ("tcp", "172.16.0.5", "10.9.9.10", 443)]

def to_int(text):
    return int(ipaddress.IPv4Address(text))

def addr_match(spec, ip):
    if spec == "any":
        return True
    if spec.startswith("host "):
        return to_int(spec[5:]) == to_int(ip)
    base, wildcard = spec.split()
    keep = ~to_int(wildcard) & 0xFFFFFFFF
    return (to_int(ip) & keep) == (to_int(base) & keep)

for proto, src, dst, port in packets:
    for seq, action, a_proto, a_src, a_dst, a_port in acl:
        if a_proto not in ("ip", proto):
            continue
        if not (addr_match(a_src, src) and addr_match(a_dst, dst)):
            continue
        if a_port is not None and a_port != port:
            continue
        print(f"{proto:<4} {src:>11} -> {dst:<11} {port or '':>5}: {action.upper()} (line {seq})")
        break
    else:
        print(f"{proto:<4} {src:>11} -> {dst:<11} {port or '':>5}: DENY (implicit deny)")

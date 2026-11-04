@@lesson c1-stp-deep Spanning Tree, EtherChannel and Discovery Protocols in Depth
@@text
The configuration lesson showed the commands; this one explains how STP elects its root and chooses port roles, how Rapid PVST+ converges, how EtherChannel negotiates with LACP and PAgP, and how CDP and LLDP help you map a network. These are among the most-tested CCNA topics.

## Why STP exists

Redundant switch links create **layer 2 loops**. Ethernet frames have no TTL, so a broadcast circulates forever: a **broadcast storm**, MAC table flapping and duplicate frames. STP (802.1D) blocks redundant ports so there is exactly one active path between any two switches.

## The election, step by step

1. **Root bridge**: the switch with the lowest **bridge ID** = priority (default **32768** + VLAN number, the "extended system ID") followed by the MAC address. Lowest priority wins; on a tie, the lowest MAC.
2. **Root ports** (one per non-root switch): the port with the lowest **root path cost** to the root. Ties: lowest neighbour bridge ID, then lowest neighbour port ID (priority, default 128, + port number).
3. **Designated ports** (one per segment): the port on the segment with the lowest root path cost; ties broken by bridge ID. All root bridge ports are designated.
4. Everything else is **blocking** (802.1D) / **alternate** (RSTP).

Default port costs (short method):

| Speed | 10 Mb/s | 100 Mb/s | 1 Gb/s | 10 Gb/s |
|---|---|---|---|---|
| STP cost | 100 | 19 | 4 | 2 |

Influence the election: `spanning-tree vlan 10 root primary` (sets priority to 24576, or lower if needed) or `spanning-tree vlan 10 priority 4096` (multiples of 4096). Put the root on the **distribution/core** switch, never an access switch by accident.

## Port states and timers

802.1D: **Blocking** (20 s max age) → **Listening** (15 s forward delay) → **Learning** (15 s) → **Forwarding**. Up to **50 seconds** to converge. Hello every 2 s.

**RSTP (802.1w)** / Cisco **Rapid PVST+**: states are **discarding**, **learning**, **forwarding**; roles add **alternate** (backup root port) and **backup**. Converges in about a second using proposal/agreement handshakes on point-to-point links. **PVST+/Rapid PVST+** run one instance per VLAN, so different VLANs can use different roots (load sharing).

## Edge features

- **PortFast**: access ports go straight to forwarding (`spanning-tree portfast`, or `spanning-tree portfast default`).
- **BPDU Guard**: err-disables a PortFast port that receives a BPDU (someone plugged in a switch). Recover with `shutdown`/`no shutdown` or `errdisable recovery cause bpduguard`.
- **Root Guard**: on ports where the root must never appear (stops a rogue switch becoming root).
- **Loop Guard**: protects against unidirectional links.

```text
SW2# show spanning-tree vlan 10
VLAN0010
  Spanning tree enabled protocol rstp
  Root ID    Priority    24586
             Address     0011.2233.4401
             Cost        4
             Port        1 (GigabitEthernet1/0/1)
  Bridge ID  Priority    32778  (priority 32768 sys-id-ext 10)
             Address     0011.2233.4402
Interface           Role Sts Cost      Prio.Nbr Type
------------------- ---- --- --------- -------- ------
Gi1/0/1             Root FWD 4         128.1    P2p
Gi1/0/2             Altn BLK 4         128.2    P2p
Gi1/0/10            Desg FWD 19        128.10   P2p Edge
```

## EtherChannel

Bundles 2–8 parallel links into one logical **Port-channel**: more bandwidth, and STP sees one link, so nothing is blocked. Load is shared per flow (hash of MACs/IPs/ports: `port-channel load-balance src-dst-ip`), so a single flow uses one member.

| Protocol | Modes | Forms a channel when |
|---|---|---|
| **LACP** (802.3ad, open) | `active`, `passive` | at least one side is active |
| **PAgP** (Cisco) | `desirable`, `auto` | at least one side is desirable |
| Static | `on` | both sides `on` (no negotiation; risky) |

`passive`+`passive` or `auto`+`auto` → no channel. Members must match: **speed, duplex, access VLAN or trunk settings (allowed VLANs, native VLAN)**. Mismatches leave members suspended (`s`) in `show etherchannel summary`.

```text
SW1# show etherchannel summary
Group  Port-channel  Protocol    Ports
------+-------------+-----------+---------------------------
1      Po1(SU)         LACP      Gi1/0/23(P)  Gi1/0/24(P)
2      Po2(SD)         LACP      Gi1/0/21(s)  Gi1/0/22(I)
```

Flags: `S` layer 2, `R` layer 3, `U` in use, `D` down, `P` bundled, `s` suspended, `I` stand-alone. Layer 3 EtherChannel: `no switchport` on the Port-channel and members, then an IP address on the Port-channel.

## CDP and LLDP

| | CDP | LLDP |
|---|---|---|
| Standard | Cisco proprietary | IEEE 802.1AB |
| Default on Cisco | enabled | disabled (`lldp run`) |
| Timers | 60 s hello, 180 s hold | 30 s, 120 s hold |
| Commands | `show cdp neighbors [detail]` | `show lldp neighbors [detail]` |

Both reveal neighbour hostname, local and remote ports, platform, capabilities and (in `detail`) management IP and IOS version. Disable them on untrusted edge ports (`no cdp enable`, `no lldp transmit/receive`): they leak information.

@@quiz
Which switch becomes the STP root bridge?
- The one with the highest MAC address
* The one with the lowest bridge ID (priority, then MAC)
- The one with the most ports
- The one with the fastest uplinks
? Default priority is 32768 + VLAN.

@@quiz
Two switches have the same priority. Which becomes root?
* The one with the lower MAC address
- The one with the higher MAC address
- The one configured first
- Neither: STP fails
? MAC is the tiebreaker.

@@quiz
What is the default STP cost of a 1 Gb/s link?
- 19
* 4
- 2
- 100
? 100 Mb/s is 19, 10 Gb/s is 2.

@@quiz
One side of an EtherChannel is LACP `passive`. What must the other side be for the channel to form?
- passive
* active
- desirable
- on
? passive + passive never negotiates.

@@quiz
A user plugs a small switch into a PortFast access port with BPDU Guard. What happens?
- The new switch becomes root
* The port is err-disabled
- The port moves to blocking only for VLAN 1
- Nothing
? BPDU Guard shuts the port when it hears a BPDU.

@@quiz
Which discovery protocol is an open standard?
- CDP
* LLDP
- PAgP
- VTP
? IEEE 802.1AB.

@@flashcards
bridge ID :: priority + MAC
default priority :: 32768 (+ VLAN)
priority steps :: multiples of 4096
root port :: lowest cost to root
designated port :: one per segment
STP cost 100M / 1G / 10G :: 19 / 4 / 2
802.1D convergence :: up to 50 seconds
RSTP states :: discarding, learning, forwarding
alternate port :: backup root port
PortFast :: edge port straight to forwarding
BPDU Guard :: err-disable on BPDU
Root Guard :: prevent a new root on that port
LACP modes :: active / passive
PAgP modes :: desirable / auto
EtherChannel must match :: speed, duplex, VLANs, trunk settings
show etherchannel summary :: P bundled, s suspended
CDP :: Cisco, on by default
LLDP :: 802.1AB, lldp run

@@drill
Which command makes a switch the root for VLAN 10 using a macro?
= spanning-tree vlan 10 root primary
? It sets priority to 24576 or lower.

@@drill
What LACP mode actively starts negotiation?
= active
? passive only responds.

@@exercise Elect the root and root ports
Each switch has a priority and MAC; `links` lists `(switch_a, port_a, switch_b, port_b, cost)`. Elect the root bridge (lowest `(priority, mac)`), compute each switch's root path cost with Dijkstra, then pick each non-root switch's root port: lowest cost through that port, ties broken by the neighbour's bridge ID, then the local port name. Print the root, then each switch's cost and root port in name order.
@@lang python
@@starter
import heapq

switches = {
    "SW1": (32768, "0011.2233.4401"),
    "SW2": (32768, "0011.2233.4402"),
    "SW3": (24576, "0011.2233.4403"),
    "SW4": (32768, "0011.2233.4400"),
}
links = [
    ("SW1", "Gi1", "SW2", "Gi1", 4),
    ("SW1", "Gi2", "SW3", "Gi1", 4),
    ("SW2", "Gi2", "SW3", "Gi2", 4),
    ("SW2", "Gi3", "SW4", "Gi1", 19),
    ("SW3", "Gi3", "SW4", "Gi2", 19),
    ("SW1", "Gi3", "SW4", "Gi3", 4),
]
@@expected
Root bridge: SW3 (24576, '0011.2233.4403')
SW1: cost 4 root port Gi2
SW2: cost 4 root port Gi2
SW3: cost 0 (root)
SW4: cost 8 root port Gi3
@@requires
heapq
min(
@@hint
Build `adj[switch] = [(local_port, neighbour, cost), ...]` from both directions of each link. After Dijkstra from the root, a candidate root port is `(dist[nbr] + cost, switches[nbr], port)`; take the `min`.
@@solution
import heapq

switches = {
    "SW1": (32768, "0011.2233.4401"),
    "SW2": (32768, "0011.2233.4402"),
    "SW3": (24576, "0011.2233.4403"),
    "SW4": (32768, "0011.2233.4400"),
}
links = [
    ("SW1", "Gi1", "SW2", "Gi1", 4),
    ("SW1", "Gi2", "SW3", "Gi1", 4),
    ("SW2", "Gi2", "SW3", "Gi2", 4),
    ("SW2", "Gi3", "SW4", "Gi1", 19),
    ("SW3", "Gi3", "SW4", "Gi2", 19),
    ("SW1", "Gi3", "SW4", "Gi3", 4),
]

adj = {name: [] for name in switches}
for a, pa, b, pb, cost in links:
    adj[a].append((pa, b, cost))
    adj[b].append((pb, a, cost))

root = min(switches, key=lambda s: switches[s])
dist = {root: 0}
queue = [(0, root)]
while queue:
    d, sw = heapq.heappop(queue)
    if d > dist.get(sw, float("inf")):
        continue
    for _, nbr, cost in adj[sw]:
        if d + cost < dist.get(nbr, float("inf")):
            dist[nbr] = d + cost
            heapq.heappush(queue, (d + cost, nbr))

print("Root bridge:", root, switches[root])
for sw in sorted(switches):
    if sw == root:
        print(f"{sw}: cost 0 (root)")
        continue
    best = min((dist[nbr] + cost, switches[nbr], port) for port, nbr, cost in adj[sw])
    print(f"{sw}: cost {dist[sw]} root port {best[2]}")

@@exercise Check EtherChannel member consistency
For each port-channel, compare its member ports' settings (`mode`, `speed`, `duplex`, `vlans`, `native`) against the first member and report every mismatch. Then check the LACP/PAgP mode pair with the far end (`local`/`remote` modes) and print whether the channel can form: LACP needs at least one `active`, PAgP at least one `desirable`, `on` must face `on`, and mixing protocols never forms.
@@lang python
@@starter
members = {
    "Po1": [
        {"port": "Gi1/0/23", "mode": "trunk", "speed": 1000, "duplex": "full", "vlans": "10,20,30", "native": 99},
        {"port": "Gi1/0/24", "mode": "trunk", "speed": 1000, "duplex": "full", "vlans": "10,20,30", "native": 99},
    ],
    "Po2": [
        {"port": "Gi1/0/21", "mode": "trunk", "speed": 1000, "duplex": "full", "vlans": "10,20", "native": 99},
        {"port": "Gi1/0/22", "mode": "trunk", "speed": 100, "duplex": "full", "vlans": "10,20,30", "native": 1},
    ],
}
negotiation = {"Po1": ("active", "passive"), "Po2": ("passive", "passive"), "Po3": ("desirable", "auto"), "Po4": ("on", "active")}
LACP = {"active", "passive"}
PAGP = {"desirable", "auto"}
@@expected
Po1: members consistent
Po2: Gi1/0/22 speed 100 != 1000; Gi1/0/22 vlans 10,20,30 != 10,20; Gi1/0/22 native 1 != 99
Po1 active/passive: forms
Po2 passive/passive: does not form
Po3 desirable/auto: forms
Po4 on/active: does not form
@@requires
def forms
LACP
PAGP
@@hint
Loop over `("mode", "speed", "duplex", "vlans", "native")` and compare `m[key]` with `first[key]`. For `forms(a, b)`: both in LACP and "active" in (a, b); both in PAGP and "desirable" in (a, b); or both "on".
@@solution
members = {
    "Po1": [
        {"port": "Gi1/0/23", "mode": "trunk", "speed": 1000, "duplex": "full", "vlans": "10,20,30", "native": 99},
        {"port": "Gi1/0/24", "mode": "trunk", "speed": 1000, "duplex": "full", "vlans": "10,20,30", "native": 99},
    ],
    "Po2": [
        {"port": "Gi1/0/21", "mode": "trunk", "speed": 1000, "duplex": "full", "vlans": "10,20", "native": 99},
        {"port": "Gi1/0/22", "mode": "trunk", "speed": 100, "duplex": "full", "vlans": "10,20,30", "native": 1},
    ],
}
negotiation = {"Po1": ("active", "passive"), "Po2": ("passive", "passive"), "Po3": ("desirable", "auto"), "Po4": ("on", "active")}
LACP = {"active", "passive"}
PAGP = {"desirable", "auto"}

for po, ports in members.items():
    first = ports[0]
    problems = []
    for m in ports[1:]:
        for key in ("mode", "speed", "duplex", "vlans", "native"):
            if m[key] != first[key]:
                problems.append(f"{m['port']} {key} {m[key]} != {first[key]}")
    print(f"{po}: " + ("members consistent" if not problems else "; ".join(problems)))

def forms(a, b):
    if a in LACP and b in LACP:
        return "active" in (a, b)
    if a in PAGP and b in PAGP:
        return "desirable" in (a, b)
    return a == b == "on"

for po, (local, remote) in negotiation.items():
    print(f"{po} {local}/{remote}: {'forms' if forms(local, remote) else 'does not form'}")

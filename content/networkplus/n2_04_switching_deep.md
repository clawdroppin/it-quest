@@lesson n2-switchdeep Trunks, Spanning Tree, Link Aggregation and Switch Features
@@text
This lesson takes switching from "a switch learns MAC addresses" to the configuration you'll actually do: access and trunk ports, native VLANs, spanning tree port states and roles, link aggregation, MTU and jumbo frames, and the switch features that stop common attacks.

## Access and trunk ports

- **Access port**: belongs to one VLAN; the device sends untagged frames. A **voice VLAN** lets an IP phone tag voice traffic while the PC behind it stays on the data VLAN.
- **Trunk port**: carries many VLANs between switches (or to a router, hypervisor or AP) using **802.1Q tags** (4 bytes inserted into the frame, with a 12-bit VLAN ID: 1–4094).
- **Native VLAN**: the one VLAN sent **untagged** on a trunk. Both ends must agree, or traffic leaks between VLANs ("native VLAN mismatch" log messages). Best practice: set it to an unused VLAN, not VLAN 1.
- **Allowed VLAN list**: only trunk the VLANs that are needed.
- Disable automatic trunk negotiation (DTP) on user ports to stop **switch spoofing**.

Example (Cisco-style):

```text
interface Gi1/0/10
 switchport mode access
 switchport access vlan 20
 switchport voice vlan 50
 spanning-tree portfast
 spanning-tree bpduguard enable
interface Gi1/0/48
 switchport mode trunk
 switchport trunk native vlan 999
 switchport trunk allowed vlan 10,20,30,50
```

## Spanning Tree Protocol (802.1D, RSTP 802.1w, MSTP 802.1s)

Redundant links between switches create **loops**, which cause **broadcast storms** and MAC table instability. STP blocks redundant paths:

1. Elect the **root bridge**: the lowest **bridge ID** (priority, default 32768, then lowest MAC). Set the priority on the core switch so the election isn't left to the oldest MAC.
2. Each other switch picks its **root port**: lowest cost path to the root.
3. Each segment gets one **designated port**.
4. Everything else is **blocked** (RSTP: **alternate** or **backup** ports).

Port costs (short method): 10 Mb/s = 100, 100 Mb/s = 19, 1 Gb/s = 4, 10 Gb/s = 2.

Classic STP port states: **blocking → listening → learning → forwarding** (plus disabled); about 30–50 s to converge. **RSTP** converges in seconds (discarding, learning, forwarding).

Protective features:
- **PortFast**: edge ports go straight to forwarding (only for end devices).
- **BPDU guard**: shuts down an edge port if a switch is plugged in.
- **Root guard**: stops a downstream switch from becoming root.
- **Loop guard** and **storm control** (limits broadcast/multicast rates).

## Link aggregation

**802.3ad/802.1AX LACP** bundles several physical links into one logical link (port channel / EtherChannel / NIC teaming): more bandwidth and redundancy. Both sides must match speed, duplex, VLANs. A single flow still uses one member link (hashing on MAC/IP/ports), so 4 × 1 Gb/s doesn't make one file copy go 4 Gb/s.

## Speed, duplex and MTU

- **Auto-negotiation** on both ends. Hard-coding one side and leaving the other on auto causes a **duplex mismatch**: slow, with **late collisions** and CRC errors on the half-duplex side.
- **MTU**: 1500 bytes for Ethernet payload by default. **Jumbo frames** (9000) for storage (iSCSI) and VM migration networks: every device on the path must support them or packets are dropped/fragmented.
- Tunnels (GRE, IPsec, VXLAN) add overhead and reduce the effective MTU: a classic cause of "small pages load, big ones hang".

## Security features on switches

| Feature | Stops |
|---|---|
| **Port security** (max MACs, sticky MACs) | MAC flooding, unauthorised devices |
| **DHCP snooping** | rogue DHCP servers; builds a binding table |
| **Dynamic ARP inspection** (uses that table) | ARP spoofing/on-path attacks |
| **IP source guard** | IP spoofing |
| **802.1X** port-based NAC | unauthenticated devices |
| **Private VLANs** / port isolation | client-to-client traffic (hotel rooms, guest Wi-Fi) |
| Disable unused ports, put them in a "parking" VLAN | someone plugging into a spare jack |

## Other switch features

**Port mirroring/SPAN** copies traffic to a monitoring port (for Wireshark or an IDS). **LLDP/CDP** discover neighbours (handy for documentation; disable on untrusted edges). **Stacking** makes several switches one logical unit.

@@quiz
Users in VLAN 10 on switch A can't reach VLAN 10 on switch B, and logs show "native VLAN mismatch". What's wrong?
* The trunk's native VLAN differs on the two ends
- PoE budget exceeded
- The access port is in the wrong VLAN on switch A
- STP is disabled
? Both ends of a trunk must agree on the native VLAN.

@@quiz
Which switch becomes the STP root bridge?
- The one with the most ports
* The one with the lowest bridge ID (priority, then MAC)
- The one with the highest priority number
- The newest switch
? Lower is better in STP elections.

@@quiz
A user plugs a small home switch into an office jack, and the port shuts down. Which feature did that?
- PortFast
* BPDU guard
- LACP
- Storm control
? BPDU guard disables edge ports that receive BPDUs.

@@quiz
A server's link shows CRC errors and late collisions; it's set to 1000/full while the switch is on auto. What is the likely problem?
- A faulty SFP
* A duplex mismatch
- Jumbo frames
- A native VLAN mismatch
? The auto side falls back to half duplex.

@@quiz
Four 1 Gb/s links are bundled with LACP. A single large file copy between two hosts runs at about 1 Gb/s. Why?
* A single flow hashes to one member link
- LACP is misconfigured
- The bundle only works for UDP
- Jumbo frames are disabled
? Aggregation increases total capacity across many flows.

@@flashcards
access port :: one untagged VLAN
trunk port :: many VLANs, 802.1Q tags
native VLAN :: untagged VLAN on a trunk
voice VLAN :: tagged phone traffic on an access port
802.1Q tag :: 4 bytes, 12-bit VLAN ID
root bridge :: lowest bridge ID
default bridge priority :: 32768
STP states :: blocking, listening, learning, forwarding
RSTP :: rapid STP, seconds to converge
PortFast :: edge ports forward immediately
BPDU guard :: shuts edge ports that hear BPDUs
LACP :: 802.3ad link aggregation
duplex mismatch :: late collisions, CRC errors
jumbo frame :: MTU 9000
DHCP snooping :: trusted DHCP ports
DAI :: stops ARP spoofing
port mirroring :: copy traffic to a monitor port

@@drill
What is the default STP bridge priority?
= 32768
? Lower it (in steps of 4096) on the switch that should be root.

@@drill
What is the standard Ethernet MTU in bytes?
= 1500
? Jumbo frames are usually 9000.

@@exercise Elect the STP root and root ports
Each switch has a priority and MAC. Elect the root bridge (lowest `(priority, mac)`), then for every other switch choose the root port: the port with the lowest total path cost to the root (cost of the link plus the neighbour's own cost to root). Use the given topology (a small, loop-containing network) and Dijkstra-like relaxation over `links`. Print the root and each switch's root port with its cost.
@@lang python
@@starter
switches = {
    "SW-A": (32768, "00:1a:00:00:00:0a"),
    "SW-B": (32768, "00:1a:00:00:00:05"),
    "SW-C": (4096, "00:1a:00:00:00:0c"),
    "SW-D": (32768, "00:1a:00:00:00:0d"),
}
COST = {1: 4, 10: 2}
links = [
    ("SW-A", "Gi1", "SW-B", "Gi1", 1),
    ("SW-A", "Te1", "SW-C", "Te1", 10),
    ("SW-B", "Te2", "SW-C", "Te2", 10),
    ("SW-B", "Gi3", "SW-D", "Gi1", 1),
    ("SW-C", "Gi4", "SW-D", "Gi2", 1),
]
@@expected
Root bridge: SW-C
SW-A: root port Te1, cost 2
SW-B: root port Te2, cost 2
SW-D: root port Gi2, cost 4
@@requires
min(switches
COST[
@@hint
`root = min(switches, key=lambda s: switches[s])`. Start `dist = {root: 0}` and repeat relaxing every link in both directions until nothing changes, remembering which local port gave the best cost.
@@solution
switches = {
    "SW-A": (32768, "00:1a:00:00:00:0a"),
    "SW-B": (32768, "00:1a:00:00:00:05"),
    "SW-C": (4096, "00:1a:00:00:00:0c"),
    "SW-D": (32768, "00:1a:00:00:00:0d"),
}
COST = {1: 4, 10: 2}
links = [
    ("SW-A", "Gi1", "SW-B", "Gi1", 1),
    ("SW-A", "Te1", "SW-C", "Te1", 10),
    ("SW-B", "Te2", "SW-C", "Te2", 10),
    ("SW-B", "Gi3", "SW-D", "Gi1", 1),
    ("SW-C", "Gi4", "SW-D", "Gi2", 1),
]

root = min(switches, key=lambda s: switches[s])
print("Root bridge:", root)

dist = {root: 0}
root_port = {}
changed = True
while changed:
    changed = False
    for a, pa, b, pb, gbps in links:
        cost = COST[gbps]
        for here, port, there in ((a, pa, b), (b, pb, a)):
            if there in dist and dist[there] + cost < dist.get(here, float("inf")):
                dist[here] = dist[there] + cost
                root_port[here] = port
                changed = True

for sw in sorted(switches):
    if sw != root:
        print(f"{sw}: root port {root_port[sw]}, cost {dist[sw]}")

@@exercise Audit a trunk configuration
Compare both ends of each trunk. Report a native VLAN mismatch, VLANs allowed on one side but not the other, native VLAN 1 (best practice violation), and allowed VLANs that don't exist in the `vlans` database. Print `OK` for clean trunks.
@@lang python
@@starter
vlans = {10, 20, 30, 50, 999}
trunks = [
    ("SW1 Te1/0/1", {"native": 999, "allowed": {10, 20, 30, 50}}, "SW2 Te1/0/1", {"native": 999, "allowed": {10, 20, 30, 50}}),
    ("SW1 Te1/0/2", {"native": 999, "allowed": {10, 20, 30}}, "SW3 Te1/0/1", {"native": 1, "allowed": {10, 20, 30, 40}}),
    ("SW2 Gi1/0/48", {"native": 1, "allowed": {10, 20}}, "AP-07 eth0", {"native": 1, "allowed": {10, 20}}),
]
@@expected
SW1 Te1/0/1 <-> SW2 Te1/0/1: OK
SW1 Te1/0/2 <-> SW3 Te1/0/1: native VLAN mismatch (999 vs 1); native VLAN 1 in use; allowed on one side only: 40; unknown VLANs: 40
SW2 Gi1/0/48 <-> AP-07 eth0: native VLAN 1 in use
@@requires
^
- vlans
@@hint
Symmetric difference `left ^ right` gives VLANs allowed on only one side; `(left | right) - vlans` gives unknown ones.
@@solution
vlans = {10, 20, 30, 50, 999}
trunks = [
    ("SW1 Te1/0/1", {"native": 999, "allowed": {10, 20, 30, 50}}, "SW2 Te1/0/1", {"native": 999, "allowed": {10, 20, 30, 50}}),
    ("SW1 Te1/0/2", {"native": 999, "allowed": {10, 20, 30}}, "SW3 Te1/0/1", {"native": 1, "allowed": {10, 20, 30, 40}}),
    ("SW2 Gi1/0/48", {"native": 1, "allowed": {10, 20}}, "AP-07 eth0", {"native": 1, "allowed": {10, 20}}),
]

for left_name, left, right_name, right in trunks:
    issues = []
    if left["native"] != right["native"]:
        issues.append(f"native VLAN mismatch ({left['native']} vs {right['native']})")
    if 1 in (left["native"], right["native"]):
        issues.append("native VLAN 1 in use")
    one_sided = left["allowed"] ^ right["allowed"]
    if one_sided:
        issues.append("allowed on one side only: " + ", ".join(map(str, sorted(one_sided))))
    unknown = (left["allowed"] | right["allowed"]) - vlans
    if unknown:
        issues.append("unknown VLANs: " + ", ".join(map(str, sorted(unknown))))
    print(f"{left_name} <-> {right_name}: " + ("; ".join(issues) if issues else "OK"))

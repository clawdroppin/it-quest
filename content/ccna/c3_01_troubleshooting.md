@@lesson c3-troubleshooting Troubleshooting Cisco Networks with Show Commands
@@text
Most CCNA simulation questions, and most of the job, come down to: something doesn't work, find out why from `show` output. This lesson gives a method and the commands that prove or rule out each cause, layer by layer.

## A method

1. **Define the problem** precisely: who can't reach what, since when, what changed?
2. **Bottom-up** (physical → data link → network → transport → application) for unclear issues; **follow the path** (source → gateway → each hop → destination) for "A can't reach B"; **divide and conquer** from a ping that works.
3. Form a hypothesis, test it with one command, change **one thing at a time**, verify, document.

## Layer 1 and 2

| Check | Command | What you look for |
|---|---|---|
| Interface up? | `show ip interface brief` | admin down, down/down (cable), up/down (L2) |
| Errors? | `show interfaces g0/1` | CRC, input errors, late collisions, duplex/speed |
| Right VLAN? | `show vlan brief`, `show interfaces switchport` | access VLAN, port missing from the VLAN, VLAN doesn't exist |
| Trunks OK? | `show interfaces trunk` | allowed VLANs, **native VLAN mismatch**, pruned VLANs, not trunking (DTP modes) |
| Err-disabled? | `show interfaces status err-disabled` | port security, BPDU guard |
| MAC learned? | `show mac address-table address X` | which port the host is on |
| STP? | `show spanning-tree vlan X` | blocked ports, unexpected root |
| Neighbours? | `show cdp neighbors detail` | correct cabling, mismatched ports |

Classic layer 2 faults: an access port in the wrong VLAN; the VLAN missing from the trunk's allowed list (`switchport trunk allowed vlan add 30`, not without `add`, which **replaces** the list); a VLAN not created on an intermediate switch; DTP `auto`+`auto` → no trunk; native VLAN mismatch (CDP logs it).

## Layer 3

| Check | Command |
|---|---|
| Host IP settings | `ipconfig /all` / `ip addr`: IP, mask, **gateway**, DNS |
| Gateway reachable | ping the SVI/subinterface; `show ip interface brief` on the router |
| Route exists | `show ip route 10.2.5.9` (shows the longest match used) |
| Return route | the destination network needs a route **back** |
| Dynamic routing | `show ip ospf neighbor`, `show ip protocols`, `show ip ospf interface brief` |
| ARP | `show ip arp`, `show arp` (incomplete = no reply on that segment) |
| Path | `traceroute` (where does it stop?) |
| ACLs | `show access-lists` (hit counts), `show ip interface g0/0` (which ACL in/out) |
| NAT | `show ip nat translations`, `show ip nat statistics` |

Inter-VLAN routing faults: subinterface `encapsulation dot1q` with the wrong VLAN, the trunk to the router not carrying the VLAN, an SVI that is down because the VLAN has no active ports (or doesn't exist), `ip routing` missing on a layer 3 switch.

## Services

- **DHCP**: client has 169.254.x.x → no `ip helper-address`, pool exhausted (`show ip dhcp pool`), excluded range covers it, DHCP snooping dropping offers on an untrusted uplink.
- **DNS**: ping by IP works but not by name → check the DNS server setting and reachability (`nslookup`).
- **NTP**: `show ntp associations` (`*` = synchronised), `show clock`.
- **SSH**: `show ip ssh`, VTY `transport input`, `access-class` blocking the source, RSA keys missing.

## Reading ping and traceroute

IOS ping symbols: `!` success, `.` timeout, `U` destination unreachable (a router sent ICMP unreachable: often no route or an ACL), `Q` source quench, `M` could not fragment, `?` unknown packet type, `&` TTL exceeded. Extended ping (`ping` without arguments) lets you choose the **source interface**, which tests the return route. The first ping often shows `.!!!!`: the first packet was lost to ARP.

Traceroute: `* * *` from one hop onwards → the packet stops there (no route, ACL, or the device doesn't send ICMP TTL exceeded); a repeating pair of hops → a **routing loop**.

@@quiz
PC1 in VLAN 10 can't reach its gateway. `show interfaces trunk` shows "Vlans allowed on trunk: 20,30". What happened?
- The native VLAN is wrong
* VLAN 10 was removed from the allowed list (probably `allowed vlan` without `add`)
- STP is blocking the trunk
- DTP failed
? Without add, the command replaces the list.

@@quiz
Extended ping from R1 sourced from its LAN interface fails, but a normal ping succeeds. What does that suggest?
- R1's LAN interface is down
* The far side has no return route to R1's LAN subnet
- An ACL blocks ICMP everywhere
- ARP is broken
? The normal ping uses the outgoing interface's address.

@@quiz
A ping shows `U.U.U`. What does `U` mean?
- Success
- Timeout
* A router sent back an ICMP destination unreachable
- TTL exceeded
? Often no route or an ACL with unreachables enabled.

@@quiz
A host has address 169.254.12.7. Which is the most likely cause?
* It didn't receive a DHCP offer (relay, pool, or snooping issue)
- Duplicate IP address
- Wrong DNS server
- STP loop
? APIPA is used when DHCP fails.

@@quiz
Traceroute shows hops alternating between 10.0.0.1 and 10.0.0.2 until it times out. What is it?
- An ACL
* A routing loop
- A duplex mismatch
- Normal load balancing
? Each router points to the other.

@@quiz
An SVI on a layer 3 switch is up/down. What's the likely reason?
- `ip routing` missing
* The VLAN has no active (up, forwarding) ports or doesn't exist
- The gateway is wrong
- DHCP is down
? An SVI needs at least one up port in its VLAN.

@@flashcards
show ip interface brief :: status/protocol summary
show interfaces trunk :: allowed, native, forwarding VLANs
allowed vlan add :: adds without replacing
show interfaces status err-disabled :: why ports are shut
show ip route X :: route used for X
return route :: destination needs a route back
show ip arp incomplete :: no ARP reply
show access-lists :: hit counts
169.254.x.x :: DHCP failed
ping ! / . :: success / timeout
ping U :: unreachable from a router
.!!!! :: first packet lost to ARP
extended ping :: choose source interface
traceroute * * * :: stops at that hop
repeating hops :: routing loop
SVI up/down :: no active port in VLAN
ip routing :: enable routing on L3 switch
one change at a time :: troubleshooting rule

@@drill
Which ping symbol indicates a timeout?
= .
? ! is success.

@@drill
Which keyword adds a VLAN to a trunk's allowed list without replacing it?
= add
? switchport trunk allowed vlan add 30

@@exercise Trace a VLAN path across switches
`switches` describes access ports and trunks. For each user `(host, switch, port, gateway_switch)`, check that the access port is up and in a VLAN that exists on its switch, then follow `path` from the access switch to the gateway switch and check every trunk on the way carries the VLAN (and the VLAN exists on each switch). Print `OK` or the first problem for each host.
@@lang python
@@starter
switches = {
    "ACC1": {"vlans": {10, 20, 99}, "access": {"Gi1/0/1": (10, "up"), "Gi1/0/2": (20, "up"), "Gi1/0/3": (30, "up")}, "trunks": {"DIST1": {10, 20, 99}}},
    "ACC2": {"vlans": {10, 20, 30, 99}, "access": {"Gi1/0/1": (10, "err-disabled"), "Gi1/0/2": (30, "up")}, "trunks": {"DIST1": {10, 20, 99}}},
    "ACC3": {"vlans": {10, 20, 99}, "access": {"Gi1/0/1": (20, "up")}, "trunks": {"DIST1": {10, 20, 99}}},
    "DIST1": {"vlans": {10, 20, 30}, "access": {}, "trunks": {"ACC1": {10, 20, 99}, "ACC2": {10, 20, 30, 99}, "ACC3": {10, 99}}},
}
users = [("PC-A", "ACC1", "Gi1/0/1", "DIST1"), ("PC-B", "ACC1", "Gi1/0/3", "DIST1"), ("PC-C", "ACC2", "Gi1/0/1", "DIST1"),
         ("PC-D", "ACC2", "Gi1/0/2", "DIST1"), ("PC-E", "ACC3", "Gi1/0/1", "DIST1")]
path = {"ACC1": ["ACC1", "DIST1"], "ACC2": ["ACC2", "DIST1"], "ACC3": ["ACC3", "DIST1"]}
@@expected
PC-A on ACC1 Gi1/0/1: OK (VLAN 10)
PC-B on ACC1 Gi1/0/3: VLAN 30 not created on ACC1
PC-C on ACC2 Gi1/0/1: ACC2 Gi1/0/1 is err-disabled
PC-D on ACC2 Gi1/0/2: VLAN 30 not allowed on trunk ACC2->DIST1
PC-E on ACC3 Gi1/0/1: VLAN 20 not allowed on trunk DIST1->ACC3
@@requires
def check
zip(
@@hint
Walk consecutive pairs with `zip(hops, hops[1:])` and check both ends of each link: `switches[a]["trunks"][b]` and `switches[b]["trunks"][a]` must contain the VLAN.
@@solution
switches = {
    "ACC1": {"vlans": {10, 20, 99}, "access": {"Gi1/0/1": (10, "up"), "Gi1/0/2": (20, "up"), "Gi1/0/3": (30, "up")}, "trunks": {"DIST1": {10, 20, 99}}},
    "ACC2": {"vlans": {10, 20, 30, 99}, "access": {"Gi1/0/1": (10, "err-disabled"), "Gi1/0/2": (30, "up")}, "trunks": {"DIST1": {10, 20, 99}}},
    "ACC3": {"vlans": {10, 20, 99}, "access": {"Gi1/0/1": (20, "up")}, "trunks": {"DIST1": {10, 20, 99}}},
    "DIST1": {"vlans": {10, 20, 30}, "access": {}, "trunks": {"ACC1": {10, 20, 99}, "ACC2": {10, 20, 30, 99}, "ACC3": {10, 99}}},
}
users = [("PC-A", "ACC1", "Gi1/0/1", "DIST1"), ("PC-B", "ACC1", "Gi1/0/3", "DIST1"), ("PC-C", "ACC2", "Gi1/0/1", "DIST1"),
         ("PC-D", "ACC2", "Gi1/0/2", "DIST1"), ("PC-E", "ACC3", "Gi1/0/1", "DIST1")]
path = {"ACC1": ["ACC1", "DIST1"], "ACC2": ["ACC2", "DIST1"], "ACC3": ["ACC3", "DIST1"]}

def check(switch, port):
    vlan, status = switches[switch]["access"][port]
    if status != "up":
        return f"{switch} {port} is {status}"
    if vlan not in switches[switch]["vlans"]:
        return f"VLAN {vlan} not created on {switch}"
    for a, b in zip(path[switch], path[switch][1:]):
        if vlan not in switches[b]["vlans"]:
            return f"VLAN {vlan} not created on {b}"
        if vlan not in switches[a]["trunks"][b]:
            return f"VLAN {vlan} not allowed on trunk {a}->{b}"
        if vlan not in switches[b]["trunks"][a]:
            return f"VLAN {vlan} not allowed on trunk {b}->{a}"
    return f"OK (VLAN {vlan})"

for host, switch, port, gateway in users:
    print(f"{host} on {switch} {port}: {check(switch, port)}")

@@exercise Interpret ping and traceroute output
Classify each IOS ping result string (`!`, `.`, `U` symbols) as `success`, `success after ARP` (first `.` then all `!`), `intermittent loss`, `unreachable (ICMP from a router)` or `no reply`, with the success rate. Then analyse each traceroute (a list of hop addresses, `*` for no reply): report `reached` if the last hop equals the destination, `routing loop between X and Y` if an address repeats, otherwise `stops after X` (the last responding hop).
@@lang python
@@starter
pings = [".!!!!", "!!!!!", "!.!.!", "U.U.U", ".....", "!!!.!"]
traces = [
    ("10.2.5.9", ["10.0.0.1", "10.0.1.2", "10.2.5.9"]),
    ("10.3.0.1", ["10.0.0.1", "10.0.1.2", "*", "*", "*"]),
    ("10.4.0.1", ["10.0.0.1", "10.0.1.2", "10.0.0.1", "10.0.1.2", "10.0.0.1"]),
]

def classify(result):
    raise NotImplementedError
@@expected
.!!!!: success after ARP, 80% success
!!!!!: success, 100% success
!.!.!: intermittent loss, 60% success
U.U.U: unreachable (ICMP from a router), 0% success
.....: no reply, 0% success
!!!.!: intermittent loss, 80% success
traceroute 10.2.5.9: reached
traceroute 10.3.0.1: stops after 10.0.1.2
traceroute 10.4.0.1: routing loop between 10.0.0.1 and 10.0.1.2
@@requires
def classify
count("!")
@@hint
`rate = result.count("!") * 100 // len(result)`. Check the patterns in order: all `!`; `.` followed by all `!`; any `U`; no `!` at all; otherwise intermittent. For loops, remember the first index where each address appears.
@@solution
pings = [".!!!!", "!!!!!", "!.!.!", "U.U.U", ".....", "!!!.!"]
traces = [
    ("10.2.5.9", ["10.0.0.1", "10.0.1.2", "10.2.5.9"]),
    ("10.3.0.1", ["10.0.0.1", "10.0.1.2", "*", "*", "*"]),
    ("10.4.0.1", ["10.0.0.1", "10.0.1.2", "10.0.0.1", "10.0.1.2", "10.0.0.1"]),
]

def classify(result):
    rate = result.count("!") * 100 // len(result)
    if set(result) == {"!"}:
        kind = "success"
    elif result[0] == "." and set(result[1:]) == {"!"}:
        kind = "success after ARP"
    elif "U" in result:
        kind = "unreachable (ICMP from a router)"
    elif "!" not in result:
        kind = "no reply"
    else:
        kind = "intermittent loss"
    return f"{kind}, {rate}% success"

for p in pings:
    print(f"{p}: {classify(p)}")

for dest, hops in traces:
    seen = {}
    verdict = None
    for i, hop in enumerate(hops):
        if hop == "*":
            continue
        if hop in seen:
            other = hops[seen[hop] + 1]
            verdict = f"routing loop between {hop} and {other}"
            break
        seen[hop] = i
    if verdict is None:
        if hops[-1] == dest:
            verdict = "reached"
        else:
            last = [h for h in hops if h != "*"][-1]
            verdict = f"stops after {last}"
    print(f"traceroute {dest}: {verdict}")

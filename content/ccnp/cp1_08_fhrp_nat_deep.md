@@lesson cp1-fhrp-nat-deep First-Hop Redundancy and NAT in Depth: HSRP, VRRP, GLBP, PAT and Order of Operations
@@text
Hosts are configured with a single default gateway, so the gateway has to survive a router failure without anyone touching the hosts. That is the job of a **first-hop redundancy protocol** (FHRP). On the same edge routers you usually find **NAT**, which rewrites addresses as traffic leaves the site. ENCOR tests both at the level of "which router is active after this event?" and "which address does this packet carry at this point?", and this lesson works through both mechanically.

## The FHRP idea

Two or more routers share a **virtual IP** and a **virtual MAC**. Hosts use the virtual IP as their gateway, and ARP for it returns the virtual MAC. One router answers for that MAC at any time. When it fails, another router takes over the same IP and MAC and sends a **gratuitous ARP** so switches update their MAC tables. The hosts never notice: their ARP cache still points to the same MAC.

## HSRP

HSRP is Cisco proprietary and the one you'll configure most often.

| Item | HSRPv1 | HSRPv2 |
|---|---|---|
| Groups | 0–255 | 0–4095 |
| Virtual MAC | `0000.0c07.acXX` (XX = group in hex) | `0000.0c9f.fXXX` |
| Multicast | 224.0.0.2, UDP 1985 | 224.0.0.102, UDP 1985 |
| IPv6 | no | yes |

- Roles: one **active** router forwards, one **standby** router is ready to take over, and any others **listen**. The state sequence is Initial → Learn → Listen → Speak → Standby → Active.
- Timers: hello every **3 seconds**, hold time **10 seconds**. Millisecond timers (`standby 1 timers msec 200 msec 750`) or BFD make failover faster.
- **Election**: the highest **priority** wins (default **100**, range 0–255). A tie goes to the highest interface IP address.
- **Preemption is off by default.** A router with a higher priority that comes up later does *not* take over unless it has `standby 1 preempt`. This catches people out: you raise R1's priority to 110, and R2 stays active because R1 has no preempt. A `preempt delay minimum 60` gives routing protocols time to converge before the router takes the role back.
- **Object tracking** lowers the priority when something else fails, typically the uplink: `track 10 interface g0/1 line-protocol`, then `standby 1 track 10 decrement 30`. If R1 (110) loses its uplink it drops to 80, below R2's 100, and R2 takes over, but only if **R2** has preempt configured. Tracking without preempt on the peer does nothing useful.

```
interface Vlan10
 ip address 10.10.10.2 255.255.255.0
 standby version 2
 standby 10 ip 10.10.10.1
 standby 10 priority 110
 standby 10 preempt delay minimum 60
 standby 10 track 10 decrement 30
 standby 10 authentication md5 key-string <key>
```

A common campus design puts the HSRP active router for each VLAN on the same distribution switch as the **STP root** for that VLAN, so traffic doesn't take an extra hop across the inter-switch link. VLAN 10 is active on D1 and VLAN 20 on D2, which also shares the load.

## VRRP

VRRP is the open standard (RFC 5798 for VRRPv3, which covers IPv4 and IPv6).

- Roles are **master** and **backup**.
- Virtual MAC `0000.5e00.01XX`, multicast **224.0.0.18**, IP protocol **112**, advertisements every **1 second**.
- Default priority 100. The router that actually owns the virtual IP as its interface address (the **address owner**) gets priority **255** and always wins.
- **Preemption is on by default**, the opposite of HSRP.
- On IOS XE, VRRPv3 is configured with `fhrp version vrrp v3` and `vrrp 10 address-family ipv4`.

## GLBP

GLBP (Cisco) load-balances in a single group, where HSRP and VRRP only use one router per group.

- The **AVG** (active virtual gateway) answers ARP requests for the virtual IP, handing out a different virtual MAC to different hosts.
- Up to **four AVFs** (active virtual forwarders) each own one virtual MAC, `0007.b400.XXYY`, and forward traffic sent to it.
- Load balancing is **round-robin** by default; it can be **weighted** or **host-dependent** (the same host always gets the same MAC).
- Multicast 224.0.0.102, UDP 3222.
- If an AVF fails, another AVF takes over its virtual MAC, so hosts that cached it keep working.

## NAT terms

Cisco names addresses from the point of view of the **inside** network:

| Term | Meaning | Example |
|---|---|---|
| **Inside local** | the inside host's real address | 10.1.1.10 |
| **Inside global** | that host as the outside world sees it | 203.0.113.5 |
| **Outside local** | the outside host as the inside sees it | usually the same as outside global |
| **Outside global** | the outside host's real address | 198.51.100.20 |

"Local" means "as seen from the inside", "global" means "as seen from the outside". Outside local only differs from outside global when you also translate the outside addresses (for example with overlapping networks after a merger).

## Types of NAT

- **Static NAT**: one-to-one, permanent, works for connections started from the outside. `ip nat inside source static 10.1.1.20 203.0.113.20`.
- **Static PAT (port forwarding)**: one inside service on one outside port. `ip nat inside source static tcp 10.1.1.20 443 203.0.113.5 443`.
- **Dynamic NAT**: inside addresses matched by an ACL take a free address from a **pool**, one-to-one. When the pool runs out, new hosts can't get out.
- **PAT (overload)**: many inside addresses share one global address (or a small pool), told apart by **port**. `ip nat inside source list NAT-ACL interface g0/1 overload`. The router tries to keep the original source port, and picks another free port when that one is already in use for the same global address.

```
interface g0/0
 ip nat inside
interface g0/1
 ip nat outside
ip access-list standard NAT-ACL
 permit 10.1.0.0 0.0.255.255
ip nat inside source list NAT-ACL interface g0/1 overload
```

Dynamic translations only exist after an inside host sends traffic out, so **unsolicited inbound traffic finds no entry and is dropped**. That is why a server needs a static entry. Inspect with `show ip nat translations` and `show ip nat statistics`, and remove dynamic entries with `clear ip nat translation *`.

## Order of operations

The router does routing and NAT in a fixed order, and it depends on the direction:

- **Inside → outside**: the router **routes first, then translates**. The routing decision uses the inside local source; the packet leaves with the inside global source.
- **Outside → inside**: the router **translates first, then routes**. The destination is changed from inside global to inside local, then routed to the inside host.

Consequences you'll meet on the exam:

- An **inbound ACL on the outside interface** sees the **inside global** destination (before translation). An **outbound ACL on the outside interface** sees the **inside global** source (after translation).
- A **policy-based route** on the inside interface sees the original private addresses.
- NAT breaks protocols that carry addresses inside the payload (FTP active mode, SIP, some VPNs). IOS has **ALGs** for common ones; IPsec needs **NAT-T** (UDP 4500).

## Putting FHRP and NAT together

With two edge routers running HSRP towards the inside and each doing PAT to its own ISP, a failover moves the gateway but not existing translations: sessions translated on R1 break when R2 takes over, because R2 has no entry for them and uses a different global address. Stateful NAT with HSRP (SNAT) or a firewall pair with state sync solves this; most designs simply accept that sessions reconnect.

@@quiz
R1 has HSRP priority 110 without preempt and R2 has priority 100 with preempt. R2 is active when R1 boots. What happens?
* R2 stays active, because R1 has no preempt and can't take the role back
- R1 becomes active, because it has the higher priority
- Both routers become active
- R2 lowers its priority to 90
? Only a router with preempt configured takes over from a lower-priority active router.

@@quiz
Which FHRP has preemption enabled by default?
* VRRP
- HSRP
- GLBP for the AVG role
- None of them
? HSRP and GLBP's AVG need preempt configured; VRRP preempts by default.

@@quiz
R1 (priority 110) tracks its uplink with a decrement of 20. R2 has priority 100 and preempt. The uplink fails. Who is active?
* R2, because R1 drops to 90
- R1, because tracking only logs the failure
- R1, because 110 − 20 is still above 100
- Neither until the hold timer expires twice
? 110 − 20 = 90 < 100, and R2 can preempt.

@@quiz
In NAT, what is the inside global address?
* The inside host's address as the outside world sees it
- The inside host's real private address
- The outside server's real address
- The outside server's address as seen from inside
? Local and global describe where the address is seen from.

@@quiz
A packet goes from an inside host to the internet. In what order does the router apply routing and NAT?
* Routing first, then NAT
- NAT first, then routing
- Both at the same time
- It depends on whether the ACL is standard or extended
? Inside to outside: route, then translate. Outside to inside: translate, then route.

@@quiz
Why does a web server behind PAT need a static entry?
* Dynamic translations are created only by outbound traffic, so unsolicited inbound connections find no entry
- PAT can't translate TCP port 443
- Static entries are faster to look up
- Dynamic NAT only works for UDP
? Port forwarding with `ip nat inside source static tcp ...` creates a permanent entry.

@@flashcards
HSRP default priority :: 100 (highest wins, tie goes to the highest IP)
HSRP hello / hold :: 3 seconds / 10 seconds
HSRPv1 virtual MAC :: 0000.0c07.acXX (XX = group number in hex)
HSRPv2 virtual MAC :: 0000.0c9f.fXXX, groups 0–4095, multicast 224.0.0.102
VRRP virtual MAC :: 0000.5e00.01XX, multicast 224.0.0.18, IP protocol 112
VRRP address owner :: the router whose interface IP is the virtual IP; priority 255
GLBP roles :: one AVG answers ARP; up to four AVFs forward traffic
Inside local :: the inside host's real address
Inside global :: the inside host as seen from outside
Inside to outside :: route first, then translate
Outside to inside :: translate first, then route
PAT :: many inside hosts share one global address, told apart by port

@@drill
What is the HSRPv1 virtual MAC address for group 10?
= 0000.0c07.ac0a
= 00:00:0c:07:ac:0a
? 10 is 0a in hex.

@@drill
What priority does a VRRP router get when it owns the virtual IP address?
= 255
? The address owner always becomes master.

@@exercise HSRP election with preempt and tracking
@@concepts networking, conditions, loops, maps
Simulate HSRP for one group. Each router has a priority, an interface IP, a preempt flag and tracked objects that lower its priority by a decrement when they go down. Apply a list of events in order (a router boots or fails, a tracked uplink goes down or comes back) and print the active router after each event. Rules: with no active router, the highest effective priority wins (tie: highest IP). An active router keeps the role unless it fails, or a router with a **higher** effective priority **and** preempt takes over.
@@lang python
@@starter
from ipaddress import ip_address

routers = {
    "D1": {"ip": "10.10.10.2", "priority": 110, "preempt": True, "track": {"uplink-D1": 30}, "up": False},
    "D2": {"ip": "10.10.10.3", "priority": 100, "preempt": False, "track": {}, "up": False},
}
objects = {"uplink-D1": True}
events = [("boot", "D2"), ("boot", "D1"), ("track-down", "uplink-D1"), ("track-up", "uplink-D1"),
          ("fail", "D1"), ("boot", "D1")]

def effective_priority(name):
    raise NotImplementedError

def elect(active):
    raise NotImplementedError

active = None
for kind, target in events:
    if kind in ("boot", "fail"):
        routers[target]["up"] = kind == "boot"
    else:
        objects[target] = kind == "track-up"
    active = elect(active)
    prios = ", ".join(f"{n}={effective_priority(n)}" for n, r in routers.items() if r["up"])
    print(f"{kind:<10} {target:<10} active={active}  ({prios})")
@@expected
boot       D2         active=D2  (D2=100)
boot       D1         active=D1  (D1=110, D2=100)
track-down uplink-D1  active=D1  (D1=80, D2=100)
track-up   uplink-D1  active=D1  (D1=110, D2=100)
fail       D1         active=D2  (D2=100)
boot       D1         active=D1  (D1=110, D2=100)
@@requires
def effective_priority
def elect
ip_address(r["ip"])
@@hint
The effective priority is the configured priority minus the decrement of every tracked object that is down. In `elect`, list the routers that are up. If the current active router is down (or there is none), return the best one by `(effective priority, ip_address(ip))`. Otherwise return the best one only if it differs from the active router, has preempt and has a strictly higher priority; else keep the active router.
@@solution
from ipaddress import ip_address

routers = {
    "D1": {"ip": "10.10.10.2", "priority": 110, "preempt": True, "track": {"uplink-D1": 30}, "up": False},
    "D2": {"ip": "10.10.10.3", "priority": 100, "preempt": False, "track": {}, "up": False},
}
objects = {"uplink-D1": True}
events = [("boot", "D2"), ("boot", "D1"), ("track-down", "uplink-D1"), ("track-up", "uplink-D1"),
          ("fail", "D1"), ("boot", "D1")]

def effective_priority(name):
    r = routers[name]
    return r["priority"] - sum(dec for obj, dec in r["track"].items() if not objects[obj])

def elect(active):
    up = [n for n, r in routers.items() if r["up"]]
    if not up:
        return None
    def rank(n):
        r = routers[n]
        return (effective_priority(n), ip_address(r["ip"]))
    best = max(up, key=rank)
    if active not in up:
        return best
    if best != active and routers[best]["preempt"] and effective_priority(best) > effective_priority(active):
        return best
    return active

active = None
for kind, target in events:
    if kind in ("boot", "fail"):
        routers[target]["up"] = kind == "boot"
    else:
        objects[target] = kind == "track-up"
    active = elect(active)
    prios = ", ".join(f"{n}={effective_priority(n)}" for n, r in routers.items() if r["up"])
    print(f"{kind:<10} {target:<10} active={active}  ({prios})")

@@exercise Build a PAT translation table
Simulate PAT (overload) on one outside address, 203.0.113.5. For each outbound connection, reuse an existing translation for the same inside socket, keep the original source port if that global port is free, and otherwise take the lowest free port from 1024 up (a simplified model of what IOS does). Print the table in the style of `show ip nat translations`, then translate inbound packets back: a packet to a known global port goes to its inside socket; anything else is dropped unless a static port forward exists.
@@lang python
@@starter
GLOBAL = "203.0.113.5"
static = {("tcp", 443): ("10.1.1.20", 443)}
outbound = [
    ("tcp", "10.1.1.10", 51000, "198.51.100.20", 443),
    ("tcp", "10.1.1.11", 51000, "198.51.100.20", 443),
    ("udp", "10.1.1.10", 53000, "8.8.8.8", 53),
    ("tcp", "10.1.1.12", 1024, "198.51.100.30", 80),
    ("tcp", "10.1.1.10", 51000, "198.51.100.20", 443),
]
inbound = [("tcp", 51000), ("tcp", 1024), ("tcp", 1025), ("tcp", 443), ("udp", 5060)]

table = {}

def translate_out(proto, src, sport, dst, dport):
    raise NotImplementedError

def translate_in(proto, gport):
    raise NotImplementedError

for packet in outbound:
    translate_out(*packet)
print(f"{'Pro':<4} {'Inside global':<20} {'Inside local':<17} {'Outside global':<20}".rstrip())
for (proto, gport), (src, sport, dst, dport) in table.items():
    print(f"{proto:<4} {GLOBAL + ':' + str(gport):<20} {src + ':' + str(sport):<17} {dst + ':' + str(dport)}")
for proto, gport in inbound:
    print(f"inbound {proto} {GLOBAL}:{gport} -> {translate_in(proto, gport)}")
@@expected
Pro  Inside global        Inside local      Outside global
tcp  203.0.113.5:51000    10.1.1.10:51000   198.51.100.20:443
tcp  203.0.113.5:1024     10.1.1.11:51000   198.51.100.20:443
udp  203.0.113.5:53000    10.1.1.10:53000   8.8.8.8:53
tcp  203.0.113.5:1025     10.1.1.12:1024    198.51.100.30:80
inbound tcp 203.0.113.5:51000 -> 10.1.1.10:51000
inbound tcp 203.0.113.5:1024 -> 10.1.1.11:51000
inbound tcp 203.0.113.5:1025 -> 10.1.1.12:1024
inbound tcp 203.0.113.5:443 -> 10.1.1.20:443 (static port forward)
inbound udp 203.0.113.5:5060 -> dropped (no translation)
@@requires
def translate_out
def translate_in
while (proto, port) in table
@@hint
In `translate_out`, first look for an entry with the same protocol, inside address and port, and return its global port. Otherwise start with `port = sport`; if `(proto, port)` is taken (in `table` or `static`), set `port = 1024` and step up with `while (proto, port) in table or (proto, port) in static`. In `translate_in`, check `table`, then `static`, else return "dropped (no translation)".
@@solution
GLOBAL = "203.0.113.5"
static = {("tcp", 443): ("10.1.1.20", 443)}
outbound = [
    ("tcp", "10.1.1.10", 51000, "198.51.100.20", 443),
    ("tcp", "10.1.1.11", 51000, "198.51.100.20", 443),
    ("udp", "10.1.1.10", 53000, "8.8.8.8", 53),
    ("tcp", "10.1.1.12", 1024, "198.51.100.30", 80),
    ("tcp", "10.1.1.10", 51000, "198.51.100.20", 443),
]
inbound = [("tcp", 51000), ("tcp", 1024), ("tcp", 1025), ("tcp", 443), ("udp", 5060)]

table = {}

def taken(proto, port):
    return (proto, port) in table or (proto, port) in static

def translate_out(proto, src, sport, dst, dport):
    for (p, gport), (s, sp, _, _) in table.items():
        if p == proto and s == src and sp == sport:
            return gport
    port = sport
    if taken(proto, port):
        port = 1024
        while (proto, port) in table or (proto, port) in static:
            port += 1
    table[(proto, port)] = (src, sport, dst, dport)
    return port

def translate_in(proto, gport):
    if (proto, gport) in table:
        src, sport, _, _ = table[(proto, gport)]
        return f"{src}:{sport}"
    if (proto, gport) in static:
        src, sport = static[(proto, gport)]
        return f"{src}:{sport} (static port forward)"
    return "dropped (no translation)"

for packet in outbound:
    translate_out(*packet)
print(f"{'Pro':<4} {'Inside global':<20} {'Inside local':<17} {'Outside global':<20}".rstrip())
for (proto, gport), (src, sport, dst, dport) in table.items():
    print(f"{proto:<4} {GLOBAL + ':' + str(gport):<20} {src + ':' + str(sport):<17} {dst + ':' + str(dport)}")
for proto, gport in inbound:
    print(f"inbound {proto} {GLOBAL}:{gport} -> {translate_in(proto, gport)}")

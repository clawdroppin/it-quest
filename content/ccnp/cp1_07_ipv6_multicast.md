@@lesson cp1-ipv6-multicast IPv6 and Multicast in Depth: EUI-64, Address Types, IGMP, PIM and RPF
@@text
ENCOR expects you to work with IPv6 addressing at the bit level and to explain how multicast is forwarded. Both topics come down to a few mechanical rules, and this lesson practises them.

## IPv6 address types

| Prefix | Type |
|---|---|
| `2000::/3` | global unicast |
| `fe80::/10` | link-local (every interface, used as the next hop by routing protocols) |
| `fc00::/7` (in practice `fd00::/8`) | unique local |
| `ff00::/8` | multicast (`ff02::1` all nodes, `ff02::2` all routers, `ff02::5`/`::6` OSPFv3, `ff02::a` EIGRP) |
| `ff02::1:ffXX:XXXX` | solicited-node multicast: the last 24 bits of the unicast address, used by NDP instead of ARP |
| `::1`, `::` | loopback, unspecified |

**Modified EUI-64**: split the 48-bit MAC in half, insert `fffe` in the middle, and **flip the 7th bit** (the universal/local bit) of the first byte. `00:1a:2b:3c:4d:5e` becomes the interface ID `021a:2bff:fe3c:4d5e`.

SLAAC: the router advertises the prefix in an RA, and the host builds its own address (EUI-64 or random/privacy). DHCPv6 is stateful or stateless, controlled by the M and O flags in the RA.

## Multicast addressing

- IPv4 multicast is `224.0.0.0/4`. `224.0.0.0/24` is link-local (OSPF uses `224.0.0.5`/`.6`), `232.0.0.0/8` is SSM, and `239.0.0.0/8` is administratively scoped.
- **Multicast MAC**: `01:00:5e` + 0 + the low **23 bits** of the group address. Since 5 bits are lost, **32 groups share each MAC** (224.1.1.1 and 239.129.1.1 collide).

## IGMP and snooping

- Hosts join with an **IGMP** Membership Report. IGMPv2 adds Leave, and IGMPv3 adds source lists for SSM.
- **IGMP snooping**: the switch listens to reports and forwards a group only to ports that joined, instead of flooding.

## PIM

- **PIM dense mode**: flood and prune (rare now).
- **PIM sparse mode**: receivers join a shared tree (**RPT**, *,G) rooted at the **Rendezvous Point**. Sources register with the RP. Last-hop routers can switch to the shortest-path tree (**SPT**, S,G).
- **SSM**: no RP. Receivers name the source (IGMPv3), and the tree is (S,G) straight away.
- **RPF check**: a multicast packet is accepted only if it arrives on the interface the unicast routing table would use to reach the **source** (or the RP, for the shared tree). This prevents loops. Most multicast "nothing arrives" problems are RPF failures caused by asymmetric routing.

@@quiz
What is the modified EUI-64 interface ID for MAC 00:1a:2b:3c:4d:5e?
* 021a:2bff:fe3c:4d5e
- 001a:2bff:fe3c:4d5e
- 021a:2b3c:4d5e:fffe
- fe80::1a:2b3c:4d5e
? Insert fffe and flip the 7th bit of the first byte.

@@quiz
How many IPv4 multicast groups map to each multicast MAC address?
- 1
- 8
* 32
- 256
? 5 bits of the 28-bit group are lost.

@@quiz
A multicast packet arrives on an interface that isn't the unicast route back to the source. What happens?
- It is forwarded to all ports
* It fails the RPF check and is dropped
- It is sent to the RP
- It is converted to unicast
? RPF prevents loops.

@@quiz
Which PIM mode needs no Rendezvous Point?
- Sparse mode
* Source-Specific Multicast
- Bidirectional PIM
- Auto-RP
? Receivers specify the source with IGMPv3.

@@quiz
Which address does NDP use to resolve 2001:db8::12:3456 instead of a broadcast?
* ff02::1:ff12:3456
- ff02::1
- ff02::2
- fe80::12:3456
? The solicited-node address uses the last 24 bits.

@@flashcards
2000::/3 :: global unicast
fe80::/10 :: link-local
fd00::/8 :: unique local
ff02::1 / ff02::2 :: all nodes / all routers
solicited-node :: ff02::1:ff + last 24 bits
EUI-64 :: insert fffe, flip bit 7
SLAAC :: host builds an address from the RA prefix
M / O flags :: stateful / stateless DHCPv6
multicast MAC :: 01:00:5e + low 23 bits
232.0.0.0/8 :: SSM range
IGMP snooping :: forward groups only to joined ports
RP :: root of the shared tree
(*,G) / (S,G) :: shared tree / source tree
SPT switchover :: last hop moves to the source tree
RPF check :: arrive on the unicast path to the source

@@drill
What is the multicast MAC address for group 239.1.2.3?
= 01:00:5e:01:02:03
= 01-00-5e-01-02-03
= 0100.5e01.0203
? The high bit of the second octet is dropped (1 is below 128).

@@drill
Which IPv6 multicast address do OSPFv3 routers use for all SPF routers?
= ff02::5
? ff02::6 is for DRs.

@@exercise EUI-64, solicited-node and address types
Build modified EUI-64 interface IDs from MAC addresses, form the full address from a /64 prefix, compute the solicited-node multicast address, and classify a list of IPv6 addresses by type, using bit operations and the `ipaddress` module only for formatting.
@@concepts networking, numbers, strings, maps, conditions
@@lang python
@@starter
import ipaddress

def eui64(mac):
    raise NotImplementedError

def solicited_node(address):
    raise NotImplementedError

TYPES = [("::1/128", "loopback"), ("fe80::/10", "link-local"), ("fc00::/7", "unique local"),
         ("ff00::/8", "multicast"), ("2000::/3", "global unicast")]
@@expected
00:1a:2b:3c:4d:5e -> 021a:2bff:fe3c:4d5e -> 2001:db8:acad:1:21a:2bff:fe3c:4d5e, solicited-node ff02::1:ff3c:4d5e
02:00:5e:10:00:01 -> 0000:5eff:fe10:0001 -> 2001:db8:acad:1:0:5eff:fe10:1, solicited-node ff02::1:ff10:1
a4-83-e7-12-9b-f0 -> a683:e7ff:fe12:9bf0 -> 2001:db8:acad:1:a683:e7ff:fe12:9bf0, solicited-node ff02::1:ff12:9bf0
fe80::21a:2bff:fe3c:4d5e   link-local
fd12:3456::1               unique local
ff02::a                    multicast
2400:cb00::1               global unicast
::1                        loopback
::                         other
@@requires
def eui64
fffe
0x02
def solicited_node
@@hint
EUI-64: take the 6 bytes, flip bit `0x02` of the first byte with `^`, and insert `0xff, 0xfe` after the third byte. Join them as 4 groups of 2 bytes. Solicited-node: take the address's integer value `& 0xFFFFFF` and add it to `ff02::1:ff00:0`. Classify with the first matching network in `TYPES`.
@@solution
import ipaddress

def eui64(mac):
    b = [int(x, 16) for x in mac.replace("-", ":").split(":")]
    b[0] ^= 0x02
    full = b[:3] + [0xff, 0xfe] + b[3:]   # fffe in the middle
    return ":".join(f"{full[i]:02x}{full[i + 1]:02x}" for i in range(0, 8, 2))

def solicited_node(address):
    low24 = int(ipaddress.IPv6Address(address)) & 0xFFFFFF
    return str(ipaddress.IPv6Address(int(ipaddress.IPv6Address("ff02::1:ff00:0")) + low24))

TYPES = [("::1/128", "loopback"), ("fe80::/10", "link-local"), ("fc00::/7", "unique local"),
         ("ff00::/8", "multicast"), ("2000::/3", "global unicast")]

def kind(address):
    ip = ipaddress.IPv6Address(address)
    return next((name for net, name in TYPES if ip in ipaddress.IPv6Network(net)), "other")

for mac in ("00:1a:2b:3c:4d:5e", "02:00:5e:10:00:01", "a4-83-e7-12-9b-f0"):
    iid = eui64(mac)
    address = ipaddress.IPv6Address(f"2001:db8:acad:1:{iid}")
    print(f"{mac} -> {iid} -> {address}, solicited-node {solicited_node(str(address))}")
for a in ("fe80::21a:2bff:fe3c:4d5e", "fd12:3456::1", "ff02::a", "2400:cb00::1", "::1", "::"):
    print(f"{a:<26} {kind(a)}")

@@exercise Multicast MACs and RPF checks
Map IPv4 groups to multicast MAC addresses and find groups that collide on the same MAC, then run RPF checks: a packet from a source passes only if it arrived on the interface the unicast routing table (longest-prefix match) uses towards that source.
@@lang python
@@starter
import ipaddress
groups = ["224.1.1.1", "239.129.1.1", "239.1.2.3", "225.1.2.3", "232.10.0.5"]
routes = {"0.0.0.0/0": "Gi0/0", "10.10.0.0/16": "Gi0/1", "10.10.20.0/24": "Gi0/2", "192.168.50.0/24": "Gi0/3"}
packets = [("10.10.20.7", "239.1.2.3", "Gi0/2"), ("10.10.30.7", "239.1.2.3", "Gi0/2"),
           ("192.168.50.10", "232.10.0.5", "Gi0/3"), ("8.8.4.4", "225.1.2.3", "Gi0/1")]

def multicast_mac(group):
    raise NotImplementedError

def rpf_interface(source):
    raise NotImplementedError
@@expected
224.1.1.1    -> 01:00:5e:01:01:01
239.129.1.1  -> 01:00:5e:01:01:01
239.1.2.3    -> 01:00:5e:01:02:03
225.1.2.3    -> 01:00:5e:01:02:03
232.10.0.5   -> 01:00:5e:0a:00:05
collision on 01:00:5e:01:01:01: 224.1.1.1, 239.129.1.1 (switches can't tell them apart)
collision on 01:00:5e:01:02:03: 239.1.2.3, 225.1.2.3 (switches can't tell them apart)
(10.10.20.7, 239.1.2.3) in Gi0/2: RPF pass, forward
(10.10.30.7, 239.1.2.3) in Gi0/2: RPF FAIL (route 10.10.0.0/16 points to Gi0/1), drop
(192.168.50.10, 232.10.0.5) in Gi0/3: RPF pass, forward
(8.8.4.4, 225.1.2.3) in Gi0/1: RPF FAIL (route 0.0.0.0/0 points to Gi0/0), drop
@@requires
def multicast_mac
0x7FFFFF
def rpf_interface
prefixlen
@@hint
MAC: `low23 = int(group) & 0x7FFFFF`, then `0x01005e000000 | low23`, formatted as six hex pairs. Group the addresses by MAC to find collisions. RPF: pick the route containing the source with the largest `prefixlen` and compare its interface with the arrival interface.
@@solution
import ipaddress
groups = ["224.1.1.1", "239.129.1.1", "239.1.2.3", "225.1.2.3", "232.10.0.5"]
routes = {"0.0.0.0/0": "Gi0/0", "10.10.0.0/16": "Gi0/1", "10.10.20.0/24": "Gi0/2", "192.168.50.0/24": "Gi0/3"}
packets = [("10.10.20.7", "239.1.2.3", "Gi0/2"), ("10.10.30.7", "239.1.2.3", "Gi0/2"),
           ("192.168.50.10", "232.10.0.5", "Gi0/3"), ("8.8.4.4", "225.1.2.3", "Gi0/1")]

def multicast_mac(group):
    low23 = int(ipaddress.IPv4Address(group)) & 0x7FFFFF
    value = 0x01005e000000 | low23
    return ":".join(f"{(value >> s) & 0xff:02x}" for s in range(40, -1, -8))

def rpf_interface(source):
    ip = ipaddress.IPv4Address(source)
    matches = [ipaddress.IPv4Network(r) for r in routes if ip in ipaddress.IPv4Network(r)]
    best = max(matches, key=lambda n: n.prefixlen)
    return routes[str(best)], str(best)

by_mac = {}
for g in groups:
    mac = multicast_mac(g)
    by_mac.setdefault(mac, []).append(g)
    print(f"{g:<12} -> {mac}")
for mac, gs in by_mac.items():
    if len(gs) > 1:
        print(f"collision on {mac}: {', '.join(gs)} (switches can't tell them apart)")

for source, group, arrived in packets:
    expected, route = rpf_interface(source)
    result = "RPF pass, forward" if expected == arrived else f"RPF FAIL (route {route} points to {expected}), drop"
    print(f"({source}, {group}) in {arrived}: {result}")

@@exercise Plan /64 subnets from a /48
A site has `2001:db8:acad::/48`, which leaves 16 bits of subnet ID, or 65,536 /64s. Use a readable scheme: the first hex digit of the subnet ID is the **building** (0–15) and the remaining three hex digits are the **VLAN ID** (up to 4094 = `ffe`), so building 2, VLAN 10 is `2001:db8:acad:200a::/64`. Build the subnet for each VLAN, show each building's /52, and print the first host address (`::1`, the gateway) of each subnet.
@@lang python
@@starter
import ipaddress

SITE = ipaddress.ip_network("2001:db8:acad::/48")
vlans = [(0, 10, "core mgmt"), (1, 10, "B1 users"), (1, 20, "B1 voice"), (2, 10, "B2 users"),
         (2, 100, "B2 wifi"), (2, 4094, "B2 native")]

def subnet(building, vlan):
    raise NotImplementedError

def building_block(building):
    raise NotImplementedError

print(f"{SITE} holds {2 ** (64 - SITE.prefixlen):,} /64 subnets")
for b in sorted({b for b, _, _ in vlans}):
    print(f"building {b}: {building_block(b)}")
for b, v, name in vlans:
    net = subnet(b, v)
    print(f"  {name:<10} vlan {v:<4} {str(net):<26} gateway {net.network_address + 1}")
@@expected
2001:db8:acad::/48 holds 65,536 /64 subnets
building 0: 2001:db8:acad::/52
building 1: 2001:db8:acad:1000::/52
building 2: 2001:db8:acad:2000::/52
  core mgmt  vlan 10   2001:db8:acad:a::/64       gateway 2001:db8:acad:a::1
  B1 users   vlan 10   2001:db8:acad:100a::/64    gateway 2001:db8:acad:100a::1
  B1 voice   vlan 20   2001:db8:acad:1014::/64    gateway 2001:db8:acad:1014::1
  B2 users   vlan 10   2001:db8:acad:200a::/64    gateway 2001:db8:acad:200a::1
  B2 wifi    vlan 100  2001:db8:acad:2064::/64    gateway 2001:db8:acad:2064::1
  B2 native  vlan 4094 2001:db8:acad:2ffe::/64    gateway 2001:db8:acad:2ffe::1
@@requires
def subnet
def building_block
<< 64
@@hint
The subnet ID is `building * 0x1000 + vlan` (one hex digit for the building, three for the VLAN). Shift it into place with `<< 64` and add it to the /48's network address as an integer: `ipaddress.ip_network((int(SITE.network_address) + (sid << 64), 64))`. A building's /52 starts at `building << 76` with prefix length 52.
@@solution
import ipaddress

SITE = ipaddress.ip_network("2001:db8:acad::/48")
vlans = [(0, 10, "core mgmt"), (1, 10, "B1 users"), (1, 20, "B1 voice"), (2, 10, "B2 users"),
         (2, 100, "B2 wifi"), (2, 4094, "B2 native")]

def subnet(building, vlan):
    if not (0 <= building <= 15 and 1 <= vlan <= 4094):
        raise ValueError("building must be 0-15 and VLAN 1-4094")
    sid = building * 0x1000 + vlan
    return ipaddress.ip_network((int(SITE.network_address) + (sid << 64), 64))

def building_block(building):
    return ipaddress.ip_network((int(SITE.network_address) + (building << 76), 52))

print(f"{SITE} holds {2 ** (64 - SITE.prefixlen):,} /64 subnets")
for b in sorted({b for b, _, _ in vlans}):
    print(f"building {b}: {building_block(b)}")
for b, v, name in vlans:
    net = subnet(b, v)
    print(f"  {name:<10} vlan {v:<4} {str(net):<26} gateway {net.network_address + 1}")

@@lesson c0-subnetting Subnetting Speed, VLSM and Wildcard Masks
@@text
CCNA questions expect you to subnet in your head in under a minute: find the network, broadcast and usable range of an address, pick the right prefix for a host count, and write the **wildcard masks** that ACLs and OSPF use. This lesson builds that speed with a single method and lots of practice.

## Powers of two you must know

| Bits | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
|---|---|---|---|---|---|---|---|---|
| 2ⁿ | 2 | 4 | 8 | 16 | 32 | 64 | 128 | 256 |

**Usable hosts** = 2^(host bits) − 2. A /26 has 6 host bits → 64 addresses → 62 hosts.

## Mask values in the "interesting" octet

| Prefix in octet | /25 /17 /9 | /26 | /27 | /28 | /29 | /30 | /31 | /32 |
|---|---|---|---|---|---|---|---|---|
| Mask value | 128 | 192 | 224 | 240 | 248 | 252 | 254 | 255 |
| **Block size** (256 − value) | 128 | 64 | 32 | 16 | 8 | 4 | 2 | 1 |

## The block-size method

For `172.16.45.130/27`:

1. The interesting octet is the 4th (a /27 ends inside it). Mask value 224 → **block size 32**.
2. Multiples of 32: 0, 32, 64, 96, **128**, 160... The address (130) falls in the block starting at **128**.
3. Network: **172.16.45.128**. Next block starts at 160, so broadcast is **172.16.45.159**.
4. Usable: **.129 – .158** (30 hosts).

For `10.20.77.5/20`: the interesting octet is the 3rd; /20 → mask 255.255.**240**.0 → block 16 in the third octet. 77 falls in 64–79 → network **10.20.64.0**, broadcast **10.20.79.255**, usable 10.20.64.1–10.20.79.254.

## Choosing a prefix

"Smallest subnet for 50 hosts": need 2^h − 2 ≥ 50 → h = 6 → **/26** (62 hosts). Point-to-point links: **/30** (2 hosts) or **/31** (RFC 3021, both addresses usable on routed links). Loopbacks: **/32**.

## VLSM

Allocate the **largest subnets first** from the block so they stay aligned:

From 192.168.10.0/24: Sales 100 hosts → /25 (.0–.127); Engineering 50 → /26 (.128–.191); Lab 20 → /27 (.192–.223); two WAN links → /30s (.224–.227, .228–.231). Remaining: .232–.255 for growth.

## Wildcard masks

A **wildcard mask** is the inverse of a subnet mask: 0 bits mean "must match", 1 bits mean "don't care". For a contiguous mask, **wildcard = 255.255.255.255 − subnet mask**.

| Subnet mask | Wildcard |
|---|---|
| 255.255.255.0 (/24) | 0.0.0.255 |
| 255.255.255.192 (/26) | 0.0.0.63 |
| 255.255.255.252 (/30) | 0.0.0.3 |
| 255.255.240.0 (/20) | 0.0.15.255 |
| 255.255.255.255 (host) | 0.0.0.0 (keyword `host`) |
| 0.0.0.0 (any) | 255.255.255.255 (keyword `any`) |

Used in ACLs (`access-list 10 permit 192.168.10.0 0.0.0.255`) and OSPF network statements (`network 10.1.1.0 0.0.0.255 area 0`). Non-contiguous wildcards exist (e.g. 0.0.254.255 to match odd/even third octets) but are rare.

## Summarisation

Summarise contiguous networks into one route: 172.16.0.0/24 through 172.16.3.0/24 → **172.16.0.0/22**. Find the common leading bits; the block must be aligned (the first network divisible by the block size).

## Private and special ranges (recap)

10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16 (RFC 1918); 127.0.0.0/8 loopback; 169.254.0.0/16 APIPA; 100.64.0.0/10 CGNAT; 224.0.0.0/4 multicast.

@@quiz
What is the network address of 172.16.45.130/27?
- 172.16.45.0
- 172.16.45.96
* 172.16.45.128
- 172.16.45.160
? Block size 32; 130 falls in 128–159.

@@quiz
What is the smallest prefix that supports 50 hosts?
- /27
* /26
- /25
- /28
? /27 gives only 30 usable hosts.

@@quiz
What wildcard mask matches the subnet 10.10.8.0/21?
- 0.0.8.255
* 0.0.7.255
- 255.255.248.0
- 0.0.3.255
? 255.255.255.255 − 255.255.248.0.

@@quiz
Which single route summarises 192.168.4.0/24 through 192.168.7.0/24?
* 192.168.4.0/22
- 192.168.0.0/21
- 192.168.4.0/23
- 192.168.4.0/24
? Four /24s aligned on a multiple of 4.

@@quiz
What is the broadcast address of 10.20.77.5/20?
- 10.20.77.255
* 10.20.79.255
- 10.20.64.255
- 10.20.127.255
? Block 16 in the third octet: 64–79.

@@flashcards
/25 /26 /27 /28 :: 128 / 64 / 32 / 16 addresses
/29 /30 :: 8 / 4 addresses
usable hosts :: 2^h − 2
block size :: 256 − mask value
mask 224 :: /27, block 32
mask 240 :: /28, block 16
mask 248 :: /29, block 8
mask 252 :: /30, block 4
/31 :: point-to-point, 2 usable (RFC 3021)
wildcard :: 255.255.255.255 − mask
wildcard 0.0.0.63 :: /26
host keyword :: wildcard 0.0.0.0
any keyword :: 255.255.255.255 wildcard
VLSM rule :: allocate largest subnets first
summary of 4 aligned /24s :: /22

@@drill
How many usable hosts does a /28 provide?
= 14
? 16 − 2.

@@drill
What is the wildcard mask for 255.255.255.224?
= 0.0.0.31
? 255 − 224 = 31.

@@exercise Subnet calculator without libraries
Implement the block-size method by hand (no `ipaddress` module): `subnet(addr, prefix)` returns network, broadcast, first and last usable host and the number of usable hosts. Convert dotted addresses to a 32-bit integer and back. Print the results for each test address. For /31 and /32, report usable hosts as 2 and 1 and use the network and broadcast as first and last.
@@lang python
@@starter
tests = [("172.16.45.130", 27), ("10.20.77.5", 20), ("192.168.1.200", 26), ("10.0.0.6", 30), ("10.0.0.0", 31), ("8.8.8.8", 32)]

def to_int(addr):
    raise NotImplementedError

def subnet(addr, prefix):
    raise NotImplementedError
@@expected
172.16.45.130/27: net 172.16.45.128 bc 172.16.45.159 hosts 172.16.45.129-172.16.45.158 (30)
10.20.77.5/20: net 10.20.64.0 bc 10.20.79.255 hosts 10.20.64.1-10.20.79.254 (4094)
192.168.1.200/26: net 192.168.1.192 bc 192.168.1.255 hosts 192.168.1.193-192.168.1.254 (62)
10.0.0.6/30: net 10.0.0.4 bc 10.0.0.7 hosts 10.0.0.5-10.0.0.6 (2)
10.0.0.0/31: net 10.0.0.0 bc 10.0.0.1 hosts 10.0.0.0-10.0.0.1 (2)
8.8.8.8/32: net 8.8.8.8 bc 8.8.8.8 hosts 8.8.8.8-8.8.8.8 (1)
@@requires
def to_int
<< 8
def subnet
@@hint
`to_int`: fold the octets with `value = (value << 8) | int(octet)`. The mask is `(0xFFFFFFFF << (32 - prefix)) & 0xFFFFFFFF`; network = `ip & mask`; broadcast = `network | (~mask & 0xFFFFFFFF)`.
@@solution
tests = [("172.16.45.130", 27), ("10.20.77.5", 20), ("192.168.1.200", 26), ("10.0.0.6", 30), ("10.0.0.0", 31), ("8.8.8.8", 32)]

def to_int(addr):
    value = 0
    for octet in addr.split("."):
        value = (value << 8) | int(octet)
    return value

def to_str(value):
    return ".".join(str((value >> shift) & 255) for shift in (24, 16, 8, 0))

def subnet(addr, prefix):
    ip = to_int(addr)
    mask = (0xFFFFFFFF << (32 - prefix)) & 0xFFFFFFFF
    network = ip & mask
    broadcast = network | (~mask & 0xFFFFFFFF)
    if prefix >= 31:
        return network, broadcast, network, broadcast, 2 if prefix == 31 else 1
    return network, broadcast, network + 1, broadcast - 1, broadcast - network - 1

for addr, prefix in tests:
    net, bc, first, last, hosts = subnet(addr, prefix)
    print(f"{addr}/{prefix}: net {to_str(net)} bc {to_str(bc)} hosts {to_str(first)}-{to_str(last)} ({hosts})")

@@exercise VLSM planner with wildcard masks
Allocate subnets for the requirements from `10.50.0.0/22`, largest first (ties in input order), each with the smallest prefix that fits its hosts. Print each allocation with its network/prefix, subnet mask and wildcard mask, then the first unused address. Use the `ipaddress` module this time.
@@lang python
@@starter
import ipaddress

block = ipaddress.ip_network("10.50.0.0/22")
needs = [("Users", 300), ("Voice", 120), ("Servers", 40), ("Printers", 12), ("WAN-A", 2), ("WAN-B", 2), ("Mgmt", 25)]
@@expected
Users     10.50.0.0/23      mask 255.255.254.0  wildcard 0.0.1.255
Voice     10.50.2.0/25      mask 255.255.255.128  wildcard 0.0.0.127
Servers   10.50.2.128/26    mask 255.255.255.192  wildcard 0.0.0.63
Mgmt      10.50.2.192/27    mask 255.255.255.224  wildcard 0.0.0.31
Printers  10.50.2.224/28    mask 255.255.255.240  wildcard 0.0.0.15
WAN-A     10.50.2.240/30    mask 255.255.255.252  wildcard 0.0.0.3
WAN-B     10.50.2.244/30    mask 255.255.255.252  wildcard 0.0.0.3
Next free: 10.50.2.248
@@requires
hostmask
sorted(
@@hint
Prefix for h hosts: the smallest p with `2 ** (32 - p) - 2 >= h`. Walk a `next_addr` pointer; each subnet is `ipaddress.ip_network(f"{next_addr}/{p}")`. `net.netmask` and `net.hostmask` give the mask and wildcard.
@@solution
import ipaddress

block = ipaddress.ip_network("10.50.0.0/22")
needs = [("Users", 300), ("Voice", 120), ("Servers", 40), ("Printers", 12), ("WAN-A", 2), ("WAN-B", 2), ("Mgmt", 25)]

def prefix_for(hosts):
    p = 30
    while 2 ** (32 - p) - 2 < hosts:
        p -= 1
    return p

next_addr = block.network_address
for name, hosts in sorted(needs, key=lambda n: -n[1]):
    net = ipaddress.ip_network(f"{next_addr}/{prefix_for(hosts)}")
    print(f"{name:<9} {str(net):<17} mask {net.netmask}  wildcard {net.hostmask}")
    next_addr = net.broadcast_address + 1
print("Next free:", next_addr)

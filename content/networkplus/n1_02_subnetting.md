@@lesson n1-subnetting Subnetting and IPv6
@@text
## CIDR notation

`/24` means the first 24 bits are the **network** and the last 8 are **hosts**. The mask is those bits written as numbers: `/24` = `255.255.255.0`.

| Prefix | Mask (last octet) | Block size | Usable hosts |
|---|---|---|---|
| /24 | 0 | 256 | 254 |
| /25 | 128 | 128 | 126 |
| /26 | 192 | 64 | 62 |
| /27 | 224 | 32 | 30 |
| /28 | 240 | 16 | 14 |
| /29 | 248 | 8 | 6 |
| /30 | 252 | 4 | 2 |
| /31 | 254 | 2 | 2 (point-to-point links only) |
| /32 | 255 | 1 | a single host |

**Usable hosts = 2^(host bits) − 2**: the first address is the **network** address, the last is the **broadcast** address.

## The block-size method

Find the subnet of `192.168.10.77/26`:

1. Block size = 256 − 192 = **64**. Subnets start at 0, 64, 128, 192.
2. 77 falls in the block starting at **64**.
3. Network `192.168.10.64`, broadcast `192.168.10.127` (next block − 1), usable `.65`–`.126`, 62 hosts.

For prefixes shorter than /24, the same idea works on the third octet: `10.4.37.9/20` → mask `255.255.240.0`, block 16 in the third octet, so the network is `10.4.32.0` and the broadcast `10.4.47.255`.

## VLSM

**Variable-length subnet masks** give each network the size it needs. Plan the **largest first**. For `192.168.50.0/24` with Sales (100 hosts), IT (50), Printers (20), and two router links:

| Need | Prefix | Subnet |
|---|---|---|
| Sales 100 | /25 (126) | 192.168.50.0/25 |
| IT 50 | /26 (62) | 192.168.50.128/26 |
| Printers 20 | /27 (30) | 192.168.50.192/27 |
| Link A | /30 | 192.168.50.224/30 |
| Link B | /30 | 192.168.50.228/30 |

## Special addresses

- Private: `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`. **CGNAT**: `100.64.0.0/10` (ISPs).
- Loopback `127.0.0.1`; APIPA `169.254.0.0/16`; multicast `224.0.0.0/4`.
- Classes (legacy, but still asked): A `1–126` (/8), B `128–191` (/16), C `192–223` (/24), D multicast, E experimental.

## IPv6

128 bits written as 8 groups of hex: `2001:0db8:0000:0000:0000:ff00:0042:8329`. Shorten it: drop leading zeros in each group, and replace **one** run of all-zero groups with `::` → `2001:db8::ff00:42:8329`.

| Type | Prefix | Like IPv4's |
|---|---|---|
| Global unicast | `2000::/3` | public addresses |
| Unique local | `fc00::/7` (in practice `fd00::/8`) | private addresses |
| Link-local | `fe80::/10` | APIPA, but always present and used by routing protocols |
| Multicast | `ff00::/8` | (IPv6 has **no broadcast**) |
| Loopback | `::1` | 127.0.0.1 |

Subnets are normally **/64**. Hosts configure themselves with **SLAAC** (from router advertisements) or **DHCPv6**. **NDP** (Neighbour Discovery) replaces ARP. Transition: **dual stack** (both at once), **tunnelling** (6to4, Teredo), **NAT64**.

@@quiz
How many usable host addresses does a /27 subnet have?
- 32
* 30
- 62
- 14
? 2^5 − 2 = 30.

@@quiz
What is the network address of 172.16.5.200/26?
* 172.16.5.192
- 172.16.5.128
- 172.16.5.0
- 172.16.5.200
? Block size 64: 0, 64, 128, 192. 200 is in the block starting at 192.

@@quiz
What is the broadcast address of 10.1.1.20/28?
* 10.1.1.31
- 10.1.1.15
- 10.1.1.32
- 10.1.1.255
? Block size 16: the subnet is .16–.31.

@@quiz
Which prefix gives the smallest subnet that holds 50 hosts?
* /26
- /25
- /27
- /24
? /27 only has 30 usable; /26 has 62.

@@quiz
Which is a correct short form of 2001:0db8:0000:0000:0000:0000:0000:0001?
* 2001:db8::1
- 2001:db8:::1
- 2001:db8::0::1
- 21:db8::1
? `::` may appear only once; leading zeros in a group may be dropped.

@@quiz
Every IPv6 interface has an address starting with fe80. What kind is it?
* Link-local
- Global unicast
- Multicast
- Unique local
? Link-local addresses are only valid on the local link and are always configured.

@@quiz
A router link between two sites needs exactly two addresses. Which prefix wastes the fewest addresses (traditional answer)?
* /30
- /29
- /24
- /28
? /30 gives 2 usable. (Modern routers also support /31 for point-to-point links.)

@@quiz
What is the subnet mask for /20?
* 255.255.240.0
- 255.255.255.240
- 255.255.248.0
- 255.240.0.0
? 20 bits: 8 + 8 + 4 (11110000 = 240).

@@flashcards
usable hosts :: 2^(host bits) − 2
/24 :: 255.255.255.0, 254 hosts
/25 :: .128, 126 hosts
/26 :: .192, 62 hosts
/27 :: .224, 30 hosts
/28 :: .240, 14 hosts
/29 :: .248, 6 hosts
/30 :: .252, 2 hosts
block size :: 256 − mask octet
VLSM :: Different subnet sizes, allocated largest first
CGNAT range :: 100.64.0.0/10
IPv6 link-local :: fe80::/10
IPv6 unique local :: fc00::/7 (fd00::/8)
IPv6 global unicast :: 2000::/3
SLAAC :: IPv6 hosts configure themselves from router advertisements
NDP :: IPv6 neighbour discovery, replaces ARP
dual stack :: Running IPv4 and IPv6 together

@@drill
What is the broadcast address of 192.168.1.130/25?
= 192.168.1.255
? /25 splits at 128: .128–.255.

@@drill
How many usable hosts are in a /29?
= 6
? 2^3 − 2.

@@exercise Block-size calculator
Write `subnet(ip, prefix)` for prefixes **/24 to /30**, returning `(network, broadcast, hosts)` as strings and an int, using the block-size method on the last octet (no `ipaddress` module). Print the result for the three addresses given.
@@lang python
@@starter
def subnet(ip, prefix):
    a, b, c, d = (int(x) for x in ip.split("."))
    block = 2 ** (32 - prefix)
    # network: the multiple of block at or below d; broadcast: network + block - 1
    raise NotImplementedError

for ip, prefix in [("192.168.10.77", 26), ("10.1.1.20", 28), ("172.16.5.200", 30)]:
    net, bc, hosts = subnet(ip, prefix)
    print(f"{ip}/{prefix}: network {net}, broadcast {bc}, {hosts} hosts")
@@expected
192.168.10.77/26: network 192.168.10.64, broadcast 192.168.10.127, 62 hosts
10.1.1.20/28: network 10.1.1.16, broadcast 10.1.1.31, 14 hosts
172.16.5.200/30: network 172.16.5.200, broadcast 172.16.5.203, 2 hosts
@@requires
block
@@hint
`net = d // block * block`, `bc = net + block - 1`, `hosts = block - 2`, then build the strings with f-strings: `f"{a}.{b}.{c}.{net}"`.
@@solution
def subnet(ip, prefix):
    a, b, c, d = (int(x) for x in ip.split("."))
    block = 2 ** (32 - prefix)
    net = d // block * block
    bc = net + block - 1
    return f"{a}.{b}.{c}.{net}", f"{a}.{b}.{c}.{bc}", block - 2

for ip, prefix in [("192.168.10.77", 26), ("10.1.1.20", 28), ("172.16.5.200", 30)]:
    net, bc, hosts = subnet(ip, prefix)
    print(f"{ip}/{prefix}: network {net}, broadcast {bc}, {hosts} hosts")

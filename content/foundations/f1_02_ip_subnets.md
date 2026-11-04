@@lesson f1-subnets IP Addresses, Subnets and IPv6
@@text
The networks lesson introduced IP addresses. Every IT certification (A+, Network+, CCNA, the cloud exams) and every network job expects you to go one step further: read a subnet mask, tell whether two addresses are on the same network, and know which addresses are private. It looks like maths; it's really just binary from module 0, applied.

## Anatomy of an IPv4 address

An IPv4 address is 32 bits, written as four bytes in decimal (**dotted decimal**): `192.168.10.25`. Part of it identifies the **network**, the rest the **host** (the device) on that network. The **subnet mask** says where the split is:

```text
Address  192.168.10.25    11000000.10101000.00001010.00011001
Mask     255.255.255.0    11111111.11111111.11111111.00000000
                          |------- network (24 bits) -------|-host-|
Network  192.168.10.0
```

The mask's 1-bits mark the network part. Writing the number of 1-bits after a slash is **CIDR notation**: `192.168.10.25/24`.

## Same network or not?

Two devices can talk directly only if they're on the same network (same network part). Otherwise traffic goes to the **default gateway** (the router). To find the network address, **AND** the address with the mask (keep a bit only where the mask has a 1). For a `/24`, that's simply "the first three numbers match".

| A | B | Mask | Same network? |
|---|---|---|---|
| 192.168.10.25 | 192.168.10.200 | /24 | yes |
| 192.168.10.25 | 192.168.11.5 | /24 | no: needs the router |
| 10.0.5.20 | 10.0.200.1 | /16 | yes (first two numbers match) |

A wrong mask or gateway on one PC is a classic ticket: "I can reach the printer next to me but not the internet" (wrong gateway) or "I can reach some colleagues but not others" (wrong mask).

## Network, broadcast and usable hosts

In every subnet, two addresses are reserved:

- the **network address** (all host bits 0): `192.168.10.0`
- the **broadcast address** (all host bits 1): `192.168.10.255`, which reaches every device on the subnet

So a /24 has 2⁸ = 256 addresses and **254 usable** hosts. In general, with *h* host bits: 2ʰ − 2 usable hosts.

| CIDR | Mask | Addresses | Usable hosts | Typical use |
|---|---|---|---|---|
| /30 | 255.255.255.252 | 4 | 2 | a link between two routers |
| /28 | 255.255.255.240 | 16 | 14 | a small server segment |
| /26 | 255.255.255.192 | 64 | 62 | a floor of an office |
| /24 | 255.255.255.0 | 256 | 254 | a typical LAN or home network |
| /16 | 255.255.0.0 | 65,536 | 65,534 | a large site or a cloud VPC |

Splitting a network into smaller ones is **subnetting**: a company might split `10.20.0.0/16` into `/24`s for each floor, Wi-Fi, servers and printers, so problems and broadcasts stay contained and firewall rules can be written per subnet.

## Private, public and special addresses

There aren't enough IPv4 addresses for every device, so three ranges (RFC 1918) are **private**: usable inside any organisation, never routed on the internet:

- `10.0.0.0/8` (10.x.x.x)
- `172.16.0.0/12` (172.16.x.x – 172.31.x.x)
- `192.168.0.0/16` (192.168.x.x)

**NAT** (Network Address Translation) on the router lets many private devices share one public address.

Other special addresses:

| Address | Meaning |
|---|---|
| `127.0.0.1` | **loopback**: "this computer" (`localhost`) |
| `169.254.x.x` | **APIPA**: Windows gave itself an address because **DHCP failed** |
| `0.0.0.0` | "any address" / no address yet |
| `100.64.0.0/10` | carrier-grade NAT (ISPs) |

Seeing `169.254.x.x` in `ipconfig` is one of the most useful clues in IT: the device didn't get an address from DHCP (cable, Wi-Fi, VLAN or DHCP server problem).

## IPv6

IPv6 addresses are 128 bits, written as eight groups of four hex digits: `2001:0db8:0000:0000:0000:ff00:0042:8329`. Shortening rules: drop leading zeros in a group, and replace **one** run of all-zero groups with `::` → `2001:db8::ff00:42:8329`.

- There are enough addresses for every device to have a public one; NAT isn't needed (firewalls still are).
- The usual subnet size is `/64`.
- `::1` is loopback; addresses starting `fe80::` are **link-local** (every interface has one automatically); `2000::/3` are global addresses; `fd00::/8` are private ("unique local").
- Devices often configure themselves (**SLAAC**) from router advertisements, with or without DHCPv6.
- Most networks today are **dual-stack**: IPv4 and IPv6 side by side. Japanese ISPs widely use IPv6 (IPoE) with IPv4 carried over it.

@@quiz
A PC shows IP address 169.254.33.7. What does that tell you?
* It didn't get an address from DHCP and assigned itself an APIPA address
- It has a public internet address
- It's the default gateway
- It's using IPv6
? Check the cable/Wi-Fi, the VLAN and the DHCP server.

@@quiz
How many usable host addresses does a /26 subnet have?
- 64
* 62
- 32
- 254
? 32 − 26 = 6 host bits: 2⁶ − 2 = 62.

@@quiz
Which of these is a private (RFC 1918) address?
- 8.8.8.8
* 172.20.1.5
- 172.40.1.5
- 100.64.0.1
? 172.16.0.0/12 covers 172.16–172.31 only.

@@quiz
With mask 255.255.255.0, are 192.168.1.10 and 192.168.2.10 on the same network?
- Yes
* No: the third number differs, so traffic goes via the router
- Only with IPv6
- Only if they use the same switch
? A /24 network part is the first three numbers.

@@quiz
How can `2001:0db8:0000:0000:0000:0000:0000:0001` be shortened?
* `2001:db8::1`
- `2001:db8:1`
- `2001::db8::1`
- `21:db8::1`
? Drop leading zeros; `::` replaces one run of zero groups.

@@flashcards
subnet mask :: marks the network part of an address
CIDR /24 :: 24 network bits = 255.255.255.0
network address :: host bits all 0
broadcast address :: host bits all 1
usable hosts :: 2^(host bits) − 2
default gateway :: router for traffic to other networks
RFC 1918 :: 10/8, 172.16/12, 192.168/16
NAT :: many private addresses share a public one
127.0.0.1 :: loopback (localhost)
169.254.x.x :: APIPA: DHCP failed
IPv6 :: 128 bits, hex, eight groups
`::` :: one run of zero groups
fe80:: :: IPv6 link-local
/64 :: standard IPv6 subnet

@@drill
What is the subnet mask for /24 in dotted decimal?
= 255.255.255.0
? Three full bytes of 1-bits.

@@drill
How many usable hosts in a /30?
= 2
? 4 addresses minus network and broadcast: used for router-to-router links.

@@drill
What is the dotted-decimal mask for /26?
= 255.255.255.192
? 128 + 64 = 192 in the last byte.

@@exercise A subnet calculator
Write a subnet calculator **without** the `ipaddress` module: `to_int("192.168.10.25")` turns an address into a 32-bit number, `to_ip(n)` turns it back, and `subnet(cidr)` takes `"192.168.10.25/24"` and returns the network address, broadcast address, mask and number of usable hosts. Build the mask with `(0xFFFFFFFF << (32 - prefix)) & 0xFFFFFFFF`, the network with `&`, and the broadcast with `| ~mask`. Print the results for the three samples.
@@lang python
@@starter
def to_int(ip):
    raise NotImplementedError

def to_ip(n):
    raise NotImplementedError

def subnet(cidr):
    raise NotImplementedError

for cidr in ["192.168.10.25/24", "10.20.33.130/26", "172.16.5.9/30"]:
    net, bcast, mask, hosts = subnet(cidr)
    print(f"{cidr}: network {net}, broadcast {bcast}, mask {mask}, {hosts} hosts")
@@expected
192.168.10.25/24: network 192.168.10.0, broadcast 192.168.10.255, mask 255.255.255.0, 254 hosts
10.20.33.130/26: network 10.20.33.128, broadcast 10.20.33.191, mask 255.255.255.192, 62 hosts
172.16.5.9/30: network 172.16.5.8, broadcast 172.16.5.11, mask 255.255.255.252, 2 hosts
@@requires
<< (32 - prefix)
& mask
def to_int
@@hint
`to_int`: start at 0 and for each of the four parts do `n = n * 256 + int(part)`. `to_ip`: `".".join(str((n >> shift) & 255) for shift in (24, 16, 8, 0))`. Usable hosts: `2 ** (32 - prefix) - 2`.
@@solution
def to_int(ip):
    n = 0
    for part in ip.split("."):
        n = n * 256 + int(part)
    return n

def to_ip(n):
    return ".".join(str((n >> shift) & 255) for shift in (24, 16, 8, 0))

def subnet(cidr):
    ip, prefix = cidr.split("/")
    prefix = int(prefix)
    mask = (0xFFFFFFFF << (32 - prefix)) & 0xFFFFFFFF
    network = to_int(ip) & mask
    broadcast = network | (~mask & 0xFFFFFFFF)
    return to_ip(network), to_ip(broadcast), to_ip(mask), 2 ** (32 - prefix) - 2

for cidr in ["192.168.10.25/24", "10.20.33.130/26", "172.16.5.9/30"]:
    net, bcast, mask, hosts = subnet(cidr)
    print(f"{cidr}: network {net}, broadcast {bcast}, mask {mask}, {hosts} hosts")

@@exercise Classify addresses from ipconfig output
Use Python's `ipaddress` module to classify each address in the list as `loopback`, `APIPA (DHCP failed)` (169.254.0.0/16), `private`, or `public`, checking in that order (`ip.is_loopback`, membership in `ip_network("169.254.0.0/16")`, `ip.is_private`). Then check whether each of the colleagues' addresses is on the same network as `192.168.10.25/24` using `ip_interface(...).network`.
@@lang python
@@starter
import ipaddress

addresses = ["127.0.0.1", "169.254.33.7", "10.0.5.20", "172.20.1.5", "8.8.8.8", "192.168.10.200"]
me = ipaddress.ip_interface("192.168.10.25/24")
colleagues = ["192.168.10.40", "192.168.11.40", "192.168.10.254"]
@@expected
127.0.0.1: loopback
169.254.33.7: APIPA (DHCP failed)
10.0.5.20: private
172.20.1.5: private
8.8.8.8: public
192.168.10.200: private
My network: 192.168.10.0/24
192.168.10.40: same network
192.168.11.40: via the router
192.168.10.254: same network
@@requires
import ipaddress
is_loopback
is_private
.network
@@hint
`ip = ipaddress.ip_address(a)`. Same network: `ipaddress.ip_address(c) in me.network`.
@@solution
import ipaddress

addresses = ["127.0.0.1", "169.254.33.7", "10.0.5.20", "172.20.1.5", "8.8.8.8", "192.168.10.200"]
me = ipaddress.ip_interface("192.168.10.25/24")
colleagues = ["192.168.10.40", "192.168.11.40", "192.168.10.254"]
APIPA = ipaddress.ip_network("169.254.0.0/16")

def classify(a):
    ip = ipaddress.ip_address(a)
    if ip.is_loopback:
        return "loopback"
    if ip in APIPA:
        return "APIPA (DHCP failed)"
    if ip.is_private:
        return "private"
    return "public"

for a in addresses:
    print(f"{a}: {classify(a)}")

print(f"My network: {me.network}")
for c in colleagues:
    same = ipaddress.ip_address(c) in me.network
    print(f"{c}: {'same network' if same else 'via the router'}")

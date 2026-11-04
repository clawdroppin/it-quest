@@lesson n4-services Troubleshooting Addressing, Routing, Services and Performance
@@text
Once the link is up, problems move up the stack: wrong IP settings, routing mistakes, DNS and DHCP failures, blocked ports, and performance issues like congestion and wireless interference. Network+ scenario questions describe the symptom; you name the cause or the next command.

## Addressing problems

| Symptom | Cause |
|---|---|
| **169.254.x.x** address | no DHCP answer: server down, scope exhausted, relay missing, VLAN wrong, port in err-disabled |
| Works locally, not off-subnet | **wrong or missing default gateway**, wrong subnet mask |
| Intermittent connectivity, "address conflict" warning | **duplicate IP** (a static address inside the DHCP scope) |
| Can reach some hosts in the subnet but not others | **wrong subnet mask** |
| Duplicate MAC | cloned VMs, a misconfigured virtual NIC |
| IP works, names don't | **DNS**: wrong server, server down, missing record, stale cache |
| Correct IP but "can't reach anything" | wrong VLAN on the access port, NAC/802.1X quarantine |
| IPv6 works, IPv4 doesn't (or the reverse) | separate configs; check both |

## Routing and switching problems

- **Routing loop**: packets bounce between routers until the TTL expires; traceroute shows the same hops repeating.
- **Missing route / wrong route**: traceroute stops at a hop; check routing tables both ways.
- **Asymmetric routing**: traffic goes out one path and returns another; stateful firewalls drop the replies.
- **Switching loop / broadcast storm**: STP disabled or a misconfiguration; all links saturated, MAC addresses flapping between ports.
- **Incorrect VLAN** or **missing VLAN on the trunk** (allowed list).
- **ACL / firewall blocking** a needed port: the app times out (drop) or is refused (reject/RST). Check the logs and hit counts.
- **NAT misconfiguration**: outbound works, inbound port forward doesn't, or the reverse.
- **Default route missing**: internal works, internet doesn't.

## Service problems

- **DHCP scope exhaustion**, rogue DHCP, relay misconfigured.
- **DNS**: wrong records, expired domain, TTL caching after a change, split DNS mismatch.
- **NTP** off → Kerberos/certificate errors.
- **Certificate** issues: expired, wrong name, untrusted CA.
- **Server/service down**: test the port directly (`Test-NetConnection host -Port 443`, `nc -zv host 443`).

## Performance problems

| Problem | Typical causes |
|---|---|
| **Congestion / bottleneck** | link saturated, oversubscribed uplinks, backups during business hours |
| **High latency** | long distance, congestion, queuing, satellite links (~600 ms) |
| **Jitter** | congestion, no QoS for voice |
| **Packet loss** | errors, congestion drops, bad Wi-Fi |
| **Bandwidth throttling** by the ISP | contract limits |
| **Wireless**: interference (microwaves, Bluetooth, neighbours), **channel overlap**, low **RSSI**, insufficient **coverage**, **client disassociation**, **roaming** misconfiguration, too many clients per AP, legacy clients slowing the cell, wrong **antenna** placement/polarisation | survey, channel plan, more APs, band steering |

## Commands and what they tell you

| Command | Use |
|---|---|
| `ipconfig /all`, `ip addr`, `ifconfig` | IP, mask, gateway, DNS, DHCP server, MAC |
| `ping` | reachability, latency, loss (ICMP may be blocked) |
| `tracert` / `traceroute` / `pathping` / `mtr` | the path and where it stops or slows |
| `nslookup`, `dig` | DNS answers from a specific server |
| `arp -a` | IP-to-MAC cache (duplicate IPs, spoofing) |
| `netstat -an`, `ss -tulpn` | listening ports and connections |
| `route print`, `ip route`, `show ip route` | routing table |
| `tcpdump`, Wireshark | packets |
| `nmap` | open ports, hosts, service versions |
| `show interface`, `show vlan`, `show mac address-table`, `show arp`, `show running-config` | switch/router state |
| `Test-NetConnection`, `curl -v` | test a specific port/app |
| `iperf` | measure throughput between two hosts |

Approach: **ping the loopback (127.0.0.1)** → own IP → **gateway** → a remote IP → a remote name. The first failure tells you the layer.

@@quiz
A PC has the address 169.254.10.7. What does this indicate?
* It didn't get an answer from a DHCP server
- It has a static IP
- DNS is misconfigured
- The gateway is down but DHCP worked
? APIPA self-assigned address.

@@quiz
A user can reach servers in their own subnet but nothing beyond it. What's the most likely misconfiguration?
- DNS server
* Default gateway
- MAC address
- DHCP lease time
? Off-subnet traffic goes to the gateway.

@@quiz
Traceroute shows hops 4 and 5 repeating until the TTL expires. What is it?
- Asymmetric routing
* A routing loop
- A duplex mismatch
- DNS failure
? Packets bounce between two routers.

@@quiz
A firewall drops return traffic for sessions that left through a different firewall. What's the cause?
* Asymmetric routing
- A broadcast storm
- An expired certificate
- Jitter
? Stateful devices need to see both directions.

@@quiz
Which tool measures actual throughput between two hosts?
- nslookup
- arp
* iperf
- nmap
? It generates traffic and reports Mb/s.

@@flashcards
169.254.x.x :: APIPA, no DHCP answer
local works, remote doesn't :: check the default gateway
some local hosts unreachable :: wrong subnet mask
address conflict :: duplicate IP
IP works, names don't :: DNS
routing loop :: repeating hops, TTL expiry
asymmetric routing :: different paths out and back
broadcast storm :: switching loop, STP issue
timeout vs refused :: dropped vs rejected by firewall/host
pathping / mtr :: traceroute + per-hop loss
arp -a :: IP-to-MAC table
ss -tulpn :: Linux listening ports
Test-NetConnection -Port :: test one TCP port
iperf :: throughput testing
ping order :: loopback, own IP, gateway, remote IP, remote name

@@drill
Which address do you ping to test the local TCP/IP stack on IPv4?
= 127.0.0.1
? The loopback address.

@@drill
Which command shows a Windows PC's DHCP server, DNS servers and MAC address?
= ipconfig /all
? `ipconfig` alone shows only the basics.

@@exercise Diagnose a host's IP configuration
Check each host's settings with the `ipaddress` module: the address must be in the expected subnet, the gateway must be inside the host's own network, the mask must match the subnet's prefix, the address mustn't be APIPA, and it mustn't be used by another host. Print each host's problems or `OK`.
@@lang python
@@starter
import ipaddress

SUBNET = ipaddress.ip_network("10.20.30.0/24")
GATEWAY = "10.20.30.1"
hosts = [
    ("pc-ana", "10.20.30.41/24", "10.20.30.1"),
    ("pc-ken", "10.20.30.42/24", "10.20.31.1"),
    ("pc-mei", "10.20.30.43/25", "10.20.30.1"),
    ("pc-omar", "169.254.7.9/16", ""),
    ("printer", "10.20.30.41/24", "10.20.30.1"),
    ("pc-sato", "10.20.40.12/24", "10.20.40.1"),
]
@@expected
pc-ana 10.20.30.41/24: duplicate IP
pc-ken 10.20.30.42/24: gateway 10.20.31.1 outside its network
pc-mei 10.20.30.43/25: wrong mask /25
pc-omar 169.254.7.9/16: APIPA: no DHCP answer
printer 10.20.30.41/24: duplicate IP
pc-sato 10.20.40.12/24: wrong subnet (wrong VLAN?)
@@requires
ip_interface(
Counter(
@@hint
`iface = ipaddress.ip_interface(addr)` gives `iface.ip` and `iface.network`. Count addresses with `Counter(ipaddress.ip_interface(a).ip for _, a, _ in hosts)` to spot duplicates.
@@solution
import ipaddress
from collections import Counter

SUBNET = ipaddress.ip_network("10.20.30.0/24")
GATEWAY = "10.20.30.1"
hosts = [
    ("pc-ana", "10.20.30.41/24", "10.20.30.1"),
    ("pc-ken", "10.20.30.42/24", "10.20.31.1"),
    ("pc-mei", "10.20.30.43/25", "10.20.30.1"),
    ("pc-omar", "169.254.7.9/16", ""),
    ("printer", "10.20.30.41/24", "10.20.30.1"),
    ("pc-sato", "10.20.40.12/24", "10.20.40.1"),
]

used = Counter(ipaddress.ip_interface(a).ip for _, a, _ in hosts)
for name, addr, gw in hosts:
    iface = ipaddress.ip_interface(addr)
    problems = []
    if iface.ip.is_link_local:
        problems.append("APIPA: no DHCP answer")
    elif iface.ip not in SUBNET:
        problems.append("wrong subnet (wrong VLAN?)")
    else:
        if iface.network.prefixlen != SUBNET.prefixlen:
            problems.append(f"wrong mask /{iface.network.prefixlen}")
        if not gw or ipaddress.ip_address(gw) not in iface.network:
            problems.append(f"gateway {gw or 'missing'} outside its network")
        elif gw != GATEWAY:
            problems.append(f"unexpected gateway {gw}")
    if used[iface.ip] > 1:
        problems.append("duplicate IP")
    print(f"{name} {addr}: " + ("; ".join(problems) if problems else "OK"))

@@exercise Read a traceroute
Analyse traceroute hops given as `(hop, address, [rtt_ms, rtt_ms, rtt_ms])` where `None` is a timeout. Report: the first hop where latency jumps by more than 50 ms over the previous responding hop; any address that appears at two different hops (routing loop); and whether the trace reached the destination. Hops with all timeouts are noted as "filtered or no reply" but don't break the analysis.
@@lang python
@@starter
DEST = "203.0.113.80"
traces = {
    "to-tokyo": [(1, "10.0.0.1", [1, 1, 2]), (2, "100.64.0.1", [5, 6, 5]), (3, None, [None, None, None]),
                 (4, "198.51.100.9", [12, 13, 12]), (5, "198.51.100.77", [140, 142, 139]), (6, "203.0.113.80", [141, 140, 143])],
    "to-branch": [(1, "10.0.0.1", [1, 1, 1]), (2, "10.9.0.2", [3, 3, 4]), (3, "10.9.0.6", [4, 4, 5]),
                  (4, "10.9.0.2", [5, 5, 5]), (5, "10.9.0.6", [6, 6, 6]), (6, "10.9.0.2", [7, 7, 7])],
}
@@expected
== to-tokyo
hop 3: filtered or no reply
hop 5: latency jump 12.3 -> 140.3 ms at 198.51.100.77
reached destination
== to-branch
hop 4: 10.9.0.2 already seen at hop 2: routing loop
did not reach destination
@@requires
DEST
sum(
@@hint
Average the non-None RTTs with `sum(r) / len(r)`. Keep a dict of address → first hop to spot repeats.
@@solution
DEST = "203.0.113.80"
traces = {
    "to-tokyo": [(1, "10.0.0.1", [1, 1, 2]), (2, "100.64.0.1", [5, 6, 5]), (3, None, [None, None, None]),
                 (4, "198.51.100.9", [12, 13, 12]), (5, "198.51.100.77", [140, 142, 139]), (6, "203.0.113.80", [141, 140, 143])],
    "to-branch": [(1, "10.0.0.1", [1, 1, 1]), (2, "10.9.0.2", [3, 3, 4]), (3, "10.9.0.6", [4, 4, 5]),
                  (4, "10.9.0.2", [5, 5, 5]), (5, "10.9.0.6", [6, 6, 6]), (6, "10.9.0.2", [7, 7, 7])],
}

for name, hops in traces.items():
    print(f"== {name}")
    previous = None
    seen = {}
    loop_reported = False
    for hop, addr, rtts in hops:
        replies = [r for r in rtts if r is not None]
        if not replies:
            print(f"hop {hop}: filtered or no reply")
            continue
        avg = sum(replies) / len(replies)
        if previous is not None and avg - previous > 50:
            print(f"hop {hop}: latency jump {previous:.1f} -> {avg:.1f} ms at {addr}")
        previous = avg
        if addr in seen and not loop_reported:
            print(f"hop {hop}: {addr} already seen at hop {seen[addr]}: routing loop")
            loop_reported = True
        seen.setdefault(addr, hop)
    reached = hops[-1][1] == DEST
    print("reached destination" if reached else "did not reach destination")

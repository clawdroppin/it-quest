@@lesson c0-fundamentals Switching Fundamentals, Interfaces and Virtualisation
@@text
CCNA domain 1 (network fundamentals, 20%) covers how switches forward frames, how to read interface status and errors, cabling and interface issues, TCP vs UDP, and virtualisation concepts. This lesson focuses on what you'll see on real Cisco switches.

## How a switch forwards frames

1. **Learn**: record the frame's **source MAC** and the ingress port in the MAC address table (CAM).
2. **Forward/filter**: look up the **destination MAC**:
   - Known unicast on another port → forward out that port only.
   - Known unicast on the **same** port → filter (drop).
   - **Unknown unicast**, **broadcast** (`ffff.ffff.ffff`) or multicast (without snooping) → **flood** out all ports in the VLAN except the ingress.
3. **Age**: entries expire after **300 seconds** of inactivity by default.

```text
SW1# show mac address-table dynamic
          Mac Address Table
Vlan    Mac Address       Type        Ports
----    -----------       --------    -----
  10    0050.7966.6800    DYNAMIC     Gi1/0/1
  10    0050.7966.6801    DYNAMIC     Gi1/0/2
  20    0050.7966.6802    DYNAMIC     Gi1/0/3
   1    5254.0012.3456    DYNAMIC     Gi1/0/24
```

Switching methods: **store-and-forward** (checks the FCS; Cisco default), **cut-through** (starts forwarding after the destination MAC: lower latency, forwards bad frames), **fragment-free** (first 64 bytes).

**Collision domain**: each switch port is its own. **Broadcast domain**: each VLAN (routers separate them).

## Interface status

```text
SW1# show interfaces status
Port      Name       Status       Vlan       Duplex  Speed Type
Gi1/0/1   PC-Ana     connected    10         a-full a-1000 10/100/1000BaseTX
Gi1/0/2   Printer    notconnect   10           auto   auto 10/100/1000BaseTX
Gi1/0/3              err-disabled 20           auto   auto 10/100/1000BaseTX
Gi1/0/4   Old-PC     connected    10         a-half  a-100 10/100/1000BaseTX
Gi1/0/24  Uplink     connected    trunk        full   1000 10/100/1000BaseTX
```

`a-` means auto-negotiated. `show ip interface brief` gives **Status/Protocol**:

| Status / Protocol | Meaning |
|---|---|
| up / up | working |
| administratively down / down | `shutdown` configured |
| down / down | no physical link (cable, far end) |
| up / down | layer 1 OK, layer 2 problem (encapsulation, keepalives) |

## Interface and cable issues

`show interfaces Gi1/0/4` counters:

- **Input errors**, **CRC** (bad frames: cabling, EMI, duplex mismatch)
- **Runts** (< 64 bytes), **giants** (> 1518 bytes)
- **Collisions** and **late collisions** (duplex mismatch: one side half, one side full)
- **Output drops** (congestion)

A **duplex mismatch** happens when one side is hard-coded (`duplex full`) and the other auto-negotiates (falls back to half for 10/100). **Speed mismatch** → link down. Fix: configure both sides the same, preferably auto.

Cabling recap: copper Cat5e/6/6a (100 m), fibre MMF (short) / SMF (long), connectors and SFPs, straight-through vs crossover (auto-MDIX makes it moot), PoE.

## TCP vs UDP

| | TCP | UDP |
|---|---|---|
| Connection | three-way handshake (SYN, SYN-ACK, ACK) | none |
| Reliability | sequence numbers, acknowledgements, retransmission | none (the app handles it) |
| Flow control | **windowing** | none |
| Overhead | 20-byte header | 8-byte header |
| Uses | HTTP(S), SSH, SMTP, FTP, BGP | DNS queries, DHCP, SNMP, TFTP, syslog, VoIP/RTP, video |

## Virtualisation fundamentals

- **Server virtualisation**: a **hypervisor** runs several **VMs** (type 1 bare metal: ESXi, Hyper-V, KVM). Each VM has virtual NICs connected to a **virtual switch** in the hypervisor, which uplinks to physical switch trunks.
- **Containers**: share the host OS kernel; lighter than VMs.
- **VRF** (virtual routing and forwarding): several separate routing tables on one router, so overlapping customer networks can coexist.
- **Cloud**: IaaS/PaaS/SaaS, public/private/hybrid; how traffic reaches the cloud (internet VPN, private circuits).

## Other fundamentals in the blueprint

Network components (routers, L2/L3 switches, NGFW/IPS, APs, WLCs, endpoints, servers, PoE); topologies (two-tier collapsed core, three-tier, spine-leaf, WAN, SOHO, on-premises vs cloud); wireless principles (channels, SSID, RF, encryption) covered in the wireless lesson.

@@quiz
A switch receives a frame whose destination MAC isn't in its MAC table. What does it do?
- Drops the frame
* Floods it out all ports in the VLAN except the one it arrived on
- Sends it to the default gateway
- Sends an ARP request
? Unknown unicast flooding.

@@quiz
`show ip interface brief` shows an interface as "administratively down / down". What's the fix?
- Replace the cable
* `no shutdown` on the interface
- Change the duplex
- Clear the MAC table
? It was shut down in the configuration.

@@quiz
An interface shows late collisions and CRC errors, and status a-half/a-100 while the PC is set to 100/full. What is the problem?
- Speed mismatch
* Duplex mismatch
- Native VLAN mismatch
- An STP loop
? The auto side fell back to half duplex.

@@quiz
Which transport does a DNS query normally use, and why?
* UDP: small request/response, no connection setup needed
- TCP: DNS needs guaranteed delivery
- ICMP: it's a control message
- UDP: because DNS is always encrypted
? DNS uses TCP for zone transfers and large responses.

@@quiz
What is the default MAC address table aging time on Cisco switches?
- 30 seconds
- 120 seconds
* 300 seconds
- 3600 seconds
? Inactive entries are removed after 5 minutes.

@@flashcards
learn :: record source MAC and port
flood :: unknown unicast, broadcast, multicast
filter :: destination on the same port
MAC aging :: 300 seconds
store-and-forward :: checks FCS (Cisco default)
cut-through :: forwards after destination MAC
broadcast domain :: one VLAN
collision domain :: one switch port
up/up :: working
administratively down :: shutdown configured
down/down :: no physical link
up/down :: layer 2 problem
runts / giants :: < 64 / > 1518 bytes
late collisions :: duplex mismatch
TCP handshake :: SYN, SYN-ACK, ACK
windowing :: TCP flow control
UDP uses :: DNS, DHCP, SNMP, TFTP, VoIP
VRF :: separate routing tables on one router

@@drill
What is the Ethernet broadcast MAC address in Cisco notation?
= ffff.ffff.ffff
= FFFF.FFFF.FFFF
? All 48 bits set.

@@drill
How many bytes is a UDP header?
= 8
? TCP's is at least 20.

@@exercise Simulate MAC learning and forwarding
Simulate a switch with one VLAN. For each frame `(port, src, dst)`, learn the source, then decide: `flood` (broadcast or unknown destination; list the other ports), `forward to PORT`, or `filter` (destination on the ingress port). Ports are `Gi1` to `Gi4`. Print each decision and the final MAC table sorted by port, then MAC.
@@lang python
@@starter
PORTS = ["Gi1", "Gi2", "Gi3", "Gi4"]
BROADCAST = "ffff.ffff.ffff"
frames = [
    ("Gi1", "aaaa.0000.0001", BROADCAST),
    ("Gi2", "aaaa.0000.0002", "aaaa.0000.0001"),
    ("Gi1", "aaaa.0000.0001", "aaaa.0000.0002"),
    ("Gi3", "aaaa.0000.0003", "aaaa.0000.0009"),
    ("Gi3", "aaaa.0000.0004", "aaaa.0000.0003"),
    ("Gi4", "aaaa.0000.0002", "aaaa.0000.0003"),
]
@@expected
Gi1 aaaa.0000.0001 -> ffff.ffff.ffff: flood to Gi2, Gi3, Gi4
Gi2 aaaa.0000.0002 -> aaaa.0000.0001: forward to Gi1
Gi1 aaaa.0000.0001 -> aaaa.0000.0002: forward to Gi2
Gi3 aaaa.0000.0003 -> aaaa.0000.0009: flood to Gi1, Gi2, Gi4
Gi3 aaaa.0000.0004 -> aaaa.0000.0003: filter
Gi4 aaaa.0000.0002 -> aaaa.0000.0003: forward to Gi3
MAC table:
  Gi1 aaaa.0000.0001
  Gi3 aaaa.0000.0003
  Gi3 aaaa.0000.0004
  Gi4 aaaa.0000.0002
@@requires
BROADCAST
table[
@@hint
`table[src] = port` before deciding. A host can move: Gi4 now has `...0002`, so the entry updates.
@@solution
PORTS = ["Gi1", "Gi2", "Gi3", "Gi4"]
BROADCAST = "ffff.ffff.ffff"
frames = [
    ("Gi1", "aaaa.0000.0001", BROADCAST),
    ("Gi2", "aaaa.0000.0002", "aaaa.0000.0001"),
    ("Gi1", "aaaa.0000.0001", "aaaa.0000.0002"),
    ("Gi3", "aaaa.0000.0003", "aaaa.0000.0009"),
    ("Gi3", "aaaa.0000.0004", "aaaa.0000.0003"),
    ("Gi4", "aaaa.0000.0002", "aaaa.0000.0003"),
]

table = {}
for port, src, dst in frames:
    table[src] = port
    if dst == BROADCAST or dst not in table:
        others = [p for p in PORTS if p != port]
        decision = "flood to " + ", ".join(others)
    elif table[dst] == port:
        decision = "filter"
    else:
        decision = f"forward to {table[dst]}"
    print(f"{port} {src} -> {dst}: {decision}")

print("MAC table:")
for mac, port in sorted(table.items(), key=lambda item: (item[1], item[0])):
    print(f"  {port} {mac}")

@@exercise Check interfaces from show output
Parse `show interfaces status` output (fixed-width columns as shown) and report: err-disabled ports, ports with half duplex (`a-half` or `half`), connected ports at 10 or 100 Mb/s (`a-100`, `a-10`, `100`, `10`) that may be on a bad cable, and the count of `notconnect` ports. Use the header line to find each column's start position.
@@lang python
@@starter
output = """Port      Name       Status       Vlan       Duplex  Speed Type
Gi1/0/1   PC-Ana     connected    10         a-full a-1000 10/100/1000BaseTX
Gi1/0/2   Printer    notconnect   10           auto   auto 10/100/1000BaseTX
Gi1/0/3              err-disabled 20           auto   auto 10/100/1000BaseTX
Gi1/0/4   Old-PC     connected    10         a-half  a-100 10/100/1000BaseTX
Gi1/0/5   Camera     connected    30         a-full  a-100 10/100/1000BaseTX
Gi1/0/6              notconnect   1            auto   auto 10/100/1000BaseTX
Gi1/0/24  Uplink     connected    trunk        full   1000 10/100/1000BaseTX"""
@@expected
Gi1/0/3: err-disabled
Gi1/0/4: half duplex (a-half)
Gi1/0/4: only 100 Mb/s, check cable
Gi1/0/5: only 100 Mb/s, check cable
notconnect ports: 2
@@requires
header.index(
splitlines()
@@hint
`header = lines[0]`; slice the Status column with `line[header.index("Status"):header.index("Vlan")].strip()` (it can't be split on spaces because Name may be empty). Duplex and Speed are right-aligned and drift left of their headers, so take the whitespace-separated fields from the Vlan position onwards: `vlan, duplex, speed = line[vlan_at:].split()[:3]`.
@@solution
output = """Port      Name       Status       Vlan       Duplex  Speed Type
Gi1/0/1   PC-Ana     connected    10         a-full a-1000 10/100/1000BaseTX
Gi1/0/2   Printer    notconnect   10           auto   auto 10/100/1000BaseTX
Gi1/0/3              err-disabled 20           auto   auto 10/100/1000BaseTX
Gi1/0/4   Old-PC     connected    10         a-half  a-100 10/100/1000BaseTX
Gi1/0/5   Camera     connected    30         a-full  a-100 10/100/1000BaseTX
Gi1/0/6              notconnect   1            auto   auto 10/100/1000BaseTX
Gi1/0/24  Uplink     connected    trunk        full   1000 10/100/1000BaseTX"""

lines = output.splitlines()
header = lines[0]
status_at = header.index("Status")
vlan_at = header.index("Vlan")
notconnect = 0
for line in lines[1:]:
    port = line.split()[0]
    status = line[status_at:vlan_at].strip()
    vlan, duplex, speed = line[vlan_at:].split()[:3]
    if status == "err-disabled":
        print(f"{port}: err-disabled")
    elif status == "notconnect":
        notconnect += 1
    elif status == "connected":
        if duplex.endswith("half"):
            print(f"{port}: half duplex ({duplex})")
        if speed.removeprefix("a-") in ("10", "100"):
            print(f"{port}: only {speed.removeprefix('a-')} Mb/s, check cable")
print("notconnect ports:", notconnect)

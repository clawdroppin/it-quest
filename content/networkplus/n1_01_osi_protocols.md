@@lesson n1-osi OSI, TCP/IP, Ports and Protocols
@@text
## Encapsulation

As data goes down the layers, each adds its own header. The pieces have names, **PDUs**, that come up constantly:

| Layer | PDU | Address / key info | Devices |
|---|---|---|---|
| 7–5 Application, Presentation, Session | data | HTTP, TLS, DNS | firewalls (layer 7), load balancers, proxies |
| 4 Transport | **segment** (TCP) / **datagram** (UDP) | **ports** | |
| 3 Network | **packet** | **IP addresses** | **routers**, layer 3 switches |
| 2 Data link | **frame** | **MAC addresses** | **switches**, bridges, access points |
| 1 Physical | **bits** | signals | cables, hubs, repeaters, media converters |

The **TCP/IP model** squeezes this into 4 layers: Application (5–7), Transport (4), Internet (3), Link (1–2).

**MTU**: the largest frame payload, 1500 bytes on Ethernet. Bigger packets get fragmented; **jumbo frames** (9000) are used in storage networks.

## TCP and UDP

TCP opens with a **three-way handshake**: **SYN** → **SYN-ACK** → **ACK**. It numbers and acknowledges every segment, resends what's lost, and controls flow with a **window**. Closing uses FIN (or RST to abort). UDP just sends.

## Ports to memorize

| Port | Protocol | | Port | Protocol |
|---|---|---|---|---|
| 20/21 | FTP | | 389 | LDAP |
| 22 | SSH, SFTP | | 443 | HTTPS |
| 23 | Telnet (plain text!) | | 445 | SMB |
| 25 | SMTP | | 514 | Syslog |
| 53 | DNS (UDP, TCP for big replies and zone transfers) | | 587 | SMTP submission (STARTTLS) |
| 67/68 | DHCP | | 636 | LDAPS |
| 69 | TFTP | | 993 / 995 | IMAPS / POP3S |
| 80 | HTTP | | 1433 | Microsoft SQL Server |
| 123 | NTP | | 3306 | MySQL |
| 161/162 | SNMP / SNMP traps | | 3389 | RDP |
| 110 / 143 | POP3 / IMAP | | 5060/5061 | SIP (VoIP) |

Other protocols by number (IP protocol field, not ports): **ICMP** (ping, traceroute), **GRE** (tunnels), **ESP/AH** (IPsec).

## Topologies and architectures

- **Star** (everything to a switch: today's LANs), **mesh** (many redundant links: WAN cores, wireless mesh), **bus/ring** (legacy), **hybrid**.
- **Three-tier** campus: **core** (fast backbone), **distribution** (routing, policy), **access** (where users plug in). A **collapsed core** merges core and distribution in smaller sites.
- **Spine and leaf** in data centres: every leaf switch connects to every spine, so any server is two hops from any other. **North-south** traffic enters/leaves the data centre; **east-west** stays inside.
- **SDN** (software-defined networking) separates the **control plane** (decisions) from the **data plane** (forwarding), managed centrally.

@@quiz
At which OSI layer does a router make forwarding decisions?
- Layer 2
* Layer 3
- Layer 4
- Layer 7
? Routers forward packets by IP address (network layer).

@@quiz
What is the PDU at the data link layer?
* Frame
- Packet
- Segment
- Bit
? Bits (L1), frames (L2), packets (L3), segments/datagrams (L4).

@@quiz
What is the order of the TCP three-way handshake?
* SYN, SYN-ACK, ACK
- SYN, ACK, FIN
- ACK, SYN, SYN-ACK
- HELLO, ACK, DONE
? The client sends SYN, the server replies SYN-ACK, the client confirms with ACK.

@@quiz
Which protocol sends network device logs to a central server, and on which port?
* Syslog, UDP 514
- SNMP, UDP 161
- NTP, UDP 123
- LDAP, TCP 389
? SNMP (161/162) polls and receives traps; syslog is for log messages.

@@quiz
An admin still uses Telnet to manage switches. Why is that a problem, and what should replace it?
* Telnet sends everything, including passwords, in plain text; use SSH (port 22)
- Telnet is too slow; use FTP
- Telnet only works on IPv6; use RDP
- Nothing; Telnet is encrypted
? Anyone capturing traffic can read Telnet sessions.

@@quiz
Which port does LDAPS (LDAP over TLS) use?
* 636
- 389
- 443
- 3268
? 389 is plain LDAP. (3268/3269 are the AD Global Catalog.)

@@quiz
In a spine-and-leaf data centre, what's the traffic between two servers inside it called?
* East-west
- North-south
- Inbound
- Broadcast
? North-south traffic enters or leaves the data centre.

@@quiz
Which device works at layer 1 only?
* A hub
- A switch
- A router
- A firewall
? Hubs repeat signals out of every port; they don't read addresses.

@@flashcards
PDUs :: Bits, frames, packets, segments (TCP) / datagrams (UDP), data
L2 devices :: Switches, bridges, access points (MAC addresses)
L3 devices :: Routers, layer 3 switches (IP addresses)
TCP handshake :: SYN, SYN-ACK, ACK
MTU :: Maximum transmission unit; 1500 bytes on Ethernet
FTP :: 20/21
SSH / SFTP :: 22
Telnet :: 23 (insecure)
SMTP :: 25 (587 submission)
DNS :: 53
DHCP :: 67/68
TFTP :: 69
NTP :: 123
SNMP :: 161/162
LDAP / LDAPS :: 389 / 636
SMB :: 445
Syslog :: 514
SQL Server / MySQL :: 1433 / 3306
RDP :: 3389
SIP :: 5060/5061
spine and leaf :: Data-centre design: every leaf connects to every spine
SDN :: Separates control plane from data plane

@@drill
Which port does NTP use?
= 123
= udp 123
? Network Time Protocol keeps clocks in sync: vital for logs and Kerberos.

@@drill
Which port does SNMP use for polling (gets and sets)?
= 161
= udp 161
? Traps from devices go to 162.

@@exercise Port and protocol lookup
Write `lookup(port)` returning `(protocol, transport)` for the ports from the lesson, and `classify(port)` returning `well-known` (0-1023), `registered` (1024-49151) or `dynamic` (49152-65535). Print each port from the capture with its service (or `unknown`) and its range.
@@lang python
@@starter
PORTS = {
    20: ("FTP data", "TCP"), 21: ("FTP control", "TCP"), 22: ("SSH", "TCP"), 23: ("Telnet", "TCP"),
    25: ("SMTP", "TCP"), 53: ("DNS", "TCP/UDP"), 67: ("DHCP server", "UDP"), 80: ("HTTP", "TCP"),
    110: ("POP3", "TCP"), 123: ("NTP", "UDP"), 143: ("IMAP", "TCP"), 161: ("SNMP", "UDP"),
    389: ("LDAP", "TCP"), 443: ("HTTPS", "TCP"), 445: ("SMB", "TCP"), 3389: ("RDP", "TCP"),
}
capture = [443, 53, 3389, 51544, 161, 8080, 22]

def lookup(port):
    raise NotImplementedError
def classify(port):
    raise NotImplementedError


for port in capture:
    found = lookup(port)
    service = f"{found[0]} over {found[1]}" if found else "unknown"
    print(f"{port}: {service} ({classify(port)})")
@@expected
443: HTTPS over TCP (well-known)
53: DNS over TCP/UDP (well-known)
3389: RDP over TCP (registered)
51544: unknown (dynamic)
161: SNMP over UDP (well-known)
8080: unknown (registered)
22: SSH over TCP (well-known)
@@requires
def lookup
def classify
49151
@@hint
Keep the ports in a dictionary of tuples; `.get` returns None for ports you don't know.
@@solution
PORTS = {
    20: ("FTP data", "TCP"), 21: ("FTP control", "TCP"), 22: ("SSH", "TCP"), 23: ("Telnet", "TCP"),
    25: ("SMTP", "TCP"), 53: ("DNS", "TCP/UDP"), 67: ("DHCP server", "UDP"), 80: ("HTTP", "TCP"),
    110: ("POP3", "TCP"), 123: ("NTP", "UDP"), 143: ("IMAP", "TCP"), 161: ("SNMP", "UDP"),
    389: ("LDAP", "TCP"), 443: ("HTTPS", "TCP"), 445: ("SMB", "TCP"), 3389: ("RDP", "TCP"),
}
capture = [443, 53, 3389, 51544, 161, 8080, 22]

def lookup(port):
    return PORTS.get(port)

def classify(port):
    if port <= 1023:
        return "well-known"
    if port <= 49151:
        return "registered"
    return "dynamic"

for port in capture:
    found = lookup(port)
    service = f"{found[0]} over {found[1]}" if found else "unknown"
    print(f"{port}: {service} ({classify(port)})")

@@exercise Which OSI layer is it?
Write `layer(item)` that returns the OSI layer number and name for protocols, devices, PDUs and addresses, using `LAYERS`. Then write `encapsulate(data)` that prints how an HTTP request is wrapped on the way down: the data, then a TCP segment (with ports), an IP packet (with addresses), an Ethernet frame (with MACs and the FCS trailer) and finally bits. Print the layer for each item and the encapsulation of one request.
@@lang python
@@starter
NAMES = {1: "physical", 2: "data link", 3: "network", 4: "transport", 5: "session", 6: "presentation", 7: "application"}
LAYERS = {
    "HTTP": 7, "DNS": 7, "SSH": 7, "TLS": 6, "JPEG": 6, "TCP": 4, "UDP": 4, "segment": 4, "port": 4,
    "IP": 3, "ICMP": 3, "router": 3, "packet": 3, "IP address": 3,
    "Ethernet": 2, "switch": 2, "frame": 2, "MAC address": 2, "ARP": 2,
    "hub": 1, "cable": 1, "bits": 1, "repeater": 1,
}
items = ["switch", "ICMP", "TLS", "frame", "port", "hub", "MAC address", "DNS", "router", "UDP", "firewall"]

def layer(item):
    raise NotImplementedError


def encapsulate(data):
    raise NotImplementedError


for it in items:
    n, name = layer(it)
    print(f"{it}: " + (f"layer {n} ({name})" if n else name))
print("Sending GET /index.html:")
encapsulate("GET /index.html")
@@expected
switch: layer 2 (data link)
ICMP: layer 3 (network)
TLS: layer 6 (presentation)
frame: layer 2 (data link)
port: layer 4 (transport)
hub: layer 1 (physical)
MAC address: layer 2 (data link)
DNS: layer 7 (application)
router: layer 3 (network)
UDP: layer 4 (transport)
firewall: depends: a firewall can work at layers 3, 4 or 7
Sending GET /index.html:
  L7 application data: GET /index.html
  L4 transport  TCP segment: src port 51514, dst port 80 + data
  L3 network    IP packet: src 192.168.1.20, dst 203.0.113.10 + segment
  L2 data link  Ethernet frame: src MAC aa:bb:cc:00:11:22, dst MAC (the gateway) + packet + FCS
  L1 physical   bits on the wire
@@requires
def layer
def encapsulate
LAYERS.get(
@@hint
A dict from item to layer number, plus one from number to name. The destination MAC in the frame is the gateway's, not the web server's: frames only cross one link.
@@solution
NAMES = {1: "physical", 2: "data link", 3: "network", 4: "transport", 5: "session", 6: "presentation", 7: "application"}
LAYERS = {
    "HTTP": 7, "DNS": 7, "SSH": 7, "TLS": 6, "JPEG": 6, "TCP": 4, "UDP": 4, "segment": 4, "port": 4,
    "IP": 3, "ICMP": 3, "router": 3, "packet": 3, "IP address": 3,
    "Ethernet": 2, "switch": 2, "frame": 2, "MAC address": 2, "ARP": 2,
    "hub": 1, "cable": 1, "bits": 1, "repeater": 1,
}
items = ["switch", "ICMP", "TLS", "frame", "port", "hub", "MAC address", "DNS", "router", "UDP", "firewall"]

def layer(item):
    n = LAYERS.get(item)
    return (n, NAMES[n]) if n else (None, "depends: a firewall can work at layers 3, 4 or 7")

def encapsulate(data):
    steps = [
        (7, f"data: {data}"),
        (4, "TCP segment: src port 51514, dst port 80 + data"),
        (3, "IP packet: src 192.168.1.20, dst 203.0.113.10 + segment"),
        (2, "Ethernet frame: src MAC aa:bb:cc:00:11:22, dst MAC (the gateway) + packet + FCS"),
        (1, "bits on the wire"),
    ]
    for n, text in steps:
        print(f"  L{n} {NAMES[n]:<10} {text}")

for it in items:
    n, name = layer(it)
    print(f"{it}: " + (f"layer {n} ({name})" if n else name))
print("Sending GET /index.html:")
encapsulate("GET /index.html")

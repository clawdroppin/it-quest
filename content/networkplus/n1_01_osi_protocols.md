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

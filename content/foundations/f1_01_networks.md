@@lesson f1-networks How Networks and the Internet Work
@@text
Most tickets that say "the computer is broken" are really network problems. Here's the core you need before the Network+ track goes deeper.

## Addresses

- A **MAC address** (`3C:52:82:1A:9F:04`) is burned into each network card. It's used *inside* a local network.
- An **IP address** (`192.168.1.25`) identifies a device on a network and is used to route across networks.
- A **subnet mask** (`255.255.255.0`, or `/24`) says which part of the IP is the network: here, all of `192.168.1.x` is one local network.
- The **default gateway** (usually the router, e.g. `192.168.1.1`) is where traffic for *other* networks goes.
- A **DNS server** turns names (`www.google.com`) into IP addresses.

**Private** addresses are used inside homes and offices and never appear on the internet: `10.x.x.x`, `172.16.x.x`–`172.31.x.x`, `192.168.x.x`. The router translates them to its one public address (**NAT**). An address starting `169.254` (**APIPA**) means the device asked for an address and nobody answered: a DHCP problem.

## DHCP and DNS

When a laptop joins a network, **DHCP** hands it everything automatically: an IP address, mask, gateway and DNS servers, as a **lease** for some hours. Then, to open a website:

1. **DNS**: "what's the IP of `example.com`?" → `93.184.215.14`
2. **Connect** to that IP on port **443** (HTTPS)
3. **TLS** sets up encryption and checks the site's certificate
4. **HTTP** asks for the page

## Ports and protocols

An IP address finds the computer; a **port** finds the program on it. **TCP** is reliable and ordered (web, email, file transfers); **UDP** is fast with no guarantees (DNS lookups, video calls, games).

| Port | Protocol | Use |
|---|---|---|
| 22 | SSH | secure remote shell (Linux, network gear) |
| 25 | SMTP | sending email between servers |
| 53 | DNS | name lookups |
| 67/68 | DHCP | automatic addresses |
| 80 / 443 | HTTP / HTTPS | web |
| 3389 | RDP | Windows Remote Desktop |
| 445 | SMB | Windows file sharing |

## The layers

Networking is described in **layers**, so you can troubleshoot one at a time. The **OSI model** has 7; most people think in 4 groups:

| OSI layer | Examples | "Is it...?" |
|---|---|---|
| 1 Physical | cables, Wi-Fi signal | plugged in? link light on? |
| 2 Data link | MAC addresses, switches | on the right network/VLAN? |
| 3 Network | IP addresses, routers | has a valid IP? can it ping the gateway? |
| 4 Transport | TCP/UDP ports | is the port open / blocked by a firewall? |
| 5–7 Session, presentation, application | HTTP, DNS, TLS | does the app or name lookup work? |

Mnemonic, from layer 1 up: **P**lease **D**o **N**ot **T**hrow **S**ausage **P**izza **A**way.

## The troubleshooting commands

```powershell,norun
ipconfig /all          # my IP, mask, gateway, DNS, DHCP lease (Linux: ip a)
ping 192.168.1.1       # can I reach the gateway?
ping 8.8.8.8           # can I reach the internet by IP?
nslookup google.com    # does DNS work?
tracert google.com     # where along the path does it stop? (Linux: traceroute)
```

If `ping 8.8.8.8` works but `ping google.com` fails, the internet is fine and **DNS** is broken. That one test saves hours.

@@quiz
A laptop has the IP address 169.254.12.7. What does that tell you?
* It didn't get an address from DHCP and assigned itself one (APIPA)
- It's connected to the internet normally
- It has a static IP set by an admin
- It's using IPv6
? 169.254.x.x means "I asked for an address and nobody answered". Check the cable/Wi-Fi and the DHCP server.

@@quiz
`ping 8.8.8.8` works, but websites won't open by name. What's the most likely problem?
* DNS
- The network cable
- The default gateway
- The web browser's cache only
? Reaching an internet IP proves the link, IP settings and gateway work. Names failing points at DNS.

@@quiz
Which port does HTTPS use by default?
- 80
* 443
- 22
- 3389
? 80 is plain HTTP, 22 is SSH, 3389 is RDP.

@@quiz
Which of these is a private IP address?
* 10.20.30.40
- 8.8.8.8
- 172.32.0.1
- 192.169.1.1
? Private ranges: 10.0.0.0/8, 172.16.0.0/12 (172.16–172.31), 192.168.0.0/16.

@@quiz
Which protocol automatically gives devices their IP address, gateway and DNS servers?
* DHCP
- DNS
- NAT
- ARP
? Dynamic Host Configuration Protocol.

@@flashcards
IP address :: Identifies a device on a network; used for routing
MAC address :: Hardware address of a network card; used on the local network
subnet mask :: Which part of an IP is the network, e.g. 255.255.255.0 = /24
default gateway :: The router that leads to other networks
DNS :: Turns names into IP addresses (port 53)
DHCP :: Gives out IP settings automatically
NAT :: Router translates private addresses to its public address
APIPA (169.254.x.x) :: Self-assigned address: DHCP failed
TCP vs UDP :: Reliable and ordered vs fast without guarantees
OSI layers :: Physical, Data link, Network, Transport, Session, Presentation, Application
ipconfig /all :: Show all IP settings on Windows
nslookup :: Test DNS lookups

@@drill
What port does SSH use?
= 22
? Secure Shell: encrypted remote command line.

@@drill
What port does Windows Remote Desktop (RDP) use?
= 3389
= tcp 3389
? Never expose 3389 directly to the internet: it's one of the most attacked ports.

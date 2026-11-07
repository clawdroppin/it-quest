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

@@exercise Walk through DHCP and fall back to APIPA
Simulate a laptop joining a network. DHCP is four messages: **D**iscover (broadcast), **O**ffer, **R**equest, **A**ck. Write `join(network)` that prints each message and returns the address the laptop ends up with:

- If no DHCP server answers the Discover (`server` is `None`), the laptop gives itself an **APIPA** address `169.254.x.y` (use the last two bytes of its MAC, as numbers) and the result says `DHCP failed`.
- If the server's pool is empty it doesn't make an offer: same fallback.
- Otherwise it offers the first free address with the gateway and DNS server, the laptop requests it, and the server acknowledges it.

Then say what the user would see: an APIPA address means "connected, no internet".
@@lang python
@@starter
networks = [
    {"name": "Office Wi-Fi", "mac": "3c:22:fb:10:0a:2f",
     "server": {"pool": ["192.168.1.50", "192.168.1.51"], "gateway": "192.168.1.1", "dns": "192.168.1.1"}},
    {"name": "Meeting room cable (DHCP server down)", "mac": "3c:22:fb:10:0a:2f", "server": None},
    {"name": "Guest Wi-Fi (pool full)", "mac": "a4:83:e7:01:c8:99",
     "server": {"pool": [], "gateway": "10.0.0.1", "dns": "1.1.1.1"}},
]

def apipa(mac):
    raise NotImplementedError


def join(network):
    raise NotImplementedError


for n in networks:
    addr, status = join(n)
    print(f"  result: {addr} ({status})")
@@expected
Office Wi-Fi:
  DISCOVER from 3c:22:fb:10:0a:2f to 255.255.255.255
  OFFER 192.168.1.50, gateway 192.168.1.1, DNS 192.168.1.1
  REQUEST 192.168.1.50
  ACK 192.168.1.50
  result: 192.168.1.50 (OK)
Meeting room cable (DHCP server down):
  DISCOVER from 3c:22:fb:10:0a:2f to 255.255.255.255
  (no answer) self-assigned 169.254.10.47
  result: 169.254.10.47 (DHCP failed: connected, no internet)
Guest Wi-Fi (pool full):
  DISCOVER from a4:83:e7:01:c8:99 to 255.255.255.255
  (no free addresses, no offer) self-assigned 169.254.200.153
  result: 169.254.200.153 (DHCP failed: connected, no internet)
@@requires
def join
def apipa
169.254
int(
@@hint
`int("0a", 16)` turns a MAC byte into a number. For the offer, `pool.pop(0)` takes the first free address and removes it from the pool.
@@solution
networks = [
    {"name": "Office Wi-Fi", "mac": "3c:22:fb:10:0a:2f",
     "server": {"pool": ["192.168.1.50", "192.168.1.51"], "gateway": "192.168.1.1", "dns": "192.168.1.1"}},
    {"name": "Meeting room cable (DHCP server down)", "mac": "3c:22:fb:10:0a:2f", "server": None},
    {"name": "Guest Wi-Fi (pool full)", "mac": "a4:83:e7:01:c8:99",
     "server": {"pool": [], "gateway": "10.0.0.1", "dns": "1.1.1.1"}},
]

def apipa(mac):
    parts = mac.split(":")
    return f"169.254.{int(parts[-2], 16)}.{int(parts[-1], 16)}"

def join(network):
    print(f"{network['name']}:")
    print(f"  DISCOVER from {network['mac']} to 255.255.255.255")
    server = network["server"]
    if server is None or not server["pool"]:
        reason = "no answer" if server is None else "no free addresses, no offer"
        addr = apipa(network["mac"])
        print(f"  ({reason}) self-assigned {addr}")
        return addr, "DHCP failed: connected, no internet"
    addr = server["pool"].pop(0)
    print(f"  OFFER {addr}, gateway {server['gateway']}, DNS {server['dns']}")
    print(f"  REQUEST {addr}")
    print(f"  ACK {addr}")
    return addr, "OK"

for n in networks:
    addr, status = join(n)
    print(f"  result: {addr} ({status})")

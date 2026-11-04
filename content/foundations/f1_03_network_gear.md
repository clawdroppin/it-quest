@@lesson f1-gear Switches, Routers, Wi-Fi and Cabling
@@text
Addresses and protocols are the logic of a network; this lesson is the physical side: the boxes, cables and radio waves. When a whole floor loses its network, or Wi-Fi is "slow in the meeting room", you need to know what each device does and where to look.

## The devices

| Device | Layer | What it does |
|---|---|---|
| **Switch** | 2 (data link) | connects devices inside one network; forwards frames by **MAC address** |
| **Router** | 3 (network) | connects different networks; forwards packets by **IP address** |
| **Access point (AP)** | 2 | connects Wi-Fi devices to the wired network |
| **Firewall** | 3–7 | allows or blocks traffic by rules |
| **Modem / ONT** | 1 | converts the ISP's line (fibre, cable, DSL) to Ethernet |
| Hub (obsolete) | 1 | repeated everything to every port |

A home "router" is all of these in one box: router, switch, AP, firewall, DHCP server and DNS forwarder. In an office they're separate devices, often from Cisco, Juniper, Aruba (HPE), Fortinet, Palo Alto or Ubiquiti.

### How a switch learns

Every network card has a **MAC address** (48 bits, hex, e.g. `3C:52:82:1A:9F:04`), burned in by the maker. A switch watches the source MAC of each frame and records which port it came from in its **MAC address table**. Frames for a known MAC go only to that port; frames for unknown MACs (and broadcasts) go to all ports. To find a device's MAC from its IP, devices use **ARP** ("who has 192.168.1.1? Tell me your MAC").

### VLANs

A managed switch can split its ports into separate virtual networks, **VLANs**: ports 1–20 for staff PCs (VLAN 10), 21–30 for phones (VLAN 20), 31–40 for printers (VLAN 30). Devices in different VLANs can't talk without a router or firewall in between, which keeps guest Wi-Fi away from servers. A **trunk** port carries several VLANs (tagged with **802.1Q**) between switches and to APs. A PC plugged into the wrong VLAN port gets an address from the wrong network, or none.

### PoE

**Power over Ethernet** sends power down the network cable to APs, IP phones and cameras. A switch has a PoE **budget** (e.g. 370 W across all ports); when it's exhausted, the last devices plugged in don't power up.

## Cabling

- **Twisted pair copper** (RJ45 plugs): categories **Cat5e** (1 Gbps), **Cat6** (1 Gbps, 10 Gbps up to 55 m), **Cat6a** (10 Gbps to 100 m). Maximum run: **100 m**.
- **Fibre**: for long runs and between buildings or floors. **Multimode** (orange/aqua, up to a few hundred metres, cheaper), **single-mode** (yellow, kilometres). Connectors: LC (small, most common), SC. Switches take **SFP** modules to plug fibre in.
- Patch panels, labelled cables and a documented port map turn "which switch port is desk 3-14?" from an hour into a minute.

Common faults: a damaged cable gives 100 Mbps instead of 1 Gbps (or link flapping); a loop (both ends of one cable plugged into the same switch) can take a whole network down unless **Spanning Tree Protocol (STP)** blocks it.

## Wi-Fi

| Standard | Name | Bands | Notes |
|---|---|---|---|
| 802.11n | Wi-Fi 4 | 2.4 / 5 GHz | old, still common |
| 802.11ac | Wi-Fi 5 | 5 GHz | |
| 802.11ax | Wi-Fi 6 / 6E | 2.4 / 5 / **6 GHz** (6E) | efficient in crowded spaces |
| 802.11be | Wi-Fi 7 | 2.4 / 5 / 6 GHz | multi-link, very wide channels |

- **2.4 GHz**: longer range, goes through walls, but only 3 non-overlapping channels (1, 6, 11) and crowded by neighbours, Bluetooth and microwaves.
- **5 GHz**: many more channels, faster, shorter range.
- **6 GHz**: lots of clean spectrum, shortest range, needs Wi-Fi 6E/7 devices.
- **SSID**: the network name. **Signal strength** in dBm: −50 is excellent, −67 the usual minimum for voice and video, −80 barely usable.
- **Security**: **WPA3** (current), WPA2 (still common). **Personal** mode uses a shared password (PSK); **Enterprise** mode (802.1X) logs each user or device in individually with a certificate or account via a RADIUS server, so a leaver's access can be removed without changing everyone's password. Never WEP or open networks for staff.
- Offices use many APs managed by a **controller** (or cloud), with **roaming** between them as people move.

"Wi-Fi is slow in the meeting room": check signal (dBm) and which band the device is on, how many people are connected to that AP, interference and channel overlap (a Wi-Fi analyser app), and whether the problem is actually the internet link rather than Wi-Fi.

@@quiz
What does a switch use to decide which port to send a frame to?
* The destination MAC address and its MAC address table
- The destination IP address
- The port number (e.g. 443)
- It always sends to all ports
? Switches are layer 2; routers use IP addresses.

@@quiz
A PC on a new desk gets an address from the printer network instead of the staff network. Likely cause?
- The PC's Wi-Fi card is broken
* The wall port is patched to a switch port in the wrong VLAN
- DNS is down
- The cable is Cat5e
? Each VLAN has its own subnet and DHCP scope.

@@quiz
What is the maximum length of a twisted-pair Ethernet run?
- 10 m
* 100 m
- 500 m
- 2 km
? Longer runs need fibre or another switch.

@@quiz
Why use WPA3/WPA2-Enterprise (802.1X) rather than a shared Wi-Fi password in an office?
* Each user/device authenticates individually, so access can be revoked per person
- It's faster
- It doesn't need a password at all
- It works only on 2.4 GHz
? With a shared PSK, one leaver means changing it for everyone.

@@flashcards
switch :: layer 2, forwards by MAC address
router :: layer 3, forwards between networks by IP
access point :: connects Wi-Fi clients to the LAN
MAC address :: 48-bit hardware address
ARP :: finds the MAC for an IP address
VLAN :: virtual network on a switch
trunk / 802.1Q :: link carrying tagged VLANs
PoE :: power over the network cable
Cat6a :: 10 Gbps up to 100 m
single-mode fibre :: long distance (km), yellow
SFP :: pluggable module for fibre/copper
STP :: prevents switching loops
2.4 / 5 / 6 GHz :: range vs speed vs clean spectrum
−67 dBm :: minimum good signal for calls
WPA3-Enterprise / 802.1X :: individual Wi-Fi logins via RADIUS

@@drill
What is the maximum run length of copper Ethernet, in metres?
= 100
? Including patch cables.

@@drill
Which three 2.4 GHz Wi-Fi channels don't overlap? (comma-separated)
= 1, 6, 11
= 1,6,11
? Everything else overlaps its neighbours.

@@exercise A switch learning MAC addresses
Simulate a switch. For each frame `(in_port, src_mac, dst_mac)`: learn `src_mac → in_port` in the MAC table, then if `dst_mac` is the broadcast address `ff:ff:ff:ff:ff:ff` or unknown, **flood** to every port except the incoming one; otherwise forward to the known port. Print one line per frame, then the final MAC table sorted by port.
@@lang python
@@starter
PORTS = [1, 2, 3, 4]
BROADCAST = "ff:ff:ff:ff:ff:ff"
frames = [
    (1, "aa:aa:aa:00:00:01", BROADCAST),            # PC1 ARPs for the printer
    (3, "aa:aa:aa:00:00:03", "aa:aa:aa:00:00:01"),  # printer replies
    (1, "aa:aa:aa:00:00:01", "aa:aa:aa:00:00:03"),  # PC1 prints
    (2, "aa:aa:aa:00:00:02", "aa:aa:aa:00:00:04"),  # PC2 to an unknown device
]
table = {}
@@expected
aa:aa:aa:00:00:01 -> ff:ff:ff:ff:ff:ff: flood to ports [2, 3, 4]
aa:aa:aa:00:00:03 -> aa:aa:aa:00:00:01: forward to port 1
aa:aa:aa:00:00:01 -> aa:aa:aa:00:00:03: forward to port 3
aa:aa:aa:00:00:02 -> aa:aa:aa:00:00:04: flood to ports [1, 3, 4]
MAC table:
  port 1: aa:aa:aa:00:00:01
  port 2: aa:aa:aa:00:00:02
  port 3: aa:aa:aa:00:00:03
@@requires
table[src
BROADCAST
@@hint
`table[src] = port` first. Then `if dst == BROADCAST or dst not in table: out = [p for p in PORTS if p != port]`.
@@solution
PORTS = [1, 2, 3, 4]
BROADCAST = "ff:ff:ff:ff:ff:ff"
frames = [
    (1, "aa:aa:aa:00:00:01", BROADCAST),
    (3, "aa:aa:aa:00:00:03", "aa:aa:aa:00:00:01"),
    (1, "aa:aa:aa:00:00:01", "aa:aa:aa:00:00:03"),
    (2, "aa:aa:aa:00:00:02", "aa:aa:aa:00:00:04"),
]
table = {}

for port, src, dst in frames:
    table[src] = port
    if dst == BROADCAST or dst not in table:
        out = [p for p in PORTS if p != port]
        print(f"{src} -> {dst}: flood to ports {out}")
    else:
        print(f"{src} -> {dst}: forward to port {table[dst]}")

print("MAC table:")
for mac, port in sorted(table.items(), key=lambda item: item[1]):
    print(f"  port {port}: {mac}")

@@exercise Wi-Fi survey report
A walk-around survey recorded the signal strength (dBm) and band at each spot. Rate each spot: `excellent` at −50 or better, `good` at −67 or better, `fair` at −75 or better, otherwise `poor`. Print each room with its rating, then list the rooms that are below `good` (they need another AP or a better placement) and the average signal on each band rounded to one decimal.
@@lang python
@@starter
survey = [
    ("Reception", -48, "5 GHz"), ("Meeting room A", -71, "5 GHz"), ("Open office", -58, "5 GHz"),
    ("Kitchen", -66, "2.4 GHz"), ("Meeting room B", -79, "2.4 GHz"), ("Server room", -62, "5 GHz"),
]

def rate(dbm):
    raise NotImplementedError
@@expected
Reception: -48 dBm (5 GHz) excellent
Meeting room A: -71 dBm (5 GHz) fair
Open office: -58 dBm (5 GHz) good
Kitchen: -66 dBm (2.4 GHz) good
Meeting room B: -79 dBm (2.4 GHz) poor
Server room: -62 dBm (5 GHz) good
Needs attention: Meeting room A, Meeting room B
Average on 5 GHz: -59.8 dBm
Average on 2.4 GHz: -72.5 dBm
@@requires
def rate
>= -67
@@hint
dBm values are negative: "better" means closer to zero, so `if dbm >= -50`. For averages, collect values per band in a dict of lists.
@@solution
survey = [
    ("Reception", -48, "5 GHz"), ("Meeting room A", -71, "5 GHz"), ("Open office", -58, "5 GHz"),
    ("Kitchen", -66, "2.4 GHz"), ("Meeting room B", -79, "2.4 GHz"), ("Server room", -62, "5 GHz"),
]

def rate(dbm):
    if dbm >= -50:
        return "excellent"
    if dbm >= -67:
        return "good"
    if dbm >= -75:
        return "fair"
    return "poor"

by_band = {}
weak = []
for room, dbm, band in survey:
    r = rate(dbm)
    print(f"{room}: {dbm} dBm ({band}) {r}")
    if r in ("fair", "poor"):
        weak.append(room)
    by_band.setdefault(band, []).append(dbm)

print("Needs attention:", ", ".join(weak))
for band, values in by_band.items():
    print(f"Average on {band}: {sum(values) / len(values):.1f} dBm")

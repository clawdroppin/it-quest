@@lesson a1-networking A+ Networking: Wi-Fi, Cables, SOHO and Tools
@@text
(Start with *How Networks Work* in the Foundations track: IP, DNS, DHCP and ports are assumed here.)

## Wi-Fi standards

| Standard | Name | Bands | Notes |
|---|---|---|---|
| 802.11a | | 5 GHz | 54 Mb/s |
| 802.11b / g | | 2.4 GHz | 11 / 54 Mb/s |
| 802.11n | Wi-Fi 4 | 2.4 + 5 GHz | MIMO |
| 802.11ac | Wi-Fi 5 | 5 GHz | |
| 802.11ax | Wi-Fi 6 / **6E** | 2.4 + 5 (+ **6 GHz** for 6E) | better in crowded places |
| 802.11be | Wi-Fi 7 | 2.4 + 5 + 6 GHz | newest |

**2.4 GHz** reaches further and through walls but is crowded (only channels **1, 6, 11** don't overlap). **5/6 GHz** is faster with a shorter range. Security: use **WPA3** (or WPA2 with AES); WEP and WPA are broken. Change default admin passwords and turn off **WPS**.

## Cables

| Cable | Max speed | Max length |
|---|---|---|
| **Cat 5e** | 1 Gb/s | 100 m |
| **Cat 6** | 10 Gb/s (up to ~55 m) | 100 m at 1 Gb/s |
| **Cat 6a** | 10 Gb/s | 100 m |
| **Fiber**: single-mode / multimode | very high | kilometres / hundreds of metres |
| **Coaxial** (RG6) | | cable internet, TV |

Twisted pair is wired to **T568A** or **T568B** (B is most common). Same standard on both ends = straight-through cable; one of each = crossover (rarely needed now: ports auto-detect, called Auto-MDIX). **STP** (shielded) resists interference near motors and fluorescent lights; **plenum**-rated cable is required in air-handling spaces because it gives off less toxic smoke.

## Network devices

- **Switch**: connects devices in one network, forwards by MAC address. **Managed** switches support VLANs. **PoE** switches power phones, cameras and access points through the cable.
- **Router**: connects networks, forwards by IP; at home it also does NAT, DHCP and Wi-Fi.
- **Access point**: provides Wi-Fi to a wired network.
- **Firewall**: allows or blocks traffic by rules.
- **Modem / ONT**: connects to the ISP (cable, DSL, fiber).

A **SOHO** (small office/home office) router setup: change the admin password, update the firmware, set WPA3, a separate **guest network**, DHCP range and **reservations** for printers, **port forwarding** only if needed, and disable UPnP if you don't need it.

## Tools

| Tool | Use |
|---|---|
| **Crimper** | attach RJ45 plugs to a cable |
| **Cable tester** | checks the wire pairs are connected correctly |
| **Toner probe** | find which cable in a bundle goes where |
| **Punchdown tool** | terminate wires on a patch panel or keystone jack |
| **Loopback plug** | test a network port on its own |
| **Wi-Fi analyzer** | see channels and signal strength |

@@quiz
A small office's Wi-Fi is slow, and a Wi-Fi analyzer shows many neighbours' networks on 2.4 GHz channels 3, 4 and 9. What should you do?
* Move the router to channel 1, 6 or 11, or use the 5 GHz band
- Switch to WEP
- Lower the transmit power to zero
- Replace all the Ethernet cables
? 1, 6 and 11 are the only non-overlapping 2.4 GHz channels; 5 GHz has many more.

@@quiz
You need to run a 10 Gb/s Ethernet link 90 metres through a building. Which cable?
* Cat 6a
- Cat 5e
- Cat 6
- RG6 coaxial
? Cat 6 only does 10 Gb/s up to about 55 m. Cat 6a does it at 100 m.

@@quiz
Which tool helps find which unlabelled cable at the patch panel goes to a particular wall jack?
* A toner probe
- A crimper
- A loopback plug
- A multimeter only
? Connect the tone generator at the jack, and the probe beeps near the matching cable.

@@quiz
Which Wi-Fi security setting should a new SOHO router use?
* WPA3 (or WPA2-AES if devices don't support WPA3)
- WEP
- WPA with TKIP
- Open network with a hidden SSID
? WEP and TKIP are broken; hiding the SSID isn't security.

@@quiz
New IP security cameras should get power and data from the same cable. What's needed?
* A PoE switch (or PoE injectors)
- A crossover cable
- A modem
- Fiber cable
? Power over Ethernet carries power on the network cable.

@@quiz
Which Wi-Fi standard added the 6 GHz band?
* Wi-Fi 6E
- Wi-Fi 4
- Wi-Fi 5
- 802.11g
? Wi-Fi 6E is 802.11ax extended into 6 GHz; Wi-Fi 7 also uses it.

@@flashcards
802.11n :: Wi-Fi 4; 2.4 and 5 GHz
802.11ac :: Wi-Fi 5; 5 GHz
802.11ax :: Wi-Fi 6 (6E adds 6 GHz)
2.4 GHz non-overlapping channels :: 1, 6, 11
WPA3 :: Current Wi-Fi security standard
Cat 5e :: 1 Gb/s, 100 m
Cat 6a :: 10 Gb/s, 100 m
T568B :: The most common RJ45 wiring standard
plenum cable :: Fire-safe cable for air-handling spaces
PoE :: Power over Ethernet
switch vs router :: Forwards by MAC in one network vs by IP between networks
toner probe :: Traces cables
punchdown tool :: Terminates wires on patch panels and keystones
DHCP reservation :: Always give a device (like a printer) the same IP

@@drill
What is the maximum length of a Cat 6a Ethernet run, in metres?
= 100
= 100m
= 100 m
? The 100 m limit applies to all common twisted-pair categories.

@@exercise Pick the right cable
Write `cable_for(distance_m, interference, speed_gbps)` that recommends a cable:

- over 100 m: `fiber` (copper Ethernet is limited to 100 m)
- heavy interference (`True`): `STP Cat 6a` (shielded)
- 10 Gbps or more: `Cat 6a`
- otherwise: `Cat 6`

Print each run with its recommendation.
@@lang python
@@starter
runs = [
    ("Office to switch", 35, False, 1),
    ("Server rack", 15, False, 10),
    ("Factory floor", 60, True, 1),
    ("Building to building", 240, False, 10),
    ("Lab bench", 90, False, 1),
]

def cable_for(distance_m, interference, speed_gbps):
    raise NotImplementedError


for name, distance, noisy, speed in runs:
    print(f"{name} ({distance} m, {speed} Gbps): {cable_for(distance, noisy, speed)}")
@@expected
Office to switch (35 m, 1 Gbps): Cat 6
Server rack (15 m, 10 Gbps): Cat 6a
Factory floor (60 m, 1 Gbps): STP Cat 6a
Building to building (240 m, 10 Gbps): fiber
Lab bench (90 m, 1 Gbps): Cat 6
@@requires
def cable_for
100
@@hint
Check the distance first: no copper cable fixes a 240 m run.
@@solution
runs = [
    ("Office to switch", 35, False, 1),
    ("Server rack", 15, False, 10),
    ("Factory floor", 60, True, 1),
    ("Building to building", 240, False, 10),
    ("Lab bench", 90, False, 1),
]

def cable_for(distance_m, interference, speed_gbps):
    if distance_m > 100:
        return "fiber"
    if interference:
        return "STP Cat 6a"
    if speed_gbps >= 10:
        return "Cat 6a"
    return "Cat 6"

for name, distance, noisy, speed in runs:
    print(f"{name} ({distance} m, {speed} Gbps): {cable_for(distance, noisy, speed)}")

@@exercise Match Wi-Fi standards to bands and speeds
Using the lesson's Wi-Fi table, write `standards_for(need)` that returns every standard that supports the needed `band` (2.4, 5 or 6 GHz), sorted oldest to newest, and `best(need)` that returns the newest one the **client** supports (`client_max`, a standard name) as long as it also supports the band. Print the advice for each device, adding `use channel 1, 6 or 11` for 2.4 GHz and `use WPA3` for everything.
@@lang python
@@starter
TABLE = [
    ("802.11a", "", {5}),
    ("802.11b", "", {2.4}),
    ("802.11g", "", {2.4}),
    ("802.11n", "Wi-Fi 4", {2.4, 5}),
    ("802.11ac", "Wi-Fi 5", {5}),
    ("802.11ax", "Wi-Fi 6", {2.4, 5}),
    ("802.11ax (6E)", "Wi-Fi 6E", {2.4, 5, 6}),
    ("802.11be", "Wi-Fi 7", {2.4, 5, 6}),
]
ORDER = [s for s, _, _ in TABLE]
devices = [
    {"name": "Old barcode scanner", "band": 2.4, "client_max": "802.11g"},
    {"name": "Laptop in a crowded office", "band": 6, "client_max": "802.11be"},
    {"name": "Smart TV", "band": 5, "client_max": "802.11ac"},
    {"name": "Warehouse sensor far from the AP", "band": 2.4, "client_max": "802.11n"},
    {"name": "Old phone", "band": 6, "client_max": "802.11ac"},
]

def standards_for(need):
    raise NotImplementedError


def best(need):
    raise NotImplementedError


for d in devices:
    pick = best(d)
    if pick is None:
        print(f"{d['name']}: no standard it supports runs on {d['band']} GHz")
        continue
    name = dict((s, n) for s, n, _ in TABLE)[pick]
    tips = ["use WPA3"] + (["use channel 1, 6 or 11"] if d["band"] == 2.4 else [])
    print(f"{d['name']}: {pick}{' (' + name + ')' if name else ''} on {d['band']} GHz; " + ", ".join(tips))
@@expected
Old barcode scanner: 802.11g on 2.4 GHz; use WPA3, use channel 1, 6 or 11
Laptop in a crowded office: 802.11be (Wi-Fi 7) on 6 GHz; use WPA3
Smart TV: 802.11ac (Wi-Fi 5) on 5 GHz; use WPA3
Warehouse sensor far from the AP: 802.11n (Wi-Fi 4) on 2.4 GHz; use WPA3, use channel 1, 6 or 11
Old phone: no standard it supports runs on 6 GHz
@@requires
def standards_for
def best
ORDER.index(
@@hint
Keep the table in age order, so an index tells you how new a standard is. Filter by band first, then by the client's limit, and take the last one.
@@solution
TABLE = [
    ("802.11a", "", {5}),
    ("802.11b", "", {2.4}),
    ("802.11g", "", {2.4}),
    ("802.11n", "Wi-Fi 4", {2.4, 5}),
    ("802.11ac", "Wi-Fi 5", {5}),
    ("802.11ax", "Wi-Fi 6", {2.4, 5}),
    ("802.11ax (6E)", "Wi-Fi 6E", {2.4, 5, 6}),
    ("802.11be", "Wi-Fi 7", {2.4, 5, 6}),
]
ORDER = [s for s, _, _ in TABLE]
devices = [
    {"name": "Old barcode scanner", "band": 2.4, "client_max": "802.11g"},
    {"name": "Laptop in a crowded office", "band": 6, "client_max": "802.11be"},
    {"name": "Smart TV", "band": 5, "client_max": "802.11ac"},
    {"name": "Warehouse sensor far from the AP", "band": 2.4, "client_max": "802.11n"},
    {"name": "Old phone", "band": 6, "client_max": "802.11ac"},
]

def standards_for(need):
    return [s for s, _, bands in TABLE if need["band"] in bands]

def best(need):
    limit = ORDER.index(need["client_max"])
    usable = [s for s in standards_for(need) if ORDER.index(s) <= limit]
    return usable[-1] if usable else None

for d in devices:
    pick = best(d)
    if pick is None:
        print(f"{d['name']}: no standard it supports runs on {d['band']} GHz")
        continue
    name = dict((s, n) for s, n, _ in TABLE)[pick]
    tips = ["use WPA3"] + (["use channel 1, 6 or 11"] if d["band"] == 2.4 else [])
    print(f"{d['name']}: {pick}{' (' + name + ')' if name else ''} on {d['band']} GHz; " + ", ".join(tips))

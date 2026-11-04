@@lesson n1-media Cabling, Connectors and Transceivers
@@text
Network+ expects you to pick the right cable, connector and transceiver for a link, and to recognise a wiring mistake. In a job this is the difference between a link that runs at 10 Gb/s and one that flaps all day.

## Copper twisted pair

| Category | Max speed | Max length at that speed | Notes |
|---|---|---|---|
| **Cat 5e** | 1 Gb/s | 100 m | minimum for gigabit |
| **Cat 6** | 1 Gb/s (10 Gb/s up to 55 m) | 100 m / 55 m | common in offices |
| **Cat 6a** | 10 Gb/s | 100 m | augmented; thicker; good for PoE++ and Wi-Fi 6/7 APs |
| **Cat 7 / 7a** | 10 Gb/s+ | 100 m | shielded, GG45/TERA connectors (not a TIA standard) |
| **Cat 8** | 25/40 Gb/s | 30 m | data centre top-of-rack only |

100 m (328 ft) is the copper Ethernet limit: 90 m horizontal run + 10 m of patch cords.

- **UTP** (unshielded) vs **STP/FTP** (shielded) for electrically noisy areas: factories, near motors, lift shafts.
- **Plenum** (fire-retardant jacket, low smoke) is required in air-handling spaces above drop ceilings; **riser** for vertical runs between floors; **PVC** elsewhere.
- **Direct burial** cable for outdoor trenches.

## Wiring standards

**T568A** and **T568B** define the pin order of the RJ45. Use the same standard on both ends for a **straight-through** cable; A on one end and B on the other makes a **crossover** (rarely needed now thanks to **auto-MDIX**).

T568B: orange-white, orange, green-white, blue, blue-white, green, brown-white, brown.

Common faults: **split pairs** (right pins, wrong pairs → crosstalk), **reversed pair**, **open** (broken wire), **short**, untwisting too much at the connector (NEXT: near-end crosstalk).

## Fibre

| | **Single-mode (SMF)** | **Multimode (MMF)** |
|---|---|---|
| Core | ~9 µm | 50 or 62.5 µm |
| Light source | laser | LED/VCSEL |
| Distance | kilometres (10–80+ km) | up to ~550 m (depends on OM grade and speed) |
| Jacket colour (typical) | yellow | aqua (OM3/OM4), lime (OM5), orange (OM1/OM2) |
| Cost | cheaper fibre, pricier optics | cheaper optics |

Fibre is immune to EMI, doesn't carry electricity between buildings (no ground loops) and is hard to tap.

Connectors: **LC** (small, the common one), **SC** (square, push-pull), **ST** (bayonet, older), **MPO/MTP** (many fibres, 40/100G). Polish: **UPC** (blue) vs **APC** (green, angled; don't mate APC with UPC).

## Transceivers

Modular optics plug into switch ports:

| Form factor | Speed |
|---|---|
| **SFP** | 1 Gb/s |
| **SFP+** | 10 Gb/s |
| **SFP28** | 25 Gb/s |
| **QSFP+** | 40 Gb/s |
| **QSFP28** | 100 Gb/s |
| **QSFP-DD / OSFP** | 400–800 Gb/s |

Both ends must match in **wavelength** and **type** (SR = short reach multimode, LR = long reach single-mode). **BiDi** optics use one fibre with two wavelengths. **DAC** (direct attach copper) and **AOC** (active optical) cables are cheap short links inside a rack.

## Other media

- **Coaxial** (RG-6 for cable internet/TV, F-connector; RG-59 older).
- **Twinaxial**: DAC cables.
- **Wireless** links between buildings (point-to-point).

## PoE

| Standard | Name | Watts at PSE (port) |
|---|---|---|
| 802.3af | PoE | 15.4 W |
| 802.3at | PoE+ | 30 W |
| 802.3bt Type 3 | PoE++ | 60 W |
| 802.3bt Type 4 | PoE++ / 4PPoE | 90–100 W |

Plan the switch's **total PoE budget**: 24 cameras at 12 W each need 288 W, more than many access switches supply.

## Termination and tools

Patch panels (punch down with a **110 block** tool), keystone jacks, **cable tester** (continuity, wiremap), **certifier** (proves the category performance), **toner and probe** to find a cable, **TDR/OTDR** to find the distance to a break (copper / fibre), **fibre light meter**, **visual fault locator** (red laser), **loopback plug**.

@@quiz
A 10 Gb/s copper run of 90 m is needed between a switch and a Wi-Fi 7 access point. Which cable category is the minimum?
- Cat 5e
- Cat 6
* Cat 6a
- Cat 8
? Cat 6 only does 10G to 55 m; Cat 8 is limited to 30 m.

@@quiz
Two buildings are 2 km apart. Which medium fits?
- Cat 6a
- Multimode fibre
* Single-mode fibre
- RG-6 coax
? Multimode reaches hundreds of metres; single-mode reaches kilometres.

@@quiz
A cable has T568A on one end and T568B on the other. What is it?
- Straight-through
* Crossover
- Rollover
- Shielded
? Auto-MDIX makes this mostly unnecessary today.

@@quiz
Cable must run through the air-handling space above a drop ceiling. What jacket is required?
* Plenum
- PVC
- Direct burial
- Riser
? Plenum jackets give off less toxic smoke in a fire.

@@quiz
A switch has a 370 W PoE budget. How many 30 W PoE+ access points can it fully power?
- 24
* 12
- 30
- 6
? 370 / 30 = 12.3, so 12.

@@flashcards
Cat 5e :: 1 Gb/s, 100 m
Cat 6 :: 10 Gb/s up to 55 m
Cat 6a :: 10 Gb/s, 100 m
Cat 8 :: 25/40 Gb/s, 30 m
plenum :: fire-rated jacket for air spaces
T568A + T568B :: crossover cable
single-mode :: yellow, laser, kilometres
multimode :: aqua/orange, hundreds of metres
LC / SC / ST :: small / square push-pull / bayonet
APC vs UPC :: green angled vs blue flat polish
SFP / SFP+ / SFP28 :: 1 / 10 / 25 Gb/s
QSFP28 :: 100 Gb/s
DAC :: direct attach copper cable
802.3af / at / bt :: 15.4 / 30 / 60-90 W
OTDR :: finds breaks in fibre
toner and probe :: traces a cable

@@drill
What is the maximum length of a copper Ethernet run, in metres?
= 100
? 90 m permanent link plus 10 m of patch cords.

@@drill
Which transceiver form factor carries 10 Gb/s?
= SFP+
= sfp+
? SFP is 1 Gb/s, SFP28 is 25 Gb/s.

@@exercise Pick the cable for each link
For each link, choose the cheapest medium from `MEDIA` (listed cheapest first) whose max speed and distance cover the link. A link marked `noisy` can't use unshielded copper (`shielded` False). Print the choice or `no single medium: add a switch or repeater`.
@@lang python
@@starter
MEDIA = [
    {"name": "Cat 5e UTP", "gbps": 1, "max_m": 100, "shielded": False, "fibre": False},
    {"name": "Cat 6 UTP", "gbps": 10, "max_m": 55, "shielded": False, "fibre": False},
    {"name": "Cat 6a UTP", "gbps": 10, "max_m": 100, "shielded": False, "fibre": False},
    {"name": "Cat 6a STP", "gbps": 10, "max_m": 100, "shielded": True, "fibre": False},
    {"name": "OM4 multimode", "gbps": 100, "max_m": 150, "shielded": True, "fibre": True},
    {"name": "OS2 single-mode", "gbps": 100, "max_m": 10000, "shielded": True, "fibre": True},
]
links = [
    ("Desk to closet", 1, 70, False),
    ("AP on ceiling", 10, 40, False),
    ("AP in warehouse", 10, 90, False),
    ("Factory floor camera", 1, 60, True),
    ("Rack to rack", 40, 120, False),
    ("Building A to B", 10, 1800, False),
    ("Remote office", 10, 25000, False),
]
@@expected
Desk to closet (1 Gb/s, 70 m): Cat 5e UTP
AP on ceiling (10 Gb/s, 40 m): Cat 6 UTP
AP in warehouse (10 Gb/s, 90 m): Cat 6a UTP
Factory floor camera (1 Gb/s, 60 m): Cat 6a STP
Rack to rack (40 Gb/s, 120 m): OM4 multimode
Building A to B (10 Gb/s, 1800 m): OS2 single-mode
Remote office (10 Gb/s, 25000 m): no single medium: add a switch or repeater
@@requires
for m in MEDIA
"shielded"
@@hint
`next((m for m in MEDIA if m["gbps"] >= speed and m["max_m"] >= dist and (m["shielded"] or not noisy)), None)`.
@@solution
MEDIA = [
    {"name": "Cat 5e UTP", "gbps": 1, "max_m": 100, "shielded": False, "fibre": False},
    {"name": "Cat 6 UTP", "gbps": 10, "max_m": 55, "shielded": False, "fibre": False},
    {"name": "Cat 6a UTP", "gbps": 10, "max_m": 100, "shielded": False, "fibre": False},
    {"name": "Cat 6a STP", "gbps": 10, "max_m": 100, "shielded": True, "fibre": False},
    {"name": "OM4 multimode", "gbps": 100, "max_m": 150, "shielded": True, "fibre": True},
    {"name": "OS2 single-mode", "gbps": 100, "max_m": 10000, "shielded": True, "fibre": True},
]
links = [
    ("Desk to closet", 1, 70, False),
    ("AP on ceiling", 10, 40, False),
    ("AP in warehouse", 10, 90, False),
    ("Factory floor camera", 1, 60, True),
    ("Rack to rack", 40, 120, False),
    ("Building A to B", 10, 1800, False),
    ("Remote office", 10, 25000, False),
]

for name, speed, dist, noisy in links:
    choice = next((m for m in MEDIA
                   if m["gbps"] >= speed and m["max_m"] >= dist and (m["shielded"] or not noisy)), None)
    if choice:
        print(f"{name} ({speed} Gb/s, {dist} m): {choice['name']}")
    else:
        print(f"{name} ({speed} Gb/s, {dist} m): no single medium: add a switch or repeater")

@@exercise PoE budget planner
Each device type needs a PoE class wattage. Check whether the switch's PoE budget covers all requested devices. If not, power devices one by one in priority order (lower number first), skipping any unit that would exceed the budget, and list how many of each type are left unpowered. Also flag any device whose wattage exceeds what its port standard provides (`PORT_MAX`).
@@lang python
@@starter
PORT_MAX = {"af": 15.4, "at": 30.0, "bt3": 60.0, "bt4": 90.0}
switch = {"name": "SW-3F", "budget_w": 300, "ports": "at"}
devices = [
    ("phone", 6.5, 3, 20),
    ("ap-wifi7", 38.0, 1, 4),
    ("camera", 12.9, 2, 6),
    ("badge-reader", 4.0, 1, 2),
]
@@expected
WARNING: ap-wifi7 needs 38.0 W but at ports give 30.0 W
Requested 367.4 W of 300 W
Powered: 295.9 W
Unpowered: 11 x phone
@@requires
PORT_MAX[
sorted(
@@hint
Expand to one entry per unit, sort by priority, then add wattages while `used + w <= budget`. Each tuple is `(type, watts, priority, count)`.
@@solution
PORT_MAX = {"af": 15.4, "at": 30.0, "bt3": 60.0, "bt4": 90.0}
switch = {"name": "SW-3F", "budget_w": 300, "ports": "at"}
devices = [
    ("phone", 6.5, 3, 20),
    ("ap-wifi7", 38.0, 1, 4),
    ("camera", 12.9, 2, 6),
    ("badge-reader", 4.0, 1, 2),
]

limit = PORT_MAX[switch["ports"]]
for kind, watts, _, _ in devices:
    if watts > limit:
        print(f"WARNING: {kind} needs {watts} W but {switch['ports']} ports give {limit} W")

total = sum(w * n for _, w, _, n in devices)
print(f"Requested {total:.1f} W of {switch['budget_w']} W")

used = 0.0
unpowered = {}
for kind, watts, prio, count in sorted(devices, key=lambda d: d[2]):
    for _ in range(count):
        if used + watts <= switch["budget_w"]:
            used += watts
        else:
            unpowered[kind] = unpowered.get(kind, 0) + 1
print(f"Powered: {used:.1f} W")
for kind, n in unpowered.items():
    print(f"Unpowered: {n} x {kind}")

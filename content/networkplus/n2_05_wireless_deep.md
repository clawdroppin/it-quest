@@lesson n2-wifideep Wi-Fi Standards, Channels, Security and Site Surveys
@@text
Wireless is where most user complaints come from. Network+ asks about 802.11 standards, frequency bands and channel planning, antenna types, enterprise security (WPA3, 802.1X/EAP), and how to read a site survey. This lesson goes deeper than the overview in the wireless/WAN lesson.

## 802.11 standards

| Standard | Wi-Fi name | Bands | Max theoretical rate | Key features |
|---|---|---|---|---|
| 802.11a | | 5 GHz | 54 Mb/s | OFDM |
| 802.11b | | 2.4 GHz | 11 Mb/s | |
| 802.11g | | 2.4 GHz | 54 Mb/s | |
| 802.11n | **Wi-Fi 4** | 2.4/5 GHz | 600 Mb/s | MIMO, 40 MHz channels |
| 802.11ac | **Wi-Fi 5** | 5 GHz | ~6.9 Gb/s | MU-MIMO (downlink), 80/160 MHz |
| 802.11ax | **Wi-Fi 6 / 6E** | 2.4/5 (+6 GHz for 6E) | ~9.6 Gb/s | **OFDMA**, uplink MU-MIMO, BSS colouring, target wake time |
| 802.11be | **Wi-Fi 7** | 2.4/5/6 GHz | ~46 Gb/s | 320 MHz channels, **multi-link operation (MLO)**, 4096-QAM |

Real-world throughput is roughly half the link rate or less, shared by everyone on the channel.

## Bands and channels

- **2.4 GHz**: longer range, better through walls, crowded (microwaves, Bluetooth). Only **3 non-overlapping 20 MHz channels: 1, 6, 11** (in most regions; Japan also allows 14 for 802.11b only).
- **5 GHz**: many more channels, less range. Some are **DFS** channels (shared with radar; the AP must move if radar is detected, which briefly drops clients).
- **6 GHz** (Wi-Fi 6E/7): lots of clean spectrum, needs WPA3, short range.
- **Channel width**: wider = faster but fewer non-overlapping channels and more interference. Use 20 MHz on 2.4 GHz and 20/40 MHz in dense 5 GHz deployments.
- **Co-channel interference**: neighbouring APs on the same channel take turns. **Adjacent-channel interference**: overlapping channels corrupt each other (worse).
- **Band steering** pushes capable clients to 5/6 GHz.
- **Regulatory domain** limits channels and transmit power per country.

## Signal terms

- **RSSI/signal strength** in dBm: −30 excellent, **−67 dBm** the usual minimum for voice and video, −70 OK for data, −80 unreliable, −90 unusable.
- **Noise floor** (e.g. −95 dBm) and **SNR** = signal − noise (25 dB+ is good; below 15 dB is poor).
- **EIRP** = transmit power + antenna gain − cable loss.
- Obstacles: concrete, metal, water (people!), mirrors, lift shafts. **Reflection, absorption, refraction, attenuation**.

## Antennas

- **Omnidirectional**: 360° coverage, ceiling-mounted APs.
- **Directional** (Yagi, patch/panel, parabolic): focused beam for point-to-point links between buildings or long corridors.
- dBi measures gain relative to an ideal isotropic antenna.

## Architectures

- **Autonomous APs**: each configured separately (small offices).
- **Controller-based (lightweight) APs**: a **wireless LAN controller (WLC)**, on-premises or cloud, manages config, channels, power and roaming. APs often tunnel traffic to it (CAPWAP).
- **Mesh**: APs relay over the air where cabling is hard.
- **Roaming**: clients move between APs with the same SSID; **802.11r** (fast roaming), **802.11k** (neighbour reports), **802.11v** (network-assisted roaming).
- **SSID** (network name), **BSSID** (each AP radio's MAC), **ESSID** (the SSID across many APs).
- **Ad hoc**, **infrastructure**, and **point-to-point** modes.

## Wireless security

| Mode | Use |
|---|---|
| Open + **captive portal** | guest Wi-Fi with terms or login; add **OWE (Enhanced Open)** for encryption without a password |
| **WPA2-Personal** (PSK, AES-CCMP) | small offices; one shared password |
| **WPA3-Personal** (SAE) | resistant to offline password guessing |
| **WPA2/WPA3-Enterprise** (802.1X) | each user authenticates to a **RADIUS** server |

**802.1X roles**: **supplicant** (the client), **authenticator** (AP/switch), **authentication server** (RADIUS). **EAP** methods: **EAP-TLS** (certificates on both sides, strongest), **PEAP** (username/password inside a TLS tunnel), **EAP-TTLS**.

Never use WEP or TKIP. Disable **WPS**. Watch for **rogue APs** and **evil twins** (wireless IDS/IPS in the controller).

## Site surveys

- **Predictive survey**: model the floor plan in software before installing.
- **Passive survey**: walk the site listening to APs (signal, noise, channels).
- **Active survey**: connected, measuring throughput, latency and roaming.
- Output: **heat maps**. Design for **capacity** (number of users and devices per AP, ~30–50 active clients) not just coverage, with 15–20% cell overlap for roaming.

@@quiz
Which three 2.4 GHz channels don't overlap?
- 1, 5, 9
* 1, 6, 11
- 2, 7, 12
- 1, 7, 13
? 20 MHz channels 5 apart.

@@quiz
Voice calls drop in a meeting room where the signal is −78 dBm. What's the problem?
* The signal is below the roughly −67 dBm needed for voice
- WPA3 is enabled
- The SSID is hidden
- DHCP is exhausted
? Add or move an AP.

@@quiz
Which 802.1X role does the wireless access point play?
- Supplicant
* Authenticator
- Authentication server
- Certificate authority
? The RADIUS server is the authentication server.

@@quiz
Which EAP method uses certificates on both client and server?
* EAP-TLS
- PEAP
- EAP-TTLS
- WPA2-PSK
? Strongest, but needs a PKI for client certificates.

@@quiz
Which Wi-Fi 6 feature lets an AP serve several clients in one transmission by dividing the channel into resource units?
- MIMO
* OFDMA
- DFS
- Band steering
? Orthogonal frequency-division multiple access.

@@flashcards
Wi-Fi 4 / 5 / 6 / 7 :: 802.11n / ac / ax / be
2.4 GHz non-overlapping :: 1, 6, 11
DFS :: 5 GHz channels shared with radar
6 GHz :: Wi-Fi 6E/7, requires WPA3
−67 dBm :: minimum for voice/video
SNR :: signal minus noise; 25 dB+ good
EIRP :: transmit power + gain − cable loss
omnidirectional :: 360° coverage
Yagi / parabolic :: directional antennas
WLC :: wireless LAN controller
BSSID :: an AP radio's MAC address
802.11r/k/v :: fast roaming, neighbour reports, assisted roaming
SAE :: WPA3-Personal handshake
supplicant / authenticator / server :: client / AP / RADIUS
EAP-TLS :: certificates both sides
PEAP :: password inside a TLS tunnel
OWE :: encrypted open Wi-Fi
predictive / passive / active survey :: model / listen / connect and test

@@drill
What is the SNR when the signal is −65 dBm and the noise floor is −92 dBm (in dB)?
= 27
? −65 − (−92).

@@drill
Which Wi-Fi generation is 802.11ax?
= Wi-Fi 6
= wi-fi 6
= 6
? Wi-Fi 6E adds the 6 GHz band.

@@exercise Plan 2.4 GHz channels
APs on one floor and their neighbours are given. Assign each AP one of channels 1, 6 and 11 so that neighbours differ where possible: process APs in order and choose the channel used by the fewest already-assigned neighbours (ties go to the lowest channel). Then report any neighbouring pairs left on the same channel.
@@lang python
@@starter
CHANNELS = [1, 6, 11]
neighbours = {
    "AP1": ["AP2", "AP3"],
    "AP2": ["AP1", "AP3", "AP4"],
    "AP3": ["AP1", "AP2", "AP4", "AP5"],
    "AP4": ["AP2", "AP3", "AP5"],
    "AP5": ["AP3", "AP4", "AP6"],
    "AP6": ["AP5"],
}
@@expected
AP1: channel 1
AP2: channel 6
AP3: channel 11
AP4: channel 1
AP5: channel 6
AP6: channel 1
no co-channel neighbours
@@requires
CHANNELS
min(
@@hint
For each AP, count `sum(1 for n in neighbours[ap] if assigned.get(n) == ch)` for every channel and pick `min(CHANNELS, key=...)`.
@@solution
CHANNELS = [1, 6, 11]
neighbours = {
    "AP1": ["AP2", "AP3"],
    "AP2": ["AP1", "AP3", "AP4"],
    "AP3": ["AP1", "AP2", "AP4", "AP5"],
    "AP4": ["AP2", "AP3", "AP5"],
    "AP5": ["AP3", "AP4", "AP6"],
    "AP6": ["AP5"],
}

assigned = {}
for ap in neighbours:
    assigned[ap] = min(CHANNELS, key=lambda ch: (sum(1 for n in neighbours[ap] if assigned.get(n) == ch), ch))
    print(f"{ap}: channel {assigned[ap]}")

clashes = sorted({tuple(sorted((a, b))) for a in neighbours for b in neighbours[a] if assigned[a] == assigned[b]})
if clashes:
    for a, b in clashes:
        print(f"co-channel: {a} and {b} on {assigned[a]}")
else:
    print("no co-channel neighbours")

@@exercise Grade a passive site survey
Survey points record the strongest AP signal (dBm) and the noise floor. Compute the SNR and grade each point: `voice-ready` if signal ≥ −67 and SNR ≥ 25, `data-ok` if signal ≥ −72 and SNR ≥ 20, otherwise `weak`. Print each point's grade, then the percentage of points that are voice-ready and the list of weak rooms.
@@lang python
@@starter
survey = [
    ("Lobby", -58, -95),
    ("Meeting A", -66, -94),
    ("Meeting B", -71, -93),
    ("Kitchen", -69, -85),
    ("Stairwell", -82, -96),
    ("Server room", -75, -90),
    ("Open office", -61, -95),
]
@@expected
Lobby: -58 dBm, SNR 37 dB -> voice-ready
Meeting A: -66 dBm, SNR 28 dB -> voice-ready
Meeting B: -71 dBm, SNR 22 dB -> data-ok
Kitchen: -69 dBm, SNR 16 dB -> weak
Stairwell: -82 dBm, SNR 14 dB -> weak
Server room: -75 dBm, SNR 15 dB -> weak
Open office: -61 dBm, SNR 34 dB -> voice-ready
Voice-ready: 43%
Weak: Kitchen, Stairwell, Server room
@@requires
snr
:.0%
@@hint
`snr = signal - noise`; check the voice-ready condition first.
@@solution
survey = [
    ("Lobby", -58, -95),
    ("Meeting A", -66, -94),
    ("Meeting B", -71, -93),
    ("Kitchen", -69, -85),
    ("Stairwell", -82, -96),
    ("Server room", -75, -90),
    ("Open office", -61, -95),
]

grades = {}
for room, signal, noise in survey:
    snr = signal - noise
    if signal >= -67 and snr >= 25:
        grade = "voice-ready"
    elif signal >= -72 and snr >= 20:
        grade = "data-ok"
    else:
        grade = "weak"
    grades[room] = grade
    print(f"{room}: {signal} dBm, SNR {snr} dB -> {grade}")

voice = sum(1 for g in grades.values() if g == "voice-ready") / len(grades)
print(f"Voice-ready: {voice:.0%}")
print("Weak:", ", ".join(r for r, g in grades.items() if g == "weak"))

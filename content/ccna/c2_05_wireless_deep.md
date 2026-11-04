@@lesson c2-wireless-deep Wireless Principles and WLC Configuration
@@text
CCNA covers wireless principles (RF, channels, SSIDs, encryption), Cisco wireless architectures and AP modes, and configuring a WLAN on a Wireless LAN Controller (WLC) through its GUI. This lesson connects the RF theory to what you click on the controller.

## RF basics

- **2.4 GHz**: longer range, better wall penetration, only **three non-overlapping 20 MHz channels: 1, 6, 11**, crowded (Bluetooth, microwaves).
- **5 GHz**: many more non-overlapping channels, wider channels possible (40/80/160 MHz), shorter range. Some channels need **DFS** (radar detection).
- **6 GHz** (Wi-Fi 6E/7): lots of clean spectrum, WPA3 required.
- Signal strength in **dBm**: −30 excellent, −67 good for voice/video, −70 OK for data, −80 poor. **SNR** (signal minus noise floor) ≥ 25 dB is good.
- Effects: **absorption** (walls, water, people), **reflection** (metal), **refraction**, **scattering**, **diffraction**, and **interference** (co-channel and adjacent-channel).

Standards: 802.11n (Wi-Fi 4), **ac** (Wi-Fi 5, 5 GHz), **ax** (Wi-Fi 6/6E, OFDMA, 2.4/5/6 GHz), **be** (Wi-Fi 7).

## Service sets

- **SSID**: the network name. **BSSID**: the AP radio's MAC address for that SSID.
- **BSS**: one AP and its clients. **ESS**: several APs with the same SSID, allowing **roaming** (15–20% cell overlap).
- **IBSS**: ad hoc, no AP. Also mesh, repeater, workgroup bridge, outdoor bridge.

## Architectures and AP modes

| Architecture | How it works |
|---|---|
| **Autonomous** | each AP configured individually |
| **Lightweight + WLC** (split-MAC) | APs handle real-time functions; the WLC handles management, authentication, roaming, RF; **CAPWAP** tunnels (UDP **5246** control, encrypted; **5247** data) |
| **FlexConnect** | lightweight APs at branches switch traffic locally and keep working if the WAN link to the WLC fails |
| **Cloud-managed** (Meraki) | managed from the cloud dashboard |

AP modes: **local** (default: serves clients, tunnels to WLC), **FlexConnect**, **monitor** (only sensing, rogue detection), **sniffer**, **rogue detector**, **SE-Connect** (spectrum analysis), **bridge/mesh**.

Switch ports: a **lightweight AP in local mode** connects to an **access port** (its management VLAN; client traffic goes inside CAPWAP). A **FlexConnect** or autonomous AP that bridges several VLANs locally needs a **trunk**. The **WLC** connects through a trunk, usually a **LAG** (EtherChannel in mode `on`: WLCs don't negotiate LACP/PAgP).

## WLC interfaces and ports

- **Ports**: physical (distribution system ports, service port for out-of-band management, console).
- **Interfaces** (logical): **management** (CAPWAP and admin access), **dynamic** interfaces (one per client VLAN, mapped to WLANs), **virtual** (used for DHCP relay/web auth, a non-routable address such as 192.0.2.1), **service port**.

## Configuring a WLAN on the WLC GUI

1. **WLANs → Create New**: profile name, **SSID**, WLAN ID.
2. **General**: status Enabled, **interface/interface group** = the dynamic interface (client VLAN), broadcast SSID.
3. **Security → Layer 2**: **WPA2/WPA3** with **AES (CCMP)**; auth key management **PSK** (enter the passphrase) or **802.1X** (Enterprise).
4. **Security → AAA Servers**: select the RADIUS server(s) for 802.1X (defined under Security → RADIUS → Authentication with the server IP, port 1812 and shared secret).
5. **QoS**: Platinum (voice), Gold (video), Silver (best effort, default), Bronze (background).
6. **Advanced**: FlexConnect local switching, session timeout, client band selection, DHCP required.
7. **Apply**, then save the configuration.

Management access to the WLC: HTTPS, SSH, TACACS+/RADIUS for admins, disable Telnet and HTTP.

## Wireless security recap

| Option | Authentication | Encryption |
|---|---|---|
| WPA2-Personal | PSK | AES-CCMP |
| WPA2-Enterprise | 802.1X / EAP / RADIUS | AES-CCMP |
| WPA3-Personal | SAE | AES (GCMP in some modes) |
| WPA3-Enterprise | 802.1X; optional 192-bit mode | AES-GCMP-256 |

@@quiz
Which three 2.4 GHz channels don't overlap?
* 1, 6, 11
- 1, 5, 9
- 2, 7, 12
- 36, 40, 44
? 20 MHz channels 5 MHz apart.

@@quiz
Which UDP ports does CAPWAP use?
- 1812 and 1813
* 5246 control and 5247 data
- 161 and 162
- 4789
? Control is encrypted with DTLS.

@@quiz
What kind of switch port does a lightweight AP in local mode normally connect to?
* An access port
- A trunk port
- A routed port
- A port-channel
? Client traffic is tunnelled to the WLC in CAPWAP.

@@quiz
Which AP mode lets branch APs keep switching traffic locally if the WAN link to the WLC fails?
- Local
- Monitor
* FlexConnect
- Sniffer
? Local switching and local authentication options.

@@quiz
On a WLC, which interface type maps a WLAN to a client VLAN?
- Management
- Virtual
* Dynamic
- Service port
? One dynamic interface per client VLAN.

@@quiz
What does WPA3-Personal use instead of a pre-shared key handshake?
- TKIP
- 802.1X
* SAE
- WEP
? Simultaneous Authentication of Equals resists offline dictionary attacks.

@@flashcards
2.4 GHz non-overlapping :: 1, 6, 11
DFS :: radar detection on some 5 GHz channels
−67 dBm :: target for voice/video
SNR :: signal minus noise floor
SSID / BSSID :: name / AP radio MAC
BSS / ESS :: one AP / many APs same SSID
roaming overlap :: 15–20%
split-MAC :: AP real-time, WLC management
CAPWAP :: UDP 5246/5247
local mode :: default, tunnels to WLC
FlexConnect :: local switching at branches
monitor mode :: sensing only
lightweight AP port :: access
WLC uplink :: trunk / LAG (mode on)
dynamic interface :: WLAN to VLAN
virtual interface :: DHCP relay / web auth
WLC QoS Platinum :: voice
WPA2-Enterprise :: 802.1X + RADIUS
WPA3-Personal :: SAE

@@drill
Which protocol tunnels traffic between lightweight APs and the WLC?
= CAPWAP
? Control and data channels.

@@drill
Which EtherChannel mode must a WLC LAG use on the switch?
= on
= mode on
? WLCs don't support LACP or PAgP.

@@exercise Plan 2.4 GHz channels
Assign channels 1, 6 and 11 to the APs so that no two neighbouring APs (in `neighbours`) share a channel if possible. Process APs in order of most neighbours first (ties by name); give each the lowest-numbered channel not used by an already-assigned neighbour, or, if all three are used, the channel shared with the fewest neighbours. Print each AP's channel, then any co-channel conflicts.
@@lang python
@@starter
CHANNELS = [1, 6, 11]
neighbours = {
    "AP-A": ["AP-B", "AP-C", "AP-D"],
    "AP-B": ["AP-A", "AP-C", "AP-E"],
    "AP-C": ["AP-A", "AP-B", "AP-D", "AP-E"],
    "AP-D": ["AP-A", "AP-C", "AP-E"],
    "AP-E": ["AP-B", "AP-C", "AP-D"],
}
@@expected
AP-A: channel 6
AP-B: channel 11
AP-C: channel 1
AP-D: channel 11
AP-E: channel 6
no co-channel neighbours
@@requires
CHANNELS
sorted(
@@hint
Order: `sorted(neighbours, key=lambda ap: (-len(neighbours[ap]), ap))`. For each AP, `used = [assigned[n] for n in neighbours[ap] if n in assigned]`; pick `min(CHANNELS, key=lambda ch: (used.count(ch), ch))`.
@@solution
CHANNELS = [1, 6, 11]
neighbours = {
    "AP-A": ["AP-B", "AP-C", "AP-D"],
    "AP-B": ["AP-A", "AP-C", "AP-E"],
    "AP-C": ["AP-A", "AP-B", "AP-D", "AP-E"],
    "AP-D": ["AP-A", "AP-C", "AP-E"],
    "AP-E": ["AP-B", "AP-C", "AP-D"],
}

assigned = {}
for ap in sorted(neighbours, key=lambda ap: (-len(neighbours[ap]), ap)):
    used = [assigned[n] for n in neighbours[ap] if n in assigned]
    assigned[ap] = min(CHANNELS, key=lambda ch: (used.count(ch), ch))

for ap in sorted(assigned):
    print(f"{ap}: channel {assigned[ap]}")

conflicts = sorted({tuple(sorted((a, b))) for a in neighbours for b in neighbours[a] if assigned[a] == assigned[b]})
if conflicts:
    for a, b in conflicts:
        print(f"co-channel: {a} and {b} on {assigned[a]}")
else:
    print("no co-channel neighbours")

@@exercise Validate WLAN profiles
Check each WLAN profile against the policy and print every problem: security must be WPA2 or WPA3 (never `open`, `wep` or `wpa`); cipher must be `aes` (no `tkip`); `psk` WLANs need a passphrase of at least 12 characters and must not be on the `corp` VLAN; `dot1x` WLANs need at least one RADIUS server; the VLAN must exist in `dynamic_interfaces`; voice WLANs need QoS `platinum`. Print `OK` for clean profiles.
@@lang python
@@starter
dynamic_interfaces = {"corp": 10, "guest": 30, "voice": 40}
wlans = [
    {"ssid": "Corp", "security": "wpa3", "cipher": "aes", "akm": "dot1x", "radius": ["10.0.50.11"], "vlan": "corp", "qos": "silver"},
    {"ssid": "Guest", "security": "wpa2", "cipher": "aes", "akm": "psk", "psk": "welcome1", "vlan": "guest", "qos": "bronze"},
    {"ssid": "Phones", "security": "wpa2", "cipher": "tkip", "akm": "dot1x", "radius": [], "vlan": "voice", "qos": "silver", "voice": True},
    {"ssid": "Legacy", "security": "wep", "cipher": "rc4", "akm": "psk", "psk": "abc", "vlan": "corp", "qos": "silver"},
    {"ssid": "IoT", "security": "wpa2", "cipher": "aes", "akm": "psk", "psk": "long-iot-passphrase-2026", "vlan": "iot", "qos": "silver"},
]
@@expected
Corp: OK
Guest: PSK shorter than 12 characters
Phones: cipher tkip (use AES); 802.1X without RADIUS server; voice WLAN QoS silver (use platinum)
Legacy: insecure security wep; cipher rc4 (use AES); PSK shorter than 12 characters; PSK WLAN on corp VLAN
IoT: no dynamic interface for VLAN iot
@@requires
def problems
.get(
@@hint
Collect messages in a list per WLAN. Use `w.get("voice")` and `w.get("psk", "")` for optional keys.
@@solution
dynamic_interfaces = {"corp": 10, "guest": 30, "voice": 40}
wlans = [
    {"ssid": "Corp", "security": "wpa3", "cipher": "aes", "akm": "dot1x", "radius": ["10.0.50.11"], "vlan": "corp", "qos": "silver"},
    {"ssid": "Guest", "security": "wpa2", "cipher": "aes", "akm": "psk", "psk": "welcome1", "vlan": "guest", "qos": "bronze"},
    {"ssid": "Phones", "security": "wpa2", "cipher": "tkip", "akm": "dot1x", "radius": [], "vlan": "voice", "qos": "silver", "voice": True},
    {"ssid": "Legacy", "security": "wep", "cipher": "rc4", "akm": "psk", "psk": "abc", "vlan": "corp", "qos": "silver"},
    {"ssid": "IoT", "security": "wpa2", "cipher": "aes", "akm": "psk", "psk": "long-iot-passphrase-2026", "vlan": "iot", "qos": "silver"},
]

def problems(w):
    found = []
    if w["security"] not in ("wpa2", "wpa3"):
        found.append(f"insecure security {w['security']}")
    if w["cipher"] != "aes":
        found.append(f"cipher {w['cipher']} (use AES)")
    if w["akm"] == "psk":
        if len(w.get("psk", "")) < 12:
            found.append("PSK shorter than 12 characters")
        if w["vlan"] == "corp":
            found.append("PSK WLAN on corp VLAN")
    if w["akm"] == "dot1x" and not w.get("radius"):
        found.append("802.1X without RADIUS server")
    if w["vlan"] not in dynamic_interfaces:
        found.append(f"no dynamic interface for VLAN {w['vlan']}")
    if w.get("voice") and w["qos"] != "platinum":
        found.append(f"voice WLAN QoS {w['qos']} (use platinum)")
    return found

for w in wlans:
    found = problems(w)
    print(f"{w['ssid']}: " + ("OK" if not found else "; ".join(found)))

@@lesson f0-devices Ports, Peripherals, Displays and Mobile Devices
@@text
A large share of help desk tickets aren't about the computer itself but about what's plugged into it: "my second monitor doesn't work", "the dock stopped charging my laptop", "my phone won't get email". This lesson is a tour of the connectors and devices you'll handle every week.

## Ports and cables

| Port | Used for | Notes |
|---|---|---|
| **USB-A** | keyboards, mice, flash drives | the classic rectangle |
| **USB-C** | almost everything now | the reversible oval; the *shape* says nothing about speed |
| **HDMI** | monitors, TVs, projectors | HDMI 2.0 does 4K at 60 Hz |
| **DisplayPort** | monitors | can daisy-chain monitors (MST); common on docks |
| **Ethernet (RJ45)** | wired network | 1 Gbps typical; 2.5 Gbps on newer machines |
| **3.5 mm jack** | headsets | combo jacks carry microphone and audio |
| **Thunderbolt 3/4/5** | docks, fast storage, eGPUs | uses the USB-C shape, marked with a ⚡ |

### USB-C is several things in one shape

A USB-C port might carry: USB data (from 480 Mbps to 40 Gbps), **video** (DisplayPort Alt Mode), **power** (USB Power Delivery, up to 100 W, or 240 W with newer cables), and Thunderbolt. A given port and cable support only some of these. The classic ticket, "the monitor doesn't work through this USB-C cable", is usually a charge-only or USB-2-only cable. Look for the icons next to the port (⚡ Thunderbolt, a D-shaped DisplayPort logo, a battery for charging) and use the cable that came with the dock.

### Speeds to remember

| Standard | Speed |
|---|---|
| USB 2.0 | 480 Mbps |
| USB 3.2 Gen 1 (old name USB 3.0) | 5 Gbps |
| USB 3.2 Gen 2 | 10 Gbps |
| USB4 / Thunderbolt 4 | 40 Gbps |
| Thunderbolt 5 | 80–120 Gbps |

## Docks

Hot-desking offices give each desk a **dock**: one USB-C/Thunderbolt cable to the laptop provides charging, two monitors, Ethernet, keyboard and mouse. Common dock problems: firmware out of date (update it with the vendor's tool), the laptop's charger wattage is higher than the dock provides ("slow charger" warnings), or the laptop's port doesn't support video.

## Displays

- **Resolution**: 1920×1080 (Full HD), 2560×1440 (QHD), 3840×2160 (4K/UHD).
- **Refresh rate**: 60 Hz for office work; 120–144 Hz is smoother.
- **Scaling**: on a 14" 4K laptop, Windows uses 200% scaling so text is readable. Mixing monitors with different scaling makes some old apps blurry.
- **Panel types**: IPS (good colours and angles, most office monitors), VA (better contrast), OLED (perfect blacks, risk of burn-in from static taskbars), TN (cheap, fast, poor colours).
- Troubleshooting order: is it powered on and on the right **input**? Is the cable seated at both ends? Try another cable or port. Then **Win+P** (projection mode) and Display settings → Detect. Then graphics drivers and dock firmware.

## Input and audio

- Bluetooth keyboards, mice and headsets need **pairing**; most problems come from pairing to several devices or from low batteries.
- Wireless dongles (2.4 GHz) are more reliable than Bluetooth in busy offices.
- Headset not working in Teams/Zoom: check the app's own device settings first, then Windows Sound settings → the right input/output device, then the privacy setting "Let apps access your microphone".

## Printers and scanners (overview)

- **Laser** printers for offices (fast, cheap per page); **inkjet** for photos; **thermal** for labels and receipts.
- Network printers usually print through a **print server** that holds the drivers and queues; "printing is stuck" is often a jammed queue: restart the **Print Spooler** service.
- Multifunction printers (MFPs) also scan, often to email or a network folder.

## Mobile devices

Staff phones and tablets need IT support too:

- **iOS/iPadOS** (Apple) and **Android** (Google, Samsung and others).
- **MDM** (Mobile Device Management, e.g. Microsoft Intune, Jamf) lets IT push email profiles, Wi-Fi settings and apps, require a PIN, and **remotely wipe** a lost device.
- **BYOD** (bring your own device): staff use their own phone. Usually managed with app protection policies or a **work profile** that separates company data from personal data, so IT can wipe only the company part.
- Email sync uses **Exchange ActiveSync** or a modern app (Outlook mobile); calendars and contacts sync too.
- Common tickets: email stops syncing after a password change (re-enter the password), the phone can't reach company Wi-Fi (certificate-based Wi-Fi needs the MDM profile), the authenticator app was lost with an old phone (identity verification, then re-registering MFA).

## Laptop care basics

- **Batteries** wear out: Windows can report the design capacity vs current capacity (`powercfg /batteryreport`). Below 70–80% of design capacity, replacement is reasonable. A **swollen** battery is a safety hazard: stop using the device and replace it.
- Heat: blocked vents and dust raise temperatures; the CPU **throttles** (slows itself down) to stay safe, which users notice as "slow".

@@quiz
A user's USB-C cable charges their laptop but the monitor connected through it stays black. Most likely cause?
* The cable or port doesn't support video (DisplayPort Alt Mode); it's a charge-only or USB 2 cable
- USB-C can't carry video
- The monitor needs a driver update
- The laptop battery is too low
? Same shape, different capabilities.

@@quiz
What does MDM let IT do with a lost company phone?
- Track the user's personal photos
* Lock or remotely wipe it (or only the company data)
- Nothing; phones are personal
- Reset the user's password automatically
? Remote wipe is a key MDM feature.

@@quiz
Several users can't print; the queue shows jobs stuck. What's a quick, common fix?
- Reinstall Windows
* Clear the queue and restart the Print Spooler service
- Replace the printer
- Change the paper size
? The spooler manages print jobs on Windows.

@@quiz
What is the theoretical speed of USB 3.2 Gen 1 (the old "USB 3.0")?
- 480 Mbps
* 5 Gbps
- 40 Gbps
- 1 Gbps
? USB 2.0 is 480 Mbps; Gen 2 is 10 Gbps.

@@flashcards
USB-C :: a connector shape; capabilities vary by port and cable
Thunderbolt 4 :: 40 Gbps over USB-C, marked ⚡
DisplayPort Alt Mode :: video over USB-C
USB PD :: USB Power Delivery charging
HDMI / DisplayPort :: monitor connections
dock :: one cable for power, displays, network and peripherals
Win+P :: projection / display mode
IPS / OLED :: common panel types
Print Spooler :: Windows service managing print jobs
MDM :: Mobile Device Management (Intune, Jamf)
BYOD :: staff use their own devices
work profile :: separates company data on a personal phone
throttling :: CPU slows down when hot
`powercfg /batteryreport` :: Windows battery health report

@@drill
What is the speed of USB 2.0 in Mbps?
= 480
? Many cheap USB-C cables are still USB 2.0 inside.

@@drill
Which Windows key combination opens the display projection options?
= Win+P
= win p
? Duplicate, Extend, Second screen only.

@@exercise Battery health report
Each laptop record has its design capacity and current full-charge capacity in mWh. Write `health(design, full)` returning the percentage (rounded to a whole number), and print each laptop's health with a status: `"replace"` under 70%, `"watch"` under 80%, otherwise `"ok"`. Then print how many laptops need replacing.
@@lang python
@@starter
laptops = [("LT-0042", 57000, 51300), ("LT-0057", 57000, 37620), ("LT-0063", 72000, 55440), ("LT-0071", 45000, 31050)]

def health(design, full):
    raise NotImplementedError

# print e.g. "LT-0042 90% ok"
@@expected
LT-0042 90% ok
LT-0057 66% replace
LT-0063 77% watch
LT-0071 69% replace
Laptops to replace: 2
@@requires
def health
round(
@@hint
`round(full / design * 100)`. Keep a counter for "replace".
@@solution
laptops = [("LT-0042", 57000, 51300), ("LT-0057", 57000, 37620), ("LT-0063", 72000, 55440), ("LT-0071", 45000, 31050)]

def health(design, full):
    return round(full / design * 100)

to_replace = 0
for tag, design, full in laptops:
    h = health(design, full)
    status = "replace" if h < 70 else "watch" if h < 80 else "ok"
    if status == "replace":
        to_replace += 1
    print(tag, f"{h}%", status)
print("Laptops to replace:", to_replace)

@@exercise Will the dock work?
A dock needs a laptop port that supports **video** and **power delivery**, and gives at most 90 W. Write `check(laptop)` that returns a list of problems for one laptop dictionary: `"no video over USB-C"` if `video` is false, `"no USB-C charging"` if `pd` is false, and `"charger needs N W, dock gives 90 W"` if `watts` is more than 90. Print each laptop's name and either `OK` or its problems joined with `"; "`.
@@lang python
@@starter
laptops = [
    {"name": "ThinkPad T14", "video": True, "pd": True, "watts": 65},
    {"name": "Budget 15", "video": False, "pd": True, "watts": 45},
    {"name": "Workstation 16", "video": True, "pd": True, "watts": 135},
    {"name": "Old Ultrabook", "video": False, "pd": False, "watts": 45},
]

def check(laptop):
    raise NotImplementedError
@@expected
ThinkPad T14: OK
Budget 15: no video over USB-C
Workstation 16: charger needs 135 W, dock gives 90 W
Old Ultrabook: no video over USB-C; no USB-C charging
@@requires
def check
"; ".join
@@hint
Start with `problems = []`, append for each failed rule, return the list. `result = "; ".join(problems) or "OK"`.
@@solution
laptops = [
    {"name": "ThinkPad T14", "video": True, "pd": True, "watts": 65},
    {"name": "Budget 15", "video": False, "pd": True, "watts": 45},
    {"name": "Workstation 16", "video": True, "pd": True, "watts": 135},
    {"name": "Old Ultrabook", "video": False, "pd": False, "watts": 45},
]

def check(laptop):
    problems = []
    if not laptop["video"]:
        problems.append("no video over USB-C")
    if not laptop["pd"]:
        problems.append("no USB-C charging")
    if laptop["watts"] > 90:
        problems.append(f"charger needs {laptop['watts']} W, dock gives 90 W")
    return problems

for laptop in laptops:
    result = "; ".join(check(laptop)) or "OK"
    print(f"{laptop['name']}: {result}")

@@lesson a1-mobile Laptops and Mobile Devices: Repair, Connectivity and Configuration
@@text
Core 1 domain 1 (mobile devices) is about 13% of the exam: replacing laptop parts, connecting accessories, and setting up phones and tablets for company use. These are daily tasks in support jobs where most staff have laptops and company phones.

## Replacing laptop components

Always: power off, **unplug the charger and disconnect or remove the battery**, ESD strap, the service manual from the vendor's site, a tray or magnetic mat with screws labelled by step (laptop screws differ in length; a long screw in the wrong hole can pierce the board).

| Part | Usually replaceable? | Notes |
|---|---|---|
| **Battery** | yes (internal in most modern laptops) | swollen = stop using, replace, recycle properly |
| **RAM** | sometimes (SO-DIMM) | often **soldered** in thin laptops |
| **Storage** | usually (M.2 or 2.5") | clone or reinstall the OS |
| **Keyboard** | yes, but sometimes riveted to the top case | |
| **Wireless card** | sometimes (M.2 2230) | antenna leads must be reconnected (main/aux) |
| **Display** | yes (LCD/OLED panel, digitiser on touch models) | inverter only on very old CCFL screens |
| **Touchpad, speakers, webcam, microphone** | yes | cables run through the hinges |
| **DC jack** | yes, often a separate cable | loose jack = intermittent charging |
| **System board** | yes, last resort | expensive; usually a warranty repair |

**Biometrics** (fingerprint readers) and **NFC** sensors appear in exam lists too.

## Display problems on laptops

- **No image but external monitor works**: the panel, its cable, or (old models) the inverter/backlight.
- **Very dim image visible with a torch**: backlight failure.
- **Flickering when the lid moves**: the display cable through the hinge.
- **Lines or dead pixels**: panel damage.
- **Touch input in the wrong place**: recalibrate the digitiser or update drivers.
- **Wi-Fi weak since a screen repair**: the antenna wires, which run around the display, weren't reconnected or were pinched.

## Accessories and connectivity

- **Docking stations** and **port replicators** (USB-C/Thunderbolt): the former adds power, displays and network; the latter just ports.
- **Bluetooth**: enable, make discoverable, **pair** (PIN if asked), test. Problems: too many paired devices, low battery, interference, outdated driver.
- **NFC**: very short range (a few centimetres), for payments, badge access, pairing.
- **Hotspot / tethering**: share a phone's mobile data over Wi-Fi, USB or Bluetooth.
- **Cellular**: 4G LTE, **5G**; SIM, **eSIM**; a laptop with a built-in WWAN modem.
- **Cables**: USB-C, Lightning (older Apple), micro-USB (older Android).
- **Stylus/pen**, **drawing pads**, **webcams**, **headsets**, **speakers**, **trackpads**.
- **Location services**: GPS (satellites) and cellular/Wi-Fi location; used by maps, find-my-device and MDM geofencing.

## Mobile device management and synchronisation

- **MDM / MAM** (Microsoft Intune, Jamf, Workspace ONE): enrol the device, push email/Wi-Fi/VPN profiles and certificates, require a PIN or biometrics and encryption, deploy apps, **remote wipe** or **selective wipe** (company data only).
- **BYOD** vs **COPE** (corporate-owned, personally enabled) vs fully corporate devices.
- **Synchronisation**: email, calendar and contacts with Exchange/Microsoft 365, Google Workspace or iCloud; files with OneDrive/iCloud Drive/Google Drive. Sync needs the right account, credentials (a password change breaks it until updated), a network connection, and enough storage.
- **Two-factor authentication** apps (Microsoft Authenticator, Google Authenticator) and **passkeys** live on the phone: losing it means account recovery procedures, so plan for it (backup methods, a second registered device).
- **Mobile VPN**: always-on or per-app VPN from the MDM profile.

## Setting up company email on a phone

1. Enrol the device in MDM (or install the company portal app).
2. Add the account: Outlook mobile or the built-in Mail app with Exchange; for generic IMAP/SMTP you'd need server names, ports (IMAP 993, SMTP 587), and SSL/TLS settings.
3. Approve MFA.
4. Confirm mail, calendar and contacts sync; check the company's conditional access isn't blocking an unmanaged app.

## Common mobile tickets

| Symptom | Usual cause / fix |
|---|---|
| Battery drains fast | a misbehaving app (check battery usage), screen brightness, old battery (battery health) |
| Overheating | heavy apps, charging while gaming, case blocking heat, faulty battery |
| Can't get email after password change | update the password in the account settings |
| App crashes | update the app/OS, clear cache, reinstall |
| No Wi-Fi to company network | missing certificate/profile from MDM, forgotten network, MAC randomisation blocked by network rules |
| Phone won't charge | lint in the port, damaged cable, wrong charger wattage |
| Screen doesn't rotate | rotation lock on |
| Bluetooth headset won't connect | forget and re-pair, check it's not connected to another device |

@@quiz
After replacing a laptop's screen, Wi-Fi signal is very weak. What's the likely cause?
* The Wi-Fi antenna wires, which run through the display assembly, weren't reconnected or were damaged
- The new screen uses a different driver
- The BIOS reset the Wi-Fi band
- The battery is low
? Antennas are in the lid.

@@quiz
A laptop screen is very dark, but you can faintly see the image with a torch. What has failed?
- The GPU
* The backlight
- The digitiser
- The hard drive
? The panel works; the light behind it doesn't.

@@quiz
A company wants to erase only company email and files from staff-owned phones when they leave. What feature does this?
- Factory reset
* Selective (enterprise) wipe through MDM/MAM
- Bluetooth pairing
- Disabling location services
? BYOD devices keep personal data.

@@quiz
Which technology has the shortest range and is used for tap-to-pay and badge readers?
* NFC
- Bluetooth
- Wi-Fi
- 5G
? A few centimetres.

@@flashcards
SO-DIMM :: laptop RAM module
M.2 2230 :: small slot for Wi-Fi cards and compact SSDs
digitiser :: touch layer of a touchscreen
backlight :: lights the LCD panel
docking station vs port replicator :: power + displays + network vs just ports
pairing :: linking Bluetooth devices
NFC :: very short range, tap-to-pay
tethering / hotspot :: share mobile data
eSIM :: embedded, downloadable SIM
MDM :: manage devices; MAM: manage apps/data
selective wipe :: removes only company data
COPE :: corporate-owned, personally enabled
IMAP 993 / SMTP 587 :: mail client ports
swollen battery :: stop using, replace safely

@@drill
What M.2 size is common for laptop Wi-Fi cards? (four digits)
= 2230
? 22 mm wide, 30 mm long.

@@drill
Which port does IMAP over SSL/TLS use?
= 993
? POP3S is 995.

@@exercise A repair checklist generator
Each repair type has its own steps, but every repair starts with the safety steps and ends with the testing steps. Write `checklist(part)` that returns the full numbered list for a part, and raises `KeyError` for unknown parts. Print the checklists for the Wi-Fi card and the display, and the error for "GPU".
@@lang python
@@starter
SAFETY = ["Back up the user's data", "Power off and unplug the charger", "Disconnect the internal battery", "Wear an ESD strap"]
TESTING = ["Reconnect the battery and power on", "Test the replaced part", "Ask the user to confirm"]
STEPS = {
    "wifi": ["Remove the bottom cover", "Disconnect the antenna leads (note main/aux)", "Remove the M.2 2230 card", "Fit the new card and reconnect the antennas"],
    "display": ["Remove the bezel", "Unscrew the panel", "Disconnect the display cable", "Fit the new panel and route the antennas correctly"],
}

def checklist(part):
    raise NotImplementedError
@@expected
== wifi
1. Back up the user's data
2. Power off and unplug the charger
3. Disconnect the internal battery
4. Wear an ESD strap
5. Remove the bottom cover
6. Disconnect the antenna leads (note main/aux)
7. Remove the M.2 2230 card
8. Fit the new card and reconnect the antennas
9. Reconnect the battery and power on
10. Test the replaced part
11. Ask the user to confirm
== display
1. Back up the user's data
2. Power off and unplug the charger
3. Disconnect the internal battery
4. Wear an ESD strap
5. Remove the bezel
6. Unscrew the panel
7. Disconnect the display cable
8. Fit the new panel and route the antennas correctly
9. Reconnect the battery and power on
10. Test the replaced part
11. Ask the user to confirm
== gpu
No procedure for gpu: check the service manual
@@requires
def checklist
enumerate(
@@hint
`steps = SAFETY + STEPS[part] + TESTING` (a missing part raises `KeyError` by itself), then `[f"{i}. {s}" for i, s in enumerate(steps, 1)]`.
@@solution
SAFETY = ["Back up the user's data", "Power off and unplug the charger", "Disconnect the internal battery", "Wear an ESD strap"]
TESTING = ["Reconnect the battery and power on", "Test the replaced part", "Ask the user to confirm"]
STEPS = {
    "wifi": ["Remove the bottom cover", "Disconnect the antenna leads (note main/aux)", "Remove the M.2 2230 card", "Fit the new card and reconnect the antennas"],
    "display": ["Remove the bezel", "Unscrew the panel", "Disconnect the display cable", "Fit the new panel and route the antennas correctly"],
}

def checklist(part):
    steps = SAFETY + STEPS[part] + TESTING
    return [f"{i}. {step}" for i, step in enumerate(steps, 1)]

for part in ["wifi", "display", "gpu"]:
    print(f"== {part}")
    try:
        print("\n".join(checklist(part)))
    except KeyError:
        print(f"No procedure for {part}: check the service manual")

@@exercise Battery drain detective
A phone's battery usage report lists apps with their share of battery and minutes active in the background. Flag apps that use more than 15% of the battery, **or** that ran more than 60 minutes in the background while using over 5%. Print the flagged apps with the reason, most battery first, then the recommendation: if the battery's maximum capacity is under 80%, recommend a battery replacement, otherwise "restrict background activity for the flagged apps".
@@lang python
@@starter
usage = [
    {"app": "Teams", "pct": 22, "bg_min": 35},
    {"app": "Maps", "pct": 9, "bg_min": 140},
    {"app": "Outlook", "pct": 7, "bg_min": 20},
    {"app": "Game", "pct": 18, "bg_min": 0},
    {"app": "Weather", "pct": 4, "bg_min": 300},
]
max_capacity = 86
@@expected
Teams: 22% of battery
Game: 18% of battery
Maps: 140 min in background
Recommendation: restrict background activity for the flagged apps
@@requires
sorted(
"bg_min"
@@hint
Sort with `key=lambda u: u["pct"], reverse=True`. Collect reasons in a list per app.
@@solution
usage = [
    {"app": "Teams", "pct": 22, "bg_min": 35},
    {"app": "Maps", "pct": 9, "bg_min": 140},
    {"app": "Outlook", "pct": 7, "bg_min": 20},
    {"app": "Game", "pct": 18, "bg_min": 0},
    {"app": "Weather", "pct": 4, "bg_min": 300},
]
max_capacity = 86

for u in sorted(usage, key=lambda u: u["pct"], reverse=True):
    reasons = []
    if u["pct"] > 15:
        reasons.append(f"{u['pct']}% of battery")
    if u["bg_min"] > 60 and u["pct"] > 5:
        reasons.append(f"{u['bg_min']} min in background")
    if reasons:
        print(f"{u['app']}: {', '.join(reasons)}")

if max_capacity < 80:
    print("Recommendation: replace the battery")
else:
    print("Recommendation: restrict background activity for the flagged apps")

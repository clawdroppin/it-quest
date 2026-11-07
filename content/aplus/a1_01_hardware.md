@@lesson a1-hardware PC and Laptop Hardware
@@text
## Memory (RAM)

| Type | Notes |
|---|---|
| **DIMM** | desktop memory sticks |
| **SO-DIMM** | smaller sticks for laptops and mini PCs |
| **DDR4** / **DDR5** | both 288 pins, but the notch is in a different place: they can't be swapped |
| **ECC** | detects and corrects memory errors; used in servers |

Install sticks in matching pairs in the right slots (see the motherboard manual) to get **dual-channel** speed. A PC that beeps and won't show anything after a RAM upgrade usually has a stick not fully seated.

## Storage interfaces

- **SATA**: 2.5"/3.5" drives, up to 6 Gb/s. SSDs and HDDs.
- **M.2**: a slot form factor. An M.2 drive can be **SATA** or **NVMe** (PCIe): check which the slot supports, and the **key** (M-key for NVMe, B+M for SATA).
- **NVMe**: talks over PCIe lanes, several GB/s.
- HDD speeds: 5,400 and 7,200 RPM (desktop), 10k/15k (old servers).

## CPU, motherboard, power

- CPUs fit a **socket**: Intel uses **LGA** (pins on the board), AMD's AM5 is also LGA, older AMD AM4 is **PGA** (pins on the CPU). Always use **thermal paste** between CPU and cooler.
- Form factors: **ATX** (standard), **microATX** (smaller), **mini-ITX** (tiny).
- Expansion slots: **PCIe** x1, x4, x16 (graphics cards use x16).
- **PSU**: rated in watts; connectors are the 24-pin motherboard, 8-pin (4+4) CPU, PCIe 6/8-pin or 12VHPWR for GPUs, SATA power. **Modular** PSUs let you attach only the cables you need. A PSU's 80 PLUS rating describes its efficiency.
- **Firmware** settings (UEFI): boot order, Secure Boot, TPM, virtualization support (Intel VT-x / AMD-V, needed for VMs), fan curves, a supervisor password.

## Ports and cables

| Connector | Use |
|---|---|
| **USB-C** | reversible; data, video and power (**USB PD**) |
| **Thunderbolt 3/4** | uses USB-C plugs; 40 Gb/s, docks, external GPUs |
| **USB 2.0 / 3.x** | 480 Mb/s / 5–20 Gb/s (blue ports are usually 3.x) |
| **HDMI** | TVs and monitors, carries audio |
| **DisplayPort** | monitors, daisy-chaining; **VGA** (analog, blue, 15 pins) and **DVI** are legacy |
| **RJ45** | Ethernet; **RJ11** is the smaller phone/DSL plug |

## Laptops and mobile devices

- Laptop parts commonly replaced: **battery**, RAM (if not soldered), SSD, keyboard, screen, Wi-Fi card. Disconnect the battery before opening, and use the right screwdrivers and plastic pry tools.
- A **swollen battery** is a fire risk: stop using it, don't puncture it, and recycle it properly.
- Screens: **LCD** (IPS has the best colours and angles, TN is fast and cheap, VA has deep blacks), **OLED** (each pixel lights itself, true blacks). A dim but visible image on an old LCD = backlight/inverter failure.
- Wireless: Wi-Fi, **Bluetooth** (pair a device: enable, discover, enter/confirm a PIN, test), **NFC** (tap to pay), cellular.
- Company phones are managed with **MDM** (Intune, Jamf): push settings and apps, require a PIN, remote wipe a lost device. **BYOD** means staff's own devices.

@@quiz
A user upgraded their desktop from DDR4 to new DDR5 memory, but the DDR5 sticks won't fit. Why?
* DDR4 and DDR5 have different notch positions and the motherboard only supports one type
- They need more force
- DDR5 is only for laptops
- The BIOS must be updated first to make them fit
? Memory generations are keyed differently on purpose. The motherboard decides which type you can use.

@@quiz
A technician wants to install an M.2 NVMe SSD, but the drive isn't detected. What should they check?
* Whether the M.2 slot supports NVMe/PCIe and not only SATA
- Whether the SATA power cable is connected
- Whether the drive is 7,200 RPM
- Whether the drive is formatted FAT32
? M.2 is a shape; the slot might only support SATA drives, and M.2 drives don't use SATA power cables.

@@quiz
Which connector carries data, video and up to 240 W of power over one reversible cable?
* USB-C with USB Power Delivery
- HDMI
- VGA
- RJ45
? USB-C with USB PD (and Thunderbolt) powers laptops and docks.

@@quiz
A laptop's screen is very dim, and you can just see the image with a flashlight. What has most likely failed?
* The backlight
- The GPU
- The OS display driver
- The digitizer
? The LCD panel still draws the image; the light behind it is gone.

@@quiz
Which BIOS/UEFI setting must be enabled to run 64-bit virtual machines?
* Hardware virtualization (Intel VT-x or AMD-V)
- Secure Boot
- Fast Boot
- Legacy USB support
? Hypervisors need CPU virtualization extensions.

@@quiz
A user's phone is lost with company email on it. What lets IT protect the data?
* MDM remote wipe
- Changing the user's desk phone
- Bluetooth pairing
- Disabling NFC
? Mobile Device Management can lock or wipe managed devices.

@@flashcards
SO-DIMM :: Laptop memory module
ECC RAM :: Corrects memory errors; for servers
dual-channel :: Matched sticks in paired slots for more bandwidth
M.2 :: Small SSD slot; SATA or NVMe depending on drive and slot
NVMe :: SSD over PCIe; much faster than SATA
LGA / PGA :: Pins on the motherboard / on the CPU
ATX / microATX / mini-ITX :: Motherboard sizes from large to small
PCIe x16 :: Slot for graphics cards
Thunderbolt :: 40 Gb/s over USB-C connectors
DisplayPort :: Monitor connector; supports daisy-chaining
RJ45 / RJ11 :: Ethernet / phone line connector
MDM :: Mobile device management: policies, apps, remote wipe
OLED :: Each pixel emits its own light; no backlight

@@drill
How many pins does a DDR5 desktop DIMM have?
= 288
? Same count as DDR4, different notch.

@@exercise Check a RAM upgrade
A user wants more memory. For each PC, decide whether the requested sticks can be installed. Write `check(pc, sticks)` returning a list of problems (empty means OK):

- every stick's type must match the board (`"DDR4"` or `"DDR5"`): `wrong type: DDR5 in a DDR4 board`
- there must be enough slots: `needs 4 slots, board has 2`
- the total must not exceed the board's maximum: `64 GB is over the 32 GB maximum`

Print `OK` or the problems joined with `; `. Then print the total GB for the PCs that pass.
@@lang python
@@starter
pcs = [
    {"name": "PC-101", "type": "DDR4", "slots": 4, "max_gb": 64},
    {"name": "PC-102", "type": "DDR4", "slots": 2, "max_gb": 32},
    {"name": "LT-201", "type": "DDR5", "slots": 2, "max_gb": 64},
]
requests = {
    "PC-101": [("DDR4", 16), ("DDR4", 16)],
    "PC-102": [("DDR4", 16), ("DDR4", 16), ("DDR4", 16), ("DDR4", 16)],
    "LT-201": [("DDR4", 32), ("DDR5", 32)],
}

def check(pc, sticks):
    raise NotImplementedError


passed = 0
for pc in pcs:
    sticks = requests[pc["name"]]
    problems = check(pc, sticks)
    print(f"{pc['name']}: {'OK' if not problems else '; '.join(problems)}")
    if not problems:
        passed += sum(size for _kind, size in sticks)
print(f"Total installed in passing PCs: {passed} GB")
@@expected
PC-101: OK
PC-102: needs 4 slots, board has 2; 64 GB is over the 32 GB maximum
LT-201: wrong type: DDR4 in a DDR5 board
Total installed in passing PCs: 32 GB
@@requires
def check
sum(
.append(
@@hint
Collect problems in a list instead of returning at the first one, so the technician sees everything at once.
@@solution
pcs = [
    {"name": "PC-101", "type": "DDR4", "slots": 4, "max_gb": 64},
    {"name": "PC-102", "type": "DDR4", "slots": 2, "max_gb": 32},
    {"name": "LT-201", "type": "DDR5", "slots": 2, "max_gb": 64},
]
requests = {
    "PC-101": [("DDR4", 16), ("DDR4", 16)],
    "PC-102": [("DDR4", 16), ("DDR4", 16), ("DDR4", 16), ("DDR4", 16)],
    "LT-201": [("DDR4", 32), ("DDR5", 32)],
}

def check(pc, sticks):
    problems = []
    for kind, _size in sticks:
        if kind != pc["type"]:
            problems.append(f"wrong type: {kind} in a {pc['type']} board")
    if len(sticks) > pc["slots"]:
        problems.append(f"needs {len(sticks)} slots, board has {pc['slots']}")
    total = sum(size for _kind, size in sticks)
    if total > pc["max_gb"]:
        problems.append(f"{total} GB is over the {pc['max_gb']} GB maximum")
    return problems

passed = 0
for pc in pcs:
    sticks = requests[pc["name"]]
    problems = check(pc, sticks)
    print(f"{pc['name']}: {'OK' if not problems else '; '.join(problems)}")
    if not problems:
        passed += sum(size for _kind, size in sticks)
print(f"Total installed in passing PCs: {passed} GB")

@@exercise Will this SSD fit the laptop?
Laptops take storage in different shapes. Write `fits(slot, drive)` that returns `(True, "")` or `(False, reason)`:

- a `2.5"` bay takes 2.5" SATA drives only (check the drive's `height_mm` is not more than the bay's);
- an `M.2` slot needs the drive to be M.2, the same **key** (`M` for NVMe ×4, `B+M` drives fit M-key and B-key slots), a **length** the slot supports (2230, 2242, 2280 …), and the right **protocol**: a SATA-only M.2 slot can't run an NVMe drive.

Print every drive against every slot of the laptop.
@@lang python
@@starter
slots = [
    {"name": "M.2 slot 1", "form": "M.2", "key": "M", "lengths": {2242, 2280}, "protocols": {"NVMe", "SATA"}},
    {"name": "M.2 slot 2", "form": "M.2", "key": "B", "lengths": {2242}, "protocols": {"SATA"}},
    {"name": "2.5in bay", "form": "2.5", "height_mm": 7},
]
drives = [
    {"name": "NVMe 2280 1TB", "form": "M.2", "key": "M", "length": 2280, "protocol": "NVMe"},
    {"name": "SATA M.2 2242", "form": "M.2", "key": "B+M", "length": 2242, "protocol": "SATA"},
    {"name": "NVMe 2230", "form": "M.2", "key": "M", "length": 2230, "protocol": "NVMe"},
    {"name": "2.5in SATA SSD 7mm", "form": "2.5", "height_mm": 7},
    {"name": "2.5in HDD 9.5mm", "form": "2.5", "height_mm": 9.5},
]

def fits(slot, drive):
    raise NotImplementedError


for d in drives:
    results = []
    for s in slots:
        ok, why = fits(s, d)
        results.append(f"{s['name']}: " + ("fits" if ok else why))
    print(f"{d['name']}: " + "; ".join(results))
@@expected
NVMe 2280 1TB: M.2 slot 1: fits; M.2 slot 2: M-key drive in a B-key slot; 2.5in bay: needs a M.2 slot
SATA M.2 2242: M.2 slot 1: fits; M.2 slot 2: fits; 2.5in bay: needs a M.2 slot
NVMe 2230: M.2 slot 1: length 2230 not supported; M.2 slot 2: M-key drive in a B-key slot; 2.5in bay: needs a M.2 slot
2.5in SATA SSD 7mm: M.2 slot 1: needs a 2.5 slot; M.2 slot 2: needs a 2.5 slot; 2.5in bay: fits
2.5in HDD 9.5mm: M.2 slot 1: needs a 2.5 slot; M.2 slot 2: needs a 2.5 slot; 2.5in bay: 9.5 mm drive in a 7 mm bay
@@requires
def fits
.split("+")
@@hint
Check in order: form factor, then (for M.2) key, length and protocol. A `B+M` drive has both notches, so split the key on `+` and see whether the slot's key is among them.
@@solution
slots = [
    {"name": "M.2 slot 1", "form": "M.2", "key": "M", "lengths": {2242, 2280}, "protocols": {"NVMe", "SATA"}},
    {"name": "M.2 slot 2", "form": "M.2", "key": "B", "lengths": {2242}, "protocols": {"SATA"}},
    {"name": "2.5in bay", "form": "2.5", "height_mm": 7},
]
drives = [
    {"name": "NVMe 2280 1TB", "form": "M.2", "key": "M", "length": 2280, "protocol": "NVMe"},
    {"name": "SATA M.2 2242", "form": "M.2", "key": "B+M", "length": 2242, "protocol": "SATA"},
    {"name": "NVMe 2230", "form": "M.2", "key": "M", "length": 2230, "protocol": "NVMe"},
    {"name": "2.5in SATA SSD 7mm", "form": "2.5", "height_mm": 7},
    {"name": "2.5in HDD 9.5mm", "form": "2.5", "height_mm": 9.5},
]

def fits(slot, drive):
    if slot["form"] != drive["form"]:
        return False, f"needs a {drive['form']} slot"
    if slot["form"] == "2.5":
        if drive["height_mm"] > slot["height_mm"]:
            return False, f"{drive['height_mm']} mm drive in a {slot['height_mm']} mm bay"
        return True, ""
    if slot["key"] not in drive["key"].split("+"):
        return False, f"{drive['key']}-key drive in a {slot['key']}-key slot"
    if drive["length"] not in slot["lengths"]:
        return False, f"length {drive['length']} not supported"
    if drive["protocol"] not in slot["protocols"]:
        return False, f"slot can't run {drive['protocol']}"
    return True, ""

for d in drives:
    results = []
    for s in slots:
        ok, why = fits(s, d)
        results.append(f"{s['name']}: " + ("fits" if ok else why))
    print(f"{d['name']}: " + "; ".join(results))

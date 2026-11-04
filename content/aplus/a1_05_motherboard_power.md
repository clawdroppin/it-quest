@@lesson a1-motherboard Motherboards, CPUs, Firmware, Power and Cooling
@@text
Core 1 objective 3.x expects you to pick and install the right motherboard, CPU, power supply and cooling for a scenario, and to configure firmware settings. Exam questions read like real tickets: "a user wants to add a second graphics card", "a new PC won't boot after a CPU upgrade", "the BIOS shows the wrong time every morning".

## Motherboard form factors

| Form factor | Size | Expansion slots | Typical use |
|---|---|---|---|
| **ATX** | 305 × 244 mm | up to 7 | standard desktops, gaming |
| **microATX (mATX)** | 244 × 244 mm | up to 4 | office desktops |
| **Mini-ITX** | 170 × 170 mm | 1 | small form factor, HTPCs |

A smaller board fits in a larger case (mounting holes line up), not the other way round. Laptops use proprietary boards.

### What's on the board

- **CPU socket**: Intel uses **LGA** (pins on the board, e.g. LGA 1700/1851), AMD desktop uses **AM5** (LGA too) and older **AM4** (PGA: pins on the CPU). The socket and **chipset** must match the CPU generation; check the board's CPU support list and whether a **BIOS update** is needed first.
- **RAM slots**: DIMM, 2 or 4; check DDR4 vs DDR5 and which slots to use for dual channel (usually A2 and B2; the manual says).
- **Expansion slots**: **PCIe** x1, x4, x8, x16. A x16 slot is used for graphics cards; a smaller card fits a bigger slot. PCIe versions (3.0, 4.0, 5.0) roughly double bandwidth each generation and are backwards compatible.
- **M.2** slots for NVMe or SATA SSDs (check which the slot supports: M key vs B key).
- **SATA** ports for drives; using some M.2 slots can disable some SATA ports (shared lanes; read the manual).
- **Headers**: front-panel (power button, reset, LEDs), USB, audio, fan headers (CPU_FAN must have the CPU cooler connected or the board may refuse to boot), RGB, TPM.
- **CMOS battery** (CR2032 coin cell): keeps the firmware settings and clock when unplugged. A dead battery causes **wrong time and date at every boot** and lost settings.

## Firmware: BIOS and UEFI

**UEFI** has replaced legacy **BIOS** but people still say "BIOS". Enter it with a key at power-on (Del, F2, F10, F12 for the boot menu) or from Windows (Settings → Recovery → Advanced startup → UEFI Firmware Settings).

Settings you must know:

- **Boot order**, and boot mode **UEFI vs Legacy/CSM**. UEFI requires **GPT** disks; legacy uses **MBR**.
- **Secure Boot**: only boots signed bootloaders, blocking bootkits. Required for Windows 11.
- **TPM** (Trusted Platform Module, version 2.0 for Windows 11): a chip (or firmware TPM: Intel PTT, AMD fTPM) that stores encryption keys, used by **BitLocker** and Windows Hello.
- **Virtualization** support (Intel VT-x, AMD-V) for Hyper-V, WSL2 and VMs: often disabled by default.
- **Supervisor/administrator password** to lock the firmware settings; **boot password** before any boot.
- **Fan curves**, XMP/EXPO (memory overclocking profiles), enabling/disabling ports.
- **Firmware updates** (flashing): use the vendor's tool, keep power stable; a failed flash can brick the board (many have a backup BIOS or "flashback" button).

## CPUs for the exam

- Architecture: **x86/x64** vs **ARM**.
- **Cores**, **threads** (hyper-threading/SMT), **cache**, clock speed.
- **Multisocket** servers have two or more CPUs.
- **Virtualization extensions** must be enabled for VMs.
- Installation: match the socket notch/triangle marker, **zero insertion force** lever, never force it; apply **thermal paste** (a pea-sized dot) between the CPU and cooler.

## Power supplies

- **Wattage**: add up the components' needs and leave 20–30% headroom. A modern GPU may need 200–450 W alone.
- **Rails and voltages**: +12 V (CPU, GPU, drives' motors), +5 V and +3.3 V (logic, older devices).
- **Connectors**: 24-pin ATX main, 4/8-pin CPU (EPS), 6/8-pin PCIe (and the newer 12V-2x6/12VHPWR for high-end GPUs), SATA power, Molex (legacy).
- **Efficiency ratings**: 80 PLUS Bronze, Silver, Gold, Platinum, Titanium.
- **Modular** PSUs let you attach only the cables you need.
- **Redundant** PSUs in servers: two hot-swappable units; one can fail without downtime.
- Input: 110–120 V (US, Japan's 100 V) or 220–240 V (Europe). Modern PSUs auto-switch; old ones have a switch.
- **Never open a PSU**: capacitors hold a dangerous charge even when unplugged. Replace it as a unit.

## Cooling

- **Air**: heat sink + fan; **liquid** (AIO closed loops); passive (fanless) for low-power devices.
- **Thermal paste** or pads transfer heat; old dried paste causes overheating.
- **Case airflow**: front/bottom intake, rear/top exhaust.
- Overheating symptoms: random shutdowns, throttling (slow), loud fans, crashes under load. Check temperatures in the firmware or with monitoring tools, clean dust with compressed air (hold the fans still), and check fans spin.

## Building and upgrading: an order that avoids trouble

1. ESD strap, work on a clean, non-carpeted surface.
2. CPU, cooler and RAM onto the board **before** it goes in the case.
3. Standoffs where the board's holes are (a missing standoff, or an extra one, can short the board).
4. Board, then PSU, then drives and expansion cards.
5. Front-panel headers, power cables (24-pin and CPU 8-pin are the commonly forgotten ones).
6. First boot into firmware: check the CPU, RAM and drives are detected, set boot mode and order.

@@quiz
A PC loses its time and date every time it's unplugged overnight. What should you replace?
* The CMOS battery
- The power supply
- The CPU
- The hard drive
? A CR2032 coin cell keeps the firmware clock running.

@@quiz
A user wants Windows 11 but the upgrade check says the PC doesn't meet requirements, although it has a supported CPU. Which firmware settings are most likely disabled?
* TPM 2.0 (firmware TPM) and Secure Boot
- XMP and fan curves
- Virtualization and USB boot
- The boot password
? Windows 11 requires TPM 2.0 and Secure Boot capability.

@@quiz
A new graphics card needs 250 W, the rest of the system 300 W. Which PSU is the sensible choice?
- 450 W
- 550 W
* 750 W
- 300 W
? 550 W total plus 20–30% headroom ≈ 660–715 W.

@@quiz
Hyper-V won't install: "virtualization is not enabled". Where do you fix it?
- Device Manager
* In the UEFI/BIOS: enable Intel VT-x or AMD-V
- Windows Update
- The PSU switch
? Virtualization extensions are often off by default.

@@quiz
Why should you never open a power supply to repair it?
- It voids the motherboard warranty
* Its capacitors can hold a dangerous charge even when unplugged
- It contains thermal paste
- It's glued shut
? Replace PSUs as a whole unit.

@@flashcards
ATX / mATX / Mini-ITX :: 7 / 4 / 1 expansion slots
LGA vs PGA :: pins in the socket vs pins on the CPU
PCIe x16 :: graphics card slot
M.2 :: slot for NVMe/SATA SSDs
CMOS battery :: CR2032; keeps clock and settings
UEFI :: modern firmware, supports GPT and Secure Boot
Secure Boot :: only signed bootloaders run
TPM 2.0 :: key storage chip; BitLocker, Windows 11
VT-x / AMD-V :: CPU virtualization extensions
80 PLUS :: PSU efficiency ratings
modular PSU :: detachable cables
redundant PSU :: two hot-swap PSUs in servers
thermal paste :: heat transfer between CPU and cooler
standoffs :: spacers between the board and the case
24-pin + 8-pin :: main and CPU power connectors

@@drill
What coin-cell battery model is used for CMOS on most motherboards?
= CR2032
? 3 V lithium.

@@drill
Which TPM version does Windows 11 require?
= 2.0
= TPM 2.0
? Firmware TPM (PTT/fTPM) counts.

@@exercise Size a power supply
Add up each build's component wattages, add 25% headroom, and round **up** to the next standard PSU size from `SIZES`. Print the total draw, the recommended size, and `"needs PCIe power"` if a GPU over 75 W is present (a PCIe x16 slot supplies at most 75 W).
@@lang python
@@starter
import math

SIZES = [450, 550, 650, 750, 850, 1000, 1200]
builds = {
    "Office PC": {"cpu": 65, "board": 50, "ram": 10, "ssd": 8},
    "CAD workstation": {"cpu": 125, "board": 60, "ram": 20, "ssd": 16, "gpu": 220},
    "Gaming PC": {"cpu": 170, "board": 70, "ram": 20, "ssd": 10, "gpu": 450, "fans": 15},
}

def recommend(total):
    raise NotImplementedError
@@expected
Office PC: 133 W draw -> 450 W PSU
CAD workstation: 441 W draw -> 650 W PSU  needs PCIe power
Gaming PC: 735 W draw -> 1000 W PSU  needs PCIe power
@@requires
def recommend
* 1.25
@@hint
`needed = total * 1.25`, then `return next(size for size in SIZES if size >= needed)`.
@@solution
SIZES = [450, 550, 650, 750, 850, 1000, 1200]
builds = {
    "Office PC": {"cpu": 65, "board": 50, "ram": 10, "ssd": 8},
    "CAD workstation": {"cpu": 125, "board": 60, "ram": 20, "ssd": 16, "gpu": 220},
    "Gaming PC": {"cpu": 170, "board": 70, "ram": 20, "ssd": 10, "gpu": 450, "fans": 15},
}

def recommend(total):
    needed = total * 1.25
    return next(size for size in SIZES if size >= needed)

for name, parts in builds.items():
    total = sum(parts.values())
    note = "  needs PCIe power" if parts.get("gpu", 0) > 75 else ""
    print(f"{name}: {total} W draw -> {recommend(total)} W PSU{note}")

@@exercise Check a CPU upgrade's compatibility
For each upgrade request, check the CPU against the motherboard: the sockets must match; the CPU's generation must be in the board's supported list, otherwise if it's in `needs_bios_update` the answer is `"BIOS update first"`; the board's RAM type must be one the CPU supports. Print `"OK"`, `"BIOS update first"`, or the problems found.
@@lang python
@@starter
boards = {
    "B650M": {"socket": "AM5", "gens": ["7000", "8000"], "needs_bios_update": ["9000"], "ram": "DDR5"},
    "B550": {"socket": "AM4", "gens": ["3000", "5000"], "needs_bios_update": [], "ram": "DDR4"},
    "Z790": {"socket": "LGA1700", "gens": ["13th", "14th"], "needs_bios_update": [], "ram": "DDR5"},
}
cpus = {
    "Ryzen 7 9700X": {"socket": "AM5", "gen": "9000", "ram": ["DDR5"]},
    "Ryzen 5 7600": {"socket": "AM5", "gen": "7000", "ram": ["DDR5"]},
    "Core i5-14600K": {"socket": "LGA1700", "gen": "14th", "ram": ["DDR4", "DDR5"]},
    "Core Ultra 7 265K": {"socket": "LGA1851", "gen": "200S", "ram": ["DDR5"]},
}
requests = [("B650M", "Ryzen 7 9700X"), ("B650M", "Ryzen 5 7600"), ("B550", "Ryzen 5 7600"), ("Z790", "Core i5-14600K"), ("Z790", "Core Ultra 7 265K")]
@@expected
Ryzen 7 9700X on B650M: BIOS update first
Ryzen 5 7600 on B650M: OK
Ryzen 5 7600 on B550: socket mismatch (AM5 CPU, AM4 board)
Core i5-14600K on Z790: OK
Core Ultra 7 265K on Z790: socket mismatch (LGA1851 CPU, LGA1700 board)
@@requires
"socket"
needs_bios_update
@@hint
Collect problems in a list. If the socket differs, that's the only problem worth reporting. Then check the generation and RAM.
@@solution
boards = {
    "B650M": {"socket": "AM5", "gens": ["7000", "8000"], "needs_bios_update": ["9000"], "ram": "DDR5"},
    "B550": {"socket": "AM4", "gens": ["3000", "5000"], "needs_bios_update": [], "ram": "DDR4"},
    "Z790": {"socket": "LGA1700", "gens": ["13th", "14th"], "needs_bios_update": [], "ram": "DDR5"},
}
cpus = {
    "Ryzen 7 9700X": {"socket": "AM5", "gen": "9000", "ram": ["DDR5"]},
    "Ryzen 5 7600": {"socket": "AM5", "gen": "7000", "ram": ["DDR5"]},
    "Core i5-14600K": {"socket": "LGA1700", "gen": "14th", "ram": ["DDR4", "DDR5"]},
    "Core Ultra 7 265K": {"socket": "LGA1851", "gen": "200S", "ram": ["DDR5"]},
}
requests = [("B650M", "Ryzen 7 9700X"), ("B650M", "Ryzen 5 7600"), ("B550", "Ryzen 5 7600"), ("Z790", "Core i5-14600K"), ("Z790", "Core Ultra 7 265K")]

def check(board_name, cpu_name):
    board, cpu = boards[board_name], cpus[cpu_name]
    if board["socket"] != cpu["socket"]:
        return f"socket mismatch ({cpu['socket']} CPU, {board['socket']} board)"
    problems = []
    if cpu["gen"] not in board["gens"]:
        if cpu["gen"] in board["needs_bios_update"]:
            return "BIOS update first"
        problems.append(f"generation {cpu['gen']} not supported")
    if board["ram"] not in cpu["ram"]:
        problems.append(f"CPU doesn't support {board['ram']}")
    return "; ".join(problems) or "OK"

for board_name, cpu_name in requests:
    print(f"{cpu_name} on {board_name}: {check(board_name, cpu_name)}")

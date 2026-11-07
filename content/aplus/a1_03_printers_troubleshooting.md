@@lesson a1-printers Printers and Hardware Troubleshooting
@@text
## Printer types

| Type | How | Typical problems |
|---|---|---|
| **Laser** | toner fused onto paper with heat | ghosting, smudges, streaks |
| **Inkjet** | sprays ink drops | clogged heads, banding: run head cleaning/alignment |
| **Thermal** | heat-sensitive paper | receipts and labels fading; clean the heating element |
| **Impact** (dot matrix) | pins hit a ribbon | multi-part forms; replace the ribbon |
| **3D** | melts filament layer by layer | |

## The laser printing process

Learn the **7 steps in order**; the exam loves them:

1. **Processing**: the printer turns the job into an image in memory.
2. **Charging**: the primary charge roller gives the **drum** a uniform negative charge.
3. **Exposing**: a laser "draws" the image on the drum, discharging those spots.
4. **Developing**: toner sticks to the discharged spots.
5. **Transferring**: the transfer roller pulls the toner from the drum onto the paper.
6. **Fusing**: the **fuser** melts the toner into the paper with heat and pressure.
7. **Cleaning**: a blade wipes leftover toner off the drum.

Symptoms map to steps: toner that **smears off** the page → the **fuser**. **Ghost images** (a faint copy repeating down the page) → the **drum** or cleaning. Vertical lines → scratched drum or dirty components. **Paper jams** → worn **pickup rollers**, wrong paper, or the separation pad. Blank pages → no toner or a failed transfer step.

Printers connect by USB, Ethernet, Wi-Fi, or through a **print server**. Common fixes: restart the **Print Spooler** service, reinstall the **driver** (PCL or PostScript), check the IP hasn't changed (give printers a DHCP reservation).

## Hardware troubleshooting, by symptom

| Symptom | Likely causes |
|---|---|
| No power, no lights | outlet, power cable, PSU, the front power button cable |
| Powers on, beeps, no display | RAM, GPU, CPU; check beep codes/POST LEDs |
| Shuts off after a few minutes | **overheating**: dust, failed fan, dried thermal paste |
| Random reboots / blue screens | RAM (test with Windows Memory Diagnostic or MemTest86), overheating, PSU, drivers |
| Clicking or grinding noise | failing **HDD**: back up immediately |
| Date and time reset at every boot | dead **CMOS battery** (CR2032) |
| Burning smell | **power off now**: failing PSU or component |
| Laptop won't charge | adapter wattage, port, battery health; a **swollen battery** is a hazard |
| No display on external monitor | input source, cable, `Win+P` display mode |
| Distorted, flickering display | cable, refresh rate, resolution, GPU driver |

Tools: a **multimeter** checks voltages, a **POST card** shows startup codes, a **PSU tester** checks the power supply. **S.M.A.R.T.** data shows a disk's health.

@@quiz
Toner on printed pages smears off when touched. Which part should be replaced?
* The fuser
- The drum
- The pickup roller
- The toner cartridge
? Fusing melts toner into the paper; if it's weak, toner stays loose.

@@quiz
What is the correct order of these laser printing steps?
* Charging, exposing, developing, transferring, fusing
- Exposing, charging, fusing, developing, transferring
- Developing, charging, exposing, fusing, transferring
- Charging, developing, exposing, fusing, transferring
? Processing, Charging, Exposing, Developing, Transferring, Fusing, Cleaning.

@@quiz
A PC loses its date and time every time it's unplugged. What's the fix?
* Replace the CMOS battery
- Reinstall Windows
- Replace the PSU
- Update the network driver
? The coin-cell battery keeps the clock and firmware settings without mains power.

@@quiz
A desktop shuts down by itself after about ten minutes of gaming. Fans are loud. What's most likely?
* Overheating
- A failed CMOS battery
- A bad keyboard
- An expired Windows licence
? Clean dust, check fans, and renew thermal paste.

@@quiz
A printer has frequent paper jams and sometimes pulls several sheets at once. What should you check first?
* The pickup rollers and the paper type
- The fuser temperature
- The network cable
- The printer driver version
? Worn rollers grab badly; damp or wrong paper also jams.

@@quiz
A hard drive is making clicking noises. What should you do FIRST?
* Back up the data
- Run defragmentation
- Update the BIOS
- Reformat the drive
? A mechanical failure is coming. Get the data off before anything else.

@@quiz
Multiple users can't print and documents are stuck in the queue. What's a quick first fix on the print server?
* Restart the Print Spooler service
- Replace the drum
- Reinstall Windows on the server
- Replace the network switch
? A stuck spooler is a classic; clear the queue and restart the service.

@@flashcards
laser steps :: Processing, Charging, Exposing, Developing, Transferring, Fusing, Cleaning
fuser :: Melts toner into paper; smearing = fuser
drum :: Holds the image charge; ghosting = drum or cleaning
pickup roller :: Feeds paper; jams and multi-feeds
Print Spooler :: Windows service that queues print jobs
CMOS battery :: Keeps firmware settings and clock; CR2032 coin cell
S.M.A.R.T. :: Disk self-monitoring health data
POST card :: Shows startup diagnostic codes
thermal printer :: Heat-sensitive paper: receipts, labels
impact printer :: Pins and ribbon; multi-part forms

@@drill
Which laser printing step comes right after "exposing"?
= developing
? Toner is attracted to the areas the laser discharged.

@@exercise Laser printer symptom lookup
Write `diagnose(symptom)` that returns the likely cause for a laser printer symptom, using the lesson's table:

- `smudges` (toner rubs off) → `fuser`
- `ghosting` (faint repeated image) → `drum or cleaning`
- `vertical lines` → `drum (scratched) or toner`
- `blank pages` → `transfer corona or toner`
- `paper jams` → `pickup rollers`
- anything else → `unknown: follow the troubleshooting method`

Also write `count_by_part(tickets)` that counts tickets per suspected part and prints them from most to fewest (ties alphabetically).
@@lang python
@@starter
CAUSES = {
    "smudges": "fuser",
    "ghosting": "drum or cleaning",
    "vertical lines": "drum (scratched) or toner",
    "blank pages": "transfer corona or toner",
    "paper jams": "pickup rollers",
}
tickets = ["smudges", "paper jams", "ghosting", "smudges", "wifi drops", "paper jams", "smudges"]

def diagnose(symptom):
    raise NotImplementedError
def count_by_part(tickets):
    raise NotImplementedError


for t in tickets:
    print(f"{t}: {diagnose(t)}")
print("Parts to stock:")
for part, n in count_by_part(tickets):
    print(f"  {n} x {part}")
@@expected
smudges: fuser
paper jams: pickup rollers
ghosting: drum or cleaning
smudges: fuser
wifi drops: unknown: follow the troubleshooting method
paper jams: pickup rollers
smudges: fuser
Parts to stock:
  3 x fuser
  2 x pickup rollers
  1 x drum or cleaning
  1 x unknown: follow the troubleshooting method
@@requires
def diagnose
def count_by_part
.get(
@@hint
A dictionary with `.get(symptom, default)` handles the unknown case in one line.
@@solution
CAUSES = {
    "smudges": "fuser",
    "ghosting": "drum or cleaning",
    "vertical lines": "drum (scratched) or toner",
    "blank pages": "transfer corona or toner",
    "paper jams": "pickup rollers",
}
tickets = ["smudges", "paper jams", "ghosting", "smudges", "wifi drops", "paper jams", "smudges"]

def diagnose(symptom):
    return CAUSES.get(symptom, "unknown: follow the troubleshooting method")

def count_by_part(tickets):
    counts = {}
    for t in tickets:
        part = diagnose(t)
        counts[part] = counts.get(part, 0) + 1
    return sorted(counts.items(), key=lambda kv: (-kv[1], kv[0]))

for t in tickets:
    print(f"{t}: {diagnose(t)}")
print("Parts to stock:")
for part, n in count_by_part(tickets):
    print(f"  {n} x {part}")

@@exercise Choose the printer for each job
Pick a printer type from the lesson's table for each request. Write `printer_for(job)`:

- multi-part carbon forms (`copies` > 1 at once) → **impact**;
- receipts or labels → **thermal**;
- plastic parts or prototypes → **3D**;
- photos → **inkjet**;
- otherwise high volume (more than 500 pages a month) → **laser**, low volume → **inkjet**.

Also return the maintenance tip from the table's "typical problems" column. Print each job with its printer and tip.
@@lang python
@@starter
TIPS = {
    "laser": "replace the toner and fuser on schedule; ghosting or smudges point to the drum or fuser",
    "inkjet": "run head cleaning and alignment if you see banding",
    "thermal": "clean the heating element when print fades",
    "impact": "replace the ribbon when print gets faint",
    "3D": "level the bed and keep filament dry",
}
jobs = [
    {"name": "Shop till receipts", "kind": "receipt"},
    {"name": "Warehouse delivery notes in triplicate", "kind": "form", "copies": 3},
    {"name": "Accounts office reports", "kind": "document", "pages_per_month": 4000},
    {"name": "Home user's holiday photos", "kind": "photo"},
    {"name": "Replacement plastic clip for a cabinet", "kind": "part"},
    {"name": "Small office letters", "kind": "document", "pages_per_month": 150},
    {"name": "Shipping labels", "kind": "label"},
]

def printer_for(job):
    raise NotImplementedError


for j in jobs:
    p, tip = printer_for(j)
    print(f"{j['name']}: {p} ({tip})")
@@expected
Shop till receipts: thermal (clean the heating element when print fades)
Warehouse delivery notes in triplicate: impact (replace the ribbon when print gets faint)
Accounts office reports: laser (replace the toner and fuser on schedule; ghosting or smudges point to the drum or fuser)
Home user's holiday photos: inkjet (run head cleaning and alignment if you see banding)
Replacement plastic clip for a cabinet: 3D (level the bed and keep filament dry)
Small office letters: inkjet (run head cleaning and alignment if you see banding)
Shipping labels: thermal (clean the heating element when print fades)
@@requires
def printer_for
elif
@@hint
An `if`/`elif` chain in the order of the rules; `job.get('copies', 1)` handles jobs that don't say how many copies.
@@solution
TIPS = {
    "laser": "replace the toner and fuser on schedule; ghosting or smudges point to the drum or fuser",
    "inkjet": "run head cleaning and alignment if you see banding",
    "thermal": "clean the heating element when print fades",
    "impact": "replace the ribbon when print gets faint",
    "3D": "level the bed and keep filament dry",
}
jobs = [
    {"name": "Shop till receipts", "kind": "receipt"},
    {"name": "Warehouse delivery notes in triplicate", "kind": "form", "copies": 3},
    {"name": "Accounts office reports", "kind": "document", "pages_per_month": 4000},
    {"name": "Home user's holiday photos", "kind": "photo"},
    {"name": "Replacement plastic clip for a cabinet", "kind": "part"},
    {"name": "Small office letters", "kind": "document", "pages_per_month": 150},
    {"name": "Shipping labels", "kind": "label"},
]

def printer_for(job):
    kind = job["kind"]
    if job.get("copies", 1) > 1:
        p = "impact"
    elif kind in ("receipt", "label"):
        p = "thermal"
    elif kind == "part":
        p = "3D"
    elif kind == "photo":
        p = "inkjet"
    else:
        p = "laser" if job.get("pages_per_month", 0) > 500 else "inkjet"
    return p, TIPS[p]

for j in jobs:
    p, tip = printer_for(j)
    print(f"{j['name']}: {p} ({tip})")

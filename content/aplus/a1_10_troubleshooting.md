@@lesson a1-troubleshoot Troubleshooting Hardware and Networks by Symptom
@@text
Domain 5 of Core 1 (hardware and network troubleshooting, about 28%) is the biggest single domain. The questions describe a symptom and ask for the most likely cause or the **next step**. The key is the **CompTIA troubleshooting methodology** (in this order) plus a mental table of symptoms.

## The CompTIA methodology (memorise the order)

1. **Identify the problem**: question the user, identify changes ("what changed?"), back up data **before** making changes, review logs. Reproduce if possible.
2. **Establish a theory of probable cause** (question the obvious). Research (knowledge base, internet) if needed.
3. **Test the theory** to determine the cause. If confirmed, determine the next steps; if not, establish a new theory or **escalate**.
4. **Establish a plan of action** to resolve the problem and implement the solution; refer to the vendor's instructions.
5. **Verify full system functionality** and, if applicable, implement **preventive measures**.
6. **Document** findings, actions and outcomes.

Exam favourites: "back up before changes" is part of step 1; "question the obvious" is step 2; preventive measures come with verification in step 5; documentation is always last.

## Boot and power problems

| Symptom | Likely cause |
|---|---|
| Nothing at all: no lights, no fans | power: outlet, PSU switch, cable, PSU failure, front-panel power header |
| Fans spin, no display, no beeps | RAM not seated, CPU/board fault, no CPU power (8-pin) |
| **POST beep codes** | the board telling you what failed (codes vary by vendor: look them up) |
| Boots then shuts off after minutes | **overheating** (cooler, paste, fans) or failing PSU |
| Random reboots, especially under load | PSU, overheating, RAM errors |
| "No boot device" / "Bootmgr is missing" | boot order, drive not detected, damaged boot sector (`bootrec`) |
| **BSOD** (blue screen) / kernel panic | drivers (most often), RAM (test with Windows Memory Diagnostic / MemTest86), disk errors, overheating |
| Burning smell, smoke | **unplug immediately**: a failing PSU or capacitor |
| Swollen capacitors on the board | board failure; replace |
| Date/time resets | CMOS battery |
| Grinding/clicking | failing HDD or fan bearing |

## Display problems

| Symptom | Likely cause |
|---|---|
| No image | input source, cable, monitor power, GPU seated/powered |
| Dim image | backlight, brightness setting |
| Flickering | cable, refresh rate, failing backlight |
| Wrong resolution, blurry | driver missing (generic display adapter), wrong native resolution |
| **Dead pixels** (always black) / stuck pixels (always one colour) | panel defect |
| Artifacts (garbled shapes) | GPU overheating, failing GPU or VRAM, driver |
| **Burn-in** / image persistence | OLED/plasma static images |
| Distorted colours | cable, colour profile, failing panel |
| Projector shuts off | overheating, dirty filter, lamp end of life |

## Storage problems

See the storage lesson: S.M.A.R.T. errors, clicking, slow performance, drive not recognised, RAID degraded. **Back up first.**

## Printers (summary)

Laser: **faded** prints → low toner; **ghosting** (repeated faint images) → drum or fuser; **smudging, toner rubs off** → **fuser**; vertical lines → drum or dirty corona; paper jams → pickup rollers, wrong paper; garbled characters → wrong driver. Inkjet: streaks → clogged heads (run cleaning). Thermal: faded → heating element or paper type.

## Network problems

| Symptom | Likely cause / check |
|---|---|
| **No connectivity** | cable/link light, Wi-Fi on, IP configuration (`ipconfig`) |
| **APIPA address** (169.254.x.x) | DHCP unreachable: cable, VLAN, DHCP server, scope exhausted |
| **Limited connectivity** / local only | gateway wrong or down, DNS, upstream ISP |
| Can ping IPs but not names | **DNS** |
| **Intermittent** connectivity | interference (Wi-Fi), bad cable, duplex mismatch, failing NIC, roaming issues |
| **Slow** speeds | congestion, interference, wrong band, half duplex, a 100 Mbps link from a bad cable |
| **IP conflict** warning | two devices with the same IP (a static address inside the DHCP scope) |
| Can't reach one website | DNS cache, the site itself, a proxy/firewall rule |
| **Jitter, latency, packet loss** | VoIP/video quality issues: QoS, congestion, Wi-Fi |
| Weak Wi-Fi signal | distance, walls, interference, AP placement, antenna |
| Port flapping (link up/down) | bad cable/port, failing NIC, auto-negotiation |

Tools: `ipconfig /all`, `ping`, `tracert`, `nslookup`, `netstat`, `pathping`, a **cable tester**, **loopback plug**, **tone generator and probe** (toner) to trace cables, a **Wi-Fi analyser**, a **network tap**.

## Order of checks for "the internet is down"

1. Is it one user, one area, or everyone? (scope)
2. Link light / Wi-Fi connected?
3. `ipconfig`: valid IP or APIPA? Gateway and DNS set?
4. `ping` the gateway → `ping 8.8.8.8` → `ping google.com` (or `nslookup`).
   - Gateway fails: local network.
   - Internet IP fails: router/ISP.
   - IP works but name fails: DNS.
5. Check the router/ISP status, then escalate.

@@quiz
In the CompTIA methodology, when should you back up the user's data?
* During step 1, identifying the problem, before making changes
- After verifying functionality
- Only if the drive is failing
- During documentation
? Back up before you change anything.

@@quiz
A user can ping 8.8.8.8 but can't open any website by name. What is the most likely cause?
- The cable
- The default gateway
* DNS
- The NIC driver
? IP connectivity works; name resolution fails.

@@quiz
Laser printouts smudge and the toner rubs off with a finger. Which part should you replace?
- Drum
* Fuser
- Toner cartridge
- Pickup roller
? The fuser melts toner onto the paper.

@@quiz
A PC powers on, fans spin, but there's no display and it beeps in a pattern. What's the best next step?
- Replace the monitor
* Look up the motherboard's beep code to identify the failed component
- Reinstall Windows
- Replace the hard drive
? POST codes point at the component (often RAM).

@@quiz
Users get "IP address conflict" warnings. What's the most likely cause?
* A device with a static IP inside the DHCP scope
- DNS is down
- The DHCP lease time is too long
- WPA3 is enabled
? Exclude static addresses from the scope or use reservations.

@@quiz
What is the final step of the CompTIA troubleshooting methodology?
- Verify functionality
- Implement preventive measures
* Document findings, actions and outcomes
- Escalate
? Documentation always comes last.

@@flashcards
step 1 :: identify the problem (question, changes, back up, logs)
step 2 :: theory of probable cause (question the obvious)
step 3 :: test the theory; escalate if needed
step 4 :: plan of action and implement
step 5 :: verify, preventive measures
step 6 :: document
beep codes :: POST failure indicators
BSOD :: often drivers, RAM or disk
ghosting :: drum/fuser issue
smudging :: fuser
APIPA :: DHCP failure
ping IP works, name fails :: DNS
IP conflict :: duplicate address
toner probe :: traces cables
loopback plug :: tests a port
jitter :: variation in latency

@@drill
Which step number is "test the theory to determine the cause"?
= 3
? Then plan and implement in step 4.

@@drill
Which laser printer part, when failing, makes toner rub off the page?
= fuser
? Heat and pressure fix the toner.

@@exercise Order the troubleshooting steps
The help desk notes below are shuffled. Each starts with a keyword that belongs to one step of the methodology. Write `step_of(note)` using `KEYWORDS`, then print the notes sorted by step (stable order within a step), each prefixed with its step number. Notes with no recognised keyword go last with step `?`.
@@lang python
@@starter
KEYWORDS = {
    "Asked": 1, "Backed up": 1, "Reviewed": 1,
    "Suspected": 2, "Researched": 2,
    "Tested": 3, "Confirmed": 3,
    "Replaced": 4, "Planned": 4,
    "Verified": 5, "Scheduled": 5,
    "Documented": 6,
}
notes = [
    "Replaced the fuser unit",
    "Asked the user when the smudging started",
    "Documented the fix in KB-0042",
    "Tested by printing a test page: smudges appear on every page",
    "Verified print quality with a full-page test",
    "Suspected the fuser because toner rubs off",
    "Backed up the printer configuration",
    "Called the vendor about the warranty",
    "Scheduled a fuser check in the maintenance calendar",
]
@@expected
1. Asked the user when the smudging started
1. Backed up the printer configuration
2. Suspected the fuser because toner rubs off
3. Tested by printing a test page: smudges appear on every page
4. Replaced the fuser unit
5. Verified print quality with a full-page test
5. Scheduled a fuser check in the maintenance calendar
6. Documented the fix in KB-0042
?. Called the vendor about the warranty
@@requires
def step_of
sorted(
@@hint
`step_of` returns `KEYWORDS[word]` for the first keyword the note starts with, or `99` if none. Python's `sorted` is stable, so notes with the same step keep their order.
@@solution
KEYWORDS = {
    "Asked": 1, "Backed up": 1, "Reviewed": 1,
    "Suspected": 2, "Researched": 2,
    "Tested": 3, "Confirmed": 3,
    "Replaced": 4, "Planned": 4,
    "Verified": 5, "Scheduled": 5,
    "Documented": 6,
}
notes = [
    "Replaced the fuser unit",
    "Asked the user when the smudging started",
    "Documented the fix in KB-0042",
    "Tested by printing a test page: smudges appear on every page",
    "Verified print quality with a full-page test",
    "Suspected the fuser because toner rubs off",
    "Backed up the printer configuration",
    "Called the vendor about the warranty",
    "Scheduled a fuser check in the maintenance calendar",
]

def step_of(note):
    for word, step in KEYWORDS.items():
        if note.startswith(word):
            return step
    return 99

for note in sorted(notes, key=step_of):
    step = step_of(note)
    print(f"{step if step != 99 else '?'}. {note}")

@@exercise "The internet is down" decision tree
Implement the lesson's order of checks as `diagnose(r)`, where `r` is a dict of test results: `link` (cable/Wi-Fi connected), `ip` (the address string), `gw_ping`, `internet_ping` (8.8.8.8) and `dns` (name lookup works). Return the first failing layer's diagnosis: no link → `"physical: check cable/Wi-Fi"`; APIPA address → `"DHCP: no address from the server"`; gateway ping fails → `"local network: gateway unreachable"`; internet ping fails → `"router/ISP: no internet route"`; DNS fails → `"DNS: name resolution failing"`; otherwise `"network OK: check the application or site"`.
@@lang python
@@starter
cases = {
    "Ana": {"link": True, "ip": "192.168.1.40", "gw_ping": True, "internet_ping": True, "dns": False},
    "Ken": {"link": True, "ip": "169.254.12.9", "gw_ping": False, "internet_ping": False, "dns": False},
    "Priya": {"link": False, "ip": "", "gw_ping": False, "internet_ping": False, "dns": False},
    "Omar": {"link": True, "ip": "192.168.1.52", "gw_ping": True, "internet_ping": False, "dns": False},
    "Mei": {"link": True, "ip": "192.168.1.61", "gw_ping": True, "internet_ping": True, "dns": True},
}

def diagnose(r):
    raise NotImplementedError
@@expected
Ana: DNS: name resolution failing
Ken: DHCP: no address from the server
Priya: physical: check cable/Wi-Fi
Omar: router/ISP: no internet route
Mei: network OK: check the application or site
@@requires
def diagnose
"169.254."
@@hint
Check in order and return early: `if not r["link"]: return ...`, `if r["ip"].startswith("169.254."): return ...`, and so on.
@@solution
cases = {
    "Ana": {"link": True, "ip": "192.168.1.40", "gw_ping": True, "internet_ping": True, "dns": False},
    "Ken": {"link": True, "ip": "169.254.12.9", "gw_ping": False, "internet_ping": False, "dns": False},
    "Priya": {"link": False, "ip": "", "gw_ping": False, "internet_ping": False, "dns": False},
    "Omar": {"link": True, "ip": "192.168.1.52", "gw_ping": True, "internet_ping": False, "dns": False},
    "Mei": {"link": True, "ip": "192.168.1.61", "gw_ping": True, "internet_ping": True, "dns": True},
}

def diagnose(r):
    if not r["link"]:
        return "physical: check cable/Wi-Fi"
    if r["ip"].startswith("169.254."):
        return "DHCP: no address from the server"
    if not r["gw_ping"]:
        return "local network: gateway unreachable"
    if not r["internet_ping"]:
        return "router/ISP: no internet route"
    if not r["dns"]:
        return "DNS: name resolution failing"
    return "network OK: check the application or site"

for user, results in cases.items():
    print(f"{user}: {diagnose(results)}")

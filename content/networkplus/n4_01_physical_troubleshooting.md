@@lesson n4-physical Troubleshooting Cables, Interfaces and Hardware
@@text
Troubleshooting is the largest Network+ domain (about 24%). It starts with the CompTIA method, then works up the layers. This lesson covers layer 1 and 2 problems: cables, interfaces, counters, transceivers and power. These are the faults you find with a cable tester and `show interface`.

## The CompTIA troubleshooting method

1. **Identify the problem**: gather information, question users, identify symptoms, determine if anything changed, duplicate the problem, approach multiple problems individually.
2. **Establish a theory of probable cause**: question the obvious; consider multiple approaches (**top-to-bottom/bottom-to-top OSI**, **divide and conquer**).
3. **Test the theory** to determine the cause. If confirmed, plan next steps; if not, establish a new theory or **escalate**.
4. **Establish a plan of action** to resolve the problem and identify potential effects.
5. **Implement the solution** or escalate as necessary.
6. **Verify full system functionality** and, if applicable, implement preventive measures.
7. **Document findings, actions, outcomes and lessons learned**.

## Cable problems

| Symptom | Likely cause |
|---|---|
| No link light | broken cable, wrong port, port disabled/err-disabled, bad transceiver, wrong fibre polarity (TX/RX swapped) |
| Link up but slow/errors | damaged cable, **EMI** near power lines or motors, too long (attenuation), bad termination, **crosstalk** (NEXT/FEXT), wrong category |
| Works then drops | loose connector, cable crushed under a chair or door, intermittent fault |
| Fibre: high loss | dirty connector (clean before you blame the fibre!), tight bends (bend radius), wrong type (SM vs MM), mismatched optics or wavelength, APC/UPC mix |
| PoE device won't power | PoE budget exceeded, wrong standard (needs PoE+ on a PoE port), cable too long or poor quality, PoE disabled on port |

**Attenuation** = signal weakens with distance. **dB loss** budget for fibre: add up connector (~0.3–0.75 dB each), splice (~0.1 dB) and per-km loss; it must be below the optics' power budget.

## Interface counters

`show interface` (or `Get-NetAdapterStatistics`, `ip -s link`) shows:

| Counter | Means | Typical cause |
|---|---|---|
| **CRC errors** | frame failed the checksum | bad cable, EMI, duplex mismatch |
| **Runts** | frames < 64 bytes | collisions, duplex mismatch |
| **Giants** | frames > 1518 bytes (without jumbo config) | MTU mismatch, misconfigured jumbo frames |
| **Late collisions** | collision after 64 bytes | **duplex mismatch**, cable too long |
| **Input/output discards (drops)** | buffers full | congestion, microbursts |
| **Interface resets / flapping** | link going up and down | bad cable/SFP, power, STP issue |
| **Encapsulation errors** | trunk/VLAN tag issues | trunk misconfig |

Counters are cumulative since the last clear or reboot: **clear them, then watch whether they still increase**.

## Interface status

- **up/up**: working.
- **down/down**: no physical link (cable, far end off, bad transceiver).
- **administratively down**: someone ran `shutdown`. Enable it.
- **err-disabled**: the switch shut it for a reason: **port security** violation, **BPDU guard**, link flap, storm control. Fix the cause, then reset the port.
- **up/down** (line protocol down): layer 1 fine, layer 2 problem: encapsulation mismatch on serial/WAN links, keepalives.

## Transceivers

Check: right type for the fibre (SR for multimode, LR for single-mode), both ends same speed and wavelength, vendor-supported (some switches refuse third-party optics), **DOM/DDM** readings (digital optical monitoring: transmit/receive power in dBm, temperature). Receive power near or below the receiver's sensitivity = dirty, bent, too long or failing.

## Hardware and environment

- **Power**: failed PSU, overloaded circuit, UPS on battery. Check the device's power and environment logs.
- **Temperature**: overheating causes reboots and errors. Check fans, airflow (hot aisle/cold aisle), blocked vents.
- **Firmware bugs**: check release notes, upgrade (or roll back).
- **Hardware failure**: replace under warranty/support contract (know your RMA process).

## Tools for this layer

Cable tester, certifier, **TDR** (distance to copper fault) and **OTDR** (fibre), **toner and probe**, **light meter**, **visual fault locator**, **loopback plug**, **multimeter**, **spectrum analyser** (Wi-Fi interference), **Wi-Fi analyser**, **punch-down tool**, **crimper**, **cable stripper**, **fibre cleaning kit**.

@@quiz
A switch port shows rising CRC errors and late collisions; the server NIC is hard-set to 1 Gb/s full duplex while the switch is on auto. What's the fix?
* Set both sides to auto-negotiate (or both to the same speed/duplex)
- Replace the switch
- Enable jumbo frames
- Disable spanning tree
? Classic duplex mismatch.

@@quiz
A port shows "err-disabled" after a user plugged in a small switch. What probably happened?
- The cable is too long
* BPDU guard or port security shut the port
- The PoE budget ran out
- DHCP snooping blocked the user
? Fix the cause, then re-enable the port.

@@quiz
A new fibre link shows no light. The optics and fibre type are correct. What's a likely cause?
- EMI
* TX and RX are swapped (polarity) or the connectors are dirty
- Duplex mismatch
- Wrong VLAN
? Check polarity and clean the connectors first.

@@quiz
A counter shows many giants on a storage link. What should you check?
- Cable category
* MTU/jumbo frame configuration on both ends
- STP root bridge
- Wi-Fi channel
? Giants are frames larger than the configured MTU.

@@quiz
Which step of the CompTIA method comes right after testing the theory and confirming the cause?
- Document findings
* Establish a plan of action and identify potential effects
- Verify full system functionality
- Identify the problem
? Then implement, verify, document.

@@flashcards
troubleshooting step 1 :: identify the problem
troubleshooting step 2 :: establish a theory
troubleshooting step 3 :: test the theory
troubleshooting step 4 :: plan of action and effects
troubleshooting step 5 :: implement or escalate
troubleshooting step 6 :: verify, prevent
troubleshooting step 7 :: document
CRC errors :: bad cable, EMI, duplex mismatch
runts :: frames under 64 bytes
giants :: frames over the MTU
late collisions :: duplex mismatch
discards :: full buffers, congestion
err-disabled :: port shut by a protection feature
administratively down :: shut by config
DOM/DDM :: optic power and temperature readings
TDR / OTDR :: find copper / fibre faults
attenuation :: signal loss over distance

@@drill
How many steps does the CompTIA troubleshooting method have?
= 7
= seven
? Identify, theory, test, plan, implement, verify, document.

@@drill
What is the minimum Ethernet frame size in bytes?
= 64
? Smaller frames are runts.

@@exercise Diagnose interfaces from counters
Two snapshots of interface counters were taken 10 minutes apart. For each interface, compute the increase in each counter, and diagnose using these rules in order: status `err-disabled` → "protection feature shut the port"; status `down` → "check cable/transceiver/far end"; late collisions increasing → "duplex mismatch"; giants increasing → "MTU mismatch"; CRC increasing → "physical layer: cable or EMI"; discards increasing → "congestion"; otherwise "healthy". Print the diagnosis with the counter increases that triggered it.
@@lang python
@@starter
before = {
    "Gi1/0/1": {"status": "up", "crc": 10, "late": 0, "giants": 0, "discards": 500},
    "Gi1/0/2": {"status": "up", "crc": 120, "late": 30, "giants": 0, "discards": 0},
    "Gi1/0/3": {"status": "up", "crc": 4, "late": 0, "giants": 900, "discards": 0},
    "Gi1/0/4": {"status": "up", "crc": 1500, "late": 0, "giants": 0, "discards": 0},
    "Gi1/0/5": {"status": "err-disabled", "crc": 0, "late": 0, "giants": 0, "discards": 0},
    "Gi1/0/6": {"status": "down", "crc": 0, "late": 0, "giants": 0, "discards": 0},
}
after = {
    "Gi1/0/1": {"status": "up", "crc": 10, "late": 0, "giants": 0, "discards": 9500},
    "Gi1/0/2": {"status": "up", "crc": 480, "late": 210, "giants": 0, "discards": 0},
    "Gi1/0/3": {"status": "up", "crc": 4, "late": 0, "giants": 4100, "discards": 0},
    "Gi1/0/4": {"status": "up", "crc": 1500, "late": 0, "giants": 0, "discards": 0},
    "Gi1/0/5": {"status": "err-disabled", "crc": 0, "late": 0, "giants": 0, "discards": 0},
    "Gi1/0/6": {"status": "down", "crc": 0, "late": 0, "giants": 0, "discards": 0},
}
@@expected
Gi1/0/1: congestion (discards +9000)
Gi1/0/2: duplex mismatch (crc +360, late +180)
Gi1/0/3: MTU mismatch (giants +3200)
Gi1/0/4: healthy
Gi1/0/5: protection feature shut the port
Gi1/0/6: check cable/transceiver/far end
@@requires
after[
"duplex mismatch"
@@hint
`delta = {k: after[i][k] - before[i][k] for k in ("crc", "late", "giants", "discards")}`. Note Gi1/0/4 has old errors but none new.
@@solution
before = {
    "Gi1/0/1": {"status": "up", "crc": 10, "late": 0, "giants": 0, "discards": 500},
    "Gi1/0/2": {"status": "up", "crc": 120, "late": 30, "giants": 0, "discards": 0},
    "Gi1/0/3": {"status": "up", "crc": 4, "late": 0, "giants": 900, "discards": 0},
    "Gi1/0/4": {"status": "up", "crc": 1500, "late": 0, "giants": 0, "discards": 0},
    "Gi1/0/5": {"status": "err-disabled", "crc": 0, "late": 0, "giants": 0, "discards": 0},
    "Gi1/0/6": {"status": "down", "crc": 0, "late": 0, "giants": 0, "discards": 0},
}
after = {
    "Gi1/0/1": {"status": "up", "crc": 10, "late": 0, "giants": 0, "discards": 9500},
    "Gi1/0/2": {"status": "up", "crc": 480, "late": 210, "giants": 0, "discards": 0},
    "Gi1/0/3": {"status": "up", "crc": 4, "late": 0, "giants": 4100, "discards": 0},
    "Gi1/0/4": {"status": "up", "crc": 1500, "late": 0, "giants": 0, "discards": 0},
    "Gi1/0/5": {"status": "err-disabled", "crc": 0, "late": 0, "giants": 0, "discards": 0},
    "Gi1/0/6": {"status": "down", "crc": 0, "late": 0, "giants": 0, "discards": 0},
}

for name in before:
    status = after[name]["status"]
    delta = {k: after[name][k] - before[name][k] for k in ("crc", "late", "giants", "discards")}
    if status == "err-disabled":
        diagnosis = "protection feature shut the port"
    elif status == "down":
        diagnosis = "check cable/transceiver/far end"
    elif delta["late"] > 0:
        diagnosis = "duplex mismatch"
    elif delta["giants"] > 0:
        diagnosis = "MTU mismatch"
    elif delta["crc"] > 0:
        diagnosis = "physical layer: cable or EMI"
    elif delta["discards"] > 0:
        diagnosis = "congestion"
    else:
        diagnosis = "healthy"
    rising = ", ".join(f"{k} +{v}" for k, v in delta.items() if v > 0)
    print(f"{name}: {diagnosis}" + (f" ({rising})" if rising else ""))

@@exercise Fibre loss budget
Compute the expected loss of each fibre link: per-km loss × length, plus connectors × 0.5 dB, plus splices × 0.1 dB. Compare it with the optic's power budget (transmit power minus receiver sensitivity) and print the margin. A margin under 3 dB is `MARGINAL`, under 0 is `FAIL`. Round to 2 decimals.
@@lang python
@@starter
OPTICS = {"10G-SR": (-1.0, -9.9), "10G-LR": (-1.0, -14.4), "10G-ER": (2.0, -15.8)}
FIBRE_LOSS = {"OM4": 3.0, "OS2": 0.35}
links = [
    ("Rack A to B", "10G-SR", "OM4", 0.15, 2, 0),
    ("Floor 1 to 9", "10G-SR", "OM4", 0.4, 4, 1),
    ("Campus north", "10G-LR", "OS2", 8, 4, 3),
    ("Campus south", "10G-LR", "OS2", 26, 6, 8),
    ("Metro link", "10G-ER", "OS2", 38, 4, 10),
]
@@expected
Rack A to B (10G-SR, 0.15 km OM4): loss 1.45 dB, budget 8.90 dB, margin 7.45 dB OK
Floor 1 to 9 (10G-SR, 0.4 km OM4): loss 3.30 dB, budget 8.90 dB, margin 5.60 dB OK
Campus north (10G-LR, 8 km OS2): loss 5.10 dB, budget 13.40 dB, margin 8.30 dB OK
Campus south (10G-LR, 26 km OS2): loss 12.90 dB, budget 13.40 dB, margin 0.50 dB MARGINAL
Metro link (10G-ER, 38 km OS2): loss 16.30 dB, budget 17.80 dB, margin 1.50 dB MARGINAL
@@requires
OPTICS[
FIBRE_LOSS[
@@hint
Budget = tx − sensitivity (e.g. −1.0 − (−9.9) = 8.9 dB). Margin = budget − loss.
@@solution
OPTICS = {"10G-SR": (-1.0, -9.9), "10G-LR": (-1.0, -14.4), "10G-ER": (2.0, -15.8)}
FIBRE_LOSS = {"OM4": 3.0, "OS2": 0.35}
links = [
    ("Rack A to B", "10G-SR", "OM4", 0.15, 2, 0),
    ("Floor 1 to 9", "10G-SR", "OM4", 0.4, 4, 1),
    ("Campus north", "10G-LR", "OS2", 8, 4, 3),
    ("Campus south", "10G-LR", "OS2", 26, 6, 8),
    ("Metro link", "10G-ER", "OS2", 38, 4, 10),
]

for name, optic, fibre, km, connectors, splices in links:
    tx, sensitivity = OPTICS[optic]
    budget = tx - sensitivity
    loss = FIBRE_LOSS[fibre] * km + connectors * 0.5 + splices * 0.1
    margin = budget - loss
    status = "FAIL" if margin < 0 else "MARGINAL" if margin < 3 else "OK"
    print(f"{name} ({optic}, {km} km {fibre}): loss {loss:.2f} dB, budget {budget:.2f} dB, margin {margin:.2f} dB {status}")

@@lesson a2-swtrouble Software, Malware and Mobile App Troubleshooting
@@text
Core 2 domain 3 (software troubleshooting, about 23%) gives you symptoms and asks for the most likely cause or the next step. It covers Windows problems, malware, and mobile OS and app issues. The answers follow a pattern: try the least invasive fix first, and know the order of malware removal by heart.

## Common Windows symptoms

| Symptom | Likely causes / fixes |
|---|---|
| **Blue screen (BSOD)** | recent driver or update → roll back, Safe Mode, check Event Viewer and the stop code, test RAM |
| **Slow performance** | too many startup apps, low disk space, malware, failing disk, not enough RAM → Task Manager, Disk Cleanup, scan |
| **Boot problems** | `bootrec /fixmbr /fixboot /rebuildbcd` from Windows RE, Startup Repair, System Restore |
| **Frequent shutdowns** | overheating (dust, fans), power supply |
| **Services fail to start** | dependency not running, wrong service account password, corrupt files → `services.msc`, Event Viewer, `sfc` |
| **Applications crash** | update/repair/reinstall the app, check compatibility mode, clear its cache |
| **Low memory warnings** | close apps, increase virtual memory (page file), add RAM |
| **USB controller resource warnings** | too many devices on one controller → use another port/hub |
| **System instability** after an update | uninstall the update, System Restore |
| **Profile doesn't load / temporary profile** | rebuild the user profile (rename the old one, copy data) |
| **Time drift** | NTP, CMOS battery |

## Repair tools in order of impact

1. Restart (it really does fix a lot).
2. **Safe Mode** (minimal drivers) to isolate driver/software problems.
3. Roll back the driver, uninstall the recent update or app.
4. **System Restore** to a restore point (keeps user files, reverts system and apps).
5. `sfc` / `DISM` repair.
6. **Reset this PC** (keep files) or a repair install.
7. **Reimage** / clean install (last resort, restore data from backup).

## Malware: symptoms

Pop-ups and browser redirects, a changed home page or search engine, fake security warnings, certificate warnings, **disabled antivirus or updates**, files renamed or encrypted (ransomware), high CPU or network use, unknown processes, email sent from the user's account that they didn't write, desktop alerts, OS update failures.

## The malware removal procedure (memorise the order)

1. **Investigate and verify** malware symptoms.
2. **Quarantine** the infected system (disconnect from the network).
3. **Disable System Restore** (in Windows) so restore points don't keep the infection.
4. **Remediate**: update anti-malware software, scan and remove (Safe Mode, a pre-installation environment/boot scanner); **reimage** if in doubt.
5. **Schedule scans and run updates**.
6. **Enable System Restore and create a restore point**.
7. **Educate the end user**.

## Mobile OS and application troubleshooting

| Symptom | Try |
|---|---|
| App won't load / crashes | force stop, clear cache (Android), update or reinstall the app, restart |
| Slow phone | storage nearly full, too many background apps, update the OS |
| Battery drains fast | check battery usage per app, background location, screen brightness, battery health |
| Won't sync email/calendar | network, account password changed, sync settings, server, date/time |
| Screen won't rotate | rotation lock, the app doesn't support it |
| OS update fails | storage space, battery level, Wi-Fi |
| Random reboots | OS bug or app, overheating, failing battery |
| Bluetooth / Wi-Fi drops | forget and re-pair, update, reset network settings |

Last resort: **reset network settings**, then **factory reset** (after backup).

## Mobile security symptoms

**Unexpected data usage**, high battery drain, apps you didn't install, **unexpected admin or root access requests**, pop-ups, sluggish response, leaked personal files, unauthorised account access, **APK** files from unofficial sources (sideloading), developer mode or USB debugging enabled, jailbroken/rooted state. Fix: remove the app, update, scan (Android), factory reset if in doubt, change passwords from a clean device, and enrol in MDM.

## Browser-related symptoms

**Random pop-ups** and redirects → malicious extension or adware; **certificate warnings** on many sites → wrong date/time, a proxy intercepting, or malware installing a root certificate; **home page changed** → browser hijacker. Reset the browser and remove extensions.

@@quiz
Which is the correct first step of the malware removal procedure?
* Investigate and verify malware symptoms
- Disable System Restore
- Reimage the PC
- Educate the user
? Then quarantine, disable System Restore, remediate...

@@quiz
Why disable System Restore while removing malware?
- It speeds up the scan
* Restore points may contain the malware and reinfect the PC
- It is required to boot Safe Mode
- To free disk space for the antivirus
? Re-enable it and create a clean point afterwards.

@@quiz
Many HTTPS sites suddenly show certificate warnings on one PC. What's the most likely cause?
- The sites all expired at once
* The PC's date and time are wrong (or something is intercepting traffic)
- The monitor's resolution changed
- The user has too many tabs
? Certificates are validated against the clock.

@@quiz
A phone's mobile data use jumped and the battery drains fast; there's an app the user doesn't remember installing. What should you do first?
- Replace the battery
* Remove the unknown app, check for sideloaded APKs, and scan/update the device
- Increase the data plan
- Turn on developer mode
? These are signs of malicious apps.

@@quiz
A PC blue-screens after a new graphics driver installed. Least invasive fix?
- Reimage the PC
* Boot Safe Mode and roll back the driver
- Replace the GPU
- Reset this PC
? Least impact first.

@@flashcards
BSOD after update :: Safe Mode, roll back driver/update
System Restore :: reverts system and apps, keeps files
Reset this PC :: reinstall Windows, optionally keep files
malware step 1 :: investigate and verify
malware step 2 :: quarantine
malware step 3 :: disable System Restore
malware step 4 :: remediate (update, scan, remove)
malware step 5 :: schedule scans and updates
malware step 6 :: enable System Restore, create a point
malware step 7 :: educate the end user
certificate warnings everywhere :: check date/time
APK :: Android package; sideloading risk
reset network settings :: before a factory reset
browser hijacker :: changes home page and search
temporary profile :: rebuild the user profile

@@drill
How many steps does the CompTIA malware removal procedure have?
= 7
= seven
? Investigate, quarantine, disable restore, remediate, schedule, enable restore, educate.

@@drill
Which Windows mode loads minimal drivers to isolate problems?
= Safe Mode
= safe mode
? F8/Shift+Restart → Troubleshoot → Startup Settings.

@@exercise Put the malware removal steps in order
The steps below are shuffled and worded informally. Using `ORDER` (keywords for each official step, in order), sort the notes into the correct sequence and print them numbered. Then check a technician's log `done` and print the first step they performed out of order, or `log follows the procedure`.
@@lang python
@@starter
ORDER = ["verify", "quarantine", "disable system restore", "remediate", "schedule", "enable system restore", "educate"]
notes = [
    "Educate Ken about not opening invoice attachments",
    "Remediate: update Defender, offline scan, remove",
    "Quarantine: unplug the network cable",
    "Enable System Restore and make a clean restore point",
    "Verify: pop-ups and redirects confirmed",
    "Disable System Restore",
    "Schedule weekly scans and turn updates on",
]
done = ["verify", "quarantine", "remediate", "disable system restore", "schedule", "enable system restore", "educate"]
@@expected
1. Verify: pop-ups and redirects confirmed
2. Quarantine: unplug the network cable
3. Disable System Restore
4. Remediate: update Defender, offline scan, remove
5. Schedule weekly scans and turn updates on
6. Enable System Restore and make a clean restore point
7. Educate Ken about not opening invoice attachments
Out of order: remediate
@@requires
ORDER.index(
sorted(
@@hint
For the key, find the ORDER keyword the lowercase note starts with (`text.startswith(keyword)`), then use its `ORDER.index`.
@@solution
ORDER = ["verify", "quarantine", "disable system restore", "remediate", "schedule", "enable system restore", "educate"]
notes = [
    "Educate Ken about not opening invoice attachments",
    "Remediate: update Defender, offline scan, remove",
    "Quarantine: unplug the network cable",
    "Enable System Restore and make a clean restore point",
    "Verify: pop-ups and redirects confirmed",
    "Disable System Restore",
    "Schedule weekly scans and turn updates on",
]
done = ["verify", "quarantine", "remediate", "disable system restore", "schedule", "enable system restore", "educate"]

def step_of(note):
    text = note.lower()
    for keyword in ORDER:
        if text.startswith(keyword):
            return ORDER.index(keyword)
    raise ValueError(note)

for number, note in enumerate(sorted(notes, key=step_of), start=1):
    print(f"{number}. {note}")

problem = None
for position, step in enumerate(done):
    if ORDER.index(step) != position:
        problem = step
        break
print(f"Out of order: {problem}" if problem else "log follows the procedure")

@@exercise Mobile symptom triage
Score each phone's report against symptom rules. Each rule adds points to "malware" or "hardware/battery" or "app/config". Print the most likely category for each phone (ties go to the category listed first in `CATEGORIES`) and its recommended first action.
@@lang python
@@starter
CATEGORIES = ["malware", "app/config", "hardware/battery"]
RULES = {
    "unknown app": ("malware", 3),
    "data spike": ("malware", 2),
    "pop-ups": ("malware", 2),
    "battery drain": ("hardware/battery", 1),
    "swollen": ("hardware/battery", 5),
    "hot": ("hardware/battery", 2),
    "crashes": ("app/config", 2),
    "won't sync": ("app/config", 3),
    "after update": ("app/config", 1),
}
ACTIONS = {
    "malware": "remove unknown apps, update, scan, reset if in doubt",
    "app/config": "clear cache, update/reinstall the app, check account settings",
    "hardware/battery": "check battery health; stop using a swollen battery",
}
reports = {
    "iPhone-Sato": "battery drain and data spike since an unknown app appeared",
    "Pixel-Lee": "email won't sync after update",
    "Galaxy-Ito": "phone is hot, battery drain, back looks swollen",
    "Moto-Kim": "app crashes and random pop-ups",
}
@@expected
iPhone-Sato: malware (5 pts) -> remove unknown apps, update, scan, reset if in doubt
Pixel-Lee: app/config (4 pts) -> clear cache, update/reinstall the app, check account settings
Galaxy-Ito: hardware/battery (8 pts) -> check battery health; stop using a swollen battery
Moto-Kim: malware (2 pts) -> remove unknown apps, update, scan, reset if in doubt
@@requires
RULES.items()
CATEGORIES.index(
@@hint
Start `scores = {c: 0 for c in CATEGORIES}`, add points for each keyword found, then `max(CATEGORIES, key=lambda c: (scores[c], -CATEGORIES.index(c)))`.
@@solution
CATEGORIES = ["malware", "app/config", "hardware/battery"]
RULES = {
    "unknown app": ("malware", 3),
    "data spike": ("malware", 2),
    "pop-ups": ("malware", 2),
    "battery drain": ("hardware/battery", 1),
    "swollen": ("hardware/battery", 5),
    "hot": ("hardware/battery", 2),
    "crashes": ("app/config", 2),
    "won't sync": ("app/config", 3),
    "after update": ("app/config", 1),
}
ACTIONS = {
    "malware": "remove unknown apps, update, scan, reset if in doubt",
    "app/config": "clear cache, update/reinstall the app, check account settings",
    "hardware/battery": "check battery health; stop using a swollen battery",
}
reports = {
    "iPhone-Sato": "battery drain and data spike since an unknown app appeared",
    "Pixel-Lee": "email won't sync after update",
    "Galaxy-Ito": "phone is hot, battery drain, back looks swollen",
    "Moto-Kim": "app crashes and random pop-ups",
}

for phone, text in reports.items():
    scores = {c: 0 for c in CATEGORIES}
    for keyword, (category, points) in RULES.items():
        if keyword in text.lower():
            scores[category] += points
    best = max(CATEGORIES, key=lambda c: (scores[c], -CATEGORIES.index(c)))
    print(f"{phone}: {best} ({scores[best]} pts) -> {ACTIONS[best]}")

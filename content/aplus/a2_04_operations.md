@@lesson a2-operations Software Troubleshooting and Operational Procedures
@@text
## Windows and app troubleshooting

| Symptom | Try |
|---|---|
| **Blue screen (BSOD)** | note the stop code; recent driver/update? roll back, Safe Mode, memory test, `sfc`/`DISM` |
| Slow performance | Task Manager: CPU/RAM/disk hogs, startup apps, malware scan, free disk space |
| App crashes | Event Viewer, update or **repair/reinstall** the app, clear its cache, check compatibility |
| Won't boot | Startup Repair, Safe Mode, System Restore, `bootrec` in the recovery environment |
| Profile loads slowly / temporary profile | corrupted user profile: rebuild it |
| Services won't start | check dependencies, the account it runs as, Event Viewer |
| Time is wrong / login fails with certificate or Kerberos errors | **time sync** (domain logins fail if the clock is off by > 5 minutes) |
| Mobile app issues | force stop, clear cache, update, reboot, reinstall, check storage and battery saver |

## Change management

Changes to production systems follow a process, so they don't cause outages:

1. A **request for change** with the purpose, **scope**, **risk analysis** and a **rollback plan**.
2. **Testing** in a sandbox / test environment.
3. Approval by a **change advisory board (CAB)**.
4. Implement in an agreed **maintenance window**, with **user acceptance** and communication.
5. Document everything.

## Backups

- **Full**: everything. **Incremental**: changes since the last backup of *any* kind (fast backup, slow restore: full + every incremental). **Differential**: changes since the last *full* (restore = full + latest differential). **Synthetic full**: built from a full + incrementals without re-reading the source.
- The **3-2-1 rule**: 3 copies, on 2 types of media, 1 off-site. Ransomware made a "1 offline/immutable" copy essential.
- **Test restores** regularly. **GFS** (grandfather-father-son) is a common rotation: daily, weekly, monthly.

## Safety and environment

- **ESD**: wear an anti-static wrist strap, use an ESD mat and anti-static bags.
- Unplug equipment before opening; never open a **PSU** or a CRT.
- Lift with your legs; use a cart for heavy items.
- Fire involving electrical equipment: use a **CO₂** extinguisher (class C in the US), not water.
- Batteries, toner and old electronics go to proper **recycling**; read the **SDS** (safety data sheet) for chemicals.
- Equipment needs suitable temperature, humidity (too dry → static), ventilation, and a **UPS** or surge protector.

## Professionalism, privacy and licensing

- Be on time, dress appropriately, listen, avoid jargon, don't take personal calls, stay calm with angry customers, and follow up.
- **PII** (personal data), **PCI DSS** (card data), **GDPR**, Japan's **APPI**: handle data only as allowed. Keep **chain of custody** if you find illegal content: don't touch, report to your manager.
- Licences: **per-seat**, **per-user**, **subscription**, **open-source** (GPL, MIT), **EULA**. **DRM** protects content.

## Documentation, scripting, remote access

- **Knowledge base** articles, network diagrams, asset inventory, **SOPs**, **AUPs** (acceptable use policies), incident reports, new user/off-boarding checklists.
- Script types on the exam: **.bat**, **.ps1**, **.vbs**, **.sh**, **.js**, **.py**. Use them to automate restarts, map drives, install apps, back up. Risks: running unknown scripts, accidental changes, using too many resources.
- Remote access: **RDP** (3389), **SSH** (22), **VPN**, **VNC**, and remote-support tools (Quick Assist, TeamViewer). Always ask for consent, and don't expose RDP to the internet.

@@quiz
A change to a server failed during the maintenance window and caused errors. What part of the change plan helps most now?
* The rollback (backout) plan
- The scope statement
- The CAB meeting minutes
- The user acceptance test
? A tested rollback plan returns the system to its last good state.

@@quiz
Backups run Sunday (full) and incremental Monday to Friday. The server fails Friday afternoon. What's needed to restore?
* Sunday's full plus every incremental from Monday to Friday
- Only Friday's incremental
- Sunday's full plus Friday's incremental only
- Only Sunday's full
? Each incremental only holds changes since the previous backup. With differentials you'd need full + Friday's only.

@@quiz
Which follows the 3-2-1 backup rule?
* Data on the server, a copy on a NAS, and a copy in cloud storage
- Three copies on the same disk
- Two copies on one USB stick
- A RAID 1 array
? 3 copies, 2 kinds of media, 1 off-site.

@@quiz
A user can't log into the domain and gets errors about time. What should you check?
* The PC's clock and time synchronization
- The monitor's refresh rate
- The printer driver
- The keyboard layout
? Kerberos authentication fails when clocks differ by more than about 5 minutes.

@@quiz
Before replacing RAM, what protects the components from static?
* An anti-static (ESD) wrist strap connected to ground
- Rubber gloves
- Working on a carpet
- Keeping the PC plugged into the wall and switched on
? ESD can damage parts without you feeling it.

@@quiz
While fixing a PC, you find illegal content. What should you do?
* Stop, don't touch anything further, and report it through the proper channel, preserving chain of custody
- Delete it
- Copy it to a USB stick as evidence
- Ignore it and finish the repair
? Changing evidence can ruin an investigation.

@@quiz
Which file extension is a PowerShell script?
* .ps1
- .bat
- .vbs
- .sh
? .bat is a Windows batch file, .vbs VBScript, .sh a shell script.

@@flashcards
change management :: Request, risk analysis, test, CAB approval, maintenance window, rollback plan, document
rollback plan :: How to undo a change
incremental backup :: Changes since the last backup of any type
differential backup :: Changes since the last full backup
3-2-1 rule :: 3 copies, 2 media types, 1 off-site
GFS :: Grandfather-father-son backup rotation
ESD :: Electrostatic discharge; use a wrist strap and mat
SDS :: Safety data sheet for chemicals
UPS :: Battery backup power
PII :: Personally identifiable information
chain of custody :: Documented handling of evidence
EULA :: End-user licence agreement
AUP :: Acceptable use policy

@@drill
Which backup type contains all changes since the last FULL backup?
= differential
= differential backup
? Incremental is since the last backup of any kind.

@@exercise Check a backup rotation
A small office runs a full backup on Sundays and incremental backups the other days. To restore a given day you need the last full backup and every incremental since it. Write `restore_set(backups, day)` returning the backup names needed, in order, or raising `ValueError("no full backup before <day>")`. Also print whether each restore set meets the 3-2-1 rule's "one copy off-site" part: at least one backup in the set must be stored `offsite`.
@@lang python
@@starter
backups = [
    ("Sun-full", "full", "local"),
    ("Mon-inc", "inc", "local"),
    ("Tue-inc", "inc", "local"),
    ("Wed-inc", "inc", "local"),
    ("Thu-inc", "inc", "offsite"),
    ("Fri-inc", "inc", "local"),
]

def restore_set(backups, day):
    raise NotImplementedError


for day in ["Sun-full", "Wed-inc", "Fri-inc"]:
    chosen = restore_set(backups, day)
    offsite = any(where == "offsite" for _name, _kind, where in chosen)
    print(f"Restore {day}: " + " + ".join(name for name, _k, _w in chosen) + f" (off-site copy: {'yes' if offsite else 'no'})")
try:
    restore_set(backups[1:], "Tue-inc")
except ValueError as e:
    print("Error:", e)
@@expected
Restore Sun-full: Sun-full (off-site copy: no)
Restore Wed-inc: Sun-full + Mon-inc + Tue-inc + Wed-inc (off-site copy: no)
Restore Fri-inc: Sun-full + Mon-inc + Tue-inc + Wed-inc + Thu-inc + Fri-inc (off-site copy: yes)
Error: no full backup before Tue-inc
@@requires
def restore_set
raise ValueError
any(
@@hint
Walk backwards from the day you want until you hit a full backup, then slice from there to the day.
@@solution
backups = [
    ("Sun-full", "full", "local"),
    ("Mon-inc", "inc", "local"),
    ("Tue-inc", "inc", "local"),
    ("Wed-inc", "inc", "local"),
    ("Thu-inc", "inc", "offsite"),
    ("Fri-inc", "inc", "local"),
]

def restore_set(backups, day):
    names = [b[0] for b in backups]
    end = names.index(day)
    start = None
    for i in range(end, -1, -1):
        if backups[i][1] == "full":
            start = i
            break
    if start is None:
        raise ValueError(f"no full backup before {day}")
    return backups[start:end + 1]

for day in ["Sun-full", "Wed-inc", "Fri-inc"]:
    chosen = restore_set(backups, day)
    offsite = any(where == "offsite" for _name, _kind, where in chosen)
    print(f"Restore {day}: " + " + ".join(name for name, _k, _w in chosen) + f" (off-site copy: {'yes' if offsite else 'no'})")
try:
    restore_set(backups[1:], "Tue-inc")
except ValueError as e:
    print("Error:", e)

@@exercise Redact personal data from a ticket
Tickets get shared with vendors, so personal data must go first. Write `redact(text)` that replaces, using regular expressions:

- email addresses with `[email]`;
- phone numbers like `090-1234-5678` or `+81 3 1234 5678` with `[phone]`;
- card numbers (13 to 16 digits, maybe split by spaces or dashes) with `[card ending 1234]`, keeping the last four digits;
- IPv4 addresses are **not** personal here, so leave them.

Print each ticket before and after, and how many items were redacted.
@@lang python
@@starter
import re

EMAIL = re.compile(r"[\w.+-]+@[\w-]+(?:\.[\w-]+)+")
CARD = re.compile(r"\b(?:\d[ -]?){12,15}\d\b")
PHONE = re.compile(r"(?:\+\d{1,3}[ -]?)?\(?0?\d{1,4}\)?[ -]\d{3,4}[ -]\d{4}\b")
tickets = [
    "User aiko.tanaka@example.co.jp cannot print. Call her on 090-1234-5678.",
    "Refund to card 4111 1111 1111 1111 failed, customer ben@example.com, PC at 10.0.4.17",
    "Caller from +81 3 1234 5678 asked about invoice 2026-118, card 5500-0055-5555-5559.",
    "Laptop 192.168.1.20 won't boot.",
]

def redact(text):
    raise NotImplementedError


for t in tickets:
    clean, n = redact(t)
    print(f"before: {t}")
    print(f"after:  {clean}  ({n} redacted)")
@@expected
before: User aiko.tanaka@example.co.jp cannot print. Call her on 090-1234-5678.
after:  User [email] cannot print. Call her on [phone].  (2 redacted)
before: Refund to card 4111 1111 1111 1111 failed, customer ben@example.com, PC at 10.0.4.17
after:  Refund to card [card ending 1111] failed, customer [email], PC at 10.0.4.17  (2 redacted)
before: Caller from +81 3 1234 5678 asked about invoice 2026-118, card 5500-0055-5555-5559.
after:  Caller from [phone] asked about invoice 2026-118, card [card ending 5559].  (2 redacted)
before: Laptop 192.168.1.20 won't boot.
after:  Laptop 192.168.1.20 won't boot.  (0 redacted)
@@requires
def redact
re.compile(
.subn(
@@hint
Redact card numbers first, so their digit groups aren't mistaken for phone numbers. `pattern.subn(replacement, text)` returns the new text and how many replacements it made; the replacement can be a function that receives the match.
@@solution
import re

EMAIL = re.compile(r"[\w.+-]+@[\w-]+(?:\.[\w-]+)+")
CARD = re.compile(r"\b(?:\d[ -]?){12,15}\d\b")
PHONE = re.compile(r"(?:\+\d{1,3}[ -]?)?\(?0?\d{1,4}\)?[ -]\d{3,4}[ -]\d{4}\b")
tickets = [
    "User aiko.tanaka@example.co.jp cannot print. Call her on 090-1234-5678.",
    "Refund to card 4111 1111 1111 1111 failed, customer ben@example.com, PC at 10.0.4.17",
    "Caller from +81 3 1234 5678 asked about invoice 2026-118, card 5500-0055-5555-5559.",
    "Laptop 192.168.1.20 won't boot.",
]

def redact(text):
    count = 0
    def card(m):
        digits = re.sub(r"\D", "", m.group())
        return f"[card ending {digits[-4:]}]"
    text, n = CARD.subn(card, text); count += n
    text, n = EMAIL.subn("[email]", text); count += n
    text, n = PHONE.subn("[phone]", text); count += n
    return text, count

for t in tickets:
    clean, n = redact(t)
    print(f"before: {t}")
    print(f"after:  {clean}  ({n} redacted)")

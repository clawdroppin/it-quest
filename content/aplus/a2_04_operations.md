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

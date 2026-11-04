@@lesson a2-winadmin Windows Administration Tools in Practice
@@text
Core 2 domain 1 (operating systems, about 28%) expects you to choose the right Windows tool for a task: which console shows why the PC crashed last night, which one schedules a script, which command repairs system files. This lesson organises the tools by the job you're doing.

## Microsoft Management Console snap-ins

Open them from Run (**Win+R**) by name:

| Tool | Run command | Use it to |
|---|---|---|
| **Event Viewer** | `eventvwr.msc` | read System, Application and Security logs; find crash causes |
| **Disk Management** | `diskmgmt.msc` | initialise, partition, format, assign letters, extend/shrink volumes |
| **Task Scheduler** | `taskschd.msc` | run programs/scripts on a schedule or at events (logon, startup) |
| **Device Manager** | `devmgmt.msc` | drivers: update, roll back, disable; spot unknown devices (yellow !) |
| **Certificate Manager** | `certmgr.msc` | user certificates (VPN, Wi-Fi, S/MIME) |
| **Local Users and Groups** | `lusrmgr.msc` | local accounts (not on Home edition) |
| **Performance Monitor** | `perfmon.msc` | counters over time, data collector sets |
| **Group Policy Editor** | `gpedit.msc` | local policies (Pro and above) |
| **Services** | `services.msc` | start/stop services, startup type, recovery actions |
| **Computer Management** | `compmgmt.msc` | many of the above in one window |

## Other utilities

| Tool | Run command | Use it to |
|---|---|---|
| **System Information** | `msinfo32` | hardware, BIOS version, drivers, all in one report |
| **Resource Monitor** | `resmon` | which process uses CPU, disk, network, memory right now |
| **System Configuration** | `msconfig` | boot options, safe boot, selective startup (startup apps moved to Task Manager) |
| **Disk Cleanup** | `cleanmgr` | delete temp files, old updates |
| **Disk Defragment / Optimize Drives** | `dfrgui` | defragment HDDs, TRIM SSDs |
| **Registry Editor** | `regedit` | edit the registry (back it up first; mistakes can break Windows) |
| **Task Manager** | Ctrl+Shift+Esc | processes, performance, startup apps, users, services |
| **Remote Desktop** | `mstsc` | connect to another PC with RDP |
| **DirectX Diagnostic** | `dxdiag` | graphics and sound info |

## Command-line tools (Core 2 lists these)

| Command | Does |
|---|---|
| `ipconfig /all`, `/release`, `/renew`, `/flushdns` | IP configuration |
| `ping`, `tracert`, `pathping`, `nslookup`, `netstat` | network tests |
| `hostname`, `whoami` | identity |
| `net user`, `net use`, `net localgroup` | accounts, mapped drives, groups |
| `gpupdate /force` | apply Group Policy now |
| `gpresult /r` | which policies apply to this user/computer |
| `sfc /scannow` | **System File Checker**: repair protected system files |
| `DISM /Online /Cleanup-Image /RestoreHealth` | repair the Windows image that SFC uses |
| `chkdsk /f /r` | fix file system errors, find bad sectors |
| `diskpart` | command-line disk partitioning |
| `format` | format a volume |
| `robocopy`, `xcopy`, `copy` | copy files (robocopy: robust, resumable, mirrors) |
| `shutdown /r /t 0` | restart now (`/s` shutdown, `/a` abort) |
| `winver` | Windows version |
| `cd`, `dir`, `md`, `rmdir` | navigation |
| `bootrec /fixmbr`, `/fixboot`, `/rebuildbcd` | repair boot problems (from Windows RE) |

Repair order for a corrupted Windows: `DISM ... /RestoreHealth` first (fixes the component store), then `sfc /scannow`.

## Event Viewer in practice

Logs: **Application** (programs), **System** (Windows components, drivers, services), **Security** (logons, auditing). Levels: Information, Warning, **Error**, **Critical**. Useful Event IDs:

| ID | Log | Means |
|---|---|---|
| **41** | System | Kernel-Power: the system rebooted without cleanly shutting down (crash or power loss) |
| **6008** | System | unexpected shutdown |
| **1001** | System/Application | BugCheck (BSOD details) / Windows Error Reporting |
| **7036** | System | a service started or stopped |
| **4624 / 4625** | Security | successful / failed logon |
| **4740** | Security | account locked out (on domain controllers) |

Filter the current log (right panel → Filter Current Log) by level and time. Create **custom views** for things you check often.

## Task Manager tabs

- **Processes**: sort by CPU, memory, disk, network; end task.
- **Performance**: CPU, memory, disk, network, GPU graphs; uptime.
- **App history**, **Startup apps** (disable to speed up boot; "startup impact" column), **Users**, **Details** (PID, priority), **Services**.

## Control Panel and Settings items for the exam

Internet Options (proxy, security zones, cache), Devices and Printers, Programs and Features (uninstall, Windows features like Hyper-V), Network and Sharing Center, System (computer name, domain join, remote settings, advanced: performance and virtual memory), Windows Defender Firewall, Mail (Outlook profiles), Sound, User Accounts, Device Manager, Indexing Options, Administrative Tools, File Explorer Options (show hidden files and extensions), Power Options (sleep, hibernate, fast startup, lid actions), Ease of Access/Accessibility.

@@quiz
A PC rebooted unexpectedly last night. Which tool and Event ID help confirm it?
* Event Viewer, System log, Event ID 41 (Kernel-Power)
- Task Manager, Startup tab
- Disk Management
- Device Manager
? 41 = rebooted without a clean shutdown.

@@quiz
`sfc /scannow` reports it found corrupt files but couldn't fix some. What should you run next?
- `chkdsk /f`
* `DISM /Online /Cleanup-Image /RestoreHealth`, then `sfc /scannow` again
- `format c:`
- `gpupdate /force`
? DISM repairs the component store SFC uses.

@@quiz
A new policy was set on the domain but a user's PC doesn't have it yet. What command applies it immediately?
- `gpresult /r`
* `gpupdate /force`
- `ipconfig /renew`
- `net use`
? `gpresult /r` shows what's applied.

@@quiz
Which tool shows which process is using the network and disk right now, per file and connection?
- msinfo32
* Resource Monitor (resmon)
- Disk Cleanup
- Certificate Manager
? Task Manager shows totals; Resource Monitor shows details.

@@quiz
Where do you disable programs that start automatically with Windows 10/11?
- msconfig Startup tab
* Task Manager → Startup apps (or Settings → Apps → Startup)
- Device Manager
- Disk Management
? msconfig's Startup tab now points to Task Manager.

@@flashcards
eventvwr.msc :: Event Viewer
diskmgmt.msc :: Disk Management
taskschd.msc :: Task Scheduler
devmgmt.msc :: Device Manager
certmgr.msc :: certificates
lusrmgr.msc :: local users and groups
perfmon.msc :: Performance Monitor
gpedit.msc :: local Group Policy
msinfo32 :: System Information
resmon :: Resource Monitor
msconfig :: boot options, safe boot
regedit :: Registry Editor
sfc /scannow :: repair system files
DISM /RestoreHealth :: repair the Windows image
gpupdate /force :: apply Group Policy now
gpresult /r :: show applied policies
chkdsk /f /r :: fix file system errors and find bad sectors
robocopy :: robust file copying
Event ID 41 :: Kernel-Power unexpected reboot
Event ID 4625 :: failed logon

@@drill
Which Run command opens Device Manager?
= devmgmt.msc
? Compare diskmgmt.msc.

@@drill
Which Security log Event ID records a failed logon?
= 4625
? 4624 is a successful logon.

@@exercise Which tool for the job?
Map each task to the right tool using keyword rules in `TOOLS` (checked in order; the first rule whose keyword appears in the lowercase task wins). Print `task -> tool (command)`, and for unmatched tasks `-> ask a colleague`.
@@lang python
@@starter
TOOLS = [
    ("driver", "Device Manager", "devmgmt.msc"),
    ("crash", "Event Viewer", "eventvwr.msc"),
    ("log", "Event Viewer", "eventvwr.msc"),
    ("partition", "Disk Management", "diskmgmt.msc"),
    ("schedule", "Task Scheduler", "taskschd.msc"),
    ("certificate", "Certificate Manager", "certmgr.msc"),
    ("bios version", "System Information", "msinfo32"),
    ("safe boot", "System Configuration", "msconfig"),
    ("system files", "System File Checker", "sfc /scannow"),
    ("policy", "Group Policy result", "gpresult /r"),
]
tasks = [
    "Roll back the graphics driver after an update",
    "Find out why the PC crashed at 03:00",
    "Create a partition on the new disk",
    "Schedule the backup script for 22:00 every night",
    "Check the BIOS version before a firmware update",
    "Boot into safe boot next restart",
    "Repair corrupted system files",
    "See which policy applies to this user",
    "Order more toner",
]
@@expected
Roll back the graphics driver after an update -> Device Manager (devmgmt.msc)
Find out why the PC crashed at 03:00 -> Event Viewer (eventvwr.msc)
Create a partition on the new disk -> Disk Management (diskmgmt.msc)
Schedule the backup script for 22:00 every night -> Task Scheduler (taskschd.msc)
Check the BIOS version before a firmware update -> System Information (msinfo32)
Boot into safe boot next restart -> System Configuration (msconfig)
Repair corrupted system files -> System File Checker (sfc /scannow)
See which policy applies to this user -> Group Policy result (gpresult /r)
Order more toner -> ask a colleague
@@requires
.lower()
for keyword, tool, command in TOOLS
@@hint
Use a `for ... else` or a variable set to `None` before the inner loop, and `break` on the first match.
@@solution
TOOLS = [
    ("driver", "Device Manager", "devmgmt.msc"),
    ("crash", "Event Viewer", "eventvwr.msc"),
    ("log", "Event Viewer", "eventvwr.msc"),
    ("partition", "Disk Management", "diskmgmt.msc"),
    ("schedule", "Task Scheduler", "taskschd.msc"),
    ("certificate", "Certificate Manager", "certmgr.msc"),
    ("bios version", "System Information", "msinfo32"),
    ("safe boot", "System Configuration", "msconfig"),
    ("system files", "System File Checker", "sfc /scannow"),
    ("policy", "Group Policy result", "gpresult /r"),
]
tasks = [
    "Roll back the graphics driver after an update",
    "Find out why the PC crashed at 03:00",
    "Create a partition on the new disk",
    "Schedule the backup script for 22:00 every night",
    "Check the BIOS version before a firmware update",
    "Boot into safe boot next restart",
    "Repair corrupted system files",
    "See which policy applies to this user",
    "Order more toner",
]

for task in tasks:
    text = task.lower()
    for keyword, tool, command in TOOLS:
        if keyword in text:
            print(f"{task} -> {tool} ({command})")
            break
    else:
        print(f"{task} -> ask a colleague")

@@exercise Summarise Event Viewer exports
An exported System and Security log is given as `(time, log, level, event_id, source)` rows. Print: the number of Errors and Criticals; the times of unexpected reboots (ID 41 or 6008); the failed logon count (4625) and whether it suggests an attack (more than 5 within the export); and the most frequent error source.
@@lang python
@@starter
events = [
    ("02:58:11", "System", "Error", 1001, "BugCheck"),
    ("02:58:40", "System", "Critical", 41, "Kernel-Power"),
    ("02:58:41", "System", "Error", 6008, "EventLog"),
    ("07:30:02", "System", "Information", 7036, "Service Control Manager"),
    ("08:01:15", "Security", "Information", 4625, "Security"),
    ("08:01:19", "Security", "Information", 4625, "Security"),
    ("08:01:23", "Security", "Information", 4625, "Security"),
    ("08:01:27", "Security", "Information", 4625, "Security"),
    ("08:01:31", "Security", "Information", 4625, "Security"),
    ("08:01:35", "Security", "Information", 4625, "Security"),
    ("08:02:00", "Security", "Information", 4624, "Security"),
    ("09:14:52", "System", "Error", 7000, "Service Control Manager"),
    ("09:15:10", "System", "Error", 7000, "Service Control Manager"),
]
@@expected
Errors and criticals: 5
Unexpected reboot events at: 02:58:40, 02:58:41
Failed logons: 6 (possible password guessing)
Most frequent error source: Service Control Manager (2)
@@requires
Counter(
4625
@@hint
`errors = [e for e in events if e[2] in ("Error", "Critical")]`. `Counter(e[4] for e in errors).most_common(1)[0]`.
@@solution
from collections import Counter

events = [
    ("02:58:11", "System", "Error", 1001, "BugCheck"),
    ("02:58:40", "System", "Critical", 41, "Kernel-Power"),
    ("02:58:41", "System", "Error", 6008, "EventLog"),
    ("07:30:02", "System", "Information", 7036, "Service Control Manager"),
    ("08:01:15", "Security", "Information", 4625, "Security"),
    ("08:01:19", "Security", "Information", 4625, "Security"),
    ("08:01:23", "Security", "Information", 4625, "Security"),
    ("08:01:27", "Security", "Information", 4625, "Security"),
    ("08:01:31", "Security", "Information", 4625, "Security"),
    ("08:01:35", "Security", "Information", 4625, "Security"),
    ("08:02:00", "Security", "Information", 4624, "Security"),
    ("09:14:52", "System", "Error", 7000, "Service Control Manager"),
    ("09:15:10", "System", "Error", 7000, "Service Control Manager"),
]

errors = [e for e in events if e[2] in ("Error", "Critical")]
print("Errors and criticals:", len(errors))
reboots = [e[0] for e in events if e[3] in (41, 6008)]
print("Unexpected reboot events at:", ", ".join(reboots))
failed = sum(1 for e in events if e[3] == 4625)
print(f"Failed logons: {failed}" + (" (possible password guessing)" if failed > 5 else ""))
source, count = Counter(e[4] for e in errors).most_common(1)[0]
print(f"Most frequent error source: {source} ({count})")

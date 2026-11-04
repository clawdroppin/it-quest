@@lesson a2-scripting Scripting Basics and Remote Access Technologies
@@text
Core 2 domain 4 (operational procedures) includes recognising script file types, knowing what scripts are used for and the risks they bring, and choosing a remote access method. You don't need to write complex scripts for the exam, but you must recognise them, and on the job a small script saves hours.

## Script file types

| Extension | Language | Runs on |
|---|---|---|
| **.bat** / .cmd | Windows batch | Windows command prompt |
| **.ps1** | PowerShell | Windows (and PowerShell 7 on Linux/macOS) |
| **.vbs** | VBScript | Windows Script Host (being deprecated) |
| **.sh** | shell script (bash/zsh) | Linux, macOS |
| **.js** | JavaScript | browsers, Node.js, Windows Script Host |
| **.py** | Python | anywhere Python is installed |

## What scripts are used for

- **Basic automation**: create users, map drives, install software, clean temp files.
- **Restarting machines** or services on a schedule.
- **Remapping network drives** and **installing applications** at logon.
- **Automated backups** and copying files.
- **Gathering information/data**: inventory, disk space reports, log collection.
- **Initiating updates**.

## Script risks

- **Unintentionally introducing malware** (copying scripts from the internet without reading them).
- **Inadvertently changing system settings** (a typo in a path deletes the wrong folder).
- **Browser or system crashes** due to mishandled resources (infinite loops, memory use).
- Mitigations: read before running, test on one machine, **least privilege**, **code signing** and PowerShell **execution policy** (`AllSigned`, `RemoteSigned`), version control and peer review, logging.

## Scripting concepts the exam names

- **Variables** (`$name = "Ana"` in PowerShell, `name="Ana"` in bash)
- **Loops** (`foreach`, `for`, `while`)
- **Branch logic** (`if/else`)
- **Comments** (`#` in PowerShell, bash, Python; `REM` or `::` in batch; `'` in VBScript; `//` in JavaScript)
- **Environment variables** (`%USERNAME%` in batch, `$env:USERNAME` in PowerShell, `$HOME` in bash)
- **Basic data types**: integers, strings

## Remote access technologies

| Technology | Port | Use | Security notes |
|---|---|---|---|
| **RDP** | 3389 | full Windows desktop | never expose to the internet; VPN or gateway, NLA, MFA |
| **VNC** | 5900 | cross-platform screen sharing | weak auth by default; tunnel it |
| **SSH** | 22 | Linux/network device command line | key authentication, disable root login |
| **Telnet** | 23 | legacy CLI | **plaintext**: replace with SSH |
| **VPN** | various | encrypted access to the internal network | MFA, split vs full tunnel |
| **RMM** | vendor | monitor and manage many endpoints, scripts, patching | a compromised RMM = all clients compromised |
| **Screen sharing / remote assistance** | vendor | help a user live (Quick Assist, **MSRA**: Microsoft Remote Assistance) | user consent, verify identity |
| **Desktop management (Intune, Configuration Manager)** | | policies, software | |
| **File transfer software** | SFTP 22, FTPS 990 | move files securely | not plain FTP (21) |
| **Video conferencing** | | screen sharing in meetings | |

**Security considerations for every remote access tool**: who can connect, MFA, logging and session recording, patched software, the user's consent for attended sessions, and remembering that scammers use the same tools ("Hi, I'm from Microsoft, please install AnyDesk").

## A small PowerShell example

```powershell,norun
# Report free space on every fixed drive and warn under 10%
foreach ($d in Get-CimInstance Win32_LogicalDisk -Filter "DriveType=3") {
    $pct = [math]::Round($d.FreeSpace / $d.Size * 100, 1)
    if ($pct -lt 10) { Write-Warning "$($d.DeviceID) only $pct% free" }
    else { "$($d.DeviceID) $pct% free" }
}
```

The same idea in bash: `df -h | awk 'NR>1 && $5+0 > 90 {print $6 " is " $5 " full"}'`.

@@quiz
A file named `install_printer.ps1` was sent by a colleague. What language is it?
- Batch
* PowerShell
- VBScript
- Python
? .bat is batch, .vbs is VBScript, .py is Python.

@@quiz
Which PowerShell execution policy lets local scripts run but requires downloaded scripts to be signed?
- Restricted
* RemoteSigned
- Unrestricted
- Bypass
? AllSigned requires every script to be signed.

@@quiz
Which remote access method should replace Telnet on network devices?
* SSH
- RDP
- VNC
- FTP
? Telnet sends everything, including passwords, in plaintext.

@@quiz
A small business wants to manage patches and run scripts on 200 client PCs from one console. Which technology fits?
- VNC
- Telnet
* An RMM tool
- Video conferencing
? Remote monitoring and management.

@@quiz
What's a risk of running a script copied from a forum?
- It always fails on Windows
* It can introduce malware or change system settings unintentionally
- It disables UAC permanently
- It can only run as a standard user
? Read before running; test on one machine.

@@flashcards
.bat :: Windows batch
.ps1 :: PowerShell
.vbs :: VBScript
.sh :: shell script
.js :: JavaScript
.py :: Python
RemoteSigned :: downloaded scripts must be signed
AllSigned :: every script must be signed
REM / :: :: comments in batch
$env:USERNAME :: environment variable in PowerShell
%USERNAME% :: environment variable in batch
RDP :: 3389
VNC :: 5900
SSH :: 22
Telnet :: 23, plaintext
MSRA :: Microsoft Remote Assistance
RMM :: remote monitoring and management

@@drill
What port does VNC use by default?
= 5900
? RDP is 3389.

@@drill
What file extension do Linux shell scripts usually use?
= .sh
= sh
? Plus a `#!/bin/bash` shebang line and execute permission.

@@exercise Identify scripts and their comment syntax
For each file name, print its language and how to write a comment in it, using `LANGS`. Unknown extensions print `unknown: do not run`. The extension check must be case-insensitive.
@@lang python
@@starter
LANGS = {
    ".bat": ("Batch", "REM"),
    ".cmd": ("Batch", "REM"),
    ".ps1": ("PowerShell", "#"),
    ".vbs": ("VBScript", "'"),
    ".sh": ("Shell", "#"),
    ".js": ("JavaScript", "//"),
    ".py": ("Python", "#"),
}
files = ["MapDrives.BAT", "cleanup.ps1", "logon.vbs", "backup.sh", "invoice.pdf.js", "report.py", "setup.exe"]
@@expected
MapDrives.BAT: Batch, comment with REM
cleanup.ps1: PowerShell, comment with #
logon.vbs: VBScript, comment with '
backup.sh: Shell, comment with #
invoice.pdf.js: JavaScript, comment with //
report.py: Python, comment with #
setup.exe: unknown: do not run
@@requires
os.path.splitext(
.lower()
@@hint
`ext = os.path.splitext(name)[1].lower()` takes only the last extension, which is what Windows runs: `invoice.pdf.js` is JavaScript.
@@solution
import os

LANGS = {
    ".bat": ("Batch", "REM"),
    ".cmd": ("Batch", "REM"),
    ".ps1": ("PowerShell", "#"),
    ".vbs": ("VBScript", "'"),
    ".sh": ("Shell", "#"),
    ".js": ("JavaScript", "//"),
    ".py": ("Python", "#"),
}
files = ["MapDrives.BAT", "cleanup.ps1", "logon.vbs", "backup.sh", "invoice.pdf.js", "report.py", "setup.exe"]

for name in files:
    ext = os.path.splitext(name)[1].lower()
    if ext in LANGS:
        lang, comment = LANGS[ext]
        print(f"{name}: {lang}, comment with {comment}")
    else:
        print(f"{name}: unknown: do not run")

@@exercise Review remote access requests
Each request asks for a remote access method. Approve it only if the method is not banned (`Telnet`, plain `FTP`), it's not exposed directly to the internet (`exposed` true is only allowed for `VPN`), and MFA is on when the user is external. Print `APPROVED` or `DENIED` with all reasons, then suggest the replacement from `REPLACE` for banned methods.
@@lang python
@@starter
BANNED = {"Telnet", "FTP"}
REPLACE = {"Telnet": "SSH", "FTP": "SFTP"}
requests = [
    {"user": "vendor-hvac", "method": "RDP", "exposed": True, "external": True, "mfa": False},
    {"user": "netadmin", "method": "Telnet", "exposed": False, "external": False, "mfa": True},
    {"user": "sales-remote", "method": "VPN", "exposed": True, "external": True, "mfa": True},
    {"user": "dev-ops", "method": "SSH", "exposed": False, "external": True, "mfa": True},
    {"user": "printer-vendor", "method": "FTP", "exposed": True, "external": True, "mfa": False},
]
@@expected
vendor-hvac RDP: DENIED: exposed to the internet; external without MFA
netadmin Telnet: DENIED: Telnet is banned (use SSH)
sales-remote VPN: APPROVED
dev-ops SSH: APPROVED
printer-vendor FTP: DENIED: FTP is banned (use SFTP); exposed to the internet; external without MFA
@@requires
BANNED
REPLACE[
@@hint
Collect reasons in a list; a request is approved only if the list is empty.
@@solution
BANNED = {"Telnet", "FTP"}
REPLACE = {"Telnet": "SSH", "FTP": "SFTP"}
requests = [
    {"user": "vendor-hvac", "method": "RDP", "exposed": True, "external": True, "mfa": False},
    {"user": "netadmin", "method": "Telnet", "exposed": False, "external": False, "mfa": True},
    {"user": "sales-remote", "method": "VPN", "exposed": True, "external": True, "mfa": True},
    {"user": "dev-ops", "method": "SSH", "exposed": False, "external": True, "mfa": True},
    {"user": "printer-vendor", "method": "FTP", "exposed": True, "external": True, "mfa": False},
]

for r in requests:
    reasons = []
    if r["method"] in BANNED:
        reasons.append(f"{r['method']} is banned (use {REPLACE[r['method']]})")
    if r["exposed"] and r["method"] != "VPN":
        reasons.append("exposed to the internet")
    if r["external"] and not r["mfa"]:
        reasons.append("external without MFA")
    if reasons:
        print(f"{r['user']} {r['method']}: DENIED: " + "; ".join(reasons))
    else:
        print(f"{r['user']} {r['method']}: APPROVED")

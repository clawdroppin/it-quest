@@lesson a2-windows Windows: Editions, Tools and Commands
@@text
## Editions

| Edition | Key differences |
|---|---|
| **Home** | can't join a domain, no Group Policy editor, no BitLocker (only "device encryption"), can't host Remote Desktop |
| **Pro** | **domain join**, **BitLocker**, **gpedit.msc**, Remote Desktop host, Hyper-V |
| **Enterprise / Education** | volume licensing, extra security features, managed by IT |

Windows 11 needs a 64-bit CPU (1 GHz, 2+ cores, on the supported list), **4 GB RAM**, **64 GB** storage, **UEFI with Secure Boot** and **TPM 2.0**. Upgrade paths: Home → Pro is a key change; a 32-bit Windows can't be upgraded in place to 64-bit.

## The tools you must know

| Command (Win+R) | Tool | Use |
|---|---|---|
| `taskmgr` | Task Manager | processes, performance, **startup apps** |
| `msconfig` | System Configuration | boot options, **safe boot** |
| `eventvwr.msc` | Event Viewer | System, Application and Security logs |
| `devmgmt.msc` | Device Manager | drivers, roll back a driver, disabled devices |
| `diskmgmt.msc` | Disk Management | partitions, initialize disks, drive letters |
| `services.msc` | Services | start/stop, startup type |
| `taskschd.msc` | Task Scheduler | run tasks on a schedule or event |
| `lusrmgr.msc` | Local Users and Groups | local accounts (not on Home) |
| `gpedit.msc` | Local Group Policy Editor | policies (Pro and up) |
| `perfmon` / `resmon` | Performance / Resource Monitor | detailed performance data |
| `msinfo32` | System Information | hardware and software summary |
| `regedit` | Registry Editor | settings database; back it up before editing |
| `certmgr.msc` | Certificate Manager | user certificates |
| `compmgmt.msc` | Computer Management | many of the above in one window |

## Command-line tools

```powershell,norun
ipconfig /all                 # IP settings; /release, /renew, /flushdns
ping, tracert, nslookup       # connectivity and DNS
netstat -ano                  # connections and listening ports, with process IDs
hostname / whoami / winver    # name, current user, Windows version
chkdsk C: /f /r               # check and repair the file system and bad sectors
sfc /scannow                  # repair protected system files
DISM /Online /Cleanup-Image /RestoreHealth   # repair the Windows image (run before sfc if sfc fails)
gpupdate /force               # apply Group Policy now
gpresult /r                   # which policies applied
net use Z: \\fs01\share       # map a drive; net user for accounts
robocopy src dst /MIR         # robust copy / mirror (careful: /MIR deletes extras in dst)
diskpart / format             # partitions and formatting
shutdown /r /t 0              # restart now
```

## Settings and accounts

- **Microsoft account** vs **local account** vs **domain account** (Active Directory, or **Entra ID** for cloud-joined PCs).
- A **workgroup** is peer-to-peer (each PC has its own accounts); a **domain** has central accounts and policies, managed by domain controllers.
- **UAC** prompts before admin changes. **Windows Defender Firewall** and **Microsoft Defender Antivirus** are built in.
- **System Restore** rolls back system files and settings to a **restore point** without touching personal files.
- Recovery options: Safe Mode, Startup Repair, **Reset this PC** (keep files or remove everything), boot from install media.

@@quiz
A small company bought Windows 11 Home laptops and now can't join them to their Active Directory domain. What's needed?
* Upgrade to Windows 11 Pro (or Enterprise)
- Enable BitLocker
- Run gpupdate /force
- Create a local account
? Home editions can't join a domain.

@@quiz
After a graphics driver update, a PC shows a distorted screen. What's the quickest fix in Device Manager?
* Roll back the driver
- Uninstall the monitor
- Disable the network adapter
- Run chkdsk
? Driver rollback restores the previous version.

@@quiz
`sfc /scannow` reports it found corrupt files but couldn't fix some of them. What should you run next?
* DISM /Online /Cleanup-Image /RestoreHealth, then sfc again
- chkdsk /f only
- format C:
- gpupdate /force
? DISM repairs the component store that sfc uses as its source.

@@quiz
A PC takes ages to start, and many apps appear in the system tray. Where do you disable startup apps on Windows 10/11?
* Task Manager → Startup apps (or Settings → Apps → Startup)
- msconfig → Startup
- Event Viewer
- Disk Management
? msconfig's Startup tab now just points to Task Manager.

@@quiz
You suspect a program is communicating with a strange IP. Which command shows active connections and their process IDs?
* netstat -ano
- ipconfig /all
- nslookup
- tracert
? -a all connections, -n numeric, -o owning process ID (match it in Task Manager).

@@quiz
Which tool shows why an application crashed last night?
* Event Viewer (the Application log)
- Device Manager
- Disk Management
- Task Scheduler
? Crashes and errors are logged with time, source and event ID.

@@quiz
A user's new policy settings haven't arrived yet. Which command applies Group Policy immediately?
* gpupdate /force
- gpresult /r
- sfc /scannow
- net use
? gpresult shows what applied; gpupdate applies it.

@@flashcards
Windows Home limits :: No domain join, no gpedit, no BitLocker, no RDP host
Windows 11 requirements :: TPM 2.0, UEFI Secure Boot, 4 GB RAM, 64 GB storage
msconfig :: Boot options, safe boot
eventvwr.msc :: Event Viewer: logs
devmgmt.msc :: Device Manager: drivers
diskmgmt.msc :: Disk Management
services.msc :: Services
taskschd.msc :: Task Scheduler
sfc /scannow :: Repair system files
DISM /RestoreHealth :: Repair the Windows image
chkdsk /f /r :: Fix file system errors and bad sectors
gpupdate /force / gpresult /r :: Apply / show Group Policy
netstat -ano :: Connections with process IDs
System Restore :: Roll back system settings, keeps personal files
workgroup vs domain :: Local accounts per PC vs central accounts and policy

@@drill
Which command repairs protected Windows system files?
= sfc /scannow
? System File Checker.

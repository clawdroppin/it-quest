@@lesson a2-other-os macOS, Linux and Installing Operating Systems
@@text
## macOS essentials

| Feature | What it does |
|---|---|
| **Finder** | file manager |
| **Spotlight** (Cmd+Space) | search |
| **System Settings** | settings (formerly System Preferences) |
| **Keychain** | stores passwords and certificates |
| **Time Machine** | built-in backup to an external disk |
| **FileVault** | full-disk encryption |
| **Disk Utility** | format, repair (First Aid) |
| **Activity Monitor** | like Task Manager |
| **Force Quit** (Cmd+Option+Esc) | kill a frozen app |
| **Terminal** | the shell (zsh) |

Apps install from the **App Store** or as **.dmg** (drag to Applications) or **.pkg** files. Companies manage Macs with MDM (Jamf, Intune).

## Linux essentials

(The Bash & Linux track covers this in depth.)

```bash,norun
ls -l, cd, pwd, cp, mv, rm, mkdir   # files
cat, less, grep, nano / vi          # read, search, edit
sudo apt update && sudo apt upgrade # update (Debian/Ubuntu); dnf on Red Hat/Fedora
chmod, chown                        # permissions and owners
ps, top, kill                       # processes
ip a, ping, dig                     # networking
df -h, du -sh                       # disk usage
```

`sudo` runs a command as **root** (the administrator). File systems: **ext4**, XFS. Logs in `/var/log` or with `journalctl`.

## Installing an OS

| Method | When |
|---|---|
| **USB/DVD** boot media | one-off installs |
| **Network boot (PXE)** | deploying many machines from a server |
| **Image deployment** | clone a standard image (Windows Autopilot, MDT, SCCM/ConfigMgr, Intune) |
| **In-place upgrade** | keep apps and files, move to a new version |
| **Clean install** | wipe and start fresh: most reliable |
| **Repair install** | reinstall the OS keeping files and apps |

Partitioning: **GPT** (modern, UEFI, large disks, up to 128 partitions) vs **MBR** (old, BIOS, max 2 TB, 4 primary partitions). Before upgrading: check **compatibility** (hardware, apps, drivers), **back up**, and have a rollback plan.

**Multiboot** = several OSs on one PC; the boot loader asks which to start. Updates: Windows Update / WSUS / Intune; macOS Software Update; Linux package managers.

@@quiz
A designer's Mac is frozen in one application. How do they close it?
* Force Quit with Cmd+Option+Esc
- Ctrl+Alt+Del
- Open Disk Utility
- Spotlight
? Ctrl+Alt+Del is Windows. Activity Monitor can also end processes.

@@quiz
Which macOS feature backs up files automatically to an external drive?
* Time Machine
- FileVault
- Keychain
- Boot Camp
? FileVault encrypts; Keychain stores passwords.

@@quiz
IT must install Windows on 50 new PCs over the network without USB sticks. What do the PCs need to support?
* PXE network boot
- MBR partitions
- A Type 2 hypervisor
- FAT32
? PXE lets a NIC boot from a deployment server.

@@quiz
Which partition style is required to boot from a 4 TB disk on a modern UEFI PC?
* GPT
- MBR
- FAT32
- exFAT
? MBR is limited to 2 TB and is used with legacy BIOS boot.

@@quiz
On Ubuntu, which command installs available updates?
* sudo apt update && sudo apt upgrade
- sudo dnf install update
- winget upgrade
- sudo yum fetch
? apt for Debian/Ubuntu; dnf (or older yum) for Red Hat family.

@@flashcards
Time Machine :: macOS backup
FileVault :: macOS disk encryption
Keychain :: macOS password store
Activity Monitor :: macOS Task Manager
.dmg / .pkg :: macOS disk image app / installer package
sudo :: Run a Linux command as root
apt / dnf :: Package managers: Debian/Ubuntu / Red Hat/Fedora
PXE :: Boot from the network for deployment
GPT vs MBR :: Modern UEFI, large disks / legacy, 2 TB max
clean install :: Wipe and install fresh
in-place upgrade :: New OS version keeping apps and files

@@drill
Which Linux command shows disk free space in human-readable units?
= df -h
? `du -sh folder` shows how much a folder uses.

@@exercise Translate commands between Windows, macOS and Linux
Technicians often know the Windows command but not the Linux or macOS one. Write `translate(command, target)` using this table (target is `"linux"` or `"macos"`):

| Windows | Linux | macOS |
|---|---|---|
| `ipconfig` | `ip a` | `ifconfig` |
| `dir` | `ls -l` | `ls -l` |
| `tasklist` | `ps aux` | `ps aux` |
| `ping -t` | `ping` | `ping` |
| `chkdsk` | `fsck` | `diskutil verifyVolume` |
| `shutdown /r` | `sudo reboot` | `sudo shutdown -r now` |

Unknown commands return `no direct equivalent`.
@@lang python
@@starter
TABLE = {
    "ipconfig": ("ip a", "ifconfig"),
    "dir": ("ls -l", "ls -l"),
    "tasklist": ("ps aux", "ps aux"),
    "ping -t": ("ping", "ping"),
    "chkdsk": ("fsck", "diskutil verifyVolume"),
    "shutdown /r": ("sudo reboot", "sudo shutdown -r now"),
}
questions = [("ipconfig", "linux"), ("ipconfig", "macos"), ("chkdsk", "macos"), ("shutdown /r", "linux"), ("gpupdate", "linux")]

def translate(command, target):
    raise NotImplementedError


for cmd, target in questions:
    print(f"{cmd} on {target}: {translate(cmd, target)}")
@@expected
ipconfig on linux: ip a
ipconfig on macos: ifconfig
chkdsk on macos: diskutil verifyVolume
shutdown /r on linux: sudo reboot
gpupdate on linux: no direct equivalent
@@requires
def translate
@@hint
Store each row as a tuple `(linux, macos)` and unpack it.
@@solution
TABLE = {
    "ipconfig": ("ip a", "ifconfig"),
    "dir": ("ls -l", "ls -l"),
    "tasklist": ("ps aux", "ps aux"),
    "ping -t": ("ping", "ping"),
    "chkdsk": ("fsck", "diskutil verifyVolume"),
    "shutdown /r": ("sudo reboot", "sudo shutdown -r now"),
}
questions = [("ipconfig", "linux"), ("ipconfig", "macos"), ("chkdsk", "macos"), ("shutdown /r", "linux"), ("gpupdate", "linux")]

def translate(command, target):
    if command not in TABLE:
        return "no direct equivalent"
    linux, macos = TABLE[command]
    return linux if target == "linux" else macos

for cmd, target in questions:
    print(f"{cmd} on {target}: {translate(cmd, target)}")

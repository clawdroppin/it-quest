@@lesson a2-install Installing, Upgrading and Configuring Operating Systems
@@text
Core 2 asks you to plan OS installations and upgrades: boot methods, partition styles, file systems, upgrade paths, editions and what each supports, plus joining a domain or setting up networking afterwards. In the job this is imaging new PCs, reinstalling broken ones and planning upgrades across a fleet.

## Windows editions

| Feature | Home | Pro | Enterprise / Education |
|---|---|---|---|
| Join a domain / Entra ID | no (Microsoft account only) | **yes** | yes |
| Group Policy (gpedit) | no | yes | yes |
| **BitLocker** (full) | no (Device Encryption only) | yes | yes |
| Remote Desktop **host** | no (client only) | yes | yes |
| Hyper-V | no | yes | yes |
| AppLocker, Credential Guard, DirectAccess | no | no (some) | yes |
| Max RAM (64-bit) | 128 GB | 2 TB | 2 TB |

A business PC needs at least **Pro**. Upgrading Home → Pro is an in-place edition upgrade with a Pro key (no reinstall).

## Requirements (Windows 11)

64-bit CPU on Microsoft's supported list, **4 GB RAM**, **64 GB** storage, **UEFI with Secure Boot** capability, **TPM 2.0**, DirectX 12 graphics. Windows 10 reached end of support in October 2025 (paid Extended Security Updates exist).

## Boot and install methods

- **USB** (Media Creation Tool or Rufus), the most common.
- **Network boot (PXE)**: the NIC boots from a deployment server (WDS, Configuration Manager, MDT).
- **Optical** (DVD), **internal recovery partition**, **ISO** mounted in a VM.
- **Image deployment**: capture a reference image and apply it to many PCs (sysprep generalises it first).
- **Windows Autopilot**: a new laptop shipped straight to the user signs into Entra ID and Intune installs everything: no imaging at all.
- **Zero-touch / remote network installs**.

## Installation types

- **Clean install**: wipe and install fresh. Most reliable; data must be backed up and restored.
- **In-place upgrade**: keeps apps, settings and files (Windows 10 → 11, or a repair install).
- **Repair installation**: reinstalls Windows over itself, keeping files and apps.
- **Recovery partition / Reset this PC**: keep my files or remove everything.
- **Multiboot**: several OSs on separate partitions with a boot menu.
- **Image deployment** and **remote network installation** (above).

## Partitioning: MBR vs GPT

| | **MBR** | **GPT** |
|---|---|---|
| Max disk size | 2 TB | ~9.4 ZB (effectively unlimited) |
| Partitions | 4 primary (or 3 + an extended with logical drives) | 128 in Windows |
| Firmware | legacy BIOS | **UEFI** (required for Windows 11 boot) |
| Redundancy | single partition table | backup copy at the end of the disk |

Convert with `mbr2gpt` (Windows, non-destructive for the system disk) or by wiping in `diskpart` (`clean`, `convert gpt`).

## File systems

| File system | Use | Notes |
|---|---|---|
| **NTFS** | Windows system and data drives | permissions, encryption (EFS), compression, journaling, large files |
| **ReFS** | Windows Server storage | resilient, for Storage Spaces |
| **FAT32** | USB drives, UEFI boot partitions | **4 GB max file size**, readable everywhere |
| **exFAT** | large USB drives, SD cards | no 4 GB limit, cross-platform, no permissions |
| **ext4** | Linux | journaling, permissions |
| **XFS**, **Btrfs** | Linux servers, snapshots | |
| **APFS** | macOS, iOS | snapshots, encryption |
| **HFS+** | older macOS | |

Common exam trap: copying a 6 GB video to a FAT32 USB stick fails ("file too large"); reformat as **exFAT** (or NTFS for Windows-only).

## After installing

1. Drivers (chipset, network, graphics) from the vendor or Windows Update.
2. All **updates**; firmware updates.
3. Join the **domain** or **Entra ID** (Settings → Accounts → Access work or school), or a **workgroup** for small offices.
4. Applications (company portal, package manager).
5. Security: antivirus/EDR, BitLocker, firewall, local admin removed.
6. Restore user data (User State Migration Tool `scanstate`/`loadstate`, OneDrive Known Folder Move, or a manual copy).
7. Time zone, region, language packs (a Japanese keyboard layout and IME for staff in Japan).

## Upgrade considerations

- **Back up** user data first.
- **Compatibility**: applications (especially old line-of-business apps), drivers (printers, scanners, VPN clients), hardware requirements. Run the PC Health Check and test a pilot group.
- **Licensing**: the right edition and activation (digital licence, KMS, MAK, or subscription activation through Microsoft 365).
- **Feature updates** (yearly) vs **quality updates** (monthly, "Patch Tuesday": second Tuesday of the month).
- End of life planning: upgrade before support ends.

## macOS and Linux installs (brief)

- macOS: **Recovery** (Command+R at startup on Intel, hold the power button on Apple Silicon) to reinstall; **Time Machine** to restore; Apple Business Manager + MDM for zero-touch.
- Linux: boot from USB, choose partitions (often ext4 with a swap partition or swap file), user and hostname, then `apt update && apt upgrade`.

@@quiz
A small business needs PCs that can join the company domain and use BitLocker. Which edition is the minimum?
- Home
* Pro
- Enterprise
- Education
? Home can't join a domain or use full BitLocker.

@@quiz
A user can't copy a 7 GB video file to a USB stick; the error says the file is too large. Why?
* The stick is formatted FAT32, which has a 4 GB file size limit
- USB sticks can't store video
- The stick is NTFS
- The file is corrupted
? Reformat as exFAT.

@@quiz
Windows 11 must boot from a 4 TB disk. Which partition style is required?
- MBR
* GPT
- FAT32
- Extended
? MBR is limited to 2 TB and legacy boot.

@@quiz
Which installation keeps the user's applications, settings and files?
- Clean install
* In-place upgrade
- Image deployment
- Format and reinstall
? Clean installs wipe everything.

@@quiz
Which technology lets a new laptop shipped directly to an employee configure itself after they sign in?
- PXE boot
- Sysprep
* Windows Autopilot with Intune
- Multiboot
? Zero-touch provisioning.

@@flashcards
Home vs Pro :: Pro adds domain join, BitLocker, RDP host, gpedit, Hyper-V
Windows 11 requirements :: TPM 2.0, Secure Boot, 4 GB RAM, 64 GB storage
PXE :: network boot for deployment
sysprep :: generalises an image before capture
Autopilot :: zero-touch Windows provisioning
clean install :: wipe and install fresh
in-place upgrade :: keeps apps, settings and files
MBR :: 2 TB, 4 primary partitions, legacy BIOS
GPT :: huge disks, 128 partitions, UEFI
FAT32 :: 4 GB file size limit
exFAT :: large USB drives, no permissions
NTFS :: Windows: permissions, EFS, journaling
ext4 / APFS :: Linux / macOS file systems
mbr2gpt :: convert the system disk to GPT
Patch Tuesday :: second Tuesday monthly updates
USMT :: User State Migration Tool

@@drill
What is the maximum file size on FAT32, in GB?
= 4
? Use exFAT or NTFS for bigger files.

@@drill
What is the maximum disk size MBR can address, in TB?
= 2
? GPT removes this limit.

@@exercise Plan the right file system and partition style
Write `plan(device)` that recommends a file system and partition style: Windows system drive → NTFS on GPT; a disk larger than 2 TB → GPT regardless; USB drive used with Macs and Windows → exFAT (FAT32 if it's 32 GB or less and no file is over 4 GB); Linux server → ext4; Mac-only → APFS. Partition style is GPT for disks over 2 TB or system drives, otherwise MBR for removable drives. Print each plan.
@@lang python
@@starter
devices = [
    {"name": "New laptop SSD", "size_gb": 512, "use": "windows-system", "largest_file_gb": 10},
    {"name": "Camera SD card", "size_gb": 128, "use": "cross-platform", "largest_file_gb": 12},
    {"name": "Old 16 GB stick", "size_gb": 16, "use": "cross-platform", "largest_file_gb": 1},
    {"name": "Backup disk", "size_gb": 8000, "use": "linux-server", "largest_file_gb": 500},
    {"name": "Designer's drive", "size_gb": 4000, "use": "mac", "largest_file_gb": 40},
]

def plan(device):
    raise NotImplementedError
@@expected
New laptop SSD: NTFS on GPT
Camera SD card: exFAT on MBR
Old 16 GB stick: FAT32 on MBR
Backup disk: ext4 on GPT
Designer's drive: APFS on GPT
@@requires
def plan
"exFAT"
"GPT"
@@hint
Decide the file system with `if/elif` on `use`, then `style = "GPT" if size > 2000 or use == "windows-system" else "MBR"`. Return a tuple.
@@solution
devices = [
    {"name": "New laptop SSD", "size_gb": 512, "use": "windows-system", "largest_file_gb": 10},
    {"name": "Camera SD card", "size_gb": 128, "use": "cross-platform", "largest_file_gb": 12},
    {"name": "Old 16 GB stick", "size_gb": 16, "use": "cross-platform", "largest_file_gb": 1},
    {"name": "Backup disk", "size_gb": 8000, "use": "linux-server", "largest_file_gb": 500},
    {"name": "Designer's drive", "size_gb": 4000, "use": "mac", "largest_file_gb": 40},
]

def plan(device):
    use, size = device["use"], device["size_gb"]
    if use == "windows-system":
        fs = "NTFS"
    elif use == "cross-platform":
        fs = "FAT32" if size <= 32 and device["largest_file_gb"] <= 4 else "exFAT"
    elif use == "linux-server":
        fs = "ext4"
    else:
        fs = "APFS"
    style = "GPT" if size > 2000 or use == "windows-system" else "MBR"
    return fs, style

for d in devices:
    fs, style = plan(d)
    print(f"{d['name']}: {fs} on {style}")

@@exercise Windows 11 upgrade readiness
Check each PC against the Windows 11 requirements (TPM 2.0, Secure Boot capable, at least 4 GB RAM and 64 GB storage, CPU generation in the supported list) and the business rule (edition must be Pro or Enterprise). Print `READY`, or `BLOCKED` with every reason. A PC whose only problems are that TPM or Secure Boot is **disabled** (but present) is `FIX IN FIRMWARE`. Finish with a count per status.
@@lang python
@@starter
SUPPORTED_GENS = {"intel-8", "intel-10", "intel-12", "intel-13", "amd-zen2", "amd-zen3", "amd-zen4"}
pcs = [
    {"name": "PC-014", "tpm": "2.0", "tpm_on": True, "secure_boot": "on", "ram": 16, "disk": 256, "cpu": "intel-12", "edition": "Pro"},
    {"name": "PC-022", "tpm": "2.0", "tpm_on": False, "secure_boot": "off", "ram": 8, "disk": 256, "cpu": "intel-10", "edition": "Pro"},
    {"name": "PC-031", "tpm": "1.2", "tpm_on": True, "secure_boot": "unsupported", "ram": 4, "disk": 128, "cpu": "intel-6", "edition": "Pro"},
    {"name": "PC-040", "tpm": "2.0", "tpm_on": True, "secure_boot": "on", "ram": 8, "disk": 512, "cpu": "amd-zen3", "edition": "Home"},
]
@@expected
PC-014: READY
PC-022: FIX IN FIRMWARE: enable TPM, enable Secure Boot
PC-031: BLOCKED: TPM 1.2, no Secure Boot, unsupported CPU intel-6
PC-040: BLOCKED: Home edition
BLOCKED: 2
FIX IN FIRMWARE: 1
READY: 1
@@requires
SUPPORTED_GENS
Counter(
@@hint
Keep two lists: `fixable` (TPM present but off, Secure Boot "off") and `blockers` (everything else). Status: blockers → BLOCKED, else fixable → FIX IN FIRMWARE, else READY.
@@solution
from collections import Counter

SUPPORTED_GENS = {"intel-8", "intel-10", "intel-12", "intel-13", "amd-zen2", "amd-zen3", "amd-zen4"}
pcs = [
    {"name": "PC-014", "tpm": "2.0", "tpm_on": True, "secure_boot": "on", "ram": 16, "disk": 256, "cpu": "intel-12", "edition": "Pro"},
    {"name": "PC-022", "tpm": "2.0", "tpm_on": False, "secure_boot": "off", "ram": 8, "disk": 256, "cpu": "intel-10", "edition": "Pro"},
    {"name": "PC-031", "tpm": "1.2", "tpm_on": True, "secure_boot": "unsupported", "ram": 4, "disk": 128, "cpu": "intel-6", "edition": "Pro"},
    {"name": "PC-040", "tpm": "2.0", "tpm_on": True, "secure_boot": "on", "ram": 8, "disk": 512, "cpu": "amd-zen3", "edition": "Home"},
]

statuses = Counter()
for pc in pcs:
    blockers, fixable = [], []
    if pc["tpm"] != "2.0":
        blockers.append(f"TPM {pc['tpm']}")
    elif not pc["tpm_on"]:
        fixable.append("enable TPM")
    if pc["secure_boot"] == "unsupported":
        blockers.append("no Secure Boot")
    elif pc["secure_boot"] == "off":
        fixable.append("enable Secure Boot")
    if pc["ram"] < 4:
        blockers.append(f"{pc['ram']} GB RAM")
    if pc["disk"] < 64:
        blockers.append(f"{pc['disk']} GB disk")
    if pc["cpu"] not in SUPPORTED_GENS:
        blockers.append(f"unsupported CPU {pc['cpu']}")
    if pc["edition"] not in ("Pro", "Enterprise"):
        blockers.append(f"{pc['edition']} edition")
    if blockers:
        status = "BLOCKED"
        detail = ": " + ", ".join(blockers + fixable)
    elif fixable:
        status = "FIX IN FIRMWARE"
        detail = ": " + ", ".join(fixable)
    else:
        status, detail = "READY", ""
    statuses[status] += 1
    print(f"{pc['name']}: {status}{detail}")

for status, count in sorted(statuses.items()):
    print(f"{status}: {count}")

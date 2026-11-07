@@lesson a1-storage Storage Devices, RAID and Removable Media
@@text
Storage questions are everywhere in Core 1: choosing a drive for a scenario, knowing interface speeds, calculating RAID capacity, and recognising a dying drive. In the job, storage is where users' data lives, so mistakes are expensive.

## Drive types

| Type | How it works | Speed (sequential) | Notes |
|---|---|---|---|
| **HDD** | spinning magnetic platters + moving heads | 80–250 MB/s | cheap per TB; RPM: 5,400 (laptops), **7,200** (desktops), 10,000/**15,000** (old enterprise) |
| **SATA SSD** | flash memory over SATA III | ~550 MB/s (the SATA limit) | 2.5" or M.2 SATA |
| **NVMe SSD** | flash memory over PCIe | 3,500 (PCIe 3.0 x4) to 14,000 MB/s (PCIe 5.0) | M.2, U.2, add-in cards |
| **Hybrid (SSHD)** | HDD with a small flash cache | between | rare now |

- **Form factors**: 3.5" (desktop HDD), 2.5" (laptop HDD/SSD), **M.2** (2242, 2260, **2280** = 22 mm wide, 80 mm long), mSATA (old).
- **M.2 keys**: **M key** (PCIe x4, NVMe), **B key** (SATA or PCIe x2), **B+M** (fits both, usually SATA). An M.2 SATA SSD in a slot that only supports NVMe won't be detected.
- **Interfaces**: SATA III 6 Gbps, **SAS** (Serial Attached SCSI, servers, 12–24 Gbps, hot-swap), PCIe/NVMe, and external USB/Thunderbolt.
- SSDs have no moving parts: silent, shock-resistant, low power. Flash cells wear with writes (**TBW**, terabytes written, rating); **TRIM** keeps performance up; never defragment an SSD (Windows "Optimize" sends TRIM instead).

## RAID

**RAID** (Redundant Array of Independent Disks) combines drives for speed, redundancy or both. **RAID is not a backup.**

| Level | Min disks | How | Usable capacity | Survives |
|---|---|---|---|---|
| **0** | 2 | striping | 100% (n × size) | **no** disk failure |
| **1** | 2 | mirroring | 50% (one disk's worth) | 1 failure |
| **5** | 3 | striping + distributed parity | (n − 1) × size | 1 failure |
| **6** | 4 | striping + double parity | (n − 2) × size | 2 failures |
| **10 (1+0)** | 4 | mirrored pairs, striped | 50% | 1 per mirror pair |

- RAID 0 is fast but one failure loses everything (scratch space, video editing caches).
- RAID 5 has a **write penalty** (parity calculation) and a risky **rebuild**: while it rebuilds onto a replacement, a second failure (more likely with big, old disks under heavy load) loses the array. That's why RAID 6 or RAID 10 is preferred for large drives.
- **Hardware RAID** (a controller card with cache and battery) vs **software RAID** (Windows Storage Spaces, Linux mdadm, ZFS).
- **Hot spare**: an idle disk that automatically replaces a failed one. **Hot-swap**: replacing a drive while the system runs.

## Removable and optical media

- **Flash drives** and **memory cards**: SD, microSD, (older) CompactFlash. SD speed classes: Class 10, UHS-I/II (U1/U3), V30/V60/V90 for video.
- **Optical**: CD (700 MB), DVD (4.7 GB single layer, 8.5 GB dual layer), Blu-ray (25 GB single, 50 GB dual). Still used for archives and some legacy installs.
- **External drives** over USB, Thunderbolt or eSATA (legacy).
- **NAS** (network-attached storage: a file server appliance, shared over SMB/NFS) vs **SAN** (storage area network: block storage over Fibre Channel or iSCSI for servers).

## Recognising a failing drive

| Symptom | Likely |
|---|---|
| Clicking or grinding noise | HDD mechanical failure: back up immediately |
| **S.M.A.R.T.** warning at boot or in tools | drive predicting its own failure (reallocated sectors, pending sectors) |
| Very slow, freezing on file access | bad sectors, failing drive, or a full disk |
| "No boot device found" / "Operating system not found" | drive not detected, boot order wrong, or the boot sector/partition damaged |
| Data corruption, files that won't open | bad sectors, failing controller, or file system errors (`chkdsk /f`) |
| Drive missing in File Explorer but in Disk Management | no drive letter, or uninitialised/unpartitioned |
| RAID degraded / failed array | a member disk failed; replace it, let it rebuild |
| Extended read/write times, overheating | failing SSD or HDD, poor airflow |

Tools: `chkdsk`, the vendor's diagnostic utility, S.M.A.R.T. readers (CrystalDiskInfo), `wmic diskdrive get status` (older), `Get-PhysicalDisk` in PowerShell, Disk Management (`diskmgmt.msc`).

## Choosing storage for a scenario

- Fastest boot and app loading in a new PC: **NVMe SSD**.
- Cheap bulk storage for video archives: **HDD** (7,200 RPM) or a **NAS**.
- Upgrading an old laptop with a 2.5" bay: **2.5" SATA SSD** (huge improvement).
- A small office file server that must survive one disk failure with good capacity: **RAID 5** (3–4 disks) or **RAID 6/10** with large disks.
- Video editing scratch disk where speed matters and data is disposable: **RAID 0** or a single NVMe.

@@quiz
Four 4 TB drives are in RAID 5. What is the usable capacity?
- 16 TB
* 12 TB
- 8 TB
- 4 TB
? (n − 1) × size = 3 × 4 TB.

@@quiz
A new M.2 SSD isn't detected. The board's M.2 slot is "PCIe/NVMe only", the SSD is M.2 SATA (B+M key). Why?
* The slot doesn't support SATA M.2 drives
- The SSD needs a SATA cable
- TRIM is disabled
- The SSD needs defragmenting
? M.2 is a form factor; the interface (SATA vs NVMe) must match.

@@quiz
Which RAID level gives the best performance but no redundancy?
* RAID 0
- RAID 1
- RAID 5
- RAID 10
? Striping only.

@@quiz
A user hears clicking from their laptop's HDD. What's the first thing to do?
- Defragment the drive
- Run Windows Update
* Back up the data immediately, then replace the drive
- Ignore it; HDDs click normally
? Mechanical failure is imminent.

@@quiz
Which is NOT true of SSDs?
- They have no moving parts
- They wear out with writes
* They should be defragmented weekly
- They benefit from TRIM
? Defragmenting adds wear and has no benefit.

@@flashcards
SATA III :: 6 Gbps (~550 MB/s)
NVMe :: SSD over PCIe; M.2 or U.2
M.2 2280 :: 22 mm wide, 80 mm long
M key / B key :: PCIe x4 NVMe / SATA or PCIe x2
SAS :: server drive interface, hot-swap
7,200 RPM :: typical desktop HDD speed
TRIM :: tells the SSD which blocks are free
S.M.A.R.T. :: drive self-monitoring
RAID 0 :: striping, no redundancy
RAID 1 :: mirroring
RAID 5 :: parity, n−1 capacity, 3+ disks
RAID 6 :: double parity, n−2, 4+ disks
RAID 10 :: mirrored stripes, 50%, 4+ disks
hot spare :: idle disk that takes over automatically
NAS vs SAN :: file shares vs block storage
DVD / Blu-ray :: 4.7 GB / 25 GB per layer

@@drill
What is the minimum number of disks for RAID 6?
= 4
? Two disks' worth of parity.

@@drill
What is the capacity of a single-layer Blu-ray disc, in GB?
= 25
? Dual-layer: 50 GB.

@@exercise A RAID calculator
Write `raid(level, disks, size_tb)` returning `(usable_tb, failures_tolerated)` for levels 0, 1, 5, 6 and 10, or raising `ValueError` with a message when there are too few disks (minimums 2, 2, 3, 4, 4; RAID 10 also needs an even number). For RAID 1, treat all disks as mirrors of one another (usable = one disk, tolerates disks − 1). For RAID 10, report the guaranteed tolerance: 1. Print each configuration or its error.
@@lang python
@@starter
configs = [(0, 4, 2), (1, 2, 8), (5, 4, 4), (6, 6, 10), (10, 4, 4), (5, 2, 4), (10, 5, 4)]

def raid(level, disks, size_tb):
    raise NotImplementedError

for level, disks, size in configs:
    try:
        usable, tolerates = raid(level, disks, size)
        print(f"RAID {level}, {disks} x {size} TB: {usable} TB usable, survives {tolerates} failure(s)")
    except ValueError as e:
        print(f"RAID {level}, {disks} x {size} TB: error: {e}")
@@expected
RAID 0, 4 x 2 TB: 8 TB usable, survives 0 failure(s)
RAID 1, 2 x 8 TB: 8 TB usable, survives 1 failure(s)
RAID 5, 4 x 4 TB: 12 TB usable, survives 1 failure(s)
RAID 6, 6 x 10 TB: 40 TB usable, survives 2 failure(s)
RAID 10, 4 x 4 TB: 8 TB usable, survives 1 failure(s)
RAID 5, 2 x 4 TB: error: RAID 5 needs at least 3 disks
RAID 10, 5 x 4 TB: error: RAID 10 needs an even number of disks
@@requires
def raid
raise ValueError
@@hint
Use a dict of minimums. RAID 5: `((disks - 1) * size, 1)`; RAID 6: `((disks - 2) * size, 2)`; RAID 10: `(disks // 2 * size, 1)`.
@@solution
configs = [(0, 4, 2), (1, 2, 8), (5, 4, 4), (6, 6, 10), (10, 4, 4), (5, 2, 4), (10, 5, 4)]
MINIMUM = {0: 2, 1: 2, 5: 3, 6: 4, 10: 4}

def raid(level, disks, size_tb):
    if disks < MINIMUM[level]:
        raise ValueError(f"RAID {level} needs at least {MINIMUM[level]} disks")
    if level == 10 and disks % 2:
        raise ValueError("RAID 10 needs an even number of disks")
    if level == 0:
        return disks * size_tb, 0
    if level == 1:
        return size_tb, disks - 1
    if level == 5:
        return (disks - 1) * size_tb, 1
    if level == 6:
        return (disks - 2) * size_tb, 2
    return disks // 2 * size_tb, 1

for level, disks, size in configs:
    try:
        usable, tolerates = raid(level, disks, size)
        print(f"RAID {level}, {disks} x {size} TB: {usable} TB usable, survives {tolerates} failure(s)")
    except ValueError as e:
        print(f"RAID {level}, {disks} x {size} TB: error: {e}")

@@exercise Read S.M.A.R.T. data
Each drive reports a few S.M.A.R.T. attributes. Rate it `FAILING` if reallocated sectors > 100 or pending sectors > 0 or SSD wear (percentage used) ≥ 90; `WARNING` if reallocated sectors > 0 or the temperature is above 60 °C or wear ≥ 70; otherwise `OK`. Print each drive's rating with the reasons, and the drives to replace first (FAILING), sorted by name.
@@lang python
@@starter
drives = [
    {"name": "PC-014 HDD", "realloc": 0, "pending": 0, "temp": 38, "wear": None},
    {"name": "FS01 disk 3", "realloc": 412, "pending": 8, "temp": 44, "wear": None},
    {"name": "LT-0042 SSD", "realloc": 0, "pending": 0, "temp": 52, "wear": 74},
    {"name": "PC-031 HDD", "realloc": 12, "pending": 0, "temp": 63, "wear": None},
    {"name": "DB01 NVMe", "realloc": 0, "pending": 0, "temp": 49, "wear": 93},
]
@@expected
PC-014 HDD: OK
FS01 disk 3: FAILING (412 reallocated sectors, 8 pending sectors)
LT-0042 SSD: WARNING (74% worn)
PC-031 HDD: WARNING (12 reallocated sectors, 63 °C)
DB01 NVMe: FAILING (93% worn)
Replace first: DB01 NVMe, FS01 disk 3
@@requires
"pending"
"wear"
elif
@@hint
Build two lists, `failing` and `warning`, of reason strings. HDDs have `wear` of `None`, so check `d["wear"] is not None` first.
@@solution
drives = [
    {"name": "PC-014 HDD", "realloc": 0, "pending": 0, "temp": 38, "wear": None},
    {"name": "FS01 disk 3", "realloc": 412, "pending": 8, "temp": 44, "wear": None},
    {"name": "LT-0042 SSD", "realloc": 0, "pending": 0, "temp": 52, "wear": 74},
    {"name": "PC-031 HDD", "realloc": 12, "pending": 0, "temp": 63, "wear": None},
    {"name": "DB01 NVMe", "realloc": 0, "pending": 0, "temp": 49, "wear": 93},
]

replace = []
for d in drives:
    failing, warning = [], []
    wear = d["wear"]
    if d["realloc"] > 100:
        failing.append(f"{d['realloc']} reallocated sectors")
    elif d["realloc"] > 0:
        warning.append(f"{d['realloc']} reallocated sectors")
    if d["pending"] > 0:
        failing.append(f"{d['pending']} pending sectors")
    if wear is not None and wear >= 90:
        failing.append(f"{wear}% worn")
    elif wear is not None and wear >= 70:
        warning.append(f"{wear}% worn")
    if d["temp"] > 60:
        warning.append(f"{d['temp']} °C")
    if failing:
        rating, reasons = "FAILING", failing + warning
        replace.append(d["name"])
    elif warning:
        rating, reasons = "WARNING", warning
    else:
        rating, reasons = "OK", []
    print(f"{d['name']}: {rating}" + (f" ({', '.join(reasons)})" if reasons else ""))

print("Replace first:", ", ".join(sorted(replace)))

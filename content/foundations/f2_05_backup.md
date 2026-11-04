@@lesson f2-backup Backups, Recovery and Business Continuity
@@text
Sooner or later every organisation loses data: a laptop is stolen, a disk dies, someone deletes the wrong folder, ransomware encrypts the file server, a data centre floods. Whether that's a bad afternoon or the end of the company depends on decisions IT made months earlier. Backups are the least glamorous and most important thing in IT.

## What counts as a backup

A backup is a **separate copy** that you can **restore** from **after** the original is lost or damaged. These are not backups on their own:

- **RAID**: protects against a disk failing, not against deletion, corruption or ransomware (they're mirrored instantly).
- **Sync services** (OneDrive, Dropbox, Google Drive): a deleted or encrypted file is synced as deleted or encrypted. Version history and recycle bins help, but have limits.
- **A copy on the same server** or a USB disk left permanently plugged in: ransomware encrypts it too.
- A backup that has **never been restored**: you don't know it works until you've tried.

## The 3-2-1 rule (and 3-2-1-1-0)

- **3** copies of the data (the original plus two backups),
- on **2** different types of media or systems,
- **1** copy **off-site** (another building, another region, the cloud).

The modern extension adds **1** copy that is **offline or immutable** (can't be changed or deleted, even by an admin, until a retention date: tape on a shelf, object-lock cloud storage) and **0** errors when restores are tested. Immutable copies are the answer to ransomware gangs who now hunt for and delete backups first.

## Backup types

| Type | Copies | Backup speed | Restore |
|---|---|---|---|
| **Full** | everything | slowest, most space | simplest: one set |
| **Incremental** | changes since the **last backup of any kind** | fastest, least space | full + **every** incremental since |
| **Differential** | changes since the **last full** | grows each day | full + the **latest** differential |

A common schedule: full on Sunday, incrementals Monday to Saturday. Modern systems often do **forever-incremental** with synthetic fulls built on the backup server, and **snapshots** (instant point-in-time copies at the storage or VM level, which must still be copied elsewhere to count as a backup).

**Retention** sets how long backups are kept: e.g. daily for 30 days, weekly for 3 months, monthly for a year, yearly for 7 years (accounting law in many countries, including Japan's 7-year rule for tax records).

## RPO and RTO

Two numbers the business must decide for each system:

- **RPO** (Recovery Point Objective): how much data, measured in time, you can afford to lose. RPO 24 hours means a nightly backup is enough; RPO 15 minutes needs frequent log or snapshot backups or replication.
- **RTO** (Recovery Time Objective): how long the system may be down while you restore. RTO 4 hours rules out "restore 20 TB from tape".

Tighter RPO and RTO cost more. A small business's file share might accept RPO 24 h / RTO 2 days; an online shop's order database might need RPO near zero / RTO under an hour (replicas and automatic failover).

## What to back up

- File shares and user data (and Microsoft 365/Google Workspace: their built-in retention is **not** a full backup; deletions past the retention window and malicious admins aren't covered).
- Servers and VMs (whole-image backups make restores fast).
- Databases (with tools that understand them, so the copy is consistent).
- Configuration: network devices, firewalls, cloud infrastructure-as-code, GPOs, documentation, and the passwords and encryption keys needed to restore. A backup encrypted with a key that lived on the destroyed server is useless.

## Testing restores

Schedule restore tests: a random file monthly, a full server or database quarterly, a whole-site disaster exercise yearly. Record how long they took and compare with the RTO. Untested backups fail at the worst moment for mundane reasons: an expired credential, a job that silently stopped months ago, a missing driver.

## Business continuity and disaster recovery

- **Disaster recovery (DR)**: how IT restores systems after a major incident: which systems first (based on business priority), from where, by whom, with what credentials.
- **Business continuity (BCP)**: how the business keeps working meanwhile: paper processes, working from home, a secondary site.
- **DR sites**: **hot** (running and synchronised; minutes), **warm** (hardware ready, data some hours old), **cold** (space and power only; days). Cloud DR replaces many physical sites.
- Write the **runbook** before you need it, print a copy, and practise it (**tabletop exercises**). Japan's earthquake and typhoon risk makes geographically separate DR a standard requirement for many companies there.

@@quiz
Why is RAID 1 not a backup?
* A deletion, corruption or ransomware encryption is mirrored to both disks instantly
- RAID 1 is too slow
- RAID 1 uses only one disk
- RAID 1 can't be restored
? RAID protects against hardware failure only.

@@quiz
A full backup runs Sunday and incrementals run Monday–Friday. The server dies Friday afternoon after Friday's backup. What's needed to restore?
- Only Friday's incremental
- Sunday's full and Friday's incremental
* Sunday's full and every incremental Monday–Friday
- Only Sunday's full
? Incrementals chain from the previous backup.

@@quiz
The business says it can lose at most 1 hour of order data. Which objective is that?
* RPO
- RTO
- SLA
- MTTR
? Point = data loss; time = downtime.

@@quiz
What makes a backup copy resistant to ransomware that has stolen admin credentials?
- Storing it on the same server
- Compressing it
* Making it immutable or offline, so it can't be changed or deleted before its retention date
- Using RAID 5
? The extra "1" in 3-2-1-1-0.

@@flashcards
3-2-1 :: 3 copies, 2 media, 1 off-site
3-2-1-1-0 :: plus 1 immutable/offline, 0 restore errors
full backup :: everything
incremental :: changes since the last backup of any kind
differential :: changes since the last full
snapshot :: instant point-in-time copy (copy it elsewhere)
retention :: how long backups are kept
RPO :: maximum acceptable data loss (time)
RTO :: maximum acceptable downtime
immutable storage :: can't be changed until a date
hot / warm / cold site :: minutes / hours / days to recover
DR runbook :: step-by-step recovery plan
tabletop exercise :: practising a disaster on paper

@@drill
In 3-2-1, how many copies should be off-site?
= 1
? At least one, in another location.

@@drill
Which objective measures acceptable downtime?
= RTO
? Recovery Time Objective.

@@exercise Which backups do I need to restore?
Backups are listed in order as `(day, type)` with types `full`, `incr` and `diff`. Write `restore_chain(backups, day)` returning the list of backup days needed to restore to the end of `day`: start at the most recent `full` on or before that day; then if there's a later `diff` up to that day, add only the **latest** diff and any incrementals after it; otherwise add every `incr` after the full. Print the chain for Wednesday, Friday and Saturday.
@@lang python
@@starter
backups = [("Sun", "full"), ("Mon", "incr"), ("Tue", "incr"), ("Wed", "diff"), ("Thu", "incr"), ("Fri", "incr"), ("Sat", "full")]

def restore_chain(backups, day):
    raise NotImplementedError

for day in ["Wed", "Fri", "Sat"]:
    print(day, "->", " + ".join(restore_chain(backups, day)))
@@expected
Wed -> Sun + Wed
Fri -> Sun + Wed + Thu + Fri
Sat -> Sat
@@requires
def restore_chain
"full"
"diff"
@@hint
Cut the list at the target day: `upto = backups[: [d for d, _ in backups].index(day) + 1]`. Find the index of the last full in `upto`, then walk forward from there; when you meet a diff, reset the chain to `[full, diff]`.
@@solution
backups = [("Sun", "full"), ("Mon", "incr"), ("Tue", "incr"), ("Wed", "diff"), ("Thu", "incr"), ("Fri", "incr"), ("Sat", "full")]

def restore_chain(backups, day):
    days = [d for d, _ in backups]
    upto = backups[: days.index(day) + 1]
    last_full = max(i for i, (_, kind) in enumerate(upto) if kind == "full")
    chain = [upto[last_full][0]]
    for d, kind in upto[last_full + 1:]:
        if kind == "diff":
            chain = [upto[last_full][0], d]
        elif kind == "incr":
            chain.append(d)
    return chain

for day in ["Wed", "Fri", "Sat"]:
    print(day, "->", " + ".join(restore_chain(backups, day)))

@@exercise Check backup jobs against RPO and 3-2-1
Each system has an RPO in hours, the hours since its last successful backup, and its copies as a list of `(media, offsite, immutable)`. For each system print `OK` or the problems: `"RPO breached (26h > 24h)"` when the last backup is older than the RPO, `"fewer than 3 copies"`, `"fewer than 2 media types"`, `"no off-site copy"`, `"no immutable copy"`. The original data counts as one of the copies (media `"disk"`, not off-site, not immutable).
@@lang python
@@starter
systems = {
    "File server": {"rpo": 24, "last": 26, "copies": [("nas", False, False), ("cloud", True, True)]},
    "Orders DB": {"rpo": 1, "last": 0.25, "copies": [("disk", False, False), ("cloud", True, False)]},
    "HR share": {"rpo": 24, "last": 8, "copies": [("tape", True, True), ("nas", False, False)]},
}
@@expected
File server: RPO breached (26h > 24h)
Orders DB: no immutable copy
HR share: OK
@@requires
"rpo"
offsite
immutable
@@hint
Build `copies = [("disk", False, False)] + info["copies"]`. Media types: `len({m for m, _, _ in copies})`. Show hours with `:g` to print 26 rather than 26.0.
@@solution
systems = {
    "File server": {"rpo": 24, "last": 26, "copies": [("nas", False, False), ("cloud", True, True)]},
    "Orders DB": {"rpo": 1, "last": 0.25, "copies": [("disk", False, False), ("cloud", True, False)]},
    "HR share": {"rpo": 24, "last": 8, "copies": [("tape", True, True), ("nas", False, False)]},
}

for name, info in systems.items():
    copies = [("disk", False, False)] + info["copies"]
    problems = []
    if info["last"] > info["rpo"]:
        problems.append(f"RPO breached ({info['last']:g}h > {info['rpo']:g}h)")
    if len(copies) < 3:
        problems.append("fewer than 3 copies")
    if len({media for media, _, _ in copies}) < 2:
        problems.append("fewer than 2 media types")
    if not any(offsite for _, offsite, _ in copies):
        problems.append("no off-site copy")
    if not any(immutable for _, _, immutable in copies):
        problems.append("no immutable copy")
    result = "; ".join(problems) or "OK"
    print(f"{name}: {result}")

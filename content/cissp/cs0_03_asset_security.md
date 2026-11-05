@@lesson cs0-asset-security Asset Security in Depth: Classification, Roles, Data States and Sanitisation (Domain 2)
@@text
Domain 2 (Asset Security, about 10% of the exam) is short but full of precise terms the exam likes to test: who owns data versus who looks after it, how classification drives handling, what protects data at rest, in transit and in use, and which sanitisation method fits which media. This lesson goes through them and the exercises apply sanitisation and retention rules.

## Classification

- Classify by **value and sensitivity** (impact if disclosed, altered or lost). Government schemes: Top Secret, Secret, Confidential, (Sensitive but) Unclassified. Commercial: Confidential/Proprietary, Private, Sensitive, Public.
- Classification is set by the **data owner**, drives **labelling** (human-readable) and **marking** (system-readable metadata), and determines handling: storage, transmission, access, retention and destruction.
- **Declassify** or downgrade when value drops (scheduled reviews). Aggregation can raise the classification: many Unclassified items together may reveal something Secret.

## Roles

| Role | Responsibility |
|---|---|
| **Data owner** (senior manager) | accountable; classifies; approves access; sets protection requirements |
| **Data custodian** (often IT) | implements the controls day to day: backups, patching, access configuration |
| **Data steward** | data quality, definitions and metadata (business side) |
| **System owner** | owns the system that processes the data |
| **Data controller** (GDPR) | decides the purposes and means of processing personal data |
| **Data processor** (GDPR) | processes personal data on the controller's behalf |
| **Data subject** | the person the data is about |
| **Users** | follow the policies |
The owner can delegate tasks but **not accountability**.

## Data states and protections

- **At rest**: encryption (full disk, database TDE, file level), access control, backups protected the same way.
- **In transit**: TLS, IPsec, SSH; end-to-end encryption.
- **In use** (in memory, being processed): the hardest; enclaves/confidential computing, homomorphic techniques in research, minimisation.
- **DLP** (data loss prevention) watches endpoints, networks and cloud for sensitive data leaving.
- **Minimisation**, **tokenisation** (replace with a random token, vault keeps the mapping), **pseudonymisation** (still personal data under GDPR), **anonymisation** (no longer personal data if truly irreversible), **masking**.

## Data lifecycle and retention

Create/collect → store → use → share → archive → destroy. Keep data only as long as required by law, contract or business need: a **retention schedule** sets the periods. A **legal hold** suspends destruction for data relevant to litigation, overriding the schedule.

## Sanitisation (NIST SP 800-88)

| Method | What | When |
|---|---|---|
| **Clear** | overwrite with standard read/write commands; protects against simple recovery | reuse inside the organisation, low sensitivity |
| **Purge** | firmware-level secure erase, cryptographic erase, degaussing (magnetic only); resists lab recovery | reuse or leaving control, sensitive data |
| **Destroy** | shred, disintegrate, pulverise, incinerate | highest sensitivity, or media that can't be purged reliably |

- **Degaussing** works on magnetic media (HDDs, tapes) only, and makes HDDs unusable. It does nothing to SSDs or optical media.
- **SSDs**: overwriting isn't reliable (wear levelling, spare blocks); use the drive's secure erase/sanitise command or **crypto erase** (destroy the encryption key), or destroy.
- **Data remanence** is the residual data left after deletion; formatting doesn't remove it.
- Keep a **certificate of sanitisation/destruction**, especially for third-party disposal.

@@quiz
Who is accountable for classifying data?
* The data owner
- The data custodian
- The security administrator
- The user
? Custodians implement the controls.

@@quiz
Which sanitisation method is useless on an SSD?
- Cryptographic erase
- Shredding
* Degaussing
- ATA Secure Erase
? Degaussing works only on magnetic media.

@@quiz
A lawsuit is expected; data covered by it is due for scheduled deletion next week. What happens?
- Delete it per the schedule
* Keep it under a legal hold until released
- Encrypt and delete it
- Move it to a personal drive
? Legal holds override retention schedules.

@@quiz
Under GDPR, who decides why and how personal data is processed?
- The processor
* The controller
- The data subject
- The custodian
? Processors act on the controller's behalf.

@@quiz
Which NIST 800-88 method suits drives leaving the organisation with sensitive data?
- Clear
* Purge (or Destroy)
- Format
- Delete the files
? Purge resists laboratory recovery.

@@flashcards
data owner :: accountable, classifies
data custodian :: implements controls
data steward :: data quality and metadata
controller :: decides purposes and means
processor :: processes for the controller
labelling vs marking :: human-readable vs system-readable
aggregation :: combined data more sensitive
data at rest :: storage encryption
data in transit :: TLS, IPsec
data in use :: memory, enclaves
tokenisation :: token replaces value
pseudonymisation :: still personal data
anonymisation :: irreversibly unlinked
retention schedule :: how long to keep
legal hold :: suspends destruction
clear :: overwrite, internal reuse
purge :: secure/crypto erase, degauss
destroy :: shred, pulverise, incinerate
degaussing :: magnetic media only
data remanence :: residual data

@@drill
Which NIST SP 800-88 category does cryptographic erase belong to?
= purge
= Purge
? It resists laboratory recovery.

@@drill
Which role implements and maintains controls on data day to day?
= custodian
= data custodian
? The owner is accountable.

@@exercise Choose a sanitisation method
For each device, choose Clear, Purge or Destroy following the rules in the lesson (sensitivity, reuse inside or outside the organisation, media type: degauss only magnetic media, no overwriting SSDs, damaged media destroyed), name the technique, and say whether a certificate is needed (always when a third party disposes of it or for high sensitivity).
@@lang python
@@starter
devices = [
    {"id": "LT-104 SSD", "media": "ssd", "sensitivity": "high", "next": "leave", "damaged": False, "third_party": True},
    {"id": "SRV-7 HDD", "media": "hdd", "sensitivity": "moderate", "next": "reuse", "damaged": False, "third_party": False},
    {"id": "Backup tape 33", "media": "tape", "sensitivity": "high", "next": "leave", "damaged": False, "third_party": True},
    {"id": "Kiosk SSD", "media": "ssd", "sensitivity": "low", "next": "reuse", "damaged": False, "third_party": False},
    {"id": "Old DVD archive", "media": "optical", "sensitivity": "moderate", "next": "leave", "damaged": False, "third_party": False},
    {"id": "Failed HDD", "media": "hdd", "sensitivity": "high", "next": "leave", "damaged": True, "third_party": False},
]

def sanitise(device):
    raise NotImplementedError
@@expected
LT-104 SSD: Purge: cryptographic erase or the sanitize command, then verify; keep a certificate of sanitisation
SRV-7 HDD: Clear: overwrite all user-addressable locations
Backup tape 33: Purge: degauss (the media is unusable afterwards); keep a certificate of sanitisation
Kiosk SSD: Purge: the drive's sanitize / secure erase command (overwriting an SSD isn't reliable)
Old DVD archive: Destroy: shred or disintegrate (media can't be reliably erased)
Failed HDD: Destroy: shred or disintegrate (media can't be reliably erased); keep a certificate of sanitisation
@@requires
def sanitise
degauss
"Destroy"
@@hint
Order the checks: damaged or optical media → Destroy; anything high-sensitivity or leaving → Purge (crypto erase / sanitize command for SSDs, degauss for magnetic media); low/moderate reuse inside → Clear (overwrite) for HDDs, but SSDs use their sanitize command (Purge) since overwrite isn't reliable. Leaving the organisation needs at least Purge: degauss for magnetic media.
@@solution
devices = [
    {"id": "LT-104 SSD", "media": "ssd", "sensitivity": "high", "next": "leave", "damaged": False, "third_party": True},
    {"id": "SRV-7 HDD", "media": "hdd", "sensitivity": "moderate", "next": "reuse", "damaged": False, "third_party": False},
    {"id": "Backup tape 33", "media": "tape", "sensitivity": "high", "next": "leave", "damaged": False, "third_party": True},
    {"id": "Kiosk SSD", "media": "ssd", "sensitivity": "low", "next": "reuse", "damaged": False, "third_party": False},
    {"id": "Old DVD archive", "media": "optical", "sensitivity": "moderate", "next": "leave", "damaged": False, "third_party": False},
    {"id": "Failed HDD", "media": "hdd", "sensitivity": "high", "next": "leave", "damaged": True, "third_party": False},
]
MAGNETIC = {"hdd", "tape"}

def sanitise(device):
    media = device["media"]
    if device["damaged"] or media == "optical":
        return "Destroy", "shred or disintegrate (media can't be reliably erased)"
    if device["next"] == "reuse" and device["sensitivity"] != "high":
        if media == "ssd":
            return "Purge", "the drive's sanitize / secure erase command (overwriting an SSD isn't reliable)"
        return "Clear", "overwrite all user-addressable locations"
    if media == "ssd":
        return "Purge", "cryptographic erase or the sanitize command, then verify"
    if media in MAGNETIC:
        return "Purge", "degauss (the media is unusable afterwards)"
    return "Destroy", "physical destruction"

for d in devices:
    method, how = sanitise(d)
    certificate = d["third_party"] or d["sensitivity"] == "high"
    print(f"{d['id']}: {method}: {how}{'; keep a certificate of sanitisation' if certificate else ''}")

@@exercise Apply retention and legal holds
For each record set, decide whether it must be kept, can be destroyed, or is held: compute its age from the creation year, compare with the retention schedule for its category, and override with any legal hold that names its category or system. Also flag sets kept longer than the schedule plus one year without a hold (over-retention is a privacy risk too).
@@lang python
@@starter
YEAR = 2026
SCHEDULE = {"invoices": 7, "hr-files": 5, "web-logs": 1, "cctv": 0.1, "contracts": 10}
holds = [{"case": "Case 2026-14", "categories": {"hr-files"}, "systems": {"mail"}}]
records = [
    {"name": "Invoices 2017", "category": "invoices", "year": 2017, "system": "erp"},
    {"name": "Invoices 2020", "category": "invoices", "year": 2020, "system": "erp"},
    {"name": "HR files 2019", "category": "hr-files", "year": 2019, "system": "hris"},
    {"name": "Web logs 2023", "category": "web-logs", "year": 2023, "system": "siem"},
    {"name": "Mailbox export 2018", "category": "contracts", "year": 2018, "system": "mail"},
    {"name": "Web logs 2025", "category": "web-logs", "year": 2025, "system": "siem"},
]

def decide(record):
    raise NotImplementedError
@@expected
Invoices 2017: destroy now: 9 years old, schedule 7 (over-retained)
Invoices 2020: keep: 6 of 7 years
HR files 2019: HOLD (Case 2026-14): do not destroy
Web logs 2023: destroy now: 3 years old, schedule 1 (over-retained)
Mailbox export 2018: HOLD (Case 2026-14): do not destroy
Web logs 2025: keep: 1 of 1 years
@@requires
def decide
SCHEDULE[
holds
@@hint
Age = `YEAR - year`. Check every hold first: if the category or the system matches, the decision is "HOLD (case)". Otherwise compare the age with `SCHEDULE[category]`: older → "destroy" (and "over-retained" if older than schedule + 1), else "keep".
@@solution
YEAR = 2026
SCHEDULE = {"invoices": 7, "hr-files": 5, "web-logs": 1, "cctv": 0.1, "contracts": 10}
holds = [{"case": "Case 2026-14", "categories": {"hr-files"}, "systems": {"mail"}}]
records = [
    {"name": "Invoices 2017", "category": "invoices", "year": 2017, "system": "erp"},
    {"name": "Invoices 2020", "category": "invoices", "year": 2020, "system": "erp"},
    {"name": "HR files 2019", "category": "hr-files", "year": 2019, "system": "hris"},
    {"name": "Web logs 2023", "category": "web-logs", "year": 2023, "system": "siem"},
    {"name": "Mailbox export 2018", "category": "contracts", "year": 2018, "system": "mail"},
    {"name": "Web logs 2025", "category": "web-logs", "year": 2025, "system": "siem"},
]

def decide(record):
    for hold in holds:
        if record["category"] in hold["categories"] or record["system"] in hold["systems"]:
            return f"HOLD ({hold['case']}): do not destroy"
    age, keep = YEAR - record["year"], SCHEDULE[record["category"]]
    if age > keep + 1:
        return f"destroy now: {age} years old, schedule {keep} (over-retained)"
    if age > keep:
        return f"destroy: {age} years old, schedule {keep}"
    return f"keep: {age} of {keep} years"

for r in records:
    print(f"{r['name']}: {decide(r)}")

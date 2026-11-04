@@lesson az2-storage-deep Azure Storage in Depth: Redundancy, Tiers, Access and Files
@@text
Storage is 15–20% of AZ-104. You need to choose a storage account type and redundancy, pick access tiers and lifecycle rules for blobs, secure access with keys, SAS, Entra ID and network rules, and set up Azure Files. This lesson is the configuration-level detail behind those choices.

## Storage accounts

One account gives a namespace (`https://<account>.blob.core.windows.net`) for **blobs**, **files**, **queues** and **tables**. Names: 3–24 lowercase letters and digits, globally unique.

| Type | Use |
|---|---|
| **Standard general-purpose v2** | most scenarios: blobs, files, queues, tables |
| **Premium block blobs** | low-latency blob workloads |
| **Premium file shares** | high-performance SMB/NFS shares |
| **Premium page blobs** | unmanaged disks (legacy) |

## Redundancy

| Option | Copies | Protects against |
|---|---|---|
| **LRS** | 3 in one datacenter | drive/rack failure |
| **ZRS** | 3 across availability zones in one region | datacenter (zone) failure |
| **GRS** | LRS + LRS in the **paired** region (6) | regional disaster (secondary readable only after failover) |
| **RA-GRS** | GRS with **read access** to the secondary endpoint | regional disaster, read availability |
| **GZRS** | ZRS + LRS in the paired region | zone and regional failure |
| **RA-GZRS** | GZRS + read access to the secondary | highest availability |

Geo-replication is **asynchronous** (some data loss possible on failover). Customer-managed **account failover** promotes the secondary.

## Blob access tiers

| Tier | Minimum retention | Use |
|---|---|---|
| **Hot** | none | frequent access |
| **Cool** | 30 days | infrequent, stored at least a month |
| **Cold** | 90 days | rarely accessed, still online |
| **Archive** | 180 days | offline: must be **rehydrated** (standard priority up to 15 hours, high priority faster) before reading |

Moving out early incurs an **early deletion charge**. **Lifecycle management** rules move or delete blobs by age (last modified or last accessed) and prefix, and clean up old versions/snapshots.

Data protection: **soft delete** (blobs, containers, file shares), **versioning**, **point-in-time restore**, **immutable storage** (time-based retention or legal hold, WORM), **object replication** between accounts.

## Access control

- **Access keys** (two, rotate alternately): full access to everything; avoid sharing. Can be disabled (`AllowSharedKeyAccess = false`).
- **Shared access signatures (SAS)**: signed URLs with **permissions** (r, w, d, l, a, c), **start/expiry**, allowed **IP range**, **protocol** (HTTPS only) and service/resource types. Kinds: **user delegation SAS** (signed with Entra credentials: preferred), **service SAS**, **account SAS**. A **stored access policy** lets you revoke or change service SAS tokens centrally.
- **Entra ID + RBAC** for data: Storage Blob Data Reader/Contributor/Owner, Storage File Data SMB Share Contributor, etc.
- **Network**: storage firewall (selected networks, IP rules), **service endpoints** (traffic from a subnet over the Azure backbone, public endpoint still used) and **private endpoints** (a private IP in your VNet; disable public access).
- Encryption at rest is always on (Microsoft-managed keys or **customer-managed keys** in Key Vault); require TLS 1.2+.

```text
https://stcontoso.blob.core.windows.net/reports/q3.pdf?sv=2023-11-03&sp=r&st=2026-11-04T09:00:00Z&se=2026-11-04T10:00:00Z&spr=https&sip=203.0.113.0-203.0.113.255&sr=b&sig=...
```

## Azure Files and tools

- **SMB** (Windows/Linux/macOS, port **445**, often blocked by ISPs) and **NFS** shares; identity-based auth with AD DS or Entra Kerberos; share snapshots.
- **Azure File Sync**: caches shares on Windows Servers on-premises (cloud tiering), with a Storage Sync Service, sync groups and server endpoints.
- **AzCopy** (`azcopy copy`, `azcopy sync`), **Storage Explorer**, **Data Box** (offline transfer of TBs), **Import/Export**.

@@quiz
Data must survive the loss of an entire region and be readable from the secondary region at all times. Which redundancy?
- ZRS
- GRS
* RA-GRS (or RA-GZRS)
- LRS
? Plain GRS only allows reads after failover.

@@quiz
A blob in the Archive tier must be read. What first?
- Nothing, it's readable with a delay
* Rehydrate it to Hot, Cool or Cold
- Restore it from soft delete
- Enable versioning
? Archive is offline storage.

@@quiz
Which SAS type is signed with Entra ID credentials and is the recommended choice for blobs?
* User delegation SAS
- Account SAS
- Service SAS with an access key
- Stored access policy
? No account key involved.

@@quiz
How can you revoke a set of service SAS tokens without rotating the account keys?
- Delete the container
* Change or delete the stored access policy they reference
- Enable soft delete
- Change the access tier
? Tokens linked to a policy inherit its validity.

@@quiz
A VM must reach a storage account over a private IP and public access must be disabled. What do you configure?
- Service endpoint
* Private endpoint
- Storage firewall with the VM's public IP
- RA-GRS
? Private endpoints place the service in your VNet.

@@quiz
Users can't mount an Azure file share from home. What's the most common cause?
- The share is in the Hot tier
* ISPs block outbound TCP 445
- SAS tokens expired
- The share uses LRS
? Use a VPN or Azure File Sync instead.

@@flashcards
general-purpose v2 :: default storage account
LRS :: 3 copies, one datacenter
ZRS :: 3 copies across zones
GRS :: 6 copies, paired region
RA- prefix :: read access to secondary
GZRS :: ZRS + geo copy
Cool / Cold / Archive minimum :: 30 / 90 / 180 days
Archive :: offline, rehydrate first
lifecycle management :: move or delete by age
soft delete :: recover deleted data
immutable storage :: WORM retention
access keys :: full access, rotate
user delegation SAS :: signed with Entra ID
stored access policy :: revoke service SAS
service endpoint :: backbone route, public endpoint
private endpoint :: private IP in the VNet
Azure Files SMB port :: 445
Azure File Sync :: cache shares on-premises
AzCopy :: command-line transfer
Data Box :: offline bulk transfer

@@drill
How many copies of data does LRS keep?
= 3
= three
? All in one datacenter.

@@drill
What is the minimum storage duration for the Archive tier in days?
= 180
= 180 days
? Cold is 90, Cool is 30.

@@exercise Choose redundancy and estimate copies
For each data set pick the cheapest redundancy that meets the requirements, checked in order of cost: LRS, ZRS, GRS, RA-GRS, GZRS, RA-GZRS. `zone` requires surviving a zone failure, `region` a regional disaster, `read_secondary` reads from the secondary region. Note that GRS/RA-GRS survive a zone failure only by failing over to the other region, so for `zone` require ZRS-based options. Print the choice, the number of copies and the copies' locations.
@@lang python
@@starter
OPTIONS = [
    ("LRS", {"zone": False, "region": False, "read": False}, "3 in one datacenter"),
    ("ZRS", {"zone": True, "region": False, "read": False}, "3 across zones"),
    ("GRS", {"zone": False, "region": True, "read": False}, "3 in one datacenter + 3 in the paired region"),
    ("RA-GRS", {"zone": False, "region": True, "read": True}, "3 in one datacenter + 3 readable in the paired region"),
    ("GZRS", {"zone": True, "region": True, "read": False}, "3 across zones + 3 in the paired region"),
    ("RA-GZRS", {"zone": True, "region": True, "read": True}, "3 across zones + 3 readable in the paired region"),
]
datasets = [
    {"name": "build cache", "zone": False, "region": False, "read_secondary": False},
    {"name": "app uploads", "zone": True, "region": False, "read_secondary": False},
    {"name": "backups", "zone": False, "region": True, "read_secondary": False},
    {"name": "product images", "zone": False, "region": True, "read_secondary": True},
    {"name": "patient records", "zone": True, "region": True, "read_secondary": True},
]
@@expected
build cache      LRS      3 copies: 3 in one datacenter
app uploads      ZRS      3 copies: 3 across zones
backups          GRS      6 copies: 3 in one datacenter + 3 in the paired region
product images   RA-GRS   6 copies: 3 in one datacenter + 3 readable in the paired region
patient records  RA-GZRS  6 copies: 3 across zones + 3 readable in the paired region
@@requires
def choose
OPTIONS
@@hint
An option fits when every required capability is True in its dict: `(not d["zone"] or caps["zone"]) and ...`. Copies are 6 when the option has `region`, else 3.
@@solution
OPTIONS = [
    ("LRS", {"zone": False, "region": False, "read": False}, "3 in one datacenter"),
    ("ZRS", {"zone": True, "region": False, "read": False}, "3 across zones"),
    ("GRS", {"zone": False, "region": True, "read": False}, "3 in one datacenter + 3 in the paired region"),
    ("RA-GRS", {"zone": False, "region": True, "read": True}, "3 in one datacenter + 3 readable in the paired region"),
    ("GZRS", {"zone": True, "region": True, "read": False}, "3 across zones + 3 in the paired region"),
    ("RA-GZRS", {"zone": True, "region": True, "read": True}, "3 across zones + 3 readable in the paired region"),
]
datasets = [
    {"name": "build cache", "zone": False, "region": False, "read_secondary": False},
    {"name": "app uploads", "zone": True, "region": False, "read_secondary": False},
    {"name": "backups", "zone": False, "region": True, "read_secondary": False},
    {"name": "product images", "zone": False, "region": True, "read_secondary": True},
    {"name": "patient records", "zone": True, "region": True, "read_secondary": True},
]

def choose(d):
    for name, caps, where in OPTIONS:
        if (not d["zone"] or caps["zone"]) and (not d["region"] or caps["region"]) and (not d["read_secondary"] or caps["read"]):
            return name, 6 if caps["region"] else 3, where
    return None, 0, ""

for d in datasets:
    name, copies, where = choose(d)
    print(f"{d['name']:<16} {name:<8} {copies} copies: {where}")

@@exercise Validate SAS URLs
Parse each SAS URL's query string and check it against the request: the permission letter needed (`r` read, `w` write, `d` delete, `l` list) must be in `sp`; the request time must be within `st`..`se`; the client IP within the `sip` range (if present); and `spr` must be `https` when the request uses HTTP. Also warn when a token is valid for more than 24 hours. Print `OK` or every reason for rejection.
@@lang python
@@starter
from urllib.parse import urlparse, parse_qs
from datetime import datetime
import ipaddress

urls = {
    "reader": "https://stcontoso.blob.core.windows.net/reports/q3.pdf?sv=2023-11-03&sp=r&st=2026-11-04T09:00:00Z&se=2026-11-04T10:00:00Z&spr=https&sip=203.0.113.0-203.0.113.255&sr=b&sig=abc",
    "uploader": "https://stcontoso.blob.core.windows.net/uploads?sv=2023-11-03&sp=wl&st=2026-11-01T00:00:00Z&se=2026-12-01T00:00:00Z&spr=https,http&sr=c&sig=def",
}
requests = [
    ("reader", "r", "2026-11-04T09:30:00Z", "203.0.113.40", "https"),
    ("reader", "d", "2026-11-04T09:30:00Z", "203.0.113.40", "https"),
    ("reader", "r", "2026-11-04T11:00:00Z", "198.51.100.7", "http"),
    ("uploader", "w", "2026-11-15T12:00:00Z", "192.0.2.10", "http"),
]

def parse_time(text):
    return datetime.strptime(text, "%Y-%m-%dT%H:%M:%SZ")
@@expected
warning: uploader token valid for 720 hours
reader r at 2026-11-04T09:30:00Z from 203.0.113.40 over https: OK
reader d at 2026-11-04T09:30:00Z from 203.0.113.40 over https: permission d not in sp=r
reader r at 2026-11-04T11:00:00Z from 198.51.100.7 over http: outside the validity window; IP 198.51.100.7 not in 203.0.113.0-203.0.113.255; http not allowed (spr=https)
uploader w at 2026-11-15T12:00:00Z from 192.0.2.10 over http: OK
@@requires
parse_qs(
def validate
@@hint
`q = {k: v[0] for k, v in parse_qs(urlparse(url).query).items()}`. For the IP range, split `sip` on `-` and compare `ipaddress.ip_address` values.
@@solution
from urllib.parse import urlparse, parse_qs
from datetime import datetime
import ipaddress

urls = {
    "reader": "https://stcontoso.blob.core.windows.net/reports/q3.pdf?sv=2023-11-03&sp=r&st=2026-11-04T09:00:00Z&se=2026-11-04T10:00:00Z&spr=https&sip=203.0.113.0-203.0.113.255&sr=b&sig=abc",
    "uploader": "https://stcontoso.blob.core.windows.net/uploads?sv=2023-11-03&sp=wl&st=2026-11-01T00:00:00Z&se=2026-12-01T00:00:00Z&spr=https,http&sr=c&sig=def",
}
requests = [
    ("reader", "r", "2026-11-04T09:30:00Z", "203.0.113.40", "https"),
    ("reader", "d", "2026-11-04T09:30:00Z", "203.0.113.40", "https"),
    ("reader", "r", "2026-11-04T11:00:00Z", "198.51.100.7", "http"),
    ("uploader", "w", "2026-11-15T12:00:00Z", "192.0.2.10", "http"),
]

def parse_time(text):
    return datetime.strptime(text, "%Y-%m-%dT%H:%M:%SZ")

def validate(name, perm, when, ip, protocol):
    q = {k: v[0] for k, v in parse_qs(urlparse(urls[name]).query).items()}
    reasons = []
    if perm not in q["sp"]:
        reasons.append(f"permission {perm} not in sp={q['sp']}")
    t = parse_time(when)
    if not parse_time(q["st"]) <= t <= parse_time(q["se"]):
        reasons.append("outside the validity window")
    if "sip" in q:
        low, high = (ipaddress.ip_address(x) for x in q["sip"].split("-"))
        if not low <= ipaddress.ip_address(ip) <= high:
            reasons.append(f"IP {ip} not in {q['sip']}")
    if protocol not in q["spr"].split(","):
        reasons.append(f"{protocol} not allowed (spr={q['spr']})")
    return reasons

for name, url in urls.items():
    q = {k: v[0] for k, v in parse_qs(urlparse(url).query).items()}
    hours = (parse_time(q["se"]) - parse_time(q["st"])).total_seconds() / 3600
    if hours > 24:
        print(f"warning: {name} token valid for {hours:.0f} hours")

for name, perm, when, ip, protocol in requests:
    reasons = validate(name, perm, when, ip, protocol)
    print(f"{name} {perm} at {when} from {ip} over {protocol}: " + ("OK" if not reasons else "; ".join(reasons)))

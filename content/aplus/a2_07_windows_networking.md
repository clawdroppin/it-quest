@@lesson a2-winnet Windows Networking, Sharing and Domain Settings
@@text
Core 2 includes configuring Windows networking for a business: joining domains, sharing folders and printers, mapping drives, firewall rules, proxies and VPNs. These are the settings a help desk changes most often.

## Workgroup vs domain

- **Workgroup**: each PC manages its own accounts. Fine up to about 10 PCs; every user needs an account on every PC they use.
- **Domain** (Active Directory): central accounts, Group Policy, single sign-on to file servers and printers. Requires Pro or above.
- **Entra ID join** (cloud) or **hybrid join** (both AD and Entra ID): modern laptops often join Entra ID and are managed by Intune.

Joining an AD domain: System → About → Domain or workgroup (or `Add-Computer -DomainName example.local` in PowerShell), domain admin credentials, restart. The PC must use the **domain's DNS servers**: the #1 cause of "the domain could not be contacted".

## Network profiles

Windows asks whether a network is **Public** (firewall strict, discovery off: cafés, hotels) or **Private** (discovery and sharing allowed: home, trusted office). Domain networks get the **Domain** profile automatically. Wrong profile = file sharing or discovery "doesn't work".

## Sharing files

1. Right-click the folder → Properties → **Sharing** → Advanced Sharing → share name and **share permissions**.
2. **Security** tab: **NTFS permissions**.
3. Effective access over the network = the **more restrictive** of share and NTFS.

**Administrative shares** exist automatically and are hidden by the `$`: `C$`, `ADMIN$`, `IPC$`. Any share name ending in `$` is hidden from browsing (but not secured by that alone).

Moving vs copying and NTFS permissions:

| Action | Permissions |
|---|---|
| Copy (same or different volume) | **inherits** the destination's permissions |
| Move within the **same** volume | **keeps** its original permissions |
| Move to a **different** volume | inherits the destination's (it's a copy + delete) |

## Mapping drives and printers

- File Explorer → This PC → Map network drive → `\\fs01\finance` → letter `F:`, "reconnect at sign-in".
- `net use F: \\fs01\finance /persistent:yes`
- PowerShell: `New-PSDrive -Name F -PSProvider FileSystem -Root \\fs01\finance -Persist`
- In companies, drives and printers are mapped by **Group Policy Preferences** or Intune, not by hand.
- **UNC path**: `\\server\share\folder`.
- Printers: Settings → Bluetooth & devices → Printers & scanners → Add, or `\\printserver\Floor3-Color`.

## Windows Defender Firewall

- Separate rules per profile (Domain, Private, Public).
- **Inbound rules** block or allow connections to this PC: by program, port (TCP/UDP number) or predefined service (File and Printer Sharing, Remote Desktop).
- **Outbound** is allowed by default.
- `wf.msc` for advanced settings; `netsh advfirewall` or `New-NetFirewallRule` from the command line.
- "Allow an app through the firewall" for simple cases.

## Proxy, VPN, metered connections

- **Proxy**: Settings → Network & internet → Proxy (automatic script `wpad`/PAC URL, or manual server:port, plus exceptions for internal sites). A wrong or stale proxy setting breaks all browsing on one machine while others work.
- **VPN**: Settings → VPN → Add, or the vendor's client (Cisco Secure Client, GlobalProtect, FortiClient, WireGuard). Types: IKEv2, SSTP, L2TP/IPsec, OpenVPN, WireGuard. **Split tunnel** sends only company traffic through the VPN; **full tunnel** sends everything.
- **Metered connection**: limits background downloads (Windows Update) on mobile data.

## IP settings in Windows

Settings → Network → (adapter) → Edit IP assignment: Automatic (DHCP) or Manual (IP, prefix length, gateway, DNS). Or `ncpa.cpl` → adapter properties → IPv4. Also there: **alternate configuration**, adapter **speed/duplex** (Device Manager → NIC → Advanced), **Wake-on-LAN**, **QoS**.

## Remote access to Windows

- **Remote Desktop** (RDP, 3389): needs Pro on the host; enable in Settings → System → Remote Desktop; users must be in the Remote Desktop Users group; **Network Level Authentication (NLA)** required.
- **Remote Assistance / Quick Assist**: the user invites a helper and watches the session.
- Third-party **RMM** tools (remote monitoring and management) and screen-sharing tools (TeamViewer, AnyDesk). Attackers abuse these in tech-support scams: "Microsoft support" never calls users to connect.

@@quiz
A PC can't join the domain: "An Active Directory Domain Controller for the domain could not be contacted". What should you check first?
* That the PC uses the domain's DNS servers
- That the user has admin rights
- That BitLocker is on
- That the firewall is disabled
? AD relies on DNS SRV records.

@@quiz
A file is moved from `D:\Projects` to `D:\Archive` on the same NTFS volume. What happens to its permissions?
* It keeps its original permissions
- It inherits the Archive folder's permissions
- All permissions are removed
- It becomes read-only
? Moves within a volume keep permissions; copies inherit.

@@quiz
File sharing doesn't work on a laptop at a trusted office, and network discovery is off. What's the likely cause?
- The domain is down
* The network is set to the Public profile
- The share name ends with $
- RDP is disabled
? Public profiles block discovery and sharing.

@@quiz
Which command maps F: to a share and keeps it after reboot?
- `ipconfig /map F:`
* `net use F: \\fs01\finance /persistent:yes`
- `mstsc F:`
- `gpresult F:`
? Or Map network drive in File Explorer.

@@flashcards
workgroup :: each PC manages its own accounts
domain :: central accounts and Group Policy (AD)
Entra join :: cloud identity join, managed by Intune
Public / Private / Domain :: Windows network profiles
share vs NTFS :: more restrictive wins over the network
C$ / ADMIN$ :: hidden administrative shares
copy :: inherits destination permissions
move (same volume) :: keeps permissions
UNC path :: \\server\share
net use :: map a drive from the command line
wf.msc :: advanced firewall
split tunnel :: only company traffic through the VPN
NLA :: Network Level Authentication for RDP
Quick Assist :: user-invited remote help
metered connection :: limits background data use

@@drill
What character at the end of a share name hides it from browsing?
= $
? For example C$ and ADMIN$.

@@drill
Which Run command opens Network Connections (adapter settings)?
= ncpa.cpl
? Adapter properties, IPv4, alternate configuration.

@@exercise Effective permissions over the network
Permissions are ranked `none < read < change < full`. Write `effective(share, ntfs)` that returns the more restrictive of the two. For each user, combine permissions from all their groups by taking the **most** permissive within each layer (share and NTFS separately), unless any group has `deny` in a layer, which makes that layer `none`. Then print each user's effective network access.
@@lang python
@@starter
RANK = ["none", "read", "change", "full"]
share = {"Everyone": "change", "Contractors": "read"}
ntfs = {"Finance": "change", "Managers": "full", "Contractors": "deny", "Everyone": "read"}
users = {"ana": ["Everyone", "Finance"], "ken": ["Everyone", "Finance", "Managers"], "omar": ["Everyone", "Contractors"], "mei": ["Everyone"]}

def effective(share_perm, ntfs_perm):
    raise NotImplementedError
@@expected
ana: share=change, ntfs=change -> change
ken: share=change, ntfs=full -> change
omar: share=change, ntfs=none -> none
mei: share=change, ntfs=read -> read
@@requires
def effective
"deny"
key=RANK.index
@@hint
Write `layer(perms, groups)`: if any group's value is `"deny"` return `"none"`; otherwise return the maximum by `RANK.index` of the values present (default `"none"`).
@@solution
RANK = ["none", "read", "change", "full"]
share = {"Everyone": "change", "Contractors": "read"}
ntfs = {"Finance": "change", "Managers": "full", "Contractors": "deny", "Everyone": "read"}
users = {"ana": ["Everyone", "Finance"], "ken": ["Everyone", "Finance", "Managers"], "omar": ["Everyone", "Contractors"], "mei": ["Everyone"]}

def layer(perms, groups):
    values = [perms[g] for g in groups if g in perms]
    if "deny" in values:
        return "none"
    return max(values, key=RANK.index, default="none")

def effective(share_perm, ntfs_perm):
    return min(share_perm, ntfs_perm, key=RANK.index)

for user, groups in users.items():
    s, n = layer(share, groups), layer(ntfs, groups)
    print(f"{user}: share={s}, ntfs={n} -> {effective(s, n)}")

@@exercise Copy or move: what happens to permissions?
For each file operation, decide whether the file ends up with its **original** permissions or **inherits** the destination folder's, following the lesson's table (the volume is the drive letter or the `\\server\share` part). Print the result for each operation.
@@lang python
@@starter
ops = [
    ("move", r"D:\Projects\plan.docx", r"D:\Archive"),
    ("copy", r"D:\Projects\plan.docx", r"D:\Archive"),
    ("move", r"D:\Projects\plan.docx", r"E:\Backup"),
    ("move", r"\\fs01\finance\q1.xlsx", r"\\fs01\finance\old"),
    ("move", r"\\fs01\finance\q1.xlsx", r"\\fs02\archive"),
]

def volume(path):
    raise NotImplementedError
@@expected
move D:\Projects\plan.docx -> D:\Archive: keeps original permissions
copy D:\Projects\plan.docx -> D:\Archive: inherits destination permissions
move D:\Projects\plan.docx -> E:\Backup: inherits destination permissions
move \\fs01\finance\q1.xlsx -> \\fs01\finance\old: keeps original permissions
move \\fs01\finance\q1.xlsx -> \\fs02\archive: inherits destination permissions
@@requires
def volume
startswith("\\\\")
@@hint
For UNC paths, `"\\".join(path.split("\\")[:4])` gives `\\server\share`. For drive paths, `path[:2]`.
@@solution
ops = [
    ("move", r"D:\Projects\plan.docx", r"D:\Archive"),
    ("copy", r"D:\Projects\plan.docx", r"D:\Archive"),
    ("move", r"D:\Projects\plan.docx", r"E:\Backup"),
    ("move", r"\\fs01\finance\q1.xlsx", r"\\fs01\finance\old"),
    ("move", r"\\fs01\finance\q1.xlsx", r"\\fs02\archive"),
]

def volume(path):
    if path.startswith("\\\\"):
        return "\\".join(path.split("\\")[:4]).lower()
    return path[:2].upper()

for action, src, dst in ops:
    same = volume(src) == volume(dst)
    keeps = action == "move" and same
    result = "keeps original permissions" if keeps else "inherits destination permissions"
    print(f"{action} {src} -> {dst}: {result}")

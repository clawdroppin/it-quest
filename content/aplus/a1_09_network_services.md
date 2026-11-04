@@lesson a1-services Network Types, Servers, Appliances and IoT
@@text
Core 1 also expects you to recognise the roles of servers and network appliances on a network diagram, the kinds of networks by size, and the risks of legacy and embedded systems. A typical question: "Which device should be added so that all event logs are collected centrally?" (a syslog server).

## Networks by size

| Type | Scope | Example |
|---|---|---|
| **PAN** | personal, a few metres | Bluetooth headset and phone |
| **LAN** | one building or floor | office Ethernet and Wi-Fi |
| **WLAN** | wireless LAN | the office Wi-Fi |
| **MAN** | a city | a university's campuses linked by fibre |
| **WAN** | countries and continents | company sites linked by MPLS/SD-WAN, the internet |
| **SAN** | storage network for servers | Fibre Channel / iSCSI |

## Server roles

| Server | Does | Protocols/ports |
|---|---|---|
| **DHCP** | hands out IP settings | 67/68 |
| **DNS** | resolves names | 53 |
| **File** | shared folders | SMB 445, NFS |
| **Print** | queues and drivers for shared printers | SMB, IPP 631 |
| **Mail** | sends/receives and stores mail | SMTP, IMAP, POP3 |
| **Web** | websites and web apps | HTTP/HTTPS |
| **Syslog** | collects logs from devices and servers | UDP/TCP 514 |
| **AAA** (RADIUS, TACACS+) | authenticates network logins, VPN and Wi-Fi Enterprise users | RADIUS 1812/1813 |
| **Database** | stores app data | SQL Server 1433, MySQL 3306, PostgreSQL 5432 |
| **Directory** | user and computer accounts | LDAP 389, Kerberos 88 |
| **NTP** | time synchronisation (Kerberos breaks if clocks differ by more than 5 minutes) | UDP 123 |

**Spam gateways** filter email before it reaches the mail server. **Unified threat management (UTM)** combines firewall, IPS, antivirus, content filtering and VPN in one appliance.

## Network appliances

- **Load balancer**: spreads traffic over several servers; removes a failed server from the pool.
- **Proxy server**: makes requests on behalf of clients; caching, filtering, logging.
- **Firewall**: filters by rules; **next-generation (NGFW)** firewalls understand applications.
- **IDS / IPS**: intrusion **detection** (alerts) / **prevention** (blocks) systems.
- **SCADA** / **ICS**: control systems for factories and utilities.

## Legacy and embedded systems

**Embedded** systems are computers built into devices: printers, door controllers, medical equipment, industrial controllers, building management. **Legacy** systems are old systems still in use because replacing them is costly or risky (a Windows XP PC running a factory machine).

Risks: no security updates, default passwords, unencrypted protocols, unknown vendors. Mitigations: **isolate** them on their own VLAN/network with strict firewall rules, no internet access, monitor them, and plan replacement.

## IoT

Smart devices: thermostats, cameras, TVs, voice assistants, smart plugs, badge readers. Same rules: change default credentials, update firmware, separate network (an IoT VLAN or guest network), disable features not needed.

## Hands-on: diagrams and documentation

Physical and logical network diagrams, IP address management (IPAM) spreadsheets or tools, rack diagrams and labelled cables are what make the server roles above findable when something breaks at 2 a.m.

## Remote access and management concepts for Core 1

- **VPN**: an encrypted tunnel over the internet to the company network (site-to-site between offices, or client-to-site for remote workers).
- **SD-WAN**: software-managed links between sites over several internet connections.
- **MPLS**: a carrier's private WAN service.

@@quiz
Which server should be added so that logs from all switches and firewalls are collected in one place?
* A syslog server
- A DHCP server
- A proxy server
- A file server
? UDP/TCP 514.

@@quiz
Users can't log in with Kerberos and the domain controller logs time errors. Which service is likely misconfigured?
- DHCP
* NTP
- SNMP
- SMTP
? Kerberos tolerates about 5 minutes of clock skew.

@@quiz
An old Windows XP PC must keep running a lab instrument. What's the best mitigation?
- Connect it directly to the internet for updates
* Isolate it on its own network segment with strict firewall rules and no internet access
- Give the user admin rights
- Install a newer browser
? Segmentation limits the damage of unpatched systems.

@@quiz
Which device detects suspicious traffic and actively blocks it?
- IDS
* IPS
- Proxy
- Syslog
? Prevention vs detection.

@@flashcards
PAN / LAN / MAN / WAN :: personal / building / city / wide area
SAN :: storage network (Fibre Channel, iSCSI)
syslog :: central log collection, 514
RADIUS :: AAA for Wi-Fi Enterprise and VPN, 1812/1813
NTP :: time sync, UDP 123
Kerberos skew :: about 5 minutes
load balancer :: spreads traffic across servers
proxy :: makes requests for clients; filters, caches
IDS / IPS :: detects / blocks intrusions
UTM :: all-in-one security appliance
SCADA / ICS :: industrial control systems
embedded system :: computer inside a device
IoT :: networked smart devices; isolate them
MySQL / SQL Server / PostgreSQL ports :: 3306 / 1433 / 5432

@@drill
What port does syslog use?
= 514
? UDP traditionally, TCP/TLS for reliability.

@@drill
What port does NTP use?
= 123
? UDP.

@@exercise Match requirements to server roles
For each requirement, find the server role whose keywords appear in the requirement text (case-insensitive) and print the role and its port. If several roles match, print them all, sorted. If none match, print `"no match: ask for more detail"`.
@@lang python
@@starter
ROLES = {
    "DHCP": (["ip address", "lease", "automatic address"], "67/68"),
    "DNS": (["name resolution", "hostname", "resolve"], "53"),
    "Syslog": (["logs", "log collection"], "514"),
    "RADIUS": (["wi-fi enterprise", "802.1x", "vpn login"], "1812"),
    "NTP": (["time", "clock"], "123"),
    "Print": (["printer", "print queue"], "631"),
    "File": (["shared folder", "file share"], "445"),
}
requirements = [
    "Staff laptops need an automatic address when they plug in",
    "All firewall and switch logs must be kept for 90 days",
    "The domain controller clock drifts and Kerberos fails",
    "Wi-Fi Enterprise logins with 802.1X",
    "Department printer with a shared print queue and a shared folder for scans",
    "Make the coffee machine smarter",
]
@@expected
Staff laptops need an automatic address when they plug in: DHCP (67/68)
All firewall and switch logs must be kept for 90 days: Syslog (514)
The domain controller clock drifts and Kerberos fails: NTP (123)
Wi-Fi Enterprise logins with 802.1X: RADIUS (1812)
Department printer with a shared print queue and a shared folder for scans: File (445), Print (631)
Make the coffee machine smarter: no match: ask for more detail
@@requires
.lower()
sorted(
@@hint
`text = req.lower()`, then `matches = sorted(role for role, (words, _) in ROLES.items() if any(w in text for w in words))`.
@@solution
ROLES = {
    "DHCP": (["ip address", "lease", "automatic address"], "67/68"),
    "DNS": (["name resolution", "hostname", "resolve"], "53"),
    "Syslog": (["logs", "log collection"], "514"),
    "RADIUS": (["wi-fi enterprise", "802.1x", "vpn login"], "1812"),
    "NTP": (["time", "clock"], "123"),
    "Print": (["printer", "print queue"], "631"),
    "File": (["shared folder", "file share"], "445"),
}
requirements = [
    "Staff laptops need an automatic address when they plug in",
    "All firewall and switch logs must be kept for 90 days",
    "The domain controller clock drifts and Kerberos fails",
    "Wi-Fi Enterprise logins with 802.1X",
    "Department printer with a shared print queue and a shared folder for scans",
    "Make the coffee machine smarter",
]

for req in requirements:
    text = req.lower()
    matches = sorted(role for role, (words, _) in ROLES.items() if any(w in text for w in words))
    if matches:
        print(f"{req}: " + ", ".join(f"{m} ({ROLES[m][1]})" for m in matches))
    else:
        print(f"{req}: no match: ask for more detail")

@@exercise Find risky legacy and IoT devices
From the asset inventory, flag devices that are risky: end-of-life OS or firmware (`eol` true), still using default credentials, or using insecure protocols (any of Telnet, FTP, HTTP, SNMPv1). For every flagged device that is **not** already on the `isolated` VLAN, print `MOVE TO ISOLATED VLAN`. Print one line per device, then the count of devices that must be moved.
@@lang python
@@starter
INSECURE = {"Telnet", "FTP", "HTTP", "SNMPv1"}
assets = [
    {"name": "lab-xp-01", "eol": True, "default_creds": False, "protocols": {"SMB"}, "vlan": "lab"},
    {"name": "cam-lobby", "eol": False, "default_creds": True, "protocols": {"HTTP", "RTSP"}, "vlan": "iot"},
    {"name": "printer-3f", "eol": False, "default_creds": False, "protocols": {"IPP", "HTTPS"}, "vlan": "printers"},
    {"name": "hvac-ctrl", "eol": True, "default_creds": True, "protocols": {"Telnet"}, "vlan": "isolated"},
    {"name": "door-ctrl", "eol": False, "default_creds": False, "protocols": {"SNMPv1", "HTTPS"}, "vlan": "servers"},
]
@@expected
lab-xp-01: end of life  MOVE TO ISOLATED VLAN
cam-lobby: default credentials, insecure HTTP  MOVE TO ISOLATED VLAN
printer-3f: OK
hvac-ctrl: end of life, default credentials, insecure Telnet
door-ctrl: insecure SNMPv1  MOVE TO ISOLATED VLAN
Devices to move: 3
@@requires
INSECURE
& INSECURE
@@hint
Insecure protocols in use: `a["protocols"] & INSECURE` (set intersection). Build a list of reasons per device.
@@solution
INSECURE = {"Telnet", "FTP", "HTTP", "SNMPv1"}
assets = [
    {"name": "lab-xp-01", "eol": True, "default_creds": False, "protocols": {"SMB"}, "vlan": "lab"},
    {"name": "cam-lobby", "eol": False, "default_creds": True, "protocols": {"HTTP", "RTSP"}, "vlan": "iot"},
    {"name": "printer-3f", "eol": False, "default_creds": False, "protocols": {"IPP", "HTTPS"}, "vlan": "printers"},
    {"name": "hvac-ctrl", "eol": True, "default_creds": True, "protocols": {"Telnet"}, "vlan": "isolated"},
    {"name": "door-ctrl", "eol": False, "default_creds": False, "protocols": {"SNMPv1", "HTTPS"}, "vlan": "servers"},
]

to_move = 0
for a in assets:
    reasons = []
    if a["eol"]:
        reasons.append("end of life")
    if a["default_creds"]:
        reasons.append("default credentials")
    bad = a["protocols"] & INSECURE
    if bad:
        reasons.append("insecure " + "/".join(sorted(bad)))
    if not reasons:
        print(f"{a['name']}: OK")
        continue
    action = ""
    if a["vlan"] != "isolated":
        action = "  MOVE TO ISOLATED VLAN"
        to_move += 1
    print(f"{a['name']}: {', '.join(reasons)}{action}")
print("Devices to move:", to_move)

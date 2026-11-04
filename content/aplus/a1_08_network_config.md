@@lesson a1-netconfig TCP/IP Configuration, Ports, DNS Records and SOHO Routers
@@text
Core 1 domain 2 (networking, about 23%) asks you to configure a client's network settings, know dozens of port numbers, recognise DNS record types, and set up a small office/home office (SOHO) router securely. Performance-based questions often show a router's settings page and ask you to fix it.

## Client TCP/IP settings

Each device needs four things:

1. **IP address**
2. **Subnet mask**
3. **Default gateway** (the router)
4. **DNS server(s)**

They come from **DHCP** automatically, or are set **statically** (servers, printers, network devices). Alternatives:

- **DHCP reservation**: the DHCP server always gives a specific device (by MAC address) the same IP. Easier to manage than static addresses scattered across devices.
- **APIPA** (169.254.x.x): self-assigned when DHCP fails; only talks to other APIPA devices.
- **Alternate configuration** in Windows: a static fallback if DHCP doesn't answer.

### DHCP details

- The **DORA** process: **D**iscover (client broadcast), **O**ffer (server), **R**equest (client), **A**cknowledge (server).
- A **scope** is the address range a DHCP server hands out, with **exclusions** (addresses not to hand out, e.g. for printers), the **lease** time, and **options** (gateway, DNS servers, domain name).
- DHCP is a broadcast, so it doesn't cross routers by itself; a **DHCP relay / IP helper** forwards requests to a central server.

## Ports you must know

| Port | Protocol | Use |
|---|---|---|
| 20/21 | FTP | file transfer (data/control), unencrypted |
| 22 | SSH / SFTP | secure remote shell and file transfer |
| 23 | Telnet | unencrypted remote shell: avoid |
| 25 | SMTP | mail between servers |
| 53 | DNS | name resolution (UDP, and TCP for zone transfers/large replies) |
| 67/68 | DHCP | server/client |
| 80 | HTTP | web |
| 110 | POP3 | mail retrieval |
| 137–139 | NetBIOS | legacy Windows naming/sessions |
| 143 | IMAP | mail retrieval |
| 161/162 | SNMP | monitoring / traps |
| 389 | LDAP | directory queries |
| 443 | HTTPS | secure web |
| 445 | SMB/CIFS | Windows file sharing |
| 3389 | RDP | Remote Desktop |

Plus the secure mail variants: SMTPS/submission 587 (465), IMAPS 993, POP3S 995; LDAPS 636.

**TCP** (connection-oriented, reliable, ordered: web, email, file transfers) vs **UDP** (connectionless, no delivery guarantee, low overhead: DNS queries, DHCP, streaming, VoIP, online games).

## DNS record types

| Record | Maps | Example |
|---|---|---|
| **A** | name → IPv4 | `portal.example.com → 203.0.113.10` |
| **AAAA** | name → IPv6 | `portal.example.com → 2001:db8::10` |
| **CNAME** | alias → another name | `www → portal.example.com` |
| **MX** | domain → mail server (with priority) | `example.com MX 10 mail.example.com` |
| **TXT** | text: SPF, DKIM, DMARC, domain verification | `v=spf1 include:... -all` |
| **PTR** | IP → name (reverse lookup) | `10.113.0.203.in-addr.arpa → portal...` |
| **NS** | which servers are authoritative | |
| **SRV** | service location (AD, SIP) | `_ldap._tcp.example.com` |

**TTL** (time to live) is how long resolvers may cache a record. Lower it before a migration so changes spread quickly.

## SOHO router configuration

A typical small-office router setup:

1. **Change the default admin password** (and username if possible), and update the **firmware**.
2. **WAN** settings from the ISP (DHCP, PPPoE with username/password, or static).
3. **LAN**: router IP (e.g. 192.168.1.1/24), DHCP scope (e.g. .100–.199), reservations for printers.
4. **Wireless**: an SSID, **WPA3** (or WPA2-AES), a strong passphrase, guest network isolated from the LAN; disable **WPS** (its PIN is easily brute-forced).
5. **Firewall/NAT**: default deny inbound. To make an internal server reachable, use **port forwarding** (e.g. WAN TCP 443 → 192.168.1.20:443) rather than a **DMZ** host (which exposes all ports of one device).
6. **Disable UPnP** if not needed (it lets devices open ports themselves).
7. **Content filtering**, MAC filtering (weak: MACs are easy to spoof), scheduled access.
8. **Static IP** for the router's management, remote management **off** (or restricted).

## IPv4 and IPv6 for the exam

- Private ranges: 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16.
- Loopback 127.0.0.1 / ::1; APIPA 169.254/16; IPv6 link-local fe80::/10.
- IPv6 has no broadcast (uses multicast), no NAT required, and devices can self-configure with **SLAAC**.

## Internet connection types

- **Fibre** (FTTH: fastest, symmetric options), **cable** (DOCSIS over coax), **DSL** (over phone lines, speed falls with distance), **cellular** (4G/5G, fixed wireless), **satellite** (geostationary: high latency ~600 ms; low-earth-orbit like Starlink: ~30–50 ms), **WISP** (wireless ISP, line-of-sight radio).

@@quiz
A printer must always get 192.168.1.50 but the company wants all addresses managed centrally. What should you configure?
* A DHCP reservation for the printer's MAC address
- A static IP typed into the printer
- APIPA
- A larger DHCP scope
? Central management with a fixed address.

@@quiz
Which record lets other servers verify mail for example.com comes from an authorised server?
- MX
- CNAME
* TXT (SPF)
- PTR
? SPF, DKIM and DMARC are TXT records.

@@quiz
A small office wants its internal web server reachable from the internet, exposing as little as possible. What should you configure?
- Put the server in the DMZ
* Port forwarding of TCP 443 to the server's internal IP
- Enable UPnP
- Turn off the firewall
? A DMZ host exposes every port.

@@quiz
Which DHCP step does the client send first?
* Discover
- Offer
- Request
- Acknowledge
? DORA.

@@quiz
Which protocol and port does Remote Desktop use?
- SSH 22
- SMB 445
* RDP 3389
- Telnet 23
? Never expose 3389 directly to the internet.

@@flashcards
DORA :: Discover, Offer, Request, Acknowledge
DHCP scope :: range of addresses to lease
DHCP reservation :: fixed IP for a MAC address
DHCP relay / IP helper :: forwards DHCP across routers
A / AAAA :: name to IPv4 / IPv6
CNAME :: alias
MX :: mail server for a domain
TXT :: SPF, DKIM, DMARC, verification
PTR :: reverse lookup
TTL :: how long a record may be cached
TCP vs UDP :: reliable connection vs connectionless
port forwarding :: map a WAN port to an internal host
DMZ host :: exposes all ports of one device
WPS :: insecure PIN pairing: disable
UPnP :: lets devices open ports: disable if unused
SMB :: 445
LDAP / LDAPS :: 389 / 636

@@drill
What port does SMB use?
= 445
? Windows file sharing.

@@drill
What port does LDAP use?
= 389
? LDAPS is 636.

@@drill
What port does Telnet use?
= 23
? Unencrypted: replace with SSH.

@@exercise A port quiz engine
Write `lookup(query)` that answers in both directions from the `PORTS` table: given a number it returns `"<port>: <protocol>"`, given a protocol name (case-insensitive) it returns all its ports joined by `", "`, and for anything unknown it returns `"unknown"`. Also write `secure_alternative(protocol)` using `SECURE` and print, for each insecure protocol, its secure replacement and port.
@@lang python
@@starter
PORTS = {20: "FTP", 21: "FTP", 22: "SSH", 23: "Telnet", 25: "SMTP", 53: "DNS", 67: "DHCP", 68: "DHCP",
         80: "HTTP", 110: "POP3", 143: "IMAP", 161: "SNMP", 389: "LDAP", 443: "HTTPS", 445: "SMB",
         636: "LDAPS", 993: "IMAPS", 995: "POP3S", 3389: "RDP"}
SECURE = {"Telnet": "SSH", "HTTP": "HTTPS", "FTP": "SSH", "POP3": "POP3S", "IMAP": "IMAPS", "LDAP": "LDAPS"}

def lookup(query):
    raise NotImplementedError

for q in [443, "dhcp", "FTP", 8080, "Gopher"]:
    print(q, "->", lookup(q))
@@expected
443 -> 443: HTTPS
dhcp -> 67, 68
FTP -> 20, 21
8080 -> unknown
Gopher -> unknown
Secure alternatives:
  Telnet (23) -> SSH (22)
  HTTP (80) -> HTTPS (443)
  FTP (20, 21) -> SSH (22)
  POP3 (110) -> POP3S (995)
  IMAP (143) -> IMAPS (993)
  LDAP (389) -> LDAPS (636)
@@requires
def lookup
isinstance(
.lower()
@@hint
`if isinstance(query, int): ...`. For names: `ports = [str(p) for p, name in PORTS.items() if name.lower() == query.lower()]`. For the secure alternatives, use `lookup(SECURE[proto])` to get the port.
@@solution
PORTS = {20: "FTP", 21: "FTP", 22: "SSH", 23: "Telnet", 25: "SMTP", 53: "DNS", 67: "DHCP", 68: "DHCP",
         80: "HTTP", 110: "POP3", 143: "IMAP", 161: "SNMP", 389: "LDAP", 443: "HTTPS", 445: "SMB",
         636: "LDAPS", 993: "IMAPS", 995: "POP3S", 3389: "RDP"}
SECURE = {"Telnet": "SSH", "HTTP": "HTTPS", "FTP": "SSH", "POP3": "POP3S", "IMAP": "IMAPS", "LDAP": "LDAPS"}

def lookup(query):
    if isinstance(query, int):
        return f"{query}: {PORTS[query]}" if query in PORTS else "unknown"
    ports = [str(p) for p, name in PORTS.items() if name.lower() == query.lower()]
    return ", ".join(ports) or "unknown"

for q in [443, "dhcp", "FTP", 8080, "Gopher"]:
    print(q, "->", lookup(q))

print("Secure alternatives:")
for proto, secure in SECURE.items():
    print(f"  {proto} ({lookup(proto)}) -> {secure} ({lookup(secure)})")

@@exercise Audit a SOHO router configuration
Check the router's settings against the hardening checklist and print a numbered list of findings: default admin password still set, firmware older than 365 days, wireless security not WPA3 or WPA2 (AES), WPS enabled, UPnP enabled, remote management enabled, a DMZ host set, and any port forward to RDP (3389). End with `"N findings"`, or `"Router passes the checklist"` if there are none.
@@lang python
@@starter
router = {
    "admin_password": "admin",
    "firmware_age_days": 610,
    "wifi_security": "WPA2-TKIP",
    "wps": True,
    "upnp": True,
    "remote_mgmt": False,
    "dmz_host": None,
    "port_forwards": [(443, "192.168.1.20", 443), (3389, "192.168.1.35", 3389)],
}
DEFAULT_PASSWORDS = {"admin", "password", "1234", ""}
@@expected
1. Change the default admin password
2. Update the firmware (610 days old)
3. Use WPA3 or WPA2-AES instead of WPA2-TKIP
4. Disable WPS
5. Disable UPnP
6. Remove the RDP forward to 192.168.1.35; use a VPN instead
6 findings
@@requires
DEFAULT_PASSWORDS
port_forwards
@@hint
Append a message to `findings` for each failed check; then `for i, f in enumerate(findings, 1): print(f"{i}. {f}")`.
@@solution
router = {
    "admin_password": "admin",
    "firmware_age_days": 610,
    "wifi_security": "WPA2-TKIP",
    "wps": True,
    "upnp": True,
    "remote_mgmt": False,
    "dmz_host": None,
    "port_forwards": [(443, "192.168.1.20", 443), (3389, "192.168.1.35", 3389)],
}
DEFAULT_PASSWORDS = {"admin", "password", "1234", ""}

findings = []
if router["admin_password"] in DEFAULT_PASSWORDS:
    findings.append("Change the default admin password")
if router["firmware_age_days"] > 365:
    findings.append(f"Update the firmware ({router['firmware_age_days']} days old)")
if router["wifi_security"] not in ("WPA3", "WPA2-AES"):
    findings.append(f"Use WPA3 or WPA2-AES instead of {router['wifi_security']}")
if router["wps"]:
    findings.append("Disable WPS")
if router["upnp"]:
    findings.append("Disable UPnP")
if router["remote_mgmt"]:
    findings.append("Disable remote management")
if router["dmz_host"]:
    findings.append(f"Remove the DMZ host {router['dmz_host']}")
for wan, host, lan in router["port_forwards"]:
    if lan == 3389:
        findings.append(f"Remove the RDP forward to {host}; use a VPN instead")

for i, finding in enumerate(findings, 1):
    print(f"{i}. {finding}")
print(f"{len(findings)} findings" if findings else "Router passes the checklist")

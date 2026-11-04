@@lesson sp3-network Secure Network Architecture and Enterprise Controls
@@text
Domain 3 (security architecture, 18%) asks you to place the right control in the right spot: which firewall type, where the IPS goes, how to segment, how remote users connect, and what happens when a device fails. Domain 4 then asks how you'd modify those controls (a new firewall rule, a web filter category, a DLP policy) to respond to a situation.

## Infrastructure considerations

- **Device placement**: perimeter firewall at the internet edge; internal firewalls between zones; IPS inline after the firewall; sensors/taps where you need visibility.
- **Security zones**: internet, **screened subnet (DMZ)**, internal, management, guest, OT. Traffic between zones passes a policy enforcement point.
- **Attack surface**: every open port, service and exposed interface; reduce it.
- **Connectivity**: how sites and users connect.
- **Failure modes**: **fail-open** (traffic passes if the device fails: availability first) vs **fail-closed** (traffic stops: security first). An inline IPS for a hospital network might fail open; a firewall for a payment zone fails closed.
- **Active vs passive** devices: active (inline) can block; passive (tap/SPAN) only observe.
- **Inline vs tap/monitor**.

## Firewall types

| Type | Works at | Notes |
|---|---|---|
| **Packet filter** (stateless) | L3/L4 | each packet alone; simple ACLs |
| **Stateful** | L3/L4 + state table | allows return traffic for established sessions |
| **NGFW** | up to L7 | application awareness, user identity, IPS, TLS inspection, URL filtering |
| **WAF** | L7 for HTTP | protects web apps from SQLi, XSS etc. |
| **UTM** | all-in-one | firewall, IPS, AV, filtering for small sites |
| **Host-based firewall** | the endpoint | Windows Defender Firewall, iptables/nftables |

## Detection and prevention

- **IDS** (alerts) vs **IPS** (blocks inline).
- **Signature-based** (known patterns, low false positives, misses new attacks) vs **anomaly/behaviour-based** (finds new attacks, more false positives).
- **NIDS/NIPS** on the network vs **HIDS/HIPS** on hosts.

## Secure communication and access

- **VPN**: IPsec (site-to-site), TLS (remote users); **tunnelling**.
- **SD-WAN** and **SASE** for distributed organisations.
- **Jump server / bastion host**: admins connect here first, then to servers; heavily monitored.
- **Proxy server**: forward proxy for users (filtering, caching, logging); reverse proxy in front of servers.
- **Load balancer**: availability and scaling; can terminate TLS.
- **802.1X / NAC**: port-based authentication and posture checks; unknown devices go to a quarantine VLAN.
- **Sensors** and **collectors** feed the SIEM.

## Secure protocols (replace the insecure ones)

| Insecure | Secure | Port |
|---|---|---|
| Telnet | SSH | 22 |
| FTP | SFTP / FTPS | 22 / 990 |
| HTTP | HTTPS | 443 |
| SNMPv1/v2c | SNMPv3 | 161/162 |
| LDAP | LDAPS | 636 |
| POP3 / IMAP | POP3S / IMAPS | 995 / 993 |
| SMTP | SMTPS / STARTTLS | 465 / 587 |
| DNS | DNSSEC, DoH/DoT | 853 |
| RDP without NLA | RDP with NLA via gateway | 3389 |

## Enterprise capabilities to modify (domain 4)

- **Firewall rules** and **access lists**: add, tighten, remove; ports/protocols; **screened subnets**.
- **IDS/IPS rules and signatures**; tune false positives.
- **Web filtering**: agent-based or centralised proxy; **URL scanning**, **content categorisation**, **block rules**, **reputation**.
- **OS security**: Group Policy, SELinux.
- **DNS filtering**.
- **Email security** (SPF/DKIM/DMARC, gateways).
- **File integrity monitoring (FIM)**.
- **DLP**: detect and block sensitive data leaving (card numbers, "confidential" labels) by email, web, USB.
- **NAC**, **EDR/XDR**, **user behaviour analytics (UBA/UEBA)**.

## Mitigation techniques (domain 2)

**Segmentation**, **access control** (ACLs, permissions), **application allow listing** (only approved programs run), **isolation** (quarantine a compromised host), **patching**, **encryption**, **monitoring**, **least privilege**, **configuration enforcement**, **decommissioning**, **hardening** (disable unused ports/protocols/services, change defaults, remove unnecessary software, host-based firewall, HIPS, EDR).

@@quiz
An inline IPS protecting a hospital's patient monitoring network fails. The hospital wants traffic to keep flowing. What failure mode is this?
* Fail-open
- Fail-closed
- Fail-secure
- Passive mode
? Availability is prioritised over security here.

@@quiz
Which device protects a web application from SQL injection and XSS at layer 7?
- Packet filter
- Stateful firewall
* Web application firewall
- Load balancer
? A WAF inspects HTTP requests.

@@quiz
Which detection method is better at spotting brand-new attacks but produces more false positives?
- Signature-based
* Anomaly/behaviour-based
- Hash-based
- Allow listing
? It compares behaviour with a baseline.

@@quiz
Admins must reach production servers only through one hardened, heavily logged host. What is it?
- Reverse proxy
* Jump server (bastion host)
- Honeypot
- Load balancer
? A single controlled entry point.

@@quiz
Which control stops employees from emailing spreadsheets that contain card numbers?
- WAF
- NAC
* DLP
- FIM
? Data loss prevention inspects content leaving the organisation.

@@flashcards
fail-open :: traffic passes when the device fails
fail-closed :: traffic stops when the device fails
screened subnet :: DMZ for public servers
stateful firewall :: tracks sessions
NGFW :: app-aware, user-aware, IPS
WAF :: layer 7 web protection
UTM :: all-in-one appliance
IDS vs IPS :: alert vs block
signature vs anomaly :: known patterns vs deviations
jump server :: controlled admin entry point
reverse proxy :: sits in front of servers
802.1X / NAC :: authenticate devices at the port
LDAPS port :: 636
IMAPS / POP3S :: 993 / 995
DLP :: stops sensitive data leaving
FIM :: detects changed files
UEBA :: user behaviour analytics
application allow listing :: only approved programs run

@@drill
What port does LDAPS use?
= 636
? Plain LDAP is 389.

@@drill
What secure protocol replaces Telnet?
= SSH
= ssh
? Port 22.

@@exercise Recommend secure protocol replacements
A port scan lists services found on servers. For every insecure protocol in `UPGRADES`, print the server, the finding and the replacement with its port. Then print the servers sorted by number of findings (most first, ties by name) as a remediation order.
@@lang python
@@starter
UPGRADES = {
    23: ("Telnet", "SSH", 22),
    21: ("FTP", "SFTP", 22),
    80: ("HTTP", "HTTPS", 443),
    389: ("LDAP", "LDAPS", 636),
    110: ("POP3", "POP3S", 995),
    143: ("IMAP", "IMAPS", 993),
    161: ("SNMPv1/v2c", "SNMPv3", 161),
}
scan = {
    "fs01": [21, 445, 3389],
    "dc01": [53, 88, 389, 636],
    "mail01": [25, 110, 143, 443, 587],
    "sw-core": [22, 23, 161],
    "web01": [80, 443],
}
@@expected
fs01: FTP on 21 -> use SFTP (22)
dc01: LDAP on 389 -> use LDAPS (636)
mail01: POP3 on 110 -> use POP3S (995)
mail01: IMAP on 143 -> use IMAPS (993)
sw-core: Telnet on 23 -> use SSH (22)
sw-core: SNMPv1/v2c on 161 -> use SNMPv3 (161)
web01: HTTP on 80 -> use HTTPS (443)
Remediation order: mail01 (2), sw-core (2), dc01 (1), fs01 (1), web01 (1)
@@requires
UPGRADES
sorted(
@@hint
Collect `findings[server]` as a list; sort with `key=lambda s: (-len(findings[s]), s)`.
@@solution
UPGRADES = {
    23: ("Telnet", "SSH", 22),
    21: ("FTP", "SFTP", 22),
    80: ("HTTP", "HTTPS", 443),
    389: ("LDAP", "LDAPS", 636),
    110: ("POP3", "POP3S", 995),
    143: ("IMAP", "IMAPS", 993),
    161: ("SNMPv1/v2c", "SNMPv3", 161),
}
scan = {
    "fs01": [21, 445, 3389],
    "dc01": [53, 88, 389, 636],
    "mail01": [25, 110, 143, 443, 587],
    "sw-core": [22, 23, 161],
    "web01": [80, 443],
}

findings = {}
for server, ports in scan.items():
    findings[server] = []
    for port in ports:
        if port in UPGRADES:
            old, new, new_port = UPGRADES[port]
            findings[server].append(old)
            print(f"{server}: {old} on {port} -> use {new} ({new_port})")

order = sorted(findings, key=lambda s: (-len(findings[s]), s))
print("Remediation order:", ", ".join(f"{s} ({len(findings[s])})" for s in order))

@@exercise Design zone-to-zone policy
Traffic between zones is allowed only if a rule in `POLICY` permits that (source zone, destination zone, port). Check each requested flow, print `allow` or `deny`, and for denied flows that come from the `internet` zone to anything other than the `dmz`, add `(never: internet must only reach the DMZ)`. Then list unused policy rules (rules no request matched).
@@lang python
@@starter
POLICY = [
    ("internet", "dmz", 443),
    ("dmz", "internal", 5432),
    ("internal", "dmz", 22),
    ("internal", "internet", 443),
    ("mgmt", "internal", 22),
    ("guest", "internet", 443),
    ("internal", "ot", 502),
]
flows = [
    ("internet", "dmz", 443),
    ("internet", "internal", 3389),
    ("dmz", "internal", 5432),
    ("dmz", "internal", 445),
    ("guest", "internal", 445),
    ("internal", "internet", 443),
    ("mgmt", "internal", 22),
]
@@expected
internet -> dmz:443 allow
internet -> internal:3389 deny (never: internet must only reach the DMZ)
dmz -> internal:5432 allow
dmz -> internal:445 deny
guest -> internal:445 deny
internal -> internet:443 allow
mgmt -> internal:22 allow
Unused rules: internal->dmz:22; guest->internet:443; internal->ot:502
@@requires
POLICY
used
@@hint
Keep a `used = set()` of matching rules; at the end, `[r for r in POLICY if r not in used]`.
@@solution
POLICY = [
    ("internet", "dmz", 443),
    ("dmz", "internal", 5432),
    ("internal", "dmz", 22),
    ("internal", "internet", 443),
    ("mgmt", "internal", 22),
    ("guest", "internet", 443),
    ("internal", "ot", 502),
]
flows = [
    ("internet", "dmz", 443),
    ("internet", "internal", 3389),
    ("dmz", "internal", 5432),
    ("dmz", "internal", 445),
    ("guest", "internal", 445),
    ("internal", "internet", 443),
    ("mgmt", "internal", 22),
]

used = set()
for flow in flows:
    src, dst, port = flow
    if flow in POLICY:
        used.add(flow)
        print(f"{src} -> {dst}:{port} allow")
    else:
        note = " (never: internet must only reach the DMZ)" if src == "internet" and dst != "dmz" else ""
        print(f"{src} -> {dst}:{port} deny{note}")

unused = [r for r in POLICY if r not in used]
print("Unused rules:", "; ".join(f"{s}->{d}:{p}" for s, d, p in unused))

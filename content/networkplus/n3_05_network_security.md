@@lesson n3-netsec Network Attacks, ACLs, Segmentation and Remote Access
@@text
Network+ domain 4 (network security, about 14%) wants you to match attacks to defences, read and write access control lists, segment networks, and choose secure remote access. This lesson extends the attack table in the services lesson with how each attack works and how to stop it.

## Security concepts

- **CIA triad**: confidentiality, integrity, availability.
- **Defence in depth**, **least privilege**, **zero trust**, **separation of duties**.
- **AAA**: authentication, authorisation, accounting (RADIUS: UDP 1812/1813, encrypts only the password; **TACACS+**: TCP 49, encrypts the whole payload and separates the three A's, preferred for device admin).
- **MFA**, **SSO** (SAML, OAuth/OpenID Connect), **LDAP/LDAPS** (389/636), **Kerberos** (88).
- **PKI** and certificates; **key management**.
- **Compliance**: PCI DSS (card data), GDPR (EU personal data), data locality.

## Attacks and defences

| Attack | How it works | Defence |
|---|---|---|
| **ARP spoofing / poisoning** | attacker answers ARP for the gateway's IP → on-path | dynamic ARP inspection, 802.1X, encryption |
| **MAC flooding** | fills the CAM table → switch floods frames to all ports | port security (max MACs) |
| **VLAN hopping** (double tagging, switch spoofing) | reaches another VLAN | unused native VLAN, disable DTP, no user ports in the native VLAN |
| **Rogue DHCP** | wrong gateway/DNS given to clients | DHCP snooping |
| **DNS poisoning/spoofing** | fake answers | DNSSEC, secure resolvers |
| **Rogue AP / evil twin** | unauthorised AP, or a fake AP with your SSID | WIDS/WIPS, 802.1X with server certificate validation |
| **Deauthentication** | forged deauth frames kick clients off | 802.11w protected management frames (required in WPA3) |
| **DoS / DDoS** | exhausts bandwidth or resources | ISP/cloud scrubbing, rate limiting, CDN |
| **On-path (MITM)** | intercepts traffic | encryption (TLS, IPsec), certificate validation |
| **Social engineering**: phishing, **tailgating/piggybacking**, **shoulder surfing**, dumpster diving | tricks people | training, access control vestibules (mantraps), badges, privacy screens, shredding |
| **Malware**, **ransomware** | | EDR, patching, backups, segmentation |
| **Password attacks**: brute force, dictionary, password spraying | | MFA, lockout, long passwords |

## Access control lists

ACLs filter traffic on routers, firewalls and layer 3 switches:

- Rules are processed **top-down**; the **first match wins**.
- An **implicit deny** sits at the end: anything not permitted is dropped.
- Put **specific rules before general ones**.
- Standard ACLs match source only; extended ACLs match source, destination, protocol and port.
- Apply close to the source (extended) or destination (standard), on the right interface and **direction** (in/out).

```text
10 permit tcp 10.10.20.0/24 host 10.10.5.10 eq 443
20 permit udp 10.10.0.0/16 host 10.10.5.53 eq 53
30 deny   tcp any any eq 23
40 permit icmp 10.10.0.0/16 any
(implicit) deny ip any any
```

## Segmentation

- **VLANs** and subnets separate departments, guests, IoT, VoIP and management.
- **Screened subnet (DMZ)** for public servers.
- **Microsegmentation**: per-workload rules in data centres and clouds.
- **Air gap** for the most sensitive systems (OT, backups).
- **Honeypots/honeynets** to detect attackers.
- **Guest networks** with **client isolation**.

## Secure remote access

| Method | Use |
|---|---|
| **Site-to-site VPN** (IPsec) | connect offices permanently |
| **Client-to-site VPN** (SSL/TLS or IPsec) | remote workers; **full tunnel** (all traffic via HQ) or **split tunnel** (only corporate traffic) |
| **Clientless VPN** | via a browser portal |
| **SSH** (22) | device CLI management |
| **RDP** (3389) via a **gateway** or VPN | Windows desktops |
| **Jump box / bastion host** | one hardened host from which admins reach others |
| **Out-of-band management** | separate console network or a modem/LTE console server, for when the main network is down |
| **In-band management** | over the production network |

IPsec terms: **IKE** (key exchange, UDP 500, NAT traversal UDP 4500), **AH** (integrity, no encryption, breaks with NAT), **ESP** (encryption + integrity), **tunnel mode** (whole packet, site-to-site) vs **transport mode** (payload only, host-to-host).

## Physical security

Locks, badges, **access control vestibules**, cameras, **asset tags**, **tamper detection**, secure rack cabinets, visitor logs.

## Device hardening checklist

Change default credentials, disable unused ports/services, SSH not Telnet, HTTPS management, **SNMPv3**, **NTP**, central **AAA** (TACACS+), logging to syslog/SIEM, firmware updates, management on a separate VLAN or out of band, **banner** warnings, config backups.

@@quiz
An ACL has `permit ip any any` as rule 10 and `deny tcp any any eq 23` as rule 20. Is Telnet blocked?
- Yes, deny rules always win
* No, rule 10 matches first and permits it
- Only for outbound traffic
- Only if the implicit deny is removed
? First match wins; put specific rules first.

@@quiz
Which AAA protocol encrypts the entire payload and is preferred for administrator logins to network devices?
- RADIUS
* TACACS+
- LDAP
- Kerberos
? RADIUS encrypts only the password.

@@quiz
Admins need to reach switches even when the production network is down. What provides this?
- A split-tunnel VPN
* Out-of-band management (a console server on a separate network/LTE)
- A screened subnet
- Port mirroring
? In-band management relies on the network being up.

@@quiz
Which IPsec protocol provides encryption?
- AH
* ESP
- IKE
- GRE
? AH gives integrity only.

@@quiz
Someone follows an employee through a badge-controlled door without badging. What attack is this and what control helps?
* Tailgating; an access control vestibule (mantrap) and training
- Phishing; spam filtering
- Shoulder surfing; privacy screens
- Evil twin; WIPS
? Physical social engineering.

@@flashcards
ARP spoofing :: fake ARP replies for on-path attacks; DAI
MAC flooding :: fills CAM table; port security
VLAN hopping :: double tagging or switch spoofing
deauth attack :: stopped by 802.11w (PMF)
RADIUS :: UDP 1812/1813, encrypts password only
TACACS+ :: TCP 49, encrypts everything, device admin
ACL processing :: top-down, first match wins
implicit deny :: end of every ACL
screened subnet :: DMZ for public servers
microsegmentation :: per-workload rules
split tunnel :: only corporate traffic via VPN
jump box :: hardened host for admin access
out-of-band :: management network separate from production
IKE :: UDP 500 / 4500
AH vs ESP :: integrity only vs encryption
tunnel vs transport mode :: whole packet vs payload
access control vestibule :: mantrap against tailgating

@@drill
What TCP port does TACACS+ use?
= 49
? RADIUS uses UDP 1812/1813.

@@drill
What UDP port does IPsec NAT traversal use?
= 4500
? IKE alone uses UDP 500.

@@exercise Evaluate an ACL
Implement `evaluate(acl, packet)` that walks the rules top-down and returns the first matching rule's action and number, or `("deny", "implicit")`. A rule matches if the protocol matches (`ip` matches anything), the source and destination IPs are inside the rule's networks (`any` = `0.0.0.0/0`), and the port matches (`None` in the rule = any port). Print the verdict for each packet.
@@lang python
@@starter
import ipaddress

acl = [
    (10, "permit", "tcp", "10.10.20.0/24", "10.10.5.10/32", 443),
    (20, "permit", "udp", "10.10.0.0/16", "10.10.5.53/32", 53),
    (30, "deny", "tcp", "any", "any", 23),
    (40, "permit", "icmp", "10.10.0.0/16", "any", None),
    (50, "permit", "tcp", "10.10.99.0/24", "10.10.0.0/16", 22),
]
packets = [
    ("tcp", "10.10.20.15", "10.10.5.10", 443),
    ("tcp", "10.10.30.15", "10.10.5.10", 443),
    ("udp", "10.10.30.15", "10.10.5.53", 53),
    ("tcp", "10.10.99.4", "10.10.1.1", 23),
    ("tcp", "10.10.99.4", "10.10.1.1", 22),
    ("icmp", "10.10.40.2", "8.8.8.8", None),
]

def evaluate(acl, packet):
    raise NotImplementedError
@@expected
tcp 10.10.20.15 -> 10.10.5.10:443: permit (rule 10)
tcp 10.10.30.15 -> 10.10.5.10:443: deny (rule implicit)
udp 10.10.30.15 -> 10.10.5.53:53: permit (rule 20)
tcp 10.10.99.4 -> 10.10.1.1:23: deny (rule 30)
tcp 10.10.99.4 -> 10.10.1.1:22: permit (rule 50)
icmp 10.10.40.2 -> 8.8.8.8: permit (rule 40)
@@requires
def evaluate
"implicit"
ipaddress.ip_network(
@@hint
Write `net(text)` that turns `"any"` into `ipaddress.ip_network("0.0.0.0/0")`. Return as soon as a rule matches.
@@solution
import ipaddress

acl = [
    (10, "permit", "tcp", "10.10.20.0/24", "10.10.5.10/32", 443),
    (20, "permit", "udp", "10.10.0.0/16", "10.10.5.53/32", 53),
    (30, "deny", "tcp", "any", "any", 23),
    (40, "permit", "icmp", "10.10.0.0/16", "any", None),
    (50, "permit", "tcp", "10.10.99.0/24", "10.10.0.0/16", 22),
]
packets = [
    ("tcp", "10.10.20.15", "10.10.5.10", 443),
    ("tcp", "10.10.30.15", "10.10.5.10", 443),
    ("udp", "10.10.30.15", "10.10.5.53", 53),
    ("tcp", "10.10.99.4", "10.10.1.1", 23),
    ("tcp", "10.10.99.4", "10.10.1.1", 22),
    ("icmp", "10.10.40.2", "8.8.8.8", None),
]

def net(text):
    return ipaddress.ip_network("0.0.0.0/0" if text == "any" else text)

def evaluate(acl, packet):
    proto, src, dst, port = packet
    for number, action, r_proto, r_src, r_dst, r_port in acl:
        if r_proto not in ("ip", proto):
            continue
        if ipaddress.ip_address(src) not in net(r_src) or ipaddress.ip_address(dst) not in net(r_dst):
            continue
        if r_port is not None and r_port != port:
            continue
        return action, number
    return "deny", "implicit"

for p in packets:
    action, rule = evaluate(acl, p)
    port = f":{p[3]}" if p[3] else ""
    print(f"{p[0]} {p[1]} -> {p[2]}{port}: {action} (rule {rule})")

@@exercise Map attacks to defences
Security alerts name an attack. Using `DEFENCES`, print the recommended control for each alert; aliases in `ALIASES` map to the canonical attack name first. Then print the controls to deploy, deduplicated and sorted, with how many alerts each one would address.
@@lang python
@@starter
DEFENCES = {
    "arp spoofing": "dynamic ARP inspection",
    "mac flooding": "port security",
    "rogue dhcp": "DHCP snooping",
    "vlan hopping": "unused native VLAN + DTP off",
    "evil twin": "WIPS + 802.1X cert validation",
    "deauthentication": "802.11w protected management frames",
    "tailgating": "access control vestibule",
    "password spraying": "MFA",
}
ALIASES = {"arp poisoning": "arp spoofing", "double tagging": "vlan hopping", "piggybacking": "tailgating", "cam overflow": "mac flooding"}
alerts = ["ARP poisoning on VLAN 20", "Rogue DHCP offer from 10.0.20.77", "Piggybacking at door 3",
          "CAM overflow on access-sw2", "Evil twin SSID CorpWiFi", "Double tagging attempt", "ARP spoofing on VLAN 30", "Password spraying against VPN"]
@@expected
ARP poisoning on VLAN 20: arp spoofing -> dynamic ARP inspection
Rogue DHCP offer from 10.0.20.77: rogue dhcp -> DHCP snooping
Piggybacking at door 3: tailgating -> access control vestibule
CAM overflow on access-sw2: mac flooding -> port security
Evil twin SSID CorpWiFi: evil twin -> WIPS + 802.1X cert validation
Double tagging attempt: vlan hopping -> unused native VLAN + DTP off
ARP spoofing on VLAN 30: arp spoofing -> dynamic ARP inspection
Password spraying against VPN: password spraying -> MFA
Deploy:
  DHCP snooping (1)
  MFA (1)
  WIPS + 802.1X cert validation (1)
  access control vestibule (1)
  dynamic ARP inspection (2)
  port security (1)
  unused native VLAN + DTP off (1)
@@requires
ALIASES
Counter(
@@hint
Lowercase each alert and look for any key of `ALIASES` or `DEFENCES` inside it; resolve aliases with `ALIASES.get(name, name)`.
@@solution
from collections import Counter

DEFENCES = {
    "arp spoofing": "dynamic ARP inspection",
    "mac flooding": "port security",
    "rogue dhcp": "DHCP snooping",
    "vlan hopping": "unused native VLAN + DTP off",
    "evil twin": "WIPS + 802.1X cert validation",
    "deauthentication": "802.11w protected management frames",
    "tailgating": "access control vestibule",
    "password spraying": "MFA",
}
ALIASES = {"arp poisoning": "arp spoofing", "double tagging": "vlan hopping", "piggybacking": "tailgating", "cam overflow": "mac flooding"}
alerts = ["ARP poisoning on VLAN 20", "Rogue DHCP offer from 10.0.20.77", "Piggybacking at door 3",
          "CAM overflow on access-sw2", "Evil twin SSID CorpWiFi", "Double tagging attempt", "ARP spoofing on VLAN 30", "Password spraying against VPN"]

controls = Counter()
for alert in alerts:
    text = alert.lower()
    name = next((n for n in list(ALIASES) + list(DEFENCES) if n in text), None)
    attack = ALIASES.get(name, name)
    control = DEFENCES[attack]
    controls[control] += 1
    print(f"{alert}: {attack} -> {control}")
print("Deploy:")
for control in sorted(controls):
    print(f"  {control} ({controls[control]})")

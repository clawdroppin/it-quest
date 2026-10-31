@@lesson n3-services Network Services, Operations and Security
@@text
## DNS records

| Record | Holds |
|---|---|
| **A** / **AAAA** | name → IPv4 / IPv6 |
| **CNAME** | alias to another name |
| **MX** | mail servers for a domain (with priority) |
| **TXT** | text: **SPF**, **DKIM**, **DMARC** (email anti-spoofing), domain verification |
| **PTR** | IP → name (**reverse** lookup) |
| **NS** | the domain's name servers |
| **SOA** | zone settings: primary server, serial, timers |
| **SRV** | services, e.g. where the Active Directory domain controllers are |

**TTL** says how long resolvers may cache a record: lower it before a migration. **DNSSEC** signs records; **DoH/DoT** encrypt lookups. Recursive resolvers look things up for clients; **authoritative** servers hold the zone.

## DHCP details

The four steps are **DORA**: **D**iscover, **O**ffer, **R**equest, **A**cknowledge. A **scope** is the pool of addresses with options (gateway, DNS, lease time); **exclusions** keep addresses out of the pool; **reservations** tie an IP to a MAC. DHCP is a broadcast, so a router needs a **DHCP relay** (IP helper) to forward requests to a server in another subnet.

## Running a network

- **Monitoring**: **SNMP** (v3 is encrypted and authenticated; v1/v2c use plain-text community strings), **syslog**, **NetFlow** (who talks to whom, how much), packet capture, **baselines** of normal performance, and alerts.
- **Documentation**: physical and logical **diagrams**, rack diagrams, IP address management (**IPAM**), asset inventory, **change management**, **SLAs**.
- **Disaster recovery**: **RPO** (how much data you can lose) and **RTO** (how long you can be down); **MTTR** and **MTBF**; **hot**, **warm** and **cold** sites; backups; **UPS** and generators.
- **High availability**: redundant links, devices and power; **active-active** vs **active-passive**.

## Security

- **Firewalls**: **stateful** (tracks connections) vs stateless; **NGFW** (application awareness, IPS, user identity); **ACLs** on routers are read top-down with an implicit **deny all** at the end.
- **DMZ / screened subnet**: public servers live in a separate zone between the internet and the internal network.
- **NAC** (network access control) checks devices before letting them on (802.1X, posture checks). **Zero trust**: never trust by location; verify every user and device every time.
- **IDS** detects and alerts; **IPS** sits inline and blocks.
- **Hardening**: disable unused ports and services, change defaults, update firmware, SSH not Telnet, SNMPv3, separate management VLAN, **port security**, DHCP snooping, dynamic ARP inspection.

| Attack | What it does |
|---|---|
| **DoS / DDoS** | floods a service until it's unavailable |
| **On-path** (man-in-the-middle) | intercepts traffic, e.g. through **ARP spoofing** |
| **DNS poisoning** | fake DNS answers send users to the attacker |
| **Rogue DHCP** / **rogue AP** | an unauthorized server or access point on the network |
| **Evil twin** | a fake AP with the real SSID |
| **VLAN hopping** | double tagging or switch spoofing to reach other VLANs |
| **MAC flooding** | fills the switch's table so it floods all traffic |
| **Deauthentication** | kicks Wi-Fi clients off (often before an evil twin) |

@@quiz
Which DNS record directs email for a domain to its mail servers?
* MX
- A
- PTR
- CNAME
? MX records list mail exchangers with priorities.

@@quiz
Which TXT-record based standard tells receiving servers which IPs may send mail for your domain?
* SPF
- PTR
- SOA
- DNSSEC
? SPF lists authorized senders; DKIM signs messages; DMARC sets the policy.

@@quiz
Clients in a new subnet can't get DHCP addresses from the server in another subnet. What's needed on the router?
* A DHCP relay / IP helper address
- A new DNS zone
- STP
- A PTR record
? DHCP discovery is a broadcast, and routers don't forward broadcasts.

@@quiz
What is the order of the DHCP process?
* Discover, Offer, Request, Acknowledge
- Request, Offer, Discover, Acknowledge
- Offer, Discover, Acknowledge, Request
- Discover, Request, Offer, Accept
? DORA.

@@quiz
The business can accept losing at most 1 hour of data. Which metric is that?
* RPO
- RTO
- MTBF
- SLA
? Recovery point objective is about data loss; recovery time objective is about downtime.

@@quiz
Which device inspects traffic inline and can block attacks in real time?
* IPS
- IDS
- Hub
- NetFlow collector
? An IDS only detects and alerts.

@@quiz
A router ACL has three permit rules and nothing else. What happens to traffic that matches none of them?
* It's denied by the implicit deny at the end
- It's permitted
- It's logged and permitted
- It's sent to the default route
? Every ACL ends with an invisible "deny any".

@@quiz
Users near a café's Wi-Fi are connecting to an access point with the café's SSID that the café doesn't own. What attack is this?
* Evil twin
- VLAN hopping
- MAC flooding
- DNS poisoning
? A fake AP imitating the real one, to intercept traffic.

@@quiz
Which SNMP version provides encryption and authentication?
* SNMPv3
- SNMPv1
- SNMPv2c
- None of them
? v1 and v2c send community strings in plain text.

@@flashcards
A / AAAA :: Name to IPv4 / IPv6
CNAME :: Alias
MX :: Mail servers
PTR :: Reverse lookup (IP to name)
SOA :: Zone authority settings
SRV :: Service locations
SPF / DKIM / DMARC :: Email anti-spoofing: allowed senders / signatures / policy
TTL :: How long DNS answers may be cached
DORA :: DHCP: Discover, Offer, Request, Acknowledge
DHCP relay :: Forwards DHCP broadcasts to another subnet
RPO / RTO :: Max data loss / max downtime
hot / warm / cold site :: Ready now / partly ready / space only
NetFlow :: Traffic flow statistics
implicit deny :: Unmatched traffic is blocked
screened subnet (DMZ) :: Zone for public-facing servers
IDS vs IPS :: Detects vs blocks
NAC :: Checks devices before network access
zero trust :: Verify every request, never trust by location
ARP spoofing :: Fake ARP replies for on-path attacks
evil twin :: Rogue AP with a legitimate SSID
VLAN hopping :: Reaching other VLANs by double tagging or switch spoofing

@@drill
Which DNS record type maps a name to an IPv6 address?
= AAAA
? "Quad A".

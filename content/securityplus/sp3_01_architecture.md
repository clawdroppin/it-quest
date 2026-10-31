@@lesson sp3-architecture Security Architecture and Data Protection
@@text
## Architecture models

- **On-premises**, **cloud**, **hybrid**; **serverless** and **microservices**; **containers**; **IaC** (infrastructure as code: reviewable, repeatable, but a mistake is copied everywhere).
- **Responsibility matrix**: in the cloud, the provider and customer split security duties (IaaS: you patch the OS; SaaS: you manage users and data).
- **Segmentation**: separate networks and zones (VLANs, subnets, **microsegmentation** between workloads). **Air-gapped** networks have no connection at all (industrial systems, backups).
- **ICS/SCADA** (factories, power plants) and **IoT** devices: long lifetimes, hard to patch: isolate them. **RTOS** and **embedded** systems likewise.
- **SDN** and **SASE** (secure access service edge: networking and security delivered from the cloud), **SD-WAN**.

## Secure infrastructure

| Concept | Meaning |
|---|---|
| **Device placement** | put firewalls, IPS, sensors where traffic crosses zones |
| **Security zones** | trusted, untrusted, screened subnet (DMZ) |
| **Fail-open / fail-closed** | if a device fails, does traffic pass or stop? (security usually fails closed) |
| **Active / passive** devices | inline IPS vs monitoring copy (TAP or SPAN port) |
| **Jump server** | hardened host admins go through to reach sensitive systems |
| **Proxy server** | forward (users → internet, filtering) or reverse (internet → servers) |
| **Load balancer** | spreads traffic; can terminate TLS |
| **WAF** | web application firewall: blocks SQLi/XSS at layer 7 |
| **UTM / NGFW** | all-in-one firewall with IPS, filtering, app awareness |
| **802.1X / EAP** | port-based network access control |
| **VPN / IPsec / TLS** | encrypted tunnels; IPsec **tunnel** mode (whole packet) vs **transport** mode (payload) |

## Data protection

- **Data types**: regulated (PII, health, card data), trade secrets, intellectual property, legal, financial.
- **Classifications**: public, internal, confidential/sensitive, restricted/critical (government: unclassified, secret, top secret).
- **Data sovereignty**: data is subject to the laws of the country where it's stored. **Geolocation** and **geofencing** restrict where data or logins can come from.
- Methods: **encryption**, **hashing**, **masking**, **tokenization**, **obfuscation**, **segmentation**, **permission restrictions**, **DLP** (data loss prevention: blocks sensitive data from leaving by email, USB or upload).

## Resilience and recovery

- **High availability**: load balancing, **clustering**; **site considerations**: hot, warm, cold sites, **geographic dispersion**.
- **Platform diversity** and **multi-cloud** avoid one weakness affecting everything.
- **Backups**: on-site/off-site, frequency, encryption, **snapshots**, **replication**, **journaling**; test them.
- **Power**: UPS (short outages), generators (long ones).
- **Testing**: **tabletop exercises** (talk through a scenario), **failover** tests, **simulations**, **parallel processing**.
- **Capacity planning**: people, technology and infrastructure.

@@quiz
An admin must reach database servers in a restricted zone. Instead of direct access from their laptop, they first connect to a hardened, monitored host. What is that host?
* A jump server
- A reverse proxy
- A honeypot
- A load balancer
? Jump servers (bastion hosts) concentrate and audit admin access.

@@quiz
Which device protects a web application by inspecting HTTP requests for SQL injection and XSS?
* A WAF
- A layer 2 switch
- A forward proxy
- A TAP
? Web application firewalls work at layer 7 on web traffic.

@@quiz
A European company's customer data must stay in the EU because of the laws that apply to it there. Which concept is this?
* Data sovereignty
- Tokenization
- Fail-open
- Microsegmentation
? Data is subject to the laws of where it's stored.

@@quiz
An employee tries to email a spreadsheet of customers' card numbers to a personal address, and the email is blocked. Which tool did this?
* DLP
- WAF
- IDS
- Jump server
? Data loss prevention detects and blocks sensitive data leaving.

@@quiz
A firewall fails. Security requires that no traffic passes until it's fixed. What's this called?
* Fail-closed
- Fail-open
- Active-active
- Load balancing
? Fail-closed favours confidentiality over availability.

@@quiz
The security team gathers department leads to talk through what they'd do if ransomware hit the file servers, without touching any systems. What is this?
* A tabletop exercise
- A failover test
- A penetration test
- A simulation of an attack on production
? Discussion-based exercises find gaps in plans cheaply.

@@quiz
Factory control systems can't be patched often. What's the best architectural protection?
* Segment or air-gap them from the corporate network
- Put them in the DMZ
- Give them public IPs for remote support
- Run them as admin
? Isolation limits who and what can reach them.

@@flashcards
responsibility matrix :: Who secures what between cloud provider and customer
IaC :: Infrastructure as code: Terraform, CloudFormation, Bicep
microsegmentation :: Fine-grained isolation between workloads
air gap :: No network connection at all
ICS / SCADA :: Industrial control systems: isolate them
SASE :: Network and security services from the cloud edge
fail-open / fail-closed :: Traffic passes / is blocked when a device fails
jump server :: Hardened host for admin access
reverse proxy :: Sits in front of servers for incoming traffic
WAF :: Web application firewall
TAP / SPAN :: Copies traffic for passive monitoring
IPsec tunnel vs transport :: Encrypts the whole packet vs the payload
data sovereignty :: Data subject to the laws where it's stored
DLP :: Data loss prevention
tabletop exercise :: Discussion-based incident scenario
geographic dispersion :: Sites far apart so one disaster can't hit all

@@drill
Which kind of exercise talks through an incident scenario in a meeting room, without touching systems?
= tabletop
= tabletop exercise
? Cheap, fast and very effective at finding gaps in plans.

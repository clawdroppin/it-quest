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

@@exercise Mask, tokenize and hash sensitive fields
Protect a customer export with three of the lesson's methods:

- `mask(card)`: show only the last 4 digits (`**** **** **** 1111`), for the support screen.
- `tokenize(card, vault)`: replace the card number with a random-looking token and keep the mapping in `vault`, so the payment system can swap it back with `detokenize`. The same card always gets the same token.
- `hash_email(email)`: SHA-256 of the lowercased address, so analysts can count unique customers without seeing addresses.

Print the support view, the analytics view and prove the token round-trips. (Tokens use a counter so the output is repeatable; a real vault would use random values.)
@@lang python
@@starter
import hashlib

customers = [
    {"name": "Aiko Tanaka", "email": "Aiko@example.jp", "card": "4111111111111111"},
    {"name": "Ben Ito", "email": "ben@example.jp", "card": "5500005555555559"},
    {"name": "Aiko Tanaka", "email": "aiko@example.jp", "card": "4111111111111111"},
]

def mask(card):
    raise NotImplementedError


def tokenize(card, vault):
    raise NotImplementedError


def detokenize(token, vault):
    raise NotImplementedError


def hash_email(email):
    raise NotImplementedError


vault = {}
print("Support view:")
for c in customers:
    print(f"  {c['name']}: {mask(c['card'])}")
print("Stored by the shop:")
for c in customers:
    print(f"  {c['name']}: {tokenize(c['card'], vault)}")
print("Analytics view:")
hashes = [hash_email(c["email"]) for c in customers]
for h in hashes:
    print("  " + h[:16] + "...")
print(f"  unique customers: {len(set(hashes))}")
token = tokenize(customers[1]["card"], vault)
print("Round trip:", detokenize(token, vault) == customers[1]["card"])
@@expected
Support view:
  Aiko Tanaka: **** **** **** 1111
  Ben Ito: **** **** **** 5559
  Aiko Tanaka: **** **** **** 1111
Stored by the shop:
  Aiko Tanaka: tok_000001
  Ben Ito: tok_000002
  Aiko Tanaka: tok_000001
Analytics view:
  0203afc6e22ea9e0...
  cd2e5b8f59600c3d...
  0203afc6e22ea9e0...
  unique customers: 2
Round trip: True
@@requires
def mask
def tokenize
def hash_email
hashlib.sha256
.lower()
@@hint
Masking is string slicing, tokenization is a dict lookup in both directions, and hashing is `hashlib.sha256(text.encode()).hexdigest()`. Lowercase the email before hashing or `Aiko@` and `aiko@` count as two people.
@@solution
import hashlib

customers = [
    {"name": "Aiko Tanaka", "email": "Aiko@example.jp", "card": "4111111111111111"},
    {"name": "Ben Ito", "email": "ben@example.jp", "card": "5500005555555559"},
    {"name": "Aiko Tanaka", "email": "aiko@example.jp", "card": "4111111111111111"},
]

def mask(card):
    return "**** **** **** " + card[-4:]

def tokenize(card, vault):
    for token, value in vault.items():
        if value == card:
            return token
    token = f"tok_{len(vault) + 1:06d}"
    vault[token] = card
    return token

def detokenize(token, vault):
    return vault[token]

def hash_email(email):
    return hashlib.sha256(email.lower().encode()).hexdigest()

vault = {}
print("Support view:")
for c in customers:
    print(f"  {c['name']}: {mask(c['card'])}")
print("Stored by the shop:")
for c in customers:
    print(f"  {c['name']}: {tokenize(c['card'], vault)}")
print("Analytics view:")
hashes = [hash_email(c["email"]) for c in customers]
for h in hashes:
    print("  " + h[:16] + "...")
print(f"  unique customers: {len(set(hashes))}")
token = tokenize(customers[1]["card"], vault)
print("Round trip:", detokenize(token, vault) == customers[1]["card"])

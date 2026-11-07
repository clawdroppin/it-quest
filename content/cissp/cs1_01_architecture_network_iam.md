@@lesson cs1-architecture-network-iam Architecture, Cryptography, Networks and Identity
@@text
## Security models (domain 3)

| Model | Protects | Rules |
|---|---|---|
| **Bell-LaPadula** | confidentiality | **no read up** (simple security), **no write down** (★-property) |
| **Biba** | integrity | **no read down**, **no write up** |
| **Clark-Wilson** | integrity in business | well-formed transactions, separation of duties, access only through programs (the access triple) |
| **Brewer-Nash** (Chinese Wall) | conflicts of interest | once you see Company A's data, you can't see its competitor's |
| Graham-Denning, Harrison-Ruzzo-Ullman | | rules for creating and deleting subjects, objects and rights |

Memory aid: in Bell-LaPadula, a Secret-cleared user can't read Top Secret (up) and can't write to Unclassified (down, which would leak). Biba is the mirror image.

Other architecture ideas:

- **Trusted Computing Base** (TCB), the **reference monitor** (mediates every access; must be tamper-proof, always invoked, and small enough to verify), the **security kernel** that implements it.
- **Common Criteria** (ISO 15408): **EAL1–EAL7** evaluation assurance levels, against a Protection Profile.
- Secure design principles: **secure defaults**, fail securely, **zero trust** (never trust the network: verify every request), privacy by design, keep it simple, **shared responsibility** in the cloud.
- Vulnerabilities to know: TOCTOU (race conditions), covert channels (timing, storage), emanations (TEMPEST), side channels, firmware and supply chain attacks, IoT and ICS/SCADA (OT networks), and **virtualization escape**.

## Cryptography

- **Symmetric** (one shared key; fast; bulk data): **AES** (128/192/256), ChaCha20; legacy DES/3DES. Problem: key distribution; *n(n−1)/2* keys for n people.
- **Asymmetric** (public/private key pair; slow; key exchange and signatures): **RSA**, **ECC** (shorter keys for the same strength), **Diffie-Hellman** (key agreement).
- **Hashing** (one-way, integrity): **SHA-256**, SHA-3; MD5 and SHA-1 are broken for collision resistance. Passwords: salted, slow hashes (**bcrypt, scrypt, Argon2, PBKDF2**).
- **Digital signature**: hash the message, encrypt the hash with the sender's **private** key; anyone verifies with the sender's **public** key → integrity, authenticity, **non-repudiation**.
- To send a secret to Bob: encrypt with **Bob's public key**; only Bob's private key decrypts.
- **PKI**: a **certificate authority** signs certificates (X.509) binding a public key to an identity. Revocation via **CRL** or **OCSP**. Hybrid crypto (TLS): asymmetric to agree on a session key, symmetric for the data.
- Modes: avoid **ECB** (identical blocks give identical ciphertext); use **GCM** (authenticated encryption).
- **Post-quantum cryptography**: NIST's new standards (ML-KEM, ML-DSA) replace RSA and ECC, which quantum computers would break. "Harvest now, decrypt later" makes this urgent for long-lived secrets.
- Attacks: brute force, dictionary and **rainbow tables** (defeated by salts), birthday attacks on hashes, man-in-the-middle, side channels, replay (defeated by nonces and timestamps).

## Physical security

Layers from outside in: site selection, fences, lighting, CCTV, guards and dogs, **mantraps** (access control vestibules, against tailgating), badges, locks. Fire: suppression for data centres is usually **clean agent gas** (FM-200, inert gases) or **pre-action** sprinklers (the pipes fill only after a detector triggers). Fire classes: A (common combustibles), B (liquids), C (electrical; class K kitchen in the US). Temperature and humidity (about 40–60%) prevent static and condensation.

## Network security (domain 4)

- **OSI** layers and what lives there: 1 physical (hubs, cables), 2 data link (switches, MAC, ARP), 3 network (routers, IP, ICMP, IPsec), 4 transport (TCP/UDP), 5 session, 6 presentation (encryption, formats), 7 application (HTTP, DNS, SMTP). Attacks map to layers too: ARP spoofing (2), IP spoofing (3), SYN flood (4).
- **Firewalls**: packet filter (stateless), **stateful**, application-level proxy, **next-generation** (application aware, IPS, identity), **WAF**. **IDS** detects; **IPS** sits inline and blocks. Signature versus anomaly (behavior) based.
- **Segmentation**: VLANs, **DMZ** (screened subnet) for public servers, **microsegmentation**, **zero trust network access**.
- **Secure protocols**: TLS 1.2/1.3, SSH, **IPsec** (AH: integrity only; **ESP**: encryption too; transport versus tunnel mode; IKE for keys), SFTP/FTPS, DNSSEC, SNMPv3, 802.1X (port-based NAC with EAP).
- **Wireless**: WPA3 (SAE); avoid WEP and WPA/TKIP. Attacks: rogue APs, evil twins, deauthentication.
- **Converged protocols**: VoIP, iSCSI, Fibre Channel over Ethernet, SDN. **CDNs**, **SD-WAN**, **SASE** (networking and security as a cloud service).

## Identity and access management (domain 5)

- **Identification** (who you claim to be) → **authentication** (prove it) → **authorization** (what you may do) → **accountability** (logs tied to identities). AAA.
- Factors: something you **know**, **have**, **are** (biometrics), plus somewhere you are and something you do. MFA means *different* types (a password and a PIN are both "know").
- Biometric errors: **FRR** (type I, false rejection), **FAR** (type II, false acceptance, the more dangerous one); the **CER/EER** where they cross is used to compare devices: lower is better.
- **Access control models**: **DAC** (the owner decides: file permissions), **MAC** (labels and clearances decided by the system: military), **RBAC** (by job role), **ABAC** (attributes and context: department, time, device), rule-based (firewall rules), risk-based.
- **Federation and SSO**: **SAML** (XML assertions, enterprise web SSO), **OAuth 2.0** (*authorization*: delegated access with tokens), **OpenID Connect** (*authentication* on top of OAuth), **Kerberos** (tickets from a KDC: TGT, then service tickets; needs synchronized clocks).
- **Identity lifecycle**: provisioning, regular **access reviews** (recertification), and prompt **deprovisioning** when people leave or move (avoid *privilege creep*). **PAM** (privileged access management) vaults and records admin sessions; just-in-time access.

@@quiz
In the Bell-LaPadula model, a user with Secret clearance wants to write a document into an Unclassified folder. What happens?
* It's denied: the ★-property forbids writing down
- It's allowed: writing down is always fine
- It's denied: no read up
- It's allowed after a hash check
? Writing down could leak Secret information into a lower level.

@@quiz
Which model is concerned with integrity and uses "no read down, no write up"?
* Biba
- Bell-LaPadula
- Brewer-Nash
- Clark-Wilson
? Clark-Wilson is also about integrity, but through well-formed transactions.

@@quiz
Alice wants to send Bob a message that only Bob can read, and that Bob knows came from Alice. What does she do?
* Sign it with her private key, and encrypt it with Bob's public key
- Encrypt it with her public key
- Encrypt it with Bob's private key
- Hash it with MD5
? Confidentiality: the recipient's public key. Authenticity: the sender's private key.

@@quiz
When comparing biometric systems, which measure is most useful?
* The crossover error rate (CER), lower is better
- The false rejection rate alone
- The false acceptance rate alone
- The enrollment time
? The CER balances FAR and FRR.

@@quiz
A web app lets users log in with their company identity provider and get an ID token. Which protocol is designed for that authentication?
* OpenID Connect
- OAuth 2.0 alone
- Kerberos
- LDAP
? OAuth 2.0 is for authorization (access tokens); OIDC adds authentication (ID tokens).

@@quiz
Which IPsec component provides confidentiality?
* ESP
- AH
- IKE
- ISAKMP alone
? AH provides integrity and authentication only.

@@quiz
Access is granted based on the user's department, the time of day and whether the device is managed. Which model is this?
* ABAC
- DAC
- MAC
- RBAC
? Attribute-based access control uses attributes of the subject, object and environment.

@@flashcards
Bell-LaPadula :: confidentiality: no read up, no write down
Biba :: integrity: no read down, no write up
Brewer-Nash :: Chinese Wall: conflicts of interest
reference monitor :: mediates all access: tamper-proof, always invoked, verifiable
Common Criteria :: EAL1–EAL7
symmetric keys for n people :: n(n−1)/2
digital signature :: hash encrypted with the sender's private key
password hashing :: salted and slow: bcrypt, scrypt, Argon2
ECB mode :: avoid: patterns leak
OCSP :: online certificate status check
mantrap :: stops tailgating
FAR / FRR :: type II (worse) / type I
CER :: where FAR = FRR; lower is better
SAML / OAuth / OIDC :: enterprise SSO / authorization / authentication
Kerberos :: tickets from a KDC; needs synchronized time
privilege creep :: access accumulating as people change jobs

@@drill
How many symmetric keys are needed for 10 people to each communicate privately with every other?
= 45
? n(n−1)/2 = 10 × 9 / 2.

@@exercise Key counts and brute-force time
Compare key management for symmetric and asymmetric cryptography: symmetric needs n(n−1)/2 keys for n people to talk privately in pairs, asymmetric needs 2n (a key pair each). Then estimate the average brute-force time (half the keyspace) for several key lengths at 10¹² guesses per second, in years.
@@lang python
@@starter
def symmetric_keys(n):
    raise NotImplementedError

def asymmetric_keys(n):
    raise NotImplementedError

def brute_force_years(bits, rate=1e12):
    raise NotImplementedError

for n in (2, 10, 100, 1000):
    print(f"{n:>5} people: symmetric {symmetric_keys(n):>7}  asymmetric {asymmetric_keys(n):>5}")
for bits in (40, 56, 80, 128):
    print(f"{bits:>3}-bit key: {brute_force_years(bits):.3g} years on average")
@@expected
    2 people: symmetric       1  asymmetric     4
   10 people: symmetric      45  asymmetric    20
  100 people: symmetric    4950  asymmetric   200
 1000 people: symmetric  499500  asymmetric  2000
 40-bit key: 1.74e-08 years on average
 56-bit key: 0.00114 years on average
 80-bit key: 1.92e+04 years on average
128-bit key: 5.39e+18 years on average
@@requires
def symmetric_keys
def brute_force_years
2 ** (bits - 1)
@@hint
`n * (n - 1) // 2` and `2 * n`. The average search covers half the keyspace: `2 ** (bits - 1)` guesses, divided by the rate gives seconds; divide by `365.25 * 24 * 3600` for years.
@@solution
def symmetric_keys(n):
    return n * (n - 1) // 2

def asymmetric_keys(n):
    return 2 * n

def brute_force_years(bits, rate=1e12):
    return 2 ** (bits - 1) / rate / (365.25 * 24 * 3600)

for n in (2, 10, 100, 1000):
    print(f"{n:>5} people: symmetric {symmetric_keys(n):>7}  asymmetric {asymmetric_keys(n):>5}")
for bits in (40, 56, 80, 128):
    print(f"{bits:>3}-bit key: {brute_force_years(bits):.3g} years on average")

@@exercise Choose SAML, OAuth 2.0 or OpenID Connect
Pick the federation protocol from the lesson for each integration. Write `protocol(need)`:

- an app needs to **act on the user's behalf** with another API (read their calendar, post to their drive) without learning their password → **OAuth 2.0** (authorization, access tokens);
- a modern web or mobile app needs to **know who the user is** → **OpenID Connect** (authentication on top of OAuth, ID tokens);
- an enterprise web app needs **SSO** with the company identity provider, especially older apps → **SAML** (XML assertions);
- a Windows domain inside the network → **Kerberos**.

A need can ask for both identity and delegated access: that is OIDC plus OAuth scopes. Print each integration with its protocol and the token or assertion it uses.
@@lang python
@@starter
TOKEN = {"OAuth 2.0": "access token", "OpenID Connect": "ID token (JWT)", "SAML": "XML assertion",
         "Kerberos": "ticket (TGT, then service tickets)", "OpenID Connect + OAuth 2.0 scopes": "ID token + access token"}
integrations = [
    ("Expense app reads the user's calendar", {"delegated": True}),
    ("Mobile app 'Sign in with Google'", {"identity": True, "modern": True}),
    ("Legacy HR portal SSO with the corporate IdP", {"identity": True, "enterprise_sso": True}),
    ("File shares on the Windows domain", {"on_prem_windows": True}),
    ("New web app needs login and access to the user's files", {"identity": True, "modern": True, "delegated": True}),
]

def protocol(need):
    raise NotImplementedError


for name, need in integrations:
    p = protocol(need)
    print(f"{name}: {p} ({TOKEN[p]})")
@@expected
Expense app reads the user's calendar: OAuth 2.0 (access token)
Mobile app 'Sign in with Google': OpenID Connect (ID token (JWT))
Legacy HR portal SSO with the corporate IdP: SAML (XML assertion)
File shares on the Windows domain: Kerberos (ticket (TGT, then service tickets))
New web app needs login and access to the user's files: OpenID Connect + OAuth 2.0 scopes (ID token + access token)
@@requires
def protocol
.get(
@@hint
Check the most specific needs first (Windows domain, enterprise SSO), then identity together with delegation, then each on its own. Remember: OAuth 2.0 alone is authorization, not login.
@@solution
TOKEN = {"OAuth 2.0": "access token", "OpenID Connect": "ID token (JWT)", "SAML": "XML assertion",
         "Kerberos": "ticket (TGT, then service tickets)", "OpenID Connect + OAuth 2.0 scopes": "ID token + access token"}
integrations = [
    ("Expense app reads the user's calendar", {"delegated": True}),
    ("Mobile app 'Sign in with Google'", {"identity": True, "modern": True}),
    ("Legacy HR portal SSO with the corporate IdP", {"identity": True, "enterprise_sso": True}),
    ("File shares on the Windows domain", {"on_prem_windows": True}),
    ("New web app needs login and access to the user's files", {"identity": True, "modern": True, "delegated": True}),
]

def protocol(need):
    if need.get("on_prem_windows"):
        return "Kerberos"
    if need.get("enterprise_sso"):
        return "SAML"
    if need.get("identity") and need.get("delegated"):
        return "OpenID Connect + OAuth 2.0 scopes"
    if need.get("identity"):
        return "OpenID Connect"
    if need.get("delegated"):
        return "OAuth 2.0"
    return "ask the identity team"

for name, need in integrations:
    p = protocol(need)
    print(f"{name}: {p} ({TOKEN[p]})")

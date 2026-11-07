@@lesson cs1-network-iam-deep Network Security and Identity in Depth (Domains 4 and 5)
@@text
Domain 4 (Communication and Network Security) and Domain 5 (Identity and Access Management) together are about a quarter of the exam. CISSP doesn't test command syntax; it tests whether you can pick the right architecture and recognise how protocols protect (or fail to protect) a system. This lesson deepens the key areas: secure network architecture and protocols, Kerberos and federation, and the identity lifecycle with separation of duties.

## Secure network architecture

- **Defence in depth** and **segmentation**: VLANs, subnets, firewalls between zones, **microsegmentation** (per workload), and a screened subnet (DMZ) for public services.
- **Zero trust** (NIST SP 800-207): no implicit trust from network location; every request is authenticated, authorised and encrypted, with a policy decision point (PDP) and policy enforcement point (PEP).
- **Firewalls**: packet filter (stateless), stateful inspection, application-level proxy, next-generation (application awareness, IPS, user identity), WAF (HTTP), and host firewalls.
- **IPsec**: AH (integrity and authentication, no encryption) and **ESP** (encryption plus integrity); **transport mode** protects the payload between hosts, **tunnel mode** wraps the whole packet (site-to-site VPNs). IKE negotiates the security associations.
- **TLS 1.3** (forward secrecy by default), **SSH**, **802.1X** with EAP for port-based network access control (supplicant, authenticator, authentication server, usually RADIUS).
- Wireless: **WPA3** (SAE replaces the PSK handshake), WPA3-Enterprise with 802.1X.
- Attacks to recognise: ARP spoofing, VLAN hopping, DNS poisoning (defend with DNSSEC), SYN floods, rogue access points, on-path attacks.

## Kerberos

1. The client authenticates to the **KDC**'s **Authentication Service** and receives a **TGT** (ticket-granting ticket) encrypted with the KDC's key.
2. The client presents the TGT to the **Ticket-Granting Service** and receives a **service ticket** for a specific server.
3. The client presents the service ticket to the server, with an **authenticator** (timestamp) proving freshness.
- Relies on **symmetric keys** and **synchronised clocks** (default tolerance 5 minutes) to stop replay. The KDC is a single point of failure and a high-value target.
- Attacks: pass-the-ticket, golden ticket (forged TGTs with the krbtgt key), kerberoasting (offline cracking of service tickets for weak service-account passwords). Defend with strong service-account passwords, managed service accounts, monitoring and AES.

## Federation and SSO

- **SAML 2.0**: XML assertions from an identity provider (IdP) to a service provider (SP); common for enterprise web SSO.
- **OAuth 2.0**: **authorisation** (delegated access with access tokens); **OpenID Connect** adds **authentication** (an ID token, a JWT) on top of OAuth.
- **Just-in-time provisioning** and **SCIM** for account creation in SaaS.

## Access control models

DAC (owner decides), MAC (labels and clearances), RBAC (roles), ABAC (attributes and policies), rule-based (firewall rules), risk-based/adaptive (context such as location and device).

## The identity lifecycle

**Provisioning** (joiner) → **changes** (mover: remove old access, not just add new) → **deprovisioning** (leaver: disable promptly). Periodic **access reviews** catch **privilege creep**. **Separation of duties** prevents one person from completing a sensitive process alone (create a vendor *and* approve its payment); **least privilege** and **need to know** limit access; privileged access management (PAM) adds just-in-time elevation and session recording.

@@quiz
Which IPsec configuration protects the entire original packet for a site-to-site VPN?
- AH in transport mode
- ESP in transport mode
* ESP in tunnel mode
- IKE alone
? Tunnel mode wraps the whole packet.

@@quiz
Why does Kerberos require synchronised clocks?
* Timestamps in authenticators stop replay attacks
- Tickets are encrypted with the time
- The KDC sleeps at night
- To expire passwords
? Default tolerance is about 5 minutes.

@@quiz
Which protocol adds authentication (an ID token) on top of OAuth 2.0?
- SAML
* OpenID Connect
- Kerberos
- RADIUS
? OAuth alone is authorisation.

@@quiz
An employee moves from finance to marketing and keeps their finance access. What's the problem?
- Least privilege
* Privilege creep, a failure of the mover process
- Separation of duties
- Federation
? Remove old access on a move.

@@quiz
In 802.1X, which component is usually the RADIUS server?
- Supplicant
- Authenticator
* Authentication server
- Access point
? The switch or AP is the authenticator.

@@flashcards
zero trust :: never trust network location
PDP / PEP :: policy decision / enforcement point
microsegmentation :: per-workload isolation
stateful firewall :: tracks connections
NGFW :: application aware, IPS, identity
AH :: integrity, no encryption
ESP :: encryption and integrity
tunnel mode :: wraps the whole packet
IKE :: negotiates security associations
802.1X :: supplicant, authenticator, auth server
WPA3 SAE :: replaces PSK handshake
KDC :: AS + TGS
TGT :: ticket-granting ticket
golden ticket :: forged TGT with krbtgt key
kerberoasting :: crack service tickets offline
SAML :: XML assertions IdP to SP
OAuth 2.0 :: delegated authorisation
OIDC :: authentication on OAuth
SCIM :: cross-domain provisioning
privilege creep :: accumulated access
separation of duties :: no one completes alone

@@drill
Which Kerberos component issues service tickets?
= TGS
= ticket-granting service
= ticket granting service
? The AS issues the TGT.

@@drill
Which IPsec protocol provides encryption?
= ESP
= encapsulating security payload
? AH doesn't encrypt.

@@exercise Simulate Kerberos ticket checks
Simulate the checks a service makes when a Kerberos service ticket arrives: the ticket must be for this service, not expired, the authenticator's timestamp must be within the allowed clock skew of the server's clock, the authenticator's client must match the ticket's client, and the same authenticator must not be replayed. Print the decision and reason for each request.
@@lang python
@@starter
SKEW = 300  # seconds
SERVICE = "HTTP/intranet.example.jp"
seen = set()
requests = [
    {"ticket": {"service": SERVICE, "client": "aiko", "expires": 10_000}, "auth": {"client": "aiko", "ts": 5_000}, "server_time": 5_030},
    {"ticket": {"service": SERVICE, "client": "aiko", "expires": 10_000}, "auth": {"client": "aiko", "ts": 5_000}, "server_time": 5_040},
    {"ticket": {"service": SERVICE, "client": "ben", "expires": 10_000}, "auth": {"client": "ben", "ts": 6_000}, "server_time": 6_400},
    {"ticket": {"service": SERVICE, "client": "chen", "expires": 7_000}, "auth": {"client": "chen", "ts": 7_100}, "server_time": 7_120},
    {"ticket": {"service": "CIFS/files.example.jp", "client": "dana", "expires": 10_000}, "auth": {"client": "dana", "ts": 8_000}, "server_time": 8_010},
    {"ticket": {"service": SERVICE, "client": "eli", "expires": 10_000}, "auth": {"client": "mallory", "ts": 9_000}, "server_time": 9_005},
]

def check(request):
    raise NotImplementedError
@@expected
request 1 from aiko: ACCEPT (valid)
request 2 from aiko: REJECT (replayed authenticator)
request 3 from ben: REJECT (clock skew 400 s exceeds 300 s)
request 4 from chen: REJECT (ticket expired: get a new one from the TGS)
request 5 from dana: REJECT (ticket is for CIFS/files.example.jp)
request 6 from mallory: REJECT (authenticator client doesn't match the ticket)
@@requires
def check
SKEW
seen
abs(
@@hint
Return `(accepted, reason)`. Check, in order: service name, expiry against `server_time`, `abs(server_time - ts) > SKEW`, client mismatch, and replay: the pair `(client, ts)` already in `seen`. Add the pair to `seen` only when accepting.
@@solution
SKEW = 300  # seconds
SERVICE = "HTTP/intranet.example.jp"
seen = set()
requests = [
    {"ticket": {"service": SERVICE, "client": "aiko", "expires": 10_000}, "auth": {"client": "aiko", "ts": 5_000}, "server_time": 5_030},
    {"ticket": {"service": SERVICE, "client": "aiko", "expires": 10_000}, "auth": {"client": "aiko", "ts": 5_000}, "server_time": 5_040},
    {"ticket": {"service": SERVICE, "client": "ben", "expires": 10_000}, "auth": {"client": "ben", "ts": 6_000}, "server_time": 6_400},
    {"ticket": {"service": SERVICE, "client": "chen", "expires": 7_000}, "auth": {"client": "chen", "ts": 7_100}, "server_time": 7_120},
    {"ticket": {"service": "CIFS/files.example.jp", "client": "dana", "expires": 10_000}, "auth": {"client": "dana", "ts": 8_000}, "server_time": 8_010},
    {"ticket": {"service": SERVICE, "client": "eli", "expires": 10_000}, "auth": {"client": "mallory", "ts": 9_000}, "server_time": 9_005},
]

def check(request):
    t, a, now = request["ticket"], request["auth"], request["server_time"]
    if t["service"] != SERVICE:
        return False, f"ticket is for {t['service']}"
    if now > t["expires"]:
        return False, "ticket expired: get a new one from the TGS"
    if abs(now - a["ts"]) > SKEW:
        return False, f"clock skew {abs(now - a['ts'])} s exceeds {SKEW} s"
    if a["client"] != t["client"]:
        return False, "authenticator client doesn't match the ticket"
    if (a["client"], a["ts"]) in seen:
        return False, "replayed authenticator"
    seen.add((a["client"], a["ts"]))
    return True, "valid"

for i, r in enumerate(requests, 1):
    ok, why = check(r)
    print(f"request {i} from {r['auth']['client']}: {'ACCEPT' if ok else 'REJECT'} ({why})")

@@exercise Find separation-of-duties conflicts
Users get permissions from roles. Expand each user's effective permissions, then report conflicts with the separation-of-duties rules (pairs of permissions one person must not hold together), privileged permissions without a recorded justification, and leavers who still have any access.
@@lang python
@@starter
ROLES = {
    "ap-clerk": {"create_vendor", "enter_invoice"},
    "ap-approver": {"approve_payment"},
    "treasury": {"release_payment"},
    "it-admin": {"manage_users", "read_logs"},
    "auditor": {"read_logs", "read_finance"},
}
SOD = [("create_vendor", "approve_payment"), ("approve_payment", "release_payment"), ("manage_users", "read_finance")]
PRIVILEGED = {"manage_users", "release_payment"}
users = {
    "aiko": {"roles": ["ap-clerk"], "status": "active", "justified": set()},
    "ben": {"roles": ["ap-clerk", "ap-approver"], "status": "active", "justified": set()},
    "chen": {"roles": ["ap-approver", "treasury"], "status": "active", "justified": {"release_payment"}},
    "dana": {"roles": ["it-admin", "auditor"], "status": "active", "justified": {"manage_users"}},
    "eli": {"roles": ["treasury"], "status": "leaver", "justified": set()},
    "farah": {"roles": ["it-admin"], "status": "active", "justified": set()},
}

def effective(user):
    raise NotImplementedError
@@expected
aiko (ap-clerk): OK
ben (ap-clerk, ap-approver): SoD conflict: create_vendor + approve_payment
chen (ap-approver, treasury): SoD conflict: approve_payment + release_payment
dana (it-admin, auditor): SoD conflict: manage_users + read_finance
eli (treasury): leaver still has release_payment: disable now
farah (it-admin): privileged without justification: manage_users
@@requires
def effective
SOD
PRIVILEGED
union(
@@hint
`effective` is the union of the permission sets of the user's roles: `set().union(*(ROLES[r] for r in roles))`. A conflict exists when both permissions of a SoD pair are in the set. Privileged permissions minus `justified` need a justification. Leavers should have no roles at all.
@@solution
ROLES = {
    "ap-clerk": {"create_vendor", "enter_invoice"},
    "ap-approver": {"approve_payment"},
    "treasury": {"release_payment"},
    "it-admin": {"manage_users", "read_logs"},
    "auditor": {"read_logs", "read_finance"},
}
SOD = [("create_vendor", "approve_payment"), ("approve_payment", "release_payment"), ("manage_users", "read_finance")]
PRIVILEGED = {"manage_users", "release_payment"}
users = {
    "aiko": {"roles": ["ap-clerk"], "status": "active", "justified": set()},
    "ben": {"roles": ["ap-clerk", "ap-approver"], "status": "active", "justified": set()},
    "chen": {"roles": ["ap-approver", "treasury"], "status": "active", "justified": {"release_payment"}},
    "dana": {"roles": ["it-admin", "auditor"], "status": "active", "justified": {"manage_users"}},
    "eli": {"roles": ["treasury"], "status": "leaver", "justified": set()},
    "farah": {"roles": ["it-admin"], "status": "active", "justified": set()},
}

def effective(user):
    return set().union(*(ROLES[r] for r in user["roles"]))

for name, user in users.items():
    perms = effective(user)
    findings = []
    if user["status"] == "leaver" and perms:
        findings.append(f"leaver still has {', '.join(sorted(perms))}: disable now")
    for a, b in SOD:
        if a in perms and b in perms:
            findings.append(f"SoD conflict: {a} + {b}")
    unjustified = sorted((perms & PRIVILEGED) - user["justified"])
    if unjustified and user["status"] != "leaver":
        findings.append(f"privileged without justification: {', '.join(unjustified)}")
    print(f"{name} ({', '.join(user['roles'])}): {'; '.join(findings) or 'OK'}")

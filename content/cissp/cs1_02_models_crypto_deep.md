@@lesson cs1-models-crypto-deep Security Models, Cryptography and Access Control in Depth (Domains 3 and 5)
@@text
Domain 3 (Security Architecture and Engineering, about 13%) and Domain 5 (Identity and Access Management, about 13%) contain the most "know the exact rule" questions on the CISSP: which model prevents what, how many keys a system needs, which algorithm gives which service, and which access control model fits. This lesson collects those rules.

## Formal security models

| Model | Protects | Rules |
|---|---|---|
| **Bell-LaPadula** | **confidentiality** | **no read up** (simple security property), **no write down** (*-property); strong star: read/write only at your level |
| **Biba** | **integrity** | **no read down** (simple integrity), **no write up** (* integrity); can't invoke higher subjects |
| **Clark-Wilson** | integrity (commercial) | well-formed transactions via programs (access triple: subject → program → object), separation of duties, auditing; CDIs, UDIs, TPs, IVPs |
| **Brewer-Nash** (Chinese Wall) | conflicts of interest | access to one company's data blocks access to competitors' data in the same class |
| **Graham-Denning / HRU** | rights management | how subjects create/delete objects and grant rights |
| **Take-Grant** | rights propagation | take, grant, create, remove |

Memory aid: Bell-LaPadula is about secrets (**"no read up, no write down"**); Biba inverts it (**"no read down, no write up"**: don't let dirty data in).

## Evaluation and architecture

- **Common Criteria** (ISO/IEC 15408): Protection Profile (what's needed), Security Target (what the vendor claims), **EAL1–EAL7** (assurance depth; EAL4 methodically designed, tested and reviewed).
- **TCB** (trusted computing base), **reference monitor** (mediates all access; tamperproof, always invoked, verifiable), **security kernel** implements it.
- Memory protection, process isolation, rings (0 kernel → 3 user), TPM, secure boot, HSM.
- Vulnerabilities: covert channels (storage, timing), TOCTOU race conditions, emanations (TEMPEST), side channels, aggregation and inference in databases (countered with polyinstantiation, cell suppression, noise).

## Cryptography essentials

| Need | Use |
|---|---|
| Confidentiality (bulk) | symmetric: **AES** (128/192/256), ChaCha20 |
| Key exchange | **Diffie-Hellman/ECDH** (no authentication by itself) |
| Digital signatures (integrity, authentication, **non-repudiation**) | RSA, ECDSA, EdDSA: sign with the **sender's private key**, verify with the sender's public key |
| Confidentiality to a recipient (asymmetric) | encrypt with the **recipient's public key** |
| Integrity | hash: **SHA-2/SHA-3** (MD5 and SHA-1 broken for collisions) |
| Integrity + authenticity with a shared key | **HMAC**, or AEAD modes (**AES-GCM**) |

- **Key counts**: symmetric needs **n(n − 1)/2** keys for n parties; asymmetric needs **2n** (a pair each).
- Modes: **ECB** (leaks patterns, never), CBC (needs random IV), CTR, **GCM** (authenticated).
- **Kerckhoffs's principle**: security relies on the key, not secrecy of the algorithm. Work factor; key length; **crypto agility** and **post-quantum** algorithms (ML-KEM, ML-DSA) for long-lived data.
- **PKI**: CA, RA, certificates (X.509), revocation by **CRL** or **OCSP** (stapling); key escrow and recovery; **perfect forward secrecy** with ephemeral DH keys.

## Access control models (Domain 5)

| Model | Who decides | Example |
|---|---|---|
| **DAC** discretionary | the owner | file permissions/ACLs |
| **MAC** mandatory | the system, by labels and clearances | military classification (Bell-LaPadula) |
| **RBAC** role-based | admin assigns roles | job functions |
| **ABAC** attribute-based | policies over subject, object, environment attributes | "managers in HR during business hours" |
| **Rule-based** | global rules | firewall rules |
| **Risk-based** | context and risk score | step-up MFA on unusual sign-in |

- Authentication factors: something you **know**, **have**, **are** (+ somewhere you are, something you do). MFA = different **types**.
- Biometrics: **FRR** (type 1 error: false rejection), **FAR** (type 2 error: false acceptance), **CER/EER** where they're equal (lower is better).
- Federation: **SAML** (XML assertions, enterprise SSO), **OAuth 2.0** (authorisation, delegated access), **OIDC** (authentication on top of OAuth), Kerberos (tickets, KDC, time-sensitive).
- Identity lifecycle: provisioning, **access reviews/recertification**, deprovisioning; watch **privilege creep**.

@@quiz
Under Bell-LaPadula, a Secret-cleared subject wants to write to a Confidential file. What happens?
- Allowed
* Denied: no write down
- Allowed only with MAC
- Denied: no read up
? It could leak secrets into a lower level.

@@quiz
Which model's rules are "no read down, no write up"?
- Bell-LaPadula
* Biba
- Brewer-Nash
- Take-Grant
? It protects integrity.

@@quiz
How many keys are needed for 20 people to communicate privately in pairs with symmetric cryptography?
- 20
- 40
* 190
- 400
? 20 × 19 / 2.

@@quiz
Alice wants Bob to be able to prove a message came from her. What does she use?
* Her private key to sign it
- Bob's public key to encrypt it
- Her public key to sign it
- A shared symmetric key
? Non-repudiation needs a key only she has.

@@quiz
Which model is designed to prevent conflicts of interest between competing clients?
- Clark-Wilson
* Brewer-Nash
- Biba
- Graham-Denning
? The Chinese Wall.

@@quiz
A biometric system is tuned so impostors are never accepted. Which error rate rises?
- FAR
* FRR
- CER
- EAL
? Tightening reduces false acceptance but rejects more valid users.

@@quiz
Which protocol provides authentication (identity) on top of OAuth 2.0?
- SAML
* OpenID Connect
- Kerberos
- RADIUS
? OAuth alone is for authorisation.

@@flashcards
Bell-LaPadula :: confidentiality: no read up, no write down
Biba :: integrity: no read down, no write up
Clark-Wilson :: well-formed transactions, access triple
Brewer-Nash :: Chinese Wall, conflicts of interest
reference monitor :: tamperproof, always invoked, verifiable
Common Criteria EAL :: assurance level 1–7
TOCTOU :: time-of-check/time-of-use race
polyinstantiation :: different data per clearance level
symmetric keys for n :: n(n − 1)/2
asymmetric keys for n :: 2n
sign with :: sender's private key
encrypt to recipient :: recipient's public key
HMAC :: integrity + authenticity with a shared key
ECB mode :: leaks patterns
GCM :: authenticated encryption
Kerckhoffs :: only the key is secret
OCSP :: online revocation check
perfect forward secrecy :: ephemeral keys
DAC :: owner decides
MAC :: labels and clearances
ABAC :: attribute policies
FRR :: type 1 error
FAR :: type 2 error
CER :: FAR = FRR, lower is better
SAML :: XML SSO assertions
OIDC :: authentication over OAuth

@@drill
How many keys does asymmetric cryptography need for 50 users?
= 100
? 2n.

@@drill
Which Bell-LaPadula property is "no write down"?
= star property
= *-property
= the star property
= * property
? The simple security property is "no read up".

@@exercise Enforce Bell-LaPadula and Biba
Implement access decisions for both models with ordered levels. Bell-LaPadula: read allowed if subject level ≥ object level; write allowed if subject level ≤ object level (strong star option: only equal). Biba (integrity levels): read allowed if subject ≤ object; write allowed if subject ≥ object. Print the decision for each request under each model.
@@lang python
@@starter
CONF = ["Unclassified", "Confidential", "Secret", "Top Secret"]
INTEG = ["Low", "Medium", "High"]
subjects = {"analyst": ("Secret", "Medium"), "intern": ("Unclassified", "Low"), "director": ("Top Secret", "High")}
objects = {"war-plan": ("Top Secret", "High"), "memo": ("Confidential", "Medium"), "web-scrape": ("Unclassified", "Low"), "report": ("Secret", "Medium")}
requests = [("analyst", "read", "war-plan"), ("analyst", "read", "memo"), ("analyst", "write", "memo"), ("analyst", "write", "war-plan"),
            ("intern", "write", "report"), ("director", "read", "web-scrape"), ("analyst", "write", "report")]

def blp(subject_level, action, object_level, strong=False):
    raise NotImplementedError
@@expected
analyst  read  war-plan   BLP DENY  strong* DENY  Biba allow
analyst  read  memo       BLP allow strong* DENY  Biba allow
analyst  write memo       BLP DENY  strong* DENY  Biba allow
analyst  write war-plan   BLP allow strong* DENY  Biba DENY
intern   write report     BLP allow strong* DENY  Biba DENY
director read  web-scrape BLP allow strong* DENY  Biba DENY
analyst  write report     BLP allow strong* allow Biba allow
@@requires
def blp
def biba
CONF.index(
@@hint
Turn labels into numbers with `CONF.index(...)` and `INTEG.index(...)`. Biba is the mirror image of Bell-LaPadula.
@@solution
CONF = ["Unclassified", "Confidential", "Secret", "Top Secret"]
INTEG = ["Low", "Medium", "High"]
subjects = {"analyst": ("Secret", "Medium"), "intern": ("Unclassified", "Low"), "director": ("Top Secret", "High")}
objects = {"war-plan": ("Top Secret", "High"), "memo": ("Confidential", "Medium"), "web-scrape": ("Unclassified", "Low"), "report": ("Secret", "Medium")}
requests = [("analyst", "read", "war-plan"), ("analyst", "read", "memo"), ("analyst", "write", "memo"), ("analyst", "write", "war-plan"),
            ("intern", "write", "report"), ("director", "read", "web-scrape"), ("analyst", "write", "report")]

def blp(subject_level, action, object_level, strong=False):
    s, o = CONF.index(subject_level), CONF.index(object_level)
    if strong:
        return s == o
    return s >= o if action == "read" else s <= o

def biba(subject_level, action, object_level):
    s, o = INTEG.index(subject_level), INTEG.index(object_level)
    return s <= o if action == "read" else s >= o

def word(ok):
    return "allow" if ok else "DENY"

for who, action, what in requests:
    (sc, si), (oc, oi) = subjects[who], objects[what]
    print(f"{who:<8} {action:<5} {what:<10} BLP {word(blp(sc, action, oc)):<5} strong* {word(blp(sc, action, oc, True)):<5} Biba {word(biba(si, action, oi))}")

@@exercise Pick the right cryptographic operation
For each requirement, choose the algorithm family and which key is used, following the lesson's table. Then compute key counts for symmetric and asymmetric schemes for several group sizes, and flag weak choices in a list of proposed configurations (MD5, SHA-1, DES/3DES, ECB, RSA below 2048 bits).
@@lang python
@@starter
needs = ["bulk confidentiality", "prove the sender", "confidential to a recipient", "integrity only", "integrity with shared key", "agree a session key"]
group_sizes = [2, 10, 100, 1000]
proposals = ["AES-256-GCM", "RSA-1024 signatures", "SHA-1 file hashes", "3DES-CBC", "AES-128-ECB", "SHA-256 HMAC", "MD5 passwords", "RSA-3072 key transport"]

def choose(need):
    raise NotImplementedError
@@expected
bulk confidentiality: symmetric (AES-GCM) with a shared secret key
prove the sender: digital signature with the sender's private key
confidential to a recipient: encrypt (a session key) with the recipient's public key
integrity only: hash (SHA-256 or SHA-3)
integrity with shared key: HMAC with the shared key
agree a session key: (EC)DH key exchange, authenticated by signatures
n      symmetric  asymmetric
2              1           4
10            45          20
100         4950         200
1000      499500        2000
AES-256-GCM: OK
RSA-1024 signatures: WEAK: RSA key too short
SHA-1 file hashes: WEAK: broken hash
3DES-CBC: WEAK: legacy cipher
AES-128-ECB: WEAK: ECB leaks patterns
SHA-256 HMAC: OK
MD5 passwords: WEAK: broken hash, use a slow password hash (Argon2, bcrypt, scrypt, PBKDF2)
RSA-3072 key transport: OK
@@requires
def choose
// 2
2 * n
@@hint
Return a string per need from a dict. For weak proposals, check substrings, and parse the RSA key size with a regular expression or `split("-")`.
@@solution
import re

needs = ["bulk confidentiality", "prove the sender", "confidential to a recipient", "integrity only", "integrity with shared key", "agree a session key"]
group_sizes = [2, 10, 100, 1000]
proposals = ["AES-256-GCM", "RSA-1024 signatures", "SHA-1 file hashes", "3DES-CBC", "AES-128-ECB", "SHA-256 HMAC", "MD5 passwords", "RSA-3072 key transport"]

def choose(need):
    return {
        "bulk confidentiality": "symmetric (AES-GCM) with a shared secret key",
        "prove the sender": "digital signature with the sender's private key",
        "confidential to a recipient": "encrypt (a session key) with the recipient's public key",
        "integrity only": "hash (SHA-256 or SHA-3)",
        "integrity with shared key": "HMAC with the shared key",
        "agree a session key": "(EC)DH key exchange, authenticated by signatures",
    }[need]

for need in needs:
    print(f"{need}: {choose(need)}")

print("n      symmetric  asymmetric")
for n in group_sizes:
    print(f"{n:<6} {n * (n - 1) // 2:>9}  {2 * n:>10}")

for p in proposals:
    issues = []
    if "MD5" in p or "SHA-1" in p:
        issues.append("broken hash")
    if "DES" in p:
        issues.append("legacy cipher")
    if "ECB" in p:
        issues.append("ECB leaks patterns")
    m = re.search(r"RSA-(\d+)", p)
    if m and int(m.group(1)) < 2048:
        issues.append("RSA key too short")
    if "MD5 passwords" in p:
        issues.append("use a slow password hash (Argon2, bcrypt, scrypt, PBKDF2)")
    print(f"{p}: {'OK' if not issues else 'WEAK: ' + ', '.join(issues)}")

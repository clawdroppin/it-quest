@@lesson sp1-cryptoapplied Applied Cryptography: TLS, Signatures, Passwords and Certificates
@@text
The cryptography lesson introduced symmetric, asymmetric and hashing. This one shows how they're combined in practice, which is what Security+ scenario questions test: how TLS sets up a session, how a digital signature proves who signed, how passwords should be stored, and how certificates are issued, checked and revoked.

## Putting the pieces together: TLS 1.3

1. The client says hello with supported cipher suites and a key share.
2. The server replies with its key share and its **certificate**, and signs the handshake with its **private key**.
3. The client **validates the certificate** (trusted CA chain, name matches, not expired, not revoked) and checks the signature with the server's **public key**.
4. Both sides derive the same **symmetric session keys** with **ephemeral Diffie-Hellman (ECDHE)**. Ephemeral keys give **perfect forward secrecy**: stealing the server's private key later doesn't decrypt recorded sessions.
5. Bulk data is encrypted with a fast symmetric **AEAD** cipher (**AES-GCM** or **ChaCha20-Poly1305**).

So: asymmetric crypto for **authentication and key exchange**, symmetric for **bulk encryption**, hashing for **integrity**.

## Digital signatures

1. Hash the message (SHA-256).
2. Encrypt the hash with the **sender's private key** → the signature.
3. The receiver hashes the message and checks the signature with the **sender's public key**.

Provides **integrity**, **authentication** and **non-repudiation** (only the private key holder could sign). Not confidentiality. Used for code signing, signed emails (S/MIME), documents, software updates.

To send a **confidential** message: encrypt with the **recipient's public key**; only their private key decrypts.

## Storing passwords

- Never store plaintext or reversible encryption.
- Hash with a **salt** (random per user, stored with the hash) so identical passwords give different hashes and **rainbow tables** don't work.
- Use **key stretching**: deliberately slow algorithms (**bcrypt**, **scrypt**, **Argon2**, **PBKDF2** with many iterations) so brute force is expensive.
- A **pepper** is a secret added to all passwords, stored separately from the database.

## Obfuscation techniques

| Technique | What it does | Example |
|---|---|---|
| **Tokenisation** | replace data with a random token; the mapping is in a secure vault | card number → token in a payment system |
| **Data masking** | hide part of the data | `****-****-****-4242` |
| **Steganography** | hide data inside other data | message hidden in an image's pixels |
| **Pseudonymisation / anonymisation** | remove or replace identifiers | analytics datasets |

## Key management

- **Key generation, exchange, storage, rotation, revocation, destruction**.
- **HSM** (hardware security module): tamper-resistant device that stores keys and does crypto operations (CAs, payment systems, cloud KMS).
- **TPM**: a chip on the motherboard for device keys (BitLocker, measured boot).
- **Secure enclave**: an isolated processor area (Apple Secure Enclave, Intel SGX).
- **Key escrow**: a trusted third party holds a copy of keys for recovery.
- **KMS** (key management service) in the cloud.
- Key length: longer = stronger (AES-256, RSA 3072+, or ECC P-256 which is as strong as RSA 3072 with much shorter keys).

## Certificates in depth

- **X.509** fields: subject, **SAN** (subject alternative names: all the names the cert is valid for), issuer, validity dates, public key, serial number, key usage.
- Types: **wildcard** (`*.example.com`: one level of subdomains), **SAN/multi-domain**, **self-signed** (no CA; fine for labs, not for public services), **root**, **intermediate**, **code signing**, **user/client** certificates.
- Validation levels: **DV** (domain), **OV** (organisation), **EV** (extended).
- **CSR**: certificate signing request, created with the key pair; the private key never leaves you.
- **Chain of trust**: leaf → intermediate → root (in the trust store). The server must send the intermediate.

**Revocation**:
- **CRL**: a list of revoked serial numbers the CA publishes; can be large and stale.
- **OCSP**: ask the CA's responder about one certificate in real time.
- **OCSP stapling**: the server fetches the OCSP response and "staples" it to the handshake: faster and more private.
- **Certificate pinning**: an app only accepts a specific certificate/key.

**Certificate transparency** logs make mis-issued certificates visible.

## Blockchain and open public ledgers

A blockchain is an append-only ledger where each block contains the hash of the previous one, so altering history breaks the chain. Security+ mentions it as an integrity technology.

@@quiz
In TLS, what protects a recorded session if the server's private key is stolen a year later?
* Perfect forward secrecy from ephemeral Diffie-Hellman keys
- The certificate's expiry date
- OCSP stapling
- The wildcard certificate
? Each session's keys were ephemeral and discarded.

@@quiz
Alice signs a contract digitally. Which key does she use, and which key does Bob use to verify?
* Alice's private key; Alice's public key
- Alice's public key; Bob's private key
- Bob's public key; Bob's private key
- A shared symmetric key for both
? Sign with your private key; anyone verifies with your public key.

@@quiz
Why add a unique salt to each password hash?
- To make hashing faster
* So identical passwords produce different hashes and rainbow tables don't work
- So the password can be decrypted later
- To compress the database
? Combine with slow algorithms like bcrypt or Argon2.

@@quiz
A payment system replaces card numbers with random values and keeps the mapping in a secure vault. What is this?
- Hashing
- Masking
* Tokenisation
- Steganography
? Tokens are useless if stolen.

@@quiz
Which revocation method lets the web server attach a CA-signed "still valid" response to the handshake?
- CRL
- OCSP
* OCSP stapling
- Certificate pinning
? Faster, and clients don't contact the CA.

@@flashcards
TLS uses asymmetric for :: authentication and key exchange
TLS uses symmetric for :: bulk encryption
ECDHE :: ephemeral key exchange, forward secrecy
AEAD :: AES-GCM, ChaCha20-Poly1305
digital signature :: hash signed with sender's private key
confidential message :: encrypt with recipient's public key
salt :: random per-user value in password hashes
key stretching :: bcrypt, scrypt, Argon2, PBKDF2
pepper :: secret added to all passwords
tokenisation :: replace data with a random token
masking :: hide part of data
steganography :: hide data inside other data
HSM :: tamper-resistant key storage device
key escrow :: third party holds keys for recovery
SAN :: names a certificate is valid for
wildcard cert :: *.example.com, one level
CSR :: certificate signing request
CRL / OCSP :: revocation list / real-time check
OCSP stapling :: server attaches the OCSP response

@@drill
Which key is used to verify a digital signature: the signer's public or private key?
= public
= the signer's public key
= signer's public key
? The private key creates it.

@@drill
Name the password hashing algorithm that won the Password Hashing Competition.
= Argon2
= argon2
? Also bcrypt, scrypt and PBKDF2.

@@exercise Salted, stretched password storage
Implement `hash_password(password, salt)` with `hashlib.pbkdf2_hmac("sha256", ...)` and 100,000 iterations, returning the hex digest, and `verify(password, salt, stored)` that compares with `hmac.compare_digest`. Use the fixed salts given (a real system uses `os.urandom(16)`). Show that two users with the same password get different hashes, then check the login attempts.
@@lang python
@@starter
import hashlib
import hmac

users = {
    "ana": {"salt": bytes.fromhex("a1b2c3d4e5f60718293a4b5c6d7e8f90")},
    "ken": {"salt": bytes.fromhex("0f1e2d3c4b5a69788796a5b4c3d2e1f0")},
}
passwords = {"ana": "correct horse battery staple", "ken": "correct horse battery staple"}
attempts = [("ana", "correct horse battery staple"), ("ken", "Correct horse battery staple"), ("ken", "correct horse battery staple")]

def hash_password(password, salt):
    raise NotImplementedError
@@expected
ana: d9c12f0a13e60410...
ken: f54e46978b9881b9...
Same hash for same password? False
login ana: success
login ken: failed
login ken: success
@@requires
pbkdf2_hmac(
hmac.compare_digest(
100_000
@@hint
`hashlib.pbkdf2_hmac("sha256", password.encode(), salt, 100_000).hex()`.
@@solution
import hashlib
import hmac

users = {
    "ana": {"salt": bytes.fromhex("a1b2c3d4e5f60718293a4b5c6d7e8f90")},
    "ken": {"salt": bytes.fromhex("0f1e2d3c4b5a69788796a5b4c3d2e1f0")},
}
passwords = {"ana": "correct horse battery staple", "ken": "correct horse battery staple"}
attempts = [("ana", "correct horse battery staple"), ("ken", "Correct horse battery staple"), ("ken", "correct horse battery staple")]

def hash_password(password, salt):
    return hashlib.pbkdf2_hmac("sha256", password.encode(), salt, 100_000).hex()

def verify(password, salt, stored):
    return hmac.compare_digest(hash_password(password, salt), stored)

for name, pw in passwords.items():
    users[name]["hash"] = hash_password(pw, users[name]["salt"])
    print(f"{name}: {users[name]['hash'][:16]}...")
print("Same hash for same password?", users["ana"]["hash"] == users["ken"]["hash"])

for name, pw in attempts:
    ok = verify(pw, users[name]["salt"], users[name]["hash"])
    print(f"login {name}: {'success' if ok else 'failed'}")

@@exercise Validate a certificate
Check each certificate (given as a dict, as a TLS client would see it) against the hostname being visited and the date `TODAY`. Fail it if: it's expired or not yet valid; the hostname doesn't match any SAN (a `*.` wildcard matches exactly one extra label); the issuer isn't in `TRUSTED`; or its serial is in the `REVOKED` list. Print `valid` or every reason.
@@lang python
@@starter
from datetime import date

TODAY = date(2026, 11, 4)
TRUSTED = {"Example Root CA", "Example Issuing CA 2"}
REVOKED = {"0x4f21"}
certs = [
    ("shop.example.com", {"serial": "0x1a01", "issuer": "Example Issuing CA 2", "san": ["shop.example.com", "www.example.com"], "from": date(2026, 6, 1), "to": date(2027, 6, 1)}),
    ("api.example.com", {"serial": "0x1a02", "issuer": "Example Issuing CA 2", "san": ["*.example.com"], "from": date(2026, 1, 1), "to": date(2026, 10, 31)}),
    ("a.b.example.com", {"serial": "0x1a03", "issuer": "Example Issuing CA 2", "san": ["*.example.com"], "from": date(2026, 1, 1), "to": date(2027, 1, 1)}),
    ("intranet.local", {"serial": "0x0001", "issuer": "intranet.local", "san": ["intranet.local"], "from": date(2025, 1, 1), "to": date(2035, 1, 1)}),
    ("mail.example.com", {"serial": "0x4f21", "issuer": "Example Issuing CA 2", "san": ["mail.example.com"], "from": date(2026, 3, 1), "to": date(2027, 3, 1)}),
]

def name_matches(host, pattern):
    raise NotImplementedError
@@expected
shop.example.com: valid
api.example.com: expired 2026-10-31
a.b.example.com: name mismatch
intranet.local: untrusted issuer (self-signed?)
mail.example.com: revoked
@@requires
def name_matches
startswith("*.")
REVOKED
@@hint
For a wildcard, split both on `.`: the host must have the same number of labels as the pattern and match every label after the first.
@@solution
from datetime import date

TODAY = date(2026, 11, 4)
TRUSTED = {"Example Root CA", "Example Issuing CA 2"}
REVOKED = {"0x4f21"}
certs = [
    ("shop.example.com", {"serial": "0x1a01", "issuer": "Example Issuing CA 2", "san": ["shop.example.com", "www.example.com"], "from": date(2026, 6, 1), "to": date(2027, 6, 1)}),
    ("api.example.com", {"serial": "0x1a02", "issuer": "Example Issuing CA 2", "san": ["*.example.com"], "from": date(2026, 1, 1), "to": date(2026, 10, 31)}),
    ("a.b.example.com", {"serial": "0x1a03", "issuer": "Example Issuing CA 2", "san": ["*.example.com"], "from": date(2026, 1, 1), "to": date(2027, 1, 1)}),
    ("intranet.local", {"serial": "0x0001", "issuer": "intranet.local", "san": ["intranet.local"], "from": date(2025, 1, 1), "to": date(2035, 1, 1)}),
    ("mail.example.com", {"serial": "0x4f21", "issuer": "Example Issuing CA 2", "san": ["mail.example.com"], "from": date(2026, 3, 1), "to": date(2027, 3, 1)}),
]

def name_matches(host, pattern):
    if pattern.startswith("*."):
        h, p = host.split("."), pattern.split(".")
        return len(h) == len(p) and h[1:] == p[1:]
    return host == pattern

for host, cert in certs:
    reasons = []
    if TODAY > cert["to"]:
        reasons.append(f"expired {cert['to']}")
    if TODAY < cert["from"]:
        reasons.append("not yet valid")
    if not any(name_matches(host, san) for san in cert["san"]):
        reasons.append("name mismatch")
    if cert["issuer"] not in TRUSTED:
        reasons.append("untrusted issuer (self-signed?)")
    if cert["serial"] in REVOKED:
        reasons.append("revoked")
    print(f"{host}: " + ("; ".join(reasons) if reasons else "valid"))

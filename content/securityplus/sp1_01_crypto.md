@@lesson sp1-crypto Cryptography and PKI
@@text
## Symmetric and asymmetric encryption

| | Symmetric | Asymmetric |
|---|---|---|
| Keys | one shared secret key | a **public** and a **private** key |
| Speed | fast: bulk data | slow: small amounts |
| Examples | **AES** (128/256), ChaCha20; old: DES, 3DES, RC4 | **RSA**, **ECC** (elliptic curve), Diffie-Hellman |
| Problem | how do you share the key safely? | solves key exchange and signatures |

Real systems combine them: **TLS** uses asymmetric crypto (and **Diffie-Hellman** / ECDHE) to agree on a session key, then AES for the data. **Perfect forward secrecy** (ephemeral DH) means a stolen server key can't decrypt recorded old sessions.

- Encrypt *to* someone with **their public key**; only their private key decrypts.
- **Sign** with **your private key**; anyone verifies with your public key.

## Hashing

A **hash** (SHA-256, SHA-3; broken: MD5, SHA-1) turns any data into a fixed-size fingerprint. It's one-way, and a tiny change gives a completely different hash. Uses: file integrity, digital signatures, password storage.

**Passwords** are never stored encrypted or as a plain hash. They're stored with a **salt** (random data per user, defeats **rainbow tables**) and a slow **key derivation function** like **bcrypt**, **scrypt**, **Argon2** or **PBKDF2** (**key stretching**). **HMAC** = a hash with a secret key: proves integrity *and* origin.

## PKI and certificates

A **certificate** binds a public key to an identity, signed by a **certificate authority** (CA). Browsers trust a set of **root CAs**; roots sign **intermediate CAs**, which sign server certificates: the **chain of trust**.

- A **CSR** (certificate signing request) is what you send a CA; it includes your public key.
- **Wildcard** certificate: `*.example.com`. **SAN** (subject alternative name) lists several names in one certificate.
- **Revocation**: a **CRL** (list of revoked certificates) or **OCSP** (ask online); **OCSP stapling** lets the server include the answer.
- **Self-signed** certificates aren't trusted by others: fine for labs and internal tests.
- **Key escrow** keeps a copy of keys with a trusted party. Keys belong in an **HSM** (hardware security module), a **TPM** or a cloud **KMS**; **secure enclaves** isolate secrets in the CPU.

## Other crypto terms

- **Data states**: at rest (disk encryption: BitLocker, LUKS; database **TDE**), in transit (TLS, IPsec, SSH), in use (enclaves).
- **Obfuscation**: **steganography** (hiding data in images), **tokenization** (replacing card numbers with tokens), **data masking** (`****-****-****-1234`).
- **Blockchain**: a distributed, append-only ledger linked by hashes.
- **Key length** matters: AES-256, RSA 2048+ (3072 recommended), ECC 256 is about as strong as RSA 3072.

@@quiz
Ana wants to send Ben a file that only Ben can read. Which key encrypts it?
* Ben's public key
- Ben's private key
- Ana's private key
- Ana's public key
? Only Ben's private key can decrypt what his public key encrypted.

@@quiz
How is a digital signature created?
* By hashing the message and encrypting the hash with the sender's private key
- By encrypting the message with the receiver's public key
- By adding a salt to the message
- With a symmetric key shared in advance
? Anyone can verify it with the sender's public key.

@@quiz
Why are passwords stored with a unique salt per user?
* So identical passwords give different hashes and precomputed rainbow tables don't work
- To make passwords shorter
- To encrypt them so admins can read them
- To speed up logins
? Salts make every hash unique; slow KDFs like bcrypt make guessing expensive.

@@quiz
Which hashing algorithm should NOT be used anymore for integrity or signatures?
* MD5
- SHA-256
- SHA-3
- SHA-512
? MD5 (and SHA-1) have practical collision attacks.

@@quiz
A browser needs to check whether a certificate has been revoked, quickly and without downloading a large list. What does it use?
* OCSP (ideally with OCSP stapling)
- A CSR
- A CRL only
- Key escrow
? OCSP answers about one certificate.

@@quiz
Which certificate secures www.example.com, mail.example.com and shop.example.com, but not example.org?
* A wildcard certificate for *.example.com
- A self-signed root certificate
- A certificate for example.org with OCSP
- A code-signing certificate
? (A SAN certificate listing all three names would also work.)

@@quiz
Credit card numbers in a database are replaced with random values, and the real numbers are kept in a separate secure vault. What is this?
* Tokenization
- Hashing
- Steganography
- Masking
? Tokens have no mathematical relationship to the original value.

@@quiz
Which property means past TLS sessions stay secret even if the server's private key is stolen later?
* Perfect forward secrecy
- Non-repudiation
- Key escrow
- Certificate pinning
? Each session uses a temporary key exchange (ephemeral Diffie-Hellman).

@@flashcards
symmetric encryption :: One shared key; fast (AES)
asymmetric encryption :: Public/private key pair (RSA, ECC)
encrypt for someone :: Use their public key
sign :: Use your own private key
hash :: One-way fixed-size fingerprint (SHA-256)
salt :: Random value per password; defeats rainbow tables
key stretching :: Slow KDFs: bcrypt, scrypt, Argon2, PBKDF2
HMAC :: Keyed hash: integrity and authenticity
CA :: Certificate authority; signs certificates
chain of trust :: Root CA → intermediate CA → server certificate
CSR :: Certificate signing request
CRL / OCSP :: Revocation list / online status check
wildcard certificate :: *.example.com
SAN :: Subject alternative names: several names in one certificate
HSM :: Hardware device that stores and uses keys securely
perfect forward secrecy :: Ephemeral key exchange; old sessions stay safe
tokenization :: Replace sensitive data with a meaningless token
steganography :: Hiding data inside other data

@@drill
Which symmetric algorithm is the current standard for encrypting data (three letters)?
= AES
? Advanced Encryption Standard, 128 or 256-bit keys.

@@exercise Integrity check
A file was downloaded, and the vendor publishes its SHA-256. Write `sha256_hex(data)` with `hashlib` and print whether each download matches the published hash. Notice how one changed character changes the whole hash.
@@lang python
@@starter
import hashlib

def sha256_hex(data):
    # data is bytes; return the hex digest
    raise NotImplementedError

published = sha256_hex(b"installer v1.0")
for name, data in [("good", b"installer v1.0"), ("tampered", b"installer v1.1")]:
    print(name, "OK" if sha256_hex(data) == published else "MISMATCH", sha256_hex(data)[:16])
@@expected
good OK 53a6b9a8b477a232
tampered MISMATCH 5fd04211eddc4c8a
@@hint
`hashlib.sha256(data).hexdigest()`
@@solution
import hashlib

def sha256_hex(data):
    return hashlib.sha256(data).hexdigest()

published = sha256_hex(b"installer v1.0")
for name, data in [("good", b"installer v1.0"), ("tampered", b"installer v1.1")]:
    print(name, "OK" if sha256_hex(data) == published else "MISMATCH", sha256_hex(data)[:16])

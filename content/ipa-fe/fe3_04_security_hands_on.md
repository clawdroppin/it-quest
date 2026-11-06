@@lesson fe3-security-hands-on Security for 科目B: Hashing and Salts, Signatures, Access Control and Log Analysis
@@concepts maps, functions, text-processing
@@lang python
@@text
Since 2023, four of the twenty 科目B questions are about **information security (情報セキュリティ)**. They aren't about memorising terms. They give you a scenario (a company's password storage, a log extract, an access matrix, a mail flow) and ask which measure fixes the problem, which key is used where, or what a procedure does. This lesson works through the patterns with small programs, using Python's standard `hashlib` and `hmac` modules.

## Storing passwords

- Never store passwords in plain text, and never with reversible encryption. Store a **hash (ハッシュ値)**: a one-way function such as SHA-256 maps the password to a fixed-length value.
- A plain hash isn't enough: identical passwords give identical hashes, and attackers use precomputed tables (**rainbow tables**). Add a random **salt (ソルト)** per user and store it beside the hash, so the same password gives different stored values.
- Make guessing slow with **key stretching (ストレッチング)**: repeat the hash many thousands of times (PBKDF2, bcrypt, scrypt, Argon2). One login check takes milliseconds, while a billion guesses take years.
- Exam phrases: ソルトの付加 defeats レインボーテーブル攻撃. ストレッチング slows down 総当たり攻撃 (brute force). Neither helps if users choose `password123`, which is why there are rate limits and account lockout (アカウントロック) against online guessing.

## Keys: which one does what?

- **Common-key (symmetric) encryption (共通鍵暗号方式)**, e.g. AES: the same key encrypts and decrypts. It's fast, but the key must be shared secretly, and n people need n(n−1)/2 keys.
- **Public-key (asymmetric) encryption (公開鍵暗号方式)**, e.g. RSA, ECC: each party has a key pair.
  - **Confidentiality**: encrypt with the **recipient's public key**, and only the recipient's **private key (秘密鍵)** decrypts.
  - **Digital signature (デジタル署名)**: the sender **signs a hash of the message with their own private key**, and anyone verifies it with the **sender's public key**. It proves who sent it (authenticity) and that it wasn't changed (integrity), and gives non-repudiation (否認防止).
- **Hybrid encryption** (TLS, S/MIME): a random common key encrypts the data, and public-key encryption protects that common key.
- **MAC / HMAC (メッセージ認証符号)**: a hash computed with a shared secret key. It gives integrity and authenticity between the parties who share the key, but no non-repudiation, since both could have produced it.
- **Certificates (デジタル証明書)** from a **certificate authority (認証局, CA)** bind a public key to its owner, so you know the public key really belongs to them.

## Access control

- **Least privilege (最小権限の原則)**: each account gets only the rights its job needs.
- **Need-to-know**: access only to the information the job requires.
- **Separation of duties (職務分掌)**: the person who requests a payment shouldn't approve it.
- **RBAC (role-based access control)**: permissions attach to roles, and users get roles. When someone changes department, change their role, and remove their old access promptly. Leftover rights are a common exam answer.
- Review accounts regularly. Disable accounts of people who have left (退職者アカウント) and shared or unused accounts.

## Reading logs

- 科目B often shows an access log and asks what's happening:
  - **Brute force (総当たり)**: many failed logins for one account from one address.
  - **Password spraying / list attacks (パスワードリスト攻撃)**: many accounts, few attempts each, often from many addresses, using leaked password lists.
  - **Unusual times or places**: a successful login at 3 a.m. from abroad after failures.
- Countermeasures:
  - account lockout and rate limiting against brute force.
  - **multi-factor authentication (多要素認証)** against list attacks, because a reused password alone isn't enough.
  - alerts in the **SIEM** for anomalies.
- Keep time synchronised (NTP) on all systems, or the logs can't be correlated.

@@quiz
Why add a salt to each password before hashing?
* So identical passwords give different hashes, which defeats rainbow tables
- To make the hash reversible
- To encrypt the password
- To make hashing faster
? Store the salt beside the hash.

@@quiz
Which key does the sender use to create a digital signature?
* The sender's private key
- The sender's public key
- The recipient's public key
- A shared common key
? Verified with the sender's public key.

@@quiz
To send a confidential message with public-key encryption, which key encrypts it?
* The recipient's public key
- The sender's private key
- The recipient's private key
- The sender's public key
? Only the recipient's private key decrypts it.

@@quiz
Why doesn't an HMAC provide non-repudiation?
* Both parties share the key, so either could have created the MAC
- It isn't a hash
- It uses public keys
- It can be reversed
? Signatures use a private key only the signer has.

@@quiz
A log shows 200 accounts each failing once from 50 addresses, then a few successes. Which attack is this and what is the best countermeasure?
* A password list (spraying) attack; multi-factor authentication
- Brute force on one account; longer lockout only
- SQL injection; input validation
- DoS; more bandwidth
? Lockout per account barely triggers with one attempt each.

@@quiz
An employee moves from accounting to sales. What does least privilege require?
* Remove the accounting rights when giving the sales role
- Keep both sets of rights
- Delete the account
- Give administrator rights temporarily
? Leftover rights accumulate (privilege creep).

@@flashcards
ハッシュ値 :: one-way fixed-length digest
ソルト :: random value per user added before hashing
ストレッチング :: repeat hashing to slow guessing
レインボーテーブル :: precomputed hash tables, defeated by salts
共通鍵暗号 :: same key both ways (AES)
公開鍵暗号 :: key pair (RSA, ECC)
confidentiality :: encrypt with the recipient's public key
デジタル署名 :: sign with the sender's private key
否認防止 :: non-repudiation
HMAC :: keyed hash, shared key, no non-repudiation
認証局 :: certificate authority, binds keys to owners
最小権限 :: least privilege
職務分掌 :: separation of duties
パスワードリスト攻撃 :: leaked passwords tried on many accounts
多要素認証 :: multi-factor authentication

@@drill
How many common keys are needed for 5 people to communicate privately in pairs?
= 10
? 5 × 4 / 2.

@@drill
Which key verifies a digital signature: the sender's public or private key?
= public
= the sender's public key
= sender's public key
? Anyone can verify.

@@exercise Salted, stretched password storage
Store passwords with a per-user random salt and PBKDF2-HMAC-SHA256 (use `hashlib.pbkdf2_hmac`), then verify logins with a constant-time comparison (`hmac.compare_digest`). To make the output repeatable, the salts come from a fixed list here. Show that two users with the same password get different stored hashes, while unsalted SHA-256 gives identical ones.
@@starter
import hashlib
import hmac

ITERATIONS = 100_000
SALTS = iter([bytes.fromhex("a1b2c3d4e5f60718"), bytes.fromhex("0f1e2d3c4b5a6978"), bytes.fromhex("1122334455667788")])

def store(password):
    raise NotImplementedError

def verify(password, record):
    raise NotImplementedError

users = {"aiko": "hunter2", "ben": "hunter2", "chen": "Correct-Horse-Battery"}
db = {name: store(pw) for name, pw in users.items()}
for name, (salt, digest) in db.items():
    plain = hashlib.sha256(users[name].encode()).hexdigest()[:16]
    print(f"{name:<5} salt {salt.hex()}  pbkdf2 {digest.hex()[:16]}...  plain sha256 {plain}...")
for name, attempt in [("aiko", "hunter2"), ("ben", "Hunter2"), ("chen", "Correct-Horse-Battery")]:
    print(f"login {name} with {attempt!r}: {'ok' if verify(attempt, db[name]) else 'rejected'}")
@@expected
aiko  salt a1b2c3d4e5f60718  pbkdf2 96beeef67610b515...  plain sha256 f52fbd32b2b3b86f...
ben   salt 0f1e2d3c4b5a6978  pbkdf2 bf6a42536f0c477d...  plain sha256 f52fbd32b2b3b86f...
chen  salt 1122334455667788  pbkdf2 c8dbac157cbd6df8...  plain sha256 47ef3ea9d6d824d9...
login aiko with 'hunter2': ok
login ben with 'Hunter2': rejected
login chen with 'Correct-Horse-Battery': ok
@@requires
def store
def verify
compare_digest(
@@hint
`store`: `salt = next(SALTS)` and `digest = hashlib.pbkdf2_hmac("sha256", password.encode(), salt, ITERATIONS)`; return `(salt, digest)`. `verify`: recompute with the stored salt and return `hmac.compare_digest(new, digest)`.
@@solution
import hashlib
import hmac

ITERATIONS = 100_000
SALTS = iter([bytes.fromhex("a1b2c3d4e5f60718"), bytes.fromhex("0f1e2d3c4b5a6978"), bytes.fromhex("1122334455667788")])

def store(password):
    salt = next(SALTS)
    return salt, hashlib.pbkdf2_hmac("sha256", password.encode(), salt, ITERATIONS)

def verify(password, record):
    salt, digest = record
    new = hashlib.pbkdf2_hmac("sha256", password.encode(), salt, ITERATIONS)
    return hmac.compare_digest(new, digest)

users = {"aiko": "hunter2", "ben": "hunter2", "chen": "Correct-Horse-Battery"}
db = {name: store(pw) for name, pw in users.items()}
for name, (salt, digest) in db.items():
    plain = hashlib.sha256(users[name].encode()).hexdigest()[:16]
    print(f"{name:<5} salt {salt.hex()}  pbkdf2 {digest.hex()[:16]}...  plain sha256 {plain}...")
for name, attempt in [("aiko", "hunter2"), ("ben", "Hunter2"), ("chen", "Correct-Horse-Battery")]:
    print(f"login {name} with {attempt!r}: {'ok' if verify(attempt, db[name]) else 'rejected'}")

@@exercise Classify login attacks from a log
Each log line is `time address account result`. Count failures per account and per address, then classify: an account with 5 or more failures is under **brute force** (recommend lockout and rate limiting); an address that failed against 3 or more different accounts is running a **password list attack** (recommend MFA and blocking the address); a success for an account that had failures from the same address is a **possible compromise** (reset the password and check the session).
@@starter
log = """02:10:01 203.0.113.9 aiko FAIL
02:10:03 203.0.113.9 aiko FAIL
02:10:05 203.0.113.9 aiko FAIL
02:10:07 203.0.113.9 aiko FAIL
02:10:09 203.0.113.9 aiko FAIL
02:11:00 198.51.100.4 ben FAIL
02:11:02 198.51.100.4 chen FAIL
02:11:04 198.51.100.4 dana FAIL
02:11:06 198.51.100.4 eiji FAIL
02:11:08 198.51.100.4 fumi OK
02:12:30 198.51.100.4 chen OK
09:00:12 192.0.2.10 ben OK"""

def analyse(lines):
    raise NotImplementedError

for finding in analyse(log.splitlines()):
    print(finding)
@@expected
brute force on aiko (5 failures): lock out and rate-limit
password list attack from 198.51.100.4 (4 accounts): require MFA, block the address
possible compromise: chen logged in from 198.51.100.4 at 02:12:30 after failures: reset the password, check the session
@@requires
def analyse
>= 5
@@hint
Split each line into `time, addr, account, result`. Keep `fails_by_account[account]`, `accounts_by_addr[addr]` (a set of accounts with failures) and `failed_pairs` (a set of `(addr, account)`). Output brute-force findings first, then list attacks, then any OK line whose `(addr, account)` is in `failed_pairs`.
@@solution
log = """02:10:01 203.0.113.9 aiko FAIL
02:10:03 203.0.113.9 aiko FAIL
02:10:05 203.0.113.9 aiko FAIL
02:10:07 203.0.113.9 aiko FAIL
02:10:09 203.0.113.9 aiko FAIL
02:11:00 198.51.100.4 ben FAIL
02:11:02 198.51.100.4 chen FAIL
02:11:04 198.51.100.4 dana FAIL
02:11:06 198.51.100.4 eiji FAIL
02:11:08 198.51.100.4 fumi OK
02:12:30 198.51.100.4 chen OK
09:00:12 192.0.2.10 ben OK"""

def analyse(lines):
    fails_by_account, accounts_by_addr, failed_pairs, successes = {}, {}, set(), []
    for line in lines:
        time, addr, account, result = line.split()
        if result == "FAIL":
            fails_by_account[account] = fails_by_account.get(account, 0) + 1
            accounts_by_addr.setdefault(addr, set()).add(account)
            failed_pairs.add((addr, account))
        else:
            successes.append((time, addr, account))
    findings = []
    for account, n in fails_by_account.items():
        if n >= 5:
            findings.append(f"brute force on {account} ({n} failures): lock out and rate-limit")
    for addr, accounts in accounts_by_addr.items():
        if len(accounts) >= 3:
            findings.append(f"password list attack from {addr} ({len(accounts)} accounts): require MFA, block the address")
    for time, addr, account in successes:
        if (addr, account) in failed_pairs:
            findings.append(f"possible compromise: {account} logged in from {addr} at {time} after failures: reset the password, check the session")
    return findings

for finding in analyse(log.splitlines()):
    print(finding)

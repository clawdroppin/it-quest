@@lesson cl3-security Cloud Security and Compliance: Identity, Encryption, Keys and Posture
@@text
Security is the largest Cloud+ domain, and the same ideas apply on every provider: identity is the perimeter, encrypt everything, manage keys deliberately, log and monitor, and continuously check your posture against a baseline. This lesson covers those controls in provider-neutral terms, with the AWS/Azure/Google names side by side.

## Identity is the new perimeter

- **Least privilege**: grant only the actions and resources needed; review regularly; remove unused permissions (access analyzers/recommenders).
- **No long-lived keys**: humans use SSO with MFA; workloads use roles/managed identities/service accounts with short-lived tokens; CI uses **OIDC federation**.
- **Separate accounts/subscriptions/projects** per environment and workload; guardrails at the organisation level (SCPs, Azure Policy, Org Policy).
- **Break-glass** accounts: few, monitored, MFA with hardware keys, tested.
- **Privileged access**: just-in-time elevation (PIM), session recording, approvals.

## Encryption

| Where | How |
|---|---|
| **In transit** | TLS 1.2+ everywhere, HSTS, mTLS between services, private connectivity |
| **At rest** | provider-managed keys by default; customer-managed keys (CMK) for control and audit |
| **In use** | confidential computing (enclaves, confidential VMs) |

**Envelope encryption**: data is encrypted with a **data encryption key (DEK)**; the DEK is encrypted with a **key encryption key (KEK)** held in the KMS. Rotating the KEK only re-wraps DEKs; the data doesn't need re-encrypting.

## Key management

| Option | Who controls the key |
|---|---|
| Provider-managed | provider (transparent, no effort) |
| Customer-managed (CMK in KMS) | you: rotation, policies, disable/delete, audit |
| Customer-supplied / BYOK / HYOK / external key manager | you, keys imported or kept outside the provider |
| Dedicated HSM (CloudHSM, Azure Managed HSM, Cloud HSM) | you, FIPS 140-3 Level 3 hardware |

Names: **AWS KMS**, **Azure Key Vault**, **Google Cloud KMS**. Enable automatic rotation (e.g. yearly), restrict key administrators separately from key users, and remember that **deleting a key destroys all data encrypted with it** (hence scheduled deletion windows).

**Secrets** (passwords, API keys) belong in a secrets manager (AWS Secrets Manager, Azure Key Vault, Google Secret Manager) with rotation, never in code, images or environment files committed to git.

## Detect and respond

- Audit logs (CloudTrail, Azure Activity/Entra logs, Cloud Audit Logs) centralised in a separate, locked-down account with immutable storage.
- Threat detection (GuardDuty, Defender for Cloud, Security Command Center), SIEM integration.
- **CSPM** (cloud security posture management) continuously compares configuration with benchmarks (**CIS Foundations**, provider best practices) and reports drift; **CWPP** protects workloads; **CNAPP** combines them.

## Compliance

- Frameworks: ISO 27001, SOC 2, PCI DSS, HIPAA, GDPR (data residency, processor agreements), FedRAMP.
- Provider compliance reports (AWS Artifact, Microsoft Service Trust Portal, Google Compliance Reports Manager) cover **their** part of shared responsibility only.
- Data classification drives controls: public, internal, confidential, restricted.

## Common misconfigurations

Public storage buckets, security groups open to `0.0.0.0/0` on SSH/RDP/databases, unencrypted volumes and snapshots, shared snapshots made public, root/global admin used daily, long-lived access keys, logging disabled, default VPCs in use, IMDSv1 enabled on AWS instances.

@@quiz
With envelope encryption, what happens when the key encryption key is rotated?
- All data is re-encrypted
* Only the data keys are re-wrapped; the data stays as it is
- Old data becomes unreadable
- Nothing; KEKs can't rotate
? That's what makes rotation cheap.

@@quiz
A CI pipeline needs to deploy to the cloud. What's the recommended credential?
- An access key stored as a pipeline secret
* OIDC federation to a role with short-lived tokens
- The root account
- A shared admin password
? No long-lived keys.

@@quiz
A customer-managed KMS key is deleted. What happens to data encrypted with it?
- It's decrypted automatically
* It becomes unrecoverable
- The provider keeps a copy
- Nothing
? That's why deletion is scheduled with a waiting period.

@@quiz
Which tool category continuously checks cloud configuration against benchmarks such as CIS?
- SIEM
* CSPM
- WAF
- DLP
? Posture management.

@@quiz
Which requirement is best met by a dedicated HSM?
* Keys must be held in single-tenant FIPS 140-3 Level 3 hardware under customer control
- Encrypting data in transit
- Rotating passwords
- Blocking SQL injection
? CloudHSM, Managed HSM, Cloud HSM.

@@quiz
A provider's SOC 2 report covers what?
- Your application's controls
* The provider's controls for its part of the shared responsibility model
- Everything in your account
- Only physical security
? Your configuration is your audit.

@@flashcards
identity :: the new perimeter
least privilege :: only what's needed
workload credentials :: roles, managed identities, short-lived
CI credentials :: OIDC federation
break-glass account :: emergency, monitored, few
in transit :: TLS 1.2+, mTLS
at rest :: provider or customer-managed keys
in use :: confidential computing
envelope encryption :: DEK wrapped by KEK
CMK :: customer controls rotation and policy
key deletion :: data unrecoverable
HSM :: FIPS 140-3 Level 3 hardware
secrets manager :: rotated secrets, not in code
audit logs :: separate locked account
CSPM :: posture vs benchmarks
CWPP :: workload protection
CNAPP :: CSPM + CWPP and more
CIS Foundations :: cloud benchmark
compliance reports :: provider's part only
data classification :: drives controls

@@drill
What does the KEK encrypt in envelope encryption?
= the data encryption key
= the DEK
= data keys
= DEK
? The DEK encrypts the data.

@@drill
Which acronym names tools that continuously assess cloud configuration posture?
= CSPM
? Cloud Security Posture Management.

@@exercise Envelope encryption with key rotation
Model envelope encryption: each object has a data key (DEK) wrapped by a version of the key encryption key (KEK). Use XOR as a toy cipher (never in real life!). Encrypt objects with KEK version 1, rotate to version 2, re-wrap only the DEKs (count bytes touched), then decrypt every object to prove the data is unchanged. Disabling a KEK version makes objects still wrapped with it unreadable.
@@lang python
@@starter
import hashlib

objects = {"invoice-001.pdf": b"total=1200 EUR", "payroll.csv": b"alice,4200\nbob,3900", "notes.txt": b"meeting at 10"}

def keystream(key, n):
    raise NotImplementedError
@@expected
encrypted 3 objects with KEK v1, data bytes: 46
rotated to KEK v2: re-wrapped 64 bytes of data keys, 0 bytes of data
KEK v1 disabled
invoice-001.pdf: KEK v2, intact=True
payroll.csv: KEK v2, intact=True
notes.txt: unreadable, wrapped with disabled KEK v1
@@requires
def keystream
def wrap
hashlib.sha256(
@@hint
`keystream(key, n)` repeats `hashlib.sha256(key + counter.to_bytes(4, "big")).digest()` blocks until n bytes. XOR is its own inverse, so `wrap` and `unwrap` are the same operation. Derive DEKs and KEKs deterministically from names so the output is stable.
@@solution
import hashlib

objects = {"invoice-001.pdf": b"total=1200 EUR", "payroll.csv": b"alice,4200\nbob,3900", "notes.txt": b"meeting at 10"}

def keystream(key, n):
    out, counter = b"", 0
    while len(out) < n:
        out += hashlib.sha256(key + counter.to_bytes(4, "big")).digest()
        counter += 1
    return out[:n]

def xor(data, key):
    return bytes(a ^ b for a, b in zip(data, keystream(key, len(data))))

def wrap(dek, kek):
    return xor(dek, kek)

keks = {1: b"kek-version-1", 2: b"kek-version-2"}
enabled = {1: True, 2: True}
store = {}
for name, data in objects.items():
    dek = hashlib.sha256(b"dek:" + name.encode()).digest()
    store[name] = {"cipher": xor(data, dek), "wrapped": wrap(dek, keks[1]), "kek": 1}
print("encrypted", len(store), "objects with KEK v1, data bytes:", sum(len(o["cipher"]) for o in store.values()))

touched = 0
for name, obj in store.items():
    if name == "notes.txt":
        continue
    dek = wrap(obj["wrapped"], keks[obj["kek"]])
    obj["wrapped"], obj["kek"] = wrap(dek, keks[2]), 2
    touched += len(obj["wrapped"])
print("rotated to KEK v2: re-wrapped", touched, "bytes of data keys, 0 bytes of data")

enabled[1] = False
print("KEK v1 disabled")
for name, obj in store.items():
    if not enabled[obj["kek"]]:
        print(f"{name}: unreadable, wrapped with disabled KEK v{obj['kek']}")
        continue
    dek = wrap(obj["wrapped"], keks[obj["kek"]])
    plain = xor(obj["cipher"], dek)
    print(f"{name}: KEK v{obj['kek']}, intact={plain == objects[name]}")

@@exercise A posture check against a benchmark
Run CIS-style checks over a cloud inventory and print each failed control with its resources, then a compliance percentage per section (passed checks / total checks, where each resource-check pair counts once). Controls: storage not public, storage encrypted, no security group allowing `0.0.0.0/0` on ports 22, 3389, 3306 or 5432, access keys younger than 90 days, MFA on all human users, audit logging enabled in every region.
@@lang python
@@starter
ADMIN_PORTS = {22, 3389, 3306, 5432}
inventory = {
    "buckets": [{"name": "web-assets", "public": True, "encrypted": True}, {"name": "backups", "public": False, "encrypted": False}, {"name": "logs", "public": False, "encrypted": True}],
    "security_groups": [{"name": "web", "rules": [("0.0.0.0/0", 443), ("0.0.0.0/0", 80)]}, {"name": "bastion", "rules": [("0.0.0.0/0", 22)]}, {"name": "db", "rules": [("10.0.0.0/16", 5432), ("0.0.0.0/0", 5432)]}],
    "users": [{"name": "alice", "mfa": True, "key_age_days": 30}, {"name": "bob", "mfa": False, "key_age_days": 400}, {"name": "ci", "mfa": None, "key_age_days": 120}],
    "regions": {"eu-west-1": True, "us-east-1": True, "ap-southeast-2": False},
}

def run_checks():
    raise NotImplementedError
@@expected
FAIL bucket not public: web-assets
FAIL bucket encrypted: backups
FAIL no admin port open to the internet: bastion, db
FAIL MFA enabled: bob
FAIL access key younger than 90 days: bob, ci
FAIL audit logging enabled: ap-southeast-2
Storage    4/6 = 67%
Networking 1/3 = 33%
Identity   2/5 = 40%
Logging    2/3 = 67%
Overall 9/17 = 53%
@@requires
def run_checks
ADMIN_PORTS
90
@@hint
Have `run_checks` return a list of `(section, control, resource, passed)`. Users with `mfa is None` are machine users, so skip the MFA check for them (but still check key age). Group by section for the percentages.
@@solution
ADMIN_PORTS = {22, 3389, 3306, 5432}
inventory = {
    "buckets": [{"name": "web-assets", "public": True, "encrypted": True}, {"name": "backups", "public": False, "encrypted": False}, {"name": "logs", "public": False, "encrypted": True}],
    "security_groups": [{"name": "web", "rules": [("0.0.0.0/0", 443), ("0.0.0.0/0", 80)]}, {"name": "bastion", "rules": [("0.0.0.0/0", 22)]}, {"name": "db", "rules": [("10.0.0.0/16", 5432), ("0.0.0.0/0", 5432)]}],
    "users": [{"name": "alice", "mfa": True, "key_age_days": 30}, {"name": "bob", "mfa": False, "key_age_days": 400}, {"name": "ci", "mfa": None, "key_age_days": 120}],
    "regions": {"eu-west-1": True, "us-east-1": True, "ap-southeast-2": False},
}

def run_checks():
    results = []
    for b in inventory["buckets"]:
        results.append(("Storage", "bucket not public", b["name"], not b["public"]))
        results.append(("Storage", "bucket encrypted", b["name"], b["encrypted"]))
    for sg in inventory["security_groups"]:
        open_admin = any(src == "0.0.0.0/0" and port in ADMIN_PORTS for src, port in sg["rules"])
        results.append(("Networking", "no admin port open to the internet", sg["name"], not open_admin))
    for u in inventory["users"]:
        if u["mfa"] is not None:
            results.append(("Identity", "MFA enabled", u["name"], u["mfa"]))
        results.append(("Identity", "access key younger than 90 days", u["name"], u["key_age_days"] < 90))
    for region, enabled in inventory["regions"].items():
        results.append(("Logging", "audit logging enabled", region, enabled))
    return results

results = run_checks()
failed = {}
for section, control, resource, passed in results:
    if not passed:
        failed.setdefault(control, []).append(resource)
for control, resources in failed.items():
    print(f"FAIL {control}: {', '.join(resources)}")

sections = {}
for section, _, _, passed in results:
    total, ok = sections.get(section, (0, 0))
    sections[section] = (total + 1, ok + passed)
for section, (total, ok) in sections.items():
    print(f"{section:<10} {ok}/{total} = {ok / total:.0%}")
all_ok = sum(ok for _, ok in sections.values())
print(f"Overall {all_ok}/{len(results)} = {all_ok / len(results):.0%}")

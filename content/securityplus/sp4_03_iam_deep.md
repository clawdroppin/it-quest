@@lesson sp4-iam Identity and Access Management in Depth
@@text
Identity is the new perimeter: most attacks now use stolen or misused credentials rather than breaking through firewalls. Security+ covers the account life cycle, federation and single sign-on, MFA factors, password policy, access control models and privileged access management. Expect "which model is this?" and "which protocol fits?" questions.

## Account life cycle

- **Provisioning**: create accounts from HR data (joiner), with the role's access.
- **Changes** (mover): add the new role's access **and remove the old** to prevent **privilege creep**.
- **Deprovisioning** (leaver): disable on the last day (or immediately for terminations), revoke tokens and keys, transfer data.
- **Permission assignments and implications**, **identity proofing** (verify who someone is before giving an account), **attestation** (managers periodically confirm their staff's access is still needed: **access reviews**).

## Single sign-on and federation

| Protocol | What it does |
|---|---|
| **LDAP** | directory queries (AD); LDAPS for TLS |
| **Kerberos** | tickets inside a Windows domain (TGT from the KDC, then service tickets) |
| **SAML 2.0** | XML assertions from an **identity provider (IdP)** to a **service provider (SP)**: enterprise web SSO |
| **OAuth 2.0** | **authorisation**: lets an app access an API on the user's behalf with a token, without the password |
| **OpenID Connect (OIDC)** | **authentication** layer on top of OAuth 2.0 (ID token): "Sign in with..." |

**Federation** means trusting another organisation's identity provider (a partner's staff sign in with their own accounts). **Interoperability** between systems relies on these standards.

## Authentication factors

| Factor | Examples |
|---|---|
| **Something you know** | password, PIN, security questions |
| **Something you have** | phone authenticator app, hardware key (FIDO2), smart card |
| **Something you are** | fingerprint, face, iris, voice |
| **Somewhere you are** | location, IP range |

MFA needs **different** factors: password + PIN is still single-factor.

Methods: **TOTP** (time-based codes), **HOTP** (counter-based), **push notifications** (use number matching), **SMS** (weakest: SIM swapping), **hardware tokens**, **security keys** / **passkeys** (FIDO2/WebAuthn: phishing-resistant because the key is bound to the real site's origin), **biometrics** (watch the **false acceptance rate** and **false rejection rate**; their crossover is the **CER**: lower is better).

## Password best practices

- **Length** over complexity (NIST: at least 8, prefer 15+; allow long passphrases).
- **No periodic forced changes** unless compromise is suspected.
- Check against **breached password lists**.
- **Password managers**; **passwordless** where possible.
- No **reuse**; **lockout** or throttling after failed attempts.

## Access control models

| Model | How access is decided | Example |
|---|---|---|
| **DAC** (discretionary) | the **owner** decides | NTFS permissions set by the file owner |
| **MAC** (mandatory) | **labels/classifications** set by the system; users can't change them | military "Secret" clearance, SELinux |
| **RBAC** (role-based) | by **job role** | "Accounts Payable" role gets the AP app |
| **Rule-based** | system-wide **rules** | firewall rules, "no logins after 20:00" |
| **ABAC** (attribute-based) | **attributes** of user, resource, environment | "doctors may read records of patients in their department during their shift" |
| **Time-of-day restrictions** | when | |
| **Least privilege** | minimum needed | |

## Privileged access management (PAM)

- **Just-in-time (JIT)** permissions: admin rights granted for a task and expire.
- **Password vaulting**: privileged passwords kept in a vault, checked out, rotated after use; session recording.
- **Ephemeral credentials**: short-lived tokens instead of permanent keys.
- Separate **admin accounts** from everyday accounts; no admin email or browsing.
- **Service accounts**: managed (gMSA), no interactive login, least privilege, rotated secrets.

## Other IAM concepts

**SSO** reduces password fatigue but makes the IdP critical (protect it with MFA). **Conditional access** policies combine signals (user risk, device compliance, location) to allow, require MFA or block. **Impossible travel** alerts fire when one account logs in from distant places too quickly.

@@quiz
An app lets users "Sign in with" their company account and receives an ID token. Which protocol is this?
- LDAP
- OAuth 2.0 alone
* OpenID Connect
- RADIUS
? OIDC adds authentication on top of OAuth 2.0.

@@quiz
A user enters a password and then a PIN. How many authentication factors is that?
* One (both are something you know)
- Two
- Three
- Zero
? MFA needs different factor types.

@@quiz
Access to files is decided by classification labels that users can't change. Which model?
- DAC
* MAC
- RBAC
- ABAC
? Mandatory access control.

@@quiz
Admin rights are granted for a two-hour task, approved, then removed automatically. What is this?
- Privilege creep
* Just-in-time access
- Federation
- Attestation
? Part of privileged access management.

@@quiz
Which MFA method resists phishing because it's bound to the real website's origin?
- SMS codes
- TOTP codes
* FIDO2 security keys / passkeys
- Security questions
? A fake site can't get a valid signature.

@@flashcards
provisioning / deprovisioning :: create / remove accounts
privilege creep :: access accumulating over role changes
attestation :: managers confirm access is still needed
SAML :: XML SSO assertions, IdP to SP
OAuth 2.0 :: delegated authorisation tokens
OIDC :: authentication on OAuth 2.0
Kerberos :: tickets from a KDC
federation :: trusting another organisation's IdP
TOTP / HOTP :: time / counter-based codes
FIDO2 :: phishing-resistant hardware/passkey auth
FAR / FRR / CER :: false accept / false reject / crossover
DAC :: owner decides
MAC :: labels decide
RBAC :: roles decide
ABAC :: attributes decide
JIT access :: temporary admin rights
password vaulting :: check out and rotate privileged passwords
impossible travel :: logins too far apart too quickly

@@drill
Which access control model lets the owner of a resource decide who can access it?
= DAC
= dac
= discretionary
? Discretionary access control.

@@drill
Which protocol carries XML assertions from an identity provider to a service provider?
= SAML
= saml
= SAML 2.0
? Used for enterprise web SSO.

@@exercise Evaluate ABAC rules
Write `allowed(user, record, hour)` implementing these attribute rules: doctors may read records in their own department during their shift (`shift` is a `(start, end)` hour range, end exclusive); nurses may read records in their department only between 8 and 20; anyone may read their **own** record at any time; auditors may read any record but only if `user["mfa"]` is True. Print the decision for each request.
@@lang python
@@starter
users = {
    "dr_sato": {"role": "doctor", "dept": "cardiology", "shift": (8, 16), "mfa": True},
    "nurse_lee": {"role": "nurse", "dept": "cardiology", "shift": (20, 24), "mfa": True},
    "audit_kim": {"role": "auditor", "dept": "compliance", "shift": (9, 17), "mfa": False},
    "patient_ito": {"role": "patient", "dept": None, "shift": (0, 24), "mfa": True},
}
records = {"rec-1": {"dept": "cardiology", "patient": "patient_ito"}, "rec-2": {"dept": "oncology", "patient": "patient_mori"}}
requests = [("dr_sato", "rec-1", 10), ("dr_sato", "rec-1", 18), ("dr_sato", "rec-2", 10),
            ("nurse_lee", "rec-1", 21), ("nurse_lee", "rec-1", 14), ("audit_kim", "rec-2", 11), ("patient_ito", "rec-1", 3)]

def allowed(user, record, hour):
    raise NotImplementedError
@@expected
dr_sato -> rec-1 at 10:00: ALLOW (on shift)
dr_sato -> rec-1 at 18:00: DENY (off shift)
dr_sato -> rec-2 at 10:00: DENY (other department)
nurse_lee -> rec-1 at 21:00: DENY (outside 08-20)
nurse_lee -> rec-1 at 14:00: ALLOW (within 08-20)
audit_kim -> rec-2 at 11:00: DENY (MFA required)
patient_ito -> rec-1 at 03:00: ALLOW (own record)
@@requires
def allowed
"auditor"
@@hint
Check the own-record rule first, then branch on the role. Return a `(decision, reason)` tuple to make the output readable.
@@solution
users = {
    "dr_sato": {"role": "doctor", "dept": "cardiology", "shift": (8, 16), "mfa": True},
    "nurse_lee": {"role": "nurse", "dept": "cardiology", "shift": (20, 24), "mfa": True},
    "audit_kim": {"role": "auditor", "dept": "compliance", "shift": (9, 17), "mfa": False},
    "patient_ito": {"role": "patient", "dept": None, "shift": (0, 24), "mfa": True},
}
records = {"rec-1": {"dept": "cardiology", "patient": "patient_ito"}, "rec-2": {"dept": "oncology", "patient": "patient_mori"}}
requests = [("dr_sato", "rec-1", 10), ("dr_sato", "rec-1", 18), ("dr_sato", "rec-2", 10),
            ("nurse_lee", "rec-1", 21), ("nurse_lee", "rec-1", 14), ("audit_kim", "rec-2", 11), ("patient_ito", "rec-1", 3)]

def allowed(user, record, hour):
    if record["patient"] == user["name"]:
        return True, "own record"
    role = user["role"]
    same_dept = user["dept"] == record["dept"]
    if role == "doctor":
        start, end = user["shift"]
        if not same_dept:
            return False, "other department"
        return (start <= hour < end), "on shift" if start <= hour < end else "off shift"
    if role == "nurse":
        if not same_dept:
            return False, "other department"
        return (8 <= hour < 20), "within 08-20" if 8 <= hour < 20 else "outside 08-20"
    if role == "auditor":
        return user["mfa"], "MFA used" if user["mfa"] else "MFA required"
    return False, "no rule"

for name, rec, hour in requests:
    user = dict(users[name], name=name)
    ok, reason = allowed(user, records[rec], hour)
    print(f"{name} -> {rec} at {hour:02d}:00: {'ALLOW' if ok else 'DENY'} ({reason})")

@@exercise Run a quarterly access review
Compare each user's current entitlements with what their role should have (`ROLE_ACCESS`). Report **excess** access (privilege creep), **missing** access, and accounts of people who left (`status` `left`) that are still enabled. Print a line per finding and a summary count of excess grants to revoke.
@@lang python
@@starter
ROLE_ACCESS = {
    "accounts-payable": {"erp-ap", "email", "file-finance"},
    "sales": {"crm", "email", "file-sales"},
    "it-support": {"helpdesk", "email", "ad-reset"},
}
staff = [
    {"user": "ana", "role": "sales", "status": "active", "enabled": True, "access": {"crm", "email", "file-sales", "erp-ap", "file-finance"}},
    {"user": "ken", "role": "accounts-payable", "status": "active", "enabled": True, "access": {"erp-ap", "email"}},
    {"user": "mei", "role": "it-support", "status": "left", "enabled": True, "access": {"helpdesk", "email", "ad-reset"}},
    {"user": "omar", "role": "it-support", "status": "active", "enabled": True, "access": {"helpdesk", "email", "ad-reset", "domain-admin"}},
]
@@expected
ana: excess erp-ap, file-finance -> revoke
ken: missing file-finance -> request
mei: LEFT but still enabled -> disable account
omar: excess domain-admin -> revoke
Grants to revoke: 3
@@requires
ROLE_ACCESS[
- expected
@@hint
`expected = ROLE_ACCESS[s["role"]]`; excess is `s["access"] - expected`, missing is `expected - s["access"]`.
@@solution
ROLE_ACCESS = {
    "accounts-payable": {"erp-ap", "email", "file-finance"},
    "sales": {"crm", "email", "file-sales"},
    "it-support": {"helpdesk", "email", "ad-reset"},
}
staff = [
    {"user": "ana", "role": "sales", "status": "active", "enabled": True, "access": {"crm", "email", "file-sales", "erp-ap", "file-finance"}},
    {"user": "ken", "role": "accounts-payable", "status": "active", "enabled": True, "access": {"erp-ap", "email"}},
    {"user": "mei", "role": "it-support", "status": "left", "enabled": True, "access": {"helpdesk", "email", "ad-reset"}},
    {"user": "omar", "role": "it-support", "status": "active", "enabled": True, "access": {"helpdesk", "email", "ad-reset", "domain-admin"}},
]

to_revoke = 0
for s in staff:
    if s["status"] == "left" and s["enabled"]:
        print(f"{s['user']}: LEFT but still enabled -> disable account")
        continue
    expected = ROLE_ACCESS[s["role"]]
    excess = s["access"] - expected
    missing = expected - s["access"]
    if excess:
        to_revoke += len(excess)
        print(f"{s['user']}: excess {', '.join(sorted(excess))} -> revoke")
    if missing:
        print(f"{s['user']}: missing {', '.join(sorted(missing))} -> request")
    if not excess and not missing:
        print(f"{s['user']}: OK")
print("Grants to revoke:", to_revoke)

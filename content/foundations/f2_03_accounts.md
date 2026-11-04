@@lesson f2-accounts Users, Groups, Permissions and Directory Services
@@text
"I can't open the Finance folder", "the new starter can't log in", "remove Ken's access, he left yesterday": identity and access work is a huge part of every IT job. The tools differ between Windows, Linux and the cloud, but the ideas are the same everywhere.

## Authentication vs authorisation

- **Authentication (AuthN)**: proving *who you are*: a password, a fingerprint, a security key, a one-time code.
- **Authorisation (AuthZ)**: deciding *what you may do* once you're known: read this folder, approve this invoice.
- **Accounting/auditing**: recording what you did (logs). Together: **AAA**.

**MFA** (multi-factor authentication) combines factors of different kinds: something you **know** (password), something you **have** (phone app, security key), something you **are** (fingerprint, face). Two passwords are not MFA. MFA blocks the vast majority of account takeovers, which is why every company should require it. Phishing-resistant MFA (FIDO2 security keys, passkeys) is the strongest.

## Local accounts vs directory accounts

A **local account** exists on one computer only. Fine at home; unmanageable for 500 staff. Organisations use a **directory service**: one central database of users, groups and computers that every system trusts.

- **Active Directory Domain Services (AD DS)**: Microsoft's on-premises directory, run on servers called **domain controllers**. Computers are **joined** to the domain; users log in with `EXAMPLE\asilva` or `asilva@example.com`. Uses **Kerberos** for logins and **LDAP** for lookups.
- **Microsoft Entra ID** (formerly Azure AD): the cloud directory behind Microsoft 365. Many companies run both, synchronised with Entra Connect (**hybrid**).
- **Google Workspace**, **Okta**, **JumpCloud**: other cloud directories and identity providers.
- **Single sign-on (SSO)**: log in once with the company identity and reach many apps (SAML, OpenID Connect). Turning off one account then removes access everywhere.

Inside AD, objects live in **Organizational Units** (OUs), folders like `Staff/Tokyo/Sales`. **Group Policy** (GPOs) linked to OUs pushes settings to users and computers: password rules, mapped drives, a desktop wallpaper, disabling USB storage. In cloud-managed fleets, **Intune** configuration profiles do the same job.

## Groups: never assign permissions to people

The golden rule: give permissions to **groups**, put **users** in groups. When Ana moves from Sales to Finance, you change her group membership; you don't hunt through 40 folders to edit permissions.

A common pattern (AGDLP in AD): user **A**ccounts go into **G**lobal groups by role (`GG-Finance-Staff`), which go into **D**omain **L**ocal groups by resource and access level (`DL-FinanceShare-Modify`), which get the **P**ermissions on the resource.

Naming groups clearly (`SG-HR-Payroll-Read`) makes audits and access reviews possible.

## File permissions

### Windows (NTFS)

| Permission | Allows |
|---|---|
| Read | open files, list folders |
| Write | create files and folders |
| Read & execute | run programs |
| Modify | read, write, delete |
| Full control | all of the above, plus change permissions and take ownership |

- Permissions **inherit** from parent folders unless inheritance is turned off.
- **Deny** beats allow.
- Network shares have **share permissions** too; the effective permission is the **more restrictive** of share and NTFS. Common practice: share permission "Everyone: Full control" (or Authenticated Users: Change) and control everything with NTFS.
- Check with right-click → Properties → Security → Advanced → **Effective Access**.

### Linux

Each file has an owner, a group and permissions for **u**ser, **g**roup and **o**thers: **r**ead (4), **w**rite (2), e**x**ecute (1).

```text
-rwxr-x---  1 asilva finance  2048 Mar  5 report.sh
 user=rwx(7) group=r-x(5) others=---(0)   → chmod 750 report.sh
```

`chmod` changes permissions, `chown` changes owner, `sudo` runs a command as root. On directories, `x` means "may enter".

## Least privilege and admin rights

- **Least privilege**: everyone gets the minimum access needed for their job, and no more. It limits mistakes and the damage from a compromised account.
- Staff shouldn't be local **administrators** on their PCs: most malware needs admin rights to dig in. Software is installed through a company portal instead.
- IT staff use a **separate admin account** (`asilva-adm`) for admin work and a normal account for email and browsing.
- **Privileged Access Management (PAM)** and just-in-time admin rights (granted for an hour, logged) are the mature version.

## The account lifecycle: joiners, movers, leavers

- **Joiner**: create the account from HR's data, put it in role groups, licence it, hand over credentials securely, require MFA enrolment on first login.
- **Mover**: add new role groups **and remove the old ones** (otherwise people accumulate access: "privilege creep").
- **Leaver**: disable (don't delete at once) the account on the last day, revoke sessions and MFA, remove from groups, convert the mailbox to shared or forward it, recover devices. Delete after a retention period.
- **Access reviews**: managers confirm every few months that their team's access is still needed.

## Password resets and account lockouts

Most help desks handle dozens daily. The rules: **verify identity** before resetting (call back on a known number, a manager confirmation, or a self-service reset portal with MFA), never send passwords by email, set "must change at next logon". A **lockout** (too many wrong passwords) often comes from an old password saved on a phone or a mapped drive retrying in the background, so find and fix the source or it'll lock again.

@@quiz
Which is an example of multi-factor authentication?
- A password and a security question
* A password and a code from an authenticator app
- Two different passwords
- A username and a password
? Factors must be different kinds: know, have, are.

@@quiz
Ana moves from Sales to Finance. What's the correct way to update her access?
* Add her to the Finance role groups and remove her from the Sales groups
- Add permissions for her account directly on each Finance folder
- Keep Sales access in case she needs it
- Create a new account
? Removing old access prevents privilege creep.

@@quiz
The share permission is Read and the NTFS permission is Modify. What can a user do over the network?
* Read only: the more restrictive of the two applies
- Modify
- Full control
- Nothing
? Effective = most restrictive of share and NTFS.

@@quiz
What does `chmod 640 file` give?
- Owner rwx, group r, others none
* Owner rw, group r, others none
- Everyone read and write
- Owner r, group rw, others none
? 6 = 4+2 (rw), 4 = r, 0 = none.

@@quiz
A user's account keeps locking every 30 minutes after a password change. Most likely cause?
- A hacker
* A device or app still trying the old password (phone mail, mapped drive, saved credential)
- The domain controller is broken
- The keyboard layout
? Find the source in the DC's lockout events.

@@flashcards
authentication :: proving who you are
authorisation :: what you're allowed to do
MFA factors :: know, have, are
Active Directory :: on-prem Microsoft directory (domain controllers)
Entra ID :: Microsoft's cloud directory
SSO :: one login for many apps (SAML, OIDC)
OU :: folder of AD objects for organising and GPOs
GPO :: Group Policy Object: pushed settings
AGDLP :: accounts → global groups → domain local groups → permissions
NTFS Modify :: read, write and delete
share + NTFS :: the more restrictive wins
`chmod 750` :: rwx for owner, r-x for group, none for others
least privilege :: minimum access needed
joiner / mover / leaver :: the account lifecycle
privilege creep :: access that accumulates over moves

@@drill
What octal value means read + write?
= 6
? r=4 + w=2.

@@drill
What does AAA stand for? (three words, comma-separated)
= authentication, authorisation, accounting
= authentication, authorization, accounting
? Who you are, what you may do, what you did.

@@exercise Linux permissions in both notations
Write `to_symbolic(octal)` that turns `"750"` into `"rwxr-x---"` and `to_octal(symbolic)` that does the reverse. Use the values r=4, w=2, x=1. Print both conversions for the samples and check that converting there and back gives the original.
@@lang python
@@starter
samples = ["750", "644", "600", "777", "755"]

def to_symbolic(octal):
    raise NotImplementedError

def to_octal(symbolic):
    raise NotImplementedError
@@expected
750 -> rwxr-x--- -> 750 ok
644 -> rw-r--r-- -> 644 ok
600 -> rw------- -> 600 ok
777 -> rwxrwxrwx -> 777 ok
755 -> rwxr-xr-x -> 755 ok
@@requires
def to_symbolic
def to_octal
@@hint
For each digit d: `("r" if d & 4 else "-") + ("w" if d & 2 else "-") + ("x" if d & 1 else "-")`. Reverse: take the string in chunks of 3 and add 4/2/1 for each letter present.
@@solution
samples = ["750", "644", "600", "777", "755"]

def to_symbolic(octal):
    out = ""
    for ch in octal:
        d = int(ch)
        out += ("r" if d & 4 else "-") + ("w" if d & 2 else "-") + ("x" if d & 1 else "-")
    return out

def to_octal(symbolic):
    digits = ""
    for i in range(0, 9, 3):
        chunk = symbolic[i:i + 3]
        value = (4 if chunk[0] == "r" else 0) + (2 if chunk[1] == "w" else 0) + (1 if chunk[2] == "x" else 0)
        digits += str(value)
    return digits

for s in samples:
    sym = to_symbolic(s)
    back = to_octal(sym)
    print(f"{s} -> {sym} -> {back} {'ok' if back == s else 'MISMATCH'}")

@@exercise A leaver checklist and access review
Given users with their groups and an HR list of leavers, print for each leaver the actions to take (disable the account, remove from each group in sorted order). Then run an access review for the movers: anyone in **both** a `Sales` and a `Finance` group is flagged for privilege creep. Finally print the members of each group sorted by name after the leavers are removed.
@@lang python
@@starter
users = {
    "asilva": {"GG-Finance-Staff", "GG-All-Staff"},
    "kito": {"GG-Sales-Staff", "GG-All-Staff", "GG-Finance-Staff"},
    "pnovak": {"GG-IT-Admins", "GG-All-Staff"},
    "fkhan": {"GG-Sales-Staff", "GG-All-Staff"},
}
leavers = ["pnovak"]
@@expected
Leaver pnovak:
  disable account, revoke sessions and MFA
  remove from GG-All-Staff
  remove from GG-IT-Admins
Review: kito is in both Sales and Finance groups
GG-All-Staff: asilva, fkhan, kito
GG-Finance-Staff: asilva, kito
GG-Sales-Staff: fkhan, kito
@@requires
sorted(
for group in
@@hint
Leaver actions: print `disable account` then `remove from <group>` for `sorted(users[name])`. Privilege creep: `any("Sales" in g for g in groups) and any("Finance" in g for g in groups)`. For the final report, build a `dict` of group → list of names, skipping leavers.
@@solution
users = {
    "asilva": {"GG-Finance-Staff", "GG-All-Staff"},
    "kito": {"GG-Sales-Staff", "GG-All-Staff", "GG-Finance-Staff"},
    "pnovak": {"GG-IT-Admins", "GG-All-Staff"},
    "fkhan": {"GG-Sales-Staff", "GG-All-Staff"},
}
leavers = ["pnovak"]

for name in leavers:
    print(f"Leaver {name}:")
    print("  disable account, revoke sessions and MFA")
    for group in sorted(users[name]):
        print(f"  remove from {group}")

for name, groups in sorted(users.items()):
    if any("Sales" in g for g in groups) and any("Finance" in g for g in groups):
        print(f"Review: {name} is in both Sales and Finance groups")

members = {}
for name, groups in users.items():
    if name in leavers:
        continue
    for group in groups:
        members.setdefault(group, []).append(name)
for group in sorted(members):
    print(f"{group}: {', '.join(sorted(members[group]))}")

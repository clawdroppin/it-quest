@@lesson a2-secdeep Securing Workstations, Browsers and Mobile Devices
@@text
Core 2 domain 2 (security, about 28%) is the largest together with operating systems. Beyond the theory in the earlier security lesson, it asks which Windows setting or practice fixes a given risk. This lesson is the hands-on checklist a help desk technician applies to every PC and phone.

## Windows security settings

| Setting | What it does | Where |
|---|---|---|
| **Microsoft Defender Antivirus** | real-time protection, scans, definition updates | Windows Security → Virus & threat protection |
| **Defender Firewall** | inbound filtering per profile | Windows Security → Firewall & network protection |
| **BitLocker** | full-volume encryption, TPM-backed; recovery key stored in AD/Entra ID | Control Panel → BitLocker (Pro+) |
| **BitLocker To Go** | encrypts USB drives | same |
| **EFS** | encrypts individual files/folders (NTFS, per user) | file Properties → Advanced |
| **UAC** | prompts before admin-level changes | Control Panel → User Accounts |
| **Windows Hello** | PIN, fingerprint, face sign-in tied to the device | Settings → Accounts → Sign-in options |
| **SmartScreen** | warns about unknown downloads and phishing sites | Windows Security → App & browser control |
| **Credential Manager** | stored web and Windows credentials | Control Panel |

**Lost BitLocker recovery key** = the data is gone. Always escrow keys (AD, Entra ID, or a printed copy kept safely).

## Account and login hardening

- **Least privilege**: users are standard users; admins use a separate admin account.
- **Disable the guest account**; rename or disable the built-in Administrator; change default passwords.
- **Password policy**: length over complexity (14+ characters), no forced frequent changes unless compromised, no reuse; **account lockout** after failed attempts.
- **Screen lock** after inactivity (e.g. 5–15 minutes); Win+L when leaving the desk.
- **Restrict login times** where appropriate.
- **Disable AutoRun/AutoPlay** so USB media can't run code automatically.
- **MFA** on every remote and cloud login.

## Data destruction and disposal

| Method | When |
|---|---|
| **Standard format** | not secure: data recoverable |
| **Low-level format / overwrite (zero-fill, multiple passes)** | reuse of HDDs inside the company |
| **Secure erase / cryptographic erase** | SSDs (overwrite tools don't reach all cells; erase the encryption key) |
| **Degaussing** | HDDs and tapes (destroys the drive for reuse) |
| **Physical destruction**: shredding, drilling, incineration | highest assurance |
| **Certificate of destruction** | proof from a third-party recycler |

## Browser security

- Use a **trusted source** for browser and extensions; verify hashes for installers.
- Keep the browser updated; remove unused **extensions and plug-ins**.
- **Password managers**, not the same password everywhere.
- Check the **padlock/certificate**; never click through certificate warnings on unknown sites.
- **Pop-up blockers**, **ad blockers**, clear cache and cookies, **private browsing** (doesn't hide you from your employer or ISP).
- **Proxy** and **secure DNS** settings.
- Browser **sync** spreads saved passwords to every signed-in device: decide if that's allowed.

## Mobile device security

- **Screen locks**: PIN, pattern, biometric, face; **failed attempts** wipe or lock out.
- **Remote wipe**, **locator apps** (Find My, Find My Device), **remote backup**.
- **Full device encryption** (on by default on modern iOS and Android with a passcode).
- **OS updates** and **antivirus/anti-malware** on Android.
- **MDM / MAM**: enforce policies; **BYOD** keeps work data in a managed container that can be wiped without touching personal data.
- **Only official app stores**; no jailbreaking/rooting; don't sideload.
- **Bluetooth/Wi-Fi**: disable when not needed; forget untrusted networks.

## SOHO (small office/home office) router hardening

Change the default admin password, update firmware, **WPA3** (or WPA2-AES), disable **WPS** and **UPnP**, disable remote admin from the WAN, use a **guest network** for visitors and IoT, change the default SSID, consider content filtering, set **static IPs or DHCP reservations** for printers, and only open ports that are needed (port forwarding, not DMZ).

@@quiz
A company is disposing of laptops with SSDs. Which method reliably removes the data while letting the drives be resold?
- Quick format
* Secure erase / cryptographic erase
- Degaussing
- Deleting the user profiles
? Degaussing doesn't work on SSDs and a format leaves data.

@@quiz
A laptop with BitLocker asks for the recovery key after a firmware update. Where should the help desk find it?
* In Active Directory or Entra ID, where keys are escrowed
- In the BIOS
- By running `sfc /scannow`
- From Microsoft support, who can bypass BitLocker
? Nobody can bypass BitLocker without the key.

@@quiz
Which setting stops code on a USB stick from running automatically when inserted?
- UAC
* Disable AutoRun/AutoPlay
- SmartScreen
- EFS
? AutoRun malware spread via USB.

@@quiz
Which SOHO router feature should be disabled because it can be brute-forced?
- WPA3
* WPS
- DHCP
- NAT
? Wi-Fi Protected Setup's PIN is weak.

@@quiz
An employee uses their personal phone for work email. How can the company protect work data without controlling the whole phone?
- Require jailbreaking
* MAM/MDM with a managed work container that can be selectively wiped
- Factory reset the phone weekly
- Ban screen locks
? BYOD selective wipe.

@@flashcards
BitLocker :: full-volume encryption, Pro+
BitLocker To Go :: USB drive encryption
EFS :: per-file NTFS encryption
UAC :: prompts for admin changes
SmartScreen :: blocks unknown downloads and phishing
Windows Hello :: PIN/biometric sign-in
least privilege :: standard user accounts by default
AutoRun :: disable to stop USB malware
secure erase :: SSD data destruction
degaussing :: magnetic wipe for HDDs and tapes
certificate of destruction :: proof from a recycler
remote wipe :: erase a lost phone
BYOD selective wipe :: removes only work data
WPS :: disable on SOHO routers
UPnP :: lets devices open ports; disable
guest network :: isolate visitors and IoT

@@drill
Which Windows feature encrypts individual files rather than the whole volume?
= EFS
? Encrypting File System.

@@drill
Which Wi-Fi security standard should a new SOHO router use?
= WPA3
= wpa3
? WPA2-AES if devices can't do WPA3.

@@exercise Workstation hardening audit
Compare each PC's settings against the baseline. Print each PC with its failed checks (in baseline order) and a score `passed/total`, then list PCs scoring under 80% as `NEEDS ATTENTION`.
@@lang python
@@starter
BASELINE = {
    "defender": True,
    "firewall": True,
    "bitlocker": True,
    "autorun": False,
    "guest_enabled": False,
    "user_is_admin": False,
    "lock_minutes_max": 15,
}
pcs = {
    "PC-101": {"defender": True, "firewall": True, "bitlocker": True, "autorun": False, "guest_enabled": False, "user_is_admin": False, "lock_minutes": 10},
    "PC-102": {"defender": True, "firewall": False, "bitlocker": False, "autorun": False, "guest_enabled": False, "user_is_admin": True, "lock_minutes": 60},
    "PC-103": {"defender": True, "firewall": True, "bitlocker": True, "autorun": True, "guest_enabled": False, "user_is_admin": False, "lock_minutes": 15},
}
@@expected
PC-101: 7/7
PC-102: 3/7 failed: firewall, bitlocker, user_is_admin, screen lock 60 min
PC-103: 6/7 failed: autorun
NEEDS ATTENTION: PC-102
@@requires
BASELINE.items()
lock_minutes_max
@@hint
Loop over `BASELINE.items()`; the lock time is a special case (`pc["lock_minutes"] <= limit`), the rest are equality checks.
@@solution
BASELINE = {
    "defender": True,
    "firewall": True,
    "bitlocker": True,
    "autorun": False,
    "guest_enabled": False,
    "user_is_admin": False,
    "lock_minutes_max": 15,
}
pcs = {
    "PC-101": {"defender": True, "firewall": True, "bitlocker": True, "autorun": False, "guest_enabled": False, "user_is_admin": False, "lock_minutes": 10},
    "PC-102": {"defender": True, "firewall": False, "bitlocker": False, "autorun": False, "guest_enabled": False, "user_is_admin": True, "lock_minutes": 60},
    "PC-103": {"defender": True, "firewall": True, "bitlocker": True, "autorun": True, "guest_enabled": False, "user_is_admin": False, "lock_minutes": 15},
}

attention = []
for name, pc in pcs.items():
    failed = []
    for check, wanted in BASELINE.items():
        if check == "lock_minutes_max":
            if pc["lock_minutes"] > wanted:
                failed.append(f"screen lock {pc['lock_minutes']} min")
        elif pc[check] != wanted:
            failed.append(check)
    passed = len(BASELINE) - len(failed)
    print(f"{name}: {passed}/{len(BASELINE)}" + (" failed: " + ", ".join(failed) if failed else ""))
    if passed / len(BASELINE) < 0.8:
        attention.append(name)
print("NEEDS ATTENTION:", ", ".join(attention) if attention else "none")

@@exercise Choose a data destruction method
For each device being retired, choose the method: if it's going to be **reused inside** the company, `overwrite` for HDDs and `secure erase` for SSDs and phones; if it **leaves** the company (sold or recycled), `secure erase` for SSDs and phones and `overwrite` for HDDs, plus `certificate of destruction` when a recycler takes it; if it held **highly sensitive** data, `shred` regardless. Tapes are always degaussed then shredded. Print the plan for each.
@@lang python
@@starter
devices = [
    ("HR laptop SSD", "ssd", "reuse", False),
    ("Old file server HDD", "hdd", "recycle", False),
    ("Finance backup tape", "tape", "recycle", True),
    ("Exec phone", "phone", "sell", False),
    ("Payroll DB disk", "hdd", "recycle", True),
]

def method(kind, fate, sensitive):
    raise NotImplementedError
@@expected
HR laptop SSD (reuse): secure erase
Old file server HDD (recycle): overwrite + certificate of destruction
Finance backup tape (recycle): degauss + shred + certificate of destruction
Exec phone (sell): secure erase
Payroll DB disk (recycle): shred + certificate of destruction
@@requires
def method
"certificate of destruction"
@@hint
Handle tape first, then `sensitive`, then the base method by kind, then add the certificate when `fate == "recycle"`.
@@solution
devices = [
    ("HR laptop SSD", "ssd", "reuse", False),
    ("Old file server HDD", "hdd", "recycle", False),
    ("Finance backup tape", "tape", "recycle", True),
    ("Exec phone", "phone", "sell", False),
    ("Payroll DB disk", "hdd", "recycle", True),
]

def method(kind, fate, sensitive):
    if kind == "tape":
        steps = ["degauss", "shred"]
    elif sensitive:
        steps = ["shred"]
    elif kind == "hdd":
        steps = ["overwrite"]
    else:
        steps = ["secure erase"]
    if fate == "recycle":
        steps.append("certificate of destruction")
    return steps

for name, kind, fate, sensitive in devices:
    print(f"{name} ({fate}): " + " + ".join(method(kind, fate, sensitive)))

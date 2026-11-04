@@lesson lx2-auth Authentication, SSH Keys, PAM, Auditing and Hardening
@@lang bash
@@text
Linux+ security (about 21%) covers how users authenticate (passwords, keys, PAM, directory services), how to harden SSH and accounts, and how to audit what happened. This lesson complements the security lesson's firewall and SELinux coverage with the account and authentication side.

## Account files

| File | Holds |
|---|---|
| `/etc/passwd` | `name:x:UID:GID:comment:home:shell` (world-readable, no passwords) |
| `/etc/shadow` | password hashes and **aging** (root-only) |
| `/etc/group`, `/etc/gshadow` | groups and members |
| `/etc/login.defs` | defaults: UID ranges, password aging, umask |
| `/etc/skel/` | files copied to new home directories |

A shadow hash starts with its algorithm: `$6$` SHA-512, `$y$` yescrypt (modern default), `$2b$` bcrypt. `!` or `*` in the hash field means the password is locked/disabled.

## Managing accounts

```bash,norun
useradd -m -s /bin/bash -G devs ana      # -m create home, -G supplementary groups
usermod -aG docker ana                   # -a append! (without it, other groups are removed)
passwd ana; passwd -l ana; passwd -u ana # set, lock, unlock
chage -l ana                             # aging info
chage -M 90 -W 14 -I 7 ana               # max age, warning days, inactive days
chage -E 2026-12-31 contractor1          # account expiry date
userdel -r olduser                       # remove with home
id ana; groups ana; who; w; last; lastb  # identity, sessions, login history, failed logins
```

Service accounts get `nologin` shells and no passwords (`useradd -r -s /usr/sbin/nologin svc`).

## SSH keys

```bash,norun
ssh-keygen -t ed25519 -C "ana@laptop"      # key pair in ~/.ssh/id_ed25519(.pub)
ssh-copy-id ana@server                      # appends the public key to ~/.ssh/authorized_keys
ssh -i ~/.ssh/id_ed25519 ana@server
ssh-agent / ssh-add                         # cache the decrypted key
```

Permissions matter: `~/.ssh` 700, `authorized_keys` 600, private keys 600, or sshd refuses them. Protect private keys with a **passphrase**.

`/etc/ssh/sshd_config` hardening:

```text
PermitRootLogin no
PasswordAuthentication no
PubkeyAuthentication yes
KbdInteractiveAuthentication no
AllowGroups ssh-users
MaxAuthTries 3
X11Forwarding no
ClientAliveInterval 300
```

Test with `sshd -t`, then `systemctl reload sshd`, and **keep your current session open** while testing a new one so you can't lock yourself out.

## PAM

**Pluggable Authentication Modules** decide how programs (login, sshd, sudo) authenticate. Config in `/etc/pam.d/`. Four **types**: `auth` (verify identity), `account` (is the account allowed: expiry, time), `password` (changing passwords), `session` (setup/teardown). **Control flags**: `required` (must pass, keep checking), `requisite` (must pass, stop now), `sufficient` (pass is enough if nothing required failed), `optional`.

Useful modules: `pam_pwquality` (password strength: `minlen`, `dcredit`...), `pam_faillock` (lock after failed attempts), `pam_limits` (`/etc/security/limits.conf`: max processes, open files), `pam_tally2` (older), `pam_google_authenticator`/`pam_u2f` (MFA).

## Directory services and SSO

- **LDAP** (OpenLDAP, 389/636), **Kerberos**, joining **Active Directory** with `realm join` (SSSD), **FreeIPA/IdM**.
- `/etc/nsswitch.conf` decides where names are looked up (`passwd: files sss`).
- `getent passwd ana` queries through NSS (local and directory accounts).

## Auditing and logs

- `auditd`: kernel auditing. `auditctl -w /etc/passwd -p wa -k identity` watches writes and attribute changes; `ausearch -k identity`, `aureport`.
- Auth logs: `/var/log/auth.log` (Debian) or `/var/log/secure` (RHEL), or `journalctl -u ssh`.
- `fail2ban` bans IPs after repeated failures.
- **File integrity**: AIDE compares files with a baseline database.

## Hardening checklist

Minimal installed packages; disable unused services (`systemctl disable --now`); patch automatically; SSH keys only; no root login; `sudo` with least privilege; firewall default deny; SELinux/AppArmor enforcing; `noexec,nosuid,nodev` on `/tmp`; strong password policy and lockout; auditing and remote logging; disable USB storage where needed (`blacklist usb-storage`); remove unnecessary SUID bits; secure boot and GRUB password for physical access; **CIS Benchmark** as the reference.

## Encryption at rest and in transit

**LUKS** (`cryptsetup luksFormat`, `luksOpen`) for disk encryption; `gpg` to encrypt/sign files; certificates and `openssl` (`openssl x509 -in cert.pem -noout -dates -subject`); `openssl s_client -connect host:443` to inspect a TLS server.

@@quiz
A user added with `usermod -G docker ana` lost access to their other groups. What should have been used?
* `usermod -aG docker ana`
- `usermod -g docker ana`
- `groupadd docker ana`
- `useradd -G docker ana`
? Without `-a`, -G replaces the supplementary group list.

@@quiz
sshd ignores a user's authorized_keys file. What's a common cause?
- The key is ed25519
* The ~/.ssh directory or authorized_keys file permissions are too open
- PasswordAuthentication is no
- The user's shell is bash
? Use 700 for ~/.ssh and 600 for authorized_keys.

@@quiz
In PAM, which control flag stops processing immediately if the module fails?
- required
* requisite
- sufficient
- optional
? `required` fails too, but keeps checking other modules first.

@@quiz
Which command makes an account expire on a specific date?
- passwd -l
* chage -E 2026-12-31 user
- usermod -L user
- userdel user
? `chage -M` sets the max password age instead.

@@quiz
Which tool watches `/etc/shadow` for changes at the kernel level and records who made them?
- fail2ban
* auditd (auditctl -w)
- logrotate
- cron
? Search with ausearch.

@@flashcards
/etc/passwd :: account list, no hashes
/etc/shadow :: hashes and aging, root-only
$y$ / $6$ :: yescrypt / SHA-512 hashes
! in shadow :: locked password
usermod -aG :: append supplementary group
chage -l / -M / -E :: show aging / max age / expiry date
lastb :: failed login history
ssh-keygen -t ed25519 :: create a modern key pair
ssh-copy-id :: install public key on a server
~/.ssh 700, authorized_keys 600 :: required permissions
sshd -t :: test sshd config
PAM types :: auth, account, password, session
requisite :: fail and stop immediately
pam_faillock :: lock after failed attempts
pam_pwquality :: password strength rules
nsswitch.conf :: where names are looked up
getent passwd :: query local and directory users
auditctl -w :: watch a file
LUKS :: Linux disk encryption

@@drill
Which sshd_config setting disables direct root login?
= PermitRootLogin no
? Use sudo from a personal account instead.

@@drill
Which PAM module type checks whether an account is allowed (expiry, time limits)?
= account
? auth verifies identity.

@@exercise Password aging report from /etc/shadow
The `shadow` variable holds shadow lines (`name:hash:lastchg:min:max:warn:inactive:expire:`), with `lastchg` in days since 1970. Given `TODAY=20396` (days since 1970), print for every account with a real password (hash not starting with `!` or `*`): days since the last change, and `EXPIRED` if that exceeds `max`, `NO MAX AGE` if `max` is empty or 99999, otherwise `ok (N days left)`. Then list locked accounts.
@@starter
TODAY=20396
shadow='root:!:20100:0:99999:7:::
ana:$y$j9T$abc$def:20380:1:90:14:::
ken:$y$j9T$ghi$jkl:20250:1:90:14:::
mei:$6$xyz$uvw:20300:0:99999:7:::
svc_backup:*:20000:0:99999:7:::
omar:!$y$j9T$mno$pqr:20200:1:90:14:::'

echo "$shadow"
@@expected
ana: changed 16 days ago, ok (74 days left)
ken: changed 146 days ago, EXPIRED
mei: changed 96 days ago, NO MAX AGE
locked: root svc_backup omar
@@requires
-F:
-v today="$TODAY"
@@hint
`awk -F: -v today="$TODAY"`. Locked: `$2 ~ /^[!*]/`. Age is `today - $3`.
@@solution
TODAY=20396
shadow='root:!:20100:0:99999:7:::
ana:$y$j9T$abc$def:20380:1:90:14:::
ken:$y$j9T$ghi$jkl:20250:1:90:14:::
mei:$6$xyz$uvw:20300:0:99999:7:::
svc_backup:*:20000:0:99999:7:::
omar:!$y$j9T$mno$pqr:20200:1:90:14:::'

echo "$shadow" | awk -F: -v today="$TODAY" '
$2 ~ /^[!*]/ { locked = locked " " $1; next }
{
    age = today - $3
    if ($5 == "" || $5 == 99999) status = "NO MAX AGE"
    else if (age > $5) status = "EXPIRED"
    else status = "ok (" ($5 - age) " days left)"
    printf "%s: changed %d days ago, %s\n", $1, age, status
}
END { print "locked:" locked }'

@@exercise Audit sshd_config
Check the effective settings in `sshd_config` (ignore comments and blank lines; keywords are case-insensitive and the **first** occurrence wins, as in sshd). Compare with `wanted` and print `PASS`/`FAIL` for each wanted setting with the actual value (or `unset`). Finish with the number of failures.
@@starter
sshd_config='# Hardening drop-in
Port 22
PermitRootLogin prohibit-password
passwordauthentication yes
PasswordAuthentication no
PubkeyAuthentication yes
MaxAuthTries 6
X11Forwarding no'
wanted='PermitRootLogin no
PasswordAuthentication no
PubkeyAuthentication yes
MaxAuthTries 3
X11Forwarding no
AllowGroups ssh-users'

echo "$sshd_config"
@@expected
FAIL PermitRootLogin prohibit-password (want no)
FAIL PasswordAuthentication yes (want no)
PASS PubkeyAuthentication yes
FAIL MaxAuthTries 6 (want 3)
PASS X11Forwarding no
FAIL AllowGroups unset (want ssh-users)
failures: 4
@@requires
declare -A
${key,,}
@@hint
Read `while read -r key value`; skip lines where `key` is empty or starts with `#`. Store with a lowercased key (`${key,,}`) only if it's not set yet.
@@solution
sshd_config='# Hardening drop-in
Port 22
PermitRootLogin prohibit-password
passwordauthentication yes
PasswordAuthentication no
PubkeyAuthentication yes
MaxAuthTries 6
X11Forwarding no'
wanted='PermitRootLogin no
PasswordAuthentication no
PubkeyAuthentication yes
MaxAuthTries 3
X11Forwarding no
AllowGroups ssh-users'

declare -A conf
while read -r key value; do
    [[ -z $key || $key == \#* ]] && continue
    k=${key,,}
    [[ -z ${conf[$k]+set} ]] && conf[$k]=$value
done <<< "$sshd_config"

failures=0
while read -r key value; do
    actual=${conf[${key,,}]:-unset}
    if [[ $actual == "$value" ]]; then
        echo "PASS $key $actual"
    else
        echo "FAIL $key $actual (want $value)"
        failures=$((failures + 1))
    fi
done <<< "$wanted"
echo "failures: $failures"

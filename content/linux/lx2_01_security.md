@@lesson lx2-security Linux Security: Permissions, SELinux, Firewalls and SSH
@@lang bash
@@text
## Permissions beyond rwx

```text
-rwxr-x---  1 ytanaka devs  2048 Oct 31 09:00 deploy.sh
 user group other           owner   group
```

Octal: r = 4, w = 2, x = 1, so `rwxr-x---` = **750**. `umask 022` makes new files 644 and folders 755.

| Special bit | On a file | On a folder | Set with |
|---|---|---|---|
| **SUID** (4) | runs as the file's **owner** (`passwd` runs as root) | | `chmod u+s` / `4755` |
| **SGID** (2) | runs as the file's group | new files inherit the folder's **group** (shared team folders) | `chmod g+s` / `2775` |
| **Sticky** (1) | | only the owner can delete their files (`/tmp`) | `chmod +t` / `1777` |

Find SUID files during an audit: `find / -perm -4000 -type f 2>/dev/null`. An unexpected SUID binary is a classic privilege escalation path.

**ACLs** give extra users/groups rights: `setfacl -m u:msato:rw report.txt`, `getfacl report.txt` (a `+` after the permissions in `ls -l` means an ACL exists). **Attributes**: `chattr +i file` makes it immutable even for root; `lsattr` shows them.

## Mandatory access control

- **SELinux** (Red Hat family) labels every file and process with a **context** and only allows what the policy permits. Modes: **enforcing**, **permissive** (logs only), **disabled**. `getenforce`, `setenforce 0/1`, `ls -Z`, `restorecon -Rv /var/www` (fix labels after copying files), `semanage port -a -t http_port_t -p tcp 8080`, booleans with `getsebool`/`setsebool -P`. Denials appear in `/var/log/audit/audit.log` (`ausearch -m avc`, `audit2why`).
- **AppArmor** (Ubuntu, SUSE) uses per-program **profiles**: `aa-status`, `aa-complain`, `aa-enforce`.

"Fixing" SELinux by disabling it is the wrong answer on the exam and at work: fix the label or the boolean.

## Firewalls

| Tool | Distros | Example |
|---|---|---|
| **firewalld** (`firewall-cmd`) | Red Hat family | `firewall-cmd --permanent --add-service=https && firewall-cmd --reload` |
| **ufw** | Ubuntu | `ufw allow 22/tcp; ufw enable; ufw status verbose` |
| **nftables** (`nft`) / **iptables** | underneath both | `iptables -A INPUT -p tcp --dport 22 -j ACCEPT` |

firewalld uses **zones** (public, internal, trusted...). Remember `--permanent` (otherwise it's lost on reload) and to allow SSH *before* enabling a firewall remotely.

## SSH hardening

```bash,norun
ssh-keygen -t ed25519 -C "ytanaka@laptop"
ssh-copy-id ytanaka@server
```

In `/etc/ssh/sshd_config`: `PermitRootLogin no`, `PasswordAuthentication no` (keys only), `AllowGroups ssh-users`, maybe a non-standard `Port`; then `sudo sshd -t` (test) and `sudo systemctl reload sshd`. Add **fail2ban** to ban IPs after failed logins. Key files: `~/.ssh/authorized_keys` (permissions 600, `~/.ssh` 700), `~/.ssh/config`, `known_hosts`.

## More security tools

- **PAM** (`/etc/pam.d/`): pluggable authentication: password quality (`pam_pwquality`), lockouts (`pam_faillock`), MFA modules.
- **auditd**: records security events (`auditctl -w /etc/passwd -p wa -k identity`).
- **LUKS** disk encryption: `cryptsetup luksFormat`, `cryptsetup open`; `/etc/crypttab`.
- **GPG** for signing and encrypting files; package signatures.
- Certificates: `openssl req`, `openssl x509 -in cert.pem -noout -dates`; trust store in `/etc/pki` or `/usr/local/share/ca-certificates` + `update-ca-certificates`.
- Integrity and scanning: **AIDE** (file integrity), **ClamAV**, `rkhunter`, **OpenSCAP**, CIS Benchmarks.

@@quiz
What octal permission is rw-r-----?
* 640
- 644
- 750
- 460
? rw- = 6, r-- = 4, --- = 0.

@@quiz
A shared team folder should give every new file the "devs" group automatically. Which bit?
* SGID on the folder
- SUID on the folder
- The sticky bit
- chattr +i
? SGID on a directory makes new files inherit its group.

@@quiz
Users can delete each other's files in a shared upload folder. Which permission stops that, as on /tmp?
* The sticky bit
- SGID
- SUID
- umask 077
? Only a file's owner (or root) can delete it.

@@quiz
Apache on Red Hat can't read files that were copied into /var/www/html, although the normal permissions are fine. What's the right fix?
* Restore the SELinux contexts with restorecon -Rv /var/www/html
- Disable SELinux
- chmod 777 the folder
- Run Apache as root
? Copied files keep their old SELinux label; restorecon applies the right one.

@@quiz
You opened port 443 with firewall-cmd, and after a reload it's closed again. What was missing?
* --permanent
- --zone=trusted
- sudo
- --now
? Without --permanent, changes are runtime only.

@@quiz
Which sshd_config settings give key-only logins and block direct root logins?
* PasswordAuthentication no and PermitRootLogin no
- UsePAM no and Port 2222
- PermitEmptyPasswords yes
- X11Forwarding yes
? Then test with `sshd -t` and reload, keeping a session open in case.

@@quiz
During an audit you want all SUID programs on a server. Which command?
* find / -perm -4000 -type f
- ls -l / | grep s
- chmod -R u-s /
- getfacl -R /
? Compare the list with a known-good baseline.

@@flashcards
chmod 750 :: rwxr-x---
umask 022 :: New files 644, folders 755
SUID (4) :: Program runs as its owner
SGID (2) :: Runs as the group / new files inherit the folder's group
sticky bit (1) :: Only owners can delete their files in the folder
setfacl / getfacl :: Set / show ACLs
chattr +i :: Immutable file, even for root
SELinux modes :: Enforcing, permissive, disabled
restorecon :: Restore default SELinux labels
semanage / setsebool :: SELinux ports and file contexts / booleans
AppArmor :: Profile-based MAC on Ubuntu/SUSE
firewall-cmd --permanent :: Persistent firewalld rule
ufw :: Ubuntu's simple firewall
ed25519 :: Modern SSH key type
fail2ban :: Bans IPs after repeated failures
PAM :: Pluggable authentication modules
auditd :: Linux security audit logging
LUKS :: Linux disk encryption

@@drill
What octal value is rwxr-xr-x?
= 755
? The usual permission for programs and folders.

@@exercise Review a remote firewall change
An admin will paste the commands in `plan` into an SSH session. Check them in order and print a warning for each problem, then `SAFE` or `NOT SAFE`:

- `ufw enable` before any `ufw allow 22/tcp` (or `ufw allow ssh`) locks you out.
- a `firewall-cmd --add-...` without `--permanent` is lost on the next reload.
- `--permanent` changes without a later `firewall-cmd --reload` don't take effect yet (a warning, but still safe).
- `setenforce 0` or `SELINUX=disabled` turns SELinux off: fix the label or boolean instead.
@@starter
plan='firewall-cmd --add-service=https
firewall-cmd --permanent --add-port=8443/tcp
setenforce 0
ufw enable
ufw allow 22/tcp'

# check each line of "$plan" in order
@@expected
line 1: runtime-only change, lost on reload: add --permanent
line 3: turns SELinux off: fix the label or boolean instead
line 4: ufw enable before allowing SSH locks you out
warning: permanent changes need firewall-cmd --reload
NOT SAFE
@@requires
--reload
NOT SAFE
@@hint
Keep two flags while you read the lines: whether SSH is allowed yet, and whether a permanent change is waiting for a reload. Check the flags again after the loop.
@@solution
plan='firewall-cmd --add-service=https
firewall-cmd --permanent --add-port=8443/tcp
setenforce 0
ufw enable
ufw allow 22/tcp'

ssh_allowed=0
pending=0
unsafe=0
n=0
while IFS= read -r cmd; do
    n=$((n + 1))
    case "$cmd" in
        "ufw allow 22/tcp"|"ufw allow ssh") ssh_allowed=1 ;;
        "ufw enable")
            if [ "$ssh_allowed" = 0 ]; then
                echo "line $n: ufw enable before allowing SSH locks you out"; unsafe=1
            fi ;;
        firewall-cmd*--reload*) pending=0 ;;
        firewall-cmd*--permanent*) pending=1 ;;
        firewall-cmd*--add-*) echo "line $n: runtime-only change, lost on reload: add --permanent" ;;
        "setenforce 0"|*SELINUX=disabled*)
            echo "line $n: turns SELinux off: fix the label or boolean instead"; unsafe=1 ;;
    esac
done <<< "$plan"
if [ "$pending" = 1 ]; then
    echo "warning: permanent changes need firewall-cmd --reload"
fi
if [ "$unsafe" = 1 ]; then echo "NOT SAFE"; else echo "SAFE"; fi

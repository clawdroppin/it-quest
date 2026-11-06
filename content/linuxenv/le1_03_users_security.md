@@lesson le1-users-security Users, sudo and a Secure Desktop: Accounts, Groups, Firewalls, SSH Keys and Encryption
@@concepts files, text-processing, conditions
@@lang bash
@@text
Linux was built as a multi-user system, and its security model starts with **users, groups and permissions**. On a personal desktop you're usually the only person, but the same model protects you: your browser can't overwrite system files, a compromised app runs with your rights rather than root's, and `sudo` asks before doing anything dangerous. This lesson covers the accounts on your machine, how `sudo` works, and the few settings that make a Linux desktop properly secure: a firewall, SSH keys, updates and disk encryption.

## Users and groups

- Every process runs as a **user** with a numeric **UID**. **root** is UID 0 and can do anything. System services have their own users (UID below 1000 on most distros) so a bug in one service can't touch the others. Human accounts start at **UID 1000**.
- `/etc/passwd` lists accounts: `name:x:UID:GID:comment:home:shell`. Passwords are hashed in `/etc/shadow`, which only root can read. Service accounts use `/usr/sbin/nologin` or `/bin/false` as their shell so nobody can log in as them.
- **Groups** give shared permissions. Your membership in some groups grants abilities:
  - `sudo` (Debian/Ubuntu) or `wheel` (Fedora, Arch) lets you use sudo.
  - `docker` lets you run containers, which is effectively root, so treat it as such.
  - `dialout` gives access to serial ports, e.g. for Arduino boards.
  - `video` and `render` give GPU access on some systems.
- Commands:
  - `id` shows your UID and groups.
  - `groups alice` lists someone's groups.
  - `sudo usermod -aG docker alice` adds a group (the `-a` matters: without it, all other groups are replaced).
  - `sudo useradd -m -s /bin/bash bob` creates an account and `sudo passwd bob` sets its password.
  - New group memberships apply after you log out and back in.

## sudo

- `sudo command` runs one command as root after asking for **your** password. It's logged (`journalctl _COMM=sudo`), and the password is cached for a few minutes.
- Rules live in `/etc/sudoers` and `/etc/sudoers.d/`. **Always edit them with `sudo visudo`**, which checks the syntax. A broken sudoers file can lock you out of sudo entirely.
- `sudo -i` opens a root shell. Use it sparingly, because every command is then dangerous.
- Don't run GUI apps with sudo. Use `pkexec` or the app's own privilege prompt (polkit), or edit system files with `sudoedit /etc/file`, which edits a copy as you and saves it as root.
- `sudo !!` reruns the previous command with sudo.

## Extra permissions: ACLs, setuid and capabilities

- **ACLs** add per-user rules beyond owner/group/other: `setfacl -m u:bob:r report.pdf` lets Bob read one file. `getfacl` shows them, and a `+` at the end of the `ls -l` mode means ACLs exist.
- **setuid** programs (an `s` in the owner execute position, like `/usr/bin/passwd`) run as their owner. Unexpected setuid files are a red flag: `find / -perm -4000 -type f 2>/dev/null` lists them.
- **Capabilities** split root's powers into pieces: `getcap /usr/bin/ping` shows `cap_net_raw`, so ping works without being setuid root.

## Securing a desktop

1. **Updates**: install security updates promptly, and enable automatic security updates (`unattended-upgrades` on Debian/Ubuntu, `dnf-automatic` on Fedora). Most real-world compromises use known, already-patched bugs.
2. **Firewall**: desktops shouldn't accept incoming connections they don't need.
   - On Ubuntu, `sudo ufw enable` with the default **deny incoming, allow outgoing** policy, then `sudo ufw allow 22/tcp` only if you run SSH.
   - Fedora uses `firewalld` (`firewall-cmd --list-all`).
   - Check what's listening with `ss -tulpn`.
3. **SSH keys instead of passwords**:
   - `ssh-keygen -t ed25519` creates a key pair. Protect the private key with a passphrase and load it into `ssh-agent`.
   - `ssh-copy-id user@server` installs the public key on a server.
   - On machines you SSH **into**, set `PasswordAuthentication no` and `PermitRootLogin no` in `/etc/ssh/sshd_config`, then `sudo systemctl reload sshd`.
   - Keep `~/.ssh` at mode 700 and private keys at 600, or ssh refuses to use them.
4. **Disk encryption**: tick "encrypt the disk" (LUKS) when installing. A stolen laptop without it is a data breach. Store the recovery passphrase somewhere safe.
5. **Mandatory access control**: **AppArmor** (Ubuntu, openSUSE) and **SELinux** (Fedora, RHEL) confine programs even if they're compromised. Leave them on. If something breaks, read the log (`journalctl -t audit` or `ausearch -m avc`) rather than disabling them.
6. **Software sources**: install from your distro or Flathub, not random scripts piped into `sh`. Flatpak's sandbox permissions can be reviewed and tightened with Flatseal.

@@quiz
Which UID does root have?
* 0
- 1
- 1000
- 65534
? Human accounts start at 1000 on most distros.

@@quiz
Why use `usermod -aG docker alice` rather than `usermod -G docker alice`?
* Without -a, alice's other supplementary groups are replaced
- -a makes the change apply immediately
- -G alone needs no sudo
- There is no difference
? -a means append.

@@quiz
How should you edit the sudoers file?
* With sudo visudo, which checks the syntax before saving
- With any text editor as root
- By deleting it and starting again
- With chmod 777 first
? A syntax error can lock you out of sudo.

@@quiz
What is a sensible firewall policy for a desktop?
* Deny incoming, allow outgoing, and open only the services you run
- Allow everything
- Deny outgoing traffic
- No firewall is ever needed on Linux
? Check listening ports with ss -tulpn.

@@quiz
Why is membership of the docker group a security concern?
* It effectively grants root access
- It slows the system down
- It disables the firewall
- It removes sudo
? Containers can mount the host's filesystem.

@@quiz
What does `PasswordAuthentication no` in sshd_config do?
* Allows only key-based SSH logins
- Disables SSH completely
- Removes all user passwords
- Lets root log in without a password
? Combine with PermitRootLogin no.

@@quiz
Something stopped working on Fedora and the audit log shows SELinux denials. What should you do?
* Read the denial and fix the label or policy, rather than disabling SELinux
- Disable SELinux permanently
- Reinstall the system
- Run the app with sudo
? ausearch -m avc shows the denials.

@@flashcards
UID 0 :: root
UID ≥ 1000 :: human accounts
/etc/passwd :: name:x:UID:GID:comment:home:shell
/etc/shadow :: password hashes, root only
nologin :: shell for service accounts
id :: show your UID and groups
usermod -aG :: append a group
sudo / wheel :: groups that may use sudo
visudo :: safe sudoers editor
sudoedit :: edit a root file as yourself
setfacl / getfacl :: per-user ACLs
setuid :: runs as the file's owner
getcap :: show file capabilities
ufw default :: deny incoming, allow outgoing
ss -tulpn :: listening ports
ssh-keygen -t ed25519 :: new SSH key pair
ssh-copy-id :: install your public key
LUKS :: full-disk encryption
AppArmor / SELinux :: mandatory access control

@@drill
Which command lists the TCP and UDP ports your machine is listening on? (two words)
= ss -tulpn
= ss -tlnp
= ss -tuln
? netstat -tulpn on older systems.

@@drill
What mode should a private SSH key file have? (three digits)
= 600
? Only you can read and write it.

@@exercise Audit the accounts
`passwd` and `group` hold excerpts of `/etc/passwd` and `/etc/group`. With awk, list the human accounts (UID 1000 or more, excluding `nobody`) with their shell, flag any account other than root with UID 0, flag service accounts (UID below 1000) that have a real login shell, and print who can use sudo (members of `sudo` or `wheel`) and who is in `docker`.
@@starter
passwd='root:x:0:0:root:/root:/bin/bash
daemon:x:1:1:daemon:/usr/sbin:/usr/sbin/nologin
sshd:x:110:65534::/run/sshd:/usr/sbin/nologin
postgres:x:120:126:PostgreSQL:/var/lib/postgresql:/bin/bash
alice:x:1000:1000:Alice:/home/alice:/bin/bash
bob:x:1001:1001:Bob:/home/bob:/usr/bin/zsh
backup2:x:0:0:old backup:/root:/bin/bash
nobody:x:65534:65534:nobody:/nonexistent:/usr/sbin/nologin'
group='sudo:x:27:alice
wheel:x:10:
docker:x:998:alice,bob
dialout:x:20:bob'

# Your code here
@@expected
human accounts:
  alice  uid 1000  /bin/bash
  bob    uid 1001  /usr/bin/zsh
warnings:
  backup2 has UID 0 (a second root account)
  service account postgres has a login shell (/bin/bash)
can use sudo: alice
docker group (root-equivalent): alice,bob
@@requires
awk -F:
$3 == 0
@@hint
Use `awk -F: '...'` so fields split on colons. Human accounts: `$3 >= 1000 && $1 != "nobody"`. Extra root: `$3 == 0 && $1 != "root"`. Service accounts with a shell: `$3 > 0 && $3 < 1000 && $7 !~ /nologin|false/`. For groups, the members are field 4, separated by commas.
@@solution
passwd='root:x:0:0:root:/root:/bin/bash
daemon:x:1:1:daemon:/usr/sbin:/usr/sbin/nologin
sshd:x:110:65534::/run/sshd:/usr/sbin/nologin
postgres:x:120:126:PostgreSQL:/var/lib/postgresql:/bin/bash
alice:x:1000:1000:Alice:/home/alice:/bin/bash
bob:x:1001:1001:Bob:/home/bob:/usr/bin/zsh
backup2:x:0:0:old backup:/root:/bin/bash
nobody:x:65534:65534:nobody:/nonexistent:/usr/sbin/nologin'
group='sudo:x:27:alice
wheel:x:10:
docker:x:998:alice,bob
dialout:x:20:bob'

echo "human accounts:"
printf '%s\n' "$passwd" | awk -F: '$3 >= 1000 && $1 != "nobody" { printf "  %-6s uid %-5s %s\n", $1, $3, $7 }'
echo "warnings:"
printf '%s\n' "$passwd" | awk -F: '$3 == 0 && $1 != "root" { print "  " $1 " has UID 0 (a second root account)" }'
printf '%s\n' "$passwd" | awk -F: '$3 > 0 && $3 < 1000 && $7 !~ /nologin|false/ { print "  service account " $1 " has a login shell (" $7 ")" }'
printf '%s\n' "$group" | awk -F: '
    ($1 == "sudo" || $1 == "wheel") && $4 != "" { admins = admins (admins ? "," : "") $4 }
    $1 == "docker" { docker = $4 }
    END {
        print "can use sudo: " (admins ? admins : "nobody")
        print "docker group (root-equivalent): " docker
    }'

@@exercise Check sshd_config
`config` holds an `sshd_config` file. Ignore comments and blank lines, read each setting (keywords are case-insensitive, and the first occurrence wins, as in sshd), apply the defaults for settings that are missing, and report each check as OK or FIX with the line to use: no root login, no password authentication, no empty passwords, X11 forwarding off, and a MaxAuthTries of 4 or fewer.
@@starter
config='# sshd_config
Port 22
#PermitRootLogin prohibit-password
permitrootlogin yes
PasswordAuthentication yes
PasswordAuthentication no
X11Forwarding yes
MaxAuthTries 6
PubkeyAuthentication yes'

# Your code here
@@expected
FIX  PermitRootLogin yes       -> PermitRootLogin no
FIX  PasswordAuthentication yes  -> PasswordAuthentication no (set up keys first)
OK   PermitEmptyPasswords no
FIX  X11Forwarding yes         -> X11Forwarding no
FIX  MaxAuthTries 6            -> MaxAuthTries 3
then: sudo sshd -t && sudo systemctl reload sshd
@@requires
tolower(
!seen[
@@hint
Use awk: skip lines matching `/^[[:space:]]*(#|$)/`, set `key = tolower($1)`, and store `val[key] = $2` only if `!seen[key]++`. In `END`, fill in defaults for missing keys (permitrootlogin "prohibit-password", passwordauthentication "yes", permitemptypasswords "no", x11forwarding "no", maxauthtries 6) and print each check.
@@solution
config='# sshd_config
Port 22
#PermitRootLogin prohibit-password
permitrootlogin yes
PasswordAuthentication yes
PasswordAuthentication no
X11Forwarding yes
MaxAuthTries 6
PubkeyAuthentication yes'

printf '%s\n' "$config" | awk '
    /^[[:space:]]*(#|$)/ { next }
    { key = tolower($1); if (!seen[key]++) val[key] = $2 }
    function get(k, d) { return (k in val) ? val[k] : d }
    function report(ok, name, fix) { printf "%-4s %-24s%s\n", ok ? "OK" : "FIX", name, ok ? "" : "  -> " fix }
    END {
        root = get("permitrootlogin", "prohibit-password")
        report(root == "no", "PermitRootLogin " root, "PermitRootLogin no")
        pw = get("passwordauthentication", "yes")
        report(pw == "no", "PasswordAuthentication " pw, "PasswordAuthentication no (set up keys first)")
        empty = get("permitemptypasswords", "no")
        report(empty == "no", "PermitEmptyPasswords " empty, "PermitEmptyPasswords no")
        x11 = get("x11forwarding", "no")
        report(x11 == "no", "X11Forwarding " x11, "X11Forwarding no")
        tries = get("maxauthtries", 6)
        report(tries + 0 <= 4, "MaxAuthTries " tries, "MaxAuthTries 3")
    }' | sed 's/[[:space:]]*$//'
echo "then: sudo sshd -t && sudo systemctl reload sshd"

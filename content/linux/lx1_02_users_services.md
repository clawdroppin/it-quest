@@lesson lx1-users-services Users, Packages, Services and Logs
@@lang bash
@@text
## Users and groups

```bash,norun
sudo useradd -m -s /bin/bash -c "Yuki Tanaka" ytanaka   # -m creates the home
sudo passwd ytanaka
sudo usermod -aG sudo ytanaka      # add to a group (-a! without it, other groups are removed); "wheel" on Red Hat
sudo chage -M 90 -l ytanaka        # password ageing
sudo userdel -r olduser            # -r removes the home too
id ytanaka ; groups ytanaka
sudo groupadd developers
```

| File | Holds |
|---|---|
| `/etc/passwd` | `name:x:UID:GID:comment:home:shell` |
| `/etc/shadow` | password hashes and ageing (root only) |
| `/etc/group` | groups and members |
| `/etc/sudoers` | who may use sudo; edit only with **`visudo`** (drop-ins in `/etc/sudoers.d/`) |
| `/etc/skel` | files copied into new homes |

UID 0 is root; system accounts usually have UIDs below 1000; normal users start at 1000. A shell of `/usr/sbin/nologin` blocks interactive logins for service accounts.

## Packages

| Family | Low level | High level | Package |
|---|---|---|---|
| Debian, Ubuntu | `dpkg` | `apt` | `.deb` |
| Red Hat, Fedora, Rocky, Alma | `rpm` | `dnf` (older: `yum`) | `.rpm` |
| SUSE | `rpm` | `zypper` | `.rpm` |

```bash,norun
sudo apt update && sudo apt install nginx      sudo dnf install nginx
apt search / dnf search ; apt show / dnf info
dpkg -l | grep nginx      rpm -qa | grep nginx      rpm -qf /usr/sbin/nginx   # which package owns a file
```

Repositories live in `/etc/apt/sources.list.d/` and `/etc/yum.repos.d/`, signed with GPG keys. Also: **snap**, **flatpak**, **AppImage**, and building from source (`./configure && make && sudo make install`).

## systemd services

```bash,norun
systemctl status nginx
sudo systemctl enable --now nginx      # start now and at boot
sudo systemctl restart / reload / stop nginx
sudo systemctl mask cups               # make it impossible to start
systemctl list-units --type=service --state=failed
sudo systemctl daemon-reload           # after editing unit files
sudo systemctl edit nginx              # override file in /etc/systemd/system/nginx.service.d/
```

A minimal unit, `/etc/systemd/system/backup.service`, plus a **timer** that replaces cron:

```ini
[Unit]
Description=Nightly backup

[Service]
Type=oneshot
ExecStart=/usr/local/bin/backup.sh
```

```ini
[Unit]
Description=Run backup at 02:00

[Timer]
OnCalendar=*-*-* 02:00:00
Persistent=true

[Install]
WantedBy=timers.target
```

## Logs

```bash,norun
journalctl -u nginx --since "1 hour ago"     # one service
journalctl -p err -b                         # errors since this boot
journalctl -f                                # follow
tail -f /var/log/syslog   (Ubuntu)   /var/log/messages   (Red Hat)
/var/log/auth.log (Ubuntu) or /var/log/secure (Red Hat)   # logins and sudo
```

**rsyslog** forwards logs to a central server; **logrotate** (`/etc/logrotate.d/`) compresses and deletes old logs.

@@quiz
`sudo usermod -G docker ana` was run, and now Ana has lost her membership of the sudo group. What went wrong?
* -G without -a replaces all her supplementary groups; it should have been usermod -aG docker ana
- Docker removes sudo rights
- usermod always resets groups
- The sudoers file was deleted
? Always use -aG to append.

@@quiz
How should /etc/sudoers be edited?
* With visudo, which checks the syntax before saving
- With any text editor as root
- With usermod
- It can't be edited
? A syntax error in sudoers can lock everyone out of sudo.

@@quiz
Which command finds which RPM package installed /usr/sbin/sshd?
* rpm -qf /usr/sbin/sshd
- rpm -qa sshd
- dnf search /usr/sbin/sshd
- dpkg -l sshd
? -q query, -f file. On Debian: `dpkg -S /usr/sbin/sshd`.

@@quiz
A service should start now and also at every boot. Which command?
* systemctl enable --now service
- systemctl start service
- systemctl mask service
- systemctl daemon-reload
? enable = at boot; --now also starts it.

@@quiz
Where would you look for failed SSH logins on Ubuntu?
* /var/log/auth.log (or journalctl -u ssh)
- /var/log/dpkg.log
- /etc/ssh/sshd_config
- /var/spool/cron
? On Red Hat it's /var/log/secure.

@@flashcards
useradd -m -s :: Create a user with a home and shell
usermod -aG group user :: Add a user to a group (append!)
/etc/passwd :: Accounts: name:x:UID:GID:comment:home:shell
/etc/shadow :: Password hashes (root-readable only)
visudo :: Safely edit sudoers
/etc/skel :: Template files for new home directories
nologin shell :: Blocks interactive logins for service accounts
apt / dnf / zypper :: Package managers: Debian / Red Hat / SUSE
dpkg -S / rpm -qf :: Which package owns a file
systemctl enable --now :: Start now and at boot
systemctl mask :: Prevent a unit from starting at all
daemon-reload :: Reload systemd after changing unit files
systemd timer :: Scheduled unit; replaces cron (OnCalendar=)
journalctl -u unit :: Logs of one service
logrotate :: Rotates and compresses old logs

@@exercise Real users
Given these `/etc/passwd` lines, print the **name and home** of every *human* account: **UID 1000 or higher** whose shell is **not** `nologin` or `false`. Use `awk -F:`.
@@starter
passwd='root:x:0:0:root:/root:/bin/bash
daemon:x:1:1:daemon:/usr/sbin:/usr/sbin/nologin
sshd:x:110:65534::/run/sshd:/usr/sbin/nologin
ytanaka:x:1000:1000:Yuki Tanaka:/home/ytanaka:/bin/bash
msato:x:1001:1001:Mei Sato:/home/msato:/bin/zsh
backup_svc:x:1002:1002::/var/backups:/usr/sbin/nologin
nobody:x:65534:65534:nobody:/nonexistent:/usr/sbin/nologin
asilva:x:1003:1003:Ana Silva:/home/asilva:/bin/bash
olduser:x:1004:1004::/home/olduser:/bin/false'

echo "$passwd" | awk -F: '# your condition and print here'
@@expected
ytanaka /home/ytanaka
msato /home/msato
asilva /home/asilva
@@requires
awk -F:
@@hint
`$3 >= 1000 && $7 !~ /(nologin|false)$/ { print $1, $6 }`. Field 3 is the UID, 6 the home, 7 the shell.
@@solution
passwd='root:x:0:0:root:/root:/bin/bash
daemon:x:1:1:daemon:/usr/sbin:/usr/sbin/nologin
sshd:x:110:65534::/run/sshd:/usr/sbin/nologin
ytanaka:x:1000:1000:Yuki Tanaka:/home/ytanaka:/bin/bash
msato:x:1001:1001:Mei Sato:/home/msato:/bin/zsh
backup_svc:x:1002:1002::/var/backups:/usr/sbin/nologin
nobody:x:65534:65534:nobody:/nonexistent:/usr/sbin/nologin
asilva:x:1003:1003:Ana Silva:/home/asilva:/bin/bash
olduser:x:1004:1004::/home/olduser:/bin/false'

echo "$passwd" | awk -F: '$3 >= 1000 && $7 !~ /(nologin|false)$/ { print $1, $6 }'

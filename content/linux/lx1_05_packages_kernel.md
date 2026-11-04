@@lesson lx1-packages Package Management, Repositories and the Kernel
@@lang bash
@@text
Installing, updating and removing software safely is daily work, and Linux+ covers both Debian-family and Red Hat-family tools. This lesson also covers kernel modules, kernel parameters with sysctl, GRUB settings and building software from source.

## Two families

| Task | Debian/Ubuntu | RHEL/Fedora/Rocky/Alma | SUSE |
|---|---|---|---|
| Low-level package tool | `dpkg` (.deb) | `rpm` (.rpm) | `rpm` |
| High-level (resolves dependencies) | `apt` | `dnf` (older: `yum`) | `zypper` |
| Refresh metadata | `apt update` | `dnf check-update` / `makecache` | `zypper refresh` |
| Upgrade all | `apt upgrade` (`full-upgrade` may remove packages) | `dnf upgrade` | `zypper update` |
| Install | `apt install nginx` | `dnf install nginx` | `zypper install nginx` |
| Remove | `apt remove` / `purge` (also config) | `dnf remove` | `zypper remove` |
| Search | `apt search`, `apt show` | `dnf search`, `dnf info` | `zypper search` |
| Which package owns a file | `dpkg -S /usr/bin/ssh` | `rpm -qf /usr/bin/ssh` | `rpm -qf` |
| List installed | `dpkg -l`, `apt list --installed` | `rpm -qa`, `dnf list installed` | `rpm -qa` |
| List a package's files | `dpkg -L openssh-client` | `rpm -ql openssh-clients` | |
| Verify files against the package | `debsums` | `rpm -V` | |
| History / rollback | `/var/log/apt/history.log` | `dnf history`, `dnf history undo 12` | |

## Repositories

- Debian: `/etc/apt/sources.list` and `/etc/apt/sources.list.d/*.list` or `.sources`; signing keys in `/etc/apt/keyrings/` referenced with `signed-by=`.
- RHEL: `/etc/yum.repos.d/*.repo` with `baseurl`, `enabled`, `gpgcheck=1`, `gpgkey`.
- **GPG signatures** prove packages come from the repository owner. Never disable `gpgcheck` to "make it work".
- Third-party repos: EPEL (RHEL extras), PPAs (Ubuntu). Each adds trust in another publisher.
- **Pinning/version locks**: `apt-mark hold pkg`, `dnf versionlock add pkg`.

## Other software sources

- **Snap**, **Flatpak**, **AppImage**: self-contained app formats.
- **Language package managers**: `pip`, `npm`, `cargo`, `gem` (prefer virtual environments; don't `sudo pip install` into the system Python).
- **Building from source**: `./configure`, `make`, `sudo make install` (installs to `/usr/local`, untracked by the package manager), or `cmake`. Requires build tools (`build-essential`, `gcc`, `make`, headers).
- Shared libraries: `ldd /usr/bin/ssh` lists them; `ldconfig` updates the cache; `LD_LIBRARY_PATH` adds search paths.

## Kernel modules

```bash,norun
lsmod                           # loaded modules
modinfo e1000e                  # details and parameters
modprobe br_netfilter           # load with dependencies (insmod: no dependency handling)
modprobe -r pcspkr              # unload
echo "br_netfilter" > /etc/modules-load.d/k8s.conf   # load at boot
echo "blacklist nouveau" > /etc/modprobe.d/blacklist-nouveau.conf
```

## Kernel parameters with sysctl

```bash,norun
sysctl net.ipv4.ip_forward              # read (also /proc/sys/net/ipv4/ip_forward)
sysctl -w net.ipv4.ip_forward=1         # set now (lost at reboot)
echo "net.ipv4.ip_forward = 1" > /etc/sysctl.d/90-router.conf
sysctl --system                         # reload all files
```

Common ones: `net.ipv4.ip_forward` (routing), `vm.swappiness` (0–100: how eagerly to swap), `fs.file-max`, `net.core.somaxconn`, `kernel.randomize_va_space` (ASLR).

## GRUB2

Edit `/etc/default/grub` (e.g. `GRUB_TIMEOUT=5`, `GRUB_CMDLINE_LINUX="... quiet"`), then regenerate: `update-grub` (Debian) or `grub2-mkconfig -o /boot/grub2/grub.cfg` (RHEL). Never edit `grub.cfg` directly. `grubby` on RHEL changes kernel arguments. `uname -r` shows the running kernel; old kernels can be removed once a new one boots fine (`apt autoremove`, `dnf remove --oldinstallonly`).

## Updating safely

1. Read the changelog/security advisory.
2. Snapshot or back up (VM snapshot, LVM snapshot).
3. Update a test system first.
4. Update in a maintenance window; `needrestart` / `dnf needs-restarting -r` tells you if a reboot is required (new kernel or core libraries).
5. Automate security updates: `unattended-upgrades` (Debian/Ubuntu), `dnf-automatic` (RHEL).

@@quiz
Which command shows which RPM package installed `/usr/sbin/sshd`?
- `rpm -qa sshd`
* `rpm -qf /usr/sbin/sshd`
- `dnf search sshd`
- `dpkg -S /usr/sbin/sshd`
? `dpkg -S` is the Debian equivalent.

@@quiz
A package install fails GPG verification. What should you do?
- Set `gpgcheck=0` and retry
* Investigate: import the correct signing key from the vendor or check the repository isn't compromised
- Use `rpm --nodeps`
- Download the package from a forum instead
? Signatures prove origin; don't disable them.

@@quiz
Which command loads a kernel module along with its dependencies?
- insmod
* modprobe
- lsmod
- depmod -a
? insmod doesn't resolve dependencies.

@@quiz
You changed `/etc/default/grub` on Ubuntu. What must you run for it to take effect?
- `grub-install /dev/sda`
* `update-grub`
- `sysctl --system`
- `dracut -f`
? On RHEL: `grub2-mkconfig -o /boot/grub2/grub.cfg`.

@@quiz
How do you make `net.ipv4.ip_forward=1` survive a reboot?
- `sysctl -w net.ipv4.ip_forward=1`
* Put it in a file in `/etc/sysctl.d/` and run `sysctl --system`
- `echo 1 > /proc/sys/net/ipv4/ip_forward`
- Add it to `/etc/fstab`
? `sysctl -w` and `/proc` changes are lost at reboot.

@@flashcards
dpkg / rpm :: low-level package tools
apt / dnf / zypper :: dependency-resolving package managers
apt purge :: remove package and config
dpkg -S / rpm -qf :: which package owns a file
dpkg -L / rpm -ql :: files in a package
rpm -V :: verify installed files
dnf history undo :: roll back a transaction
apt-mark hold :: pin a package version
gpgcheck=1 :: require signed packages
/etc/yum.repos.d :: RHEL repository files
make install :: installs to /usr/local, untracked
ldd :: shared libraries a binary uses
lsmod / modprobe / modinfo :: list / load / describe modules
/etc/modprobe.d blacklist :: stop a module loading
sysctl -w :: set kernel parameter now
/etc/sysctl.d :: persistent kernel parameters
update-grub / grub2-mkconfig :: regenerate GRUB config
uname -r :: running kernel version

@@drill
Which Debian command lists the files installed by a package?
= dpkg -L
? `rpm -ql` on RHEL.

@@drill
Which command lists loaded kernel modules?
= lsmod
? Reads /proc/modules.

@@exercise Pending security updates report
`apt list --upgradable` output is in the variable `upgradable`. Print each package that comes from a `-security` pocket as `package current -> new`, then the count of security and other upgrades. Lines look like `name/pocket new_version arch [upgradable from: old_version]`.
@@starter
upgradable='Listing...
openssl/noble-security 3.0.13-0ubuntu3.5 amd64 [upgradable from: 3.0.13-0ubuntu3.4]
curl/noble-updates 8.5.0-2ubuntu10.6 amd64 [upgradable from: 8.5.0-2ubuntu10.5]
linux-image-generic/noble-security 6.8.0-49.49 amd64 [upgradable from: 6.8.0-48.48]
vim/noble-updates 2:9.1.0016-1ubuntu7.4 amd64 [upgradable from: 2:9.1.0016-1ubuntu7.3]
sudo/noble-security 1.9.15p5-3ubuntu5.1 amd64 [upgradable from: 1.9.15p5-3ubuntu5]'

echo "$upgradable"
@@expected
openssl 3.0.13-0ubuntu3.4 -> 3.0.13-0ubuntu3.5
linux-image-generic 6.8.0-48.48 -> 6.8.0-49.49
sudo 1.9.15p5-3ubuntu5 -> 1.9.15p5-3ubuntu5.1
security: 3 other: 2
@@requires
-security
NR > 1
@@hint
Split `$1` on `/` with `split($1, p, "/")`. The old version is the last field without the closing `]`: `sub(/\]$/, "", $NF)`.
@@solution
upgradable='Listing...
openssl/noble-security 3.0.13-0ubuntu3.5 amd64 [upgradable from: 3.0.13-0ubuntu3.4]
curl/noble-updates 8.5.0-2ubuntu10.6 amd64 [upgradable from: 8.5.0-2ubuntu10.5]
linux-image-generic/noble-security 6.8.0-49.49 amd64 [upgradable from: 6.8.0-48.48]
vim/noble-updates 2:9.1.0016-1ubuntu7.4 amd64 [upgradable from: 2:9.1.0016-1ubuntu7.3]
sudo/noble-security 1.9.15p5-3ubuntu5.1 amd64 [upgradable from: 1.9.15p5-3ubuntu5]'

echo "$upgradable" | awk 'NR > 1 {
    split($1, p, "/")
    old = $NF
    sub(/\]$/, "", old)
    if (p[2] ~ /-security$/) { print p[1], old, "->", $2; sec++ } else other++
}
END { print "security:", sec, "other:", other }'

@@exercise Audit sysctl hardening settings
Compare the current kernel parameters (`current`, as `key = value` lines like `sysctl -a` prints) with the hardening baseline in `baseline`. Print `OK key`, `FIX key: value -> wanted`, or `MISSING key` for each baseline key, in baseline order, then the number of fixes needed.
@@starter
baseline='net.ipv4.ip_forward = 0
net.ipv4.conf.all.accept_redirects = 0
net.ipv4.conf.all.send_redirects = 0
net.ipv4.tcp_syncookies = 1
kernel.randomize_va_space = 2
kernel.kptr_restrict = 1'
current='kernel.randomize_va_space = 2
net.ipv4.ip_forward = 1
net.ipv4.tcp_syncookies = 1
net.ipv4.conf.all.accept_redirects = 1
net.ipv4.conf.all.send_redirects = 0'

echo "compare here"
@@expected
FIX net.ipv4.ip_forward: 1 -> 0
FIX net.ipv4.conf.all.accept_redirects: 1 -> 0
OK net.ipv4.conf.all.send_redirects
OK net.ipv4.tcp_syncookies
OK kernel.randomize_va_space
MISSING kernel.kptr_restrict
fixes needed: 3
@@requires
declare -A
while IFS=' = ' read -r
@@hint
Load `current` into an associative array: `declare -A cur; while IFS=' = ' read -r k v; do cur[$k]=$v; done <<< "$current"`. Then loop over the baseline the same way.
@@solution
baseline='net.ipv4.ip_forward = 0
net.ipv4.conf.all.accept_redirects = 0
net.ipv4.conf.all.send_redirects = 0
net.ipv4.tcp_syncookies = 1
kernel.randomize_va_space = 2
kernel.kptr_restrict = 1'
current='kernel.randomize_va_space = 2
net.ipv4.ip_forward = 1
net.ipv4.tcp_syncookies = 1
net.ipv4.conf.all.accept_redirects = 1
net.ipv4.conf.all.send_redirects = 0'

declare -A cur
while IFS=' = ' read -r key value; do
    cur[$key]=$value
done <<< "$current"

fixes=0
while IFS=' = ' read -r key wanted; do
    if [[ -z ${cur[$key]+set} ]]; then
        echo "MISSING $key"
        fixes=$((fixes + 1))
    elif [[ ${cur[$key]} != "$wanted" ]]; then
        echo "FIX $key: ${cur[$key]} -> $wanted"
        fixes=$((fixes + 1))
    else
        echo "OK $key"
    fi
done <<< "$baseline"
echo "fixes needed: $fixes"

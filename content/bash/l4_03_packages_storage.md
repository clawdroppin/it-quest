@@lesson l4-system Packages, Disks and Server Hardening
@@text
The system administration side of Linux+ and of any Linux job.

## Package managers

| Task | Debian / Ubuntu | Red Hat / Rocky / Fedora |
|---|---|---|
| refresh the list | `sudo apt update` | (automatic) |
| upgrade everything | `sudo apt upgrade` | `sudo dnf upgrade` |
| install | `sudo apt install nginx` | `sudo dnf install nginx` |
| remove | `sudo apt remove nginx` | `sudo dnf remove nginx` |
| search | `apt search nginx` | `dnf search nginx` |
| which package owns a file | `dpkg -S /usr/bin/curl` | `rpm -qf /usr/bin/curl` |
| package file format | `.deb` | `.rpm` |

Also seen: `snap` and `flatpak` (self-contained apps), `pip` (Python), `npm` (JavaScript). Unattended security updates: `unattended-upgrades` on Ubuntu, `dnf-automatic` on Red Hat.

## Disks, partitions and mounts

```bash,norun
lsblk                      # disks and partitions as a tree
df -h                      # usage per mounted file system
sudo fdisk -l              # partition tables (or: parted -l)
sudo mkfs.ext4 /dev/sdb1   # format a partition (destroys data!)
sudo mount /dev/sdb1 /mnt/data
blkid                      # UUIDs of partitions
```

To mount at every boot, add a line to `/etc/fstab`, using the **UUID** (device names like `/dev/sdb` can change):

```text
UUID=3f1c...  /mnt/data  ext4  defaults,nofail  0  2
```

Test with `sudo mount -a` before rebooting: a broken fstab can stop the server from booting.

**LVM** (Logical Volume Manager) puts a flexible layer between disks and file systems: physical volumes → a volume group → logical volumes that can grow (`lvextend -r -L +20G /dev/vg0/data`). Common on servers and a Linux+ topic.

## Hardening checklist for a new server

1. Update everything: `apt update && apt upgrade`.
2. Create a normal user with sudo; log in with **SSH keys**.
3. In `/etc/ssh/sshd_config`: `PermitRootLogin no`, `PasswordAuthentication no`; then `systemctl reload sshd` (test a second session before closing the first!).
4. Firewall: default deny incoming, allow only SSH and the service ports.
5. **fail2ban** to ban IPs that brute-force SSH.
6. Automatic security updates.
7. Remove services you don't need (`systemctl list-units --type=service`, `ss -tlnp`).
8. Time sync (`timedatectl`, chrony): logs and certificates depend on correct time.
9. Know the mandatory access control system: **SELinux** on Red Hat (`getenforce`, `ls -Z`), **AppArmor** on Ubuntu. Don't just disable them when something is denied; read the denial in the log.
10. Backups, and a tested restore.

@@quiz
Which file makes a disk mount automatically at boot?
* `/etc/fstab`
- `/etc/mtab`
- `/etc/disks.conf`
- `/boot/mount`
? fstab = file systems table. Use UUIDs and test with `mount -a`.

@@quiz
On a Rocky Linux server, how do you install nginx?
- `apt install nginx`
* `dnf install nginx`
- `pacman -S nginx`
- `brew install nginx`
? Red Hat family: dnf (formerly yum), .rpm packages. Debian family: apt, .deb.

@@quiz
After editing `sshd_config` on a remote server, what's the safe way to apply it?
* Reload sshd, then test a **new** SSH session before closing the current one
- Reboot immediately
- Close your session and reconnect
- Nothing, changes apply instantly
? Your existing session stays open, so you can fix a mistake if the new login fails.

@@drill
Which command shows free space on mounted file systems in human-readable units?
= df -h
? du -sh shows the size of a folder instead.

@@flashcards
apt / dnf :: Package managers for Debian/Ubuntu / Red Hat family
lsblk :: Show disks and partitions as a tree
mkfs.ext4 :: Create an ext4 file system (formats!)
/etc/fstab :: Mounts made at boot; use UUIDs; test with mount -a
LVM :: Flexible volumes: PV -> VG -> LV, can grow online
fail2ban :: Bans IPs after repeated failed logins
SELinux / AppArmor :: Mandatory access control on Red Hat / Ubuntu
getenforce :: Show SELinux mode (Enforcing, Permissive, Disabled)

@@exercise Audit sshd_config
Write `audit_sshd` that reads an sshd_config from **stdin** and prints one line per problem: `FAIL PermitRootLogin` if PermitRootLogin isn't `no`, `FAIL PasswordAuthentication` if it isn't `no`, `FAIL Port` if the port is `22` or missing (a non-default port cuts down on noise from bots). Commented lines (`#...`) don't count; a missing setting counts as a failure for the first two too. Print `PASS` if there are no problems.
@@starter
audit_sshd() {
    echo "TODO: not done yet" >&2
    return 1
}

@@tests
good=$'Port 2222\nPermitRootLogin no\nPasswordAuthentication no\nUsePAM yes'
bad=$'#Port 2222\nPermitRootLogin yes\n#PasswordAuthentication no'
check "$(audit_sshd <<< "$good")" "PASS" "a hardened config"
check "$(audit_sshd <<< "$bad")" $'FAIL PermitRootLogin\nFAIL PasswordAuthentication\nFAIL Port' "a default config"
check "$(audit_sshd <<< $'Port 22\nPermitRootLogin no\nPasswordAuthentication no')" "FAIL Port" "port 22"
@@hint
Read the active settings with `local cfg; cfg=$(grep -v '^\s*#')`. Then get one value with `awk '$1 == "PermitRootLogin" {print $2}' <<< "$cfg"`, compare, and count problems.
@@solution
audit_sshd() {
    local cfg root pass port problems=0
    cfg=$(grep -v '^[[:space:]]*#')
    root=$(awk '$1 == "PermitRootLogin" {print $2}' <<< "$cfg")
    pass=$(awk '$1 == "PasswordAuthentication" {print $2}' <<< "$cfg")
    port=$(awk '$1 == "Port" {print $2}' <<< "$cfg")
    if [[ $root != "no" ]]; then echo "FAIL PermitRootLogin"; (( problems++ )); fi
    if [[ $pass != "no" ]]; then echo "FAIL PasswordAuthentication"; (( problems++ )); fi
    if [[ -z $port || $port == 22 ]]; then echo "FAIL Port"; (( problems++ )); fi
    if (( problems == 0 )); then echo "PASS"; fi
}

@@exercise Parse lsblk
`lsblk_output` imitates `lsblk -o NAME,SIZE,TYPE,MOUNTPOINT -rn` (raw, no header). Print the partitions (`TYPE` is `part`) that have **no mount point**, as `/dev/<name> <size>`.
@@starter
lsblk_output='sda 100G disk
sda1 1G part /boot
sda2 99G part /
sdb 500G disk
sdb1 500G part
sdc 20G disk
sdc1 10G part /mnt/data
sdc2 10G part'

@@expected
/dev/sdb1 500G
/dev/sdc2 10G
@@hint
`awk '$3 == "part" && $4 == "" {print "/dev/" $1, $2}'`
@@solution
lsblk_output='sda 100G disk
sda1 1G part /boot
sda2 99G part /
sdb 500G disk
sdb1 500G part
sdc 20G disk
sdc1 10G part /mnt/data
sdc2 10G part'
echo "$lsblk_output" | awk '$3 == "part" && $4 == "" {print "/dev/" $1, $2}'

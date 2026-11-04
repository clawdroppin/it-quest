@@lesson lx1-storageops Storage Operations: Space, fstab, LVM Changes and File System Repair
@@lang bash
@@text
The storage lesson covered partitions, file systems and LVM concepts. This one is the operational side: finding what fills a disk, writing a correct `/etc/fstab`, growing a logical volume online, checking file systems, and the inode trap that makes a "half-empty" disk refuse new files.

## How full is it, and what's filling it?

```bash,norun
df -h                         # space per mounted file system
df -i                         # inodes per file system (100% = no new files, even with free space)
du -sh /var/* 2>/dev/null | sort -h | tail       # biggest directories
du -xh / --max-depth=1        # -x: stay on this file system
find / -xdev -type f -size +1G -exec ls -lh {} +
lsof +L1                      # deleted files still held open (space not freed until the process closes them)
ncdu /                        # interactive (if installed)
```

Classic causes: logs without rotation (`logrotate`), old kernels in `/boot`, container images (`docker system df`, `podman system prune`), core dumps, package caches (`apt clean`, `dnf clean all`), a deleted-but-open log file (restart the service or truncate via `/proc/PID/fd/N`).

## Block devices

`lsblk -f` (tree with file systems and UUIDs), `blkid` (UUIDs and types), `fdisk -l` / `parted -l` / `gdisk` (partition tables), `wipefs`, `partprobe` (reread partition table).

## /etc/fstab

Six fields:

```text
# device                                  mountpoint  type  options                 dump fsck
UUID=3f1c9a1e-5b2d-4c7e-9b1a-0d2e3f4a5b6c  /           ext4  defaults                0    1
UUID=8a7b6c5d-4e3f-2a1b-0c9d-8e7f6a5b4c3d  /boot/efi   vfat  umask=0077              0    2
/dev/vg_data/lv_www                       /var/www    xfs   defaults,noatime        0    2
//fs01/share                              /mnt/share  cifs  credentials=/root/.smb,_netdev  0  0
192.168.1.20:/export/home                 /home       nfs   defaults,_netdev        0    0
/swapfile                                 none        swap  sw                      0    0
```

- Use **UUIDs** or LVM paths, not `/dev/sdb1` (names can change between boots).
- Field 6: `1` for root, `2` for other local file systems, `0` to skip fsck (network and swap).
- Options: `defaults` (rw, suid, dev, exec, auto, nouser, async), `ro`, `noexec`, `nosuid`, `nodev` (harden `/tmp`, `/var/tmp`, `/dev/shm`), `noatime`, `_netdev` (wait for the network), `nofail` (don't block boot if missing), `x-systemd.automount`.
- **Test before rebooting**: `mount -a` (mounts everything not yet mounted) and `findmnt --verify`. A typo in fstab can drop the server into emergency mode.

## Growing storage with LVM (online)

```bash,norun
pvcreate /dev/sdc                         # new disk becomes a physical volume
vgextend vg_data /dev/sdc                 # add it to the volume group
lvextend -r -L +50G /dev/vg_data/lv_www   # grow the LV and (-r) the file system
# or separately: resize2fs /dev/vg_data/lv_www (ext4)  /  xfs_growfs /var/www (XFS)
pvs; vgs; lvs                             # summaries
lvcreate -s -L 5G -n www_snap /dev/vg_data/lv_www    # snapshot before a risky change
```

**XFS can grow but not shrink**; ext4 can shrink offline (`resize2fs` after `umount` and `e2fsck -f`).

## File system checks and repair

- `fsck` / `e2fsck -f /dev/sdb1` (ext family) and `xfs_repair /dev/sdb1` (XFS): run on an **unmounted** file system (or from rescue mode for root).
- `tune2fs -l` (ext4 details, mount counts), `xfs_info`.
- `dmesg`/`journalctl -k` show I/O errors; `smartctl -a /dev/sda` reads disk health (reallocated sectors, pending sectors).

## Swap

`swapon --show`, `free -h`. Add a swap file: `fallocate -l 2G /swapfile && chmod 600 /swapfile && mkswap /swapfile && swapon /swapfile`, then an fstab line.

## Quotas and network storage

User/group quotas (`usrquota,grpquota` mount options, `quotacheck`, `edquota`, `repquota`), XFS project quotas. NFS (`showmount -e server`, `exportfs -ra`, `/etc/exports`) and SMB/CIFS (`mount -t cifs`, `smbclient`). iSCSI (`iscsiadm`) for block storage over IP.

@@quiz
`df -h` shows 40% used on /var, but new files can't be created ("No space left on device"). What should you check?
* Inode usage with `df -i`
- The partition table with `fdisk -l`
- The swap with `free -h`
- The GRUB config
? Millions of tiny files can exhaust inodes.

@@quiz
A large log file was deleted but `df` shows no space freed. Why?
- The file is in the trash
* A running process still has the file open; restart it or check `lsof +L1`
- `df` caches results for an hour
- Deleted files are moved to /tmp
? Space is released when the last handle closes.

@@quiz
Which fstab option stops the boot from failing if a USB backup disk is missing?
- noexec
- _netdev
* nofail
- noatime
? `_netdev` waits for networking for network mounts.

@@quiz
How do you grow a logical volume by 20 GB and its file system in one command?
- `lvcreate -L 20G`
* `lvextend -r -L +20G /dev/vg/lv`
- `vgextend -L +20G vg`
- `resize2fs -L +20G`
? `-r` resizes the file system too.

@@quiz
Which command safely checks your fstab edits before rebooting?
* `mount -a` (and `findmnt --verify`)
- `fsck -A`
- `update-grub`
- `systemctl daemon-reload` only
? A bad fstab can send the server to emergency mode.

@@flashcards
df -h :: space per file system
df -i :: inode usage
du -sh * | sort -h :: biggest directories
lsof +L1 :: deleted files still open
lsblk -f / blkid :: devices, file systems, UUIDs
fstab fields :: device, mount point, type, options, dump, fsck
UUID in fstab :: stable device naming
fsck field :: 1 root, 2 others, 0 skip
nofail :: don't block boot if missing
_netdev :: network mount; wait for network
noexec,nosuid,nodev :: hardening options for /tmp
mount -a :: test fstab
pvcreate / vgextend / lvextend :: grow LVM step by step
lvextend -r :: also resize the file system
XFS :: can grow, can't shrink
xfs_repair / e2fsck :: repair unmounted file systems
smartctl -a :: disk health
mkswap / swapon :: create / enable swap

@@drill
Which df option shows inode usage?
= -i
= df -i
? Inodes run out with many tiny files.

@@drill
Which command grows an XFS file system after its LV is extended?
= xfs_growfs
? ext4 uses resize2fs.

@@exercise Validate an fstab file
Check each non-comment line of `fstab`: it must have exactly 6 fields; the device should use `UUID=`, `/dev/mapper/`, `/dev/vg`, a network source (contains `:` or starts with `//`) or be a swap file; network mounts must include `_netdev`; the fsck field must be `1` only for `/` and `0` for network and swap entries. Print `OK mountpoint` or `WARN mountpoint: reasons`.
@@starter
fstab='# /etc/fstab
UUID=3f1c9a1e-5b2d-4c7e-9b1a-0d2e3f4a5b6c / ext4 defaults 0 1
/dev/sdb1 /data ext4 defaults 0 2
/dev/vg_data/lv_www /var/www xfs defaults,noatime 0 2
192.168.1.20:/export/home /home nfs defaults 0 0
//fs01/share /mnt/share cifs credentials=/root/.smb,_netdev 0 2
UUID=8a7b6c5d-4e3f-2a1b-0c9d-8e7f6a5b4c3d /boot/efi vfat umask=0077 0 1
/swapfile none swap sw 0 0'

echo "$fstab" | grep -v '^#'
@@expected
OK /
WARN /data: unstable device name
OK /var/www
WARN /home: network mount without _netdev
WARN /mnt/share: fsck should be 0
WARN /boot/efi: fsck 1 is only for /
OK none
@@requires
_netdev
NF != 6
@@hint
Use one awk program. Build a `reasons` string per line; `net = ($1 ~ /:/ || $1 ~ /^\/\//)`.
@@solution
fstab='# /etc/fstab
UUID=3f1c9a1e-5b2d-4c7e-9b1a-0d2e3f4a5b6c / ext4 defaults 0 1
/dev/sdb1 /data ext4 defaults 0 2
/dev/vg_data/lv_www /var/www xfs defaults,noatime 0 2
192.168.1.20:/export/home /home nfs defaults 0 0
//fs01/share /mnt/share cifs credentials=/root/.smb,_netdev 0 2
UUID=8a7b6c5d-4e3f-2a1b-0c9d-8e7f6a5b4c3d /boot/efi vfat umask=0077 0 1
/swapfile none swap sw 0 0'

echo "$fstab" | awk '!/^#/ && NF {
    r = ""
    if (NF != 6) { print "WARN " $2 ": expected 6 fields"; next }
    net = ($1 ~ /:/ || $1 ~ /^\/\//)
    swap = ($3 == "swap")
    if (!net && !swap && $1 !~ /^UUID=/ && $1 !~ /^\/dev\/(mapper|vg)/) r = r "unstable device name; "
    if (net && $4 !~ /_netdev/) r = r "network mount without _netdev; "
    if ($6 == 1 && $2 != "/") r = r "fsck 1 is only for /; "
    if ((net || swap) && $6 != 0) r = r "fsck should be 0; "
    if (r == "") print "OK " $2
    else { sub(/; $/, "", r); print "WARN " $2 ": " r }
}'

@@exercise Disk space alert
`df -P` output is in `dfout` (sizes in 1K blocks). For each real file system (skip `tmpfs`, `devtmpfs` and `overlay`), print `CRITICAL` at 90% or more, `WARNING` at 80% or more, otherwise nothing. Show the mount point, the percentage and free space in GiB with one decimal. Finish with `all clear` if nothing was printed.
@@starter
dfout='Filesystem     1024-blocks      Used Available Capacity Mounted on
/dev/sda2        102687672  93290112   4141344      96% /
tmpfs              8133048         0   8133048       0% /dev/shm
/dev/sda1          1046508    322344    724164      31% /boot/efi
/dev/mapper/vg-var 52403200  43515904   8887296      84% /var
overlay          102687672  93290112   4141344      96% /var/lib/docker/overlay2/abc/merged
/dev/sdb1        976285652 312412188 663873464      33% /data'

echo "$dfout"
@@expected
CRITICAL / 96% used, 3.9 GiB free
WARNING /var 84% used, 8.5 GiB free
@@requires
tmpfs|devtmpfs|overlay
/ 1048576
@@hint
`awk 'NR > 1 && $1 !~ /^(tmpfs|devtmpfs|overlay)$/'`; `pct = $5 + 0` turns `96%` into 96; free GiB is `$4 / 1048576`.
@@solution
dfout='Filesystem     1024-blocks      Used Available Capacity Mounted on
/dev/sda2        102687672  93290112   4141344      96% /
tmpfs              8133048         0   8133048       0% /dev/shm
/dev/sda1          1046508    322344    724164      31% /boot/efi
/dev/mapper/vg-var 52403200  43515904   8887296      84% /var
overlay          102687672  93290112   4141344      96% /var/lib/docker/overlay2/abc/merged
/dev/sdb1        976285652 312412188 663873464      33% /data'

echo "$dfout" | awk 'NR > 1 && $1 !~ /^(tmpfs|devtmpfs|overlay)$/ {
    pct = $5 + 0
    level = pct >= 90 ? "CRITICAL" : pct >= 80 ? "WARNING" : ""
    if (level != "") { printf "%s %s %d%% used, %.1f GiB free\n", level, $6, pct, $4 / 1048576; n++ }
}
END { if (!n) print "all clear" }'

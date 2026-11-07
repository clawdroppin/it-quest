@@lesson lx1-storage Storage: Partitions, LVM, File Systems and Mounts
@@lang bash
@@text
## Disks and partitions

```bash,norun
lsblk                          # disks, partitions, LVM volumes and mount points as a tree
sudo fdisk -l                  # partition tables
sudo fdisk /dev/sdb            # interactive (MBR or GPT); gdisk / parted for GPT
sudo parted /dev/sdb mklabel gpt mkpart data ext4 1MiB 100%
blkid                          # UUIDs and file system types
```

Disks are `/dev/sda`, `/dev/sdb` (SATA/SAS/virtual), `/dev/nvme0n1` (NVMe, partitions `nvme0n1p1`), `/dev/vda` (virtio VMs).

## File systems

```bash,norun
sudo mkfs.ext4 /dev/sdb1       # or mkfs.xfs (Red Hat default), mkfs.vfat for EFI
sudo mount /dev/sdb1 /data
df -h ; df -i                  # space ; inodes
du -sh /var/log/*              # what's using space
sudo umount /data              # "target is busy"? lsof +f -- /data, or fuser -m /data
sudo e2fsck -f /dev/sdb1       # check ext4 (unmounted!); xfs_repair for XFS
sudo resize2fs /dev/sdb1       # grow ext4; xfs_growfs /mountpoint for XFS (XFS can't shrink)
```

**Persistent mounts** go in `/etc/fstab`. Use the **UUID**, since `/dev/sdX` names can change:

```text
# device                                    mount  type  options          dump fsck
UUID=3f2b1c9e-8d7a-4e21-9a55-0c3d1b2a4f6e  /data  ext4  defaults,noatime  0    2
```

Test with `sudo mount -a` **before** rebooting: a broken fstab line can drop the server into emergency mode. Use `nofail` for disks that may be missing.

## LVM

**Logical Volume Manager** puts a flexible layer between disks and file systems:

**Physical volumes** (disks/partitions) → a **volume group** (a pool) → **logical volumes** (like partitions you can resize and snapshot).

```bash,norun
sudo pvcreate /dev/sdb /dev/sdc
sudo vgcreate vg_data /dev/sdb /dev/sdc
sudo lvcreate -n lv_app -L 50G vg_data
sudo mkfs.xfs /dev/vg_data/lv_app
sudo lvextend -r -L +20G /dev/vg_data/lv_app    # -r also grows the file system
pvs ; vgs ; lvs                                 # summaries
```

Adding a disk to a full server: `pvcreate` it, `vgextend` the group, `lvextend -r` the volume: no downtime.

## RAID, swap, quotas, network storage

- Software RAID: `mdadm --create /dev/md0 --level=1 --raid-devices=2 /dev/sdb /dev/sdc`; status in `/proc/mdstat`.
- **Swap**: `mkswap /dev/sdd1 && swapon /dev/sdd1`, or a swap file; `free -h`, `swapon --show`.
- **Quotas** limit space per user/group (`quotacheck`, `edquota`, `repquota`; XFS: `xfs_quota`).
- Network file systems: **NFS** (`mount -t nfs server:/export /mnt`), **SMB/CIFS** (`mount -t cifs //server/share /mnt -o credentials=...`). iSCSI provides block storage over the network.

@@quiz
`df -h` shows a file system at 40% used, but no new files can be created and the error is "No space left on device". What should you check?
* Inode usage with df -i (millions of tiny files can exhaust inodes)
- The swap size
- The GRUB configuration
- The kernel version
? Every file needs an inode; running out stops file creation even with free space.

@@quiz
Why should /etc/fstab refer to disks by UUID instead of /dev/sdb1?
* Device names can change between boots when disks are added or reordered; UUIDs don't
- UUIDs make the disk faster
- fstab doesn't support /dev names
- UUIDs encrypt the file system
? Find them with blkid.

@@quiz
You edited /etc/fstab. What should you run before rebooting?
* sudo mount -a, to test the new entries
- sudo reboot -f
- sudo mkfs on every disk
- sudo update-grub
? A mistake in fstab can stop the system from booting normally.

@@quiz
A logical volume is full. A new disk /dev/sdd was added. What's the correct order?
* pvcreate /dev/sdd, vgextend the volume group, lvextend -r the logical volume
- mkfs /dev/sdd, mount it over the old volume
- lvcreate, then pvcreate, then vgextend
- fdisk the old disk to make it bigger
? Disk → PV → add to VG → grow the LV and file system.

@@quiz
You need to shrink a file system. Which can't be shrunk?
* XFS
- ext4 (offline)
- A logical volume itself
- A swap file
? XFS can only grow. Plan sizes carefully, or use ext4 where you might shrink.

@@quiz
umount /data fails with "target is busy". How do you find what's using it?
* lsof or fuser on the mount point
- df -h
- blkid
- mkfs
? Often a shell sitting in that folder, or a running service.

@@flashcards
lsblk :: Tree of block devices and mount points
blkid :: Shows UUIDs and file system types
mkfs.ext4 / mkfs.xfs :: Create a file system
/etc/fstab :: Persistent mounts: device, mount point, type, options, dump, fsck
mount -a :: Mount everything in fstab (test it!)
df -h / df -i :: Free space / free inodes
du -sh :: Size of a folder
LVM :: PV → VG → LV
lvextend -r :: Grow a logical volume and its file system
xfs_growfs :: Grow XFS (it can't shrink)
mdadm :: Linux software RAID; status in /proc/mdstat
mkswap / swapon :: Create and enable swap
NFS / CIFS :: Linux / Windows network file shares

@@drill
Which command shows disks and partitions as a tree?
= lsblk
? Add `-f` to see file systems and UUIDs.

@@exercise Plan LVM extensions from vgs output
`vgs` holds the output of `vgs --units g --noheadings` (name, PVs, LVs, snapshots, attributes, size, free) and `requests` holds `volume group, logical volume, GB to add` lines. Work through the requests in order. If the group still has enough free space, print the `lvextend -r -L +NG /dev/<vg>/<lv>` command and take the space away. Otherwise print how many GB are missing and the three steps to add a disk (`pvcreate /dev/NEW`, `vgextend <vg> /dev/NEW`, then the `lvextend`). Finish with each group's free space left.
@@starter
vgs='  vg_data   2   3   0 wz--n- 199.99g  35.00g
  vg_logs   1   1   0 wz--n-  49.99g   4.00g'

requests='vg_data lv_app 20
vg_data lv_db 10
vg_logs lv_var 10
vg_data lv_app 8'

# read the free space per group into an associative array, then process the requests
@@expected
lvextend -r -L +20G /dev/vg_data/lv_app
lvextend -r -L +10G /dev/vg_data/lv_db
# vg_logs/lv_var needs 6G more than vg_logs has free:
pvcreate /dev/NEW
vgextend vg_logs /dev/NEW
lvextend -r -L +10G /dev/vg_logs/lv_var
# vg_data/lv_app needs 3G more than vg_data has free:
pvcreate /dev/NEW
vgextend vg_data /dev/NEW
lvextend -r -L +8G /dev/vg_data/lv_app
vg_data free: 5G
vg_logs free: 4G
@@requires
declare -A
lvextend -r
vgextend
@@hint
Load the free column (strip the `g`, keep whole GB with `${free%%.*}`) into `declare -A free`, then compare each request with `(( ${free[$vg]} >= gb ))`.
@@solution
vgs='  vg_data   2   3   0 wz--n- 199.99g  35.00g
  vg_logs   1   1   0 wz--n-  49.99g   4.00g'

requests='vg_data lv_app 20
vg_data lv_db 10
vg_logs lv_var 10
vg_data lv_app 8'

declare -A free
while read -r name pv lv sn attr size vfree; do
    vfree=${vfree%g}
    free[$name]=${vfree%%.*}
done <<< "$vgs"

while read -r vg lv gb; do
    if (( ${free[$vg]} >= gb )); then
        echo "lvextend -r -L +${gb}G /dev/$vg/$lv"
        free[$vg]=$(( ${free[$vg]} - gb ))
    else
        echo "# $vg/$lv needs $(( gb - ${free[$vg]} ))G more than $vg has free:"
        echo "pvcreate /dev/NEW"
        echo "vgextend $vg /dev/NEW"
        echo "lvextend -r -L +${gb}G /dev/$vg/$lv"
    fi
done <<< "$requests"

for vg in vg_data vg_logs; do
    echo "$vg free: ${free[$vg]}G"
done

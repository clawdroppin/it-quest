@@lesson le0-disks-boot Disks, Partitions and Booting: lsblk, fstab, UEFI and GRUB
@@lang bash
@@text
Installing Linux, writing a live USB and fixing a machine that won't boot all depend on understanding disks. The most dangerous command in this course is `dd` or a partitioning tool pointed at **the wrong disk**, so this lesson starts with how to identify disks reliably.

## Naming disks and partitions

- `/dev/sda`, `/dev/sdb`: SATA and USB disks. `/dev/nvme0n1`: an NVMe SSD, with partitions `/dev/nvme0n1p1`, `p2`... `/dev/mmcblk0`: an SD card.
- Names can change between boots. **UUIDs** (`blkid`) and **labels** don't, which is why `/etc/fstab` uses `UUID=...`.
- `lsblk -o NAME,SIZE,TYPE,RM,TRAN,MOUNTPOINTS,MODEL` lists block devices. `RM 1` means removable, and `TRAN usb` means USB.
- **Before writing to a disk**: check the size and model, unplug other USB drives, and run `lsblk` before and after plugging in the stick. The device that appears is the one.

## Partition tables and file systems

- **GPT** (modern, UEFI) vs **MBR** (legacy BIOS, max 2 TiB, 4 primary partitions).
- A UEFI system needs an **EFI System Partition** (ESP): FAT32, a few hundred MB, mounted at `/boot/efi`. Bootloaders live there as `.efi` files.
- File systems: **ext4** (default, robust), **btrfs** (snapshots, compression: Fedora and openSUSE default), **xfs** (large servers), **FAT32/exFAT** (USB sticks shared with Windows/macOS), **NTFS** (Windows disks, readable with ntfs3).
- **Swap**: a partition or a swap file. Many distros now use **zram** (compressed swap in RAM).

## /etc/fstab

```
# <device>                                  <mountpoint> <type> <options>          <dump> <pass>
UUID=3f2c...-a1                              /            ext4   defaults            0      1
UUID=7A1B-33C0                               /boot/efi    vfat   umask=0077          0      2
UUID=9d0e...-77                              /data        ext4   defaults,nofail     0      2
```

- `pass`: 1 for root, 2 for the others (fsck order), 0 to skip.
- `nofail`: boot continues even if the disk is missing (use it for external disks). A wrong UUID **without** `nofail` drops you into **emergency mode** at boot.
- Test changes with `sudo mount -a` (and `findmnt --verify`) **before** rebooting.

## Booting: UEFI → GRUB → kernel → systemd

1. **Firmware (UEFI)** reads its boot entries (`efibootmgr -v`) and runs a `.efi` from the ESP. Secure Boot checks signatures (shim is signed by Microsoft and loads GRUB).
2. **GRUB** shows the menu and loads the kernel and initramfs. Press **e** to edit a boot entry temporarily (for example, add `nomodeset` for graphics problems, or `systemd.unit=rescue.target`).
3. The **kernel** and **initramfs** find the root file system (by UUID) and start **systemd**.
4. **systemd** mounts fstab entries and starts services. `systemd-analyze blame` shows what's slow.
- After Windows updates break dual boot, boot a live USB and reinstall GRUB, or fix the boot order with `efibootmgr -o`.

@@quiz
Why does /etc/fstab use UUIDs rather than /dev/sdb1?
* Device names can change between boots; UUIDs stay the same
- UUIDs are faster
- /dev names don't work in fstab
- To hide disk names from users
? Find them with blkid.

@@quiz
An external disk listed in fstab is unplugged, and the system drops into emergency mode at boot. What was missing?
- The pass value 1
* The nofail option
- The ESP
- Swap
? nofail lets boot continue without it.

@@quiz
Which partition must a UEFI system have?
- A swap partition
* An EFI System Partition (FAT32)
- A /home partition
- An MBR
? It holds the .efi bootloaders.

@@quiz
Graphics hang at boot after an install. What can you add temporarily in GRUB by pressing e?
* nomodeset to the kernel line
- nofail to fstab
- A new UUID
- quiet splash twice
? Then install the proper driver.

@@quiz
What is the safest way to identify a USB stick before writing an ISO?
* Compare lsblk output before and after plugging it in, and check size, RM and TRAN
- Guess /dev/sda
- Use the largest disk
- Use the first disk in the list
? Writing to the wrong disk destroys it.

@@flashcards
/dev/nvme0n1p2 :: NVMe disk 0, partition 2
lsblk :: list block devices
blkid :: show UUIDs and file system types
RM 1 :: removable device
GPT / MBR :: modern / legacy partition tables
ESP :: FAT32 EFI System Partition, /boot/efi
ext4 :: default Linux file system
btrfs :: snapshots and compression
exFAT :: USB sticks shared with other systems
fstab pass :: 1 root, 2 others, 0 skip
nofail :: boot even if the disk is missing
mount -a :: test fstab before rebooting
efibootmgr :: UEFI boot entries
GRUB e :: edit a boot entry once
initramfs :: early boot image that finds root
systemd-analyze blame :: slow boot units

@@drill
Which mount option lets the system boot even if a disk in fstab is missing?
= nofail
? Use it for external and data disks.

@@drill
Which command shows UUIDs of partitions?
= blkid
= sudo blkid
? lsblk -f also shows them.

@@exercise Find the USB stick safely
The starter prints `lsblk -P` (key="value") output. Print the disks that are safe candidates for writing an ISO: whole disks (TYPE "disk") that are removable (RM "1"), use USB transport, and have no mounted partitions. Print `candidate: /dev/NAME SIZE MODEL`, then for every other disk print `skip: /dev/NAME (reason)` with the first reason that applies, checked in the order not removable, not USB, a partition is mounted. (Note that big external USB hard drives often report RM "0".)
@@starter
lsblk_p() {
    cat <<'EOF'
NAME="nvme0n1" SIZE="953.9G" TYPE="disk" RM="0" TRAN="nvme" MOUNTPOINTS="" MODEL="Samsung SSD 980"
NAME="nvme0n1p1" SIZE="512M" TYPE="part" RM="0" TRAN="" MOUNTPOINTS="/boot/efi" MODEL=""
NAME="nvme0n1p2" SIZE="953.4G" TYPE="part" RM="0" TRAN="" MOUNTPOINTS="/" MODEL=""
NAME="sda" SIZE="1.8T" TYPE="disk" RM="0" TRAN="usb" MOUNTPOINTS="" MODEL="Backup Drive"
NAME="sda1" SIZE="1.8T" TYPE="part" RM="0" TRAN="" MOUNTPOINTS="/media/backup" MODEL=""
NAME="sdb" SIZE="28.9G" TYPE="disk" RM="1" TRAN="usb" MOUNTPOINTS="" MODEL="Cruzer Blade"
NAME="sdb1" SIZE="28.9G" TYPE="part" RM="1" TRAN="" MOUNTPOINTS="" MODEL=""
NAME="sdc" SIZE="57.3G" TYPE="disk" RM="1" TRAN="usb" MOUNTPOINTS="" MODEL="DataTraveler"
NAME="sdc1" SIZE="57.3G" TYPE="part" RM="1" TRAN="" MOUNTPOINTS="/media/photos" MODEL=""
EOF
}

echo "candidate: /dev/??"
@@expected
skip: /dev/nvme0n1 (not removable)
skip: /dev/sda (not removable)
candidate: /dev/sdb 28.9G Cruzer Blade
skip: /dev/sdc (partition sdc1 is mounted)
@@requires
lsblk_p
RM
MOUNTPOINTS
field(
@@hint
Read each line with `while read -r line` and pull fields out with a helper such as `field() { sed -n "s/.*$1=\"\([^\"]*\)\".*/\1/p" <<<"$line"; }`. A partition belongs to a disk when its NAME starts with the disk's name, so collect mounted partitions first, then loop over disks.
@@solution
lsblk_p() {
    cat <<'EOF'
NAME="nvme0n1" SIZE="953.9G" TYPE="disk" RM="0" TRAN="nvme" MOUNTPOINTS="" MODEL="Samsung SSD 980"
NAME="nvme0n1p1" SIZE="512M" TYPE="part" RM="0" TRAN="" MOUNTPOINTS="/boot/efi" MODEL=""
NAME="nvme0n1p2" SIZE="953.4G" TYPE="part" RM="0" TRAN="" MOUNTPOINTS="/" MODEL=""
NAME="sda" SIZE="1.8T" TYPE="disk" RM="0" TRAN="usb" MOUNTPOINTS="" MODEL="Backup Drive"
NAME="sda1" SIZE="1.8T" TYPE="part" RM="0" TRAN="" MOUNTPOINTS="/media/backup" MODEL=""
NAME="sdb" SIZE="28.9G" TYPE="disk" RM="1" TRAN="usb" MOUNTPOINTS="" MODEL="Cruzer Blade"
NAME="sdb1" SIZE="28.9G" TYPE="part" RM="1" TRAN="" MOUNTPOINTS="" MODEL=""
NAME="sdc" SIZE="57.3G" TYPE="disk" RM="1" TRAN="usb" MOUNTPOINTS="" MODEL="DataTraveler"
NAME="sdc1" SIZE="57.3G" TYPE="part" RM="1" TRAN="" MOUNTPOINTS="/media/photos" MODEL=""
EOF
}

field() { sed -n "s/.*$1=\"\([^\"]*\)\".*/\1/p" <<<"$line"; }

mounted=""
while read -r line; do
    [ -n "$(field MOUNTPOINTS)" ] && mounted="$mounted $(field NAME)"
done < <(lsblk_p)

while read -r line; do
    [ "$(field TYPE)" = disk ] || continue
    name=$(field NAME)
    reason=""
    if [ "$(field RM)" != 1 ]; then
        reason="not removable"
    elif [ "$(field TRAN)" != usb ]; then
        reason="not USB"
    else
        for m in $mounted; do
            case $m in "$name"*) reason="partition $m is mounted"; break ;; esac
        done
    fi
    if [ -z "$reason" ]; then
        echo "candidate: /dev/$name $(field SIZE) $(field MODEL)"
    else
        echo "skip: /dev/$name ($reason)"
    fi
done < <(lsblk_p)

@@exercise Check fstab against blkid
The starter prints an fstab and `blkid` output. For each fstab entry that uses `UUID=`, print `ok`, `MISSING (boot continues, nofail)` or `MISSING (emergency mode at boot!)` depending on whether the UUID exists in blkid and whether the options include `nofail`. Also warn when the file system type in fstab differs from blkid's TYPE. Skip comment lines.
@@starter
fstab() {
    cat <<'EOF'
# /etc/fstab
UUID=3f2c9a10-1111-4b1e-9c1a-0a1b2c3d4e5f / ext4 defaults 0 1
UUID=7A1B-33C0 /boot/efi vfat umask=0077 0 2
UUID=9d0e5b22-2222-4c2d-8f3e-77aa88bb99cc /data ext4 defaults 0 2
UUID=55aa66bb-3333-4d4e-9f5a-123456789abc /media/backup ext4 defaults,nofail 0 2
UUID=c0ffee00-4444-4e5f-8a6b-cafe00000001 /games ext4 defaults 0 2
EOF
}
blkid_out() {
    cat <<'EOF'
/dev/nvme0n1p1: UUID="7A1B-33C0" TYPE="vfat" PARTUUID="aa-01"
/dev/nvme0n1p2: UUID="3f2c9a10-1111-4b1e-9c1a-0a1b2c3d4e5f" TYPE="ext4" PARTUUID="aa-02"
/dev/sdb1: UUID="9d0e5b22-2222-4c2d-8f3e-77aa88bb99cc" TYPE="ext4" PARTUUID="bb-01"
/dev/sdc1: UUID="c0ffee00-4444-4e5f-8a6b-cafe00000001" TYPE="btrfs" PARTUUID="cc-01"
EOF
}

echo "ok"
@@expected
/: ok (/dev/nvme0n1p2)
/boot/efi: ok (/dev/nvme0n1p1)
/data: ok (/dev/sdb1)
/media/backup: MISSING (boot continues, nofail)
/games: type mismatch (fstab ext4, disk is btrfs)
@@requires
nofail
blkid_out
grep
@@hint
Loop over `fstab | grep -v '^#'` with `while read -r dev mnt type opts dump pass`. Strip `UUID=` with `${dev#UUID=}`, then find the blkid line with `blkid_out | grep "UUID=\"$uuid\""`. Get its TYPE with sed. `case ",$opts," in *,nofail,*)` checks the options.
@@solution
fstab() {
    cat <<'EOF'
# /etc/fstab
UUID=3f2c9a10-1111-4b1e-9c1a-0a1b2c3d4e5f / ext4 defaults 0 1
UUID=7A1B-33C0 /boot/efi vfat umask=0077 0 2
UUID=9d0e5b22-2222-4c2d-8f3e-77aa88bb99cc /data ext4 defaults 0 2
UUID=55aa66bb-3333-4d4e-9f5a-123456789abc /media/backup ext4 defaults,nofail 0 2
UUID=c0ffee00-4444-4e5f-8a6b-cafe00000001 /games ext4 defaults 0 2
EOF
}
blkid_out() {
    cat <<'EOF'
/dev/nvme0n1p1: UUID="7A1B-33C0" TYPE="vfat" PARTUUID="aa-01"
/dev/nvme0n1p2: UUID="3f2c9a10-1111-4b1e-9c1a-0a1b2c3d4e5f" TYPE="ext4" PARTUUID="aa-02"
/dev/sdb1: UUID="9d0e5b22-2222-4c2d-8f3e-77aa88bb99cc" TYPE="ext4" PARTUUID="bb-01"
/dev/sdc1: UUID="c0ffee00-4444-4e5f-8a6b-cafe00000001" TYPE="btrfs" PARTUUID="cc-01"
EOF
}

fstab | grep -v '^#' | while read -r dev mnt type opts dump pass; do
    case $dev in UUID=*) ;; *) continue ;; esac
    uuid=${dev#UUID=}
    found=$(blkid_out | grep "UUID=\"$uuid\"")
    if [ -z "$found" ]; then
        case ",$opts," in
            *,nofail,*) echo "$mnt: MISSING (boot continues, nofail)" ;;
            *) echo "$mnt: MISSING (emergency mode at boot!)" ;;
        esac
        continue
    fi
    real=$(sed -n 's/.* TYPE="\([^"]*\)".*/\1/p' <<<"$found")
    if [ "$real" != "$type" ]; then
        echo "$mnt: type mismatch (fstab $type, disk is $real)"
    else
        echo "$mnt: ok (${found%%:*})"
    fi
done

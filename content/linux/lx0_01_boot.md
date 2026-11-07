@@lesson lx0-boot Linux+, the Boot Process and the File System
@@lang bash
@@text
**CompTIA Linux+** proves you can administer Linux servers: the operating system of the cloud, containers, and most of the internet. The current exam is **XK0-006** (2025); check comptia.org for the objectives. Its domains cover **system management**, **services and user management**, **security**, **automation, orchestration and scripting**, and **troubleshooting**. Do the **Bash & Linux** track first: Linux+ assumes you're comfortable in the shell.

Red Hat's **RHCSA** (a hands-on exam) is the other big Linux certification; everything here helps for both.

## The boot process

1. **Firmware**: UEFI (or BIOS) runs POST and loads the boot loader from the **EFI system partition** (`/boot/efi`).
2. **GRUB2** shows the menu and loads the **kernel** (`/boot/vmlinuz-*`) and the **initramfs** (a small temporary root file system with the drivers needed to mount the real one).
3. The kernel starts **systemd** as process **1** (PID 1).
4. systemd starts units until it reaches the default **target**.

```bash,norun
systemctl get-default                     # graphical.target or multi-user.target
sudo systemctl set-default multi-user.target
sudo systemctl isolate rescue.target      # switch now (single-user maintenance)
grub2-mkconfig -o /boot/grub2/grub.cfg    # Red Hat; on Ubuntu: sudo update-grub
dracut -f   /   update-initramfs -u       # rebuild the initramfs
```

Edit GRUB defaults in `/etc/default/grub`, then regenerate the config. To reset a lost root password, interrupt GRUB, edit the kernel line and add `rd.break` (Red Hat) or `init=/bin/bash`.

| Old SysV runlevel | systemd target |
|---|---|
| 0 | poweroff.target |
| 1 | rescue.target |
| 3 | multi-user.target (text, network) |
| 5 | graphical.target |
| 6 | reboot.target |

## Kernel modules

```bash,norun
lsmod                         # loaded modules
modinfo e1000e                # details
sudo modprobe vfat            # load (with dependencies); modprobe -r to remove
echo "blacklist nouveau" | sudo tee /etc/modprobe.d/blacklist-nouveau.conf
uname -r                      # running kernel version
dmesg | tail                  # kernel messages: hardware, drivers
```

Kernel parameters live in `/proc/sys`, set with `sysctl` (`sysctl net.ipv4.ip_forward=1`, persist in `/etc/sysctl.d/*.conf`).

## The file system hierarchy (FHS)

| Path | Holds |
|---|---|
| `/etc` | configuration files |
| `/var` | changing data: `/var/log`, `/var/lib` (databases, containers), `/var/spool` |
| `/home`, `/root` | users' homes; root's home |
| `/usr/bin`, `/usr/sbin`, `/usr/lib` | installed programs and libraries |
| `/opt` | add-on software |
| `/tmp` | temporary files (cleared at boot) |
| `/dev` | device files: `/dev/sda`, `/dev/nvme0n1`, `/dev/null` |
| `/proc`, `/sys` | virtual: live kernel and process information |
| `/boot` | kernel, initramfs, GRUB |
| `/mnt`, `/media` | mount points |

@@quiz
Which is the first process the Linux kernel starts on a modern distribution?
* systemd (PID 1)
- GRUB2
- initramfs
- cron
? GRUB loads the kernel; the kernel starts systemd.

@@quiz
Which systemd target is the equivalent of runlevel 3: multi-user with networking, no GUI?
* multi-user.target
- graphical.target
- rescue.target
- emergency.target
? graphical.target is runlevel 5.

@@quiz
A new storage controller isn't recognized at boot because its driver isn't available early enough. What needs rebuilding?
* The initramfs (dracut or update-initramfs)
- /etc/fstab
- The GRUB theme
- The swap partition
? The initramfs holds the drivers needed to mount the root file system.

@@quiz
Where are system-wide configuration files kept?
* /etc
- /var
- /usr/bin
- /proc
? /var is for data that changes, like logs.

@@quiz
Which command loads a kernel module together with any modules it depends on?
* modprobe
- insmod
- lsmod
- modinfo
? insmod loads a single file without resolving dependencies.

@@flashcards
GRUB2 :: Boot loader; config in /etc/default/grub, then update-grub / grub2-mkconfig
initramfs :: Temporary root FS with drivers to mount the real root
systemd :: PID 1: starts and manages services (units)
multi-user.target :: Text mode with networking (runlevel 3)
graphical.target :: GUI (runlevel 5)
rescue.target :: Single-user maintenance (runlevel 1)
lsmod / modprobe / modinfo :: List / load / describe kernel modules
sysctl :: Read and set kernel parameters
dmesg :: Kernel ring buffer messages
/etc :: Configuration
/var/log :: Log files
/proc, /sys :: Virtual kernel and process information
/dev :: Device files
/opt :: Add-on software

@@drill
Which command shows the running kernel's version?
= uname -r
? `uname -a` shows everything.

@@exercise Translate runlevel commands to systemctl
Old runbooks still use SysV runlevels. The variable `runbook` holds one command per line. Translate each with the lesson's table (0 poweroff, 1 rescue, 3 multi-user, 5 graphical, 6 reboot):

- `init N` or `telinit N` → `systemctl isolate <target>` (for 0 and 6, `systemctl poweroff` / `systemctl reboot`)
- an inittab line `id:N:initdefault:` → `systemctl set-default <target>`
- `runlevel` → `systemctl get-default`
- anything else → `# no translation: <line>`

Print `old  =>  new` for each line.
@@starter
runbook='init 3
telinit 1
id:5:initdefault:
runlevel
init 6
telinit 0
init 4
service nginx restart'

# translate each line of "$runbook"
@@expected
init 3  =>  systemctl isolate multi-user.target
telinit 1  =>  systemctl isolate rescue.target
id:5:initdefault:  =>  systemctl set-default graphical.target
runlevel  =>  systemctl get-default
init 6  =>  systemctl reboot
telinit 0  =>  systemctl poweroff
init 4  =>  # no translation: runlevel 4 is unused on systemd
service nginx restart  =>  # no translation: service nginx restart
@@requires
case
isolate
set-default
@@hint
A function with a `case "$1" in 0) ... ;; esac` maps the number to a target. Match the line shapes with `case "$line" in "init "*|"telinit "*) ...`, and take the number with `${line##* }`.
@@solution
runbook='init 3
telinit 1
id:5:initdefault:
runlevel
init 6
telinit 0
init 4
service nginx restart'

target_for() {
    case "$1" in
        0) echo poweroff.target ;;
        1) echo rescue.target ;;
        3) echo multi-user.target ;;
        5) echo graphical.target ;;
        6) echo reboot.target ;;
        *) echo "" ;;
    esac
}

while IFS= read -r line; do
    case "$line" in
        "init "*|"telinit "*)
            n=${line##* }
            t=$(target_for "$n")
            if [ "$n" = 0 ]; then new="systemctl poweroff"
            elif [ "$n" = 6 ]; then new="systemctl reboot"
            elif [ -n "$t" ]; then new="systemctl isolate $t"
            else new="# no translation: runlevel $n is unused on systemd"
            fi ;;
        id:*:initdefault:)
            n=$(echo "$line" | cut -d: -f2)
            new="systemctl set-default $(target_for "$n")" ;;
        runlevel) new="systemctl get-default" ;;
        *) new="# no translation: $line" ;;
    esac
    echo "$line  =>  $new"
done <<< "$runbook"

@@exercise Where does it belong in the FHS?
Write a function `fhs_dir` that prints which FHS directory a file belongs in, from a short description, using the lesson's table:

| Clue | Directory |
|---|---|
| `config` | /etc |
| `log` | /var/log |
| `database` or `container data` | /var/lib |
| `mail queue` or `print queue` | /var/spool |
| `add-on` or `vendor app` | /opt |
| `installed program` | /usr/bin |
| `library` | /usr/lib |
| `temporary` | /tmp (cleared at boot) |
| `device` | /dev |
| `kernel` | /boot |

Check the clues in this order and print `ask: not in the table` when nothing matches.
@@starter
files='nginx config file
web server log
PostgreSQL database files
vendor app installed from a .run file
temporary download
installed program from a package
outgoing mail queue
shared library for OpenSSL
the second NVMe disk device
kernel image and initramfs
my holiday photos'

fhs_dir() {
    : # your code here
}

while IFS= read -r f; do
    echo "$f -> $(fhs_dir "$f")"
done <<< "$files"
@@expected
nginx config file -> /etc
web server log -> /var/log
PostgreSQL database files -> /var/lib
vendor app installed from a .run file -> /opt
temporary download -> /tmp (cleared at boot)
installed program from a package -> /usr/bin
outgoing mail queue -> /var/spool
shared library for OpenSSL -> /usr/lib
the second NVMe disk device -> /dev
kernel image and initramfs -> /boot
my holiday photos -> ask: not in the table
@@requires
case
/var/lib
@@hint
A `case "$1" in *config*) echo /etc ;; ... esac` checks patterns in order and stops at the first match. Use `*database*|*"container data"*)` for a clue with two options.
@@solution
files='nginx config file
web server log
PostgreSQL database files
vendor app installed from a .run file
temporary download
installed program from a package
outgoing mail queue
shared library for OpenSSL
the second NVMe disk device
kernel image and initramfs
my holiday photos'

fhs_dir() {
    case "$1" in
        *config*) echo /etc ;;
        *log*) echo /var/log ;;
        *database*|*"container data"*) echo /var/lib ;;
        *"mail queue"*|*"print queue"*) echo /var/spool ;;
        *add-on*|*"vendor app"*) echo /opt ;;
        *"installed program"*) echo /usr/bin ;;
        *library*) echo /usr/lib ;;
        *temporary*) echo "/tmp (cleared at boot)" ;;
        *device*) echo /dev ;;
        *kernel*) echo /boot ;;
        *) echo "ask: not in the table" ;;
    esac
}

while IFS= read -r f; do
    echo "$f -> $(fhs_dir "$f")"
done <<< "$files"

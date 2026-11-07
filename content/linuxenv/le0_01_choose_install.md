@@lesson le0-choose-install Choosing and Installing Linux
@@concepts strings, conditions
@@lang bash
@@text
This course is about living in Linux: installing it on your own computer, using the desktop and the terminal every day, installing software, customising it until it feels like yours, keeping it healthy, and fixing it when something breaks. (The **Linux Administration** certification track covers running Linux *servers*; the two fit together.)

The exercises run in Bash. On Windows the app uses Git Bash, which has the same core commands (`ls`, `grep`, `sed`, `awk`, `find`...) you'll use on a real Linux machine, so everything you practise here carries over.

## What "Linux" actually is

**Linux** is the kernel: the part that talks to the hardware. A **distribution** (distro) bundles the kernel with everything else: the GNU tools, a package manager, an init system (almost always **systemd**), a display server, a desktop environment and default apps. That's why "which Linux?" is really "which distro?".

| Family | Package format / manager | Popular distros | Feel |
|---|---|---|---|
| **Debian** | `.deb` / `apt` | Debian, **Ubuntu**, **Linux Mint**, Pop!_OS, Zorin OS, elementary OS | stable, huge community, the most tutorials |
| **Red Hat** | `.rpm` / `dnf` | **Fedora**, RHEL, Rocky Linux, AlmaLinux, Nobara | Fedora: new but polished; RHEL clones: servers and work |
| **Arch** | `pacman` (+ AUR) | Arch, **EndeavourOS**, Manjaro, **CachyOS**, SteamOS | rolling release, newest software, you assemble it |
| **SUSE** | `.rpm` / `zypper` | openSUSE Tumbleweed and Leap | YaST settings tool, Btrfs snapshots by default |
| **Independent** | various | NixOS (declarative), Gentoo (compile everything), Alpine (tiny, containers), Void | for when you know why |

Release models:

- **Fixed / point release**: tested versions frozen for a release, security fixes only, a new version every 6 months to 2 years. Ubuntu LTS (5 years support), Debian stable, Mint, RHEL. Fewer surprises.
- **Rolling**: updates arrive continuously, no "upgrade to version X". Arch, Tumbleweed, CachyOS. Newest drivers and software, occasionally something breaks.
- **Immutable / atomic**: the system is a read-only image updated as a whole, with rollback, apps as Flatpaks. Fedora Silverblue/Kinoite, Bazzite, openSUSE Aeon, SteamOS. Very hard to break.

**Which one first?** Linux Mint (looks familiar to Windows users, very calm), Ubuntu (most guides online assume it), Fedora Workstation (modern GNOME, great hardware support), or Bazzite/CachyOS if gaming is the main goal. You can't really choose wrong: the skills transfer.

## Desktop environments

The **desktop environment** (DE) is everything you see: panels, windows, settings, file manager. Most distros offer several "flavours".

| DE | Style | Notes |
|---|---|---|
| **GNOME** | modern, minimal, keyboard- and touch-friendly, activities overview | Ubuntu and Fedora default; extensions add features |
| **KDE Plasma** | Windows-like by default, endlessly configurable | Kubuntu, Fedora KDE, openSUSE, SteamOS desktop mode |
| **Cinnamon** | traditional start menu and taskbar | Linux Mint's flagship |
| **Xfce** | light and traditional | older hardware |
| **COSMIC** | new, written in Rust, tiling built in | Pop!_OS |
| tiling window managers | **i3**, **Sway**, **Hyprland**: windows tile automatically, keyboard-driven | for power users |

Under the DE is the **display server protocol**: the old **X11** (Xorg) or the modern **Wayland**, which all major DEs now default to. Wayland is smoother and more secure (apps can't read each other's windows); old X11-only apps run through **XWayland**.

## Before installing

1. **Back up** everything you care about. Really.
2. Check the hardware: most PCs work out of the box. NVIDIA graphics need the proprietary driver (or a distro that ships it); very new Wi-Fi chips may need a newer kernel; some laptops' fingerprint readers aren't supported.
3. Download the ISO from the **official site** and **verify the checksum** (`sha256sum file.iso`, compare with the published SHA256SUMS, ideally after checking its GPG signature).
4. Write it to a USB stick with **Ventoy** (copy several ISOs onto one stick), **Fedora Media Writer**, **balenaEtcher** or `dd`.
5. Boot the stick from the firmware boot menu (F12, F11, F8 or Esc at power-on) and try the **live session** first: does Wi-Fi, sound, the touchpad, the screen and sleep all work?

## Installing

- **Firmware**: modern PCs use **UEFI** with **GPT** partition tables. Keep **Secure Boot** on if the distro supports it (Ubuntu, Fedora, Mint, openSUSE do); you may need to enrol a key for NVIDIA drivers (MOK).
- **Partitions** (the installer's "erase disk" option does this for you):
  - **EFI System Partition** (ESP), ~512 MB–1 GB, FAT32, mounted at `/boot/efi` (or `/boot`).
  - **root** `/`: ext4 (simple, robust) or **Btrfs** (snapshots, compression; default on Fedora and openSUSE).
  - optional separate `/home` so you can reinstall without touching your files.
  - **swap**: a swap file or zram is usually set up automatically; needed for hibernation.
- **Encryption**: tick "encrypt the disk" (LUKS) on laptops. You'll type a passphrase at boot.
- **Dual boot with Windows**: shrink the Windows partition **from Windows** (Disk Management) first, disable Windows **Fast Startup** (it leaves the disk locked), suspend BitLocker or have its recovery key, then choose "install alongside" or use the free space. The **GRUB** (or systemd-boot) menu then offers both. Windows uses local time for the hardware clock and Linux UTC: fix it with `timedatectl set-local-rtc 1` if the clock jumps.
- Alternatives that don't touch your disk: a **virtual machine** (VirtualBox, VMware, GNOME Boxes, Hyper-V), or **WSL 2** on Windows for the command line plus Linux GUI apps.

## First hour after installing

```text
sudo apt update && sudo apt upgrade        # Debian/Ubuntu/Mint (or the Update Manager)
sudo dnf upgrade --refresh                 # Fedora
sudo pacman -Syu                           # Arch family
```

Then: install drivers (Ubuntu: **Additional Drivers**; Fedora: enable **RPM Fusion**; Mint: **Driver Manager**), enable the **firewall** (`sudo ufw enable` on Ubuntu/Mint; Fedora has firewalld on), set up **Timeshift** or Btrfs snapshots, enable **Flathub**, and install the apps you need.

`/etc/os-release` tells scripts (and you) what system they're on; the exercise reads it.

@@quiz
What is the difference between Linux and Ubuntu?
- Ubuntu is the paid version of Linux
* Linux is the kernel; Ubuntu is a distribution that bundles the kernel with tools, a desktop and a package manager
- They are two names for the same thing
- Ubuntu is a desktop environment
? Ubuntu, Fedora and Arch are distributions; GNOME and KDE are desktop environments.

@@quiz
Which release model gives you the newest software continuously, with no version upgrades?
- Fixed / LTS
* Rolling release
- Immutable
- Point release
? Arch, openSUSE Tumbleweed and CachyOS are rolling.

@@quiz
Before installing Linux next to Windows, what should you do in Windows?
* Back up, shrink the Windows partition with Disk Management, disable Fast Startup, and have the BitLocker recovery key ready
- Delete the EFI partition
- Turn off UEFI and switch to legacy BIOS
- Uninstall all drivers
? The Linux installer then uses the free space and adds a boot menu.

@@quiz
Why verify the checksum of a downloaded ISO?
- It makes the USB stick faster
* To be sure the file is complete and is exactly what the distro published, not corrupted or tampered with
- The installer refuses to run otherwise
- It activates the license
? Compare `sha256sum` output with the signed SHA256SUMS file.

@@quiz
Which statement about Wayland is true?
- It's a desktop environment
* It's the modern display protocol that replaces X11; old X11 apps run through XWayland
- It only works on Arch
- It's required for gaming
? GNOME, KDE Plasma and COSMIC all default to Wayland.

@@drill
What do you call the graphical shell such as GNOME or KDE Plasma?
= desktop environment
= a desktop environment
= de
? The distribution is the whole system around it.

@@flashcards
kernel vs distribution :: Linux itself vs the complete operating system built on it
apt / dnf / pacman / zypper :: Debian / Fedora / Arch / openSUSE package managers
LTS :: long-term support release (Ubuntu: 5 years)
rolling release :: continuous updates, no version upgrades
immutable distro :: read-only system image with rollback (Silverblue, Bazzite)
desktop environment :: GNOME, KDE Plasma, Cinnamon, Xfce, COSMIC
tiling window manager :: i3, Sway, Hyprland: windows arranged automatically
Wayland vs X11 :: modern vs legacy display protocol
live session :: try Linux from the USB stick without installing
Ventoy :: boot many ISOs from one USB stick
UEFI + GPT :: modern firmware and partition table
ESP :: EFI System Partition, FAT32, holds the boot loaders
LUKS :: Linux full-disk encryption
GRUB :: the usual boot menu for dual boot
Fast Startup :: Windows feature to disable before dual-booting
/etc/os-release :: which distro and version this is

@@exercise Which distro is this?
`/etc/os-release` is a file of `KEY=value` lines, and `ID_LIKE` lists the families a distro is based on. The starter writes three sample files. Write a function `pm` that takes a file path and prints the package manager to use: `apt` for the debian/ubuntu family, `dnf` for fedora/rhel, `pacman` for arch, `zypper` for suse/opensuse, otherwise `unknown`. Check `ID` and `ID_LIKE` together. Then print `NAME: manager` for each file.
@@starter
cd "$(mktemp -d)"
cat > mint <<'EOF'
NAME="Linux Mint"
VERSION="22.1 (Xia)"
ID=linuxmint
ID_LIKE="ubuntu debian"
EOF
cat > fedora <<'EOF'
NAME="Fedora Linux"
VERSION="42 (Workstation Edition)"
ID=fedora
EOF
cat > cachy <<'EOF'
NAME="CachyOS Linux"
ID=cachyos
ID_LIKE=arch
EOF

pm() {
    # Your code here: read ID and ID_LIKE from the file "$1"
    :
}

for f in mint fedora cachy; do
    name=$(grep '^NAME=' "$f" | cut -d= -f2 | tr -d '"')
    echo "$name: $(pm "$f")"
done
@@expected
Linux Mint: apt
Fedora Linux: dnf
CachyOS Linux: pacman
@@requires
case
@@hint
Load the file's variables in a subshell with `( . "$1"; echo "$ID $ID_LIKE" )`, then use `case " $ids " in *" debian "*|*" ubuntu "*) echo apt ;; ... esac`.
@@solution
cd "$(mktemp -d)"
cat > mint <<'EOF'
NAME="Linux Mint"
VERSION="22.1 (Xia)"
ID=linuxmint
ID_LIKE="ubuntu debian"
EOF
cat > fedora <<'EOF'
NAME="Fedora Linux"
VERSION="42 (Workstation Edition)"
ID=fedora
EOF
cat > cachy <<'EOF'
NAME="CachyOS Linux"
ID=cachyos
ID_LIKE=arch
EOF

pm() {
    local ids
    ids=$( . "./$1"; echo "$ID $ID_LIKE" )
    case " $ids " in
        *" debian "*|*" ubuntu "*) echo apt ;;
        *" fedora "*|*" rhel "*) echo dnf ;;
        *" arch "*) echo pacman ;;
        *" suse "*|*" opensuse "*) echo zypper ;;
        *) echo unknown ;;
    esac
}

for f in mint fedora cachy; do
    name=$(grep '^NAME=' "$f" | cut -d= -f2 | tr -d '"')
    echo "$name: $(pm "$f")"
done

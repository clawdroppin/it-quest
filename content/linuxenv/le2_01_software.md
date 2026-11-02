@@lesson le2-software Installing Software: Packages, Flatpak, Snap and AppImage
@@concepts text-processing, conditions, automation
@@lang bash
@@text
On Windows you download installers from websites. On Linux you mostly don't: software comes from **repositories**, signed collections your package manager trusts, and one command (or the Software app) installs, updates and removes everything at once. This lesson covers every way software gets onto a Linux desktop and when to use which.

## System packages

The distro's package manager installs software built for that distro, resolves **dependencies** (libraries it needs) and updates everything together.

| Task | apt (Debian, Ubuntu, Mint) | dnf (Fedora, RHEL) | pacman (Arch family) | zypper (openSUSE) |
|---|---|---|---|---|
| refresh the package list | `sudo apt update` | (automatic) | `sudo pacman -Sy` | `sudo zypper refresh` |
| upgrade everything | `sudo apt upgrade` | `sudo dnf upgrade` | `sudo pacman -Syu` | `sudo zypper dup` / `up` |
| install | `sudo apt install vlc` | `sudo dnf install vlc` | `sudo pacman -S vlc` | `sudo zypper in vlc` |
| remove | `sudo apt remove vlc` | `sudo dnf remove vlc` | `sudo pacman -Rns vlc` | `sudo zypper rm vlc` |
| search | `apt search editor` | `dnf search editor` | `pacman -Ss editor` | `zypper se editor` |
| show info | `apt show vlc` | `dnf info vlc` | `pacman -Si vlc` | `zypper info vlc` |
| which package owns a file | `dpkg -S /usr/bin/vlc` | `rpm -qf /usr/bin/vlc` | `pacman -Qo /usr/bin/vlc` | `rpm -qf` |
| list installed | `apt list --installed` | `dnf list --installed` | `pacman -Q` | `zypper se -i` |
| remove unneeded deps | `sudo apt autoremove` | `sudo dnf autoremove` | `sudo pacman -Rns $(pacman -Qdtq)` | |

Extra repositories: Ubuntu **PPAs** (`add-apt-repository`), Fedora **RPM Fusion** (codecs, NVIDIA) and **COPR**, Arch's **AUR** (community build scripts, installed with a helper such as `yay` or `paru`; read the PKGBUILD, anyone can upload). Only add sources you trust: a repository can install anything as root.

Never mix in random `.deb`/`.rpm` files from websites unless the vendor is trustworthy (Chrome, VS Code, Steam do this properly by adding their own repository).

## Flatpak

**Flatpak** installs desktop apps in a **sandbox** with their own runtime libraries, the same package on every distro, mostly from **Flathub**.

```text
flatpak remote-add --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo
flatpak install flathub org.gimp.GIMP
flatpak run org.gimp.GIMP
flatpak update
flatpak list --app
flatpak uninstall --unused
```

Pros: newest app versions even on an LTS distro, sandboxing, can't break the system. Cons: larger downloads, the sandbox sometimes blocks access to files or devices: manage permissions with **Flatseal** (or KDE's settings). Fedora, Mint, Pop!_OS, openSUSE and immutable distros use Flatpak by default.

## Snap

**Snap** is Canonical's equivalent, built into Ubuntu (`snap install`, `snap refresh`, the Snap Store). It also packages command-line tools and server software, updates automatically, and some Ubuntu apps (Firefox) come as snaps.

## AppImage

An **AppImage** is one file that contains the whole app: download, `chmod +x App.AppImage`, run. Nothing installed, no automatic updates, no sandbox. Fine for portable tools; manage them with **Gear Lever** to get menu entries.

## Other ways

- **Language package managers**: `pipx install` (Python CLI tools, isolated), `cargo install`, `npm -g`: for developer tools, never `sudo pip install`.
- **Containers**: **Distrobox** or Toolbx run another distro's packages inside your desktop ("I need an Arch package on Fedora").
- **From source**: `./configure && make && sudo make install`, last resort; prefer packaging it.

## Which one?

1. The distro's repository for system tools, drivers, the desktop, and command-line programs.
2. Flatpak (Flathub) for desktop apps, especially when you want a newer version.
3. The vendor's official repository or package for proprietary apps.
4. AppImage for portable one-off tools.

## Updates and keeping the system healthy

- Update regularly (weekly at least): security fixes come through updates. GUI: Update Manager (Mint), Software (GNOME), Discover (KDE).
- Read what a big upgrade wants to **remove** before saying yes.
- On rolling distros, read the news before big updates (archlinux.org news), and take a snapshot first.
- **Distro version upgrades**: `sudo do-release-upgrade` (Ubuntu), `sudo dnf system-upgrade` (Fedora), Mint's upgrade tool. Back up first.
- Clean up: `sudo apt autoremove`, `flatpak uninstall --unused`, `sudo journalctl --vacuum-time=2weeks`, `paccache -r` (Arch).

The exercises translate commands between package managers and read upgrade lists the way an update script would.

@@quiz
What does `sudo apt update` do?
- Installs all available upgrades
* Downloads the latest package lists from the repositories, without installing anything
- Updates apt itself only
- Upgrades to the next Ubuntu version
? `apt upgrade` then installs the newer versions.

@@quiz
Why might you install GIMP as a Flatpak on Ubuntu LTS?
* To get the newest GIMP version in a sandbox, without waiting for the next LTS release
- Because apt can't install GIMP
- Flatpaks run faster than native packages
- Flatpak is required for plugins
? LTS repositories keep the version that shipped with the release, plus fixes.

@@quiz
A Flatpak app can't open files on your second disk. What do you use?
- chmod 777 on the disk
* Flatseal (or the DE's app permissions) to grant that folder to the app
- Reinstall it as root
- Switch to X11
? Flatpak apps only see the folders they have permission for.

@@quiz
What is the AUR?
- Arch's official, signed repository
* Community-maintained build scripts for Arch; useful, but you should review what you install
- A Fedora codec repository
- Ubuntu's snap store
? AUR helpers like yay and paru build and install AUR packages.

@@quiz
Which command shows which package installed `/usr/bin/convert` on Fedora?
- dnf search convert
* rpm -qf /usr/bin/convert
- which convert
- dpkg -S convert
? dpkg -S is the Debian equivalent.

@@flashcards
repository :: signed collection of packages your system trusts
dependency :: a library or program a package needs
apt update / apt upgrade :: refresh the lists / install newer versions
dnf upgrade :: refresh and upgrade on Fedora
pacman -Syu :: full upgrade on Arch
pacman -S / -Rns :: install / remove with unused dependencies and configs
dpkg -S / rpm -qf / pacman -Qo :: which package owns this file
PPA / COPR / AUR :: extra repositories for Ubuntu / Fedora / Arch
RPM Fusion :: Fedora repository for codecs and NVIDIA drivers
Flatpak + Flathub :: sandboxed cross-distro desktop apps
Flatseal :: manage Flatpak permissions
Snap :: Canonical's auto-updating packages
AppImage :: one portable executable file, no install
pipx :: install Python command-line tools in isolation
Distrobox :: other distros' packages in a container
autoremove :: delete dependencies nothing needs any more

@@exercise Translate install commands
Write a function `install_cmd` that takes a package manager (`apt`, `dnf`, `pacman`, `zypper`, `flatpak`) and a package name, and prints the install command: `sudo apt install -y NAME`, `sudo dnf install -y NAME`, `sudo pacman -S --noconfirm NAME`, `sudo zypper install -y NAME`, `flatpak install -y flathub NAME` (no sudo). For anything else print `unsupported: MANAGER` to standard error and return 1.
@@starter
install_cmd() {
    # Your code here
    :
}

for pm in apt dnf pacman zypper; do
    install_cmd "$pm" htop
done
install_cmd flatpak org.gimp.GIMP
install_cmd brew htop 2>&1 || echo "exit code $?"
@@expected
sudo apt install -y htop
sudo dnf install -y htop
sudo pacman -S --noconfirm htop
sudo zypper install -y htop
flatpak install -y flathub org.gimp.GIMP
unsupported: brew
exit code 1
@@requires
case
>&2
@@hint
`case "$1" in apt|dnf|zypper) echo "sudo $1 install -y $2" ;; pacman) ... ;; flatpak) ... ;; *) echo "unsupported: $1" >&2; return 1 ;; esac`
@@solution
install_cmd() {
    case "$1" in
        apt|dnf|zypper) echo "sudo $1 install -y $2" ;;
        pacman) echo "sudo pacman -S --noconfirm $2" ;;
        flatpak) echo "flatpak install -y flathub $2" ;;
        *) echo "unsupported: $1" >&2; return 1 ;;
    esac
}

for pm in apt dnf pacman zypper; do
    install_cmd "$pm" htop
done
install_cmd flatpak org.gimp.GIMP
install_cmd brew htop 2>&1 || echo "exit code $?"

@@exercise Read the upgrade list
The starter prints output in the format of `apt list --upgradable`: `name/suite new-version arch [upgradable from: old-version]`, after a `Listing...` header. Print one line per package, `name: old -> new`, sorted by name, then a summary line `N upgrades, K security`. A package is a security update when its suite contains `-security`.
@@starter
upgradable() {
    cat <<'EOF'
Listing... Done
openssl/noble-security 3.0.13-0ubuntu3.6 amd64 [upgradable from: 3.0.13-0ubuntu3.5]
firefox/noble-updates 1:1snap1-0ubuntu5.3 amd64 [upgradable from: 1:1snap1-0ubuntu5.2]
libc6/noble-security 2.39-0ubuntu8.5 amd64 [upgradable from: 2.39-0ubuntu8.4]
gimp/noble-updates 2.10.36-3ubuntu0.1 amd64 [upgradable from: 2.10.36-3]
EOF
}

# Your code here: upgradable | ...
@@expected
firefox: 1:1snap1-0ubuntu5.2 -> 1:1snap1-0ubuntu5.3
gimp: 2.10.36-3 -> 2.10.36-3ubuntu0.1
libc6: 2.39-0ubuntu8.4 -> 2.39-0ubuntu8.5
openssl: 3.0.13-0ubuntu3.5 -> 3.0.13-0ubuntu3.6
4 upgrades, 2 security
@@requires
awk
@@hint
Skip the header with `NR > 1` (or `grep -v '^Listing'`). In awk, `split($1, p, "/")` gives the name and suite; the old version is the last field without its `]`: `old = $NF; sub(/\]$/, "", old)`. Sort the lines, then count with a second command, or print the summary from awk's `END` block after sorting separately.
@@solution
upgradable() {
    cat <<'EOF'
Listing... Done
openssl/noble-security 3.0.13-0ubuntu3.6 amd64 [upgradable from: 3.0.13-0ubuntu3.5]
firefox/noble-updates 1:1snap1-0ubuntu5.3 amd64 [upgradable from: 1:1snap1-0ubuntu5.2]
libc6/noble-security 2.39-0ubuntu8.5 amd64 [upgradable from: 2.39-0ubuntu8.4]
gimp/noble-updates 2.10.36-3ubuntu0.1 amd64 [upgradable from: 2.10.36-3]
EOF
}

upgradable | awk 'NR > 1 { split($1, p, "/"); old = $NF; sub(/\]$/, "", old); print p[1] ": " old " -> " $2 }' | sort
total=$(upgradable | grep -c '/')
security=$(upgradable | grep -c -- '-security')
echo "$total upgrades, $security security"

@@lesson le4-hardware-fix Drivers, Gaming, Backups and Fixing Things
@@concepts processes, text-processing, files
@@lang bash
@@text
Sooner or later something won't work: no sound, a black screen after an update, a full disk, a game that won't start. This lesson covers the hardware side of a Linux desktop, gaming, keeping backups, and a calm method for fixing problems with the system's own logs.

## Know your hardware

```text
inxi -Fxz                 # a complete, shareable summary (install inxi)
lspci -k                  # PCI devices and the driver in use ("Kernel driver in use")
lsusb                     # USB devices
lsblk -f                  # disks, partitions, filesystems, mount points
free -h                   # memory and swap
sensors                   # temperatures and fans (lm-sensors)
uname -r                  # the running kernel version
```

Most drivers are **in the kernel**: plug it in and it works. The exceptions:

- **NVIDIA**: install the proprietary driver (Ubuntu: Additional Drivers or `ubuntu-drivers install`; Fedora: `akmod-nvidia` from RPM Fusion; Arch: `nvidia-open`), or pick a distro that ships it (Pop!_OS, Bazzite, CachyOS). The open-source `nouveau` driver works for the desktop but is slow for games. With Secure Boot, enrol the MOK key when asked at the next boot.
- **AMD and Intel graphics**: open-source **Mesa** drivers in the distro, nothing to install; newer Mesa means faster games.
- **Wi-Fi**: Intel and most Realtek/MediaTek cards work; some Broadcom chips need `broadcom-sta` (`bcmwl-kernel-source`). Very new hardware may need a newer kernel (Ubuntu HWE kernel, or Fedora).
- **Printers**: CUPS with **driverless IPP Everywhere**: most network printers just appear. Otherwise the vendor's package (HP: hplip). Admin page: http://localhost:631.
- **Firmware updates**: `fwupdmgr refresh && fwupdmgr update` updates BIOS, SSDs, docks and more from LVFS on supported machines (GNOME Firmware is the GUI).

Audio runs on **PipeWire** (which replaced PulseAudio and JACK): `pavucontrol` shows every input, output and app; `wpctl status` in the terminal; `systemctl --user restart pipewire wireplumber` fixes most audio glitches.

## Gaming

- **Steam** (from the distro repository or Flathub) with **Proton** runs most Windows games: Settings → Compatibility → enable Steam Play for all titles. Check games on **ProtonDB** for tips and launch options.
- Other stores: **Heroic** (Epic, GOG, Amazon), **Lutris** (everything), **Bottles** (Windows apps in Wine prefixes).
- Helpers: **GameMode** (`gamemoderun %command%`), **MangoHud** (FPS overlay: `mangohud %command%`), **ProtonUp-Qt** (newer Proton-GE builds), **Gamescope** (a micro-compositor for upscaling and HDR).
- Limits: games with kernel-level anti-cheat (some competitive shooters) refuse Linux; check before you switch.
- Controllers: Steam Input supports Xbox, PlayStation and Switch controllers.

## Backups

Two different things, and you want both:

| Tool | Protects | Restores |
|---|---|---|
| **Timeshift**, Btrfs snapshots (Snapper) | the **system**: a bad update, broken driver | roll the system back, even from the boot menu (snapper/grub-btrfs) |
| **Déjà Dup** (GNOME Backups), **Vorta**/BorgBackup, **restic**, **rsync** | your **files**: deleted, overwritten, disk dead, laptop stolen | individual files or everything |

Follow **3-2-1**: three copies, on two kinds of media, one off-site (a NAS at a friend's, cloud storage with encryption such as restic to Backblaze B2). Automate it, and try a restore once. The second exercise picks files for a backup the way `rsync` would.

```text
rsync -aAXv --delete ~/Documents/ /media/aiko/backup/Documents/    # mirror a folder
restic -r /media/aiko/backup/restic backup ~/Documents ~/Pictures  # versioned, deduplicated, encrypted
```

(The trailing `/` on the source matters to rsync: copy the *contents* of the folder.)

## A method for fixing anything

1. **What changed?** An update, a new driver, a setting? Undoing it (or rolling back a snapshot) is often the fastest fix.
2. **Read the error.** Run the app from a terminal to see its messages.
3. **Read the logs** with `journalctl`:

```text
journalctl -b -p err            # errors in this boot
journalctl -b -1 -p warning     # warnings in the previous boot (after a crash)
journalctl -u NetworkManager    # one service
journalctl -f                   # follow live while you reproduce the problem
dmesg -w                        # kernel messages: hardware, USB, drivers
systemctl --failed              # services that failed to start
```

4. **Search** the exact error message with your distro name; the Arch Wiki is excellent for every distro.
5. **Change one thing at a time**, and note what you did.

The first exercise summarizes a journal the way you would when hunting a problem.

## Common problems

| Symptom | Try |
|---|---|
| Black screen or no login after an update | boot an older kernel from the GRUB "Advanced options" menu; Ctrl+Alt+F3 for a text console; reinstall or remove the graphics driver; roll back a snapshot |
| Doesn't boot at all | boot the live USB, `mount` the root partition, `chroot`, then repair (reinstall GRUB, fix `/etc/fstab`); **Boot Repair** on Ubuntu |
| Windows disappeared from the boot menu | `sudo os-prober` enabled in `/etc/default/grub` (`GRUB_DISABLE_OS_PROBER=false`), then `sudo update-grub` / `grub2-mkconfig` |
| Disk full | `df -h`, `du -sh` / ncdu; clean the package cache, old Flatpak runtimes, `journalctl --vacuum-size=200M`, Docker images, Steam shader caches |
| No sound | pavucontrol: right output? muted? restart PipeWire; check `wpctl status` |
| No Wi-Fi | `nmcli device`, `rfkill list` (blocked by a hardware switch?), `lspci -k` for a driver |
| App frozen | `xkill` (X11), the System Monitor, `kill PID`, `pkill -f name`; `kill -9` as a last resort |
| Whole desktop frozen | Ctrl+Alt+F3 to a console and restart the display manager; or the **Magic SysRq** keys REISUB for a clean-ish reboot |
| Slow | `btop`/System Monitor for CPU, memory and disk; `systemd-analyze blame` for slow boots |

A frozen app is a **process**; managing processes is the third exercise.

@@quiz
How do you see which kernel driver a graphics card or Wi-Fi card is using?
- lsusb
* lspci -k
- free -h
- uname -a
? Look for "Kernel driver in use".

@@quiz
After an NVIDIA driver update the desktop shows a black screen. What's a quick way back in?
* Boot an older kernel from GRUB's Advanced options, or switch to a text console with Ctrl+Alt+F3 and remove or reinstall the driver (or roll back a snapshot)
- Reinstall Linux from scratch
- Unplug the monitor for an hour
- Reset the BIOS
? Snapshots (Timeshift, Snapper) make this a one-click fix.

@@quiz
Which command shows the errors from the previous boot, after a crash?
- dmesg -w
* journalctl -b -1 -p err
- systemctl --failed
- lsblk
? -b -1 selects the previous boot; -p err filters by priority.

@@quiz
Does Timeshift protect your photos if the disk dies?
- Yes, it backs up everything
* No: by default it snapshots the system files, usually on the same disk; you also need a file backup to another device, following 3-2-1
- Only if Secure Boot is on
- Only on Btrfs
? System snapshots and file backups solve different problems.

@@quiz
What makes most Windows games run on Linux through Steam?
- WSL
* Proton (based on Wine and DXVK/VKD3D)
- PipeWire
- Flatpak
? ProtonDB shows community reports for each game.

@@flashcards
inxi -Fxz :: full hardware summary, safe to share
lspci -k / lsusb / lsblk -f :: PCI devices and drivers / USB devices / disks and filesystems
Mesa :: open-source AMD and Intel graphics drivers
proprietary NVIDIA driver :: needed for good GPU performance on NVIDIA
MOK :: key enrolment so Secure Boot accepts your own kernel modules
fwupdmgr :: firmware updates from LVFS
PipeWire / pavucontrol :: the audio system / its volume control
Proton :: Steam's compatibility layer for Windows games
ProtonDB :: community reports on how games run
GameMode / MangoHud :: performance tweaks / FPS overlay
Timeshift / Snapper :: system snapshots and rollback
restic / Borg / Déjà Dup :: file backups with versions
3-2-1 :: three copies, two media, one off-site
rsync -a src/ dst/ :: mirror the contents of src into dst
journalctl -b -p err :: this boot's errors
dmesg :: kernel messages
systemctl --failed :: services that didn't start
Ctrl+Alt+F3 :: switch to a text console
chroot from a live USB :: repair a system that doesn't boot
kill / pkill / kill -9 :: ask a process to stop / by name / force

@@exercise Summarize the journal
The starter prints journal lines like `Nov 02 09:14:03 thinkpad unit[pid]: message`. Print how many lines each unit logged that contain `error` or `fail` (any case), as `count unit`, most first, then by unit name; the unit is the fifth field without the `[pid]` and the colon.
@@starter
journal() {
    cat <<'EOF'
Nov 02 09:14:03 thinkpad NetworkManager[812]: <info> device (wlp2s0): state change: activated
Nov 02 09:14:05 thinkpad kernel: iwlwifi 0000:02:00.0: Microcode SW error detected
Nov 02 09:15:11 thinkpad pipewire[1290]: spa.alsa: hw:1: snd_pcm_open failed: Device or resource busy
Nov 02 09:15:12 thinkpad pipewire[1290]: spa.alsa: hw:1: snd_pcm_open failed: Device or resource busy
Nov 02 09:20:40 thinkpad gnome-shell[1544]: Error: extension "dash-to-dock" failed to load
Nov 02 09:21:00 thinkpad NetworkManager[812]: <warn> dhcp4 (wlp2s0): request timed out
Nov 02 09:22:13 thinkpad kernel: usb 3-2: device descriptor read/64, error -71
Nov 02 09:30:00 thinkpad systemd[1]: Started Daily backup.
EOF
}

# Your pipeline here: journal | ...
@@expected
2 kernel
2 pipewire
1 gnome-shell
@@requires
awk
sort
@@hint
`journal | grep -iE 'error|fail' | awk '{ u = $5; sub(/\[.*$/, "", u); sub(/:$/, "", u); print u }' | sort | uniq -c | sort -k1,1nr -k2 | awk '{ print $1, $2 }'`
@@solution
journal() {
    cat <<'EOF'
Nov 02 09:14:03 thinkpad NetworkManager[812]: <info> device (wlp2s0): state change: activated
Nov 02 09:14:05 thinkpad kernel: iwlwifi 0000:02:00.0: Microcode SW error detected
Nov 02 09:15:11 thinkpad pipewire[1290]: spa.alsa: hw:1: snd_pcm_open failed: Device or resource busy
Nov 02 09:15:12 thinkpad pipewire[1290]: spa.alsa: hw:1: snd_pcm_open failed: Device or resource busy
Nov 02 09:20:40 thinkpad gnome-shell[1544]: Error: extension "dash-to-dock" failed to load
Nov 02 09:21:00 thinkpad NetworkManager[812]: <warn> dhcp4 (wlp2s0): request timed out
Nov 02 09:22:13 thinkpad kernel: usb 3-2: device descriptor read/64, error -71
Nov 02 09:30:00 thinkpad systemd[1]: Started Daily backup.
EOF
}

journal | grep -iE 'error|fail' \
    | awk '{ u = $5; sub(/\[.*$/, "", u); sub(/:$/, "", u); print u }' \
    | sort | uniq -c | sort -k1,1nr -k2 | awk '{ print $1, $2 }'

@@exercise What would the backup copy?
Write a function `to_copy SRC DST` that prints, sorted, the relative paths of files in `SRC` that are **missing** from `DST` or **newer** than the copy there (`[ a -nt b ]`), like `rsync` decides. Skip anything inside a `.cache` folder. Then make the backup with `cp` for exactly those files (create folders with `mkdir -p`) and run `to_copy` again: it should print nothing.
@@starter
cd "$(mktemp -d)"
mkdir -p home/Documents home/Pictures home/.cache/thumbs backup/Documents
echo "tax" > home/Documents/tax.pdf
echo "cv" > home/Documents/cv.odt
echo "cat" > home/Pictures/cat.jpg
echo "x" > home/.cache/thumbs/1.png
echo "cv-old" > backup/Documents/cv.odt
echo "tax" > backup/Documents/tax.pdf
touch -d "2026-01-01" backup/Documents/cv.odt
touch -d "2026-06-01" home/Documents/cv.odt
touch -d "2026-09-01" backup/Documents/tax.pdf
touch -d "2026-03-01" home/Documents/tax.pdf

to_copy() {
    # Your code here
    :
}

echo "to copy:"; to_copy home backup
# copy them here

echo "after:"; to_copy home backup
cat backup/Documents/cv.odt
@@expected
to copy:
Documents/cv.odt
Pictures/cat.jpg
after:
cv
@@requires
find
-nt
@@hint
`(cd "$1" && find . -type f -not -path '*/.cache/*') | sed 's#^\./##' | sort | while read -r f; do if [ ! -e "$2/$f" ] || [ "$1/$f" -nt "$2/$f" ]; then echo "$f"; fi; done`. To copy: `to_copy home backup | while read -r f; do mkdir -p "backup/$(dirname "$f")"; cp -p "home/$f" "backup/$f"; done` (`-p` keeps the time).
@@solution
cd "$(mktemp -d)"
mkdir -p home/Documents home/Pictures home/.cache/thumbs backup/Documents
echo "tax" > home/Documents/tax.pdf
echo "cv" > home/Documents/cv.odt
echo "cat" > home/Pictures/cat.jpg
echo "x" > home/.cache/thumbs/1.png
echo "cv-old" > backup/Documents/cv.odt
echo "tax" > backup/Documents/tax.pdf
touch -d "2026-01-01" backup/Documents/cv.odt
touch -d "2026-06-01" home/Documents/cv.odt
touch -d "2026-09-01" backup/Documents/tax.pdf
touch -d "2026-03-01" home/Documents/tax.pdf

to_copy() {
    (cd "$1" && find . -type f -not -path '*/.cache/*') | sed 's#^\./##' | sort | while read -r f; do
        if [ ! -e "$2/$f" ] || [ "$1/$f" -nt "$2/$f" ]; then
            echo "$f"
        fi
    done
}

echo "to copy:"; to_copy home backup
to_copy home backup | while read -r f; do
    mkdir -p "backup/$(dirname "$f")"
    cp -p "home/$f" "backup/$f"
done

echo "after:"; to_copy home backup
cat backup/Documents/cv.odt

@@exercise Find the runaway process
The starter prints `ps`-style output: `PID USER %CPU %MEM COMMAND`. Write a function `hogs` that reads it on standard input and prints the processes using more than 50% CPU **or** more than 20% memory, as `PID COMMAND (cpu% CPU, mem% MEM)`, highest CPU first, skipping the header. Then print the `kill` command you'd run for the top one: `kill PID`.
@@starter
processes() {
    cat <<'EOF'
PID USER %CPU %MEM COMMAND
1544 aiko 3.1 4.2 gnome-shell
2210 aiko 98.7 1.3 python3
2388 aiko 12.0 31.5 firefox
2990 aiko 0.3 0.8 nautilus
3120 aiko 55.2 2.1 steam
EOF
}

hogs() {
    # Your code here
    :
}

processes | hogs
echo "kill $(processes | hogs | head -1 | cut -d' ' -f1)"
@@expected
2210 python3 (98.7% CPU, 1.3% MEM)
3120 steam (55.2% CPU, 2.1% MEM)
2388 firefox (12.0% CPU, 31.5% MEM)
kill 2210
@@requires
awk
sort
@@hint
`awk 'NR > 1 && ($3 > 50 || $4 > 20)' | sort -k3,3nr | awk '{ printf "%s %s (%s%% CPU, %s%% MEM)\n", $1, $5, $3, $4 }'`. In printf, `%%` prints a percent sign.
@@solution
processes() {
    cat <<'EOF'
PID USER %CPU %MEM COMMAND
1544 aiko 3.1 4.2 gnome-shell
2210 aiko 98.7 1.3 python3
2388 aiko 12.0 31.5 firefox
2990 aiko 0.3 0.8 nautilus
3120 aiko 55.2 2.1 steam
EOF
}

hogs() {
    awk 'NR > 1 && ($3 > 50 || $4 > 20)' | sort -k3,3nr \
        | awk '{ printf "%s %s (%s%% CPU, %s%% MEM)\n", $1, $5, $3, $4 }'
}

processes | hogs
echo "kill $(processes | hogs | head -1 | cut -d' ' -f1)"

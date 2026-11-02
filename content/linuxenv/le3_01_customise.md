@@lesson le3-customise Making It Yours: Desktop, Shell and Startup
@@concepts files, functions, automation
@@lang bash
@@text
Linux lets you change almost everything: how the desktop looks and behaves, what starts at login, what your terminal does. And because nearly all of it is **text files in your home folder**, your whole setup can be backed up, versioned in Git and restored on a new machine in minutes.

## The desktop

- **GNOME**: Settings for the basics; **GNOME Tweaks** for fonts, title-bar buttons and startup apps; **Extensions** (install **Extension Manager** from Flathub) add a dock (Dash to Dock), a system tray (AppIndicator), tiling, clipboard history. Settings are stored in **dconf**: `gsettings set org.gnome.desktop.interface color-scheme 'prefer-dark'`, and `dconf dump / > gnome.ini` backs them all up.
- **KDE Plasma**: System Settings covers nearly everything; right-click the panel to edit it; **Get New...** buttons download themes, widgets and icons. Settings live in `~/.config/*rc` files (`kdeglobals`, `kwinrc`, `plasma-org.kde.plasma.desktop-appletsrc`).
- **Cinnamon / Xfce**: right-click the panel and desktop; Themes, Applets, Desklets, Extensions in Settings.
- **Themes and icons**: GTK themes go in `~/.themes` or `~/.local/share/themes`, icons in `~/.local/share/icons`. Popular: Papirus icons, Adwaita (GNOME's default), Breeze (KDE). Flatpak apps follow the theme when you install the matching Flatpak theme or use the portal.
- **Fonts**: copy `.ttf`/`.otf` files into `~/.local/share/fonts` and run `fc-cache -f`. For Japanese, install **Noto Sans CJK** (`fonts-noto-cjk`), and for typing Japanese an input method framework: **Fcitx5 + Mozc** (or IBus + Mozc/Anthy).
- **Keyboard shortcuts**: every DE lets you bind keys to commands: launch the terminal with Ctrl+Alt+T, take an area screenshot, move windows between monitors.
- **Multiple monitors and HiDPI**: Display settings; fractional scaling (125%, 150%) works best on Wayland.

## Launchers: `.desktop` files

App menu entries are small text files in `/usr/share/applications` (system) and `~/.local/share/applications` (yours). Write one to add a script or an AppImage to the menu:

```text
[Desktop Entry]
Type=Application
Name=Backup Photos
Comment=Copy my photos to the NAS
Exec=/home/aiko/bin/backup-photos.sh
Icon=folder-pictures
Terminal=true
Categories=Utility;
```

Copy one from `/usr/share/applications` into `~/.local/share/applications` and edit it to change a system app's name, icon or launch options: yours wins. The first exercise writes and checks a launcher.

## Autostart

- Desktop apps at login: a `.desktop` file in `~/.config/autostart/` (the DE's "Startup Applications" tool writes these).
- Background jobs: a **systemd user service** in `~/.config/systemd/user/`, managed with `systemctl --user`. It restarts on failure and logs to the journal:

```text
# ~/.config/systemd/user/syncnotes.service
[Unit]
Description=Sync my notes folder

[Service]
ExecStart=/home/aiko/bin/syncnotes.sh
Restart=on-failure

[Install]
WantedBy=default.target
```

```text
systemctl --user daemon-reload
systemctl --user enable --now syncnotes.service
systemctl --user status syncnotes
journalctl --user -u syncnotes -f
```

- Scheduled jobs: a systemd **timer** (`OnCalendar=daily`) next to the service, or `crontab -e` (`0 21 * * * /home/aiko/bin/backup.sh`).

## The shell, customised

`~/.bashrc` runs for every interactive Bash; put your additions there (or in `~/.bashrc.d/*.sh` on Fedora).

```text
export EDITOR=nano                       # or vim, micro, code --wait
export PATH="$HOME/.local/bin:$PATH"     # your own scripts, run by name
alias ll='ls -lah'
alias update='sudo apt update && sudo apt upgrade'
mkcd() { mkdir -p "$1" && cd "$1"; }     # functions take arguments, aliases can't
HISTSIZE=50000
```

Run `source ~/.bashrc` (or open a new terminal) to apply. Friendly upgrades: **Starship** prompt (shows the Git branch, the exit code, the Python env), **fzf** (fuzzy Ctrl+R history and file search), **zoxide** (`z proj` jumps to your most-used folder), **bat** (cat with colours), **eza** (a nicer ls), **btop** (a beautiful task manager).

The second exercise builds the useful functions above and tests them.

## Dotfiles

Your settings files (`.bashrc`, `.gitconfig`, `~/.config/...`) are called **dotfiles**. Keep them in a Git repository:

- the simple way: `~/dotfiles/` with the real files and **symlinks** into place (`ln -s ~/dotfiles/bashrc ~/.bashrc`), managed by hand or with **GNU Stow** (`stow bash` links everything in `~/dotfiles/bash/` into your home);
- or a dedicated tool: **chezmoi** (templates, secrets, per-machine differences) or **yadm**.

Never commit secrets (tokens, SSH private keys) to a dotfiles repository, especially a public one. The third exercise writes an idempotent setup function: running it twice changes nothing the second time.

@@quiz
Where do you put a `.desktop` file to add your own app to the menu, without root?
- /usr/share/applications
* ~/.local/share/applications
- /etc/xdg
- ~/Desktop only
? The user's copy overrides the system one with the same name.

@@quiz
What's the advantage of a systemd user service over adding `&` to a command in .bashrc?
* It runs at login without a terminal, restarts on failure, logs to the journal, and can be managed with systemctl --user
- It runs as root
- It's the only way to run scripts on Linux
- It doesn't need a script
? Timers in the same directory replace many cron jobs.

@@quiz
Why use a function instead of an alias for `mkcd`?
- Aliases are deprecated
* Functions can use arguments ($1) anywhere and run several commands; aliases only expand text at the start
- Functions are faster to type
- Aliases can't contain mkdir
? `mkcd() { mkdir -p "$1" && cd "$1"; }`

@@quiz
How do you make new fonts available in your apps?
* Copy them to ~/.local/share/fonts and run fc-cache -f
- Put them in /tmp
- chmod +x the font files
- Install them with flatpak only
? System-wide fonts go in /usr/share/fonts (or /usr/local/share/fonts).

@@quiz
What does GNU Stow do with dotfiles?
- Encrypts them
* Creates symlinks from your home folder to the files kept in a dotfiles repository
- Uploads them to GitHub
- Compresses old versions
? So editing ~/.bashrc edits the file in the Git repository.

@@flashcards
GNOME Tweaks / Extension Manager :: extra GNOME settings / install extensions
gsettings / dconf :: read and write GNOME settings from the terminal
~/.config/*rc :: where KDE Plasma keeps its settings
~/.local/share/fonts + fc-cache -f :: install fonts for yourself
Fcitx5 + Mozc :: typing Japanese on Linux
.desktop file :: app menu entry: Name, Exec, Icon, Terminal, Categories
~/.config/autostart :: apps started at login
systemctl --user enable --now :: start a user service now and at every login
systemd timer :: scheduled job, alternative to cron
crontab -e :: edit your scheduled jobs
~/.bashrc :: runs for every interactive Bash
alias vs function :: text shortcut vs real command with arguments
PATH :: folders searched for commands
~/.local/bin :: your own scripts, runnable by name
Starship / fzf / zoxide :: prompt / fuzzy finder / smart cd
dotfiles :: your settings files, kept in Git
GNU Stow / chezmoi :: tools to install dotfiles on a new machine
idempotent :: running it again changes nothing

@@exercise Write a launcher
Write a function `make_launcher NAME EXEC ICON` that writes `~/.local/share/applications/<name in lower case with spaces as dashes>.desktop` (use the `$HOME` the starter sets) containing the `[Desktop Entry]` header and the lines `Type=Application`, `Name=`, `Exec=`, `Icon=`, `Terminal=false` and `Categories=Utility;`. Then write `check_launcher FILE` that prints `ok FILE` when the file has `Type`, `Name` and `Exec` lines, otherwise `missing KEY` for each one missing.
@@starter
export HOME="$(mktemp -d)"
cd "$HOME"

make_launcher() {
    # Your code here
    :
}

check_launcher() {
    # Your code here
    :
}

make_launcher "Photo Backup" "$HOME/bin/backup-photos.sh" folder-pictures
f="$HOME/.local/share/applications/photo-backup.desktop"
sed "s#$HOME#~#" "$f"
check_launcher "$f" | sed "s#$HOME#~#"
printf '[Desktop Entry]\nName=Broken\n' > broken.desktop
check_launcher broken.desktop
@@expected
[Desktop Entry]
Type=Application
Name=Photo Backup
Exec=~/bin/backup-photos.sh
Icon=folder-pictures
Terminal=false
Categories=Utility;
ok ~/.local/share/applications/photo-backup.desktop
missing Type
missing Exec
@@requires
mkdir -p
grep
@@hint
The file name: `slug=$(echo "$1" | tr '[:upper:] ' '[:lower:]-')`. `mkdir -p "$HOME/.local/share/applications"`, then write the lines with a heredoc (`cat > "$file" <<EOF ... EOF`, unquoted so `$1` expands). For the check, loop over `Type Name Exec` and test `grep -q "^$key=" "$1"`.
@@solution
export HOME="$(mktemp -d)"
cd "$HOME"

make_launcher() {
    local dir="$HOME/.local/share/applications"
    local slug
    slug=$(echo "$1" | tr '[:upper:] ' '[:lower:]-')
    mkdir -p "$dir"
    cat > "$dir/$slug.desktop" <<EOF
[Desktop Entry]
Type=Application
Name=$1
Exec=$2
Icon=$3
Terminal=false
Categories=Utility;
EOF
}

check_launcher() {
    local ok=1 key
    for key in Type Name Exec; do
        if ! grep -q "^$key=" "$1"; then
            echo "missing $key"
            ok=0
        fi
    done
    [ "$ok" = 1 ] && echo "ok $1"
    return 0
}

make_launcher "Photo Backup" "$HOME/bin/backup-photos.sh" folder-pictures
f="$HOME/.local/share/applications/photo-backup.desktop"
sed "s#$HOME#~#" "$f"
check_launcher "$f" | sed "s#$HOME#~#"
printf '[Desktop Entry]\nName=Broken\n' > broken.desktop
check_launcher broken.desktop

@@exercise Shell functions for your .bashrc
Write three functions: `mkcd DIR` (create the folder with parents and change into it), `extract FILE` (print the command that would unpack it: `tar -xzf FILE` for `.tar.gz` or `.tgz`, `tar -xJf FILE` for `.tar.xz`, `unzip FILE` for `.zip`, `7z x FILE` for `.7z`, otherwise `don't know how to extract FILE` and return 1), and `bak FILE` (copy it to `FILE.bak`, or `FILE.bak.2`, `.bak.3`... if that exists).
@@starter
cd "$(mktemp -d)"

mkcd() {
    :
}

extract() {
    :
}

bak() {
    :
}

mkcd projects/site && pwd | sed 's#.*/projects#projects#'
for f in photos.tar.gz backup.tgz kernel.tar.xz fonts.zip game.7z notes.rar; do
    extract "$f" || true
done
echo "v1" > config.ini
bak config.ini; bak config.ini; bak config.ini
ls config.ini*
@@expected
projects/site
tar -xzf photos.tar.gz
tar -xzf backup.tgz
tar -xJf kernel.tar.xz
unzip fonts.zip
7z x game.7z
don't know how to extract notes.rar
config.ini
config.ini.bak
config.ini.bak.2
config.ini.bak.3
@@requires
case
@@hint
`mkcd() { mkdir -p "$1" && cd "$1"; }`. In `extract`, `case "$1" in *.tar.gz|*.tgz) ... ;; esac`. For `bak`, start with `target="$1.bak"` and `n=2`, and while `[ -e "$target" ]` set `target="$1.bak.$n"` and increment `n`.
@@solution
cd "$(mktemp -d)"

mkcd() {
    mkdir -p "$1" && cd "$1"
}

extract() {
    case "$1" in
        *.tar.gz|*.tgz) echo "tar -xzf $1" ;;
        *.tar.xz) echo "tar -xJf $1" ;;
        *.zip) echo "unzip $1" ;;
        *.7z) echo "7z x $1" ;;
        *) echo "don't know how to extract $1"; return 1 ;;
    esac
}

bak() {
    local target="$1.bak" n=2
    while [ -e "$target" ]; do
        target="$1.bak.$n"
        n=$((n + 1))
    done
    cp "$1" "$target"
}

mkcd projects/site && pwd | sed 's#.*/projects#projects#'
for f in photos.tar.gz backup.tgz kernel.tar.xz fonts.zip game.7z notes.rar; do
    extract "$f" || true
done
echo "v1" > config.ini
bak config.ini; bak config.ini; bak config.ini
ls config.ini*

@@exercise An idempotent setup step
A setup script must be safe to run many times. Write `ensure_line FILE LINE` that appends `LINE` to `FILE` only if that exact line isn't already there (create the file if needed) and prints `added: LINE` or `present: LINE`, and `ensure_dir DIR` that prints `created: DIR` or `exists: DIR`. The starter runs the setup twice.
@@starter
cd "$(mktemp -d)"

ensure_line() {
    :
}

ensure_dir() {
    :
}

setup() {
    ensure_dir .local/bin
    ensure_line .bashrc 'export PATH="$HOME/.local/bin:$PATH"'
    ensure_line .bashrc "alias ll='ls -lah'"
}

echo "first run"; setup
echo "second run"; setup
echo "--- .bashrc"; cat .bashrc
@@expected
first run
created: .local/bin
added: export PATH="$HOME/.local/bin:$PATH"
added: alias ll='ls -lah'
second run
exists: .local/bin
present: export PATH="$HOME/.local/bin:$PATH"
present: alias ll='ls -lah'
--- .bashrc
export PATH="$HOME/.local/bin:$PATH"
alias ll='ls -lah'
@@requires
grep -qxF
@@hint
`grep -qxF -- "$2" "$1" 2>/dev/null` is true when the exact line exists (`-x` whole line, `-F` no regex). Otherwise `printf '%s\n' "$2" >> "$1"`. For the folder: `[ -d "$1" ]`.
@@solution
cd "$(mktemp -d)"

ensure_line() {
    if grep -qxF -- "$2" "$1" 2>/dev/null; then
        echo "present: $2"
    else
        printf '%s\n' "$2" >> "$1"
        echo "added: $2"
    fi
}

ensure_dir() {
    if [ -d "$1" ]; then
        echo "exists: $1"
    else
        mkdir -p "$1"
        echo "created: $1"
    fi
}

setup() {
    ensure_dir .local/bin
    ensure_line .bashrc 'export PATH="$HOME/.local/bin:$PATH"'
    ensure_line .bashrc "alias ll='ls -lah'"
}

echo "first run"; setup
echo "second run"; setup
echo "--- .bashrc"; cat .bashrc

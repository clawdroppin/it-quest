from gen_projects import P

PROJECTS = [
    P("bash", "Intermediate", "dotfiles", "Dotfiles Installer", "Keep your shell setup in Git and install it on any Linux machine.",
      """Your aliases, prompt, Git config and editor settings, in one Git repository, with an `install.sh` that symlinks them into your home folder, backs up anything it would replace, and can be run again safely.""",
      ["symlinks (ln -s)", "idempotent scripts", "backups with timestamps", "set -euo pipefail"],
      ["Collect: put .bashrc, .gitconfig and .vimrc (or .inputrc) in the dotfiles folder.",
       "Link: for each file, create `~/<file>` as a symlink to the repo copy.",
       "Back up: if a real file is there already, move it to `~/.dotfiles-backup/<date>/`.",
       "Idempotent: running it twice changes nothing the second time and says so.",
       "Uninstall: `install.sh --uninstall` removes the links and restores the backups."],
      ["On a fresh VM or WSL, `git clone` plus `./install.sh` gives you your setup."],
      ["How do I check whether a path is already a symlink to the right place?",
       "Review my script for quoting bugs with filenames that contain spaces."],
      {"install.sh": '''#!/usr/bin/env bash
# Symlink every file in ./home into $HOME, backing up anything that's in the way.
set -euo pipefail

repo="$(cd "$(dirname "$0")" && pwd)"
backup="$HOME/.dotfiles-backup/$(date +%Y%m%d-%H%M%S)"

for src in "$repo"/home/.[!.]*; do
  name="$(basename "$src")"
  dest="$HOME/$name"
  # TODO: skip if $dest already links to $src
  # TODO: back up a real file at $dest into $backup
  # TODO: ln -s "$src" "$dest"
  echo "would link $dest -> $src"
done
''', "home/.bash_aliases": '''alias ll='ls -lah'
alias gs='git status -sb'
alias ..='cd ..'
'''}),
    P("bash", "Advanced", "motd", "Server Login Dashboard", "A message of the day showing a server's health at login.",
      """When you SSH into a server you want to see at a glance: hostname, uptime, load, memory, disks over 80%, failed systemd units and pending updates. Build that as a script that runs at login, with colours that turn red when something's wrong.""",
      ["/proc and standard tools", "awk", "functions", "ANSI colours", "systemctl and package managers"],
      ["Basics: hostname, OS (from /etc/os-release), uptime and load averages.",
       "Memory and disks: used percentages from `free` and `df -P`, red above 80%.",
       "Services: `systemctl --failed` count and names.",
       "Updates: count pending updates with apt or dnf, whichever exists.",
       "Install: put it in /etc/profile.d/ or /etc/update-motd.d/ and test with a new SSH login."],
      ["Logging in shows a correct, coloured summary in under a second."],
      ["How can I make the update check fast enough for every login?",
       "Show me how to parse `df -P` with awk to get the use percentage."],
      {"motd.sh": '''#!/usr/bin/env bash
# A login dashboard. Install: sudo cp motd.sh /etc/profile.d/zz-motd.sh
red=$'\\e[31m'; green=$'\\e[32m'; bold=$'\\e[1m'; reset=$'\\e[0m'

pct_colour() { # $1 = a number 0-100
  if (( $1 >= 80 )); then printf '%s%s%%%s' "$red" "$1" "$reset"; else printf '%s%s%%%s' "$green" "$1" "$reset"; fi
}

echo "${bold}$(hostname)${reset}  $(. /etc/os-release && echo "$PRETTY_NAME")"
echo "up $(uptime -p 2>/dev/null || uptime)"
# TODO: load, memory, disks, failed units, pending updates
'''}),
]

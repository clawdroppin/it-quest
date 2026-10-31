@@lesson xp-motd Server Login Dashboard
@@text
When you SSH into a server you want to see at a glance: hostname, uptime, load, memory, disks over 80%, failed systemd units and pending updates. Build that as a script that runs at login, with colours that turn red when something's wrong.

**You'll practise:** /proc and standard tools · awk · functions · ANSI colours · systemctl and package managers

## Milestones

1. **Basics**: hostname, OS (from /etc/os-release), uptime and load averages.
2. **Memory and disks**: used percentages from `free` and `df -P`, red above 80%.
3. **Services**: `systemctl --failed` count and names.
4. **Updates**: count pending updates with apt or dnf, whichever exists.
5. **Install**: put it in /etc/profile.d/ or /etc/update-motd.d/ and test with a new SSH login.

## You're done when

- Logging in shows a correct, coloured summary in under a second.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"How can I make the update check fast enough for every login?"*
- *"Show me how to parse `df -P` with awk to get the use percentage."*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project motd Server Login Dashboard
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Server Login Dashboard

When you SSH into a server you want to see at a glance: hostname, uptime, load, memory, disks over 80%, failed systemd units and pending updates. Build that as a script that runs at login, with colours that turn red when something's wrong.

## Milestones

- [ ] 1. Basics: hostname, OS (from /etc/os-release), uptime and load averages.
- [ ] 2. Memory and disks: used percentages from `free` and `df -P`, red above 80%.
- [ ] 3. Services: `systemctl --failed` count and names.
- [ ] 4. Updates: count pending updates with apt or dnf, whichever exists.
- [ ] 5. Install: put it in /etc/profile.d/ or /etc/update-motd.d/ and test with a new SSH login.

## Done when

- [ ] Logging in shows a correct, coloured summary in under a second.
@@file motd.sh
#!/usr/bin/env bash
# A login dashboard. Install: sudo cp motd.sh /etc/profile.d/zz-motd.sh
red=$'\e[31m'; green=$'\e[32m'; bold=$'\e[1m'; reset=$'\e[0m'

pct_colour() { # $1 = a number 0-100
  if (( $1 >= 80 )); then printf '%s%s%%%s' "$red" "$1" "$reset"; else printf '%s%s%%%s' "$green" "$1" "$reset"; fi
}

echo "${bold}$(hostname)${reset}  $(. /etc/os-release && echo "$PRETTY_NAME")"
echo "up $(uptime -p 2>/dev/null || uptime)"
# TODO: load, memory, disks, failed units, pending updates

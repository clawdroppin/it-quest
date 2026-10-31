@@lesson xp-lamp_lab Self-hosted Web Server Lab
@@text
In a VirtualBox, Hyper-V or WSL machine, install a web server, put a site on it, and secure it the way a junior Linux admin would: a non-root sudo user, SSH keys only, a firewall, automatic security updates, log rotation and a backup job. Keep a runbook (RUNBOOK.md) so anyone could rebuild it.

**You'll practise:** users and sudo · SSH keys and sshd_config · ufw or firewalld · nginx · systemd and timers · cron or timers for backups

## Milestones

1. **VM**: install Ubuntu Server or Debian and note the version, IP and resources.
2. **Access**: create an admin user, add your SSH key, disable password and root login.
3. **Firewall**: allow only SSH and HTTP/HTTPS.
4. **Web**: install nginx, serve your HTML project, check logs in /var/log/nginx.
5. **Maintenance**: unattended-upgrades, a backup script on a systemd timer, and a check with `systemctl list-timers`.

## You're done when

- You can rebuild the whole server from RUNBOOK.md and check.sh reports everything green.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"Review my sshd_config changes before I restart sshd and lock myself out."*
- *"How do I write a systemd timer instead of a cron job?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project lamp_lab Self-hosted Web Server Lab
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Self-hosted Web Server Lab

In a VirtualBox, Hyper-V or WSL machine, install a web server, put a site on it, and secure it the way a junior Linux admin would: a non-root sudo user, SSH keys only, a firewall, automatic security updates, log rotation and a backup job. Keep a runbook (RUNBOOK.md) so anyone could rebuild it.

## Milestones

- [ ] 1. VM: install Ubuntu Server or Debian and note the version, IP and resources.
- [ ] 2. Access: create an admin user, add your SSH key, disable password and root login.
- [ ] 3. Firewall: allow only SSH and HTTP/HTTPS.
- [ ] 4. Web: install nginx, serve your HTML project, check logs in /var/log/nginx.
- [ ] 5. Maintenance: unattended-upgrades, a backup script on a systemd timer, and a check with `systemctl list-timers`.

## Done when

- [ ] You can rebuild the whole server from RUNBOOK.md and check.sh reports everything green.
@@file RUNBOOK.md
# Web server runbook

## Machine
- OS:
- IP:

## Steps
1. Create the admin user: `sudo adduser admin && sudo usermod -aG sudo admin`
2. ...
@@file check.sh
#!/usr/bin/env bash
# Checks the server's security basics. Run on the server: bash check.sh
ok() { printf '\e[32m✔\e[0m %s\n' "$1"; }
bad() { printf '\e[31m✘\e[0m %s\n' "$1"; }

if grep -Eq '^\s*PasswordAuthentication\s+no' /etc/ssh/sshd_config /etc/ssh/sshd_config.d/*.conf 2>/dev/null; then
  ok "SSH password login disabled"
else
  bad "SSH password login is still allowed"
fi
# TODO: root login, firewall active, nginx running, unattended-upgrades, backup timer

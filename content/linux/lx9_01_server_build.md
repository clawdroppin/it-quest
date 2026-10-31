@@lesson lx9-server-build Build a Hardened Web Server
@@text
This is the job interview classic: *"Here's a fresh Ubuntu VM. Make it a secure web server."* You'll do it twice: first **by hand**, following the lab guide (so you understand each step), then with a **provisioning script** that does it all repeatably. Run it in your home lab (the Foundations track) or on a cheap cloud VM (AWS Lightsail, an EC2 free-tier instance, or a ¥500/month VPS).

What you'll end up with:

- an admin user with **SSH keys only**, no root login, **fail2ban**
- **ufw** allowing only SSH and HTTP/HTTPS
- **automatic security updates**
- **nginx** serving a site, with its own log rotation
- a **nightly backup** with a systemd timer
- a short **report** you can paste into your portfolio

@@project server_build Hardened Web Server Lab
Read `LAB.md` and do each step by hand first. Then read `provision.sh`, run it with `--dry-run` to see what it would do, and finally run it for real on a **fresh** VM. Keep a terminal logged in while you change SSH settings, so you can't lock yourself out.
@@file LAB.md
# Lab: a hardened Ubuntu web server

Use Ubuntu Server LTS in a VM. Take a snapshot first ("fresh install").

## 1. Update and basics
    sudo apt update && sudo apt full-upgrade -y
    sudo hostnamectl set-hostname web01
    sudo timedatectl set-timezone Asia/Tokyo
    timedatectl                      # check NTP is active

## 2. An admin user with SSH keys
On YOUR computer:

    ssh-keygen -t ed25519 -C "you@laptop"
    ssh-copy-id -i ~/.ssh/id_ed25519.pub you@web01

Check you can log in with the key, then on the server create /etc/ssh/sshd_config.d/10-hardening.conf:

    PermitRootLogin no
    PasswordAuthentication no
    KbdInteractiveAuthentication no
    AllowGroups sudo

    sudo sshd -t && sudo systemctl reload ssh

Open a SECOND terminal and log in again before you close the first one.

## 3. Firewall
    sudo ufw default deny incoming
    sudo ufw default allow outgoing
    sudo ufw allow OpenSSH
    sudo ufw allow 'Nginx Full'      # after installing nginx in step 5
    sudo ufw enable
    sudo ufw status verbose

## 4. Automatic security updates and fail2ban
    sudo apt install -y unattended-upgrades fail2ban
    sudo dpkg-reconfigure -plow unattended-upgrades
    sudo systemctl enable --now fail2ban
    sudo fail2ban-client status sshd

## 5. Web server
    sudo apt install -y nginx
    echo '<h1>web01 works</h1>' | sudo tee /var/www/html/index.html
    curl -I http://localhost
    sudo nginx -t

Optional: a real certificate with certbot (needs a domain name pointing at the server).

## 6. Backups with a systemd timer
See backup.sh, backup.service and backup.timer in this folder.

    sudo install -m 750 backup.sh /usr/local/bin/backup.sh
    sudo cp backup.service backup.timer /etc/systemd/system/
    sudo systemctl daemon-reload
    sudo systemctl enable --now backup.timer
    systemctl list-timers backup.timer
    sudo systemctl start backup.service && ls -lh /var/backups/web

## 7. Check your work
    sudo ss -tulpn                       # only 22, 80 (and 443) should listen publicly
    sudo find / -perm -4000 -type f 2>/dev/null   # any unexpected SUID programs?
    sudo journalctl -p warning -b        # warnings since boot
    sudo lynis audit system              # optional: apt install lynis; note the hardening index

## 8. Break it and fix it (do at least two)
- Add a typo to the nginx config, restart it, and troubleshoot with systemctl status, journalctl -u nginx and nginx -t.
- Fill the disk with fallocate -l 5G /tmp/big and see what fails; clean up.
- Block port 80 in ufw and debug it from another machine with curl and nmap.
- Change a file's permissions under /var/www so nginx gets "403 Forbidden"; find it in /var/log/nginx/error.log.

Write what happened in your lab log.
@@file provision.sh
#!/usr/bin/env bash
# Provision a fresh Ubuntu server as a hardened nginx web server.
# Usage: sudo ./provision.sh [--dry-run]
# Safe to run more than once: every step checks before it changes anything.
set -euo pipefail

DRY_RUN=0
[[ "${1:-}" == "--dry-run" ]] && DRY_RUN=1

say() { printf '\n== %s\n' "$*"; }
run() {
    if (( DRY_RUN )); then
        printf '   would run: %s\n' "$*"
    else
        printf '   + %s\n' "$*"
        "$@"
    fi
}

if (( ! DRY_RUN )) && [[ $EUID -ne 0 ]]; then
    echo "Run as root: sudo $0" >&2
    exit 1
fi

say "Packages"
run apt-get update -q
run apt-get install -y -q nginx ufw fail2ban unattended-upgrades

say "SSH hardening"
conf=/etc/ssh/sshd_config.d/10-hardening.conf
if [[ -f "$conf" ]]; then
    echo "   $conf already exists"
else
    # Refuse to disable passwords if nobody has an SSH key yet: that would lock everyone out.
    if ! compgen -G "/home/*/.ssh/authorized_keys" > /dev/null; then
        echo "   No user has ~/.ssh/authorized_keys. Add your key first (ssh-copy-id)." >&2
        (( DRY_RUN )) || exit 1
    fi
    if (( DRY_RUN )); then
        echo "   would write $conf"
    else
        printf 'PermitRootLogin no\nPasswordAuthentication no\nKbdInteractiveAuthentication no\n' > "$conf"
    fi
    run sshd -t
    run systemctl reload ssh
fi

say "Firewall"
run ufw default deny incoming
run ufw default allow outgoing
run ufw allow OpenSSH
run ufw allow 'Nginx Full'
run ufw --force enable

say "fail2ban and automatic updates"
run systemctl enable --now fail2ban
if (( DRY_RUN )); then
    echo "   would enable unattended-upgrades"
else
    printf 'APT::Periodic::Update-Package-Lists "1";\nAPT::Periodic::Unattended-Upgrade "1";\n' > /etc/apt/apt.conf.d/20auto-upgrades
fi

say "Web site"
page=/var/www/html/index.html
if (( DRY_RUN )); then
    echo "   would write $page"
else
    printf '<h1>%s works</h1>\n<p>Provisioned %s</p>\n' "$(hostname)" "$(date -I)" > "$page"
fi
run nginx -t
run systemctl enable --now nginx

say "Backups"
here="$(cd "$(dirname "$0")" && pwd)"
run install -m 750 "$here/backup.sh" /usr/local/bin/backup.sh
run cp "$here/backup.service" "$here/backup.timer" /etc/systemd/system/
run systemctl daemon-reload
run systemctl enable --now backup.timer

say "Report"
if (( ! DRY_RUN )); then
    echo "   Listening:"
    ss -tlpn | awk 'NR > 1 { print "     " $4 }'
    echo "   Firewall:"
    ufw status | sed 's/^/     /'
fi
echo
echo "Done. Test from another machine: curl -I http://$(hostname -I 2>/dev/null | awk '{print $1}')"
@@file backup.sh
#!/usr/bin/env bash
# Nightly backup of the web root and nginx config, keeping 14 days.
set -euo pipefail

dest=/var/backups/web
keep_days=14
mkdir -p "$dest"
stamp=$(date +%Y%m%d-%H%M%S)
archive="$dest/web-$stamp.tar.gz"

tar -czf "$archive" /var/www /etc/nginx 2>/dev/null
sha256sum "$archive" > "$archive.sha256"
find "$dest" -name 'web-*.tar.gz*' -mtime +"$keep_days" -delete

echo "backup ok: $archive ($(du -h "$archive" | cut -f1))"
@@file backup.service
[Unit]
Description=Back up the web site

[Service]
Type=oneshot
ExecStart=/usr/local/bin/backup.sh
@@file backup.timer
[Unit]
Description=Back up the web site every night

[Timer]
OnCalendar=*-*-* 02:30:00
Persistent=true

[Install]
WantedBy=timers.target
@@text
🎉 Snapshot, provision, verify, break, fix, document: that's the real job. Put `LAB.md`, the scripts and your lab-log notes on GitHub as **"Hardened Ubuntu web server"**, with the before/after output of `ss -tulpn` and lynis' hardening index. Then try the same with Ansible (the Linux+ automation lesson has a starter playbook).

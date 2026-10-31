@@lesson xp-log_rotation Log Rotation and Disk Alerting
@@text
Full disks take servers down. Set up logrotate for an application that writes big logs, then write a script, run by a systemd timer, that checks every filesystem, writes a warning to the journal at 80% and a critical message at 90%, and lists the largest directories so the on-call person knows where to look.

**You'll practise:** logrotate · df and du · systemd service and timer units · logger and journalctl

## Milestones

1. **App**: a script that writes lots of log lines to /var/log/myapp/app.log.
2. **logrotate**: daily, keep 7, compress, copytruncate or a postrotate signal.
3. **Check**: diskcheck.sh reads `df -P`, logs with `logger -p user.warning`.
4. **Largest**: `du -xh --max-depth=2 / | sort -h | tail` for filesystems over the limit.
5. **Schedule**: diskcheck.service and diskcheck.timer every 15 minutes.

## You're done when

- `logrotate -d` shows your rules working and `journalctl -t diskcheck` shows alerts after you fill a test disk.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"copytruncate or postrotate: which should I use, and what can go wrong?"*
- *"Review my systemd timer unit."*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project log_rotation Log Rotation and Disk Alerting
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Log Rotation and Disk Alerting

Full disks take servers down. Set up logrotate for an application that writes big logs, then write a script, run by a systemd timer, that checks every filesystem, writes a warning to the journal at 80% and a critical message at 90%, and lists the largest directories so the on-call person knows where to look.

## Milestones

- [ ] 1. App: a script that writes lots of log lines to /var/log/myapp/app.log.
- [ ] 2. logrotate: daily, keep 7, compress, copytruncate or a postrotate signal.
- [ ] 3. Check: diskcheck.sh reads `df -P`, logs with `logger -p user.warning`.
- [ ] 4. Largest: `du -xh --max-depth=2 / | sort -h | tail` for filesystems over the limit.
- [ ] 5. Schedule: diskcheck.service and diskcheck.timer every 15 minutes.

## Done when

- [ ] `logrotate -d` shows your rules working and `journalctl -t diskcheck` shows alerts after you fill a test disk.
@@file diskcheck.sh
#!/usr/bin/env bash
# Log a warning for filesystems above the thresholds.
set -euo pipefail
warn=80; crit=90
df -P -x tmpfs -x devtmpfs | awk 'NR > 1 { gsub("%", "", $5); print $5, $6 }' | while read -r use mount; do
  if (( use >= crit )); then
    logger -t diskcheck -p user.crit "$mount is ${use}% full"
  elif (( use >= warn )); then
    logger -t diskcheck -p user.warning "$mount is ${use}% full"
  fi
done
# TODO: list the largest directories on full filesystems
@@file myapp.logrotate
/var/log/myapp/*.log {
    daily
    rotate 7
    compress
    missingok
    notifempty
    # TODO: copytruncate or postrotate
}
@@file diskcheck.timer
[Unit]
Description=Check disk space every 15 minutes

[Timer]
OnCalendar=*:0/15
Persistent=true

[Install]
WantedBy=timers.target

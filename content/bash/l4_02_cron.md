@@lesson l4-cron Scheduling: cron, systemd Timers and logrotate
@@concepts automation
@@text
## cron

Each user has a **crontab**: a table of commands and when to run them. `crontab -e` edits yours, `crontab -l` lists it.

```text
# ┌ minute (0-59)
# │ ┌ hour (0-23)
# │ │ ┌ day of month (1-31)
# │ │ │ ┌ month (1-12)
# │ │ │ │ ┌ day of week (0-7, 0 and 7 = Sunday)
# │ │ │ │ │
  30 2 * * *   /opt/scripts/backup.sh >> /var/log/backup.log 2>&1
  */5 * * * *  /opt/scripts/check_site.sh
  0 9 * * 1-5  /opt/scripts/report.sh
  0 0 1 * *    /opt/scripts/monthly.sh
  @reboot      /opt/scripts/on_boot.sh
```

| Field value | Means |
|---|---|
| `*` | every |
| `*/5` | every 5th |
| `1-5` | a range (Monday to Friday) |
| `1,15` | a list |

Cron gotchas, all of which have cost someone a weekend:

1. Cron runs with a **tiny PATH** and no profile. Use full paths (`/usr/bin/python3`) or set `PATH=` at the top of the crontab.
2. Output goes nowhere (or to local mail) unless you redirect it: `>> log 2>&1`.
3. `%` is special in crontabs; escape it as `\%` (`date +\%F`).
4. Times are the **server's** time zone. Check with `timedatectl`.

[crontab.guru](https://crontab.guru) explains any schedule in plain English.

## systemd timers

The modern alternative: a `.timer` unit next to a `.service`. Advantages: logs in the journal, `Persistent=true` catches up on runs missed while the machine was off, and `systemctl list-timers` shows when everything runs next.

```ini
# /etc/systemd/system/backup.timer
[Timer]
OnCalendar=*-*-* 02:30:00
Persistent=true

[Install]
WantedBy=timers.target
```

## logrotate

Logs grow forever unless something rotates them. `logrotate` (run daily by cron or a timer) renames `app.log` to `app.log.1`, compresses old ones and deletes the oldest:

```text
# /etc/logrotate.d/myapp
/var/log/myapp/*.log {
    daily
    rotate 14
    compress
    delaycompress
    missingok
    notifempty
}
```

Test with `sudo logrotate -d /etc/logrotate.d/myapp` (`-d` = dry run).

@@quiz
What does `*/15 * * * *` mean?
* every 15 minutes
- at minute 15 of every hour
- every 15 hours
- on the 15th of each month
? `*/15` in the minute field: minutes 0, 15, 30, 45.

@@quiz
A script works when you run it, but fails from cron with "command not found". The most likely cause?
* Cron's PATH is minimal; use full paths or set PATH in the crontab
- Cron can't run Bash scripts
- The script needs `sudo`
- Cron only runs at night
? Cron doesn't load your shell profile.

@@drill
Write the cron schedule (the five time fields) for **every Monday at 06:00**.
= 0 6 * * 1
= 00 06 * * 1
= 0 6 * * mon
= 0 6 * * MON
? minute 0, hour 6, any day of month, any month, day of week 1 (Monday).

@@flashcards
crontab -e / -l :: Edit / list your cron jobs
cron fields :: minute hour day-of-month month day-of-week
*/5 :: Every 5th unit (every 5 minutes in the minute field)
@reboot :: Run once at boot
systemd timer :: Modern cron alternative; Persistent=true catches up missed runs
systemctl list-timers :: When each timer runs next
logrotate :: Rotates, compresses and deletes old logs

@@exercise Explain a schedule
Write `cron_hour_minute` that takes a cron line and prints the time it runs as `HH:MM` when both the minute and hour fields are plain numbers, or `complex` otherwise. `30 2 * * * /opt/backup.sh` gives `02:30`.
@@starter
cron_hour_minute() {
    echo "TODO: not done yet" >&2
    return 1
}

@@tests
check "$(cron_hour_minute '30 2 * * * /opt/backup.sh')" "02:30" "30 2 * * *"
check "$(cron_hour_minute '0 18 * * 1-5 /opt/report.sh')" "18:00" "0 18 * * 1-5"
check "$(cron_hour_minute '*/5 * * * * /opt/check.sh')" "complex" "*/5 * * * *"
check "$(cron_hour_minute '0 */2 * * * /opt/sync.sh')" "complex" "0 */2 * * *"
@@hint
`read -r min hour _ <<< "$1"` splits the first two fields. Test both with `[[ $min =~ ^[0-9]+$ ]]`, then `printf '%02d:%02d\n' "$hour" "$min"`.
@@solution
cron_hour_minute() {
    local min hour
    read -r min hour _ <<< "$1"
    if [[ $min =~ ^[0-9]+$ && $hour =~ ^[0-9]+$ ]]; then
        printf '%02d:%02d\n' "$hour" "$min"
    else
        echo "complex"
    fi
}

@@exercise Rotate by hand
Simulate logrotate's numbering: the starter creates `app.log`, `app.log.1` and `app.log.2` in a temp folder. Write `rotate` that, with `keep` = 3: deletes `app.log.3` if it exists, renames `.2` to `.3`, `.1` to `.2`, `app.log` to `app.log.1`, and creates a new empty `app.log`. After two rotations, the starter prints the folder.
@@starter
rotate() {
    local log="$1" keep=3
    echo "TODO: not done yet" >&2
    return 1
}

d=$(mktemp -d)
echo "newest" > "$d/app.log"; echo "older" > "$d/app.log.1"; echo "oldest" > "$d/app.log.2"
rotate "$d/app.log"
echo "second" > "$d/app.log"
rotate "$d/app.log"
for f in app.log app.log.1 app.log.2 app.log.3; do
    echo "$f: $(cat "$d/$f" 2>/dev/null || echo '(none)')"
done
rm -rf "$d"
@@expected
app.log:
app.log.1: second
app.log.2: newest
app.log.3: older
@@hint
`rm -f "$log.$keep"`, then a loop counting down: `for (( i = keep - 1; i >= 1; i-- )); do [[ -f "$log.$i" ]] && mv "$log.$i" "$log.$(( i + 1 ))"; done`, then `mv "$log" "$log.1"` and `touch "$log"`.
@@solution
rotate() {
    local log="$1" keep=3 i
    rm -f "$log.$keep"
    for (( i = keep - 1; i >= 1; i-- )); do
        if [[ -f "$log.$i" ]]; then
            mv "$log.$i" "$log.$(( i + 1 ))"
        fi
    done
    mv "$log" "$log.1"
    touch "$log"
}

d=$(mktemp -d)
echo "newest" > "$d/app.log"; echo "older" > "$d/app.log.1"; echo "oldest" > "$d/app.log.2"
rotate "$d/app.log"
echo "second" > "$d/app.log"
rotate "$d/app.log"
for f in app.log app.log.1 app.log.2 app.log.3; do
    echo "$f: $(cat "$d/$f" 2>/dev/null || echo '(none)')"
done
rm -rf "$d"

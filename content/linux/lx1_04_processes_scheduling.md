@@lesson lx1-processes Processes, Signals, Jobs and Scheduling
@@lang bash
@@text
A Linux admin constantly asks "what is running, what is it using, and how do I stop or schedule it?" Linux+ covers process inspection, signals, priorities, background jobs, and scheduling with cron, at and systemd timers.

## Inspecting processes

```bash,norun
ps aux                      # every process: USER PID %CPU %MEM VSZ RSS TTY STAT START TIME COMMAND
ps -ef --forest             # full format with parent-child tree
ps -eo pid,ppid,user,%cpu,%mem,cmd --sort=-%cpu | head
pgrep -a nginx              # PIDs and command lines by name
top / htop                  # live view; in top press P (CPU), M (memory), k (kill), q
pstree -p                   # tree view
lsof -p 1234                # files a process has open
lsof -i :443                # which process uses port 443
/proc/1234/                 # the kernel's view: status, cmdline, fd/
```

**Process states** (STAT column): `R` running, `S` sleeping (interruptible), `D` uninterruptible sleep (usually waiting for disk/NFS: many D processes means I/O trouble), `T` stopped, `Z` **zombie** (finished but the parent hasn't collected its exit status; fix the parent).

**Load average** (`uptime`, `top`): average number of runnable plus D-state processes over 1, 5 and 15 minutes. Compare it to the number of CPU cores (`nproc`): load 8 on a 4-core box means work is waiting.

## Signals

| Signal | Number | Default | Use |
|---|---|---|---|
| `SIGHUP` | 1 | terminate | many daemons **reload config** on HUP |
| `SIGINT` | 2 | terminate | Ctrl+C |
| `SIGKILL` | 9 | kill, can't be caught | last resort; no cleanup |
| `SIGTERM` | 15 | terminate (default for `kill`) | polite stop; process can clean up |
| `SIGSTOP` / `SIGCONT` | 19 / 18 | pause / resume | |
| `SIGTSTP` | 20 | stop | Ctrl+Z |

`kill 1234` (TERM), `kill -9 1234`, `kill -HUP $(pidof nginx)`, `pkill -u ana`, `killall firefox`.

## Priorities

**Nice** values run from **−20** (highest priority) to **19** (lowest); default 0. Only root can lower the value (raise priority). `nice -n 10 ./backup.sh`, `renice -n 5 -p 1234`. **ionice** sets I/O priority.

## Jobs

`command &` runs in the background. `jobs` lists jobs, `fg %1` / `bg %1` move them, **Ctrl+Z** suspends. `nohup command &` keeps running after logout; better: `tmux`/`screen`, or a systemd service.

## cron

User crontabs: `crontab -e` (edit), `crontab -l` (list), `crontab -r` (remove). System: `/etc/crontab` and `/etc/cron.d/` (these include a **user** field), plus `/etc/cron.daily/` etc. Access: `/etc/cron.allow`, `/etc/cron.deny`.

Five time fields, then the command:

```text
# min  hour  day-of-month  month  day-of-week   command
  30   2     *             *      *             /usr/local/bin/backup.sh
  */15 *     *             *      *             /usr/local/bin/check.sh
  0    9     *             *      1-5           /usr/local/bin/report.sh
  0    0     1             */3    *             /usr/local/bin/quarterly.sh
```

`*` every, `*/15` every 15, `1-5` a range (Mon–Fri; 0 and 7 are Sunday), `1,15` a list. Cron runs with a **minimal environment**: use full paths, and redirect output (`>> /var/log/backup.log 2>&1`) or it gets mailed/lost.

## at

One-time jobs: `echo "/usr/local/bin/patch.sh" | at 23:00`, `atq` (list), `atrm 3` (remove).

## systemd timers

A modern alternative to cron with logging in the journal, dependencies and catch-up of missed runs:

```ini
# /etc/systemd/system/backup.timer
[Timer]
OnCalendar=*-*-* 02:30:00
Persistent=true
[Install]
WantedBy=timers.target
```

It triggers `backup.service` of the same name. `systemctl enable --now backup.timer`, `systemctl list-timers`.

@@quiz
Which signal asks a process to terminate gracefully and is the default for `kill`?
- SIGKILL (9)
* SIGTERM (15)
- SIGHUP (1)
- SIGSTOP (19)
? SIGKILL can't be caught and gives no chance to clean up.

@@quiz
`ps` shows a process in state `Z`. What is it?
* A zombie: it has exited but its parent hasn't reaped it
- A process waiting for disk I/O
- A stopped process
- A kernel thread
? `D` is uninterruptible sleep (I/O).

@@quiz
Which crontab line runs a script at 09:00 Monday to Friday?
- `9 0 * * 1-5 /script`
* `0 9 * * 1-5 /script`
- `0 9 1-5 * * /script`
- `* 9 * * 1-5 /script`
? Minute, hour, day of month, month, day of week.

@@quiz
A 4-core server shows a load average of 12. What does that suggest?
- It's idle
* Work is queuing: about three times more runnable/waiting tasks than cores
- It has 12 users logged in
- The disk is 12% full
? Compare load with the core count.

@@quiz
Which nice value gives a process the lowest scheduling priority?
- -20
- 0
* 19
- 99
? Lower nice = higher priority.

@@flashcards
ps aux :: all processes, BSD style
ps -ef :: all processes, full format
pgrep / pkill :: find / signal by name
lsof -i :443 :: who uses port 443
STAT D :: uninterruptible sleep (I/O)
STAT Z :: zombie
load average :: runnable + waiting tasks over 1/5/15 min
SIGHUP :: reload config (many daemons)
SIGTERM 15 :: graceful stop
SIGKILL 9 :: forced, can't be caught
nice range :: -20 (high) to 19 (low)
Ctrl+Z :: suspend a job
bg / fg :: resume in background / foreground
nohup :: survive logout
crontab fields :: min hour dom month dow
*/15 :: every 15 units
at :: one-time scheduled job
systemd timer :: OnCalendar, Persistent=true

@@drill
What is the signal number of SIGKILL?
= 9
? SIGTERM is 15.

@@drill
Which command lists the current user's crontab?
= crontab -l
? `crontab -e` edits it.

@@exercise Find the top memory users
The variable `psout` holds `ps -eo user,pid,%mem,rss,comm` output. Skip the header, print the three processes using the most memory as `comm (pid) user rss_MB MB` with RSS converted from KiB to MiB with one decimal, then print the total RSS of all processes owned by `www-data` in MiB.
@@starter
psout='USER         PID %MEM   RSS COMMAND
root           1  0.1  12288 systemd
mysql        812 18.4 1507328 mysqld
www-data    1021  4.2  344064 php-fpm
www-data    1022  4.0  327680 php-fpm
root         640  0.6   49152 sshd
ana         2201 22.9 1875968 java
www-data     980  1.1   90112 nginx
root         720  2.3  188416 containerd'

echo "$psout"
@@expected
java (2201) ana 1832.0 MB
mysqld (812) mysql 1472.0 MB
php-fpm (1021) www-data 336.0 MB
www-data total: 744.0 MB
@@requires
NR > 1
sort -k4 -rn
@@hint
`awk 'NR > 1'` drops the header; `sort -k4 -rn | head -3` takes the top three by RSS; `printf "%.1f", $4/1024` converts.
@@solution
psout='USER         PID %MEM   RSS COMMAND
root           1  0.1  12288 systemd
mysql        812 18.4 1507328 mysqld
www-data    1021  4.2  344064 php-fpm
www-data    1022  4.0  327680 php-fpm
root         640  0.6   49152 sshd
ana         2201 22.9 1875968 java
www-data     980  1.1   90112 nginx
root         720  2.3  188416 containerd'

echo "$psout" | awk 'NR > 1' | sort -k4 -rn | head -3 | awk '{ printf "%s (%s) %s %.1f MB\n", $5, $2, $1, $4 / 1024 }'
echo "$psout" | awk 'NR > 1 && $1 == "www-data" { kb += $4 } END { printf "www-data total: %.1f MB\n", kb / 1024 }'

@@exercise Explain crontab entries
Write a bash function `explain` that reads a crontab line and prints it in words for these patterns only: minute and hour as numbers with `*` for day/month and a day-of-week of `*` (`daily at HH:MM`) or `1-5` (`weekdays at HH:MM`); `*/N` in the minute field with everything else `*` (`every N minutes`). Anything else prints `custom schedule`. Always append ` -> command`. Skip comment lines starting with `#`.
@@starter
crontab='# backups and checks
30 2 * * * /usr/local/bin/backup.sh
*/15 * * * * /usr/local/bin/check_disk.sh
0 9 * * 1-5 /usr/local/bin/report.sh
0 0 1 */3 * /usr/local/bin/quarterly.sh
5 7 * * * /usr/local/bin/rotate.sh'

explain() {
    echo "TODO"
}
@@expected
daily at 02:30 -> /usr/local/bin/backup.sh
every 15 minutes -> /usr/local/bin/check_disk.sh
weekdays at 09:00 -> /usr/local/bin/report.sh
custom schedule -> /usr/local/bin/quarterly.sh
daily at 07:05 -> /usr/local/bin/rotate.sh
@@requires
read -r min hour dom mon dow cmd
printf "%02d:%02d"
@@hint
Inside the loop: `while read -r min hour dom mon dow cmd; do ... done <<< "$crontab"`. Use `[[ $min == \*/* ]]` and `${min#*/}` for the step.
@@solution
crontab='# backups and checks
30 2 * * * /usr/local/bin/backup.sh
*/15 * * * * /usr/local/bin/check_disk.sh
0 9 * * 1-5 /usr/local/bin/report.sh
0 0 1 */3 * /usr/local/bin/quarterly.sh
5 7 * * * /usr/local/bin/rotate.sh'

explain() {
    local when
    if [[ $min == \*/* && $hour == "*" && $dom == "*" && $mon == "*" && $dow == "*" ]]; then
        when="every ${min#*/} minutes"
    elif [[ $min =~ ^[0-9]+$ && $hour =~ ^[0-9]+$ && $dom == "*" && $mon == "*" && $dow == "*" ]]; then
        when="daily at $(printf "%02d:%02d" "$hour" "$min")"
    elif [[ $min =~ ^[0-9]+$ && $hour =~ ^[0-9]+$ && $dom == "*" && $mon == "*" && $dow == "1-5" ]]; then
        when="weekdays at $(printf "%02d:%02d" "$hour" "$min")"
    else
        when="custom schedule"
    fi
    echo "$when -> $cmd"
}

while read -r min hour dom mon dow cmd; do
    [[ $min == \#* ]] && continue
    explain
done <<< "$crontab"

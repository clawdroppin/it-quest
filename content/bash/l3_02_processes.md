@@lesson l3-processes Processes, Services and Logs
@@concepts processes
@@text
These need a real Linux system (WSL, a VM or a cloud server), so the examples don't run here. The exercises practise the *thinking* with sample output.

## Processes

```bash,norun
ps aux                       # every process: user, PID, %CPU, %MEM, command
ps aux --sort=-%mem | head   # biggest memory users
pgrep -a nginx               # find processes by name
top                          # live view (q to quit); htop is friendlier if installed
kill 1234                    # ask process 1234 to stop (SIGTERM, 15)
kill -9 1234                 # force it (SIGKILL): last resort, no cleanup
pkill -f "python app.py"
uptime                       # load average for the last 1, 5, 15 minutes
free -h                      # memory
```

**Load average** is roughly "how many processes want the CPU". On a 4-core server, a load of 4 is fully busy; 12 means work is waiting.

Background jobs: `long_task &` starts in the background, `jobs` lists, `fg` brings one back, `nohup cmd &` survives logging out.

## systemd: services

Almost every modern distribution uses **systemd** to start and supervise services.

```bash,norun
systemctl status nginx
sudo systemctl start nginx       # stop, restart, reload
sudo systemctl enable nginx      # start at boot (enable --now: and right now)
systemctl list-units --type=service --state=failed
systemctl is-active sshd
```

A service is described by a **unit file**:

```ini
# /etc/systemd/system/myapp.service
[Unit]
Description=My web app
After=network.target

[Service]
User=myapp
ExecStart=/usr/bin/python3 /opt/myapp/app.py
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

After creating or changing one: `sudo systemctl daemon-reload`, then `enable --now`.

## Logs

```bash,norun
journalctl -u nginx              # one service's log
journalctl -u nginx -f           # follow live
journalctl -p err -b             # errors since the last boot
journalctl --since "1 hour ago"
tail -f /var/log/syslog          # Debian/Ubuntu (/var/log/messages on Red Hat)
less /var/log/auth.log           # logins and sudo (Red Hat: /var/log/secure)
dmesg | tail                     # kernel messages: disks, USB, hardware errors
```

## The troubleshooting routine

When a server "is slow" or "a site is down", work through it in order:

1. `uptime` and `top`: CPU or load?
2. `free -h`: out of memory? (`dmesg | grep -i "out of memory"` shows the OOM killer.)
3. `df -h` and `df -i`: disk or inodes full? A full `/var` stops databases and logging.
4. `systemctl status <service>` and `journalctl -u <service>`: is it running, and what did it last say?
5. `ss -tlnp`: is it listening on the expected port?

@@quiz
A process ignores `kill 4321`. What's the last-resort command?
* `kill -9 4321`
- `kill -1 4321`
- `stop 4321`
- `systemctl kill 4321`
? SIGKILL (9) can't be ignored, but the program gets no chance to clean up.

@@quiz
You installed nginx and want it to start now **and** after every reboot. Which command?
- `systemctl start nginx`
* `systemctl enable --now nginx`
- `systemctl daemon-reload nginx`
- `service nginx boot`
? `enable` adds it to boot; `--now` also starts it immediately.

@@quiz
Which command shows the log of the `sshd` service since the last boot?
- `cat /var/log/sshd`
* `journalctl -u sshd -b`
- `systemctl log sshd`
- `dmesg sshd`
? `-u` selects the unit, `-b` the current boot.

@@drill
Which command shows the logs for a systemd service?
= journalctl -u
= journalctl
? journalctl -u nginx --since today.

@@flashcards
ps aux :: List all processes
kill / kill -9 :: Ask a process to stop / force it
load average :: Processes wanting CPU, averaged over 1, 5, 15 minutes
systemctl status :: Is a service running, plus its latest log lines
systemctl enable --now :: Start a service now and at every boot
journalctl -u name -f :: Follow a service's log
/var/log/auth.log :: Logins and sudo on Debian/Ubuntu (/var/log/secure on Red Hat)
df -h / free -h :: Disk space / memory, human-readable
daemon-reload :: Reload systemd after changing unit files

@@exercise Hungry processes
`ps_output` imitates `ps aux` (user, pid, %cpu, %mem, command). Print the **PID and command** of every process using more than **10% memory**, as `pid command`, biggest memory first.
@@starter
ps_output='USER PID %CPU %MEM COMMAND
root 1 0.0 0.1 systemd
mysql 812 12.5 35.2 mysqld
www-data 1430 3.1 4.0 nginx
ana 2201 45.0 22.8 python3
root 900 0.5 10.0 snapd'

@@expected
812 mysqld
2201 python3
@@hint
`awk 'NR > 1 && $4 > 10 {print $4, $2, $5}' | sort -rn | awk '{print $2, $3}'`. Note "more than 10", so snapd at exactly 10.0 doesn't count.
@@solution
ps_output='USER PID %CPU %MEM COMMAND
root 1 0.0 0.1 systemd
mysql 812 12.5 35.2 mysqld
www-data 1430 3.1 4.0 nginx
ana 2201 45.0 22.8 python3
root 900 0.5 10.0 snapd'
echo "$ps_output" | awk 'NR > 1 && $4 > 10 {print $4, $2, $5}' | sort -rn | awk '{print $2, $3}'

@@exercise Failed units
`units` imitates `systemctl list-units --type=service --all --no-legend` (unit, load, active, sub). Print the names of units whose **active** state is `failed`, without the `.service` ending, then `N failed`.
@@starter
units='nginx.service loaded active running
mysql.service loaded failed failed
ssh.service loaded active running
backup.service loaded failed failed
cron.service loaded active running'

@@expected
mysql
backup
2 failed
@@hint
`awk '$3 == "failed" {sub(/\.service$/, "", $1); print $1; n++} END {print n " failed"}'`
@@solution
units='nginx.service loaded active running
mysql.service loaded failed failed
ssh.service loaded active running
backup.service loaded failed failed
cron.service loaded active running'
echo "$units" | awk '$3 == "failed" {sub(/\.service$/, "", $1); print $1; n++} END {print n " failed"}'

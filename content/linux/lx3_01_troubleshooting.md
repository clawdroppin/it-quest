@@lesson lx3-troubleshooting Linux Troubleshooting
@@lang bash
@@text
## Performance

```bash,norun
uptime                     # load averages for 1, 5, 15 minutes
top / htop                 # CPU, memory, processes (press M / P to sort)
free -h                    # memory and swap
vmstat 2                   # CPU, memory, swap in/out (si/so), I/O wait (wa)
iostat -xz 2               # disk utilisation and latency (sysstat package)
sar -u -r -d               # history (if sysstat collects it)
ps aux --sort=-%mem | head
nice / renice -n 10 -p PID # lower a process's priority
kill PID ; kill -9 PID     # TERM (polite) ; KILL (forced)
```

A **load average** higher than the number of CPU cores (`nproc`) means processes are waiting. High load with low CPU use usually means **I/O wait**: slow or overloaded disks. Lots of swapping (`si`/`so` in vmstat) means not enough RAM. The **OOM killer** kills processes when memory runs out: `journalctl -k | grep -i oom`.

## Common problems

| Symptom | Check |
|---|---|
| Disk full | `df -h`, `du -sh /* 2>/dev/null \| sort -h`, big logs, old kernels, `journalctl --vacuum-size=500M`; a deleted file still held open: `lsof +L1` |
| "No space" but `df -h` shows space | inodes: `df -i` |
| Service won't start | `systemctl status`, `journalctl -u`, config test (`nginx -t`, `sshd -t`), port already used (`ss -tlpn`), SELinux denials, permissions |
| Permission denied | owner/mode, ACLs, SELinux/AppArmor, the parent folder needs `x`, `noexec` mount option |
| Can't resolve names | `/etc/resolv.conf`, `resolvectl`, `dig @8.8.8.8 name` |
| Can't reach a service | is it listening (`ss`)? on the right address (127.0.0.1 vs 0.0.0.0)? firewall? routing? |
| Boot drops to emergency mode | bad `/etc/fstab` entry, failed file system check |
| Can't log in | locked account (`faillock`), expired password (`chage -l`), shell is nologin, home missing, SSH key permissions |
| Time wrong | `timedatectl`, `chronyc tracking` |
| Kernel panic after an update | boot the previous kernel from GRUB |

The habit: **read the actual error message**, then the logs (`journalctl -xe`), then change one thing at a time.

@@quiz
A 4-core server has load averages of 12, 11, 10, but CPU usage in top is only 15%. What's the most likely bottleneck?
* Disk I/O (processes waiting for storage)
- The CPU
- The network card
- Too few users
? High load with idle CPUs means processes are blocked, usually on I/O. Check iostat and the wa value.

@@quiz
A huge log file was deleted, but df still shows the disk full. Why?
* A process still has the deleted file open; restart it (find it with lsof +L1)
- df is cached for 24 hours
- Deleted files go to a trash folder
- The inode table is corrupt
? The space is freed only when the last open handle closes.

@@quiz
A web service is running but can't be reached from other machines. `ss -tlpn` shows it listening on 127.0.0.1:8080. What's wrong?
* It's bound to localhost only; configure it to listen on 0.0.0.0 or the server's IP
- The firewall is fine, the client is broken
- The service needs a reboot
- DNS is down
? 127.0.0.1 accepts only local connections.

@@quiz
After adding a disk to /etc/fstab, the server boots into emergency mode. What's the likely cause and fix?
* A wrong fstab entry; fix or comment it out from the emergency shell (or add nofail)
- A bad CPU; replace it
- An expired SSL certificate
- The firewall blocked the boot
? Always test with `mount -a` before rebooting.

@@quiz
Which command sends the gentle request to stop a process, allowing it to clean up?
* kill PID (SIGTERM)
- kill -9 PID
- nice PID
- renice PID
? -9 (SIGKILL) can't be caught or cleaned up after; use it last.

@@flashcards
load average :: Processes running or waiting; compare to nproc
vmstat :: CPU, memory, swap (si/so), I/O wait (wa)
iostat -xz :: Disk utilisation and latency
OOM killer :: Kills processes when memory runs out
lsof +L1 :: Deleted files still held open
SIGTERM vs SIGKILL :: kill (polite) vs kill -9 (forced)
renice :: Change a running process's priority
127.0.0.1 vs 0.0.0.0 :: Localhost only vs all interfaces
journalctl -xe :: Recent logs with explanations
timedatectl / chronyc :: Time and NTP sync status

@@drill
Which command shows how many CPU cores the system has?
= nproc
= lscpu
? Compare it with the load average.

@@exercise Pick the first check for each symptom
Turn tickets into a first step, using the lesson's symptom table. Write a function `first_check` that reads a ticket (case-insensitive) and prints the check for the first matching clue, in this order:

| Clue | First check |
|---|---|
| `no space` and `df` shows space | `df -i` (inodes) |
| `disk full` or `no space` | `df -h; du -sh /* \| sort -h` |
| `won't start` or `failed` | `systemctl status <unit>; journalctl -u <unit>` |
| `permission denied` | `namei -l <path>` (each folder needs x) |
| `resolve` or `dns` | `resolvectl status; dig @8.8.8.8 <name>` |
| `log in` or `login` | `faillock --user <user>; chage -l <user>` |
| `emergency mode` | `check /etc/fstab, then mount -a` |
| anything else | `read the error, then journalctl -xe` |
@@starter
tickets='Disk full on web01, uploads failing
No space left on device but df shows 40% used
nginx won'"'"'t start after the config change
Permission denied reading /srv/app/config.yml
Cannot resolve internal.example.com
User ksato cannot log in since Monday
Server boots into emergency mode
Clock is 5 minutes slow'

first_check() {
    : # your code here
}

while IFS= read -r t; do
    echo "$t -> $(first_check "$t")"
done <<< "$tickets"
@@expected
Disk full on web01, uploads failing -> df -h; du -sh /* | sort -h
No space left on device but df shows 40% used -> df -i (inodes)
nginx won't start after the config change -> systemctl status <unit>; journalctl -u <unit>
Permission denied reading /srv/app/config.yml -> namei -l <path> (each folder needs x)
Cannot resolve internal.example.com -> resolvectl status; dig @8.8.8.8 <name>
User ksato cannot log in since Monday -> faillock --user <user>; chage -l <user>
Server boots into emergency mode -> check /etc/fstab, then mount -a
Clock is 5 minutes slow -> read the error, then journalctl -xe
@@requires
${1,,}
df -i
@@hint
Lowercase once with `local t=${1,,}`, then test the clues in order with `[[ $t == *"no space"* ]]`. The inode case must come before the plain disk-full case.
@@solution
tickets='Disk full on web01, uploads failing
No space left on device but df shows 40% used
nginx won'"'"'t start after the config change
Permission denied reading /srv/app/config.yml
Cannot resolve internal.example.com
User ksato cannot log in since Monday
Server boots into emergency mode
Clock is 5 minutes slow'

first_check() {
    local t=${1,,}
    if [[ $t == *"no space"* && $t == *"df"* ]]; then echo "df -i (inodes)"
    elif [[ $t == *"disk full"* || $t == *"no space"* ]]; then echo "df -h; du -sh /* | sort -h"
    elif [[ $t == *"won't start"* || $t == *"failed"* ]]; then echo "systemctl status <unit>; journalctl -u <unit>"
    elif [[ $t == *"permission denied"* ]]; then echo "namei -l <path> (each folder needs x)"
    elif [[ $t == *"resolve"* || $t == *"dns"* ]]; then echo "resolvectl status; dig @8.8.8.8 <name>"
    elif [[ $t == *"log in"* || $t == *"login"* ]]; then echo "faillock --user <user>; chage -l <user>"
    elif [[ $t == *"emergency mode"* ]]; then echo "check /etc/fstab, then mount -a"
    else echo "read the error, then journalctl -xe"
    fi
}

while IFS= read -r t; do
    echo "$t -> $(first_check "$t")"
done <<< "$tickets"

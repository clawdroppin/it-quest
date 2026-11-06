@@lesson l0-filesystem Files, Folders and the Linux File System
@@concepts files
@@text
## One tree, starting at /

Linux has no `C:` or `D:`. Everything is in one tree that starts at `/` (the **root**); disks are *mounted* into folders. Knowing where things live is a Linux+ exam favourite:

| Path | Holds |
|---|---|
| `/etc` | configuration files (`/etc/ssh/sshd_config`, `/etc/passwd`) |
| `/var/log` | logs (`/var/log/syslog`, `/var/log/auth.log`) |
| `/home/ana` | users' home folders (`~` is yours) |
| `/root` | the root user's home |
| `/bin`, `/usr/bin` | programs |
| `/tmp` | temporary files, often cleared at boot |
| `/dev` | devices (`/dev/sda` is the first disk) |
| `/proc` | live kernel and process info (`/proc/cpuinfo`) |
| `/mnt`, `/media` | mounted disks and USB sticks |
| `/opt` | extra software |

**Absolute** paths start at `/` (`/var/log/syslog`). **Relative** paths start where you are (`logs/app.log`). `.` is the current folder, `..` its parent, `~` your home.

## Moving around and looking

```bash
cd /tmp
pwd
mkdir -p itq-demo/logs/old
cd itq-demo
ls -l
cd ..
ls -la itq-demo
rm -r itq-demo
```

`ls -l` shows permissions, owner, size and date; `-a` shows **hidden** files (names starting with `.`, like `.bashrc`); `-h` makes sizes human-readable.

## Working with files

```bash
cd /tmp && mkdir -p itq-files && cd itq-files
echo "server=web01" > config.txt     # > writes (overwrites!) a file
echo "port=8080" >> config.txt       # >> appends
cat config.txt
cp config.txt config.bak
mv config.bak backup.conf
touch empty.log                      # create an empty file (or update its time)
ls
rm config.txt backup.conf empty.log
cd .. && rmdir itq-files
```

| Command | Does |
|---|---|
| `cat`, `less` | show a file (all at once / page by page, `q` quits) |
| `head -n 5`, `tail -n 5` | first / last lines; `tail -f` follows a growing log live |
| `cp -r`, `mv`, `rm -r` | copy, move or rename, delete (folders need `-r`) |
| `mkdir -p` | make folders, including missing parents |
| `touch` | create an empty file |
| `wc -l` | count lines |
| `du -sh`, `df -h` | folder size, free disk space |

> ⚠ `rm` has no recycle bin. `rm -rf /some/path` deletes everything below that path immediately. Double-check paths, especially ones built from variables.

## Wildcards (globbing)

The shell expands patterns **before** the command runs:

```bash
cd /tmp && mkdir -p itq-glob && cd itq-glob
touch app1.log app2.log app10.log notes.txt
echo *.log
echo app?.log
echo app[12].log
cd .. && rm -r itq-glob
```

`*` matches anything, `?` one character, `[12]` one of these.

@@quiz
Where do you look for system configuration files on Linux?
* `/etc`
- `/var`
- `/bin`
- `/home`
? `/etc` holds configuration. Logs are in `/var/log`.

@@quiz
`echo "x" > file.txt` when `file.txt` already has 100 lines. What happens?
- "x" is added at the end
* the file now contains only "x"
- an error: the file exists
- nothing
? `>` overwrites. `>>` appends.

@@quiz
What does `ls *.log` list?
* Files in the current folder whose names end in .log
- Every log file on the system
- A file literally named *.log only
- Folders called log
? The shell expands the wildcard before ls even runs.

@@drill
Which command shows the **last 20 lines** of `/var/log/syslog`?
= tail -n 20 /var/log/syslog
= tail -20 /var/log/syslog
= tail -n20 /var/log/syslog
? `tail` shows the end of a file; `-n 20` sets how many lines.

@@flashcards
/etc :: Configuration files
/var/log :: Log files
/home :: Users' home folders
/tmp :: Temporary files
/proc :: Live kernel and process information
~ :: Your home folder
. and .. :: The current folder and its parent
tail -f :: Follow a log file as new lines are written
df -h / du -sh :: Free space per file system / size of a folder

@@exercise Build a folder structure
Create the folders `/tmp/itq-site/www/images` and `/tmp/itq-site/logs` (in **one** `mkdir` command), create an empty file `/tmp/itq-site/logs/access.log`, then print the folder tree with `find /tmp/itq-site | sort`. The cleanup line at the end is already there.
@@starter
rm -rf /tmp/itq-site
# your commands here

rm -rf /tmp/itq-site
@@expected
/tmp/itq-site
/tmp/itq-site/logs
/tmp/itq-site/logs/access.log
/tmp/itq-site/www
/tmp/itq-site/www/images
@@hint
`mkdir -p /tmp/itq-site/www/images /tmp/itq-site/logs`, then `touch` the log, then `find /tmp/itq-site | sort`.
@@solution
rm -rf /tmp/itq-site
mkdir -p /tmp/itq-site/www/images /tmp/itq-site/logs
touch /tmp/itq-site/logs/access.log
find /tmp/itq-site | sort
rm -rf /tmp/itq-site

@@exercise Write and read a config
Write the two lines `user=admin` and `port=2222` into `/tmp/itq.conf` (first with `>`, then `>>`), print the file, then print its line count with `wc -l < /tmp/itq.conf`.
@@starter
# write /tmp/itq.conf here

rm -f /tmp/itq.conf
@@expected
user=admin
port=2222
2
@@hint
`echo "user=admin" > /tmp/itq.conf`, `echo "port=2222" >> /tmp/itq.conf`, `cat /tmp/itq.conf`, `wc -l < /tmp/itq.conf`.
@@solution
echo "user=admin" > /tmp/itq.conf
echo "port=2222" >> /tmp/itq.conf
cat /tmp/itq.conf
wc -l < /tmp/itq.conf
rm -f /tmp/itq.conf

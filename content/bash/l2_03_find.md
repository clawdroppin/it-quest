@@lesson l2-find find, xargs and Archives
@@concepts files, automation
@@text
## find: search the file system by name, size, age, type

```bash
mkdir -p /tmp/itq-find/logs/old && cd /tmp/itq-find
echo "x" > logs/app.log; echo "y" > logs/old/app-2025.log; echo "z" > notes.txt
touch -d "40 days ago" logs/old/app-2025.log
find . -name "*.log"
find . -type d
find . -name "*.log" -mtime +30
cd / && rm -rf /tmp/itq-find
```

| Test | Matches |
|---|---|
| `-name "*.log"` / `-iname` | name (case-insensitive with `-iname`) |
| `-type f` / `-type d` | files / directories |
| `-size +100M` | bigger than 100 MB (`-size -1k`: smaller than 1 KB) |
| `-mtime +30` | modified more than 30 days ago (`-mtime -1`: within the last day) |
| `-user ana` | owned by ana |
| `-perm -o+w` | writable by everyone (a security audit classic) |
| `-maxdepth 1` | don't go deeper than this |

Actions: `-print` (default), `-delete`, `-exec cmd {} \;` (once per file), `-exec cmd {} +` (many files at once).

```bash,norun
find /var/log -name "*.log" -mtime +30 -delete                 # clean old logs
find /home -type f -size +1G -exec ls -lh {} +                 # who's filling the disk?
find / -type f -perm -o+w -not -path "/proc/*" 2>/dev/null     # world-writable files
find /etc -name "*.conf" -newer /tmp/marker                    # changed since the marker file
```

## xargs: turn input into arguments

```bash
printf "a.txt\nb.txt\n" | xargs echo "files:"
printf "web01\nweb02\n" | xargs -I{} echo "ssh {} uptime"
```

`find ... -print0 | xargs -0 cmd` is the safe pair for names with spaces. `xargs -P 4` runs 4 at once.

## Archives: tar and gzip

```bash
mkdir -p /tmp/itq-tar/site && echo "<h1>hi</h1>" > /tmp/itq-tar/site/index.html
cd /tmp/itq-tar
tar -czf site-backup.tar.gz site        # c=create z=gzip f=file
tar -tzf site-backup.tar.gz             # t=list
rm -r site && tar -xzf site-backup.tar.gz && cat site/index.html    # x=extract
cd / && rm -rf /tmp/itq-tar
```

Memory aid: **c**reate, e**x**tract, lis**t**, with **z** for gzip and **f** followed by the file name. `gzip file` / `gunzip file.gz` compress single files; `zcat` and `zgrep` read `.gz` logs without unpacking them.

@@quiz
Which finds files under `/var/log` modified more than 7 days ago?
* `find /var/log -type f -mtime +7`
- `find /var/log -mtime -7`
- `find /var/log -age 7d`
- `ls -t /var/log | tail -7`
? `+7` is "more than 7 days ago", `-7` "within the last 7 days".

@@quiz
Which `tar` command **extracts** `backup.tar.gz`?
- `tar -czf backup.tar.gz`
* `tar -xzf backup.tar.gz`
- `tar -tzf backup.tar.gz`
- `gzip -x backup.tar.gz`
? x = extract, z = gzip, f = file. `-c` creates, `-t` lists.

@@quiz
Why use `find ... -print0 | xargs -0` instead of plain `| xargs`?
* File names with spaces or newlines are passed safely
- It's faster
- It deletes files
- It sorts the results
? -print0 separates names with a null byte, which can't appear in a file name.

@@flashcards
find -name :: Find by name, e.g. find /etc -name "*.conf"
find -mtime +30 :: Modified more than 30 days ago
find -size +1G :: Bigger than 1 GB
find -perm -o+w :: World-writable: a security risk
find -exec cmd {} + :: Run a command on the found files
xargs :: Turn lines of input into command arguments
tar -czf / -xzf / -tzf :: Create / extract / list a .tar.gz
zgrep :: grep inside .gz files

@@exercise Old logs
The script creates some log files with different ages. Using **find**, print the paths of `.log` files (not other files) older than **30 days**, sorted.
@@starter
rm -rf /tmp/itq-old && mkdir -p /tmp/itq-old/app /tmp/itq-old/web
touch /tmp/itq-old/app/today.log /tmp/itq-old/web/new.log
touch -d "45 days ago" /tmp/itq-old/app/september.log /tmp/itq-old/web/access-old.log /tmp/itq-old/web/old-notes.txt
touch -d "10 days ago" /tmp/itq-old/app/recent.log
# your find command here

rm -rf /tmp/itq-old
@@expected
/tmp/itq-old/app/september.log
/tmp/itq-old/web/access-old.log
@@hint
`find /tmp/itq-old -type f -name "*.log" -mtime +30 | sort`
@@solution
rm -rf /tmp/itq-old && mkdir -p /tmp/itq-old/app /tmp/itq-old/web
touch /tmp/itq-old/app/today.log /tmp/itq-old/web/new.log
touch -d "45 days ago" /tmp/itq-old/app/september.log /tmp/itq-old/web/access-old.log /tmp/itq-old/web/old-notes.txt
touch -d "10 days ago" /tmp/itq-old/app/recent.log
find /tmp/itq-old -type f -name "*.log" -mtime +30 | sort
rm -rf /tmp/itq-old

@@exercise Backup and verify
Create a gzipped tar archive `/tmp/itq-b/etc-backup.tar.gz` of the folder `conf` (run tar from inside `/tmp/itq-b`, so the paths in the archive start with `conf/`), then **list** the archive's contents, sorted.
@@starter
rm -rf /tmp/itq-b && mkdir -p /tmp/itq-b/conf/ssh
echo "Port 22" > /tmp/itq-b/conf/ssh/sshd_config
echo "127.0.0.1 localhost" > /tmp/itq-b/conf/hosts
cd /tmp/itq-b
# create and list the archive here

cd / && rm -rf /tmp/itq-b
@@expected
conf/
conf/hosts
conf/ssh/
conf/ssh/sshd_config
@@hint
`tar -czf etc-backup.tar.gz conf`, then `tar -tzf etc-backup.tar.gz | sort`.
@@solution
rm -rf /tmp/itq-b && mkdir -p /tmp/itq-b/conf/ssh
echo "Port 22" > /tmp/itq-b/conf/ssh/sshd_config
echo "127.0.0.1 localhost" > /tmp/itq-b/conf/hosts
cd /tmp/itq-b
tar -czf etc-backup.tar.gz conf
tar -tzf etc-backup.tar.gz | sort
cd / && rm -rf /tmp/itq-b

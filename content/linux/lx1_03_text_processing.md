@@lesson lx1-text Files, Text Processing and Redirection
@@lang bash
@@text
Linux administration is mostly reading and changing text: config files, logs, command output. Linux+ expects you to find files, filter and transform text with the standard tools, and chain them with pipes and redirection. These are also the skills every later lesson relies on.

## Finding files

```bash,norun
find /var/log -name "*.log" -mtime -1          # changed in the last day
find / -type f -size +500M 2>/dev/null         # big files, hide permission errors
find /home -user ana -perm -o+w                # world-writable files owned by ana
find /etc -name "*.conf" -exec grep -l "Listen" {} +
locate sshd_config                             # fast, uses a database (updatedb)
which python3; type ls; whereis ssh
```

## Viewing and comparing

`cat`, `less` (search with `/`, quit with `q`), `head -n 20`, `tail -n 50`, `tail -f /var/log/syslog` (follow), `diff -u old new`, `wc -l`, `file` (identify type), `stat` (times, permissions, inode).

## The text toolkit

| Tool | Does | Example |
|---|---|---|
| `grep` | match lines | `grep -i "error" app.log`, `-v` invert, `-c` count, `-r` recursive, `-E` extended regex, `-n` line numbers |
| `cut` | pick columns | `cut -d: -f1,7 /etc/passwd` |
| `awk` | fields, conditions, sums | `awk -F: '$3 >= 1000 {print $1}' /etc/passwd` |
| `sed` | stream edit | `sed 's/http:/https:/g' file`, `sed -i` in place, `sed -n '10,20p'` |
| `sort` | sort | `-n` numeric, `-r` reverse, `-k2` by field 2, `-t:` delimiter, `-u` unique |
| `uniq` | collapse **adjacent** duplicates | `sort | uniq -c | sort -rn` (the classic "top N") |
| `tr` | translate characters | `tr 'a-z' 'A-Z'`, `tr -d '\r'` (strip Windows line endings) |
| `paste`, `join` | merge files side by side / by key | |
| `xargs` | turn input into arguments | `find . -name "*.tmp" | xargs rm` |
| `tee` | write to a file and the screen | `cmd | tee out.log` |

## Redirection and pipes

| Syntax | Meaning |
|---|---|
| `>` | stdout to a file (overwrite) |
| `>>` | append |
| `2>` | stderr to a file |
| `2>&1` | stderr to wherever stdout goes |
| `&>` | both to a file (bash) |
| `<` | file as stdin |
| `<<EOF ... EOF` | here document |
| `<<<` | here string |
| `|` | pipe stdout to the next command |
| `/dev/null` | discard |

Exit status: `$?` is 0 for success. `cmd1 && cmd2` runs cmd2 only if cmd1 succeeded; `cmd1 || cmd2` only if it failed.

## Regular expressions (basic)

`^` start, `$` end, `.` any character, `*` zero or more, `+` one or more (ERE), `[0-9]` a class, `[^a-z]` negated, `\b` word boundary (GNU), `( | )` groups and alternation (ERE). Example: `grep -E '^[0-9]{1,3}(\.[0-9]{1,3}){3}' access.log` finds lines starting with an IPv4 address.

## Editors

**vi/vim** is on every system: `i` insert, `Esc` normal mode, `:w` save, `:q!` quit without saving, `:wq` save and quit, `/text` search, `dd` delete line, `yy` copy, `p` paste, `u` undo. **nano** is simpler (`Ctrl+O` save, `Ctrl+X` exit).

## Archiving and compression

`tar -czf backup.tar.gz /etc` (create gzip), `tar -xzf file.tar.gz -C /tmp` (extract), `tar -tf` (list), `-j` bzip2, `-J` xz. `gzip`/`gunzip`, `zip`/`unzip`, `cpio`, `dd if=/dev/sda of=disk.img bs=4M` (block copy: careful!), `rsync -av --delete src/ dest/` (sync, efficient over SSH).

## Links and inodes

**Hard link** (`ln file link`): another name for the same inode; can't cross file systems or point to directories. **Symbolic link** (`ln -s target link`): a pointer to a path; can break if the target moves.

@@quiz
Which command shows the 10 most frequent IP addresses at the start of each line in access.log?
* `cut -d' ' -f1 access.log | sort | uniq -c | sort -rn | head`
- `uniq -c access.log | head`
- `grep -c ip access.log`
- `sort -u access.log | tail`
? uniq only collapses adjacent lines, so sort first.

@@quiz
What does `2>&1` do in `cmd > out.log 2>&1`?
- Sends stdout to stderr
* Sends stderr to the same place as stdout (out.log)
- Appends to out.log
- Discards errors
? Order matters: redirect stdout first.

@@quiz
Which command replaces every `http:` with `https:` in a file, editing it in place?
- `grep -r http: file`
* `sed -i 's/http:/https:/g' file`
- `tr http https < file`
- `awk 's/http/https/' file`
? `-i` edits in place; `g` replaces every match on a line.

@@quiz
What breaks if you move the target of a symbolic link?
* The symbolic link (it points to a path)
- Hard links to the file
- The inode
- Nothing
? Hard links point to the inode, so they survive moves.

@@quiz
In vim, how do you quit without saving changes?
- `:wq`
- `:w`
* `:q!`
- `Ctrl+X`
? `:wq` saves and quits.

@@flashcards
find -mtime -1 :: modified in the last day
find -exec ... {} + :: run a command on the results
grep -v :: invert match
grep -c :: count matching lines
cut -d: -f1 :: first colon-separated field
awk -F: :: set the field separator
sed -n '10,20p' :: print lines 10–20
sort -k2 -n :: numeric sort on field 2
uniq -c :: count adjacent duplicates
tr -d '\r' :: strip Windows line endings
xargs :: input lines become arguments
tee :: write to file and stdout
2>&1 :: stderr to stdout's destination
&& / || :: run next if success / failure
tar -czf :: create a gzip archive
rsync -av :: sync preserving attributes
hard link :: same inode, same file system
ln -s :: symbolic link

@@drill
Which command counts the lines in a file?
= wc -l
? `wc -w` counts words.

@@drill
Which tar option extracts an archive?
= -x
= x
? `-c` creates, `-t` lists.

@@exercise Top talkers in a web log
The web access log is in the variable `log`. Print the three IP addresses (first field) with the most requests, as `count ip`, most first. Use a pipeline with `sort`, `uniq -c` and `head`, and trim the leading spaces `uniq -c` adds.
@@starter
log='203.0.113.5 - - [04/Nov/2026:09:00:01] "GET / HTTP/1.1" 200
198.51.100.7 - - [04/Nov/2026:09:00:02] "GET /login HTTP/1.1" 200
203.0.113.5 - - [04/Nov/2026:09:00:03] "POST /login HTTP/1.1" 401
192.0.2.44 - - [04/Nov/2026:09:00:04] "GET /about HTTP/1.1" 200
203.0.113.5 - - [04/Nov/2026:09:00:05] "POST /login HTTP/1.1" 401
198.51.100.7 - - [04/Nov/2026:09:00:06] "GET /cart HTTP/1.1" 200
203.0.113.5 - - [04/Nov/2026:09:00:07] "POST /login HTTP/1.1" 401
10.0.0.8 - - [04/Nov/2026:09:00:08] "GET /health HTTP/1.1" 200
192.0.2.44 - - [04/Nov/2026:09:00:09] "GET /contact HTTP/1.1" 200
198.51.100.7 - - [04/Nov/2026:09:00:10] "GET /checkout HTTP/1.1" 200'

echo "$log" | cut -d' ' -f1
@@expected
4 203.0.113.5
3 198.51.100.7
2 192.0.2.44
@@requires
uniq -c
sort -rn
@@hint
`cut -d' ' -f1 | sort | uniq -c | sort -rn | head -3`, then `awk '{print $1, $2}'` to drop the padding.
@@solution
log='203.0.113.5 - - [04/Nov/2026:09:00:01] "GET / HTTP/1.1" 200
198.51.100.7 - - [04/Nov/2026:09:00:02] "GET /login HTTP/1.1" 200
203.0.113.5 - - [04/Nov/2026:09:00:03] "POST /login HTTP/1.1" 401
192.0.2.44 - - [04/Nov/2026:09:00:04] "GET /about HTTP/1.1" 200
203.0.113.5 - - [04/Nov/2026:09:00:05] "POST /login HTTP/1.1" 401
198.51.100.7 - - [04/Nov/2026:09:00:06] "GET /cart HTTP/1.1" 200
203.0.113.5 - - [04/Nov/2026:09:00:07] "POST /login HTTP/1.1" 401
10.0.0.8 - - [04/Nov/2026:09:00:08] "GET /health HTTP/1.1" 200
192.0.2.44 - - [04/Nov/2026:09:00:09] "GET /contact HTTP/1.1" 200
198.51.100.7 - - [04/Nov/2026:09:00:10] "GET /checkout HTTP/1.1" 200'

echo "$log" | cut -d' ' -f1 | sort | uniq -c | sort -rn | head -3 | awk '{print $1, $2}'

@@exercise Report failed logins with awk
The `auth` variable holds SSH log lines. Using a single `awk` program, count failed password attempts per user (the word after `for`, skipping the words `invalid user`), and print `user failures` for users with 3 or more failures, sorted by user name. Finish with the total number of failed attempts.
@@starter
auth='Nov  4 09:01:10 web1 sshd[812]: Failed password for root from 203.0.113.9 port 51022 ssh2
Nov  4 09:01:14 web1 sshd[812]: Failed password for root from 203.0.113.9 port 51023 ssh2
Nov  4 09:01:20 web1 sshd[815]: Accepted publickey for ana from 10.0.0.12 port 50110 ssh2
Nov  4 09:02:01 web1 sshd[820]: Failed password for invalid user admin from 203.0.113.9 port 51100 ssh2
Nov  4 09:02:05 web1 sshd[820]: Failed password for invalid user admin from 203.0.113.9 port 51101 ssh2
Nov  4 09:02:09 web1 sshd[820]: Failed password for invalid user admin from 203.0.113.9 port 51102 ssh2
Nov  4 09:03:30 web1 sshd[830]: Failed password for ken from 10.0.0.15 port 50200 ssh2
Nov  4 09:03:40 web1 sshd[812]: Failed password for root from 203.0.113.9 port 51024 ssh2'

echo "$auth" | awk '/Failed password/ { }'
@@expected
admin 3
root 3
total 7
@@requires
/Failed password/
END
@@hint
Find the field index of `for`; if the next field is `invalid`, the user is two fields later. Store counts in an array and loop over it in `END`. Pipe the per-user lines to `sort` before printing the total, or use `asorti` (gawk) — the simplest is to print users in END and sort outside, then print the total separately.
@@solution
auth='Nov  4 09:01:10 web1 sshd[812]: Failed password for root from 203.0.113.9 port 51022 ssh2
Nov  4 09:01:14 web1 sshd[812]: Failed password for root from 203.0.113.9 port 51023 ssh2
Nov  4 09:01:20 web1 sshd[815]: Accepted publickey for ana from 10.0.0.12 port 50110 ssh2
Nov  4 09:02:01 web1 sshd[820]: Failed password for invalid user admin from 203.0.113.9 port 51100 ssh2
Nov  4 09:02:05 web1 sshd[820]: Failed password for invalid user admin from 203.0.113.9 port 51101 ssh2
Nov  4 09:02:09 web1 sshd[820]: Failed password for invalid user admin from 203.0.113.9 port 51102 ssh2
Nov  4 09:03:30 web1 sshd[830]: Failed password for ken from 10.0.0.15 port 50200 ssh2
Nov  4 09:03:40 web1 sshd[812]: Failed password for root from 203.0.113.9 port 51024 ssh2'

echo "$auth" | awk '/Failed password/ {
    for (i = 1; i <= NF; i++) if ($i == "for") break
    user = ($(i + 1) == "invalid") ? $(i + 3) : $(i + 1)
    count[user]++
}
END { for (u in count) if (count[u] >= 3) print u, count[u] }' | sort
echo "$auth" | awk '/Failed password/ { total++ } END { print "total", total }'

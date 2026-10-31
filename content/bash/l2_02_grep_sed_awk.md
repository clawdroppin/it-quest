@@lesson l2-grep-sed-awk grep, sed and awk
@@concepts text-processing
@@text
Three tools, decades old, still used every single day.

## grep: find lines

```bash
cat > /tmp/itq-auth.log <<'EOF'
Oct 31 08:15:02 web01 sshd[1234]: Failed password for root from 203.0.113.7 port 52211 ssh2
Oct 31 08:15:09 web01 sshd[1234]: Failed password for invalid user admin from 203.0.113.7 port 52213 ssh2
Oct 31 08:20:41 web01 sshd[1301]: Accepted publickey for ana from 10.0.0.4 port 50022 ssh2
Oct 31 09:02:44 web01 sshd[1290]: Failed password for ben from 198.51.100.23 port 40001 ssh2
Oct 31 09:03:04 web01 sshd[1290]: Accepted password for ben from 198.51.100.23 port 40005 ssh2
EOF
grep "Failed" /tmp/itq-auth.log | wc -l
grep -c "Accepted" /tmp/itq-auth.log
grep -i "ROOT" /tmp/itq-auth.log
grep -v "Failed" /tmp/itq-auth.log
grep -oE "from [0-9.]+" /tmp/itq-auth.log | sort | uniq -c
```

| Option | Means |
|---|---|
| `-i` | ignore case |
| `-v` | lines that do **not** match |
| `-c` | count matching lines |
| `-n` | show line numbers |
| `-r` | search folders recursively (`grep -rn "password" /etc`) |
| `-E` | extended regex: `+`, `?`, `\|`, `{n}` without backslashes |
| `-o` | print only the matching part |
| `-q` | quiet: just the exit code, for `if` |
| `-A 2`, `-B 2` | also 2 lines after / before each match |

## sed: edit streams

The classic use is **search and replace**: `s/old/new/` (add `g` for every match on a line):

```bash
echo "Port 22" | sed 's/22/2222/'
echo "a-b-c" | sed 's/-/:/g'
sed -n '2,3p' /tmp/itq-auth.log | cut -c1-40     # print only lines 2-3
sed '/Failed/d' /tmp/itq-auth.log | wc -l       # delete matching lines
```

`sed -i` edits a file **in place**, how config changes are scripted on servers:

```bash,norun
sudo sed -i 's/^#\?PermitRootLogin.*/PermitRootLogin no/' /etc/ssh/sshd_config
sudo sed -i.bak 's/old/new/' file.conf     # keep a backup as file.conf.bak
```

## awk: columns and calculations

awk splits each line into fields `$1`, `$2`, ... (`$NF` is the last) and runs a program on each line:

```bash
awk '{print $1, $2, $3}' /tmp/itq-auth.log | head -n 2
awk '/Failed/ {print $(NF-3)}' /tmp/itq-auth.log          # the IP: 4th field from the end
awk '/Failed/ {count[$(NF-3)]++} END {for (ip in count) print count[ip], ip}' /tmp/itq-auth.log | sort -rn
printf "C: 476 38\nD: 931 610\n" | awk '{printf "%s %.1f%% free\n", $1, $3 / $2 * 100}'
awk -F: '$3 >= 1000 {print $1}' /etc/passwd     # -F sets the separator: regular users
rm /tmp/itq-auth.log
```

- `pattern { action }`: run the action on lines that match the pattern (no pattern: every line).
- `BEGIN { }` runs before the first line, `END { }` after the last: great for totals.
- `count[key]++` builds a counting table, like a Python dict.
- `NR` is the line number, `NF` the number of fields.

@@quiz
Which command prints lines of `app.log` that do **not** contain `DEBUG`?
* `grep -v DEBUG app.log`
- `grep -n DEBUG app.log`
- `grep DEBUG -c app.log`
- `sed DEBUG app.log`
? `-v` inverts the match.

@@quiz
In awk, what is `$NF`?
- the line number
* the last field of the line
- the number of fields
- the file name
? `NF` is the number of fields, so `$NF` is the last one. `NR` is the line number.

@@drill
Write the sed command (reading from a pipe) that replaces **every** `http://` with `https://` on each line.
= sed 's|http://|https://|g'
= sed 's/http:\/\//https:\/\//g'
= sed "s|http://|https://|g"
= sed 's#http://#https://#g'
? You can use any separator instead of `/`, which saves escaping slashes: `s|http://|https://|g`.

@@exercise Failed login sources
From the auth log lines in `log`, print each IP that had a **Failed password**, with its count, as `count ip`, most first.
@@starter
log='Oct 31 08:15:02 web01 sshd[1]: Failed password for root from 203.0.113.7 port 1 ssh2
Oct 31 08:15:09 web01 sshd[1]: Failed password for invalid user admin from 203.0.113.7 port 2 ssh2
Oct 31 08:20:41 web01 sshd[2]: Accepted publickey for ana from 10.0.0.4 port 3 ssh2
Oct 31 09:02:44 web01 sshd[3]: Failed password for ben from 198.51.100.23 port 4 ssh2
Oct 31 09:02:50 web01 sshd[3]: Failed password for root from 203.0.113.7 port 5 ssh2'

@@expected
3 203.0.113.7
1 198.51.100.23
@@hint
`echo "$log" | grep "Failed password" | grep -oE "from [0-9.]+" | cut -d' ' -f2 | sort | uniq -c | sort -rn | awk '{print $1, $2}'`. Or do it all in awk with a counting array.
@@solution
log='Oct 31 08:15:02 web01 sshd[1]: Failed password for root from 203.0.113.7 port 1 ssh2
Oct 31 08:15:09 web01 sshd[1]: Failed password for invalid user admin from 203.0.113.7 port 2 ssh2
Oct 31 08:20:41 web01 sshd[2]: Accepted publickey for ana from 10.0.0.4 port 3 ssh2
Oct 31 09:02:44 web01 sshd[3]: Failed password for ben from 198.51.100.23 port 4 ssh2
Oct 31 09:02:50 web01 sshd[3]: Failed password for root from 203.0.113.7 port 5 ssh2'
echo "$log" | grep "Failed password" | grep -oE "from [0-9.]+" | cut -d' ' -f2 | sort | uniq -c | sort -rn | awk '{print $1, $2}'

@@exercise Harden a config
`config` is part of an `sshd_config`. With **sed**, change it so that `PermitRootLogin` is `no` and `PasswordAuthentication` is `no`, also when the line is commented out with `#`. Print the result.
@@starter
config='Port 22
#PermitRootLogin yes
PasswordAuthentication yes
UsePAM yes'

@@expected
Port 22
PermitRootLogin no
PasswordAuthentication no
UsePAM yes
@@requires
sed
@@hint
`sed -e 's/^#\?PermitRootLogin.*/PermitRootLogin no/' -e 's/^#\?PasswordAuthentication.*/PasswordAuthentication no/'`. `#\?` means "an optional #".
@@solution
config='Port 22
#PermitRootLogin yes
PasswordAuthentication yes
UsePAM yes'
echo "$config" | sed -e 's/^#\?PermitRootLogin.*/PermitRootLogin no/' \
                     -e 's/^#\?PasswordAuthentication.*/PasswordAuthentication no/'

@@exercise Free space report
`df_output` imitates `df` (size and available in GB). With **awk**, print `<mount> <used%>%` for every line **except the header**, where used% = (size - avail) / size * 100, rounded to a whole number. Then a last line `total free: <sum of avail> GB`.
@@starter
df_output='Filesystem Size Avail Mount
/dev/sda1 100 20 /
/dev/sda2 500 450 /home
/dev/sdb1 200 10 /var'

@@expected
/ 80%
/home 10%
/var 95%
total free: 480 GB
@@hint
`awk 'NR > 1 { printf "%s %.0f%%\n", $4, ($2 - $3) / $2 * 100; free += $3 } END { print "total free: " free " GB" }'`
@@solution
df_output='Filesystem Size Avail Mount
/dev/sda1 100 20 /
/dev/sda2 500 450 /home
/dev/sdb1 200 10 /var'
echo "$df_output" | awk 'NR > 1 { printf "%s %.0f%%\n", $4, ($2 - $3) / $2 * 100; free += $3 } END { print "total free: " free " GB" }'

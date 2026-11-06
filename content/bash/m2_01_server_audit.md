@@lesson m2-server-audit Linux Server Security Audit
@@text
Security teams don't start with fancy tools. They start with a checklist: *Can root log in over SSH? Is there a second account with UID 0? Accounts without passwords? Service accounts with login shells? sudo without a password?* Each of those has been the way into a real breached server.

In this build you'll write **audit.sh**, a script that checks a Linux system's configuration files and gives it a score:

```text
$ ./audit.sh sample_root
FAIL ssh: PermitRootLogin is yes
WARN ssh: PasswordAuthentication is yes (default)
WARN ssh: X11Forwarding is yes
FAIL uid0: toor has UID 0
...
Score: 15/100
```

The trick that makes it testable on Windows: every check takes a **root folder** as its argument and reads `<root>/etc/passwd`, `<root>/etc/shadow` and so on. The project includes `sample_root/`, a fake system full of problems. On a real server you run `sudo ./audit.sh /`.

## What each check looks for

| Check | Finding | Why it matters |
|---|---|---|
| `check_ssh` | `PermitRootLogin yes` (FAIL), `PasswordAuthentication` not `no` (WARN; missing means the default, yes), `X11Forwarding yes` (WARN) | root is the first username every bot tries |
| `check_uid0` | any account other than root with UID 0 (FAIL) | a classic backdoor: a second root under another name |
| `check_empty_passwords` | an empty password field in `/etc/shadow` (FAIL) | login without a password |
| `check_service_shells` | UID 1–999 (system accounts) with a real shell, not `nologin`/`false` (WARN) | service accounts shouldn't be able to log in |
| `check_sudo_nopasswd` | `NOPASSWD` in `/etc/sudoers` or `/etc/sudoers.d/*`, ignoring comments (WARN) | a stolen session becomes root without a password |
| `score` | 100 − 15 per FAIL − 5 per WARN, never below 0 | one number for the report |

Reminder of the file formats: `/etc/passwd` is `name:x:uid:gid:comment:home:shell`, and `/etc/shadow` is `name:hash:...`, where the hash field is empty, `*` or `!` (locked), or a hash like `$6$...`.

@@project server_audit Linux Server Audit
Write the checks in `auditlib.sh` until **▶ tests.sh** prints `All tests passed`, then run **▶ audit.sh**.
@@file auditlib.sh
# auditlib.sh: security checks on a Linux system's configuration.
# Every check takes the ROOT folder as $1 ("/" on a real server) and prints one line per
# finding, "FAIL <check>: <detail>" or "WARN <check>: <detail>", and nothing when all is well.

todo() { echo "TODO: not done yet: $1" >&2; return 1; }

# sshd_config: FAIL if PermitRootLogin is yes; WARN if PasswordAuthentication isn't no
# (missing = "yes (default)"); WARN if X11Forwarding is yes. Ignore commented lines.
check_ssh() {
    todo check_ssh
}

# FAIL "uid0: <name> has UID 0" for every account except root with UID 0.
check_uid0() {
    todo check_uid0
}

# FAIL "empty-password: <name>" for every shadow entry with an empty password field.
check_empty_passwords() {
    todo check_empty_passwords
}

# WARN "service-shell: <name> (<shell>)" for UID 1-999 accounts whose shell doesn't end in nologin or false.
check_service_shells() {
    todo check_service_shells
}

# WARN "sudo-nopasswd: <first word of the line>" for uncommented NOPASSWD lines in
# ROOT/etc/sudoers and every file in ROOT/etc/sudoers.d/ (sudoers first, then the files in name order).
check_sudo_nopasswd() {
    todo check_sudo_nopasswd
}

# Reads findings on stdin and prints 100 - 15 per FAIL - 5 per WARN, minimum 0.
score() {
    todo score
}

# Every check, in order.
run_all() {
    check_ssh "$1"
    check_uid0 "$1"
    check_empty_passwords "$1"
    check_service_shells "$1"
    check_sudo_nopasswd "$1"
}
@@file audit.sh
#!/usr/bin/env bash
# usage: ./audit.sh [ROOT]     (on a real server: sudo ./audit.sh /)
set -uo pipefail
export LC_ALL=C
here="$(cd "$(dirname "$0")" && pwd)"
source "$here/auditlib.sh"

root="${1:-$here/sample_root}"
[[ -d $root/etc ]] || { echo "no etc folder under $root" >&2; exit 2; }

findings=$(run_all "$root")
if [[ -n $findings ]]; then
    echo "$findings"
else
    echo "No findings."
fi
echo
echo "Score: $(score <<< "$findings")/100"
grep -q '^FAIL' <<< "$findings" && exit 1
exit 0
@@file tests.sh
#!/usr/bin/env bash
# Run: bash tests.sh
cd "$(dirname "$0")"
export LC_ALL=C
source ./auditlib.sh
failed=0

assert_eq() {
    if [[ "$1" == "$2" ]]; then
        echo "  ok    $3"
    else
        echo "  FAIL  $3"
        echo "        expected: $(printf '%s' "$2" | tr '\n' '|')"
        echo "        got:      $(printf '%s' "$1" | tr '\n' '|')"
        failed=$(( failed + 1 ))
    fi
}

r=sample_root
assert_eq "$(check_ssh $r 2>&1)" $'FAIL ssh: PermitRootLogin is yes\nWARN ssh: PasswordAuthentication is yes (default)\nWARN ssh: X11Forwarding is yes' "check_ssh"
assert_eq "$(check_uid0 $r 2>&1)" "FAIL uid0: toor has UID 0" "check_uid0"
assert_eq "$(check_empty_passwords $r 2>&1)" $'FAIL empty-password: toor\nFAIL empty-password: deploy' "check_empty_passwords"
assert_eq "$(check_service_shells $r 2>&1)" "WARN service-shell: backup (/bin/sh)" "check_service_shells"
assert_eq "$(check_sudo_nopasswd $r 2>&1)" $'WARN sudo-nopasswd: deploy\nWARN sudo-nopasswd: backup' "check_sudo_nopasswd"
assert_eq "$(printf 'FAIL a\nWARN b\nWARN c\n' | score 2>&1)" "75" "score: 1 FAIL, 2 WARN"
assert_eq "$(printf '' | score 2>&1)" "100" "score: no findings"
assert_eq "$(printf 'FAIL a\n%.0s' 1 2 3 4 5 6 7 | score 2>&1)" "0" "score never goes below 0"

good=$(mktemp -d)
mkdir -p "$good/etc/ssh"
printf 'Port 2222\nPermitRootLogin no\nPasswordAuthentication no\n' > "$good/etc/ssh/sshd_config"
printf 'root:x:0:0:root:/root:/bin/bash\ndaemon:x:1:1::/:/usr/sbin/nologin\nana:x:1000:1000::/home/ana:/bin/bash\n' > "$good/etc/passwd"
printf 'root:$6$salt$hash:19000::::::\ndaemon:*:19000::::::\nana:$6$salt$hash:19000::::::\n' > "$good/etc/shadow"
printf 'root ALL=(ALL:ALL) ALL\n# deploy ALL=(ALL) NOPASSWD: ALL\n' > "$good/etc/sudoers"
assert_eq "$(run_all "$good" 2>&1)" "" "a hardened system has no findings"
rm -rf "$good"

if (( failed > 0 )); then echo "$failed test(s) failed"; exit 1; fi
echo "All tests passed"
@@file sample_root/etc/ssh/sshd_config
# A default-ish config with problems.
Port 22
PermitRootLogin yes
#PasswordAuthentication no
PubkeyAuthentication yes
X11Forwarding yes
@@file sample_root/etc/passwd
root:x:0:0:root:/root:/bin/bash
daemon:x:1:1:daemon:/usr/sbin:/usr/sbin/nologin
www-data:x:33:33:www-data:/var/www:/usr/sbin/nologin
backup:x:34:34:backup:/var/backups:/bin/sh
mysql:x:110:115:MySQL Server:/nonexistent:/bin/false
toor:x:0:0::/root:/bin/bash
ana:x:1000:1000:Ana Silva:/home/ana:/bin/bash
deploy:x:1001:1001::/home/deploy:/bin/bash
@@file sample_root/etc/shadow
root:$6$Vx8f$k3nQ7bS0hash:19800:0:99999:7:::
daemon:*:19800:0:99999:7:::
www-data:*:19800:0:99999:7:::
backup:*:19800:0:99999:7:::
mysql:!:19800::::::
toor::19810:0:99999:7:::
ana:$6$Lp2q$9xVhash:19805:0:99999:7:::
deploy::19820:0:99999:7:::
@@file sample_root/etc/sudoers
Defaults env_reset
root ALL=(ALL:ALL) ALL
%sudo ALL=(ALL:ALL) ALL
# ana ALL=(ALL) NOPASSWD: ALL
deploy ALL=(ALL) NOPASSWD: ALL
@@file sample_root/etc/sudoers.d/backup
backup ALL=(root) NOPASSWD: /usr/bin/rsync
@@text
> For `check_ssh`, first drop comments with `grep -v '^[[:space:]]*#'`, then read a setting with `awk '$1 == "PermitRootLogin" { print $2; exit }'`. `${value:-yes (default)}` fills in the default when it's missing. For the sudoers files, a glob like `"$1"/etc/sudoers.d/*` is already in name order.

@@project server_audit_finished Linux Server Audit: the finished version
The complete project. Try it on a real Linux machine (WSL counts): `sudo ./audit.sh /`. Ideas for more checks: world-writable files (`find "$1" -xdev -type f -perm -o+w`), unexpected SUID programs (`-perm -4000`), the firewall being off, `/etc/shadow` readable by others, password ageing (`PASS_MAX_DAYS` in `/etc/login.defs`). Professional tools like **Lynis** and the **CIS Benchmarks** are the same idea with hundreds of checks.
@@file auditlib.sh
# auditlib.sh: security checks on a Linux system's configuration.
# Every check takes the ROOT folder as $1 ("/" on a real server) and prints one line per
# finding, "FAIL <check>: <detail>" or "WARN <check>: <detail>", and nothing when all is well.

# The value of one sshd_config setting (read from stdin), or nothing.
_sshd_value() {
    awk -v key="$1" 'tolower($1) == tolower(key) { print $2; exit }'
}

# sshd_config: FAIL if PermitRootLogin is yes; WARN if PasswordAuthentication isn't no
# (missing = "yes (default)"); WARN if X11Forwarding is yes. Ignore commented lines.
check_ssh() {
    local cfg="$1/etc/ssh/sshd_config" active value
    if [[ ! -r $cfg ]]; then
        echo "WARN ssh: can't read $cfg"
        return 0
    fi
    active=$(grep -v '^[[:space:]]*#' "$cfg")
    value=$(_sshd_value PermitRootLogin <<< "$active")
    if [[ $value == yes ]]; then echo "FAIL ssh: PermitRootLogin is yes"; fi
    value=$(_sshd_value PasswordAuthentication <<< "$active")
    value=${value:-yes (default)}
    if [[ $value != no ]]; then echo "WARN ssh: PasswordAuthentication is $value"; fi
    value=$(_sshd_value X11Forwarding <<< "$active")
    if [[ $value == yes ]]; then echo "WARN ssh: X11Forwarding is yes"; fi
}

# FAIL "uid0: <name> has UID 0" for every account except root with UID 0.
check_uid0() {
    awk -F: '$3 == 0 && $1 != "root" { print "FAIL uid0: " $1 " has UID 0" }' "$1/etc/passwd"
}

# FAIL "empty-password: <name>" for every shadow entry with an empty password field.
check_empty_passwords() {
    awk -F: '$2 == "" { print "FAIL empty-password: " $1 }' "$1/etc/shadow"
}

# WARN "service-shell: <name> (<shell>)" for UID 1-999 accounts whose shell doesn't end in nologin or false.
check_service_shells() {
    awk -F: '$3 > 0 && $3 < 1000 && $7 !~ /(nologin|false)$/ { print "WARN service-shell: " $1 " (" $7 ")" }' "$1/etc/passwd"
}

# WARN "sudo-nopasswd: <first word of the line>" for uncommented NOPASSWD lines in
# ROOT/etc/sudoers and every file in ROOT/etc/sudoers.d/ (sudoers first, then the files in name order).
check_sudo_nopasswd() {
    local files=() f
    [[ -f $1/etc/sudoers ]] && files+=("$1/etc/sudoers")
    for f in "$1"/etc/sudoers.d/*; do
        [[ -f $f ]] && files+=("$f")
    done
    (( ${#files[@]} )) || return 0
    grep -hv '^[[:space:]]*#' "${files[@]}" | awk '/NOPASSWD/ { print "WARN sudo-nopasswd: " $1 }'
}

# Reads findings on stdin and prints 100 - 15 per FAIL - 5 per WARN, minimum 0.
score() {
    awk '/^FAIL/ { f++ } /^WARN/ { w++ } END { s = 100 - 15 * f - 5 * w; if (s < 0) s = 0; print s }'
}

# Every check, in order.
run_all() {
    check_ssh "$1"
    check_uid0 "$1"
    check_empty_passwords "$1"
    check_service_shells "$1"
    check_sudo_nopasswd "$1"
}
@@file audit.sh
#!/usr/bin/env bash
# usage: ./audit.sh [ROOT]     (on a real server: sudo ./audit.sh /)
set -uo pipefail
export LC_ALL=C
here="$(cd "$(dirname "$0")" && pwd)"
source "$here/auditlib.sh"

root="${1:-$here/sample_root}"
[[ -d $root/etc ]] || { echo "no etc folder under $root" >&2; exit 2; }

findings=$(run_all "$root")
if [[ -n $findings ]]; then
    echo "$findings"
else
    echo "No findings."
fi
echo
echo "Score: $(score <<< "$findings")/100"
grep -q '^FAIL' <<< "$findings" && exit 1
exit 0
@@file tests.sh
#!/usr/bin/env bash
# Run: bash tests.sh
cd "$(dirname "$0")"
export LC_ALL=C
source ./auditlib.sh
failed=0

assert_eq() {
    if [[ "$1" == "$2" ]]; then
        echo "  ok    $3"
    else
        echo "  FAIL  $3"
        echo "        expected: $(printf '%s' "$2" | tr '\n' '|')"
        echo "        got:      $(printf '%s' "$1" | tr '\n' '|')"
        failed=$(( failed + 1 ))
    fi
}

r=sample_root
assert_eq "$(check_ssh $r 2>&1)" $'FAIL ssh: PermitRootLogin is yes\nWARN ssh: PasswordAuthentication is yes (default)\nWARN ssh: X11Forwarding is yes' "check_ssh"
assert_eq "$(check_uid0 $r 2>&1)" "FAIL uid0: toor has UID 0" "check_uid0"
assert_eq "$(check_empty_passwords $r 2>&1)" $'FAIL empty-password: toor\nFAIL empty-password: deploy' "check_empty_passwords"
assert_eq "$(check_service_shells $r 2>&1)" "WARN service-shell: backup (/bin/sh)" "check_service_shells"
assert_eq "$(check_sudo_nopasswd $r 2>&1)" $'WARN sudo-nopasswd: deploy\nWARN sudo-nopasswd: backup' "check_sudo_nopasswd"
assert_eq "$(printf 'FAIL a\nWARN b\nWARN c\n' | score 2>&1)" "75" "score: 1 FAIL, 2 WARN"
assert_eq "$(printf '' | score 2>&1)" "100" "score: no findings"
assert_eq "$(printf 'FAIL a\n%.0s' 1 2 3 4 5 6 7 | score 2>&1)" "0" "score never goes below 0"

good=$(mktemp -d)
mkdir -p "$good/etc/ssh"
printf 'Port 2222\nPermitRootLogin no\nPasswordAuthentication no\n' > "$good/etc/ssh/sshd_config"
printf 'root:x:0:0:root:/root:/bin/bash\ndaemon:x:1:1::/:/usr/sbin/nologin\nana:x:1000:1000::/home/ana:/bin/bash\n' > "$good/etc/passwd"
printf 'root:$6$salt$hash:19000::::::\ndaemon:*:19000::::::\nana:$6$salt$hash:19000::::::\n' > "$good/etc/shadow"
printf 'root ALL=(ALL:ALL) ALL\n# deploy ALL=(ALL) NOPASSWD: ALL\n' > "$good/etc/sudoers"
assert_eq "$(run_all "$good" 2>&1)" "" "a hardened system has no findings"
rm -rf "$good"

if (( failed > 0 )); then echo "$failed test(s) failed"; exit 1; fi
echo "All tests passed"
@@file sample_root/etc/ssh/sshd_config
# A default-ish config with problems.
Port 22
PermitRootLogin yes
#PasswordAuthentication no
PubkeyAuthentication yes
X11Forwarding yes
@@file sample_root/etc/passwd
root:x:0:0:root:/root:/bin/bash
daemon:x:1:1:daemon:/usr/sbin:/usr/sbin/nologin
www-data:x:33:33:www-data:/var/www:/usr/sbin/nologin
backup:x:34:34:backup:/var/backups:/bin/sh
mysql:x:110:115:MySQL Server:/nonexistent:/bin/false
toor:x:0:0::/root:/bin/bash
ana:x:1000:1000:Ana Silva:/home/ana:/bin/bash
deploy:x:1001:1001::/home/deploy:/bin/bash
@@file sample_root/etc/shadow
root:$6$Vx8f$k3nQ7bS0hash:19800:0:99999:7:::
daemon:*:19800:0:99999:7:::
www-data:*:19800:0:99999:7:::
backup:*:19800:0:99999:7:::
mysql:!:19800::::::
toor::19810:0:99999:7:::
ana:$6$Lp2q$9xVhash:19805:0:99999:7:::
deploy::19820:0:99999:7:::
@@file sample_root/etc/sudoers
Defaults env_reset
root ALL=(ALL:ALL) ALL
%sudo ALL=(ALL:ALL) ALL
# ana ALL=(ALL) NOPASSWD: ALL
deploy ALL=(ALL) NOPASSWD: ALL
@@file sample_root/etc/sudoers.d/backup
backup ALL=(root) NOPASSWD: /usr/bin/rsync

@@drill
Which `find` option matches files with the SUID bit set?
= -perm -4000
= -perm /4000
= -perm -u+s
? SUID programs run with their owner's permissions.

@@text
🎉 This is how security auditing starts: turn a hardening checklist into automated checks that anyone can rerun. Mention **CIS Benchmarks** and **Lynis** in interviews, and that you've written checks like these yourself.

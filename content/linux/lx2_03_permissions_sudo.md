@@lesson lx2-perms Permissions in Practice: Octal, umask, Special Bits, ACLs and sudo
@@lang bash
@@text
The security lesson introduced permissions beyond rwx. This one makes them second nature: converting between symbolic and octal, predicting what umask produces, using setgid directories for shared folders, ACLs for exceptions, and writing a safe sudoers rule. These come up constantly in tickets and on the exam.

## Reading `ls -l`

```text
-rwxr-x---  1 ana  devs  4096 Nov  4 09:00 deploy.sh
drwxrws---  2 root devs  4096 Nov  4 09:00 shared
lrwxrwxrwx  1 root root    11 Nov  4 09:00 current -> releases/42
```

First character: `-` file, `d` directory, `l` symlink, `c`/`b` character/block device, `s` socket, `p` pipe. Then three triplets: **user (owner)**, **group**, **others**.

## Octal

r = 4, w = 2, x = 1. Add them per triplet: `rwxr-x---` = 7 5 0 → `chmod 750`.

| Octal | Meaning | Typical use |
|---|---|---|
| 644 | rw-r--r-- | normal files |
| 600 | rw------- | private keys, `.ssh/authorized_keys` |
| 755 | rwxr-xr-x | programs, directories |
| 700 | rwx------ | private directories, `~/.ssh` |
| 640 | rw-r----- | config readable by a service group |
| 775 / 2775 | group-writable (with setgid) | shared project directories |

Symbolic: `chmod u+x,g-w,o= file`, `chmod -R g+rX dir` (capital X: execute only on directories and already-executable files).

**On directories**: `r` lists names, `w` creates/deletes entries (deleting a file needs `w` on the **directory**, not the file), `x` enters/traverses.

## Ownership

`chown ana:devs file`, `chown -R www-data: /var/www/site`, `chgrp devs file`. Only root can give files away.

## umask

The umask removes permissions from the defaults (666 for files, 777 for directories). umask `022` → files 644, directories 755. umask `027` → files 640, directories 750. umask `077` → 600/700. Set in `/etc/login.defs`, `/etc/profile`, shell rc files, or per service (`UMask=` in systemd units).

## Special bits

| Bit | Octal | On a file | On a directory | Shown as |
|---|---|---|---|---|
| **SUID** | 4000 | runs as the file's **owner** (e.g. `/usr/bin/passwd` runs as root) | (ignored) | `s` in user x (`S` if no x) |
| **SGID** | 2000 | runs as the file's group | new files **inherit the directory's group** | `s` in group x |
| **Sticky** | 1000 | (ignored) | only the owner can delete their files (`/tmp`) | `t` in others x |

`chmod 2770 /srv/projects` makes a shared directory where every new file belongs to the project group. `find / -perm -4000 -type f 2>/dev/null` lists SUID programs (audit these: an unexpected SUID binary is a privilege escalation risk).

## Access control lists (ACLs)

For exceptions beyond owner/group/others:

```bash,norun
setfacl -m u:ken:rx /srv/reports        # give ken read+execute
setfacl -m g:auditors:r report.pdf
setfacl -d -m g:devs:rwX /srv/projects  # default ACL: inherited by new files
getfacl /srv/reports
setfacl -x u:ken /srv/reports           # remove an entry
setfacl -b file                         # remove all ACLs
```

A `+` at the end of the permission string (`drwxr-x---+`) means an ACL exists. The **mask** entry limits the maximum permissions for named users and groups.

## File attributes

`chattr +i file` makes it **immutable** (even root can't change or delete it until `chattr -i`); `+a` append-only (logs). `lsattr` shows them. Attackers sometimes use `+i` to protect their own files: check it when "rm: Operation not permitted" appears as root.

## sudo

- `visudo` edits `/etc/sudoers` safely (syntax check); prefer drop-in files in `/etc/sudoers.d/` (`visudo -f /etc/sudoers.d/web`).
- Rule format: `who  where=(as_whom)  commands`.

```text
%wheel        ALL=(ALL:ALL) ALL
%sudo         ALL=(ALL:ALL) ALL
ken           ALL=(root) /usr/bin/systemctl restart nginx, /usr/bin/systemctl status nginx
deploy        ALL=(www-data) NOPASSWD: /usr/local/bin/deploy.sh
```

- **Least privilege**: list exact commands with full paths. Avoid giving editors, shells, `less`, `find`, `tar` or interpreters, which can spawn a root shell. Avoid wildcards in arguments.
- `sudo -l` shows what you may run; logs go to the auth log/journal.
- `su -` switches to root with root's environment (needs root's password); `sudo -i` gives a root shell with your own password (if allowed).

@@quiz
What is `rwxr-x---` in octal?
- 755
* 750
- 740
- 770
? 4+2+1, 4+0+1, 0.

@@quiz
With a umask of 027, what permissions does a new file get?
- 644
* 640
- 750
- 600
? 666 minus 027 → 640 (files don't get x by default).

@@quiz
Every new file in /srv/projects must belong to the `projects` group. What should you set on the directory?
- The sticky bit
* The SGID bit (chmod g+s)
- The SUID bit
- chattr +i
? SGID on a directory makes new files inherit its group.

@@quiz
Root gets "Operation not permitted" when deleting a file. What could cause it?
- The file is 644
* The immutable attribute (chattr +i)
- The sticky bit on the file
- An ACL for another user
? Check with lsattr.

@@quiz
Which sudoers rule follows least privilege for a web admin who only restarts nginx?
- `ken ALL=(ALL) ALL`
- `ken ALL=(root) /usr/bin/vim /etc/nginx/nginx.conf`
* `ken ALL=(root) /usr/bin/systemctl restart nginx`
- `ken ALL=(root) NOPASSWD: ALL`
? Editors can spawn shells; ALL gives everything.

@@flashcards
r / w / x :: 4 / 2 / 1
644 :: rw-r--r--
600 :: rw-------
755 :: rwxr-xr-x
750 :: rwxr-x---
umask 022 :: files 644, dirs 755
umask 027 :: files 640, dirs 750
directory w :: create and delete entries
directory x :: enter/traverse
SUID 4000 :: run as file owner
SGID 2000 on dir :: new files inherit group
sticky 1000 :: only owners delete (/tmp)
capital X :: execute only for dirs/executables
setfacl -m u:ken:rx :: add a named user ACL
setfacl -d :: default (inherited) ACL
+ after permissions :: ACL present
chattr +i :: immutable
visudo :: safe sudoers editing
sudo -l :: list allowed commands

@@drill
What octal value sets the sticky bit together with rwxrwxrwx?
= 1777
? Used on /tmp.

@@drill
Which command shows a file's ACL entries?
= getfacl
? setfacl changes them.

@@exercise Convert permission strings to octal
Write a bash function `to_octal` that converts a 9-character permission string (like `rwxr-x---`, possibly with `s`, `S`, `t` or `T` for special bits) to its 4-digit octal form. Lowercase `s`/`t` mean the special bit **and** x are set; uppercase means only the special bit. Print each input with its octal value.
@@starter
perms='rwxr-xr-x
rw-r-----
rwsr-xr-x
rwxrws---
rwxrwxrwt
rw-r-Sr--
rw-------'

to_octal() {
    echo "0000"
}
while read -r p; do echo "$p $(to_octal "$p")"; done <<< "$perms"
@@expected
rwxr-xr-x 0755
rw-r----- 0640
rwsr-xr-x 4755
rwxrws--- 2770
rwxrwxrwt 1777
rw-r-Sr-- 2644
rw------- 0600
@@requires
to_octal()
${p:
@@hint
Loop over the three triplets with `${p:i*3:3}`. For each, add 4/2/1 for r/w/x; the third character can be `s`/`t` (x + special) or `S`/`T` (special only). The special bit values are 4 (user), 2 (group), 1 (others).
@@solution
perms='rwxr-xr-x
rw-r-----
rwsr-xr-x
rwxrws---
rwxrwxrwt
rw-r-Sr--
rw-------'

to_octal() {
    local p=$1 special=0 digits="" i t v c
    local bits=(4 2 1)
    for i in 0 1 2; do
        t=${p:i*3:3}
        v=0
        [[ ${t:0:1} == r ]] && v=$((v + 4))
        [[ ${t:1:1} == w ]] && v=$((v + 2))
        c=${t:2:1}
        [[ $c == x || $c == s || $c == t ]] && v=$((v + 1))
        [[ $c == s || $c == S || $c == t || $c == T ]] && special=$((special + bits[i]))
        digits+=$v
    done
    echo "$special$digits"
}
while read -r p; do echo "$p $(to_octal "$p")"; done <<< "$perms"

@@exercise Audit SUID files and world-writable paths
The variable `listing` holds `find / -perm /6000 -o -perm -o+w` results in `ls -l` style (`permissions owner group path`). Print every SUID/SGID file **not** in the `known` allow list as `UNEXPECTED SUID/SGID path (owner)`, and every world-writable entry that is not a directory with the sticky bit as `WORLD-WRITABLE path`. Finish with a count of findings.
@@starter
known='/usr/bin/passwd
/usr/bin/sudo
/usr/bin/su
/usr/bin/mount
/usr/bin/chage'
listing='-rwsr-xr-x root root /usr/bin/passwd
-rwsr-xr-x root root /usr/bin/sudo
-rwsr-xr-x root root /usr/bin/mount
-rwsr-xr-x root root /usr/local/bin/backup-helper
-rwxr-sr-x root tty /usr/bin/write
drwxrwxrwt root root /tmp
drwxrwxrwx root root /srv/upload
-rw-rw-rw- www-data www-data /var/www/html/config.php
-rwsr-xr-x root root /home/ana/.cache/x'

echo "$listing"
@@expected
UNEXPECTED SUID/SGID /usr/local/bin/backup-helper (root)
UNEXPECTED SUID/SGID /usr/bin/write (root)
WORLD-WRITABLE /srv/upload
WORLD-WRITABLE /var/www/html/config.php
UNEXPECTED SUID/SGID /home/ana/.cache/x (root)
findings: 5
@@requires
grep -qxF
${perms:
@@hint
Read lines with `while read -r perms owner group path`. SUID is `${perms:3:1}` being `s`/`S`; SGID is `${perms:6:1}`. World-writable is `${perms:8:1} == w`; skip it when `${perms:0:1} == d` and `${perms:9:1}` is `t`.
@@solution
known='/usr/bin/passwd
/usr/bin/sudo
/usr/bin/su
/usr/bin/mount
/usr/bin/chage'
listing='-rwsr-xr-x root root /usr/bin/passwd
-rwsr-xr-x root root /usr/bin/sudo
-rwsr-xr-x root root /usr/bin/mount
-rwsr-xr-x root root /usr/local/bin/backup-helper
-rwxr-sr-x root tty /usr/bin/write
drwxrwxrwt root root /tmp
drwxrwxrwx root root /srv/upload
-rw-rw-rw- www-data www-data /var/www/html/config.php
-rwsr-xr-x root root /home/ana/.cache/x'

findings=0
while read -r perms owner group path; do
    if [[ ${perms:3:1} == [sS] || ${perms:6:1} == [sS] ]]; then
        if ! grep -qxF "$path" <<< "$known"; then
            echo "UNEXPECTED SUID/SGID $path ($owner)"
            findings=$((findings + 1))
        fi
    fi
    if [[ ${perms:8:1} == w ]] && ! [[ ${perms:0:1} == d && ${perms:9:1} == t ]]; then
        echo "WORLD-WRITABLE $path"
        findings=$((findings + 1))
    fi
done <<< "$listing"
echo "findings: $findings"

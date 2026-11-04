@@lesson lx3-bootsvc Troubleshooting Boot, systemd Services, Access and SELinux
@@lang bash
@@text
The other half of Linux+ troubleshooting is "it won't start": the server won't boot, a service fails, a user can't log in or a file can't be read even though permissions look right. This lesson walks through each with the commands and log messages that lead to the cause.

## Boot problems

| Symptom | Likely cause and fix |
|---|---|
| GRUB prompt / "no such device" | GRUB config or disk UUID changed; boot a rescue image, `chroot`, reinstall/regenerate GRUB |
| Kernel panic "unable to mount root fs" | wrong `root=` parameter, missing driver in the initramfs → rebuild with `dracut -f` (RHEL) or `update-initramfs -u` (Debian) |
| **Emergency mode** ("Give root password for maintenance") | usually a bad `/etc/fstab` line or a missing disk: fix fstab (add `nofail`), `systemctl daemon-reload`, reboot |
| Stuck on a service | `systemd-analyze blame`, `systemd-analyze critical-chain` |
| Forgotten root password | at GRUB, edit the entry, add `rd.break` (RHEL) or `init=/bin/bash`, remount `/sysroot` rw, `passwd`, then `touch /.autorelabel` on SELinux systems |

Boot targets: `multi-user.target` (text, servers), `graphical.target`, `rescue.target` (single user, minimal services), `emergency.target` (only root fs read-only). `systemctl get-default`, `systemctl set-default multi-user.target`; add `systemd.unit=rescue.target` at GRUB for one boot.

## Service failures

```bash,norun
systemctl status nginx            # state, recent log lines, main PID, exit code
journalctl -u nginx -b --no-pager # this boot's logs for the unit
journalctl -xe                    # recent with explanations
systemctl list-units --failed
systemctl cat nginx               # the unit file and drop-ins in effect
systemctl edit nginx              # create a drop-in override
systemctl daemon-reload           # after changing unit files
nginx -t; apachectl configtest; sshd -t; named-checkconf   # validate configs before restarting
```

Common causes: **config syntax error**, **port already in use** (`ss -tlpn | grep :80`), **permission denied** on files/sockets, **missing dependency** (database not up), **SELinux denial**, wrong `User=`, **exhausted limits**, and a service in a **restart loop** (`start-limit-hit`: `systemctl reset-failed`).

## Unit file essentials

```ini
[Unit]
Description=Order API
After=network-online.target postgresql.service
Wants=network-online.target
[Service]
User=orders
ExecStart=/opt/orders/bin/api --port 8080
Restart=on-failure
RestartSec=5
[Install]
WantedBy=multi-user.target
```

`enable` creates the symlink for boot; `start` starts now; `enable --now` both; `mask` prevents starting at all.

## Login and access problems

- **Account locked/expired**: `passwd -S user`, `chage -l user`, `faillock --user user` (reset with `--reset`).
- **Shell is nologin** or the home directory is missing/wrong owner.
- **SSH**: `journalctl -u sshd` shows "Permission denied (publickey)" (key or permissions), "Authentication refused: bad ownership or modes", `AllowGroups`/`AllowUsers` restrictions, firewall, `MaxStartups`.
- **sudo**: "user is not in the sudoers file" → group membership (`id user`), log out/in after adding a group.
- **Name resolution**: `getent passwd user` to see if SSSD/LDAP resolves the account.

## File access problems

1. Check permissions on the file **and every parent directory** (`namei -l /srv/app/data/file.db`): each directory needs `x` for the user.
2. Ownership and group membership (`id`).
3. ACLs (`getfacl`) and attributes (`lsattr`: immutable).
4. Mount options (`ro`, `noexec`): `findmnt /srv`.
5. **SELinux / AppArmor**.

## SELinux denials

When permissions are right but access is denied on RHEL-family systems, suspect SELinux:

```bash,norun
getenforce                             # Enforcing / Permissive / Disabled
ausearch -m AVC -ts recent             # denial records
journalctl -t setroubleshoot           # human-readable suggestions (if installed)
ls -Z /var/www/html/index.html         # context, e.g. httpd_sys_content_t
restorecon -Rv /var/www/html           # reset to the default context
semanage fcontext -a -t httpd_sys_content_t "/srv/site(/.*)?" && restorecon -Rv /srv/site
setsebool -P httpd_can_network_connect on   # allow web apps to make outbound connections
semanage port -a -t http_port_t -p tcp 8081 # allow httpd to listen on 8081
```

Classic case: files **moved** (`mv`) from a home directory keep the `user_home_t` context, so nginx/httpd can't read them; `cp` creates new files with the target directory's context. Fix with `restorecon`. Never "fix" it by disabling SELinux: use `setenforce 0` only briefly to confirm the cause.

AppArmor (Ubuntu/SUSE) equivalents: `aa-status`, profiles in `/etc/apparmor.d/`, `aa-complain` to test, denials in the kernel log.

@@quiz
After an fstab edit, the server boots into emergency mode. What's the most likely fix?
* Correct or comment out the bad fstab line (or add nofail), then reboot
- Reinstall the kernel
- Reset the root password
- Disable SELinux
? `mount -a` would have caught it before the reboot.

@@quiz
nginx won't start and `journalctl -u nginx` shows "bind() to 0.0.0.0:80 failed (98: Address already in use)". What next?
- Disable the firewall
* Find what's listening on port 80 with `ss -tlpn` and stop or reconfigure it
- Change SELinux to permissive
- Reinstall nginx
? Another web server or a container probably holds the port.

@@quiz
Files moved into /var/www/html are denied to httpd although permissions are 644. What fixes it on an SELinux system?
- `chmod 777`
* `restorecon -Rv /var/www/html`
- `setenforce 0` permanently
- `chown root:root`
? Moved files keep their old SELinux context.

@@quiz
Which command shows permissions for each directory along a path?
- `ls -l`
- `stat`
* `namei -l /path/to/file`
- `getfacl -R`
? Each parent directory needs x for the user.

@@quiz
After editing a unit file, `systemctl restart app` still uses the old settings. What was missed?
- `systemctl enable app`
* `systemctl daemon-reload`
- `systemctl mask app`
- A reboot is required
? systemd caches unit files until reloaded.

@@flashcards
emergency mode :: often a bad fstab entry
dracut -f / update-initramfs -u :: rebuild initramfs
rd.break :: RHEL root password recovery
systemd-analyze blame :: slow boot units
rescue.target :: single-user maintenance
systemctl status :: state and recent logs
journalctl -u unit -b :: unit logs this boot
systemctl list-units --failed :: failed units
systemctl cat :: effective unit file
systemctl edit :: drop-in override
daemon-reload :: reread unit files
reset-failed :: clear start-limit-hit
mask :: prevent a unit from starting
faillock --reset :: unlock after failed logins
namei -l :: permissions along a path
getenforce :: SELinux mode
ausearch -m AVC :: SELinux denials
restorecon :: reset default SELinux context
setsebool -P :: persistent SELinux boolean
aa-status :: AppArmor status

@@drill
Which command reloads systemd after a unit file change?
= systemctl daemon-reload
? Then restart the service.

@@drill
Which command shows the current SELinux mode?
= getenforce
? sestatus shows more detail.

@@exercise Triage failed services from journal lines
The variable `journal` holds `journalctl` lines for failed units. For each unit, classify the first matching cause from `RULES` (pattern → cause → next step), checking rules in order, and print `unit: cause -> next step`. Units with no match get `unknown -> read journalctl -u unit`.
@@starter
RULES='Address already in use|port conflict|ss -tlpn and free the port
Permission denied|permissions|check namei -l and ownership
avc:  denied|SELinux denial|ausearch -m AVC and restorecon
emerg|config syntax error|run the service config test (nginx -t)
start-limit-hit|restart loop|fix the cause then systemctl reset-failed'
journal='nginx.service: nginx: [emerg] unknown directive "listn" in /etc/nginx/conf.d/site.conf:3
api.service: bind() failed: Address already in use
reports.service: open(/srv/reports/out.csv): Permission denied
httpd.service: avc:  denied  { read } for comm="httpd" name="index.html"
worker.service: Start request repeated too quickly. start-limit-hit
backup.service: Main process exited, code=exited, status=3'

echo "$journal"
@@expected
nginx.service: config syntax error -> run the service config test (nginx -t)
api.service: port conflict -> ss -tlpn and free the port
reports.service: permissions -> check namei -l and ownership
httpd.service: SELinux denial -> ausearch -m AVC and restorecon
worker.service: restart loop -> fix the cause then systemctl reset-failed
backup.service: unknown -> read journalctl -u backup.service
@@requires
IFS='|' read -r pattern cause step
grep -q
@@hint
For each journal line, split off the unit with `${line%%:*}` and the message with `${line#*: }`, then loop over RULES with `while IFS='|' read -r pattern cause step` and test with `grep -qF -- "$pattern" <<< "$msg"`.
@@solution
RULES='Address already in use|port conflict|ss -tlpn and free the port
Permission denied|permissions|check namei -l and ownership
avc:  denied|SELinux denial|ausearch -m AVC and restorecon
emerg|config syntax error|run the service config test (nginx -t)
start-limit-hit|restart loop|fix the cause then systemctl reset-failed'
journal='nginx.service: nginx: [emerg] unknown directive "listn" in /etc/nginx/conf.d/site.conf:3
api.service: bind() failed: Address already in use
reports.service: open(/srv/reports/out.csv): Permission denied
httpd.service: avc:  denied  { read } for comm="httpd" name="index.html"
worker.service: Start request repeated too quickly. start-limit-hit
backup.service: Main process exited, code=exited, status=3'

while IFS= read -r line; do
    unit=${line%%:*}
    msg=${line#*: }
    result="unknown -> read journalctl -u $unit"
    while IFS='|' read -r pattern cause step; do
        if grep -qF -- "$pattern" <<< "$msg"; then
            result="$cause -> $step"
            break
        fi
    done <<< "$RULES"
    echo "$unit: $result"
done <<< "$journal"

@@exercise Find the directory that blocks access
`namei -l` output for a path is in `namei_out` (one line per component: `permissions owner group name`, starting from `/`). The user `www-data` (not the owner of anything, member of group `www-data`) needs to read the final file. For each component, decide which permission class applies (owner, group or other) and check that directories grant `x` and the final file grants `r`. Print the first component that blocks access, or `access OK`.
@@starter
USER_NAME=www-data
USER_GROUPS="www-data"
namei_out='drwxr-xr-x root     root     /
drwxr-xr-x root     root     srv
drwxr-x--- deploy   deploy   app
drwxr-xr-x deploy   www-data static
-rw-r----- deploy   www-data logo.png'

echo "$namei_out"
@@expected
blocked at app (other has ---, needs x)
@@requires
read -r perms owner group name
${perms:
@@hint
Pick the triplet: owner → `${perms:1:3}`, group → `${perms:4:3}`, other → `${perms:7:3}`. Directories need `x` at position 2 of the triplet, the last component needs `r` at position 0.
@@solution
USER_NAME=www-data
USER_GROUPS="www-data"
namei_out='drwxr-xr-x root     root     /
drwxr-xr-x root     root     srv
drwxr-x--- deploy   deploy   app
drwxr-xr-x deploy   www-data static
-rw-r----- deploy   www-data logo.png'

total=$(wc -l <<< "$namei_out")
i=0
blocked=""
while read -r perms owner group name; do
    i=$((i + 1))
    if [[ $owner == "$USER_NAME" ]]; then
        cls=owner; t=${perms:1:3}
    elif [[ " $USER_GROUPS " == *" $group "* ]]; then
        cls=group; t=${perms:4:3}
    else
        cls=other; t=${perms:7:3}
    fi
    if (( i < total )); then
        [[ ${t:2:1} == x ]] || { blocked="$name ($cls has $t, needs x)"; break; }
    else
        [[ ${t:0:1} == r ]] || { blocked="$name ($cls has $t, needs r)"; break; }
    fi
done <<< "$namei_out"
if [[ -n $blocked ]]; then echo "blocked at $blocked"; else echo "access OK"; fi

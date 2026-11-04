@@lesson le4-systemd-network systemd Services, Timers and Networking
@@concepts processes, networking, text-processing
@@lang bash
@@text
Two things go wrong on every Linux desktop sooner or later: a service that won't start, and a network that "doesn't work". This lesson explains systemd units and timers (the modern replacement for many cron jobs), how to read their state, and a layered way to diagnose networking with `ip`, `ss`, `resolvectl` and NetworkManager.

## systemd units

systemd starts and supervises everything after the kernel. Each thing it manages is a **unit**: `.service` (a program), `.timer` (a schedule), `.socket`, `.mount`, `.target` (a group, like `graphical.target`).

| Command | Does |
|---|---|
| `systemctl status sshd` | state, PID, recent log lines |
| `systemctl start/stop/restart X` | now |
| `systemctl enable/disable X` | at boot (`--now` does both) |
| `systemctl --failed` | units that failed |
| `systemctl list-timers` | scheduled timers |
| `systemctl --user ...` | your own user services |
| `journalctl -u X -b` | logs for a unit this boot |
| `systemctl daemon-reload` | after editing unit files |
| `systemctl edit X` | create an override (drop-in) |

Unit files live in `/usr/lib/systemd/system` (packages; don't edit) and `/etc/systemd/system` (yours, takes priority). User units go in `~/.config/systemd/user`.

```ini,norun
# ~/.config/systemd/user/backup.service
[Unit]
Description=Back up Documents

[Service]
Type=oneshot
ExecStart=%h/.local/bin/backup.sh

# ~/.config/systemd/user/backup.timer
[Unit]
Description=Daily backup

[Timer]
OnCalendar=*-*-* 21:00
Persistent=true

[Install]
WantedBy=timers.target
```

Enable with `systemctl --user enable --now backup.timer`. `Persistent=true` runs a missed job after the computer was off. `OnCalendar` examples: `daily`, `Mon..Fri 09:00`, `*-*-01 03:00` (monthly); test with `systemd-analyze calendar "Mon..Fri 09:00"`.

States: **active (running)**, **active (exited)** (oneshot finished), **inactive**, **failed** (see `journalctl`), **activating**. `Restart=on-failure` restarts crashed services.

## Networking, layer by layer

1. **Link**: is the interface up and connected? `ip link`, `nmcli device`, Wi-Fi with `nmcli dev wifi`.
2. **Address**: do you have an IP? `ip -br addr` (`169.254.x.x` means DHCP failed).
3. **Route**: is there a default gateway? `ip route` → `default via 192.168.1.1`. `ping -c3 192.168.1.1`.
4. **Internet**: `ping -c3 1.1.1.1` (by IP, no DNS).
5. **DNS**: `resolvectl query example.com` or `dig example.com`; `resolvectl status` shows servers. If 4 works but names fail, it's DNS.
6. **Service**: `curl -I https://example.com`; is something listening locally? `ss -tlnp` (TCP, listening, numeric, process).

The firewall: `sudo ufw status` (Ubuntu) or `firewall-cmd --list-all` (Fedora). Hostnames: `/etc/hosts`, `hostnamectl`.

@@quiz
Which command shows why a service failed, with its logs from this boot?
- `systemctl enable X`
* `journalctl -u X -b`
- `ip route`
- `ss -tlnp`
? status shows only the last few lines.

@@quiz
What does `systemctl enable --now X` do?
- Starts it only now
- Enables it only at boot
* Enables it at boot and starts it now
- Restarts it
? --now combines enable and start.

@@quiz
Your machine has the address 169.254.12.7. What does that suggest?
- A static IP
* DHCP failed and a link-local address was assigned
- IPv6 is in use
- DNS is broken
? APIPA/link-local range.

@@quiz
`ping 1.1.1.1` works but `ping example.com` fails. Where is the problem?
- The cable
- The default route
* Name resolution (DNS)
- The firewall on example.com
? IP connectivity is fine.

@@quiz
What does `Persistent=true` do in a timer?
* Runs a missed run after the machine was off
- Keeps the service running forever
- Makes the timer survive uninstall
- Restarts on failure
? Like anacron.

@@flashcards
unit :: anything systemd manages
.service / .timer / .target :: program / schedule / group
systemctl status :: state and recent logs
enable --now :: at boot and now
systemctl --failed :: failed units
journalctl -u X -b :: unit logs this boot
/etc/systemd/system :: admin unit files
~/.config/systemd/user :: user units
daemon-reload :: after editing units
systemctl edit :: drop-in override
OnCalendar :: timer schedule
Persistent=true :: catch up missed runs
Restart=on-failure :: restart on crash
ip -br addr :: brief addresses
ip route :: routes and default gateway
169.254.x.x :: DHCP failed
resolvectl query :: test DNS
ss -tlnp :: listening TCP ports with processes
nmcli :: NetworkManager CLI

@@drill
Which command lists TCP sockets that are listening, numerically, with processes?
= ss -tlnp
= ss -ltnp
= ss -tulpn
= ss -tlpn
? netstat is the older tool.

@@drill
Which command reloads systemd after you edit a unit file?
= systemctl daemon-reload
= sudo systemctl daemon-reload
? Then restart the service.

@@exercise Triage systemctl output
`units` holds `systemctl list-units --type=service --all`-style lines (unit, load, active, sub, description). With awk, print the failed units, a count of services per active state, and which `loaded active exited` units are oneshots that finished. Then print the next command to run for each failed unit.
@@starter
units='NetworkManager.service      loaded active   running Network Manager
bluetooth.service           loaded active   running Bluetooth service
cups.service                loaded inactive dead    CUPS Scheduler
docker.service              loaded failed   failed  Docker Application Container Engine
fwupd-refresh.service       loaded active   exited  Refresh fwupd metadata
nfs-client.service          loaded failed   failed  NFS client services
sshd.service                loaded active   running OpenSSH Daemon
systemd-tmpfiles-setup.service loaded active exited Create Volatile Files and Directories'

# Your code here
@@expected
failed:
  docker.service
  nfs-client.service
by state:
  active/exited 2
  active/running 3
  failed/failed 2
  inactive/dead 1
finished oneshots:
  fwupd-refresh.service
  systemd-tmpfiles-setup.service
next steps:
  journalctl -u docker.service -b --no-pager | tail -20
  journalctl -u nfs-client.service -b --no-pager | tail -20
@@requires
awk
journalctl -u
@@hint
Fields: `$1` unit, `$3` active, `$4` sub; the description is the rest (`$5` onwards). Count with `count[$3]++` and print in `END`. Pipe the failed unit names into a `while read -r unit` loop to print `journalctl -u $unit -b --no-pager`.
@@solution
units='NetworkManager.service      loaded active   running Network Manager
bluetooth.service           loaded active   running Bluetooth service
cups.service                loaded inactive dead    CUPS Scheduler
docker.service              loaded failed   failed  Docker Application Container Engine
fwupd-refresh.service       loaded active   exited  Refresh fwupd metadata
nfs-client.service          loaded failed   failed  NFS client services
sshd.service                loaded active   running OpenSSH Daemon
systemd-tmpfiles-setup.service loaded active exited Create Volatile Files and Directories'

echo "failed:"
printf '%s\n' "$units" | awk '$3 == "failed" { print "  " $1 }'
echo "by state:"
printf '%s\n' "$units" | awk '{ count[$3 "/" $4]++ } END { for (s in count) print "  " s, count[s] }' | sort
echo "finished oneshots:"
printf '%s\n' "$units" | awk '$3 == "active" && $4 == "exited" { print "  " $1 }'
echo "next steps:"
printf '%s\n' "$units" | awk '$3 == "failed" { print $1 }' | while read -r unit; do
    echo "  journalctl -u $unit -b --no-pager | tail -20"
done

@@exercise Diagnose the network layer by layer
`check` receives a machine's state as `key=value` lines (link, ip, gateway_ping, internet_ping, dns, https). Walk the layers in order and print the first failing layer with a likely cause and a command to try, or "all layers OK". Detect a link-local `169.254.` address as a DHCP failure even though an IP is present.
@@starter
check() {
    # Your code here
    :
}

check 'link=down
ip=
gateway_ping=fail
internet_ping=fail
dns=fail
https=fail'
check 'link=up
ip=169.254.33.2
gateway_ping=fail
internet_ping=fail
dns=fail
https=fail'
check 'link=up
ip=192.168.1.23
gateway_ping=ok
internet_ping=ok
dns=fail
https=fail'
check 'link=up
ip=192.168.1.23
gateway_ping=ok
internet_ping=ok
dns=ok
https=ok'
@@expected
layer 1 link: down. Check cable or Wi-Fi: nmcli device; nmcli dev wifi
layer 2 address: 169.254.33.2 is link-local, DHCP failed. Check the router, then: nmcli connection up <name>
layer 5 DNS: names fail while IPs work. Inspect: resolvectl status; resolvectl query example.com
all layers OK (ip 192.168.1.23)
@@requires
local -A
case
169.254.
@@hint
Read the lines into an associative array (`local -A s` inside the function) with `while IFS='=' read -r k v`. Then check in order: link down → `nmcli device`; ip empty or `169.254.*` → DHCP (`nmcli connection up ...`); gateway → `ip route`; internet → router/ISP; dns → `resolvectl status`; https → firewall or proxy (`curl -v`). Use `return` after the first failure.
@@solution
check() {
    local -A s
    local k v
    while IFS='=' read -r k v; do
        s[$k]=$v
    done <<< "$1"
    if [[ ${s[link]} != up ]]; then
        echo "layer 1 link: down. Check cable or Wi-Fi: nmcli device; nmcli dev wifi"
        return
    fi
    case ${s[ip]} in
        "") echo "layer 2 address: no IP. Renew DHCP: nmcli connection up <name>"; return ;;
        169.254.*) echo "layer 2 address: ${s[ip]} is link-local, DHCP failed. Check the router, then: nmcli connection up <name>"; return ;;
    esac
    if [[ ${s[gateway_ping]} != ok ]]; then
        echo "layer 3 route: gateway unreachable. Inspect: ip route"
        return
    fi
    if [[ ${s[internet_ping]} != ok ]]; then
        echo "layer 4 internet: router has no uplink. Check the router or ISP"
        return
    fi
    if [[ ${s[dns]} != ok ]]; then
        echo "layer 5 DNS: names fail while IPs work. Inspect: resolvectl status; resolvectl query example.com"
        return
    fi
    if [[ ${s[https]} != ok ]]; then
        echo "layer 6 service: HTTPS fails. Check firewall or proxy: curl -v https://example.com"
        return
    fi
    echo "all layers OK (ip ${s[ip]})"
}

check 'link=down
ip=
gateway_ping=fail
internet_ping=fail
dns=fail
https=fail'
check 'link=up
ip=169.254.33.2
gateway_ping=fail
internet_ping=fail
dns=fail
https=fail'
check 'link=up
ip=192.168.1.23
gateway_ping=ok
internet_ping=ok
dns=fail
https=fail'
check 'link=up
ip=192.168.1.23
gateway_ping=ok
internet_ping=ok
dns=ok
https=ok'

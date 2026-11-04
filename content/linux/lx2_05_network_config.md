@@lesson lx2-netconfig Configuring and Inspecting Linux Networking
@@lang bash
@@text
Linux+ expects you to configure addresses, routes and DNS with the tools each distribution uses (NetworkManager, netplan, systemd-networkd), inspect sockets and traffic, and manage host firewalls. This lesson goes deeper than the networking overview, with the commands and files you'll actually touch.

## The `ip` command (iproute2)

```bash,norun
ip -br addr                         # brief: interface, state, addresses
ip addr add 192.168.10.5/24 dev eth1
ip link set eth1 up
ip route                            # routing table
ip route add 10.50.0.0/16 via 192.168.10.1
ip route get 8.8.8.8                # which route and source address would be used
ip neigh                            # ARP/neighbour cache
ip -s link show eth0                # counters: errors, drops
```

Changes made with `ip` are **not persistent**: they vanish at reboot. Older tools (`ifconfig`, `route`, `arp`, `netstat`) are from **net-tools**, deprecated but still seen.

## Persistent configuration

**NetworkManager** (RHEL, Fedora, desktops, many servers):

```bash,norun
nmcli device status
nmcli connection show
nmcli con mod "System eth0" ipv4.method manual ipv4.addresses 192.168.10.5/24 \
      ipv4.gateway 192.168.10.1 ipv4.dns "192.168.10.53 1.1.1.1"
nmcli con up "System eth0"
nmtui                               # text UI
```

Profiles live in `/etc/NetworkManager/system-connections/`.

**netplan** (Ubuntu): YAML in `/etc/netplan/*.yaml`, rendered to systemd-networkd or NetworkManager.

```yaml
network:
  version: 2
  ethernets:
    eth0:
      addresses: [192.168.10.5/24]
      routes:
        - to: default
          via: 192.168.10.1
      nameservers:
        addresses: [192.168.10.53, 1.1.1.1]
```

`netplan try` applies with automatic rollback if you lose connectivity; `netplan apply` makes it permanent.

**systemd-networkd**: `.network` files in `/etc/systemd/network/`.

## Names and DNS

- `/etc/hostname`, `hostnamectl set-hostname web01`.
- `/etc/hosts`: static entries, checked before DNS (per `/etc/nsswitch.conf`: `hosts: files dns`).
- `/etc/resolv.conf`: resolvers; often managed by **systemd-resolved** (`resolvectl status`, stub at 127.0.0.53) or NetworkManager. Don't hand-edit a managed file.
- Lookups: `dig example.com`, `dig @1.1.1.1 example.com MX +short`, `host`, `nslookup`, `getent hosts web01` (follows nsswitch, unlike dig).

## Sockets and traffic

```bash,norun
ss -tulpn                    # listening TCP/UDP sockets with processes
ss -tan state established    # current TCP connections
ss -s                        # summary
tcpdump -i eth0 -nn port 53  # capture DNS traffic
tcpdump -i any host 10.0.0.5 -w capture.pcap
nc -zv db01 5432             # test a TCP port
curl -v https://example.com  # HTTP(S) details
mtr 8.8.8.8 / traceroute / tracepath
ethtool eth0                 # speed, duplex, link
```

## Bonding, bridging and VLANs

- **Bonding/teaming**: combine NICs (`mode=active-backup` for failover, `802.3ad` for LACP).
- **Bridges** (`br0`) connect VMs or containers to the physical network.
- **VLAN interfaces**: `ip link add link eth0 name eth0.20 type vlan id 20`, or `nmcli con add type vlan ...`.

## Host firewalls

| Tool | Distros | Example |
|---|---|---|
| **firewalld** (`firewall-cmd`) | RHEL family | `firewall-cmd --permanent --add-service=https && firewall-cmd --reload`; zones (`public`, `internal`) |
| **ufw** | Ubuntu | `ufw default deny incoming`, `ufw allow 22/tcp`, `ufw enable`, `ufw status numbered` |
| **nftables** (`nft`) | modern kernels (backend for both) | `nft list ruleset` |
| **iptables** | legacy | `iptables -L -n -v`, chains INPUT/OUTPUT/FORWARD, targets ACCEPT/DROP/REJECT |

Remember: allow SSH **before** enabling a default-deny policy on a remote server.

## Network services you'll configure

`chronyd`/`timedatectl` for NTP, `sshd`, DHCP client (`dhclient`, NetworkManager), a web server (nginx/Apache), and `/etc/services` (port-name mapping).

@@quiz
You added an IP with `ip addr add`, and it disappeared after a reboot. Why?
* `ip` changes are runtime-only; use NetworkManager, netplan or networkd files to persist
- The DHCP server removed it
- The firewall deleted it
- ip addr only works on IPv6
? Persist with nmcli, netplan or .network files.

@@quiz
Which command lists listening TCP and UDP ports with the owning processes?
- `ip route`
* `ss -tulpn`
- `dig -x`
- `nmcli device status`
? -t TCP, -u UDP, -l listening, -p process, -n numeric.

@@quiz
`dig web01` fails but `ping web01` works. What's the likely reason?
- DNS is down for everyone
* web01 is in /etc/hosts; dig queries DNS directly and ignores the hosts file
- ICMP is faster than DNS
- The firewall blocks dig
? `getent hosts web01` follows nsswitch like most programs.

@@quiz
Which netplan command applies a change but rolls back automatically if you don't confirm?
- `netplan apply`
* `netplan try`
- `netplan generate`
- `netplan rollback`
? Safe for remote changes.

@@quiz
On Ubuntu with ufw, what should you do before `ufw enable` on a remote server with default deny incoming?
* `ufw allow 22/tcp` (or OpenSSH) so you don't lock yourself out
- Disable sshd
- Flush the routing table
- Set a static IP
? Enabling cuts off your session otherwise.

@@flashcards
ip -br addr :: brief interface addresses
ip route get :: which route a destination uses
ip neigh :: ARP/neighbour cache
nmcli con mod :: change a NetworkManager profile
nmtui :: NetworkManager text UI
netplan try :: apply with auto-rollback
/etc/netplan/*.yaml :: Ubuntu network config
hostnamectl :: set hostname
/etc/hosts :: static name entries
resolvectl status :: systemd-resolved DNS info
getent hosts :: lookup via nsswitch
ss -tulpn :: listening sockets and processes
tcpdump -nn :: capture without name resolution
nc -zv :: test a TCP port
ethtool :: NIC speed and duplex
firewall-cmd --permanent :: persistent firewalld rule
ufw allow 22/tcp :: allow SSH in ufw
nft list ruleset :: show nftables rules

@@drill
Which file maps hostnames to IPs before DNS is consulted (by default)?
= /etc/hosts
? Order set in /etc/nsswitch.conf.

@@drill
Which command reloads firewalld after permanent changes?
= firewall-cmd --reload
? Without --permanent, rules are runtime only.

@@exercise Find unexpected listening services
`ss -tulpn` output is in `ssout`. Print every listening socket as `proto port process`, and mark it `EXPOSED` if it listens on all interfaces (`0.0.0.0`, `*` or `[::]`) and its port isn't in the allowed list, `local` if it's bound to 127.0.0.1/::1, otherwise `allowed`. Finish with the number of exposed services.
@@starter
allowed="22 80 443"
ssout='Netid State  Recv-Q Send-Q Local Address:Port  Peer Address:Port Process
tcp   LISTEN 0      128    0.0.0.0:22           0.0.0.0:*    users:(("sshd",pid=640,fd=3))
tcp   LISTEN 0      511    0.0.0.0:80           0.0.0.0:*    users:(("nginx",pid=980,fd=6))
tcp   LISTEN 0      511    [::]:443             [::]:*       users:(("nginx",pid=980,fd=7))
tcp   LISTEN 0      80     127.0.0.1:3306       0.0.0.0:*    users:(("mysqld",pid=812,fd=21))
tcp   LISTEN 0      128    0.0.0.0:6379         0.0.0.0:*    users:(("redis-server",pid=1301,fd=6))
udp   UNCONN 0      0      *:161                *:*          users:(("snmpd",pid=700,fd=8))
tcp   LISTEN 0      4096   [::1]:9090           [::]:*       users:(("prometheus",pid=1500,fd=9))'

echo "$ssout"
@@expected
tcp 22 sshd allowed
tcp 80 nginx allowed
tcp 443 nginx allowed
tcp 3306 mysqld local
tcp 6379 redis-server EXPOSED
udp 161 snmpd EXPOSED
tcp 9090 prometheus local
exposed: 2
@@requires
-v allowed="$allowed"
match(
@@hint
The port is after the last `:` of field 5 (`n = split($5, a, ":"); port = a[n]`). Extract the process name with `match($7, /"[^"]+"/)` and `substr`.
@@solution
allowed="22 80 443"
ssout='Netid State  Recv-Q Send-Q Local Address:Port  Peer Address:Port Process
tcp   LISTEN 0      128    0.0.0.0:22           0.0.0.0:*    users:(("sshd",pid=640,fd=3))
tcp   LISTEN 0      511    0.0.0.0:80           0.0.0.0:*    users:(("nginx",pid=980,fd=6))
tcp   LISTEN 0      511    [::]:443             [::]:*       users:(("nginx",pid=980,fd=7))
tcp   LISTEN 0      80     127.0.0.1:3306       0.0.0.0:*    users:(("mysqld",pid=812,fd=21))
tcp   LISTEN 0      128    0.0.0.0:6379         0.0.0.0:*    users:(("redis-server",pid=1301,fd=6))
udp   UNCONN 0      0      *:161                *:*          users:(("snmpd",pid=700,fd=8))
tcp   LISTEN 0      4096   [::1]:9090           [::]:*       users:(("prometheus",pid=1500,fd=9))'

echo "$ssout" | awk -v allowed="$allowed" '
BEGIN { n = split(allowed, list, " "); for (i = 1; i <= n; i++) ok[list[i]] = 1 }
NR > 1 {
    n = split($5, a, ":"); port = a[n]
    addr = substr($5, 1, length($5) - length(port) - 1)
    match($7, /"[^"]+"/); proc = substr($7, RSTART + 1, RLENGTH - 2)
    if (addr == "127.0.0.1" || addr == "[::1]") status = "local"
    else if (port in ok) status = "allowed"
    else { status = "EXPOSED"; exposed++ }
    print $1, port, proc, status
}
END { print "exposed:", exposed + 0 }'

@@exercise Generate a netplan file
Write a bash function `netplan_for` that takes an interface, address/prefix, gateway and a comma-separated DNS list, and prints a netplan YAML document like the one in the lesson (two-space indentation, DNS addresses as a YAML list `[a, b]`). Validate the address with a regex first: on failure print `invalid address: ...` and return 1. Generate configs for the two servers below.
@@starter
netplan_for() {
    echo "TODO"
}

netplan_for eth0 192.168.10.5/24 192.168.10.1 "192.168.10.53,1.1.1.1"
netplan_for ens18 10.0.0.300/24 10.0.0.1 "10.0.0.53" || echo "skipped ens18"
@@expected
network:
  version: 2
  ethernets:
    eth0:
      addresses: [192.168.10.5/24]
      routes:
        - to: default
          via: 192.168.10.1
      nameservers:
        addresses: [192.168.10.53, 1.1.1.1]
invalid address: 10.0.0.300/24
skipped ens18
@@requires
=~
${dns//,/, }
@@hint
Validate with `[[ $addr =~ ^([0-9]{1,3}\.){3}[0-9]{1,3}/[0-9]{1,2}$ ]]` and also check each octet is at most 255. Use a here-document (`cat <<EOF`) for the YAML.
@@solution
netplan_for() {
    local iface=$1 addr=$2 gw=$3 dns=$4 ip octet
    if ! [[ $addr =~ ^([0-9]{1,3}\.){3}[0-9]{1,3}/[0-9]{1,2}$ ]]; then
        echo "invalid address: $addr"
        return 1
    fi
    ip=${addr%/*}
    IFS=. read -r -a octets <<< "$ip"
    for octet in "${octets[@]}"; do
        if (( octet > 255 )); then
            echo "invalid address: $addr"
            return 1
        fi
    done
    cat <<EOF
network:
  version: 2
  ethernets:
    $iface:
      addresses: [$addr]
      routes:
        - to: default
          via: $gw
      nameservers:
        addresses: [${dns//,/, }]
EOF
}

netplan_for eth0 192.168.10.5/24 192.168.10.1 "192.168.10.53,1.1.1.1"
netplan_for ens18 10.0.0.300/24 10.0.0.1 "10.0.0.53" || echo "skipped ens18"

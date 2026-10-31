@@lesson l3-networking Linux Networking, SSH and Firewalls
@@concepts networking
@@text
## Who am I on the network?

```bash,norun
ip a                       # addresses (ip addr); the old command was ifconfig
ip r                       # routes: "default via 192.168.1.1" is the gateway
cat /etc/resolv.conf       # DNS servers
hostname -I
```

## Is it reachable? The troubleshooting ladder

Work up the layers:

```bash,norun
ping -c 3 192.168.1.1          # 1. can I reach the gateway? (ICMP)
ping -c 3 8.8.8.8              # 2. the internet by IP?
dig example.com +short         # 3. does DNS work? (or: nslookup example.com)
traceroute example.com         # 4. where along the path does it stop? (tracepath, mtr)
curl -I https://example.com    # 5. does the web server answer? (headers only)
nc -zv db01 5432               # 6. is a specific TCP port open?
```

If (2) works but (3) fails, it's DNS. It's always DNS. (Not always, but check it early.)

## What is listening here?

```bash,norun
ss -tlnp        # TCP, listening, numeric, with process (old: netstat -tlnp)
ss -tn          # current TCP connections
sudo lsof -i :80
```

## SSH: how you'll reach every server

```bash,norun
ssh ana@203.0.113.10
ssh -p 2222 ana@server              # a non-default port
ssh-keygen -t ed25519 -C "ana@laptop"   # make a key pair (once)
ssh-copy-id ana@server                  # install your public key on the server
scp backup.tar.gz ana@server:/tmp/      # copy a file
rsync -avz ./site/ ana@server:/var/www/site/   # sync a folder, only changes
ssh -L 8080:localhost:80 ana@server     # tunnel: your localhost:8080 -> server's port 80
```

`~/.ssh/config` saves typing:

```text
Host web
    HostName 203.0.113.10
    User ana
    Port 2222
    IdentityFile ~/.ssh/id_ed25519
```

Then just `ssh web`. Keys beat passwords: they can't be guessed. After keys work, set `PasswordAuthentication no` in `/etc/ssh/sshd_config`.

## Firewalls

```bash,norun
# Ubuntu: ufw
sudo ufw allow OpenSSH
sudo ufw allow 443/tcp
sudo ufw enable
sudo ufw status numbered

# Red Hat / Rocky / Fedora: firewalld
sudo firewall-cmd --add-service=https --permanent
sudo firewall-cmd --reload
sudo firewall-cmd --list-all
```

Both are front-ends to the kernel's **netfilter** (`nftables`, formerly `iptables`).

> ⚠ When configuring a firewall over SSH, allow SSH **first**, or you lock yourself out.

## curl for APIs

```bash,norun
curl -s https://api.github.com/repos/torvalds/linux | grep stargazers_count
curl -s -X POST -H "Content-Type: application/json" -d '{"name":"web01"}' https://api.example.com/servers
curl -o file.iso -L https://example.com/download   # follow redirects, save to a file
```

@@quiz
`ping 8.8.8.8` works but `ping google.com` says "unknown host". What's the likely problem?
* DNS
- The network cable
- The firewall blocks ICMP
- The default gateway
? Reaching an IP but not a name points at name resolution. Check `/etc/resolv.conf` and `dig`.

@@quiz
Which command shows which programs are **listening** on TCP ports?
- `ip a`
* `ss -tlnp`
- `ping -l`
- `curl localhost`
? t = TCP, l = listening, n = numeric, p = process.

@@quiz
You're enabling `ufw` on a remote server over SSH. What must you do first?
* `ufw allow OpenSSH` (or your SSH port)
- reboot
- disable SSH
- nothing, ufw allows SSH by default
? Enabling a default-deny firewall without allowing SSH drops your own connection.

@@flashcards
ip a / ip r :: Addresses / routing table (gateway)
dig / nslookup :: DNS lookups
ss -tlnp :: Listening TCP ports with their processes
curl -I :: Fetch only the HTTP headers
nc -zv host port :: Test whether a TCP port is open
ssh-keygen -t ed25519 :: Create an SSH key pair
ssh-copy-id :: Install your public key on a server
scp / rsync :: Copy files / sync folders over SSH
ufw / firewalld :: Firewall front-ends on Ubuntu / Red Hat

@@exercise Parse ss output
`ss_output` imitates `ss -tlnp`. Print the listening **ports** (the number after the last `:` in the Local Address column), unique and numerically sorted.
@@starter
ss_output='State  Recv-Q Send-Q Local-Address:Port Peer-Address:Port Process
LISTEN 0      128    0.0.0.0:22         0.0.0.0:*         users:(("sshd",pid=812))
LISTEN 0      511    0.0.0.0:80         0.0.0.0:*         users:(("nginx",pid=1430))
LISTEN 0      80     127.0.0.1:3306     0.0.0.0:*         users:(("mysqld",pid=900))
LISTEN 0      128    [::]:22            [::]:*            users:(("sshd",pid=812))
LISTEN 0      511    [::]:443           [::]:*            users:(("nginx",pid=1430))'

@@expected
22
80
443
3306
@@hint
`awk 'NR > 1 {print $4}'` gives the local address; `sed 's/.*://'` removes everything up to the last colon; then `sort -nu`.
@@solution
ss_output='State  Recv-Q Send-Q Local-Address:Port Peer-Address:Port Process
LISTEN 0      128    0.0.0.0:22         0.0.0.0:*         users:(("sshd",pid=812))
LISTEN 0      511    0.0.0.0:80         0.0.0.0:*         users:(("nginx",pid=1430))
LISTEN 0      80     127.0.0.1:3306     0.0.0.0:*         users:(("mysqld",pid=900))
LISTEN 0      128    [::]:22            [::]:*            users:(("sshd",pid=812))
LISTEN 0      511    [::]:443           [::]:*            users:(("nginx",pid=1430))'
echo "$ss_output" | awk 'NR > 1 {print $4}' | sed 's/.*://' | sort -nu

@@exercise Exposed services
Using the same data, print the **process names** that listen on **all** addresses (`0.0.0.0` or `[::]`), unique and sorted. Services bound to `127.0.0.1` are only reachable locally, which is what you want for a database.
@@starter
ss_output='State  Recv-Q Send-Q Local-Address:Port Peer-Address:Port Process
LISTEN 0      128    0.0.0.0:22         0.0.0.0:*         users:(("sshd",pid=812))
LISTEN 0      511    0.0.0.0:80         0.0.0.0:*         users:(("nginx",pid=1430))
LISTEN 0      80     127.0.0.1:3306     0.0.0.0:*         users:(("mysqld",pid=900))
LISTEN 0      128    [::]:22            [::]:*            users:(("sshd",pid=812))
LISTEN 0      511    [::]:443           [::]:*            users:(("nginx",pid=1430))'

@@expected
nginx
sshd
@@hint
`awk '$4 ~ /^(0\.0\.0\.0|\[::\]):/ {print $6}'`, then pull the name out of `users:(("sshd",pid=812))` with `cut -d'"' -f2`, then `sort -u`.
@@solution
ss_output='State  Recv-Q Send-Q Local-Address:Port Peer-Address:Port Process
LISTEN 0      128    0.0.0.0:22         0.0.0.0:*         users:(("sshd",pid=812))
LISTEN 0      511    0.0.0.0:80         0.0.0.0:*         users:(("nginx",pid=1430))
LISTEN 0      80     127.0.0.1:3306     0.0.0.0:*         users:(("mysqld",pid=900))
LISTEN 0      128    [::]:22            [::]:*            users:(("sshd",pid=812))
LISTEN 0      511    [::]:443           [::]:*            users:(("nginx",pid=1430))'
echo "$ss_output" | awk '$4 ~ /^(0\.0\.0\.0|\[::\]):/ {print $6}' | cut -d'"' -f2 | sort -u

@@lesson lx2-net-auto Networking, Containers and Automation on Linux
@@lang bash
@@text
## Networking

```bash,norun
ip a                                  # addresses (old: ifconfig)
ip route ; ip route add 10.9.0.0/16 via 10.0.0.1
ip link set eth0 up
ss -tulpn                             # listening ports with processes (old: netstat -tulpn)
nmcli con show ; nmcli con mod "Wired 1" ipv4.addresses 10.0.0.20/24 ipv4.method manual ; nmcli con up "Wired 1"
hostnamectl set-hostname web01
resolvectl status                     # DNS with systemd-resolved; also /etc/resolv.conf
dig example.com ; host example.com
cat /etc/hosts                        # local name overrides
```

Configuration tools differ: **NetworkManager** (`nmcli`, Red Hat and desktops), **netplan** (Ubuntu servers: YAML in `/etc/netplan/`, `netplan apply`), **systemd-networkd**. **Bonding/teaming** combines NICs; **bridges** connect VMs to the network.

## Containers

```bash,norun
podman run -d --name web -p 8080:80 docker.io/library/nginx   # podman: daemonless, rootless; same commands as docker
docker ps ; docker logs web ; docker exec -it web sh
docker build -t myapp:1.0 .            # from a Dockerfile
docker volume create data ; docker run -v data:/var/lib/data ...
docker compose up -d                   # several containers from compose.yaml
```

Images come from **registries** (Docker Hub, Quay, a company registry). Containers are ephemeral: data that must survive goes in **volumes** or bind mounts. **Kubernetes** orchestrates containers across many servers (pods, deployments, services); `kubectl get pods`.

## Version control and automation

- **Git**: `clone`, `add`, `commit`, `push`, `pull`, `branch`, `merge`; `.gitignore`.
- **Ansible**: agentless configuration management over SSH, with YAML **playbooks**. **Idempotent**: running it twice changes nothing the second time.

```yaml
- name: Web servers
  hosts: web
  become: true
  tasks:
    - name: nginx installed
      ansible.builtin.package:
        name: nginx
        state: present
    - name: nginx running
      ansible.builtin.service:
        name: nginx
        state: started
        enabled: true
```

- Others: **Puppet**, **Chef**, **SaltStack** (agent-based), **Terraform** (infrastructure as code), **cloud-init** (configures new cloud VMs on first boot), CI/CD pipelines.
- Shell scripting (the Bash track): variables, tests, loops, functions, exit codes, `set -euo pipefail`. **Python** is common for bigger automation.
## Troubleshooting network problems in order

When a Linux server "has no network", work from the bottom of the stack up, so you don't chase DNS when the cable is unplugged:

1. **Link:** `ip link` shows whether the interface is `UP` and has a carrier. `ethtool eth0` shows speed and duplex.
2. **Address:** `ip a` should show the expected IP. No address usually means a DHCP or configuration problem.
3. **Gateway:** `ip route` should show a default route. Test it with `ping -c 3 <gateway>`.
4. **Beyond the gateway:** `ping -c 3 1.1.1.1` tests routing without DNS. `tracepath` or `traceroute` shows where packets stop.
5. **Name resolution:** `dig example.com` or `resolvectl query example.com`. If an IP works and a name doesn't, the problem is DNS.
6. **The service itself:** `ss -tulpn` shows whether something is listening on the port. Check the firewall with `firewall-cmd --list-all` or `nft list ruleset`.

@@quiz
Which command shows which processes are listening on which TCP and UDP ports?
* ss -tulpn
- ip route
- dig
- nmcli con show
? -t TCP, -u UDP, -l listening, -p process, -n numeric.

@@quiz
On an Ubuntu server, where is the network configuration usually kept?
* YAML files in /etc/netplan/, applied with netplan apply
- /etc/sysconfig/network-scripts only
- /etc/fstab
- /etc/hosts
? Red Hat uses NetworkManager (nmcli, keyfiles).

@@quiz
A container's database data disappears every time the container is recreated. What's the fix?
* Store the data in a volume (or bind mount)
- Use a bigger image
- Run the container as root
- Use docker logs
? The container's own writable layer is thrown away with the container.

@@quiz
What does it mean that an Ansible playbook is idempotent?
* Running it again leaves an already-correct system unchanged
- It runs only once
- It needs an agent on every server
- It runs faster the second time
? Tasks describe the desired state, not the steps.

@@quiz
Which tool configures a brand new cloud VM (users, packages, files) on its first boot?
* cloud-init
- logrotate
- GRUB
- fail2ban
? Cloud providers pass "user data" to cloud-init.

@@quiz
A server can ping 1.1.1.1 but `curl https://example.com` fails with "could not resolve host". What is the most likely problem?
* DNS resolution
- The network cable
- The default gateway
- The web server's TLS certificate
? Reaching an IP proves link, address and routing work; failing names points to DNS.

@@drill
Which command shows listening ports with their processes?
= ss -tulpn
= ss -tlnp
= ss
? The modern replacement for netstat.

@@flashcards
ip a / ip route :: Addresses / routes
ss -tulpn :: Listening ports and processes
nmcli :: NetworkManager command line
netplan :: Ubuntu's YAML network configuration
hostnamectl :: Set the hostname
/etc/hosts :: Local name resolution overrides
podman :: Daemonless, rootless docker-compatible container engine
container volume :: Persistent data outside the container
docker compose :: Multi-container apps from one file
Kubernetes :: Container orchestration across servers
Ansible :: Agentless, SSH, YAML playbooks, idempotent
cloud-init :: First-boot configuration of cloud VMs

@@exercise Find the first failing network layer
A helpdesk script collected check results from four servers, as `host check=result ...` lines. Walk the lesson's order (`link`, `addr`, `gw`, `internet`, `dns`, `listen`) and report the **first** check that failed for each host with the command to run next. If everything passed, print `all checks passed: look at the firewall`.
@@starter
results='web01 link=up addr=ok gw=ok internet=ok dns=fail listen=ok
web02 link=down addr=none gw=fail internet=fail dns=fail listen=ok
db01 link=up addr=ok gw=ok internet=ok dns=ok listen=fail
app01 link=up addr=ok gw=ok internet=ok dns=ok listen=ok
app02 link=up addr=none gw=fail internet=fail dns=fail listen=fail'

declare -A next_cmd=(
    [link]="ip link; ethtool eth0"
    [addr]="ip a; nmcli con show"
    [gw]="ip route; ping -c 3 <gateway>"
    [internet]="tracepath 1.1.1.1"
    [dns]="resolvectl status; dig example.com"
    [listen]="ss -tulpn"
)

# for each host, find the first failing check in order
@@expected
web01: dns failed (fail): resolvectl status; dig example.com
web02: link failed (down): ip link; ethtool eth0
db01: listen failed (fail): ss -tulpn
app01: all checks passed: look at the firewall
app02: addr failed (none): ip a; nmcli con show
@@requires
for check in
@@hint
Turn each line's `key=value` words into an associative array, then loop `for check in link addr gw internet dns listen` and stop at the first value that isn't `up` or `ok`.
@@solution
results='web01 link=up addr=ok gw=ok internet=ok dns=fail listen=ok
web02 link=down addr=none gw=fail internet=fail dns=fail listen=ok
db01 link=up addr=ok gw=ok internet=ok dns=ok listen=fail
app01 link=up addr=ok gw=ok internet=ok dns=ok listen=ok
app02 link=up addr=none gw=fail internet=fail dns=fail listen=fail'

declare -A next_cmd=(
    [link]="ip link; ethtool eth0"
    [addr]="ip a; nmcli con show"
    [gw]="ip route; ping -c 3 <gateway>"
    [internet]="tracepath 1.1.1.1"
    [dns]="resolvectl status; dig example.com"
    [listen]="ss -tulpn"
)

while read -r host rest; do
    declare -A r=()
    for kv in $rest; do
        r[${kv%%=*}]=${kv#*=}
    done
    verdict="all checks passed: look at the firewall"
    for check in link addr gw internet dns listen; do
        if [ "${r[$check]}" != up ] && [ "${r[$check]}" != ok ]; then
            verdict="$check failed (${r[$check]}): ${next_cmd[$check]}"
            break
        fi
    done
    echo "$host: $verdict"
    unset r
done <<< "$results"

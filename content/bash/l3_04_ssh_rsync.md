@@lesson l3-ssh-rsync SSH and rsync in Depth: Keys, Config, Agents, Tunnels and Safe Syncs
@@concepts networking, automation, files, loops, conditions, text-processing, maps
@@text
Most Linux administration happens over SSH, and most file moving between machines happens with `rsync` (which usually runs over SSH). The basics take five minutes to learn, but the details decide whether your setup is secure, whether your scripts hang at 3 a.m. waiting for a password prompt, and whether a sync quietly deletes the wrong folder. This lesson goes through those details.

## Keys, properly

```bash,norun
ssh-keygen -t ed25519 -C "ana@laptop-2026" -f ~/.ssh/id_ed25519
ssh-copy-id -i ~/.ssh/id_ed25519.pub ana@server
```

- **ed25519** keys are short, fast and secure; use them unless an old system only supports RSA (then use `-t rsa -b 4096`).
- The **private key** (`id_ed25519`) never leaves your machine. Protect it with a **passphrase**, and keep its permissions at `600` (`~/.ssh` itself at `700`). SSH refuses keys that other users can read.
- The **public key** (`.pub`) goes into `~/.ssh/authorized_keys` on the server, one key per line. `ssh-copy-id` does that and sets the permissions.
- Lines in `authorized_keys` can carry **options** that limit what a key may do. A backup key that may only run one command from one address:

```text
from="10.0.5.20",command="/usr/local/bin/backup-receive",no-port-forwarding,no-pty ssh-ed25519 AAAA... backup@nas
```

- Use **separate keys** for separate purposes (your laptop, the CI system, the backup job), so you can revoke one without touching the others.
- Hardware keys (FIDO2, `ssh-keygen -t ed25519-sk`) keep the private key on a token you have to touch.

## The agent

Typing a passphrase for every connection gets old. **ssh-agent** holds decrypted keys in memory for the session:

```bash,norun
eval "$(ssh-agent -s)"     # start an agent (desktops usually start one for you)
ssh-add ~/.ssh/id_ed25519  # unlock once
ssh-add -l                 # list loaded keys
ssh-add -t 4h ~/.ssh/id_ed25519   # forget it after four hours
```

**Agent forwarding** (`ssh -A`) lets you use your local keys from a remote server, for example to `git pull` there. It's convenient but risky: anyone with root on that server can use your agent while you're connected. Prefer **ProxyJump** (below), which never exposes the agent to the intermediate host.

## ~/.ssh/config

The client config turns long commands into short names and keeps your options consistent:

```text
Host bastion
    HostName 203.0.113.10
    User ana
    IdentityFile ~/.ssh/id_ed25519

Host web-*
    User deploy
    ProxyJump bastion

Host web-prod
    HostName 10.0.1.20

Host *
    ServerAliveInterval 30
    IdentitiesOnly yes
    AddKeysToAgent yes
```

Rules that matter:

- `ssh web-prod` reads the file **top to bottom** and, for each option, the **first value found wins**. So put **specific hosts first** and general defaults (`Host *`) **last**. Here `web-prod` gets `User deploy` and `ProxyJump bastion` from `web-*`, `HostName` from its own block and the keep-alive from `*`.
- Patterns use `*` and `?`, several patterns can share a line, and `!pattern` excludes.
- `ssh -G web-prod` prints the **fully resolved** configuration for a host: the quickest way to debug "why is it using that user?"
- `IdentitiesOnly yes` stops the client offering every key in the agent, which otherwise can hit the server's `MaxAuthTries` limit and fail with "Too many authentication failures".
- `ServerAliveInterval 30` sends a keep-alive every 30 seconds, so NAT routers and firewalls don't silently drop idle sessions.
- `Include ~/.ssh/config.d/*` lets you split the file per project.

## Jump hosts

Private servers often sit behind a **bastion** (jump host). `ProxyJump` (or `ssh -J bastion web-prod`) makes the client connect to the bastion, open a forwarded connection from there to the target, and run the real SSH session **end to end** through it. The bastion never sees your private key or your session contents. Chains work too: `-J bastion1,bastion2`.

## Tunnels

SSH can forward TCP ports through the encrypted connection:

| Type | Command | Meaning |
|---|---|---|
| **Local** | `ssh -L 5432:db.internal:5432 bastion` | your `localhost:5432` reaches `db.internal:5432` as seen from the bastion |
| **Remote** | `ssh -R 8080:localhost:3000 server` | the server's `localhost:8080` reaches your local port 3000 |
| **Dynamic** | `ssh -D 1080 bastion` | a SOCKS proxy on your port 1080; traffic exits from the bastion |

Add `-N` (no remote command) and `-f` (go to the background) for tunnel-only sessions. Local forwards are the everyday tool: reach a database or an admin UI that only listens on a private network, without opening a firewall port. Remote forwards are how you briefly show a local dev server to someone, but they're also a way to punch through firewalls, which is why many organisations disable them on servers (`AllowTcpForwarding no`).

## Running commands remotely

```bash,norun
ssh web-prod 'df -h / && uptime'          # quote so the remote shell expands, not yours
ssh web-prod "echo $HOSTNAME"              # $HOSTNAME expands LOCALLY (double quotes)
ssh web-prod 'bash -s' < local-script.sh   # run a local script remotely
ssh -t web-prod sudo systemctl restart nginx   # -t allocates a terminal for sudo's prompt
```

For scripts and cron jobs:

- Use **`-o BatchMode=yes`** so SSH fails immediately instead of waiting forever for a password or a host-key question.
- Use **`-o ConnectTimeout=5`** so an unreachable host fails fast.
- Loop over hosts with care: `ssh` reads standard input, so inside `while read host; do ssh "$host" uptime; done < hosts.txt` the first `ssh` swallows the rest of the file. Use `ssh -n` (or `< /dev/null`) inside loops.
- Check exit codes: `ssh` returns the remote command's exit status, or 255 for SSH's own errors.

## Host keys

The first time you connect, SSH shows the server's **host key fingerprint** and asks you to confirm it; afterwards it's stored in `~/.ssh/known_hosts`. If the key later changes, you get a loud warning. That's either a reinstalled server or someone intercepting the connection, so check before you delete the old entry with `ssh-keygen -R host`. For automation, distribute known host keys (or use SSH certificates) instead of setting `StrictHostKeyChecking no`, which turns the protection off.

## Hardening the server

In `/etc/ssh/sshd_config` (or a file in `/etc/ssh/sshd_config.d/`):

```text
PasswordAuthentication no
KbdInteractiveAuthentication no
PermitRootLogin no
AllowGroups ssh-users
MaxAuthTries 3
```

Run `sudo sshd -t` to check the syntax **before** reloading, and keep an existing session open while you test a new login in a second terminal. Locking yourself out of a remote server is a rite of passage you can skip. Tools like **fail2ban** block addresses with repeated failures; moving SSH to another port reduces log noise but isn't real security.

## Troubleshooting a connection

When `ssh` fails, add `-v` (or `-vvv` for more detail). The output shows which config blocks applied, which keys were offered and why the server rejected them. Common findings: the key file has the wrong permissions, the server expects a different user, `IdentitiesOnly` is missing and too many keys were offered, or the host key changed. On the server side, `journalctl -u ssh` (or `-u sshd`, depending on the distribution) shows the matching log lines, including "Authentication refused: bad ownership or modes" when the home directory or `~/.ssh` is writable by others. `ssh -o PreferredAuthentications=password` checks whether password login still works before you disable it for good.

## rsync: copy only what changed

```bash,norun
rsync -av --dry-run ./site/ deploy@web-prod:/var/www/site/
rsync -av --delete --exclude '.git/' ./site/ deploy@web-prod:/var/www/site/
```

- **`-a`** (archive) means recursive and preserve permissions, times, symlinks, owner and group (owner and group only as root). **`-v`** lists files; **`-z`** compresses over slow links (pointless for already-compressed media).
- rsync decides what to send by comparing **size and modification time** by default (`--checksum` compares content, which is slower but catches same-size edits with reset times). For changed files it sends only the **differing blocks**.
- **The trailing slash matters.** `rsync -a site/ dest/` copies the *contents* of `site` into `dest`; `rsync -a site dest/` creates `dest/site/`. Get this wrong with `--delete` and you can wipe a directory, so:
- **Always run with `--dry-run` (`-n`) first** when using `--delete`. It prints exactly what would be copied and deleted.
- **`--delete`** removes files at the destination that no longer exist at the source, making it a mirror. `--delete-after` deletes once the transfer has finished, and `--backup --backup-dir=../deleted-$(date +%F)` keeps what would have been deleted.
- **`--exclude`** and **`--include`** filter paths (`--exclude-from=file` for long lists). Patterns are matched relative to the source root.
- **`--partial --progress`** (or `-P`) keeps partly transferred files so a big copy can resume.
- **`--link-dest=../previous`** makes **snapshot backups**: unchanged files become hard links to the previous snapshot, so every daily folder looks complete but only changed files use space.
- **`-e 'ssh -p 2222'`** passes SSH options; with `~/.ssh/config` you rarely need it.

`scp` is fine for a single file, and modern `scp` actually uses the SFTP protocol underneath. For directories, repeated syncs or anything large, use rsync.

## A safe deploy pattern

1. Sync into a **new release folder** (`/var/www/releases/2026-11-06_0915/`), using `--link-dest` against the current release to save space and time.
2. Run checks on the new folder.
3. Switch a **symlink** (`ln -sfn releases/2026-11-06_0915 /var/www/current`) atomically.
4. Keep the last few releases so rollback is just pointing the symlink back.

The web server never sees a half-copied site, and a rollback takes a second.

@@quiz
Two blocks in ~/.ssh/config set `User`: `Host web-*` (User deploy) comes first, then `Host *` (User ana). Which user does `ssh web-prod` use?
* deploy, because the first value found for each option wins
- ana, because the last value wins
- Both, tried in order
- Neither; it uses your local user name
? Put specific hosts first and Host * last.

@@quiz
Why is ProxyJump safer than agent forwarding for reaching servers behind a bastion?
* The bastion only relays an end-to-end encrypted connection and never gets access to your agent
- ProxyJump uses a stronger cipher
- Agent forwarding sends your private key file to the server
- ProxyJump disables host key checking
? With -A, root on the bastion can use your agent while you're connected.

@@quiz
In `while read host; do ssh "$host" uptime; done < hosts.txt`, only the first host is contacted. Why?
* ssh reads standard input and swallows the rest of hosts.txt; use ssh -n
- read stops after one line by default
- ssh exits the loop on success
- uptime closes the file
? -n redirects ssh's stdin from /dev/null.

@@quiz
What does `rsync -a site dest/` (no trailing slash on site) do?
* Creates dest/site/ with the contents inside it
- Copies the contents of site directly into dest/
- Deletes dest first
- Fails without a trailing slash
? A trailing slash on the source means "the contents of".

@@quiz
Which option should you always try first before running rsync with --delete?
* --dry-run (-n)
- --checksum
- --compress
- --partial
? It shows exactly what would be copied and deleted.

@@quiz
What does `--link-dest` give you in backups?
* Snapshot folders where unchanged files are hard links to the previous snapshot, so each looks complete but uses little space
- Symlinks to the source files
- Compressed archives
- Encrypted copies
? Only changed files take new space.

@@flashcards
ssh-keygen -t ed25519 :: create a modern key pair
~/.ssh permissions :: directory 700, private key 600
authorized_keys options :: from=, command=, no-port-forwarding restrict a key
ssh-add -t 4h :: load a key into the agent for four hours
ssh -G host :: print the fully resolved config for a host
IdentitiesOnly yes :: offer only the configured key
ProxyJump / -J :: connect through a bastion end to end
ssh -L 5432:db:5432 bastion :: local port forward
ssh -D 1080 :: dynamic SOCKS proxy
BatchMode=yes :: fail instead of prompting (scripts)
ssh -n :: don't read stdin (loops)
sshd -t :: check sshd_config syntax before reloading
rsync -a :: archive: recursive, keep permissions and times
rsync --delete :: remove destination files missing from the source
rsync -n :: dry run
--link-dest :: hard-link unchanged files to a previous snapshot

@@drill
Which ssh option makes a script fail instead of waiting for a password prompt?
= -o BatchMode=yes
= BatchMode=yes
= BatchMode yes

@@drill
Which command prints the resolved SSH configuration for the host web-prod?
= ssh -G web-prod

@@exercise Resolve options from an ssh config
`config` imitates `~/.ssh/config`. Print the options that apply to the host in `target`, the way ssh resolves them: go through the blocks in order; a block applies when any of its `Host` patterns matches the target (glob matching with `*` and `?`); for each option keyword, the **first** value found wins. Print `keyword value` lines in the order the options were first set, with keywords in lower case as `ssh -G` does.
@@starter
target=web-prod
config='Host bastion
    HostName 203.0.113.10
    User ana

Host web-* app-*
    User deploy
    ProxyJump bastion
    Port 2222

Host web-prod
    HostName 10.0.1.20
    Port 22

Host *
    User ana
    ServerAliveInterval 30
    IdentitiesOnly yes'

# Your code here
@@expected
user deploy
proxyjump bastion
port 2222
hostname 10.0.1.20
serveraliveinterval 30
identitiesonly yes
@@hint
Read with `while read -r key value`. When `key` is `Host`, set `match=0`, split the patterns with `read -ra patterns <<< "$value"` (a plain `for p in $value` would expand `*` to file names) and test each pattern with `[[ $target == $pattern ]]` (an unquoted right side is a glob). For other lines, when `match` is 1 and the lower-cased key (`${key,,}`) isn't in an associative array yet, store it and append the key to an ordered list.
@@solution
target=web-prod
config='Host bastion
    HostName 203.0.113.10
    User ana

Host web-* app-*
    User deploy
    ProxyJump bastion
    Port 2222

Host web-prod
    HostName 10.0.1.20
    Port 22

Host *
    User ana
    ServerAliveInterval 30
    IdentitiesOnly yes'

declare -A seen
order=()
match=0
while read -r key value; do
    [[ -z $key ]] && continue
    if [[ $key == Host ]]; then
        match=0
        read -ra patterns <<< "$value"   # an array, so * isn't expanded to file names
        for pattern in "${patterns[@]}"; do
            if [[ $target == $pattern ]]; then
                match=1
            fi
        done
        continue
    fi
    k=${key,,}
    if (( match )) && [[ -z ${seen[$k]+x} ]]; then
        seen[$k]=$value
        order+=("$k")
    fi
done <<< "$config"

for k in "${order[@]}"; do
    echo "$k ${seen[$k]}"
done

@@exercise Plan an rsync mirror
`src` and `dest` list files as `path size mtime`. Print what `rsync -a --delete` would do: `copy` for files missing at the destination, `update` for files whose size or mtime differs, `delete` for files at the destination that are not in the source, then a summary line with the counts and the number unchanged. Keep the source order for copies and updates, and the destination order for deletes.
@@starter
src='index.html 5120 1730880000
css/site.css 2048 1730880100
js/app.js 9100 1730883600
img/logo.svg 1400 1730000000
about.html 3300 1730885000'
dest='index.html 5120 1730880000
css/site.css 2048 1730700000
img/logo.svg 1400 1730000000
old/promo.html 800 1720000000
js/app.js 9000 1730800000
tmp.log 120 1730000000'

# Your code here
@@expected
update css/site.css
update js/app.js
copy   about.html
delete old/promo.html
delete tmp.log
1 to copy, 2 to update, 2 to delete, 2 unchanged
@@hint
With awk you can read two inputs: `awk 'NR == FNR { ... ; next } { ... }' <(echo "$dest") <(echo "$src")`. Store the destination as `d[$1] = $2 " " $3` and keep its order in an array; then for each source line decide copy, update or unchanged, and mark the path as seen. In the `END` block, walk the destination order and print `delete` for paths not seen.
@@solution
src='index.html 5120 1730880000
css/site.css 2048 1730880100
js/app.js 9100 1730883600
img/logo.svg 1400 1730000000
about.html 3300 1730885000'
dest='index.html 5120 1730880000
css/site.css 2048 1730700000
img/logo.svg 1400 1730000000
old/promo.html 800 1720000000
js/app.js 9000 1730800000
tmp.log 120 1730000000'

awk '
NR == FNR { d[$1] = $2 " " $3; order[++n] = $1; next }
{
    seen[$1] = 1
    if (!($1 in d)) { print "copy   " $1; copies++ }
    else if (d[$1] != $2 " " $3) { print "update " $1; updates++ }
    else same++
}
END {
    for (i = 1; i <= n; i++) {
        if (!(order[i] in seen)) { print "delete " order[i]; deletes++ }
    }
    printf "%d to copy, %d to update, %d to delete, %d unchanged\n", copies, updates, deletes, same
}' <(echo "$dest") <(echo "$src")

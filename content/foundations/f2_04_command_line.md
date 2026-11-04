@@lesson f2-cli The Command Line: Windows and Linux
@@text
Graphical tools are fine for one computer. IT work happens at scale, remotely and in a hurry, which is where the command line wins: one line can check 50 servers, a pasted command is easier to document than "click Settings, then...", and many servers have no graphical interface at all. This lesson gives you the core commands on both families of systems. The PowerShell and Bash tracks take each much further.

## Three shells you'll meet

- **PowerShell** (Windows, also on Linux and macOS): commands are `Verb-Noun` cmdlets, and they pass **objects** down the pipeline.
- **Command Prompt** (`cmd.exe`): the old Windows shell; many classic tools (`ipconfig`, `ping`) still run there and in PowerShell.
- **Bash** (Linux, macOS via zsh, Windows via WSL or Git Bash): commands pass **text**.

## Getting around

| Task | PowerShell | Bash |
|---|---|---|
| Where am I? | `Get-Location` (`pwd`) | `pwd` |
| List files | `Get-ChildItem` (`ls`, `dir`) | `ls -la` |
| Change folder | `Set-Location C:\Logs` (`cd`) | `cd /var/log` |
| Make a folder | `New-Item -ItemType Directory Reports` (`mkdir`) | `mkdir reports` |
| Copy / move / delete | `Copy-Item`, `Move-Item`, `Remove-Item` | `cp`, `mv`, `rm` |
| Show a file | `Get-Content app.log` (`cat`) | `cat app.log`, `less app.log` |
| Last lines | `Get-Content app.log -Tail 20` | `tail -n 20 app.log` |
| Follow a log live | `Get-Content app.log -Wait` | `tail -f app.log` |
| Search text | `Select-String error app.log` | `grep -i error app.log` |
| Help | `Get-Help Get-ChildItem -Examples` | `man ls`, `ls --help` |

Paths: Windows uses `C:\Users\ana\Documents` (backslashes, drive letters); Linux uses `/home/ana/documents` (forward slashes, one tree starting at `/`). Linux is **case-sensitive**: `Report.txt` and `report.txt` are different files. **Tab** completes names in both: use it constantly.

## System information and processes

| Task | Windows | Linux |
|---|---|---|
| Computer name | `hostname` | `hostname` |
| OS version | `Get-ComputerInfo`, `winver` | `cat /etc/os-release`, `uname -a` |
| Running processes | `Get-Process` | `ps aux`, `top`, `htop` |
| Stop a process | `Stop-Process -Name notepad` | `kill <pid>`, `pkill firefox` |
| Services | `Get-Service`, `Restart-Service Spooler` | `systemctl status nginx`, `systemctl restart nginx` |
| Disk space | `Get-PSDrive -PSProvider FileSystem`, `Get-Volume` | `df -h`, `du -sh *` |
| Event logs | `Get-WinEvent -LogName System -MaxEvents 20` | `journalctl -u nginx --since today` |
| Logged-in user | `whoami` | `whoami` |
| Uptime | `(Get-CimInstance Win32_OperatingSystem).LastBootUpTime` | `uptime` |

## Networking commands (both families)

| Task | Windows | Linux |
|---|---|---|
| IP configuration | `ipconfig /all` | `ip a` (`ip addr`) |
| Renew DHCP | `ipconfig /release` then `/renew` | `sudo dhclient -r && sudo dhclient` (varies) |
| Clear DNS cache | `ipconfig /flushdns` | `resolvectl flush-caches` |
| Reachability | `ping 8.8.8.8`, `Test-NetConnection` | `ping -c 4 8.8.8.8` |
| Route to a host | `tracert example.com` | `traceroute example.com` |
| DNS lookup | `nslookup example.com`, `Resolve-DnsName` | `dig example.com`, `nslookup` |
| Test a TCP port | `Test-NetConnection server -Port 443` | `nc -zv server 443` |
| Open connections | `netstat -ano`, `Get-NetTCPConnection` | `ss -tulpn` |
| Routing table | `route print` | `ip route` |

## Pipelines and redirection

Both shells chain commands with `|` and redirect output to files:

```text
PowerShell:  Get-Process | Sort-Object CPU -Descending | Select-Object -First 5 Name, CPU
Bash:        ps aux --sort=-%cpu | head -n 6

PowerShell:  Get-Service | Where-Object Status -eq Stopped | Export-Csv stopped.csv
Bash:        grep -c "Failed password" /var/log/auth.log > failed_count.txt
```

`>` writes (overwriting), `>>` appends. In PowerShell, objects keep their properties through the pipeline, so you sort and filter by property names; in Bash you cut and filter text with `grep`, `awk`, `sort`, `uniq`, `cut`.

## Remote command lines

- **SSH**: `ssh admin@server01` gives you a shell on a Linux (or Windows) server, encrypted. Keys (`ssh-keygen`, `ssh-copy-id`) replace passwords.
- **PowerShell Remoting**: `Enter-PSSession -ComputerName PC-014`, or `Invoke-Command -ComputerName (list) -ScriptBlock { ... }` to run on many machines at once.

## Admin rights and safety

- Windows: "Run as administrator"; Linux: prefix with `sudo`. Use elevated rights only when needed.
- Commands like `rm -rf`, `Remove-Item -Recurse -Force`, `format` and `dd` don't ask twice and there's no recycle bin. Read the command, check which folder you're in, and use `-WhatIf` in PowerShell to see what would happen.
- Never paste commands from random websites or "support" emails without understanding them: it's a common attack.

@@quiz
Which command shows full IP configuration including DNS servers on Windows?
- `ping /all`
* `ipconfig /all`
- `netstat -r`
- `hostname /ip`
? On Linux: `ip a` plus `resolvectl status`.

@@quiz
How do you see the last 20 lines of a log on Linux and keep watching new ones?
- `cat -20 app.log`
* `tail -n 20 -f app.log`
- `head -f app.log`
- `less -20 app.log`
? `-f` follows the file.

@@quiz
What does PowerShell's `-WhatIf` do?
* Shows what the command would do without doing it
- Runs the command twice
- Asks for confirmation for each item
- Saves the output to a file
? Use it before risky bulk operations.

@@quiz
What's the main difference between PowerShell and Bash pipelines?
- Bash pipelines are faster
* PowerShell passes objects with properties; Bash passes plain text
- PowerShell can't use pipes
- Bash only works on Windows
? Sort/filter by property in PowerShell, by text in Bash.

@@flashcards
`Get-ChildItem` / `ls` :: list files
`Get-Content -Tail 20` / `tail -n 20` :: last lines
`Select-String` / `grep` :: search text
`Get-Process` / `ps aux` :: running processes
`Restart-Service` / `systemctl restart` :: restart a service
`df -h` :: Linux disk space
`ipconfig /flushdns` :: clear the Windows DNS cache
`Test-NetConnection -Port` :: test a TCP port on Windows
`ss -tulpn` :: listening ports on Linux
`tracert` / `traceroute` :: show the route
`>` / `>>` :: overwrite / append to a file
`ssh user@host` :: remote Linux shell
`Invoke-Command` :: run PowerShell on remote computers
`-WhatIf` :: dry run in PowerShell
`sudo` :: run as root

@@drill
Which Linux command shows disk space in human-readable units?
= df -h
? `du -sh` shows the size of folders.

@@drill
Which Windows command clears the DNS resolver cache?
= ipconfig /flushdns
? Useful after a DNS record changes.

@@exercise Search a log like grep
Write a tiny `grep(pattern, lines, ignore_case=False, count=False)` in Python: return the matching lines (or just how many, when `count` is true). Use it on the sample auth log to print the failed login lines, the number of lines containing "sshd" case-insensitively, and the unique IP addresses of failed logins (the word after `from`), sorted.
@@lang python
@@starter
log = """Mar 05 09:01:12 web01 sshd[811]: Accepted publickey for deploy from 10.0.0.5
Mar 05 09:03:40 web01 sshd[822]: Failed password for root from 203.0.113.7
Mar 05 09:03:44 web01 sshd[822]: Failed password for root from 203.0.113.7
Mar 05 09:05:01 web01 CRON[900]: (root) CMD (backup.sh)
Mar 05 09:07:19 web01 SSHD[831]: Failed password for admin from 198.51.100.23
Mar 05 09:10:02 web01 sshd[840]: Accepted password for asilva from 10.0.0.8""".splitlines()

def grep(pattern, lines, ignore_case=False, count=False):
    raise NotImplementedError
@@expected
Mar 05 09:03:40 web01 sshd[822]: Failed password for root from 203.0.113.7
Mar 05 09:03:44 web01 sshd[822]: Failed password for root from 203.0.113.7
Mar 05 09:07:19 web01 SSHD[831]: Failed password for admin from 198.51.100.23
sshd lines: 5
Failed login sources: 198.51.100.23, 203.0.113.7
@@requires
def grep
.lower()
split()
@@hint
Inside `grep`, compare `pattern.lower() in line.lower()` when `ignore_case`, otherwise `pattern in line`. For the IP, `words = line.split(); words[words.index("from") + 1]`.
@@solution
log = """Mar 05 09:01:12 web01 sshd[811]: Accepted publickey for deploy from 10.0.0.5
Mar 05 09:03:40 web01 sshd[822]: Failed password for root from 203.0.113.7
Mar 05 09:03:44 web01 sshd[822]: Failed password for root from 203.0.113.7
Mar 05 09:05:01 web01 CRON[900]: (root) CMD (backup.sh)
Mar 05 09:07:19 web01 SSHD[831]: Failed password for admin from 198.51.100.23
Mar 05 09:10:02 web01 sshd[840]: Accepted password for asilva from 10.0.0.8""".splitlines()

def grep(pattern, lines, ignore_case=False, count=False):
    if ignore_case:
        matches = [line for line in lines if pattern.lower() in line.lower()]
    else:
        matches = [line for line in lines if pattern in line]
    return len(matches) if count else matches

failed = grep("Failed password", log)
for line in failed:
    print(line)
print("sshd lines:", grep("sshd", log, ignore_case=True, count=True))

ips = set()
for line in failed:
    words = line.split()
    ips.add(words[words.index("from") + 1])
print("Failed login sources:", ", ".join(sorted(ips)))

@@exercise Translate commands between Windows and Linux
Build a dictionary mapping tasks to a `(PowerShell, Bash)` pair for at least these tasks: list files, show the last 20 lines of app.log, search app.log for "error", restart the nginx/Spooler service, show IP addresses, test TCP port 443 on server01, show disk space. Print them as an aligned table with `f"{task:<22}{ps:<45}{bash}"` and a header row.
@@lang python
@@starter
commands = {
    "list files": ("Get-ChildItem", "ls -la"),
    # add the rest
}
@@expected
Task                  PowerShell                                   Bash
list files            Get-ChildItem                                ls -la
last 20 lines         Get-Content app.log -Tail 20                 tail -n 20 app.log
search for error      Select-String error app.log                  grep -i error app.log
restart a service     Restart-Service Spooler                      sudo systemctl restart nginx
show IP addresses     ipconfig /all                                ip a
test port 443         Test-NetConnection server01 -Port 443        nc -zv server01 443
disk space            Get-Volume                                   df -h
@@requires
Test-NetConnection
nc -zv
df -h
:<22
@@hint
Print the header with the same format string: `print(f"{'Task':<22}{'PowerShell':<45}Bash")`.
@@solution
commands = {
    "list files": ("Get-ChildItem", "ls -la"),
    "last 20 lines": ("Get-Content app.log -Tail 20", "tail -n 20 app.log"),
    "search for error": ("Select-String error app.log", "grep -i error app.log"),
    "restart a service": ("Restart-Service Spooler", "sudo systemctl restart nginx"),
    "show IP addresses": ("ipconfig /all", "ip a"),
    "test port 443": ("Test-NetConnection server01 -Port 443", "nc -zv server01 443"),
    "disk space": ("Get-Volume", "df -h"),
}

print(f"{'Task':<22}{'PowerShell':<45}Bash")
for task, (ps, bash) in commands.items():
    print(f"{task:<22}{ps:<45}{bash}")

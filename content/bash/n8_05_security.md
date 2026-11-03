@@lesson n8-security Secure Shell Scripting
@@concepts errors, files, strings
@@text
Shell scripts often run as root, from cron, with input from files, users and the network. Their typical vulnerabilities are well known and avoidable.

## 1. Word splitting and globbing (quote everything)

```bash,norun
file="my report.txt"
rm $file            # removes "my" and "report.txt"
f='*'; rm $f        # removes EVERYTHING in the directory
rm -- "$file"       # correct
```

## 2. Option injection: `--`

A file named `-rf` or `--help` is read as an option. End option parsing with `--`, or prefix relative paths with `./`:

```bash,norun
touch -- '-rf'
rm *            # expands to: rm -rf other files...   (!!)
rm -- *         # safe
rm ./*          # also safe: ./-rf can't be an option
```

## 3. Command injection

Data must never become code. Dangerous constructs with untrusted data: `eval`, `bash -c "...$var..."`, `ssh host "...$var..."` (the remote shell parses it again), `awk "... $var ..."`, `sed "s/$var/.../"`, `find -exec sh -c "... {} ..."`, and unquoted `$(...)`.

```bash,norun
ssh "$host" "ls $dir"                         # $dir='/tmp; rm -rf ~' runs on the server
ssh "$host" "ls $(printf '%q' "$dir")"        # quoted for the remote shell
find . -name '*.log' -exec sh -c 'gzip -- "$1"' _ {} \;   # pass data as arguments
awk -v pat="$pattern" '$0 ~ pat'              # pass data with -v
```

## 4. Validate input with allow-lists

Check that input has the **expected shape** before using it, rather than trying to strip bad characters:

```bash,norun
[[ $user =~ ^[a-z_][a-z0-9_-]{0,31}$ ]] || die "invalid user name"
[[ $port =~ ^[0-9]+$ ]] && (( port >= 1 && port <= 65535 )) || die "invalid port"
case $action in start|stop|status) ;; *) die "unknown action" ;; esac
```

Path traversal: a "file name" like `../../etc/shadow` escapes your directory. Reject `/` and `..`, or resolve with `realpath` and check the prefix.

## 5. Temporary files

`/tmp/myscript.$$` is predictable: another user can create it first as a symlink to a file you own (a **symlink attack**). Always use `mktemp`, which creates the file atomically with mode 600:

```bash,norun
tmp=$(mktemp) || exit 1
trap 'rm -f -- "$tmp"' EXIT
```

## 6. Permissions and umask

Set `umask 077` at the top of scripts that write secrets, so new files are only readable by the owner. Write config atomically: write to a temp file in the same directory, then `mv` it into place (rename is atomic).

## 7. Secrets

- Not on the command line: arguments are visible in `ps` and `/proc/*/cmdline`. Pass secrets via files, stdin or environment variables.
- Not in the script or Git: read them from a file with mode 600, a secrets manager, or `systemd-creds`.
- Not in logs: `set -x` prints expanded variables; turn tracing off around secrets (`{ set +x; } 2>/dev/null`).
- Read passwords with `read -rs`.

## 8. The environment

- Use absolute paths or set `PATH` explicitly in scripts run by root (`PATH=/usr/sbin:/usr/bin:/sbin:/bin`): a writable directory early in `PATH` lets someone plant a fake `ls`.
- `IFS` and `BASH_ENV` can be inherited; scripts run with sudo get a cleaned environment, but cron and systemd units run with whatever you give them.
- Never make shell scripts setuid: Linux ignores the setuid bit on scripts for good reasons. Use `sudo` rules for specific commands instead.

@@quiz
A directory contains a file named `-rf`. What makes `rm *` safe?
- `rm "*"`
* `rm -- *` or `rm ./*`
- `rm -i *`
- Nothing can make it safe
? `--` ends option parsing; `./` prevents names from starting with `-`.

@@quiz
Why is `ssh "$host" "rm -rf $dir"` dangerous even though `$dir` is quoted locally?
* The remote shell parses the command string again, so special characters in `$dir` are interpreted there
- ssh doesn't support quotes
- rm -rf is always dangerous
- The host name can contain spaces
? Quote for the remote shell with printf %q.

@@quiz
What's wrong with `tmp=/tmp/backup.$$`?
- $$ is not a number
* The name is predictable, so another user can pre-create it (for example as a symlink) before your script writes to it
- /tmp is read-only
- Nothing
? mktemp creates a unique file atomically with safe permissions.

@@flashcards
`--` :: end of options
`./*` :: glob that can't produce options
`mktemp` :: safe temporary file
`umask 077` :: new files private to the owner
`printf %q` :: quote for another shell
allow-list :: accept only known-good input
`sh -c '...' _ "$arg"` :: pass data as an argument
secrets on argv :: visible in ps
`mv` in the same directory :: atomic replace
`PATH=...` :: fixed PATH for root scripts

@@exercise Validate before acting
A script receives requests as `action user port file` lines (from a web hook, so they're untrusted). Write validators and process each line:

- action: allow-list `start`, `stop`, `status`
- user: `^[a-z_][a-z0-9_-]{0,31}$`
- port: digits only, 1–65535 (force base 10 with `10#`, and reject more than 5 digits first)
- file: a plain name, no `/`, no leading `-` or `.`, only `[A-Za-z0-9._-]`

Print `ACCEPT ...` or `REJECT (reason): ...`, and show the command you would run as an array printed with `%q`.
@@starter
requests='start alice 8080 app.conf
stop bob 99999 app.conf
status root;id 22 app.conf
start carol 443 ../../etc/shadow
restart dave 80 app.conf
start erin 0080 -rf
start frank 3000 site.v2.conf'
echo TODO
@@expected
ACCEPT: systemctl start app@alice --port=8080 --config=/srv/conf/app.conf
REJECT (port): stop bob 99999 app.conf
REJECT (user): status root\;id 22 app.conf
REJECT (file): start carol 443 ../../etc/shadow
REJECT (action): restart dave 80 app.conf
REJECT (file): start erin 0080 -rf
ACCEPT: systemctl start app@frank --port=3000 --config=/srv/conf/site.v2.conf
@@requires
=~
10#
%q
@@hint
Read with `read -r action user port file`. Write `reason=""` validators in order: action, user, port, file, stopping at the first failure. Print rejected requests with each field through `printf '%q '` so control characters can't mess up your logs. `0080` is a valid port (80) once read in base 10.
@@solution
requests='start alice 8080 app.conf
stop bob 99999 app.conf
status root;id 22 app.conf
start carol 443 ../../etc/shadow
restart dave 80 app.conf
start erin 0080 -rf
start frank 3000 site.v2.conf'

valid_action() { case $1 in start|stop|status) return 0 ;; *) return 1 ;; esac; }
valid_user() { [[ $1 =~ ^[a-z_][a-z0-9_-]{0,31}$ ]]; }
valid_port() { [[ $1 =~ ^[0-9]{1,5}$ ]] && (( 10#$1 >= 1 && 10#$1 <= 65535 )); }
valid_file() { [[ $1 =~ ^[A-Za-z0-9_][A-Za-z0-9._-]*$ && $1 != *..* ]]; }

while read -r action user port file; do
    reason=""
    if ! valid_action "$action"; then reason=action
    elif ! valid_user "$user"; then reason=user
    elif ! valid_port "$port"; then reason=port
    elif ! valid_file "$file"; then reason=file
    fi
    if [[ -n $reason ]]; then
        printf -v shown '%q ' "$action" "$user" "$port" "$file"
        echo "REJECT ($reason): ${shown% }"
        continue
    fi
    cmd=(systemctl "$action" "app@$user" "--port=$(( 10#$port ))" "--config=/srv/conf/$file")
    printf -v shown '%q ' "${cmd[@]}"
    echo "ACCEPT: ${shown% }"
done <<< "$requests"

@@exercise Safe files: mktemp, umask, atomic writes and dangerous names
In a scratch directory:

1. create files named `-rf`, `my notes.txt` and `a.log`, and show that `ls -- *` and a `for f in ./*` loop handle them safely
2. write a secret config with `umask 077` in a subshell, print the umask, and compute the mode new files get (`666 & ~umask`, in octal with `printf %o`). Windows file systems ignore Unix permissions, so don't rely on `stat` here
3. update a config **atomically**: write to `mktemp` in the same directory, then `mv` over the original; show the content and that no temp file is left
4. quote a dangerous path for a remote shell with `%q`
@@starter
echo TODO
@@expected
files: -rf a.log my notes.txt
loop: ./-rf | ./a.log | ./my notes.txt
umask 0077: new files get mode 600
config: port=8443
leftover temp files: 0
remote command: ls /tmp/x\;\ rm\ -rf\ ~
@@requires
mktemp
umask 077
mv
%q
@@hint
List with `printf '%s ' *` (sorted by the glob, which follows the locale: set `LC_ALL=C` for the script to be safe). For the atomic write: `new=$(mktemp "$dir/.app.conf.XXXXXX") && printf 'port=8443\n' > "$new" && mv -f -- "$new" "$dir/app.conf"`. Count leftovers with a glob on `.app.conf.*` and `nullglob`.
@@solution
export LC_ALL=C
dir=$(mktemp -d)
trap 'rm -rf -- "$dir"' EXIT
cd "$dir" || exit 1

touch -- -rf 'my notes.txt' a.log
files=(*)
printf -v listing '%s ' "${files[@]}"
echo "files: ${listing% }"
loop=""
for f in ./*; do loop+="${loop:+ | }$f"; done
echo "loop: $loop"

mask=$( umask 077; printf 'token=s3cr3t\n' > secret.conf; umask )
printf 'umask %s: new files get mode %o\n' "$mask" $(( 8#666 & ~8#$mask ))

printf 'port=80\n' > app.conf
new=$(mktemp "$dir/.app.conf.XXXXXX")
printf 'port=8443\n' > "$new"
mv -f -- "$new" app.conf
echo "config: $(< app.conf)"
shopt -s nullglob dotglob
left=(.app.conf.*)
echo "leftover temp files: ${#left[@]}"

danger='/tmp/x; rm -rf ~'
printf -v remote 'ls %q' "$danger"
echo "remote command: $remote"
cd / || exit 1

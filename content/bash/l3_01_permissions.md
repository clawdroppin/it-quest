@@lesson l3-permissions Users, Groups and Permissions
@@text
## Reading ls -l

```text
-rwxr-x---  1  ana  devops  4096  Oct 31 08:15  deploy.sh
│└┬┘└┬┘└┬┘     │    │
│ │  │  │      │    └─ group
│ │  │  │      └─ owner
│ │  │  └─ others: no access
│ │  └─ group: read, execute
│ └─ owner (user): read, write, execute
└─ type: - file, d directory, l symbolic link
```

| | File | Directory |
|---|---|---|
| **r** (4) | read the contents | list the names |
| **w** (2) | change it | create, delete, rename files in it |
| **x** (1) | run it as a program | enter it (`cd`) and reach files inside |

## Numeric (octal) modes

Add the numbers for each group of three: `rwx` = 4+2+1 = **7**, `r-x` = **5**, `r--` = **4**, `---` = **0**.

| Mode | Means | Typical for |
|---|---|---|
| `755` | rwxr-xr-x | programs, scripts, web directories |
| `644` | rw-r--r-- | normal files, web pages |
| `600` | rw------- | private keys (`~/.ssh/id_ed25519`), secrets |
| `700` | rwx------ | private folders (`~/.ssh`) |
| `777` | everyone can do everything | **almost never correct** |

```bash,norun
chmod 755 deploy.sh
chmod u+x script.sh          # add execute for the user (owner)
chmod go-w file.txt          # remove write from group and others
chmod -R 750 /srv/app        # recursively
chown ana:devops deploy.sh   # change owner and group (needs sudo)
```

## Users, groups and sudo

```bash,norun
id                      # your uid, gid and groups
sudo useradd -m -s /bin/bash kenji      # -m: create the home folder
sudo passwd kenji
sudo usermod -aG sudo kenji             # add to the sudo group (wheel on Red Hat). -a: append!
sudo userdel -r olduser
groups kenji
sudo visudo                             # edit sudo rules safely
```

Accounts live in `/etc/passwd` (anyone can read it), password hashes in `/etc/shadow` (root only), groups in `/etc/group`.

`root` (uid 0) can do anything. Good practice: log in as a normal user and use `sudo` for single commands; it asks for *your* password and logs what you did.

## Special bits (Linux+ loves these)

- **SUID** (`4xxx`, `s` in the user's x position): the program runs as its **owner**. `/usr/bin/passwd` is SUID root so users can change their own password. Unexpected SUID files are a classic privilege-escalation path: `find / -perm -4000 2>/dev/null`.
- **SGID** on a directory (`2xxx`): new files get the directory's group, good for shared team folders.
- **Sticky bit** (`1xxx`, `t`): in `/tmp`, only a file's owner may delete it.
- **umask** removes permissions from new files: with umask `022`, new files are `644` and new folders `755`.

@@quiz
What does `chmod 640 report.txt` give?
* owner read+write, group read, others nothing
- owner read, group write, others nothing
- everyone read
- owner everything, group read
? 6 = rw-, 4 = r--, 0 = ---.

@@quiz
SSH refuses your private key: "permissions are too open". What should they be?
- 644
- 755
* 600
- 777
? A private key must be readable only by its owner.

@@quiz
Why is a program with the SUID bit set worth investigating during a security audit?
* It runs with its owner's privileges (often root), whoever starts it
- It can't be deleted
- It runs faster
- It hides from `ps`
? A vulnerable SUID root program can give a normal user root.

@@drill
What is the octal mode for `rwxr-x---`?
= 750
= 0750
? rwx = 7, r-x = 5, --- = 0.

@@flashcards
r / w / x :: 4 / 2 / 1
755 :: rwxr-xr-x: scripts and directories
644 :: rw-r--r--: normal files
600 :: rw-------: private keys and secrets
chmod / chown :: Change permissions / change owner and group
/etc/shadow :: Password hashes, readable only by root
SUID :: Program runs as its owner (find / -perm -4000)
Sticky bit :: Only the owner can delete files in the folder (/tmp)
umask 022 :: New files 644, new directories 755
usermod -aG :: Add a user to a group (without -a, other groups are removed!)

@@exercise Mode to text
Write `mode_to_text` that turns an octal mode like `754` into `rwxr-xr--`.
@@starter
mode_to_text() {
    echo "TODO: not done yet" >&2
    return 1
}

@@tests
check "$(mode_to_text 754)" "rwxr-xr--" "mode_to_text 754"
check "$(mode_to_text 600)" "rw-------" "mode_to_text 600"
check "$(mode_to_text 777)" "rwxrwxrwx" "mode_to_text 777"
check "$(mode_to_text 000)" "---------" "mode_to_text 000"
@@hint
Loop over the 3 digits with `${1:i:1}` (one character at position i). For each digit d: `(( d & 4 ))` means r, `(( d & 2 ))` w, `(( d & 1 ))` x.
@@solution
mode_to_text() {
    local out="" i d
    for (( i = 0; i < 3; i++ )); do
        d=${1:i:1}
        (( d & 4 )) && out+="r" || out+="-"
        (( d & 2 )) && out+="w" || out+="-"
        (( d & 1 )) && out+="x" || out+="-"
    done
    echo "$out"
}

@@exercise Text to mode
And the other way: write `text_to_mode` that turns `rw-r-----` into `640`.
@@starter
text_to_mode() {
    echo "TODO: not done yet" >&2
    return 1
}

@@tests
check "$(text_to_mode rw-r-----)" "640" "text_to_mode rw-r-----"
check "$(text_to_mode rwxr-xr-x)" "755" "text_to_mode rwxr-xr-x"
check "$(text_to_mode rw-------)" "600" "text_to_mode rw-------"
check "$(text_to_mode ---------)" "000" "text_to_mode ---------"
@@hint
For each group of 3 characters (`${1:0:3}`, `${1:3:3}`, `${1:6:3}`), start at 0 and add 4, 2 or 1 when the character at that place isn't `-`.
@@solution
text_to_mode() {
    local out="" g part d
    for g in 0 3 6; do
        part=${1:g:3}
        d=0
        [[ ${part:0:1} != "-" ]] && (( d += 4 ))
        [[ ${part:1:1} != "-" ]] && (( d += 2 ))
        [[ ${part:2:1} != "-" ]] && (( d += 1 ))
        out+="$d"
    done
    echo "$out"
}

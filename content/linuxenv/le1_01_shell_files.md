@@lesson le1-shell-files The Terminal, Files and Permissions
@@concepts files, strings, text-processing
@@lang bash
@@text
You can use a Linux desktop for months without opening a terminal, but the terminal is where Linux becomes fast. This lesson covers the shell, the filesystem layout, finding things, and permissions: the knowledge every other lesson builds on.

## The shell

The terminal app (GNOME Terminal/Console, Konsole, Ptyxis, Alacritty, kitty...) runs a **shell**: usually **Bash**, sometimes **zsh** (with Oh My Zsh) or **fish** (friendly autosuggestions). The prompt `aiko@thinkpad:~$` shows user, host and current directory; `$` means a normal user, `#` root.

| Keys | Does |
|---|---|
| Tab / Tab Tab | complete a command or path / show the choices |
| ↑ / Ctrl+R | previous commands / search the history |
| Ctrl+C | stop the running command |
| Ctrl+D | end input / log out |
| Ctrl+L | clear the screen |
| Ctrl+A / Ctrl+E | start / end of the line |
| Ctrl+Shift+C / V | copy / paste in most terminals |

Getting help: `man ls` (q to quit, / to search), `ls --help`, `tldr ls` (short examples, install `tldr`), `type cmd` (alias, builtin or file?), `which cmd`.

## The filesystem hierarchy

There are no drive letters: everything hangs off one tree starting at `/`, and other disks are **mounted** into it.

| Path | Holds |
|---|---|
| `/home/aiko` (`~`) | your files and settings |
| `/etc` | system-wide configuration (text files) |
| `/usr/bin`, `/usr/lib` | installed programs and libraries |
| `/usr/share` | icons, docs, `.desktop` launchers, themes |
| `/var/log`, `/var/cache` | logs, caches, other changing data |
| `/tmp` | temporary files, cleared at boot |
| `/opt` | self-contained third-party apps |
| `/mnt`, `/media/aiko` | mounted disks; USB sticks appear under `/run/media` or `/media` |
| `/dev`, `/proc`, `/sys` | devices and live kernel information, not real files |
| `/boot` | kernels and the boot loader |

In your home, **hidden** files and folders start with a dot. Most app settings follow the **XDG** convention: `~/.config/app/` (settings), `~/.local/share/app/` (data), `~/.cache/app/` (deletable cache). Your shell settings live in `~/.bashrc`. Show hidden files with `ls -a` or Ctrl+H in the file manager.

## Moving around and managing files

```text
pwd                     # where am I?
ls -lah                 # long list, hidden files, human sizes
cd ~/Documents          # go there; "cd" alone = home, "cd -" = back
mkdir -p projects/web   # make folders, parents too
cp -r src/ backup/      # copy a folder
mv old.txt new.txt      # move or rename
rm file.txt             # delete (no recycle bin!)  rm -r folder   rm -i asks first
ln -s ~/Sync/notes ~/notes    # a symbolic link (shortcut)
```

Paths are **case-sensitive**: `Photos` and `photos` are different folders. Quote names with spaces: `cd "My Files"`.

## Finding things

```text
find ~ -name "*.pdf" -mtime -7          # PDFs changed in the last 7 days
find ~/Downloads -size +500M             # big files
du -sh ~/* | sort -h                     # what uses the space in my home?
df -h                                    # free space per mounted disk
grep -rn "TODO" ~/projects               # text inside files
locate report.odt                        # fast name search (plocate)
```

Graphical alternatives: the file manager's search, **Baobab** (Disk Usage Analyzer) or **Filelight**, and **ncdu** in the terminal.

## Pipes and redirection

Small tools joined together: `|` sends one command's output into the next; `>` writes it to a file (replacing), `>>` appends, `2>` redirects errors, `2>/dev/null` hides them.

```text
ls ~/Pictures | wc -l                               # how many files?
history | grep apt | tail -5                        # recent apt commands
journalctl -b -p err > errors-today.txt             # save this boot's errors
sort names.txt | uniq -c | sort -rn | head          # most common lines
```

The first exercise is a pipeline that tidies a Downloads folder.

## Permissions

`ls -l` shows something like `-rwxr-x---  1 aiko staff  2048 Nov  2 10:00 backup.sh`:

- the first character is the type: `-` file, `d` directory, `l` link;
- then three groups of `rwx` for the **user** (owner), the **group** and **others**;
- `r` read, `w` write, `x` execute (for a directory: may enter it).

As numbers, r=4, w=2, x=1, added per group: `rwxr-x---` = **750**. Common values: `644` (normal file), `755` (program or folder), `600` (private file such as an SSH key), `700` (private folder).

```text
chmod +x backup.sh            # make a script runnable, then ./backup.sh
chmod 600 ~/.ssh/id_ed25519   # SSH refuses keys others can read
chown aiko:aiko file          # change owner (needs sudo for other people's files)
sudo command                  # run one command as root; you type YOUR password
```

**sudo** gives admin rights for one command; on most desktops your user is in the `sudo` or `wheel` group. Don't run the whole desktop or random scripts as root, and be very careful with `sudo rm`. The second exercise converts permission strings to numbers.

@@quiz
Where do most apps keep your personal settings?
- /etc
* ~/.config (hidden folders in your home)
- /usr/share
- /var/log
? /etc is for system-wide settings; per-user settings follow the XDG convention in your home.

@@quiz
What does `rwxr-x---` equal as a number?
- 755
* 750
- 640
- 770
? user rwx = 7, group r-x = 5, others --- = 0.

@@quiz
What does `2>/dev/null` do?
- Deletes the file named 2
* Throws away the command's error messages
- Runs the command twice
- Sends output to the printer
? File descriptor 2 is stderr; /dev/null discards whatever is written to it.

@@quiz
How do you see which folders take the most space in your home?
- ls -l
* du -sh ~/* | sort -h (or ncdu, or Disk Usage Analyzer)
- df -h only
- free -h
? df shows free space per disk; du measures folders.

@@quiz
Why is `rm` more dangerous than deleting in the file manager?
* It deletes immediately, with no recycle bin to restore from
- It deletes files on every computer on the network
- It always needs root
- It formats the disk
? Use `rm -i`, or `gio trash file` to move things to the trash instead.

@@drill
Which command makes a script executable for its owner?
= chmod u+x
= chmod +x
= chmod 744
? chmod u+x script.sh.

@@flashcards
Tab completion :: finish commands and paths, Tab Tab lists choices
Ctrl+R :: search command history
man / --help / tldr :: built-in manual / short help / example-based help
/ :: the root of the single file tree
~ :: your home directory
/etc :: system-wide configuration
~/.config :: per-user app settings (XDG)
dotfile :: hidden file whose name starts with a dot
/var/log :: log files
mount :: attach a disk at a folder in the tree
ls -lah :: long, all, human-readable list
find -name / -size / -mtime :: search by name / size / age
du -sh vs df -h :: folder sizes vs free disk space
| > >> 2> :: pipe, overwrite, append, redirect errors
r w x = 4 2 1 :: permission values
644 / 755 / 600 :: normal file / program or folder / private file
chmod +x :: make a script executable
sudo :: run one command as root

@@exercise Tidy the Downloads folder
The starter fills a fake Downloads folder. Using `find`, a loop and `mv`, sort the files into sub-folders by extension: `pdf` → `Documents`, `jpg` and `png` → `Pictures`, `iso` → `ISOs`; leave anything else where it is. Create the folders with `mkdir -p`. Extensions may be upper case (`.JPG`). The last lines print the result.
@@starter
cd "$(mktemp -d)"
mkdir Downloads && cd Downloads
touch invoice-2026-10.pdf "holiday 01.JPG" screenshot.png fedora-42.iso notes.txt manual.PDF

# Your code here


find . -type f | sort
@@expected
./Documents/invoice-2026-10.pdf
./Documents/manual.PDF
./ISOs/fedora-42.iso
./Pictures/holiday 01.JPG
./Pictures/screenshot.png
./notes.txt
@@requires
mkdir -p
mv
@@hint
`mkdir -p Documents Pictures ISOs`, then `for f in *.*; do ext="${f##*.}"; case "${ext,,}" in pdf) mv "$f" Documents/ ;; ... esac; done`. `${ext,,}` lower-cases the extension, and quoting `"$f"` keeps names with spaces in one piece.
@@solution
cd "$(mktemp -d)"
mkdir Downloads && cd Downloads
touch invoice-2026-10.pdf "holiday 01.JPG" screenshot.png fedora-42.iso notes.txt manual.PDF

mkdir -p Documents Pictures ISOs
for f in *.*; do
    [ -f "$f" ] || continue
    ext="${f##*.}"
    case "${ext,,}" in
        pdf) mv "$f" Documents/ ;;
        jpg|png) mv "$f" Pictures/ ;;
        iso) mv "$f" ISOs/ ;;
    esac
done

find . -type f | sort

@@exercise Permissions as numbers
Write a function `octal` that turns a 9-character permission string like `rwxr-x---` into its number (`750`), and a function `explain` that takes a number like `640` and prints `user=rw- group=r-- others=---`. Use string slicing (`${s:0:3}`) and arithmetic (`$(( ))`).
@@starter
octal() {
    # Your code here
    :
}

explain() {
    # Your code here
    :
}

for p in rwxr-x--- rw-r--r-- rw------- rwxrwxrwx; do
    echo "$p = $(octal "$p")"
done
explain 640
explain 755
@@expected
rwxr-x--- = 750
rw-r--r-- = 644
rw------- = 600
rwxrwxrwx = 777
user=rw- group=r-- others=---
user=rwx group=r-x others=r-x
@@requires
${
$((
@@hint
For each group `g=${s:i:3}` add 4 if `${g:0:1}` is `r`, 2 if `${g:1:1}` is `w`, 1 if `${g:2:1}` is `x`, and append the digit. For `explain`, loop over the digits `${n:i:1}` and build `rwx` with tests like `(( d & 4 ))`.
@@solution
octal() {
    local s=$1 out="" i g d
    for i in 0 3 6; do
        g=${s:i:3}
        d=0
        [ "${g:0:1}" = r ] && d=$((d + 4))
        [ "${g:1:1}" = w ] && d=$((d + 2))
        [ "${g:2:1}" = x ] && d=$((d + 1))
        out="$out$d"
    done
    echo "$out"
}

explain() {
    local n=$1 parts=() i d s
    for i in 0 1 2; do
        d=${n:i:1}
        s=""
        if (( d & 4 )); then s="${s}r"; else s="${s}-"; fi
        if (( d & 2 )); then s="${s}w"; else s="${s}-"; fi
        if (( d & 1 )); then s="${s}x"; else s="${s}-"; fi
        parts+=("$s")
    done
    echo "user=${parts[0]} group=${parts[1]} others=${parts[2]}"
}

for p in rwxr-x--- rw-r--r-- rw------- rwxrwxrwx; do
    echo "$p = $(octal "$p")"
done
explain 640
explain 755

@@exercise What's using my disk?
The starter prints a `du`-style listing (size in KiB, a tab, the path). Using a pipeline, print the **three biggest** folders, biggest first, with the size converted to MiB (KiB / 1024, rounded down) as `<MiB> MiB  <path>`. Use `sort`, `head` and `awk`.
@@starter
usage() {
    printf '%s\t%s\n' 1048576 /home/aiko/Videos 20480 /home/aiko/Documents 3145728 /home/aiko/.local/share/Steam 512 /home/aiko/Desktop 409600 /home/aiko/.cache 81920 /home/aiko/Pictures
}

# Your pipeline here: usage | ...
@@expected
3072 MiB  /home/aiko/.local/share/Steam
1024 MiB  /home/aiko/Videos
400 MiB  /home/aiko/.cache
@@requires
sort
head
awk
@@hint
`usage | sort -rn | head -3 | awk -F'\t' '{ printf "%d MiB  %s\n", $1 / 1024, $2 }'`. `-rn` sorts numerically, biggest first.
@@solution
usage() {
    printf '%s\t%s\n' 1048576 /home/aiko/Videos 20480 /home/aiko/Documents 3145728 /home/aiko/.local/share/Steam 512 /home/aiko/Desktop 409600 /home/aiko/.cache 81920 /home/aiko/Pictures
}

usage | sort -rn | head -3 | awk -F'\t' '{ printf "%d MiB  %s\n", $1 / 1024, $2 }'

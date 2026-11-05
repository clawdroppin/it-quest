@@lesson le2-packages-deep Packages in Depth: Versions, Dependencies, Ownership and Cleaning Up
@@lang bash
@@text
Package managers do a lot for you. When something goes wrong (a held package, a conflict, a full disk, "which package put this file here?"), it helps to know the questions to ask and the commands that answer them. The examples use apt (Debian/Ubuntu/Mint), with dnf and pacman equivalents.

## Versions

- Debian version strings: `[epoch:]upstream[-revision]`, e.g. `1:2.39-0ubuntu8.4`. The **epoch** wins over everything, then upstream, then revision. `~` sorts **before** anything (so `1.0~rc1` < `1.0`).
- Compare correctly with `dpkg --compare-versions 1.10 gt 1.9 && echo yes`. In plain shell, `sort -V` handles most version strings (not epochs or `~` the Debian way).
- **Pinning and holding**: `sudo apt-mark hold pkg` stops upgrades (`dnf versionlock`, or `IgnorePkg` in pacman.conf). Holds explain "kept back" packages, and so do phased updates on Ubuntu.

## Questions and the commands that answer them

| Question | apt / dpkg | dnf / rpm | pacman |
|---|---|---|---|
| Which package owns this file? | `dpkg -S /usr/bin/convert` | `rpm -qf` | `pacman -Qo` |
| What files did a package install? | `dpkg -L pkg` | `rpm -ql` | `pacman -Ql` |
| Why is this installed? | `apt rdepends --installed pkg`, `aptitude why` | `dnf repoquery --whatrequires` | `pacman -Qi` (Required By) |
| Which package provides a missing command? | `apt-file search bin/foo` | `dnf provides */foo` | `pacman -F foo` |
| What was installed by hand? | `apt-mark showmanual` | `dnf history userinstalled` | `pacman -Qe` |
| Biggest packages? | `dpkg-query -W -f '${Installed-Size}\t${Package}\n'` | `rpm -qa --qf '%{SIZE} %{NAME}\n'` | `expac '%m %n'` |

## Cleaning up

- `sudo apt autoremove --purge`: remove dependencies nothing needs any more, including old kernels (and their config, with `--purge`).
- `sudo apt clean`: delete downloaded .deb files in `/var/cache/apt/archives`.
- `flatpak uninstall --unused`: remove runtimes no app uses. Flatpak runtimes are often the biggest hidden space users.
- `journalctl --vacuum-size=200M`: trim logs.
- Never delete files under `/usr` by hand: the package manager will think they're still there.

## Repositories and trust

- Sources live in `/etc/apt/sources.list.d/*.sources` (the deb822 format), each with a `Signed-By:` key file in `/etc/apt/keyrings/`. Third-party repos and PPAs can replace system packages, so add only ones you trust and remove ones you no longer use.
- **Partial upgrades** on Arch (`pacman -Sy pkg` without `-u`) break things. Always use `pacman -Syu`.

@@quiz
Which version is newer: 1:1.0 or 2.0?
* 1:1.0, because the epoch wins
- 2.0
- They are equal
- It can't be compared
? Epochs exist to fix versioning mistakes.

@@quiz
In Debian versions, how does 1.0~rc1 compare with 1.0?
- It is newer
* It is older; ~ sorts before everything
- They are equal
- It is invalid
? That's why release candidates use ~.

@@quiz
Which command tells you which package installed /usr/bin/convert on Ubuntu?
* dpkg -S /usr/bin/convert
- apt show convert
- which convert
- dpkg -L convert
? dpkg -L lists a package's files.

@@quiz
What removes dependencies that are no longer needed, including old kernels?
- apt clean
* apt autoremove --purge
- apt-mark hold
- rm -rf /usr/lib
? apt clean only empties the download cache.

@@quiz
Why is `pacman -Sy pkg` dangerous on Arch?
* It's a partial upgrade: new libraries may break installed programs
- It deletes the package cache
- It disables signatures
- It downgrades the kernel
? Always run pacman -Syu.

@@flashcards
epoch :: number before the colon, wins over everything
~ in versions :: sorts before everything
dpkg --compare-versions :: correct Debian comparison
sort -V :: version sort for most strings
apt-mark hold :: stop a package from upgrading
dpkg -S :: which package owns a file
dpkg -L :: a package's files
apt-file search :: which package provides a missing file
apt-mark showmanual :: packages you installed yourself
autoremove --purge :: drop unneeded dependencies and their config
apt clean :: empty the .deb cache
flatpak uninstall --unused :: drop unused runtimes
Signed-By :: per-repository key
partial upgrade :: pacman -Sy without -u, avoid it

@@drill
Which apt-mark subcommand stops a package from being upgraded?
= hold
= apt-mark hold
? apt-mark unhold reverses it.

@@drill
Which dpkg option lists the files a package installed?
= -L
= dpkg -L
? -S goes the other way.

@@exercise Who owns these files?
The starter prints `dpkg -S` style lines (`package: /path`) for files, plus some paths that dpkg doesn't know. For each queried path, print `path -> package`, or `path -> not from a package (installed by hand?)`. Then print each package that owns at least one path with its count, sorted by count (descending), then name.
@@starter
dpkg_s() {
    cat <<'EOF'
imagemagick-6.q16: /usr/bin/convert-im6.q16
coreutils: /usr/bin/ls
coreutils: /usr/bin/cp
python3-minimal: /usr/bin/python3.12
coreutils: /usr/bin/sort
libreoffice-core: /usr/lib/libreoffice/program/soffice.bin
EOF
}
queries="/usr/bin/ls /usr/local/bin/mytool /usr/bin/python3.12 /usr/bin/cp /opt/game/run.sh /usr/bin/sort /usr/bin/convert-im6.q16"

echo "/usr/bin/ls -> ?"
@@expected
/usr/bin/ls -> coreutils
/usr/local/bin/mytool -> not from a package (installed by hand?)
/usr/bin/python3.12 -> python3-minimal
/usr/bin/cp -> coreutils
/opt/game/run.sh -> not from a package (installed by hand?)
/usr/bin/sort -> coreutils
/usr/bin/convert-im6.q16 -> imagemagick-6.q16
--
coreutils: 3
imagemagick-6.q16: 1
python3-minimal: 1
@@requires
dpkg_s
sort
uniq -c
@@hint
For each path in `$queries`, grep the dpkg output for `": $path$"` and cut the package before the colon. Collect the owners in a variable (one per line), then count them with `sort | uniq -c | sort -k1,1nr -k2`.
@@solution
dpkg_s() {
    cat <<'EOF'
imagemagick-6.q16: /usr/bin/convert-im6.q16
coreutils: /usr/bin/ls
coreutils: /usr/bin/cp
python3-minimal: /usr/bin/python3.12
coreutils: /usr/bin/sort
libreoffice-core: /usr/lib/libreoffice/program/soffice.bin
EOF
}
queries="/usr/bin/ls /usr/local/bin/mytool /usr/bin/python3.12 /usr/bin/cp /opt/game/run.sh /usr/bin/sort /usr/bin/convert-im6.q16"

owners=""
for path in $queries; do
    pkg=$(dpkg_s | grep -- ": $path\$" | cut -d: -f1)
    if [ -n "$pkg" ]; then
        echo "$path -> $pkg"
        owners="$owners$pkg"$'\n'
    else
        echo "$path -> not from a package (installed by hand?)"
    fi
done
echo "--"
printf '%s' "$owners" | sort | uniq -c | sort -k1,1nr -k2 | awk '{ print $2 ": " $1 }'

@@exercise Find what's outdated and what's big
The starter prints installed versions and the latest available versions of some tools (`name version`). Using `sort -V` to compare (not string comparison), print `name: installed -> latest` for each outdated tool and `name: up to date` otherwise. Then print the three biggest packages from the `dpkg-query` size listing (in KiB) as `name SIZE MiB`, rounded to whole MiB.
@@starter
installed() { printf '%s\n' "node 20.9.0" "git 2.43.0" "python 3.12.3" "ripgrep 14.1.0" "neovim 0.9.5"; }
latest()    { printf '%s\n' "node 20.18.1" "git 2.43.0" "python 3.12.10" "ripgrep 14.1.1" "neovim 0.10.2"; }
sizes() {
    printf '%s\t%s\n' 412345 libreoffice-core 98765 firefox 1534000 linux-firmware 250112 thunderbird 66000 gimp 801234 texlive-base
}

echo "node: up to date"
@@expected
node: 20.9.0 -> 20.18.1
git: up to date
python: 3.12.3 -> 3.12.10
ripgrep: 14.1.0 -> 14.1.1
neovim: 0.9.5 -> 0.10.2
-- biggest packages
linux-firmware 1498 MiB
texlive-base 782 MiB
libreoffice-core 403 MiB
@@requires
sort -V
sizes
head -3
@@hint
Join the lists by name (loop over `installed`, and `latest | awk -v n="$name" '$1 == n {print $2}'`). The newer of two versions is `printf '%s\n' "$a" "$b" | sort -V | tail -1`. Note that a plain string comparison would say "20.9.0" > "20.18.1". For sizes: `sizes | sort -nr | head -3`, then awk with `int($1 / 1024 + 0.5)`.
@@solution
installed() { printf '%s\n' "node 20.9.0" "git 2.43.0" "python 3.12.3" "ripgrep 14.1.0" "neovim 0.9.5"; }
latest()    { printf '%s\n' "node 20.18.1" "git 2.43.0" "python 3.12.10" "ripgrep 14.1.1" "neovim 0.10.2"; }
sizes() {
    printf '%s\t%s\n' 412345 libreoffice-core 98765 firefox 1534000 linux-firmware 250112 thunderbird 66000 gimp 801234 texlive-base
}

installed | while read -r name have; do
    want=$(latest | awk -v n="$name" '$1 == n { print $2 }')
    newest=$(printf '%s\n' "$have" "$want" | sort -V | tail -1)
    if [ "$have" = "$want" ] || [ "$newest" = "$have" ]; then
        echo "$name: up to date"
    else
        echo "$name: $have -> $want"
    fi
done
echo "-- biggest packages"
sizes | sort -nr | head -3 | awk -F'\t' '{ printf "%s %d MiB\n", $2, int($1 / 1024 + 0.5) }'

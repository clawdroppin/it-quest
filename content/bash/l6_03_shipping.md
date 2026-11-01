@@lesson l6-shipping Shipping Shell Tools, and Knowing When to Stop
@@concepts files, processes, automation
@@text
A tool that lives only in your home folder helps one person. This lesson is about getting it onto other machines properly, and about the moment a Bash script should become a program in another language.

## Installing: the standard layout

| Path (with `PREFIX=/usr/local`, or `~/.local` for one user) | What goes there |
|---|---|
| `$PREFIX/bin/tool` | the command (on everyone's `PATH`) |
| `$PREFIX/lib/tool/` | libraries the command sources |
| `$PREFIX/share/bash-completion/completions/tool` | Tab completion, loaded automatically |
| `$PREFIX/share/man/man1/tool.1` | the manual page (`man tool`) |
| `/etc/tool/` | system-wide config |

A `Makefile` is the traditional installer, and `make install` is what Linux users expect:

```text
PREFIX ?= /usr/local

install:
	install -Dm755 tool $(DESTDIR)$(PREFIX)/bin/tool
	install -Dm644 tool-completion.bash $(DESTDIR)$(PREFIX)/share/bash-completion/completions/tool
	install -Dm644 tool.1 $(DESTDIR)$(PREFIX)/share/man/man1/tool.1
```

`install -D` creates the folders and sets permissions in one step. `DESTDIR` lets package builders install into a staging folder instead of the real system.

## Tab completion

```bash,norun
_backup() {
    local cur=${COMP_WORDS[COMP_CWORD]} prev=${COMP_WORDS[COMP_CWORD-1]}
    case $prev in
        -d) COMPREPLY=($(compgen -d -- "$cur")); return ;;     # -d DEST: complete folder names
        -k) COMPREPLY=($(compgen -W "1 7 14 30" -- "$cur")); return ;;
    esac
    COMPREPLY=($(compgen -W "-v -d -k -h" -- "$cur"))
}
complete -F _backup backup
```

`COMP_WORDS` is the command line split into words, `COMP_CWORD` the word being typed, and `compgen -W "list"` filters the list by what's been typed so far.

## A man page

Man pages are written in a small markup language (roff), but you can write Markdown and convert it with `pandoc tool.1.md -s -t man -o tool.1`. Every man page has the same sections: NAME, SYNOPSIS, DESCRIPTION, OPTIONS, EXIT STATUS, FILES, EXAMPLES.

## Packages

For servers, a **package** is the right way: it records which files belong to it, handles upgrades and removal, and declares dependencies.

```text
tool_1.2.0/
├── DEBIAN/control
└── usr/
    ├── bin/tool
    └── share/bash-completion/completions/tool

# DEBIAN/control
Package: tool
Version: 1.2.0
Architecture: all
Maintainer: IT Team <it@contoso.com>
Depends: bash (>= 5), coreutils, gawk
Description: Nightly backup rotation for the file servers

$ dpkg-deb --build --root-owner-group tool_1.2.0
$ sudo apt install ./tool_1.2.0.deb
```

`.rpm` packages (Red Hat, Fedora) are built with `rpmbuild` from a `.spec` file; **fpm** builds both from one command. Companies host them in an internal apt/yum repository, or deploy with Ansible.

## Versions and releases

- Put the version in the script (`VERSION=1.2.0`) and support `--version`.
- Use **semantic versioning**: MAJOR.MINOR.PATCH, with the major number changing when options or output change in a way that breaks other people's scripts.
- Tag releases in Git (`git tag v1.2.0`), and keep a CHANGELOG.

## About `curl ... | bash`

Many tools install with `curl -fsSL https://example.com/install.sh | bash`. It's convenient but it runs whatever the server sends, as you, without you seeing it. For anything you install on company servers: download it, **read it**, check its checksum or signature, then run it. And if you publish an installer like that, wrap everything in a function called at the very end, so a download cut off halfway can't run half a script.

## When to stop using Bash

Bash is the best language for gluing commands together. It's a poor language for:

- data structures beyond arrays and simple maps (nested data, JSON with more than `jq` one-liners)
- arithmetic beyond integers
- anything over a few hundred lines
- calling web APIs with error handling and retries
- running on Windows without WSL or Git Bash

The usual next step is **Python** (in the standard library: `argparse`, `subprocess`, `json`, `pathlib`) or **Go** (one static binary, no dependencies on the target machine). Rewriting is easiest when the Bash version already had tests: the same tests show the new version behaves the same.

@@quiz
Where should an installed command's Tab-completion file go?
- `/etc/profile`
- the user's `.bashrc`
* `$PREFIX/share/bash-completion/completions/<command>`, where bash-completion loads it automatically
- next to the command in `bin/`
? bash-completion loads files from that folder on demand, the first time you press Tab after the command.

@@quiz
What is `DESTDIR` used for in `make install`?
* Installing into a staging folder, so a package can be built without touching the real system
- Choosing the target CPU architecture
- Setting the download location
- Installing documentation only
? Packagers run `make install DESTDIR=/tmp/pkgroot PREFIX=/usr` and package what lands there.

@@quiz
Why is a `.deb` package better than copying the script with `scp` to 50 servers?
- It runs faster
* The package manager tracks the files, versions, upgrades, removal and dependencies
- `.deb` files are encrypted
- `scp` can't copy executable files
? `apt list --installed | grep tool` tells you exactly which version every server has.

@@quiz
Your 600-line Bash script now parses JSON from three APIs and does floating-point maths. What's the sensible next step?
- Split it into six 100-line Bash scripts
* Rewrite it in Python or Go, reusing its tests to check the new version
- Add more `jq` and `bc` calls
- Nothing, Bash scales fine
? Know your tools' limits. Bash glues programs together; it isn't meant for complex data.

@@flashcards
`PREFIX` :: install root: `/usr/local` (everyone) or `~/.local` (one user)
`install -Dm755 src dest` :: create folders, copy, and set permissions in one step
`DESTDIR` :: staging folder for building packages
`complete -F func cmd` :: use `func` for Tab completion of `cmd`
`compgen -W "words" -- "$cur"` :: the words that start with what's typed
`COMP_WORDS` / `COMP_CWORD` :: the words on the command line / which one is being completed
man page sections :: NAME, SYNOPSIS, DESCRIPTION, OPTIONS, EXIT STATUS, FILES, EXAMPLES
`DEBIAN/control` :: a .deb's metadata: name, version, dependencies
`dpkg-deb --build` :: builds a .deb from a folder
fpm :: builds .deb and .rpm packages from one command
semantic versioning :: MAJOR.MINOR.PATCH

@@exercise Completion candidates
Without the completion machinery, write the core of it: `complete_words CURRENT WORD...` prints (one per line, in order) every `WORD` that starts with `CURRENT`. An empty `CURRENT` matches everything. This is what `compgen -W` does, so **don't use compgen**.
@@starter
complete_words() {
    echo "TODO" >&2; return 1
}

@@tests
check "$(complete_words re add list remove rename report)" $'remove\nrename\nreport' "prefix re"
check "$(complete_words '' start stop)" $'start\nstop' "empty prefix"
check "$(complete_words x start stop)" "" "no match"
check "$(complete_words -- --help --version -v)" $'--help\n--version' "prefixes starting with a dash"
@@hint
`local cur=$1; shift; local w; for w in "$@"; do [[ $w == "$cur"* ]] && printf '%s\n' "$w"; done`. Quoting `"$cur"` makes it match literally, not as a pattern. Finish with `return 0`, or a final non-match makes the function's exit code 1.
@@solution
complete_words() {
    local cur=$1 w
    shift
    for w in "$@"; do
        [[ $w == "$cur"* ]] && printf '%s\n' "$w"
    done
    return 0
}

@@exercise A version check
Install scripts often need "is the installed version at least X?". Write `version_ge A B` that **returns 0** when version `A` is greater than or equal to `B` and 1 otherwise, comparing MAJOR.MINOR.PATCH numerically (so `1.10.0` is newer than `1.9.3`). Missing parts count as 0 (`2.1` = `2.1.0`).
@@starter
version_ge() {
    echo "TODO" >&2; return 1
}

@@tests
v() { version_ge "$1" "$2" && echo yes || echo no; }
check "$(v 1.10.0 1.9.3)" "yes" "1.10.0 >= 1.9.3"
check "$(v 1.9.3 1.10.0)" "no" "1.9.3 < 1.10.0"
check "$(v 2.0.0 2.0.0)" "yes" "equal"
check "$(v 2.1 2.1.0)" "yes" "missing part is 0"
check "$(v 2.1 2.1.1)" "no" "2.1 < 2.1.1"
check "$(v 10.0.0 9.99.99)" "yes" "major wins"
@@hint
Split with `IFS=. read -r a1 a2 a3 <<< "$1"` (and the same for `$2`), then compare each part with `${a1:-0}` defaults: `if (( a1 != b1 )); then (( a1 > b1 )); return; fi`, and so on. (`sort -V` can also do it: `[[ $(printf '%s\n' "$2" "$1" | sort -V | head -1) == "$2" ]]`.)
@@solution
version_ge() {
    local a1 a2 a3 b1 b2 b3
    IFS=. read -r a1 a2 a3 <<< "$1"
    IFS=. read -r b1 b2 b3 <<< "$2"
    a1=${a1:-0} a2=${a2:-0} a3=${a3:-0} b1=${b1:-0} b2=${b2:-0} b3=${b3:-0}
    if (( a1 != b1 )); then (( a1 > b1 )); return; fi
    if (( a2 != b2 )); then (( a2 > b2 )); return; fi
    (( a3 >= b3 ))
}

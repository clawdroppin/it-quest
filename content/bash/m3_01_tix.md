@@lesson m3-tix tix: a Help-Desk Ticket Tracker for the Command Line
@@text
Git, Docker, kubectl and the AWS CLI all work the same way: one program, several **subcommands**, short options, helpful errors, clean exit codes, and tab completion. You'll build one of your own: `tix`, a ticket tracker for a small IT team or a home lab, written in Bash and installed with `make install`.

```text
$ tix add -p urgent -a yuki VPN down for Osaka office
Created ticket #2
$ tix list
#2    urgent  yuki       VPN down for Osaka office
#4    high    ken        Email bounces
#1    normal  -          Printer on 3F is offline
$ tix close 2
Closed ticket #2
$ tix close 2
tix: ticket #2 is already closed
$ echo $?
1
$ tix report
open: 2  closed: 1
open by priority: high=1 normal=1
open by assignee:
  -          1
  ken        1
```

## How it's built

| File | Job |
|---|---|
| `tix` | the program: `set -euo pipefail`, find and `source` the library, call `main "$@"` |
| `tixlib.sh` | every command as a function (`cmd_add`, `cmd_list`, ...) and `main`, which dispatches |
| `tests.sh` | 36 tests that run the real `tix` against a temporary data file |
| `tix-completion.bash` | Tab completion for commands, priorities and ticket numbers |
| `Makefile` | `make test`, `make lint`, `make install` and `make uninstall` |

Tickets are one **tab-separated** line each, in `~/.local/share/tix/tickets.tsv` (the XDG data folder), or in `$TIX_FILE` when it's set, which is how the tests use a temporary file:

```text
1	open	normal	-	2026-11-01	Printer on 3F is offline
2	closed	urgent	yuki	2026-11-01	VPN down for Osaka office
```

## Your tasks

Four functions in `tixlib.sh` print `TODO`. The comment above each one says exactly what to do:

1. `next_id`: one line of `awk` that finds the highest id.
2. `update_field`: `awk` with `-v` variables to change one column of one line, writing to a temp file and `mv`-ing it over the original.
3. `cmd_add`: `getopts ":p:a:"`, validation, and appending a line with `printf '%s\t%s...'`.
4. `cmd_list`: filter with `awk`, sort by priority rank and then id with `sort -t$'\t' -k1,1n -k2,2n`, then print the columns with `printf '%-5s %-7s %-10s %s\n'`.

`cmd_close`, `cmd_show` and the others are written for you. Read them first: they show the style.

```text
bash tests.sh            # or: make test
```

Then install it for yourself with `make install` (make sure `~/.local/bin` is on your `PATH`), and `source tix-completion.bash` to try Tab completion.
@@project tix tix
Write the four functions that print `TODO` in `tixlib.sh` until `bash tests.sh` passes, then install `tix` with `make install`.
@@file tix
#!/usr/bin/env bash
# tix: a help-desk ticket tracker for the command line. Run "tix help".
set -euo pipefail
# Find tixlib.sh next to this script (or next to where a symlink to it points).
here=$(dirname "$(readlink -f "${BASH_SOURCE[0]}")")
source "$here/tixlib.sh"
main "$@"
@@file tixlib.sh
#!/usr/bin/env bash
# tixlib.sh: everything tix does, as functions. The tix script only calls main.
# Tickets live in one tab-separated file, one ticket per line:
#   id <TAB> status <TAB> priority <TAB> assignee <TAB> created <TAB> title
# status is open or closed; priority is urgent, high, normal or low; assignee is "-" when nobody.

TIX_FILE="${TIX_FILE:-${XDG_DATA_HOME:-$HOME/.local/share}/tix/tickets.tsv}"
PRIORITIES="urgent high normal low"

die() { echo "tix: $*" >&2; exit 1; }
today() { echo "${TIX_TODAY:-$(date +%F)}"; }

ensure_store() {
    mkdir -p "$(dirname "$TIX_FILE")"
    [[ -f "$TIX_FILE" ]] || : > "$TIX_FILE"
}

# Tabs and newlines would break the file format, so they become spaces.
clean_field() { printf '%s' "$*" | tr '\t\n\r' '   '; }

is_priority() { [[ " $PRIORITIES " == *" $1 "* ]]; }

# Prints the ticket's line, or fails with "no ticket #ID".
get_ticket() {
    [[ "$1" =~ ^[0-9]+$ ]] || die "not a ticket number: $1"
    local line
    line=$(awk -F'\t' -v id="$1" '$1 == id' "$TIX_FILE")
    [[ -n "$line" ]] || die "no ticket #$1"
    printf '%s\n' "$line"
}

# The next ticket number: one more than the highest in the file (1 for an empty file).
# Not "number of lines + 1": lines may be out of order, or removed by hand.
next_id() {
    echo "TODO: not done yet" >&2
    return 1
}

# update_field ID COLUMN VALUE: sets one column (1-6) of one ticket.
# Writes a temporary file next to the real one and renames it over, so a crash
# never leaves a half-written file.
update_field() {
    echo "TODO: not done yet" >&2
    return 1
}

usage() {
    cat <<'EOF'
usage: tix <command> [options]

  add [-p PRIORITY] [-a USER] TITLE...   create a ticket (priority: urgent high normal low)
  list [-a USER] [-p PRIORITY] [-c]      open tickets, most urgent first (-c: closed ones instead)
  show ID                                one ticket in full
  assign ID USER                         give a ticket to someone ("-" for nobody)
  close ID                               close a ticket
  reopen ID                              reopen a closed ticket
  report                                 counts by status, priority and assignee
  help                                   this text

Tickets are stored in $TIX_FILE (default ~/.local/share/tix/tickets.tsv).
EOF
}

# add [-p PRIORITY] [-a USER] TITLE...
# Prints "Created ticket #ID". The title is required. Errors: "unknown priority: X",
# "a title is required", and getopts' own message for an unknown option (exit 2 with usage).
cmd_add() {
    echo "TODO: not done yet" >&2
    return 1
}

# list [-a USER] [-p PRIORITY] [-c]
# Open tickets (or closed ones with -c), filtered, sorted by priority (urgent first), then by id.
# One line each:  printf '%-5s %-7s %-10s %s\n' "#ID" PRIORITY ASSIGNEE TITLE
# Prints "no tickets" when nothing matches.
cmd_list() {
    echo "TODO: not done yet" >&2
    return 1
}

cmd_show() {
    [[ $# -eq 1 ]] || die "usage: tix show ID"
    local id status priority assignee created title
    IFS=$'\t' read -r id status priority assignee created title <<< "$(get_ticket "$1")"
    printf 'Ticket #%s: %s\n' "$id" "$title"
    printf '  status:   %s\n  priority: %s\n  assignee: %s\n  created:  %s\n' "$status" "$priority" "$assignee" "$created"
}

cmd_assign() {
    [[ $# -eq 2 ]] || die "usage: tix assign ID USER"
    get_ticket "$1" > /dev/null
    update_field "$1" 4 "$2"
    echo "Ticket #$1 assigned to $2"
}

cmd_close() {
    [[ $# -eq 1 ]] || die "usage: tix close ID"
    local status
    status=$(get_ticket "$1" | cut -f2)
    [[ "$status" == open ]] || die "ticket #$1 is already closed"
    update_field "$1" 2 closed
    echo "Closed ticket #$1"
}

cmd_reopen() {
    [[ $# -eq 1 ]] || die "usage: tix reopen ID"
    local status
    status=$(get_ticket "$1" | cut -f2)
    [[ "$status" == closed ]] || die "ticket #$1 is already open"
    update_field "$1" 2 open
    echo "Reopened ticket #$1"
}

cmd_report() {
    awk -F'\t' '
        { status[$2]++ }
        $2 == "open" { prio[$3]++ }
        END {
            printf "open: %d  closed: %d\n", status["open"], status["closed"]
            n = split("urgent high normal low", order, " ")
            line = "open by priority:"
            for (i = 1; i <= n; i++) if (prio[order[i]]) line = line " " order[i] "=" prio[order[i]]
            print line
        }
    ' "$TIX_FILE"
    echo "open by assignee:"
    awk -F'\t' '$2 == "open" { who[$4]++ } END { for (w in who) printf "  %-10s %d\n", w, who[w] }' "$TIX_FILE" | sort
}

main() {
    local cmd=${1:-help}
    [[ $# -gt 0 ]] && shift
    case $cmd in
        help | -h | --help) usage; return ;;
        add | list | show | assign | close | reopen | report) ;;
        *) echo "tix: unknown command: $cmd" >&2; usage >&2; exit 2 ;;
    esac
    ensure_store
    "cmd_$cmd" "$@"
}
@@file tests.sh
#!/usr/bin/env bash
# Run: bash tests.sh
cd "$(dirname "$0")"
export LC_ALL=C
export TIX_TODAY=2026-11-01
work=$(mktemp -d)
export TIX_FILE="$work/tickets.tsv"
trap 'rm -rf "$work"' EXIT
failed=0

assert_eq() {
    if [[ "$1" == "$2" ]]; then
        echo "  ok    $3"
    else
        echo "  FAIL  $3"
        echo "        expected: $(printf '%s' "$2" | tr '\n' '|')"
        echo "        got:      $(printf '%s' "$1" | tr '\n' '|')"
        failed=$(( failed + 1 ))
    fi
}

tix() { bash ./tix "$@" 2>&1; }      # run the real program, errors included
status() { bash ./tix "$@" > /dev/null 2>&1; echo $?; }

echo "next_id"
source ./tixlib.sh
: > "$TIX_FILE"
assert_eq "$(next_id)" "1" "empty file"
printf '3\topen\tnormal\t-\t2026-10-01\ta\n9\tclosed\tlow\t-\t2026-10-02\tb\n4\topen\tlow\t-\t2026-10-03\tc\n' > "$TIX_FILE"
assert_eq "$(next_id)" "10" "after the highest, not the last"
: > "$TIX_FILE"

echo "add"
assert_eq "$(tix add Printer on 3F is offline)" "Created ticket #1" "a title from several words"
assert_eq "$(tix add -p urgent -a yuki VPN down for Osaka office)" "Created ticket #2" "with options"
assert_eq "$(tix add -p low "Order more toner")" "Created ticket #3" "a quoted title"
assert_eq "$(tix add -p high -a ken $'Email\tbounces')" "Created ticket #4" "a tab in the title"
assert_eq "$(sed -n 1p "$TIX_FILE")" $'1\topen\tnormal\t-\t2026-11-01\tPrinter on 3F is offline' "the stored line"
assert_eq "$(sed -n 4p "$TIX_FILE" | cut -f6)" "Email bounces" "tabs become spaces"
assert_eq "$(tix add -p whenever Coffee)" "tix: unknown priority: whenever" "bad priority"
assert_eq "$(tix add -p high)" "tix: a title is required" "no title"
assert_eq "$(status add -x thing)" "2" "unknown option exits 2"
assert_eq "$(wc -l < "$TIX_FILE" | tr -d ' ')" "4" "errors add nothing"

echo "list"
assert_eq "$(tix list)" "$(printf '%s\n' \
    '#2    urgent  yuki       VPN down for Osaka office' \
    '#4    high    ken        Email bounces' \
    '#1    normal  -          Printer on 3F is offline' \
    '#3    low     -          Order more toner')" "sorted by priority"
assert_eq "$(tix list -a ken)" "#4    high    ken        Email bounces" "by assignee"
assert_eq "$(tix list -p low)" "#3    low     -          Order more toner" "by priority"
assert_eq "$(tix list -a nobody)" "no tickets" "nothing matches"
tix add -p urgent Server room too hot > /dev/null
assert_eq "$(tix list | head -2 | cut -c1-12)" $'#2    urgent\n#5    urgent' "same priority: by id"

echo "assign, close, reopen"
assert_eq "$(tix assign 1 aiko)" "Ticket #1 assigned to aiko" "assign"
assert_eq "$(tix list -a aiko)" "#1    normal  aiko       Printer on 3F is offline" "assigned"
assert_eq "$(tix close 2)" "Closed ticket #2" "close"
assert_eq "$(tix close 2)" "tix: ticket #2 is already closed" "close twice"
assert_eq "$(tix list -c)" "#2    urgent  yuki       VPN down for Osaka office" "closed list"
assert_eq "$(tix list | grep -c '#2 ')" "0" "closed tickets leave the open list"
assert_eq "$(tix reopen 2)" "Reopened ticket #2" "reopen"
assert_eq "$(tix close 99)" "tix: no ticket #99" "unknown ticket"
assert_eq "$(tix close abc)" "tix: not a ticket number: abc" "not a number"
assert_eq "$(sed -n 3p "$TIX_FILE")" $'3\topen\tlow\t-\t2026-11-01\tOrder more toner' "other lines untouched"
assert_eq "$(ls "$work")" "tickets.tsv" "no temporary files left"

echo "show and report"
assert_eq "$(tix show 4)" "$(printf '%s\n' 'Ticket #4: Email bounces' '  status:   open' '  priority: high' '  assignee: ken' '  created:  2026-11-01')" "show"
tix close 3 > /dev/null
assert_eq "$(tix report)" "$(printf '%s\n' 'open: 4  closed: 1' 'open by priority: urgent=2 high=1 normal=1' 'open by assignee:' \
    '  -          1' '  aiko       1' '  ken        1' '  yuki       1')" "report"

echo "commands"
assert_eq "$(tix frobnicate | head -1)" "tix: unknown command: frobnicate" "unknown command"
assert_eq "$(status frobnicate)" "2" "unknown command exits 2"
assert_eq "$(tix help | head -1)" "usage: tix <command> [options]" "help"
assert_eq "$(status close 99)" "1" "errors exit 1"
TIX_FILE="$work/new/dir/t.tsv" bash ./tix list > /dev/null
assert_eq "$([[ -f "$work/new/dir/t.tsv" ]] && echo yes)" "yes" "creates the store"

if (( failed > 0 )); then echo "$failed test(s) failed"; exit 1; fi
echo "All tests passed"
@@file tix-completion.bash
# Tab completion for tix. Load it with:  source tix-completion.bash
# ("make install" copies it to where bash-completion finds it automatically.)
_tix() {
    local cur=${COMP_WORDS[COMP_CWORD]} prev=${COMP_WORDS[COMP_CWORD-1]}
    if (( COMP_CWORD == 1 )); then
        COMPREPLY=($(compgen -W "add list show assign close reopen report help" -- "$cur"))
        return
    fi
    case $prev in
        -p) COMPREPLY=($(compgen -W "urgent high normal low" -- "$cur")); return ;;
    esac
    case ${COMP_WORDS[1]} in
        show | assign | close | reopen)
            if (( COMP_CWORD == 2 )); then
                # offer the ticket numbers that exist
                local file=${TIX_FILE:-${XDG_DATA_HOME:-$HOME/.local/share}/tix/tickets.tsv}
                [[ -f $file ]] && COMPREPLY=($(compgen -W "$(cut -f1 "$file")" -- "$cur"))
            fi ;;
        add) COMPREPLY=($(compgen -W "-p -a" -- "$cur")) ;;
        list) COMPREPLY=($(compgen -W "-p -a -c" -- "$cur")) ;;
    esac
}
complete -F _tix tix
@@file Makefile
# make test                     run the tests
# make install                  install for yourself, into ~/.local
# sudo make install PREFIX=/usr/local   install for everyone
PREFIX ?= $(HOME)/.local
LIBDIR = $(PREFIX)/lib/tix
BINDIR = $(PREFIX)/bin
COMPDIR = $(PREFIX)/share/bash-completion/completions

.PHONY: test install uninstall lint

test:
	bash tests.sh

lint:
	shellcheck tix tixlib.sh tests.sh tix-completion.bash

install:
	install -d $(LIBDIR) $(BINDIR) $(COMPDIR)
	install -m 755 tix $(LIBDIR)/tix
	install -m 644 tixlib.sh $(LIBDIR)/tixlib.sh
	ln -sf $(LIBDIR)/tix $(BINDIR)/tix
	install -m 644 tix-completion.bash $(COMPDIR)/tix

uninstall:
	rm -rf $(LIBDIR) $(BINDIR)/tix $(COMPDIR)/tix
@@text
## Stretch goals

- **`tix edit ID`**: open the title in `$EDITOR` (write it to a temp file, run the editor, read it back).
- **Comments**: `tix note ID text` appending to a second file, `notes.tsv`, shown by `tix show`.
- **Locking**: two people running `tix add` at the same moment can both get the same id. Wrap changes in `flock` (`exec 9>"$TIX_FILE.lock"; flock 9`).
- **Colour**, only when the output is a terminal: `[[ -t 1 ]]`, and respect the `NO_COLOR` environment variable.
- **Due dates and `tix overdue`**: `date -d "$created + 3 days" +%F` and string comparison of ISO dates.
- **Package it**: build a `.deb` with `dpkg-deb --build` (a `DEBIAN/control` file plus the `make install` layout under `usr/`).
- **ShellCheck it**: `make lint`, and fix every warning.
@@project tix_finished tix: the finished version
A complete solution to compare with yours.
@@file tix
#!/usr/bin/env bash
# tix: a help-desk ticket tracker for the command line. Run "tix help".
set -euo pipefail
# Find tixlib.sh next to this script (or next to where a symlink to it points).
here=$(dirname "$(readlink -f "${BASH_SOURCE[0]}")")
source "$here/tixlib.sh"
main "$@"
@@file tixlib.sh
#!/usr/bin/env bash
# tixlib.sh: everything tix does, as functions. The tix script only calls main.
# Tickets live in one tab-separated file, one ticket per line:
#   id <TAB> status <TAB> priority <TAB> assignee <TAB> created <TAB> title
# status is open or closed; priority is urgent, high, normal or low; assignee is "-" when nobody.

TIX_FILE="${TIX_FILE:-${XDG_DATA_HOME:-$HOME/.local/share}/tix/tickets.tsv}"
PRIORITIES="urgent high normal low"

die() { echo "tix: $*" >&2; exit 1; }
today() { echo "${TIX_TODAY:-$(date +%F)}"; }

ensure_store() {
    mkdir -p "$(dirname "$TIX_FILE")"
    [[ -f "$TIX_FILE" ]] || : > "$TIX_FILE"
}

# Tabs and newlines would break the file format, so they become spaces.
clean_field() { printf '%s' "$*" | tr '\t\n\r' '   '; }

is_priority() { [[ " $PRIORITIES " == *" $1 "* ]]; }

# Prints the ticket's line, or fails with "no ticket #ID".
get_ticket() {
    [[ "$1" =~ ^[0-9]+$ ]] || die "not a ticket number: $1"
    local line
    line=$(awk -F'\t' -v id="$1" '$1 == id' "$TIX_FILE")
    [[ -n "$line" ]] || die "no ticket #$1"
    printf '%s\n' "$line"
}

# The next ticket number: one more than the highest in the file (1 for an empty file).
# Not "number of lines + 1": lines may be out of order, or removed by hand.
next_id() {
    awk -F'\t' '$1 > max { max = $1 } END { print max + 1 }' "$TIX_FILE"
}

# update_field ID COLUMN VALUE: sets one column (1-6) of one ticket.
# Writes a temporary file next to the real one and renames it over, so a crash
# never leaves a half-written file.
update_field() {
    local id=$1 col=$2 value
    value=$(clean_field "$3")
    local tmp="$TIX_FILE.tmp.$$"
    awk -F'\t' -v OFS='\t' -v id="$id" -v col="$col" -v value="$value" \
        '$1 == id { $col = value } { print }' "$TIX_FILE" > "$tmp" && mv "$tmp" "$TIX_FILE"
}

usage() {
    cat <<'EOF'
usage: tix <command> [options]

  add [-p PRIORITY] [-a USER] TITLE...   create a ticket (priority: urgent high normal low)
  list [-a USER] [-p PRIORITY] [-c]      open tickets, most urgent first (-c: closed ones instead)
  show ID                                one ticket in full
  assign ID USER                         give a ticket to someone ("-" for nobody)
  close ID                               close a ticket
  reopen ID                              reopen a closed ticket
  report                                 counts by status, priority and assignee
  help                                   this text

Tickets are stored in $TIX_FILE (default ~/.local/share/tix/tickets.tsv).
EOF
}

# add [-p PRIORITY] [-a USER] TITLE...
# Prints "Created ticket #ID". The title is required. Errors: "unknown priority: X",
# "a title is required", and getopts' own message for an unknown option (exit 2 with usage).
cmd_add() {
    local priority=normal assignee=- opt OPTIND=1
    while getopts ":p:a:" opt; do
        case $opt in
            p) priority=$OPTARG ;;
            a) assignee=$OPTARG ;;
            :) die "-$OPTARG needs a value" ;;
            \?) echo "tix: unknown option -$OPTARG" >&2; usage >&2; exit 2 ;;
        esac
    done
    shift $((OPTIND - 1))
    is_priority "$priority" || die "unknown priority: $priority"
    local title
    title=$(clean_field "$*")
    [[ -n "${title// /}" ]] || die "a title is required"
    local id
    id=$(next_id)
    printf '%s\t%s\t%s\t%s\t%s\t%s\n' "$id" open "$priority" "$(clean_field "$assignee")" "$(today)" "$title" >> "$TIX_FILE"
    echo "Created ticket #$id"
}

# list [-a USER] [-p PRIORITY] [-c]
# Open tickets (or closed ones with -c), filtered, sorted by priority (urgent first), then by id.
# One line each:  printf '%-5s %-7s %-10s %s\n' "#ID" PRIORITY ASSIGNEE TITLE
# Prints "no tickets" when nothing matches.
cmd_list() {
    local status=open assignee="" priority="" opt OPTIND=1
    while getopts ":a:p:c" opt; do
        case $opt in
            a) assignee=$OPTARG ;;
            p) priority=$OPTARG ;;
            c) status=closed ;;
            :) die "-$OPTARG needs a value" ;;
            \?) echo "tix: unknown option -$OPTARG" >&2; usage >&2; exit 2 ;;
        esac
    done
    local rows
    rows=$(awk -F'\t' -v OFS='\t' -v s="$status" -v a="$assignee" -v p="$priority" '
        BEGIN { rank["urgent"] = 0; rank["high"] = 1; rank["normal"] = 2; rank["low"] = 3 }
        $2 == s && (a == "" || $4 == a) && (p == "" || $3 == p) { print rank[$3], $1, $3, $4, $6 }
    ' "$TIX_FILE" | sort -t$'\t' -k1,1n -k2,2n)
    if [[ -z "$rows" ]]; then
        echo "no tickets"
        return
    fi
    local rank id prio who title
    while IFS=$'\t' read -r rank id prio who title; do
        printf '%-5s %-7s %-10s %s\n' "#$id" "$prio" "$who" "$title"
    done <<< "$rows"
}

cmd_show() {
    [[ $# -eq 1 ]] || die "usage: tix show ID"
    local id status priority assignee created title
    IFS=$'\t' read -r id status priority assignee created title <<< "$(get_ticket "$1")"
    printf 'Ticket #%s: %s\n' "$id" "$title"
    printf '  status:   %s\n  priority: %s\n  assignee: %s\n  created:  %s\n' "$status" "$priority" "$assignee" "$created"
}

cmd_assign() {
    [[ $# -eq 2 ]] || die "usage: tix assign ID USER"
    get_ticket "$1" > /dev/null
    update_field "$1" 4 "$2"
    echo "Ticket #$1 assigned to $2"
}

cmd_close() {
    [[ $# -eq 1 ]] || die "usage: tix close ID"
    local status
    status=$(get_ticket "$1" | cut -f2)
    [[ "$status" == open ]] || die "ticket #$1 is already closed"
    update_field "$1" 2 closed
    echo "Closed ticket #$1"
}

cmd_reopen() {
    [[ $# -eq 1 ]] || die "usage: tix reopen ID"
    local status
    status=$(get_ticket "$1" | cut -f2)
    [[ "$status" == closed ]] || die "ticket #$1 is already open"
    update_field "$1" 2 open
    echo "Reopened ticket #$1"
}

cmd_report() {
    awk -F'\t' '
        { status[$2]++ }
        $2 == "open" { prio[$3]++ }
        END {
            printf "open: %d  closed: %d\n", status["open"], status["closed"]
            n = split("urgent high normal low", order, " ")
            line = "open by priority:"
            for (i = 1; i <= n; i++) if (prio[order[i]]) line = line " " order[i] "=" prio[order[i]]
            print line
        }
    ' "$TIX_FILE"
    echo "open by assignee:"
    awk -F'\t' '$2 == "open" { who[$4]++ } END { for (w in who) printf "  %-10s %d\n", w, who[w] }' "$TIX_FILE" | sort
}

main() {
    local cmd=${1:-help}
    [[ $# -gt 0 ]] && shift
    case $cmd in
        help | -h | --help) usage; return ;;
        add | list | show | assign | close | reopen | report) ;;
        *) echo "tix: unknown command: $cmd" >&2; usage >&2; exit 2 ;;
    esac
    ensure_store
    "cmd_$cmd" "$@"
}
@@file tests.sh
#!/usr/bin/env bash
# Run: bash tests.sh
cd "$(dirname "$0")"
export LC_ALL=C
export TIX_TODAY=2026-11-01
work=$(mktemp -d)
export TIX_FILE="$work/tickets.tsv"
trap 'rm -rf "$work"' EXIT
failed=0

assert_eq() {
    if [[ "$1" == "$2" ]]; then
        echo "  ok    $3"
    else
        echo "  FAIL  $3"
        echo "        expected: $(printf '%s' "$2" | tr '\n' '|')"
        echo "        got:      $(printf '%s' "$1" | tr '\n' '|')"
        failed=$(( failed + 1 ))
    fi
}

tix() { bash ./tix "$@" 2>&1; }      # run the real program, errors included
status() { bash ./tix "$@" > /dev/null 2>&1; echo $?; }

echo "next_id"
source ./tixlib.sh
: > "$TIX_FILE"
assert_eq "$(next_id)" "1" "empty file"
printf '3\topen\tnormal\t-\t2026-10-01\ta\n9\tclosed\tlow\t-\t2026-10-02\tb\n4\topen\tlow\t-\t2026-10-03\tc\n' > "$TIX_FILE"
assert_eq "$(next_id)" "10" "after the highest, not the last"
: > "$TIX_FILE"

echo "add"
assert_eq "$(tix add Printer on 3F is offline)" "Created ticket #1" "a title from several words"
assert_eq "$(tix add -p urgent -a yuki VPN down for Osaka office)" "Created ticket #2" "with options"
assert_eq "$(tix add -p low "Order more toner")" "Created ticket #3" "a quoted title"
assert_eq "$(tix add -p high -a ken $'Email\tbounces')" "Created ticket #4" "a tab in the title"
assert_eq "$(sed -n 1p "$TIX_FILE")" $'1\topen\tnormal\t-\t2026-11-01\tPrinter on 3F is offline' "the stored line"
assert_eq "$(sed -n 4p "$TIX_FILE" | cut -f6)" "Email bounces" "tabs become spaces"
assert_eq "$(tix add -p whenever Coffee)" "tix: unknown priority: whenever" "bad priority"
assert_eq "$(tix add -p high)" "tix: a title is required" "no title"
assert_eq "$(status add -x thing)" "2" "unknown option exits 2"
assert_eq "$(wc -l < "$TIX_FILE" | tr -d ' ')" "4" "errors add nothing"

echo "list"
assert_eq "$(tix list)" "$(printf '%s\n' \
    '#2    urgent  yuki       VPN down for Osaka office' \
    '#4    high    ken        Email bounces' \
    '#1    normal  -          Printer on 3F is offline' \
    '#3    low     -          Order more toner')" "sorted by priority"
assert_eq "$(tix list -a ken)" "#4    high    ken        Email bounces" "by assignee"
assert_eq "$(tix list -p low)" "#3    low     -          Order more toner" "by priority"
assert_eq "$(tix list -a nobody)" "no tickets" "nothing matches"
tix add -p urgent Server room too hot > /dev/null
assert_eq "$(tix list | head -2 | cut -c1-12)" $'#2    urgent\n#5    urgent' "same priority: by id"

echo "assign, close, reopen"
assert_eq "$(tix assign 1 aiko)" "Ticket #1 assigned to aiko" "assign"
assert_eq "$(tix list -a aiko)" "#1    normal  aiko       Printer on 3F is offline" "assigned"
assert_eq "$(tix close 2)" "Closed ticket #2" "close"
assert_eq "$(tix close 2)" "tix: ticket #2 is already closed" "close twice"
assert_eq "$(tix list -c)" "#2    urgent  yuki       VPN down for Osaka office" "closed list"
assert_eq "$(tix list | grep -c '#2 ')" "0" "closed tickets leave the open list"
assert_eq "$(tix reopen 2)" "Reopened ticket #2" "reopen"
assert_eq "$(tix close 99)" "tix: no ticket #99" "unknown ticket"
assert_eq "$(tix close abc)" "tix: not a ticket number: abc" "not a number"
assert_eq "$(sed -n 3p "$TIX_FILE")" $'3\topen\tlow\t-\t2026-11-01\tOrder more toner' "other lines untouched"
assert_eq "$(ls "$work")" "tickets.tsv" "no temporary files left"

echo "show and report"
assert_eq "$(tix show 4)" "$(printf '%s\n' 'Ticket #4: Email bounces' '  status:   open' '  priority: high' '  assignee: ken' '  created:  2026-11-01')" "show"
tix close 3 > /dev/null
assert_eq "$(tix report)" "$(printf '%s\n' 'open: 4  closed: 1' 'open by priority: urgent=2 high=1 normal=1' 'open by assignee:' \
    '  -          1' '  aiko       1' '  ken        1' '  yuki       1')" "report"

echo "commands"
assert_eq "$(tix frobnicate | head -1)" "tix: unknown command: frobnicate" "unknown command"
assert_eq "$(status frobnicate)" "2" "unknown command exits 2"
assert_eq "$(tix help | head -1)" "usage: tix <command> [options]" "help"
assert_eq "$(status close 99)" "1" "errors exit 1"
TIX_FILE="$work/new/dir/t.tsv" bash ./tix list > /dev/null
assert_eq "$([[ -f "$work/new/dir/t.tsv" ]] && echo yes)" "yes" "creates the store"

if (( failed > 0 )); then echo "$failed test(s) failed"; exit 1; fi
echo "All tests passed"
@@file tix-completion.bash
# Tab completion for tix. Load it with:  source tix-completion.bash
# ("make install" copies it to where bash-completion finds it automatically.)
_tix() {
    local cur=${COMP_WORDS[COMP_CWORD]} prev=${COMP_WORDS[COMP_CWORD-1]}
    if (( COMP_CWORD == 1 )); then
        COMPREPLY=($(compgen -W "add list show assign close reopen report help" -- "$cur"))
        return
    fi
    case $prev in
        -p) COMPREPLY=($(compgen -W "urgent high normal low" -- "$cur")); return ;;
    esac
    case ${COMP_WORDS[1]} in
        show | assign | close | reopen)
            if (( COMP_CWORD == 2 )); then
                # offer the ticket numbers that exist
                local file=${TIX_FILE:-${XDG_DATA_HOME:-$HOME/.local/share}/tix/tickets.tsv}
                [[ -f $file ]] && COMPREPLY=($(compgen -W "$(cut -f1 "$file")" -- "$cur"))
            fi ;;
        add) COMPREPLY=($(compgen -W "-p -a" -- "$cur")) ;;
        list) COMPREPLY=($(compgen -W "-p -a -c" -- "$cur")) ;;
    esac
}
complete -F _tix tix
@@file Makefile
# make test                     run the tests
# make install                  install for yourself, into ~/.local
# sudo make install PREFIX=/usr/local   install for everyone
PREFIX ?= $(HOME)/.local
LIBDIR = $(PREFIX)/lib/tix
BINDIR = $(PREFIX)/bin
COMPDIR = $(PREFIX)/share/bash-completion/completions

.PHONY: test install uninstall lint

test:
	bash tests.sh

lint:
	shellcheck tix tixlib.sh tests.sh tix-completion.bash

install:
	install -d $(LIBDIR) $(BINDIR) $(COMPDIR)
	install -m 755 tix $(LIBDIR)/tix
	install -m 644 tixlib.sh $(LIBDIR)/tixlib.sh
	ln -sf $(LIBDIR)/tix $(BINDIR)/tix
	install -m 644 tix-completion.bash $(COMPDIR)/tix

uninstall:
	rm -rf $(LIBDIR) $(BINDIR)/tix $(COMPDIR)/tix
@@text
🎉 You've built a real command-line application: subcommands, option parsing, safe file updates, exit codes, tests, completion and an installer. It's the same structure as tools used by millions of people.

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

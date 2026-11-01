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

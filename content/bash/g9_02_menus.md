@@lesson g9-menus Interactive Menus: select, Key Handling and whiptail
@@concepts input, conditions, loops
@@text
Not every tool needs to be fully scriptable. Install wizards, on-call runbooks and server setup helpers are friendlier with a **menu**. Bash gives you three levels.

## 1. select: numbered menus for free

```bash,norun
PS3="Choose an action: "
select action in "Show disk usage" "Restart nginx" "Quit"; do
    case $REPLY in
        1) df -h ;;
        2) sudo systemctl restart nginx ;;
        3) break ;;
        *) echo "invalid choice: $REPLY" ;;
    esac
done
```

`select` prints the numbered list (to stderr), reads a line into `REPLY`, sets the variable to the chosen item (empty if invalid), and loops until `break` or end of input. It works over SSH, needs no packages and is easy to test by piping answers in.

## 2. read -n1: single keys

```bash,norun
read -rsn1 key                 # one key, silent
if [[ $key == $'\e' ]]; then   # arrow keys send ESC [ A/B/C/D
    read -rsn2 -t 0.01 rest
    key+=$rest
fi
case $key in
    $'\e[A') echo up ;;  $'\e[B') echo down ;;
    '') echo enter ;;    q) echo quit ;;
esac
```

With a list in an array, a selected index, and a redraw after every key, you have a full arrow-key menu, the basis of the capstone project.

Remember to restore the terminal: `stty -echo` / `stty echo`, cursor visibility and the alternate screen, all in an `EXIT` trap.

## 3. whiptail and dialog: ready-made widgets

`whiptail` (preinstalled on Debian/Ubuntu, used by `raspi-config`) and `dialog` draw real dialog boxes:

```bash,norun
whiptail --title "Setup" --msgbox "Welcome!" 8 40
name=$(whiptail --inputbox "Server name:" 8 40 web01 3>&1 1>&2 2>&3) || exit 1
choice=$(whiptail --menu "Pick one" 15 50 4 \
    nginx "Web server" postgres "Database" redis "Cache" 3>&1 1>&2 2>&3)
whiptail --yesno "Apply changes?" 8 40 && apply
whiptail --checklist "Packages" 15 50 5 git "" ON curl "" ON htop "" OFF 3>&1 1>&2 2>&3
{ for i in 20 40 60 80 100; do echo $i; sleep 1; done; } | whiptail --gauge "Installing..." 6 50 0
```

whiptail draws on stdout and writes the **answer to stderr**, so `3>&1 1>&2 2>&3` swaps them to capture the answer with `$( )`. The exit status tells you whether the user pressed OK (0) or Cancel/Esc (1/255).

## Also worth knowing

- **fzf**: fuzzy finder; `choice=$(printf '%s\n' "${hosts[@]}" | fzf)` gives any list instant search.
- **gum** (Charm): modern prompts, spinners and tables for shell scripts.
- Always provide a **non-interactive path** too (flags or environment variables), so the tool works in CI and cron: `[[ -t 0 ]] || die "no terminal: use --yes"`.

@@quiz
What does `select` store in `REPLY`?
* The raw text the user typed (e.g. the number)
- The chosen item's text
- The exit status
- The number of items
? The chosen item goes into the loop variable; it's empty for invalid input.

@@quiz
Why does capturing whiptail's answer need `3>&1 1>&2 2>&3`?
- whiptail requires three descriptors
* whiptail draws on stdout and writes the answer to stderr, so the descriptors are swapped to capture the answer with `$( )`
- To hide errors
- For colours
? fd 3 is a temporary holder during the swap.

@@quiz
What does pressing the Up arrow send to a program reading the terminal?
- The character `^`
* An escape sequence: ESC `[` `A`
- Nothing
- The number 38
? Read one key, and if it's ESC, read the rest.

@@flashcards
`select x in a b c` :: numbered menu
`PS3` :: select's prompt
`REPLY` :: raw input in select/read
`read -rsn1` :: one silent key
`$'\e[A'` / `$'\e[B'` :: up / down arrow
`stty -echo` :: stop echoing keys
whiptail :: dialog boxes (Debian/Ubuntu)
`3>&1 1>&2 2>&3` :: swap stdout/stderr
`--menu` / `--yesno` / `--gauge` :: whiptail widgets
fzf :: fuzzy picker
`[[ -t 0 ]]` :: stdin is a terminal

@@exercise A select-driven runbook
Write an on-call menu with `select` over the options `Check disk`, `Tail errors`, `Restart service`, `Quit`:

- `PS3="runbook> "`; print a heading per action
- `Restart service` asks for a service name with `read -rp` and validates it against an allow-list (`nginx`, `redis`)
- invalid numbers print `invalid choice: N`
- the loop also ends at end of input, and the script prints how many actions ran

The answers come from stdin (`@@stdin`), as if typed. `select` and `read -p` write their prompts to stderr, so only your output appears below.
@@stdin
1
5
3
postgres
3
redis
2
4
@@starter
disk='/ 82%
/var 93%'
errors='09:12 ERROR db timeout
09:15 ERROR cache miss storm'
echo TODO
@@expected
== disk
/var 93% (over 90%)
invalid choice: 5
== restart
refused: postgres is not in the allow-list
== restart
restarting redis... done
== errors
09:15 ERROR cache miss storm
bye after 4 actions
@@requires
select
PS3
REPLY
read -rp
@@hint
`case $REPLY in 1) ...;; 2) ...;; 3) ...;; 4) break;; *) echo "invalid choice: $REPLY"; continue;; esac` then `((count++))`. For the disk check, use awk on `$disk` to print mounts above 90%. "Tail errors" prints the last line with `tail -n 1`.
@@solution
disk='/ 82%
/var 93%'
errors='09:12 ERROR db timeout
09:15 ERROR cache miss storm'

PS3="runbook> "
count=0
select action in "Check disk" "Tail errors" "Restart service" "Quit"; do
    case $REPLY in
        1)
            echo "== disk"
            awk '{ p = $2; sub(/%/, "", p); if (p + 0 > 90) print $1, $2, "(over 90%)" }' <<< "$disk"
            ;;
        2)
            echo "== errors"
            tail -n 1 <<< "$errors"
            ;;
        3)
            echo "== restart"
            read -rp "service: " svc
            case $svc in
                nginx|redis) echo "restarting $svc... done" ;;
                *) echo "refused: $svc is not in the allow-list" ;;
            esac
            ;;
        4) break ;;
        *) echo "invalid choice: $REPLY"; continue ;;
    esac
    ((count++))
done
echo "bye after $count actions"

@@exercise Decode key presses
A full-screen menu reads raw keys. Write `read_key` that reads one key from stdin and sets `KEY` to `up`, `down`, `enter`, `quit`, `space` or `other:<char>`:

- read one character with `read -rsn1`; an empty result means Enter (newline)
- on ESC, read up to two more characters (`read -rsn2 -t 0.1`) and decode `[A` / `[B`
- `k`/`j` are vi-style up/down; `q` quits

Then drive a list cursor with a stream of keys: move within bounds, toggle selection with space, stop at Enter or q, and print the selected items.
@@starter
items=(nginx postgres redis grafana)
keys=$'j j \e[A\e[B\e[B j j x \n'
echo TODO
@@expected
down -> 1
space: postgres selected
down -> 2
space: redis selected
up -> 1
down -> 2
down -> 3
space: grafana selected
down -> 3
space: grafana unselected
down -> 3
space: grafana selected
other: x
space: grafana unselected
enter
selected: postgres redis
@@requires
read -rsn1
$'\e'
@@hint
Feed the keys with `done < <(printf '%s' "$keys")` or `exec 3< <(...)` and read with `-u 3`. Note the spaces in `keys` are key presses too (space toggles). Clamp the cursor: `(( cursor < ${#items[@]} - 1 )) && ((cursor++))`. Keep selections in an associative array keyed by index, and print the selected items in list order at the end.
@@solution
items=(nginx postgres redis grafana)
keys=$'j j \e[A\e[B\e[B j j x \n'

read_key() {
    local c rest
    IFS= read -rsn1 -u 3 c || { KEY=quit; return; }
    if [[ $c == $'\e' ]]; then
        IFS= read -rsn2 -t 0.1 -u 3 rest
        case $rest in
            '[A') KEY=up ;;
            '[B') KEY=down ;;
            *) KEY=other:esc ;;
        esac
        return
    fi
    case $c in
        '') KEY=enter ;;
        ' ') KEY=space ;;
        k) KEY=up ;;
        j) KEY=down ;;
        q) KEY=quit ;;
        *) KEY="other:$c" ;;
    esac
}

exec 3< <(printf '%s' "$keys")
cursor=0
declare -A selected=()
while true; do
    read_key
    case $KEY in
        up) (( cursor > 0 )) && ((cursor--)); echo "up -> $cursor" ;;
        down) (( cursor < ${#items[@]} - 1 )) && ((cursor++)); echo "down -> $cursor" ;;
        space)
            if [[ -n ${selected[$cursor]:-} ]]; then
                unset 'selected[$cursor]'
                echo "space: ${items[cursor]} unselected"
            else
                selected[$cursor]=1
                echo "space: ${items[cursor]} selected"
            fi
            ;;
        enter|quit) echo "$KEY"; break ;;
        other:*) echo "other: ${KEY#other:}" ;;
    esac
done
exec 3<&-

chosen=()
for i in "${!items[@]}"; do
    [[ -n ${selected[$i]:-} ]] && chosen+=("${items[i]}")
done
echo "selected: ${chosen[*]:-none}"

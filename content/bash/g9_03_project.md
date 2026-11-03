@@lesson g9-project Project: A Full-Screen Service Dashboard
@@concepts loops, strings, automation
@@text
The capstone: `svcdash`, a full-screen terminal app in the style of `htop` or `lazydocker`. It lists services with their state, lets you move with the arrow keys or `j`/`k`, filter with `/`, and restart the selected service with `r`, redrawing the screen after every key.

## Architecture: state → render → input

Every terminal UI, in any language, has the same loop:

```bash,norun
setup_terminal          # alternate screen, hide cursor, stty -echo, traps
while true; do
    render              # draw the whole screen from the current state
    read_key            # wait for one key (with a timeout for auto-refresh)
    handle "$KEY"       # change the state; never draw here
done
```

Keeping drawing and state changes separate makes the app testable: `render` is a pure function of the state, so tests can call it with any state and compare text, and the event handler can be tested with a list of keys, exactly what the exercises do.

## Terminal setup and cleanup

```bash,norun
setup_terminal() {
    saved_stty=$(stty -g)
    printf '\e[?1049h\e[?25l'          # alternate screen, hide cursor
    stty -echo -icanon                 # keys arrive immediately, not echoed
    trap restore_terminal EXIT
    trap 'exit 130' INT TERM
    trap 'resized=1' WINCH             # terminal resized: re-read size
}
restore_terminal() {
    stty "$saved_stty"
    printf '\e[?25h\e[?1049l'
}
```

## Rendering without flicker

- Build the whole frame into one string (or array) and print it **once**, starting with `\e[H` (cursor home) instead of clearing the screen. Clearing first makes the screen flash.
- Pad every line to the full width, so leftovers from the previous frame are overwritten.
- Keep the selected row visible: if the list is longer than the screen, maintain a **scroll offset**: when the cursor goes below `offset + height - 1`, increase the offset; when it goes above `offset`, decrease it.

## Auto-refresh

`read -rsn1 -t 2` waits up to two seconds for a key. On timeout (status > 128), refresh the data and redraw anyway. That gives a live dashboard without a second process.

## Real data

In the real tool, `refresh` would run:

```bash,norun
systemctl list-units --type=service --all --no-legend --plain |
    awk '{ print $1, $3, $4 }'          # unit, active, sub-state
```

and `r` would call `sudo systemctl restart "$unit"`. The exercises use an in-memory list so they run anywhere.

@@quiz
Why should the event handler change state but never draw?
* Rendering stays a pure function of the state, so it can be tested and the screen is always consistent
- Drawing in handlers is slower
- Bash can't print inside functions
- To save memory
? State → render → input → state...

@@quiz
How do you avoid flicker when redrawing?
- Clear the screen before each frame
* Move the cursor home and overwrite with full-width lines in a single write
- Use more colours
- Redraw less often
? Clearing shows a blank screen for a moment.

@@quiz
What does `trap 'resized=1' WINCH` handle?
- Mouse clicks
* The terminal window being resized, so the next render can use the new size
- Window focus
- Ctrl+W
? SIGWINCH = window change.

@@flashcards
state → render → input :: the TUI loop
`\e[H` :: cursor home (redraw without clearing)
scroll offset :: first visible row
`stty -g` :: save terminal settings
`stty -echo -icanon` :: raw-ish key input
SIGWINCH :: terminal resized
`read -t 2` :: input with timeout (auto-refresh)
full-width lines :: overwrite the previous frame

@@exercise Render a frame
Write `render WIDTH HEIGHT` that prints the dashboard from the global state, every line exactly WIDTH characters (pad with spaces; show the right edge as `|` so the tests can see it):

- line 1: ` svcdash` on the left and `N/M shown` on the right
- a list area of `HEIGHT - 3` rows showing services from `offset`: `>` marks the cursor row, then name (12 wide) and state; empty rows if the list is shorter
- a status line: the last message, or `filter: TEXT` if a filter is set
- a footer: ` j/k move  r restart  / filter  q quit`

Write `ensure_visible` to adjust `offset` so the cursor row is on screen, and render three states.
@@starter
names=(api-gateway billing cron postgres redis search worker)
states=(active active inactive active failed active active)
cursor=0 offset=0 message="" filter=""
echo TODO
@@expected
| svcdash                     7/7 shown|
|> api-gateway  active                 |
|  billing      active                 |
|  cron         inactive               |
|  postgres     active                 |
|                                      |
| j/k move  r restart  / filter  q quit|
---
| svcdash                     7/7 shown|
|  billing      active                 |
|  cron         inactive               |
|  postgres     active                 |
|> redis        failed                 |
|redis needs attention                 |
| j/k move  r restart  / filter  q quit|
---
| svcdash                     7/7 shown|
|  postgres     active                 |
|  redis        failed                 |
|  search       active                 |
|> worker       active                 |
|                                      |
| j/k move  r restart  / filter  q quit|
@@requires
ensure_visible
printf
offset
@@hint
`rows=$(( height - 3 ))`; in `ensure_visible`: `(( cursor < offset )) && offset=$cursor; (( cursor >= offset + rows )) && offset=$(( cursor - rows + 1 ))`. Pad a line with `printf '|%-*s|\n' "$width" "$text"`. For the header, compute the gap: `width - ${#left} - ${#right}` spaces.
@@solution
names=(api-gateway billing cron postgres redis search worker)
states=(active active inactive active failed active active)
cursor=0 offset=0 message="" filter=""

line() { printf '|%-*s|\n' "$1" "$2"; }

ensure_visible() {
    local rows=$1
    (( cursor < offset )) && offset=$cursor
    (( cursor >= offset + rows )) && offset=$(( cursor - rows + 1 ))
    return 0
}

render() {
    local width=$1 height=$2 rows i text left=" svcdash" right
    rows=$(( height - 3 ))
    ensure_visible "$rows"
    right="${#names[@]}/${#names[@]} shown"
    printf -v text '%s%*s' "$left" $(( width - ${#left} )) "$right"
    line "$width" "$text"
    for (( i = offset; i < offset + rows; i++ )); do
        if (( i < ${#names[@]} )); then
            printf -v text '%s %-12s %s' "$( (( i == cursor )) && echo '>' || echo ' ')" "${names[i]}" "${states[i]}"
        else
            text=""
        fi
        line "$width" "$text"
    done
    if [[ -n $filter ]]; then line "$width" "filter: $filter"; else line "$width" "$message"; fi
    line "$width" " j/k move  r restart  / filter  q quit"
}

render 38 7
echo ---
cursor=4 message="redis needs attention"
render 38 7
echo ---
cursor=6 message=""
render 38 7

@@exercise The event loop
Add input handling on top of a renderer (the starter includes a compact one). `handle KEY` changes the state only:

- `j`/`down` and `k`/`up` move within the **visible** (filtered) list
- `r` restarts the selected service: its state becomes `active` and the message `restarted NAME`
- `/` starts filter mode: following characters are appended to the filter, Enter ends filter mode, and the cursor resets to the first match; `esc` clears the filter
- `q` sets `running=0`

Feed the key list, rendering after each `render` pseudo-key, and stop at `q`.
@@starter
names=(api-gateway billing cron postgres redis search worker)
states=(active active inactive active failed active active)
cursor=0 filter="" filtering=0 message="" running=1
visible() { local i; vis=(); for i in "${!names[@]}"; do [[ ${names[i]} == *"$filter"* ]] && vis+=("$i"); done; }
render() {
    visible
    printf '[%d/%d] %s\n' "${#vis[@]}" "${#names[@]}" "${filter:+filter=$filter }$message"
    local n
    for n in "${!vis[@]}"; do
        printf '  %s %-12s %s\n' "$( (( n == cursor )) && echo '>' || echo ' ')" "${names[vis[n]]}" "${states[vis[n]]}"
    done
}
keys=(j j j j render r render / r e enter render j render esc render q render)
echo TODO
@@expected
[7/7] 
    api-gateway  active
    billing      active
    cron         inactive
    postgres     active
  > redis        failed
    search       active
    worker       active
[7/7] restarted redis
    api-gateway  active
    billing      active
    cron         inactive
    postgres     active
  > redis        active
    search       active
    worker       active
[2/7] filter=re restarted redis
  > postgres     active
    redis        active
[2/7] filter=re restarted redis
    postgres     active
  > redis        active
[7/7] restarted redis
    api-gateway  active
  > billing      active
    cron         inactive
    postgres     active
    redis        active
    search       active
    worker       active
@@requires
handle()
case
@@hint
In `handle`, check filter mode first: `if (( filtering )); then case $1 in enter) filtering=0 ;; *) filter+=$1; cursor=0 ;; esac; return; fi`. Moving: call `visible` and clamp against `${#vis[@]}`. Restart: `local idx=${vis[cursor]}; states[idx]=active`. Note that the cursor is an index into the *visible* list, so after clearing the filter it stays at the same position number (1), which now points at `billing`.
@@solution
names=(api-gateway billing cron postgres redis search worker)
states=(active active inactive active failed active active)
cursor=0 filter="" filtering=0 message="" running=1
visible() { local i; vis=(); for i in "${!names[@]}"; do [[ ${names[i]} == *"$filter"* ]] && vis+=("$i"); done; }
render() {
    visible
    printf '[%d/%d] %s\n' "${#vis[@]}" "${#names[@]}" "${filter:+filter=$filter }$message"
    local n
    for n in "${!vis[@]}"; do
        printf '  %s %-12s %s\n' "$( (( n == cursor )) && echo '>' || echo ' ')" "${names[vis[n]]}" "${states[vis[n]]}"
    done
}
keys=(j j j j render r render / r e enter render j render esc render q render)

handle() {
    local key=$1 idx
    if (( filtering )); then
        case $key in
            enter) filtering=0 ;;
            *) filter+=$key; cursor=0 ;;
        esac
        return
    fi
    visible
    case $key in
        j|down) (( cursor < ${#vis[@]} - 1 )) && ((cursor++)) ;;
        k|up) (( cursor > 0 )) && ((cursor--)) ;;
        r)
            idx=${vis[cursor]}
            states[idx]=active
            message="restarted ${names[idx]}"
            ;;
        /) filtering=1 filter="" ;;
        esc) filter="" ;;
        q) running=0 ;;
    esac
    return 0
}

for key in "${keys[@]}"; do
    (( running )) || break
    if [[ $key == render ]]; then
        render
    else
        handle "$key"
    fi
done

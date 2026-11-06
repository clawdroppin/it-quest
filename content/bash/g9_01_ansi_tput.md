@@lesson g9-ansi Colours, Cursor Control and tput
@@concepts strings, text-processing
@@text
Terminals understand **escape sequences**: byte strings starting with ESC (`\e`, `\033`, `\x1b`) that change colour, move the cursor or clear the screen. They power every coloured log line, progress bar and full-screen terminal app.

## SGR: colours and styles

`\e[<codes>m` sets the style; `\e[0m` resets it.

| Code | Effect | Code | Effect |
|---|---|---|---|
| 0 | reset | 30–37 | foreground: black, red, green, yellow, blue, magenta, cyan, white |
| 1 | bold | 90–97 | bright foreground |
| 2 | dim | 40–47 | background |
| 3 | italic | `38;5;N` | 256-colour foreground |
| 4 | underline | `38;2;R;G;B` | true colour (24-bit) |
| 7 | reverse | `48;...` | same for background |

```bash,norun
printf '\e[1;31mERROR\e[0m disk full\n'
printf '\e[38;2;255;140;0morange\e[0m\n'
```

## Cursor and screen

```bash,norun
printf '\e[2J\e[H'     # clear screen, cursor home
printf '\e[5;10H'      # move to row 5, column 10 (1-based)
printf '\e[2K\r'       # clear the line, back to column 1 (progress bars)
printf '\e[?25l'       # hide cursor   (\e[?25h shows it again)
printf '\e[?1049h'     # alternate screen (like vim/less); \e[?1049l to leave
```

## tput: the portable way

Different terminals use different sequences. `tput` looks them up in the **terminfo** database:

```bash,norun
red=$(tput setaf 1) bold=$(tput bold) reset=$(tput sgr0)
cols=$(tput cols) lines=$(tput lines)
tput cup 5 10; tput civis; tput cnorm; tput smcup; tput rmcup
```

## Be a good citizen

- Only colour when output goes to a **terminal**: `[[ -t 1 ]]` (stdout is a TTY). Logs and pipes should get plain text.
- Respect the [`NO_COLOR`](https://no-color.org) environment variable, and offer `--color=auto|always|never`.
- Always reset styles, and restore the cursor and screen in an `EXIT` trap, or a Ctrl+C leaves the user's terminal hidden or coloured.
- Colour should add meaning, not carry it alone: keep the words `ERROR`/`OK` for colour-blind users and plain logs.

## Seeing escape codes

`cat -v` shows control characters: ESC appears as `^[`, so `\e[31m` displays as `^[[31m`. That's how the exercises check your output.

## Width of text

`${#s}` counts characters, but escape sequences add invisible characters, and East Asian characters (日本) take **two** columns. When padding coloured text, pad the plain text first and colour afterwards.

@@quiz
What does `\e[0m` do?
* Resets all colours and styles
- Moves the cursor to the top
- Clears the screen
- Makes text bold
? Always end coloured output with it.

@@quiz
How should a script decide whether to print colours?
- Always print them
* Only when stdout is a terminal (`[[ -t 1 ]]`) and NO_COLOR isn't set, unless the user forces it
- Only for root
- Only on Linux
? Pipes and log files should get plain text.

@@quiz
Why pad text before adding colour codes?
- Colour codes are slow
* The escape sequences count as characters for `printf` widths but take no space on screen, which breaks alignment
- printf can't print escapes
- tput requires it
? `printf '%-8s'` would count the invisible bytes.

@@drill
Which tput capability resets all colours and styles?
= sgr0
= tput sgr0
? Use it after coloured output.

@@flashcards
`\e[...m` :: SGR: set style
`\e[0m` :: reset
`31` / `32` / `33` :: red / green / yellow
`1` / `4` / `7` :: bold / underline / reverse
`38;5;N` :: 256-colour foreground
`\e[2J\e[H` :: clear screen and home
`\e[row;colH` :: move the cursor
`\e[2K` :: clear the line
`\e[?25l` / `h` :: hide / show cursor
`\e[?1049h` :: alternate screen
`tput setaf 1` :: portable red
`[[ -t 1 ]]` :: stdout is a terminal
`NO_COLOR` :: user opts out of colour

@@exercise Coloured log levels with a --color switch
Write `log LEVEL MESSAGE` that prints aligned, optionally coloured lines:

- the level padded to 5 characters **before** colouring; ERROR red+bold (`1;31`), WARN yellow (`33`), INFO green (`32`), DEBUG dim (`2`)
- `color_mode` is `auto`, `always` or `never`; `auto` colours only if stdout is a terminal and `NO_COLOR` is unset
- show `always` (through `cat -v` so the escapes are visible), `never`, and `auto` (stdout is a pipe here, so no colour)
@@starter
echo TODO
@@expected
== always
^[[1;31mERROR^[[0m disk /var at 93%
^[[33mWARN ^[[0m cert expires in 12 days
^[[32mINFO ^[[0m backup finished
== never
ERROR disk /var at 93%
WARN  cert expires in 12 days
== auto
INFO  piped output stays plain
@@requires
-t 1
NO_COLOR
cat -v
@@hint
`use_color() { case $color_mode in always) return 0 ;; never) return 1 ;; esac; [[ -t 1 && -z ${NO_COLOR:-} ]]; }`. Pad with `printf -v padded '%-5s' "$1"`, then wrap: `printf '\e[%sm%s\e[0m %s\n' "$code" "$padded" "$2"`. The `auto` case runs inside `$( )` or a pipe, where stdout isn't a terminal.
@@solution
color_mode=auto

use_color() {
    case $color_mode in
        always) return 0 ;;
        never) return 1 ;;
    esac
    [[ -t 1 && -z ${NO_COLOR:-} ]]
}

log() {
    local padded code
    printf -v padded '%-5s' "$1"
    case $1 in
        ERROR) code='1;31' ;;
        WARN) code=33 ;;
        INFO) code=32 ;;
        *) code=2 ;;
    esac
    if use_color; then
        printf '\e[%sm%s\e[0m %s\n' "$code" "$padded" "$2"
    else
        printf '%s %s\n' "$padded" "$2"
    fi
}

echo "== always"
color_mode=always
{
    log ERROR "disk /var at 93%"
    log WARN "cert expires in 12 days"
    log INFO "backup finished"
} | cat -v
echo "== never"
color_mode=never
log ERROR "disk /var at 93%"
log WARN "cert expires in 12 days"
echo "== auto"
color_mode=auto
log INFO "piped output stays plain" | cat

@@exercise Boxes, bars and a sparkline
Build three drawing helpers used by terminal dashboards (plain Unicode, no escapes):

- `box TITLE LINE...`: draws a box with `┌─┐│└┘` around the lines, as wide as the longest line or title + 2, with the title in the top border
- `bar LABEL VALUE MAX WIDTH`: `label  [██████░░░░] 60%` using `█` and `░`
- `spark NUMBERS...`: maps each value to one of `▁▂▃▄▅▆▇█` relative to the min and max
@@starter
echo TODO
@@expected
┌─ web01 ─────────┐
│ uptime: 41 days │
│ load: 0.42      │
│ kernel: 6.17.2  │
└─────────────────┘
cpu   [██████░░░░] 60%
mem   [█████████░] 85%
disk  [██░░░░░░░░] 23%
requests/h: ▁▂▄▇█▅▃▁
@@requires
┌
printf
@@hint
Repeat a character N times with `printf -v line '%*s' "$n" ''; line=${line// /─}`. Box width = the longest line + 2 (one space each side). Top border: `┌─ $title ` followed by enough `─` to reach the width, then `┐`. Bar: `filled=$(( (value * width + max / 2) / max ))` (rounded). Sparkline index: `(( (v - min) * 7 / (max - min) ))`. In Git Bash, set `LC_ALL=C.UTF-8` so `${#s}` counts characters, not bytes.
@@solution
export LC_ALL=C.UTF-8

repeat() { local out; printf -v out '%*s' "$2" ''; printf '%s' "${out// /$1}"; }

box() {
    local title=$1 line width=0
    shift
    for line in "$@"; do (( ${#line} > width )) && width=${#line}; done
    (( ${#title} + 2 > width )) && width=$(( ${#title} + 2 ))
    width=$(( width + 2 ))
    printf '┌─ %s %s┐\n' "$title" "$(repeat ─ $(( width - ${#title} - 3 )))"
    for line in "$@"; do
        printf '│ %s%*s │\n' "$line" $(( width - 2 - ${#line} )) ''
    done
    printf '└%s┘\n' "$(repeat ─ "$width")"
}

bar() {
    local label=$1 value=$2 max=$3 width=$4 filled
    filled=$(( (value * width + max / 2) / max ))
    printf '%-5s [%s%s] %d%%\n' "$label" "$(repeat █ "$filled")" "$(repeat ░ $(( width - filled )))" $(( value * 100 / max ))
}

spark() {
    local ticks=(▁ ▂ ▃ ▄ ▅ ▆ ▇ █) min=$1 max=$1 v out=""
    for v in "$@"; do
        (( v < min )) && min=$v
        (( v > max )) && max=$v
    done
    for v in "$@"; do
        out+=${ticks[ (max == min) ? 0 : (v - min) * 7 / (max - min) ]}
    done
    printf '%s\n' "$out"
}

box web01 "uptime: 41 days" "load: 0.42" "kernel: 6.17.2"
bar cpu 60 100 10
bar mem 85 100 10
bar disk 23 100 10
printf 'requests/h: '
spark 120 300 610 1020 1100 800 450 130

@@lesson b10-interpreters 🎓 Beyond IT: Writing Interpreters in Bash
@@concepts strings, lists, functions
@@text
> **Beyond IT scope.** Building language interpreters is computer science and a fun puzzle, not a day-to-day IT skill. Interpreters for small "esoteric" languages are a classic exercise because they fit in one screen.

Every interpreter has the same parts:

1. **Tokenise**: split the source into meaningful pieces (numbers, words, symbols).
2. **Parse** (optional for simple languages): build a structure, like matching brackets.
3. **Evaluate**: walk the tokens or structure and update the machine's state.

## Stack machines and RPN

Reverse Polish Notation (RPN) puts operators after operands: `3 4 + 2 *` means `(3 + 4) × 2`. It needs no parentheses and no precedence rules: push numbers on a stack; an operator pops two values and pushes the result. HP calculators, PostScript, Forth and the JVM all work this way, and so does the `dc` command (Unix's oldest surviving program, older than C).

Bash arrays make good stacks:

```bash
stack=()
push() { stack+=("$1"); }
pop()  { POPPED=${stack[-1]}; unset 'stack[-1]'; }
push 3; push 4; pop; b=$POPPED; pop; a=$POPPED
push $(( a + b )); echo "${stack[*]}"
```

## Brainf*ck: a Turing-complete language with 8 commands

Its machine has a tape of 30,000 byte cells and a data pointer:

| Command | Meaning |
|---|---|
| `>` `<` | move the pointer right / left |
| `+` `-` | increment / decrement the current cell (wrapping at 256) |
| `.` | output the current cell as a character |
| `,` | read one character into the current cell |
| `[` | if the cell is 0, jump past the matching `]` |
| `]` | if the cell is not 0, jump back to the matching `[` |

The only tricky part is bracket matching. Pre-compute it once with a stack: on `[` push the position; on `]` pop the partner and record both directions in a `jump` array. Then evaluation is a simple loop with `case`.

Printing a byte as a character in Bash: `printf "\\$(printf '%03o' "$code")"` converts the number to an octal escape, or collect the characters with `printf -v ch "\\x$(printf '%x' "$code")"`.

## Where to go from here

- *Crafting Interpreters* by Robert Nystrom (free online) builds a full language twice: a tree-walking interpreter and a bytecode VM.
- Write a Forth: an RPN interpreter plus a dictionary of user-defined words (`: square dup * ;`) is a few dozen more lines.
- Brainf*ck compilers that translate to C (`+` → `++*p;`) run orders of magnitude faster than any interpreter.

@@quiz
What does the RPN expression `5 1 2 + 4 * + 3 -` evaluate to?
- 3
* 14
- 18
- 9
? 1+2 = 3; 3×4 = 12; 5+12 = 17; 17−3 = 14.

@@quiz
Why pre-compute matching brackets before running a Brainf*ck program?
* So each `[` or `]` can jump directly instead of scanning the program every time
- Bash can't scan strings
- The language requires it
- To print errors
? A stack pass makes jumps O(1).

@@quiz
In a stack machine, what does the `+` instruction do?
* Pops the top two values, adds them and pushes the result
- Pushes the number 1
- Adds every value on the stack
- Moves to the next instruction
? RPN works because every operator takes its inputs from the stack.

@@flashcards
tokenise / parse / evaluate :: interpreter stages
RPN :: operators after operands
stack machine :: push operands, pop for operators
`dc` :: Unix RPN calculator
Forth :: stack-based language with user-defined words
Brainf*ck :: 8-command Turing-complete language
jump table :: pre-matched bracket positions
`printf '%03o'` :: number to octal escape

@@exercise An RPN calculator
Write `rpn EXPRESSION` that evaluates integer RPN with:

- numbers (including negative ones like `-3`), `+ - * / %`, and stack words `dup`, `swap`, `drop`
- errors: `stack underflow at TOKEN`, `division by zero`, `unknown token TOKEN`, and `stack has N values left` if the final stack doesn't hold exactly one value
- print `EXPR = RESULT` or `EXPR: error: ...`
@@starter
echo TODO
@@expected
3 4 + 2 * = 14
5 1 2 + 4 * + 3 - = 14
2 dup * dup * = 16
10 3 swap - = -7
-3 4 * = -12
1 + : error: stack underflow at +
4 0 / : error: division by zero
2 3 pow : error: unknown token pow
1 2 3 + : error: stack has 2 values left
@@requires
stack
case
@@hint
Use `read -ra tokens <<< "$1"`. For binary operators, check `${#stack[@]} >= 2`, pop `b` then `a`, and push `$(( a op b ))`. Recognise numbers with `[[ $t =~ ^-?[0-9]+$ ]]` before the operator `case`, so `-3` isn't read as subtraction. Return errors from a helper with `return 1` and print them in the caller.
@@solution
rpn() {
    local -a stack=() tokens
    local t a b
    read -ra tokens <<< "$1"
    for t in "${tokens[@]}"; do
        if [[ $t =~ ^-?[0-9]+$ ]]; then
            stack+=("$t")
            continue
        fi
        case $t in
            +|-|'*'|/|%|swap)
                (( ${#stack[@]} >= 2 )) || { echo "error: stack underflow at $t"; return 1; }
                b=${stack[-1]}; unset 'stack[-1]'
                a=${stack[-1]}; unset 'stack[-1]'
                case $t in
                    +) stack+=($(( a + b ))) ;;
                    -) stack+=($(( a - b ))) ;;
                    '*') stack+=($(( a * b ))) ;;
                    /|%)
                        (( b != 0 )) || { echo "error: division by zero"; return 1; }
                        if [[ $t == / ]]; then stack+=($(( a / b ))); else stack+=($(( a % b ))); fi
                        ;;
                    swap) stack+=("$b" "$a") ;;
                esac
                ;;
            dup|drop)
                (( ${#stack[@]} >= 1 )) || { echo "error: stack underflow at $t"; return 1; }
                if [[ $t == dup ]]; then stack+=("${stack[-1]}"); else unset 'stack[-1]'; fi
                ;;
            *) echo "error: unknown token $t"; return 1 ;;
        esac
    done
    (( ${#stack[@]} == 1 )) || { echo "error: stack has ${#stack[@]} values left"; return 1; }
    echo "${stack[0]}"
}

for expr in "3 4 + 2 *" "5 1 2 + 4 * + 3 -" "2 dup * dup *" "10 3 swap -" "-3 4 *" \
            "1 +" "4 0 /" "2 3 pow" "1 2 3 +"; do
    if result=$(rpn "$expr"); then
        echo "$expr = $result"
    else
        echo "$expr : $result"
    fi
done

@@exercise A Brainf*ck interpreter
Write `bf PROGRAM [INPUT]`:

- strip every character that isn't one of the 8 commands
- pre-compute the `jump` table with a stack; report `unmatched [` / `unmatched ]` errors
- run with a tape of cells that wrap at 256, collecting output characters in a string
- `,` reads the next character of INPUT (0 at the end)
- count executed instructions

Run a "Hello" program, an echo-uppercase program and a broken one.
@@starter
hello='++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.'
upper=',[>++++[<-------->-]<.,]'
echo TODO
@@expected
Hello (steps: 870)
BASH (steps: 234)
error: unmatched [
@@requires
jump
printf
@@hint
Clean the program with `${prog//[^][<>+.,-]/}`. Convert the current cell to a character with `printf -v ch "\\$(printf '%03o' "${tape[p]}")"`; read input with `printf -v code '%d' "'$c"` (a leading quote gives the character code). The uppercase program subtracts 32 from each character and stops at end of input, so feed it `bash`.
@@solution
hello='++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.'
upper=',[>++++[<-------->-]<.,]'

bf() {
    local prog=${1//[^][<>+.,-]/} input=${2:-}
    local -a jump=() open=() tape=()
    local i c n=${#prog}
    for (( i = 0; i < n; i++ )); do
        c=${prog:i:1}
        if [[ $c == '[' ]]; then
            open+=("$i")
        elif [[ $c == ']' ]]; then
            (( ${#open[@]} )) || { echo "error: unmatched ]"; return 1; }
            jump[i]=${open[-1]}
            jump[${open[-1]}]=$i
            unset 'open[-1]'
        fi
    done
    (( ${#open[@]} == 0 )) || { echo "error: unmatched ["; return 1; }

    local pc=0 p=0 in=0 steps=0 out="" ch code
    while (( pc < n )); do
        c=${prog:pc:1}
        ((steps++))
        case $c in
            '>') ((p++)) ;;
            '<') ((p--)) ;;
            '+') tape[p]=$(( (${tape[p]:-0} + 1) % 256 )) ;;
            '-') tape[p]=$(( (${tape[p]:-0} + 255) % 256 )) ;;
            '.') printf -v ch "\\$(printf '%03o' "${tape[p]:-0}")"; out+=$ch ;;
            ',')
                if (( in < ${#input} )); then
                    printf -v code '%d' "'${input:in:1}"
                    tape[p]=$code
                    ((in++))
                else
                    tape[p]=0
                fi
                ;;
            '[') (( ${tape[p]:-0} == 0 )) && pc=${jump[pc]} ;;
            ']') (( ${tape[p]:-0} != 0 )) && pc=${jump[pc]} ;;
        esac
        ((pc++))
    done
    echo "$out (steps: $steps)"
}

bf "$hello"
bf "$upper" bash
bf '+[>+' || true

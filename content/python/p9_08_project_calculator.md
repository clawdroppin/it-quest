@@lesson p9-calculator 🏆 Module Project: A Formula Engine
@@concepts objects, text-processing, errors
@@text
This project uses everything from the module: classes with special methods, enums, pattern matching, composition and closures. You'll build the core of a **formula engine**, the kind of thing behind spreadsheet cells, alert rules (`cpu > 90 and disk > 80`), pricing rules or a monitoring tool's threshold expressions.

## How interpreters work

Every language, from Python itself to SQL to Excel formulas, is processed in the same stages:

```text
"2 * (load + 3)"
     │  1. tokenizer (lexer): text → tokens
     ▼
[NUM 2] [*] [(] [NAME load] [+] [NUM 3] [)]
     │  2. parser: tokens → tree (AST)
     ▼
      Mul
     /   \
  Num 2   Add
         /   \
   Name load  Num 3
     │  3. evaluator: walk the tree with variable values
     ▼
    2 * (40 + 3) = 86
```

## Stage 1: tokens

A **token** is the smallest meaningful piece: a number, a name, an operator, a parenthesis. A tokenizer scans the text left to right; spaces are skipped. Representing token kinds with an `Enum` and tokens as small classes (or tuples) makes the parser simple to write.

## Stage 2: recursive descent parsing

Operator **precedence** (`*` before `+`) is encoded in a **grammar**, one function per level:

```text
expr   := term (("+" | "-") term)*
term   := factor (("*" | "/") factor)*
factor := NUMBER | NAME | "-" factor | "(" expr ")"
```

`expr` calls `term`, which calls `factor`, which may call `expr` again for parentheses: hence *recursive* descent. Because `term` handles `*` before control returns to `expr`, multiplication binds tighter. The loop `(...)*` makes operators **left-associative**: `8 - 3 - 2` is `(8 - 3) - 2`.

## Stage 3: evaluating the tree

The tree is made of node objects (`Num`, `Name`, `BinOp`, `Neg`). Evaluating is a recursive `match` over node types: a number evaluates to itself, a name is looked up in the variables, a binary operation evaluates both sides and combines them. The same tree can be evaluated many times with different variables, which is why real tools parse once and evaluate per row or per measurement.

## Why not just `eval()`?

Python's `eval("2 * load", {"load": 40})` would work, but it executes **arbitrary Python**: a formula like `__import__('os').system('rm -rf /')` would run. Never `eval` text from users, config files or the network. A small dedicated parser only allows what you define. (`ast.literal_eval` is safe but only handles literals.)

## Ideas to extend it

- Comparison and logic (`>`, `and`) for alert rules.
- Functions: `max(a, b)`, `round(x)`.
- Error messages with the position of the bad character (`col 7: unexpected ')'`).
- Turning the tree back into text with minimal parentheses (`__str__` on nodes).

@@quiz
What does a tokenizer produce?
* A list of tokens like numbers, names and operators
- A syntax tree
- The final result
- Bytecode
? Parsing turns tokens into a tree.

@@quiz
In the grammar, why does `*` bind tighter than `+`?
- Python decides
* `term` (which handles `*`) is parsed inside `expr` (which handles `+`), so products are complete before sums combine them
- Because `*` comes first in the text
- Because of the tokenizer
? Lower grammar levels bind tighter.

@@quiz
Why is `eval()` dangerous for user formulas?
- It's slow
* It runs arbitrary Python code, including deleting files
- It can't do math
- It ignores variables
? Write a small parser that only allows what you define.

@@drill
What's the name of the parsing technique where each grammar rule becomes a function?
= recursive descent
= recursive descent parsing
? expression() calls term(), which calls factor().

@@flashcards
Tokenizer (lexer) :: turns text into tokens
Parser :: turns tokens into a syntax tree
AST :: abstract syntax tree
Recursive descent :: a parser with one function per grammar rule
Precedence :: which operators bind tighter
Left-associative :: `a - b - c` means `(a - b) - c`
Evaluator :: walks the tree to compute a value

@@exercise Tokenize a formula
Write `tokenize(text)` returning a list of tokens, each a tuple `(kind, value)`:

- numbers (digits with an optional single `.` part, like `3` or `2.5`) → `("NUM", float)`
- names (letters, digits and `_`, starting with a letter or `_`) → `("NAME", "load")`
- one of `+ - * / ( )` → `("OP", "+")`
- spaces are skipped
- anything else raises `ValueError` with the message `unexpected '<char>' at <index>`
@@starter
def tokenize(text):
    pass


print(tokenize("2 * (load_avg + 3.5)"))
@@tests
check(tokenize("2 * (load_avg + 3.5)"), [("NUM", 2.0), ("OP", "*"), ("OP", "("), ("NAME", "load_avg"), ("OP", "+"), ("NUM", 3.5), ("OP", ")")], "tokens")
check(tokenize(""), [], "empty")
check(tokenize("x1-10"), [("NAME", "x1"), ("OP", "-"), ("NUM", 10.0)], "no spaces")
try:
    tokenize("cpu > 90")
    check("ok", "ValueError", "bad char")
except ValueError as e:
    check(str(e), "unexpected '>' at 4", "message")
@@hint
Use an index `i` and a `while i < len(text)` loop. For a number, advance `j` while `text[j].isdigit() or text[j] == "."`; for a name, while `text[j].isalnum() or text[j] == "_"`. Append, then set `i = j`.
@@solution
def tokenize(text):
    tokens = []
    i = 0
    while i < len(text):
        c = text[i]
        if c == " ":
            i += 1
        elif c.isdigit():
            j = i
            while j < len(text) and (text[j].isdigit() or text[j] == "."):
                j += 1
            tokens.append(("NUM", float(text[i:j])))
            i = j
        elif c.isalpha() or c == "_":
            j = i
            while j < len(text) and (text[j].isalnum() or text[j] == "_"):
                j += 1
            tokens.append(("NAME", text[i:j]))
            i = j
        elif c in "+-*/()":
            tokens.append(("OP", c))
            i += 1
        else:
            raise ValueError(f"unexpected {c!r} at {i}")
    return tokens


print(tokenize("2 * (load_avg + 3.5)"))

@@exercise Parse and evaluate
The tokenizer is given. Write the parser and evaluator:

- `parse(tokens)` returns a tree made of tuples: `("num", 2.0)`, `("name", "x")`, `("neg", node)`, `("bin", op, left, right)`. Follow the grammar from the lesson (`expr`, `term`, `factor`). Leftover tokens or a missing `)` raise `ValueError`.
- `evaluate(node, variables)` computes the value with `match`. An unknown variable raises `KeyError`; division by zero raises `ZeroDivisionError` (Python does this for you).
- `calc(text, **variables)` ties them together.
@@starter
def tokenize(text):
    tokens, i = [], 0
    while i < len(text):
        c = text[i]
        if c == " ":
            i += 1
        elif c.isdigit():
            j = i
            while j < len(text) and (text[j].isdigit() or text[j] == "."):
                j += 1
            tokens.append(("NUM", float(text[i:j])))
            i = j
        elif c.isalpha() or c == "_":
            j = i
            while j < len(text) and (text[j].isalnum() or text[j] == "_"):
                j += 1
            tokens.append(("NAME", text[i:j]))
            i = j
        elif c in "+-*/()":
            tokens.append(("OP", c))
            i += 1
        else:
            raise ValueError(f"unexpected {c!r} at {i}")
    return tokens


def parse(tokens):
    pass


def evaluate(node, variables):
    pass


def calc(text, **variables):
    return evaluate(parse(tokenize(text)), variables)


print(calc("2 * (load + 3)", load=40))
print(calc("8 - 3 - 2"), calc("-2 * -3"), calc("used / total * 100", used=45, total=180))
@@expected
86.0
3.0 6.0 25.0
@@tests
check(parse(tokenize("1 + 2 * x")), ("bin", "+", ("num", 1.0), ("bin", "*", ("num", 2.0), ("name", "x"))), "precedence")
check(calc("(1 + 2) * 3"), 9.0, "parentheses")
check(calc("10 / 4 / 5"), 0.5, "left-assoc division")
check(calc("--x", x=3), 3.0, "double negation")
for bad in ["1 +", "(1 + 2", "1 2", ")"]:
    try:
        calc(bad)
        check(bad, "ValueError", "invalid formula")
    except ValueError:
        pass
try:
    calc("cpu * 2")
    check("ok", "KeyError", "unknown variable")
except KeyError:
    pass
@@hint
Keep a position in a closure or a small class: `pos = 0`, `peek()` returns `tokens[pos]` or `None`, `take()` returns it and advances. `expr`: `node = term(); while peek() in (("OP", "+"), ("OP", "-")): op = take()[1]; node = ("bin", op, node, term())`. `factor` handles NUM, NAME, `-` (return `("neg", factor())`) and `(` (call `expr`, then require `)`); anything else raises ValueError. After `expr()` returns, raise if `pos != len(tokens)`.
@@solution
def tokenize(text):
    tokens, i = [], 0
    while i < len(text):
        c = text[i]
        if c == " ":
            i += 1
        elif c.isdigit():
            j = i
            while j < len(text) and (text[j].isdigit() or text[j] == "."):
                j += 1
            tokens.append(("NUM", float(text[i:j])))
            i = j
        elif c.isalpha() or c == "_":
            j = i
            while j < len(text) and (text[j].isalnum() or text[j] == "_"):
                j += 1
            tokens.append(("NAME", text[i:j]))
            i = j
        elif c in "+-*/()":
            tokens.append(("OP", c))
            i += 1
        else:
            raise ValueError(f"unexpected {c!r} at {i}")
    return tokens


def parse(tokens):
    pos = 0

    def peek():
        return tokens[pos] if pos < len(tokens) else None

    def take():
        nonlocal pos
        tok = peek()
        if tok is None:
            raise ValueError("unexpected end of formula")
        pos += 1
        return tok

    def expr():
        node = term()
        while peek() in (("OP", "+"), ("OP", "-")):
            op = take()[1]
            node = ("bin", op, node, term())
        return node

    def term():
        node = factor()
        while peek() in (("OP", "*"), ("OP", "/")):
            op = take()[1]
            node = ("bin", op, node, factor())
        return node

    def factor():
        kind, value = take()
        if kind == "NUM":
            return ("num", value)
        if kind == "NAME":
            return ("name", value)
        if (kind, value) == ("OP", "-"):
            return ("neg", factor())
        if (kind, value) == ("OP", "("):
            node = expr()
            if take() != ("OP", ")"):
                raise ValueError("expected ')'")
            return node
        raise ValueError(f"unexpected {value!r}")

    tree = expr()
    if pos != len(tokens):
        raise ValueError(f"unexpected {tokens[pos][1]!r}")
    return tree


def evaluate(node, variables):
    match node:
        case ("num", value):
            return value
        case ("name", name):
            return float(variables[name])
        case ("neg", inner):
            return -evaluate(inner, variables)
        case ("bin", op, left, right):
            a, b = evaluate(left, variables), evaluate(right, variables)
            match op:
                case "+":
                    return a + b
                case "-":
                    return a - b
                case "*":
                    return a * b
                case "/":
                    return a / b
    raise ValueError(f"bad node {node!r}")


def calc(text, **variables):
    return evaluate(parse(tokenize(text)), variables)


print(calc("2 * (load + 3)", load=40))
print(calc("8 - 3 - 2"), calc("-2 * -3"), calc("used / total * 100", used=45, total=180))

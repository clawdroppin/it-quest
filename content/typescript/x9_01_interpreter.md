@@lesson x9-interpreter 🎓 Beyond IT: Building an Interpreter
@@concepts strings, functions, objects
@@text
> **Beyond IT scope.** This module goes past IT and business software into computer science and creative programming: language implementation, game engines and simulations. TypeScript's union types make it a great language for these, and they deepen your understanding of how tools you use every day work. Skip it freely if your goal is an IT job.

Every compiler, interpreter, template engine, query language and config format follows the same pipeline:

```text
source text ──► lexer ──► tokens ──► parser ──► AST ──► evaluator (or type checker, or code generator)
"2 * (x + 1)"            [2] [*] [(] ...        Binary(*, Num 2, Binary(+, Var x, Num 1))
```

TypeScript itself (`tsc`), Babel, ESLint, Prettier, SQL engines and even spreadsheet formulas work this way. Discriminated unions are a perfect fit for tokens and syntax trees.

## Tokens

```ts
type Token =
  | { kind: "num"; value: number }
  | { kind: "ident"; name: string }
  | { kind: "op"; op: "+" | "-" | "*" | "/" | "^" | "(" | ")" | "," | "=" };

function lex(src: string): Token[] {
  const tokens: Token[] = [];
  const re = /\s*(?:(\d+(?:\.\d+)?)|([A-Za-z_]\w*)|([-+*/^(),=]))/y;
  let m: RegExpExecArray | null;
  while (re.lastIndex < src.length && (m = re.exec(src))) {
    if (m[1]) tokens.push({ kind: "num", value: Number(m[1]) });
    else if (m[2]) tokens.push({ kind: "ident", name: m[2] });
    else if (m[3]) tokens.push({ kind: "op", op: m[3] as "+" });
  }
  return tokens;
}
console.log(lex("2 * (x + 1)").length);
```

The sticky flag `y` makes the regex match exactly at `lastIndex`, a neat trick for lexers.

## The AST

```ts,norun
type Expr =
  | { type: "num"; value: number }
  | { type: "var"; name: string }
  | { type: "unary"; op: "-"; arg: Expr }
  | { type: "binary"; op: "+" | "-" | "*" | "/" | "^"; left: Expr; right: Expr }
  | { type: "call"; callee: string; args: Expr[] };
```

## Pratt parsing (precedence climbing)

Operators have **binding powers**: `*` binds tighter than `+`, and `^` is right-associative (`2^3^2 = 2^9`). A Pratt parser handles all of that in a few lines:

```text
parseExpr(minPower):
  left = parsePrefix()                  // number, variable, call, (expr), -expr
  loop:
    op = peek(); if op's left power < minPower: stop
    consume op
    right = parseExpr(op's right power)
    left = Binary(op, left, right)
  return left
```

| Operator | left power | right power |
|---|---|---|
| `+` `-` | 1 | 2 |
| `*` `/` | 3 | 4 |
| `^` | 6 | 5 (lower: right-associative) |

## Evaluation

A recursive `switch` over the node type. With `never`-based exhaustiveness checking, adding a new node type to `Expr` makes the compiler point at every place that must handle it: the main reason union-based ASTs are so pleasant in TypeScript.

## Where to go next

Statements and scopes (closures), a bytecode VM, type checking (Hindley-Milner inference), "Crafting Interpreters" by Robert Nystrom (free online), and parser generators (ANTLR, tree-sitter, Chevrotain, Ohm).

@@quiz
What does a lexer produce?
* A list of tokens (numbers, names, operators) from the source text
- An AST
- Machine code
- A result value
? The parser then builds a tree from the tokens.

@@quiz
How does a Pratt parser make `^` right-associative?
- It parses `^` last
* Its right binding power is lower than its left one, so `a^b^c` groups as `a^(b^c)`
- It reverses the tokens
- With a special case in the evaluator
? For left-associative operators, the right power is higher.

@@quiz
Why are discriminated unions a good fit for ASTs?
* Each node type has its own fields, and exhaustive switches catch unhandled node types
- They're faster at runtime
- They avoid recursion
- They make parsing unnecessary
? Add a node type and the compiler lists every place to update.

@@flashcards
Lexer (tokenizer) :: text → tokens
Parser :: tokens → abstract syntax tree
AST :: tree of the program's structure
Pratt parser :: precedence via binding powers
Right-associative :: `a^b^c = a^(b^c)`
Evaluator :: walks the AST to compute a result
Sticky regex `y` :: match exactly at lastIndex
Environment :: map of variable names to values

@@exercise Parse expressions into an AST
Write `parse(src: string): Expr` for the grammar in the lesson:

- numbers (`12`, `3.5`), variables (`x`), calls (`max(a, 2)`) with zero or more comma-separated arguments, parentheses, unary minus, and binary `+ - * / ^` with the binding powers from the table (unary minus binds tighter than `* /` but looser than `^`: `-2^2` is `-(2^2)`)
- syntax errors throw `SyntaxError("unexpected <token text or 'end'>")`, for example `"unexpected )"` or `"unexpected end"`; leftover tokens after a complete expression are also unexpected

Also write `show(e: Expr): string` that prints the AST fully parenthesized: `(1 + (2 * x))`, `(-x)`, `max(a, 2)`.
@@starter
type Token =
  | { kind: "num"; value: number }
  | { kind: "ident"; name: string }
  | { kind: "op"; op: string };
type Expr =
  | { type: "num"; value: number }
  | { type: "var"; name: string }
  | { type: "unary"; op: "-"; arg: Expr }
  | { type: "binary"; op: "+" | "-" | "*" | "/" | "^"; left: Expr; right: Expr }
  | { type: "call"; callee: string; args: Expr[] };

function lex(src: string): Token[] {
  const tokens: Token[] = [];
  const re = /\s*(?:(\d+(?:\.\d+)?)|([A-Za-z_]\w*)|([-+*/^(),=])|(\S))/y;
  let m: RegExpExecArray | null;
  while (re.lastIndex < src.trimEnd().length && (m = re.exec(src))) {
    if (m[1]) tokens.push({ kind: "num", value: Number(m[1]) });
    else if (m[2]) tokens.push({ kind: "ident", name: m[2] });
    else if (m[3]) tokens.push({ kind: "op", op: m[3] });
    else throw new SyntaxError(`unexpected ${m[4]}`);
  }
  return tokens;
}

function parse(src: string): Expr {
  throw new Error("TODO: not done yet");
}

function show(e: Expr): string {
  throw new Error("TODO: not done yet");
}

@@tests
check(show(parse("1 + 2 * x")), "(1 + (2 * x))", "precedence");
check(show(parse("(1 + 2) * x")), "((1 + 2) * x)", "parentheses");
check(show(parse("10 - 4 - 3")), "((10 - 4) - 3)", "left associative");
check(show(parse("2 ^ 3 ^ 2")), "(2 ^ (3 ^ 2))", "right associative");
check(show(parse("-2 ^ 2 * -x")), "((-(2 ^ 2)) * (-x))", "unary minus");
check(show(parse("max(a, 2 + b, min())")), "max(a, (2 + b), min())", "calls");
check(parse("3.5"), { type: "num", value: 3.5 }, "ast shape");
const errors: string[] = [];
for (const src of ["1 +", "(1 + 2", "1 2", ")", "f(1,", "2 $ 3"]) {
  try { parse(src); } catch (e) { errors.push((e as Error).message); }
}
check(errors, ["unexpected end", "unexpected end", "unexpected 2", "unexpected )", "unexpected end", "unexpected $"], "syntax errors");
@@hint
Keep `let pos = 0` with helpers `peek()`, `next()` and `text(t)` (token → string). Binding powers: `const POWER: Record<string, [number, number]> = { "+": [1, 2], "-": [1, 2], "*": [3, 4], "/": [3, 4], "^": [6, 5] }`. In the prefix part, for unary minus parse the operand with `parseExpr(5)` so `^` (left power 6) still binds inside it but `*` doesn't. After an identifier, check for `(` to parse a call.
@@solution
type Token =
  | { kind: "num"; value: number }
  | { kind: "ident"; name: string }
  | { kind: "op"; op: string };
type Expr =
  | { type: "num"; value: number }
  | { type: "var"; name: string }
  | { type: "unary"; op: "-"; arg: Expr }
  | { type: "binary"; op: "+" | "-" | "*" | "/" | "^"; left: Expr; right: Expr }
  | { type: "call"; callee: string; args: Expr[] };

function lex(src: string): Token[] {
  const tokens: Token[] = [];
  const re = /\s*(?:(\d+(?:\.\d+)?)|([A-Za-z_]\w*)|([-+*/^(),=])|(\S))/y;
  let m: RegExpExecArray | null;
  while (re.lastIndex < src.trimEnd().length && (m = re.exec(src))) {
    if (m[1]) tokens.push({ kind: "num", value: Number(m[1]) });
    else if (m[2]) tokens.push({ kind: "ident", name: m[2] });
    else if (m[3]) tokens.push({ kind: "op", op: m[3] });
    else throw new SyntaxError(`unexpected ${m[4]}`);
  }
  return tokens;
}

const POWER: Record<string, [number, number]> = { "+": [1, 2], "-": [1, 2], "*": [3, 4], "/": [3, 4], "^": [6, 5] };
const text = (t: Token | undefined): string =>
  t === undefined ? "end" : t.kind === "num" ? String(t.value) : t.kind === "ident" ? t.name : t.op;

function parse(src: string): Expr {
  const tokens = lex(src);
  let pos = 0;
  const peek = (): Token | undefined => tokens[pos];
  const fail = (): never => {
    throw new SyntaxError(`unexpected ${text(peek())}`);
  };
  const isOp = (op: string): boolean => {
    const t = peek();
    return t !== undefined && t.kind === "op" && t.op === op;
  };
  const expect = (op: string): void => {
    if (!isOp(op)) fail();
    pos++;
  };

  function prefix(): Expr {
    const t = peek();
    if (t === undefined) return fail();
    pos++;
    if (t.kind === "num") return { type: "num", value: t.value };
    if (t.kind === "ident") {
      if (!isOp("(")) return { type: "var", name: t.name };
      pos++;
      const args: Expr[] = [];
      if (!isOp(")")) {
        args.push(expr(0));
        while (isOp(",")) {
          pos++;
          args.push(expr(0));
        }
      }
      expect(")");
      return { type: "call", callee: t.name, args };
    }
    if (t.op === "(") {
      const inner = expr(0);
      expect(")");
      return inner;
    }
    if (t.op === "-") return { type: "unary", op: "-", arg: expr(5) };
    pos--;
    return fail();
  }

  function expr(minPower: number): Expr {
    let left = prefix();
    for (;;) {
      const t = peek();
      if (t === undefined || t.kind !== "op") break;
      const power = POWER[t.op];
      if (!power || power[0] < minPower) break;
      pos++;
      const right = expr(power[1]);
      left = { type: "binary", op: t.op as "+" | "-" | "*" | "/" | "^", left, right };
    }
    return left;
  }

  const result = expr(0);
  if (pos < tokens.length) fail();
  return result;
}

function show(e: Expr): string {
  switch (e.type) {
    case "num": return String(e.value);
    case "var": return e.name;
    case "unary": return `(-${show(e.arg)})`;
    case "binary": return `(${show(e.left)} ${e.op} ${show(e.right)})`;
    case "call": return `${e.callee}(${e.args.map(show).join(", ")})`;
  }
}

@@exercise Evaluate with variables and functions
The parser is given. Write `evaluate(e: Expr, env: Env): number` with `type Env = { vars: Record<string, number>; fns: Record<string, (...args: number[]) => number> }`:

- unknown variable → `ReferenceError("unknown variable <name>")`; unknown function → `ReferenceError("unknown function <name>")`
- division by zero → `RangeError("division by zero")`
- use an exhaustive `switch` with `assertNever`

Then write `run(program: string, env: Env): number[]`: the program has one statement per line (blank lines skipped); a line `name = expr` assigns (and yields the value), any other line is an expression. Each statement's value is collected.
@@starter
type Token =
  | { kind: "num"; value: number }
  | { kind: "ident"; name: string }
  | { kind: "op"; op: string };
type Expr =
  | { type: "num"; value: number }
  | { type: "var"; name: string }
  | { type: "unary"; op: "-"; arg: Expr }
  | { type: "binary"; op: "+" | "-" | "*" | "/" | "^"; left: Expr; right: Expr }
  | { type: "call"; callee: string; args: Expr[] };
type Env = { vars: Record<string, number>; fns: Record<string, (...args: number[]) => number> };

function lex(src: string): Token[] {
  const tokens: Token[] = [];
  const re = /\s*(?:(\d+(?:\.\d+)?)|([A-Za-z_]\w*)|([-+*/^(),=])|(\S))/y;
  let m: RegExpExecArray | null;
  while (re.lastIndex < src.trimEnd().length && (m = re.exec(src))) {
    if (m[1]) tokens.push({ kind: "num", value: Number(m[1]) });
    else if (m[2]) tokens.push({ kind: "ident", name: m[2] });
    else if (m[3]) tokens.push({ kind: "op", op: m[3] });
    else throw new SyntaxError(`unexpected ${m[4]}`);
  }
  return tokens;
}

const POWER: Record<string, [number, number]> = { "+": [1, 2], "-": [1, 2], "*": [3, 4], "/": [3, 4], "^": [6, 5] };

function parse(src: string): Expr {
  const tokens = lex(src);
  let pos = 0;
  const peek = (): Token | undefined => tokens[pos];
  const fail = (): never => {
    const t = peek();
    throw new SyntaxError(`unexpected ${t === undefined ? "end" : t.kind === "num" ? t.value : t.kind === "ident" ? t.name : t.op}`);
  };
  const isOp = (op: string): boolean => {
    const t = peek();
    return t !== undefined && t.kind === "op" && t.op === op;
  };
  const expect = (op: string): void => {
    if (!isOp(op)) fail();
    pos++;
  };
  function prefix(): Expr {
    const t = peek();
    if (t === undefined) return fail();
    pos++;
    if (t.kind === "num") return { type: "num", value: t.value };
    if (t.kind === "ident") {
      if (!isOp("(")) return { type: "var", name: t.name };
      pos++;
      const args: Expr[] = [];
      if (!isOp(")")) {
        args.push(expr(0));
        while (isOp(",")) { pos++; args.push(expr(0)); }
      }
      expect(")");
      return { type: "call", callee: t.name, args };
    }
    if (t.op === "(") { const inner = expr(0); expect(")"); return inner; }
    if (t.op === "-") return { type: "unary", op: "-", arg: expr(5) };
    pos--;
    return fail();
  }
  function expr(minPower: number): Expr {
    let left = prefix();
    for (;;) {
      const t = peek();
      if (t === undefined || t.kind !== "op") break;
      const power = POWER[t.op];
      if (!power || power[0] < minPower) break;
      pos++;
      left = { type: "binary", op: t.op as "+" | "-" | "*" | "/" | "^", left, right: expr(power[1]) };
    }
    return left;
  }
  const result = expr(0);
  if (pos < tokens.length) fail();
  return result;
}

function evaluate(e: Expr, env: Env): number {
  throw new Error("TODO: not done yet");
}

function run(program: string, env: Env): number[] {
  throw new Error("TODO: not done yet");
}

@@tests
const env: Env = { vars: { pi: Math.PI }, fns: { max: Math.max, sqrt: Math.sqrt, round: Math.round } };
check(evaluate(parse("2 ^ 3 ^ 2 - 10 / 4"), env), 509.5, "arithmetic");
check(evaluate(parse("round(pi * 100)"), env), 314, "vars and calls");
check(run("w = 1920\nh = 1080\n\nratio = w / h\nround(ratio * 100) / 100\nmax(w, h, sqrt(w * h))", env), [1920, 1080, 1920 / 1080, 1.78, 1920], "program");
check(env.vars.ratio, 1920 / 1080, "assignment stored");
const errs: string[] = [];
for (const src of ["nope + 1", "cube(3)", "1 / (2 - 2)"]) {
  try { evaluate(parse(src), env); } catch (e) { errs.push(`${(e as Error).name}: ${(e as Error).message}`); }
}
check(errs, ["ReferenceError: unknown variable nope", "ReferenceError: unknown function cube", "RangeError: division by zero"], "errors");
@@hint
For assignments, match the line with `/^\s*([A-Za-z_]\w*)\s*=(.*)$/` (the parser doesn't know `=`). In `evaluate`, `case "binary"` evaluates both sides then switches on `e.op`. Look up variables with `Object.hasOwn(env.vars, name)` so names like `toString` aren't found on the prototype.
@@solution
type Token =
  | { kind: "num"; value: number }
  | { kind: "ident"; name: string }
  | { kind: "op"; op: string };
type Expr =
  | { type: "num"; value: number }
  | { type: "var"; name: string }
  | { type: "unary"; op: "-"; arg: Expr }
  | { type: "binary"; op: "+" | "-" | "*" | "/" | "^"; left: Expr; right: Expr }
  | { type: "call"; callee: string; args: Expr[] };
type Env = { vars: Record<string, number>; fns: Record<string, (...args: number[]) => number> };

function lex(src: string): Token[] {
  const tokens: Token[] = [];
  const re = /\s*(?:(\d+(?:\.\d+)?)|([A-Za-z_]\w*)|([-+*/^(),=])|(\S))/y;
  let m: RegExpExecArray | null;
  while (re.lastIndex < src.trimEnd().length && (m = re.exec(src))) {
    if (m[1]) tokens.push({ kind: "num", value: Number(m[1]) });
    else if (m[2]) tokens.push({ kind: "ident", name: m[2] });
    else if (m[3]) tokens.push({ kind: "op", op: m[3] });
    else throw new SyntaxError(`unexpected ${m[4]}`);
  }
  return tokens;
}

const POWER: Record<string, [number, number]> = { "+": [1, 2], "-": [1, 2], "*": [3, 4], "/": [3, 4], "^": [6, 5] };

function parse(src: string): Expr {
  const tokens = lex(src);
  let pos = 0;
  const peek = (): Token | undefined => tokens[pos];
  const fail = (): never => {
    const t = peek();
    throw new SyntaxError(`unexpected ${t === undefined ? "end" : t.kind === "num" ? t.value : t.kind === "ident" ? t.name : t.op}`);
  };
  const isOp = (op: string): boolean => {
    const t = peek();
    return t !== undefined && t.kind === "op" && t.op === op;
  };
  const expect = (op: string): void => {
    if (!isOp(op)) fail();
    pos++;
  };
  function prefix(): Expr {
    const t = peek();
    if (t === undefined) return fail();
    pos++;
    if (t.kind === "num") return { type: "num", value: t.value };
    if (t.kind === "ident") {
      if (!isOp("(")) return { type: "var", name: t.name };
      pos++;
      const args: Expr[] = [];
      if (!isOp(")")) {
        args.push(expr(0));
        while (isOp(",")) { pos++; args.push(expr(0)); }
      }
      expect(")");
      return { type: "call", callee: t.name, args };
    }
    if (t.op === "(") { const inner = expr(0); expect(")"); return inner; }
    if (t.op === "-") return { type: "unary", op: "-", arg: expr(5) };
    pos--;
    return fail();
  }
  function expr(minPower: number): Expr {
    let left = prefix();
    for (;;) {
      const t = peek();
      if (t === undefined || t.kind !== "op") break;
      const power = POWER[t.op];
      if (!power || power[0] < minPower) break;
      pos++;
      left = { type: "binary", op: t.op as "+" | "-" | "*" | "/" | "^", left, right: expr(power[1]) };
    }
    return left;
  }
  const result = expr(0);
  if (pos < tokens.length) fail();
  return result;
}

function assertNever(x: never): never {
  throw new Error(`unexpected node ${JSON.stringify(x)}`);
}

function applyOp(op: "+" | "-" | "*" | "/" | "^", a: number, b: number): number {
  switch (op) {
    case "+": return a + b;
    case "-": return a - b;
    case "*": return a * b;
    case "/":
      if (b === 0) throw new RangeError("division by zero");
      return a / b;
    case "^": return a ** b;
    default: return assertNever(op);
  }
}

function evaluate(e: Expr, env: Env): number {
  switch (e.type) {
    case "num":
      return e.value;
    case "var": {
      if (!Object.hasOwn(env.vars, e.name)) throw new ReferenceError(`unknown variable ${e.name}`);
      return env.vars[e.name] ?? NaN;
    }
    case "unary":
      return -evaluate(e.arg, env);
    case "binary":
      return applyOp(e.op, evaluate(e.left, env), evaluate(e.right, env));
    case "call": {
      const fn = Object.hasOwn(env.fns, e.callee) ? env.fns[e.callee] : undefined;
      if (!fn) throw new ReferenceError(`unknown function ${e.callee}`);
      return fn(...e.args.map((a) => evaluate(a, env)));
    }
    default:
      return assertNever(e);
  }
}

function run(program: string, env: Env): number[] {
  const results: number[] = [];
  for (const line of program.split("\n")) {
    if (line.trim() === "") continue;
    const assign = /^\s*([A-Za-z_]\w*)\s*=(.*)$/.exec(line);
    if (assign) {
      const [, name = "", rest = ""] = assign;
      const value = evaluate(parse(rest), env);
      env.vars[name] = value;
      results.push(value);
    } else {
      results.push(evaluate(parse(line), env));
    }
  }
  return results;
}

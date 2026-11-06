@@lesson t0-welcome Welcome to TypeScript
@@concepts hello, variables
@@text
## JavaScript with types

**TypeScript** is JavaScript plus a type system. You write types; the TypeScript compiler (`tsc`) checks them *before* the code runs, then the types are removed and plain JavaScript is left.

```typescript
const host: string = "web01";
let port: number = 443;
const online: boolean = true;

console.log(`${host}:${port} online=${online}`);
// port = "https";   ← tsc: Type 'string' is not assignable to type 'number'.
```

Why it matters in IT:

- Most new web front ends, Node.js services, and cloud tools are written in TypeScript: VS Code, the Azure and AWS CDKs, Pulumi, Angular, and much of the npm ecosystem.
- In a big codebase, types catch typos, wrong arguments, and "undefined is not a function" before production does.
- Your editor knows what everything is: autocomplete, rename across files, and jump to definition all work.

This track assumes you've done the first lessons of the **JavaScript** track: TypeScript *is* JavaScript, so everything there still applies.

## How this app runs TypeScript

Node.js (version 22.6 or newer) can run `.ts` files directly by **stripping the types**. That's what the ▶ Run button does. Two consequences:

1. **The types aren't checked when you press Run.** Code with a type error still runs. Type checking is `tsc`'s job: you'll set it up in the projects (`npx tsc --noEmit`), and VS Code shows the errors live as you type.
2. Only syntax that can be erased works: no `enum`, no `namespace`, and no `constructor(private x: number)` shorthand. Modern TypeScript style avoids them anyway; the lessons show the alternatives.

## Type inference

You rarely need to write types on variables: TypeScript **infers** them.

```typescript
let count = 5;            // inferred: number
const name = "db01";      // inferred: the literal type "db01" (a const can never change)
const ports = [22, 80];   // inferred: number[]

count += 1;
console.log(count, name, ports.length);
```

Write types where they help: **function parameters**, return types of exported functions, and empty values whose type can't be guessed (`let users: string[] = [];`).

## Types are erased

Types exist only at compile time. At run time it's plain JavaScript, so you can't ask "is this value a `Server`?" with types alone. You check the *data* (`typeof x === "string"`), which you'll learn under *narrowing*.

```typescript
function greet(user: string): string {
  return `ようこそ, ${user}!`;
}
console.log(greet("tanaka"));
console.log(typeof greet);   // "function": JavaScript's typeof, at run time
```

## `any` and `unknown`

- `any` switches type checking **off** for that value. It spreads, and it hides bugs. Avoid it.
- `unknown` means "could be anything, so check before you use it". Use it for data from outside: JSON, user input, API responses.

```typescript
const data: unknown = JSON.parse('{"host": "web01"}');
// data.host;  ← error: 'data' is of type 'unknown'
if (typeof data === "object" && data !== null && "host" in data) {
  console.log((data as { host: string }).host);
}
```

@@quiz
What happens to TypeScript types when the code runs?
* They're removed: the code that runs is plain JavaScript
- They're checked again at run time
- They make the code run faster
- They're turned into comments
? That's why you validate outside data at run time, for example with type guards.

@@quiz
In this app, you press Run on code with a type error. What happens?
* It runs anyway: Node strips the types without checking them. `tsc` or your editor reports the error
- It refuses to run
- Node prints the type error and stops
- The types are converted to `any`
? Use `tsc --noEmit` (or VS Code) to check types.

@@quiz
Which type should you use for parsed JSON whose shape you haven't checked yet?
* `unknown`
- `any`
- `object`
- `string`
? `unknown` forces you to check before using it. `any` silently allows anything.

@@drill
Which type is the safe alternative to `any` for values you haven't checked yet?
= unknown
? You must narrow it before using it.

@@flashcards
`let x: number = 1` :: a type annotation
type inference :: TypeScript works out the type from the value
`tsc` :: the TypeScript compiler / type checker
`tsc --noEmit` :: check types without writing JavaScript files
`any` :: turns type checking off (avoid)
`unknown` :: anything, but you must check before using it
type stripping :: Node removing types to run `.ts` directly

@@exercise Typed server label
Write `label(host: string, port: number, tls: boolean): string` that returns `https://web01:443` when `tls` is true and `http://web01:8080` style when it's false. Leave out the port if it's the default (443 for https, 80 for http).
@@starter
function label(host: string, port: number, tls: boolean): string {
  throw new Error("TODO: not done yet");
}

@@tests
check(label("web01", 443, true), "https://web01", "default https port");
check(label("web01", 8443, true), "https://web01:8443", "custom https port");
check(label("intranet", 80, false), "http://intranet", "default http port");
check(label("intranet", 8080, false), "http://intranet:8080", "custom http port");
@@hint
`const scheme = tls ? "https" : "http";` and `const defaultPort = tls ? 443 : 80;`. Add `:${port}` only when `port !== defaultPort`.
@@solution
function label(host: string, port: number, tls: boolean): string {
  const scheme = tls ? "https" : "http";
  const defaultPort = tls ? 443 : 80;
  return port === defaultPort ? `${scheme}://${host}` : `${scheme}://${host}:${port}`;
}

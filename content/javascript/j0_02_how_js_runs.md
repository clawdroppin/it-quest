@@lesson j0-how-js-runs How JavaScript Runs: Engines, Node.js and the Browser
@@concepts hello, variables
@@text
JavaScript was created in 1995 to make web pages interactive. Today the same language runs in every browser, on servers (Node.js, Deno, Bun), in desktop apps (Electron, Tauri front ends, VS Code itself), in databases and in automation scripts. Knowing **where** your code runs explains most of what it can and can't do.

## Engines and runtimes

```text
 your code (.js)
      │
      ▼
 ┌─────────────── runtime ────────────────┐
 │  engine (V8, SpiderMonkey, JavaScriptCore)
 │    parse → bytecode → JIT-compiled machine code
 │  + APIs the runtime adds:
 │      browser: DOM, fetch, localStorage, canvas
 │      Node.js: fs, http, child_process, process
 │  + event loop (runs callbacks, timers, I/O)
 └────────────────────────────────────────┘
```

- The **engine** runs the language itself: variables, functions, objects, arrays. Chrome and Node.js both use Google's **V8**.
- The **runtime** adds the APIs. `document` exists only in browsers; `require("fs")` only in Node.js. The language standard (ECMAScript, a new edition every June) defines neither.
- Engines **JIT-compile** hot code to machine code, which is why modern JavaScript is fast despite being dynamic.

## Running code

```javascript
console.log("Hello from", typeof process !== "undefined" ? "Node.js" : "a browser");
console.log("Node version:", process.version);
console.log("platform:", process.platform, "args:", process.argv.length);
```

- `node file.js` runs a file; `node` alone opens the **REPL** (type code, see results, `.exit` to quit).
- In a browser, press F12 for the **DevTools console**, the same idea.
- `node --watch file.js` reruns on every save.

## The console is more than log

```javascript
console.log("normal output");
console.error("goes to stderr");
console.table([{ host: "web01", up: true }, { host: "db01", up: false }]);
console.time("loop");
let total = 0;
for (let i = 0; i < 1e6; i++) total += i;
console.timeEnd("loop");
```

`console.error` writes to **stderr**, so it shows up even when normal output is redirected to a file. `console.table` prints arrays of objects as a table, handy for quick inspection.

## Strict mode and modules

Old JavaScript silently allowed mistakes, like assigning to an undeclared variable (which created a global). **Strict mode** turns those into errors. It's on automatically in ES modules and classes; in plain scripts you opt in with `"use strict";` as the first line.

Node.js has two module systems:

| | CommonJS | ES modules |
|---|---|---|
| Syntax | `const fs = require("fs")` / `module.exports = ...` | `import fs from "node:fs"` / `export ...` |
| Files | `.js` (default), `.cjs` | `.mjs`, or `.js` with `"type": "module"` in package.json |
| Loading | synchronous | asynchronous, allows top-level `await` |

Browsers only understand ES modules. New Node.js projects should use ES modules too; you'll still read plenty of CommonJS. The exercises in this course run as plain scripts and use `require` when they need Node modules.

## Versions

Node.js releases a new major version every six months; **even-numbered** versions become **LTS** (long-term support, about 30 months). Use an LTS version for servers and tools. `nvm` (Linux/macOS) or `nvm-windows`/`fnm` switch between versions.

@@quiz
What does the JavaScript engine provide, and what does the runtime add?
* The engine runs the language; the runtime adds APIs like the DOM (browser) or fs and http (Node.js)
- The engine adds the DOM; the runtime runs the language
- They are the same thing
- The runtime compiles TypeScript
? `document` doesn't exist in Node.js, and `require("fs")` doesn't exist in browsers.

@@quiz
Where does `console.error` write?
- To a log file
* To stderr, separate from normal output
- Only to the browser
- Nowhere unless DEBUG is set
? Redirecting stdout (`> out.txt`) still shows errors on screen.

@@quiz
Which Node.js versions should you use for servers?
- The newest odd-numbered release
* An LTS (even-numbered) release
- Version 0.12
- Whatever npm installs
? LTS versions get security fixes for about 30 months.

@@drill
Which JavaScript engine do Chrome and Node.js use?
= V8
? Firefox uses SpiderMonkey; Safari uses JavaScriptCore.

@@flashcards
V8 :: Google's JavaScript engine used by Chrome and Node.js
Runtime :: engine plus APIs and an event loop (browser, Node.js, Deno, Bun)
ECMAScript :: the language standard; new edition every year
REPL :: interactive prompt: read, evaluate, print, loop
`console.table` :: prints arrays of objects as a table
Strict mode :: turns silent mistakes into errors
CommonJS :: `require` / `module.exports`
ES modules :: `import` / `export`; the standard module system
LTS :: long-term support release (even-numbered Node.js versions)

@@exercise Describe the environment
Write `describeRuntime(info)` that takes an object like `{ name: "node", version: "v24.11.0", platform: "win32" }` and returns `"Node.js 24 on Windows"`.

- names: `node` → `Node.js`, `deno` → `Deno`, `bun` → `Bun`, anything else unchanged
- the major version is the first number in `version` (a leading `v` is optional)
- platforms: `win32` → `Windows`, `darwin` → `macOS`, `linux` → `Linux`, anything else unchanged
- add `" (LTS line)"` when the runtime is Node.js and the major version is even
@@starter
function describeRuntime(info) {
  throw new Error("TODO: not done yet");
}

@@tests
check(describeRuntime({ name: "node", version: "v24.11.0", platform: "win32" }), "Node.js 24 on Windows (LTS line)", "Node 24 on Windows");
check(describeRuntime({ name: "node", version: "25.1.0", platform: "linux" }), "Node.js 25 on Linux", "odd Node version");
check(describeRuntime({ name: "deno", version: "2.5.4", platform: "darwin" }), "Deno 2 on macOS", "Deno on macOS");
check(describeRuntime({ name: "workerd", version: "v1", platform: "freebsd" }), "workerd 1 on freebsd", "unknown values");
@@hint
Use lookup objects: `const names = { node: "Node.js", deno: "Deno", bun: "Bun" };` then `names[info.name] ?? info.name`. Get the major with `parseInt(info.version.replace(/^v/, ""), 10)`.
@@solution
function describeRuntime(info) {
  const names = { node: "Node.js", deno: "Deno", bun: "Bun" };
  const platforms = { win32: "Windows", darwin: "macOS", linux: "Linux" };
  const major = parseInt(info.version.replace(/^v/, ""), 10);
  let text = `${names[info.name] ?? info.name} ${major} on ${platforms[info.platform] ?? info.platform}`;
  if (info.name === "node" && major % 2 === 0) text += " (LTS line)";
  return text;
}

@@exercise Read command-line arguments
Node puts the command line in `process.argv`: `[nodePath, scriptPath, ...args]`. Write `parseArgs(argv)` that skips the first two entries and returns `{ flags: {...}, positional: [...] }`:

- `--name=value` → `flags.name = "value"`
- `--verbose` (no `=`) → `flags.verbose = true`
- `--` on its own: everything after it is positional, even if it starts with `--`
- anything else is positional
@@starter
function parseArgs(argv) {
  throw new Error("TODO: not done yet");
}

@@tests
check(parseArgs(["node", "ping.js", "--count=3", "web01", "--verbose", "db01"]), { flags: { count: "3", verbose: true }, positional: ["web01", "db01"] }, "flags and hosts");
check(parseArgs(["node", "x.js"]), { flags: {}, positional: [] }, "no arguments");
check(parseArgs(["node", "x.js", "--", "--not-a-flag", "a"]), { flags: {}, positional: ["--not-a-flag", "a"] }, "after --");
check(parseArgs(["node", "x.js", "--url=http://h/?a=b"]), { flags: { url: "http://h/?a=b" }, positional: [] }, "value containing =");
@@hint
Loop over `argv.slice(2)` with a `rest` flag. For flags use `arg.indexOf("=")` and split only at the first `=`: `arg.slice(2, eq)` and `arg.slice(eq + 1)`. (Real tools use `util.parseArgs` from `node:util`.)
@@solution
function parseArgs(argv) {
  const flags = {};
  const positional = [];
  let rest = false;
  for (const arg of argv.slice(2)) {
    if (rest) {
      positional.push(arg);
    } else if (arg === "--") {
      rest = true;
    } else if (arg.startsWith("--")) {
      const eq = arg.indexOf("=");
      if (eq === -1) flags[arg.slice(2)] = true;
      else flags[arg.slice(2, eq)] = arg.slice(eq + 1);
    } else {
      positional.push(arg);
    }
  }
  return { flags, positional };
}

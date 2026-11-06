@@lesson l7-modules Modules: ES Modules, CommonJS and Packages
@@concepts modules
@@text
A **module** is a file with its own scope that chooses what to **export**. Modules are how JavaScript projects stay organised, and the two module systems in Node.js are a frequent source of confusing errors.

## ES modules (the standard)

```javascript,norun
// lib/net.js
export const DEFAULT_PORT = 443;
export function parseHost(text) { /* ... */ }
export default class Client { /* ... */ }      // one default export per module

// app.js
import Client, { parseHost, DEFAULT_PORT as PORT } from "./lib/net.js";
import * as net from "./lib/net.js";            // namespace object: net.parseHost
import fs from "node:fs/promises";              // built-ins use the node: prefix
import config from "./config.json" with { type: "json" };   // import attributes

const data = await fs.readFile("hosts.txt", "utf8");        // top-level await
```

Key facts:

- Imports are **static**: they must be at the top level, with string literal paths, so tools can analyse and **tree-shake** (remove unused exports).
- In Node.js and browsers, relative imports need the **file extension** (`./net.js`).
- Imported bindings are **live** and read-only: if the exporting module changes `export let count`, importers see the new value; they can't assign to it.
- Modules run **once**, the first time they're imported; later imports get the same instance (useful for shared config or a single DB pool).
- ES modules are always in strict mode and `this` at the top level is `undefined`.

## Dynamic import

```javascript
async function loadFormatter(kind) {
  const mod = kind === "json" ? await import("node:util") : await import("node:path");
  return Object.keys(mod).length > 0;
}
loadFormatter("json").then((ok) => console.log("loaded:", ok));
```

`import()` returns a promise and works in both module systems. Use it to load code on demand (plugins, big optional features, code-splitting in web apps).

## CommonJS (Node's original system)

```javascript
const path = require("node:path");
const { EventEmitter } = require("node:events");
module.exports = { joinAll: (...parts) => path.join(...parts) };
console.log(typeof module.exports.joinAll, typeof EventEmitter);
```

| | ES modules | CommonJS |
|---|---|---|
| Import | `import x from "y"` | `const x = require("y")` |
| Export | `export` / `export default` | `module.exports = ...` / `exports.x = ...` |
| When | resolved before running (static) | at the `require` call (dynamic, synchronous) |
| `__dirname` | `import.meta.dirname` | `__dirname` |
| Top-level await | yes | no |

**Interop:** ESM can `import` a CommonJS module (you get `module.exports` as the default). CommonJS can `require()` an ES module in Node 22.12+ if it has no top-level await; otherwise use `await import()`.

## package.json essentials

```json
{
  "name": "@acme/net-tools",
  "version": "1.4.2",
  "type": "module",
  "exports": {
    ".": "./src/index.js",
    "./cli": "./src/cli.js"
  },
  "bin": { "net-tools": "./src/cli.js" },
  "engines": { "node": ">=22" },
  "dependencies": { "undici": "^7.0.0" },
  "devDependencies": { "eslint": "^9.0.0" },
  "scripts": { "test": "node --test", "lint": "eslint ." }
}
```

- `"type": "module"` makes `.js` files ES modules.
- `"exports"` defines the public entry points; deep imports of other files are blocked.
- **Semantic versioning**: `MAJOR.MINOR.PATCH`. `^7.0.0` allows 7.x.y; `~7.1.0` allows 7.1.y.
- Commit **package-lock.json** so every install gets identical versions; use `npm ci` in CI.

## Circular imports

If `a.js` imports `b.js` and `b.js` imports `a.js`, one of them sees the other **half-initialised**. Symptoms: `undefined` imports or "Cannot access before initialization" errors. Fix by moving shared code into a third module.

@@quiz
Why must ES module imports be static?
* So tools can resolve dependencies before running and remove unused code (tree-shaking)
- Because Node.js is slow
- So imports can be conditional
- They don't have to be
? Use `import()` for dynamic loading.

@@quiz
A module is imported by three files. How many times does its top-level code run?
- Three
* Once; later imports share the same instance
- Zero
- Once per function call
? This makes modules natural singletons.

@@quiz
What does `"^7.2.0"` allow npm to install?
- Exactly 7.2.0
* Any 7.x.y version at or above 7.2.0
- Any version
- 7.2.x only
? `^` allows minor and patch updates within the same major.

@@drill
Which package.json field makes Node treat .js files as ES modules?
= "type": "module"
= type
= type: module
? Without it, .js files are CommonJS.

@@flashcards
`export default` :: the module's main export; one per module
Named export :: `export function f()`, imported with `{ f }`
`import * as ns` :: namespace object of all exports
`import()` :: dynamic import returning a promise
`require` :: CommonJS synchronous import
`"type": "module"` :: makes .js files ES modules
`"exports"` :: package.json map of public entry points
Tree-shaking :: removing unused exports when bundling
package-lock.json :: exact installed versions; commit it

@@exercise A tiny module loader
To see how `require` works, write `createLoader(files)`, where `files` maps paths to source code strings. It returns a `load(path)` function that:

- runs the source as a function body with `module`, `exports` and `require` parameters (use `new Function("module", "exports", "require", source)`)
- returns `module.exports`
- **caches** modules: each file runs only once
- handles **circular** requires like Node: put the module in the cache **before** running it, so a cycle returns the partially-filled exports
- throws `Error("Cannot find module '<path>'")` for unknown paths
@@starter
function createLoader(files) {
  throw new Error("TODO: not done yet");
}

@@tests
const files = {
  "math.js": "globalThis.__runs = (globalThis.__runs ?? 0) + 1; exports.add = (a, b) => a + b;",
  "app.js": "const { add } = require('math.js'); module.exports = add(2, 3);",
  "a.js": "exports.name = 'a'; const b = require('b.js'); exports.seenB = b.name;",
  "b.js": "const a = require('a.js'); exports.name = 'b'; exports.seenA = a.name;",
};
const load = createLoader(files);
check(load("app.js"), 5, "require works");
load("math.js");
check(globalThis.__runs, 1, "cached");
check(load("a.js"), { name: "a", seenB: "b" }, "circular a");
check(load("b.js"), { name: "b", seenA: "a" }, "circular b sees partial a");
let msg = "";
try { load("nope.js"); } catch (e) { msg = e.message; }
check(msg, "Cannot find module 'nope.js'", "missing module");
@@hint
Keep `const cache = {}`. In `load(path)`: return `cache[path].exports` if cached; throw if `!(path in files)`; otherwise `const module = { exports: {} }; cache[path] = module;` then `new Function("module", "exports", "require", files[path])(module, module.exports, load);` and return `module.exports`.
@@solution
function createLoader(files) {
  const cache = {};
  function load(path) {
    if (cache[path]) return cache[path].exports;
    if (!(path in files)) throw new Error(`Cannot find module '${path}'`);
    const module = { exports: {} };
    cache[path] = module;
    const run = new Function("module", "exports", "require", files[path]);
    run(module, module.exports, load);
    return module.exports;
  }
  return load;
}

@@exercise Semantic version ranges
Write `satisfies(version, range)` for npm-style ranges:

- `"1.4.2"` exact match
- `"^1.4.2"`: same major, and ≥ 1.4.2; but for `^0.x.y` the **minor** must also match (npm treats 0.x as unstable)
- `"~1.4.2"`: same major and minor, ≥ 1.4.2
- `">=1.4.0"`, `"<2.0.0"`, and space-separated combinations like `">=1.2.0 <1.5.0"` (all must hold)
- `"*"` matches everything

Also write `maxSatisfying(versions, range)` returning the highest matching version or `null`. Versions are always `MAJOR.MINOR.PATCH`.
@@starter
function satisfies(version, range) {
  throw new Error("TODO: not done yet");
}

function maxSatisfying(versions, range) {
  throw new Error("TODO: not done yet");
}

@@tests
check(["1.4.2", "1.9.0", "2.0.0", "1.4.1"].map((v) => satisfies(v, "^1.4.2")), [true, true, false, false], "caret");
check(["0.3.5", "0.4.0"].map((v) => satisfies(v, "^0.3.1")), [true, false], "caret on 0.x");
check(["1.4.9", "1.5.0"].map((v) => satisfies(v, "~1.4.2")), [true, false], "tilde");
check(satisfies("1.3.0", ">=1.2.0 <1.5.0"), true, "combined range");
check(satisfies("1.5.0", ">=1.2.0 <1.5.0"), false, "upper bound");
check(satisfies("3.1.4", "*"), true, "star");
check(maxSatisfying(["1.2.0", "1.10.0", "1.9.9", "2.0.0"], "^1.0.0"), "1.10.0", "numeric, not string, comparison");
check(maxSatisfying(["1.0.0"], "^2.0.0"), null, "none");
@@hint
Parse with `v.split(".").map(Number)` and compare arrays with a `compare(a, b)` that returns the first non-zero difference. Turn `^` and `~` into lower and upper bounds: `^1.4.2` → `>=1.4.2 <2.0.0`; `^0.3.1` → `<0.4.0`; `~1.4.2` → `<1.5.0`.
@@solution
const parse = (v) => v.split(".").map(Number);

function compare(a, b) {
  for (let i = 0; i < 3; i++) if (a[i] !== b[i]) return a[i] - b[i];
  return 0;
}

function test(v, part) {
  if (part === "*" || part === "") return true;
  const m = part.match(/^(\^|~|>=|<=|>|<|=)?(\d+\.\d+\.\d+)$/);
  if (!m) throw new Error(`bad range ${part}`);
  const [, op = "=", ver] = m;
  const target = parse(ver);
  const c = compare(v, target);
  switch (op) {
    case "=": return c === 0;
    case ">=": return c >= 0;
    case "<=": return c <= 0;
    case ">": return c > 0;
    case "<": return c < 0;
    case "~": return c >= 0 && v[0] === target[0] && v[1] === target[1];
    case "^":
      if (c < 0 || v[0] !== target[0]) return false;
      return target[0] !== 0 || v[1] === target[1];
  }
}

function satisfies(version, range) {
  const v = parse(version);
  return range.trim().split(/\s+/).every((part) => test(v, part));
}

function maxSatisfying(versions, range) {
  const ok = versions.filter((v) => satisfies(v, range));
  if (ok.length === 0) return null;
  return ok.reduce((best, v) => (compare(parse(v), parse(best)) > 0 ? v : best));
}

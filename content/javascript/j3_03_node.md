@@lesson j3-node Node.js: Files, Modules, CLI Tools and npm
@@concepts files, modules
@@text
## Files and paths

```javascript
const fs = require("node:fs");
const path = require("node:path");
const os = require("node:os");

const dir = fs.mkdtempSync(path.join(os.tmpdir(), "itq-"));
const file = path.join(dir, "servers.json");
fs.writeFileSync(file, JSON.stringify(["web01", "db01"]));
const servers = JSON.parse(fs.readFileSync(file, "utf8"));
console.log(servers.length, path.basename(file), path.extname(file));
console.log(fs.readdirSync(dir));
fs.rmSync(dir, { recursive: true });
```

Every `...Sync` function has an asynchronous version in `require("node:fs/promises")`: `await fs.readFile(file, "utf8")`. Servers use the async versions so one slow disk read doesn't block everything else.

## Modules

Split code into files. With **CommonJS** (the classic Node style, used in this course's runner):

```javascript,norun
// checks.js
function isPortValid(p) { return Number.isInteger(p) && p >= 1 && p <= 65535; }
module.exports = { isPortValid };

// main.js
const { isPortValid } = require("./checks");
```

The modern standard is **ES modules** (`export function ...` and `import { isPortValid } from "./checks.js"`), used in browsers, TypeScript and newer Node projects (with `"type": "module"` in package.json). You'll see both.

## Command-line tools

```javascript
const args = process.argv.slice(2);    // [node, script, ...your arguments]
console.log("arguments:", args);
console.log("platform:", process.platform, "node", process.version);
console.log("HOME set:", Boolean(process.env.HOME || process.env.USERPROFILE));
```

`process.exit(1)` ends with an error code; `process.stdout.write` prints without a newline; `node:util`'s `parseArgs` parses `--flags` properly.

## npm: the package ecosystem

```bash,norun
npm init -y                 # create package.json
npm install express         # add a dependency (into node_modules/, recorded in package.json)
npm install -D eslint       # a development-only dependency
npm run test                # run a script from package.json's "scripts"
npx eslint .                # run a tool from a package
npm audit                   # known vulnerabilities in your dependencies
```

Commit `package.json` and `package-lock.json` (exact versions), never `node_modules/`.

> ⚠ Every package you install is code you run with your permissions. Malicious and hijacked packages ("supply-chain attacks") happen regularly. Prefer well-known packages, check `npm audit`, and pin versions.

@@quiz
What does `process.argv.slice(2)` give you?
* The arguments the user passed to your script
- The Node.js version
- The first two arguments
- The environment variables
? `argv[0]` is node itself and `argv[1]` your script, so real arguments start at index 2.

@@quiz
Which files belong in Git for a Node project?
* `package.json` and `package-lock.json`, not `node_modules/`
- everything, including `node_modules/`
- only `node_modules/`
- only `package.json`
? `npm install` (or `npm ci`) recreates node_modules from the lock file.

@@quiz
What is `package-lock.json` for?
* It records the exact versions installed, so every install gets the same dependencies
- It stops other people editing package.json
- It stores your npm password
- It lists files to ignore in Git
? Commit it, and use npm ci in CI to install exactly those versions.

@@flashcards
require / module.exports :: CommonJS modules (classic Node)
import / export :: ES modules (the modern standard)
fs.readFileSync / writeFileSync :: Read / write a whole file
fs/promises :: Async file functions for await
path.join :: Build file paths safely on any OS
process.argv :: Command-line arguments
npm install :: Add a package to the project
package-lock.json :: Exact dependency versions; commit it
npm audit :: Check dependencies for known vulnerabilities

@@exercise Inventory file
Write `addServer(file, name)`: read the JSON array in `file` (treat a **missing** file as an empty array), add `name` if it isn't already there, keep the list **sorted**, write it back (pretty-printed with 2 spaces), and return the new list.
@@starter
const fs = require("node:fs");

function addServer(file, name) {
  throw new Error("TODO: not done yet");
}

@@tests
const path = require("node:path");
const os = require("node:os");
const dir = fs.mkdtempSync(path.join(os.tmpdir(), "itq-inv-"));
const f = path.join(dir, "inventory.json");
check(addServer(f, "web02"), ["web02"], "first server, missing file");
check(addServer(f, "db01"), ["db01", "web02"], "second server, sorted");
check(addServer(f, "web02"), ["db01", "web02"], "no duplicates");
check(fs.readFileSync(f, "utf8"), JSON.stringify(["db01", "web02"], null, 2), "the file is pretty-printed JSON");
fs.rmSync(dir, { recursive: true });
@@hint
`const list = fs.existsSync(file) ? JSON.parse(fs.readFileSync(file, "utf8")) : [];`, then `if (!list.includes(name)) list.push(name);`, `list.sort()`, and `fs.writeFileSync(file, JSON.stringify(list, null, 2))`.
@@solution
const fs = require("node:fs");

function addServer(file, name) {
  const list = fs.existsSync(file) ? JSON.parse(fs.readFileSync(file, "utf8")) : [];
  if (!list.includes(name)) list.push(name);
  list.sort();
  fs.writeFileSync(file, JSON.stringify(list, null, 2));
  return list;
}

@@exercise Parse flags
Write `parseFlags(args)` for a CLI: `--name value` pairs become properties, a `--flag` followed by another flag (or nothing) becomes `true`, and everything else is collected in `_`. `["scan", "--host", "web01", "--verbose", "extra"]` gives `{ _: ["scan", "extra"], host: "web01", verbose: true }`.
@@starter
function parseFlags(args) {
  throw new Error("TODO: not done yet");
}

@@tests
check(parseFlags(["scan", "--host", "web01", "--verbose", "--port", "22"]), { _: ["scan"], host: "web01", verbose: true, port: "22" }, "flags with values and a switch");
check(parseFlags(["a", "b"]), { _: ["a", "b"] }, "no flags");
check(parseFlags(["--dry-run"]), { _: [], "dry-run": true }, "a flag at the end");
@@hint
Loop with an index. When `args[i].startsWith("--")`: the key is `args[i].slice(2)`; if the next item exists and doesn't start with `--`, it's the value (and skip it with `i++`), else the value is `true`.
@@solution
function parseFlags(args) {
  const result = { _: [] };
  for (let i = 0; i < args.length; i++) {
    const a = args[i];
    if (a.startsWith("--")) {
      const next = args[i + 1];
      if (next !== undefined && !next.startsWith("--")) {
        result[a.slice(2)] = next;
        i++;
      } else {
        result[a.slice(2)] = true;
      }
    } else {
      result._.push(a);
    }
  }
  return result;
}

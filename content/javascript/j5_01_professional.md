@@lesson j5-professional Professional JavaScript: Tests, TypeScript, Security
@@concepts testing
@@text
## Testing with node:test

Node has a test runner built in. Put tests in `*.test.js` files and run `node --test`:

```javascript,norun
// tickets.test.js
const test = require("node:test");
const assert = require("node:assert/strict");
const { validateTicket } = require("./tickets");

test("a valid ticket has no errors", () => {
  assert.deepEqual(validateTicket({ title: "VPN down" }), []);
});

test("a blank title is rejected", () => {
  assert.deepEqual(validateTicket({ title: " " }), ["title is required"]);
});
```

`assert.equal` / `deepEqual` compare values, `assert.throws` / `assert.rejects` expect an error. Popular alternatives with the same shape: Jest, Vitest, Mocha.

## TypeScript

**TypeScript** is JavaScript with types, checked before the code runs. Most large JavaScript projects use it now:

```typescript
interface Ticket {
  id: number;
  title: string;
  priority?: 1 | 2 | 3 | 4;        // optional, and only these values
  status: "open" | "closed";
}

function summary(t: Ticket): string {
  return `#${t.id} ${t.title} [${t.status}]`;
}

summary({ id: 1, title: "VPN down", status: "opne" });   // error before running: "opne" isn't allowed
```

`npx tsc` checks and compiles to JavaScript. If you can read the JavaScript in this course, you can read TypeScript: it's the same code with `: type` annotations.

## Linting and formatting

**ESLint** finds bugs and bad patterns (unused variables, `==`, missing `await`); **Prettier** formats code automatically, so nobody argues about style. Both run in VS Code as you type.

## Security checklist for JavaScript developers

1. **XSS**: never put untrusted text into `innerHTML`; use `textContent` or a framework that escapes for you (React, Vue). Add a Content-Security-Policy header.
2. **Injection**: never build SQL, shell commands or file paths by gluing strings with user input. Use parameterized queries, `execFile` with an argument array (not `exec` with a string), and validate paths.
3. **Secrets**: API keys don't belong in front-end code (anyone can read it in DevTools) or in Git. Use environment variables on the server.
4. **Dependencies**: `npm audit`, few and well-known packages, lock files.
5. **CORS** is a browser rule about which *websites* may call your API from JavaScript. It is not authentication: anyone can still call your API with curl.
6. **Validate on the server**. Front-end validation is for user convenience only; it can be bypassed.

@@quiz
Your web page validates the ticket form in JavaScript. Does the server still need to validate?
* Yes: anyone can send requests directly, skipping the page
- No, the browser already checked
- Only for admins
- Only if CORS is disabled
? Client-side checks are a convenience; the server is the security boundary.

@@quiz
What does TypeScript add to JavaScript?
* Static types checked before running
- A faster runtime
- Automatic security
- A new browser
? TypeScript compiles to plain JavaScript; the types catch mistakes early.

@@quiz
Which Node call is safe with a user-supplied file name?
* `execFile("gzip", [userFile])`
- ``exec(`gzip ${userFile}`)``
- `eval(userFile)`
- `exec("gzip " + userFile)`
? With a string, the shell interprets `; rm -rf ~` in the name. An argument array is never parsed by a shell.

@@flashcards
node --test :: Run Node's built-in test runner
assert.deepEqual :: Compare arrays/objects in tests
TypeScript :: JavaScript with static types (npx tsc)
ESLint / Prettier :: Find code problems / format code automatically
CSP :: Content-Security-Policy header that limits what scripts may run (XSS defence)
execFile vs exec :: Argument array (safe) vs a shell command string (injection risk)
CORS :: Browser rule for cross-origin requests; not authentication

@@exercise Write the tests
Here's a function and a tiny `test` helper like `node:test`. Write **three tests** using `test(...)` and `assert.equal` / `assert.deepEqual`: a valid IP returns `true`, `"256.1.1.1"` returns `false`, and `splitHostPort("db01:5432")` returns `{ host: "db01", port: 5432 }`. Your test names don't matter; the helper prints how many passed.
@@starter
const assert = require("node:assert/strict");
let passed = 0;
function test(name, fn) {
  try { fn(); passed++; } catch (e) { console.log(`FAIL ${name}: ${e.message}`); }
}

function isIPv4(s) {
  const parts = s.split(".");
  return parts.length === 4 && parts.every((p) => /^\d{1,3}$/.test(p) && Number(p) <= 255);
}
function splitHostPort(s) {
  const [host, port] = s.split(":");
  return { host, port: Number(port) };
}

// your three tests here

console.log(`${passed} passed`);
@@expected
3 passed
@@requires
assert.equal(
assert.deepEqual(
@@hint
`test("valid ip", () => { assert.equal(isIPv4("10.0.0.4"), true); });` and the same shape for the other two (use `assert.deepEqual` for the object).
@@solution
const assert = require("node:assert/strict");
let passed = 0;
function test(name, fn) {
  try { fn(); passed++; } catch (e) { console.log(`FAIL ${name}: ${e.message}`); }
}

function isIPv4(s) {
  const parts = s.split(".");
  return parts.length === 4 && parts.every((p) => /^\d{1,3}$/.test(p) && Number(p) <= 255);
}
function splitHostPort(s) {
  const [host, port] = s.split(":");
  return { host, port: Number(port) };
}

test("a valid IPv4 address", () => {
  assert.equal(isIPv4("10.0.0.4"), true);
});
test("an octet over 255 is invalid", () => {
  assert.equal(isIPv4("256.1.1.1"), false);
});
test("host and port are split", () => {
  assert.deepEqual(splitHostPort("db01:5432"), { host: "db01", port: 5432 });
});

console.log(`${passed} passed`);

@@exercise Safe file paths
A download API serves files from a folder. Write `safeJoin(base, userPath)` that returns the full path if it stays **inside** `base`, and throws an Error with message `path escapes base` otherwise (like `../../etc/passwd`). Use `path.resolve` and compare.
@@starter
const path = require("node:path");

function safeJoin(base, userPath) {
  throw new Error("TODO: not done yet");
}

@@tests
const base = path.resolve("/srv/files");
check(safeJoin(base, "report.pdf"), path.join(base, "report.pdf"), "a normal file");
check(safeJoin(base, "2026/oct/log.txt"), path.join(base, "2026", "oct", "log.txt"), "a subfolder");
for (const bad of ["../secret.txt", "../../etc/passwd", "a/../../x", "/etc/passwd"]) {
  let msg = "no error";
  try { safeJoin(base, bad); } catch (e) { msg = e.message; }
  check(msg, "path escapes base", `safeJoin(base, ${JSON.stringify(bad)})`);
}
@@hint
`const full = path.resolve(base, userPath);` then require that `full` starts with `base + path.sep` (the `path.sep` stops `/srv/files-other` from passing).
@@solution
const path = require("node:path");

function safeJoin(base, userPath) {
  const root = path.resolve(base);
  const full = path.resolve(root, userPath);
  if (!full.startsWith(root + path.sep)) throw new Error("path escapes base");
  return full;
}

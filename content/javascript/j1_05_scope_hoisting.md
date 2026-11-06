@@lesson j1-scope Scope, Hoisting and const
@@concepts variables, functions
@@text
**Scope** decides where a variable can be seen. JavaScript has three ways to declare variables, and they behave differently.

| | `let` | `const` | `var` (old) |
|---|---|---|---|
| Scope | block `{ }` | block `{ }` | whole function |
| Reassign | yes | no | yes |
| Use before declaration | error (TDZ) | error (TDZ) | `undefined` |
| Redeclare in same scope | error | error | allowed |

**Rule of thumb:** use `const` by default, `let` when you must reassign, and never `var` in new code.

## Block scope

```javascript
const host = "web01";
if (true) {
  const host = "db01";          // a different variable that shadows the outer one
  let count = 1;
  console.log("inside:", host, count);
}
console.log("outside:", host, typeof count);   // count doesn't exist here
```

## const means the binding can't change, not the value

```javascript
const servers = ["web01"];
servers.push("db01");           // fine: the array itself is mutable
console.log(servers);
try {
  servers = [];                 // error: can't point the name at a new value
} catch (e) {
  console.log(e.name, e.message);
}
const frozen = Object.freeze({ port: 443 });
frozen.port = 80;               // silently ignored (an error in strict mode)
console.log(frozen.port);
```

`Object.freeze` makes an object shallowly immutable: nested objects can still change.

## Hoisting and the temporal dead zone

Declarations are processed before the code in their scope runs (**hoisting**), but they behave differently:

```javascript
console.log(typeof legacy);    // "undefined": var is hoisted and set to undefined
var legacy = 1;

console.log(add(2, 3));        // 5: function declarations are fully hoisted
function add(a, b) { return a + b; }

try {
  console.log(modern);         // ReferenceError: still in the temporal dead zone
} catch (e) {
  console.log(e.name);
}
let modern = 2;
```

The **temporal dead zone** (TDZ) is the time between entering a scope and reaching the `let`/`const` line. Using the variable there throws, which catches bugs that `var` would hide.

## The classic loop bug

```javascript
const withVar = [];
for (var i = 0; i < 3; i++) withVar.push(() => i);
const withLet = [];
for (let j = 0; j < 3; j++) withLet.push(() => j);
console.log(withVar.map((f) => f()), withLet.map((f) => f()));   // [3,3,3] [0,1,2]
```

`var` has one `i` for the whole function, so every closure sees its final value. `let` creates a **new binding per iteration**.

## Global scope

- In a browser script, top-level `var` and function declarations become properties of `window`. In modules and in Node.js files, top-level variables stay private to the file.
- `globalThis` is the global object in every runtime (`window` in browsers, `global` in Node.js).
- Avoid globals: two scripts using the same global name will overwrite each other.

## Closures capture variables, not values

A closure keeps the **variable** alive, so it sees later changes:

```javascript
let level = "info";
const log = (msg) => `[${level}] ${msg}`;
level = "debug";
console.log(log("started"));    // [debug] started
```

@@quiz
Which declaration should you use by default?
* `const`
- `let`
- `var`
- none, assign directly
? Use `let` only when the variable must be reassigned.

@@quiz
`const list = [1]; list.push(2);` What happens?
- TypeError, list is const
* It works; const stops reassignment, not changes to the array
- list becomes undefined
- A new array is created
? Use `Object.freeze` for (shallow) immutability.

@@quiz
What happens when you read a `let` variable before its declaration line?
- You get `undefined`
* A ReferenceError, because it's in the temporal dead zone
- You get `null`
- It works if the variable is a number
? `var` would give `undefined`, which hides bugs.

@@drill
What's the name of the period before a `let` variable's declaration, when using it throws?
= temporal dead zone
= tdz
? Variables declared with var are undefined there instead.

@@flashcards
Block scope :: `let` and `const` exist only inside their `{ }`
Function scope :: `var` exists in the whole function
Hoisting :: declarations are processed before the code runs
TDZ :: temporal dead zone; reading `let`/`const` before declaration throws
Shadowing :: an inner variable with the same name hides the outer one
`Object.freeze` :: makes an object's own properties read-only (shallow)
`globalThis` :: the global object in any runtime
Closure :: a function plus the variables it captured

@@exercise Fix the delayed messages
`scheduleAll(hosts)` should return an array of functions; calling the `n`th function returns `"checking <host n> (<n+1> of <total>)"`. The starter code returns functions that all report the wrong host. Rewrite it so each function remembers its own host and position.
@@starter
function scheduleAll(hosts) {
  var jobs = [];
  for (var i = 0; i < hosts.length; i++) {
    jobs.push(function () {
      return "checking " + hosts[i] + " (" + (i + 1) + " of " + hosts.length + ")";
    });
  }
  throw new Error("TODO: fix the var bug, then return jobs");
}

@@tests
const jobs = scheduleAll(["web01", "db01", "mail01"]);
check(jobs.map((f) => f()), ["checking web01 (1 of 3)", "checking db01 (2 of 3)", "checking mail01 (3 of 3)"], "each job remembers its host");
check(scheduleAll([]), [], "no hosts");
@@hint
Change both `var`s to `let`/`const` so the loop gets a fresh `i` per iteration, remove the `throw`, and return `jobs`. Or use `hosts.map((host, i) => () => ...)`.
@@solution
function scheduleAll(hosts) {
  return hosts.map((host, i) => () => `checking ${host} (${i + 1} of ${hosts.length})`);
}

@@exercise Deep freeze a config
`Object.freeze` is shallow. Write `deepFreeze(obj)` that freezes an object and, recursively, every object or array inside it, and returns the same object. Then write `isDeepFrozen(obj)` that returns true only if the object and everything inside it is frozen. Primitive values count as frozen.
@@starter
function deepFreeze(obj) {
  throw new Error("TODO: not done yet");
}

function isDeepFrozen(obj) {
  throw new Error("TODO: not done yet");
}

@@tests
const config = { db: { host: "db01", ports: [5432, 5433] }, debug: false };
const shallow = Object.freeze({ db: { host: "x" } });
check(isDeepFrozen(shallow), false, "shallow freeze is not deep");
check(deepFreeze(config) === config, true, "returns the same object");
check(isDeepFrozen(config), true, "deep frozen");
try { config.db.ports.push(1); } catch (e) {}
check(config.db.ports, [5432, 5433], "nested array unchanged");
check(isDeepFrozen(42), true, "a number");
@@hint
`for (const value of Object.values(obj)) if (typeof value === "object" && value !== null) deepFreeze(value);` then `return Object.freeze(obj);`. `Object.isFrozen(x)` tests one level.
@@solution
function deepFreeze(obj) {
  for (const value of Object.values(obj)) {
    if (typeof value === "object" && value !== null) deepFreeze(value);
  }
  return Object.freeze(obj);
}

function isDeepFrozen(obj) {
  if (typeof obj !== "object" || obj === null) return true;
  return Object.isFrozen(obj) && Object.values(obj).every(isDeepFrozen);
}

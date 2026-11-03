@@lesson j2-destructuring Destructuring, Spread and Copying Data
@@concepts maps, lists, variables
@@text
Modern JavaScript code is full of `{ a, b } = obj`, `[first, ...rest]` and `{ ...defaults, ...options }`. This lesson covers that syntax and the question behind it: **when do two variables share the same object?**

## Destructuring

```javascript
const server = { name: "web01", ip: "10.0.0.5", tags: ["prod", "eu"], os: { family: "linux" } };

const { name, ip: address, port = 22 } = server;          // rename and default
const { os: { family } } = server;                         // nested
const [primaryTag, ...otherTags] = server.tags;            // arrays by position
console.log(name, address, port, family, primaryTag, otherTags);

let a = 1, b = 2;
[a, b] = [b, a];                                           // swap
console.log(a, b);
```

In function parameters, destructuring gives you named options with defaults:

```javascript
function connect({ host, port = 443, timeout = 5000 } = {}) {
  return `${host}:${port} (timeout ${timeout} ms)`;
}
console.log(connect({ host: "api.local" }), connect({ host: "db", port: 5432 }));
```

The `= {}` at the end lets callers omit the argument entirely.

## Spread and rest

```javascript
const defaults = { retries: 3, timeout: 5000, verbose: false };
const options = { timeout: 1000 };
const merged = { ...defaults, ...options };          // later keys win
const { verbose, ...withoutVerbose } = merged;       // rest: everything else
console.log(merged, withoutVerbose);
console.log([..."abc"], Math.max(...[3, 9, 4]), [...new Set([1, 1, 2])]);
```

## References: the big idea

Objects and arrays are held **by reference**. Assigning one to another variable copies the reference, not the data:

```javascript
const original = { host: "web01", tags: ["prod"] };
const alias = original;
alias.host = "web02";
console.log(original.host);              // web02: same object

const shallow = { ...original };
shallow.host = "web03";                  // top level: independent
shallow.tags.push("eu");                 // nested array: still shared!
console.log(original.host, original.tags);

const deep = structuredClone(original);
deep.tags.push("us");
console.log(original.tags, deep.tags);
```

| Copy | How | Nested objects |
|---|---|---|
| Alias | `b = a` | same object |
| Shallow | `{ ...a }`, `[...a]`, `Object.assign({}, a)`, `arr.slice()` | **shared** |
| Deep | `structuredClone(a)` | copied (also handles Dates, Maps, Sets, cycles; not functions or class instances) |

`JSON.parse(JSON.stringify(x))` is the old deep-copy trick; it loses Dates (become strings), `undefined`, Maps and Sets.

## Equality compares identity

```javascript
console.log({ a: 1 } === { a: 1 }, [1] === [1]);   // false false: different objects
const x = [1];
console.log(x === x);
```

To compare contents, write a function (the exercise does) or compare `JSON.stringify` output for simple data.

## Immutable updates

Front-end frameworks (React, Redux) detect changes by **identity**, so you update data by creating new objects rather than mutating:

```javascript
const state = { user: { name: "Ana", roles: ["user"] }, theme: "dark" };
const next = { ...state, user: { ...state.user, roles: [...state.user.roles, "admin"] } };
console.log(state.user.roles, next.user.roles, state.theme === next.theme);
```

Only the changed path is copied; everything else is shared, which is cheap.

Array methods that **don't mutate**: `map`, `filter`, `slice`, `concat`, and the newer `toSorted`, `toReversed`, `toSpliced`, `with(index, value)`. Methods that **do**: `push`, `pop`, `shift`, `unshift`, `splice`, `sort`, `reverse`, `fill`.

@@quiz
`const b = { ...a };` Then `b.list.push(1)`. What happens to `a.list`?
* It changes too, because spread makes a shallow copy and the nested array is shared
- Nothing; spread copies deeply
- An error is thrown
- a.list becomes undefined
? Use `structuredClone` for a deep copy.

@@quiz
What does `{ ...defaults, ...options }` produce when both have `timeout`?
- defaults' timeout
* options' timeout, because later properties overwrite earlier ones
- an array with both
- an error
? Spread order matters.

@@quiz
Which array method returns a new sorted array without changing the original?
- sort
* toSorted
- reverse
- splice
? `toSorted`, `toReversed`, `toSpliced` and `with` are the non-mutating versions.

@@flashcards
Destructuring :: unpack values: `const { a, b } = obj`, `const [x, y] = arr`
Rename while destructuring :: `const { ip: address } = server`
Rest pattern :: `const { a, ...others } = obj`
Spread :: `{ ...a, ...b }`, `[...arr]`, `f(...args)`
Shallow copy :: copies the top level; nested objects shared
`structuredClone` :: built-in deep copy
Reference equality :: `===` on objects compares identity
Immutable update :: copy the changed path instead of mutating

@@exercise Deep equality
Write `deepEqual(a, b)`: true when two values have the same structure and contents.

- primitives compare with `Object.is` (so `NaN` equals `NaN`)
- arrays: same length and equal elements in order; an array never equals a plain object
- plain objects: the same set of keys (order doesn't matter) with equal values
- `Date`s: equal when they have the same time
@@starter
function deepEqual(a, b) {
  throw new Error("TODO: not done yet");
}

@@tests
check(deepEqual({ a: [1, { b: 2 }], c: "x" }, { c: "x", a: [1, { b: 2 }] }), true, "nested, keys in other order");
check(deepEqual({ a: 1 }, { a: 1, b: undefined }), false, "extra key");
check(deepEqual([1, 2], { 0: 1, 1: 2 }), false, "array vs object");
check(deepEqual(NaN, NaN), true, "NaN");
check(deepEqual(new Date(5), new Date(5)), true, "dates");
check(deepEqual([1, [2, 3]], [1, [2, 4]]), false, "nested difference");
check(deepEqual(null, {}), false, "null vs object");
@@hint
Handle `Object.is(a, b)` first. Then both must be non-null objects. Dates: compare `getTime()`. Arrays: `Array.isArray(a) !== Array.isArray(b)` → false. Then compare `Object.keys` lengths and check every key with `Object.hasOwn(b, key) && deepEqual(a[key], b[key])`.
@@solution
function deepEqual(a, b) {
  if (Object.is(a, b)) return true;
  if (typeof a !== "object" || typeof b !== "object" || a === null || b === null) return false;
  if (a instanceof Date || b instanceof Date) {
    return a instanceof Date && b instanceof Date && a.getTime() === b.getTime();
  }
  if (Array.isArray(a) !== Array.isArray(b)) return false;
  const keysA = Object.keys(a);
  const keysB = Object.keys(b);
  if (keysA.length !== keysB.length) return false;
  return keysA.every((key) => Object.hasOwn(b, key) && deepEqual(a[key], b[key]));
}

@@exercise Immutable state updates
Write `setIn(obj, path, value)` that returns a **new** object with the value at `path` (an array of keys/indexes) replaced, copying only the objects along the path and sharing everything else. Missing objects along the path are created (as arrays when the next key is a number, otherwise objects). The original must not change.

Also write `updateIn(obj, path, fn)`, which does the same with `fn(oldValue)`.
@@starter
function setIn(obj, path, value) {
  throw new Error("TODO: not done yet");
}

function updateIn(obj, path, fn) {
  throw new Error("TODO: not done yet");
}

@@tests
const state = { user: { name: "Ana", roles: ["user"] }, settings: { theme: "dark" } };
const next = setIn(state, ["user", "roles", 1], "admin");
check(next.user.roles, ["user", "admin"], "new value");
check(state.user.roles, ["user"], "original untouched");
check(next.settings === state.settings, true, "unchanged branch shared");
check(next.user === state.user, false, "changed path copied");
check(setIn({}, ["a", 0, "b"], 1), { a: [{ b: 1 }] }, "creates missing containers");
check(updateIn(state, ["user", "name"], (n) => n.toUpperCase()).user.name, "ANA", "updateIn");
check(setIn(state, [], 5), 5, "empty path replaces everything");
@@hint
Recursive: `if (path.length === 0) return value;` Let `[key, ...rest] = path`. The current container is `obj ?? (typeof key === "number" ? [] : {})`. Copy it (`Array.isArray(c) ? [...c] : { ...c }`), set `copy[key] = setIn(c[key], rest, value)`, return the copy. `updateIn` can read the old value by walking the path, then call `setIn`.
@@solution
function setIn(obj, path, value) {
  if (path.length === 0) return value;
  const [key, ...rest] = path;
  const current = obj ?? (typeof key === "number" ? [] : {});
  const copy = Array.isArray(current) ? [...current] : { ...current };
  const nextKey = rest[0];
  const child = current[key] ?? (rest.length === 0 ? undefined : typeof nextKey === "number" ? [] : {});
  copy[key] = setIn(child, rest, value);
  return copy;
}

function updateIn(obj, path, fn) {
  const old = path.reduce((node, key) => (node == null ? undefined : node[key]), obj);
  return setIn(obj, path, fn(old));
}

@@lesson j1-types Types, Truthiness and Coercion
@@concepts variables, conditions
@@text
JavaScript has seven **primitive** types and one structural type, **object**. Most surprising bugs in JavaScript come from mixing them up, so this lesson looks at them closely.

| Type | Examples | `typeof` |
|---|---|---|
| number | `42`, `3.14`, `NaN`, `Infinity` | `"number"` |
| bigint | `9007199254740993n` | `"bigint"` |
| string | `"web01"`, `` `x=${x}` `` | `"string"` |
| boolean | `true`, `false` | `"boolean"` |
| undefined | a variable with no value | `"undefined"` |
| null | "deliberately nothing" | `"object"` (a historic bug) |
| symbol | `Symbol("id")` | `"symbol"` |
| object | `{}`, `[]`, functions, dates, maps | `"object"` / `"function"` |

```javascript
console.log(typeof 42, typeof "a", typeof undefined, typeof null, typeof [], typeof (() => 1));
console.log(Array.isArray([]), null === null, Number.isNaN(NaN));
```

Use `Array.isArray` for arrays and `x === null` for null, because `typeof` reports `"object"` for both.

## Numbers are floating point

All `number`s are 64-bit floats, like Python's `float`:

```javascript
console.log(0.1 + 0.2, 0.1 + 0.2 === 0.3, Math.abs(0.1 + 0.2 - 0.3) < Number.EPSILON);
console.log(Number.MAX_SAFE_INTEGER, 2 ** 53 + 1);      // integers above 2^53 lose precision
console.log(2n ** 64n);                                  // bigint: exact large integers
console.log(1 / 0, -1 / 0, 0 / 0);                       // Infinity -Infinity NaN
```

For money, store whole cents as integers. For IDs from other systems that exceed 2^53 (Twitter/X IDs, some database keys), keep them as strings or use `BigInt`.

## Truthy and falsy

In an `if`, every value counts as true except the **falsy** ones: `false`, `0`, `-0`, `0n`, `""`, `null`, `undefined` and `NaN`. Everything else is truthy, **including** `"0"`, `"false"`, `[]` and `{}`.

```javascript
const port = 0;
console.log(port || 8080);     // 8080: || treats 0 as missing (a bug here!)
console.log(port ?? 8080);     // 0:    ?? only replaces null and undefined
const user = { profile: null };
console.log(user.profile?.name ?? "anonymous");   // optional chaining stops at null
```

Use `??` for defaults and `?.` for "might be missing" chains.

## == versus ===

`===` (strict equality) compares type and value. `==` converts types first, with rules almost nobody remembers:

```javascript
console.log(0 == "", "1" == 1, null == undefined, [] == false, "0" == false);
console.log(0 === "", "1" === 1, null === undefined);
```

**Always use `===` and `!==`.** The one common exception is `x == null`, which checks for both null and undefined.

## Explicit conversion

Convert on purpose, at the edges of your program (input, files, query strings):

```javascript
console.log(Number("42"), Number(""), Number("42px"), parseInt("42px", 10), parseFloat("3.5GB"));
console.log(String(42), (255).toString(16), Boolean(""), Boolean("no"));
console.log(`${[1, 2]}`, JSON.stringify([1, 2]), [10, 9, 1].sort());
```

- `Number(x)` is strict (`"42px"` → `NaN`) but turns `""` into `0`; `parseInt` reads digits until it can't.
- `[10, 9, 1].sort()` gives `[1, 10, 9]`: the default sort compares **as strings**. Always pass a compare function for numbers: `sort((a, b) => a - b)`.
- `+` with a string concatenates: `"5" + 1` is `"51"`, but `"5" - 1` is `4`.

@@quiz
Which value is truthy?
- `0`
- `""`
* `"0"`
- `null`
? Non-empty strings are always truthy, even `"0"` and `"false"`.

@@quiz
`const retries = 0; const n = retries || 3;` What is `n`, and how do you fix it?
- 0; nothing to fix
* 3, because `||` treats 0 as missing; use `retries ?? 3`
- undefined; use `?.`
- an error
? `??` only replaces `null` and `undefined`.

@@quiz
What does `[10, 9, 1].sort()` return?
- `[1, 9, 10]`
* `[1, 10, 9]`, because the default sort compares strings
- `[10, 9, 1]`
- an error
? Pass `(a, b) => a - b` to sort numbers.

@@quiz
What does `typeof null` return?
- `"null"`
* `"object"`
- `"undefined"`
- `"none"`
? A bug from 1995 that can't be fixed without breaking the web.

@@drill
What does `0.1 + 0.2 === 0.3` evaluate to?
= false
? Floating-point numbers can't store 0.1 exactly; compare with a small tolerance.

@@flashcards
Primitive types :: number, bigint, string, boolean, undefined, null, symbol
Falsy values :: false, 0, -0, 0n, "", null, undefined, NaN
`===` :: strict equality, no type conversion
`??` :: default only for null or undefined
`?.` :: optional chaining; stops at null or undefined
`Number.isNaN` :: the reliable NaN check
`Number.MAX_SAFE_INTEGER` :: 2^53 − 1, largest exact integer
BigInt :: exact integers of any size, written `123n`
Default sort :: compares as strings; pass a compare function for numbers

@@exercise Parse settings safely
Settings arrive as strings (from environment variables or a query string). Write `parseSetting(value, kind, fallback)`:

- `value` `undefined`, `null` or a string of only spaces → `fallback`
- `kind` `"int"`: the trimmed text must be an optional `-` and digits only, else `fallback` (so `"8080"` → 8080, `"80px"` and `"1.5"` → fallback); `"0"` → 0, not the fallback
- `kind` `"bool"`: `"true"`, `"yes"`, `"1"`, `"on"` → true; `"false"`, `"no"`, `"0"`, `"off"` → false (any case); anything else → fallback
- `kind` `"list"`: split on commas, trim each item, drop empty ones
@@starter
function parseSetting(value, kind, fallback) {
  throw new Error("TODO: not done yet");
}

@@tests
check(parseSetting("8080", "int", 80), 8080, "an int");
check(parseSetting("0", "int", 5), 0, "zero is a real value");
check(parseSetting("80px", "int", 80), 80, "junk int");
check(parseSetting(" -3 ", "int", 1), -3, "negative with spaces");
check(parseSetting(undefined, "int", 7), 7, "missing");
check(parseSetting("   ", "bool", false), false, "blank");
check(parseSetting("YES", "bool", false), true, "bool yes");
check(parseSetting("off", "bool", true), false, "bool off");
check(parseSetting("maybe", "bool", true), true, "bool junk");
check(parseSetting("web01, db01,,  ", "list", []), ["web01", "db01"], "list");
@@hint
Start with `if (value == null || value.trim() === "") return fallback;` For ints test `/^-?\d+$/.test(text)` before `Number(text)`.
@@solution
function parseSetting(value, kind, fallback) {
  if (value == null || value.trim() === "") return fallback;
  const text = value.trim();
  if (kind === "int") {
    return /^-?\d+$/.test(text) ? Number(text) : fallback;
  }
  if (kind === "bool") {
    const lower = text.toLowerCase();
    if (["true", "yes", "1", "on"].includes(lower)) return true;
    if (["false", "no", "0", "off"].includes(lower)) return false;
    return fallback;
  }
  if (kind === "list") {
    return text.split(",").map((s) => s.trim()).filter((s) => s !== "");
  }
  return fallback;
}

@@exercise Describe any value
Write `describe(value)` that names a value's type more precisely than `typeof`:

`"null"`, `"undefined"`, `"NaN"`, `"integer"` (a whole number, including `-0` and big-but-exact values), `"float"`, `"Infinity"` (positive or negative), `"bigint"`, `"string"`, `"boolean"`, `"symbol"`, `"array"`, `"function"`, `"date"`, `"map"`, `"set"`, `"regexp"`, and `"object"` for anything else.

Then `sortNumbers(values)` returning a **new** array of the numbers sorted ascending, without changing the input.
@@starter
function describe(value) {
  throw new Error("TODO: not done yet");
}

function sortNumbers(values) {
  throw new Error("TODO: not done yet");
}

@@tests
check([null, undefined, NaN, 42, -0, 1.5, -Infinity, 10n].map(describe), ["null", "undefined", "NaN", "integer", "integer", "float", "Infinity", "bigint"], "numbers and empties");
check(["", true, Symbol("x"), [], () => 1, new Date(0), new Map(), new Set(), /a/, {}].map(describe), ["string", "boolean", "symbol", "array", "function", "date", "map", "set", "regexp", "object"], "others");
const input = [10, 9, 1, 100];
check(sortNumbers(input), [1, 9, 10, 100], "sorted numerically");
check(input, [10, 9, 1, 100], "input untouched");
@@hint
Check `null` and arrays before using `typeof`. For numbers: `Number.isNaN`, then `!Number.isFinite` (Infinity), then `Number.isInteger`. For objects, `instanceof Date`, `Map`, `Set`, `RegExp`. Copy before sorting: `[...values].sort((a, b) => a - b)`.
@@solution
function describe(value) {
  if (value === null) return "null";
  if (Array.isArray(value)) return "array";
  const t = typeof value;
  if (t === "number") {
    if (Number.isNaN(value)) return "NaN";
    if (!Number.isFinite(value)) return "Infinity";
    return Number.isInteger(value) ? "integer" : "float";
  }
  if (t !== "object") return t;
  if (value instanceof Date) return "date";
  if (value instanceof Map) return "map";
  if (value instanceof Set) return "set";
  if (value instanceof RegExp) return "regexp";
  return "object";
}

function sortNumbers(values) {
  return [...values].sort((a, b) => a - b);
}

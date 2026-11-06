@@lesson j1-control Decisions and Loops
@@concepts conditions, loops
@@text
## if / else if / else

```javascript
const cpu = 91;
if (cpu >= 95) {
  console.log("CRITICAL");
} else if (cpu >= 80) {
  console.log("WARNING");
} else {
  console.log("OK");
}
```

Combine with `&&` (and), `||` (or), `!` (not). The **ternary** is a one-line if/else for values: `const level = cpu > 80 ? "high" : "normal";`

## switch

```javascript
const code = 404;
switch (code) {
  case 200: console.log("OK"); break;
  case 301:
  case 302: console.log("Redirect"); break;
  case 404: console.log("Not found"); break;
  default:  console.log("Other");
}
```

Don't forget `break`, or execution "falls through" into the next case (sometimes on purpose, like 301/302 above).

## Loops

```javascript
for (let i = 1; i <= 3; i++) {
  console.log(`attempt ${i}`);
}

const servers = ["web01", "web02", "db01"];
for (const s of servers) {        // for...of: the values
  console.log(`checking ${s}`);
}

let tries = 0;
while (tries < 2) {
  tries++;
  console.log(`ping ${tries}`);
}
```

`for...of` loops over **values** (arrays, strings). There's also `for...in`, which loops over an object's **keys**; don't use it on arrays. `break` and `continue` work as in other languages.
## Guard clauses

Deeply nested `if` statements are hard to read. A common professional style is to handle the special cases **first** and return early, so the main logic isn't indented:

```javascript
function diskStatus(freeGb) {
  if (freeGb === undefined) return "unknown";
  if (freeGb < 5) return "critical";
  if (freeGb < 20) return "low";
  return "ok";
}
console.log(diskStatus(3), diskStatus(12), diskStatus(80), diskStatus());
```

Each check reads like a rule, and the order matters: the most specific or most severe condition comes first.

## Loops that stop at the right time

A `while` loop needs something inside it that eventually makes the condition false, or it runs forever. When you loop over an array, prefer `for...of` (or array methods like `forEach`, `map` and `filter`, covered in the arrays lesson) to counting indexes by hand: there's no off-by-one error to make.

@@quiz
Which loop gives the **values** of an array?
* `for (const x of arr)`
- `for (const x in arr)`
- `for (x = arr)`
- `arr.loop(x)`
? `for...of` gives values; `for...in` gives keys (indexes, as strings, for arrays).

@@quiz
What happens in a `switch` case without `break`?
* Execution continues into the next case
- A syntax error
- The switch ends anyway
- The default case runs
? It "falls through". Useful for grouping cases, a bug when forgotten.

@@quiz
What does `const level = cpu > 80 ? "high" : "normal";` set when `cpu` is 80?
* `"normal"`
- `"high"`
- `undefined`
- an error
? 80 > 80 is false, so the value after the colon is used.

@@flashcards
if / else if / else :: choose one branch by condition
ternary :: condition ? valueIfTrue : valueIfFalse
switch :: compare one value against cases; remember break
for...of :: loop over values (arrays, strings)
for...in :: loop over object keys (not for arrays)
break / continue :: leave the loop / skip to the next iteration
guard clause :: return early for special cases

@@exercise HTTP status class
Write `statusClass(code)` returning `"success"` for 200–299, `"redirect"` for 300–399, `"client error"` for 400–499, `"server error"` for 500–599 and `"invalid"` otherwise.
@@starter
function statusClass(code) {
  throw new Error("TODO: not done yet");
}

@@tests
check(statusClass(200), "success", "statusClass(200)");
check(statusClass(301), "redirect", "statusClass(301)");
check(statusClass(404), "client error", "statusClass(404)");
check(statusClass(503), "server error", "statusClass(503)");
check(statusClass(99), "invalid", "statusClass(99)");
check(statusClass(600), "invalid", "statusClass(600)");
@@hint
`if (code >= 200 && code < 300) return "success";` and so on, ending with `return "invalid";`.
@@solution
function statusClass(code) {
  if (code >= 200 && code < 300) return "success";
  if (code >= 300 && code < 400) return "redirect";
  if (code >= 400 && code < 500) return "client error";
  if (code >= 500 && code < 600) return "server error";
  return "invalid";
}

@@exercise Retry schedule
Exponential backoff: wait 1 s, then 2, 4, 8... up to a cap. Print the waits for 6 attempts with a cap of 10 seconds, as `attempt N: wait X s`.
@@starter
const attempts = 6;
const cap = 10;

@@expected
attempt 1: wait 1 s
attempt 2: wait 2 s
attempt 3: wait 4 s
attempt 4: wait 8 s
attempt 5: wait 10 s
attempt 6: wait 10 s
@@hint
In a loop, the wait is `Math.min(2 ** (i - 1), cap)`.
@@solution
const attempts = 6;
const cap = 10;
for (let i = 1; i <= attempts; i++) {
  console.log(`attempt ${i}: wait ${Math.min(2 ** (i - 1), cap)} s`);
}

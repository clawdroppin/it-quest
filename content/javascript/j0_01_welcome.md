@@lesson j0-welcome Welcome to JavaScript
@@concepts hello, variables
@@text
**JavaScript** runs every website you've ever used. With **Node.js** it also runs servers, command-line tools, and desktop apps (VS Code, Teams and this app's interface are built with web technology). It's the most used programming language in the world, and it's everywhere in IT: web admin panels, monitoring dashboards, automation with APIs, cloud functions.

This course runs JavaScript with **Node.js**, so you'll see the output right here. In a browser, press **F12** and open the **Console** tab: that's a JavaScript prompt too.

## First program

```javascript
console.log("Hello, IT world!");
console.log("2 + 3 =", 2 + 3);
```

`console.log` prints its arguments, separated by spaces. Lines usually end with `;` (JavaScript adds missing ones for you, but professionals write them).

## Variables: const and let

```javascript
const server = "web01";   // const: can't be reassigned
let attempts = 0;         // let: can change
attempts = attempts + 1;
attempts++;
console.log(server, attempts);
```

Use **`const` by default** and `let` only when the value changes. You'll see the old `var` in older code; avoid it (its scoping rules are confusing).

## Template literals

Backticks make strings with `${...}` placeholders, which can hold any expression:

```javascript
const host = "db01";
const freeGB = 38;
console.log(`${host} has ${freeGB} GB free (${freeGB < 50 ? "low!" : "ok"})`);
```

## Comments

```javascript
// a comment to the end of the line
/* a comment
   over several lines */
```

@@quiz
Which declaration should you use for a value that never gets reassigned?
* `const`
- `let`
- `var`
- none, just write the name
? `const` by default; `let` when it changes; avoid `var`.

@@quiz
What does `` `${2 + 2} servers` `` produce?
* `4 servers`
- `${2 + 2} servers`
- `2 + 2 servers`
- an error
? Inside backticks, `${ }` evaluates the expression.

@@flashcards
console.log :: Print values to the console
const :: A variable that can't be reassigned
let :: A variable that can be reassigned
template literal :: `Text with ${expression}` in backticks
Node.js :: Runs JavaScript outside the browser: servers, tools, scripts
F12 :: Opens a browser's developer tools and JavaScript console

@@exercise Hello, Node
Print exactly `Hello from Node.js` and on the next line `Ready to build`.
@@starter
// two console.log calls

@@expected
Hello from Node.js
Ready to build
@@hint
`console.log("Hello from Node.js");` then the second line.
@@solution
console.log("Hello from Node.js");
console.log("Ready to build");

@@exercise A status line
Using a template literal and the given constants, print `web01: 3 of 4 checks passed (75%)`. Compute the percentage, don't type it.
@@starter
const host = "web01";
const passed = 3;
const total = 4;

@@expected
web01: 3 of 4 checks passed (75%)
@@requires
`
@@hint
`` console.log(`${host}: ${passed} of ${total} checks passed (${passed / total * 100}%)`); ``
@@solution
const host = "web01";
const passed = 3;
const total = 4;
console.log(`${host}: ${passed} of ${total} checks passed (${passed / total * 100}%)`);

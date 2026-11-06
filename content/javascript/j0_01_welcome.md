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
## Reading error messages

Mistakes are part of programming, and JavaScript tells you what went wrong if you read the message carefully:

```javascript,norun
console.log(servr);
// ReferenceError: servr is not defined
```

- **ReferenceError**: you used a name that doesn't exist, usually a typo.
- **TypeError**: you did something a value can't do, like calling `undefined()` or reading a property of `null`.
- **SyntaxError**: the code isn't valid JavaScript at all: a missing bracket or quote. Nothing runs until it's fixed.

The message includes a **line number**. Start there, but remember the real mistake is sometimes a line or two earlier (a bracket that was never closed). When you're stuck, `console.log` the values just before the failing line: most bugs are a value that isn't what you expected.

## Semicolons, case and style

JavaScript is **case-sensitive**: `Server` and `server` are different names, and `Console.log` doesn't exist. Names use **camelCase** (`freeDiskGb`, `maxRetries`), constants that never change are sometimes written `UPPER_CASE`, and classes use `PascalCase`. Indent with two spaces, which is the common style in JavaScript projects, and let a formatter such as Prettier keep it consistent.

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

@@quiz
`console.log(Total)` fails with "ReferenceError: Total is not defined", but the code has `const total = 5;`. Why?
* JavaScript is case-sensitive, so `Total` and `total` are different names
- `const` values can't be printed
- console.log only prints strings
- The variable must be declared with `var`
? Check spelling and capitals first when you see a ReferenceError.

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

@@lesson j1-functions Functions, Arrow Functions and Closures
@@concepts functions
@@text
```javascript
function greet(name, greeting = "Hello") {
  return `${greeting}, ${name}!`;
}
console.log(greet("Ana"));
console.log(greet("Kenji", "Konnichiwa"));
```

## Arrow functions

A shorter syntax, used everywhere in modern JavaScript, especially for small functions passed to other functions:

```javascript
const square = (n) => n * n;               // one expression: returned automatically
const add = (a, b) => {
  const sum = a + b;
  return sum;                              // with braces, return is needed
};
console.log(square(5), add(2, 3));
```

## Functions are values

You can store functions in variables, pass them to other functions, and return them:

```javascript
function retry(task, times) {
  for (let i = 1; i <= times; i++) {
    if (task(i)) return `succeeded on attempt ${i}`;
  }
  return "gave up";
}
console.log(retry((attempt) => attempt === 3, 5));
```

## Closures

A function remembers the variables around it when it was created. That's a **closure**, and it's how JavaScript keeps private state:

```javascript
function makeCounter() {
  let count = 0;
  return () => ++count;
}
const nextTicket = makeCounter();
console.log(nextTicket(), nextTicket(), nextTicket());   // 1 2 3
```

## Rest parameters and spread

```javascript
function maxLoad(...loads) {           // collects all arguments into an array
  return Math.max(...loads);           // spreads an array into arguments
}
console.log(maxLoad(0.4, 2.1, 1.7));
```
## Pure functions

A **pure** function returns a result based only on its arguments and changes nothing outside itself. `square(5)` is always 25 and touches nothing else. Pure functions are easy to test and reuse, so put your logic in them and keep the parts that print, read files or call the network separate and small.

```javascript
// pure: the same input always gives the same output
const toGb = (bytes) => bytes / 1024 ** 3;

// not pure: depends on and changes outside state
let total = 0;
function addToTotal(n) { total += n; return total; }

console.log(toGb(5 * 1024 ** 3), addToTotal(2), addToTotal(2));
```

Calling `addToTotal(2)` twice gives different answers, which is exactly what makes code like that hard to debug when it's spread across a program.

@@quiz
What does `const double = (x) => x * 2;` return for `double(4)`?
* 8
- undefined, there is no `return`
- an error
- `x * 2`
? An arrow function with an expression body (no braces) returns that expression.

@@quiz
What is a closure?
* A function together with the variables it remembers from where it was created
- A function that closes a file
- The last line of a function
- A function with no parameters
? Closures let `makeCounter` keep `count` private between calls.

@@quiz
With `function greet(name, greeting = "Hello")` from above, what does `greet("Ana")` return?
* `"Hello, Ana!"`
- `"undefined, Ana!"`
- an error, because `greeting` is missing
- `"Ana, Hello!"`
? Default parameters are used when the argument isn't passed.

@@flashcards
function :: a named, reusable block of code
default parameter :: function f(x = 10): used when x isn't passed
arrow function :: (a, b) => a + b
expression body :: an arrow without braces returns its expression
closure :: a function plus the variables it remembers
rest parameter :: ...args collects arguments into an array
spread :: ...array expands an array into arguments
pure function :: same input, same output, no side effects

@@exercise Default parameters
Write `formatAlert(host, message, level = "WARN")` that returns `[LEVEL] host: message`.
@@starter
function formatAlert(host, message, level) {
  throw new Error("TODO: not done yet");
}

@@tests
check(formatAlert("db01", "disk 91%"), "[WARN] db01: disk 91%", "formatAlert without a level");
check(formatAlert("web01", "down", "CRIT"), "[CRIT] web01: down", "formatAlert with CRIT");
@@hint
Put the default in the parameter list: `level = "WARN"`, and return a template literal.
@@solution
function formatAlert(host, message, level = "WARN") {
  return `[${level}] ${host}: ${message}`;
}

@@exercise A rate limiter
Write `makeLimiter(max)` that returns a function. Each call of that function returns `true` while it has been called at most `max` times, and `false` after that. (Login forms use this idea to stop password guessing.) Each limiter counts separately.
@@starter
function makeLimiter(max) {
  throw new Error("TODO: not done yet");
}

@@tests
const allow = makeLimiter(3);
check([allow(), allow(), allow(), allow(), allow()], [true, true, true, false, false], "a limiter of 3, called 5 times");
const other = makeLimiter(1);
check([other(), other()], [true, false], "a second, separate limiter of 1");
@@hint
Keep a `let used = 0;` inside `makeLimiter`, and return `() => { used++; return used <= max; }`.
@@solution
function makeLimiter(max) {
  let used = 0;
  return () => {
    used++;
    return used <= max;
  };
}

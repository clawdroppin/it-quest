@@lesson l7-this `this`, call, apply and bind
@@concepts functions, objects
@@text
In most languages `self` or `this` always means "the object this method belongs to". In JavaScript, `this` is decided **when a function is called**, by **how** it's called. That one rule explains a whole family of bugs.

## The four rules

| How the function is called | `this` is |
|---|---|
| `obj.method()` | `obj` (the thing before the dot) |
| `fn()` (plain call) | `undefined` in strict mode / modules, the global object in sloppy scripts |
| `new Fn()` | the newly created object |
| `fn.call(x)`, `fn.apply(x)`, `fn.bind(x)` | `x`, explicitly |

**Arrow functions** have no `this` of their own: they use the `this` of the code around them.

```javascript
"use strict";
const monitor = {
  name: "uptime",
  report() { return `report from ${this?.name}`; },
};
console.log(monitor.report());          // "uptime": called with monitor before the dot
const detached = monitor.report;
console.log(detached());                // undefined: plain call, this is undefined
```

## Losing `this` in callbacks

Passing a method as a callback **detaches** it from its object:

```javascript
"use strict";
class Counter {
  count = 0;
  increment() { this.count++; }
}
const c = new Counter();
try {
  [1, 2, 3].forEach(c.increment);       // TypeError: this is undefined
} catch (e) {
  console.log(e.constructor.name);
}
[1, 2, 3].forEach(() => c.increment()); // fix 1: wrap in an arrow
[1, 2, 3].forEach(c.increment.bind(c)); // fix 2: bind
console.log(c.count);
```

This is the classic bug with event handlers: `button.addEventListener("click", this.save)` calls `save` with `this` set to the button, not your object.

## Class fields with arrow functions

A third fix is defining the method as an arrow-function **field**, which captures `this` once per instance:

```javascript
class Saver {
  saved = 0;
  save = () => { this.saved++; };       // a new function per instance, always bound
}
const s = new Saver();
const handler = s.save;
handler();
handler();
console.log(s.saved);
```

The trade-off: each instance gets its own function (more memory), and it isn't on the prototype, so subclasses can't call `super.save()`.

## call, apply and bind

```javascript
function describe(prefix, suffix) {
  return `${prefix} ${this.host}:${this.port}${suffix}`;
}
const db = { host: "db01", port: 5432 };
console.log(describe.call(db, "→", "!"));        // arguments listed
console.log(describe.apply(db, ["→", "?"]));      // arguments as an array
const boundDescribe = describe.bind(db, "DB");    // returns a new function, partially applied
console.log(boundDescribe("."));
```

`bind` also fixes **leading arguments** (partial application). A bound function's `this` can't be changed again, even with `call`.

## Arrow functions inside methods

Arrows are perfect **inside** methods, because they keep the method's `this`:

```javascript
const checker = {
  hosts: ["web01", "db01"],
  prefix: "checking",
  run() {
    return this.hosts.map((h) => `${this.prefix} ${h}`);   // arrow uses run's this
  },
};
console.log(checker.run());
```

But don't use an arrow **as** an object method: `{ run: () => this.hosts }` gets the outer `this` (usually `undefined`), not the object.

@@quiz
`const f = obj.method; f();` What is `this` inside `method` in strict mode?
- obj
* undefined
- the global object
- f
? `this` depends on how the function is called; a plain call has no object.

@@quiz
Why does `button.addEventListener("click", this.save)` often break?
* The browser calls `save` with `this` set to the button, so the method loses its object
- addEventListener doesn't accept methods
- save runs immediately
- Events don't support classes
? Use `() => this.save()` or `this.save.bind(this)`.

@@quiz
What's different about `this` in arrow functions?
- It's always the global object
* They don't have their own `this`; they use the surrounding scope's
- They have two
- It's always undefined
? This makes arrows ideal for callbacks inside methods.

@@flashcards
`this` rule :: decided by how a function is called, not where it's defined
Method call :: `obj.f()` → `this` is obj
Plain call :: `f()` → `this` is undefined (strict)
`new` call :: `this` is the new object
`call(thisArg, ...args)` :: call with an explicit this
`apply(thisArg, argsArray)` :: call with explicit this and an array of args
`bind(thisArg, ...args)` :: new function with this (and leading args) fixed
Arrow function this :: inherited from the surrounding scope

@@exercise Fix the event bus
The `Logger` below registers its `log` method with an `EventBus`, but when events fire, `this` is lost and lines don't get stored. Fix `Logger` (keep `EventBus` unchanged) so that `bus.emit("alert", "disk full")` stores `"[ops] alert: disk full"` in `logger.lines`. Then add `Logger.prototype.stop()`, which unsubscribes the logger (so `bus.off` must receive the **same** function that was passed to `bus.on`).
@@starter
class EventBus {
  constructor() { this.handlers = new Map(); }
  on(event, fn) { (this.handlers.get(event) ?? this.handlers.set(event, []).get(event)).push(fn); }
  off(event, fn) { this.handlers.set(event, (this.handlers.get(event) ?? []).filter((h) => h !== fn)); }
  emit(event, data) { for (const fn of this.handlers.get(event) ?? []) fn.call(null, event, data); }
}

class Logger {
  constructor(bus, prefix) {
    this.bus = bus;
    this.prefix = prefix;
    this.lines = [];
    bus.on("alert", this.log);
  }
  log(event, data) {
    this.lines.push(`[${this.prefix}] ${event}: ${data}`);
  }
  stop() {
    throw new Error("TODO: not done yet");
  }
}

@@tests
const bus = new EventBus();
const logger = new Logger(bus, "ops");
bus.emit("alert", "disk full");
check(logger.lines, ["[ops] alert: disk full"], "logs with the right this");
logger.stop();
bus.emit("alert", "cpu high");
check(logger.lines.length, 1, "stopped listening");
check(bus.handlers.get("alert").length, 0, "handler removed");
@@hint
In the constructor, store a bound version once: `this.log = this.log.bind(this);` (or `this.handler = (e, d) => this.log(e, d)`), register that, and pass the same function to `bus.off` in `stop`.
@@solution
class EventBus {
  constructor() { this.handlers = new Map(); }
  on(event, fn) { (this.handlers.get(event) ?? this.handlers.set(event, []).get(event)).push(fn); }
  off(event, fn) { this.handlers.set(event, (this.handlers.get(event) ?? []).filter((h) => h !== fn)); }
  emit(event, data) { for (const fn of this.handlers.get(event) ?? []) fn.call(null, event, data); }
}

class Logger {
  constructor(bus, prefix) {
    this.bus = bus;
    this.prefix = prefix;
    this.lines = [];
    this.log = this.log.bind(this);
    bus.on("alert", this.log);
  }
  log(event, data) {
    this.lines.push(`[${this.prefix}] ${event}: ${data}`);
  }
  stop() {
    this.bus.off("alert", this.log);
  }
}

@@exercise Write your own bind
Write `myBind(fn, thisArg, ...preset)` that behaves like `fn.bind(thisArg, ...preset)` for normal calls: the returned function calls `fn` with `this` set to `thisArg` and the preset arguments followed by the new ones, and returns its result. Use `apply` or `call`, not `bind`. The returned function should also have a `boundTarget` property pointing at `fn`.
@@starter
function myBind(fn, thisArg, ...preset) {
  throw new Error("TODO: not done yet");
}

@@tests
function ping(count, timeout) { return `${this.host} x${count} (${timeout} ms)`; }
const bound = myBind(ping, { host: "web01" }, 3);
check(bound(500), "web01 x3 (500 ms)", "preset and new args");
check(bound.call({ host: "other" }, 100), "web01 x3 (100 ms)", "this can't be overridden");
check(bound.boundTarget === ping, true, "boundTarget");
check(myBind(Math.max, null)(1, 9, 4), 9, "no preset args");
@@hint
Return a regular function: `function bound(...args) { return fn.apply(thisArg, [...preset, ...args]); }`, set `bound.boundTarget = fn`, and return it. Because you call `fn.apply(thisArg, ...)`, whatever `this` the bound function itself gets is ignored.
@@solution
function myBind(fn, thisArg, ...preset) {
  function bound(...args) {
    return fn.apply(thisArg, [...preset, ...args]);
  }
  bound.boundTarget = fn;
  return bound;
}

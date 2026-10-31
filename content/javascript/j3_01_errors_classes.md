@@lesson j3-errors-classes Errors and Classes
@@concepts errors, objects
@@text
## try / catch / finally

```javascript
function parsePort(text) {
  const port = Number(text);
  if (!Number.isInteger(port) || port < 1 || port > 65535) {
    throw new RangeError(`invalid port: ${text}`);
  }
  return port;
}

try {
  console.log(parsePort("443"));
  console.log(parsePort("http"));
} catch (err) {
  console.log(`${err.name}: ${err.message}`);
} finally {
  console.log("done checking");
}
```

Throw `Error` objects (or `TypeError`, `RangeError`, your own subclasses), never plain strings: Error objects carry a **stack trace** that shows where the problem happened.

## Your own error types

```javascript
class TimeoutError extends Error {
  constructor(host, ms) {
    super(`${host} did not answer within ${ms} ms`);
    this.name = "TimeoutError";
    this.host = host;
  }
}
try {
  throw new TimeoutError("db01", 5000);
} catch (err) {
  if (err instanceof TimeoutError) console.log(`retry later: ${err.host}`);
  else throw err;              // not ours: let it go on
}
```

## Classes

```javascript
class Server {
  #failures = 0;                         // # = private field

  constructor(name, role) {
    this.name = name;
    this.role = role;
  }

  recordCheck(ok) {
    this.#failures = ok ? 0 : this.#failures + 1;
  }

  get status() {                         // a getter: used like a property
    if (this.#failures >= 3) return "down";
    return this.#failures > 0 ? "degraded" : "up";
  }

  toString() {
    return `${this.name} (${this.role}): ${this.status}`;
  }
}

const web = new Server("web01", "nginx");
web.recordCheck(false);
console.log(`${web}`);
web.recordCheck(false); web.recordCheck(false);
console.log(web.status);
```

`extends` makes a subclass, `super(...)` calls the parent's constructor, and `static` methods belong to the class itself (`Server.fromJSON(...)`).

@@quiz
Why throw `new Error("...")` rather than a plain string?
* Error objects include a stack trace and work with `instanceof`
- Strings can't be thrown
- It's faster
- Only Error objects can be caught
? You *can* throw a string, but you lose the stack trace and the type.

@@quiz
What does `#failures` in a class mean?
* A private field, not accessible from outside the class
- A comment
- A static field
- A field that can't change
? Private fields start with `#`. `web.#failures` outside the class is a syntax error.

@@exercise Validate a hostname
Write `validateHostname(name)` that returns the name in lowercase if it's valid, and otherwise **throws** an `Error` whose message starts with `invalid hostname`. Valid: 1 to 63 characters, letters, digits and hyphens only, not starting or ending with a hyphen.
@@starter
function validateHostname(name) {
  throw new Error("TODO: not done yet");
}

@@tests
check(validateHostname("Web-01"), "web-01", "validateHostname('Web-01')");
for (const bad of ["", "-web", "web-", "web_01", "a".repeat(64), "web 01"]) {
  let message = "no error";
  try { validateHostname(bad); } catch (e) { message = e.message.startsWith("invalid hostname") ? "rejected" : e.message; }
  check(message, "rejected", `validateHostname(${JSON.stringify(bad)})`);
}
@@hint
`/^[a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?$/i.test(name)` checks all the rules at once.
@@solution
function validateHostname(name) {
  if (!/^[a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?$/i.test(name)) {
    throw new Error(`invalid hostname: ${JSON.stringify(name)}`);
  }
  return name.toLowerCase();
}

@@exercise A ticket class
Write a class `Ticket` with a constructor `(id, title)`. New tickets have `status` `"open"` and an empty `notes` array. Methods: `addNote(text)` adds to `notes`; `close(resolution)` sets the status to `"closed"` and stores `resolution`, but **throws** an Error if the ticket is already closed; a getter `summary` returns `"INC1 [open] Printer jam (0 notes)"`.
@@starter
class Ticket {
  constructor(id, title) {
    throw new Error("TODO: not done yet");
  }
}

@@tests
const t = new Ticket("INC1", "Printer jam");
check(t.status, "open", "a new ticket is open");
check(t.summary, "INC1 [open] Printer jam (0 notes)", "summary of a new ticket");
t.addNote("cleared paper tray");
t.close("restarted spooler");
check(t.summary, "INC1 [closed] Printer jam (1 notes)", "summary after a note and close");
check(t.resolution, "restarted spooler", "the resolution is stored");
let threw = false;
try { t.close("again"); } catch { threw = true; }
check(threw, true, "closing twice throws");
@@hint
In the constructor: `this.id = id; this.title = title; this.status = "open"; this.notes = [];`. The getter is a method written as `get summary() { ... }` that returns a template literal.
@@solution
class Ticket {
  constructor(id, title) {
    this.id = id;
    this.title = title;
    this.status = "open";
    this.notes = [];
  }

  addNote(text) {
    this.notes.push(text);
  }

  close(resolution) {
    if (this.status === "closed") throw new Error(`${this.id} is already closed`);
    this.status = "closed";
    this.resolution = resolution;
  }

  get summary() {
    return `${this.id} [${this.status}] ${this.title} (${this.notes.length} notes)`;
  }
}

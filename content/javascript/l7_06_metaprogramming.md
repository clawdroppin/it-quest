@@lesson l7-meta Symbols, Property Descriptors, Proxy and Reflect
@@concepts objects
@@text
**Metaprogramming** means code that inspects or changes how other code behaves. JavaScript has unusually strong tools for it, used by frameworks (Vue's reactivity is built on `Proxy`), ORMs, test mocks and validation libraries.

## Symbols

A `Symbol` is a unique value, mainly used as a property key that can't clash with any string key:

```javascript
const ID = Symbol("id");
const user = { name: "ana", [ID]: 1042 };
console.log(user[ID], Object.keys(user), JSON.stringify(user));   // symbol keys are hidden from keys and JSON
console.log(Symbol("id") === Symbol("id"), Symbol.for("app.id") === Symbol.for("app.id"));
```

**Well-known symbols** let your objects plug into language features:

| Symbol | Hooks into |
|---|---|
| `Symbol.iterator` | `for...of`, spread |
| `Symbol.asyncIterator` | `for await...of` |
| `Symbol.toPrimitive` | conversion to number/string (`+obj`, `` `${obj}` ``) |
| `Symbol.toStringTag` | `Object.prototype.toString` output |
| `Symbol.dispose` | the `using` declaration (automatic cleanup) |

```javascript
class Money {
  constructor(cents) { this.cents = cents; }
  [Symbol.toPrimitive](hint) {
    return hint === "number" ? this.cents / 100 : `€${(this.cents / 100).toFixed(2)}`;
  }
  get [Symbol.toStringTag]() { return "Money"; }
}
const price = new Money(1999);
console.log(`${price}`, +price * 2, Object.prototype.toString.call(price));
```

## Property descriptors

Every property has hidden attributes: `writable`, `enumerable`, `configurable` (or `get`/`set` for accessors).

```javascript
const config = {};
Object.defineProperty(config, "version", { value: "1.0", writable: false, enumerable: false });
config.version = "2.0";                                 // ignored (throws in strict mode)
console.log(config.version, Object.keys(config));
console.log(Object.getOwnPropertyDescriptor(config, "version"));

const temp = {
  celsius: 20,
  get fahrenheit() { return this.celsius * 9 / 5 + 32; },
  set fahrenheit(f) { this.celsius = (f - 32) * 5 / 9; },
};
temp.fahrenheit = 212;
console.log(temp.celsius);
```

`Object.freeze` sets every property to non-writable and non-configurable; `Object.seal` prevents adding or removing properties but allows changes; `Object.preventExtensions` only stops additions.

## Proxy

A `Proxy` wraps an object and intercepts operations on it with **traps**: `get`, `set`, `has`, `deleteProperty`, `ownKeys`, `apply` (calls), `construct` (`new`) and more.

```javascript
function strict(target, name) {
  return new Proxy(target, {
    get(obj, key, receiver) {
      if (typeof key === "string" && !(key in obj)) throw new ReferenceError(`${name} has no setting "${key}"`);
      return Reflect.get(obj, key, receiver);
    },
    set(obj, key, value) {
      if (typeof value !== typeof obj[key]) throw new TypeError(`${key} must be a ${typeof obj[key]}`);
      return Reflect.set(obj, key, value);
    },
  });
}
const settings = strict({ port: 8080, host: "localhost" }, "settings");
settings.port = 9090;
console.log(settings.port);
try { console.log(settings.prot); } catch (e) { console.log(e.message); }
try { settings.port = "80"; } catch (e) { console.log(e.message); }
```

Common uses: validation, default values, logging/auditing access, **reactivity** (record which properties a render function reads; re-render when they're set), read-only views, and mocks in tests.

## Reflect

`Reflect` has one function per trap (`Reflect.get`, `Reflect.set`, `Reflect.has`, `Reflect.ownKeys`, `Reflect.apply`...). Inside a trap, call the `Reflect` version to perform the default behaviour, so you only customise what you need. `Reflect.ownKeys(obj)` lists **all** own keys, including symbols and non-enumerable ones.

## Explicit resource management

The `using` declaration calls `[Symbol.dispose]()` automatically when the block ends, like Python's `with` or C#'s `using` (supported in recent Node.js and TypeScript):

```javascript,norun
function openLock(name) {
  console.log("lock", name);
  return { [Symbol.dispose]() { console.log("unlock", name); } };
}
{
  using lock = openLock("deploy");
  console.log("deploying...");
}   // "unlock deploy" runs here, even if an error was thrown
```

## Use with care

Metaprogramming makes code harder to follow and slower (proxies prevent some engine optimisations). Use it in libraries and infrastructure, not everyday business logic.

@@quiz
Why use a Symbol as a property key?
* It's unique, so it can't collide with other keys, and it's hidden from Object.keys and JSON
- It's faster than strings
- It makes the property private
- Symbols are required for methods
? `Symbol.for(name)` creates shared, registered symbols.

@@quiz
What does a Proxy's `get` trap intercept?
- Only method calls
* Every property read on the proxy
- Only reads of missing properties
- Network requests
? Use `Reflect.get` inside to do the default read.

@@quiz
Which property attribute stops a property from showing up in `Object.keys` and `for...in`?
- writable: false
* enumerable: false
- configurable: false
- hidden: true
? Built-in methods like `Array.prototype.map` are non-enumerable.

@@flashcards
Symbol :: unique primitive, used as a collision-free key
`Symbol.for(key)` :: shared symbol from the global registry
`Symbol.toPrimitive` :: controls conversion to number/string
Property descriptor :: value, writable, enumerable, configurable (or get/set)
`Object.defineProperty` :: create a property with explicit attributes
Proxy :: wrapper that intercepts operations through traps
Trap :: handler method like get, set, has, apply
Reflect :: default implementations of each trap
`using` :: calls `[Symbol.dispose]()` when the block ends

@@exercise Observable objects
Write `observable(target, onChange)` returning a Proxy that calls `onChange(path, oldValue, newValue)` whenever a property is **set to a different value** or **deleted** (then `newValue` is `undefined`), including properties of nested objects. `path` is the dotted path like `"db.port"`. Setting the same value again does not notify. Nested objects read through the proxy must also be observable (wrap them lazily in the `get` trap).
@@starter
function observable(target, onChange, prefix = "") {
  throw new Error("TODO: not done yet");
}

@@tests
const changes = [];
const cfg = observable({ debug: false, db: { host: "db01", port: 5432 } }, (...args) => changes.push(args));
cfg.debug = true;
cfg.debug = true;
cfg.db.port = 5433;
delete cfg.db.host;
cfg.newKey = "x";
check(changes, [["debug", false, true], ["db.port", 5432, 5433], ["db.host", "db01", undefined], ["newKey", undefined, "x"]], "changes");
check(cfg.db.port, 5433, "reads work");
check(Object.keys(cfg), ["debug", "db", "newKey"], "keys pass through");
@@hint
In `get`, read with `Reflect.get`; if the value is a non-null object, return `observable(value, onChange, path)` where `path = prefix ? `${prefix}.${key}` : key`. In `set`, compare with `Object.is(old, value)` before calling `onChange`. Add a `deleteProperty` trap.
@@solution
function observable(target, onChange, prefix = "") {
  const pathOf = (key) => (prefix ? `${prefix}.${String(key)}` : String(key));
  return new Proxy(target, {
    get(obj, key, receiver) {
      const value = Reflect.get(obj, key, receiver);
      if (typeof value === "object" && value !== null) return observable(value, onChange, pathOf(key));
      return value;
    },
    set(obj, key, value, receiver) {
      const old = obj[key];
      const ok = Reflect.set(obj, key, value, receiver);
      if (ok && !Object.is(old, value)) onChange(pathOf(key), old, value);
      return ok;
    },
    deleteProperty(obj, key) {
      const had = Object.hasOwn(obj, key);
      const old = obj[key];
      const ok = Reflect.deleteProperty(obj, key);
      if (ok && had) onChange(pathOf(key), old, undefined);
      return ok;
    },
  });
}

@@exercise A test spy
Test frameworks replace functions with **spies** that record calls. Write `spy(fn = () => undefined)` returning a function that calls `fn` (with the same `this` and arguments) and returns its result, and has:

- `calls`: array of argument arrays
- `results`: array of `{ type: "return", value }` or `{ type: "throw", value: error }` (errors are re-thrown)
- `callCount` (a getter) and `calledWith(...args)` (true if any call had exactly those arguments, compared with `JSON.stringify`)

Then `spyOn(obj, methodName)` replaces the method with a spy that wraps the original, and gives the spy a `restore()` method that puts the original back.
@@starter
function spy(fn = () => undefined) {
  throw new Error("TODO: not done yet");
}

function spyOn(obj, methodName) {
  throw new Error("TODO: not done yet");
}

@@tests
const s = spy((a, b) => a + b);
check([s(1, 2), s(3, 4)], [3, 7], "passes through");
check([s.callCount, s.calls, s.calledWith(3, 4), s.calledWith(4)], [2, [[1, 2], [3, 4]], true, false], "records calls");
const failing = spy(() => { throw new Error("boom"); });
try { failing(); } catch {}
check([failing.results[0].type, failing.results[0].value.message], ["throw", "boom"], "records throws");
const mailer = { sent: 0, send(to) { this.sent++; return `sent to ${to}`; } };
const original = mailer.send;
const sendSpy = spyOn(mailer, "send");
check(mailer.send("ops@corp"), "sent to ops@corp", "spyOn calls the original");
check([mailer.sent, sendSpy.callCount], [1, 1], "this preserved");
sendSpy.restore();
check(mailer.send === original, true, "restored");
@@hint
Use a regular `function wrapper(...args)` so it receives `this`; inside, `try { const value = fn.apply(this, args); results.push({ type: "return", value }); return value; } catch (e) { results.push({ type: "throw", value: e }); throw e; }`. Add the getter with `Object.defineProperty(wrapper, "callCount", { get: () => wrapper.calls.length })`.
@@solution
function spy(fn = () => undefined) {
  function wrapper(...args) {
    wrapper.calls.push(args);
    try {
      const value = fn.apply(this, args);
      wrapper.results.push({ type: "return", value });
      return value;
    } catch (error) {
      wrapper.results.push({ type: "throw", value: error });
      throw error;
    }
  }
  wrapper.calls = [];
  wrapper.results = [];
  Object.defineProperty(wrapper, "callCount", { get: () => wrapper.calls.length });
  wrapper.calledWith = (...args) => wrapper.calls.some((c) => JSON.stringify(c) === JSON.stringify(args));
  return wrapper;
}

function spyOn(obj, methodName) {
  const original = obj[methodName];
  const s = spy(original);
  s.restore = () => {
    obj[methodName] = original;
  };
  obj[methodName] = s;
  return s;
}

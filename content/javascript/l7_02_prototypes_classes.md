@@lesson l7-prototypes Prototypes and Classes in Depth
@@concepts objects
@@text
JavaScript classes look like Java's or Python's, but underneath they're built on **prototypes**: objects that other objects fall back to when a property is missing. Knowing this explains `instanceof`, inheritance, method sharing and why monkey-patching works.

## The prototype chain

```javascript
const base = { greet() { return `hi from ${this.name}`; } };
const server = Object.create(base);          // server's prototype is base
server.name = "web01";
console.log(server.greet());                 // found on base, this = server
console.log(Object.getPrototypeOf(server) === base, Object.hasOwn(server, "greet"));
console.log(Object.getPrototypeOf(base) === Object.prototype);
```

Reading `server.greet` looks at `server`, then its prototype, then **that** object's prototype, up to `Object.prototype` and finally `null`. **Writing** a property always creates it on the object itself (shadowing the prototype's).

## What `class` actually builds

```javascript
class Device {
  constructor(name) { this.name = name; }
  describe() { return `${this.constructor.name} ${this.name}`; }
}
const d = new Device("sw01");
console.log(typeof Device, Object.getPrototypeOf(d) === Device.prototype);
console.log(Object.hasOwn(d, "name"), Object.hasOwn(d, "describe"), d.describe());
```

- A class is a function. Methods go on `Device.prototype`, **shared** by all instances (one copy in memory).
- `new Device("sw01")` creates an object whose prototype is `Device.prototype`, then runs the constructor with `this` = that object.
- `d instanceof Device` checks whether `Device.prototype` is somewhere in `d`'s chain.

## Modern class features

```javascript
class Account {
  static #count = 0;                          // private static field
  #balance = 0;                               // private instance field
  owner;                                      // public field
  constructor(owner) { this.owner = owner; Account.#count++; }
  get balance() { return this.#balance; }     // getter: read like a property
  deposit(amount) {
    if (amount <= 0) throw new RangeError("amount must be positive");
    this.#balance += amount;
    return this;                              // allows chaining
  }
  #audit() { return `audit ${this.owner}`; }  // private method
  static get count() { return Account.#count; }
  static { console.log("class initialised"); } // static initialisation block
}
const acc = new Account("Ana").deposit(50).deposit(25);
console.log(acc.balance, Account.count, Object.keys(acc));
```

`#private` members are truly private: code outside the class body gets a SyntaxError, not just a convention like Python's `_name`. `#field in obj` checks whether an object has that private field (a safe brand check).

## Inheritance

```javascript
class Device {
  constructor(name) { this.name = name; }
  status() { return "unknown"; }
  toString() { return `${this.name}: ${this.status()}`; }
}
class Switch extends Device {
  constructor(name, ports) {
    super(name);                              // must call super before using this
    this.ports = ports;
  }
  status() { return `${this.ports} ports, ${super.status()}`; }
}
const sw = new Switch("sw01", 48);
console.log(String(sw), sw instanceof Device);
```

`super.method()` calls the parent's version; `super(...)` runs the parent constructor. You can extend built-ins too: `class HttpError extends Error`, `class Stack extends Array`.

## Composition and mixins

Deep inheritance trees get rigid. Prefer **composition** (an object holds helpers), or share behaviour with **mixins**: functions that take a class and return a subclass.

```javascript
const Timestamped = (Base) => class extends Base {
  touch() { this.updated = "2026-11-03"; return this; }
};
const Serializable = (Base) => class extends Base {
  toJSON() { return { ...this, kind: this.constructor.name }; }
};
class Ticket { constructor(title) { this.title = title; } }
class SmartTicket extends Serializable(Timestamped(Ticket)) {}
console.log(JSON.stringify(new SmartTicket("VPN down").touch()));
```

@@quiz
Where does a class's method live?
- Copied onto every instance
* On the class's `prototype` object, shared by all instances
- In a global table
- Inside the constructor
? Instances find it through the prototype chain.

@@quiz
What does `obj instanceof Device` check?
* Whether `Device.prototype` appears in obj's prototype chain
- Whether obj was created in the same file
- Whether obj has a `Device` property
- Whether obj's constructor name is "Device"
? Changing prototypes changes the answer.

@@quiz
What happens if code outside the class reads `acc.#balance`?
- It returns undefined
- It returns the value
* A SyntaxError: private names only exist inside the class body
- A warning is logged
? `#` fields are enforced by the language.

@@quiz
In a subclass constructor, what must happen before you use `this`?
- Nothing
* Call `super(...)`
- Call `Object.create`
- Return an object
? The parent constructor creates and initialises the object first.

@@flashcards
Prototype :: object another object falls back to for missing properties
Prototype chain :: obj → its prototype → ... → Object.prototype → null
`Object.create(proto)` :: new object with the given prototype
`Object.getPrototypeOf(obj)` :: read an object's prototype
`#field` :: truly private class member
getter/setter :: `get x()` / `set x(v)`: property syntax backed by methods
`static` :: belongs to the class, not instances
`super` :: call the parent constructor or method
Mixin :: function that takes a class and returns an extended class

@@exercise A prototype chain by hand
Without using `class`, write:

- `makeDevice(name)`: returns an object whose prototype is `deviceProto`, with an own `name` property
- `deviceProto.describe()`: returns `"<kind> <name>"` using `this.kind`
- `deviceProto.kind` is `"device"`
- `makeRouter(name, routes)`: an object whose prototype is `routerProto`, which in turn has `deviceProto` as its prototype; `routerProto.kind` is `"router"`; `routerProto.routeCount()` returns the number of routes
- `chainOf(obj)`: the list of `kind` values found by walking the prototype chain from `obj`'s prototype upwards, only counting objects that have their **own** `kind`
@@starter
const deviceProto = {};
const routerProto = {};

function makeDevice(name) {
  throw new Error("TODO: not done yet");
}

function makeRouter(name, routes) {
  throw new Error("TODO: not done yet");
}

function chainOf(obj) {
  throw new Error("TODO: not done yet");
}

@@tests
const d = makeDevice("ap01");
const r = makeRouter("rt01", ["0.0.0.0/0", "10.0.0.0/8"]);
check([d.describe(), r.describe(), r.routeCount()], ["device ap01", "router rt01", 2], "methods via the chain");
check([Object.hasOwn(r, "describe"), Object.getPrototypeOf(routerProto) === deviceProto], [false, true], "shared through prototypes");
check(chainOf(r), ["router", "device"], "chain");
@@hint
Fill the prototypes: `deviceProto.kind = "device"; deviceProto.describe = function () { ... }` (a regular function, so `this` works). Link them with `Object.setPrototypeOf(routerProto, deviceProto)`. Create objects with `Object.assign(Object.create(proto), { name })`. For `chainOf`, loop `let p = Object.getPrototypeOf(obj); while (p) { ...; p = Object.getPrototypeOf(p); }`.
@@solution
const deviceProto = {
  kind: "device",
  describe() {
    return `${this.kind} ${this.name}`;
  },
};
const routerProto = {
  kind: "router",
  routeCount() {
    return this.routes.length;
  },
};
Object.setPrototypeOf(routerProto, deviceProto);

function makeDevice(name) {
  return Object.assign(Object.create(deviceProto), { name });
}

function makeRouter(name, routes) {
  return Object.assign(Object.create(routerProto), { name, routes });
}

function chainOf(obj) {
  const kinds = [];
  let p = Object.getPrototypeOf(obj);
  while (p) {
    if (Object.hasOwn(p, "kind")) kinds.push(p.kind);
    p = Object.getPrototypeOf(p);
  }
  return kinds;
}

@@exercise A class hierarchy with private state
Write a `Disk` class:

- `new Disk(name, sizeGb)`; a private `#usedGb` starting at 0
- getters `used`, `free` and `percent` (used / size × 100, rounded to 1 decimal)
- `write(gb)`: throws `RangeError("not enough space on <name>")` if it doesn't fit; otherwise adds and returns `this`
- `static fromString("sda:500")` creates a disk

And `MirroredDisk extends Disk`, whose `write(gb)` writes the same amount to itself and to a `mirror` Disk passed to its constructor as the third argument, and refuses (same error, naming itself) unless **both** have room.
@@starter
class Disk {
  constructor(name, sizeGb) {
    throw new Error("TODO: not done yet");
  }
}

class MirroredDisk extends Disk {}

@@tests
const d = Disk.fromString("sda:500").write(100).write(25);
check([d.name, d.used, d.free, d.percent], ["sda", 125, 375, 25], "a disk");
let msg = "";
try { d.write(400); } catch (e) { msg = `${e.name}: ${e.message}`; }
check(msg, "RangeError: not enough space on sda", "too big");
check(d.used, 125, "unchanged after a failed write");
check(Object.keys(d).includes("usedGb") || Object.keys(d).includes("#usedGb"), false, "used is private");
const backup = new Disk("sdb", 100);
const m = new MirroredDisk("md0", 200, backup);
m.write(80);
check([m.used, backup.used], [80, 80], "mirrored write");
try { m.write(50); msg = "written"; } catch (e) { msg = e.message; }
check([msg, m.used, backup.used], ["not enough space on md0", 80, 80], "mirror full");
check(new Disk("x", 3).write(1).percent, 33.3, "rounded percent");
@@hint
`get percent() { return Math.round((this.#usedGb / this.sizeGb) * 1000) / 10; }`. A subclass can't touch `#usedGb` directly: in `MirroredDisk.write`, check `gb > this.free || gb > this.mirror.free`, then call `this.mirror.write(gb)` and `super.write(gb)`.
@@solution
class Disk {
  #usedGb = 0;

  constructor(name, sizeGb) {
    this.name = name;
    this.sizeGb = sizeGb;
  }

  static fromString(text) {
    const [name, size] = text.split(":");
    return new Disk(name, Number(size));
  }

  get used() {
    return this.#usedGb;
  }

  get free() {
    return this.sizeGb - this.#usedGb;
  }

  get percent() {
    return Math.round((this.#usedGb / this.sizeGb) * 1000) / 10;
  }

  write(gb) {
    if (gb > this.free) throw new RangeError(`not enough space on ${this.name}`);
    this.#usedGb += gb;
    return this;
  }
}

class MirroredDisk extends Disk {
  constructor(name, sizeGb, mirror) {
    super(name, sizeGb);
    this.mirror = mirror;
  }

  write(gb) {
    if (gb > this.free || gb > this.mirror.free) throw new RangeError(`not enough space on ${this.name}`);
    this.mirror.write(gb);
    return super.write(gb);
  }
}

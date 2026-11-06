@@lesson t7-decorators Decorators: Standard Decorators and Method Wrappers
@@concepts functions, objects
@@text
Decorators attach reusable behaviour to classes and their members with `@name` syntax. Frameworks like Angular, NestJS, TypeORM and Lit use them heavily. TypeScript 5.0 implements the **standard** (TC39) decorators; older code uses `experimentalDecorators`, which works differently.

## What a decorator is

A method decorator is just a function that receives the method and a **context** object, and returns a replacement:

```ts,norun
function logged<This, Args extends unknown[], R>(
  target: (this: This, ...args: Args) => R,
  context: ClassMethodDecoratorContext<This, (this: This, ...args: Args) => R>,
) {
  const name = String(context.name);
  return function (this: This, ...args: Args): R {
    console.log(`-> ${name}(${args.join(", ")})`);
    const result = target.call(this, ...args);
    console.log(`<- ${name} = ${String(result)}`);
    return result;
  };
}

class Inventory {
  @logged
  count(kind: string): number { return kind.length; }
}
new Inventory().count("laptop");
```

`@logged` on `count` runs once, when the class is defined, roughly as `Inventory.prototype.count = logged(Inventory.prototype.count, context)`.

## Decorator factories

To pass options, write a function that **returns** a decorator:

```ts,norun
function retry(times: number) {
  return function <This, Args extends unknown[], R>(target: (this: This, ...args: Args) => Promise<R>, _ctx: ClassMethodDecoratorContext) {
    return async function (this: This, ...args: Args): Promise<R> {
      for (let attempt = 1; ; attempt++) {
        try { return await target.call(this, ...args); }
        catch (e) { if (attempt >= times) throw e; }
      }
    };
  };
}

class Api {
  @retry(3)
  async fetchStatus(host: string): Promise<string> { /* ... */ return "up"; }
}
```

## The context object

| Property | Meaning |
|---|---|
| `kind` | `"class"`, `"method"`, `"getter"`, `"setter"`, `"field"`, `"accessor"` |
| `name` | the member's name |
| `static`, `private` | where the member lives |
| `addInitializer(fn)` | run code when an instance (or the class) is set up, e.g. bind `this` |
| `metadata` | a shared object for storing metadata (TypeScript 5.2+, `Symbol.metadata`) |

Other decorator kinds: **class** decorators (register a class, wrap its constructor), **field** decorators (return an initializer that transforms the starting value), and **accessor** decorators for `accessor` fields (auto get/set pairs, good for observable state).

```ts,norun
function bound<This>(target: Function, context: ClassMethodDecoratorContext<This>) {
  context.addInitializer(function (this: This) {
    (this as any)[context.name] = (this as any)[context.name].bind(this);
  });
}
```

## Standard vs experimental decorators

| | Standard (TS 5.0+) | `experimentalDecorators` |
|---|---|---|
| Signature | `(value, context)` | `(target, key, descriptor)` |
| Parameter decorators | no | yes (used by NestJS/Angular DI) |
| Metadata | `context.metadata` | `reflect-metadata` + `emitDecoratorMetadata` |

Existing Angular and NestJS projects still use experimental decorators; new libraries move to the standard ones. Check `tsconfig.json` to know which you're in.

## In this course's runner

Decorators are real JavaScript syntax that Node's type stripping doesn't support yet, so the exercises write decorators as **ordinary functions** and apply them by hand. The function is exactly what you'd put after the `@`; only the application line differs:

```ts
type MethodContext = { kind: "method"; name: string | symbol };
function shout<This, Args extends unknown[]>(target: (this: This, ...args: Args) => string, _ctx: MethodContext) {
  return function (this: This, ...args: Args): string { return target.call(this, ...args).toUpperCase(); };
}

class Greeter {
  greet(name: string): string { return `hello ${name}`; }
}
Greeter.prototype.greet = shout(Greeter.prototype.greet, { kind: "method", name: "greet" });   // = @shout
console.log(new Greeter().greet("ana"));
```

`MethodContext` is a small subset of `ClassMethodDecoratorContext`, so these functions also work as real `@decorators`.

@@quiz
When does a method decorator function run?
* Once, when the class is defined
- Every time the method is called
- When the object is garbage collected
- Only in development builds
? The function it returns is what runs on each call.

@@quiz
What is a decorator factory?
- A class that creates decorators at runtime
* A function that takes options and returns a decorator, used as `@retry(3)`
- A build tool plugin
- An abstract class
? Call it with arguments, and it returns the actual decorator.

@@quiz
Which feature exists only with `experimentalDecorators`?
- Method decorators
- Class decorators
* Parameter decorators (used for dependency injection in NestJS/Angular)
- Decorator factories
? Standard decorators don't support parameters.

@@drill
Which character starts a decorator?
= @
? @logged method() { ... }.

@@flashcards
Decorator :: function applied with `@name` to a class or member
Method decorator signature (standard) :: `(method, context) => replacement`
Decorator factory :: `@retry(3)`: a function returning a decorator
`context.addInitializer` :: run setup per instance or class
`context.metadata` :: shared metadata object (TS 5.2)
`experimentalDecorators` :: legacy decorators: `(target, key, descriptor)`
`accessor` keyword :: auto-accessor field that decorators can wrap

@@exercise Memoize and timing decorators
Using `type MethodContext = { kind: "method"; name: string | symbol }`, write decorator functions:

- `memoize<This, A extends string | number, R>(target: (this: This, arg: A) => R, ctx: MethodContext)`: caches results **per instance** (use a `WeakMap<object, Map<A, R>>`) by the single argument
- `timed(log: string[], clock: () => number)`: a decorator factory; the returned decorator logs `"<name> took <ms>ms"` after each call (also when the method throws, then rethrows)

The tests apply them by hand, as in the lesson.
@@starter
type MethodContext = { kind: "method"; name: string | symbol };

function memoize<This, A extends string | number, R>(target: (this: This, arg: A) => R, ctx: MethodContext): (this: This, arg: A) => R {
  throw new Error("TODO: not done yet");
}

function timed(log: string[], clock: () => number) {
  return function <This, Args extends unknown[], R>(target: (this: This, ...args: Args) => R, ctx: MethodContext): (this: This, ...args: Args) => R {
    throw new Error("TODO: not done yet");
  };
}

@@tests
class Resolver {
  calls = 0;
  prefix: string;
  constructor(prefix: string) { this.prefix = prefix; }
  resolve(host: string): string { this.calls++; return `${this.prefix}.${host.length}`; }
}
Resolver.prototype.resolve = memoize(Resolver.prototype.resolve, { kind: "method", name: "resolve" });
const a = new Resolver("10.0.0");
const b = new Resolver("192.168.1");
check([a.resolve("web01"), a.resolve("web01"), a.resolve("db1"), a.calls], ["10.0.0.5", "10.0.0.5", "10.0.0.3", 2], "cached per argument");
check([b.resolve("web01"), b.calls], ["192.168.1.5", 1], "cache per instance");

const log: string[] = [];
let now = 0;
const clock = () => now;
class Job {
  run(ms: number): string { now += ms; return "done"; }
  fail(): never { now += 7; throw new Error("boom"); }
}
Job.prototype.run = timed(log, clock)(Job.prototype.run, { kind: "method", name: "run" });
Job.prototype.fail = timed(log, clock)(Job.prototype.fail, { kind: "method", name: "fail" });
const job = new Job();
check(job.run(120), "done", "result passed through");
let msg = "";
try { job.fail(); } catch (e) { msg = (e as Error).message; }
check([msg, log], ["boom", ["run took 120ms", "fail took 7ms"]], "timed logs even on error");
@@hint
`memoize`: create the `WeakMap` once, outside the returned function; inside, `this` is the instance: `let cache = caches.get(this as object); if (!cache) caches.set(this as object, (cache = new Map()));`. `timed`: `const start = clock(); try { return target.call(this, ...args); } finally { log.push(...) }`.
@@solution
type MethodContext = { kind: "method"; name: string | symbol };

function memoize<This, A extends string | number, R>(target: (this: This, arg: A) => R, ctx: MethodContext): (this: This, arg: A) => R {
  const caches = new WeakMap<object, Map<A, R>>();
  return function (this: This, arg: A): R {
    let cache = caches.get(this as object);
    if (!cache) caches.set(this as object, (cache = new Map()));
    if (cache.has(arg)) return cache.get(arg) as R;
    const result = target.call(this, arg);
    cache.set(arg, result);
    return result;
  };
}

function timed(log: string[], clock: () => number) {
  return function <This, Args extends unknown[], R>(target: (this: This, ...args: Args) => R, ctx: MethodContext): (this: This, ...args: Args) => R {
    const name = String(ctx.name);
    return function (this: This, ...args: Args): R {
      const start = clock();
      try {
        return target.call(this, ...args);
      } finally {
        log.push(`${name} took ${clock() - start}ms`);
      }
    };
  };
}

@@exercise Validation decorators for fields
Field decorators return an **initializer** that transforms the field's starting value. Write field decorator factories using `type FieldContext = { kind: "field"; name: string | symbol }`:

- `trimmed()`: returns a decorator whose initializer trims strings
- `range(min: number, max: number)`: initializer throws `RangeError("<name> must be between <min> and <max>")` for out-of-range numbers, else returns the value
- `oneOf<const T extends readonly string[]>(options: T)`: throws `Error("<name> must be one of <a|b|c>")` when the value isn't in the list

and `applyField(obj, field, decorators)` that runs the initializers in **reverse** order (like stacked decorators: the one nearest the field runs first) over `obj[field]` and stores the result.
@@starter
type FieldContext = { kind: "field"; name: string | symbol };
type FieldDecorator<V> = (value: undefined, ctx: FieldContext) => (initial: V) => V;

function trimmed(): FieldDecorator<string> {
  throw new Error("TODO: not done yet");
}

function range(min: number, max: number): FieldDecorator<number> {
  throw new Error("TODO: not done yet");
}

function oneOf<const T extends readonly string[]>(options: T): FieldDecorator<string> {
  throw new Error("TODO: not done yet");
}

function applyField<O, K extends keyof O & string>(obj: O, field: K, decorators: FieldDecorator<O[K]>[]): void {
  throw new Error("TODO: not done yet");
}

@@tests
class Server {
  name = "  web01  ";
  port = 8080;
  env = " prod ";
}
const s = new Server();
applyField(s, "name", [trimmed()]);
applyField(s, "port", [range(1, 65535)]);
applyField(s, "env", [oneOf(["dev", "prod"]), trimmed()]);     // trimmed runs first
check([s.name, s.port, s.env], ["web01", 8080, "prod"], "decorated fields");
const bad = new Server();
bad.port = 70000;
let msg = "";
try { applyField(bad, "port", [range(1, 65535)]); } catch (e) { msg = `${(e as Error).name}: ${(e as Error).message}`; }
check(msg, "RangeError: port must be between 1 and 65535", "range");
bad.env = "staging";
try { applyField(bad, "env", [oneOf(["dev", "prod"])]); } catch (e) { msg = (e as Error).message; }
check(msg, "env must be one of dev|prod", "one of");
@@hint
A field decorator looks like `(_value, ctx) => (initial) => newValue`. In `applyField`, go through `[...decorators].reverse()`, call each with `(undefined, { kind: "field", name: field })` to get the initializer, and pipe `obj[field]` through them.
@@solution
type FieldContext = { kind: "field"; name: string | symbol };
type FieldDecorator<V> = (value: undefined, ctx: FieldContext) => (initial: V) => V;

function trimmed(): FieldDecorator<string> {
  return () => (initial) => initial.trim();
}

function range(min: number, max: number): FieldDecorator<number> {
  return (_value, ctx) => (initial) => {
    if (initial < min || initial > max) throw new RangeError(`${String(ctx.name)} must be between ${min} and ${max}`);
    return initial;
  };
}

function oneOf<const T extends readonly string[]>(options: T): FieldDecorator<string> {
  return (_value, ctx) => (initial) => {
    if (!options.includes(initial)) throw new Error(`${String(ctx.name)} must be one of ${options.join("|")}`);
    return initial;
  };
}

function applyField<O, K extends keyof O & string>(obj: O, field: K, decorators: FieldDecorator<O[K]>[]): void {
  let value = obj[field];
  for (const decorator of [...decorators].reverse()) {
    value = decorator(undefined, { kind: "field", name: field })(value);
  }
  obj[field] = value;
}

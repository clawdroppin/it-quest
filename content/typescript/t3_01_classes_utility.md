@@lesson t3-classes-utility Classes, Interfaces and Utility Types
@@concepts objects
@@text
## Classes with types

```typescript
interface Notifier {
  send(message: string): string;
}

class SlackNotifier implements Notifier {
  readonly channel: string;
  #sent = 0;                                   // # = truly private (JavaScript private field)

  constructor(channel: string) {
    this.channel = channel;
  }

  send(message: string): string {
    this.#sent++;
    return `#${this.channel}: ${message}`;
  }

  get sentCount(): number {                    // a getter: used like a property
    return this.#sent;
  }
}

const n: Notifier = new SlackNotifier("ops");
console.log(n.send("disk full on db01"));
console.log((n as SlackNotifier).sentCount);
```

- `implements Notifier` makes the compiler check that the class has everything the interface promises.
- `readonly` fields can only be set in the constructor.
- TypeScript's `private` keyword is only checked at compile time; `#field` is enforced by JavaScript itself at run time. Prefer `#`.
- TypeScript also has a shorthand `constructor(private name: string) {}`. It generates code, so Node's type stripping (and this app) doesn't support it: declare fields explicitly as above.

## Utility types

TypeScript ships with types that transform other types. They save you from writing near-duplicates:

```typescript
type Ticket = {
  id: number;
  title: string;
  priority: 1 | 2 | 3 | 4;
  assignee?: string;
  createdAt: string;
};

type NewTicket = Omit<Ticket, "id" | "createdAt">;    // what a client sends
type TicketPatch = Partial<Pick<Ticket, "title" | "priority" | "assignee">>;   // an update
type TicketSummary = Pick<Ticket, "id" | "title">;
type FrozenTicket = Readonly<Ticket>;

function update(t: Ticket, patch: TicketPatch): Ticket {
  return { ...t, ...patch };
}

const t: Ticket = { id: 1, title: "VPN down", priority: 2, createdAt: "2026-10-31" };
console.log(update(t, { priority: 1, assignee: "kenji" }));
const draft: NewTicket = { title: "Printer jam", priority: 4 };
console.log(draft);
```

| Utility | Gives |
|---|---|
| `Partial<T>` | every property optional |
| `Required<T>` | every property required |
| `Readonly<T>` | every property readonly |
| `Pick<T, K>` / `Omit<T, K>` | only / all except some properties |
| `Record<K, V>` | an object with keys K and values V |
| `ReturnType<F>` / `Parameters<F>` | a function's return / parameter types |
| `Awaited<P>` | what a Promise resolves to |
| `NonNullable<T>` | T without `null` and `undefined` |

## typeof and keyof on values

Derive types from values instead of writing them twice:

```typescript
const defaults = { timeoutMs: 5000, retries: 3, tls: true };
type Options = typeof defaults;                 // { timeoutMs: number; retries: number; tls: boolean }
type OptionName = keyof Options;                // "timeoutMs" | "retries" | "tls"

function withDefaults(o: Partial<Options>): Options {
  return { ...defaults, ...o };
}
const names: OptionName[] = ["retries", "tls"];
console.log(withDefaults({ retries: 5 }), names);
```

## Mapped types, briefly

The utilities above are built from **mapped types**, which loop over keys:

```typescript
type Flags<T> = { [K in keyof T]: boolean };
type ServerFields = { name: string; ip: string; cpu: number };
const changed: Flags<ServerFields> = { name: false, ip: true, cpu: false };
console.log(Object.entries(changed).filter(([, v]) => v).map(([k]) => k));
```

You'll rarely write mapped types yourself, but you'll read them in libraries.

## Replacing enums

TypeScript `enum`s generate code (so type stripping can't run them), and they have quirks. Use a const object plus a type:

```typescript
const Status = { Up: "up", Degraded: "degraded", Down: "down" } as const;
type Status = (typeof Status)[keyof typeof Status];   // "up" | "degraded" | "down"

function icon(s: Status): string {
  return s === Status.Up ? "🟢" : s === Status.Degraded ? "🟡" : "🔴";
}
console.log(icon(Status.Down), icon("up"));
```

@@quiz
What does `Omit<Ticket, "id">` give?
* The Ticket type without the `id` property
- A Ticket where `id` is optional
- Only the `id` property
- An error
? `Pick` keeps listed properties; `Omit` removes them.

@@quiz
Why prefer `#count` to `private count` in a class?
* `#` is enforced at run time by JavaScript; `private` only by the compiler
- `#` is shorter
- `private` doesn't exist in TypeScript
- `#` makes the field public
? `private` disappears when the types are stripped.

@@quiz
What is `typeof defaults` in a type position?
* The type of the `defaults` value, inferred from it
- The string "object"
- `any`
- A runtime check
? In a type position, `typeof` reads a value's static type.

@@flashcards
`implements I` :: the class must provide everything interface I has
`#field` :: a private field, enforced at run time
`Partial<T>` :: all properties optional
`Pick<T, "a" | "b">` :: only those properties
`Omit<T, "a">` :: all but those properties
`ReturnType<typeof f>` :: the return type of function f
`keyof typeof obj` :: the keys of a value's type
`as const` object :: the enum replacement

@@exercise Typed rate limiter
Complete the `RateLimiter` class: `allow(user: string, now: number): boolean` lets each user make at most `max` requests in any `windowMs`-millisecond window, keeping timestamps in a private `#hits: Map<string, number[]>`.
@@starter
class RateLimiter {
  readonly max: number;
  readonly windowMs: number;
  #hits = new Map<string, number[]>();

  constructor(max: number, windowMs: number) {
    this.max = max;
    this.windowMs = windowMs;
  }

  allow(user: string, now: number): boolean {
    throw new Error("TODO: not done yet");
  }
}

@@tests
const rl = new RateLimiter(2, 1000);
check(rl.allow("aiko", 0), true, "first request");
check(rl.allow("aiko", 100), true, "second request");
check(rl.allow("aiko", 200), false, "third within the window");
check(rl.allow("kenji", 200), true, "other users are separate");
check(rl.allow("aiko", 1001), true, "the first hit has expired");
check(rl.allow("aiko", 1050), false, "still two in the window (100 and 1001)");
@@hint
Keep only the hits with `now - t < this.windowMs`. If fewer than `max` are left, push `now` and return true. Save the list back with `this.#hits.set(user, recent)`.
@@solution
class RateLimiter {
  readonly max: number;
  readonly windowMs: number;
  #hits = new Map<string, number[]>();

  constructor(max: number, windowMs: number) {
    this.max = max;
    this.windowMs = windowMs;
  }

  allow(user: string, now: number): boolean {
    const recent = (this.#hits.get(user) ?? []).filter((t) => now - t < this.windowMs);
    const ok = recent.length < this.max;
    if (ok) recent.push(now);
    this.#hits.set(user, recent);
    return ok;
  }
}

@@exercise Apply a patch
Write `applyPatch(user: User, patch: Partial<Omit<User, "id">>): User` that returns a **new** user with the patch applied. It must ignore keys in the patch whose value is `undefined`, and must not change the original.
@@starter
type User = { id: number; name: string; dept: string; admin: boolean };

function applyPatch(user: User, patch: Partial<Omit<User, "id">>): User {
  throw new Error("TODO: not done yet");
}

@@tests
const u: User = { id: 1, name: "aiko", dept: "sales", admin: false };
check(applyPatch(u, { dept: "it", admin: true }), { id: 1, name: "aiko", dept: "it", admin: true }, "applies changes");
check(applyPatch(u, { name: undefined }).name, "aiko", "ignores undefined");
check(u.dept, "sales", "original unchanged");
@@hint
`const clean = Object.fromEntries(Object.entries(patch).filter(([, v]) => v !== undefined));` then `return { ...user, ...clean };`.
@@solution
type User = { id: number; name: string; dept: string; admin: boolean };

function applyPatch(user: User, patch: Partial<Omit<User, "id">>): User {
  const clean = Object.fromEntries(Object.entries(patch).filter(([, v]) => v !== undefined));
  return { ...user, ...clean };
}

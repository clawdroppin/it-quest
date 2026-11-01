@@lesson t5-domain Modeling a Domain with Types
@@text
In application code, TypeScript's biggest gift isn't catching typos. It's that you can describe your business so precisely that **wrong states can't be written down**. A bug the compiler rejects never reaches a test, a review or a customer.

## Make illegal states unrepresentable

Here's a typical first attempt at an order type:

```typescript,norun
type Order = {
  status: "draft" | "paid" | "shipped" | "cancelled";
  paidAt?: Date;          // only when paid or shipped... hopefully
  trackingNumber?: string; // only when shipped... hopefully
  cancelReason?: string;   // only when cancelled... hopefully
};
```

Nothing stops `{ status: "draft", trackingNumber: "JP123" }`, and every function must check optional fields that "should" be there. A **discriminated union** says exactly which fields exist in which state:

```typescript
type Order =
  | { status: "draft"; items: string[] }
  | { status: "paid"; items: string[]; paidAt: Date }
  | { status: "shipped"; items: string[]; paidAt: Date; trackingNumber: string }
  | { status: "cancelled"; items: string[]; reason: string };

function describe(o: Order): string {
  switch (o.status) {
    case "draft":
      return `draft with ${o.items.length} item(s)`;
    case "paid":
      return `paid on ${o.paidAt.toISOString().slice(0, 10)}`;
    case "shipped":
      return `shipped, tracking ${o.trackingNumber}`;   // TypeScript knows it exists here
    case "cancelled":
      return `cancelled: ${o.reason}`;
  }
}

console.log(describe({ status: "shipped", items: ["laptop"], paidAt: new Date("2026-11-01"), trackingNumber: "JP4410" }));
console.log(describe({ status: "cancelled", items: [], reason: "duplicate order" }));
```

## State machines

State changes become functions that only accept the right state:

```typescript
type Draft = { status: "draft"; items: string[] };
type Paid = { status: "paid"; items: string[]; paidAt: Date };
type Shipped = { status: "shipped"; items: string[]; paidAt: Date; trackingNumber: string };

const pay = (o: Draft, at: Date): Paid => ({ ...o, status: "paid", paidAt: at });
const ship = (o: Paid, trackingNumber: string): Shipped => ({ ...o, status: "shipped", trackingNumber });

const order: Draft = { status: "draft", items: ["monitor"] };
const shipped = ship(pay(order, new Date("2026-11-02")), "JP4411");
console.log(shipped);
// ship(order, "JP1") would not compile: a Draft isn't Paid. The rule is enforced by the compiler.
```

## Exhaustiveness: never forget a case

Add a `default` that assigns the value to `never`. When someone adds a new status next year, every `switch` that doesn't handle it becomes a compile error:

```typescript
type Alert = { level: "info" } | { level: "warning"; team: string } | { level: "page"; team: string; phone: string };

function assertNever(x: never): never {
  throw new Error(`unhandled case: ${JSON.stringify(x)}`);
}

function route(a: Alert): string {
  switch (a.level) {
    case "info":
      return "log only";
    case "warning":
      return `ticket for ${a.team}`;
    case "page":
      return `call ${a.phone}`;
    default:
      return assertNever(a);   // if a new level is added and not handled, this line stops compiling
  }
}

console.log(route({ level: "page", team: "network", phone: "+81-3-1234-5678" }));
```

## Branded types: an id is not just a number

`UserId` and `TicketId` are both numbers, so `assign(ticketId, userId)` with the arguments swapped compiles fine and corrupts data. A **brand** makes them different types with zero runtime cost:

```typescript
type Brand<T, B extends string> = T & { readonly __brand: B };
type UserId = Brand<number, "UserId">;
type TicketId = Brand<number, "TicketId">;

const userId = (n: number) => n as UserId;
const ticketId = (n: number) => n as TicketId;

function assign(ticket: TicketId, user: UserId): string {
  return `ticket #${ticket} → user ${user}`;
}

console.log(assign(ticketId(4410), userId(7)));
// assign(userId(7), ticketId(4410))  ← compile error: the arguments are swapped
```

The same trick gives you `Email`, `Yen` vs `Usd`, or `SanitizedHtml`: values that can only be created by a function that checked them.

## Result types for expected failures

Exceptions are invisible in a function's type: nothing tells the caller that `parsePort` can throw. For **expected** failures (bad input, not found, not allowed), return a value instead:

```typescript
type Result<T, E = string> = { ok: true; value: T } | { ok: false; error: E };

function parsePort(text: string): Result<number> {
  const n = Number(text);
  if (!Number.isInteger(n)) return { ok: false, error: `"${text}" is not a whole number` };
  if (n < 1 || n > 65535) return { ok: false, error: `${n} is out of range` };
  return { ok: true, value: n };
}

for (const input of ["443", "http", "70000"]) {
  const r = parsePort(input);
  console.log(r.ok ? `port ${r.value}` : `error: ${r.error}`);   // you must check ok to reach value
}
```

Keep exceptions for the truly unexpected: bugs, a lost database connection.

## Immutability

`readonly` and `Readonly<T>` stop accidental changes, and returning new objects (`{ ...o, status: "paid" }`) instead of editing old ones makes state changes easy to follow, test and undo:

```typescript
type Config = Readonly<{ hosts: readonly string[]; retries: number }>;

const config: Config = { hosts: ["web01", "web02"], retries: 3 };
const more: Config = { ...config, hosts: [...config.hosts, "web03"] };
console.log(config.hosts, more.hosts);
// config.retries = 5;        ← compile error
// config.hosts.push("x");    ← compile error
```

@@quiz
What's the main benefit of a discriminated union for an order's status?
- It uses less memory
* Each state has exactly its own fields, so invalid combinations can't be written
- It makes JSON smaller
- It replaces the need for tests
? `{ status: "draft", trackingNumber: ... }` becomes a compile error instead of a bug.

@@quiz
What does `assertNever` in a `default` branch give you?
- Faster switch statements
* A compile error in every switch that doesn't handle a newly added case
- Automatic logging
- A way to skip cases
? If all cases are handled, the value's type in `default` is `never`. A new, unhandled case makes it not-never, which doesn't compile.

@@quiz
Why brand `UserId` and `TicketId` instead of using `number` for both?
- Branded numbers are faster
- JSON needs brands
* So swapping them by mistake is a compile error
- To store them as strings
? Brands exist only for the compiler; at runtime they're still plain numbers.

@@quiz
When should a function return a `Result` instead of throwing?
* For expected failures that callers must handle, like invalid input or "not found"
- Always: exceptions are bad
- Only in async functions
- For bugs and crashes
? The `Result` type appears in the signature, so callers can't forget. Unexpected failures still throw.

@@flashcards
discriminated union :: a union of object types with a shared literal field (e.g. `status`)
make illegal states unrepresentable :: design types so invalid data can't be written
state machine with types :: transition functions accept only the right state (`ship(o: Paid)`)
`assertNever(x: never)` :: exhaustiveness check in a switch's default
branded type :: `number & { __brand: "UserId" }`: same runtime value, different compile-time type
`Result<T, E>` :: `{ ok: true; value } | { ok: false; error }` for expected failures
`Readonly<T>` / `readonly T[]` :: no assignments or mutating methods allowed
spread update :: `{ ...o, status: "paid" }` returns a new object instead of changing the old one

@@exercise A ticket state machine
Tickets are `open`, `in_progress` (with `assignee`), `resolved` (with `assignee` and `resolution`) or `closed` (with `closedAt`). Write `transition(t, event)` returning a `Result<Ticket>`. Events:

- `{ type: "start", assignee }`: only from `open` → `in_progress`
- `{ type: "resolve", resolution }`: only from `in_progress` → `resolved` (keep the assignee)
- `{ type: "reopen" }`: only from `resolved` → `in_progress` (keep the assignee)
- `{ type: "close", at }`: only from `resolved` → `closed`

Anything else: `{ ok: false, error: "can't <type> a ticket that is <status>" }`.
@@starter
type Ticket =
  | { id: number; status: "open" }
  | { id: number; status: "in_progress"; assignee: string }
  | { id: number; status: "resolved"; assignee: string; resolution: string }
  | { id: number; status: "closed"; closedAt: string };

type Event =
  | { type: "start"; assignee: string }
  | { type: "resolve"; resolution: string }
  | { type: "reopen" }
  | { type: "close"; at: string };

type Result<T> = { ok: true; value: T } | { ok: false; error: string };

function transition(t: Ticket, e: Event): Result<Ticket> {
  throw new Error("TODO: not done yet");
}

console.log(transition({ id: 1, status: "open" }, { type: "start", assignee: "yuki" }));
@@tests
const _open: Ticket = { id: 7, status: "open" };
const _s1 = transition(_open, { type: "start", assignee: "yuki" });
check(_s1, { ok: true, value: { id: 7, status: "in_progress", assignee: "yuki" } }, "start");
const _s2 = _s1.ok ? transition(_s1.value, { type: "resolve", resolution: "replaced cable" }) : _s1;
check(_s2, { ok: true, value: { id: 7, status: "resolved", assignee: "yuki", resolution: "replaced cable" } }, "resolve");
const _s3 = _s2.ok ? transition(_s2.value, { type: "reopen" }) : _s2;
check(_s3, { ok: true, value: { id: 7, status: "in_progress", assignee: "yuki" } }, "reopen");
check(_s2.ok ? transition(_s2.value, { type: "close", at: "2026-11-02" }) : _s2, { ok: true, value: { id: 7, status: "closed", closedAt: "2026-11-02" } }, "close");
check(transition(_open, { type: "close", at: "x" }), { ok: false, error: "can't close a ticket that is open" }, "can't close an open ticket");
check(transition(_open, { type: "reopen" }), { ok: false, error: "can't reopen a ticket that is open" }, "can't reopen an open ticket");
check(_open, { id: 7, status: "open" }, "the original ticket is unchanged");
@@hint
Switch on `e.type`, and inside each case check `t.status`. For example: `case "start": if (t.status === "open") return { ok: true, value: { id: t.id, status: "in_progress", assignee: e.assignee } }; break;`. After the switch, return the failure, building the message from `e.type` and `t.status` with a template string.
@@solution
type Ticket =
  | { id: number; status: "open" }
  | { id: number; status: "in_progress"; assignee: string }
  | { id: number; status: "resolved"; assignee: string; resolution: string }
  | { id: number; status: "closed"; closedAt: string };

type Event =
  | { type: "start"; assignee: string }
  | { type: "resolve"; resolution: string }
  | { type: "reopen" }
  | { type: "close"; at: string };

type Result<T> = { ok: true; value: T } | { ok: false; error: string };

function transition(t: Ticket, e: Event): Result<Ticket> {
  switch (e.type) {
    case "start":
      if (t.status === "open") return { ok: true, value: { id: t.id, status: "in_progress", assignee: e.assignee } };
      break;
    case "resolve":
      if (t.status === "in_progress") return { ok: true, value: { id: t.id, status: "resolved", assignee: t.assignee, resolution: e.resolution } };
      break;
    case "reopen":
      if (t.status === "resolved") return { ok: true, value: { id: t.id, status: "in_progress", assignee: t.assignee } };
      break;
    case "close":
      if (t.status === "resolved") return { ok: true, value: { id: t.id, status: "closed", closedAt: e.at } };
      break;
  }
  return { ok: false, error: `can't ${e.type} a ticket that is ${t.status}` };
}

console.log(transition({ id: 1, status: "open" }, { type: "start", assignee: "yuki" }));

@@exercise Money without mistakes
Adding yen to dollars is a classic bug. Write a `Money` type `{ amount: number; currency: "JPY" | "USD" }` (amounts in the smallest unit: yen, or cents) and:

- `add(a, b)`: a `Result<Money>`; different currencies → `{ ok: false, error: "currency mismatch: JPY + USD" }`,
- `allocate(m, parts)`: split an amount into `parts` shares that add up **exactly** to the original (the first shares get the leftover units). `allocate(¥1000, 3)` → 334, 333, 333.
@@starter
type Money = { amount: number; currency: "JPY" | "USD" };
type Result<T> = { ok: true; value: T } | { ok: false; error: string };

function add(a: Money, b: Money): Result<Money> {
  throw new Error("TODO: not done yet");
}

function allocate(m: Money, parts: number): Money[] {
  throw new Error("TODO: not done yet");
}

console.log(allocate({ amount: 1000, currency: "JPY" }, 3));
@@tests
const _yen = (amount: number): Money => ({ amount, currency: "JPY" });
check(add(_yen(1200), _yen(800)), { ok: true, value: _yen(2000) }, "yen + yen");
check(add(_yen(100), { amount: 100, currency: "USD" }), { ok: false, error: "currency mismatch: JPY + USD" }, "yen + dollars");
check(allocate(_yen(1000), 3).map((m) => m.amount), [334, 333, 333], "1000 in 3 parts");
check(allocate({ amount: 1001, currency: "USD" }, 4).map((m) => m.amount), [251, 250, 250, 250], "1001 cents in 4 parts");
check(allocate(_yen(10), 5).map((m) => m.amount), [2, 2, 2, 2, 2], "an even split");
check(allocate(_yen(7), 3).every((m) => m.currency === "JPY"), true, "the currency is kept");
@@hint
`add`: compare the currencies first. `allocate`: `const base = Math.floor(m.amount / parts)` and `const extra = m.amount % parts`; share `i` is `base + (i < extra ? 1 : 0)`. Build them with `Array.from({ length: parts }, (_, i) => ...)`.
@@solution
type Money = { amount: number; currency: "JPY" | "USD" };
type Result<T> = { ok: true; value: T } | { ok: false; error: string };

function add(a: Money, b: Money): Result<Money> {
  if (a.currency !== b.currency) return { ok: false, error: `currency mismatch: ${a.currency} + ${b.currency}` };
  return { ok: true, value: { amount: a.amount + b.amount, currency: a.currency } };
}

function allocate(m: Money, parts: number): Money[] {
  const base = Math.floor(m.amount / parts);
  const extra = m.amount % parts;
  return Array.from({ length: parts }, (_, i) => ({ amount: base + (i < extra ? 1 : 0), currency: m.currency }));
}

console.log(allocate({ amount: 1000, currency: "JPY" }, 3));

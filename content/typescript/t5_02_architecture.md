@@lesson t5-architecture Ports, Adapters, Events and Testable Design
@@text
A codebase that's easy to change for years has one property above all: the **business rules don't depend on details**. Details are the database, the web framework, the email provider, the clock. This lesson shows the patterns that achieve that in TypeScript; they're the same in C#, Java and Go.

## Ports and adapters (hexagonal architecture)

```text
                 ┌───────────────────────────────┐
  HTTP API ─────►│                               │◄──── port: OrderRepo ◄── PostgresOrderRepo
  CLI ──────────►│   use cases + domain rules    │◄──── port: Payments  ◄── StripePayments
  test ─────────►│   (no imports of details)     │◄──── port: Clock     ◄── () => new Date()
                 └───────────────────────────────┘
```

A **port** is an interface the core defines, in the core's own words. An **adapter** implements it for a specific technology. The core never imports an adapter.

```typescript
// The core defines what it needs...
interface InventoryRepo {
  stock(sku: string): number;
  reserve(sku: string, qty: number): void;
}
interface Mailer {
  send(to: string, subject: string): void;
}

// ...and the use case only knows those interfaces.
class ReserveStock {
  private repo: InventoryRepo;
  private mailer: Mailer;
  constructor(repo: InventoryRepo, mailer: Mailer) {
    this.repo = repo;
    this.mailer = mailer;
  }
  run(sku: string, qty: number, buyer: string): string {
    if (this.repo.stock(sku) < qty) return "not enough stock";
    this.repo.reserve(sku, qty);
    this.mailer.send(buyer, `Reserved ${qty} × ${sku}`);
    return "reserved";
  }
}

// Adapters: in-memory ones for tests and demos; a database and SMTP in production.
class MemoryInventory implements InventoryRepo {
  private items = new Map<string, number>([["LAPTOP-T14", 3]]);
  stock(sku: string) { return this.items.get(sku) ?? 0; }
  reserve(sku: string, qty: number) { this.items.set(sku, this.stock(sku) - qty); }
}
class RecordingMailer implements Mailer {
  sent: string[] = [];
  send(to: string, subject: string) { this.sent.push(`${to}: ${subject}`); }
}

const mailer = new RecordingMailer();
const useCase = new ReserveStock(new MemoryInventory(), mailer);
console.log(useCase.run("LAPTOP-T14", 2, "yuki@example.jp"), useCase.run("LAPTOP-T14", 2, "ken@example.jp"));
console.log(mailer.sent);
```

## Dependency injection, without a framework

"Dependency injection" just means passing dependencies in (usually through the constructor), as above. One place at startup, the **composition root**, creates the real adapters and wires everything:

```typescript,norun
// main.ts: the only file that knows every concrete class
const config = loadConfig(process.env);
const db = new Pool({ connectionString: config.databaseUrl });
const reserve = new ReserveStock(new PostgresInventory(db), new SmtpMailer(config.smtp));
startHttpServer({ reserve }, config.port);
```

Frameworks like **NestJS** and **Angular** (and .NET, Spring in Java) automate this with a DI container. It's the same idea with less typing.

## Test doubles

| Double | What it is | Example |
|---|---|---|
| **fake** | a working, simple implementation | `MemoryInventory` |
| **stub** | returns fixed answers | a clock that always says 09:00 |
| **spy / mock** | records how it was called, so the test can check | `RecordingMailer` |

Prefer fakes and stubs, and check **results**, not every call: tests that assert "method X was called with Y" break whenever you refactor, even when behaviour is unchanged.

## Events: decoupling the "and then"

"When an order is paid: email the customer, update the dashboard, tell the warehouse, write the audit log." Putting all of that in `payOrder()` makes it huge and couples it to five systems. Instead, the use case **publishes an event**, and independent handlers subscribe:

```typescript
type Events = {
  OrderPaid: { orderId: number; total: number };
  OrderCancelled: { orderId: number; reason: string };
};

class EventBus<E extends Record<string, object>> {
  private handlers: { [K in keyof E]?: ((payload: E[K]) => void)[] } = {};

  on<K extends keyof E>(name: K, handler: (payload: E[K]) => void): void {
    (this.handlers[name] ??= []).push(handler);
  }

  emit<K extends keyof E>(name: K, payload: E[K]): void {
    for (const h of this.handlers[name] ?? []) h(payload);
  }
}

const bus = new EventBus<Events>();
bus.on("OrderPaid", (e) => console.log(`email: thanks for order #${e.orderId}`));
bus.on("OrderPaid", (e) => console.log(`warehouse: pick order #${e.orderId}`));
bus.on("OrderCancelled", (e) => console.log(`audit: #${e.orderId} cancelled (${e.reason})`));

bus.emit("OrderPaid", { orderId: 4410, total: 129800 });
bus.emit("OrderCancelled", { orderId: 4411, reason: "customer request" });
// bus.emit("OrderPaid", { orderId: 1 })   ← compile error: total is missing
```

The mapped type `{ [K in keyof E]?: ... }` makes every event name and payload type-checked. Across services, the same pattern runs over a message broker (Kafka, RabbitMQ, AWS SNS/SQS), which adds retries and durability.

## Modules and boundaries

- Group code **by feature** (`orders/`, `inventory/`, `billing/`), not by technical type (`controllers/`, `models/`). Changes usually touch one feature.
- Each feature exports a small public API from an `index.ts`; other features import only that.
- Watch the dependency direction: `orders` may use `inventory`'s public API, but if `inventory` also imports `orders`, you have a cycle. Break it with an event or an interface.

@@quiz
In ports and adapters, who defines the `InventoryRepo` interface?
- The database driver
* The core (use cases), in its own terms
- The web framework
- Each adapter defines its own
? The core says what it needs. Adapters conform to it, so the core never depends on them.

@@quiz
What is the composition root?
- The first commit of the repository
* The one place at startup that creates the real adapters and wires everything together
- The top folder of the project
- The base class of all services
? Only `main.ts` (or the DI container setup) knows concrete classes. Everything else depends on interfaces.

@@quiz
Why publish an `OrderPaid` event instead of calling the mailer, warehouse and dashboard directly from `payOrder`?
- Events are faster
* New reactions can be added without changing payOrder, and it isn't coupled to every system
- Function calls can't be tested
- TypeScript requires it
? Each subscriber is independent. Adding "send a Slack message" is one new handler.

@@quiz
Which test is least likely to break during a refactor?
* One that checks the result (stock went down, an email was recorded)
- One that checks the exact order of every internal method call
- One that mocks every private function
- One that checks the number of lines in a file
? Test behaviour, not implementation.

@@flashcards
port :: an interface the core defines for something it needs (repo, clock, mailer)
adapter :: an implementation of a port for one technology (Postgres, SMTP, memory)
hexagonal architecture :: core in the middle, all details plugged in through ports
dependency injection :: pass dependencies in, usually via the constructor
composition root :: the startup code that wires concrete classes together
fake / stub / spy :: working simple version / fixed answers / records calls
event bus :: publishers emit named events; independent handlers subscribe
mapped type `{ [K in keyof E]: ... }` :: build a type from every key of another type
organize by feature :: `orders/`, `billing/` instead of `controllers/`, `models/`

@@exercise A typed event bus with unsubscribe
Write `createBus<E>()` returning `{ on, emit }`. `on(name, handler)` subscribes and **returns a function that unsubscribes** that handler. `emit(name, payload)` calls the handlers for that name in the order they subscribed and returns how many were called. A handler that throws must not stop the others: count it, and log `handler failed: <message>` with `console.log`.
@@starter
function createBus<E extends Record<string, unknown>>() {
  // TODO: return { on, emit }
}

type AppEvents = { UserCreated: { name: string }; DiskFull: { host: string } };
const bus = createBus<AppEvents>();
const off = bus.on("UserCreated", (e) => console.log("welcome", e.name));
console.log(bus.emit("UserCreated", { name: "yuki" }));
off();
console.log(bus.emit("UserCreated", { name: "ken" }));
@@tests
type _E = { A: { n: number }; B: { s: string } };
const _bus = createBus<_E>();
const _log: string[] = [];
const _off1 = _bus.on("A", (e) => _log.push(`first ${e.n}`));
_bus.on("A", () => { throw new Error("boom"); });
_bus.on("A", (e) => _log.push(`third ${e.n}`));
_bus.on("B", (e) => _log.push(`b ${e.s}`));
check(_bus.emit("A", { n: 1 }), 3, "three handlers were called");
check(_log, ["first 1", "third 1"], "handlers ran in order; the failing one didn't stop the third");
_off1();
_log.length = 0;
check(_bus.emit("A", { n: 2 }), 2, "after unsubscribing");
check(_log, ["third 2"], "the first handler is gone");
check(_bus.emit("B", { s: "x" }), 1, "event B");
@@hint
Keep `const handlers = new Map<keyof E, Array<(p: any) => void>>()`. `on` pushes the handler and returns `() => { const list = handlers.get(name) ?? []; const i = list.indexOf(handler); if (i >= 0) list.splice(i, 1); }`. In `emit`, loop over a copy of the list with `try { h(payload) } catch (err) { console.log(...) }` and count every call.
@@solution
function createBus<E extends Record<string, unknown>>() {
  const handlers = new Map<keyof E, Array<(payload: any) => void>>();
  return {
    on<K extends keyof E>(name: K, handler: (payload: E[K]) => void): () => void {
      const list = handlers.get(name) ?? [];
      list.push(handler);
      handlers.set(name, list);
      return () => {
        const i = list.indexOf(handler);
        if (i >= 0) list.splice(i, 1);
      };
    },
    emit<K extends keyof E>(name: K, payload: E[K]): number {
      let called = 0;
      for (const h of [...(handlers.get(name) ?? [])]) {
        called++;
        try {
          h(payload);
        } catch (err) {
          console.log(`handler failed: ${err instanceof Error ? err.message : String(err)}`);
        }
      }
      return called;
    },
  };
}

type AppEvents = { UserCreated: { name: string }; DiskFull: { host: string } };
const bus = createBus<AppEvents>();
const off = bus.on("UserCreated", (e) => console.log("welcome", e.name));
console.log(bus.emit("UserCreated", { name: "yuki" }));
off();
console.log(bus.emit("UserCreated", { name: "ken" }));

@@exercise Use case with ports
Write the `TransferLaptop` use case. Its constructor takes an `AssetRepo` and an `AuditLog` (interfaces below). `run(tag, toUser, by)` returns a string:

- unknown asset → `"no asset <tag>"`
- the asset is already assigned to `toUser` → `"<tag> already belongs to <toUser>"`
- otherwise set the owner, write the audit line `"<by> moved <tag> from <old owner or nobody> to <toUser>"`, and return `"moved"`.

Then make the tests pass using the provided fakes.
@@starter
type Asset = { tag: string; owner: string | null };
interface AssetRepo { find(tag: string): Asset | undefined; save(a: Asset): void; }
interface AuditLog { write(line: string): void; }

class TransferLaptop {
  // TODO: constructor(repo, audit) and run(tag, toUser, by)
}

class MemoryAssets implements AssetRepo {
  items = new Map<string, Asset>([["PC-0142", { tag: "PC-0142", owner: "yuki" }], ["PC-0200", { tag: "PC-0200", owner: null }]]);
  find(tag: string) { return this.items.get(tag); }
  save(a: Asset) { this.items.set(a.tag, a); }
}
class MemoryAudit implements AuditLog {
  lines: string[] = [];
  write(line: string) { this.lines.push(line); }
}

const audit = new MemoryAudit();
console.log(new TransferLaptop(new MemoryAssets(), audit).run("PC-0142", "ken", "admin"), audit.lines);
@@tests
const _assets = new MemoryAssets();
const _audit = new MemoryAudit();
const _uc = new TransferLaptop(_assets, _audit);
check(_uc.run("PC-0142", "ken", "it-admin"), "moved", "a transfer");
check(_assets.find("PC-0142"), { tag: "PC-0142", owner: "ken" }, "the owner changed");
check(_uc.run("PC-0200", "aiko", "it-admin"), "moved", "an unassigned laptop");
check(_audit.lines, ["it-admin moved PC-0142 from yuki to ken", "it-admin moved PC-0200 from nobody to aiko"], "the audit log");
check(_uc.run("PC-0142", "ken", "it-admin"), "PC-0142 already belongs to ken", "no-op transfer");
check(_uc.run("PC-9999", "ken", "it-admin"), "no asset PC-9999", "unknown asset");
check(_audit.lines.length, 2, "failures write no audit lines");
@@hint
Store the two ports in fields (assign them in the constructor; this TypeScript runs with type stripping, so write the fields out instead of using `constructor(private repo: ...)`). In `run`, `const asset = this.repo.find(tag)`, handle the two failure cases, then `this.repo.save({ ...asset, owner: toUser })` and write the audit line with `asset.owner ?? "nobody"`.
@@solution
type Asset = { tag: string; owner: string | null };
interface AssetRepo { find(tag: string): Asset | undefined; save(a: Asset): void; }
interface AuditLog { write(line: string): void; }

class TransferLaptop {
  private repo: AssetRepo;
  private audit: AuditLog;

  constructor(repo: AssetRepo, audit: AuditLog) {
    this.repo = repo;
    this.audit = audit;
  }

  run(tag: string, toUser: string, by: string): string {
    const asset = this.repo.find(tag);
    if (!asset) return `no asset ${tag}`;
    if (asset.owner === toUser) return `${tag} already belongs to ${toUser}`;
    this.repo.save({ ...asset, owner: toUser });
    this.audit.write(`${by} moved ${tag} from ${asset.owner ?? "nobody"} to ${toUser}`);
    return "moved";
  }
}

class MemoryAssets implements AssetRepo {
  items = new Map<string, Asset>([["PC-0142", { tag: "PC-0142", owner: "yuki" }], ["PC-0200", { tag: "PC-0200", owner: null }]]);
  find(tag: string) { return this.items.get(tag); }
  save(a: Asset) { this.items.set(a.tag, a); }
}
class MemoryAudit implements AuditLog {
  lines: string[] = [];
  write(line: string) { this.lines.push(line); }
}

const audit = new MemoryAudit();
console.log(new TransferLaptop(new MemoryAssets(), audit).run("PC-0142", "ken", "admin"), audit.lines);

@@lesson t0-type-checking How Type Checking Works: Structural Types, any, unknown and never
@@concepts variables, functions
@@text
TypeScript's type system is unusual in a few important ways. Understanding them early saves a lot of confusion later.

## Types are erased

Types exist only at **compile time**. After `tsc` checks your code (or Node strips the types), none of them are left:

```ts
type Port = number;
const port: Port = 443;
console.log(typeof port);          // "number": there's no runtime "Port"
```

So you can't ask "is this value of type `Port`?" while the program runs. To check data from outside (JSON, files, user input) you write runtime checks, as in the narrowing lesson, or use a schema library.

## Structural typing

TypeScript compares types by their **shape**, not their name. If an object has the right properties, it fits:

```ts
type Server = { name: string; ip: string };
type Printer = { name: string; ip: string };

function ping(target: Server): string {
  return `pinging ${target.name} at ${target.ip}`;
}
const office: Printer = { name: "prn01", ip: "10.0.0.40" };
console.log(ping(office));          // fine: a Printer has the same shape as a Server

const extra = { name: "sw01", ip: "10.0.0.2", ports: 48 };
console.log(ping(extra));           // fine: extra properties are allowed when passing a variable
```

This is "duck typing, checked at compile time". In Java or C#, `Printer` and `Server` would be incompatible because their names differ (**nominal typing**). The branded types lesson shows how to get nominal behaviour when you need it.

**Excess property checks** are the one exception: an object **literal** written directly where a type is expected may not have unknown properties, because that's usually a typo:

```ts,norun
ping({ name: "x", ip: "1.1.1.1", prot: 22 });   // error: 'prot' does not exist in type 'Server'
```

## Inference

You rarely need to write types for local variables; TypeScript **infers** them:

```ts
let retries = 3;                     // number
const level = "warn";                // "warn" (a literal type: const can't change)
const hosts = ["web01", "db01"];     // string[]
const up = hosts.map((h) => h.length > 3);   // boolean[]
console.log(retries, level, hosts, up);
```

Annotate **function parameters** (they can't be inferred from inside), exported APIs and places where inference picks something too wide.

## The top and bottom types

| Type | Meaning | Use it for |
|---|---|---|
| `any` | "don't check this" | almost never; it silently spreads |
| `unknown` | "could be anything; check before use" | data from outside: JSON, `catch (e)`, user input |
| `never` | "this can't happen" | exhaustiveness checks, functions that always throw |
| `void` | a function's return value isn't used | callbacks, functions without `return` |

```ts
function parsePort(raw: unknown): number {
  if (typeof raw === "number" && Number.isInteger(raw)) return raw;
  if (typeof raw === "string" && /^\d+$/.test(raw)) return Number(raw);
  throw new Error(`not a port: ${String(raw)}`);
}
console.log(parsePort(8080), parsePort("443"));
```

With `unknown`, TypeScript forces you to narrow before using the value. With `any`, `raw.toFixed()` would compile and crash at runtime.

## Strict mode

`"strict": true` in `tsconfig.json` turns on the checks that make TypeScript worth it. The most important:

- `strictNullChecks`: `null` and `undefined` are separate types, so `string` can't be `null` unless you say `string | null`. This eliminates the most common JavaScript crash.
- `noImplicitAny`: parameters without types are errors instead of silently `any`.
- `useUnknownInCatchVariables`: `catch (e)` gives `e: unknown`.

Also recommended: `noUncheckedIndexedAccess` (array and record lookups may be `undefined`) and `exactOptionalPropertyTypes`.

## Type assertions

`value as Type` tells the compiler "trust me". It changes nothing at runtime and can hide bugs:

```ts
const data = JSON.parse('{"port": "443"}') as { port: number };
console.log(typeof data.port);       // "string": the assertion was a lie
```

Prefer narrowing and validation. The non-null assertion `x!` ("this isn't null") is the same kind of promise.

@@quiz
`type A = { name: string }` and `type B = { name: string }`. Can a `B` be passed where an `A` is expected?
* Yes; TypeScript compares types by shape (structural typing)
- No; the names differ
- Only with a cast
- Only if B extends A
? This is structural, not nominal, typing.

@@quiz
Which type should you give to `JSON.parse` results before validating them?
- any
* unknown
- never
- object
? unknown forces you to check before use.

@@quiz
`const data = JSON.parse(text) as Config;` What does the `as` do at runtime?
- Validates the data
- Converts the data
* Nothing; it only tells the compiler to trust you
- Throws if the data doesn't match
? Use a type guard or schema to actually check.

@@quiz
What does `strictNullChecks` change?
* `null` and `undefined` are no longer allowed in other types unless you include them explicitly
- It removes null from JavaScript
- It makes every property optional
- It checks for null at runtime
? `string | null` must be narrowed before use.

@@flashcards
Type erasure :: types disappear at runtime
Structural typing :: compatibility by shape, not name
Nominal typing :: compatibility by declared name (Java, C#)
Excess property check :: object literals can't have unknown properties
Inference :: TypeScript works out types from values
`unknown` :: any value; must be narrowed before use
`any` :: disables checking; avoid
`never` :: a value that can't exist
`strict` :: tsconfig option enabling the important checks
Type assertion (`as`) :: compile-time claim with no runtime effect

@@exercise Safe JSON reading
Write `readServer(json: string): { name: string; port: number; tags: string[] } | { error: string }`:

- parse the JSON; on a syntax error return `{ error: "invalid JSON" }`
- the result must be an object (not null or an array) else `{ error: "expected an object" }`
- `name`: a non-empty string, else `{ error: "name must be a non-empty string" }`
- `port`: an integer 1–65535, else `{ error: "port must be 1-65535" }`
- `tags`: optional; if present, an array of strings, else `{ error: "tags must be strings" }`; default `[]`

Treat the parsed value as `unknown` and narrow it; don't use `any` or `as`.
@@starter
type Server = { name: string; port: number; tags: string[] };

function readServer(json: string): Server | { error: string } {
  throw new Error("TODO: not done yet");
}

@@tests
check(readServer('{"name":"web01","port":443,"tags":["prod"]}'), { name: "web01", port: 443, tags: ["prod"] }, "valid");
check(readServer('{"name":"db01","port":5432}'), { name: "db01", port: 5432, tags: [] }, "default tags");
check(readServer("{oops"), { error: "invalid JSON" }, "syntax");
check(readServer("[1,2]"), { error: "expected an object" }, "array");
check(readServer('{"name":"","port":1}'), { error: "name must be a non-empty string" }, "empty name");
check(readServer('{"name":"x","port":"443"}'), { error: "port must be 1-65535" }, "port as text");
check(readServer('{"name":"x","port":22,"tags":["a",1]}'), { error: "tags must be strings" }, "bad tags");
@@hint
`let data: unknown; try { data = JSON.parse(json); } catch { return { error: "invalid JSON" }; }`. Then `if (typeof data !== "object" || data === null || Array.isArray(data))`. Read properties with `"name" in data` or by assigning to `const o: Record<string, unknown> = { ...data }` after the object check.
@@solution
type Server = { name: string; port: number; tags: string[] };

function readServer(json: string): Server | { error: string } {
  let data: unknown;
  try {
    data = JSON.parse(json);
  } catch {
    return { error: "invalid JSON" };
  }
  if (typeof data !== "object" || data === null || Array.isArray(data)) return { error: "expected an object" };
  const o: Record<string, unknown> = { ...data };
  const { name, port, tags } = o;
  if (typeof name !== "string" || name === "") return { error: "name must be a non-empty string" };
  if (typeof port !== "number" || !Number.isInteger(port) || port < 1 || port > 65535) return { error: "port must be 1-65535" };
  if (tags === undefined) return { name, port, tags: [] };
  if (!Array.isArray(tags) || !tags.every((t): t is string => typeof t === "string")) return { error: "tags must be strings" };
  return { name, port, tags };
}

@@exercise Exhaustive handling with never
Write `assertNever(x: never): never` (throws `Error("unexpected: <JSON of x>")`) and use it in `ttlSeconds(record: DnsRecord): number`, which returns the default TTL per record type: `A` and `AAAA` → 3600, `CNAME` → 86400, `MX` → `record.priority < 10 ? 7200 : 14400`, `TXT` → 300. The `default` branch must call `assertNever(record)`, so adding a new record type later becomes a compile error until it's handled.

Also write `describe(record: DnsRecord): string` returning `"A web01 -> 10.0.0.5"`, `"MX corp.local -> mail01 (10)"`, etc.: `<type> <name> -> <value>`, adding ` (<priority>)` for MX.
@@starter
type DnsRecord =
  | { type: "A" | "AAAA"; name: string; value: string }
  | { type: "CNAME"; name: string; value: string }
  | { type: "MX"; name: string; value: string; priority: number }
  | { type: "TXT"; name: string; value: string };

function assertNever(x: never): never {
  throw new Error("TODO: not done yet");
}

function ttlSeconds(record: DnsRecord): number {
  throw new Error("TODO: not done yet");
}

function describe(record: DnsRecord): string {
  throw new Error("TODO: not done yet");
}

@@tests
const recs: DnsRecord[] = [
  { type: "A", name: "web01", value: "10.0.0.5" },
  { type: "MX", name: "corp.local", value: "mail01", priority: 10 },
  { type: "MX", name: "corp.local", value: "mail02", priority: 5 },
  { type: "TXT", name: "corp.local", value: "v=spf1 -all" },
  { type: "CNAME", name: "www", value: "web01" },
];
check(recs.map(ttlSeconds), [3600, 14400, 7200, 300, 86400], "ttls");
check(describe(recs[1]), "MX corp.local -> mail01 (10)", "describe MX");
check(describe(recs[0]), "A web01 -> 10.0.0.5", "describe A");
let msg = "";
try { ttlSeconds({ type: "SRV", name: "x", value: "y" } as unknown as DnsRecord); } catch (e) { msg = (e as Error).message; }
check(msg, 'unexpected: {"type":"SRV","name":"x","value":"y"}', "runtime safety net");
@@hint
`switch (record.type) { case "A": case "AAAA": return 3600; ... default: return assertNever(record); }`. Inside `case "MX":` TypeScript knows `record.priority` exists.
@@solution
type DnsRecord =
  | { type: "A" | "AAAA"; name: string; value: string }
  | { type: "CNAME"; name: string; value: string }
  | { type: "MX"; name: string; value: string; priority: number }
  | { type: "TXT"; name: string; value: string };

function assertNever(x: never): never {
  throw new Error(`unexpected: ${JSON.stringify(x)}`);
}

function ttlSeconds(record: DnsRecord): number {
  switch (record.type) {
    case "A":
    case "AAAA":
      return 3600;
    case "CNAME":
      return 86400;
    case "MX":
      return record.priority < 10 ? 7200 : 14400;
    case "TXT":
      return 300;
    default:
      return assertNever(record);
  }
}

function describe(record: DnsRecord): string {
  const base = `${record.type} ${record.name} -> ${record.value}`;
  return record.type === "MX" ? `${base} (${record.priority})` : base;
}

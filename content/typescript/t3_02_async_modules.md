@@lesson t3-async-modules Async Code, APIs and Modules
@@concepts concurrency, networking, modules, json
@@text
## Promises with types

An `async` function returns a `Promise<T>`; `await` unwraps it to `T`.

```typescript
const sleep = (ms: number): Promise<void> => new Promise((resolve) => setTimeout(resolve, ms));

async function check(host: string, ms: number): Promise<{ host: string; up: boolean }> {
  await sleep(ms);
  return { host, up: !host.startsWith("old") };
}

const results = await Promise.all([check("web01", 30), check("old-ftp", 10), check("db01", 20)]);
console.log(results.filter((r) => r.up).map((r) => r.host));
```

- `Promise.all` runs them at the same time and fails if any fails.
- `Promise.allSettled` waits for all and tells you which failed. Use it for health checks, where one failure shouldn't hide the rest.
- `Promise.race`/`Promise.any` return the first to finish/succeed.

## Typing API responses

`fetch` gives you `Promise<Response>`, and `response.json()` returns `Promise<any>`. Don't trust it: the server can send anything. Type it as `unknown` and validate:

```typescript,norun
type Health = { status: "ok" | "degraded"; version: string };

function isHealth(x: unknown): x is Health {
  const o = x as Health;
  return typeof x === "object" && x !== null && (o.status === "ok" || o.status === "degraded") && typeof o.version === "string";
}

async function getHealth(url: string): Promise<Health> {
  const res = await fetch(url, { signal: AbortSignal.timeout(5000) });
  if (!res.ok) throw new Error(`${url}: HTTP ${res.status}`);
  const body: unknown = await res.json();
  if (!isHealth(body)) throw new Error(`${url}: unexpected response`);
  return body;
}
```

`AbortSignal.timeout(5000)` cancels the request after 5 seconds. Without a timeout, a hung server can hang your tool forever.

## Modules

Each file is a **module**. `export` what other files may use, and `import` it:

```typescript,norun
// subnet.ts
export type Cidr = { network: number; prefix: number };
export function parseCidr(text: string): Cidr { /* ... */ }
export const MAX_PREFIX = 32;

// main.ts
import { parseCidr, MAX_PREFIX } from "./subnet.ts";
import type { Cidr } from "./subnet.ts";        // a type-only import: erased completely
import { readFile } from "node:fs/promises";     // Node's built-in modules
import path from "node:path";
```

- Use `import type` for types: it makes clear that nothing is loaded at run time (and type stripping requires it when the import is only a type).
- With Node running `.ts` directly, write the **`.ts` extension** in relative imports. In `tsconfig.json`, `"allowImportingTsExtensions": true` with `"noEmit": true` lets `tsc` accept that.
- `package.json` with `"type": "module"` makes `.js`/`.ts` files ES modules.

## Reading and writing JSON files

```typescript
import { writeFile, readFile, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";

type Inventory = { hosts: { name: string; ip: string }[] };

const file = join(tmpdir(), "itquest-inventory.json");
const inv: Inventory = { hosts: [{ name: "web01", ip: "10.0.0.11" }] };
await writeFile(file, JSON.stringify(inv, null, 2));
const back = JSON.parse(await readFile(file, "utf8")) as Inventory;   // trusted: we just wrote it
console.log(back.hosts[0].name);
await rm(file);
```

`as Inventory` is a **type assertion**: "trust me". It does no checking, so use it only for data you control; validate everything else.

@@quiz
What type does an `async function f(): Promise<number>` give you after `await f()`?
* `number`
- `Promise<number>`
- `any`
- `void`
? `await` unwraps the promise.

@@quiz
Five health checks run with `Promise.all`; one rejects. What happens?
* The whole `Promise.all` rejects, and you don't get the other four results
- You get four results and one error
- It waits forever
- The failed one returns `undefined`
? Use `Promise.allSettled` when you need every result, failures included.

@@quiz
What's the risk of `const data = (await res.json()) as User[];`?
* No checking happens: if the API sends something else, the types lie and the code breaks later
- It's slower
- It doesn't compile
- It deletes the response
? Assertions are promises to the compiler. Validate untrusted data with a type guard or Zod.

@@quiz
Why write `import type { Cidr } from "./subnet.ts"`?
* The import is only a type, so it's erased completely, and nothing is loaded at run time
- It's required for every import
- It imports the type as a value
- It makes the import faster to type
? Type-only imports keep the runtime and the type layers clearly apart.

@@exercise Check them all
Write `checkAll(hosts, probe)` that runs `probe` on every host **at the same time** and returns `{ up: string[]; down: string[] }`. A probe that **rejects** counts as down (use `Promise.allSettled`). Keep the hosts' original order in each list.
@@starter
type Probe = (host: string) => Promise<boolean>;

async function checkAll(hosts: string[], probe: Probe): Promise<{ up: string[]; down: string[] }> {
  throw new Error("TODO: not done yet");
}

@@tests
const probe: Probe = async (h) => {
  await new Promise((r) => setTimeout(r, 10));
  if (h === "broken") throw new Error("DNS failure");
  return !h.startsWith("old");
};
const start = Date.now();
const r = await checkAll(["web01", "old-ftp", "broken", "db01"], probe);
check(r, { up: ["web01", "db01"], down: ["old-ftp", "broken"] }, "sorted into up and down");
check(Date.now() - start < 35, true, "ran at the same time (not one after another)");
@@hint
`const settled = await Promise.allSettled(hosts.map(probe));` Then loop with the index: a host is up when `settled[i].status === "fulfilled" && settled[i].value`.
@@solution
type Probe = (host: string) => Promise<boolean>;

async function checkAll(hosts: string[], probe: Probe): Promise<{ up: string[]; down: string[] }> {
  const settled = await Promise.allSettled(hosts.map(probe));
  const up: string[] = [];
  const down: string[] = [];
  settled.forEach((s, i) => (s.status === "fulfilled" && s.value ? up : down).push(hosts[i]));
  return { up, down };
}

@@exercise Timeout wrapper
Write a generic `withTimeout<T>(p: Promise<T>, ms: number): Promise<T>` that resolves with `p`'s value, or rejects with `new Error("timeout after <ms> ms")` if `p` takes longer than `ms`.
@@starter
function withTimeout<T>(p: Promise<T>, ms: number): Promise<T> {
  throw new Error("TODO: not done yet");
}

@@tests
const slow = (ms: number, v: string) => new Promise<string>((r) => setTimeout(() => r(v), ms));
check(await withTimeout(slow(5, "fast"), 100), "fast", "resolves in time");
let msg = "";
try { await withTimeout(slow(200, "late"), 20); } catch (e) { msg = (e as Error).message; }
check(msg, "timeout after 20 ms", "rejects when too slow");
@@hint
`Promise.race([p, new Promise<never>((_, reject) => setTimeout(() => reject(new Error(...)), ms))])`. For tidiness, clear the timer when `p` settles.
@@solution
function withTimeout<T>(p: Promise<T>, ms: number): Promise<T> {
  let timer: ReturnType<typeof setTimeout> | undefined;
  const timeout = new Promise<never>((_, reject) => {
    timer = setTimeout(() => reject(new Error(`timeout after ${ms} ms`)), ms);
  });
  return Promise.race([p, timeout]).finally(() => clearTimeout(timer));
}

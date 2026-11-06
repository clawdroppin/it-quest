@@lesson t7-declarations Declaration Files, Ambient Types and Module Augmentation
@@concepts modules
@@text
Most npm packages are written in JavaScript. TypeScript knows their types through **declaration files** (`.d.ts`): type-only files that describe what a JavaScript module exports. You'll read them constantly and occasionally write them.

## What a .d.ts looks like

```ts,norun
// node_modules/tiny-ping/index.d.ts
export interface PingResult { host: string; alive: boolean; ms?: number }
export interface PingOptions { timeoutMs?: number; count?: number }
export function ping(host: string, options?: PingOptions): Promise<PingResult>;
export default ping;
```

No function bodies, just shapes. Where they come from:

1. **Bundled** with the package (`"types": "index.d.ts"` in its package.json): most modern packages.
2. **DefinitelyTyped**: community types installed separately, e.g. `npm i -D @types/node @types/express`.
3. **Generated** by `tsc --declaration` from your own TypeScript (that's how you publish a library).
4. **Written by you** for an untyped package or internal JavaScript.

## Ambient declarations

`declare` says "this exists at runtime; trust me about its type". Nothing is emitted:

```ts,norun
declare const __APP_VERSION__: string;          // injected by the bundler
declare function gtag(event: string, data?: Record<string, unknown>): void;   // from a <script> tag

declare module "legacy-inventory" {             // types for an untyped package
  export function lookup(tag: string): { tag: string; owner?: string } | null;
}

declare module "*.svg" {                        // non-code imports handled by a bundler
  const url: string;
  export default url;
}
```

A quick escape hatch for an untyped package is `declare module "some-package";`, which types everything as `any`. Replace it with real types as soon as you use the package seriously.

## Global augmentation

Add properties to globals (in a module file, wrap in `declare global`):

```ts,norun
declare global {
  interface Window { __featureFlags?: Record<string, boolean> }
  namespace NodeJS { interface ProcessEnv { DATABASE_URL: string; LOG_LEVEL?: "debug" | "info" } }
}
export {};
```

The `ProcessEnv` trick gives you typed `process.env.DATABASE_URL`.

## Module augmentation and declaration merging

Interfaces with the same name **merge**. Libraries expose interfaces on purpose so you can extend them:

```ts,norun
// Add a typed field to Express's Request
declare module "express-serve-static-core" {
  interface Request { user?: { id: string; roles: string[] } }
}
```

You can use the same pattern for your own extensible registries, which is how many plugin systems are typed:

```ts
interface CommandRegistry {
  restart: { service: string };
}
interface CommandRegistry {                       // could live in another file (a plugin)
  backup: { target: string; full: boolean };
}
function run<K extends keyof CommandRegistry>(name: K, args: CommandRegistry[K]): string {
  return `${name} ${JSON.stringify(args)}`;
}
console.log(run("backup", { target: "db01", full: true }));
```

## Typing JavaScript gradually

You don't have to convert a codebase at once:

- `allowJs` + `checkJs` type-check `.js` files using **JSDoc** comments: `/** @param {string} host @returns {Promise<boolean>} */`.
- `// @ts-check` at the top of one JS file opts it in.
- Convert leaf modules first (utilities with no imports), then work inwards.
- Turn on `strict` per directory with separate tsconfig files (project references) if needed.

## Wrapping untyped code

When you depend on loosely typed code, put **one typed wrapper** around it and validate at that boundary, so `any` doesn't leak into the rest of the app:

```ts
const legacy: any = { find: (t: string) => (t === "IT-1" ? { tag: "IT-1", owner: "ana" } : null) };

interface Asset { tag: string; owner: string | null }
function findAsset(tag: string): Asset | null {
  const raw: unknown = legacy.find(tag);
  if (typeof raw !== "object" || raw === null || !("tag" in raw)) return null;
  const owner = "owner" in raw && typeof raw.owner === "string" ? raw.owner : null;
  return { tag: String(raw.tag), owner };
}
console.log(findAsset("IT-1"), findAsset("IT-9"));
```

Note how the result of the `any` call is immediately assigned to `unknown`: that stops `any` from spreading.

@@quiz
Where do the types for an npm package like `express` usually come from if it doesn't ship them?
- They're generated at install
* DefinitelyTyped, installed as `@types/express`
- TypeScript guesses them
- They can't be used from TypeScript
? `npm i -D @types/express`

@@quiz
What does `declare const VERSION: string;` emit in the JavaScript output?
* Nothing; it only informs the type checker
- `const VERSION = ""`
- `var VERSION`
- An import
? Declarations are type-only.

@@quiz
How do you add a field to a library's interface, like Express's Request?
- Edit node_modules
* Module augmentation: `declare module "..." { interface Request { ... } }`
- Subclass it
- Use any
? Interfaces with the same name merge.

@@quiz
Why assign the result of an `any` call to `unknown` immediately?
- It's faster
* To stop `any` from spreading and force validation at the boundary
- unknown is shorter
- TypeScript requires it
? Contain `any` in one wrapper.

@@drill
Which file extension holds type declarations only?
= .d.ts
= d.ts
? No runtime code.

@@flashcards
`.d.ts` :: a declaration (types-only) file
DefinitelyTyped :: community types published as `@types/*`
`declare` :: ambient declaration: exists at runtime, no emit
`declare module "x"` :: types for an untyped module
`declare global` :: augment global types from a module
Declaration merging :: same-named interfaces combine
Module augmentation :: add to a library's interfaces
`checkJs` + JSDoc :: type-check JavaScript files
`tsc --declaration` :: generate .d.ts for your library

@@exercise A typed wrapper for a legacy module
A legacy JavaScript inventory module (given as `legacyDb: any`) returns loosely shaped records: `serial` may be a number or string, `assigned` is a string or missing, `purchased` is an ISO date string, and the `type` field uses codes `"LT"`, `"MN"`, `"PH"`.

Write a typed wrapper:

- `type DeviceKind = "laptop" | "monitor" | "phone"` and `interface Device { serial: string; kind: DeviceKind; assignedTo: string | null; purchased: Date }`
- `toDevice(raw: unknown): Device | null`: returns `null` for anything that doesn't fit (unknown type code, missing or invalid date, missing serial)
- `listDevices(): Device[]`: all valid devices from `legacyDb.all()`, sorted by serial
- `devicesOlderThan(years: number, now: Date): Device[]`: purchased more than `years * 365` days before `now`
@@starter
const legacyDb: any = {
  all: () => [
    { serial: 1042, type: "LT", assigned: "ana", purchased: "2021-03-01" },
    { serial: "M-77", type: "MN", purchased: "2024-11-20" },
    { serial: "P-5", type: "XX", purchased: "2023-01-01" },
    { type: "PH", purchased: "2022-01-01" },
    { serial: "P-9", type: "PH", assigned: "kenji", purchased: "not a date" },
    { serial: "1001", type: "PH", assigned: "bo", purchased: "2019-06-15" },
  ],
};

type DeviceKind = "laptop" | "monitor" | "phone";
interface Device { serial: string; kind: DeviceKind; assignedTo: string | null; purchased: Date }

function toDevice(raw: unknown): Device | null {
  throw new Error("TODO: not done yet");
}

function listDevices(): Device[] {
  throw new Error("TODO: not done yet");
}

function devicesOlderThan(years: number, now: Date): Device[] {
  throw new Error("TODO: not done yet");
}

@@tests
const all = listDevices();
check(all.map((d) => `${d.serial}:${d.kind}:${d.assignedTo ?? "-"}`), ["1001:phone:bo", "1042:laptop:ana", "M-77:monitor:-"], "valid devices");
check(all[0]?.purchased.toISOString().slice(0, 10), "2019-06-15", "dates parsed");
check(devicesOlderThan(4, new Date("2025-06-01")).map((d) => d.serial), ["1001", "1042"], "older than 4 years");
check(toDevice("junk"), null, "junk rejected");
@@hint
After the object check, copy into `const r: Record<string, unknown> = { ...raw }`. Map codes with `const KINDS: Record<string, DeviceKind> = { LT: "laptop", MN: "monitor", PH: "phone" }`. A date is invalid when `Number.isNaN(date.getTime())`. `listDevices`: `const raws: unknown[] = legacyDb.all();` then map, filter out nulls with a type predicate, and sort with `localeCompare`.
@@solution
const legacyDb: any = {
  all: () => [
    { serial: 1042, type: "LT", assigned: "ana", purchased: "2021-03-01" },
    { serial: "M-77", type: "MN", purchased: "2024-11-20" },
    { serial: "P-5", type: "XX", purchased: "2023-01-01" },
    { type: "PH", purchased: "2022-01-01" },
    { serial: "P-9", type: "PH", assigned: "kenji", purchased: "not a date" },
    { serial: "1001", type: "PH", assigned: "bo", purchased: "2019-06-15" },
  ],
};

type DeviceKind = "laptop" | "monitor" | "phone";
interface Device { serial: string; kind: DeviceKind; assignedTo: string | null; purchased: Date }

const KINDS: Record<string, DeviceKind> = { LT: "laptop", MN: "monitor", PH: "phone" };

function toDevice(raw: unknown): Device | null {
  if (typeof raw !== "object" || raw === null) return null;
  const r: Record<string, unknown> = { ...raw };
  const serial = typeof r.serial === "number" || typeof r.serial === "string" ? String(r.serial) : null;
  const kind = typeof r.type === "string" ? KINDS[r.type] : undefined;
  const purchased = typeof r.purchased === "string" ? new Date(r.purchased) : null;
  if (serial === null || kind === undefined || purchased === null || Number.isNaN(purchased.getTime())) return null;
  return { serial, kind, assignedTo: typeof r.assigned === "string" ? r.assigned : null, purchased };
}

function listDevices(): Device[] {
  const raws: unknown[] = legacyDb.all();
  return raws
    .map(toDevice)
    .filter((d): d is Device => d !== null)
    .sort((a, b) => a.serial.localeCompare(b.serial));
}

function devicesOlderThan(years: number, now: Date): Device[] {
  const limit = years * 365 * 24 * 60 * 60 * 1000;
  return listDevices().filter((d) => now.getTime() - d.purchased.getTime() > limit);
}

@@exercise An extensible plugin registry
Plugins describe their settings by **merging** into an interface:

```ts,norun
interface PluginSettings { backup: { target: string; keepDays: number } }
interface PluginSettings { notify: { channel: string } }
```

Write a `PluginHost` class:

- `register<K extends keyof PluginSettings>(name: K, plugin: { defaults: PluginSettings[K]; run(settings: PluginSettings[K]): string }): this`: registering a name twice throws `Error("plugin <name> already registered")`
- `configure<K extends keyof PluginSettings>(name: K, overrides: Partial<PluginSettings[K]>): this`: merges overrides over the defaults (later calls merge over earlier ones); unknown plugin → `Error("unknown plugin <name>")`
- `runAll(): string[]`: runs every plugin in registration order with its effective settings
@@starter
interface PluginSettings { backup: { target: string; keepDays: number } }
interface PluginSettings { notify: { channel: string } }

class PluginHost {
  register<K extends keyof PluginSettings>(name: K, plugin: { defaults: PluginSettings[K]; run(settings: PluginSettings[K]): string }): this {
    throw new Error("TODO: not done yet");
  }

  configure<K extends keyof PluginSettings>(name: K, overrides: Partial<PluginSettings[K]>): this {
    throw new Error("TODO: not done yet");
  }

  runAll(): string[] {
    throw new Error("TODO: not done yet");
  }
}

@@tests
const host = new PluginHost()
  .register("backup", { defaults: { target: "/srv/backup", keepDays: 7 }, run: (s) => `backup to ${s.target}, keep ${s.keepDays}d` })
  .register("notify", { defaults: { channel: "#ops" }, run: (s) => `notify ${s.channel}` })
  .configure("backup", { keepDays: 30 })
  .configure("backup", { target: "/mnt/nas" });
check(host.runAll(), ["backup to /mnt/nas, keep 30d", "notify #ops"], "run all");
let msg = "";
try { host.register("notify", { defaults: { channel: "x" }, run: () => "" }); } catch (e) { msg = (e as Error).message; }
check(msg, "plugin notify already registered", "duplicate");
try { new PluginHost().configure("backup", {}); } catch (e) { msg = (e as Error).message; }
check(msg, "unknown plugin backup", "unknown");
function _typeTests(): void {
  // @ts-expect-error keepDays must be a number
  new PluginHost().configure("backup", { keepDays: "30" });
  // @ts-expect-error no such plugin in the registry
  new PluginHost().configure("deploy", {});
}
@@hint
Internally, store entries in a `Map<string, { settings: Record<string, unknown>; run: (s: Record<string, unknown>) => string }>`. When storing, convert the typed plugin once: `run: plugin.run as unknown as (s: Record<string, unknown>) => string`. The public methods stay fully typed; the loose types live only inside the class.
@@solution
interface PluginSettings { backup: { target: string; keepDays: number } }
interface PluginSettings { notify: { channel: string } }

interface Entry {
  settings: Record<string, unknown>;
  run: (settings: Record<string, unknown>) => string;
}

class PluginHost {
  private readonly entries = new Map<string, Entry>();

  register<K extends keyof PluginSettings>(name: K, plugin: { defaults: PluginSettings[K]; run(settings: PluginSettings[K]): string }): this {
    if (this.entries.has(name)) throw new Error(`plugin ${name} already registered`);
    this.entries.set(name, {
      settings: { ...plugin.defaults },
      run: plugin.run as unknown as (settings: Record<string, unknown>) => string,
    });
    return this;
  }

  configure<K extends keyof PluginSettings>(name: K, overrides: Partial<PluginSettings[K]>): this {
    const entry = this.entries.get(name);
    if (!entry) throw new Error(`unknown plugin ${name}`);
    entry.settings = { ...entry.settings, ...overrides };
    return this;
  }

  runAll(): string[] {
    return [...this.entries.values()].map((e) => e.run(e.settings));
  }
}

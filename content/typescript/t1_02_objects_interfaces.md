@@lesson t1-objects Object Types, Interfaces, Index Signatures and Records
@@concepts objects, maps
@@text
Most TypeScript code describes objects: API responses, config files, database rows, component props. This lesson covers every tool for that.

## type vs interface

```ts
type ServerT = { name: string; ip: string };
interface ServerI { name: string; ip: string }

interface Monitored extends ServerI { checkEvery: number }   // extend
type MonitoredT = ServerT & { checkEvery: number };          // intersection does the same

const s: Monitored = { name: "web01", ip: "10.0.0.5", checkEvery: 60 };
console.log(s);
```

They're nearly interchangeable for objects. Differences:

- Only `type` can name unions, tuples, primitives and computed types: `type Status = "up" | "down"`.
- Interfaces can be **reopened** (declaration merging): two `interface Window {}` declarations combine. Libraries use this to let you add fields (module augmentation).
- Error messages for interfaces can be shorter.

A common convention: `interface` for object shapes that others may extend, `type` for everything else. Pick one style per codebase.

## Optional, readonly and defaults

```ts
interface Ticket {
  readonly id: number;          // can't be reassigned after creation
  title: string;
  assignee?: string;            // may be missing: type is string | undefined
  tags: readonly string[];      // the array can't be modified (no push)
}
const t: Ticket = { id: 1, title: "VPN down", tags: ["network"] };
console.log(t.assignee ?? "unassigned", t.tags.length);
```

`readonly` is compile-time only (like all types); use `Object.freeze` if you need runtime protection. `Readonly<T>` makes every property readonly.

## Index signatures and Record

When keys aren't known in advance (a lookup table, counts, a JSON map), use an **index signature** or `Record`:

```ts
const ownerOf: Record<string, string> = { web01: "ana", db01: "kenji" };
const portNames: { [port: number]: string } = { 22: "ssh", 443: "https" };
type Level = "debug" | "info" | "warn" | "error";
const counts: Record<Level, number> = { debug: 0, info: 12, warn: 3, error: 1 };   // every key required
console.log(ownerOf.web01, portNames[443], counts.warn);
```

`Record<Level, number>` with a **union** of keys requires every key: a great way to make sure a table covers all cases. With `noUncheckedIndexedAccess`, `ownerOf.mail01` has type `string | undefined`, which reminds you the key may be missing. For truly dynamic dictionaries, `Map<K, V>` is often better.

## Nested and recursive types

```ts
interface OrgUnit {
  name: string;
  members: string[];
  children: OrgUnit[];                 // a type can refer to itself
}
const corp: OrgUnit = { name: "Corp", members: [], children: [{ name: "IT", members: ["ana"], children: [] }] };

type Json = string | number | boolean | null | Json[] | { [key: string]: Json };
const payload: Json = { ok: true, items: [1, "two", { three: null }] };
console.log(corp.children[0]?.name, JSON.stringify(payload));
```

## Methods and function properties

```ts,norun
interface Storage {
  get(key: string): string | undefined;            // method syntax
  set: (key: string, value: string) => void;       // property holding a function
}
```

The two forms behave almost the same; method syntax is checked slightly more loosely (bivariantly) for parameters, which the variance lesson explains.

## Picking types from values

```ts
const defaults = { host: "localhost", port: 8080, tls: false };
type Settings = typeof defaults;                   // { host: string; port: number; tls: boolean }
type SettingKey = keyof Settings;                  // "host" | "port" | "tls"
function getSetting(key: SettingKey): Settings[SettingKey] {
  return defaults[key];
}
console.log(getSetting("port"));
```

`typeof value` gives a value's type, `keyof Type` the union of its keys, and `Type[Key]` the type of a property ("indexed access"). The type system in depth module builds on these three.

@@quiz
What can a `type` alias do that an `interface` can't?
* Name unions, tuples and other non-object types
- Have optional properties
- Be extended
- Describe methods
? `type Status = "up" | "down"` has no interface equivalent.

@@quiz
What does `Record<"info" | "warn" | "error", number>` require?
- Any string keys
* An object with exactly those three keys, each a number
- An array of numbers
- At least one of the keys
? Union keys must all be present: perfect for exhaustive lookup tables.

@@quiz
What does `readonly` on a property do at runtime?
- Freezes the object
* Nothing; it's a compile-time check only
- Makes the property private
- Throws on assignment
? Use Object.freeze for runtime immutability.

@@flashcards
interface :: named object shape that can be extended and merged
Intersection `A & B` :: has all properties of A and B
Optional property `x?:` :: may be missing (`T | undefined`)
`readonly` :: property can't be reassigned (compile time)
Index signature :: `{ [key: string]: T }` for unknown keys
`Record<K, V>` :: object type with keys K and values V
`typeof value` :: the type of a value
`keyof T` :: union of T's keys
`T[K]` :: indexed access: the type of property K
Declaration merging :: interfaces with the same name combine

@@exercise A typed inventory index
Write:

- `interface Asset { readonly tag: string; kind: AssetKind; owner?: string; specs: Record<string, string | number> }` with `type AssetKind = "laptop" | "monitor" | "phone"`
- `countByKind(assets: readonly Asset[]): Record<AssetKind, number>` (every kind present, even with 0)
- `byOwner(assets: readonly Asset[]): Map<string, string[]>`: owner → tags (sorted); assets without an owner go under `"(unassigned)"`
- `specOf(asset: Asset, key: string): string`: the spec value as text, or `"n/a"` if missing
@@starter
type AssetKind = "laptop" | "monitor" | "phone";

interface Asset {
  readonly tag: string;
  kind: AssetKind;
  owner?: string;
  specs: Record<string, string | number>;
}

function countByKind(assets: readonly Asset[]): Record<AssetKind, number> {
  throw new Error("TODO: not done yet");
}

function byOwner(assets: readonly Asset[]): Map<string, string[]> {
  throw new Error("TODO: not done yet");
}

function specOf(asset: Asset, key: string): string {
  throw new Error("TODO: not done yet");
}

@@tests
const assets: Asset[] = [
  { tag: "IT-3", kind: "laptop", owner: "ana", specs: { ramGb: 16, cpu: "i7" } },
  { tag: "IT-1", kind: "monitor", owner: "ana", specs: { inches: 27 } },
  { tag: "IT-2", kind: "laptop", specs: {} },
];
check(countByKind(assets), { laptop: 2, monitor: 1, phone: 0 }, "counts include zero");
check([...byOwner(assets)], [["ana", ["IT-1", "IT-3"]], ["(unassigned)", ["IT-2"]]], "by owner");
check([specOf(assets[0]!, "ramGb"), specOf(assets[0]!, "gpu")], ["16", "n/a"], "specs");
@@hint
Start the counts with `{ laptop: 0, monitor: 0, phone: 0 }`. For the map: `const key = a.owner ?? "(unassigned)"; map.set(key, [...(map.get(key) ?? []), a.tag]);` then sort each list.
@@solution
type AssetKind = "laptop" | "monitor" | "phone";

interface Asset {
  readonly tag: string;
  kind: AssetKind;
  owner?: string;
  specs: Record<string, string | number>;
}

function countByKind(assets: readonly Asset[]): Record<AssetKind, number> {
  const counts: Record<AssetKind, number> = { laptop: 0, monitor: 0, phone: 0 };
  for (const a of assets) counts[a.kind]++;
  return counts;
}

function byOwner(assets: readonly Asset[]): Map<string, string[]> {
  const map = new Map<string, string[]>();
  for (const a of assets) {
    const key = a.owner ?? "(unassigned)";
    map.set(key, [...(map.get(key) ?? []), a.tag]);
  }
  for (const tags of map.values()) tags.sort();
  return map;
}

function specOf(asset: Asset, key: string): string {
  const value = asset.specs[key];
  return value === undefined ? "n/a" : String(value);
}

@@exercise A recursive org chart
Using `interface OrgUnit { name: string; members: string[]; children: OrgUnit[] }`, write:

- `headcount(unit: OrgUnit): number`: members of the unit and all its descendants
- `findPath(unit: OrgUnit, person: string): string[] | null`: unit names from the root to the unit containing `person` (first match, depth-first), or `null`
- `flattenUnits(unit: OrgUnit, depth = 0): { name: string; depth: number }[]`: every unit in depth-first order with its depth
@@starter
interface OrgUnit {
  name: string;
  members: string[];
  children: OrgUnit[];
}

function headcount(unit: OrgUnit): number {
  throw new Error("TODO: not done yet");
}

function findPath(unit: OrgUnit, person: string): string[] | null {
  throw new Error("TODO: not done yet");
}

function flattenUnits(unit: OrgUnit, depth = 0): { name: string; depth: number }[] {
  throw new Error("TODO: not done yet");
}

@@tests
const corp: OrgUnit = {
  name: "Corp", members: ["ceo"], children: [
    { name: "IT", members: ["cio"], children: [
      { name: "Service Desk", members: ["ana", "bo"], children: [] },
      { name: "Infrastructure", members: ["kenji"], children: [] },
    ] },
    { name: "Sales", members: ["dee", "eve", "fay"], children: [] },
  ],
};
check(headcount(corp), 8, "headcount");
check(findPath(corp, "kenji"), ["Corp", "IT", "Infrastructure"], "path");
check(findPath(corp, "nobody"), null, "not found");
check(flattenUnits(corp).map((u) => `${"-".repeat(u.depth)}${u.name}`), ["Corp", "-IT", "--Service Desk", "--Infrastructure", "-Sales"], "flatten");
@@hint
Recursion mirrors the type: `unit.members.length + unit.children.reduce((sum, c) => sum + headcount(c), 0)`. For the path, check `unit.members.includes(person)`, then each child's path, prepending `unit.name`.
@@solution
interface OrgUnit {
  name: string;
  members: string[];
  children: OrgUnit[];
}

function headcount(unit: OrgUnit): number {
  return unit.members.length + unit.children.reduce((sum, c) => sum + headcount(c), 0);
}

function findPath(unit: OrgUnit, person: string): string[] | null {
  if (unit.members.includes(person)) return [unit.name];
  for (const child of unit.children) {
    const path = findPath(child, person);
    if (path) return [unit.name, ...path];
  }
  return null;
}

function flattenUnits(unit: OrgUnit, depth = 0): { name: string; depth: number }[] {
  return [{ name: unit.name, depth }, ...unit.children.flatMap((c) => flattenUnits(c, depth + 1))];
}

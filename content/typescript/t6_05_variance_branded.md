@@lesson t6-variance Variance, Assignability and Branded Types
@@concepts functions
@@text
When is a `Box<Laptop>` usable where a `Box<Asset>` is expected? The answer is **variance**, and it explains many confusing errors. This lesson also shows how to opt out of structural typing with **branded types** when two strings must never be mixed up.

## Assignability recap

`A` is assignable to `B` when every `A` value is a valid `B`. A subtype has **at least** the properties of its supertype:

```ts
interface Asset { tag: string }
interface Laptop extends Asset { ramGb: number }
const l: Laptop = { tag: "IT-1", ramGb: 16 };
const a: Asset = l;          // fine: a Laptop is an Asset
console.log(a.tag);
```

## Covariance: producers

Things you only **read from** keep the direction: if `Laptop` → `Asset`, then `() => Laptop` → `() => Asset` and `readonly Laptop[]` → `readonly Asset[]`. That's **covariance**.

## Contravariance: consumers

Things you **pass values into** reverse the direction. A function that handles any `Asset` can be used where a `Laptop` handler is needed, but not the other way round:

```ts
interface Asset { tag: string }
interface Laptop extends Asset { ramGb: number }
type Handler<T> = (item: T) => void;

const logAsset: Handler<Asset> = (a) => console.log("asset", a.tag);
const laptopHandler: Handler<Laptop> = logAsset;     // fine: it only needs .tag
// const assetHandler: Handler<Asset> = (l: Laptop) => console.log(l.ramGb);   // error
laptopHandler({ tag: "IT-2", ramGb: 8 });
```

The commented-out line would crash when given a monitor that has no `ramGb`. With `strictFunctionTypes` (part of `strict`), TypeScript catches it.

## The array hole: method bivariance

Mutable arrays are both read and written, so in theory they should be **invariant**. TypeScript allows `Laptop[]` → `Asset[]` anyway for convenience, which lets this bug through:

```ts
interface Asset { tag: string }
interface Laptop extends Asset { ramGb: number }
const laptops: Laptop[] = [{ tag: "IT-1", ramGb: 16 }];
const assets: Asset[] = laptops;          // allowed
assets.push({ tag: "MON-1" });            // a monitor sneaks into laptops!
console.log(laptops[1]?.ramGb);           // undefined at runtime, though the type says number
```

Defence: accept `readonly T[]` in functions that only read. Also, parameters declared with **method syntax** (`handle(x: T): void`) are checked bivariantly (loosely); **property syntax** (`handle: (x: T) => void`) is checked strictly.

## Variance annotations

For generic types you can state the variance with `out` (covariant) and `in` (contravariant). It documents intent and makes checks faster in big codebases:

```ts
interface Producer<out T> { next(): T }
interface Consumer<in T> { accept(value: T): void }
type Comparator<in T> = (a: T, b: T) => number;
const byTag: Comparator<{ tag: string }> = (a, b) => a.tag.localeCompare(b.tag);
console.log([{ tag: "b" }, { tag: "a" }].sort(byTag));
```

## Branded types

Structural typing means any `string` fits any other `string`. But a `UserId` and an `AssetTag` are both strings, and swapping them is a real bug. A **brand** makes them incompatible at compile time with no runtime cost:

```ts
declare const brand: unique symbol;
type Brand<T, B extends string> = T & { readonly [brand]: B };

type UserId = Brand<string, "UserId">;
type AssetTag = Brand<string, "AssetTag">;

function userId(raw: string): UserId {
  if (!/^u\d+$/.test(raw)) throw new Error(`bad user id: ${raw}`);
  return raw as UserId;                  // the only place we assert
}
function assignTo(tag: AssetTag, owner: UserId): string { return `${tag} -> ${owner}`; }

const owner = userId("u42");
// assignTo(owner, owner);               // error: UserId isn't AssetTag
console.log(owner.toUpperCase());        // still a normal string at runtime
```

The function that creates a branded value (the **smart constructor**) is the only place that validates and asserts. Everything downstream can trust the type: "parse, don't validate". Brands are great for IDs, validated emails and IPs, units (`Meters` vs `Feet`), and sanitized HTML.

@@quiz
`type Handler<T> = (item: T) => void`. If Laptop extends Asset, which is allowed?
* Using a `Handler<Asset>` where a `Handler<Laptop>` is expected
- Using a `Handler<Laptop>` where a `Handler<Asset>` is expected
- Both
- Neither
? Function parameters are contravariant.

@@quiz
Why accept `readonly Asset[]` instead of `Asset[]` in a function that only reads?
- It's faster
* Readonly arrays are safely covariant, and callers know you won't modify their array
- Mutable arrays can't hold subtypes
- readonly is required by strict mode
? Mutable arrays are unsoundly covariant in TypeScript.

@@quiz
What does a branded type cost at runtime?
* Nothing; the brand exists only in the type system
- An extra property on every value
- A wrapper object
- A Symbol per value
? The brand is erased; only the smart constructor's check runs.

@@drill
Function parameter types vary in the opposite direction to their function types. What's this called?
= contravariance
= contravariant
? A handler for Animal can stand in for a handler for Dog.

@@flashcards
Assignability :: A fits where B is expected
Covariance :: same direction (outputs, readonly)
Contravariance :: reversed direction (inputs, parameters)
Invariance :: no subtyping either way (read and write)
`strictFunctionTypes` :: checks function parameters contravariantly
Method bivariance :: method-syntax parameters checked loosely
`in` / `out` :: variance annotations on type parameters
Branded type :: `T & { [brand]: "Name" }` for nominal typing
Smart constructor :: validates once and returns the branded type

@@exercise Branded network values
Using `type Brand<T, B extends string> = T & { readonly [brand]: B }` (with `declare const brand: unique symbol`), define `Ipv4`, `Port` and `Hostname` and write smart constructors:

- `ipv4(raw: string): Ipv4`: four dot-separated integers 0–255 without leading zeros (except `"0"`); else `Error("invalid IPv4: <raw>")`
- `port(raw: number): Port`: integer 1–65535; else `Error("invalid port: <raw>")`
- `hostname(raw: string): Hostname`: lower-cased; labels of 1–63 characters from `a-z`, `0-9` and `-`, not starting or ending with `-`; total length up to 253; else `Error("invalid hostname: <raw>")`

and `endpoint(host: Ipv4 | Hostname, p: Port): string` returning `"host:port"`. The type tests use `@ts-expect-error` to prove raw strings are rejected.
@@starter
declare const brand: unique symbol;
type Brand<T, B extends string> = T & { readonly [brand]: B };
type Ipv4 = Brand<string, "Ipv4">;
type Port = Brand<number, "Port">;
type Hostname = Brand<string, "Hostname">;

function ipv4(raw: string): Ipv4 {
  throw new Error("TODO: not done yet");
}

function port(raw: number): Port {
  throw new Error("TODO: not done yet");
}

function hostname(raw: string): Hostname {
  throw new Error("TODO: not done yet");
}

function endpoint(host: Ipv4 | Hostname, p: Port): string {
  throw new Error("TODO: not done yet");
}

@@tests
function _typeTests(): void {
  // @ts-expect-error a raw string is not an Ipv4
  endpoint("10.0.0.1", port(22));
  // @ts-expect-error a raw number is not a Port
  endpoint(ipv4("10.0.0.1"), 22);
}
check(endpoint(ipv4("10.0.0.5"), port(443)), "10.0.0.5:443", "ip endpoint");
check(endpoint(hostname("DB01.corp.local"), port(5432)), "db01.corp.local:5432", "host endpoint");
const errors: string[] = [];
for (const bad of ["10.0.0.256", "10.0.0", "10.00.0.1", "a.b.c.d"]) {
  try { ipv4(bad); } catch (e) { errors.push((e as Error).message); }
}
check(errors.length, 4, "bad IPs rejected");
const bad: string[] = [];
for (const f of [() => port(0), () => port(70000), () => port(22.5), () => hostname("-web"), () => hostname("we_b"), () => hostname("a".repeat(64))]) {
  try { f(); } catch (e) { bad.push((e as Error).message.split(":")[0] ?? ""); }
}
check(bad, ["invalid port", "invalid port", "invalid port", "invalid hostname", "invalid hostname", "invalid hostname"], "bad ports and hosts");
@@hint
IPv4: `const parts = raw.split("."); parts.length === 4 && parts.every((p) => /^(0|[1-9]\d{0,2})$/.test(p) && Number(p) <= 255)`. Hostname label: `/^[a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?$/`. Each constructor ends with `return value as Ipv4;` (etc.).
@@solution
declare const brand: unique symbol;
type Brand<T, B extends string> = T & { readonly [brand]: B };
type Ipv4 = Brand<string, "Ipv4">;
type Port = Brand<number, "Port">;
type Hostname = Brand<string, "Hostname">;

function ipv4(raw: string): Ipv4 {
  const parts = raw.split(".");
  const ok = parts.length === 4 && parts.every((p) => /^(0|[1-9]\d{0,2})$/.test(p) && Number(p) <= 255);
  if (!ok) throw new Error(`invalid IPv4: ${raw}`);
  return raw as Ipv4;
}

function port(raw: number): Port {
  if (!Number.isInteger(raw) || raw < 1 || raw > 65535) throw new Error(`invalid port: ${raw}`);
  return raw as Port;
}

function hostname(raw: string): Hostname {
  const host = raw.toLowerCase();
  const label = /^[a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?$/;
  if (host.length > 253 || !host.split(".").every((l) => label.test(l))) throw new Error(`invalid hostname: ${raw}`);
  return host as Hostname;
}

function endpoint(host: Ipv4 | Hostname, p: Port): string {
  return `${host}:${p}`;
}

@@exercise Comparator combinators
Write `type Comparator<in T> = (a: T, b: T) => number` and these combinators:

- `by<T, K extends keyof T>(key: K): Comparator<T>`: compares `T[K]` values with `<`/`>` (strings by `localeCompare`)
- `reverse<T>(cmp: Comparator<T>): Comparator<T>`
- `thenBy<T>(...cmps: Comparator<T>[]): Comparator<T>`: the first non-zero result wins
- `nullsLast<T>(cmp: Comparator<T>): Comparator<T | null | undefined>`

Thanks to contravariance, a `Comparator<Asset>` sorts a `Laptop[]` directly (see the tests).
@@starter
type Comparator<in T> = (a: T, b: T) => number;

function by<T, K extends keyof T>(key: K): Comparator<T> {
  throw new Error("TODO: not done yet");
}

function reverse<T>(cmp: Comparator<T>): Comparator<T> {
  throw new Error("TODO: not done yet");
}

function thenBy<T>(...cmps: Comparator<T>[]): Comparator<T> {
  throw new Error("TODO: not done yet");
}

function nullsLast<T>(cmp: Comparator<T>): Comparator<T | null | undefined> {
  throw new Error("TODO: not done yet");
}

@@tests
interface Asset { tag: string; site: string }
interface Laptop extends Asset { ramGb: number }
const assetOrder: Comparator<Asset> = thenBy<Asset>(by("site"), by("tag"));
const laptops: Laptop[] = [
  { tag: "IT-9", site: "tokyo", ramGb: 16 },
  { tag: "IT-2", site: "osaka", ramGb: 32 },
  { tag: "IT-1", site: "tokyo", ramGb: 8 },
];
check([...laptops].sort(assetOrder).map((l) => l.tag), ["IT-2", "IT-1", "IT-9"], "asset comparator sorts laptops");
check([...laptops].sort(reverse(by<Laptop, "ramGb">("ramGb"))).map((l) => l.ramGb), [32, 16, 8], "reverse numeric");
const withGaps: (Laptop | null)[] = [null, laptops[0] ?? null, null, laptops[1] ?? null];
check(withGaps.sort(nullsLast(by<Laptop, "tag">("tag"))).map((l) => l?.tag ?? "-"), ["IT-2", "IT-9", "-", "-"], "nulls last");
@@hint
In `by`: `const x = a[key], y = b[key]; if (typeof x === "string" && typeof y === "string") return x.localeCompare(y); return x < y ? -1 : x > y ? 1 : 0;`. In `nullsLast`, handle `a == null` / `b == null` first.
@@solution
type Comparator<in T> = (a: T, b: T) => number;

function by<T, K extends keyof T>(key: K): Comparator<T> {
  return (a, b) => {
    const x = a[key];
    const y = b[key];
    if (typeof x === "string" && typeof y === "string") return x.localeCompare(y);
    return x < y ? -1 : x > y ? 1 : 0;
  };
}

function reverse<T>(cmp: Comparator<T>): Comparator<T> {
  return (a, b) => cmp(b, a);
}

function thenBy<T>(...cmps: Comparator<T>[]): Comparator<T> {
  return (a, b) => {
    for (const cmp of cmps) {
      const r = cmp(a, b);
      if (r !== 0) return r;
    }
    return 0;
  };
}

function nullsLast<T>(cmp: Comparator<T>): Comparator<T | null | undefined> {
  return (a, b) => {
    if (a == null) return b == null ? 0 : 1;
    if (b == null) return -1;
    return cmp(a, b);
  };
}

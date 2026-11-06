@@lesson l7-generators Iterators, Generators and Async Iteration
@@concepts loops, functions
@@text
`for...of`, spread (`[...x]`), destructuring and `Array.from` all work on anything **iterable**. You can make your own objects iterable, and **generators** make that easy. Async generators then let you loop over data that arrives over time: paginated APIs, file lines, event streams.

## The iterator protocol

An object is **iterable** if it has a `[Symbol.iterator]()` method that returns an **iterator**: an object with `next()` returning `{ value, done }`.

```javascript
const range = {
  from: 1,
  to: 4,
  [Symbol.iterator]() {
    let current = this.from;
    const last = this.to;
    return {
      next: () => (current <= last ? { value: current++, done: false } : { value: undefined, done: true }),
    };
  },
};
console.log([...range], Math.max(...range));
for (const n of range) if (n % 2 === 0) console.log("even", n);
```

## Generators

A **generator function** (`function*`) returns an iterator automatically. Each `yield` hands out one value and **pauses** the function until the next value is requested:

```javascript
function* subnetHosts(prefix, first, last) {
  for (let i = first; i <= last; i++) {
    yield `${prefix}.${i}`;
  }
}
const gen = subnetHosts("10.0.0", 1, 3);
console.log(gen.next(), gen.next().value);
console.log([...subnetHosts("192.168.1", 10, 12)]);
```

Generators are **lazy**: nothing runs until you ask for values. That allows infinite sequences and memory-efficient pipelines:

```javascript
function* naturals() { let n = 1; while (true) yield n++; }
function* take(iter, n) { for (const x of iter) { if (n-- <= 0) return; yield x; } }
function* map(iter, fn) { for (const x of iter) yield fn(x); }
console.log([...take(map(naturals(), (n) => n * n), 5)]);
```

`yield* other` delegates to another iterable, handy for walking trees:

```javascript
function* walk(node, depth = 0) {
  yield "  ".repeat(depth) + node.name;
  for (const child of node.children ?? []) yield* walk(child, depth + 1);
}
const tree = { name: "corp.local", children: [{ name: "Servers", children: [{ name: "web01" }] }, { name: "Users" }] };
console.log([...walk(tree)].join("\n"));
```

## Iterator helpers

Node.js 22+ and current browsers add array-like methods directly on iterators, so you don't need to write `map`/`take` yourself:

```javascript
function* naturals() { let n = 1; while (true) yield n++; }
if (typeof Iterator !== "undefined" && Iterator.prototype.take) {
  console.log(naturals().filter((n) => n % 3 === 0).map((n) => n * 10).take(4).toArray());
}
```

## Async iteration

An **async generator** (`async function*`) can `await` between yields, and `for await...of` consumes it:

```javascript
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
async function* fetchPages(totalPages) {
  for (let page = 1; page <= totalPages; page++) {
    await sleep(5);                                  // pretend to call an API
    yield { page, items: [`item${page}a`, `item${page}b`] };
  }
}
async function main() {
  const all = [];
  for await (const { items } of fetchPages(3)) all.push(...items);
  console.log(all);
}
main();
```

The same loop reads files line by line (`readline.createInterface` is async-iterable), Node streams, and `fetch` response bodies. Breaking out of a `for await` loop calls the generator's `return()`, so cleanup in its `finally` block runs.

## Generators as two-way channels

`next(value)` sends a value **into** the generator, which becomes the result of the `yield` expression. This is how early async libraries (and `co`) implemented `async/await` before it existed.

@@quiz
What makes an object usable in `for...of`?
* A `[Symbol.iterator]()` method returning an iterator with `next()`
- Having a `length` property
- Being an array
- Having a `forEach` method
? Strings, arrays, Maps, Sets and generators all implement it.

@@quiz
What does `yield` do in a generator?
- Ends the function
* Produces a value and pauses until the next value is requested
- Starts a thread
- Returns a promise
? Generators are lazy.

@@quiz
How do you consume an async generator?
- `for...in`
* `for await (const x of gen())`
- `gen().forEach`
- `await gen()`
? It awaits each value in turn.

@@drill
Which symbol names the method that makes an object iterable?
= Symbol.iterator
? for...of calls obj[Symbol.iterator]().

@@flashcards
Iterable :: has `[Symbol.iterator]()`
Iterator :: object with `next()` → `{ value, done }`
`function*` :: generator function
`yield` :: produce a value and pause
`yield*` :: delegate to another iterable
Lazy evaluation :: values computed only when requested
`async function*` :: async generator; can await between yields
`for await...of` :: loop over async iterables
Iterator helpers :: `.map/.filter/.take/.toArray` on iterators

@@exercise IP range iterator
Write a generator `ipRange(start, end)` that yields every IPv4 address from `start` to `end` inclusive (as strings), correctly carrying over octets (`10.0.0.255` → `10.0.1.0`). Then write `cidrHosts(cidr)` that returns an iterable of the **usable** host addresses of a network like `"192.168.1.0/30"` (all addresses except the network and broadcast address; for `/31` and `/32`, all addresses). Use `ipRange`.
@@starter
function* ipRange(start, end) {
  throw new Error("TODO: not done yet");
}

function cidrHosts(cidr) {
  throw new Error("TODO: not done yet");
}

@@tests
check([...ipRange("10.0.0.254", "10.0.1.1")], ["10.0.0.254", "10.0.0.255", "10.0.1.0", "10.0.1.1"], "carry");
check([...cidrHosts("192.168.1.0/30")], ["192.168.1.1", "192.168.1.2"], "/30");
check([...cidrHosts("10.1.2.3/32")], ["10.1.2.3"], "/32");
check([...cidrHosts("10.0.0.0/31")], ["10.0.0.0", "10.0.0.1"], "/31");
const big = ipRange("10.0.0.0", "10.255.255.255");
check(big.next().value, "10.0.0.0", "lazy: a huge range is fine");
check([...cidrHosts("172.16.5.77/24")].length, 254, "/24 from any address");
@@hint
Convert addresses to 32-bit numbers: `ip.split(".").reduce((n, o) => n * 256 + Number(o), 0)` and back with `[24, 16, 8, 0].map((s) => Math.floor(n / 2 ** s) % 256).join(".")` (avoid `<<` because it's signed). For CIDR, `size = 2 ** (32 - bits)`, `network = Math.floor(ip / size) * size`.
@@solution
const toNum = (ip) => ip.split(".").reduce((n, o) => n * 256 + Number(o), 0);
const toIp = (n) => [24, 16, 8, 0].map((s) => Math.floor(n / 2 ** s) % 256).join(".");

function* ipRange(start, end) {
  for (let n = toNum(start); n <= toNum(end); n++) yield toIp(n);
}

function cidrHosts(cidr) {
  const [ip, bitsText] = cidr.split("/");
  const bits = Number(bitsText);
  const size = 2 ** (32 - bits);
  const network = Math.floor(toNum(ip) / size) * size;
  const first = bits >= 31 ? network : network + 1;
  const last = bits >= 31 ? network + size - 1 : network + size - 2;
  return ipRange(toIp(first), toIp(last));
}

@@exercise Paginated API reader
APIs return data in pages. Write `async function* readAll(fetchPage)` where `fetchPage(cursor)` returns a promise of `{ items: [...], next: cursorOrNull }`. Start with cursor `null`, yield **items one at a time**, and stop when `next` is `null`. Then write `async function firstMatching(fetchPage, predicate)` that returns the first item matching `predicate` and **stops fetching** as soon as it's found (or returns `undefined`).
@@starter
async function* readAll(fetchPage) {
  throw new Error("TODO: not done yet");
}

async function firstMatching(fetchPage, predicate) {
  throw new Error("TODO: not done yet");
}

@@tests
const pages = { null: { items: [1, 2], next: "b" }, b: { items: [3], next: "c" }, c: { items: [4, 5], next: null } };
let calls = 0;
const fetchPage = async (cursor) => { calls++; return pages[cursor]; };
(async () => {
  const all = [];
  for await (const x of readAll(fetchPage)) all.push(x);
  check(all, [1, 2, 3, 4, 5], "all items");
  check(calls, 3, "three pages");
  calls = 0;
  check(await firstMatching(fetchPage, (x) => x === 3), 3, "found");
  check(calls, 2, "stopped after page 2");
  check(await firstMatching(fetchPage, (x) => x > 10), undefined, "not found");
})();
@@hint
`let cursor = null; do { const page = await fetchPage(cursor); yield* page.items; cursor = page.next; } while (cursor !== null);`. In `firstMatching`, `for await (const item of readAll(fetchPage)) if (predicate(item)) return item;`: returning from the loop stops the generator.
@@solution
async function* readAll(fetchPage) {
  let cursor = null;
  do {
    const page = await fetchPage(cursor);
    yield* page.items;
    cursor = page.next;
  } while (cursor !== null);
}

async function firstMatching(fetchPage, predicate) {
  for await (const item of readAll(fetchPage)) {
    if (predicate(item)) return item;
  }
  return undefined;
}

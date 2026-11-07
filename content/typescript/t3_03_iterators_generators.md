@@lesson t3-iterators-generators Iterators, Generators and Async Iteration
@@concepts loops, functions, concurrency
@@text
`for...of` works on arrays, strings, Maps, Sets, and anything else that follows the **iteration protocol**. Generators let you write such sequences with ordinary code, and async generators extend the idea to data that arrives over time: pages from an API, lines from a file, messages from a socket. TypeScript types all of it, so the compiler knows what each loop produces.

## The protocol

An object is **iterable** if it has a `[Symbol.iterator]()` method that returns an **iterator**: an object with `next()` returning `{ value, done }`.

```typescript
class Countdown implements Iterable<number> {
  start: number;
  constructor(start: number) {
    this.start = start;
  }
  [Symbol.iterator](): Iterator<number> {
    let n = this.start;
    return {
      next: () => (n > 0 ? { value: n--, done: false } : { value: undefined, done: true }),
    };
  }
}

for (const n of new Countdown(3)) console.log(n);
console.log([...new Countdown(4)]);
```

Everything that consumes iterables works with your class: spread `[...x]`, `Array.from(x)`, destructuring `const [a, b] = x`, `new Set(x)`, `new Map(entries)`, `Promise.all(x)`.

## Generators: iterators the easy way

Writing `next()` by hand is fiddly. A **generator function** (`function*`) does it for you: each `yield` hands out one value and pauses the function until the next value is asked for.

```typescript
function* countdown(start: number): Generator<number> {
  for (let n = start; n > 0; n--) yield n;
}

function* fibonacci(): Generator<number, never> {
  let [a, b] = [0, 1];
  while (true) {
    yield a;
    [a, b] = [b, a + b];
  }
}

function* take<T>(source: Iterable<T>, count: number): Generator<T> {
  if (count <= 0) return;
  for (const item of source) {
    yield item;
    if (--count === 0) return;
  }
}

console.log([...countdown(3)]);
console.log([...take(fibonacci(), 10)]);
```

Key points:

- Generators are **lazy**. `fibonacci()` is infinite, but `take` only pulls ten values, so the loop ends. Nothing runs until someone calls `next()`.
- The type is `Generator<T, TReturn, TNext>`: the yielded type, the type of the final `return` value, and the type of values passed *into* `next(value)`. Most code only needs `Generator<T>`, and parameters that just consume a sequence should accept `Iterable<T>`, the widest type.
- `yield* other` delegates to another iterable, which is how you write recursive generators (walking a tree, flattening nested arrays).
- A generator object is **single-use**: once it's done, iterating it again yields nothing. If you need to iterate twice, call the generator function again or make a class whose `[Symbol.iterator]` returns a fresh generator.
- Leaving a `for...of` early with `break`, `return` or an exception calls the generator's `return()`, which runs its `finally` blocks. That's how generators clean up resources.

## Lazy pipelines

Because generators are lazy, you can chain them into pipelines that process one item at a time, using constant memory even for huge inputs:

```typescript
function* map<T, U>(source: Iterable<T>, fn: (item: T) => U): Generator<U> {
  for (const item of source) yield fn(item);
}

function* filter<T>(source: Iterable<T>, keep: (item: T) => boolean): Generator<T> {
  for (const item of source) if (keep(item)) yield item;
}

const lines = ["INFO start", "ERROR disk full", "INFO ok", "ERROR timeout", "WARN slow"];
const errors = map(filter(lines, (l) => l.startsWith("ERROR")), (l) => l.slice(6));
console.log([...errors]);
```

Modern runtimes (Node 22+, current browsers) also have **iterator helpers** built in, so iterators get `.map`, `.filter`, `.take`, `.drop`, `.flatMap`, `.reduce`, `.toArray`, `.some`, `.every` and `.find` directly. With `"lib": ["esnext"]` in `tsconfig.json`, TypeScript types them:

```typescript,norun
const firstTenEven = Iterator.from(fibonacci()).filter((n) => n % 2 === 0).take(10).toArray();
```

Arrays are still the right choice when you need random access, sorting or the length. Use generators when the data is large, infinite, expensive to produce, or arrives gradually.

## Async iteration

An **async iterable** has `[Symbol.asyncIterator]()`, and you consume it with `for await...of`. An **async generator** (`async function*`) can `await` inside and `yield` values as they become ready:

```typescript
type Page<T> = { items: T[]; next: string | null };

async function* paginate<T>(fetchPage: (cursor: string | null) => Promise<Page<T>>): AsyncGenerator<T> {
  let cursor: string | null = null;
  do {
    const page = await fetchPage(cursor);
    yield* page.items;
    cursor = page.next;
  } while (cursor !== null);
}

const pages: Record<string, Page<number>> = {
  start: { items: [1, 2], next: "p2" },
  p2: { items: [3, 4], next: "p3" },
  p3: { items: [5], next: null },
};
const fakeFetch = async (cursor: string | null) => pages[cursor ?? "start"];

for await (const n of paginate(fakeFetch)) console.log("item", n);
```

The caller doesn't know about pages at all: it just sees a stream of items. And because it's lazy, if the caller `break`s after the first match, the remaining pages are never fetched.

Where you'll meet async iterables:

- Node streams: `for await (const chunk of fs.createReadStream(path))`, and `readline.createInterface` for lines.
- `fetch` response bodies (`response.body` is a `ReadableStream`, async-iterable in Node).
- Event sources wrapped with `events.on(emitter, "data")`.
- Paginated SDK calls in AWS, Azure and GitHub clients, which often return async iterables directly.

`for await` processes items **one at a time**. If each item needs slow work and the order doesn't matter, collect a batch and use `Promise.all` on it, or use a small concurrency limiter, rather than awaiting each item in turn.

## Typing summary

| You want to... | Use |
|---|---|
| accept anything you can loop over | `Iterable<T>` |
| return a lazy sequence | `Generator<T>` (or `IterableIterator<T>`) |
| accept async sources | `AsyncIterable<T>` |
| return a lazy async sequence | `AsyncGenerator<T>` |
| accept both sync and async | `Iterable<T> \| AsyncIterable<T>`, consumed with `for await` (which handles both) |

@@quiz
What makes an object usable with `for...of`?
* A `[Symbol.iterator]()` method that returns an iterator
- A `length` property
- Extending `Array`
- A `forEach` method
? Arrays, strings, Maps and Sets all implement the iteration protocol.

@@quiz
What happens when you iterate the same generator object a second time?
* Nothing is produced, because the generator is already finished
- It starts again from the beginning
- It throws a TypeError
- It repeats the last value forever
? Call the generator function again to get a fresh generator.

@@quiz
`take(fibonacci(), 5)` where `fibonacci` is an infinite generator. Why doesn't it hang?
* Generators are lazy: take stops pulling values after five
- TypeScript limits infinite loops
- fibonacci stops itself after a timeout
- Spread syntax only reads five items
? Nothing in a generator runs until a value is requested.

@@quiz
A caller `break`s out of `for await (const item of paginate(...))` after the first item. What happens to the remaining pages?
* They are never fetched, and the generator's finally blocks run
- They are fetched in the background and discarded
- The loop throws an error
- All pages were fetched before the loop started
? Async generators are lazy too; break calls return() on the generator.

@@flashcards
Iterable<T> :: has [Symbol.iterator]() returning an iterator
Iterator.next() :: returns { value, done }
function* :: generator function; each yield pauses it
Generator<T, R, N> :: yielded type, return type, type passed into next()
yield* :: delegate to another iterable
Lazy :: values are produced only when requested
Single-use :: a finished generator yields nothing again
async function* :: async generator: await and yield
for await...of :: consume an async (or sync) iterable
AsyncIterable<T> :: has [Symbol.asyncIterator]()
Iterator helpers :: map, filter, take, drop, toArray on iterators (Node 22+)

@@drill
Which keyword hands all the values of another iterable out of a generator?
= yield*
= yield *

@@drill
Which loop consumes an async iterable?
= for await...of
= for await of
= for await

@@exercise Lazy sequence helpers
Write three generic generator functions: `range(start, end, step = 1)` yields numbers from `start` up to (not including) `end`; `chunk(source, size)` yields arrays of up to `size` items; and `zip(a, b)` yields pairs `[x, y]` until either input runs out. They must be **lazy**: `zip` and `chunk` must work on infinite inputs as long as the consumer stops.
@@starter
function* range(start: number, end: number, step = 1): Generator<number> {
  throw new Error("TODO: not done yet");
}

function* chunk<T>(source: Iterable<T>, size: number): Generator<T[]> {
  throw new Error("TODO: not done yet");
}

function* zip<A, B>(a: Iterable<A>, b: Iterable<B>): Generator<[A, B]> {
  throw new Error("TODO: not done yet");
}

@@tests
function* naturals() {
  let n = 0;
  while (true) yield n++;
}
check([...range(0, 5)], [0, 1, 2, 3, 4], "range 0..5");
check([...range(10, 20, 4)], [10, 14, 18], "range with a step");
check([...range(3, 3)], [], "empty range");
check([...chunk([1, 2, 3, 4, 5], 2)], [[1, 2], [3, 4], [5]], "chunk keeps the remainder");
check([...chunk("abcdef", 3)], [["a", "b", "c"], ["d", "e", "f"]], "chunk works on strings");
check([...zip(["a", "b", "c"], naturals())], [["a", 0], ["b", 1], ["c", 2]], "zip with an infinite source");
const firstChunk = chunk(naturals(), 3).next().value;
check(firstChunk, [0, 1, 2], "chunk is lazy");
@@hint
`range` is a plain `for` loop with `yield`. `chunk` collects into a `batch` array and yields it whenever it reaches `size`, then yields what's left after the loop if it isn't empty. For `zip`, get both iterators with `a[Symbol.iterator]()` and call `next()` on each in a loop until either is `done`.
@@solution
function* range(start: number, end: number, step = 1): Generator<number> {
  for (let n = start; n < end; n += step) yield n;
}

function* chunk<T>(source: Iterable<T>, size: number): Generator<T[]> {
  let batch: T[] = [];
  for (const item of source) {
    batch.push(item);
    if (batch.length === size) {
      yield batch;
      batch = [];
    }
  }
  if (batch.length > 0) yield batch;
}

function* zip<A, B>(a: Iterable<A>, b: Iterable<B>): Generator<[A, B]> {
  const ia = a[Symbol.iterator]();
  const ib = b[Symbol.iterator]();
  while (true) {
    const x = ia.next();
    const y = ib.next();
    if (x.done || y.done) return;
    yield [x.value, y.value];
  }
}

@@exercise Paginate an API with an async generator
Write `paginate(fetchPage)`, an async generator that yields every item from a cursor-paginated API (each page is `{ items, next }`, and `next` is `null` on the last page). Then write `findFirst(source, predicate)`, which returns the first matching item from any async iterable, or `undefined`, and stops as soon as it finds one so no further pages are fetched.
@@starter
type Page<T> = { items: T[]; next: string | null };

async function* paginate<T>(fetchPage: (cursor: string | null) => Promise<Page<T>>): AsyncGenerator<T> {
  throw new Error("TODO: not done yet");
}

async function findFirst<T>(source: AsyncIterable<T>, predicate: (item: T) => boolean): Promise<T | undefined> {
  throw new Error("TODO: not done yet");
}

@@tests
const pages: Record<string, Page<{ id: number; status: string }>> = {
  start: { items: [{ id: 1, status: "ok" }, { id: 2, status: "ok" }], next: "c2" },
  c2: { items: [{ id: 3, status: "failed" }, { id: 4, status: "ok" }], next: "c3" },
  c3: { items: [{ id: 5, status: "failed" }], next: null },
};
const fetched: string[] = [];
const fetchPage = async (cursor: string | null) => {
  const key = cursor ?? "start";
  fetched.push(key);
  return pages[key];
};
const all: number[] = [];
for await (const item of paginate(fetchPage)) all.push(item.id);
check(all, [1, 2, 3, 4, 5], "every item from every page, in order");
check(fetched, ["start", "c2", "c3"], "each page fetched once");
fetched.length = 0;
const failed = await findFirst(paginate(fetchPage), (i) => i.status === "failed");
check(failed, { id: 3, status: "failed" }, "first failed item");
check(fetched, ["start", "c2"], "stopped before fetching page 3");
const none = await findFirst(paginate(fetchPage), (i) => i.id > 99);
check(none, undefined, "undefined when nothing matches");
@@hint
In `paginate`, keep a `cursor` starting at `null`; in a `do...while (cursor !== null)` loop, await the page, `yield* page.items`, and move to `page.next`. In `findFirst`, `for await (const item of source)` and `return item` as soon as the predicate is true; returning from the loop stops the generator.
@@solution
type Page<T> = { items: T[]; next: string | null };

async function* paginate<T>(fetchPage: (cursor: string | null) => Promise<Page<T>>): AsyncGenerator<T> {
  let cursor: string | null = null;
  do {
    const page = await fetchPage(cursor);
    yield* page.items;
    cursor = page.next;
  } while (cursor !== null);
}

async function findFirst<T>(source: AsyncIterable<T>, predicate: (item: T) => boolean): Promise<T | undefined> {
  for await (const item of source) {
    if (predicate(item)) return item;
  }
  return undefined;
}

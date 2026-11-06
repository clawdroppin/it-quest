@@lesson j3-async Asynchronous JavaScript: Promises and async/await
@@concepts concurrency, networking
@@text
Network requests, file reads and timers take time. JavaScript doesn't stop and wait: it starts the work, carries on, and comes back when the result is ready. This **event loop** model is why one Node.js process can handle thousands of connections.

```javascript
console.log("1: start");
setTimeout(() => console.log("3: timer fired"), 0);
console.log("2: end of script");
```

Even with 0 ms, the timer runs *after* the current code finishes.

## Promises and async/await

A **Promise** is a value that arrives later. `await` pauses an `async` function until it arrives, so asynchronous code reads like normal code:

```javascript
const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

async function checkHost(name, ms) {
  await sleep(ms);                     // pretend this is a network request
  if (name === "db02") throw new Error(`${name} unreachable`);
  return `${name} ok`;
}

async function main() {
  console.log(await checkHost("web01", 50));
  try {
    await checkHost("db02", 10);
  } catch (err) {
    console.log("caught:", err.message);
  }
}
main();
```

## Doing things in parallel

`await` in a loop does one thing at a time. `Promise.all` runs them **together**:

```javascript
const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
const check = async (name) => { await sleep(100); return `${name} ok`; };

async function main() {
  const start = Date.now();
  const results = await Promise.all(["web01", "web02", "db01"].map(check));
  console.log(results, `${Date.now() - start < 250 ? "fast" : "slow"}`);   // ~100 ms, not 300
}
main();
```

`Promise.all` fails as soon as one fails. `Promise.allSettled` waits for all and tells you which succeeded, usually what you want for health checks.

## fetch: talking to web APIs

`fetch` is built into browsers and Node 18+:

```javascript,norun
async function getStatus(url) {
  const res = await fetch(url, { signal: AbortSignal.timeout(5000) });   // give up after 5 s
  if (!res.ok) throw new Error(`HTTP ${res.status}`);
  return res.json();
}

const data = await getStatus("https://www.githubstatus.com/api/v2/status.json");
console.log(data.status.description);
```

> Note `res.ok`: `fetch` only throws on **network** errors. A 404 or 500 response is a "successful" fetch, so you must check the status yourself.

@@quiz
In what order does this print? `console.log("A"); setTimeout(() => console.log("B"), 0); console.log("C");`
* A, C, B
- A, B, C
- B, A, C
- C, A, B
? Timer callbacks run after the current code has finished, even with a 0 ms delay.

@@quiz
Five health checks take 1 second each. How long does `await Promise.all(checks)` take, roughly?
* 1 second
- 5 seconds
- 0 seconds
- it depends on the order
? They run at the same time; Promise.all waits for the slowest.

@@quiz
`const res = await fetch(url)` and the server answers 500. What happens?
* `fetch` succeeds; `res.ok` is false and `res.status` is 500
- `fetch` throws an error
- `res` is undefined
- it retries automatically
? Only network failures (DNS, refused connection, timeout) make fetch throw.

@@drill
Which Promise method waits for all promises and rejects as soon as one fails?
= Promise.all
= all
? Promise.allSettled reports each result instead.

@@flashcards
Promise :: a value that will be ready later (or fail)
async function :: always returns a promise; can use await
await :: pause until a promise settles
Promise.all :: wait for all; fails if any fails
Promise.allSettled :: wait for all; reports each result
Promise.race :: settles with the first to finish
fetch :: HTTP request; throws only on network failure
res.ok :: true for status 200-299

@@exercise Timeout wrapper
Write `withTimeout(promise, ms)` that resolves with the promise's value if it settles within `ms` milliseconds, and **rejects** with an Error whose message is `timeout after <ms> ms` otherwise.
@@starter
const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

function withTimeout(promise, ms) {
  throw new Error("TODO: not done yet");
}

@@tests
(async () => {
  check(await withTimeout(sleep(10).then(() => "fast"), 200), "fast", "a fast promise");
  let msg = "no error";
  try { await withTimeout(sleep(300).then(() => "slow"), 50); } catch (e) { msg = e.message; }
  check(msg, "timeout after 50 ms", "a slow promise");
})();
@@hint
`Promise.race` settles with whichever promise settles first: race the original against `new Promise((_, reject) => setTimeout(() => reject(new Error(...)), ms))`.
@@solution
const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

function withTimeout(promise, ms) {
  const timer = new Promise((_, reject) => setTimeout(() => reject(new Error(`timeout after ${ms} ms`)), ms));
  return Promise.race([promise, timer]);
}

@@exercise Check them all
Write `async function checkAll(hosts, checkFn)` that runs `checkFn(host)` for every host **in parallel** and returns `{ up: [...], down: [...] }`: a host is up when its check resolves, down when it rejects. Keep the input order within each list.
@@starter
async function checkAll(hosts, checkFn) {
  throw new Error("TODO: not done yet");
}

@@tests
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
const fake = async (h) => { await sleep(100); if (h.startsWith("db")) throw new Error("down"); return true; };
(async () => {
  const start = Date.now();
  const result = await checkAll(["web01", "db01", "web02", "db02"], fake);
  check(result, { up: ["web01", "web02"], down: ["db01", "db02"] }, "checkAll result");
  check(Date.now() - start < 300, true, "the checks ran in parallel (under 300 ms)");
})();
@@hint
`const results = await Promise.allSettled(hosts.map(checkFn));` Each result has `status` `"fulfilled"` or `"rejected"`; `results[i]` belongs to `hosts[i]`.
@@solution
async function checkAll(hosts, checkFn) {
  const results = await Promise.allSettled(hosts.map(checkFn));
  const up = [];
  const down = [];
  results.forEach((r, i) => (r.status === "fulfilled" ? up : down).push(hosts[i]));
  return { up, down };
}

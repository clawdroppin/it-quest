@@lesson m8-streams Streams: Processing Data Piece by Piece
@@concepts files, text-processing
@@text
`fs.readFile` loads a whole file into memory. That's fine for a config file, but not for a 20 GB log, a database dump or a video upload. **Streams** process data in chunks as it flows, using a small, constant amount of memory.

## Four kinds of stream

| Kind | Examples | You... |
|---|---|---|
| Readable | `fs.createReadStream`, HTTP request body, `process.stdin` | read from it |
| Writable | `fs.createWriteStream`, HTTP response, `process.stdout` | write to it |
| Duplex | TCP socket | both, independently |
| Transform | `zlib.createGzip()`, encryption, line splitters | write in, read changed data out |

## pipeline: connect and handle errors

```javascript
const { pipeline } = require("node:stream/promises");
const { Readable, Transform, Writable } = require("node:stream");

async function main() {
  const lines = Readable.from(["web01 up\n", "db01 DOWN\n", "mail01 up\n"]);
  const upper = new Transform({
    transform(chunk, encoding, callback) {
      callback(null, chunk.toString().toUpperCase());
    },
  });
  const collected = [];
  const sink = new Writable({
    write(chunk, encoding, callback) { collected.push(chunk.toString()); callback(); },
  });
  await pipeline(lines, upper, sink);
  console.log(collected.join(""));
}
main();
```

**Always use `pipeline`** (not `.pipe()` chains): it forwards errors from any stage, and destroys every stream on failure so file handles aren't leaked.

A real example, compressing a log:

```javascript,norun
const fs = require("node:fs");
const zlib = require("node:zlib");
await pipeline(fs.createReadStream("app.log"), zlib.createGzip(), fs.createWriteStream("app.log.gz"));
```

## Backpressure

If a reader produces data faster than the writer can handle (fast disk → slow network), chunks pile up in memory. **Backpressure** is the mechanism that slows the producer: `writable.write()` returns `false` when its buffer is full (above `highWaterMark`, 16 KB by default), and the producer should wait for the `"drain"` event. `pipeline` and `for await` handle this for you; manual `write` loops must respect it:

```javascript
const { Writable } = require("node:stream");
const { once } = require("node:events");
const slow = new Writable({
  highWaterMark: 4,
  write(chunk, enc, cb) { setTimeout(cb, 1); },
});
async function produce() {
  let waits = 0;
  for (let i = 0; i < 20; i++) {
    if (!slow.write(`chunk${i}`)) { waits++; await once(slow, "drain"); }
  }
  slow.end();
  console.log("producer waited", waits > 0 ? "(backpressure worked)" : "(never)");
}
produce();
```

## Streams are async iterables

The easiest way to **consume** a readable stream is `for await`:

```javascript
const { Readable } = require("node:stream");
async function countBytes(stream) {
  let total = 0;
  for await (const chunk of stream) total += chunk.length;
  return total;
}
countBytes(Readable.from([Buffer.from("abc"), Buffer.from("de")])).then((n) => console.log(n, "bytes"));
```

## Reading lines

Chunks don't respect line boundaries: one chunk may end in the middle of a line. `readline` handles this:

```javascript,norun
const readline = require("node:readline");
const rl = readline.createInterface({ input: fs.createReadStream("access.log"), crlfDelay: Infinity });
for await (const line of rl) {
  if (line.includes(" 500 ")) errors++;
}
```

## Web streams

Browsers and `fetch` use the standard **Web Streams** API (`ReadableStream`, `WritableStream`, `TransformStream`). Node supports both; convert with `Readable.fromWeb()` / `Readable.toWeb()`. A `fetch` response body is a web stream, so you can process a large download as it arrives.

@@quiz
Why use streams for a 20 GB log file?
* They process data in chunks with constant memory instead of loading the whole file
- They make the file smaller
- They are the only way to read files
- They run in parallel threads
? `readFile` would try to hold 20 GB in memory.

@@quiz
Why prefer `pipeline()` over `.pipe()`?
- It's shorter
* It propagates errors from every stage and cleans up all streams on failure
- `.pipe()` doesn't exist
- It compresses data
? A plain `.pipe()` chain can leak file handles when an error occurs.

@@quiz
What does `writable.write()` returning `false` mean?
- The write failed
* The internal buffer is full; wait for "drain" before writing more (backpressure)
- The stream is closed
- The data was compressed
? Ignoring it lets memory grow without limit.

@@drill
Which `stream` function connects streams and handles errors and cleanup?
= pipeline
= stream.pipeline
? await pipeline(source, transform, destination).

@@flashcards
Readable stream :: source of data chunks
Writable stream :: destination for data chunks
Transform stream :: modifies data passing through
`pipeline` :: connects streams with error handling and cleanup
Backpressure :: slowing a producer when the consumer is behind
highWaterMark :: buffer size threshold for backpressure
"drain" :: event when a writable can accept more data
`readline` :: splits a stream into lines
Web Streams :: standard stream API used by fetch and browsers

@@exercise A line-splitting transform
Write `createLineSplitter()` returning a `Transform` stream in **object mode** (`readableObjectMode: true`) that turns incoming text chunks into **complete lines** (without the line ending, handling both `\n` and `\r\n`). Chunks can split lines anywhere. A final line without a newline is emitted at the end (in `flush`); empty lines are kept.

Then `async function countStatus(chunks)` that pipes `Readable.from(chunks)` through your splitter and counts lines by HTTP status (the 9th space-separated field in a combined log line), returning an object like `{ "200": 2, "500": 1 }`. Lines with fewer than 9 fields are skipped.
@@starter
const { Transform, Readable } = require("node:stream");

function createLineSplitter() {
  throw new Error("TODO: not done yet");
}

async function countStatus(chunks) {
  throw new Error("TODO: not done yet");
}

@@tests
(async () => {
  const lines = [];
  for await (const line of Readable.from(["web01 u", "p\r\ndb0", "1 down\n\nlast"]).pipe(createLineSplitter())) lines.push(line);
  check(lines, ["web01 up", "db01 down", "", "last"], "lines across chunks");
  const log = [
    '10.0.0.1 - - [03/Nov/2026:10:00:00 +0000] "GET / HTTP/1.1" 200 512\n10.0.0.2 - - [03/Nov/2026:10:00:01 +0000] "GET /api HTTP/1.1" 5',
    '00 0\n10.0.0.3 - - [03/Nov/2026:10:00:02 +0000] "GET /x HTTP/1.1" 200 99\nbroken line\n',
  ];
  check(await countStatus(log), { 200: 2, 500: 1 }, "status counts");
})();
@@hint
Keep a `buffer` string in the closure. In `transform`, add the chunk, `const parts = buffer.split("\n"); buffer = parts.pop();` then push each part with any trailing `\r` removed. In `flush`, push the buffer if it's not empty. For counting, `for await (const line of Readable.from(chunks).pipe(createLineSplitter()))`.
@@solution
const { Transform, Readable } = require("node:stream");

function createLineSplitter() {
  let buffer = "";
  return new Transform({
    readableObjectMode: true,
    transform(chunk, encoding, callback) {
      buffer += chunk.toString();
      const parts = buffer.split("\n");
      buffer = parts.pop();
      for (const part of parts) this.push(part.replace(/\r$/, ""));
      callback();
    },
    flush(callback) {
      if (buffer !== "") this.push(buffer.replace(/\r$/, ""));
      callback();
    },
  });
}

async function countStatus(chunks) {
  const counts = {};
  for await (const line of Readable.from(chunks).pipe(createLineSplitter())) {
    const fields = line.split(" ");
    if (fields.length < 9) continue;
    const status = fields[8];
    counts[status] = (counts[status] ?? 0) + 1;
  }
  return counts;
}

@@exercise A rate-limited writable
Write `createCollector({ maxPerTick })`: a `Writable` (object mode) that stores every written object in an array exposed as `collector.items`, but only accepts `maxPerTick` writes before signalling backpressure: each `write` callback is delayed with `setImmediate`. Set `highWaterMark: maxPerTick` and `objectMode: true`.

Then `async function produce(items, writable)` that writes every item, **respecting backpressure** (await `"drain"` when `write` returns false), ends the stream, waits for `"finish"`, and returns how many times it had to wait.
@@starter
const { Writable } = require("node:stream");
const { once } = require("node:events");

function createCollector({ maxPerTick }) {
  throw new Error("TODO: not done yet");
}

async function produce(items, writable) {
  throw new Error("TODO: not done yet");
}

@@tests
(async () => {
  const c = createCollector({ maxPerTick: 3 });
  const items = Array.from({ length: 10 }, (_, i) => ({ n: i }));
  const waits = await produce(items, c);
  check(c.items.map((x) => x.n), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9], "all items in order");
  check(waits > 0, true, "had to wait for drain");
  check(c.writableFinished, true, "finished");
})();
@@hint
`const w = new Writable({ objectMode: true, highWaterMark: maxPerTick, write(obj, enc, cb) { w.items.push(obj); setImmediate(cb); } }); w.items = []; return w;`. In `produce`: `for (const item of items) if (!writable.write(item)) { waits++; await once(writable, "drain"); }` then `writable.end(); await once(writable, "finish");`.
@@solution
const { Writable } = require("node:stream");
const { once } = require("node:events");

function createCollector({ maxPerTick }) {
  const w = new Writable({
    objectMode: true,
    highWaterMark: maxPerTick,
    write(obj, encoding, callback) {
      w.items.push(obj);
      setImmediate(callback);
    },
  });
  w.items = [];
  return w;
}

async function produce(items, writable) {
  let waits = 0;
  for (const item of items) {
    if (!writable.write(item)) {
      waits++;
      await once(writable, "drain");
    }
  }
  const finished = once(writable, "finish");
  writable.end();
  await finished;
  return waits;
}

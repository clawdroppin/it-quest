@@lesson m8-workers Worker Threads and Child Processes
@@concepts processes, concurrency, automation
@@text
Node.js runs your JavaScript on one thread. For **CPU-heavy work** (hashing, compression, image processing, parsing huge files) you need more threads; for **running other programs** (git, ping, PowerShell, ffmpeg) you need child processes.

## Worker threads

A worker is a separate JavaScript thread with its own event loop and memory. You talk to it by **messages**:

```javascript
const { Worker } = require("node:worker_threads");

const workerCode = `
  const { parentPort, workerData } = require("node:worker_threads");
  let primes = 0;
  for (let n = 2; n < workerData.limit; n++) {
    let isPrime = true;
    for (let d = 2; d * d <= n; d++) if (n % d === 0) { isPrime = false; break; }
    if (isPrime) primes++;
  }
  parentPort.postMessage({ primes });
`;

function countPrimes(limit) {
  return new Promise((resolve, reject) => {
    const worker = new Worker(workerCode, { eval: true, workerData: { limit } });
    worker.once("message", (msg) => resolve(msg.primes));
    worker.once("error", reject);
  });
}

Promise.all([countPrimes(50000), countPrimes(60000)]).then((r) => console.log("primes:", r));
```

- Normally the worker code lives in its own file: `new Worker("./hash-worker.js", { workerData })`.
- Messages are **copied** (structured clone). For big binary data, **transfer** an `ArrayBuffer` (zero-copy, the sender loses access) or share a `SharedArrayBuffer` and coordinate with `Atomics`.
- Starting a worker costs a few milliseconds and some memory, so reuse workers with a **pool** (e.g. the `piscina` library) for many small jobs.
- Workers don't help with I/O: Node's I/O is already asynchronous.

## Child processes

`node:child_process` runs other programs:

| Function | Use it for |
|---|---|
| `execFile(file, args)` | run a program, get all output at the end (no shell) |
| `spawn(cmd, args)` | stream output as it arrives; long-running processes |
| `exec("cmd string")` | runs through a **shell**: pipes, globbing, but injection-prone |
| `fork("script.js")` | another Node.js process with an IPC message channel |
| `*Sync` versions | simple scripts only; they block the event loop |

```javascript
const { execFile } = require("node:child_process");
const { promisify } = require("node:util");
const run = promisify(execFile);

async function main() {
  const { stdout } = await run(process.execPath, ["-e", "console.log(JSON.stringify({ pid: process.pid > 0, v: process.version }))"]);
  console.log("child said:", stdout.trim());
  try {
    await run(process.execPath, ["-e", "process.exit(3)"]);
  } catch (e) {
    console.log("exit code:", e.code);
  }
}
main();
```

Streaming output with `spawn`:

```javascript,norun
const { spawn } = require("node:child_process");
const ping = spawn("ping", process.platform === "win32" ? ["-n", "3", "1.1.1.1"] : ["-c", "3", "1.1.1.1"]);
ping.stdout.on("data", (chunk) => process.stdout.write(chunk));
ping.on("close", (code) => console.log("ping exited with", code));
```

## Never build shell commands from input

```javascript,norun
exec(`ping -c 1 ${userInput}`);                 // DANGER: "8.8.8.8; rm -rf ~" runs both
execFile("ping", ["-c", "1", userInput]);       // safe: the argument is passed as-is, no shell
```

Use `execFile`/`spawn` with an **argument array** and validate input (e.g. that it's an IP address or hostname) anyway. On Windows, `.bat`/`.cmd` files always go through `cmd.exe`, so be extra careful there.

## Practical notes

- Set a `timeout` option and handle `"error"` (program not found → `ENOENT`).
- Output encoding: Windows tools may print in the console code page, not UTF-8.
- Large outputs: `execFile` has a `maxBuffer` (1 MB default); use `spawn` for big outputs.
- `AbortController` can cancel a child: pass `{ signal }` and call `controller.abort()`.

@@quiz
When do worker threads help?
* For CPU-heavy JavaScript work that would block the event loop
- For network requests
- For reading files
- Always; they make every program faster
? I/O is already asynchronous in Node.js.

@@quiz
Why is `exec(`ping ${input}`)` dangerous?
* It runs through a shell, so input like `; rm -rf ~` executes extra commands
- ping is deprecated
- exec is slow
- It only works on Linux
? Use `execFile`/`spawn` with an argument array.

@@quiz
How do worker threads exchange data with the main thread?
- Shared global variables
* Messages (copied by structured clone), transferred ArrayBuffers or SharedArrayBuffer
- Files only
- They can't
? Each worker has its own memory and event loop.

@@drill
Which `child_process` function runs a program with an argument array and no shell?
= execFile
= spawn
? Never build a shell string from user input.

@@flashcards
Worker thread :: separate JS thread with its own event loop
`workerData` :: initial data passed to a worker
`postMessage` :: send a message between threads
Transferable :: ArrayBuffer moved without copying
SharedArrayBuffer :: memory shared between threads; use Atomics
`execFile` :: run a program without a shell, output at the end
`spawn` :: run a program with streamed output
`exec` :: run a command string through a shell
`fork` :: start a Node.js child with an IPC channel
Command injection :: shell metacharacters in input running extra commands

@@exercise Safe command arguments
Write `buildPingArgs(target, { count = 4, platform })` returning `[program, args]` for a ping command, **refusing unsafe input**:

- `target` must be an IPv4 address (four 0–255 parts) or a hostname (letters, digits, hyphens and dots; each label 1–63 characters, not starting or ending with a hyphen; total ≤ 253); otherwise throw `Error("invalid target")`
- `count` must be an integer 1–100, else throw `Error("invalid count")`
- Windows (`platform === "win32"`): `["ping", ["-n", String(count), target]]`; others: `["ping", ["-c", String(count), target]]`
@@starter
function buildPingArgs(target, { count = 4, platform = process.platform } = {}) {
  throw new Error("TODO: not done yet");
}

@@tests
check(buildPingArgs("10.0.0.1", { platform: "linux" }), ["ping", ["-c", "4", "10.0.0.1"]], "linux ip");
check(buildPingArgs("web-01.corp.local", { count: 2, platform: "win32" }), ["ping", ["-n", "2", "web-01.corp.local"]], "windows host");
const bad = ["8.8.8.8; rm -rf ~", "$(whoami)", "-x.local", "256.1.1.1", "a..b", "", "host name"];
check(bad.map((t) => { try { buildPingArgs(t, { platform: "linux" }); return "accepted"; } catch (e) { return e.message; } }), bad.map(() => "invalid target"), "unsafe targets refused");
let msg = "";
try { buildPingArgs("1.1.1.1", { count: 1000, platform: "linux" }); } catch (e) { msg = e.message; }
check(msg, "invalid count", "count");
@@hint
IPv4: `/^\d{1,3}(\.\d{1,3}){3}$/` plus each part ≤ 255. Hostname: split on dots and test each label with `/^[a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?$/i`. Reject digits-and-dots strings that aren't valid IPv4 (like `"256.1.1.1"`) instead of accepting them as hostnames.
@@solution
function isIpv4(text) {
  if (!/^\d{1,3}(\.\d{1,3}){3}$/.test(text)) return false;
  return text.split(".").every((p) => Number(p) <= 255);
}

function isHostname(text) {
  if (text.length === 0 || text.length > 253) return false;
  if (/^[\d.]+$/.test(text)) return false;
  return text.split(".").every((label) => /^[a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?$/i.test(label));
}

function buildPingArgs(target, { count = 4, platform = process.platform } = {}) {
  if (!isIpv4(target) && !isHostname(target)) throw new Error("invalid target");
  if (!Number.isInteger(count) || count < 1 || count > 100) throw new Error("invalid count");
  const flag = platform === "win32" ? "-n" : "-c";
  return ["ping", [flag, String(count), target]];
}

@@exercise A worker pool
Write `class WorkerPool` that runs jobs on a fixed number of worker threads:

- `constructor(code, size)`: `code` is worker source (run with `{ eval: true }`); it receives jobs via `parentPort.on("message", ...)` and replies with `parentPort.postMessage(result)`. Start `size` workers.
- `run(data)`: returns a promise for the result; if all workers are busy, the job waits in a FIFO queue
- `close()`: terminates all workers (returns a promise)

Each worker handles one job at a time.
@@starter
const { Worker } = require("node:worker_threads");

class WorkerPool {
  constructor(code, size) {
    throw new Error("TODO: not done yet");
  }
}

@@tests
const code = `
  const { parentPort, threadId } = require("node:worker_threads");
  parentPort.on("message", (n) => {
    let x = 0;
    for (let i = 0; i < n * 100000; i++) x += i % 7;
    parentPort.postMessage({ n, threadId, x: x > 0 });
  });
`;
(async () => {
  const pool = new WorkerPool(code, 2);
  const results = await Promise.all([5, 1, 3, 2, 4].map((n) => pool.run(n)));
  check(results.map((r) => r.n), [5, 1, 3, 2, 4], "results match their jobs");
  check(new Set(results.map((r) => r.threadId)).size, 2, "two worker threads used");
  await pool.close();
  check(true, true, "closed");
})();
@@hint
Keep `this.idle = [...workers]` and `this.queue = []`. `run(data)` returns a promise and pushes `{ data, resolve, reject }` to the queue, then calls `#dispatch()`. `#dispatch` takes an idle worker and a queued job, attaches `worker.once("message", (r) => { job.resolve(r); this.idle.push(worker); this.#dispatch(); })` and calls `worker.postMessage(job.data)`.
@@solution
const { Worker } = require("node:worker_threads");

class WorkerPool {
  constructor(code, size) {
    this.workers = Array.from({ length: size }, () => new Worker(code, { eval: true }));
    this.idle = [...this.workers];
    this.queue = [];
  }

  run(data) {
    return new Promise((resolve, reject) => {
      this.queue.push({ data, resolve, reject });
      this.#dispatch();
    });
  }

  #dispatch() {
    while (this.idle.length > 0 && this.queue.length > 0) {
      const worker = this.idle.shift();
      const job = this.queue.shift();
      const onMessage = (result) => {
        worker.off("error", onError);
        job.resolve(result);
        this.idle.push(worker);
        this.#dispatch();
      };
      const onError = (error) => {
        worker.off("message", onMessage);
        job.reject(error);
      };
      worker.once("message", onMessage);
      worker.once("error", onError);
      worker.postMessage(job.data);
    }
  }

  close() {
    return Promise.all(this.workers.map((w) => w.terminate()));
  }
}

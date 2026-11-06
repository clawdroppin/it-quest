@@lesson l7-errors Error Handling in Depth
@@concepts errors
@@text
Good error handling is what separates a script from a dependable tool. This lesson goes past `try/catch` to error types, causes, async errors, aggregate errors, cleanup and the "result" style of handling failures.

## Built-in error types

| Type | Thrown when |
|---|---|
| `TypeError` | wrong type: calling a non-function, reading a property of `undefined` |
| `RangeError` | a number out of range: `new Array(-1)`, `toFixed(200)` |
| `ReferenceError` | using a variable that doesn't exist |
| `SyntaxError` | bad code, or `JSON.parse("{oops")` |
| `URIError` | bad `decodeURIComponent` input |
| `AggregateError` | several errors together, e.g. from `Promise.any` |

Every error has `name`, `message` and `stack`. Node.js system errors also have a `code` like `ENOENT` (file not found), `ECONNREFUSED`, `EACCES` or `ETIMEDOUT`: check `code`, not the message text.

## Custom errors and causes

```javascript
class AppError extends Error {
  constructor(message, { code, cause, details } = {}) {
    super(message, { cause });              // standard `cause` option keeps the original error
    this.name = new.target.name;            // the subclass's name
    this.code = code;
    this.details = details;
  }
}
class NotFoundError extends AppError {}

function loadUser(id) {
  try {
    JSON.parse("{broken");
  } catch (err) {
    throw new NotFoundError(`user ${id} could not be loaded`, { code: "USER_LOAD", cause: err });
  }
}
try {
  loadUser(7);
} catch (e) {
  console.log(e.name, e.code, e instanceof AppError, "| caused by:", e.cause.name);
}
```

**Wrap, don't lose**: when you translate a low-level error into a meaningful one, pass `{ cause }`. Logs and debuggers then show the whole chain.

## finally and cleanup

```javascript
function withConnection(work) {
  const conn = { open: true };
  try {
    return work(conn);
  } finally {
    conn.open = false;                      // runs on return AND on throw
    console.log("connection closed");
  }
}
console.log(withConnection(() => "result"));
```

Don't `return` from inside `finally`: it silently replaces the original return value or swallows the thrown error.

## Async errors

```javascript
const fail = (ms, msg) => new Promise((_, reject) => setTimeout(() => reject(new Error(msg)), ms));
async function main() {
  try {
    await fail(5, "network down");
  } catch (e) {
    console.log("caught:", e.message);
  }
  const results = await Promise.allSettled([fail(5, "a"), Promise.resolve("ok")]);
  console.log(results.map((r) => r.status));
  try {
    await Promise.any([fail(5, "mirror 1"), fail(6, "mirror 2")]);
  } catch (e) {
    console.log(e.constructor.name, e.errors.map((x) => x.message));
  }
}
main();
```

- A rejected promise that nobody handles is an **unhandled rejection**. Modern Node.js **crashes the process** on it (the safe default). Always `await` or `.catch()` your promises.
- `try/catch` only catches errors from promises you `await` inside it. A promise started without `await` escapes.
- `process.on("uncaughtException")` / `"unhandledRejection"` are for logging before exiting, not for continuing as if nothing happened.

## Expected failures vs bugs

- **Operational errors** are expected: network timeouts, missing files, invalid user input. Handle them: retry, show a message, return a 4xx response.
- **Programmer errors** are bugs: `undefined is not a function`. Don't try to recover; let them crash loudly (in a supervised process that restarts), and fix the code.

## The result pattern

For expected failures, some codebases return values instead of throwing, so callers can't forget to handle them:

```javascript
function parsePort(text) {
  const n = Number(text);
  if (!Number.isInteger(n) || n < 1 || n > 65535) return { ok: false, error: `invalid port "${text}"` };
  return { ok: true, value: n };
}
for (const t of ["443", "http"]) {
  const r = parsePort(t);
  console.log(r.ok ? `port ${r.value}` : r.error);
}
```

This is the same idea as Rust's `Result` and Go's `value, err`.

@@quiz
Why pass `{ cause: err }` when throwing a new error from a catch block?
* To keep the original error attached so logs show the full chain
- To make the new error faster
- It's required by the language
- To suppress the original error
? `err.cause` is standard in modern JavaScript.

@@quiz
What should you check to detect "file not found" in Node.js?
- `err.message.includes("not found")`
* `err.code === "ENOENT"`
- `err.name === "NotFound"`
- `err.status === 404`
? Messages can change between versions and languages; codes don't.

@@quiz
What happens in current Node.js when a promise rejection is never handled?
- Nothing
- A warning, and the program continues
* The process crashes with the error
- The promise retries
? Always await or attach `.catch()`.

@@quiz
Which is a programmer error rather than an operational error?
- A DNS lookup that times out
- A user typing letters into a port field
* Calling a method on `undefined` because of a typo
- A full disk
? Programmer errors are bugs; fix the code instead of handling them.

@@drill
Which option links a new error to the error that caused it?
= cause
? new Error("save failed", { cause: err }).

@@flashcards
TypeError :: wrong type, e.g. calling undefined
RangeError :: number out of allowed range
AggregateError :: several errors; `.errors` array
`error.cause` :: the original error wrapped by a new one
`err.code` :: Node system error code like ENOENT
finally :: runs after try/catch on both success and failure
Unhandled rejection :: rejected promise nobody handles; crashes Node
Operational error :: expected failure to handle
Result pattern :: return `{ ok, value | error }` instead of throwing

@@exercise An error hierarchy for an API client
Write:

- `class ApiError extends Error` with constructor `(message, { status, code, cause } = {})`; `name` set to the class name (works for subclasses), and `status`, `code`, `cause` properties
- `class NotFoundError extends ApiError`, `class AuthError extends ApiError`, `class RateLimitError extends ApiError` (with an extra `retryAfter` option and property)
- `fromResponse(status, body)` returning the right error: 404 → NotFoundError, 401/403 → AuthError, 429 → RateLimitError (with `retryAfter` from `body.retryAfter`), any other status ≥ 400 → ApiError. The message is `body.message` or `"HTTP <status>"`; `code` is `body.code`.
- `describeChain(error)` returning the names and messages along the `cause` chain: `"AuthError: token expired <- TypeError: bad"`
@@starter
class ApiError extends Error {}
class NotFoundError extends ApiError {}
class AuthError extends ApiError {}
class RateLimitError extends ApiError {}

function fromResponse(status, body = {}) {
  throw new Error("TODO: not done yet");
}

function describeChain(error) {
  throw new Error("TODO: not done yet");
}

@@tests
const e1 = fromResponse(404, { message: "no such ticket", code: "T404" });
check([e1 instanceof NotFoundError, e1 instanceof ApiError, e1 instanceof Error, e1.name, e1.message, e1.status, e1.code], [true, true, true, "NotFoundError", "no such ticket", 404, "T404"], "404");
const e2 = fromResponse(429, { retryAfter: 30 });
check([e2.name, e2.message, e2.retryAfter], ["RateLimitError", "HTTP 429", 30], "429");
check([fromResponse(403).name, fromResponse(500).name], ["AuthError", "ApiError"], "others");
const chained = new AuthError("token expired", { status: 401, cause: new TypeError("bad") });
check(describeChain(chained), "AuthError: token expired <- TypeError: bad", "chain");
check(describeChain(new Error("solo")), "Error: solo", "single");
@@hint
In the constructor: `super(message, { cause }); this.name = new.target.name; this.status = status; this.code = code;`. `RateLimitError`'s constructor takes the same options plus `retryAfter`, calls `super(message, options)` and stores it. For the chain, loop `for (let e = error; e; e = e.cause)`.
@@solution
class ApiError extends Error {
  constructor(message, { status, code, cause } = {}) {
    super(message, { cause });
    this.name = new.target.name;
    this.status = status;
    this.code = code;
  }
}

class NotFoundError extends ApiError {}
class AuthError extends ApiError {}

class RateLimitError extends ApiError {
  constructor(message, options = {}) {
    super(message, options);
    this.retryAfter = options.retryAfter;
  }
}

function fromResponse(status, body = {}) {
  const message = body.message ?? `HTTP ${status}`;
  const options = { status, code: body.code };
  if (status === 404) return new NotFoundError(message, options);
  if (status === 401 || status === 403) return new AuthError(message, options);
  if (status === 429) return new RateLimitError(message, { ...options, retryAfter: body.retryAfter });
  return new ApiError(message, options);
}

function describeChain(error) {
  const parts = [];
  for (let e = error; e; e = e.cause) parts.push(`${e.name}: ${e.message}`);
  return parts.join(" <- ");
}

@@exercise Retry with backoff and a deadline
Write `async function retry(task, { attempts = 3, baseDelay = 10, shouldRetry = () => true, sleep })`:

- call `task(attemptNumber)` (1-based); return its result on success
- on failure, if attempts remain **and** `shouldRetry(error)` is true, wait `baseDelay * 2 ** (attempt - 1)` ms using the provided `sleep(ms)` function, then try again
- otherwise throw an `AggregateError` with all the errors so far and the message `"failed after <n> attempts"` (n = the number of attempts actually made)

Using an injected `sleep` keeps tests fast: they pass a fake that records delays.
@@starter
async function retry(task, { attempts = 3, baseDelay = 10, shouldRetry = () => true, sleep } = {}) {
  throw new Error("TODO: not done yet");
}

@@tests
(async () => {
  const delays = [];
  const sleep = async (ms) => { delays.push(ms); };
  const flaky = async (n) => { if (n < 3) throw new Error(`fail ${n}`); return "ok"; };
  check(await retry(flaky, { attempts: 5, baseDelay: 100, sleep }), "ok", "eventually succeeds");
  check(delays, [100, 200], "exponential backoff");
  let err;
  try { await retry(async () => { throw new Error("down"); }, { attempts: 3, sleep }); } catch (e) { err = e; }
  check([err.constructor.name, err.message, err.errors.length], ["AggregateError", "failed after 3 attempts", 3], "gives up");
  const fatal = async () => { const e = new Error("bad password"); e.code = "AUTH"; throw e; };
  try { await retry(fatal, { attempts: 5, sleep, shouldRetry: (e) => e.code !== "AUTH" }); } catch (e) { err = e; }
  check([err.message, err.errors.length], ["failed after 1 attempts", 1], "no retry for auth errors");
})();
@@hint
`const errors = []; for (let attempt = 1; attempt <= attempts; attempt++) { try { return await task(attempt); } catch (e) { errors.push(e); if (attempt === attempts || !shouldRetry(e)) break; await sleep(baseDelay * 2 ** (attempt - 1)); } } throw new AggregateError(errors, `failed after ${errors.length} attempts`);`
@@solution
async function retry(task, { attempts = 3, baseDelay = 10, shouldRetry = () => true, sleep } = {}) {
  const wait = sleep ?? ((ms) => new Promise((r) => setTimeout(r, ms)));
  const errors = [];
  for (let attempt = 1; attempt <= attempts; attempt++) {
    try {
      return await task(attempt);
    } catch (error) {
      errors.push(error);
      if (attempt === attempts || !shouldRetry(error)) break;
      await wait(baseDelay * 2 ** (attempt - 1));
    }
  }
  throw new AggregateError(errors, `failed after ${errors.length} attempts`);
}

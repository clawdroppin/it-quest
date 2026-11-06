@@lesson j6-production Production Apps: Validation, Auth, Config and Deployment
@@text
Your app works on your laptop. Production is where real users, real attackers and real failures are. This lesson covers what separates a demo from an application a company would run.

## Validate every input at the boundary

Data from outside (request bodies, query strings, files, other APIs) is untrusted until checked. Check it **once, at the edge**, and turn it into clean values the rest of the code can trust. Libraries like **zod** do this declaratively; the idea is simple enough to write yourself:

```javascript
const rules = {
  title: (v) => (typeof v === "string" && v.trim().length >= 3 && v.length <= 100 ? null : "title must be 3-100 characters"),
  priority: (v) => (Number.isInteger(v) && v >= 1 && v <= 4 ? null : "priority must be 1-4"),
  email: (v) => (typeof v === "string" && /^[^@\s]+@[^@\s]+\.[^@\s]+$/.test(v) ? null : "email is not valid"),
};

function validate(body, schema) {
  const errors = Object.entries(schema).map(([field, check]) => check(body[field])).filter(Boolean);
  return errors.length ? { ok: false, errors } : { ok: true };
}

console.log(validate({ title: "VPN down", priority: 2, email: "yuki@example.jp" }, rules));
console.log(validate({ title: "x", priority: "high", email: "nope" }, rules));
```

Return **all** the problems at once (a form shows every error together), with status 400.

## Passwords

Never store passwords. Store a **slow, salted hash**. Node has `scrypt` built in:

```javascript
const crypto = require("node:crypto");

function hashPassword(password) {
  const salt = crypto.randomBytes(16);
  const hash = crypto.scryptSync(password, salt, 32);
  return `${salt.toString("hex")}:${hash.toString("hex")}`;
}

function verifyPassword(password, stored) {
  const [saltHex, hashHex] = stored.split(":");
  const hash = crypto.scryptSync(password, Buffer.from(saltHex, "hex"), 32);
  return crypto.timingSafeEqual(hash, Buffer.from(hashHex, "hex"));
}

const stored = hashPassword("Tokyo-2026-correct-horse");
console.log(stored.slice(0, 32) + "...");
console.log(verifyPassword("Tokyo-2026-correct-horse", stored), verifyPassword("password123", stored));
```

The salt makes identical passwords hash differently; "slow" makes guessing billions of passwords impractical. (`argon2` and `bcrypt` packages are also good choices.)

## Sessions or tokens?

| | Session cookie | JWT (JSON Web Token) |
|---|---|---|
| what the browser holds | a random id; the server stores the session | a signed token holding the user id and expiry |
| log out / revoke | delete the session: instant | hard: the token stays valid until it expires |
| best for | web apps with a browser front end | APIs called by other services, mobile apps |

For a normal web app, use a session cookie with every safety flag:

```text
Set-Cookie: sid=Jx8f...random...; HttpOnly; Secure; SameSite=Lax; Path=/; Max-Age=28800
```

- `HttpOnly`: JavaScript can't read it, so XSS can't steal it.
- `Secure`: only sent over HTTPS.
- `SameSite=Lax`: not sent on most cross-site requests, which blocks most **CSRF**.

**Authorization** is a separate question from authentication: "who are you?" vs "may you do this?". Check it on the server for every request: "is this ticket in this user's team?" Hiding a button in the UI is not security.

## Configuration: the twelve-factor way

Config that changes between environments (ports, database URLs, API keys) comes from **environment variables**, read and checked once at startup:

```javascript
function loadConfig(env) {
  const problems = [];
  const port = Number(env.PORT ?? 3000);
  if (!Number.isInteger(port) || port < 1 || port > 65535) problems.push("PORT must be 1-65535");
  if (!env.DATABASE_URL) problems.push("DATABASE_URL is required");
  if (env.NODE_ENV === "production" && (env.SESSION_SECRET ?? "").length < 32) problems.push("SESSION_SECRET must be at least 32 characters");
  if (problems.length) throw new Error("bad configuration: " + problems.join("; "));
  return Object.freeze({ port, databaseUrl: env.DATABASE_URL, production: env.NODE_ENV === "production" });
}

console.log(loadConfig({ DATABASE_URL: "postgres://db/app" }));
try {
  loadConfig({ PORT: "99999", NODE_ENV: "production" });
} catch (e) {
  console.log(e.message);
}
```

Fail **at startup** with a clear message, not at 3 a.m. when the first request needs the missing value. Locally, Node 20.6+ can load a file: `node --env-file=.env server.js` (and `.env` goes in `.gitignore`).

## Logging and health checks

- Log **structured JSON** (one object per line) so log systems can search it: `{"level":"info","msg":"request","method":"GET","path":"/api/links","status":200,"ms":4}`. **pino** is the standard library for this.
- Never log passwords, tokens or full card numbers.
- Expose `GET /health` (am I running?) and maybe `/ready` (can I reach the database?) for load balancers and Kubernetes.

## Project structure and scripts

```text
link-vault/
├── package.json          scripts, dependencies, "engines"
├── package-lock.json     exact versions: commit it, install with `npm ci`
├── src/
│   ├── server.js         start-up: config, wiring, listen
│   ├── routes/           HTTP only
│   ├── services/         business rules
│   └── db/               storage and migrations
├── public/               front end
└── test/
```

```text
"scripts": {
  "dev": "node --watch --env-file=.env src/server.js",
  "start": "node src/server.js",
  "test": "node --test",
  "lint": "eslint ."
}
```

`npm ci` installs exactly what the lock file says (use it on servers and in CI). `npm audit` lists known vulnerabilities in your dependencies. Every dependency is code you're trusting: prefer few, popular, maintained ones.

## Deployment

```text
# Dockerfile
FROM node:24-slim
WORKDIR /app
COPY package*.json ./
RUN npm ci --omit=dev
COPY . .
USER node
ENV NODE_ENV=production PORT=3000
EXPOSE 3000
CMD ["node", "src/server.js"]
```

In front of it, a **reverse proxy** (nginx, Caddy, a cloud load balancer) handles HTTPS. The app should also shut down cleanly: on `SIGTERM`, stop accepting connections, finish the ones in progress, close the database, then exit, so deployments don't drop requests:

```javascript,norun
process.on("SIGTERM", () => {
  server.close(async () => {
    await db.close();
    process.exit(0);
  });
});
```

Where it runs: a VPS with **PM2** or systemd, a container platform (Cloud Run, AWS App Runner, Azure Container Apps), or Kubernetes for bigger systems.

@@quiz
Where should request input be validated?
- In the database
- In the front end only
* Once at the boundary (the route), before the business logic sees it
- Wherever a bug appears
? Front-end checks are for convenience: anyone can call your API directly. Validate on the server, at the edge.

@@quiz
Why add a salt before hashing each password?
- To make the hash shorter
* So identical passwords produce different hashes, which defeats precomputed tables
- To encrypt the password so it can be read later
- Because scrypt requires an even number of characters
? Without salts, one table of hashes of common passwords would crack everyone who used "password123".

@@quiz
What does the `HttpOnly` cookie flag do?
- Sends the cookie only over HTTP, not HTTPS
* Stops JavaScript on the page from reading the cookie
- Makes the cookie expire when the tab closes
- Shares the cookie with other sites
? Even if an attacker injects a script (XSS), they can't read the session id.

@@quiz
The app needs `DATABASE_URL` but it isn't set. When should the app fail?
* At startup, with a clear message
- On the first request that uses the database
- Never: use a default database
- After 10 retries
? Failing fast turns a confusing runtime error into an obvious deployment error.

@@drill
Which `crypto` function derives a slow, salted hash for storing passwords?
= scrypt
= crypto.scrypt
? Never store passwords themselves.

@@flashcards
validate at the boundary :: check untrusted input once, at the edge; return all errors with 400
`crypto.scryptSync(pw, salt, 32)` :: slow salted password hash in Node
`crypto.timingSafeEqual` :: compare secrets without timing leaks
session cookie flags :: `HttpOnly; Secure; SameSite=Lax`
authentication vs authorization :: who are you? vs may you do this?
JWT :: a signed token holding claims; hard to revoke before it expires
twelve-factor config :: settings from environment variables, checked at startup
`node --env-file=.env` :: load env vars from a file (Node 20.6+)
`npm ci` :: install exactly the lock file's versions (servers and CI)
SIGTERM :: "please stop": close the server, finish requests, then exit
structured logging :: one JSON object per log line (pino)

@@exercise Validate a sign-up form
Write `validateSignup(body)`, returning a list of **all** problems (an empty list when everything is fine), in this order:

- `name`: a string, 1-50 characters after trimming → `"name is required"`
- `email`: a string with exactly one `@`, something before it and a dot in the part after it → `"email is not valid"`
- `password`: at least 12 characters → `"password must be at least 12 characters"`; and not containing the name (case-insensitive, only if a name was given) → `"password must not contain your name"`
- `age`: optional; if present, an integer from 18 to 120 → `"age must be 18-120"`
@@starter
function validateSignup(body) {
  // TODO
}

console.log(validateSignup({ name: "Yuki", email: "yuki@example.jp", password: "yuki-is-great-2026" }));
@@tests
check(validateSignup({ name: "Yuki", email: "yuki@example.jp", password: "a-long-passphrase" }), [], "a valid sign-up");
check(validateSignup({ name: " ", email: "yuki@", password: "short" }), ["name is required", "email is not valid", "password must be at least 12 characters"], "every problem at once");
check(validateSignup({ name: "Yuki", email: "yuki@example.jp", password: "my-name-is-YUKI" }), ["password must not contain your name"], "password contains the name");
check(validateSignup({ name: "Ken", email: "ken@@example.jp", password: "a-long-passphrase", age: 17 }), ["email is not valid", "age must be 18-120"], "two @ and too young");
check(validateSignup({ name: "Ken", email: "ken@example", password: "a-long-passphrase", age: 30.5 }), ["email is not valid", "age must be 18-120"], "no dot, age not whole");
check(validateSignup({}), ["name is required", "email is not valid", "password must be at least 12 characters"], "nothing sent");
@@hint
Start with `const problems = [];` and `const name = typeof body.name === "string" ? body.name.trim() : "";`. For the email, `const parts = String(body.email ?? "").split("@")` must have length 2, `parts[0]` non-empty and `parts[1].includes(".")`. For age, check `body.age !== undefined && !(Number.isInteger(body.age) && body.age >= 18 && body.age <= 120)`.
@@solution
function validateSignup(body) {
  const problems = [];
  const name = typeof body.name === "string" ? body.name.trim() : "";
  if (name.length < 1 || name.length > 50) problems.push("name is required");
  const parts = String(body.email ?? "").split("@");
  if (parts.length !== 2 || !parts[0] || !parts[1].includes(".")) problems.push("email is not valid");
  const password = typeof body.password === "string" ? body.password : "";
  if (password.length < 12) problems.push("password must be at least 12 characters");
  else if (name && password.toLowerCase().includes(name.toLowerCase())) problems.push("password must not contain your name");
  if (body.age !== undefined && !(Number.isInteger(body.age) && body.age >= 18 && body.age <= 120)) problems.push("age must be 18-120");
  return problems;
}

console.log(validateSignup({ name: "Yuki", email: "yuki@example.jp", password: "yuki-is-great-2026" }));

@@exercise Sessions
Write `createSessions(now)`, a session store for a web app. It returns `{ create, get, destroy }`:

- `create(userId)` makes a random id with `crypto.randomBytes(32).toString("base64url")`, stores `{ userId, expires: now() + 8 hours }` and returns the id,
- `get(id)` returns the userId, or `null` for an unknown or expired session (delete expired ones),
- `destroy(id)` removes it (logging out).

`now()` returns the time in milliseconds.
@@starter
const crypto = require("node:crypto");
const HOURS_8 = 8 * 60 * 60 * 1000;

function createSessions(now = Date.now) {
  // TODO
}

const sessions = createSessions();
const sid = sessions.create("yuki");
console.log(sid.length, sessions.get(sid));
@@tests
let _t = 1_000_000;
const _s = createSessions(() => _t);
const _a = _s.create("yuki");
const _b = _s.create("ken");
check(_a !== _b && _a.length >= 40, true, "ids are long and different");
check(_s.get(_a), "yuki", "a fresh session");
check(_s.get("made-up-id"), null, "an unknown id");
_s.destroy(_b);
check(_s.get(_b), null, "after logout");
_t += HOURS_8 - 1;
check(_s.get(_a), "yuki", "still valid just before 8 hours");
_t += 1;
check(_s.get(_a), null, "expired at 8 hours");
@@hint
Keep a `Map`. `create`: `const id = crypto.randomBytes(32).toString("base64url"); map.set(id, { userId, expires: now() + HOURS_8 }); return id;`. `get`: if there's no entry, return null; if `now() >= entry.expires`, delete it and return null; else return `entry.userId`.
@@solution
const crypto = require("node:crypto");
const HOURS_8 = 8 * 60 * 60 * 1000;

function createSessions(now = Date.now) {
  const sessions = new Map();
  return {
    create(userId) {
      const id = crypto.randomBytes(32).toString("base64url");
      sessions.set(id, { userId, expires: now() + HOURS_8 });
      return id;
    },
    get(id) {
      const entry = sessions.get(id);
      if (!entry) return null;
      if (now() >= entry.expires) {
        sessions.delete(id);
        return null;
      }
      return entry.userId;
    },
    destroy(id) {
      sessions.delete(id);
    },
  };
}

const sessions = createSessions();
const sid = sessions.create("yuki");
console.log(sid.length, sessions.get(sid));

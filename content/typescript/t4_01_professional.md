@@lesson t4-professional Professional TypeScript
@@concepts testing, modules
@@text
## A project

```bash,norun
mkdir alert-tool && cd alert-tool
npm init -y
npm install --save-dev typescript @types/node
npx tsc --init
```

A good `tsconfig.json` for a Node.js tool that's run directly with Node:

```json
{
  "compilerOptions": {
    "target": "es2023",
    "module": "nodenext",
    "strict": true,
    "noUncheckedIndexedAccess": true,
    "exactOptionalPropertyTypes": true,
    "noEmit": true,
    "allowImportingTsExtensions": true,
    "verbatimModuleSyntax": true,
    "erasableSyntaxOnly": true,
    "skipLibCheck": true
  }
}
```

- **`strict: true`** is non-negotiable in new projects. It includes `strictNullChecks`, without which `null` fits every type and half of TypeScript's value disappears.
- `noUncheckedIndexedAccess`: `arr[5]` and `record["x"]` include `undefined`.
- `erasableSyntaxOnly` and `verbatimModuleSyntax` make `tsc` reject anything Node's type stripping can't run (enums, parameter properties, type imports without `type`).
- `noEmit`: `tsc` only checks; Node runs the `.ts` files. For a browser app or a library you'd let `tsc` (or a bundler like Vite or esbuild) emit JavaScript instead.

Scripts in `package.json`:

```json
{
  "type": "module",
  "scripts": {
    "check": "tsc",
    "test": "node --test",
    "start": "node src/main.ts"
  }
}
```

Run `npm run check` in CI on every push: a type error fails the build like a failing test.

## Testing with node:test

Node has a built-in test runner, so there's nothing extra to install:

```typescript,norun
// subnet.test.ts
import { test } from "node:test";
import assert from "node:assert/strict";
import { parseCidr } from "./subnet.ts";

test("parses a /24", () => {
  assert.deepEqual(parseCidr("10.0.0.0/24"), { network: 0x0a000000, prefix: 24 });
});

test("rejects bad prefixes", () => {
  assert.throws(() => parseCidr("10.0.0.0/33"), /prefix/);
});
```

`node --test` finds `*.test.ts` files and runs them. Vitest and Jest are popular alternatives in front-end projects.

## Types for JavaScript libraries

Packages written in JavaScript come with type declarations (`.d.ts` files), either inside the package or from DefinitelyTyped: `npm install -D @types/node @types/express`. A `.d.ts` file contains only types:

```typescript,norun
// legacy-billing.d.ts: describing an old JS module you can't change
declare module "legacy-billing" {
  export function invoice(customerId: string, amountYen: number): Promise<{ id: string }>;
}
```

## Linting and formatting

- **ESLint** with `typescript-eslint` catches bugs types can't: floating promises (forgotten `await`), unused variables, `any` creeping in.
- **Prettier** or Biome formats the code, so reviews are about logic, not commas.

## Migrating JavaScript to TypeScript

Most companies have JavaScript to migrate. The safe path:

1. Add `tsconfig.json` with `"allowJs": true, "checkJs": false`.
2. Rename files one at a time from `.js` to `.ts`, fixing errors as you go; start with the leaves (files with no imports).
3. Use `unknown` and narrowing rather than `any`; mark the remaining `any`s with `// TODO: type this`.
4. Turn on `strict` early, even if that means some `// @ts-expect-error` comments to clean up later.

## Runtime validation

Types end at your program's edges. Everything that comes in (HTTP bodies, files, environment variables, queue messages) must be checked at run time. **Zod** is the most common library:

```typescript,norun
import { z } from "zod";

const Alert = z.object({
  name: z.string().min(1),
  severity: z.enum(["info", "warning", "critical"]),
  labels: z.record(z.string(), z.string()),
});
type Alert = z.infer<typeof Alert>;          // the TypeScript type, derived from the schema

const parsed = Alert.safeParse(JSON.parse(body));
if (!parsed.success) console.error(parsed.error.issues);
```

One schema gives you both the validation and the type. That's the pattern in modern APIs (tRPC, Hono, Next.js server actions).

@@quiz
Which compiler option matters most for a new project?
* `"strict": true`
- `"target": "es5"`
- `"allowJs": true`
- `"noEmit": false`
? `strict` turns on null checks and other essential checks.

@@quiz
What does `noUncheckedIndexedAccess` change?
* Indexing an array or record gives `T | undefined`, so you must handle missing items
- Arrays can't be indexed
- It disables bounds checks at run time
- Nothing at compile time
? It catches the classic "undefined is not an object" bug.

@@quiz
What's in a `.d.ts` file?
* Only type declarations, describing JavaScript code
- Compiled JavaScript
- Tests
- Configuration
? `@types/*` packages are collections of `.d.ts` files.

@@quiz
Why validate a request body with Zod when the handler already has types?
* Types are erased; at run time the body can be anything the client sends
- Zod makes it faster
- TypeScript requires it
- It isn't needed
? Validate at the edges, then trust the types inside.

@@flashcards
`strict` :: the essential set of type checks
`noEmit` :: tsc only checks; something else runs the code
`node --test` :: Node's built-in test runner
`@types/x` :: type declarations for a JS package
`.d.ts` :: a declaration file: types only
`z.infer<typeof Schema>` :: derive a type from a Zod schema
`// @ts-expect-error` :: allow a known error (fails if the error disappears)
typescript-eslint :: lint rules that use type information

@@exercise Environment config
Write `loadConfig(env: Record<string, string | undefined>)` returning `{ port: number; logLevel: "debug" | "info" | "warn"; dbUrl: string }`. `PORT` defaults to `8080` and must be a whole number 1–65535; `LOG_LEVEL` defaults to `"info"` and must be one of the three; `DATABASE_URL` is required. Collect **every** problem and throw one `Error` whose message lists them joined with `"; "`.
@@starter
type Config = { port: number; logLevel: "debug" | "info" | "warn"; dbUrl: string };

function loadConfig(env: Record<string, string | undefined>): Config {
  throw new Error("TODO: not done yet");
}

@@tests
check(loadConfig({ DATABASE_URL: "postgres://db01/app" }), { port: 8080, logLevel: "info", dbUrl: "postgres://db01/app" }, "defaults");
check(loadConfig({ PORT: "3000", LOG_LEVEL: "debug", DATABASE_URL: "x" }).port, 3000, "port from env");
let msg = "";
try { loadConfig({ PORT: "http", LOG_LEVEL: "loud" }); } catch (e) { msg = (e as Error).message; }
check(msg, "PORT must be 1-65535; LOG_LEVEL must be debug, info or warn; DATABASE_URL is required", "all problems at once");
@@hint
`const problems: string[] = [];` Parse the port with `Number(env.PORT ?? "8080")` and check `Number.isInteger`. Check the level with a `const LEVELS = ["debug", "info", "warn"] as const` array and `includes`. At the end, `if (problems.length) throw new Error(problems.join("; "));`.
@@solution
type Config = { port: number; logLevel: "debug" | "info" | "warn"; dbUrl: string };

const LEVELS = ["debug", "info", "warn"] as const;

function isLevel(x: string): x is Config["logLevel"] {
  return (LEVELS as readonly string[]).includes(x);
}

function loadConfig(env: Record<string, string | undefined>): Config {
  const problems: string[] = [];
  const port = Number(env.PORT ?? "8080");
  if (!Number.isInteger(port) || port < 1 || port > 65535) problems.push("PORT must be 1-65535");
  const level = env.LOG_LEVEL ?? "info";
  if (!isLevel(level)) problems.push("LOG_LEVEL must be debug, info or warn");
  const dbUrl = env.DATABASE_URL;
  if (!dbUrl) problems.push("DATABASE_URL is required");
  if (problems.length > 0 || !dbUrl || !isLevel(level)) throw new Error(problems.join("; "));
  return { port, logLevel: level, dbUrl };
}

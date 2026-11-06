@@lesson t7-tooling Build Tooling, tsconfig and Publishing Packages
@@concepts modules, files
@@text
Knowing the type system isn't enough: in real projects you also have to understand **how TypeScript gets built and shipped**. Most "TypeScript problems" at work are really configuration problems.

## Who does what

| Tool | Type-checks? | Emits JS? | Typical use |
|---|---|---|---|
| `tsc` | yes | yes (or `--noEmit`) | CI type checking, emitting `.d.ts` for libraries |
| esbuild, swc, Babel | no | yes, very fast | bundlers (Vite), test runners |
| Node 22.18+/24 type stripping | no | runs `.ts` directly | scripts, servers (erasable syntax only) |
| tsx, ts-node | no / optional | runs `.ts` | dev scripts |
| Vite, Next.js, Tauri templates | no (via esbuild/swc) | bundles | apps |

The usual setup: a fast tool **builds**, and `tsc --noEmit` **checks** in CI and in your editor. That's why code can run fine while having type errors: the build tool never checked.

## The tsconfig options that matter

```json,norun
{
  "compilerOptions": {
    "target": "es2023",              // which JS syntax to output
    "module": "nodenext",            // module system: nodenext for Node, esnext/preserve for bundlers
    "moduleResolution": "nodenext",  // how imports are found (bundler for Vite etc.)
    "strict": true,
    "noUncheckedIndexedAccess": true,
    "verbatimModuleSyntax": true,    // type-only imports must say `import type`
    "erasableSyntaxOnly": true,      // forbid enum/namespace/parameter properties (Node type stripping)
    "declaration": true,             // emit .d.ts (libraries)
    "outDir": "dist",
    "rootDir": "src",
    "skipLibCheck": true             // don't type-check .d.ts in node_modules (faster)
  },
  "include": ["src"]
}
```

- `extends` lets projects share a base config (`"extends": "@tsconfig/node22/tsconfig.json"`). Options merge shallowly; `include`/`files` are replaced, not merged.
- **Project references** (`"references"` + `"composite": true`) split a monorepo into separately built parts with `tsc --build`.

## ES modules vs CommonJS

Node decides per file: `.mjs`/`.mts` are ESM, `.cjs`/`.cts` are CommonJS, and `.js`/`.ts` follow `"type"` in the nearest package.json. With `module: nodenext`, relative imports need the **runtime** extension: `import { x } from "./util.js"` (even though the file is `util.ts`), or `./util.ts` with `allowImportingTsExtensions` when running TypeScript directly.

## Publishing a library

The package.json `exports` field controls what consumers can import and which file each environment gets:

```json,norun
{
  "name": "@corp/netutils",
  "type": "module",
  "exports": {
    ".": {
      "types": "./dist/index.d.ts",
      "import": "./dist/index.js",
      "require": "./dist/index.cjs"
    },
    "./cidr": {
      "types": "./dist/cidr.d.ts",
      "default": "./dist/cidr.js"
    },
    "./package.json": "./package.json"
  },
  "files": ["dist"]
}
```

Conditions are checked **in object order** and the first match wins, so `types` goes first. Anything not listed in `exports` can't be imported. Tools like `tsup` build ESM + CJS + `.d.ts` in one step, and `@arethetypeswrong/cli` checks that the result resolves correctly everywhere.

## Linting and formatting

- **typescript-eslint** adds rules that use type information: `no-floating-promises` (forgotten `await`), `no-misused-promises`, `switch-exhaustiveness-check`, `no-unsafe-*` (any leaking).
- **Prettier** or **Biome** for formatting; Biome also lints and is very fast.
- Run `tsc --noEmit`, the linter and tests in CI on every pull request.

## Monorepos

npm/pnpm/yarn **workspaces** link local packages together. Combine with project references, or with a tool like Turborepo or Nx that caches builds. Share one `tsconfig.base.json` and keep each package's config small.

@@quiz
Your Vite app runs fine but CI fails on `tsc --noEmit`. Why can that happen?
* Vite builds with esbuild, which strips types without checking them
- Vite uses a different TypeScript version that ignores errors
- tsc is broken
- CI uses JavaScript
? Builders strip types; only tsc (or your editor) checks them.

@@quiz
In package.json `exports`, why should `"types"` come first in a condition object?
- Alphabetical order is required
* Conditions match in order; TypeScript stops at the first one that applies
- Node ignores it otherwise
- It doesn't matter
? Put `types` first, `default` last.

@@quiz
With `module: nodenext`, how do you import `./util.ts` from another TS file?
- `import "./util"`
* `import "./util.js"` (the runtime extension)
- `import "util.ts"`
- `require("./util")`
? Node's ESM requires extensions; TS maps `.js` back to `.ts`.

@@drill
Which tsconfig option makes the compiler emit .d.ts files?
= declaration
= "declaration": true
= declaration: true
? Needed when publishing a library.

@@flashcards
`tsc --noEmit` :: type-check only
esbuild / swc :: fast type-stripping compilers
`target` :: output JavaScript version
`module` / `moduleResolution` :: module system and import lookup
`verbatimModuleSyntax` :: requires `import type` for types
`erasableSyntaxOnly` :: only syntax Node can strip
`extends` in tsconfig :: shared base config (shallow merge)
Project references :: `composite` + `tsc --build` for monorepos
`exports` :: package entry points with conditions
typescript-eslint :: type-aware lint rules

@@exercise Resolve a tsconfig extends chain
Write `resolveTsconfig(name: string, files: Record<string, TsConfig>): TsConfig` with `interface TsConfig { extends?: string; compilerOptions?: Record<string, unknown>; include?: string[]; exclude?: string[]; references?: { path: string }[] }`:

- Follow `extends` (names are keys in `files`) to the root, then apply configs from the root down
- `compilerOptions` merge **shallowly** (child keys override parent keys)
- `include` and `exclude` are **replaced** by the child if present
- `references` are **never inherited**: only the starting config's own references
- the result has no `extends`; omit keys that end up undefined
- a missing config throws `Error("config not found: <name>")`; a cycle throws `Error("circular extends: a -> b -> a")`
@@starter
interface TsConfig {
  extends?: string;
  compilerOptions?: Record<string, unknown>;
  include?: string[];
  exclude?: string[];
  references?: { path: string }[];
}

function resolveTsconfig(name: string, files: Record<string, TsConfig>): TsConfig {
  throw new Error("TODO: not done yet");
}

@@tests
const files: Record<string, TsConfig> = {
  base: { compilerOptions: { strict: true, target: "es2022", skipLibCheck: true }, exclude: ["node_modules"] },
  node: { extends: "base", compilerOptions: { module: "nodenext", target: "es2023" }, include: ["src"], references: [{ path: "../shared" }] },
  api: { extends: "node", compilerOptions: { outDir: "dist" }, include: ["src", "scripts"] },
  loopA: { extends: "loopB" },
  loopB: { extends: "loopA" },
};
check(resolveTsconfig("api", files), {
  compilerOptions: { strict: true, target: "es2023", skipLibCheck: true, module: "nodenext", outDir: "dist" },
  include: ["src", "scripts"],
  exclude: ["node_modules"],
}, "api config");
check(resolveTsconfig("node", files).references, [{ path: "../shared" }], "own references kept");
let msg = "";
try { resolveTsconfig("loopA", files); } catch (e) { msg = (e as Error).message; }
check(msg, "circular extends: loopA -> loopB -> loopA", "cycle");
try { resolveTsconfig("web", files); } catch (e) { msg = (e as Error).message; }
check(msg, "config not found: web", "missing");
@@hint
Build the chain first: walk `extends` while pushing names into an array; if a name is already in the array, throw with `[...chain, name].join(" -> ")`. Then loop over `chain.reverse()` merging. Build the result object last, adding only keys that are defined.
@@solution
interface TsConfig {
  extends?: string;
  compilerOptions?: Record<string, unknown>;
  include?: string[];
  exclude?: string[];
  references?: { path: string }[];
}

function resolveTsconfig(name: string, files: Record<string, TsConfig>): TsConfig {
  const chain: string[] = [];
  const configs: TsConfig[] = [];
  let current: string | undefined = name;
  while (current !== undefined) {
    if (chain.includes(current)) throw new Error(`circular extends: ${[...chain, current].join(" -> ")}`);
    const config: TsConfig | undefined = files[current];
    if (!config) throw new Error(`config not found: ${current}`);
    chain.push(current);
    configs.push(config);
    current = config.extends;
  }
  let compilerOptions: Record<string, unknown> | undefined;
  let include: string[] | undefined;
  let exclude: string[] | undefined;
  for (const config of configs.reverse()) {
    if (config.compilerOptions) compilerOptions = { ...compilerOptions, ...config.compilerOptions };
    if (config.include) include = config.include;
    if (config.exclude) exclude = config.exclude;
  }
  const own = files[name];
  const result: TsConfig = {};
  if (compilerOptions) result.compilerOptions = compilerOptions;
  if (include) result.include = include;
  if (exclude) result.exclude = exclude;
  if (own?.references) result.references = own.references;
  return result;
}

@@exercise Resolve package exports
Write `resolveExport(exportsField: ExportsField, subpath: string, conditions: string[]): string | null` that mimics how Node and TypeScript pick a file from a package.json `exports` field:

- `type ExportsField = string | { [key: string]: ExportsField }`
- If the field is a string, it's the target for `"."` only
- If its keys start with `"."`, they're subpaths: find the exact key (e.g. `"./cidr"`), or a pattern key with one `*` (e.g. `"./utils/*"`), whose `*` match is substituted into `*` in the target
- Otherwise the object is a **condition object**: go through its keys **in order** and use the first key that is `"default"` or is in `conditions`, resolving nested objects recursively; if a nested object resolves to `null`, keep looking
- Return `null` if nothing matches
@@starter
type ExportsField = string | { [key: string]: ExportsField };

function resolveExport(exportsField: ExportsField, subpath: string, conditions: string[]): string | null {
  throw new Error("TODO: not done yet");
}

@@tests
const exportsField: ExportsField = {
  ".": { types: "./dist/index.d.ts", import: "./dist/index.js", require: "./dist/index.cjs" },
  "./cidr": { node: { import: "./dist/cidr.node.js" }, default: "./dist/cidr.js" },
  "./utils/*": { types: "./dist/utils/*.d.ts", default: "./dist/utils/*.js" },
  "./package.json": "./package.json",
};
check(resolveExport(exportsField, ".", ["types", "import"]), "./dist/index.d.ts", "types first");
check(resolveExport(exportsField, ".", ["require"]), "./dist/index.cjs", "require");
check(resolveExport(exportsField, "./cidr", ["node", "require"]), "./dist/cidr.js", "nested null falls through");
check(resolveExport(exportsField, "./cidr", ["node", "import"]), "./dist/cidr.node.js", "nested match");
check(resolveExport(exportsField, "./utils/ip", ["import"]), "./dist/utils/ip.js", "pattern");
check(resolveExport(exportsField, "./internal", ["import"]), null, "not exported");
check(resolveExport("./main.js", ".", []), "./main.js", "string shorthand");
check(resolveExport({ import: "./a.mjs" }, ".", ["require"]), null, "no condition matches");
@@hint
Write a helper `pick(target: ExportsField): string | null` for condition objects (string → itself; object → loop `Object.entries`). In the main function: if every key starts with `"."` it's a subpath map; otherwise treat the whole field as the target for `"."`. For patterns: `const [pre, post] = key.split("*")` and check `subpath.startsWith(pre) && subpath.endsWith(post)`; the match is `subpath.slice(pre.length, subpath.length - post.length)`.
@@solution
type ExportsField = string | { [key: string]: ExportsField };

function pick(target: ExportsField, conditions: string[], star: string | null): string | null {
  if (typeof target === "string") return star === null ? target : target.replaceAll("*", star);
  for (const [cond, value] of Object.entries(target)) {
    if (cond === "default" || conditions.includes(cond)) {
      const found = pick(value, conditions, star);
      if (found !== null) return found;
    }
  }
  return null;
}

function resolveExport(exportsField: ExportsField, subpath: string, conditions: string[]): string | null {
  const isSubpathMap = typeof exportsField === "object" && Object.keys(exportsField).every((k) => k.startsWith("."));
  if (!isSubpathMap) return subpath === "." ? pick(exportsField, conditions, null) : null;
  const map = exportsField as { [key: string]: ExportsField };
  const exact = map[subpath];
  if (exact !== undefined) return pick(exact, conditions, null);
  for (const [key, target] of Object.entries(map)) {
    if (!key.includes("*")) continue;
    const [pre = "", post = ""] = key.split("*");
    if (subpath.startsWith(pre) && subpath.endsWith(post) && subpath.length >= pre.length + post.length) {
      return pick(target, conditions, subpath.slice(pre.length, subpath.length - post.length));
    }
  }
  return null;
}

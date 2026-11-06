@@lesson g8-tooling ES Modules, Import Maps and Front-End Tooling
@@text
Every page in this course so far has had its JavaScript inline or in one file. Real front ends have hundreds of files and dozens of dependencies. This lesson covers how browsers load modular code natively, and what the build tools you'll meet in every job (npm, Vite, bundlers, linters, formatters) actually do, so a `package.json` and a `vite.config.js` don't look like magic.

## ES modules in the browser

```html,norun
<script type="module" src="/js/main.js"></script>
```

```js,norun
// /js/format.js
export function formatDuration(s) { /* ... */ }
export const SLA_HOURS = { P1: 4, P2: 8, P3: 24, P4: 72 };
export default class TicketStore { /* ... */ }

// /js/main.js
import TicketStore, { formatDuration, SLA_HOURS } from "./format.js";
import * as api from "./api.js";
const { renderChart } = await import("./chart.js");   // dynamic import: loaded on demand
```

What `type="module"` changes:

- `import`/`export` work; each file has its own scope (no accidental globals).
- Modules are **deferred** automatically and run once, even if imported many times.
- Strict mode is always on.
- They're fetched with CORS, so they **don't work from `file://`**: you need a local web server (`python -m http.server`, or a dev server).
- Paths must be full: `"./format.js"`, not `"./format"` and not a bare `"lodash"`, unless you use an import map.

`await import()` (dynamic import) splits rarely used code (a chart library, an admin panel) into a separate download that only happens when needed.

## Import maps

An import map tells the browser what bare names mean, so you can write `import { html } from "lit"` without a bundler:

```html,norun
<script type="importmap">
{
  "imports": {
    "lit": "https://cdn.jsdelivr.net/npm/lit@3.1.2/index.js",
    "app/": "/js/"
  }
}
</script>
<script type="module">
  import { html, render } from "lit";
  import { formatDuration } from "app/format.js";
</script>
```

Fine for small sites and prototypes. For production, loading many small files from a CDN is slower than one optimised bundle, and you're trusting a third party; pin exact versions and add `integrity` hashes for CDN scripts.

## npm and package.json

**npm** (Node Package Manager, installed with Node.js) downloads libraries from the npm registry into `node_modules/` and records them:

```json
{
  "name": "it-portal",
  "private": true,
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "test": "vitest",
    "lint": "eslint src",
    "format": "prettier --write ."
  },
  "dependencies": { "lit": "^3.1.2" },
  "devDependencies": { "vite": "^5.2.0", "vitest": "^1.4.0", "eslint": "^9.0.0", "prettier": "^3.2.5" }
}
```

- `npm install` installs everything listed; `npm install lit` adds a dependency; `npm install -D vite` adds a dev dependency (tools, not shipped to users).
- `npm run dev` runs a script.
- **`package-lock.json`** records the exact version of every package (including dependencies of dependencies). Commit it; use `npm ci` in CI for reproducible installs.
- `^3.1.2` means "3.x.x, at least 3.1.2" (semver: MAJOR.MINOR.PATCH; major versions may break things).
- **Never commit `node_modules/`** (add it to `.gitignore`).
- `pnpm` and `yarn` are alternative package managers with the same ideas.

## Supply-chain security

Each dependency is code from a stranger that runs on your build machine and in your users' browsers. There have been real attacks: hijacked maintainer accounts publishing malware, typo-squatted names (`lodahs`), packages that steal environment variables during `npm install`. Defences: fewer dependencies, `npm audit`, lock files, reviewing what a package does before adding it, Dependabot or Renovate for updates, and `npm ci --ignore-scripts` where install scripts aren't needed.

## What a build tool does

**Vite** (the current default for new projects) has two modes:

- **`vite` (dev)**: a local server that serves your source files as native ES modules, transforms TypeScript/JSX on the fly, and **hot-reloads** changes in the browser without a full refresh.
- **`vite build` (production)**: uses Rollup to **bundle** modules into a few files, **tree-shake** (drop unused exports), **minify**, **split code** at dynamic imports, **fingerprint** file names (`index-3f9a1c.js`) for long caching, and process CSS and images.

Older and alternative tools do similar jobs: webpack (still common in older codebases), esbuild (very fast, used inside Vite), Parcel, Turbopack. Frameworks bundle their own setups (Next.js, Nuxt, SvelteKit, Astro).

## Code quality tools

- **ESLint**: finds bugs and bad patterns in JavaScript/TypeScript (unused variables, `==` instead of `===`, missing `await`). Plugins add accessibility rules for JSX (`eslint-plugin-jsx-a11y`).
- **Prettier**: formats code automatically, ending style arguments in code review.
- **Stylelint**: the same for CSS.
- **TypeScript**: types catch whole classes of bugs before runtime (see the TypeScript track).
- **Husky + lint-staged**: run linters on staged files before each commit.

## Deploying a static front end

`vite build` produces a `dist/` folder of static files. Host it on any static host or CDN (GitHub Pages, Netlify, Cloudflare Pages, Azure Static Web Apps, S3 + CloudFront, or a plain nginx server). Configure HTML to be revalidated and fingerprinted assets to be cached forever.

@@quiz
Why does a page with `<script type="module">` fail when opened by double-clicking the HTML file?
- Modules need TypeScript
* Modules are fetched with CORS rules, which don't allow `file://`; you need a local web server
- Browsers don't support modules
- The file needs a `.mjs` extension
? Run `python -m http.server` or a dev server.

@@quiz
What does an import map do?
* Maps bare specifiers like `"lit"` to URLs so the browser can resolve them without a bundler
- Bundles files together
- Minifies code
- Lists the site's pages
? It's native module resolution configuration.

@@quiz
Why commit `package-lock.json`?
- It contains your code
* It pins the exact versions of every package so installs are reproducible
- npm requires it to run
- It speeds up the browser
? `npm ci` installs exactly what the lock file says.

@@quiz
What is tree-shaking?
- Removing old browsers
* Dropping exported code that nothing imports from the production bundle
- Sorting files
- Compressing images
? It relies on static `import`/`export`.

@@drill
Which script type loads a file as an ES module?
= module
= type="module"
= type=module
? <script type="module" src="app.js">.

@@flashcards
`type="module"` :: ES module script; deferred, scoped
`export` / `import` :: share code between files
`await import()` :: load on demand
import map :: resolve bare names
`npm install -D` :: dev dependency
`package-lock.json` :: exact versions, commit it
`npm ci` :: clean reproducible install
`^3.1.2` :: compatible 3.x versions
`node_modules/` :: never commit
Vite dev :: native ESM + hot reload
Vite build :: bundle, tree-shake, minify, fingerprint
ESLint / Prettier :: lint / format
`npm audit` :: known vulnerabilities

@@exercise Split a page into modules with an import map
The page has all its code in one inline script. Split it (in the page, using inline module scripts) into a `<script type="importmap">` that maps `"app/format"` and `"app/store"` to the module files `/js/format.js` and `/js/store.js`, and a `<script type="module">` that imports `formatDuration` and `TicketStore` from those names. Write the contents of the two module files below the page as comments, each with `export`. (The page won't run from `file://`; the point is the structure.)
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Modules</title></head>
<body>
  <ul id="list"></ul>
  <script>
    function formatDuration(s) { const h = Math.floor(s / 3600), m = Math.floor(s % 3600 / 60); return h ? `${h}h ${m}m` : `${m}m`; }
    class TicketStore { constructor(items) { this.items = items; } open() { return this.items.filter(t => !t.closed); } }
    const store = new TicketStore([{ id: 9, age: 7200 }, { id: 11, age: 600, closed: true }]);
    for (const t of store.open()) document.getElementById("list").insertAdjacentHTML("beforeend", `<li>#${t.id} open ${formatDuration(t.age)}</li>`);
  </script>
</body>
</html>
@@requires
<script type="importmap">
"app/format": "/js/format.js"
"app/store": "/js/store.js"
<script type="module">
import { formatDuration } from "app/format"
import { TicketStore } from "app/store"
export function formatDuration
export class TicketStore
@@hint
The module scripts go after the import map. Put the file contents in an HTML comment: `<!-- /js/format.js: export function formatDuration(s) { ... } -->`. Use `textContent` with `createElement` instead of `insertAdjacentHTML` while you're at it.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Modules</title>
<script type="importmap">
{
  "imports": {
    "app/format": "/js/format.js",
    "app/store": "/js/store.js"
  }
}
</script>
</head>
<body>
  <ul id="list"></ul>
  <script type="module">
    import { formatDuration } from "app/format";
    import { TicketStore } from "app/store";
    const store = new TicketStore([{ id: 9, age: 7200 }, { id: 11, age: 600, closed: true }]);
    const list = document.getElementById("list");
    for (const t of store.open()) {
      const li = document.createElement("li");
      li.textContent = `#${t.id} open ${formatDuration(t.age)}`;
      list.append(li);
    }
  </script>
  <!--
  /js/format.js:
    export function formatDuration(s) {
      const h = Math.floor(s / 3600), m = Math.floor((s % 3600) / 60);
      return h ? `${h}h ${m}m` : `${m}m`;
    }

  /js/store.js:
    export class TicketStore {
      constructor(items) { this.items = items; }
      open() { return this.items.filter(t => !t.closed); }
    }
  -->
</body>
</html>

@@exercise Read a package.json
Answer the questions about the `package.json` in the starter by filling in the `<dd>` elements: which command starts the dev server; which packages are shipped to users (dependencies) versus build-only (devDependencies); what `^5.2.0` allows; which file should be committed to pin exact versions; and which command installs exactly what that file says in CI.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>package.json</title></head>
<body>
<pre>
{
  "name": "it-portal",
  "private": true,
  "type": "module",
  "scripts": { "dev": "vite", "build": "vite build", "test": "vitest" },
  "dependencies": { "lit": "^3.1.2", "chart.js": "^4.4.0" },
  "devDependencies": { "vite": "^5.2.0", "vitest": "^1.4.0" }
}
</pre>
<dl>
  <dt>Start the dev server</dt><dd></dd>
  <dt>Shipped to users</dt><dd></dd>
  <dt>Build-only tools</dt><dd></dd>
  <dt>What ^5.2.0 allows</dt><dd></dd>
  <dt>File that pins exact versions</dt><dd></dd>
  <dt>CI install command</dt><dd></dd>
</dl>
</body>
</html>
@@requires
npm run dev
lit, chart.js
vite, vitest
5.2.0
package-lock.json
npm ci
@@hint
Scripts run with `npm run <name>`. `^` allows newer minor and patch versions within the same major version.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>package.json</title></head>
<body>
<pre>
{
  "name": "it-portal",
  "private": true,
  "type": "module",
  "scripts": { "dev": "vite", "build": "vite build", "test": "vitest" },
  "dependencies": { "lit": "^3.1.2", "chart.js": "^4.4.0" },
  "devDependencies": { "vite": "^5.2.0", "vitest": "^1.4.0" }
}
</pre>
<dl>
  <dt>Start the dev server</dt><dd><code>npm run dev</code></dd>
  <dt>Shipped to users</dt><dd>lit, chart.js</dd>
  <dt>Build-only tools</dt><dd>vite, vitest</dd>
  <dt>What ^5.2.0 allows</dt><dd>Any 5.x.y version that is at least 5.2.0 (minor and patch updates), but not 6.0.0</dd>
  <dt>File that pins exact versions</dt><dd><code>package-lock.json</code></dd>
  <dt>CI install command</dt><dd><code>npm ci</code></dd>
</dl>
</body>
</html>

@@exercise Lazy-load a heavy module
The "Show chart" button should load the chart code only when clicked. In a `<script type="module">`, add a click listener that does `const { drawChart } = await import(...)` from a `data:` URL module (so it works without a server: `"data:text/javascript," + encodeURIComponent(source)`), then calls `drawChart(canvas)`. The module's source defines `export function drawChart(canvas)` that draws five bars with the canvas 2D API. Disable the button and set `aria-busy="true"` on the chart area while loading.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Lazy chart</title></head>
<body>
  <button id="show">Show chart</button>
  <div id="area"><canvas id="c" width="300" height="150" role="img" aria-label="Tickets per weekday, rising from 4 to 11"></canvas></div>
</body>
</html>
@@requires
<script type="module">
await import(
data:text/javascript,
encodeURIComponent(
export function drawChart(canvas)
aria-busy
getContext("2d")
@@hint
`const source = `export function drawChart(canvas) { const ctx = canvas.getContext("2d"); [4, 7, 5, 9, 11].forEach((v, i) => { ctx.fillStyle = "#2563eb"; ctx.fillRect(10 + i * 58, 150 - v * 12, 40, v * 12); }); }`;`. In a real project the module is a normal file and the bundler splits it out automatically at `import()`.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Lazy chart</title></head>
<body>
  <button id="show">Show chart</button>
  <div id="area"><canvas id="c" width="300" height="150" role="img" aria-label="Tickets per weekday, rising from 4 to 11"></canvas></div>
  <script type="module">
    const source = `
      export function drawChart(canvas) {
        const ctx = canvas.getContext("2d");
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        [4, 7, 5, 9, 11].forEach((v, i) => {
          ctx.fillStyle = "#2563eb";
          ctx.fillRect(10 + i * 58, 150 - v * 12, 40, v * 12);
        });
      }`;
    const button = document.getElementById("show");
    const area = document.getElementById("area");
    button.addEventListener("click", async () => {
      button.disabled = true;
      area.setAttribute("aria-busy", "true");
      try {
        const { drawChart } = await import("data:text/javascript," + encodeURIComponent(source));
        drawChart(document.getElementById("c"));
      } finally {
        area.setAttribute("aria-busy", "false");
        button.disabled = false;
      }
    });
  </script>
</body>
</html>

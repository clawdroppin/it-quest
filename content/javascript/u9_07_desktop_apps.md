@@lesson u9-desktop Desktop Apps with Electron and Tauri
@@concepts processes, networking, objects
@@text
Web technologies also build desktop apps: VS Code, Slack, Discord, Teams, Postman and 1Password use **Electron**; this learning app itself uses **Tauri**. You write the UI in HTML/CSS/JavaScript (React, Svelte, or plain DOM) and get a real installable app with menus, tray icons, file access and auto-updates.

## Electron vs Tauri

| | Electron | Tauri |
|---|---|---|
| Rendering | bundles its own Chromium | uses the OS web view (WebView2, WKWebView, WebKitGTK) |
| Back end | Node.js (main process) | Rust (with JavaScript or Rust plugins) |
| App size | ~80–150 MB | ~3–15 MB |
| Memory | higher | lower |
| Consistency | identical rendering everywhere | small differences between web views |
| Ecosystem | very large, mature | growing fast; also targets iOS/Android (Tauri 2) |

Both use the same architecture: an untrusted **renderer** (the web page) and a privileged **back end** that can touch the file system, the OS and the network, talking through **IPC** (inter-process communication).

## Electron's processes

```text
 ┌── main process (Node.js) ───────────────┐      ┌── renderer (Chromium page) ──┐
 │ app lifecycle, windows, menus, tray      │ IPC  │ your React/HTML UI            │
 │ fs, child_process, auto-update           │◀────▶│ NO Node.js access             │
 │ ipcMain.handle("tickets:list", ...)      │      │ window.api.listTickets()      │
 └─────────────────────────────────────────┘      └───────────────▲──────────────┘
                                                   preload script │ contextBridge
```

```javascript,norun
// main.js
const { app, BrowserWindow, ipcMain } = require("electron");
const path = require("node:path");

app.whenReady().then(() => {
  const win = new BrowserWindow({
    width: 1100, height: 700,
    webPreferences: {
      preload: path.join(__dirname, "preload.js"),
      contextIsolation: true,      // default: the page can't reach preload internals
      sandbox: true,               // default: renderer has no Node.js
      nodeIntegration: false,      // never true for remote or untrusted content
    },
  });
  win.loadFile("index.html");
});

ipcMain.handle("tickets:list", async (event, filter) => {
  if (typeof filter?.status !== "string") throw new Error("bad filter");   // validate everything
  return db.listTickets(filter);
});

// preload.js: expose a small, specific API, never ipcRenderer itself
const { contextBridge, ipcRenderer } = require("electron");
contextBridge.exposeInMainWorld("api", {
  listTickets: (filter) => ipcRenderer.invoke("tickets:list", filter),
});

// renderer (your React code)
const tickets = await window.api.listTickets({ status: "open" });
```

## Tauri's model

```rust,norun
#[tauri::command]
fn list_tickets(status: String) -> Result<Vec<Ticket>, String> {
    db::list(&status).map_err(|e| e.to_string())
}
```

```javascript,norun
import { invoke } from "@tauri-apps/api/core";
const tickets = await invoke("list_tickets", { status: "open" });
```

Tauri 2 adds **capabilities**: a configuration that lists exactly which commands and plugin permissions (file system paths, shell, HTTP domains) each window may use. Everything else is denied.

## Security: treat the renderer as hostile

A renderer can be compromised by an XSS bug, a malicious link or a bad dependency. The back end must assume any IPC message may be an attack:

- Keep `contextIsolation`, `sandbox` and `nodeIntegration: false` (Electron defaults since v20).
- Expose **narrow, specific functions** ("listTickets"), never generic ones ("runCommand", "readFile(anyPath)").
- **Validate** every IPC argument (schemas from the forms lesson work here) and check which window/origin sent it.
- Set a strict **Content-Security-Policy**; never load remote websites into a privileged window. Open external links in the user's browser (`shell.openExternal` after checking the URL scheme).
- Electron's **fuses** can disable risky features (like running as plain Node via `ELECTRON_RUN_AS_NODE`) in the packaged binary.

## Packaging, signing and updates

- **electron-builder** or **Electron Forge** produce installers (`.exe`/MSI, `.dmg`, AppImage/deb/rpm); `tauri build` does the same for Tauri.
- **Code signing**: Windows (Authenticode certificate or Azure Trusted Signing) and macOS (Developer ID + notarisation) are needed to avoid scary warnings.
- **Auto-update**: `electron-updater` or Tauri's updater plugin download signed updates from GitHub Releases or your server. Updates must be signed; the app verifies them before installing.
- In companies, apps are often deployed with Intune, SCCM or winget instead; per-machine MSI installers suit that.

@@quiz
Why does the renderer process not get Node.js access in a secure Electron app?
* A compromised page (XSS, malicious content) would otherwise be able to run commands and read files on the user's machine
- Node.js is too slow in renderers
- Chromium doesn't support JavaScript
- It's only for Tauri
? Expose a small API through the preload script instead.

@@quiz
Which preload API is safer?
- `exposeInMainWorld("api", { ipcRenderer })`
- `exposeInMainWorld("api", { run: (cmd) => ipcRenderer.invoke("run", cmd) })`
* `exposeInMainWorld("api", { listTickets: (f) => ipcRenderer.invoke("tickets:list", f) })`
- Enabling nodeIntegration
? Narrow, specific functions limit what a compromised page can do.

@@quiz
What's the main reason Tauri apps are much smaller than Electron apps?
- Tauri compresses JavaScript better
* Tauri uses the operating system's web view instead of bundling Chromium
- Tauri doesn't support CSS
- Electron includes Python
? The trade-off is small rendering differences between platforms.

@@flashcards
Electron :: desktop apps with bundled Chromium and Node.js
Tauri :: desktop/mobile apps with the OS web view and a Rust back end
Main process :: Electron's privileged Node.js process
Renderer :: the web page process showing the UI
Preload script :: bridge exposing a safe API to the renderer
contextBridge :: Electron API for exposing functions to pages
IPC :: messages between renderer and back end
Tauri capabilities :: allow-list of commands and permissions per window
Code signing :: proves the publisher; avoids OS warnings
Auto-update :: downloading and verifying signed new versions

@@exercise A validated IPC router
Write `createIpcRouter(handlers)` for the back-end side. `handlers` maps channel names to `{ validate, handle, windows }`:

- `validate(args)` returns an error message string or `null`
- `handle(args)` returns a value or a promise
- `windows` (optional) is a list of window labels allowed to call this channel

The router returns `async dispatch(sender, channel, args)` where `sender` is `{ window, origin }`. It must:

- reject unknown channels: `{ ok: false, error: "unknown channel" }`
- reject senders whose `origin` isn't `"app://local"`: `{ ok: false, error: "untrusted origin" }`
- reject windows not in `windows` (when given): `{ ok: false, error: "forbidden" }`
- return `{ ok: false, error: "invalid: <message>" }` when validation fails
- return `{ ok: true, value }` on success; if the handler throws, return `{ ok: false, error: "internal error" }` (never leak error details to the renderer), and record the real message in `router.errors`
@@starter
function createIpcRouter(handlers) {
  throw new Error("TODO: not done yet");
}

@@tests
const router = createIpcRouter({
  "tickets:list": {
    validate: (a) => (["open", "closed"].includes(a?.status) ? null : "status must be open or closed"),
    handle: async (a) => [`${a.status} ticket`],
  },
  "settings:save": {
    validate: () => null,
    handle: () => { throw new Error("disk full at C:\\Users\\ana"); },
    windows: ["settings"],
  },
});
const main = { window: "main", origin: "app://local" };
(async () => {
  check(await router.dispatch(main, "tickets:list", { status: "open" }), { ok: true, value: ["open ticket"] }, "ok");
  check(await router.dispatch(main, "tickets:list", { status: "x" }), { ok: false, error: "invalid: status must be open or closed" }, "invalid");
  check(await router.dispatch(main, "fs:readAny", {}), { ok: false, error: "unknown channel" }, "unknown");
  check(await router.dispatch({ window: "main", origin: "https://evil.example" }, "tickets:list", { status: "open" }), { ok: false, error: "untrusted origin" }, "origin");
  check(await router.dispatch(main, "settings:save", {}), { ok: false, error: "forbidden" }, "window not allowed");
  check(await router.dispatch({ window: "settings", origin: "app://local" }, "settings:save", {}), { ok: false, error: "internal error" }, "errors hidden");
  check(router.errors, ["settings:save: disk full at C:\\Users\\ana"], "real error recorded");
  check(await router.dispatch(main, "__proto__", {}), { ok: false, error: "unknown channel" }, "prototype keys are not channels");
})();
@@hint
Check `Object.hasOwn(handlers, channel)` (not `channel in handlers`, which finds `__proto__`). Then the origin, the window list, validation, and finally `try { return { ok: true, value: await h.handle(args) }; } catch (e) { router.errors.push(`${channel}: ${e.message}`); return { ok: false, error: "internal error" }; }`.
@@solution
function createIpcRouter(handlers) {
  const router = { errors: [] };
  router.dispatch = async (sender, channel, args) => {
    if (!Object.hasOwn(handlers, channel)) return { ok: false, error: "unknown channel" };
    if (sender.origin !== "app://local") return { ok: false, error: "untrusted origin" };
    const h = handlers[channel];
    if (h.windows && !h.windows.includes(sender.window)) return { ok: false, error: "forbidden" };
    const problem = h.validate(args);
    if (problem) return { ok: false, error: `invalid: ${problem}` };
    try {
      return { ok: true, value: await h.handle(args) };
    } catch (e) {
      router.errors.push(`${channel}: ${e.message}`);
      return { ok: false, error: "internal error" };
    }
  };
  return router;
}

@@exercise Update checker
Write `checkForUpdate(current, feed, { channel = "stable", now })`:

- `feed` is a list of releases `{ version, channel, publishedAt, minimumVersion?, rollout? }` (versions `MAJOR.MINOR.PATCH`, optionally with a `-beta.N` suffix)
- consider releases on the requested channel; the `"beta"` channel also sees stable releases
- ignore releases published in the future (`publishedAt` ISO string later than `now`)
- staged rollout: a release with `rollout` (0–100) is only offered if `bucket < rollout`, where `bucket` is passed in `options.bucket` (0–99, a stable per-machine number)
- pick the highest version greater than `current` (a pre-release like `2.0.0-beta.2` is lower than `2.0.0` and compares its number numerically)
- return `null` if none, else `{ version, mandatory }` where `mandatory` is true if `current` is lower than that release's `minimumVersion`
@@starter
function checkForUpdate(current, feed, { channel = "stable", now, bucket = 0 } = {}) {
  throw new Error("TODO: not done yet");
}

@@tests
const feed = [
  { version: "1.4.0", channel: "stable", publishedAt: "2026-10-01T00:00:00Z" },
  { version: "1.5.0", channel: "stable", publishedAt: "2026-11-01T00:00:00Z", minimumVersion: "1.3.0", rollout: 50 },
  { version: "2.0.0-beta.10", channel: "beta", publishedAt: "2026-10-20T00:00:00Z" },
  { version: "2.0.0-beta.9", channel: "beta", publishedAt: "2026-10-10T00:00:00Z" },
  { version: "1.6.0", channel: "stable", publishedAt: "2026-12-01T00:00:00Z" },
];
const now = "2026-11-03T00:00:00Z";
check(checkForUpdate("1.3.2", feed, { now, bucket: 10 }), { version: "1.5.0", mandatory: false }, "staged rollout includes bucket 10");
check(checkForUpdate("1.2.0", feed, { now, bucket: 10 }), { version: "1.5.0", mandatory: true }, "below minimum");
check(checkForUpdate("1.3.2", feed, { now, bucket: 70 }), { version: "1.4.0", mandatory: false }, "outside rollout");
check(checkForUpdate("1.5.0", feed, { now, bucket: 10 }), null, "up to date; future release ignored");
check(checkForUpdate("1.5.0", feed, { channel: "beta", now, bucket: 10 }), { version: "2.0.0-beta.10", mandatory: false }, "beta numeric pre-release order");
@@hint
Parse `"2.0.0-beta.10"` into `{ nums: [2, 0, 0], pre: 10 }` (no suffix → `pre: Infinity`, so releases sort above their pre-releases). Compare `nums` element by element, then `pre`. Filter the feed, then take the maximum.
@@solution
function parseVersion(v) {
  const [main, pre] = v.split("-");
  const n = pre ? Number(pre.split(".").pop()) : Infinity;
  return { nums: main.split(".").map(Number), pre: n };
}

function compareVersions(a, b) {
  const x = parseVersion(a);
  const y = parseVersion(b);
  for (let i = 0; i < 3; i++) if (x.nums[i] !== y.nums[i]) return x.nums[i] - y.nums[i];
  if (x.pre === y.pre) return 0;
  return x.pre > y.pre ? 1 : -1;
}

function checkForUpdate(current, feed, { channel = "stable", now, bucket = 0 } = {}) {
  const channels = channel === "beta" ? ["beta", "stable"] : [channel];
  const nowMs = Date.parse(now);
  const candidates = feed.filter((r) =>
    channels.includes(r.channel)
    && Date.parse(r.publishedAt) <= nowMs
    && (r.rollout === undefined || bucket < r.rollout)
    && compareVersions(r.version, current) > 0);
  if (candidates.length === 0) return null;
  const best = candidates.reduce((a, b) => (compareVersions(b.version, a.version) > 0 ? b : a));
  return {
    version: best.version,
    mandatory: best.minimumVersion !== undefined && compareVersions(current, best.minimumVersion) < 0,
  };
}

@@lesson h5-offline-storage Offline Apps: Storage, IndexedDB, Service Workers and Installable PWAs
@@text
A web app that loses everything when the Wi-Fi drops, or shows the browser's dinosaur on a train, feels broken next to a native app. The web platform has everything needed to fix that: several kinds of storage, a background script that can answer requests while offline, and a manifest that lets users install the app like any other. This lesson covers each piece and how they fit together.

## Choosing where to store data

| Storage | Holds | Size | Survives | Use for |
|---|---|---|---|---|
| `localStorage` | strings, synchronous API | ~5 MB per origin | until cleared | small preferences: theme, last tab |
| `sessionStorage` | strings | ~5 MB | until the tab closes | per-tab state, a multi-step form |
| **IndexedDB** | structured objects, Blobs, indexes; async | a large share of free disk | until evicted or cleared | app data, offline queues, cached records |
| **Cache API** | HTTP request/response pairs | shares the IndexedDB quota | until evicted or cleared | app shell files, API responses |
| Cookies | small strings sent with every request | ~4 KB each | per expiry | sessions (server-side), not app data |

Points that matter in practice:

- `localStorage` is **synchronous**: a large read blocks the main thread. Keep it small and store JSON with `JSON.stringify` and `JSON.parse` (wrapped in `try`, because private modes and full disks throw).
- Storage is **per origin** (scheme + host + port). `http://localhost:5173` and `http://localhost:3000` don't share anything.
- Browsers can **evict** storage under disk pressure, starting with the least recently used origins. `navigator.storage.persist()` asks for persistent storage (browsers grant it more readily to installed or frequently used apps), and `navigator.storage.estimate()` reports usage and quota.
- Never store secrets such as access tokens in `localStorage`: any script injected by an XSS bug can read it. HttpOnly cookies are out of reach of JavaScript.

## IndexedDB in a nutshell

IndexedDB is a transactional database of **object stores** (like tables) holding JavaScript objects, with optional **indexes**. Its API is event-based and verbose, so most apps wrap it in a few promise helpers (or use a small library such as idb):

```html,norun
<script>
function openDb() {
  return new Promise((resolve, reject) => {
    const req = indexedDB.open("notes-app", 1);
    req.onupgradeneeded = () => {
      const db = req.result;
      const store = db.createObjectStore("notes", { keyPath: "id" });
      store.createIndex("byUpdated", "updated");
    };
    req.onsuccess = () => resolve(req.result);
    req.onerror = () => reject(req.error);
  });
}

function tx(db, mode, fn) {
  return new Promise((resolve, reject) => {
    const t = db.transaction("notes", mode);
    const result = fn(t.objectStore("notes"));
    t.oncomplete = () => resolve(result.result);
    t.onerror = () => reject(t.error);
  });
}

const db = await openDb();
await tx(db, "readwrite", (s) => s.put({ id: crypto.randomUUID(), text: "Buy milk", updated: Date.now() }));
const all = await tx(db, "readonly", (s) => s.getAll());
</script>
```

- The **version number** in `open` drives schema changes: when it goes up, `onupgradeneeded` runs and is the only place you can create or delete stores and indexes.
- Every read and write happens inside a **transaction** (`readonly` or `readwrite`). A transaction commits automatically when it has no more pending requests, so don't `await` unrelated things (like `fetch`) in the middle of one; it will have closed.
- `put` inserts or replaces by key; `add` fails if the key exists; `get`, `getAll`, `delete` and `count` do what you'd expect; indexes give you `index("byUpdated").getAll(range)`.

## Service workers

A **service worker** is a script the browser runs separately from your pages. Once installed, it sits between your pages and the network and can answer their requests itself, including when there is no network at all.

```html,norun
<script>
if ("serviceWorker" in navigator) {
  navigator.serviceWorker.register("/sw.js");
}
</script>
```

Rules:

- Service workers need a **secure context**: HTTPS, or `localhost` during development.
- The worker's **scope** is the folder it's served from: `/sw.js` controls the whole site, `/app/sw.js` only `/app/`.
- It has no DOM. It talks to pages with `postMessage` and uses `fetch`, the Cache API and IndexedDB.

The **lifecycle** is where most confusion comes from:

1. **install**: runs once per new version of the worker file. The usual job is to **precache the app shell** (HTML, CSS, JS, icons).
2. **waiting**: if an older worker is still controlling open tabs, the new one **waits** until they're all closed, so a page never runs with two different versions. `self.skipWaiting()` skips the wait (and you then usually reload the page or tell the user an update is ready).
3. **activate**: the new worker takes over. The usual job is deleting **old caches**. `clients.claim()` makes it control already-open pages immediately.
4. **fetch**: fires for every request from controlled pages. Call `event.respondWith(...)` to answer.

```js,norun
// sw.js
const CACHE = "shell-v3";
const SHELL = ["/", "/index.html", "/app.css", "/app.js", "/offline.html"];

self.addEventListener("install", (event) => {
  event.waitUntil(caches.open(CACHE).then((c) => c.addAll(SHELL)));
});

self.addEventListener("activate", (event) => {
  event.waitUntil(
    caches.keys().then((keys) => Promise.all(keys.filter((k) => k !== CACHE).map((k) => caches.delete(k))))
  );
});

self.addEventListener("fetch", (event) => {
  if (event.request.mode === "navigate") {
    event.respondWith(fetch(event.request).catch(() => caches.match("/offline.html")));
    return;
  }
  event.respondWith(caches.match(event.request).then((hit) => hit || fetch(event.request)));
});
```

Changing the **cache name** (`shell-v3` to `shell-v4`) together with the file list is the simplest way to roll out a new version: the new worker installs, precaches into a new cache, and deletes the old one on activate.

## Caching strategies

Different requests deserve different strategies:

| Strategy | How | Good for |
|---|---|---|
| **Cache first** | answer from the cache, fall back to the network | versioned static files (`app.3f9a1c.js`), fonts, icons |
| **Network first** | try the network, fall back to the cache when offline | HTML pages, API data that must be fresh when possible |
| **Stale-while-revalidate** | answer from the cache immediately, update the cache from the network in the background | avatars, feeds, data where slightly old is fine |
| **Network only** | never cache | payments, analytics, anything with side effects |
| **Cache only** | never hit the network | the precached app shell |

Two traps: caching **non-GET** requests (the Cache API only stores GET; never replay a POST from cache), and caching **error responses** (check `response.ok` before `cache.put`). Libraries like Workbox package these strategies with expiry rules, but understanding the raw version makes debugging much easier.

## Offline writes

Reading offline is the easy half. For writes, save the change locally first (IndexedDB), update the UI immediately, and keep an **outbox** of pending changes. When the browser fires the `online` event (or a Background Sync event, where supported), send the queued changes in order and remove each one when the server confirms it. Give each change an ID so retrying it twice can't create a duplicate on the server.

Tell users what's happening: an `aria-live="polite"` status such as "Offline: changes will sync when you reconnect" builds trust, and `navigator.onLine` plus the `online`/`offline` events tell you when to update it. Note that `navigator.onLine` being true only means there's *a* network connection, not that your server is reachable, so treat failed requests as offline too.

## Installable apps: the manifest

A **web app manifest** makes the site installable: it appears in the app list, opens in its own window, and has a proper icon.

```html,norun
<link rel="manifest" href="/manifest.webmanifest">
<meta name="theme-color" content="#4f46e5">
```

```json
{
  "name": "Field Notes",
  "short_name": "Notes",
  "start_url": "/?source=pwa",
  "scope": "/",
  "display": "standalone",
  "background_color": "#ffffff",
  "theme_color": "#4f46e5",
  "icons": [
    { "src": "/icons/192.png", "sizes": "192x192", "type": "image/png" },
    { "src": "/icons/512.png", "sizes": "512x512", "type": "image/png" },
    { "src": "/icons/maskable-512.png", "sizes": "512x512", "type": "image/png", "purpose": "maskable" }
  ]
}
```

- `display: "standalone"` removes the browser UI; `"minimal-ui"` keeps a small toolbar.
- Provide at least **192 and 512 px** icons, plus a **maskable** icon with its important content inside the central safe zone, because Android crops icons into circles and other shapes.
- Chromium browsers fire `beforeinstallprompt`, which you can save and trigger from your own "Install" button. Safari on iOS installs through Share → Add to Home Screen.

## Testing offline behaviour

In Chromium DevTools, the **Application** panel shows the manifest, service workers (with "Update on reload" and "Bypass for network" for development), Cache Storage, IndexedDB and storage usage. The **Network** panel's throttling menu has an **Offline** option. Test the full journey: first visit online, reload offline, make a change offline, reconnect, and check that it synced exactly once.

@@quiz
Why is localStorage a poor place for an app's main data?
* It's synchronous, small and string-only; IndexedDB is asynchronous and stores structured data
- It is deleted when the tab closes
- It's shared between all websites
- It can only hold numbers
? Keep localStorage for small preferences.

@@quiz
Where can you create or delete IndexedDB object stores?
* Only in the onupgradeneeded handler, triggered by opening with a higher version
- Anywhere inside a readwrite transaction
- Only from a service worker
- With localStorage.setItem
? The version number drives schema changes.

@@quiz
A new service worker is installed but an older one still controls two open tabs. What happens by default?
* The new worker waits until those tabs are closed before it activates
- Both workers run at the same time
- The old worker is killed immediately
- The new worker is discarded
? self.skipWaiting() can skip the wait; clients.claim() takes control of open pages.

@@quiz
Which caching strategy suits versioned static files like app.3f9a1c.js?
* Cache first
- Network only
- Network first
- Never cache them
? The file name changes when the content changes, so a cached copy is never stale.

@@quiz
Why can't a service worker run on a plain http:// production site?
* Service workers require a secure context: HTTPS or localhost
- They need HTTP/3
- Browsers only allow them on .app domains
- They require a manifest first
? A script that intercepts every request must not be injectable on the network.

@@quiz
What is a maskable icon for?
* Platforms that crop icons into circles or other shapes; its content stays inside the safe zone
- Hiding the icon in dark mode
- Favicons in old browsers
- Notification badges only
? Add it to the manifest with "purpose": "maskable".

@@flashcards
localStorage :: small synchronous string storage per origin
IndexedDB :: async object stores with indexes and transactions
onupgradeneeded :: the only place to change the IndexedDB schema
Cache API :: stores request/response pairs (GET only)
navigator.storage.persist() :: ask the browser not to evict your storage
Service worker scope :: the folder the worker file is served from
install event :: precache the app shell
activate event :: delete old caches
skipWaiting / clients.claim :: activate now / control open pages now
Cache first :: static versioned files
Network first :: pages and fresh data, cache when offline
Stale-while-revalidate :: answer from cache, refresh in the background
Outbox :: queue of offline changes sent when back online
display: standalone :: installed app opens without browser UI

@@drill
Which IndexedDB event handler runs when you open a database with a higher version number?
= onupgradeneeded
= upgradeneeded

@@drill
Which caching strategy answers from the cache and refreshes it from the network in the background?
= stale-while-revalidate
= stale while revalidate

@@exercise An offline-aware notes page
@@concepts functions, json
Make the notes page offline friendly. Register a service worker at `/sw.js` only if `"serviceWorker" in navigator`. Keep the draft in `localStorage` under the key `"draft"` (save on every `input` event, restore on load, wrapping storage access in `try`). Show the connection state in a `role="status"` element with `aria-live="polite"`, updated from `navigator.onLine` and the `online` and `offline` events. Link a manifest with `rel="manifest"`.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Field Notes</title></head>
<body>
  <h1>Field Notes</h1>
  <p id="status"></p>
  <label for="note">Note</label>
  <textarea id="note" rows="6"></textarea>
  <script>
    // Your code here
  </script>
</body>
</html>
@@requires
rel="manifest"
"serviceWorker" in navigator
navigator.serviceWorker.register("/sw.js")
localStorage.setItem("draft"
aria-live="polite"
addEventListener("offline"
navigator.onLine
@@hint
Write a `showStatus()` that sets the text from `navigator.onLine` and call it on load and from both `online` and `offline` listeners. For the draft: `note.value = localStorage.getItem("draft") ?? ""` on load and `localStorage.setItem("draft", note.value)` on input, each inside `try { } catch { }`.
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Field Notes</title>
  <link rel="manifest" href="/manifest.webmanifest">
  <meta name="theme-color" content="#4f46e5">
</head>
<body>
  <h1>Field Notes</h1>
  <p id="status" role="status" aria-live="polite"></p>
  <label for="note">Note</label>
  <textarea id="note" rows="6"></textarea>
  <script>
    if ("serviceWorker" in navigator) {
      navigator.serviceWorker.register("/sw.js").catch((err) => console.warn("SW registration failed", err));
    }

    const status = document.getElementById("status");
    function showStatus() {
      status.textContent = navigator.onLine
        ? "Online: notes are saved on this device."
        : "Offline: your note is saved on this device and will be here when you come back.";
    }
    window.addEventListener("online", showStatus);
    window.addEventListener("offline", showStatus);
    showStatus();

    const note = document.getElementById("note");
    try {
      note.value = localStorage.getItem("draft") ?? "";
    } catch {
      // storage unavailable (private mode or blocked): start empty
    }
    note.addEventListener("input", () => {
      try {
        localStorage.setItem("draft", note.value);
      } catch {
        status.textContent = "Couldn't save the draft on this device.";
      }
    });
  </script>
</body>
</html>

@@exercise A task list in IndexedDB
@@concepts functions, lists, concurrency
Store tasks in IndexedDB instead of memory. Write `openDb()` that opens database `"tasks-db"` version 1 and, in `onupgradeneeded`, creates an object store `"tasks"` with `keyPath: "id"` and an index `"byDone"` on `"done"`. Write promise helpers to `put` a task and `getAll` tasks, use `crypto.randomUUID()` for ids, and render the list after each change. Toggling a checkbox updates the task's `done` field with `put`.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Tasks</title></head>
<body>
  <form id="add"><label for="t">New task</label> <input id="t" required> <button>Add</button></form>
  <ul id="list"></ul>
  <script type="module">
    const tasks = [];
    // Replace the in-memory array with IndexedDB
    document.getElementById("add").addEventListener("submit", (e) => {
      e.preventDefault();
    });
  </script>
</body>
</html>
@@requires
indexedDB.open("tasks-db", 1)
onupgradeneeded
createObjectStore("tasks", { keyPath: "id" })
createIndex("byDone", "done")
"readwrite"
getAll()
crypto.randomUUID()
@@hint
Wrap each request in a promise: `new Promise((resolve, reject) => { req.onsuccess = () => resolve(req.result); req.onerror = () => reject(req.error); })`. A helper `store(mode)` can return `db.transaction("tasks", mode).objectStore("tasks")`.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Tasks</title>
<style>
  .done span { text-decoration: line-through; color: #64748b; }
</style>
</head>
<body>
  <form id="add"><label for="t">New task</label> <input id="t" required> <button>Add</button></form>
  <ul id="list"></ul>
  <script type="module">
    function request(req) {
      return new Promise((resolve, reject) => {
        req.onsuccess = () => resolve(req.result);
        req.onerror = () => reject(req.error);
      });
    }

    function openDb() {
      const req = indexedDB.open("tasks-db", 1);
      req.onupgradeneeded = () => {
        const store = req.result.createObjectStore("tasks", { keyPath: "id" });
        store.createIndex("byDone", "done");
      };
      return request(req);
    }

    const db = await openDb();
    const store = (mode) => db.transaction("tasks", mode).objectStore("tasks");
    const putTask = (task) => request(store("readwrite").put(task));
    const allTasks = () => request(store("readonly").getAll());

    const list = document.getElementById("list");
    async function render() {
      const tasks = (await allTasks()).sort((a, b) => a.created - b.created);
      list.replaceChildren(...tasks.map((task) => {
        const li = document.createElement("li");
        li.className = task.done ? "done" : "";
        const label = document.createElement("label");
        const box = document.createElement("input");
        box.type = "checkbox";
        box.checked = task.done;
        box.addEventListener("change", async () => {
          await putTask({ ...task, done: box.checked });
          render();
        });
        const text = document.createElement("span");
        text.textContent = " " + task.title;
        label.append(box, text);
        li.append(label);
        return li;
      }));
    }

    const input = document.getElementById("t");
    document.getElementById("add").addEventListener("submit", async (e) => {
      e.preventDefault();
      await putTask({ id: crypto.randomUUID(), title: input.value.trim(), done: false, created: Date.now() });
      input.value = "";
      render();
    });
    render();
  </script>
</body>
</html>

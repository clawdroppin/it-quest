@@lesson h5-data Data in Web Apps: fetch, Storage and Routing
@@concepts json, errors, networking
@@text
Real web apps load data from servers, remember things between visits, and have several "pages" without reloading. This lesson covers the three browser features behind that: `fetch`, storage, and URL routing.

## Loading data with fetch

```js,norun
async function loadTickets() {
  const response = await fetch("/api/tickets?status=open", {
    headers: { Accept: "application/json" },
  });
  if (!response.ok) {                                  // fetch only rejects on network errors,
    throw new Error(`Server said ${response.status}`); // not on 404 or 500: check ok yourself
  }
  return response.json();
}

// Sending data
await fetch("/api/tickets", {
  method: "POST",
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify({ title, priority }),
});
```

## Every request has four states

A request can be **loading**, can **fail**, can return **nothing**, or can **succeed**. Apps that only handle the last one look broken the first time the Wi-Fi drops. Put the status in your state and let `render()` show each one:

```js,norun
let state = { status: "loading", tickets: [], error: null };

async function refresh() {
  state = { ...state, status: "loading" };
  render();
  try {
    const tickets = await loadTickets();
    state = { status: "ready", tickets, error: null };
  } catch (e) {
    state = { ...state, status: "error", error: e.message };
  }
  render();
}

function render() {
  spinner.hidden = state.status !== "loading";
  errorBox.hidden = state.status !== "error";
  errorBox.textContent = state.error ?? "";
  empty.hidden = !(state.status === "ready" && state.tickets.length === 0);
  // ... draw state.tickets
}
```

Two more details: **cancel** stale requests with an `AbortController` (a search box sends a request per keystroke, and an old, slow answer must not overwrite a newer one), and **debounce** input, waiting until the user pauses typing before sending a request:

```js,norun
let timer, controller;
search.addEventListener("input", () => {
  clearTimeout(timer);
  timer = setTimeout(async () => {
    controller?.abort();                         // cancel the previous request
    controller = new AbortController();
    const r = await fetch(`/api/search?q=${encodeURIComponent(search.value)}`, { signal: controller.signal });
    showResults(await r.json());
  }, 300);                                       // 300 ms after the last keystroke
});
```

Browsers block requests to **other origins** (another domain or port) unless that server allows it with **CORS** headers (`Access-Control-Allow-Origin`). That's a server setting; you can't fix it from the page.

## Remembering things: localStorage

```js,norun
localStorage.setItem("settings", JSON.stringify({ theme: "dark", pageSize: 50 }));
const settings = JSON.parse(localStorage.getItem("settings") ?? "{}");   // null if never saved
localStorage.removeItem("settings");
```

| Storage | Size | Lifetime | Use for |
|---|---|---|---|
| `localStorage` | ~5 MB, strings only | until cleared | settings, drafts, small app data |
| `sessionStorage` | ~5 MB | until the tab closes | per-tab state |
| **IndexedDB** | hundreds of MB, objects and files | until cleared | offline data, big lists, files |
| cookies | 4 KB, sent with every request | set by expiry | sessions (set by the server, `HttpOnly`) |

Never put tokens or secrets in `localStorage`: any script on the page (including an XSS attack) can read it. Login sessions belong in `HttpOnly` cookies, which JavaScript can't read. Also wrap storage calls in `try`/`catch`: private browsing and some company policies block storage.

## Several pages, one document: routing

A **single-page application** (SPA) changes the view without loading a new page, but the URL should still change, so Back works and links can be shared. The simplest router uses the part of the URL after `#`:

```html
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Router</title></head>
<body>
  <nav><a href="#/">Dashboard</a> · <a href="#/tickets">Tickets</a> · <a href="#/tickets/42">Ticket 42</a></nav>
  <main id="view"></main>
  <script>
    const routes = [
      [/^#\/$/, () => "Dashboard: 3 open tickets"],
      [/^#\/tickets$/, () => "All tickets"],
      [/^#\/tickets\/(\d+)$/, (id) => `Ticket #${id}`],
    ];

    function route() {
      const hash = location.hash || "#/";
      for (const [pattern, view] of routes) {
        const m = hash.match(pattern);
        if (m) {
          document.getElementById("view").textContent = view(...m.slice(1));
          return;
        }
      }
      document.getElementById("view").textContent = "Page not found";
    }

    window.addEventListener("hashchange", route);   // Back, Forward, and clicked links
    route();
  </script>
</body>
</html>
```

Clean URLs without `#` (`/tickets/42`) use the History API (`history.pushState` and the `popstate` event), and need the server to return `index.html` for every path. Frameworks' routers do exactly this.

@@quiz
`fetch("/api/tickets")` gets a 500 Internal Server Error. What happens?
- The promise rejects, so `catch` runs
* The promise resolves with `response.ok === false`, and your code must check it
- fetch retries automatically
- The browser shows an error page
? fetch only rejects when no response arrives at all (network down, CORS blocked).

@@quiz
A search box sends a request on each keystroke. Sometimes the results for "pri" appear after the results for "printer". The fix?
- Use XMLHttpRequest
* Debounce the input and abort the previous request with an AbortController
- Use localStorage
- Add `async` to the listener
? Responses can arrive in any order. Cancel requests whose answers you no longer want.

@@quiz
Where should a login session token live?
- localStorage
- sessionStorage
* An HttpOnly cookie set by the server
- A global JavaScript variable
? HttpOnly cookies can't be read by scripts, so an XSS bug can't steal them.

@@quiz
Why use URLs like `#/tickets/42` in a single-page app?
* So Back and Forward work, and a view can be bookmarked or shared
- They load faster
- Search engines require them
- They're more secure
? The URL is part of the app's state: users expect it to describe what they're looking at.

@@drill
Which browser API stores small strings that survive reloads?
= localStorage
? sessionStorage clears when the tab closes.

@@flashcards
`await fetch(url)` :: send a request; resolves with a Response
`response.ok` :: true for status 200-299
`response.json()` :: parse the body as JSON (a promise)
`JSON.stringify(obj)` :: object to JSON text, for a request body
loading / error / empty / ready :: the four states of any request
`AbortController` :: cancel a fetch
debounce :: wait until input pauses before acting
CORS :: a server's permission for other origins to call it
`localStorage.setItem(k, v)` / `getItem(k)` :: save / load a string
IndexedDB :: the browser's database, for big or offline data
HttpOnly cookie :: a cookie scripts can't read
`hashchange` :: event fired when the part after `#` changes
`history.pushState` :: change the URL without a reload

@@exercise Remember the theme
Make the "Dark mode" checkbox **remembered**: when it changes, save `"dark"` or `"light"` under the key `theme` with `localStorage.setItem`; when the page loads, read it with `localStorage.getItem` and set the checkbox and the class to match. Wrap both in `try`/`catch` so a browser that blocks storage still works.
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Settings</title>
  <style>
    body.dark { background: #111; color: #eee; }
  </style>
</head>
<body>
  <label><input type="checkbox" id="dark"> Dark mode</label>
  <script>
    const box = document.getElementById("dark");

    function apply(theme) {
      box.checked = theme === "dark";
      document.body.classList.toggle("dark", theme === "dark");
    }

    box.addEventListener("change", () => {
      const theme = box.checked ? "dark" : "light";
      apply(theme);
      // TODO: save it
    });

    // TODO: load the saved theme (default "light") and apply it
  </script>
</body>
</html>
@@requires
localStorage.setItem("theme"
localStorage.getItem("theme")
catch
@@hint
In the listener: `try { localStorage.setItem("theme", theme); } catch {}`. At the end: `let saved = "light"; try { saved = localStorage.getItem("theme") ?? "light"; } catch {} apply(saved);`.
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Settings</title>
  <style>
    body.dark { background: #111; color: #eee; }
  </style>
</head>
<body>
  <label><input type="checkbox" id="dark"> Dark mode</label>
  <script>
    const box = document.getElementById("dark");

    function apply(theme) {
      box.checked = theme === "dark";
      document.body.classList.toggle("dark", theme === "dark");
    }

    box.addEventListener("change", () => {
      const theme = box.checked ? "dark" : "light";
      apply(theme);
      try { localStorage.setItem("theme", theme); } catch { /* storage blocked: still works for this visit */ }
    });

    let saved = "light";
    try { saved = localStorage.getItem("theme") ?? "light"; } catch { /* blocked */ }
    apply(saved);
  </script>
</body>
</html>

@@exercise A tiny router
Make the three links switch the view without reloading: listen for `hashchange`, read `location.hash`, and show the matching section (`#/` or no hash → `home`, `#/devices` → `devices`, `#/about` → `about`) by setting `hidden` on the others. Anything else shows `notfound`. Run it once on load too.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Router</title></head>
<body>
  <nav><a href="#/">Home</a> · <a href="#/devices">Devices</a> · <a href="#/about">About</a> · <a href="#/nope">Broken link</a></nav>
  <section id="home"><h1>Home</h1></section>
  <section id="devices" hidden><h1>Devices</h1></section>
  <section id="about" hidden><h1>About</h1></section>
  <section id="notfound" hidden><h1>Page not found</h1></section>
  <script>
    const views = { "#/": "home", "#/devices": "devices", "#/about": "about" };

    function route() {
      // TODO
    }
  </script>
</body>
</html>
@@requires
hashchange
location.hash
route();
@@hint
`const id = views[location.hash || "#/"] ?? "notfound"; for (const s of document.querySelectorAll("section")) s.hidden = s.id !== id;`. Then `window.addEventListener("hashchange", route); route();`.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Router</title></head>
<body>
  <nav><a href="#/">Home</a> · <a href="#/devices">Devices</a> · <a href="#/about">About</a> · <a href="#/nope">Broken link</a></nav>
  <section id="home"><h1>Home</h1></section>
  <section id="devices" hidden><h1>Devices</h1></section>
  <section id="about" hidden><h1>About</h1></section>
  <section id="notfound" hidden><h1>Page not found</h1></section>
  <script>
    const views = { "#/": "home", "#/devices": "devices", "#/about": "about" };

    function route() {
      const id = views[location.hash || "#/"] ?? "notfound";
      for (const s of document.querySelectorAll("section")) s.hidden = s.id !== id;
    }

    window.addEventListener("hashchange", route);
    route();
  </script>
</body>
</html>

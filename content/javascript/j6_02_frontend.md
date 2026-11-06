@@lesson j6-frontend Front End: State, Rendering and Talking to the API
@@text
A front end is a program running in the user's browser. Small pages can get away with "change this element when that button is clicked". Real apps can't: with 20 buttons changing 30 elements, the page ends up showing things that disagree with each other. The fix that every modern framework uses is one idea:

> **The page is a function of the state.** Keep all data in one place. When it changes, redraw from it.

```text
   event (click, typing, data arrived)
        │
        ▼
   update the state  ──►  render(state)  ──►  the page
        ▲                                        │
        └──────────── user sees it, acts ◄───────┘
```

## State and render, without a framework

```javascript
// The state: the only source of truth.
const state = { tickets: [{ id: 1, title: "VPN down", done: false }], filter: "open" };

// render() reads the state and returns what the page should show. (In a browser it would
// build DOM elements; here it returns text so you can run it.)
function render(s) {
  const visible = s.tickets.filter((t) => (s.filter === "open" ? !t.done : true));
  return [`Filter: ${s.filter}`, ...visible.map((t) => `${t.done ? "✓" : "○"} #${t.id} ${t.title}`)].join("\n");
}

// Events never touch the page: they change the state, then render again.
function dispatch(action) {
  if (action.type === "add") state.tickets.push({ id: state.tickets.length + 1, title: action.title, done: false });
  if (action.type === "toggle") {
    const t = state.tickets.find((t) => t.id === action.id);
    t.done = !t.done;
  }
  if (action.type === "filter") state.filter = action.value;
  console.log(render(state) + "\n");
}

dispatch({ type: "add", title: "Printer jam" });
dispatch({ type: "toggle", id: 1 });
dispatch({ type: "filter", value: "all" });
```

This pattern (actions → one place that changes state → render) is exactly **Redux**, React's `useReducer`, and Vue's Pinia stores.

## Building the DOM safely

In the browser, `render` builds elements. Use `textContent` for anything a user typed, never `innerHTML`:

```javascript,norun
function el(tag, props = {}, ...children) {
  const node = Object.assign(document.createElement(tag), props);
  node.append(...children);           // strings become text nodes: HTML in them is NOT run
  return node;
}

function render() {
  document.getElementById("list").replaceChildren(
    ...state.tickets.map((t) => el("li", { className: t.done ? "done" : "" }, `#${t.id} ${t.title}`)),
  );
}
```

A ticket titled `<img src=x onerror=stealCookies()>` shows up as harmless text. With `innerHTML` it would run: that's **XSS**, the most common web vulnerability.

## Event delegation

Instead of adding a listener to every button (including ones `render()` creates later), add **one** listener to the container and look at what was clicked:

```javascript,norun
document.getElementById("list").addEventListener("click", (e) => {
  const button = e.target.closest("button[data-id]");
  if (button) dispatch({ type: "toggle", id: Number(button.dataset.id) });
});
```

## Talking to the API

`fetch` returns a promise. A real app handles three states for every request: **loading**, **success** and **error**, and shows each one.

```javascript
// A pretend fetch so this runs here: the real one is built into browsers and Node.
const fakeFetch = async (url) => {
  await new Promise((r) => setTimeout(r, 50));
  if (url.includes("broken")) return { ok: false, status: 500, json: async () => ({ error: "database down" }) };
  return { ok: true, status: 200, json: async () => [{ id: 1, title: "VPN down" }] };
};

async function loadTickets(url, state) {
  state.loading = true;
  state.error = null;
  try {
    const res = await fakeFetch(url);
    const data = await res.json();
    if (!res.ok) throw new Error(data.error ?? `HTTP ${res.status}`);  // fetch doesn't throw on 4xx/5xx!
    state.tickets = data;
  } catch (err) {
    state.error = err.message;
  } finally {
    state.loading = false;
  }
  return state;
}

(async () => {
  console.log(await loadTickets("/api/tickets", {}));
  console.log(await loadTickets("/api/broken", {}));
})();
```

`fetch` only rejects on network failure. A 404 or 500 is a *successful* fetch with `res.ok === false`, so always check it.

## Debouncing and races

Searching on every keystroke sends a request per letter. **Debounce**: wait until typing pauses.

```javascript
function debounce(fn, ms) {
  let timer;
  return (...args) => {
    clearTimeout(timer);
    timer = setTimeout(() => fn(...args), ms);
  };
}

const search = debounce((q) => console.log("searching for", q), 100);
for (const q of ["k", "ku", "kub", "kube"]) search(q);   // only "kube" is searched
```

A related bug: the response to an *old* search arriving after a newer one, so the page shows stale results. Fix it with an `AbortController` (cancel the previous request) or by ignoring responses whose query isn't the current one.

## Keeping state across reloads

| Where | Use for |
|---|---|
| URL (`?q=vpn&tab=open`) | anything the user might bookmark or share |
| `localStorage` | small preferences: theme, last filter (strings only, about 5 MB, not secret) |
| IndexedDB | lots of data for offline apps |
| the server | everything that matters |

Never put tokens or personal data in `localStorage`: any script on the page (including an XSS attack) can read it.

## Frameworks: the same loop, done for you

| | React | Vue | Svelte |
|---|---|---|---|
| state | `useState` / `useReducer` | `ref` / `reactive` | plain `let` variables |
| render | JSX returned by a function | HTML templates | HTML templates, compiled |
| updates | re-render, then diff | tracks what each part uses | compiled to direct DOM updates |

```javascript,norun
// React: the ticket list from above. {t.title} is escaped automatically.
function TicketList({ tickets, onToggle }) {
  return (
    <ul>
      {tickets.map((t) => (
        <li key={t.id} onClick={() => onToggle(t.id)}>{t.title}</li>
      ))}
    </ul>
  );
}
```

Learn one well (React has the most jobs in Japan and worldwide; Vue is very popular in Japanese companies), and the others take days, because the model is the same.

@@quiz
In the "page is a function of the state" model, what should a click handler do?
- Change the text of the elements directly
* Update the state, then let render() redraw the page from it
- Reload the whole page
- Call the API and put the response straight into innerHTML
? One source of truth means the page can't show two different versions of the data.

@@quiz
`fetch("/api/tickets")` gets a 500 response. What happens?
- The promise rejects with an error
* The promise resolves; `res.ok` is false and `res.status` is 500
- fetch retries automatically
- The browser shows an error page
? fetch rejects only for network failures. Check `res.ok` yourself.

@@quiz
Why use `textContent` (or `append` with strings) instead of `innerHTML` for user data?
- It's the only way to add text
* HTML in the data is shown as text instead of being run, which prevents XSS
- It's required by CSS
- innerHTML is deprecated
? `innerHTML` parses HTML, including event handlers in attacker-controlled data.

@@quiz
What does debouncing a search box do?
* Waits until the user pauses typing before searching
- Searches on every key press, faster
- Caches every search forever
- Cancels the search if the user types too fast
? Fewer requests, less flicker, and less load on the server.

@@drill
What's the name of the technique of handling child events with one listener on a parent?
= event delegation
= delegation
? Check event.target.closest(...) inside the handler.

@@flashcards
state → render :: the page is drawn from one source of truth
dispatch(action) :: one place where state changes (Redux, useReducer)
`textContent` / `append("text")` :: insert text safely (no HTML parsing)
event delegation :: one listener on a container; check `e.target.closest(...)`
`res.ok` :: false for HTTP 4xx/5xx; fetch doesn't throw for them
loading / success / error :: the three states of every request in a UI
debounce :: run only after calls stop for N ms
`AbortController` :: cancel an in-flight fetch
`localStorage` :: small, string-only, per-site storage; not for secrets
React / Vue / Svelte :: component frameworks built on the state → render idea

@@exercise A reducer
Write `reducer(state, action)` for a shopping cart. It must **not change** the old state: return a new object. Actions:

- `{ type: "add", item, price }`: add 1 of the item (keep its price),
- `{ type: "remove", item }`: remove 1; when the quantity reaches 0, remove the line,
- `{ type: "clear" }`: empty cart.

State looks like `{ lines: [{ item, price, qty }], total }`, where `total` is the sum of price × qty. Unknown actions return the same state.
@@starter
const empty = { lines: [], total: 0 };

function reducer(state, action) {
  // TODO
}

let s = reducer(empty, { type: "add", item: "USB-C cable", price: 1200 });
s = reducer(s, { type: "add", item: "USB-C cable", price: 1200 });
console.log(s);
@@tests
let _s = reducer(empty, { type: "add", item: "cable", price: 1200 });
_s = reducer(_s, { type: "add", item: "mouse", price: 3000 });
const _before = _s;
_s = reducer(_s, { type: "add", item: "cable", price: 1200 });
check(_s, { lines: [{ item: "cable", price: 1200, qty: 2 }, { item: "mouse", price: 3000, qty: 1 }], total: 5400 }, "add twice");
check(_before.lines[0].qty, 1, "the old state was not changed");
_s = reducer(_s, { type: "remove", item: "mouse" });
check(_s, { lines: [{ item: "cable", price: 1200, qty: 2 }], total: 2400 }, "remove the last mouse");
check(reducer(_s, { type: "clear" }), { lines: [], total: 0 }, "clear");
check(reducer(_s, { type: "dance" }) === _s, true, "unknown action: same object");
check(empty, { lines: [], total: 0 }, "the empty cart was never changed");
@@hint
Build the new `lines` with `map`/`filter` and spread: for add, if a line exists, `lines.map((l) => l.item === item ? { ...l, qty: l.qty + 1 } : l)`, otherwise `[...lines, { item, price, qty: 1 }]`. For remove, map to `qty - 1` then `filter((l) => l.qty > 0)`. Then return `{ lines, total: lines.reduce((sum, l) => sum + l.price * l.qty, 0) }`.
@@solution
const empty = { lines: [], total: 0 };

function reducer(state, action) {
  let lines;
  if (action.type === "add") {
    const exists = state.lines.some((l) => l.item === action.item);
    lines = exists
      ? state.lines.map((l) => (l.item === action.item ? { ...l, qty: l.qty + 1 } : l))
      : [...state.lines, { item: action.item, price: action.price, qty: 1 }];
  } else if (action.type === "remove") {
    lines = state.lines.map((l) => (l.item === action.item ? { ...l, qty: l.qty - 1 } : l)).filter((l) => l.qty > 0);
  } else if (action.type === "clear") {
    lines = [];
  } else {
    return state;
  }
  return { lines, total: lines.reduce((sum, l) => sum + l.price * l.qty, 0) };
}

let s = reducer(empty, { type: "add", item: "USB-C cable", price: 1200 });
s = reducer(s, { type: "add", item: "USB-C cable", price: 1200 });
console.log(s);

@@exercise Ignore stale responses
Searches can finish in the wrong order. Write `createSearcher(fetchResults)`: it returns `search(query)`, an async function that calls `fetchResults(query)` and returns its results, **unless a newer search was started in the meantime**, in which case it returns `null` (the old answer must not overwrite the new one).
@@starter
function createSearcher(fetchResults) {
  // TODO
}

const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
const slowFirst = async (q) => { await sleep(q === "ku" ? 100 : 10); return [`results for ${q}`]; };
const search = createSearcher(slowFirst);
search("ku").then((r) => console.log("ku:", r));
search("kube").then((r) => console.log("kube:", r));
@@tests
const _sleep = (ms) => new Promise((r) => setTimeout(r, ms));
const _api = async (q) => { await _sleep(q.length === 1 ? 80 : 10); return [q]; };
(async () => {
  const s = createSearcher(_api);
  const [a, b] = await Promise.all([s("k"), s("kube")]);
  check(a, null, "the older, slower search is ignored");
  check(b, ["kube"], "the newest search wins");
  check(await s("vpn"), ["vpn"], "a search on its own works");
})();
@@hint
Keep a counter in the closure: `let latest = 0;`. In `search`, take `const id = ++latest;`, await the results, then `return id === latest ? results : null;`.
@@solution
function createSearcher(fetchResults) {
  let latest = 0;
  return async (query) => {
    const id = ++latest;
    const results = await fetchResults(query);
    return id === latest ? results : null;
  };
}

const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
const slowFirst = async (q) => { await sleep(q === "ku" ? 100 : 10); return [`results for ${q}`]; };
const search = createSearcher(slowFirst);
search("ku").then((r) => console.log("ku:", r));
search("kube").then((r) => console.log("kube:", r));

@@lesson u9-routing Routing and Data Fetching in Single-Page Apps
@@concepts networking, text-processing
@@text
A **single-page app** (SPA) loads once and then switches "pages" in JavaScript, fetching data as JSON. Two problems every SPA must solve: **routing** (which screen matches the URL) and **server state** (fetching, caching and refreshing data).

## Client-side routing

The URL is part of your UI state: users bookmark it, share it, and press Back. A router maps URL patterns to screens:

```jsx,norun
import { BrowserRouter, Routes, Route, Link, useParams, useNavigate } from "react-router";

<BrowserRouter>
  <Routes>
    <Route path="/" element={<Dashboard />} />
    <Route path="/tickets" element={<TicketList />} />
    <Route path="/tickets/:id" element={<TicketDetails />} />
    <Route path="*" element={<NotFound />} />
  </Routes>
</BrowserRouter>

function TicketDetails() {
  const { id } = useParams();                 // "/tickets/42" → { id: "42" }
  const navigate = useNavigate();
  return <button onClick={() => navigate("/tickets")}>Back to list</button>;
}
```

Under the hood: the **History API** (`history.pushState(state, "", "/tickets/42")` and the `popstate` event) changes the URL without a page load, and the router re-renders the matching component. `<Link>` intercepts clicks on `<a>` elements. Put filters and sorting in the **query string** (`/tickets?status=open&sort=-created`) so they survive reloads and can be shared.

The server must return `index.html` for every app route (a "fallback"), otherwise reloading `/tickets/42` gives a 404. Desktop apps (Electron/Tauri) often use **hash routing** (`#/tickets/42`) to avoid this.

## Server state is different

Data from an API isn't really *your* state: it lives on the server, can change at any time, and is shared with other users. Treat it as a **cache**:

- deduplicate identical requests from several components
- show cached data instantly, then **revalidate** in the background ("stale-while-revalidate")
- refetch when the window regains focus or the network reconnects
- **invalidate** related queries after a change (after creating a ticket, refetch the list)
- handle loading and error states, retries, pagination

**TanStack Query** (React Query) and **SWR** do exactly this:

```jsx,norun
const { data, isPending, error } = useQuery({
  queryKey: ["tickets", { status }],
  queryFn: () => fetch(`/api/tickets?status=${status}`).then((r) => r.json()),
  staleTime: 30_000,
});
const create = useMutation({
  mutationFn: (t) => fetch("/api/tickets", { method: "POST", body: JSON.stringify(t) }),
  onSuccess: () => queryClient.invalidateQueries({ queryKey: ["tickets"] }),
});
```

## Optimistic updates

For snappy UIs, update the screen **before** the server confirms, and roll back if it fails:

```text
click "close ticket" → mark closed in the cache immediately → send PATCH
                                     │
            success: keep it ◀───────┴───────▶ failure: restore the old value, show an error
```

## Loading UX

- Skeleton placeholders instead of spinners for content areas.
- Keep showing old data while new data loads (no flashing).
- Show errors with a **Retry** button, not a blank page.
- Debounce search inputs (see the functional patterns lesson).

The exercises build a route matcher and a small stale-while-revalidate cache.

@@quiz
Why put list filters in the URL query string?
* So the view survives reloads, can be bookmarked and shared, and Back works
- URLs are faster than state
- Routers require it
- To hide them from users
? The URL is part of UI state.

@@quiz
Why does reloading `/tickets/42` in an SPA sometimes give a 404?
- The ticket was deleted
* The web server has no file at that path; it must fall back to serving index.html for app routes
- The router is broken
- Browsers can't reload SPAs
? Configure a fallback or use hash routing.

@@quiz
What does stale-while-revalidate mean?
- Never cache data
* Show cached data immediately, then fetch fresh data in the background and update
- Retry failed requests forever
- Cache data forever
? Users see something instantly and get fresh data shortly after.

@@flashcards
SPA :: single-page app; switches screens without full page loads
History API :: pushState and popstate for URL changes
Route parameter :: `:id` part of a path pattern
Query string :: `?status=open` part of a URL
Hash routing :: routes after `#`; no server fallback needed
Server state :: remote data you cache, not own
Stale-while-revalidate :: serve cache, refresh in background
Invalidate :: mark cached queries as needing a refetch
Optimistic update :: update UI before the server confirms, roll back on error

@@exercise A route matcher
Write `createRouter(routes)` where `routes` is a list of `[pattern, name]`, returning `match(url)`:

- patterns are like `/tickets/:id`, `/users/:userId/devices/:deviceId`, `/files/*` (the `*` captures the rest, possibly with slashes, as `params["*"]`) and `/`
- static segments match exactly (case-sensitive); `:param` matches one non-empty segment, decoded with `decodeURIComponent`
- trailing slashes are ignored (`/tickets/` = `/tickets`)
- the query string becomes `query` (use `URLSearchParams`; repeated keys become arrays)
- the **most specific** route wins: static segments beat params beat `*`, compared segment by segment from the left, regardless of the order routes were given
- returns `{ name, params, query }` or `null`
@@starter
function createRouter(routes) {
  throw new Error("TODO: not done yet");
}

@@tests
const match = createRouter([
  ["/files/*", "files"],
  ["/tickets/:id", "ticket"],
  ["/tickets/new", "newTicket"],
  ["/", "home"],
  ["/users/:userId/devices/:deviceId", "device"],
]);
check(match("/tickets/42?tab=history&tag=a&tag=b"), { name: "ticket", params: { id: "42" }, query: { tab: "history", tag: ["a", "b"] } }, "params and query");
check(match("/tickets/new/").name, "newTicket", "static beats param, trailing slash");
check(match("/users/ana%20k/devices/LT-9").params, { userId: "ana k", deviceId: "LT-9" }, "decoded params");
check(match("/files/reports/2026/q3.pdf").params, { "*": "reports/2026/q3.pdf" }, "wildcard");
check([match("/").name, match("/tickets"), match("/Tickets/1")], ["home", null, null], "no match");
@@hint
Split paths with `p.split("/").filter(Boolean)`. Give each segment a rank (static 3, param 2, wildcard 1) and sort routes by their rank arrays (compare element by element, higher first; longer routes first on ties). Try routes in that order and return the first match.
@@solution
function createRouter(routes) {
  const compiled = routes.map(([pattern, name]) => {
    const segments = pattern.split("/").filter(Boolean);
    const ranks = segments.map((s) => (s === "*" ? 1 : s.startsWith(":") ? 2 : 3));
    return { name, segments, ranks };
  });
  compiled.sort((a, b) => {
    for (let i = 0; i < Math.max(a.ranks.length, b.ranks.length); i++) {
      const diff = (b.ranks[i] ?? 0) - (a.ranks[i] ?? 0);
      if (diff !== 0) return diff;
    }
    return 0;
  });

  function parseQuery(search) {
    const query = {};
    for (const [k, v] of new URLSearchParams(search)) {
      if (k in query) query[k] = [].concat(query[k], v);
      else query[k] = v;
    }
    return query;
  }

  return function match(url) {
    const [pathPart, search = ""] = url.split("?");
    const parts = pathPart.split("/").filter(Boolean);
    for (const route of compiled) {
      const params = {};
      let ok = true;
      for (let i = 0; i < route.segments.length; i++) {
        const seg = route.segments[i];
        if (seg === "*") {
          if (i >= parts.length) { ok = false; break; }
          params["*"] = parts.slice(i).map(decodeURIComponent).join("/");
          break;
        }
        if (i >= parts.length) { ok = false; break; }
        if (seg.startsWith(":")) params[seg.slice(1)] = decodeURIComponent(parts[i]);
        else if (seg !== parts[i]) { ok = false; break; }
      }
      const hasWildcard = route.segments.at(-1) === "*";
      if (ok && (hasWildcard || route.segments.length === parts.length)) {
        return { name: route.name, params, query: parseQuery(search) };
      }
    }
    return null;
  };
}

@@exercise A stale-while-revalidate cache
Write `createQueryClient({ now, staleTime })` with:

- `async fetchQuery(key, fn)`: `key` is an array (cache by `JSON.stringify(key)`). If there's no cached data, call `fn()` and cache the result with its time. If cached data is **fresh** (younger than `staleTime` ms), return it without calling `fn`. If it's **stale**, return the cached data **immediately** and start a background refetch that updates the cache.
- concurrent calls for the same key share one in-flight request
- `invalidate(prefix)`: mark every query whose key **starts with** the `prefix` array as stale (so the next `fetchQuery` revalidates)
- `getData(key)`: cached data or `undefined`
- `setData(key, updater)`: for optimistic updates: replace the cached data with `updater(old)` and return the old value (for rollback)
@@starter
function createQueryClient({ now = () => Date.now(), staleTime = 0 } = {}) {
  throw new Error("TODO: not done yet");
}

@@tests
(async () => {
  let t = 0;
  let calls = 0;
  const client = createQueryClient({ now: () => t, staleTime: 1000 });
  const fetchTickets = async () => { calls++; await new Promise((r) => setTimeout(r, 5)); return [`v${calls}`]; };
  const [a, b] = await Promise.all([client.fetchQuery(["tickets", "open"], fetchTickets), client.fetchQuery(["tickets", "open"], fetchTickets)]);
  check([a, b, calls], [["v1"], ["v1"], 1], "deduplicated");
  t = 500;
  check([await client.fetchQuery(["tickets", "open"], fetchTickets), calls], [["v1"], 1], "fresh: no fetch");
  t = 2000;
  check(await client.fetchQuery(["tickets", "open"], fetchTickets), ["v1"], "stale: old data returned at once");
  await new Promise((r) => setTimeout(r, 20));
  check([client.getData(["tickets", "open"]), calls], [["v2"], 2], "revalidated in background");
  client.invalidate(["tickets"]);
  await client.fetchQuery(["tickets", "open"], fetchTickets);
  await new Promise((r) => setTimeout(r, 20));
  check(calls, 3, "invalidated by prefix");
  const old = client.setData(["tickets", "open"], (list) => [...list, "optimistic"]);
  check([old, client.getData(["tickets", "open"])], [["v3"], ["v3", "optimistic"]], "optimistic update");
})();
@@hint
Store entries as `{ data, updatedAt, stale, promise }` in a `Map`. A helper `load(hash, key, fn)` returns the in-flight promise if there is one, otherwise starts `fn()`, and on success stores data and `updatedAt = now()`, clearing `stale` and `promise`. Freshness: `!entry.stale && now() - entry.updatedAt < staleTime`.
@@solution
function createQueryClient({ now = () => Date.now(), staleTime = 0 } = {}) {
  const cache = new Map();

  function load(hash, key, fn) {
    const entry = cache.get(hash) ?? { key };
    cache.set(hash, entry);
    if (entry.promise) return entry.promise;
    entry.promise = fn().then(
      (data) => {
        Object.assign(entry, { data, updatedAt: now(), stale: false, promise: null, hasData: true });
        return data;
      },
      (error) => {
        entry.promise = null;
        throw error;
      },
    );
    return entry.promise;
  }

  async function fetchQuery(key, fn) {
    const hash = JSON.stringify(key);
    const entry = cache.get(hash);
    if (!entry || !entry.hasData) return load(hash, key, fn);
    const fresh = !entry.stale && now() - entry.updatedAt < staleTime;
    if (!fresh) load(hash, key, fn).catch(() => {});
    return entry.data;
  }

  function invalidate(prefix) {
    for (const entry of cache.values()) {
      if (prefix.every((part, i) => JSON.stringify(entry.key[i]) === JSON.stringify(part))) entry.stale = true;
    }
  }

  function getData(key) {
    return cache.get(JSON.stringify(key))?.data;
  }

  function setData(key, updater) {
    const hash = JSON.stringify(key);
    const entry = cache.get(hash) ?? { key, hasData: true, updatedAt: now() };
    cache.set(hash, entry);
    const old = entry.data;
    entry.data = updater(old);
    entry.hasData = true;
    return old;
  }

  return { fetchQuery, invalidate, getData, setData };
}

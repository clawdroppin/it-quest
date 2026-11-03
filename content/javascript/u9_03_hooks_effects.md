@@lesson u9-hooks Hooks and Effects: How React Remembers
@@concepts functions, concurrency
@@text
A function component is just a function, called again on every render. So how does `useState` remember values between calls, and how do you run side effects (fetching, timers, subscriptions) safely? This lesson explains hooks by building them.

## How hooks remember

React keeps a list of "hook slots" for each component instance. During a render, each hook call takes the **next slot in order**: the first `useState` gets slot 0, the second slot 1, and so on. That's why the **Rules of Hooks** exist:

1. Call hooks only at the **top level** of a component (not inside `if`, loops or nested functions), so the order is the same on every render.
2. Call hooks only from components or other hooks.

```javascript
function createRuntime(component) {
  const slots = [];
  let index = 0;
  function useState(initial) {
    const i = index++;
    if (!(i in slots)) slots[i] = initial;
    const setState = (value) => { slots[i] = typeof value === "function" ? value(slots[i]) : value; };
    return [slots[i], setState];
  }
  return { render() { index = 0; return component({ useState }); } };
}
const app = createRuntime(({ useState }) => {
  const [count, setCount] = useState(0);
  const [name] = useState("ana");
  return { count, name, inc: () => setCount((c) => c + 1) };
});
let out = app.render(); out.inc(); out.inc();
console.log(app.render());
```

## useEffect

Rendering must be **pure**: no fetching, no timers, no DOM changes during render. Side effects go in `useEffect`, which runs **after** the render is shown:

```jsx,norun
function Uptime({ host }) {
  const [status, setStatus] = useState("checking");

  useEffect(() => {
    const controller = new AbortController();
    const id = setInterval(async () => {
      try {
        const res = await fetch(`/api/ping?host=${encodeURIComponent(host)}`, { signal: controller.signal });
        setStatus((await res.json()).up ? "up" : "down");
      } catch (e) {
        if (e.name !== "AbortError") setStatus("error");
      }
    }, 5000);
    return () => {                        // cleanup: runs before the next effect and on unmount
      clearInterval(id);
      controller.abort();
    };
  }, [host]);                             // dependencies: re-run only when host changes

  return <span className={status}>{host}: {status}</span>;
}
```

- The **dependency array** says when to re-run: `[host]` after renders where `host` changed; `[]` once after the first render; omitted, after every render.
- The **cleanup** function undoes the effect (clear timers, abort fetches, unsubscribe). Without it you get leaks and "state update on an unmounted component" bugs, or **race conditions** where a slow old response overwrites a newer one.
- In development, React's Strict Mode runs effects twice on mount to surface missing cleanups.
- Many things people put in effects don't belong there: derive values during render instead of syncing them with an effect, and handle user actions in event handlers.

## Other built-in hooks

| Hook | Purpose |
|---|---|
| `useRef` | a mutable box that survives renders without causing them (DOM nodes, timer IDs) |
| `useMemo` / `useCallback` | cache a value / function until dependencies change |
| `useContext` / `use` | read context (and, with `use`, promises) |
| `useId` | stable unique IDs for accessibility attributes |
| `useTransition` / `useDeferredValue` | keep typing responsive while heavy updates render |
| `useSyncExternalStore` | subscribe to an external store (like the one you built) |

## Custom hooks

A **custom hook** is a function starting with `use` that calls other hooks. It packages stateful logic for reuse:

```jsx,norun
function useDebouncedValue(value, ms) {
  const [debounced, setDebounced] = useState(value);
  useEffect(() => {
    const t = setTimeout(() => setDebounced(value), ms);
    return () => clearTimeout(t);
  }, [value, ms]);
  return debounced;
}
const query = useDebouncedValue(searchText, 300);
```

@@quiz
Why can't you call `useState` inside an `if`?
* Hooks are matched to their slots by call order, which must be the same on every render
- if statements are not allowed in components
- useState is async
- It would be slower
? Conditional hooks shift every following slot.

@@quiz
What does the dependency array `[host]` mean in useEffect?
- Run the effect once
* Re-run the effect (after cleanup) only when `host` changes between renders
- Run the effect before render
- Store host in state
? `[]` means only after the first render.

@@quiz
What should an effect that starts an interval return?
- Nothing
- The interval ID
* A cleanup function that clears the interval
- A promise
? Cleanup runs before the next effect and on unmount.

@@flashcards
Rules of Hooks :: call hooks at the top level, in the same order every render
`useEffect` :: run side effects after render
Dependency array :: values that trigger re-running an effect
Cleanup function :: undoes an effect; returned from useEffect
`useRef` :: mutable value that persists without re-rendering
`useMemo` :: cache a computed value
`useCallback` :: cache a function
Custom hook :: reusable function starting with `use` that calls hooks
Race condition (fetch) :: an old response overwriting newer state

@@exercise A mini hooks runtime
Write `createComponent(renderFn)` returning `{ render(props), unmount(), effectLog }` where `renderFn(props, hooks)` receives `hooks = { useState, useEffect }`:

- `useState(initial)`: returns `[value, setState]`; slots matched by call order; `setState(v)` or `setState((old) => v)`; setting a value marks the component **dirty** but doesn't re-render by itself
- `useEffect(effect, deps)`: after `render` finishes, run each effect whose deps changed (compare item by item with `Object.is`; `undefined` deps = every render; first render always runs). Before re-running an effect, call its previous cleanup (if it returned a function). `unmount()` runs all remaining cleanups.
- `render(props)` returns what `renderFn` returns
- the component exposes `isDirty` (true after a setState until the next render)
@@starter
function createComponent(renderFn) {
  throw new Error("TODO: not done yet");
}

@@tests
const log = [];
const c = createComponent((props, { useState, useEffect }) => {
  const [count, setCount] = useState(0);
  useEffect(() => { log.push(`subscribe ${props.host}`); return () => log.push(`unsubscribe ${props.host}`); }, [props.host]);
  useEffect(() => { log.push(`title ${count}`); });
  return { text: `${props.host}: ${count}`, inc: () => setCount((n) => n + 1) };
});
let ui = c.render({ host: "web01" });
check(ui.text, "web01: 0", "first render");
ui.inc(); ui.inc();
check(c.isDirty, true, "dirty after setState");
ui = c.render({ host: "web01" });
check([ui.text, c.isDirty], ["web01: 2", false], "state kept");
c.render({ host: "db01" });
c.unmount();
check(log, ["subscribe web01", "title 0", "title 2", "unsubscribe web01", "subscribe db01", "title 2", "unsubscribe db01"], "effects and cleanups");
@@hint
Keep `slots = []`, `effects = []` (each `{ deps, cleanup }`), `index`, `pending = []`. In `useEffect`, compare with the stored deps; if changed, push `{ i, effect, deps }` to `pending`. After calling `renderFn`, loop over `pending`: call the old cleanup if it's a function, run the effect, store the new cleanup and deps.
@@solution
function createComponent(renderFn) {
  const slots = [];
  const effects = [];
  let index = 0;
  let effectIndex = 0;
  let pending = [];
  const component = { isDirty: false };

  function useState(initial) {
    const i = index++;
    if (!(i in slots)) slots[i] = initial;
    const setState = (value) => {
      slots[i] = typeof value === "function" ? value(slots[i]) : value;
      component.isDirty = true;
    };
    return [slots[i], setState];
  }

  function useEffect(effect, deps) {
    const i = effectIndex++;
    const previous = effects[i];
    const changed = !previous || deps === undefined || previous.deps === undefined
      || deps.length !== previous.deps.length || deps.some((d, k) => !Object.is(d, previous.deps[k]));
    if (changed) pending.push({ i, effect, deps });
  }

  component.render = (props) => {
    index = 0;
    effectIndex = 0;
    pending = [];
    const result = renderFn(props, { useState, useEffect });
    component.isDirty = false;
    for (const { i, effect, deps } of pending) {
      const old = effects[i];
      if (old && typeof old.cleanup === "function") old.cleanup();
      const cleanup = effect();
      effects[i] = { deps, cleanup };
    }
    return result;
  };

  component.unmount = () => {
    for (const e of effects) if (e && typeof e.cleanup === "function") e.cleanup();
    effects.length = 0;
  };

  return component;
}

@@exercise Fix the fetch race
A search box calls `search(query)` on every keystroke. Responses can arrive **out of order**, so a slow response for `"we"` can overwrite the result for `"web01"`. Write `createSearch(fetchResults, onResults)` returning `{ setQuery(q) }` that:

- calls `fetchResults(q, signal)` for each new query, passing an `AbortSignal`
- **aborts** the previous request when a new query starts
- only calls `onResults(q, results)` for the **latest** query (ignore stale responses even if the fetch function ignores the signal)
- treats an empty query as "clear": aborts and calls `onResults("", [])` immediately without fetching
@@starter
function createSearch(fetchResults, onResults) {
  throw new Error("TODO: not done yet");
}

@@tests
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
(async () => {
  const shown = [];
  const aborted = [];
  const fetchResults = async (q, signal) => {
    signal.addEventListener("abort", () => aborted.push(q));
    await sleep(q === "we" ? 40 : 10);
    return [`${q}-result`];
  };
  const s = createSearch(fetchResults, (q, r) => shown.push([q, r]));
  s.setQuery("we");
  await sleep(1);
  s.setQuery("web01");
  await sleep(60);
  check(shown, [["web01", ["web01-result"]]], "stale response ignored");
  check(aborted, ["we"], "old request aborted");
  s.setQuery("db");
  s.setQuery("");
  await sleep(30);
  check(shown.at(-1), ["", []], "cleared");
  check(shown.length, 2, "aborted db result not shown");
})();
@@hint
Keep `let controller = null; let latest = 0;`. In `setQuery`, `controller?.abort()`, then `const id = ++latest;`. For an empty query, call `onResults("", [])` and return. Otherwise create a new `AbortController`, `await fetchResults(q, controller.signal)` inside try/catch, and only call `onResults` if `id === latest`.
@@solution
function createSearch(fetchResults, onResults) {
  let controller = null;
  let latest = 0;
  async function setQuery(q) {
    controller?.abort();
    controller = null;
    const id = ++latest;
    if (q === "") {
      onResults("", []);
      return;
    }
    const current = new AbortController();
    controller = current;
    try {
      const results = await fetchResults(q, current.signal);
      if (id === latest) onResults(q, results);
    } catch (error) {
      if (error.name !== "AbortError") throw error;
    }
  }
  return { setQuery };
}

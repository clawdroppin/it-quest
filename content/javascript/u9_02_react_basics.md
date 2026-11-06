@@lesson u9-react React Fundamentals: JSX, Props, State and Reducers
@@concepts objects, functions
@@text
**React** is the most widely used UI library, for web apps, desktop apps (Electron, Tauri) and mobile (React Native). With the ideas from the previous lesson, React is mostly vocabulary.

## Setting up

```bash,norun
npm create vite@latest helpdesk -- --template react      # or react-ts for TypeScript
cd helpdesk && npm install && npm run dev
```

**Vite** gives you a dev server with instant reload and a production build. For full sites with routing and server rendering, frameworks like **Next.js** or **React Router** (framework mode) build on React.

## Components and JSX

```jsx,norun
function TicketRow({ ticket, selected, onSelect }) {
  return (
    <li
      className={selected ? "row selected" : "row"}
      onClick={() => onSelect(ticket.id)}
    >
      <span className={`priority p${ticket.priority}`}>{ticket.priority}</span>
      {ticket.title}
      {ticket.overdue && <strong> (overdue)</strong>}
    </li>
  );
}

export function TicketList({ tickets, selectedId, onSelect }) {
  if (tickets.length === 0) return <p>No tickets 🎉</p>;
  return (
    <ul>
      {tickets.map((t) => (
        <TicketRow key={t.id} ticket={t} selected={t.id === selectedId} onSelect={onSelect} />
      ))}
    </ul>
  );
}
```

JSX rules: one root element (or a fragment `<>...</>`), `className` instead of `class`, `{expression}` for JavaScript, camelCase event props (`onClick`), and components start with a capital letter.

## State with useState

```jsx,norun
import { useState } from "react";

function Counter() {
  const [count, setCount] = useState(0);
  return <button onClick={() => setCount((c) => c + 1)}>Clicked {count} times</button>;
}
```

- Calling the setter schedules a **re-render**; the component function runs again with the new value.
- State is a **snapshot**: inside one render, `count` never changes. Use the updater form `setCount((c) => c + 1)` when the new value depends on the old.
- **Never mutate** state (`tickets.push(t)`); create new arrays/objects. React compares by identity (`Object.is`) to decide whether anything changed.

## Reducers for complex state

When several values change together, or updates follow rules, put the logic in a **reducer**: a pure function `(state, action) => newState`:

```javascript
function ticketsReducer(state, action) {
  switch (action.type) {
    case "added":
      return { ...state, items: [...state.items, { id: action.id, title: action.title, status: "open" }] };
    case "closed":
      return { ...state, items: state.items.map((t) => (t.id === action.id ? { ...t, status: "closed" } : t)) };
    case "filtered":
      return { ...state, filter: action.filter };
    default:
      throw new Error(`unknown action ${action.type}`);
  }
}
let state = { items: [], filter: "all" };
for (const action of [{ type: "added", id: 1, title: "VPN" }, { type: "added", id: 2, title: "Printer" }, { type: "closed", id: 1 }]) {
  state = ticketsReducer(state, action);
}
console.log(state.items);
```

In a component: `const [state, dispatch] = useReducer(ticketsReducer, initialState)` and `dispatch({ type: "closed", id })`. Reducers are pure, so they're trivial to unit-test without React, which is exactly what the exercise does. Redux and Zustand are libraries built around the same idea for app-wide state.

## Sharing state: context

Passing props through many layers ("prop drilling") gets tedious. **Context** makes a value available to a whole subtree:

```jsx,norun
const ThemeContext = createContext("light");
<ThemeContext value={theme}><App /></ThemeContext>      // React 19 syntax
const theme = use(ThemeContext);                          // or useContext(ThemeContext)
```

Use context for things many components need and that change rarely: theme, current user, locale. For fast-changing shared state, a store library (Zustand, Redux Toolkit, Jotai) re-renders less.

## Selectors: derive, don't store

Compute derived values from state during render instead of storing them, so they can't get out of sync. Expensive derivations can be cached with `useMemo(() => compute(items), [items])`.

@@quiz
Why is `state.items.push(newItem); setState(state)` wrong in React?
* It mutates state; the object identity doesn't change, so React may not re-render, and old snapshots get corrupted
- push is not allowed in JavaScript
- setState only accepts numbers
- It's correct
? Create a new array: `[...state.items, newItem]`.

@@quiz
What is a reducer?
- A function that makes arrays shorter
* A pure function `(state, action) => newState` that holds all update logic
- A React component
- A CSS minifier
? Use it with `useReducer` or libraries like Redux.

@@quiz
When should you use `setCount((c) => c + 1)` instead of `setCount(count + 1)`?
* When the new value depends on the previous one, especially for several updates in a row
- Never
- Only in class components
- Only for strings
? Each render's `count` is a fixed snapshot.

@@drill
Which React hook gives a component a piece of state?
= useState
= useState()
? const [count, setCount] = useState(0).

@@flashcards
Vite :: fast dev server and build tool for front-end apps
JSX :: HTML-like syntax compiled to function calls
Fragment :: `<>...</>` grouping without an extra element
`useState` :: local state with a setter that triggers re-render
State snapshot :: state values are fixed during a render
`useReducer` :: state managed by a reducer function
Reducer :: pure `(state, action) => newState`
Context :: value shared with a whole component subtree
Prop drilling :: passing props through many layers
`useMemo` :: cache a derived value until dependencies change

@@exercise A help desk reducer
Write `helpdeskReducer(state, action)` for state `{ tickets: [], nextId: 1, selectedId: null, filter: { status: "all", text: "" } }`. Never mutate the input. Actions:

- `{ type: "add", title, priority }`: append `{ id: nextId, title, priority, status: "open" }` and increment `nextId`; titles are trimmed, and blank titles are ignored (return the same state object)
- `{ type: "setStatus", id, status }`: status one of `open`, `pending`, `closed`; unknown ids or statuses return the same state object
- `{ type: "select", id }` and `{ type: "remove", id }` (removing the selected ticket clears `selectedId`)
- `{ type: "filter", status?, text? }`: merge into `filter`
- unknown action types throw `Error("unknown action: <type>")`

Then `selectVisible(state)`: tickets matching the filter (status `"all"` matches everything; text matches the title case-insensitively), sorted by priority (1 = highest first), then id.
@@starter
const initialState = { tickets: [], nextId: 1, selectedId: null, filter: { status: "all", text: "" } };

function helpdeskReducer(state, action) {
  throw new Error("TODO: not done yet");
}

function selectVisible(state) {
  throw new Error("TODO: not done yet");
}

@@tests
const frozen = (x) => { Object.freeze(x); Object.values(x).forEach((v) => typeof v === "object" && v && frozen(v)); return x; };
let s = frozen(structuredClone(initialState));
const run = (a) => { s = frozen(helpdeskReducer(s, a)); };
run({ type: "add", title: " VPN down ", priority: 2 });
run({ type: "add", title: "Printer jam", priority: 3 });
run({ type: "add", title: "Server room hot", priority: 1 });
const before = s;
run({ type: "add", title: "   ", priority: 1 });
check(s === before, true, "blank title ignored");
run({ type: "setStatus", id: 2, status: "closed" });
run({ type: "setStatus", id: 99, status: "closed" });
run({ type: "select", id: 1 });
check(selectVisible(s).map((t) => t.id), [3, 1, 2], "sorted by priority");
run({ type: "filter", status: "open" });
check(selectVisible(s).map((t) => t.title), ["Server room hot", "VPN down"], "status filter");
run({ type: "filter", text: "vpn" });
check(selectVisible(s).map((t) => t.id), [1], "text filter");
run({ type: "remove", id: 1 });
check([s.tickets.length, s.selectedId, s.nextId], [2, null, 4], "remove clears selection");
let msg = "";
try { helpdeskReducer(s, { type: "explode" }); } catch (e) { msg = e.message; }
check(msg, "unknown action: explode", "unknown action");
@@hint
Use a `switch` on `action.type`; each case returns `{ ...state, ... }`. For `setStatus`, first check `["open", "pending", "closed"].includes(action.status)` and `state.tickets.some((t) => t.id === action.id)`. For `selectVisible`, filter then `toSorted((a, b) => a.priority - b.priority || a.id - b.id)`.
@@solution
const initialState = { tickets: [], nextId: 1, selectedId: null, filter: { status: "all", text: "" } };
const STATUSES = ["open", "pending", "closed"];

function helpdeskReducer(state, action) {
  switch (action.type) {
    case "add": {
      const title = (action.title ?? "").trim();
      if (!title) return state;
      const ticket = { id: state.nextId, title, priority: action.priority, status: "open" };
      return { ...state, tickets: [...state.tickets, ticket], nextId: state.nextId + 1 };
    }
    case "setStatus": {
      if (!STATUSES.includes(action.status) || !state.tickets.some((t) => t.id === action.id)) return state;
      return { ...state, tickets: state.tickets.map((t) => (t.id === action.id ? { ...t, status: action.status } : t)) };
    }
    case "select":
      return { ...state, selectedId: action.id };
    case "remove":
      return {
        ...state,
        tickets: state.tickets.filter((t) => t.id !== action.id),
        selectedId: state.selectedId === action.id ? null : state.selectedId,
      };
    case "filter": {
      const { type, ...changes } = action;
      return { ...state, filter: { ...state.filter, ...changes } };
    }
    default:
      throw new Error(`unknown action: ${action.type}`);
  }
}

function selectVisible(state) {
  const { status, text } = state.filter;
  const needle = text.toLowerCase();
  return state.tickets
    .filter((t) => (status === "all" || t.status === status) && t.title.toLowerCase().includes(needle))
    .toSorted((a, b) => a.priority - b.priority || a.id - b.id);
}

@@exercise A tiny store with subscriptions
Libraries like Redux and Zustand wrap a reducer in a **store**. Write `createStore(reducer, initialState)` returning:

- `getState()`
- `dispatch(action)`: runs the reducer; if the new state is a different object, notifies subscribers
- `subscribe(listener)`: returns an `unsubscribe` function
- `select(selector, onChange)`: calls `onChange(newValue, oldValue)` only when `selector(state)` changes (compared with `Object.is`); returns an unsubscribe function. This is how components avoid re-rendering for unrelated changes.

Dispatching from inside a listener must work (process it after the current notification loop finishes, not recursively mid-loop).
@@starter
function createStore(reducer, initialState) {
  throw new Error("TODO: not done yet");
}

@@tests
const reducer = (s, a) => {
  if (a.type === "inc") return { ...s, count: s.count + 1 };
  if (a.type === "rename") return { ...s, name: a.name };
  return s;
};
const store = createStore(reducer, { count: 0, name: "x" });
const seen = [];
const unsub = store.subscribe(() => seen.push(store.getState().count));
const names = [];
store.select((s) => s.name, (n, old) => names.push(`${old}->${n}`));
store.dispatch({ type: "inc" });
store.dispatch({ type: "noop" });
store.dispatch({ type: "rename", name: "y" });
store.dispatch({ type: "inc" });
check(seen, [1, 1, 2], "subscribers on real changes only");
check(names, ["x->y"], "selector fires only when its slice changes");
unsub();
store.dispatch({ type: "inc" });
check(seen.length, 3, "unsubscribed");
const chain = [];
store.subscribe(() => { chain.push(store.getState().count); if (store.getState().count === 4) store.dispatch({ type: "inc" }); });
store.dispatch({ type: "inc" });
check([chain, store.getState().count], [[4, 5], 5], "dispatch from a listener");
@@hint
Keep `state`, a `Set` of listeners, a `queue` of actions and a `dispatching` flag. `dispatch` pushes the action; if already dispatching, return. Otherwise loop while the queue has actions: reduce, and if changed, call every listener (iterate over a copy). `select` can be built on `subscribe` with a closure holding the last selected value.
@@solution
function createStore(reducer, initialState) {
  let state = initialState;
  const listeners = new Set();
  const queue = [];
  let dispatching = false;

  function dispatch(action) {
    queue.push(action);
    if (dispatching) return;
    dispatching = true;
    try {
      while (queue.length > 0) {
        const next = reducer(state, queue.shift());
        if (next !== state) {
          state = next;
          for (const listener of [...listeners]) listener();
        }
      }
    } finally {
      dispatching = false;
    }
  }

  function subscribe(listener) {
    listeners.add(listener);
    return () => listeners.delete(listener);
  }

  function select(selector, onChange) {
    let last = selector(state);
    return subscribe(() => {
      const value = selector(state);
      if (!Object.is(value, last)) {
        const old = last;
        last = value;
        onChange(value, old);
      }
    });
  }

  return { getState: () => state, dispatch, subscribe, select };
}

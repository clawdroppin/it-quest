@@lesson t8-state Typed State: Reducers, Context and Stores
@@concepts objects, functions
@@text
As an app grows, state management becomes the hardest part of the UI. TypeScript helps by making every state change explicit and checked.

## Reducers with discriminated actions

`useReducer` (and Redux) model state changes as **actions**. A union of action types gives you exhaustive handling and autocomplete for `dispatch`:

```tsx,norun
type State = { items: Device[]; selected: string | null; filter: string };
type Action =
  | { type: "loaded"; items: Device[] }
  | { type: "select"; serial: string | null }
  | { type: "filter"; text: string };

function reducer(state: State, action: Action): State {
  switch (action.type) {
    case "loaded": return { ...state, items: action.items };
    case "select": return { ...state, selected: action.serial };
    case "filter": return { ...state, filter: action.text };
  }
}

const [state, dispatch] = useReducer(reducer, { items: [], selected: null, filter: "" });
dispatch({ type: "select", serial: "LT-1" });      // checked: wrong fields are errors
```

Reducers are pure functions: easy to test, easy to log, easy to undo (keep past states).

## Typed context

```tsx,norun
const SessionContext = createContext<Session | null>(null);

function useSession(): Session {
  const s = useContext(SessionContext);
  if (!s) throw new Error("useSession must be used inside <SessionProvider>");
  return s;                                 // callers get Session, never null
}
```

The custom hook hides the `null` check, so components don't need it.

## External stores

For app-wide state, small store libraries are popular: **Zustand**, **Jotai**, Redux Toolkit, and signals (Preact, Solid, Angular). The core idea is tiny: a value, a way to update it, and subscriptions. React connects to a store through `useSyncExternalStore(subscribe, getSnapshot)`.

```ts,norun
const useDevices = create<DeviceState>()((set) => ({
  items: [],
  add: (d) => set((s) => ({ items: [...s.items, d] })),
}));
const count = useDevices((s) => s.items.length);    // re-renders only when the count changes
```

**Selectors** are the key to performance: a component subscribes to the slice it needs and only re-renders when that slice changes (compared with `Object.is` or a custom equality).

## Server state is different

Data fetched from an API (lists, details) is a **cache** of server state, with loading, errors, refetching and invalidation. Use TanStack Query or SWR for it instead of putting it in a global store by hand. Keep the global store for genuine client state (UI preferences, selection, drafts).

## Immutability

React compares state by reference, so never mutate: create new objects (`{ ...s, x }`, `[...items, d]`, `items.map(...)`). `readonly` types and `Readonly<T>` catch accidental mutation at compile time; Immer lets you write "mutating" code that produces immutable updates.

@@quiz
Why use a discriminated union for reducer actions?
* `dispatch` is type-checked and the reducer can handle every action exhaustively
- It's required by React
- It makes the reducer async
- It avoids re-renders
? Each action has its own payload shape.

@@quiz
What do selectors in a store give you?
- Persistence
* A component re-renders only when the selected slice changes
- Type erasure
- Server caching
? Subscribe to as little state as possible.

@@quiz
Where should data fetched from an API usually live?
- In a global Zustand store, updated by hand
* In a server-state cache like TanStack Query or SWR
- In localStorage
- In component props only
? It needs loading/error states, refetching and invalidation.

@@drill
Which React hook manages state with a reducer function?
= useReducer
= useReducer()
? const [state, dispatch] = useReducer(reducer, initial).

@@flashcards
Reducer :: `(state, action) => newState`, pure
Action union :: `{ type: "a"; ... } | { type: "b"; ... }`
`createContext<T | null>(null)` :: typed context with a guard hook
Store :: state + update + subscribe
Selector :: picks a slice of state for a subscriber
`useSyncExternalStore` :: connects React to an external store
Server state :: cached remote data (TanStack Query, SWR)
Immutability :: new objects on every change

@@exercise A reducer with undo and redo
Write `undoable<S, A extends { type: string }>(reducer: (s: S, a: A) => S, limit = 50)` that wraps any reducer into one with history:

- state: `type History<S> = { past: S[]; present: S; future: S[] }`
- actions: the inner actions plus `{ type: "undo" }` and `{ type: "redo" }`
- an inner action that returns the **same** state object (no change) doesn't create a history entry
- a new change clears `future`; `past` keeps at most `limit` entries (drop the oldest)
- also write `init<S>(present: S): History<S>`

Then use it with the given `devicesReducer`.
@@starter
type History<S> = { past: S[]; present: S; future: S[] };
type HistoryAction = { type: "undo" } | { type: "redo" };

function init<S>(present: S): History<S> {
  throw new Error("TODO: not done yet");
}

function undoable<S, A extends { type: string }>(reducer: (s: S, a: A) => S, limit = 50): (h: History<S>, a: A | HistoryAction) => History<S> {
  throw new Error("TODO: not done yet");
}

type DevAction = { type: "add"; serial: string } | { type: "remove"; serial: string };
function devicesReducer(state: readonly string[], action: DevAction): readonly string[] {
  switch (action.type) {
    case "add": return state.includes(action.serial) ? state : [...state, action.serial];
    case "remove": return state.filter((s) => s !== action.serial);
  }
}

@@tests
const reduce = undoable(devicesReducer, 2);
let h = init<readonly string[]>([]);
h = reduce(h, { type: "add", serial: "LT-1" });
h = reduce(h, { type: "add", serial: "LT-2" });
h = reduce(h, { type: "add", serial: "LT-2" });          // no change: no history entry
check([h.present, h.past.length], [["LT-1", "LT-2"], 2], "adds");
h = reduce(h, { type: "undo" });
check([h.present, h.future], [["LT-1"], [["LT-1", "LT-2"]]], "undo");
h = reduce(h, { type: "redo" });
check(h.present, ["LT-1", "LT-2"], "redo");
h = reduce(h, { type: "undo" });
h = reduce(h, { type: "add", serial: "PH-1" });
check([h.present, h.future.length], [["LT-1", "PH-1"], 0], "new change clears future");
h = reduce(h, { type: "add", serial: "PH-2" });
check(h.past, [["LT-1"], ["LT-1", "PH-1"]], "limit keeps newest");
const same = reduce(init<readonly string[]>([]), { type: "undo" });
check([same.present, same.past.length], [[], 0], "undo with no past");
@@hint
Check `a.type === "undo"` / `"redo"` first. TypeScript can't fully narrow `A | HistoryAction` by `type` because `A`'s `type` is just `string`, so after handling undo/redo, call the reducer with `a as A`. For the change check, compare `next === h.present`.
@@solution
type History<S> = { past: S[]; present: S; future: S[] };
type HistoryAction = { type: "undo" } | { type: "redo" };

function init<S>(present: S): History<S> {
  return { past: [], present, future: [] };
}

function undoable<S, A extends { type: string }>(reducer: (s: S, a: A) => S, limit = 50): (h: History<S>, a: A | HistoryAction) => History<S> {
  return (h, a) => {
    if (a.type === "undo") {
      const prev = h.past[h.past.length - 1];
      if (h.past.length === 0) return h;
      return { past: h.past.slice(0, -1), present: prev as S, future: [h.present, ...h.future] };
    }
    if (a.type === "redo") {
      const [next, ...rest] = h.future;
      if (h.future.length === 0) return h;
      return { past: [...h.past, h.present], present: next as S, future: rest };
    }
    const next = reducer(h.present, a as A);
    if (next === h.present) return h;
    return { past: [...h.past, h.present].slice(-limit), present: next, future: [] };
  };
}

type DevAction = { type: "add"; serial: string } | { type: "remove"; serial: string };
function devicesReducer(state: readonly string[], action: DevAction): readonly string[] {
  switch (action.type) {
    case "add": return state.includes(action.serial) ? state : [...state, action.serial];
    case "remove": return state.filter((s) => s !== action.serial);
  }
}

@@exercise A tiny store with selectors
Write `createStore<S extends object>(initial: S)` returning:

- `getState(): S`
- `setState(update: Partial<S> | ((s: S) => Partial<S>)): void`: shallow-merges into a **new** state object; if every updated key already has an `Object.is`-equal value, nothing happens
- `subscribe<T>(selector: (s: S) => T, listener: (value: T, previous: T) => void, equals?: (a: T, b: T) => boolean): () => void`: the listener runs only when the selected value changes (default `Object.is`); returns an unsubscribe function

Listeners run synchronously after each effective change, in subscription order.
@@starter
function createStore<S extends object>(initial: S) {
  return {
    getState(): S {
      throw new Error("TODO: not done yet");
    },
    setState(update: Partial<S> | ((s: S) => Partial<S>)): void {
      throw new Error("TODO: not done yet");
    },
    subscribe<T>(selector: (s: S) => T, listener: (value: T, previous: T) => void, equals: (a: T, b: T) => boolean = Object.is): () => void {
      throw new Error("TODO: not done yet");
    },
  };
}

@@tests
type AppState = { theme: "light" | "dark"; devices: string[]; selected: string | null };
const store = createStore<AppState>({ theme: "light", devices: ["LT-1"], selected: null });
const log: string[] = [];
const unsubTheme = store.subscribe((s) => s.theme, (t, prev) => log.push(`theme ${prev}->${t}`));
store.subscribe((s) => s.devices.length, (n) => log.push(`count ${n}`));
store.subscribe((s) => s.devices, (d) => log.push(`devices ${d.join(",")}`), (a, b) => a.join() === b.join());
const before = store.getState();
store.setState({ selected: "LT-1" });
store.setState((s) => ({ devices: [...s.devices, "PH-2"] }));
store.setState({ theme: "dark" });
store.setState({ theme: "dark" });                           // no change
store.setState((s) => ({ devices: [...s.devices] }));         // new array, same content
unsubTheme();
store.setState({ theme: "light" });
check(log, ["count 2", "devices LT-1,PH-2", "theme light->dark"], "selective notifications");
check([before.selected, store.getState().selected, before === store.getState()], [null, "LT-1", false], "immutable updates");
@@hint
Keep `let state = initial;` and an array of `{ select, last, listener, equals }` records. In `setState`, compute the patch, return early if `Object.keys(patch).every((k) => Object.is(patch[k], state[k]))`, then `state = { ...state, ...patch }` and for each subscription compare the newly selected value with `last`.
@@solution
function createStore<S extends object>(initial: S) {
  let state = initial;
  type Sub = { check: () => void };
  let subs: Sub[] = [];
  return {
    getState(): S {
      return state;
    },
    setState(update: Partial<S> | ((s: S) => Partial<S>)): void {
      const patch = typeof update === "function" ? update(state) : update;
      const keys = Object.keys(patch) as (keyof S)[];
      if (keys.every((k) => Object.is(patch[k], state[k]))) return;
      state = { ...state, ...patch };
      for (const sub of [...subs]) sub.check();
    },
    subscribe<T>(selector: (s: S) => T, listener: (value: T, previous: T) => void, equals: (a: T, b: T) => boolean = Object.is): () => void {
      let last = selector(state);
      const sub: Sub = {
        check: () => {
          const next = selector(state);
          if (equals(next, last)) return;
          const previous = last;
          last = next;
          listener(next, previous);
        },
      };
      subs.push(sub);
      return () => {
        subs = subs.filter((s) => s !== sub);
      };
    },
  };
}

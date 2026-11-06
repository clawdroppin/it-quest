@@lesson u9-components Thinking in Components: UI as a Function of State
@@concepts objects, functions
@@text
Modern front-end frameworks (React, Vue, Svelte, Solid, Angular) all share one idea: **the UI is a function of the state**. You describe what the screen should look like for the current data, and the framework works out how to update the DOM. This lesson builds that idea from scratch so React stops being magic.

## The problem with manual DOM updates

```javascript,norun
// imperative: every change must remember to update every affected element
function addTicket(t) {
  tickets.push(t);
  list.appendChild(renderRow(t));
  counter.textContent = tickets.length;
  if (tickets.length === 1) emptyMessage.hidden = true;
  // ... and the filter, the badge in the header, the page title ...
}
```

As an app grows, keeping every piece of the DOM in sync with the data becomes the main source of bugs.

## Declarative rendering

Instead, write a function from state to a description of the UI, and re-run it whenever the state changes:

```javascript
function h(type, props, ...children) {
  return { type, props: props ?? {}, children: children.flat() };
}
function TicketList({ tickets, filter }) {
  const visible = tickets.filter((t) => t.title.toLowerCase().includes(filter));
  return h("section", null,
    h("h2", null, `Tickets (${visible.length})`),
    visible.length === 0
      ? h("p", { class: "empty" }, "Nothing here")
      : h("ul", null, visible.map((t) => h("li", { key: t.id }, t.title))),
  );
}
const tree = TicketList({ tickets: [{ id: 1, title: "VPN down" }, { id: 2, title: "New laptop" }], filter: "" });
console.log(JSON.stringify(tree, null, 1).slice(0, 160) + "...");
```

The returned objects are a **virtual DOM**: a cheap, plain-object description of the page. JSX (`<li key={t.id}>{t.title}</li>`) is just syntax that compiles to calls like `h("li", { key: t.id }, t.title)` (React's version is `jsx()` / `createElement`).

## Reconciliation

Re-creating the whole real DOM on every change would be slow and lose focus, scroll position and input text. So the framework **diffs** the new virtual tree against the previous one and applies only the differences:

- same element type at the same place → keep the DOM node, update changed attributes and text
- different type → replace the subtree
- lists → match children by **key**, so reordering moves nodes instead of rebuilding them

That's why React warns about missing `key`s, and why keys must be **stable IDs**, not array indexes (with index keys, deleting the first row makes every row's state shift to its neighbour).

## Components

A **component** is a function (props → UI). Components compose like functions: `App` renders `Header`, `TicketList`, `TicketDetails`. Rules of thumb:

- **Props** flow down (read-only inputs); **events** flow up (callbacks like `onSelect(id)`).
- Keep state in the **lowest common parent** of the components that need it ("lifting state up").
- Derive what you can: don't store `visibleTickets` in state if you can compute it from `tickets` and `filter`.
- Small, focused components with clear props are easier to test and reuse.

## Other approaches

- **Svelte** and **Solid** compile components to precise DOM updates (no virtual DOM diffing); Solid and Vue use **signals**: reactive values that know exactly which parts of the UI depend on them.
- **htmx** and server-rendered HTML keep most logic on the server.
- For small widgets, plain DOM code is fine.

The exercises implement `h`, rendering to an HTML string (what server-side rendering does), and a keyed list diff.

@@quiz
What does "UI is a function of state" mean?
* You describe the whole UI for the current data, and the framework updates the DOM to match
- The UI is stored in the database
- Each function draws one pixel
- State is a function
? It replaces manual, incremental DOM updates.

@@quiz
Why should list keys be stable IDs rather than array indexes?
- Indexes are too slow
* With index keys, inserting or deleting rows makes the framework match the wrong items, mixing up their state
- Keys must be strings
- Indexes are not allowed
? Keys identify items across renders.

@@quiz
What is JSX?
- A separate language that runs in the browser
* Syntax that compiles to function calls creating virtual DOM objects
- A CSS framework
- A JSON format
? `<b>{x}</b>` becomes something like `jsx("b", { children: x })`.

@@drill
What's the name of the process where a framework compares the new UI tree with the old one?
= reconciliation
= diffing
? Only the differences are applied to the DOM.

@@flashcards
Declarative UI :: describe the result for the current state
Virtual DOM :: plain objects describing the UI
Reconciliation :: diffing new and old trees to update the DOM
Key :: stable identity of a list item across renders
Component :: function from props to UI
Props :: read-only inputs passed down
Lifting state up :: moving state to the common parent
Derived state :: values computed from state instead of stored
Signals :: reactive values that track their dependents

@@exercise Render to HTML
Write `h(type, props, ...children)` (as in the lesson, flattening children and dropping `null`, `undefined`, `false` and `true`) and `renderToString(node)`:

- strings and numbers are **escaped** text (`& < > "` → `&amp; &lt; &gt; &quot;`)
- if `type` is a **function**, call it with `{ ...props, children }` and render the result (that's a component)
- elements: `<type attr="value">children</type>`; attributes in the order given, values escaped; `key` is not rendered; a `true` value renders just the name (`disabled`), `false`/`null`/`undefined` are omitted; `className` renders as `class`
- void elements `br`, `hr`, `img`, `input`, `meta`, `link` render without children or closing tag: `<br>`
@@starter
function h(type, props, ...children) {
  throw new Error("TODO: not done yet");
}

function renderToString(node) {
  throw new Error("TODO: not done yet");
}

@@tests
const Badge = ({ count, children }) => h("span", { className: "badge" }, children, ": ", count);
const page = h("div", { id: "app" },
  h("h1", null, "Tickets <3"),
  h(Badge, { count: 2 }, "open"),
  false,
  h("ul", null, [1, 2].map((n) => h("li", { key: n, "data-id": n }, `#${n}`))),
  h("input", { type: "checkbox", checked: true, disabled: false }),
  h("a", { href: 'x" onclick="alert(1)' }, "link"),
);
check(renderToString(page), '<div id="app"><h1>Tickets &lt;3</h1><span class="badge">open: 2</span><ul><li data-id="1">#1</li><li data-id="2">#2</li></ul><input type="checkbox" checked><a href="x&quot; onclick=&quot;alert(1)">link</a></div>', "html");
check(renderToString(h("p", null, null, 0, undefined, "")), "<p>0</p>", "empty children skipped, 0 kept");
@@hint
In `h`: `children.flat(Infinity).filter((c) => c !== null && c !== undefined && typeof c !== "boolean")`. In `renderToString`: strings/numbers → escape; functions → `renderToString(node.type({ ...node.props, children: node.children }))`; arrays → map and join; elements → build attributes, then children.
@@solution
const VOID = new Set(["br", "hr", "img", "input", "meta", "link"]);

function h(type, props, ...children) {
  return {
    type,
    props: props ?? {},
    children: children.flat(Infinity).filter((c) => c !== null && c !== undefined && typeof c !== "boolean"),
  };
}

function escape(text) {
  return String(text).replaceAll("&", "&amp;").replaceAll("<", "&lt;").replaceAll(">", "&gt;").replaceAll('"', "&quot;");
}

function renderToString(node) {
  if (Array.isArray(node)) return node.map(renderToString).join("");
  if (typeof node === "string" || typeof node === "number") return escape(node);
  if (node === null || node === undefined || typeof node === "boolean") return "";
  if (typeof node.type === "function") {
    return renderToString(node.type({ ...node.props, children: node.children }));
  }
  let attrs = "";
  for (const [name, value] of Object.entries(node.props)) {
    if (name === "key" || value === false || value === null || value === undefined) continue;
    const attr = name === "className" ? "class" : name;
    attrs += value === true ? ` ${attr}` : ` ${attr}="${escape(value)}"`;
  }
  if (VOID.has(node.type)) return `<${node.type}${attrs}>`;
  return `<${node.type}${attrs}>${node.children.map(renderToString).join("")}</${node.type}>`;
}

@@exercise Keyed list diff
When a list re-renders, the framework must decide what to do with each row. Write `diffKeyed(oldKeys, newKeys)` returning a list of operations that turn the old list into the new one:

- `{ op: "remove", key }` for keys that disappeared (in old order)
- then, walking the new list in order: `{ op: "insert", key, index }` for new keys, and `{ op: "move", key, index }` for existing keys whose position **relative to the other surviving keys** changed. To decide moves, find the **longest increasing subsequence** of old positions among surviving keys (in new order): keys in it stay; others move.

Keys are unique strings.
@@starter
function diffKeyed(oldKeys, newKeys) {
  throw new Error("TODO: not done yet");
}

@@tests
check(diffKeyed(["a", "b", "c"], ["a", "b", "c"]), [], "no change");
check(diffKeyed(["a", "b", "c"], ["a", "c"]), [{ op: "remove", key: "b" }], "remove");
check(diffKeyed(["a", "b"], ["x", "a", "b", "y"]), [{ op: "insert", key: "x", index: 0 }, { op: "insert", key: "y", index: 3 }], "inserts");
check(diffKeyed(["a", "b", "c", "d"], ["d", "a", "b", "c"]), [{ op: "move", key: "d", index: 0 }], "one move, not three");
check(diffKeyed(["a", "b", "c"], ["c", "b", "a"]), [{ op: "move", key: "c", index: 0 }, { op: "move", key: "b", index: 1 }], "reverse");
@@hint
Map old keys to their index. Survivors are new keys present in the old map; their old positions form an array. Compute an LIS of that array (an O(n²) dynamic programming version is fine; when several are equally long, prefer the one ending latest) and put the keys in it in a `Set` of keys that stay. Then walk `newKeys` with their new index.
@@solution
function longestIncreasing(values) {
  const n = values.length;
  const len = new Array(n).fill(1);
  const prev = new Array(n).fill(-1);
  let best = -1;
  for (let i = 0; i < n; i++) {
    for (let j = 0; j < i; j++) {
      if (values[j] < values[i] && len[j] + 1 > len[i]) {
        len[i] = len[j] + 1;
        prev[i] = j;
      }
    }
    if (best === -1 || len[i] >= len[best]) best = i;
  }
  const picked = new Set();
  for (let i = best; i !== -1; i = prev[i]) picked.add(i);
  return picked;
}

function diffKeyed(oldKeys, newKeys) {
  const oldIndex = new Map(oldKeys.map((k, i) => [k, i]));
  const newSet = new Set(newKeys);
  const ops = oldKeys.filter((k) => !newSet.has(k)).map((key) => ({ op: "remove", key }));
  const survivors = newKeys.filter((k) => oldIndex.has(k));
  const stayIdx = longestIncreasing(survivors.map((k) => oldIndex.get(k)));
  const stay = new Set(survivors.filter((_, i) => stayIdx.has(i)));
  newKeys.forEach((key, index) => {
    if (!oldIndex.has(key)) ops.push({ op: "insert", key, index });
    else if (!stay.has(key)) ops.push({ op: "move", key, index });
  });
  return ops;
}

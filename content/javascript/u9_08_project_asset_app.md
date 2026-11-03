@@lesson u9-project 🏆 Module Project: An Asset Tracker Desktop App
@@concepts objects, files, json, testing
@@text
The capstone combines the module: an **IT asset tracker** desktop app. A React UI lists laptops, monitors and phones in a sortable, filterable table with a validated form, undo, keyboard shortcuts and CSV import/export. It runs in Electron or Tauri with a narrow, validated IPC API to a local SQLite database.

## Architecture

```text
src/
├── domain/                     pure logic, no React, no Electron: unit-tested
│   ├── assets.js               reducer + selectors + undo history
│   └── csv.js                  CSV import/export with validation
├── ui/                         React components (TicketTable-style table, AssetForm, Toolbar)
│   └── hooks/useAssets.js      connects the store to IPC
├── main/                       Electron main process (or src-tauri/ for Tauri)
│   ├── ipc.js                  validated handlers (the IPC router from the last lesson)
│   └── db.js                   node:sqlite / better-sqlite3
└── preload.js                  exposes window.api.assets.{list,save,remove,import,export}
```

The exercises build the `domain/` folder, the part worth testing most. The UI and IPC glue follow the patterns from the previous lessons.

## The UI, sketched

```jsx,norun
export function AssetsPage() {
  const { state, dispatch, undo, redo } = useAssets();
  const rows = selectRows(state);
  useHotkeys({ "mod+z": undo, "mod+shift+z": redo, "mod+n": () => setEditing({}) });
  return (
    <main>
      <Toolbar filter={state.filter} onFilter={(f) => dispatch({ type: "filter", ...f })} />
      <table aria-label="Assets">
        <thead>
          <tr>{COLUMNS.map((c) => (
            <th key={c.key} aria-sort={sortState(state, c.key)}>
              <button onClick={() => dispatch({ type: "sort", column: c.key })}>{c.label}</button>
            </th>
          ))}</tr>
        </thead>
        <tbody>{rows.map((a) => <AssetRow key={a.tag} asset={a} />)}</tbody>
      </table>
      <p role="status" aria-live="polite">{rows.length} of {state.assets.length} assets</p>
    </main>
  );
}
```

Note the accessibility details: real buttons in sortable headers, `aria-sort`, a labelled table and a live region announcing the count.

## Undo and redo

Because the reducer is pure and state is immutable, undo is easy: keep a list of **past** states and a list of **future** states. Every action pushes the current state onto `past` and clears `future`; undo moves one state from `past` to `future`; redo does the reverse. Limit the history length (say 100). Only undo **user actions** (add, edit, delete, import), not filter or sort changes.

## CSV import and export

CSV is how IT data moves between tools (Intune exports, spreadsheets, procurement lists). The format looks trivial but isn't: fields can contain commas, quotes (written as `""`) and even newlines when quoted. And Excel opens CSV files that start with `=`, `+`, `-` or `@` as **formulas** ("CSV injection"), so exports must neutralise those by prefixing a `'`.

## Shipping checklist

1. Domain tests pass (`node --test`); one end-to-end test with Playwright's Electron support clicks through add/edit/undo.
2. Keyboard-only use works; axe reports no violations.
3. IPC handlers validate input; CSP is strict; renderer is sandboxed.
4. Installer is signed; auto-update tested from the previous version.
5. Data lives in the user's app-data folder, with a backup/export option.

@@quiz
Why keep the reducer and CSV code free of React and Electron imports?
* So the core logic can be unit-tested quickly and reused (CLI, web version, server)
- React can't import plain functions
- Electron forbids it
- It makes the app smaller
? Keep frameworks at the edges.

@@quiz
How does immutable state make undo simple?
* Previous states are untouched snapshots, so undo just restores the last one from a history list
- Immutable state can't be changed, so there's nothing to undo
- React provides undo automatically
- It requires a database
? Keep `past`, `present` and `future`.

@@quiz
Why prefix exported CSV cells starting with `=` or `+` with a `'`?
* To stop spreadsheet apps from executing them as formulas (CSV injection)
- To make numbers bigger
- It's required by RFC 4180
- To mark comments
? A cell like `=HYPERLINK(...)` could trick users or leak data.

@@flashcards
Domain layer :: pure business logic without UI or framework code
Undo history :: past / present / future states
CSV quoting :: fields with commas, quotes or newlines are wrapped in quotes; `"` becomes `""`
CSV injection :: spreadsheet formulas hidden in exported data
`aria-sort` :: tells assistive tech how a column is sorted
Live region :: announces changing status text
Playwright :: browser and Electron end-to-end testing tool

@@exercise Assets with undo and redo
Write `createHistory(reducer, initial, { limit = 100, undoable = () => true })` returning `{ present, past, future }`-style state through these functions:

- `getState()` → the present state
- `dispatch(action)`: compute the next state; if `undoable(action)` and the state changed, push the old present onto `past` (keeping at most `limit` entries, dropping the oldest) and clear `future`; non-undoable actions change the present without touching history
- `undo()` / `redo()`: move between states; return true if something happened
- `canUndo()` / `canRedo()`

Also write `assetsReducer(state, action)` for state `{ assets: [], sort: "tag" }` with actions `add` (`asset` with a unique `tag`; duplicates are ignored, returning the same state), `update` (`tag`, `changes`), `remove` (`tag`) and `sort` (`column`).
@@starter
function assetsReducer(state, action) {
  throw new Error("TODO: not done yet");
}

function createHistory(reducer, initial, { limit = 100, undoable = () => true } = {}) {
  throw new Error("TODO: not done yet");
}

@@tests
const h = createHistory(assetsReducer, { assets: [], sort: "tag" }, { limit: 3, undoable: (a) => a.type !== "sort" });
const tags = () => h.getState().assets.map((a) => `${a.tag}:${a.owner}`);
h.dispatch({ type: "add", asset: { tag: "IT-1", owner: "ana" } });
h.dispatch({ type: "add", asset: { tag: "IT-2", owner: "bo" } });
h.dispatch({ type: "add", asset: { tag: "IT-2", owner: "dup" } });
h.dispatch({ type: "update", tag: "IT-1", changes: { owner: "cy" } });
h.dispatch({ type: "sort", column: "owner" });
check([tags(), h.getState().sort], [["IT-1:cy", "IT-2:bo"], "owner"], "state");
check([h.undo(), tags()], [true, ["IT-1:ana", "IT-2:bo"]], "undo steps over the sort and the ignored duplicate");
check([h.undo(), h.undo(), tags(), h.canUndo()], [true, true, [], false], "back to start");
check([h.undo(), h.redo(), tags()], [false, true, ["IT-1:ana"]], "redo");
h.dispatch({ type: "remove", tag: "IT-1" });
check(h.canRedo(), false, "new action clears future");
for (let i = 3; i < 10; i++) h.dispatch({ type: "add", asset: { tag: `IT-${i}`, owner: "x" } });
let undos = 0;
while (h.undo()) undos++;
check(undos, 3, "history limited");
@@hint
Keep `let past = [], present = initial, future = [];`. In `dispatch`: `const next = reducer(present, action); if (next === present) return; if (undoable(action)) { past = [...past, present].slice(-limit); future = []; } present = next;`. Undo: `future = [present, ...future]; present = past.at(-1); past = past.slice(0, -1);`.
@@solution
function assetsReducer(state, action) {
  switch (action.type) {
    case "add":
      if (state.assets.some((a) => a.tag === action.asset.tag)) return state;
      return { ...state, assets: [...state.assets, { ...action.asset }] };
    case "update":
      return { ...state, assets: state.assets.map((a) => (a.tag === action.tag ? { ...a, ...action.changes } : a)) };
    case "remove":
      return { ...state, assets: state.assets.filter((a) => a.tag !== action.tag) };
    case "sort":
      return { ...state, sort: action.column };
    default:
      return state;
  }
}

function createHistory(reducer, initial, { limit = 100, undoable = () => true } = {}) {
  let past = [];
  let present = initial;
  let future = [];
  return {
    getState: () => present,
    dispatch(action) {
      const next = reducer(present, action);
      if (next === present) return;
      if (undoable(action)) {
        past = [...past, present].slice(-limit);
        future = [];
      }
      present = next;
    },
    undo() {
      if (past.length === 0) return false;
      future = [present, ...future];
      present = past.at(-1);
      past = past.slice(0, -1);
      return true;
    },
    redo() {
      if (future.length === 0) return false;
      past = [...past, present];
      present = future[0];
      future = future.slice(1);
      return true;
    },
    canUndo: () => past.length > 0,
    canRedo: () => future.length > 0,
  };
}

@@exercise CSV import and export
Write:

- `toCsv(rows, columns)`: header line from `columns`, then one line per row, joined with `\r\n`. Quote a field (wrapping in `"` and doubling inner quotes) when it contains a comma, quote, `\r` or `\n`. **Neutralise formulas**: a string starting with `=`, `+`, `-`, `@`, tab or carriage return gets a `'` prefix (then quoting rules apply). `null`/`undefined` become empty fields.
- `parseCsv(text)`: parse RFC 4180 CSV (quoted fields with commas, `""` escapes and newlines; `\n` or `\r\n` line endings; a trailing newline is ignored) into objects keyed by the header row. Throw `Error("row <n>: expected <a> fields, got <b>")` (n = 1-based data row) when a row has the wrong number of fields.
@@starter
function toCsv(rows, columns) {
  throw new Error("TODO: not done yet");
}

function parseCsv(text) {
  throw new Error("TODO: not done yet");
}

@@tests
const rows = [
  { tag: "IT-1", model: 'Dell 14", silver', owner: "Ana", notes: "line1\nline2" },
  { tag: "IT-2", model: "=HYPERLINK(\"http://x\")", owner: null, notes: "-5 dents" },
];
const csv = toCsv(rows, ["tag", "model", "owner", "notes"]);
check(csv, 'tag,model,owner,notes\r\nIT-1,"Dell 14"", silver",Ana,"line1\nline2"\r\nIT-2,"\'=HYPERLINK(""http://x"")",,\'-5 dents', "export");
check(parseCsv(csv)[0], { tag: "IT-1", model: 'Dell 14", silver', owner: "Ana", notes: "line1\nline2" }, "round trip");
check(parseCsv("a,b\n1,2\n3,4\n"), [{ a: "1", b: "2" }, { a: "3", b: "4" }], "simple with trailing newline");
let msg = "";
try { parseCsv("a,b\n1,2\n3\n"); } catch (e) { msg = e.message; }
check(msg, "row 2: expected 2 fields, got 1", "wrong field count");
@@hint
Parse character by character with an `inQuotes` flag: in quotes, `""` adds one quote and a lone `"` ends quoting; outside, `,` ends a field, `\n` ends a row (drop a preceding `\r`), and `"` at the start of a field starts quoting. Collect rows as arrays, then map them to objects with the header.
@@solution
function cell(value) {
  if (value === null || value === undefined) return "";
  let text = String(value);
  if (typeof value === "string" && /^[=+\-@\t\r]/.test(text)) text = "'" + text;
  if (/[",\r\n]/.test(text)) text = `"${text.replaceAll('"', '""')}"`;
  return text;
}

function toCsv(rows, columns) {
  const lines = [columns.join(",")];
  for (const row of rows) lines.push(columns.map((c) => cell(row[c])).join(","));
  return lines.join("\r\n");
}

function parseCsv(text) {
  const records = [];
  let row = [];
  let field = "";
  let inQuotes = false;
  let fieldStarted = false;
  for (let i = 0; i < text.length; i++) {
    const ch = text[i];
    if (inQuotes) {
      if (ch === '"' && text[i + 1] === '"') { field += '"'; i++; }
      else if (ch === '"') inQuotes = false;
      else field += ch;
    } else if (ch === '"' && field === "") {
      inQuotes = true;
      fieldStarted = true;
    } else if (ch === ",") {
      row.push(field); field = ""; fieldStarted = true;
    } else if (ch === "\n" || (ch === "\r" && text[i + 1] === "\n")) {
      if (ch === "\r") i++;
      row.push(field);
      records.push(row);
      row = []; field = ""; fieldStarted = false;
    } else {
      field += ch;
      fieldStarted = true;
    }
  }
  if (fieldStarted || field !== "" || row.length > 0) {
    row.push(field);
    records.push(row);
  }
  const [header, ...data] = records;
  return data.map((values, i) => {
    if (values.length !== header.length) throw new Error(`row ${i + 1}: expected ${header.length} fields, got ${values.length}`);
    return Object.fromEntries(header.map((h, j) => [h, values[j]]));
  });
}

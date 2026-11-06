@@lesson t8-react-ts React with TypeScript: Props, Events and Generic Components
@@concepts functions, objects
@@text
React and TypeScript are the most common pairing for modern UIs, in the browser and in desktop apps built with Tauri or Electron. This module assumes the React basics from the JavaScript track and focuses on **typing** components well, plus testing UI logic without a browser.

## Typing props

```tsx,norun
type BadgeProps = {
  label: string;
  tone?: "neutral" | "good" | "bad";          // optional with a union of allowed values
  children?: React.ReactNode;                 // anything renderable
  onClick?: (event: React.MouseEvent<HTMLButtonElement>) => void;
};

function Badge({ label, tone = "neutral", children, onClick }: BadgeProps) {
  return <button className={`badge badge-${tone}`} onClick={onClick}>{label}{children}</button>;
}
```

- Prefer a plain function with typed props over `React.FC` (simpler, and no implicit `children`).
- `React.ReactNode` for children; `React.ReactElement` when you need exactly one element.
- Extend native element props to pass attributes through: `type InputProps = React.ComponentProps<"input"> & { label: string }`.

## Events

| Event | Type |
|---|---|
| click | `React.MouseEvent<HTMLButtonElement>` |
| change on input | `React.ChangeEvent<HTMLInputElement>` (`e.target.value`) |
| submit | `React.FormEvent<HTMLFormElement>` |
| key | `React.KeyboardEvent<HTMLElement>` (`e.key === "Enter"`) |

Inline handlers get these types automatically through contextual typing: `onChange={(e) => setName(e.target.value)}`.

## Props that depend on each other

Use a **discriminated union** when some props only make sense together, so impossible combinations don't compile:

```tsx,norun
type ButtonProps =
  | { variant: "link"; href: string; onClick?: never }
  | { variant: "action"; onClick: () => void; href?: never; busy?: boolean };

<Button variant="link" href="/docs" />          // OK
<Button variant="action" href="/docs" />        // error: href not allowed, onClick missing
```

## Generic components

A table that works for any row type, where column keys are checked against the row:

```tsx,norun
type Column<T> = { key: keyof T & string; header: string; render?: (row: T) => React.ReactNode };

function DataTable<T extends { id: string }>({ rows, columns }: { rows: T[]; columns: Column<T>[] }) {
  return (
    <table>
      <thead><tr>{columns.map((c) => <th key={c.key}>{c.header}</th>)}</tr></thead>
      <tbody>{rows.map((r) => <tr key={r.id}>{columns.map((c) => <td key={c.key}>{c.render ? c.render(r) : String(r[c.key])}</td>)}</tr>)}</tbody>
    </table>
  );
}

<DataTable rows={devices} columns={[{ key: "serial", header: "Serial" }, { key: "owner", header: "Owner" }]} />
// { key: "colour" } would be a compile error: not a key of Device
```

## Separate logic from rendering

Components should mostly **arrange** things; the logic (filtering, sorting, formatting, validation, state transitions) lives in plain typed functions and hooks. Plain functions are trivial to test without a DOM, which is what this module's exercises do. For the rendering itself, use React Testing Library (queries by role and label, like a user would) and Playwright for end-to-end tests.

@@quiz
What's the type of `e` in an `<input onChange={(e) => ...}>` handler?
* `React.ChangeEvent<HTMLInputElement>`
- `Event`
- `React.MouseEvent<HTMLInputElement>`
- `string`
? Contextual typing gives it to you automatically in inline handlers.

@@quiz
How can a Button component reject `href` when `variant="action"`?
- With a runtime check only
* With a discriminated union of prop types, using `href?: never` in the action variant
- With PropTypes
- It can't
? Make impossible combinations unrepresentable.

@@quiz
In a generic `DataTable<T>`, what does `key: keyof T & string` give you?
- Any string
* Column keys checked against the row type, with autocomplete
- Numeric keys only
- Nothing at runtime
? Typos in column keys become compile errors.

@@drill
Which React type describes anything a component can render?
= ReactNode
= React.ReactNode
? children: ReactNode.

@@flashcards
Props type :: an object type for a component's parameters
`React.ReactNode` :: anything React can render
`React.ComponentProps<"input">` :: all native input attributes
`React.ChangeEvent<HTMLInputElement>` :: input change event
Discriminated props :: union of prop shapes with a tag like `variant`
`prop?: never` :: forbids a prop in a union variant
Generic component :: `function Table<T>(props: { rows: T[] })`
React Testing Library :: test components by role/label like a user

@@exercise A generic table model
Write the logic behind a generic data table:

- `type Column<T> = { key: keyof T & string; header: string; format?: (value: T[keyof T], row: T) => string; align?: "left" | "right" }`
- `tableModel<T>(rows: readonly T[], columns: Column<T>[], opts: { sortBy?: keyof T & string; desc?: boolean; filter?: string })` returning `{ headers: string[]; cells: string[][]; count: string }`:
  - `filter` keeps rows where any **displayed** cell contains the text (case-insensitive)
  - sort by the raw value of `sortBy` (numbers numerically, everything else by `String(...).localeCompare`), descending when `desc`
  - cells are `format(value, row)` or `String(value)`; `undefined`/`null` show as `"—"`
  - `count` is `"<shown> of <total>"`
@@starter
type Column<T> = { key: keyof T & string; header: string; format?: (value: T[keyof T], row: T) => string; align?: "left" | "right" };

function tableModel<T>(rows: readonly T[], columns: Column<T>[], opts: { sortBy?: keyof T & string; desc?: boolean; filter?: string } = {}): { headers: string[]; cells: string[][]; count: string } {
  throw new Error("TODO: not done yet");
}

@@tests
interface Device { serial: string; owner?: string; ramGb: number; site: string }
const devices: Device[] = [
  { serial: "LT-10", owner: "ana", ramGb: 16, site: "Tokyo" },
  { serial: "LT-2", ramGb: 32, site: "Osaka" },
  { serial: "LT-7", owner: "kenji", ramGb: 8, site: "Tokyo" },
];
const columns: Column<Device>[] = [
  { key: "serial", header: "Serial" },
  { key: "owner", header: "Owner" },
  { key: "ramGb", header: "RAM", format: (v) => `${String(v)} GB`, align: "right" },
];
const all = tableModel(devices, columns, { sortBy: "ramGb", desc: true });
check(all.headers, ["Serial", "Owner", "RAM"], "headers");
check(all.cells, [["LT-2", "—", "32 GB"], ["LT-10", "ana", "16 GB"], ["LT-7", "kenji", "8 GB"]], "sorted desc by number");
check(tableModel(devices, columns, { filter: "KEN" }).cells, [["LT-7", "kenji", "8 GB"]], "filter on displayed cells");
check(tableModel(devices, columns, { filter: "tokyo" }).count, "0 of 3", "site isn't displayed");
check(tableModel(devices, columns, { sortBy: "serial" }).cells.map((r) => r[0]), ["LT-10", "LT-2", "LT-7"], "string sort");
function _typeTests(): void {
  // @ts-expect-error not a key of Device
  tableModel(devices, [{ key: "colour", header: "Colour" }]);
}
@@hint
Format first (`const shown = rows.map((row) => ({ row, cells: columns.map(...) }))`), then filter on `cells`, then sort on `row[sortBy]`. A cell: `const v = row[c.key]; v === undefined || v === null ? "—" : c.format ? c.format(v, row) : String(v)`.
@@solution
type Column<T> = { key: keyof T & string; header: string; format?: (value: T[keyof T], row: T) => string; align?: "left" | "right" };

function tableModel<T>(rows: readonly T[], columns: Column<T>[], opts: { sortBy?: keyof T & string; desc?: boolean; filter?: string } = {}): { headers: string[]; cells: string[][]; count: string } {
  const cellOf = (row: T, c: Column<T>): string => {
    const v = row[c.key];
    if (v === undefined || v === null) return "—";
    return c.format ? c.format(v, row) : String(v);
  };
  const needle = (opts.filter ?? "").toLowerCase();
  let shown = rows
    .map((row) => ({ row, cells: columns.map((c) => cellOf(row, c)) }))
    .filter((r) => needle === "" || r.cells.some((cell) => cell.toLowerCase().includes(needle)));
  const sortBy = opts.sortBy;
  if (sortBy !== undefined) {
    const dir = opts.desc ? -1 : 1;
    shown = shown.sort((a, b) => {
      const x = a.row[sortBy];
      const y = b.row[sortBy];
      const cmp = typeof x === "number" && typeof y === "number" ? x - y : String(x).localeCompare(String(y));
      return cmp * dir;
    });
  }
  return { headers: columns.map((c) => c.header), cells: shown.map((r) => r.cells), count: `${shown.length} of ${rows.length}` };
}

@@exercise Discriminated button props
A design system's `Button` accepts:

```ts,norun
type ButtonProps =
  | { variant: "link"; label: string; href: string; external?: boolean }
  | { variant: "action"; label: string; onPress: () => void; busy?: boolean; danger?: boolean }
  | { variant: "menu"; label: string; items: { id: string; label: string }[]; onSelect: (id: string) => void };
```

Write `buttonView(props: ButtonProps): { tag: "a" | "button"; attrs: Record<string, string | boolean>; text: string }`:

- `link`: tag `a`, attrs `{ href }`, plus `target: "_blank"` and `rel: "noopener noreferrer"` when `external`
- `action`: tag `button`, attrs `{ type: "button", disabled: busy, "aria-busy": busy }` (busy defaults to false), plus `class: "danger"` when `danger`; text is `"Working…"` while busy
- `menu`: tag `button`, attrs `{ type: "button", "aria-haspopup": "menu", "aria-label": "<label> (<n> options)" }`

Also write `press(props: ButtonProps, itemId?: string): string` that triggers the behaviour: link → returns `"navigate <href>"`; action → calls `onPress` unless busy, returns `"pressed"` or `"ignored"`; menu → calls `onSelect(itemId)` if the id exists, returns `"selected <id>"` or `"ignored"`. End both switches with an exhaustiveness check.
@@starter
type ButtonProps =
  | { variant: "link"; label: string; href: string; external?: boolean }
  | { variant: "action"; label: string; onPress: () => void; busy?: boolean; danger?: boolean }
  | { variant: "menu"; label: string; items: { id: string; label: string }[]; onSelect: (id: string) => void };

function buttonView(props: ButtonProps): { tag: "a" | "button"; attrs: Record<string, string | boolean>; text: string } {
  throw new Error("TODO: not done yet");
}

function press(props: ButtonProps, itemId?: string): string {
  throw new Error("TODO: not done yet");
}

@@tests
check(buttonView({ variant: "link", label: "Docs", href: "https://docs.example", external: true }), { tag: "a", attrs: { href: "https://docs.example", target: "_blank", rel: "noopener noreferrer" }, text: "Docs" }, "external link");
const log: string[] = [];
const save: ButtonProps = { variant: "action", label: "Save", onPress: () => log.push("saved"), danger: true };
check(buttonView(save), { tag: "button", attrs: { type: "button", disabled: false, "aria-busy": false, class: "danger" }, text: "Save" }, "action");
check(buttonView({ ...save, busy: true }).text, "Working…", "busy text");
check([press(save), press({ ...save, busy: true })], ["pressed", "ignored"], "press action");
const menu: ButtonProps = { variant: "menu", label: "Export", items: [{ id: "csv", label: "CSV" }, { id: "pdf", label: "PDF" }], onSelect: (id) => log.push(`export ${id}`) };
check(buttonView(menu).attrs["aria-label"], "Export (2 options)", "menu label");
check([press(menu, "pdf"), press(menu, "xls"), press({ variant: "link", label: "x", href: "/a" })], ["selected pdf", "ignored", "navigate /a"], "press menu and link");
check(log, ["saved", "export pdf"], "side effects");
function _typeTests(): void {
  // @ts-expect-error an action button has no href
  buttonView({ variant: "action", label: "x", onPress: () => {}, href: "/x" });
}
@@hint
`switch (props.variant)`: inside each case TypeScript narrows `props`, so `props.href` is only available under `"link"`. Write `function assertNever(x: never): never` and call it in `default`.
@@solution
type ButtonProps =
  | { variant: "link"; label: string; href: string; external?: boolean }
  | { variant: "action"; label: string; onPress: () => void; busy?: boolean; danger?: boolean }
  | { variant: "menu"; label: string; items: { id: string; label: string }[]; onSelect: (id: string) => void };

function assertNever(x: never): never {
  throw new Error(`unexpected ${JSON.stringify(x)}`);
}

function buttonView(props: ButtonProps): { tag: "a" | "button"; attrs: Record<string, string | boolean>; text: string } {
  switch (props.variant) {
    case "link": {
      const attrs: Record<string, string | boolean> = { href: props.href };
      if (props.external) Object.assign(attrs, { target: "_blank", rel: "noopener noreferrer" });
      return { tag: "a", attrs, text: props.label };
    }
    case "action": {
      const busy = props.busy ?? false;
      const attrs: Record<string, string | boolean> = { type: "button", disabled: busy, "aria-busy": busy };
      if (props.danger) attrs.class = "danger";
      return { tag: "button", attrs, text: busy ? "Working…" : props.label };
    }
    case "menu":
      return {
        tag: "button",
        attrs: { type: "button", "aria-haspopup": "menu", "aria-label": `${props.label} (${props.items.length} options)` },
        text: props.label,
      };
    default:
      return assertNever(props);
  }
}

function press(props: ButtonProps, itemId?: string): string {
  switch (props.variant) {
    case "link":
      return `navigate ${props.href}`;
    case "action":
      if (props.busy) return "ignored";
      props.onPress();
      return "pressed";
    case "menu":
      if (itemId === undefined || !props.items.some((i) => i.id === itemId)) return "ignored";
      props.onSelect(itemId);
      return `selected ${itemId}`;
    default:
      return assertNever(props);
  }
}

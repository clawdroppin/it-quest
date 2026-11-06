@@lesson t8-project-desk Project: Asset Desk, a Keyboard-First Desktop App Core
@@concepts objects, functions, strings
@@text
This project builds the core of a desktop asset tracker the way production teams structure it: the React components are thin, and the behaviour lives in a typed **view model** and small pure modules that are tested without a window.

## Architecture

```text
React components (render only)
      │  props / callbacks
      ▼
View models: AssetListVM, CommandPalette     ← this project
      │  typed commands
      ▼
Bridge (typed invoke/listen)  ──►  Rust backend (SQLite, OS, network)
```

```tsx,norun
function AssetList({ vm }: { vm: AssetListVM }) {
  const view = useSyncExternalStore(vm.subscribe, vm.snapshot);
  return (
    <div role="listbox" tabIndex={0} aria-activedescendant={view.activeId} onKeyDown={(e) => vm.key(e.key)}>
      {view.rows.map((r) => <div role="option" id={r.tag} key={r.tag} aria-selected={r.selected}>{r.label}</div>)}
      <p aria-live="polite">{view.summary}</p>
    </div>
  );
}
```

## Keyboard-first design

Power users (IT staff!) live on the keyboard:

- Arrow keys move the selection, Home/End jump, PageUp/PageDown move a page, Enter opens, Escape clears.
- A **command palette** (Ctrl/Cmd+K or Ctrl+Shift+P) finds any action by typing a few letters, with fuzzy matching and recently used commands first. VS Code, Slack, Linear and Figma all have one.
- `role="listbox"` + `aria-activedescendant` makes the selection visible to screen readers while focus stays on the list.

## Fuzzy matching

A simple, explainable fuzzy matcher: every query character must appear **in order** in the text (a subsequence). Score each matched character, with bonuses for consecutive matches and matches at the start of a word, so `"nd"` ranks "**N**ew **D**evice" above "co**n**fig rea**d**er".

@@quiz
Why keep behaviour in view models instead of in React components?
* It can be tested without rendering and reused across components and platforms
- React can't run logic
- It's required by Tauri
- It makes bundles smaller
? Components become thin rendering code.

@@quiz
What does `aria-activedescendant` do on a listbox?
- Moves keyboard focus to the option
* Tells assistive technology which option is active while focus stays on the listbox
- Selects all options
- Hides inactive options
? It's the standard pattern for keyboard-driven lists.

@@quiz
Why give keyboard shortcuts a visible hint in the UI?
* Users discover them without reading documentation
- Shortcuts don't work without hints
- It's required by Tauri
- It makes the app faster
? Show them in tooltips, menus or a command palette.

@@flashcards
View model :: UI state and behaviour without rendering
Command palette :: search-driven list of every app action
Subsequence match :: query characters appear in order in the text
Word-start bonus :: rank matches at word beginnings higher
`aria-activedescendant` :: active option for screen readers
`useSyncExternalStore` :: render a view model's snapshot in React

@@exercise The asset list view model
Write `class AssetListVM` for `type Asset = { tag: string; name: string; site: string; status: "in-use" | "spare" | "repair" }`:

- `constructor(assets: Asset[], pageSize = 5)`; assets are shown sorted by tag with natural number order (`localeCompare(b, undefined, { numeric: true })`)
- `setFilter(text: string)`: case-insensitive match on tag, name or site; resets the cursor to the first row
- `setStatus(status: Asset["status"] | "all")`: also resets the cursor
- `key(k: string): string | null`: `ArrowDown`/`ArrowUp` move by one, `PageDown`/`PageUp` by `pageSize`, `Home`/`End` jump (all clamped); `Enter` returns `"open <tag>"` for the selected row (or `null` if the list is empty); `Escape` clears the filter (and resets the cursor); any key other than Enter returns `null`
- `view(): { rows: { tag: string; label: string; selected: boolean }[]; page: number; pages: number; summary: string }`: the page containing the cursor (pages are 1-based; at least 1 page); `label` is `"<name> (<site>)"`; `summary` is `"<n> assets"` plus `" · <r> in repair"` when the filtered list has repair items
@@starter
type Asset = { tag: string; name: string; site: string; status: "in-use" | "spare" | "repair" };

class AssetListVM {
  constructor(assets: Asset[], pageSize = 5) {}

  setFilter(text: string): void {
    throw new Error("TODO: not done yet");
  }

  setStatus(status: Asset["status"] | "all"): void {
    throw new Error("TODO: not done yet");
  }

  key(k: string): string | null {
    throw new Error("TODO: not done yet");
  }

  view(): { rows: { tag: string; label: string; selected: boolean }[]; page: number; pages: number; summary: string } {
    throw new Error("TODO: not done yet");
  }
}

@@tests
const assets: Asset[] = Array.from({ length: 12 }, (_, i) => ({
  tag: `IT-${i + 1}`,
  name: i % 3 === 0 ? `Laptop ${i + 1}` : `Monitor ${i + 1}`,
  site: i < 6 ? "Tokyo" : "Osaka",
  status: i % 4 === 0 ? "repair" : "in-use",
}));
const vm = new AssetListVM([...assets].reverse(), 5);
const v1 = vm.view();
check([v1.rows.map((r) => r.tag), v1.page, v1.pages, v1.summary], [["IT-1", "IT-2", "IT-3", "IT-4", "IT-5"], 1, 3, "12 assets · 3 in repair"], "first page, natural sort");
vm.key("ArrowDown");
vm.key("ArrowDown");
check(vm.view().rows.find((r) => r.selected)?.tag, "IT-3", "arrows");
vm.key("PageDown");
check([vm.view().page, vm.view().rows.find((r) => r.selected)?.label], [2, "Monitor 8 (Osaka)"], "page down");
vm.key("End");
check([vm.view().page, vm.key("Enter")], [3, "open IT-12"], "end and enter");
vm.key("ArrowDown");
check(vm.key("Enter"), "open IT-12", "clamped at end");
vm.setFilter("laptop");
check([vm.view().rows.map((r) => r.tag), vm.view().summary], [["IT-1", "IT-4", "IT-7", "IT-10"], "4 assets · 1 in repair"], "filter");
vm.setStatus("repair");
check(vm.view().rows.map((r) => r.tag), ["IT-1"], "filter and status");
vm.setFilter("nothing");
check([vm.view().rows, vm.view().pages, vm.key("Enter")], [[], 1, null], "empty");
vm.key("Escape");
check(vm.view().rows.map((r) => r.tag), ["IT-1", "IT-5", "IT-9"], "escape clears filter, keeps status");
@@hint
Keep `all`, `filter`, `status`, `cursor` and `pageSize` fields, and a private `visible(): Asset[]` that filters and sorts. `cursor` is an index into `visible()`; clamp it with `Math.max(0, Math.min(len - 1, c))`. The page is `Math.floor(cursor / pageSize)`.
@@solution
type Asset = { tag: string; name: string; site: string; status: "in-use" | "spare" | "repair" };

class AssetListVM {
  private readonly all: Asset[];
  private readonly pageSize: number;
  private filter = "";
  private status: Asset["status"] | "all" = "all";
  private cursor = 0;

  constructor(assets: Asset[], pageSize = 5) {
    this.all = [...assets].sort((a, b) => a.tag.localeCompare(b.tag, undefined, { numeric: true }));
    this.pageSize = pageSize;
  }

  private visible(): Asset[] {
    const f = this.filter.toLowerCase();
    return this.all.filter(
      (a) =>
        (this.status === "all" || a.status === this.status) &&
        (f === "" || [a.tag, a.name, a.site].some((s) => s.toLowerCase().includes(f))),
    );
  }

  setFilter(text: string): void {
    this.filter = text;
    this.cursor = 0;
  }

  setStatus(status: Asset["status"] | "all"): void {
    this.status = status;
    this.cursor = 0;
  }

  key(k: string): string | null {
    const rows = this.visible();
    const last = Math.max(0, rows.length - 1);
    const moves: Record<string, number> = {
      ArrowDown: this.cursor + 1,
      ArrowUp: this.cursor - 1,
      PageDown: this.cursor + this.pageSize,
      PageUp: this.cursor - this.pageSize,
      Home: 0,
      End: last,
    };
    const target = moves[k];
    if (target !== undefined) this.cursor = Math.max(0, Math.min(last, target));
    else if (k === "Escape") this.setFilter("");
    else if (k === "Enter") {
      const row = rows[this.cursor];
      return row ? `open ${row.tag}` : null;
    }
    return null;
  }

  view(): { rows: { tag: string; label: string; selected: boolean }[]; page: number; pages: number; summary: string } {
    const rows = this.visible();
    const page = Math.floor(this.cursor / this.pageSize);
    const start = page * this.pageSize;
    const repair = rows.filter((a) => a.status === "repair").length;
    return {
      rows: rows.slice(start, start + this.pageSize).map((a, i) => ({ tag: a.tag, label: `${a.name} (${a.site})`, selected: start + i === this.cursor })),
      page: page + 1,
      pages: Math.max(1, Math.ceil(rows.length / this.pageSize)),
      summary: `${rows.length} assets${repair > 0 ? ` · ${repair} in repair` : ""}`,
    };
  }
}

@@exercise A command palette
Write:

- `fuzzyScore(query: string, text: string): number | null`: case-insensitive; an empty query scores 0; match each query character to the **leftmost** matching character after the previous match (null if one is missing); each matched character scores 1, plus 2 if it directly follows the previous matched character, plus 3 if it's at the start of the text or right after a space, `-` or `_`
- `palette(commands: readonly PaletteCommand[], query: string, recent: readonly string[]): string[]`: matching commands (by title) sorted by score (highest first), then by recency (ids in `recent`, most recent first, before others), then by title; each entry is `"<title>"` or `"<title>  <shortcut>"` (two spaces) when the command has a shortcut

with `type PaletteCommand = { id: string; title: string; shortcut?: string }`.
@@starter
type PaletteCommand = { id: string; title: string; shortcut?: string };

function fuzzyScore(query: string, text: string): number | null {
  throw new Error("TODO: not done yet");
}

function palette(commands: readonly PaletteCommand[], query: string, recent: readonly string[]): string[] {
  throw new Error("TODO: not done yet");
}

@@tests
check([fuzzyScore("nd", "New Device"), fuzzyScore("nd", "config reader"), fuzzyScore("xyz", "New Device"), fuzzyScore("", "x")], [8, 2, null, 0], "scores");
check(fuzzyScore("dev", "New Device"), 4 + 3 + 3, "consecutive at word start");
const commands: PaletteCommand[] = [
  { id: "new", title: "New Device", shortcut: "Ctrl+N" },
  { id: "export", title: "Export to CSV" },
  { id: "scan", title: "Scan Network", shortcut: "Ctrl+Shift+S" },
  { id: "settings", title: "Open Settings", shortcut: "Ctrl+," },
  { id: "sync", title: "Sync Now" },
];
check(palette(commands, "s", ["sync"]), ["Sync Now", "Open Settings  Ctrl+,", "Scan Network  Ctrl+Shift+S", "Export to CSV"], "recent breaks ties");
check(palette(commands, "nd", []), ["New Device  Ctrl+N"], "fuzzy filter");
check(palette(commands, "", ["export"]).slice(0, 2), ["Export to CSV", "New Device  Ctrl+N"], "empty query lists all, recent first");
@@hint
Walk the text with an index: for each query char, `const i = lower.indexOf(ch, pos)`; if `i < 0` return null. Bonus checks use `i === prev + 1` and `i === 0 || " -_".includes(text[i - 1])`. For sorting, compute `rank = recent.indexOf(id)` and treat `-1` as `Infinity`.
@@solution
type PaletteCommand = { id: string; title: string; shortcut?: string };

function fuzzyScore(query: string, text: string): number | null {
  const q = query.toLowerCase();
  const t = text.toLowerCase();
  let score = 0;
  let pos = 0;
  let prev = -2;
  for (const ch of q) {
    const i = t.indexOf(ch, pos);
    if (i < 0) return null;
    score += 1;
    if (i === prev + 1) score += 2;
    if (i === 0 || " -_".includes(t[i - 1] ?? "")) score += 3;
    prev = i;
    pos = i + 1;
  }
  return score;
}

function palette(commands: readonly PaletteCommand[], query: string, recent: readonly string[]): string[] {
  const rank = (id: string): number => {
    const i = recent.indexOf(id);
    return i < 0 ? Infinity : i;
  };
  return commands
    .flatMap((c) => {
      const score = fuzzyScore(query, c.title);
      return score === null ? [] : [{ c, score }];
    })
    .sort((a, b) => b.score - a.score || rank(a.c.id) - rank(b.c.id) || a.c.title.localeCompare(b.c.title))
    .map(({ c }) => (c.shortcut ? `${c.title}  ${c.shortcut}` : c.title));
}

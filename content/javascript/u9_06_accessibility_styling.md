@@lesson u9-a11y Accessibility, Keyboard Support and Styling at Scale
@@concepts text-processing, objects
@@text
An app that can't be used with a keyboard or a screen reader is broken for many people: blind and low-vision users, people with motor impairments, and power users who never touch the mouse. In many countries accessibility is also a **legal requirement** for public-sector and, increasingly, commercial software (the European Accessibility Act applies from 2025). The standard is **WCAG 2.2**, level AA.

## Semantic HTML first

The single most effective accessibility technique is using the right element:

```jsx,norun
<div onClick={save}>Save</div>          // not focusable, no role, no Enter/Space: broken
<button onClick={save}>Save</button>    // focusable, announced as a button, works with keys
```

- `<button>` for actions, `<a href>` for navigation, `<label for>` (`htmlFor` in React) for every input, headings (`h1`–`h6`) in order, lists for lists, `<table>` with `<th>` for tabular data, landmarks (`<header>`, `<nav>`, `<main>`, `<footer>`).
- Images need `alt` text (empty `alt=""` for decorative ones).
- **ARIA** fills gaps where HTML has no element: `role="tab"`, `aria-expanded`, `aria-live="polite"` for status messages, `aria-label` for icon-only buttons. The first rule of ARIA: don't use it when a native element does the job.

## Keyboard support

- Everything must work with **Tab**, **Shift+Tab**, **Enter**, **Space**, **Escape** and arrow keys where expected.
- Never remove the **focus outline** without a visible replacement (`:focus-visible` styles).
- Dialogs **trap focus** inside while open, close on Escape and return focus to the button that opened them (the native `<dialog>` element with `showModal()` does most of this).
- Composite widgets (tabs, menus, toolbars, grids) use **roving tabindex**: one item in the group has `tabIndex=0`, the others `-1`; arrow keys move between items, Tab leaves the group. The exercise implements this.

## Colour and motion

- Text contrast at least **4.5:1** (3:1 for large text and UI component borders). The Tkinter lesson in the Python track computes this; browsers' DevTools show it too.
- Don't convey meaning by colour alone: add an icon or text ("✕ Failed", not just red).
- Respect `prefers-reduced-motion` and `prefers-color-scheme`.

## Testing accessibility

- Navigate your app with only the keyboard.
- Try a screen reader: NVDA (free, Windows), VoiceOver (macOS/iOS, built in), Orca (Linux).
- Automated checks: **axe** (browser extension, `@axe-core/playwright`), Lighthouse, `eslint-plugin-jsx-a11y`. They catch about a third of issues; manual testing finds the rest.
- Testing Library's queries (`getByRole("button", { name: "Save" })`) push you towards accessible markup.

## Styling at scale

| Approach | Idea |
|---|---|
| Plain CSS + naming (BEM) | `.ticket-row__title--overdue`; simple, global |
| CSS Modules | `import s from "./Row.module.css"`; class names scoped per file |
| Utility-first (**Tailwind CSS**) | small classes in markup: `class="flex gap-2 p-4 text-sm"` |
| CSS-in-JS (styled-components, Emotion) | styles in JavaScript; runtime cost |
| Component libraries | MUI, Mantine, Radix + shadcn/ui, Fluent UI: accessible components ready to use |

Whatever you choose, use **design tokens** (CSS custom properties: `--color-danger`, `--space-2`) for colours, spacing and fonts, so themes (dark mode, high contrast, brand changes) are one place to edit. Modern CSS covers most layout needs: flexbox, grid, `clamp()`, container queries, `:has()`, nesting and cascade layers.

For anything complex and interactive (comboboxes, date pickers, menus), prefer a tested accessible library (Radix, React Aria, Headless UI) over writing your own.

@@quiz
Why use `<button>` instead of a `<div onClick>`?
* Buttons are focusable, announced correctly and work with Enter and Space automatically
- Divs can't have click handlers
- Buttons are faster
- It looks better
? Native elements bring keyboard and screen reader support for free.

@@quiz
What minimum contrast ratio does WCAG AA require for normal text?
- 2:1
- 3:1
* 4.5:1
- 7:1
? 3:1 applies to large text and UI component boundaries.

@@quiz
In a toolbar using roving tabindex, what does Tab do?
- Moves to the next button in the toolbar
* Leaves the toolbar; arrow keys move between its items
- Nothing
- Closes the toolbar
? Only one item in the group is in the tab order.

@@flashcards
WCAG 2.2 AA :: the usual accessibility standard to meet
Semantic HTML :: using elements for their meaning (button, nav, label)
ARIA :: attributes adding roles and states for assistive tech
`aria-live` :: announces dynamic content changes
Focus trap :: keeping focus inside an open dialog
Roving tabindex :: one tabbable item in a group; arrows move focus
`:focus-visible` :: focus styles for keyboard users
Design tokens :: named values (CSS variables) for colours, spacing, fonts
axe :: automated accessibility checker

@@exercise Accessibility audit
A UI is described as a tree of nodes `{ tag, attrs = {}, text = "", children = [] }`. Write `audit(tree)` returning a list of problems as strings, in document order:

- `img` without an `alt` attribute → `"img missing alt"`
- `button` with no accessible name (no non-blank text in it or its descendants, and no non-blank `aria-label`) → `"button has no name"`
- `input` (except `type="hidden"`) without an `id` that some `label` in the tree points to with `for`, and without `aria-label` → `"input <id or ?> has no label"`
- a `div` or `span` with an `onclick` attribute but no `role` and no `tabindex` → `"<tag> is clickable but not focusable"`
- headings that skip a level going down (e.g. `h2` followed by `h4`) → `"heading jumps from h2 to h4"`
@@starter
function audit(tree) {
  throw new Error("TODO: not done yet");
}

@@tests
const el = (tag, attrs = {}, ...children) => ({ tag, attrs, text: children.filter((c) => typeof c === "string").join(""), children: children.filter((c) => typeof c === "object") });
const page = el("main", {},
  el("h1", {}, "Tickets"),
  el("img", { src: "logo.png" }),
  el("img", { src: "spacer.png", alt: "" }),
  el("h2", {}, "New ticket"),
  el("label", { for: "title" }, "Title"),
  el("input", { id: "title" }),
  el("input", { id: "email" }),
  el("input", { type: "hidden", name: "csrf" }),
  el("input", { "aria-label": "Search" }),
  el("button", {}, el("span", {}, "Save")),
  el("button", { "aria-label": "  " }, el("svg", {})),
  el("h4", {}, "Details"),
  el("div", { onclick: "open()" }, "Open"),
  el("span", { onclick: "x()", role: "button", tabindex: "0" }, "OK"),
);
check(audit(page), ["img missing alt", "input email has no label", "button has no name", "heading jumps from h2 to h4", "div is clickable but not focusable"], "problems found");
check(audit(el("main", {}, el("h1", {}, "Hi"), el("h2", {}, "a"), el("h3", {}, "b"), el("h2", {}, "c"))), [], "going back up is fine");
@@hint
First walk the tree to collect all `label` `for` values. Then walk again in document order (node, then children). For button names write `textOf(node)` = own text plus children's text. For headings keep `lastLevel`; a jump is `level > lastLevel + 1` when `lastLevel` is set.
@@solution
function walk(node, visit) {
  visit(node);
  for (const child of node.children ?? []) walk(child, visit);
}

function textOf(node) {
  return (node.text ?? "") + (node.children ?? []).map(textOf).join("");
}

function audit(tree) {
  const labelled = new Set();
  walk(tree, (n) => {
    if (n.tag === "label" && n.attrs?.for) labelled.add(n.attrs.for);
  });
  const problems = [];
  let lastLevel = null;
  walk(tree, (n) => {
    const a = n.attrs ?? {};
    if (n.tag === "img" && !("alt" in a)) problems.push("img missing alt");
    if (n.tag === "button" && !textOf(n).trim() && !(a["aria-label"] ?? "").trim()) problems.push("button has no name");
    if (n.tag === "input" && a.type !== "hidden" && !labelled.has(a.id) && !(a["aria-label"] ?? "").trim()) {
      problems.push(`input ${a.id ?? "?"} has no label`);
    }
    if ((n.tag === "div" || n.tag === "span") && "onclick" in a && !("role" in a) && !("tabindex" in a)) {
      problems.push(`${n.tag} is clickable but not focusable`);
    }
    const m = /^h([1-6])$/.exec(n.tag);
    if (m) {
      const level = Number(m[1]);
      if (lastLevel !== null && level > lastLevel + 1) problems.push(`heading jumps from h${lastLevel} to h${level}`);
      lastLevel = level;
    }
  });
  return problems;
}

@@exercise Roving tabindex
Write `createRovingGroup(items, { orientation = "horizontal", wrap = true })` for a toolbar or tab list. `items` is a list of `{ id, disabled }`. It returns:

- `tabIndexes()`: an object id → `0` for the current item and `-1` for the others
- `current()`: the current id (initially the first enabled item)
- `key(name)`: handle a key and return the new current id. Horizontal groups use `ArrowRight`/`ArrowLeft`, vertical ones `ArrowDown`/`ArrowUp`; `Home`/`End` go to the first/last enabled item. Disabled items are skipped. With `wrap`, moving past the end goes to the start (and back); without it, the position stays. Other keys change nothing.
- `focus(id)`: make an enabled item current (a click); ignore disabled or unknown ids
@@starter
function createRovingGroup(items, { orientation = "horizontal", wrap = true } = {}) {
  throw new Error("TODO: not done yet");
}

@@tests
const items = [{ id: "bold" }, { id: "italic", disabled: true }, { id: "underline" }, { id: "link" }];
const g = createRovingGroup(items);
check([g.current(), g.tabIndexes()], ["bold", { bold: 0, italic: -1, underline: -1, link: -1 }], "initial");
check([g.key("ArrowRight"), g.key("ArrowRight"), g.key("ArrowRight")], ["underline", "link", "bold"], "skips disabled, wraps");
check([g.key("ArrowLeft"), g.key("End"), g.key("Home"), g.key("ArrowDown")], ["link", "link", "bold", "bold"], "home, end, wrong axis ignored");
g.focus("italic"); g.focus("underline");
check(g.current(), "underline", "focus by click");
const v = createRovingGroup([{ id: "a", disabled: true }, { id: "b" }, { id: "c" }], { orientation: "vertical", wrap: false });
check([v.current(), v.key("ArrowUp"), v.key("ArrowDown"), v.key("ArrowDown")], ["b", "b", "c", "c"], "vertical without wrap");
@@hint
Keep the current index. To move by `step` (+1 or −1), loop up to `items.length` times: `i += step`; if out of range, wrap with modulo (or stop and return the old index without wrap); return the first enabled index. Map keys with `const next = orientation === "horizontal" ? "ArrowRight" : "ArrowDown"`.
@@solution
function createRovingGroup(items, { orientation = "horizontal", wrap = true } = {}) {
  const enabled = (i) => !items[i].disabled;
  let index = items.findIndex((_, i) => enabled(i));

  function move(step) {
    let i = index;
    for (let n = 0; n < items.length; n++) {
      i += step;
      if (i < 0 || i >= items.length) {
        if (!wrap) return index;
        i = (i + items.length) % items.length;
      }
      if (enabled(i)) return i;
    }
    return index;
  }

  const nextKey = orientation === "horizontal" ? "ArrowRight" : "ArrowDown";
  const prevKey = orientation === "horizontal" ? "ArrowLeft" : "ArrowUp";

  return {
    current: () => items[index]?.id,
    tabIndexes: () => Object.fromEntries(items.map((item, i) => [item.id, i === index ? 0 : -1])),
    key(name) {
      if (name === nextKey) index = move(1);
      else if (name === prevKey) index = move(-1);
      else if (name === "Home") index = items.findIndex((_, i) => enabled(i));
      else if (name === "End") index = items.findLastIndex((_, i) => enabled(i));
      return items[index]?.id;
    },
    focus(id) {
      const i = items.findIndex((item) => item.id === id);
      if (i !== -1 && enabled(i)) index = i;
    },
  };
}

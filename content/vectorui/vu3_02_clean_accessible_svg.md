@@lesson vu3-clean-svg Clean, Small and Accessible SVG
@@concepts strings, files
@@lang html
@@text
An SVG exported straight from a design tool is often five times bigger than it needs to be, full of editor metadata, and silent or confusing for screen readers. This lesson covers what optimisers like **SVGO** (and Inkscape's "Optimized SVG" export) remove (the previous geometry lesson's last exercise minifies path data by hand), and how to make icons accessible in both decorative and meaningful roles.

## What to remove or simplify

- Editor metadata: `inkscape:*`/`sodipodi:*` attributes, `<metadata>`, comments, hidden layers, empty groups.
- Unneeded precision: `12.000001` → `12`; 1–2 decimals are plenty on a 24 px grid.
- Redundant attributes: default values (`fill-opacity="1"`), `version`, `xml:space`, inline styles that could be attributes or a shared class.
- Path data: use **relative** commands when shorter, drop spaces where a minus sign or a command letter separates numbers (`M2 3l4-5`), merge paths with the same style, convert simple shapes consistently.
- Transforms: bake them into coordinates when possible.
- Keep: `viewBox` (never remove it, or the icon won't scale), `id`s that are referenced (`url(#grad)`, `href="#sym"`), and `<title>` if the image is meaningful.

SVGO is configurable (`svgo.config.mjs`), so you can keep `viewBox`, prefix ids to avoid collisions when several inline SVGs share a page, and set the float precision.

## Theme-friendly colours

Use `fill="currentColor"` or `stroke="currentColor"` in icons so they take the text colour of their container and follow light and dark themes automatically. Multi-colour icons can use CSS custom properties (`fill: var(--icon-accent)`).

## Accessibility

| Situation | Markup |
|---|---|
| **Decorative** icon next to visible text | `<svg aria-hidden="true" focusable="false">` |
| **Meaningful** standalone image | `<svg role="img" aria-labelledby="t"><title id="t">Upload complete</title>…` |
| `<img src="logo.svg">` | `alt="Company name"` (or `alt=""` if decorative) |
| **Icon-only button** | the `<button>` gets `aria-label="Delete"`; the SVG inside is `aria-hidden` |

- Icon-only controls also need a **visible focus style** and a target of at least **24 × 24 CSS px** (WCAG 2.2, 2.5.8), better 44 × 44 for touch.
- Non-text contrast: icons that convey meaning need **3:1** against their background (WCAG 1.4.11).
- Don't rely on colour alone: pair a red error icon with text.
- A `<title>` also shows as a tooltip in browsers; it's not a replacement for a visible label.

## A clean-up pass, step by step

Exported SVG from design tools is usually correct but heavy. A typical clean-up pass, in order:

1. **Remove editor data**: Inkscape's `sodipodi:` and `inkscape:` attributes, Penpot or Figma IDs, `<metadata>` blocks, and empty `<g>` groups. Inkscape's *Save As → Optimized SVG* or SVGO handles most of this.
2. **Round coordinates** to one or two decimal places. `M12.000001 4.999998` and `M12 5` look identical on screen, and the shorter file is easier to read in a diff.
3. **Merge and simplify paths.** Several shapes with the same fill can often be one path, and boolean operations leave behind extra nodes that a simplify step removes. Check the result visually, because aggressive simplification rounds off corners.
4. **Replace fixed colours with `currentColor`** for single-colour icons, so the icon takes the text colour of its parent and works in light and dark themes without separate files.
5. **Keep the `viewBox`, drop fixed `width` and `height`** (or set them in CSS), so the icon scales cleanly.
6. **Check the result** at the sizes you actually use (16, 20, 24 px) and in both themes.

Keep the original, editable source file (with layers and guides) next to the optimized one. Clean-up is a one-way step: you don't want to edit minified paths by hand.

## Decorative or meaningful?

Every SVG on a page is one of two kinds, and each needs different markup:

- **Decorative** (an icon next to a text label, a background flourish): hide it from assistive technology with `aria-hidden="true"` and make sure it can't receive focus (`focusable="false"` for old Edge, and no `tabindex`).
- **Meaningful** (an icon-only button, a chart, a logo that is the only link to the home page): give it a name. For an inline SVG, use `role="img"` with `aria-label`, or a `<title>` referenced by `aria-labelledby`. For an icon-only button, it's often simpler to put the label on the button itself (`<button aria-label="Close">`) and keep the SVG decorative inside it.

For charts and diagrams, a short name isn't enough: add a longer description (`<desc>` or nearby text) or a data table, so someone who can't see the shapes gets the same information.

@@quiz
Which attribute must you keep for an icon to scale correctly?
- width
* viewBox
- xmlns:inkscape
- version
? It defines the coordinate system.

@@quiz
How should a decorative icon next to the text "Settings" be marked up?
* `aria-hidden="true"` on the SVG
- `role="img"` with a title "gear"
- `alt="icon"`
- `tabindex="0"`
? The text already says it.

@@quiz
An icon-only button deletes an item. Where does the accessible name go?
- In the SVG's fill
* `aria-label="Delete"` on the button
- In a CSS comment
- Nowhere; icons are self-explanatory
? The button is what gets focused.

@@quiz
Why use `fill="currentColor"` in icons?
- It's smaller
* The icon inherits the text colour and follows themes
- It enables animation
- Browsers require it
? One icon works on light and dark.

@@quiz
What minimum contrast do meaningful icons need against their background?
- 1.5:1
* 3:1
- 4.5:1
- 7:1
? Non-text contrast, WCAG 1.4.11.

@@flashcards
SVGO :: SVG optimiser
remove :: metadata, editor attributes, comments
keep :: viewBox, referenced ids
precision :: 1–2 decimals
relative commands :: shorter path data
prefix ids :: avoid collisions inline
currentColor :: inherit text colour
decorative SVG :: aria-hidden="true"
meaningful SVG :: role="img" + title
icon-only button :: aria-label on button
target size :: at least 24 × 24 px
non-text contrast :: 3:1
focusable="false" :: old IE/Edge tab stop fix

@@drill
Which attribute hides a decorative SVG from screen readers?
= aria-hidden="true"
= aria-hidden
? The visible text carries the meaning.

@@drill
Which colour keyword makes an icon use the surrounding text colour?
= currentColor
= currentcolor
? Case-insensitive in CSS.

@@exercise Accessible icon buttons
Build a toolbar with three icon-only buttons (Bold, Italic, Delete) and one icon with visible text (Settings). Each icon-only `<button>` needs an `aria-label`, its SVG must be `aria-hidden="true"` and use `currentColor`, buttons must be at least 44 px square with a visible `:focus-visible` outline, and the decorative Settings icon must be hidden from screen readers too. Add one meaningful standalone status SVG with `role="img"` and a `<title>`.
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Icon toolbar</title>
  <style>
    body { font-family: system-ui, sans-serif; padding: 32px; background: #f8fafc; color: #0f172a; }
    .toolbar { display: flex; gap: 8px; align-items: center; }
  </style>
</head>
<body>
  <div class="toolbar">
    <!-- buttons here -->
  </div>
</body>
</html>
@@requires
aria-label="Bold"
aria-label="Delete"
aria-hidden="true"
currentColor
:focus-visible
min-width: 44px
role="img"
<title
@@hint
`<button aria-label="Bold"><svg aria-hidden="true" focusable="false" viewBox="0 0 24 24" width="20" height="20"><path d="M7 4h6a4 4 0 0 1 0 8H7zM7 12h7a4 4 0 0 1 0 8H7z" fill="none" stroke="currentColor" stroke-width="2"/></svg></button>`. Style buttons with `min-width: 44px; min-height: 44px;` and `button:focus-visible { outline: 3px solid #2563eb; outline-offset: 2px; }`.
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Icon toolbar</title>
  <style>
    body { font-family: system-ui, sans-serif; padding: 32px; background: #f8fafc; color: #0f172a; }
    .toolbar { display: flex; gap: 8px; align-items: center; }
    button {
      min-width: 44px; min-height: 44px;
      display: inline-flex; align-items: center; justify-content: center; gap: 6px;
      border: 1px solid #cbd5e1; border-radius: 10px; background: #fff; color: #334155;
      font: inherit; cursor: pointer;
    }
    button:hover { background: #eef2ff; color: #3730a3; }
    button:focus-visible { outline: 3px solid #2563eb; outline-offset: 2px; }
    button.danger { color: #b91c1c; }
    .status { margin-top: 24px; display: flex; gap: 8px; align-items: center; color: #15803d; }
  </style>
</head>
<body>
  <div class="toolbar">
    <button aria-label="Bold">
      <svg aria-hidden="true" focusable="false" viewBox="0 0 24 24" width="20" height="20"><path d="M7 4h6a4 4 0 0 1 0 8H7zM7 12h7a4 4 0 0 1 0 8H7z" fill="none" stroke="currentColor" stroke-width="2" stroke-linejoin="round"/></svg>
    </button>
    <button aria-label="Italic">
      <svg aria-hidden="true" focusable="false" viewBox="0 0 24 24" width="20" height="20"><path d="M10 4h8M6 20h8M14 4 10 20" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"/></svg>
    </button>
    <button class="danger" aria-label="Delete">
      <svg aria-hidden="true" focusable="false" viewBox="0 0 24 24" width="20" height="20"><path d="M4 7h16M9 7V4h6v3M6 7l1 13h10l1-13" fill="none" stroke="currentColor" stroke-width="2" stroke-linejoin="round"/></svg>
    </button>
    <button>
      <svg aria-hidden="true" focusable="false" viewBox="0 0 24 24" width="20" height="20"><circle cx="12" cy="12" r="3" fill="none" stroke="currentColor" stroke-width="2"/><path d="M12 2v3M12 19v3M2 12h3M19 12h3M4.9 4.9l2.1 2.1M17 17l2.1 2.1M4.9 19.1 7 17M17 7l2.1-2.1" stroke="currentColor" stroke-width="2" stroke-linecap="round"/></svg>
      Settings
    </button>
  </div>
  <p class="status">
    <svg role="img" aria-labelledby="saved-title" viewBox="0 0 24 24" width="24" height="24"><title id="saved-title">Saved</title><circle cx="12" cy="12" r="10" fill="currentColor"/><path d="m7 12 3.5 3.5L17 9" fill="none" stroke="#fff" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"/></svg>
    All changes are stored on this device.
  </p>
</body>
</html>

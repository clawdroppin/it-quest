@@lesson vu3-inkscape Inkscape: Paths, Icons and Clean SVG Export
@@concepts hello, files
@@lang html
@@text
Inkscape is the precision tool: icons that are crisp at 16 px, logos with perfect curves, illustrations for empty states and onboarding screens. This lesson covers the tools you'll use daily, a method for drawing a consistent icon set, and exporting clean SVG that apps (and Penpot) can use.

## The interface (Inkscape 1.x)

| Area | Use |
|---|---|
| **Toolbox** (left) | Selector (S), Node (N), shapes (R rectangle, E ellipse, * star/polygon), Pen/Bézier (B), Pencil (P), Calligraphy, Text (T), Gradient (G), Dropper (D), Measure (M) |
| **Tool controls bar** (top) | settings of the current tool: corner radius, number of star points, pen mode... |
| **Commands bar** | file, undo, zoom, docks |
| **Docked dialogs** (right) | **Fill and Stroke** (Ctrl+Shift+F), **Layers and Objects** (Ctrl+Shift+L), **Align and Distribute** (Ctrl+Shift+A), **Transform** (Ctrl+Shift+M), **XML Editor** (Ctrl+Shift+X), **Export** (Ctrl+Shift+E), **Swatches**, **Symbols** |
| **Snap controls** | top-right: snap to grid, nodes, paths, bounding boxes, centres |

Navigation: scroll and Ctrl+scroll to zoom, **5** fit page, **3** zoom to selection, **1** 1:1, middle-drag or Space+drag to pan.

## Paths and nodes

Everything complex is a **path**. With the **Node tool (N)**:

- click a path to see its nodes; drag nodes and the **handles** of curves;
- node types: **cusp** (sharp corner), **smooth**, **symmetric**, **auto-smooth** (buttons in the tool bar);
- double-click a segment to add a node, Delete to remove (Inkscape keeps the shape smooth), Ctrl+Delete removes without smoothing;
- **Path → Object to Path** (Shift+Ctrl+C) turns shapes and text into editable paths;
- **Path → Stroke to Path** (Ctrl+Alt+C) turns an outline into a filled shape: needed before boolean operations on strokes.

The **Pen tool (B)**: click for straight segments, click-drag for curves, Enter to finish, click the first node to close. Hold Ctrl to snap angles to 15°.

## Boolean (path) operations

Combine shapes like building blocks; select two or more and use the **Path** menu:

| Operation | Shortcut | Result |
|---|---|---|
| **Union** | Ctrl++ | merge into one shape |
| **Difference** | Ctrl+- | cut the top shape out of the bottom one |
| **Intersection** | Ctrl+* | keep only the overlap |
| **Exclusion** | Ctrl+^ | keep everything except the overlap |
| **Division** | Ctrl+/ | split the bottom shape along the top one |
| **Cut Path** | Ctrl+Alt+/ | cut the bottom path's stroke |
| **Combine / Break Apart** | Ctrl+K / Shift+Ctrl+K | one path with several parts / separate them |

Most icons are a few rectangles and circles combined this way: a bell is a union of a rounded rectangle, a circle and a small ellipse; a padlock is a rounded rectangle plus a stroked arc converted to a path. The **Shape Builder** tool (X, Inkscape 1.3+) does the same by clicking and dragging over regions.

## Drawing a consistent icon set

1. **Document Properties** (Ctrl+Shift+D): 24 × 24 px page, **scale 1 user unit = 1 px**, and a **grid** with 1 px spacing and major lines every 4.
2. Turn on **snapping** to the grid and nodes. Keep straight edges on whole pixels (filled shapes) or half pixels (odd-width strokes).
3. Work with **strokes** of one width (2 px), round caps and joins (Fill and Stroke → Stroke style), 2 px padding inside the page.
4. Keep a **template**: a layer with keylines (a 20 × 20 square, an 18 px circle, 16 × 20 rectangles) so every icon has the same visual weight.
5. Draw each icon on its own page (Inkscape 1.2+ supports **multiple pages**) or in a **Symbols** library.
6. Check at real size (View → Zoom 1:1, or Ctrl+Shift+D and a second window at 100%), and in a 16 px version.
7. Before export: **Stroke to Path** only if the app needs filled shapes (some icon fonts do); otherwise keep strokes so the width can be themed.

## Clean export

- **File → Save As → Optimized SVG** (Scour): removes Inkscape metadata, rounds coordinates, shortens IDs. Or **Plain SVG** for a simpler but larger file.
- In the **Export dialog** (Ctrl+Shift+E): export the page, a selection or each page; formats PNG (set DPI or pixel size: export icons at 1×, 2×, 3×), SVG, PDF, WebP; batch-export every object or page.
- Replace hard-coded colours with **`currentColor`** in the XML Editor (or in a text editor) so the icon follows the theme. The exercise does this.
- **SVGO** (`npx svgo icon.svg`) is the standard command-line optimiser for whole folders.

## Other powerful features

- **Clones** (Alt+D): live copies that update with the original; tiled clones for patterns.
- **Path Effects** (Path → Path Effects): Corners (rounded corners on any path), Offset, Pattern Along Path, Taper Stroke, Mirror Symmetry, Boolean operation (non-destructive).
- **Trace Bitmap** (Shift+Alt+B): convert a sketch or a raster logo into vector paths.
- **Text on Path**, **Flow text into shape**.
- **Filters** (blur, drop shadow, textures): use sparingly for UI assets, many apps don't render SVG filters fast.
- **Extensions**: generate barcodes, grids, gears, export to HTML5 canvas, and many more.
- **Command line**: `inkscape icon.svg --export-type=png --export-width=64` for scripted exports (also in CI).

@@quiz
Which operation cuts the top shape out of the bottom one?
- Union
* Difference (Ctrl+-)
- Intersection
- Combine
? Union merges; Intersection keeps the overlap.

@@quiz
You want to use a boolean operation on a stroked circle. What must you do first?
* Path → Stroke to Path, so the outline becomes a filled shape
- Group it
- Lock its layer
- Export it as PNG
? Boolean operations work on filled areas, not stroke styles.

@@quiz
Why save the final icons as Optimized SVG?
- It adds Inkscape layers for later editing
* It strips editor metadata and rounds coordinates for smaller, cleaner files
- It converts them to PNG
- It's required to open them in Penpot
? Keep the Inkscape SVG as the editable source.

@@quiz
Why keep straight edges on whole or half pixels?
- It makes the file smaller
* So lines render crisp instead of blurry at small sizes
- Inkscape can't draw elsewhere
- It's required by SVG
? A 2 px stroke centred on a whole pixel covers two full pixels.

@@quiz
What does Path → Object to Path do to a text object?
* Converts the letters into plain path outlines, so the font is no longer needed (and the text is no longer editable)
- Makes the text follow a path
- Exports the text as a font file
- Adds an outline stroke to the text
? Do it on a copy for logos and icons that must render the same everywhere.

@@flashcards
Node tool (N) :: edit path nodes and handles
cusp / smooth / symmetric :: node types
Object to Path :: shapes and text become editable paths
Stroke to Path :: outline becomes a filled shape
Union / Difference / Intersection :: merge / cut out / keep the overlap
Combine / Break Apart :: one path with parts / separate paths
Shape Builder (X) :: build shapes by clicking regions
Document Properties :: page size, scale, grid
pixel snapping :: crisp edges on whole or half pixels
Optimized SVG :: cleaned export via Scour
SVGO :: command-line SVG optimiser
clones (Alt+D) :: live copies of an object
Path Effects :: non-destructive corners, offset, patterns
Trace Bitmap :: raster to vector
inkscape --export-type :: scripted exports

@@exercise Theme-ready icons
The starter has two icons exported from Inkscape with hard-coded colours and editor clutter. Clean them up: remove the `inkscape:label` and `sodipodi` attributes, change every hard-coded stroke colour to `currentColor`, and add `aria-hidden="true"` (they sit next to text). Then style `.icon` with `width: 24px; height: 24px` and make `.dark` switch the colour by setting `color` on the container: both icons must change colour with it.
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Icons</title>
  <style>
    body { font-family: system-ui, sans-serif; padding: 24px; }
    .bar { display: flex; gap: 16px; padding: 12px 16px; border-radius: 12px; color: #1e293b; background: #f1f5f9; }
    .bar span { display: inline-flex; align-items: center; gap: 6px; }
    /* Your CSS here */
  </style>
</head>
<body>
  <div class="bar">
    <span><svg class="icon" viewBox="0 0 24 24" fill="none" stroke="#1e293b" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" inkscape:label="Layer 1" sodipodi:docname="bell.svg"><path d="M6 16V11a6 6 0 0 1 12 0v5l2 2H4z"/><path d="M10 21h4"/></svg> Alerts</span>
    <span><svg class="icon" viewBox="0 0 24 24" fill="none" stroke="#1e293b" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" inkscape:label="Layer 1" sodipodi:docname="lock.svg"><rect x="5" y="11" width="14" height="10" rx="2"/><path d="M8 11V8a4 4 0 0 1 8 0v3"/></svg> Privacy</span>
  </div>
  <div class="bar dark">
    <span><svg class="icon" viewBox="0 0 24 24" fill="none" stroke="#1e293b" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M6 16V11a6 6 0 0 1 12 0v5l2 2H4z"/><path d="M10 21h4"/></svg> Alerts</span>
  </div>
</body>
</html>
@@requires
stroke="currentColor"
aria-hidden="true"
.icon
width: 24px
height: 24px
.dark
color:
@@hint
In each `<svg>`, replace `stroke="#1e293b"` with `stroke="currentColor"`, delete the `inkscape:label=...` and `sodipodi:docname=...` attributes, and add `aria-hidden="true"`. Then `.bar.dark { color: #e2e8f0; background: #1e293b; }`.
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Icons</title>
  <style>
    body { font-family: system-ui, sans-serif; padding: 24px; }
    .bar { display: flex; gap: 16px; padding: 12px 16px; border-radius: 12px; color: #1e293b; background: #f1f5f9; }
    .bar span { display: inline-flex; align-items: center; gap: 6px; }
    .icon { width: 24px; height: 24px; flex-shrink: 0; }
    .bar.dark { color: #e2e8f0; background: #1e293b; margin-top: 12px; }
  </style>
</head>
<body>
  <div class="bar">
    <span><svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M6 16V11a6 6 0 0 1 12 0v5l2 2H4z"/><path d="M10 21h4"/></svg> Alerts</span>
    <span><svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><rect x="5" y="11" width="14" height="10" rx="2"/><path d="M8 11V8a4 4 0 0 1 8 0v3"/></svg> Privacy</span>
  </div>
  <div class="bar dark">
    <span><svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M6 16V11a6 6 0 0 1 12 0v5l2 2H4z"/><path d="M10 21h4"/></svg> Alerts</span>
  </div>
</body>
</html>

@@exercise An icon sprite
Put a set of icons in one hidden SVG **sprite** so each is defined once: a `<svg>` with `style="display:none"` (or `hidden`) containing `<symbol>` elements with ids `i-home`, `i-search` and `i-heart`, each with `viewBox="0 0 24 24"`. Use them in a navigation bar with `<svg class="icon"><use href="#i-home"></use></svg>` (and so on). Style `.icon` with `stroke: currentColor` and `fill: none`, and make the nav links change colour on `:hover` with a `transition`.
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Sprite</title>
  <style>
    body { font-family: system-ui, sans-serif; padding: 24px; }
    nav { display: flex; gap: 8px; }
    nav a { display: inline-flex; align-items: center; gap: 6px; padding: 8px 12px; border-radius: 10px; color: #475569; text-decoration: none; }
    /* Your CSS here */
  </style>
</head>
<body>
  <!-- The sprite here -->
  <nav>
    <a href="#">Home</a>
    <a href="#">Search</a>
    <a href="#">Favourites</a>
  </nav>
</body>
</html>
@@requires
<symbol id="i-home" viewBox="0 0 24 24"
<symbol id="i-search" viewBox="0 0 24 24"
<symbol id="i-heart" viewBox="0 0 24 24"
<use href="#i-home">
<use href="#i-search">
<use href="#i-heart">
stroke: currentColor
fill: none
nav a:hover
transition:
@@hint
Home: `<path d="M3 11l9-7 9 7v9a1 1 0 0 1-1 1h-5v-6h-6v6H4a1 1 0 0 1-1-1z"/>`; search: `<circle cx="11" cy="11" r="7"/><path d="M20 20l-4-4"/>`; heart: `<path d="M12 20s-7-4.5-7-10a4 4 0 0 1 7-2.5A4 4 0 0 1 19 10c0 5.5-7 10-7 10z"/>`.
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Sprite</title>
  <style>
    body { font-family: system-ui, sans-serif; padding: 24px; }
    nav { display: flex; gap: 8px; }
    nav a { display: inline-flex; align-items: center; gap: 6px; padding: 8px 12px; border-radius: 10px; color: #475569; text-decoration: none;
            transition: color .15s ease-out, background-color .15s ease-out; }
    nav a:hover { color: #4f46e5; background: #eef2ff; }
    .icon { width: 20px; height: 20px; stroke: currentColor; fill: none; stroke-width: 2; stroke-linecap: round; stroke-linejoin: round; }
  </style>
</head>
<body>
  <svg xmlns="http://www.w3.org/2000/svg" style="display:none">
    <symbol id="i-home" viewBox="0 0 24 24"><path d="M3 11l9-7 9 7v9a1 1 0 0 1-1 1h-5v-6h-6v6H4a1 1 0 0 1-1-1z"/></symbol>
    <symbol id="i-search" viewBox="0 0 24 24"><circle cx="11" cy="11" r="7"/><path d="M20 20l-4-4"/></symbol>
    <symbol id="i-heart" viewBox="0 0 24 24"><path d="M12 20s-7-4.5-7-10a4 4 0 0 1 7-2.5A4 4 0 0 1 19 10c0 5.5-7 10-7 10z"/></symbol>
  </svg>
  <nav>
    <a href="#"><svg class="icon" aria-hidden="true"><use href="#i-home"></use></svg>Home</a>
    <a href="#"><svg class="icon" aria-hidden="true"><use href="#i-search"></use></svg>Search</a>
    <a href="#"><svg class="icon" aria-hidden="true"><use href="#i-heart"></use></svg>Favourites</a>
  </nav>
</body>
</html>

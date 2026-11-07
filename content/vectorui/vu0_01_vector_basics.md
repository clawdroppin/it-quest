@@lesson vu0-basics Vector Design Basics: SVG, the Tools and Your Setup
@@concepts hello, files
@@lang html
@@text
Every crisp icon, button and logo in a modern app is a **vector**: shapes described with maths (points, curves, fills, strokes) instead of a grid of pixels. Vectors scale to any size without blurring, recolour instantly for dark mode, and are tiny files. This course teaches the two best free tools for designing them, **Penpot** (UI and UX design, the open-source alternative to Figma) and **Inkscape** (vector illustration, the alternative to Illustrator), and how to bring their work into real apps: HTML/CSS, Tauri, and the Rust toolkits from the Rust UI course.

The exercises are small SVG and CSS pages: they preview right in the app, and they teach exactly what Penpot and Inkscape write when you export.

## Raster versus vector

| | Raster (PNG, JPEG, WebP) | Vector (SVG, PDF, AI) |
|---|---|---|
| Made of | pixels | paths, shapes, text |
| Scaling | blurs or pixelates when enlarged | perfect at any size |
| Best for | photos, painting, textures | icons, logos, UI, illustrations, diagrams |
| Editing | GIMP, Krita | Inkscape, Penpot |
| Recolour | repaint | change one attribute |

Apps need both: vectors for the interface, rasters for photos (and exported PNG fallbacks of icons where SVG isn't supported).

## The tools

**Penpot** (penpot.app) is a UI design and prototyping tool built on web standards: its designs *are* SVG, and its layouts *are* CSS flexbox and grid, so what you design is what developers build. It has components with variants, design tokens and themes, prototyping with interactions, comments for teams, and a code **Inspect** tab that gives CSS, SVG and HTML.

- **Use it in the browser** at design.penpot.app (free account), or
- **self-host it** on your own computer or server with Docker, so your files never leave your machine:

```text
curl -o docker-compose.yaml https://raw.githubusercontent.com/penpot/penpot/main/docker/images/docker-compose.yaml
docker compose -p penpot -f docker-compose.yaml up -d
# open http://localhost:9001 and create the first account
```

  (Docker Desktop on Windows/macOS, Docker Engine on Linux. The DevOps track's Docker module explains what this does. Update with `docker compose pull` then `up -d`; back up the `penpot_postgres_v15` and `penpot_assets` volumes.)

**Inkscape** (inkscape.org) is a desktop vector editor for Windows, macOS and Linux (also on Flathub). Its native format *is* SVG. It's the tool for precise icons, logos, illustrations, complex path work, and print (PDF, EPS).

| Task | Use |
|---|---|
| App screens, layouts, flows, clickable prototypes | Penpot |
| A component library and design system (buttons, inputs, cards, tokens) | Penpot |
| Icons, logos, illustrations, complex shapes | Inkscape (or Penpot for simple icons) |
| Print, PDF, laser cutting, plotting | Inkscape |
| Handing designs to developers | Penpot Inspect, or clean SVG from Inkscape |

The usual workflow: draw icons and illustrations in Inkscape → import them into a Penpot library → design screens and components in Penpot → export SVG/CSS → build the app.

## Anatomy of an SVG

```text
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 120 120" width="120" height="120">
  <rect x="10" y="10" width="100" height="100" rx="24" fill="#6366f1"/>
  <circle cx="60" cy="52" r="18" fill="#fff"/>
  <path d="M30 100 Q60 70 90 100" stroke="#fff" stroke-width="8" fill="none" stroke-linecap="round"/>
</svg>
```

- `viewBox="minX minY width height"` defines the drawing's own coordinate system; `width`/`height` are its displayed size. The same drawing shows sharply at 16 or 512 px.
- Basic shapes: `rect` (with `rx` for rounded corners), `circle`, `ellipse`, `line`, `polyline`, `polygon`, and the universal **`path`**.
- **Fill** colours the inside; **stroke** draws the outline (`stroke-width`, `stroke-linecap`, `stroke-linejoin`, `stroke-dasharray`).
- `<g>` groups elements (with a shared `transform` or style); `<defs>` holds reusable things: gradients, clip paths, masks, filters, symbols.
- Later elements are drawn **on top** of earlier ones.

## Path commands

| Command | Meaning |
|---|---|
| `M x y` | move to (start a new shape) |
| `L x y` / `H x` / `V y` | straight lines |
| `C x1 y1 x2 y2 x y` | cubic Bézier curve with two control points |
| `Q x1 y1 x y` | quadratic Bézier with one control point |
| `A rx ry rot large sweep x y` | elliptical arc |
| `Z` | close the shape |

Lower-case letters are **relative** to the current point. You rarely type paths by hand: the Pen/Bézier tools in both apps write them for you. But reading them helps you clean exports and animate them.

## A tidy setup

- Inkscape: **Edit → Preferences → Input/Output → SVG output** → set numeric precision to 3–5 and enable path data optimisation; save final files as **Optimized SVG** (or Plain SVG) to strip editor metadata.
- Learn the shortcuts you use most (both apps show them in menus; Inkscape: **Edit → Preferences → Interface → Keyboard**; Penpot: `?` shows the list).
- Keep a folder structure: `design/` (source `.svg`, `.penpot` exports), `export/` (optimized SVG, PNG), and version it with Git.

@@quiz
Why are vectors better than PNGs for app icons?
* They stay sharp at any size, recolour easily (dark mode) and are small files
- They load photos faster
- Every browser renders vectors faster than pixels
- PNGs can't have transparency
? Use raster formats for photos and textures.

@@quiz
What does an SVG's viewBox do?
- Sets the background colour
* Defines the drawing's internal coordinate system, so it can be shown at any width and height
- Crops the image permanently
- Adds a border
? `viewBox="0 0 24 24"` is the classic icon grid.

@@quiz
Which tool is designed for app screens, components, design tokens and clickable prototypes?
- Inkscape
* Penpot
- GIMP
- Synfig
? Inkscape is better for detailed icons, logos and illustrations.

@@quiz
How can you run Penpot so your designs never leave your computer?
- You can't; it's cloud only
* Self-host it with Docker Compose and open http://localhost:9001
- Install it from the Microsoft Store only
- Use it offline in Inkscape
? Penpot is open source (MPL 2.0).

@@quiz
In SVG, which element is drawn on top?
- The first one
* The last one in the document
- The biggest one
- The one with the highest id
? Order in the file is drawing order (unless you use z-index tricks in HTML).

@@drill
Which SVG path command draws a straight line to a point?
= L
? M moves, C draws a curve, Z closes the path.

@@flashcards
vector :: shapes from maths, scale without blur
raster :: a grid of pixels
Penpot :: open-source UI design and prototyping, SVG/CSS native
Inkscape :: open-source vector editor, SVG native
self-hosted Penpot :: docker compose up, localhost:9001
viewBox :: the internal coordinate system of an SVG
fill / stroke :: inside colour / outline
rx :: rounded corners on rect
<g> :: group of elements
<defs> :: reusable gradients, masks, symbols, filters
M L H V C Q A Z :: SVG path commands
relative commands :: lower-case letters
Optimized SVG :: Inkscape export without editor metadata

@@exercise Draw an app badge in SVG
Draw a 120 × 120 app badge inside the `<body>`: an `<svg>` with `viewBox="0 0 120 120"`, a rounded `<rect>` background (use `rx`) filled `#6366f1`, a white `<circle>`, and a white curved `<path>` smile drawn with a stroke and `stroke-linecap="round"` and `fill="none"`. Give the SVG `role="img"` and an `aria-label`.
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Badge</title>
  <style>
    body { font-family: system-ui, sans-serif; display: grid; place-items: center; min-height: 90vh; background: #f8fafc; }
  </style>
</head>
<body>
  <!-- Your SVG here -->
</body>
</html>
@@requires
<svg
viewBox="0 0 120 120"
<rect
rx=
fill="#6366f1"
<circle
<path
stroke-linecap="round"
fill="none"
role="img"
aria-label=
@@hint
`<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 120 120" width="120" height="120" role="img" aria-label="Happy app">`, then `<rect x="10" y="10" width="100" height="100" rx="24" fill="#6366f1"/>`, `<circle cx="60" cy="50" r="16" fill="#fff"/>`, `<path d="M36 82 Q60 104 84 82" stroke="#fff" stroke-width="8" stroke-linecap="round" fill="none"/>`.
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Badge</title>
  <style>
    body { font-family: system-ui, sans-serif; display: grid; place-items: center; min-height: 90vh; background: #f8fafc; }
  </style>
</head>
<body>
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 120 120" width="120" height="120" role="img" aria-label="Happy app">
    <rect x="10" y="10" width="100" height="100" rx="24" fill="#6366f1"/>
    <circle cx="60" cy="50" r="16" fill="#fff"/>
    <path d="M36 82 Q60 104 84 82" stroke="#fff" stroke-width="8" stroke-linecap="round" fill="none"/>
  </svg>
</body>
</html>

@@exercise Gradients and reuse with defs
Make the badge glossy: inside `<defs>`, define a `<linearGradient id="bg">` with two `<stop>` elements (offset `0` and `1`), and use it as the rectangle's fill with `fill="url(#bg)"`. Then define the smile once as a `<symbol id="smile">` and show it twice at different sizes with `<use href="#smile">`.
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Gradient badge</title>
  <style>
    body { font-family: system-ui, sans-serif; display: flex; gap: 24px; align-items: center; padding: 32px; background: #0f172a; }
  </style>
</head>
<body>
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 120 120" width="120" height="120" role="img" aria-label="Glossy badge">
    <!-- defs with the gradient here -->
    <rect x="10" y="10" width="100" height="100" rx="24" fill="#6366f1"/>
  </svg>
  <!-- a second, smaller svg that reuses the symbol -->
</body>
</html>
@@requires
<defs>
<linearGradient id="bg"
<stop offset="0"
<stop offset="1"
fill="url(#bg)"
<symbol id="smile"
<use href="#smile"
@@hint
`<defs><linearGradient id="bg" x1="0" y1="0" x2="1" y2="1"><stop offset="0" stop-color="#818cf8"/><stop offset="1" stop-color="#4f46e5"/></linearGradient><symbol id="smile" viewBox="0 0 48 24"><path d="M4 4 Q24 28 44 4" stroke="#fff" stroke-width="6" stroke-linecap="round" fill="none"/></symbol></defs>`, then `<use href="#smile" x="36" y="64" width="48" height="24"/>`. A `<use>` in another SVG on the same page can point at the same symbol.
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Gradient badge</title>
  <style>
    body { font-family: system-ui, sans-serif; display: flex; gap: 24px; align-items: center; padding: 32px; background: #0f172a; }
  </style>
</head>
<body>
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 120 120" width="120" height="120" role="img" aria-label="Glossy badge">
    <defs>
      <linearGradient id="bg" x1="0" y1="0" x2="1" y2="1">
        <stop offset="0" stop-color="#818cf8"/>
        <stop offset="1" stop-color="#4f46e5"/>
      </linearGradient>
      <symbol id="smile" viewBox="0 0 48 24">
        <path d="M4 4 Q24 28 44 4" stroke="#fff" stroke-width="6" stroke-linecap="round" fill="none"/>
      </symbol>
    </defs>
    <rect x="10" y="10" width="100" height="100" rx="24" fill="url(#bg)"/>
    <circle cx="60" cy="46" r="14" fill="#fff"/>
    <use href="#smile" x="36" y="68" width="48" height="24"/>
  </svg>
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 24" width="48" height="24" role="img" aria-label="Small smile">
    <use href="#smile" width="48" height="24"/>
  </svg>
</body>
</html>

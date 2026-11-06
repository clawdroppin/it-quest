@@lesson n7-performance Web Performance: Core Web Vitals, Images, Fonts and Loading
@@text
A slow page loses users: on mobile, many leave if a page takes more than about 3 seconds. Performance is also a search-ranking factor and an accessibility issue (slow devices and connections are common). This lesson covers what to measure and the fixes that matter most for HTML and CSS.

## Core Web Vitals

Google's three user-centred metrics, measured on real visitors (the 75th percentile):

| Metric | Measures | Good |
|---|---|---|
| **LCP** (Largest Contentful Paint) | when the main content (biggest image or text block) appears | ≤ 2.5 s |
| **INP** (Interaction to Next Paint) | how quickly the page responds to clicks and key presses | ≤ 200 ms |
| **CLS** (Cumulative Layout Shift) | how much content jumps around while loading | ≤ 0.1 |

Measure with: Chrome dev tools **Lighthouse** and **Performance** panels (lab data), **PageSpeed Insights** (lab plus real-user data from Chrome users), and the `web-vitals` JS library for your own analytics.

## How a page loads (the critical path)

1. The browser downloads the HTML and parses it top to bottom.
2. A `<link rel="stylesheet">` **blocks rendering** until it's downloaded: the browser won't paint unstyled content.
3. A plain `<script src>` **blocks parsing** until it's downloaded **and** run.
4. Images, fonts and other resources load in parallel; when the main one arrives, that's LCP.

So: keep CSS small, keep scripts out of the way, and make the LCP resource start downloading early.

## Scripts: defer and async

```html
<script src="app.js" defer></script>   <!-- download in parallel, run after parsing, in order -->
<script src="stats.js" async></script> <!-- download in parallel, run as soon as ready, any order -->
<script type="module" src="main.js"></script>  <!-- modules are deferred by default -->
```

`defer` is right for almost all your own scripts. `async` suits independent third-party scripts (analytics). Third-party scripts (chat widgets, trackers, A/B testing) are the most common cause of bad INP: audit them.

## Images: the biggest win

Images are usually most of a page's bytes and often the LCP element.

```html
<img src="team-800.avif"
     srcset="team-400.avif 400w, team-800.avif 800w, team-1600.avif 1600w"
     sizes="(min-width: 60rem) 50vw, 100vw"
     width="1600" height="900"
     alt="The help desk team in the Osaka office"
     fetchpriority="high">

<img src="chart.webp" width="600" height="400" alt="..." loading="lazy" decoding="async">
```

- **Modern formats**: AVIF and WebP are 30–50% smaller than JPEG at the same quality. Use `<picture>` with a JPEG fallback if you must support very old browsers.
- **Responsive sizes** (`srcset` + `sizes`, module 1): phones download the small file.
- **`width` and `height` attributes**: the browser reserves the right space before the image loads, preventing layout shift (CLS). Keep them even when CSS resizes the image; set `height: auto` in CSS.
- **`loading="lazy"`** for images below the fold. **Never** lazy-load the LCP image; give it `fetchpriority="high"` instead.
- Compress: tools like Squoosh, or build-time image pipelines.

## Fonts

Web fonts block text rendering or cause it to jump when they arrive.

```html
<link rel="preload" href="/fonts/inter-var.woff2" as="font" type="font/woff2" crossorigin>
<style>
  @font-face { font-family: "Inter"; src: url(/fonts/inter-var.woff2) format("woff2");
               font-display: swap; font-weight: 100 900; }
  body { font-family: Inter, system-ui, sans-serif; }
</style>
```

- **WOFF2** only (every current browser supports it).
- **`font-display: swap`**: show fallback text immediately, swap when the font arrives.
- **Self-host** and **subset** (only the characters you need). Japanese fonts are huge (thousands of glyphs, several MB); use system fonts (`"Hiragino Sans", "Noto Sans JP", "Yu Gothic", sans-serif`) or services that split fonts into per-character-range files.
- A **variable font** replaces several weight files with one.
- Or use the system font stack and skip web fonts entirely.

## Preventing layout shift (CLS)

- Dimensions on images, videos and iframes (`width`/`height` or `aspect-ratio`).
- Reserve space for ads, embeds and banners (`min-height`).
- Don't insert content above existing content (cookie banners at the bottom or as overlays).
- `font-display: optional` or matched fallback metrics (`size-adjust`) reduce font-swap jumps.

## Resource hints

```html
<link rel="preconnect" href="https://cdn.example.com">          <!-- open the connection early -->
<link rel="preload" href="hero.avif" as="image" fetchpriority="high">  <!-- fetch now, needed soon -->
<link rel="prefetch" href="/next-page.html">                    <!-- maybe needed later, low priority -->
```

Preload only what's needed for the first view; preloading everything means nothing is prioritised.

## Other wins

- **Compression**: servers should send Brotli or gzip (check the `content-encoding` response header).
- **Caching**: fingerprinted file names (`app.3f9a1c.js`) with `Cache-Control: max-age=31536000, immutable`; HTML with short or no caching.
- **Minify** CSS and JS (build tools do it).
- **`content-visibility: auto`** on long off-screen sections lets the browser skip rendering them until they're near.
- **A CDN** serves files from a location near the user.
- **Less**: the fastest request is the one you don't make. Question every library and every font weight.

@@quiz
Which metric measures how much the page jumps around while loading?
- LCP
- INP
* CLS
- TTFB
? Cumulative Layout Shift; fix it with reserved dimensions.

@@quiz
What does `defer` do on a script?
* Downloads in parallel and runs after the HTML is parsed, in order
- Never runs the script
- Runs it before CSS
- Runs it as soon as it downloads, in any order
? `async` is the "any order" one.

@@quiz
Should the hero image at the top of the page use `loading="lazy"`?
- Yes, always lazy-load images
* No: it's likely the LCP element; load it eagerly with `fetchpriority="high"`
- Only on mobile
- Only if it's AVIF
? Lazy-load only below-the-fold images.

@@quiz
Why keep `width` and `height` attributes on images even when CSS sizes them?
- They're required by HTML
* The browser reserves space at the right aspect ratio before the image loads, preventing layout shift
- They make the image sharper
- They enable lazy loading
? Pair them with `height: auto` in CSS.

@@drill
Which script attribute downloads in parallel and runs after the HTML is parsed, in order?
= defer
? async runs as soon as it arrives, in any order.

@@flashcards
LCP :: main content visible, ≤ 2.5 s
INP :: response to input, ≤ 200 ms
CLS :: layout shift, ≤ 0.1
render-blocking :: stylesheets in the head
`defer` :: run after parsing, in order
`async` :: run when ready, any order
AVIF / WebP :: modern image formats
`loading="lazy"` :: load near the viewport
`fetchpriority="high"` :: prioritise the LCP image
`font-display: swap` :: show fallback text first
`rel="preconnect"` :: open a connection early
`rel="preload"` :: fetch now
`Cache-Control: immutable` :: cache fingerprinted files forever
`content-visibility: auto` :: skip rendering off-screen sections

@@exercise Fix a slow page
This page scores badly. Fix it: move the script to `defer`; give the hero image `width`, `height`, `fetchpriority="high"` and a `srcset` with 800w and 1600w versions plus `sizes="100vw"`; make the three gallery images `loading="lazy"` with dimensions; add `font-display: swap` to the `@font-face`; preload the font file; and add `preconnect` to the image CDN.
@@starter
<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>Help desk</title>
<script src="analytics-and-chat-widget.js"></script>
<style>
  @font-face { font-family: "Brand"; src: url(/fonts/brand.woff2) format("woff2"); }
  body { font-family: Brand, system-ui, sans-serif; }
  img { max-width: 100%; height: auto; }
</style>
</head>
<body>
  <img src="https://img.example.com/hero-1600.jpg" alt="The help desk team">
  <h1>We're here to help</h1>
  <img src="https://img.example.com/office-1.jpg" alt="Tokyo office">
  <img src="https://img.example.com/office-2.jpg" alt="Osaka office">
  <img src="https://img.example.com/office-3.jpg" alt="Fukuoka office">
</body>
</html>
@@requires
defer
fetchpriority="high"
srcset="https://img.example.com/hero-800.jpg 800w, https://img.example.com/hero-1600.jpg 1600w"
sizes="100vw"
loading="lazy"
font-display: swap
rel="preload"
rel="preconnect" href="https://img.example.com"
width="1600" height="900"
@@hint
Head: `<link rel="preconnect" href="https://img.example.com">`, `<link rel="preload" href="/fonts/brand.woff2" as="font" type="font/woff2" crossorigin>`, `<script src="..." defer></script>`. Gallery: `<img src="..." alt="..." width="600" height="400" loading="lazy">`.
@@solution
<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>Help desk</title>
<link rel="preconnect" href="https://img.example.com">
<link rel="preload" href="/fonts/brand.woff2" as="font" type="font/woff2" crossorigin>
<script src="analytics-and-chat-widget.js" defer></script>
<style>
  @font-face { font-family: "Brand"; src: url(/fonts/brand.woff2) format("woff2"); font-display: swap; }
  body { font-family: Brand, system-ui, sans-serif; }
  img { max-width: 100%; height: auto; }
</style>
</head>
<body>
  <img src="https://img.example.com/hero-1600.jpg"
       srcset="https://img.example.com/hero-800.jpg 800w, https://img.example.com/hero-1600.jpg 1600w"
       sizes="100vw" width="1600" height="900" fetchpriority="high" alt="The help desk team">
  <h1>We're here to help</h1>
  <img src="https://img.example.com/office-1.jpg" alt="Tokyo office" width="600" height="400" loading="lazy" decoding="async">
  <img src="https://img.example.com/office-2.jpg" alt="Osaka office" width="600" height="400" loading="lazy" decoding="async">
  <img src="https://img.example.com/office-3.jpg" alt="Fukuoka office" width="600" height="400" loading="lazy" decoding="async">
</body>
</html>

@@exercise Modern image formats with picture
Serve the team photo as AVIF, then WebP, then JPEG as a fallback, using `<picture>` with two `<source>` elements (`type="image/avif"` and `type="image/webp"`) and an `<img>` that has `width`, `height` and `alt`. Then stop the cookie banner from shifting the layout: make it `position: fixed` at the bottom instead of inserting it at the top of the page.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Team</title>
<style>
  .cookie { background: #0f172a; color: white; padding: 1rem; }
  img { max-width: 100%; height: auto; }
</style>
</head>
<body>
  <div class="cookie">We use only essential cookies. <button>OK</button></div>
  <h1>Our team</h1>
  <img src="team.jpg" alt="The IT team">
</body>
</html>
@@requires
<picture>
<source srcset="team.avif" type="image/avif">
<source srcset="team.webp" type="image/webp">
width="1200" height="800"
position: fixed
bottom: 0
@@hint
The browser picks the first `<source>` whose `type` it supports, otherwise the `<img>`. For the banner: `.cookie { position: fixed; inset-inline: 0; bottom: 0; }`.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Team</title>
<style>
  .cookie { position: fixed; inset-inline: 0; bottom: 0; background: #0f172a; color: white; padding: 1rem; }
  img { max-width: 100%; height: auto; }
</style>
</head>
<body>
  <h1>Our team</h1>
  <picture>
    <source srcset="team.avif" type="image/avif">
    <source srcset="team.webp" type="image/webp">
    <img src="team.jpg" alt="The IT team" width="1200" height="800">
  </picture>
  <div class="cookie" role="region" aria-label="Cookie notice">We use only essential cookies. <button>OK</button></div>
</body>
</html>

@@exercise Measure layout shift and LCP in the page
Add a script that uses `PerformanceObserver` to log Core Web Vitals in the console: observe `largest-contentful-paint` (log the latest entry's `startTime`) and `layout-shift` (sum `entry.value` for entries without `hadRecentInput`, and log the running CLS total). Use `buffered: true` so entries from before the script ran are included. Then load the page, open the console and see the numbers.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Vitals</title></head>
<body>
  <h1>Status</h1>
  <p id="late"></p>
  <img src="https://picsum.photos/seed/vitals/800/400" alt="Server room">
  <script>
    setTimeout(() => { document.getElementById("late").textContent = "A banner that arrives late and pushes things down."; }, 800);
  </script>
</body>
</html>
@@requires
new PerformanceObserver
largest-contentful-paint
layout-shift
hadRecentInput
buffered: true
console.log
@@hint
`new PerformanceObserver(list => { const entries = list.getEntries(); console.log("LCP", entries.at(-1).startTime); }).observe({ type: "largest-contentful-paint", buffered: true });` and similarly for `layout-shift` with a running `cls` total.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Vitals</title>
<script>
  new PerformanceObserver(list => {
    const last = list.getEntries().at(-1);
    console.log("LCP (ms):", Math.round(last.startTime), last.element);
  }).observe({ type: "largest-contentful-paint", buffered: true });

  let cls = 0;
  new PerformanceObserver(list => {
    for (const entry of list.getEntries()) {
      if (!entry.hadRecentInput) cls += entry.value;
    }
    console.log("CLS so far:", cls.toFixed(3));
  }).observe({ type: "layout-shift", buffered: true });
</script>
</head>
<body>
  <h1>Status</h1>
  <p id="late"></p>
  <img src="https://picsum.photos/seed/vitals/800/400" alt="Server room">
  <script>
    setTimeout(() => { document.getElementById("late").textContent = "A banner that arrives late and pushes things down."; }, 800);
  </script>
</body>
</html>

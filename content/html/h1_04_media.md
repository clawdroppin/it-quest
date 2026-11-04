@@lesson h1-media Images, Video, Audio and Embeds Done Right
@@text
Media is usually the heaviest part of a page. Done right, images are sharp on every screen, fast on slow connections, and meaningful to people who can't see them.

## Alt text

Every `<img>` needs `alt`:

- **Informative image**: describe what it tells you, in context. A screenshot in a help article: `alt="The Map network drive button in File Explorer's toolbar"`.
- **Functional image** (an image link or button): describe the action: `alt="Search"`, not `alt="magnifying glass"`.
- **Decorative image**: `alt=""` (empty), so screen readers skip it. Missing alt is different: the reader may read the file name aloud.
- **Complex image** (a chart): a short alt plus the data in text or a table nearby.

## Size attributes stop layout shift

Always set `width` and `height` (the image's real pixel size, or anything with the same ratio). The browser reserves the space before the image loads, so text doesn't jump. With CSS `img { max-width: 100%; height: auto; }` the image still scales.

## Responsive images: srcset and sizes

Phones shouldn't download a 3000-pixel hero image. Offer several files and let the browser choose:

```html,norun
<img src="office-800.jpg"
     srcset="office-400.jpg 400w, office-800.jpg 800w, office-1600.jpg 1600w"
     sizes="(max-width: 600px) 100vw, 50vw"
     width="800" height="533" alt="The Tokyo office reception">
```

- `srcset` lists files with their real widths (`400w`).
- `sizes` tells the browser how wide the image will be **displayed**: full viewport width on small screens, half otherwise.
- The browser picks the smallest file that's sharp enough for the screen's pixel density.

For fixed-size images (a 64-pixel logo) use density descriptors: `srcset="logo.png 1x, logo@2x.png 2x"`.

## `<picture>`: formats and art direction

```html,norun
<picture>
  <source type="image/avif" srcset="chart.avif">
  <source type="image/webp" srcset="chart.webp">
  <img src="chart.png" width="600" height="300" alt="Tickets per week, rising from 40 to 65">
</picture>
```

The browser uses the first `<source>` it supports, so modern browsers get AVIF/WebP (often 30–50% smaller) and old ones get PNG. `<source media="(max-width: 600px)">` can also swap in a different crop for small screens ("art direction").

## Loading

- `loading="lazy"` defers images below the fold until the user scrolls near them. Don't lazy-load the main image at the top (it delays the most important paint).
- `decoding="async"` lets the browser decode off the main thread.
- `fetchpriority="high"` on the hero image tells the browser to fetch it first.

## Formats in one table

| Format | Use for |
|---|---|
| AVIF / WebP | photos and most images on modern sites |
| JPEG | photos, maximum compatibility |
| PNG | screenshots, transparency, sharp edges |
| SVG | logos, icons, diagrams (vector: sharp at any size) |
| GIF | avoid: use a short muted video instead |

## Video and audio

```html,norun
<video controls width="640" height="360" poster="intro.jpg" preload="metadata">
  <source src="intro.webm" type="video/webm">
  <source src="intro.mp4" type="video/mp4">
  <track kind="captions" src="intro.en.vtt" srclang="en" label="English" default>
  <track kind="captions" src="intro.ja.vtt" srclang="ja" label="日本語">
  <a href="intro.mp4">Download the video</a>
</video>
```

- `controls` shows the player. `autoplay` only works with `muted` and annoys people; avoid it except for silent background loops.
- **Captions** (`<track>` with a WebVTT file) are required for accessibility and used by many people with the sound off.
- `poster` shows an image before playback; `preload="metadata"` avoids downloading the whole file up front.
- `<audio controls>` works the same way.

## Embeds: iframe

`<iframe>` embeds another page (maps, videos, dashboards). Give it a `title` (screen readers announce it), `loading="lazy"`, and restrict what it may do with `sandbox` (allow only what it needs, e.g. `sandbox="allow-scripts allow-same-origin"`) and `allow` (camera, fullscreen...). Embedding third-party pages also sends your visitors' data to them; think about privacy.

@@quiz
What alt text should a purely decorative image have?
* An empty alt: `alt=""`
- No alt attribute at all
- `alt="image"`
- The file name
? Empty alt tells screen readers to skip it; missing alt may make them read the file name.

@@quiz
Why set width and height on images?
- To make them blurry
* So the browser reserves the space before loading, preventing layout shift
- To disable responsive scaling
- It's only for SVG
? With `height: auto` in CSS the image still scales.

@@quiz
What does `sizes` tell the browser in a responsive image?
- The file sizes in KB
* How wide the image will be displayed at different viewport widths
- Which formats are supported
- The image's real pixel size
? The browser combines it with srcset widths and screen density.

@@quiz
Which images should not get `loading="lazy"`?
- Images in the footer
* The main image visible at the top of the page
- Thumbnails in a long list
- Images in closed tabs
? Lazy-loading the hero delays the largest paint.

@@flashcards
`alt=""` :: decorative image: skipped by screen readers
functional alt :: describe the action, not the picture
`width` + `height` :: reserve space, prevent layout shift
`srcset="a.jpg 400w, ..."` :: candidate files with widths
`sizes` :: displayed width per viewport condition
`<picture><source type>` :: format fallbacks
`loading="lazy"` :: defer offscreen images
`fetchpriority="high"` :: fetch the hero image first
`<track kind="captions">` :: video captions (WebVTT)
`poster` :: image before the video plays
`<iframe title sandbox>` :: accessible, restricted embed

@@exercise Fix the alt texts
Give every image the right `alt`: the logo link goes to the home page (functional), the divider is decorative, the screenshot shows "the Wi-Fi settings with SAKURA-CORP selected", and the search button image is a button that searches. Also add `width` and `height` to the screenshot (`800` × `450`).
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Connect to Wi-Fi</title></head>
<body>
  <a href="/"><img src="logo.svg"></a>
  <img src="divider.png">
  <h1>Connect to Wi-Fi</h1>
  <img src="wifi-settings.png">
  <form role="search">
    <input type="search" name="q" aria-label="Search the help center">
    <button><img src="search.svg"></button>
  </form>
</body>
</html>
@@requires
alt="Home"
<img src="divider.png" alt="">
alt="The Wi-Fi settings with SAKURA-CORP selected"
width="800" height="450"
alt="Search"
@@hint
Logo link: `alt="Home"` (or "Sakura Systems home"; the checker expects `Home`). Divider: `alt=""`. Button image: `alt="Search"`.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Connect to Wi-Fi</title></head>
<body>
  <a href="/"><img src="logo.svg" alt="Home"></a>
  <img src="divider.png" alt="">
  <h1>Connect to Wi-Fi</h1>
  <img src="wifi-settings.png" alt="The Wi-Fi settings with SAKURA-CORP selected" width="800" height="450">
  <form role="search">
    <input type="search" name="q" aria-label="Search the help center">
    <button><img src="search.svg" alt="Search"></button>
  </form>
</body>
</html>

@@exercise A responsive, modern-format hero
Make the office photo responsive: a `<picture>` with an AVIF `<source>` (`office-800.avif 800w, office-1600.avif 1600w`), and an `<img>` fallback with `srcset="office-800.jpg 800w, office-1600.jpg 1600w"`, `sizes="(max-width: 700px) 100vw, 700px"`, width 800, height 533, alt "The Tokyo office reception", and `fetchpriority="high"` (it's the main image, so no lazy loading). Lazy-load the two thumbnails below it.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Our office</title></head>
<body>
  <h1>Our Tokyo office</h1>
  <img src="office-1600.jpg">
  <h2>Meeting rooms</h2>
  <img src="room-a.jpg" alt="Meeting room Sakura, 8 seats" width="300" height="200">
  <img src="room-b.jpg" alt="Meeting room Fuji, 12 seats" width="300" height="200">
</body>
</html>
@@requires
<picture>
<source type="image/avif"
sizes="(max-width: 700px) 100vw, 700px"
fetchpriority="high"
loading="lazy"
@@hint
`<picture><source type="image/avif" srcset="office-800.avif 800w, office-1600.avif 1600w" sizes="(max-width: 700px) 100vw, 700px"><img src="office-800.jpg" srcset="..." sizes="..." width="800" height="533" alt="..." fetchpriority="high"></picture>`. Add `loading="lazy"` to both room images.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Our office</title></head>
<body>
  <h1>Our Tokyo office</h1>
  <picture>
    <source type="image/avif" srcset="office-800.avif 800w, office-1600.avif 1600w" sizes="(max-width: 700px) 100vw, 700px">
    <img src="office-800.jpg" srcset="office-800.jpg 800w, office-1600.jpg 1600w"
         sizes="(max-width: 700px) 100vw, 700px" width="800" height="533"
         alt="The Tokyo office reception" fetchpriority="high">
  </picture>
  <h2>Meeting rooms</h2>
  <img src="room-a.jpg" alt="Meeting room Sakura, 8 seats" width="300" height="200" loading="lazy">
  <img src="room-b.jpg" alt="Meeting room Fuji, 12 seats" width="300" height="200" loading="lazy">
</body>
</html>

@@exercise An accessible training video
Turn the training video into an accessible player: show `controls`, a `poster` (`training.jpg`), `preload="metadata"`, two sources (WebM first, then MP4), English captions as the default track and Japanese captions, and a download link as fallback content. Embed the map below in an `<iframe>` with a `title`, `loading="lazy"` and `sandbox="allow-scripts"`.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Security training</title></head>
<body>
  <h1>Security awareness training</h1>
  <video src="training.mp4" autoplay></video>
  <h2>Where to find us</h2>
</body>
</html>
@@requires
<video controls
preload="metadata"
<source src="training.webm" type="video/webm">
<track kind="captions"
srclang="ja"
<iframe
sandbox="allow-scripts"
@@hint
`<video controls width="640" height="360" poster="training.jpg" preload="metadata"><source src="training.webm" type="video/webm"><source src="training.mp4" type="video/mp4"><track kind="captions" src="training.en.vtt" srclang="en" label="English" default><track kind="captions" src="training.ja.vtt" srclang="ja" label="日本語"><a href="training.mp4">Download the video</a></video>`. Remove `autoplay`.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Security training</title></head>
<body>
  <h1>Security awareness training</h1>
  <video controls width="640" height="360" poster="training.jpg" preload="metadata">
    <source src="training.webm" type="video/webm">
    <source src="training.mp4" type="video/mp4">
    <track kind="captions" src="training.en.vtt" srclang="en" label="English" default>
    <track kind="captions" src="training.ja.vtt" srclang="ja" label="日本語">
    <a href="training.mp4">Download the video</a>
  </video>
  <h2>Where to find us</h2>
  <iframe src="https://maps.example/embed?q=Sakura+Systems+Tokyo" title="Map of the Tokyo office"
          width="600" height="400" loading="lazy" sandbox="allow-scripts"></iframe>
</body>
</html>

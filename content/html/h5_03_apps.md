@@lesson h5-apps Components, Accessible Interfaces, PWAs and Frameworks
@@concepts objects, modules, testing
@@text
The last pieces of building web applications: reusable components, interfaces that work for everyone even as they change, installing web apps like native ones, and how the professional toolchain (npm, Vite, React and friends) fits in.

## Web components: your own HTML elements

Browsers let you define new tags, with their own behaviour and private styles. They work in any page and any framework:

```html
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Components</title></head>
<body>
  <p>Mail server <status-badge state="up"></status-badge></p>
  <p>VPN <status-badge state="down"></status-badge></p>
  <p>Backups <status-badge state="degraded"></status-badge></p>
  <script>
    class StatusBadge extends HTMLElement {
      static observedAttributes = ["state"];       // re-render when these attributes change

      constructor() {
        super();
        this.attachShadow({ mode: "open" });       // private DOM: page CSS can't leak in or out
      }
      attributeChangedCallback() { this.render(); }
      connectedCallback() { this.render(); }       // added to the page

      render() {
        const state = this.getAttribute("state") ?? "unknown";
        const colours = { up: "#1b7f3b", down: "#b3261e", degraded: "#9a6700" };
        this.shadowRoot.innerHTML = `<style>
            span { padding: 2px 8px; border-radius: 99px; color: white; font: 600 0.8em system-ui; }
          </style><span></span>`;
        const span = this.shadowRoot.querySelector("span");
        span.textContent = state;                  // text set safely, as always
        span.style.background = colours[state] ?? "gray";
      }
    }
    customElements.define("status-badge", StatusBadge);   // custom names must contain a dash
  </script>
</body>
</html>
```

## Accessible dynamic interfaces

When the page changes without a reload, people using screen readers or keyboards can be left behind. The rules:

| Situation | What to do |
|---|---|
| a message appears ("Saved", "3 results") | put it in a live region: `<p aria-live="polite">`; screen readers read changes in it |
| a button shows or hides a panel | `aria-expanded="true/false"` on the button, `aria-controls="panel-id"` |
| a toggle button (on/off) | `aria-pressed="true/false"` |
| a dialog opens | use `<dialog>` and `showModal()`: it traps focus and closes with Escape |
| an item is deleted | move focus somewhere sensible (the next item, or the list's heading) |
| custom controls | use real `<button>`s, not clickable `<div>`s: they're focusable and work with Enter and Space for free |

Test with the keyboard alone (Tab, Shift+Tab, Enter, Space, Escape), and with a screen reader at least once: NVDA on Windows, VoiceOver on macOS/iOS (Cmd+F5), TalkBack on Android.

## Progressive Web Apps

A **PWA** is a website that can be installed and can work offline. It needs three things:

1. **HTTPS** (or `localhost` while developing).
2. A **web app manifest**: the name, icons, colours and how to open it.

```text
<link rel="manifest" href="manifest.webmanifest">

{
  "name": "Help Desk",
  "short_name": "Help Desk",
  "start_url": "./",
  "display": "standalone",
  "theme_color": "#2f6f5e",
  "icons": [{ "src": "icon-512.png", "sizes": "512x512", "type": "image/png" }]
}
```

3. A **service worker**: a script that sits between the page and the network and can answer requests from a cache when offline.

```js,norun
// in the page
if ("serviceWorker" in navigator) navigator.serviceWorker.register("sw.js");

// sw.js
self.addEventListener("install", (e) => e.waitUntil(caches.open("v1").then((c) => c.addAll(["./", "app.js", "style.css"]))));
self.addEventListener("fetch", (e) => e.respondWith(fetch(e.request).catch(() => caches.match(e.request))));
```

Chrome and Edge then offer "Install app", and on phones it gets a home-screen icon. PWAs are a serious alternative to native apps for internal company tools: one codebase, no app store.

## The professional toolchain

You've written everything by hand so far, which is the best way to learn. Real projects add tools:

| Tool | What it does |
|---|---|
| **npm** | installs JavaScript packages (`npm install`) listed in `package.json` |
| **Vite** | a dev server with instant reload, and a build step that bundles and minifies for production |
| **TypeScript** | JavaScript with types, catches mistakes before running (see the TypeScript track) |
| **React / Vue / Svelte / Angular** | frameworks that make `state → render` efficient for big UIs, with components |
| **ESLint / Prettier** | find bugs / format code |
| **Vitest / Playwright** | unit tests / tests that drive a real browser |

```text
npm create vite@latest helpdesk -- --template react-ts
cd helpdesk && npm install && npm run dev
```

The same counter as earlier, in React: the `render()` you wrote by hand is now the component function, and `setCount` both updates the state and re-renders.

```js,norun
import { useState } from "react";

export function Counter() {
  const [count, setCount] = useState(0);
  return (
    <>
      <p>Coffees today: <strong>{count}</strong></p>
      <button onClick={() => setCount(count + 1)}>+1</button>
      {count >= 4 && <p>Maybe switch to water?</p>}
    </>
  );
}
```

Which framework? React has the most jobs and libraries; Vue and Svelte are often found easier to learn; Angular is common in large enterprises. The concepts from this module transfer to all of them.

## Deploying

A front end is just files, so it can be hosted almost anywhere: **GitHub Pages**, **Netlify**, **Cloudflare Pages**, **Azure Static Web Apps** or an S3 bucket, usually deployed automatically from Git on every push. (The IT Quest website is a GitHub Pages site.)

@@quiz
Why does a web component use `attachShadow()`?
- To make it load faster
* To give it a private DOM, so the page's CSS doesn't affect it and its CSS doesn't leak out
- To hide it from screen readers
- It's required for `customElements.define`
? Shadow DOM is encapsulation: the component looks the same in any page.

@@quiz
After clicking "Save", the page shows "Saved ✓" in a paragraph. How do screen-reader users find out?
- They can't
* Put the message in an element with `aria-live="polite"`, so changes to it are read out
- Use `alert()` every time
- Add `role="button"` to the paragraph
? Live regions announce changes without moving the user's focus.

@@quiz
What does a service worker let a PWA do?
- Run faster JavaScript
* Answer requests from a cache, so the app opens offline
- Access the file system
- Skip HTTPS
? It's a programmable proxy between the page and the network.

@@quiz
What does Vite do for a front-end project?
* Runs a fast development server and builds optimized, bundled files for production
- Hosts the site
- Replaces JavaScript with TypeScript
- Tests the site in real browsers
? Bundlers like Vite turn many source files (and npm packages) into a few small files for the browser.

@@drill
Which file describes a Progressive Web App's name and icons?
= manifest.json
= web app manifest
= manifest
= manifest.webmanifest
? Link it with <link rel="manifest">.

@@flashcards
`customElements.define("my-tag", Class)` :: register a custom element (the name needs a dash)
`class X extends HTMLElement` :: the base class of custom elements
`connectedCallback` :: runs when the element is added to the page
`static observedAttributes` + `attributeChangedCallback` :: react to attribute changes
shadow DOM :: a component's private, encapsulated DOM
`aria-live="polite"` :: screen readers announce changes in this element
`aria-expanded` / `aria-controls` :: a button that shows/hides a panel
`<dialog>` + `showModal()` :: an accessible modal dialog
PWA :: installable, offline-capable web app
web app manifest :: name, icons and display mode of a PWA
service worker :: script between page and network, enables offline
Vite :: dev server and production bundler
React / Vue / Svelte / Angular :: front-end frameworks

@@exercise A custom element
Create a `<disk-meter>` element: `<disk-meter used="73"></disk-meter>` shows a `<meter>` with that value (`min` 0, `max` 100, `high` 80) followed by the text `73% used`. Define a class that `extends HTMLElement`, build the content in `connectedCallback`, and register it with `customElements.define`.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Disks</title></head>
<body>
  <p>C: <disk-meter used="73"></disk-meter></p>
  <p>D: <disk-meter used="91"></disk-meter></p>
  <script>
    // TODO: the DiskMeter class and customElements.define
  </script>
</body>
</html>
@@requires
extends HTMLElement
connectedCallback
customElements.define("disk-meter"
@@hint
`class DiskMeter extends HTMLElement { connectedCallback() { const used = Number(this.getAttribute("used")); const m = document.createElement("meter"); m.min = 0; m.max = 100; m.high = 80; m.value = used; const label = document.createElement("span"); label.textContent = ` ${used}% used`; this.replaceChildren(m, label); } }` and `customElements.define("disk-meter", DiskMeter);`.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Disks</title></head>
<body>
  <p>C: <disk-meter used="73"></disk-meter></p>
  <p>D: <disk-meter used="91"></disk-meter></p>
  <script>
    class DiskMeter extends HTMLElement {
      connectedCallback() {
        const used = Number(this.getAttribute("used"));
        const meter = document.createElement("meter");
        meter.min = 0;
        meter.max = 100;
        meter.high = 80;
        meter.value = used;
        const label = document.createElement("span");
        label.textContent = ` ${used}% used`;
        this.replaceChildren(meter, label);
      }
    }
    customElements.define("disk-meter", DiskMeter);
  </script>
</body>
</html>

@@exercise An accessible disclosure
The "Details" button shows and hides the panel. Make it accessible: give the button `aria-expanded` (kept in sync: `"false"` at first, `"true"` when open) and `aria-controls="details"`, and when it opens put the text `Details shown` in a new `aria-live="polite"` status paragraph (and `Details hidden` when it closes).
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Ticket</title></head>
<body>
  <h1>Ticket #5120: VPN drops</h1>
  <button type="button" id="toggle">Details</button>
  <div id="details" hidden>
    <p>Reported by aiko. Happens on the Osaka office Wi-Fi only, every 60 minutes.</p>
  </div>
  <script>
    const button = document.getElementById("toggle");
    const panel = document.getElementById("details");
    button.addEventListener("click", () => {
      panel.hidden = !panel.hidden;
    });
  </script>
</body>
</html>
@@requires
aria-expanded="false"
aria-controls="details"
aria-live="polite"
setAttribute("aria-expanded"
@@hint
`<button type="button" id="toggle" aria-expanded="false" aria-controls="details">`, add `<p id="status" aria-live="polite"></p>`, and in the handler: `const open = panel.hidden; panel.hidden = !open; button.setAttribute("aria-expanded", String(open)); document.getElementById("status").textContent = open ? "Details shown" : "Details hidden";`.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Ticket</title></head>
<body>
  <h1>Ticket #5120: VPN drops</h1>
  <button type="button" id="toggle" aria-expanded="false" aria-controls="details">Details</button>
  <div id="details" hidden>
    <p>Reported by aiko. Happens on the Osaka office Wi-Fi only, every 60 minutes.</p>
  </div>
  <p id="status" aria-live="polite"></p>
  <script>
    const button = document.getElementById("toggle");
    const panel = document.getElementById("details");
    button.addEventListener("click", () => {
      const open = panel.hidden;
      panel.hidden = !open;
      button.setAttribute("aria-expanded", String(open));
      document.getElementById("status").textContent = open ? "Details shown" : "Details hidden";
    });
  </script>
</body>
</html>

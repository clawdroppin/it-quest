@@lesson g8-project Project: A Component-Based Service Status Board
@@text
The final professional project: a status board built from your own web components, the kind of widget an IT team embeds on the intranet home page, in Teams tabs and on a lobby screen. Because it's made of custom elements, other teams can drop `<service-status>` into their own pages whatever framework they use.

## Components

- **`<status-board>`**: fetches (here: simulates fetching) a list of services, shows a loading state, renders a `<service-status>` per service, and refreshes every 30 seconds. Shows an error message with a Retry button if loading fails.
- **`<service-status name="VPN" state="degraded" since="09:12">`**: a row with a coloured dot, the name, the state as text (not only colour) and since-when. Clicking "Details" expands a slotted or attribute-provided message (an accessible disclosure).
- **`<uptime-bar values="100,100,99.2,...">`**: 30 small bars for the last 30 days, colour by uptime, with an accessible summary ("29 of 30 days fully operational").

## Requirements

- Shadow DOM for each component; theming through custom properties (`--ok`, `--warn`, `--bad`) set on the page.
- `observedAttributes` so changing `state` re-renders.
- A `status-change` `CustomEvent` (composed, bubbling) when a service's state changes between refreshes, and a polite live region on the page that announces it ("VPN is now operational").
- `prefers-reduced-motion` and dark mode respected.
- Works with keyboard only; no colour-only information.
- No external libraries.

@@quiz
Why announce state changes in a live region instead of only changing the dot's colour?
* Colour changes are invisible to screen readers and to people who can't distinguish the colours
- Live regions are faster
- Dots can't change colour
- It's required by shadow DOM
? Information must not rely on colour alone.

@@quiz
How should the page theme all three components consistently?
- Copy the CSS into each shadow root manually
* Set custom properties like `--ok` on the page; the components read them with `var(--ok, fallback)`
- Use `!important`
- Avoid shadow DOM
? Custom properties cross the shadow boundary.

@@quiz
Why use custom elements with Shadow DOM for the status board components?
* Each component's markup and styles are encapsulated and can't clash with the page
- They load faster than HTML
- They don't need JavaScript
- Search engines require it
? CSS custom properties still pass through, which is how theming works.

@@flashcards
board :: fetch, loading state, error + retry, refresh
row :: dot + text state + since + disclosure
uptime bar :: 30 days, accessible summary
theming :: custom properties through shadow DOM
announcing changes :: CustomEvent + polite live region

@@exercise Build the status board
Build the three components described in the lesson and use them on a page. Requirements checked: three `customElements.define` calls (`status-board`, `service-status`, `uptime-bar`), shadow roots, `observedAttributes`, a disclosure with `aria-expanded`, a `status-change` CustomEvent that is `composed`, a `role="status"` region announcing changes, custom-property theming with fallbacks, a refresh with `setInterval` that is cleared in `disconnectedCallback`, and a reduced-motion rule.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Service status</title></head>
<body>
  <h1>IT service status</h1>
  <status-board></status-board>
  <script>
    // Simulated API: returns services with random state changes.
    async function fetchServices() {
      await new Promise(r => setTimeout(r, 400));
      const pick = () => ["ok", "ok", "ok", "degraded", "down"][Math.floor(Math.random() * 5)];
      return [
        { name: "Email", state: "ok", since: "08:00", message: "", uptime: Array(30).fill(100) },
        { name: "VPN", state: pick(), since: "09:12", message: "Some users in Osaka can't connect.", uptime: [...Array(29).fill(100), 97.5] },
        { name: "File server", state: pick(), since: "08:40", message: "D: drive almost full.", uptime: [...Array(28).fill(100), 99.1, 100] },
      ];
    }
  </script>
</body>
</html>
@@requires
customElements.define("status-board"
customElements.define("service-status"
customElements.define("uptime-bar"
attachShadow(
observedAttributes
aria-expanded
new CustomEvent("status-change"
composed: true
role="status"
var(--ok,
setInterval(
clearInterval(
disconnectedCallback
prefers-reduced-motion
@@hint
Build `uptime-bar` first (pure rendering from an attribute), then `service-status` (attributes in, disclosure inside), then `status-board` (data, timers, events). Keep the previous states in a `Map` in the board to detect changes between refreshes.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Service status</title>
<style>
  :root { --ok: #15803d; --warn: #b45309; --bad: #b91c1c; --surface: #ffffff; --text: #0f172a; color-scheme: light dark; }
  @media (prefers-color-scheme: dark) { :root { --ok: #4ade80; --warn: #fbbf24; --bad: #f87171; --surface: #1e293b; --text: #f1f5f9; } body { background: #0f172a; } }
  body { font-family: system-ui, sans-serif; color: var(--text); max-inline-size: 48rem; margin-inline: auto; padding: 1rem; }
</style>
</head>
<body>
  <h1>IT service status</h1>
  <p role="status" id="announce"></p>
  <status-board></status-board>
  <script>
    async function fetchServices() {
      await new Promise(r => setTimeout(r, 400));
      const pick = () => ["ok", "ok", "ok", "degraded", "down"][Math.floor(Math.random() * 5)];
      return [
        { name: "Email", state: "ok", since: "08:00", message: "", uptime: Array(30).fill(100) },
        { name: "VPN", state: pick(), since: "09:12", message: "Some users in Osaka can't connect.", uptime: [...Array(29).fill(100), 97.5] },
        { name: "File server", state: pick(), since: "08:40", message: "D: drive almost full.", uptime: [...Array(28).fill(100), 99.1, 100] },
      ];
    }

    const STATE_TEXT = { ok: "Operational", degraded: "Degraded", down: "Down" };

    class UptimeBar extends HTMLElement {
      static observedAttributes = ["values"];
      constructor() {
        super();
        this.attachShadow({ mode: "open" }).innerHTML = `
          <style>
            :host { display: block; }
            .bars { display: flex; gap: 2px; block-size: 1.5rem; align-items: stretch; }
            .bar { flex: 1; border-radius: 2px; background: var(--ok, #15803d); }
            .bar.warn { background: var(--warn, #b45309); }
            .bar.bad { background: var(--bad, #b91c1c); }
            p { margin: .25rem 0 0; font-size: .8rem; }
          </style>
          <div class="bars" aria-hidden="true"></div><p></p>`;
      }
      attributeChangedCallback() {
        const values = (this.getAttribute("values") || "").split(",").filter(Boolean).map(Number);
        const bars = this.shadowRoot.querySelector(".bars");
        bars.replaceChildren(...values.map(v => {
          const d = document.createElement("span");
          d.className = "bar" + (v < 99 ? " bad" : v < 100 ? " warn" : "");
          d.title = `${v}%`;
          return d;
        }));
        const full = values.filter(v => v === 100).length;
        this.shadowRoot.querySelector("p").textContent = `${full} of ${values.length} days fully operational`;
      }
    }
    customElements.define("uptime-bar", UptimeBar);

    class ServiceStatus extends HTMLElement {
      static observedAttributes = ["name", "state", "since", "message"];
      constructor() {
        super();
        this.attachShadow({ mode: "open" }).innerHTML = `
          <style>
            :host { display: block; background: var(--surface, #fff); border: 1px solid color-mix(in srgb, currentColor 20%, transparent);
                    border-radius: 10px; padding: .75rem 1rem; margin-block: .5rem; }
            .row { display: flex; gap: .6rem; align-items: center; flex-wrap: wrap; }
            .dot { inline-size: .75rem; block-size: .75rem; border-radius: 50%; background: var(--ok, #15803d); }
            :host([state="degraded"]) .dot { background: var(--warn, #b45309); }
            :host([state="down"]) .dot { background: var(--bad, #b91c1c); animation: pulse 1.5s ease-in-out infinite; }
            @keyframes pulse { 50% { opacity: .3; } }
            @media (prefers-reduced-motion: reduce) { .dot { animation: none !important; } }
            .name { font-weight: 600; }
            .since { margin-inline-start: auto; opacity: .75; font-size: .9rem; }
            button { font: inherit; }
            button:focus-visible { outline: 3px solid currentColor; outline-offset: 2px; }
            .details { margin: .5rem 0 0; }
          </style>
          <div class="row">
            <span class="dot" aria-hidden="true"></span>
            <span class="name"></span>
            <span class="state"></span>
            <span class="since"></span>
            <button aria-expanded="false" aria-controls="details">Details</button>
          </div>
          <div id="details" class="details" hidden><slot name="uptime"></slot><p class="msg"></p></div>`;
        const btn = this.shadowRoot.querySelector("button");
        btn.addEventListener("click", () => {
          const open = btn.getAttribute("aria-expanded") === "true";
          btn.setAttribute("aria-expanded", String(!open));
          this.shadowRoot.getElementById("details").hidden = open;
        });
      }
      attributeChangedCallback() {
        const $ = s => this.shadowRoot.querySelector(s);
        const name = this.getAttribute("name") || "";
        $(".name").textContent = name;
        $(".state").textContent = STATE_TEXT[this.getAttribute("state")] || "Unknown";
        $(".since").textContent = this.getAttribute("since") ? `since ${this.getAttribute("since")}` : "";
        $(".msg").textContent = this.getAttribute("message") || "No incidents.";
        $("button").setAttribute("aria-label", `Details for ${name}`);
      }
    }
    customElements.define("service-status", ServiceStatus);

    class StatusBoard extends HTMLElement {
      constructor() {
        super();
        this.previous = new Map();
        this.attachShadow({ mode: "open" }).innerHTML = `
          <style>
            :host { display: block; }
            .error { color: var(--bad, #b91c1c); }
          </style>
          <p class="loading">Loading services…</p>
          <div class="list"></div>
          <p class="error" hidden>Couldn't load the status. <button>Retry</button></p>`;
        this.shadowRoot.querySelector(".error button").addEventListener("click", () => this.load());
      }
      connectedCallback() {
        this.load();
        this.timer = setInterval(() => this.load(), 30000);
      }
      disconnectedCallback() {
        clearInterval(this.timer);
      }
      async load() {
        const root = this.shadowRoot;
        this.setAttribute("aria-busy", "true");
        try {
          const services = await fetchServices();
          root.querySelector(".error").hidden = true;
          root.querySelector(".loading").hidden = true;
          const list = root.querySelector(".list");
          for (const s of services) {
            let row = list.querySelector(`service-status[name="${CSS.escape(s.name)}"]`);
            if (!row) {
              row = document.createElement("service-status");
              const bar = document.createElement("uptime-bar");
              bar.slot = "uptime";
              row.append(bar);
              list.append(row);
            }
            row.setAttribute("name", s.name);
            row.setAttribute("state", s.state);
            row.setAttribute("since", s.since);
            row.setAttribute("message", s.message);
            row.querySelector("uptime-bar").setAttribute("values", s.uptime.join(","));
            const before = this.previous.get(s.name);
            if (before && before !== s.state) {
              this.dispatchEvent(new CustomEvent("status-change", {
                detail: { name: s.name, from: before, to: s.state }, bubbles: true, composed: true,
              }));
            }
            this.previous.set(s.name, s.state);
          }
        } catch {
          root.querySelector(".error").hidden = false;
        } finally {
          this.setAttribute("aria-busy", "false");
        }
      }
    }
    customElements.define("status-board", StatusBoard);

    document.addEventListener("status-change", e => {
      const { name, to } = e.detail;
      document.getElementById("announce").textContent = `${name} is now ${STATE_TEXT[to].toLowerCase()}`;
    });
  </script>
</body>
</html>

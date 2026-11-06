@@lesson g8-components Web Components: Custom Elements, Shadow DOM and Slots
@@text
Frameworks like React, Vue and Svelte let you build reusable components, but each component only works inside its own framework. **Web Components** are the browser's built-in component model: a `<status-badge>` you write once works in plain HTML, in React, in a CMS template and in a Markdown page. Large design systems (GitHub, Adobe, Microsoft Fluent, SAP, Salesforce) ship web components for exactly that reason.

Three standards make them up:

1. **Custom elements**: define your own HTML tags with a JavaScript class.
2. **Shadow DOM**: an encapsulated DOM and CSS scope inside the element.
3. **Templates and slots**: markup to stamp out, and holes where the user's content goes.

## A first custom element

```html
<status-badge state="ok">Backups</status-badge>
<status-badge state="down">VPN</status-badge>
<script>
  class StatusBadge extends HTMLElement {
    static observedAttributes = ["state"];
    connectedCallback() { this.render(); }                 // added to the page
    attributeChangedCallback() { this.render(); }          // an observed attribute changed
    render() {
      const ok = this.getAttribute("state") === "ok";
      this.style.cssText = `display:inline-block;padding:.1rem .6rem;border-radius:999px;color:white;background:${ok ? "#15803d" : "#b91c1c"}`;
      this.title = ok ? "Operational" : "Down";
    }
  }
  customElements.define("status-badge", StatusBadge);
</script>
```

Rules: the name **must contain a hyphen** (so it can't clash with future HTML elements), the class extends `HTMLElement`, and `customElements.define` registers it. Elements in the page before the definition are **upgraded** when it runs.

## Lifecycle callbacks

| Callback | When |
|---|---|
| `constructor()` | created; call `super()` first; don't read attributes or children yet |
| `connectedCallback()` | inserted into the document (can happen more than once if moved) |
| `disconnectedCallback()` | removed; clean up timers and listeners on `window`/`document` |
| `attributeChangedCallback(name, old, value)` | an attribute listed in `observedAttributes` changed |

## Shadow DOM: encapsulation

```js,norun
class InfoCard extends HTMLElement {
  constructor() {
    super();
    const root = this.attachShadow({ mode: "open" });
    root.innerHTML = `
      <style>
        :host { display: block; border: 1px solid #cbd5e1; border-radius: 10px; padding: 1rem; }
        :host([variant="warning"]) { border-color: #f59e0b; background: #fffbeb; }
        h2 { margin: 0 0 .5rem; font-size: 1.1rem; }     /* only affects THIS h2 */
      </style>
      <h2><slot name="title">Untitled</slot></h2>
      <slot></slot>`;
  }
}
customElements.define("info-card", InfoCard);
```

```html,norun
<info-card variant="warning">
  <span slot="title">Maintenance</span>
  <p>Saturday 02:00–04:00.</p>
</info-card>
```

- Styles inside the shadow root don't leak out, and page styles don't leak in (except inherited properties like `color` and `font`, and custom properties).
- `:host` styles the element itself; `:host([attr])` styles it by attribute; `::slotted(p)` styles slotted children (top level only).
- **Slots**: the default `<slot>` receives children without a `slot` attribute; `<slot name="title">` receives `slot="title"` children. Content inside the slot tag is the fallback.
- **Theming from outside**: custom properties pass through the shadow boundary (`var(--card-accent, #2563eb)` inside, set `--card-accent` outside), and `::part()` lets the page style elements you expose with `part="label"`.

## Templates

`<template>` holds inert markup that isn't rendered until cloned, which is cheaper than parsing `innerHTML` for every instance:

```html,norun
<template id="row-tpl"><li><strong class="id"></strong> <span class="title"></span></li></template>
<script>
  const li = document.getElementById("row-tpl").content.cloneNode(true);
  li.querySelector(".id").textContent = "#9";
</script>
```

## Events and properties

Components talk to the outside with **attributes/properties in** and **events out**:

```js,norun
this.dispatchEvent(new CustomEvent("ticket-close", { detail: { id: 9 }, bubbles: true, composed: true }));
```

`composed: true` lets the event cross the shadow boundary. Attributes are strings; for rich data (arrays, objects) use JavaScript properties (`el.tickets = [...]`) with a getter and setter.

## Accessibility in components

Shadow DOM doesn't change accessibility rules: use real `<button>`s inside, give the host a role if needed, and remember `id` references (`aria-labelledby`, `label for`) **don't cross** the shadow boundary. Form-associated custom elements (`static formAssociated = true` + `attachInternals()`) can take part in forms and validation.

## When to use them

Good fit: design-system primitives shared across many apps and frameworks, embeddable widgets (a status badge other teams paste into their pages), progressive enhancement of server-rendered HTML. For a whole single-page app, a framework (or a small library on top of web components such as Lit) gives you state management and efficient re-rendering that raw custom elements don't.

@@quiz
Why must custom element names contain a hyphen?
* So they can never clash with current or future built-in HTML elements
- Hyphens make them faster
- It's a CSS requirement
- Only for shadow DOM elements
? `<status-badge>` is valid; `<badge>` is not.

@@quiz
Which callback is the right place to start reading attributes and rendering?
- constructor
* connectedCallback
- disconnectedCallback
- adoptedCallback
? The element is in the document then; constructors should stay light.

@@quiz
How can a page theme the inside of a component's shadow DOM?
- Regular selectors reach inside
* Custom properties (`--card-accent`) and `::part()` cross the boundary by design
- `!important`
- It's impossible
? Encapsulation with deliberate escape hatches.

@@quiz
What does `composed: true` on a CustomEvent do?
- Makes it bubble
* Lets it propagate out across the shadow DOM boundary
- Makes it cancelable
- Sends it to the server
? Combine with `bubbles: true` for listeners on ancestors.

@@drill
Which method registers a custom element?
= customElements.define
= define
? Names must contain a hyphen.

@@flashcards
`customElements.define("x-y", Class)` :: register an element
hyphen in name :: required
`connectedCallback` :: inserted into the page
`disconnectedCallback` :: removed; clean up
`observedAttributes` :: attributes to watch
`attachShadow({ mode: "open" })` :: create a shadow root
`:host` :: style the element itself
`<slot>` :: where children go
`slot="title"` :: target a named slot
`::slotted()` :: style slotted children
`::part()` :: style exposed internals
`<template>` :: inert markup to clone
`CustomEvent(..., { composed: true })` :: event crosses shadow DOM

@@exercise A status badge element
Define `<status-badge>` with a shadow root. It observes a `state` attribute (`ok`, `degraded`, `down`) and renders a coloured dot plus the slotted text, with styles inside the shadow root using `:host` and `:host([state="down"])` etc. The dot colour comes from a custom property `--badge-color` so the page can override it. Add a button on the page that cycles the VPN badge through the three states.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Badges</title></head>
<body>
  <status-badge state="ok">Backups</status-badge>
  <status-badge state="down" id="vpn">VPN</status-badge>
  <button id="cycle">Cycle VPN state</button>
</body>
</html>
@@requires
class StatusBadge extends HTMLElement
customElements.define("status-badge"
attachShadow(
observedAttributes
attributeChangedCallback
:host([state="down"])
<slot>
--badge-color
@@hint
In the constructor: `this.attachShadow({ mode: "open" }).innerHTML = `<style>:host { --badge-color: #15803d; display: inline-flex; gap: .4rem; align-items: center; } :host([state="degraded"]) { --badge-color: #b45309; } :host([state="down"]) { --badge-color: #b91c1c; } .dot { width: .6rem; height: .6rem; border-radius: 50%; background: var(--badge-color); }</style><span class="dot"></span><slot></slot>`;`. The CSS reacts to the attribute by itself; `attributeChangedCallback` can update an accessible label.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Badges</title></head>
<body>
  <status-badge state="ok">Backups</status-badge>
  <status-badge state="down" id="vpn">VPN</status-badge>
  <button id="cycle">Cycle VPN state</button>
  <script>
    const LABELS = { ok: "operational", degraded: "degraded", down: "down" };
    class StatusBadge extends HTMLElement {
      static observedAttributes = ["state"];
      constructor() {
        super();
        this.attachShadow({ mode: "open" }).innerHTML = `
          <style>
            :host { --badge-color: #15803d; display: inline-flex; gap: .4rem; align-items: center;
                    padding: .15rem .6rem; border-radius: 999px; background: #f1f5f9; font: 0.9rem system-ui; }
            :host([state="degraded"]) { --badge-color: #b45309; }
            :host([state="down"]) { --badge-color: #b91c1c; }
            .dot { width: .6rem; height: .6rem; border-radius: 50%; background: var(--badge-color); }
            .sr { position: absolute; width: 1px; height: 1px; overflow: hidden; clip: rect(0 0 0 0); }
          </style>
          <span class="dot"></span><slot></slot><span class="sr"></span>`;
      }
      attributeChangedCallback() {
        this.shadowRoot.querySelector(".sr").textContent = `: ${LABELS[this.getAttribute("state")] ?? "unknown"}`;
      }
    }
    customElements.define("status-badge", StatusBadge);

    const order = ["ok", "degraded", "down"];
    document.getElementById("cycle").addEventListener("click", () => {
      const vpn = document.getElementById("vpn");
      const next = order[(order.indexOf(vpn.getAttribute("state")) + 1) % order.length];
      vpn.setAttribute("state", next);
    });
  </script>
</body>
</html>

@@exercise A card with named slots
Build `<info-card>` with a shadow root that has a named slot `title` inside an `<h2>` (fallback text "Untitled"), a default slot for the body, and a named slot `actions` in a footer. Style it with `:host`, give it a `variant="warning"` look with `:host([variant="warning"])`, and expose the heading with `part="title"` so the page can restyle it with `info-card::part(title)`. Use a `<template>` for the shadow markup.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Cards</title></head>
<body>
  <info-card variant="warning">
    <span slot="title">Maintenance window</span>
    <p>Saturday 02:00–04:00, email may be slow.</p>
    <button slot="actions">Add to calendar</button>
  </info-card>
  <info-card>
    <p>No title given, so the fallback shows.</p>
  </info-card>
</body>
</html>
@@requires
<template id="info-card-tpl">
<slot name="title">Untitled</slot>
<slot name="actions">
part="title"
:host([variant="warning"])
::part(title)
content.cloneNode(true)
@@hint
`<template id="info-card-tpl"><style>...</style><h2 part="title"><slot name="title">Untitled</slot></h2><slot></slot><footer><slot name="actions"></slot></footer></template>`, then in the constructor: `this.attachShadow({ mode: "open" }).append(document.getElementById("info-card-tpl").content.cloneNode(true));`.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Cards</title>
<style>
  info-card { margin-block: 1rem; }
  info-card::part(title) { font-family: Georgia, serif; }
</style>
</head>
<body>
  <template id="info-card-tpl">
    <style>
      :host { display: block; border: 1px solid #cbd5e1; border-radius: 10px; padding: 1rem; max-inline-size: 30rem; }
      :host([variant="warning"]) { border-color: #f59e0b; background: #fffbeb; }
      h2 { margin: 0 0 0.5rem; font-size: 1.1rem; }
      footer { margin-block-start: 0.75rem; }
      ::slotted(p) { margin: 0; }
    </style>
    <h2 part="title"><slot name="title">Untitled</slot></h2>
    <slot></slot>
    <footer><slot name="actions"></slot></footer>
  </template>

  <info-card variant="warning">
    <span slot="title">Maintenance window</span>
    <p>Saturday 02:00–04:00, email may be slow.</p>
    <button slot="actions">Add to calendar</button>
  </info-card>
  <info-card>
    <p>No title given, so the fallback shows.</p>
  </info-card>

  <script>
    class InfoCard extends HTMLElement {
      constructor() {
        super();
        this.attachShadow({ mode: "open" }).append(document.getElementById("info-card-tpl").content.cloneNode(true));
      }
    }
    customElements.define("info-card", InfoCard);
  </script>
</body>
</html>

@@exercise A ticket list component with properties and events
Build `<ticket-list>` that takes its data through a JavaScript **property** (`el.tickets = [...]`, with a setter that re-renders), renders each ticket as a list item with a "Close" `<button>`, and when Close is clicked dispatches a `ticket-close` `CustomEvent` with `detail: { id }`, `bubbles: true` and `composed: true`. The page listens for `ticket-close`, removes that ticket from its array and sets the property again. Render text with `textContent`, not `innerHTML`.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Tickets</title></head>
<body>
  <ticket-list id="list"></ticket-list>
  <p id="log" role="status"></p>
  <script>
    let tickets = [
      { id: 7, title: "Badge reader offline" },
      { id: 9, title: "File server almost full" },
      { id: 11, title: "Suspicious login from abroad" },
    ];
  </script>
</body>
</html>
@@requires
class TicketList extends HTMLElement
set tickets(
new CustomEvent("ticket-close"
composed: true
bubbles: true
detail: { id
addEventListener("ticket-close"
textContent
@@hint
`set tickets(value) { this._tickets = value; this.render(); }`. In `render()`, build `li`s with `document.createElement`, and in each button's click handler: `this.dispatchEvent(new CustomEvent("ticket-close", { detail: { id: t.id }, bubbles: true, composed: true }));`.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Tickets</title></head>
<body>
  <ticket-list id="list"></ticket-list>
  <p id="log" role="status"></p>
  <script>
    let tickets = [
      { id: 7, title: "Badge reader offline" },
      { id: 9, title: "File server almost full" },
      { id: 11, title: "Suspicious login from abroad" },
    ];

    class TicketList extends HTMLElement {
      constructor() {
        super();
        this._tickets = [];
        this.attachShadow({ mode: "open" }).innerHTML = `
          <style>
            ul { list-style: none; padding: 0; margin: 0; }
            li { display: flex; justify-content: space-between; gap: 1rem; padding: .4rem 0; border-bottom: 1px solid #e2e8f0; }
          </style>
          <ul></ul>`;
      }
      get tickets() { return this._tickets; }
      set tickets(value) { this._tickets = value; this.render(); }
      render() {
        const ul = this.shadowRoot.querySelector("ul");
        ul.replaceChildren(...this._tickets.map(t => {
          const li = document.createElement("li");
          const text = document.createElement("span");
          text.textContent = `#${t.id} ${t.title}`;
          const btn = document.createElement("button");
          btn.textContent = "Close";
          btn.setAttribute("aria-label", `Close ticket ${t.id}`);
          btn.addEventListener("click", () => {
            this.dispatchEvent(new CustomEvent("ticket-close", { detail: { id: t.id }, bubbles: true, composed: true }));
          });
          li.append(text, btn);
          return li;
        }));
      }
    }
    customElements.define("ticket-list", TicketList);

    const list = document.getElementById("list");
    list.tickets = tickets;
    document.addEventListener("ticket-close", e => {
      tickets = tickets.filter(t => t.id !== e.detail.id);
      list.tickets = tickets;
      document.getElementById("log").textContent = `Ticket ${e.detail.id} closed`;
    });
  </script>
</body>
</html>

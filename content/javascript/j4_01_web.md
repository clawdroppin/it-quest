@@lesson j4-web The Web: HTML, CSS and the DOM
@@text
Every admin panel, dashboard and status page you'll ever use is HTML + CSS + JavaScript. You don't need to be a designer, but being able to build a simple internal page, or read what a broken one is doing, is a real advantage.

## The three layers

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Server status</title>
  <style>
    body { font-family: system-ui, sans-serif; margin: 2rem; }
    .down { color: white; background: #c0392b; }
    .up { background: #d4efdf; }
  </style>
</head>
<body>
  <h1>Server status</h1>
  <button id="refresh">Refresh</button>
  <ul id="list"></ul>
  <script src="app.js"></script>
</body>
</html>
```

- **HTML** is the structure: headings, lists, buttons, forms.
- **CSS** is the look: `selector { property: value; }`. `.down` selects elements with `class="down"`, `#list` the one with `id="list"`.
- **JavaScript** is the behaviour.

## The DOM

The browser turns the HTML into a tree of objects, the **Document Object Model**, that JavaScript can read and change:

```javascript,norun
// app.js
const list = document.querySelector("#list");
const servers = [{ name: "web01", up: true }, { name: "db01", up: false }];

function render() {
  list.replaceChildren();                       // clear
  for (const s of servers) {
    const li = document.createElement("li");
    li.textContent = `${s.name}: ${s.up ? "up" : "DOWN"}`;   // textContent, not innerHTML!
    li.className = s.up ? "up" : "down";
    list.append(li);
  }
}

document.querySelector("#refresh").addEventListener("click", async () => {
  const res = await fetch("/api/servers");      // ask our own server's API
  servers.splice(0, servers.length, ...(await res.json()));
  render();
});
render();
```

## Why textContent and not innerHTML

`li.innerHTML = name` treats `name` as **HTML**. If a server name, a ticket title or a username contains `<img src=x onerror=alert(document.cookie)>`, that code runs in every viewer's browser. That's **cross-site scripting (XSS)**, one of the most common web vulnerabilities. `textContent` always shows text as text.

## DevTools: your best friend

Press **F12** in any browser:

- **Elements**: the live DOM; edit HTML and CSS to experiment.
- **Console**: errors, and a JavaScript prompt.
- **Network**: every request the page makes, with status codes, headers, timing. When "the website is broken", the Network tab usually says why (a 500 from the API, a blocked request, a slow file).
- **Application**: cookies and local storage.

@@quiz
A ticket system shows titles with `element.innerHTML = ticket.title`. Why is that dangerous?
* A title containing HTML/JavaScript would run in every viewer's browser (XSS)
- It's slower than textContent
- innerHTML doesn't work with text
- It isn't dangerous
? Use `textContent` for text, or escape the HTML.

@@quiz
A user says "the dashboard shows no data". Where in DevTools do you look first to see whether the API call failed?
* The Network tab
- The Elements tab
- The Sources tab
- The Lighthouse tab
? The Network tab shows each request's status code and response.

@@quiz
Which method finds the first element matching a CSS selector?
* `document.querySelector(".alert")`
- `document.getElementsByCss(".alert")`
- `document.find(".alert")`
- `document.select(".alert")`
? querySelectorAll returns every match.

@@drill
Which DOM property sets text safely, without parsing HTML?
= textContent
= .textContent
? innerHTML would run injected markup.

@@flashcards
HTML :: The structure of a page
CSS :: The appearance: selector { property: value; }
DOM :: The browser's tree of objects for a page, changeable from JavaScript
querySelector :: Find an element with a CSS selector, e.g. "#list" or ".down"
addEventListener :: Run a function on an event like "click"
textContent :: Safe way to set text (no HTML parsing)
XSS :: Cross-site scripting: attacker-controlled HTML/JS running in other users' browsers
DevTools Network tab :: Every request a page makes, with status and timing

@@exercise Escape HTML
When you *must* build HTML from text (an email, a server-generated report), escape it. Write `escapeHtml(text)` that replaces `&` `<` `>` `"` and `'` with `&amp;` `&lt;` `&gt;` `&quot;` and `&#39;`.
@@starter
function escapeHtml(text) {
  throw new Error("TODO: not done yet");
}

@@tests
check(escapeHtml("<img src=x onerror=alert(1)>"), "&lt;img src=x onerror=alert(1)&gt;", "an XSS attempt");
check(escapeHtml(`Tom & "Jerry's"`), "Tom &amp; &quot;Jerry&#39;s&quot;", "quotes and ampersands");
check(escapeHtml("&lt;"), "&amp;lt;", "& is replaced first, so nothing is double-escaped wrongly");
@@hint
Replace `&` **first** (otherwise you'd escape the `&` of `&lt;`): `text.replaceAll("&", "&amp;").replaceAll("<", "&lt;")...`. Or one regex with a lookup object: `text.replace(/[&<>"']/g, (c) => map[c])`.
@@solution
function escapeHtml(text) {
  const map = { "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" };
  return text.replace(/[&<>"']/g, (c) => map[c]);
}

@@exercise Render a status table
Write `statusTable(servers)` that returns an HTML table string: a header row `<tr><th>Server</th><th>Status</th></tr>`, then one row per server like `<tr class="down"><td>db01</td><td>DOWN</td></tr>` (class `up`/`down`, text `up`/`DOWN`). Escape server names with the `escapeHtml` function given. Join everything with no spaces or newlines, wrapped in `<table>...</table>`.
@@starter
function escapeHtml(text) {
  const map = { "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" };
  return text.replace(/[&<>"']/g, (c) => map[c]);
}

function statusTable(servers) {
  throw new Error("TODO: not done yet");
}

@@tests
check(
  statusTable([{ name: "web01", up: true }, { name: "db<01>", up: false }]),
  '<table><tr><th>Server</th><th>Status</th></tr><tr class="up"><td>web01</td><td>up</td></tr><tr class="down"><td>db&lt;01&gt;</td><td>DOWN</td></tr></table>',
  "statusTable",
);
@@hint
Build the rows with `servers.map(...)` returning one template literal per server, then `.join("")`, and glue header and rows together inside `<table>` and `</table>`.
@@solution
function escapeHtml(text) {
  const map = { "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" };
  return text.replace(/[&<>"']/g, (c) => map[c]);
}

function statusTable(servers) {
  const header = "<tr><th>Server</th><th>Status</th></tr>";
  const rows = servers
    .map((s) => `<tr class="${s.up ? "up" : "down"}"><td>${escapeHtml(s.name)}</td><td>${s.up ? "up" : "DOWN"}</td></tr>`)
    .join("");
  return `<table>${header}${rows}</table>`;
}

@@lesson h5-dom-state Interactive Pages: the DOM, Events and State
@@concepts objects, functions, input
@@text
Up to now your pages have been documents: the browser shows them and that's it. A **web application** changes while you use it: a list grows, a counter goes up, a form says what's wrong. That's JavaScript working with the **DOM** (Document Object Model), the browser's live tree of the page's elements.

## Finding and changing elements

```js,norun
const title = document.querySelector("h1");            // the first match of a CSS selector
const rows = document.querySelectorAll(".ticket");     // all matches (a NodeList, use for...of)
const list = document.getElementById("tickets");

title.textContent = "Open tickets (3)";                // set text, safely
list.classList.add("compact");                         // classList.add / remove / toggle
list.hidden = true;                                    // the hidden attribute
button.disabled = true;
input.value = "";                                      // a form field's current value
item.dataset.id = "42";                                // sets data-id="42"
```

## Creating elements: textContent, not innerHTML

```js,norun
const li = document.createElement("li");
li.textContent = ticket.title;                // shown as text, whatever it contains
list.append(li);

// DANGER: if a user's ticket title is <img src=x onerror="stealCookies()">, this runs their code.
list.innerHTML += `<li>${ticket.title}</li>`;
```

`innerHTML` with any text that came from a user, a URL or an API is a **cross-site scripting (XSS)** hole, one of the most common web vulnerabilities. Build elements and set `textContent`. For bigger chunks of markup, put a `<template>` in the HTML and clone it.

## Events

```js,norun
button.addEventListener("click", () => { ... });
input.addEventListener("input", () => { ... });     // every keystroke
select.addEventListener("change", () => { ... });   // when a choice is made
form.addEventListener("submit", (event) => {
  event.preventDefault();                           // stop the browser from reloading the page
  ...
});
document.addEventListener("keydown", (event) => {
  if (event.key === "Escape") closeDialog();
});
```

**Event delegation**: instead of a listener on each of 200 rows (and adding listeners to rows created later), put one on the container and ask which row was clicked:

```js,norun
list.addEventListener("click", (event) => {
  const button = event.target.closest("button[data-action]");   // the clicked button, if any
  if (!button) return;
  const id = Number(button.closest("li").dataset.id);
  if (button.dataset.action === "close") closeTicket(id);
});
```

## State → render: the pattern behind every framework

The simplest way to keep a page correct is: keep your data in **one state variable**, and write **one `render()`** that draws the whole UI from it. Events never touch the DOM directly; they change the state and call `render()`.

```html
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Counter</title></head>
<body>
  <p>Coffees today: <strong id="count"></strong></p>
  <button id="more" type="button">+1</button>
  <button id="reset" type="button">Reset</button>
  <p id="note"></p>
  <script>
    let state = { coffees: 0 };                       // the single source of truth

    function render() {                               // draws everything from the state
      document.getElementById("count").textContent = state.coffees;
      document.getElementById("note").textContent = state.coffees >= 4 ? "Maybe switch to water?" : "";
      document.getElementById("reset").disabled = state.coffees === 0;
    }

    document.getElementById("more").addEventListener("click", () => {
      state = { ...state, coffees: state.coffees + 1 };
      render();
    });
    document.getElementById("reset").addEventListener("click", () => {
      state = { ...state, coffees: 0 };
      render();
    });
    render();
  </script>
</body>
</html>
```

With this pattern, "the counter says 3 but the warning still shows" bugs can't happen: everything is drawn from the same data. React, Vue, Svelte and Angular are all, at heart, a faster and more convenient `render()`.

## Forms

```js,norun
form.addEventListener("submit", (event) => {
  event.preventDefault();
  const data = new FormData(form);              // every named field
  const email = data.get("email").trim();

  if (!form.checkValidity()) {                  // uses required, type="email", min, pattern...
    form.reportValidity();                      // shows the browser's own messages
    return;
  }
  save({ email, priority: data.get("priority") });
  form.reset();
});
```

The HTML attributes (`required`, `type="email"`, `minlength`, `pattern`) do most validation for free. But **always validate again on the server**: anyone can bypass the browser.

@@quiz
A ticket's title comes from a user. Which line is safe?
* `li.textContent = ticket.title;`
- `li.innerHTML = ticket.title;`
- `list.innerHTML += "<li>" + ticket.title + "</li>";`
- `document.write(ticket.title);`
? textContent never interprets HTML. The others run any script hidden in the title (XSS).

@@quiz
Why call `event.preventDefault()` in a form's submit handler?
- To stop other listeners
* To stop the browser's default action: sending the form and loading a new page
- To clear the form
- To validate the fields
? Without it, your JavaScript runs and then the page reloads, losing everything.

@@quiz
What is event delegation?
- Running events in a web worker
* One listener on a container that handles events from its children (using `event.target`)
- Passing an event to another page
- Calling `dispatchEvent`
? Events bubble up, so one listener on the list sees clicks on every row, including rows added later.

@@quiz
In the state → render pattern, what does a click handler do?
- Changes the right elements directly
* Updates the state and calls `render()`
- Reloads the page
- Calls `innerHTML` on the body
? The DOM is always drawn from the state, so it can never disagree with it.

@@drill
Which method finds the first element matching a CSS selector?
= querySelector
= document.querySelector
? querySelectorAll finds them all.

@@flashcards
DOM :: the browser's live tree of the page's elements
`querySelector` / `querySelectorAll` :: find one / all elements by CSS selector
`textContent` :: an element's text; setting it is XSS-safe
`innerHTML` :: parses HTML: never with untrusted text
`createElement` + `append` :: build and add elements
`classList.toggle("x")` :: add or remove a class
`dataset.id` :: reads/writes the `data-id` attribute
`addEventListener("click", fn)` :: run fn on each click
`event.preventDefault()` :: cancel the browser's default action
`event.target.closest(sel)` :: the nearest ancestor (or itself) matching a selector
`new FormData(form)` :: all named fields of a form
`form.checkValidity()` :: are all the HTML validation rules met?

@@exercise Render from state
The page has a `tickets` array and an empty list. Write `render()` so it fills `#tickets` with one `<li>` per ticket showing `#<id> <title>` (use `textContent`, not `innerHTML`: one of the titles is an attack), and sets `#count` to the number of tickets. Call it once at the end.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Tickets</title></head>
<body>
  <h1>Open tickets: <span id="count"></span></h1>
  <ul id="tickets"></ul>
  <script>
    const tickets = [
      { id: 101, title: "Printer on 3F is offline" },
      { id: 102, title: "VPN drops every hour" },
      { id: 103, title: "<img src=x onerror=alert('hacked')> laptop slow" },
    ];

    function render() {
      // TODO
    }
  </script>
</body>
</html>
@@requires
createElement
textContent
render();
@@hint
`const list = document.getElementById("tickets"); list.replaceChildren();` then `for (const t of tickets) { const li = document.createElement("li"); li.textContent = "#" + t.id + " " + t.title; list.append(li); }`, and `document.getElementById("count").textContent = tickets.length;`. Finally call `render();` after the function.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Tickets</title></head>
<body>
  <h1>Open tickets: <span id="count"></span></h1>
  <ul id="tickets"></ul>
  <script>
    const tickets = [
      { id: 101, title: "Printer on 3F is offline" },
      { id: 102, title: "VPN drops every hour" },
      { id: 103, title: "<img src=x onerror=alert('hacked')> laptop slow" },
    ];

    function render() {
      const list = document.getElementById("tickets");
      list.replaceChildren();
      for (const t of tickets) {
        const li = document.createElement("li");
        li.textContent = `#${t.id} ${t.title}`;
        list.append(li);
      }
      document.getElementById("count").textContent = tickets.length;
    }

    render();
  </script>
</body>
</html>

@@exercise A form that adds to the list
Make the form work: on submit, **prevent the page reload**, read the title with `FormData`, ignore it if it's blank after trimming, otherwise add it to `items` and call `render()`, then `form.reset()`. Use `required` on the input as well.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Shopping list</title></head>
<body>
  <form id="add">
    <label>Item <input name="title"></label>
    <button>Add</button>
  </form>
  <ul id="list"></ul>
  <script>
    let items = ["HDMI cables", "Toner"];
    const form = document.getElementById("add");

    function render() {
      const list = document.getElementById("list");
      list.replaceChildren(...items.map((text) => {
        const li = document.createElement("li");
        li.textContent = text;
        return li;
      }));
    }

    // TODO: handle the submit event

    render();
  </script>
</body>
</html>
@@requires
preventDefault()
new FormData(
required
form.reset()
@@hint
`form.addEventListener("submit", (event) => { event.preventDefault(); const title = new FormData(form).get("title").trim(); if (!title) return; items = [...items, title]; render(); form.reset(); });` and `<input name="title" required>`.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Shopping list</title></head>
<body>
  <form id="add">
    <label>Item <input name="title" required></label>
    <button>Add</button>
  </form>
  <ul id="list"></ul>
  <script>
    let items = ["HDMI cables", "Toner"];
    const form = document.getElementById("add");

    function render() {
      const list = document.getElementById("list");
      list.replaceChildren(...items.map((text) => {
        const li = document.createElement("li");
        li.textContent = text;
        return li;
      }));
    }

    form.addEventListener("submit", (event) => {
      event.preventDefault();
      const title = new FormData(form).get("title").trim();
      if (!title) return;
      items = [...items, title];
      render();
      form.reset();
    });

    render();
  </script>
</body>
</html>

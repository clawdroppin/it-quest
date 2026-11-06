@@lesson h1-forms Forms
@@concepts input
@@text
## A form

Forms collect input and send it to a server: logins, searches, ticket requests.

```html
<form action="/tickets" method="post">
  <p>
    <label for="name">Your name</label><br>
    <input id="name" name="name" type="text" required autocomplete="name">
  </p>
  <p>
    <label for="email">Email</label><br>
    <input id="email" name="email" type="email" required placeholder="you@example.jp">
  </p>
  <p>
    <label for="category">Category</label><br>
    <select id="category" name="category">
      <option value="hardware">Hardware</option>
      <option value="account" selected>Account / password</option>
      <option value="network">Network</option>
    </select>
  </p>
  <fieldset>
    <legend>Priority</legend>
    <label><input type="radio" name="priority" value="low" checked> Low</label>
    <label><input type="radio" name="priority" value="high"> High: I can't work</label>
  </fieldset>
  <p>
    <label for="details">What happened?</label><br>
    <textarea id="details" name="details" rows="4" cols="40" minlength="10"></textarea>
  </p>
  <p><label><input type="checkbox" name="remote"> I'm working from home</label></p>
  <button type="submit">Send ticket</button>
</form>
```

- `action` is the URL the data is sent to; `method="post"` sends it in the request body (use `get` only for searches: GET data ends up in the URL, browser history and server logs).
- **`name`** is the key the server receives (`email=you@example.jp`). An input without a `name` isn't sent at all.
- **`<label>`** says what the field is. Connect it with `for` = the input's `id`, or by wrapping the input. Clicking the label then focuses the field, and screen readers read it. Placeholder text is *not* a label.
- Radio buttons with the same `name` form one group: only one can be chosen.

## Input types

| `type` | Gives you |
|---|---|
| `text`, `password` | text (password hides it) |
| `email`, `url`, `tel` | text with checks; the right keyboard on phones |
| `number` | `min`, `max`, `step` |
| `date`, `time`, `datetime-local` | a date or time picker |
| `checkbox`, `radio` | on/off, or one of several |
| `file` | upload; add `accept=".csv,.txt"` |
| `hidden` | a value sent with the form but not shown |

## Built-in validation

```html
<form>
  <label for="ip">Server IP</label>
  <input id="ip" name="ip" required pattern="\d{1,3}(\.\d{1,3}){3}" title="Four numbers separated by dots">
  <label for="port">Port</label>
  <input id="port" name="port" type="number" min="1" max="65535" value="443">
  <button>Check</button>
</form>
```

`required`, `min`/`max`, `minlength`/`maxlength`, `pattern` (a regular expression) and `type="email"` make the browser refuse bad input before it's sent.

**But never trust it.** Anyone can turn validation off in the developer tools, or send a request with `curl`. The server must check everything again. Browser validation is only for the user's convenience; security happens on the server.

## Passwords and forms

- `type="password"` hides what's typed on screen, but it's sent as plain text unless the page uses **HTTPS**. Never put a login form on an `http://` page.
- `autocomplete="current-password"` / `"new-password"` helps password managers, which helps security.
- A phishing page is just an HTML form copied from the real one, with an `action` pointing at the attacker. Check the address bar, not the look of the page.

@@quiz
An `<input>` has `id="user"` but no `name`. What does the server receive for it?
* Nothing: inputs without a `name` aren't sent
- `user=<value>`
- `id=<value>`
- An error
? `name` is the key in the submitted data. `id` is for labels, CSS and scripts.

@@quiz
Your form uses `required` and `pattern`. Does the server still need to validate?
* Yes: anyone can bypass browser checks and send any data
- No, the browser already checked
- Only for passwords
- Only if JavaScript is disabled
? Client-side validation is a convenience. Server-side validation is the security.

@@quiz
Why use `method="post"` for a login form rather than `get`?
* GET puts the data in the URL, where it's saved in history and server logs
- POST is encrypted, GET isn't
- GET can't send passwords
- POST is faster
? Neither is encrypted by itself: HTTPS does that. But URLs get logged and shared.

@@quiz
How do you connect a `<label>` to its input?
* `for` on the label = `id` on the input, or put the input inside the label
- `name` on both
- Put them next to each other
- `aria-input="true"`
? A connected label can be clicked to focus the field, and is read by screen readers.

@@flashcards
<form action method> :: where and how to send data
name :: the key the server receives
<label for="id"> :: connect a label to an input
type="email" / "number" / "date" :: specialised inputs
required :: field must be filled in
pattern :: regular expression the value must match
autocomplete :: helps browsers and password managers fill fields
method="post" :: send data in the request body

@@exercise Account request form
Make a form that posts to `/accounts` with: a required text input `fullname` with a label, a required email input `email` with a label, a `<select>` named `dept` with options `sales`, `it` and `hr`, and a submit button. Every input needs an `id` matching its label's `for`.
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>New account</title>
</head>
<body>
  <h1>Request a new account</h1>
</body>
</html>
@@requires
<form action="/accounts" method="post">
<label for="fullname">
id="fullname" name="fullname"
<label for="email">
type="email"
<select id="dept" name="dept">
<option value="it">
<button type="submit">
required
@@hint
For example: `<label for="fullname">Full name</label> <input id="fullname" name="fullname" type="text" required>`. The select: `<label for="dept">Department</label> <select id="dept" name="dept"><option value="sales">Sales</option>…</select>`.
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>New account</title>
</head>
<body>
  <h1>Request a new account</h1>
  <form action="/accounts" method="post">
    <p>
      <label for="fullname">Full name</label><br>
      <input id="fullname" name="fullname" type="text" required>
    </p>
    <p>
      <label for="email">Email</label><br>
      <input id="email" name="email" type="email" required>
    </p>
    <p>
      <label for="dept">Department</label><br>
      <select id="dept" name="dept">
        <option value="sales">Sales</option>
        <option value="it">IT</option>
        <option value="hr">HR</option>
      </select>
    </p>
    <button type="submit">Request account</button>
  </form>
</body>
</html>

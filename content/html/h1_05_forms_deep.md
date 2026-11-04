@@lesson h1-forms-deep Forms in Depth: Input Types, Validation and Autocomplete
@@text
Forms are where users give you data, and where most frustration on the web happens. HTML alone gives you a lot: the right keyboard on phones, built-in validation, password managers that work, and accessibility, before writing a line of JavaScript.

## Pick the right input type

| Type | Gives you |
|---|---|
| `email` | email keyboard on phones, basic format validation |
| `tel` | phone keypad (no validation: phone formats vary) |
| `url` | URL keyboard and validation |
| `number` | numeric keypad, `min`/`max`/`step`, spin buttons (for quantities, not for IDs or card numbers) |
| `date`, `time`, `datetime-local`, `month`, `week` | native pickers, ISO values |
| `search` | search keyboard, clear button |
| `password` | hidden text; password managers notice it |
| `checkbox`, `radio` | on/off and one-of-many |
| `range` | a slider |
| `color` | a colour picker |
| `file` | file chooser; `accept=".pdf,image/*"`, `multiple` |
| `hidden` | a value sent with the form, not shown |

For numbers that aren't amounts (employee IDs, postcodes, card numbers) use `type="text" inputmode="numeric"`: the numeric keyboard without spinners or rounding.

## Built-in validation

| Attribute | Rule |
|---|---|
| `required` | must not be empty |
| `minlength`, `maxlength` | text length |
| `min`, `max`, `step` | numbers and dates |
| `pattern="[A-Z]{2}-\d{4}"` | a regular expression the **whole** value must match |
| `type="email"` / `url` | format |

The browser blocks submission and shows a message. CSS can style states with `:invalid`, `:valid`, `:user-invalid` (only after the user has interacted, much friendlier), `:required` and `:placeholder-shown`. Add `novalidate` on the form when you take over validation in JavaScript.

**Client-side validation is for convenience only.** Anyone can bypass it (DevTools, curl), so the server must validate everything again.

## Labels and help text

- Every control needs a `<label>`: either wrapping it, or `for="id"` matching the input's `id`. Clicking the label focuses the input; screen readers read it.
- **Placeholder is not a label**: it disappears when typing, has low contrast and isn't reliably announced. Use it only for an example (`placeholder="LT-0042"`).
- Hints go in a separate element connected with `aria-describedby="hint-id"`, so they're announced with the field.

## Grouping

`<fieldset>` with a `<legend>` groups related controls: essential for radio buttons ("Priority: Low / Normal / High"), useful for sections (Shipping address). `<select>` options can be grouped with `<optgroup label="...">`.

## Suggestions: datalist

`<input list="models">` with `<datalist id="models"><option value="ThinkPad T14">...</datalist>` suggests values but still allows anything. A combo box with zero JavaScript.

## Autocomplete: let the browser fill it

The `autocomplete` attribute tells browsers and password managers exactly what a field is: `name`, `given-name`, `family-name`, `email`, `tel`, `organization`, `street-address`, `postal-code`, `country`, `username`, `current-password`, `new-password`, `one-time-code`. It saves users minutes and is a WCAG requirement (identify input purpose). Use `autocomplete="off"` rarely; browsers ignore it for passwords anyway.

## Buttons

- `<button>` inside a form defaults to `type="submit"`. Any other button (show password, add row) must say `type="button"`, or it submits the form.
- `type="reset"` clears the form; it's almost never what users want.
- The form's `action` is where data goes, `method="post"` for anything that changes data (and never GET for passwords: GET puts values in the URL, logs and history).

@@quiz
Which input type is best for an employee ID like `004217`?
- `type="number"`
* `type="text"` with `inputmode="numeric"`
- `type="tel"`
- `type="range"`
? number inputs drop leading zeros and add spinners.

@@quiz
Why isn't a placeholder a replacement for a label?
* It disappears while typing, often has low contrast and isn't reliably announced
- Placeholders can't contain text
- Labels are only for checkboxes
- Placeholders break validation
? Use placeholder only for an example value.

@@quiz
A "Show password" `<button>` inside a form submits the form when clicked. What's the fix?
- Move it outside the body
* Add `type="button"`
- Add `novalidate`
- Use a link instead
? Buttons in forms default to type="submit".

@@quiz
Why must the server validate again?
- Browsers validate incorrectly
* Client-side checks can be bypassed by anyone sending requests directly
- To slow down attackers
- HTML validation only works offline
? Client-side validation is a convenience, not security.

@@flashcards
`type="email"` :: email keyboard + format check
`inputmode="numeric"` :: numeric keyboard on a text field
`required` :: must not be empty
`pattern` :: regex for the whole value
`:user-invalid` :: invalid after interaction
`novalidate` :: turn off built-in validation
`<label for>` :: connect a label to an input id
`aria-describedby` :: connect hint text
`<fieldset>`/`<legend>` :: group controls (radios!)
`<datalist>` :: suggestions, any value allowed
`autocomplete="email"` :: tell the browser what the field is
`autocomplete="new-password"` :: password managers suggest a new one
`type="button"` :: a button that doesn't submit

@@exercise A new-hire request form
Build the request form with the right types and validation:

- `Full name` (text, required, `autocomplete="name"`)
- `Work email` (email, required, `autocomplete="email"`)
- `Employee ID`: exactly 6 digits, as text with `inputmode="numeric"` and `pattern="\d{6}"`, with the hint "6 digits, from your contract" connected by `aria-describedby="id-hint"`
- `Start date` (date, required, `min="2026-11-01"`)
- `Laptop`: radio buttons `Windows` and `Mac` in a `<fieldset>` with the legend `Laptop`
- a submit button; the form posts to `/requests`
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>New hire</title></head>
<body>
  <h1>New hire request</h1>
  <form>
    Full name <input>
    Email <input>
    Employee ID <input>
    Start date <input>
    Laptop: Windows or Mac? <input>
    <button>Send</button>
  </form>
</body>
</html>
@@requires
method="post"
type="email"
inputmode="numeric"
pattern="\d{6}"
aria-describedby="id-hint"
type="date"
min="2026-11-01"
<fieldset>
<legend>Laptop</legend>
type="radio"
autocomplete="name"
@@hint
Each field: `<label for="email">Work email</label> <input id="email" name="email" type="email" required autocomplete="email">`. Hint: `<input id="empid" ... aria-describedby="id-hint"> <small id="id-hint">6 digits, from your contract</small>`. Radios share a `name`: `<label><input type="radio" name="laptop" value="windows" required> Windows</label>`.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>New hire</title></head>
<body>
  <h1>New hire request</h1>
  <form action="/requests" method="post">
    <p>
      <label for="name">Full name</label>
      <input id="name" name="name" required autocomplete="name">
    </p>
    <p>
      <label for="email">Work email</label>
      <input id="email" name="email" type="email" required autocomplete="email">
    </p>
    <p>
      <label for="empid">Employee ID</label>
      <input id="empid" name="employee_id" inputmode="numeric" pattern="\d{6}" required aria-describedby="id-hint">
      <small id="id-hint">6 digits, from your contract</small>
    </p>
    <p>
      <label for="start">Start date</label>
      <input id="start" name="start" type="date" required min="2026-11-01">
    </p>
    <fieldset>
      <legend>Laptop</legend>
      <label><input type="radio" name="laptop" value="windows" required> Windows</label>
      <label><input type="radio" name="laptop" value="mac"> Mac</label>
    </fieldset>
    <button type="submit">Send</button>
  </form>
</body>
</html>

@@exercise Friendly validation styles
Style the asset form so invalid fields get a red border and a message **only after the user has interacted** (`:user-invalid`), valid fields get a green border, and the asset tag must match `LT-1234` style (`pattern="[A-Z]{2}-\d{4}"`). Use a `<datalist>` to suggest the models `ThinkPad T14`, `ThinkPad X1` and `MacBook Air`. The message element after each input should only be shown when that input is user-invalid (`input:user-invalid + .error { display: block; }`).
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Register an asset</title>
<style>
  .error { display: none; color: #b91c1c; }
</style>
</head>
<body>
  <form>
    <p><label for="tag">Asset tag</label>
       <input id="tag" name="tag" required placeholder="LT-0042">
       <span class="error">Use two capital letters, a dash and four digits.</span></p>
    <p><label for="model">Model</label>
       <input id="model" name="model" required>
       <span class="error">Enter the model.</span></p>
    <button>Register</button>
  </form>
</body>
</html>
@@requires
pattern="[A-Z]{2}-\d{4}"
:user-invalid
:valid
input:user-invalid + .error
<datalist id="models">
list="models"
@@hint
CSS: `input:user-invalid { border: 2px solid #b91c1c; } input:valid { border: 2px solid #15803d; } input:user-invalid + .error { display: block; }`. Datalist: `<input id="model" name="model" list="models" required> ... <datalist id="models"><option value="ThinkPad T14"></option>...</datalist>`.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Register an asset</title>
<style>
  .error { display: none; color: #b91c1c; }
  input:valid { border: 2px solid #15803d; }
  input:user-invalid { border: 2px solid #b91c1c; }
  input:user-invalid + .error { display: block; }
</style>
</head>
<body>
  <form>
    <p><label for="tag">Asset tag</label>
       <input id="tag" name="tag" required placeholder="LT-0042" pattern="[A-Z]{2}-\d{4}">
       <span class="error">Use two capital letters, a dash and four digits.</span></p>
    <p><label for="model">Model</label>
       <input id="model" name="model" list="models" required>
       <span class="error">Enter the model.</span></p>
    <datalist id="models">
      <option value="ThinkPad T14"></option>
      <option value="ThinkPad X1"></option>
      <option value="MacBook Air"></option>
    </datalist>
    <button type="submit">Register</button>
  </form>
</body>
</html>

@@exercise A sign-in form password managers love
Fix the sign-in form: label both fields, `autocomplete="username"` and `autocomplete="current-password"`, `method="post"` (never put a password in the URL), and add a "Show password" button that toggles the field's type between `password` and `text` **without submitting** the form (it needs `type="button"`). Update the button's `aria-pressed` to `true`/`false`.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Sign in</title></head>
<body>
  <form action="/login">
    <input name="u" placeholder="Username">
    <input name="p" type="password" placeholder="Password">
    <button id="show">Show password</button>
    <button>Sign in</button>
  </form>
  <script>
    // TODO: toggle the password field
  </script>
</body>
</html>
@@requires
method="post"
autocomplete="username"
autocomplete="current-password"
type="button"
aria-pressed
addEventListener
@@hint
`const pw = document.getElementById("password"); const show = document.getElementById("show"); show.addEventListener("click", () => { const visible = pw.type === "password"; pw.type = visible ? "text" : "password"; show.setAttribute("aria-pressed", String(visible)); });`
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Sign in</title></head>
<body>
  <form action="/login" method="post">
    <p><label for="username">Username</label>
       <input id="username" name="username" required autocomplete="username"></p>
    <p><label for="password">Password</label>
       <input id="password" name="password" type="password" required autocomplete="current-password">
       <button type="button" id="show" aria-pressed="false">Show password</button></p>
    <button type="submit">Sign in</button>
  </form>
  <script>
    const pw = document.getElementById("password");
    const show = document.getElementById("show");
    show.addEventListener("click", () => {
      const visible = pw.type === "password";
      pw.type = visible ? "text" : "password";
      show.setAttribute("aria-pressed", String(visible));
    });
  </script>
</body>
</html>

@@lesson n7-forms Form UX: The Constraint Validation API and Accessible Errors
@@text
Module 1 covered HTML's built-in validation (`required`, `type="email"`, `pattern`). Built-in error bubbles are inconsistent across browsers, disappear quickly, aren't styled like your site and are announced poorly by some screen readers. Production forms usually keep the HTML constraints (they're free and work without JavaScript) and add their own messages using the **Constraint Validation API**.

## The validity API

Every input has a `validity` object describing what's wrong:

```js,norun
const email = document.querySelector("#email");
email.validity.valid           // true if all constraints pass
email.validity.valueMissing    // required but empty
email.validity.typeMismatch    // not a valid email/url
email.validity.patternMismatch // doesn't match pattern
email.validity.tooShort        // shorter than minlength
email.validity.rangeUnderflow  // below min
email.validity.customError     // setCustomValidity was called
email.validationMessage        // the browser's message, localised
email.checkValidity()          // returns valid, fires "invalid" if not
form.reportValidity()          // shows messages and focuses the first bad field
email.setCustomValidity("Use your company address")  // "" to clear
```

## Turning off the bubbles, keeping the rules

```html
<form novalidate>
```

`novalidate` stops the browser's own pop-ups but keeps `validity` working, so your script decides how to show errors.

## Accessible error messages

A good error message is: **visible** next to the field, **associated** with it so screen readers announce it, **specific** ("Enter an email like ana@company.jp", not "Invalid input"), and doesn't rely on colour alone.

```html
<label for="email">Work email</label>
<input id="email" type="email" required aria-describedby="email-hint email-error">
<p id="email-hint" class="hint">We'll send the ticket updates here.</p>
<p id="email-error" class="error" hidden></p>
```

When invalid: put the message in `#email-error`, show it, and set `aria-invalid="true"` on the input. `aria-describedby` makes screen readers read the hint and error after the label.

## When to validate

- **On submit**: always. Show all errors, move focus to the first invalid field (or to an error summary).
- **On blur** (leaving a field): good for format checks; the user has finished typing.
- **On input** (every keystroke): only to **clear** an error once fixed. Showing "invalid email" while someone is still typing "ana@" is hostile.

## Error summary

For long forms, show a summary at the top listing every problem as a link to its field:

```html
<div role="alert" id="summary" tabindex="-1">
  <h2>There are 2 problems</h2>
  <ul><li><a href="#email">Enter a valid work email</a></li><li><a href="#device">Choose a device</a></li></ul>
</div>
```

Focus the summary after a failed submit. This pattern comes from the UK government's design system (GOV.UK), which tests its forms heavily with real users.

## CSS for validation states

```css
input:user-invalid { border-color: #dc2626; }   /* only after the user interacted */
input:invalid { }                                 /* immediately, even on page load: usually too eager */
[aria-invalid="true"] { border-color: #dc2626; }
.error { color: #b91c1c; }
.error::before { content: "⚠ "; }                 /* not colour alone */
```

`:user-invalid` (all current browsers) solves the "form is red before I've typed anything" problem.

## Other form UX rules

- **Labels always visible**: placeholders disappear on input and have poor contrast.
- **Right input types and `autocomplete`**: `type="email"`, `inputmode="numeric"`, `autocomplete="email"`, `autocomplete="one-time-code"` for 2FA codes; phones show the right keyboard and browsers autofill.
- **Don't block paste** in password or email fields (it breaks password managers).
- **Mark optional fields** rather than required ones when most are required.
- **Disable the submit button only while submitting**, and say what's happening ("Sending…").
- **Keep the user's input** after a server error. Never clear the form.
- **Server-side validation is mandatory**: client-side checks are for convenience only; anyone can bypass them.

@@quiz
What does `novalidate` on a form do?
- Turns off validation entirely, including `validity`
* Stops the browser's own error pop-ups while `validity` still works for your script
- Validates on the server
- Makes all fields optional
? You keep the rules and control the display.

@@quiz
How should an error message be connected to its input for screen readers?
* `aria-describedby` pointing at the message's id, plus `aria-invalid="true"`
- A tooltip
- Red border only
- `title` attribute
? It's read after the label when the field is focused.

@@quiz
When should a field show "invalid email"?
- On every keystroke from the first character
* After the user leaves the field or submits; on input only to clear a fixed error
- Never
- Only after 10 seconds
? Don't shout at people while they're still typing.

@@quiz
Why is client-side validation never enough on its own?
- Browsers don't support it
* Anyone can bypass it (dev tools, curl), so the server must validate too
- It's too slow
- It doesn't work offline
? Client checks are for convenience.

@@flashcards
`validity.valid` :: all constraints pass
`valueMissing` :: required and empty
`typeMismatch` :: wrong email/url format
`setCustomValidity(msg)` :: custom error ("" clears)
`reportValidity()` :: show errors, focus the first
`novalidate` :: no browser bubbles
`aria-describedby` :: link hint/error to the field
`aria-invalid="true"` :: mark the field invalid
`:user-invalid` :: invalid after interaction
error summary :: list of links to bad fields
`autocomplete="one-time-code"` :: 2FA code autofill
server-side validation :: always required

@@exercise Inline errors with the validity API
Add `novalidate` to the form and write the script: on submit, check each field with `validity`. For an invalid field, show a specific message in its error element (`valueMissing`: "Enter your work email"; `typeMismatch`: "Enter an email like ana@company.jp"; for the code field `patternMismatch`: "The asset tag is 2 letters and 4 digits, like LT-0042"), set `aria-invalid="true"`, and focus the first invalid field. Each input's `aria-describedby` points to its error element. On `input`, clear the error once the field becomes valid.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Request</title>
<style>.error { color: #b91c1c; } [aria-invalid="true"] { border-color: #dc2626; }</style>
</head>
<body>
  <form id="f">
    <label for="email">Work email</label>
    <input id="email" type="email" required>
    <p id="email-error" class="error" hidden></p>
    <label for="tag">Asset tag</label>
    <input id="tag" required pattern="[A-Z]{2}-[0-9]{4}">
    <p id="tag-error" class="error" hidden></p>
    <button>Send</button>
  </form>
</body>
</html>
@@requires
novalidate
aria-describedby="email-error"
aria-describedby="tag-error"
.valueMissing
.typeMismatch
.patternMismatch
setAttribute("aria-invalid", "true")
preventDefault()
.focus()
@@hint
Write `messageFor(input)` that returns "" when `input.validity.valid` and otherwise picks the message by flag. On submit: `e.preventDefault()`, loop over the inputs, show messages, remember the first invalid one and focus it. On input: if the field is now valid, hide its error and remove `aria-invalid`.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Request</title>
<style>.error { color: #b91c1c; } .error::before { content: "⚠ "; } [aria-invalid="true"] { border-color: #dc2626; }</style>
</head>
<body>
  <form id="f" novalidate>
    <label for="email">Work email</label>
    <input id="email" type="email" required aria-describedby="email-error">
    <p id="email-error" class="error" hidden></p>
    <label for="tag">Asset tag</label>
    <input id="tag" required pattern="[A-Z]{2}-[0-9]{4}" aria-describedby="tag-error">
    <p id="tag-error" class="error" hidden></p>
    <button>Send</button>
    <p role="status" id="done"></p>
  </form>
  <script>
    const form = document.getElementById("f");
    const fields = [...form.querySelectorAll("input")];
    function messageFor(input) {
      const v = input.validity;
      if (v.valid) return "";
      if (input.id === "email") {
        if (v.valueMissing) return "Enter your work email";
        if (v.typeMismatch) return "Enter an email like ana@company.jp";
      }
      if (input.id === "tag") {
        if (v.valueMissing) return "Enter the asset tag";
        if (v.patternMismatch) return "The asset tag is 2 letters and 4 digits, like LT-0042";
      }
      return input.validationMessage;
    }
    function show(input) {
      const msg = messageFor(input);
      const err = document.getElementById(input.id + "-error");
      err.textContent = msg;
      err.hidden = !msg;
      if (msg) input.setAttribute("aria-invalid", "true");
      else input.removeAttribute("aria-invalid");
      return !msg;
    }
    form.addEventListener("submit", e => {
      e.preventDefault();
      let first = null;
      for (const input of fields) {
        if (!show(input) && !first) first = input;
      }
      if (first) first.focus();
      else document.getElementById("done").textContent = "Request sent.";
    });
    for (const input of fields) {
      input.addEventListener("input", () => { if (input.hasAttribute("aria-invalid")) show(input); });
    }
  </script>
</body>
</html>

@@exercise A custom rule with setCustomValidity
The "Confirm new password" field must match the first password, and the password must not contain the user's username. Use `setCustomValidity()` on `input` events: set a message when the rule fails and `""` when it passes, so `checkValidity()` and `:user-invalid` work for your custom rules too. Show the message in an element linked with `aria-describedby`. Style invalid fields with `:user-invalid` (not `:invalid`).
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Password</title></head>
<body>
  <form>
    <label for="user">Username</label> <input id="user" value="asilva" readonly>
    <label for="pw">New password</label> <input id="pw" type="password" minlength="12" required autocomplete="new-password">
    <label for="pw2">Confirm new password</label> <input id="pw2" type="password" required autocomplete="new-password">
    <button>Change password</button>
  </form>
</body>
</html>
@@requires
setCustomValidity(
setCustomValidity("")
:user-invalid
aria-describedby="pw-error"
aria-describedby="pw2-error"
@@hint
`function check() { pw.setCustomValidity(pw.value.toLowerCase().includes(user.value.toLowerCase()) ? "The password must not contain your username" : ""); pw2.setCustomValidity(pw2.value && pw2.value !== pw.value ? "The passwords don't match" : ""); pwErr.textContent = pw.validationMessage; ... }` and call it on `input` for both fields.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Password</title>
<style>
  input:user-invalid { border: 2px solid #dc2626; }
  .error { color: #b91c1c; min-height: 1.2em; margin: 0; }
</style>
</head>
<body>
  <form>
    <label for="user">Username</label> <input id="user" value="asilva" readonly>
    <label for="pw">New password</label> <input id="pw" type="password" minlength="12" required autocomplete="new-password" aria-describedby="pw-error">
    <p id="pw-error" class="error"></p>
    <label for="pw2">Confirm new password</label> <input id="pw2" type="password" required autocomplete="new-password" aria-describedby="pw2-error">
    <p id="pw2-error" class="error"></p>
    <button>Change password</button>
  </form>
  <script>
    const user = document.getElementById("user");
    const pw = document.getElementById("pw");
    const pw2 = document.getElementById("pw2");
    function check() {
      if (pw.value && pw.value.toLowerCase().includes(user.value.toLowerCase())) {
        pw.setCustomValidity("The password must not contain your username");
      } else {
        pw.setCustomValidity("");
      }
      if (pw2.value && pw2.value !== pw.value) {
        pw2.setCustomValidity("The passwords don't match");
      } else {
        pw2.setCustomValidity("");
      }
      document.getElementById("pw-error").textContent = pw.value ? pw.validationMessage : "";
      document.getElementById("pw2-error").textContent = pw2.value ? pw2.validationMessage : "";
    }
    pw.addEventListener("input", check);
    pw2.addEventListener("input", check);
  </script>
</body>
</html>

@@exercise An error summary
On a failed submit, build an error summary at the top of the form: a `role="alert"` container with `tabindex="-1"`, a heading "There are N problems", and a list of links (`href="#fieldId"`) to each invalid field with its message. Focus the summary. Hide it again when the form is valid.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Summary</title></head>
<body>
  <form id="f" novalidate>
    <label for="name">Full name</label> <input id="name" required>
    <label for="email">Work email</label> <input id="email" type="email" required>
    <label for="device">Device</label>
    <select id="device" required><option value="">Choose…</option><option>Laptop</option><option>Phone</option></select>
    <button>Submit</button>
  </form>
</body>
</html>
@@requires
role="alert"
tabindex="-1"
problems
href
summary.focus()
checkValidity()
@@hint
On submit: `const bad = fields.filter(f => !f.checkValidity());`. If there are any, fill `summary` with `<h2>There are ${bad.length} problems</h2>` and a `<ul>` of `<li><a href="#${f.id}">${label}: ${f.validationMessage}</a></li>`, unhide it and call `summary.focus()`. Build the list with DOM methods or escape the text if it could contain user input.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Summary</title>
<style>#summary { border: 3px solid #b91c1c; padding: 0.5rem 1rem; } #summary:focus { outline: 3px solid #f59e0b; }</style>
</head>
<body>
  <form id="f" novalidate>
    <div id="summary" role="alert" tabindex="-1" hidden></div>
    <label for="name">Full name</label> <input id="name" required>
    <label for="email">Work email</label> <input id="email" type="email" required>
    <label for="device">Device</label>
    <select id="device" required><option value="">Choose…</option><option>Laptop</option><option>Phone</option></select>
    <button>Submit</button>
  </form>
  <script>
    const form = document.getElementById("f");
    const summary = document.getElementById("summary");
    const fields = [...form.querySelectorAll("input, select")];
    form.addEventListener("submit", e => {
      e.preventDefault();
      const bad = fields.filter(f => !f.checkValidity());
      summary.replaceChildren();
      if (bad.length === 0) { summary.hidden = true; return; }
      const h = document.createElement("h2");
      h.textContent = `There ${bad.length === 1 ? "is 1 problem" : `are ${bad.length} problems`}`;
      const ul = document.createElement("ul");
      for (const f of bad) {
        const li = document.createElement("li");
        const a = document.createElement("a");
        a.href = "#" + f.id;
        a.textContent = `${form.querySelector(`label[for="${f.id}"]`).textContent}: ${f.validationMessage}`;
        li.append(a);
        ul.append(li);
      }
      summary.append(h, ul);
      summary.hidden = false;
      summary.focus();
    });
  </script>
</body>
</html>

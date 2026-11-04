@@lesson n7-project Project: An Accessible, Fast IT Request Form
@@text
This project brings the module together into something every company has: the form staff use to ask IT for things (a new laptop, software, access to a shared drive). It's a page that gets used by everyone, on every device, often in a hurry. That makes accessibility, clear errors and speed matter more than visual flair.

## Requirements

**Content and structure**
- A proper `<head>`: title, description, `noindex` (it's internal), viewport, `lang`.
- A skip link, a header with the site name, a `<main>` with one `h1`.
- Fields grouped with `<fieldset>`/`<legend>`: "About you" (full name, work email, department), "Request" (type as radio buttons, details textarea, needed-by date), "Approval" (manager email, a cost-centre code with a pattern).

**Form behaviour**
- `novalidate` with custom messages via the validity API.
- Errors appear on blur and submit, clear on input once fixed, are linked with `aria-describedby`, and the fields get `aria-invalid`.
- On a failed submit: an error summary (`role="alert"`, links to fields) that receives focus.
- The details textarea shows a live character count ("143 of 500 characters") in a polite live region, updated sparingly.
- The "needed by" date can't be in the past (`min` set by script to today).
- On success: the form is replaced by a confirmation with a reference number, and focus moves to its heading.

**Quality**
- Every control has a visible label; required fields marked in text, not only with a colour or asterisk.
- Visible `:focus-visible` styles; works at 320px wide and 200% zoom; respects dark mode.
- No layout shift; the script uses `defer`; no external fonts.

## Testing checklist

Run the in-page audit from the testing lesson, then test by hand: complete the whole form with only the keyboard; submit it empty; fix one error and watch it clear; try it with Narrator (Win+Ctrl+Enter) or NVDA; run Lighthouse and get 100 for accessibility.

@@quiz
Why mark required fields in text ("required") rather than only with a red asterisk?
* Colour and symbols alone aren't reliable for everyone, and screen readers may not announce an asterisk meaningfully
- Asterisks are invalid HTML
- Red is hard to print
- Text is faster
? Pair visual cues with text; `required` also exposes it to assistive tech.

@@quiz
After a successful submit replaces the form, where should focus go?
- Nowhere
* To the confirmation heading (with `tabindex="-1"`), so keyboard and screen-reader users know what happened
- Back to the first field
- To the browser address bar
? Without moving focus, it's lost on a removed element.

@@flashcards
`<fieldset>` + `<legend>` :: group related fields
`noindex` :: keep internal pages out of search
live character count :: polite status region
`min` on a date :: no past dates
success focus :: move to the confirmation heading
required marking :: text, not colour alone

@@exercise Build the IT request form
Build the form described in the lesson. It must include: `lang`, `noindex`, a skip link, three `<fieldset>`s with legends, radio buttons for the request type, `novalidate` with custom messages from the validity API, `aria-describedby` and `aria-invalid` on errors, an error summary with `role="alert"` that gets focus, a polite live character counter, a date `min` set to today by script, a confirmation with focus moved to its heading, a deferred script, and `:focus-visible` styles.
@@starter
<!doctype html>
<html>
<head>
<meta charset="utf-8">
<title>Request</title>
</head>
<body>
  <h1>IT request</h1>
  <form>
    <!-- TODO -->
  </form>
</body>
</html>
@@requires
<html lang="en">
content="noindex"
class="skip"
<fieldset>
<legend>
type="radio"
novalidate
aria-describedby=
aria-invalid
role="alert"
aria-live="polite"
.min =
tabindex="-1"
defer
:focus-visible
@@hint
Start with the HTML and CSS until it looks and reads right with no script. Then add the script in a separate `<script defer>` (inline scripts can't really be deferred; in a real site put it in `request.js` with `defer`, or put the inline script at the end of the body). Reuse your validity and error-summary code from the form UX lesson.
@@solution
<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<meta name="robots" content="noindex">
<meta name="description" content="Ask the IT team for hardware, software or access.">
<title>New IT request · Example Corp IT</title>
<style>
  :root { --text: #0f172a; --bg: #ffffff; --muted: #475569; --accent: #1d4ed8; --error: #b91c1c; --line: #cbd5e1; color-scheme: light dark; }
  @media (prefers-color-scheme: dark) { :root { --text: #f1f5f9; --bg: #0f172a; --muted: #cbd5e1; --accent: #93c5fd; --error: #fca5a5; --line: #475569; } }
  * { box-sizing: border-box; }
  body { margin: 0; font: 1rem/1.5 system-ui, sans-serif; color: var(--text); background: var(--bg); }
  .skip { position: absolute; left: -100vw; }
  .skip:focus { left: 1rem; top: 1rem; background: var(--bg); padding: 0.5rem; }
  header, main { max-inline-size: 42rem; margin-inline: auto; padding-inline: 1rem; }
  fieldset { border: 1px solid var(--line); border-radius: 8px; margin-block: 1rem; padding: 1rem; }
  legend { font-weight: 700; padding-inline: 0.25rem; }
  label { display: block; font-weight: 600; margin-block-start: 0.75rem; }
  .req { font-weight: 400; color: var(--muted); }
  input:not([type="radio"]), select, textarea { display: block; inline-size: 100%; font: inherit; padding: 0.5rem; border: 1px solid var(--line); border-radius: 6px; background: var(--bg); color: var(--text); }
  .radio label { display: inline; font-weight: 400; margin-inline-end: 1rem; }
  .hint { color: var(--muted); margin: 0.25rem 0 0; font-size: 0.9rem; }
  .error { color: var(--error); margin: 0.25rem 0 0; font-weight: 600; }
  .error::before { content: "⚠ "; }
  [aria-invalid="true"] { border: 2px solid var(--error); }
  :focus-visible { outline: 3px solid var(--accent); outline-offset: 2px; }
  button { font: inherit; padding: 0.6rem 1.2rem; border: 0; border-radius: 6px; background: var(--accent); color: var(--bg); }
  #summary { border: 3px solid var(--error); border-radius: 8px; padding: 0.5rem 1rem; }
</style>
<script defer>
  // In a real site this lives in request.js, loaded with <script src="request.js" defer>.
  document.addEventListener("DOMContentLoaded", () => {
    const form = document.getElementById("request");
    const summary = document.getElementById("summary");
    const fields = [...form.querySelectorAll("input:not([type=radio])[required], textarea[required], select[required]")];
    const typeRadios = [...form.querySelectorAll("input[name=type]")];

    const today = new Date().toISOString().slice(0, 10);
    const needed = document.getElementById("needed");
    needed.min = today;

    const messages = {
      name: { valueMissing: "Enter your full name" },
      email: { valueMissing: "Enter your work email", typeMismatch: "Enter an email like ana@example.com" },
      dept: { valueMissing: "Choose your department" },
      details: { valueMissing: "Describe what you need", tooLong: "Use 500 characters or fewer" },
      needed: { valueMissing: "Enter the date you need it by", rangeUnderflow: "The date can't be in the past" },
      manager: { valueMissing: "Enter your manager's email", typeMismatch: "Enter an email like manager@example.com" },
      cost: { valueMissing: "Enter the cost centre", patternMismatch: "The cost centre is CC- and 4 digits, like CC-1042" },
    };

    function messageFor(input) {
      const v = input.validity;
      if (v.valid) return "";
      const m = messages[input.id] || {};
      for (const key in m) if (v[key]) return m[key];
      return input.validationMessage;
    }

    function show(input) {
      const msg = messageFor(input);
      const err = document.getElementById(input.id + "-error");
      err.textContent = msg;
      err.hidden = !msg;
      if (msg) input.setAttribute("aria-invalid", "true");
      else input.removeAttribute("aria-invalid");
      return msg;
    }

    function typeError() {
      const err = document.getElementById("type-error");
      const ok = typeRadios.some(r => r.checked);
      err.hidden = ok;
      err.textContent = ok ? "" : "Choose a request type";
      return ok ? "" : err.textContent;
    }

    for (const input of fields) {
      input.addEventListener("blur", () => { if (input.value) show(input); });
      input.addEventListener("input", () => { if (input.hasAttribute("aria-invalid")) show(input); });
    }
    typeRadios.forEach(r => r.addEventListener("change", typeError));

    const details = document.getElementById("details");
    const count = document.getElementById("count");
    let timer;
    details.addEventListener("input", () => {
      clearTimeout(timer);
      timer = setTimeout(() => { count.textContent = `${details.value.length} of 500 characters`; }, 400);
    });

    form.addEventListener("submit", e => {
      e.preventDefault();
      const problems = [];
      const typeMsg = typeError();
      if (typeMsg) problems.push([typeRadios[0], typeMsg]);
      for (const input of fields) {
        const msg = show(input);
        if (msg) problems.push([input, msg]);
      }
      summary.replaceChildren();
      if (problems.length) {
        const h = document.createElement("h2");
        h.textContent = problems.length === 1 ? "There is 1 problem" : `There are ${problems.length} problems`;
        const ul = document.createElement("ul");
        for (const [input, msg] of problems) {
          const a = document.createElement("a");
          a.href = "#" + input.id;
          a.textContent = msg;
          const li = document.createElement("li");
          li.append(a);
          ul.append(li);
        }
        summary.append(h, ul);
        summary.hidden = false;
        summary.focus();
        return;
      }
      const ref = "REQ-" + String(Math.floor(Math.random() * 90000) + 10000);
      const done = document.getElementById("done");
      document.getElementById("ref").textContent = ref;
      form.hidden = true;
      done.hidden = false;
      document.getElementById("done-title").focus();
    });
  });
</script>
</head>
<body>
  <a class="skip" href="#main">Skip to the form</a>
  <header><p><strong>Example Corp IT</strong></p></header>
  <main id="main">
    <h1>New IT request</h1>
    <p>All fields are required unless marked optional.</p>
    <form id="request" novalidate>
      <div id="summary" role="alert" tabindex="-1" hidden></div>

      <fieldset>
        <legend>About you</legend>
        <label for="name">Full name</label>
        <input id="name" required autocomplete="name" aria-describedby="name-error">
        <p id="name-error" class="error" hidden></p>
        <label for="email">Work email</label>
        <input id="email" type="email" required autocomplete="email" aria-describedby="email-error">
        <p id="email-error" class="error" hidden></p>
        <label for="dept">Department</label>
        <select id="dept" required aria-describedby="dept-error">
          <option value="">Choose…</option><option>Finance</option><option>Sales</option><option>Engineering</option><option>HR</option>
        </select>
        <p id="dept-error" class="error" hidden></p>
      </fieldset>

      <fieldset aria-describedby="type-error">
        <legend>Request</legend>
        <div class="radio">
          <input type="radio" id="t-hw" name="type" value="hardware"><label for="t-hw">Hardware</label>
          <input type="radio" id="t-sw" name="type" value="software"><label for="t-sw">Software</label>
          <input type="radio" id="t-ac" name="type" value="access"><label for="t-ac">Access</label>
        </div>
        <p id="type-error" class="error" hidden></p>
        <label for="details">What do you need?</label>
        <textarea id="details" rows="5" maxlength="500" required aria-describedby="details-hint details-error"></textarea>
        <p id="details-hint" class="hint">Include model numbers, software names or folder paths.</p>
        <p id="count" class="hint" aria-live="polite"></p>
        <p id="details-error" class="error" hidden></p>
        <label for="needed">Needed by</label>
        <input id="needed" type="date" required aria-describedby="needed-error">
        <p id="needed-error" class="error" hidden></p>
      </fieldset>

      <fieldset>
        <legend>Approval</legend>
        <label for="manager">Manager's email</label>
        <input id="manager" type="email" required aria-describedby="manager-error">
        <p id="manager-error" class="error" hidden></p>
        <label for="cost">Cost centre</label>
        <input id="cost" required pattern="CC-[0-9]{4}" aria-describedby="cost-hint cost-error">
        <p id="cost-hint" class="hint">Like CC-1042. Ask your manager if you don't know it.</p>
        <p id="cost-error" class="error" hidden></p>
        <label for="notes">Notes <span class="req">(optional)</span></label>
        <textarea id="notes" rows="2"></textarea>
      </fieldset>

      <button>Send request</button>
    </form>

    <section id="done" hidden>
      <h2 id="done-title" tabindex="-1">Request sent</h2>
      <p>Your reference is <strong id="ref"></strong>. We'll email you when it's approved.</p>
    </section>
  </main>
</body>
</html>

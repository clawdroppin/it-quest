@@lesson n7-testing Testing and Auditing Front Ends
@@text
"It looks fine in my browser" is not a test. Front ends break in other browsers, at other screen sizes, for keyboard and screen-reader users, after a CSS change three files away, and when the HTML is invalid. This lesson covers the layers of checking a professional front end goes through, from free built-in tools to automated test suites.

## Layer 1: validate the markup

The W3C validator (validator.w3.org, also `vnu.jar` offline) catches unclosed tags, duplicate ids, invalid nesting (a `<div>` inside a `<p>`, a `<button>` inside an `<a>`), missing `alt`, and misspelt attributes. Browsers silently "fix" broken HTML, often differently from each other, so invalid markup produces bugs that only appear in one browser.

Common real-world errors: duplicate `id`s (breaks `label for`, `aria-labelledby` and `getElementById`), `<a>` without `href` used as a button, `<li>` outside a list, and block elements inside inline ones.

## Layer 2: automated audits

- **Lighthouse** (Chrome dev tools → Lighthouse): scores for performance, accessibility, best practices and SEO with specific fixes. Run it in an incognito window (extensions skew results) with mobile throttling.
- **axe DevTools** (browser extension) or the open-source **axe-core** library: the most widely used accessibility rule engine. Finds contrast failures, missing labels, invalid ARIA, duplicate ids, missing landmarks.
- **Dev tools panels**: the Accessibility tree, the CSS Overview (all colours and contrast issues on a page), Rendering → emulate vision deficiencies, dark mode, reduced motion; Coverage (unused CSS/JS).

Automated tools catch perhaps a third of accessibility problems. They can't tell whether alt text is **accurate**, whether focus order **makes sense**, or whether an error message is **helpful**.

## Layer 3: manual checks (a 10-minute routine)

1. **Keyboard**: Tab through everything. Can you reach and operate every control? Is focus always visible? Can you get out of every widget? Does Escape close things?
2. **Zoom to 200%** and **resize to 320px wide**: no horizontal scroll, no overlap, no cut-off text (WCAG "reflow").
3. **Screen reader** on the main flow (NVDA or Narrator on Windows, VoiceOver on Mac).
4. **Text spacing**: increase line height and letter spacing (a bookmarklet does it); does anything break?
5. **Real devices**: at least one phone, and Safari (WebKit has its own bugs).
6. **Slow network**: dev tools → Network → "Slow 4G". What does the user see while waiting?

## Layer 4: automated tests in code

Logic in front-end JavaScript (formatting, validation, state updates) is tested like any other code:

- **Unit tests** with Vitest or Jest for pure functions: `formatDuration(3700) === "1h 1m"`.
- **Component tests** with Testing Library: render a component, find elements **the way a user would** (`getByRole("button", { name: "Save" })`, `getByLabelText("Email")`), interact, assert. Finding by role and label means your tests also check accessibility.
- **End-to-end tests** with **Playwright** or Cypress: drive real browsers (Chromium, Firefox, WebKit) through whole flows, e.g. log in, open a ticket, close it.
- **Visual regression**: screenshot pages and diff them against approved images to catch unintended CSS changes (Playwright's `toHaveScreenshot`).
- **axe in tests**: `@axe-core/playwright` runs accessibility rules on every page in your E2E suite, so regressions fail the build.

```js,norun
// Playwright example
import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

test("new ticket flow", async ({ page }) => {
  await page.goto("/tickets/new");
  await page.getByLabel("Title").fill("VPN drops every hour");
  await page.getByRole("combobox", { name: "Priority" }).selectOption("P2");
  await page.getByRole("button", { name: "Create ticket" }).click();
  await expect(page.getByRole("status")).toHaveText("Ticket created");
  const results = await new AxeBuilder({ page }).analyze();
  expect(results.violations).toEqual([]);
});
```

These run in **CI** (module on Git/DevOps) on every pull request.

## A tiny in-page test runner

Without any tools you can still write assertions in the page itself, which is how the exercises below work:

```html
<output id="results"></output>
<script>
  const results = [];
  function check(name, cond) { results.push(`${cond ? "PASS" : "FAIL"} ${name}`); }
  check("every image has alt", [...document.images].every(img => img.hasAttribute("alt")));
  check("ids are unique", new Set([...document.querySelectorAll("[id]")].map(e => e.id)).size === document.querySelectorAll("[id]").length);
  document.getElementById("results").textContent = results.join("\n");
</script>
```

## Browser support

- **Baseline** (shown on MDN and caniuse.com): "Baseline widely available" means a feature has worked in all major browsers for 2.5+ years.
- **`@supports`** in CSS for progressive enhancement.
- Test in the browsers your users actually use (check analytics); in Japan, Safari on iPhone has a large share.

@@quiz
Why does invalid HTML cause bugs if the page "looks fine"?
- It doesn't; browsers fix everything
* Browsers repair broken markup in different ways, so behaviour can differ between browsers and break scripts and ARIA
- Validators slow the page
- Only search engines care
? Duplicate ids, for example, silently break labels and scripts.

@@quiz
What can automated accessibility checkers NOT tell you?
- Whether contrast is too low
- Whether an input has a label
* Whether the alt text accurately describes the image
- Whether ARIA attributes are valid
? They check presence and syntax, not meaning.

@@quiz
Why does Testing Library recommend `getByRole("button", { name: "Save" })` over `querySelector(".save-btn")`?
* It finds elements the way users and assistive tech do, so the test also checks accessibility and survives CSS refactors
- It's faster
- Classes don't work in tests
- It's required by Jest
? Tests coupled to class names break on harmless refactors.

@@quiz
What does "reflow" at 320px wide check?
- Animation speed
* That content works without horizontal scrolling at narrow widths or 400% zoom
- Print layout
- Font loading
? A WCAG requirement for people who zoom.

@@drill
Which Chrome DevTools panel audits performance, accessibility and SEO?
= Lighthouse
? Run it in an incognito window.

@@flashcards
W3C validator :: checks HTML validity
duplicate ids :: break labels, ARIA and scripts
Lighthouse :: performance/a11y/SEO audit
axe-core :: accessibility rule engine
automated a11y coverage :: roughly a third of issues
keyboard test :: Tab, Enter, Space, Escape, arrows
reflow :: usable at 320px / 400% zoom
Testing Library :: query by role and label
Playwright :: end-to-end tests in real browsers
visual regression :: screenshot diffs
Baseline :: cross-browser support status
`@supports` :: feature test in CSS

@@exercise A page self-audit
Write an in-page audit script that checks this page and prints PASS/FAIL lines into an `<output id="results">`: every `img` has an `alt` attribute; all `id`s are unique; every `input` (except type hidden/submit) has a `<label for>` or `aria-label`; the page has exactly one `h1`; and `<html>` has a `lang` attribute. Then fix the page until everything passes.
@@starter
<!doctype html>
<html>
<head><meta charset="utf-8"><title>Audit me</title></head>
<body>
  <h1>Help desk</h1>
  <img src="logo.png">
  <h1>New ticket</h1>
  <form>
    <input id="title" placeholder="Title">
    <label for="email">Email</label> <input id="email" type="email">
    <input id="title" type="text" placeholder="Phone">
    <input type="submit" value="Send">
  </form>
</body>
</html>
@@requires
<html lang="en">
<output id="results"
alt=
check(
PASS
FAIL
label[for=
@@hint
Write `check(name, cond)` that pushes `${cond ? "PASS" : "FAIL"} ${name}`. Unique ids: compare `new Set(ids).size` with `ids.length`. Labels: `[...document.querySelectorAll("input:not([type=hidden]):not([type=submit])")].every(i => i.hasAttribute("aria-label") || document.querySelector(`label[for="${i.id}"]`))`. Fixes: add `lang`, `alt`, make the second h1 an h2, give the inputs unique ids and labels.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Audit me</title></head>
<body>
  <h1>Help desk</h1>
  <img src="logo.png" alt="Help desk logo">
  <h2>New ticket</h2>
  <form>
    <label for="title">Title</label> <input id="title">
    <label for="email">Email</label> <input id="email" type="email">
    <label for="phone">Phone</label> <input id="phone" type="tel">
    <input type="submit" value="Send">
  </form>
  <output id="results" style="white-space: pre; display: block; font-family: monospace"></output>
  <script>
    const lines = [];
    function check(name, cond) { lines.push(`${cond ? "PASS" : "FAIL"} ${name}`); }
    check("every img has alt", [...document.images].every(img => img.hasAttribute("alt")));
    const ids = [...document.querySelectorAll("[id]")].map(e => e.id);
    check("ids are unique", new Set(ids).size === ids.length);
    const inputs = [...document.querySelectorAll('input:not([type="hidden"]):not([type="submit"])')];
    check("inputs are labelled", inputs.every(i => i.hasAttribute("aria-label") || (i.id && document.querySelector(`label[for="${i.id}"]`))));
    check("exactly one h1", document.querySelectorAll("h1").length === 1);
    check("html has lang", document.documentElement.hasAttribute("lang"));
    document.getElementById("results").textContent = lines.join("\n");
  </script>
</body>
</html>

@@exercise Unit-test a formatting function in the page
Write `formatDuration(seconds)` returning strings like `"45s"`, `"2m 5s"`, `"1h 1m"` (hours drop the seconds) and `"2d 3h"` (days drop the minutes), then a tiny test runner `test(name, actual, expected)` that compares with `===` and prints PASS/FAIL into a `<pre id="out">` with a final "N passed, M failed" line. Include at least the 6 cases in the starter's comment.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Tests</title></head>
<body>
  <pre id="out"></pre>
  <script>
    // formatDuration(45)     -> "45s"
    // formatDuration(125)    -> "2m 5s"
    // formatDuration(3700)   -> "1h 1m"
    // formatDuration(0)      -> "0s"
    // formatDuration(183600) -> "2d 3h"
    // formatDuration(3600)   -> "1h 0m"
  </script>
</body>
</html>
@@requires
function formatDuration(
function test(
passed
failed
Math.floor(
@@hint
`const d = Math.floor(s / 86400), h = Math.floor(s % 86400 / 3600), m = Math.floor(s % 3600 / 60), sec = s % 60;` then return the first matching format: `if (d) return `${d}d ${h}h`; if (h) return `${h}h ${m}m`; if (m) return `${m}m ${sec}s`; return `${sec}s`;`.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Tests</title></head>
<body>
  <pre id="out"></pre>
  <script>
    function formatDuration(s) {
      const d = Math.floor(s / 86400);
      const h = Math.floor((s % 86400) / 3600);
      const m = Math.floor((s % 3600) / 60);
      const sec = s % 60;
      if (d) return `${d}d ${h}h`;
      if (h) return `${h}h ${m}m`;
      if (m) return `${m}m ${sec}s`;
      return `${sec}s`;
    }

    const lines = [];
    let passed = 0, failed = 0;
    function test(name, actual, expected) {
      if (actual === expected) { passed++; lines.push(`PASS ${name}`); }
      else { failed++; lines.push(`FAIL ${name}: expected "${expected}", got "${actual}"`); }
    }
    test("seconds", formatDuration(45), "45s");
    test("minutes", formatDuration(125), "2m 5s");
    test("hours", formatDuration(3700), "1h 1m");
    test("zero", formatDuration(0), "0s");
    test("days", formatDuration(183600), "2d 3h");
    test("exact hour", formatDuration(3600), "1h 0m");
    lines.push(`${passed} passed, ${failed} failed`);
    document.getElementById("out").textContent = lines.join("\n");
  </script>
</body>
</html>

@@exercise A contrast checker
Implement the WCAG contrast ratio in JavaScript: convert each sRGB channel to linear light (`c <= 0.04045 ? c / 12.92 : ((c + 0.055) / 1.055) ** 2.4` with c in 0–1), compute relative luminance `L = 0.2126 R + 0.7152 G + 0.0722 B`, and the ratio `(Lmax + 0.05) / (Lmin + 0.05)`. Check the four colour pairs in the table and fill in each row's ratio (2 decimals) and AA result (≥ 4.5 for normal text).
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Contrast</title></head>
<body>
  <table>
    <thead><tr><th>Text</th><th>Background</th><th>Ratio</th><th>AA</th></tr></thead>
    <tbody id="rows">
      <tr data-fg="#64748b" data-bg="#ffffff"><td>#64748b</td><td>#ffffff</td><td></td><td></td></tr>
      <tr data-fg="#94a3b8" data-bg="#ffffff"><td>#94a3b8</td><td>#ffffff</td><td></td><td></td></tr>
      <tr data-fg="#ffffff" data-bg="#2563eb"><td>#ffffff</td><td>#2563eb</td><td></td><td></td></tr>
      <tr data-fg="#f59e0b" data-bg="#ffffff"><td>#f59e0b</td><td>#ffffff</td><td></td><td></td></tr>
    </tbody>
  </table>
</body>
</html>
@@requires
0.04045
12.92
2.4
0.2126
0.7152
0.0722
+ 0.05
4.5
@@hint
`function luminance(hex) { const [r, g, b] = [1, 3, 5].map(i => parseInt(hex.slice(i, i + 2), 16) / 255).map(c => c <= 0.04045 ? c / 12.92 : ((c + 0.055) / 1.055) ** 2.4); return 0.2126 * r + 0.7152 * g + 0.0722 * b; }`. Expect about 4.76 (pass), 2.56 (fail), 5.17 (pass) and 2.15 (fail).
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Contrast</title></head>
<body>
  <table>
    <thead><tr><th>Text</th><th>Background</th><th>Ratio</th><th>AA</th></tr></thead>
    <tbody id="rows">
      <tr data-fg="#64748b" data-bg="#ffffff"><td>#64748b</td><td>#ffffff</td><td></td><td></td></tr>
      <tr data-fg="#94a3b8" data-bg="#ffffff"><td>#94a3b8</td><td>#ffffff</td><td></td><td></td></tr>
      <tr data-fg="#ffffff" data-bg="#2563eb"><td>#ffffff</td><td>#2563eb</td><td></td><td></td></tr>
      <tr data-fg="#f59e0b" data-bg="#ffffff"><td>#f59e0b</td><td>#ffffff</td><td></td><td></td></tr>
    </tbody>
  </table>
  <script>
    function luminance(hex) {
      const [r, g, b] = [1, 3, 5]
        .map(i => parseInt(hex.slice(i, i + 2), 16) / 255)
        .map(c => (c <= 0.04045 ? c / 12.92 : ((c + 0.055) / 1.055) ** 2.4));
      return 0.2126 * r + 0.7152 * g + 0.0722 * b;
    }
    function contrast(a, b) {
      const [hi, lo] = [luminance(a), luminance(b)].sort((x, y) => y - x);
      return (hi + 0.05) / (lo + 0.05);
    }
    for (const row of document.querySelectorAll("#rows tr")) {
      const ratio = contrast(row.dataset.fg, row.dataset.bg);
      row.cells[2].textContent = ratio.toFixed(2);
      row.cells[3].textContent = ratio >= 4.5 ? "pass" : "fail";
      row.style.color = row.dataset.fg;
      row.style.background = row.dataset.bg;
    }
  </script>
</body>
</html>

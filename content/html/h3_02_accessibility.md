@@lesson h3-accessibility Semantic HTML and Accessibility
@@text
## Semantic elements

Instead of `<div>` everywhere, HTML has elements that say *what* a part of the page is:

```html
<header>
  <nav aria-label="Main">
    <a href="/">Home</a> <a href="/tickets">Tickets</a>
  </nav>
</header>
<main>
  <article>
    <h1>How to connect to the VPN</h1>
    <p>Updated <time datetime="2026-10-31">31 Oct 2026</time></p>
    <section>
      <h2>Windows</h2>
      <p>Open the client and choose <b>Tokyo-HQ</b>.</p>
    </section>
    <aside>Tip: the VPN reconnects automatically after sleep.</aside>
  </article>
</main>
<footer>© Sakura Systems IT</footer>
```

| Element | Means |
|---|---|
| `<header>`, `<footer>` | the top and bottom of a page or section |
| `<nav>` | a block of navigation links |
| `<main>` | the main content (one per page) |
| `<article>` | something complete on its own: a post, a KB article, a ticket |
| `<section>` | a themed part with its own heading |
| `<aside>` | related but secondary content |
| `<figure>` + `<figcaption>` | an image or diagram with a caption |

Screen readers let users jump straight to `<main>` or between headings. Search engines understand the page better too. And your HTML becomes easier to read.

## Accessibility (a11y)

Accessible pages work for people who are blind, have low vision, are colour-blind, can't use a mouse, or are deaf. It's also often a **legal requirement**: Japan's 障害者差別解消法 requires reasonable accommodation, and public-sector sites follow JIS X 8341-3 (based on **WCAG 2.x**, the international standard).

The checklist that catches most problems:

1. **Real elements for real jobs.** A `<button>` can be focused, pressed with Enter and Space, and is announced as a button. A `<div onclick>` is none of those.
2. **Every image has `alt`**; every input has a **`<label>`**.
3. **Headings in order**, one `<h1>`.
4. **Keyboard**: everything works with Tab, Enter, Space and Esc, and you can see where the focus is. Never write `outline: none` without a replacement:

```html
<style>
  a:focus-visible, button:focus-visible { outline: 3px solid #f59e0b; outline-offset: 2px; }
</style>
<p><a href="#">Tab to me</a> <button>and me</button></p>
```

5. **Contrast**: normal text needs a contrast ratio of at least **4.5:1** with its background (WCAG AA). Light grey on white usually fails.
6. **Don't use colour alone**: "errors are red" fails for colour-blind users. Add an icon or text: "✖ Failed".
7. **`lang`** on `<html>`, and on parts in another language: `<span lang="ja">東京</span>`.
8. **Link text that makes sense on its own**: "Download the VPN guide", not "click here".

## ARIA, carefully

ARIA attributes add information for assistive technology when HTML has no element for it:

```html
<button aria-expanded="false" aria-controls="menu">Menu</button>
<ul id="menu" hidden><li>Settings</li></ul>
<p role="alert">Session expires in 1 minute.</p>
<button aria-label="Close dialog">✕</button>
<span class="spinner" aria-hidden="true">⏳</span>
```

The first rule of ARIA: **don't use ARIA if a native element does the job.** `<button>` beats `<div role="button" tabindex="0">`. Wrong ARIA is worse than none.

## Testing

- Unplug the mouse and use the page with the keyboard only.
- Lighthouse (in Chrome's F12 tools) and axe DevTools find many problems automatically.
- Try a screen reader: **Narrator** (Win+Ctrl+Enter) on Windows, **VoiceOver** on Mac and iPhone, **NVDA** (free).

@@quiz
Why is `<button>` better than `<div onclick="...">`?
* A button can be focused and pressed with the keyboard, and is announced as a button
- It looks nicer
- div can't run JavaScript
- No difference
? Native elements give you accessibility for free.

@@quiz
What minimum contrast ratio does WCAG AA require for normal text?
* 4.5:1
- 2:1
- 10:1
- 1.5:1
? Large text (about 24px, or 19px bold) needs 3:1.

@@quiz
A form shows errors only by turning the field border red. What's the problem?
* Colour-blind users and screen-reader users may not notice: add text explaining the error
- Red is the wrong colour
- Nothing
- Borders don't work on inputs
? Never rely on colour alone. Add a message, and connect it with `aria-describedby`.

@@quiz
The first rule of ARIA is...
* Use a native HTML element instead whenever one exists
- Add `role` to every element
- Always use `aria-label`
- ARIA replaces HTML
? ARIA can't add keyboard behaviour. Native elements have it built in.

@@drill
Which element should wrap a page's main content?
= <main>
= main
? Screen reader users can jump straight to it.

@@flashcards
`<main>` :: the page's main content (one per page)
`<nav>` :: navigation links
`<article>` :: self-contained content
WCAG :: the international web accessibility standard
4.5:1 :: minimum contrast for normal text (AA)
`:focus-visible` :: style for keyboard focus
`aria-label` :: an accessible name when there's no visible text
`alt=""` :: a decorative image screen readers skip

@@exercise Make it accessible
This page has several accessibility problems. Fix them: add `lang="ja"` to `<html>`, use `<header>`, `<nav>`, `<main>` and `<footer>` instead of the divs, give the image an `alt`, connect the label to the input, turn the clickable div into a real `<button>`, and give the icon-only button an `aria-label` of `Search`.
@@starter
<!doctype html>
<html>
<head>
  <meta charset="utf-8">
  <title>社内ポータル</title>
</head>
<body>
  <div class="header">
    <img src="https://placehold.co/100x30?text=Logo">
    <div class="nav"><a href="/">ホーム</a> <a href="/help">ヘルプ</a></div>
  </div>
  <div class="main">
    <h1>パスワードの再設定</h1>
    <label>社員番号</label>
    <input id="employee" name="employee">
    <div class="btn" onclick="reset()">再設定する</div>
    <button>🔍</button>
  </div>
  <div class="footer">© IT部</div>
</body>
</html>
@@requires
<html lang="ja">
<header>
<nav
<main>
<footer>
alt="
<label for="employee">
<button type="submit">
aria-label="Search"
@@hint
`<label for="employee">社員番号</label>` connects to `id="employee"`. The div with `onclick` becomes `<button type="submit">再設定する</button>`. The magnifier: `<button aria-label="Search">🔍</button>`.
@@solution
<!doctype html>
<html lang="ja">
<head>
  <meta charset="utf-8">
  <title>社内ポータル</title>
</head>
<body>
  <header>
    <img src="https://placehold.co/100x30?text=Logo" alt="Sakura Systems">
    <nav aria-label="メイン"><a href="/">ホーム</a> <a href="/help">ヘルプ</a></nav>
  </header>
  <main>
    <h1>パスワードの再設定</h1>
    <form>
      <label for="employee">社員番号</label>
      <input id="employee" name="employee">
      <button type="submit">再設定する</button>
    </form>
    <button aria-label="Search">🔍</button>
  </main>
  <footer>© IT部</footer>
</body>
</html>

@@lesson h4-professional Professional Web Pages: Security, Performance and Publishing
@@text
## How a page reaches the browser

1. The browser looks up the name with **DNS**, connects with **TCP** and **TLS** (HTTPS), and sends `GET /index.html`.
2. The server answers with the HTML. The browser parses it and finds more resources: CSS, scripts, images, fonts, each one another request.
3. It builds the DOM (the HTML tree) and the CSSOM, works out the layout, and paints.

So every file costs a request, and **CSS in the `<head>` blocks rendering** until it has loaded. That's why performance work is mostly about sending less, and sending it sooner.

## Performance habits

- Compress images and use modern formats (`.webp`, `.avif`); give `width`/`height`; add `loading="lazy"` to images below the fold.
- `<script defer src="app.js">` so scripts don't block the page.
- Minify and cache CSS and JS. Use a CDN for static files.
- Measure with **Lighthouse** (F12 → Lighthouse) and the Network tab. *Core Web Vitals* (LCP, INP, CLS) are what Google measures.

## Security for web pages

You'll hear these in any security or web role:

**XSS (Cross-Site Scripting)**: an attacker gets their HTML or script into your page. If a ticket system shows a title as-is, a title like `<script>steal(document.cookie)</script>` runs in every agent's browser.

```html
<!-- The fix is escaping on output: < becomes &lt; so it's shown as text. -->
<p>Ticket title: &lt;script&gt;alert(1)&lt;/script&gt;</p>
```

Template engines (Jinja2, Razor, React) escape by default; the danger is in the escape hatches (`innerHTML`, `|safe`, `@Html.Raw`, `dangerouslySetInnerHTML`).

**Content Security Policy (CSP)**: an HTTP header (or meta tag) that tells the browser which sources are allowed. Even if XSS gets in, inline scripts won't run:

```html,norun
<meta http-equiv="Content-Security-Policy" content="default-src 'self'; img-src 'self' https:; script-src 'self'">
```

Other headers worth knowing: `Strict-Transport-Security` (HSTS: always HTTPS), `X-Frame-Options: DENY` / CSP `frame-ancestors` (stops clickjacking in iframes), `X-Content-Type-Options: nosniff`, `Referrer-Policy`.

**Mixed content**: an `https://` page loading `http://` scripts is blocked by browsers, and it should be.

**Subresource Integrity**: when loading a library from a CDN, `integrity="sha384-…"` makes the browser refuse a file that's been tampered with.

## Organising CSS

- One stylesheet (or a few), linked from every page.
- Name classes by **what things are**, not how they look: `.ticket-card`, not `.blue-box`. A common convention is **BEM**: `.card`, `.card__title`, `.card--urgent`.
- Utility frameworks like **Tailwind** (`class="p-4 rounded bg-sky-100"`) and component libraries like **Bootstrap** are widely used. Learn plain CSS first, and you'll understand what they do.
- A small reset at the top: `*, *::before, *::after { box-sizing: border-box; } body { margin: 0; }`.

## Publishing a static site

Static HTML/CSS needs no server code, and hosting is free or cheap:

- **GitHub Pages**: push to a repo, enable Pages in the settings, and it's live at `you.github.io`. Ideal for a portfolio.
- **Cloudflare Pages**, **Netlify**, **Azure Static Web Apps**, **AWS S3 + CloudFront**.
- On your own server: nginx with `root /var/www/site;`, plus a TLS certificate from **Let's Encrypt** (`certbot`).

## Validating

The W3C validator (validator.w3.org) checks your HTML. This app's checker finds unclosed tags; the validator finds everything else (wrong nesting, missing attributes, duplicate ids).

@@quiz
A help desk tool shows ticket titles without escaping them. What's the risk?
* XSS: a title containing a script runs in the browser of everyone who views it
- SQL injection
- The page loads slowly
- None, titles are text
? Escape all output (`<` → `&lt;`), and add a Content Security Policy as a second layer.

@@quiz
What does a Content Security Policy do?
* Tells the browser which sources of scripts, styles and images are allowed, blocking others
- Encrypts the page
- Hides the source code
- Validates forms
? A strict CSP turns many XSS bugs into harmless errors.

@@quiz
Why put `defer` on `<script>` tags?
* The script downloads without blocking the page, and runs after the HTML is parsed
- It delays the script by 5 seconds
- It disables the script
- It makes the script run first
? Blocking scripts in the head are a classic cause of slow pages.

@@quiz
Which is the simplest free way to publish a portfolio site?
* GitHub Pages
- Renting a dedicated server
- Running it on your laptop
- Emailing the HTML files
? Push the files to a repository and enable Pages. Add a custom domain later if you like.

@@drill
Which HTTP header limits where a page may load scripts from?
= Content-Security-Policy
= csp
? It blocks many XSS attacks.

@@flashcards
XSS :: attacker-controlled HTML/script running in your page
escaping :: turning `<` into `&lt;` so text can't become HTML
CSP :: header that limits where scripts/styles can come from
HSTS :: header forcing HTTPS
`loading="lazy"` :: load an image only when it's needed
`defer` :: run a script after parsing, without blocking
BEM :: class naming: block__element--modifier
GitHub Pages :: free static site hosting from a repository

@@exercise Escape the output
A script was going to insert the ticket title straight into the page. Write the title **escaped**, so the browser shows the text `<img src=x onerror=alert(1)> printer broken` instead of running it. Also add a Content Security Policy meta tag with `default-src 'self'`.
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Ticket</title>
</head>
<body>
  <h1>Ticket #5120</h1>
  <p>Title: <img src=x onerror=alert(1)> printer broken</p>
</body>
</html>
@@requires
&lt;img src=x onerror=alert(1)&gt;
http-equiv="Content-Security-Policy"
default-src 'self'
@@hint
Replace `<` with `&lt;` and `>` with `&gt;`. The CSP goes in the head: `<meta http-equiv="Content-Security-Policy" content="default-src 'self'">`.
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta http-equiv="Content-Security-Policy" content="default-src 'self'">
  <title>Ticket</title>
</head>
<body>
  <h1>Ticket #5120</h1>
  <p>Title: &lt;img src=x onerror=alert(1)&gt; printer broken</p>
</body>
</html>

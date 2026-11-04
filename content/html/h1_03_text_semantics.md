@@lesson h1-text Text Semantics: Headings, Quotes, Code, Time and Disclosure
@@text
HTML isn't about how text **looks** (that's CSS); it's about what text **means**. Meaning is what screen readers announce, what search engines index, what translators and reader modes rely on, and what makes a page survive a missing stylesheet. HTML has an element for most kinds of text you'll write.

## Headings are an outline

`<h1>` to `<h6>` form the page's table of contents. Screen reader users jump between headings the way sighted users skim. Rules:

- One `<h1>` per page: what the page is about.
- Don't skip levels going down (h2 → h4); going back up is fine.
- Choose the level by **structure**, never by size. Want a small h2? Style it.

## Inline meaning

| Element | Means | Typical look |
|---|---|---|
| `<strong>` | important | bold |
| `<em>` | stress emphasis (changes the meaning of the sentence) | italic |
| `<b>`, `<i>` | stylistically different, no importance (a product name, a foreign word) | bold / italic |
| `<mark>` | highlighted / relevant (search hits) | yellow |
| `<small>` | side comments, fine print | smaller |
| `<s>` | no longer accurate | struck through |
| `<del>` / `<ins>` | removed / added (edits) | struck / underlined |
| `<abbr title="...">` | abbreviation, with its expansion | dotted underline |
| `<code>` | code | monospace |
| `<kbd>` | keys the user presses | monospace |
| `<samp>` | program output | monospace |
| `<var>` | a variable | italic |
| `<sub>`, `<sup>` | subscript, superscript | H₂O, x² |

```html
<p>Press <kbd>Ctrl</kbd> + <kbd>Alt</kbd> + <kbd>Del</kbd>, then choose <b>Change a password</b>.
The <abbr title="Virtual Private Network">VPN</abbr> must be <strong>off</strong> first.</p>
<p>Run <code>ipconfig /flushdns</code>. You should see: <samp>Successfully flushed the DNS Resolver Cache.</samp></p>
```

## Blocks of code

`<pre>` keeps whitespace and line breaks exactly; put `<code>` inside for code. Escape `<` as `&lt;` and `&` as `&amp;` inside, or the browser reads them as tags.

```html
<pre><code>Get-Service -Name Spooler |
    Restart-Service -Verbose</code></pre>
```

## Quotations and citations

- `<blockquote cite="url">` for a quoted block, `<q>` for a short inline quote (the browser adds quotation marks in the page's language: 「」 for `lang="ja"`).
- `<cite>` for the **title of a work** (a book, a policy document), not for a person's name.

## Dates and times: `<time>`

`<time datetime="2026-11-03">3 November</time>` gives humans a friendly text and machines an exact value (calendars, search engines, scripts). Use ISO formats: `2026-11-03`, `2026-11-03T09:30+09:00`, durations like `PT2H30M`.

## Description lists

`<dl>` pairs terms (`<dt>`) with descriptions (`<dd>`): glossaries, key-value details, FAQ-like metadata.

```html
<dl>
  <dt>Hostname</dt><dd>SV-0001</dd>
  <dt>OS</dt><dd>Ubuntu 24.04</dd>
  <dt>Owner</dt><dd>Infrastructure team</dd>
</dl>
```

## Figures

`<figure>` wraps a self-contained piece (image, diagram, code listing, table) and `<figcaption>` labels it.

## Disclosure without JavaScript: `<details>`

```html
<details>
  <summary>How do I map a network drive?</summary>
  <p>Open File Explorer, right-click <b>This PC</b> and choose <b>Map network drive</b>.</p>
</details>
```

`<details open>` starts expanded. Several `<details name="faq">` with the same name form an exclusive accordion in current browsers: opening one closes the others.

## Characters and entities

Most characters can be typed directly in a UTF-8 page (`<meta charset="utf-8">`): é, ü, 日本語, →, ©. You only **must** escape `<` (`&lt;`), `&` (`&amp;`), and inside attribute values the quote you're using (`&quot;`). `&nbsp;` is a non-breaking space: it keeps "10&nbsp;GB" on one line.

## Line breaks

`<br>` is for line breaks that are part of the content (addresses, poems), not for spacing. `<wbr>` marks where a long word (a URL, a file path) may break. `<hr>` is a thematic break between sections.

@@quiz
How should you pick a heading level?
* By the document's structure; style the size with CSS
- By how big you want the text
- Always use h1 for anything important
- Headings are optional decoration
? Screen reader users navigate by heading level.

@@quiz
Which element marks keys the user should press?
- `<code>`
- `<samp>`
* `<kbd>`
- `<var>`
? `<samp>` is output; `<code>` is code.

@@quiz
What's the `datetime` attribute of `<time>` for?
- It changes how the date looks
* It gives a machine-readable value while the text stays human-friendly
- It makes the date clickable
- It's required for all dates
? Use ISO 8601 values.

@@quiz
What does `<details>` give you?
- A tooltip
* A built-in disclosure widget that opens and closes without JavaScript, keyboard accessible
- A modal dialog
- A table of contents
? `<summary>` is the clickable label.

@@flashcards
`<strong>` :: important
`<em>` :: stress emphasis
`<kbd>` :: keyboard input
`<samp>` :: program output
`<abbr title>` :: abbreviation with expansion
`<pre><code>` :: preformatted code block
`<blockquote cite>` :: quoted block
`<cite>` :: title of a work
`<time datetime>` :: machine-readable date/time
`<dl>`/`<dt>`/`<dd>` :: term and description list
`<figure>`/`<figcaption>` :: captioned content
`<details>`/`<summary>` :: disclosure widget
`&lt;` `&amp;` :: must-escape characters
`&nbsp;` :: non-breaking space

@@exercise A help article
Turn the plain text into a semantic help article:

- the title in an `<h1>`, the two step groups as `<h2>`s
- the keys `Win` and `R` in `<kbd>`, the command in `<code>`, the expected output in `<samp>`
- "DNS" as an `<abbr>` with the title `Domain Name System`
- the "last reviewed" date in a `<time>` with `datetime="2026-10-15"`
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Flush the DNS cache</title></head>
<body>
  <p>Flush the DNS cache</p>
  <p>Open a command prompt</p>
  <p>Press Win + R, type cmd and press Enter.</p>
  <p>Run the command</p>
  <p>Type ipconfig /flushdns. You should see Successfully flushed the DNS Resolver Cache.</p>
  <p>Last reviewed 15 October 2026.</p>
</body>
</html>
@@requires
<h1>
<h2>
<kbd>Win</kbd>
<code>ipconfig /flushdns</code>
<samp>
<abbr title="Domain Name System">DNS</abbr>
<time datetime="2026-10-15">
@@hint
Wrap each piece in its element: `<h1>Flush the <abbr title="Domain Name System">DNS</abbr> cache</h1>`, `<h2>Open a command prompt</h2>`, `<kbd>Win</kbd> + <kbd>R</kbd>`, `<code>ipconfig /flushdns</code>`, `<samp>Successfully flushed the DNS Resolver Cache.</samp>`, `<time datetime="2026-10-15">15 October 2026</time>`.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Flush the DNS cache</title></head>
<body>
  <article>
    <h1>Flush the <abbr title="Domain Name System">DNS</abbr> cache</h1>
    <h2>Open a command prompt</h2>
    <p>Press <kbd>Win</kbd> + <kbd>R</kbd>, type <kbd>cmd</kbd> and press <kbd>Enter</kbd>.</p>
    <h2>Run the command</h2>
    <p>Type <code>ipconfig /flushdns</code>. You should see <samp>Successfully flushed the DNS Resolver Cache.</samp></p>
    <footer><p>Last reviewed <time datetime="2026-10-15">15 October 2026</time>.</p></footer>
  </article>
</body>
</html>

@@exercise Server details and a code listing
Show the server's details as a description list (`Hostname`, `IP address`, `OS`) and the restart script as a captioned code listing: a `<figure>` containing `<pre><code>` and a `<figcaption>`. The script contains `<` and `&`, which must be escaped as `&lt;` and `&amp;`.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>SV-0001</title></head>
<body>
  <h1>SV-0001</h1>
  <p>Hostname: SV-0001, IP address: 10.0.5.11, OS: Ubuntu 24.04</p>
  <p>Restart script:</p>
  <p>if [ "$(uptime -p)" ]; then systemctl restart nginx && echo done > /tmp/restart.log; fi</p>
</body>
</html>
@@requires
<dl>
<dt>Hostname</dt>
<dd>10.0.5.11</dd>
<figure>
<pre><code>
&amp;&amp;
<figcaption>
@@hint
`<dl><dt>Hostname</dt><dd>SV-0001</dd>...</dl>`. In the code, `&&` becomes `&amp;&amp;` and `>` may stay as is (only `<` and `&` must be escaped), but `&gt;` is fine too.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>SV-0001</title></head>
<body>
  <h1>SV-0001</h1>
  <dl>
    <dt>Hostname</dt><dd>SV-0001</dd>
    <dt>IP address</dt><dd>10.0.5.11</dd>
    <dt>OS</dt><dd>Ubuntu 24.04</dd>
  </dl>
  <figure>
    <pre><code>if [ "$(uptime -p)" ]; then systemctl restart nginx &amp;&amp; echo done &gt; /tmp/restart.log; fi</code></pre>
    <figcaption>Restart script for the web server</figcaption>
  </figure>
</body>
</html>

@@exercise An FAQ accordion
Make an FAQ with three questions using `<details>` and `<summary>`, with the first one open by default. Give all three the same `name="faq"` so opening one closes the others in modern browsers.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>IT FAQ</title></head>
<body>
  <h1>IT FAQ</h1>
  <h2>How do I reset my password?</h2>
  <p>Use the self-service portal at https://reset.sakura.example.</p>
  <h2>Where are printer drivers?</h2>
  <p>On the file server, in \\fs01\drivers.</p>
  <h2>Who do I call after hours?</h2>
  <p>The on-call number on the back of your badge.</p>
</body>
</html>
@@requires
<details name="faq" open>
<summary>
</details>
@@hint
`<details name="faq" open><summary>How do I reset my password?</summary><p>...</p></details>` and two more without `open`.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>IT FAQ</title></head>
<body>
  <h1>IT FAQ</h1>
  <details name="faq" open>
    <summary>How do I reset my password?</summary>
    <p>Use the self-service portal at <a href="https://reset.sakura.example">reset.sakura.example</a>.</p>
  </details>
  <details name="faq">
    <summary>Where are printer drivers?</summary>
    <p>On the file server, in <code>\\fs01\drivers</code>.</p>
  </details>
  <details name="faq">
    <summary>Who do I call after hours?</summary>
    <p>The on-call number on the back of your badge.</p>
  </details>
</body>
</html>

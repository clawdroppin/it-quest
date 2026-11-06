@@lesson h0-welcome Your First Web Page
@@concepts hello
@@text
## What HTML and CSS are

Every web page is made of three layers:

- **HTML** (HyperText Markup Language): the *structure and content*. "This is a heading, this is a list, this is a link."
- **CSS** (Cascading Style Sheets): the *look*: colours, fonts, spacing, layout.
- **JavaScript**: the *behaviour* (its own track in this app).

You don't need to be a web developer to work in IT, but you'll constantly meet HTML: internal wiki pages, status pages, email templates, reports your scripts generate (the PowerShell health monitor writes HTML), router admin pages, and phishing emails you need to analyse. Being able to read and write it is a basic skill.

In this track you type HTML in the editor and press **▶ Run**: the result appears in a **preview** below. The checker looks for tags you forgot to close.

## Elements, tags and attributes

```html
<p>This is a <strong>paragraph</strong> with a <a href="https://example.jp">link</a>.</p>
```

- `<p>` is an **opening tag**, `</p>` the **closing tag**; together with the content they make an **element**.
- Elements nest inside each other, like boxes: close them in the reverse order you opened them. `<p><strong>…</strong></p>`, never `<p><strong>…</p></strong>`.
- `href="…"` is an **attribute**: extra information in the opening tag.
- A few elements have no content and no closing tag: `<br>`, `<img>`, `<input>`, `<meta>`, `<link>`, `<hr>`.

## The skeleton of every page

```html
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>IT Team Wiki</title>
</head>
<body>
  <h1>IT Team Wiki</h1>
  <p>Welcome! Start with the <em>new starter checklist</em>.</p>
  <h2>Contacts</h2>
  <p>Help desk: extension 1234</p>
</body>
</html>
```

| Part | Purpose |
|---|---|
| `<!doctype html>` | "this is modern HTML". Always the first line |
| `<html lang="en">` | the root; `lang` tells browsers and screen readers the language (`ja` for Japanese) |
| `<head>` | information *about* the page: not shown in it |
| `<meta charset="utf-8">` | text encoding. Without it, 日本語 can turn into mojibake (文字化け) |
| `<meta name="viewport" …>` | makes the page work properly on phones |
| `<title>` | the text on the browser tab and in search results |
| `<body>` | everything that's shown |

## Headings and paragraphs

`<h1>` to `<h6>` are headings, from most to least important. Use **one `<h1>`** per page, and don't skip levels (h1 → h2 → h3): screen readers and search engines use them as the page's outline. Pick headings for their meaning, never for their size: size is CSS's job.

`<p>` is a paragraph. HTML ignores extra spaces and line breaks in your source, so

```html
<p>These     words
are on     one line.</p>
```

shows as one line with single spaces. `<br>` forces a line break, but use it rarely (addresses, poems). For separate ideas, use separate paragraphs.

## Comments

```html
<!-- A comment: not shown on the page, but anyone can read it with "View source"! -->
<p>Visible text</p>
```

Never put passwords or internal notes in HTML comments. Attackers read page source too.

## Developer tools

In any browser, press **F12** (or right-click → *Inspect*). You can see the HTML of any page, change it live, and see which CSS applies. It's the most useful tool for learning web pages, and for debugging them at work.

@@quiz
Where does the text inside `<title>` appear?
* On the browser tab and in search results
- As the biggest heading on the page
- At the bottom of the page
- Nowhere: it's a comment
? `<title>` is in the `<head>`, so it isn't part of the page content. Use `<h1>` for the visible main heading.

@@quiz
Which nesting is correct?
* `<p><em>important</em> text</p>`
- `<p><em>important</p></em>`
- `<em><p>important</em></p>`
- `<p><em>important text</p>`
? Close elements in the reverse order you opened them: the last one opened is the first one closed.

@@quiz
Why add `<meta charset="utf-8">`?
* So the browser reads the text in the right encoding, and characters like 日本語 or é display correctly
- To make the page load faster
- To make the page secure
- It sets the font
? Without the right charset, text can turn into 文字化け (mojibake).

@@quiz
How should you choose between `<h2>` and `<h3>`?
* By the structure: h3 is a sub-section of the h2 above it
- By the font size you want
- h3 is for less important pages
- It doesn't matter
? Headings form the outline of the page. Change sizes with CSS.

@@drill
Which element holds the page title shown in the browser tab?
= <title>
= title
? It goes inside <head>.

@@flashcards
`<!doctype html>` :: first line of every modern HTML page
`<head>` :: information about the page (title, charset, styles)
`<body>` :: the visible content
`<h1>`…`<h6>` :: headings, by importance
`<p>` :: a paragraph
`<!-- … -->` :: a comment (still visible in the source!)
`lang="ja"` :: the page's language, for screen readers and search engines
F12 :: open the browser's developer tools

@@exercise Team page
Make a complete page with the title `Network Team`, an `<h1>` saying `Network Team`, a paragraph introducing the team, an `<h2>` `On call this week` and a paragraph with a name in `<strong>`. Use `lang="en"` and the UTF-8 charset.
@@starter
<!doctype html>
<html>
<head>
</head>
<body>
</body>
</html>
@@requires
<html lang="en">
<meta charset="utf-8">
<title>Network Team</title>
<h1>Network Team</h1>
<h2>On call this week</h2>
<strong>
@@hint
Put `<meta charset="utf-8">` and `<title>` inside `<head>`, and the headings and paragraphs inside `<body>`. Add `lang="en"` to the `<html>` tag.
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Network Team</title>
</head>
<body>
  <h1>Network Team</h1>
  <p>We look after the routers, switches, Wi-Fi and VPN for all three offices.</p>
  <h2>On call this week</h2>
  <p>Call <strong>Kenji Sato</strong> on extension 2201.</p>
</body>
</html>

@@exercise Fix the nesting
This page has tags closed in the wrong order and one that's never closed. Fix it (the checker lists the problems when you press Run).
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Outage notice</title>
</head>
<body>
  <h1>Planned outage</h1>
  <p>Email will be <strong>unavailable</p></strong>
  <p>on Saturday from <em>22:00 to 23:00.</p>
</body>
</html>
@@requires
<strong>unavailable</strong></p>
<em>22:00 to 23:00</em>
@@hint
The last tag opened must be closed first: `<strong>unavailable</strong></p>`. The `<em>` needs a `</em>` before the paragraph ends.
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Outage notice</title>
</head>
<body>
  <h1>Planned outage</h1>
  <p>Email will be <strong>unavailable</strong></p>
  <p>on Saturday from <em>22:00 to 23:00</em>.</p>
</body>
</html>

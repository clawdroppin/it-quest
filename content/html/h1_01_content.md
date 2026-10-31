@@lesson h1-content Links, Images, Lists and Tables
@@concepts lists
@@text
## Links

```html
<p>
  <a href="https://status.example.jp">Status page</a> ·
  <a href="contacts.html">Contacts</a> ·
  <a href="#vpn">VPN help (on this page)</a> ·
  <a href="mailto:helpdesk@example.jp">Email the help desk</a>
</p>
<h2 id="vpn">VPN help</h2>
```

- An **absolute** URL starts with `https://`. A **relative** one (`contacts.html`, `../img/logo.png`) is relative to the current page.
- `#vpn` jumps to the element with `id="vpn"`. Each `id` must be unique on the page.
- `target="_blank"` opens a new tab; add `rel="noopener"` with it.

Security habit: the text of a link and where it goes can be completely different. `<a href="https://evil.example">https://bank.co.jp</a>` shows a bank's address but goes elsewhere. That's how phishing emails work, so always hover to check the real URL.

## Images

```html
<img src="https://placehold.co/240x80?text=Logo" alt="Sakura Systems logo" width="240" height="80">
```

- `alt` is the text that screen readers read aloud, and that's shown if the image fails to load. It's required. Describe what matters about the image; use `alt=""` for purely decorative images.
- `width` and `height` let the browser reserve space, so the page doesn't jump around while it loads.

## Lists

```html
<h3>New starter checklist</h3>
<ol>
  <li>Collect your laptop from IT</li>
  <li>Set up MFA on your phone</li>
  <li>Change your temporary password</li>
</ol>

<h3>Supported browsers</h3>
<ul>
  <li>Edge</li>
  <li>Chrome
    <ul>
      <li>version 120 or newer</li>
    </ul>
  </li>
</ul>

<dl>
  <dt>DHCP</dt>
  <dd>Hands out IP addresses automatically.</dd>
  <dt>DNS</dt>
  <dd>Turns names into IP addresses.</dd>
</dl>
```

`<ol>` is numbered (order matters), `<ul>` is bulleted (order doesn't), `<dl>` is a list of terms (`<dt>`) and descriptions (`<dd>`). A nested list goes *inside* an `<li>`.

## Text-level elements

```html
<p>Run <code>ipconfig /flushdns</code>, then press <kbd>Ctrl</kbd>+<kbd>R</kbd>.</p>
<p><strong>Never</strong> share your password. It's <em>really</em> important.</p>
<p>Updated <time datetime="2026-10-31">31 October 2026</time>. H<sub>2</sub>O, 10<sup>3</sup>.</p>
<pre>
PING 10.0.0.1: 56 data bytes
64 bytes from 10.0.0.1: icmp_seq=0 ttl=64 time=0.9 ms
</pre>
```

`<strong>` means important, `<em>` means stressed (spoken emphasis). `<b>` and `<i>` only change the look without meaning. `<pre>` keeps spaces and line breaks exactly: good for command output.

## Tables

Tables are for **tabular data** (rows and columns), never for page layout.

```html
<table>
  <caption>Server inventory</caption>
  <thead>
    <tr><th>Host</th><th>IP</th><th>OS</th></tr>
  </thead>
  <tbody>
    <tr><td>web01</td><td>10.0.0.11</td><td>Ubuntu 24.04</td></tr>
    <tr><td>db01</td><td>10.0.0.21</td><td>Windows Server 2022</td></tr>
  </tbody>
</table>
```

`<tr>` is a row, `<th>` a header cell, `<td>` a data cell. `<thead>`/`<tbody>` group the rows, and `<caption>` titles the table. `colspan="2"` and `rowspan="2"` make a cell span several columns or rows.

@@quiz
What is the `alt` attribute of an image for?
* Text for screen readers, and to show if the image can't load
- The tooltip colour
- The image's file name
- Search engines only
? `alt` is essential for accessibility. Decorative images get `alt=""`.

@@quiz
`<a href="https://evil.example">https://bank.co.jp</a>`: where does it go?
* `https://evil.example`
- `https://bank.co.jp`
- Nowhere: it's invalid
- Both
? The `href` decides the destination; the visible text can say anything. Hover before you click.

@@quiz
Which list should a step-by-step procedure use?
* `<ol>`
- `<ul>`
- `<dl>`
- `<table>`
? The steps happen in order, so use an ordered (numbered) list.

@@quiz
In a table, what's the difference between `<th>` and `<td>`?
* `<th>` is a header cell, `<td>` a data cell
- `<th>` is a row
- `<td>` is the table's title
- No difference
? Header cells help screen readers announce what column a value is in.

@@exercise Help desk links
Make a page with an `<h1>` `Help desk`, and an unordered list of three links: `Reset password` to `https://passwords.example.jp`, `Status page` to `https://status.example.jp`, and `Email us` to `mailto:helpdesk@example.jp`. Add an image of a logo from `https://placehold.co/120x40?text=IT` with a meaningful `alt`.
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Help desk</title>
</head>
<body>
  <h1>Help desk</h1>
  <!-- logo and links here -->
</body>
</html>
@@requires
<ul>
<li><a href="https://passwords.example.jp">Reset password</a></li>
<li><a href="https://status.example.jp">Status page</a></li>
<li><a href="mailto:helpdesk@example.jp">Email us</a></li>
<img src="https://placehold.co/120x40?text=IT" alt="
@@hint
Each item is `<li><a href="...">Text</a></li>`, all inside `<ul>…</ul>`. The image is `<img src="..." alt="IT help desk logo">`.
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Help desk</title>
</head>
<body>
  <img src="https://placehold.co/120x40?text=IT" alt="IT help desk logo" width="120" height="40">
  <h1>Help desk</h1>
  <ul>
    <li><a href="https://passwords.example.jp">Reset password</a></li>
    <li><a href="https://status.example.jp">Status page</a></li>
    <li><a href="mailto:helpdesk@example.jp">Email us</a></li>
  </ul>
</body>
</html>

@@exercise Port table
Make a table with a `<caption>` `Common ports`, a header row (`Port`, `Protocol`, `Service`) in `<thead>`, and these rows in `<tbody>`: 22 TCP SSH, 53 UDP DNS, 443 TCP HTTPS.
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Ports</title>
</head>
<body>
</body>
</html>
@@requires
<caption>Common ports</caption>
<thead>
<th>Port</th>
<th>Service</th>
<tbody>
<td>22</td>
<td>DNS</td>
<td>HTTPS</td>
@@hint
`<table><caption>…</caption><thead><tr><th>Port</th><th>Protocol</th><th>Service</th></tr></thead><tbody><tr><td>22</td><td>TCP</td><td>SSH</td></tr>…</tbody></table>`
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Ports</title>
</head>
<body>
  <table>
    <caption>Common ports</caption>
    <thead>
      <tr><th>Port</th><th>Protocol</th><th>Service</th></tr>
    </thead>
    <tbody>
      <tr><td>22</td><td>TCP</td><td>SSH</td></tr>
      <tr><td>53</td><td>UDP</td><td>DNS</td></tr>
      <tr><td>443</td><td>TCP</td><td>HTTPS</td></tr>
    </tbody>
  </table>
</body>
</html>

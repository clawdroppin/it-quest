@@lesson n7-seo-i18n SEO, Social Sharing and Internationalisation (including Japanese)
@@text
Two things a public site needs that have nothing to do with how it looks: being **found and shared** (search engines, chat apps, social networks), and working in **other languages**, which for anyone aiming to work in Japan means handling Japanese properly.

## The head that search engines and chat apps read

```html
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>VPN setup guide · Example Corp IT</title>
  <meta name="description" content="Install and connect the company VPN on Windows, macOS, iPhone and Android in 5 minutes.">
  <link rel="canonical" href="https://it.example.com/guides/vpn">

  <!-- Open Graph: link previews in Slack, Teams, LINE, Facebook, LinkedIn -->
  <meta property="og:title" content="VPN setup guide">
  <meta property="og:description" content="Install and connect the company VPN in 5 minutes.">
  <meta property="og:image" content="https://it.example.com/img/vpn-card.png">
  <meta property="og:url" content="https://it.example.com/guides/vpn">
  <meta property="og:type" content="article">
  <meta name="twitter:card" content="summary_large_image">
</head>
```

- **`<title>`**: unique per page, most important words first, under ~60 characters. It's the link text in search results.
- **`description`**: the snippet under the link (not a ranking factor, but it decides whether people click).
- **`canonical`**: the one true URL when the same content is reachable several ways (`?utm_source=...`, trailing slash, http/https).
- **Open Graph image**: 1200×630 px is the common size.

## Structured data (JSON-LD)

Machine-readable facts about the page, which search engines can show as rich results (FAQ answers, breadcrumbs, ratings, events):

```html
<script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "FAQPage",
  "mainEntity": [{
    "@type": "Question",
    "name": "How do I reset my password?",
    "acceptedAnswer": { "@type": "Answer", "text": "Go to password.example.com and choose Forgot password." }
  }]
}
</script>
```

Types come from **schema.org** (Organization, Article, BreadcrumbList, Event, Product, FAQPage, HowTo). Test with Google's Rich Results Test.

## The rest of technical SEO

- **Semantic HTML**: one `h1`, logical headings, real links (`<a href>`, which crawlers follow; `onclick` navigation they don't).
- **`robots.txt`** at the site root says which paths crawlers may visit; **`sitemap.xml`** lists your URLs. `<meta name="robots" content="noindex">` keeps a page (e.g. an internal tool, a staging site) out of results.
- **Speed and mobile** (previous lessons) are ranking factors.
- **Server-side or static rendering** for content that must be indexed; content that only appears after JavaScript runs may be indexed late or not at all.
- **Descriptive URLs**: `/guides/vpn` beats `/page.php?id=372`.

Internal IT sites usually want the opposite: `noindex` and authentication, so internal documentation never appears on Google.

## Internationalisation (i18n)

**i18n** is making a site able to support many languages; **l10n** (localisation) is the work for one language.

```html
<html lang="ja">                                     <!-- page language: fonts, hyphenation, screen-reader voice -->
<p>Our office is in <span lang="ja">大阪</span>.</p>   <!-- a different language inline -->
<link rel="alternate" hreflang="ja" href="https://example.com/ja/">   <!-- the same page in Japanese -->
<link rel="alternate" hreflang="en" href="https://example.com/en/">
<link rel="alternate" hreflang="x-default" href="https://example.com/">
```

The `lang` attribute matters more than people think: a screen reader reads Japanese text with an English voice if the page says `lang="en"`, and browsers pick Chinese glyph shapes for kanji if the language isn't marked as Japanese (the same Unicode characters look slightly different in Japanese and Chinese fonts).

### Formatting with Intl

Never build dates, numbers or currency by string concatenation. The built-in `Intl` API does it for every locale:

```js,norun
new Intl.DateTimeFormat("ja-JP", { dateStyle: "full" }).format(new Date(2024, 2, 5))
// "2024年3月5日火曜日"
new Intl.DateTimeFormat("en-GB", { dateStyle: "full" }).format(new Date(2024, 2, 5))
// "Tuesday 5 March 2024"
new Intl.NumberFormat("ja-JP", { style: "currency", currency: "JPY" }).format(128000)
// "￥128,000"
new Intl.NumberFormat("de-DE").format(1234567.8)      // "1.234.567,8"
new Intl.RelativeTimeFormat("ja", { numeric: "auto" }).format(-1, "day")   // "昨日"
new Intl.PluralRules("en").select(1)                   // "one"
```

### Designing for translation

- Text **expands**: German and Finnish run 30–40% longer than English. Don't fix widths on buttons.
- Text can be **shorter but taller**: Japanese is compact but kanji need a larger font size (at least 16px) and more line height (1.7–2 is common for body text).
- Don't put text in images. Don't concatenate translated fragments ("You have " + n + " tickets"): word order differs between languages; use whole-sentence templates with placeholders.
- **Right-to-left** languages (Arabic, Hebrew): `dir="rtl"` plus logical properties (module 6) and the layout mirrors.

## Japanese typography on the web

```css
:lang(ja) {
  font-family: "Hiragino Sans", "Hiragino Kaku Gothic ProN", "Noto Sans JP", "Yu Gothic", Meiryo, sans-serif;
  line-height: 1.8;
  line-break: strict;            /* follow Japanese line-breaking rules (kinsoku) */
  word-break: normal;
  overflow-wrap: anywhere;
  font-feature-settings: "palt"; /* proportional spacing for kana, common in headings */
  text-spacing-trim: trim-start; /* tidy full-width punctuation spacing (newer browsers) */
}
em:lang(ja) { font-style: normal; text-emphasis: filled dot; }   /* 傍点 emphasis dots instead of italics */
```

- **Italic** doesn't exist in Japanese typography; use bold or emphasis dots.
- **Ruby** (furigana) shows readings above kanji:

```html
<ruby>情報<rp>(</rp><rt>じょうほう</rt><rp>)</rp></ruby>技術
```

  `<rt>` holds the reading; `<rp>` provides fallback parentheses for browsers without ruby support.
- **Vertical writing**: `writing-mode: vertical-rl` (used in novels, newspapers and some formal designs) with `text-orientation: mixed` and `text-combine-upright: all` for short numbers like "24" so they sit upright in one cell.
- **Full-width and half-width** forms: forms should accept both "１２３" and "123" and normalise them (`value.normalize("NFKC")` in JavaScript).
- **Names**: Japanese names are family name first (佐藤 健二). Don't force "First name / Last name" order; use "Family name / Given name" fields or a single "Full name" field, and possibly a furigana field (フリガナ) for reading.
- **Dates**: both the Western year and the Japanese era (令和6年 = 2024) are used; `Intl.DateTimeFormat("ja-JP-u-ca-japanese", { era: "long", year: "numeric" })` gives "令和6年".
- **Postal codes and addresses** run from large to small: 〒530-0001 大阪府大阪市北区梅田1-1-1.

@@quiz
What is the `<link rel="canonical">` for?
* Telling search engines the one preferred URL when the same content is reachable at several URLs
- Linking to the home page
- Loading a stylesheet
- Translating the page
? It prevents duplicate-content splitting.

@@quiz
Which tags control how a link looks when pasted into Slack, Teams or LINE?
- `<meta name="keywords">`
* Open Graph tags like `og:title`, `og:description`, `og:image`
- `<title>` only
- JSON-LD only
? Most chat apps read Open Graph.

@@quiz
Why does `lang="ja"` matter on Japanese text?
- It doesn't; Unicode is universal
* Screen readers pick the right voice, and browsers pick Japanese glyph shapes and line-breaking rules
- It's required for UTF-8
- It translates the page
? The same kanji code points render differently in Chinese and Japanese fonts.

@@quiz
How do you show emphasis in Japanese typography instead of italics?
- `font-style: oblique`
* Bold, or emphasis dots with `text-emphasis`
- Underline every word
- Larger font size only
? Japanese fonts have no italic.

@@quiz
What does `Intl.NumberFormat("ja-JP", { style: "currency", currency: "JPY" }).format(128000)` produce?
* "￥128,000"
- "128000 JPY"
- "¥128.000,00"
- "128,000円" always
? The Intl API formats per locale; yen has no decimal places.

@@drill
Which attribute on `<html>` sets the page's language?
= lang
? <html lang="ja"> for Japanese.

@@flashcards
`<title>` :: search result link text
`meta description` :: search result snippet
`rel="canonical"` :: preferred URL
`og:image` :: link preview image (1200×630)
JSON-LD :: structured data with schema.org types
`noindex` :: keep a page out of search results
`robots.txt` :: crawler rules
`hreflang` :: same page in another language
i18n / l10n :: make translatable / translate
`Intl.DateTimeFormat` :: locale dates
`<ruby>` / `<rt>` :: furigana
`writing-mode: vertical-rl` :: vertical Japanese
`text-emphasis` :: emphasis dots
`line-break: strict` :: Japanese line-breaking rules
`normalize("NFKC")` :: full-width → half-width
`ja-JP-u-ca-japanese` :: Japanese era calendar

@@exercise A shareable, indexable help page
Complete the head of the VPN guide: a specific `<title>`, a `description`, a `canonical` link to `https://it.example.com/guides/vpn`, Open Graph tags (`og:title`, `og:description`, `og:image`, `og:url`, `og:type` "article"), and a JSON-LD `FAQPage` with the two questions from the body. The page itself needs `lang="en"`.
@@starter
<!doctype html>
<html>
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Page</title>
</head>
<body>
  <h1>VPN setup guide</h1>
  <h2>How do I install the VPN?</h2>
  <p>Open the Company Portal app and install "Example VPN".</p>
  <h2>Why does the VPN disconnect?</h2>
  <p>Sleep mode ends the session; reconnect, or turn off sleep while working remotely.</p>
</body>
</html>
@@requires
<html lang="en">
<meta name="description"
<link rel="canonical" href="https://it.example.com/guides/vpn">
property="og:title"
property="og:image"
property="og:type" content="article"
type="application/ld+json"
"@type": "FAQPage"
"@type": "Question"
"acceptedAnswer"
@@hint
JSON-LD goes in `<script type="application/ld+json">` with `"mainEntity": [ { "@type": "Question", "name": "...", "acceptedAnswer": { "@type": "Answer", "text": "..." } }, ... ]`.
@@solution
<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>VPN setup guide · Example Corp IT</title>
<meta name="description" content="Install the company VPN from the Company Portal and fix common disconnects.">
<link rel="canonical" href="https://it.example.com/guides/vpn">
<meta property="og:title" content="VPN setup guide">
<meta property="og:description" content="Install the company VPN and fix common disconnects.">
<meta property="og:image" content="https://it.example.com/img/vpn-card.png">
<meta property="og:url" content="https://it.example.com/guides/vpn">
<meta property="og:type" content="article">
<script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "FAQPage",
  "mainEntity": [
    { "@type": "Question", "name": "How do I install the VPN?",
      "acceptedAnswer": { "@type": "Answer", "text": "Open the Company Portal app and install Example VPN." } },
    { "@type": "Question", "name": "Why does the VPN disconnect?",
      "acceptedAnswer": { "@type": "Answer", "text": "Sleep mode ends the session; reconnect, or turn off sleep while working remotely." } }
  ]
}
</script>
</head>
<body>
  <h1>VPN setup guide</h1>
  <h2>How do I install the VPN?</h2>
  <p>Open the Company Portal app and install "Example VPN".</p>
  <h2>Why does the VPN disconnect?</h2>
  <p>Sleep mode ends the session; reconnect, or turn off sleep while working remotely.</p>
</body>
</html>

@@exercise A Japanese notice with ruby and proper typography
Mark up the Japanese maintenance notice: `lang="ja"` on the `<html>`, furigana on 保守 (ほしゅ) and 再起動 (さいきどう) using `<ruby>`, `<rt>` and `<rp>` fallbacks, emphasis on 必ず with `text-emphasis` instead of italics, and Japanese typography CSS: a Japanese font stack including "Noto Sans JP", `line-height: 1.8` and `line-break: strict`. Format the date with `Intl.DateTimeFormat("ja-JP", { dateStyle: "full" })` into the `<time>` element.
@@starter
<!doctype html>
<html>
<head><meta charset="utf-8"><title>お知らせ</title></head>
<body>
  <h1>システム保守のお知らせ</h1>
  <p><time id="when" datetime="2024-03-09"></time>の22時から、サーバーを再起動します。</p>
  <p>作業前に<em>必ず</em>ファイルを保存してください。</p>
</body>
</html>
@@requires
<html lang="ja">
<ruby>保守<rp>(</rp><rt>ほしゅ</rt><rp>)</rp></ruby>
<ruby>再起動<rp>(</rp><rt>さいきどう</rt><rp>)</rp></ruby>
text-emphasis
font-style: normal
"Noto Sans JP"
line-break: strict
Intl.DateTimeFormat("ja-JP"
@@hint
`em { font-style: normal; text-emphasis: filled dot; }`. Format the date: `new Intl.DateTimeFormat("ja-JP", { dateStyle: "full" }).format(new Date(2024, 2, 9))` gives 2024年3月9日土曜日.
@@solution
<!doctype html>
<html lang="ja">
<head><meta charset="utf-8"><title>お知らせ</title>
<style>
  body { font-family: "Hiragino Sans", "Hiragino Kaku Gothic ProN", "Noto Sans JP", "Yu Gothic", Meiryo, sans-serif;
         line-height: 1.8; line-break: strict; font-size: 1.0625rem; }
  em { font-style: normal; text-emphasis: filled dot; }
  rt { font-size: 0.55em; }
</style>
</head>
<body>
  <h1>システム<ruby>保守<rp>(</rp><rt>ほしゅ</rt><rp>)</rp></ruby>のお知らせ</h1>
  <p><time id="when" datetime="2024-03-09"></time>の22時から、サーバーを<ruby>再起動<rp>(</rp><rt>さいきどう</rt><rp>)</rp></ruby>します。</p>
  <p>作業前に<em>必ず</em>ファイルを保存してください。</p>
  <script>
    const when = document.getElementById("when");
    when.textContent = new Intl.DateTimeFormat("ja-JP", { dateStyle: "full" }).format(new Date(2024, 2, 9));
  </script>
</body>
</html>

@@exercise One price list, four locales
Build a locale switcher (`<select>` with en-US, en-GB, de-DE and ja-JP) that re-renders a small price list with `Intl`: each price formatted with `Intl.NumberFormat(locale, { style: "currency", currency })` where the currency is USD, GBP, EUR or JPY respectively, the renewal date with `Intl.DateTimeFormat(locale, { dateStyle: "long" })`, and "renews in N days" with `Intl.RelativeTimeFormat(locale, { numeric: "auto" })`. Set `document.documentElement.lang` to the chosen locale.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Prices</title></head>
<body>
  <label for="loc">Locale</label>
  <select id="loc"><option>en-US</option><option>en-GB</option><option>de-DE</option><option>ja-JP</option></select>
  <ul id="list"></ul>
  <script>
    const items = [{ name: "Microsoft 365 E3", price: 36.0 }, { name: "Adobe Acrobat", price: 19.99 }, { name: "Zoom Pro", price: 13.33 }];
    const renews = new Date(2025, 0, 15);
  </script>
</body>
</html>
@@requires
Intl.NumberFormat(
style: "currency"
Intl.DateTimeFormat(
Intl.RelativeTimeFormat(
documentElement.lang
addEventListener("change"
@@hint
Map locales to currencies: `{ "en-US": "USD", "en-GB": "GBP", "de-DE": "EUR", "ja-JP": "JPY" }`. Days until renewal: `Math.round((renews - new Date()) / 86400000)`. (Real prices would also need converting; here we just show formatting.)
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Prices</title></head>
<body>
  <label for="loc">Locale</label>
  <select id="loc"><option>en-US</option><option>en-GB</option><option>de-DE</option><option>ja-JP</option></select>
  <ul id="list"></ul>
  <p id="renew"></p>
  <script>
    const items = [{ name: "Microsoft 365 E3", price: 36.0 }, { name: "Adobe Acrobat", price: 19.99 }, { name: "Zoom Pro", price: 13.33 }];
    const renews = new Date(2025, 0, 15);
    const currencies = { "en-US": "USD", "en-GB": "GBP", "de-DE": "EUR", "ja-JP": "JPY" };
    function render(locale) {
      document.documentElement.lang = locale;
      const money = new Intl.NumberFormat(locale, { style: "currency", currency: currencies[locale] });
      const list = document.getElementById("list");
      list.replaceChildren(...items.map(item => {
        const li = document.createElement("li");
        li.textContent = `${item.name}: ${money.format(item.price)}`;
        return li;
      }));
      const date = new Intl.DateTimeFormat(locale, { dateStyle: "long" }).format(renews);
      const days = Math.round((renews - new Date()) / 86400000);
      const rel = new Intl.RelativeTimeFormat(locale, { numeric: "auto" }).format(days, "day");
      document.getElementById("renew").textContent = `${date} (${rel})`;
    }
    const select = document.getElementById("loc");
    select.addEventListener("change", () => render(select.value));
    render(select.value);
  </script>
</body>
</html>

@@lesson xp-it_handbook Searchable IT Handbook
@@text
Write the IT handbook every company needs: how to connect to Wi-Fi and VPN, set up MFA, report a phishing email, request software. Several linked pages, a shared stylesheet, a table of contents, a small JavaScript search over the page titles and headings, and a print stylesheet so people can print a page.

**You'll practise:** multi-page structure · navigation and breadcrumbs · shared CSS · accessible forms · print CSS

## Milestones

1. **Pages**: index plus at least four how-to pages with numbered steps and screenshots placeholders.
2. **Navigation**: the same header and nav on every page, with the current page marked.
3. **Search**: an input that filters a list of all headings as you type.
4. **Print**: hide the nav and show link URLs when printed.
5. **Check**: test keyboard navigation and run Lighthouse's accessibility audit.

## You're done when

- A colleague can find and follow the VPN guide without asking you.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"How do I mark the current page in the navigation accessibly?"*
- *"Help me write clear step-by-step instructions for non-technical staff."*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project it_handbook Searchable IT Handbook
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Searchable IT Handbook

Write the IT handbook every company needs: how to connect to Wi-Fi and VPN, set up MFA, report a phishing email, request software. Several linked pages, a shared stylesheet, a table of contents, a small JavaScript search over the page titles and headings, and a print stylesheet so people can print a page.

## Milestones

- [ ] 1. Pages: index plus at least four how-to pages with numbered steps and screenshots placeholders.
- [ ] 2. Navigation: the same header and nav on every page, with the current page marked.
- [ ] 3. Search: an input that filters a list of all headings as you type.
- [ ] 4. Print: hide the nav and show link URLs when printed.
- [ ] 5. Check: test keyboard navigation and run Lighthouse's accessibility audit.

## Done when

- [ ] A colleague can find and follow the VPN guide without asking you.
@@file index.html
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>IT Handbook</title>
  <link rel="stylesheet" href="style.css">
</head>
<body>
  <header><a href="index.html">IT Handbook</a></header>
  <nav aria-label="Guides">
    <ul>
      <li><a href="vpn.html">Connect to the VPN</a></li>
      <!-- TODO: more guides -->
    </ul>
  </nav>
  <main>
    <h1>How can we help?</h1>
    <label for="q">Search the guides</label>
    <input id="q" type="search">
  </main>
</body>
</html>
@@file vpn.html
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Connect to the VPN · IT Handbook</title>
  <link rel="stylesheet" href="style.css">
</head>
<body>
  <main>
    <h1>Connect to the VPN</h1>
    <ol>
      <li>Open the VPN app from the Start menu.</li>
      <!-- TODO -->
    </ol>
  </main>
</body>
</html>
@@file style.css
body { font-family: system-ui, sans-serif; max-width: 70ch; margin: 0 auto; padding: 1rem; line-height: 1.6; }
@media print { nav, header { display: none; } a::after { content: " (" attr(href) ")"; } }

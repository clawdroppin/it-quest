@@lesson xp-status_page Service Status Page
@@text
When email is down, people want one page that says so. Build a clean, accessible status page: overall status at the top, a list of services with coloured badges, current incidents with updates, and a 90-day uptime bar per service, all in HTML and CSS (with a small JSON file you could later fill from monitoring).

**You'll practise:** semantic HTML · CSS grid · status colours with text, not colour alone · responsive layout

## Milestones

1. **Structure**: header, overall status banner, services list, incidents, footer.
2. **Badges**: Operational, Degraded, Outage, each with an icon and text.
3. **Uptime bars**: 90 small boxes per service (CSS grid), colour by status.
4. **Incident timeline**: Investigating → Identified → Monitoring → Resolved with times.
5. **Responsive and accessible**: works on a phone; passes a keyboard and contrast check.

## You're done when

- The page looks good on desktop and phone and makes sense with a screen reader.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"How can I show status without relying only on colour?"*
- *"Review my HTML structure for accessibility."*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project status_page Service Status Page
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Service Status Page

When email is down, people want one page that says so. Build a clean, accessible status page: overall status at the top, a list of services with coloured badges, current incidents with updates, and a 90-day uptime bar per service, all in HTML and CSS (with a small JSON file you could later fill from monitoring).

## Milestones

- [ ] 1. Structure: header, overall status banner, services list, incidents, footer.
- [ ] 2. Badges: Operational, Degraded, Outage, each with an icon and text.
- [ ] 3. Uptime bars: 90 small boxes per service (CSS grid), colour by status.
- [ ] 4. Incident timeline: Investigating → Identified → Monitoring → Resolved with times.
- [ ] 5. Responsive and accessible: works on a phone; passes a keyboard and contrast check.

## Done when

- [ ] The page looks good on desktop and phone and makes sense with a screen reader.
@@file index.html
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Acme IT Status</title>
  <link rel="stylesheet" href="style.css">
</head>
<body>
  <header>
    <h1>Acme IT Status</h1>
  </header>
  <main>
    <section class="banner ok" aria-live="polite">
      <p>All systems operational</p>
    </section>
    <section aria-labelledby="services">
      <h2 id="services">Services</h2>
      <ul class="services">
        <li><span class="name">Email</span> <span class="badge ok">Operational</span></li>
        <!-- TODO: more services, each with a 90-day uptime bar -->
      </ul>
    </section>
    <!-- TODO: incidents section -->
  </main>
</body>
</html>
@@file style.css
body { font-family: system-ui, sans-serif; max-width: 860px; margin: 0 auto; padding: 1rem; }
.banner { padding: 1rem; border-radius: 8px; color: white; }
.banner.ok, .badge.ok { background: #1a7f37; }
.badge { color: white; padding: .1rem .5rem; border-radius: 999px; font-size: .85rem; }
/* TODO: degraded and outage colours, uptime bars, incident timeline */

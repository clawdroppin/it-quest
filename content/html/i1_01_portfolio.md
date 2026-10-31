@@lesson h9-portfolio Portfolio and Résumé Site
@@text
Recruiters, in Japan and everywhere, look you up before an interview. A clean personal site that lists your **skills, certifications and the projects you built** in this app does more than any cover letter: it shows your work instead of describing it.

You'll build a one-page site with:

- a header with navigation, and a "skip to content" link for keyboard users;
- an intro with your photo and **certification badges** (CCNA, AWS, 基本情報技術者, JLPT…);
- a responsive **grid of skill cards** and **project cards** that rearranges itself on any screen;
- an **experience timeline** drawn with pure CSS;
- **dark mode** that follows the system setting;
- a **print stylesheet**: press Ctrl+P and the same page prints as a tidy one-page résumé.

Everything you've learned in this track comes together: semantic HTML, flexbox, grid, custom properties, media queries and accessibility.

Work through the numbered TODO comments: 1–8 in `index.html`, then A–H in `style.css`. Open `index.html` in your browser (the button on the card) and refresh after each change. Use your **own** details: this is your real portfolio.
@@project portfolio Portfolio and Résumé Site
Fill in the TODOs in `index.html` (1–8) and `style.css` (A–H), with your own name, skills, certifications and projects. Open the page in your browser and check it on a narrow window, in dark mode, and in the print preview.
@@file index.html
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <!-- TODO 1: a meta description (one sentence about you: what search results show) -->
  <title>Your Name · IT Engineer</title>
  <link rel="stylesheet" href="style.css">
</head>
<body>
  <a class="skip" href="#main">Skip to content</a>

  <header class="site-header">
    <a class="logo" href="#">YN</a>
    <!-- TODO 2: a <nav aria-label="Main"> with links to #about, #skills, #projects, #experience, #contact -->
  </header>

  <main id="main">
    <section class="hero" id="about">
      <!-- TODO 3: your photo (or a placeholder) with an alt text, width and height -->
      <div>
        <h1>Your Name</h1>
        <p class="tagline">IT engineer · City</p>
        <p>Two or three sentences: what you do, what you're good at, and what job you're looking for.</p>
        <!-- TODO 4: your certifications as <span class="badge"> items inside <p class="badges"> -->
      </div>
    </section>

    <section id="skills">
      <h2>Skills</h2>
      <div class="skill-grid">
        <!-- TODO 5: one <article class="card"> per skill area, each with an <h3> and a <ul> -->
      </div>
    </section>

    <section id="projects">
      <h2>Projects</h2>
      <div class="project-list">
        <!-- TODO 6: one <article class="card project"> per project you built in this app:
             an <h3>, a sentence, a <p class="tech"> list of technologies, and a link to the code -->
      </div>
    </section>

    <section id="experience">
      <h2>Experience</h2>
      <!-- TODO 7: an <ol class="timeline"> with one <li> per job (or study): an <h3>,
           a <p class="when"> with <time datetime="2024-04"> dates, and what you achieved -->
    </section>

    <section id="contact">
      <h2>Contact</h2>
      <!-- TODO 8: a mailto: link, and links to your GitHub and LinkedIn -->
    </section>
  </main>

  <footer class="site-footer">
    <p>© 2026 Your Name.</p>
  </footer>
</body>
</html>
@@file style.css
/* ---------- Theme ---------- */
:root {
  --bg: #f8fafc;
  --surface: #ffffff;
  --text: #0f172a;
  --muted: #475569;
  --accent: #0369a1;
  --accent-soft: #e0f2fe;
  --border: #e2e8f0;
  --radius: 10px;
  --max: 70rem;
}

/* TODO A: redefine the colour variables inside @media (prefers-color-scheme: dark) */

/* ---------- Base ---------- */
*, *::before, *::after { box-sizing: border-box; }

body {
  margin: 0;
  background: var(--bg);
  color: var(--text);
  font: 1rem/1.6 system-ui, "Noto Sans JP", sans-serif;
}

a { color: var(--accent); }
a:focus-visible { outline: 3px solid #f59e0b; outline-offset: 2px; }
img { max-width: 100%; height: auto; }

main, .site-header, .site-footer {
  width: min(100% - 2rem, var(--max));
  margin-inline: auto;
}

section { padding-block: 2.5rem; }

.skip { position: absolute; left: -999px; }
.skip:focus { left: 1rem; top: 1rem; background: var(--surface); padding: 0.5rem 1rem; z-index: 10; }

/* TODO B: .site-header as a flex row (logo left, nav pushed right with margin-left: auto), wrapping on small screens */

/* TODO C: .hero as a grid: one column on phones, `auto 1fr` from 700px wide (a media query) */

/* TODO D: .badge as a rounded "pill": accent-soft background, accent text, padding, border-radius: 999px */

/* ---------- Cards ---------- */
.card {
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  padding: 1.25rem;
}

/* TODO E: .skill-grid and .project-list as responsive grids: repeat(auto-fill, minmax(15rem, 1fr)) */

/* TODO F: lift .project cards slightly on :hover with a transition, but not with prefers-reduced-motion */

/* TODO G: the .timeline: no bullets, a left border, and a dot before each item (li::before) */

.site-footer { color: var(--muted); border-top: 1px solid var(--border); padding-block: 1.5rem; font-size: 0.9rem; }

/* TODO H: @media print: hide the nav, white background, black text, no card borders,
   so Ctrl+P prints the page as a one-page résumé */
@@text
> Tips: `margin-left: auto` on the nav pushes it to the right of a flex row. For the timeline dot: `li { position: relative; }` and `li::before { content: ""; position: absolute; left: -0.55rem; width: 0.8rem; height: 0.8rem; border-radius: 50%; background: var(--accent); }`. In the print styles, set the colour variables again inside `@media print`, and everything that uses them follows.
@@project portfolio_finished Portfolio and Résumé Site: the finished version
A complete example with sample content. Publish yours with **GitHub Pages**: create a repository called `<your-username>.github.io`, push `index.html` and `style.css`, and it's live at `https://<your-username>.github.io`. Ideas: a Japanese version (`ja/index.html` with `lang="ja"`), a downloadable PDF résumé, or a blog page for your lab write-ups.
@@file index.html
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <meta name="description" content="Portfolio of Alex Morgan, IT infrastructure engineer in Tokyo: networks, cloud and automation.">
  <title>Alex Morgan · IT Infrastructure Engineer</title>
  <link rel="stylesheet" href="style.css">
</head>
<body>
  <a class="skip" href="#main">Skip to content</a>

  <header class="site-header">
    <a class="logo" href="#">AM</a>
    <nav aria-label="Main">
      <a href="#about">About</a>
      <a href="#skills">Skills</a>
      <a href="#projects">Projects</a>
      <a href="#experience">Experience</a>
      <a href="#contact">Contact</a>
    </nav>
  </header>

  <main id="main">
    <section class="hero" id="about">
      <img class="avatar" src="https://placehold.co/160x160?text=AM" alt="Photo of Alex Morgan" width="160" height="160">
      <div>
        <h1>Alex Morgan</h1>
        <p class="tagline">IT infrastructure engineer · Tokyo · <span lang="ja">インフラエンジニア</span></p>
        <p>I keep networks, servers and cloud services running, and I automate everything I do twice. Looking for an infrastructure or cloud role in Japan.</p>
        <p class="badges">
          <span class="badge">CCNA</span>
          <span class="badge">AWS SAA</span>
          <span class="badge">Azure AZ-104</span>
          <span class="badge">基本情報技術者</span>
          <span class="badge">JLPT N2</span>
        </p>
      </div>
    </section>

    <section id="skills">
      <h2>Skills</h2>
      <div class="skill-grid">
        <article class="card">
          <h3>Networking</h3>
          <ul>
            <li>VLANs, OSPF, ACLs on Cisco IOS</li>
            <li>Subnetting and IP planning</li>
            <li>Firewalls and site-to-site VPN</li>
          </ul>
        </article>
        <article class="card">
          <h3>Cloud</h3>
          <ul>
            <li>AWS: VPC, EC2, S3, IAM</li>
            <li>Azure: Entra ID, VMs, Bicep</li>
            <li>Docker and Kubernetes basics</li>
          </ul>
        </article>
        <article class="card">
          <h3>Automation</h3>
          <ul>
            <li>PowerShell and Bash</li>
            <li>Python tools and tests</li>
            <li>Git and CI pipelines</li>
          </ul>
        </article>
        <article class="card">
          <h3>Operations</h3>
          <ul>
            <li>ITIL 4 incident and change</li>
            <li>Monitoring and on-call</li>
            <li>Windows Server and Linux admin</li>
          </ul>
        </article>
      </div>
    </section>

    <section id="projects">
      <h2>Projects</h2>
      <div class="project-list">
        <article class="card project">
          <h3>Firewall Rule Engine</h3>
          <p>A first-match ACL engine in C++ with CIDR matching and shadowed-rule detection.</p>
          <p class="tech">C++20 · networking · tests</p>
          <a href="https://github.com/example/firewall">Source code on GitHub</a>
        </article>
        <article class="card project">
          <h3>Patch Compliance Reporter</h3>
          <p>Checks a fleet against a security baseline and exits with an alert code for CI.</p>
          <p class="tech">C# · .NET 8 · JSON</p>
          <a href="https://github.com/example/patch-report">Source code on GitHub</a>
        </article>
        <article class="card project">
          <h3>Log Sentinel</h3>
          <p>Finds brute-force attacks and suspicious logins in SSH logs.</p>
          <p class="tech">Python · regex · security</p>
          <a href="https://github.com/example/log-sentinel">Source code on GitHub</a>
        </article>
      </div>
    </section>

    <section id="experience">
      <h2>Experience</h2>
      <ol class="timeline">
        <li>
          <h3>IT Support Engineer · Example Corp</h3>
          <p class="when"><time datetime="2024-04">Apr 2024</time> – present</p>
          <p>First-line support for 300 users, Microsoft 365 administration, and onboarding automation that cut setup time from 2 hours to 15 minutes.</p>
        </li>
        <li>
          <h3>Help Desk Technician · Sample Ltd</h3>
          <p class="when"><time datetime="2022-06">Jun 2022</time> – <time datetime="2024-03">Mar 2024</time></p>
          <p>Hardware, accounts and printers for two offices; wrote the team's knowledge base.</p>
        </li>
      </ol>
    </section>

    <section id="contact">
      <h2>Contact</h2>
      <p>The fastest way to reach me is email: <a href="mailto:alex@example.com">alex@example.com</a>.</p>
      <p><a href="https://github.com/example">GitHub</a> · <a href="https://www.linkedin.com/in/example">LinkedIn</a></p>
    </section>
  </main>

  <footer class="site-footer">
    <p>© 2026 Alex Morgan. Built with plain HTML and CSS.</p>
  </footer>
</body>
</html>
@@file style.css
/* ---------- Theme ---------- */
:root {
  --bg: #f8fafc;
  --surface: #ffffff;
  --text: #0f172a;
  --muted: #475569;
  --accent: #0369a1;
  --accent-soft: #e0f2fe;
  --border: #e2e8f0;
  --radius: 10px;
  --max: 70rem;
}

@media (prefers-color-scheme: dark) {
  :root {
    --bg: #0b1120;
    --surface: #111a2e;
    --text: #e2e8f0;
    --muted: #94a3b8;
    --accent: #38bdf8;
    --accent-soft: #0c2a43;
    --border: #1e293b;
  }
}

/* ---------- Base ---------- */
*, *::before, *::after { box-sizing: border-box; }

body {
  margin: 0;
  background: var(--bg);
  color: var(--text);
  font: 1rem/1.6 system-ui, "Noto Sans JP", sans-serif;
}

a { color: var(--accent); }
a:focus-visible { outline: 3px solid #f59e0b; outline-offset: 2px; }
img { max-width: 100%; height: auto; }
h1, h2, h3 { line-height: 1.2; }

main, .site-header, .site-footer {
  width: min(100% - 2rem, var(--max));
  margin-inline: auto;
}

section { padding-block: 2.5rem; }

/* Visible only when focused with the keyboard. */
.skip { position: absolute; left: -999px; }
.skip:focus { left: 1rem; top: 1rem; background: var(--surface); padding: 0.5rem 1rem; z-index: 10; }

/* ---------- Header ---------- */
.site-header {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 1rem;
  padding-block: 1rem;
}
.logo {
  font-weight: 800;
  font-size: 1.25rem;
  text-decoration: none;
}
.site-header nav { display: flex; flex-wrap: wrap; gap: 1rem; margin-left: auto; }
.site-header nav a { color: var(--muted); text-decoration: none; }
.site-header nav a:hover { color: var(--accent); }

/* ---------- Hero ---------- */
.hero {
  display: grid;
  gap: 1.5rem;
  align-items: center;
}
.avatar { border-radius: 50%; border: 4px solid var(--accent-soft); }
.hero h1 { font-size: clamp(2rem, 6vw, 3.25rem); margin: 0; }
.tagline { color: var(--muted); font-size: 1.1rem; margin-top: 0.25rem; }
.badges { display: flex; flex-wrap: wrap; gap: 0.5rem; }
.badge {
  background: var(--accent-soft);
  color: var(--accent);
  padding: 0.2rem 0.7rem;
  border-radius: 999px;
  font-size: 0.85rem;
  font-weight: 600;
}

/* ---------- Cards ---------- */
.card {
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  padding: 1.25rem;
}
.card h3 { margin-top: 0; }
.card ul { padding-left: 1.2rem; margin: 0; }

.skill-grid, .project-list {
  display: grid;
  gap: 1rem;
  grid-template-columns: repeat(auto-fill, minmax(15rem, 1fr));
}

.project { transition: transform 0.2s, box-shadow 0.2s; }
.project:hover { transform: translateY(-3px); box-shadow: 0 8px 24px rgb(0 0 0 / 0.12); }
.tech { color: var(--muted); font-size: 0.9rem; }

/* ---------- Experience timeline ---------- */
.timeline { list-style: none; padding: 0; border-left: 3px solid var(--accent-soft); }
.timeline li { padding: 0 0 1.5rem 1.25rem; position: relative; }
.timeline li::before {
  content: "";
  position: absolute;
  left: -0.55rem;
  top: 0.3rem;
  width: 0.8rem;
  height: 0.8rem;
  border-radius: 50%;
  background: var(--accent);
}
.timeline h3 { margin: 0; }
.when { color: var(--muted); margin: 0.2rem 0 0.5rem; }

/* ---------- Footer ---------- */
.site-footer { color: var(--muted); border-top: 1px solid var(--border); padding-block: 1.5rem; font-size: 0.9rem; }

/* ---------- Bigger screens ---------- */
@media (min-width: 700px) {
  .hero { grid-template-columns: auto 1fr; }
}

/* ---------- Motion ---------- */
@media (prefers-reduced-motion: reduce) {
  .project { transition: none; }
  .project:hover { transform: none; }
}

/* ---------- Print: the same page as a one-page résumé ---------- */
@media print {
  :root { --bg: white; --surface: white; --text: black; --muted: #333; --accent: black; --accent-soft: #eee; --border: #ccc; }
  .site-header nav, .skip, .project a { display: none; }
  section { padding-block: 0.75rem; break-inside: avoid; }
  .card { border: none; padding: 0.25rem 0; }
  .project:hover { transform: none; box-shadow: none; }
  a { color: black; text-decoration: none; }
  #contact a[href^="mailto"]::after { content: " (" attr(href) ")"; }
}
@@text
🎉 Put the link on your 職務経歴書, your LinkedIn and your GitHub profile. Keep adding each project you finish: by the end of this app, the page will show a recruiter everything you can do.

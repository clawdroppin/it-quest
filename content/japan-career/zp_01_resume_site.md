@@lesson xp-resume_site Bilingual Résumé (履歴書 and 職務経歴書)
@@text
Japanese employers expect a 履歴書 (a standard-format résumé) and a 職務経歴書 (a free-format career history). Keep your details in one JSON file and generate both as printable HTML pages (A4, with the expected sections), plus an English CV for foreign-capital companies, so updating one file updates all three.

**You'll practise:** 履歴書 and 職務経歴書 conventions · print CSS for A4 · generating HTML from data · presenting IT experience

## Milestones

1. **Data**: me.json with education, work history, certifications (資格) and skills.
2. **履歴書**: the standard sections in the usual table layout, with dates in 和暦 or 西暦 consistently.
3. **職務経歴書**: a summary (職務要約), each job's projects with your role, environment and results.
4. **English CV**: the same data, one page, achievement-focused.
5. **Print**: A4 page size with `@page`, check page breaks in the print preview.

## You're done when

- All three documents print cleanly on A4 and a reviewer finds no missing sections.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"What does a good 職務要約 look like for an infrastructure engineer?"*
- *"Should I use 和暦 or 西暦 on my 履歴書?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project resume_site Bilingual Résumé (履歴書 and 職務経歴書)
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Bilingual Résumé (履歴書 and 職務経歴書)

Japanese employers expect a 履歴書 (a standard-format résumé) and a 職務経歴書 (a free-format career history). Keep your details in one JSON file and generate both as printable HTML pages (A4, with the expected sections), plus an English CV for foreign-capital companies, so updating one file updates all three.

## Milestones

- [ ] 1. Data: me.json with education, work history, certifications (資格) and skills.
- [ ] 2. 履歴書: the standard sections in the usual table layout, with dates in 和暦 or 西暦 consistently.
- [ ] 3. 職務経歴書: a summary (職務要約), each job's projects with your role, environment and results.
- [ ] 4. English CV: the same data, one page, achievement-focused.
- [ ] 5. Print: A4 page size with `@page`, check page breaks in the print preview.

## Done when

- [ ] All three documents print cleanly on A4 and a reviewer finds no missing sections.
@@file build.py
"""Generate résumé pages from me.json: python build.py"""
import html
import json
from pathlib import Path

me = json.load(open("me.json", encoding="utf-8"))
rows = "".join(f"<tr><td>{c['date']}</td><td>{html.escape(c['name'])}</td></tr>" for c in me["certifications"])
page = f"""<!doctype html><html lang="ja"><meta charset="utf-8"><link rel="stylesheet" href="print.css">
<title>履歴書</title><h1>履歴書</h1><p>{html.escape(me['name'])}</p><h2>免許・資格</h2><table>{rows}</table></html>"""
Path("rirekisho.html").write_text(page, encoding="utf-8")
# TODO: education and work history, 職務経歴書, English CV
print("wrote rirekisho.html")
@@file me.json
{
  "name": "Your Name",
  "certifications": [
    { "date": "2026年4月", "name": "CompTIA A+" },
    { "date": "2026年9月", "name": "AWS Certified Cloud Practitioner" }
  ],
  "education": [],
  "work": []
}
@@file print.css
@page { size: A4; margin: 15mm; }
body { font-family: "Yu Gothic", "Noto Sans JP", sans-serif; font-size: 10.5pt; }
table { border-collapse: collapse; width: 100%; }
td { border: 1px solid #000; padding: 2mm; }

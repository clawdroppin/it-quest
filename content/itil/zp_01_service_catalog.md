@@lesson xp-service_catalog IT Service Catalog Site
@@text
A service catalog tells users what IT offers and what to expect. Build it as a small website: each service (email, laptops, VPN, software requests) with a description, who can use it, the service owner, support hours, SLA targets and how to request it. Generate the pages from services.json with a Python script, so updating the catalog is editing one file.

**You'll practise:** service catalog management · service level management · service owners · generating HTML

## Milestones

1. **Define**: services.json with at least six services and all the fields above.
2. **Generate**: build.py writes index.html and one page per service.
3. **SLAs**: show targets in plain language ("we answer within 1 business hour").
4. **Request**: each page links to the right request form or email.
5. **Review**: ask a non-IT person whether they can find how to get a new laptop.

## You're done when

- The generated site covers six services and a non-IT tester found what they needed.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"What's the difference between the business and technical service catalog?"*
- *"Help me write SLA targets users understand."*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project service_catalog IT Service Catalog Site
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# IT Service Catalog Site

A service catalog tells users what IT offers and what to expect. Build it as a small website: each service (email, laptops, VPN, software requests) with a description, who can use it, the service owner, support hours, SLA targets and how to request it. Generate the pages from services.json with a Python script, so updating the catalog is editing one file.

## Milestones

- [ ] 1. Define: services.json with at least six services and all the fields above.
- [ ] 2. Generate: build.py writes index.html and one page per service.
- [ ] 3. SLAs: show targets in plain language ("we answer within 1 business hour").
- [ ] 4. Request: each page links to the right request form or email.
- [ ] 5. Review: ask a non-IT person whether they can find how to get a new laptop.

## Done when

- [ ] The generated site covers six services and a non-IT tester found what they needed.
@@file build.py
"""Generate the service catalog site: python build.py"""
import html
import json
from pathlib import Path

services = json.load(open("services.json", encoding="utf-8"))
items = "\n".join(f'<li><a href="{s["id"]}.html">{html.escape(s["name"])}</a> · {html.escape(s["summary"])}</li>' for s in services)
Path("index.html").write_text(f"<!doctype html><html lang=en><meta charset=utf-8><title>IT Services</title><h1>IT Services</h1><ul>{items}</ul></html>", encoding="utf-8")
# TODO: one page per service with owner, hours, SLA and how to request it
print("wrote index.html")
@@file services.json
[
  { "id": "email", "name": "Email and calendar", "summary": "Outlook mailbox and shared calendars", "owner": "Ann (IT)", "hours": "24/7", "sla": "Restored within 4 hours" },
  { "id": "laptop", "name": "New laptop", "summary": "A standard laptop for new and existing staff", "owner": "Ken (IT)", "hours": "Weekdays 9–18", "sla": "Delivered within 5 business days" }
]

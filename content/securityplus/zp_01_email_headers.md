@@lesson xp-email_headers Suspicious Email Header Analyzer
@@text
Users forward suspicious emails to IT every day. Write a tool that reads a saved .eml file and reports the defensive checks an analyst makes: SPF, DKIM and DMARC results from Authentication-Results, a From domain that differs from Reply-To or Return-Path, link text that doesn't match the real link target, and attachments with risky extensions. Then write the friendly reply you'd send the user.

**You'll practise:** SPF, DKIM, DMARC · email headers · Python email package · security awareness

## Milestones

1. **Parse**: use `email.message_from_binary_file` with `policy.default`.
2. **Authentication**: extract spf=, dkim= and dmarc= results.
3. **Mismatches**: compare the domains of From, Reply-To and Return-Path.
4. **Links**: find `<a href>` tags whose visible text shows a different domain.
5. **Verdict**: a short risk summary and a template reply to the user who reported it.

## You're done when

- The sample email is flagged for its failed DMARC, mismatched Reply-To and misleading link.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"Explain how SPF, DKIM and DMARC work together."*
- *"How should IT reply to a user who reported a phishing email, to encourage reporting?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project email_headers Suspicious Email Header Analyzer
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Suspicious Email Header Analyzer

Users forward suspicious emails to IT every day. Write a tool that reads a saved .eml file and reports the defensive checks an analyst makes: SPF, DKIM and DMARC results from Authentication-Results, a From domain that differs from Reply-To or Return-Path, link text that doesn't match the real link target, and attachments with risky extensions. Then write the friendly reply you'd send the user.

## Milestones

- [ ] 1. Parse: use `email.message_from_binary_file` with `policy.default`.
- [ ] 2. Authentication: extract spf=, dkim= and dmarc= results.
- [ ] 3. Mismatches: compare the domains of From, Reply-To and Return-Path.
- [ ] 4. Links: find `<a href>` tags whose visible text shows a different domain.
- [ ] 5. Verdict: a short risk summary and a template reply to the user who reported it.

## Done when

- [ ] The sample email is flagged for its failed DMARC, mismatched Reply-To and misleading link.
@@file analyze.py
"""Analyze a saved email: python analyze.py sample.eml"""
import re
import sys
from email import message_from_binary_file, policy
from email.utils import parseaddr


def domain(address: str) -> str:
    return parseaddr(address)[1].rpartition("@")[2].lower()


msg = message_from_binary_file(open(sys.argv[1] if len(sys.argv) > 1 else "sample.eml", "rb"), policy=policy.default)
auth = msg.get("Authentication-Results", "")
print("From:", msg["From"], "| Reply-To:", msg.get("Reply-To"))
print("results:", re.findall(r"(spf|dkim|dmarc)=(\w+)", auth))
# TODO: domain mismatches, misleading links in the HTML part, risky attachments, a verdict
@@file sample.eml
Authentication-Results: mx.example.com; spf=softfail smtp.mailfrom=billing-notice.example.net; dkim=none; dmarc=fail header.from=example-bank.co.jp
From: "Example Bank" <support@example-bank.co.jp>
Reply-To: help@billing-notice.example.net
Return-Path: <bounce@billing-notice.example.net>
Subject: Your account will be suspended
MIME-Version: 1.0
Content-Type: text/html; charset=utf-8

<p>Please confirm your details within 24 hours:</p>
<p><a href="http://billing-notice.example.net/confirm">https://www.example-bank.co.jp/login</a></p>

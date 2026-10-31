@@lesson xp-keigo_mail Business Email Template Builder
@@text
Japanese business email follows fixed patterns: the opening (お世話になっております), self-introduction, the main request in 敬語, and the closing (よろしくお願いいたします). Build a tool that assembles emails for common IT situations (reporting an outage, asking for approval, apologizing for a delay) from phrase blocks, and quizzes you on choosing the right politeness level.

**You'll practise:** business email structure · 敬語: 尊敬語, 謙譲語, 丁寧語 · IT situations in Japanese

## Milestones

1. **Phrases**: phrases.json with openings, intros, requests and closings, with readings and meanings.
2. **Build**: choose a situation and a recipient (customer, boss, colleague) and assemble an email.
3. **Situations**: outage report, approval request, schedule change, apology for a delay.
4. **Quiz**: show a plain sentence and pick the correct humble or respectful version.
5. **Check**: ask a Japanese speaker (or the AI helper) to review three emails.

## You're done when

- You can write an outage report email to a customer with the right structure and politeness.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"Explain the difference between 申し上げる and おっしゃる with work examples."*
- *"Review this email I wrote to my manager for politeness and naturalness."*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project keigo_mail Business Email Template Builder
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Business Email Template Builder

Japanese business email follows fixed patterns: the opening (お世話になっております), self-introduction, the main request in 敬語, and the closing (よろしくお願いいたします). Build a tool that assembles emails for common IT situations (reporting an outage, asking for approval, apologizing for a delay) from phrase blocks, and quizzes you on choosing the right politeness level.

## Milestones

- [ ] 1. Phrases: phrases.json with openings, intros, requests and closings, with readings and meanings.
- [ ] 2. Build: choose a situation and a recipient (customer, boss, colleague) and assemble an email.
- [ ] 3. Situations: outage report, approval request, schedule change, apology for a delay.
- [ ] 4. Quiz: show a plain sentence and pick the correct humble or respectful version.
- [ ] 5. Check: ask a Japanese speaker (or the AI helper) to review three emails.

## Done when

- [ ] You can write an outage report email to a customer with the right structure and politeness.
@@file mail.py
"""Assemble a Japanese business email: python mail.py outage customer"""
import json
import sys

sys.stdout.reconfigure(encoding="utf-8")
phrases = json.load(open("phrases.json", encoding="utf-8"))
situation = sys.argv[1] if len(sys.argv) > 1 else "outage"
recipient = sys.argv[2] if len(sys.argv) > 2 else "customer"
print(phrases["opening"][recipient])
print(phrases["body"][situation])
print(phrases["closing"][recipient])
# TODO: more situations, the politeness quiz
@@file phrases.json
{
  "opening": { "customer": "いつも大変お世話になっております。", "boss": "お疲れ様です。", "colleague": "お疲れ様です。" },
  "body": {
    "outage": "本日10時頃より、メールシステムに障害が発生しております。現在、復旧に向けて対応中でございます。",
    "approval": "下記の件につきまして、ご承認いただけますでしょうか。"
  },
  "closing": { "customer": "ご迷惑をおかけし、誠に申し訳ございません。何卒よろしくお願いいたします。", "boss": "よろしくお願いいたします。", "colleague": "よろしくお願いします。" }
}

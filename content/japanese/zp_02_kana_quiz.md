@@lesson xp-kana_quiz Kana and IT Vocabulary Quiz
@@text
Build your own study tool: a quiz that shows hiragana, katakana or IT vocabulary (サーバー, 障害, 設定...), checks your answer, and schedules each card again sooner if you got it wrong (a simple Leitner box system), saving your progress between runs.

**You'll practise:** Leitner spaced repetition · Unicode text · JSON save files · random

## Milestones

1. **Cards**: cards.csv with front, back and deck (hiragana, katakana, it).
2. **Quiz**: ask 20 due cards, accept answers ignoring case and spaces.
3. **Boxes**: right moves a card up a box, wrong back to box 1; box n is due every 2^n days.
4. **Save**: progress.json keeps each card's box and next due date.
5. **Stats**: how many cards per box, and your accuracy this week.

## You're done when

- You've used it for a week and the stats show cards moving up the boxes.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"How does the Leitner system decide when to show a card again?"*
- *"Give me 30 more IT words used in Japanese job ads, with readings."*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project kana_quiz Kana and IT Vocabulary Quiz
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Kana and IT Vocabulary Quiz

Build your own study tool: a quiz that shows hiragana, katakana or IT vocabulary (サーバー, 障害, 設定...), checks your answer, and schedules each card again sooner if you got it wrong (a simple Leitner box system), saving your progress between runs.

## Milestones

- [ ] 1. Cards: cards.csv with front, back and deck (hiragana, katakana, it).
- [ ] 2. Quiz: ask 20 due cards, accept answers ignoring case and spaces.
- [ ] 3. Boxes: right moves a card up a box, wrong back to box 1; box n is due every 2^n days.
- [ ] 4. Save: progress.json keeps each card's box and next due date.
- [ ] 5. Stats: how many cards per box, and your accuracy this week.

## Done when

- [ ] You've used it for a week and the stats show cards moving up the boxes.
@@file quiz.py
"""Kana and IT vocabulary quiz: python quiz.py [deck]"""
import csv
import datetime as dt
import json
import random
import sys
from pathlib import Path

PROGRESS = Path("progress.json")


def load_cards(deck: str | None) -> list[dict]:
    cards = list(csv.DictReader(open("cards.csv", encoding="utf-8")))
    return [c for c in cards if deck is None or c["deck"] == deck]


if __name__ == "__main__":
    sys.stdout.reconfigure(encoding="utf-8")
    cards = load_cards(sys.argv[1] if len(sys.argv) > 1 else None)
    random.shuffle(cards)
    # TODO: only due cards, Leitner boxes, save progress
    for c in cards[:10]:
        answer = input(f"{c['front']} = ")
        print("✔" if answer.strip().lower() == c["back"].lower() else f"✘ {c['back']}")
@@file cards.csv
front,back,deck
あ,a,hiragana
し,shi,hiragana
つ,tsu,hiragana
ン,n,katakana
ソ,so,katakana
サーバー,server,it
障害,outage,it
設定,settings,it
本番環境,production environment,it
開発,development,it

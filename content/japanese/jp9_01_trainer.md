@@lesson jp9-trainer Workplace Japanese Trainer and Interview Kit
@@text
Vocabulary sticks when you review it **just before you'd forget it**. You'll build a **spaced-repetition** trainer, the idea behind Anki, for the Japanese you need at work: 障害, 復旧, 承知しました, 職務経歴書...

```text
[3/20]  暫定対応
reading> zanteitaiou
  正解!  ざんていたいおう (zanteitaiou): temporary fix, workaround
```

**Leitner boxes**: every card starts in box 1. Answer right and it moves up a box; wrong and it goes back to box 1. Higher boxes come back less often:

| Box | 1 | 2 | 3 | 4 | 5 |
|---|---|---|---|---|---|
| Review again in | 1 day | 2 days | 4 days | 8 days | 16 days |

You type the **reading**, in romaji or kana, so the program has to understand both. That's the fun part: a **kana → romaji converter** (Hepburn), with every tricky rule of the writing system you learned in this track:

- katakana → hiragana by code point (they're 0x60 apart in Unicode);
- combos (きょ → kyo, しゃ → sha, ファ → fa);
- small っ doubling the next consonant (きって → kitte, まっちゃ → matcha);
- ー repeating the previous vowel (サーバー → saabaa);
- forgiving answers: `Shou-Gai`, `しょうがい`, `ショウガイ`, full-width `ｓｈｏｕｇａｉ` and `shōgai` are all exact; `shogai` is "close" (long vowel missing) and still counts.

The kit also has `jikoshoukai.md` (a self-introduction template for interviews) and `interview.md` (the questions every Japanese interviewer asks, and questions to ask back).
@@project nihongo_trainer Workplace Japanese Trainer
Write `kana_to_romaji`, `check_answer`, `review`, `due_cards` and `sync` in `nihongo.py` until `python -m unittest` passes. Then use it daily: `python main.py`. Add your own words to `deck.txt`, and fill in `jikoshoukai.md` with your story.
@@file nihongo.py
"""Workplace Japanese trainer: kana → romaji, answer checking, and a Leitner spaced-repetition box."""
import json
import unicodedata
from datetime import date, timedelta

# ---------------------------------------------------------------- kana → romaji (Hepburn)

BASIC = {
    "あ": "a", "い": "i", "う": "u", "え": "e", "お": "o",
    "か": "ka", "き": "ki", "く": "ku", "け": "ke", "こ": "ko",
    "さ": "sa", "し": "shi", "す": "su", "せ": "se", "そ": "so",
    "た": "ta", "ち": "chi", "つ": "tsu", "て": "te", "と": "to",
    "な": "na", "に": "ni", "ぬ": "nu", "ね": "ne", "の": "no",
    "は": "ha", "ひ": "hi", "ふ": "fu", "へ": "he", "ほ": "ho",
    "ま": "ma", "み": "mi", "む": "mu", "め": "me", "も": "mo",
    "や": "ya", "ゆ": "yu", "よ": "yo",
    "ら": "ra", "り": "ri", "る": "ru", "れ": "re", "ろ": "ro",
    "わ": "wa", "を": "o", "ん": "n",
    "が": "ga", "ぎ": "gi", "ぐ": "gu", "げ": "ge", "ご": "go",
    "ざ": "za", "じ": "ji", "ず": "zu", "ぜ": "ze", "ぞ": "zo",
    "だ": "da", "ぢ": "ji", "づ": "zu", "で": "de", "ど": "do",
    "ば": "ba", "び": "bi", "ぶ": "bu", "べ": "be", "ぼ": "bo",
    "ぱ": "pa", "ぴ": "pi", "ぷ": "pu", "ぺ": "pe", "ぽ": "po",
    "ゔ": "vu",
    "ぁ": "a", "ぃ": "i", "ぅ": "u", "ぇ": "e", "ぉ": "o",
    "ゃ": "ya", "ゅ": "yu", "ょ": "yo",
}

# two-kana sounds: a kana + small ゃゅょ, or katakana-style + small vowel
COMBOS = {
    "しゃ": "sha", "しゅ": "shu", "しぇ": "she", "しょ": "sho",
    "ちゃ": "cha", "ちゅ": "chu", "ちぇ": "che", "ちょ": "cho",
    "じゃ": "ja", "じゅ": "ju", "じぇ": "je", "じょ": "jo",
    "ぢゃ": "ja", "ぢゅ": "ju", "ぢょ": "jo",
    "ふぁ": "fa", "ふぃ": "fi", "ふぇ": "fe", "ふぉ": "fo",
    "てぃ": "ti", "でぃ": "di", "でゅ": "dyu", "とぅ": "tu", "どぅ": "du",
    "うぃ": "wi", "うぇ": "we", "うぉ": "wo",
    "ゔぁ": "va", "ゔぃ": "vi", "ゔぇ": "ve", "ゔぉ": "vo",
}
for k, base in [("き", "k"), ("ぎ", "g"), ("に", "n"), ("ひ", "h"), ("び", "b"), ("ぴ", "p"),
                ("み", "m"), ("り", "r")]:
    for small, vowel in [("ゃ", "ya"), ("ゅ", "yu"), ("ょ", "yo")]:
        COMBOS[k + small] = base + vowel


def to_hiragana(text):
    """Katakana → hiragana (the two blocks are 0x60 apart). Other characters stay the same."""
    out = []
    for ch in text:
        code = ord(ch)
        if 0x30A1 <= code <= 0x30F6:
            out.append(chr(code - 0x60))
        else:
            out.append(ch)
    return "".join(out)


def kana_to_romaji(text):
    """Hepburn romaji for hiragana/katakana. ー repeats the previous vowel, っ doubles the next
    consonant (っち → tchi), and anything that isn't kana (spaces, Latin letters) is kept."""
    raise NotImplementedError


# ---------------------------------------------------------------- answer checking

LONG = {"ā": "aa", "ī": "ii", "ū": "uu", "ē": "ee", "ō": "ou", "â": "aa", "î": "ii", "û": "uu", "ê": "ee", "ô": "ou"}


def normalize(answer):
    """Lower case, full-width → half-width (NFKC), macrons spelled out, and spaces, hyphens,
    apostrophes and the Japanese full stop removed. Kana input is converted to romaji."""
    s = unicodedata.normalize("NFKC", answer).strip().lower()
    s = "".join(LONG.get(c, c) for c in s)
    s = kana_to_romaji(s)
    for junk in " -'’。、.":
        s = s.replace(junk, "")
    return s


def loose(romaji):
    """Forgive long-vowel spelling: ou/oo → o, uu → u, aa → a, ii → i, ee → e, ei → e."""
    for long, short in [("ou", "o"), ("oo", "o"), ("uu", "u"), ("aa", "a"), ("ii", "i"), ("ee", "e"), ("ei", "e")]:
        romaji = romaji.replace(long, short)
    return romaji


def check_answer(given, reading):
    """True if `given` (romaji or kana) matches the reading. Returns "exact", "close" (only long
    vowels differ: shogai for shougai) or None (wrong)."""
    raise NotImplementedError


# ---------------------------------------------------------------- deck and Leitner boxes

INTERVALS = {1: 1, 2: 2, 3: 4, 4: 8, 5: 16}   # box → days until the next review


def load_deck(lines):
    """Lines "japanese | reading | meaning". Skips blanks and # comments. The id is the japanese."""
    cards = []
    for n, line in enumerate(lines, 1):
        line = line.strip()
        if not line or line.startswith("#"):
            continue
        parts = [p.strip() for p in line.split("|")]
        if len(parts) != 3 or not all(parts):
            raise ValueError(f"line {n}: expected 'japanese | reading | meaning'")
        cards.append({"id": parts[0], "reading": parts[1], "meaning": parts[2]})
    return cards


def new_state(cards, today):
    """Every card starts in box 1, due today."""
    return {c["id"]: {"box": 1, "due": today.isoformat()} for c in cards}


def review(state, card_id, correct, today):
    """Right: move up one box (at most 5). Wrong: back to box 1. Then due = today + INTERVALS[box]."""
    raise NotImplementedError


def due_cards(state, today):
    """Ids due today or earlier, lowest box first, then oldest due date, then id."""
    raise NotImplementedError


def sync(state, cards, today):
    """Add new deck cards to the state and drop removed ones. Returns the state."""
    raise NotImplementedError


def box_counts(state):
    """{1: n, 2: n, ..., 5: n}"""
    counts = {b: 0 for b in INTERVALS}
    for v in state.values():
        counts[v["box"]] += 1
    return counts


def save(state, path):
    with open(path, "w", encoding="utf-8") as f:
        json.dump(state, f, ensure_ascii=False, indent=1)


def load(path):
    with open(path, encoding="utf-8") as f:
        return json.load(f)
@@file deck.txt
# Workplace and IT Japanese: japanese | reading (kana) | meaning
# Add your own lines. Readings in hiragana or katakana; you answer in romaji or kana.

# --- incidents and operations
障害 | しょうがい | incident, failure, outage
復旧 | ふっきゅう | recovery, restoring service
不具合 | ふぐあい | bug, defect, malfunction
原因 | げんいん | cause
調査 | ちょうさ | investigation
対応 | たいおう | handling, response, dealing with
暫定対応 | ざんていたいおう | temporary fix, workaround
恒久対応 | こうきゅうたいおう | permanent fix
再発防止 | さいはつぼうし | preventing recurrence
影響範囲 | えいきょうはんい | scope of impact
切り分け | きりわけ | isolating the problem (troubleshooting)
再起動 | さいきどう | restart, reboot
監視 | かんし | monitoring
運用保守 | うんようほしゅ | operations and maintenance
本番環境 | ほんばんかんきょう | production environment
検証環境 | けんしょうかんきょう | test / staging environment
手順書 | てじゅんしょ | procedure document, runbook
報告書 | ほうこくしょ | report (document)
# --- systems and settings
設定 | せってい | settings, configuration
権限 | けんげん | permission, privileges
認証 | にんしょう | authentication
暗号化 | あんごうか | encryption
バックアップ | バックアップ | backup
アカウント | アカウント | account
パスワード | パスワード | password
サーバー | サーバー | server
ネットワーク | ネットワーク | network
データベース | データベース | database
回線 | かいせん | line, connection (network circuit)
通信 | つうしん | communication, network traffic
容量 | ようりょう | capacity, storage size
端末 | たんまつ | terminal, device (PC or phone)
導入 | どうにゅう | introduction, rollout, installation
更新 | こうしん | update, renewal
脆弱性 | ぜいじゃくせい | vulnerability
# --- projects and documents
要件定義 | ようけんていぎ | requirements definition
設計書 | せっけいしょ | design document
見積もり | みつもり | estimate, quote
納期 | のうき | delivery deadline
進捗 | しんちょく | progress
課題 | かだい | issue, open problem (on an issue list)
仕様 | しよう | specification
申請 | しんせい | application, request (e.g. for access)
承認 | しょうにん | approval
# --- workplace phrases
お疲れ様です | おつかれさまです | hello / thanks for your work (standard greeting at work)
お世話になっております | おせわになっております | thank you for your continued support (email opener)
よろしくお願いいたします | よろしくおねがいいたします | I appreciate your help (closing phrase)
承知しました | しょうちしました | understood, certainly (polite)
かしこまりました | かしこまりました | certainly (very polite, to customers)
申し訳ございません | もうしわけございません | I am very sorry (formal apology)
恐れ入りますが | おそれいりますが | I'm sorry to trouble you, but...
少々お待ちください | しょうしょうおまちください | please wait a moment
確認いたします | かくにんいたします | I will check (humble)
折り返しご連絡いたします | おりかえしごれんらくいたします | I will get back to you
失礼いたします | しつれいいたします | excuse me / goodbye (formal)
報連相 | ほうれんそう | report, inform, consult (workplace communication rule)
# --- job hunting
履歴書 | りれきしょ | résumé (standard form)
職務経歴書 | しょくむけいれきしょ | career history document (CV)
面接 | めんせつ | interview
内定 | ないてい | job offer (informal, before contract)
自己紹介 | じこしょうかい | self-introduction
志望動機 | しぼうどうき | reason for applying
在留資格 | ざいりゅうしかく | residence status (visa category)
@@file test_nihongo.py
import os
import unittest
from datetime import date

from nihongo import (box_counts, check_answer, due_cards, kana_to_romaji, load_deck, new_state, review, sync,
                     to_hiragana)

TODAY = date(2026, 10, 31)


class RomajiTests(unittest.TestCase):
    def test_basic(self):
        self.assertEqual(kana_to_romaji("すし"), "sushi")
        self.assertEqual(kana_to_romaji("ちかてつ"), "chikatetsu")
        self.assertEqual(kana_to_romaji("ふじさん"), "fujisan")

    def test_combos(self):
        self.assertEqual(kana_to_romaji("しょうがい"), "shougai")
        self.assertEqual(kana_to_romaji("きょう"), "kyou")
        self.assertEqual(kana_to_romaji("じゅんび"), "junbi")
        self.assertEqual(kana_to_romaji("りょこう"), "ryokou")

    def test_small_tsu(self):
        self.assertEqual(kana_to_romaji("きって"), "kitte")
        self.assertEqual(kana_to_romaji("まっちゃ"), "matcha")
        self.assertEqual(kana_to_romaji("いっしょ"), "issho")

    def test_katakana_and_long_vowels(self):
        self.assertEqual(to_hiragana("サーバー"), "さーばー")
        self.assertEqual(kana_to_romaji("サーバー"), "saabaa")
        self.assertEqual(kana_to_romaji("パスワード"), "pasuwaado")
        self.assertEqual(kana_to_romaji("ネットワーク"), "nettowaaku")
        self.assertEqual(kana_to_romaji("ファイル"), "fairu")
        self.assertEqual(kana_to_romaji("ディスク"), "disuku")
        self.assertEqual(kana_to_romaji("ウィンドウズ"), "windouzu")

    def test_other_characters_kept(self):
        self.assertEqual(kana_to_romaji("PCを"), "PCo")
        self.assertEqual(kana_to_romaji("ー"), "")


class AnswerTests(unittest.TestCase):
    def test_exact(self):
        self.assertEqual(check_answer("shougai", "しょうがい"), "exact")
        self.assertEqual(check_answer("  Shou-Gai ", "しょうがい"), "exact")
        self.assertEqual(check_answer("しょうがい", "しょうがい"), "exact")
        self.assertEqual(check_answer("ショウガイ", "しょうがい"), "exact")
        self.assertEqual(check_answer("ｓｈｏｕｇａｉ", "しょうがい"), "exact")       # full-width letters
        self.assertEqual(check_answer("shōgai", "しょうがい"), "exact")

    def test_close(self):
        self.assertEqual(check_answer("shogai", "しょうがい"), "close")
        self.assertEqual(check_answer("saba", "サーバー"), "close")
        self.assertEqual(check_answer("sensei", "せんせい"), "exact")
        self.assertEqual(check_answer("sense", "せんせい"), "close")

    def test_wrong(self):
        self.assertIsNone(check_answer("shoukai", "しょうがい"))
        self.assertIsNone(check_answer("", "しょうがい"))
        self.assertIsNone(check_answer("   ", "しょうがい"))


class LeitnerTests(unittest.TestCase):
    def setUp(self):
        self.cards = load_deck(["# deck", "", "障害 | しょうがい | incident, failure",
                                "設定 | せってい | settings, configuration", "復旧 | ふっきゅう | recovery"])
        self.state = new_state(self.cards, TODAY)

    def test_deck(self):
        self.assertEqual([c["id"] for c in self.cards], ["障害", "設定", "復旧"])
        self.assertEqual(self.cards[1], {"id": "設定", "reading": "せってい", "meaning": "settings, configuration"})
        with self.assertRaises(ValueError):
            load_deck(["障害 | しょうがい"])
        with self.assertRaises(ValueError):
            load_deck(["障害 | | incident"])

    def test_starts_in_box_1_due_today(self):
        self.assertEqual(self.state["障害"], {"box": 1, "due": "2026-10-31"})
        self.assertEqual(due_cards(self.state, TODAY), ["復旧", "設定", "障害"])   # same box/due: by id

    def test_review_moves_boxes(self):
        self.assertEqual(review(self.state, "障害", True, TODAY), {"box": 2, "due": "2026-11-02"})
        self.assertEqual(review(self.state, "障害", True, TODAY), {"box": 3, "due": "2026-11-04"})
        self.assertEqual(review(self.state, "障害", False, TODAY), {"box": 1, "due": "2026-11-01"})

    def test_box_5_is_the_top(self):
        for _ in range(7):
            review(self.state, "設定", True, TODAY)
        self.assertEqual(self.state["設定"], {"box": 5, "due": "2026-11-16"})

    def test_due_order(self):
        review(self.state, "障害", True, TODAY)              # box 2, due 11-02
        review(self.state, "復旧", False, TODAY)             # box 1, due 11-01
        self.assertEqual(due_cards(self.state, TODAY), ["設定"])
        self.assertEqual(due_cards(self.state, date(2026, 11, 5)), ["設定", "復旧", "障害"])

    def test_sync_and_counts(self):
        review(self.state, "障害", True, TODAY)
        cards = self.cards[:1] + [{"id": "手順書", "reading": "てじゅんしょ", "meaning": "procedure document"}]
        sync(self.state, cards, TODAY)
        self.assertEqual(sorted(self.state), ["手順書", "障害"])
        self.assertEqual(self.state["障害"]["box"], 2, "keeps progress")
        self.assertEqual(box_counts(self.state), {1: 1, 2: 1, 3: 0, 4: 0, 5: 0})

    def test_real_deck_loads(self):
        path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "deck.txt")
        with open(path, encoding="utf-8") as f:
            cards = load_deck(f)
        self.assertGreaterEqual(len(cards), 50)
        self.assertEqual(len({c["id"] for c in cards}), len(cards), "ids must be unique")
        for c in cards:
            with self.subTest(card=c["id"]):
                self.assertEqual(check_answer(c["reading"], c["reading"]), "exact")


if __name__ == "__main__":
    unittest.main()
@@file main.py
"""Workplace Japanese trainer with spaced repetition.

    python main.py            review today's cards (up to 20)
    python main.py 50         up to 50 cards
    python main.py --stats    how many cards are in each box

Type the reading in romaji (shougai) or kana (しょうがい). Progress is saved in progress.json.
"""
import os
import sys
from datetime import date

import nihongo as jp

HERE = os.path.dirname(os.path.abspath(__file__))
DECK = os.path.join(HERE, "deck.txt")
PROGRESS = os.path.join(HERE, "progress.json")


def main(argv):
    sys.stdout.reconfigure(errors="replace")
    today = date.today()
    with open(DECK, encoding="utf-8") as f:
        cards = jp.load_deck(f)
    by_id = {c["id"]: c for c in cards}
    state = jp.load(PROGRESS) if os.path.exists(PROGRESS) else jp.new_state(cards, today)
    jp.sync(state, cards, today)

    if "--stats" in argv:
        counts = jp.box_counts(state)
        for box, n in counts.items():
            print(f"box {box} (every {jp.INTERVALS[box]:>2} days): {'#' * n} {n}")
        print(f"due today: {len(jp.due_cards(state, today))}")
        return 0

    limit = int(argv[0]) if argv and argv[0].isdigit() else 20
    due = jp.due_cards(state, today)[:limit]
    if not due:
        print("Nothing due today. お疲れ様でした!")
        return 0
    right = 0
    for n, card_id in enumerate(due, 1):
        card = by_id[card_id]
        print(f"\n[{n}/{len(due)}]  {card['id']}")
        try:
            given = input("reading> ")
        except EOFError:
            break
        result = jp.check_answer(given, card["reading"])
        romaji = jp.kana_to_romaji(card["reading"])
        if result == "exact":
            print(f"  正解!  {card['reading']} ({romaji}): {card['meaning']}")
        elif result == "close":
            print(f"  Nearly: watch the long vowels. {card['reading']} ({romaji}): {card['meaning']}")
        else:
            print(f"  ✗  {card['reading']} ({romaji}): {card['meaning']}")
        jp.review(state, card_id, result is not None, today)
        right += result is not None
        jp.save(state, PROGRESS)
    print(f"\n{right}/{len(due)} right. See you tomorrow!")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
@@file jikoshoukai.md
# 自己紹介 (self-introduction) template

Practise this out loud until you can say it in about **one minute** without reading.
Replace the 〇〇 parts. Polite (です/ます) style throughout.

```
はじめまして。〇〇と申します。
〇〇から参りました。日本に来て〇年になります。 (or: 現在は〇〇に住んでおります。)

前職では、〇〇株式会社で〇年間、ITサポートを担当しておりました。
主に、PCのセットアップ、アカウント管理、ネットワークの障害対応などを行っておりました。

また、PowerShell と Python を使って、定型作業の自動化に取り組みました。
例えば、新入社員のアカウント作成を自動化し、作業時間を〇割削減しました。

資格は、CompTIA A+ と 基本情報技術者 を取得しております。
現在は AWS の資格の勉強をしております。

御社では、これまでの経験を活かして、安定したシステム運用に貢献したいと考えております。
どうぞよろしくお願いいたします。
```

## The pattern
1. Name: 〇〇と申します (humble "I am called")
2. Where from / how long in Japan
3. Experience: 〜を担当しておりました ("I was in charge of")
4. One concrete achievement with a number (作業時間を30%削減)
5. Qualifications: 〜を取得しております
6. What you want to do at *their* company: 御社で〜に貢献したい
7. よろしくお願いいたします

Words: 御社 (おんしゃ, "your company", spoken) / 貴社 (きしゃ, written, in documents).
@@file interview.md
# よくある面接の質問 (common interview questions)

Prepare an answer for each one, in Japanese, and a short English version too
(外資系 companies may interview in English).

| 質問 | Meaning | Tip |
|---|---|---|
| 自己紹介をお願いします。 | Please introduce yourself. | 1 minute: use jikoshoukai.md |
| 志望動機を教えてください。 | Why do you want to join us? | something specific to this company, not "Japan is nice" |
| なぜ日本で働きたいのですか。 | Why do you want to work in Japan? | long-term plan: shows you'll stay |
| 前職の退職理由は何ですか。 | Why did you leave your last job? | positive: growth, new challenge; never criticize the old employer |
| あなたの強みと弱みは何ですか。 | Strengths and weaknesses? | weakness + what you do about it |
| チームで働いた経験を教えてください。 | Experience working in a team? | a short story: situation, your action, result |
| 障害対応の経験はありますか。 | Experience handling incidents? | 切り分け → 暫定対応 → 報告 → 恒久対応 |
| 日本語はどのくらい話せますか。 | How good is your Japanese? | JLPT level, and an honest example of what you can do |
| 残業は大丈夫ですか。 | Is overtime OK? | honest but flexible: 繁忙期は対応可能です |
| 何か質問はありますか。 | Do you have any questions? | ALWAYS ask one or two: team size, tools, training, on-call |

## 逆質問 (questions to ask them)
- 配属予定のチームの体制を教えていただけますか。(How is the team I'd join organized?)
- 入社後の研修制度はありますか。(Is there training after joining?)
- 資格取得の支援制度はありますか。(Do you support getting certifications?)
- 夜間や休日の対応はありますか。(Is there night/weekend on-call?)

## Manners (面接マナー)
- Arrive 5-10 minutes early; for online interviews, test the camera and connection.
- Knock 3 times, 「失礼いたします」, wait to be asked to sit (「どうぞおかけください」).
- At the end: 「本日はお忙しいところ、ありがとうございました」.
@@text
> Tips: in `kana_to_romaji`, loop with an index so you can look at **two** characters (`text[i:i + 2]`) for combos before falling back to one. Remember a flag when you see っ and apply it to the next syllable's first consonant (`ch` becomes `tch`). For ー, find the last vowel already in the output. In `check_answer`, `normalize` both sides first, then compare exactly, then with `loose()`.
@@project nihongo_trainer_finished Workplace Japanese Trainer: the finished version
The complete trainer. Next steps: a reverse mode (show the English meaning, type the reading), audio with a text-to-speech engine, importing an Anki deck exported as text, or a streak counter in `--stats`.
@@file nihongo.py
"""Workplace Japanese trainer: kana → romaji, answer checking, and a Leitner spaced-repetition box."""
import json
import unicodedata
from datetime import date, timedelta

# ---------------------------------------------------------------- kana → romaji (Hepburn)

BASIC = {
    "あ": "a", "い": "i", "う": "u", "え": "e", "お": "o",
    "か": "ka", "き": "ki", "く": "ku", "け": "ke", "こ": "ko",
    "さ": "sa", "し": "shi", "す": "su", "せ": "se", "そ": "so",
    "た": "ta", "ち": "chi", "つ": "tsu", "て": "te", "と": "to",
    "な": "na", "に": "ni", "ぬ": "nu", "ね": "ne", "の": "no",
    "は": "ha", "ひ": "hi", "ふ": "fu", "へ": "he", "ほ": "ho",
    "ま": "ma", "み": "mi", "む": "mu", "め": "me", "も": "mo",
    "や": "ya", "ゆ": "yu", "よ": "yo",
    "ら": "ra", "り": "ri", "る": "ru", "れ": "re", "ろ": "ro",
    "わ": "wa", "を": "o", "ん": "n",
    "が": "ga", "ぎ": "gi", "ぐ": "gu", "げ": "ge", "ご": "go",
    "ざ": "za", "じ": "ji", "ず": "zu", "ぜ": "ze", "ぞ": "zo",
    "だ": "da", "ぢ": "ji", "づ": "zu", "で": "de", "ど": "do",
    "ば": "ba", "び": "bi", "ぶ": "bu", "べ": "be", "ぼ": "bo",
    "ぱ": "pa", "ぴ": "pi", "ぷ": "pu", "ぺ": "pe", "ぽ": "po",
    "ゔ": "vu",
    "ぁ": "a", "ぃ": "i", "ぅ": "u", "ぇ": "e", "ぉ": "o",
    "ゃ": "ya", "ゅ": "yu", "ょ": "yo",
}

# two-kana sounds: a kana + small ゃゅょ, or katakana-style + small vowel
COMBOS = {
    "しゃ": "sha", "しゅ": "shu", "しぇ": "she", "しょ": "sho",
    "ちゃ": "cha", "ちゅ": "chu", "ちぇ": "che", "ちょ": "cho",
    "じゃ": "ja", "じゅ": "ju", "じぇ": "je", "じょ": "jo",
    "ぢゃ": "ja", "ぢゅ": "ju", "ぢょ": "jo",
    "ふぁ": "fa", "ふぃ": "fi", "ふぇ": "fe", "ふぉ": "fo",
    "てぃ": "ti", "でぃ": "di", "でゅ": "dyu", "とぅ": "tu", "どぅ": "du",
    "うぃ": "wi", "うぇ": "we", "うぉ": "wo",
    "ゔぁ": "va", "ゔぃ": "vi", "ゔぇ": "ve", "ゔぉ": "vo",
}
for k, base in [("き", "k"), ("ぎ", "g"), ("に", "n"), ("ひ", "h"), ("び", "b"), ("ぴ", "p"),
                ("み", "m"), ("り", "r")]:
    for small, vowel in [("ゃ", "ya"), ("ゅ", "yu"), ("ょ", "yo")]:
        COMBOS[k + small] = base + vowel


def to_hiragana(text):
    """Katakana → hiragana (the two blocks are 0x60 apart). Other characters stay the same."""
    out = []
    for ch in text:
        code = ord(ch)
        if 0x30A1 <= code <= 0x30F6:
            out.append(chr(code - 0x60))
        else:
            out.append(ch)
    return "".join(out)


def kana_to_romaji(text):
    """Hepburn romaji for hiragana/katakana. ー repeats the previous vowel, っ doubles the next
    consonant (っち → tchi), and anything that isn't kana (spaces, Latin letters) is kept."""
    text = to_hiragana(text)
    out = ""
    i = 0
    double_next = False
    while i < len(text):
        pair, ch = text[i:i + 2], text[i]
        if ch == "っ":
            double_next = True
            i += 1
            continue
        if ch == "ー":
            vowels = [c for c in out if c in "aeiou"]
            out += vowels[-1] if vowels else ""
            i += 1
            continue
        if pair in COMBOS:
            syllable, i = COMBOS[pair], i + 2
        elif ch in BASIC:
            syllable, i = BASIC[ch], i + 1
        else:
            syllable, i = ch, i + 1
        if double_next and syllable[:1].isalpha() and syllable[0] not in "aeiou":
            syllable = ("t" if syllable.startswith("ch") else syllable[0]) + syllable
        double_next = False
        out += syllable
    return out


# ---------------------------------------------------------------- answer checking

LONG = {"ā": "aa", "ī": "ii", "ū": "uu", "ē": "ee", "ō": "ou", "â": "aa", "î": "ii", "û": "uu", "ê": "ee", "ô": "ou"}


def normalize(answer):
    """Lower case, full-width → half-width (NFKC), macrons spelled out, and spaces, hyphens,
    apostrophes and the Japanese full stop removed. Kana input is converted to romaji."""
    s = unicodedata.normalize("NFKC", answer).strip().lower()
    s = "".join(LONG.get(c, c) for c in s)
    s = kana_to_romaji(s)
    for junk in " -'’。、.":
        s = s.replace(junk, "")
    return s


def loose(romaji):
    """Forgive long-vowel spelling: ou/oo → o, uu → u, aa → a, ii → i, ee → e, ei → e."""
    for long, short in [("ou", "o"), ("oo", "o"), ("uu", "u"), ("aa", "a"), ("ii", "i"), ("ee", "e"), ("ei", "e")]:
        romaji = romaji.replace(long, short)
    return romaji


def check_answer(given, reading):
    """True if `given` (romaji or kana) matches the reading. Returns "exact", "close" (only long
    vowels differ: shogai for shougai) or None (wrong)."""
    g, r = normalize(given), normalize(reading)
    if not g:
        return None
    if g == r:
        return "exact"
    if loose(g) == loose(r):
        return "close"
    return None


# ---------------------------------------------------------------- deck and Leitner boxes

INTERVALS = {1: 1, 2: 2, 3: 4, 4: 8, 5: 16}   # box → days until the next review


def load_deck(lines):
    """Lines "japanese | reading | meaning". Skips blanks and # comments. The id is the japanese."""
    cards = []
    for n, line in enumerate(lines, 1):
        line = line.strip()
        if not line or line.startswith("#"):
            continue
        parts = [p.strip() for p in line.split("|")]
        if len(parts) != 3 or not all(parts):
            raise ValueError(f"line {n}: expected 'japanese | reading | meaning'")
        cards.append({"id": parts[0], "reading": parts[1], "meaning": parts[2]})
    return cards


def new_state(cards, today):
    """Every card starts in box 1, due today."""
    return {c["id"]: {"box": 1, "due": today.isoformat()} for c in cards}


def review(state, card_id, correct, today):
    """Right: move up one box (at most 5). Wrong: back to box 1. Then due = today + INTERVALS[box]."""
    entry = state[card_id]
    entry["box"] = min(entry["box"] + 1, 5) if correct else 1
    entry["due"] = (today + timedelta(days=INTERVALS[entry["box"]])).isoformat()
    return entry


def due_cards(state, today):
    """Ids due today or earlier, lowest box first, then oldest due date, then id."""
    due = [(v["box"], v["due"], k) for k, v in state.items() if date.fromisoformat(v["due"]) <= today]
    return [k for _, _, k in sorted(due)]


def sync(state, cards, today):
    """Add new deck cards to the state and drop removed ones. Returns the state."""
    ids = {c["id"] for c in cards}
    for c in cards:
        state.setdefault(c["id"], {"box": 1, "due": today.isoformat()})
    for k in list(state):
        if k not in ids:
            del state[k]
    return state


def box_counts(state):
    """{1: n, 2: n, ..., 5: n}"""
    counts = {b: 0 for b in INTERVALS}
    for v in state.values():
        counts[v["box"]] += 1
    return counts


def save(state, path):
    with open(path, "w", encoding="utf-8") as f:
        json.dump(state, f, ensure_ascii=False, indent=1)


def load(path):
    with open(path, encoding="utf-8") as f:
        return json.load(f)
@@file deck.txt
# Workplace and IT Japanese: japanese | reading (kana) | meaning
# Add your own lines. Readings in hiragana or katakana; you answer in romaji or kana.

# --- incidents and operations
障害 | しょうがい | incident, failure, outage
復旧 | ふっきゅう | recovery, restoring service
不具合 | ふぐあい | bug, defect, malfunction
原因 | げんいん | cause
調査 | ちょうさ | investigation
対応 | たいおう | handling, response, dealing with
暫定対応 | ざんていたいおう | temporary fix, workaround
恒久対応 | こうきゅうたいおう | permanent fix
再発防止 | さいはつぼうし | preventing recurrence
影響範囲 | えいきょうはんい | scope of impact
切り分け | きりわけ | isolating the problem (troubleshooting)
再起動 | さいきどう | restart, reboot
監視 | かんし | monitoring
運用保守 | うんようほしゅ | operations and maintenance
本番環境 | ほんばんかんきょう | production environment
検証環境 | けんしょうかんきょう | test / staging environment
手順書 | てじゅんしょ | procedure document, runbook
報告書 | ほうこくしょ | report (document)
# --- systems and settings
設定 | せってい | settings, configuration
権限 | けんげん | permission, privileges
認証 | にんしょう | authentication
暗号化 | あんごうか | encryption
バックアップ | バックアップ | backup
アカウント | アカウント | account
パスワード | パスワード | password
サーバー | サーバー | server
ネットワーク | ネットワーク | network
データベース | データベース | database
回線 | かいせん | line, connection (network circuit)
通信 | つうしん | communication, network traffic
容量 | ようりょう | capacity, storage size
端末 | たんまつ | terminal, device (PC or phone)
導入 | どうにゅう | introduction, rollout, installation
更新 | こうしん | update, renewal
脆弱性 | ぜいじゃくせい | vulnerability
# --- projects and documents
要件定義 | ようけんていぎ | requirements definition
設計書 | せっけいしょ | design document
見積もり | みつもり | estimate, quote
納期 | のうき | delivery deadline
進捗 | しんちょく | progress
課題 | かだい | issue, open problem (on an issue list)
仕様 | しよう | specification
申請 | しんせい | application, request (e.g. for access)
承認 | しょうにん | approval
# --- workplace phrases
お疲れ様です | おつかれさまです | hello / thanks for your work (standard greeting at work)
お世話になっております | おせわになっております | thank you for your continued support (email opener)
よろしくお願いいたします | よろしくおねがいいたします | I appreciate your help (closing phrase)
承知しました | しょうちしました | understood, certainly (polite)
かしこまりました | かしこまりました | certainly (very polite, to customers)
申し訳ございません | もうしわけございません | I am very sorry (formal apology)
恐れ入りますが | おそれいりますが | I'm sorry to trouble you, but...
少々お待ちください | しょうしょうおまちください | please wait a moment
確認いたします | かくにんいたします | I will check (humble)
折り返しご連絡いたします | おりかえしごれんらくいたします | I will get back to you
失礼いたします | しつれいいたします | excuse me / goodbye (formal)
報連相 | ほうれんそう | report, inform, consult (workplace communication rule)
# --- job hunting
履歴書 | りれきしょ | résumé (standard form)
職務経歴書 | しょくむけいれきしょ | career history document (CV)
面接 | めんせつ | interview
内定 | ないてい | job offer (informal, before contract)
自己紹介 | じこしょうかい | self-introduction
志望動機 | しぼうどうき | reason for applying
在留資格 | ざいりゅうしかく | residence status (visa category)
@@file test_nihongo.py
import os
import unittest
from datetime import date

from nihongo import (box_counts, check_answer, due_cards, kana_to_romaji, load_deck, new_state, review, sync,
                     to_hiragana)

TODAY = date(2026, 10, 31)


class RomajiTests(unittest.TestCase):
    def test_basic(self):
        self.assertEqual(kana_to_romaji("すし"), "sushi")
        self.assertEqual(kana_to_romaji("ちかてつ"), "chikatetsu")
        self.assertEqual(kana_to_romaji("ふじさん"), "fujisan")

    def test_combos(self):
        self.assertEqual(kana_to_romaji("しょうがい"), "shougai")
        self.assertEqual(kana_to_romaji("きょう"), "kyou")
        self.assertEqual(kana_to_romaji("じゅんび"), "junbi")
        self.assertEqual(kana_to_romaji("りょこう"), "ryokou")

    def test_small_tsu(self):
        self.assertEqual(kana_to_romaji("きって"), "kitte")
        self.assertEqual(kana_to_romaji("まっちゃ"), "matcha")
        self.assertEqual(kana_to_romaji("いっしょ"), "issho")

    def test_katakana_and_long_vowels(self):
        self.assertEqual(to_hiragana("サーバー"), "さーばー")
        self.assertEqual(kana_to_romaji("サーバー"), "saabaa")
        self.assertEqual(kana_to_romaji("パスワード"), "pasuwaado")
        self.assertEqual(kana_to_romaji("ネットワーク"), "nettowaaku")
        self.assertEqual(kana_to_romaji("ファイル"), "fairu")
        self.assertEqual(kana_to_romaji("ディスク"), "disuku")
        self.assertEqual(kana_to_romaji("ウィンドウズ"), "windouzu")

    def test_other_characters_kept(self):
        self.assertEqual(kana_to_romaji("PCを"), "PCo")
        self.assertEqual(kana_to_romaji("ー"), "")


class AnswerTests(unittest.TestCase):
    def test_exact(self):
        self.assertEqual(check_answer("shougai", "しょうがい"), "exact")
        self.assertEqual(check_answer("  Shou-Gai ", "しょうがい"), "exact")
        self.assertEqual(check_answer("しょうがい", "しょうがい"), "exact")
        self.assertEqual(check_answer("ショウガイ", "しょうがい"), "exact")
        self.assertEqual(check_answer("ｓｈｏｕｇａｉ", "しょうがい"), "exact")       # full-width letters
        self.assertEqual(check_answer("shōgai", "しょうがい"), "exact")

    def test_close(self):
        self.assertEqual(check_answer("shogai", "しょうがい"), "close")
        self.assertEqual(check_answer("saba", "サーバー"), "close")
        self.assertEqual(check_answer("sensei", "せんせい"), "exact")
        self.assertEqual(check_answer("sense", "せんせい"), "close")

    def test_wrong(self):
        self.assertIsNone(check_answer("shoukai", "しょうがい"))
        self.assertIsNone(check_answer("", "しょうがい"))
        self.assertIsNone(check_answer("   ", "しょうがい"))


class LeitnerTests(unittest.TestCase):
    def setUp(self):
        self.cards = load_deck(["# deck", "", "障害 | しょうがい | incident, failure",
                                "設定 | せってい | settings, configuration", "復旧 | ふっきゅう | recovery"])
        self.state = new_state(self.cards, TODAY)

    def test_deck(self):
        self.assertEqual([c["id"] for c in self.cards], ["障害", "設定", "復旧"])
        self.assertEqual(self.cards[1], {"id": "設定", "reading": "せってい", "meaning": "settings, configuration"})
        with self.assertRaises(ValueError):
            load_deck(["障害 | しょうがい"])
        with self.assertRaises(ValueError):
            load_deck(["障害 | | incident"])

    def test_starts_in_box_1_due_today(self):
        self.assertEqual(self.state["障害"], {"box": 1, "due": "2026-10-31"})
        self.assertEqual(due_cards(self.state, TODAY), ["復旧", "設定", "障害"])   # same box/due: by id

    def test_review_moves_boxes(self):
        self.assertEqual(review(self.state, "障害", True, TODAY), {"box": 2, "due": "2026-11-02"})
        self.assertEqual(review(self.state, "障害", True, TODAY), {"box": 3, "due": "2026-11-04"})
        self.assertEqual(review(self.state, "障害", False, TODAY), {"box": 1, "due": "2026-11-01"})

    def test_box_5_is_the_top(self):
        for _ in range(7):
            review(self.state, "設定", True, TODAY)
        self.assertEqual(self.state["設定"], {"box": 5, "due": "2026-11-16"})

    def test_due_order(self):
        review(self.state, "障害", True, TODAY)              # box 2, due 11-02
        review(self.state, "復旧", False, TODAY)             # box 1, due 11-01
        self.assertEqual(due_cards(self.state, TODAY), ["設定"])
        self.assertEqual(due_cards(self.state, date(2026, 11, 5)), ["設定", "復旧", "障害"])

    def test_sync_and_counts(self):
        review(self.state, "障害", True, TODAY)
        cards = self.cards[:1] + [{"id": "手順書", "reading": "てじゅんしょ", "meaning": "procedure document"}]
        sync(self.state, cards, TODAY)
        self.assertEqual(sorted(self.state), ["手順書", "障害"])
        self.assertEqual(self.state["障害"]["box"], 2, "keeps progress")
        self.assertEqual(box_counts(self.state), {1: 1, 2: 1, 3: 0, 4: 0, 5: 0})

    def test_real_deck_loads(self):
        path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "deck.txt")
        with open(path, encoding="utf-8") as f:
            cards = load_deck(f)
        self.assertGreaterEqual(len(cards), 50)
        self.assertEqual(len({c["id"] for c in cards}), len(cards), "ids must be unique")
        for c in cards:
            with self.subTest(card=c["id"]):
                self.assertEqual(check_answer(c["reading"], c["reading"]), "exact")


if __name__ == "__main__":
    unittest.main()
@@file main.py
"""Workplace Japanese trainer with spaced repetition.

    python main.py            review today's cards (up to 20)
    python main.py 50         up to 50 cards
    python main.py --stats    how many cards are in each box

Type the reading in romaji (shougai) or kana (しょうがい). Progress is saved in progress.json.
"""
import os
import sys
from datetime import date

import nihongo as jp

HERE = os.path.dirname(os.path.abspath(__file__))
DECK = os.path.join(HERE, "deck.txt")
PROGRESS = os.path.join(HERE, "progress.json")


def main(argv):
    sys.stdout.reconfigure(errors="replace")
    today = date.today()
    with open(DECK, encoding="utf-8") as f:
        cards = jp.load_deck(f)
    by_id = {c["id"]: c for c in cards}
    state = jp.load(PROGRESS) if os.path.exists(PROGRESS) else jp.new_state(cards, today)
    jp.sync(state, cards, today)

    if "--stats" in argv:
        counts = jp.box_counts(state)
        for box, n in counts.items():
            print(f"box {box} (every {jp.INTERVALS[box]:>2} days): {'#' * n} {n}")
        print(f"due today: {len(jp.due_cards(state, today))}")
        return 0

    limit = int(argv[0]) if argv and argv[0].isdigit() else 20
    due = jp.due_cards(state, today)[:limit]
    if not due:
        print("Nothing due today. お疲れ様でした!")
        return 0
    right = 0
    for n, card_id in enumerate(due, 1):
        card = by_id[card_id]
        print(f"\n[{n}/{len(due)}]  {card['id']}")
        try:
            given = input("reading> ")
        except EOFError:
            break
        result = jp.check_answer(given, card["reading"])
        romaji = jp.kana_to_romaji(card["reading"])
        if result == "exact":
            print(f"  正解!  {card['reading']} ({romaji}): {card['meaning']}")
        elif result == "close":
            print(f"  Nearly: watch the long vowels. {card['reading']} ({romaji}): {card['meaning']}")
        else:
            print(f"  ✗  {card['reading']} ({romaji}): {card['meaning']}")
        jp.review(state, card_id, result is not None, today)
        right += result is not None
        jp.save(state, PROGRESS)
    print(f"\n{right}/{len(due)} right. See you tomorrow!")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
@@file jikoshoukai.md
# 自己紹介 (self-introduction) template

Practise this out loud until you can say it in about **one minute** without reading.
Replace the 〇〇 parts. Polite (です/ます) style throughout.

```
はじめまして。〇〇と申します。
〇〇から参りました。日本に来て〇年になります。 (or: 現在は〇〇に住んでおります。)

前職では、〇〇株式会社で〇年間、ITサポートを担当しておりました。
主に、PCのセットアップ、アカウント管理、ネットワークの障害対応などを行っておりました。

また、PowerShell と Python を使って、定型作業の自動化に取り組みました。
例えば、新入社員のアカウント作成を自動化し、作業時間を〇割削減しました。

資格は、CompTIA A+ と 基本情報技術者 を取得しております。
現在は AWS の資格の勉強をしております。

御社では、これまでの経験を活かして、安定したシステム運用に貢献したいと考えております。
どうぞよろしくお願いいたします。
```

## The pattern
1. Name: 〇〇と申します (humble "I am called")
2. Where from / how long in Japan
3. Experience: 〜を担当しておりました ("I was in charge of")
4. One concrete achievement with a number (作業時間を30%削減)
5. Qualifications: 〜を取得しております
6. What you want to do at *their* company: 御社で〜に貢献したい
7. よろしくお願いいたします

Words: 御社 (おんしゃ, "your company", spoken) / 貴社 (きしゃ, written, in documents).
@@file interview.md
# よくある面接の質問 (common interview questions)

Prepare an answer for each one, in Japanese, and a short English version too
(外資系 companies may interview in English).

| 質問 | Meaning | Tip |
|---|---|---|
| 自己紹介をお願いします。 | Please introduce yourself. | 1 minute: use jikoshoukai.md |
| 志望動機を教えてください。 | Why do you want to join us? | something specific to this company, not "Japan is nice" |
| なぜ日本で働きたいのですか。 | Why do you want to work in Japan? | long-term plan: shows you'll stay |
| 前職の退職理由は何ですか。 | Why did you leave your last job? | positive: growth, new challenge; never criticize the old employer |
| あなたの強みと弱みは何ですか。 | Strengths and weaknesses? | weakness + what you do about it |
| チームで働いた経験を教えてください。 | Experience working in a team? | a short story: situation, your action, result |
| 障害対応の経験はありますか。 | Experience handling incidents? | 切り分け → 暫定対応 → 報告 → 恒久対応 |
| 日本語はどのくらい話せますか。 | How good is your Japanese? | JLPT level, and an honest example of what you can do |
| 残業は大丈夫ですか。 | Is overtime OK? | honest but flexible: 繁忙期は対応可能です |
| 何か質問はありますか。 | Do you have any questions? | ALWAYS ask one or two: team size, tools, training, on-call |

## 逆質問 (questions to ask them)
- 配属予定のチームの体制を教えていただけますか。(How is the team I'd join organized?)
- 入社後の研修制度はありますか。(Is there training after joining?)
- 資格取得の支援制度はありますか。(Do you support getting certifications?)
- 夜間や休日の対応はありますか。(Is there night/weekend on-call?)

## Manners (面接マナー)
- Arrive 5-10 minutes early; for online interviews, test the camera and connection.
- Knock 3 times, 「失礼いたします」, wait to be asked to sit (「どうぞおかけください」).
- At the end: 「本日はお忙しいところ、ありがとうございました」.
@@text
🎉 Ten minutes a day with your own trainer, and the words in Japanese tickets and emails start to look familiar within weeks. 毎日少しずつ、頑張りましょう!

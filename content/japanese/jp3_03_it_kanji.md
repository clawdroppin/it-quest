@@lesson jp3-it-kanji IT Kanji: Readings, Compounds and a Study System
@@text
Reading is where kanji pays off fastest at work: tickets, monitoring alerts, specs and emails reuse a few hundred IT compounds. You don't need to write them by hand (the IME converts from kana), but you must **recognise** them and know their **readings** to type them. This lesson explains how readings work in compounds, gives the high-frequency IT kanji, and builds a spaced-repetition schedule so they stick.

## Readings

- **音読み (on'yomi)**: the Chinese-derived reading, used mostly in **compounds** of two or more kanji: 障害 (しょうがい), 設定 (せってい), 確認 (かくにん).
- **訓読み (kun'yomi)**: the native Japanese reading, used for words on their own and with okurigana: 止める (とめる), 送る (おくる), 新しい (あたらしい).
- Sound changes in compounds: **rendaku** (連濁) voices the second part (手 + 紙 → てがみ). **Small っ** doubles a consonant: 設定 = せつ + てい → せってい, 発生 = はつ + せい → はっせい.

## High-frequency IT compounds

| Compound | Reading | Meaning |
|---|---|---|
| 障害 | しょうがい | failure, outage |
| 復旧 | ふっきゅう | recovery |
| 設定 | せってい | settings, configuration |
| 確認 | かくにん | check, confirmation |
| 対応 | たいおう | handling, response |
| 原因 | げんいん | cause |
| 影響 | えいきょう | impact |
| 手順 | てじゅん | procedure |
| 環境 | かんきょう | environment (本番環境 production, 検証環境 staging) |
| 運用 | うんよう | operations |
| 保守 | ほしゅ | maintenance |
| 要件 | ようけん | requirements |
| 仕様 | しよう | specification |
| 不具合 | ふぐあい | defect, bug |
| 権限 | けんげん | permission |

## A study system

Use **spaced repetition**: a card you answer correctly comes back after a longer interval, and one you miss comes back soon. The **Leitner system** uses boxes: box 1 is reviewed every day, box 2 every 2 days, box 3 every 4, box 4 every 8, box 5 every 16. Correct answers move a card up a box, and wrong answers send it back to box 1. Anki uses a refined version of the same idea.

@@quiz
Which reading is usually used in kanji compounds like 確認?
* 音読み (on'yomi)
- 訓読み (kun'yomi)
- Neither
- Romaji
? Kun'yomi is used for words on their own and verbs with okurigana.

@@quiz
How is 設定 read?
- せつてい
* せってい
- せいてい
- しゃてい
? せつ + てい becomes せってい.

@@quiz
What does 本番環境 mean?
* Production environment
- Staging environment
- Development PC
- Data centre
? 検証環境 is staging.

@@quiz
In a Leitner system, what happens to a card you get wrong?
* It goes back to box 1 and is reviewed soon
- It moves up a box
- It is deleted
- Nothing
? Mistakes come back quickly.

@@quiz
What does 影響 mean in an incident report?
- Cause
* Impact
- Recovery
- Procedure
? 原因 is the cause.

@@flashcards
音読み :: Chinese-derived reading, used in compounds
訓読み :: native reading, used for single words
連濁 :: second part becomes voiced (かみ → がみ)
障害 :: しょうがい, failure
復旧 :: ふっきゅう, recovery
設定 :: せってい, settings
確認 :: かくにん, check
対応 :: たいおう, handling
原因 :: げんいん, cause
影響 :: えいきょう, impact
手順 :: てじゅん, procedure
環境 :: かんきょう, environment
運用 :: うんよう, operations
保守 :: ほしゅ, maintenance
要件 :: ようけん, requirements
仕様 :: しよう, specification
不具合 :: ふぐあい, defect
権限 :: けんげん, permission
Leitner boxes :: 1, 2, 4, 8, 16 days

@@drill
Type the reading of 障害 in hiragana.
= しょうがい
? An outage or failure.

@@drill
Type the reading of 復旧 in hiragana.
= ふっきゅう
? ふく + きゅう with a small っ.

@@exercise Add furigana with a dictionary
Annotate Japanese text with readings using a compound dictionary and longest-match lookup: at each position, try the longest dictionary word first. Output the text in the 漢字（かな） style, and list compounds the dictionary doesn't know (runs of kanji left without a reading).
@@lang python
@@concepts strings, maps, loops
@@starter
DICT = {"本番環境": "ほんばんかんきょう", "環境": "かんきょう", "障害": "しょうがい", "発生": "はっせい", "原因": "げんいん",
        "調査": "ちょうさ", "復旧": "ふっきゅう", "設定": "せってい", "変更": "へんこう", "確認": "かくにん", "影響": "えいきょう",
        "範囲": "はんい", "本番": "ほんばん", "手順": "てじゅん"}
texts = ["本番環境で障害が発生しました。", "原因を調査し、設定変更で復旧しました。", "影響範囲を確認中です。", "負荷試験の手順を共有します。"]

def furigana(text):
    raise NotImplementedError
@@expected
本番環境（ほんばんかんきょう）で障害（しょうがい）が発生（はっせい）しました。
原因（げんいん）を調査（ちょうさ）し、設定（せってい）変更（へんこう）で復旧（ふっきゅう）しました。
影響（えいきょう）範囲（はんい）を確認（かくにん）中です。
  unknown: 中
負荷試験の手順（てじゅん）を共有します。
  unknown: 負荷試験、共有
@@requires
def furigana
DICT
max(
@@hint
Let `longest = max(map(len, DICT))`. At position i, try lengths from `longest` down to 1, and if `text[i:i+n]` is in DICT, emit `word（reading）` and skip n. Otherwise emit the character and move on. A kanji is a character between "一" and "鿿"; collect consecutive kanji that weren't matched as unknown.
@@solution
DICT = {"本番環境": "ほんばんかんきょう", "環境": "かんきょう", "障害": "しょうがい", "発生": "はっせい", "原因": "げんいん",
        "調査": "ちょうさ", "復旧": "ふっきゅう", "設定": "せってい", "変更": "へんこう", "確認": "かくにん", "影響": "えいきょう",
        "範囲": "はんい", "本番": "ほんばん", "手順": "てじゅん"}
texts = ["本番環境で障害が発生しました。", "原因を調査し、設定変更で復旧しました。", "影響範囲を確認中です。", "負荷試験の手順を共有します。"]
LONGEST = max(map(len, DICT))

def is_kanji(c):
    return "一" <= c <= "鿿"

def furigana(text):
    out, unknown, run, i = [], [], "", 0
    while i < len(text):
        for n in range(min(LONGEST, len(text) - i), 0, -1):
            word = text[i:i + n]
            if word in DICT:
                if run:
                    unknown.append(run); run = ""
                out.append(f"{word}（{DICT[word]}）")
                i += n
                break
        else:
            c = text[i]
            if is_kanji(c):
                run += c
            elif run:
                unknown.append(run); run = ""
            out.append(c)
            i += 1
    if run:
        unknown.append(run)
    return "".join(out), unknown

for t in texts:
    annotated, unknown = furigana(t)
    print(annotated)
    if unknown:
        print("  unknown:", "、".join(unknown))

@@exercise Schedule reviews with Leitner boxes
Simulate 14 days of reviews for a deck of IT kanji cards: each card is due when its box interval has passed since its last review, correct answers move it up a box (maximum 5) and wrong answers send it to box 1. The answers are given per card as a string of results for each review. Print each day's reviews and the final boxes.
@@lang python
@@concepts maps, loops, conditions
@@starter
INTERVAL = {1: 1, 2: 2, 3: 4, 4: 8, 5: 16}
# card: results of its successive reviews ("o" correct, "x" wrong)
answers = {"障害": "ooooo", "復旧": "oxooo", "影響": "xxooo", "権限": "ooxoo", "不具合": "oooxo"}
DAYS = 14

def due(card_state, day):
    raise NotImplementedError
@@expected
day  0: 障害○→2 復旧○→2 影響×→1 権限○→2 不具合○→2
day  1: 影響×→1
day  2: 障害○→3 復旧×→1 影響○→2 権限○→3 不具合○→3
day  3: 復旧○→2
day  4: 影響○→3
day  5: 復旧○→3
day  6: 障害○→4 権限×→1 不具合○→4
day  7: 権限○→2
day  8: 影響○→4
day  9: 復旧○→4 権限○→3
final boxes: 障害 4, 復旧 4, 影響 4, 権限 3, 不具合 4
@@requires
def due
INTERVAL[
DAYS
@@hint
Keep `state[card] = {"box": 1, "last": -1, "reviews": 0}` so every card is due on day 0. A card is due when `day - last >= INTERVAL[box]`. On review, read `answers[card][reviews]` (stop reviewing when the string runs out), move the box up (up to 5) or back to 1, and set `last = day`.
@@solution
INTERVAL = {1: 1, 2: 2, 3: 4, 4: 8, 5: 16}
# card: results of its successive reviews ("o" correct, "x" wrong)
answers = {"障害": "ooooo", "復旧": "oxooo", "影響": "xxooo", "権限": "ooxoo", "不具合": "oooxo"}
DAYS = 14

def due(card_state, day):
    return day - card_state["last"] >= INTERVAL[card_state["box"]]

state = {card: {"box": 1, "last": -1, "reviews": 0} for card in answers}
for day in range(DAYS):
    reviewed = []
    for card, s in state.items():
        if s["reviews"] >= len(answers[card]) or not due(s, day):
            continue
        result = answers[card][s["reviews"]]
        s["box"] = min(5, s["box"] + 1) if result == "o" else 1
        s["last"], s["reviews"] = day, s["reviews"] + 1
        reviewed.append(f"{card}{'○' if result == 'o' else '×'}→{s['box']}")
    if reviewed:
        print(f"day {day:>2}: {' '.join(reviewed)}")
print("final boxes:", ", ".join(f"{c} {s['box']}" for c, s in state.items()))

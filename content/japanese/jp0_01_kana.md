@@lesson jp0-kana Hiragana, Katakana and How Japanese Works
@@text
Japanese for IT work is very learnable: grammar is regular, pronunciation is simple, and **IT vocabulary is full of English loanwords** (サーバー, ネットワーク, パスワード). This track focuses on what you need at work and in job hunting, and pairs with the 🗾 trainer project.

## Three scripts, used together

| Script | Used for | Example |
|---|---|---|
| **ひらがな** hiragana | grammar endings, small words, native words | です, を, ありがとう |
| **カタカナ** katakana | **foreign loanwords**, names, emphasis | サーバー, クラウド, ジョン |
| **漢字** kanji | meaning-carrying words | 障害 (incident), 設定 (settings) |

A work sentence mixes all three: **サーバー**の**設定**を**変更**しました (I changed the server's settings).

## The sound system

Five vowels, always pronounced the same: **a** (ah), **i** (ee), **u** (oo, lips relaxed), **e** (eh), **o** (oh). Every kana is a vowel or consonant + vowel, plus **ん** (n).

| | a | i | u | e | o |
|---|---|---|---|---|---|
| – | あ ア | い イ | う ウ | え エ | お オ |
| k | か カ | き キ | く ク | け ケ | こ コ |
| s | さ サ | し シ (shi) | す ス | せ セ | そ ソ |
| t | た タ | ち チ (chi) | つ ツ (tsu) | て テ | と ト |
| n | な ナ | に ニ | ぬ ヌ | ね ネ | の ノ |
| h | は ハ | ひ ヒ | ふ フ (fu) | へ ヘ | ほ ホ |
| m | ま マ | み ミ | む ム | め メ | も モ |
| y | や ヤ | | ゆ ユ | | よ ヨ |
| r | ら ラ | り リ | る ル | れ レ | ろ ロ |
| w | わ ワ | | | | を ヲ (o) |
| n | ん ン | | | | |

- **Dakuten** ゛ voices a sound: か→が (ga), さ→ざ (za), た→だ (da), は→ば (ba); **handakuten** ゜: は→ぱ (pa).
- **Small ゃゅょ** combine: き+ゃ = きゃ (kya), し+ょ = しょ (sho), ち+ゅ = ちゅ (chu).
- **Small っ** doubles the next consonant (a short pause): きって (kitte), ネット (netto).
- **Long vowels**: in hiragana by adding a vowel (おう = ō: しょうがい shōgai); in katakana with **ー**: サーバー (sābā).
- Particles are written は (pronounced **wa**), へ (**e**), を (**o**).

## Katakana IT words: guess them!

| Katakana | Reading | English |
|---|---|---|
| サーバー | sābā | server |
| ネットワーク | nettowāku | network |
| パスワード | pasuwādo | password |
| ログイン | roguin | log in |
| ファイル | fairu | file |
| データベース | dētabēsu | database |
| セキュリティ | sekyuriti | security |
| アップデート | appudēto | update |
| トラブル | toraburu | trouble, a problem |
| パソコン | pasokon | PC (from "personal computer") |

Some are shortened Japanese creations: パソコン (PC), スマホ (smartphone), リモコン (remote control), アプリ (app).

**How to learn kana**: one week for hiragana, one week for katakana. Write each one 10 times, use an app (Anki, Tofugu's guide), and read katakana IT words everywhere. Then kanji gradually, *in words* (障害, 設定), not one by one.

## JLPT

The **Japanese-Language Proficiency Test** has levels **N5** (beginner) to **N1**. For IT jobs in Japan: **N2** is the common requirement for Japanese-speaking roles; **N3** is sometimes enough at companies with international teams; English-only roles exist (外資系 foreign companies, startups, some web companies), but Japanese widens your options enormously. The test is held in July and December, in Japan and abroad.

@@quiz
Which script is used for foreign loanwords like "server"?
* Katakana
- Hiragana
- Kanji
- Romaji
? サーバー.

@@quiz
How is the particle は pronounced?
* wa
- ha
- ba
- he
? Only as a particle; in words it's ha (はい = hai).

@@quiz
What does the small っ in ネット do?
* Doubles the next consonant: netto
- Makes a long vowel
- Adds a "tsu" sound: netsuto
- Nothing
? It's a tiny pause before the consonant.

@@quiz
What's the usual Japanese level requirement for Japanese-speaking IT roles?
* JLPT N2
- JLPT N5
- No Japanese at all
- JLPT N1 always
? N1 is great but rarely strictly required; English-only roles exist too.

@@quiz
What does パソコン mean?
* A personal computer (PC)
- A password
- A pass code
- A person
? A Japanese abbreviation of パーソナルコンピューター.

@@flashcards
ひらがな :: Hiragana: grammar and native words
カタカナ :: Katakana: loanwords and names
漢字 :: Kanji: meaning-carrying characters
し / ち / つ / ふ :: shi / chi / tsu / fu
ん :: n
゛ (dakuten) :: Voices: か→が, さ→ざ, た→だ, は→ば
゜ (handakuten) :: は→ぱ (p)
きゃ / しょ / ちゅ :: kya / sho / chu
っ (small tsu) :: Doubles the next consonant
ー :: Long vowel in katakana
サーバー :: server
ログイン :: log in
トラブル :: trouble, problem
アプリ :: app
JLPT N5 → N1 :: Beginner → advanced

@@drill
Write ネットワーク in romaji.
= nettowaaku
= nettowāku
= nettowaku
? The っ doubles the t; ー lengthens the a.

@@drill
Read this IT word and write its English meaning: パスワード
= password
? pa-su-wā-do.

@@drill
Write しょうがい (incident) in romaji.
= shougai
= shōgai
= shogai
? しょ = sho, う lengthens the o.

@@exercise Convert between hiragana and katakana
In Unicode, each katakana character sits exactly 0x60 code points after its hiragana partner (あ U+3042, ア U+30A2). Write converters in both directions that leave other characters (kanji, ASCII, the long-vowel mark ー) unchanged, and use them to write IT loanwords in katakana and check whether a word is "all katakana".
@@lang python
@@starter
words = ["さーばー", "ねっとわーく", "ぱすわーど", "ろぐいん", "クラウド", "データベース"]

def to_katakana(text):
    raise NotImplementedError

def to_hiragana(text):
    raise NotImplementedError
@@expected
さーばー     katakana サーバー     hiragana さーばー     all katakana? False
ねっとわーく   katakana ネットワーク   hiragana ねっとわーく   all katakana? False
ぱすわーど    katakana パスワード    hiragana ぱすわーど    all katakana? False
ろぐいん     katakana ログイン     hiragana ろぐいん     all katakana? False
クラウド     katakana クラウド     hiragana くらうど     all katakana? True
データベース   katakana データベース   hiragana でーたべーす   all katakana? True
あ U+3042, ア U+30A2, difference 0x60
@@requires
def to_katakana
0x60
ord(
chr(
@@hint
Hiragana runs from U+3041 to U+3096, and katakana from U+30A1 to U+30F6. For each character, if it's in the source range add (or subtract) 0x60 with `chr(ord(c) + 0x60)`, otherwise keep it. "All katakana" means every character is in the katakana range or is ー.
@@solution
words = ["さーばー", "ねっとわーく", "ぱすわーど", "ろぐいん", "クラウド", "データベース"]

def to_katakana(text):
    return "".join(chr(ord(c) + 0x60) if "ぁ" <= c <= "ゖ" else c for c in text)

def to_hiragana(text):
    return "".join(chr(ord(c) - 0x60) if "ァ" <= c <= "ヶ" else c for c in text)

def all_katakana(text):
    return all("ァ" <= c <= "ヶ" or c == "ー" for c in text)

for w in words:
    k, h = to_katakana(w), to_hiragana(w)
    print(f"{w:<8} katakana {k:<8} hiragana {h:<8} all katakana? {all_katakana(w)}")
print(f"あ U+{ord('あ'):04X}, ア U+{ord('ア'):04X}, difference 0x{ord('ア') - ord('あ'):X}")

@@exercise Read katakana loanwords
Katakana IT words are English in disguise once you can read the special marks: **ー** lengthens the vowel before it, a small **ッ** doubles the next consonant, and a small **ャュョ/ィェ** replaces the `i` (or `u`) of the kana before it (キ + ャ = kya, テ + ィ = ti, チ + ェ = che). Write `romanize(word)` that reads katakana into romaji (long vowels doubled, like `saabaa`), using the `KANA` table, then `respell(english)` that spells an English word the way katakana would hear it, using the `RESPELL` pairs in order (`er` → `aa`, `l` → `r`, `v` → `b` …), and `guess(word)` that picks the candidate whose respelling is closest to the romaji by `difflib.SequenceMatcher` ratio. Print each word, its reading and the guess.
@@lang python
@@starter
import difflib

KANA = {
    "ア": "a", "イ": "i", "ウ": "u", "エ": "e", "オ": "o", "カ": "ka", "キ": "ki", "ク": "ku", "ケ": "ke", "コ": "ko",
    "サ": "sa", "シ": "shi", "ス": "su", "セ": "se", "ソ": "so", "タ": "ta", "チ": "chi", "ツ": "tsu", "テ": "te",
    "ト": "to", "ナ": "na", "ニ": "ni", "ヌ": "nu", "ネ": "ne", "ノ": "no", "ハ": "ha", "ヒ": "hi", "フ": "fu",
    "ヘ": "he", "ホ": "ho", "マ": "ma", "ミ": "mi", "ム": "mu", "メ": "me", "モ": "mo", "ヤ": "ya", "ユ": "yu",
    "ヨ": "yo", "ラ": "ra", "リ": "ri", "ル": "ru", "レ": "re", "ロ": "ro", "ワ": "wa", "ン": "n", "ガ": "ga",
    "ギ": "gi", "グ": "gu", "ゲ": "ge", "ゴ": "go", "ザ": "za", "ジ": "ji", "ズ": "zu", "ゼ": "ze", "ゾ": "zo",
    "ダ": "da", "デ": "de", "ド": "do", "バ": "ba", "ビ": "bi", "ブ": "bu", "ベ": "be", "ボ": "bo", "パ": "pa",
    "ピ": "pi", "プ": "pu", "ペ": "pe", "ポ": "po",
}
SMALL = {"ャ": "ya", "ュ": "yu", "ョ": "yo", "ィ": "i", "ェ": "e", "ァ": "a", "ォ": "o"}
RESPELL = [("er", "aa"), ("or", "oo"), ("ck", "kk"), ("c", "k"), ("v", "b"), ("l", "r"), ("ey", "ii"), ("ee", "ii")]
CANDIDATES = ["server", "network", "password", "login", "file", "database", "security", "update", "cloud",
              "keyboard", "router", "printer", "backup", "mail", "ticket", "check", "chat"]
words = ["サーバー", "ネットワーク", "パスワード", "ログイン", "クラウド", "キーボード", "バックアップ", "チケット",
         "チェック", "セキュリティ", "プリンター", "チャット", "ファイル"]

def romanize(word):
    raise NotImplementedError


def respell(english):
    raise NotImplementedError


def guess(word):
    raise NotImplementedError


for w in words:
    print(f"{w}: {romanize(w)} -> {guess(w)}")
@@expected
サーバー: saabaa -> server
ネットワーク: nettowaaku -> network
パスワード: pasuwaado -> password
ログイン: roguin -> login
クラウド: kuraudo -> cloud
キーボード: kiiboodo -> keyboard
バックアップ: bakkuappu -> backup
チケット: chiketto -> ticket
チェック: chekku -> check
セキュリティ: sekyuriti -> security
プリンター: purintaa -> printer
チャット: chatto -> chat
ファイル: fairu -> file
@@requires
def romanize
def respell
def guess
SequenceMatcher
@@hint
Walk the word one character at a time. `ー` repeats the last letter, `ッ` sets a flag that doubles the next consonant, and a small kana replaces the last vowel of what you've written so far. After `sh`, `ch` or `j`, drop the `y` (シャ is sha, not shya). Compare the reading with the *respelled* candidates: "server" and "saabaa" look nothing alike until `er` becomes `aa` and `v` becomes `b`.
@@solution
import difflib

KANA = {
    "ア": "a", "イ": "i", "ウ": "u", "エ": "e", "オ": "o", "カ": "ka", "キ": "ki", "ク": "ku", "ケ": "ke", "コ": "ko",
    "サ": "sa", "シ": "shi", "ス": "su", "セ": "se", "ソ": "so", "タ": "ta", "チ": "chi", "ツ": "tsu", "テ": "te",
    "ト": "to", "ナ": "na", "ニ": "ni", "ヌ": "nu", "ネ": "ne", "ノ": "no", "ハ": "ha", "ヒ": "hi", "フ": "fu",
    "ヘ": "he", "ホ": "ho", "マ": "ma", "ミ": "mi", "ム": "mu", "メ": "me", "モ": "mo", "ヤ": "ya", "ユ": "yu",
    "ヨ": "yo", "ラ": "ra", "リ": "ri", "ル": "ru", "レ": "re", "ロ": "ro", "ワ": "wa", "ン": "n", "ガ": "ga",
    "ギ": "gi", "グ": "gu", "ゲ": "ge", "ゴ": "go", "ザ": "za", "ジ": "ji", "ズ": "zu", "ゼ": "ze", "ゾ": "zo",
    "ダ": "da", "デ": "de", "ド": "do", "バ": "ba", "ビ": "bi", "ブ": "bu", "ベ": "be", "ボ": "bo", "パ": "pa",
    "ピ": "pi", "プ": "pu", "ペ": "pe", "ポ": "po",
}
SMALL = {"ャ": "ya", "ュ": "yu", "ョ": "yo", "ィ": "i", "ェ": "e", "ァ": "a", "ォ": "o"}
RESPELL = [("er", "aa"), ("or", "oo"), ("ck", "kk"), ("c", "k"), ("v", "b"), ("l", "r"), ("ey", "ii"), ("ee", "ii")]
CANDIDATES = ["server", "network", "password", "login", "file", "database", "security", "update", "cloud",
              "keyboard", "router", "printer", "backup", "mail", "ticket", "check", "chat"]
words = ["サーバー", "ネットワーク", "パスワード", "ログイン", "クラウド", "キーボード", "バックアップ", "チケット",
         "チェック", "セキュリティ", "プリンター", "チャット", "ファイル"]

def romanize(word):
    out, double = "", False
    for ch in word:
        if ch == "ー":
            out += out[-1]
        elif ch == "ッ":
            double = True
        elif ch in SMALL:
            small = SMALL[ch]
            if out.endswith(("sh", "ch", "j"), 0, len(out) - 1) and small.startswith("y"):
                small = small[1:]
            out = out[:-1] + small
        else:
            r = KANA[ch]
            if double:
                r = ("t" if r.startswith("ch") else r[0]) + r
                double = False
            out += r
    return out

def respell(english):
    for a, b in RESPELL:
        english = english.replace(a, b)
    return english

def guess(word):
    reading = romanize(word)
    return max(CANDIDATES, key=lambda c: difflib.SequenceMatcher(None, reading, respell(c)).ratio())

for w in words:
    print(f"{w}: {romanize(w)} -> {guess(w)}")

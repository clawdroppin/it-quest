@@lesson jp0-kana-systems Kana in Depth: Romaji, Small Kana, Long Sounds and Text on Computers
@@text
Once you can read the basic kana, the details trip you up: small ゃゅょ, the small っ that doubles a consonant, long vowels written differently in hiragana and katakana, and the many romanisation systems. As an engineer you'll also meet kana as **data**: Unicode ranges, full-width and half-width forms, and input methods. This lesson covers both, and the exercises build a romaji converter and kana utilities.

## Combined sounds (拗音)

An i-column kana plus a small ゃ, ゅ or ょ makes one syllable: き + ゃ = きゃ (kya), し + ょ = しょ (sho), ち + ゅ = ちゅ (chu), じ + ゃ = じゃ (ja). Katakana adds more combinations for foreign sounds: ティ (ti), ファ (fa), ウィ (wi), ヴ (vu), ジェ (je).

## Small っ (促音)

A small っ/ッ doubles the next consonant: きって (kitte, stamp), ネット (netto), ちょっと (chotto). Before ch, Hepburn writes **tch**: まっちゃ = matcha.

## Long vowels (長音)

- Katakana uses **ー**: サーバー (sābā / saabaa, server), メール (mēru, email), コンピューター.
- Hiragana doubles the vowel: おかあさん, おにいさん. **o + u** is usually a long o: とうきょう (Tōkyō), ありがとう. **e + i** is often read as a long e: せんせい (sensei).
- IT terms dropped some final ー in the past (コンピュータ vs コンピューター). Microsoft and many companies switched to the ー form around 2008; both appear in documents.

## Romanisation

- **Hepburn** (ヘボン式): closest to English pronunciation (shi, chi, tsu, fu, ji); used in passports and signs.
- **Kunrei** (訓令式): systematic (si, ti, tu, hu, zi); taught in Japanese schools.
- **Typing (IME) romaji**: whatever the input method accepts: si and shi both work; ん is typed `nn` or `n'` before a vowel; small kana with x or l prefixes (`xtu`, `ltu`, `xya`).
- ん before a vowel or y is written n' in Hepburn to avoid ambiguity: きんえん (kin'en, no smoking) vs きねん (kinen, anniversary).

## Kana as data

- Unicode: hiragana **U+3041–U+3096**, katakana **U+30A1–U+30F6**; each katakana is the hiragana code point **+ 0x60**. The long vowel mark ー is U+30FC.
- **Half-width katakana** (ｶﾀｶﾅ, U+FF66–U+FF9F) appear in legacy systems, bank transfers (振込) and some forms. **Full-width** Latin letters and digits (ＡＢＣ１２３) are common in user input. Normalise with Unicode **NFKC** before comparing or validating (Python: `unicodedata.normalize("NFKC", s)`).
- Sorting Japanese names needs the reading (ふりがな), not the kanji; forms often ask for フリガナ in katakana.
- Legacy encodings: **Shift_JIS** (CP932 on Windows) and EUC-JP; garbled text (文字化け) usually means the wrong one was assumed. Use UTF-8.

@@quiz
How is まっちゃ written in Hepburn?
- macha
* matcha
- mattya
- maccha
? Before ch, small っ becomes t.

@@quiz
How do you write a long vowel in katakana?
- Double the vowel kana
* With the long vowel mark ー
- With a small っ
- It isn't written
? サーバー.

@@quiz
Which romanisation is used in passports?
* Hepburn
- Kunrei
- Nihon-shiki
- IME romaji
? Closest to English pronunciation.

@@quiz
What's the relationship between hiragana and katakana code points?
- They're the same
* Katakana = hiragana + 0x60
- Katakana = hiragana − 0x60
- No relationship
? あ U+3042, ア U+30A2.

@@quiz
A user typed ＡＢＣ１２３ (full-width). What should you do before validation?
- Reject it
* Normalise it with NFKC
- Convert to Shift_JIS
- Lowercase it
? NFKC maps full-width to ASCII.

@@flashcards
拗音 :: combined sounds with small ゃゅょ
促音 :: small っ, doubled consonant
長音 :: long vowel
ー :: katakana long vowel mark
Hepburn :: shi, chi, tsu, fu, ji
Kunrei :: si, ti, tu, hu, zi
n' :: ん before a vowel or y
IME small kana :: x or l prefix
hiragana range :: U+3041–U+3096
katakana offset :: +0x60
half-width katakana :: ｶﾀｶﾅ in legacy systems
NFKC :: normalise width and compatibility forms
フリガナ :: katakana reading on forms
文字化け :: garbled text from wrong encoding
Shift_JIS / CP932 :: legacy Japanese encoding

@@drill
Write サーバー in Hepburn romaji without macrons.
= saabaa
= sabaa
= saba
= sābā
? Long vowels are often doubled when macrons aren't available.

@@drill
What does 文字化け mean?
= garbled text
= mojibake
= garbled characters
= corrupted text
? Usually a wrong encoding.

@@exercise Hiragana to Hepburn romaji
Convert hiragana to Hepburn romaji: single kana, combined sounds with small ゃゅょ, small っ (doubling the next consonant, `t` before `ch`), and ん (written `n'` before a vowel or y). Leave long vowels as written (とうきょう → toukyou) and katakana's ー as a doubled vowel after converting katakana to hiragana first.
@@lang python
@@starter
BASE = dict(zip(
    "あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへほまみむめもやゆよらりるれろわをがぎぐげござじずぜぞだぢづでどばびぶべぼぱぴぷぺぽ",
    "a i u e o ka ki ku ke ko sa shi su se so ta chi tsu te to na ni nu ne no ha hi fu he ho ma mi mu me mo ya yu yo ra ri ru re ro wa o ga gi gu ge go za ji zu ze zo da ji zu de do ba bi bu be bo pa pi pu pe po".split()))
SMALL = {"ゃ": "ya", "ゅ": "yu", "ょ": "yo"}
words = ["さーばー", "とうきょう", "きって", "まっちゃ", "しゃいん", "きんえん", "きねん", "ちょっと", "じゅうしょ", "でんわ", "ネットワーク", "メール"]

def to_hiragana(text):
    raise NotImplementedError
@@expected
さーばー -> saabaa
とうきょう -> toukyou
きって -> kitte
まっちゃ -> matcha
しゃいん -> shain
きんえん -> kin'en
きねん -> kinen
ちょっと -> chotto
じゅうしょ -> juusho
でんわ -> denwa
ネットワーク -> nettowaaku
メール -> meeru
@@requires
def to_hiragana
def romaji
0x60
@@hint
`to_hiragana`: subtract 0x60 from characters between ァ and ヶ. Walk with an index. For a kana followed by a small ゃゅょ: drop the base's final `i` and add the small sound; but shi/chi/ji keep only their consonant (sha, cha, ja). For っ, remember to double the first letter of the next syllable (`t` if it starts with `ch`). For ー, repeat the previous vowel. ん becomes `n'` if the next syllable starts with a vowel or `y`.
@@solution
BASE = dict(zip(
    "あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへほまみむめもやゆよらりるれろわをがぎぐげござじずぜぞだぢづでどばびぶべぼぱぴぷぺぽ",
    "a i u e o ka ki ku ke ko sa shi su se so ta chi tsu te to na ni nu ne no ha hi fu he ho ma mi mu me mo ya yu yo ra ri ru re ro wa o ga gi gu ge go za ji zu ze zo da ji zu de do ba bi bu be bo pa pi pu pe po".split()))
SMALL = {"ゃ": "ya", "ゅ": "yu", "ょ": "yo"}
words = ["さーばー", "とうきょう", "きって", "まっちゃ", "しゃいん", "きんえん", "きねん", "ちょっと", "じゅうしょ", "でんわ", "ネットワーク", "メール"]

def to_hiragana(text):
    return "".join(chr(ord(c) - 0x60) if "ァ" <= c <= "ヶ" else c for c in text)

def syllables(text):
    out, i = [], 0
    while i < len(text):
        c = text[i]
        if c in BASE and i + 1 < len(text) and text[i + 1] in SMALL:
            base = BASE[c]
            small = SMALL[text[i + 1]]
            out.append(base[:-1] + small[1:] if base in ("shi", "chi", "ji") else base[:-1] + small)
            i += 2
        else:
            out.append(c if c in "っんー" else BASE.get(c, c))
            i += 1
    return out

def romaji(text):
    parts = syllables(to_hiragana(text))
    result = ""
    for i, s in enumerate(parts):
        nxt = parts[i + 1] if i + 1 < len(parts) else ""
        if s == "っ":
            result += "t" if nxt.startswith("ch") else nxt[:1]
        elif s == "ん":
            result += "n'" if nxt[:1] in ("a", "i", "u", "e", "o", "y") else "n"
        elif s == "ー":
            result += result[-1]
        else:
            result += s
    return result

for w in words:
    print(f"{w} -> {romaji(w)}")

@@exercise Normalise Japanese form input
Clean values a user typed into a registration form: normalise full-width letters and digits and half-width katakana with NFKC, convert the フリガナ field to katakana (hiragana + 0x60), check that it contains only katakana, spaces and ー, and validate a postal code (〒123-4567, with or without the hyphen) and a phone number.
@@lang python
@@starter
import re
import unicodedata

forms = [
    {"name": "佐藤　愛子", "furigana": "さとう　あいこ", "postal": "〒１００－０００５", "phone": "０３－１２３４－５６７８", "email": "ＡＩＫＯ＠example.jp"},
    {"name": "Ben Smith", "furigana": "ﾍﾞﾝ ｽﾐｽ", "postal": "5300001", "phone": "090-1234-5678", "email": "ben@example.com"},
    {"name": "田中 健", "furigana": "田中 けん", "postal": "123-45", "phone": "1234", "email": "ken@example"},
]

def normalise(text):
    raise NotImplementedError
@@expected
佐藤 愛子 / サトウ アイコ / 〒100-0005 / 03-1234-5678 / aiko@example.jp
   OK
Ben Smith / ベン スミス / 〒530-0001 / 090-1234-5678 / ben@example.com
   OK
田中 健 / 田中 ケン / 〒123-45 / 1234 / ken@example
   フリガナ must be kana only; 郵便番号 must be 7 digits; 電話番号 is not a Japanese number; メールアドレス is invalid
@@requires
def normalise
unicodedata.normalize("NFKC"
0x60
@@hint
NFKC turns full-width ＡＢＣ and １２３ into ASCII, the ideographic space into a normal space, and ﾍﾞﾝ into ベン. It doesn't map the full-width hyphen (－) to `-`, so also replace "－" and "ー" in number fields. Katakana check: `re.fullmatch(r"[ァ-ヶー ]+", s)`. Postal code: strip 〒, then `\d{3}-?\d{4}`. Phone: 10–11 digits once hyphens are removed, starting with 0.
@@solution
import re
import unicodedata

forms = [
    {"name": "佐藤　愛子", "furigana": "さとう　あいこ", "postal": "〒１００－０００５", "phone": "０３－１２３４－５６７８", "email": "ＡＩＫＯ＠example.jp"},
    {"name": "Ben Smith", "furigana": "ﾍﾞﾝ ｽﾐｽ", "postal": "5300001", "phone": "090-1234-5678", "email": "ben@example.com"},
    {"name": "田中 健", "furigana": "田中 けん", "postal": "123-45", "phone": "1234", "email": "ken@example"},
]

def normalise(text):
    return unicodedata.normalize("NFKC", text).strip()

def to_katakana(text):
    return "".join(chr(ord(c) + 0x60) if "ぁ" <= c <= "ゖ" else c for c in text)

def digits_field(text):
    return normalise(text).replace("〒", "").replace("－", "-").replace("ー", "-").replace("−", "-")

for form in forms:
    errors = []
    name = normalise(form["name"])
    kana = to_katakana(normalise(form["furigana"]))
    if not re.fullmatch(r"[ァ-ヶー ]+", kana):
        errors.append("フリガナ must be kana only")
    postal = digits_field(form["postal"])
    m = re.fullmatch(r"(\d{3})-?(\d{4})", postal)
    postal = f"{m.group(1)}-{m.group(2)}" if m else postal
    if not m:
        errors.append("郵便番号 must be 7 digits")
    phone = digits_field(form["phone"])
    if not re.fullmatch(r"0\d{9,10}", phone.replace("-", "")):
        errors.append("電話番号 is not a Japanese number")
    email = normalise(form["email"]).lower()
    if not re.fullmatch(r"[^@\s]+@[^@\s]+\.[a-z]{2,}", email):
        errors.append("メールアドレス is invalid")
    print(f"{name} / {kana} / 〒{postal} / {phone} / {email}")
    print("   " + ("OK" if not errors else "; ".join(errors)))

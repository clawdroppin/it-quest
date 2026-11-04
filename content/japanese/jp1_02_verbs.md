@@lesson jp1-verbs Verbs: Groups, ます, て, ない and た Forms
@@text
Almost every workplace sentence ends in a verb: 確認します (I'll check), 送ってください (please send it), 使えません (it doesn't work), 再起動しました (I restarted it). Japanese verbs are very regular: learn which of three groups a verb is in, and four or five rules give you every form you need for polite work conversation. This lesson teaches those rules and has you implement them, which is the fastest way to make them stick.

## The three groups

| Group | Also called | How to recognise | Examples |
|---|---|---|---|
| **1** | 五段動詞 (godan, u-verbs) | ends in an u-sound: う く ぐ す つ ぬ ぶ む る | 書く (write), 話す (speak), 待つ (wait), 使う (use), 送る (send) |
| **2** | 一段動詞 (ichidan, ru-verbs) | ends in **-iru** or **-eru** (most of them) | 見る (see), 食べる (eat), 調べる (look up), 起きる (get up) |
| **3** | irregular | only two | する (do), 来る (くる, come) |

Trap: some **-iru/-eru** verbs are group 1: 帰る (go home), 入る (enter), 知る (know), 走る (run), 切る (cut). Dictionaries mark the group; IT verbs made from nouns + する (確認する, 設定する, 再起動する) are all group 3.

## ます form (polite present/future)

- Group 1: change the final u-sound to its **i-sound**, add ます: 書く → 書**き**ます, 待つ → 待**ち**ます, 使う → 使**い**ます, 送る → 送**り**ます.
- Group 2: drop る, add ます: 調べる → 調べます.
- Group 3: する → します, 来る → 来ます (きます).
- Past: ました; negative: ません; negative past: ませんでした.

## て form (requests, "and then", ongoing actions)

| Group 1 ending | て form | Example |
|---|---|---|
| う つ る | って | 使って, 待って, 送って |
| む ぶ ぬ | んで | 読んで, 呼んで, 死んで |
| く | いて | 書いて (**exception: 行く → 行って**) |
| ぐ | いで | 急いで |
| す | して | 話して |

Group 2: drop る, add て (調べて). Group 3: して, 来て (きて). The た form (plain past) follows exactly the same rules with た/だ: 送った, 読んだ, 書いた.

Uses at work: **〜てください** (please do), **〜ています** (is doing / state: エラーが出ています), **〜てもいいですか** (may I), **〜てから** (after doing).

## ない form (plain negative)

- Group 1: final u-sound → **a-sound** + ない: 書かない, 待たない, 送らない; **う → わ**: 使わない. Exception: ある → ない.
- Group 2: drop る + ない: 調べない. Group 3: しない, 来ない (こない).
- Uses: **〜ないでください** (please don't), **〜なければなりません** (must).

## Potential form (can do)

Group 1: u → **e** + る (書ける, 使える, 送れる); Group 2: る → **られる** (見られる; casual speech often drops ら); する → **できる**; 来る → 来られる. Very common in support: ログインできません (can't log in), 印刷できません, 接続できません.

@@quiz
Which group is 帰る (to go home)?
* Group 1 (godan)
- Group 2 (ichidan)
- Group 3 (irregular)
- It has no group
? An exception that looks like -eru.

@@quiz
What is the ます form of 待つ?
- 待ります
* 待ちます
- 待ます
- 待ついます
? つ → ち.

@@quiz
What is the て form of 書く?
- 書って
* 書いて
- 書きて
- 書んで
? く → いて.

@@quiz
What is the て form of 行く?
- 行いて
* 行って
- 行きて
- 行んで
? The one exception to the く rule.

@@quiz
What is the ない form of 使う?
- 使あない
* 使わない
- 使ない
- 使いない
? う becomes わ.

@@quiz
A user says ログインできません. What do they mean?
- They don't want to log in
* They can't log in
- They logged out
- They will log in later
? できる is the potential of する.

@@flashcards
group 1 :: 五段, u-verbs
group 2 :: 一段, ru-verbs (-iru/-eru)
group 3 :: する and 来る
帰る group :: 1 (exception)
ます form g1 :: u → i + ます
ます form g2 :: drop る + ます
う つ る て :: って
む ぶ ぬ て :: んで
く て :: いて
ぐ て :: いで
す て :: して
行く て :: 行って
ない form g1 :: u → a + ない
う ない :: わない
来る ない :: こない
する potential :: できる
〜てください :: please do
〜ています :: is doing / state
〜ないでください :: please don't
〜なければなりません :: must

@@drill
What is the て form of 送る (to send)?
= 送って
= おくって
? る → って for group 1.

@@drill
What is the ます form of 確認する?
= 確認します
= かくにんします
? する → します.

@@exercise Conjugate verbs
Implement the conjugation rules. Given each verb's written form and group, produce the ます, て, た, ない and potential forms. Handle the exceptions 行く (て/た), ある (ない, and no potential form) and the two group 3 verbs, including する compounds such as 確認する.
@@lang python
@@starter
I_ROW = {"う": "い", "く": "き", "ぐ": "ぎ", "す": "し", "つ": "ち", "ぬ": "に", "ぶ": "び", "む": "み", "る": "り"}
A_ROW = {"う": "わ", "く": "か", "ぐ": "が", "す": "さ", "つ": "た", "ぬ": "な", "ぶ": "ば", "む": "ま", "る": "ら"}
E_ROW = {"う": "え", "く": "け", "ぐ": "げ", "す": "せ", "つ": "て", "ぬ": "ね", "ぶ": "べ", "む": "め", "る": "れ"}
verbs = [("書く", 1), ("使う", 1), ("待つ", 1), ("送る", 1), ("読む", 1), ("呼ぶ", 1), ("急ぐ", 1), ("話す", 1), ("行く", 1),
         ("帰る", 1), ("ある", 1), ("調べる", 2), ("見る", 2), ("する", 3), ("来る", 3), ("確認する", 3)]

def te_form(verb, group):
    raise NotImplementedError
@@expected
dictionary  ます        て          た          ない          potential
書く    (g1) 書きます     書いて      書いた      書かない       書ける
使う    (g1) 使います     使って      使った      使わない       使える
待つ    (g1) 待ちます     待って      待った      待たない       待てる
送る    (g1) 送ります     送って      送った      送らない       送れる
読む    (g1) 読みます     読んで      読んだ      読まない       読める
呼ぶ    (g1) 呼びます     呼んで      呼んだ      呼ばない       呼べる
急ぐ    (g1) 急ぎます     急いで      急いだ      急がない       急げる
話す    (g1) 話します     話して      話した      話さない       話せる
行く    (g1) 行きます     行って      行った      行かない       行ける
帰る    (g1) 帰ります     帰って      帰った      帰らない       帰れる
ある    (g1) あります     あって      あった      ない         (none: use あり得る)
調べる   (g2) 調べます     調べて      調べた      調べない       調べられる
見る    (g2) 見ます      見て       見た       見ない        見られる
する    (g3) します      して       した       しない        できる
来る    (g3) 来ます      来て       来た       来ない (こない)  来られる
確認する  (g3) 確認します    確認して     確認した     確認しない      確認できる
@@requires
def te_form
I_ROW[
A_ROW[
@@hint
For group 1 the stem is `verb[:-1]` and the ending `verb[-1]`. The た form is the て form with て→た and で→だ. For group 3, handle `verb.endswith("する")` (replace する) and 来る separately.
@@solution
I_ROW = {"う": "い", "く": "き", "ぐ": "ぎ", "す": "し", "つ": "ち", "ぬ": "に", "ぶ": "び", "む": "み", "る": "り"}
A_ROW = {"う": "わ", "く": "か", "ぐ": "が", "す": "さ", "つ": "た", "ぬ": "な", "ぶ": "ば", "む": "ま", "る": "ら"}
E_ROW = {"う": "え", "く": "け", "ぐ": "げ", "す": "せ", "つ": "て", "ぬ": "ね", "ぶ": "べ", "む": "め", "る": "れ"}
verbs = [("書く", 1), ("使う", 1), ("待つ", 1), ("送る", 1), ("読む", 1), ("呼ぶ", 1), ("急ぐ", 1), ("話す", 1), ("行く", 1),
         ("帰る", 1), ("ある", 1), ("調べる", 2), ("見る", 2), ("する", 3), ("来る", 3), ("確認する", 3)]

def te_form(verb, group):
    if group == 3:
        return verb[:-2] + "して" if verb.endswith("する") else "来て"
    if group == 2:
        return verb[:-1] + "て"
    if verb == "行く":
        return "行って"
    stem, end = verb[:-1], verb[-1]
    if end in "うつる":
        return stem + "って"
    if end in "むぶぬ":
        return stem + "んで"
    if end == "く":
        return stem + "いて"
    if end == "ぐ":
        return stem + "いで"
    return stem + "して"

def conjugate(verb, group):
    te = te_form(verb, group)
    ta = te[:-1] + ("た" if te.endswith("て") else "だ")
    if group == 3:
        if verb.endswith("する"):
            base = verb[:-2]
            return base + "します", te, ta, base + "しない", base + "できる"
        return "来ます", te, ta, "来ない (こない)", "来られる"
    if group == 2:
        stem = verb[:-1]
        return stem + "ます", te, ta, stem + "ない", stem + "られる"
    stem, end = verb[:-1], verb[-1]
    negative = "ない" if verb == "ある" else stem + A_ROW[end] + "ない"
    potential = "(none: use あり得る)" if verb == "ある" else stem + E_ROW[end] + "る"
    return stem + I_ROW[end] + "ます", te, ta, negative, potential

print("dictionary  ます        て          た          ない          potential")
for verb, group in verbs:
    masu, te, ta, nai, pot = conjugate(verb, group)
    print(f"{verb:<6}(g{group}) {masu:<8} {te:<8} {ta:<8} {nai:<10} {pot}")

@@exercise Build support sentences
Use the て form and ない form rules to build polite workplace sentences from templates: `request` → 〜てください, `progress` → 〜ています, `permission` → 〜てもいいですか, `prohibit` → 〜ないでください, `cannot` → potential negative 〜ません (できません for する verbs). Print each sentence with its English meaning.
@@lang python
@@starter
A_ROW = {"う": "わ", "く": "か", "ぐ": "が", "す": "さ", "つ": "た", "ぬ": "な", "ぶ": "ば", "む": "ま", "る": "ら"}
E_ROW = {"う": "え", "く": "け", "ぐ": "げ", "す": "せ", "つ": "て", "ぬ": "ね", "ぶ": "べ", "む": "め", "る": "れ"}
tasks = [
    ("request", "パスワードを", "変更する", 3, "Please change your password."),
    ("request", "画面を", "送る", 1, "Please send (me) the screen(shot)."),
    ("progress", "原因を", "調べる", 2, "We are investigating the cause."),
    ("permission", "PCを", "再起動する", 3, "May I restart the PC?"),
    ("prohibit", "電源を", "切る", 1, "Please don't turn off the power."),
    ("cannot", "プリンターが", "使う", 1, "The printer can't be used."),
    ("cannot", "VPNに", "接続する", 3, "(I) can't connect to the VPN."),
]

def te_form(verb, group):
    raise NotImplementedError
@@expected
パスワードを変更してください。  (Please change your password.)
画面を送ってください。  (Please send (me) the screen(shot).)
原因を調べています。  (We are investigating the cause.)
PCを再起動してもいいですか。  (May I restart the PC?)
電源を切らないでください。  (Please don't turn off the power.)
プリンターが使えません。  (The printer can't be used.)
VPNに接続できません。  ((I) can't connect to the VPN.)
@@requires
def te_form
def cannot
でください
@@hint
Reuse your て form logic. For `cannot`: する verbs become 〜できません; group 1 becomes the e-row + ません (使えません); group 2 becomes stem + られません.
@@solution
A_ROW = {"う": "わ", "く": "か", "ぐ": "が", "す": "さ", "つ": "た", "ぬ": "な", "ぶ": "ば", "む": "ま", "る": "ら"}
E_ROW = {"う": "え", "く": "け", "ぐ": "げ", "す": "せ", "つ": "て", "ぬ": "ね", "ぶ": "べ", "む": "め", "る": "れ"}
tasks = [
    ("request", "パスワードを", "変更する", 3, "Please change your password."),
    ("request", "画面を", "送る", 1, "Please send (me) the screen(shot)."),
    ("progress", "原因を", "調べる", 2, "We are investigating the cause."),
    ("permission", "PCを", "再起動する", 3, "May I restart the PC?"),
    ("prohibit", "電源を", "切る", 1, "Please don't turn off the power."),
    ("cannot", "プリンターが", "使う", 1, "The printer can't be used."),
    ("cannot", "VPNに", "接続する", 3, "(I) can't connect to the VPN."),
]

def te_form(verb, group):
    if group == 3:
        return verb[:-2] + "して" if verb.endswith("する") else "来て"
    if group == 2:
        return verb[:-1] + "て"
    stem, end = verb[:-1], verb[-1]
    if verb == "行く":
        return "行って"
    return stem + {"う": "って", "つ": "って", "る": "って", "む": "んで", "ぶ": "んで", "ぬ": "んで", "く": "いて", "ぐ": "いで", "す": "して"}[end]

def negative(verb, group):
    if group == 3:
        return verb[:-2] + "しない" if verb.endswith("する") else "来ない"
    if group == 2:
        return verb[:-1] + "ない"
    return verb[:-1] + A_ROW[verb[-1]] + "ない"

def cannot(verb, group):
    if group == 3 and verb.endswith("する"):
        return verb[:-2] + "できません"
    if group == 2:
        return verb[:-1] + "られません"
    return verb[:-1] + E_ROW[verb[-1]] + "ません"

for kind, obj, verb, group, english in tasks:
    if kind == "request":
        sentence = obj + te_form(verb, group) + "ください。"
    elif kind == "progress":
        sentence = obj + te_form(verb, group) + "います。"
    elif kind == "permission":
        sentence = obj + te_form(verb, group) + "もいいですか。"
    elif kind == "prohibit":
        sentence = obj + negative(verb, group) + "でください。"
    else:
        sentence = obj + cannot(verb, group) + "。"
    print(f"{sentence}  ({english})")

@@lesson jp1-particles Particles in Practice: は, が, を, に, で, へ, と, から, まで
@@text
Particles (助詞) are the small hiragana words that mark what each part of a sentence does. Japanese word order is flexible (the verb comes last, but the rest can move), so **particles carry the grammar**. Getting them right matters more than vocabulary for being understood at work. This lesson covers the core particles with IT examples.

## The core particles

| Particle | Marks | Example |
|---|---|---|
| **は** (wa) | the **topic**: "as for X" | サーバー**は**正常です。 As for the server, it's normal. |
| **が** | the **subject**, especially new information, and with ある/いる/できる/好き/分かる | エラー**が**発生しました。 An error occurred. |
| **を** (o) | the **direct object** | ログ**を**確認します。 I'll check the logs. |
| **に** | a point in **time**, a **destination**, the **target** of an action, or **existence** location | 10時**に**再起動します。 / 担当者**に**連絡します。 / 東京**に**データセンターがあります。 |
| **で** | where an **action** happens, or the **means** | 本番環境**で**テストしないでください。 / メール**で**送ります。 |
| **へ** (e) | **direction** (similar to に for movement) | 大阪**へ**出張します。 |
| **と** | "and" (complete lists), "with" | AWS**と**Azure / 田中さん**と**確認しました。 |
| **から / まで** | **from / until** (time and place) | 9時**から**17時**まで** |
| **の** | possession or linking nouns | 会社**の**PC, システム**の**障害 |
| **も** | "also", replacing は/が/を | こちら**も**確認済みです。 |

## The two most common confusions

- **は vs が**: は sets the topic (often known information, contrasts), and が points at the subject (new information, answers "which?"). 「どのサーバーが落ちましたか。」→「Web01**が**落ちました。」 To introduce an event, use が: 「障害**が**発生しました」.
- **に vs で for places**: に for **existence** (ある/いる) and **destination**, で for where an **action** happens. 「会議室**に**プロジェクターがあります」 vs 「会議室**で**会議をします」.

## Particles in IT writing

Status updates chain particles predictably: 「本日10時**から**、DBサーバー**の**メンテナンス**を**実施します。メンテナンス中**は**サービス**が**ご利用いただけません。」

@@quiz
Which particle fits? 「エラー＿発生しました。」 (an error occurred)
- を
* が
- で
- へ
? 発生する takes the subject with が.

@@quiz
Which particle fits? 「本番環境＿テストしないでください。」
- に
* で
- を
- が
? で marks where an action happens.

@@quiz
Which particle fits? 「会議室＿プロジェクターがあります。」
* に
- で
- を
- へ
? Existence (ある) takes に.

@@quiz
"Which server went down?" "Web01 went down." Which particle marks Web01 in the answer?
- は
* が
- を
- も
? が answers "which?".

@@quiz
How do you say "from 9 until 17 o'clock"?
* 9時から17時まで
- 9時まで17時から
- 9時に17時で
- 9時と17時
? から = from, まで = until.

@@quiz
Which particle marks the means, as in "send by email"?
- に
- を
* で
- の
? メールで送ります。

@@flashcards
は :: topic, "as for"
が :: subject, new information
を :: direct object
に (time) :: 10時に
に (destination/target) :: 担当者に連絡する
に (existence) :: 東京にあります
で (place of action) :: 本番環境で
で (means) :: メールで
へ :: direction
と :: and (complete list), with
から / まで :: from / until
の :: possession, linking nouns
も :: also
発生する :: takes が

@@drill
Fill in the particle (hiragana): ログ＿確認します。
= を
? Logs are the direct object.

@@drill
Fill in the particle (hiragana): 田中さん＿メールを送りました。 (to Tanaka)
= に
? The target of sending takes に.

@@exercise Choose particles from roles
Build sentences from (word, role) pairs by attaching the right particle for each role, then put the verb last. Roles: topic, subject, object, time, destination, place of action, existence place, means, from, until, with. Check sentences where a learner used the wrong particle for the role.
@@lang python
@@concepts maps, strings, conditions
@@starter
PARTICLE = {"topic": "は", "subject": "が", "object": "を", "time": "に", "destination": "に", "target": "に",
            "place_action": "で", "place_exist": "に", "means": "で", "from": "から", "until": "まで", "with": "と"}
sentences = [
    ([("本日", "topic"), ("10時", "time"), ("DBサーバー", "object")], "再起動します"),
    ([("障害", "subject")], "発生しました"),
    ([("検証環境", "place_action"), ("新しいバージョン", "object")], "テストします"),
    ([("報告書", "object"), ("メール", "means"), ("お客様", "target")], "送ります"),
    ([("9時", "from"), ("12時", "until"), ("田中さん", "with"), ("会議室", "place_action")], "打ち合わせをします"),
]
learner = [("会議室で", "place_exist"), ("メールに", "means"), ("ログを", "object"), ("東京に", "place_exist")]

def build(parts, verb):
    raise NotImplementedError
@@expected
本日は10時にDBサーバーを再起動します。
障害が発生しました。
検証環境で新しいバージョンをテストします。
報告書をメールでお客様に送ります。
9時から12時まで田中さんと会議室で打ち合わせをします。
-- learner check
fix  会議室で -> 会議室に (place_exist takes に)
fix  メールに -> メールで (means takes で)
ok   ログを (object)
ok   東京に (place_exist)
@@requires
def build
PARTICLE[
endswith
@@hint
Join `word + PARTICLE[role]` for each part, then add the verb and 「。」. For the learner check, the word must end with the particle for its role: if it doesn't, strip the last character and show the corrected form.
@@solution
PARTICLE = {"topic": "は", "subject": "が", "object": "を", "time": "に", "destination": "に", "target": "に",
            "place_action": "で", "place_exist": "に", "means": "で", "from": "から", "until": "まで", "with": "と"}
sentences = [
    ([("本日", "topic"), ("10時", "time"), ("DBサーバー", "object")], "再起動します"),
    ([("障害", "subject")], "発生しました"),
    ([("検証環境", "place_action"), ("新しいバージョン", "object")], "テストします"),
    ([("報告書", "object"), ("メール", "means"), ("お客様", "target")], "送ります"),
    ([("9時", "from"), ("12時", "until"), ("田中さん", "with"), ("会議室", "place_action")], "打ち合わせをします"),
]
learner = [("会議室で", "place_exist"), ("メールに", "means"), ("ログを", "object"), ("東京に", "place_exist")]

def build(parts, verb):
    return "".join(word + PARTICLE[role] for word, role in parts) + verb + "。"

for parts, verb in sentences:
    print(build(parts, verb))
print("-- learner check")
for phrase, role in learner:
    want = PARTICLE[role]
    if phrase.endswith(want):
        print(f"ok   {phrase} ({role})")
    else:
        print(f"fix  {phrase} -> {phrase[:-1]}{want} ({role} takes {want})")

@@exercise は or が?
Apply simple rules for は and が: question words (どれ, 誰, 何, どの…) and the answers to them take が; events with 発生する, ある, いる, できる, 分かる and 好き take が for their subject; a contrast or a known topic takes は. Classify the cases and print the sentence with the right particle.
@@lang python
@@starter
GA_PREDICATES = ("発生しました", "あります", "います", "できます", "分かります", "好きです")
QUESTION_WORDS = ("どれ", "誰", "何", "どの", "どこ")
cases = [
    ("どのサーバー", "落ちましたか", "question"),
    ("Web01", "落ちました", "answer"),
    ("エラー", "発生しました", "statement"),
    ("このシステム", "社内専用です", "topic"),
    ("Linux", "分かります", "statement"),
    ("昨日のバックアップ", "正常に完了しました", "topic"),
    ("Windowsサーバー", "問題ありませんが、Linuxサーバーは要確認です", "contrast"),
]

def particle(subject, predicate, kind):
    raise NotImplementedError
@@expected
どのサーバーが落ちましたか。  (question word)
Web01が落ちました。  (answers a which/who question)
エラーが発生しました。  (this predicate takes が)
このシステムは社内専用です。  (topic)
Linuxが分かります。  (this predicate takes が)
昨日のバックアップは正常に完了しました。  (topic)
Windowsサーバーは問題ありませんが、Linuxサーバーは要確認です。  (contrast)
@@requires
def particle
GA_PREDICATES
QUESTION_WORDS
@@hint
Return が if the subject starts with a question word, if the kind is "answer", or if the predicate starts with one of `GA_PREDICATES` (use `predicate.startswith(GA_PREDICATES)`). Otherwise return は. Print the reason alongside.
@@solution
GA_PREDICATES = ("発生しました", "あります", "います", "できます", "分かります", "好きです")
QUESTION_WORDS = ("どれ", "誰", "何", "どの", "どこ")
cases = [
    ("どのサーバー", "落ちましたか", "question"),
    ("Web01", "落ちました", "answer"),
    ("エラー", "発生しました", "statement"),
    ("このシステム", "社内専用です", "topic"),
    ("Linux", "分かります", "statement"),
    ("昨日のバックアップ", "正常に完了しました", "topic"),
    ("Windowsサーバー", "問題ありませんが、Linuxサーバーは要確認です", "contrast"),
]

def particle(subject, predicate, kind):
    if subject.startswith(QUESTION_WORDS):
        return "が", "question word"
    if kind == "answer":
        return "が", "answers a which/who question"
    if predicate.startswith(GA_PREDICATES):
        return "が", "this predicate takes が"
    if kind == "contrast":
        return "は", "contrast"
    return "は", "topic"

for subject, predicate, kind in cases:
    p, why = particle(subject, predicate, kind)
    print(f"{subject}{p}{predicate}。  ({why})")

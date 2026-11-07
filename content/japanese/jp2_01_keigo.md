@@lesson jp2-keigo 敬語 and Workplace Culture
@@text
**敬語** (keigo, respectful language) isn't optional at Japanese workplaces, but nobody expects perfection from a foreign colleague. Using the common set phrases correctly takes you a long way.

## Three kinds of keigo

| Type | Use | Example |
|---|---|---|
| **丁寧語** (teineigo, polite) | です/ます with everyone at work | 確認します |
| **尊敬語** (sonkeigo, respectful) | raising the *other person's* actions (customers, bosses) | いらっしゃいます (is/comes), ご覧になります (looks), おっしゃいます (says) |
| **謙譲語** (kenjōgo, humble) | lowering *your own* actions | 参ります (I go/come), 拝見します (I look), 申します (I say), いたします (I do) |

| Plain | 尊敬語 (their action) | 謙譲語 (my action) |
|---|---|---|
| する (do) | なさいます | いたします |
| 行く/来る (go/come) | いらっしゃいます | 参ります / 伺います |
| 言う (say) | おっしゃいます | 申します |
| 見る (see) | ご覧になります | 拝見します |
| 知っている (know) | ご存じです | 存じております |
| もらう (receive) | | いただきます |

Pattern for any verb: **お/ご + verb stem + します/いたします** (humble): ご連絡いたします (I'll contact you), お送りします (I'll send). **お/ご + stem + ください** (please, politely): ご確認ください (please check).

**Uchi/soto** (inside/outside): when talking to customers, your own boss is "inside", so you don't use respectful language for them, and you drop さん: 「田中は席を外しております」 (Tanaka is away from their desk).

## The phrases you'll use every day

| Phrase | When |
|---|---|
| **お疲れ様です** | greeting colleagues all day, starting chats and emails inside the company |
| **お先に失礼します** | leaving before others |
| **失礼します** | entering a room, ending a call |
| **承知しました** / **かしこまりました** | "understood" (to bosses / to customers). Not 了解です to superiors |
| **恐れ入りますが**、〜 | "sorry to trouble you, but..." before a request |
| **申し訳ございません** | a formal apology (to customers) |
| **すみません** | sorry / excuse me (everyday) |
| **少々お待ちください** | please wait a moment |
| **お手数ですが**、〜 | "sorry for the trouble, but could you..." |
| **ありがとうございます** | thank you |

## 報連相 (hō-ren-sō): the communication rule

- **報告** (hōkoku, report): tell your boss results and progress, *before* they ask. Bad news first and fast.
- **連絡** (renraku, inform): share facts with everyone affected (schedule changes, outages).
- **相談** (sōdan, consult): ask for advice *before* deciding alone.

A foreign engineer who reports early, often and briefly earns trust quickly. Silence while stuck is the worst thing you can do.

## Workplace culture, briefly

- **Punctuality**: being on time means being 5 minutes early.
- **Consensus** (根回し, nemawashi): ideas are discussed one-to-one before a meeting decides; meetings often confirm what's already agreed.
- **Documentation**: procedures (手順書), records, and reports are taken seriously, especially in operations (運用).
- **Hierarchy**: seniors (先輩) and juniors (後輩); seating order in meetings and taxis exists, so watch and copy.
- **Overtime** has been reduced by law (働き方改革, work-style reform), but varies a lot by company. Ask in interviews.
- **Drinking parties** (飲み会) are social but increasingly optional; it's fine to drink tea.

@@quiz
Your boss asks you to send a report by 3 p.m. What do you reply?
* 承知しました
- 了解
- OKです
- わかった
? 了解です is common between peers but considered too casual towards superiors by many.

@@quiz
Which is humble language (謙譲語) for "I will look at it"?
* 拝見します
- ご覧になります
- 見ます
- 見てください
? ご覧になる is respectful language for someone else's looking.

@@quiz
A customer calls for your manager, Suzuki, who is out. What do you say?
* 鈴木は席を外しております
- 鈴木さんはいらっしゃいません
- 鈴木様は外出なさっています
- 鈴木部長さんはいないです
? To outsiders, your own colleagues are "inside": no さん, no respectful verbs.

@@quiz
What does 報連相 stand for?
* Report, inform, consult
- Read, learn, practise
- Request, approve, close
- Plan, do, check
? 報告・連絡・相談.

@@quiz
You're stuck on a task and the deadline is near. What does good 報連相 suggest?
* Consult (相談) your senior or boss early
- Keep trying alone until the deadline
- Say nothing and finish it late
- Hand it to someone else without telling anyone
? Early bad news is far better than late surprises.

@@quiz
What do you say when leaving the office before your colleagues?
* お先に失礼します
- いただきます
- お疲れ様でした only
- さようなら
? Colleagues answer お疲れ様でした.

@@flashcards
敬語 :: Respectful language
丁寧語 :: Polite (です/ます)
尊敬語 :: Respectful: raises others' actions
謙譲語 :: Humble: lowers your own actions
いたします :: I do (humble)
参ります / 伺います :: I go/come; I visit (humble)
いらっしゃいます :: Is / comes / goes (respectful)
おっしゃいます :: Says (respectful)
拝見します :: I look (humble)
ご覧になります :: Looks (respectful)
ご確認ください :: Please check (polite)
お疲れ様です :: Standard workplace greeting
お先に失礼します :: Excuse me for leaving first
承知しました :: Understood (to superiors)
恐れ入りますが :: Sorry to trouble you, but...
お手数ですが :: Sorry for the trouble, but...
報連相 :: Report, inform, consult
根回し :: Building consensus beforehand
先輩 / 後輩 :: Senior / junior colleague
働き方改革 :: Work-style reform (overtime limits)

@@drill
What's the standard greeting to colleagues at work (also used to start internal emails)? (in Japanese)
= お疲れ様です
= おつかれさまです
= otsukaresama desu
= otsukaresamadesu
? Used from morning to evening.

@@exercise Uchi and soto: who gets honorifics?
In Japanese business, how you refer to people depends on who you're talking to. To an outside customer, your own colleagues and boss are "inside" (内): no さん and no titles after the name, and humble verbs. The customer's people are "outside" (外): 様 or a title, and respectful verbs. Write a function that picks the right form for a person and a company in each situation, and show the classic trap: your own manager, mentioned to a customer.
@@lang python
@@starter
people = {
    "田中": {"company": "own", "title": "部長"},
    "佐藤": {"company": "own", "title": None},
    "鈴木": {"company": "client", "title": "課長"},
    "山本": {"company": "client", "title": None},
}

def refer(name, listener):
    raise NotImplementedError

def company(which, listener, written=True):
    raise NotImplementedError
@@expected
talking to a colleague:
  田中: 田中部長
  佐藤: 佐藤さん
  鈴木: 鈴木課長
  山本: 山本様
talking to a client:
  田中: 部長の田中
  佐藤: 佐藤
  鈴木: 鈴木課長
  山本: 山本様
own company to a client: 弊社
client's company, email: 貴社 / speech: 御社
trap: to a client, say 「部長の田中は席を外しております」, not 「田中部長はいらっしゃいません」
@@requires
def refer
弊社
御社
貴社
@@hint
If the listener is from your own company, use name + title (or さん) for colleagues. If the listener is outside, refer to your own people by bare surname (or 「部長の田中」), and to the client's people with title or 様. Companies: your own is 弊社 (or 当社 internally). The listener's company is 御社 in speech and 貴社 in writing.
@@solution
people = {
    "田中": {"company": "own", "title": "部長"},
    "佐藤": {"company": "own", "title": None},
    "鈴木": {"company": "client", "title": "課長"},
    "山本": {"company": "client", "title": None},
}

def refer(name, listener):
    p = people[name]
    if p["company"] == "own" and listener == "client":
        return f"{p['title']}の{name}" if p["title"] else name
    if p["company"] == "own":
        return f"{name}{p['title']}" if p["title"] else f"{name}さん"
    return f"{name}{p['title']}" if p["title"] else f"{name}様"

def company(which, listener, written=True):
    if which == "own":
        return "弊社" if listener == "client" else "当社"
    return "貴社" if written else "御社"

for listener in ("colleague", "client"):
    print(f"talking to a {listener}:")
    for name in people:
        print(f"  {name}: {refer(name, listener)}")
print("own company to a client:", company("own", "client"))
print("client's company, email:", company("client", "client", written=True), "/ speech:", company("client", "client", written=False))
print(f"trap: to a client, say 「{refer('田中', 'client')}は席を外しております」, not 「田中部長はいらっしゃいません」")

@@exercise Sort messages into 報・連・相
Classify chat messages to a manager by the 報連相 rule and open each with the right phrase. Write `kind(message)` that returns `報告` for results and progress (keywords `done`, `finished`, `fixed`, `progress`, `failed`), `連絡` for facts everyone affected needs (`change`, `outage`, `maintenance`, `schedule`, `will be`), and `相談` for questions before deciding (`should I`, `which`, `advice`, `?`). Check 相談 first, then 報告, then 連絡.

Write `opener(k)` returning `ご報告です。`, `ご連絡です。` or `ご相談があるのですが、`, and print each message with its opener. Bad news first: list the failures (`failed`) at the top.
@@lang python
@@starter
messages = [
    "The server migration is finished and all checks passed",
    "Tomorrow's maintenance will be from 22:00 to 23:00",
    "Should I restart the database now or wait until tonight?",
    "The backup job failed last night",
    "There is an outage at the Osaka office network",
    "Progress: 3 of 5 laptops are set up",
    "Which vendor do you prefer for the new printers?",
]
RULES = [
    ("相談", ["should i", "which", "advice", "?"]),
    ("報告", ["done", "finished", "fixed", "progress", "failed"]),
    ("連絡", ["change", "outage", "maintenance", "schedule", "will be"]),
]
OPENERS = {"報告": "ご報告です。", "連絡": "ご連絡です。", "相談": "ご相談があるのですが、"}

def kind(message):
    raise NotImplementedError


def opener(k):
    raise NotImplementedError


ordered = sorted(messages, key=lambda m: "failed" not in m)
for m in ordered:
    k = kind(m)
    print(f"[{k}] {opener(k)} {m}")
@@expected
[報告] ご報告です。 The backup job failed last night
[報告] ご報告です。 The server migration is finished and all checks passed
[連絡] ご連絡です。 Tomorrow's maintenance will be from 22:00 to 23:00
[相談] ご相談があるのですが、 Should I restart the database now or wait until tonight?
[連絡] ご連絡です。 There is an outage at the Osaka office network
[報告] ご報告です。 Progress: 3 of 5 laptops are set up
[相談] ご相談があるのですが、 Which vendor do you prefer for the new printers?
@@requires
def kind
def opener
.lower()
@@hint
Lowercase once, then check the rule lists in order and return at the first match. `sorted(..., key=lambda m: "failed" not in m)` puts the failures first, because `False` sorts before `True`.
@@solution
messages = [
    "The server migration is finished and all checks passed",
    "Tomorrow's maintenance will be from 22:00 to 23:00",
    "Should I restart the database now or wait until tonight?",
    "The backup job failed last night",
    "There is an outage at the Osaka office network",
    "Progress: 3 of 5 laptops are set up",
    "Which vendor do you prefer for the new printers?",
]
RULES = [
    ("相談", ["should i", "which", "advice", "?"]),
    ("報告", ["done", "finished", "fixed", "progress", "failed"]),
    ("連絡", ["change", "outage", "maintenance", "schedule", "will be"]),
]
OPENERS = {"報告": "ご報告です。", "連絡": "ご連絡です。", "相談": "ご相談があるのですが、"}

def kind(message):
    text = message.lower()
    for k, words in RULES:
        if any(w in text for w in words):
            return k
    return "連絡"

def opener(k):
    return OPENERS[k]

ordered = sorted(messages, key=lambda m: "failed" not in m)
for m in ordered:
    k = kind(m)
    print(f"[{k}] {opener(k)} {m}")

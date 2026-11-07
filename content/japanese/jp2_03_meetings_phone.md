@@lesson jp2-meetings-phone Meetings and Phone Calls: 会議, 議事録 and 電話応対
@@text
Meetings and phone calls are where newcomers to Japanese workplaces feel the most pressure. Both follow **set patterns**, though, and once you know them you can follow along and take part. This lesson covers the flow of a meeting, the phrases for speaking up, writing minutes (議事録), and taking a phone message for a colleague (伝言).

## Meetings (会議・打ち合わせ)

- **打ち合わせ** is a smaller, working meeting. **会議** is more formal. **定例** is a regular meeting (weekly sync).
- Typical flow: 開始 (start) → 議題の確認 (agenda) → 報告 (reports) → 議論 (discussion) → 決定事項 (decisions) → 宿題・TODO (action items) → 次回 (next meeting).
- Useful phrases:
  - 「よろしいでしょうか。」 May I (speak / ask)?
  - 「一点確認させてください。」 Let me confirm one point.
  - 「おっしゃる通りです。」 Exactly as you say.
  - 「別の観点から申し上げると…」 From a different angle...
  - 「持ち帰って検討いたします。」 I'll take it back and consider it. (often a polite "not now")
  - 「認識を合わせたいのですが…」 I'd like to align our understanding...
- Decisions are often made **before** the meeting through informal agreement (**根回し**). The meeting confirms them.

## 議事録 (meeting minutes)

Standard headings: 日時, 場所 (or Teams/Zoom), 参加者, 議題, 決定事項, 宿題 (担当・期限), 次回予定. Minutes are usually sent the **same day**, with action items listing the person responsible (担当) and the deadline (期限).

## Phone calls (電話応対)

- Answer within three rings. If it took longer: 「お待たせいたしました。」
- 「はい、XYZ株式会社でございます。」 → caller: 「ABC社の鈴木と申します。田中様はいらっしゃいますか。」
- Your colleague is out: 「申し訳ございません。田中はただいま席を外しております。」 (no さん for your colleague to an outsider)
- Offer a callback or take a message: 「折り返しお電話いたしましょうか。」 「ご伝言を承りましょうか。」
- Confirm the details: 「念のため、お名前とお電話番号を頂戴できますか。」, then **repeat them back** (復唱).
- Close: 「私、スミスが承りました。」 「失礼いたします。」 Let the caller hang up first.

@@quiz
Which heading in 議事録 lists the agreed outcomes?
- 議題
* 決定事項
- 参加者
- 場所
? 宿題 lists the action items.

@@quiz
A customer calls for your colleague Tanaka, who is away from their desk. What do you say?
* 申し訳ございません。田中はただいま席を外しております。
- 田中さんはいません。
- 田中様はいらっしゃいません。
- 田中部長はお休みになっています。
? Inside people get no さん or respectful verbs when talking to outsiders.

@@quiz
What does 「持ち帰って検討いたします」 often mean in practice?
* A polite way to avoid deciding now
- Yes, definitely
- The meeting is cancelled
- Please send it by email
? Follow up later.

@@quiz
What is 根回し?
* Building informal agreement before the meeting
- Writing the minutes
- Booking the room
- A late-night meeting
? Meetings often confirm decisions already agreed.

@@quiz
What should you do with the caller's name and number?
* Repeat them back to confirm (復唱)
- Write them down silently
- Ask for an email instead
- Search for them online
? 復唱 avoids errors.

@@flashcards
打ち合わせ :: working meeting
会議 :: (formal) meeting
定例 :: regular meeting
議題 :: agenda item
決定事項 :: decisions
宿題 :: action items
担当 / 期限 :: owner / deadline
議事録 :: meeting minutes
根回し :: informal agreement beforehand
よろしいでしょうか :: may I?
一点確認させてください :: let me confirm one point
持ち帰って検討いたします :: I'll take it back and consider
席を外しております :: away from the desk
折り返しお電話いたします :: I'll call you back
ご伝言を承ります :: I'll take a message
復唱 :: repeating back to confirm

@@drill
Write the Japanese word for meeting minutes.
= 議事録
= ぎじろく
= gijiroku
? Sent the same day.

@@drill
What do you say when answering after several rings? (hiragana or kanji)
= お待たせいたしました
= おまたせいたしました
= お待たせいたしました。
? Literally "I have kept you waiting."

@@exercise Generate 議事録 from meeting notes
Turn structured meeting notes into standard 議事録 text with the usual headings, list the decisions, and format action items as 「・内容（担当：名前、期限：M/D）」, sorted by deadline. Flag action items that have no owner or no deadline.
@@lang python
@@concepts strings, lists, maps, conditions, functions
@@starter
from datetime import date
notes = {
    "title": "基盤チーム定例", "date": date(2026, 11, 12), "time": "10:00〜10:45", "place": "Teams",
    "attendees": ["田中", "佐藤", "スミス", "ABC社 鈴木"],
    "agenda": ["DBサーバー移行の進捗", "監視アラートの見直し"],
    "decisions": ["移行は11月29日（日）深夜に実施する", "ディスク使用率の警告閾値を80%に変更する"],
    "todos": [("移行手順書の最終版作成", "スミス", date(2026, 11, 20)), ("お客様への事前告知", "田中", date(2026, 11, 15)),
              ("アラート設定の変更", None, date(2026, 11, 18)), ("ロールバック手順の確認", "佐藤", None)],
    "next": date(2026, 11, 19),
}
WEEKDAYS = "月火水木金土日"

def jp_date(d):
    raise NotImplementedError
@@expected
【議事録】基盤チーム定例
日時：2026年11月12日（木） 10:00〜10:45
場所：Teams
参加者：田中、佐藤、スミス、ABC社 鈴木（敬称略）
議題：
  1. DBサーバー移行の進捗
  2. 監視アラートの見直し
決定事項：
  ・移行は11月29日（日）深夜に実施する
  ・ディスク使用率の警告閾値を80%に変更する
宿題：
  ・お客様への事前告知（担当：田中、期限：11/15）
  ・アラート設定の変更（担当：未定、期限：11/18）
  ・移行手順書の最終版作成（担当：スミス、期限：11/20）
  ・ロールバック手順の確認（担当：佐藤、期限：未定）
次回：2026年11月19日（木）
WARNING アラート設定の変更: no owner
WARNING ロールバック手順の確認: no deadline
@@requires
def jp_date
WEEKDAYS[
決定事項
宿題
@@hint
`jp_date` returns `f"{d.year}年{d.month}月{d.day}日（{WEEKDAYS[d.weekday()]}）"`. Sort the todos with a key that puts missing deadlines last: `(t[2] is None, t[2] or date.max)`. Use 「未定」 for a missing owner or deadline and collect warnings.
@@solution
from datetime import date
notes = {
    "title": "基盤チーム定例", "date": date(2026, 11, 12), "time": "10:00〜10:45", "place": "Teams",
    "attendees": ["田中", "佐藤", "スミス", "ABC社 鈴木"],
    "agenda": ["DBサーバー移行の進捗", "監視アラートの見直し"],
    "decisions": ["移行は11月29日（日）深夜に実施する", "ディスク使用率の警告閾値を80%に変更する"],
    "todos": [("移行手順書の最終版作成", "スミス", date(2026, 11, 20)), ("お客様への事前告知", "田中", date(2026, 11, 15)),
              ("アラート設定の変更", None, date(2026, 11, 18)), ("ロールバック手順の確認", "佐藤", None)],
    "next": date(2026, 11, 19),
}
WEEKDAYS = "月火水木金土日"

def jp_date(d):
    return f"{d.year}年{d.month}月{d.day}日（{WEEKDAYS[d.weekday()]}）"

n = notes
print(f"【議事録】{n['title']}")
print(f"日時：{jp_date(n['date'])} {n['time']}")
print(f"場所：{n['place']}")
print(f"参加者：{'、'.join(n['attendees'])}（敬称略）")
print("議題：")
for i, a in enumerate(n["agenda"], 1):
    print(f"  {i}. {a}")
print("決定事項：")
for d in n["decisions"]:
    print(f"  ・{d}")
print("宿題：")
warnings = []
for task, owner, due in sorted(n["todos"], key=lambda t: (t[2] is None, t[2] or date.max)):
    print(f"  ・{task}（担当：{owner or '未定'}、期限：{f'{due.month}/{due.day}' if due else '未定'}）")
    if not owner or not due:
        warnings.append(f"{task}: {'no owner' if not owner else 'no deadline'}")
print(f"次回：{jp_date(n['next'])}")
for w in warnings:
    print("WARNING", w)

@@exercise Take a phone message (伝言メモ)
Simulate the phone flow: choose the right response for each situation (the colleague is available, away from their desk, in a meeting, or off today), refer to your colleague without さん, and produce a 伝言メモ for the colleague with time, caller, company, callback number (repeated back digit by digit in groups), and the message.
@@lang python
@@starter
STATUS = {"available": "少々お待ちください。おつなぎいたします。",
          "away": "ただいま席を外しております。",
          "meeting": "ただいま会議中でございます。",
          "off": "本日は休みをいただいております。"}
calls = [
    {"time": "14:05", "caller": "鈴木", "company": "ABC社", "for": "田中", "status": "away", "phone": "0312345678", "message": "見積書の件で折り返しお電話をいただきたい"},
    {"time": "15:30", "caller": "山本", "company": "DEF社", "for": "佐藤", "status": "meeting", "phone": "0667891234", "message": "明日の打ち合わせを30分遅らせたい"},
    {"time": "16:10", "caller": "高橋", "company": "GHI社", "for": "田中", "status": "available", "phone": "", "message": ""},
]
ME = "スミス"

def respond(call):
    raise NotImplementedError
@@expected
ABC社の鈴木様 → 申し訳ございません。田中はただいま席を外しております。折り返しお電話いたしましょうか。
  復唱：「03-1234-5678、ABC社の鈴木様でございますね。私、スミスが承りました。」
  【伝言メモ】田中さんへ
   14:05 ABC社 鈴木様より
   内容：見積書の件で折り返しお電話をいただきたい
   連絡先：03-1234-5678　受付：スミス
DEF社の山本様 → 申し訳ございません。佐藤はただいま会議中でございます。折り返しお電話いたしましょうか。
  復唱：「06-6789-1234、DEF社の山本様でございますね。私、スミスが承りました。」
  【伝言メモ】佐藤さんへ
   15:30 DEF社 山本様より
   内容：明日の打ち合わせを30分遅らせたい
   連絡先：06-6789-1234　受付：スミス
GHI社の高橋様 → 少々お待ちください。おつなぎいたします。
@@requires
def respond
STATUS[
折り返し
@@hint
`respond` returns the line said to the caller: 「申し訳ございません。{colleague}は」 + the STATUS text after removing 「ただいま」/「本日は」 duplicates, or just STATUS["available"]. When not available, offer 「折り返しお電話いたしましょうか。」. Format the phone number for repeating back as 03-1234-5678 (2-4-4 for 03 and 06 numbers), then print the memo.
@@solution
STATUS = {"available": "少々お待ちください。おつなぎいたします。",
          "away": "ただいま席を外しております。",
          "meeting": "ただいま会議中でございます。",
          "off": "本日は休みをいただいております。"}
calls = [
    {"time": "14:05", "caller": "鈴木", "company": "ABC社", "for": "田中", "status": "away", "phone": "0312345678", "message": "見積書の件で折り返しお電話をいただきたい"},
    {"time": "15:30", "caller": "山本", "company": "DEF社", "for": "佐藤", "status": "meeting", "phone": "0667891234", "message": "明日の打ち合わせを30分遅らせたい"},
    {"time": "16:10", "caller": "高橋", "company": "GHI社", "for": "田中", "status": "available", "phone": "", "message": ""},
]
ME = "スミス"

def respond(call):
    if call["status"] == "available":
        return STATUS["available"]
    return f"申し訳ございません。{call['for']}は{STATUS[call['status']]}折り返しお電話いたしましょうか。"

def phone_format(n):
    return f"{n[:2]}-{n[2:6]}-{n[6:]}"

for call in calls:
    print(f"{call['company']}の{call['caller']}様 → {respond(call)}")
    if call["status"] == "available":
        continue
    print(f"  復唱：「{phone_format(call['phone'])}、{call['company']}の{call['caller']}様でございますね。私、{ME}が承りました。」")
    print(f"  【伝言メモ】{call['for']}さんへ")
    print(f"   {call['time']} {call['company']} {call['caller']}様より")
    print(f"   内容：{call['message']}")
    print(f"   連絡先：{phone_format(call['phone'])}　受付：{ME}")

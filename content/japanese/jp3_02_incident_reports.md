@@lesson jp3-incident-reports 障害報告 and Status Updates: Writing Like a Japanese IT Team
@@text
When a system goes down in a Japanese company, the incident report (**障害報告**) and its updates matter almost as much as the fix. Customers and managers expect a fixed structure, careful time notation, apologies at the right points and humble language for your own actions. This lesson gives the structure, the set phrases and the vocabulary, and the exercises generate reports and status updates from incident data.

## Structure of a 障害報告

| Section | Example |
|---|---|
| 件名 (subject) | 【障害報告】ログイン不可の件（第2報） |
| 宛名 | ○○株式会社　情報システム部　佐藤様 |
| 挨拶 | いつもお世話になっております。株式会社△△の田中です。 |
| 謝罪 | このたびはご迷惑をおかけしており、大変申し訳ございません。 |
| 発生日時 | 2026年11月5日（木）9時12分 |
| 影響範囲 | 全ユーザーがログインできない状態 |
| 原因 | 認証サーバーの証明書の期限切れ（調査中 if unknown） |
| 対応状況 | 証明書を更新し、10時03分に復旧いたしました |
| 今後の対応 / 再発防止策 | 期限の自動監視を導入いたします |
| 結び | 引き続きどうぞよろしくお願いいたします。 |

- Updates are numbered: **第1報** (first report, quick, facts only), **第2報**, … and the **最終報告** (final report with root cause and prevention).
- Send the first report early, even with 原因：調査中 (cause: under investigation). Promise the next update time: 次回は11時頃にご報告いたします.
- Subjects use 【】 brackets for the category: 【障害報告】, 【復旧報告】, 【メンテナンスのお知らせ】.

## Key vocabulary

| Japanese | Reading | Meaning |
|---|---|---|
| 障害 | しょうがい | failure, outage |
| 発生 | はっせい | occurrence |
| 復旧 | ふっきゅう | recovery, restored |
| 暫定対応 | ざんていたいおう | workaround, temporary fix |
| 恒久対応 | こうきゅうたいおう | permanent fix |
| 影響範囲 | えいきょうはんい | scope of impact |
| 原因究明 | げんいんきゅうめい | root-cause investigation |
| 再発防止策 | さいはつぼうしさく | prevention measures |
| 切り戻し | きりもどし | rollback |
| 調査中 | ちょうさちゅう | under investigation |

## Times and dates

Write the day of the week in brackets: 11月5日（木）. Use the 24-hour clock in reports (14時30分 or 14:30) and state the timezone (JST) when overseas teams are involved. Durations: 約2時間 (about two hours). Ranges: 9時12分～10時03分.

## Phrases

- Apology: ご迷惑をおかけし、大変申し訳ございません。(formal) / ご不便をおかけしております。(ongoing inconvenience)
- Reporting your action humbly: 対応いたしました, 確認いたしました, ご報告いたします.
- Asking: ご確認いただけますでしょうか。/ ご協力をお願いいたします。
- Thanks for patience: ご理解のほど、よろしくお願い申し上げます。

@@quiz
What is the first update of an incident report usually called?
* 第1報
- 最終報告
- 復旧報告
- 議事録
? Later ones are 第2報, 第3報…

@@quiz
The cause isn't known yet. What do you write?
- Nothing; wait until you know
* 原因：調査中, and send the report anyway
- A guess
- 不明 without a next update time
? Report early, promise the next update.

@@quiz
What does 暫定対応 mean?
- Permanent fix
* Temporary fix or workaround
- Rollback
- Root cause
? 恒久対応 is the permanent fix.

@@quiz
Which phrase humbly reports your own action?
- 対応されました
* 対応いたしました
- 対応してくれました
- 対応なさいました
? いたす is humble.

@@quiz
How is the date usually written in a report?
- 5/11/2026
* 2026年11月5日（木）
- Nov 5th
- 木曜日5日
? With the weekday in brackets.

@@flashcards
障害報告 :: incident report
第1報 :: first report
最終報告 :: final report
復旧 :: recovery
暫定対応 :: workaround
恒久対応 :: permanent fix
影響範囲 :: scope of impact
原因究明 :: root-cause investigation
再発防止策 :: prevention measures
切り戻し :: rollback
調査中 :: under investigation
【】 :: subject category brackets
ご迷惑をおかけし :: apology for trouble
いたしました :: humble "did"
次回のご報告 :: next update

@@drill
What is the Japanese word for a rollback?
= 切り戻し
= きりもどし
= kirimodoshi
? Also ロールバック in many teams.

@@drill
What is the reading of 復旧?
= ふっきゅう
= fukkyuu
= fukkyu
= fukkyū
? Recovery, restored.

@@exercise Generate a 障害報告
From incident data, generate a numbered incident report in the standard structure: a 【障害報告】 subject with the report number (or 【復旧報告】 once recovered), the date with the weekday in brackets, times as H時MM分, the duration if recovered, 調査中 when the cause is unknown, and the next update time when not yet recovered.
@@lang python
@@starter
from datetime import datetime

WEEKDAYS = "月火水木金土日"
incidents = [
    {"no": 1, "title": "ログイン不可", "customer": "○○株式会社 佐藤様", "start": "2026-11-05 09:12", "end": None,
     "impact": "全ユーザーがログインできない状態", "cause": None, "action": "認証サーバーを調査しております", "next": "2026-11-05 10:30"},
    {"no": 2, "title": "ログイン不可", "customer": "○○株式会社 佐藤様", "start": "2026-11-05 09:12", "end": "2026-11-05 10:03",
     "impact": "全ユーザーがログインできない状態", "cause": "認証サーバーの証明書の期限切れ", "action": "証明書を更新いたしました", "next": None},
]

def jp_date(dt):
    raise NotImplementedError
@@expected
件名：【障害報告】ログイン不可の件（第1報）

○○株式会社 佐藤様

いつもお世話になっております。株式会社△△の田中です。
このたびはご迷惑をおかけしており、大変申し訳ございません。

■発生日時：2026年11月5日（木）9時12分
■影響範囲：全ユーザーがログインできない状態
■原因：調査中
■対応状況：認証サーバーを調査しております
次回は10時30分頃にご報告いたします。

引き続きどうぞよろしくお願いいたします。
--------------------
件名：【復旧報告】ログイン不可の件（第2報）

○○株式会社 佐藤様

いつもお世話になっております。株式会社△△の田中です。
このたびはご迷惑をおかけしており、大変申し訳ございません。

■発生日時：2026年11月5日（木）9時12分
■影響範囲：全ユーザーがログインできない状態
■原因：認証サーバーの証明書の期限切れ
■対応状況：証明書を更新いたしました
■復旧日時：10時03分（影響時間：約51分）
再発防止策につきましては、最終報告にてご報告いたします。

引き続きどうぞよろしくお願いいたします。
--------------------
@@requires
def jp_date
WEEKDAYS[
調査中
@@hint
`jp_date` → `f"{dt.year}年{dt.month}月{dt.day}日（{WEEKDAYS[dt.weekday()]}）"`; `jp_time` → `f"{dt.hour}時{dt.minute:02d}分"`. Duration in minutes = `(end - start).seconds // 60`, shown as `約X時間Y分` or `約Y分`. Build the lines in order and print them.
@@solution
from datetime import datetime

WEEKDAYS = "月火水木金土日"
incidents = [
    {"no": 1, "title": "ログイン不可", "customer": "○○株式会社 佐藤様", "start": "2026-11-05 09:12", "end": None,
     "impact": "全ユーザーがログインできない状態", "cause": None, "action": "認証サーバーを調査しております", "next": "2026-11-05 10:30"},
    {"no": 2, "title": "ログイン不可", "customer": "○○株式会社 佐藤様", "start": "2026-11-05 09:12", "end": "2026-11-05 10:03",
     "impact": "全ユーザーがログインできない状態", "cause": "認証サーバーの証明書の期限切れ", "action": "証明書を更新いたしました", "next": None},
]

def parse(text):
    return datetime.strptime(text, "%Y-%m-%d %H:%M")

def jp_date(dt):
    return f"{dt.year}年{dt.month}月{dt.day}日（{WEEKDAYS[dt.weekday()]}）"

def jp_time(dt):
    return f"{dt.hour}時{dt.minute:02d}分"

def report(inc):
    start = parse(inc["start"])
    end = parse(inc["end"]) if inc["end"] else None
    kind = "【復旧報告】" if end else "【障害報告】"
    lines = [f"件名：{kind}{inc['title']}の件（第{inc['no']}報）", "", inc["customer"], "",
             "いつもお世話になっております。株式会社△△の田中です。",
             "このたびはご迷惑をおかけしており、大変申し訳ございません。", "",
             f"■発生日時：{jp_date(start)}{jp_time(start)}",
             f"■影響範囲：{inc['impact']}",
             f"■原因：{inc['cause'] or '調査中'}",
             f"■対応状況：{inc['action']}"]
    if end:
        minutes = (end - start).seconds // 60
        length = f"約{minutes // 60}時間{minutes % 60}分" if minutes >= 60 else f"約{minutes}分"
        lines.append(f"■復旧日時：{jp_time(end)}（影響時間：{length}）")
        lines.append("再発防止策につきましては、最終報告にてご報告いたします。")
    else:
        lines.append(f"次回は{jp_time(parse(inc['next']))}頃にご報告いたします。")
    lines += ["", "引き続きどうぞよろしくお願いいたします。"]
    return "\n".join(lines)

for inc in incidents:
    print(report(inc))
    print("-" * 20)

@@exercise Choose the right phrase
Given a list of situations (apologising for an outage, ongoing inconvenience, asking a customer to check something, reporting your own completed action, thanking for patience), pick the matching set phrase, and rewrite draft sentences that use casual forms which shouldn't appear in a customer report (した, します, が〜してくれた, やった, ごめん) with the polite replacements in `FIXES`. A customer's action becomes 〜にご確認いただきました (we received the favour of their checking), not が〜していただいた.
@@lang python
@@starter
import re

PHRASES = {
    "apology": "ご迷惑をおかけし、大変申し訳ございません。",
    "inconvenience": "ご不便をおかけしております。",
    "request_check": "ご確認いただけますでしょうか。",
    "report_done": "対応いたしました。",
    "patience": "ご理解のほど、よろしくお願い申し上げます。",
}
FIXES = {"ごめん、": "申し訳ございません。", "が確認してくれた。": "にご確認いただきました。", "やった。": "対応いたしました。", "した。": "いたしました。", "します。": "いたします。"}
situations = ["the service was down for an hour", "users still see slow pages", "the customer should verify their login", "we finished the patch", "thank them for waiting"]
drafts = ["サーバーを再起動した。", "佐藤様が確認してくれた。", "ごめん、すぐ確認します。", "対応いたしました。"]

def pick(situation):
    raise NotImplementedError
@@expected
the service was down for an hour: ご迷惑をおかけし、大変申し訳ございません。
users still see slow pages: ご不便をおかけしております。
the customer should verify their login: ご確認いただけますでしょうか。
we finished the patch: 対応いたしました。
thank them for waiting: ご理解のほど、よろしくお願い申し上げます。
サーバーを再起動した。 -> サーバーを再起動いたしました。
佐藤様が確認してくれた。 -> 佐藤様にご確認いただきました。
ごめん、すぐ確認します。 -> 申し訳ございません。すぐ確認いたします。
対応いたしました。 -> OK
@@requires
def pick
FIXES
PHRASES[
@@hint
Match keywords in the situation: "down" or "outage" → apology; "still" or "slow" → inconvenience; "verify" or "check" → request_check; "finished" or "done" → report_done; "waiting" or "patience" → patience. For drafts already ending in いたしました。 or いたします。 print OK; otherwise replace in a single pass with one regex built from the FIXES keys (longest first), so a replacement is never re-matched (いただきました。 also ends in した。).
@@solution
import re

PHRASES = {
    "apology": "ご迷惑をおかけし、大変申し訳ございません。",
    "inconvenience": "ご不便をおかけしております。",
    "request_check": "ご確認いただけますでしょうか。",
    "report_done": "対応いたしました。",
    "patience": "ご理解のほど、よろしくお願い申し上げます。",
}
FIXES = {"ごめん、": "申し訳ございません。", "が確認してくれた。": "にご確認いただきました。", "やった。": "対応いたしました。", "した。": "いたしました。", "します。": "いたします。"}
situations = ["the service was down for an hour", "users still see slow pages", "the customer should verify their login", "we finished the patch", "thank them for waiting"]
drafts = ["サーバーを再起動した。", "佐藤様が確認してくれた。", "ごめん、すぐ確認します。", "対応いたしました。"]
PATTERN = re.compile("|".join(re.escape(k) for k in sorted(FIXES, key=len, reverse=True)))
KEYWORDS = [(("down", "outage"), "apology"), (("still", "slow"), "inconvenience"), (("verify", "check"), "request_check"),
            (("finished", "done"), "report_done"), (("waiting", "patience"), "patience")]

def pick(situation):
    for words, key in KEYWORDS:
        if any(w in situation for w in words):
            return PHRASES[key]
    return None

for s in situations:
    print(f"{s}: {pick(s)}")
for d in drafts:
    fixed = d
    if not fixed.endswith(("いたしました。", "いたします。")):
        fixed = PATTERN.sub(lambda m: FIXES[m.group()], fixed)
    print(f"{d} -> {'OK' if fixed == d else fixed}")

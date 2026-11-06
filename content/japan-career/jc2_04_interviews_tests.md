@@lesson jc2-interviews-tests Interviews in Japan: The Selection Flow, Aptitude Tests, Common Questions and Scheduling in Japanese
@@concepts strings, conditions, lists
@@lang python
@@text
Japanese hiring (選考) has more steps than many candidates expect, and each step has its own conventions. Foreign-friendly and 外資系 companies may run a Western-style process in English. Traditional Japanese companies, SIers and many SES firms follow the patterns below closely, even for mid-career (中途採用) hires.

## The selection flow

1. **書類選考** (document screening): your 履歴書 and 職務経歴書. Typical results come in about a week.
2. **適性検査** (aptitude test): often **SPI** (verbal 言語, numerical 非言語 and a personality section) or similar tests like 玉手箱 and CAB/GAB for IT roles. For engineers, a **coding test** (online, 60–90 minutes) is common.
3. **一次面接** (first interview): usually with HR or the team leader. They check basic fit, communication and Japanese level.
4. **二次面接** (second interview): the department manager. They go deep on skills and experience.
5. **最終面接** (final interview): an executive or director. They check motivation, values and long-term fit. Even if it feels like a formality, prepare: people do fail here.
6. **内定** (job offer), followed by **労働条件通知書** (the written terms, covered in the offers lesson), then **内定承諾** (acceptance) or **内定辞退** (declining politely).

Through an agency (エージェント), the recruiter schedules interviews, prepares you, gives you feedback and negotiates salary. Use them.

## Common questions and how to answer

- **自己紹介 / 自己PR**: one minute (roughly 300 Japanese characters) summarising your experience and strengths, tied to the role.
- **志望動機** (why this company): be specific, mentioning their products, technology, clients or culture, and how your experience fits. "Because it's a big company" is a weak answer.
- **転職理由** (why you're changing jobs): always **positive and forward-looking**: "I want to work on cloud migration projects" rather than "my manager was bad". Never criticise your current employer.
- **長所と短所** (strengths and weaknesses): give a real weakness plus what you're doing about it.
- **Why Japan? How long will you stay?** Employers worry foreign hires will leave quickly. Show commitment: your life plans, Japanese study, family.
- **逆質問** (your questions for them): always have 2–3 ready: the team's tech stack, how success is measured, the onboarding process. Avoid asking about holidays and overtime as your first question. Ask through the agent instead.

A structure that works for most answers is **結論 → 理由 → 具体例 → 結論** (conclusion, reason, example, conclusion), the same as PREP in English.

## Manners

- **In person**: arrive 5–10 minutes early, knock three times, wait for 「どうぞ」, say 「失礼します」, bow, and don't sit until invited (「おかけください」). At the end, stand, thank them (「本日はありがとうございました」), bow at the door, and say 「失礼いたします」.
- **Online**: test your camera and audio, use a plain background, look at the camera when speaking, and join a few minutes early.
- **Dress**: a dark suit is safest for traditional companies. Business casual for many startups and 外資系, but ask the agent.
- Send a short thank-you email the same day if you applied directly. Through an agency, tell the recruiter instead.

## Scheduling in Japanese

You'll often be asked for 候補日 (possible dates). The standard format is 6月3日(火) 10:00〜11:00: month, day, the weekday kanji in brackets, and a time range with 〜. The weekday kanji are 月 (Mon), 火 (Tue), 水 (Wed), 木 (Thu), 金 (Fri), 土 (Sat), 日 (Sun). Offer three or more options.

```
お世話になっております。○○です。
面接日程について、以下の日時でしたら参加可能です。
・6月3日(火) 10:00〜11:00
・6月4日(水) 14:00〜15:00
・6月6日(金) 16:00〜17:00
ご確認のほど、よろしくお願いいたします。
```

@@quiz
Which step usually checks motivation and long-term fit with an executive?
- 書類選考
- 一次面接
* 最終面接
- 適性検査
? Prepare for it seriously.

@@quiz
How should you answer 転職理由?
* Positively, about what you want to do next
- Explain everything wrong with your current employer
- Say you don't know
- Talk only about salary
? Never criticise your current employer.

@@quiz
What is SPI?
* A common aptitude test with verbal, numerical and personality sections
- A visa document
- A type of interview
- A salary negotiation
? 言語, 非言語 and personality.

@@quiz
In a Japanese interview room, when should you sit down?
* When the interviewer invites you to
- As soon as you enter
- After bowing twice
- When the second interviewer arrives
? 「おかけください」.

@@quiz
What is 逆質問?
* The questions you ask the interviewers
- A trick question
- A follow-up interview
- A rejection
? Always prepare a few.

@@quiz
How do you write Tuesday 3 June, 10 to 11 am, as a 候補日?
* 6月3日(火) 10:00〜11:00
- 3/6 Tue 10-11
- 6月3日(水) 10:00〜11:00
- 火6月3日 10時
? 火 is Tuesday.

@@flashcards
選考 :: selection process
書類選考 :: document screening
適性検査 :: aptitude test (SPI, 玉手箱)
一次 / 二次 / 最終面接 :: first / second / final interview
内定 :: job offer
内定承諾 / 内定辞退 :: accept / decline the offer
自己PR :: self-promotion, about 1 minute
志望動機 :: why this company
転職理由 :: why you're changing jobs, keep it positive
逆質問 :: your questions for them
結論 → 理由 → 具体例 → 結論 :: answer structure
候補日 :: possible dates
月火水木金土日 :: Mon to Sun
失礼します :: said entering and leaving
おかけください :: please sit down

@@drill
Which weekday kanji means Thursday?
= 木
? 木曜日.

@@drill
About how many Japanese characters is a one-minute 自己PR?
= 300
? Roughly 300 characters a minute.

@@exercise Write 候補日 lines
From a list of free time slots, produce the 候補日 lines in the standard Japanese format, skipping weekends and Japanese holidays, slots shorter than the interview length, and slots that start before 9:00 or end after 18:00. Offer at most three, earliest first.
@@starter
from datetime import datetime, timedelta

WEEKDAYS = "月火水木金土日"
HOLIDAYS = {"2026-07-20"}
LENGTH = timedelta(hours=1)
free = [
    ("2026-07-17 08:00", "2026-07-17 09:30"),
    ("2026-07-17 15:00", "2026-07-17 16:00"),
    ("2026-07-18 10:00", "2026-07-18 12:00"),
    ("2026-07-20 10:00", "2026-07-20 11:00"),
    ("2026-07-21 13:30", "2026-07-21 14:00"),
    ("2026-07-21 16:30", "2026-07-21 18:30"),
    ("2026-07-22 10:00", "2026-07-22 12:00"),
    ("2026-07-23 14:00", "2026-07-23 15:00"),
]

def candidate(start, end):
    raise NotImplementedError

def format_slot(start):
    raise NotImplementedError

lines = []
for s, e in free:
    start = candidate(datetime.strptime(s, "%Y-%m-%d %H:%M"), datetime.strptime(e, "%Y-%m-%d %H:%M"))
    if start:
        lines.append(format_slot(start))
print("面接日程について、以下の日時でしたら参加可能です。")
for line in lines[:3]:
    print("・" + line)
@@expected
面接日程について、以下の日時でしたら参加可能です。
・7月17日(金) 15:00〜16:00
・7月21日(火) 16:30〜17:30
・7月22日(水) 10:00〜11:00
@@requires
def candidate
def format_slot
WEEKDAYS[
@@hint
`candidate` returns the earliest usable start or `None`: skip `start.weekday() >= 5` and holidays (`start.strftime("%Y-%m-%d") in HOLIDAYS`); move the start up to 9:00 if earlier; the slot is usable if `start + LENGTH` is no later than both `end` and 18:00 that day. `format_slot` builds `f"{start.month}月{start.day}日({WEEKDAYS[start.weekday()]}) {start:%H:%M}〜{start + LENGTH:%H:%M}"`.
@@solution
from datetime import datetime, timedelta

WEEKDAYS = "月火水木金土日"
HOLIDAYS = {"2026-07-20"}
LENGTH = timedelta(hours=1)
free = [
    ("2026-07-17 08:00", "2026-07-17 09:30"),
    ("2026-07-17 15:00", "2026-07-17 16:00"),
    ("2026-07-18 10:00", "2026-07-18 12:00"),
    ("2026-07-20 10:00", "2026-07-20 11:00"),
    ("2026-07-21 13:30", "2026-07-21 14:00"),
    ("2026-07-21 16:30", "2026-07-21 18:30"),
    ("2026-07-22 10:00", "2026-07-22 12:00"),
    ("2026-07-23 14:00", "2026-07-23 15:00"),
]

def candidate(start, end):
    if start.weekday() >= 5 or start.strftime("%Y-%m-%d") in HOLIDAYS:
        return None
    start = max(start, start.replace(hour=9, minute=0))
    close = start.replace(hour=18, minute=0)
    if start + LENGTH <= min(end, close):
        return start
    return None

def format_slot(start):
    return f"{start.month}月{start.day}日({WEEKDAYS[start.weekday()]}) {start:%H:%M}〜{start + LENGTH:%H:%M}"

lines = []
for s, e in free:
    start = candidate(datetime.strptime(s, "%Y-%m-%d %H:%M"), datetime.strptime(e, "%Y-%m-%d %H:%M"))
    if start:
        lines.append(format_slot(start))
print("面接日程について、以下の日時でしたら参加可能です。")
for line in lines[:3]:
    print("・" + line)

@@exercise Check interview answers
Check draft answers before an interview: estimate the speaking time (about 300 Japanese characters per minute, or 150 English words per minute), flag answers longer than the target, flag negative words in a 転職理由, and check that an answer starts with a conclusion (its first sentence names what you want or offer, using one of the given phrases).
@@starter
NEGATIVE = ["上司", "不満", "残業が多", "給料が低", "boss", "hate", "unfair", "underpaid"]
CONCLUSION = ["たい", "できます", "強みは", "want to", "I can", "my strength"]
answers = [
    {"q": "自己PR", "target": 60, "text": "私の強みは、AWSを使ったインフラ構築です。前職では3年間、ECサイトの基盤を担当しました。" * 8},
    {"q": "転職理由", "target": 45, "text": "上司とのコミュニケーションに不満があり、残業が多かったためです。"},
    {"q": "転職理由", "target": 45, "text": "クラウド移行の案件に、より深く関わりたいと考えています。現職では運用が中心のため、設計から携われる環境を探しています。"},
    {"q": "Why this company", "target": 45, "text": "I want to build developer tools used across Asia. Your platform team works on exactly that, and my five years of CI/CD work fits it well."},
]

def seconds(text):
    raise NotImplementedError

def check(answer):
    raise NotImplementedError

for a in answers:
    problems = check(a)
    print(f"{a['q']:<17} {seconds(a['text']):>3.0f}s  " + ("ok" if not problems else "; ".join(problems)))
@@expected
自己PR               74s  too long (74s, target 60s)
転職理由                6s  negative words: 上司, 不満, 残業が多; start with your conclusion
転職理由               12s  ok
Why this company   10s  ok
@@requires
def seconds
def check
isascii()
@@hint
If `text.isascii()`, count words with `len(text.split())` at 150 per minute; otherwise count characters with `len(text)` at 300 per minute. In `check`, collect problems: "too long (Ns, target Ts)", "negative words: ..." for a 転職理由 containing any NEGATIVE word, and "start with your conclusion" when the first sentence (split on "。" or ".") contains none of the CONCLUSION phrases.
@@solution
NEGATIVE = ["上司", "不満", "残業が多", "給料が低", "boss", "hate", "unfair", "underpaid"]
CONCLUSION = ["たい", "できます", "強みは", "want to", "I can", "my strength"]
answers = [
    {"q": "自己PR", "target": 60, "text": "私の強みは、AWSを使ったインフラ構築です。前職では3年間、ECサイトの基盤を担当しました。" * 8},
    {"q": "転職理由", "target": 45, "text": "上司とのコミュニケーションに不満があり、残業が多かったためです。"},
    {"q": "転職理由", "target": 45, "text": "クラウド移行の案件に、より深く関わりたいと考えています。現職では運用が中心のため、設計から携われる環境を探しています。"},
    {"q": "Why this company", "target": 45, "text": "I want to build developer tools used across Asia. Your platform team works on exactly that, and my five years of CI/CD work fits it well."},
]

def seconds(text):
    if text.isascii():
        return len(text.split()) / 150 * 60
    return len(text) / 300 * 60

def check(answer):
    text, problems = answer["text"], []
    s = seconds(text)
    if s > answer["target"]:
        problems.append(f"too long ({s:.0f}s, target {answer['target']}s)")
    if answer["q"] == "転職理由":
        found = [w for w in NEGATIVE if w in text]
        if found:
            problems.append("negative words: " + ", ".join(found))
    first = text.replace("。", ".").split(".")[0]
    if not any(p in first for p in CONCLUSION):
        problems.append("start with your conclusion")
    return problems

for a in answers:
    problems = check(a)
    print(f"{a['q']:<17} {seconds(a['text']):>3.0f}s  " + ("ok" if not problems else "; ".join(problems)))

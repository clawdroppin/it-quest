@@lesson fe0-about 基本情報技術者試験: The Exam
@@text
The **Fundamental Information Technology Engineer Examination** (基本情報技術者試験, **FE**, 「基本情報」) is Japan's national IT exam, run by **IPA** (情報処理推進機構). Japanese employers know it, many SIers expect new engineers to pass it in their first year, and some pay a bonus (資格手当) for it. It's also part of the **Information Technology Professionals Examination** family recognized in several Asian countries through mutual recognition agreements.

## Why it matters if you're moving to Japan

- It proves fundamentals in a format every Japanese manager recognizes. It's the "CompTIA A+ / Network+" of Japan, but broader: theory, algorithms, management, law.
- Certain IPA exams are listed by the Ministry of Justice as IT qualifications that can **substitute for the degree/experience requirement** for the **技術・人文知識・国際業務** work visa, and IT qualifications can earn points in the **Highly Skilled Professional** system. Rules change: check the current 法務省 notice (IT告示) and ask an immigration lawyer (行政書士) before relying on this.
- Japanese IT vocabulary: studying FE teaches you the exact terms (排他制御, 正規化, 稼働率...) used at work.

## The format (CBT since 2023)

| | 科目A | 科目B |
|---|---|---|
| Questions | 60 multiple choice | 20 |
| Time | 90 minutes | 100 minutes |
| Covers | everything: テクノロジ系, マネジメント系, ストラテジ系 | **algorithms and programming** (16 questions, in **疑似言語** pseudo-language) + **information security** (4) |
| Pass | 600 / 1000 | 600 / 1000 |

You need **both** on the same day. It's held **year-round** at test centres (Prometric); book online. The fee is about **¥7,500**; check IPA's site for the current fee and dates. The exam is in **Japanese**: there's no English version, so this track teaches the Japanese terms alongside the English.

## The ladder

| Level | Exam | For |
|---|---|---|
| 1 | **ITパスポート** (IP) | every working person: basic IT, business, law |
| 2 | **基本情報技術者** (FE) | entry-level engineers |
| 2 | 情報セキュリティマネジメント (SG) | security for non-engineers |
| 3 | **応用情報技術者** (AP) | experienced engineers: written answers in the afternoon |
| 4 | 高度試験: ネットワークスペシャリスト, 情報処理安全確保支援士 (registered security specialist), データベーススペシャリスト, プロジェクトマネージャ, etc. | specialists |

## How to study

1. Learn the vocabulary **in Japanese** from the start (this track has the terms).
2. **科目B is the hard part**: practise tracing pseudo-code by hand with a **trace table** (トレース表): every variable in a column, one row per step. Do it on paper, on time.
3. Do past questions (過去問): IPA publishes sample questions, and sites like 「基本情報技術者試験ドットコム」 have thousands of old 科目A-style questions with explanations.
4. Popular books: the 「キタミ式イラストIT塾 基本情報技術者」 (illustrated, beginner-friendly) and 「栢木先生の基本情報技術者教室」.

@@quiz
How many questions and how long is 科目B?
* 20 questions, 100 minutes
- 60 questions, 90 minutes
- 80 questions, 150 minutes
- 4 essays, 120 minutes
? 科目A is 60 questions in 90 minutes.

@@quiz
What does 科目B mainly test?
* Algorithms and programming in pseudo-language, plus information security
- Business strategy essays
- Japanese grammar
- Typing speed
? 16 algorithm questions and 4 security questions.

@@quiz
Which IPA exam comes right after 基本情報技術者 on the ladder?
* 応用情報技術者 (AP)
- ITパスポート
- CCNA
- 情報処理安全確保支援士
? The 高度試験 come after AP.

@@quiz
What score is needed to pass each part?
* 600 out of 1000 in both 科目A and 科目B
- 60% overall
- 700 in 科目A only
- 26 out of 40
? You need both on the same day.

@@flashcards
基本情報技術者試験 (FE) :: Fundamental IT Engineer Exam
IPA (情報処理推進機構) :: Runs Japan's national IT exams
ITパスポート :: Level 1, for every working person
応用情報技術者 (AP) :: Level 3, after FE
科目A :: 60 questions, 90 minutes, all areas
科目B :: 20 questions, 100 minutes: algorithms (疑似言語) + security
疑似言語 :: The exam's pseudo-language
過去問 :: Past exam questions
トレース表 :: Trace table: variables per step
テクノロジ系 / マネジメント系 / ストラテジ系 :: Technology / management / strategy areas
資格手当 :: Company bonus for holding a qualification

@@drill
What's the Japanese abbreviation-style nickname for 基本情報技術者試験? (two kanji words, 4 characters)
= 基本情報
? 「基本情報、受かりました!」

@@exercise Did I pass, and what should I study next?
科目A and 科目B are each scored out of 1000 (by IRT, so scores aren't a simple percentage), and you pass only if **both** reach 600. From practice results by field, estimate each 科目's score as the weighted share of correct answers × 1000, decide pass or fail, and list the two weakest fields (lowest accuracy) to study next, with roughly how many of the 60 科目A (41 technology, 7 management, 12 strategy) or 20 科目B questions they carry.
@@lang python
@@starter
practice = {
    "A": {"基礎理論": (12, 20), "アルゴリズム": (8, 10), "コンピュータシステム": (14, 18), "データベース": (5, 10),
          "ネットワーク": (6, 10), "セキュリティ": (9, 12), "マネジメント": (7, 10), "ストラテジ": (8, 10)},
    "B": {"アルゴリズムとプログラミング": (9, 16), "情報セキュリティ": (3, 4)},
}
WEIGHTS = {"A": {"基礎理論": 7, "アルゴリズム": 5, "コンピュータシステム": 11, "データベース": 5,
                 "ネットワーク": 5, "セキュリティ": 8, "マネジメント": 7, "ストラテジ": 12},
           "B": {"アルゴリズムとプログラミング": 16, "情報セキュリティ": 4}}

def estimate(subject):
    raise NotImplementedError

def weakest(subject, n=2):
    raise NotImplementedError

for s in ("A", "B"):
    score = estimate(s)
    print(f"科目{s}: about {score} / 1000 -> {'pass' if score >= 600 else 'below 600'}")
    for field, acc, weight in weakest(s):
        print(f"    study {field}: {acc:.0%} correct, about {weight} questions on the exam")
result = all(estimate(s) >= 600 for s in ("A", "B"))
print("overall:", "pass" if result else "not yet: both 科目 need 600")
@@expected
科目A: about 713 / 1000 -> pass
    study データベース: 50% correct, about 5 questions on the exam
    study 基礎理論: 60% correct, about 7 questions on the exam
科目B: about 600 / 1000 -> pass
    study アルゴリズムとプログラミング: 56% correct, about 16 questions on the exam
    study 情報セキュリティ: 75% correct, about 4 questions on the exam
overall: pass
@@requires
def estimate
def weakest
WEIGHTS[subject]
@@hint
For each field, accuracy is `correct / total`. The estimate is `round(sum(acc * weight) / sum(weights) * 1000)` with the weights from `WEIGHTS[subject]`. For the weakest fields, sort by accuracy (lowest first) and take the first `n`, returning `(field, accuracy, weight)`.
@@solution
practice = {
    "A": {"基礎理論": (12, 20), "アルゴリズム": (8, 10), "コンピュータシステム": (14, 18), "データベース": (5, 10),
          "ネットワーク": (6, 10), "セキュリティ": (9, 12), "マネジメント": (7, 10), "ストラテジ": (8, 10)},
    "B": {"アルゴリズムとプログラミング": (9, 16), "情報セキュリティ": (3, 4)},
}
WEIGHTS = {"A": {"基礎理論": 7, "アルゴリズム": 5, "コンピュータシステム": 11, "データベース": 5,
                 "ネットワーク": 5, "セキュリティ": 8, "マネジメント": 7, "ストラテジ": 12},
           "B": {"アルゴリズムとプログラミング": 16, "情報セキュリティ": 4}}

def accuracy(subject):
    return {field: c / t for field, (c, t) in practice[subject].items()}

def estimate(subject):
    weights = WEIGHTS[subject]
    acc = accuracy(subject)
    return round(sum(acc[f] * weights[f] for f in weights) / sum(weights.values()) * 1000)

def weakest(subject, n=2):
    acc = accuracy(subject)
    fields = sorted(acc, key=lambda f: acc[f])[:n]
    return [(f, acc[f], WEIGHTS[subject][f]) for f in fields]

for s in ("A", "B"):
    score = estimate(s)
    print(f"科目{s}: about {score} / 1000 -> {'pass' if score >= 600 else 'below 600'}")
    for field, acc, weight in weakest(s):
        print(f"    study {field}: {acc:.0%} correct, about {weight} questions on the exam")
result = all(estimate(s) >= 600 for s in ("A", "B"))
print("overall:", "pass" if result else "not yet: both 科目 need 600")

@@lesson jc0-market The IT Job Market in Japan
@@text
Japan has a long-running **shortage of IT workers**. Government studies have projected a gap of hundreds of thousands of engineers by 2030, and companies increasingly hire from abroad. A foreign IT generalist with real skills, a few certifications and working Japanese is employable. The goal of this whole app, a job that pays clearly above minimum wage, is very reachable.

> Figures on this page are rough and change every year (minimum wage rises every October). Use them to calibrate, then check current job ads.

## Types of employers

| Type | 日本語 | What it's like |
|---|---|---|
| **SIer** (system integrator) | エスアイヤー | builds and runs systems for client companies. Big names (NTT Data, Fujitsu, NEC, Hitachi) sit at the top of a **multi-layer subcontracting pyramid** (多重下請け構造): 元請け (prime) → 二次請け → 三次請け... Pay and conditions get worse further down |
| **SES** (System Engineering Service) | エスイーエス | sends engineers to work at client sites, billed by the hour/month (準委任 contracts). The most common entry point for foreigners and career changers. Quality varies enormously: some are great training grounds, some just rent people out |
| **In-house product company** | 自社開発 | builds its own product (web services, SaaS, games). Often more modern tools, flexible culture, English-friendly teams in some (Mercari, Rakuten, LINE Yahoo, SmartNews...) |
| **Internal IT** | 社内SE / 情報システム部 (情シス) | runs IT for its own company: help desk, infrastructure, vendors. Stable; good for generalists |
| **Foreign companies** | 外資系 | AWS, Microsoft, Google, banks, consultancies. Higher pay, English, higher expectations |
| **Managed service / data centre** | 運用監視, データセンター | 24/7 monitoring and operations; shift work; an entry door into infrastructure |

## Entry roles for an IT generalist

**ヘルプデスク / テクニカルサポート** (help desk), **社内SE** (internal IT), **運用監視** (monitoring/operations), **インフラエンジニア** (servers, networks, cloud), **ネットワークエンジニア**, **クラウドエンジニア**, **セキュリティ運用 (SOC)**. Infrastructure roles are usually more open to non-native Japanese speakers than roles talking to customers all day.

## Salary (年収): a rough picture

| Level | Typical range (annual, gross) |
|---|---|
| Help desk / operations, entry | ¥3.0M – ¥4.0M |
| Infrastructure / network engineer, a few years | ¥4.0M – ¥6.0M |
| Cloud / SRE / security, experienced | ¥6.0M – ¥9.0M+ |
| 外資系 or senior bilingual | ¥8.0M – ¥15M+ |

Minimum wage (最低賃金) is set **per prefecture** and rises each October. Tokyo's was **¥1,226/hour from October 2025**; a full-time job at that rate is roughly ¥2.4M a year. So even entry IT roles pay well above minimum wage, *if* you read the contract carefully (see みなし残業 in the job-hunting lesson).

What raises your pay: **Japanese level** (N2 → N1 opens far more roles), **cloud skills** (AWS/Azure), **certifications** (CCNA, AWS SAA, 基本情報 and 応用情報), **English** (a real advantage at 外資系 and global teams), and moving from SES to in-house or 外資系 after 2-3 years of experience.

## Location

**Tokyo** has by far the most IT jobs, then Osaka, Nagoya and Fukuoka (a growing startup scene, lower living costs). Remote work became common after 2020, but many companies, especially SIers and SES, still expect office or client-site attendance.

@@quiz
What is SES in the Japanese IT industry?
* Companies that send engineers to work at client sites under service contracts
- A Japanese certification
- A cloud provider
- An email standard
? Common entry point; quality varies, so research the company.

@@quiz
What does 多重下請け構造 describe?
* The multi-layer subcontracting pyramid below large SIers
- A Japanese network protocol
- A database design
- The JLPT levels
? Lower layers usually mean lower pay and less say.

@@quiz
Which role runs IT for its own company (help desk, infrastructure, vendor management)?
* 社内SE
- SES engineer
- 元請け
- 外資系 consultant
? Also called 情報システム部 (情シス).

@@quiz
How is minimum wage set in Japan?
* Per prefecture, revised every year (usually in October)
- Once, nationally, for ever
- By each company
- Only for part-time jobs
? Check the current figure for the prefecture where you'll work.

@@quiz
Which of these most typically raises an IT engineer's salary in Japan?
* Better Japanese, cloud skills and a move to in-house or 外資系 companies
- Staying many years at the bottom of a subcontracting chain
- Avoiding certifications
- Only working night shifts
? Skills and the type of employer matter most.

@@flashcards
SIer :: System integrator building systems for clients
SES :: Engineers placed at client sites under service contracts
多重下請け構造 :: Multi-layer subcontracting pyramid
元請け :: Prime contractor
自社開発 :: In-house product development
社内SE / 情シス :: Internal IT / information systems department
外資系 :: Foreign-owned company
運用監視 :: Operations and monitoring
インフラエンジニア :: Infrastructure engineer
年収 :: Annual income (gross)
最低賃金 :: Minimum wage (per prefecture)
人材不足 :: Shortage of workers

@@drill
What's the Japanese term for a company's internal IT engineer? (社内...)
= 社内SE
= shanai SE
= 社内se
? 社内 = inside the company, SE = system engineer.

@@exercise Sort employers by type
Classify company descriptions into the employer types from this lesson using keyword rules (the first matching rule wins), then print the type with a note on what to ask in the interview.
@@lang python
@@starter
RULES = [
    ("外資系", ["global headquarters", "外資", "us-based", "european parent"]),
    ("SES", ["client site", "客先常駐", "準委任", "engineers dispatched"]),
    ("SIer", ["system integration", "受託開発", "prime contractor", "元請け"]),
    ("社内SE", ["社内se", "internal it", "corporate it", "情報システム部"]),
    ("in-house (自社開発)", ["our own product", "自社サービス", "自社開発", "saas"]),
]
ASK = {
    "外資系": "reporting lines to headquarters, English use, and job security",
    "SES": "which clients, how projects are assigned, and the commercial layer (商流)",
    "SIer": "where the company sits in the subcontracting chain and what you'll build",
    "社内SE": "the size of the IT team and how much is outsourced",
    "in-house (自社開発)": "the tech stack, release cycle and product roadmap",
}
companies = {
    "Company A": "We place engineers at client sites (客先常駐) on 準委任 contracts.",
    "Company B": "A Japanese subsidiary of a US-based cloud company, reporting to global headquarters.",
    "Company C": "We run our own product: a SaaS for restaurant reservations, built 自社開発.",
    "Company D": "Prime contractor (元請け) for government system integration projects.",
    "Company E": "Manufacturer looking for a 社内SE to join the 情報システム部.",
    "Company F": "Trading company with a big warehouse.",
}

def employer_type(description):
    raise NotImplementedError

for name, desc in companies.items():
    kind = employer_type(desc)
    print(f"{name}: {kind or 'unclear: ask what the IT team does'}" + (f" -> ask about {ASK[kind]}" if kind else ""))
@@expected
Company A: SES -> ask about which clients, how projects are assigned, and the commercial layer (商流)
Company B: 外資系 -> ask about reporting lines to headquarters, English use, and job security
Company C: in-house (自社開発) -> ask about the tech stack, release cycle and product roadmap
Company D: SIer -> ask about where the company sits in the subcontracting chain and what you'll build
Company E: 社内SE -> ask about the size of the IT team and how much is outsourced
Company F: unclear: ask what the IT team does
@@requires
def employer_type
.lower()
@@hint
Lower-case the description (Japanese text is unaffected), loop over `RULES`, and return the first type where any keyword is in the text. Return `None` if nothing matches.
@@solution
RULES = [
    ("外資系", ["global headquarters", "外資", "us-based", "european parent"]),
    ("SES", ["client site", "客先常駐", "準委任", "engineers dispatched"]),
    ("SIer", ["system integration", "受託開発", "prime contractor", "元請け"]),
    ("社内SE", ["社内se", "internal it", "corporate it", "情報システム部"]),
    ("in-house (自社開発)", ["our own product", "自社サービス", "自社開発", "saas"]),
]
ASK = {
    "外資系": "reporting lines to headquarters, English use, and job security",
    "SES": "which clients, how projects are assigned, and the commercial layer (商流)",
    "SIer": "where the company sits in the subcontracting chain and what you'll build",
    "社内SE": "the size of the IT team and how much is outsourced",
    "in-house (自社開発)": "the tech stack, release cycle and product roadmap",
}
companies = {
    "Company A": "We place engineers at client sites (客先常駐) on 準委任 contracts.",
    "Company B": "A Japanese subsidiary of a US-based cloud company, reporting to global headquarters.",
    "Company C": "We run our own product: a SaaS for restaurant reservations, built 自社開発.",
    "Company D": "Prime contractor (元請け) for government system integration projects.",
    "Company E": "Manufacturer looking for a 社内SE to join the 情報システム部.",
    "Company F": "Trading company with a big warehouse.",
}

def employer_type(description):
    text = description.lower()
    for kind, keywords in RULES:
        if any(k in text for k in keywords):
            return kind
    return None

for name, desc in companies.items():
    kind = employer_type(desc)
    print(f"{name}: {kind or 'unclear: ask what the IT team does'}" + (f" -> ask about {ASK[kind]}" if kind else ""))

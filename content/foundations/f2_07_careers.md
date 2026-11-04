@@lesson f2-careers IT Careers, Certifications and Professional Skills
@@text
"IT" covers dozens of jobs that have little in common day to day. Knowing the map helps you choose what to learn next in this app, which certifications are worth their exam fees, and how to talk about your skills in interviews, including in Japan.

## The main career paths

| Path | Typical first job | Later roles | Tracks in this app |
|---|---|---|---|
| **Support** | help desk / service desk (L1) | desktop engineer, IT manager | A+, ITIL |
| **Systems** | junior sysadmin | Windows/Linux engineer, systems architect | Linux+, PowerShell, Bash |
| **Networking** | NOC technician | network engineer, network architect | Network+, CCNA, CCNP |
| **Security** | SOC analyst (tier 1) | security engineer, incident responder, CISO | Security+, CISSP |
| **Cloud and DevOps** | junior cloud/DevOps engineer | SRE, platform engineer, cloud architect | AWS, Azure, GCP, Terraform, Kubernetes, DevOps |
| **Development** | junior developer | senior/lead engineer, architect | the language tracks |
| **Data** | data/BI analyst | data engineer, database administrator | SQL, Python |
| **Project and management** | coordinator, PMO | project manager, IT manager | PMP, ITIL |

Many people start in support: it teaches how real users, real networks and real outages behave, and it's the most common entry point without a degree. Moving on usually takes a year or two plus visible learning (certifications, a home lab, scripts that saved your team time).

## Certifications: which and when

Certifications prove a baseline to employers who can't test you themselves, and they're often required by HR filters, government contracts and visa points systems.

- **Entry**: CompTIA **A+** (support), **Network+**, **Security+** (also a US DoD baseline), **Linux+**. Vendor-neutral and widely recognised.
- **Vendor associate**: **CCNA** (Cisco networking), **AWS Solutions Architect Associate**, **Azure Administrator (AZ-104)**, Google **Associate Cloud Engineer**, **CKA** (Kubernetes), **Terraform Associate**.
- **Professional/expert**: CCNP, AWS Professional, Google Professional Cloud Architect, **CISSP** (needs 5 years' experience), **PMP**.
- **Process**: **ITIL 4 Foundation** (valued in service management and in Japan's large enterprises).
- **Japan**: IPA's national exams: **基本情報技術者 (FE)**, then 応用情報技術者 (AP), and specialist exams like 情報処理安全確保支援士 (security). They're cheap, respected by Japanese employers, and count for visa points.

Rules of thumb: pick certifications that match a job you want, take them in a sensible order, and pair each with something practical (a lab, a project, a script on GitHub). A certification without hands-on skill shows quickly in a technical interview.

## Skills that decide careers

Technical skill gets you hired; these get you trusted and promoted:

- **Communication**: explaining technical things without jargon, writing clear emails and ticket notes, saying "I don't know yet, I'll find out by 3 pm".
- **Customer service**: patience with stressed users, setting expectations, following up.
- **Troubleshooting method** over memorised fixes.
- **Documentation habits**.
- **Time management**: juggling tickets by priority, not by who shouts loudest.
- **Learning to learn**: technology changes every few years. Reading documentation, release notes and error messages carefully is a superpower.
- **Ethics**: admins can read email, see salaries and reset anyone's password. Access is for the task at hand, never curiosity. Breaches of trust end careers.

## Building evidence

- A **home lab** (module 2's build) with notes.
- A **GitHub** profile with scripts and small tools, each with a README explaining the problem it solves.
- A **portfolio** or blog of things you've built or fixed.
- **Contributions**: answering questions in forums, documentation fixes to open-source projects.
- For a CV: describe **results**, not duties: "Automated new-starter setup with PowerShell, cutting it from 2 hours to 10 minutes" beats "Responsible for user accounts".

## Working in IT in Japan (overview)

The Japan Career track covers this in depth. In short:

- Demand for IT workers is high and rising; many companies hire foreign engineers, especially in Tokyo, Osaka and Fukuoka.
- **Japanese ability** widens options enormously: JLPT **N2** is a common requirement for support and in-house IT roles; English-only roles exist mainly in development, cloud and at international companies.
- The **Engineer/Specialist in Humanities/International Services** visa usually needs a relevant degree or 10 years' experience (3 for some language/international roles); IT certifications on the Ministry of Justice's list can substitute in some cases.
- Japanese CVs: the **履歴書** (rirekisho, a standard form) and the **職務経歴書** (shokumu keirekisho, a career history describing projects and results).
- Many IT jobs go through **SIers** (system integrators) and staffing agencies; in-house (社内SE) roles are more stable and often more relaxed.

@@quiz
What's the most common entry point into IT without a degree?
* Help desk / service desk support
- CISO
- Cloud architect
- Network architect
? It builds broad practical experience.

@@quiz
Which certification usually requires about 5 years of security work experience?
- Security+
- A+
* CISSP
- Network+
? It's a senior certification.

@@quiz
Which CV line is stronger?
- "Responsible for user accounts"
* "Automated new-starter setup with PowerShell, cutting it from 2 hours to 10 minutes"
- "Good with computers"
- "Familiar with Windows"
? Show results with numbers.

@@quiz
What is a 職務経歴書 (shokumu keirekisho)?
- A visa application
* A Japanese career history document describing projects and achievements
- The JLPT certificate
- An employment contract
? It goes with the standard 履歴書 form.

@@flashcards
A+ :: entry support certification
Network+ / Security+ :: entry networking / security certifications
CCNA :: Cisco associate networking
AZ-104 :: Azure Administrator
CKA :: Certified Kubernetes Administrator
CISSP :: senior security certification (5 years)
ITIL 4 Foundation :: service management basics
基本情報技術者 (FE) :: Japan's national entry IT exam
SOC analyst :: entry security role
SRE :: Site Reliability Engineer
履歴書 :: standard Japanese résumé form
職務経歴書 :: Japanese career history
JLPT N2 :: common Japanese requirement for IT support roles in Japan
社内SE :: in-house IT engineer in Japan

@@drill
What is the usual Japanese level asked for in IT support jobs in Japan? (JLPT level)
= N2
? N1 is the highest.

@@drill
Name Japan's national entry-level IT exam (English abbreviation).
= FE
= IPA FE
? 基本情報技術者試験 (Fundamental Information Technology Engineer).

@@exercise A certification roadmap
Each certification has prerequisites (certifications that should come first). Write `roadmap(target, prereqs)` that returns the full ordered list of certifications to take to reach `target`, prerequisites first, without duplicates (a depth-first walk that appends a cert after its prerequisites). Print the roadmaps for CCNP, CISSP and the AWS Solutions Architect Professional.
@@lang python
@@starter
prereqs = {
    "A+": [],
    "Network+": ["A+"],
    "Security+": ["Network+"],
    "CCNA": ["Network+"],
    "CCNP": ["CCNA"],
    "CISSP": ["Security+"],
    "Linux+": ["A+"],
    "AWS SAA": ["Network+", "Linux+"],
    "AWS SAP": ["AWS SAA"],
}

def roadmap(target, prereqs):
    raise NotImplementedError
@@expected
CCNP: A+ -> Network+ -> CCNA -> CCNP
CISSP: A+ -> Network+ -> Security+ -> CISSP
AWS SAP: A+ -> Network+ -> Linux+ -> AWS SAA -> AWS SAP
@@requires
def roadmap
def visit(
@@hint
Inside `roadmap`, define `order = []` and a nested `def visit(cert):` that returns if `cert in order`, otherwise visits each prerequisite and then appends `cert`. Call `visit(target)` and return `order`.
@@solution
prereqs = {
    "A+": [],
    "Network+": ["A+"],
    "Security+": ["Network+"],
    "CCNA": ["Network+"],
    "CCNP": ["CCNA"],
    "CISSP": ["Security+"],
    "Linux+": ["A+"],
    "AWS SAA": ["Network+", "Linux+"],
    "AWS SAP": ["AWS SAA"],
}

def roadmap(target, prereqs):
    order = []

    def visit(cert):
        if cert in order:
            return
        for before in prereqs[cert]:
            visit(before)
        order.append(cert)

    visit(target)
    return order

for target in ["CCNP", "CISSP", "AWS SAP"]:
    print(f"{target}: {' -> '.join(roadmap(target, prereqs))}")

@@exercise Turn duties into results
CV bullet points are stronger with an action verb and a measured result. Write `improve(duty, verb, before, after, unit)` that returns `"<Verb> <duty>, cutting it from <before> to <after> <unit> (<p>% less)"` where p is the rounded percentage reduction. Print the improved bullets for the three examples.
@@lang python
@@starter
duties = [
    ("new-starter account setup with PowerShell", "automated", 120, 10, "minutes"),
    ("the laptop imaging process with Autopilot", "rebuilt", 4, 0.5, "hours"),
    ("password reset tickets with a self-service portal", "reduced", 300, 90, "tickets per month"),
]

def improve(duty, verb, before, after, unit):
    raise NotImplementedError
@@expected
- Automated new-starter account setup with PowerShell, cutting it from 120 to 10 minutes (92% less)
- Rebuilt the laptop imaging process with Autopilot, cutting it from 4 to 0.5 hours (88% less)
- Reduced password reset tickets with a self-service portal, cutting it from 300 to 90 tickets per month (70% less)
@@requires
def improve
.capitalize()
round(
@@hint
`p = round((before - after) / before * 100)`. Use `:g` to print 0.5 rather than 0.50 and 120 rather than 120.0.
@@solution
duties = [
    ("new-starter account setup with PowerShell", "automated", 120, 10, "minutes"),
    ("the laptop imaging process with Autopilot", "rebuilt", 4, 0.5, "hours"),
    ("password reset tickets with a self-service portal", "reduced", 300, 90, "tickets per month"),
]

def improve(duty, verb, before, after, unit):
    p = round((before - after) / before * 100)
    return f"{verb.capitalize()} {duty}, cutting it from {before:g} to {after:g} {unit} ({p}% less)"

for duty in duties:
    print("-", improve(*duty))

@@lesson f3-software How Software Is Built, Shipped and Licensed
@@text
Even if you never become a developer, you'll install, update, licence, troubleshoot and support software every day, and you'll work next to developers. Knowing how software goes from an idea to a laptop explains release notes, version numbers, why "just patch it" isn't always simple, and why the licence agreement matters to the company's lawyers.

## From source code to a running program

Programmers write **source code** in a programming language. Then:

- **Compiled languages** (C, C++, Go, Rust, C# to some degree) are translated by a **compiler** into machine code for a specific CPU and OS before shipping. The result is an executable (`.exe` on Windows, an ELF binary on Linux).
- **Interpreted / scripting languages** (Python, PowerShell, Bash, JavaScript) are run by an **interpreter** or runtime that reads the code when it runs. The runtime must be installed on the machine.
- **Bytecode** languages (Java, C#, Kotlin) compile to an intermediate form run by a virtual machine (the JVM, .NET runtime). "This app needs .NET 8 Desktop Runtime" or "Java 17" is that runtime.

Programs also rely on **libraries** (shared code): DLLs on Windows, `.so` files on Linux, packages from npm, PyPI or NuGet. A missing or wrong-version library is behind many "the program won't start" tickets ("VCRUNTIME140.dll was not found" means the Visual C++ Redistributable isn't installed).

## The software development life cycle (SDLC)

1. **Requirements**: what problem, for whom, what must it do?
2. **Design**: architecture, data, interfaces.
3. **Build**: write the code, in small reviewed changes.
4. **Test**: automated tests (unit, integration, end-to-end) plus human testing.
5. **Release / deploy** to users.
6. **Operate and maintain**: monitor, fix bugs, patch security issues, add features.

Two broad styles:

- **Waterfall**: each phase completes before the next. Suits fixed, well-understood requirements (and some government and Japanese SIer projects, with detailed design documents 設計書).
- **Agile** (Scrum, Kanban): short iterations (**sprints** of 1–4 weeks), working software early, frequent feedback, changing priorities welcome. Most product teams work this way.

**DevOps** joins development and operations: automated builds, tests and deployments (**CI/CD**), infrastructure as code, monitoring, and shared responsibility for running the software. The DevOps track covers it in depth.

## Version control

**Git** records every change to the code with who, when and why. Teams work on **branches**, propose changes in **pull requests** that colleagues review, and merge into the main branch. GitHub, GitLab, Azure DevOps and Bitbucket host Git repositories. IT teams use Git too: for scripts, configuration and documentation.

## Versions and releases

**Semantic versioning** (`MAJOR.MINOR.PATCH`, e.g. `2.14.3`):

- **PATCH** (2.14.**3** → 2.14.**4**): bug fixes, safe to apply.
- **MINOR** (2.**14** → 2.**15**): new features, backwards compatible.
- **MAJOR** (**2** → **3**): breaking changes; read the notes, test first.

Other terms: **alpha/beta/RC** (release candidate) for pre-releases; **LTS** (long-term support) versions get security fixes for years (Ubuntu 24.04 LTS, Node.js LTS), while others are supported for months. **End of life (EOL)** means no more security fixes: running EOL software (Windows 10 after October 2025 without paid extended updates, old PHP versions) is a major security risk and an audit finding.

## Installing and managing software in organisations

- Windows: **MSI** packages and installers that support **silent** flags (`/quiet`, `/S`), deployed by Intune, Configuration Manager (SCCM) or a package manager like **winget** (`winget install Microsoft.PowerToys`).
- macOS: `.pkg`, `.dmg`, and MDM deployment; Homebrew for developers.
- Linux: package managers (`apt`, `dnf`), plus Flatpak and Snap.
- A **software catalogue / company portal** lets staff install approved software themselves without admin rights.
- **Patch management**: test updates on a pilot group, then roll out in rings; prioritise security updates by severity (**CVSS** score) and whether they're actively exploited.

## Software as a service (SaaS)

Much software is now rented and runs in the vendor's cloud: Microsoft 365, Salesforce, Slack, Zoom, ServiceNow. IT's job shifts from installing it to **identity** (SSO, provisioning), **licences**, **data protection**, configuration and vendor management. **Shadow IT**, staff signing up for SaaS tools without IT, creates data and security risks; offering good approved tools is the best cure.

## Licensing

| Model | Meaning |
|---|---|
| **Proprietary, per user / per device** | you pay for each user or computer (Microsoft 365 E3 per user) |
| **Subscription** | monthly/yearly; stops working if unpaid |
| **Perpetual** | buy once, use forever (support/updates may cost extra) |
| **Per core / per server** | servers and databases (SQL Server, Oracle) |
| **Concurrent / floating** | a pool shared by whoever is using it now |
| **Open source** | source code available under a licence: **permissive** (MIT, Apache, BSD: use almost freely, keep the notice) or **copyleft** (GPL: if you distribute modified versions, you must share the source under the GPL too) |
| **Freeware** | free to use, closed source; often "free for personal use only" |

Being **under-licensed** risks audits and fines (vendors such as Microsoft, Oracle, Adobe audit customers); being **over-licensed** wastes money. **Software asset management (SAM)** tracks installs against licences. "Free for personal use" software installed on company PCs is a common audit finding.

@@quiz
A program fails with "VCRUNTIME140.dll was not found". What's the usual fix?
* Install the Microsoft Visual C++ Redistributable the program needs
- Reinstall Windows
- Delete the program's folder
- Update the graphics driver
? A missing shared runtime library.

@@quiz
An app goes from version 4.2.7 to 5.0.0. What does semantic versioning suggest?
- Only bug fixes
- New compatible features
* Breaking changes; read the release notes and test before upgrading
- Nothing; numbers are random
? MAJOR version bump.

@@quiz
What does "end of life" mean for software?
* The vendor no longer provides security updates or support
- The software stops working immediately
- It becomes free
- It's in beta
? Running EOL software is a security risk.

@@quiz
A company modifies GPL-licensed software and distributes it to customers. What must it do?
- Nothing
- Pay a fee to the original author
* Provide the source code of the modified version under the GPL
- Rename it
? GPL is copyleft.

@@flashcards
compiler :: translates source code to machine code before running
interpreter :: runs source code directly
runtime :: software an app needs to run (.NET, Java, Python)
library / DLL :: shared code used by programs
SDLC :: requirements, design, build, test, release, maintain
Agile / Scrum :: short iterations (sprints)
CI/CD :: automated build, test and deployment
Git :: version control
pull request :: proposed change for review
semantic versioning :: MAJOR.MINOR.PATCH
LTS :: long-term support release
EOL :: end of life: no more fixes
silent install :: `/quiet` or `/S` installer flags
winget :: Windows package manager
CVSS :: vulnerability severity score
SaaS :: software rented as a cloud service
shadow IT :: unapproved tools used by staff
MIT / Apache :: permissive open-source licences
GPL :: copyleft licence
SAM :: software asset management

@@drill
In version 3.8.1, which number is the MINOR version?
= 8
? MAJOR.MINOR.PATCH.

@@drill
Which Windows command-line tool installs software from Microsoft's package repository?
= winget
? `winget install <id>`.

@@exercise Compare versions and plan upgrades
Write `parse(v)` turning `"2.14.3"` into a tuple of integers, and `upgrade_type(current, latest)` returning `"none"`, `"patch"`, `"minor"` or `"major"`. For each installed application print the upgrade type, and flag major upgrades with `"test first"`. Compare tuples, not strings: as text, `"2.9.0" > "2.14.0"`, which is wrong.
@@lang python
@@starter
apps = [("7-Zip", "24.7.0", "24.8.0"), ("Firefox", "126.0.1", "128.0.0"), ("Python", "3.12.4", "3.12.4"),
        ("Node.js", "18.20.3", "22.4.0"), ("Agent", "2.9.0", "2.14.0")]

def parse(v):
    raise NotImplementedError

def upgrade_type(current, latest):
    raise NotImplementedError
@@expected
7-Zip 24.7.0 -> 24.8.0: minor
Firefox 126.0.1 -> 128.0.0: major  test first
Python 3.12.4 -> 3.12.4: none
Node.js 18.20.3 -> 22.4.0: major  test first
Agent 2.9.0 -> 2.14.0: minor
String comparison says 2.9.0 > 2.14.0: True
@@requires
def parse
def upgrade_type
tuple(
@@hint
`tuple(int(part) for part in v.split("."))`. Then compare the parts in order: if the majors differ it's a major upgrade, and so on.
@@solution
apps = [("7-Zip", "24.7.0", "24.8.0"), ("Firefox", "126.0.1", "128.0.0"), ("Python", "3.12.4", "3.12.4"),
        ("Node.js", "18.20.3", "22.4.0"), ("Agent", "2.9.0", "2.14.0")]

def parse(v):
    return tuple(int(part) for part in v.split("."))

def upgrade_type(current, latest):
    c, l = parse(current), parse(latest)
    if l <= c:
        return "none"
    if l[0] != c[0]:
        return "major"
    if l[1] != c[1]:
        return "minor"
    return "patch"

for name, current, latest in apps:
    kind = upgrade_type(current, latest)
    note = "  test first" if kind == "major" else ""
    print(f"{name} {current} -> {latest}: {kind}{note}")
print("String comparison says 2.9.0 > 2.14.0:", "2.9.0" > "2.14.0")

@@exercise A licence compliance check
Compare installations against purchased licences. For each product print installed vs owned and the status: `"under-licensed by N"` (a compliance risk), `"N spare"` or `"exact"`. Products installed but with no licences at all are `"NO LICENCE"`. Finally print the total cost of buying the missing licences using the price list.
@@lang python
@@starter
installs = ["Visio", "Visio", "Project", "Acrobat Pro", "Acrobat Pro", "Acrobat Pro", "Visio", "WinRAR", "Acrobat Pro"]
owned = {"Visio": 2, "Project": 3, "Acrobat Pro": 4}
price = {"Visio": 18000, "Project": 36000, "Acrobat Pro": 26000, "WinRAR": 4000}
@@expected
Acrobat Pro: installed 4, owned 4: exact
Project: installed 1, owned 3: 2 spare
Visio: installed 3, owned 2: under-licensed by 1
WinRAR: installed 1, owned 0: NO LICENCE
Cost to become compliant: ¥22,000
@@requires
Counter(
NO LICENCE
@@hint
`counts = Counter(installs)`. Loop over `sorted(set(counts) | set(owned))`. The missing count is `max(0, installed - owned.get(name, 0))`.
@@solution
from collections import Counter

installs = ["Visio", "Visio", "Project", "Acrobat Pro", "Acrobat Pro", "Acrobat Pro", "Visio", "WinRAR", "Acrobat Pro"]
owned = {"Visio": 2, "Project": 3, "Acrobat Pro": 4}
price = {"Visio": 18000, "Project": 36000, "Acrobat Pro": 26000, "WinRAR": 4000}

counts = Counter(installs)
cost = 0
for name in sorted(set(counts) | set(owned)):
    have, used = owned.get(name, 0), counts.get(name, 0)
    missing = max(0, used - have)
    cost += missing * price[name]
    if have == 0:
        status = "NO LICENCE"
    elif used > have:
        status = f"under-licensed by {missing}"
    elif used < have:
        status = f"{have - used} spare"
    else:
        status = "exact"
    print(f"{name}: installed {used}, owned {have}: {status}")
print(f"Cost to become compliant: ¥{cost:,}")

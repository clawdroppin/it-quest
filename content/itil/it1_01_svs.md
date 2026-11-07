@@lesson it1-svs The Service Value System, Principles and Dimensions
@@text
## The four dimensions of service management

Every service, and every practice, must consider all four. Neglect one and things break:

1. **Organizations and people**: structure, roles, skills, culture.
2. **Information and technology**: the information managed, and the tools and tech used.
3. **Partners and suppliers**: other organizations involved (cloud providers, outsourcers, SIers).
4. **Value streams and processes**: how the parts work together to create value.

Outside forces (**PESTLE**: political, economic, social, technological, legal, environmental) affect all of them.

## The service value system (SVS)

**Opportunity / demand** goes in, **value** comes out. In between:

- **Guiding principles** (below)
- **Governance**: direction and control (evaluate, direct, monitor)
- **Service value chain**: the operating model
- **Practices**: sets of organizational resources for work (34 in ITIL 4)
- **Continual improvement**: at every level

## The seven guiding principles (learn them word for word)

| Principle | In plain words |
|---|---|
| **Focus on value** | everything should link back to value for stakeholders |
| **Start where you are** | don't throw everything away; measure and reuse what works |
| **Progress iteratively with feedback** | small steps, learn from each |
| **Collaborate and promote visibility** | work across teams; make work and data visible |
| **Think and work holistically** | no service or part works in isolation |
| **Keep it simple and practical** | if it adds no value, remove it; outcome-based thinking |
| **Optimize and automate** | optimize first, then automate; humans where they add value |

## The service value chain: six activities

| Activity | Purpose |
|---|---|
| **Plan** | shared understanding of vision, status and direction |
| **Improve** | continual improvement of products, services and practices |
| **Engage** | understand stakeholder needs, transparency, good relationships |
| **Design and transition** | services meet expectations for quality, cost and time to market |
| **Obtain/build** | service components are available when and where needed |
| **Deliver and support** | services delivered and supported to agreed specifications |

**Value streams** combine these activities in different orders for a specific scenario (e.g. "a user reports a fault": engage → deliver and support → improve).

## The continual improvement model

1. **What is the vision?** 2. **Where are we now?** (baseline) 3. **Where do we want to be?** (measurable targets) 4. **How do we get there?** 5. **Take action** 6. **Did we get there?** 7. **How do we keep the momentum going?**

> The guiding principles in ITIL 4 come from earlier ideas you'll also hear about: Lean, Agile, DevOps. ITIL 4 explicitly works *with* them, not against them.

@@quiz
Which is one of the four dimensions of service management?
* Partners and suppliers
- Guiding principles
- Continual improvement
- Governance
? The four: organizations and people; information and technology; partners and suppliers; value streams and processes.

@@quiz
A team wants to replace the entire ticketing process from scratch, although parts work well. Which principle warns against this?
* Start where you are
- Optimize and automate
- Focus on value
- Think and work holistically
? Assess what exists and reuse what works.

@@quiz
Which principle says to optimize a process BEFORE automating it?
* Optimize and automate
- Keep it simple and practical
- Progress iteratively with feedback
- Collaborate and promote visibility
? Automating a bad process just makes bad results faster.

@@quiz
Which value chain activity ensures stakeholder needs are understood and relationships are good?
* Engage
- Plan
- Obtain/build
- Improve
? Engage is the contact point with stakeholders.

@@quiz
Which value chain activity makes sure service components are available when and where needed?
* Obtain/build
- Design and transition
- Deliver and support
- Plan
? Including buying, building and configuring components.

@@quiz
In the continual improvement model, what comes right after "What is the vision?"
* Where are we now?
- Take action
- Did we get there?
- How do we get there?
? You need a baseline before setting targets.

@@quiz
Which component of the SVS provides direction and control?
* Governance
- Practices
- Value chain
- Opportunity
? Evaluate, direct, monitor.

@@quiz
A plan is broken into small pieces with feedback after each one. Which principle is this?
* Progress iteratively with feedback
- Start where you are
- Think and work holistically
- Focus on value
? Smaller steps, earlier learning.

@@flashcards
four dimensions :: Organizations & people; information & technology; partners & suppliers; value streams & processes
PESTLE :: Political, economic, social, technological, legal, environmental
SVS :: Service value system: opportunity/demand → value
SVS components :: Guiding principles, governance, value chain, practices, continual improvement
7 guiding principles :: Focus on value; start where you are; progress iteratively with feedback; collaborate and promote visibility; think and work holistically; keep it simple and practical; optimize and automate
value chain activities :: Plan, improve, engage, design and transition, obtain/build, deliver and support
value stream :: Series of steps to create value for a specific scenario
continual improvement model :: Vision → now → target → how → act → check → momentum

@@drill
How many guiding principles does ITIL 4 have? (a number)
= 7
? Learn the exact wording; the exam uses it.

@@exercise Which guiding principle?
Match each situation to the guiding principle it applies, using keyword rules (the first rule whose keywords appear wins), and report any situation no rule matches so a person can decide.
@@lang python
@@starter
RULES = [
    ("Optimize and automate", ["automate", "script", "robot"]),
    ("Start where you are", ["existing", "reuse", "from scratch"]),
    ("Progress iteratively with feedback", ["small steps", "pilot", "feedback"]),
    ("Collaborate and promote visibility", ["share", "dashboard", "involve"]),
    ("Think and work holistically", ["end to end", "all four dimensions", "whole"]),
    ("Keep it simple and practical", ["remove", "fewer", "simplify"]),
    ("Focus on value", ["customer", "value", "outcome"]),
]

situations = [
    "Before replacing the tool from scratch, check which existing workflows work well",
    "Run a two-week pilot of the new form and gather feedback",
    "Simplify the change form: remove fields nobody uses",
    "Put the backlog on a dashboard everyone can see",
    "Automate password resets after streamlining the steps",
    "Ask what outcome the customer actually needs from the report",
    "Order more coffee for the service desk",
]

def principle_for(text):
    raise NotImplementedError

for s in situations:
    print(f"{principle_for(s) or 'no match (ask a person)':<36} {s}")
@@expected
Start where you are                  Before replacing the tool from scratch, check which existing workflows work well
Progress iteratively with feedback   Run a two-week pilot of the new form and gather feedback
Keep it simple and practical         Simplify the change form: remove fields nobody uses
Collaborate and promote visibility   Put the backlog on a dashboard everyone can see
Optimize and automate                Automate password resets after streamlining the steps
Focus on value                       Ask what outcome the customer actually needs from the report
no match (ask a person)              Order more coffee for the service desk
@@requires
def principle_for
.lower()
@@hint
Lower-case the text once, then loop over `RULES` and return the principle of the first rule where `any(k in text for k in keywords)`. Return `None` if nothing matches.
@@solution
RULES = [
    ("Optimize and automate", ["automate", "script", "robot"]),
    ("Start where you are", ["existing", "reuse", "from scratch"]),
    ("Progress iteratively with feedback", ["small steps", "pilot", "feedback"]),
    ("Collaborate and promote visibility", ["share", "dashboard", "involve"]),
    ("Think and work holistically", ["end to end", "all four dimensions", "whole"]),
    ("Keep it simple and practical", ["remove", "fewer", "simplify"]),
    ("Focus on value", ["customer", "value", "outcome"]),
]

situations = [
    "Before replacing the tool from scratch, check which existing workflows work well",
    "Run a two-week pilot of the new form and gather feedback",
    "Simplify the change form: remove fields nobody uses",
    "Put the backlog on a dashboard everyone can see",
    "Automate password resets after streamlining the steps",
    "Ask what outcome the customer actually needs from the report",
    "Order more coffee for the service desk",
]

def principle_for(text):
    text = text.lower()
    for principle, keywords in RULES:
        if any(k in text for k in keywords):
            return principle
    return None

for s in situations:
    print(f"{principle_for(s) or 'no match (ask a person)':<36} {s}")

@@exercise Which of the four dimensions?
Every change touches the four dimensions of service management: **organizations and people**, **information and technology**, **partners and suppliers**, **value streams and processes**. Write `dimensions(task)` that returns every dimension a project task touches, using keywords (`training`, `roles`, `culture` → people; `software`, `data`, `server`, `tool` → technology; `vendor`, `contract`, `outsourc` → partners; `workflow`, `process`, `handoff`, `approval` → value streams).

Print each task with its dimensions, then warn about any dimension the whole plan never touches: the classic reason a tool rollout fails.
@@lang python
@@starter
DIMENSIONS = {
    "organizations and people": ["training", "roles", "culture"],
    "information and technology": ["software", "data", "server", "tool"],
    "partners and suppliers": ["vendor", "contract", "outsourc"],
    "value streams and processes": ["workflow", "process", "handoff", "approval"],
}
plans = {
    "New ticketing tool": [
        "Buy the ticketing software licence",
        "Migrate old ticket data",
        "Install the tool on two servers",
    ],
    "Outsourced night shift": [
        "Sign the contract with the support vendor",
        "Define the handoff between day and night teams",
        "Training for the vendor's agents",
        "Give the vendor read access to the knowledge base data",
    ],
}

def dimensions(task):
    raise NotImplementedError


for plan, tasks in plans.items():
    print(plan + ":")
    touched = set()
    for t in tasks:
        d = dimensions(t)
        touched.update(d)
        print(f"  {t}: {', '.join(d) or 'none'}")
    for d in DIMENSIONS:
        if d not in touched:
            print(f"  warning: nothing covers {d}")
@@expected
New ticketing tool:
  Buy the ticketing software licence: information and technology
  Migrate old ticket data: information and technology
  Install the tool on two servers: information and technology
  warning: nothing covers organizations and people
  warning: nothing covers partners and suppliers
  warning: nothing covers value streams and processes
Outsourced night shift:
  Sign the contract with the support vendor: partners and suppliers
  Define the handoff between day and night teams: value streams and processes
  Training for the vendor's agents: organizations and people, partners and suppliers
  Give the vendor read access to the knowledge base data: information and technology, partners and suppliers
@@requires
def dimensions
any(
@@hint
Return every matching dimension, not just the first: a task can touch several. Keep a set of the dimensions touched by the whole plan and compare it with all four at the end.
@@solution
DIMENSIONS = {
    "organizations and people": ["training", "roles", "culture"],
    "information and technology": ["software", "data", "server", "tool"],
    "partners and suppliers": ["vendor", "contract", "outsourc"],
    "value streams and processes": ["workflow", "process", "handoff", "approval"],
}
plans = {
    "New ticketing tool": [
        "Buy the ticketing software licence",
        "Migrate old ticket data",
        "Install the tool on two servers",
    ],
    "Outsourced night shift": [
        "Sign the contract with the support vendor",
        "Define the handoff between day and night teams",
        "Training for the vendor's agents",
        "Give the vendor read access to the knowledge base data",
    ],
}

def dimensions(task):
    text = task.lower()
    return [d for d, words in DIMENSIONS.items() if any(w in text for w in words)]

for plan, tasks in plans.items():
    print(plan + ":")
    touched = set()
    for t in tasks:
        d = dimensions(t)
        touched.update(d)
        print(f"  {t}: {', '.join(d) or 'none'}")
    for d in DIMENSIONS:
        if d not in touched:
            print(f"  warning: nothing covers {d}")

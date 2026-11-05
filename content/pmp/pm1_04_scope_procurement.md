@@lesson pm1-scope-procurement Scope and Procurement in Depth: WBS, Traceability and Contract Math
@@text
Two areas produce precise, calculable PMP questions: **scope** (does the work breakdown cover everything, and can each requirement be traced?) and **procurement** (which contract type, and what does the buyer pay when costs overrun?). This lesson covers both.

## Scope: requirements to WBS

- **Collect requirements** → the **requirements traceability matrix** (RTM) links each requirement to its business objective, its deliverable (WBS element) and its test. A requirement with no test can't be validated, and a deliverable with no requirement is **gold plating**, which you shouldn't do.
- **WBS**: decomposition of the **total scope** of work into deliverables. The **100% rule**: the children of each element add up to exactly 100% of the parent, with nothing missing and nothing extra. The lowest level is the **work package**, estimated and assigned. Its control accounts and the WBS dictionary describe each element.
- **Scope baseline** = scope statement + WBS + WBS dictionary.
- **Validate scope** (the customer formally accepts deliverables) vs **control quality** (internal correctness, done first).
- **Scope creep**: uncontrolled additions. All changes go through integrated change control.

## Procurement: contract types and risk

| Type | Who carries cost risk | Use when |
|---|---|---|
| **Firm fixed price (FFP)** | seller | scope is well defined |
| **Fixed price incentive fee (FPIF)** | shared, up to a ceiling | defined scope, with an incentive to save |
| **Fixed price with economic price adjustment (FP-EPA)** | seller, adjusted for inflation | multi-year contracts |
| **Cost plus fixed fee (CPFF)** | buyer | scope is uncertain |
| **Cost plus incentive fee (CPIF)** | shared by a ratio | uncertain scope, with an incentive to save |
| **Cost plus award fee (CPAF)** | buyer; fee at the buyer's discretion | subjective performance |
| **Time and materials (T&M)** | buyer (use a not-to-exceed cap) | staff augmentation, small, urgent work |

### Incentive math

- **Share ratio** buyer/seller, e.g. 80/20: the seller gets 20% of any underrun as extra fee, and loses 20% of any overrun.
- **CPIF**: fee = target fee + seller share × (target cost − actual cost). The buyer pays actual cost + fee.
- **FPIF**: price = actual cost + target fee + seller share × (target cost − actual cost), capped at the **ceiling price**.
- **Point of total assumption (PTA)** = target cost + (ceiling price − target price) ÷ buyer share. Above the PTA, every extra dollar of cost comes out of the seller's profit.

## Make-or-buy and source selection

Make-or-buy compares the total cost of doing the work in-house against buying it (including management overhead and risk). Source selection weighs criteria such as price, technical approach, past performance and capacity, usually with a **weighted scoring** model, and may screen out bidders who miss mandatory criteria.

@@quiz
A deliverable appears in the WBS but traces to no requirement. What is it?
- A work package
* Gold plating, unless a requirement is missing
- A control account
- A risk response
? The RTM exposes this.

@@quiz
What does the 100% rule say?
* Each WBS level's children add up to exactly all the work of their parent
- Every task must be 100% complete before the next starts
- The budget must be fully spent
- All stakeholders must approve the WBS
? Nothing missing, nothing extra.

@@quiz
Scope is unclear and the buyer wants to pay actual costs while rewarding the seller for savings. Which contract fits?
- Firm fixed price
* Cost plus incentive fee
- Time and materials without a cap
- Fixed price with economic price adjustment
? Cost reimbursable with a shared incentive.

@@quiz
Which process involves the customer formally accepting deliverables?
- Control quality
* Validate scope
- Define scope
- Close project
? Control quality happens first, internally.

@@quiz
Under a fixed price contract, who mainly carries the risk of a cost overrun?
- The buyer
* The seller
- The sponsor
- Both equally
? That's why fixed price needs a well-defined scope.

@@flashcards
RTM :: requirement to objective, deliverable and test
gold plating :: extra work nobody asked for
100% rule :: children equal the parent's total
work package :: lowest WBS level
scope baseline :: scope statement + WBS + dictionary
validate scope :: customer acceptance
control quality :: internal correctness
FFP :: seller carries cost risk
CPFF :: buyer carries cost risk
CPIF :: shared by a ratio
T&M :: add a not-to-exceed cap
share ratio :: buyer/seller split of over- and underruns
ceiling price :: maximum the buyer pays (FPIF)
PTA :: target cost + (ceiling − target price) ÷ buyer share

@@drill
Target cost $100k, target price $110k, ceiling price $120k and a buyer share of 80%. What is the PTA in $k?
= 112.5
= 112.5k
? 100 + (120 − 110) ÷ 0.8.

@@drill
What is the name for work delivered beyond the requirements, without a change request?
= gold plating
? It's discouraged even if the customer might like it.

@@exercise Check a WBS and trace requirements
Roll up work package estimates through a WBS, check the 100% rule against each parent's budgeted total, and use a traceability matrix to find requirements with no deliverable or test, and deliverables with no requirement.
@@concepts maps, loops, functions
@@lang python
@@starter
wbs = {
    "1 Website": ["1.1 Design", "1.2 Build", "1.3 Launch"],
    "1.1 Design": ["1.1.1 Wireframes", "1.1.2 Visual design"],
    "1.2 Build": ["1.2.1 Front end", "1.2.2 CMS", "1.2.3 Animated mascot"],
    "1.3 Launch": ["1.3.1 Migration", "1.3.2 Training"],
}
estimates = {"1.1.1 Wireframes": 40, "1.1.2 Visual design": 60, "1.2.1 Front end": 120, "1.2.2 CMS": 80,
             "1.2.3 Animated mascot": 30, "1.3.1 Migration": 50, "1.3.2 Training": 20}
budgets = {"1 Website": 400, "1.1 Design": 100, "1.2 Build": 200, "1.3 Launch": 70}
rtm = {
    "R1 Editors publish pages": {"deliverable": "1.2.2 CMS", "test": "UAT-03"},
    "R2 Mobile friendly": {"deliverable": "1.2.1 Front end", "test": "UAT-01"},
    "R3 Old content kept": {"deliverable": "1.3.1 Migration", "test": None},
    "R4 Accessible (WCAG AA)": {"deliverable": None, "test": None},
    "R5 Staff can use it": {"deliverable": "1.3.2 Training", "test": "Survey-1"},
    "R6 Brand look": {"deliverable": "1.1.2 Visual design", "test": "Review-2"},
    "R7 Page layout agreed": {"deliverable": "1.1.1 Wireframes", "test": "Review-1"},
}

def rollup(element):
    raise NotImplementedError
@@expected
1 Website     400 h  ok
1.1 Design    100 h  ok
1.2 Build     230 h  100% rule broken: children total 230, budget 200
1.3 Launch     70 h  ok
R3 Old content kept: no test
R4 Accessible (WCAG AA): no deliverable and no test
1.2.3 Animated mascot: no requirement -> gold plating? (30 h)
@@requires
def rollup
budgets
rtm
@@hint
`rollup` returns the estimate for a work package, or else the sum of `rollup` over its children (recursion). Compare each parent's rollup with its budget. In the RTM, look for a missing deliverable or test. Gold plating is any work package that no requirement points to.
@@solution
wbs = {
    "1 Website": ["1.1 Design", "1.2 Build", "1.3 Launch"],
    "1.1 Design": ["1.1.1 Wireframes", "1.1.2 Visual design"],
    "1.2 Build": ["1.2.1 Front end", "1.2.2 CMS", "1.2.3 Animated mascot"],
    "1.3 Launch": ["1.3.1 Migration", "1.3.2 Training"],
}
estimates = {"1.1.1 Wireframes": 40, "1.1.2 Visual design": 60, "1.2.1 Front end": 120, "1.2.2 CMS": 80,
             "1.2.3 Animated mascot": 30, "1.3.1 Migration": 50, "1.3.2 Training": 20}
budgets = {"1 Website": 400, "1.1 Design": 100, "1.2 Build": 200, "1.3 Launch": 70}
rtm = {
    "R1 Editors publish pages": {"deliverable": "1.2.2 CMS", "test": "UAT-03"},
    "R2 Mobile friendly": {"deliverable": "1.2.1 Front end", "test": "UAT-01"},
    "R3 Old content kept": {"deliverable": "1.3.1 Migration", "test": None},
    "R4 Accessible (WCAG AA)": {"deliverable": None, "test": None},
    "R5 Staff can use it": {"deliverable": "1.3.2 Training", "test": "Survey-1"},
    "R6 Brand look": {"deliverable": "1.1.2 Visual design", "test": "Review-2"},
    "R7 Page layout agreed": {"deliverable": "1.1.1 Wireframes", "test": "Review-1"},
}

def rollup(element):
    if element in estimates:
        return estimates[element]
    return sum(rollup(child) for child in wbs[element])

for element, budget in budgets.items():
    total = rollup(element)
    status = "ok" if total == budget else f"100% rule broken: children total {total}, budget {budget}"
    print(f"{element:<12} {total:>4} h  {status}")

for req, link in rtm.items():
    gaps = [k for k in ("deliverable", "test") if link[k] is None]
    if gaps:
        print(f"{req}: no {' and no '.join(gaps)}")
traced = {link["deliverable"] for link in rtm.values()}
for package in estimates:
    if package not in traced:
        print(f"{package}: no requirement -> gold plating? ({estimates[package]} h)")

@@exercise Contract incentive calculations
Compute what the buyer pays and the seller earns under CPFF, CPIF and FPIF contracts for several actual-cost outcomes, apply the FPIF ceiling, and compute the point of total assumption.
@@concepts numbers, functions, conditions
@@lang python
@@starter
TARGET_COST, TARGET_FEE = 100_000, 10_000
BUYER_SHARE, SELLER_SHARE = 0.8, 0.2
CEILING = 120_000
outcomes = [90_000, 100_000, 115_000, 130_000]

def cpif(actual):
    raise NotImplementedError

def fpif(actual):
    raise NotImplementedError
@@expected
actual $90,000: CPFF buyer pays $100,000; CPIF buyer pays $102,000 (fee $12,000); FPIF buyer pays $102,000 (seller profit $12,000)
actual $100,000: CPFF buyer pays $110,000; CPIF buyer pays $110,000 (fee $10,000); FPIF buyer pays $110,000 (seller profit $10,000)
actual $115,000: CPFF buyer pays $125,000; CPIF buyer pays $122,000 (fee $7,000); FPIF buyer pays $120,000 (seller profit $5,000)
actual $130,000: CPFF buyer pays $140,000; CPIF buyer pays $134,000 (fee $4,000); FPIF buyer pays $120,000 (seller loss $10,000)
point of total assumption: $112,500; above it the seller absorbs every extra dollar
@@requires
def cpif
def fpif
CEILING
SELLER_SHARE
@@hint
Incentive = SELLER_SHARE × (TARGET_COST − actual). CPIF: fee = TARGET_FEE + incentive, buyer pays actual + fee. FPIF: price = min(actual + TARGET_FEE + incentive, CEILING), and the seller's profit = price − actual. CPFF: buyer pays actual + TARGET_FEE. PTA = TARGET_COST + (CEILING − (TARGET_COST + TARGET_FEE)) / BUYER_SHARE.
@@solution
TARGET_COST, TARGET_FEE = 100_000, 10_000
BUYER_SHARE, SELLER_SHARE = 0.8, 0.2
CEILING = 120_000
outcomes = [90_000, 100_000, 115_000, 130_000]

def cpif(actual):
    fee = TARGET_FEE + SELLER_SHARE * (TARGET_COST - actual)
    return actual + fee, fee

def fpif(actual):
    price = min(actual + TARGET_FEE + SELLER_SHARE * (TARGET_COST - actual), CEILING)
    return price, price - actual

for actual in outcomes:
    cp_pay, cp_fee = cpif(actual)
    fp_pay, fp_profit = fpif(actual)
    print(f"actual ${actual:,}: CPFF buyer pays ${actual + TARGET_FEE:,}; "
          f"CPIF buyer pays ${cp_pay:,.0f} (fee ${cp_fee:,.0f}); "
          f"FPIF buyer pays ${fp_pay:,.0f} (seller {'profit' if fp_profit >= 0 else 'loss'} ${abs(fp_profit):,.0f})")
pta = TARGET_COST + (CEILING - (TARGET_COST + TARGET_FEE)) / BUYER_SHARE
print(f"point of total assumption: ${pta:,.0f}; above it the seller absorbs every extra dollar")

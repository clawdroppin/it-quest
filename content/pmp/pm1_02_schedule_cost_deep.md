@@lesson pm1-schedule-cost-deep Schedule and Cost in Depth: Estimating, Compression and Earned Value Forecasts
@@text
The PMP's calculation questions concentrate here: three-point estimates, float, schedule compression, and earned value with its several forecasting formulas. You also need to know when each formula applies, because the exam describes the situation in words ("the variance is expected to continue"). This lesson covers estimating, the critical path and compression, budgets and reserves, and EVM forecasting.

## Estimating

| Technique | Notes |
|---|---|
| **Analogous** | top-down, from a similar past project; fast, least accurate |
| **Parametric** | unit rate × quantity (e.g. 2 hours per user × 400 users) |
| **Bottom-up** | estimate work packages and roll up; most accurate, slowest |
| **Three-point (triangular)** | (O + M + P) / 3 |
| **Three-point (beta / PERT)** | **(O + 4M + P) / 6**, standard deviation **(P − O) / 6** |

Ranges: rough order of magnitude **−25% to +75%** (initiation), definitive **−5% to +10%**. Agile estimating: story points, planning poker, affinity estimating, T-shirt sizes.

PERT for a path: add the activity means; add the **variances** (σ²), then take the square root for the path's σ. ±1σ ≈ 68%, ±2σ ≈ 95%, ±3σ ≈ 99.7%.

## Critical path and float

- **Forward pass**: ES, EF = ES + duration; **backward pass**: LF, LS = LF − duration.
- **Total float** = LS − ES (= LF − EF); critical path activities have zero float (in a normal network). **Free float**: delay without delaying any successor's early start.
- Dependencies: FS (most common), SS, FF, SF; **leads** (start early) and **lags** (wait). Mandatory (hard logic), discretionary (preferred), external, internal.
- **Resource levelling** can lengthen the critical path; **resource smoothing** stays within float.

## Schedule compression

- **Crashing**: add resources to critical path activities; **costs money**; choose the cheapest crash cost per time unit on the critical path first, and watch for new critical paths.
- **Fast tracking**: do activities in parallel that were planned in sequence; **adds risk/rework**, no direct cost; only for discretionary dependencies.

## Cost baseline and budget

Activity estimates + **contingency reserves** (known-unknowns, for identified risks; PM controls) = **cost baseline**; + **management reserves** (unknown-unknowns; sponsor approval to use) = **project budget**. Using management reserve changes the baseline through change control.

## Earned value management

| Term | Formula | Meaning |
|---|---|---|
| PV | planned value | budgeted cost of scheduled work |
| EV | % complete × BAC | budgeted cost of work performed |
| AC | actual cost | spent so far |
| CV | EV − AC | negative = over budget |
| SV | EV − PV | negative = behind schedule |
| CPI | EV / AC | < 1 over budget |
| SPI | EV / PV | < 1 behind schedule |

Forecasts (**EAC**):
- **BAC / CPI**: the current cost performance is expected to **continue** (typical).
- **AC + (BAC − EV)**: the variance was **atypical**; the remaining work will go as planned.
- **AC + (BAC − EV) / (CPI × SPI)**: both cost and schedule performance influence the rest (e.g. a fixed date must be met).
- **AC + bottom-up ETC**: the original plan is no longer valid; re-estimate.

**ETC** = EAC − AC; **VAC** = BAC − EAC; **TCPI** = (BAC − EV) / (BAC − AC) to meet the budget, or (BAC − EV) / (EAC − AC) to meet the new EAC. TCPI > 1 means the remaining work must be done more efficiently than planned.

@@quiz
An activity has O = 4, M = 6, P = 14 days. What is the PERT estimate?
- 6 days
* 7 days
- 8 days
- 8.5 days
? (4 + 24 + 14) / 6 = 7.

@@quiz
What is the standard deviation of that PERT estimate?
- 1 day
* 1.67 days
- 3 days
- 10 days
? (14 − 4) / 6.

@@quiz
To shorten the schedule without adding cost, the PM overlaps design and build. What is this, and what's the main downside?
- Crashing; higher cost
* Fast tracking; more risk and possible rework
- Resource levelling; longer schedule
- Smoothing; lower quality
? Only works for discretionary dependencies.

@@quiz
BAC is $100,000, EV $40,000, AC $50,000. Performance is expected to continue. What is the EAC?
- $110,000
* $125,000
- $100,000
- $90,000
? CPI = 0.8; 100,000 / 0.8.

@@quiz
Same project, but the overrun was a one-off, atypical event. What's the EAC?
* $110,000
- $125,000
- $150,000
- $100,000
? AC + (BAC − EV) = 50,000 + 60,000.

@@quiz
Who must approve use of management reserves?
- The project manager
* The sponsor or management (through change control)
- The team
- The vendor
? Contingency reserves are the PM's to use for identified risks.

@@quiz
TCPI to meet the original budget is 1.2. What does that mean?
- The project is under budget
* Remaining work must be 20% more cost-efficient than planned
- The project is 20% complete
- Nothing; TCPI is a schedule measure
? (BAC − EV) / (BAC − AC).

@@flashcards
analogous estimate :: top-down from similar project
parametric estimate :: unit rate × quantity
bottom-up :: most accurate, slowest
triangular :: (O + M + P) / 3
PERT :: (O + 4M + P) / 6
PERT σ :: (P − O) / 6
path σ :: sqrt of summed variances
ROM range :: −25% to +75%
total float :: LS − ES
free float :: delay without affecting successors
crashing :: add resources, costs money
fast tracking :: parallel work, adds risk
contingency reserve :: identified risks, PM controls
management reserve :: unknown-unknowns, sponsor
cost baseline :: estimates + contingency
budget :: baseline + management reserve
CV :: EV − AC
SV :: EV − PV
CPI :: EV / AC
SPI :: EV / PV
EAC typical :: BAC / CPI
EAC atypical :: AC + (BAC − EV)
EAC both indices :: AC + (BAC − EV) / (CPI × SPI)
VAC :: BAC − EAC
TCPI :: (BAC − EV) / (BAC − AC)

@@drill
What is the EAC formula when current cost performance is expected to continue?
= BAC / CPI
= BAC/CPI
? Atypical: AC + (BAC − EV).

@@drill
Which compression technique adds resources and cost?
= crashing
= Crashing
? Fast tracking adds risk.

@@exercise PERT estimates and the probability of meeting a date
For each activity on a path, compute the PERT mean and variance. Sum the means and variances for the path, take the path σ, then print the estimate at ±1σ, ±2σ and ±3σ, and say which confidence band a deadline falls into.
@@lang python
@@starter
import math

path = [("requirements", 4, 6, 14), ("design", 8, 10, 18), ("build", 15, 20, 37), ("test", 5, 8, 11), ("deploy", 1, 2, 3)]
deadlines = [44, 52, 57, 65]

def pert(o, m, p):
    raise NotImplementedError
@@expected
requirements O  4 M  6 P 14  mean  7.00  sigma 1.67
design       O  8 M 10 P 18  mean 11.00  sigma 1.67
build        O 15 M 20 P 37  mean 22.00  sigma 3.67
test         O  5 M  8 P 11  mean  8.00  sigma 1.00
deploy       O  1 M  2 P  3  mean  2.00  sigma 0.33
Path: mean 50.00 days, sigma 4.48
  ±1 sigma (68%): 45.5 to 54.5 days
  ±2 sigma (95%): 41.0 to 59.0 days
  ±3 sigma (99.7%): 36.5 to 63.5 days
deadline 44: below the mean, less than 50% likely
deadline 52: between +0 and +1 sigma, probability 50% to about 84%
deadline 57: between +1 and +2 sigma, probability about 84% to about 97.7%
deadline 65: beyond +3 sigma, almost certain
@@requires
def pert
math.sqrt(
@@hint
`pert` returns `(mean, variance)` with variance `((p - o) / 6) ** 2`. For a deadline, find the smallest k in 1, 2, 3 with `deadline <= mean + k * sigma`; one-sided probabilities are about 84%, 97.7% and 99.9%.
@@solution
import math

path = [("requirements", 4, 6, 14), ("design", 8, 10, 18), ("build", 15, 20, 37), ("test", 5, 8, 11), ("deploy", 1, 2, 3)]
deadlines = [44, 52, 57, 65]

def pert(o, m, p):
    return (o + 4 * m + p) / 6, ((p - o) / 6) ** 2

mean = variance = 0
for name, o, m, p in path:
    e, v = pert(o, m, p)
    mean += e
    variance += v
    print(f"{name:<12} O{o:>3} M{m:>3} P{p:>3}  mean {e:5.2f}  sigma {math.sqrt(v):4.2f}")
sigma = math.sqrt(variance)
print(f"Path: mean {mean:.2f} days, sigma {sigma:.2f}")
for k, pct in ((1, "68%"), (2, "95%"), (3, "99.7%")):
    print(f"  ±{k} sigma ({pct}): {mean - k * sigma:.1f} to {mean + k * sigma:.1f} days")

ONE_SIDED = {1: "about 84%", 2: "about 97.7%", 3: "about 99.9%"}
for deadline in deadlines:
    if deadline < mean:
        print(f"deadline {deadline}: below the mean, less than 50% likely")
        continue
    k = next((k for k in (1, 2, 3) if deadline <= mean + k * sigma), None)
    if k is None:
        print(f"deadline {deadline}: beyond +3 sigma, almost certain")
    else:
        lower = ONE_SIDED[k - 1] if k > 1 else "50%"
        print(f"deadline {deadline}: between +{k - 1} and +{k} sigma, probability {lower} to {ONE_SIDED[k]}")

@@exercise Forecast with earned value
For each status report, compute CV, SV, CPI, SPI and the EAC using the formula that fits the situation (`typical`, `atypical`, `both` or `replan` with a bottom-up ETC). Then compute ETC, VAC and TCPI to meet BAC, and print a one-line verdict.
@@lang python
@@starter
reports = [
    ("CRM rollout", 500_000, 0.40, 220_000, 240_000, "typical", None),
    ("Network refresh", 300_000, 0.50, 150_000, 170_000, "atypical", None),
    ("ERP upgrade", 800_000, 0.30, 280_000, 300_000, "both", None),
    ("Data centre move", 1_200_000, 0.60, 750_000, 820_000, "replan", 650_000),
]

def eac(bac, ev, ac, cpi, spi, situation, etc_bottom_up):
    raise NotImplementedError
@@expected
CRM rollout: EV 200,000 CV -40,000 SV -20,000 CPI 0.83 SPI 0.91
  EAC (typical) 600,000, ETC 360,000, VAC -100,000, TCPI 1.15: over budget, behind schedule
Network refresh: EV 150,000 CV -20,000 SV 0 CPI 0.88 SPI 1.00
  EAC (atypical) 320,000, ETC 150,000, VAC -20,000, TCPI 1.15: over budget, on/ahead of schedule
ERP upgrade: EV 240,000 CV -60,000 SV -40,000 CPI 0.80 SPI 0.86
  EAC (both) 1,116,667, ETC 816,667, VAC -316,667, TCPI 1.12: over budget, behind schedule
Data centre move: EV 720,000 CV -100,000 SV -30,000 CPI 0.88 SPI 0.96
  EAC (replan) 1,470,000, ETC 650,000, VAC -270,000, TCPI 1.26: over budget, behind schedule
@@requires
def eac
bac / cpi
cpi * spi
@@hint
Each report is `(name, BAC, % complete, PV, AC, situation, bottom-up ETC)`. EV = % complete × BAC. TCPI = (BAC − EV) / (BAC − AC).
@@solution
reports = [
    ("CRM rollout", 500_000, 0.40, 220_000, 240_000, "typical", None),
    ("Network refresh", 300_000, 0.50, 150_000, 170_000, "atypical", None),
    ("ERP upgrade", 800_000, 0.30, 280_000, 300_000, "both", None),
    ("Data centre move", 1_200_000, 0.60, 750_000, 820_000, "replan", 650_000),
]

def eac(bac, ev, ac, cpi, spi, situation, etc_bottom_up):
    if situation == "typical":
        return bac / cpi
    if situation == "atypical":
        return ac + (bac - ev)
    if situation == "both":
        return ac + (bac - ev) / (cpi * spi)
    return ac + etc_bottom_up

for name, bac, pct, pv, ac, situation, etc_bu in reports:
    ev = pct * bac
    cv, sv = ev - ac, ev - pv
    cpi, spi = ev / ac, ev / pv
    forecast = eac(bac, ev, ac, cpi, spi, situation, etc_bu)
    etc = forecast - ac
    vac = bac - forecast
    tcpi = (bac - ev) / (bac - ac)
    status = ("over budget" if cpi < 1 else "on/under budget") + ", " + ("behind schedule" if spi < 1 else "on/ahead of schedule")
    print(f"{name}: EV {ev:,.0f} CV {cv:,.0f} SV {sv:,.0f} CPI {cpi:.2f} SPI {spi:.2f}")
    print(f"  EAC ({situation}) {forecast:,.0f}, ETC {etc:,.0f}, VAC {vac:,.0f}, TCPI {tcpi:.2f}: {status}")

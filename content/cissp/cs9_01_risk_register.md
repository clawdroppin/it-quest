@@lesson cs9-risk-register Quantitative Risk Register
@@text
A CISO doesn't ask the board for "more security". They say: *ransomware costs us an expected ¥6,000,000 a year; ¥800,000 of immutable backups cuts that to ¥750,000.* That sentence is the CISSP's risk domain in one line, and it's how security budgets get approved. You'll build the register that produces it:

```text
ID   Asset                 Threat                           SLE   ARO          ALE  Treatment
R2   file servers          ransomware               ¥24,000,000  0.25   ¥6,000,000  mitigate: immutable offline backups (saves ¥4,450,000/yr)
R3   Osaka data centre     earthquake              ¥240,000,000  0.02   ¥4,800,000  transfer or avoid
R1   customer database     data breach              ¥20,000,000   0.2   ¥4,000,000  mitigate: DLP and DB activity monitoring (saves ¥1,500,000/yr)
R4   sales laptops         theft or loss               ¥300,000     2     ¥600,000  accept
R5   public website        defacement                  ¥400,000   0.5     ¥200,000  accept
Total ALE ¥15,600,000 per year; risk appetite ¥1,000,000 per risk

Continuity:
  ✘ online orders: RTO + WRT (5h) exceeds MTD (4h)
  ✘ online orders: backups every 1h can't meet RPO 0.25h
```

`register.json` lists the risks (asset value, exposure factor, ARO) with **candidate controls**, each with its annual cost and what it changes: a lower ARO (it happens less often) or a lower EF (it hurts less when it happens). It also lists business processes from the **BIA** with their MTD, RTO, WRT, RPO and backup interval.

Look at the decisions the numbers make:

- **R2**: two controls are worth it, but the backups save more. (In reality you'd likely buy both: run the program with only EDR to see its value.)
- **R3**: the retrofit costs more than the risk. The ALE is still above the appetite, so the answer is **transfer** (earthquake insurance) or avoid (move to another region). Japan makes this one very real.
- **R4**: cable locks save exactly what they cost: a value of zero isn't worth the effort, and the ALE is within the appetite, so **accept**.

You'll write `sle`, `ale`, `control_value`, `best_control`, `treatment` and `continuity_problems` in `risk.py`.
@@project risk_register Quantitative Risk Register
Write the six functions in `risk.py` until `python -m unittest` passes, then `python main.py`. Change the numbers in `register.json` and watch the recommendations change.
@@file risk.py
"""A quantitative risk register and continuity check, the CISSP way."""
from dataclasses import dataclass, field


@dataclass
class Control:
    name: str
    annual_cost: float
    aro: float | None = None  # the ARO after the control (None: unchanged)
    ef: float | None = None   # the exposure factor after the control (None: unchanged)


@dataclass
class Risk:
    id: str
    asset: str
    threat: str
    av: float    # asset value
    ef: float    # exposure factor, 0..1
    aro: float   # annualized rate of occurrence
    controls: list[Control] = field(default_factory=list)


@dataclass
class Process:
    name: str
    mtd: float              # maximum tolerable downtime, hours
    rto: float              # recovery time objective, hours
    wrt: float              # work recovery time, hours
    rpo: float              # recovery point objective, hours of data
    backup_interval: float  # hours between backups


def load(data: dict) -> tuple[list[Risk], list[Process], float]:
    """Read register.json: {"appetite": ..., "risks": [...], "processes": [...]}."""
    risks = [Risk(**{**r, "controls": [Control(**c) for c in r.get("controls", [])]}) for r in data["risks"]]
    processes = [Process(**p) for p in data.get("processes", [])]
    return risks, processes, data["appetite"]


def sle(av: float, ef: float) -> float:
    """Single loss expectancy: AV × EF."""
    raise NotImplementedError


def ale(av: float, ef: float, aro: float) -> float:
    """Annualized loss expectancy: SLE × ARO."""
    raise NotImplementedError


# HINT: the ALE after uses the control's aro and ef where they're set, and the risk's own otherwise.
def control_value(risk: Risk, control: Control) -> float:
    """The annual value of a safeguard: ALE before − ALE after − the control's annual cost.
    Negative means it costs more than it saves."""
    raise NotImplementedError


# HINT: max(..., key=...) finds the best; then check that its value is above zero.
def best_control(risk: Risk) -> tuple[Control, float] | None:
    """The control with the highest positive value, and that value; None if no control is worth it."""
    raise NotImplementedError


def treatment(risk: Risk, appetite: float) -> str:
    """What to recommend:
    - a control with positive value → "mitigate: <control name>"
    - otherwise, an ALE at or below the appetite → "accept"
    - otherwise → "transfer or avoid"
    """
    raise NotImplementedError


# HINT: two checks per process; a process can have both problems.
def continuity_problems(p: Process) -> list[str]:
    """Problems with a process's continuity plan:
    - RTO + WRT longer than the MTD → "<name>: RTO + WRT (<x>h) exceeds MTD (<y>h)"
    - backups less frequent than the RPO → "<name>: backups every <x>h can't meet RPO <y>h"
    Hours are printed with :g formatting (4.0 → 4, 0.5 → 0.5)."""
    raise NotImplementedError


def yen(x: float) -> str:
    return f"¥{x:,.0f}"


def report(risks: list[Risk], processes: list[Process], appetite: float) -> str:
    lines = [f"{'ID':<5}{'Asset':<22}{'Threat':<22}{'SLE':>14}{'ARO':>6}{'ALE':>13}  Treatment"]
    total = 0.0
    for r in sorted(risks, key=lambda r: -ale(r.av, r.ef, r.aro)):
        a = ale(r.av, r.ef, r.aro)
        total += a
        t = treatment(r, appetite)
        best = best_control(r)
        if best:
            t += f" (saves {yen(best[1])}/yr)"
        lines.append(f"{r.id:<5}{r.asset:<22}{r.threat:<22}{yen(sle(r.av, r.ef)):>14}{r.aro:>6g}{yen(a):>13}  {t}")
    lines.append(f"Total ALE {yen(total)} per year; risk appetite {yen(appetite)} per risk")
    problems = [x for p in processes for x in continuity_problems(p)]
    lines += ["", "Continuity:"] + ([f"  ✘ {x}" for x in problems] or ["  ✔ every plan meets its MTD and RPO"])
    return "\n".join(lines)
@@file test_risk.py
import unittest

from risk import Control, Process, Risk, ale, best_control, continuity_problems, control_value, sle, treatment

# The example from the lesson: a ¥50,000,000 database, EF 0.4, once every 5 years.
DB = Risk("R1", "customer DB", "breach", av=50_000_000, ef=0.4, aro=0.2,
          controls=[Control("DLP", 1_500_000, aro=0.05), Control("gold plating", 5_000_000, aro=0.0)])


class Formulas(unittest.TestCase):
    def test_sle_ale(self):
        self.assertEqual(sle(50_000_000, 0.4), 20_000_000)
        self.assertAlmostEqual(ale(50_000_000, 0.4, 0.2), 4_000_000)
        self.assertAlmostEqual(ale(200_000, 0.25, 0.1), 5_000)


class Controls(unittest.TestCase):
    def test_value(self):
        self.assertAlmostEqual(control_value(DB, DB.controls[0]), 1_500_000)
        self.assertAlmostEqual(control_value(DB, DB.controls[1]), -1_000_000)

    def test_ef_control(self):
        backup = Control("offline backups", 400_000, ef=0.1)
        r = Risk("R2", "file server", "ransomware", av=10_000_000, ef=0.9, aro=0.3)
        # before 2,700,000; after 300,000; minus 400,000
        self.assertAlmostEqual(control_value(r, backup), 2_000_000)

    def test_best(self):
        control, value = best_control(DB)
        self.assertEqual(control.name, "DLP")
        self.assertAlmostEqual(value, 1_500_000)

    def test_none_worth_it(self):
        r = Risk("R3", "lobby TV", "theft", av=200_000, ef=1.0, aro=0.1, controls=[Control("guard", 3_000_000, aro=0.0)])
        self.assertIsNone(best_control(r))
        self.assertIsNone(best_control(Risk("R4", "x", "y", 1, 1, 1)))


class Treatment(unittest.TestCase):
    def test_choices(self):
        self.assertEqual(treatment(DB, 1_000_000), "mitigate: DLP")
        small = Risk("R5", "lobby TV", "theft", av=200_000, ef=1.0, aro=0.1)
        self.assertEqual(treatment(small, 1_000_000), "accept")
        big = Risk("R6", "factory", "earthquake", av=2_000_000_000, ef=0.5, aro=0.01)
        self.assertEqual(treatment(big, 1_000_000), "transfer or avoid")

    def test_appetite_is_inclusive(self):
        r = Risk("R7", "a", "b", av=1_000_000, ef=1.0, aro=1.0)
        self.assertEqual(treatment(r, 1_000_000), "accept")


class Continuity(unittest.TestCase):
    def test_ok(self):
        self.assertEqual(continuity_problems(Process("payroll", mtd=72, rto=24, wrt=8, rpo=24, backup_interval=24)), [])

    def test_both(self):
        p = Process("orders", mtd=4, rto=4, wrt=1.5, rpo=0.25, backup_interval=1)
        self.assertEqual(continuity_problems(p), [
            "orders: RTO + WRT (5.5h) exceeds MTD (4h)",
            "orders: backups every 1h can't meet RPO 0.25h",
        ])


if __name__ == "__main__":
    unittest.main()
@@file main.py
"""`python main.py [register.json]`: print the risk register and the continuity check."""
import json
import sys
from pathlib import Path

from risk import load, report


def main(argv: list[str]) -> int:
    data = json.loads(Path(argv[1] if len(argv) > 1 else "register.json").read_text(encoding="utf-8"))
    print(report(*load(data)))
    return 0


if __name__ == "__main__":
    sys.stdout.reconfigure(encoding="utf-8")
    sys.exit(main(sys.argv))
@@file register.json
{
  "appetite": 1000000,
  "risks": [
    {
      "id": "R1", "asset": "customer database", "threat": "data breach",
      "av": 50000000, "ef": 0.4, "aro": 0.2,
      "controls": [
        { "name": "DLP and DB activity monitoring", "annual_cost": 1500000, "aro": 0.05 },
        { "name": "full tokenization project", "annual_cost": 4500000, "aro": 0.01 }
      ]
    },
    {
      "id": "R2", "asset": "file servers", "threat": "ransomware",
      "av": 30000000, "ef": 0.8, "aro": 0.25,
      "controls": [
        { "name": "immutable offline backups", "annual_cost": 800000, "ef": 0.1 },
        { "name": "EDR on every endpoint", "annual_cost": 2400000, "aro": 0.08 }
      ]
    },
    {
      "id": "R3", "asset": "Osaka data centre", "threat": "earthquake",
      "av": 400000000, "ef": 0.6, "aro": 0.02,
      "controls": [
        { "name": "seismic isolation retrofit", "annual_cost": 9000000, "ef": 0.1 }
      ]
    },
    {
      "id": "R4", "asset": "sales laptops", "threat": "theft or loss",
      "av": 300000, "ef": 1.0, "aro": 2,
      "controls": [
        { "name": "cable locks", "annual_cost": 150000, "aro": 1.5 }
      ]
    },
    {
      "id": "R5", "asset": "public website", "threat": "defacement",
      "av": 2000000, "ef": 0.2, "aro": 0.5,
      "controls": []
    }
  ],
  "processes": [
    { "name": "online orders", "mtd": 4, "rto": 3, "wrt": 2, "rpo": 0.25, "backup_interval": 1 },
    { "name": "payroll", "mtd": 72, "rto": 24, "wrt": 8, "rpo": 24, "backup_interval": 24 },
    { "name": "email", "mtd": 24, "rto": 8, "wrt": 2, "rpo": 1, "backup_interval": 0.5 }
  ]
}
@@text
> Tips: `control_value` needs the ALE twice: once with the risk's own EF and ARO, once with the control's where they're given (`risk.aro if control.aro is None else control.aro`; don't write `control.aro or risk.aro`, because an ARO of `0` is a real value). `best_control` is `max(risk.controls, key=...)` plus a check that the value is above zero.

Quantitative analysis is only as good as its numbers. ARO and EF are estimates: use incident history, insurance data and industry reports, write down where each number came from, and review them every year. Many organizations combine this with a qualitative heat map for risks that are hard to put money on (reputation, safety).
@@project risk_register_finished Quantitative Risk Register: the finished version
The complete register. Ideas: combinations of controls (apply two and compute the joint ALE), a qualitative heat map (likelihood × impact on a 5 × 5 grid), residual risk after the chosen treatment, an owner and review date per risk, or CSV export for the risk committee.
@@file risk.py
"""A quantitative risk register and continuity check, the CISSP way."""
from dataclasses import dataclass, field


@dataclass
class Control:
    name: str
    annual_cost: float
    aro: float | None = None  # the ARO after the control (None: unchanged)
    ef: float | None = None   # the exposure factor after the control (None: unchanged)


@dataclass
class Risk:
    id: str
    asset: str
    threat: str
    av: float    # asset value
    ef: float    # exposure factor, 0..1
    aro: float   # annualized rate of occurrence
    controls: list[Control] = field(default_factory=list)


@dataclass
class Process:
    name: str
    mtd: float              # maximum tolerable downtime, hours
    rto: float              # recovery time objective, hours
    wrt: float              # work recovery time, hours
    rpo: float              # recovery point objective, hours of data
    backup_interval: float  # hours between backups


def load(data: dict) -> tuple[list[Risk], list[Process], float]:
    """Read register.json: {"appetite": ..., "risks": [...], "processes": [...]}."""
    risks = [Risk(**{**r, "controls": [Control(**c) for c in r.get("controls", [])]}) for r in data["risks"]]
    processes = [Process(**p) for p in data.get("processes", [])]
    return risks, processes, data["appetite"]


def sle(av: float, ef: float) -> float:
    """Single loss expectancy: AV × EF."""
    return av * ef


def ale(av: float, ef: float, aro: float) -> float:
    """Annualized loss expectancy: SLE × ARO."""
    return sle(av, ef) * aro


# HINT: the ALE after uses the control's aro and ef where they're set, and the risk's own otherwise.
def control_value(risk: Risk, control: Control) -> float:
    """The annual value of a safeguard: ALE before − ALE after − the control's annual cost.
    Negative means it costs more than it saves."""
    aro = risk.aro if control.aro is None else control.aro
    ef = risk.ef if control.ef is None else control.ef
    return ale(risk.av, risk.ef, risk.aro) - ale(risk.av, ef, aro) - control.annual_cost


# HINT: max(..., key=...) finds the best; then check that its value is above zero.
def best_control(risk: Risk) -> tuple[Control, float] | None:
    """The control with the highest positive value, and that value; None if no control is worth it."""
    if not risk.controls:
        return None
    best = max(risk.controls, key=lambda c: control_value(risk, c))
    value = control_value(risk, best)
    return (best, value) if value > 0 else None


def treatment(risk: Risk, appetite: float) -> str:
    """What to recommend:
    - a control with positive value → "mitigate: <control name>"
    - otherwise, an ALE at or below the appetite → "accept"
    - otherwise → "transfer or avoid"
    """
    best = best_control(risk)
    if best:
        return f"mitigate: {best[0].name}"
    if ale(risk.av, risk.ef, risk.aro) <= appetite:
        return "accept"
    return "transfer or avoid"


# HINT: two checks per process; a process can have both problems.
def continuity_problems(p: Process) -> list[str]:
    """Problems with a process's continuity plan:
    - RTO + WRT longer than the MTD → "<name>: RTO + WRT (<x>h) exceeds MTD (<y>h)"
    - backups less frequent than the RPO → "<name>: backups every <x>h can't meet RPO <y>h"
    Hours are printed with :g formatting (4.0 → 4, 0.5 → 0.5)."""
    out = []
    if p.rto + p.wrt > p.mtd:
        out.append(f"{p.name}: RTO + WRT ({p.rto + p.wrt:g}h) exceeds MTD ({p.mtd:g}h)")
    if p.backup_interval > p.rpo:
        out.append(f"{p.name}: backups every {p.backup_interval:g}h can't meet RPO {p.rpo:g}h")
    return out


def yen(x: float) -> str:
    return f"¥{x:,.0f}"


def report(risks: list[Risk], processes: list[Process], appetite: float) -> str:
    lines = [f"{'ID':<5}{'Asset':<22}{'Threat':<22}{'SLE':>14}{'ARO':>6}{'ALE':>13}  Treatment"]
    total = 0.0
    for r in sorted(risks, key=lambda r: -ale(r.av, r.ef, r.aro)):
        a = ale(r.av, r.ef, r.aro)
        total += a
        t = treatment(r, appetite)
        best = best_control(r)
        if best:
            t += f" (saves {yen(best[1])}/yr)"
        lines.append(f"{r.id:<5}{r.asset:<22}{r.threat:<22}{yen(sle(r.av, r.ef)):>14}{r.aro:>6g}{yen(a):>13}  {t}")
    lines.append(f"Total ALE {yen(total)} per year; risk appetite {yen(appetite)} per risk")
    problems = [x for p in processes for x in continuity_problems(p)]
    lines += ["", "Continuity:"] + ([f"  ✘ {x}" for x in problems] or ["  ✔ every plan meets its MTD and RPO"])
    return "\n".join(lines)
@@file test_risk.py
import unittest

from risk import Control, Process, Risk, ale, best_control, continuity_problems, control_value, sle, treatment

# The example from the lesson: a ¥50,000,000 database, EF 0.4, once every 5 years.
DB = Risk("R1", "customer DB", "breach", av=50_000_000, ef=0.4, aro=0.2,
          controls=[Control("DLP", 1_500_000, aro=0.05), Control("gold plating", 5_000_000, aro=0.0)])


class Formulas(unittest.TestCase):
    def test_sle_ale(self):
        self.assertEqual(sle(50_000_000, 0.4), 20_000_000)
        self.assertAlmostEqual(ale(50_000_000, 0.4, 0.2), 4_000_000)
        self.assertAlmostEqual(ale(200_000, 0.25, 0.1), 5_000)


class Controls(unittest.TestCase):
    def test_value(self):
        self.assertAlmostEqual(control_value(DB, DB.controls[0]), 1_500_000)
        self.assertAlmostEqual(control_value(DB, DB.controls[1]), -1_000_000)

    def test_ef_control(self):
        backup = Control("offline backups", 400_000, ef=0.1)
        r = Risk("R2", "file server", "ransomware", av=10_000_000, ef=0.9, aro=0.3)
        # before 2,700,000; after 300,000; minus 400,000
        self.assertAlmostEqual(control_value(r, backup), 2_000_000)

    def test_best(self):
        control, value = best_control(DB)
        self.assertEqual(control.name, "DLP")
        self.assertAlmostEqual(value, 1_500_000)

    def test_none_worth_it(self):
        r = Risk("R3", "lobby TV", "theft", av=200_000, ef=1.0, aro=0.1, controls=[Control("guard", 3_000_000, aro=0.0)])
        self.assertIsNone(best_control(r))
        self.assertIsNone(best_control(Risk("R4", "x", "y", 1, 1, 1)))


class Treatment(unittest.TestCase):
    def test_choices(self):
        self.assertEqual(treatment(DB, 1_000_000), "mitigate: DLP")
        small = Risk("R5", "lobby TV", "theft", av=200_000, ef=1.0, aro=0.1)
        self.assertEqual(treatment(small, 1_000_000), "accept")
        big = Risk("R6", "factory", "earthquake", av=2_000_000_000, ef=0.5, aro=0.01)
        self.assertEqual(treatment(big, 1_000_000), "transfer or avoid")

    def test_appetite_is_inclusive(self):
        r = Risk("R7", "a", "b", av=1_000_000, ef=1.0, aro=1.0)
        self.assertEqual(treatment(r, 1_000_000), "accept")


class Continuity(unittest.TestCase):
    def test_ok(self):
        self.assertEqual(continuity_problems(Process("payroll", mtd=72, rto=24, wrt=8, rpo=24, backup_interval=24)), [])

    def test_both(self):
        p = Process("orders", mtd=4, rto=4, wrt=1.5, rpo=0.25, backup_interval=1)
        self.assertEqual(continuity_problems(p), [
            "orders: RTO + WRT (5.5h) exceeds MTD (4h)",
            "orders: backups every 1h can't meet RPO 0.25h",
        ])


if __name__ == "__main__":
    unittest.main()
@@file main.py
"""`python main.py [register.json]`: print the risk register and the continuity check."""
import json
import sys
from pathlib import Path

from risk import load, report


def main(argv: list[str]) -> int:
    data = json.loads(Path(argv[1] if len(argv) > 1 else "register.json").read_text(encoding="utf-8"))
    print(report(*load(data)))
    return 0


if __name__ == "__main__":
    sys.stdout.reconfigure(encoding="utf-8")
    sys.exit(main(sys.argv))
@@file register.json
{
  "appetite": 1000000,
  "risks": [
    {
      "id": "R1", "asset": "customer database", "threat": "data breach",
      "av": 50000000, "ef": 0.4, "aro": 0.2,
      "controls": [
        { "name": "DLP and DB activity monitoring", "annual_cost": 1500000, "aro": 0.05 },
        { "name": "full tokenization project", "annual_cost": 4500000, "aro": 0.01 }
      ]
    },
    {
      "id": "R2", "asset": "file servers", "threat": "ransomware",
      "av": 30000000, "ef": 0.8, "aro": 0.25,
      "controls": [
        { "name": "immutable offline backups", "annual_cost": 800000, "ef": 0.1 },
        { "name": "EDR on every endpoint", "annual_cost": 2400000, "aro": 0.08 }
      ]
    },
    {
      "id": "R3", "asset": "Osaka data centre", "threat": "earthquake",
      "av": 400000000, "ef": 0.6, "aro": 0.02,
      "controls": [
        { "name": "seismic isolation retrofit", "annual_cost": 9000000, "ef": 0.1 }
      ]
    },
    {
      "id": "R4", "asset": "sales laptops", "threat": "theft or loss",
      "av": 300000, "ef": 1.0, "aro": 2,
      "controls": [
        { "name": "cable locks", "annual_cost": 150000, "aro": 1.5 }
      ]
    },
    {
      "id": "R5", "asset": "public website", "threat": "defacement",
      "av": 2000000, "ef": 0.2, "aro": 0.5,
      "controls": []
    }
  ],
  "processes": [
    { "name": "online orders", "mtd": 4, "rto": 3, "wrt": 2, "rpo": 0.25, "backup_interval": 1 },
    { "name": "payroll", "mtd": 72, "rto": 24, "wrt": 8, "rpo": 24, "backup_interval": 24 },
    { "name": "email", "mtd": 24, "rto": 8, "wrt": 2, "rpo": 1, "backup_interval": 0.5 }
  ]
}

@@drill
Write the formula for annualized loss expectancy using SLE and ARO.
= ALE = SLE × ARO
= ale = sle x aro
= sle × aro
= sle x aro
= sle * aro
= ale = sle * aro
? SLE = asset value × exposure factor.

@@text
🎉 You can now turn security into business language, which is the core skill the CISSP tests and the reason security managers are paid what they are.

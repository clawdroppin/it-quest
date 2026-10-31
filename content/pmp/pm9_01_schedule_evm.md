@@lesson pm9-schedule-evm Office Move: Critical Path and Earned Value
@@text
The PMP exam expects you to do a forward and backward pass and the earned value formulas quickly and correctly. The fastest way to really understand them is to program them. The project: an IT office move, with eight activities, a fixed weekend for the move, and a budget of ¥6,000,000:

```text
ID  Activity                           Dur  ES  EF  LS  LF Float
A   Survey the new office                3   0   3   0   3     0 ◀ critical
B   Design the network and Wi-Fi         5   3   8   3   8     0 ◀ critical
C   Order switches, APs and cabling     10   8  18   8  18     0 ◀ critical
D   Contract the ISP line               20   3  23   4  24     1
...
Project duration: 35 days. Critical path: A → B → C → E → F → G → H

Earned value: CV -400,000, SV -300,000, CPI 0.84, SPI 0.88, EAC 7,142,857, ...
Status: over budget, behind schedule
```

Look at activity D, the ISP line: only **one day of float**. In real IT moves the ISP is famous for being late, so a good project manager would put it in the risk register and order the line on day one.

`topo_order` (sorting activities so predecessors come first) is written for you. You'll write `critical_path` (the forward and backward pass) and `earned_value` in `schedule.py`.
@@project office_move Office Move: Critical Path and Earned Value
Write `critical_path` and `earned_value` in `schedule.py` until `python -m unittest` passes, then `python main.py`. Do the forward and backward pass on paper first and compare.
@@file schedule.py
"""Critical path and earned value: the two calculations every PMP candidate must master."""
from dataclasses import dataclass, field


@dataclass
class Activity:
    id: str
    duration: int
    preds: list[str] = field(default_factory=list)
    es: int = 0
    ef: int = 0
    ls: int = 0
    lf: int = 0

    @property
    def float(self) -> int:
        return self.ls - self.es


def topo_order(acts: dict[str, Activity]) -> list[str]:
    """Activity IDs so that every activity comes after its predecessors. Raises ValueError on a loop."""
    order, state = [], {}

    def visit(a: str):
        if state.get(a) == "done":
            return
        if state.get(a) == "busy":
            raise ValueError(f"dependency loop at {a}")
        state[a] = "busy"
        for p in acts[a].preds:
            visit(p)
        state[a] = "done"
        order.append(a)

    for a in acts:
        visit(a)
    return order


# HINT: forward pass: ES = the largest EF of the predecessors (0 if none); EF = ES + duration.
# HINT: backward pass in reverse order: LF = the smallest LS of the successors (the project end if none); LS = LF − duration.
def critical_path(acts: dict[str, Activity]) -> tuple[int, list[str]]:
    """Fill in ES, EF, LS and LF for every activity, and return (project duration, the critical
    activities in order: those with zero float)."""
    raise NotImplementedError


def pert(o: float, m: float, p: float) -> float:
    """The PERT (beta) estimate."""
    return (o + 4 * m + p) / 6


# HINT: one dictionary with every value; round the indexes to 2 decimals and money to whole numbers.
def earned_value(bac: float, pv: float, ev: float, ac: float) -> dict[str, float]:
    """CV, SV, CPI, SPI, EAC (= BAC / CPI), ETC, VAC and TCPI (to finish on the original budget)."""
    raise NotImplementedError


def status(ev: dict[str, float]) -> str:
    cost = "under budget" if ev["CPI"] >= 1 else "over budget"
    time = "ahead of schedule" if ev["SPI"] >= 1 else "behind schedule"
    return f"{cost}, {time}"
@@file test_schedule.py
import unittest

from schedule import Activity, critical_path, earned_value, pert, topo_order


def network():
    # A: 3 → B: 4 → D: 2 ; A → C: 6 → D
    return {
        "A": Activity("A", 3),
        "B": Activity("B", 4, ["A"]),
        "C": Activity("C", 6, ["A"]),
        "D": Activity("D", 2, ["B", "C"]),
    }


class CriticalPath(unittest.TestCase):
    def test_duration_and_path(self):
        acts = network()
        self.assertEqual(critical_path(acts), (11, ["A", "C", "D"]))

    def test_floats(self):
        acts = network()
        critical_path(acts)
        self.assertEqual({a: x.float for a, x in acts.items()}, {"A": 0, "B": 2, "C": 0, "D": 0})
        self.assertEqual((acts["B"].es, acts["B"].ef, acts["B"].ls, acts["B"].lf), (3, 7, 5, 9))

    def test_two_starts(self):
        acts = {"A": Activity("A", 2), "B": Activity("B", 5), "C": Activity("C", 1, ["A", "B"])}
        self.assertEqual(critical_path(acts), (6, ["B", "C"]))
        self.assertEqual(acts["A"].float, 3)

    def test_loop(self):
        with self.assertRaises(ValueError):
            topo_order({"A": Activity("A", 1, ["B"]), "B": Activity("B", 1, ["A"])})


class Estimates(unittest.TestCase):
    def test_pert(self):
        self.assertEqual(pert(4, 6, 14), 7)


class EarnedValue(unittest.TestCase):
    def test_lesson_example(self):
        ev = earned_value(bac=10_000_000, pv=4_000_000, ev=3_000_000, ac=3_750_000)
        self.assertEqual(ev["CPI"], 0.8)
        self.assertEqual(ev["SPI"], 0.75)
        self.assertEqual(ev["CV"], -750_000)
        self.assertEqual(ev["SV"], -1_000_000)
        self.assertEqual(ev["EAC"], 12_500_000)
        self.assertEqual(ev["ETC"], 8_750_000)
        self.assertEqual(ev["VAC"], -2_500_000)
        self.assertEqual(ev["TCPI"], 1.12)


if __name__ == "__main__":
    unittest.main()
@@file main.py
"""`python main.py`: schedule and earned value for the office move project in project.json."""
import json
import sys

from schedule import Activity, critical_path, earned_value, status


def main() -> None:
    data = json.load(open(sys.argv[1] if len(sys.argv) > 1 else "project.json", encoding="utf-8"))
    acts = {a["id"]: Activity(a["id"], a["days"], a.get("after", [])) for a in data["activities"]}
    names = {a["id"]: a["name"] for a in data["activities"]}
    end, path = critical_path(acts)
    print(f"{'ID':<4}{'Activity':<34}{'Dur':>4}{'ES':>4}{'EF':>4}{'LS':>4}{'LF':>4}{'Float':>6}")
    for a in acts.values():
        mark = " ◀ critical" if a.id in path else ""
        print(f"{a.id:<4}{names[a.id]:<34}{a.duration:>4}{a.es:>4}{a.ef:>4}{a.ls:>4}{a.lf:>4}{a.float:>6}{mark}")
    print(f"\nProject duration: {end} days. Critical path: {' → '.join(path)}")
    ev = earned_value(**data["status"])
    print("\nEarned value: " + ", ".join(f"{k} {v:,}" for k, v in ev.items()))
    print("Status:", status(ev))


if __name__ == "__main__":
    sys.stdout.reconfigure(encoding="utf-8")
    main()
@@file project.json
{
  "activities": [
    { "id": "A", "name": "Survey the new office", "days": 3 },
    { "id": "B", "name": "Design the network and Wi-Fi", "days": 5, "after": ["A"] },
    { "id": "C", "name": "Order switches, APs and cabling", "days": 10, "after": ["B"] },
    { "id": "D", "name": "Contract the ISP line", "days": 20, "after": ["A"] },
    { "id": "E", "name": "Install cabling and racks", "days": 6, "after": ["C"] },
    { "id": "F", "name": "Configure and test the network", "days": 4, "after": ["E", "D"] },
    { "id": "G", "name": "Move PCs and printers (weekend)", "days": 2, "after": ["F"] },
    { "id": "H", "name": "Hypercare: on-site support", "days": 5, "after": ["G"] }
  ],
  "status": { "bac": 6000000, "pv": 2400000, "ev": 2100000, "ac": 2500000 }
}
@@text
> Tips: the forward pass is `max(..., default=0)` over the predecessors' EF. For the backward pass you need each activity's *successors*: build that dict first (`{a: [b for b in acts if a in acts[b].preds]}`), then walk the order in reverse with `min(..., default=project_end)`.

Then try exam-style "what if" questions: the ISP line slips by 3 days (change D to 23): what happens to the project end? Add more people to activity C so it takes 7 days (crashing): which path becomes critical?
@@project office_move_finished Office Move: the finished version
The complete calculator. Ideas: resource levelling (no more than two technicians per day), a Gantt chart in text or HTML, three-point estimates per activity with the project's standard deviation, or a Monte Carlo simulation of the finish date.
@@file schedule.py
"""Critical path and earned value: the two calculations every PMP candidate must master."""
from dataclasses import dataclass, field


@dataclass
class Activity:
    id: str
    duration: int
    preds: list[str] = field(default_factory=list)
    es: int = 0
    ef: int = 0
    ls: int = 0
    lf: int = 0

    @property
    def float(self) -> int:
        return self.ls - self.es


def topo_order(acts: dict[str, Activity]) -> list[str]:
    """Activity IDs so that every activity comes after its predecessors. Raises ValueError on a loop."""
    order, state = [], {}

    def visit(a: str):
        if state.get(a) == "done":
            return
        if state.get(a) == "busy":
            raise ValueError(f"dependency loop at {a}")
        state[a] = "busy"
        for p in acts[a].preds:
            visit(p)
        state[a] = "done"
        order.append(a)

    for a in acts:
        visit(a)
    return order


# HINT: forward pass: ES = the largest EF of the predecessors (0 if none); EF = ES + duration.
# HINT: backward pass in reverse order: LF = the smallest LS of the successors (the project end if none); LS = LF − duration.
def critical_path(acts: dict[str, Activity]) -> tuple[int, list[str]]:
    """Fill in ES, EF, LS and LF for every activity, and return (project duration, the critical
    activities in order: those with zero float)."""
    order = topo_order(acts)
    for a in order:
        act = acts[a]
        act.es = max((acts[p].ef for p in act.preds), default=0)
        act.ef = act.es + act.duration
    end = max(a.ef for a in acts.values())
    succs = {a: [b for b in acts if a in acts[b].preds] for a in acts}
    for a in reversed(order):
        act = acts[a]
        act.lf = min((acts[s].ls for s in succs[a]), default=end)
        act.ls = act.lf - act.duration
    return end, [a for a in order if acts[a].float == 0]


def pert(o: float, m: float, p: float) -> float:
    """The PERT (beta) estimate."""
    return (o + 4 * m + p) / 6


# HINT: one dictionary with every value; round the indexes to 2 decimals and money to whole numbers.
def earned_value(bac: float, pv: float, ev: float, ac: float) -> dict[str, float]:
    """CV, SV, CPI, SPI, EAC (= BAC / CPI), ETC, VAC and TCPI (to finish on the original budget)."""
    cpi, spi = ev / ac, ev / pv
    eac = bac / cpi
    return {
        "CV": round(ev - ac), "SV": round(ev - pv),
        "CPI": round(cpi, 2), "SPI": round(spi, 2),
        "EAC": round(eac), "ETC": round(eac - ac), "VAC": round(bac - eac),
        "TCPI": round((bac - ev) / (bac - ac), 2),
    }


def status(ev: dict[str, float]) -> str:
    cost = "under budget" if ev["CPI"] >= 1 else "over budget"
    time = "ahead of schedule" if ev["SPI"] >= 1 else "behind schedule"
    return f"{cost}, {time}"
@@file test_schedule.py
import unittest

from schedule import Activity, critical_path, earned_value, pert, topo_order


def network():
    # A: 3 → B: 4 → D: 2 ; A → C: 6 → D
    return {
        "A": Activity("A", 3),
        "B": Activity("B", 4, ["A"]),
        "C": Activity("C", 6, ["A"]),
        "D": Activity("D", 2, ["B", "C"]),
    }


class CriticalPath(unittest.TestCase):
    def test_duration_and_path(self):
        acts = network()
        self.assertEqual(critical_path(acts), (11, ["A", "C", "D"]))

    def test_floats(self):
        acts = network()
        critical_path(acts)
        self.assertEqual({a: x.float for a, x in acts.items()}, {"A": 0, "B": 2, "C": 0, "D": 0})
        self.assertEqual((acts["B"].es, acts["B"].ef, acts["B"].ls, acts["B"].lf), (3, 7, 5, 9))

    def test_two_starts(self):
        acts = {"A": Activity("A", 2), "B": Activity("B", 5), "C": Activity("C", 1, ["A", "B"])}
        self.assertEqual(critical_path(acts), (6, ["B", "C"]))
        self.assertEqual(acts["A"].float, 3)

    def test_loop(self):
        with self.assertRaises(ValueError):
            topo_order({"A": Activity("A", 1, ["B"]), "B": Activity("B", 1, ["A"])})


class Estimates(unittest.TestCase):
    def test_pert(self):
        self.assertEqual(pert(4, 6, 14), 7)


class EarnedValue(unittest.TestCase):
    def test_lesson_example(self):
        ev = earned_value(bac=10_000_000, pv=4_000_000, ev=3_000_000, ac=3_750_000)
        self.assertEqual(ev["CPI"], 0.8)
        self.assertEqual(ev["SPI"], 0.75)
        self.assertEqual(ev["CV"], -750_000)
        self.assertEqual(ev["SV"], -1_000_000)
        self.assertEqual(ev["EAC"], 12_500_000)
        self.assertEqual(ev["ETC"], 8_750_000)
        self.assertEqual(ev["VAC"], -2_500_000)
        self.assertEqual(ev["TCPI"], 1.12)


if __name__ == "__main__":
    unittest.main()
@@file main.py
"""`python main.py`: schedule and earned value for the office move project in project.json."""
import json
import sys

from schedule import Activity, critical_path, earned_value, status


def main() -> None:
    data = json.load(open(sys.argv[1] if len(sys.argv) > 1 else "project.json", encoding="utf-8"))
    acts = {a["id"]: Activity(a["id"], a["days"], a.get("after", [])) for a in data["activities"]}
    names = {a["id"]: a["name"] for a in data["activities"]}
    end, path = critical_path(acts)
    print(f"{'ID':<4}{'Activity':<34}{'Dur':>4}{'ES':>4}{'EF':>4}{'LS':>4}{'LF':>4}{'Float':>6}")
    for a in acts.values():
        mark = " ◀ critical" if a.id in path else ""
        print(f"{a.id:<4}{names[a.id]:<34}{a.duration:>4}{a.es:>4}{a.ef:>4}{a.ls:>4}{a.lf:>4}{a.float:>6}{mark}")
    print(f"\nProject duration: {end} days. Critical path: {' → '.join(path)}")
    ev = earned_value(**data["status"])
    print("\nEarned value: " + ", ".join(f"{k} {v:,}" for k, v in ev.items()))
    print("Status:", status(ev))


if __name__ == "__main__":
    sys.stdout.reconfigure(encoding="utf-8")
    main()
@@file project.json
{
  "activities": [
    { "id": "A", "name": "Survey the new office", "days": 3 },
    { "id": "B", "name": "Design the network and Wi-Fi", "days": 5, "after": ["A"] },
    { "id": "C", "name": "Order switches, APs and cabling", "days": 10, "after": ["B"] },
    { "id": "D", "name": "Contract the ISP line", "days": 20, "after": ["A"] },
    { "id": "E", "name": "Install cabling and racks", "days": 6, "after": ["C"] },
    { "id": "F", "name": "Configure and test the network", "days": 4, "after": ["E", "D"] },
    { "id": "G", "name": "Move PCs and printers (weekend)", "days": 2, "after": ["F"] },
    { "id": "H", "name": "Hypercare: on-site support", "days": 5, "after": ["G"] }
  ],
  "status": { "bac": 6000000, "pv": 2400000, "ev": 2100000, "ac": 2500000 }
}
@@text
🎉 The math part of the PMP is now something you've built, not just memorized. In an interview for a PM or team lead role, explaining critical path and CPI with your own project is far more convincing than a definition.

@@lesson xp-slo_budget SLO and Error Budget Tracker
@@text
SREs run on numbers: what share of requests succeeded and were fast enough (the SLIs), whether that meets the SLO, how much error budget is left this month, and how fast it's burning. Compute all of it from an access log, including the multi-window burn-rate alert from Google's SRE workbook.

**You'll practise:** SLI, SLO, error budget · burn rate · parsing logs · time windows

## Milestones

1. **Parse**: read requests.log (time, status, latency_ms).
2. **SLIs**: availability (status < 500) and latency (under 300 ms) per hour.
3. **Budget**: for a 99.5% SLO over 30 days, budget used and left.
4. **Burn rate**: errors in the last 1 h and 6 h divided by the allowed rate; alert at 14.4× and 6×.
5. **Report**: a short text report a manager understands.

## You're done when

- The numbers are right on a log you made by hand, and an artificial outage triggers the alert.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"Explain burn rate and why multi-window alerts reduce false alarms."*
- *"Is 99.9% always better than 99.5%? What does it cost?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project slo_budget SLO and Error Budget Tracker
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# SLO and Error Budget Tracker

SREs run on numbers: what share of requests succeeded and were fast enough (the SLIs), whether that meets the SLO, how much error budget is left this month, and how fast it's burning. Compute all of it from an access log, including the multi-window burn-rate alert from Google's SRE workbook.

## Milestones

- [ ] 1. Parse: read requests.log (time, status, latency_ms).
- [ ] 2. SLIs: availability (status < 500) and latency (under 300 ms) per hour.
- [ ] 3. Budget: for a 99.5% SLO over 30 days, budget used and left.
- [ ] 4. Burn rate: errors in the last 1 h and 6 h divided by the allowed rate; alert at 14.4× and 6×.
- [ ] 5. Report: a short text report a manager understands.

## Done when

- [ ] The numbers are right on a log you made by hand, and an artificial outage triggers the alert.
@@file slo.py
"""SLO report: python slo.py requests.log"""
import sys

SLO = 0.995
WINDOW_DAYS = 30


def parse(path: str) -> list[tuple[str, int, float]]:
    """(iso time, status, latency ms) per request; lines look like: 2026-10-30T10:00:01 200 123.4"""
    out = []
    for line in open(path, encoding="utf-8"):
        t, status, ms = line.split()
        out.append((t, int(status), float(ms)))
    return out


def availability(reqs) -> float:
    # TODO
    return 1.0


if __name__ == "__main__":
    reqs = parse(sys.argv[1] if len(sys.argv) > 1 else "requests.log")
    print(f"availability {availability(reqs):.4%} (SLO {SLO:.1%})")
    # TODO: error budget left, burn rates, latency SLI
@@file requests.log
2026-10-30T10:00:01 200 120.5
2026-10-30T10:00:02 200 98.0
2026-10-30T10:00:03 500 15.2
2026-10-30T10:00:04 200 410.9
2026-10-30T10:00:05 404 30.1
2026-10-30T10:00:06 200 101.7

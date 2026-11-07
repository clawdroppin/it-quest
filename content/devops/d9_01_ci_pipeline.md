@@lesson d9-ci-pipeline SLO Report with a CI Pipeline
@@text
Two DevOps ideas in one project. First, an **SLO report**: read a web server's access log and answer *"are we meeting our objectives, and how much error budget is left?"* Then, a **CI pipeline** on GitHub Actions that tests the code on three Python versions and runs an **SLO gate**: if the latest log shows the budget is spent, the pipeline fails and the deploy is blocked.

```text
2026-10-31T09:00:01Z GET /api/orders 200 123ms     ──▶  requests: 400
2026-10-31T09:13:20Z GET / 503 1630ms                   availability: 99.50% (SLO 99.50%) OK
...                                                     latency: 98.00% under 300 ms (target 95.00%) OK
                                                        p50: 95 ms, p95: 235 ms, p99: 412 ms
                                                        error budget: 0.0 of 2.0 bad requests left (0.0%)
                                                        burn rate: 1.00
```

The sample log is right on the edge: 2 server errors in 400 requests is exactly the 0.5% a 99.5% SLO allows. One more 5xx and the gate fails. You'll do exactly that in the lab.

Rules worth knowing before you start:

- only **5xx** responses count as bad: a 404 is the client's mistake, not an outage;
- **nearest-rank percentile**: sort, take the value at position `ceil(p/100 × n)` (1-based);
- malformed lines are **skipped, not fatal**: real logs always contain junk.
@@project slo_report SLO Report with a CI Pipeline
Write the functions in `slo.py` (the docstrings give the exact formats) until `python -m unittest` passes and `python main.py` prints the report. Then follow `LAB.md`: push it to GitHub, watch the pipeline, protect `main`, and break the build on purpose, twice.
@@file slo.py
"""SLO report: availability, latency and error budget from a web access log.

Log lines look like:
    2026-10-31T09:00:01Z GET /api/orders 200 123ms
"""
import math
from datetime import datetime


def parse_line(line):
    """Parse one log line into a dict, or return None for blank, comment (#) or malformed lines.

    Returns {"time": datetime, "method": str, "path": str, "status": int, "ms": int}.
    The time ends in "Z" (UTC); the latency ends in "ms". Status must be 100-599.
    """
    raise NotImplementedError


def is_good(record):
    """A request is good unless the server failed it (5xx). 4xx is the client's fault."""
    return record["status"] < 500


def availability(records):
    """Fraction of good requests, 0.0-1.0. No requests counts as fully available (1.0)."""
    raise NotImplementedError


def percentile(values, p):
    """Nearest-rank percentile: the smallest value with at least p% of values at or below it.

    percentile([], p) is None. p is 0 < p <= 100.
    """
    raise NotImplementedError


def error_budget(slo, total, bad):
    """How much of the error budget is used.

    allowed = total * (1 - slo) bad requests. Returns
    {"allowed": float, "bad": int, "remaining": float (allowed - bad, can go negative),
     "remaining_pct": float rounded to 1 decimal (100 when allowed is 0 and bad is 0, else 0 when allowed is 0)}.
    """
    raise NotImplementedError


def burn_rate(slo, records):
    """How fast the budget burns: (error rate) / (1 - slo). 1.0 = exactly on budget. 0.0 for no records."""
    raise NotImplementedError


def report(records, slo=0.995, latency_ms=300, latency_target=0.95):
    """Build the report lines and an overall pass/fail.

    Returns (lines, ok). ok is False when availability < slo, OR fewer than latency_target of the
    requests are at or under latency_ms. Line formats (percentages with 2 decimals):
        "requests: 200"
        "availability: 99.50% (SLO 99.50%) OK"            or "... FAIL"
        "latency: 96.00% under 300 ms (target 95.00%) OK" or "... FAIL"
        "p50: 120 ms, p95: 290 ms, p99: 800 ms"
        "error budget: 0.5 of 1.0 bad requests left (50.0%)"
        "burn rate: 1.00"
    """
    raise NotImplementedError
@@file test_slo.py
import unittest
from datetime import datetime

from slo import availability, burn_rate, error_budget, parse_line, percentile, report


def rec(status=200, ms=100):
    return {"time": datetime(2026, 10, 31, 9, 0), "method": "GET", "path": "/", "status": status, "ms": ms}


class ParseTests(unittest.TestCase):
    def test_good_line(self):
        r = parse_line("2026-10-31T09:00:01Z GET /api/orders 200 123ms\n")
        self.assertEqual(r, {"time": datetime(2026, 10, 31, 9, 0, 1), "method": "GET",
                             "path": "/api/orders", "status": 200, "ms": 123})

    def test_skipped_lines(self):
        for line in ["", "   ", "# comment", "garbage", "2026-10-31T09:00:01Z GET / 200",
                     "2026-10-31T09:00:01Z GET / 200 123", "2026-10-31T09:00:01Z GET / abc 123ms",
                     "2026-13-31T09:00:01Z GET / 200 123ms", "2026-10-31T09:00:01Z GET / 700 123ms",
                     "2026-10-31T09:00:01Z GET / 200 -5ms", "2026-10-31 GET / 200 5ms"]:
            with self.subTest(line=line):
                self.assertIsNone(parse_line(line))


class MathTests(unittest.TestCase):
    def test_availability_counts_only_5xx(self):
        recs = [rec(200), rec(404), rec(401), rec(500), rec(503)]
        self.assertAlmostEqual(availability(recs), 3 / 5)
        self.assertEqual(availability([]), 1.0)

    def test_percentile_nearest_rank(self):
        values = [15, 20, 35, 40, 50]
        self.assertEqual(percentile(values, 30), 20)
        self.assertEqual(percentile(values, 40), 20)
        self.assertEqual(percentile(values, 50), 35)
        self.assertEqual(percentile(values, 100), 50)
        self.assertEqual(percentile([3, 1, 2], 1), 1)
        self.assertIsNone(percentile([], 95))

    def test_error_budget(self):
        b = error_budget(0.99, 1000, 4)
        self.assertAlmostEqual(b["allowed"], 10)
        self.assertAlmostEqual(b["remaining"], 6)
        self.assertEqual(b["bad"], 4)
        self.assertEqual(b["remaining_pct"], 60.0)
        self.assertEqual(error_budget(0.99, 1000, 15)["remaining_pct"], -50.0)
        self.assertEqual(error_budget(0.99, 0, 0)["remaining_pct"], 100.0)

    def test_burn_rate(self):
        recs = [rec(200)] * 98 + [rec(500)] * 2   # 2% errors against a 1% budget
        self.assertAlmostEqual(burn_rate(0.99, recs), 2.0)
        self.assertEqual(burn_rate(0.99, []), 0.0)


class ReportTests(unittest.TestCase):
    def test_passing_report(self):
        recs = [rec(200, 100)] * 199 + [rec(500, 900)]
        lines, ok = report(recs, slo=0.99, latency_ms=300, latency_target=0.95)
        self.assertTrue(ok)
        self.assertEqual(lines, [
            "requests: 200",
            "availability: 99.50% (SLO 99.00%) OK",
            "latency: 99.50% under 300 ms (target 95.00%) OK",
            "p50: 100 ms, p95: 100 ms, p99: 100 ms",
            "error budget: 1.0 of 2.0 bad requests left (50.0%)",
            "burn rate: 0.50",
        ])

    def test_availability_fails(self):
        recs = [rec(200)] * 97 + [rec(503)] * 3
        lines, ok = report(recs, slo=0.99)
        self.assertFalse(ok)
        self.assertTrue(lines[1].endswith("FAIL"))

    def test_latency_fails(self):
        recs = [rec(200, 100)] * 90 + [rec(200, 1000)] * 10
        lines, ok = report(recs, slo=0.99, latency_ms=300, latency_target=0.95)
        self.assertFalse(ok)
        self.assertEqual(lines[2], "latency: 90.00% under 300 ms (target 95.00%) FAIL")
        self.assertEqual(lines[3], "p50: 100 ms, p95: 1000 ms, p99: 1000 ms")

    def test_exactly_on_target_passes(self):
        recs = [rec(200)] * 199 + [rec(500)]
        self.assertTrue(report(recs, slo=0.995)[1])

    def test_empty(self):
        lines, ok = report([])
        self.assertTrue(ok)
        self.assertEqual(lines[0], "requests: 0")
        self.assertEqual(lines[3], "p50: -, p95: -, p99: -")


if __name__ == "__main__":
    unittest.main()
@@file main.py
"""Print the SLO report for a log file. Exit code 1 when the SLO is missed, so CI can block a deploy.

    python main.py                       (access.log, SLO 99.5%, 95% of requests under 300 ms)
    python main.py other.log --slo 0.999 --latency-ms 200
"""
import argparse
import sys

from slo import parse_line, report


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("log", nargs="?", default="access.log")
    ap.add_argument("--slo", type=float, default=0.995, help="availability objective, e.g. 0.995")
    ap.add_argument("--latency-ms", type=int, default=300)
    ap.add_argument("--latency-target", type=float, default=0.95)
    args = ap.parse_args(argv)

    with open(args.log, encoding="utf-8") as f:
        lines = list(f)
    records = [r for r in map(parse_line, lines) if r]
    skipped = sum(1 for line in lines if line.strip() and not line.startswith("#")) - len(records)

    out, ok = report(records, args.slo, args.latency_ms, args.latency_target)
    print("\n".join(out))
    if skipped:
        print(f"({skipped} malformed line(s) skipped)")
    print("SLO met" if ok else "SLO MISSED")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
@@file access.log
# sample access log for the SLO report: 2026-10-31, 09:00-09:40 UTC
2026-10-31T09:00:03Z GET / 200 77ms
2026-10-31T09:00:06Z GET /static/app.js 200 109ms
2026-10-31T09:00:08Z GET / 200 55ms
2026-10-31T09:00:18Z GET /api/orders 200 133ms
2026-10-31T09:00:27Z POST /login 304 129ms
2026-10-31T09:00:35Z GET /api/orders/42 200 35ms
2026-10-31T09:00:38Z GET / 200 82ms
2026-10-31T09:00:44Z GET / 200 316ms
2026-10-31T09:00:52Z GET / 200 208ms
2026-10-31T09:00:59Z POST /login 200 183ms
2026-10-31T09:01:02Z GET /api/orders 304 82ms
2026-10-31T09:01:09Z GET /api/orders 200 70ms
2026-10-31T09:01:12Z GET /login 200 66ms
2026-10-31T09:01:18Z GET /static/app.js 200 69ms
2026-10-31T09:01:20Z GET /api/orders 201 153ms
2026-10-31T09:01:23Z GET /api/orders 304 122ms
2026-10-31T09:01:28Z GET /static/app.js 200 250ms
2026-10-31T09:01:32Z GET /api/orders/42 200 156ms
2026-10-31T09:01:40Z POST /login 200 40ms
2026-10-31T09:01:48Z POST /login 200 139ms
2026-10-31T09:01:54Z GET /static/app.js 200 80ms
2026-10-31T09:01:56Z GET /static/app.js 304 66ms
2026-10-31T09:02:00Z GET /login 304 61ms
2026-10-31T09:02:05Z GET /api/orders 200 117ms
2026-10-31T09:02:08Z GET /api/orders/42 304 150ms
2026-10-31T09:02:10Z GET /api/orders 304 98ms
2026-10-31T09:02:20Z GET /api/orders 200 94ms
2026-10-31T09:02:26Z GET /login 200 58ms
2026-10-31T09:02:31Z GET /static/app.js 200 89ms
2026-10-31T09:02:34Z GET /api/orders 200 70ms
2026-10-31T09:02:39Z POST /login 200 115ms
2026-10-31T09:02:41Z GET /api/orders/42 200 50ms
2026-10-31T09:02:46Z POST /login 404 111ms
2026-10-31T09:02:55Z GET /api/users/me 200 54ms
2026-10-31T09:03:03Z GET /api/users/me 200 181ms
2026-10-31T09:03:06Z GET / 200 36ms
2026-10-31T09:03:11Z GET /api/orders 200 92ms
2026-10-31T09:03:17Z GET /api/users/me 200 190ms
2026-10-31T09:03:27Z GET / 200 142ms
2026-10-31T09:03:36Z GET /api/users/me 200 90ms
2026-10-31T09:03:44Z GET / 401 39ms
2026-10-31T09:03:53Z GET /api/orders/42 200 117ms
2026-10-31T09:03:57Z GET /api/orders 200 114ms
2026-10-31T09:03:59Z GET / 200 101ms
2026-10-31T09:04:03Z GET / 404 63ms
2026-10-31T09:04:06Z GET /static/app.js 304 24ms
2026-10-31T09:04:11Z GET /login 200 84ms
2026-10-31T09:04:21Z GET /api/orders/42 404 40ms
2026-10-31T09:04:26Z GET /api/orders/42 200 122ms
2026-10-31T09:04:33Z GET / 200 126ms
2026-10-31T09:04:36Z GET / 200 100ms
2026-10-31T09:04:43Z GET / 304 87ms
2026-10-31T09:04:53Z GET /static/app.js 200 107ms
2026-10-31T09:05:03Z GET /api/orders/42 404 127ms
2026-10-31T09:05:09Z POST /login 200 296ms
2026-10-31T09:05:19Z GET /api/users/me 200 195ms
2026-10-31T09:05:21Z GET / 200 59ms
2026-10-31T09:05:29Z POST /login 200 126ms
2026-10-31T09:05:39Z GET /api/orders 200 53ms
2026-10-31T09:05:41Z GET /api/orders/42 200 78ms
2026-10-31T09:05:51Z GET /api/users/me 404 75ms
2026-10-31T09:05:55Z GET /api/users/me 200 134ms
2026-10-31T09:06:03Z GET /static/app.js 304 143ms
2026-10-31T09:06:06Z GET /api/users/me 304 147ms
2026-10-31T09:06:15Z GET /api/orders/42 200 138ms
2026-10-31T09:06:17Z GET /static/app.js 200 33ms
2026-10-31T09:06:23Z GET /api/orders/42 200 165ms
2026-10-31T09:06:26Z GET /api/orders/42 200 170ms
2026-10-31T09:06:29Z POST /login 200 66ms
2026-10-31T09:06:39Z GET / 200 108ms
2026-10-31T09:06:47Z GET / 200 230ms
2026-10-31T09:06:52Z GET /api/orders/42 200 136ms
2026-10-31T09:06:55Z GET /static/app.js 304 101ms
2026-10-31T09:07:02Z GET /api/users/me 200 105ms
2026-10-31T09:07:10Z GET /static/app.js 200 51ms
2026-10-31T09:07:18Z GET /login 200 31ms
2026-10-31T09:07:25Z GET /api/users/me 200 135ms
2026-10-31T09:07:29Z GET /static/app.js 200 103ms
2026-10-31T09:07:36Z GET / 201 192ms
2026-10-31T09:07:41Z GET /api/orders 200 150ms
2026-10-31T09:07:50Z GET /api/users/me 304 125ms
2026-10-31T09:07:58Z GET /api/users/me 200 37ms
2026-10-31T09:08:01Z GET /api/users/me 200 234ms
2026-10-31T09:08:10Z GET / 200 55ms
2026-10-31T09:08:20Z GET /login 404 236ms
2026-10-31T09:08:30Z GET /api/users/me 201 148ms
2026-10-31T09:08:39Z GET /api/users/me 200 54ms
2026-10-31T09:08:49Z GET /api/users/me 200 80ms
2026-10-31T09:08:55Z GET /api/orders 200 71ms
2026-10-31T09:08:59Z GET /api/orders 200 175ms
2026-10-31T09:09:02Z GET /api/users/me 200 105ms
2026-10-31T09:09:07Z GET /api/users/me 200 112ms
2026-10-31T09:09:14Z GET /api/orders/42 200 75ms
2026-10-31T09:09:24Z GET /login 200 68ms
2026-10-31T09:09:32Z GET /api/orders/42 200 251ms
2026-10-31T09:09:41Z GET /api/orders 401 136ms
2026-10-31T09:09:43Z GET / 200 55ms
2026-10-31T09:09:45Z GET /api/orders/42 200 68ms
2026-10-31T09:09:53Z GET /api/orders/42 200 145ms
2026-10-31T09:09:56Z GET /api/users/me 200 68ms
2026-10-31T09:09:58Z GET / 404 70ms
2026-10-31T09:10:01Z GET /login 200 71ms
2026-10-31T09:10:04Z GET /api/orders 404 60ms
2026-10-31T09:10:12Z GET /api/users/me 404 52ms
2026-10-31T09:10:17Z GET / 200 58ms
2026-10-31T09:10:20Z GET /login 200 90ms
2026-10-31T09:10:30Z GET /static/app.js 200 196ms
2026-10-31T09:10:34Z GET /api/users/me 200 104ms
2026-10-31T09:10:44Z GET /api/users/me 200 197ms
2026-10-31T09:10:51Z GET /api/orders 201 65ms
2026-10-31T09:10:59Z GET /api/orders/42 200 143ms
2026-10-31T09:11:04Z GET /api/orders/42 200 31ms
2026-10-31T09:11:10Z POST /login 404 95ms
2026-10-31T09:11:19Z GET /static/app.js 200 95ms
2026-10-31T09:11:25Z GET /api/orders 200 51ms
2026-10-31T09:11:32Z GET /api/users/me 200 106ms
2026-10-31T09:11:41Z GET /api/orders/42 200 43ms
2026-10-31T09:11:48Z GET / 200 36ms
2026-10-31T09:11:54Z GET /static/app.js 200 173ms
2026-10-31T09:11:57Z GET /api/orders 200 83ms
2026-10-31T09:12:07Z GET /api/orders 200 120ms
2026-10-31T09:12:16Z GET / 304 112ms
2026-10-31T09:12:25Z GET /api/orders/42 200 89ms
2026-10-31T09:12:33Z GET /api/users/me 200 111ms
2026-10-31T09:12:37Z GET /login 200 61ms
2026-10-31T09:12:45Z GET /login 200 145ms
2026-10-31T09:12:54Z GET /api/orders/42 304 129ms
2026-10-31T09:12:57Z GET /api/orders 200 79ms
2026-10-31T09:13:02Z GET /api/orders 200 60ms
2026-10-31T09:13:11Z GET /api/orders/42 200 77ms
2026-10-31T09:13:20Z GET / 503 1630ms
2026-10-31T09:13:25Z GET /api/users/me 503 1631ms
2026-10-31T09:13:31Z GET /api/orders 200 65ms
2026-10-31T09:13:37Z GET /api/users/me 200 170ms
2026-10-31T09:13:43Z GET /login 200 212ms
2026-10-31T09:13:50Z POST /login 200 368ms
2026-10-31T09:13:56Z POST /login 200 92ms
2026-10-31T09:14:02Z GET /api/orders 401 73ms
2026-10-31T09:14:05Z GET /api/users/me 200 64ms
2026-10-31T09:14:09Z GET /api/orders/42 200 98ms
2026-10-31T09:14:17Z POST /login 200 78ms
2026-10-31T09:14:25Z GET /login 200 179ms
2026-10-31T09:14:27Z GET /api/orders 200 122ms
2026-10-31T09:14:36Z GET / 200 178ms
2026-10-31T09:14:45Z GET /static/app.js 200 240ms
2026-10-31T09:14:51Z POST /login 200 115ms
2026-10-31T09:14:57Z GET /login 200 96ms
2026-10-31T09:15:06Z POST /login 200 139ms
2026-10-31T09:15:10Z GET / 200 152ms
2026-10-31T09:15:20Z GET /api/orders/42 201 76ms
2026-10-31T09:15:29Z GET /api/orders 200 97ms
2026-10-31T09:15:39Z GET /api/users/me 200 82ms
2026-10-31T09:15:44Z POST /login 304 88ms
2026-10-31T09:15:53Z GET /static/app.js 304 204ms
2026-10-31T09:15:57Z GET /api/users/me 404 84ms
2026-10-31T09:16:06Z GET / 200 87ms
2026-10-31T09:16:14Z GET /static/app.js 404 58ms
2026-10-31T09:16:22Z GET /static/app.js 200 63ms
2026-10-31T09:16:32Z GET /api/users/me 404 261ms
2026-10-31T09:16:41Z POST /login 200 412ms
2026-10-31T09:16:46Z GET /static/app.js 200 120ms
2026-10-31T09:16:50Z GET /static/app.js 200 54ms
2026-10-31T09:16:52Z GET /api/orders/42 200 91ms
2026-10-31T09:17:00Z GET /login 200 48ms
2026-10-31T09:17:08Z GET /login 200 109ms
2026-10-31T09:17:14Z GET /api/users/me 200 58ms
2026-10-31T09:17:21Z POST /login 404 134ms
2026-10-31T09:17:27Z GET /api/orders 401 84ms
2026-10-31T09:17:33Z GET /static/app.js 200 63ms
2026-10-31T09:17:35Z GET /login 201 30ms
2026-10-31T09:17:40Z GET /static/app.js 200 41ms
2026-10-31T09:17:44Z GET /static/app.js 200 111ms
2026-10-31T09:17:53Z GET / 200 123ms
2026-10-31T09:17:59Z GET /api/orders/42 200 72ms
2026-10-31T09:18:09Z GET /api/orders/42 200 30ms
2026-10-31T09:18:18Z GET /api/orders/42 200 85ms
2026-10-31T09:18:21Z GET /api/orders 200 48ms
2026-10-31T09:18:29Z POST /login 200 88ms
2026-10-31T09:18:35Z POST /login 200 70ms
2026-10-31T09:18:44Z GET / 200 175ms
2026-10-31T09:18:49Z GET /static/app.js 200 74ms
2026-10-31T09:18:53Z GET / 404 71ms
2026-10-31T09:18:56Z GET /api/orders 304 56ms
2026-10-31T09:19:06Z GET /api/users/me 200 221ms
2026-10-31T09:19:14Z GET /api/orders/42 200 81ms
2026-10-31T09:19:19Z GET /api/orders/42 200 141ms
2026-10-31T09:19:26Z GET / 200 73ms
2026-10-31T09:19:35Z GET / 200 134ms
2026-10-31T09:19:37Z GET / 200 77ms
2026-10-31T09:19:42Z GET / 201 111ms
2026-10-31T09:19:47Z GET /api/orders/42 200 183ms
2026-10-31T09:19:53Z GET /api/orders 401 118ms
2026-10-31T09:19:56Z GET /static/app.js 304 60ms
2026-10-31T09:20:01Z POST /login 200 108ms
2026-10-31T09:20:10Z GET /api/users/me 200 100ms
2026-10-31T09:20:20Z GET /api/orders 404 192ms
2026-10-31T09:20:29Z GET /static/app.js 404 48ms
2026-10-31T09:20:37Z GET /api/users/me 200 90ms
2026-10-31T09:20:40Z GET / 200 70ms
this line is garbage and should be skipped
2026-10-31T09:20:50Z GET /static/app.js 200 89ms
2026-10-31T09:20:53Z GET /static/app.js 200 294ms
2026-10-31T09:21:03Z GET /static/app.js 304 58ms
2026-10-31T09:21:11Z GET /api/orders/42 201 72ms
2026-10-31T09:21:18Z GET /api/users/me 200 134ms
2026-10-31T09:21:21Z GET / 200 181ms
2026-10-31T09:21:31Z POST /login 200 68ms
2026-10-31T09:21:39Z GET /static/app.js 200 76ms
2026-10-31T09:21:42Z GET /login 200 90ms
2026-10-31T09:21:49Z POST /login 200 138ms
2026-10-31T09:21:59Z GET /login 200 146ms
2026-10-31T09:22:03Z GET /api/orders/42 200 77ms
2026-10-31T09:22:05Z GET /api/orders 304 109ms
2026-10-31T09:22:09Z GET /static/app.js 200 65ms
2026-10-31T09:22:16Z GET /api/orders 200 233ms
2026-10-31T09:22:19Z GET / 200 52ms
2026-10-31T09:22:27Z GET /api/users/me 200 90ms
2026-10-31T09:22:37Z GET / 200 35ms
2026-10-31T09:22:46Z GET /login 200 79ms
2026-10-31T09:22:48Z GET /api/orders 200 74ms
2026-10-31T09:22:54Z GET /api/orders/42 200 63ms
2026-10-31T09:22:59Z GET /login 200 80ms
2026-10-31T09:23:01Z GET /api/users/me 200 115ms
2026-10-31T09:23:09Z GET /api/orders/42 200 54ms
2026-10-31T09:23:18Z GET /static/app.js 304 58ms
2026-10-31T09:23:28Z GET / 200 77ms
2026-10-31T09:23:35Z GET /api/orders 200 126ms
2026-10-31T09:23:40Z GET /api/orders/42 200 77ms
2026-10-31T09:23:44Z GET /static/app.js 200 92ms
2026-10-31T09:23:53Z GET /static/app.js 304 115ms
2026-10-31T09:24:00Z GET /static/app.js 200 90ms
2026-10-31T09:24:06Z GET /api/orders/42 200 86ms
2026-10-31T09:24:15Z GET /api/users/me 200 84ms
2026-10-31T09:24:18Z GET /login 200 448ms
2026-10-31T09:24:20Z GET /api/users/me 304 107ms
2026-10-31T09:24:29Z POST /login 200 62ms
2026-10-31T09:24:39Z GET /static/app.js 200 67ms
2026-10-31T09:24:48Z GET /login 200 85ms
2026-10-31T09:24:56Z GET /api/orders/42 200 80ms
2026-10-31T09:25:06Z GET /static/app.js 200 61ms
2026-10-31T09:25:14Z GET /api/orders 200 171ms
2026-10-31T09:25:22Z GET /static/app.js 200 224ms
2026-10-31T09:25:32Z GET / 200 89ms
2026-10-31T09:25:39Z GET /api/orders/42 200 97ms
2026-10-31T09:25:43Z GET /api/orders 200 54ms
2026-10-31T09:25:47Z POST /login 200 136ms
2026-10-31T09:25:56Z GET / 200 121ms
2026-10-31T09:26:05Z POST /login 200 90ms
2026-10-31T09:26:11Z GET /api/users/me 304 43ms
2026-10-31T09:26:20Z GET /api/users/me 200 160ms
2026-10-31T09:26:28Z GET /api/orders 201 134ms
2026-10-31T09:26:34Z GET / 200 204ms
2026-10-31T09:26:37Z GET /static/app.js 201 151ms
2026-10-31T09:26:41Z GET /api/orders/42 200 106ms
2026-10-31T09:26:44Z GET /api/orders 201 88ms
2026-10-31T09:26:51Z GET /api/orders 200 62ms
2026-10-31T09:26:58Z GET / 201 184ms
2026-10-31T09:27:02Z GET / 200 252ms
2026-10-31T09:27:06Z GET /static/app.js 200 284ms
2026-10-31T09:27:08Z GET /api/orders/42 200 116ms
2026-10-31T09:27:14Z GET /api/orders 200 50ms
2026-10-31T09:27:23Z GET / 200 235ms
2026-10-31T09:27:25Z GET /static/app.js 200 76ms
2026-10-31T09:27:33Z GET / 200 67ms
2026-10-31T09:27:36Z GET /login 200 104ms
2026-10-31T09:27:46Z POST /login 304 52ms
2026-10-31T09:27:53Z GET /api/orders/42 200 87ms
2026-10-31T09:27:58Z GET /static/app.js 200 44ms
2026-10-31T09:28:05Z GET /api/orders 200 122ms
2026-10-31T09:28:09Z GET /static/app.js 200 156ms
2026-10-31T09:28:13Z GET / 200 29ms
2026-10-31T09:28:15Z GET /api/orders/42 404 334ms
2026-10-31T09:28:25Z GET /api/users/me 200 130ms
2026-10-31T09:28:32Z GET / 304 84ms
2026-10-31T09:28:39Z GET /api/users/me 304 98ms
2026-10-31T09:28:49Z GET /api/orders/42 201 122ms
2026-10-31T09:28:53Z POST /login 200 80ms
2026-10-31T09:29:02Z GET /login 200 91ms
2026-10-31T09:29:09Z GET /login 200 36ms
2026-10-31T09:29:16Z GET /login 200 151ms
2026-10-31T09:29:18Z GET /api/orders 200 145ms
2026-10-31T09:29:26Z POST /login 200 92ms
2026-10-31T09:29:35Z GET / 200 103ms
2026-10-31T09:29:41Z GET /api/orders 404 118ms
2026-10-31T09:29:44Z GET /api/orders/42 200 124ms
2026-10-31T09:29:50Z GET /static/app.js 304 98ms
2026-10-31T09:29:53Z GET /api/users/me 200 148ms
2026-10-31T09:29:59Z GET /api/orders/42 304 176ms
2026-10-31T09:30:09Z GET /api/orders 200 73ms
2026-10-31T09:30:18Z GET / 200 111ms
2026-10-31T09:30:28Z GET /api/orders 200 59ms
2026-10-31T09:30:37Z POST /login 200 107ms
2026-10-31T09:30:44Z GET /static/app.js 200 93ms
2026-10-31T09:30:48Z GET /login 200 201ms
2026-10-31T09:30:50Z GET / 200 81ms
2026-10-31T09:30:55Z GET /api/orders/42 404 101ms
2026-10-31T09:30:58Z GET /api/orders/42 200 170ms
2026-10-31T09:31:04Z GET /login 200 41ms
2026-10-31T09:31:11Z POST /login 304 136ms
2026-10-31T09:31:16Z POST /login 200 99ms
2026-10-31T09:31:22Z GET /static/app.js 404 92ms
2026-10-31T09:31:30Z GET /api/orders/42 200 120ms
2026-10-31T09:31:38Z GET /api/users/me 200 84ms
2026-10-31T09:31:45Z GET /static/app.js 401 79ms
2026-10-31T09:31:52Z GET /login 200 95ms
2026-10-31T09:31:57Z GET /api/orders 200 74ms
2026-10-31T09:32:05Z GET /api/orders 404 72ms
2026-10-31T09:32:15Z GET /api/users/me 200 215ms
2026-10-31T09:32:20Z GET /api/orders 200 58ms
2026-10-31T09:32:29Z GET /api/orders/42 200 167ms
2026-10-31T09:32:35Z GET /static/app.js 200 91ms
2026-10-31T09:32:37Z GET /login 200 74ms
2026-10-31T09:32:46Z POST /login 401 118ms
2026-10-31T09:32:49Z GET /api/users/me 200 86ms
2026-10-31T09:32:59Z GET /api/users/me 200 75ms
2026-10-31T09:33:08Z GET /api/orders 200 101ms
2026-10-31T09:33:15Z GET /api/orders 200 187ms
2026-10-31T09:33:22Z GET /login 200 128ms
2026-10-31T09:33:24Z GET /api/orders/42 200 56ms
2026-10-31T09:33:30Z GET /static/app.js 304 130ms
2026-10-31T09:33:36Z GET /static/app.js 200 174ms
2026-10-31T09:33:38Z POST /login 200 230ms
2026-10-31T09:33:46Z GET /api/orders 201 59ms
2026-10-31T09:33:49Z GET /api/orders 200 82ms
2026-10-31T09:33:54Z GET /api/users/me 200 119ms
2026-10-31T09:33:56Z GET /static/app.js 200 68ms
2026-10-31T09:34:05Z POST /login 404 92ms
2026-10-31T09:34:15Z GET /api/orders/42 200 267ms
2026-10-31T09:34:21Z POST /login 404 75ms
2026-10-31T09:34:27Z GET /api/orders/42 200 136ms
2026-10-31T09:34:36Z GET /api/orders/42 200 64ms
2026-10-31T09:34:46Z GET /api/users/me 304 86ms
2026-10-31T09:34:50Z GET /api/orders/42 200 130ms
2026-10-31T09:35:00Z GET /api/orders/42 404 120ms
2026-10-31T09:35:05Z GET /api/orders 200 155ms
2026-10-31T09:35:10Z GET / 200 75ms
2026-10-31T09:35:14Z GET / 404 298ms
2026-10-31T09:35:23Z GET /api/users/me 200 135ms
2026-10-31T09:35:29Z GET /static/app.js 200 193ms
2026-10-31T09:35:34Z GET /login 304 64ms
2026-10-31T09:35:36Z GET /api/users/me 200 34ms
2026-10-31T09:35:42Z GET / 200 106ms
2026-10-31T09:35:49Z GET /api/orders/42 200 128ms
2026-10-31T09:35:53Z POST /login 200 66ms
2026-10-31T09:36:02Z GET /static/app.js 200 35ms
2026-10-31T09:36:08Z GET /login 200 129ms
2026-10-31T09:36:16Z GET /api/users/me 200 72ms
2026-10-31T09:36:23Z GET /api/orders 200 83ms
2026-10-31T09:36:29Z GET /api/orders/42 200 152ms
2026-10-31T09:36:31Z GET /static/app.js 200 75ms
2026-10-31T09:36:41Z GET /static/app.js 200 156ms
2026-10-31T09:36:47Z GET /api/orders/42 200 75ms
2026-10-31T09:36:51Z GET /api/users/me 200 116ms
2026-10-31T09:37:01Z GET /api/users/me 200 200ms
2026-10-31T09:37:04Z POST /login 200 43ms
2026-10-31T09:37:06Z GET /login 200 103ms
2026-10-31T09:37:16Z GET /api/users/me 200 54ms
2026-10-31T09:37:19Z POST /login 200 57ms
2026-10-31T09:37:21Z POST /login 201 136ms
2026-10-31T09:37:23Z GET /static/app.js 200 52ms
2026-10-31T09:37:26Z GET /login 404 50ms
2026-10-31T09:37:30Z GET /static/app.js 404 123ms
2026-10-31T09:37:38Z GET /api/users/me 304 173ms
2026-10-31T09:37:48Z GET /api/orders 200 84ms
2026-10-31T09:37:58Z GET /api/users/me 200 127ms
2026-10-31T09:38:00Z GET / 404 88ms
2026-10-31T09:38:08Z POST /login 200 95ms
2026-10-31T09:38:13Z GET /static/app.js 200 466ms
2026-10-31T09:38:15Z GET /api/orders 200 131ms
2026-10-31T09:38:17Z GET /api/orders 200 89ms
2026-10-31T09:38:22Z GET / 200 231ms
2026-10-31T09:38:29Z GET /api/orders 200 40ms
2026-10-31T09:38:34Z GET /api/users/me 200 120ms
2026-10-31T09:38:43Z GET / 404 142ms
2026-10-31T09:38:51Z GET /api/users/me 200 64ms
2026-10-31T09:39:00Z GET /api/orders 201 55ms
2026-10-31T09:39:09Z GET / 200 65ms
2026-10-31T09:39:14Z GET /api/orders/42 200 69ms
2026-10-31T09:39:16Z GET / 200 224ms
2026-10-31T09:39:26Z GET /api/orders 200 159ms
2026-10-31T09:39:28Z GET /api/orders 404 56ms
2026-10-31T09:39:33Z GET /api/users/me 304 121ms
2026-10-31T09:39:41Z GET /api/users/me 200 84ms
2026-10-31T09:39:51Z GET /api/users/me 200 253ms
2026-10-31T09:39:58Z GET /api/orders 200 84ms
2026-10-31T09:40:03Z GET /static/app.js 200 111ms
2026-10-31T09:40:07Z GET /api/orders 200 53ms
2026-10-31T09:40:16Z GET /login 200 30ms
2026-10-31T09:40:25Z GET /api/users/me 200 134ms
2026-10-31T09:40:35Z GET /api/users/me 200 226ms
2026-10-31T09:40:38Z GET /static/app.js 200 112ms
2026-10-31T09:40:42Z GET / 200 95ms
2026-10-31T09:40:51Z GET /static/app.js 200 214ms
2026-10-31T09:40:55Z GET / 304 102ms
2026-10-31T09:41:05Z GET /api/orders/42 201 71ms
2026-10-31T09:41:12Z GET /api/users/me 200 48ms
2026-10-31T09:41:18Z GET /static/app.js 201 113ms
2026-10-31T09:41:26Z GET /api/users/me 200 94ms
2026-10-31T09:41:30Z GET / 200 76ms
2026-10-31T09:41:33Z GET /login 200 107ms
2026-10-31T09:41:36Z GET /static/app.js 304 218ms
@@file .github/workflows/ci.yml
# GitHub Actions: runs on every push and pull request.
name: ci

on:
  push:
    branches: [main]
  pull_request:

permissions:
  contents: read            # least privilege for the workflow's token

jobs:
  test:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        python-version: ["3.11", "3.12", "3.13"]
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
        with:
          python-version: ${{ matrix.python-version }}
      - name: Byte-compile (catches syntax errors)
        run: python -m compileall -q .
      - name: Unit tests
        run: python -m unittest -v

  slo-gate:
    needs: test               # only if the tests passed
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
        with:
          python-version: "3.12"
      - name: Check the SLO on the latest access log
        run: python main.py access.log --slo 0.995
      - name: Save the report
        if: always()
        run: python main.py access.log > slo-report.txt || true
      - uses: actions/upload-artifact@v4
        if: always()
        with:
          name: slo-report
          path: slo-report.txt
@@file .gitignore
__pycache__/
*.pyc
.venv/
slo-report.txt
@@file LAB.md
# CI Pipeline Lab

You need **Git** and a free **GitHub** account. (GitLab CI or Azure Pipelines work the same way,
with different YAML.)

## 1. Make it pass locally
```
python -m unittest -v
python main.py
```

## 2. Put it in Git
```
git init -b main
git add .
git status                 (check .gitignore did its job: no __pycache__)
git commit -m "SLO report with CI"
```
Create an empty repository on GitHub (no README), then:
```
git remote add origin https://github.com/<you>/slo-report.git
git push -u origin main
```
Open the **Actions** tab: the `ci` workflow runs `test` on three Python versions, then `slo-gate`.
Download the `slo-report` artifact.

## 3. The pull request workflow
```
git switch -c add-p90
```
Add `p90` to the report line in `slo.py` and update the test that checks that line. Push the branch,
open a **pull request**, and watch the checks run on it.

Settings → Branches → add a **branch protection rule** (or ruleset) for `main`:
*Require status checks to pass* (select `test` and `slo-gate`) and *Require a pull request before merging*.
Merge the PR when it's green.

## 4. Break it on purpose
- On a new branch, break a test (change an expected value). Push, open a PR: the PR can't be merged.
- On another branch, append one more server error to `access.log`:
  `2026-10-31T09:40:00Z GET /api/orders 500 30ms`
  The tests pass, but **slo-gate fails**: the error budget is spent. In a real team this is the
  moment to stop shipping features and work on reliability. That's what an error budget is for.

## 5. Extras
- Add a **linter** step: `pip install ruff` then `ruff check .`
- Add a **scheduled** run (`on: schedule: - cron: "0 0 * * *"`), which is 09:00 in Japan.
- Add a `deploy` job with `environment: production` and a required reviewer.
- Add **Dependabot** (`.github/dependabot.yml`) to keep the `actions/*` versions updated.
@@text
> Tips: `datetime.strptime(stamp, "%Y-%m-%dT%H:%M:%SZ")` raises `ValueError` on bad dates, so catch it along with `int()` failures. `math.ceil(p / 100 * len(values))` is the rank. In `report`, format percentages with `f"{x * 100:.2f}%"`. Floating point: compare with a tiny tolerance (`avail >= slo - 1e-12`), or 398/400 might not count as 99.5%.
@@project slo_report_finished SLO Report with a CI Pipeline: the finished version
The complete project. Next steps: a multi-window burn-rate alert (fast burn over 1 hour, slow burn over 6 hours), per-endpoint SLOs, reading JSON logs, or publishing the report to GitHub Pages from the pipeline.
@@file slo.py
"""SLO report: availability, latency and error budget from a web access log.

Log lines look like:
    2026-10-31T09:00:01Z GET /api/orders 200 123ms
"""
import math
from datetime import datetime


def parse_line(line):
    """Parse one log line into a dict, or return None for blank, comment (#) or malformed lines.

    Returns {"time": datetime, "method": str, "path": str, "status": int, "ms": int}.
    The time ends in "Z" (UTC); the latency ends in "ms". Status must be 100-599.
    """
    line = line.strip()
    if not line or line.startswith("#"):
        return None
    parts = line.split()
    if len(parts) != 5 or not parts[4].endswith("ms"):
        return None
    stamp, method, path, status, ms = parts
    try:
        time = datetime.strptime(stamp, "%Y-%m-%dT%H:%M:%SZ")
        status = int(status)
        ms = int(ms[:-2])
    except ValueError:
        return None
    if not 100 <= status <= 599 or ms < 0:
        return None
    return {"time": time, "method": method, "path": path, "status": status, "ms": ms}


def is_good(record):
    """A request is good unless the server failed it (5xx). 4xx is the client's fault."""
    return record["status"] < 500


def availability(records):
    """Fraction of good requests, 0.0-1.0. No requests counts as fully available (1.0)."""
    if not records:
        return 1.0
    return sum(1 for r in records if is_good(r)) / len(records)


def percentile(values, p):
    """Nearest-rank percentile: the smallest value with at least p% of values at or below it.

    percentile([], p) is None. p is 0 < p <= 100.
    """
    if not values:
        return None
    ordered = sorted(values)
    rank = math.ceil(p / 100 * len(ordered))
    return ordered[max(rank, 1) - 1]


def error_budget(slo, total, bad):
    """How much of the error budget is used.

    allowed = total * (1 - slo) bad requests. Returns
    {"allowed": float, "bad": int, "remaining": float (allowed - bad, can go negative),
     "remaining_pct": float rounded to 1 decimal (100 when allowed is 0 and bad is 0, else 0 when allowed is 0)}.
    """
    allowed = total * (1 - slo)
    remaining = allowed - bad
    if allowed == 0:
        pct = 100.0 if bad == 0 else 0.0
    else:
        pct = round(remaining / allowed * 100, 1)
    return {"allowed": allowed, "bad": bad, "remaining": remaining, "remaining_pct": pct}


def burn_rate(slo, records):
    """How fast the budget burns: (error rate) / (1 - slo). 1.0 = exactly on budget. 0.0 for no records."""
    if not records:
        return 0.0
    error_rate = 1 - availability(records)
    return error_rate / (1 - slo)


def report(records, slo=0.995, latency_ms=300, latency_target=0.95):
    """Build the report lines and an overall pass/fail.

    Returns (lines, ok). ok is False when availability < slo, OR fewer than latency_target of the
    requests are at or under latency_ms. Line formats (percentages with 2 decimals):
        "requests: 200"
        "availability: 99.50% (SLO 99.50%) OK"            or "... FAIL"
        "latency: 96.00% under 300 ms (target 95.00%) OK" or "... FAIL"
        "p50: 120 ms, p95: 290 ms, p99: 800 ms"
        "error budget: 0.5 of 1.0 bad requests left (50.0%)"
        "burn rate: 1.00"
    """
    total = len(records)
    bad = sum(1 for r in records if not is_good(r))
    avail = availability(records)
    fast = sum(1 for r in records if r["ms"] <= latency_ms) / total if total else 1.0
    lat = [r["ms"] for r in records]
    budget = error_budget(slo, total, bad)
    avail_ok = avail >= slo - 1e-12
    lat_ok = fast >= latency_target - 1e-12

    def p(v):
        return "-" if v is None else f"{v} ms"

    lines = [
        f"requests: {total}",
        f"availability: {avail * 100:.2f}% (SLO {slo * 100:.2f}%) {'OK' if avail_ok else 'FAIL'}",
        f"latency: {fast * 100:.2f}% under {latency_ms} ms (target {latency_target * 100:.2f}%) {'OK' if lat_ok else 'FAIL'}",
        f"p50: {p(percentile(lat, 50))}, p95: {p(percentile(lat, 95))}, p99: {p(percentile(lat, 99))}",
        f"error budget: {budget['remaining']:.1f} of {budget['allowed']:.1f} bad requests left ({budget['remaining_pct']}%)",
        f"burn rate: {burn_rate(slo, records):.2f}",
    ]
    return lines, avail_ok and lat_ok
@@file test_slo.py
import unittest
from datetime import datetime

from slo import availability, burn_rate, error_budget, parse_line, percentile, report


def rec(status=200, ms=100):
    return {"time": datetime(2026, 10, 31, 9, 0), "method": "GET", "path": "/", "status": status, "ms": ms}


class ParseTests(unittest.TestCase):
    def test_good_line(self):
        r = parse_line("2026-10-31T09:00:01Z GET /api/orders 200 123ms\n")
        self.assertEqual(r, {"time": datetime(2026, 10, 31, 9, 0, 1), "method": "GET",
                             "path": "/api/orders", "status": 200, "ms": 123})

    def test_skipped_lines(self):
        for line in ["", "   ", "# comment", "garbage", "2026-10-31T09:00:01Z GET / 200",
                     "2026-10-31T09:00:01Z GET / 200 123", "2026-10-31T09:00:01Z GET / abc 123ms",
                     "2026-13-31T09:00:01Z GET / 200 123ms", "2026-10-31T09:00:01Z GET / 700 123ms",
                     "2026-10-31T09:00:01Z GET / 200 -5ms", "2026-10-31 GET / 200 5ms"]:
            with self.subTest(line=line):
                self.assertIsNone(parse_line(line))


class MathTests(unittest.TestCase):
    def test_availability_counts_only_5xx(self):
        recs = [rec(200), rec(404), rec(401), rec(500), rec(503)]
        self.assertAlmostEqual(availability(recs), 3 / 5)
        self.assertEqual(availability([]), 1.0)

    def test_percentile_nearest_rank(self):
        values = [15, 20, 35, 40, 50]
        self.assertEqual(percentile(values, 30), 20)
        self.assertEqual(percentile(values, 40), 20)
        self.assertEqual(percentile(values, 50), 35)
        self.assertEqual(percentile(values, 100), 50)
        self.assertEqual(percentile([3, 1, 2], 1), 1)
        self.assertIsNone(percentile([], 95))

    def test_error_budget(self):
        b = error_budget(0.99, 1000, 4)
        self.assertAlmostEqual(b["allowed"], 10)
        self.assertAlmostEqual(b["remaining"], 6)
        self.assertEqual(b["bad"], 4)
        self.assertEqual(b["remaining_pct"], 60.0)
        self.assertEqual(error_budget(0.99, 1000, 15)["remaining_pct"], -50.0)
        self.assertEqual(error_budget(0.99, 0, 0)["remaining_pct"], 100.0)

    def test_burn_rate(self):
        recs = [rec(200)] * 98 + [rec(500)] * 2   # 2% errors against a 1% budget
        self.assertAlmostEqual(burn_rate(0.99, recs), 2.0)
        self.assertEqual(burn_rate(0.99, []), 0.0)


class ReportTests(unittest.TestCase):
    def test_passing_report(self):
        recs = [rec(200, 100)] * 199 + [rec(500, 900)]
        lines, ok = report(recs, slo=0.99, latency_ms=300, latency_target=0.95)
        self.assertTrue(ok)
        self.assertEqual(lines, [
            "requests: 200",
            "availability: 99.50% (SLO 99.00%) OK",
            "latency: 99.50% under 300 ms (target 95.00%) OK",
            "p50: 100 ms, p95: 100 ms, p99: 100 ms",
            "error budget: 1.0 of 2.0 bad requests left (50.0%)",
            "burn rate: 0.50",
        ])

    def test_availability_fails(self):
        recs = [rec(200)] * 97 + [rec(503)] * 3
        lines, ok = report(recs, slo=0.99)
        self.assertFalse(ok)
        self.assertTrue(lines[1].endswith("FAIL"))

    def test_latency_fails(self):
        recs = [rec(200, 100)] * 90 + [rec(200, 1000)] * 10
        lines, ok = report(recs, slo=0.99, latency_ms=300, latency_target=0.95)
        self.assertFalse(ok)
        self.assertEqual(lines[2], "latency: 90.00% under 300 ms (target 95.00%) FAIL")
        self.assertEqual(lines[3], "p50: 100 ms, p95: 1000 ms, p99: 1000 ms")

    def test_exactly_on_target_passes(self):
        recs = [rec(200)] * 199 + [rec(500)]
        self.assertTrue(report(recs, slo=0.995)[1])

    def test_empty(self):
        lines, ok = report([])
        self.assertTrue(ok)
        self.assertEqual(lines[0], "requests: 0")
        self.assertEqual(lines[3], "p50: -, p95: -, p99: -")


if __name__ == "__main__":
    unittest.main()
@@file main.py
"""Print the SLO report for a log file. Exit code 1 when the SLO is missed, so CI can block a deploy.

    python main.py                       (access.log, SLO 99.5%, 95% of requests under 300 ms)
    python main.py other.log --slo 0.999 --latency-ms 200
"""
import argparse
import sys

from slo import parse_line, report


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("log", nargs="?", default="access.log")
    ap.add_argument("--slo", type=float, default=0.995, help="availability objective, e.g. 0.995")
    ap.add_argument("--latency-ms", type=int, default=300)
    ap.add_argument("--latency-target", type=float, default=0.95)
    args = ap.parse_args(argv)

    with open(args.log, encoding="utf-8") as f:
        lines = list(f)
    records = [r for r in map(parse_line, lines) if r]
    skipped = sum(1 for line in lines if line.strip() and not line.startswith("#")) - len(records)

    out, ok = report(records, args.slo, args.latency_ms, args.latency_target)
    print("\n".join(out))
    if skipped:
        print(f"({skipped} malformed line(s) skipped)")
    print("SLO met" if ok else "SLO MISSED")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
@@file access.log
# sample access log for the SLO report: 2026-10-31, 09:00-09:40 UTC
2026-10-31T09:00:03Z GET / 200 77ms
2026-10-31T09:00:06Z GET /static/app.js 200 109ms
2026-10-31T09:00:08Z GET / 200 55ms
2026-10-31T09:00:18Z GET /api/orders 200 133ms
2026-10-31T09:00:27Z POST /login 304 129ms
2026-10-31T09:00:35Z GET /api/orders/42 200 35ms
2026-10-31T09:00:38Z GET / 200 82ms
2026-10-31T09:00:44Z GET / 200 316ms
2026-10-31T09:00:52Z GET / 200 208ms
2026-10-31T09:00:59Z POST /login 200 183ms
2026-10-31T09:01:02Z GET /api/orders 304 82ms
2026-10-31T09:01:09Z GET /api/orders 200 70ms
2026-10-31T09:01:12Z GET /login 200 66ms
2026-10-31T09:01:18Z GET /static/app.js 200 69ms
2026-10-31T09:01:20Z GET /api/orders 201 153ms
2026-10-31T09:01:23Z GET /api/orders 304 122ms
2026-10-31T09:01:28Z GET /static/app.js 200 250ms
2026-10-31T09:01:32Z GET /api/orders/42 200 156ms
2026-10-31T09:01:40Z POST /login 200 40ms
2026-10-31T09:01:48Z POST /login 200 139ms
2026-10-31T09:01:54Z GET /static/app.js 200 80ms
2026-10-31T09:01:56Z GET /static/app.js 304 66ms
2026-10-31T09:02:00Z GET /login 304 61ms
2026-10-31T09:02:05Z GET /api/orders 200 117ms
2026-10-31T09:02:08Z GET /api/orders/42 304 150ms
2026-10-31T09:02:10Z GET /api/orders 304 98ms
2026-10-31T09:02:20Z GET /api/orders 200 94ms
2026-10-31T09:02:26Z GET /login 200 58ms
2026-10-31T09:02:31Z GET /static/app.js 200 89ms
2026-10-31T09:02:34Z GET /api/orders 200 70ms
2026-10-31T09:02:39Z POST /login 200 115ms
2026-10-31T09:02:41Z GET /api/orders/42 200 50ms
2026-10-31T09:02:46Z POST /login 404 111ms
2026-10-31T09:02:55Z GET /api/users/me 200 54ms
2026-10-31T09:03:03Z GET /api/users/me 200 181ms
2026-10-31T09:03:06Z GET / 200 36ms
2026-10-31T09:03:11Z GET /api/orders 200 92ms
2026-10-31T09:03:17Z GET /api/users/me 200 190ms
2026-10-31T09:03:27Z GET / 200 142ms
2026-10-31T09:03:36Z GET /api/users/me 200 90ms
2026-10-31T09:03:44Z GET / 401 39ms
2026-10-31T09:03:53Z GET /api/orders/42 200 117ms
2026-10-31T09:03:57Z GET /api/orders 200 114ms
2026-10-31T09:03:59Z GET / 200 101ms
2026-10-31T09:04:03Z GET / 404 63ms
2026-10-31T09:04:06Z GET /static/app.js 304 24ms
2026-10-31T09:04:11Z GET /login 200 84ms
2026-10-31T09:04:21Z GET /api/orders/42 404 40ms
2026-10-31T09:04:26Z GET /api/orders/42 200 122ms
2026-10-31T09:04:33Z GET / 200 126ms
2026-10-31T09:04:36Z GET / 200 100ms
2026-10-31T09:04:43Z GET / 304 87ms
2026-10-31T09:04:53Z GET /static/app.js 200 107ms
2026-10-31T09:05:03Z GET /api/orders/42 404 127ms
2026-10-31T09:05:09Z POST /login 200 296ms
2026-10-31T09:05:19Z GET /api/users/me 200 195ms
2026-10-31T09:05:21Z GET / 200 59ms
2026-10-31T09:05:29Z POST /login 200 126ms
2026-10-31T09:05:39Z GET /api/orders 200 53ms
2026-10-31T09:05:41Z GET /api/orders/42 200 78ms
2026-10-31T09:05:51Z GET /api/users/me 404 75ms
2026-10-31T09:05:55Z GET /api/users/me 200 134ms
2026-10-31T09:06:03Z GET /static/app.js 304 143ms
2026-10-31T09:06:06Z GET /api/users/me 304 147ms
2026-10-31T09:06:15Z GET /api/orders/42 200 138ms
2026-10-31T09:06:17Z GET /static/app.js 200 33ms
2026-10-31T09:06:23Z GET /api/orders/42 200 165ms
2026-10-31T09:06:26Z GET /api/orders/42 200 170ms
2026-10-31T09:06:29Z POST /login 200 66ms
2026-10-31T09:06:39Z GET / 200 108ms
2026-10-31T09:06:47Z GET / 200 230ms
2026-10-31T09:06:52Z GET /api/orders/42 200 136ms
2026-10-31T09:06:55Z GET /static/app.js 304 101ms
2026-10-31T09:07:02Z GET /api/users/me 200 105ms
2026-10-31T09:07:10Z GET /static/app.js 200 51ms
2026-10-31T09:07:18Z GET /login 200 31ms
2026-10-31T09:07:25Z GET /api/users/me 200 135ms
2026-10-31T09:07:29Z GET /static/app.js 200 103ms
2026-10-31T09:07:36Z GET / 201 192ms
2026-10-31T09:07:41Z GET /api/orders 200 150ms
2026-10-31T09:07:50Z GET /api/users/me 304 125ms
2026-10-31T09:07:58Z GET /api/users/me 200 37ms
2026-10-31T09:08:01Z GET /api/users/me 200 234ms
2026-10-31T09:08:10Z GET / 200 55ms
2026-10-31T09:08:20Z GET /login 404 236ms
2026-10-31T09:08:30Z GET /api/users/me 201 148ms
2026-10-31T09:08:39Z GET /api/users/me 200 54ms
2026-10-31T09:08:49Z GET /api/users/me 200 80ms
2026-10-31T09:08:55Z GET /api/orders 200 71ms
2026-10-31T09:08:59Z GET /api/orders 200 175ms
2026-10-31T09:09:02Z GET /api/users/me 200 105ms
2026-10-31T09:09:07Z GET /api/users/me 200 112ms
2026-10-31T09:09:14Z GET /api/orders/42 200 75ms
2026-10-31T09:09:24Z GET /login 200 68ms
2026-10-31T09:09:32Z GET /api/orders/42 200 251ms
2026-10-31T09:09:41Z GET /api/orders 401 136ms
2026-10-31T09:09:43Z GET / 200 55ms
2026-10-31T09:09:45Z GET /api/orders/42 200 68ms
2026-10-31T09:09:53Z GET /api/orders/42 200 145ms
2026-10-31T09:09:56Z GET /api/users/me 200 68ms
2026-10-31T09:09:58Z GET / 404 70ms
2026-10-31T09:10:01Z GET /login 200 71ms
2026-10-31T09:10:04Z GET /api/orders 404 60ms
2026-10-31T09:10:12Z GET /api/users/me 404 52ms
2026-10-31T09:10:17Z GET / 200 58ms
2026-10-31T09:10:20Z GET /login 200 90ms
2026-10-31T09:10:30Z GET /static/app.js 200 196ms
2026-10-31T09:10:34Z GET /api/users/me 200 104ms
2026-10-31T09:10:44Z GET /api/users/me 200 197ms
2026-10-31T09:10:51Z GET /api/orders 201 65ms
2026-10-31T09:10:59Z GET /api/orders/42 200 143ms
2026-10-31T09:11:04Z GET /api/orders/42 200 31ms
2026-10-31T09:11:10Z POST /login 404 95ms
2026-10-31T09:11:19Z GET /static/app.js 200 95ms
2026-10-31T09:11:25Z GET /api/orders 200 51ms
2026-10-31T09:11:32Z GET /api/users/me 200 106ms
2026-10-31T09:11:41Z GET /api/orders/42 200 43ms
2026-10-31T09:11:48Z GET / 200 36ms
2026-10-31T09:11:54Z GET /static/app.js 200 173ms
2026-10-31T09:11:57Z GET /api/orders 200 83ms
2026-10-31T09:12:07Z GET /api/orders 200 120ms
2026-10-31T09:12:16Z GET / 304 112ms
2026-10-31T09:12:25Z GET /api/orders/42 200 89ms
2026-10-31T09:12:33Z GET /api/users/me 200 111ms
2026-10-31T09:12:37Z GET /login 200 61ms
2026-10-31T09:12:45Z GET /login 200 145ms
2026-10-31T09:12:54Z GET /api/orders/42 304 129ms
2026-10-31T09:12:57Z GET /api/orders 200 79ms
2026-10-31T09:13:02Z GET /api/orders 200 60ms
2026-10-31T09:13:11Z GET /api/orders/42 200 77ms
2026-10-31T09:13:20Z GET / 503 1630ms
2026-10-31T09:13:25Z GET /api/users/me 503 1631ms
2026-10-31T09:13:31Z GET /api/orders 200 65ms
2026-10-31T09:13:37Z GET /api/users/me 200 170ms
2026-10-31T09:13:43Z GET /login 200 212ms
2026-10-31T09:13:50Z POST /login 200 368ms
2026-10-31T09:13:56Z POST /login 200 92ms
2026-10-31T09:14:02Z GET /api/orders 401 73ms
2026-10-31T09:14:05Z GET /api/users/me 200 64ms
2026-10-31T09:14:09Z GET /api/orders/42 200 98ms
2026-10-31T09:14:17Z POST /login 200 78ms
2026-10-31T09:14:25Z GET /login 200 179ms
2026-10-31T09:14:27Z GET /api/orders 200 122ms
2026-10-31T09:14:36Z GET / 200 178ms
2026-10-31T09:14:45Z GET /static/app.js 200 240ms
2026-10-31T09:14:51Z POST /login 200 115ms
2026-10-31T09:14:57Z GET /login 200 96ms
2026-10-31T09:15:06Z POST /login 200 139ms
2026-10-31T09:15:10Z GET / 200 152ms
2026-10-31T09:15:20Z GET /api/orders/42 201 76ms
2026-10-31T09:15:29Z GET /api/orders 200 97ms
2026-10-31T09:15:39Z GET /api/users/me 200 82ms
2026-10-31T09:15:44Z POST /login 304 88ms
2026-10-31T09:15:53Z GET /static/app.js 304 204ms
2026-10-31T09:15:57Z GET /api/users/me 404 84ms
2026-10-31T09:16:06Z GET / 200 87ms
2026-10-31T09:16:14Z GET /static/app.js 404 58ms
2026-10-31T09:16:22Z GET /static/app.js 200 63ms
2026-10-31T09:16:32Z GET /api/users/me 404 261ms
2026-10-31T09:16:41Z POST /login 200 412ms
2026-10-31T09:16:46Z GET /static/app.js 200 120ms
2026-10-31T09:16:50Z GET /static/app.js 200 54ms
2026-10-31T09:16:52Z GET /api/orders/42 200 91ms
2026-10-31T09:17:00Z GET /login 200 48ms
2026-10-31T09:17:08Z GET /login 200 109ms
2026-10-31T09:17:14Z GET /api/users/me 200 58ms
2026-10-31T09:17:21Z POST /login 404 134ms
2026-10-31T09:17:27Z GET /api/orders 401 84ms
2026-10-31T09:17:33Z GET /static/app.js 200 63ms
2026-10-31T09:17:35Z GET /login 201 30ms
2026-10-31T09:17:40Z GET /static/app.js 200 41ms
2026-10-31T09:17:44Z GET /static/app.js 200 111ms
2026-10-31T09:17:53Z GET / 200 123ms
2026-10-31T09:17:59Z GET /api/orders/42 200 72ms
2026-10-31T09:18:09Z GET /api/orders/42 200 30ms
2026-10-31T09:18:18Z GET /api/orders/42 200 85ms
2026-10-31T09:18:21Z GET /api/orders 200 48ms
2026-10-31T09:18:29Z POST /login 200 88ms
2026-10-31T09:18:35Z POST /login 200 70ms
2026-10-31T09:18:44Z GET / 200 175ms
2026-10-31T09:18:49Z GET /static/app.js 200 74ms
2026-10-31T09:18:53Z GET / 404 71ms
2026-10-31T09:18:56Z GET /api/orders 304 56ms
2026-10-31T09:19:06Z GET /api/users/me 200 221ms
2026-10-31T09:19:14Z GET /api/orders/42 200 81ms
2026-10-31T09:19:19Z GET /api/orders/42 200 141ms
2026-10-31T09:19:26Z GET / 200 73ms
2026-10-31T09:19:35Z GET / 200 134ms
2026-10-31T09:19:37Z GET / 200 77ms
2026-10-31T09:19:42Z GET / 201 111ms
2026-10-31T09:19:47Z GET /api/orders/42 200 183ms
2026-10-31T09:19:53Z GET /api/orders 401 118ms
2026-10-31T09:19:56Z GET /static/app.js 304 60ms
2026-10-31T09:20:01Z POST /login 200 108ms
2026-10-31T09:20:10Z GET /api/users/me 200 100ms
2026-10-31T09:20:20Z GET /api/orders 404 192ms
2026-10-31T09:20:29Z GET /static/app.js 404 48ms
2026-10-31T09:20:37Z GET /api/users/me 200 90ms
2026-10-31T09:20:40Z GET / 200 70ms
this line is garbage and should be skipped
2026-10-31T09:20:50Z GET /static/app.js 200 89ms
2026-10-31T09:20:53Z GET /static/app.js 200 294ms
2026-10-31T09:21:03Z GET /static/app.js 304 58ms
2026-10-31T09:21:11Z GET /api/orders/42 201 72ms
2026-10-31T09:21:18Z GET /api/users/me 200 134ms
2026-10-31T09:21:21Z GET / 200 181ms
2026-10-31T09:21:31Z POST /login 200 68ms
2026-10-31T09:21:39Z GET /static/app.js 200 76ms
2026-10-31T09:21:42Z GET /login 200 90ms
2026-10-31T09:21:49Z POST /login 200 138ms
2026-10-31T09:21:59Z GET /login 200 146ms
2026-10-31T09:22:03Z GET /api/orders/42 200 77ms
2026-10-31T09:22:05Z GET /api/orders 304 109ms
2026-10-31T09:22:09Z GET /static/app.js 200 65ms
2026-10-31T09:22:16Z GET /api/orders 200 233ms
2026-10-31T09:22:19Z GET / 200 52ms
2026-10-31T09:22:27Z GET /api/users/me 200 90ms
2026-10-31T09:22:37Z GET / 200 35ms
2026-10-31T09:22:46Z GET /login 200 79ms
2026-10-31T09:22:48Z GET /api/orders 200 74ms
2026-10-31T09:22:54Z GET /api/orders/42 200 63ms
2026-10-31T09:22:59Z GET /login 200 80ms
2026-10-31T09:23:01Z GET /api/users/me 200 115ms
2026-10-31T09:23:09Z GET /api/orders/42 200 54ms
2026-10-31T09:23:18Z GET /static/app.js 304 58ms
2026-10-31T09:23:28Z GET / 200 77ms
2026-10-31T09:23:35Z GET /api/orders 200 126ms
2026-10-31T09:23:40Z GET /api/orders/42 200 77ms
2026-10-31T09:23:44Z GET /static/app.js 200 92ms
2026-10-31T09:23:53Z GET /static/app.js 304 115ms
2026-10-31T09:24:00Z GET /static/app.js 200 90ms
2026-10-31T09:24:06Z GET /api/orders/42 200 86ms
2026-10-31T09:24:15Z GET /api/users/me 200 84ms
2026-10-31T09:24:18Z GET /login 200 448ms
2026-10-31T09:24:20Z GET /api/users/me 304 107ms
2026-10-31T09:24:29Z POST /login 200 62ms
2026-10-31T09:24:39Z GET /static/app.js 200 67ms
2026-10-31T09:24:48Z GET /login 200 85ms
2026-10-31T09:24:56Z GET /api/orders/42 200 80ms
2026-10-31T09:25:06Z GET /static/app.js 200 61ms
2026-10-31T09:25:14Z GET /api/orders 200 171ms
2026-10-31T09:25:22Z GET /static/app.js 200 224ms
2026-10-31T09:25:32Z GET / 200 89ms
2026-10-31T09:25:39Z GET /api/orders/42 200 97ms
2026-10-31T09:25:43Z GET /api/orders 200 54ms
2026-10-31T09:25:47Z POST /login 200 136ms
2026-10-31T09:25:56Z GET / 200 121ms
2026-10-31T09:26:05Z POST /login 200 90ms
2026-10-31T09:26:11Z GET /api/users/me 304 43ms
2026-10-31T09:26:20Z GET /api/users/me 200 160ms
2026-10-31T09:26:28Z GET /api/orders 201 134ms
2026-10-31T09:26:34Z GET / 200 204ms
2026-10-31T09:26:37Z GET /static/app.js 201 151ms
2026-10-31T09:26:41Z GET /api/orders/42 200 106ms
2026-10-31T09:26:44Z GET /api/orders 201 88ms
2026-10-31T09:26:51Z GET /api/orders 200 62ms
2026-10-31T09:26:58Z GET / 201 184ms
2026-10-31T09:27:02Z GET / 200 252ms
2026-10-31T09:27:06Z GET /static/app.js 200 284ms
2026-10-31T09:27:08Z GET /api/orders/42 200 116ms
2026-10-31T09:27:14Z GET /api/orders 200 50ms
2026-10-31T09:27:23Z GET / 200 235ms
2026-10-31T09:27:25Z GET /static/app.js 200 76ms
2026-10-31T09:27:33Z GET / 200 67ms
2026-10-31T09:27:36Z GET /login 200 104ms
2026-10-31T09:27:46Z POST /login 304 52ms
2026-10-31T09:27:53Z GET /api/orders/42 200 87ms
2026-10-31T09:27:58Z GET /static/app.js 200 44ms
2026-10-31T09:28:05Z GET /api/orders 200 122ms
2026-10-31T09:28:09Z GET /static/app.js 200 156ms
2026-10-31T09:28:13Z GET / 200 29ms
2026-10-31T09:28:15Z GET /api/orders/42 404 334ms
2026-10-31T09:28:25Z GET /api/users/me 200 130ms
2026-10-31T09:28:32Z GET / 304 84ms
2026-10-31T09:28:39Z GET /api/users/me 304 98ms
2026-10-31T09:28:49Z GET /api/orders/42 201 122ms
2026-10-31T09:28:53Z POST /login 200 80ms
2026-10-31T09:29:02Z GET /login 200 91ms
2026-10-31T09:29:09Z GET /login 200 36ms
2026-10-31T09:29:16Z GET /login 200 151ms
2026-10-31T09:29:18Z GET /api/orders 200 145ms
2026-10-31T09:29:26Z POST /login 200 92ms
2026-10-31T09:29:35Z GET / 200 103ms
2026-10-31T09:29:41Z GET /api/orders 404 118ms
2026-10-31T09:29:44Z GET /api/orders/42 200 124ms
2026-10-31T09:29:50Z GET /static/app.js 304 98ms
2026-10-31T09:29:53Z GET /api/users/me 200 148ms
2026-10-31T09:29:59Z GET /api/orders/42 304 176ms
2026-10-31T09:30:09Z GET /api/orders 200 73ms
2026-10-31T09:30:18Z GET / 200 111ms
2026-10-31T09:30:28Z GET /api/orders 200 59ms
2026-10-31T09:30:37Z POST /login 200 107ms
2026-10-31T09:30:44Z GET /static/app.js 200 93ms
2026-10-31T09:30:48Z GET /login 200 201ms
2026-10-31T09:30:50Z GET / 200 81ms
2026-10-31T09:30:55Z GET /api/orders/42 404 101ms
2026-10-31T09:30:58Z GET /api/orders/42 200 170ms
2026-10-31T09:31:04Z GET /login 200 41ms
2026-10-31T09:31:11Z POST /login 304 136ms
2026-10-31T09:31:16Z POST /login 200 99ms
2026-10-31T09:31:22Z GET /static/app.js 404 92ms
2026-10-31T09:31:30Z GET /api/orders/42 200 120ms
2026-10-31T09:31:38Z GET /api/users/me 200 84ms
2026-10-31T09:31:45Z GET /static/app.js 401 79ms
2026-10-31T09:31:52Z GET /login 200 95ms
2026-10-31T09:31:57Z GET /api/orders 200 74ms
2026-10-31T09:32:05Z GET /api/orders 404 72ms
2026-10-31T09:32:15Z GET /api/users/me 200 215ms
2026-10-31T09:32:20Z GET /api/orders 200 58ms
2026-10-31T09:32:29Z GET /api/orders/42 200 167ms
2026-10-31T09:32:35Z GET /static/app.js 200 91ms
2026-10-31T09:32:37Z GET /login 200 74ms
2026-10-31T09:32:46Z POST /login 401 118ms
2026-10-31T09:32:49Z GET /api/users/me 200 86ms
2026-10-31T09:32:59Z GET /api/users/me 200 75ms
2026-10-31T09:33:08Z GET /api/orders 200 101ms
2026-10-31T09:33:15Z GET /api/orders 200 187ms
2026-10-31T09:33:22Z GET /login 200 128ms
2026-10-31T09:33:24Z GET /api/orders/42 200 56ms
2026-10-31T09:33:30Z GET /static/app.js 304 130ms
2026-10-31T09:33:36Z GET /static/app.js 200 174ms
2026-10-31T09:33:38Z POST /login 200 230ms
2026-10-31T09:33:46Z GET /api/orders 201 59ms
2026-10-31T09:33:49Z GET /api/orders 200 82ms
2026-10-31T09:33:54Z GET /api/users/me 200 119ms
2026-10-31T09:33:56Z GET /static/app.js 200 68ms
2026-10-31T09:34:05Z POST /login 404 92ms
2026-10-31T09:34:15Z GET /api/orders/42 200 267ms
2026-10-31T09:34:21Z POST /login 404 75ms
2026-10-31T09:34:27Z GET /api/orders/42 200 136ms
2026-10-31T09:34:36Z GET /api/orders/42 200 64ms
2026-10-31T09:34:46Z GET /api/users/me 304 86ms
2026-10-31T09:34:50Z GET /api/orders/42 200 130ms
2026-10-31T09:35:00Z GET /api/orders/42 404 120ms
2026-10-31T09:35:05Z GET /api/orders 200 155ms
2026-10-31T09:35:10Z GET / 200 75ms
2026-10-31T09:35:14Z GET / 404 298ms
2026-10-31T09:35:23Z GET /api/users/me 200 135ms
2026-10-31T09:35:29Z GET /static/app.js 200 193ms
2026-10-31T09:35:34Z GET /login 304 64ms
2026-10-31T09:35:36Z GET /api/users/me 200 34ms
2026-10-31T09:35:42Z GET / 200 106ms
2026-10-31T09:35:49Z GET /api/orders/42 200 128ms
2026-10-31T09:35:53Z POST /login 200 66ms
2026-10-31T09:36:02Z GET /static/app.js 200 35ms
2026-10-31T09:36:08Z GET /login 200 129ms
2026-10-31T09:36:16Z GET /api/users/me 200 72ms
2026-10-31T09:36:23Z GET /api/orders 200 83ms
2026-10-31T09:36:29Z GET /api/orders/42 200 152ms
2026-10-31T09:36:31Z GET /static/app.js 200 75ms
2026-10-31T09:36:41Z GET /static/app.js 200 156ms
2026-10-31T09:36:47Z GET /api/orders/42 200 75ms
2026-10-31T09:36:51Z GET /api/users/me 200 116ms
2026-10-31T09:37:01Z GET /api/users/me 200 200ms
2026-10-31T09:37:04Z POST /login 200 43ms
2026-10-31T09:37:06Z GET /login 200 103ms
2026-10-31T09:37:16Z GET /api/users/me 200 54ms
2026-10-31T09:37:19Z POST /login 200 57ms
2026-10-31T09:37:21Z POST /login 201 136ms
2026-10-31T09:37:23Z GET /static/app.js 200 52ms
2026-10-31T09:37:26Z GET /login 404 50ms
2026-10-31T09:37:30Z GET /static/app.js 404 123ms
2026-10-31T09:37:38Z GET /api/users/me 304 173ms
2026-10-31T09:37:48Z GET /api/orders 200 84ms
2026-10-31T09:37:58Z GET /api/users/me 200 127ms
2026-10-31T09:38:00Z GET / 404 88ms
2026-10-31T09:38:08Z POST /login 200 95ms
2026-10-31T09:38:13Z GET /static/app.js 200 466ms
2026-10-31T09:38:15Z GET /api/orders 200 131ms
2026-10-31T09:38:17Z GET /api/orders 200 89ms
2026-10-31T09:38:22Z GET / 200 231ms
2026-10-31T09:38:29Z GET /api/orders 200 40ms
2026-10-31T09:38:34Z GET /api/users/me 200 120ms
2026-10-31T09:38:43Z GET / 404 142ms
2026-10-31T09:38:51Z GET /api/users/me 200 64ms
2026-10-31T09:39:00Z GET /api/orders 201 55ms
2026-10-31T09:39:09Z GET / 200 65ms
2026-10-31T09:39:14Z GET /api/orders/42 200 69ms
2026-10-31T09:39:16Z GET / 200 224ms
2026-10-31T09:39:26Z GET /api/orders 200 159ms
2026-10-31T09:39:28Z GET /api/orders 404 56ms
2026-10-31T09:39:33Z GET /api/users/me 304 121ms
2026-10-31T09:39:41Z GET /api/users/me 200 84ms
2026-10-31T09:39:51Z GET /api/users/me 200 253ms
2026-10-31T09:39:58Z GET /api/orders 200 84ms
2026-10-31T09:40:03Z GET /static/app.js 200 111ms
2026-10-31T09:40:07Z GET /api/orders 200 53ms
2026-10-31T09:40:16Z GET /login 200 30ms
2026-10-31T09:40:25Z GET /api/users/me 200 134ms
2026-10-31T09:40:35Z GET /api/users/me 200 226ms
2026-10-31T09:40:38Z GET /static/app.js 200 112ms
2026-10-31T09:40:42Z GET / 200 95ms
2026-10-31T09:40:51Z GET /static/app.js 200 214ms
2026-10-31T09:40:55Z GET / 304 102ms
2026-10-31T09:41:05Z GET /api/orders/42 201 71ms
2026-10-31T09:41:12Z GET /api/users/me 200 48ms
2026-10-31T09:41:18Z GET /static/app.js 201 113ms
2026-10-31T09:41:26Z GET /api/users/me 200 94ms
2026-10-31T09:41:30Z GET / 200 76ms
2026-10-31T09:41:33Z GET /login 200 107ms
2026-10-31T09:41:36Z GET /static/app.js 304 218ms
@@file .github/workflows/ci.yml
# GitHub Actions: runs on every push and pull request.
name: ci

on:
  push:
    branches: [main]
  pull_request:

permissions:
  contents: read            # least privilege for the workflow's token

jobs:
  test:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        python-version: ["3.11", "3.12", "3.13"]
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
        with:
          python-version: ${{ matrix.python-version }}
      - name: Byte-compile (catches syntax errors)
        run: python -m compileall -q .
      - name: Unit tests
        run: python -m unittest -v

  slo-gate:
    needs: test               # only if the tests passed
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
        with:
          python-version: "3.12"
      - name: Check the SLO on the latest access log
        run: python main.py access.log --slo 0.995
      - name: Save the report
        if: always()
        run: python main.py access.log > slo-report.txt || true
      - uses: actions/upload-artifact@v4
        if: always()
        with:
          name: slo-report
          path: slo-report.txt
@@file .gitignore
__pycache__/
*.pyc
.venv/
slo-report.txt
@@file LAB.md
# CI Pipeline Lab

You need **Git** and a free **GitHub** account. (GitLab CI or Azure Pipelines work the same way,
with different YAML.)

## 1. Make it pass locally
```
python -m unittest -v
python main.py
```

## 2. Put it in Git
```
git init -b main
git add .
git status                 (check .gitignore did its job: no __pycache__)
git commit -m "SLO report with CI"
```
Create an empty repository on GitHub (no README), then:
```
git remote add origin https://github.com/<you>/slo-report.git
git push -u origin main
```
Open the **Actions** tab: the `ci` workflow runs `test` on three Python versions, then `slo-gate`.
Download the `slo-report` artifact.

## 3. The pull request workflow
```
git switch -c add-p90
```
Add `p90` to the report line in `slo.py` and update the test that checks that line. Push the branch,
open a **pull request**, and watch the checks run on it.

Settings → Branches → add a **branch protection rule** (or ruleset) for `main`:
*Require status checks to pass* (select `test` and `slo-gate`) and *Require a pull request before merging*.
Merge the PR when it's green.

## 4. Break it on purpose
- On a new branch, break a test (change an expected value). Push, open a PR: the PR can't be merged.
- On another branch, append one more server error to `access.log`:
  `2026-10-31T09:40:00Z GET /api/orders 500 30ms`
  The tests pass, but **slo-gate fails**: the error budget is spent. In a real team this is the
  moment to stop shipping features and work on reliability. That's what an error budget is for.

## 5. Extras
- Add a **linter** step: `pip install ruff` then `ruff check .`
- Add a **scheduled** run (`on: schedule: - cron: "0 0 * * *"`), which is 09:00 in Japan.
- Add a `deploy` job with `environment: production` and a required reviewer.
- Add **Dependabot** (`.github/dependabot.yml`) to keep the `actions/*` versions updated.

@@drill
What does SLO stand for?
= service level objective
= service-level objective
? For example: 99.9% of requests succeed each month.

@@text
🎉 Git, pull requests, branch protection, a CI matrix, job dependencies, artifacts, least-privilege tokens, SLIs, SLOs, error budgets and burn rates. In an interview for a DevOps or SRE role, walking through this repository is a better answer than any definition.

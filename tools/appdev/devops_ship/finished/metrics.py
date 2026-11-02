"""Prometheus metrics with only the standard library: counters, histograms and the text format.

In a real service you'd use the prometheus-client package; writing it yourself shows exactly
what Prometheus reads from /metrics.
"""
import threading

INF = float("inf")


def format_value(v):
    """Prometheus writes +Inf, and whole numbers without a trailing .0."""
    if v == INF:
        return "+Inf"
    if float(v).is_integer():
        return str(int(v))
    return repr(float(v))


def format_labels(names, values):
    """('route', 'status'), ('/tickets', '200') -> '{route="/tickets",status="200"}'.

    No labels gives "". Inside a value, a backslash becomes \\\\, a double quote \\" and a
    newline \\n, so the line can always be parsed again.
    """
    if not names:
        return ""
    parts = []
    for name, value in zip(names, values):
        escaped = str(value).replace("\\", "\\\\").replace('"', '\\"').replace("\n", "\\n")
        parts.append(f'{name}="{escaped}"')
    return "{" + ",".join(parts) + "}"


class Counter:
    """A number that only goes up, one per combination of label values."""

    def __init__(self, name, help_text, labelnames=()):
        self.name, self.help, self.labelnames = name, help_text, tuple(labelnames)
        self._values = {}
        self._lock = threading.Lock()

    def inc(self, labels=(), amount=1):
        if amount < 0:
            raise ValueError("counters can only go up")
        labels = tuple(str(v) for v in labels)
        if len(labels) != len(self.labelnames):
            raise ValueError(f"{self.name} needs labels {self.labelnames}")
        with self._lock:
            self._values[labels] = self._values.get(labels, 0) + amount

    def value(self, labels=()):
        return self._values.get(tuple(str(v) for v in labels), 0)

    def render(self):
        """The HELP and TYPE lines, then one line per label combination, sorted by labels."""
        lines = [f"# HELP {self.name} {self.help}", f"# TYPE {self.name} counter"]
        with self._lock:
            for labels in sorted(self._values):
                lines.append(f"{self.name}{format_labels(self.labelnames, labels)} {format_value(self._values[labels])}")
        return "\n".join(lines)


class Histogram:
    """Counts observations into cumulative buckets, plus their sum and count."""

    DEFAULT_BUCKETS = (0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1, 2.5, 5)

    def __init__(self, name, help_text, labelnames=(), buckets=DEFAULT_BUCKETS):
        self.name, self.help, self.labelnames = name, help_text, tuple(labelnames)
        self.buckets = tuple(sorted(buckets)) + (INF,)
        self._series = {}  # labels -> {"counts": [per bucket], "sum": float, "count": int}
        self._lock = threading.Lock()

    def observe(self, value, labels=()):
        """Adds one observation. Buckets are cumulative: a value counts in EVERY bucket whose
        upper bound (le) is >= the value, including +Inf."""
        labels = tuple(str(v) for v in labels)
        with self._lock:
            s = self._series.setdefault(labels, {"counts": [0] * len(self.buckets), "sum": 0.0, "count": 0})
            for i, le in enumerate(self.buckets):
                if value <= le:
                    s["counts"][i] += 1
            s["sum"] += value
            s["count"] += 1

    def render(self):
        """HELP and TYPE, then for each label combination (sorted): one _bucket line per bucket
        with an extra le label, then _sum and _count."""
        lines = [f"# HELP {self.name} {self.help}", f"# TYPE {self.name} histogram"]
        with self._lock:
            for labels in sorted(self._series):
                s = self._series[labels]
                for le, count in zip(self.buckets, s["counts"]):
                    lbl = format_labels(self.labelnames + ("le",), labels + (format_value(le),))
                    lines.append(f"{self.name}_bucket{lbl} {count}")
                base = format_labels(self.labelnames, labels)
                lines.append(f"{self.name}_sum{base} {format_value(round(s['sum'], 6))}")
                lines.append(f"{self.name}_count{base} {s['count']}")
        return "\n".join(lines)


class Registry:
    def __init__(self):
        self.metrics = []

    def register(self, metric):
        self.metrics.append(metric)
        return metric

    def render(self):
        """The whole /metrics page: every metric, separated by newlines, ending with one."""
        return "\n".join(m.render() for m in self.metrics) + "\n"

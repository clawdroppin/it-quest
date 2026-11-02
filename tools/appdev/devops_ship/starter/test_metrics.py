import unittest

from metrics import Counter, Histogram, Registry, format_labels, format_value


class FormatTest(unittest.TestCase):
    def test_values(self):
        self.assertEqual(format_value(3), "3")
        self.assertEqual(format_value(2.0), "2")
        self.assertEqual(format_value(0.25), "0.25")
        self.assertEqual(format_value(float("inf")), "+Inf")

    def test_labels(self):
        self.assertEqual(format_labels((), ()), "")
        self.assertEqual(format_labels(("route", "status"), ("/tickets", "200")), '{route="/tickets",status="200"}')

    def test_label_escaping(self):
        self.assertEqual(format_labels(("msg",), ('say "hi"\\n',)), '{msg="say \\"hi\\"\\\\n"}')
        self.assertEqual(format_labels(("msg",), ("two\nlines",)), '{msg="two\\nlines"}')


class CounterTest(unittest.TestCase):
    def test_render(self):
        c = Counter("http_requests_total", "Requests.", ["route", "status"])
        c.inc(("/tickets", 201))
        c.inc(("/healthz", 200), 2)
        c.inc(("/tickets", 201))
        self.assertEqual(c.render(), "\n".join([
            "# HELP http_requests_total Requests.",
            "# TYPE http_requests_total counter",
            'http_requests_total{route="/healthz",status="200"} 2',
            'http_requests_total{route="/tickets",status="201"} 2',
        ]))

    def test_no_labels_and_no_data(self):
        c = Counter("jobs_total", "Jobs.")
        self.assertEqual(c.render(), "# HELP jobs_total Jobs.\n# TYPE jobs_total counter")
        c.inc()
        self.assertTrue(c.render().endswith("\njobs_total 1"))

    def test_only_up(self):
        c = Counter("x_total", "X.")
        with self.assertRaises(ValueError):
            c.inc(amount=-1)
        with self.assertRaises(ValueError):
            c.inc(("unexpected",))


class HistogramTest(unittest.TestCase):
    def test_cumulative_buckets(self):
        h = Histogram("req_seconds", "Latency.", ["route"], buckets=(0.1, 0.5))
        for v in (0.05, 0.2, 0.3, 0.7):
            h.observe(v, ("/tickets",))
        self.assertEqual(h.render(), "\n".join([
            "# HELP req_seconds Latency.",
            "# TYPE req_seconds histogram",
            'req_seconds_bucket{route="/tickets",le="0.1"} 1',
            'req_seconds_bucket{route="/tickets",le="0.5"} 3',
            'req_seconds_bucket{route="/tickets",le="+Inf"} 4',
            'req_seconds_sum{route="/tickets"} 1.25',
            'req_seconds_count{route="/tickets"} 4',
        ]))

    def test_boundary_counts_in_its_bucket(self):
        h = Histogram("x", "X.", buckets=(1,))
        h.observe(1)
        self.assertIn('x_bucket{le="1"} 1', h.render())


class RegistryTest(unittest.TestCase):
    def test_page(self):
        r = Registry()
        r.register(Counter("a_total", "A.")).inc()
        r.register(Counter("b_total", "B."))
        page = r.render()
        self.assertTrue(page.endswith("\n"))
        self.assertIn("a_total 1\n# HELP b_total", page)


if __name__ == "__main__":
    unittest.main()

import json
import threading
import unittest
import urllib.error
import urllib.request

from service import TicketStore, make_server


class ValidateTest(unittest.TestCase):
    def test_good(self):
        self.assertEqual(TicketStore.validate({"title": "Printer jammed", "priority": "high"}), [])
        self.assertEqual(TicketStore.validate({"title": "No priority given"}), [])

    def test_every_problem(self):
        self.assertEqual(TicketStore.validate({"title": "  ", "priority": "asap"}),
                         ["title is required", "priority must be one of: low, normal, high, urgent"])
        self.assertEqual(TicketStore.validate([]), ["the body must be a JSON object"])
        self.assertEqual(TicketStore.validate({"title": "x" * 121}), ["title must be 120 characters or fewer"])


class ApiTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.server = make_server(port=0, log=lambda line: None)   # any free port, quiet
        cls.base = f"http://127.0.0.1:{cls.server.server_address[1]}"
        cls.thread = threading.Thread(target=cls.server.serve_forever, daemon=True)
        cls.thread.start()

    @classmethod
    def tearDownClass(cls):
        cls.server.shutdown()
        cls.server.server_close()

    def call(self, method, path, body=None):
        data = None if body is None else (body if isinstance(body, bytes) else json.dumps(body).encode())
        req = urllib.request.Request(self.base + path, data=data, method=method)
        try:
            with urllib.request.urlopen(req) as resp:
                return resp.status, resp.read().decode()
        except urllib.error.HTTPError as e:
            return e.code, e.read().decode()

    def test_flow(self):
        status, body = self.call("GET", "/healthz")
        self.assertEqual((status, json.loads(body)), (200, {"status": "ok"}))

        status, body = self.call("POST", "/tickets", {"title": "VPN down", "priority": "urgent"})
        self.assertEqual(status, 201)
        self.assertEqual(json.loads(body)["title"], "VPN down")
        self.call("POST", "/tickets", {"title": "New mouse", "priority": "low"})

        status, body = self.call("GET", "/tickets?priority=urgent")
        self.assertEqual([t["title"] for t in json.loads(body)], ["VPN down"])

        self.assertEqual(self.call("POST", "/tickets", {"title": ""})[0], 422)
        self.assertEqual(self.call("POST", "/tickets", b"{not json")[0], 400)
        self.assertEqual(self.call("GET", "/nope")[0], 404)
        self.assertEqual(self.call("POST", "/healthz", {})[0], 405)

        status, page = self.call("GET", "/metrics")
        self.assertEqual(status, 200)
        self.assertIn('http_requests_total{method="POST",route="/tickets",status="201"} 2', page)
        self.assertIn('http_requests_total{method="GET",route="other",status="404"} 1', page)
        self.assertIn('tickets_created_total{priority="urgent"} 1', page)
        self.assertIn('http_request_duration_seconds_bucket{route="/tickets",le="+Inf"}', page)


if __name__ == "__main__":
    unittest.main()

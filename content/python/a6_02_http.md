@@lesson a6-http HTTP and REST APIs
@@concepts networking, json
@@text
Almost everything in modern IT has a **REST API**: cloud platforms, ticketing systems (ServiceNow, Jira), monitoring, Microsoft 365, firewalls. An API lets your script do what you'd otherwise click through in a web page.

## HTTP in one minute

A client sends a **request**: a **method**, a **URL**, **headers**, and sometimes a **body**. The server answers with a **status code**, headers and a body (often JSON).

| Method | Meaning |
|---|---|
| `GET` | read something |
| `POST` | create something / send data |
| `PUT` / `PATCH` | replace / update |
| `DELETE` | delete |

Status codes: `200 OK`, `201 Created`, `204 No Content`, `301/302` redirect, `400 Bad Request`, `401 Unauthorized` (not logged in), `403 Forbidden` (logged in, not allowed), `404 Not Found`, `429 Too Many Requests`, `500 Internal Server Error`, `503 Service Unavailable`.

## A real request, locally

The standard library's `urllib.request` can make HTTP requests. To keep this runnable offline, the example starts a tiny API server on your own machine first:

```python
import json, threading, urllib.request
from http.server import BaseHTTPRequestHandler, HTTPServer

class Api(BaseHTTPRequestHandler):
    def do_GET(self):
        body = json.dumps({"tickets": [{"id": 1, "title": "VPN down", "open": True}, {"id": 2, "title": "New laptop", "open": False}]}).encode()
        self.send_response(200)
        self.send_header("Content-Type", "application/json")
        self.end_headers()
        self.wfile.write(body)

    def log_message(self, *args):
        pass     # keep the example quiet

server = HTTPServer(("127.0.0.1", 0), Api)
threading.Thread(target=server.serve_forever, daemon=True).start()
url = f"http://127.0.0.1:{server.server_port}/api/tickets"

req = urllib.request.Request(url, headers={"Accept": "application/json", "Authorization": "Bearer demo-token"})
with urllib.request.urlopen(req, timeout=5) as resp:
    print(resp.status, resp.headers["Content-Type"])
    data = json.load(resp)
for t in data["tickets"]:
    if t["open"]:
        print("open ticket:", t["id"], t["title"])
server.shutdown()
```

The pattern is always the same: build the request (URL, headers with your **API token**), send it, check the status, parse the JSON, use the data.

## The requests package

In real projects most people install the third-party **requests** package (`pip install requests`), which makes this shorter:

```python,norun
import requests

resp = requests.get("https://api.example.com/tickets", headers={"Authorization": f"Bearer {token}"}, timeout=10)
resp.raise_for_status()          # raise an error for 4xx/5xx
for t in resp.json()["tickets"]:
    print(t["title"])
```

## Good API habits

- Always set a **timeout**.
- Keep tokens **out of your code**: read them from environment variables or a secrets manager.
- Handle `429` and `503` by waiting and retrying (exponential backoff, from the Loops lesson).
- APIs return data in **pages**: keep requesting the next page until there's none.

@@quiz
Your script gets status `403` from an API. What does it mean?
- The server crashed
- The URL doesn't exist
* You're authenticated, but not allowed to do this
- Success
? 401 = who are you? (not authenticated). 403 = I know who you are, and the answer is no.

@@flashcards
HTTP 200 / 201 / 204 :: OK / Created / No Content
HTTP 401 vs 403 :: not authenticated vs not allowed
HTTP 404 :: not found
HTTP 429 :: too many requests: slow down and retry
HTTP 500 / 503 :: server error / service unavailable
GET / POST / PUT / DELETE :: read / create / replace / delete

@@exercise Paginated API
APIs return results page by page. The function `fetch(page)` is given (it pretends to be an API): it returns a dict with `"items"` and `"next"` (the next page number, or `None` on the last page). Write `fetch_all()` that starts at page 1, follows `"next"` until it's `None`, and returns all items in order.
@@starter
PAGES = {1: {"items": ["a", "b"], "next": 2}, 2: {"items": ["c"], "next": 3}, 3: {"items": ["d", "e"], "next": None}}


def fetch(page):
    return PAGES[page]


def fetch_all():
    pass


print(fetch_all())
@@tests
check(fetch_all(), ["a", "b", "c", "d", "e"], "fetch_all()")
PAGES.clear()
PAGES[1] = {"items": [], "next": None}
check(fetch_all(), [], "a single empty page")
@@hint
`page = 1`, `items = []`, then `while page is not None:` — fetch, extend the list, move to `resp["next"]`.
@@solution
PAGES = {1: {"items": ["a", "b"], "next": 2}, 2: {"items": ["c"], "next": 3}, 3: {"items": ["d", "e"], "next": None}}


def fetch(page):
    return PAGES[page]


def fetch_all():
    items = []
    page = 1
    while page is not None:
        resp = fetch(page)
        items.extend(resp["items"])
        page = resp["next"]
    return items


print(fetch_all())

@@exercise Health check
Write `health(url)` that requests `url` with a 3-second timeout and returns `"UP"` if the status is 200, `"DOWN (code)"` for an HTTP error status (`urllib.error.HTTPError` has a `.code`), and `"UNREACHABLE"` if the connection fails (`urllib.error.URLError` or `OSError`). The check runs a local server for you.
@@starter
import urllib.request, urllib.error


def health(url):
    pass
@@tests
import threading as _t
from http.server import BaseHTTPRequestHandler as _H, HTTPServer as _S

class _Api(_H):
    def do_GET(self):
        self.send_response(200 if self.path == "/ok" else 503)
        self.end_headers()
        self.wfile.write(b"x")
    def log_message(self, *a):
        pass

_srv = _S(("127.0.0.1", 0), _Api)
_t.Thread(target=_srv.serve_forever, daemon=True).start()
_base = f"http://127.0.0.1:{_srv.server_port}"
check(health(_base + "/ok"), "UP", "a healthy endpoint")
check(health(_base + "/broken"), "DOWN (503)", "an endpoint answering 503")
_srv.shutdown()
_srv.server_close()
check(health(_base + "/ok"), "UNREACHABLE", "a server that's gone")
@@hint
`try: with urllib.request.urlopen(url, timeout=3) as r: return "UP" if r.status == 200 else ...` — catch `urllib.error.HTTPError as e` **before** `urllib.error.URLError` (HTTPError is a kind of URLError).
@@solution
import urllib.request, urllib.error


def health(url):
    try:
        with urllib.request.urlopen(url, timeout=3) as r:
            return "UP" if r.status == 200 else f"DOWN ({r.status})"
    except urllib.error.HTTPError as e:
        return f"DOWN ({e.code})"
    except (urllib.error.URLError, OSError):
        return "UNREACHABLE"

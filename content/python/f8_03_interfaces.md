@@lesson f8-interfaces Web Apps, Desktop GUIs and Shipping Python
@@text
Your business rules are ready. Now people need a way to use them: a **web API**, a **web page**, a **desktop window**, or all three. Then the app has to be **shipped** to the people who'll run it.

## How Python web apps work: WSGI

Every classic Python web framework (Flask, Django) sits on one simple standard, **WSGI**: a web app is a function that gets the request as a dict and returns the response body.

```python
import json


def app(environ, start_response):
    method, path = environ["REQUEST_METHOD"], environ["PATH_INFO"]
    if method == "GET" and path == "/api/health":
        status, body = "200 OK", {"status": "ok"}
    else:
        status, body = "404 Not Found", {"error": "not found"}
    data = json.dumps(body).encode()
    start_response(status, [("Content-Type", "application/json"), ("Content-Length", str(len(data)))])
    return [data]


# Calling it directly, the way a test (or a server) does:
def fake_start(status, headers):
    print(status, headers[0])

print(app({"REQUEST_METHOD": "GET", "PATH_INFO": "/api/health"}, fake_start))
print(app({"REQUEST_METHOD": "GET", "PATH_INFO": "/admin"}, fake_start))
```

To serve it for real, the standard library has a small server (fine for learning and internal tools; use **gunicorn** or **waitress** in production):

```python,norun
from wsgiref.simple_server import make_server

with make_server("127.0.0.1", 8000, app) as server:
    print("http://localhost:8000/api/health")
    server.serve_forever()
```

Newer async frameworks (FastAPI, Starlette) use the async version of the same idea, called **ASGI**, served by **uvicorn**.

## A router

A router maps `METHOD /path/{param}` patterns to handler functions. Turning `{id}` into a regex group is all the magic there is:

```python
import re

ROUTES = [("GET", "/api/tickets/{id}", "get_ticket"), ("POST", "/api/tickets/{id}/close", "close_ticket")]

def compile_pattern(pattern):
    return re.compile("^" + re.sub(r"\{(\w+)\}", r"(?P<\1>[^/]+)", pattern) + "$")

for method, pattern, name in ROUTES:
    print(pattern, "->", compile_pattern(pattern).pattern)

m = compile_pattern("/api/tickets/{id}/close").match("/api/tickets/42/close")
print(m.groupdict())
```

## The frameworks

| Framework | Style | Choose it for |
|---|---|---|
| **Flask** | small, add what you need | APIs, internal tools, learning how things fit |
| **FastAPI** | async, type hints become validation and API docs | JSON APIs; it generates an OpenAPI page for free |
| **Django** | "batteries included": ORM, migrations, admin site, auth, forms | full websites with users and a database; the admin site alone saves weeks |

The same handler in Flask and FastAPI, so you can see how little changes:

```python,norun
# Flask
@app.post("/api/tickets")
def create_ticket():
    data = request.get_json()
    ticket = service.create(data["title"], data.get("priority", 3))
    return ticket.to_dict(), 201

# FastAPI: the type hints validate the request and document it
class TicketIn(BaseModel):
    title: str
    priority: int = 3

@app.post("/api/tickets", status_code=201)
def create_ticket(body: TicketIn):
    return service.create(body.title, body.priority).to_dict()
```

Notice both handlers are three lines that call `service`. If your business rules live in the service layer, switching frameworks is a small job.

## REST API design in one table

| Request | Meaning | Success |
|---|---|---|
| `GET /api/tickets?status=open` | list (filters in the query string) | 200 + list |
| `GET /api/tickets/42` | one item | 200, or 404 |
| `POST /api/tickets` | create | **201** + the new item |
| `PATCH /api/tickets/42` | change some fields | 200 + the item |
| `DELETE /api/tickets/42` | delete | **204**, no body |
| `POST /api/tickets/42/close` | an action that isn't plain editing | 200 |

Errors return JSON too (`{"error": "title is required"}`) with **400** for bad input, **401** not logged in, **403** not allowed, **404** not found, **409** conflict, **500** for bugs.

## Web security essentials

- **Escape everything** users wrote before putting it in HTML (`html.escape`, or a template engine like Jinja2 that does it by default). Otherwise: XSS.
- **Passwords**: never stored, only hashed with a slow hash: `hashlib.scrypt`, or `argon2`/`bcrypt` packages.
- **Sessions**: a random token (`secrets.token_urlsafe(32)`) in a cookie with `HttpOnly; Secure; SameSite=Lax`.
- **CSRF**: forms that change data need a CSRF token (frameworks do this for you).
- **Authorization**: check on the server that *this* user may change *this* record, every time.

```python
import hashlib
import hmac
import secrets

def hash_password(password: str) -> str:
    salt = secrets.token_bytes(16)
    digest = hashlib.scrypt(password.encode(), salt=salt, n=2**14, r=8, p=1)
    return salt.hex() + ":" + digest.hex()

def check_password(password: str, stored: str) -> bool:
    salt_hex, digest_hex = stored.split(":")
    digest = hashlib.scrypt(password.encode(), salt=bytes.fromhex(salt_hex), n=2**14, r=8, p=1)
    return hmac.compare_digest(digest.hex(), digest_hex)   # constant time: no timing leaks

stored = hash_password("correct horse battery staple")
print(stored[:40] + "...")
print(check_password("correct horse battery staple", stored), check_password("123456", stored))
```

## Desktop GUIs

| Toolkit | Notes |
|---|---|
| **tkinter** | built into Python; plain but zero setup. Good for internal tools |
| **PySide6 / PyQt6** | Qt: professional, native-looking apps with a visual designer |
| **Flet**, **NiceGUI** | write Python, get a modern UI (Flutter or a web page) |
| **Textual** | rich apps that run in the terminal (great over SSH) |

A complete tkinter app. Run it on your own computer; it opens a window, so it can't run inside this lesson:

```python,norun
import tkinter as tk
from tkinter import ttk, messagebox

class TodoApp(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("Todo")
        self.entry = ttk.Entry(self, width=40)
        self.entry.grid(row=0, column=0, padx=8, pady=8)
        self.entry.bind("<Return>", lambda e: self.add())
        ttk.Button(self, text="Add", command=self.add).grid(row=0, column=1, padx=8)
        self.items = tk.Listbox(self, height=10)
        self.items.grid(row=1, column=0, columnspan=2, sticky="nsew", padx=8, pady=(0, 8))
        self.items.bind("<Delete>", lambda e: self.remove())

    def add(self):
        text = self.entry.get().strip()
        if not text:
            messagebox.showwarning("Todo", "Type something first")
            return
        self.items.insert(tk.END, text)
        self.entry.delete(0, tk.END)

    def remove(self):
        for index in reversed(self.items.curselection()):
            self.items.delete(index)

TodoApp().mainloop()
```

GUI rule number one: **never do slow work on the GUI thread** (downloads, big queries). The window freezes. Run it in a thread and send the result back with `self.after(...)`, or use the toolkit's own worker system (Qt's `QThread`).

## Shipping it

| You want to... | Use |
|---|---|
| install it as a command (`task-board`) | `pyproject.toml` with `[project.scripts]`, then `pip install .` or `pipx install .` |
| publish a library | build a wheel (`python -m build`) and upload to PyPI with `twine` |
| give a Windows user one `.exe` | **PyInstaller** (`pyinstaller --onefile --windowed app.py`) or Nuitka |
| run a web app on a server | a **Docker** image, behind a reverse proxy (nginx, Caddy) with HTTPS |

```text
# pyproject.toml
[project]
name = "task-board"
version = "1.0.0"
requires-python = ">=3.11"
dependencies = []

[project.scripts]
task-board = "task_board.cli:main"
```

```text
# Dockerfile
FROM python:3.13-slim
WORKDIR /app
COPY . .
RUN pip install --no-cache-dir . gunicorn
USER nobody
EXPOSE 8000
CMD ["gunicorn", "-b", "0.0.0.0:8000", "task_board.web:application"]
```

@@quiz
In WSGI, what is a web application?
- A class that inherits from `Server`
* A function that takes the request (`environ`) and `start_response`, and returns the body as bytes
- An HTML file with Python inside
- A process that listens on port 80
? That's the whole contract. Flask and Django apps are WSGI functions under the hood, which is why any WSGI server can run them.

@@quiz
What status should `POST /api/tickets` return when it creates a ticket?
- 200 OK
* 201 Created, with the new ticket in the body
- 204 No Content
- 302 Found
? 201 says "a new resource exists now". 204 is typical for DELETE.

@@quiz
Why compare password hashes with `hmac.compare_digest` instead of `==`?
* It takes the same time whether the first or the last character differs, so attackers can't learn anything from timing
- It's faster
- `==` doesn't work on strings
- It also checks the password's strength
? `==` stops at the first different character. Over many attempts, the time differences can leak information.

@@quiz
Your tkinter app freezes for 10 seconds while it downloads a report. What's the fix?
- Use a faster computer
- Call `mainloop()` twice
* Do the download in a background thread and update the window when it's finished (with `after`)
- Make the window smaller
? The GUI thread draws the window and handles clicks. If it's busy downloading, nothing else happens.

@@drill
What does WSGI stand for?
= web server gateway interface
? The standard interface between Python web apps and servers.

@@flashcards
WSGI :: Python's standard interface between web servers and web apps (sync)
ASGI :: the async version, used by FastAPI and Starlette
Flask :: small, flexible web framework
FastAPI :: async API framework; type hints → validation and OpenAPI docs
Django :: full framework: ORM, migrations, admin, auth
gunicorn / uvicorn :: production servers for WSGI / ASGI apps
`hashlib.scrypt` :: a slow, salted password hash in the standard library
`secrets.token_urlsafe` :: random tokens for sessions and reset links
tkinter / PySide6 :: Python's built-in GUI toolkit / Qt for Python
PyInstaller :: bundles a Python app into one .exe
`[project.scripts]` :: pyproject.toml section that installs a command

@@exercise A tiny router
Write `route(method, path)` for these routes, returning `(handler_name, params)`:

- `GET /api/assets` → `"list_assets"`
- `GET /api/assets/{tag}` → `"get_asset"`
- `POST /api/assets/{tag}/assign/{user}` → `"assign_asset"`

A `{name}` matches one path segment (anything but `/`). If no pattern matches the path, return `(None, 404)`. If a pattern matches the path but not the method, return `(None, 405)`.
@@starter
import re

ROUTES = [
    ("GET", "/api/assets", "list_assets"),
    ("GET", "/api/assets/{tag}", "get_asset"),
    ("POST", "/api/assets/{tag}/assign/{user}", "assign_asset"),
]


def route(method, path):
    pass


print(route("POST", "/api/assets/PC-0142/assign/yuki"))
@@tests
check(route("GET", "/api/assets"), ("list_assets", {}), "the list")
check(route("GET", "/api/assets/PC-0142"), ("get_asset", {"tag": "PC-0142"}), "one asset")
check(route("POST", "/api/assets/PC-0142/assign/yuki"), ("assign_asset", {"tag": "PC-0142", "user": "yuki"}), "two params")
check(route("DELETE", "/api/assets/PC-0142"), (None, 405), "known path, wrong method")
check(route("GET", "/api/users"), (None, 404), "unknown path")
check(route("GET", "/api/assets/a/b"), (None, 404), "a param never matches a slash")
@@hint
For each route, build `regex = "^" + re.sub(r"\{(\w+)\}", r"(?P<\1>[^/]+)", pattern) + "$"`. If `re.match(regex, path)` matches and the method is right, return `(name, m.groupdict())`; if it matches with another method, remember that. After the loop return `(None, 405)` or `(None, 404)`.
@@solution
import re

ROUTES = [
    ("GET", "/api/assets", "list_assets"),
    ("GET", "/api/assets/{tag}", "get_asset"),
    ("POST", "/api/assets/{tag}/assign/{user}", "assign_asset"),
]


def route(method, path):
    path_known = False
    for route_method, pattern, name in ROUTES:
        regex = "^" + re.sub(r"\{(\w+)\}", r"(?P<\1>[^/]+)", pattern) + "$"
        m = re.match(regex, path)
        if m:
            if route_method == method:
                return name, m.groupdict()
            path_known = True
    return (None, 405) if path_known else (None, 404)


print(route("POST", "/api/assets/PC-0142/assign/yuki"))

@@exercise Test a WSGI app without a server
The `app` below is a complete WSGI app. Write `call(app, method, path)` that calls it directly (no server) and returns `(status_code, body_dict)`: the status code as an `int` (from `"404 Not Found"` → `404`) and the JSON body decoded.
@@starter
import json


def app(environ, start_response):
    if environ["PATH_INFO"] == "/api/health":
        status, body = "200 OK", {"status": "ok"}
    elif environ["REQUEST_METHOD"] != "GET":
        status, body = "405 Method Not Allowed", {"error": "method not allowed"}
    else:
        status, body = "404 Not Found", {"error": "not found"}
    start_response(status, [("Content-Type", "application/json")])
    return [json.dumps(body).encode()]


def call(app, method, path):
    pass


print(call(app, "GET", "/api/health"))
@@tests
check(call(app, "GET", "/api/health"), (200, {"status": "ok"}), "health check")
check(call(app, "GET", "/nope"), (404, {"error": "not found"}), "unknown path")
check(call(app, "POST", "/nope"), (405, {"error": "method not allowed"}), "wrong method")
@@hint
Make a dict `environ = {"REQUEST_METHOD": method, "PATH_INFO": path}`. Write a nested `start_response(status, headers)` that saves `status` (e.g. in a list or dict defined outside it). Join the returned chunks: `b"".join(app(environ, start_response))`, then `int(status[:3])` and `json.loads(...)`.
@@solution
import json


def app(environ, start_response):
    if environ["PATH_INFO"] == "/api/health":
        status, body = "200 OK", {"status": "ok"}
    elif environ["REQUEST_METHOD"] != "GET":
        status, body = "405 Method Not Allowed", {"error": "method not allowed"}
    else:
        status, body = "404 Not Found", {"error": "not found"}
    start_response(status, [("Content-Type", "application/json")])
    return [json.dumps(body).encode()]


def call(app, method, path):
    seen = {}

    def start_response(status, headers):
        seen["status"] = status

    body = b"".join(app({"REQUEST_METHOD": method, "PATH_INFO": path}, start_response))
    return int(seen["status"][:3]), json.loads(body)


print(call(app, "GET", "/api/health"))

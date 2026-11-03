@@lesson r11-fastapi Modern Web APIs: FastAPI, Pydantic and API Design
@@concepts networking, json, objects
@@text
The application-development module showed how web apps work underneath (WSGI, routing). In practice most new Python APIs are built with **FastAPI**: you write ordinary type-hinted functions and get validation, JSON conversion, async support and interactive documentation for free. This lesson covers it and the design decisions every API needs.

## A complete small API

```python,norun
# pip install "fastapi[standard]"   then:   fastapi dev main.py
from datetime import datetime
from typing import Annotated, Literal
from fastapi import FastAPI, HTTPException, Query, Depends, status
from pydantic import BaseModel, Field

app = FastAPI(title="Asset API", version="1.0")

class AssetIn(BaseModel):                       # what clients send
    tag: str = Field(pattern=r"^[A-Z]{2}-\d{4}$")
    kind: Literal["laptop", "monitor", "phone"]
    owner: str | None = None
    price_eur: Annotated[float, Field(ge=0)] = 0

class AssetOut(AssetIn):                        # what we return
    id: int
    created: datetime

DB: dict[int, AssetOut] = {}

def current_user(token: str = Query(...)) -> str:    # a dependency (real apps read a header)
    if token != "secret":
        raise HTTPException(status.HTTP_401_UNAUTHORIZED, "bad token")
    return "maya"

@app.post("/assets", status_code=201, response_model=AssetOut)
def create_asset(asset: AssetIn, user: str = Depends(current_user)):
    if any(a.tag == asset.tag for a in DB.values()):
        raise HTTPException(409, f"tag {asset.tag} already exists")
    new = AssetOut(**asset.model_dump(), id=len(DB) + 1, created=datetime.now())
    DB[new.id] = new
    return new

@app.get("/assets", response_model=list[AssetOut])
def list_assets(kind: str | None = None, limit: int = Query(20, le=100), offset: int = 0):
    items = [a for a in DB.values() if kind is None or a.kind == kind]
    return items[offset:offset + limit]

@app.get("/assets/{asset_id}", response_model=AssetOut)
def get_asset(asset_id: int):
    if asset_id not in DB:
        raise HTTPException(404, "asset not found")
    return DB[asset_id]
```

What FastAPI does from those type hints:

- **Path parameters** (`asset_id: int`) and **query parameters** (`kind`, `limit`) are parsed and converted; `/assets/abc` returns a 422 error automatically.
- The **request body** (`asset: AssetIn`) is parsed from JSON and validated by **Pydantic**: wrong types, a bad tag pattern or a negative price produce a clear 422 response listing every problem.
- `response_model` filters and serialises the output (fields not in the model never leak).
- **Dependencies** (`Depends(current_user)`) inject shared logic: authentication, database sessions, settings. Tests can override them.
- Interactive docs appear at `/docs` (Swagger UI) and `/redoc`, generated from an **OpenAPI** schema at `/openapi.json`.
- `async def` endpoints run on the event loop (use async database drivers); plain `def` endpoints run in a thread pool.

## Pydantic models

Pydantic turns type hints into a validator and parser. It powers FastAPI, but is just as useful for config files and any JSON you receive:

```python,norun
from pydantic import BaseModel, EmailStr, field_validator, ValidationError

class User(BaseModel):
    name: str
    email: EmailStr
    quota_gb: int = 10

    @field_validator("name")
    @classmethod
    def clean(cls, v: str) -> str:
        return v.strip().lower()

try:
    User.model_validate({"name": " Maya ", "email": "not-an-email", "quota_gb": "lots"})
except ValidationError as e:
    print(e.errors())     # a list: field, message, input; for every problem at once
```

`model_dump()` / `model_dump_json()` serialise; `model_json_schema()` produces JSON Schema; `pydantic-settings` loads settings from environment variables.

## Designing a good API

| Topic | Convention |
|---|---|
| Resources | plural nouns: `/assets`, `/assets/42`, `/assets/42/repairs` |
| Methods | GET read, POST create, PUT replace, PATCH partial update, DELETE remove |
| Status codes | 200 OK, 201 Created, 204 No Content, 400 bad input, 401 not logged in, 403 not allowed, 404 not found, 409 conflict, 422 validation, 429 rate limited, 500 server bug |
| Errors | a consistent JSON shape: `{"error": "not_found", "detail": "asset 42 not found"}` |
| Collections | pagination (`limit` + `offset` or a `cursor`), filters as query parameters, a stable sort |
| Versions | `/v1/...` or a header; never break existing clients silently |
| Idempotency | GET/PUT/DELETE can be repeated safely; for POST accept an `Idempotency-Key` header |
| Time | ISO 8601 with time zone: `2026-11-03T09:00:00Z` |

## Other frameworks

- **Flask**: minimal and mature, huge ecosystem; great for small services and server-rendered pages.
- **Django** (+ Django REST Framework / Ninja): "batteries included": ORM, admin site, auth, migrations. Excellent for database-heavy business apps.
- **Litestar**, **Starlette** (FastAPI's foundation), **aiohttp**.

Run in production behind **uvicorn**/**gunicorn** (multiple worker processes) and a reverse proxy (nginx, Caddy, a cloud load balancer), usually in a container.

@@quiz
What does FastAPI return automatically when a request body fails Pydantic validation?
- 500 Internal Server Error
- 400 with no details
* 422 with a list of every validation problem
- It accepts the data anyway
? Validation errors are reported per field.

@@quiz
What is `Depends(current_user)` for?
* Injecting shared logic like authentication or a database session into endpoints
- Declaring package dependencies
- Making the endpoint async
- Caching responses
? Tests can override dependencies.

@@quiz
Which status code fits "a ticket with that number already exists"?
- 200
- 404
* 409 Conflict
- 500
? 409 signals a conflict with the current state.

@@quiz
Why use `response_model` on an endpoint?
- It's required
* It controls exactly which fields are returned and serialises them, so internal fields don't leak
- It speeds up the database
- It adds authentication
? E.g. never return a password hash.

@@flashcards
FastAPI :: type-hint-driven async web framework with automatic docs
Pydantic :: data validation and parsing from type hints
OpenAPI :: standard machine-readable description of an HTTP API
Path parameter :: part of the URL path: `/assets/{id}`
Query parameter :: after `?`: `/assets?kind=laptop`
422 Unprocessable Content :: request understood but failed validation
Dependency injection (FastAPI) :: `Depends(...)` supplies shared resources to endpoints
Idempotent :: repeating the request has the same effect as doing it once
uvicorn :: ASGI server that runs FastAPI apps

@@exercise Validate like Pydantic
Without installing anything, write a tiny model validator in the Pydantic style. `validate(schema, data)` checks a dict against a schema like:

```text
{"tag": {"type": str, "pattern": r"^[A-Z]{2}-\d{4}$"},
 "kind": {"type": str, "choices": ["laptop", "monitor"]},
 "price": {"type": float, "min": 0, "default": 0.0}}
```

- Return `(clean, errors)`. `clean` has every schema field (defaults filled in); `errors` is a list of `{"field": name, "msg": text}` in schema order.
- Missing without default → `"field required"`.
- `int` values are accepted for `float` fields and converted; otherwise the type must match exactly (`bool` is not an `int`) → `"expected <type name>"`.
- Then `pattern` (use `re.fullmatch`) → `"invalid format"`, `choices` → `"must be one of laptop, monitor"`, `min` → `"must be >= 0"`.
- Unknown fields → `"extra field not allowed"` (after the schema fields).
- If there are errors, `clean` is `None`.
@@starter
import re


def validate(schema, data):
    pass


SCHEMA = {
    "tag": {"type": str, "pattern": r"^[A-Z]{2}-\d{4}$"},
    "kind": {"type": str, "choices": ["laptop", "monitor"]},
    "price": {"type": float, "min": 0, "default": 0.0},
}
print(validate(SCHEMA, {"tag": "LT-0042", "kind": "laptop", "price": 999}))
print(validate(SCHEMA, {"tag": "lt-42", "price": -5, "color": "red"}))
@@tests
SCHEMA = {
    "tag": {"type": str, "pattern": r"^[A-Z]{2}-\d{4}$"},
    "kind": {"type": str, "choices": ["laptop", "monitor"]},
    "price": {"type": float, "min": 0, "default": 0.0},
}
check(validate(SCHEMA, {"tag": "LT-0042", "kind": "laptop", "price": 999}), ({"tag": "LT-0042", "kind": "laptop", "price": 999.0}, []), "valid, int to float")
check(validate(SCHEMA, {"tag": "LT-0042", "kind": "monitor"}), ({"tag": "LT-0042", "kind": "monitor", "price": 0.0}, []), "default")
check(validate(SCHEMA, {"tag": "lt-42", "price": -5, "color": "red"}), (None, [
    {"field": "tag", "msg": "invalid format"},
    {"field": "kind", "msg": "field required"},
    {"field": "price", "msg": "must be >= 0"},
    {"field": "color", "msg": "extra field not allowed"},
]), "errors")
check(validate(SCHEMA, {"tag": 42, "kind": "phone", "price": True})[1], [
    {"field": "tag", "msg": "expected str"},
    {"field": "kind", "msg": "must be one of laptop, monitor"},
    {"field": "price", "msg": "expected float"},
], "type and choice errors")
@@hint
For each `name, rules` in the schema: if missing, use `rules["default"]` if present or record "field required" and `continue`. Type check: `kind = rules["type"]`; `if kind is float and type(value) is int: value = float(value)`; then `if type(value) is not kind:` error and continue. Then check pattern, choices and min in order, stopping at the first error for that field.
@@solution
import re


def validate(schema, data):
    clean, errors = {}, []

    def err(field, msg):
        errors.append({"field": field, "msg": msg})

    for name, rules in schema.items():
        if name not in data:
            if "default" in rules:
                clean[name] = rules["default"]
            else:
                err(name, "field required")
            continue
        value = data[name]
        kind = rules["type"]
        if kind is float and type(value) is int:
            value = float(value)
        if type(value) is not kind:
            err(name, f"expected {kind.__name__}")
            continue
        if "pattern" in rules and not re.fullmatch(rules["pattern"], value):
            err(name, "invalid format")
            continue
        if "choices" in rules and value not in rules["choices"]:
            err(name, "must be one of " + ", ".join(rules["choices"]))
            continue
        if "min" in rules and value < rules["min"]:
            err(name, f"must be >= {rules['min']}")
            continue
        clean[name] = value
    for name in data:
        if name not in schema:
            err(name, "extra field not allowed")
    return (None if errors else clean), errors


SCHEMA = {
    "tag": {"type": str, "pattern": r"^[A-Z]{2}-\d{4}$"},
    "kind": {"type": str, "choices": ["laptop", "monitor"]},
    "price": {"type": float, "min": 0, "default": 0.0},
}
print(validate(SCHEMA, {"tag": "LT-0042", "kind": "laptop", "price": 999}))
print(validate(SCHEMA, {"tag": "lt-42", "price": -5, "color": "red"}))

@@exercise Pagination and filtering
Write the logic behind `GET /assets?kind=laptop&sort=-price&limit=2&cursor=...`:

`list_assets(items, params)` where `items` is a list of dicts with `id`, `kind`, `price`, and `params` is a dict of strings (from the query string):

- `kind` filters (optional)
- `sort`: a field name, prefixed with `-` for descending (default `"id"`); ties broken by `id` ascending; unknown field → `ValueError("cannot sort by <field>")`
- `limit`: default 20, must be 1–100 → else `ValueError("limit must be 1-100")`
- `cursor`: an opaque string from a previous page: the **position** (index into the sorted, filtered list) where the next page starts, encoded as `"p" + str(position)`; invalid → `ValueError("bad cursor")`
- Return `{"items": [...], "next": cursor or None}`
@@starter
def list_assets(items, params):
    pass


ITEMS = [
    {"id": 1, "kind": "laptop", "price": 900},
    {"id": 2, "kind": "monitor", "price": 200},
    {"id": 3, "kind": "laptop", "price": 1500},
    {"id": 4, "kind": "laptop", "price": 900},
]
page = list_assets(ITEMS, {"kind": "laptop", "sort": "-price", "limit": "2"})
print([a["id"] for a in page["items"]], page["next"])
page = list_assets(ITEMS, {"kind": "laptop", "sort": "-price", "limit": "2", "cursor": page["next"]})
print([a["id"] for a in page["items"]], page["next"])
@@expected
[3, 1] p2
[4] None
@@tests
ITEMS = [
    {"id": 1, "kind": "laptop", "price": 900},
    {"id": 2, "kind": "monitor", "price": 200},
    {"id": 3, "kind": "laptop", "price": 1500},
    {"id": 4, "kind": "laptop", "price": 900},
]
check([a["id"] for a in list_assets(ITEMS, {})["items"]], [1, 2, 3, 4], "defaults")
check(list_assets(ITEMS, {"sort": "price", "limit": "3"})["next"], "p3", "next cursor")
check([a["id"] for a in list_assets(ITEMS, {"sort": "price"})["items"]], [2, 1, 4, 3], "ties by id")
for bad, msg in [({"limit": "0"}, "limit must be 1-100"), ({"limit": "abc"}, "limit must be 1-100"), ({"sort": "owner"}, "cannot sort by owner"), ({"cursor": "x9"}, "bad cursor")]:
    try:
        list_assets(ITEMS, bad)
        check(bad, "ValueError", "invalid params")
    except ValueError as e:
        check(str(e), msg, "message")
@@hint
Parse limit with `try: limit = int(params.get("limit", "20"))` and check the range (raise from the `except` too). Sort with `key=lambda a: a["id"]` first, then a stable sort by the field (`reverse=desc`): stable sorting keeps the id order for ties. Cursor: must start with `"p"` and the rest must be digits. `next` is `f"p{start + limit}"` if `start + limit < len(filtered)` else None.
@@solution
def list_assets(items, params):
    try:
        limit = int(params.get("limit", "20"))
    except ValueError:
        raise ValueError("limit must be 1-100") from None
    if not 1 <= limit <= 100:
        raise ValueError("limit must be 1-100")
    sort = params.get("sort", "id")
    desc = sort.startswith("-")
    field = sort.lstrip("-")
    if field not in ("id", "kind", "price"):
        raise ValueError(f"cannot sort by {field}")
    cursor = params.get("cursor")
    start = 0
    if cursor is not None:
        if not (cursor.startswith("p") and cursor[1:].isdigit()):
            raise ValueError("bad cursor")
        start = int(cursor[1:])
    kind = params.get("kind")
    filtered = [a for a in items if kind is None or a["kind"] == kind]
    filtered.sort(key=lambda a: a["id"])
    filtered.sort(key=lambda a: a[field], reverse=desc)
    page = filtered[start:start + limit]
    nxt = f"p{start + limit}" if start + limit < len(filtered) else None
    return {"items": page, "next": nxt}


ITEMS = [
    {"id": 1, "kind": "laptop", "price": 900},
    {"id": 2, "kind": "monitor", "price": 200},
    {"id": 3, "kind": "laptop", "price": 1500},
    {"id": 4, "kind": "laptop", "price": 900},
]
page = list_assets(ITEMS, {"kind": "laptop", "sort": "-price", "limit": "2"})
print([a["id"] for a in page["items"]], page["next"])
page = list_assets(ITEMS, {"kind": "laptop", "sort": "-price", "limit": "2", "cursor": page["next"]})
print([a["id"] for a in page["items"]], page["next"])

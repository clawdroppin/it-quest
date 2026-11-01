OUT = "content/go/h2_01_device_api.md"
BASE = "tools/appdev/go_device_api"
FILES = ["go.mod", "device.go", "store.go", "api.go", "api_test.go", "main.go"]

PARTS = [
    ("text", r'''
@@lesson g9-device-api Device Inventory REST API
@@text
Build a production-shaped REST API in Go with only the standard library: the kind of internal service an IT team runs to keep track of every switch, server, laptop and printer, and that scripts, dashboards and other services call.

```text
$ go run .
level=INFO msg=listening url=http://127.0.0.1:8080 data=devices.json

$ curl -X POST localhost:8080/devices -d '{"hostname":"TK-SW-CORE","ip":"10.0.0.2","kind":"switch"}'
{"id":1,"hostname":"tk-sw-core","ip":"10.0.0.2","kind":"switch"}

$ curl -X POST localhost:8080/devices -d '{"hostname":"bad host","ip":"10.0.0.999","kind":"toaster"}'
{"error":"invalid device: hostname \"bad host\" must be ...","problems":[...3 problems...]}

$ curl 'localhost:8080/devices?kind=switch&q=tk'
[{"id":1,"hostname":"tk-sw-core","ip":"10.0.0.2","kind":"switch"}]
```

(On Windows PowerShell, use `curl.exe`, or `Invoke-RestMethod`.)

## The API

| Request | Response |
|---|---|
| `GET /healthz` | 200 `{"status":"ok"}` |
| `GET /devices?kind=&owner=&q=` | 200, the matching devices sorted by hostname |
| `POST /devices` | 201 with the new device and a `Location` header; 422 invalid; 409 hostname taken; 400 bad JSON |
| `GET /devices/{id}` | 200, or 404 |
| `PUT /devices/{id}` | 200 with the updated device; 404, 409, 422 or 400 |
| `DELETE /devices/{id}` | 204, or 404 |

## How it's built

| File | Job |
|---|---|
| `device.go` | the domain: the `Device` type, `Normalize`, `Validate`, and the error values |
| `store.go` | the `Store` interface and `MemoryStore`: a map behind a `sync.RWMutex`, saved to a JSON file with an atomic rename |
| `api.go` | routes (Go 1.22 patterns), handlers, JSON helpers, error → status mapping, and middleware for logging and panics |
| `api_test.go` | 15 tests with `httptest`, including a fake store and 50 concurrent requests |
| `main.go` | configuration from flags and environment variables, server timeouts, graceful shutdown |

## Your tasks

Four functions `panic("TODO: not done yet")`. The comment above each one is its specification:

1. `Validate` (device.go): collect *every* problem, not just the first. A client that sends three mistakes should hear about all three.
2. `Filter.matches` (store.go): the search filters.
3. `statusFor` (api.go): `errors.As` for the validation error type, `errors.Is` for the others.
4. `update` (api.go): the PUT handler. `create` right above it does almost the same thing.

```text
go test ./...                 all the tests
go test -run TestValidate -v  just one
go test -race ./...           with the race detector (needs a C compiler on Windows; works out of the box on Linux and macOS)
go run .                      the server
```

The first unfinished function panics and stops the test run, so work through them in order: `Validate`, `matches`, `statusFor`, then `update`.
'''),
    ("project", "device_api", "Device Inventory REST API",
     "Finish the four functions that panic with `TODO` until `go test ./...` passes, then `go run .` and try it with curl.",
     BASE + "/starter", FILES),
    ("text", r'''
@@text
## Stretch goals

- **PostgreSQL or SQLite storage**: a `SQLStore` that satisfies the same `Store` interface with `database/sql` (pgx, or modernc.org/sqlite which needs no C compiler). The API code shouldn't change at all.
- **Authentication**: middleware that checks an `Authorization: Bearer` token with `subtle.ConstantTimeCompare`, so reads are public but changes need a key.
- **Pagination**: `?limit=50&after=<hostname>` with keyset pagination, and a `next` link in the response.
- **Metrics**: count requests by route and status, and expose them at `/metrics` (by hand, or with the Prometheus client library).
- **A Dockerfile** with a multi-stage build and a distroless image, plus a `/readyz` that fails during shutdown.
- **An OpenAPI description** (`openapi.yaml`) of the API, so other teams can generate clients for it.
'''),
    ("project", "device_api_finished", "Device Inventory REST API: the finished version",
     "A complete solution to compare with yours.",
     BASE + "/finished", FILES),
    ("text", r'''
@@text
🎉 Routing, validation, error mapping, middleware, concurrency-safe storage, graceful shutdown and tests: this is the skeleton of most Go services running in production.
'''),
]

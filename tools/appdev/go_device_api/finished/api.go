package main

import (
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"log/slog"
	"net/http"
	"strconv"
	"time"
)

// API holds what the handlers need. Nothing is global, so tests can build as many as they like.
type API struct {
	store Store
	log   *slog.Logger
}

// NewAPI returns the whole HTTP application: the routes, wrapped in the middleware.
func NewAPI(store Store, log *slog.Logger) http.Handler {
	a := &API{store: store, log: log}
	mux := http.NewServeMux()
	// Go 1.22+ patterns: a method, a path, and {wildcards} read with r.PathValue.
	mux.HandleFunc("GET /healthz", a.health)
	mux.HandleFunc("GET /devices", a.list)
	mux.HandleFunc("POST /devices", a.create)
	mux.HandleFunc("GET /devices/{id}", a.get)
	mux.HandleFunc("PUT /devices/{id}", a.update)
	mux.HandleFunc("DELETE /devices/{id}", a.delete)
	return a.recoverPanics(a.logRequests(mux))
}

func (a *API) health(w http.ResponseWriter, r *http.Request) {
	writeJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}

// GET /devices?kind=laptop&owner=aiko&q=tk-
func (a *API) list(w http.ResponseWriter, r *http.Request) {
	q := r.URL.Query()
	writeJSON(w, http.StatusOK, a.store.List(Filter{Kind: q.Get("kind"), Owner: q.Get("owner"), Query: q.Get("q")}))
}

func (a *API) get(w http.ResponseWriter, r *http.Request) {
	id, err := pathID(r)
	if err != nil {
		a.fail(w, err)
		return
	}
	d, err := a.store.Get(id)
	if err != nil {
		a.fail(w, err)
		return
	}
	writeJSON(w, http.StatusOK, d)
}

// POST /devices with a JSON device (without id): 201 Created, the device with its new id,
// and a Location header pointing at it.
func (a *API) create(w http.ResponseWriter, r *http.Request) {
	var d Device
	if err := decodeJSON(r, &d); err != nil {
		a.fail(w, err)
		return
	}
	d = Normalize(d)
	if err := Validate(d); err != nil {
		a.fail(w, err)
		return
	}
	created, err := a.store.Create(d)
	if err != nil {
		a.fail(w, err)
		return
	}
	w.Header().Set("Location", fmt.Sprintf("/devices/%d", created.ID))
	writeJSON(w, http.StatusCreated, created)
}

// PUT /devices/{id} with a JSON device: replaces the device, keeping the id from the URL
// (any id in the body is ignored). Same steps as create: decode, normalize, validate, then
// a.store.Update. 200 OK with the updated device. Every error goes through a.fail.
func (a *API) update(w http.ResponseWriter, r *http.Request) {
	id, err := pathID(r)
	if err != nil {
		a.fail(w, err)
		return
	}
	var d Device
	if err := decodeJSON(r, &d); err != nil {
		a.fail(w, err)
		return
	}
	d = Normalize(d)
	d.ID = id
	if err := Validate(d); err != nil {
		a.fail(w, err)
		return
	}
	updated, err := a.store.Update(d)
	if err != nil {
		a.fail(w, err)
		return
	}
	writeJSON(w, http.StatusOK, updated)
}

// DELETE /devices/{id}: 204 No Content.
func (a *API) delete(w http.ResponseWriter, r *http.Request) {
	id, err := pathID(r)
	if err != nil {
		a.fail(w, err)
		return
	}
	if err := a.store.Delete(id); err != nil {
		a.fail(w, err)
		return
	}
	w.WriteHeader(http.StatusNoContent)
}

// errBadRequest marks errors caused by the request itself (bad JSON, a bad id).
var errBadRequest = errors.New("bad request")

func pathID(r *http.Request) (int, error) {
	id, err := strconv.Atoi(r.PathValue("id"))
	if err != nil || id < 1 {
		return 0, fmt.Errorf("%w: the id must be a positive whole number", errBadRequest)
	}
	return id, nil
}

// decodeJSON reads one JSON object from the body: at most 1 MB, and no unknown fields
// (a typo like "hostnme" is an error, not silently ignored).
func decodeJSON(r *http.Request, v any) error {
	dec := json.NewDecoder(io.LimitReader(r.Body, 1<<20))
	dec.DisallowUnknownFields()
	if err := dec.Decode(v); err != nil {
		return fmt.Errorf("%w: invalid JSON: %v", errBadRequest, err)
	}
	return nil
}

// statusFor chooses the HTTP status for an error, looking through wrapping:
// a *ValidationError -> 422 Unprocessable Entity, errBadRequest -> 400, ErrNotFound -> 404,
// ErrDuplicate -> 409 Conflict, anything else -> 500. Use errors.As and errors.Is.
func statusFor(err error) int {
	var verr *ValidationError
	switch {
	case errors.As(err, &verr):
		return http.StatusUnprocessableEntity
	case errors.Is(err, errBadRequest):
		return http.StatusBadRequest
	case errors.Is(err, ErrNotFound):
		return http.StatusNotFound
	case errors.Is(err, ErrDuplicate):
		return http.StatusConflict
	default:
		return http.StatusInternalServerError
	}
}

// fail sends an error as JSON: {"error": "..."}, plus "problems" for validation errors.
// Unexpected errors are logged, and the client only gets a generic message: internal details
// (file paths, SQL) must never leak out.
func (a *API) fail(w http.ResponseWriter, err error) {
	status := statusFor(err)
	body := map[string]any{"error": err.Error()}
	var verr *ValidationError
	if errors.As(err, &verr) {
		body["problems"] = verr.Problems
	}
	if status == http.StatusInternalServerError {
		a.log.Error("internal error", "err", err)
		body = map[string]any{"error": "internal server error"}
	}
	writeJSON(w, status, body)
}

func writeJSON(w http.ResponseWriter, status int, v any) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)
	_ = json.NewEncoder(w).Encode(v) // the client went away; nothing useful to do
}

// statusRecorder remembers the status a handler sent, for the request log.
type statusRecorder struct {
	http.ResponseWriter
	status int
}

func (s *statusRecorder) WriteHeader(code int) {
	s.status = code
	s.ResponseWriter.WriteHeader(code)
}

// logRequests is middleware: it wraps a handler and logs every request after it's handled.
func (a *API) logRequests(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		start := time.Now()
		rec := &statusRecorder{ResponseWriter: w, status: http.StatusOK}
		next.ServeHTTP(rec, r)
		a.log.Info("request", "method", r.Method, "path", r.URL.Path, "status", rec.status,
			"duration", time.Since(start))
	})
}

// recoverPanics turns a panic in a handler into a 500 response instead of a dropped connection.
func (a *API) recoverPanics(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		defer func() {
			if v := recover(); v != nil {
				a.log.Error("panic", "value", v, "path", r.URL.Path)
				writeJSON(w, http.StatusInternalServerError, map[string]string{"error": "internal server error"})
			}
		}()
		next.ServeHTTP(w, r)
	})
}

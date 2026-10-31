@@lesson xp-url_shortener URL Shortener Service
@@text
Build a link shortener: `POST /shorten` with a URL returns a short code, `GET /{code}` redirects, `GET /stats/{code}` shows the click count. Store links in a map protected by a mutex, save them to a JSON file, and test the handlers with `httptest`.

**You'll practise:** net/http and the 1.22 router patterns · encoding/json · sync.Mutex · httptest

## Milestones

1. **Store**: a struct with a map and a mutex, and Add/Get/Hit methods.
2. **Handlers**: shorten, redirect (302) and stats, with JSON errors.
3. **Codes**: 6 random characters from crypto/rand, retry on collision.
4. **Persistence**: load and save links.json.
5. **Tests**: httptest.NewRecorder for each handler, including bad input.

## You're done when

- `go test ./...` passes and curl can shorten and follow a link.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"How do the new path patterns like GET /{code} work in Go 1.22?"*
- *"Is my mutex usage free of data races? How do I check with -race?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project url_shortener URL Shortener Service
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# URL Shortener Service

Build a link shortener: `POST /shorten` with a URL returns a short code, `GET /{code}` redirects, `GET /stats/{code}` shows the click count. Store links in a map protected by a mutex, save them to a JSON file, and test the handlers with `httptest`.

## Milestones

- [ ] 1. Store: a struct with a map and a mutex, and Add/Get/Hit methods.
- [ ] 2. Handlers: shorten, redirect (302) and stats, with JSON errors.
- [ ] 3. Codes: 6 random characters from crypto/rand, retry on collision.
- [ ] 4. Persistence: load and save links.json.
- [ ] 5. Tests: httptest.NewRecorder for each handler, including bad input.

## Done when

- [ ] `go test ./...` passes and curl can shorten and follow a link.
@@file go.mod
module shortener

go 1.22
@@file main.go
package main

import (
	"log"
	"net/http"
	"sync"
)

type Store struct {
	mu    sync.Mutex
	links map[string]string
	hits  map[string]int
}

func main() {
	s := &Store{links: map[string]string{}, hits: map[string]int{}}
	mux := http.NewServeMux()
	mux.HandleFunc("GET /{code}", func(w http.ResponseWriter, r *http.Request) {
		s.mu.Lock()
		url, ok := s.links[r.PathValue("code")]
		s.mu.Unlock()
		if !ok {
			http.NotFound(w, r)
			return
		}
		http.Redirect(w, r, url, http.StatusFound)
	})
	// TODO: POST /shorten and GET /stats/{code}
	log.Fatal(http.ListenAndServe(":8080", mux))
}

package main

import (
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"slices"
	"strings"
	"sync"
)

// Filter selects devices. Empty fields match everything.
type Filter struct {
	Kind  string // exactly this kind
	Owner string // exactly this owner, ignoring case
	Query string // part of the hostname, ignoring case
}

// Store is what the API needs from storage. The API only knows this interface, so the
// tests (or a future PostgreSQL version) can give it something else.
type Store interface {
	List(f Filter) []Device
	Get(id int) (Device, error)
	Create(d Device) (Device, error)
	Update(d Device) (Device, error)
	Delete(id int) error
}

// MemoryStore keeps the devices in a map, guarded by a mutex because every HTTP request
// runs in its own goroutine. With a path, it saves to a JSON file after every change.
type MemoryStore struct {
	mu      sync.RWMutex
	devices map[int]Device
	nextID  int
	path    string
}

func NewMemoryStore() *MemoryStore {
	return &MemoryStore{devices: map[int]Device{}, nextID: 1}
}

// OpenFileStore loads the devices from path, if the file exists, and saves there from now on.
func OpenFileStore(path string) (*MemoryStore, error) {
	s := NewMemoryStore()
	s.path = path
	data, err := os.ReadFile(path)
	if errors.Is(err, os.ErrNotExist) {
		return s, nil
	}
	if err != nil {
		return nil, err
	}
	var list []Device
	if err := json.Unmarshal(data, &list); err != nil {
		return nil, fmt.Errorf("reading %s: %w", path, err)
	}
	for _, d := range list {
		s.devices[d.ID] = d
		if d.ID >= s.nextID {
			s.nextID = d.ID + 1
		}
	}
	return s, nil
}

// matches reports whether d passes every filter that is set: Kind equal, Owner equal ignoring
// case (strings.EqualFold), and Query contained in the hostname ignoring case.
func (f Filter) matches(d Device) bool {
	if f.Kind != "" && d.Kind != f.Kind {
		return false
	}
	if f.Owner != "" && !strings.EqualFold(d.Owner, f.Owner) {
		return false
	}
	if f.Query != "" && !strings.Contains(d.Hostname, strings.ToLower(f.Query)) {
		return false
	}
	return true
}

// List returns the matching devices sorted by hostname (an empty slice, never nil, so the
// JSON is [] and not null).
func (s *MemoryStore) List(f Filter) []Device {
	s.mu.RLock()
	defer s.mu.RUnlock()
	return s.sorted(f)
}

func (s *MemoryStore) sorted(f Filter) []Device {
	out := []Device{}
	for _, d := range s.devices {
		if f.matches(d) {
			out = append(out, d)
		}
	}
	slices.SortFunc(out, func(a, b Device) int { return strings.Compare(a.Hostname, b.Hostname) })
	return out
}

func (s *MemoryStore) Get(id int) (Device, error) {
	s.mu.RLock()
	defer s.mu.RUnlock()
	d, ok := s.devices[id]
	if !ok {
		return Device{}, fmt.Errorf("device %d: %w", id, ErrNotFound)
	}
	return d, nil
}

// hostnameTaken reports whether another device (not id) has this hostname. Caller holds the lock.
func (s *MemoryStore) hostnameTaken(hostname string, id int) bool {
	for _, d := range s.devices {
		if d.Hostname == hostname && d.ID != id {
			return true
		}
	}
	return false
}

func (s *MemoryStore) Create(d Device) (Device, error) {
	s.mu.Lock()
	defer s.mu.Unlock()
	if s.hostnameTaken(d.Hostname, 0) {
		return Device{}, fmt.Errorf("hostname %s: %w", d.Hostname, ErrDuplicate)
	}
	d.ID = s.nextID
	s.devices[d.ID] = d
	if err := s.save(); err != nil {
		delete(s.devices, d.ID)
		return Device{}, err
	}
	s.nextID++
	return d, nil
}

func (s *MemoryStore) Update(d Device) (Device, error) {
	s.mu.Lock()
	defer s.mu.Unlock()
	old, ok := s.devices[d.ID]
	if !ok {
		return Device{}, fmt.Errorf("device %d: %w", d.ID, ErrNotFound)
	}
	if s.hostnameTaken(d.Hostname, d.ID) {
		return Device{}, fmt.Errorf("hostname %s: %w", d.Hostname, ErrDuplicate)
	}
	s.devices[d.ID] = d
	if err := s.save(); err != nil {
		s.devices[d.ID] = old
		return Device{}, err
	}
	return d, nil
}

func (s *MemoryStore) Delete(id int) error {
	s.mu.Lock()
	defer s.mu.Unlock()
	old, ok := s.devices[id]
	if !ok {
		return fmt.Errorf("device %d: %w", id, ErrNotFound)
	}
	delete(s.devices, id)
	if err := s.save(); err != nil {
		s.devices[id] = old
		return err
	}
	return nil
}

// save writes every device to a temporary file and renames it over the real one, so a crash
// can't leave a half-written file. The caller holds the lock.
func (s *MemoryStore) save() error {
	if s.path == "" {
		return nil
	}
	data, err := json.MarshalIndent(s.sorted(Filter{}), "", "  ")
	if err != nil {
		return err
	}
	tmp := s.path + ".tmp"
	if err := os.WriteFile(tmp, data, 0o644); err != nil {
		return err
	}
	return os.Rename(tmp, s.path)
}

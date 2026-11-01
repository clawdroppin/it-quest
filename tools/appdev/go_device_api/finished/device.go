package main

import (
	"errors"
	"fmt"
	"net/netip"
	"regexp"
	"strings"
)

// Device is one piece of network-connected equipment. The struct tags are the JSON field names.
type Device struct {
	ID       int    `json:"id"`
	Hostname string `json:"hostname"`
	IP       string `json:"ip"`
	Kind     string `json:"kind"`
	Owner    string `json:"owner,omitempty"`
}

func (d Device) String() string {
	return fmt.Sprintf("#%d %s (%s, %s)", d.ID, d.Hostname, d.Kind, d.IP)
}

// Errors the store returns. Callers check them with errors.Is, even when they're wrapped.
var (
	ErrNotFound  = errors.New("not found")
	ErrDuplicate = errors.New("already exists")
)

// ValidationError lists every problem with a device, so a client can fix them all at once.
type ValidationError struct {
	Problems []string
}

func (e *ValidationError) Error() string {
	return "invalid device: " + strings.Join(e.Problems, "; ")
}

var hostnamePattern = regexp.MustCompile(`^[a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?$`)

var kinds = map[string]bool{"desktop": true, "laptop": true, "printer": true, "server": true, "switch": true}

// Normalize tidies what a client sent: spaces trimmed everywhere, the hostname and kind
// in lower case, and a valid IP address in its standard form (2001:DB8::1 -> 2001:db8::1).
func Normalize(d Device) Device {
	d.Hostname = strings.ToLower(strings.TrimSpace(d.Hostname))
	d.Kind = strings.ToLower(strings.TrimSpace(d.Kind))
	d.Owner = strings.TrimSpace(d.Owner)
	d.IP = strings.TrimSpace(d.IP)
	if addr, err := netip.ParseAddr(d.IP); err == nil {
		d.IP = addr.String()
	}
	return d
}

// Validate returns nil for a valid (normalized) device, or a *ValidationError listing every
// problem, in this order and with exactly these messages:
//
//	hostname "<h>" must be 1-63 lower-case letters, digits or dashes, and not start or end with a dash
//	ip "<ip>" is not an IP address
//	kind "<k>" must be one of: desktop, laptop, printer, server, switch
//
// Use hostnamePattern, netip.ParseAddr and the kinds map.
func Validate(d Device) error {
	var problems []string
	if !hostnamePattern.MatchString(d.Hostname) {
		problems = append(problems, fmt.Sprintf("hostname %q must be 1-63 lower-case letters, digits or dashes, and not start or end with a dash", d.Hostname))
	}
	if _, err := netip.ParseAddr(d.IP); err != nil {
		problems = append(problems, fmt.Sprintf("ip %q is not an IP address", d.IP))
	}
	if !kinds[d.Kind] {
		problems = append(problems, fmt.Sprintf("kind %q must be one of: desktop, laptop, printer, server, switch", d.Kind))
	}
	if len(problems) > 0 {
		return &ValidationError{Problems: problems}
	}
	return nil
}

@@lesson p7-grpc Protocol Buffers and gRPC
@@concepts networking, json
@@text
Most internal service-to-service traffic at large companies (and Kubernetes, etcd and many cloud APIs) uses **gRPC**: remote procedure calls over HTTP/2 with messages encoded as **Protocol Buffers**. Go has first-class support.

## Defining a service

```text
// inventory.proto
syntax = "proto3";
package inventory.v1;
option go_package = "example.com/inventory/gen/inventoryv1";

message Host {
  string name = 1;
  uint32 port = 2;
  repeated string tags = 3;
  reserved 4;            // a removed field: never reuse its number
}

message GetHostRequest { string name = 1; }

service Inventory {
  rpc GetHost(GetHostRequest) returns (Host);
  rpc WatchHosts(WatchRequest) returns (stream Host); // server streaming
}
```

Code generation produces Go structs and client/server interfaces:

```text
protoc --go_out=. --go-grpc_out=. inventory.proto
# or, with the popular buf tool:
buf generate
```

## Server and client

```go,norun
type server struct {
	inventoryv1.UnimplementedInventoryServer // forward compatibility
	store *Store
}

func (s *server) GetHost(ctx context.Context, req *inventoryv1.GetHostRequest) (*inventoryv1.Host, error) {
	h, ok := s.store.Get(req.GetName())
	if !ok {
		return nil, status.Errorf(codes.NotFound, "host %q not found", req.GetName())
	}
	return h, nil
}

func main() {
	lis, _ := net.Listen("tcp", ":50051")
	s := grpc.NewServer(grpc.UnaryInterceptor(loggingInterceptor))
	inventoryv1.RegisterInventoryServer(s, &server{store: NewStore()})
	s.Serve(lis)
}

// client
conn, err := grpc.NewClient("inventory:50051", grpc.WithTransportCredentials(creds))
client := inventoryv1.NewInventoryClient(conn)
ctx, cancel := context.WithTimeout(ctx, 2*time.Second) // deadlines propagate to the server
defer cancel()
host, err := client.GetHost(ctx, &inventoryv1.GetHostRequest{Name: "web01"})
if status.Code(err) == codes.NotFound { ... }
```

- **Status codes** (`NotFound`, `InvalidArgument`, `Unavailable`, `DeadlineExceeded`, `PermissionDenied`...) replace HTTP status codes.
- **Interceptors** are gRPC middleware: logging, auth, metrics, retries.
- Four call types: unary, server streaming, client streaming, bidirectional streaming.
- `grpcurl` is curl for gRPC; enable server reflection to use it.
- Connect (`connectrpc.com`) and grpc-gateway serve the same API over plain HTTP/JSON for browsers.

## The wire format

Protobuf is compact because it sends **field numbers, not names**. Each field is a key followed by a value:

- key = `field_number << 3 | wire_type`
- wire type 0 = **varint** (ints, bools, enums), 2 = **length-delimited** (strings, bytes, nested messages, packed repeated fields), plus 1 and 5 for fixed 64/32-bit values
- a **varint** stores 7 bits per byte, low bits first, with the high bit set on every byte except the last: 300 = `ac 02`
- signed `sint32`/`sint64` use **zigzag** encoding (0, −1, 1, −2... → 0, 1, 2, 3...) so small negative numbers stay small

Unknown field numbers are skipped by decoders. That's why you can add fields without breaking old clients, and why you must **never reuse or change** a field number.

Go's `encoding/binary` already has `AppendUvarint` and `Uvarint`, which use the same varint encoding, so the exercises can check your encoder against them.

@@quiz
Why must you never reuse a protobuf field number?
* Old clients would decode the new data as the old field, silently misreading it
- protoc forbids numbers above 100
- Field numbers are sorted alphabetically
- It changes the JSON names
? Mark removed numbers as `reserved`.

@@quiz
What's the protobuf key byte for field 2 with wire type 0 (varint)?
- 0x02
* 0x10
- 0x20
- 0x12
? 2 << 3 | 0 = 16 = 0x10.

@@quiz
Which gRPC status should a lookup for a missing record return?
- Unknown
- Internal
* NotFound
- Unavailable
? Clients can branch on it with status.Code(err).

@@flashcards
gRPC :: RPC over HTTP/2 with protobuf
`.proto` :: schema for messages and services
Field number :: identifies a field on the wire
`reserved` :: block reuse of removed fields
Varint :: 7 bits per byte, MSB = continuation
Zigzag :: maps signed to unsigned for small varints
Wire type 2 :: length-delimited
`status.Errorf(codes.X, ...)` :: gRPC error with a code
Interceptor :: gRPC middleware
`grpcurl` :: CLI client for gRPC

@@exercise Varints and zigzag
Implement the protobuf integer encodings by hand:

- `EncodeVarint(x uint64) []byte`
- `DecodeVarint(b []byte) (uint64, int, error)` returning the value and bytes consumed; errors `truncated varint` (input ends while the continuation bit is set) and `varint overflow` (more than 10 bytes)
- `ZigZag(n int64) uint64` and `UnZigZag(u uint64) int64`

`main` compares your encoding with `binary.AppendUvarint` for several values, then prints zigzag mappings and the two error cases.
@@starter
package main

import (
	"bytes"
	"encoding/binary"
	"errors"
	"fmt"
	"math"
)

func EncodeVarint(x uint64) []byte {
	return nil
}

func DecodeVarint(b []byte) (uint64, int, error) {
	_ = errors.New
	return 0, 0, nil
}

func ZigZag(n int64) uint64     { return 0 }
func UnZigZag(u uint64) int64   { return 0 }

func main() {
	for _, n := range []uint64{0, 1, 127, 128, 300, 16384, math.MaxUint64} {
		enc := EncodeVarint(n)
		dec, used, err := DecodeVarint(enc)
		fmt.Printf("%d -> % x  stdlib=%v roundtrip=%v\n", n, enc,
			bytes.Equal(enc, binary.AppendUvarint(nil, n)), dec == n && used == len(enc) && err == nil)
	}
	for _, n := range []int64{0, -1, 1, -2, 2, -64, 64} {
		z := ZigZag(n)
		fmt.Printf("zigzag %d -> %d -> %d\n", n, z, UnZigZag(z))
	}
	_, _, err := DecodeVarint([]byte{0x80})
	fmt.Println(err)
	_, _, err = DecodeVarint(bytes.Repeat([]byte{0xff}, 11))
	fmt.Println(err)
}
@@expected
0 -> 00  stdlib=true roundtrip=true
1 -> 01  stdlib=true roundtrip=true
127 -> 7f  stdlib=true roundtrip=true
128 -> 80 01  stdlib=true roundtrip=true
300 -> ac 02  stdlib=true roundtrip=true
16384 -> 80 80 01  stdlib=true roundtrip=true
18446744073709551615 -> ff ff ff ff ff ff ff ff ff 01  stdlib=true roundtrip=true
zigzag 0 -> 0 -> 0
zigzag -1 -> 1 -> -1
zigzag 1 -> 2 -> 1
zigzag -2 -> 3 -> -2
zigzag 2 -> 4 -> 2
zigzag -64 -> 127 -> -64
zigzag 64 -> 128 -> 64
truncated varint
varint overflow
@@requires
0x80
@@hint
Encode: `for x >= 0x80 { out = append(out, byte(x)|0x80); x >>= 7 }`, then append `byte(x)`. Decode: `x |= uint64(b[i]&0x7f) << (7*i)`, stop when `b[i] < 0x80`. ZigZag: `uint64((n << 1) ^ (n >> 63))`; back: `int64(u>>1) ^ -int64(u&1)`.
@@solution
package main

import (
	"bytes"
	"encoding/binary"
	"errors"
	"fmt"
	"math"
)

func EncodeVarint(x uint64) []byte {
	var out []byte
	for x >= 0x80 {
		out = append(out, byte(x)|0x80)
		x >>= 7
	}
	return append(out, byte(x))
}

func DecodeVarint(b []byte) (uint64, int, error) {
	var x uint64
	for i := 0; i < len(b); i++ {
		if i >= 10 {
			return 0, 0, errors.New("varint overflow")
		}
		x |= uint64(b[i]&0x7f) << (7 * i)
		if b[i] < 0x80 {
			return x, i + 1, nil
		}
	}
	if len(b) > 10 {
		return 0, 0, errors.New("varint overflow")
	}
	return 0, 0, errors.New("truncated varint")
}

func ZigZag(n int64) uint64   { return uint64((n << 1) ^ (n >> 63)) }
func UnZigZag(u uint64) int64 { return int64(u>>1) ^ -int64(u&1) }

func main() {
	for _, n := range []uint64{0, 1, 127, 128, 300, 16384, math.MaxUint64} {
		enc := EncodeVarint(n)
		dec, used, err := DecodeVarint(enc)
		fmt.Printf("%d -> % x  stdlib=%v roundtrip=%v\n", n, enc,
			bytes.Equal(enc, binary.AppendUvarint(nil, n)), dec == n && used == len(enc) && err == nil)
	}
	for _, n := range []int64{0, -1, 1, -2, 2, -64, 64} {
		z := ZigZag(n)
		fmt.Printf("zigzag %d -> %d -> %d\n", n, z, UnZigZag(z))
	}
	_, _, err := DecodeVarint([]byte{0x80})
	fmt.Println(err)
	_, _, err = DecodeVarint(bytes.Repeat([]byte{0xff}, 11))
	fmt.Println(err)
}

@@exercise Encode and decode a protobuf message
Hand-encode this message (as the generated code would):

```text
message Host { string name = 1; uint32 port = 2; repeated string tags = 3; }
```

- `Encode(h Host) []byte`: fields in number order; skip empty name and zero port (proto3 doesn't send default values); each tag is its own field-3 entry
- `Decode(b []byte) (Host, error)`: read key varints, dispatch on field number, and **skip unknown fields** of wire type 0 (read a varint) and 2 (read a length and skip it); other wire types → `unsupported wire type <n>`

`binary.AppendUvarint` and `binary.Uvarint` may be used for varints. `main` also compares the size with JSON.
@@starter
package main

import (
	"encoding/binary"
	"encoding/json"
	"fmt"
)

type Host struct {
	Name string   `json:"name"`
	Port uint32   `json:"port"`
	Tags []string `json:"tags"`
}

func Encode(h Host) []byte {
	_ = binary.AppendUvarint
	return nil
}

func Decode(b []byte) (Host, error) {
	return Host{}, nil
}

func main() {
	h := Host{Name: "web01", Port: 443, Tags: []string{"prod", "eu"}}
	b := Encode(h)
	fmt.Printf("% x\n", b)
	back, err := Decode(b)
	fmt.Printf("%+v %v\n", back, err)
	newer := append(append([]byte{}, b...), 0x48, 0x07, 0x52, 0x01, 'x') // fields 9 and 10 from a newer schema
	back, err = Decode(newer)
	fmt.Printf("%+v %v\n", back, err)
	_, err = Decode([]byte{0x0d, 1, 2, 3, 4})
	fmt.Println(err)
	j, _ := json.Marshal(h)
	fmt.Printf("protobuf %d bytes, json %d bytes\n", len(b), len(j))
}
@@expected
0a 05 77 65 62 30 31 10 bb 03 1a 04 70 72 6f 64 1a 02 65 75
{Name:web01 Port:443 Tags:[prod eu]} <nil>
{Name:web01 Port:443 Tags:[prod eu]} <nil>
unsupported wire type 5
protobuf 20 bytes, json 48 bytes
@@requires
binary.Uvarint
@@hint
Helpers: `appendString(b []byte, field int, s string)` appends `uint64(field<<3|2)`, the length and the bytes. In `Decode`, loop while `len(b) > 0`: `key, n := binary.Uvarint(b)`, `field, wt := key>>3, key&7`, `b = b[n:]`. For wire type 2, read the length and slice `b[:l]`; check bounds and return `truncated message` if they don't fit.
@@solution
package main

import (
	"encoding/binary"
	"encoding/json"
	"errors"
	"fmt"
)

type Host struct {
	Name string   `json:"name"`
	Port uint32   `json:"port"`
	Tags []string `json:"tags"`
}

func appendString(b []byte, field int, s string) []byte {
	b = binary.AppendUvarint(b, uint64(field<<3|2))
	b = binary.AppendUvarint(b, uint64(len(s)))
	return append(b, s...)
}

func Encode(h Host) []byte {
	var b []byte
	if h.Name != "" {
		b = appendString(b, 1, h.Name)
	}
	if h.Port != 0 {
		b = binary.AppendUvarint(b, uint64(2<<3|0))
		b = binary.AppendUvarint(b, uint64(h.Port))
	}
	for _, t := range h.Tags {
		b = appendString(b, 3, t)
	}
	return b
}

var errTrunc = errors.New("truncated message")

func Decode(b []byte) (Host, error) {
	var h Host
	for len(b) > 0 {
		key, n := binary.Uvarint(b)
		if n <= 0 {
			return h, errTrunc
		}
		b = b[n:]
		field, wt := key>>3, key&7
		switch wt {
		case 0:
			v, n := binary.Uvarint(b)
			if n <= 0 {
				return h, errTrunc
			}
			b = b[n:]
			if field == 2 {
				h.Port = uint32(v)
			}
		case 2:
			l, n := binary.Uvarint(b)
			if n <= 0 || uint64(len(b)-n) < l {
				return h, errTrunc
			}
			data := string(b[n : n+int(l)])
			b = b[n+int(l):]
			switch field {
			case 1:
				h.Name = data
			case 3:
				h.Tags = append(h.Tags, data)
			}
		default:
			return h, fmt.Errorf("unsupported wire type %d", wt)
		}
	}
	return h, nil
}

func main() {
	h := Host{Name: "web01", Port: 443, Tags: []string{"prod", "eu"}}
	b := Encode(h)
	fmt.Printf("% x\n", b)
	back, err := Decode(b)
	fmt.Printf("%+v %v\n", back, err)
	newer := append(append([]byte{}, b...), 0x48, 0x07, 0x52, 0x01, 'x') // fields 9 and 10 from a newer schema
	back, err = Decode(newer)
	fmt.Printf("%+v %v\n", back, err)
	_, err = Decode([]byte{0x0d, 1, 2, 3, 4})
	fmt.Println(err)
	j, _ := json.Marshal(h)
	fmt.Printf("protobuf %d bytes, json %d bytes\n", len(b), len(j))
}

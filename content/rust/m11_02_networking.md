@@lesson m11-networking Networking with std::net: TCP Servers, Clients and Protocols
@@text
Every web server, database driver and chat app sits on top of **sockets**. Rust's standard library gives you TCP and UDP directly in `std::net`. Building a small server by hand teaches what frameworks hide: connections, byte streams, framing and concurrency.

## TCP in one picture

- A **server** binds a `TcpListener` to an address and port (`127.0.0.1:7878` for local only, `0.0.0.0:7878` for every network interface) and **accepts** connections.
- A **client** connects with `TcpStream::connect("host:port")`.
- Each connection is a **bidirectional byte stream**: whatever one side writes, the other reads, in order, reliably. TCP has **no message boundaries**: two writes of 5 bytes may arrive as one read of 10, or 10 reads of 1. Your protocol must define where messages start and end (**framing**).

## A line-based echo server

```rust,norun
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle(stream: TcpStream) -> std::io::Result<()> {
    let peer = stream.peer_addr()?;
    let mut writer = stream.try_clone()?;          // one handle to read, one to write
    for line in BufReader::new(stream).lines() {
        let line = line?;
        writeln!(writer, "echo: {line}")?;
    }
    println!("{peer} disconnected");
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    for stream in listener.incoming() {
        let stream = stream?;
        thread::spawn(move || {
            if let Err(e) = handle(stream) {
                eprintln!("connection error: {e}");
            }
        });
    }
    Ok(())
}
```

Try it with `telnet 127.0.0.1 7878` or `ncat`. A thread per connection is fine for hundreds of clients; for many thousands, use async (Tokio) or the thread pool from Module 10.

## Framing strategies

| Strategy | How | Used by |
|---|---|---|
| **Delimiter** | messages end with `\n` (or `\r\n`) | SMTP, Redis inline commands, IRC, many simple protocols |
| **Length prefix** | send the length (e.g. 4 bytes, big-endian) then the bytes | binary protocols, databases, gRPC |
| **Header + body** | text headers end with an empty line; `Content-Length` says the body size | HTTP/1.1 |

Length prefixes with `u32::to_be_bytes` / `from_be_bytes` and `read_exact` are robust for binary data (they can carry newlines).

## Timeouts and robustness

A client that connects and never sends anything would block your thread forever. Always set timeouts in servers: `stream.set_read_timeout(Some(Duration::from_secs(30)))`. Also:

- Limit line and message sizes (a malicious client can send a 10 GB "line").
- Treat every byte from the network as untrusted input: validate, never `unwrap()` parsing.
- Handle disconnects (`read` returning 0, or `ConnectionReset`) as normal events.
- Clients: `TcpStream::connect_timeout` and retries with backoff.

## UDP, DNS and addresses

- `UdpSocket` sends independent **datagrams**: no connection, no ordering or delivery guarantee, but low latency (DNS, games, VoIP, metrics like StatsD).
- `"example.com:80".to_socket_addrs()` resolves names with the system resolver.
- `IpAddr`, `Ipv4Addr`, `Ipv6Addr`, `SocketAddr` parse and compare addresses (`"10.0.0.5".parse::<Ipv4Addr>()`, `.is_private()`, `.is_loopback()`).

## Higher-level crates

| Need | Crate |
|---|---|
| Async TCP/UDP | `tokio::net` |
| HTTP client | `reqwest` (async or blocking), `ureq` (small, blocking) |
| HTTP server | `axum` (next lesson), `actix-web`, `hyper` underneath |
| TLS | `rustls` (pure Rust), `native-tls` |
| WebSockets | `tokio-tungstenite`, axum's `ws` |
| Framing helpers | `tokio-util::codec` (`LinesCodec`, `LengthDelimitedCodec`) |
| Packet parsing / raw sockets | `etherparse`, `pnet`, `socket2` |

## Testing network code

Separate the **protocol logic** (a pure function from a request line to a response) from the **I/O loop**. The logic gets ordinary unit tests; the loop gets a few integration tests that bind to port **0** (the OS picks a free port: `listener.local_addr()?` tells you which) and talk to it from a thread, exactly as the second exercise does.

@@quiz
Why does a TCP protocol need framing?
* TCP is a byte stream with no message boundaries; writes can be split or merged on the way
- TCP loses messages
- To encrypt the data
- Framing makes it faster
? Delimiters, length prefixes or headers mark where messages end.

@@quiz
What does binding to `127.0.0.1` instead of `0.0.0.0` do?
- Makes the server faster
* Accepts connections only from the same machine
- Uses IPv6
- Picks a random port
? 0.0.0.0 listens on every interface, including the network.

@@quiz
Why bind to port 0 in tests?
- Port 0 is reserved for tests
* The OS assigns a free port, so tests don't clash with other programs or each other
- It disables the firewall
- It's faster
? `local_addr()` reveals the chosen port.

@@quiz
When would you choose UDP over TCP?
- When data must arrive reliably and in order
* For low-latency messages where an occasional loss is acceptable (DNS, games, metrics)
- For file transfers
- Never
? UDP has no connection, ordering or retransmission.

@@exercise A tiny key-value protocol
Write the protocol logic for a line-based server. `handle_line(store: &mut HashMap<String, String>, line: &str) -> String` understands:

- `SET key value...` → stores (the value is everything after the key, may contain spaces) and returns `OK`
- `GET key` → the value, or `NIL`
- `DEL key` → `1` if removed, `0` if not there
- `COUNT` → the number of keys
- commands are case-insensitive (`get` = `GET`); anything else, or missing arguments, returns `ERR <reason>` (any reason text)

Leading and trailing whitespace on the line is ignored.
@@starter
use std::collections::HashMap;

fn handle_line(store: &mut HashMap<String, String>, line: &str) -> String {
    todo!()
}

fn main() {
    let mut store = HashMap::new();
    for line in ["SET name Ferris the crab", "get name", "COUNT", "DEL name", "GET name", "FLY away", "SET onlykey"] {
        println!("{line} -> {}", handle_line(&mut store, line));
    }
}
@@expected
SET name Ferris the crab -> OK
get name -> Ferris the crab
COUNT -> 1
DEL name -> 1
GET name -> NIL
FLY away -> ERR unknown command FLY
SET onlykey -> ERR SET needs a key and a value
@@tests
#[test]
fn checks() {
    let mut s = HashMap::new();
    assert_eq!(handle_line(&mut s, "  set a 1  "), "OK");
    assert_eq!(handle_line(&mut s, "GET a"), "1");
    assert_eq!(handle_line(&mut s, "DEL zzz"), "0");
    assert!(handle_line(&mut s, "").starts_with("ERR"));
    assert!(handle_line(&mut s, "GET").starts_with("ERR"));
    assert_eq!(handle_line(&mut s, "count"), "1");
}
@@requires
splitn(3
to_uppercase
@@hint
`let mut parts = line.trim().splitn(3, ' ');` gives the command, the key and the rest. `match cmd.to_uppercase().as_str() { "SET" => ..., "GET" => ..., ... }`. For errors use `format!("ERR unknown command {cmd}")`.
@@solution
use std::collections::HashMap;

fn handle_line(store: &mut HashMap<String, String>, line: &str) -> String {
    let mut parts = line.trim().splitn(3, ' ');
    let cmd = parts.next().unwrap_or("");
    let key = parts.next();
    let rest = parts.next();
    match (cmd.to_uppercase().as_str(), key, rest) {
        ("SET", Some(k), Some(v)) => {
            store.insert(k.to_string(), v.to_string());
            "OK".to_string()
        }
        ("SET", _, _) => "ERR SET needs a key and a value".to_string(),
        ("GET", Some(k), None) => store.get(k).cloned().unwrap_or_else(|| "NIL".to_string()),
        ("DEL", Some(k), None) => if store.remove(k).is_some() { "1" } else { "0" }.to_string(),
        ("COUNT", None, None) => store.len().to_string(),
        ("GET" | "DEL", _, _) => format!("ERR {cmd} needs exactly one key"),
        ("", _, _) => "ERR empty command".to_string(),
        _ => format!("ERR unknown command {cmd}"),
    }
}

fn main() {
    let mut store = HashMap::new();
    for line in ["SET name Ferris the crab", "get name", "COUNT", "DEL name", "GET name", "FLY away", "SET onlykey"] {
        println!("{line} -> {}", handle_line(&mut store, line));
    }
}

@@exercise Serve it over TCP
Now put the protocol on the network. Write `serve_one(listener: TcpListener) -> std::io::Result<usize>`: accept **one** connection, read lines with a `BufReader`, answer each with `handle_line` followed by `\n`, and stop when the client sends `QUIT` (reply `BYE`) or disconnects. Return the number of commands answered (including `QUIT`).

`main` starts the server on a free port in a thread and plays a client against it.
@@starter
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_line(store: &mut HashMap<String, String>, line: &str) -> String {
    let mut parts = line.trim().splitn(3, ' ');
    let cmd = parts.next().unwrap_or("").to_uppercase();
    match (cmd.as_str(), parts.next(), parts.next()) {
        ("SET", Some(k), Some(v)) => {
            store.insert(k.into(), v.into());
            "OK".into()
        }
        ("GET", Some(k), None) => store.get(k).cloned().unwrap_or_else(|| "NIL".into()),
        _ => format!("ERR bad command"),
    }
}

fn serve_one(listener: TcpListener) -> std::io::Result<usize> {
    todo!()
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let addr = listener.local_addr()?;
    let server = thread::spawn(move || serve_one(listener));

    let stream = TcpStream::connect(addr)?;
    let mut writer = stream.try_clone()?;
    let mut reader = BufReader::new(stream);
    for cmd in ["SET lang Rust", "GET lang", "HELLO", "QUIT"] {
        writeln!(writer, "{cmd}")?;
        let mut reply = String::new();
        reader.read_line(&mut reply)?;
        println!("{cmd} => {}", reply.trim_end());
    }
    println!("served {}", server.join().unwrap()?);
    Ok(())
}
@@expected
SET lang Rust => OK
GET lang => Rust
HELLO => ERR bad command
QUIT => BYE
served 4
@@tests
#[test]
fn checks() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    let server = thread::spawn(move || serve_one(listener));
    {
        let mut s = TcpStream::connect(addr).unwrap();
        writeln!(s, "SET a 1").unwrap();
        let mut r = BufReader::new(s.try_clone().unwrap());
        let mut line = String::new();
        r.read_line(&mut line).unwrap();
        assert_eq!(line, "OK\n");
    } // client disconnects without QUIT
    assert_eq!(server.join().unwrap().unwrap(), 1);
}
@@requires
listener.accept()
BufReader::new
@@hint
`let (stream, _addr) = listener.accept()?; let mut writer = stream.try_clone()?; let mut store = HashMap::new(); let mut n = 0; for line in BufReader::new(stream).lines() { let line = line?; n += 1; if line.trim().eq_ignore_ascii_case("QUIT") { writeln!(writer, "BYE")?; break; } writeln!(writer, "{}", handle_line(&mut store, &line))?; } Ok(n)`.
@@solution
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_line(store: &mut HashMap<String, String>, line: &str) -> String {
    let mut parts = line.trim().splitn(3, ' ');
    let cmd = parts.next().unwrap_or("").to_uppercase();
    match (cmd.as_str(), parts.next(), parts.next()) {
        ("SET", Some(k), Some(v)) => {
            store.insert(k.into(), v.into());
            "OK".into()
        }
        ("GET", Some(k), None) => store.get(k).cloned().unwrap_or_else(|| "NIL".into()),
        _ => format!("ERR bad command"),
    }
}

fn serve_one(listener: TcpListener) -> std::io::Result<usize> {
    let (stream, _addr) = listener.accept()?;
    let mut writer = stream.try_clone()?;
    let mut store = HashMap::new();
    let mut answered = 0;
    for line in BufReader::new(stream).lines() {
        let line = line?;
        answered += 1;
        if line.trim().eq_ignore_ascii_case("QUIT") {
            writeln!(writer, "BYE")?;
            break;
        }
        writeln!(writer, "{}", handle_line(&mut store, &line))?;
    }
    Ok(answered)
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let addr = listener.local_addr()?;
    let server = thread::spawn(move || serve_one(listener));

    let stream = TcpStream::connect(addr)?;
    let mut writer = stream.try_clone()?;
    let mut reader = BufReader::new(stream);
    for cmd in ["SET lang Rust", "GET lang", "HELLO", "QUIT"] {
        writeln!(writer, "{cmd}")?;
        let mut reply = String::new();
        reader.read_line(&mut reply)?;
        println!("{cmd} => {}", reply.trim_end());
    }
    println!("served {}", server.join().unwrap()?);
    Ok(())
}

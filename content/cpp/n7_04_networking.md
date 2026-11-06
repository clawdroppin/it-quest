@@lesson n7-networking Network Programming: Sockets, Asio and Protocol Parsing
@@concepts networking, text-processing
@@text
Network services in C++ (proxies, game servers, trading systems, databases) are built on sockets, usually through **Asio** (standalone or Boost.Asio), which wraps the OS's async I/O: epoll on Linux, kqueue on macOS, IOCP on Windows.

```cpp,norun
#include <asio.hpp>
using asio::ip::tcp;

asio::awaitable<void> session(tcp::socket socket) {
    char data[4096];
    for (;;) {
        size_t n = co_await socket.async_read_some(asio::buffer(data), asio::use_awaitable);
        co_await asio::async_write(socket, asio::buffer(data, n), asio::use_awaitable);   // echo
    }
}

asio::awaitable<void> listener() {
    auto executor = co_await asio::this_coro::executor;
    tcp::acceptor acceptor(executor, {tcp::v4(), 5555});
    for (;;) asio::co_spawn(executor, session(co_await acceptor.async_accept(asio::use_awaitable)), asio::detached);
}

int main() {
    asio::io_context io;
    asio::co_spawn(io, listener(), asio::detached);
    io.run();                     // one thread serves thousands of connections
}
```

## TCP is a byte stream

The most important fact in network programming: **TCP doesn't preserve message boundaries**. One `send` of 100 bytes can arrive as three reads of 30, 50 and 20 bytes, and two sends can arrive in one read. Every protocol therefore needs **framing**:

- **length prefix**: a fixed-size header with the payload length (binary protocols, gRPC, Kafka)
- **delimiters**: lines ending in `\r\n` (HTTP/1.1 headers, SMTP, Redis), plus a length for bodies (`Content-Length`)

The parser keeps a buffer, appends whatever arrives, and extracts complete messages; anything incomplete waits for the next read. It must also **limit** sizes, or a malicious peer sends a 4 GB length and your server allocates it.

## Byte order

Multi-byte integers on the wire are **big-endian** ("network order") by convention, while x86 and ARM are little-endian in memory. Build integers from bytes explicitly (`b[0] << 24 | b[1] << 16 | ...`), which works on any CPU, or use `std::byteswap` (C++23) with `std::endian`.

## Other building blocks

- TLS: OpenSSL, or `asio::ssl` on top of it
- HTTP: Boost.Beast, cpp-httplib, Drogon; gRPC for RPC
- serialisation: Protocol Buffers, FlatBuffers, Cap'n Proto, nlohmann/json, simdjson
- timeouts on every operation, and graceful shutdown (stop accepting, finish in-flight requests)

@@quiz
A client sends two 50-byte messages over TCP. What can the server's reads return?
- Exactly two reads of 50 bytes
* Any split: one read of 100, or 30 + 70, or 50 + 20 + 30, ...
- Only one read of 100
- It depends on the message content
? TCP is a byte stream; framing is your job.

@@quiz
Why must a length-prefixed protocol enforce a maximum frame size?
* Otherwise a peer can claim a huge length and make the server allocate unbounded memory
- To make parsing faster
- Because TCP limits frames
- It doesn't need to
? Check the length before allocating.

@@quiz
Why convert integers with htonl/ntohl (or std::byteswap) before sending them?
* Network protocols use big-endian byte order, while most CPUs are little-endian
- To compress them
- To encrypt them
- Sockets only accept strings
? Both sides must agree on the byte order of multi-byte numbers.

@@flashcards
Asio :: async I/O library (epoll/kqueue/IOCP)
`io_context::run()` :: event loop
`co_await async_read_some` :: coroutine-based I/O
Framing :: finding message boundaries in a stream
Length prefix :: header with payload size
Big-endian :: network byte order
`std::endian` :: detect native byte order
Boost.Beast :: HTTP and WebSocket on Asio
Maximum frame size :: defence against memory exhaustion

@@exercise A length-prefixed frame reader
Implement framing for a binary protocol: each frame is a 4-byte big-endian length followed by that many payload bytes.

- `std::string encode(std::string_view payload)` writes the header byte by byte (no casting of integers to bytes)
- `FrameReader::feed(std::string_view chunk)` appends to an internal buffer and returns every **complete** frame; partial data stays buffered
- frames larger than `max_frame` (1 MiB) throw `std::length_error("frame of N bytes exceeds limit 1048576")`

Encode three messages, concatenate them, and feed the stream to the reader **3 bytes at a time**, printing each frame with the chunk number (0-based) where it completed.
@@starter
#include <cstdint>
#include <iostream>
#include <stdexcept>
#include <string>
#include <string_view>
#include <vector>

int main() {
    std::cout << "TODO\n";
}
@@expected
chunk 2: frame "hello" (5 bytes)
chunk 4: frame "" (0 bytes)
chunk 8: frame "ping web01" (10 bytes)
buffered after all chunks: 0
oversized: frame of 16777216 bytes exceeds limit 1048576
@@requires
std::length_error
<< 24
@@hint
Read the header with `uint32_t(uint8_t(buf[0])) << 24 | uint32_t(uint8_t(buf[1])) << 16 | ...`. Loop in `feed`: while the buffer holds at least 4 bytes, read the length, check the limit, stop if the payload isn't complete yet, else extract it and `erase` the consumed bytes. For the oversized test, feed `std::string("\x01\x00\x00\x00", 4)`.
@@solution
#include <cstdint>
#include <iostream>
#include <stdexcept>
#include <string>
#include <string_view>
#include <vector>

std::string encode(std::string_view payload) {
    uint32_t n = static_cast<uint32_t>(payload.size());
    std::string out;
    out += static_cast<char>(n >> 24 & 0xFF);
    out += static_cast<char>(n >> 16 & 0xFF);
    out += static_cast<char>(n >> 8 & 0xFF);
    out += static_cast<char>(n & 0xFF);
    out += payload;
    return out;
}

class FrameReader {
public:
    explicit FrameReader(uint32_t max_frame) : max_frame_(max_frame) {}

    std::vector<std::string> feed(std::string_view chunk) {
        buffer_ += chunk;
        std::vector<std::string> frames;
        while (buffer_.size() >= 4) {
            auto byte = [&](size_t i) { return uint32_t(uint8_t(buffer_[i])); };
            uint32_t n = byte(0) << 24 | byte(1) << 16 | byte(2) << 8 | byte(3);
            if (n > max_frame_)
                throw std::length_error("frame of " + std::to_string(n) + " bytes exceeds limit " + std::to_string(max_frame_));
            if (buffer_.size() < 4 + size_t(n)) break;
            frames.push_back(buffer_.substr(4, n));
            buffer_.erase(0, 4 + size_t(n));
        }
        return frames;
    }

    size_t buffered() const { return buffer_.size(); }

private:
    uint32_t max_frame_;
    std::string buffer_;
};

int main() {
    std::string stream = encode("hello") + encode("") + encode("ping web01");
    FrameReader reader(1 << 20);
    for (size_t i = 0, chunk = 0; i < stream.size(); i += 3, ++chunk) {
        for (const auto& f : reader.feed(std::string_view(stream).substr(i, 3)))
            std::cout << "chunk " << chunk << ": frame \"" << f << "\" (" << f.size() << " bytes)\n";
    }
    std::cout << "buffered after all chunks: " << reader.buffered() << '\n';

    FrameReader strict(1 << 20);
    try {
        strict.feed(std::string("\x01\x00\x00\x00", 4));
    } catch (const std::length_error& e) {
        std::cout << "oversized: " << e.what() << '\n';
    }
}

@@exercise An incremental HTTP request parser
Parse HTTP/1.1 requests that arrive in arbitrary pieces:

- states: request line → headers → body → done; `feed(chunk)` processes as much as it can and returns true when the request is complete
- request line: exactly three space-separated parts and a version starting with `HTTP/1.`, else `bad request line`
- headers: `Name: value` lines (lower-case the name, trim the value), else `bad header line`; an empty line ends the headers
- body: exactly `content-length` bytes (0 if absent)
- errors throw `std::runtime_error`

Feed the sample request in 7-byte chunks and print it, then show the two errors.
@@starter
#include <algorithm>
#include <cctype>
#include <iostream>
#include <stdexcept>
#include <string>
#include <string_view>
#include <utility>
#include <vector>

int main() {
    std::string request =
        "POST /api/hosts HTTP/1.1\r\nHost: inventory\r\nContent-Length: 12\r\n"
        "Content-Type: application/json\r\n\r\n{\"name\":\"a\"}";
    std::cout << "TODO\n";
}
@@expected
POST /api/hosts
host: inventory
content-length: 12
content-type: application/json
body: {"name":"a"}
chunks fed: 16
GET / -> error: bad request line
header without colon -> error: bad header line
@@requires
\r\n
std::runtime_error
@@hint
Keep a `std::string buffer_` and an `enum class State`. In a loop: in the line-based states, `auto eol = buffer_.find("\r\n"); if (eol == npos) return false;` then take the line and erase it plus the 2 bytes. In the body state, wait until `buffer_.size() >= content_length`. Look up `content-length` among the stored headers and convert it with `std::stoul`.
@@solution
#include <algorithm>
#include <cctype>
#include <iostream>
#include <stdexcept>
#include <string>
#include <string_view>
#include <utility>
#include <vector>

class RequestParser {
public:
    std::string method, path, body;
    std::vector<std::pair<std::string, std::string>> headers;

    bool feed(std::string_view chunk) {
        buffer_ += chunk;
        while (true) {
            if (state_ == State::Done) return true;
            if (state_ == State::Body) {
                if (buffer_.size() < content_length_) return false;
                body = buffer_.substr(0, content_length_);
                buffer_.erase(0, content_length_);
                state_ = State::Done;
                continue;
            }
            auto eol = buffer_.find("\r\n");
            if (eol == std::string::npos) return false;
            std::string line = buffer_.substr(0, eol);
            buffer_.erase(0, eol + 2);
            if (state_ == State::RequestLine) request_line(line);
            else header_line(line);
        }
    }

    std::string header(std::string_view name) const {
        for (const auto& [k, v] : headers)
            if (k == name) return v;
        return "";
    }

private:
    enum class State { RequestLine, Headers, Body, Done };

    void request_line(const std::string& line) {
        std::vector<std::string> parts;
        size_t start = 0;
        while (start <= line.size()) {
            size_t sp = line.find(' ', start);
            if (sp == std::string::npos) sp = line.size();
            parts.push_back(line.substr(start, sp - start));
            start = sp + 1;
        }
        if (parts.size() != 3 || !parts[2].starts_with("HTTP/1.")) throw std::runtime_error("bad request line");
        method = parts[0];
        path = parts[1];
        state_ = State::Headers;
    }

    void header_line(const std::string& line) {
        if (line.empty()) {
            std::string len = header("content-length");
            content_length_ = len.empty() ? 0 : std::stoul(len);
            state_ = content_length_ > 0 ? State::Body : State::Done;
            return;
        }
        auto colon = line.find(':');
        if (colon == std::string::npos) throw std::runtime_error("bad header line");
        std::string name = line.substr(0, colon);
        std::transform(name.begin(), name.end(), name.begin(), [](unsigned char c) { return char(std::tolower(c)); });
        std::string value = line.substr(colon + 1);
        value.erase(0, value.find_first_not_of(' '));
        value.erase(value.find_last_not_of(' ') + 1);
        headers.emplace_back(name, value);
    }

    State state_ = State::RequestLine;
    std::string buffer_;
    size_t content_length_ = 0;
};

void try_parse(const char* label, const std::string& text) {
    RequestParser p;
    try {
        p.feed(text);
        std::cout << label << " -> ok\n";
    } catch (const std::runtime_error& e) {
        std::cout << label << " -> error: " << e.what() << '\n';
    }
}

int main() {
    std::string request =
        "POST /api/hosts HTTP/1.1\r\nHost: inventory\r\nContent-Length: 12\r\n"
        "Content-Type: application/json\r\n\r\n{\"name\":\"a\"}";
    RequestParser parser;
    int chunks = 0;
    for (size_t i = 0; i < request.size(); i += 7) {
        ++chunks;
        if (parser.feed(std::string_view(request).substr(i, 7))) break;
    }
    std::cout << parser.method << ' ' << parser.path << '\n';
    for (const auto& [k, v] : parser.headers) std::cout << k << ": " << v << '\n';
    std::cout << "body: " << parser.body << '\n';
    std::cout << "chunks fed: " << chunks << '\n';

    try_parse("GET /", "GET /\r\n\r\n");
    try_parse("header without colon", "GET / HTTP/1.1\r\nbadheader\r\n\r\n");
}

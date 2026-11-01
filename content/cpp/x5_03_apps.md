@@lesson x5-apps Building Real C++ Programs: Patterns, GUIs, Networking and Shipping
@@text
The standard library gives you containers and algorithms. Real applications also need structure for user actions, a user interface, network communication, and a way to reach users' machines. Here's how C++ developers handle each.

## The command pattern: actions, undo and scripting

Editors, CAD tools and games represent each user action as an **object** with `execute` and `undo`. That gives you undo/redo, macros, and a log of what the user did, almost for free:

```cpp
#include <iostream>
#include <memory>
#include <string>
#include <vector>

struct Document { std::string text; };

class Command {
public:
    virtual ~Command() = default;
    virtual void execute(Document& doc) = 0;
    virtual void undo(Document& doc) = 0;
};

class Insert : public Command {
public:
    Insert(std::size_t pos, std::string s) : pos_(pos), s_(std::move(s)) {}
    void execute(Document& doc) override { doc.text.insert(pos_, s_); }
    void undo(Document& doc) override { doc.text.erase(pos_, s_.size()); }
private:
    std::size_t pos_;
    std::string s_;
};

class History {
public:
    void run(std::unique_ptr<Command> cmd, Document& doc) {
        cmd->execute(doc);
        done_.push_back(std::move(cmd));
        undone_.clear();                       // a new action clears the redo list
    }
    void undo(Document& doc) {
        if (done_.empty()) return;
        done_.back()->undo(doc);
        undone_.push_back(std::move(done_.back()));
        done_.pop_back();
    }
    void redo(Document& doc) {
        if (undone_.empty()) return;
        undone_.back()->execute(doc);
        done_.push_back(std::move(undone_.back()));
        undone_.pop_back();
    }
private:
    std::vector<std::unique_ptr<Command>> done_, undone_;
};

int main() {
    Document doc;
    History h;
    h.run(std::make_unique<Insert>(0, "router"), doc);
    h.run(std::make_unique<Insert>(0, "core "), doc);
    std::cout << doc.text << '\n';
    h.undo(doc);
    std::cout << doc.text << '\n';
    h.redo(doc);
    std::cout << doc.text << '\n';
}
```

## The event loop

GUIs, servers and games all run the same shape of program: wait for an event (a click, a network packet, a timer, a frame), handle it quickly, repeat. Slow work must not block the loop: hand it to a worker thread, and post the result back to the loop.

```text
while (running) {
    Event e = waitForNextEvent();     // GUI toolkits, Asio and game engines provide this
    dispatch(e);                      // call the right handler; must return fast
}
```

## GUIs

| Library | Style | Notes |
|---|---|---|
| **Qt** (Qt Widgets, Qt Quick/QML) | retained-mode, signals and slots | the most complete cross-platform toolkit: Windows, macOS, Linux, embedded (cars, medical devices, factory panels) |
| **Dear ImGui** | immediate mode: redraw every frame | developer tools, game editors, debug UIs; extremely quick to build |
| **wxWidgets** | native widgets | lighter than Qt |
| **Win32 / WinUI** | Windows-only | the native Windows APIs |

A Qt window with a button that counts clicks (needs Qt installed and CMake's `find_package(Qt6 COMPONENTS Widgets)`):

```cpp,norun
#include <QApplication>
#include <QPushButton>

int main(int argc, char** argv) {
    QApplication app(argc, argv);
    int clicks = 0;
    QPushButton button("Click me");
    QObject::connect(&button, &QPushButton::clicked, [&] {
        button.setText(QString("Clicked %1 times").arg(++clicks));    // signal → slot (a lambda)
    });
    button.resize(240, 60);
    button.show();
    return app.exec();                                                 // the event loop
}
```

And the same idea in Dear ImGui, which redraws the whole UI every frame from your variables:

```cpp,norun
static int clicks = 0;
ImGui::Begin("Counter");
if (ImGui::Button("Click me")) ++clicks;          // true on the frame it was clicked
ImGui::Text("Clicked %d times", clicks);
ImGui::End();
```

## Networking

The standard library has no networking yet. The usual choices:

- **Asio** (standalone, or Boost.Asio): sockets, timers and async I/O; the basis of many servers.
- **libcurl** / **cpr**: HTTP clients.
- **Boost.Beast**, **Drogon**, **Crow**: HTTP and WebSocket servers.
- **gRPC** with Protocol Buffers: typed service-to-service calls, common in large companies.

```cpp,norun
// A synchronous TCP client with Asio: connect, send a line, read the reply.
asio::io_context io;
asio::ip::tcp::socket socket(io);
asio::connect(socket, asio::ip::tcp::resolver(io).resolve("127.0.0.1", "6379"));
asio::write(socket, asio::buffer(std::string("PING\r\n")));
asio::streambuf reply;
asio::read_until(socket, reply, "\r\n");
std::cout << std::istream(&reply).rdbuf();
```

Every byte from the network is untrusted: check lengths before copying, never trust a length field without a limit. Most security holes in C and C++ servers are buffer overflows in protocol parsers.

## Shipping

- **Build in Release**, with warnings as errors in CI, and run the tests with sanitizers in CI too.
- **Static vs dynamic linking**: static (`-static`, `/MT`) gives one self-contained file; dynamic needs the runtime (the Visual C++ Redistributable on Windows, `.so` files on Linux) installed or shipped beside it.
- **Installers**: CPack (part of CMake) makes `.msi`, `.deb`, `.rpm` and `.dmg` packages; Qt has `windeployqt` / `linuxdeploy` to collect its libraries.
- **Cross-platform CI**: GitHub Actions with a matrix of `windows-latest`, `ubuntu-latest`, `macos-latest` building the same CMake project (exactly how IT Quest itself is released).

@@quiz
What does the command pattern give an editor?
- Faster rendering
* Each action is an object with execute and undo, which makes undo/redo, macros and action logs easy
- Automatic saving
- Multi-threading
? The history is just a list of command objects.

@@quiz
In the History class, why does running a new command clear the redo list?
- To save memory
* After a new action, the undone actions no longer follow from the current state
- Because unique_ptr can't be stored twice
- It's required by the C++ standard
? Every editor behaves this way: undo, then type, and "redo" is no longer available.

@@quiz
Which GUI approach redraws the entire UI every frame from your program's variables?
- Qt Widgets
* Immediate mode (Dear ImGui)
- Win32
- wxWidgets
? Immediate-mode UIs have no widget objects to keep in sync, which makes them very fast to write for tools.

@@quiz
Your C++ server reads a 4-byte length from the network, then copies that many bytes into a buffer. What must it check?
- Nothing: TCP is reliable
* That the length is within a sane limit and fits the buffer, before copying
- That the length is even
- That the client is on the same subnet
? Trusting lengths from the network is the classic buffer overflow. Validate every field.

@@flashcards
command pattern :: actions as objects with execute/undo
undo/redo stacks :: done list + undone list; a new action clears redo
event loop :: wait for an event, dispatch it quickly, repeat
Qt :: complete cross-platform C++ GUI toolkit (signals and slots)
Dear ImGui :: immediate-mode GUI for tools and editors
Asio :: asynchronous networking library for C++
gRPC + Protocol Buffers :: typed RPC between services
static linking :: one self-contained executable
CPack :: CMake's installer and package generator
Visual C++ Redistributable :: the runtime DLLs dynamically linked MSVC programs need

@@exercise Undo and redo
Complete `History::undo` and `History::redo` (the same design as in the lesson) for a counter, so the program prints the expected values. Undo/redo with nothing to undo/redo must do nothing.
@@starter
#include <iostream>
#include <memory>
#include <vector>

class Command {
public:
    virtual ~Command() = default;
    virtual void execute(int& value) = 0;
    virtual void undo(int& value) = 0;
};

class Add : public Command {
public:
    explicit Add(int n) : n_(n) {}
    void execute(int& value) override { value += n_; }
    void undo(int& value) override { value -= n_; }
private:
    int n_;
};

class History {
public:
    void run(std::unique_ptr<Command> cmd, int& value) {
        cmd->execute(value);
        done_.push_back(std::move(cmd));
        undone_.clear();
    }
    void undo(int& value) {
        // TODO
    }
    void redo(int& value) {
        // TODO
    }
private:
    std::vector<std::unique_ptr<Command>> done_, undone_;
};

int main() {
    int v = 0;
    History h;
    h.run(std::make_unique<Add>(5), v);
    h.run(std::make_unique<Add>(10), v);
    std::cout << v << ' ';
    h.undo(v);
    std::cout << v << ' ';
    h.undo(v);
    h.undo(v);
    std::cout << v << ' ';
    h.redo(v);
    std::cout << v << ' ';
    h.run(std::make_unique<Add>(100), v);
    h.redo(v);
    std::cout << v << '\n';
}
@@expected
15 5 0 5 105
@@hint
`undo`: `if (done_.empty()) return; done_.back()->undo(value); undone_.push_back(std::move(done_.back())); done_.pop_back();`. `redo` is the mirror image, calling `execute`.
@@solution
#include <iostream>
#include <memory>
#include <vector>

class Command {
public:
    virtual ~Command() = default;
    virtual void execute(int& value) = 0;
    virtual void undo(int& value) = 0;
};

class Add : public Command {
public:
    explicit Add(int n) : n_(n) {}
    void execute(int& value) override { value += n_; }
    void undo(int& value) override { value -= n_; }
private:
    int n_;
};

class History {
public:
    void run(std::unique_ptr<Command> cmd, int& value) {
        cmd->execute(value);
        done_.push_back(std::move(cmd));
        undone_.clear();
    }
    void undo(int& value) {
        if (done_.empty()) return;
        done_.back()->undo(value);
        undone_.push_back(std::move(done_.back()));
        done_.pop_back();
    }
    void redo(int& value) {
        if (undone_.empty()) return;
        undone_.back()->execute(value);
        done_.push_back(std::move(undone_.back()));
        undone_.pop_back();
    }
private:
    std::vector<std::unique_ptr<Command>> done_, undone_;
};

int main() {
    int v = 0;
    History h;
    h.run(std::make_unique<Add>(5), v);
    h.run(std::make_unique<Add>(10), v);
    std::cout << v << ' ';
    h.undo(v);
    std::cout << v << ' ';
    h.undo(v);
    h.undo(v);
    std::cout << v << ' ';
    h.redo(v);
    std::cout << v << ' ';
    h.run(std::make_unique<Add>(100), v);
    h.redo(v);
    std::cout << v << '\n';
}

@@exercise A safe message parser
A protocol sends messages as `<length>:<payload>` one after another, e.g. `5:hello3:abc`. Write `std::vector<std::string> parseMessages(const std::string& data)` that returns the payloads, and **throws `std::runtime_error("bad message")`** if a length is missing, isn't a number, is over 1000, or is longer than the data that's left. Never read past the end of the input.
@@starter
#include <iostream>
#include <stdexcept>
#include <string>
#include <vector>

std::vector<std::string> parseMessages(const std::string& data) {
    // TODO
    return {};
}

int main() {
    for (auto& m : parseMessages("5:hello3:abc0:")) std::cout << "[" << m << "]";
    std::cout << '\n';
    for (std::string bad : {"5:hel", "x:abc", "3abc", "9999:a", "2:ab:"}) {
        try {
            parseMessages(bad);
            std::cout << bad << " accepted?!\n";
        } catch (const std::runtime_error& e) {
            std::cout << bad << " -> " << e.what() << '\n';
        }
    }
}
@@expected
[hello][abc][]
5:hel -> bad message
x:abc -> bad message
3abc -> bad message
9999:a -> bad message
2:ab: -> bad message
@@hint
Loop while `pos < data.size()`: find the next `':'` from `pos` (none → throw). The text between must be 1-4 digits (check with `std::all_of(..., ::isdigit)`), then `std::stoul`. If `len > 1000 || colon + 1 + len > data.size()` throw. Take `data.substr(colon + 1, len)` and set `pos = colon + 1 + len`.
@@solution
#include <algorithm>
#include <cctype>
#include <iostream>
#include <stdexcept>
#include <string>
#include <vector>

std::vector<std::string> parseMessages(const std::string& data) {
    std::vector<std::string> out;
    std::size_t pos = 0;
    while (pos < data.size()) {
        std::size_t colon = data.find(':', pos);
        if (colon == std::string::npos) throw std::runtime_error("bad message");
        std::string digits = data.substr(pos, colon - pos);
        if (digits.empty() || digits.size() > 4 ||
            !std::all_of(digits.begin(), digits.end(), [](unsigned char c) { return std::isdigit(c); }))
            throw std::runtime_error("bad message");
        std::size_t len = std::stoul(digits);
        if (len > 1000 || colon + 1 + len > data.size()) throw std::runtime_error("bad message");
        out.push_back(data.substr(colon + 1, len));
        pos = colon + 1 + len;
    }
    return out;
}

int main() {
    for (auto& m : parseMessages("5:hello3:abc0:")) std::cout << "[" << m << "]";
    std::cout << '\n';
    for (std::string bad : {"5:hel", "x:abc", "3abc", "9999:a", "2:ab:"}) {
        try {
            parseMessages(bad);
            std::cout << bad << " accepted?!\n";
        } catch (const std::runtime_error& e) {
            std::cout << bad << " -> " << e.what() << '\n';
        }
    }
}

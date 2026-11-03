@@lesson e6-polymorphism Polymorphism: Virtual Functions, variant and CRTP
@@concepts objects
@@text
C++ offers three kinds of polymorphism. Picking the right one is a design decision.

## 1. Virtual functions (runtime, open set)

```cpp,norun
class Notifier {
public:
    virtual ~Notifier() = default;                      // ALWAYS virtual in a base class
    virtual bool accepts(int severity) const = 0;      // pure virtual: must be overridden
    virtual std::string name() const = 0;
};

class Pager final : public Notifier {                  // final: can't be derived from further
public:
    bool accepts(int severity) const override { return severity >= 3; }   // override: compiler checks
    std::string name() const override { return "pager"; }
};

std::vector<std::unique_ptr<Notifier>> notifiers;     // hold by pointer
```

- Each polymorphic object carries a hidden pointer to a **vtable**; a virtual call is an indirect jump. That's cheap, but it prevents inlining.
- Without a virtual destructor, `delete basePtr` skips the derived destructor: undefined behaviour.
- **Slicing**: copying a derived object into a base *value* (`Notifier n = pager;`) cuts off the derived part. Use pointers or references.
- Anyone can add new derived classes later: the set of types is **open**.

## 2. std::variant + std::visit (closed set)

When you know every alternative up front (the messages in a protocol, the events in a log), a `std::variant` is a type-safe union with no heap allocation and no vtables. `std::visit` makes you handle every case:

```cpp,norun
template <class... Ts> struct overloaded : Ts... { using Ts::operator()...; };

using Event = std::variant<Login, Logout, Failed>;
std::visit(overloaded{
    [](const Login& e)  { ... },
    [](const Logout& e) { ... },
    [](const Failed& e) { ... },
}, event);
```

Adding a new event type makes every incomplete visitor fail to compile, which is exactly what you want.

## 3. Templates and CRTP (compile time)

The **Curiously Recurring Template Pattern** passes the derived class to its base as a template argument, so the base can call derived functions without virtual dispatch:

```cpp,norun
template <class Derived>
struct Counted {
    static inline int created = 0;
    Counted() { ++created; }
};
struct Session : Counted<Session> {};      // each derived class gets its own counter
```

C++23's "deducing this" simplifies many CRTP uses.

@@quiz
Why must a polymorphic base class have a virtual destructor?
* So deleting a derived object through a base pointer runs the derived destructor
- To make the class abstract
- For faster calls
- It's optional style
? Otherwise it's undefined behaviour.

@@quiz
When is `std::variant` a better fit than a class hierarchy?
- When plugins add new types at runtime
* When the set of alternatives is fixed and known, and you want every case handled
- Never
- When types need virtual functions
? Visitors fail to compile when a case is missing.

@@quiz
What is object slicing?
- Splitting a vector
* Copying a derived object into a base-class value, losing the derived part
- Removing virtual functions
- Using string_view
? Hold polymorphic objects by pointer or reference.

@@flashcards
`virtual` :: dispatch at runtime through the vtable
`= 0` :: pure virtual (abstract)
`override` :: compiler checks you really override
`final` :: no further overriding/deriving
Virtual destructor :: required in polymorphic bases
Slicing :: base value loses derived part
`std::variant` :: type-safe union of a fixed set
`std::visit` :: call the right lambda for the held type
`overloaded` :: combine lambdas into one visitor
CRTP :: base template parameterised by the derived class

@@exercise Notifiers with virtual dispatch
Build an alert router:

- abstract `Notifier` with a virtual destructor, `virtual bool accepts(int severity) const = 0` and `virtual std::string name() const = 0`
- `Email` (severity ≥ 1), `Slack` (≥ 2) and `Pager` (≥ 3, marked `final`), each printing `destroy <name>` in its destructor
- keep them in `std::vector<std::unique_ptr<Notifier>>`; for each alert, list the notifiers that accept it
- clearing the vector must run every derived destructor
@@starter
#include <iostream>
#include <memory>
#include <string>
#include <vector>

struct Alert {
    int severity;
    std::string text;
};

int main() {
    std::vector<Alert> alerts = {{1, "disk 70%"}, {2, "disk 85%"}, {3, "disk 99%"}};
    std::cout << "TODO\n";
}
@@expected
[sev 1] disk 70%: email
[sev 2] disk 85%: email, slack
[sev 3] disk 99%: email, slack, pager
destroy email
destroy slack
destroy pager
done
@@requires
virtual ~Notifier
override
final
@@hint
Put the destructor message in each derived class: `~Email() override { std::cout << "destroy email\n"; }`. Build the list of names with a `std::string` and a `", "` separator. Call `notifiers.clear()` before printing `done`.
@@solution
#include <iostream>
#include <memory>
#include <string>
#include <vector>

struct Alert {
    int severity;
    std::string text;
};

class Notifier {
public:
    virtual ~Notifier() = default;
    virtual bool accepts(int severity) const = 0;
    virtual std::string name() const = 0;
};

class Email : public Notifier {
public:
    ~Email() override { std::cout << "destroy email\n"; }
    bool accepts(int severity) const override { return severity >= 1; }
    std::string name() const override { return "email"; }
};

class Slack : public Notifier {
public:
    ~Slack() override { std::cout << "destroy slack\n"; }
    bool accepts(int severity) const override { return severity >= 2; }
    std::string name() const override { return "slack"; }
};

class Pager final : public Notifier {
public:
    ~Pager() override { std::cout << "destroy pager\n"; }
    bool accepts(int severity) const override { return severity >= 3; }
    std::string name() const override { return "pager"; }
};

int main() {
    std::vector<Alert> alerts = {{1, "disk 70%"}, {2, "disk 85%"}, {3, "disk 99%"}};
    std::vector<std::unique_ptr<Notifier>> notifiers;
    notifiers.push_back(std::make_unique<Email>());
    notifiers.push_back(std::make_unique<Slack>());
    notifiers.push_back(std::make_unique<Pager>());

    for (const auto& alert : alerts) {
        std::string targets;
        for (const auto& n : notifiers)
            if (n->accepts(alert.severity)) targets += (targets.empty() ? "" : ", ") + n->name();
        std::cout << "[sev " << alert.severity << "] " << alert.text << ": " << targets << '\n';
    }
    notifiers.clear();
    std::cout << "done\n";
}

@@exercise Login events with std::variant
Model security log events as `using Event = std::variant<Login, Logout, Failed>`:

- `Login { user, host }`, `Logout { user }`, `Failed { user, reason }`
- write the `overloaded` helper and use `std::visit` to print each event and to update a `Stats` struct (logins, logouts, failures, and the set of users with failures)
- at the end, print the stats and use `std::holds_alternative` to count the failures in the list
@@starter
#include <iostream>
#include <set>
#include <string>
#include <variant>
#include <vector>

struct Login { std::string user, host; };
struct Logout { std::string user; };
struct Failed { std::string user, reason; };
using Event = std::variant<Login, Logout, Failed>;

int main() {
    std::vector<Event> events = {
        Login{"ana", "web01"}, Failed{"bob", "bad password"}, Failed{"bob", "bad password"},
        Login{"bob", "web02"}, Logout{"ana"}, Failed{"eve", "unknown user"}};
    std::cout << "TODO\n";
}
@@expected
login ana on web01
FAILED bob: bad password
FAILED bob: bad password
login bob on web02
logout ana
FAILED eve: unknown user
logins=2 logouts=1 failures=3 users with failures: bob eve
holds_alternative<Failed>: 3
@@requires
std::visit
overloaded
std::holds_alternative<Failed>
@@hint
`template <class... Ts> struct overloaded : Ts... { using Ts::operator()...; };` (C++20 deduces the template arguments). Capture the stats by reference: `[&](const Failed& f) { ++stats.failures; stats.failed_users.insert(f.user); ... }`.
@@solution
#include <iostream>
#include <set>
#include <string>
#include <variant>
#include <vector>

struct Login { std::string user, host; };
struct Logout { std::string user; };
struct Failed { std::string user, reason; };
using Event = std::variant<Login, Logout, Failed>;

template <class... Ts>
struct overloaded : Ts... {
    using Ts::operator()...;
};
template <class... Ts>
overloaded(Ts...) -> overloaded<Ts...>;

struct Stats {
    int logins = 0, logouts = 0, failures = 0;
    std::set<std::string> failed_users;
};

int main() {
    std::vector<Event> events = {
        Login{"ana", "web01"}, Failed{"bob", "bad password"}, Failed{"bob", "bad password"},
        Login{"bob", "web02"}, Logout{"ana"}, Failed{"eve", "unknown user"}};
    Stats stats;
    for (const auto& e : events) {
        std::visit(overloaded{
            [&](const Login& l) {
                ++stats.logins;
                std::cout << "login " << l.user << " on " << l.host << '\n';
            },
            [&](const Logout& l) {
                ++stats.logouts;
                std::cout << "logout " << l.user << '\n';
            },
            [&](const Failed& f) {
                ++stats.failures;
                stats.failed_users.insert(f.user);
                std::cout << "FAILED " << f.user << ": " << f.reason << '\n';
            },
        }, e);
    }
    std::cout << "logins=" << stats.logins << " logouts=" << stats.logouts << " failures=" << stats.failures
              << " users with failures:";
    for (const auto& u : stats.failed_users) std::cout << ' ' << u;
    int failed = 0;
    for (const auto& e : events)
        if (std::holds_alternative<Failed>(e)) ++failed;
    std::cout << "\nholds_alternative<Failed>: " << failed << '\n';
}

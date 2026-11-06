@@lesson g8-qt Qt: Widgets, Signals and Slots, and the Object Tree
@@concepts objects, functions
@@text
**Qt** is the most widely used C++ GUI framework: KDE, OBS Studio, Autodesk tools, VirtualBox, Telegram Desktop, car dashboards and medical devices. It runs on Windows, macOS, Linux, Android, iOS and embedded Linux, and also includes networking, SQL, threading and serial ports. (It's dual-licensed: LGPL/GPL or commercial.)

## A first window

```cpp,norun
#include <QApplication>
#include <QLabel>
#include <QPushButton>
#include <QVBoxLayout>

int main(int argc, char* argv[]) {
    QApplication app(argc, argv);

    auto* window = new QWidget;
    auto* label = new QLabel("Not checked yet");
    auto* button = new QPushButton("Check servers");
    auto* layout = new QVBoxLayout(window);      // the layout and the widgets become children of window
    layout->addWidget(label);
    layout->addWidget(button);

    QObject::connect(button, &QPushButton::clicked, label, [label] { label->setText("All servers OK"); });

    window->show();
    return app.exec();                           // the event loop
}
```

```cmake,norun
find_package(Qt6 REQUIRED COMPONENTS Widgets)
qt_standard_project_setup()                       # turns on AUTOMOC
qt_add_executable(monitor main.cpp mainwindow.cpp mainwindow.ui)
target_link_libraries(monitor PRIVATE Qt6::Widgets)
```

## Signals and slots

A **signal** is an event an object announces (`clicked`, `textChanged`, `finished`); a **slot** is any function or lambda connected to it. One signal can drive many slots, and the sender doesn't know the receivers, so a `ServerModel` can update a table, a tray icon and a log without depending on any of them.

```cpp,norun
class ServerModel : public QObject {
    Q_OBJECT                                           // processed by moc, Qt's code generator
public:
    void setCpu(const QString& host, int cpu) { emit cpuChanged(host, cpu); }
signals:
    void cpuChanged(const QString& host, int cpu);
};

auto conn = connect(model, &ServerModel::cpuChanged, this, &MainWindow::updateRow);
disconnect(conn);
```

Connections made with a receiver object (the third argument) are removed automatically when the receiver is destroyed. Across threads, Qt queues the call to run in the receiver's thread: that's how worker threads safely update the UI.

## The object tree

Every `QObject` can have a **parent**. A parent deletes its children when it's destroyed, so you write `new QLabel(...)` with a parent and never `delete` it yourself. `findChild<QPushButton*>("refresh")` searches the tree by object name, and `setParent` moves an object to another branch. Widgets also get their on-screen position from the tree.

Widgets vs **Qt Quick/QML**: widgets are classic desktop controls written in C++; QML is a declarative language for fluid, touch-friendly and animated UIs, with C++ for the logic behind it. Both use the same signals, slots and properties.

@@quiz
What happens to a QLabel created with `new QLabel("x", window)` when `window` is deleted?
* It's deleted automatically, because window is its parent
- It leaks
- It becomes a top-level window
- The program crashes
? Parents own their children.

@@quiz
What does a signal know about the slots connected to it?
- Their names and types
* Nothing: it just emits, and the connections decide who runs
- Only the first slot
- Their return values
? That's what decouples senders from receivers.

@@quiz
What is moc?
- A testing tool
* Qt's Meta-Object Compiler, which generates the code behind signals, slots and properties from Q_OBJECT classes
- A memory checker
- A layout engine
? CMake's AUTOMOC runs it for you.

@@drill
Which Qt function links a signal to a slot?
= connect
= QObject::connect
? connect(button, &QPushButton::clicked, this, &Window::save).

@@flashcards
`QApplication::exec()` :: run the event loop
Signal :: event announced by an object
Slot :: function connected to a signal
`connect(sender, &Cls::sig, receiver, slot)` :: make a connection
`Q_OBJECT` :: enables signals/slots via moc
Parent/child :: parents delete their children
`findChild` :: search the object tree
Queued connection :: cross-thread slot call
QML / Qt Quick :: declarative UI language
`qt_add_executable` :: CMake helper for Qt apps

@@exercise Signals and slots from scratch
Write `template <class... Args> class Signal`:

- `int connect(std::function<void(Args...)>)` returns an id; `bool disconnect(int id)` returns whether it removed something
- `emit(args...)` calls the slots in connection order; it must work even if a slot disconnects itself during the emit (iterate over a copy)
- `size()` is the number of connections
- `ScopedConnection`: an RAII object that disconnects in its destructor (store a `std::function<void()>` that does the disconnecting)

Wire a `cpuChanged(host, cpu)` signal to a label, an alarm (above 90), a scoped logger and a one-shot slot that disconnects itself.
@@starter
#include <functional>
#include <iostream>
#include <string>
#include <utility>
#include <vector>

int main() {
    std::cout << "TODO\n";
}
@@expected
label: web01 42%
log: web01=42
one-shot: first update seen
label: web01 95%
ALARM web01 at 95%
label: db01 10%
slots connected: 2
disconnect twice: true false
@@requires
std::function<void(Args...)>
ScopedConnection
@@hint
Store `std::vector<std::pair<int, std::function<void(Args...)>>>`. The one-shot slot needs its own id: `int once = -1; once = cpuChanged.connect([&](const std::string&, int) { ...; cpuChanged.disconnect(once); });`. Put the logger's `ScopedConnection` and the one-shot slot in an inner `{ }` block together with the first emit.
@@solution
#include <algorithm>
#include <functional>
#include <iostream>
#include <string>
#include <utility>
#include <vector>

template <class... Args>
class Signal {
public:
    using Slot = std::function<void(Args...)>;

    int connect(Slot slot) {
        int id = next_id_++;
        slots_.emplace_back(id, std::move(slot));
        return id;
    }

    bool disconnect(int id) {
        auto removed = std::erase_if(slots_, [id](const auto& s) { return s.first == id; });
        return removed > 0;
    }

    void emit(Args... args) {
        auto copy = slots_;
        for (const auto& [id, slot] : copy)
            if (std::any_of(slots_.begin(), slots_.end(), [id](const auto& s) { return s.first == id; })) slot(args...);
    }

    size_t size() const { return slots_.size(); }

private:
    std::vector<std::pair<int, Slot>> slots_;
    int next_id_ = 0;
};

class ScopedConnection {
public:
    explicit ScopedConnection(std::function<void()> disconnect) : disconnect_(std::move(disconnect)) {}
    ScopedConnection(const ScopedConnection&) = delete;
    ScopedConnection& operator=(const ScopedConnection&) = delete;
    ~ScopedConnection() {
        if (disconnect_) disconnect_();
    }

private:
    std::function<void()> disconnect_;
};

int main() {
    Signal<const std::string&, int> cpuChanged;
    cpuChanged.connect([](const std::string& host, int cpu) { std::cout << "label: " << host << ' ' << cpu << "%\n"; });
    int alarm = cpuChanged.connect([](const std::string& host, int cpu) {
        if (cpu > 90) std::cout << "ALARM " << host << " at " << cpu << "%\n";
    });

    {
        int log = cpuChanged.connect([](const std::string& host, int cpu) { std::cout << "log: " << host << '=' << cpu << '\n'; });
        ScopedConnection scoped([&cpuChanged, log] { cpuChanged.disconnect(log); });
        int once = -1;
        once = cpuChanged.connect([&](const std::string&, int) {
            std::cout << "one-shot: first update seen\n";
            cpuChanged.disconnect(once);
        });
        cpuChanged.emit("web01", 42);
    }

    cpuChanged.emit("web01", 95);
    cpuChanged.emit("db01", 10);
    std::cout << "slots connected: " << cpuChanged.size() << '\n';
    bool first = cpuChanged.disconnect(alarm);
    bool second = cpuChanged.disconnect(alarm);
    std::cout << std::boolalpha << "disconnect twice: " << first << ' ' << second << '\n';
}

@@exercise An object tree with parent ownership
Recreate `QObject`'s ownership model:

- `Object(std::string name, Object* parent = nullptr)` registers itself with its parent
- the destructor prints `destroy <name>`, then deletes its children in order, then removes itself from its parent (if the parent isn't the one deleting it)
- `findChild(name)` searches depth-first; `path()` joins names from the root with `/`; `setParent(p)` moves the object
- a static counter tracks live objects

Build `window → toolbar → refresh, export` plus `window → table, statusbar`, print the tree, move `export` under `statusbar`, delete `toolbar` on its own, then delete `window`.
@@starter
#include <algorithm>
#include <iostream>
#include <string>
#include <string_view>
#include <vector>

int main() {
    std::cout << "TODO\n";
}
@@expected
window
  toolbar
    refresh
    export
  table
  statusbar
found: window/toolbar/export
after reparent: window/statusbar/export
destroy toolbar
destroy refresh
window children: table statusbar
destroy window
destroy table
destroy statusbar
destroy export
alive objects: 0
@@requires
findChild
setParent
@@hint
Children are raw `Object*` created with `new` and owned by the parent, like in Qt. In the destructor: `for (Object* c : std::vector(children_)) { c->parent_ = nullptr; delete c; }` and afterwards `if (parent_) std::erase(parent_->children_, this);`. A recursive `print(int depth)` indents with `2 * depth` spaces.
@@solution
#include <algorithm>
#include <iostream>
#include <string>
#include <string_view>
#include <vector>

class Object {
public:
    explicit Object(std::string name, Object* parent = nullptr) : name_(std::move(name)) {
        ++alive;
        setParent(parent);
    }

    Object(const Object&) = delete;
    Object& operator=(const Object&) = delete;

    virtual ~Object() {
        std::cout << "destroy " << name_ << '\n';
        for (Object* c : std::vector<Object*>(children_)) {
            c->parent_ = nullptr;
            delete c;
        }
        if (parent_) std::erase(parent_->children_, this);
        --alive;
    }

    void setParent(Object* parent) {
        if (parent_) std::erase(parent_->children_, this);
        parent_ = parent;
        if (parent_) parent_->children_.push_back(this);
    }

    Object* findChild(std::string_view name) {
        for (Object* c : children_) {
            if (c->name_ == name) return c;
            if (Object* found = c->findChild(name)) return found;
        }
        return nullptr;
    }

    std::string path() const { return parent_ ? parent_->path() + '/' + name_ : name_; }

    void print(int depth = 0) const {
        std::cout << std::string(depth * 2, ' ') << name_ << '\n';
        for (const Object* c : children_) c->print(depth + 1);
    }

    const std::vector<Object*>& children() const { return children_; }
    const std::string& name() const { return name_; }

    static inline int alive = 0;

private:
    std::string name_;
    Object* parent_ = nullptr;
    std::vector<Object*> children_;
};

int main() {
    auto* window = new Object("window");
    auto* toolbar = new Object("toolbar", window);
    new Object("refresh", toolbar);
    new Object("export", toolbar);
    new Object("table", window);
    auto* statusbar = new Object("statusbar", window);

    window->print();
    Object* exp = window->findChild("export");
    std::cout << "found: " << exp->path() << '\n';
    exp->setParent(statusbar);
    std::cout << "after reparent: " << exp->path() << '\n';

    delete toolbar;
    std::cout << "window children:";
    for (const Object* c : window->children()) std::cout << ' ' << c->name();
    std::cout << '\n';

    delete window;
    std::cout << "alive objects: " << Object::alive << '\n';
}

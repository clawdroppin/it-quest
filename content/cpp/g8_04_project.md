@@lesson g8-project Project: A Desktop Server Monitor
@@concepts concurrency, files
@@text
The capstone is a **server monitor**: a Qt Widgets (or QML) app that pings a list of servers, shows them in a sortable table, raises a tray notification when one goes down, and remembers its window size and settings.

```text,norun
monitor/
  CMakeLists.txt          Qt6::Widgets, Qt6::Network
  src/main.cpp            QApplication, MainWindow
  src/mainwindow.cpp/.ui  table view, search box, status bar
  src/hostmodel.cpp       QAbstractTableModel + QSortFilterProxyModel
  src/checker.cpp         worker in a QThread (or QtConcurrent), emits results
  src/settings.cpp        QSettings wrapper with typed accessors
  tests/                  Qt Test or GoogleTest for model and settings
```

## Everything goes through the event loop

A GUI app is a single **event loop** on the main thread:

- input events (mouse, keyboard) and paint events
- **timers**: `QTimer` fires every N milliseconds (refresh) or once (`QTimer::singleShot`)
- **posted events**: when a worker thread emits a signal connected to a UI object, Qt posts the call to the main thread's queue (a queued connection), and the loop runs it there
- never block the loop: a 3-second synchronous ping freezes the whole window

## Settings

`QSettings` stores key/value pairs in the right place per platform (registry on Windows, plist on macOS, INI files on Linux) with groups like `window/width`. Read with a default (`settings.value("window/width", 1024).toInt()`), and store window geometry on close. Writing the format yourself, as in the second exercise, shows the parsing and type-conversion problems it solves.

## Shipping

`windeployqt` / `macdeployqt` copy the Qt libraries next to your executable; package with an installer (Inno Setup, WiX, MSIX) or as an AppImage/Flatpak on Linux. Sign the binaries.

@@quiz
A worker thread finishes a ping. How does the result safely reach a label on the UI thread?
* The worker emits a signal; with a queued connection Qt posts the slot call to the main thread's event loop
- The worker calls label->setText directly
- The UI thread polls a global variable in a busy loop
- The worker locks the UI thread
? Widgets may only be touched from the main thread.

@@quiz
What happens if a button handler performs a 3-second blocking network call?
- The call runs in the background automatically
* The event loop is blocked: the window freezes for 3 seconds
- Qt cancels it
- Only that button freezes
? Use async APIs or a worker thread.

@@flashcards
Event loop :: processes input, paint, timers, posted events
`QTimer` :: repeating or single-shot timer
Queued connection :: slot call posted to the receiver's thread
`QThread` / `QtConcurrent::run` :: background work
`QSettings` :: persistent key/value settings
`windeployqt` :: bundle Qt DLLs with an app
Tray notification :: `QSystemTrayIcon::showMessage`

@@exercise An event loop with timers and posted events
Write a single-threaded `EventLoop` with a fake clock (`std::chrono::milliseconds now`):

- `post(fn)` is thread-safe (mutex) and queues a function to run on the loop
- `start_timer(interval, fn)` repeats and returns an id; `single_shot(delay, fn)` fires once; `stop_timer(id)`
- `run_until(t)`: repeatedly run all posted events, then advance the clock to the earliest due timer (ties by creation order) and fire it, until no timer is due at or before `t`; finish at `now = t`

A worker thread pings two servers and **posts** the results (join it before running the loop). A refresh timer runs every second and stops itself after 3 refreshes, posting a status bar update; a single-shot at 2.5 s shows a notification.
@@starter
#include <chrono>
#include <deque>
#include <functional>
#include <iostream>
#include <map>
#include <mutex>
#include <string>
#include <thread>

using std::chrono::milliseconds;

int main() {
    std::cout << "TODO\n";
}
@@expected
t=0 ping web01: ok 12ms
t=0 ping db01: timeout
t=1000 refresh #1
t=2000 refresh #2
t=2500 notification: db01 is down
t=3000 refresh #3 (stopping refresh timer)
t=3000 status bar updated
loop idle at t=3500, timers left: 0
@@requires
std::mutex
post(
single_shot
@@hint
Store timers in a `std::map<int, Timer>` with `struct Timer { milliseconds due, interval; bool repeat; std::function<void()> fn; }`; ids increase, so iterating the map in order breaks ties by creation. To fire: copy the function, update `due` (or erase a single-shot) **before** calling it, because the callback may stop its own timer. Drain posted events by swapping the queue out under the lock.
@@solution
#include <chrono>
#include <deque>
#include <functional>
#include <iostream>
#include <map>
#include <mutex>
#include <string>
#include <thread>

using std::chrono::milliseconds;

class EventLoop {
public:
    milliseconds now{0};

    void post(std::function<void()> fn) {
        std::lock_guard lock(m_);
        posted_.push_back(std::move(fn));
    }

    int start_timer(milliseconds interval, std::function<void()> fn) { return add(interval, interval, true, std::move(fn)); }
    int single_shot(milliseconds delay, std::function<void()> fn) { return add(delay, delay, false, std::move(fn)); }
    void stop_timer(int id) { timers_.erase(id); }
    size_t timers() const { return timers_.size(); }

    void run_until(milliseconds until) {
        while (true) {
            drain();
            auto next = timers_.end();
            for (auto it = timers_.begin(); it != timers_.end(); ++it)
                if (next == timers_.end() || it->second.due < next->second.due) next = it;
            if (next == timers_.end() || next->second.due > until) break;
            now = next->second.due;
            auto fn = next->second.fn;
            if (next->second.repeat) next->second.due += next->second.interval;
            else timers_.erase(next);
            fn();
        }
        now = until;
    }

private:
    struct Timer {
        milliseconds due, interval;
        bool repeat;
        std::function<void()> fn;
    };

    int add(milliseconds delay, milliseconds interval, bool repeat, std::function<void()> fn) {
        int id = next_id_++;
        timers_[id] = Timer{now + delay, interval, repeat, std::move(fn)};
        return id;
    }

    void drain() {
        while (true) {
            std::deque<std::function<void()>> batch;
            {
                std::lock_guard lock(m_);
                batch.swap(posted_);
            }
            if (batch.empty()) return;
            for (auto& fn : batch) fn();
        }
    }

    std::mutex m_;
    std::deque<std::function<void()>> posted_;
    std::map<int, Timer> timers_;
    int next_id_ = 1;
};

int main() {
    EventLoop loop;
    auto log = [&](const std::string& msg) { std::cout << "t=" << loop.now.count() << ' ' << msg << '\n'; };

    std::thread worker([&] {
        loop.post([&] { log("ping web01: ok 12ms"); });
        loop.post([&] { log("ping db01: timeout"); });
    });
    worker.join();

    int refreshes = 0, refresh_timer = 0;
    refresh_timer = loop.start_timer(milliseconds(1000), [&] {
        ++refreshes;
        if (refreshes == 3) {
            log("refresh #3 (stopping refresh timer)");
            loop.stop_timer(refresh_timer);
            loop.post([&] { log("status bar updated"); });
        } else {
            log("refresh #" + std::to_string(refreshes));
        }
    });
    loop.single_shot(milliseconds(2500), [&] { log("notification: db01 is down"); });

    loop.run_until(milliseconds(3500));
    std::cout << "loop idle at t=" << loop.now.count() << ", timers left: " << loop.timers() << '\n';
}

@@exercise Typed settings in an INI file
Write a small `QSettings`-style class:

- keys like `window/width` (group `window`) or `theme` (group `General`), stored as strings in a `std::map`
- `set(key, value)` for `bool` (`true`/`false`), integers and strings; `template <class T> T value(key, T fallback)` converts back, returning the fallback for a missing key **or** a value that doesn't parse (`std::from_chars` must consume the whole text)
- `to_ini()` writes `[group]` sections in sorted order with sorted `key=value` lines
- `from_ini(text, errors)` ignores blank lines and `;`/`#` comments, trims spaces around keys and values, and reports `line N: expected key=value`
@@starter
#include <charconv>
#include <iostream>
#include <map>
#include <sstream>
#include <string>
#include <string_view>
#include <type_traits>
#include <vector>

int main() {
    std::string user_file =
        "; monitor settings\n[window]\nwidth = 1280\nthis line is broken\nmaximized=false\n[servers]\nretries=abc\n";
    std::cout << "TODO\n";
}
@@expected
[General]
theme=dark
[servers]
refresh_ms=1000
[window]
height=600
maximized=true
width=1024
--
width=1280 maximized=false timeout_ms=5000 (default)
retries=abc -> default 3
errors: line 4: expected key=value
round trip equal: true
@@requires
if constexpr
std::from_chars
@@hint
Split a key at its last `/`: before it is the group, after it the name; no `/` means `General`. When parsing, keep the current group and store `group + "/" + key` (or just `key` for `General`). A `trim` helper with `find_first_not_of(" \t")` and `find_last_not_of(" \t")` handles the spaces.
@@solution
#include <charconv>
#include <iostream>
#include <map>
#include <sstream>
#include <string>
#include <string_view>
#include <type_traits>
#include <vector>

std::string trim(std::string_view s) {
    auto b = s.find_first_not_of(" \t");
    if (b == std::string_view::npos) return "";
    auto e = s.find_last_not_of(" \t");
    return std::string(s.substr(b, e - b + 1));
}

class Settings {
public:
    template <class T>
    void set(const std::string& key, const T& value) {
        if constexpr (std::is_same_v<T, bool>) values_[key] = value ? "true" : "false";
        else if constexpr (std::is_arithmetic_v<T>) values_[key] = std::to_string(value);
        else values_[key] = std::string(value);
    }

    template <class T>
    T value(const std::string& key, T fallback) const {
        auto it = values_.find(key);
        if (it == values_.end()) return fallback;
        const std::string& s = it->second;
        if constexpr (std::is_same_v<T, bool>) {
            if (s == "true") return true;
            if (s == "false") return false;
            return fallback;
        } else if constexpr (std::is_integral_v<T>) {
            T v{};
            auto [p, ec] = std::from_chars(s.data(), s.data() + s.size(), v);
            return ec == std::errc{} && p == s.data() + s.size() ? v : fallback;
        } else {
            return s;
        }
    }

    std::string to_ini() const {
        std::map<std::string, std::map<std::string, std::string>> groups;
        for (const auto& [key, v] : values_) {
            auto slash = key.rfind('/');
            if (slash == std::string::npos) groups["General"][key] = v;
            else groups[key.substr(0, slash)][key.substr(slash + 1)] = v;
        }
        std::string out;
        for (const auto& [group, entries] : groups) {
            out += '[' + group + "]\n";
            for (const auto& [k, v] : entries) out += k + '=' + v + '\n';
        }
        return out;
    }

    static Settings from_ini(const std::string& text, std::vector<std::string>& errors) {
        Settings s;
        std::istringstream in(text);
        std::string raw, group = "General";
        int line_no = 0;
        while (std::getline(in, raw)) {
            ++line_no;
            std::string line = trim(raw);
            if (line.empty() || line[0] == ';' || line[0] == '#') continue;
            if (line.front() == '[' && line.back() == ']') {
                group = line.substr(1, line.size() - 2);
                continue;
            }
            auto eq = line.find('=');
            if (eq == std::string::npos) {
                errors.push_back("line " + std::to_string(line_no) + ": expected key=value");
                continue;
            }
            std::string key = trim(std::string_view(line).substr(0, eq));
            std::string value = trim(std::string_view(line).substr(eq + 1));
            s.values_[group == "General" ? key : group + '/' + key] = value;
        }
        return s;
    }

    bool operator==(const Settings&) const = default;

private:
    std::map<std::string, std::string> values_;
};

int main() {
    Settings s;
    s.set("window/width", 1024);
    s.set("window/height", 600);
    s.set("window/maximized", true);
    s.set("theme", "dark");
    s.set("servers/refresh_ms", 1000);
    std::cout << s.to_ini() << "--\n";

    std::string user_file =
        "; monitor settings\n[window]\nwidth = 1280\nthis line is broken\nmaximized=false\n[servers]\nretries=abc\n";
    std::vector<std::string> errors;
    Settings user = Settings::from_ini(user_file, errors);
    std::cout << std::boolalpha << "width=" << user.value("window/width", 1024)
              << " maximized=" << user.value("window/maximized", true)
              << " timeout_ms=" << user.value("servers/timeout_ms", 5000) << " (default)\n";
    std::cout << "retries=" << user.value<std::string>("servers/retries", "") << " -> default "
              << user.value("servers/retries", 3) << '\n';
    for (const auto& e : errors) std::cout << "errors: " << e << '\n';

    std::vector<std::string> none;
    std::cout << "round trip equal: " << (Settings::from_ini(s.to_ini(), none) == s) << '\n';
}

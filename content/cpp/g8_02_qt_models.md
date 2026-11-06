@@lesson g8-models Qt Model/View and QML Property Bindings
@@concepts objects, lists
@@text
Showing 50,000 servers in a table with sorting and filtering is a classic IT-tool requirement. Qt solves it with **Model/View**: the data lives in a model, the view asks it only for the visible cells, and proxies sort and filter without copying.

## A table model

```cpp,norun
class HostModel : public QAbstractTableModel {
public:
    int rowCount(const QModelIndex& = {}) const override { return int(hosts_.size()); }
    int columnCount(const QModelIndex& = {}) const override { return 3; }

    QVariant data(const QModelIndex& index, int role) const override {
        const Host& h = hosts_[index.row()];
        if (role == Qt::DisplayRole) {
            switch (index.column()) {
            case 0: return h.name;
            case 1: return h.cpu;
            case 2: return h.online ? "yes" : "no";
            }
        }
        if (role == Qt::ForegroundRole && index.column() == 1 && h.cpu > 80) return QColor(Qt::red);
        return {};
    }

    QVariant headerData(int section, Qt::Orientation o, int role) const override;
    bool setData(const QModelIndex& index, const QVariant& value, int role) override;   // editing
};

auto* proxy = new QSortFilterProxyModel(this);
proxy->setSourceModel(model);
proxy->setFilterKeyColumn(0);
connect(searchBox, &QLineEdit::textChanged, proxy, &QSortFilterProxyModel::setFilterFixedString);
tableView->setModel(proxy);
tableView->setSortingEnabled(true);
```

- **Roles** let one cell provide several things: display text, colour, tooltip, icon, sort key.
- The model emits `dataChanged`, `rowsInserted` and so on; views and proxies update themselves.
- A proxy row isn't a source row: convert with `mapToSource`/`mapFromSource` before touching the data.

## QML and property bindings

In QML, a property can be bound to an **expression**. The engine records which properties the expression read, and re-evaluates it whenever one of them changes:

```qml,norun
Rectangle {
    color: server.cpu > settings.threshold ? "red" : "green"      // re-evaluated automatically
    Text { text: server.name + ": " + server.cpu + "%" }
}
```

Assigning a plain value to a bound property (`color = "blue"` in JavaScript) **breaks the binding**: it won't update again. Since Qt 6, C++ has the same feature with `QProperty<T>` and `QBindable`. The second exercise builds the dependency-tracking engine behind it.

@@quiz
Why use a proxy model for sorting and filtering?
* The source data stays unchanged and shared; the proxy only maps rows, and views update automatically
- Views can't sort by themselves
- Proxies make the data editable
- To copy the data for thread safety
? Several views can show the same model differently.

@@quiz
A table view sorted by a proxy shows a row at position 1. How do you edit the underlying data?
- Edit source row 1
* Map the proxy index to the source with mapToSource, then edit that row
- Call sort() first
- You can't edit through a proxy
? Proxy rows and source rows differ.

@@quiz
In QML, what happens after `color = "blue"` on a property that was bound to an expression?
- The binding updates to "blue" + expression
* The binding is removed; the property stays blue
- It throws an error
- Nothing changes
? Use Qt.binding(...) to create a new binding from JavaScript.

@@drill
Which Qt class do you subclass to provide data for a QTableView?
= QAbstractTableModel
? Implement rowCount, columnCount and data.

@@flashcards
`QAbstractTableModel` :: base for table models
Role :: kind of data for a cell (display, colour, ...)
`dataChanged` :: model signal for updated cells
`QSortFilterProxyModel` :: sort/filter without copying
`mapToSource` :: proxy row → source row
Property binding :: property computed from an expression
Dependency tracking :: record which properties a binding read
`QProperty<T>` :: C++ bindable property (Qt 6)

@@exercise A model with a sort/filter proxy
Write a `HostModel` (rows of name, cpu, online; `rowCount`, `columnCount`, `headerData`, `data(row, col)` returning text, `setData(row, col, text)` that notifies listeners) and a `SortFilterProxy`:

- the proxy keeps a vector of source row indices; `invalidate()` rebuilds it by filtering (name contains the filter text) and stable-sorting by the sort column (numeric for cpu), descending if requested
- the proxy registers with the model so any `setData` invalidates it
- `mapToSource(proxyRow)`
- `print(proxy)` shows the header and rows with `std::format("{:<8} {:<4} {}", ...)`

Sort by cpu descending, filter by `web`, then edit the cpu of proxy row 1 to 99 through `mapToSource`.
@@starter
#include <algorithm>
#include <format>
#include <functional>
#include <iostream>
#include <string>
#include <vector>

struct Host {
    std::string name;
    int cpu;
    bool online;
};

int main() {
    std::vector<Host> hosts = {{"web01", 42, true}, {"db01", 88, true}, {"web02", 5, false}, {"cache01", 63, true}};
    std::cout << "TODO\n";
}
@@expected
name     cpu  online
db01     88   yes
cache01  63   yes
web01    42   yes
web02    5    no
filter "web":
web01    42   yes
web02    5    no
proxy row 1 -> source row 2
web02    99   no
web01    42   yes
@@requires
mapToSource
std::stable_sort
@@hint
The model holds `std::vector<std::function<void()>> listeners_` and calls them at the end of `setData`. In the proxy's sort comparator, compare `std::stoi(model.data(a, 1))` for the cpu column and the text otherwise. `print` takes a `bool header` flag.
@@solution
#include <algorithm>
#include <format>
#include <functional>
#include <iostream>
#include <string>
#include <vector>

struct Host {
    std::string name;
    int cpu;
    bool online;
};

class HostModel {
public:
    explicit HostModel(std::vector<Host> hosts) : hosts_(std::move(hosts)) {}

    int rowCount() const { return static_cast<int>(hosts_.size()); }
    int columnCount() const { return 3; }

    std::string headerData(int col) const {
        static const char* names[] = {"name", "cpu", "online"};
        return names[col];
    }

    std::string data(int row, int col) const {
        const Host& h = hosts_[row];
        switch (col) {
        case 0: return h.name;
        case 1: return std::to_string(h.cpu);
        default: return h.online ? "yes" : "no";
        }
    }

    void setData(int row, int col, const std::string& value) {
        Host& h = hosts_[row];
        if (col == 0) h.name = value;
        else if (col == 1) h.cpu = std::stoi(value);
        else h.online = value == "yes";
        for (const auto& l : listeners_) l();
    }

    void onChanged(std::function<void()> listener) { listeners_.push_back(std::move(listener)); }

private:
    std::vector<Host> hosts_;
    std::vector<std::function<void()>> listeners_;
};

class SortFilterProxy {
public:
    explicit SortFilterProxy(HostModel& model) : model_(model) {
        model_.onChanged([this] { invalidate(); });
        invalidate();
    }

    void setFilter(std::string text) {
        filter_ = std::move(text);
        invalidate();
    }

    void sortBy(int column, bool descending) {
        sort_column_ = column;
        descending_ = descending;
        invalidate();
    }

    int rowCount() const { return static_cast<int>(rows_.size()); }
    int mapToSource(int proxy_row) const { return rows_[proxy_row]; }
    std::string data(int row, int col) const { return model_.data(mapToSource(row), col); }
    const HostModel& source() const { return model_; }

private:
    void invalidate() {
        rows_.clear();
        for (int r = 0; r < model_.rowCount(); ++r)
            if (model_.data(r, 0).find(filter_) != std::string::npos) rows_.push_back(r);
        if (sort_column_ < 0) return;
        std::stable_sort(rows_.begin(), rows_.end(), [&](int a, int b) {
            bool less = sort_column_ == 1 ? std::stoi(model_.data(a, 1)) < std::stoi(model_.data(b, 1))
                                          : model_.data(a, sort_column_) < model_.data(b, sort_column_);
            bool greater = sort_column_ == 1 ? std::stoi(model_.data(b, 1)) < std::stoi(model_.data(a, 1))
                                             : model_.data(b, sort_column_) < model_.data(a, sort_column_);
            return descending_ ? greater : less;
        });
    }

    HostModel& model_;
    std::vector<int> rows_;
    std::string filter_;
    int sort_column_ = -1;
    bool descending_ = false;
};

void print(const SortFilterProxy& proxy, bool header) {
    const HostModel& m = proxy.source();
    if (header) std::cout << std::format("{:<8} {:<4} {}\n", m.headerData(0), m.headerData(1), m.headerData(2));
    for (int r = 0; r < proxy.rowCount(); ++r)
        std::cout << std::format("{:<8} {:<4} {}\n", proxy.data(r, 0), proxy.data(r, 1), proxy.data(r, 2));
}

int main() {
    std::vector<Host> hosts = {{"web01", 42, true}, {"db01", 88, true}, {"web02", 5, false}, {"cache01", 63, true}};
    HostModel model(hosts);
    SortFilterProxy proxy(model);
    proxy.sortBy(1, true);
    print(proxy, true);

    proxy.setFilter("web");
    std::cout << "filter \"web\":\n";
    print(proxy, false);

    int source_row = proxy.mapToSource(1);
    std::cout << "proxy row 1 -> source row " << source_row << '\n';
    model.setData(source_row, 1, "99");
    print(proxy, false);
}

@@exercise QML-style property bindings
Write `Property<T>` with automatic dependency tracking:

- `get()` returns the value; if a binding is currently being evaluated (a global `current_binding` pointer), it records that binding as a dependent of this property
- `bind(f)` stores an expression and evaluates it; `reevaluate()` runs the expression with `current_binding` set to this property, then assigns the result
- assigning a **different** value calls `onChanged` and re-evaluates all dependents; `set(v)` removes the property's own binding first (like QML)

Bind `status` to host, cpu and threshold; `color` to status; `label` to status and color (bind them in that order), and print every label change.
@@starter
#include <algorithm>
#include <functional>
#include <iostream>
#include <string>
#include <vector>

int main() {
    std::cout << "TODO\n";
}
@@expected
label: web01: ok [green]
> cpu = 95
label: web01: HIGH [red]
> threshold = 99
label: web01: ok [green]
> host = db01
label: db01: ok [green]
> cpu = 50
> status.set("manual") breaks its binding
label: manual [green]
> cpu = 100
label updates: 5
@@requires
current_binding
reevaluate
@@hint
A non-template base `struct PropertyBase { std::vector<PropertyBase*> dependents; virtual void reevaluate() = 0; void notify() { auto copy = dependents; for (auto* d : copy) d->reevaluate(); } };` lets properties of different types depend on each other. In `get()`, add `current_binding` to `dependents` only if it isn't there yet. Because `color` is bound before `label`, a status change updates color first, so label never shows a half-updated state.
@@solution
#include <algorithm>
#include <functional>
#include <iostream>
#include <string>
#include <vector>

struct PropertyBase {
    std::vector<PropertyBase*> dependents;
    virtual ~PropertyBase() = default;
    virtual void reevaluate() = 0;
    void notify() {
        auto copy = dependents;
        for (auto* d : copy) d->reevaluate();
    }
};

inline PropertyBase* current_binding = nullptr;

template <class T>
class Property : public PropertyBase {
public:
    explicit Property(T value = {}) : value_(std::move(value)) {}

    const T& get() {
        if (current_binding && std::find(dependents.begin(), dependents.end(), current_binding) == dependents.end())
            dependents.push_back(current_binding);
        return value_;
    }

    void set(T value) {
        binding_ = nullptr;
        assign(std::move(value));
    }

    void bind(std::function<T()> expression) {
        binding_ = std::move(expression);
        reevaluate();
    }

    void reevaluate() override {
        if (!binding_) return;
        PropertyBase* previous = current_binding;
        current_binding = this;
        T value = binding_();
        current_binding = previous;
        assign(std::move(value));
    }

    std::function<void(const T&)> onChanged;

private:
    void assign(T value) {
        if (value == value_) return;
        value_ = std::move(value);
        if (onChanged) onChanged(value_);
        notify();
    }

    T value_;
    std::function<T()> binding_;
};

int main() {
    Property<std::string> host("web01");
    Property<int> cpu(42), threshold(80);
    Property<std::string> status, color, label;
    int updates = 0;
    label.onChanged = [&](const std::string& text) {
        ++updates;
        std::cout << "label: " << text << '\n';
    };

    status.bind([&] { return host.get() + ": " + (cpu.get() > threshold.get() ? "HIGH" : "ok"); });
    color.bind([&] { return std::string(status.get().find("HIGH") != std::string::npos ? "red" : "green"); });
    label.bind([&] { return status.get() + " [" + color.get() + "]"; });

    std::cout << "> cpu = 95\n";
    cpu.set(95);
    std::cout << "> threshold = 99\n";
    threshold.set(99);
    std::cout << "> host = db01\n";
    host.set("db01");
    std::cout << "> cpu = 50\n";
    cpu.set(50);
    std::cout << "> status.set(\"manual\") breaks its binding\n";
    status.set("manual");
    std::cout << "> cpu = 100\n";
    cpu.set(100);
    std::cout << "label updates: " << updates << '\n';
}

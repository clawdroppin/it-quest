@@lesson g8-imgui Immediate-Mode GUIs with Dear ImGui
@@concepts loops, functions
@@text
**Dear ImGui** takes a completely different approach from Qt. There are no widget objects, no signals and no layouts to keep in sync: every frame, your code simply *describes* the UI again, and functions like `Button` return true when they were clicked.

```cpp,norun
// inside the render loop, 60 times per second
ImGui::Begin("Server monitor");
ImGui::Checkbox("Monitoring", &settings.monitoring);       // edits the bool directly
ImGui::SliderInt("Threshold", &settings.threshold, 0, 100);
if (ImGui::Button("Restart all")) restart_all();            // true on the frame it was clicked

if (ImGui::BeginTable("hosts", 3)) {
    for (auto& h : hosts) {
        ImGui::PushID(h.name.c_str());                        // make the IDs inside this row unique
        ImGui::TableNextColumn(); ImGui::Text("%s", h.name.c_str());
        ImGui::TableNextColumn(); ImGui::ProgressBar(h.cpu / 100.0f);
        ImGui::TableNextColumn(); if (ImGui::SmallButton("Restart")) restart(h);
        ImGui::PopID();
    }
    ImGui::EndTable();
}
ImGui::End();
```

## How it works

- Each widget has an **ID**, a hash of its label and the **ID stack**. The library remembers a little state between frames under those IDs: which widget is **hot** (hovered) and which is **active** (being pressed or dragged).
- Two widgets with the same label in a loop get the same ID and steal each other's clicks. `PushID(i)` / `PopID()` (or a `##suffix` in the label) fixes it.
- The UI is just a function of your state, so there's no state to synchronise. That makes immediate-mode GUIs perfect for debug overlays, game editors, profilers (Tracy), embedded tools and quick internal utilities.
- ImGui produces vertex buffers that your renderer draws (DirectX, Vulkan, Metal, OpenGL, SDL backends included).

Trade-offs: it redraws continuously (fine in games, wasteful for an idle desktop app), and it has weaker accessibility and native look than Qt.

@@quiz
In immediate-mode UI, how do you know a button was clicked?
* The Button call returns true on that frame
- You connect a signal to it
- You subclass Button and override onClick
- You poll a global event queue
? The UI code runs every frame.

@@quiz
Why does a loop of `Button("Restart")` calls need PushID?
- For performance
* Same labels produce the same IDs, so the buttons would share hover and click state
- ImGui limits buttons per window
- It sorts the buttons
? `PushID(i)` mixes i into the ID hash.

@@quiz
Where does an immediate-mode GUI keep the application's state?
* In your own variables; the UI is rebuilt from them every frame
- In widget objects created once
- In a separate XML file
- In the GPU
? That's why ImGui code reads like ordinary straight-line code.

@@drill
Which ImGui function draws a button and returns true when it's clicked?
= ImGui::Button
= Button
? if (ImGui::Button("Save")) save();

@@flashcards
Immediate mode :: UI described anew every frame
Retained mode :: persistent widget objects (Qt, WPF)
Hot widget :: under the mouse
Active widget :: being pressed or dragged
ID stack :: PushID/PopID scopes widget IDs
`##label` :: hidden ID suffix in ImGui labels
`ImGui::Begin` / `End` :: a window
Backend :: renderer + platform glue (DX, Vulkan, SDL...)

@@exercise A tiny immediate-mode UI
Write `Ui`, a text-rendered immediate-mode toolkit. Each widget takes one row (row 0, 1, 2...), and the mouse is an `Input { x, y, down }`:

- `begin_frame(input)` resets the row and output; `end_frame()` clears `active` when the button isn't held and returns the rendered lines
- `pressed` means the button went down this frame (it wasn't down last frame)
- `checkbox(label, bool&)`: toggles when pressed on its row; renders `[x] label` or `[ ] label`
- `slider(label, int&, min, max)`: pressing on its row makes it active; while active and held, the value is `min + x * (max - min) / 10`, clamped, even if the mouse leaves the row; renders `label [=====-----] value` (10 cells)
- `button(label)`: returns true when pressed on its row; renders `( label )`
- each line starts with `>` if the mouse is on that row, else a space

Run the 9 input frames and print the state after each, then the last frame's rendering.
@@starter
#include <algorithm>
#include <cstdint>
#include <format>
#include <iostream>
#include <string>
#include <string_view>
#include <vector>

struct Input {
    int x = -1, y = -1;
    bool down = false;
};

int main() {
    std::vector<Input> frames = {{-1, -1, false}, {0, 0, true}, {0, 0, false}, {7, 1, true}, {3, 1, true},
                                 {9, 5, true},    {9, 5, false}, {0, 2, true},  {0, 2, true}};
    std::cout << "TODO\n";
}
@@expected
frame 1: monitoring=false threshold=50 restarts=0
frame 2: monitoring=true threshold=50 restarts=0
frame 3: monitoring=true threshold=50 restarts=0
frame 4: monitoring=true threshold=70 restarts=0
frame 5: monitoring=true threshold=30 restarts=0
frame 6: monitoring=true threshold=90 restarts=0
frame 7: monitoring=true threshold=90 restarts=0
frame 8: monitoring=true threshold=90 restarts=1
frame 9: monitoring=true threshold=90 restarts=1
 [x] Monitoring
 Threshold [=========-] 90
>( Restart all )
@@requires
begin_frame
active
@@hint
Give each widget an ID from its label (`std::hash<std::string_view>{}(label)`). In the slider: `if (hovered() && pressed()) active_ = id; if (active_ == id && in_.down) value = std::clamp(min + in_.x * (max - min) / 10, min, max);`. Keep `prev_down_` from the previous frame to detect presses. The application code is the same every frame: checkbox, slider, button.
@@solution
#include <algorithm>
#include <cstdint>
#include <format>
#include <functional>
#include <iostream>
#include <string>
#include <string_view>
#include <vector>

struct Input {
    int x = -1, y = -1;
    bool down = false;
};

class Ui {
public:
    void begin_frame(Input in) {
        prev_down_ = in_.down;
        in_ = in;
        row_ = 0;
        lines_.clear();
    }

    std::vector<std::string> end_frame() {
        if (!in_.down) active_ = 0;
        return lines_;
    }

    bool checkbox(std::string_view label, bool& value) {
        bool clicked = hovered() && pressed();
        if (clicked) value = !value;
        emit(std::format("[{}] {}", value ? 'x' : ' ', label));
        return clicked;
    }

    bool slider(std::string_view label, int& value, int min, int max) {
        size_t id = std::hash<std::string_view>{}(label);
        if (hovered() && pressed()) active_ = id;
        int before = value;
        if (active_ == id && in_.down) value = std::clamp(min + in_.x * (max - min) / 10, min, max);
        int filled = (value - min) * 10 / (max - min);
        emit(std::format("{} [{}{}] {}", label, std::string(filled, '='), std::string(10 - filled, '-'), value));
        return value != before;
    }

    bool button(std::string_view label) {
        bool clicked = hovered() && pressed();
        emit(std::format("( {} )", label));
        return clicked;
    }

private:
    bool hovered() const { return in_.y == row_; }
    bool pressed() const { return in_.down && !prev_down_; }

    void emit(const std::string& text) {
        lines_.push_back((hovered() ? ">" : " ") + text);
        ++row_;
    }

    Input in_;
    bool prev_down_ = false;
    int row_ = 0;
    size_t active_ = 0;
    std::vector<std::string> lines_;
};

int main() {
    std::vector<Input> frames = {{-1, -1, false}, {0, 0, true}, {0, 0, false}, {7, 1, true}, {3, 1, true},
                                 {9, 5, true},    {9, 5, false}, {0, 2, true},  {0, 2, true}};
    bool monitoring = false;
    int threshold = 50, restarts = 0;
    Ui ui;
    std::vector<std::string> last;
    for (size_t f = 0; f < frames.size(); ++f) {
        ui.begin_frame(frames[f]);
        ui.checkbox("Monitoring", monitoring);
        ui.slider("Threshold", threshold, 0, 100);
        if (ui.button("Restart all")) ++restarts;
        last = ui.end_frame();
        std::cout << std::boolalpha << "frame " << f + 1 << ": monitoring=" << monitoring << " threshold=" << threshold
                  << " restarts=" << restarts << '\n';
    }
    for (const auto& line : last) std::cout << line << '\n';
}

@@exercise The ID stack
Show why ImGui needs `PushID`:

- `Ui` gives each button an ID by hashing its label together with the current ID stack (FNV-1a over the stack values and the label characters), and records a warning when an ID is used twice in one frame
- `push_id(int)` / `pop_id()` maintain the stack
- `button(label)` returns true when the mouse was pressed on its row (rows count up from 0)

Render a row with a `Restart` button for each of `web01`, `web02`, `db01`, first without and then with `push_id(i)`, and in the second version click row 2.
@@starter
#include <cstdint>
#include <iostream>
#include <set>
#include <string>
#include <string_view>
#include <vector>

int main() {
    std::vector<std::string> servers = {"web01", "web02", "db01"};
    std::cout << "TODO\n";
}
@@expected
without push_id: 2 duplicate id warnings
with push_id: 0 duplicate id warnings
clicked row 2: restart db01
@@requires
push_id
pop_id
@@hint
FNV-1a: `uint32_t h = 2166136261u; for each byte b: h = (h ^ b) * 16777619u;`. Feed the stack values first (each `int` as its 4 bytes, or simply as one value), then the label. A `std::set<uint32_t> seen_` per frame detects duplicates: `if (!seen_.insert(id).second) ++warnings;`.
@@solution
#include <cstdint>
#include <iostream>
#include <set>
#include <string>
#include <string_view>
#include <vector>

class Ui {
public:
    void begin_frame(int click_row) {
        click_row_ = click_row;
        row_ = 0;
        seen_.clear();
        warnings = 0;
    }

    void push_id(int id) { stack_.push_back(id); }
    void pop_id() { stack_.pop_back(); }

    bool button(std::string_view label) {
        if (!seen_.insert(make_id(label)).second) ++warnings;
        return row_++ == click_row_;
    }

    int warnings = 0;

private:
    uint32_t make_id(std::string_view label) const {
        uint32_t h = 2166136261u;
        auto mix = [&h](uint32_t byte) { h = (h ^ byte) * 16777619u; };
        for (int v : stack_)
            for (int shift = 0; shift < 32; shift += 8) mix((static_cast<uint32_t>(v) >> shift) & 0xFF);
        for (char c : label) mix(static_cast<unsigned char>(c));
        return h;
    }

    std::vector<int> stack_;
    std::set<uint32_t> seen_;
    int row_ = 0;
    int click_row_ = -1;
};

int main() {
    std::vector<std::string> servers = {"web01", "web02", "db01"};
    Ui ui;

    ui.begin_frame(-1);
    for (size_t i = 0; i < servers.size(); ++i) ui.button("Restart");
    std::cout << "without push_id: " << ui.warnings << " duplicate id warnings\n";

    ui.begin_frame(2);
    std::string clicked;
    for (size_t i = 0; i < servers.size(); ++i) {
        ui.push_id(static_cast<int>(i));
        if (ui.button("Restart")) clicked = servers[i];
        ui.pop_id();
    }
    std::cout << "with push_id: " << ui.warnings << " duplicate id warnings\n";
    std::cout << "clicked row 2: restart " << clicked << '\n';
}

@@lesson ru0-architecture App Architecture: Messages, Update, Undo and Async Work
@@concepts objects, functions, lists
@@lang rust
@@text
Small GUI apps start with state sprinkled through widget callbacks; they become hard to change quickly. Rust GUI toolkits push you towards a cleaner shape: iced uses **The Elm Architecture** (state, messages, `update`, `view`), and the same pattern works in egui, Slint and Tauri front ends. This lesson explains it, shows how it makes **undo/redo** and testing easy, and how to keep slow work off the UI thread.

## The Elm Architecture

1. **State**: one struct holds everything the UI shows.
2. **Message**: an enum of everything that can happen (`AddTodo(String)`, `Toggle(usize)`, `FilterChanged(Filter)`, `Loaded(Result<Data, Error>)`).
3. **update(&mut state, message)**: the only place state changes.
4. **view(&state)**: builds widgets from state; widgets emit messages.

Because `update` is a plain function, you can unit-test the whole app logic without opening a window: feed messages, assert on state.

In egui (immediate mode) you can follow the same discipline: collect `Message`s while drawing the frame, then apply them in one `update` call after it.

## Derived state

Don't store what you can compute: a filtered list, a count of remaining items or a "can save" flag should be methods on the state. Stored copies drift out of sync.

## Undo and redo

Two common designs:
- **Snapshots**: push a clone of the state (or the part that changes) before each change. Simple; fine for small states.
- **Commands**: store each change with enough information to reverse it (`Rename { id, old, new }`). Memory-efficient; needs care.
Rules: a new edit **clears the redo stack**; group rapid edits (typing) into one undo step; don't record view-only changes (scrolling, selection) unless users expect it; cap the history size.

## Async work

Never block the UI thread with file, network or heavy computation: the window freezes and the OS may mark it "not responding".
- iced: return a `Task` (formerly `Command`) from `update`; its result comes back as a message.
- egui: spawn a thread or async task, send results through a channel (`std::sync::mpsc`), and call `ctx.request_repaint()`; poll the channel each frame.
- Tauri: `#[tauri::command] async fn` runs off the main thread; emit events to the front end for progress.
Track **loading / loaded / failed** explicitly in state (an enum), and ignore stale responses (keep a request ID).

@@quiz
In The Elm Architecture, where is state allowed to change?
- In view
* In update, in response to a message
- In any widget callback
- In main only
? One place, easy to test.

@@quiz
Why compute "items left" with a method rather than storing it?
* Stored copies can drift out of sync with the list
- Methods are faster
- Rust forbids storing numbers
- It's required by iced
? Derived state.

@@quiz
A user undoes twice, then types a new character. What happens to the redo stack?
- It's kept
* It's cleared
- The new edit is added to it
- It's reversed
? A new branch of history starts.

@@quiz
How should an egui app load a large file?
- Read it inside the frame function
* Read it on another thread and send the result through a channel, requesting a repaint
- Increase the frame time
- Use a bigger window
? Never block the UI thread.

@@quiz
Why keep a request ID with async searches?
- To encrypt the request
* To ignore stale responses that arrive after a newer request
- To limit memory
- It's required by Tokio
? Out-of-order responses are common.

@@flashcards
TEA :: state, message, update, view
Message enum :: everything that can happen
update :: only place state changes
view :: widgets from state
derived state :: compute, don't store
snapshot undo :: clone before change
command undo :: store reversible changes
new edit :: clears redo
coalesce typing :: one undo step
iced Task :: async result as message
egui + channel :: background work, request_repaint
Loading enum :: loading / loaded / failed
request ID :: drop stale responses

@@drill
In The Elm Architecture, which function builds the widgets from state?
= view
? update changes state.

@@drill
What happens to the redo stack when a new edit is made?
= it is cleared
= cleared
= it's cleared
= emptied
? History branches.

@@exercise A todo app with update and undo
Implement `update` for the messages and snapshot-based undo/redo: every message except `Undo`, `Redo` and `SetFilter` pushes a snapshot of the todo list first and clears the redo stack; history is capped at `MAX_HISTORY`. `visible()` returns titles matching the filter, and `left()` counts undone items.
@@starter
#[derive(Clone, Debug, PartialEq)]
struct Todo { title: String, done: bool }

#[derive(Clone, Copy, Debug, PartialEq)]
enum Filter { All, Active, Done }

enum Message { Add(String), Toggle(usize), Remove(usize), SetFilter(Filter), Undo, Redo }

const MAX_HISTORY: usize = 3;

struct App { todos: Vec<Todo>, filter: Filter, undo: Vec<Vec<Todo>>, redo: Vec<Vec<Todo>> }

impl App {
    fn new() -> Self { App { todos: vec![], filter: Filter::All, undo: vec![], redo: vec![] } }

    fn update(&mut self, msg: Message) {
        todo!()
    }

    fn visible(&self) -> Vec<&str> {
        todo!()
    }

    fn left(&self) -> usize {
        todo!()
    }
}

fn main() {
    let mut app = App::new();
    for m in [Message::Add("Buy milk".into()), Message::Add("Write lesson".into()), Message::Toggle(0)] {
        app.update(m);
    }
    println!("{:?}, {} left", app.visible(), app.left());
}
@@tests
#[test]
fn add_toggle_filter() {
    let mut app = App::new();
    app.update(Message::Add("a".into()));
    app.update(Message::Add("b".into()));
    app.update(Message::Add("  ".into()));
    assert_eq!(app.todos.len(), 2, "blank titles are ignored");
    app.update(Message::Toggle(1));
    assert_eq!(app.left(), 1);
    app.update(Message::SetFilter(Filter::Done));
    assert_eq!(app.visible(), vec!["b"]);
    app.update(Message::SetFilter(Filter::Active));
    assert_eq!(app.visible(), vec!["a"]);
    app.update(Message::Toggle(9));
    app.update(Message::Remove(9));
    assert_eq!(app.todos.len(), 2, "out-of-range indexes are ignored");
}

#[test]
fn undo_redo() {
    let mut app = App::new();
    app.update(Message::Add("a".into()));
    app.update(Message::Add("b".into()));
    app.update(Message::Remove(0));
    assert_eq!(app.visible(), vec!["b"]);
    app.update(Message::Undo);
    assert_eq!(app.visible(), vec!["a", "b"]);
    app.update(Message::Undo);
    assert_eq!(app.visible(), vec!["a"]);
    app.update(Message::Redo);
    assert_eq!(app.visible(), vec!["a", "b"]);
    app.update(Message::SetFilter(Filter::Done));
    app.update(Message::Undo);
    app.update(Message::SetFilter(Filter::All));
    assert_eq!(app.visible(), vec!["a"], "filter changes are not undo steps");
    app.update(Message::Add("c".into()));
    app.update(Message::Redo);
    assert_eq!(app.visible(), vec!["a", "c"], "a new edit clears redo");
    app.update(Message::Undo);
    app.update(Message::Undo);
    app.update(Message::Undo);
    assert_eq!(app.todos.len(), 0);
    app.update(Message::Undo);
    assert_eq!(app.todos.len(), 0, "undo with empty history does nothing");
}

#[test]
fn history_is_capped() {
    let mut app = App::new();
    for t in ["a", "b", "c", "d", "e"] {
        app.update(Message::Add(t.into()));
    }
    assert_eq!(app.undo.len(), MAX_HISTORY);
    for _ in 0..10 {
        app.update(Message::Undo);
    }
    assert_eq!(app.visible(), vec!["a", "b"]);
}
@@hint
Write a helper `fn record(&mut self)` that pushes `self.todos.clone()`, removes the oldest entry (`self.undo.remove(0)`) when longer than `MAX_HISTORY`, and clears `redo`. Only record when the change will actually happen (a valid index, a non-blank title). For `Undo`: `if let Some(prev) = self.undo.pop() { self.redo.push(std::mem::replace(&mut self.todos, prev)); }`, and the mirror image for `Redo`.
@@solution
#[derive(Clone, Debug, PartialEq)]
struct Todo { title: String, done: bool }

#[derive(Clone, Copy, Debug, PartialEq)]
enum Filter { All, Active, Done }

enum Message { Add(String), Toggle(usize), Remove(usize), SetFilter(Filter), Undo, Redo }

const MAX_HISTORY: usize = 3;

struct App { todos: Vec<Todo>, filter: Filter, undo: Vec<Vec<Todo>>, redo: Vec<Vec<Todo>> }

impl App {
    fn new() -> Self { App { todos: vec![], filter: Filter::All, undo: vec![], redo: vec![] } }

    fn record(&mut self) {
        self.undo.push(self.todos.clone());
        if self.undo.len() > MAX_HISTORY {
            self.undo.remove(0);
        }
        self.redo.clear();
    }

    fn update(&mut self, msg: Message) {
        match msg {
            Message::Add(title) => {
                let title = title.trim().to_string();
                if !title.is_empty() {
                    self.record();
                    self.todos.push(Todo { title, done: false });
                }
            }
            Message::Toggle(i) => {
                if i < self.todos.len() {
                    self.record();
                    self.todos[i].done = !self.todos[i].done;
                }
            }
            Message::Remove(i) => {
                if i < self.todos.len() {
                    self.record();
                    self.todos.remove(i);
                }
            }
            Message::SetFilter(f) => self.filter = f,
            Message::Undo => {
                if let Some(prev) = self.undo.pop() {
                    self.redo.push(std::mem::replace(&mut self.todos, prev));
                }
            }
            Message::Redo => {
                if let Some(next) = self.redo.pop() {
                    self.undo.push(std::mem::replace(&mut self.todos, next));
                }
            }
        }
    }

    fn visible(&self) -> Vec<&str> {
        self.todos
            .iter()
            .filter(|t| match self.filter {
                Filter::All => true,
                Filter::Active => !t.done,
                Filter::Done => t.done,
            })
            .map(|t| t.title.as_str())
            .collect()
    }

    fn left(&self) -> usize {
        self.todos.iter().filter(|t| !t.done).count()
    }
}

fn main() {
    let mut app = App::new();
    for m in [Message::Add("Buy milk".into()), Message::Add("Write lesson".into()), Message::Toggle(0)] {
        app.update(m);
    }
    println!("{:?}, {} left", app.visible(), app.left());
}

@@exercise Background work without freezing the UI
Simulate an egui-style app that starts searches on a worker thread and polls a channel once per "frame". Each search carries a request ID; the app shows `Loading` until the newest request's result arrives and ignores stale results from older requests.
@@starter
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq, Clone)]
enum Status { Idle, Loading(u32), Ready(Vec<String>) }

struct App { status: Status, next_id: u32, tx: Sender<(u32, Vec<String>)>, rx: Receiver<(u32, Vec<String>)> }

const WORDS: [&str; 6] = ["apple", "apricot", "banana", "blueberry", "cherry", "grape"];

impl App {
    fn new() -> Self {
        let (tx, rx) = channel();
        App { status: Status::Idle, next_id: 0, tx, rx }
    }

    /// Start a search on another thread; `delay_ms` simulates slow work.
    fn search(&mut self, query: &str, delay_ms: u64) {
        todo!()
    }

    /// Called once per frame: apply any results that arrived, ignoring stale ones.
    fn poll(&mut self) {
        todo!()
    }
}

fn main() {
    let mut app = App::new();
    app.search("ap", 10);
    while matches!(app.status, Status::Loading(_)) {
        app.poll();
        thread::sleep(Duration::from_millis(2));
    }
    println!("{:?}", app.status);
}
@@tests
fn wait(app: &mut App) {
    for _ in 0..500 {
        app.poll();
        if !matches!(app.status, Status::Loading(_)) {
            return;
        }
        thread::sleep(Duration::from_millis(2));
    }
    panic!("never finished loading");
}

#[test]
fn single_search() {
    let mut app = App::new();
    assert_eq!(app.status, Status::Idle);
    app.search("b", 5);
    assert_eq!(app.status, Status::Loading(1), "search starts loading immediately with id 1");
    wait(&mut app);
    assert_eq!(app.status, Status::Ready(vec!["banana".to_string(), "blueberry".to_string()]));
}

#[test]
fn stale_results_are_ignored() {
    let mut app = App::new();
    app.search("a", 120);
    app.search("ch", 5);
    assert_eq!(app.status, Status::Loading(2));
    wait(&mut app);
    assert_eq!(app.status, Status::Ready(vec!["cherry".to_string()]));
    thread::sleep(Duration::from_millis(200));
    app.poll();
    assert_eq!(app.status, Status::Ready(vec!["cherry".to_string()]), "the slow, older result must not replace the newer one");
}
@@hint
In `search`: increment `next_id`, set `Loading(id)`, clone the sender and the query (`query.to_string()`), and `thread::spawn(move || { thread::sleep(...); let hits = WORDS.iter().filter(|w| w.starts_with(&q)).map(|w| w.to_string()).collect(); let _ = tx.send((id, hits)); })`. In `poll`: `while let Ok((id, hits)) = self.rx.try_recv()`, and accept only when `self.status == Status::Loading(id)`.
@@solution
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq, Clone)]
enum Status { Idle, Loading(u32), Ready(Vec<String>) }

struct App { status: Status, next_id: u32, tx: Sender<(u32, Vec<String>)>, rx: Receiver<(u32, Vec<String>)> }

const WORDS: [&str; 6] = ["apple", "apricot", "banana", "blueberry", "cherry", "grape"];

impl App {
    fn new() -> Self {
        let (tx, rx) = channel();
        App { status: Status::Idle, next_id: 0, tx, rx }
    }

    /// Start a search on another thread; `delay_ms` simulates slow work.
    fn search(&mut self, query: &str, delay_ms: u64) {
        self.next_id += 1;
        let id = self.next_id;
        self.status = Status::Loading(id);
        let tx = self.tx.clone();
        let q = query.to_string();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(delay_ms));
            let hits: Vec<String> = WORDS.iter().filter(|w| w.starts_with(&q)).map(|w| w.to_string()).collect();
            let _ = tx.send((id, hits));
        });
    }

    /// Called once per frame: apply any results that arrived, ignoring stale ones.
    fn poll(&mut self) {
        while let Ok((id, hits)) = self.rx.try_recv() {
            if self.status == Status::Loading(id) {
                self.status = Status::Ready(hits);
            }
        }
    }
}

fn main() {
    let mut app = App::new();
    app.search("ap", 10);
    while matches!(app.status, Status::Loading(_)) {
        app.poll();
        thread::sleep(Duration::from_millis(2));
    }
    println!("{:?}", app.status);
}

@@lesson xp-kanban Kanban Board in the Browser
@@text
A Trello-style board for your study tasks: columns To do, Doing and Done, cards you can add, edit, drag between columns and delete, saved in the browser so it survives a reload. Plain HTML, CSS and JavaScript, no framework.

**You'll practise:** DOM creation · events and event delegation · drag and drop API · localStorage and JSON · state → render

## Milestones

1. **State**: one array of cards `{id, title, column}` and a `render()` that draws everything from it.
2. **Add and delete**: a form per column, a ✕ button per card.
3. **Drag and drop**: `dragstart`, `dragover`, `drop` move a card to another column.
4. **Save**: write the state to localStorage after every change; load it at start.
5. **Polish**: edit a card's title by double-clicking; count cards per column.

## You're done when

- Cards can be added, moved and deleted, and everything is still there after a reload.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"Why should I re-render from state instead of moving DOM elements around?"*
- *"My drop event never fires. What am I missing?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project kanban Kanban Board in the Browser
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Kanban Board in the Browser

A Trello-style board for your study tasks: columns To do, Doing and Done, cards you can add, edit, drag between columns and delete, saved in the browser so it survives a reload. Plain HTML, CSS and JavaScript, no framework.

## Milestones

- [ ] 1. State: one array of cards `{id, title, column}` and a `render()` that draws everything from it.
- [ ] 2. Add and delete: a form per column, a ✕ button per card.
- [ ] 3. Drag and drop: `dragstart`, `dragover`, `drop` move a card to another column.
- [ ] 4. Save: write the state to localStorage after every change; load it at start.
- [ ] 5. Polish: edit a card's title by double-clicking; count cards per column.

## Done when

- [ ] Cards can be added, moved and deleted, and everything is still there after a reload.
@@file index.html
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Kanban</title>
  <link rel="stylesheet" href="style.css">
</head>
<body>
  <h1>Study board</h1>
  <main id="board"></main>
  <script src="app.js"></script>
</body>
</html>
@@file style.css
body { font-family: system-ui, sans-serif; margin: 2rem; background: #f4f5f7; }
#board { display: grid; grid-template-columns: repeat(3, 1fr); gap: 1rem; }
.column { background: #ebecf0; border-radius: 8px; padding: .75rem; min-height: 200px; }
.card { background: white; border-radius: 6px; padding: .5rem; margin: .5rem 0; box-shadow: 0 1px 2px #0002; cursor: grab; }
@@file app.js
// Kanban board: all state in one array, drawn by render().
const COLUMNS = ["To do", "Doing", "Done"];
let cards = JSON.parse(localStorage.getItem("cards") ?? "[]");

function save() {
  localStorage.setItem("cards", JSON.stringify(cards));
}

function render() {
  const board = document.getElementById("board");
  board.innerHTML = "";
  for (const name of COLUMNS) {
    const col = document.createElement("section");
    col.className = "column";
    col.innerHTML = `<h2>${name}</h2>`;
    // TODO: draw this column's cards, an add form, and drag-and-drop handlers
    board.append(col);
  }
}

render();

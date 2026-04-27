Build a high-performance, native Obsidian-like note-taking application in pure Rust with the Iced GUI library, targeting Windows and Linux. Follow the Elm Architecture rigorously, use `frostmark` for Markdown rendering, and `iced_aw` for syntax highlighting. The app must be type-safe, resource-efficient, and fully modular.

## Project Name
**RustyNotes**

## Core Features (MVP)
- **Vault-based notes**: All notes are plain Markdown files inside a chosen folder (the vault). Support nested subfolders.
- **File explorer**: Left sidebar showing the vault’s folder tree. Create, rename, delete notes and folders.
- **Markdown editor with live preview**: Split‑pane view – raw Markdown on the left, rendered HTML on the right. Preview updates in real‑time.
- **Wikilinks**: `[[note name]]` syntax links notes together. Autocomplete suggestions while typing `[[`. Open linked note on click.
- **Backlinks panel**: Show a list of notes that link to the currently open note.
- **Graph view**: A canvas widget showing notes as nodes and wikilinks as edges, using a simple force‑directed layout. Nodes can be dragged.
- **Full‑text search**: Search bar that filters the file tree or shows a result list. Highlight occurrences.
- **Syntax highlighting** in fenced code blocks (using `syntect` / `iced_aw`).
- **Frontmatter support**: Optional YAML/TOML frontmatter parsed per note (title, tags, etc.).
- **Tags**: Extract tags from frontmatter or inline `#tag` syntax. Clickable to filter notes.
- **Dark theme** by default, with a modern, Obsidian‑inspired aesthetic. Resizable panels.

## Technology Stack
- **GUI**: `iced` 0.14 (latest) with `wgpu` backend, features: `wgpu`, `image`, `svg`, `canvas`, `tokio`
- **Markdown**: `frostmark` crate for parsing and rendering (produces `iced` widgets)
- **Syntax Highlighting**: `iced_aw` `SyntaxHighlighter` widget backed by `syntect`
- **File System**: `walkdir` for scanning vault, `notify` for live file watching (optional), `tokio::fs` for async I/O
- **Frontmatter**: `serde_yaml` + `serde`
- **Graph Rendering**: `iced::widget::canvas` with a custom force‑directed algorithm
- **Additional**: `chrono` for time display, `log` for debugging, `once_cell` for global state if needed, `regex` for wikilink parsing

## Architecture (Elm Pattern)
`Model` – full application state (current vault path, notes index, open note, editor content, search query, graph positions…)
`Message` – exhaustive enum of all possible events
`update` – pure function `fn(&mut Model, Message) -> Task<Message>` (commands for side‑effects like file I/O)
`view` – pure function `fn(&Model) -> Element<Message>`

Split into modules:
- `main.rs` – entry point, Iced `Application` trait impl
- `app.rs` – `Model`, `Message`, `update`, `view`
- `vault.rs` – scanning, file tree, note indexing (links, backlinks)
- `editor.rs` – editor widget composition (raw text + preview)
- `graph.rs` – graph view canvas, force layout
- `search.rs` – search logic
- `theme.rs` – custom Iced theme (dark)

## Detailed Implementation Steps

### 1. Project Setup
- Create a new Rust binary project `rustynotes`
- Add dependencies to `Cargo.toml`
- Set up `Application` trait: main window with title "RustyNotes", size 1200×800, default font
- Implement a custom `Theme` that provides a dark color palette.

### 2. Vault Selection
- On startup, if no vault is set, show a welcome screen with a "Open Vault" button.
- Use `rfd` (Rust File Dialog) or a native folder picker to select a folder. Because Iced doesn’t have built‑in dialogs, integrate `rfd` with `iced` using a `Task` (spawn async dialog, send `VaultSelected(Option<PathBuf>)`).
- Once selected, scan the folder recursively with `walkdir`, build the file tree, and parse all `.md` files to build the note index (title, path, wikilinks, frontmatter).

### 3. File Explorer Panel
- Left side: A `Scrollable` inside a `Column` showing a tree view.
- Use `Tree` custom widget (or build from nested rows with indentation and expand/collapse arrows, storing expanded folders in model).
- Click a note to open it; show context menu (right‑click) for rename/delete (need a popup menu custom widget or use `iced_aw` Menu).
- Create new note/folder via a button and an input dialog (modal overlay).

### 4. Note Editor
- Center area split into two panes (horizontally resizable with a draggable divider).
- **Left pane**: raw text `TextEditor` (from `iced::widget::text_editor`). Bind its content to `model.editor_content`. On edit, send `Message::EditorChanged(new_text)`.
- **Right pane**: Markdown preview. Use `frostmark` to parse the text into a vector of `Element`s. Re‑parse on every change (debounced or in `view`). For performance, store parsed blocks in model and update incrementally if possible; otherwise re‑parse per frame (acceptable for normal note lengths).
- Handle wikilinks: when rendering, any `[[link]]` is turned into an interactive `Button` with a clickable message `Message::OpenNote(link)`. Use a custom `frostmark` renderer if needed.
- Syntax highlighted code blocks: Use `iced_aw::SyntaxHighlighter` widget for code blocks, passing language and content from the Markdown AST.

### 5. Wikilinks & Backlinks
- **Parsing**: Use a regex to extract `[[target]]` from note content. Build a global map `note_links: HashMap<PathBuf, Vec<String>>` (note → outgoing links). Also compute `incoming_links: HashMap<String, Vec<PathBuf>>` for backlinks.
- Update these maps whenever a note is saved (or on each write, debounced). Re‑scan affected notes only.
- **Backlinks panel**: Right sidebar shows incoming links for the current note. List them as clickable items.
- **Autocomplete**: While editing, when the user types `[[`, show a popup list of existing note titles filtered by input. On selection, insert the selected link and close brackets.

### 6. Search
- A search bar at the top of the file explorer (or a global search panel).
- On query change, filter all note file names and/or full‑text content. Use a simple substring match, optionally `regex`.
- Display results in a drop‑down or a separate pane, with excerpts and highlighted matches. Clicking opens the note.

### 7. Graph View (Local Canvas)
- Toggle to a graph view (replace center area or show in a separate tab).
- Use `iced::widget::canvas` with a `Program` that draws nodes as circles and edges as lines.
- Maintain a `Vec<Node>` and `Vec<Edge>` derived from `note_links`. Each node has a position (initial random or layout). On first display, run a simple force‑directed simulation (already computed maybe off‑thread with `Task`).
- Support dragging nodes: on mouse down, identify hit node; on mouse move, update its position; on mouse up, stop dragging. This is done by handling canvas events (`canvas::Event`).
- Click a node to open the corresponding note.

### 8. Frontmatter & Tags
- Assume notes start with `---` frontmatter block. Parse with `serde_yaml`. Extract optional `title`, `tags` fields.
- Display tags as rounded chips below the note title. Clickable to filter the vault by tag (filter file tree and graph view to only matching notes).
- If no title in frontmatter, use the file name (without extension) as the note title.

## Code Organization Example
- `main.rs`:
```rust
mod app;
mod vault;
mod editor;
mod graph;
mod search;
mod theme;

use iced::{Application, Settings};

fn main() -> iced::Result {
    app::RustyNotes::run(Settings::default())
}
```

- `app.rs`:
```rust
pub struct RustyNotes {
    model: Model,
}

impl Application for RustyNotes {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = theme::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Task<Message>) { ... }
    fn title(&self) -> String { "RustyNotes".into() }
    fn update(&mut self, message: Message) -> Task<Message> { ... }
    fn view(&self) -> Element<Message> { ... }
}
```

- `Message` enum covers all interactions (see detailed spec).
- Use `Task::perform(async {...}, Message::X)` for file reads/writes, vault scanning, graph simulation.

## Important Considerations
- **File Watching** (nice‑to‑have): Use `notify` crate to watch the vault for external changes and update automatically. Send `Message::FileChange(..)` on event.
- **Performance**: For large vaults, keep scanning incremental and use background tasks. Avoid blocking the UI thread.
- **Undo/Redo**: not required for MVP, but architecture should allow adding it later.
- **Persistence of UI state**: Remember last opened note, window size, panel widths using a simple JSON config file in the vault (optional for MVP).

## Final Output
Generate the full, compilable Rust project. Include all necessary `Cargo.toml` dependencies and every source file with complete, well‑commented code. The application must build and run on Windows and Linux with a single `cargo run`. Use the Elm Architecture exclusively, with no unsafe code and full type safety. Implement every feature described above as a functional minimum‑viable‑product.
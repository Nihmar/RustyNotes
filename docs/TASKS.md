# RustyNotes — Task Breakdown

> **How to load tasks in a new session**: Ask opencode to read this file and recreate the task list using the `todowrite` tool.  
> Example prompt: _"Read docs/TASKS.md and load all tasks into the session. Start working on T1."_

Total tasks: **19** across **10 phases**.  
After Phase 1 (T1–T7), the app is **usable** as a basic notebook manager with markdown editing.

---

## Dependency Graph

```
T1 (deps + types)
 └─► T2 (notebook CRUD)
      ├─► T4 (NotebookSelector UI)
      │    └─► T7 (App layout)
      └─► T3 (note CRUD)
           ├─► T5 (FileTree)
           │    └─► T7 (App layout)
           ├─► T6 (CM6 edit mode)
           │    ├─► T8 (reading view)
           │    │    └─► T9 (live preview)
           │    └─► T7 (App layout)
           ├─► T10 (tabs)
           ├─► T11 (search BE) → T12 (search UI)
           ├─► T13 (tags BE) → T14 (tags UI + wikilinks)
           └─► T15 (file watcher) → T16 (autosave)

T17 (themes) — independent, can be done anytime after T1
T18 (plugins) — depends on T6/T7
T19 (mobile) — depends on T7 (app layout exists)
```

---

## Tasks

### T1 — Dependencies & Boilerplate [P1, High Priority]

**Depends on**: nothing (first task)

**Actions**:
1. Install NPM dependencies:
   ```
   yarn add codemirror @codemirror/state @codemirror/view @codemirror/commands @codemirror/lang-markdown @codemirror/language @codemirror/search @codemirror/lint @lezer/highlight marked
   ```
2. Add Rust dependencies to `src-tauri/Cargo.toml`:
   ```toml
   notify = { version = "6", features = ["macos_kqueue"] }
   walkdir = "2"
   regex = "1"
   chrono = { version = "0.4", features = ["serde"] }
   ignore = "0.4"
   ```
3. Create TypeScript types file: `src/lib/types.ts`
4. Create typed invoke wrappers: `src/lib/commands.ts`
5. Create empty Svelte stores in `src/lib/stores/`
6. Clean up `src/routes/+page.svelte` (remove greet demo, prepare for App mount)

**Files to create**:
- `src/lib/types.ts`
- `src/lib/commands.ts`
- `src/lib/stores/notebook.svelte.ts`
- `src/lib/stores/notes.svelte.ts`
- `src/lib/stores/tabs.svelte.ts`
- `src/lib/stores/search.svelte.ts`
- `src/lib/stores/ui.svelte.ts`
- `src/lib/stores/settings.svelte.ts`

**Files to modify**:
- `package.json`
- `src-tauri/Cargo.toml`
- `src/routes/+page.svelte`

---

### T2 — Notebook Rust Commands [P1, High Priority]

**Depends on**: T1

**Actions**:
1. Create `src-tauri/src/state.rs` — `AppState` managed state (active notebook path)
2. Create `src-tauri/src/notebook.rs`:
   - `create_notebook(name, path)` — creates directory + `.rustynotes/config.json`
   - `open_notebook(path)` — sets active notebook in AppState, starts watcher (placeholder)
   - `list_notebooks()` — reads recent notebooks from `~/.config/rustynotes/settings.json`
   - `close_notebook()` — clears active notebook, stops watcher
3. Register all notebook commands in `src-tauri/src/lib.rs`
4. Global settings path: `~/.config/rustynotes/settings.json`

**Files to create**:
- `src-tauri/src/state.rs`
- `src-tauri/src/notebook.rs`

**Files to modify**:
- `src-tauri/src/lib.rs`

---

### T3 — Note Rust Commands [P1, High Priority]

**Depends on**: T1

**Actions**:
1. Create `src-tauri/src/notes.rs`:
   - `list_notes(dir?)` — recursive walkdir for `.md` files, return `Vec<NoteMeta>`
   - `read_note(path)` — read file content to string
   - `write_note(path, content)` — atomic write to file
   - `create_note(dir, title)` — create `.md` file with optional YAML frontmatter
   - `delete_note(path)` — delete file (move to trash on supported platforms)
   - `rename_note(path, new_name)` — rename file
   - `move_note(from, to)` — move file between directories
2. Define `NoteMeta` struct: `title`, `path`, `modified`, `size`, `tags`
3. Register all note commands in `src-tauri/src/lib.rs`

**Files to create**:
- `src-tauri/src/notes.rs`

**Files to modify**:
- `src-tauri/src/lib.rs`

---

### T4 — NotebookSelector UI [P1, High Priority]

**Depends on**: T2

**Actions**:
1. Create `NotebookSelector.svelte`:
   - Dropdown/list of recent notebooks (loaded on mount)
   - "Open folder" button → native dialog → calls `open_notebook`
   - "New notebook" button → name input → calls `create_notebook`
2. Create `WelcomeScreen.svelte` — shown when no notebook is open, prompts to open/create
3. Implement `notebook.svelte.ts` store (active notebook, list of recent notebooks, loading state)
4. Implement typed invoke wrappers in `commands.ts` for notebook commands

**Files to create**:
- `src/lib/components/sidebar/NotebookSelector.svelte`
- `src/lib/components/WelcomeScreen.svelte`
- `src/lib/stores/notebook.svelte.ts`

**Files to modify**:
- `src/lib/commands.ts`
- `src/lib/types.ts`

---

### T5 — FileTree [P1, High Priority]

**Depends on**: T3, T4

**Actions**:
1. Create `FileTree.svelte`:
   - Recursive tree from `list_notes` result
   - Folders expandable/collapsible, files clickable to open
   - Sort: folders first (alphabetical), then files (alphabetical)
   - Highlight currently active note
   - Right-click context menu: rename, delete, new note
2. Implement `notes.svelte.ts` store (tree data, active note path, notes map)

**Files to create**:
- `src/lib/components/sidebar/FileTree.svelte`
- `src/lib/stores/notes.svelte.ts`

**Files to modify**:
- `src/lib/commands.ts`
- `src/lib/types.ts`

---

### T6 — CodeMirror 6 Edit Mode [P1, High Priority]

**Depends on**: T1

**Actions**:
1. Create `src/lib/editor-engine/setup.ts` — CM6 extensions configuration:
   - `@codemirror/lang-markdown` for markdown syntax
   - Line numbers, history (undo/redo), bracket matching
   - Theme extension (placeholder, real themes come in T17)
2. Create `src/lib/editor-engine/wikilinks.ts`:
   - Custom syntax extension for `[[wiki-link]]` highlighting
   - `Decoration.mark` to style wikilinks differently
3. Create `Editor.svelte`:
   - Mounts CodeMirror 6 editor
   - Exposes `content` bindable prop
   - Auto-resize to fill container
   - Emits change events

**Files to create**:
- `src/lib/editor-engine/setup.ts`
- `src/lib/editor-engine/wikilinks.ts`
- `src/lib/components/editor/Editor.svelte`

**Files to modify**:
- none (new files only)

---

### T7 — App Layout & Full Wiring [P1, High Priority]

**Depends on**: T4, T5, T6

**Actions**:
1. Create `App.svelte`:
   - Split pane: resizable sidebar (left) + editor area (right)
   - Conditional: show WelcomeScreen if no notebook open, else show Sidebar + EditorPane
2. Create `Sidebar.svelte`:
   - Contains NotebookSelector + FileTree + tabbed panels for Search/Tags (future)
3. Create `EditorPane.svelte`:
   - Contains Editor + (future: TabBar + ModeSwitcher)
   - Loads note content on mount, saves on change
4. Update `+page.svelte` to mount `<App/>`

**Files to create**:
- `src/lib/App.svelte`
- `src/lib/components/sidebar/Sidebar.svelte`
- `src/lib/components/editor/EditorPane.svelte`

**Files to modify**:
- `src/routes/+page.svelte`

---

### T8 — Reading View [P2, High Priority]

**Depends on**: T6, T7

**Actions**:
1. Create `src/lib/editor-engine/reading-view.ts`:
   - Parse markdown → HTML using `marked`
   - Custom renderer extension for `[[wiki-links]]`
   - Code blocks get CSS classes for syntax highlighting
2. Update `Editor.svelte`:
   - When `mode === 'reading'`, hide CM6, show rendered HTML
   - Preserve scroll position on mode switch
3. Create `ModeSwitcher.svelte`:
   - 3 buttons: Edit | Live Preview | Reading
   - Highlight active mode
   - Keyboard shortcut handler (Ctrl+E cycles)
4. Implement `ui.svelte.ts` store (editor mode, sidebar visibility)

**Files to create**:
- `src/lib/editor-engine/reading-view.ts`
- `src/lib/components/editor/ModeSwitcher.svelte`
- `src/lib/stores/ui.svelte.ts`

**Files to modify**:
- `src/lib/components/editor/Editor.svelte`

---

### T9 — Live Preview Mode [P2, High Priority]

**Depends on**: T8

**Actions**:
1. Create `src/lib/editor-engine/live-preview.ts`:
   - CM6 ViewPlugin using the Decoration API
   - Per-line decoration: hide markdown syntax chars, apply formatting styles
   - Cursor line detection: line containing cursor stays in raw edit mode
   - Handle headings (# → larger font, colored), bold (** → font-weight), italic (* → font-style), lists (- → bullet glyph), code (` → monospace bg), blockquotes (> → left border), horizontal rules (--- → styled line)
   - Wiki-links rendered as clickable styled pills
2. Integrate with `setup.ts`: add the ViewPlugin when mode is `live-preview`
3. Update `Editor.svelte`: toggle the extension when mode changes

**Files to create**:
- `src/lib/editor-engine/live-preview.ts`

**Files to modify**:
- `src/lib/editor-engine/setup.ts`
- `src/lib/components/editor/Editor.svelte`

---

### T10 — Multiple Tabs [P3, Medium Priority]

**Depends on**: T5, T6

**Actions**:
1. Create `TabBar.svelte`:
   - Horizontal tab list with note titles
   - Close button per tab, click to switch active
   - Dirty indicator (dot for unsaved changes)
   - Drag-and-drop reorder (future enhancement)
2. Create `Tab.svelte` (optional, can be inline in TabBar)
3. Implement `tabs.svelte.ts` store:
   - Open tabs array: `{ path, title, isDirty, mode }`
   - Active tab index
   - Open/close/switch operations
4. Update `EditorPane.svelte`:
   - Render TabBar + active tab's Editor
   - Save/restore open tabs per notebook

**Files to create**:
- `src/lib/components/editor/TabBar.svelte`
- `src/lib/stores/tabs.svelte.ts`

**Files to modify**:
- `src/lib/components/editor/EditorPane.svelte`

---

### T11 — Search Backend [P4, Medium Priority]

**Depends on**: T3

**Actions**:
1. Create `src-tauri/src/search.rs`:
   - `search_notes(query)` — walkdir all `.md` files, read content, regex match
   - Return results sorted by relevance (exact match > partial, title match > body match)
   - Each result: note path, match snippet (surrounding context), line number
   - Respect `.rustynotes/ignore` or `.gitignore` for excluded directories
   - Case-insensitive search
2. Register command in `lib.rs`

**Files to create**:
- `src-tauri/src/search.rs`

**Files to modify**:
- `src-tauri/src/lib.rs`

---

### T12 — Search UI [P4, Medium Priority]

**Depends on**: T11, T7

**Actions**:
1. Create `SearchPanel.svelte`:
   - Search input with debounce (300ms)
   - Results list: note title + match snippet + line number
   - Click result → open note in editor, scroll to match line (best effort)
   - Escape to clear
2. Implement `search.svelte.ts` store

**Files to create**:
- `src/lib/components/sidebar/SearchPanel.svelte`
- `src/lib/stores/search.svelte.ts`

**Files to modify**:
- `src/lib/components/sidebar/Sidebar.svelte` (add SearchPanel to sidebar tabs)

---

### T13 — Tags Backend [P5, Medium Priority]

**Depends on**: T3

**Actions**:
1. Create `src-tauri/src/tags.rs`:
   - `get_tags()` — walkdir all `.md`, regex `#[\w/-]+`, deduplicate
   - Return `Vec<TagInfo>`: tag name + count of notes containing it
   - Exclude tags inside code blocks
2. Register command in `lib.rs`

**Files to create**:
- `src-tauri/src/tags.rs`

**Files to modify**:
- `src-tauri/src/lib.rs`

---

### T14 — Tags UI & Wikilinks [P5, Medium Priority]

**Depends on**: T13, T7, T6

**Actions**:
1. Create `TagBrowser.svelte`:
   - Tag list with note counts
   - Click tag → filter file tree to show matching notes
2. Extend `wikilinks.ts`:
   - Click handler on `[[link]]` → navigate to that note
   - Create note if wikilink target doesn't exist (prompt user: "Create X.md?")
3. Add TagBrowser to sidebar tabs

**Files to create**:
- `src/lib/components/sidebar/TagBrowser.svelte`

**Files to modify**:
- `src/lib/editor-engine/wikilinks.ts`
- `src/lib/components/sidebar/Sidebar.svelte`

---

### T15 — File Watcher [P6, Medium Priority]

**Depends on**: T3

**Actions**:
1. Create `src-tauri/src/fs_watcher.rs`:
   - Use `notify` crate to watch notebook directory recursively
   - Debounce events (100ms window, batch rapid changes)
   - Emit Tauri events: `note-created`, `note-modified`, `note-deleted`, `note-renamed`
   - Each event carries the file path + metadata
2. Integrate with `notebook.rs`: start watcher on open, stop on close
3. Create `src/lib/events.ts`:
   - Listen for Tauri events from file watcher
   - Dispatch to relevant stores (update file tree, reload open note if modified externally)

**Files to create**:
- `src-tauri/src/fs_watcher.rs`
- `src/lib/events.ts`

**Files to modify**:
- `src-tauri/src/notebook.rs`
- `src-tauri/src/lib.rs`

---

### T16 — Autosave & Conflict Detection [P6, Medium Priority]

**Depends on**: T15, T6

**Actions**:
1. In `Editor.svelte` (or `EditorPane.svelte`):
   - Debounced save on content change (500ms default, configurable)
   - Save indicator in toolbar: "Saving..." / "Saved" / "Unsaved changes"
2. Conflict detection:
   - When a `note-modified` event arrives and the note is open and dirty:
     - Show notification/prompt: "Note modified externally. Reload / Keep your changes / Diff"
3. Basic dirty tracking in `tabs` or `notes` store

**Files to modify**:
- `src/lib/components/editor/Editor.svelte`
- `src/lib/components/editor/EditorPane.svelte`
- `src/lib/stores/notes.svelte.ts` or `src/lib/stores/tabs.svelte.ts`

---

### T17 — Dark/Light Theme [P7, Medium Priority]

**Depends on**: T1 (independent, but needs app to test)

**Actions**:
1. Define CSS custom properties for colors:
   - Light theme variables (backgrounds, text, accents, borders)
   - Dark theme variables
   - Apply via `:root[data-theme="dark"]` and `:root[data-theme="light"]`
2. Create CM6 themes:
   - `src/lib/editor-engine/themes/dark.ts`
   - `src/lib/editor-engine/themes/light.ts`
3. Theme toggle button in sidebar/toolbar
4. Persist choice in `settings.svelte.ts`
5. Auto-detect `prefers-color-scheme` on first launch
6. Ensure rendered markdown (reading view) respects theme

**Files to create**:
- `src/lib/editor-engine/themes/dark.ts`
- `src/lib/editor-engine/themes/light.ts`
- CSS file or inline in `app.html`

**Files to modify**:
- `src/lib/stores/settings.svelte.ts`
- `src/lib/stores/ui.svelte.ts`
- `src/app.html`

---

### T18 — Plugin Hooks [P8, Low Priority]

**Depends on**: T6, T7

**Actions**:
1. Create `src/lib/plugins/types.ts`:
   ```ts
   interface Plugin {
     name: string;
     onNoteOpen?: (content: string) => string;
     onNoteSave?: (content: string) => string;
     onEditorInit?: (view: EditorView) => void;
     markdownExtensions?: () => Extension[];
   }
   ```
2. Create `src/lib/plugins/loader.ts`:
   - Skeleton loader that scans `.rustynotes/plugins/` for `.js` files
   - Dynamic `import()` to load plugins
   - Call hooks at defined points
3. Call `onNoteOpen`/`onNoteSave` in EditorPane, `onEditorInit` in Editor, `markdownExtensions` in setup.ts

**Files to create**:
- `src/lib/plugins/types.ts`
- `src/lib/plugins/loader.ts`

**Files to modify**:
- `src/lib/components/editor/EditorPane.svelte`
- `src/lib/components/editor/Editor.svelte`
- `src/lib/editor-engine/setup.ts`

---

### T19 — Mobile Adaptations [P10, Low Priority]

**Depends on**: T7 (app layout exists)

**Actions**:
1. Sidebar → collapsible drawer:
   - Hamburger button to toggle
   - Overlay on mobile, persistent on desktop
   - Media query breakpoint at 768px
2. Touch-friendly adjustments:
   - Larger tap targets (min 44px) in FileTree
   - ModeSwitcher as a segmented control (not tiny buttons)
3. Mobile capability configuration in `tauri.conf.json`
4. Test with `cargo tauri android dev`

**Files to modify**:
- `src/lib/components/sidebar/Sidebar.svelte`
- `src/lib/components/sidebar/FileTree.svelte`
- `src/lib/components/editor/ModeSwitcher.svelte`
- `src-tauri/tauri.conf.json`

---

## How to Resume in a New Session

Ask opencode with either of these prompts:

```
Read docs/TASKS.md and load all tasks using todowrite.
Start from T1 and proceed sequentially.
```

```
Read docs/ARCHITECTURE.md and docs/TASKS.md.
The current task is T6. Load all remaining tasks starting from T6.
```

The numbered structure (T1–T19) lets you resume from any checkpoint by asking to
"start from TX".

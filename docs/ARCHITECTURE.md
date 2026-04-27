# RustyNotes — Architecture

An Obsidian-like note-taking app built with Tauri 2.0 + SvelteKit + TypeScript.

## Target Platforms

- Windows (WebView2)
- Linux (WebKitGTK)
- macOS (WKWebView)
- Android (Tauri mobile)
- iOS (Tauri mobile)

## Terminology

| Obsidian | RustyNotes |
|----------|-------------|
| Vault    | **Notebook** |
| Note     | Note |

A **Notebook** is a folder on disk containing `.md` files and subfolders. The app manages multiple notebooks; one active at a time.

---

## Technology Stack

### Frontend
- **Framework**: Svelte 5 + SvelteKit (SPA mode via `@sveltejs/adapter-static`)
- **Editor**: CodeMirror 6 (`codemirror`, `@codemirror/*`, `@lezer/*`)
- **Markdown rendering**: `pulldown-cmark` (Rust crate) — renders markdown to HTML via a Tauri command
- **Math rendering**: KaTeX (`katex`) — frontend post-processing after Rust renders HTML
- **Bundler**: Vite 6
- **Language**: TypeScript

### Backend (Rust via Tauri)
- **Framework**: Tauri 2.0
- **File watching**: `notify` crate
- **File traversal**: `walkdir`
- **Text search**: `regex` + `walkdir`
- **Markdown rendering**: `pulldown-cmark`
- **Date handling**: `chrono`
- **Gitignore support**: `ignore` crate (for search exclusions)
- **Serialization**: `serde` + `serde_json` (already in project)

---

## Crate Structure (`src-tauri/src/`)

```
src-tauri/src/
├── main.rs              # Entry point
├── lib.rs               # Tauri builder, command registration, AppState
├── notebook.rs          # Notebook CRUD + settings persistence
├── notes.rs             # Note CRUD (list, read, write, create, delete, rename, move)
├── render.rs            # Markdown → HTML rendering (pulldown-cmark) with wiki-link/math/image pre-processing
├── search.rs            # Full-text search across all notes
├── tags.rs              # Tag extraction (#tag regex)
├── fs_watcher.rs        # File system watcher (notify crate)
└── state.rs             # Tauri managed state (active notebook path, watcher handle)
```

### Rust Dependencies (in `src-tauri/Cargo.toml`)

```toml
notify = { version = "6", features = ["macos_kqueue"] }
walkdir = "2"
regex = "1"
pulldown-cmark = "0.13"
chrono = { version = "0.4", features = ["serde"] }
ignore = "0.4"
```

---

## Tauri Commands

### Notebook Management
| Command | Args | Returns |
|---------|------|---------|
| `create_notebook` | `name: String, path: String` | `Result<Notebook>` |
| `open_notebook` | `path: String` | `Result<Notebook>` |
| `list_notebooks` | — | `Result<Vec<NotebookInfo>>` |
| `close_notebook` | — | `Result<()>` |

### Note Operations
| Command | Args | Returns |
|---------|------|---------|
| `list_notes` | `dir: Option<String>` | `Result<Vec<NoteMeta>>` |
| `read_note` | `path: String` | `Result<String>` |
| `write_note` | `path: String, content: String` | `Result<()>` |
| `create_note` | `dir: String, title: String` | `Result<NoteMeta>` |
| `delete_note` | `path: String` | `Result<()>` |
| `rename_note` | `path: String, new_name: String` | `Result<()>` |
| `move_note` | `from: String, to: String` | `Result<()>` |

### Search & Tags & Rendering
| Command | Args | Returns |
|---------|------|---------|
| `search_notes` | `query: String` | `Result<Vec<SearchResult>>` |
| `get_tags` | — | `Result<Vec<TagInfo>>` |
| `render_markdown` | `content: String` | `Result<String>` |

---

## Notebook Config File

Each notebook gets a `.rustynotes/` directory at its root:

```
MyNotebook/
├── .rustynotes/
│   └── config.json     # { "name": "My Notebook", "created": "..." }
├── note1.md
├── note2.md
└── subfolder/
    └── note3.md
```

A global settings file at `~/.config/rustynotes/settings.json` stores:
- List of recent notebooks (name + path)
- User preferences (theme, default editor mode, etc.)

---

## Frontend Structure (`src/`)

```
src/
├── app.html
├── routes/
│   └── +page.svelte              # Entry: mounts <App/>
│
├── lib/
│   ├── App.svelte                 # Root layout (sidebar + editor area)
│   ├── commands.ts                # Typed invoke() wrappers + buildFileTree()
│   ├── types.ts                   # Shared TypeScript types
│   ├── events.ts                  # Tauri event listeners (file watcher)
│   │
│   ├── components/
│   │   ├── sidebar/
│   │   │   ├── Sidebar.svelte              # Left sidebar container (Files/Search/Tags tabs)
│   │   │   ├── NotebookSelector.svelte     # Open/switch/create notebooks
│   │   │   ├── FileTree.svelte             # Tree view of notes
│   │   │   ├── SearchPanel.svelte          # Full-text search UI
│   │   │   └── TagBrowser.svelte           # Tag list + filter
│   │   │
│   │   ├── editor/
│   │   │   ├── EditorPane.svelte           # Holds TabBar + active Editor
│   │   │   ├── TabBar.svelte               # Open tabs
│   │   │   ├── Editor.svelte               # CodeMirror 6 wrapper (edit mode)
│   │   │   ├── ReadingView.svelte           # Rust-rendered HTML reading view
│   │   │   └── ModeSwitcher.svelte         # Edit / Reading toggle
│   │   │
│   │   └── WelcomeScreen.svelte            # Shown when no notebook is open
│   │
│   ├── editor-engine/
│   │   ├── setup.ts                # CM6 extensions for edit mode
│   │   ├── wikilinks.ts            # [[wiki-link]] syntax highlighting in CM6 (decoration only)
│   │   └── themes/
│   │       ├── dark.ts             # Dark theme for CodeMirror
│   │       └── light.ts            # Light theme for CodeMirror
│   │
│   ├── stores/
│   │   ├── notebook.svelte.ts      # Active notebook state
│   │   ├── notes.svelte.ts         # Notes list, active note, dirty tracking
│   │   ├── tabs.svelte.ts          # Open tabs state + per-mode scroll preservation
│   │   ├── search.svelte.ts        # Search query + results
│   │   ├── ui.svelte.ts            # Theme, sidebar visibility, editor mode
│   │   └── settings.svelte.ts      # Persisted user preferences
│   │
│   └── plugins/
│       ├── types.ts                # Plugin interfaces
│       └── loader.ts               # Plugin loader skeleton (scan/load/apply hooks)
```

---

## Editor — Two Modes

### 1. Edit Mode
- Raw markdown in CodeMirror 6
- Full syntax highlighting via `@codemirror/lang-markdown`
- Wiki-link `[[ ]]` highlighting via custom syntax extension
- Standard editor features: line numbers, bracket matching, undo/redo, search

### 2. Reading View
- CM6 editor hidden, replaced by rendered HTML
- Markdown → HTML rendering done in Rust via `pulldown-cmark` (`render_markdown` Tauri command)
- Rust pre-processes wiki-links (`[[...]]` → `<a class="wikilink">`), math (`$...$`, `$$...$$` → `<span>/<div>`), and image embeds (`![[...]]` → `<img>`)
- KaTeX math rendered client-side as post-processing step after HTML insertion
- Wiki-link click navigation handled client-side via `wikilinks.ts`
- Scroll position preserved per-tab per-mode when switching modes

### Mode Switching
- 2-button toggle in the toolbar (Edit | Reading)
- Keyboard shortcut: `Ctrl+E` / `Cmd+E` cycles modes (matches Obsidian)
- Mode is per-tab (different notes can be in different modes)
- Default mode configurable in settings

---

## Data Flow

```
┌──────────────────────────────────────────────────────────┐
│  Svelte Frontend                                          │
│  ┌──────────┐  ┌──────────┐  ┌────────────────────────┐ │
│  │ Sidebar  │  │ EditorPane│  │ CodeMirror 6           │ │
│  │(FileTree,│  │(TabBar,  │  │ (Edit / ReadingView)  │ │
│  │ Search,  │  │ ModeSw,  │  │                        │ │
│  │ Tags)    │  │ Editor)  │  │                        │ │
│  └────┬─────┘  └────┬─────┘  └───────────┬────────────┘ │
│       │              │                    │               │
│       └──────┬───────┘                    │               │
│              │ Tauri invoke()             │               │
└──────────────┼────────────────────────────┼───────────────┘
               │                            │
┌──────────────┼────────────────────────────┼───────────────┐
│  Rust Backend (src-tauri/)                │               │
│  ┌───────────┴──────────┐  ┌──────────────┴───────────┐  │
│  │ Tauri Commands        │  │ File Watcher (notify)    │  │
│  │ - notebook CRUD       │  │ → emits events to FE     │  │
│  │ - notes CRUD          │  │                          │  │
│  │ - search              │  │                          │  │
│  │ - tags                │  │                          │  │
│  └───────────┬───────────┘  └──────────────────────────┘  │
│              │                                            │
│  ┌───────────┴───────────┐                               │
│  │ File System (disk)    │                               │
│  │ ~/Notebooks/mynotes/  │                               │
│  │   ├── note1.md        │                               │
│  │   ├── note2.md        │                               │
│  │   └── subfolder/      │                               │
│  │       └── note3.md    │                               │
│  └───────────────────────┘                               │
└──────────────────────────────────────────────────────────┘
```

---

## Plugin Architecture (Skeleton Implemented)

Plugin interfaces and loader skeleton are in `src/lib/plugins/`:

| Hook Point | Signature |
|------------|-----------|
| `onNoteOpen` | `(content: string) => string` |
| `onNoteSave` | `(content: string) => string` |
| `onEditorInit` | `(view: EditorView) => void` |
| `markdownExtensions` | `() => Extension[]` |

- `types.ts` — Plugin interface with optional hook methods
- `loader.ts` — In-memory plugin registry + `loadPlugins()` (placeholder, future: scan `.rustynotes/plugins/` for `.js` files)
- Hooks (`applyOnNoteOpen`, `applyOnNoteSave`) are wired but no plugins are loaded at runtime yet

---

## Cross-Platform Notes

| Platform | Runtime | Considerations |
|----------|---------|----------------|
| Windows  | WebView2 | Native support |
| Linux    | WebKitGTK | Tested locally |
| macOS    | WKWebView | Build on CI (no local Mac) |
| Android  | Tauri mobile | File picker for notebook folder, responsive sidebar (drawer), touch targets, on-screen keyboard |
| iOS      | Tauri mobile | Same mobile considerations. Build on CI (no local device) |

Mobile adaptations (in progress):
- Sidebar has hamburger toggle + responsive `@media (max-width: 768px)` fixed overlay (basic)
- Still missing: larger touch targets, touch-friendly mode switcher, mobile capability config
- The sidebar already includes collapsible hamburger logic for all viewport sizes

---

## Settings Persistence

Global settings at `~/.config/rustynotes/settings.json`:
```json
{
  "recent_notebooks": [
    { "name": "My Notes", "path": "/home/user/MyNotes" }
  ],
  "theme": "dark",
  "default_editor_mode": "live-preview",
  "sidebar_visible": true,
  "font_size": 16,
  "autosave_interval_ms": 500
}
```

Notebook-local settings at `<notebook>/.rustynotes/config.json`:
```json
{
  "name": "My Notebook",
  "created": "2026-04-24T12:00:00Z"
}
```

# RustyNotes

An [Obsidian](https://obsidian.md)-like note-taking app built with **Tauri 2.0** + **SvelteKit** + **TypeScript**.
Runs on Windows, Linux, macOS, Android, and iOS.

## Features

| Status | Feature |
|--------|---------|
| ✅ | Notebook management (create, open, switch between note folders) |
| ✅ | Markdown editing with syntax highlighting (CodeMirror 6) |
| ✅ | Three editor modes: Edit, Live Preview, Reading — all inline, no split panes |
| ✅ | File tree sidebar with recursive note browsing |
| ✅ | Full-text search across all notes |
| ✅ | `#tags` extraction and tag browser |
| ✅ | `[[wiki-link]]` navigation between notes |
| ✅ | Multiple tabs |
| ✅ | Dark/light theme |
| ✅ | Autosave (debounced write) + conflict detection for external changes |
| ✅ | Plugin system (types + loader skeleton) |
| ⚠️ | Mobile adaptations (basic responsive sidebar, rest pending) |

## Tech Stack

| Layer | Technology |
|-------|-----------|
| **Desktop/Mobile** | [Tauri 2.0](https://tauri.app) |
| **Frontend** | [Svelte 5](https://svelte.dev) + [SvelteKit](https://kit.svelte.dev) (SPA mode) |
| **Editor** | [CodeMirror 6](https://codemirror.net) |
| **Markdown rendering** | [marked](https://marked.js.org) |
| **Backend** | Rust (commands for FS, search, tags, file watching) |
| **Bundler** | [Vite 6](https://vite.dev) |
| **Language** | TypeScript |

## Platform Support

| Platform | Status |
|----------|--------|
| Windows  | ✅ via WebView2 |
| Linux    | ✅ via WebKitGTK (primary development platform) |
| macOS    | ✅ via WKWebView (build-only, not locally tested) |
| Android  | ✅ via Tauri mobile |
| iOS      | ✅ via Tauri mobile (build-only, not locally tested) |

## Development

### Prerequisites

- [Rust](https://rustup.rs) (stable)
- [Node.js](https://nodejs.org) >= 18
- [Yarn](https://yarnpkg.com) (v1)
- Tauri system dependencies:
  - **Linux**: `sudo apt install libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf`
  - **Windows**: WebView2 (bundled with Windows 10+)
  - **macOS**: Xcode Command Line Tools

### Setup

```bash
git clone <repo-url>
cd RustyNotes
yarn install         # Install frontend dependencies
```

### Commands

```bash
yarn tauri dev       # Start dev server with hot reload
yarn tauri build     # Production build
yarn check           # Type-check the frontend
```

### IDE Setup

Recommended VS Code extensions (see `.vscode/extensions.json`):

- [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode)
- [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Development with opencode

Most development tasks (T1–T18 out of 19) are implemented. Remaining work:

- **T19** — Mobile adaptations (touch targets, drawer overlay, mobile capability config)

See [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) and [`docs/TASKS.md`](docs/TASKS.md) for details.

## Project Structure

```
RustyNotes/
├── src/                      # SvelteKit frontend
│   ├── lib/
│   │   ├── components/       # UI components (sidebar, editor, etc.)
│   │   ├── editor-engine/    # CodeMirror 6 setup, modes, themes, live preview, math rendering, lazy reading view
│   │   ├── stores/           # Svelte 5 runes stores (notebook, notes, tabs, search, UI, settings)
│   │   ├── plugins/          # Plugin interfaces + loader
│   │   ├── commands.ts       # Typed Tauri invoke() wrappers + buildFileTree()
│   │   ├── events.ts         # Tauri event listeners (file watcher)
│   │   └── types.ts          # TypeScript types
│   └── routes/               # SvelteKit routes (SPA: single page)
├── src-tauri/                # Tauri / Rust backend
│   └── src/
│       ├── main.rs           # Entry point
│       ├── lib.rs            # Tauri builder + command registration
│       ├── notebook.rs       # Notebook CRUD
│       ├── notes.rs          # Note CRUD
│       ├── search.rs         # Full-text search
│       ├── tags.rs           # Tag extraction
│       ├── fs_watcher.rs     # File system watcher
│       └── state.rs          # Tauri managed state
├── docs/
│   ├── ARCHITECTURE.md       # Full architecture plan
│   └── TASKS.md              # Task breakdown + dependencies
└── static/                   # Static assets
```

## License

MIT

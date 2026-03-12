# RustyNotes Implementation Plan

This plan breaks down the roadmap into logical phases and implementation steps.

## Phase 1: Core Infrastructure

### 1.1 Project Setup
- [ ] Set up Iced framework with basic window
- [ ] Configure logging system
- [ ] Define core data structures (Note, Folder, Settings)
- [ ] Set up dependency management ( iced, pulldown-cmark, etc.)

### 1.2 Folder Management
- [ ] Implement folder selection dialog (native file picker)
- [ ] Scan and index markdown files in selected folder
- [ ] Create sidebar with folder tree view
- [ ] Store known folders in `.cache/RustyNotes`
- [ ] Add/remove folders from known folders list

## Phase 2: Note Editor

### 2.1 Basic Editor
- [ ] Create note list view in sidebar
- [ ] Implement text editor widget for note editing
- [ ] Add file watcher for external changes
- [ ] Implement note save/load operations

### 2.2 Markdown Support
- [ ] Integrate markdown parser (pulldown-cmark)
- [ ] Implement Edit/Live-Preview/View modes
- [ ] Add syntax highlighting for markdown
- [ ] Support wiki-links (`[[note-name]]`)
- [ ] Handle internal link navigation

## Phase 3: Performance & UX

### 3.1 Performance Optimization
- [ ] Implement lazy loading for large notes
- [ ] Cache rendered markdown
- [ ] Optimize file indexing
- [ ] Background file operations

### 3.2 Settings & Customization
- [ ] Create per-folder settings (`.rusty/config`)
- [ ] Implement font selection UI
- [ ] Build theme system (`.rusty/themes/`)
- [ ] Add OS theme detection (dark/light mode)

## Phase 4: Polish

### 4.1 Polish & Features
- [ ] Keyboard shortcuts
- [ ] Search functionality
- [ ] Recent files list
- [ ] Undo/redo support

## Technical Notes

### Suggested Dependencies
- `iced` - GUI framework
- `pulldown-cmark` - Markdown parsing
- `syntect` - Syntax highlighting
- `notify` - File watching
- `serde` / `serde_json` - Serialization
- `directories` - Platform-specific paths
- `thiserror` - Error handling

### Architecture Layers
```
UI Layer (Iced)
    ↓
Service Layer (business logic)
    ↓
Repository Layer (file I/O)
    ↓
Platform Layer (OS integration)
```

### Milestone Order Rationale
1. **Phase 1** - Foundation: Need folder management before editing notes
2. **Phase 2** - Core Feature: Editor is the main user-facing feature
3. **Phase 3** - Performance: Optimize after core works
4. **Phase 4** - Polish: Final touches after everything functional

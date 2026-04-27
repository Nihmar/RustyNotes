# Plan: Move Reading View Rendering to Rust (pulldown-cmark)

## Summary

| Aspect | Before | After |
|---|---|---|
| Reading view renderer | `marked` (JS) + lazy IntersectionObserver | `pulldown-cmark` (Rust) — single IPC call |
| Editor modes | Edit / Live Preview / Reading | Edit / Reading |
| KaTeX | Pre-rendered server-side by `marked` extension | Post-processed client-side after Rust HTML arrives |
| Wiki-links | Custom `marked` tokenizer | Rust regex pre-processor → `<a class="wikilink">` |
| Image embeds | Custom `marked` tokenizer | Rust regex pre-processor → `<img>` with `vault://` |

## Step 1: Rust backend — create `render.rs`

**`src-tauri/Cargo.toml`**: add `pulldown-cmark = "0.13"`

**`src-tauri/src/render.rs`** (new):
- `preprocess_images()` — regex `![[...]]` → `<img class="image-embed">`
- `preprocess_wikilinks()` — regex `[[...]]` → `<a class="wikilink" href="note://...">`
- `preprocess_math()` — regex `$$...$$` → `<div class="math-block">`, `$...$` → `<span class="math-inline">`
- `render_markdown(content: &str) -> String` — chains preprocessing → `Parser::new_ext()` (tables, footnotes, strikethrough, tasklists enabled) → `html::push_html()`

**`src-tauri/src/lib.rs`**:
- Add `mod render;`
- Add `#[tauri::command] fn render_markdown(content: String) -> Result<String, String>`
- Register in `generate_handler![]`

## Step 2: Frontend — add `renderMarkdown` command

**`src/lib/commands.ts`**: add `renderMarkdown(content: string): Promise<string>`

## Step 3: Frontend — reading view rewrite

**Delete** (4 files):
- `src/lib/editor-engine/reading-view.ts` — entire module obsoleted by Rust renderer
- `src/lib/editor-engine/lazy-reading-view.ts` — entire module obsoleted by single IPC call
- `src/lib/editor-engine/math-live.ts` — only used by live preview mode
- `src/lib/editor-engine/image-embed.ts` — only used by live preview mode

**Create** `src/lib/components/editor/ReadingView.svelte`:
- Props: `content: string`
- `$effect` on `content`: calls `renderMarkdown(content)` via Tauri, stores result in `htmlContent`
- Template: `<div class="reading-view" bind:this={container}>{@html htmlContent}</div>`
- Second `$effect` on `htmlContent` change: queries `.math-block` / `.math-inline` in DOM, calls `katex.render()`. Uses `requestAnimationFrame` to ensure DOM is ready.
- Click handler (via `onMount`): catches `a.wikilink` clicks, calls `navigateWikilink()` — same logic as current `LazyReadingView`

**Delete** `src/lib/components/editor/LazyReadingView.svelte` (replaced by `ReadingView.svelte`)

## Step 4: Frontend — Editor.svelte surgery

**Remove:**
- `setMouseSelecting` import from `codemirror-live-markdown`
- `createLivePreviewExtensions` import from `setup.ts`
- `Compartment` import (no longer needed)
- `livePreviewExtensionsCompartment` field
- `onMouseDown()` / `onMouseUp()` methods + their event listeners
- `isLivePreview` logic in `onMount`
- `setMode()` export method
- Mode transition `$effect`: simplify — no LP compartment reconfig, just CM hidden/visible + scroll restore
- `lp-active` class and all `lp-active` CSS
- `.cm-image-embed-*` styles
- `.cm-formatting-block-*` styles

**Update:**
- Replace `LazyReadingView` import → `ReadingView` import
- Reading mode scroll tracking: keep

**Keep:**
- `.reading-view` CSS block
- `.cm-hidden` toggling
- `.cm-wikilink-*` styles
- Wikilink click handler in CM6

## Step 5: Frontend — setup.ts cleanup

**Remove:**
- All imports from `codemirror-live-markdown`
- `mouseSelectingField` from `createEditorExtensions()` return array
- `mathExtensions` import
- `imageEmbedExtensions` import
- `listMarkPlugin` view plugin class
- `createLivePreviewExtensions()` function entirely
- `visibleListMark` decoration

**Keep:**
- `createEditorExtensions()` — still the foundation for edit mode
- `createEditorState()` — unchanged
- `wikilinks()` import (still needed in edit mode)

## Step 6: Frontend — type and store updates

**`src/lib/types.ts`**: `EditorMode = 'edit' | 'reading'`

**`src/lib/stores/ui.svelte.ts`**: `cycleEditorMode()` → cycles `['edit', 'reading']`

**`src/lib/components/editor/ModeSwitcher.svelte`**: remove `live-preview` entry, show only Edit / Reading

## Step 7: Dependencies

**`package.json`**: remove `marked`, `codemirror-live-markdown`. Keep `katex` + `@types/katex`.

## Step 8: Documentation

Update `docs/ARCHITECTURE.md`:
- Replace `marked` → `pulldown-cmark` in tech stack
- Document `render.rs` module and `render_markdown` command
- Update editor mode description to 2 modes
- Remove lazy-reading-view references

## Verification

- `yarn check` — must pass cleanly
- `yarn tauri dev` — manual smoke test


<script lang="ts">
    /// CodeMirror editor component with support for two modes:
    /// - **edit**: raw markdown editing with syntax highlighting
    /// - **reading**: rendered HTML reading view (Rust-rendered via pulldown-cmark)
    ///
    /// Manages CodeMirror lifecycle (init, destroy, theme switching)
    /// and dispatches content changes to the parent via the `onchange` callback.
    import { onMount, onDestroy } from 'svelte';
    import { EditorView } from '@codemirror/view';
    import { EditorState, Compartment } from '@codemirror/state';
    import type { EditorMode } from '$lib/types';
    import { createEditorExtensions } from '$lib/editor-engine/setup';
    import ReadingView from './ReadingView.svelte';
    import { darkExtensions } from '$lib/editor-engine/themes/dark';
    import { lightExtensions } from '$lib/editor-engine/themes/light';
    import { getActiveTab, getScrollState, setScrollEdit, setScrollReading } from '$lib/stores/tabs.svelte';
    import { navigateWikilink } from '$lib/editor-engine/wikilinks';
    import 'katex/dist/katex.min.css';

    let {
        content = '',
        mode = 'edit' as EditorMode,
        theme = 'dark' as 'dark' | 'light',
        onchange
    }: {
        content?: string;
        mode?: EditorMode;
        theme?: 'dark' | 'light';
        onchange?: (content: string) => void;
    } = $props();

    let cmContainer: HTMLDivElement | undefined = $state();
    let view: EditorView | undefined = $state();
    let isExternalUpdate = false;

    let readingViewEl: HTMLDivElement | undefined = $state();
    let previousMode: EditorMode | null = null;
    let pendingContent: string | null = null;

    function getScrollRatio(el: HTMLElement): number {
        const scrollable = el.scrollHeight - el.clientHeight;
        return scrollable > 0 ? el.scrollTop / scrollable : 0;
    }

    function setScrollRatio(el: HTMLElement, ratio: number) {
        const scrollable = el.scrollHeight - el.clientHeight;
        if (scrollable > 0) el.scrollTop = ratio * scrollable;
    }

    function pathForContent(): string {
        return getActiveTab()?.path ?? '';
    }

    const themeCompartment = new Compartment();

    onMount(() => {
        if (!cmContainer) return;

        const initialState = EditorState.create({
            doc: content,
            extensions: [
                ...createEditorExtensions(),
                themeCompartment.of(
                    theme === 'light' ? lightExtensions() : darkExtensions()
                ),
                EditorView.updateListener.of((v) => {
                    if (v.docChanged) {
                        if (isExternalUpdate) {
                            isExternalUpdate = false;
                            return;
                        }
                        onchange?.(v.state.doc.toString());
                    }
                })
            ]
        });

        view = new EditorView({
            state: initialState,
            parent: cmContainer
        });

        view.scrollDOM.addEventListener('scroll', () => {
            const p = pathForContent();
            if (p) setScrollEdit(p, getScrollRatio(view!.scrollDOM));
        });

        cmContainer.addEventListener('click', onWikilinkClick);
    });

    onDestroy(() => {
        if (cmContainer) cmContainer.removeEventListener('click', onWikilinkClick);
        view?.destroy();
    });

    function onWikilinkClick(e: MouseEvent) {
        const el = (e.target as HTMLElement).closest('.cm-wikilink-content');
        if (!el) return;
        e.preventDefault();
        e.stopPropagation();
        const text = el.textContent ?? '';
        navigateWikilink(text, e.ctrlKey || e.metaKey);
    }

    export function getContent(): string {
        return view?.state.doc.toString() ?? content;
    }

    export function setContent(newContent: string) {
        if (view) {
            view.dispatch({
                changes: {
                    from: 0,
                    to: view.state.doc.length,
                    insert: newContent
                }
            });
        }
    }

    export function focus() {
        view?.focus();
    }

    export function setEditorTheme(t: 'dark' | 'light') {
        if (!view) return;
        view.dispatch({
            effects: themeCompartment.reconfigure(
                t === 'light' ? lightExtensions() : darkExtensions()
            )
        });
    }

    // React to content prop changes from outside (tab switch)
    $effect(() => {
        if (!view) return;
        const currentDoc = view.state.doc.toString();
        if (content !== currentDoc) {
            if (mode === 'reading') {
                pendingContent = content;
                return;
            }
            isExternalUpdate = true;
            view.dispatch({
                changes: {
                    from: 0,
                    to: view.state.doc.length,
                    insert: content
                }
            });
        }
    });

    // Track reading view scroll position
    $effect(() => {
        const el = readingViewEl;
        if (!el) return;
        const handler = () => {
            const p = pathForContent();
            if (p) setScrollReading(p, getScrollRatio(el));
        };
        el.addEventListener('scroll', handler);
        return () => el.removeEventListener('scroll', handler);
    });

    // Handle mode transitions: flush pending content + restore scroll
    $effect(() => {
        const currentMode = mode;
        if (previousMode === currentMode) return;
        const p = pathForContent();

        // Flush any deferred content when switching out of reading mode
        if (previousMode === 'reading' && pendingContent !== null) {
            if (view) {
                view.dispatch({
                    changes: { from: 0, to: view.state.doc.length, insert: pendingContent },
                    effects: []
                });
            }
            pendingContent = null;
            isExternalUpdate = true;
        }

        // Restore scroll position
        if (currentMode === 'reading') {
            const st = getScrollState(p);
            requestAnimationFrame(() => {
                if (readingViewEl) {
                    const target = st.reading !== null ? st.reading : st.edit;
                    setScrollRatio(readingViewEl, target);
                }
            });
        } else if (previousMode === 'reading') {
            const st = getScrollState(p);
            requestAnimationFrame(() => {
                if (view) setScrollRatio(view.scrollDOM, st.edit);
            });
        }

        previousMode = currentMode;
    });
</script>

<div class="editor-wrapper">
    <div class="cm-container" class:cm-hidden={mode === 'reading'} bind:this={cmContainer}></div>
    {#if mode === 'reading'}
        <div class="reading-view" bind:this={readingViewEl}>
            <ReadingView {content} />
        </div>
    {/if}
</div>

<style>
    .editor-wrapper {
        width: 100%;
        height: 100%;
        overflow: auto;
        position: relative;
    }

    .cm-container {
        height: 100%;
    }

    .cm-container.cm-hidden {
        position: absolute;
        inset: 0;
        visibility: hidden;
        pointer-events: none;
    }

    :global(.cm-container .cm-editor) {
        height: 100%;
    }

    :global(.cm-container .cm-scroller) {
        overflow: auto;
        font-family: 'Cascadia Code', 'Fira Code', 'JetBrains Mono', 'Consolas', monospace;
        font-size: 14px;
        line-height: 1.6;
    }

    :global(.cm-container .cm-content) {
        padding: 16px 0;
        max-width: 800px;
        margin: 0 auto;
        caret-color: var(--accent, #61afef);
    }

    .reading-view {
        max-width: 800px;
        margin: 0 auto;
        padding: 16px 24px;
        overflow-wrap: break-word;
        height: 100%;
        overflow-y: auto;
    }

    :global(.reading-view h1) { font-size: 2em; margin: 0.5em 0; }
    :global(.reading-view h2) { font-size: 1.5em; margin: 0.5em 0; }
    :global(.reading-view h3) { font-size: 1.25em; margin: 0.4em 0; }
    :global(.reading-view h4) { font-size: 1em; margin: 0.3em 0; }
    :global(.reading-view p) { margin: 0.5em 0; }
    :global(.reading-view ul), :global(.reading-view ol) { padding-left: 1.5em; margin: 0.5em 0; }
    :global(.reading-view blockquote) {
        border-left: 3px solid var(--accent, #61afef);
        margin: 0.5em 0;
        padding: 0.2em 1em;
        color: var(--text-muted, #888);
    }
    :global(.reading-view code) {
        background: var(--bg-secondary, #2a2a2a);
        padding: 2px 6px;
        border-radius: 3px;
        font-size: 0.9em;
    }
    :global(.reading-view pre) {
        background: var(--bg-secondary, #2a2a2a);
        padding: 12px 16px;
        border-radius: 6px;
        overflow-x: auto;
    }
    :global(.reading-view pre code) { background: none; padding: 0; }
    :global(.reading-view table) { border-collapse: collapse; width: 100%; margin: 0.5em 0; }
    :global(.reading-view th), :global(.reading-view td) { border: 1px solid var(--border-color, #444); padding: 6px 12px; text-align: left; }
    :global(.reading-view a) { color: var(--accent, #61afef); }
    :global(.reading-view a.wikilink) { color: var(--accent, #61afef); text-decoration: none; }
    :global(.reading-view a.wikilink:hover) { text-decoration: underline; }
    :global(.reading-view hr) { border: none; border-top: 1px solid var(--border-color, #444); margin: 1em 0; }

    :global(.reading-view ul:has(input[type="checkbox"])) {
        list-style: none;
        padding-left: 1.5em;
    }
    :global(.reading-view li:has(input[type="checkbox"])) {
        list-style: none;
    }
    :global(.reading-view input[type="checkbox"]) {
        width: 1em;
        height: 1em;
        margin: 0 0.4em 0 0;
        vertical-align: middle;
        accent-color: var(--accent, #61afef);
        cursor: default;
    }

    :global(.cm-wikilink-bracket) { opacity: 0.5; }
    :global(.cm-wikilink-content) { color: var(--accent, #61afef); cursor: pointer; }
    :global(.cm-wikilink-content:hover) { text-decoration: underline; }

    :global(.reading-view .math-inline) { display: inline; }
    :global(.reading-view .math-block) {
        display: block;
        text-align: center;
        margin: 12px 0;
        overflow-x: auto;
        overflow-y: hidden;
    }
    :global(.reading-view .math-error) {
        color: #e06c75;
        font-family: monospace;
        background: rgba(224, 108, 117, 0.1);
        padding: 2px 6px;
        border-radius: 3px;
    }

    :global(.reading-view img.image-embed) {
        max-width: 100%;
        border-radius: 4px;
        margin: 8px 0;
    }
</style>

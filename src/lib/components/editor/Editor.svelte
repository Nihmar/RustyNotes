<script lang="ts">
    import { onMount, onDestroy } from 'svelte';
    import { EditorView } from '@codemirror/view';
    import { EditorState, Compartment } from '@codemirror/state';
    import { createEditorExtensions } from '$lib/editor-engine/setup';
    import { renderMarkdown } from '$lib/editor-engine/reading-view';
    import { livePreview } from '$lib/editor-engine/live-preview';
    import { getEditorMode } from '$lib/stores/ui.svelte';

    let { content = '', onchange }: { content?: string; onchange?: (content: string) => void } = $props();

    let editorContainer: HTMLDivElement | undefined = $state();
    let view: EditorView | undefined = $state();
    let renderedHtml = $derived(renderMarkdown(content));

    const livePreviewCompartment = new Compartment();

    onMount(() => {
        if (!editorContainer) return;
        const state = EditorState.create({
            doc: content,
            extensions: [
                ...createEditorExtensions(),
                livePreviewCompartment.of([]),
                EditorView.updateListener.of((v) => {
                    if (v.docChanged) {
                        onchange?.(v.state.doc.toString());
                    }
                })
            ]
        });
        view = new EditorView({
            state,
            parent: editorContainer
        });
    });

    onDestroy(() => {
        view?.destroy();
    });

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

    $effect(() => {
        if (!view) return;
        const wrapper = editorContainer;
        if (!wrapper) return;

        const cmEl = wrapper.querySelector('.cm-editor') as HTMLElement | null;
        const readingEl = wrapper.querySelector('.reading-view') as HTMLElement | null;

        if (getEditorMode() === 'reading') {
            if (cmEl) cmEl.style.display = 'none';
            if (readingEl) readingEl.style.display = 'block';
            // Remove live preview if active
            view.dispatch({
                effects: livePreviewCompartment.reconfigure([])
            });
        } else {
            if (cmEl) cmEl.style.display = '';
            if (readingEl) readingEl.style.display = 'none';

            if (getEditorMode() === 'live-preview') {
                view.dispatch({
                    effects: livePreviewCompartment.reconfigure(livePreview())
                });
            } else {
                view.dispatch({
                    effects: livePreviewCompartment.reconfigure([])
                });
            }
        }
    });
</script>

<div class="editor-wrapper" bind:this={editorContainer}>
    <div class="reading-view">
        {@html renderedHtml}
    </div>
</div>

<style>
    .editor-wrapper {
        width: 100%;
        height: 100%;
        overflow: auto;
        position: relative;
    }

    :global(.editor-wrapper .cm-editor) {
        height: 100%;
    }

    :global(.editor-wrapper .cm-scroller) {
        overflow: auto;
        font-family: 'Cascadia Code', 'Fira Code', 'JetBrains Mono', 'Consolas', monospace;
        font-size: 14px;
        line-height: 1.6;
    }

    :global(.editor-wrapper .cm-content) {
        padding: 16px 0;
        max-width: 800px;
        margin: 0 auto;
        caret-color: var(--accent, #61afef);
    }

    .reading-view {
        max-width: 800px;
        margin: 0 auto;
        padding: 16px 24px;
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
        font-size: 15px;
        line-height: 1.7;
        display: none;
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

    :global(.cm-wikilink-bracket) { opacity: 0.5; }
    :global(.cm-wikilink-content) { color: var(--accent, #61afef); cursor: pointer; }
    :global(.cm-wikilink-content:hover) { text-decoration: underline; }

    /* Live Preview styles */
    :global(.cm-lp-bold) { font-weight: 700; }
    :global(.cm-lp-italic) { font-style: italic; }
    :global(.cm-lp-strikethrough) { text-decoration: line-through; }
    :global(.cm-lp-code) {
        background: var(--bg-secondary, #2a2a2a);
        padding: 0 4px;
        border-radius: 3px;
        font-family: monospace;
        font-size: 0.9em;
    }
    :global(.cm-lp-blockquote) {
        border-left: 3px solid var(--accent, #61afef);
        color: var(--text-muted, #888);
        padding-left: 12px;
    }
    :global(.cm-lp-hr) {
        border: none;
        border-top: 1px solid var(--border-color, #444);
        margin: 8px 0;
    }
    :global(.cm-lp-h1) { font-size: 1.8em; font-weight: 700; margin-top: 0.6em; }
    :global(.cm-lp-h2) { font-size: 1.5em; font-weight: 700; margin-top: 0.5em; }
    :global(.cm-lp-h3) { font-size: 1.3em; font-weight: 600; margin-top: 0.4em; }
    :global(.cm-lp-h4) { font-size: 1.1em; font-weight: 600; }
    :global(.cm-lp-h5) { font-size: 1em; font-weight: 600; color: var(--text-muted, #888); }
    :global(.cm-lp-h6) { font-size: 0.9em; font-weight: 600; color: var(--text-muted, #888); }
    :global(.cm-lp-bullet), :global(.cm-lp-checkbox), :global(.cm-lp-wikilink-pill) {
        display: inline-block;
        width: 1.2em;
        text-align: center;
    }
</style>

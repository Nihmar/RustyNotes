/// CodeMirror 6 editor setup and extension configuration.
///
/// Provides two extension bundles:
/// - `createEditorExtensions()` — core edit-mode extensions (syntax highlighting, keybindings, history, etc.)
/// - `createLivePreviewExtensions()` — live preview mode extensions via `codemirror-live-markdown`
///
/// Also includes custom plugins for wiki-links, math rendering, and image embeds.

import {
    keymap,
    lineNumbers,
    highlightSpecialChars,
    drawSelection,
    highlightActiveLine,
    dropCursor,
    rectangularSelection,
    crosshairCursor,
    highlightActiveLineGutter,
    Decoration,
    ViewPlugin,
    EditorView,
    ViewUpdate
} from '@codemirror/view';
import type { DecorationSet } from '@codemirror/view';
import { EditorState, RangeSetBuilder } from '@codemirror/state';
import {
    defaultHighlightStyle,
    syntaxHighlighting,
    indentOnInput,
    bracketMatching,
    foldGutter,
    foldKeymap,
    syntaxTree
} from '@codemirror/language';
import { defaultKeymap, history, historyKeymap } from '@codemirror/commands';
import { searchKeymap, highlightSelectionMatches } from '@codemirror/search';
import { closeBrackets, closeBracketsKeymap } from '@codemirror/autocomplete';
import { lintKeymap } from '@codemirror/lint';
import { markdown, markdownLanguage } from '@codemirror/lang-markdown';
import { Table } from '@lezer/markdown';
import type { Extension } from '@codemirror/state';
import { wikilinks } from './wikilinks';
import {
    livePreviewPlugin,
    markdownStylePlugin,
    editorTheme,
    mouseSelectingField,
    collapseOnSelectionFacet,
    tableField,
} from 'codemirror-live-markdown';
import { mathExtensions } from './math-live';
import { imageEmbedExtensions } from './image-embed';

/// Creates the base set of CodeMirror extensions for edit mode.
/// Includes line numbers, history, folding, bracket matching, markdown language support,
/// wiki-link highlighting, and default keybindings.
export function createEditorExtensions(): Extension[] {
    return [
        lineNumbers(),
        EditorView.lineWrapping,
        highlightActiveLineGutter(),
        highlightSpecialChars(),
        history(),
        foldGutter(),
        drawSelection(),
        dropCursor(),
        indentOnInput(),
        bracketMatching(),
        closeBrackets(),
        highlightActiveLine(),
        highlightSelectionMatches(),
        rectangularSelection(),
        crosshairCursor(),

        markdown({
            base: markdownLanguage,
            extensions: [Table]
        }),

        wikilinks(),

        keymap.of([
            ...closeBracketsKeymap,
            ...defaultKeymap,
            ...searchKeymap,
            ...historyKeymap,
            ...foldKeymap,
            ...lintKeymap
        ]),

        syntaxHighlighting(defaultHighlightStyle, { fallback: true }),

        mouseSelectingField
    ];
}

/// Decoration for visible list markers in live preview mode.
const visibleListMark = Decoration.mark({ class: 'cm-lp-list-visible' });

/// Custom view plugin that applies visible list marker decorations in the CodeMirror editor.
/// Iterates the syntax tree and marks `ListMark` nodes for rendering in live preview mode.
const listMarkPlugin = ViewPlugin.fromClass(class {
    decorations: DecorationSet;

    constructor(view: EditorView) {
        this.decorations = this.build(view);
    }

    update(update: ViewUpdate) {
        if (update.docChanged || update.viewportChanged) {
            this.decorations = this.build(update.view);
        }
    }

    build(view: EditorView) {
        const builder = new RangeSetBuilder<Decoration>();
        syntaxTree(view.state).iterate({
            enter: (node) => {
                if (node.name === 'ListMark') {
                    builder.add(node.from, node.to, visibleListMark);
                }
            }
        });
        return builder.finish();
    }
}, {
    decorations: (v) => v.decorations
});

/// Creates extensions for live preview mode on top of base edit extensions.
/// Enables WYSIWYG-like markdown rendering (collapsible formatting, table rendering, etc.).
export function createLivePreviewExtensions(): Extension[] {
    return [
        collapseOnSelectionFacet.of(true),
        livePreviewPlugin,
        markdownStylePlugin,
        editorTheme,
        tableField,
        listMarkPlugin,
        ...mathExtensions(),
        ...imageEmbedExtensions(),
    ];
}

export function createEditorState(doc: string): EditorState {
    return EditorState.create({
        doc,
        extensions: createEditorExtensions()
    });
}

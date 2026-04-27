/// CodeMirror 6 editor setup and extension configuration.
///
/// Provides the core edit-mode extension bundle:
/// - `createEditorExtensions()` — syntax highlighting, keybindings, history, wiki-links, etc.
///
/// Wiki-link highlighting in the editor is handled by the `wikilinks` module.
/// Reading view rendering is handled on the Rust side via `render_markdown` command.

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
    EditorView
} from '@codemirror/view';
import { EditorState } from '@codemirror/state';
import {
    defaultHighlightStyle,
    syntaxHighlighting,
    indentOnInput,
    bracketMatching,
    foldGutter,
    foldKeymap
} from '@codemirror/language';
import { defaultKeymap, history, historyKeymap } from '@codemirror/commands';
import { searchKeymap, highlightSelectionMatches } from '@codemirror/search';
import { closeBrackets, closeBracketsKeymap } from '@codemirror/autocomplete';
import { lintKeymap } from '@codemirror/lint';
import { markdown, markdownLanguage } from '@codemirror/lang-markdown';
import { Table } from '@lezer/markdown';
import type { Extension } from '@codemirror/state';
import { wikilinks } from './wikilinks';

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
    ];
}

export function createEditorState(doc: string): EditorState {
    return EditorState.create({
        doc,
        extensions: createEditorExtensions()
    });
}

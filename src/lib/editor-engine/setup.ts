import {
    keymap,
    lineNumbers,
    highlightSpecialChars,
    drawSelection,
    highlightActiveLine,
    dropCursor,
    rectangularSelection,
    crosshairCursor,
    highlightActiveLineGutter
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
import type { Extension } from '@codemirror/state';
import { wikilinks } from './wikilinks';

export function createEditorExtensions(): Extension[] {
    return [
        lineNumbers(),
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

        // Markdown with syntax highlighting
        markdown({
            base: markdownLanguage
        }),

        // Wiki-links support
        wikilinks(),

        // Keymaps
        keymap.of([
            ...closeBracketsKeymap,
            ...defaultKeymap,
            ...searchKeymap,
            ...historyKeymap,
            ...foldKeymap,
            ...lintKeymap
        ]),

        // Basic syntax highlighting
        syntaxHighlighting(defaultHighlightStyle, { fallback: true })
    ];
}

export function createEditorState(doc: string): EditorState {
    return EditorState.create({
        doc,
        extensions: createEditorExtensions()
    });
}

/// Light color theme for the CodeMirror editor.
/// Provides syntax highlighting styles for markdown elements (headings, links, code, etc.).

import { EditorView } from '@codemirror/view';
import { HighlightStyle, syntaxHighlighting } from '@codemirror/language';
import { tags } from '@lezer/highlight';
import type { Extension } from '@codemirror/state';

const lightTheme = EditorView.theme({
    '&': {
        backgroundColor: '#ffffff',
        color: '#1e1e1e'
    },
    '.cm-content': {
        caretColor: '#0366d6'
    },
    '.cm-cursor, .cm-dropCursor': {
        borderLeftColor: '#0366d6'
    },
    '&.cm-focused .cm-selectionBackground, .cm-selectionBackground, .cm-content ::selection': {
        backgroundColor: '#add6ff'
    },
    '.cm-activeLine': {
        backgroundColor: '#f0f0f0'
    },
    '.cm-activeLineGutter': {
        backgroundColor: '#f5f5f5'
    },
    '.cm-gutters': {
        backgroundColor: '#ffffff',
        color: '#6e6e6e',
        border: 'none'
    }
}, { dark: false });

const lightHighlight = HighlightStyle.define([
    { tag: tags.heading1, fontSize: '1.8em', fontWeight: '700', color: '#1e1e1e' },
    { tag: tags.heading2, fontSize: '1.5em', fontWeight: '700', color: '#1e1e1e' },
    { tag: tags.heading3, fontSize: '1.3em', fontWeight: '600' },
    { tag: tags.emphasis, fontStyle: 'italic' },
    { tag: tags.strong, fontWeight: '700' },
    { tag: tags.strikethrough, textDecoration: 'line-through' },
    { tag: tags.url, color: '#0366d6' },
    { tag: tags.link, color: '#0366d6' },
    { tag: tags.monospace, fontFamily: 'monospace', backgroundColor: '#f6f8fa', borderRadius: '3px', padding: '0 4px' },
    { tag: tags.comment, color: '#6a737d' },
    { tag: tags.keyword, color: '#d73a49' },
    { tag: tags.string, color: '#032f62' },
    { tag: tags.number, color: '#005cc5' },
]);

export function lightExtensions(): Extension[] {
    return [lightTheme, syntaxHighlighting(lightHighlight)];
}

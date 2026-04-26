/// Dark color theme for the CodeMirror editor (default).
/// Provides syntax highlighting styles for markdown elements (headings, links, code, etc.).

import { EditorView } from '@codemirror/view';
import { HighlightStyle, syntaxHighlighting } from '@codemirror/language';
import { tags } from '@lezer/highlight';
import type { Extension } from '@codemirror/state';

const darkTheme = EditorView.theme({
    '&': {
        backgroundColor: '#1e1e1e',
        color: '#d4d4d4'
    },
    '.cm-content': {
        caretColor: '#61afef'
    },
    '.cm-cursor, .cm-dropCursor': {
        borderLeftColor: '#61afef'
    },
    '&.cm-focused .cm-selectionBackground, .cm-selectionBackground, .cm-content ::selection': {
        backgroundColor: '#264f78'
    },
    '.cm-activeLine': {
        backgroundColor: '#2a2a2a'
    },
    '.cm-activeLineGutter': {
        backgroundColor: '#252525'
    },
    '.cm-gutters': {
        backgroundColor: '#1e1e1e',
        color: '#858585',
        border: 'none'
    },
    '.cm-foldPlaceholder': {
        backgroundColor: '#333',
        color: '#888',
        border: 'none'
    }
}, { dark: true });

const darkHighlight = HighlightStyle.define([
    { tag: tags.heading1, fontSize: '1.8em', fontWeight: '700' },
    { tag: tags.heading2, fontSize: '1.5em', fontWeight: '700' },
    { tag: tags.heading3, fontSize: '1.3em', fontWeight: '600' },
    { tag: tags.heading4, fontSize: '1.1em', fontWeight: '600' },
    { tag: tags.heading5, fontSize: '1em', fontWeight: '600', color: '#888' },
    { tag: tags.heading6, fontSize: '0.9em', fontWeight: '600', color: '#888' },
    { tag: tags.emphasis, fontStyle: 'italic' },
    { tag: tags.strong, fontWeight: '700' },
    { tag: tags.strikethrough, textDecoration: 'line-through' },
    { tag: tags.url, color: '#61afef' },
    { tag: tags.link, color: '#61afef' },
    { tag: tags.monospace, fontFamily: 'monospace', backgroundColor: '#2a2a2a', borderRadius: '3px', padding: '0 4px' },
    { tag: tags.comment, color: '#6a9955' },
    { tag: tags.keyword, color: '#c586c0' },
    { tag: tags.string, color: '#ce9178' },
    { tag: tags.number, color: '#b5cea8' },
    { tag: tags.bracket, color: '#d4d4d4' },
    { tag: tags.list, color: '#d4d4d4' },
    { tag: tags.quote, color: '#888' },
    { tag: tags.contentSeparator, color: '#555' },
]);

export function darkExtensions(): Extension[] {
    return [darkTheme, syntaxHighlighting(darkHighlight)];
}

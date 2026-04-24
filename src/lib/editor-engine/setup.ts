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
    mathPlugin,
    blockMathField
} from 'codemirror-live-markdown';

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

const visibleListMark = Decoration.mark({ class: 'cm-lp-list-visible' });

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

export function createLivePreviewExtensions(): Extension[] {
    return [
        collapseOnSelectionFacet.of(true),
        livePreviewPlugin,
        markdownStylePlugin,
        editorTheme,
        listMarkPlugin,
        mathPlugin,
        blockMathField
    ];
}

export function createEditorState(doc: string): EditorState {
    return EditorState.create({
        doc,
        extensions: createEditorExtensions()
    });
}

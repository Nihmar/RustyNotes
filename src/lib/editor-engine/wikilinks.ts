import {
    Decoration,
    EditorView,
    ViewPlugin,
    ViewUpdate
} from '@codemirror/view';
import type { DecorationSet } from '@codemirror/view';
import { RangeSetBuilder } from '@codemirror/state';
import { syntaxTree } from '@codemirror/language';

const wikiLinkDeco = Decoration.mark({ class: 'cm-wikilink' });

function decorateWikilinks(view: EditorView): DecorationSet {
    const builder = new RangeSetBuilder<Decoration>();
    const doc = view.state.doc;

    // Simple regex approach for wikilinks
    const text = doc.toString();
    const regex = /\[\[([^\]]+)\]\]/g;
    let match;

    while ((match = regex.exec(text)) !== null) {
        const from = match.index;
        const to = match.index + match[0].length;
        builder.add(from, from + 2, Decoration.mark({ class: 'cm-wikilink-bracket' }));
        builder.add(from + 2, to - 2, Decoration.mark({ class: 'cm-wikilink-content' }));
        builder.add(to - 2, to, Decoration.mark({ class: 'cm-wikilink-bracket' }));
    }

    return builder.finish();
}

const wikilinkPlugin = ViewPlugin.fromClass(class {
    decorations: DecorationSet;

    constructor(view: EditorView) {
        this.decorations = decorateWikilinks(view);
    }

    update(update: ViewUpdate) {
        if (update.docChanged || update.viewportChanged) {
            this.decorations = decorateWikilinks(update.view);
        }
    }
}, {
    decorations: (v) => v.decorations
});

export function wikilinks() {
    return wikilinkPlugin;
}

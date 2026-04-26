import {
    Decoration,
    EditorView,
    ViewPlugin,
    ViewUpdate,
    WidgetType
} from '@codemirror/view';
import type { DecorationSet } from '@codemirror/view';
import { EditorState, RangeSetBuilder } from '@codemirror/state';

class ImageEmbedWidget extends WidgetType {
    constructor(private target: string) { super(); }

    toDOM() {
        const img = document.createElement('img');
        img.src = `vault://localhost/${this.target}`;
        img.className = 'cm-image-embed-img';
        img.alt = this.target;
        img.loading = 'lazy';
        const wrapper = document.createElement('span');
        wrapper.className = 'cm-image-embed-wrapper';
        wrapper.appendChild(img);
        return wrapper;
    }

    eq(other: ImageEmbedWidget) {
        return this.target === other.target;
    }

    ignoreEvent() { return false; }
}

function isTouched(state: EditorState, from: number, to: number): boolean {
    const sel = state.selection.main;
    return sel.from <= to && sel.to >= from;
}

const imageEmbedRe = /!\[\[([^\]]+)\]\]/g;

const imageEmbedPlugin = ViewPlugin.fromClass(class {
    decorations: DecorationSet;

    constructor(view: EditorView) {
        this.decorations = this.build(view);
    }

    update(update: ViewUpdate) {
        if (update.docChanged || update.viewportChanged || update.selectionSet) {
            this.decorations = this.build(update.view);
        }
    }

    build(view: EditorView) {
        const builder = new RangeSetBuilder<Decoration>();
        const doc = view.state.doc;

        for (let i = 1; i <= doc.lines; i++) {
            const line = doc.line(i);
            const text = line.text;
            imageEmbedRe.lastIndex = 0;
            let match;
            while ((match = imageEmbedRe.exec(text)) !== null) {
                const from = line.from + match.index;
                const to = from + match[0].length;
                if (!isTouched(view.state, from, to)) {
                    const inner = match[1].trim();
                    const target = inner.includes('|') ? inner.split('|')[0].trim() : inner;
                    const widget = new ImageEmbedWidget(target);
                    builder.add(from, to, Decoration.replace({ widget }));
                }
            }
        }
        return builder.finish();
    }
}, {
    decorations: (v) => v.decorations
});

export function imageEmbedExtensions() {
    return [imageEmbedPlugin];
}

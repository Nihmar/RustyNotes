/// Live math rendering for the CodeMirror editor.
///
/// Provides inline (`$...$`) and display (`$$...$$`) KaTeX math rendering as
/// editor decorations (widgets). Math is rendered in-place when the cursor is
/// not touching the expression, and shows the raw source when editing.
///
/// Includes two extensions:
/// - `inlineMathPlugin` — ViewPlugin for `$...$` inline math
/// - `displayMathField` — StateField for `$$...$$` display math blocks

import {
    Decoration,
    EditorView,
    ViewPlugin,
    ViewUpdate,
    WidgetType
} from '@codemirror/view';
import type { DecorationSet } from '@codemirror/view';
import { EditorState, RangeSetBuilder, StateField } from '@codemirror/state';
import katex from 'katex';

/// KaTeX widget that replaces math source text with rendered output.
/// Falls back to raw text on render errors.
class MathWidget extends WidgetType {
    constructor(private source: string, private displayMode: boolean) { super(); }

    toDOM() {
        const container = document.createElement(this.displayMode ? 'div' : 'span');
        container.className = this.displayMode ? 'cm-math-block' : 'cm-math-inline';
        try {
            katex.render(this.source, container, { displayMode: this.displayMode, throwOnError: false });
        } catch {
            container.textContent = `$${this.source}$`;
            container.className += ' cm-math-error';
        }
        return container;
    }

    eq(other: MathWidget) {
        return this.source === other.source && this.displayMode === other.displayMode;
    }

    ignoreEvent() { return false; }
}

function isTouched(state: EditorState, from: number, to: number): boolean {
    const sel = state.selection.main;
    return sel.from <= to && sel.to >= from;
}

const inlineRe = /(?<!\$)\$(?!\$)([^$\n]+?)\$(?!\$)/g;

const inlineMathPlugin = ViewPlugin.fromClass(class {
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
            inlineRe.lastIndex = 0;
            let match;
            while ((match = inlineRe.exec(text)) !== null) {
                const from = line.from + match.index;
                const to = from + match[0].length;
                if (!isTouched(view.state, from, to)) {
                    const widget = new MathWidget(match[1].trim(), false);
                    builder.add(from, to, Decoration.replace({ widget }));
                }
            }
        }
        return builder.finish();
    }
}, {
    decorations: (v) => v.decorations
});

function buildDisplayMath(state: EditorState): DecorationSet {
    const decorations: { from: number; to: number; value: Decoration }[] = [];
    const doc = state.doc;
    let inBlock = false;
    let blockStartLine = -1;
    let blockFrom = -1;
    const lines: string[] = [];

    for (let i = 1; i <= doc.lines; i++) {
        const line = doc.line(i);
        const text = line.text;

        if (!inBlock && /^\$\$\s*$/.test(text)) {
            inBlock = true;
            blockStartLine = i;
            blockFrom = line.from;
            lines.length = 0;
            continue;
        }
        if (inBlock && /^\$\$\s*$/.test(text)) {
            const blockTo = line.to;
            if (!isTouched(state, blockFrom, blockTo)) {
                const widget = new MathWidget(lines.join('\n').trim(), true);
                decorations.push({
                    from: blockFrom,
                    to: blockTo,
                    value: Decoration.replace({ widget, block: true })
                });
            } else {
                for (let j = blockStartLine; j <= i; j++) {
                    const l = doc.line(j);
                    decorations.push({
                        from: l.from,
                        to: l.from,
                        value: Decoration.line({ class: 'cm-math-source-block' })
                    });
                }
            }
            inBlock = false;
            continue;
        }
        if (inBlock) {
            lines.push(text);
        }
    }

    return Decoration.set(
        decorations.sort((a, b) => a.from - b.from),
        true
    );
}

const displayMathField = StateField.define<DecorationSet>({
    create(state) {
        return buildDisplayMath(state);
    },
    update(deco, tr) {
        if (tr.docChanged || tr.reconfigured || tr.selection) {
            return buildDisplayMath(tr.state);
        }
        return deco;
    },
    provide: (f) => EditorView.decorations.from(f)
});

export function mathExtensions() {
    return [inlineMathPlugin, displayMathField];
}

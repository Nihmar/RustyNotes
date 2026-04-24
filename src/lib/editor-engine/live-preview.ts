import {
    Decoration,
    EditorView,
    ViewPlugin,
    ViewUpdate,
    WidgetType
} from '@codemirror/view';
import type { DecorationSet } from '@codemirror/view';
import { RangeSetBuilder } from '@codemirror/state';

class HrWidget extends WidgetType {
    toDOM() {
        const el = document.createElement('hr');
        el.className = 'cm-lp-hr';
        return el;
    }
    eq() { return true; }
}

class SpanWidget extends WidgetType {
    constructor(private span: HTMLSpanElement) { super(); }
    toDOM() { return this.span; }
    eq(other: SpanWidget) {
        return this.span.outerHTML === other.span.outerHTML;
    }
}

const boldMark = Decoration.mark({ class: 'cm-lp-bold' });
const italicMark = Decoration.mark({ class: 'cm-lp-italic' });
const strikethroughMark = Decoration.mark({ class: 'cm-lp-strikethrough' });
const codeMark = Decoration.mark({ class: 'cm-lp-code' });
const blockquoteMark = Decoration.line({ class: 'cm-lp-blockquote' });
const hrWidget = Decoration.widget({ widget: new HrWidget() });
const hiddenSyntax = Decoration.replace({});

function decorateLivePreview(view: EditorView): DecorationSet {
    const builder = new RangeSetBuilder<Decoration>();
    const doc = view.state.doc;
    const cursorLine = view.state.selection.main.head;
    const cursorLineNum = doc.lineAt(cursorLine).number;
    const text = doc.toString();

    // Process line by line
    for (let i = 1; i <= doc.lines; i++) {
        const line = doc.line(i);
        const lineText = line.text;
        const lineFrom = line.from;
        const isCursorLine = i === cursorLineNum;

        // Cursor line stays raw (no decorations)
        if (isCursorLine) continue;

        // Headings
        const headingMatch = lineText.match(/^(#{1,6})\s+(.*)/);
        if (headingMatch) {
            const level = headingMatch[1].length;
            builder.add(lineFrom, lineFrom + headingMatch[1].length, hiddenSyntax);
            builder.add(
                lineFrom,
                lineFrom + line.length,
                Decoration.line({ class: `cm-lp-h${level}` })
            );
            continue;
        }

        // Horizontal rules
        if (/^[-*_]{3,}\s*$/.test(lineText)) {
            builder.add(lineFrom, line.to, hrWidget);
            continue;
        }

        // Blockquotes
        const bqMatch = lineText.match(/^>\s?(.*)/);
        if (bqMatch) {
            builder.add(lineFrom, lineFrom + bqMatch[0].length - bqMatch[1].length, hiddenSyntax);
            builder.add(lineFrom, line.to, blockquoteMark);
        }

        // Unordered list items
        const ulMatch = lineText.match(/^(\s*)([-*+])\s+(.+)/);
        if (ulMatch && !bqMatch) {
            const indent = ulMatch[1].length;
            const markerEnd = lineFrom + ulMatch[1].length + ulMatch[2].length;
            builder.add(lineFrom, markerEnd + 1, hiddenSyntax);
            const bulletSpan = document.createElement('span');
            bulletSpan.className = 'cm-lp-bullet';
            bulletSpan.textContent = '•';
            builder.add(
                lineFrom + indent,
                markerEnd,
                Decoration.widget({ widget: new SpanWidget(bulletSpan), side: 1 })
            );
            continue;
        }

        // Ordered list items
        const olMatch = lineText.match(/^(\s*)(\d+)\.\s+(.+)/);
        if (olMatch && !bqMatch) {
            const indent = olMatch[1].length;
            builder.add(lineFrom, lineFrom + indent + olMatch[2].length + 1, hiddenSyntax);
            continue;
        }

        // Checkbox / task list
        const checkMatch = lineText.match(/^(\s*)[-*+]\s+\[([ xX])\]\s+(.+)/);
        if (checkMatch) {
            const indent = checkMatch[1].length;
            const checked = checkMatch[2].toLowerCase() === 'x';
            builder.add(lineFrom, lineFrom + indent, hiddenSyntax);
            const endOfMarker = lineFrom + indent + 1; // after -
            builder.add(endOfMarker, endOfMarker + 4, hiddenSyntax);

            const span = document.createElement('span');
            span.className = 'cm-lp-checkbox';
            span.textContent = checked ? '☑' : '☐';
            builder.add(
                lineFrom + indent + 1,
                lineFrom + indent + 4,
                Decoration.widget({ widget: new SpanWidget(span), side: 1 })
            );
            continue;
        }

        // Code blocks (triple backtick)
        if (lineText.startsWith('```')) {
            builder.add(lineFrom, line.to, hiddenSyntax);
            continue;
        }
    }

    // Process inline styles: bold, italic, code, strikethrough
    const inlineRegexes = [
        { regex: /\*\*(.+?)\*\*/g, deco: boldMark, group: 1, openLen: 2, closeLen: 2 },
        { regex: /(?<!\*)\*(?!\*)(.+?)(?<!\*)\*(?!\*)/g, deco: italicMark, group: 1, openLen: 1, closeLen: 1 },
        { regex: /__(.+?)__/g, deco: boldMark, group: 1, openLen: 2, closeLen: 2 },
        { regex: /(?<!_)_(?!_)(.+?)(?<!_)_(?!_)/g, deco: italicMark, group: 1, openLen: 1, closeLen: 1 },
        { regex: /`([^`]+)`/g, deco: codeMark, group: 1, openLen: 1, closeLen: 1 },
        { regex: /~~(.+?)~~/g, deco: strikethroughMark, group: 1, openLen: 2, closeLen: 2 },
    ];

    for (let i = 1; i <= doc.lines; i++) {
        if (i === cursorLineNum) continue;
        const line = doc.line(i);
        const lineText = line.text;
        const lineFrom = line.from;

        for (const { regex, deco, openLen, closeLen } of inlineRegexes) {
            let match;
            const r = new RegExp(regex.source, regex.flags);
            r.lastIndex = 0;
            while ((match = r.exec(lineText)) !== null) {
                const absStart = lineFrom + match.index;
                const absEnd = absStart + match[0].length;
                builder.add(absStart, absStart + openLen, hiddenSyntax);
                builder.add(absEnd - closeLen, absEnd, hiddenSyntax);
                builder.add(absStart + openLen, absEnd - closeLen, deco);
            }
        }
    }

    return builder.finish();
}

const livePreviewPlugin = ViewPlugin.fromClass(class {
    decorations: DecorationSet;

    constructor(view: EditorView) {
        this.decorations = decorateLivePreview(view);
    }

    update(update: ViewUpdate) {
        if (update.docChanged || update.selectionSet || update.viewportChanged) {
            this.decorations = decorateLivePreview(update.view);
        }
    }
}, {
    decorations: (v) => v.decorations
});

export function livePreview() {
    return livePreviewPlugin;
}
